use crate::body::Body;

mod append_headers;
mod into_response;
mod into_response_parts;

pub type Response<T = Body> = http::Response<T>;

pub type Result<T, E = ErrorResponse> = std::result::Result<T, E>;

#[derive(Debug)]
#[must_use]
pub struct ErrorResponse(Response);

// impl<T> From<T> for ErrorResponse
// where
//     T: IntoResponse,
// {
//     fn from(value: T) -> Self {
//         Self(value.into_response())
//     }
// }
