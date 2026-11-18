//! This crate test the deserialization of the data, to be sure the LLM response is correctly parsed

use system_of_a_one::{SystemOneDecision, TaskResult};

#[test]
/// Try to deserialize a simulated response from a system one api
fn test_deserialize_system_one_decision() {
    let raw_json = r#"
    {
        "model": "jev-latest",
        "answers": {
            "billing": {
                "type": "noul",
                "noul": 0.92
            },
            "department": {
                "type": "choice",
                "choice": "billing",
                "confidence": 0.94,
                "probabilities": {
                    "returns": 0.02,
                    "shipping": 0.04,
                    "billing": 0.94
                }
            },
            "urgency": {
                "type": "score",
                "score": 0.75
            }
        },
        "usage": {
            "input_tokens": 42,
            "output_tokens": 15
        }
    }
    "#;

    let decision: SystemOneDecision =
        serde_json::from_str(raw_json).expect("Failed to deserialize SystemOneDecision");

    assert_eq!(decision.model, "jev-latest");
    assert_eq!(decision.usage.input_tokens, 42);
    assert_eq!(decision.usage.output_tokens, 15);
    assert_eq!(decision.answers.len(), 3);

    // Check Noul variant
    match decision.answers.get("billing") {
        Some(TaskResult::Noul { noul }) => assert!((noul - 0.92).abs() < f64::EPSILON),
        other => panic!("Expected Noul variant, got {:?}", other),
    }

    // Check Choice variant
    match decision.answers.get("department") {
        Some(TaskResult::Choice {
            choice,
            confidence,
            probabilities,
        }) => {
            assert_eq!(choice, "billing");
            assert!((confidence - 0.94).abs() < f64::EPSILON);
            assert_eq!(probabilities.get("billing"), Some(&0.94));
        }
        other => panic!("Expected Choice variant, got {:?}", other),
    }

    // Check Score variant (testing #[serde(default)] on optional confidence)
    match decision.answers.get("urgency") {
        Some(TaskResult::Score { score, confidence }) => {
            assert!((score - 0.75).abs() < f64::EPSILON);
            assert_eq!(*confidence, None);
        }
        other => panic!("Expected Score variant, got {:?}", other),
    }
}

#[test]
fn test_deserialize_unknown_task_type_fails() {
    let invalid_json = r#"
    {
        "model": "jev-latest",
        "answers": {
            "test": { "type": "unknown", "value": 1 }
        },
        "usage": { "input_token": 10, "output_token": 5 }
    }
    "#;

    let result = serde_json::from_str::<SystemOneDecision>(invalid_json);
    assert!(result.is_err());
}
