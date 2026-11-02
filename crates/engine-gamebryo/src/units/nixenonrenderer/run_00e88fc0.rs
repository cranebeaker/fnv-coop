//! `NiXenonRenderer/run_00e88fc0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00e895f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem; no arguments):
/// allocates 0x18 bytes from the memory manager, zeroes the first four words, sets the
/// words at `+0x10` and `+0x14` to 8 and keeps the block in `0126f9a0` (null when the
/// allocation fails).
pub fn fn_00e895f0(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(0x0126_f9a0, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(0x0126_f9a0, block);
}

/// The pool this item goes back to (`0126f9a0`) and `NiTPrimitiveSet<..>::AddUnique`
/// (`00e7ec00`, `thiscall` on the set, the address of the element; the engine map names it
/// `NiControllerSequence::SetState`).
const ITEM_POOL_0126F9A0: u32 = 0x0126_f9a0;
const POOL_ADD_UNIQUE_00E7EC00: u32 = 0x00e7_ec00;
/// The words and bytes `00e89670` clears, in its order.
const CLEARED_WORDS_00E89670: [u32; 38] = [
    0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50,
    0x54, 0x58, 0x5c, 0x60, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x90, 0x94, 0x98, 0x9c, 0xa0, 0xa4,
    0xb0, 0x34, 0x4, 0x88, 0x64, 0xac,
];
const CLEARED_BYTES_00E89670: [u32; 3] = [0x84, 0xa8, 0xb4];

// Translated from 00e89670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): an item's release to its pool, `cdecl` with one
/// word (the item; no null check). When the item's first byte is set (a pool item): clears the
/// words +0xc to +0x30, +0x3c to +0x60, +0x6c to +0x80, +0x90 to +0xa4, +0xb0, +0x34, +4, +0x88,
/// +0x64 and +0xac and the bytes +0x84, +0xa8, +0xb4, then gives the item back to the pool at
/// `0126f9a0` (`00e7ec00`, with the item in the argument's own stack word). Otherwise nothing.
pub fn fn_00e89670(e: &mut Engine, item: Ptr) {
    let item = item.addr();
    if e.mem.u8(item) == 0 {
        return;
    }
    for offset in CLEARED_WORDS_00E89670 {
        e.mem.set_u32(item + offset, 0);
    }
    for offset in CLEARED_BYTES_00E89670 {
        e.mem.set_u8(item + offset, 0);
    }
    let pool = e.global::<u32>(ITEM_POOL_0126F9A0);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(POOL_ADD_UNIQUE_00E7EC00, &args![pool, slot]);
    });
}

// Translated from 00e891f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem), `thiscall` with no
/// stack words (called from `00e7ecd0`): for each of 8 steps clears the byte at `+0x2c + i`
/// and the byte at `+0x5c + i`, then the words at `+0x34` and `+4`.
pub fn fn_00e891f0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    for i in 0..8u32 {
        e.mem.set_u8(this + 0x2c + i, 0);
        e.mem.set_u8(this + 0x5c + i, 0);
    }
    e.mem.set_u32(this + 0x34, 0);
    e.mem.set_u32(this + 4, 0);
}

// Translated from 00e89220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem), `thiscall` with no
/// stack words (called from `00e7ecd0`): for each of 5 steps clears the bytes at `+0x80 + i`,
/// `+0xa4 + i` and `+0xb0 + i`, then the words at `+0x88`, `+0x64` and `+0xac`.
pub fn fn_00e89220(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    for i in 0..5u32 {
        e.mem.set_u8(this + 0x80 + i, 0);
        e.mem.set_u8(this + 0xa4 + i, 0);
        e.mem.set_u8(this + 0xb0 + i, 0);
    }
    e.mem.set_u32(this + 0x88, 0);
    e.mem.set_u32(this + 0x64, 0);
    e.mem.set_u32(this + 0xac, 0);
}

/// The render-state slot maps the two setters index (16-bit entries): `0126f958` for
/// `00e88fc0` (slots below 8 are kept), `0126f92c` for `00e890c0` (slots below 5).
const STATE_SLOTS_0126F958: u32 = 0x0126_f958;
const SAMPLER_SLOTS_0126F92C: u32 = 0x0126_f92c;

