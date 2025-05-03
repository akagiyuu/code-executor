use std::{process::ExitStatus, time::Duration};

#[derive(Debug)]
pub struct Metrics {
    pub exit_status: ExitStatus,
    pub run_time: Duration,
    pub stdout: String,
    pub stderr: String,
}
