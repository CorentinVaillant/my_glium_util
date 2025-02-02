mod wavefront_parser;
mod wavefront_struct;

use std::fmt::Display;

pub use wavefront_parser::*;
pub use wavefront_struct::*;

pub trait WavefrontParsable {
    fn read_from_obj(path: &std::path::Path) -> Result<Self, WavefrontError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum WavefrontError {
    CouldNotReadFile,
    InvalidFaceData,
}

impl Display for WavefrontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WavefrontError::CouldNotReadFile => write!(f, "could not read the file"),
            WavefrontError::InvalidFaceData => write!(f, "invalid face data has been found"),
        }
    }
}

impl std::error::Error for WavefrontError {}
