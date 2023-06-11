#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    RsaError(rsa::Error),
    RsaPkcs1Error(rsa::pkcs1::Error),
    SerdeJsonError(serde_json::Error),
    RawBytesReadError
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
error!(rsa::Error, Error::RsaError);
error!(rsa::pkcs1::Error, Error::RsaPkcs1Error);
error!(serde_json::Error, Error::SerdeJsonError);

pub type Result<T> = std::result::Result<T, Error>;
