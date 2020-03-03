use crate::{
    composer::FromageCook, error::Result, fromage::FromageKind, parser::FromagemakingProcess,
};
use std::io::Write;

pub struct AToolsComposer;

impl FromageCook for AToolsComposer {
    fn process<Process, Writer>(self, mut process: Process, mut o: Writer) -> Result<()>
    where
        Process: FromagemakingProcess,
        Writer: Write,
    {
        while let Some(fromage_res) = process.next_fromage() {
            let fromage = match fromage_res {
                Ok(f) => f,
                Err(e) => {
                    log::warn!("{}", e);
                    continue;
                }
            };

            match fromage.kind {
                FromageKind::Empty => write!(o, "\r\n")?,
                FromageKind::Comment(text) => write!(o, "; {}\r\n", text)?,
                FromageKind::Str { id, val } => {
                    if fromage.ignored {
                        write!(o, ";s[{}] = \"{}\"\r\n", id, val)?
                    } else {
                        write!(o, "s[{}] = \"{}\"\r\n", id, val)?
                    }
                }
                FromageKind::Msg { id, val } => {
                    if fromage.ignored {
                        write!(o, ";m[{}] = \"{}\"\r\n", id, val)?
                    } else {
                        write!(o, "m[{}] = \"{}\"\r\n", id, val)?
                    }
                }
            }
        }

        Ok(())
    }
}
