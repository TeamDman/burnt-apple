We have an old clap-based command line argument parsing situation right now.

We want to use teamy-rust-cli as the basis and we want to merge that into this repo such that we get the latest from that.

This existing cli has the `experiment list` and `experiment run` commands which are effectively tests we are writing as we learn the burn framework.

We have a `model list` and `model run` command structure but they don't do anything yet.

We want to hardcode the models we are going to support.

At this time, we are working on programmatic manipulation of the whisperX library.

{
	"folders": [
		{
			"name": "burnt-apple",
			"path": "../Repos/burnt-apple"
		},
		{
			"name": "nanuak-whisper",
			"path": "../Repos/Nanuak/nanuak-whisper"
		},
		{
			"name": "teamy-rust-cli",
			"path": "../Repos/teamy-rust-cli"
		},
		{
			"name": "roam",
			"path": "../Repos/roam"
		},
		{
			"name": "facet",
			"path": "../Repos/facet"
		},
		{
			"name": "figue",
			"path": "../Repos/figue"
		},
		{
			"name": "whisperX",
			"path": "../Repos/whisperX"
		},
		{
			"name": "whisper-burn",
			"path": "../Repos/whisper-burn"
		},
		{
			"name": "vetchricore",
			"path": "../Repos/vetchricore"
		}
	],
	"settings": {}
}

those are the repos I have open right now.

We are in the burnt apple repo with this text file, but I will use it to convey my desires which will likely result in the modification of the teamy-rust-cli repository as well.

In the vetchricore repository we have some [output_format.rs](../../vetchricore/src/cli/output_format.rs) logic that is a more recent attempt at having different command output formats that we want to copy into the teamy-rust-cli.

We want to remove the burnt apple `model kind` stuff.

should end up looking like

```
burnt-apple.exe model list
> mbain/whisperX (transcribe command)
burnt-apple.exe mbain/whisperX serve --detach
> Daemon has been started in a standalone process.
burnt-apple.exe mbain/whisperX set-output-home . # canonicalize
> The output home for mbain/whisperX has been set to "{canonicalizedpath}" # print with colours
burnt-apple.exe mbain/whisperX infer --input some-movie.mp4
> {Success message showing output path}
```

For example, whisper-x has many output formats.

