use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(std::io::Error),
    #[error("Image Error: {0}")]
    ImageError(image::error::ImageError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<image::error::ImageError> for Error {
    fn from(err: image::error::ImageError) -> Self {
        Self::ImageError(err)
    }
}
