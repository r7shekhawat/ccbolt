use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ccbolt", about = "A fast compiler cache for C/C++ builds", version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Wrap a compiler invocation (used via CC/CXX or build system config)
    Wrap {
        /// Compiler to run, e.g. gcc
        compiler: String,
        /// Arguments passed to the compiler
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Show cache statistics
    Stats,
    /// Remove all cached entries
    Clean,
}

pub fn parse() -> Args {
    Args::parse()
}
