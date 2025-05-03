use tokio::time::error::Elapsed;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Compilation error: {message}")]
    Compilation { message: String },

    #[error("Runtime error: {message}")]
    Runtime { message: String },

    #[error("The process exceed time limit")]
    Timeout,

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::IO(std::io::Error::other(error))
    }
}

impl From<libseccomp::error::SeccompError> for Error {
    fn from(error: libseccomp::error::SeccompError) -> Self {
        Self::IO(std::io::Error::other(error.to_string()))
    }
}

impl From<cgroups_rs::error::Error> for Error {
    fn from(error: cgroups_rs::error::Error) -> Self {
        Self::IO(std::io::Error::other(error))
    }
}

impl From<Elapsed> for Error {
    fn from(_: Elapsed) -> Self {
        Self::Timeout
    }
}
