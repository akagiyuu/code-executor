use std::{ffi::c_int, time::Duration};

use nix::libc::{self, rusage};

pub fn get_default_rusage() -> rusage {
    rusage {
        ru_utime: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    }
}

#[derive(Debug, Default)]
pub struct Rusage {
    pub user_time: Duration,
    pub system_time: Duration,
    pub max_rss: i64,
    pub page_faults: i64,
    pub involuntary_context_switches: i64,
    pub voluntary_context_switches: i64,
}

impl From<rusage> for Rusage {
    fn from(rusage: rusage) -> Self {
        Self {
            user_time: Duration::new(
                rusage.ru_utime.tv_sec as u64,
                rusage.ru_utime.tv_usec as u32 * 1000,
            ),
            system_time: Duration::new(
                rusage.ru_stime.tv_sec as u64,
                rusage.ru_stime.tv_usec as u32 * 1000,
            ),
            max_rss: rusage.ru_maxrss,
            page_faults: rusage.ru_majflt,
            involuntary_context_switches: rusage.ru_nivcsw,
            voluntary_context_switches: rusage.ru_nvcsw,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    CompileTimeError(String),
    RunTimeError(String),
    Success(String),
}

impl Default for Output {
    fn default() -> Self {
        Output::CompileTimeError(Default::default())
    }
}

#[derive(Debug, Default)]
pub struct Metrics {
    pub exit_status: c_int,
    pub exit_signal: c_int,
    pub exit_code: c_int,
    pub real_time_cost: Duration,
    pub resource_usage: Rusage,
    pub output: Output,
}
