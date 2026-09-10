//! Patches class bytecode in Minecraft jar archives.

use crate::name::name;

pub const TITLE_SCREEN_CLASS: &str = "cki.class";
pub const OLD_TITLE_STRING: &[u8] = b"Minecraft 1.13.2";

/// Replaces a UTF-8 constant pool entry in Java class bytes.
///
/// Java constant pool CONSTANT_Utf8 entries have the structure:
/// - tag (1 byte): 0x01
/// - length (2 bytes, big-endian): string length
/// - bytes: UTF-8 character data
pub fn patch_utf8_constant(class_bytes: &[u8], old_str: &[u8], new_str: &[u8]) -> Option<Vec<u8>> {
    let old_len = u16::try_from(old_str.len()).ok()?;
    let new_len = u16::try_from(new_str.len()).ok()?;

    let mut old_entry = Vec::with_capacity(3 + old_str.len());
    old_entry.push(0x01);
    old_entry.extend_from_slice(&old_len.to_be_bytes());
    old_entry.extend_from_slice(old_str);

    let mut new_entry = Vec::with_capacity(3 + new_str.len());
    new_entry.push(0x01);
    new_entry.extend_from_slice(&new_len.to_be_bytes());
    new_entry.extend_from_slice(new_str);

    let pos = class_bytes
        .windows(old_entry.len())
        .position(|w| w == old_entry.as_slice())?;

    let mut result = Vec::with_capacity(class_bytes.len() - old_entry.len() + new_entry.len());
    result.extend_from_slice(&class_bytes[..pos]);
    result.extend_from_slice(&new_entry);
    result.extend_from_slice(&class_bytes[pos + old_entry.len()..]);
    Some(result)
}

/// Returns true if the class with the given name requires bytecode patching.
pub fn should_patch_class(class_name: &str) -> bool {
    class_name == TITLE_SCREEN_CLASS
}

/// Applies bytecode patches to the class file if applicable.
/// Returns the (potentially modified) bytes and a boolean indicating whether a patch was applied.
pub fn patch_class(class_name: &str, class_bytes: &[u8]) -> (Vec<u8>, bool) {
    if class_name == TITLE_SCREEN_CLASS {
        let new_title_str = format!("1.13.2 (modded)/{name}");
        if let Some(patched) = patch_utf8_constant(class_bytes, OLD_TITLE_STRING, new_title_str.as_bytes()) {
            println!("[INFO] Patched title screen string in {}", TITLE_SCREEN_CLASS);
            return (patched, true);
        }
    }
    (class_bytes.to_vec(), false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_utf8_constant_success() {
        let mut data = vec![0xCA, 0xFE, 0xBA, 0xBE];
        // Add CONSTANT_Utf8 entry: tag 0x01, len 0x0005, b"hello"
        data.extend_from_slice(&[0x01, 0x00, 0x05, b'h', b'e', b'l', b'l', b'o']);
        data.extend_from_slice(&[0x00, 0x01]); // trailing bytes

        let patched = patch_utf8_constant(&data, b"hello", b"world!").expect("Should patch");

        let expected_new_entry = &[0x01, 0x00, 0x06, b'w', b'o', b'r', b'l', b'd', b'!'];
        assert_eq!(&patched[4..13], expected_new_entry);
        assert_eq!(&patched[13..], &[0x00, 0x01]);
    }

    #[test]
    fn test_patch_utf8_constant_not_found() {
        let data = vec![0xCA, 0xFE, 0xBA, 0xBE];
        let result = patch_utf8_constant(&data, b"missing", b"replacement");
        assert!(result.is_none());
    }

    #[test]
    fn test_should_patch_class() {
        assert!(should_patch_class("cki.class"));
        assert!(!should_patch_class("other.class"));
    }

    #[test]
    fn test_patch_class_with_mod_name() {
        let mut data = vec![0xCA, 0xFE, 0xBA, 0xBE];
        let old_entry = &[
            0x01, 0x00, 0x10, b'M', b'i', b'n', b'e', b'c', b'r', b'a', b'f', b't', b' ', b'1',
            b'.', b'1', b'3', b'.', b'2',
        ];
        data.extend_from_slice(old_entry);

        let (patched, was_patched) = patch_class("cki.class", &data);
        assert!(was_patched);
        let expected_str = format!("1.13.2 (modded)/{name}");
        let mut expected_entry = vec![0x01, 0x00, expected_str.len() as u8];
        expected_entry.extend_from_slice(expected_str.as_bytes());
        assert_eq!(&patched[4..], expected_entry.as_slice());
    }
}
