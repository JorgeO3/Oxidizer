use clap::Subcommand;

use crate::{prelude::*, Ctx};

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

pub fn run(action: &DaemonAction, _ctx: Ctx) -> Result<()> {
    Exec::exec(action)
}

struct Exec;
impl Exec {
    pub fn exec(action: &DaemonAction) -> Result<()> {
        println!("Analyze: {:?}", action);
        Ok(())
    }
}
