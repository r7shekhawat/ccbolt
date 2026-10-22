use anyhow::Result;

/// TODO(week 1): maintain a stats file (hits/misses/bytes saved) in the
/// cache dir, increment it in cache::lookup/store, and render it here.
pub fn print() -> Result<()> {
    let dir = crate::cache::cache_dir();
    let mut entries = 0u64;
    let mut bytes = 0u64;
    if dir.exists() {
        for e in walkdir::WalkDir::new(&dir) {
            let e = e?;
            if e.file_type().is_file() {
                entries += 1;
                bytes += e.metadata()?.len();
            }
        }
    }
    println!("cache dir:  {}", dir.display());
    println!("entries:    {entries}");
    println!("size:       {} MiB", bytes / 1_048_576);
    Ok(())
}
