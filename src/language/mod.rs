mod cpp;
mod java;
mod python;
mod rust;

pub use cpp::CPP;
pub use java::JAVA;
pub use python::PYTHON;
pub use rust::RUST;

const DEFAULT_MAX_MEMORY: i64 = 1024 * 1024 * 1024;
const DEFAULT_MAX_CPU_PERCENTAGE: i64 = 50;
