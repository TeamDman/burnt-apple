pub mod kind;
pub mod model_command;
pub mod model_list_command;
pub mod model_train_command;

use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;

use crate::cli::command::model::model_command::ModelCommand;
use crate::cli::to_args::ToArgs;

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ModelArgs {
    #[clap(subcommand)]
    pub command: ModelCommand,
}

impl ModelArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        self.command.invoke()
    }
}

impl ToArgs for ModelArgs {
    fn to_args(&self) -> Vec<OsString> {
        self.command.to_args()
    }
}
