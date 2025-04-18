use serde::{Deserialize, Serialize};

use bdk::prelude::*;

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Error {
    #[translate(ko = "잘못된 요청입니다.", en = "Bad Request")]
    Unknown(String),
    #[translate(
        ko = "이미 구독중입니다.",
        en = "You may have already submitted a request."
    )]
    AlreadySubscribed,
}

impl Error {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl<E: std::error::Error + 'static> From<E> for Error {
    fn from(e: E) -> Self {
        Error::Unknown(e.to_string())
    }
}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for Error {
    fn into_response(self) -> by_axum::axum::response::Response {
        (
            by_axum::axum::http::StatusCode::BAD_REQUEST,
            by_axum::axum::Json(self),
        )
            .into_response()
    }
}
