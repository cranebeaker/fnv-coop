//! `NiAnimation/run_00a26d70` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

// Translated from 00a26df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two stack words): reads a 4-byte value from the
/// stream's input stream into `destination`. Calls the input stream's read
/// function as `function(input, destination, 4, &count, 1)` with `count` a local set
/// to 4, and adds the byte count it returns to the input stream's position.
pub fn fn_00a26df0(e: &mut Engine, stream: Ptr<NiStream>, destination: Ptr) {
    let input = e.get(stream, NiStream::m_pkIstr);
    let input = Ptr::<NiBinaryStream>::new(input);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(function, &args![input, destination, 4u32, count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

// Translated from 00a26e20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3654` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a26e20(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3654) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d20, 0x0);
    e.mem.set_u32(0x011f_3c00, 0x0);
    e.mem.set_u32(0x011f_3b70, 0x0);
    e.mem.set_u32(0x011f_3810, 0x0);
    e.mem.set_u8(0x011f_3654, 0x1);
    e.mem.set_u32(0x011f_3db0, 0xe7cd40);
    e.mem.set_u32(0x011f_3a50, 0xa29680);
    e.mem.set_u32(0x011f_39c0, 0xa29680);
    e.mem.set_u32(0x011f_3930, 0xe7cd40);
    e.mem.set_u32(0x011f_38a0, 0xa29680);
    e.mem.set_u8(0x011f_3758, 0x8);
    e.mem.set_u32(0x011f_3c90, 0xa29680);
    e.mem.set_u32(0x011f_3ae0, 0xa26b00);
    e.mem.set_u32(0x011f_3780, 0xa29680);
    1
}

// Translated from 00a273a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3655` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a273a0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3655) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d28, 0x0);
    e.mem.set_u32(0x011f_3c08, 0x0);
    e.mem.set_u32(0x011f_3b78, 0x0);
    e.mem.set_u8(0x011f_3655, 0x1);
    e.mem.set_u32(0x011f_3db8, 0xa27250);
    e.mem.set_u32(0x011f_3a58, 0xa27320);
    e.mem.set_u32(0x011f_39c8, 0xa26f80);
    e.mem.set_u32(0x011f_3938, 0xa27040);
    e.mem.set_u32(0x011f_38a8, 0xa2a600);
    e.mem.set_u8(0x011f_375a, 0x10);
    e.mem.set_u32(0x011f_3c98, 0xa26fe0);
    e.mem.set_u32(0x011f_3ae8, 0xa26f30);
    e.mem.set_u32(0x011f_3788, 0xa29680);
    e.mem.set_u32(0x011f_3818, 0xa27080);
    1
}

/// `00a3da20` (`thiscall` on the key, the stream): reads the key's 4-byte time.
const READ_KEY_TIME: u32 = 0x00a3_da20;
/// `NiColor::LoadBinary` (`thiscall`, the stream): three floats read in place.
const READ_THREE_FLOATS: u32 = 0x00a5_9be0;
/// `00a42070` (`cdecl`, five floats; answers in `ST0`): the interpolation of two keys'
/// values and tangents at a time.
const INTERPOLATE_VALUE: u32 = 0x00a4_2070;
/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

/// Reads 4 bytes from the stream's input into `destination`, the byte-count word at `count`
/// set to 4 first.
fn read_word(e: &mut Engine, stream: Ptr<NiStream>, destination: u32, count: u32) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

// Translated from 00a26eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a key, the stream): the key's time (`00a3da20`),
/// then its 4-byte value at +4 (the stream argument's stack word is the byte-count word).
pub fn fn_00a26eb0(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(READ_KEY_TIME, &args![this, stream]);
    e.with_stack(4, |e, count| {
        read_word(e, stream, this.addr() + 4, count.addr());
    });
}

// Translated from 00a26fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (time, first key, second key, result)): stores at
/// `result` the float `00a42070` answers for `time`, the first key's floats at +4 and +0xc
/// and the second key's floats at +4 and +8 (each copied through a `float` stack word).
pub fn fn_00a26fe0(e: &mut Engine, time: f32, first: Ptr, second: Ptr, result: Ptr) {
    let copy = |e: &Engine, at: u32| x87_store32(x87_load32(e.mem.u32(at)));
    let second_8 = copy(e, second.addr() + 8);
    let second_4 = copy(e, second.addr() + 4);
    let first_c = copy(e, first.addr() + 0xc);
    let first_4 = copy(e, first.addr() + 4);
    let words = [
        x87_store32(x87_load32(time.to_bits())),
        x87_store32(x87_load32(first_4)),
        x87_store32(x87_load32(first_c)),
        x87_store32(x87_load32(second_4)),
        x87_store32(x87_load32(second_8)),
    ];
    let value = e.call(INTERPOLATE_VALUE, &words).f64();
    e.mem.set_u32(result.addr(), x87_store32(value));
}

// Translated from 00a27250 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (stream, count)):
/// makes `count` keys of 16 bytes from the memory manager (`count * 16 + 4` bytes,
/// saturating, the count in the first word; null when the allocation fails) and reads each:
/// its time and value (`00a26eb0`), then two more 4-byte floats at +8 and +0xc. Answers the
/// keys.
pub fn fn_00a27250(e: &mut Engine, stream: Ptr<NiStream>, count: u32) -> u32 {
    let bytes = count
        .checked_mul(0x10)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    let keys = if block != 0 {
        e.mem.set_u32(block, count);
        block + 4
    } else {
        0
    };
    e.with_stack(4, |e, local| {
        let mut key = keys;
        for _ in 0..count {
            fn_00a26eb0(e, Ptr::new(key), stream);
            read_word(e, stream, key.wrapping_add(8), local.addr());
            read_word(e, stream, key.wrapping_add(0xc), local.addr());
            key = key.wrapping_add(0x10);
        }
    });
    keys
}

// Translated from 00a27430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a key, the stream): the key's time (`00a3da20`),
/// then three floats at +4 (`00a59be0`).
pub fn fn_00a27430(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(READ_KEY_TIME, &args![this, stream]);
    e.call(READ_THREE_FLOATS, &args![this.addr() + 4, stream]);
}

/// `NiFloatKey::GenInterp` (`cdecl`, (time, keys, type, count, &last index, size); answers in
/// `ST0`).
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// The float a key list without keys answers (`011f3650`).
const NO_KEYS_VALUE: u32 = 0x011f_3650;
/// `NiStreamSaveBinary<float>` (`00a3da50`, `cdecl`, (stream, key)): writes the key's time.
const WRITE_KEY_TIME: u32 = 0x00a3_da50;
/// `NiColor::SaveBinary` (`00a69780`, `thiscall`, the stream): writes three floats.
const WRITE_THREE_FLOATS: u32 = 0x00a6_9780;
/// `00a26b00` (`cdecl`, two keys): whether two keys' times and values are equal.
const KEYS_EQUAL: u32 = 0x00a2_6b00;
/// `00a3d5d0` (`cdecl`, (time, keys, count, &index, size)): finds where a key at `time`
/// goes (the index of the first key not before it); false when a key has that time.
const FIND_INSERT_INDEX: u32 = 0x00a3_d5d0;
/// `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `00aa4060` (`thiscall` on the memory manager, the block): frees.
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `00a420e0` (`cdecl`, ten words): the tangents around a new key (the previous key's value,
/// time and out tangent pointer, the next key's value, time and in tangent pointer, the new
/// key's time and value and its in and out tangent pointers).
const NEW_KEY_TANGENTS: u32 = 0x00a4_20e0;
/// The three words a zero vector key value is copied from (`011f426c`).
const ZERO_POINT: u32 = 0x011f_426c;

/// Writes 4 bytes from `source` to the stream's output, the byte-count word at `count`
/// set to 4 first.
fn write_word(e: &mut Engine, stream: Ptr<NiStream>, source: u32, count: u32) {
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let function = e.get(output, NiBinaryStream::m_pfnWrite);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![output, source, 4u32, count, 1u32])
        .u32();
    let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        output,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// The compiler's array new of `count` keys of `size` bytes from the memory manager: `count *
/// size + 4` bytes (each step all ones on overflow), the count in the first word; answers the
/// keys after it, or null.
fn allocate_keys(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = count
        .checked_mul(size)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    if block != 0 {
        e.mem.set_u32(block, count);
        block + 4
    } else {
        0
    }
}

// Translated from 00a26d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (time, keys, type, count, size); answers in `ST0`): the
/// float keys' value at `time`. No keys: the float at `011f3650`. Before the first key's time:
/// the first key's value (+4); after the time of the last key (at `(count - 1) * size`, the
/// size's low byte): the last key's value; otherwise `NiFloatKey::GenInterp` (`00a26b40`),
/// the count's stack word (set to 0) as its last-index word.
pub fn fn_00a26d70(e: &mut Engine, time: f32, keys: Ptr, kind: u32, count: u32, size: u32) -> f64 {
    if count == 0 {
        return x87_load32(e.mem.u32(NO_KEYS_VALUE));
    }
    let t = x87_load32(time.to_bits());
    if x87_load32(e.mem.u32(keys.addr())) > t {
        return x87_load32(e.mem.u32(keys.addr() + 4));
    }
    let last = count.wrapping_sub(1).wrapping_mul(size & 0xff);
    let last = keys.addr().wrapping_add(last);
    if x87_load32(e.mem.u32(last)) < t {
        return x87_load32(e.mem.u32(last.wrapping_add(4)));
    }
    e.with_stack(4, |e, last_index| {
        e.mem.set_u32(last_index.addr(), 0);
        e.call(
            FLOAT_KEY_GEN_INTERP,
            &args![x87_store32(t), keys, kind, count, last_index, size],
        )
        .f64()
    })
}

// Translated from 00a26ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (stream, key)): writes the key's time
/// (`NiStreamSaveBinary<float>`, `00a3da50`), then its 4-byte value at +4 through the stream's
/// output (the key argument's stack word is the byte-count word).
pub fn fn_00a26ef0(e: &mut Engine, stream: Ptr<NiStream>, key: Ptr) {
    e.call(WRITE_KEY_TIME, &args![stream, key]);
    e.with_stack(4, |e, count| {
        write_word(e, stream, key.addr().wrapping_add(4), count.addr());
    });
}

// Translated from 00a26f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two keys): whether the keys are equal: `00a26b00` (time
/// and value), then the floats at +8 and +0xc (the tangents), each `==` as the x87 compares
/// them (a NaN is unequal).
pub fn fn_00a26f30(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    if !e.call(KEYS_EQUAL, &args![first, second]).bool() {
        return false;
    }
    let at = |e: &Engine, key: Ptr, offset: u32| x87_load32(e.mem.u32(key.addr() + offset));
    at(e, first, 8) == at(e, second, 8) && at(e, first, 0xc) == at(e, second, 0xc)
}

// Translated from 00a26f80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, (destination, source)): copies a 16-byte key,
/// its four floats each through the x87 stack (`FLD`/`FSTP`, which quiets a signalling NaN).
pub fn fn_00a26f80(e: &mut Engine, destination: Ptr, source: Ptr) {
    for offset in [0u32, 4, 8, 0xc] {
        let value = x87_quiet_bits(e.mem.u32(source.addr() + offset));
        e.mem.set_u32(destination.addr() + offset, value);
    }
}

// Translated from 00a26fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body the compiler's array destructor iterator
/// (`stdcall`, (array, element size, count, destructor)): runs `destructor` (`thiscall`, no
/// stack words) on each element from the last to the first (none when the signed count is
/// 0 or less). Answers what is left in `EAX`: the last destructor's result, or `size * count`
/// when nothing ran.
pub fn fn_00a26fa0(e: &mut Engine, array: u32, size: u32, count: i32, destructor: u32) -> u32 {
    let mut eax = size.wrapping_mul(count as u32);
    let mut element = array.wrapping_add(eax);
    let mut left = count.wrapping_sub(1);
    while left >= 0 {
        element = element.wrapping_sub(size);
        eax = e.call(destructor, &args![element]).u32();
        left = left.wrapping_sub(1);
    }
    eax
}

// Translated from 00a27040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the count): makes `count` uninitialised keys of 16
/// bytes from the memory manager (`count * 16 + 4` bytes, saturating, the count in the first
/// word). Answers the keys, or null when the allocation fails.
pub fn fn_00a27040(e: &mut Engine, count: u32) -> u32 {
    allocate_keys(e, count, 0x10)
}

// Translated from 00a27080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (time, &keys, &count); a bool): inserts a key at
/// `time` into a list of 16-byte keys (time, value, in tangent, out tangent). `00a3d5d0`
/// finds the index; when a key already has that time nothing changes (false). Otherwise a
/// list one longer (from the memory manager, saturating size) takes the keys before the index,
/// the new key and the keys after it (`_memcpy`); the new key's value is the old keys' value at
/// `time` (`00a26d70`, type 2, size 16) and its tangents 0. Between two keys, `00a420e0`
/// gives the previous key's out tangent, the new key's tangents and the next key's in tangent
/// (every float copied through the x87 stack and stack temporaries as the exe does). The count
/// grows by one, the old list is freed (`00aa4060`, from its count word) and the new one
/// stored. Answers true.
pub fn fn_00a27080(e: &mut Engine, time: f32, keys: Ptr, count: Ptr) -> bool {
    let old = e.mem.u32(keys.addr());
    let time_word = x87_quiet_bits(time.to_bits());
    // The stack locals from ESP-0x20 up: the index (+0), the previous key's out tangent
    // (+4), the value and then the next key's in tangent (+8), the next key's time (+0xc)
    // and value (+0x10), the previous key's time (+0x14) and value (+0x18).
    e.with_stack(0x20, |e, locals| {
        let locals = locals.addr();
        let index_word = locals;
        let previous_out = locals + 4;
        let value_word = locals + 8;
        let next_time = locals + 0xc;
        let next_value = locals + 0x10;
        let previous_time = locals + 0x14;
        let previous_value = locals + 0x18;
        let n = e.mem.u32(count.addr());
        let found = e
            .call(
                FIND_INSERT_INDEX,
                &args![time_word, old, n, index_word, 0x10u32],
            )
            .bool();
        if !found {
            return false;
        }
        let grown = e.mem.u32(count.addr()).wrapping_add(1);
        let new = allocate_keys(e, grown, 0x10);
        let index = e.mem.u32(index_word);
        e.call(MEMCPY, &args![new, old, index << 4]);
        let n = e.mem.u32(count.addr());
        let index = e.mem.u32(index_word);
        if n > index {
            e.call(
                MEMCPY,
                &args![
                    (index << 4).wrapping_add(new).wrapping_add(0x10),
                    (index << 4).wrapping_add(old),
                    (n - index) << 4
                ],
            );
        }
        let n = e.mem.u32(count.addr());
        let value = fn_00a26d70(e, f32::from_bits(time_word), Ptr::new(old), 2, n, 0x10);
        e.mem.set_u32(value_word, x87_store32(value));
        let key = (e.mem.u32(index_word) << 4).wrapping_add(new);
        let time_st = x87_load32(time.to_bits());
        e.mem.set_u32(key, x87_store32(time_st));
        let value_st = x87_load32(e.mem.u32(value_word));
        e.mem.set_u32(key.wrapping_add(4), x87_store32(value_st));
        e.mem.set_u32(key.wrapping_add(8), 0);
        e.mem.set_u32(key.wrapping_add(0xc), 0);
        let index = e.mem.u32(index_word);
        if index != 0 && index != e.mem.u32(count.addr()) {
            let at = (index << 4).wrapping_add(new);
            let copy = |e: &mut Engine, to: u32, from: u32| {
                let word = x87_quiet_bits(e.mem.u32(from));
                e.mem.set_u32(to, word);
            };
            copy(e, previous_value, at.wrapping_sub(0xc));
            copy(e, previous_time, at.wrapping_sub(0x10));
            copy(e, previous_out, at.wrapping_sub(4));
            copy(e, next_value, at.wrapping_add(0x14));
            copy(e, next_time, at.wrapping_add(0x10));
            copy(e, value_word, at.wrapping_add(0x18));
            let pushed = |e: &Engine, from: u32| x87_quiet_bits(e.mem.u32(from));
            let words = [
                pushed(e, previous_value),
                pushed(e, previous_time),
                previous_out,
                pushed(e, next_value),
                pushed(e, next_time),
                value_word,
                x87_store32(time_st),
                x87_store32(value_st),
                previous_value,
                previous_time,
            ];
            e.call(NEW_KEY_TANGENTS, &words);
            copy(e, at.wrapping_sub(4), previous_out);
            copy(e, key.wrapping_add(8), previous_value);
            copy(e, key.wrapping_add(0xc), previous_time);
            copy(e, at.wrapping_add(0x18), value_word);
        }
        let n = e.mem.u32(count.addr());
        e.mem.set_u32(count.addr(), n.wrapping_add(1));
        if old != 0 {
            e.call(MEMORY_FREE, &args![MEMORY_MANAGER, old.wrapping_sub(4)]);
        }
        e.mem.set_u32(keys.addr(), new);
        true
    })
}

// Translated from 00a27320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (stream, keys, count)): writes `count` 16-byte keys:
/// each one's time and value (`00a26ef0`), then its 4-byte floats at +8 and +0xc through the
/// stream's output (the count argument's stack word is the byte-count word).
pub fn fn_00a27320(e: &mut Engine, stream: Ptr<NiStream>, keys: Ptr, count: u32) {
    if count == 0 {
        return;
    }
    e.with_stack(4, |e, word| {
        let mut key = keys.addr();
        for _ in 0..count {
            fn_00a26ef0(e, stream, Ptr::new(key));
            write_word(e, stream, key.wrapping_add(8), word.addr());
            write_word(e, stream, key.wrapping_add(0xc), word.addr());
            key = key.wrapping_add(0x10);
        }
    });
}

// Translated from 00a27450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (stream, key)): writes the key's time
/// (`NiStreamSaveBinary<float>`, `00a3da50`), then its three floats at +4
/// (`NiColor::SaveBinary`, `00a69780`).
pub fn fn_00a27450(e: &mut Engine, stream: Ptr<NiStream>, key: Ptr) {
    e.call(WRITE_KEY_TIME, &args![stream, key]);
    e.call(
        WRITE_THREE_FLOATS,
        &args![key.addr().wrapping_add(4), stream],
    );
}

// Translated from 00a27470 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, four words, the shape of the interpolation
/// functions this unit's registration stores: (time, key, next key, result)): copies the three
/// words at `011f426c` to `result`; the other words are not read.
pub fn fn_00a27470(e: &mut Engine, _unused_0: u32, _unused_1: u32, _unused_2: u32, result: Ptr) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(ZERO_POINT + offset);
        e.mem.set_u32(result.addr() + offset, word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_6eb0, fn_00a26eb0(Ptr, Ptr<NiStream>)),
        entry!(0x00a2_6fe0, fn_00a26fe0(f32, Ptr, Ptr, Ptr)),
        entry!(0x00a2_7250, fn_00a27250(Ptr<NiStream>, u32) -> u32),
        entry!(0x00a2_7430, fn_00a27430(Ptr, Ptr<NiStream>)),
        entry!(0x00a2_6df0, fn_00a26df0(Ptr<NiStream>, Ptr)),
        entry!(0x00a26e20, fn_00a26e20() -> u32),
        entry!(0x00a273a0, fn_00a273a0() -> u32),
        entry!(0x00a2_6d70, fn_00a26d70(f32, Ptr, u32, u32, u32) -> f64),
        entry!(0x00a2_6ef0, fn_00a26ef0(Ptr<NiStream>, Ptr)),
        entry!(0x00a2_6f30, fn_00a26f30(Ptr, Ptr) -> bool),
        entry!(0x00a2_6f80, fn_00a26f80(Ptr, Ptr)),
        entry!(0x00a2_6fa0, fn_00a26fa0(u32, u32, i32, u32) -> u32),
        entry!(0x00a2_7040, fn_00a27040(u32) -> u32),
        entry!(0x00a2_7080, fn_00a27080(f32, Ptr, Ptr) -> bool),
        entry!(0x00a2_7320, fn_00a27320(Ptr<NiStream>, Ptr, u32)),
        entry!(0x00a2_7450, fn_00a27450(Ptr<NiStream>, Ptr)),
        entry!(0x00a2_7470, fn_00a27470(u32, u32, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ: u32 = 0x00aa_2e60;

    #[test]
    fn fn_00a26df0_reads_four_bytes_through_the_input_stream() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 4, 10);
        e.mem.set_u32(input.addr() + 8, READ);
        let destination = e.mem.alloc(8);
        e.register(READ, |e, a| {
            // (input, destination, 4, &count, 1) with count set to 4
            assert_eq!(a[2], 4);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x1234_5678);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00a2_6df0, &args![stream, destination]);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|call| call.0 == READ).collect();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].1[..3], [input.addr(), destination, 4]);
        assert_eq!(e.mem.u32(destination), 0x1234_5678);
        assert_eq!(e.mem.u32(input.addr() + 4), 14);
    }

    #[test]
    fn fn_00a26df0_adds_a_short_read_to_the_position() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 8, READ);
        e.register(READ, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        let destination = e.mem.alloc(8);
        e.call(0x00a2_6df0, &args![stream, destination]);
        assert_eq!(e.mem.u32(input.addr() + 4), 2);
    }
}

