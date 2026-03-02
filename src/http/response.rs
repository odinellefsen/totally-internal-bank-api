use serde::Serialize;
#[derive(Serialize)]
pub struct ApiSuccessBody<T: Serialize> {
    pub status: u16,
    pub code: String,
    pub message: String,
    pub data: T,
}

#[derive(Serialize)]
pub struct ApiErrorBody {
    pub status: u16,
    pub code: String,
    pub message: String,
}
