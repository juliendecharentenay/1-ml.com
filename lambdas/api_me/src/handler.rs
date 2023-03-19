use lambda_http::RequestExt;
use async_trait::async_trait;

#[async_trait]
pub trait Request: Sync {
  async fn run(&self, event: &lambda_http::Request) -> Result<(http::StatusCode, String), Box<dyn std::error::Error>> {
    let identity = match event.request_context() {
      lambda_http::request::RequestContext::ApiGatewayV1(context) => {
        match context.authorizer.get("claims") {
          Some(claim) => Some(oneml::Identity::from_authorizer(claim)?),
          None => None,
        }
      },
      _ => None,
    };
    if let Some(identity) = identity {
      self.run_with_identity(event, identity).await
    } else {
      let r = oneml::response::Unauthorized { message: "Unable to retrieve authorization".to_string() };
      Ok((http::StatusCode::UNAUTHORIZED, serde_json::to_string(&r)?))
    }
  }

  async fn run_with_identity(&self, event: &lambda_http::Request,
               identity: oneml::Identity) -> Result<(http::StatusCode, String), Box<dyn std::error::Error>> {
    if let lambda_http::Body::Text(body) = event.body() {
      self.run_with_identity_body(event, identity, body).await
    } else {
      Err("Empty body in request".into())
    }
  }

  async fn run_with_identity_body(&self, _event: &lambda_http::Request,
               _identity: oneml::Identity,
               _body: &String) -> Result<(http::StatusCode, String), Box<dyn std::error::Error>> {
    Err("Request is not implemented".into())
  }
}

pub struct Handler {
  event: lambda_http::Request,
}

impl Handler {
  pub fn new(event: lambda_http::Request) -> Handler { Handler { event } }

  pub async fn run(&self) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
    log::info!("Event: {:?}", self.event);
    log::info!("Lambda context: {:?}", self.event.lambda_context());
    log::info!("Request context: {:?}", self.event.request_context());

    let f = self.parse_request()?;
    let (code, string) = f.run(&self.event).await?;
    Ok(lambda_http::Response::builder().status(code).body(string)?)
  }
}

impl Handler {
  fn parse_request(&self) -> Result<Box<dyn Request>, Box<dyn std::error::Error>> {
    if let lambda_http::request::RequestContext::ApiGatewayV1(context) = self.event.request_context() {
      let path = context.resource_path.as_ref().ok_or("Unable to retrieve resource path")?;
      match context.http_method {
        http::Method::GET if regex::Regex::new(r"^/api/me$")?.is_match(path) 
          => Ok(Box::new(crate::requestapime::Get::default())),
        http::Method::PATCH if regex::Regex::new(r"^/api/me$")?.is_match(path) 
          => Ok(Box::new(crate::requestapime::Patch::default())),

        http::Method::GET if regex::Regex::new(r"^/api/email$")?.is_match(path) 
          => Ok(Box::new(crate::requestapiemail::Get::default())),
        http::Method::PATCH if regex::Regex::new(r"^/api/email/.*$")?.is_match(path) 
          => Ok(Box::new(crate::requestapiemail::Patch::default())),

        _ => Err(format!("Unable to match method {} on path {}", context.http_method, path).into()),
      }
    } else {
      Err("Request is not from apigateway v1".into())
    }
  }
}

