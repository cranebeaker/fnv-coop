//! `create-texture DIR`: `D3DXCreateTextureFromFileInMemory` on a windowed HAL device for every
//! file in DIR, as the game calls it (the device, the data, the size, the out pointer). Each
//! line: `CreateTexture<TAB>file<TAB>HRESULT<TAB>level count<TAB>for each level
//! `format,usage,pool,width,height,pitch,bytes` where `bytes` is `file` when the locked level
//! (rows packed) is the file's level byte for byte, else `hash:<fnv1a64>`, `;`-separated`.
//! The file's levels are found from its header (the documented layout).
use std::ffi::c_void;
use std::io::Write;

#[link(name = "d3d9")]
extern "system" {
    fn Direct3DCreate9(sdk: u32) -> *mut *const usize;
}
#[link(name = "user32")]
extern "system" {
    fn CreateWindowExA(
        ex_style: u32,
        class: *const u8,
        title: *const u8,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: *mut c_void,
        menu: *mut c_void,
        instance: *mut c_void,
        param: *mut c_void,
    ) -> *mut c_void;
}

type Create = unsafe extern "system" fn(*mut c_void, *const u8, u32, *mut *mut c_void) -> u32;

fn fnv(b: &[u8]) -> u64 {
    b.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &x| (h ^ u64::from(x)).wrapping_mul(0x100_0000_01b3))
}

unsafe fn vt(object: *mut c_void, slot: usize) -> usize {
    *(*(object as *mut *const usize)).add(slot)
}

/// The file's level bytes (offset, rows, row bytes) by the DDS header, for comparing.
fn file_levels(d: &[u8]) -> Vec<(usize, usize, usize)> {
    let w32 = |at: usize| u32::from_le_bytes(d[at..at + 4].try_into().unwrap()) as usize;
    let (height, width, mips) = (w32(12), w32(16), w32(28).max(1));
    let (flags, fourcc, bits) = (w32(80), w32(84) as u32, w32(88));
    let (block, bytes) = if flags & 4 != 0 {
        (4, if &fourcc.to_le_bytes() == b"DXT1" { 8 } else { 16 })
    } else {
        (1, bits / 8)
    };
    let mut at = 128;
    (0..mips)
        .map(|l| {
            let (w, h) = ((width >> l).max(1), (height >> l).max(1));
            let rows = h.div_ceil(block);
            let row = w.div_ceil(block) * bytes;
            let level = (at, rows, row);
            at += rows * row;
            level
        })
        .collect()
}

