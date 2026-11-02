//! `Havok SDK/run_00e499d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{fcom_status, load_vec, store_vec};
use crate::units::havok_sdk::run_00e401d0::reserve_block;

/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`; its `+0x10` is the heap
/// allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`): cdecl, `(allocator, array, count, element size)`.
const RESERVE: u32 = 0x00c9_08c0;
/// The base constructor (`00cbfba0`): thiscall, `this` only.
const BASE_CONSTRUCTOR: u32 = 0x00cb_fba0;
/// `hkReferencedObject::addReference` (Xbox PDB, `00c90510`): thiscall, `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkReferencedObject::removeReference` (Xbox PDB, `00c905b0`): thiscall, `this` only.
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The `float`s the height range starts from (`0104da48` for the minimum, `0104e148` for the
/// maximum) and the `double` the range is divided by (`010217d0`).
const RANGE_MINIMUM_START: u32 = 0x0104_da48;
const RANGE_MAXIMUM_START: u32 = 0x0104_e148;
const RANGE_DIVISOR: u32 = 0x0102_17d0;

/// `FISTP dword` with the rounding control set to truncate (`0xc00`): the 32-bit integer, or the
/// integer indefinite (`0x80000000`) for a NaN or a value out of range.
pub(crate) fn fistp_truncate_i32(value: f64) -> i32 {
    let truncated = value.trunc();
    if value.is_nan() || truncated >= 2_147_483_648.0 || truncated < -2_147_483_648.0 {
        i32::MIN
    } else {
        truncated as i32
    }
}

/// The height the source returns for cell `(x, y)` (virtual slot `0x34`, a result in `ST0`), as the
/// float the exe stores it as.
fn source_height(e: &mut Engine, source: u32, x: i32, y: i32) -> u32 {
    let height = e.vcall(source, 0x34, &args![x, y]).f64();
    x87_store32(height)
}

// Translated from 00e499d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall constructor, `RET 4`, quantising a height field. `source`'s width and height
/// (`+0x10`, `+0x14`), the float at `+0x18` (through the x87 stack) and the four vectors at
/// `+0x20` are copied; a byte from the source's virtual slot `0x38` (written through a pointer)
/// goes to `+0x6c`; `+0x1d` is 1, `+6` is 1, `+0xc` is `0xd` and the vtable `010e901c` is stored.
/// The `u16` array at `+0x60` is reserved for width * height entries (`00c908c0`, at least double
/// the capacity). Every cell's height (source slot `0x34`, `(x, y)`, a result in `ST0`) gives the
/// minimum and maximum (strict comparisons on the x87 flags: a NaN never replaces them); the
/// minimum is stored as a float at `+0x70` and `(max - min) / 65535.0` (the double at
/// `010217d0`) at `+0x74`. Each cell is then stored at `row * width + column` as
/// `trunc((height - min) / scale)` (the control word forced to truncate; the low 16 bits).
/// Returns `this`.
pub fn fn_00e499d0(e: &mut Engine, this: Ptr, source: u32) -> u32 {
    let this = this.addr();
    let array = this.wrapping_add(0x60);
    e.mem.set_u16(this.wrapping_add(6), 1);
    e.mem.set_u32(this.wrapping_add(8), 0);
    e.mem.set_u32(this.wrapping_add(0xc), 0xd);
    e.mem.set_u32(this, 0x010e_901c);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array.wrapping_add(4), 0);
    e.mem.set_u32(array.wrapping_add(8), 0x8000_0000);
    e.mem.set_u8(this.wrapping_add(0x1d), 1);
    let width = e.mem.u32(source.wrapping_add(0x10));
    e.mem.set_u32(this.wrapping_add(0x10), width);
    let height = e.mem.u32(source.wrapping_add(0x14));
    e.mem.set_u32(this.wrapping_add(0x14), height);
    let flag = e.with_stack(0x10, |e, local| {
        e.vcall(source, 0x38, &args![local]);
        e.mem.u8(local.addr())
    });
    e.mem.set_u8(this.wrapping_add(0x6c), flag);
    let spacing = x87_quiet_bits(e.mem.u32(source.wrapping_add(0x18)));
    e.mem.set_u32(this.wrapping_add(0x18), spacing);
    for row in 0..4u32 {
        let value = load_vec(e, source.wrapping_add(0x20 + 16 * row));
        store_vec(e, this.wrapping_add(0x20 + 16 * row), value);
    }
    let cells = e
        .mem
        .i32(this.wrapping_add(0x14))
        .wrapping_mul(e.mem.i32(this.wrapping_add(0x10)));
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let capacity = (e.mem.u32(array.wrapping_add(8)) & 0x3fff_ffff) as i32;
    if capacity < cells {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if cells < doubled { doubled } else { cells };
        e.call(RESERVE, &args![allocator, array, wanted, 2u32]);
    }
    e.mem.set_i32(array.wrapping_add(4), cells);
    let mut minimum = x87_quiet_bits(e.mem.u32(RANGE_MINIMUM_START));
    let mut maximum = x87_quiet_bits(e.mem.u32(RANGE_MAXIMUM_START));
    let mut row = 0i32;
    if e.mem.i32(this.wrapping_add(0x14)) > 0 {
        loop {
            let mut column = 0i32;
            if e.mem.i32(this.wrapping_add(0x10)) > column {
                loop {
                    let value = x87_load32(source_height(e, source, column, row));
                    // FCOMP of the minimum with the value, `TEST AH, 0x41`: a smaller value only.
                    if fcom_status(x87_load32(minimum), value) & 0x41 == 0 {
                        minimum = x87_store32(value);
                    }
                    // FCOMP of the maximum with the value, `TEST AH, 5` and `JP`: C0 set, C2
                    // clear, a larger value only.
                    if fcom_status(x87_load32(maximum), value) & 0x05 == 0x01 {
                        maximum = x87_store32(value);
                    }
                    column += 1;
                    if column >= e.mem.i32(this.wrapping_add(0x10)) {
                        break;
                    }
                }
            }
            row += 1;
            if row >= e.mem.i32(this.wrapping_add(0x14)) {
                break;
            }
        }
    }
    let minimum_value = x87_load32(minimum);
    e.mem
        .set_u32(this.wrapping_add(0x70), x87_store32(minimum_value));
    let range = x87_sub(x87_m32(maximum), minimum_value);
    let scale = x87_div(range, e.mem.f64(RANGE_DIVISOR));
    e.mem.set_u32(this.wrapping_add(0x74), x87_store32(scale));
    let mut row = 0i32;
    if e.mem.i32(this.wrapping_add(0x14)) > 0 {
        loop {
            let mut column = 0i32;
            if e.mem.i32(this.wrapping_add(0x10)) > column {
                loop {
                    let value = x87_load32(source_height(e, source, column, row));
                    let offset = x87_sub(value, x87_m32(e.mem.u32(this.wrapping_add(0x70))));
                    let width = e.mem.i32(this.wrapping_add(0x10));
                    let quantised = x87_div(offset, x87_m32(e.mem.u32(this.wrapping_add(0x74))));
                    let index = width.wrapping_mul(row).wrapping_add(column);
                    column += 1;
                    let integer = fistp_truncate_i32(quantised);
                    let data = e.mem.u32(array);
                    e.mem.set_u16(
                        data.wrapping_add((index as u32).wrapping_mul(2)),
                        integer as u16,
                    );
                    if column >= e.mem.i32(this.wrapping_add(0x10)) {
                        break;
                    }
                }
            }
            row += 1;
            if row >= e.mem.i32(this.wrapping_add(0x14)) {
                break;
            }
        }
    }
    this
}

