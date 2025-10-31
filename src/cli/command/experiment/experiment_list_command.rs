use crate::cli::command::experiment::Experiment;
use crate::cli::json_to_stdout_pretty::JsonToStdoutPretty;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;
use strum::VariantArray;

#[derive(Args, Clone, Arbitrary, PartialEq, Debug, Default)]
pub struct ExperimentListArgs {}

impl ExperimentListArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        Experiment::VARIANTS.write_json_to_stdout_pretty()?;
        Ok(())
    }
}

impl ToArgs for ExperimentListArgs {
    fn to_args(&self) -> Vec<OsString> {
        Vec::new()
    }
}
