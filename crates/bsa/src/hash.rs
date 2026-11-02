//! The name hash stored with every folder and file in version 103/104
//! archives. The game looks files up by these hashes; this reader uses them
//! only as a consistency check on the directory it parsed.

/// Hash of a file name without its folder, such as `"10mmpistol.nif"`.
pub fn hash_file_name(file_name: &str) -> u64 {
    let name = crate::normalize_path(file_name);
    let bytes = name.as_bytes();
    match name.rfind('.') {
        Some(dot) => hash_parts(&bytes[..dot], &bytes[dot..]),
        None => hash_parts(bytes, b""),
    }
}

/// Hash of a folder path, such as `"meshes\\weapons\\1handpistol"`.
pub fn hash_folder_name(folder: &str) -> u64 {
    hash_parts(crate::normalize_path(folder).as_bytes(), b"")
}

fn hash_parts(stem: &[u8], ext: &[u8]) -> u64 {
    let len = stem.len();
    let mut low: u32 = 0;
    if len > 0 {
        low = u32::from(stem[len - 1])
            | if len > 2 {
                u32::from(stem[len - 2]) << 8
            } else {
                0
            }
            | (len as u32) << 16
            | u32::from(stem[0]) << 24;
    }
    match ext {
        b".kf" => low |= 0x80,
        b".nif" => low |= 0x8000,
        b".dds" => low |= 0x8080,
        b".wav" => low |= 0x8000_0000,
        _ => {}
    }

    let rolling = |bytes: &[u8]| {
        bytes.iter().fold(0u32, |h, &c| {
            h.wrapping_mul(0x1003F).wrapping_add(u32::from(c))
        })
    };
    let middle = if len > 2 {
        rolling(&stem[1..len - 2])
    } else {
        0
    };
    let high = middle.wrapping_add(rolling(ext));
    (u64::from(high) << 32) | u64::from(low)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_case_and_separator_style() {
        assert_eq!(hash_file_name("Gun.NIF"), hash_file_name("gun.nif"));
        assert_eq!(
            hash_folder_name("Meshes/Weapons"),
            hash_folder_name("meshes\\weapons")
        );
    }

    #[test]
    fn low_word_packs_first_last_and_length() {
        // "abc.nif": last 'c', second-to-last 'b', length 3, first 'a', .nif bit.
        let h = hash_file_name("abc.nif");
        let expected_low =
            u32::from(b'c') | u32::from(b'b') << 8 | 3 << 16 | u32::from(b'a') << 24 | 0x8000;
        assert_eq!(h as u32, expected_low);
        assert_ne!(hash_file_name("abc.nif"), hash_file_name("abc.dds"));
        assert_ne!(hash_file_name("abxc.nif"), hash_file_name("abyc.nif"));
    }
}
