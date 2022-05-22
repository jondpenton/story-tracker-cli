pub struct Client {
  pub api_key: String,
  pub api_version: i8,
}

#[derive(Debug)]
pub struct ClientNewOptions {
  pub api_key: String,
  pub api_version: i8,
}

impl Client {
  pub fn new(options: ClientNewOptions) -> Self {
    Client {
      api_key: options.api_key,
      api_version: options.api_version,
    }
  }
}
