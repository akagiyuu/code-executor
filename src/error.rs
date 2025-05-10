pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Compilation error: {message}")]
    Compilation { message: String },

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Failed to create cgroup: {0}")]
    Cgroup(#[from] cgroups_rs::error::Error),
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::IO(std::io::Error::other(error))
    }
}