// Translated from 00e88fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), `thiscall` with three words: a state, its
/// value and a byte flag. The state's slot is the 16-bit entry at `0126f958 + 2 * state` (no
/// bounds check); slots 8 and up do nothing. With the flag set: a slot already marked in the
/// bytes at +0x2c only gets the value (words at +0xc); otherwise a slot marked in the bytes at
/// +0x5c moves over (+0x5c byte cleared, +0x2c byte set, value stored, count +0x34 down and
/// count +4 up), and then (in either case) the +0x2c byte is set, the value stored and +4
/// counted up once more. With the flag clear the same with the roles swapped: a slot marked at
/// +0x5c only gets the value (words at +0x3c); otherwise one marked at +0x2c moves over (+0x2c
/// cleared, +0x5c set, value stored, +0x34 up, +4 down), then the +0x5c byte is set, the value
/// stored and +0x34 counted up.
pub fn fn_00e88fc0(e: &mut Engine, this: Ptr, state: u32, value: u32, flag: u8) {
    let base = this.addr();
    let slot = u32::from(
        e.mem
            .u16(STATE_SLOTS_0126F958.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 8 {
        return;
    }
    let add = |e: &mut Engine, offset: u32, delta: u32| {
        let at = base.wrapping_add(offset);
        let counted = e.mem.u32(at).wrapping_add(delta);
        e.mem.set_u32(at, counted);
    };
    let set_byte = |e: &mut Engine, offset: u32, value: u8| {
        e.mem.set_u8(base.wrapping_add(slot + offset), value)
    };
    let byte = |e: &mut Engine, offset: u32| e.mem.u8(base.wrapping_add(slot + offset));
    if flag != 0 {
        let word = base.wrapping_add(0xc + slot * 4);
        if byte(e, 0x2c) != 0 {
            e.mem.set_u32(word, value);
            return;
        }
        if byte(e, 0x5c) != 0 {
            set_byte(e, 0x5c, 0);
            set_byte(e, 0x2c, 1);
            e.mem.set_u32(word, value);
            add(e, 0x34, 0xffff_ffff);
            add(e, 4, 1);
        }
        set_byte(e, 0x2c, 1);
        e.mem.set_u32(word, value);
        add(e, 4, 1);
        return;
    }
    let word = base.wrapping_add(0x3c + slot * 4);
    if byte(e, 0x5c) != 0 {
        e.mem.set_u32(word, value);
        return;
    }
    if byte(e, 0x2c) != 0 {
        set_byte(e, 0x2c, 0);
        set_byte(e, 0x5c, 1);
        e.mem.set_u32(word, value);
        add(e, 0x34, 1);
        add(e, 4, 0xffff_ffff);
    }
    set_byte(e, 0x5c, 1);
    e.mem.set_u32(word, value);
    add(e, 0x34, 1);
}

// Translated from 00e890c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), `thiscall` with four words: a state, its
/// value and two byte flags. The slot is the 16-bit entry at `0126f92c + 2 * state` (no
/// bounds check); slots 5 and up do nothing. With the first flag set: a slot marked in the
/// bytes at +0x80 gets the value (words at +0x6c) and goes on to the second flag's update
/// below; otherwise a slot marked at +0xa4 is unmarked (count +0x88 down, and when its +0xb0
/// byte is set count +0xac down, then that byte cleared), then the +0x80 byte is set, the
/// value stored, +0x64 counted up, +0xac counted up when the second flag is set, and the
/// +0xb0 byte set to the second flag. With the first flag clear the same with +0xa4 (words at
/// +0x90, count +0x88) and +0x80 (count +0x64) swapped. The second flag's update for an
/// already marked slot: a set flag counts +0xac up when the +0xb0 byte was clear, a clear flag
/// counts it down when that byte was set, and the byte becomes the flag.
pub fn fn_00e890c0(e: &mut Engine, this: Ptr, state: u32, value: u32, enable: u8, extra: u8) {
    let base = this.addr();
    let slot = u32::from(
        e.mem
            .u16(SAMPLER_SLOTS_0126F92C.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 5 {
        return;
    }
    let add = |e: &mut Engine, offset: u32, delta: u32| {
        let at = base.wrapping_add(offset);
        let counted = e.mem.u32(at).wrapping_add(delta);
        e.mem.set_u32(at, counted);
    };
    let at = |offset: u32| base.wrapping_add(slot + offset);
    // (marked byte, value word, count) of the side the first flag picks, then the other side.
    let (mine, word, count, other, other_count) = if enable != 0 {
        (0x80u32, 0x6cu32, 0x64u32, 0xa4u32, 0x88u32)
    } else {
        (0xa4, 0x90, 0x88, 0x80, 0x64)
    };
    let word = base.wrapping_add(word + slot * 4);
    if e.mem.u8(at(mine)) != 0 {
        e.mem.set_u32(word, value);
        let extra_byte = at(0xb0);
        if extra != 0 {
            if e.mem.u8(extra_byte) == 0 {
                add(e, 0xac, 1);
            }
        } else if e.mem.u8(extra_byte) != 0 {
            add(e, 0xac, 0xffff_ffff);
        }
        e.mem.set_u8(extra_byte, extra);
        return;
    }
    if e.mem.u8(at(other)) != 0 {
        e.mem.set_u8(at(other), 0);
        add(e, other_count, 0xffff_ffff);
        if e.mem.u8(at(0xb0)) != 0 {
            add(e, 0xac, 0xffff_ffff);
        }
        e.mem.set_u8(at(0xb0), 0);
    }
    e.mem.set_u8(at(mine), 1);
    e.mem.set_u32(word, value);
    add(e, count, 1);
    if extra != 0 {
        add(e, 0xac, 1);
    }
    e.mem.set_u8(at(0xb0), extra);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_95f0, fn_00e895f0()),
        entry!(0x00e8_9670, fn_00e89670(Ptr)),
        entry!(0x00e8_91f0, fn_00e891f0(Ptr)),
        entry!(0x00e8_9220, fn_00e89220(Ptr)),
        entry!(0x00e8_8fc0, fn_00e88fc0(Ptr, u32, u32, u8)),
        entry!(0x00e8_90c0, fn_00e890c0(Ptr, u32, u32, u8, u8)),
        entry!(0x00e8_9060, fn_00e89060(Ptr, u32, Ptr, Ptr) -> bool),
        entry!(0x00e8_9250, fn_00e89250(Ptr, u32) -> u32),
        entry!(0x00e8_92d0, fn_00e892d0(Ptr, u32) -> u32),
        entry!(0x00e8_9350, fn_00e89350(Ptr) -> Ptr),
        entry!(0x00e8_9410, fn_00e89410(Ptr, u32) -> u32),
        entry!(0x00e8_9440, fn_00e89440(Ptr, u32) -> Ptr),
        entry!(0x00e8_9460, fn_00e89460(u32) -> u32),
        entry!(0x00e8_94b0, fn_00e894b0(Ptr, u32)),
        entry!(0x00e8_9560, fn_00e89560(Ptr)),
        entry!(0x00e8_95a0, fn_00e895a0(Ptr)),
        entry!(0x00e8_9630, fn_00e89630() -> u32),
        entry!(0x00e8_9740, fn_00e89740()),
    ]
}

// Translated from 00e89060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), `thiscall` with three words: a render state and
/// the addresses of a value and of a byte. The state's slot is the 16-bit entry at
/// `0126f958 + 2 * state`; for a slot below 8 marked in the bytes at +0x5c the value is its
/// word at +0x3c and the byte 0, else for one marked at +0x2c the value is its word at +0xc and
/// the byte 1; answers whether one was found (nothing written otherwise).
pub fn fn_00e89060(e: &mut Engine, this: Ptr, state: u32, value: Ptr, list: Ptr) -> bool {
    let base = this.addr();
    let slot = e
        .mem
        .u16(STATE_SLOTS_0126F958.wrapping_add(state.wrapping_mul(2)));
    if slot >= 8 {
        return false;
    }
    let slot = u32::from(slot);
    if e.mem.u8(base.wrapping_add(slot + 0x5c)) != 0 {
        let word = e.mem.u32(base.wrapping_add(slot * 4 + 0x3c));
        e.mem.set_u32(value.addr(), word);
        e.mem.set_u8(list.addr(), 0);
        return true;
    }
    if e.mem.u8(base.wrapping_add(slot + 0x2c)) != 0 {
        let word = e.mem.u32(base.wrapping_add(slot * 4 + 0xc));
        e.mem.set_u32(value.addr(), word);
        e.mem.set_u8(list.addr(), 1);
        return true;
    }
    false
}

/// The shader library global (`0126f99c`) whose slots `00e89250` (0xc0) and `00e892d0` (0xcc)
/// call (`thiscall`, four words), and the state numbers for their slots: 16-bit entries at
/// `0126f948` (8 render states) and words at `0126f918` (5 sampler states).
const SHADER_LIBRARY_0126F99C: u32 = 0x0126_f99c;
const RENDER_STATES_0126F948: u32 = 0x0126_f948;
const SAMPLER_STATES_0126F918: u32 = 0x0126_f918;

// Translated from 00e89250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), `thiscall` with one word (a stage). For each of
/// the 8 slots: one marked in the bytes at +0x5c sends its word at +0x3c, else one marked at
/// +0x2c its word at +0xc, to the shader library (`0126f99c`, slot 0xc0: the stage, the state
/// from the 16-bit table at `0126f948`, the value and 0). Answers 0.
pub fn fn_00e89250(e: &mut Engine, this: Ptr, stage: u32) -> u32 {
    let base = this.addr();
    for slot in 0..8u32 {
        let value = if e.mem.u8(base.wrapping_add(slot + 0x5c)) != 0 {
            e.mem.u32(base.wrapping_add(slot * 4 + 0x3c))
        } else if e.mem.u8(base.wrapping_add(slot + 0x2c)) != 0 {
            e.mem.u32(base.wrapping_add(slot * 4 + 0xc))
        } else {
            continue;
        };
        let library = e.mem.u32(SHADER_LIBRARY_0126F99C);
        let state = u32::from(e.mem.u16(RENDER_STATES_0126F948 + slot * 2));
        e.vcall(library, 0xc0, &args![stage, state, value, 0u32]);
    }
    0
}

// Translated from 00e892d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), `thiscall` with one word (a stage). For each of
/// the 5 slots: one marked in the bytes at +0xa4 sends its word at +0x90, else one marked at
/// +0x80 its word at +0x6c, to the shader library (`0126f99c`, slot 0xcc: the stage, the state
/// word from `0126f918`, the value and 0). Answers 0.
pub fn fn_00e892d0(e: &mut Engine, this: Ptr, stage: u32) -> u32 {
    let base = this.addr();
    for slot in 0..5u32 {
        let value = if e.mem.u8(base.wrapping_add(slot + 0xa4)) != 0 {
            e.mem.u32(base.wrapping_add(slot * 4 + 0x90))
        } else if e.mem.u8(base.wrapping_add(slot + 0x80)) != 0 {
            e.mem.u32(base.wrapping_add(slot * 4 + 0x6c))
        } else {
            continue;
        };
        let library = e.mem.u32(SHADER_LIBRARY_0126F99C);
        let state = e.mem.u32(SAMPLER_STATES_0126F918 + slot * 4);
        e.vcall(library, 0xcc, &args![stage, state, value, 0u32]);
    }
    0
}

/// The words `00e89350` clears after its first byte, in its order (the bytes +0x84, +0xa8 and
/// +0xb4 are cleared in their places in the list).
const CONSTRUCT_WORDS_00E89350: [u32; 9] = [4, 8, 0x34, 0x38, 0x64, 0x68, 0x88, 0x8c, 0xac];

// Translated from 00e89350 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor the item pool's `vector constructor iterator` runs on each 0xb8-byte item
/// (code Ghidra made no function of), `thiscall`, no stack words: clears the byte +0, the words
/// +4, +8, +0x34, +0x38, +0x64, +0x68, +0x88, +0x8c, +0xac, +0xc to +0x30, +0x3c to +0x60, +0x6c
/// to +0x80, the byte +0x84, the words +0x90 to +0xa4, the byte +0xa8, the word +0xb0 and the
/// byte +0xb4. Answers `this`.
pub fn fn_00e89350(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u8(base, 0);
    for offset in CONSTRUCT_WORDS_00E89350 {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for offset in (0xcu32..=0x30)
        .step_by(4)
        .chain((0x3c..=0x60).step_by(4))
        .chain((0x6c..=0x80).step_by(4))
    {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u8(base.wrapping_add(0x84), 0);
    for offset in (0x90u32..=0xa4).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u8(base.wrapping_add(0xa8), 0);
    e.mem.set_u32(base.wrapping_add(0xb0), 0);
    e.mem.set_u8(base.wrapping_add(0xb4), 0);
    this
}

/// The global word `00e89410` compares the stage with (`011bf558`).
const STREAM_LIMIT_011BF558: u32 = 0x011b_f558;

// Translated from 00e89410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), `thiscall` with one word (a stage): `00e89250`
/// for a stage below the word at `011bf558`, then `00e892d0`; answers the first one's result
/// when not 0, else the second's.
pub fn fn_00e89410(e: &mut Engine, this: Ptr, stage: u32) -> u32 {
    let mut first = 0;
    if stage < e.mem.u32(STREAM_LIMIT_011BF558) {
        first = fn_00e89250(e, this, stage);
    }
    let second = fn_00e892d0(e, this, stage);
    if first != 0 {
        first
    } else {
        second
    }
}

/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, the block), the Ni array
/// free `00aa1490` (`cdecl`: block, element size), the free `00aa10f0` (`cdecl`, one word), the
/// Ni array allocation `00aa1420` (`cdecl`, the byte count), the compiler's `vector constructor
/// iterator` (`00401050`, `stdcall`: array, element size, count, constructor) and
/// `NiTSet<..>::Realloc` (`00a311e0`, `thiscall` on the set, the new capacity).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const NI_ARRAY_FREE_00AA1490: u32 = 0x00aa_1490;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const NI_ARRAY_ALLOCATE_00AA1420: u32 = 0x00aa_1420;
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const SET_REALLOC_00A311E0: u32 = 0x00a3_11e0;

// Translated from 00e89560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): the destructor of the item pool's 0xc-byte
/// block, `thiscall`, no stack words: its items (array at +0, the count word before it) freed
/// (`00aa1490` with 0xb8) when set, then the next block of the chain (+8), when there is one,
/// destroyed the same way and given back to the memory manager.
pub fn fn_00e89560(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let items = e.mem.u32(base);
    if items != 0 {
        e.call(
            NI_ARRAY_FREE_00AA1490,
            &args![items.wrapping_sub(4), 0xb8u32],
        );
    }
    let next = e.mem.u32(base.wrapping_add(8));
    if next != 0 {
        fn_00e89560(e, Ptr::new(next));
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, next]);
    }
}

// Translated from 00e89440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): the scalar deleting destructor of the item
/// pool's block, `thiscall` with the flags: `00e89560`, then with bit 0 set the block given back
/// to the memory manager. Answers `this`.
pub fn fn_00e89440(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e89560(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00e89460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): `operator new[]` of `count` 0xb8-byte items
/// after a count word, `cdecl` with the count: `count * 0xb8 + 4` bytes from `00aa1420` (all
/// ones when the product or the sum overflows); null when that fails, otherwise the count is
/// stored, the items built (`vector constructor iterator` with `00e89350`) and the address
/// after the count answered.
pub fn fn_00e89460(e: &mut Engine, count: u32) -> u32 {
    let product = u64::from(count) * 0xb8;
    let product = if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    };
    let total = product.checked_add(4).unwrap_or(u32::MAX);
    let block = e.call(NI_ARRAY_ALLOCATE_00AA1420, &args![total]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT_00401050,
        &args![block + 4, 0xb8u32, count, 0x00e8_9350u32],
    );
    block + 4
}

// Translated from 00e894b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): the item pool's grow, `thiscall` on the pool
/// with one stack word (the growth size). A new 0xc-byte block from the memory manager (null
/// when that fails): +4 the growth size, +8 cleared, +0 the items `00e89460` makes (0 for a
/// growth of 0). Each new item (its address while the index is below the block's +4, else 0;
/// the block's words read even when it is null) is appended to the pool's set (array +0,
/// capacity +4, count +8; grown through `00a311e0` to twice its capacity, 1 when 0, when full);
/// then the block is linked in front of the pool's chain at +0xc (its +8 takes the old head).
pub fn fn_00e894b0(e: &mut Engine, this: Ptr, growth: u32) {
    let pool = this.addr();
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0xcu32],
        )
        .u32();
    if block != 0 {
        e.mem.set_u32(block + 4, growth);
        e.mem.set_u32(block + 8, 0);
        if growth != 0 {
            let items = fn_00e89460(e, growth);
            e.mem.set_u32(block, items);
        } else {
            e.mem.set_u32(block, 0);
        }
    }
    let mut offset = 0u32;
    for index in 0..growth {
        let item = if index < e.mem.u32(block.wrapping_add(4)) {
            e.mem.u32(block).wrapping_add(offset)
        } else {
            0
        };
        let capacity = e.mem.u32(pool.wrapping_add(4));
        if e.mem.u32(pool.wrapping_add(8)) == capacity {
            let grown = if capacity != 0 {
                capacity.wrapping_add(capacity)
            } else {
                1
            };
            e.call(SET_REALLOC_00A311E0, &args![pool, grown]);
        }
        let used = e.mem.u32(pool.wrapping_add(8));
        let elements = e.mem.u32(pool);
        e.mem
            .set_u32(elements.wrapping_add(used.wrapping_mul(4)), item);
        e.mem.set_u32(pool.wrapping_add(8), used.wrapping_add(1));
        offset = offset.wrapping_add(0xb8);
    }
    let chain = e.mem.u32(pool.wrapping_add(0xc));
    e.mem.set_u32(block.wrapping_add(8), chain);
    e.mem.set_u32(pool.wrapping_add(0xc), block);
}

