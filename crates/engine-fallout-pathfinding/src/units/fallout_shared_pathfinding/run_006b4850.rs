//! `fallout shared/pathfinding/run_006b4850` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0069bda0` (unnamed in the engine map): the destructor body the
/// scalar deleting destructor `006b4850` runs.
const DTOR_BODY_0069BDA0: u32 = 0x0069_bda0;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `00401460` (unnamed in the engine map; cdecl: destination, source,
/// byte count): copies bytes out of the record.
const COPY_BYTES_00401460: u32 = 0x0040_1460;
/// `00401680` (unnamed in the engine map): the file's byte at +0x299.
const FILE_FLAG_00401680: u32 = 0x0040_1680;
/// `00401080` (unnamed in the engine map; cdecl, the address of a word and
/// a byte): run on each word read while the file's byte at +0x299 is set.
const FIX_UP_00401080: u32 = 0x0040_1080;
/// `TESForm::AddCompileIndex` (`00485d50`, Xbox PDB; cdecl, the address of
/// a form id and a file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `0069bcf0` (unnamed in the engine map): the constructor of the 0x38-byte
/// object kept at +0x58.
const EXTRA_CTOR_0069BCF0: u32 = 0x0069_bcf0;
/// `0069c8f0` (unnamed in the engine map; `thiscall`, `RET 0xC`: data,
/// file, version): reads the object at +0x58 from the record.
const EXTRA_LOAD_0069C8F0: u32 = 0x0069_c8f0;
/// `0069c460` (unnamed in the engine map): the byte count that object read.
const EXTRA_SIZE_0069C460: u32 = 0x0069_c460;

// Translated from 006b4850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): runs the destructor body `0069bda0` on `this`, then frees
/// `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_006b4850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DTOR_BODY_0069BDA0, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// Copies `size` bytes from `data` to `this + offset`, then, when the
/// file's byte is set, runs the word fix-up on each of `words` offsets.
fn read_field(
    e: &mut Engine,
    this: Ptr,
    offset: u32,
    data: u32,
    size: u32,
    file: Ptr,
    words: &[u32],
) {
    e.call(
        COPY_BYTES_00401460,
        &args![this.addr() + offset, data, size],
    );
    if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
        for word in words {
            e.call(FIX_UP_00401080, &args![this.addr() + word, 0u32]);
        }
    }
}

