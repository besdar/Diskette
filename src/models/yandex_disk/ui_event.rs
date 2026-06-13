use super::{DiskOutput, DiskProgress, SetupSave};
use std::path::PathBuf;

pub(crate) enum UiEvent {
    CommandProgress(DiskProgress),
    CommandFinished(DiskOutput),
    SetupSaved(Result<SetupSave, String>),
    TokenSetupSaved {
        auth_file: PathBuf,
        result: Result<SetupSave, String>,
    },
}
