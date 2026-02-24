use crate::cli::command::model::direct::{InferArgs, ServeArgs, SetOutputHomeArgs};
use crate::cli::command::model::model_list_command::ModelListArgs;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Subcommand, Clone, Arbitrary, PartialEq, Debug)]
pub enum ModelCommand {
    /// List supported hardcoded models
    List(ModelListArgs),
    /// Start model daemon
    Serve(ServeArgs),
    /// Set output home for a model
    SetOutputHome(SetOutputHomeArgs),
    /// Run inference via model daemon
    Infer(InferArgs),
}

impl ModelCommand {
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            ModelCommand::List(args) => args.invoke(),
            ModelCommand::Serve(args) => args.invoke(),
            ModelCommand::SetOutputHome(args) => args.invoke(),
            ModelCommand::Infer(args) => args.invoke(),
        }
    }
}

impl ToArgs for ModelCommand {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            ModelCommand::List(list_args) => {
                args.push("list".into());
                args.extend(list_args.to_args());
            }
            ModelCommand::Serve(serve_args) => {
                args.push("serve".into());
                args.extend(serve_args.to_args());
            }
            ModelCommand::SetOutputHome(set_output_home_args) => {
                args.push("set-output-home".into());
                args.extend(set_output_home_args.to_args());
            }
            ModelCommand::Infer(infer_args) => {
                args.push("infer".into());
                args.extend(infer_args.to_args());
            }
        }
        args
    }
}
