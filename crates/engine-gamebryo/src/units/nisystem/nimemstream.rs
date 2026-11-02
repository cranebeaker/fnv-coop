//! `NiSystem/nimemstream.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base class constructor `00aa1d10` (it sets the base virtual table and zeroes
/// the position and the two function pointers) and destructor `00aa1d30` (it
/// sets the base virtual table back); both `thiscall` with no stack word.
pub(crate) const BASE_CONSTRUCTOR: u32 = 0x00aa_1d10;
const BASE_DESTRUCTOR: u32 = 0x00aa_1d30;
/// `00aa1070` and `00aa10f0`: the `cdecl` allocate (size) and free (block).
pub(crate) const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
/// `_memcpy` (`00ec44d0`, `cdecl`: destination, source, count).
const MEMCPY: u32 = 0x00ec_44d0;
/// The virtual table the memory stream constructors store (`010a256c`; its
/// scalar deleting destructor is the first slot).
pub(crate) const MEM_STREAM_VTABLE: u32 = 0x010a_256c;
/// The read and write functions the constructors store in the base class's
/// `m_pfnRead` and `m_pfnWrite` (`00aa2e60`, `00aa2ea0`; not translated here).
pub(crate) const MEM_STREAM_READ: u32 = 0x00aa_2e60;
pub(crate) const MEM_STREAM_WRITE: u32 = 0x00aa_2ea0;

layout! {
    /// `NiMemStream` (Xbox PDB), 0x24 bytes: `NiBinaryStream` (vtable, current
    /// position, read and write function pointers) and the buffer.
    pub struct NiMemStream: 0x24 {
        /// `m_uiAbsoluteCurrentPos` (Xbox PDB, `NiBinaryStream`).
        0x04 m_uiAbsoluteCurrentPos: u32,
        /// `m_pfnRead` (Xbox PDB, `NiBinaryStream`).
        0x08 m_pfnRead: u32,
        /// `m_pfnWrite` (Xbox PDB, `NiBinaryStream`).
        0x0C m_pfnWrite: u32,
        /// `m_pBuffer` (Xbox PDB).
        0x10 m_pBuffer: u32,
        /// `m_uiPos` (Xbox PDB).
        0x14 m_uiPos: u32,
        /// `m_uiEnd` (Xbox PDB).
        0x18 m_uiEnd: u32,
        /// `m_uiAllocSize` (Xbox PDB).
        0x1C m_uiAllocSize: u32,
        /// `m_bUserMemory` (Xbox PDB): the buffer belongs to the caller.
        0x20 m_bUserMemory: bool,
        /// `m_bFreeze` (Xbox PDB).
        0x21 m_bFreeze: bool,
    }
}

// Translated from 00aa2fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMemStream::NiMemStream(char* buffer, unsigned int size)` (engine map): a stream
/// over the caller's `buffer` of `size` bytes (user memory, already full:
/// `m_uiEnd` and `m_uiAllocSize` are `size`, the position 0). Returns `this`.
pub fn ni_mem_stream_constructor(
    e: &mut Engine,
    this: Ptr<NiMemStream>,
    buffer: u32,
    size: u32,
) -> Ptr<NiMemStream> {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.set(this, NiMemStream::m_pBuffer, buffer);
    e.set(this, NiMemStream::m_uiAllocSize, size);
    e.set(this, NiMemStream::m_uiEnd, size);
    e.mem.set_u32(this.addr(), MEM_STREAM_VTABLE);
    e.set(this, NiMemStream::m_uiPos, 0);
    e.set(this, NiMemStream::m_bUserMemory, true);
    e.set(this, NiMemStream::m_bFreeze, false);
    e.set(this, NiMemStream::m_pfnRead, MEM_STREAM_READ);
    e.set(this, NiMemStream::m_pfnWrite, MEM_STREAM_WRITE);
    this
}

