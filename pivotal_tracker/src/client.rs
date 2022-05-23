use core::fmt;
use std::{error::Error, fmt::Display};

use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use url::ParseError;

pub struct Client {
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

  pub async fn request<T, F>(&self, func: F) -> Result<T, RequestError>
  where
    T: DeserializeOwned,
    F: Fn(&reqwest::Client, String) -> reqwest::RequestBuilder,
  {
    let mut headers = HeaderMap::new();

    headers.append("X-TrackerToken", self.api_key.parse().unwrap());

    let client = reqwest::Client::builder()
      .default_headers(headers)
      .build()
      .unwrap();
    let base_url = format!(
      "https://www.pivotaltracker.com/services/v{}",
      self.api_version
    );
    let builder = func(&client, base_url);
    let result = builder.send().await?.json::<T>().await?;

    Ok(result)
  }
}

#[derive(Debug)]
pub struct ClientNewOptions {
  pub api_key: String,
  pub api_version: i8,
}

#[derive(Debug)]
pub enum RequestError {
  Reqwest(reqwest::Error),
  Parse(ParseError),
}

impl Display for RequestError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "RequestError")
  }
}

impl Error for RequestError {}

impl From<reqwest::Error> for RequestError {
  fn from(err: reqwest::Error) -> Self {
    Self::Reqwest(err)
  }
}

impl From<ParseError> for RequestError {
  fn from(err: ParseError) -> Self {
    Self::Parse(err)
  }
}
