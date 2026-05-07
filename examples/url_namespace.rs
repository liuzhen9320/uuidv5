use uuidv5::{new, Uuid};

fn main() {
    let urls = &[
        "https://example.com",
        "https://www.rust-lang.org",
        "https://en.wikipedia.org",
    ];

    for url in urls {
        let id = new(Uuid::NAMESPACE_URL, url.as_bytes());
        println!("{:35} => {}", url, id);
    }
}
