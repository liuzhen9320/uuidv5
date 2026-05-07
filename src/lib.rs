#![no_std]

use sha1::{Digest, Sha1};

/// A Universally Unique Identifier (UUID).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid([u8; 16]);

impl Uuid {
    /// DNS namespace UUID.
    pub const NAMESPACE_DNS: Uuid = Uuid([
        0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1,
        0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
    ]);

    /// URL namespace UUID.
    pub const NAMESPACE_URL: Uuid = Uuid([
        0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1,
        0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
    ]);

    /// OID namespace UUID.
    pub const NAMESPACE_OID: Uuid = Uuid([
        0x6b, 0xa7, 0xb8, 0x12, 0x9d, 0xad, 0x11, 0xd1,
        0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
    ]);

    /// X.500 namespace UUID.
    pub const NAMESPACE_X500: Uuid = Uuid([
        0x6b, 0xa7, 0xb8, 0x14, 0x9d, 0xad, 0x11, 0xd1,
        0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
    ]);

    /// Create a `Uuid` from a 16-byte array.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Uuid(bytes)
    }

    /// Return the inner byte array.
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl core::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let b = &self.0;
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3],
            b[4], b[5],
            b[6], b[7],
            b[8], b[9],
            b[10], b[11], b[12], b[13], b[14], b[15],
        )
    }
}

impl From<[u8; 16]> for Uuid {
    fn from(bytes: [u8; 16]) -> Self {
        Uuid(bytes)
    }
}

/// UUID v5 parsing errors.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Input string is not exactly 36 characters long.
    InvalidLength,
    /// Invalid character at a given position.
    InvalidCharacter(char, usize),
    /// Version nibble is not 5.
    InvalidVersion(u8),
    /// Variant bits are not `10xx`.
    InvalidVariant(u8),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::InvalidLength => write!(f, "invalid UUID length: expected 36 characters"),
            ParseError::InvalidCharacter(c, pos) => {
                write!(f, "invalid character '{}' at position {}", c, pos)
            }
            ParseError::InvalidVersion(v) => write!(f, "invalid version {}: expected 5", v),
            ParseError::InvalidVariant(v) => {
                write!(f, "invalid variant {:02x}: expected 8x/9x/Ax/Bx", v)
            }
        }
    }
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

impl TryFrom<&str> for Uuid {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let bytes = s.as_bytes();
        if bytes.len() != 36 {
            return Err(ParseError::InvalidLength);
        }

        // Check hyphen positions
        for &pos in &[8, 13, 18, 23] {
            if bytes[pos] != b'-' {
                let ch = s.chars().nth(pos).unwrap_or('\0');
                return Err(ParseError::InvalidCharacter(ch, pos));
            }
        }

        let mut buf = [0u8; 16];
        let hex_positions: [(usize, usize); 16] = [
            (0, 1), (2, 3), (4, 5), (6, 7),
            (9, 10), (11, 12),
            (14, 15), (16, 17),
            (19, 20), (21, 22),
            (24, 25), (26, 27), (28, 29), (30, 31), (32, 33), (34, 35),
        ];

        for (i, (hi, lo)) in hex_positions.iter().enumerate() {
            let h = hex_val(bytes[*hi]).ok_or(ParseError::InvalidCharacter(
                s.chars().nth(*hi).unwrap_or('\0'),
                *hi,
            ))?;
            let l = hex_val(bytes[*lo]).ok_or(ParseError::InvalidCharacter(
                s.chars().nth(*lo).unwrap_or('\0'),
                *lo,
            ))?;
            buf[i] = (h << 4) | l;
        }

        // Verify version (nibble at byte 6, high nibble)
        let version = (buf[6] >> 4) & 0x0F;
        if version != 5 {
            return Err(ParseError::InvalidVersion(version));
        }

        // Verify variant (byte 8, high 2 bits must be 10)
        let variant = buf[8] >> 6;
        if variant != 0b10 {
            return Err(ParseError::InvalidVariant(buf[8]));
        }

        Ok(Uuid(buf))
    }
}

/// Generate a UUID v5 from a namespace UUID and a name.
///
/// # Example
///
/// ```
/// use uuidv5::{Uuid, new};
///
/// let id = new(Uuid::NAMESPACE_DNS, b"example.com");
/// println!("{}", id);
/// ```
pub fn new(namespace: Uuid, name: &[u8]) -> Uuid {
    let mut hasher = Sha1::new();
    hasher.update(&namespace.0);
    hasher.update(name);
    let hash = hasher.finalize();

    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&hash[..16]);

    // Set version to 5: byte 6 high nibble = 5
    bytes[6] = (bytes[6] & 0x0F) | 0x50;
    // Set variant to RFC 4122: byte 8 high 2 bits = 10
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    Uuid(bytes)
}

/// Generate a UUID v5 from a namespace UUID and a name.
///
/// Equivalent to [`new`].
///
/// # Example
///
/// ```
/// use uuidv5::uuidv5;
///
/// let id = uuidv5!(uuidv5::Uuid::NAMESPACE_DNS, b"example.com");
/// println!("{}", id);
/// ```
#[macro_export]
macro_rules! uuidv5 {
    ($ns:expr, $name:expr) => {
        $crate::new($ns, $name)
    };
}
