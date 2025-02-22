//[openapi]   unauthenticated_request:
use super::*;

pub enum UnauthenticatedRequest {
  Ok,
}

impl UnauthenticatedRequest {
  pub async fn handle(self) -> Result<lambda_http::Response<String>> {
    match self {
      UnauthenticatedRequest::Ok => self.ok_impl().await,
    }
  }
}

//[openapi]    get_api_ok:
//[openapi]      summary: Health check
//[openapi]      operationId: getOk
//[openapi]      responses:
//[openapi]        '200':
//[openapi]          description: OK
//[openapi]          content:
//[openapi]            text/plain:
//[openapi]              schema: 
//[openapi]                type: string
//[openapi]              examples:
//[openapi]                test_request:
//[openapi]                  value: "OK"
//[openapi]
impl UnauthenticatedRequest {
  async fn ok_impl(&self) -> Result<lambda_http::Response<String>> {
    log::info!("ApiOk: request");
    Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body("ok".to_string())?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn it_handles_request_ok() -> Result<()> {
    let response = UnauthenticatedRequest::Ok.handle().await?;
    assert!(matches!(response.status(), lambda_http::http::StatusCode::OK));
    assert!(response.body().eq("ok"));
    Ok(())
  }
}

