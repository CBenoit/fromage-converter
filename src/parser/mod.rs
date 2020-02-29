use crate::{error::Result, fromage::Fromage};
use std::io::BufRead;

pub mod atools;
pub mod csv;

pub use atools::AToolsMaker;
pub use csv::CsvMaker;

pub trait FromageMaker {
    type Process: FromagemakingProcess;
    type Reader: BufRead;

    fn process(self, reader: Self::Reader) -> Result<Self::Process>;
}

pub trait FromagemakingProcess {
    fn next_fromage(&mut self) -> Option<Result<Fromage>>;
}
