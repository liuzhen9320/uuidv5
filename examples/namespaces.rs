use uuidv5::Uuid;

fn main() {
    let namespaces = &[
        ("DNS", Uuid::NAMESPACE_DNS),
        ("URL", Uuid::NAMESPACE_URL),
        ("OID", Uuid::NAMESPACE_OID),
        ("X500", Uuid::NAMESPACE_X500),
    ];

    for (name, ns) in namespaces {
        println!("NAMESPACE_{:<4} = {}", name, ns);

        // Verify version is 5
        let bytes = ns.as_bytes();
        let version = (bytes[6] >> 4) & 0x0F;
        let variant = bytes[8] >> 6;
        println!("  version: {}, variant: {:02b}", version, variant);
    }
}
