use crate::cli::command::model::model_registry::require_supported_model;
use crate::cli::command::model::settings::{
    BurntAppleConfig, DaemonMetadata, clear_daemon_metadata, load_daemon_metadata,
    save_daemon_metadata,
};
use arbitrary::Arbitrary;
use crate::cli::to_args::ToArgs;
use clap::Args;
use eyre::{Context, bail};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::fs;
use std::net::{SocketAddrV4, TcpListener, TcpStream};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Args, Clone, Arbitrary, PartialEq, Debug)]
pub struct ServeArgs {
    pub model_id: String,
    #[arg(long, default_value_t = false)]
    pub detach: bool,
}

#[derive(Args, Clone, Arbitrary, PartialEq, Debug)]
pub struct SetOutputHomeArgs {
    pub model_id: String,
    #[arg(value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Args, Clone, Arbitrary, PartialEq, Debug)]
pub struct InferArgs {
    pub model_id: String,
    #[arg(long, value_name = "FILE")]
    pub input: PathBuf,
}

impl ServeArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        let model = require_supported_model(&self.model_id)?;
        let model_id = model.id;
        if daemon_is_healthy(model_id)? {
            println!("Daemon appears to already be running for {}.", model_id);
            return Ok(());
        }

        clear_daemon_metadata(model_id)?;
        let daemon_port = reserve_local_port()?;

        let python_script = daemon_script_path();
        if !python_script.exists() {
            bail!(
                "Daemon script missing at {}",
                python_script.display()
            );
        }

        let whisperx_runtime_dir = resolve_whisperx_runtime_dir()?;

        let mut command = Command::new("uv");
        command
            .arg("run")
            .arg("python")
            .arg(&python_script)
            .arg("--port")
            .arg(daemon_port.to_string())
            .arg("--model")
            .arg("large-v2")
            .arg("--language")
            .arg("en")
            .arg("--device")
            .arg("cuda")
            .arg("--compute-type")
            .arg("float16")
            .current_dir(&whisperx_runtime_dir);

        if self.detach {
            command
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());

            #[cfg(windows)]
            {
                const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
                const DETACHED_PROCESS: u32 = 0x0000_0008;
                command.creation_flags(CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS);
            }

            let child = command
                .spawn()
                .wrap_err("Failed to spawn detached python daemon process")?;

            let metadata = DaemonMetadata {
                model_id: model_id.to_string(),
                pid: child.id(),
                port: daemon_port,
            };
            save_daemon_metadata(model_id, &metadata)?;

            wait_for_daemon_ready(model_id)?;
            println!("Daemon has been started in a standalone process.");
            return Ok(());
        }

        command
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        let mut child = command
            .spawn()
            .wrap_err("Failed to start foreground python daemon process")?;

        let metadata = DaemonMetadata {
            model_id: model_id.to_string(),
            pid: child.id(),
            port: daemon_port,
        };
        save_daemon_metadata(model_id, &metadata)?;

        let exit = child.wait().wrap_err("Foreground daemon process failed")?;
        clear_daemon_metadata(model_id)?;
        if !exit.success() {
            bail!("Daemon process exited with status {}", exit);
        }

        Ok(())
    }
}

impl SetOutputHomeArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        let model = require_supported_model(&self.model_id)?;
        let model_id = model.id;
        fs::create_dir_all(&self.path)
            .wrap_err_with(|| format!("Failed to create output directory {}", self.path.display()))?;
        let canonicalized = self
            .path
            .canonicalize()
            .wrap_err_with(|| format!("Failed to canonicalize {}", self.path.display()))?;

        let mut config = BurntAppleConfig::load()?;
        config.model_settings_mut(model_id).output_home = Some(canonicalized.clone());
        config.save()?;

        println!(
            "The output home for {} has been set to \"{}\"",
            model_id,
            canonicalized.display()
        );
        Ok(())
    }
}

