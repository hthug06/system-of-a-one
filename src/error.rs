/// Convenience [`Result`] alias using [`SystemOneError`] as the error type.
pub type SystemOneResult<T> = Result<T, SystemOneError>;

/// Errors returned by the System One client and request builders.
#[derive(thiserror::Error, Debug)]
pub enum SystemOneError {
    /// Underlying HTTP or network failure raised by [`reqwest`].
    #[error("HTTP transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// Invalid configuration supplied to [`SystemOneClientBuilder`](crate::SystemOneClientBuilder)
    /// (for example, an empty or malformed base URL).
    #[error("Invalid Client configuration: {0}")]
    ClientConfig(String),

    /// Invalid or incomplete parameters supplied to [`SystemOneRequestBuilder`](crate::SystemOneRequestBuilder)
    /// (for example, missing `state` or `questions`).
    #[error("Invalid Request configuration: {0}")]
    RequestConfig(String),

    /// The server responded with a non-2xx HTTP status code.
    #[error("API returned error status {status}: {message}")]
    ApiError {
        /// HTTP status code returned by the server (e.g., `400`, `401`, `500`).
        status: u16,
        /// Error message or truncated response body returned by the server.
        message: String,
    },

    /// The response body could not be deserialized as valid JSON matching the expected schema.
    #[error("failed to decode response JSON: {source} (body: {body:?})")]
    Decode {
        /// Underlying JSON deserialization error.
        #[source]
        source: serde_json::Error,
        /// Truncated preview of the raw response payload that failed to parse.
        body: String,
    },
}
