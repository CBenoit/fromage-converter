use crate::{
    composer::FromageCook,
    error::{LactoseIntolerance, Result},
    parser::FromageMaker,
};
use env_logger::{Builder, Env};
use format::Format;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
    time::Instant,
};
use structopt::StructOpt;

mod composer;
mod error;
mod format;
mod fromage;
mod parser;

#[derive(Debug, StructOpt)]
#[structopt(name = "Fromage Converter", about = "Converts translation files")]
struct Opt {
    /// CSV separator
    #[structopt(default_value = ",", short, long)]
    csv_separator: char,

    /// Name of column containing translations to export from CSV file
    #[structopt(short, long, default_value("TRANSLATION"))]
    translation_column: String,

    /// Input file
    #[structopt(parse(from_os_str), short, long)]
    input: PathBuf,

    /// Input format
    #[structopt(default_value = "atools", long = "if")]
    input_format: Format,

    /// Output file
    #[structopt(default_value = "out.txt", parse(from_os_str), short, long)]
    output: PathBuf,

    /// Output format
    #[structopt(default_value = "csv", long = "of")]
    output_format: Format,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    Builder::from_env(Env::default().default_filter_or("info")).init();

    if opt.input == opt.output {
        return Err(LactoseIntolerance::Static(
            "input file can't be the same as output file",
        ));
    }

    log::info!(
        "{} ({:?}) => {} ({:?})",
        opt.input.display(),
        opt.input_format,
        opt.output.display(),
        opt.output_format,
    );

    let i = BufReader::new(File::open(opt.input)?);
    let o = BufWriter::new(File::create(opt.output)?);

    let start = Instant::now();

    match (opt.input_format, opt.output_format) {
        (Format::ATools, Format::Csv) => {
            let maker = parser::AToolsMaker::default();
            let parser = maker.process(i)?;
            let composer = composer::CsvComposer {
                sep: opt.csv_separator,
            };
            composer.process(parser, o)?;
        }
        (Format::Csv, Format::ATools) => {
            let maker = parser::CsvMaker::new(opt.csv_separator, opt.translation_column);
            let parser = maker.process(i)?;
            let composer = composer::AToolsComposer;
            composer.process(parser, o)?;
        }
        (in_f, out_f) => {
            return Err(LactoseIntolerance::Dyn(format!(
                "unsupported conversion: {:?} => {:?}",
                in_f, out_f
            )))
        }
    }

    log::info!("done in {}ms", start.elapsed().as_millis());

    Ok(())
}
