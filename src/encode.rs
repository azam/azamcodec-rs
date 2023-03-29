use std::io::{ErrorKind, Read, Result, Write};

const LOWER_ALPHABETS: &'static [u8] = b"0123456789abcdef";
const HIGHER_ALPHABETS: &'static [u8] = b"ghjkmnpqrstvwxyz";

enum LeadNybbleStatus {
    None,
    PreviousHigh,
    PreviousLow,
}
/// Given a source of a [`Read`] instance of byte stream, encode to Azam codeca nd write to [`Write`] instance.
///
/// # Arguments
///
/// * `reader` - Byte stream
/// * `writer` - Stream to write Azam codec encoded bytes
/// * `limit` - Maximum number of bytes to read
///
/// # Examples
/// ```rust
/// use azamcodec::encode::azam_encode_write;
///
/// let mut src = 0xdeadbeefu32.to_be_bytes();
/// let mut dst = Vec::<u8>::new();
/// azam_encode_write(&mut src.as_slice(), &mut dst).unwrap();
/// let encoded = String::from_utf8(dst).unwrap(); // "xytxvyyfh5wgg1"
/// ```
pub fn azam_encode_write<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<usize> {
    let mut byte = [0u8; 1];
    let mut count: usize = 0;

    // Read first byte
    reader.read_exact(&mut byte)?;
    count += 1;

    // Update lead nybble status
    let mut lead_nybble = if (byte[0] >> 4) > 0 {
        LeadNybbleStatus::PreviousHigh
    } else if (byte[0] & 0x0fu8) > 0 {
        LeadNybbleStatus::PreviousLow
    } else {
        LeadNybbleStatus::None
    };
    let mut prev_byte = byte[0];

    loop {
        // Read and consume some of read error
        match reader.read_exact(&mut byte) {
            Ok(()) => {
                count += 1;
                match lead_nybble {
                    LeadNybbleStatus::None => {
                        // Not flushing previous byte
                        // Update lead nybble status
                        lead_nybble = if (byte[0] >> 4) > 0 {
                            LeadNybbleStatus::PreviousHigh
                        } else if (byte[0] & 0x0fu8) > 0 {
                            LeadNybbleStatus::PreviousLow
                        } else {
                            LeadNybbleStatus::None
                        };
                    }
                    LeadNybbleStatus::PreviousHigh => {
                        // Flush previous byte as highs
                        let high_nybble = prev_byte >> 4;
                        let low_nybble = prev_byte & 0x0fu8;
                        writer.write(&[HIGHER_ALPHABETS[high_nybble as usize]])?;
                        writer.write(&[HIGHER_ALPHABETS[low_nybble as usize]])?;
                    }
                    LeadNybbleStatus::PreviousLow => {
                        // Only flush previous byte's low nybble as high
                        let low_nybble = prev_byte & 0x0fu8;
                        writer.write(&[HIGHER_ALPHABETS[low_nybble as usize]])?;
                        lead_nybble = LeadNybbleStatus::PreviousHigh;
                    }
                }
                prev_byte = byte[0];
            }
            Err(err) => {
                // Consume EOF error, because we expect it
                if err.kind() == ErrorKind::UnexpectedEof {
                    // Flushing of previous byte done after exit loop
                    break;
                } else {
                    // Rethrow error
                    return Err(err);
                }
            }
        }
    }

    // Flush last byte
    match lead_nybble {
        LeadNybbleStatus::None => {
            if count > 0 {
                // Previous byte is 0x00
                // Flush as low nybble
                writer.write(&[LOWER_ALPHABETS[0]])?;
            }
        }
        LeadNybbleStatus::PreviousHigh => {
            // Flush previous byte's high nybble as high
            // Flush previous byte's low nybble as low
            let high_nybble = prev_byte >> 4;
            let low_nybble = prev_byte & 0x0fu8;
            writer.write(&[HIGHER_ALPHABETS[high_nybble as usize]])?;
            writer.write(&[LOWER_ALPHABETS[low_nybble as usize]])?;
        }
        LeadNybbleStatus::PreviousLow => {
            // Flush previous byte's low nybble as low
            let low_nybble = prev_byte & 0x0fu8;
            writer.write(&[LOWER_ALPHABETS[low_nybble as usize]])?;
        }
    }

    Ok(count)
}

