use std::path::PathBuf;

mod build_system;
mod cli;
mod error;
mod prelude;

use build_system::{CompilationRecipe, CompilationStep};
use cli::{AnalyzeArgs, BenchmarkArgs, Cli, Command, DaemonAction, Parser};
pub use prelude::*;

pub struct Oxidizer {
    verbose: bool,
    command: Command,
    config: Option<PathBuf>,
    build_system: String,
}

impl Oxidizer {
    pub fn new(cli: Cli) -> Self {
        Self {
            config: cli.config,
            command: cli.command,
            verbose: cli.verbose,
            build_system: String::default(),
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
        for file in &args.files {
            println!("Benchmarking {:?} as {:?}", file.path, file.file_type);
        }
        Ok(())
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

    let cpp_recipe = CompilationRecipe::new("cpp")
        .add_step(CompilationStep::Compile("program.cpp".into()))
        .
        .add_step(CompilationStep::Output("program".into()));

    let oxidizer = Oxidizer::new(cli);
    let data: &[u8] = &[];

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
