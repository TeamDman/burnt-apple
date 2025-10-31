use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;
use tracing::info;

use crate::cli::to_args::ToArgs;

#[derive(Args, Clone, Arbitrary, PartialEq, Debug)]
pub struct ExperimentRunArgs {
    /// Name of the experiment to execute
    pub experiment: String,
}

impl ExperimentRunArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        info!(experiment = %self.experiment, "experiment run invoked");
        Ok(())
    }
}

impl ToArgs for ExperimentRunArgs {
    fn to_args(&self) -> Vec<OsString> {
        vec![OsString::from(&self.experiment)]
    }
}
