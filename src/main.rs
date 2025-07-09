// src/main.rs
/*
 * Main executable for QuantumAiToolkitSDK
 */

use clap::Parser;
use quantumaitoolkitsdk::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumAiToolkitSDK - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
