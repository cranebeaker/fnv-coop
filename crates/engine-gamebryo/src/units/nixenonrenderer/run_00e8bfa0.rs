//! `NiXenonRenderer/run_00e8bfa0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two `thiscall` helpers on the object the renderer keeps at +0x8BC (the lock is also called
//! from the function Ghidra names `NiXenonVertexBufferManager::PackUntransformedVB`):
//! remaking one stream's chip through the data's geometry group, and locking a vertex buffer
//! into the object's staging copy (its unlock is `00e8c0d0`, unit `run_00e8c0d0`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeometryBufferData` (Xbox PDB) fields read here: `m_pkGeometryGroup` +0x08,
/// `m_uiStreamCount` +0x20, `m_ppkVBChip` +0x28.
const DATA_GEOMETRY_GROUP: u32 = 0x08;
const DATA_STREAM_COUNT: u32 = 0x20;
const DATA_CHIPS: u32 = 0x28;
/// `NiGeometryGroup::CreateChip` and `ReleaseChip` (Xbox PDB vtable slots +0x18 and +0x1C,
/// `thiscall`: the data and the stream).
const GROUP_CREATE_CHIP: u32 = 0x18;
const GROUP_RELEASE_CHIP: u32 = 0x1c;

/// `IDirect3DVertexBuffer9::Lock` (slot 11: offset, size, out pointer, flags).
const VERTEX_BUFFER_LOCK: u32 = 0x2c;
/// `EnterCriticalSection` (KERNEL32 import slot).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// `00aa1070` (`cdecl`, the size) allocates a block, `00aa10f0` (`cdecl`, the block) frees it.
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;

/// The object's staging fields (unnamed; read from the code): the staging block +0x40, its
/// capacity +0x44, the locked address +0x48, the locked size +0x4C and a critical section
/// +0x80 held from the lock to the unlock.
const STAGING_BLOCK: u32 = 0x40;
const STAGING_CAPACITY: u32 = 0x44;
const LOCKED_ADDRESS: u32 = 0x48;
const LOCKED_SIZE: u32 = 0x4c;
const LOCK_SECTION: u32 = 0x80;

// Translated from 00e8bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: remakes stream `stream` of the geometry data `data`. Its
/// geometry group (+0x08) releases the stream's chip (`ReleaseChip`, slot +0x1C) and makes a
/// new one (`CreateChip`, slot +0x18); a null chip answers false; otherwise it is stored in
/// the data's chip array (when the stream is within the count) and the answer is true. `this`
/// is not read.
pub fn fn_00e8bfa0(e: &mut Engine, _this: Ptr, data: Ptr, stream: u32) -> bool {
    let group = e.mem.u32(data.addr() + DATA_GEOMETRY_GROUP);
    e.vcall(group, GROUP_RELEASE_CHIP, &args![data, stream]);
    let chip = e
        .vcall(group, GROUP_CREATE_CHIP, &args![data, stream])
        .u32();
    if chip == 0 {
        return false;
    }
    if stream < e.mem.u32(data.addr() + DATA_STREAM_COUNT) {
        let chips = e.mem.u32(data.addr() + DATA_CHIPS);
        e.mem
            .set_u32(chips.wrapping_add(stream.wrapping_mul(4)), chip);
    }
    true
}

