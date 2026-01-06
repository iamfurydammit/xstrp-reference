#[cfg(test)]
mod tests {
    use crate::TransferIntent;

    #[test]
    fn create_basic_intent() {
        let intent = TransferIntent::new(1_000_000, 1_768_000_000);
        assert_eq!(intent.amount_xrp_drops, 1_000_000);
        assert_eq!(intent.expiry_unix, 1_768_000_000);
        assert_eq!(intent.protocol_version, "RFC-XSTRP-0001");
        assert_eq!(intent.fee_drops, 10_000);
    }

    #[test]
    fn serialization_round_trip() {
        let intent = TransferIntent::new(500_000, 1_800_000_000);
        let serialized = serde_json::to_string(&intent).unwrap();
        let deserialized: TransferIntent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(intent.amount_xrp_drops, deserialized.amount_xrp_drops);
        assert_eq!(intent.expiry_unix, deserialized.expiry_unix);
    }
}
