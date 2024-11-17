//[openapi] openapi: 3.1.0
//[openapi] info:
//[openapi]   title: 1-ml.com API
//[openapi]   version: 1.0.0
//[openapi]   
//[openapi] paths:
//[openapi]  /api/me:
//[openapi]    get:
//[openapi]      $ref: '#/components/authenticated_request/me_get'
//[openapi]    patch:
//[openapi]      $ref: '#/components/authenticated_request/me_patch'
//[openapi]  /api/email:
//[openapi]    get:
//[openapi]      $ref: '#/components/authenticated_request/email_get'
//[openapi]  /api/ok:
//[openapi]    get:
//[openapi]      $ref: '#/components/unauthenticated_request/get_api_ok'
//[openapi]
//[openapi] components:
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