// Translated from 00aa2d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMemStream::~NiMemStream` (engine map): puts the memory stream's virtual table
/// back, frees the buffer unless it is user memory or was taken
/// (`m_bFreeze`, see `00aa2d60`), then runs the base class destructor.
pub fn ni_mem_stream_destructor(e: &mut Engine, this: Ptr<NiMemStream>) {
    let user_memory = e.get(this, NiMemStream::m_bUserMemory);
    e.mem.set_u32(this.addr(), MEM_STREAM_VTABLE);
    if !user_memory && !e.get(this, NiMemStream::m_bFreeze) {
        let buffer = e.get(this, NiMemStream::m_pBuffer);
        e.call(FREE, &args![buffer]);
    }
    e.call(BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00aa2d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets `m_bFreeze` (Xbox PDB field at +0x21), which
/// stops the destructor from freeing the buffer, and returns `m_pBuffer`, so the
/// caller owns the buffer from here on.
pub fn fn_00aa2d60(e: &mut Engine, this: Ptr<NiMemStream>) -> u32 {
    e.set(this, NiMemStream::m_bFreeze, true);
    e.get(this, NiMemStream::m_pBuffer)
}

// Translated from 00aa2e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The read function the constructors store in `m_pfnRead` (called through the
/// pointer, `cdecl`, five stack words: the stream, the destination, the byte
/// count, and two words this body never reads (the component sizes array and
/// the number of components that `NiStreamLoadBinary` passes)). Copies
/// `min(count, m_uiEnd - m_uiPos)` bytes (unsigned) from `m_pBuffer + m_uiPos`
/// to the destination through `memcpy` (`00ec44d0`), advances `m_uiPos` by that
/// number and answers it.
pub fn ni_mem_stream_read(
    e: &mut Engine,
    this: Ptr<NiMemStream>,
    destination: u32,
    count: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> u32 {
    let position = e.get(this, NiMemStream::m_uiPos);
    let available = e.get(this, NiMemStream::m_uiEnd).wrapping_sub(position);
    let count = count.min(available);
    let buffer = e.get(this, NiMemStream::m_pBuffer);
    e.call(
        MEMCPY,
        &args![destination, buffer.wrapping_add(position), count],
    );
    e.set(this, NiMemStream::m_uiPos, position.wrapping_add(count));
    count
}

// Translated from 00aa2dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The write body that `00aa2ea0` (the `m_pfnWrite` function) calls: unnamed in
/// the engine map (`thiscall`, `RET 8`: source, byte count). A stream over user
/// memory refuses (answers 0). Otherwise, when `count` is more than the room left
/// in the allocation (`m_uiAllocSize - m_uiPos`), a larger buffer is allocated
/// (`00aa1070`): `m_uiAllocSize * 2`, unless `count` is also more than the
/// allocation plus that room, in which case `m_uiPos + count`. The old bytes
/// (`m_uiEnd` of them) are copied across (`memcpy`), the old buffer is freed
/// (`00aa10f0`) and the new one and its size are stored. Then `count` bytes from
/// `source` are copied to `m_pBuffer + m_uiPos`, `m_uiPos` advances by `count`,
/// `m_uiEnd` follows it when the position passes it, and `count` is answered.
pub fn fn_00aa2dd0(e: &mut Engine, this: Ptr<NiMemStream>, source: u32, count: u32) -> u32 {
    if e.get(this, NiMemStream::m_bUserMemory) {
        return 0;
    }
    let allocated = e.get(this, NiMemStream::m_uiAllocSize);
    let room = allocated.wrapping_sub(e.get(this, NiMemStream::m_uiPos));
    if count > room {
        let new_size = if count > allocated.wrapping_add(room) {
            allocated.wrapping_sub(room).wrapping_add(count)
        } else {
            allocated.wrapping_add(allocated)
        };
        let new_buffer = e.call(ALLOCATE, &args![new_size]).u32();
        let old_buffer = e.get(this, NiMemStream::m_pBuffer);
        let end = e.get(this, NiMemStream::m_uiEnd);
        e.call(MEMCPY, &args![new_buffer, old_buffer, end]);
        let old_buffer = e.get(this, NiMemStream::m_pBuffer);
        e.call(FREE, &args![old_buffer]);
        e.set(this, NiMemStream::m_pBuffer, new_buffer);
        e.set(this, NiMemStream::m_uiAllocSize, new_size);
    }
    let buffer = e.get(this, NiMemStream::m_pBuffer);
    let position = e.get(this, NiMemStream::m_uiPos);
    e.call(MEMCPY, &args![buffer.wrapping_add(position), source, count]);
    let position = position.wrapping_add(count);
    e.set(this, NiMemStream::m_uiPos, position);
    if position > e.get(this, NiMemStream::m_uiEnd) {
        e.set(this, NiMemStream::m_uiEnd, position);
    }
    count
}

/// The byte-swapping read and write functions `SetEndianSwap` stores (`00aa2ec0`,
/// `00aa2f20`; not translated here).
const MEM_STREAM_SWAP_READ: u32 = 0x00aa_2ec0;
const MEM_STREAM_SWAP_WRITE: u32 = 0x00aa_2f20;

// Translated from 00aa2f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMemStream::SetEndianSwap` (Xbox PDB; `thiscall`, `RET 4`): with `swap` set the
/// stream reads and writes through the byte-swapping functions (`00aa2ec0`, `00aa2f20`),
/// else through the plain ones (`00aa2e60`, `00aa2ea0`).
pub fn ni_mem_stream_set_endian_swap(e: &mut Engine, this: Ptr<NiMemStream>, swap: bool) {
    if swap {
        e.set(this, NiMemStream::m_pfnRead, MEM_STREAM_SWAP_READ);
        e.set(this, NiMemStream::m_pfnWrite, MEM_STREAM_SWAP_WRITE);
    } else {
        e.set(this, NiMemStream::m_pfnRead, MEM_STREAM_READ);
        e.set(this, NiMemStream::m_pfnWrite, MEM_STREAM_WRITE);
    }
}

// Translated from 00aa2d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMemStream::Seek` (Xbox PDB; `thiscall`, `RET 4`): moves the position by `offset`
/// when the new position is not negative (sign of the 32-bit sum) and lies below the word
/// at `+0x1C` (`m_uiAllocSize`, unsigned compare); the absolute position follows.
/// Otherwise nothing changes.
pub fn ni_mem_stream_seek(e: &mut Engine, this: Ptr<NiMemStream>, offset: i32) {
    let position = e
        .get(this, NiMemStream::m_uiPos)
        .wrapping_add(offset as u32);
    if (position as i32) < 0 {
        return;
    }
    if position >= e.get(this, NiMemStream::m_uiAllocSize) {
        return;
    }
    e.set(this, NiMemStream::m_uiPos, position);
    e.set(this, NiMemStream::m_uiAbsoluteCurrentPos, position);
}

/// The component byte swap (`00aa1de0`, cdecl: buffer, size, the component sizes, their
/// count).
const SWAP_COMPONENTS: u32 = 0x00aa_1de0;
/// The memory manager singleton and its `Deallocate` (`00aa4060`, thiscall, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00aa2d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMemStream::scalar deleting destructor` (engine map; thiscall, one word): the
/// destructor's body inline (the vtable `010a256c`, the buffer freed (`00aa10f0`) unless it
/// is user memory or frozen, the base destructor `00aa1d30`), then, with bit 0 of `flags`,
/// the stream given back to the memory manager. Answers `this`.
pub fn ni_mem_stream_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiMemStream>,
    flags: u8,
) -> Ptr<NiMemStream> {
    e.mem.set_u32(this.addr(), MEM_STREAM_VTABLE);
    if !e.get(this, NiMemStream::m_bUserMemory) && !e.get(this, NiMemStream::m_bFreeze) {
        let buffer = e.get(this, NiMemStream::m_pBuffer);
        e.call(FREE, &args![buffer]);
    }
    e.call(BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00aa2ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte-swapping read `SetEndianSwap` stores (unnamed in the engine map; cdecl, five
/// words): 0 for an empty read; otherwise copies `min(count, m_uiEnd - m_uiPos)` bytes from
/// `m_pBuffer + m_uiPos` (`memcpy`), advances `m_uiPos`, swaps the destination's `count`
/// bytes by the component sizes (`00aa1de0`) and answers the bytes copied.
pub fn fn_00aa2ec0(
    e: &mut Engine,
    this: Ptr<NiMemStream>,
    destination: u32,
    count: u32,
    sizes: u32,
    components: u32,
) -> u32 {
    if count == 0 {
        return 0;
    }
    let position = e.get(this, NiMemStream::m_uiPos);
    let available = e.get(this, NiMemStream::m_uiEnd).wrapping_sub(position);
    let copied = if count > available { available } else { count };
    let buffer = e.get(this, NiMemStream::m_pBuffer);
    e.call(
        MEMCPY,
        &args![destination, buffer.wrapping_add(position), copied],
    );
    let position = e.get(this, NiMemStream::m_uiPos).wrapping_add(copied);
    e.set(this, NiMemStream::m_uiPos, position);
    e.call(
        SWAP_COMPONENTS,
        &args![destination, count, sizes, components],
    );
    copied
}

// Translated from 00aa2f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte-swapping write `SetEndianSwap` stores (unnamed in the engine map; cdecl, five
/// words): 0 for an empty write; otherwise copies `source` into a block from `00aa1070`,
/// swaps the copy by the component sizes (`00aa1de0`), writes it ([`fn_00aa2dd0`]), frees the
/// copy (`00aa10f0`) and answers what the write answered.
pub fn fn_00aa2f20(
    e: &mut Engine,
    this: Ptr<NiMemStream>,
    source: u32,
    count: u32,
    sizes: u32,
    components: u32,
) -> u32 {
    if count == 0 {
        return 0;
    }
    let copy = e.call(ALLOCATE, &args![count]).u32();
    e.call(MEMCPY, &args![copy, source, count]);
    e.call(SWAP_COMPONENTS, &args![copy, count, sizes, components]);
    let written = fn_00aa2dd0(e, this, copy, count);
    e.call(FREE, &args![copy]);
    written
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_2d90,
            ni_mem_stream_scalar_deleting_destructor(Ptr<NiMemStream>, u8) -> Ptr<NiMemStream>
        ),
        entry!(
            0x00aa_2ec0,
            fn_00aa2ec0(Ptr<NiMemStream>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00aa_2f20,
            fn_00aa2f20(Ptr<NiMemStream>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00aa_2fb0,
            ni_mem_stream_constructor(Ptr<NiMemStream>, u32, u32) -> Ptr<NiMemStream>
        ),
        entry!(0x00aa_2d30, ni_mem_stream_destructor(Ptr<NiMemStream>)),
        entry!(0x00aa_2d60, fn_00aa2d60(Ptr<NiMemStream>) -> u32),
        entry!(0x00aa_2dd0, fn_00aa2dd0(Ptr<NiMemStream>, u32, u32) -> u32),
        entry!(
            0x00aa_2e60,
            ni_mem_stream_read(Ptr<NiMemStream>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00aa_2f80,
            ni_mem_stream_set_endian_swap(Ptr<NiMemStream>, bool)
        ),
        entry!(0x00aa_2d70, ni_mem_stream_seek(Ptr<NiMemStream>, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], target: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == target)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn constructor_over_a_user_buffer() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        let stream = e.new_object::<NiMemStream>();
        // Dirty fields prove every one is written.
        for off in (0u32..0x24).step_by(4) {
            e.mem.set_u32(stream.addr() + off, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        let result = e.call(0x00aa_2fb0, &args![stream, 0x4000u32, 0x30u32]);
        assert_eq!(result.u32(), stream.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BASE_CONSTRUCTOR), vec![vec![stream.addr()]]);
        let at = stream.addr();
        assert_eq!(e.mem.u32(at), MEM_STREAM_VTABLE);
        assert_eq!(e.mem.u32(at + 0x08), 0x00aa_2e60);
        assert_eq!(e.mem.u32(at + 0x0c), 0x00aa_2ea0);
        assert_eq!(e.mem.u32(at + 0x10), 0x4000);
        assert_eq!(e.mem.u32(at + 0x14), 0);
        assert_eq!(e.mem.u32(at + 0x18), 0x30);
        assert_eq!(e.mem.u32(at + 0x1c), 0x30);
        assert_eq!(e.mem.u8(at + 0x20), 1);
        assert_eq!(e.mem.u8(at + 0x21), 0);
    }

    #[test]
    fn destructor_frees_only_an_owned_buffer_that_was_not_taken() {
        // (user memory, taken, buffer freed)
        for (user, taken, freed) in [
            (0u8, 0u8, true),
            (1, 0, false),
            (0, 1, false),
            (1, 1, false),
        ] {
            let mut e = Engine::new();
            e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
            e.register(FREE, |_, _| Ret::default());
            let stream = e.new_object::<NiMemStream>();
            e.mem.set_u32(stream.addr() + 0x10, 0x7000);
            e.mem.set_u8(stream.addr() + 0x20, user);
            e.mem.set_u8(stream.addr() + 0x21, taken);
            e.call_log = Some(vec![]);
            e.call(0x00aa_2d30, &args![stream]);
            let log = e.call_log.take().unwrap();
            let frees = calls_to(&log, FREE);
            assert_eq!(frees, if freed { vec![vec![0x7000]] } else { vec![] });
            assert_eq!(calls_to(&log, BASE_DESTRUCTOR), vec![vec![stream.addr()]]);
            assert_eq!(e.mem.u32(stream.addr()), MEM_STREAM_VTABLE);
        }
    }

    #[test]
    fn taking_the_buffer_sets_the_freeze_flag_and_returns_it() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiMemStream>();
        e.mem.set_u32(stream.addr() + 0x10, 0x7000);
        assert_eq!(e.call(0x00aa_2d60, &args![stream]).u32(), 0x7000);
        assert_eq!(e.mem.u8(stream.addr() + 0x21), 1);
    }

    fn read_stream(e: &mut Engine, position: u32, end: u32) -> Ptr<NiMemStream> {
        e.register(MEMCPY, |_, _| Ret::default());
        let stream = e.new_object::<NiMemStream>();
        e.set(stream, NiMemStream::m_pBuffer, 0x7000);
        e.set(stream, NiMemStream::m_uiPos, position);
        e.set(stream, NiMemStream::m_uiEnd, end);
        stream
    }

    #[test]
    fn reading_copies_the_requested_bytes_and_advances() {
        let mut e = Engine::new();
        let stream = read_stream(&mut e, 4, 0x20);
        e.call_log = Some(vec![]);
        let words = args![stream, 0x9000u32, 0x10u32, 0xaaaau32, 1u32];
        assert_eq!(e.call(MEM_STREAM_READ, &words).u32(), 0x10);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMCPY), vec![vec![0x9000, 0x7004, 0x10]]);
        assert_eq!(e.get(stream, NiMemStream::m_uiPos), 0x14);
    }

    #[test]
    fn reading_stops_at_the_end() {
        let mut e = Engine::new();
        let stream = read_stream(&mut e, 0x18, 0x20);
        e.call_log = Some(vec![]);
        let words = args![stream, 0x9000u32, 0x10u32, 0u32, 1u32];
        assert_eq!(e.call(MEM_STREAM_READ, &words).u32(), 8);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMCPY), vec![vec![0x9000, 0x7018, 8]]);
        assert_eq!(e.get(stream, NiMemStream::m_uiPos), 0x20);
        // At the end nothing is left to read.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(MEM_STREAM_READ, &words).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMCPY), vec![vec![0x9000, 0x7020, 0]]);
    }

    fn copying_memcpy(e: &mut Engine) {
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    /// A growing stream: buffer of `allocated` bytes holding `end` bytes of
    /// 0xaa, the position at `position`.
    fn write_stream(e: &mut Engine, allocated: u32, position: u32, end: u32) -> Ptr<NiMemStream> {
        copying_memcpy(e);
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
        let stream = e.new_object::<NiMemStream>();
        let buffer = e.mem.alloc(allocated);
        e.mem.write(buffer, &vec![0xaa; end as usize]);
        e.set(stream, NiMemStream::m_pBuffer, buffer);
        e.set(stream, NiMemStream::m_uiAllocSize, allocated);
        e.set(stream, NiMemStream::m_uiPos, position);
        e.set(stream, NiMemStream::m_uiEnd, end);
        stream
    }

    #[test]
    fn writing_to_user_memory_answers_zero() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiMemStream>();
        e.set(stream, NiMemStream::m_bUserMemory, true);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_2dd0, &args![stream, 0x9000u32, 4u32]).u32(),
            0
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn writing_within_the_room_copies_and_moves_the_end() {
        let mut e = Engine::new();
        let stream = write_stream(&mut e, 0x40, 0x10, 0x14);
        let buffer = e.get(stream, NiMemStream::m_pBuffer);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[1, 2, 3, 4, 5, 6, 7, 8]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_2dd0, &args![stream, source, 8u32]).u32(), 8);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ALLOCATE).is_empty());
        assert_eq!(calls_to(&log, MEMCPY), vec![vec![buffer + 0x10, source, 8]]);
        assert_eq!(e.get(stream, NiMemStream::m_uiPos), 0x18);
        assert_eq!(e.get(stream, NiMemStream::m_uiEnd), 0x18);
        assert_eq!(e.mem.bytes(buffer + 0x10, 8), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        // A write that stays inside the stored bytes leaves the end alone.
        e.set(stream, NiMemStream::m_uiPos, 0);
        e.call(0x00aa_2dd0, &args![stream, source, 4u32]);
        assert_eq!(e.get(stream, NiMemStream::m_uiPos), 4);
        assert_eq!(e.get(stream, NiMemStream::m_uiEnd), 0x18);
    }

    #[test]
    fn a_write_past_the_room_doubles_the_allocation() {
        let mut e = Engine::new();
        let stream = write_stream(&mut e, 0x10, 0xc, 0xc);
        let old = e.get(stream, NiMemStream::m_pBuffer);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[9; 8]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_2dd0, &args![stream, source, 8u32]).u32(), 8);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x20]]);
        let new = e.get(stream, NiMemStream::m_pBuffer);
        assert_ne!(new, old);
        assert_eq!(calls_to(&log, MEMCPY)[0], vec![new, old, 0xc]);
        assert_eq!(calls_to(&log, FREE), vec![vec![old]]);
        assert_eq!(e.get(stream, NiMemStream::m_uiAllocSize), 0x20);
        assert_eq!(e.mem.bytes(new, 0xc), vec![0xaa; 0xc]);
        assert_eq!(e.mem.bytes(new + 0xc, 8), vec![9; 8]);
        assert_eq!(e.get(stream, NiMemStream::m_uiEnd), 0x14);
    }

    #[test]
    fn a_write_past_double_the_room_allocates_position_plus_count() {
        let mut e = Engine::new();
        // room 4, allocation 0x10: count 0x20 is more than 0x10 + 4.
        let stream = write_stream(&mut e, 0x10, 0xc, 0xc);
        let source = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00aa_2dd0, &args![stream, source, 0x20u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x2c]]);
        assert_eq!(e.get(stream, NiMemStream::m_uiAllocSize), 0x2c);
        assert_eq!(e.get(stream, NiMemStream::m_uiPos), 0x2c);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    #[test]
    fn set_endian_swap_picks_the_function_pair() {
        let mut e = Engine::new();
        let this = e.new_object::<NiMemStream>();
        e.call(0x00aa_2f80, &args![this, 1u32]);
        assert_eq!(e.get(this, NiMemStream::m_pfnRead), MEM_STREAM_SWAP_READ);
        assert_eq!(e.get(this, NiMemStream::m_pfnWrite), MEM_STREAM_SWAP_WRITE);
        // Only the low byte counts.
        e.call(0x00aa_2f80, &args![this, 0x100u32]);
        assert_eq!(e.get(this, NiMemStream::m_pfnRead), MEM_STREAM_READ);
        assert_eq!(e.get(this, NiMemStream::m_pfnWrite), MEM_STREAM_WRITE);
    }

    #[test]
    fn seek_moves_inside_the_buffer_only() {
        let mut e = Engine::new();
        let this = e.new_object::<NiMemStream>();
        e.set(this, NiMemStream::m_uiPos, 10);
        e.set(this, NiMemStream::m_uiAbsoluteCurrentPos, 10);
        e.set(this, NiMemStream::m_uiAllocSize, 20);
        e.call(0x00aa_2d70, &args![this, 5u32]);
        assert_eq!(e.get(this, NiMemStream::m_uiPos), 15);
        assert_eq!(e.get(this, NiMemStream::m_uiAbsoluteCurrentPos), 15);
        // Past the end, exactly the end, and below zero: nothing moves.
        for offset in [10i32, 5, -16] {
            e.call(0x00aa_2d70, &args![this, offset as u32]);
            assert_eq!(e.get(this, NiMemStream::m_uiPos), 15);
        }
        e.call(0x00aa_2d70, &args![this, (-15i32) as u32]);
        assert_eq!(e.get(this, NiMemStream::m_uiPos), 0);
        assert_eq!(e.get(this, NiMemStream::m_uiAbsoluteCurrentPos), 0);
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn deleting_destructor_frees_an_owned_buffer_and_itself() {
        let mut e = Engine::new();
        for callee in [FREE, BASE_DESTRUCTOR, MEMORY_DEALLOCATE] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.new_object::<NiMemStream>();
        e.set(this, NiMemStream::m_pBuffer, 0x5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_2d90, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), MEM_STREAM_VTABLE);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x5000]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this.addr()]]
        );
        for (user, frozen) in [(true, false), (false, true)] {
            e.set(this, NiMemStream::m_bUserMemory, user);
            e.set(this, NiMemStream::m_bFreeze, frozen);
            e.call_log = Some(vec![]);
            e.call(0x00aa_2d90, &args![this, 0u32]);
            assert!(calls_to(&e, FREE).is_empty());
            assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        }
    }

    #[test]
    fn swapping_read_copies_what_is_left_and_swaps_the_count() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(SWAP_COMPONENTS, |_, _| Ret::default());
        let this = e.new_object::<NiMemStream>();
        e.set(this, NiMemStream::m_pBuffer, 0x5000);
        e.set(this, NiMemStream::m_uiPos, 4);
        e.set(this, NiMemStream::m_uiEnd, 10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_2ec0, &args![this, 0x6000u32, 0u32, 1u32, 2u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, MEMCPY).is_empty());
        assert_eq!(
            e.call(0x00aa_2ec0, &args![this, 0x6000u32, 8u32, 1u32, 2u32])
                .u32(),
            6
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x6000, 0x5004, 6]]);
        assert_eq!(calls_to(&e, SWAP_COMPONENTS), vec![vec![0x6000, 8, 1, 2]]);
        assert_eq!(e.get(this, NiMemStream::m_uiPos), 10);
    }

    #[test]
    fn swapping_write_swaps_a_copy() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(SWAP_COMPONENTS, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| 0x7000u32.into_ret());
        e.register(FREE, |_, _| Ret::default());
        let this = e.new_object::<NiMemStream>();
        e.set(this, NiMemStream::m_pBuffer, 0x5000);
        e.set(this, NiMemStream::m_uiAllocSize, 0x100);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_2f20, &args![this, 0x6000u32, 0u32, 1u32, 2u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(
            e.call(0x00aa_2f20, &args![this, 0x6000u32, 8u32, 1u32, 2u32])
                .u32(),
            8
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x7000, 0x6000, 8], vec![0x5000, 0x7000, 8]]
        );
        assert_eq!(calls_to(&e, SWAP_COMPONENTS), vec![vec![0x7000, 8, 1, 2]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x7000]]);
        assert_eq!(e.get(this, NiMemStream::m_uiPos), 8);
        // A stream over user memory writes nothing.
        e.set(this, NiMemStream::m_bUserMemory, true);
        assert_eq!(
            e.call(0x00aa_2f20, &args![this, 0x6000u32, 8u32, 1u32, 2u32])
                .u32(),
            0
        );
    }
}
