use crate::{
    error::{LactoseIntolerance, Result},
    fromage::Fromage,
    parser::{FromageMaker, FromagemakingProcess},
};
use std::io::{BufRead, Lines};

pub struct CsvMaker<Reader: BufRead> {
    sep: char,
    translation_column: String,
    _pd: std::marker::PhantomData<Reader>,
}

impl<Reader: BufRead> CsvMaker<Reader> {
    pub fn new<S: Into<String>>(separator: char, translation_column: S) -> Self {
        Self {
            sep: separator,
            translation_column: translation_column.into(),
            _pd: std::marker::PhantomData,
        }
    }
}

impl<Reader: BufRead> FromageMaker for CsvMaker<Reader> {
    type Reader = Reader;
    type Process = CsvParser<Self::Reader>;

    fn process(self, mut reader: Self::Reader) -> Result<Self::Process> {
        let mut line = String::default();
        reader.read_line(&mut line)?;

        let mut translation_column = 0;
        let mut val = String::new();
        let mut iter = line.chars();
        while {
            match iter.next() {
                Some(c) if c == self.sep => {
                    if val == self.translation_column {
                        false
                    } else {
                        translation_column += 1;
                        val.clear();
                        true
                    }
                }
                Some(c) => {
                    val.push(c);
                    true
                }
                None => false,
            }
        } {}

        if translation_column < 2 {
            return Err(LactoseIntolerance::Static("bad translation column"));
        }

        log::info!(
            "CSV Parser: separator = '{}', translation column = {} ({})",
            self.sep,
            self.translation_column,
            translation_column
        );

        Ok(CsvParser {
            sep: self.sep,
            translation_column,
            inner: reader.lines(),
            current_line: String::new(),
        })
    }
}

pub struct CsvParser<Reader: BufRead> {
    sep: char,
    translation_column: u8,
    inner: Lines<Reader>,
    current_line: String,
}

impl<Reader: BufRead> FromagemakingProcess for CsvParser<Reader> {
    fn next_fromage(&mut self) -> Option<Result<Fromage>> {
        self.current_line = match self.inner.next()? {
            Ok(line) => line,
            Err(e) => return Some(Err(LactoseIntolerance::Io(e))),
        };

        match parse_line(&self.current_line, self.sep, self.translation_column) {
            Ok(kind) => Some(Ok(kind)),
            Err(bad_part) => Some(Err(LactoseIntolerance::Dyn(format!(
                "bad line: {}",
                bad_part
            )))),
        }
    }
}

fn parse_line(
    line: &str,
    sep: char,
    mut translation_column: u8,
) -> std::result::Result<Fromage, &str> {
    if line.is_empty() {
        return Ok(Fromage::empty());
    }

    let mut iter = line.chars();

    let mut kind = String::new();
    while {
        match iter.next() {
            None => {
                log::debug!("id is missing");
                return Err(line);
            }
            Some(c) if c == sep => false,
            Some(c) => {
                kind.push(c);
                true
            }
        }
    } {}

    if kind == "com" {
        translation_column = 2;
    }

    let mut id = String::new();
    while {
        match iter.next() {
            None => {
                log::debug!("text is missing");
                return Err(line);
            }
            Some(c) if c == sep => false,
            Some(c) => {
                id.push(c);
                true
            }
        }
    } {}

    let mut current_column = 2;

    let mut val = String::new();
    while {
        match iter.next() {
            None => false,
            Some(c) if c == sep => {
                if current_column == translation_column {
                    false
                } else {
                    current_column += 1;
                    true
                }
            }
            Some('"') => {
                while {
                    match iter.next() {
                        None => {
                            log::debug!("string literal ending \" is missing");
                            return Err(line);
                        }
                        Some('"') => false,
                        Some(c) => {
                            if current_column == translation_column {
                                val.push(c);
                            }
                            true
                        }
                    }
                } {}
                true
            }
            Some(c) => {
                if current_column == translation_column {
                    val.push(c);
                }
                true
            }
        }
    } {}

    match kind.as_str() {
        "com" => Ok(Fromage::comment(val)),
        "(str)" => {
            let id = id.parse::<u64>().map_err(|_| line)?;
            let mut fromage = Fromage::str(id, val);
            fromage.ignored = true;
            Ok(fromage)
        }
        "str" => {
            let id = id.parse::<u64>().map_err(|_| line)?;
            Ok(Fromage::str(id, val))
        }
        "msg" => {
            let id = id.parse::<u64>().map_err(|_| line)?;
            Ok(Fromage::msg(id, val))
        }
        unknown => {
            log::debug!("unknown CSV kind: {}", unknown);
            Err(line)
        }
    }
}
