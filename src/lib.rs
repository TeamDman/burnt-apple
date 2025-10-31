#![recursion_limit = "256"]

pub mod cli;
pub mod tracing;

use crate::cli::Cli;
use clap::CommandFactory;
use clap::FromArgMatches;
use tracing_subscriber::filter::LevelFilter;

pub fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli_command = Cli::command();
    let matches = cli_command.get_matches();
    let cli = Cli::from_arg_matches(&matches)?;

    let level_filter = LevelFilter::from_level(cli.global_args.log_level());
    crate::tracing::init_tracing(level_filter, cli.global_args.write_json_logs())?;

    ::tracing::debug!(?cli, "CLI initialised");

    cli.invoke()
}
