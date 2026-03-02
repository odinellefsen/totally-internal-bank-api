use serde::Serialize;
#[derive(Serialize)]
pub struct ApiSuccessBody<T: Serialize> {
    pub status: u16,
    pub code: &'static str,
    pub message: &'static str,
    pub data: T,
}

#[derive(Serialize)]
pub struct ApiErrorBody<'a> {
    pub status: u16,
    pub code: &'a str,
    pub message: &'a str,
}
