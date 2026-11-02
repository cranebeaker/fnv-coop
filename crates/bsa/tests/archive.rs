//! BSA tests against archives assembled byte by byte.

use std::fs;

use bsa::hash::{hash_file_name, hash_folder_name};
use bsa::{archive_flags as af, Archive, Error};

struct TestFile<'a> {
    folder: &'a str,
    name: &'a str,
    data: Vec<u8>,
    /// Flip the archive's default compression for this file.
    toggle: bool,
}

fn file<'a>(folder: &'a str, name: &'a str, data: &[u8]) -> TestFile<'a> {
    TestFile {
        folder,
        name,
        data: data.to_vec(),
        toggle: false,
    }
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &x in data {
        a = (a + u32::from(x)) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

/// A valid zlib stream made of stored (uncompressed) blocks.
fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut v = vec![0x78, 0x01];
    let chunks: Vec<&[u8]> = if data.is_empty() {
        vec![&[]]
    } else {
        data.chunks(65_535).collect()
    };
    for (i, chunk) in chunks.iter().enumerate() {
        v.push(u8::from(i == chunks.len() - 1));
        let len = chunk.len() as u16;
        v.extend(len.to_le_bytes());
        v.extend((!len).to_le_bytes());
        v.extend(*chunk);
    }
    v.extend(adler32(data).to_be_bytes());
    v
}

/// Builds an archive the way the format lays it out: header, folder records,
/// per-folder name + file records, the file name table, then file data.
fn build(version: u32, flags: u32, files: &[TestFile<'static>]) -> Vec<u8> {
    let mut folders: Vec<&'static str> = Vec::new();
    for f in files {
        if !folders.contains(&f.folder) {
            folders.push(f.folder);
        }
    }
    let in_folder = |folder: &'static str| files.iter().filter(move |f| f.folder == folder);
    let total_folder_names: usize = folders.iter().map(|f| f.len() + 1).sum();
    let total_file_names: usize = files.iter().map(|f| f.name.len() + 1).sum();
    let dir_names = flags & af::DIRECTORY_NAMES != 0;
    let file_names = flags & af::FILE_NAMES != 0;

    let mut directory_len = 36 + folders.len() * 16 + files.len() * 16;
    if dir_names {
        directory_len += folders.len() + total_folder_names;
    }
    if file_names {
        directory_len += total_file_names;
    }

    // File data blobs, in folder order.
    let mut blobs: Vec<(usize, Vec<u8>)> = Vec::new(); // (file index, blob)
    for folder in &folders {
        for f in in_folder(folder) {
            let index = files.iter().position(|x| std::ptr::eq(x, f)).unwrap();
            let mut blob = Vec::new();
            if version == 104 && flags & af::EMBEDDED_FILE_NAMES != 0 {
                let full = format!("{}\\{}", f.folder, f.name);
                blob.push(full.len() as u8);
                blob.extend(full.as_bytes());
            }
            let compressed = (flags & af::COMPRESSED != 0) ^ f.toggle;
            if compressed {
                blob.extend((f.data.len() as u32).to_le_bytes());
                blob.extend(zlib_stored(&f.data));
            } else {
                blob.extend(&f.data);
            }
            blobs.push((index, blob));
        }
    }

    let mut out = Vec::new();
    out.extend(b"BSA\0");
    out.extend(version.to_le_bytes());
    out.extend(36u32.to_le_bytes());
    out.extend(flags.to_le_bytes());
    out.extend((folders.len() as u32).to_le_bytes());
    out.extend((files.len() as u32).to_le_bytes());
    out.extend((total_folder_names as u32).to_le_bytes());
    out.extend((total_file_names as u32).to_le_bytes());
    out.extend(0x3u32.to_le_bytes()); // meshes + textures

    for folder in &folders {
        out.extend(hash_folder_name(folder).to_le_bytes());
        out.extend((in_folder(folder).count() as u32).to_le_bytes());
        out.extend(0u32.to_le_bytes()); // offset: unused by the reader
    }
    let mut data_offset = directory_len;
    let mut blob_iter = blobs.iter();
    for folder in &folders {
        if dir_names {
            out.push((folder.len() + 1) as u8);
            out.extend(folder.as_bytes());
            out.push(0);
        }
        for f in in_folder(folder) {
            let (_, blob) = blob_iter.next().unwrap();
            out.extend(hash_file_name(f.name).to_le_bytes());
            let mut size = blob.len() as u32;
            if f.toggle {
                size |= 0x4000_0000;
            }
            out.extend(size.to_le_bytes());
            out.extend((data_offset as u32).to_le_bytes());
            data_offset += blob.len();
        }
    }
    if file_names {
        for folder in &folders {
            for f in in_folder(folder) {
                out.extend(f.name.as_bytes());
                out.push(0);
            }
        }
    }
    assert_eq!(out.len(), directory_len);
    for (_, blob) in &blobs {
        out.extend(blob);
    }
    out
}

fn sample_files() -> Vec<TestFile<'static>> {
    vec![
        file("meshes\\weapons", "gun.nif", b"NIF mesh bytes"),
        file("meshes\\weapons", "knife.nif", &[7u8; 1000]),
        file("textures\\weapons", "gun.dds", b"DDS texture bytes"),
    ]
}

const NAMES: u32 = af::DIRECTORY_NAMES | af::FILE_NAMES;

#[test]
fn reads_an_uncompressed_archive() {
    let archive = Archive::from_bytes(build(104, NAMES, &sample_files())).unwrap();
    assert_eq!(archive.files().len(), 3);
    assert_eq!(archive.folders().len(), 2);
    assert_eq!(archive.folders()[0].name, "meshes\\weapons");
    assert_eq!(archive.folders()[0].files, 0..2);

    let gun = archive.find("meshes\\weapons\\gun.nif").unwrap();
    assert!(!gun.compressed);
    assert_eq!(archive.read(gun).unwrap(), b"NIF mesh bytes");
    let knife = archive.find("meshes\\weapons\\knife.nif").unwrap();
    assert_eq!(archive.read(knife).unwrap(), vec![7u8; 1000]);
}

#[test]
fn lookups_ignore_case_slashes_and_a_data_prefix() {
    let archive = Archive::from_bytes(build(104, NAMES, &sample_files())).unwrap();
    for path in [
        "Meshes/Weapons/Gun.NIF",
        "data\\meshes\\weapons\\gun.nif",
        "Data/meshes/weapons/gun.nif",
        "\\meshes\\weapons\\gun.nif",
    ] {
        assert!(archive.find(path).is_some(), "{path}");
    }
    assert!(archive.find("meshes\\weapons\\rifle.nif").is_none());
}

#[test]
fn decompresses_by_default_with_per_file_toggle() {
    let mut files = sample_files();
    files[1].toggle = true; // knife.nif stored raw in a compressed archive
    let archive = Archive::from_bytes(build(104, NAMES | af::COMPRESSED, &files)).unwrap();

    let gun = archive.find("meshes\\weapons\\gun.nif").unwrap();
    assert!(gun.compressed);
    assert_eq!(archive.read(gun).unwrap(), b"NIF mesh bytes");
    let knife = archive.find("meshes\\weapons\\knife.nif").unwrap();
    assert!(!knife.compressed);
    assert_eq!(archive.read(knife).unwrap(), vec![7u8; 1000]);
}

#[test]
fn toggle_compresses_single_files_in_an_uncompressed_archive() {
    let mut files = sample_files();
    files[2].toggle = true;
    let archive = Archive::from_bytes(build(104, NAMES, &files)).unwrap();
    let dds = archive.find("textures\\weapons\\gun.dds").unwrap();
    assert!(dds.compressed);
    assert_eq!(archive.read(dds).unwrap(), b"DDS texture bytes");
}

#[test]
fn strips_embedded_names_in_version_104() {
    let flags = NAMES | af::COMPRESSED | af::EMBEDDED_FILE_NAMES;
    let archive = Archive::from_bytes(build(104, flags, &sample_files())).unwrap();
    for f in sample_files() {
        let entry = archive.find(&format!("{}\\{}", f.folder, f.name)).unwrap();
        assert_eq!(archive.read(entry).unwrap(), f.data);
    }
}

#[test]
fn reads_version_103() {
    let archive = Archive::from_bytes(build(103, NAMES | af::COMPRESSED, &sample_files())).unwrap();
    assert_eq!(archive.header().version, 103);
    let gun = archive.find("meshes/weapons/gun.nif").unwrap();
    assert_eq!(archive.read(gun).unwrap(), b"NIF mesh bytes");
}

#[test]
fn name_hashes_check_out() {
    let archive = Archive::from_bytes(build(104, NAMES, &sample_files())).unwrap();
    let check = archive.check_name_hashes();
    assert_eq!(check.files_matched, 3);
    assert_eq!(check.files_total, 3);
    assert_eq!(check.folders_matched, 2);
}

#[test]
fn works_without_name_tables() {
    let archive = Archive::from_bytes(build(104, 0, &sample_files())).unwrap();
    assert_eq!(archive.files().len(), 3);
    let first = &archive.files()[0];
    assert_eq!(first.name, format!("{:016x}", hash_file_name("gun.nif")));
    assert_eq!(archive.read(first).unwrap(), b"NIF mesh bytes");
}

#[test]
fn opens_from_disk() {
    let path = std::env::temp_dir().join(format!("nv-rs-bsa-{}.bsa", std::process::id()));
    fs::write(&path, build(104, NAMES | af::COMPRESSED, &sample_files())).unwrap();
    let archive = Archive::open(&path).unwrap();
    let dds = archive.find("textures/weapons/gun.dds").unwrap();
    assert_eq!(archive.read(dds).unwrap(), b"DDS texture bytes");
    drop(archive);
    fs::remove_file(path).unwrap();
}

#[test]
fn rejects_other_files_and_versions() {
    assert!(matches!(
        Archive::from_bytes(b"TES4 plugin".to_vec()),
        Err(Error::NotAnArchive { .. })
    ));
    let ba2 = Archive::from_bytes(b"BTDX\x01\0\0\0GNRL".to_vec())
        .err()
        .unwrap();
    assert!(ba2.to_string().contains("BA2"), "{ba2}");

    let mut sse = build(104, NAMES, &sample_files());
    sse[4] = 105;
    let err = Archive::from_bytes(sse).err().unwrap();
    assert!(err.to_string().contains("Skyrim"), "{err}");

    let mut xbox = build(104, NAMES, &sample_files());
    xbox[12] |= af::XBOX as u8;
    assert!(matches!(
        Archive::from_bytes(xbox),
        Err(Error::Unsupported(_))
    ));
}

#[test]
fn reports_truncated_or_inconsistent_archives() {
    let full = build(104, NAMES, &sample_files());
    for cut in [10, 40, 100, full.len() - 3] {
        assert!(
            Archive::from_bytes(full[..cut].to_vec()).is_err(),
            "cut at {cut}"
        );
    }
    let mut wrong_count = full.clone();
    wrong_count[20] = 4; // header claims 4 files, folders list 3
    assert!(Archive::from_bytes(wrong_count).is_err());
}

#[test]
fn reports_corrupt_compressed_data_with_its_path() {
    let mut bytes = build(104, NAMES | af::COMPRESSED, &sample_files());
    let last = bytes.len() - 1;
    bytes[last] ^= 0xFF; // break the final file's checksum
    let archive = Archive::from_bytes(bytes).unwrap();
    let dds = archive.find("textures\\weapons\\gun.dds").unwrap();
    let err = archive.read(dds).unwrap_err();
    assert!(err.to_string().contains("gun.dds"), "{err}");
}
