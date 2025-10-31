pub mod model_kind_command;
pub mod model_kind_list_command;

use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;

use crate::cli::command::model::kind::model_kind_command::ModelKindCommand;
use crate::cli::to_args::ToArgs;

#[derive(Args, Clone, Arbitrary, PartialEq, Debug)]
pub struct ModelKindArgs {
    #[clap(subcommand)]
    pub command: ModelKindCommand,
}

impl ModelKindArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        self.command.invoke()
    }
}

impl ToArgs for ModelKindArgs {
    fn to_args(&self) -> Vec<OsString> {
        self.command.to_args()
    }
}
