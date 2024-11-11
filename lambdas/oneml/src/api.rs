use super::*;

mod authenticated_request; pub use authenticated_request::AuthenticatedRequest;
mod unauthenticated_request; pub use unauthenticated_request::UnauthenticatedRequest;

pub enum Request {
  Authenticated { identity: Identity, request: AuthenticatedRequest, },
  Unauthenticated { request: UnauthenticatedRequest, },
}

impl Request {
  pub async fn handle(self) -> Result<lambda_http::Response<String>> {
    match self {
      Request::Authenticated { identity, request } => request.handle(identity).await,
      Request::Unauthenticated { request } => request.handle().await,
    }
  }
}

fn to_response<I>(item: &I) -> Result<lambda_http::Response<String>>
where I: serde::Serialize
{
  Ok(lambda_http::Response::builder()
     .status(lambda_http::http::StatusCode::OK)
     .body(serde_json::to_string(item)?)?
  )
}

