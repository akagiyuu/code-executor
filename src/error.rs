pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Compilation error: {message}")]
    Compilation { message: String },
    
    #[error("Runtime error: {message}")]
    Runtime { message: String },
    
    #[error("IO error: {0}")] 
    IO(#[from] std::io::Error),
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::IO(std::io::Error::other(error))
    }
}

impl From<nix::errno::Errno> for Error {
    fn from(value: nix::errno::Errno) -> Self {
        Self::IO(std::io::Error::from(value))
    }
}

impl From<libseccomp::error::SeccompError> for Error {
    fn from(value: libseccomp::error::SeccompError) -> Self {
        Self::IO(std::io::Error::other(value.to_string()))
    }
}
