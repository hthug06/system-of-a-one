//! Create a client with [`SystemOneClientBuilder`] and use the client to send request with [`decide`](`client::decide`)

use reqwest::Client;
use std::fmt::Debug;

pub use crate::client::builder::SystemOneClientBuilder;
use crate::{
    decision::SystemOneDecision,
    error::{SystemOneError, SystemOneResult},
    request::SystemOneRequest,
};

mod builder;

/// Asynchronous HTTP client for interacting with a System One inference server.
///
/// Created via [`SystemOneClient::builder`]. Once initialized, use [`SystemOneClient::decide`]
/// to evaluate requests against the model.
///
/// # Examples
///
/// ```no_run
/// use std::time::Duration;
/// use system_of_a_one::SystemOneClient;
///
/// let client: SystemOneClient = SystemOneClient::builder()
///     .api_url("http://localhost:8080".to_string())
///     .api_key("your-api-key".to_string()) // Optional
///     .timeout(Duration::from_secs(15)) // Optional (defaults to 10s)
///     .build()?;
/// # Ok::<(), system_of_a_one::SystemOneError>(())
/// ```
///
/// # Errors
/// [`SystemOneClientBuilder::build`] returns an error if [`SystemOneClientBuilder::api_url`]
/// is not provided or if the underlying HTTP client fails to initialize.
pub struct SystemOneClient {
    /// The reqwest client, used to send the request to the server
    http_client: Client,
    /// The api url (ex: http://localhost:8080/v1/systemone).
    api_url: String,
    api_key: Option<String>,
}

impl SystemOneClient {
    /// The builder for this instance
    pub fn builder() -> SystemOneClientBuilder {
        SystemOneClientBuilder::default()
    }

    /// Sends the request to the decision model via [`SystemOneClientBuilder::api_url`]`/v1/systemone`.
    ///
    /// # Errors
    ///
    /// - [`SystemOneError::Transport`] if the HTTP request fails to send or read the response body.
    /// - [`SystemOneError::ApiError`] if the server responds with a non-2xx status code.
    /// - [`SystemOneError::Decode`] if the response body cannot be deserialized into [`SystemOneDecision`].
    pub async fn decide(&self, req: &SystemOneRequest<'_>) -> SystemOneResult<SystemOneDecision> {
        let mut builder: reqwest::RequestBuilder = self.http_client.post(&self.api_url).json(&req);

        if let Some(key) = &self.api_key {
            builder = builder.bearer_auth(key);
        }

        let resp: reqwest::Response = builder.send().await?;
        let status: reqwest::StatusCode = resp.status();

        if !resp.status().is_success() {
            let message: String = read_error_body(resp, 2048).await;
            return Err(SystemOneError::ApiError {
                status: status.as_u16(),
                message,
            });
        }

        let bytes = resp.bytes().await?;
        serde_json::from_slice(&bytes).map_err(|source| SystemOneError::Decode {
            source,
            body: String::from_utf8_lossy(&bytes[..bytes.len().min(512)]).into_owned(),
        })
    }
}

impl Debug for SystemOneClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemOneClient")
            .field("api_url", &self.api_url)
            .field("http", &self.http_client)
            .finish()
    }
}

/// Avoid Having the whole request in memory in case of a malicious server
async fn read_error_body(mut resp: reqwest::Response, max_bytes: usize) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(max_bytes.min(2048));

    while buf.len() < max_bytes {
        match resp.chunk().await {
            Ok(Some(chunk)) => {
                let remaining = max_bytes - buf.len();
                let take = chunk.len().min(remaining);
                buf.extend_from_slice(&chunk[..take]);
            }
            Ok(None) | Err(_) => break,
        }
    }

    let mut text: String = String::from_utf8_lossy(&buf).into_owned();
    if buf.len() == max_bytes {
        text.push_str("... [truncated]");
    }
    text
}
