use chrono::Local;
use eyre::Result;
use std::fs::File;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
pub use tracing::Level;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::Directive;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;

pub const DEFAULT_EXTRA_FILTERS: &str = r#"
bevy_shader=warn
offset_allocator=warn
bevy_app=info
bevy_render=info
gilrs=info
cosmic_text=info
naga=warn
wgpu=error
wgpu_hal=error
bevy_skein=trace
bevy_winit::system=info
cubecl_wgpu=warn
"#;

pub fn init_tracing(level: impl Into<Directive>, json: bool) -> Result<()> {
    let default_directive: Directive = level.into();
    let make_env_filter = || {
        let mut filter = EnvFilter::builder()
            .with_default_directive(default_directive.clone())
            .from_env_lossy();

        for directive in DEFAULT_EXTRA_FILTERS
            .split('\n')
            .map(str::trim)
            .filter(|directive| !directive.is_empty())
        {
            if let Ok(extra) = Directive::from_str(directive) {
                filter = filter.add_directive(extra);
            }
        }

        filter
    };
    let make_stderr_layer = || {
        tracing_subscriber::fmt::layer()
            .with_file(cfg!(debug_assertions))
            .with_target(true)
            .with_line_number(cfg!(debug_assertions))
            .with_writer(std::io::stderr)
            .pretty()
            .without_time()
    };

    if json {
        let timestamp = Local::now().format("%Y-%m-%d_%Hh%Mm%Ss");
        let json_log_path = format!("burnt_apple_log_{}.jsonl", timestamp);

        let file = File::create(&json_log_path)?;
        let file = Arc::new(Mutex::new(file));
        let json_writer = {
            let file = Arc::clone(&file);
            BoxMakeWriter::new(move || {
                file.lock()
                    .expect("failed to lock json log file")
                    .try_clone()
                    .expect("failed to clone json log file handle")
            })
        };

        let json_format = tracing_subscriber::fmt::format().json();
        let json_layer = tracing_subscriber::fmt::layer()
            .event_format(json_format)
            .with_file(true)
            .with_target(false)
            .with_line_number(true)
            .with_writer(json_writer)
            .without_time();

        tracing_subscriber::registry()
            .with(make_env_filter())
            .with(make_stderr_layer())
            .with(json_layer)
            .try_init()?;

        info!(?json_log_path, "JSON log output initialized");
    } else {
        tracing_subscriber::registry()
            .with(make_env_filter())
            .with(make_stderr_layer())
            .try_init()?;
    }

    Ok(())
}
