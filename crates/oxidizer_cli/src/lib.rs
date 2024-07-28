use std::path::PathBuf;

mod build_system;
mod cli;
mod commands;
mod error;
mod prelude;

use cli::{Cli, Command, Parser};
pub use prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Ctx;

pub struct Oxidizer {
    verbose: bool,
    command: Command,
    config: Option<PathBuf>,
}

impl Oxidizer {
    pub fn new(cli: Cli) -> Self {
        Self {
            config: cli.config,
            command: cli.command,
            verbose: cli.verbose,
        }
    }

    fn enable_logging(&self) {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();
    }

    pub fn run(&self, ctx: Ctx) -> Result<()> {
        if self.verbose {
            self.enable_logging()
        };

        self.command.exec(ctx)
    }
}

pub fn run() -> Result<()> {
    let ctx = Ctx;
    let cli = Cli::parse();
    let oxi = Oxidizer::new(cli);

    oxi.run(ctx)
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
