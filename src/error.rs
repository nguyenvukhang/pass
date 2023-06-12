#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SerdeJsonError(serde_json::Error),
    RawBytesReadError,
    GpgIdNotFound,
    DataFileNotFound,
}

macro_rules! error {
    ($from:path, $to:path) => {
        impl From<$from> for Error {
            fn from(e: $from) -> Self {
                $to(e)
            }
        }
    };
}

error!(std::io::Error, Error::IoError);
error!(serde_json::Error, Error::SerdeJsonError);

pub type Result<T> = std::result::Result<T, Error>;
