use uuidv5::{new, Uuid};

fn main() {
    let id = new(Uuid::NAMESPACE_DNS, b"example.com");
    println!("UUID v5: {}", id);
}
