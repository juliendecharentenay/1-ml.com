pub struct Config {}
impl Config {
  pub const BUCKET: &'static str  = "${bucket}";
  pub const SES_IDENTITY: &'static str = r"${sesIdentity}";

  pub fn topic_arn() -> Option<String> {
    std::env::var("SNS_TOPIC_ARN").ok()
  }
}

