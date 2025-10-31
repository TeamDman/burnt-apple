pub mod experiment_command;
pub mod experiment_list_command;
pub mod experiment_run_command;

mod experiment;
pub use experiment::*;

use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;

use crate::cli::command::experiment::experiment_command::ExperimentCommand;
use crate::cli::to_args::ToArgs;

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ExperimentArgs {
    #[clap(subcommand)]
    pub command: ExperimentCommand,
}

impl ExperimentArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        self.command.invoke()
    }
}

impl ToArgs for ExperimentArgs {
    fn to_args(&self) -> Vec<OsString> {
        self.command.to_args()
    }
}
