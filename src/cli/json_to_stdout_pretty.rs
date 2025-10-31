use serde::Serialize;

pub trait JsonToStdoutPretty: Serialize {
    fn write_json_to_stdout_pretty(&self) -> eyre::Result<()> {
        let stdout = std::io::stdout();
        let handle = stdout.lock();
        serde_json::to_writer_pretty(handle, &self)?;
        Ok(())
    }
}
impl<T> JsonToStdoutPretty for T where T: Serialize + ?Sized {}
