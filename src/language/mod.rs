mod cpp;
mod java;
mod python;
mod rust;

use nix::sys::resource::Resource;

use crate::sandbox::{RlimitConfig, SandboxConfig};

pub use cpp::CPP;
pub use java::JAVA;
pub use python::PYTHON;
pub use rust::RUST;

const DEFAULT_SANDBOX_CONFIG: SandboxConfig = SandboxConfig {
    rlimit_configs: &[
        RlimitConfig {
            resource: Resource::RLIMIT_STACK,
            soft_limit: 1024 * 1024 * 1024,
            hard_limit: 1024 * 1024 * 1024,
        },
        RlimitConfig {
            resource: Resource::RLIMIT_AS,
            soft_limit: 1024 * 1024 * 1024,
            hard_limit: 1024 * 1024 * 1024,
        },
        RlimitConfig {
            resource: Resource::RLIMIT_CPU,
            soft_limit: 60,
            hard_limit: 90,
        },
        RlimitConfig {
            resource: Resource::RLIMIT_FSIZE,
            soft_limit: 1024,
            hard_limit: 1024,
        },
    ],
    scmp_black_list: &["fork", "vfork"],
};
