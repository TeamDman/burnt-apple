pub mod command;
pub mod global_args;
pub mod json_to_stdout_pretty;
pub mod to_args;

use crate::cli::command::Command;
use crate::cli::global_args::GlobalArgs;
use crate::cli::to_args::Invocable;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Parser;
use std::ffi::OsString;

#[derive(Parser, Arbitrary, PartialEq, Debug)]
#[clap(version)]
pub struct Cli {
    #[clap(flatten)]
    pub global_args: GlobalArgs,
    #[clap(subcommand)]
    pub command: Command,
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

#[cfg(test)]
mod tests {
    use super::*;
    use arbitrary::Arbitrary;
    use clap::Parser;

    #[test]
    fn fuzz_cli_args_roundtrip() {
        let mut data = vec![42u8; 1024];
        let mut rng = arbitrary::Unstructured::new(&data);

        for i in 0..100 {
            let cli = match Cli::arbitrary(&mut rng) {
                Ok(cli) => cli,
                Err(_) => {
                    data = vec![i as u8; 1024];
                    rng = arbitrary::Unstructured::new(&data);
                    Cli::arbitrary(&mut rng).expect("Failed to generate CLI instance")
                }
            };

            let args = cli.to_args();

            let mut full_args = vec!["burnt-apple".into()];
            full_args.extend(args);

            let parsed_cli = Cli::try_parse_from(&full_args)
                .unwrap_or_else(|e| panic!("Failed to parse CLI args on iteration {}: {}", i, e));

            assert_eq!(cli, parsed_cli, "CLI roundtrip failed on iteration {}", i);
        }
    }

    #[test]
    fn fuzz_cli_args_consistency() {
        let mut data = vec![123u8; 1024];
        let mut rng = arbitrary::Unstructured::new(&data);

        for i in 0..50 {
            let cli = match Cli::arbitrary(&mut rng) {
                Ok(cli) => cli,
                Err(_) => {
                    data = vec![(i * 2) as u8; 1024];
                    rng = arbitrary::Unstructured::new(&data);
                    Cli::arbitrary(&mut rng).expect("Failed to generate CLI instance")
                }
            };

            let args1 = cli.to_args();
            let args2 = cli.to_args();

            assert_eq!(
                args1, args2,
                "CLI.to_args() should be deterministic for iteration {}",
                i
            );
        }
    }
}
