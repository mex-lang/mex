use std::path::Path;
use crate::ast::error::Error;

pub enum Source {
    File(Box<Path>),
}

impl<'input> Source {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Source {
        Source::File(path.as_ref().into())
    }

    pub fn get_name(&self) -> &str {
        match self {
            Source::File(path) => path.file_stem().unwrap().to_str().unwrap(),
        }
    }

    pub fn read(&'input self) -> Result<String, Error<'input>> {
        match self {
            Source::File(ref path) => Ok(std::fs::read_to_string(path)?)
        }
    }
}