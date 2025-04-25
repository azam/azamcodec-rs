use std::io::{Error, ErrorKind, Read, Result, Write};

const INITIAL_CAPACITY: usize = if cfg!(target_pointer_width = "64") {
    8
} else if cfg!(target_pointer_width = "32") {
    4
} else {
    2
};

/// Returns the nybble value [0..15] for given byte symbol.
/// If nybble is a high nybble, 16 will be added to the value.
/// If give byte symbol is not a valid symbol, None is returned.
const fn nybble_value(value: u8) -> Option<u8> {
    match value {
        // Lower nybble
        b'0' | b'o' | b'O' => Some(0x00u8),
        b'1' | b'i' | b'l' | b'I' | b'L' => Some(0x01u8),
        b'2' => Some(0x02u8),
        b'3' => Some(0x03u8),
        b'4' => Some(0x04u8),
        b'5' => Some(0x05u8),
        b'6' => Some(0x06u8),
        b'7' => Some(0x07u8),
        b'8' => Some(0x08u8),
        b'9' => Some(0x09u8),
        b'a' | b'A' => Some(0x0au8),
        b'b' | b'B' => Some(0x0bu8),
        b'c' | b'C' => Some(0x0cu8),
        b'd' | b'D' => Some(0x0du8),
        b'e' | b'E' => Some(0x0eu8),
        b'f' | b'F' => Some(0x0fu8),
        // Higher nybble
        b'g' | b'G' => Some(0x10u8),
        b'h' | b'H' => Some(0x11u8),
        b'j' | b'J' => Some(0x12u8),
        b'k' | b'K' => Some(0x13u8),
        b'm' | b'M' => Some(0x14u8),
        b'n' | b'N' => Some(0x15u8),
        b'p' | b'P' => Some(0x16u8),
        b'q' | b'Q' => Some(0x17u8),
        b'r' | b'R' => Some(0x18u8),
        b's' | b'S' => Some(0x19u8),
        b't' | b'T' => Some(0x1au8),
        b'v' | b'V' => Some(0x1bu8),
        b'w' | b'W' => Some(0x1cu8),
        b'x' | b'X' => Some(0x1du8),
        b'y' | b'Y' => Some(0x1eu8),
        b'z' | b'Z' => Some(0x1fu8),
        _ => None,
    }
}

/// Given a source of a [`Read`] instance of Azam coded encoded stream, limit the read up to `limit` bytes, decode and write to [`Write`] instance.
///
/// # Arguments
///
/// * `reader` - Azam codec encoded stream
/// * `writer` - Stream to write decoded bytes
/// * `limit` - Maximum number of bytes to read
///
/// # Examples
/// ```rust
/// use azamcodec::decode::azam_decode_read_until;
///
/// let mut src = &mut "xytxvyyfh5wgg1".as_bytes();
/// let mut dst = Vec::<u8>::new();
/// azam_decode_read_until(&mut src, &mut dst, 8).unwrap();
/// let decoded = u32::from_be_bytes(dst.as_slice()[..4].try_into().unwrap()); // 0xdeadbeefu32
/// ```
pub fn azam_decode_read_until<R: Read + ?Sized, W: Write>(
    reader: &mut R,
    writer: &mut W,
    limit: u64,
) -> Result<usize> {
    let limited_reader = &mut reader.take(limit);
    azam_decode_read(limited_reader, writer)
}

/// Given a source of a [`Read`] instance of Azam coded encoded stream, read all bytes up to EOF or end of first section, decode and write to [`Write`] instance.
/// This method always write at least one byte, or throw an error.
///
/// # Arguments

