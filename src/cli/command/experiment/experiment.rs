use arbitrary::Arbitrary;
use clap::ValueEnum;
use serde::Serialize;
use strum::Display;
use strum::VariantArray;

#[derive(VariantArray, ValueEnum, Clone, Debug, PartialEq, Eq, Arbitrary, Serialize, Display)]
pub enum Experiment {
    TensorAddition,
}
