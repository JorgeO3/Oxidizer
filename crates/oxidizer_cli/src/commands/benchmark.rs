#![allow(unused)]

use std::fs;
use std::{path::PathBuf, str::FromStr};

use clap::Args;
use struct_iterable::Iterable;

use crate::{prelude::*, Ctx};

#[derive(Args, Debug, Iterable, Clone)]
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

#[derive(Debug, Clone)]
pub struct BenchmarkTarget {
    pub path: PathBuf,
    pub tool: BuilderSystem,
    pub mode: ProjectType,
    pub compiler_flags: Option<Vec<String>>,
}
impl FromStr for BenchmarkTarget {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split(':');

        let path = PathBuf::from(parts.next().ok_or("Missing path")?);
        let mode = ProjectType::from_str(parts.next().ok_or("Missing mode")?)?;
        let tool = BuilderSystem::from_str(parts.next().ok_or("Missing tool")?)?;

        let compiler_flags = parts
            .next()
            .map(|flags| {
                flags
                    .split(',')
                    .filter(|&s| !s.is_empty())
                    .map(String::from)
                    .collect::<Vec<_>>()
            })
            .filter(|v| !v.is_empty());

        if parts.next().is_some() {
            return Err("Too many parts in the input string".into());
        }

        Ok(Self {
            path,
            mode,
            tool,
            compiler_flags,
        })
    }
}

fn parse_benchmark_target(s: &str) -> std::result::Result<BenchmarkTarget, String> {
    BenchmarkTarget::from_str(s)
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

fn parse_duration(s: &str) -> std::result::Result<TimeUnit, String> {
    TimeUnit::from_str(s)
}

#[derive(Clone, Debug)]
pub enum ProjectType {
    Standalone,
    Workspace,
}
impl FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "s" | "standalone" => Ok(ProjectType::Standalone),
            "w" | "workspace" => Ok(ProjectType::Workspace),
            _ => Err(format!("Unsupported project type: {}", s)),
        }
    }
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

struct BenchmarkResult;

pub fn run(args: &BenchmarkArgs, ctx: Ctx) -> Result<()> {
    Exec::exec(args, ctx)
}

struct Exec {
    args: BenchmarkArgs,
    ctx: Ctx,
    results: Vec<BenchmarkResult>,
}
impl Exec {
    pub fn new(args: &BenchmarkArgs, ctx: Ctx) -> Self {
        Self {
            ctx,
            args: args.clone(),
            results: Vec::new(),
        }
    }

    pub fn exec(args: &BenchmarkArgs, _ctx: Ctx) -> Result<()> {
        for target in &args.targets {
            use BuilderSystem::*;
            use ProjectType::*;

            match (&target.tool, &target.mode) {
                (Cargo, Standalone) => Self::cargo_standalone(target),
                (Cargo, Workspace) => Self::cargo_workspace(target),
                (Cmake, Standalone) => todo!(),
                (Cmake, Workspace) => todo!(),
                (Clang, Standalone) => todo!(),
                (Clang, Workspace) => todo!(),
                (Gcc, Standalone) => todo!(),
                (Gcc, Workspace) => todo!(),
            }?;
        }
        Ok(())
    }

    fn cargo_workspace(target: &BenchmarkTarget) -> Result<()> {
        let path = &target.path;
        let dir = fs::read_dir(path)?;

        let (mut cargo_toml, mut gitignore) = (false, false);

        for entry in dir {
            let entry = entry?;
            let path = entry.file_name();

            match path.to_str() {
                Some("Cargo.toml") => cargo_toml = true,
                Some(".gitignore") => gitignore = true,
                _ => {}
            }
        }

        if cargo_toml {
            return Err(Error::Static("No Cargo.toml found in the workspace"));
        }

        todo!("cargo_workspace")
    }

    fn cargo_standalone(target: &BenchmarkTarget) -> Result<()> {
        todo!("cargo_standalone")
    }

    fn validate_cargo_manifest() {}

    fn cmake(target: &BenchmarkTarget) -> Result<()> {
        todo!("cmake")
    }
    fn clang(target: &BenchmarkTarget) -> Result<()> {
        todo!("clang")
    }
    fn gcc(target: &BenchmarkTarget) -> Result<()> {
        todo!("gcc")
    }

    fn read_dir(path: &PathBuf) -> Result<()> {
        let dir = fs::read_dir(path)?;

        for entry in dir {
            let entry = entry?;
            let path = entry.file_name();
            println!("{:?}", path);
        }
        Ok(())
    }
}
