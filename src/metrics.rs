use std::time::Duration;

#[derive(Debug)]
pub enum ExitStatus {
    Success,
    RuntimeError,
    Timeout,
}

impl From<std::process::ExitStatus> for ExitStatus {
    fn from(raw: std::process::ExitStatus) -> Self {
        if raw.success() {
            Self::Success
        } else {
            Self::RuntimeError
        }
    }
}

#[derive(Debug)]
pub struct Metrics {
    pub exit_status: ExitStatus,
    pub run_time: Duration,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
