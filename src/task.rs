use std::{borrow::Cow, collections::BTreeMap};

use serde::Serialize;

/// Configuration for a single decision task evaluated by the model.
///
/// Combines task `instructions` with a flattened [`SystemOneTaskType`] discriminator
/// (`"noul"`, `"choice"`, or `"score"`).
#[derive(Debug, Clone, Serialize)]
pub struct SystemOneTaskConfig<'a> {
    instructions: Cow<'a, str>,
    #[serde(flatten)]
    kind: SystemOneTaskType<'a>,
}

impl<'a> SystemOneTaskConfig<'a> {
    /// Creates a binary/boolean task configuration (`"type": "noul"`) without custom criteria descriptions.
    #[must_use]
    pub fn noul(instruction: impl Into<Cow<'a, str>>) -> Self {
        Self {
            instructions: instruction.into(),
            kind: SystemOneTaskType::Noul,
        }
    }

    /// Creates a binary/boolean task configuration (`"type": "noul"`) with explicit descriptions
    /// for the `true` and `false` outcomes.
    #[must_use]
    pub fn noul_with_criteria(
        instruction: impl Into<Cow<'a, str>>,
        criteria: NoulCriteria<'a>,
    ) -> Self {
        Self {
            instructions: instruction.into(),
            kind: SystemOneTaskType::NoulWithCriteria { criteria },
        }
    }

    /// Creates a discrete classification task configuration (`"type": "choice"`).
    ///
    /// `criteria` maps each candidate label to its semantic description
    /// (e.g., `[("attack", "Engage the nearest enemy"), ("flee", "Retreat to safety")]`).
    #[must_use]
    pub fn choice<I, K, V, C>(instruction: I, criteria: C) -> Self
    where
        I: Into<Cow<'a, str>>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
        C: IntoIterator<Item = (K, V)>,
    {
        let criteria = criteria
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        Self {
            instructions: instruction.into(),
            kind: SystemOneTaskType::Choice { criteria },
        }
    }

    /// Creates a numeric scoring task configuration (`"type": "score"`).
    ///
    /// `criteria` lists the ordered rubric items or evaluation dimensions used to score the state.
    #[must_use]
    pub fn score<I, T, C>(instruction: I, criteria: C) -> Self
    where
        I: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        C: IntoIterator<Item = T>,
    {
        let criteria = criteria.into_iter().map(Into::into).collect();

        Self {
            instructions: instruction.into(),
            kind: SystemOneTaskType::Score { criteria },
        }
    }
}

/// Task variant and its associated evaluation criteria, serialized with a `"type"` tag.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SystemOneTaskType<'a> {
    /// Discrete selection among labeled options (`"type": "choice"`).
    Choice {
        /// Map of candidate labels to their descriptions.
        criteria: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    },
    /// Numeric scoring based on a list of rubric criteria (`"type": "score"`).
    Score {
        /// Rubric items or guidelines used to compute the score.
        criteria: Vec<Cow<'a, str>>,
    },
    /// Binary/boolean evaluation without explicit outcome descriptions (`"type": "noul"`).
    Noul,
    /// Binary/boolean evaluation with explicit descriptions for `true` and `false` (`"type": "noul"`).
    #[serde(rename = "noul")]
    NoulWithCriteria {
        /// Descriptions defining when the outcome should be `true` or `false`.
        criteria: NoulCriteria<'a>,
    },
}

/// Outcome descriptions for a [`SystemOneTaskConfig::noul_with_criteria`] task.
///
/// Serializes as `{"true": "...", "false": "..."}`.
#[derive(Debug, Clone, Serialize)]
pub struct NoulCriteria<'a> {
    /// Condition or description corresponding to a positive (`true`) evaluation.
    pub r#true: &'a str,
    /// Condition or description corresponding to a negative (`false`) evaluation.
    pub r#false: &'a str,
}