/// * `reader` - Azam codec encoded stream
/// * `writer` - Stream to write decoded bytes
///
/// # Examples
/// ```rust
/// use azamcodec::decode::azam_decode_read;
///
/// let mut src = &mut "xytxvyyfh5wgg1".as_bytes();
/// let mut dst = Vec::<u8>::new();
/// azam_decode_read(&mut src, &mut dst).unwrap();
/// let decoded = u32::from_be_bytes(dst.as_slice()[..4].try_into().unwrap()); // 0xdeadbeefu32
/// ```
pub fn azam_decode_read<R: Read + ?Sized, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> Result<usize> {
    let mut bytes = Vec::<u8>::with_capacity(INITIAL_CAPACITY);
    let mut byte = [0u8; 1];
    let mut prev_nybble = 0u8;
    let mut is_odd = false;
    let mut lead_nybble_checked = false;
    let mut count = 0usize;
    loop {
        // Read one byte, or exit loop if read fails (EOF etc)
        reader.read_exact(&mut byte)?;
        count += 1;
        // Use ok_or_else instead ok_or, because ok_or eagerly evaluates the value, which performance is significant at this level
        let value = nybble_value(byte[0]).ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
        // Flip oddness
        is_odd = !is_odd;
        if !lead_nybble_checked {
            // If the first byte starts with a high nibble 0 (g or G), return error as invalid data
            if value == 0x10u8 {
                return Err(ErrorKind::InvalidData.into());
            }
            lead_nybble_checked = true;
        }
        // Take previous nybble, shift left 4 and bit or current nybble
        if !is_odd {
            bytes.push((prev_nybble << 4) | (value & 0x0fu8));
        }
        // Remember current nybble for next iteration
        prev_nybble = value & 0x0fu8;
        // If current nybble is a low nybble, this is the last one, so exit loop
        if value >> 4 == 00u8 {
            break;
        }
    }
    // If nybble count is odd, then there is one unwritten nybble.
    // Add the unwritten nybble, and shift whole byte array 4 bits to the right.
    if is_odd && count > 0 {
        bytes.push(prev_nybble << 4);
        let mut high_nybble = 0u8;
        for i in 0..bytes.len() {
            let byte = bytes[i];
            bytes[i] = byte >> 4 | high_nybble;
            high_nybble = byte << 4;
        }
    }
    writer.write_all(&bytes)?;
    Ok(count)
}

/// Given a source of a [`&str`] instance of Azam coded encoded stream, read all bytes up to EOF or end of first section, decode and return as byte array.
/// This method always write at least one byte, or throw an error.
///
/// # Arguments
///
/// * `value` - Azam codec encoded string
///
/// # Examples
/// ```rust
/// use azamcodec::decode::azam_decode_bytes;
///
/// let mut src = &mut "xytxvyyfh5wgg1".as_bytes();
/// let mut dst = Vec::<u8>::new();
/// let decoded = azam_decode_bytes("xytxvyyfh5wgg1").unwrap(); // vec![0xde, 0xad, 0xbe, 0xef]
/// ```
pub fn azam_decode_bytes(value: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::<u8>::new();
    azam_decode_read(&mut value.as_bytes(), &mut bytes)?;
    Ok(bytes)
}

/// Given a source of a [`&str`] instance of Azam coded encoded stream, read all bytes up to `limit` or end of first section, decode and return as byte array.
/// This method always write at least one byte, or throw an error.
///
/// # Arguments
///
/// * `value` - Azam codec encoded string
/// * `limit` - Maximum bytes to read
///
/// # Examples
/// ```rust
/// use azamcodec::decode::azam_decode_bytes_until;
///
/// let mut src = &mut "xytxvyyfh5wgg1".as_bytes();
/// let mut dst = Vec::<u8>::new();
/// let decoded = azam_decode_bytes_until("xytxvyyfh5wgg1", 8).unwrap(); // vec![0xde, 0xad, 0xbe, 0xef]
/// ```
pub fn azam_decode_bytes_until(value: &str, limit: u64) -> Result<Vec<u8>> {
    let mut bytes = Vec::<u8>::new();
    azam_decode_read_until(&mut value.as_bytes(), &mut bytes, limit)?;
    Ok(bytes)
}

/// Given a source of a [`&str`] instance of Azam coded encoded stream, read all bytes up to EOF or end of first section, decode all sections, and return as array of byte array.
/// This method always write at least one byte, or throw an error.
///
/// # Arguments
///
/// * `value` - Azam codec encoded string
/// * `limit` - Maximum bytes to read
///
/// # Examples
/// ```rust
/// use azamcodec::decode::azam_decode_bytes_vec;
///
/// let decoded = azam_decode_bytes_vec("xytxvyyfh5wgg1").unwrap();
/// // decoded[0] = vec![0xde, 0xad, 0xbe, 0xef]
/// // decoded[1] = vec![0x15]
/// // decoded[2] = vec![0xc0, 0x01]
/// ```
pub fn azam_decode_bytes_vec(value: &str) -> Result<Vec<Vec<u8>>> {
    let mut all_bytes = Vec::<Vec<u8>>::new();
    let mut index = 0usize;
    while index < value.len() {
        let mut bytes = Vec::<u8>::new();
        let mut reader = &value.as_bytes()[index..];
        let read_size = azam_decode_read(&mut reader, &mut bytes)?;
        all_bytes.push(bytes);
        index += read_size;
    }
    Ok(all_bytes)
}