impl InferArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        let model = require_supported_model(&self.model_id)?;
        let model_id = model.id;
        if !self.input.exists() {
            bail!("Input file does not exist: {}", self.input.display());
        }

        let config = BurntAppleConfig::load()?;
        let output_home = config
            .model_output_home(model_id)
            .ok_or_else(|| eyre::eyre!("No output home configured for {}. Run '{} set-output-home <path>' first.", model_id, model_id))?;

        fs::create_dir_all(&output_home)
            .wrap_err_with(|| format!("Failed to create output directory {}", output_home.display()))?;

        let metadata = load_daemon_metadata(model_id)?.ok_or_else(|| {
            eyre::eyre!(
                "No daemon metadata for {}. Start daemon with '{} serve --detach' first.",
                model_id,
                model_id
            )
        })?;

        let request = DaemonRequest::Infer {
            input: self.input.clone(),
            output_home: output_home.clone(),
        };

        let response = send_request(&metadata, &request)?;
        if !response.ok {
            bail!(
                "Daemon inference failed: {}",
                response.error.unwrap_or_else(|| "unknown daemon error".to_string())
            );
        }

        println!(
            "Inference succeeded for {}. Outputs written under {}",
            self.input.display(),
            output_home.display()
        );
        Ok(())
    }
}

impl ToArgs for ServeArgs {
    fn to_args(&self) -> Vec<std::ffi::OsString> {
        let mut args = vec![self.model_id.clone().into()];
        if self.detach {
            args.push("--detach".into());
        }
        args
    }
}

impl ToArgs for SetOutputHomeArgs {
    fn to_args(&self) -> Vec<std::ffi::OsString> {
        vec![self.model_id.clone().into(), self.path.as_os_str().into()]
    }
}

impl ToArgs for InferArgs {
    fn to_args(&self) -> Vec<std::ffi::OsString> {
        vec![
            self.model_id.clone().into(),
            "--input".into(),
            self.input.as_os_str().into(),
        ]
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
enum DaemonRequest {
    Health,
    Infer { input: PathBuf, output_home: PathBuf },
}

#[derive(Debug, Clone, Deserialize)]
struct DaemonResponse {
    ok: bool,
    #[serde(default)]
    error: Option<String>,
}

fn daemon_is_healthy(model_id: &str) -> eyre::Result<bool> {
    let Some(metadata) = load_daemon_metadata(model_id)? else {
        return Ok(false);
    };

    let response = send_request(&metadata, &DaemonRequest::Health);
    if response.is_err() {
        return Ok(false);
    }

    Ok(response?.ok)
}

fn wait_for_daemon_ready(model_id: &str) -> eyre::Result<()> {
    for _ in 0..40 {
        if daemon_is_healthy(model_id)? {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(250));
    }

    bail!(
        "Daemon failed to become ready for {}. Check your uv/python/whisperx environment.",
        model_id
    )
}

fn reserve_local_port() -> eyre::Result<u16> {
    let listener = TcpListener::bind(SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0))
        .wrap_err("Failed to reserve local daemon port")?;
    let port = listener
        .local_addr()
        .wrap_err("Failed to read reserved socket address")?
        .port();
    drop(listener);
    Ok(port)
}

fn send_request(metadata: &DaemonMetadata, request: &DaemonRequest) -> eyre::Result<DaemonResponse> {
    let socket = SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, metadata.port);
    let mut stream = TcpStream::connect_timeout(&socket.into(), Duration::from_secs(2))
        .wrap_err_with(|| format!("Failed to connect to daemon on {}", socket))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(300)))
        .wrap_err("Failed to configure daemon read timeout")?;
    stream
        .set_write_timeout(Some(Duration::from_secs(30)))
        .wrap_err("Failed to configure daemon write timeout")?;

    let request_json = serde_json::to_string(request).wrap_err("Failed to serialize daemon request")?;
    stream
        .write_all(request_json.as_bytes())
        .wrap_err("Failed to send daemon request")?;
    stream
        .write_all(b"\n")
        .wrap_err("Failed to terminate daemon request line")?;

    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    reader
        .read_line(&mut line)
        .wrap_err("Failed to read daemon response")?;
    if line.trim().is_empty() {
        bail!("Daemon returned an empty response")
    }

    serde_json::from_str(&line).wrap_err("Failed to parse daemon response JSON")
}

fn daemon_script_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("python")
        .join("whisperx_daemon.py")
}

fn resolve_whisperx_runtime_dir() -> eyre::Result<PathBuf> {
    if let Some(path) = std::env::var_os("BURNT_APPLE_WHISPERX_HOME") {
        let runtime = PathBuf::from(path);
        if runtime.exists() {
            return Ok(runtime);
        }
    }

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest.join("..\\Nanuak\\nanuak-whisper"),
        manifest.join("..\\nanuak-whisper"),
    ];

    for candidate in candidates {
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    bail!(
        "Could not locate whisperx runtime directory. Set BURNT_APPLE_WHISPERX_HOME to your nanuak-whisper path."
    )
}