/// Given a source of a [`Read`] instance of byte stream, encode to Azam codec and return as byte array.
///
/// # Arguments
///
/// * `value` - Byte array
///
/// # Examples
/// ```rust
/// use azamcodec::encode::azam_encode_bytes_to_bytes;
///
/// let mut src = 0xdeadbeefu32.to_be_bytes().to_vec();
/// let dst = azam_encode_bytes_to_bytes(src);
/// let encoded = String::from_utf8(dst).unwrap(); // "xytxvyyf"
/// ```
pub fn azam_encode_bytes_to_bytes(value: Vec<u8>) -> Vec<u8> {
    let mut encoded = Vec::<u8>::new();
    azam_encode_write(&mut value.as_slice(), &mut encoded).unwrap();
    encoded
}

/// Given a source of byte stream, encode to Azam codec and return as string.
///
/// # Arguments
///
/// * `value` - Byte array
///
/// # Examples
/// ```rust
/// use azamcodec::encode::azam_encode_bytes;
///
/// let mut src = 0xdeadbeefu32.to_be_bytes().to_vec();
/// let encoded = azam_encode_bytes(src); // "xytxvyyf"
/// ```
pub fn azam_encode_bytes(value: Vec<u8>) -> String {
    String::from_utf8(azam_encode_bytes_to_bytes(value)).unwrap()
}

/// Given a source of multiple byte stream, encode to multi-section Azam codec and return as mstring.
///
/// # Arguments
///
/// * `value` - Byte array
///
/// # Examples
/// ```rust
/// use azamcodec::encode::azam_encode_bytes_vec_to_bytes;
///
/// let mut src = Vec::<Vec<u8>>::new();
/// src.push(0xdeadbeefu32.to_be_bytes().to_vec());
/// src.push(0x15u8.to_be_bytes().to_vec());
/// src.push(0xc001u16.to_be_bytes().to_vec());
/// let dst = azam_encode_bytes_vec_to_bytes(src);
/// let encoded = String::from_utf8(dst); // "xytxvyyfh5wgg1"
/// ```
pub fn azam_encode_bytes_vec_to_bytes(value: Vec<Vec<u8>>) -> Vec<u8> {
    let mut encoded = Vec::<u8>::new();
    for i in 0..value.len() {
        encoded.append(&mut azam_encode_bytes_to_bytes(value[i].clone()));
    }
    encoded
}

/// Given a source of multiple byte stream, encode to multi-section Azam codec and return as a string.
///
/// # Arguments
///
/// * `value` - Byte array
///
/// # Examples
/// ```rust
/// use azamcodec::encode::azam_encode_bytes_vec;
///
/// let mut src = Vec::<Vec<u8>>::new();
/// src.push(0xdeadbeefu32.to_be_bytes().to_vec());
/// src.push(0x15u8.to_be_bytes().to_vec());
/// src.push(0xc001u16.to_be_bytes().to_vec());
/// let encoded = azam_encode_bytes_vec(src); // "xytxvyyfh5wgg1"
/// ```
pub fn azam_encode_bytes_vec(value: Vec<Vec<u8>>) -> String {
    String::from_utf8(azam_encode_bytes_vec_to_bytes(value)).unwrap()
}

/// Trait to extend types to support encoding to Azam codec.
pub trait AzamEncode {
    /// Given a destination of a [`Write`] instance, generate byte array as needed, and write Azam codec encoded byte stream.
    ///
    /// # Arguments
    ///
    /// * `writer` - Azam codec encoded stream
    fn azam_encode_write<W: Write>(&self, writer: &mut W) -> Result<usize>;

    /// Generate byte array as needed and return Azam codec encoded string.
    fn azam_encode(&self) -> String;
}

macro_rules! azam_encode_impl {
    ($t:ty) => {
        impl AzamEncode for $t {
            fn azam_encode_write<W: Write>(&self, writer: &mut W) -> Result<usize> {
                azam_encode_write(&mut self.to_be_bytes().as_ref(), writer)
            }

            fn azam_encode(&self) -> String {
                let mut bytes = Vec::<u8>::new();
                self.azam_encode_write(&mut bytes).unwrap();
                String::from_utf8(bytes).unwrap()
            }
        }
    };
}

