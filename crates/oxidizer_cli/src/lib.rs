use clap::{Args, Subcommand};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

mod error;
mod prelude;

pub use clap::Parser;
pub use prelude::*;

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
    command: Command,

    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
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
struct AnalyzeArgs {
    /// Input file containing benchmark data
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
    /// Output file for analysis results
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct BenchmarkArgs {
    /// Files to benchmark with their types
    #[arg(required = true, num_args = 1.., value_parser = parse_file_with_type)]
    files: Vec<FileWithType>,
    /// Number of iterations
    #[arg(short, long, default_value = "1")]
    iterations: u32,
    /// Output file for benchmark results
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum DaemonAction {
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
struct FileWithType {
    path: PathBuf,
    file_type: FileType,
}

#[derive(Debug, Clone)]
enum FileType {
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

pub struct Oxidizer {
    verbose: bool,
    command: Command,
    config: Option<PathBuf>,
    build_system: BuildSystem,
}

impl Oxidizer {
    pub fn new(cli: Cli) -> Self {
        Self {
            config: cli.config,
            command: cli.command,
            verbose: cli.verbose,
            build_system: BuildSystem::new(),
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Language {
    C,
    Cpp,
    Rust,
    CMake,
}

#[derive(Clone)]
enum CompilationStep {
    Compile(String),
    Output(String),
    Optimize(u8),
    EmitLLVM,
    Shared,
    Build,
    Test,
    Configure(String, String),
    Install(String),
    Target(String),
}

struct CompilationRecipe {
    language: Language,
    steps: Vec<CompilationStep>,
}

impl CompilationRecipe {
    fn new(language: Language) -> Self {
        let steps = Vec::new();
        Self { language, steps }
    }

    fn add_step(mut self, step: CompilationStep) -> Self {
        self.steps.push(step);
        self
    }
}

trait Compiler {
    fn execute_step(&self, step: &CompilationStep) -> Result<()>;
}

struct Clang;
impl Compiler for Clang {
    fn execute_step(&self, step: &CompilationStep) -> Result<()> {
        match step {
            CompilationStep::Compile(source) => println!("Clang: Compiling {}", source),
            CompilationStep::Output(output) => println!("Clang: Output to {}", output),
            CompilationStep::Optimize(level) => println!("Clang: Optimizing at level {}", level),
            CompilationStep::EmitLLVM => println!("Clang: Emitting LLVM IR"),
            CompilationStep::Shared => println!("Clang: Creating shared library"),
            _ => return Err(Error::Static("Unsupported step for Clang")),
        }
        Ok(())
    }
}

struct Cargo;
impl Compiler for Cargo {
    fn execute_step(&self, step: &CompilationStep) -> Result<()> {
        match step {
            CompilationStep::Compile(source) => println!("Cargo: Building {}", source),
            CompilationStep::Build => println!("Cargo: Building project"),
            CompilationStep::Test => println!("Cargo: Running tests"),
            CompilationStep::EmitLLVM => println!("Cargo: Emitting LLVM IR"),
            _ => return Err(Error::Static("Unsupported step for Cargo")),
        }
        Ok(())
    }
}

struct CMake;
impl Compiler for CMake {
    fn execute_step(&self, step: &CompilationStep) -> Result<()> {
        match step {
            CompilationStep::Configure(source, build) => {
                println!("CMake: Configuring {} to {}", source, build)
            }
            CompilationStep::Build => println!("CMake: Building project"),
            CompilationStep::Install(dir) => println!("CMake: Installing to {}", dir),
            CompilationStep::Target(target) => println!("CMake: Setting target to {}", target),
            _ => return Err(Error::Static("Unsupported step for CMake")),
        }
        Ok(())
    }
}

struct BuildSystem {
    compilers: HashMap<Language, Box<dyn Compiler>>,
}

impl BuildSystem {
    fn new() -> Self {
        let mut compilers: HashMap<Language, Box<dyn Compiler>> = HashMap::new();
        compilers.insert(Language::Rust, Box::new(Cargo));
        compilers.insert(Language::C, Box::new(Clang));
        compilers.insert(Language::Cpp, Box::new(Clang));
        compilers.insert(Language::CMake, Box::new(CMake));
        Self { compilers }
    }

    fn compile(&self, recipe: CompilationRecipe) -> Result<()> {
        let CompilationRecipe { language, .. } = recipe;

        let compiler = self
            .compilers
            .get(&language)
            .ok_or_else(|| Error::Generic(format!("Unsupported language: {:?}", language)))?;

        for step in &recipe.steps {
            compiler.execute_step(step)?;
        }
        Ok(())
    }
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let oxidizer = Oxidizer::new(cli);

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