#[cfg(test)]
mod key_tests {
    use super::*;

    const READ: u32 = 0x0300_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A stream whose input answers 4-byte words counting up from `first`.
    fn stream(e: &mut Engine, first: u32) -> Ptr<NiStream> {
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, READ);
        let mut next = first;
        e.register_double(READ, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], next);
            next += 1;
            4u32.into_ret()
        });
        stream
    }

    #[test]
    fn fn_00a26eb0_reads_the_time_then_the_value() {
        let mut e = Engine::new();
        e.register(READ_KEY_TIME, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            Ret::default()
        });
        let stream = stream(&mut e, 0x10);
        let key = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00a2_6eb0, &args![key, stream]);
        assert_eq!(calls_to(&e, READ_KEY_TIME), vec![vec![key, stream.addr()]]);
        assert_eq!((e.mem.u32(key), e.mem.u32(key + 4)), (0x77, 0x10));
    }

    #[test]
    fn fn_00a26fe0_passes_the_key_floats_and_stores_the_result() {
        let mut e = Engine::new();
        e.register(INTERPOLATE_VALUE, |_, a| {
            let f = |i: usize| f64::from(f32::from_bits(a[i]));
            Ret {
                st0: f(0) + 10.0 * f(1) + 100.0 * f(2) + 1000.0 * f(3) + 10000.0 * f(4),
                ..Ret::default()
            }
        });
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        for (key, values) in [
            (first, [9.0f32, 1.0, 9.0, 2.0]),
            (second, [9.0, 3.0, 4.0, 9.0]),
        ] {
            for (i, v) in values.iter().enumerate() {
                e.mem.set_f32(key + 4 * i as u32, *v);
            }
        }
        let result = e.mem.alloc(4);
        e.call(0x00a2_6fe0, &args![0.5f32, first, second, result]);
        assert_eq!(e.mem.f32(result), 43210.5);
    }

    #[test]
    fn fn_00a27250_makes_and_reads_the_keys() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(READ_KEY_TIME, |e, a| {
            e.mem.set_u32(a[0], 0xaa);
            Ret::default()
        });
        let stream = stream(&mut e, 1);
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_7250, &args![stream, 2u32]).u32();
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x24]]
        );
        assert_eq!(e.mem.u32(keys - 4), 2);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(keys + 4 * i)).collect();
        assert_eq!(words, vec![0xaa, 1, 2, 3, 0xaa, 4, 5, 6]);
        // A failed allocation with no keys answers null.
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a2_7250, &args![stream, 0u32]).u32(), 0);
    }

    #[test]
    fn fn_00a27430_reads_the_time_then_three_floats() {
        let mut e = Engine::new();
        e.register(READ_KEY_TIME, |_, _| Ret::default());
        e.register(READ_THREE_FLOATS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_7430, &args![0x5000u32, 0x6000u32]);
        assert_eq!(calls_to(&e, READ_KEY_TIME), vec![vec![0x5000, 0x6000]]);
        assert_eq!(calls_to(&e, READ_THREE_FLOATS), vec![vec![0x5004, 0x6000]]);
    }
}

