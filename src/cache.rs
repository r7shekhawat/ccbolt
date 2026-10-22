use anyhow::{Context, Result};
use blake3::Hasher;
use std::fs;
use std::path::{Path, PathBuf};

/// Where cached objects live. Content-addressed: hash → directory.
pub fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ccbolt")
}

/// Hash the preprocessed source + relevant compiler flags into a cache key.
/// TODO(week 3): add direct mode — hash raw source + include deps instead
/// of paying for preprocessing every time.
pub fn cache_key(compiler: &str, preprocessed: &[u8], args: &[String]) -> String {
    let mut h = Hasher::new();
    h.update(compiler.as_bytes());
    // Only flags that affect codegen should be hashed (filter in week 2).
    for a in args {
        h.update(a.as_bytes());
        h.update(&[0]);
    }
    h.update(preprocessed);
    h.finalize().to_hex().to_string()
}

pub fn lookup(key: &str) -> Option<PathBuf> {
    let p = cache_dir().join(key).join("output.o");
    p.exists().then_some(p)
}

pub fn store(key: &str, object: &Path) -> Result<()> {
    let dir = cache_dir().join(key);
    fs::create_dir_all(&dir)?;
    fs::copy(object, dir.join("output.o")).context("storing cached object")?;
    Ok(())
}

pub fn clean() -> Result<()> {
    let dir = cache_dir();
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    println!("cache cleared");
    Ok(())
}
