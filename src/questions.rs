use std::{borrow::Cow, collections::BTreeMap};

use serde::Serialize;

use crate::task::SystemOneTaskConfig;

/// Collection of named decision tasks sent to the model.
///
/// Serializes transparently as a JSON object mapping each question identifier
/// to its [`SystemOneTaskConfig`]. Keys are ordered deterministically via a [`BTreeMap`].
#[derive(Clone, Debug, Default, Serialize)]
#[serde(transparent)]
pub struct SystemOneQuestions<'a> {
    map: BTreeMap<Cow<'a, str>, SystemOneTaskConfig<'a>>,
}

impl<'a> SystemOneQuestions<'a> {
    /// Creates an empty [`SystemOneQuestions`] collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a named task configuration and returns the updated collection.
    ///
    /// If `key` was already present, its previous [`SystemOneTaskConfig`] is overwritten.
    pub fn with_question(
        mut self,
        key: impl Into<Cow<'a, str>>,
        value: SystemOneTaskConfig<'a>,
    ) -> Self {
        self.map.insert(key.into(), value);
        self
    }
}
