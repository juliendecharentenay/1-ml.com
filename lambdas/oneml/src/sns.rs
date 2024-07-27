use super::*;

/// Notify to an SNS topic
pub fn sns_notify(msg: String) {
  if let Ok(topic_arn) = std::env::var("SNS_TOPIC_ARN") {
    if let Err(e) = notify(topic_arn, msg) {
      log::error!("sns notification error: {e:?}");
    }
  } else {
    log::warn!("No topic ARN provided. Create topic and set environment variable SNS_TOPIC_ARN to topic arn.");
    log::error!("Error message: {msg}");
  }
}

fn notify(topic_arn: String, msg: String) -> error::Result<()> {
  let config = futures::executor::block_on(aws_config::load_defaults(aws_config::BehaviorVersion::latest()));
  let client = aws_sdk_sns::Client::new(&config);
  futures::executor::block_on(
    client
    .publish()
    .topic_arn(topic_arn)
    .message(msg)
    .send()
  ).map_err(error::Error::from)?;
  Ok(())
}
