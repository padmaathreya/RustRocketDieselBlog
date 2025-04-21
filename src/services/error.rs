

use serde::Serialize;
#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
impl From<diesel::result::Error> for ErrorResponse {
    fn from(err: diesel::result::Error) -> Self {
        ErrorResponse { message: format!("Database Error: {}", err) }
    }
}