// Translated from 006b4b50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b4b50 - its FS:[0] exception frame (the unwind state that frees the 0x38-byte block when its constructor throws) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0xC`): reads the navmesh
/// info's fixed part from the record bytes at `data`, each field copied
/// with `00401460` and, when the file's byte (`00401680`) is set, fixed up
/// word by word with `00401080`: the word at +8 (from `data`), the word at
/// +0 (from `data + 4`, then given its compile index from `file`), the word
/// at +4 (from `data + 8`, compile index too), the word at +0xC (from
/// `data + 0xC`), and 0xC bytes at +0x10 (from `data + 0x10`, three
/// words). The data then continues at `data + 0x1C`. When bit 0x20 of +8 is
/// set, the 0x38-byte object at +0x58 is made first if missing (allocated,
/// then constructed by `0069bcf0`, null when the allocation fails), reads
/// itself (`0069c8f0(data, file, version)`) and the data moves on by the
/// size `0069c460` reports. With `version` 10 or more (signed), one more
/// word goes to +0x20.
pub fn fn_006b4b50(e: &mut Engine, this: Ptr, data: u32, file: Ptr, version: i32) {
    let mut data = data;
    read_field(e, this, 8, data, 4, file, &[8]);
    data = data.wrapping_add(4);
    read_field(e, this, 0, data, 4, file, &[0]);
    data = data.wrapping_add(4);
    e.call(TES_FORM_ADD_COMPILE_INDEX, &args![this, file]);
    read_field(e, this, 4, data, 4, file, &[4]);
    data = data.wrapping_add(4);
    e.call(TES_FORM_ADD_COMPILE_INDEX, &args![this.addr() + 4, file]);
    read_field(e, this, 0xc, data, 4, file, &[0xc]);
    data = data.wrapping_add(4);
    read_field(e, this, 0x10, data, 0xc, file, &[0x10, 0x14, 0x18]);
    data = data.wrapping_add(0xc);
    if e.mem.u32(this.addr() + 8) & 0x20 != 0 {
        if e.mem.u32(this.addr() + 0x58) == 0 {
            let block = e.call(ALLOCATE, &args![0x38u32]).u32();
            let extra = if block != 0 {
                e.call(EXTRA_CTOR_0069BCF0, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(this.addr() + 0x58, extra);
        }
        let extra = e.mem.u32(this.addr() + 0x58);
        e.call(EXTRA_LOAD_0069C8F0, &args![extra, data, file, version]);
        let extra = e.mem.u32(this.addr() + 0x58);
        let read = e.call(EXTRA_SIZE_0069C460, &args![extra]).u32();
        data = read.wrapping_add(data);
    }
    if version >= 10 {
        read_field(e, this, 0x20, data, 4, file, &[0x20]);
    }
}

/// `00401500` (no arguments): the byte at `011c54ba`; when it is set, each
/// word written goes through `00401080` first.
const SAVE_FLAG_00401500: u32 = 0x0040_1500;
/// `0084e3a0` (`thiscall`): the word at the object's `+0xc`.
const WORD_AT_C_0084E3A0: u32 = 0x0084_e3a0;
/// `0069c4c0` (unnamed in the engine map; `thiscall`, `RET 4`: data): writes
/// the object at `+0x58` into the record.
const EXTRA_SAVE_0069C4C0: u32 = 0x0069_c4c0;
/// `0044ddc0` (`thiscall`): the element count of an array.
const COUNT_0044DDC0: u32 = 0x0044_ddc0;
/// `00877a30` (`thiscall`, `RET 4`: index): the address of element `index`.
const ELEMENT_00877A30: u32 = 0x0087_7a30;
/// `0042fb00` and `0042fa40` (`thiscall`): construct and destroy the 4-byte
/// local `00690860` fills.
const LOCAL_CONSTRUCT_0042FB00: u32 = 0x0042_fb00;
const LOCAL_DESTRUCT_0042FA40: u32 = 0x0042_fa40;
/// `00690860` (`thiscall` on an element, `RET 4`: the local): a byte.
const ELEMENT_FILL_00690860: u32 = 0x0069_0860;
/// `00559450` (`thiscall` on the local): an object.
const LOCAL_OBJECT_00559450: u32 = 0x0055_9450;

// Translated from 006b4880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte count `006b48f0` writes (`thiscall`, plain `RET`): 0x1c for the
/// fixed fields, plus the size `0069c460` reports for the object at `+0x58`
/// when there is one, plus 4. The engine map gives it no name.
pub fn fn_006b4880(e: &mut Engine, this: Ptr) -> u32 {
    let mut size = 0x1cu32;
    if e.mem.u32(this.addr() + 0x58) != 0 {
        let extra = e.mem.u32(this.addr() + 0x58);
        size = e
            .call(EXTRA_SIZE_0069C460, &args![extra])
            .u32()
            .wrapping_add(size);
    }
    size.wrapping_add(4)
}

/// The word at `local` through `00401080` (with the byte 0) when
/// `00401500` says so.
fn fix_up_if_set(e: &mut Engine, local: u32) {
    if e.call(SAVE_FLAG_00401500, &[]).u8() != 0 {
        e.call(FIX_UP_00401080, &args![local, 0u32]);
    }
}

// Translated from 006b48f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the navmesh info's fixed part to the record bytes at `data`
/// (`thiscall`; one word; `RET 4`), the counterpart of `006b4b50`: each
/// field is put in a local on the stack, fixed up there with `00401080`
/// when `00401500` is set, and copied out with `00401460`: the word at +8,
/// the word at +0 (first replaced by `0084e3a0` of the object at `+0x54`
/// when there is one and its answer differs; that call is made a second
/// time for the value stored), the word at +4 (first set to `0084e3a0` of
/// the object at `+0x1c`), the word at `+0xc`, the three words at `+0x10`
/// (12 bytes). The data then continues at `data + 0x1c`: the object at
/// `+0x58`, when there is one, writes itself (`0069c4c0`) and the data
/// moves on by the size `0069c460` reports. Last comes the `float` at
/// `+0x20`, copied into the local by `FLD`/`FSTP`. The engine map gives it
/// no name.
pub fn fn_006b48f0(e: &mut Engine, this: Ptr, data: u32) {
    let base = this.addr();
    // The locals: `[EBP-0x14]` at +0, `[EBP-0x10]` at +4 and the three
    // words `[EBP-0xc]` to `[EBP-4]` at +8.
    e.with_stack(0x14, |e, frame| {
        let word = frame.addr();
        let last = frame.addr() + 4;
        let point = frame.addr() + 8;
        let mut data = data;

        let value = e.mem.u32(base + 8);
        e.mem.set_u32(word, value);
        fix_up_if_set(e, word);
        e.call(COPY_BYTES_00401460, &args![data, word, 4u32]);
        data = data.wrapping_add(4);

        if e.mem.u32(base + 0x54) != 0 {
            let owner = e.mem.u32(base + 0x54);
            let id = e.call(WORD_AT_C_0084E3A0, &args![owner]).u32();
            if id != e.mem.u32(base) {
                let owner = e.mem.u32(base + 0x54);
                let id = e.call(WORD_AT_C_0084E3A0, &args![owner]).u32();
                e.mem.set_u32(base, id);
            }
        }
        let value = e.mem.u32(base);
        e.mem.set_u32(word, value);
        fix_up_if_set(e, word);
        e.call(COPY_BYTES_00401460, &args![data, word, 4u32]);
        data = data.wrapping_add(4);

        let object = e.mem.u32(base + 0x1c);
        let id = e.call(WORD_AT_C_0084E3A0, &args![object]).u32();
        e.mem.set_u32(base + 4, id);
        let value = e.mem.u32(base + 4);
        e.mem.set_u32(word, value);
        fix_up_if_set(e, word);
        e.call(COPY_BYTES_00401460, &args![data, word, 4u32]);
        data = data.wrapping_add(4);

        let value = e.mem.u32(base + 0xc);
        e.mem.set_u32(word, value);
        fix_up_if_set(e, word);
        e.call(COPY_BYTES_00401460, &args![data, word, 4u32]);
        data = data.wrapping_add(4);

        for i in 0..3 {
            let value = e.mem.u32(base + 0x10 + 4 * i);
            e.mem.set_u32(point + 4 * i, value);
        }
        if e.call(SAVE_FLAG_00401500, &[]).u8() != 0 {
            for i in 0..3 {
                e.call(FIX_UP_00401080, &args![point + 4 * i, 0u32]);
            }
        }
        e.call(COPY_BYTES_00401460, &args![data, point, 0xcu32]);
        data = data.wrapping_add(0xc);

        if e.mem.u32(base + 0x58) != 0 {
            let extra = e.mem.u32(base + 0x58);
            e.call(EXTRA_SAVE_0069C4C0, &args![extra, data]);
            let extra = e.mem.u32(base + 0x58);
            data = e
                .call(EXTRA_SIZE_0069C460, &args![extra])
                .u32()
                .wrapping_add(data);
        }

        let value = x87_quiet_bits(e.mem.u32(base + 0x20));
        e.mem.set_u32(last, value);
        fix_up_if_set(e, last);
        e.call(COPY_BYTES_00401460, &args![data, last, 4u32]);
    });
}

// Translated from 006b4e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte count `006b4e80` writes (`thiscall`, plain `RET`): a count word
/// and 4 bytes per element for each of the arrays at `+0x24`, `+0x34` and
/// `+0x44` (`0044ddc0`). The engine map gives it no name.
pub fn fn_006b4e10(e: &mut Engine, this: Ptr) -> u32 {
    let mut size = 0u32;
    for array in [0x24, 0x34, 0x44] {
        size = size.wrapping_add(4);
        let count = e.call(COUNT_0044DDC0, &args![this.addr() + array]).u32();
        size = size.wrapping_add(count.wrapping_mul(4));
    }
    size
}

/// The word in the local `value` through the fix-up when set, then stored
/// at `out`.
fn put_word(e: &mut Engine, value: u32, out: u32) {
    fix_up_if_set(e, value);
    let word = e.mem.u32(value);
    e.mem.set_u32(out, word);
}

// Translated from 006b4e80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b4e80 - the exception-unwinding frame (FS:[0]) that destroys the loop's local on unwind is not translated.
/// Writes the three arrays at `+0x24`, `+0x34` and `+0x44` to the record
/// words at `out` (`thiscall`; one word; `RET 4`), each word put in the
/// local `[EBP-0x1c]` and fixed up there (`00401080`) when `00401500` is
/// set. For each array its count (`0044ddc0`) comes first. The first two
/// write one word per element: a local is constructed (`0042fb00`), filled
/// by the element's `00690860`, and when that answers true the word is
/// `0084e3a0` of the local's object (`00559450`); otherwise the slot is
/// left as it is (the output still moves on); then the local is destroyed
/// (`0042fa40`). The third writes, per element, 0 for a null element and
/// `0084e3a0` of the element otherwise. The engine map gives it no name.
pub fn fn_006b4e80(e: &mut Engine, this: Ptr, out: u32) {
    let base = this.addr();
    // The locals: `[EBP-0x2c]` at +0, `[EBP-0x24]` at +8, `[EBP-0x1c]` at
    // +0x10.
    e.with_stack(0x14, |e, frame| {
        let value = frame.addr() + 0x10;
        let mut out = out;
        for (array, local) in [(0x24, frame.addr() + 8), (0x34, frame.addr())] {
            let count = e.call(COUNT_0044DDC0, &args![base + array]).u32();
            e.mem.set_u32(value, count);
            put_word(e, value, out);
            out = out.wrapping_add(4);
            let count = e.call(COUNT_0044DDC0, &args![base + array]).u32();
            let mut index = 0u32;
            while index < count {
                let slot = e.call(ELEMENT_00877A30, &args![base + array, index]).u32();
                let element = e.mem.u32(slot);
                e.call(LOCAL_CONSTRUCT_0042FB00, &args![local]);
                if e.call(ELEMENT_FILL_00690860, &args![element, local]).bool() {
                    let object = e.call(LOCAL_OBJECT_00559450, &args![local]).u32();
                    let id = e.call(WORD_AT_C_0084E3A0, &args![object]).u32();
                    e.mem.set_u32(value, id);
                    put_word(e, value, out);
                }
                out = out.wrapping_add(4);
                e.call(LOCAL_DESTRUCT_0042FA40, &args![local]);
                index += 1;
            }
        }
        let count = e.call(COUNT_0044DDC0, &args![base + 0x44]).u32();
        e.mem.set_u32(value, count);
        put_word(e, value, out);
        out = out.wrapping_add(4);
        let count = e.call(COUNT_0044DDC0, &args![base + 0x44]).u32();
        let mut index = 0u32;
        while index < count {
            let slot = e.call(ELEMENT_00877A30, &args![base + 0x44, index]).u32();
            let element = e.mem.u32(slot);
            if element == 0 {
                e.mem.set_u32(out, 0);
            } else {
                let id = e.call(WORD_AT_C_0084E3A0, &args![element]).u32();
                e.mem.set_u32(value, id);
                put_word(e, value, out);
            }
            out = out.wrapping_add(4);
            index += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_4850, fn_006b4850(Ptr, u32) -> Ptr),
        entry!(0x006b_4b50, fn_006b4b50(Ptr, u32, Ptr, i32)),
        entry!(0x006b_4880, fn_006b4880(Ptr) -> u32),
        entry!(0x006b_48f0, fn_006b48f0(Ptr, u32)),
        entry!(0x006b_4e10, fn_006b4e10(Ptr) -> u32),
        entry!(0x006b_4e80, fn_006b4e80(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_006b4850_runs_destructor_body_then_deletes_on_flag_one() {
        let mut e = Engine::new();
        e.register(DTOR_BODY_0069BDA0, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006b_4850, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x006b_4850).collect();
            let mut expected = vec![(DTOR_BODY_0069BDA0, vec![this])];
            if deletes {
                expected.push((DEALLOCATE, vec![this]));
            }
            assert_eq!(calls, expected);
        }
    }

    fn engine(flag: u32) -> Engine {
        let mut e = Engine::new();
        e.register(COPY_BYTES_00401460, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        if flag != 0 {
            e.register(FILE_FLAG_00401680, |_, _| 1u32.into_ret());
        } else {
            e.register(FILE_FLAG_00401680, |_, _| Ret::default());
        }
        e.register(FIX_UP_00401080, |_, _| Ret::default());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTRA_CTOR_0069BCF0, |_, a| a[0].into_ret());
        e.register(EXTRA_LOAD_0069C8F0, |_, _| Ret::default());
        e.register(EXTRA_SIZE_0069C460, |_, _| 0x10u32.into_ret());
        e
    }

    fn record(e: &mut Engine, flags: u32) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, flags);
        for i in 1..16u32 {
            e.mem.set_u32(data + 4 * i, 0x100 + i);
        }
        data
    }

    #[test]
    fn at_006b4b50_reads_the_fixed_fields() {
        let mut e = engine(0);
        let this = e.mem.alloc(0x60);
        let data = record(&mut e, 0x1);
        let file = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        e.call(0x006b_4b50, &args![this, data, file, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 8), 0x1);
        assert_eq!(e.mem.u32(this), 0x101);
        assert_eq!(e.mem.u32(this + 4), 0x102);
        assert_eq!(e.mem.u32(this + 0xc), 0x103);
        assert_eq!(e.mem.u32(this + 0x10), 0x104);
        assert_eq!(e.mem.u32(this + 0x18), 0x106);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        let compile: Vec<_> = log
            .iter()
            .filter(|c| c.0 == TES_FORM_ADD_COMPILE_INDEX)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(compile, vec![vec![this, file], vec![this + 4, file]]);
        assert!(log
            .iter()
            .all(|c| c.0 != FIX_UP_00401080 && c.0 != ALLOCATE));
    }

    #[test]
    fn at_006b4b50_fixes_words_reads_the_extra_part_and_the_version_word() {
        let mut e = engine(1);
        let this = e.mem.alloc(0x60);
        let data = record(&mut e, 0x20);
        let file = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        e.call(0x006b_4b50, &args![this, data, file, 10u32]);
        let log = e.call_log.take().unwrap();
        let fixes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == FIX_UP_00401080)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(fixes, vec![8, 0, 4, 0xc, 0x10, 0x14, 0x18, 0x20]);
        let extra = e.mem.u32(this + 0x58);
        assert_ne!(extra, 0);
        let load = log.iter().find(|c| c.0 == EXTRA_LOAD_0069C8F0).unwrap();
        assert_eq!(load.1, vec![extra, data + 0x1c, file, 10]);
        // The version word comes after the extra part's 0x10 bytes.
        assert_eq!(e.mem.u32(this + 0x20), 0x10b);

        // An existing extra object is reused.
        e.call_log = Some(vec![]);
        e.call(0x006b_4b50, &args![this, data, file, 10u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != ALLOCATE));
    }

    #[test]
    fn at_006b4b50_stores_null_when_the_extra_allocation_fails() {
        let mut e = engine(0);
        e.register(ALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let data = record(&mut e, 0x20);
        let file = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        e.call(0x006b_4b50, &args![this, data, file, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert!(log.iter().all(|c| c.0 != EXTRA_CTOR_0069BCF0));
        let load = log.iter().find(|c| c.0 == EXTRA_LOAD_0069C8F0).unwrap();
        assert_eq!(load.1[0], 0);
    }

    /// Doubles for the writers: the save flag answers `flag`, the fix-up
    /// swaps the bytes of the word at its address, `0084e3a0` reads `+0xc`.
    fn save_engine(flag: bool) -> Engine {
        let mut e = Engine::new();
        e.register(COPY_BYTES_00401460, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        if flag {
            e.register(SAVE_FLAG_00401500, |_, _| 0x101u32.into_ret());
        } else {
            e.register(SAVE_FLAG_00401500, |_, _| 0x100u32.into_ret());
        }
        e.register(FIX_UP_00401080, |e, a| {
            let word = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], word.swap_bytes());
            Ret::default()
        });
        e.register(WORD_AT_C_0084E3A0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(EXTRA_SAVE_0069C4C0, |_, _| Ret::default());
        e.register(EXTRA_SIZE_0069C460, |_, _| 0x10u32.into_ret());
        e
    }

    fn object_with_c(e: &mut Engine, value: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, value);
        object
    }

    #[test]
    fn at_006b4880_counts_the_extra_part() {
        let mut e = save_engine(false);
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x006b_4880, &args![this]).u32(), 0x20);
        e.mem.set_u32(this + 0x58, 0x5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_4880, &args![this]).u32(), 0x30);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == EXTRA_SIZE_0069C460)
            .collect();
        assert_eq!(sizes[0].1, vec![0x5000]);
    }

    #[test]
    fn at_006b48f0_writes_the_fixed_fields() {
        let mut e = save_engine(false);
        let this = e.mem.alloc(0x60);
        for i in 0..8u32 {
            e.mem.set_u32(this + 4 * i, 0x100 + i);
        }
        // A signalling NaN in the float comes out quieted (FLD/FSTP).
        e.mem.set_u32(this + 0x20, 0x7f80_0001);
        let owner = object_with_c(&mut e, 0x100);
        e.mem.set_u32(this + 0x54, owner);
        let object = object_with_c(&mut e, 0x777);
        e.mem.set_u32(this + 0x1c, object);
        let data = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x006b_48f0, &args![this, data]);
        let log = e.call_log.take().unwrap();
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(data + 4 * i)).collect();
        // +8, +0 (the owner answers the same id: kept), +4 (from the object
        // at +0x1c), +0xc, the three words at +0x10, then the float.
        assert_eq!(
            words,
            vec![0x102, 0x100, 0x777, 0x103, 0x104, 0x105, 0x106, 0x7fc0_0001]
        );
        assert_eq!(e.mem.u32(this + 4), 0x777);
        let ids: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == WORD_AT_C_0084E3A0)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(ids, vec![owner, object]);
        assert!(!log.iter().any(|(a, _)| *a == FIX_UP_00401080));
        assert!(!log.iter().any(|(a, _)| *a == EXTRA_SAVE_0069C4C0));
    }

    #[test]
    fn at_006b48f0_fixes_up_replaces_the_id_and_writes_the_extra_part() {
        let mut e = save_engine(true);
        let this = e.mem.alloc(0x60);
        for i in 0..8u32 {
            e.mem.set_u32(this + 4 * i, 0x100 + i);
        }
        let owner = object_with_c(&mut e, 0x555);
        e.mem.set_u32(this + 0x54, owner);
        let object = object_with_c(&mut e, 0x777);
        e.mem.set_u32(this + 0x1c, object);
        e.mem.set_u32(this + 0x58, 0x6000);
        e.mem.set_u32(this + 0x20, 0x200);
        let data = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x006b_48f0, &args![this, data]);
        let log = e.call_log.take().unwrap();
        // The owner's id differs: asked twice, stored at +0.
        assert_eq!(e.mem.u32(this), 0x555);
        let ids = log.iter().filter(|(a, _)| *a == WORD_AT_C_0084E3A0).count();
        assert_eq!(ids, 3);
        // Every word is fixed up in the local, the object's own fields keep
        // their values.
        assert_eq!(e.mem.u32(data), 0x102u32.swap_bytes());
        assert_eq!(e.mem.u32(data + 4), 0x555u32.swap_bytes());
        assert_eq!(e.mem.u32(data + 0x18), 0x106u32.swap_bytes());
        assert_eq!(e.mem.u32(this + 0x18), 0x106);
        let saves: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == EXTRA_SAVE_0069C4C0)
            .collect();
        assert_eq!(saves[0].1, vec![0x6000, data + 0x1c]);
        // The float follows the extra part's 0x10 bytes.
        assert_eq!(e.mem.u32(data + 0x2c), 0x200u32.swap_bytes());
        assert_eq!(log.iter().filter(|(a, _)| *a == FIX_UP_00401080).count(), 8);
    }

    #[test]
    fn at_006b4e10_counts_three_arrays() {
        let mut e = save_engine(false);
        e.register(COUNT_0044DDC0, |_, a| (a[0] & 0xff).into_ret());
        let this = e.mem.alloc(0x50);
        // Counts 0x24, 0x34, 0x44 (the low byte of each array's address).
        let expected = 12
            + 4 * ((this + 0x24) & 0xff)
            + 4 * ((this + 0x34) & 0xff)
            + 4 * ((this + 0x44) & 0xff);
        assert_eq!(e.call(0x006b_4e10, &args![this]).u32(), expected);
    }

    #[test]
    fn at_006b4e80_writes_counts_and_element_ids() {
        let mut e = save_engine(false);
        let this = e.mem.alloc(0x50);
        // Arrays: +0x24 holds two elements, +0x34 one, +0x44 two (one null).
        let first = [object_with_c(&mut e, 0xa1), object_with_c(&mut e, 0xa2)];
        let second = object_with_c(&mut e, 0xb1);
        let third = object_with_c(&mut e, 0xc1);
        let slots = e.mem.alloc(0x20);
        for (i, word) in [first[0], first[1], second, 0, third].iter().enumerate() {
            e.mem.set_u32(slots + 4 * i as u32, *word);
        }
        e.register_double(COUNT_0044DDC0, move |_, a| {
            let count = match a[0] - this {
                0x24 => 2,
                0x34 => 1,
                _ => 2,
            };
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        e.register_double(ELEMENT_00877A30, move |_, a| {
            let start = match a[0] - this {
                0x24 => 0,
                0x34 => 2,
                _ => 3,
            };
            Ret {
                eax: slots + 4 * (start + a[1]),
                ..Ret::default()
            }
        });
        e.register(LOCAL_CONSTRUCT_0042FB00, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(LOCAL_DESTRUCT_0042FA40, |_, _| Ret::default());
        // The element's fill answers true except for the second element of
        // the first array; it puts the element itself in the local.
        let skipped = first[1];
        e.register_double(ELEMENT_FILL_00690860, move |e, a| {
            e.mem.set_u32(a[1], a[0]);
            Ret {
                eax: u32::from(a[0] != skipped),
                ..Ret::default()
            }
        });
        e.register(LOCAL_OBJECT_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        let out = e.mem.alloc(0x40);
        for i in 0..8u32 {
            e.mem.set_u32(out + 4 * i, 0xeeee);
        }
        e.call_log = Some(vec![]);
        e.call(0x006b_4e80, &args![this, out]);
        let log = e.call_log.take().unwrap();
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![2, 0xa1, 0xeeee, 1, 0xb1, 2, 0, 0xc1]);
        // One local per element of the first two arrays, constructed and
        // destroyed around each fill.
        let locals: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == LOCAL_CONSTRUCT_0042FB00 || *a == LOCAL_DESTRUCT_0042FA40)
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            locals,
            vec![
                LOCAL_CONSTRUCT_0042FB00,
                LOCAL_DESTRUCT_0042FA40,
                LOCAL_CONSTRUCT_0042FB00,
                LOCAL_DESTRUCT_0042FA40,
                LOCAL_CONSTRUCT_0042FB00,
                LOCAL_DESTRUCT_0042FA40,
            ]
        );
    }

    #[test]
    fn at_006b4e80_fixes_up_each_word_when_set() {
        let mut e = save_engine(true);
        let this = e.mem.alloc(0x50);
        let element = object_with_c(&mut e, 0x1234);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, element);
        e.register_double(COUNT_0044DDC0, move |_, a| Ret {
            eax: u32::from(a[0] == this + 0x44),
            ..Ret::default()
        });
        e.register_double(ELEMENT_00877A30, move |_, _| Ret {
            eax: slot,
            ..Ret::default()
        });
        let out = e.mem.alloc(0x20);
        e.call(0x006b_4e80, &args![this, out]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 1u32.swap_bytes(), 0x1234u32.swap_bytes()]);
    }
}
