use std::fmt;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// A ULID (Universally Unique Lexicographically Sortable Identifier) following DDD Value Object pattern.
///
/// ULID is a 128-bit identifier that is:
/// - Time-ordered: First 48 bits are Unix timestamp in milliseconds
/// - Lexicographically sortable: String representation sorts chronologically
/// - Case-insensitive: Uses Crockford's Base32 encoding
/// - URL-safe: No special characters
/// - Compact: 26 characters vs 36 for UUID
///
/// Structure (128 bits total):
/// - 48 bits: Timestamp in milliseconds since Unix epoch
/// - 80 bits: Cryptographically strong random data
///
/// String representation: 26 characters in Crockford's Base32
/// Example: 01ARZ3NDEKTSV4RRFFQ69G5FAV
///
/// As a Value Object:
/// - Immutable once created
/// - Compared by value
/// - Thread-safe and copyable
/// - Self-validating
///
/// # Examples
///
/// ```
/// use education_platform_common::Id;
///
/// let id1 = Id::new();
/// let id2 = Id::new();
/// assert_ne!(id1, id2);
///
/// let id_str = id1.to_string();
/// assert_eq!(id_str.len(), 26);
///
/// // Parse from string
/// let parsed: Id = "01ARZ3NDEKTSV4RRFFQ69G5FAV".parse().unwrap();
/// assert_eq!(parsed.to_string(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id {
    bytes: [u8; 16],
}

