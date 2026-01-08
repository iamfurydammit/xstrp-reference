use xstrp::{
    execute::execute_transition,
    proofs::{
        Proof, ReceiverAcknowledgmentProof, SenderCommitmentProof,
        SettlementFulfillmentProof,
    },
    state_machine::IntentState,
    validation::ValidationOutcome,
    validation_request::ValidationRequest,
    TransferIntent,
};

fn sample_intent() -> TransferIntent {
    TransferIntent {
        id: "intent-1".to_string(),
        sender: "alice".to_string(),
        receiver: "bob".to_string(),
        amount: 100,
        expiry: 999999,
    }
}

#[test]
fn created_to_receiver_confirmed_requires_receiver_proof() {
    let intent = sample_intent();

    let request = ValidationRequest {
        intent: &intent,
        current_state: IntentState::Created,
        requested_state: IntentState::ReceiverConfirmed,
        proofs: vec![],
    };

    let (state, outcome) =
        execute_transition(IntentState::Created, &request);

    assert_eq!(state, IntentState::Created);
    assert!(matches!(outcome, ValidationOutcome::Rejected(_)));
}

#[test]
fn created_to_receiver_confirmed_succeeds_with_proof() {
    let intent = sample_intent();

    let request = ValidationRequest {
        intent: &intent,
        current_state: IntentState::Created,
        requested_state: IntentState::ReceiverConfirmed,
        proofs: vec![Proof::ReceiverAcknowledgment(
            ReceiverAcknowledgmentProof {
                intent_id: intent.id.clone(),
                receiver_id: intent.receiver.clone(),
            },
        )],
    };

    let (state, outcome) =
        execute_transition(IntentState::Created, &request);

    assert_eq!(state, IntentState::ReceiverConfirmed);
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

    let (state, outcome) =
        execute_transition(IntentState::Completed, &request);

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

    let (state, outcome) =
        execute_transition(IntentState::Committed, &request);

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
                intent_id: intent.id.clone(),
                settlement_reference: "settlement-xyz".to_string(),
            },
        )],
    };

    let (state, outcome) =
        execute_transition(IntentState::Committed, &request);

    assert_eq!(state, IntentState::Completed);
    assert_eq!(outcome, ValidationOutcome::Approved);
}
