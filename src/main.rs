mod cache;
mod cli;
mod stats;
mod wrapper;

use anyhow::Result;

fn main() -> Result<()> {
    match cli::parse().command {
        cli::Commands::Wrap { compiler, args } => wrapper::run(&compiler, &args),
        cli::Commands::Stats => stats::print(),
        cli::Commands::Clean => cache::clean(),
    }
}
