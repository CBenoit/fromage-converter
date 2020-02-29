use std::borrow::Cow;

#[derive(Debug)]
pub enum FromageKind<'a> {
    Empty,
    Comment(Cow<'a, str>),
    Str { id: u64, val: Cow<'a, str> },
    Msg { id: u64, val: Cow<'a, str> },
}

/// Abstract intermediate format representation
///
/// This is called Fromage as in Fromage Converter.
#[derive(Debug)]
pub struct Fromage<'a> {
    pub kind: FromageKind<'a>,
    pub ignored: bool,
}

impl<'a> Fromage<'a> {
    pub fn empty() -> Self {
        Self {
            kind: FromageKind::Empty,
            ignored: false,
        }
    }

    pub fn comment<S: Into<Cow<'a, str>>>(val: S) -> Self {
        Self {
            kind: FromageKind::Comment(val.into()),
            ignored: false,
        }
    }

    pub fn str<S: Into<Cow<'a, str>>>(id: u64, val: S) -> Self {
        Self {
            kind: FromageKind::Str {
                id,
                val: val.into(),
            },
            ignored: false,
        }
    }

    pub fn msg<S: Into<Cow<'a, str>>>(id: u64, val: S) -> Self {
        Self {
            kind: FromageKind::Msg {
                id,
                val: val.into(),
            },
            ignored: false,
        }
    }
}
