use uuid::Uuid as RefUuid;
use uuidv5::{new, ParseError, Uuid};

#[test]
fn test_dns_example() {
    let ours = new(Uuid::NAMESPACE_DNS, b"www.example.com");
    let theirs = RefUuid::new_v5(&RefUuid::NAMESPACE_DNS, b"www.example.com");
    assert_eq!(ours.to_string(), theirs.to_string());
}

#[test]
fn test_dns_example_com() {
    let ours = new(Uuid::NAMESPACE_DNS, b"example.com");
    let theirs = RefUuid::new_v5(&RefUuid::NAMESPACE_DNS, b"example.com");
    assert_eq!(ours.to_string(), theirs.to_string());
}

#[test]
fn test_url_namespace() {
    let ours = new(Uuid::NAMESPACE_URL, b"https://example.com");
    let theirs = RefUuid::new_v5(&RefUuid::NAMESPACE_URL, b"https://example.com");
    assert_eq!(ours.to_string(), theirs.to_string());
}

#[test]
fn test_oid_namespace() {
    let ours = new(Uuid::NAMESPACE_OID, b"1.2.3.4");
    let theirs = RefUuid::new_v5(&RefUuid::NAMESPACE_OID, b"1.2.3.4");
    assert_eq!(ours.to_string(), theirs.to_string());
}

#[test]
fn test_x500_namespace() {
    let ours = new(Uuid::NAMESPACE_X500, b"c=US");
    let theirs = RefUuid::new_v5(&RefUuid::NAMESPACE_X500, b"c=US");
    assert_eq!(ours.to_string(), theirs.to_string());
}

#[test]
fn test_empty_name() {
    let ours = new(Uuid::NAMESPACE_DNS, b"");
    let theirs = RefUuid::new_v5(&RefUuid::NAMESPACE_DNS, b"");
    assert_eq!(ours.to_string(), theirs.to_string());
}

#[test]
fn test_deterministic() {
    let a = new(Uuid::NAMESPACE_DNS, b"hello");
    let b = new(Uuid::NAMESPACE_DNS, b"hello");
    assert_eq!(a, b);
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn test_different_names_yield_different_uuids() {
    let a = new(Uuid::NAMESPACE_DNS, b"alpha");
    let b = new(Uuid::NAMESPACE_DNS, b"beta");
    assert_ne!(a, b);
}

#[test]
fn test_round_trip() {
    let u = new(Uuid::NAMESPACE_DNS, b"roundtrip");
    let parsed = Uuid::try_from(u.to_string().as_str()).unwrap();
    assert_eq!(u, parsed);
}

#[test]
fn test_round_trip_namespace_dns() {
    let u = Uuid::NAMESPACE_DNS;
    let parsed = Uuid::try_from(u.to_string().as_str()).unwrap();
    assert_eq!(u, parsed);
}

#[test]
fn test_parse_valid_lowercase() {
    // Generated with uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, b"parse-test")
    let s = "baae46ac-37e7-58a8-ab43-4cfc1106eb9c";
    let u = Uuid::try_from(s).unwrap();
    assert_eq!(u.to_string(), s);
}

#[test]
fn test_parse_valid_uppercase() {
    let s = "BAAE46AC-37E7-58A8-AB43-4CFC1106EB9C";
    let u = Uuid::try_from(s).unwrap();
    assert_eq!(u.to_string(), s.to_lowercase());
}

#[test]
fn test_parse_invalid_length() {
    assert_eq!(
        Uuid::try_from("too-short"),
        Err(ParseError::InvalidLength)
    );
}

#[test]
fn test_parse_invalid_character() {
    let s = "baae46ac-37e7-58a8-ab43-4cfc1106eb9g";
    assert!(matches!(
        Uuid::try_from(s),
        Err(ParseError::InvalidCharacter(..))
    ));
}

#[test]
fn test_parse_wrong_version() {
    let s = "baae46ac-37e7-48a8-ab43-4cfc1106eb9c";
    assert!(matches!(
        Uuid::try_from(s),
        Err(ParseError::InvalidVersion(4))
    ));
}

#[test]
fn test_parse_wrong_variant() {
    let s = "baae46ac-37e7-58a8-0b43-4cfc1106eb9c";
    assert!(matches!(
        Uuid::try_from(s),
        Err(ParseError::InvalidVariant(_))
    ));
}

#[test]
fn test_debug_format() {
    let u = Uuid::NAMESPACE_DNS;
    let debug_str = format!("{:?}", u);
    assert_eq!(debug_str, "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
}

#[test]
fn test_partial_eq() {
    let a = Uuid::from_bytes([0u8; 16]);
    let b = Uuid::from_bytes([0u8; 16]);
    assert_eq!(a, b);
}

#[test]
fn test_partial_ord() {
    let a = Uuid::from_bytes([0u8; 16]);
    let b = Uuid::from_bytes([1u8; 16]);
    assert!(a < b);
}

#[test]
fn test_clone() {
    let u = Uuid::NAMESPACE_DNS;
    let c = u;
    assert_eq!(u, c);
}

#[test]
fn test_copy() {
    let u = Uuid::NAMESPACE_DNS;
    let c = u;
    assert_eq!(u, c);
}

#[test]
fn test_from_bytes() {
    let bytes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let u = Uuid::from(bytes);
    assert_eq!(u.as_bytes(), &bytes);
}

#[test]
fn test_display_format() {
    let u = Uuid::from_bytes([
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
    ]);
    assert_eq!(u.to_string(), "00112233-4455-6677-8899-aabbccddeeff");
}

#[test]
fn test_parse_error_display() {
    let e = ParseError::InvalidLength;
    let s = format!("{}", e);
    assert!(s.contains("length"));

    let e = ParseError::InvalidCharacter('g', 35);
    let s = format!("{}", e);
    assert!(s.contains("'g'"));
    assert!(s.contains("35"));

    let e = ParseError::InvalidVersion(4);
    let s = format!("{}", e);
    assert!(s.contains("4"));

    let e = ParseError::InvalidVariant(0x3F);
    let s = format!("{}", e);
    assert!(s.contains("3f"));
}
