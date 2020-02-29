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
                FromageKind::Empty => writeln!(o)?,
                FromageKind::Comment(text) => writeln!(o, "; {}", text)?,
                FromageKind::Str { id, val } => {
                    if fromage.ignored {
                        writeln!(o, r#";s[{}] = "{}""#, id, val)?
                    } else {
                        writeln!(o, r#"s[{}] = "{}""#, id, val)?
                    }
                }
                FromageKind::Msg { id, val } => {
                    if fromage.ignored {
                        writeln!(o, r#";m[{}] = "{}""#, id, val)?
                    } else {
                        writeln!(o, r#"m[{}] = "{}""#, id, val)?
                    }
                }
            }
        }

        Ok(())
    }
}
