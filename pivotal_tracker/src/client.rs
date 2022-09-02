use core::fmt;
use std::{error::Error, fmt::Display};

use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::ParseError;

pub struct Client {
  pub api_key: String,
  pub api_version: u8,
}

impl Client {
  pub fn new(options: ClientNewOptions) -> Self {
    Client {
      api_key: options.api_key,
      api_version: options.api_version,
    }
  }

  pub async fn request<TSuccess, TGetBuilder>(
    &self,
    func: TGetBuilder,
  ) -> Result<TSuccess, RequestError>
  where
    TSuccess: DeserializeOwned,
    TGetBuilder: Fn(&reqwest::Client, String) -> reqwest::RequestBuilder,
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
    let res = func(&client, base_url).send().await?;

    if res.status().as_u16() >= 400 {
      let result = res.json::<ResponseError>().await?;

      return Err(RequestError::Response(result));
    }

    let result = res.json::<TSuccess>().await?;

    Ok(result)
  }
}

#[derive(Debug)]
pub struct ClientNewOptions {
  pub api_key: String,
  pub api_version: u8,
}

#[derive(Debug)]
pub enum RequestError {
  Reqwest(reqwest::Error),
  Parse(ParseError),
  Response(ResponseError),
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

impl From<ResponseError> for RequestError {
  fn from(err: ResponseError) -> Self {
    Self::Response(err)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
  /// Class of error that has occurred or been detected.
  pub code: ResponseErrorCode,
  pub error: String,

  /// An English string with a broader/alternative description of the problem.
  pub general_problem: Option<String>,
  pub kind: String,

  /// An English string with hints to the API client developer. Consider this
  /// a micro-FAQ answer specific to the particular error case. Very unlikely
  /// to be suitable for direct presentation to a user.
  pub possible_fix: Option<String>,

  /// An English string describing the constraint on the API that wasn't met
  /// by the current request.
  pub requirement: Option<String>,

  /// In the case where the server detected one or more errors with regard to the value of specific individual parameters, rather than with the request as a whole, the error response hash may be augmented with parameter-specific error messages. These messages are packaged in an array of hashes, stored under the validation_errors key. Each validation error hash contains a field key with the name of a request parameter (resource attribute), and a problem key with an English string describing the error(s) detected in the value of that parameter.
  pub validation_errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseErrorCode {
  #[serde(rename = "api_external_error")]
  APIExternalError,

  #[serde(rename = "api_internal_error")]
  APIInternalError,
  CantParseJSON,
  CapabilityAccessDenied,
  ComponentUnavailable,
  ContentFormatNotFound,
  CouldNotParseBody,
  Deadlock,
  DuplicateEntry,
  EnterpriseSignupError,

  #[serde(rename = "http_method_not_supported")]
  HTTPMethodNotSupported,

  #[serde(rename = "https_required")]
  HTTPSRequired,
  IntegrationError,
  InvalidAuthentication,
  InvalidParameter,
  InvalidUpload,
  NotAcceptable,
  RequestEntityTooLarge,
  RequiresGet,
  RequiresPost,
  RouteNotFound,
  ServerUnderLoad,
  Timeout,
  Unauthenticated,
  UnauthorizedOperation,
  UnfoundResource,
  UnhandledCondition,

  #[serde(rename = "xhr_required")]
  XHRRequired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
  /// Name of request parameter
  pub field: String,

  /// The error(s) detected in the field
  pub problem: String,
}
