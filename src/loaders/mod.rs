use std::io;
use ultraviolet::Vec3;

pub mod obj;

#[derive(Debug)]
pub enum LoadError {
    IOError(io::Error),
    ParseError(String)
}

impl From<io::Error> for LoadError {
    fn from(value: io::Error) -> Self {
        LoadError::IOError(value)
    }
}

impl From<std::num::ParseFloatError> for LoadError {
    fn from(value: std::num::ParseFloatError) -> Self {
        LoadError::ParseError(value.to_string())
    }
}

impl From<std::num::ParseIntError> for LoadError {
    fn from(value: std::num::ParseIntError) -> Self {
        LoadError::ParseError(value.to_string())
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ERROR] {}", match self {
            LoadError::IOError(_) => "IOError",
            LoadError::ParseError(_) => "ParseError",
        })
    }
}

pub trait LoaderTrait {
    fn load(&mut self, path: &str) -> Result<(), LoadError>;
    
    fn parse_vertex(line: &[&str]) -> Result<Vec3, LoadError>;
    fn parse_face(line: &[&str]) -> Result<[u32;3], LoadError>;
}