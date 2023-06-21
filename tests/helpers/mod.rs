mod settings;
use std::path::PathBuf;

pub use settings::{RedisSettings, ScopedSettings, ServerSettings, Settings};

pub fn get_test_file(path: &str) -> PathBuf {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    base_path.join("tests/assets").join(path)
}
