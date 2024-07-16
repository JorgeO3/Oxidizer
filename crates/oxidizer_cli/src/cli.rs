use clap::{Args, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;
use struct_iterable::Iterable;

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

#[derive(Debug, Clone)]
pub enum TimeUnit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}
impl FromStr for TimeUnit {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "s" => Ok(TimeUnit::Seconds),
            "ms" => Ok(TimeUnit::Milliseconds),
            "us" => Ok(TimeUnit::Microseconds),
            "ns" => Ok(TimeUnit::Nanoseconds),
            _ => Err(format!("Unsupported time unit: {}", s)),
        }
    }
}

#[derive(Args, Debug, Iterable)]
pub struct BenchmarkArgs {
    /// Files to benchmark with their types
    #[arg(required = true, num_args = 1.., value_parser = parse_benchmark_target)]
    pub targets: Vec<BenchmarkTarget>,

    /// Number of benchmark runs
    #[arg(short = 'r', long, default_value = "10")]
    pub runs: u32,

    /// Number of warmup runs
    #[arg(short = 'w', long, default_value = "0")]
    pub warmup: u32,

    /// Preparation command
    #[arg(long)]
    pub prepare: Option<String>,

    /// Cleanup command
    #[arg(long)]
    pub cleanup: Option<String>,

    /// Time unit for results
    #[arg(long, value_parser = parse_duration)]
    pub time_unit: Option<TimeUnit>,

    /// Export results to JSON
    #[arg(long)]
    pub export_json: Option<PathBuf>,

    /// Export results to Markdown
    #[arg(long)]
    pub export_markdown: Option<PathBuf>,

    /// Export results to CSV
    #[arg(long)]
    pub export_csv: Option<PathBuf>,

    /// Ignore command failures
    #[arg(long)]
    pub ignore_failure: bool,

    /// Timeout for each run (in seconds)
    #[arg(long)]
    pub timeout: Option<u64>,

    /// Measure memory usage
    #[arg(long)]
    pub measure_memory: bool,

    /// Show relative comparison
    #[arg(long)]
    pub relative_comparison: bool,

    /// Enable detailed performance metrics
    #[arg(long)]
    pub perf_metrics: bool,

    /// Perf events to record (comma-separated)
    #[arg(long, use_value_delimiter = true)]
    pub perf_events: Option<Vec<String>>,

    /// Perf sampling frequency (Hz)
    #[arg(long, default_value = "999")]
    pub sampling_frequency: u32,

    /// Generate flamegraph
    #[arg(long)]
    pub flamegraph: bool,

    /// Generate call graph
    #[arg(long)]
    pub call_graph: bool,

    /// Annotate source code with perf data
    #[arg(long)]
    pub annotate_source: bool,

    /// Enable system-wide profiling
    #[arg(long)]
    pub system_wide: bool,

    /// Analyze scheduler latency
    #[arg(long)]
    pub analyze_latency: bool,

    /// Custom perf record options
    #[arg(long)]
    pub perf_record_options: Option<String>,

    /// Custom perf report options
    #[arg(long)]
    pub perf_report_options: Option<String>,
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
pub struct BenchmarkTarget {
    pub path: PathBuf,
    pub tool: BuilderSystem,
    pub compiler_flags: Option<Vec<String>>,
}
impl FromStr for BenchmarkTarget {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() < 2 {
            return Err(
                "File must be specified as 'path:type[:opt_level][,flag1,flag2,...]'".to_string(),
            );
        }
        let path = PathBuf::from(parts[0]);
        let tool = BuilderSystem::from_str(parts[1])?;
        let mut compiler_flags = None;

        if parts.len() > 2 {
            let options: Vec<&str> = parts[2].split(',').collect();
            if options.len() > 1 {
                compiler_flags = Some(options.iter().map(|&s| s.to_string()).collect());
            }
        }

        Ok(BenchmarkTarget::new(path, tool, compiler_flags))
    }
}
impl BenchmarkTarget {
    pub fn new(path: PathBuf, tool: BuilderSystem, compiler_flags: Option<Vec<String>>) -> Self {
        Self {
            path,
            tool,
            compiler_flags,
        }
    }
}

fn parse_benchmark_target(s: &str) -> std::result::Result<BenchmarkTarget, String> {
    BenchmarkTarget::from_str(s)
}

#[derive(Debug, Clone)]
pub enum BuilderSystem {
    Cargo,
    Cmake,
    Clang,
    Gcc,
}

impl FromStr for BuilderSystem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cargo" => Ok(BuilderSystem::Cargo),
            "cmake" => Ok(BuilderSystem::Cmake),
            "clang" => Ok(BuilderSystem::Clang),
            "gcc" => Ok(BuilderSystem::Gcc),
            _ => Err(format!("Unsupported file type: {}", s)),
        }
    }
}

fn parse_file_with_type(s: &str) -> std::result::Result<BenchmarkTarget, String> {
    BenchmarkTarget::from_str(s)
}

fn parse_duration(s: &str) -> std::result::Result<TimeUnit, String> {
    TimeUnit::from_str(s)
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
