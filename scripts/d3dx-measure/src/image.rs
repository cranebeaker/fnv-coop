//! `image-info DIR`: `D3DXGetImageInfoFromFileInMemory` on every file in DIR (copies of the
//! game's textures, kept under build43-scratch/measure) and on variants made from them (the
//! header alone, a short file, the mip count or its flag changed, the magic broken). Each line:
//! `ImageInfo<TAB>file<TAB>variant<TAB>size<TAB>HRESULT<TAB>D3DXIMAGE_INFO (28 bytes, hex,
//! 0x55 where the DLL wrote nothing)`.
use std::ffi::c_void;
use std::io::Write;

type GetImageInfo = unsafe extern "system" fn(*const u8, u32, *mut u8) -> u32;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// The variants of one file: (name, bytes).
fn variants(data: &[u8]) -> Vec<(&'static str, Vec<u8>)> {
    let mut out = vec![("as-is", data.to_vec())];
    if data.len() >= 128 {
        out.push(("header-only", data[..128].to_vec()));
        out.push(("short-127", data[..127].to_vec()));
        out.push(("minus-1", data[..data.len() - 1].to_vec()));
        let mut v = data.to_vec();
        // dwFlags (+8) without DDSD_MIPMAPCOUNT (0x20000).
        let flags = u32::from_le_bytes(v[8..12].try_into().unwrap()) & !0x2_0000;
        v[8..12].copy_from_slice(&flags.to_le_bytes());
        out.push(("no-mipcount-flag", v));
        let mut v = data.to_vec();
        v[28..32].copy_from_slice(&0u32.to_le_bytes());
        out.push(("mipcount-0", v));
        let mut v = data.to_vec();
        v[28..32].copy_from_slice(&99u32.to_le_bytes());
        out.push(("mipcount-99", v));
        let mut v = data.to_vec();
        v[0] = b'X';
        out.push(("bad-magic", v));
    }
    out
}

pub unsafe fn image_info(o: &mut impl Write, module: *mut c_void, dir: &str) {
    let f = crate::GetProcAddress(module, b"D3DXGetImageInfoFromFileInMemory\0".as_ptr());
    assert!(!f.is_null());
    let f: GetImageInfo = std::mem::transmute(f);
    let mut names: Vec<_> = std::fs::read_dir(dir).unwrap().filter_map(|d| d.ok()).map(|d| d.path()).collect();
    names.sort();
    for (n, path) in names.iter().enumerate() {
        let data = std::fs::read(path).unwrap();
        let name = path.file_name().unwrap().to_string_lossy();
        // Every file as it is; the variants for every 10th.
        let all = variants(&data);
        let take = if n % 10 == 0 { all.len() } else { 1 };
        for (variant, bytes) in all.into_iter().take(take) {
            let mut info = [0x55u8; 28];
            let hr = f(bytes.as_ptr(), bytes.len() as u32, info.as_mut_ptr());
            writeln!(o, "ImageInfo\t{name}\t{variant}\t{:x}\t{hr:08x}\t{}", bytes.len(), hex(&info)).unwrap();
        }
    }
}
