use clap::{Args, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;

pub use clap::Parser;

const EXAMPLES: &str = "\x1b[1mExamples:\x1b[0m
oxidizer benchmark file.rs:rust --iterations 5
oxidizer benchmark file1.cpp:cpp file2.rs:rust --output results.txt
oxidizer analyze input.json --output analysis.txt
";

#[derive(Parser, Debug)]
#[command(
    name = "Oxidizer",
    version,
    about = "Distributed Adaptive Benchmarking System",
    long_about = None,
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

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Analyze benchmark results
    Analyze(AnalyzeArgs),
    /// Run benchmarks and compare programs
    Benchmark(BenchmarkArgs),
    /// Manage the benchmarking daemon
    Daemon {
        #[command(subcommand)]
        action: DaemonAction,
    },
}

#[derive(Args, Debug)]
pub struct AnalyzeArgs {
    /// Input file containing benchmark data
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,
    /// Output file for analysis results
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct BenchmarkArgs {
    /// Files to benchmark with their types
    #[arg(required = true, num_args = 1.., value_parser = parse_file_with_type)]
    pub files: Vec<FileWithType>,
    /// Number of iterations
    #[arg(short, long, default_value = "1")]
    pub iterations: u32,
    /// Output file for benchmark results
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum DaemonAction {
    /// Start the benchmarking daemon
    Start,
    /// Stop the benchmarking daemon
    Stop,
    /// Restart the benchmarking daemon
    Restart,
    /// Check the status of the benchmarking daemon
    Status,
}

#[derive(Debug, Clone)]
pub struct FileWithType {
    pub path: PathBuf,
    pub file_type: FileType,
}

#[derive(Debug, Clone)]
pub enum FileType {
    Rust,
    C,
    Cpp,
    Go,
}

impl FromStr for FileType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(FileType::Rust),
            "cpp" => Ok(FileType::Cpp),
            "c" => Ok(FileType::C),
            "go" => Ok(FileType::Go),
            _ => Err(format!("Unsupported file type: {}", s)),
        }
    }
}

fn parse_file_with_type(s: &str) -> std::result::Result<FileWithType, String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err("File must be specified as 'path:type'".to_string());
    }
    let path = PathBuf::from(parts[0]);
    let file_type = FileType::from_str(parts[1])?;
    Ok(FileWithType { path, file_type })
}