// Translated from 00e895a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): the item pool's destructor, `thiscall`, no stack
/// words. The count (+8) cleared; the chain's first block (+0xc), when there is one, destroyed
/// as `00e89560` inlines it (its items freed, a next block deleted through `00e89440` with 1)
/// and given back to the memory manager; then the set's array (+0) freed (`00aa10f0`).
pub fn fn_00e895a0(e: &mut Engine, this: Ptr) {
    let pool = this.addr();
    let chain = e.mem.u32(pool.wrapping_add(0xc));
    e.mem.set_u32(pool.wrapping_add(8), 0);
    if chain != 0 {
        let items = e.mem.u32(chain);
        if items != 0 {
            e.call(
                NI_ARRAY_FREE_00AA1490,
                &args![items.wrapping_sub(4), 0xb8u32],
            );
        }
        let next = e.mem.u32(chain.wrapping_add(8));
        if next != 0 {
            fn_00e89440(e, Ptr::new(next), 1);
        }
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, chain]);
    }
    let elements = e.mem.u32(pool);
    e.call(FREE_00AA10F0, &args![elements]);
}

// Translated from 00e89630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): a new item from the item pool (`0126f9a0`), no
/// arguments. When the pool is empty it is grown (`00e894b0` with its growth size at +0x10)
/// and that size doubled; then the first element is taken (the count at +8 goes down and the
/// last element moves into the first place), its first byte set when clear, and it is
/// answered.
pub fn fn_00e89630(e: &mut Engine) -> u32 {
    let pool = e.mem.u32(ITEM_POOL_0126F9A0);
    if e.mem.u32(pool.wrapping_add(8)) == 0 {
        let growth = e.mem.u32(pool.wrapping_add(0x10));
        fn_00e894b0(e, Ptr::new(pool), growth);
        let growth = e.mem.u32(pool.wrapping_add(0x10));
        e.mem
            .set_u32(pool.wrapping_add(0x10), growth.wrapping_add(growth));
    }
    let elements = e.mem.u32(pool);
    let item = e.mem.u32(elements);
    let count = e.mem.u32(pool.wrapping_add(8)).wrapping_sub(1);
    e.mem.set_u32(pool.wrapping_add(8), count);
    let last = e.mem.u32(elements.wrapping_add(count.wrapping_mul(4)));
    e.mem.set_u32(elements, last);
    if e.mem.u8(item) == 0 {
        e.mem.set_u8(item, 1);
    }
    item
}

