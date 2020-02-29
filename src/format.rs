use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Format {
    ATools,
    Csv,
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "atools" => Ok(Format::ATools),
            "csv" => Ok(Format::Csv),
            _ => Err("invalid format name ; available formats are: atools, csv"),
        }
    }
}
