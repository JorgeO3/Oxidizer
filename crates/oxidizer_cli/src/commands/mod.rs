use crate::{prelude::*, Ctx};
use clap::Subcommand;

mod analyze;
mod benchmark;
mod daemon;

use analyze::AnalyzeArgs;
use benchmark::BenchmarkArgs;
use daemon::DaemonAction;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Analyze benchmark results
    Analyze(AnalyzeArgs),
    /// Run benchmarks and compare programs
    Benchmark(Box<BenchmarkArgs>),
    /// Manage the benchmarking daemon
    Daemon {
        #[command(subcommand)]
        action: DaemonAction,
    },
}

impl Command {
    pub fn exec(&self, ctx: Ctx) -> Result<()> {
        match self {
            Command::Analyze(args) => analyze::run(args, ctx),
            Command::Benchmark(args) => benchmark::run(args, ctx),
            Command::Daemon { action } => daemon::run(action, ctx),
        }
    }
}
