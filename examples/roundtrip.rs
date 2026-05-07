use uuidv5::{new, Uuid};

fn main() {
    // Generate a UUID v5 and round-trip it through Display + TryFrom
    let id = new(Uuid::NAMESPACE_DNS, b"roundtrip-test");

    let s = id.to_string();
    println!("Generated:  {}", s);

    let parsed = Uuid::try_from(s.as_str()).expect("round-trip failed");
    println!("Parsed:     {}", parsed);

    assert_eq!(id, parsed);
    println!("Round-trip successful!");
}
