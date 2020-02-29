use crate::{error::Result, parser::FromagemakingProcess};
use std::io::Write;

pub mod atools;
pub mod csv;

pub use atools::AToolsComposer;
pub use csv::CsvComposer;

pub trait FromageCook {
    fn process<Process, Writer>(self, process: Process, o: Writer) -> Result<()>
    where
        Process: FromagemakingProcess,
        Writer: Write;
}