// Translated from 00e8bff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: locks `size` bytes at `offset` of the vertex buffer `buffer`
/// with `flags` (`Lock`, slot +0x2C, the locked address returned through a local cleared
/// first). A failed lock (negative result) answers null. Otherwise the object's critical
/// section (+0x80) is entered, the locked address and size are kept (+0x48, +0x4C), the
/// staging block (+0x40) is replaced by a new one of `size` bytes when its capacity (+0x44) is
/// smaller, and unless `flags` has a bit of 0x3000 (`D3DLOCK_DISCARD` / `D3DLOCK_NOOVERWRITE`)
/// the locked bytes are copied into it. Answers the staging block.
pub fn fn_00e8bff0(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    offset: u32,
    size: u32,
    flags: u32,
) -> u32 {
    let base = this.addr();
    let (result, locked) = e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        let result = e
            .vcall(buffer, VERTEX_BUFFER_LOCK, &args![offset, size, out, flags])
            .i32();
        (result, e.mem.u32(out.addr()))
    });
    if result < 0 {
        return 0;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![base + LOCK_SECTION]);
    e.mem.set_u32(base + LOCKED_ADDRESS, locked);
    e.mem.set_u32(base + LOCKED_SIZE, size);
    if e.mem.u32(base + STAGING_CAPACITY) < size {
        let old = e.mem.u32(base + STAGING_BLOCK);
        e.call(FREE_00AA10F0, &args![old]);
        let block = e.call(ALLOCATE_00AA1070, &args![size]).u32();
        e.mem.set_u32(base + STAGING_BLOCK, block);
        e.mem.set_u32(base + STAGING_CAPACITY, size);
    }
    if flags & 0x3000 == 0 {
        let block = e.mem.u32(base + STAGING_BLOCK);
        e.call(MEMCPY, &args![block, locked, size]);
    }
    e.mem.u32(base + STAGING_BLOCK)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_bfa0, fn_00e8bfa0(Ptr, Ptr, u32) -> bool),
        entry!(0x00e8_bff0, fn_00e8bff0(Ptr, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const CREATE_FN: u32 = 0x0040_3000;
    const RELEASE_FN: u32 = 0x0040_3010;
    const LOCK_FN: u32 = 0x0040_3020;

    #[test]
    fn remake_chip_releases_creates_and_stores() {
        let mut e = Engine::new();
        e.register(RELEASE_FN, |_, _| Ret::default());
        e.register(CREATE_FN, |_, _| 0xc41fu32.into_ret());
        e.put_vtable(0x0090_4000, &[0, 0, 0, 0, 0, 0, CREATE_FN, RELEASE_FN]);
        let group = e.mem.alloc(0xc);
        e.mem.set_u32(group, 0x0090_4000);
        let data: Ptr = Ptr::new(e.mem.alloc(0x50));
        let chips = e.mem.alloc(8);
        e.mem.set_u32(data.addr() + DATA_GEOMETRY_GROUP, group);
        e.mem.set_u32(data.addr() + DATA_STREAM_COUNT, 2);
        e.mem.set_u32(data.addr() + DATA_CHIPS, chips);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_bfa0, &args![0u32, data, 1u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RELEASE_FN),
            vec![vec![group, data.addr(), 1]]
        );
        assert_eq!(calls_to(&log, CREATE_FN), vec![vec![group, data.addr(), 1]]);
        assert_eq!(e.mem.u32(chips + 4), 0xc41f);
        // A stream past the count: true, nothing stored.
        assert!(e.call(0x00e8_bfa0, &args![0u32, data, 2u32]).bool());
        // No chip: false.
        e.register(CREATE_FN, |_, _| Ret::default());
        e.mem.set_u32(chips, 0x77);
        assert!(!e.call(0x00e8_bfa0, &args![0u32, data, 0u32]).bool());
        assert_eq!(e.mem.u32(chips), 0x77);
    }

    #[test]
    fn lock_copies_into_the_staging_block() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x10);
        e.mem.write(source, &[1, 2, 3, 4, 5, 6, 7, 8]);
        e.register_double(LOCK_FN, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0, "the out pointer is cleared first");
            e.mem.set_u32(a[3], source);
            Ret::default()
        });
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        let mut slots = vec![0u32; 12];
        slots[11] = LOCK_FN;
        e.put_vtable(0x0090_5000, &slots);
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, 0x0090_5000);
        let this: Ptr = Ptr::new(e.mem.alloc(0x100));
        let old = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + STAGING_BLOCK, old);
        e.mem.set_u32(this.addr() + STAGING_CAPACITY, 4);

        e.call_log = Some(vec![]);
        let block = e
            .call(0x00e8_bff0, &args![this, buffer, 0x20u32, 8u32, 0u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOCK_FN)[0][..3], [buffer, 0x20, 8]);
        assert_eq!(calls_to(&log, LOCK_FN)[0][4], 0);
        assert_eq!(
            calls_to(&log, ENTER_CRITICAL_SECTION),
            vec![vec![this.addr() + LOCK_SECTION]]
        );
        assert_eq!(calls_to(&log, FREE_00AA10F0), vec![vec![old]]);
        assert_ne!(block, old);
        assert_eq!(e.mem.u32(this.addr() + STAGING_BLOCK), block);
        assert_eq!(e.mem.u32(this.addr() + STAGING_CAPACITY), 8);
        assert_eq!(e.mem.u32(this.addr() + LOCKED_ADDRESS), source);
        assert_eq!(e.mem.u32(this.addr() + LOCKED_SIZE), 8);
        assert_eq!(e.mem.bytes(block, 8), vec![1, 2, 3, 4, 5, 6, 7, 8]);

        // Big enough and discarding: no new block, no copy.
        e.call_log = Some(vec![]);
        let again = e
            .call(0x00e8_bff0, &args![this, buffer, 0u32, 4u32, 0x2000u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(again, block);
        assert!(calls_to(&log, FREE_00AA10F0).is_empty());
        assert!(calls_to(&log, MEMCPY).is_empty());

        // A failed lock answers 0 and enters nothing.
        e.register(LOCK_FN, |_, _| 0x8000_0000u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e8_bff0, &args![this, buffer, 0u32, 4u32, 0u32])
                .u32(),
            0
        );
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ENTER_CRITICAL_SECTION).is_empty());
    }
}
