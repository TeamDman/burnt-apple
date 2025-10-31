use crate::cli::command::experiment::Experiment;
use crate::cli::command::experiment::experiments::tensor_addition_experiment::TensorAdditionExperiment;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use burn::backend::Wgpu;
use clap::Args;
use std::ffi::OsString;
use tracing::info;

#[derive(Args, Clone, Arbitrary, PartialEq, Debug)]
pub struct ExperimentRunArgs {
    /// Name of the experiment to execute
    pub experiment: Experiment,
}

impl ExperimentRunArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        info!(experiment = %self.experiment, "experiment run invoked");
        match self.experiment {
            Experiment::TensorAddition => {
                TensorAdditionExperiment::run::<Wgpu>()?;
            }
        }
        Ok(())
    }
}

impl ToArgs for ExperimentRunArgs {
    fn to_args(&self) -> Vec<OsString> {
        vec![OsString::from(self.experiment.to_string())]
    }
}
