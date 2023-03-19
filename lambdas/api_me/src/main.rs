
mod handler;
mod requestapime;
mod requestapiemail;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
  simple_logger::init_with_level(log::Level::Info)?;
  lambda_http::run(lambda_http::service_fn(handle)).await?;
  Ok(())
}

async fn handle(event: lambda_http::Request) -> Result<lambda_http::Response<String>, lambda_http::Error> {
    match handler::Handler::new(event).run().await {
        Ok(r) => Ok(r),
        Err(e) => {
            let r = oneml::response::InternalServerError { message: e.to_string() };
            let r = lambda_http::Response::builder()
                     .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                     .body(serde_json::to_string(&r)?)?;
            Ok(r)
        },
    }
}