/// Asks each object of the pointer array at `*(this + array_offset)` (`count` of them, last
/// first) through virtual slot `slot` of its interface at `object + interface_offset`, with the
/// stack words `words`. The first word the callee gets is a pointer to a stack word that starts
/// as the last of `words`; the callee writes its answer byte there and returns the pointer. The
/// first object that answers 0 stops the loop. Writes 1 to `out` when every object answered
/// nonzero (or there are none), else 0.
fn all_accept(
    e: &mut Engine,
    out: u32,
    list: (u32, i32),
    interface_offset: u32,
    slot: u32,
    words: &[u32],
) {
    let (array_address, count) = list;
    let last = words.last().copied().unwrap_or(0);
    let mut index = count.wrapping_sub(1);
    let mut accepted = true;
    while index >= 0 {
        let array = e.mem.u32(array_address);
        let object = e
            .mem
            .u32(array.wrapping_add((index as u32).wrapping_mul(4)));
        let answer = e.with_stack(4, |e, answer_slot| {
            e.mem.set_u32(answer_slot.addr(), last);
            let mut call_words = vec![answer_slot.addr()];
            call_words.extend_from_slice(words);
            let at = e
                .vcall(object.wrapping_add(interface_offset), slot, &call_words)
                .u32();
            e.mem.u8(at)
        });
        if answer == 0 {
            accepted = false;
            break;
        }
        index -= 1;
    }
    e.mem.set_u8(out, u8::from(accepted));
}

