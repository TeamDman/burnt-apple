use crate::cli::output_format::OutputFormatArg;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Args;
use std::ffi::OsString;
use std::str::FromStr;

#[derive(Args, Default, Arbitrary, PartialEq, Debug)]
pub struct GlobalArgs {
    /// Enable debug logging
    #[clap(long, global = true)]
    pub debug: bool,

    /// Emit structured JSON logs alongside stderr output
    #[clap(long, global = true)]
    pub json: bool,

    /// Output format for command responses (auto, text, json, pretty-json)
    #[clap(long, global = true, default_value = "auto")]
    pub output_format: String,
}

impl GlobalArgs {
    pub fn log_level(&self) -> tracing::Level {
        if self.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        }
    }

    pub fn write_json_logs(&self) -> bool {
        self.json
    }

    pub fn output_format(&self) -> eyre::Result<OutputFormatArg> {
        OutputFormatArg::from_str(&self.output_format)
    }
}

impl ToArgs for GlobalArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        if self.debug {
            args.push("--debug".into());
        }
        if self.json {
            args.push("--json".into());
        }
        if self.output_format != "auto" {
            args.push("--output-format".into());
            args.push(self.output_format.clone().into());
        }
        args
    }
}
