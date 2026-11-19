use std::fmt::{Debug, Formatter};
use std::time::Duration;

use reqwest::Client;

use crate::SystemOneClient;
use crate::error::{SystemOneError, SystemOneResult};

/// Builder used to configure and instantiate a [`SystemOneClient`].
#[derive(Default)]
pub struct SystemOneClientBuilder {
    /// The full url, where the API is (ex: `http://localhost:8080/v1/systemone`)
    api_url: Option<String>,
    api_key: Option<String>,
    timeout: Option<Duration>,
}

impl SystemOneClientBuilder {
    /// Set the [`SystemOneClientBuilder::api_url`] and return the instance
    pub fn api_url(mut self, api_url: String) -> Self {
        self.api_url = Some(api_url);
        self
    }

    /// Set the [`SystemOneClientBuilder::api_key`] and return the instance
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Set the [`SystemOneClientBuilder::timeout`] and return the instance
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build a [`SystemOneClient`].
    ///
    /// # Errors
    /// Returns an error if [`SystemOneClientBuilder::api_url`]
    /// is not provided or if the underlying HTTP client fails to initialize.
    pub fn build(self) -> SystemOneResult<SystemOneClient> {
        let api_url: String = self
            .api_url
            .ok_or_else(|| SystemOneError::ClientConfig("api_url is required".into()))?;

        let base: &str = api_url.trim_end_matches('/');
        if base.is_empty() {
            return Err(SystemOneError::ClientConfig(
                "api_url cannot be empty".to_string(),
            ));
        }

        let http_client: Client = Client::builder()
            .timeout(self.timeout.unwrap_or(Duration::from_secs(10)))
            .tcp_nodelay(true)
            .build()?;

        Ok(SystemOneClient {
            http_client,
            api_url: base.to_string(),
            api_key: self.api_key,
        })
    }
}

impl Debug for SystemOneClientBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SystemOneClientBuilder")
            .field(&self.timeout)
            .field(&self.api_url)
            .finish()
    }
}
