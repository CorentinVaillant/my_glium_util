mod wavefront_parser;
mod wavefront_struct;

use std::{fmt::Display, path::Path};

pub use wavefront_parser::*;
pub use wavefront_struct::*;

pub trait WavefrontParsable {
    fn read_from_obj<P: AsRef<Path> >(path: P) -> Result<Self, WavefrontError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum WavefrontError {
    IOError(std::io::Error),
    InvalidLineData(String),
    InvalidFaceData(String),
}

impl Display for WavefrontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WavefrontError::IOError(e) => write!(f,"IO error :{}",e),
            WavefrontError::InvalidLineData(line) => write!(f, "invalid line data has been found :{}",line),
            WavefrontError::InvalidFaceData(line) => write!(f, "invalid face data has been found :{}",line),
        }
    }
}

impl std::error::Error for WavefrontError {}
