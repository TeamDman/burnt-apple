use crate::cli::command::model::model_registry::supported_models;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;

#[derive(Args, Clone, Arbitrary, PartialEq, Debug, Default)]
pub struct ModelListArgs {}

impl ModelListArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        for model in supported_models() {
            println!("{} ({})", model.id, model.purpose);
        }
        Ok(())
    }
}

impl ToArgs for ModelListArgs {
    fn to_args(&self) -> Vec<OsString> {
        Vec::new()
    }
}
