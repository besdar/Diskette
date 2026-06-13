#[derive(Clone, Debug)]
pub(crate) struct DiskProgress {
    pub(crate) text: String,
    pub(crate) auth_url: Option<String>,
    pub(crate) auth_code: Option<String>,
}
