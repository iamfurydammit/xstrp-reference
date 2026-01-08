----------------------------- MODULE XSTRP_C5 -----------------------------

EXTENDS Naturals, TLC

(*
Phase C5 formal model for XSTRP.

This module models only the protocol logic defined in Phases C1–C4.
It does not model XRPL behavior, cryptography, wallets, networking,
or persistence.

This is a safety-only model.
*)

VARIABLES
    intent_state,
    authorization_valid,
    proof_present,
    is_expired

(*
The set of all valid protocol states.
*)
States ==
{
    "Created",
    "Committed",
    "Completed",
    "Expired",
    "Refunded",
    "Invalid"
}

(*
Initial conditions.
*)
Init ==
    /\ intent_state = "Created"
    /\ authorization_valid = FALSE
    /\ proof_present = FALSE
    /\ is_expired = FALSE

(*
Abstract transition relation.
Each action corresponds to a documented protocol transition.
*)

CreatedToCommitted ==
    /\ intent_state = "Created"
    /\ authorization_valid = TRUE
    /\ proof_present = TRUE
    /\ is_expired = FALSE
    /\ intent_state' = "Committed"
    /\ UNCHANGED << authorization_valid, proof_present, is_expired >>

CreatedToInvalid ==
    /\ intent_state = "Created"
    /\ proof_present = TRUE
    /\ authorization_valid = FALSE
    /\ intent_state' = "Invalid"
    /\ UNCHANGED << authorization_valid, proof_present, is_expired >>

CreatedToExpired ==
    /\ intent_state = "Created"
    /\ is_expired = TRUE
    /\ intent_state' = "Expired"
    /\ UNCHANGED << authorization_valid, proof_present, is_expired >>

CommittedToCompleted ==
    /\ intent_state = "Committed"
    /\ authorization_valid = TRUE
    /\ proof_present = TRUE
    /\ is_expired = FALSE
    /\ intent_state' = "Completed"
    /\ UNCHANGED << authorization_valid, proof_present, is_expired >>

CommittedToInvalid ==
    /\ intent_state = "Committed"
    /\ proof_present = TRUE
    /\ authorization_valid = FALSE
    /\ intent_state' = "Invalid"
    /\ UNCHANGED << authorization_valid, proof_present, is_expired >>

CommittedToExpired ==
    /\ intent_state = "Committed"
    /\ is_expired = TRUE
    /\ intent_state' = "Expired"
    /\ UNCHANGED << authorization_valid, proof_present, is_expired >>

ExpiredToRefunded ==
    /\ intent_state = "Expired"
    /\ intent_state' = "Refunded"
    /\ UNCHANGED << authorization_valid, proof_present, is_expired >>

(*
Terminal states have no outgoing transitions.
*)

Next ==
    \/ CreatedToCommitted
    \/ CreatedToInvalid
    \/ CreatedToExpired
    \/ CommittedToCompleted
    \/ CommittedToInvalid
    \/ CommittedToExpired
    \/ ExpiredToRefunded

(*
Safety invariants corresponding to formal/INVARIANTS.md
*)

ValidStateSpace ==
    intent_state \in States

TerminalStates ==
    {"Completed", "Refunded", "Invalid"}

NoCompletionWithoutAuthorization ==
    intent_state = "Completed" =>
        /\ authorization_valid = TRUE
        /\ proof_present = TRUE
        /\ is_expired = FALSE

NoCompletionAfterExpiry ==
    intent_state = "Completed" =>
        is_expired = FALSE

AuthorizationGating ==
    (intent_state' = "Completed") =>
        authorization_valid = TRUE

NoStrandedFunds ==
    intent_state \notin TerminalStates =>
        (intent_state = "Created" \/ intent_state = "Committed" \/ intent_state = "Expired")
=============================================================================
