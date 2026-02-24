pub mod experiment;
pub mod model;

use crate::cli::command::experiment::ExperimentArgs;
use crate::cli::command::model::ModelArgs;
use crate::cli::to_args::ToArgs;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Subcommand, PartialEq, Debug)]
pub enum CliCommand {
    /// Operations for managing burnt apple models
    Model(ModelArgs),
    /// Experiment workflow helpers
    Experiment(ExperimentArgs),
}

impl CliCommand {
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            CliCommand::Model(args) => args.invoke(),
            CliCommand::Experiment(args) => args.invoke(),
        }
    }
}

impl ToArgs for CliCommand {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            CliCommand::Model(model_args) => {
                args.push("model".into());
                args.extend(model_args.to_args());
            }
            CliCommand::Experiment(experiment_args) => {
                args.push("experiment".into());
                args.extend(experiment_args.to_args());
            }
        }
        args
    }
}
