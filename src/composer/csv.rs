use crate::{
    composer::FromageCook, error::Result, fromage::FromageKind, parser::FromagemakingProcess,
};
use std::{collections::HashSet, io::Write};

pub struct CsvComposer {
    pub sep: char,
}

impl FromageCook for CsvComposer {
    fn process<Process, Writer>(self, mut process: Process, mut o: Writer) -> Result<()>
    where
        Process: FromagemakingProcess,
        Writer: Write,
    {
        let mut inserted_str_id = HashSet::<u64>::new();

        writeln!(o, r#"KIND{}ID{}ORIGINAL"#, self.sep, self.sep)?;

        while let Some(fromage_res) = process.next_fromage() {
            let fromage = match fromage_res {
                Ok(f) => f,
                Err(e) => {
                    log::warn!("{}", e);
                    continue;
                }
            };

            match fromage.kind {
                FromageKind::Empty => writeln!(o)?,
                FromageKind::Comment(text) => {
                    writeln!(o, "com{}###{}{}", self.sep, self.sep, text)?
                }
                FromageKind::Str { id, val } => {
                    if inserted_str_id.contains(&id) {
                        writeln!(o, r#"(str){}{}{}"{}""#, self.sep, id, self.sep, val)?
                    } else {
                        inserted_str_id.insert(id);
                        writeln!(o, r#"str{}{}{}"{}""#, self.sep, id, self.sep, val)?
                    }
                }
                FromageKind::Msg { id, val } => {
                    writeln!(o, r#"msg{}{}{}"{}""#, self.sep, id, self.sep, val)?
                }
            }
        }

        Ok(())
    }
}
