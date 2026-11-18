//! This crate test the serialization of data. The data need to be formatted the right way to be understood by the LLM

use system_of_a_one::{NoulCriteria, SystemOneQuestions, SystemOneRequest, SystemOneTaskConfig};

#[test]
/// Compare an official noul request (https://docs.typesafe.ai/api/system-one) with one made with [`SystemOneRequest`]
fn test_create_noul_request() {
    let expected = serde_json::json!({
        "model": "jev-latest",
        "state": "I was charged twice for my subscription this month. Please refund the extra charge.",
        "questions": {
            "billing": {
                "type": "noul",
                "instructions": "Is this message about billing?"
            }
        }
    });

    let questions: SystemOneQuestions<'_> = SystemOneQuestions::default().with_question(
        "billing",
        SystemOneTaskConfig::noul("Is this message about billing?"),
    );

    let request: SystemOneRequest<'_> = SystemOneRequest::builder()
        .model("jev-latest")
        .state(
            "I was charged twice for my subscription this month. Please refund the extra charge.",
        )
        .questions(questions)
        .build()
        .unwrap();

    // Convert the struct into a serde_json::Value for structural comparison
    let actual = serde_json::to_value(&request).expect("Failed to serialize request");

    assert_eq!(actual, expected);
}

#[test]
/// Compare an official noul with criteria request (https://docs.typesafe.ai/api/system-one) with one made with [`SystemOneRequest`]
fn test_create_noul_with_criteria_request() {
    let expected = serde_json::json!({
        "model": "jev-latest",
        "state": "I was charged twice for my subscription this month. Please refund the extra charge.",
        "questions": {
            "billing": {
                "type": "noul",
                "instructions": "Is this message about billing?",
                "criteria": {
                    "true": "The message is about billing",
                    "false": "The message isn't about billing but about Chicken Nuggets"
                }
            }
        },
    });

    let questions: SystemOneQuestions<'_> = SystemOneQuestions::default().with_question(
        "billing",
        SystemOneTaskConfig::noul_with_criteria(
            "Is this message about billing?",
            NoulCriteria {
                r#true: "The message is about billing",
                r#false: "The message isn't about billing but about Chicken Nuggets",
            },
        ),
    );

    let request: SystemOneRequest<'_> = SystemOneRequest::builder()
        .model("jev-latest")
        .state(
            "I was charged twice for my subscription this month. Please refund the extra charge.",
        )
        .questions(questions)
        .build()
        .unwrap();

    // Convert the struct into a serde_json::Value for structural comparison
    let actual = serde_json::to_value(&request).expect("Failed to serialize request");

    assert_eq!(actual, expected);
}

#[test]
/// Compare an official choice with criteria request (https://docs.typesafe.ai/api/system-one) with one made with [`SystemOneRequest`]
fn test_create_choice_request() {
    let expected = serde_json::json!({
        "model": "jev-latest",
        "state": "I was charged twice for my subscription this month. Please refund the extra charge.",
        "questions": {
            "tone": {
                "type": "choice",
                "instructions": "What is the tone of this message?",
                "criteria": {
                    "angry": "An upset or hostile message",
                    "calm": "A neutral or polite message",
                    "excited": "An enthusiastic or eager message"
                }
            }
        }
    });

    let questions = SystemOneQuestions::default().with_question(
        "tone",
        SystemOneTaskConfig::choice(
            "What is the tone of this message?",
            [
                ("angry", "An upset or hostile message"),
                ("calm", "A neutral or polite message"),
                ("excited", "An enthusiastic or eager message"),
            ],
        ),
    );

    let request: SystemOneRequest<'_> = SystemOneRequest::builder()
        .model("jev-latest")
        .state(
            "I was charged twice for my subscription this month. Please refund the extra charge.",
        )
        .questions(questions)
        .build()
        .unwrap();

    // Convert the struct into a serde_json::Value for structural comparison
    let actual = serde_json::to_value(&request).expect("Failed to serialize request");

    assert_eq!(actual, expected);
}

#[test]
/// Compare an official choice with criteria request (https://docs.typesafe.ai/api/system-one) with one made with [`SystemOneRequest`]
fn test_create_score_request() {
    let expected = serde_json::json!({
        "model": "jev-latest",
        "state": "I was charged twice for my subscription this month. Please refund the extra charge.",
        "questions": {
            "urgency": {
                "type": "score",
                "instructions": "How urgent is this message?",
                "criteria": [
                  "Can wait",
                  "Needs attention this week",
                  "Needs attention today"
                ]
            }
        }
    });

    let questions = SystemOneQuestions::default().with_question(
        "urgency",
        SystemOneTaskConfig::score(
            "How urgent is this message?",
            vec![
                "Can wait",
                "Needs attention this week",
                "Needs attention today",
            ],
        ),
    );

    let request: SystemOneRequest<'_> = SystemOneRequest::builder()
        .model("jev-latest")
        .state(
            "I was charged twice for my subscription this month. Please refund the extra charge.",
        )
        .questions(questions)
        .build()
        .unwrap();

    // Convert the struct into a serde_json::Value for structural comparison
    let actual = serde_json::to_value(&request).expect("Failed to serialize request");

    assert_eq!(actual, expected);
}
