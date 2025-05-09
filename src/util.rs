use std::env;
use std::path::PathBuf;

use uuid::Uuid;

pub fn generate_unique_path() -> PathBuf {
    let id = Uuid::new_v4().to_string();
    let mut path = env::temp_dir();
    path.push(id);

    path
}
