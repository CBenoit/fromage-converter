use std::fmt;

pub type Result<T> = std::result::Result<T, LactoseIntolerance>;

#[derive(Debug)]
pub enum LactoseIntolerance {
    Io(std::io::Error),
    Static(&'static str),
    Dyn(String),
}

impl std::error::Error for LactoseIntolerance {}

impl fmt::Display for LactoseIntolerance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LactoseIntolerance::Io(e) => write!(f, "io error: {}", e),
            LactoseIntolerance::Static(msg) => write!(f, "{}", msg),
            LactoseIntolerance::Dyn(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for LactoseIntolerance {
    fn from(e: std::io::Error) -> LactoseIntolerance {
        Self::Io(e)
    }
}
