use crate::cli::command::experiment::experiment_list_command::ExperimentListArgs;
use crate::cli::command::experiment::experiment_run_command::ExperimentRunArgs;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Subcommand, Clone, Arbitrary, PartialEq, Debug)]
pub enum ExperimentCommand {
    /// List experiments and their metadata
    List(ExperimentListArgs),
    /// Run an experiment by name
    Run(ExperimentRunArgs),
}

impl ExperimentCommand {
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            ExperimentCommand::List(args) => args.invoke(),
            ExperimentCommand::Run(args) => args.invoke(),
        }
    }
}

impl ToArgs for ExperimentCommand {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            ExperimentCommand::List(list_args) => {
                args.push("list".into());
                args.extend(list_args.to_args());
            }
            ExperimentCommand::Run(run_args) => {
                args.push("run".into());
                args.extend(run_args.to_args());
            }
        }
        args
    }
}
