use crate::cli::command::model::kind::model_kind_list_command::ModelKindListArgs;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Subcommand, Clone, Arbitrary, PartialEq, Debug)]
pub enum ModelKindCommand {
    /// List the supported model kinds
    List(ModelKindListArgs),
}

impl ModelKindCommand {
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            ModelKindCommand::List(args) => args.invoke(),
        }
    }
}

impl ToArgs for ModelKindCommand {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            ModelKindCommand::List(list_args) => {
                args.push("list".into());
                args.extend(list_args.to_args());
            }
        }
        args
    }
}
