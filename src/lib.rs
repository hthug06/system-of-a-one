//! Rust client for the System One decision model API.
//!
//! > **Note:** This crate is functional for `/v1/systemone` decision requests,
//! > but currently covers only a subset of the full API. Breaking changes may
//! > occur before `1.0.0`.
//!
//! Build a [`SystemOneClient`], define a batch of named tasks with [`SystemOneQuestions`],
//! and send a [`SystemOneRequest`] to evaluate a state in a single inference pass.
//!
//! # Example
//!
//! ```no_run
//! use system_of_a_one::{
//!     SystemOneClient, SystemOneDecision, SystemOneQuestions, SystemOneRequest, SystemOneTaskConfig,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     println!("Hello, world!");
//!
//!    // Create the client
//!    let client: SystemOneClient = SystemOneClient::builder()
//!        .base_url("http://localhost:8080".to_string())
//!        .build()?;
//!    // Create the full request, with all questions in one variable
//!    let request: SystemOneRequest = SystemOneRequest::builder()
//!         .state(
//!             "I was charged twice for my subscription this month. Please refund the extra charge.",
//!         )
//!         .questions(
//!             SystemOneQuestions::default()
//!                 // First question, type score
//!                 .with_question(
//!                     "urgency",
//!                     SystemOneTaskConfig::score(
//!                         "How urgent is this message?",
//!                         vec![
//!                             "Can wait",
//!                             "Needs attention this week",
//!                             "Needs attention today",
//!                         ],
//!                     ),
//!                 )
//!                 // Second question, type noul
//!                 .with_question(
//!                     "billing",
//!                     SystemOneTaskConfig::noul("Is this message about billing?"),
//!                 )
//!                 // Last question, type choice
//!                 .with_question(
//!                     "tone",
//!                     SystemOneTaskConfig::choice(
//!                         "What is the tone of this message?",
//!                         [
//!                             ("angry", "An upset or hostile message"),
//!                             ("calm", "A neutral or polite message"),
//!                             ("excited", "An enthusiastic or eager message"),
//!                         ],
//!                     ),
//!                 ),
//!         )
//!         .build()?;
//!
//!     // send the request
//!     let decision: SystemOneDecision = client.decide(&request).await?;
//!
//!     if let Some(tone) = decision.get_choice("tone") {
//!         println!("Detected tone: {tone}");
//!     }
//!     if let Some(urgency) = decision.get_clamped_score("urgency") {
//!         println!("Urgency score: {urgency:.2}");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Supported Task Types
//!
//! - **Noul** ([`SystemOneTaskConfig::noul`]): Evaluates a truth-value probability (`0.0` to `1.0`).
//! - **Noul with criteria** ([`SystemOneTaskConfig::noul_with_criteria`]): Evaluates a truth-value probability (`0.0` to `1.0`), but with criteria to be more precise.
//! - **Choice** ([`SystemOneTaskConfig::choice`]): Classifies the input state into one of the provided labeled criteria.
//! - **Score** ([`SystemOneTaskConfig::score`]): Produces a continuous regression score across ordered criteria.

mod client;
mod decision;
mod error;
mod questions;
mod request;
mod task;

pub use client::{SystemOneClient, SystemOneClientBuilder};
pub use decision::{SystemOneDecision, SystemOneResults, SystemOneUsage, TaskResult};
pub use error::{SystemOneError, SystemOneResult};
pub use questions::SystemOneQuestions;
pub use request::{SystemOneRequest, SystemOneRequestBuilder};
pub use task::{NoulCriteria, SystemOneTaskConfig, SystemOneTaskType};
