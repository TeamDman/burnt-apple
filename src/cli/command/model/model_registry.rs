use eyre::bail;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportedModel {
    pub id: &'static str,
    pub purpose: &'static str,
}

pub const MBAIN_WHISPER_X: SupportedModel = SupportedModel {
    id: "mbain/whisperX",
    purpose: "transcribe command",
};

#[must_use]
pub fn supported_models() -> &'static [SupportedModel] {
    &[MBAIN_WHISPER_X]
}

pub fn require_supported_model(model_id: &str) -> eyre::Result<SupportedModel> {
    if let Some(model) = supported_models()
        .iter()
        .copied()
        .find(|model| model.id.eq_ignore_ascii_case(model_id))
    {
        return Ok(model);
    }

    let supported = supported_models()
        .iter()
        .map(|model| model.id)
        .collect::<Vec<_>>()
        .join(", ");
    bail!(
        "Unsupported model '{}'. Supported models: {}.",
        model_id,
        supported
    )
}
