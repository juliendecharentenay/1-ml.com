use super::*;

use lambda_http::RequestExt;

pub struct Handler {
  event: lambda_http::Request,
}

impl Handler {
  pub fn new(event: lambda_http::Request) -> Handler { Handler { event } }
}

impl Handler {
  pub fn body<T>(&self) -> Result<T, oneml::Error>
  where T: serde::de::DeserializeOwned,
  {
    if let lambda_http::Body::Text(body) = self.event.body() {
      Ok(serde_json::from_str(body.as_str())?)
    } else {
      Err(oneml::Error::EmptyRequestTextBody)
    }
  }

  pub async fn run(&self) -> Result<lambda_http::Response<String>, oneml::Error> {
    log::info!("Event: {:?}", self.event);
    log::info!("Lambda context: {:?}", self.event.lambda_context());
    log::info!("Request context: {:?}", self.event.request_context());

    if let lambda_http::request::RequestContext::ApiGatewayV1(context) = self.event.request_context() {
      let identity = match context.authorizer.fields.get("claims") {
        Some(claim) => Some(oneml::Identity::from_authorizer(claim)?),
        None => None,
      };

      let request: oneml::api::Request = match context.resource_path.as_ref().ok_or("Unable to retrieve resource path")?.as_str() {
        "/api/me" if matches!(context.http_method, lambda_http::http::Method::GET)
        => oneml::api::Request::Authenticated { 
          identity: identity.ok_or("Unauthenticated request")?,
          request: oneml::api::AuthenticatedRequest::GetMe,
        },

        "/api/me" if matches!(context.http_method, lambda_http::http::Method::PATCH)
        => oneml::api::Request::Authenticated { 
          identity: identity.ok_or("Unauthenticated request")?,
          request: oneml::api::AuthenticatedRequest::PatchMe(self.body()?),
        },

        "/api/email" if matches!(context.http_method, lambda_http::http::Method::GET)
        => oneml::api::Request::Authenticated {
          identity: identity.ok_or("Unauthenticated request")?,
          request: oneml::api::AuthenticatedRequest::GetEmail,
        },

        "/api/email/{email}" if matches!(context.http_method, lambda_http::http::Method::PATCH)
        => oneml::api::Request::Authenticated {
          identity: identity.ok_or("Unauthenticated request")?,
          request: oneml::api::AuthenticatedRequest::PatchEmail {
            email: self.event.path_parameters().first("email").map(urlencoding::decode).ok_or("Unable to retrieve email")??.to_string(),
            body: self.body()?,
          },
        },

        "/api/email/{email}" if matches!(context.http_method, lambda_http::http::Method::GET)
        => oneml::api::Request::Authenticated {
          identity: identity.ok_or("Unauthenticated request")?,
          request: oneml::api::AuthenticatedRequest::GetReceivedEmail {
            email: self.event.path_parameters().first("email").map(urlencoding::decode).ok_or("Unable to retrieve email")??.to_string(),
          },
        },

        "/api/ok" if matches!(context.http_method, lambda_http::http::Method::GET)
        => oneml::api::Request::Unauthenticated {
          request: oneml::api::UnauthenticatedRequest::Ok,
        },

        _ => return Err("Request unknown".into()),
      };

      request.handle().await
    } else {
      Err("Request is not from ApiGatewayV1".into())
    }
  }
}
