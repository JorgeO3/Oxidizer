use clap::Args;
use std::path::PathBuf;

use crate::{prelude::*, Ctx};

#[derive(Args, Debug)]
pub struct AnalyzeArgs {
    /// Input file containing benchmark data
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,
    /// Output file for analysis results
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

pub fn run(args: &AnalyzeArgs, ctx: Ctx) -> Result<()> {
    Exec::exec(args)
}

struct Exec;
impl Exec {
    pub fn exec(args: &AnalyzeArgs) -> Result<()> {
        println!("Analyze: {:?}", args);
        Ok(())
    }
}
