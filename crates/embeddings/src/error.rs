use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("parse error")]
    ParsingError,

    #[error("pdf error")]
    PdfError(lopdf::Error),
}
