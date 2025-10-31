pub mod experiment;
pub mod model;

use crate::cli::command::experiment::ExperimentArgs;
use crate::cli::command::model::ModelArgs;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum Command {
    /// Operations for managing burnt apple models
    Model(ModelArgs),
    /// Experiment workflow helpers
    Experiment(ExperimentArgs),
}

impl Command {
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            Command::Model(args) => args.invoke(),
            Command::Experiment(args) => args.invoke(),
        }
    }
}

impl ToArgs for Command {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            Command::Model(model_args) => {
                args.push("model".into());
                args.extend(model_args.to_args());
            }
            Command::Experiment(experiment_args) => {
                args.push("experiment".into());
                args.extend(experiment_args.to_args());
            }
        }
        args
    }
}