```
Nanuak\nanuak-whisper on  main [!] is 📦 v0.1.0 via 🐍 v3.12.2 
❯ ls .\output\
'BAND-MAID - glory.json'                                                                                                  'Super Factory Manager 4.20.0 changelog.json'
'BAND-MAID - glory.srt'                                                                                                   'Super Factory Manager 4.20.0 changelog.srt'
'BAND-MAID - glory.tsv'                                                                                                   'Super Factory Manager 4.20.0 changelog.tsv'
'BAND-MAID - glory.txt'                                                                                                   'Super Factory Manager 4.20.0 changelog.txt'
'BAND-MAID - glory.vtt'                                                                                                   'Super Factory Manager 4.20.0 changelog.vtt'
'Benedict Cumberbatch reads a funny letter from a frustrated bank customer [GNXnmDy4jmg].json'                            the-birth-and-death-of-javascript.json
'Benedict Cumberbatch reads a funny letter from a frustrated bank customer [GNXnmDy4jmg].srt'                             the-birth-and-death-of-javascript.srt
'Benedict Cumberbatch reads a funny letter from a frustrated bank customer [GNXnmDy4jmg].tsv'                             the-birth-and-death-of-javascript.tsv
'Benedict Cumberbatch reads a funny letter from a frustrated bank customer [GNXnmDy4jmg].txt'                             the-birth-and-death-of-javascript.txt
'Benedict Cumberbatch reads a funny letter from a frustrated bank customer [GNXnmDy4jmg].vtt'                             the-birth-and-death-of-javascript.vtt
"Brandon's Philosophy on Plot—Promises, Progress, and Payoffs [-hO7fM9EHU4].json"                                         'YouTube Transcripts - Shortcut.lnk'
"Brandon's Philosophy on Plot—Promises, Progress, and Payoffs [-hO7fM9EHU4].srt"
"Brandon's Philosophy on Plot—Promises, Progress, and Payoffs [-hO7fM9EHU4].tsv"
"Brandon's Philosophy on Plot—Promises, Progress, and Payoffs [-hO7fM9EHU4].txt"
"Brandon's Philosophy on Plot—Promises, Progress, and Payoffs [-hO7fM9EHU4].vtt"
'brandon philosophy.zip'
'Brian Warner - Magic Wormhole- Simple Secure File Transfer - PyCon 2016.mp4 [oFrTqQw0_3c].json'
'Brian Warner - Magic Wormhole- Simple Secure File Transfer - PyCon 2016.mp4 [oFrTqQw0_3c].srt'
'Brian Warner - Magic Wormhole- Simple Secure File Transfer - PyCon 2016.mp4 [oFrTqQw0_3c].tsv'
'Brian Warner - Magic Wormhole- Simple Secure File Transfer - PyCon 2016.mp4 [oFrTqQw0_3c].txt'
'Brian Warner - Magic Wormhole- Simple Secure File Transfer - PyCon 2016.mp4 [oFrTqQw0_3c].vtt'
'Building a Distributed Protocol by Dominik Tornow [DW9gNduZATE].json'
'Building a Distributed Protocol by Dominik Tornow [DW9gNduZATE].srt'
'Building a Distributed Protocol by Dominik Tornow [DW9gNduZATE].tsv'
'Building a Distributed Protocol by Dominik Tornow [DW9gNduZATE].txt'
'Building a Distributed Protocol by Dominik Tornow [DW9gNduZATE].vtt'
'Comprehending Proc Macros (128kbit_AAC).json'
'Comprehending Proc Macros (128kbit_AAC).srt'
'Comprehending Proc Macros (128kbit_AAC).tsv'
'Comprehending Proc Macros (128kbit_AAC).txt'
'Comprehending Proc Macros (128kbit_AAC).vtt'
'Embedded Rust setup explained [TOAynddiu5M].json'
'Embedded Rust setup explained [TOAynddiu5M].srt'
'Embedded Rust setup explained [TOAynddiu5M].tsv'
'Embedded Rust setup explained [TOAynddiu5M].txt'
'Embedded Rust setup explained [TOAynddiu5M].vtt'
'IT All Candidates TownHall 2025.json'
'IT All Candidates TownHall 2025.srt'
'IT All Candidates TownHall 2025.tsv'
'IT All Candidates TownHall 2025.txt'
'IT All Candidates TownHall 2025.vtt'
jon.json
jon.srt
jon.tsv
jon.txt
jon.vtt
'Jonathan Blow on his programming language jai and upcoming game(s)! [jamU6SQBtxk].json'
'Jonathan Blow on his programming language jai and upcoming game(s)! [jamU6SQBtxk].srt'
'Jonathan Blow on his programming language jai and upcoming game(s)! [jamU6SQBtxk].tsv'
'Jonathan Blow on his programming language jai and upcoming game(s)! [jamU6SQBtxk].txt'
'Jonathan Blow on his programming language jai and upcoming game(s)! [jamU6SQBtxk].vtt'
'My thoughts On Vibe Coding Pretty.json'
'My Thoughts On ＂Vibe Coding＂ (And Prime) [b0fv54xGOwY].json'
'My Thoughts On ＂Vibe Coding＂ (And Prime) [b0fv54xGOwY].srt'
'My Thoughts On ＂Vibe Coding＂ (And Prime) [b0fv54xGOwY].tsv'
'My Thoughts On ＂Vibe Coding＂ (And Prime) [b0fv54xGOwY].txt'
'My Thoughts On ＂Vibe Coding＂ (And Prime) [b0fv54xGOwY].vtt'
'sfm 4.15.0 changelog.json'
'sfm 4.15.0 changelog.srt'
'sfm 4.15.0 changelog.tsv'
'sfm 4.15.0 changelog.txt'
'sfm 4.15.0 changelog.vtt'
'SFM 4.17.0.json'
'SFM 4.17.0.srt'
'SFM 4.17.0.tsv'
'SFM 4.17.0.txt'
'SFM 4.17.0.vtt'
'SFM Performance.json'
'SFM Performance.srt'
'SFM Performance.tsv'
'SFM Performance.txt'
'SFM Performance.vtt'
'Stephen Wolfram tackles the big questions—from God and free will to whether AI will destroy us all. [8SD9WgPCZ28].json'
'Stephen Wolfram tackles the big questions—from God and free will to whether AI will destroy us all. [8SD9WgPCZ28].srt'
'Stephen Wolfram tackles the big questions—from God and free will to whether AI will destroy us all. [8SD9WgPCZ28].tsv'
'Stephen Wolfram tackles the big questions—from God and free will to whether AI will destroy us all. [8SD9WgPCZ28].txt'
'Stephen Wolfram tackles the big questions—from God and free will to whether AI will destroy us all. [8SD9WgPCZ28].vtt'
Nanuak\nanuak-whisper on  main [!] is 📦 v0.1.0 via 🐍 v3.12.2 
```