// Translated from 00e89740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), no arguments: the item pool at `0126f9a0`, when
/// there is one, destroyed (`00e895a0`) and given back to the memory manager.
pub fn fn_00e89740(e: &mut Engine) {
    let pool = e.mem.u32(ITEM_POOL_0126F9A0);
    if pool != 0 {
        fn_00e895a0(e, Ptr::new(pool));
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, pool]);
    }
}

#[cfg(test)]
mod clear_tests {
    use super::*;

    /// A 0xc0-byte block of 0xaa; the call clears exactly the bytes in `cleared`.
    fn check(addr: u32, cleared: &[std::ops::Range<u32>]) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        e.mem.write(this, &[0xaa; 0xc0]);
        e.call(addr, &args![this]);
        for offset in 0..0xc0u32 {
            let want = if cleared.iter().any(|r| r.contains(&offset)) {
                0
            } else {
                0xaa
            };
            assert_eq!(e.mem.u8(this + offset), want, "offset {offset:#x}");
        }
    }

    #[test]
    fn fn_00e891f0_clears_its_bytes_and_words() {
        check(0x00e8_91f0, &[0x2c..0x38, 0x5c..0x64, 4..8]);
    }

    #[test]
    fn fn_00e89220_clears_its_bytes_and_words() {
        check(
            0x00e8_9220,
            &[
                0x80..0x85,
                0xa4..0xa9,
                0xb0..0xb5,
                0x88..0x8c,
                0x64..0x68,
                0xac..0xb0,
            ],
        );
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_or_stores_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.call(0x00e8_95f0, &args![]);
        assert_eq!(e.mem.u32(0x0126_f9a0), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e8_95f0, &args![]);
        assert_eq!(e.mem.u32(0x0126_f9a0), 0);
    }
}

