use std::borrow::Cow;
use std::path::Path;

pub enum Source {
    File(Box<Path>),
    String(String),
}

impl Source {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Source {
        Source::File(path.as_ref().into())
    }

    pub fn from_str(text: &str) -> Source {
        Source::String(String::from(text))
    }

    pub fn get_name(&self) -> Cow<str> {
        match self {
            Source::File(path) => Cow::Borrowed(path.file_stem().unwrap().to_str().unwrap()),
            _ => Cow::Owned(String::default()),
        }
    }

    pub fn read(&self) -> crate::Result<Cow<str>> {
        match self {
            Source::File(path) => Ok(Cow::Owned(std::fs::read_to_string(path)?)),
            Source::String(s) => Ok(Cow::Borrowed(s)),
        }
    }
}