use std::env;
use std::hash::{DefaultHasher, Hash, Hasher as _};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

pub fn generate_unique_path(code: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));
    let unique_id = hash((now, code)).to_string();
    let mut path = env::temp_dir();
    path.push(unique_id);

    path
}

