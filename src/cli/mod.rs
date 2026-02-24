pub mod command;
pub mod global_args;
pub mod json_to_stdout_pretty;
pub mod output_format;
pub mod to_args;

use crate::cli::command::CliCommand;
use crate::cli::global_args::GlobalArgs;
use crate::cli::to_args::Invocable;
use crate::cli::to_args::ToArgs;
use clap::Parser;
use std::ffi::OsString;

#[derive(Parser, PartialEq, Debug)]
#[clap(version)]
pub struct Cli {
    #[clap(flatten)]
    pub global_args: GlobalArgs,
    #[clap(subcommand)]
    pub command: CliCommand,
}

impl Cli {
    pub fn invoke(self) -> eyre::Result<()> {
        self.command.invoke()
    }
}

impl ToArgs for Cli {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        args.extend(self.global_args.to_args());
        args.extend(self.command.to_args());
        args
    }
}

impl Invocable for Cli {
    fn executable(&self) -> std::path::PathBuf {
        std::env::current_exe().expect("Failed to get current executable path")
    }

    fn args(&self) -> Vec<OsString> {
        self.to_args()
    }
}
