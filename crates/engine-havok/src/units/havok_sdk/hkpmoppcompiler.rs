//! `Havok SDK/hkpmoppcompiler.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The compiler object (`this` of `compile`) holds, as far as the code shows: the root result
//! at `+0`, an optional list of sub-generators at `+4` (count at `+0x14`, array of 8-byte
//! entries at `+0x10`, a flag at `+0x1C` and a fixed size in its first word), a word at `+0xC`,
//! six settings words at `+0x20` and the assembler's arguments at `+0x40`. The member names
//! are not confirmed in the exe.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): word `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMemoryRouter::easyAlloc` (`00c85690`, cdecl `(allocator, size)`) and `easyFree`
/// (`00c856c0`, cdecl `(allocator, block)`).
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkString::memCpy` (`00cb1260`, cdecl `(destination, source, size)`) and the C runtime
/// `memset` (`00ec61c0`, cdecl `(destination, byte, size)`).
const MEM_COPY: u32 = 0x00cb_1260;
const MEM_SET: u32 = 0x00ec_61c0;
/// `hkReferencedObject::removeReference` (`00c905b0`, thiscall without stack arguments).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpMoppCodeGenerator`: constructor (`00e065f0`, `RET 4`), `compileCode` (`00e064f0`) and
/// destructor (`00e06650`).
const GENERATOR_CONSTRUCTOR: u32 = 0x00e0_65f0;
const GENERATOR_COMPILE_CODE: u32 = 0x00e0_64f0;
const GENERATOR_DESTRUCTOR: u32 = 0x00e0_6650;
/// `hkpMoppDefaultAssembler`: constructor (`00e090c0`, `RET 0xC`) and destructor (`00e09450`).
const ASSEMBLER_CONSTRUCTOR: u32 = 0x00e0_90c0;
const ASSEMBLER_DESTRUCTOR: u32 = 0x00e0_9450;
/// `hkpMoppDefaultSplitter`: constructor (`00e09ba0`), `buildTree` (`00e0a550`, `RET 0x14`) and
/// destructor (`00e09bd0`).
const SPLITTER_CONSTRUCTOR: u32 = 0x00e0_9ba0;
const SPLITTER_BUILD_TREE: u32 = 0x00e0_a550;
const SPLITTER_DESTRUCTOR: u32 = 0x00e0_9bd0;
/// `00dd4b80` (thiscall, `RET 4`, `(mediator)`): the number of bytes of scratch memory the
/// compilation needs.
const REQUIRED_SIZE: u32 = 0x00dd_4b80;
/// The settings' first initializer (`00dd5150`, thiscall, `RET 4`, one word) and the copy
/// (`00dd5170`, thiscall, `RET 4`, a pointer to six words).
const SETTINGS_RESET: u32 = 0x00dd_5150;
const SETTINGS_COPY: u32 = 0x00dd_5170;
/// Vtables: the settings (`010d7ba8`), the compiled code (`0102e368`) and the base (`0102e388`).
const VTABLE_SETTINGS: u32 = 0x010d_7ba8;
const VTABLE_CODE: u32 = 0x0102_e368;
const VTABLE_BASE: u32 = 0x0102_e388;

// Translated from 00dd4bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the big-endian 32-bit word stored in the four bytes at `+1` to `+4` (the engine map has
/// no name for this function), `__fastcall`.
pub fn fn_00dd4bb0(e: &mut Engine, this: Ptr) -> u32 {
    read_command_word(e, this.addr())
}

fn read_command_word(e: &Engine, at: u32) -> u32 {
    let b = |offset: u32| u32::from(e.mem.u8(at + offset));
    (((b(1) << 8).wrapping_add(b(2)) << 8).wrapping_add(b(3)) << 8).wrapping_add(b(4))
}

// Translated from 00dd4bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reverse of `00dd4bb0` (the engine map has no name): writes `value` big-endian into the
/// bytes at `+1` to `+4`. This function is not a standard convention: `this` is in `ECX` and
/// the value in `EAX`, which has no place in the uniform form, so the value is the word after
/// `this` here.
pub fn fn_00dd4bd0(e: &mut Engine, this: Ptr, value: u32) {
    write_command_word(e, this.addr(), value);
}

