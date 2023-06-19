use serde::Serialize;

#[derive(Serialize)]
pub struct PatchName {
    pub global_name: String,
}

#[derive(Serialize)]
pub struct CustomStatus {
    pub text: String,
    pub emoji_id: Option<String>,
    pub emoji_name: String,
    pub expires_at: Option<u64>,
}

#[derive(Serialize)]
pub struct PatchStatus {
    pub custom_status: CustomStatus,
}