// Translated from 00e49bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: asks the `[+0x2c]` objects of `*(this + 0x28)` (last first, count read
/// once) through virtual slot `4` of their interface at `+8` with `(slot, first, second)` (the
/// slot word starts as `second`); `out` gets 1 when all answer nonzero, else 0.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn fn_00e49bc0(e: &mut Engine, this: Ptr, out: u32, first: u32, second: u32) -> u32 {
    let this = this.addr();
    let count = e.mem.i32(this.wrapping_add(0x2c));
    all_accept(
        e,
        out,
        (this.wrapping_add(0x28), count),
        8,
        4,
        &[first, second],
    );
    out
}

// Translated from 00e49c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x18`: as `00e49bc0` over `[+0x28]` objects of `*(this + 0x24)`, through
/// virtual slot `4` of their interface at `+0xc`, with five words after the slot word.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00cfae7e).
#[allow(clippy::too_many_arguments)]
pub fn fn_00e49c20(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
) -> u32 {
    let this = this.addr();
    let count = e.mem.i32(this.wrapping_add(0x28));
    all_accept(
        e,
        out,
        (this.wrapping_add(0x24), count),
        0xc,
        4,
        &[word_2, word_3, word_4, word_5, word_6],
    );
    out
}

// Translated from 00e49c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x14`: as `00e49bc0` over `[+0x24]` objects of `*(this + 0x20)`, through
/// virtual slot `0` of their interface at `+0x10`, with four words after the slot word.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00e49c90(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
) -> u32 {
    let this = this.addr();
    let count = e.mem.i32(this.wrapping_add(0x24));
    all_accept(
        e,
        out,
        (this.wrapping_add(0x20), count),
        0x10,
        0,
        &[word_2, word_3, word_4, word_5],
    );
    out
}

// Translated from 00e49cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x20`: as `00e49bc0` over `[+0x28]` objects of `*(this + 0x24)`, through
/// virtual slot `0` of their interface at `+0xc`, with seven words after the slot word.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00e49cf0(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
) -> u32 {
    let this = this.addr();
    let count = e.mem.i32(this.wrapping_add(0x28));
    all_accept(
        e,
        out,
        (this.wrapping_add(0x24), count),
        0xc,
        0,
        &[word_2, word_3, word_4, word_5, word_6, word_7, word_8],
    );
    out
}

// Translated from 00e49d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: as `00e49bc0` over `[+0x20]` objects of `*(this + 0x1c)`, through
/// virtual slot `4` of their interface at `+0x14`, with two words after the slot word.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn fn_00e49d60(e: &mut Engine, this: Ptr, out: u32, first: u32, second: u32) -> u32 {
    let this = this.addr();
    let count = e.mem.i32(this.wrapping_add(0x20));
    all_accept(
        e,
        out,
        (this.wrapping_add(0x1c), count),
        0x14,
        4,
        &[first, second],
    );
    out
}

/// Stores the vtables `010e90e8` at `+0`, `010e90dc`, `010e90c8`, `010e90bc` and `010e90b0` at
/// `+8`, `+0xc`, `+0x10` and `+0x14`.
fn store_array_vtables(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, 0x010e_90e8);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_90dc);
    e.mem.set_u32(this.wrapping_add(0xc), 0x010e_90c8);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010e_90bc);
    e.mem.set_u32(this.wrapping_add(0x14), 0x010e_90b0);
}