/// Trait to extend types to support decoding of Azam encoded strings.
pub trait AzamDecode: Sized {
    /// Given a source of a [`Read`] instance of Azam codec encoded stream, read bytes as needed, decode and return the correct representation of own object.
    ///
    /// # Arguments
    ///
    /// * `reader` - Azam codec encoded stream
    fn azam_decode_read<R: Read + Sized>(reader: &mut R) -> Result<Self>;

    /// Given a source of a [`&str`] instance of Azam codec encoded stream, read bytes as needed, decode and return the correct representation of own object.
    ///
    /// # Arguments
    ///
    /// * `value` - Azam codec encoded string
    fn azam_decode(value: &str) -> Result<Self> {
        Self::azam_decode_read(&mut value.as_bytes())
    }
}

impl AzamDecode for u8 {
    fn azam_decode_read<R: Read + Sized>(reader: &mut R) -> Result<Self> {
        let mut bytes = Vec::<u8>::new();
        // Limit to twice the byte size of type
        azam_decode_read_until(reader, &mut bytes, 2)?;
        // This is safe because decode will always write at least one byte or error out.
        Ok(u8::from_be_bytes(bytes[..1].try_into().unwrap()))
    }
}

macro_rules! azam_decode_uint_impl {
    ($t:ty, $s:expr) => {
        impl AzamDecode for $t {
            fn azam_decode_read<R: Read + Sized>(reader: &mut R) -> Result<Self> {
                // Saving one call instead of calling below
                // let size = mem::size_of::<Self>();
                const SIZE: usize = $s;
                let mut bytes = Vec::<u8>::new();
                // Limit to twice the byte size of type
                $crate::decode::azam_decode_read_until(reader, &mut bytes, (SIZE * 2) as u64)?;
                // Extend byte array to byte size of type and prepend with zeroes
                let original_len = bytes.len();
                if original_len < SIZE {
                    bytes.resize(SIZE, 0);
                    bytes.rotate_right(SIZE - original_len);
                }
                Ok(Self::from_be_bytes(bytes[..SIZE].try_into().unwrap()))
            }
        }
    };
}

azam_decode_uint_impl!(u16, 2);
azam_decode_uint_impl!(u32, 4);
azam_decode_uint_impl!(u64, 8);
azam_decode_uint_impl!(u128, 16);

/// Macro to decode Azam encoded string to tuples of any types that implements the [`AzamDecode`] trait
///
/// # Examples
///
/// ```rust
/// use azamcodec::azam_decode;
/// // Decode multiple sections of Azam-encoded string to a tuple using azam_decode! macro.
/// // "xytxvyyf" decodes to 0xdeadbeefu32.
/// // "h5" decodes to 0x15u8.
/// // "wgg1" decodes to 0xc001u16.
/// let (x, y, z) = azam_decode!("xytxvyyfh5wgg1", u32, u8, u16).unwrap(); // (0xdeadbeefu32, 0x15u8, c001u16)
/// ```
#[macro_export]
macro_rules! azam_decode {
    () => {Result::<()>::Ok(())};
    ($r:expr) => {Result::<()>::Ok(())};
    ($r:expr $(,$t:ty)*) => {{
        use $crate::decode::AzamDecode;
        let reader = &mut $r.as_bytes();
        // Using loop hack to not to use break-labels.
        // This might help when using strict clippy rules.
        // https://github.com/rust-lang/rfcs/pull/2046
        loop {
            break Ok((
                $(
                    match <$t>::azam_decode_read(reader) {
                        Ok(v) => v,
                        Err(e) => break Err(e),
                    }
                ),*
            ));
        }
    }};
}

