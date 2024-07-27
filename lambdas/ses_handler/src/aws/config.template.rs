pub struct Config {}
impl Config {
  pub const BUCKET: &'static str  = "${bucket}";
  pub const SES_IDENTITY: &'static str = r"${sesIdentity}";
}

