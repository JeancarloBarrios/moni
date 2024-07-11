use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("io error")]
    IOError(io::Error),

    #[error("parse error reason: {0}")]
    ParsingError(String),

    #[error("pdf error")]
    PdfError(lopdf::Error),

    #[error("unsuported file type error")]
    UnsuportedFileType,
}
