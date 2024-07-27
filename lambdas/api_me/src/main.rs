
mod handler;
mod requestapime;
mod requestapiemail;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
  env_logger::init();
  lambda_http::run(lambda_http::service_fn(handle)).await?;
  Ok(())
}

async fn handle(event: lambda_http::Request) -> Result<lambda_http::Response<String>, lambda_http::Error> {
  match handler::Handler::new(event).run().await {
    Ok(r) => Ok(r),
    Err(e) => {
      log::error!("Error: {e:?}");
      oneml::sns_notify(format!("{e:#?}"));
      let r = oneml::response::InternalServerError { message: e.to_string() };
      let r = lambda_http::Response::builder()
        .status(lambda_http::http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(serde_json::to_string(&r)?)?;
      Ok(r)
    },
  }
}