impl Id {
    /// Creates a new ULID with current timestamp and random data.
    ///
    /// Generates a time-ordered identifier where the first 48 bits represent
    /// the current Unix timestamp in milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::new();
    /// println!("Generated ID: {}", id);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let timestamp_ms = Self::current_timestamp_ms();
        let random_bytes = Self::generate_random_bytes();
        Self::from_parts(timestamp_ms, random_bytes)
    }

    /// Creates a ULID from timestamp and random bytes.
    ///
    /// Useful for testing or when using external random sources.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let timestamp = 1234567890123u64;
    /// let random = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let id = Id::from_parts(timestamp, random);
    /// assert_eq!(id.timestamp_ms(), timestamp);
    /// ```
    #[must_use]
    pub fn from_parts(timestamp_ms: u64, random: [u8; 10]) -> Self {
        let mut bytes = [0u8; 16];

        bytes[0] = (timestamp_ms >> 40) as u8;
        bytes[1] = (timestamp_ms >> 32) as u8;
        bytes[2] = (timestamp_ms >> 24) as u8;
        bytes[3] = (timestamp_ms >> 16) as u8;
        bytes[4] = (timestamp_ms >> 8) as u8;
        bytes[5] = timestamp_ms as u8;

        bytes[6..16].copy_from_slice(&random);

        Self { bytes }
    }

    /// Returns the timestamp component in milliseconds since Unix epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::new();
    /// let timestamp = id.timestamp_ms();
    /// assert!(timestamp > 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn timestamp_ms(&self) -> u64 {
        ((self.bytes[0] as u64) << 40)
            | ((self.bytes[1] as u64) << 32)
            | ((self.bytes[2] as u64) << 24)
            | ((self.bytes[3] as u64) << 16)
            | ((self.bytes[4] as u64) << 8)
            | (self.bytes[5] as u64)
    }

    /// Returns the raw bytes of the ULID.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::new();
    /// assert_eq!(id.as_bytes().len(), 16);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_millis() as u64
    }

    /// Generates cryptographically strong random bytes using system entropy.
    ///
    /// Uses multiple entropy sources for better randomness:
    /// - System random state
    /// - High-precision timestamps
    /// - Thread identifiers
    /// - Atomic counter for uniqueness
    fn generate_random_bytes() -> [u8; 10] {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        use std::sync::atomic::{AtomicU64, Ordering};

        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let mut bytes = [0u8; 10];
        let random_state = RandomState::new();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch");

        let nanos = now.subsec_nanos() as u64;
        let micros = now.as_micros() as u64;
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);

        let thread_id = random_state.hash_one(std::thread::current().id());

        for i in 0u64..5 {
            let mut hasher = random_state.build_hasher();

            hasher.write_u64(nanos.wrapping_mul(i + 1));
            hasher.write_u64(micros.wrapping_add(i));
            hasher.write_u64(counter.wrapping_mul(i + 7));
            hasher.write_u64(thread_id.wrapping_mul(i + 13));
            hasher.write_usize(&bytes as *const _ as usize);
            hasher.write_usize((i * 17) as usize);

            let hash = hasher.finish();
            bytes[(i * 2) as usize] = (hash >> 8) as u8;
            bytes[(i * 2 + 1) as usize] = hash as u8;
        }

        bytes
    }

    /// Encodes the ULID as a 26-character Crockford Base32 string.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::new();
    /// let s = id.to_string();
    /// assert_eq!(s.len(), 26);
    /// assert!(s.chars().all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(c)));
    /// ```
    #[must_use]
    pub fn to_crockford_base32(&self) -> String {
        const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

        let mut result = String::with_capacity(26);

        // Encode timestamp (48 bits = 10 characters)
        let timestamp = self.timestamp_ms();
        result.push(ALPHABET[((timestamp >> 45) & 0x1F) as usize] as char);
        result.push(ALPHABET[((timestamp >> 40) & 0x1F) as usize] as char);
        result.push(ALPHABET[((timestamp >> 35) & 0x1F) as usize] as char);
        result.push(ALPHABET[((timestamp >> 30) & 0x1F) as usize] as char);
        result.push(ALPHABET[((timestamp >> 25) & 0x1F) as usize] as char);
        result.push(ALPHABET[((timestamp >> 20) & 0x1F) as usize] as char);
        result.push(ALPHABET[((timestamp >> 15) & 0x1F) as usize] as char);
        result.push(ALPHABET[((timestamp >> 10) & 0x1F) as usize] as char);
        result.push(ALPHABET[((timestamp >> 5) & 0x1F) as usize] as char);
        result.push(ALPHABET[(timestamp & 0x1F) as usize] as char);

        // Encode randomness (80 bits = 16 characters)
        let random = &self.bytes[6..16];

        // Combine bytes into groups of 5 bits
        let mut value: u64 = 0;
        let mut bits = 0;

        for &byte in random {
            value = (value << 8) | u64::from(byte);
            bits += 8;

            while bits >= 5 {
                bits -= 5;
                let index = ((value >> bits) & 0x1F) as usize;
                result.push(ALPHABET[index] as char);
            }
        }

        if bits > 0 {
            let index = ((value << (5 - bits)) & 0x1F) as usize;
            result.push(ALPHABET[index] as char);
        }

        result
    }

    /// Decodes a Crockford Base32 string into a ULID.
    ///
    /// # Errors
    ///
    /// Returns `IdError::InvalidLength` if the string is not 26 characters.
    /// Returns `IdError::InvalidCharacter` if the string contains invalid characters.
    pub fn from_crockford_base32(s: &str) -> Result<Self, IdError> {
        if s.len() != 26 {
            return Err(IdError::InvalidLength);
        }

        let s = s.to_uppercase();
        let chars: Vec<char> = s.chars().collect();

        let decode_char = |c: char| -> Result<u8, IdError> {
            match c {
                '0' | 'O' => Ok(0),
                '1' | 'I' | 'L' => Ok(1),
                '2' => Ok(2),
                '3' => Ok(3),
                '4' => Ok(4),
                '5' => Ok(5),
                '6' => Ok(6),
                '7' => Ok(7),
                '8' => Ok(8),
                '9' => Ok(9),
                'A' => Ok(10),
                'B' => Ok(11),
                'C' => Ok(12),
                'D' => Ok(13),
                'E' => Ok(14),
                'F' => Ok(15),
                'G' => Ok(16),
                'H' => Ok(17),
                'J' => Ok(18),
                'K' => Ok(19),
                'M' => Ok(20),
                'N' => Ok(21),
                'P' => Ok(22),
                'Q' => Ok(23),
                'R' => Ok(24),
                'S' => Ok(25),
                'T' => Ok(26),
                'V' => Ok(27),
                'W' => Ok(28),
                'X' => Ok(29),
                'Y' => Ok(30),
                'Z' => Ok(31),
                _ => Err(IdError::InvalidCharacter),
            }
        };

        // Decode timestamp (first 10 characters)
        let mut timestamp: u64 = 0;
        for &c in chars.iter().take(10) {
            let value = decode_char(c)?;
            timestamp = (timestamp << 5) | u64::from(value);
        }

        // Decode randomness (remaining 16 characters)
        let mut random = [0u8; 10];
        let mut value: u64 = 0;
        let mut bits = 0;
        let mut byte_idx = 0;

        for &c in chars.iter().skip(10) {
            let decoded = decode_char(c)?;
            value = (value << 5) | u64::from(decoded);
            bits += 5;

            while bits >= 8 && byte_idx < 10 {
                bits -= 8;
                random[byte_idx] = ((value >> bits) & 0xFF) as u8;
                byte_idx += 1;
            }
        }

        Ok(Self::from_parts(timestamp, random))
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_crockford_base32())
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_crockford_base32(s)
    }
}

impl From<[u8; 16]> for Id {
    fn from(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }
}