We want the user to be able to configure their output home path for each model then they can invoke the model from wherever knowing it will safely go their results dir.


right now the nanuak thing is invoking using uv 

Nanuak\nanuak-whisper on  main [!] is 📦 v0.1.0 via 🐍 v3.12.2 
❯ ls -la      
-a---   7 26 Jan  2025 .gitignore
-a--- 244 10 Feb  2025 Get-HuggingFaceToken.ps1
-a--- 109 26 Jan  2025 hello.py
d----   -  9 Feb 18:09 output
-a--- 250 15 Apr  2025 pyproject.toml
-a--- 300 26 Jan  2025 README.md
-a--- 551  2 Feb 22:41 Run.ps1
-a--- 130 23 Feb 21:01 target_file.txt
Nanuak\nanuak-whisper on  main [!] is 📦 v0.1.0 via 🐍 v3.12.2 

Nanuak\nanuak-whisper on  main [!] is 📦 v0.1.0 via 🐍 v3.12.2 
❯ cat .\Run.ps1
# . .\Get-HuggingFaceToken.ps1
$files = Get-Content target_file.txt
foreach ($file in $files) {
    $file = $file.Replace("`"", "")
    Write-Host "Processing file: $file" -ForegroundColor Green
    # --hf_token "$Env:HF_TOKEN" `
    uv run whisperx `
    --highlight_words True `
    --task "transcribe" `
    --language "en" `
    --output_dir "output" `
    --device "cuda" `
    --model "large-v2" `
    --min_speakers 1 `
    --max_speakers 3 `
    --diarize `
    $file | Out-Host
}

# --min_speakers 2 `
# --max_speakers 2 `
Nanuak\nanuak-whisper on  main [!] is 📦 v0.1.0 via 🐍 v3.12.2 

we want to basically make the rust command line do the same thing, but we want to daemonize the model so that it doesn't unload between invocations.

basically, we start the model and then we can call `infer` multiple times without suffering the loading up cost each time.

Tracey has a daemon example using roam that we can take inspiration from.

```
tracey on  teamy-main via 🦀 v1.92.0 
❯ rg "daemon"
docs\spec\tracey.md
803:Tracey uses a daemon architecture where a single persistent daemon per workspace owns all state and computation. Protocol bridges (HTTP, LSP, MCP) connect as clients to the 
daemon via roam RPC over Unix sockets.
807:r[daemon.lifecycle.socket]
808:The daemon MUST listen on `.tracey/daemon.sock` in the workspace root directory for client connections.
810:r[daemon.lifecycle.auto-start]
811:Protocol bridges MUST auto-start the daemon if it is not already running when they need to connect.
813:r[daemon.lifecycle.stale-socket]
814:When connecting to the daemon, bridges MUST detect stale socket files (left over from crashed daemons) and remove them before attempting to start a new daemon.
816:r[daemon.lifecycle.idle-timeout]
817:The daemon MAY exit after a configurable idle period with no active connections to conserve resources.
821:r[daemon.state.single-source]
822:The daemon MUST own a single `DashboardData` instance that serves as the source of truth for all coverage data.
824:r[daemon.state.file-watcher]
825:The daemon MUST run a file watcher that monitors all files matching the configuration's include/exclude patterns and triggers rebuilds on changes.
827:r[daemon.state.vfs-overlay]
828:The daemon MUST maintain a virtual filesystem (VFS) overlay that stores in-memory content for files opened in editors, allowing coverage computation on unsaved changes.     
830:r[daemon.state.blocking-rebuild]
831:On file changes, the daemon MUST block all incoming requests until the rebuild completes. This ensures clients never see stale or inconsistent data.
835:r[daemon.roam.protocol]
836:The daemon MUST expose a `TraceyDaemon` service via the roam RPC protocol.
838:r[daemon.roam.unix-socket]
839:Communication between the daemon and bridges MUST occur over Unix domain sockets.
841:r[daemon.roam.framing]
846:r[daemon.vfs.open]
849:r[daemon.vfs.change]
852:r[daemon.vfs.close]
855:r[daemon.vfs.priority]
860:r[daemon.bridge.http]
863:r[daemon.bridge.mcp]
866:r[daemon.bridge.lsp]
871:r[daemon.cli.daemon]
872:The `tracey daemon` command MUST start the daemon in the foreground for the current workspace.
874:r[daemon.cli.web]
875:The `tracey web` command MUST start the HTTP bridge, auto-starting the daemon if needed.
877:r[daemon.cli.mcp]
878:The `tracey mcp` command MUST start the MCP bridge, auto-starting the daemon if needed.
880:r[daemon.cli.lsp]
881:The `tracey lsp` command MUST start the LSP bridge, auto-starting the daemon if needed.
883:r[daemon.cli.logs]
884:The `tracey logs` command MUST display the daemon's log output from `.tracey/daemon.log`.
886:> r[daemon.cli.logs.follow]
889:> r[daemon.cli.logs.lines]
892:r[daemon.cli.status]
893:The `tracey status` command MUST display the daemon's current status, including uptime, watcher state, and any errors.
895:r[daemon.cli.kill]
896:The `tracey kill` command MUST send a shutdown signal to the running daemon and clean up any stale sockets.
898:r[daemon.logs.file]
899:The daemon MUST write all log output to `.tracey/daemon.log` in the workspace root.

crates\tracey-proto\src\lib.rs
1://! Protocol definitions for the tracey daemon RPC service.
4://! macro. The daemon exposes this service over a Unix socket, and bridges
254:/// This provides visibility into daemon internals for monitoring.
578:/// The tracey daemon RPC service.
580:/// This service is exposed by the daemon over a Unix socket. Bridges (HTTP, MCP, LSP)
625:    /// Get daemon health status
628:    /// Request the daemon to shut down gracefully
633:    /// The daemon will send `DataUpdate` messages through the Tx channel

crates\tracey-proto\Cargo.toml
4:description = "Protocol definitions for tracey daemon RPC"

crates\tracey\tests\watcher_tests.rs
15:use tracey::daemon::watcher::{WatcherState, glob_to_watch_dir};
141:async fn create_test_service() -> tracey::daemon::TraceyService {
142:    use tracey::daemon::{Engine, TraceyService};
157:async fn create_test_service_with_watcher() -> (tracey::daemon::TraceyService, Arc<WatcherState>) {
158:    use tracey::daemon::{Engine, TraceyService};
234:    use tracey::daemon::Engine;
256:// the daemon and watching for file changes. These are more complex and

crates\tracey\tests\lsp_diagnostic_lifecycle_tests.rs
27:async fn create_test_engine() -> Arc<tracey::daemon::Engine> {
32:        tracey::daemon::Engine::new(project_root, config_path)
39:async fn create_test_service() -> tracey::daemon::TraceyService {
41:    tracey::daemon::TraceyService::new(engine)
45:async fn create_isolated_test_service() -> (tempfile::TempDir, tracey::daemon::TraceyService) {
51:        tracey::daemon::Engine::new(project_root, config_path)
55:    let service = tracey::daemon::TraceyService::new(engine);
359:/// This tests the daemon service layer - the actual LSP bridge bug
479:    // The daemon correctly returns empty diagnostics

crates\tracey\tests\mcp_tests.rs
4://! daemon service methods that MCP tools call.
19:async fn create_test_engine() -> Arc<tracey::daemon::Engine> {
24:        tracey::daemon::Engine::new(project_root, config_path)
31:async fn create_test_service() -> tracey::daemon::TraceyService {
33:    tracey::daemon::TraceyService::new(engine)

crates\tracey\tests\integration_tests.rs
1://! Integration tests for tracey daemon service.
3://! These tests verify the daemon service functionality by setting up
22:async fn create_test_engine() -> Arc<tracey::daemon::Engine> {
27:        tracey::daemon::Engine::new(project_root, config_path)
34:async fn create_test_service() -> tracey::daemon::TraceyService {
36:    tracey::daemon::TraceyService::new(engine)

crates\tracey\src\data.rs
476:/// r[impl daemon.vfs.priority]

crates\tracey\src\main.rs
13:use tracey::{bridge, daemon, find_project_root};
76:    /// Start the tracey daemon (persistent server for this workspace)
87:    /// Show daemon logs
102:    /// Show daemon status
109:    /// Stop the running daemon
136:        // r[impl daemon.cli.web]
155:        // r[impl daemon.cli.mcp]
161:        // r[impl daemon.cli.lsp]
167:        // r[impl daemon.cli.daemon]
173:            // r[impl daemon.logs.file]
174:            let log_path = project_root.join(".tracey/daemon.log");
180:            rt.block_on(daemon::run(project_root, config_path))
182:        // r[impl daemon.cli.logs]
188:        // r[impl daemon.cli.status]
193:        // r[impl daemon.cli.kill]
196:            rt.block_on(kill_daemon(root))
217:    {daemon}    Start the tracey daemon (persistent server)
218:    {logs}      Show daemon logs
219:    {status}    Show daemon status
220:    {kill}      Stop the running daemon
231:        daemon = "daemon".cyan(),
288:/// r[impl daemon.cli.logs]
289:/// Show daemon logs from .tracey/daemon.log
298:    let log_path = project_root.join(".tracey/daemon.log");
302:            "{}: No daemon log found at {}",
306:        eprintln!("Start the daemon with 'tracey daemon' to generate logs.");
313:    // r[impl daemon.cli.logs.lines]
322:    // r[impl daemon.cli.logs.follow]
350:/// r[impl daemon.cli.status]
351:/// Show daemon status by connecting and calling health()
358:    let endpoint = daemon::local_endpoint(&project_root);
362:        println!("{}: No daemon running", "Status".yellow());
434:            println!("  The daemon may have crashed. Run 'tracey kill' to clean up.");
441:/// r[impl daemon.cli.kill]
442:/// Kill the running daemon by sending a shutdown request
443:async fn kill_daemon(root: Option<PathBuf>) -> Result<()> {
449:    let endpoint = daemon::local_endpoint(&project_root);
453:        println!("{}: No daemon running", "Info".cyan());

crates\tracey\src\daemon\client.rs
1://! Client for connecting to the tracey daemon.
16:/// Type alias for the full daemon client type.
19:/// Create a new daemon client for the given project root.
22:/// - Connect to the daemon on first use (lazy)
23:/// - Start the daemon if it's not running
31:/// Connector that establishes connections to the tracey daemon.
33:/// r[impl daemon.lifecycle.auto-start]
35:/// If the daemon is not running, this will automatically spawn it
47:    /// Spawn the daemon process in the background.
48:    fn spawn_daemon(&self) -> io::Result<()> {
55:        info!("Auto-starting daemon for {}", self.project_root.display());
57:        // Spawn daemon process detached
59:        cmd.arg("daemon")
81:            .map_err(|e| io::Error::other(format!("Failed to spawn daemon: {e}")))?;
86:    /// Wait for the daemon endpoint to appear and connect.
94:                info!("Connected to daemon");
102:                        "Daemon failed to start within {} seconds. Check logs at {}/.tracey/daemon.log",
120:        // Try to connect to existing daemon
123:                info!("Connected to daemon");
127:                // r[impl daemon.lifecycle.stale-socket]
132:                // Auto-start the daemon
133:                self.spawn_daemon()?;
135:                // Wait for daemon to be ready

crates\tracey\src\daemon\watcher.rs
73:/// This struct is shared between the watcher thread and the main daemon.

crates\tracey\src\bridge\mod.rs
1://! Protocol bridges to the tracey daemon.
4://! daemon's roam RPC interface. Bridges are thin protocol adapters that
5://! connect as clients to the daemon.

crates\tracey\src\daemon\mod.rs
1://! Tracey daemon - persistent server for a workspace.
3://! r[impl daemon.state.single-source]
5://! The daemon owns the `DashboardData` and exposes the `TraceyDaemon` RPC service
11://! r[impl daemon.lifecycle.socket]
13://! The daemon listens on `.tracey/daemon.sock` in the workspace root (Unix)
49:const SOCKET_FILENAME: &str = "daemon.sock";
53:/// On Unix, this returns a path to `.tracey/daemon.sock`.
56:/// r[impl daemon.roam.unix-socket]
64:/// On Unix, this returns a path to `.tracey/daemon.sock`.
121:/// Run the daemon for the given workspace.
123:/// r[impl daemon.roam.protocol]
125:/// This function blocks until the daemon exits (idle timeout or signal).
127:    // r[impl daemon.logs.file]
128:    info!("Starting tracey daemon for {}", project_root.display());
136:    // r[impl daemon.lifecycle.stale-socket]
153:    // r[impl daemon.state.file-watcher]
361:    // r[impl daemon.lifecycle.idle-timeout]
590:/// Check if a daemon is running for the given workspace.
608:/// Connect to a running daemon, or return an error.
615:        .wrap_err_with(|| format!("Failed to connect to daemon at {}", endpoint.display()))
618:/// Connect to a running daemon, or return an error.
625:        .wrap_err_with(|| format!("Failed to connect to daemon at {}", endpoint))

crates\tracey\src\lib.rs
8:pub mod daemon;

crates\tracey\src\daemon\service.rs
430:    /// Get daemon health status
474:    /// Request the daemon to shut down gracefully

crates\tracey\Cargo.toml
96:# roam RPC framework (for daemon architecture)

crates\tracey\src\daemon\engine.rs
1://! Core engine for the tracey daemon.
3://! r[impl daemon.state.vfs-overlay]
4://! r[impl daemon.state.blocking-rebuild]
125:    /// r[impl daemon.vfs.open]
139:    /// r[impl daemon.vfs.change]
153:    /// r[impl daemon.vfs.close]

crates\tracey\src\bridge\http\mod.rs
1://! HTTP bridge for the tracey daemon.
4://! to daemon RPC calls. It serves the dashboard SPA and proxies API calls
5://! to the daemon.
7://! r[impl daemon.bridge.http]
32:use crate::daemon::{DaemonClient, new_client};
45:    /// Client connection to daemon (protected by mutex for single-threaded access)
61:/// This function starts an HTTP server that connects to the daemon and
102:    // Start background task to poll daemon version and broadcast changes
384:/// GET /api/health - Get daemon health status.
646:/// Background task that polls daemon version and broadcasts changes.

crates\tracey\src\bridge\mcp.rs
1://! MCP bridge for the tracey daemon.
4://! to daemon RPC calls. It connects to the daemon as a client and
7://! r[impl daemon.bridge.mcp]
27:use crate::daemon::{DaemonClient, new_client};
38:        "⚠️  CONFIG ERROR ⚠️\n{}\n\nFix the config file and the daemon will automatically reload.\n\n---\n\n",
187:/// MCP handler that delegates to the daemon.

crates\tracey\src\bridge\http\dashboard\playwright.config.ts
6: * Note: Tests require a running tracey daemon and web server.
9: *   tracey daemon &

crates\tracey\src\bridge\lsp.rs
1://! LSP bridge for the tracey daemon.
4://! daemon RPC calls. It connects to the daemon as a client and forwards
5://! all operations to the daemon.
7://! r[impl daemon.bridge.lsp]
17:use crate::daemon::{DaemonClient, new_client};
39:/// This function starts an LSP server that connects to the tracey daemon
60:    // Create daemon client (connects lazily, auto-reconnects)
61:    let daemon_client = new_client(project_root.clone());
67:            daemon_client,
85:    /// Client connection to daemon (connects lazily, auto-reconnects)
86:    daemon_client: DaemonClient,
117:    /// Get path and content for a document, for daemon calls.
125:    /// Publish diagnostics for a document by calling daemon.
142:        let Ok(daemon_diagnostics) = rpc(state.daemon_client.lsp_diagnostics(req).await) else {
146:        // Convert daemon diagnostics to LSP diagnostics
147:        let diagnostics: Vec<Diagnostic> = daemon_diagnostics
186:    /// Notify daemon that a file was opened.
191:                .daemon_client
197:    /// Notify daemon that a file changed.
202:                .daemon_client
208:    /// Notify daemon that a file was closed.
213:                .daemon_client
223:        // First, gather all data we need from daemon while holding lock
228:            let config_error = rpc(state.daemon_client.health().await)
233:            let all_diagnostics = match rpc(state.daemon_client.lsp_workspace_diagnostics().await) {
458:        let Ok(completions) = rpc(state.daemon_client.lsp_completions(req).await) else {
503:        let Ok(Some(info)) = rpc(state.daemon_client.lsp_hover(req).await) else {
588:        let Ok(locations) = rpc(state.daemon_client.lsp_definition(req).await) else {
636:        let Ok(locations) = rpc(state.daemon_client.lsp_implementation(req).await) else {
685:        let Ok(locations) = rpc(state.daemon_client.lsp_references(req).await) else {
735:        let Ok(locations) = rpc(state.daemon_client.lsp_document_highlight(req).await) else {
776:        let Ok(symbols) = rpc(state.daemon_client.lsp_document_symbols(req).await) else {
822:            .daemon_client
881:        let Ok(actions) = rpc(state.daemon_client.lsp_code_actions(req).await) else {
924:        let Ok(lenses) = rpc(state.daemon_client.lsp_code_lens(req).await) else {
977:        let Ok(hints) = rpc(state.daemon_client.lsp_inlay_hints(req).await) else {
1024:        let Ok(Some(result)) = rpc(state.daemon_client.lsp_prepare_rename(req).await) else {
1061:        let Ok(edits) = rpc(state.daemon_client.lsp_rename(req).await) else {
1111:        let Ok(tokens) = rpc(state.daemon_client.lsp_semantic_tokens(req).await) else {

crates\tracey\src\bridge\http\dashboard\tests\dashboard.spec.ts
7: * Requires a running tracey daemon and web server.
10: *   1. Start daemon: tracey daemon

crates\tracey\src\bridge\http\dashboard\src\types.ts
24:// Health data from daemon

crates\tracey\src\bridge\http\dashboard\src\main.tsx
669:      ? "Failed to connect to the tracey daemon. Make sure it's running."
tracey on  teamy-main via 🦀 v1.92.0 
❯ code -a .
tracey on  teamy-main via 🦀 v1.92.0 
```

So we have:

1. Refactor burnt-apple to use the new teamy-rust-cli template using facet and figue instead of clap for argument structures
2. commit changes
3. Create experiment that demonstrates RPC between rust and python, our daemon will need to have a persistent python environment where the model stays loaded
4. commit
5. introduce command for setting the output path for the whisper-x model
6. commit
7. introduce command for `burnt-apple.exe mbain/whisperX set-output-home . # canonicalize` and `mbain/whisperX show-output-home`
8. commit
9. introduce command for `burnt-apple.exe mbain/whisperX serve --detach` to start the python daemon that will load our model
10. commit
11. introduce command for `burnt-apple.exe mbain/whisperX infer some-file.mp4`
12. commit

the nuance of the python daemon should just load the model immediately and then we can stop the daemon if we want it unloaded.