azam_encode_impl!(u8);
azam_encode_impl!(u16);
azam_encode_impl!(u32);
azam_encode_impl!(u64);
azam_encode_impl!(u128);

/// Macro to encode tuples of any types that implements the [`AzamEncode`] trait to Azam codec encoded string.
///
/// # Examples
///
/// ```rust
/// use azamcodec::{azam_encode, encode::AzamEncode};
/// // Encode multiple values as Azam-encoded string, using azam_encode! macro.
/// // 0xdeadbeefu32 encodes to "xytxvyyf".
/// // 0x15u8 encodes to "h5".
/// // 0xc001u16 encodes to "wgg1".
/// let x = azam_encode!(0xdeadbeefu32, 0x15u8, 0xc001u16); // "xytxvyyfh5wgg1"
/// ```
#[macro_export]
macro_rules! azam_encode {
    () => {};
    ($v1:expr) => {{
        $v1.azam_encode()
    }};
    ($v1:expr, $v2:expr) => {{
        let mut bytes = Vec::<u8>::new();
        $v1.azam_encode_write(&mut bytes).unwrap();
        $v2.azam_encode_write(&mut bytes).unwrap();
        String::from_utf8(bytes).unwrap()
    }};
    ($v1:expr, $v2:expr, $v3:expr) => {{
        let mut bytes = Vec::<u8>::new();
        $v1.azam_encode_write(&mut bytes).unwrap();
        $v2.azam_encode_write(&mut bytes).unwrap();
        $v3.azam_encode_write(&mut bytes).unwrap();
        String::from_utf8(bytes).unwrap()
    }};
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr) => {{
        let mut bytes = Vec::<u8>::new();
        $v1.azam_encode_write(&mut bytes).unwrap();
        $v2.azam_encode_write(&mut bytes).unwrap();
        $v3.azam_encode_write(&mut bytes).unwrap();
        $v4.azam_encode_write(&mut bytes).unwrap();
        String::from_utf8(bytes).unwrap()
    }};
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr) => {{
        let mut bytes = Vec::<u8>::new();
        $v1.azam_encode_write(&mut bytes).unwrap();
        $v2.azam_encode_write(&mut bytes).unwrap();
        $v3.azam_encode_write(&mut bytes).unwrap();
        $v4.azam_encode_write(&mut bytes).unwrap();
        $v5.azam_encode_write(&mut bytes).unwrap();
        String::from_utf8(bytes).unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use crate::encode::*;

    #[test]
    fn test_azam_encode_example() {
        assert_eq!("xytxvyyf", 0xdeadbeefu32.azam_encode());
        assert_eq!("h5", 0x15u8.azam_encode());
        assert_eq!("wgg1", 0xc001u16.azam_encode());
    }

    #[test]
    fn test_azam_encode_uints() {
        assert_eq!("0", 0x00u8.azam_encode());
        assert_eq!("1", 0x01u8.azam_encode());
        assert_eq!("f", 0x0fu8.azam_encode());
        assert_eq!("h0", 0x10u8.azam_encode());
        assert_eq!("zf", 0xffu8.azam_encode());

        assert_eq!("0", 0x0000u32.azam_encode());
        assert_eq!("1", 0x0001u32.azam_encode());
        assert_eq!("f", 0x000fu32.azam_encode());
        assert_eq!("h0", 0x0010u32.azam_encode());
        assert_eq!("zf", 0x00ffu32.azam_encode());
        assert_eq!("hgg0", 0x1000u32.azam_encode());
    }

    #[test]
    fn test_azam_encode_macro() {
        assert_eq!("zf", azam_encode!(0xffu8));
        assert_eq!("zzzf", azam_encode!(0xffffu16));
        assert_eq!(
            "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf",
            azam_encode!(0xffffffffffffffffffffffffffffffffu128)
        );
        assert_eq!(
            "zfzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf",
            azam_encode!(0xffu8, 0xffffu16, 0xffffffffffffffffffffffffffffffffu128)
        );
    }
}
