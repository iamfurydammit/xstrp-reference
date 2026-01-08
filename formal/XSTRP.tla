---- MODULE XSTRP ----
EXTENDS Naturals

(*
 XSTRP Formal Model
 ------------------
 This module models the frozen XSTRP protocol state space.

 Scope:
 - Phases C1–C4 only
 - No ledger, cryptography, networking, or persistence
 - State-machine logic only
*)

VARIABLE state
States ==
  {
    "Created",
    "Committed",
    "Completed",
    "Expired",
    "Refunded",
    "Invalid"
  }
Init ==
  state = "Created"

Next ==
  \/ /\ state = "Created"
     /\ state' = "Committed"
  \/ /\ state = "Committed"
     /\ state' = "Completed"
  \/ /\ state = "Committed"
     /\ state' = "Expired"


