use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

/// Intercept a compiler invocation: serve from cache, or compile and store.
///
/// v0.1 = preprocessor mode: run the preprocessor to get a stable hash input.
/// Fallback rule (week 2): if anything is unusual, just run the compiler.
pub fn run(compiler: &str, args: &[String]) -> Result<()> {
    // Find -o <output> to know where the .o goes.
    let out_path = find_output(args).context("no -o output argument found; passing through")?;

    // Preprocess to obtain hash input.
    let mut child = Command::new(compiler)
        .args(args)
        .arg("-E")
        .arg("-o")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawning {compiler}"))?;
    let mut preprocessed = Vec::new();
    child
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut preprocessed)?;
    let status = child.wait()?;
    if !status.success() {
        // Preprocessor failed: run the real compile so the user sees real errors.
        return run_real(compiler, args);
    }

    let key = crate::cache::cache_key(compiler, &preprocessed, args);
    if let Some(cached) = crate::cache::lookup(&key) {
        std::fs::copy(&cached, &out_path).context("restoring cached object")?;
        eprintln!("ccbolt: cached  {}", out_path.display());
        return Ok(());
    }

    run_real(compiler, args)?;
    crate::cache::store(&key, Path::new(&out_path))?;
    eprintln!("ccbolt: stored  {}", out_path.display());
    Ok(())
}

fn find_output(args: &[String]) -> Option<String> {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "-o" {
            return it.next().cloned();
        }
        if let Some(rest) = a.strip_prefix("-o") {
            if !rest.is_empty() {
                return Some(rest.to_string());
            }
        }
    }
    None
}

fn run_real(compiler: &str, args: &[String]) -> Result<()> {
    let status = Command::new(compiler)
        .args(args)
        .status()
        .with_context(|| format!("running {compiler}"))?;
    if !status.success() {
        anyhow::bail!("compiler exited with {status}");
    }
    Ok(())
}

use std::path::Path;
