use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;
use tracing::info;

#[derive(Args, Clone, Arbitrary, PartialEq, Debug, Default)]
pub struct ModelKindListArgs {}

impl ModelKindListArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        info!("model kind list invoked");
        Ok(())
    }
}

impl ToArgs for ModelKindListArgs {
    fn to_args(&self) -> Vec<OsString> {
        Vec::new()
    }
}
