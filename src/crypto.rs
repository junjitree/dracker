use tracing::error;

#[derive(Debug)]
pub enum Error {
    Base64Decode,
    Encryption,
    Json,
    MalformedData,
    Utf8,
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        error!("{err}");
        Error::Base64Decode
    }
}

impl From<aes_gcm::Error> for Error {
    fn from(err: aes_gcm::Error) -> Self {
        error!("{err}");
        Error::Encryption
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        error!("{err}");
        Error::Json
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        error!("{err}");
        Error::Utf8
    }
}
