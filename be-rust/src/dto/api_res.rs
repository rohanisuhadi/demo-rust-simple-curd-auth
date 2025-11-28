use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(serde::Serialize)]
pub struct ApiRes<T> {
    pub status: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

pub struct ApiHttpResponse<T> {
    pub status: StatusCode,
    pub body: ApiRes<T>,
}

impl<T: Serialize> ApiHttpResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            status: StatusCode::OK,
            body: ApiRes {
                status: true,
                message: Some("Success".into()),
                data: Some(data),
            },
        }
    }

    pub fn error(status: StatusCode, msg: impl Into<String>) -> Self {
        Self {
            status,
            body: ApiRes {
                status: false,
                message: Some(msg.into()),
                data: None,
            },
        }
    }
}

impl<T: serde::Serialize> IntoResponse for ApiHttpResponse<T> {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