fn write_command_word(e: &mut Engine, at: u32, value: u32) {
    e.mem.set_u8(at + 1, (value >> 24) as u8);
    e.mem.set_u8(at + 2, (value >> 16) as u8);
    e.mem.set_u8(at + 3, (value >> 8) as u8);
    e.mem.set_u8(at + 4, value as u8);
}

// Translated from 00dd51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the compiler settings (the engine map has no name), `RET 4` with a pointer
/// to six words: sets the 16-bit word 1 at `+6` and the vtable `010d7ba8`, runs `00dd5150(0)`
/// on the words at `+8` and then `00dd5170(settings)`. Returns `this`.
pub fn fn_00dd51a0(e: &mut Engine, this: Ptr, settings: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, VTABLE_SETTINGS);
    e.call(SETTINGS_RESET, &args![me + 8, 0u32]);
    e.call(SETTINGS_COPY, &args![this, settings]);
    this
}

/// Grows the array of 4-byte entries at `array` (data, size, capacity word) by the policy
/// the code spells out: the doubled capacity, or `wanted` when that is not more.
fn grow_words(e: &mut Engine, allocator: u32, array: u32, wanted: i32) {
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_mul(2);
        let count = if wanted < doubled { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![allocator, array, count as u32, 4u32]);
    }
}

// Translated from 00dd4cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCompiler::compile` (Xbox PDB), `RET 0xC` with `(mediator, buffer, buffer_size)`:
/// builds a code generator sized for the mediator's primitives (`slot 0x10`) plus the word at
/// `+0xC`, the settings (a copy of the six words at `+0x20`) and the default assembler, tells
/// the mediator and the assembler about each other, takes the scratch memory from `buffer`
/// when it is given and big enough for `00dd4b80` (otherwise from `easyAlloc`), splits the
/// primitives into a tree with the default splitter and stores the root at `+0`, and frees
/// the scratch memory it allocated. Then it compiles the code: with no list of
/// sub-generators at `+4` this is the generator's `compileCode`; with one, the prefix sums of
/// the generators' sizes (16-byte aligned, or the fixed size of the list) give the offset of
/// each, the jump words of each generator (big-endian words at `buffer - jump + capacity` for the
/// jumps listed at `generator + 0x14`) are replaced by the offsets, and the code of every
/// generator is copied to its offset in a new 0x30-byte code object (which gets the `0xCD` fill
/// first), and the generators are released. At the end the assembler's slot `0x18` gets the root
/// and the code's vector at `+0x10`, the assembler and the generator are destroyed and the
/// code object is returned.
pub fn hkp_mopp_compiler_compile(
    e: &mut Engine,
    this: Ptr,
    mediator: Ptr,
    buffer: u32,
    buffer_size: i32,
) -> Ptr {
    let me = this.addr();
    let p = mediator.addr();
    let code = e.with_stack(0x140, |e, frame| {
        let f = frame.addr();
        let list = f + 0x20;
        let pool = f + 0x34;
        let generator = f + 0x40;
        let settings = f + 0x60;
        let assembler = f + 0x80;
        let splitter = f + 0x104;
        let primitives = e.vcall(p, 0x10, &args![]).u32();
        let units = primitives.wrapping_add(e.mem.u32(me + 0xc));
        e.call(
            GENERATOR_CONSTRUCTOR,
            &args![generator, units.wrapping_mul(12)],
        );
        fn_00dd51a0(e, Ptr::new(settings), Ptr::new(me + 0x20));
        e.call(
            ASSEMBLER_CONSTRUCTOR,
            &args![assembler, me + 0x40, generator, p],
        );
        let user = e.mem.u32(me + 4);
        e.mem.set_u32(assembler + 0x14, user);
        let first = e.vcall(assembler, 0xc, &args![]).u32();
        let second = e.vcall(assembler, 0x10, &args![first]).u32();
        e.vcall(p, 0xc, &args![second]);
        let required = e.call(REQUIRED_SIZE, &args![this, p]).u32() as i32;
        let scratch = if buffer == 0 || buffer_size < required {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(EASY_ALLOC, &args![allocator, required as u32]).u32()
        } else {
            buffer
        };
        let nodes_end = scratch.wrapping_add(units.wrapping_mul(20));
        e.mem.set_u32(pool, scratch);
        e.mem.set_u32(pool + 4, nodes_end);
        let records = if e.mem.u8(me + 0x1c) != 0 {
            0x1000
        } else {
            units
        };
        e.mem
            .set_u32(pool + 8, records.wrapping_mul(0x7c).wrapping_add(nodes_end));
        e.vcall(p, 0x14, &args![scratch]);
        e.call(SPLITTER_CONSTRUCTOR, &args![splitter]);
        let root = e
            .call(
                SPLITTER_BUILD_TREE,
                &args![splitter, p, settings, assembler, me + 8, pool],
            )
            .u32();
        e.call(SPLITTER_DESTRUCTOR, &args![splitter]);
        if scratch != buffer {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(EASY_FREE, &args![allocator, scratch]);
        }
        e.mem.set_u32(me, root);
        let user = e.mem.u32(me + 4);
        let code = if user == 0 {
            e.call(GENERATOR_COMPILE_CODE, &args![generator]).u32()
        } else {
            join_generators(e, me, list)
        };
        e.vcall(assembler, 0x18, &args![root, code + 0x10]);
        e.call(ASSEMBLER_DESTRUCTOR, &args![assembler]);
        e.mem.set_u32(settings, VTABLE_BASE);
        e.call(GENERATOR_DESTRUCTOR, &args![generator]);
        code
    });
    Ptr::new(code)
}

