use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;
use tracing::info;

use crate::cli::to_args::ToArgs;

#[derive(Args, Clone, Arbitrary, PartialEq, Debug, Default)]
pub struct ModelTrainArgs {}

impl ModelTrainArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        info!("model train invoked");
        Ok(())
    }
}

impl ToArgs for ModelTrainArgs {
    fn to_args(&self) -> Vec<OsString> {
        Vec::new()
    }
}
