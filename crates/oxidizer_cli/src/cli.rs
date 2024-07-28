use std::path::PathBuf;

pub use clap::Parser;

pub use crate::commands::Command;

const EXAMPLES: &str = "\x1b[1mExamples:\x1b[0m
oxidizer benchmark file.rs:rust --iterations 5
oxidizer benchmark file1.cpp:cpp file2.rs:rust --output results.txt
oxidizer analyze input.json --output analysis.txt
";

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    after_help = EXAMPLES,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

// #[derive(Debug, Default)]
// struct PerfMetrics {
//     cpu_cycles: u64,
//     instructions: u64,
//     cache_misses: u64,
//     branch_misses: u64,
//     page_faults: u64,
//     context_switches: u64,
//     cpu_migrations: u64,
//     io_operations: u64,
//     memory_bandwidth: f64,
//     frontend_stalls: u64,
//     backend_stalls: u64,
// }

// #[derive(Debug, Default)]
// struct HighPerformanceMetrics {
//     instructions: u64,
//     cpu_cycles: u64,
//     ipc: f64,
//     l1_cache_misses: u64,
//     llc_misses: u64,
//     branch_mispredictions: u64,
//     frontend_stall_cycles: u64,
//     backend_stall_cycles: u64,
//     flops: f64,
//     memory_bandwidth: f64,
//     page_faults: u64,
// }
