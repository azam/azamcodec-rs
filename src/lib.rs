/*!
An encoder and decoder implementation in Rust for [Azam Codec](https://github.com/azam/azamcodec), a lexicographically sortable multi-section base16 encoding of byte array. Zero external dependencies.

## License

MIT Licence
Copyright (c) 2022 Azamshul Azizy

## Usage

Import the crate and start using it.

### Decoding

```rust
use azamcodec::{azam_decode, decode::AzamDecode};

// Decode first section of Azam-encoded string as u32, using trait [`AzamDecode`] on uint type.
// "xytxvyyf" decodes to 0xdeadbeefu32, the rest of string is ignored.
let x = u32::azam_decode("xytxvyyfh5wgg1"); // 0xdeadbeefu32

// Decode multiple sections of Azam-encoded string to a tuple using azam_decode! macro.
// "xytxvyyf" decodes to 0xdeadbeefu32.
// "h5" decodes to 0x15u8.
// "wgg1" decodes to 0xc001u16.
let (x, y, z) = azam_decode!("xytxvyyfh5wgg1", u32, u8, u16).unwrap(); // (0xdeadbeefu32, 0x15u8, c001u16)

// Decode multiple sections of Azam-encoded string into custom struct.
struct Id {
    record_id: u32,
    type_id: u8,
    variant_id: u16,
}
impl Id {
    pub fn from_str(value: &str) -> Self {
        // reader can be anything that implements std::io::Read.
        // e.g. network stream or file
        // This example reads from a byte array, which is backed by a string.
        let reader = &mut value.as_bytes();
        let record_id = u32::azam_decode_read(reader).unwrap();
        let type_id = u8::azam_decode_read(reader).unwrap();
        let variant_id = u16::azam_decode_read(reader).unwrap();
        Self {
            record_id,
            type_id,
            variant_id,
        }
    }
}
```

### Encoding

```rust
use azamcodec::{azam_encode, encode::AzamEncode};

// Encode u32 value as Azam-encoded string as u32, using trait [`AzamEncode`] on uint type.
// 0xdeadbeefu32 encodes to "xytxvyyf".
let x = 0xdeadbeefu32.azam_encode(); // "xytxvyyf"

// Encode multiple values as Azam-encoded string, using [`azam_decode!`] macro.
// 0xdeadbeefu32 encodes to "xytxvyyf".
// 0x15u8 encodes to "h5".
// 0xc001u16 encodes to "wgg1".
let x = azam_encode!(0xdeadbeefu32, 0x15u8, 0xc001u16); // "xytxvyyfh5wgg1"

// Encode multiple values as Azam-encoded string from custom struct.
struct Id {
    record_id: u32,
    type_id: u8,
    variant_id: u16,
}
impl Id {
    pub fn to_str(&self) -> String {
        // writer can be anything that implements std::io::Write.
        // This example writes to a byte array, then converted to string.
        // e.g. network stream or file
        let mut writer = Vec::<u8>::new();
        self.record_id.azam_encode_write(&mut writer).unwrap();
        self.type_id.azam_encode_write(&mut writer).unwrap();
        self.variant_id.azam_encode_write(&mut writer).unwrap();
        String::from_utf8(writer).unwrap()
    }
}
```

## Development

Standard Rust development applies. Benchmark is also included, executable via `cargo bench`.
 */

#![deny(missing_docs)]

/// Decoding functions for Azam codec
pub mod decode;

/// Encoding functions for Azam codec
pub mod encode;