// Translated from 00e49e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall constructor, `RET 4`: runs the base constructor (`00cbfba0`), stores the vtables
/// (`010e90e8`, `010e90dc`, `010e90c8`, `010e90bc`, `010e90b0` at `+0`, `+8`, `+0xc`, `+0x10`,
/// `+0x14`), an empty unowned `hkArray` (`+0x30`, `+0x34`, `+0x38`) and 3 at `+0x20`. The
/// pointer array `source` (`data`, `size`) is copied into it (room made through the allocator, as
/// in `00e402e0`) and every copied object gets a reference (`00c90510`, without a null check).
/// Returns `this`.
pub fn fn_00e49e50(e: &mut Engine, this: Ptr, source: u32) -> u32 {
    let this = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    store_array_vtables(e, this);
    e.mem.set_u32(this.wrapping_add(0x30), 0);
    e.mem.set_u32(this.wrapping_add(0x34), 0);
    e.mem.set_u32(this.wrapping_add(0x38), 0x8000_0000);
    e.mem.set_u32(this.wrapping_add(0x20), 3);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let count = e.mem.i32(source.wrapping_add(4));
    reserve_block(e, allocator, this.wrapping_add(0x30), count, 4);
    let count = e.mem.i32(source.wrapping_add(4));
    let target = e.mem.u32(this.wrapping_add(0x30));
    e.mem.set_i32(this.wrapping_add(0x34), count);
    let data = e.mem.u32(source);
    for index in 0..count.max(0) as u32 {
        let value = e.mem.u32(data.wrapping_add(4 * index));
        e.mem.set_u32(target.wrapping_add(4 * index), value);
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this.wrapping_add(0x34)) {
        let objects = e.mem.u32(this.wrapping_add(0x30));
        let object = e.mem.u32(objects.wrapping_add(4 * index));
        e.call(ADD_REFERENCE, &args![object]);
        index += 1;
    }
    this
}

