use xstrp::{
    execute::execute_transition,
    proofs::{Proof, ReceiverAcknowledgmentProof, SettlementFulfillmentProof},
    state_machine::IntentState,
    validation::ValidationOutcome,
    validation_request::ValidationRequest,
    TransferIntent,
};

fn sample_intent() -> TransferIntent {
    TransferIntent {
        amount_xrp_drops: 1_000_000,
        expiry_unix: 9_999_999,
        protocol_version: 1,
        fee_drops: 10,
        binding: None,
    }
}

#[test]
fn created_to_committed_requires_receiver_proof() {
    let intent = sample_intent();

    let request = ValidationRequest {
        intent: &intent,
        current_state: IntentState::Created,
        requested_state: IntentState::Committed,
        proofs: vec![],
    };

    let (state, outcome) = execute_transition(IntentState::Created, &request);

    assert_eq!(state, IntentState::Created);
    assert!(matches!(outcome, ValidationOutcome::Rejected(_)));
}

#[test]
fn created_to_committed_succeeds_with_receiver_proof() {
    let intent = sample_intent();

    let request = ValidationRequest {
        intent: &intent,
        current_state: IntentState::Created,
        requested_state: IntentState::Committed,
        proofs: vec![Proof::ReceiverAcknowledgment(
            ReceiverAcknowledgmentProof {
                intent_id: "intent-1".to_string(),
                receiver_id: "receiver".to_string(),
            },
        )],
    };

    let (state, outcome) = execute_transition(IntentState::Created, &request);

    assert_eq!(state, IntentState::Committed);
    assert_eq!(outcome, ValidationOutcome::Approved);
}

#[test]
fn terminal_state_cannot_transition() {
    let intent = sample_intent();

    let request = ValidationRequest {
        intent: &intent,
        current_state: IntentState::Completed,
        requested_state: IntentState::Created,
        proofs: vec![],
    };

    let (state, outcome) = execute_transition(IntentState::Completed, &request);

    assert_eq!(state, IntentState::Completed);
    assert!(matches!(outcome, ValidationOutcome::Rejected(_)));
}

#[test]
fn committed_to_completed_requires_settlement_proof() {
    let intent = sample_intent();

    let request = ValidationRequest {
        intent: &intent,
        current_state: IntentState::Committed,
        requested_state: IntentState::Completed,
        proofs: vec![],
    };

    let (state, outcome) = execute_transition(IntentState::Committed, &request);

    assert_eq!(state, IntentState::Committed);
    assert!(matches!(outcome, ValidationOutcome::Rejected(_)));
}

#[test]
fn committed_to_completed_succeeds_with_settlement_proof() {
    let intent = sample_intent();

    let request = ValidationRequest {
        intent: &intent,
        current_state: IntentState::Committed,
        requested_state: IntentState::Completed,
        proofs: vec![Proof::SettlementFulfillment(
            SettlementFulfillmentProof {
                intent_id: "intent-1".to_string(),
                settlement_reference: "settlement-xyz".to_string(),
            },
        )],
    };

    let (state, outcome) = execute_transition(IntentState::Committed, &request);

    assert_eq!(state, IntentState::Completed);
    assert_eq!(outcome, ValidationOutcome::Approved);
}
