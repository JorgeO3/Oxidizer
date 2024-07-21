use std::path::PathBuf;

mod build_system;
mod cli;
mod commands;
mod error;
mod prelude;

use build_system::{CompilationRecipe, CompilationStep, Compiler, Language};
use cli::{AnalyzeArgs, BenchmarkArgs, BuilderSystem, Cli, Command, DaemonAction, Parser};
pub use prelude::*;

pub struct Oxidizer {
    verbose: bool,
    command: Command,
    config: Option<PathBuf>,
    compiler: Compiler,
}

impl Oxidizer {
    pub fn new(cli: Cli, compiler: Compiler) -> Self {
        Self {
            config: cli.config,
            command: cli.command,
            verbose: cli.verbose,
            compiler,
        }
    }

    pub fn run(&self) -> Result<()> {
        if self.verbose {
            self.enable_logging()
        };

        match &self.command {
            Command::Analyze(args) => self.analyze(args),
            Command::Benchmark(args) => self.benchmark(args),
            Command::Daemon { action } => self.daemon(action),
        }
    }

    fn enable_logging(&self) {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();
    }

    fn benchmark(&self, args: &BenchmarkArgs) -> Result<()> {
        // for target in &args.targets {
        //     match target.tool {
        //         BuilderSystem::Cargo => todo!(),
        //         BuilderSystem::Cmake => todo!(),
        //         BuilderSystem::Clang => todo!(),
        //         BuilderSystem::Gcc => todo!(),
        //     }
        // }
        todo!()
    }

    fn analyze(&self, args: &AnalyzeArgs) -> Result<()> {
        println!("Analyzing from {:?} to {:?}", args.input, args.output);
        Ok(())
    }

    fn daemon(&self, action: &DaemonAction) -> Result<()> {
        match action {
            DaemonAction::Start => println!("Starting daemon"),
            DaemonAction::Stop => println!("Stopping daemon"),
            DaemonAction::Restart => println!("Restarting daemon"),
            DaemonAction::Status => println!("Checking daemon status"),
        }
        Ok(())
    }
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    Ok(())
}

// Oxidizer functions
// Parse the command line arguments
// Authenticate the client the first time
// compile to LLVM IR
// Send the LLVM IR to the server (daemon)
// Check the status of the server
// Create the benchmarking configurations for the server
// Receive the results from the server
// Package single files or entire projects

// Examples commands
// oxi -v benchmark main.rs:cargo:O3,target-cpu=native main.cpp:cmake main.c:gcc --flamegraph --call-graph --perf-metrics --analyze-latency
// oxi -v benchmark path1:mode1:tool1:args1 path2:mode2:tool2:args2 ... pathN:modeN:toolN:argsN  flag1,flag2,flag3,...,flagN
