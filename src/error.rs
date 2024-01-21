use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Generic(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Pdf(#[from] lopdf::Error),
}

impl From<String> for Error {
    fn from(v: String) -> Self {
        Self::Generic(v)
    }
}