/// The branch of `compile` that joins the sub-generators listed at `[this + 4]` into one code
/// object; `list` is the 12-byte array of offsets on the stack.
fn join_generators(e: &mut Engine, me: u32, list: u32) -> u32 {
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
    e.mem.set_u32(list + 8, 0x8000_0000);
    let count = e.mem.u32(e.mem.u32(me + 4) + 0x14);
    let wanted = count.wrapping_add(1) as i32;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    grow_words(e, allocator, list, wanted);
    e.mem.set_u32(list + 4, wanted as u32);
    let offsets = e.mem.u32(list);
    e.mem.set_u32(offsets, 0);
    // The offset of every generator: the running sum of the sizes.
    if (e.mem.u32(e.mem.u32(me + 4) + 0x14) as i32) > 0 {
        let mut index = 0u32;
        loop {
            let user = e.mem.u32(me + 4);
            let size = if e.mem.u32(user + 0x1c) == 0 {
                let entries = e.mem.u32(user + 0x10);
                let generator = e.mem.u32(entries + 8 * index);
                e.mem.u32(generator + 0xc).wrapping_add(0xf) & 0xffff_fff0
            } else {
                e.mem.u32(user)
            };
            let offsets = e.mem.u32(list);
            let next = e.mem.u32(offsets + 4 * index).wrapping_add(size);
            e.mem.set_u32(offsets + 4 * index + 4, next);
            index += 1;
            let user = e.mem.u32(me + 4);
            if (index as i32) >= e.mem.u32(user + 0x14) as i32 {
                break;
            }
        }
    }
    // Every jump word of every generator is replaced by the offset it names.
    let user = e.mem.u32(me + 4);
    if (e.mem.u32(user + 0x14) as i32) > 0 {
        let mut index = 0u32;
        loop {
            let user = e.mem.u32(me + 4);
            let entries = e.mem.u32(user + 0x10);
            let entry = entries + 8 * index;
            let generator = e.mem.u32(entry);
            let jumps = generator + 0x14;
            let mut jump = 0u32;
            while (jump as i32) < e.mem.u32(jumps + 4) as i32 {
                let generator = e.mem.u32(entry);
                let table = e.mem.u32(jumps);
                let at = e
                    .mem
                    .u32(generator + 0x10)
                    .wrapping_sub(e.mem.u32(table + 8 * jump))
                    .wrapping_add(e.mem.u32(generator + 8));
                let word = read_command_word(e, at);
                let offsets = e.mem.u32(list);
                let replacement = e.mem.u32(offsets.wrapping_add(word.wrapping_mul(4)));
                write_command_word(e, at, replacement);
                jump += 1;
            }
            index += 1;
            let user = e.mem.u32(me + 4);
            if (index as i32) >= e.mem.u32(user + 0x14) as i32 {
                break;
            }
        }
    }
    let size = e.mem.u32(list + 4);
    let offsets = e.mem.u32(list);
    let total = e.mem.u32(offsets + 4 * size - 4);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let code = e.vcall(allocator, 4, &args![0x30u32]).u32();
    let bytes = code + 0x20;
    e.mem.set_u16(code + 4, 0x30);
    e.mem.set_u16(code + 6, 1);
    e.mem.set_u32(code, VTABLE_CODE);
    e.mem.set_u32(bytes, 0);
    e.mem.set_u32(bytes + 4, 0);
    e.mem.set_u32(bytes + 8, 0x8000_0000);
    for word in 0..4 {
        e.mem.set_u32(code + 0x10 + 4 * word, 0);
    }
    e.mem.set_u8(code + 0x2c, 2);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = (e.mem.u32(bytes + 8) & 0x3fff_ffff) as i32;
    if capacity < total as i32 {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE, &args![allocator, bytes, total, 1u32]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(bytes + 8) & 0x3fff_ffff) as i32;
    if capacity < total as i32 {
        let doubled = capacity.wrapping_add(capacity);
        let count = if (total as i32) < doubled {
            doubled as u32
        } else {
            total
        };
        e.call(ARRAY_RESERVE, &args![allocator, bytes, count, 1u32]);
    }
    let written = e.mem.u32(bytes + 4);
    let data = e.mem.u32(bytes);
    let missing = total.wrapping_sub(written) as i32;
    if missing > 0 {
        e.call(
            MEM_SET,
            &args![data.wrapping_add(written), 0xcdu32, missing as u32],
        );
    }
    e.mem.set_u32(bytes + 4, total);
    let mut index = 0u32;
    while (index as i32) < e.mem.u32(e.mem.u32(me + 4) + 0x14) as i32 {
        let offsets = e.mem.u32(list);
        let destination = e
            .mem
            .u32(offsets + 4 * index)
            .wrapping_add(e.mem.u32(bytes));
        let user = e.mem.u32(me + 4);
        let entries = e.mem.u32(user + 0x10);
        let generator = e.mem.u32(entries + 8 * index);
        let piece = e.call(GENERATOR_COMPILE_CODE, &args![generator]).u32();
        let piece_size = e.mem.u32(piece + 0x24);
        let piece_data = e.mem.u32(piece + 0x20);
        e.call(MEM_COPY, &args![destination, piece_data, piece_size]);
        e.vcall(piece, 0, &args![1u32]);
        let user = e.mem.u32(me + 4);
        let entries = e.mem.u32(user + 0x10);
        let generator = e.mem.u32(entries + 8 * index);
        e.call(REMOVE_REFERENCE, &args![generator]);
        index += 1;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(list + 8);
    e.mem.set_u32(list + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(list);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 8, 0x8000_0000);
    code
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_4bb0, fn_00dd4bb0(Ptr) -> u32),
        entry!(0x00dd_4bd0, fn_00dd4bd0(Ptr, u32)),
        entry!(
            0x00dd_4cb0,
            hkp_mopp_compiler_compile(Ptr, Ptr, u32, i32) -> Ptr
        ),
        entry!(0x00dd_51a0, fn_00dd51a0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const ALLOCATE_SLOT: u32 = 0x0900_0601;
    const FREE_SLOT: u32 = 0x0900_0602;
    const MEDIATOR_COUNT: u32 = 0x0900_0603;
    const MEDIATOR_ACCEPT: u32 = 0x0900_0604;
    const MEDIATOR_SCRATCH: u32 = 0x0900_0605;
    const ASSEMBLER_KIND: u32 = 0x0900_0606;
    const ASSEMBLER_BLOCK: u32 = 0x0900_0607;
    const ASSEMBLER_FINISH: u32 = 0x0900_0608;
    const PIECE_DELETE: u32 = 0x0900_0609;

    fn push(log: &Log, row: Vec<u32>) {
        log.borrow_mut().push(row);
    }

    /// Everything `compile` calls outside this file, as logging stand-ins. Returns the engine,
    /// the log and the allocator.
    fn rig() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            push(&sink, vec![1, a[1]]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(FREE_SLOT, move |_, a| {
            push(&sink, vec![2, a[1], a[2]]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(EASY_ALLOC, move |e, a| {
            push(&sink, vec![3, a[0], a[1]]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(EASY_FREE, move |_, a| {
            push(&sink, vec![4, a[0], a[1]]);
            Ret::default()
        });
        e.register_double(MEM_SET, move |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register_double(MEM_COPY, move |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register_double(ARRAY_RESERVE, move |e, a| {
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(GENERATOR_CONSTRUCTOR, move |_, a| {
            push(&sink, vec![5, a[1]]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(SETTINGS_RESET, |_, _| Ret::default());
        let sink = Rc::clone(&log);
        e.register_double(SETTINGS_COPY, move |_, a| {
            push(&sink, vec![6, a[1]]);
            Ret::default()
        });
        let assembler_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(assembler_vtable + 0xc, ASSEMBLER_KIND);
        e.mem.set_u32(assembler_vtable + 0x10, ASSEMBLER_BLOCK);
        e.mem.set_u32(assembler_vtable + 0x18, ASSEMBLER_FINISH);
        e.register_double(ASSEMBLER_CONSTRUCTOR, move |e, a| {
            e.mem.set_u32(a[0], assembler_vtable);
            Ret::default()
        });
        e.register_double(ASSEMBLER_KIND, |_, _| Ret {
            eax: 0xd,
            ..Ret::default()
        });
        e.register_double(ASSEMBLER_BLOCK, |_, a| Ret {
            eax: a[1] + 0x100,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ASSEMBLER_FINISH, move |_, a| {
            push(&sink, vec![7, a[1], a[2]]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ASSEMBLER_DESTRUCTOR, move |_, _| {
            push(&sink, vec![8]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(GENERATOR_DESTRUCTOR, move |_, _| {
            push(&sink, vec![9]);
            Ret::default()
        });
        e.register_double(SPLITTER_CONSTRUCTOR, |_, _| Ret::default());
        let sink = Rc::clone(&log);
        e.register_double(SPLITTER_DESTRUCTOR, move |_, _| {
            push(&sink, vec![10]);
            Ret::default()
        });
        (e, log, allocator)
    }

    /// A mediator whose slots 0x10 (count), 0xC (accept the block id, logged) and 0x14
    /// (receive the scratch memory, logged) are stand-ins.
    fn mediator(e: &mut Engine, log: &Log, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, MEDIATOR_ACCEPT);
        e.mem.set_u32(vtable + 0x10, MEDIATOR_COUNT);
        e.mem.set_u32(vtable + 0x14, MEDIATOR_SCRATCH);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.register_double(MEDIATOR_COUNT, move |_, _| Ret {
            eax: count,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(MEDIATOR_ACCEPT, move |_, a| {
            push(&sink, vec![11, a[1]]);
            Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(MEDIATOR_SCRATCH, move |_, a| {
            push(&sink, vec![12, a[1]]);
            Ret::default()
        });
        object
    }

    #[test]
    fn the_command_word_reads_and_writes_big_endian_at_one_to_four() {
        let (mut e, _log, _allocator) = rig();
        let at = e.mem.alloc(8);
        for (i, byte) in [0xaau8, 0x12, 0x34, 0x56, 0x78, 0xbb].iter().enumerate() {
            e.mem.set_u8(at + i as u32, *byte);
        }
        assert_eq!(e.call(0x00dd4bb0, &args![at]).u32(), 0x1234_5678);
        e.call(0x00dd4bd0, &args![at, 0xdead_beefu32]);
        let bytes: Vec<u8> = (0..6).map(|i| e.mem.u8(at + i)).collect();
        assert_eq!(bytes, vec![0xaa, 0xde, 0xad, 0xbe, 0xef, 0xbb]);
    }

    #[test]
    fn the_settings_constructor_sets_the_vtable_and_runs_its_two_initializers() {
        let (mut e, log, _allocator) = rig();
        let this = e.mem.alloc(0x40);
        let settings = e.mem.alloc(0x20);
        let sink = Rc::clone(&log);
        e.register_double(SETTINGS_RESET, move |_, a| {
            push(&sink, vec![20, a[0], a[1]]);
            Ret::default()
        });
        let got = e.call(0x00dd51a0, &args![this, settings]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), VTABLE_SETTINGS);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(
            *log.borrow(),
            vec![vec![20, this + 8, 0], vec![6, settings]]
        );
    }

    #[test]
    fn compile_without_sub_generators_uses_the_given_buffer_and_the_generators_code() {
        let (mut e, log, _allocator) = rig();
        let p = mediator(&mut e, &log, 4);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u8(this + 0x1c, 0);
        e.register_double(REQUIRED_SIZE, |_, _| Ret {
            eax: 0x200,
            ..Ret::default()
        });
        let buffer = e.mem.alloc(0x400);
        let code = e.mem.alloc(0x40);
        let root = 0x7777u32;
        let pool: Rc<RefCell<Vec<u32>>> = Rc::default();
        let seen = Rc::clone(&pool);
        let sink = Rc::clone(&log);
        e.register_double(SPLITTER_BUILD_TREE, move |e, a| {
            push(&sink, vec![13, a[1], a[4]]);
            *seen.borrow_mut() = (0..3).map(|i| e.mem.u32(a[5] + 4 * i)).collect();
            Ret {
                eax: root,
                ..Ret::default()
            }
        });
        e.register_double(GENERATOR_COMPILE_CODE, move |_, _| Ret {
            eax: code,
            ..Ret::default()
        });
        let got = e.call(0x00dd4cb0, &args![this, p, buffer, 0x400u32]).u32();
        assert_eq!(got, code);
        assert_eq!(e.mem.u32(this), root);
        // 4 primitives plus 3 more: 7 units of 12 bytes for the generator.
        assert_eq!(
            *log.borrow(),
            vec![
                vec![5, 84],
                vec![6, this + 0x20],
                vec![11, 0x10d],
                vec![12, buffer],
                vec![13, p, this + 8],
                vec![10],
                vec![7, root, code + 0x10],
                vec![8],
                vec![9],
            ]
        );
        // The pool: the scratch memory, the end of 7 nodes of 20 bytes, and 7 records of 0x7C.
        assert_eq!(
            *pool.borrow(),
            vec![buffer, buffer + 140, buffer + 140 + 7 * 0x7c]
        );
    }

    #[test]
    fn compile_allocates_scratch_when_the_buffer_is_too_small_and_frees_it_again() {
        let (mut e, log, allocator) = rig();
        let p = mediator(&mut e, &log, 1);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u8(this + 0x1c, 1);
        e.register_double(REQUIRED_SIZE, |_, _| Ret {
            eax: 0x200,
            ..Ret::default()
        });
        let buffer = e.mem.alloc(0x100);
        let code = e.mem.alloc(0x40);
        let pool: Rc<RefCell<Vec<u32>>> = Rc::default();
        let seen = Rc::clone(&pool);
        e.register_double(SPLITTER_BUILD_TREE, move |e, a| {
            *seen.borrow_mut() = (0..3).map(|i| e.mem.u32(a[5] + 4 * i)).collect();
            Ret::default()
        });
        e.register_double(GENERATOR_COMPILE_CODE, move |_, _| Ret {
            eax: code,
            ..Ret::default()
        });
        e.call(0x00dd4cb0, &args![this, p, buffer, 0x1ffu32]);
        let log = log.borrow();
        assert!(log.contains(&vec![3, allocator, 0x200]));
        let scratch = pool.borrow()[0];
        assert_ne!(scratch, buffer);
        assert!(log.contains(&vec![4, allocator, scratch]));
        // The flag at +0x1C makes the record count 0x1000.
        assert_eq!(pool.borrow()[2], scratch + 40 + 0x1000 * 0x7c);
    }

    #[test]
    fn compile_with_sub_generators_patches_the_jumps_and_joins_the_pieces() {
        let (mut e, log, _allocator) = rig();
        let p = mediator(&mut e, &log, 2);
        let this = e.mem.alloc(0x80);
        e.register_double(REQUIRED_SIZE, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        let buffer = e.mem.alloc(0x200);
        e.register_double(SPLITTER_BUILD_TREE, |_, _| Ret {
            eax: 0x4242,
            ..Ret::default()
        });
        // Two sub-generators, sizes 20 and 5 (rounded up to 32 and 16).
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        e.mem.set_u32(first + 0xc, 20);
        e.mem.set_u32(second + 0xc, 5);
        // The first has one jump: its word is at `[+0x10] - table + [+8]`, here the start of `code_bytes`.
        let code_bytes = e.mem.alloc(0x20);
        for (i, byte) in [0x00u8, 0, 0, 0, 1].iter().enumerate() {
            e.mem.set_u8(code_bytes + i as u32, *byte);
        }
        e.mem.set_u32(first + 0x10, code_bytes + 0x10);
        e.mem.set_u32(first + 8, 0);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, 0x10);
        e.mem.set_u32(first + 0x14, table);
        e.mem.set_u32(first + 0x18, 1);
        let entries = e.mem.alloc(16);
        e.mem.set_u32(entries, first);
        e.mem.set_u32(entries + 8, second);
        let user = e.mem.alloc(0x40);
        e.mem.set_u32(user + 0x14, 2);
        e.mem.set_u32(user + 0x10, entries);
        e.mem.set_u32(user + 0x1c, 0);
        e.mem.set_u32(this + 4, user);
        // The pieces compileCode returns, in order: 4 bytes, then 2 bytes.
        let piece_vtable = e.mem.alloc(8);
        e.mem.set_u32(piece_vtable, PIECE_DELETE);
        let mut pieces = Vec::new();
        for bytes in [vec![1u8, 2, 3, 4], vec![9u8, 8]] {
            let piece = e.mem.alloc(0x30);
            let data = e.mem.alloc(8);
            for (i, byte) in bytes.iter().enumerate() {
                e.mem.set_u8(data + i as u32, *byte);
            }
            e.mem.set_u32(piece, piece_vtable);
            e.mem.set_u32(piece + 0x20, data);
            e.mem.set_u32(piece + 0x24, bytes.len() as u32);
            pieces.push(piece);
        }
        let queue = Rc::new(RefCell::new(pieces.clone().into_iter()));
        let sink = Rc::clone(&log);
        e.register_double(GENERATOR_COMPILE_CODE, move |_, a| {
            push(&sink, vec![14, a[0]]);
            Ret {
                eax: queue.borrow_mut().next().unwrap_or(0),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(PIECE_DELETE, move |_, a| {
            push(&sink, vec![15, a[0], a[1]]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            push(&sink, vec![16, a[0]]);
            Ret::default()
        });
        let got = e.call(0x00dd4cb0, &args![this, p, buffer, 0x200u32]).u32();
        // The jump word 1 became the offset of the second generator: 32.
        let patched: Vec<u8> = (0..5).map(|i| e.mem.u8(code_bytes + i)).collect();
        assert_eq!(patched, vec![0, 0, 0, 0, 0x20]);
        assert_eq!(e.mem.u32(this), 0x4242);
        // The joined code: 48 bytes, 0xCD where nothing was copied.
        assert_eq!(e.mem.u32(got), VTABLE_CODE);
        assert_eq!(e.mem.u16(got + 4), 0x30);
        assert_eq!(e.mem.u32(got + 0x24), 48);
        let data = e.mem.u32(got + 0x20);
        let joined: Vec<u8> = (0..48).map(|i| e.mem.u8(data + i)).collect();
        let mut expected = vec![0xcdu8; 48];
        expected[..4].copy_from_slice(&[1, 2, 3, 4]);
        expected[32..34].copy_from_slice(&[9, 8]);
        assert_eq!(joined, expected);
        let log = log.borrow();
        let order: Vec<Vec<u32>> = log
            .iter()
            .filter(|row| matches!(row[0], 14..=16))
            .cloned()
            .collect();
        assert_eq!(
            order,
            vec![
                vec![14, first],
                vec![15, pieces[0], 1],
                vec![16, first],
                vec![14, second],
                vec![15, pieces[1], 1],
                vec![16, second],
            ]
        );
        // The offsets array (3 words) is freed with its byte size, and the finish call gets
        // the code's vector.
        assert!(log.iter().any(|r| r[0] == 2 && r[2] == 12));
        assert!(log.contains(&vec![7, 0x4242, got + 0x10]));
    }
}