pub unsafe fn create_texture(o: &mut impl Write, module: *mut c_void, dir: &str) {
    let f = crate::GetProcAddress(module, b"D3DXCreateTextureFromFileInMemory\0".as_ptr());
    assert!(!f.is_null());
    let create: Create = std::mem::transmute(f);
    // A window and a windowed HAL device with hardware vertex processing.
    let class = b"STATIC\0";
    let window = CreateWindowExA(0, class.as_ptr(), class.as_ptr(), 0x1000_0000, 0, 0, 640, 480,
        std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
    let d3d = Direct3DCreate9(32) as *mut c_void;
    let mut present = [0u32; 14];
    present[0] = 640;
    present[1] = 480;
    present[6] = 1;
    present[7] = window as u32;
    present[8] = 1;
    type CreateDevice =
        unsafe extern "system" fn(*mut c_void, u32, u32, *mut c_void, u32, *mut u32, *mut *mut c_void) -> u32;
    let cd: CreateDevice = std::mem::transmute(vt(d3d, 16));
    let mut device: *mut c_void = std::ptr::null_mut();
    let hr = cd(d3d, 0, 1, window, 0x40, present.as_mut_ptr(), &mut device);
    writeln!(o, "Device\t{hr:08x}").unwrap();
    assert_eq!(hr, 0);
    let mut names: Vec<_> = std::fs::read_dir(dir).unwrap().filter_map(|d| d.ok()).map(|d| d.path()).collect();
    names.sort();
    for path in names {
        let data = std::fs::read(&path).unwrap();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let mut texture: *mut c_void = std::ptr::null_mut();
        let hr = create(device, data.as_ptr(), data.len() as u32, &mut texture);
        if hr != 0 || texture.is_null() {
            writeln!(o, "CreateTexture\t{name}\t{hr:08x}\t0\t").unwrap();
            continue;
        }
        type Count = unsafe extern "system" fn(*mut c_void) -> u32;
        type Desc = unsafe extern "system" fn(*mut c_void, u32, *mut u32) -> u32;
        type Lock = unsafe extern "system" fn(*mut c_void, u32, *mut u32, *const u32, u32) -> u32;
        type Unlock = unsafe extern "system" fn(*mut c_void, u32) -> u32;
        type Release = unsafe extern "system" fn(*mut c_void) -> u32;
        let count = (std::mem::transmute::<usize, Count>(vt(texture, 13)))(texture);
        let levels = file_levels(&data);
        let mut parts = Vec::new();
        for level in 0..count {
            let mut desc = [0u32; 8];
            (std::mem::transmute::<usize, Desc>(vt(texture, 17)))(texture, level, desc.as_mut_ptr());
            let mut locked = [0u32; 2];
            // D3DLOCK_READONLY.
            let lr = (std::mem::transmute::<usize, Lock>(vt(texture, 19)))(texture, level, locked.as_mut_ptr(), std::ptr::null(), 0x10);
            let (pitch, bits) = (locked[0] as usize, locked[1] as usize as *const u8);
            let bytes = if lr != 0 {
                format!("lock:{lr:08x}")
            } else {
                let (fmt, w, h) = (desc[0], desc[6] as usize, desc[7] as usize);
                let dxt = fmt & 0xff == 0x44 && (fmt >> 8) & 0xff == 0x58;
                let (rows, row) = if dxt {
                    let bytes = if fmt == 0x3154_5844 { 8 } else { 16 };
                    (h.div_ceil(4), w.div_ceil(4) * bytes)
                } else {
                    let bpp = match fmt { 20 => 3, 21 | 22 => 4, 23 | 25 | 26 => 2, _ => 4 };
                    (h, w * bpp)
                };
                let mut packed = Vec::with_capacity(rows * row);
                for r in 0..rows {
                    packed.extend_from_slice(std::slice::from_raw_parts(bits.add(r * pitch), row));
                }
                (std::mem::transmute::<usize, Unlock>(vt(texture, 20)))(texture, level);
                match levels.get(level as usize) {
                    Some(&(at, frows, frow)) if frows * frow == packed.len() && data.len() >= at + packed.len() && data[at..at + packed.len()] == packed[..] => "file".to_string(),
                    // Small levels in full, with the file's bytes beside them.
                    Some(&(at, _, _)) if packed.len() <= 64 && data.len() >= at + packed.len() => {
                        let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02x}")).collect::<String>();
                        format!("bytes:{}/file:{}", hex(&packed), hex(&data[at..at + packed.len()]))
                    }
                    _ => format!("hash:{:016x}", fnv(&packed)),
                }
            };
            parts.push(format!("{:x},{:x},{:x},{},{},{},{bytes}", desc[0], desc[2], desc[3], desc[6], desc[7], pitch));
        }
        writeln!(o, "CreateTexture\t{name}\t{hr:08x}\t{count}\t{}", parts.join(";")).unwrap();
        (std::mem::transmute::<usize, Release>(vt(texture, 2)))(texture);
    }
}

