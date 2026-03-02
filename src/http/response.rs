use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessBody<T: Serialize> {
    status: u16,
    code: &'static str,
    data: T,
}
