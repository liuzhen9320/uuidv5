use uuidv5::{new, Uuid};

fn main() {
    let names = &[
        "rust-lang.org",
        "github.com",
        "crates.io",
        "docs.rs",
        "wikipedia.org",
    ];

    for name in names {
        let id = new(Uuid::NAMESPACE_DNS, name.as_bytes());
        println!("{:25} => {}", name, id);
    }
}
