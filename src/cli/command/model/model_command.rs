use crate::cli::command::model::kind::ModelKindArgs;
use crate::cli::command::model::model_list_command::ModelListArgs;
use crate::cli::command::model::model_train_command::ModelTrainArgs;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Subcommand, Clone, Arbitrary, PartialEq, Debug)]
pub enum ModelCommand {
    /// Manage model kinds
    Kind(ModelKindArgs),
    /// List models available on disk
    List(ModelListArgs),
    /// Train a new model from the dataset
    Train(ModelTrainArgs),
}

impl ModelCommand {
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            ModelCommand::Kind(args) => args.invoke(),
            ModelCommand::List(args) => args.invoke(),
            ModelCommand::Train(args) => args.invoke(),
        }
    }
}

impl ToArgs for ModelCommand {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            ModelCommand::Kind(kind_args) => {
                args.push("kind".into());
                args.extend(kind_args.to_args());
            }
            ModelCommand::List(list_args) => {
                args.push("list".into());
                args.extend(list_args.to_args());
            }
            ModelCommand::Train(train_args) => {
                args.push("train".into());
                args.extend(train_args.to_args());
            }
        }
        args
    }
}
