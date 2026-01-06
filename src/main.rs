use xstrp::TransferIntent;

fn main() {
    let intent = TransferIntent::new(1_000_000, 1_768_000_000);
    println!("Created intent: {:?}", intent);

    let serialized = serde_json::to_string_pretty(&intent).unwrap();
    println!("\nSerialized JSON:\n{}", serialized);
}