/// Macro to decode Azam codec encoded stream to tuples of any types that implements the [`AzamDecode`] trait.
///
/// # Examples
///
/// ```rust
/// use azamcodec::{azam_decode_read, decode::AzamDecode};
/// // Decode multiple sections of Azam-encoded string to a tuple using azam_decode! macro.
/// // "xytxvyyf" decodes to 0xdeadbeefu32.
/// // "h5" decodes to 0x15u8.
/// // "wgg1" decodes to 0xc001u16.
/// let mut reader = &mut "xytxvyyfh5wgg1".as_bytes();
/// let (x, y, z) = azam_decode_read!(&mut reader, u32, u8, u16).unwrap(); // (0xdeadbeefu32, 0x15u8, c001u16)
/// ```
#[macro_export]
macro_rules! azam_decode_read {
    () => {Result::<()>::Ok(())};
    ($r:expr) => {Result::<()>::Ok(())};
    ($r:expr $(,$t:ty)*) => {{
        use $crate::decode::AzamDecode;
        let reader = $r;
        // Using loop hack to not to use break-labels.
        // This might help when using strict clippy rules.
        // https://github.com/rust-lang/rfcs/pull/2046
        loop {
            break Ok((
                $(
                    match <$t>::azam_decode_read(reader) {
                        Ok(v) => v,
                        Err(e) => break Err(e),
                    }
                ),*
            ))
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::decode::*;

    #[test]
    fn test_azam_decode_macro() {
        assert_eq!((), azam_decode!("123").unwrap());
        assert_eq!((0x01u8), azam_decode!("123", u8).unwrap());
        assert_eq!((0x01u8, 0x02u16), azam_decode!("123", u8, u16).unwrap());
        assert_eq!(
            (0x01u8, 0x02u16, 0x03u32),
            azam_decode!("123", u8, u16, u32).unwrap()
        );
    }

    #[test]
    fn test_azam_decode_macro_err() {
        assert_eq!(
            ErrorKind::UnexpectedEof,
            azam_decode!("", u8, u16).unwrap_err().kind()
        );
        assert_eq!(
            ErrorKind::UnexpectedEof,
            azam_decode!("12", u8, u16, u32).unwrap_err().kind()
        );
        assert_eq!(
            ErrorKind::InvalidData,
            azam_decode!("_2", u8, u16).unwrap_err().kind()
        );
    }

    #[test]
    fn test_azam_decode_read_macro() {
        assert_eq!((), azam_decode_read!("123").unwrap());
        assert_eq!(
            (0x01u8),
            azam_decode_read!(&mut "123".as_bytes(), u8).unwrap()
        );
        assert_eq!(
            (0x01u8, 0x02u16),
            azam_decode_read!(&mut "123".as_bytes(), u8, u16).unwrap()
        );
        assert_eq!(
            (0x01u8, 0x02u16, 0x03u32),
            azam_decode_read!(&mut "123".as_bytes(), u8, u16, u32).unwrap()
        );
    }

    #[test]
    fn test_azam_decode_read_macro_err() {
        assert_eq!(
            ErrorKind::UnexpectedEof,
            azam_decode_read!(&mut "".as_bytes(), u8, u16)
                .unwrap_err()
                .kind()
        );
        assert_eq!(
            ErrorKind::UnexpectedEof,
            azam_decode_read!(&mut "12".as_bytes(), u8, u16, u32)
                .unwrap_err()
                .kind()
        );
        assert_eq!(
            ErrorKind::InvalidData,
            azam_decode_read!(&mut "_2".as_bytes(), u8, u16)
                .unwrap_err()
                .kind()
        );
    }

    fn assert_decode_uints_err(
        kind: ErrorKind,
        u8_str: &str,
        u16_str: &str,
        u32_str: &str,
        u64_str: &str,
        u128_str: &str,
    ) {
        assert_eq!(kind, u8::azam_decode(u8_str).unwrap_err().kind());
        assert_eq!(kind, u16::azam_decode(u16_str).unwrap_err().kind());
        assert_eq!(kind, u32::azam_decode(u32_str).unwrap_err().kind());
        assert_eq!(kind, u64::azam_decode(u64_str).unwrap_err().kind());
        assert_eq!(kind, u128::azam_decode(u128_str).unwrap_err().kind());
    }

    #[test]
    fn test_decode_uints_empty() {
        assert_decode_uints_err(ErrorKind::UnexpectedEof, "", "", "", "", "");
    }

    #[test]
    fn test_decode_uints_invalid_symbol() {
        assert_decode_uints_err(ErrorKind::InvalidData, "_", "_", "_", "_", "_");
    }

    #[test]
    fn test_decode_uints_invalid_middle_nybble() {
        assert_decode_uints_err(
            ErrorKind::InvalidData,
            "z_",
            "zz_f",
            "zzzzzz_f",
            "zzzzzzzzzzzzzz_f",
            "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzz_f",
        );
    }

    #[test]
    fn test_decode_uints_invalid_last_nybble() {
        assert_decode_uints_err(
            ErrorKind::UnexpectedEof,
            "hh",
            "hhhh",
            "hhhhhhhh",
            "hhhhhhhhhhhhhhhh",
            "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh",
        );
    }

    #[test]
    fn test_decode_uints_invalid_last_nybble_odd() {
        assert_decode_uints_err(
            ErrorKind::UnexpectedEof,
            "h",
            "hhh",
            "hhhhhhh",
            "hhhhhhhhhhhhhhh",
            "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh",
        );
    }

    macro_rules! assert_azam_decode {
        ($e:expr, $t:ty, $v:expr) => {
            assert_eq!($e, <$t>::azam_decode($v).unwrap());
        };
    }

    #[test]
    fn test_decode_uints() {
        assert_azam_decode!(0x00u8, u8, "0");
        assert_azam_decode!(0x10u8, u8, "h0");
        assert_azam_decode!(0x0fu8, u8, "f");
        assert_azam_decode!(0xffu8, u8, "zf");
        assert_azam_decode!(0xffu8, u8, "zf_");
        assert_azam_decode!(0xffu8, u8, "zfaaaa");

        assert_azam_decode!(0x0000u16, u16, "0");
        assert_azam_decode!(0x0010u16, u16, "h0");
        assert_azam_decode!(0x1000u16, u16, "hgg0");
        assert_azam_decode!(0x000fu16, u16, "f");
        assert_azam_decode!(0x00ffu16, u16, "zf");
        assert_azam_decode!(0x0fffu16, u16, "zzf");
        assert_azam_decode!(0xffffu16, u16, "zzzf");
        assert_azam_decode!(0xffffu16, u16, "zzzf_");
        assert_azam_decode!(0xffffu16, u16, "zzzfaaaa");

        assert_azam_decode!(0x00000000u32, u32, "0");
        assert_azam_decode!(0x00000010u32, u32, "h0");
        assert_azam_decode!(0x10000000u32, u32, "hgggggg0");
        assert_azam_decode!(0x0000000fu32, u32, "f");
        assert_azam_decode!(0x000000ffu32, u32, "zf");
        assert_azam_decode!(0x00000fffu32, u32, "zzf");
        assert_azam_decode!(0x0000ffffu32, u32, "zzzf");
        assert_azam_decode!(0x000fffffu32, u32, "zzzzf");
        assert_azam_decode!(0x00ffffffu32, u32, "zzzzzf");
        assert_azam_decode!(0x0fffffffu32, u32, "zzzzzzf");
        assert_azam_decode!(0xffffffffu32, u32, "zzzzzzzf");
        assert_azam_decode!(0xffffffffu32, u32, "zzzzzzzf_");
        assert_azam_decode!(0xffffffffu32, u32, "zzzzzzzfaaaa");

        assert_azam_decode!(0x0000000000000000u64, u64, "0");
        assert_azam_decode!(0x0000000000000010u64, u64, "h0");
        assert_azam_decode!(0x1000000000000000u64, u64, "hgggggggggggggg0");
        assert_azam_decode!(0x000000000000000fu64, u64, "f");
        assert_azam_decode!(0x00000000000000ffu64, u64, "zf");
        assert_azam_decode!(0x0000000000000fffu64, u64, "zzf");
        assert_azam_decode!(0x000000000000ffffu64, u64, "zzzf");
        assert_azam_decode!(0x00000000000fffffu64, u64, "zzzzf");
        assert_azam_decode!(0x0000000000ffffffu64, u64, "zzzzzf");
        assert_azam_decode!(0x000000000fffffffu64, u64, "zzzzzzf");
        assert_azam_decode!(0x00000000ffffffffu64, u64, "zzzzzzzf");
        assert_azam_decode!(0x0000000fffffffffu64, u64, "zzzzzzzzf");
        assert_azam_decode!(0x000000ffffffffffu64, u64, "zzzzzzzzzf");
        assert_azam_decode!(0x00000fffffffffffu64, u64, "zzzzzzzzzzf");
        assert_azam_decode!(0x0000ffffffffffffu64, u64, "zzzzzzzzzzzf");
        assert_azam_decode!(0x000fffffffffffffu64, u64, "zzzzzzzzzzzzf");
        assert_azam_decode!(0x00ffffffffffffffu64, u64, "zzzzzzzzzzzzzf");
        assert_azam_decode!(0x0fffffffffffffffu64, u64, "zzzzzzzzzzzzzzf");
        assert_azam_decode!(0xffffffffffffffffu64, u64, "zzzzzzzzzzzzzzzf");
        assert_azam_decode!(0xffffffffffffffffu64, u64, "zzzzzzzzzzzzzzzf_");
        assert_azam_decode!(0xffffffffffffffffu64, u64, "zzzzzzzzzzzzzzzfaaaa");
    }
}