/// A window and a windowed HAL device with hardware vertex processing (as `create-texture`).
unsafe fn device() -> *mut c_void {
    let class = b"STATIC\0";
    let window = CreateWindowExA(0, class.as_ptr(), class.as_ptr(), 0x1000_0000, 0, 0, 640, 480,
        std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
    let d3d = Direct3DCreate9(32) as *mut c_void;
    let mut present = [0u32; 14];
    present[0] = 640;
    present[1] = 480;
    present[6] = 1;
    present[7] = window as u32;
    present[8] = 1;
    type CreateDevice =
        unsafe extern "system" fn(*mut c_void, u32, u32, *mut c_void, u32, *mut u32, *mut *mut c_void) -> u32;
    let cd: CreateDevice = std::mem::transmute(vt(d3d, 16));
    let mut device: *mut c_void = std::ptr::null_mut();
    let hr = cd(d3d, 0, 1, window, 0x40, present.as_mut_ptr(), &mut device);
    assert_eq!(hr, 0, "CreateDevice");
    device
}

/// The bytes of one locked level (rows packed), its desc words and the lock's answer.
unsafe fn level_bytes(desc: &[u32; 8], pitch: usize, bits: *const u8) -> Vec<u8> {
    let (fmt, w, h) = (desc[0], desc[6] as usize, desc[7] as usize);
    let dxt = fmt & 0xff == 0x44 && (fmt >> 8) & 0xff == 0x58;
    let (rows, row) = if dxt {
        let bytes = if fmt == 0x3154_5844 { 8 } else { 16 };
        (h.div_ceil(4), w.div_ceil(4) * bytes)
    } else {
        let bpp = match fmt {
            20 => 3,
            21 | 22 => 4,
            23 | 25 | 26 => 2,
            28 | 50 => 1,
            other => panic!("level format {other:#x}: no size known"),
        };
        (h, w * bpp)
    };
    let mut packed = Vec::with_capacity(rows * row);
    for r in 0..rows {
        packed.extend_from_slice(std::slice::from_raw_parts(bits.add(r * pitch), row));
    }
    packed
}

fn read_u32(input: &mut impl std::io::Read) -> Option<u32> {
    let mut b = [0u8; 4];
    input.read_exact(&mut b).ok()?;
    Some(u32::from_le_bytes(b))
}

/// `serve`: the 32-bit side of `scripts/d3dx-texture-capture`. Reads requests from stdin
/// (`kind` u32: 1 `D3DXCreateTextureFromFileInMemory`, 2 `D3DXCreateCubeTextureFromFileInMemory`;
/// the data's length u32; the data) and answers each on stdout: the HRESULT, the level count,
/// the face count (1, or 6 for a cube), then for each face and level its format, width,
/// height, byte count and the level's bytes (rows packed). Ends at the end of stdin.
pub unsafe fn serve(module: *mut c_void) {
    let texture_fn = crate::GetProcAddress(module, b"D3DXCreateTextureFromFileInMemory\0".as_ptr());
    let cube_fn = crate::GetProcAddress(module, b"D3DXCreateCubeTextureFromFileInMemory\0".as_ptr());
    assert!(!texture_fn.is_null() && !cube_fn.is_null());
    let device = device();
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    type Count = unsafe extern "system" fn(*mut c_void) -> u32;
    type Desc = unsafe extern "system" fn(*mut c_void, u32, *mut u32) -> u32;
    type Lock = unsafe extern "system" fn(*mut c_void, u32, *mut u32, *const u32, u32) -> u32;
    type Unlock = unsafe extern "system" fn(*mut c_void, u32) -> u32;
    type CubeLock = unsafe extern "system" fn(*mut c_void, u32, u32, *mut u32, *const u32, u32) -> u32;
    type CubeUnlock = unsafe extern "system" fn(*mut c_void, u32, u32) -> u32;
    type Release = unsafe extern "system" fn(*mut c_void) -> u32;
    while let Some(kind) = read_u32(&mut input) {
        let len = read_u32(&mut input).expect("a length") as usize;
        let mut data = vec![0u8; len];
        std::io::Read::read_exact(&mut input, &mut data).expect("the data");
        let mut texture: *mut c_void = std::ptr::null_mut();
        let f: Create = std::mem::transmute(if kind == 2 { cube_fn } else { texture_fn });
        let hr = f(device, data.as_ptr(), len as u32, &mut texture);
        let put = |out: &mut std::io::BufWriter<_>, w: u32| out.write_all(&w.to_le_bytes()).unwrap();
        put(&mut out, hr);
        if hr != 0 || texture.is_null() {
            put(&mut out, 0);
            put(&mut out, 0);
            out.flush().unwrap();
            continue;
        }
        let count = (std::mem::transmute::<usize, Count>(vt(texture, 13)))(texture);
        let faces = if kind == 2 { 6 } else { 1 };
        put(&mut out, count);
        put(&mut out, faces);
        for face in 0..faces {
            for level in 0..count {
                let mut desc = [0u32; 8];
                (std::mem::transmute::<usize, Desc>(vt(texture, 17)))(texture, level, desc.as_mut_ptr());
                let mut locked = [0u32; 2];
                let lr = if kind == 2 {
                    (std::mem::transmute::<usize, CubeLock>(vt(texture, 19)))(texture, face, level, locked.as_mut_ptr(), std::ptr::null(), 0x10)
                } else {
                    (std::mem::transmute::<usize, Lock>(vt(texture, 19)))(texture, level, locked.as_mut_ptr(), std::ptr::null(), 0x10)
                };
                assert_eq!(lr, 0, "LockRect");
                let bytes = level_bytes(&desc, locked[0] as usize, locked[1] as usize as *const u8);
                if kind == 2 {
                    (std::mem::transmute::<usize, CubeUnlock>(vt(texture, 20)))(texture, face, level);
                } else {
                    (std::mem::transmute::<usize, Unlock>(vt(texture, 20)))(texture, level);
                }
                for w in [desc[0], desc[6], desc[7], bytes.len() as u32] {
                    put(&mut out, w);
                }
                out.write_all(&bytes).unwrap();
            }
        }
        out.flush().unwrap();
        (std::mem::transmute::<usize, Release>(vt(texture, 2)))(texture);
    }
}
