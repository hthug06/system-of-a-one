use std::borrow::Cow;

use serde::Serialize;

use crate::{
    error::{SystemOneError, SystemOneResult},
    questions::SystemOneQuestions,
};

/// Payload sent to the decision model endpoint (via [`api_url`](`crate::SystemOneClient::api_url`)).
///
/// Constructed via [`SystemOneRequest::builder`].
#[derive(Clone, Debug, Serialize)]
pub struct SystemOneRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<Cow<'a, str>>,
    state: Cow<'a, str>,
    questions: SystemOneQuestions<'a>,
}

impl<'a> SystemOneRequest<'a> {
    /// Creates a new [`SystemOneRequestBuilder`] to construct a request.
    #[must_use]
    pub fn builder() -> SystemOneRequestBuilder<'a> {
        SystemOneRequestBuilder::default()
    }
}

/// Builder for [`SystemOneRequest`].
///
/// Both [`state`](Self::state) and [`questions`](Self::questions) are required
/// before calling [`build`](Self::build).
#[derive(Debug, Default)]
pub struct SystemOneRequestBuilder<'a> {
    state: Option<Cow<'a, str>>,
    model: Option<Cow<'a, str>>,
    questions: Option<SystemOneQuestions<'a>>,
}

impl<'a> SystemOneRequestBuilder<'a> {
    /// Sets the context or environment state evaluated by the model (required).
    ///
    /// Accepts either a borrowed `&'a str` or an owned `String` without unnecessary allocation.
    #[must_use]
    pub fn state(mut self, state: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Overrides the target model identifier for this request (optional).
    ///
    /// If omitted, the server uses its default configured model.
    #[must_use]
    pub fn model(mut self, model: impl Into<Cow<'a, str>>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Sets the batch of decision questions to evaluate against the state (required).
    #[must_use]
    pub fn questions(mut self, questions: SystemOneQuestions<'a>) -> Self {
        self.questions = Some(questions);
        self
    }

    /// Validates the builder fields and constructs a [`SystemOneRequest`].
    ///
    /// # Errors
    ///
    /// Returns [`SystemOneError::RequestConfig`] if `state` or `questions` is missing.
    pub fn build(self) -> SystemOneResult<SystemOneRequest<'a>> {
        let state: Cow<'a, str> = self
            .state
            .ok_or_else(|| SystemOneError::RequestConfig("'state' is required".into()))?;

        let questions: SystemOneQuestions<'_> = self
            .questions
            .ok_or_else(|| SystemOneError::RequestConfig(("'questions' is required").into()))?;

        Ok(SystemOneRequest {
            state,
            model: self.model,
            questions,
        })
    }
}