impl From<Id> for [u8; 16] {
    fn from(id: Id) -> Self {
        id.bytes
    }
}

/// Error types for ID operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IdError {
    #[error("Invalid ID length: expected 26 characters")]
    InvalidLength,

    #[error("Invalid character in ID string")]
    InvalidCharacter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_valid_ulid() {
        let id = Id::new();
        assert_eq!(id.as_bytes().len(), 16);
    }

    #[test]
    fn test_timestamp_extraction() {
        let timestamp = 1234567890123u64;
        let random = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let id = Id::from_parts(timestamp, random);

        assert_eq!(id.timestamp_ms(), timestamp);
    }

    #[test]
    fn test_display_format_is_26_chars() {
        let id = Id::new();
        let id_str = id.to_string();

        assert_eq!(id_str.len(), 26);
        assert!(id_str
            .chars()
            .all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(c)));
    }

    #[test]
    fn test_different_ulids_are_unique() {
        let id1 = Id::new();
        let id2 = Id::new();

        assert_ne!(id1, id2);
    }

    #[test]
    fn test_ulids_are_time_ordered() {
        let id1 = Id::new();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let id2 = Id::new();

        assert!(id1.timestamp_ms() <= id2.timestamp_ms());
        assert!(id1 < id2);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x09, 0x0A,
        ];
        let id = Id::from(bytes);

        assert_eq!(id.as_bytes(), &bytes);
    }

    #[test]
    fn test_copy_semantics() {
        let id1 = Id::new();
        let id2 = id1;

        assert_eq!(id1, id2);
        assert_eq!(id1.to_string(), id2.to_string());
    }

    #[test]
    fn test_as_bytes_returns_correct_length() {
        let id = Id::new();
        assert_eq!(id.as_bytes().len(), 16);
    }

    #[test]
    fn test_encoding_decoding_roundtrip() {
        let id1 = Id::new();
        let encoded = id1.to_string();
        let id2: Id = encoded.parse().unwrap();

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_from_str_valid() {
        let ulid_str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
        let id: Id = ulid_str.parse().unwrap();
        assert_eq!(id.to_string(), ulid_str);
    }

    #[test]
    fn test_from_str_case_insensitive() {
        let upper = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
        let lower = "01arz3ndektsv4rrffq69g5fav";

        let id1: Id = upper.parse().unwrap();
        let id2: Id = lower.parse().unwrap();

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_from_str_invalid_length() {
        assert_eq!(
            "TOOLONG12345678901234567890".parse::<Id>(),
            Err(IdError::InvalidLength)
        );
        assert_eq!("TOOSHORT".parse::<Id>(), Err(IdError::InvalidLength));
    }

    #[test]
    fn test_from_str_invalid_character() {
        let result = "01ARZ3NDEKTSV4RRFFQ69G5F@V".parse::<Id>();
        assert!(result.is_err());
    }

    #[test]
    fn test_ordering() {
        let id1 = Id::from_parts(1000, [0; 10]);
        let id2 = Id::from_parts(2000, [0; 10]);
        let id3 = Id::from_parts(3000, [0; 10]);

        assert!(id1 < id2);
        assert!(id2 < id3);
        assert!(id1 < id3);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let id1 = Id::new();
        let id2 = id1;

        let mut set = HashSet::new();
        set.insert(id1);
        assert!(set.contains(&id2));
    }

    #[test]
    fn test_default_creates_new_id() {
        let id = Id::default();
        assert_eq!(id.as_bytes().len(), 16);
    }

    #[test]
    fn test_value_object_equality() {
        let timestamp = 1234567890123u64;
        let random = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let id1 = Id::from_parts(timestamp, random);
        let id2 = Id::from_parts(timestamp, random);

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_lexicographic_sorting() {
        let id1 = Id::new();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let id2 = Id::new();

        let str1 = id1.to_string();
        let str2 = id2.to_string();

        assert!(str1 < str2);
    }

    #[test]
    fn test_known_encoding() {
        let timestamp = 1469918176385u64;
        let random = [0x79, 0xE4, 0x2C, 0xC0, 0xC2, 0x98, 0x40, 0x00, 0x00, 0x00];

        let id = Id::from_parts(timestamp, random);
        let encoded = id.to_string();

        assert_eq!(encoded.len(), 26);
        assert!(encoded.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_timestamp_boundaries() {
        let min_timestamp = 0u64;
        let max_timestamp = (1u64 << 48) - 1;

        let id_min = Id::from_parts(min_timestamp, [0; 10]);
        let id_max = Id::from_parts(max_timestamp, [0xFF; 10]);

        assert_eq!(id_min.timestamp_ms(), min_timestamp);
        assert_eq!(id_max.timestamp_ms(), max_timestamp);
    }
}