#[cfg(test)]
mod item_release_tests {
    use super::*;

    #[test]
    fn fn_00e89670_clears_a_pool_item_and_gives_it_back() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.set_global(ITEM_POOL_0126F9A0, 0x0e00_0000u32);
        // The double notes the element its slot holds.
        let seen = e.mem.alloc(4);
        e.register_double(POOL_ADD_UNIQUE_00E7EC00, move |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(seen, item);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let item = e.mem.alloc(0xb8);
        e.mem.write(item, &[0xcc; 0xb8]);
        e.call(0x00e8_9670, &args![item]);
        let mut expected = vec![0xccu8; 0xb8];
        for offset in CLEARED_WORDS_00E89670.iter() {
            expected[*offset as usize..*offset as usize + 4].copy_from_slice(&[0; 4]);
        }
        for offset in CLEARED_BYTES_00E89670 {
            expected[offset as usize] = 0;
        }
        assert_eq!(e.mem.bytes(item, 0xb8), expected);
        assert_eq!(e.mem.u32(seen), item);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, POOL_ADD_UNIQUE_00E7EC00);
        assert_eq!(log[1].1[0], 0x0e00_0000);
        // An item whose first byte is clear is left alone.
        let other = e.mem.alloc(0xb8);
        e.mem.write(other + 4, &[0xcc; 4]);
        e.call(0x00e8_9670, &args![other]);
        assert_eq!(e.mem.u32(other + 4), 0xcccc_cccc);
    }
}

