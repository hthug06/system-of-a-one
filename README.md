# system-of-a-one

[![Crates.io](https://img.shields.io/crates/v/system-of-a-one.svg)](https://crates.io/crates/system-of-a-one)
[![Docs.rs](https://docs.rs/system-of-a-one/badge.svg)](https://docs.rs/system-of-a-one)
[![License](https://img.shields.io/crates/l/system-of-a-one.svg)](#license)

> [!WARNING]
> **Early-stage / Partial API coverage:** This crate is functional and tested for core decision inference
(`/v1/systemone`), but does not yet cover the entire System One API surface.

Async Rust client for the **System One** decision model API (`/v1/systemone`).

Evaluate a given state against a batch of structured decision tasks (`noul`, `choice`, and `score`) in a single
low-latency inference pass, with zero-copy borrowing (`Cow<'a, str>`) and deterministic JSON serialization.

## Installation

Add `system-of-a-one` and `tokio` to your `Cargo.toml`:

```toml
[dependencies]
system-of-a-one = "0.1.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

# Quick Start

```rust
use system_of_a_one::{
    NoulCriteria, SystemOneClient, SystemOneQuestions, SystemOneRequest, SystemOneTaskConfig,
    TaskResult,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the HTTP client
    let client = SystemOneClient::builder()
        .api_url("http://localhost:8080/v1/systemone".to_string())
        .api_key("your-api-key".to_string()) // Optional
        .build()?;

    // Define the batch of decision tasks
    let questions = SystemOneQuestions::new()
        .with_question(
            "action",
            SystemOneTaskConfig::choice(
                "Select the best next action based on the current state.",
                [
                    ("engage", "Attack the nearest target"),
                    ("retreat", "Fall back to recover health"),
                    ("loot", "Collect nearby items"),
                ],
            ),
        )
        .with_question(
            "is_critical",
            SystemOneTaskConfig::noul_with_criteria(
                "Is the agent in immediate danger?",
                NoulCriteria {
                    r#true: "Health is below 25% or surrounded by multiple enemies.",
                    r#false: "Health is stable and no immediate threat is present.",
                },
            ),
        )
        .with_question(
            "threat_level",
            SystemOneTaskConfig::score(
                "Rate the overall threat level from 0 to 10.",
                [
                    "Number and proximity of hostile entities",
                    "Remaining health and ammunition",
                ],
            ),
        );

    // Build and send the request
    let request = SystemOneRequest::builder()
        .state("HP: 18/100, ammo: 2, 3 enemies within 5 meters.")
        .questions(questions)
        .build()?;

    let decision = client.decide(&request).await?;

    println!("Model: {}", decision.model);
    println!("Tokens: {:?}", decision.usage);

    if let Some(TaskResult::Choice { choice, confidence, .. }) = decision.answers.get("action") {
        println!("Selected action: {choice} (confidence: {confidence:.2})");
    }

    Ok(())
}
```

## Supported task type

Not all task type are supported right now, this project is very new but can do a lot !

Each question in `SystemOneQuestions` maps a task identifier to a `SystemOneTaskConfig`:

| Task Type  | Constructor                                                              | JSON Tag           | Output Variant (`TaskResult`)                              | (TaskResult)    Description                                                                                   |
|------------|:-------------------------------------------------------------------------|--------------------|------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|
| **Noul**   | `SystemOneTaskConfig::noul`<br>`SystemOneTaskConfig::noul_with_criteria` | `"type": "noul"`   | `TaskResult::Noul { noul }`                                | Binary/continuous scalar evaluation, optionally guided by explicit true/false descriptions.                   |
| **Choice** | `SystemOneTaskConfig::choice`                                            | `"type": "choice"` | `TaskResult::Choice { choice, confidence, probabilities }` | Discrete classification among labeled options, returning the winning label and full probability distribution. |
| **Score**  | `SystemOneTaskConfig::score`                                             | `"type": "score"`  | `TaskResult::Choice { choice, confidence, probabilities }` | Numeric scoring evaluated against a list of rubric criteria.                                                  |

## Error Handling

All fallible operations return `SystemOneResult<T>` (`Result<T, SystemOneError>`):

- `SystemOneError::ClientConfig` — Invalid client builder parameters (ex: empty or malformed `api_url`).
- `SystemOneError::RequestConfig` — Missing required fields when building a `SystemOneRequest`.
- `SystemOneError::Transport` — Underlying HTTP or network error from `reqwest`.
- `SystemOneError::ApiError { status, message }` — Non-2xx HTTP response from the server (with truncated error body).
- `SystemOneError::Decode { source, body }` — Invalid JSON or schema mismatch in the server response.

---

## About the use of IA

I'm not against IA, but i'm against an AI doing an entire project. This project is made by me, because i love coding, i
want to be better in rust and i'm fascinated in new technology (especially in the decision model here).

This project is **NOT** created and will not be maintained by an IA. I did use it, but only for some optimization or
documentation, **Not for the full project or the architecture of it !!**

## Other things

Feel free to make an issue, or open a pull request, I would be very happy to take a look at it !

System of a one is inspired by the group **System of a down** if you haven't noticed :)