// Translated from 00e49f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall destructor: stores the vtables of `00e49e50`, drops the reference on each of the
/// `[+0x34]` objects of `*(this + 0x30)` (`00c905b0`, without a null check), empties the array
/// (size 0, the block freed through the allocator, virtual slot `0x10`, with the pointer and
/// `4 * capacity`, unless the sign bit marks it unowned; then `0` and `0x80000000`) and stores
/// the final vtables (`010c3b8c` at `+0x14`, `010c3bac`, `010c3b98`, `010c3b80` at `+0x10`, `+0xc`,
/// `+8`, and `0102e388` at `+0`).
pub fn fn_00e49f40(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    store_array_vtables(e, this);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this.wrapping_add(0x34)) {
        let objects = e.mem.u32(this.wrapping_add(0x30));
        let object = e.mem.u32(objects.wrapping_add(4 * index));
        e.call(REMOVE_REFERENCE, &args![object]);
        index += 1;
    }
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let flags = e.mem.u32(this.wrapping_add(0x38));
    e.mem.set_u32(this.wrapping_add(0x34), 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(this.wrapping_add(0x30));
        e.vcall(
            allocator,
            0x10,
            &args![data, (flags & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(this.wrapping_add(0x30), 0);
    e.mem.set_u32(this.wrapping_add(0x38), 0x8000_0000);
    e.mem.set_u32(this.wrapping_add(0x14), 0x010c_3b8c);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010c_3bac);
    e.mem.set_u32(this.wrapping_add(0xc), 0x010c_3b98);
    e.mem.set_u32(this.wrapping_add(8), 0x010c_3b80);
    e.mem.set_u32(this, 0x0102_e388);
}

// Translated from 00e49fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall constructor: runs the base constructor (`00cbfba0`), stores the vtables of
/// `00e49e50` and an empty unowned `hkArray` (`+0x30`, `+0x34`, `+0x38`). Returns `this`.
pub fn fn_00e49fe0(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    store_array_vtables(e, this);
    e.mem.set_u32(this.wrapping_add(0x30), 0);
    e.mem.set_u32(this.wrapping_add(0x34), 0);
    e.mem.set_u32(this.wrapping_add(0x38), 0x8000_0000);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_99d0, fn_00e499d0(Ptr, u32) -> u32),
        entry!(0x00e4_9bc0, fn_00e49bc0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_9c20, fn_00e49c20(Ptr, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e4_9c90, fn_00e49c90(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x00e4_9cf0,
            fn_00e49cf0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e4_9d60, fn_00e49d60(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_9e50, fn_00e49e50(Ptr, u32) -> u32),
        entry!(0x00e4_9f40, fn_00e49f40(Ptr)),
        entry!(0x00e4_9fe0, fn_00e49fe0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The argument words each double saw, tagged with the address.
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(0x0126_0010, 0x4444);
        e.mem.set_f32(RANGE_MINIMUM_START, f32::MAX);
        e.mem.set_f32(RANGE_MAXIMUM_START, -f32::MAX);
        e.mem.set_f64(RANGE_DIVISOR, 65535.0);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e
    }

    fn record(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let address = *address;
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    fn put_u32s(e: &mut Engine, address: u32, values: &[(u32, u32)]) {
        for (offset, value) in values {
            e.mem.set_u32(address + offset, *value);
        }
    }

    /// A height field source with the given `heights` (row by row, `width` per row); slot `0x38`
    /// writes the byte `0x5a` and slot `0x34` returns the height of `(x, y)` in `ST0`.
    fn height_source(e: &mut Engine, width: u32, heights: &[f64]) -> u32 {
        let table: Vec<f64> = heights.to_vec();
        e.register_double(0x0ff0_0034, move |_, a| Ret {
            st0: table[(a[2] * width + a[1]) as usize],
            ..Ret::default()
        });
        e.register_double(0x0ff0_0038, |e, a| {
            e.mem.set_u8(a[1], 0x5a);
            Ret::default()
        });
        let mut slots = [0u32; 15];
        slots[13] = 0x0ff0_0034;
        slots[14] = 0x0ff0_0038;
        e.put_vtable(0x0126_0800, &slots);
        let source = e.mem.alloc(0x80);
        let height = heights.len() as u32 / width.max(1);
        put_u32s(
            e,
            source,
            &[
                (0, 0x0126_0800),
                (0x10, width),
                (0x14, height),
                (0x18, 0.25f32.to_bits()),
            ],
        );
        for row in 0..4u32 {
            store_vec(e, source + 0x20 + 16 * row, [row as f32 + 1.0; 4]);
        }
        source
    }

    fn reserve_double(e: &mut Engine) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(RESERVE, move |e, a| {
            log.borrow_mut().push((RESERVE, a.to_vec()));
            let data = e.mem.alloc(2 * a[2] + 8);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        calls
    }

    fn quantised(e: &Engine, this: u32, count: u32) -> Vec<u16> {
        let data = e.mem.u32(this + 0x60);
        (0..count)
            .map(|index| e.mem.u16(data + 2 * index))
            .collect()
    }

    #[test]
    fn heights_are_quantised_to_16_bits_by_truncation() {
        let mut e = engine();
        let reserved = reserve_double(&mut e);
        let source = height_source(&mut e, 2, &[-10.0, -9.5, 0.0, 65525.0]);
        let this = e.mem.alloc(0x80);
        let result = e.call(0x00e4_99d0, &args![this, source]).u32();
        assert_eq!(result, this);
        assert_eq!(
            *reserved.borrow(),
            vec![(RESERVE, vec![0x4444, this + 0x60, 4, 2])]
        );
        assert_eq!(e.mem.u32(this), 0x010e_901c);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0xd);
        assert_eq!(e.mem.u8(this + 0x1d), 1);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(e.mem.u32(this + 0x14), 2);
        assert_eq!(e.mem.f32(this + 0x18), 0.25);
        assert_eq!(e.mem.u8(this + 0x6c), 0x5a);
        assert_eq!(load_vec(&e, this + 0x50), [4.0; 4]);
        assert_eq!(e.mem.u32(this + 0x64), 4);
        assert_eq!(e.mem.f32(this + 0x70), -10.0);
        assert_eq!(e.mem.f32(this + 0x74), 1.0);
        assert_eq!(quantised(&e, this, 4), vec![0, 0, 10, 65535]);
    }

    #[test]
    fn nan_heights_never_set_the_range() {
        let mut e = engine();
        reserve_double(&mut e);
        let source = height_source(&mut e, 2, &[5.0, f64::NAN, 7.0, 65540.0]);
        let this = e.mem.alloc(0x80);
        e.call(0x00e4_99d0, &args![this, source]);
        assert_eq!(e.mem.f32(this + 0x70), 5.0);
        assert_eq!(e.mem.f32(this + 0x74), 1.0);
        // The NaN cell becomes the integer indefinite, whose low 16 bits are 0.
        assert_eq!(quantised(&e, this, 4), vec![0, 0, 2, 65535]);
    }

    #[test]
    fn empty_field_keeps_the_starting_range() {
        let mut e = engine();
        let reserved = reserve_double(&mut e);
        let source = height_source(&mut e, 1, &[]);
        let this = e.mem.alloc(0x80);
        e.call(0x00e4_99d0, &args![this, source]);
        assert!(reserved.borrow().is_empty());
        assert_eq!(e.mem.f32(this + 0x70), f32::MAX);
        let big = f64::from(f32::MAX);
        let expected = (((-big) - big) / 65535.0) as f32;
        assert_eq!(e.mem.u32(this + 0x74), expected.to_bits());
    }

    #[test]
    fn truncation_to_32_bits_handles_overflow() {
        assert_eq!(fistp_truncate_i32(-1.9), -1);
        assert_eq!(fistp_truncate_i32(2.9), 2);
        assert_eq!(fistp_truncate_i32(2_147_483_648.0), i32::MIN);
        assert_eq!(fistp_truncate_i32(-2_147_483_649.0), i32::MIN);
        assert_eq!(fistp_truncate_i32(f64::NAN), i32::MIN);
    }

    /// An owner whose array (`array_offset`, count at `count_offset`) holds objects with an
    /// interface at `interface_offset` whose vtable slot `slot_offset` answers with the byte at
    /// `object + 0x60` and records its words and the initial slot word.
    fn acceptors(
        e: &mut Engine,
        offsets: [u32; 4],
        answers: &[u8],
    ) -> (u32, Vec<u32>, Rc<RefCell<Vec<(Vec<u32>, u32)>>>) {
        let [array_offset, count_offset, interface_offset, slot_offset] = offsets;
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(0x0ff0_0200, move |e, a| {
            let answer = e.mem.u8(a[0] - interface_offset + 0x60);
            log.borrow_mut().push((a.to_vec(), e.mem.u32(a[1])));
            e.mem.set_u8(a[1], answer);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let mut slots = vec![0u32; slot_offset as usize / 4 + 1];
        slots[slot_offset as usize / 4] = 0x0ff0_0200;
        e.put_vtable(0x0126_0900, &slots);
        let this = e.mem.alloc(0x80);
        let list = e.mem.alloc(4 * answers.len() as u32 + 4);
        let mut objects = Vec::new();
        for (index, answer) in answers.iter().enumerate() {
            let object = e.mem.alloc(0x80);
            e.mem.set_u32(object + interface_offset, 0x0126_0900);
            e.mem.set_u8(object + 0x60, *answer);
            e.mem.set_u32(list + 4 * index as u32, object);
            objects.push(object);
        }
        e.mem.set_u32(this + array_offset, list);
        e.mem.set_u32(this + count_offset, answers.len() as u32);
        (this, objects, calls)
    }

    fn out_byte(e: &mut Engine) -> u32 {
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0xee);
        out
    }

    #[test]
    fn two_word_acceptor_asks_last_first_and_stops_at_a_refusal() {
        let mut e = engine();
        let (this, objects, calls) = acceptors(&mut e, [0x28, 0x2c, 8, 4], &[1, 0, 1]);
        let out = out_byte(&mut e);
        e.call(0x00e4_9bc0, &args![this, out, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u8(out), 0);
        let seen = calls.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].0[0], objects[2] + 8);
        assert_eq!(seen[1].0[0], objects[1] + 8);
        assert_eq!(&seen[0].0[2..], &[0x11, 0x22]);
        // The slot word started as the last word.
        assert_eq!(seen[0].1, 0x22);
        drop(seen);
        let (this, _, calls) = acceptors(&mut e, [0x28, 0x2c, 8, 4], &[1, 7, 1]);
        e.call(0x00e4_9bc0, &args![this, out, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls.borrow().len(), 3);
        let (this, _, calls) = acceptors(&mut e, [0x28, 0x2c, 8, 4], &[]);
        e.mem.set_u8(out, 0);
        e.call(0x00e4_9bc0, &args![this, out, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u8(out), 1);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn fn_00e49bc0_answers_its_result_pointer() {
        let mut e = engine();
        let (this, _objects, _calls) = acceptors(&mut e, [0x28, 0x2c, 8, 4], &[1, 0, 1]);
        let out = out_byte(&mut e);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00e4_9bc0, &args![this, out, 0x11u32, 0x22u32])
                .u32(),
            args![out][0]
        );
    }

    #[test]
    fn wider_acceptors_pass_all_their_words() {
        let mut e = engine();
        let out = out_byte(&mut e);
        let (this, objects, calls) = acceptors(&mut e, [0x24, 0x28, 0xc, 4], &[1, 1]);
        e.call(0x00e4_9c20, &args![this, out, 2u32, 3u32, 4u32, 5u32, 6u32]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5, 6]);
        assert_eq!(calls.borrow()[0].0[0], objects[1] + 0xc);
        assert_eq!(calls.borrow()[0].1, 6);
        let (this2, _, calls) = acceptors(&mut e, [0x20, 0x24, 0x10, 0], &[1, 0]);
        e.call(0x00e4_9c90, &args![this2, out, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(calls.borrow().len(), 1);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5]);
        assert_eq!(calls.borrow()[0].1, 5);
        let (this3, _, calls) = acceptors(&mut e, [0x24, 0x28, 0xc, 0], &[1, 1, 1]);
        e.call(
            0x00e4_9cf0,
            &args![this3, out, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32],
        );
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls.borrow().len(), 3);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(calls.borrow()[0].1, 8);
        let (this4, _, calls) = acceptors(&mut e, [0x1c, 0x20, 0x14, 4], &[1, 0]);
        e.call(0x00e4_9d60, &args![this4, out, 9u32, 10u32]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(&calls.borrow()[0].0[2..], &[9, 10]);
    }

    #[test]
    fn fn_00e49d60_answers_its_result_pointer() {
        let mut e = engine();
        let out = out_byte(&mut e);
        let (this, objects, calls) = acceptors(&mut e, [0x24, 0x28, 0xc, 4], &[1, 1]);
        e.call(0x00e4_9c20, &args![this, out, 2u32, 3u32, 4u32, 5u32, 6u32]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5, 6]);
        assert_eq!(calls.borrow()[0].0[0], objects[1] + 0xc);
        assert_eq!(calls.borrow()[0].1, 6);
        let (this2, _, calls) = acceptors(&mut e, [0x20, 0x24, 0x10, 0], &[1, 0]);
        e.call(0x00e4_9c90, &args![this2, out, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(calls.borrow().len(), 1);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5]);
        assert_eq!(calls.borrow()[0].1, 5);
        let (this3, _, calls) = acceptors(&mut e, [0x24, 0x28, 0xc, 0], &[1, 1, 1]);
        e.call(
            0x00e4_9cf0,
            &args![this3, out, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32],
        );
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls.borrow().len(), 3);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(calls.borrow()[0].1, 8);
        let (this4, _, _calls) = acceptors(&mut e, [0x1c, 0x20, 0x14, 4], &[1, 0]);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00e4_9d60, &args![this4, out, 9u32, 10u32]).u32(),
            args![out][0]
        );
    }

    #[test]
    fn fn_00e49cf0_answers_its_result_pointer() {
        let mut e = engine();
        let out = out_byte(&mut e);
        let (this, objects, calls) = acceptors(&mut e, [0x24, 0x28, 0xc, 4], &[1, 1]);
        e.call(0x00e4_9c20, &args![this, out, 2u32, 3u32, 4u32, 5u32, 6u32]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5, 6]);
        assert_eq!(calls.borrow()[0].0[0], objects[1] + 0xc);
        assert_eq!(calls.borrow()[0].1, 6);
        let (this2, _, calls) = acceptors(&mut e, [0x20, 0x24, 0x10, 0], &[1, 0]);
        e.call(0x00e4_9c90, &args![this2, out, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(calls.borrow().len(), 1);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5]);
        assert_eq!(calls.borrow()[0].1, 5);
        let (this3, _, _calls) = acceptors(&mut e, [0x24, 0x28, 0xc, 0], &[1, 1, 1]);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(
                0x00e4_9cf0,
                &args![this3, out, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32],
            )
            .u32(),
            args![out][0]
        );
    }

    #[test]
    fn fn_00e49c90_answers_its_result_pointer() {
        let mut e = engine();
        let out = out_byte(&mut e);
        let (this, objects, calls) = acceptors(&mut e, [0x24, 0x28, 0xc, 4], &[1, 1]);
        e.call(0x00e4_9c20, &args![this, out, 2u32, 3u32, 4u32, 5u32, 6u32]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(&calls.borrow()[0].0[2..], &[2, 3, 4, 5, 6]);
        assert_eq!(calls.borrow()[0].0[0], objects[1] + 0xc);
        assert_eq!(calls.borrow()[0].1, 6);
        let (this2, _, _calls) = acceptors(&mut e, [0x20, 0x24, 0x10, 0], &[1, 0]);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00e4_9c90, &args![this2, out, 2u32, 3u32, 4u32, 5u32])
                .u32(),
            args![out][0]
        );
    }

    #[test]
    fn fn_00e49c20_answers_its_result_pointer() {
        let mut e = engine();
        let out = out_byte(&mut e);
        let (this, _objects, _calls) = acceptors(&mut e, [0x24, 0x28, 0xc, 4], &[1, 1]);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00e4_9c20, &args![this, out, 2u32, 3u32, 4u32, 5u32, 6u32])
                .u32(),
            args![out][0]
        );
    }

    /// A router with an allocator whose allocate returns a fresh block and grants `granted`
    /// bytes, and whose free is recorded.
    fn allocator(e: &mut Engine, granted: i32) -> Calls {
        let calls = record(e, &[0x0ff0_0010]);
        e.mem.set_u32(0x0126_0010, 0x0126_0100);
        e.mem.set_u32(0x0126_0100, 0x0126_0200);
        e.put_vtable(0x0126_0200, &[0, 0, 0, 0x0ff0_000c, 0x0ff0_0010]);
        let log = Rc::clone(&calls);
        e.register_double(0x0ff0_000c, move |e, a| {
            let asked = e.mem.i32(a[1]);
            log.borrow_mut().push((0xc, vec![asked as u32]));
            e.mem.set_i32(a[1], granted);
            Ret {
                eax: e.mem.alloc(asked as u32 + 64),
                ..Ret::default()
            }
        });
        calls
    }

    #[test]
    fn array_constructor_copies_the_pointers_and_references_them() {
        let mut e = engine();
        let calls = allocator(&mut e, 12);
        let refs = record(&mut e, &[BASE_CONSTRUCTOR, ADD_REFERENCE]);
        let source = e.mem.alloc(8);
        let data = e.mem.alloc(8);
        put_u32s(&mut e, data, &[(0, 0x1000), (4, 0x2000)]);
        put_u32s(&mut e, source, &[(0, data), (4, 2)]);
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e4_9e50, &args![this, source]).u32(), this);
        assert_eq!(
            *refs.borrow(),
            vec![
                (BASE_CONSTRUCTOR, vec![this]),
                (ADD_REFERENCE, vec![0x1000]),
                (ADD_REFERENCE, vec![0x2000])
            ]
        );
        assert_eq!(*calls.borrow(), vec![(0xc, vec![8])]);
        assert_eq!(e.mem.u32(this), 0x010e_90e8);
        assert_eq!(e.mem.u32(this + 0x14), 0x010e_90b0);
        assert_eq!(e.mem.u32(this + 0x20), 3);
        assert_eq!(e.mem.u32(this + 0x34), 2);
        assert_eq!(e.mem.u32(this + 0x38), 3);
        let block = e.mem.u32(this + 0x30);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (0x1000, 0x2000));
    }

    #[test]
    fn array_constructor_with_an_empty_source_allocates_nothing() {
        let mut e = engine();
        let calls = allocator(&mut e, 0);
        record(&mut e, &[BASE_CONSTRUCTOR]);
        let source = e.mem.alloc(8);
        let this = e.mem.alloc(0x40);
        e.call(0x00e4_9e50, &args![this, source]);
        assert!(calls.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
    }

    #[test]
    fn array_destructor_releases_and_frees_an_owned_block() {
        let mut e = engine();
        let calls = allocator(&mut e, 0);
        let refs = record(&mut e, &[REMOVE_REFERENCE]);
        let this = e.mem.alloc(0x40);
        let block = e.mem.alloc(16);
        put_u32s(&mut e, block, &[(0, 0x1000), (4, 0x2000)]);
        put_u32s(&mut e, this, &[(0x30, block), (0x34, 2), (0x38, 8)]);
        e.call(0x00e4_9f40, &args![this]);
        assert_eq!(
            *refs.borrow(),
            vec![
                (REMOVE_REFERENCE, vec![0x1000]),
                (REMOVE_REFERENCE, vec![0x2000])
            ]
        );
        assert_eq!(
            *calls.borrow(),
            vec![(0x0ff0_0010, vec![0x0126_0100, block, 32])]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 8), 0x010c_3b80);
        assert_eq!(e.mem.u32(this + 0xc), 0x010c_3b98);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_3bac);
        assert_eq!(e.mem.u32(this + 0x14), 0x010c_3b8c);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
        // An unowned array is not freed and no references are held.
        put_u32s(
            &mut e,
            this,
            &[(0x30, block), (0x34, 0), (0x38, 0x8000_0008)],
        );
        e.call(0x00e4_9f40, &args![this]);
        assert_eq!(calls.borrow().len(), 1);
    }

    #[test]
    fn plain_constructor_stores_the_vtables_and_an_empty_array() {
        let mut e = engine();
        let refs = record(&mut e, &[BASE_CONSTRUCTOR]);
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e4_9fe0, &args![this]).u32(), this);
        assert_eq!(*refs.borrow(), vec![(BASE_CONSTRUCTOR, vec![this])]);
        assert_eq!(e.mem.u32(this), 0x010e_90e8);
        assert_eq!(e.mem.u32(this + 8), 0x010e_90dc);
        assert_eq!(e.mem.u32(this + 0xc), 0x010e_90c8);
        assert_eq!(e.mem.u32(this + 0x10), 0x010e_90bc);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
    }
}
