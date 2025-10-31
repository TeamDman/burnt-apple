use arbitrary::Arbitrary;
use clap::ValueEnum;

#[derive(strum::VariantArray, clap::ValueEnum, Clone, Debug, PartialEq, Eq, Arbitrary)]
pub enum Experiment {
    TensorAddition,
}