#[cfg(test)]
mod n006_tests {
    use super::*;

    const WRITE: u32 = 0x0300_0010;
    const WRITE_LOG: u32 = 0x0300_0020;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A stream whose output logs each 4-byte word written to it (as a call to `WRITE_LOG`).
    fn output_stream(e: &mut Engine) -> Ptr<NiStream> {
        let stream = e.new_object::<NiStream>();
        let output = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkOstr, output.addr());
        e.set(output, NiBinaryStream::m_pfnWrite, WRITE);
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            let word = e.mem.u32(a[1]);
            e.call(WRITE_LOG, &args![word]);
            4u32.into_ret()
        });
        e.register(WRITE_LOG, |_, _| Ret::default());
        stream
    }

    fn words_written(e: &Engine) -> Vec<u32> {
        calls_to(e, WRITE_LOG).iter().map(|c| c[0]).collect()
    }

    fn keys(e: &mut Engine, words: &[f32]) -> u32 {
        let keys = e.mem.alloc(4 * words.len() as u32 + 0x40);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_f32(keys + 4 * i as u32, *w);
        }
        keys
    }

    #[test]
    fn fn_00a26d70_answers_the_ends_or_interpolates() {
        let mut e = Engine::new();
        e.map(NO_KEYS_VALUE, 4);
        e.mem.set_f32(NO_KEYS_VALUE, 7.5);
        e.register(FLOAT_KEY_GEN_INTERP, |e, a| {
            assert_eq!(e.mem.u32(a[4]), 0);
            Ret {
                st0: 99.0,
                ..Ret::default()
            }
        });
        // Two 12-byte keys: (1, 10) and (3, 30); the size word's low byte is 12.
        let k = keys(&mut e, &[1.0, 10.0, 0.0, 3.0, 30.0, 0.0]);
        let call = |e: &mut Engine, t: f32, count: u32| {
            e.call(0x00a2_6d70, &args![t, k, 2u32, count, 0x10cu32])
                .f64()
        };
        assert_eq!(call(&mut e, 2.0, 0), 7.5);
        assert_eq!(call(&mut e, 0.5, 2), 10.0);
        assert_eq!(call(&mut e, 4.0, 2), 30.0);
        e.call_log = Some(vec![]);
        assert_eq!(call(&mut e, 3.0, 2), 99.0);
        let interp = calls_to(&e, FLOAT_KEY_GEN_INTERP);
        assert_eq!(interp.len(), 1);
        assert_eq!(interp[0][..4], [3.0f32.to_bits(), k, 2, 2]);
        assert_eq!(interp[0][5], 0x10c);
        // A NaN time is neither before the first key nor after the last.
        assert_eq!(call(&mut e, f32::NAN, 2), 99.0);
    }

    #[test]
    fn fn_00a26ef0_writes_the_time_then_the_value() {
        let mut e = Engine::new();
        e.register(WRITE_KEY_TIME, |_, _| Ret::default());
        let stream = output_stream(&mut e);
        let key = keys(&mut e, &[1.0, 2.0]);
        e.call_log = Some(vec![]);
        e.call(0x00a2_6ef0, &args![stream, key]);
        assert_eq!(calls_to(&e, WRITE_KEY_TIME), vec![vec![stream.addr(), key]]);
        assert_eq!(words_written(&e), vec![2.0f32.to_bits()]);
        let output = e.get(stream, NiStream::m_pkOstr);
        assert_eq!(e.mem.u32(output + 4), 4);
    }

    #[test]
    fn fn_00a26f30_compares_the_tangents_after_the_keys() {
        let mut e = Engine::new();
        e.register(KEYS_EQUAL, |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let a = keys(&mut e, &[1.0, 2.0, 3.0, 4.0]);
        let b = keys(&mut e, &[1.0, 2.0, 3.0, 4.0]);
        assert!(e.call(0x00a2_6f30, &args![a, b]).bool());
        e.mem.set_f32(b + 0xc, 5.0);
        assert!(!e.call(0x00a2_6f30, &args![a, b]).bool());
        e.mem.set_f32(b + 0xc, 4.0);
        e.mem.set_f32(a + 8, f32::NAN);
        e.mem.set_f32(b + 8, f32::NAN);
        assert!(!e.call(0x00a2_6f30, &args![a, b]).bool());
        e.mem.set_f32(b, 9.0);
        assert!(!e.call(0x00a2_6f30, &args![a, b]).bool());
    }

    #[test]
    fn fn_00a26f80_copies_four_floats_quieting_nans() {
        let mut e = Engine::new();
        let source = keys(&mut e, &[1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u32(source + 8, 0x7f80_0001);
        let destination = e.mem.alloc(0x10);
        e.call(0x00a2_6f80, &args![destination, source]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(destination + 4 * i)).collect();
        assert_eq!(
            words,
            vec![
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                0x7fc0_0001,
                4.0f32.to_bits()
            ]
        );
    }

    #[test]
    fn fn_00a26fa0_destroys_from_the_last_element() {
        const DESTRUCTOR: u32 = 0x0300_0030;
        let mut e = Engine::new();
        e.register(DESTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        let eax = e
            .call(0x00a2_6fa0, &args![0x1000u32, 0x10u32, 3i32, DESTRUCTOR])
            .u32();
        assert_eq!(
            calls_to(&e, DESTRUCTOR),
            vec![vec![0x1020], vec![0x1010], vec![0x1000]]
        );
        assert_eq!(eax, 0x1001);
        e.call_log = Some(vec![]);
        let eax = e
            .call(0x00a2_6fa0, &args![0x1000u32, 0x10u32, -2i32, DESTRUCTOR])
            .u32();
        assert!(calls_to(&e, DESTRUCTOR).is_empty());
        assert_eq!(eax, 0xffff_ffe0);
    }

    #[test]
    fn fn_00a27040_allocates_counted_keys() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_7040, &args![3u32]).u32();
        assert_eq!(e.mem.u32(keys - 4), 3);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a2_7040, &args![0x1000_0000u32]).u32(), 0);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|c| c[1]).collect();
        assert_eq!(sizes, vec![0x34, 0xffff_ffff]);
    }

    /// Doubles for `fn_00a27080`: a real allocator and copy, the index finder answering
    /// `found` (`None`: a key has that time), the value at the time and the tangents.
    fn insert_engine(found: Option<u32>) -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            a[0].into_ret()
        });
        e.register_double(FIND_INSERT_INDEX, move |e, a| match found {
            Some(index) => {
                e.mem.set_u32(a[3], index);
                1u32.into_ret()
            }
            None => 0u32.into_ret(),
        });
        e.register(FLOAT_KEY_GEN_INTERP, |_, _| Ret {
            st0: 25.0,
            ..Ret::default()
        });
        e.register(NEW_KEY_TANGENTS, |e, a| {
            // Recognisable tangents through the four pointers.
            e.mem.set_f32(a[2], 101.0);
            e.mem.set_f32(a[5], 105.0);
            e.mem.set_f32(a[8], 108.0);
            e.mem.set_f32(a[9], 109.0);
            Ret::default()
        });
        e
    }

    #[test]
    fn fn_00a27080_inserts_between_two_keys() {
        let mut e = insert_engine(Some(1));
        let block = e.mem.alloc(0x24);
        e.mem.set_u32(block, 2);
        let old = block + 4;
        for (i, w) in [1.0f32, 10.0, 0.5, 0.25, 3.0, 30.0, 0.75, 0.125]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(old + 4 * i as u32, *w);
        }
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, old);
        e.mem.set_u32(slots + 4, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_7080, &args![2.0f32, slots, slots + 4]).bool());
        let new = e.mem.u32(slots);
        assert_eq!(e.mem.u32(slots + 4), 3);
        assert_eq!(e.mem.u32(new - 4), 3);
        let words: Vec<f32> = (0..12).map(|i| e.mem.f32(new + 4 * i)).collect();
        assert_eq!(
            words,
            vec![1.0, 10.0, 0.5, 101.0, 2.0, 25.0, 108.0, 109.0, 3.0, 30.0, 105.0, 0.125]
        );
        let tangents = calls_to(&e, NEW_KEY_TANGENTS);
        assert_eq!(tangents.len(), 1);
        let t = &tangents[0];
        assert_eq!((t[0], t[1]), (10.0f32.to_bits(), 1.0f32.to_bits()));
        assert_eq!((t[3], t[4]), (30.0f32.to_bits(), 3.0f32.to_bits()));
        assert_eq!((t[6], t[7]), (2.0f32.to_bits(), 25.0f32.to_bits()));
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, block]]);
        let interp = calls_to(&e, FLOAT_KEY_GEN_INTERP);
        assert_eq!(interp[0][..4], [2.0f32.to_bits(), old, 2, 2]);
    }

    #[test]
    fn fn_00a27080_appends_without_tangents_and_refuses_a_duplicate() {
        let mut e = insert_engine(Some(1));
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block, 1);
        let old = block + 4;
        e.mem.set_f32(old, 1.0);
        e.mem.set_f32(old + 4, 10.0);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, old);
        e.mem.set_u32(slots + 4, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_7080, &args![2.0f32, slots, slots + 4]).bool());
        assert!(calls_to(&e, NEW_KEY_TANGENTS).is_empty());
        let new = e.mem.u32(slots);
        assert_eq!(e.mem.f32(new + 0x10), 2.0);
        // After the last key's time the value is the last key's (no interpolation).
        assert_eq!(e.mem.f32(new + 0x14), 10.0);
        assert_eq!((e.mem.u32(new + 0x18), e.mem.u32(new + 0x1c)), (0, 0));
        // A key at that time already: nothing changes.
        let mut e = insert_engine(None);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, 0x1234);
        e.mem.set_u32(slots + 4, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a2_7080, &args![2.0f32, slots, slots + 4]).bool());
        assert_eq!((e.mem.u32(slots), e.mem.u32(slots + 4)), (0x1234, 1));
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
    }

    #[test]
    fn fn_00a27320_writes_each_key() {
        let mut e = Engine::new();
        e.register(WRITE_KEY_TIME, |_, _| Ret::default());
        let stream = output_stream(&mut e);
        let k = keys(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        e.call_log = Some(vec![]);
        e.call(0x00a2_7320, &args![stream, k, 2u32]);
        let expected: Vec<u32> = [2.0f32, 3.0, 4.0, 6.0, 7.0, 8.0]
            .iter()
            .map(|f| f.to_bits())
            .collect();
        assert_eq!(words_written(&e), expected);
        assert_eq!(
            calls_to(&e, WRITE_KEY_TIME),
            vec![vec![stream.addr(), k], vec![stream.addr(), k + 0x10]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00a2_7320, &args![stream, k, 0u32]);
        assert!(words_written(&e).is_empty());
    }

    #[test]
    fn fn_00a27450_writes_the_time_then_three_floats() {
        let mut e = Engine::new();
        e.register(WRITE_KEY_TIME, |_, _| Ret::default());
        e.register(WRITE_THREE_FLOATS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_7450, &args![0x6000u32, 0x5000u32]);
        assert_eq!(calls_to(&e, WRITE_KEY_TIME), vec![vec![0x6000, 0x5000]]);
        assert_eq!(calls_to(&e, WRITE_THREE_FLOATS), vec![vec![0x5004, 0x6000]]);
    }

    #[test]
    fn fn_00a27470_copies_the_zero_point() {
        let mut e = Engine::new();
        e.map(ZERO_POINT, 12);
        for (i, w) in [5u32, 6, 7].iter().enumerate() {
            e.mem.set_u32(ZERO_POINT + 4 * i as u32, *w);
        }
        let result = e.mem.alloc(12);
        e.call(0x00a2_7470, &args![1.0f32, 0u32, 0u32, result]);
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(result + 4 * i)).collect();
        assert_eq!(words, vec![5, 6, 7]);
    }
}
