use uuidv5::{new, Uuid};

fn main() {
    let x500s = &["c=US", "c=CN", "c=GB", "c=DE"];

    for x500 in x500s {
        let id = new(Uuid::NAMESPACE_X500, x500.as_bytes());
        println!("{:10} => {}", x500, id);
    }
}
