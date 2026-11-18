use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

/// Parsed response returned by the decision model server.
#[derive(Clone, Debug, Deserialize)]
pub struct SystemOneDecision {
    /// Identifier of the model that processed the request.
    pub model: String,
    /// Decision outputs returned by the model.
    pub answers: SystemOneResults,
    /// Input and output token counts for this request.
    pub usage: SystemOneUsage,
}

impl SystemOneDecision {
    /// Convenience helper to directly retrieve a clamped [`TaskResult::Score`] by question key.
    ///
    /// If this key does not exist, return `None`
    pub fn get_clamped_score(&self, key: &str) -> Option<f64> {
        self.answers.get(key)?.as_clamped_score()
    }

    /// Convenience helper to directly retrieve a raw [`TaskResult::Score`] by question key.
    ///
    /// If this key does not exist, return `None`
    pub fn get_score(&self, key: &str) -> Option<f64> {
        self.answers.get(key)?.as_score()
    }

    /// Convenience helper to directly retrieve a [`TaskResult::Choice`] by question key.
    ///
    /// If this key does not exist, return `None`
    pub fn get_choice(&self, key: &str) -> Option<&str> {
        self.answers.get(key)?.as_choice()
    }

    /// Convenience helper to directly retrieve a [`TaskResult::Noul`] value by question key.
    ///
    /// If this key does not exist, return `None`
    pub fn get_noul(&self, key: &str) -> Option<f64> {
        self.answers.get(key)?.as_noul()
    }
}

/// All the Result with the key of the Map being the `question` in the request
pub type SystemOneResults = BTreeMap<String, TaskResult>;

/// Output produced by a single decision task, tagged by `"type"` in JSON.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TaskResult {
    /// Continuous scalar output for a numeric or regression task (`"type": "noul"`).
    Noul {
        /// Raw numeric value predicted by the model.
        noul: f64,
    },

    /// Discrete classification output selecting one label among candidates (`"type": "choice"`).
    Choice {
        /// Winning label selected by the model.
        choice: String,
        /// Confidence score associated with the selected `choice` (typically in `[0.0, 1.0]`).
        confidence: f64,
        /// Full probability distribution across all candidate labels.
        probabilities: HashMap<String, f64>,
    },

    /// Numeric scoring output (`"type": "score"`).
    Score {
        /// Predicted numeric score.
        score: f64,
        /// Optional confidence score for the prediction, if provided by the model.
        #[serde(default)]
        confidence: Option<f64>,
    },
}

impl TaskResult {
    /// Returns the raw score if this result is a [`TaskResult::Score`].
    pub fn as_score(&self) -> Option<f64> {
        match self {
            TaskResult::Score { score, .. } => Some(*score),
            _ => None,
        }
    }

    /// Returns the score clamped to `[0.0, 1.0]` if this result is a [`TaskResult::Score`].
    pub fn as_clamped_score(&self) -> Option<f64> {
        self.as_score().map(|s| s.clamp(0.0, 1.0))
    }

    /// Returns the selected choice if this result is a [`TaskResult::Choice`].
    pub fn as_choice(&self) -> Option<&str> {
        match self {
            TaskResult::Choice { choice, .. } => Some(choice.as_str()),
            _ => None,
        }
    }

    /// Returns the noul probability if this result is a [`TaskResult::Noul`].
    pub fn as_noul(&self) -> Option<f64> {
        match self {
            TaskResult::Noul { noul } => Some(*noul),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
/// The usage of token the decision model used for this request
pub struct SystemOneUsage {
    /// The number of token it takes to the llm to read the request
    pub input_tokens: u32,
    /// The number of token it takesto the llm to generate the response
    pub output_tokens: u32,
}
