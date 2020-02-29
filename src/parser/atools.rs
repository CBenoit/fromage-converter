use crate::{
    error::{LactoseIntolerance, Result},
    fromage::Fromage,
    parser::{FromageMaker, FromagemakingProcess},
};
use std::io::{BufRead, Lines};

pub struct AToolsMaker<Reader: BufRead>(std::marker::PhantomData<Reader>);

impl<Reader: BufRead> Default for AToolsMaker<Reader> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<Reader: BufRead> FromageMaker for AToolsMaker<Reader> {
    type Reader = Reader;
    type Process = AToolsParser<Self::Reader>;

    fn process(self, reader: Self::Reader) -> Result<Self::Process> {
        Ok(AToolsParser {
            inner: reader.lines(),
            current_line: String::new(),
        })
    }
}

pub struct AToolsParser<Reader: BufRead> {
    inner: Lines<Reader>,
    current_line: String,
}

impl<Reader: BufRead> FromagemakingProcess for AToolsParser<Reader> {
    fn next_fromage(&mut self) -> Option<Result<Fromage>> {
        self.current_line = match self.inner.next()? {
            Ok(line) => line,
            Err(e) => return Some(Err(LactoseIntolerance::Io(e))),
        };

        match parse_line(&self.current_line) {
            Ok(kind) => Some(Ok(kind)),
            Err(bad_part) => Some(Err(LactoseIntolerance::Dyn(format!(
                "bad line: {}",
                bad_part
            )))),
        }
    }
}

fn parse_line(line: &str) -> std::result::Result<Fromage, &str> {
    let mut iter = line.chars();
    match iter.next() {
        None => Ok(Fromage::empty()),
        Some(';') => match iter.next() {
            Some(' ') => Ok(Fromage::comment(&line[2..])),
            Some('s') | Some('m') => parse_line(&line[1..]),
            _ => Ok(Fromage::comment(&line[1..])),
        },
        Some(ty) => {
            while {
                match iter.next() {
                    Some('[') => false,
                    Some(_) => true,
                    None => {
                        log::debug!("expected '[' but reached unexpected end of line");
                        return Err(line);
                    }
                }
            } {}

            let mut id = String::new();
            while {
                match iter.next() {
                    Some(']') => false,
                    Some(c) => {
                        id.push(c);
                        true
                    }
                    None => {
                        log::debug!("found '[' but there is no matching ']'");
                        return Err(line);
                    }
                }
            } {}

            let id = id.parse::<u64>().map_err(|_| line)?;

            while {
                match iter.next() {
                    Some('"') => false,
                    Some(_) => true,
                    None => {
                        log::debug!("no opening \" found");
                        return Err(line);
                    }
                }
            } {}

            let mut val = String::new();
            while {
                match iter.next() {
                    Some('"') => false,
                    Some(c) => {
                        val.push(c);
                        true
                    }
                    None => {
                        log::debug!("no closing \" found");
                        return Err(line);
                    }
                }
            } {}

            match ty {
                's' => Ok(Fromage::str(id, val)),
                'm' => Ok(Fromage::msg(id, val)),
                unknown => {
                    log::debug!("unknown type tag: {}", unknown);
                    Err(line)
                }
            }
        }
    }
}