#[cfg(test)]
mod b000x_tests {
    use super::*;

    /// The slot maps mapped: state 5 is slot 2 in both, state 6 slot 8 / 5 (ignored).
    fn setup(e: &mut Engine) -> u32 {
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u16(STATE_SLOTS_0126F958 + 10, 2);
        e.mem.set_u16(STATE_SLOTS_0126F958 + 12, 8);
        e.mem.set_u16(SAMPLER_SLOTS_0126F92C + 10, 2);
        e.mem.set_u16(SAMPLER_SLOTS_0126F92C + 12, 5);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 4, 10);
        e.mem.set_u32(this + 0x34, 20);
        e.mem.set_u32(this + 0x64, 30);
        e.mem.set_u32(this + 0x88, 40);
        e.mem.set_u32(this + 0xac, 50);
        this
    }

    fn set_state(e: &mut Engine, this: u32, state: u32, value: u32, flag: u32) {
        e.call(0x00e8_8fc0, &args![this, state, value, flag]);
    }

    /// (+4, +0x34, byte +0x2e, byte +0x5e, word +0x14, word +0x44).
    fn state_view(e: &mut Engine, this: u32) -> (u32, u32, u8, u8, u32, u32) {
        (
            e.mem.u32(this + 4),
            e.mem.u32(this + 0x34),
            e.mem.u8(this + 0x2e),
            e.mem.u8(this + 0x5e),
            e.mem.u32(this + 0x14),
            e.mem.u32(this + 0x44),
        )
    }

    #[test]
    fn fn_00e88fc0_marks_and_moves_slots() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        // Slot 8: nothing.
        set_state(&mut e, this, 6, 7, 1);
        assert_eq!(state_view(&mut e, this), (10, 20, 0, 0, 0, 0));
        // Flag set, unmarked: marked at +0x2c, value at +0xc + 8, +4 up.
        set_state(&mut e, this, 5, 0x77, 1);
        assert_eq!(state_view(&mut e, this), (11, 20, 1, 0, 0x77, 0));
        // Flag set, marked: only the value.
        set_state(&mut e, this, 5, 0x78, 1);
        assert_eq!(state_view(&mut e, this), (11, 20, 1, 0, 0x78, 0));
        // Flag clear, marked at +0x2c: moved (+0x34 up, +4 down), then +0x34 up again.
        set_state(&mut e, this, 5, 0x79, 0);
        assert_eq!(state_view(&mut e, this), (10, 22, 0, 1, 0x78, 0x79));
        // Flag clear, marked at +0x5c: only the value.
        set_state(&mut e, this, 5, 0x7a, 0);
        assert_eq!(state_view(&mut e, this), (10, 22, 0, 1, 0x78, 0x7a));
        // Flag set, marked at +0x5c: moved (+0x34 down, +4 up), then +4 up again.
        set_state(&mut e, this, 5, 0x7b, 1);
        assert_eq!(state_view(&mut e, this), (12, 21, 1, 0, 0x7b, 0x7a));
    }

    #[test]
    fn fn_00e88fc0_flag_clear_unmarked() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        set_state(&mut e, this, 5, 0x55, 0x100);
        assert_eq!(state_view(&mut e, this), (10, 21, 0, 1, 0, 0x55));
    }

    fn set_sampler(e: &mut Engine, this: u32, value: u32, enable: u32, extra: u32) {
        e.call(0x00e8_90c0, &args![this, 5u32, value, enable, extra]);
    }

    /// (+0x64, +0x88, +0xac, bytes +0x82, +0xa6, +0xb2, words +0x74, +0x98).
    type SamplerView = (u32, u32, u32, u8, u8, u8, u32, u32);

    fn sampler_view(e: &mut Engine, this: u32) -> SamplerView {
        (
            e.mem.u32(this + 0x64),
            e.mem.u32(this + 0x88),
            e.mem.u32(this + 0xac),
            e.mem.u8(this + 0x82),
            e.mem.u8(this + 0xa6),
            e.mem.u8(this + 0xb2),
            e.mem.u32(this + 0x74),
            e.mem.u32(this + 0x98),
        )
    }

    #[test]
    fn fn_00e890c0_marks_and_moves_slots() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        // Slot 5: nothing.
        e.call(0x00e8_90c0, &args![this, 6u32, 1u32, 1u32, 1u32]);
        assert_eq!(sampler_view(&mut e, this), (30, 40, 50, 0, 0, 0, 0, 0));
        // Enable, unmarked, extra: +0x80 marked, +0x64 up, +0xac up, +0xb0 byte set.
        set_sampler(&mut e, this, 0x11, 1, 1);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 51, 1, 0, 1, 0x11, 0));
        // Enable, marked, extra again: value only (the +0xb0 byte was set).
        set_sampler(&mut e, this, 0x12, 1, 1);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 51, 1, 0, 1, 0x12, 0));
        // Enable, marked, no extra: +0xac down, byte cleared.
        set_sampler(&mut e, this, 0x13, 1, 0);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 50, 1, 0, 0, 0x13, 0));
        // No extra again: nothing counted.
        set_sampler(&mut e, this, 0x13, 1, 0);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 50, 1, 0, 0, 0x13, 0));
        // Enable, marked, extra with the byte clear: +0xac up.
        set_sampler(&mut e, this, 0x14, 1, 2);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 51, 1, 0, 2, 0x14, 0));
        // Disable, +0x80 marked with the +0xb0 byte set: unmarked (+0x64 down, +0xac down),
        // +0xa4 marked, +0x88 up; no extra.
        set_sampler(&mut e, this, 0x21, 0, 0);
        assert_eq!(
            sampler_view(&mut e, this),
            (30, 41, 50, 0, 1, 0, 0x14, 0x21)
        );
        // Disable, marked: value; extra counts up.
        set_sampler(&mut e, this, 0x22, 0, 1);
        assert_eq!(
            sampler_view(&mut e, this),
            (30, 41, 51, 0, 1, 1, 0x14, 0x22)
        );
        // Enable, +0xa4 marked with the byte set: +0x88 down, +0xac down, then +0x80
        // marked, +0x64 up, and the extra flag set counts +0xac up again.
        set_sampler(&mut e, this, 0x31, 1, 1);
        assert_eq!(
            sampler_view(&mut e, this),
            (31, 40, 51, 1, 0, 1, 0x31, 0x22)
        );
    }

    #[test]
    fn fn_00e890c0_disable_unmarked() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        set_sampler(&mut e, this, 0x41, 0x100, 0);
        assert_eq!(sampler_view(&mut e, this), (30, 41, 50, 0, 1, 0, 0, 0x41));
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const RENDER_STATE: u32 = 0x0e80_00c0;
    const SAMPLER_STATE: u32 = 0x0e80_00cc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011b_f000, 0x1000);
        for no_op in [
            MEMORY_MANAGER_FREE,
            NI_ARRAY_FREE_00AA1490,
            FREE_00AA10F0,
            VECTOR_CONSTRUCT_00401050,
            RENDER_STATE,
            SAMPLER_STATE,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_ARRAY_ALLOCATE_00AA1420, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(SET_REALLOC_00A311E0, |e, a| {
            let array = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0]);
            let used = e.mem.u32(a[0] + 8);
            let bytes = e.mem.bytes(old, 4 * used);
            e.mem.write(array, &bytes);
            e.mem.set_u32(a[0], array);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let mut slots = vec![0u32; 0x34];
        slots[0xc0 / 4] = RENDER_STATE;
        slots[0xcc / 4] = SAMPLER_STATE;
        e.put_vtable(0x0e81_0000, &slots);
        let library = e.mem.alloc(4);
        e.mem.set_u32(library, 0x0e81_0000);
        e.mem.set_u32(SHADER_LIBRARY_0126F99C, library);
        e
    }

    #[test]
    fn fn_00e89060_finds_the_state_in_either_list() {
        let mut e = engine();
        e.mem.set_u16(STATE_SLOTS_0126F958 + 2 * 7, 3);
        e.mem.set_u16(STATE_SLOTS_0126F958 + 2 * 8, 8);
        let this = e.mem.alloc(0xb8);
        let (value, list) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u8(list, 9);
        assert!(!e.call(0x00e8_9060, &args![this, 7u32, value, list]).bool());
        assert_eq!(e.mem.u8(list), 9);
        e.mem.set_u8(this + 0x2c + 3, 1);
        e.mem.set_u32(this + 0xc + 12, 0x11);
        assert!(e.call(0x00e8_9060, &args![this, 7u32, value, list]).bool());
        assert_eq!((e.mem.u32(value), e.mem.u8(list)), (0x11, 1));
        e.mem.set_u8(this + 0x5c + 3, 1);
        e.mem.set_u32(this + 0x3c + 12, 0x22);
        assert!(e.call(0x00e8_9060, &args![this, 7u32, value, list]).bool());
        assert_eq!((e.mem.u32(value), e.mem.u8(list)), (0x22, 0));
        // Slot 8 and up: nothing.
        assert!(!e.call(0x00e8_9060, &args![this, 8u32, value, list]).bool());
    }

    #[test]
    fn fn_00e89410_sends_the_marked_states() {
        let mut e = engine();
        let library = e.mem.u32(SHADER_LIBRARY_0126F99C);
        e.mem.set_u16(RENDER_STATES_0126F948 + 2, 0x0a0b);
        e.mem.set_u16(RENDER_STATES_0126F948 + 10, 0x0c0d);
        e.mem.set_u32(SAMPLER_STATES_0126F918 + 16, 0x0e0f);
        e.mem.set_u32(SAMPLER_STATES_0126F918, 0x1011);
        let this = e.mem.alloc(0xb8);
        // Render slot 1 in the first list, slot 5 in the second (its +0x3c word wins).
        e.mem.set_u8(this + 0x2c + 1, 1);
        e.mem.set_u32(this + 0xc + 4, 0x101);
        e.mem.set_u8(this + 0x2c + 5, 1);
        e.mem.set_u8(this + 0x5c + 5, 1);
        e.mem.set_u32(this + 0x3c + 20, 0x505);
        // Sampler slot 0 in the first list, slot 4 in the second.
        e.mem.set_u8(this + 0x80, 1);
        e.mem.set_u32(this + 0x6c, 0x600);
        e.mem.set_u8(this + 0xa4 + 4, 1);
        e.mem.set_u32(this + 0x90 + 16, 0x704);
        e.mem.set_u32(STREAM_LIMIT_011BF558, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_9410, &args![this, 3u32]).u32(), 0);
        assert_eq!(
            calls(&e, RENDER_STATE),
            vec![
                vec![library, 3, 0x0a0b, 0x101, 0],
                vec![library, 3, 0x0c0d, 0x505, 0]
            ]
        );
        assert_eq!(
            calls(&e, SAMPLER_STATE),
            vec![
                vec![library, 3, 0x1011, 0x600, 0],
                vec![library, 3, 0x0e0f, 0x704, 0]
            ]
        );
        // A stage at the limit: only the sampler states.
        e.call_log = Some(vec![]);
        e.call(0x00e8_9410, &args![this, 4u32]);
        assert!(calls(&e, RENDER_STATE).is_empty());
        assert_eq!(calls(&e, SAMPLER_STATE).len(), 2);
    }

    #[test]
    fn fn_00e89350_clears_the_item() {
        let mut e = engine();
        let this = e.mem.alloc(0xbc);
        e.mem.write(this, &[0xee; 0xbc]);
        assert_eq!(e.call(0x00e8_9350, &args![this]).u32(), this);
        let mut expected = vec![0u8; 0xb8];
        for gap in [
            1usize, 2, 3, 0x85, 0x86, 0x87, 0xa9, 0xaa, 0xab, 0xb5, 0xb6, 0xb7,
        ] {
            expected[gap] = 0xee;
        }
        assert_eq!(e.mem.bytes(this, 0xb8), expected);
        assert_eq!(e.mem.u8(this + 0xb8), 0xee);
    }

    #[test]
    fn fn_00e89460_makes_the_items() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let items = e.call(0x00e8_9460, &args![2u32]).u32();
        assert_eq!(
            calls(&e, NI_ARRAY_ALLOCATE_00AA1420),
            vec![vec![2 * 0xb8 + 4]]
        );
        assert_eq!(e.mem.u32(items - 4), 2);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_00401050),
            vec![vec![items, 0xb8, 2, 0x00e8_9350]]
        );
        // An overflowing count asks for all ones; a failed allocation answers 0.
        e.register(NI_ARRAY_ALLOCATE_00AA1420, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_9460, &args![0x0200_0000u32]).u32(), 0);
        assert_eq!(calls(&e, NI_ARRAY_ALLOCATE_00AA1420), vec![vec![u32::MAX]]);
    }

    #[test]
    fn pool_grows_and_hands_out_items() {
        let mut e = engine();
        let pool = e.mem.alloc(0x18);
        e.mem.set_u32(pool + 0x10, 2);
        e.mem.set_u32(ITEM_POOL_0126F9A0, pool);
        let first = e.call(0x00e8_9630, &args![]).u32();
        let block = e.mem.u32(pool + 0xc);
        let items = e.mem.u32(block);
        assert_eq!(e.mem.u32(block + 4), 2);
        assert_eq!(first, items);
        assert_eq!(e.mem.u8(first), 1);
        assert_eq!(e.mem.u32(pool + 0x10), 4);
        assert_eq!(e.mem.u32(pool + 8), 1);
        // The last item moved into the first place.
        assert_eq!(e.mem.u32(e.mem.u32(pool)), items + 0xb8);
        let second = e.call(0x00e8_9630, &args![]).u32();
        assert_eq!(second, items + 0xb8);
        assert_eq!(e.mem.u32(pool + 8), 0);
        // Destroying the pool frees the items, the block and the set.
        let elements = e.mem.u32(pool);
        e.call_log = Some(vec![]);
        e.call(0x00e8_9740, &args![]);
        assert_eq!(
            calls(&e, NI_ARRAY_FREE_00AA1490),
            vec![vec![items - 4, 0xb8]]
        );
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![elements]]);
        let freed: Vec<u32> = calls(&e, MEMORY_MANAGER_FREE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(freed, vec![block, pool]);
    }

    #[test]
    fn item_block_destructors_free_the_chain() {
        let mut e = engine();
        let (first, second) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.mem.set_u32(first, 0x1004);
        e.mem.set_u32(first + 8, second);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_9440, &args![first, 1u32]).u32(), first);
        assert_eq!(calls(&e, NI_ARRAY_FREE_00AA1490), vec![vec![0x1000, 0xb8]]);
        let freed: Vec<u32> = calls(&e, MEMORY_MANAGER_FREE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(freed, vec![second, first]);
        // The pool destructor inlines the first block's.
        let pool = e.mem.alloc(0x18);
        e.mem.set_u32(pool, 0x2000);
        e.mem.set_u32(pool + 8, 3);
        e.mem.set_u32(pool + 0xc, first);
        e.call_log = Some(vec![]);
        e.call(0x00e8_95a0, &args![pool]);
        assert_eq!(e.mem.u32(pool + 8), 0);
        let freed: Vec<u32> = calls(&e, MEMORY_MANAGER_FREE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(freed, vec![second, first]);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x2000]]);
        // No pool: nothing.
        e.mem.set_u32(ITEM_POOL_0126F9A0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_9740, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
