use uuidv5::{new, Uuid};

fn main() {
    let oids = &["1.2.840.113549", "1.3.6.1.4.1.64517", "2.16.840.1.101"];

    for oid in oids {
        let id = new(Uuid::NAMESPACE_OID, oid.as_bytes());
        println!("{:20} => {}", oid, id);
    }
}
