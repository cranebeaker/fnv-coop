//! `fallout shared/facegen/bsfacegenmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `tolower` (LIBCMT, `00ec67aa`).
const TOLOWER: u32 = 0x00ec_67aa;

// Translated from 00600b20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<..>::KeyToHashIndex` (Xbox PDB; `thiscall`,
/// `RET 4`): the bucket of a key. Each byte up to the terminator, lowered
/// (`tolower` on the sign-extended byte), is folded in by [`fn_00659210`];
/// the hash modulo the map's bucket count (+4, unsigned).
pub fn key_to_hash_index(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut hash = 0u32;
    let mut p = key;
    loop {
        let c = e.mem.u8(p) as i8 as i32;
        if c == 0 {
            break;
        }
        let lowered = e.call(TOLOWER, &args![c as u32]).u32() as u8;
        hash = fn_00659210(e, this, lowered, hash);
        p = p.wrapping_add(1);
    }
    hash % e.mem.u32(this.addr() + 4)
}

// Translated from 00659210 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// One step of the string hash (`thiscall`, `RET 8`; `this` unused):
/// `hash * 33 + c`, the byte sign-extended (`MOVSX`).
pub fn fn_00659210(_e: &mut Engine, _this: Ptr, c: u8, hash: u32) -> u32 {
    (hash << 5)
        .wrapping_add(hash)
        .wrapping_add(c as i8 as i32 as u32)
}

/// The `__stricmp` wrapper (`00404dc0`, `cdecl`).
const STRICMP_WRAPPER: u32 = 0x0040_4dc0;

// Translated from 00486b50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<..>::IsKeysEqual` (Xbox PDB; `thiscall`,
/// `RET 8`; `this` unused): whether the keys are equal ignoring case
/// (`00404dc0` answers 0).
pub fn is_keys_equal(e: &mut Engine, _this: Ptr, a: u32, b: u32) -> bool {
    e.call(STRICMP_WRAPPER, &args![a, b]).u32() == 0
}

/// `006532c0`: thiscall (`this`, one word), returns a bool in AL.
const CHECK_006532C0: u32 = 0x0065_32c0;

// Translated from 00653290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bsfacegenmanager.cpp`). Thiscall
/// (`RET 4`): `this` when `006532c0(this, argument)` is true, otherwise null.
pub fn fn_00653290(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    if e.call(CHECK_006532C0, &args![this, argument]).bool() {
        this
    } else {
        Ptr::NULL
    }
}

/// The function `fn_00657fd0` calls on the member (`00658100`).
const CALLEE_00658100: u32 = 0x0065_8100;

/// The function `fn_00658040` calls on the member (`00658130`).
const CALLEE_00658130: u32 = 0x0065_8130;

// Translated from 00657fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00658100` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_00657fd0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00658100, &args![this.byte_add(0x8)]).u32()
}

// Translated from 00658040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00658130` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_00658040(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00658130, &args![this.byte_add(0x8)]).u32()
}

/// `BSFaceGenManager::GetInstance` (Xbox PDB name of `00651b30`; no
/// arguments): the manager singleton.
const FACE_GEN_GET_INSTANCE: u32 = 0x0065_1b30;
/// `009611e0` (`ECX` = matrix): the word at `+0x18`.
const MATRIX_FIRST_DIMENSION: u32 = 0x0096_11e0;
/// `PathingLocation::GetWorldspace` (Xbox PDB name of `00441110`; `ECX` =
/// matrix): the word at `+0x1c`.
const MATRIX_SECOND_DIMENSION: u32 = 0x0044_1110;
/// `std::vector<float>::resize` (engine map name of `0060b340`; `ECX` =
/// matrix, two words): stores them at `+0x18` and `+0x1c` and sizes the
/// storage for their product.
const MATRIX_RESIZE: u32 = 0x0060_b340;
/// `FR2MatrixVTC<float>::operator=` (Xbox PDB name of `006581e0`; `ECX` =
/// destination matrix, source matrix): copies the dimensions and the values.
const MATRIX_ASSIGN: u32 = 0x0065_81e0;

// Translated from 006521e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::GetFaceGenCoord` (Xbox PDB; no arguments): the address
/// of the coordinate block, 8 bytes into the manager singleton
/// (`BSFaceGenManager::GetInstance`, `00651b30`).
pub fn bs_face_gen_manager_get_face_gen_coord(e: &mut Engine) -> u32 {
    e.call(FACE_GEN_GET_INSTANCE, &[]).u32() + 8
}

// Translated from 006529a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::CopyFaceGenCoord` (Xbox PDB; `cdecl`: source block,
/// destination block, two bytes). Both blocks hold two rows of two 0x20-byte
/// matrices (row `i`, column `j` at `i * 0x40 + j * 0x20`). Does nothing for
/// a null block. For each matrix: when the source has both dimensions
/// non-zero, the destination takes the source's dimensions, and its values
/// too when `first` is non-zero and the row is 0, `second` is non-zero and the
/// row is 1, or the two bytes are equal; otherwise the destination is sized
/// 0 by 0.
pub fn bs_face_gen_manager_copy_face_gen_coord(
    e: &mut Engine,
    source: u32,
    destination: u32,
    first: u8,
    second: u8,
) {
    if source == 0 || destination == 0 {
        return;
    }
    for row in 0..2u32 {
        for column in 0..2u32 {
            let from = source + row * 0x40 + column * 0x20;
            let to = destination + row * 0x40 + column * 0x20;
            if e.call(MATRIX_FIRST_DIMENSION, &args![from]).u32() != 0
                && e.call(MATRIX_SECOND_DIMENSION, &args![from]).u32() != 0
            {
                let columns = e.call(MATRIX_SECOND_DIMENSION, &args![from]).u32();
                let rows = e.call(MATRIX_FIRST_DIMENSION, &args![from]).u32();
                e.call(MATRIX_RESIZE, &args![to, rows, columns]);
                if (first != 0 && row == 0) || (second != 0 && row == 1) || first == second {
                    e.call(MATRIX_ASSIGN, &args![to, from]);
                }
            } else {
                e.call(MATRIX_RESIZE, &args![to, 0u32, 0u32]);
            }
        }
    }
}

// Translated from 00658bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01065870` (RTTI `.?AV?$BSTCaseInsensitiveStringMap@V?$NiPointer@VEntry@BSFaceGenModelMap@@@@@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00658d30`. One word, plain `RET`, returns nothing.
pub fn fn_00658bb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_5870);
    e.call(0x0065_8d30, &args![this]);
}

/// `NiTPointerListBase<..,NiPointer<NiProperty>>::RemovePos` (`00b9efa0`, unnamed in the
/// engine map; `thiscall` on the list: the address of the returned `NiPointer`, the address of
/// the position).
const REMOVE_POSITION_00B9EFA0: u32 = 0x00b9_efa0;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00b9fcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,NiPointer<NiProperty> >::Remove`
/// (engine map), `thiscall` with two stack words: the address of the returned `NiPointer`
/// and the address of the element to remove. Walks the list from its head (+0; items: next
/// +0, element +8) for an item holding the element. When one is found it is removed through
/// `00b9efa0` (with a temporary `NiPointer` and the position) and the result is the removed
/// element, the temporary released afterwards (`InterlockedDecrement` on its +4, deleted
/// through vtable slot +4 at 0); otherwise the result is the element asked for. The result is
/// stored in the returned pointer with its count at +4 incremented when not null. Answers the
/// returned pointer's address.
// Translation incomplete: 00b9fcc0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the temporary) is not translated.
pub fn ni_t_pointer_list_base_remove(e: &mut Engine, this: Ptr, result: Ptr, element: Ptr) -> Ptr {
    let wanted = e.mem.u32(element.addr());
    let mut item = e.mem.u32(this.addr());
    let found = loop {
        if item == 0 {
            break 0;
        }
        let next = e.mem.u32(item);
        if wanted == e.mem.u32(item + 8) {
            break item;
        }
        item = next;
    };
    e.with_stack(8, |e, locals| {
        let temporary = locals.addr();
        let position = temporary + 4;
        e.mem.set_u32(position, found);
        let source = if found != 0 {
            e.call(REMOVE_POSITION_00B9EFA0, &args![this, temporary, position])
                .u32()
        } else {
            element.addr()
        };
        let value = e.mem.u32(source);
        e.mem.set_u32(result.addr(), value);
        if value != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![value + 4]);
        }
        if found != 0 {
            let removed = e.mem.u32(temporary);
            if removed != 0 {
                let left = e
                    .call(IMPORT_INTERLOCKED_DECREMENT, &args![removed + 4])
                    .u32();
                if left == 0 {
                    e.vcall(removed, 4, &args![]);
                }
            }
        }
    });
    result
}

/// The list's remove-head `00b9e7c0` and remove-tail `00b9e850` (unit `shadowscenelight.cpp`,
/// unnamed; `thiscall` on the list, one word: the address of the returned `NiPointer`) and its
/// item release (`00e6e830`, `thiscall` on the list, the item; the engine map names it
/// `NiTPointerMap<..>::DeleteItem`).
const REMOVE_HEAD_00B9E7C0: u32 = 0x00b9_e7c0;
const REMOVE_TAIL_00B9E850: u32 = 0x00b9_e850;
const FREE_ITEM_00E6E830: u32 = 0x00e6_e830;

// Translated from 00b9efa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,NiPointer<NiProperty> >::RemovePos`
/// (engine map), `thiscall` with two stack words: the address of the returned `NiPointer` and
/// the address of the position (an item pointer). At the head (+0) the position moves to the
/// next item and `00b9e7c0` removes the head into the result; at the tail (+4) the position
/// becomes null and `00b9e850` removes the tail into it. Otherwise the position moves to the
/// next item, the neighbours are linked to each other, the item's element (+8) is held by a
/// temporary `NiPointer` (count at +4 up), the item released (`00e6e830`), the list's count
/// (+8) goes down, the result takes the element (held again) and the temporary lets it go
/// (`InterlockedDecrement`, deleted through vtable slot +4 at 0). Answers the result's address.
// Translation incomplete: 00b9efa0 - the C++ exception frame (FS:[0] chain, unwind state 1 for the temporary) is not translated.
pub fn ni_t_pointer_list_base_remove_pos(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    position: Ptr,
) -> Ptr {
    let list = this.addr();
    let item = e.mem.u32(position.addr());
    if item == e.mem.u32(list) {
        let next = e.mem.u32(item);
        e.mem.set_u32(position.addr(), next);
        e.call(REMOVE_HEAD_00B9E7C0, &args![list, result]);
        return result;
    }
    if item == e.mem.u32(list + 4) {
        e.mem.set_u32(position.addr(), 0);
        e.call(REMOVE_TAIL_00B9E850, &args![list, result]);
        return result;
    }
    let previous = e.mem.u32(item + 4);
    let next = e.mem.u32(item);
    e.mem.set_u32(position.addr(), next);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    let element = e.mem.u32(item + 8);
    if element != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![element + 4]);
    }
    e.call(FREE_ITEM_00E6E830, &args![list, item]);
    let count = e.mem.u32(list + 8).wrapping_sub(1);
    e.mem.set_u32(list + 8, count);
    e.mem.set_u32(result.addr(), element);
    if element != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![element + 4]);
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![element + 4])
            .u32();
        if left == 0 {
            e.vcall(element, 4, &args![]);
        }
    }
    result
}

// --- Recorded game, batches b049 and b050 ---

/// `BSFaceGenModelMap`'s virtual table (RTTI) and `BSFaceGenModelMap::Entry`'s.
const MODEL_MAP_VTABLE: u32 = 0x0106_5678;
const MODEL_MAP_ENTRY_VTABLE: u32 = 0x0106_569c;
/// The face-gen lock (`011d5a9c`; `004538e0` takes it, `thiscall` with one word,
/// `0044b130` tries it, `0082f1f0` releases it) and its depth counter (`011d59dc`).
const FACE_GEN_LOCK: u32 = 0x011d_5a9c;
const FACE_GEN_LOCK_DEPTH: u32 = 0x011d_59dc;
const LOCK_TAKE: u32 = 0x0045_38e0;
const LOCK_TRY: u32 = 0x0044_b130;
const LOCK_RELEASE: u32 = 0x0082_f1f0;
/// `NiPointer<T>` helpers (`thiscall` on the pointer): the constructor from a raw pointer
/// (`00633c90`, one word), the destructor (`0045cec0`), the dereference (`00559450`), the
/// assignment from a raw pointer (`0066b0d0`, one word), the copy constructor (`00559a40`,
/// one word: the source pointer's address) and the assignment from another `NiPointer`
/// (`006e5cc0`, one word: its address).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_COPY: u32 = 0x0055_9a40;
const POINTER_ASSIGN_POINTER: u32 = 0x006e_5cc0;
/// The model map's own map (+4) operations (`thiscall`): its constructor (`00658aa0`: bucket
/// count, flag), clean-up (`00438af0`), `RemoveAt` (`00405430`, the key) and look-up
/// (`006c62d0`: key, `NiPointer` slot for the entry).
const MAP_CONSTRUCT: u32 = 0x0065_8aa0;
const MAP_CLEAN_UP: u32 = 0x0043_8af0;
const MAP_REMOVE_AT: u32 = 0x0040_5430;
const MAP_LOOKUP: u32 = 0x006c_62d0;
/// The game's frame counter (`00457fe0`, no arguments), kept in an entry's +0xc when used.
const FRAME_COUNTER: u32 = 0x0045_7fe0;
/// `00650ba0` (`thiscall` on the model map, one word: an entry or 0): the entry's touch.
const MODEL_MAP_TOUCH: u32 = 0x0065_0ba0;
/// `0065bf80` and `0065c2c0` (`thiscall` on a model, no stack words): its two attachments.
const MODEL_ATTACH_0065BF80: u32 = 0x0065_bf80;
const MODEL_ATTACH_0065C2C0: u32 = 0x0065_c2c0;
/// The name source's two getters (`006505d0`, `00650600`, `thiscall`, no stack words) and
/// `0044a670` (cdecl, one word: a string), its length.
const NAME_FIRST: u32 = 0x0065_05d0;
const NAME_SECOND: u32 = 0x0065_0600;
const STRING_LENGTH: u32 = 0x0044_a670;
/// The allocation (`00aa13e0`, cdecl: size), the sized free (`00aa1460`, cdecl: block, size),
/// the plain free (`00401030`, cdecl: block).
const ALLOCATE: u32 = 0x00aa_13e0;
const FREE_SIZED: u32 = 0x00aa_1460;
const FREE: u32 = 0x0040_1030;
/// The entry's base constructor and destructor bodies (`004968b0`, `00496910`, `thiscall`).
const ENTRY_BASE_CONSTRUCT: u32 = 0x0049_68b0;
const ENTRY_BASE_DESTRUCT: u32 = 0x0049_6910;
/// `` `eh vector constructor iterator' `` (`00ec782f`, `stdcall`: array, element size,
/// count, constructor, destructor) and `` `eh vector destructor iterator' `` (`00ec5fce`,
/// `stdcall`: array, element size, count, destructor).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The element constructors and destructors those iterators get: `00651520`/`006515b0`,
/// `00651630`/`00651650`, `00651670`/`006516a0` (this unit) and `00449610`/`00449680`.
const ELEMENT_CONSTRUCT_00449610: u32 = 0x0044_9610;
const ELEMENT_DESTRUCT_00449680: u32 = 0x0044_9680;

// Translated from 0064fd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSFaceGenModelMap` constructor (`thiscall`, no stack
/// words; the virtual table it stores has that RTTI): the map at +4 (`00658aa0` with 0x25
/// buckets and 1), the two words +0x18 and +0x1c set to 0x10000000 and the byte +0x20
/// cleared. Answers `this`.
pub fn fn_0064fd80(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, MODEL_MAP_VTABLE);
    e.call(MAP_CONSTRUCT, &args![base.wrapping_add(4), 0x25u32, 1u32]);
    for index in 0..2u32 {
        e.mem
            .set_u32(base.wrapping_add(0x18 + index * 4), 0x1000_0000);
    }
    e.mem.set_u8(base.wrapping_add(0x20), 0);
    this
}

// Translated from 0064fde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelMap`'s scalar deleting destructor (engine map name; `thiscall`, one word of
/// flags): the destructor body `0064fe10`, then with flag bit 0 the block freed
/// (`00401030`). Answers `this`.
pub fn bs_face_gen_model_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0064fe10(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0064fe10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064fe10 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the `BSFaceGenModelMap` destructor body (`thiscall`, no stack
/// words): its virtual table, the map at +4 cleaned up (`00438af0`) and destroyed
/// (`00658bb0`).
pub fn fn_0064fe10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, MODEL_MAP_VTABLE);
    e.call(MAP_CLEAN_UP, &args![base.wrapping_add(4)]);
    fn_00658bb0(e, Ptr::new(base.wrapping_add(4)));
}

/// Runs `body` with an `NiPointer` the exe keeps on its stack: constructed null
/// (`00633c90` with 0) before and destroyed (`0045cec0`) after.
fn with_pointer<R>(e: &mut Engine, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.call(POINTER_CONSTRUCT, &args![slot, 0u32]);
        let result = body(e, slot);
        e.call(POINTER_DESTRUCT, &args![slot]);
        result
    })
}

/// The object an `NiPointer` at `slot` holds (`00559450`).
fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

// Translated from 0064ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ffd0 - the FS:[0] exception frame (C++ unwinding of the stack NiPointer and the new entry) is not translated
/// Unnamed in the engine map: sets the model of `key` in the model map (`thiscall`, two
/// words: the key and the model), under the face-gen lock. Without a model the key is
/// removed from the map (`RemoveAt`). With one, a new 0x10-byte entry (`00650100`) is held
/// in a stack `NiPointer`, gets the model (+8) and the frame counter (+0xc), goes into the
/// map by value (`00658950`, through an `NiPointer` copy in the argument word) and is touched
/// (`00650ba0`).
pub fn fn_0064ffd0(e: &mut Engine, this: Ptr, key: u32, model: u32) {
    let base = this.addr();
    fn_00652140(e);
    with_pointer(e, |e, entry| {
        if model == 0 {
            e.call(MAP_REMOVE_AT, &args![base.wrapping_add(4), key]);
            fn_00652190(e);
            return;
        }
        let block = e.call(ALLOCATE, &args![0x10u32]).u32();
        let made = if block != 0 {
            fn_00650100(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.call(POINTER_ASSIGN, &args![entry, made]);
        let held = pointer_get(e, entry);
        e.call(POINTER_ASSIGN, &args![held.wrapping_add(8), model]);
        let frame = e.call(FRAME_COUNTER, &args![]).u32();
        let held = pointer_get(e, entry);
        e.mem.set_u32(held.wrapping_add(0xc), frame);
        let copy = e.with_stack(4, |e, argument| {
            e.call(POINTER_COPY, &args![argument, entry]);
            e.mem.u32(argument.addr())
        });
        fn_00658950(e, Ptr::new(base.wrapping_add(4)), key, copy);
        let held = pointer_get(e, entry);
        e.call(MODEL_MAP_TOUCH, &args![this, held]);
        fn_00652190(e);
    });
}

// Translated from 00650100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650100 - the FS:[0] exception frame (C++ unwinding during construction) is not translated
/// Unnamed in the engine map: the `BSFaceGenModelMap::Entry` constructor (`thiscall`, no
/// stack words): the base constructor `004968b0`, the entry's virtual table and its model
/// `NiPointer` at +8 constructed null. Answers `this`.
pub fn fn_00650100(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(ENTRY_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, MODEL_MAP_ENTRY_VTABLE);
    e.call(POINTER_CONSTRUCT, &args![base.wrapping_add(8), 0u32]);
    this
}

// Translated from 00650170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelMap::Entry`'s scalar deleting destructor (engine map name; `thiscall`, one
/// word of flags): the destructor body `006501a0`, then with flag bit 0 the 0x10-byte block
/// freed (`00aa1460`). Answers `this`.
pub fn bs_face_gen_model_map_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006501a0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x10u32]);
    }
    this
}

// Translated from 006501a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006501a0 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the `BSFaceGenModelMap::Entry` destructor body (`thiscall`, no
/// stack words): the model `NiPointer` at +8 destroyed, then the base destructor `00496910`.
pub fn fn_006501a0(e: &mut Engine, this: Ptr) {
    e.call(POINTER_DESTRUCT, &args![this.addr().wrapping_add(8)]);
    e.call(ENTRY_BASE_DESTRUCT, &args![this]);
}

// Translated from 00650200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650200 - the FS:[0] exception frame (C++ unwinding of the stack NiPointer) is not translated
/// Unnamed in the engine map: looks `key` up in the model map (`thiscall`, two words: the key
/// and an `NiPointer` to receive the model), under the face-gen lock. A found entry gets the
/// frame counter (+0xc), its model goes into `model` (`NiPointer` assignment), the entry is
/// touched (`00650ba0`), the model's `0065bf80` and `0065c2c0` run and the map's byte +0x20
/// is set. Answers whether it was found.
pub fn fn_00650200(e: &mut Engine, this: Ptr, key: u32, model: u32) -> bool {
    let base = this.addr();
    fn_00652140(e);
    with_pointer(e, |e, entry| {
        let mut found = false;
        if e.call(MAP_LOOKUP, &args![base.wrapping_add(4), key, entry])
            .bool()
        {
            let frame = e.call(FRAME_COUNTER, &args![]).u32();
            let held = pointer_get(e, entry);
            e.mem.set_u32(held.wrapping_add(0xc), frame);
            let held = pointer_get(e, entry);
            e.call(POINTER_ASSIGN_POINTER, &args![model, held.wrapping_add(8)]);
            let held = pointer_get(e, entry);
            e.call(MODEL_MAP_TOUCH, &args![this, held]);
            let object = pointer_get(e, model);
            e.call(MODEL_ATTACH_0065BF80, &args![object]);
            let object = pointer_get(e, model);
            e.call(MODEL_ATTACH_0065C2C0, &args![object]);
            e.mem.set_u8(base.wrapping_add(0x20), 1);
            found = true;
        }
        fn_00652190(e);
        found
    })
}

/// The shared body of `00650480` and `00650630`: looks the name source's `first` name up,
/// else its `second` name, under the face-gen lock; a found entry gets the frame counter, is
/// touched, its model gets `attach`, and the map's byte +0x20 is set.
fn touch_named(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    first: u32,
    second: u32,
    attach: u32,
) -> bool {
    let base = this.addr();
    with_pointer(e, |e, entry| {
        if source == 0 || e.call(first, &args![source]).u32() == 0 {
            return false;
        }
        let name = e.call(first, &args![source]).u32();
        if e.call(STRING_LENGTH, &args![name]).u32() == 0 {
            return false;
        }
        let mut found = false;
        fn_00652140(e);
        let name = e.call(first, &args![source]).u32();
        let mut hit = e
            .call(MAP_LOOKUP, &args![base.wrapping_add(4), name, entry])
            .bool();
        if !hit && e.call(second, &args![source]).u32() != 0 {
            let name = e.call(second, &args![source]).u32();
            hit = e
                .call(MAP_LOOKUP, &args![base.wrapping_add(4), name, entry])
                .bool();
        }
        if hit {
            let frame = e.call(FRAME_COUNTER, &args![]).u32();
            let held = pointer_get(e, entry);
            e.mem.set_u32(held.wrapping_add(0xc), frame);
            let held = pointer_get(e, entry);
            e.call(MODEL_MAP_TOUCH, &args![this, held]);
            let held = pointer_get(e, entry);
            let model = pointer_get(e, held.wrapping_add(8));
            e.call(attach, &args![model]);
            e.mem.set_u8(base.wrapping_add(0x20), 1);
            found = true;
        }
        fn_00652190(e);
        found
    })
}

// Translated from 00650480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650480 - the FS:[0] exception frame (C++ unwinding of the stack NiPointer) is not translated
/// Unnamed in the engine map: touches the model map's entry for the name source `source`
/// (`thiscall`, one word). Nothing (false) without a source or without a non-empty first name
/// (`006505d0`, `0044a670`). Otherwise, under the face-gen lock, the first name is looked up,
/// else the second (`00650600`) when there is one; a found entry gets the frame counter, is
/// touched (`00650ba0`), its model gets `0065bf80`, and the map's byte +0x20 is set. Answers
/// whether it was found.
pub fn fn_00650480(e: &mut Engine, this: Ptr, source: u32) -> bool {
    touch_named(
        e,
        this,
        source,
        NAME_FIRST,
        NAME_SECOND,
        MODEL_ATTACH_0065BF80,
    )
}

// Translated from 00650630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650630 - the FS:[0] exception frame (C++ unwinding of the stack NiPointer) is not translated
/// Unnamed in the engine map: `00650480` with the name source's names the other way round
/// (`00650600` first, then `006505d0`) and `0065c2c0` for the found model (`thiscall`, one
/// word).
pub fn fn_00650630(e: &mut Engine, this: Ptr, source: u32) -> bool {
    touch_named(
        e,
        this,
        source,
        NAME_SECOND,
        NAME_FIRST,
        MODEL_ATTACH_0065C2C0,
    )
}

// Translated from 006508e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the model map's word `index` (from +0x18) to `value`,
/// then touches the map with no entry (`00650ba0` with 0) (`thiscall`, two words).
pub fn fn_006508e0(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let slot = this
        .addr()
        .wrapping_add(0x18)
        .wrapping_add(index.wrapping_mul(4));
    e.mem.set_u32(slot, value);
    e.call(MODEL_MAP_TOUCH, &args![this, 0u32]);
}

// Translated from 00651450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00651450 - the FS:[0] exception frame (C++ unwinding of the array constructions) is not translated
/// Unnamed in the engine map: a constructor (`thiscall`, no stack words): the byte +0 cleared,
/// then four arrays built by `` `eh vector constructor iterator' ``: five 0xa0-byte elements
/// at +4 (`00651520`/`006515b0`), twenty 0x28-byte ones at +0x324 (`00651630`/`00651650`),
/// twenty-five 0x48-byte ones at +0x644 (`00651670`/`006516a0`) and twenty 0x20-byte ones at
/// +0xd4c (`00449610`/`00449680`). Answers `this`.
pub fn fn_00651450(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u8(base, 0);
    for (offset, size, count, construct, destruct) in [
        (4u32, 0xa0u32, 5u32, 0x0065_1520u32, 0x0065_15b0u32),
        (0x324, 0x28, 0x14, 0x0065_1630, 0x0065_1650),
        (0x644, 0x48, 0x19, 0x0065_1670, 0x0065_16a0),
        (
            0xd4c,
            0x20,
            0x14,
            ELEMENT_CONSTRUCT_00449610,
            ELEMENT_DESTRUCT_00449680,
        ),
    ] {
        e.call(
            VECTOR_CONSTRUCT,
            &args![base.wrapping_add(offset), size, count, construct, destruct],
        );
    }
    this
}

// Translated from 00651520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00651520 - the FS:[0] exception frame (C++ unwinding of the array constructions) is not translated
/// Unnamed in the engine map: the 0xa0-byte element's constructor (`thiscall`, no stack
/// words): two 0x20-byte elements at +0 (`00449610`/`00449680` by the vector constructor
/// iterator), one at +0x40 (`00449610`) and two at +0x60. Answers `this`.
pub fn fn_00651520(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let pair = |e: &mut Engine, at: u32| {
        e.call(
            VECTOR_CONSTRUCT,
            &args![
                at,
                0x20u32,
                2u32,
                ELEMENT_CONSTRUCT_00449610,
                ELEMENT_DESTRUCT_00449680
            ],
        );
    };
    pair(e, base);
    e.call(ELEMENT_CONSTRUCT_00449610, &args![base.wrapping_add(0x40)]);
    pair(e, base.wrapping_add(0x60));
    this
}

// Translated from 006515b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006515b0 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the 0xa0-byte element's destructor (`thiscall`, no stack
/// words): the two elements at +0x60 (`` `eh vector destructor iterator' `` with
/// `00449680`), the one at +0x40, then the two at +0.
pub fn fn_006515b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(
        VECTOR_DESTRUCT,
        &args![
            base.wrapping_add(0x60),
            0x20u32,
            2u32,
            ELEMENT_DESTRUCT_00449680
        ],
    );
    e.call(ELEMENT_DESTRUCT_00449680, &args![base.wrapping_add(0x40)]);
    e.call(
        VECTOR_DESTRUCT,
        &args![base, 0x20u32, 2u32, ELEMENT_DESTRUCT_00449680],
    );
}

// Translated from 00652140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: takes the face-gen lock (`004538e0` on `011d5a9c` with 0) and
/// counts the depth up (`011d59dc`). No arguments (callers leave `ECX` set, which it never
/// reads).
pub fn fn_00652140(e: &mut Engine) {
    e.call(LOCK_TAKE, &args![FACE_GEN_LOCK, 0u32]);
    let depth = e.mem.u32(FACE_GEN_LOCK_DEPTH);
    e.mem.set_u32(FACE_GEN_LOCK_DEPTH, depth.wrapping_add(1));
}

// Translated from 00652160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: tries the face-gen lock (`0044b130` on `011d5a9c`); when that
/// answers exactly 1 the depth (`011d59dc`) counts up. Answers what the try answered. No
/// arguments.
pub fn fn_00652160(e: &mut Engine) -> u8 {
    let taken = e.call(LOCK_TRY, &args![FACE_GEN_LOCK]).u8();
    if taken == 1 {
        let depth = e.mem.u32(FACE_GEN_LOCK_DEPTH);
        e.mem.set_u32(FACE_GEN_LOCK_DEPTH, depth.wrapping_add(1));
    }
    taken
}

// Translated from 00652190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: releases the face-gen lock (`0082f1f0` on `011d5a9c`) and
/// counts the depth (`011d59dc`) down. No arguments.
pub fn fn_00652190(e: &mut Engine) {
    e.call(LOCK_RELEASE, &args![FACE_GEN_LOCK]);
    let depth = e.mem.u32(FACE_GEN_LOCK_DEPTH);
    e.mem.set_u32(FACE_GEN_LOCK_DEPTH, depth.wrapping_sub(1));
}

/// The memory-context scope the manager's functions open (`00404eb0`, `thiscall` on a stack
/// word: context, 1, source file name, line) and close (`00404ee0`), with the source file name
/// `"D:\_Fallout3\...\FaceGen\BSFaceGenManager.cpp"` (`01065720`).
const SCOPE_OPEN: u32 = 0x0040_4eb0;
const SCOPE_CLOSE: u32 = 0x0040_4ee0;
const SOURCE_FILE_NAME: u32 = 0x0106_5720;
/// The manager singleton (`011d59e8`), the plain allocation (`00401000`, cdecl: size) and the
/// flag the constructor sets at its end (`011d59e0`).
const MANAGER: u32 = 0x011d_59e8;
const ALLOCATE_PLAIN: u32 = 0x0040_1000;
const MANAGER_BUILT_011D59E0: u32 = 0x011d_59e0;
/// `std::basic_string<char>` constructor from a C string (`0044a4c0`, `thiscall`, one word)
/// and destructor (`0044a540`); `fanReadCtlFile` (`00aaa7d0`, cdecl, six words); the
/// control file name it reads (`0106570c`) and the message printed by `005b5e40` (cdecl, one
/// word) when both reads fail (`010656ec`).
const STD_STRING_CONSTRUCT: u32 = 0x0044_a4c0;
const STD_STRING_DESTRUCT: u32 = 0x0044_a540;
const FAN_READ_CTL_FILE: u32 = 0x00aa_a7d0;
const CTL_FILE_NAME: u32 = 0x0106_570c;
const PRINT_005B5E40: u32 = 0x005b_5e40;
const CTL_FAILED_MESSAGE: u32 = 0x0106_56ec;
/// The constructor's other callees: the 0x1c-byte object (`004391c0`) and its three setters
/// (`004393c0`, `004393e0`, `0094db80`, `thiscall`, one word), `NiPixelData::NiPixelData`
/// (`00a7c190`, `thiscall`, five words) with the pixel format at `011aa2a0`, the pixel data's
/// pixels (`0064dac0`, `thiscall`: 0, 0), `memset`-like `00403d30` (cdecl: block, value,
/// count), `004f31b0` (`thiscall`), a fixed string (`00438170`, `thiscall`, one word, and
/// `004381b0`, its destructor; `0043b1b0` its text), `NiSourceTexture::Create` (`00a5ff10`,
/// cdecl: pixel data, name, preferences `011a9598`) and the two texture names (`010656cc`,
/// `010656ac`).
const OBJECT_CONSTRUCT_004391C0: u32 = 0x0043_91c0;
const OBJECT_SET_004393C0: u32 = 0x0043_93c0;
const OBJECT_SET_004393E0: u32 = 0x0043_93e0;
const OBJECT_SET_0094DB80: u32 = 0x0094_db80;
const PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
const PIXEL_FORMAT_011AA2A0: u32 = 0x011a_a2a0;
const PIXEL_DATA_PIXELS: u32 = 0x0064_dac0;
const FILL_00403D30: u32 = 0x0040_3d30;
const PIXEL_DATA_STEP_004F31B0: u32 = 0x004f_31b0;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const FIXED_STRING_TEXT: u32 = 0x0043_b1b0;
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_ff10;
const TEXTURE_PREFERENCES_011A9598: u32 = 0x011a_9598;
const FIRST_TEXTURE_NAME: u32 = 0x0106_56cc;
const SECOND_TEXTURE_NAME: u32 = 0x0106_56ac;

/// Runs `body` inside the memory-context scope (`00404eb0` with `context`, 1, the source file
/// name and `line` on a stack word; `00404ee0` after).
fn in_scope<R>(e: &mut Engine, context: u32, line: u32, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_OPEN,
            &args![scope, context, 1u32, SOURCE_FILE_NAME, line],
        );
        let result = body(e);
        e.call(SCOPE_CLOSE, &args![scope]);
        result
    })
}

/// `fanReadCtlFile` on the control file name (a stack `std::string`) with the manager's parts;
/// answers whether it failed (its byte result 0).
fn read_ctl_file(e: &mut Engine, base: u32) -> bool {
    e.with_stack(0x1c, |e, name| {
        e.call(STD_STRING_CONSTRUCT, &args![name, CTL_FILE_NAME]);
        let read = e
            .call(
                FAN_READ_CTL_FILE,
                &args![
                    name,
                    base,
                    base.wrapping_add(4),
                    base.wrapping_add(0x1184),
                    base.wrapping_add(0xb8),
                    base.wrapping_add(0x118)
                ],
            )
            .u8();
        e.call(STD_STRING_DESTRUCT, &args![name]);
        read == 0
    })
}

/// A 32 by 32 `NiPixelData` (`00a7c190` on a 0x74-byte block, or none) for the constructor's
/// two default textures; answers it and its pixels (`0064dac0`).
fn new_pixel_data(e: &mut Engine) -> (u32, u32) {
    let block = e.call(ALLOCATE, &args![0x74u32]).u32();
    let data = if block != 0 {
        e.call(
            PIXEL_DATA_CONSTRUCT,
            &args![block, 0x20u32, 0x20u32, PIXEL_FORMAT_011AA2A0, 1u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    let pixels = e.call(PIXEL_DATA_PIXELS, &args![data, 0u32, 0u32]).u32();
    (data, pixels)
}

/// Makes a source texture of `data` named `name` and keeps it in the `NiPointer` at `slot`.
fn keep_texture(e: &mut Engine, data: u32, name: u32, slot: u32) {
    e.call(PIXEL_DATA_STEP_004F31B0, &args![data]);
    fn_006516c0(e, Ptr::new(data), 1);
    e.with_stack(4, |e, text| {
        let string = e.call(FIXED_STRING_CONSTRUCT, &args![text, name]).u32();
        let chars = e.call(FIXED_STRING_TEXT, &args![string]).u32();
        let texture = e
            .call(
                SOURCE_TEXTURE_CREATE,
                &args![data, chars, TEXTURE_PREFERENCES_011A9598],
            )
            .u32();
        e.call(POINTER_ASSIGN, &args![slot, texture]);
        e.call(FIXED_STRING_DESTRUCT, &args![text]);
    });
}

// Translated from 00650ea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650ea0 - the FS:[0] exception frame (C++ unwinding during construction) is not translated
/// `BSFaceGenManager::BSFaceGenManager` (Xbox PDB; `thiscall`, no stack words). Its parts are
/// constructed (`006516e0` on `this`, `006517e0` at +0xb8, `00651450` at +0x118, three null
/// `NiPointer`s at +0x1198, +0x119c, +0x11a0) and, inside the memory-context scope (0x37, line
/// 0x2b4), the control file is read (`fanReadCtlFile`), once more when it fails, with a message
/// (`005b5e40`) when that fails too. Each of the 2 x 2 coordinate matrices (+8, row `i` at
/// `i * 0x40`, column `j` at `j * 0x20`) takes its size from the matching statistics entry
/// (+0xb8, `i * 0x30 + j * 0x18`; `00657a30`, `00657a50`): the entry's `00441110` and
/// `009611e0` words, nothing for a null entry, 0 by 0 for an empty one. The model map
/// (+0x1194) is cleared, a 0x1c-byte object (`004391c0`) goes into +0x1198 with its setters
/// (1, 4, 100), and two 32 by 32 textures are made: the first cleared to 0x80, the second
/// filled with the bytes 0x3e, 0x41, 0x3e, 0x40, kept at +0x119c and +0x11a0. `011d59e0` is
/// set. Answers `this`.
pub fn bs_face_gen_manager_bs_face_gen_manager(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    fn_006516e0(e, this);
    fn_006517e0(e, Ptr::new(base.wrapping_add(0xb8)));
    fn_00651450(e, Ptr::new(base.wrapping_add(0x118)));
    for offset in [0x1198u32, 0x119c, 0x11a0] {
        e.call(POINTER_CONSTRUCT, &args![base.wrapping_add(offset), 0u32]);
    }
    in_scope(e, 0x37, 0x2b4, |e| {
        if read_ctl_file(e, base) && read_ctl_file(e, base) {
            e.call(PRINT_005B5E40, &args![CTL_FAILED_MESSAGE]);
        }
        for row in 0..2u32 {
            for column in 0..2u32 {
                let entries = Ptr::new(base.wrapping_add(0xb8 + row * 0x30 + column * 0x18));
                let matrix = base.wrapping_add(8 + row * 0x40 + column * 0x20);
                if fn_00657a30(e, entries) != 0 {
                    let entry = fn_00657a50(e, entries, 0);
                    if entry != 0 {
                        let first = e.call(MATRIX_FIRST_DIMENSION, &args![entry]).u32();
                        let second = e.call(MATRIX_SECOND_DIMENSION, &args![entry]).u32();
                        e.call(MATRIX_RESIZE, &args![matrix, second, first]);
                    }
                } else {
                    e.call(MATRIX_RESIZE, &args![matrix, 0u32, 0u32]);
                }
            }
        }
        e.mem.set_u32(base.wrapping_add(0x1194), 0);
        let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let object = if block != 0 {
            e.call(OBJECT_CONSTRUCT_004391C0, &args![block]).u32()
        } else {
            0
        };
        let slot = base.wrapping_add(0x1198);
        e.call(POINTER_ASSIGN, &args![slot, object]);
        for (setter, value) in [
            (OBJECT_SET_004393C0, 1u32),
            (OBJECT_SET_004393E0, 4),
            (OBJECT_SET_0094DB80, 0x64),
        ] {
            let object = pointer_get(e, slot);
            e.call(setter, &args![object, value]);
        }

        let (data, pixels) = new_pixel_data(e);
        e.call(FILL_00403D30, &args![pixels, 0x80u32, 0x1000u32]);
        keep_texture(e, data, FIRST_TEXTURE_NAME, base.wrapping_add(0x119c));

        let (data, pixels) = new_pixel_data(e);
        for index in 0..0x400u32 {
            let pixel = pixels.wrapping_add(index * 4);
            e.mem.set_u8(pixel, 0x3e);
            e.mem.set_u8(pixel.wrapping_add(1), 0x41);
            e.mem.set_u8(pixel.wrapping_add(2), 0x3e);
            e.mem.set_u8(pixel.wrapping_add(3), 0x40);
        }
        keep_texture(e, data, SECOND_TEXTURE_NAME, base.wrapping_add(0x11a0));
        e.mem.set_u8(MANAGER_BUILT_011D59E0, 1);
    });
    this
}

// Translated from 00651630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x28-byte element's constructor (`thiscall`, no stack
/// words): `00449610` on `this`. Answers `this`.
pub fn fn_00651630(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ELEMENT_CONSTRUCT_00449610, &args![this]);
    this
}

// Translated from 00651650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x28-byte element's destructor (`thiscall`, no stack
/// words): `00449680` on `this`.
pub fn fn_00651650(e: &mut Engine, this: Ptr) {
    e.call(ELEMENT_DESTRUCT_00449680, &args![this]);
}

// Translated from 00651670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x48-byte element's constructor (`thiscall`, no stack
/// words): two 0x20-byte elements (`00449610`/`00449680` by the vector constructor
/// iterator). Answers `this`.
pub fn fn_00651670(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            this,
            0x20u32,
            2u32,
            ELEMENT_CONSTRUCT_00449610,
            ELEMENT_DESTRUCT_00449680
        ],
    );
    this
}

// Translated from 006516a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x48-byte element's destructor (`thiscall`, no stack
/// words): its two 0x20-byte elements (`00449680` by the vector destructor iterator).
pub fn fn_006516a0(e: &mut Engine, this: Ptr) {
    e.call(
        VECTOR_DESTRUCT,
        &args![this, 0x20u32, 2u32, ELEMENT_DESTRUCT_00449680],
    );
}

// Translated from 006516c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the byte +0x70 (`thiscall`, one word, its low byte).
pub fn fn_006516c0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x70), value);
}

// Translated from 006516e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006516e0 - the FS:[0] exception frame (C++ unwinding during construction) is not translated
/// Unnamed in the engine map: the constructor of the manager's first part (`thiscall`, no
/// stack words): four 0x20-byte coordinate matrices at +8 (`00449610`/`00449680` by the
/// vector constructor iterator), the vector at +0x88 (`00657a80`) and the one at +0xa0
/// (`00657b50`). Answers `this`.
pub fn fn_006516e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base.wrapping_add(8),
            0x20u32,
            4u32,
            ELEMENT_CONSTRUCT_00449610,
            ELEMENT_DESTRUCT_00449680
        ],
    );
    fn_00657a80(e, Ptr::new(base.wrapping_add(0x88)));
    fn_00657b50(e, Ptr::new(base.wrapping_add(0xa0)));
    this
}

// Translated from 00651760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00651760 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the destructor of the manager's first part (`thiscall`, no
/// stack words): the vector at +0xa0 (`0065f060`), the one at +0x88 (`00657af0`), then the
/// four coordinate matrices at +8 (`00449680` by the vector destructor iterator).
pub fn fn_00651760(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0065_f060, &args![base.wrapping_add(0xa0)]);
    fn_00657af0(e, Ptr::new(base.wrapping_add(0x88)));
    e.call(
        VECTOR_DESTRUCT,
        &args![
            base.wrapping_add(8),
            0x20u32,
            4u32,
            ELEMENT_DESTRUCT_00449680
        ],
    );
}

// Translated from 006517e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of the manager's statistics (`thiscall`, no
/// stack words): four 0x18-byte vectors (`00657960`/`006579d0` by the vector constructor
/// iterator). Answers `this`.
pub fn fn_006517e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![this, 0x18u32, 4u32, 0x0065_7960u32, 0x0065_79d0u32],
    );
    this
}

// Translated from 00651810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor of the manager's statistics (`thiscall`, no
/// stack words): its four vectors (`006579d0` by the vector destructor iterator).
pub fn fn_00651810(e: &mut Engine, this: Ptr) {
    e.call(VECTOR_DESTRUCT, &args![this, 0x18u32, 4u32, 0x0065_79d0u32]);
}

// Translated from 00651830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00651830 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the destructor of `00651450`'s object (`thiscall`, no stack
/// words): its arrays from the last down (+0xd4c with `00449680`, +0x644 with `006516a0`,
/// +0x324 with `00651650`, +4 with `006515b0`, by the vector destructor iterator).
pub fn fn_00651830(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, size, count, destruct) in [
        (0xd4cu32, 0x20u32, 0x14u32, ELEMENT_DESTRUCT_00449680),
        (0x644, 0x48, 0x19, 0x0065_16a0),
        (0x324, 0x28, 0x14, 0x0065_1650),
        (4, 0xa0, 5, 0x0065_15b0),
    ] {
        e.call(
            VECTOR_DESTRUCT,
            &args![base.wrapping_add(offset), size, count, destruct],
        );
    }
}

// Translated from 006518e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006518e0 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the `BSFaceGenManager` destructor body (`thiscall`, no stack
/// words): `00651e30`; the model map (+0x1194), when there is one, gets `00650a30` with 0 and
/// is deleted (its virtual slot 0 with 1) and cleared; the `NiPointer`s at +0x11a0, +0x119c,
/// +0x1198 are destroyed, then the parts at +0x118 (`00651830`), +0xb8 (`00651810`) and +0
/// (`00651760`).
pub fn fn_006518e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_00651e30(e);
    let map = e.mem.u32(base.wrapping_add(0x1194));
    if map != 0 {
        e.call(0x0065_0a30, &args![map, 0u32]);
    }
    let map = e.mem.u32(base.wrapping_add(0x1194));
    if map != 0 {
        e.vcall(map, 0, &args![1u32]);
    }
    e.mem.set_u32(base.wrapping_add(0x1194), 0);
    for offset in [0x11a0u32, 0x119c, 0x1198] {
        e.call(POINTER_DESTRUCT, &args![base.wrapping_add(offset)]);
    }
    fn_00651830(e, Ptr::new(base.wrapping_add(0x118)));
    fn_00651810(e, Ptr::new(base.wrapping_add(0xb8)));
    fn_00651760(e, this);
}

// Translated from 006519f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006519f0 - the FS:[0] exception frame (C++ unwinding of the singleton's construction) is not translated
/// `BSFaceGenManager::InitializeSystem` (Xbox PDB; no arguments): inside the memory-context
/// scope (0x37, line 0x316), when there is no manager yet (`011d59e8`), a 0x11a4-byte block
/// (`00401000`) is constructed (`BSFaceGenManager::BSFaceGenManager`, or null) and kept as the
/// manager.
pub fn bs_face_gen_manager_initialize_system(e: &mut Engine) {
    in_scope(e, 0x37, 0x316, |e| {
        if e.mem.u32(MANAGER) != 0 {
            return;
        }
        let block = e.call(ALLOCATE_PLAIN, &args![0x11a4u32]).u32();
        let manager = if block != 0 {
            bs_face_gen_manager_bs_face_gen_manager(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(MANAGER, manager);
    });
}

// Translated from 00651ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: deletes the manager (`011d59e8`) when there is one (its
/// scalar deleting destructor `00651b00` with 1) and clears it. No arguments.
pub fn fn_00651ab0(e: &mut Engine) {
    let manager = e.mem.u32(MANAGER);
    if manager == 0 {
        return;
    }
    let manager = e.mem.u32(MANAGER);
    if manager != 0 {
        fn_00651b00(e, Ptr::new(manager), 1);
    }
    e.mem.set_u32(MANAGER, 0);
}

// Translated from 00651b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the manager's scalar deleting destructor (`thiscall`, one word
/// of flags): the destructor body `006518e0`, then with flag bit 0 the block freed
/// (`00401030`). Answers `this`.
pub fn fn_00651b00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006518e0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00651b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::GetInstance` (Xbox PDB; no arguments): the manager (`011d59e8`), made
/// first by `BSFaceGenManager::InitializeSystem` when there is none.
pub fn bs_face_gen_manager_get_instance(e: &mut Engine) -> u32 {
    if e.mem.u32(MANAGER) == 0 {
        bs_face_gen_manager_initialize_system(e);
    }
    e.mem.u32(MANAGER)
}

/// The manager's model map (+0x1194).
fn model_map(e: &mut Engine) -> u32 {
    let manager = bs_face_gen_manager_get_instance(e);
    e.mem.u32(manager.wrapping_add(0x1194))
}

// Translated from 00651b50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00651b50 - the FS:[0] exception frame (C++ unwinding of the stack NiPointer and the new model) is not translated
/// Unnamed in the engine map: finds or makes the face-gen model for `first_key` or
/// `second_key` (cdecl, seven words), inside the memory-context scope (0x37, line 0x3e5),
/// through a stack `NiPointer`. Null when both keys are null. A missing model map (+0x1194 of
/// the manager) is made first (`0064fd80` on a 0x24-byte block) with its words 0 and 1 from
/// `004503f0` on `011d59f4` and `011d5afc`. A model found under `first_key` (`00650200`) gets
/// `0065a760` with `second_key` when it has none (`00650600`) and is answered; without a
/// first key one found under the second key is answered. Otherwise, with `make` set, a new
/// model (`00659ae0` on a 0x10-byte block) is built (`00659c70` with the keys, `a2`, `a3`,
/// `a6` and `a7`); a built one goes into the map under the first key, else the second
/// (`0064ffd0`), a failed one is dropped. Answers the pointer's object.
#[allow(clippy::too_many_arguments)]
pub fn fn_00651b50(
    e: &mut Engine,
    first_key: u32,
    a2: u32,
    a3: u32,
    second_key: u32,
    make: u8,
    a6: u32,
    a7: u8,
) -> u32 {
    in_scope(e, 0x37, 0x3e5, |e| {
        with_pointer(e, |e, model| {
            if first_key == 0 && second_key == 0 {
                return 0;
            }
            let manager = bs_face_gen_manager_get_instance(e);
            if e.mem.u32(manager.wrapping_add(0x1194)) == 0 {
                let block = e.call(ALLOCATE_PLAIN, &args![0x24u32]).u32();
                let map = if block != 0 {
                    fn_0064fd80(e, Ptr::new(block)).addr()
                } else {
                    0
                };
                let manager = bs_face_gen_manager_get_instance(e);
                e.mem.set_u32(manager.wrapping_add(0x1194), map);
                for (index, source) in [(0u32, 0x011d_59f4u32), (1, 0x011d_5afc)] {
                    let value = e.call(0x0045_03f0, &args![source]).u32();
                    let map = model_map(e);
                    fn_006508e0(e, Ptr::new(map), index, value);
                }
            }
            if first_key != 0 {
                let map = model_map(e);
                if fn_00650200(e, Ptr::new(map), first_key, model) {
                    if second_key != 0 {
                        let object = pointer_get(e, model);
                        if e.call(NAME_SECOND, &args![object]).u32() == 0 {
                            let object = pointer_get(e, model);
                            e.call(0x0065_a760, &args![object, second_key]);
                        }
                    }
                    return pointer_get(e, model);
                }
            } else if second_key != 0 {
                let map = model_map(e);
                if fn_00650200(e, Ptr::new(map), second_key, model) {
                    return pointer_get(e, model);
                }
            }
            if make != 0 {
                let block = e.call(ALLOCATE, &args![0x10u32]).u32();
                let made = if block != 0 {
                    fn_00659ae0(e, Ptr::new(block)).addr()
                } else {
                    0
                };
                e.call(POINTER_ASSIGN, &args![model, made]);
                let object = pointer_get(e, model);
                let built = e
                    .call(
                        0x0065_9c70,
                        &args![object, first_key, a2, a3, second_key, a6, u32::from(a7)],
                    )
                    .bool();
                if built {
                    let key = if first_key != 0 {
                        first_key
                    } else {
                        second_key
                    };
                    let object = pointer_get(e, model);
                    let map = model_map(e);
                    fn_0064ffd0(e, Ptr::new(map), key, object);
                } else {
                    e.call(POINTER_ASSIGN, &args![model, 0u32]);
                }
            }
            pointer_get(e, model)
        })
    })
}

// Translated from 00651e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: with a manager (`011d59e8`) and a model map (+0x1194), the
/// map's `006503b0` (the engine map names it `BSShaderUtil::UpdateDebugBoundVis`;
/// `thiscall`, no stack words). No arguments.
pub fn fn_00651e30(e: &mut Engine) {
    let manager = e.mem.u32(MANAGER);
    if manager == 0 || e.mem.u32(manager.wrapping_add(0x1194)) == 0 {
        return;
    }
    let manager = e.mem.u32(MANAGER);
    let map = e.mem.u32(manager.wrapping_add(0x1194));
    e.call(0x0065_03b0, &args![map]);
}

// Translated from 00651e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00651e70 - the FS:[0] exception frame (C++ unwinding of the stack NiPointer) is not translated
/// Unnamed in the engine map: walks the model map (`thiscall` on it, no stack words) under
/// the face-gen lock: from the map's first position (`004b9ba0` on +4) each entry is fetched
/// (`00559120`: position, key and `NiPointer` slots) and an entry's model (+8) gets
/// `0065c3a0`.
pub fn fn_00651e70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_00652140(e);
    let first = e.call(0x004b_9ba0, &args![base.wrapping_add(4)]).u32();
    e.with_stack(8, |e, slots| {
        let position = slots.addr();
        let key = position + 4;
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            with_pointer(e, |e, entry| {
                e.call(
                    0x0055_9120,
                    &args![base.wrapping_add(4), position, key, entry],
                );
                if pointer_get(e, entry) != 0 {
                    let held = pointer_get(e, entry);
                    let model = pointer_get(e, held.wrapping_add(8));
                    if model != 0 {
                        e.call(0x0065_c3a0, &args![model]);
                    }
                }
            });
        }
    });
    fn_00652190(e);
}

// Translated from 00651f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: with a manager and a model map, walks the map (`00651e70`). No
/// arguments.
pub fn fn_00651f40(e: &mut Engine) {
    let manager = e.mem.u32(MANAGER);
    if manager == 0 || e.mem.u32(manager.wrapping_add(0x1194)) == 0 {
        return;
    }
    let manager = e.mem.u32(MANAGER);
    let map = e.mem.u32(manager.wrapping_add(0x1194));
    fn_00651e70(e, Ptr::new(map));
}

/// `00651f70` and `00652040`: inside the memory-context scope, `touch` on the model map with
/// `source` when there is a manager and a map, else false.
fn touch_in_scope(
    e: &mut Engine,
    context: u32,
    line: u32,
    source: u32,
    touch: fn(&mut Engine, Ptr, u32) -> bool,
) -> bool {
    in_scope(e, context, line, |e| {
        let manager = e.mem.u32(MANAGER);
        if manager == 0 || e.mem.u32(manager.wrapping_add(0x1194)) == 0 {
            return false;
        }
        let manager = e.mem.u32(MANAGER);
        let map = e.mem.u32(manager.wrapping_add(0x1194));
        touch(e, Ptr::new(map), source)
    })
}

// Translated from 00651f70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00651f70 - the FS:[0] exception frame (C++ unwinding of the scope) is not translated
/// Unnamed in the engine map: inside the memory-context scope (0x38, line 0x44c), the model
/// map's `00650480` with `source` (cdecl, one word); false without a manager or a map.
pub fn fn_00651f70(e: &mut Engine, source: u32) -> bool {
    touch_in_scope(e, 0x38, 0x44c, source, fn_00650480)
}

// Translated from 00652040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00652040 - the FS:[0] exception frame (C++ unwinding of the scope) is not translated
/// Unnamed in the engine map: inside the memory-context scope (0x39, line 0x45a), the model
/// map's `00650630` with `source` (cdecl, one word); false without a manager or a map.
pub fn fn_00652040(e: &mut Engine, source: u32) -> bool {
    touch_in_scope(e, 0x39, 0x45a, source, fn_00650630)
}

// Translated from 00652110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::GetModelCache` (Xbox PDB; no arguments): the manager's model map
/// (+0x1194), 0 without a manager.
pub fn bs_face_gen_manager_get_model_cache(e: &mut Engine) -> u32 {
    let manager = e.mem.u32(MANAGER);
    if manager == 0 {
        return 0;
    }
    e.mem.u32(manager.wrapping_add(0x1194))
}

/// The manager's statistics vector for row `row`, column `column` (+0xb8, `row * 0x30 +
/// column * 0x18`).
fn statistics(e: &mut Engine, row: u32, column: u32) -> Ptr {
    let manager = bs_face_gen_manager_get_instance(e);
    Ptr::new(
        manager
            .wrapping_add(row.wrapping_mul(0x30))
            .wrapping_add(0xb8)
            .wrapping_add(column.wrapping_mul(0x18)),
    )
}

// Translated from 006521b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the number of statistics entries for `row`, `column` (cdecl,
/// two words; `00657a30` on the manager's vector) minus one.
pub fn fn_006521b0(e: &mut Engine, row: u32, column: u32) -> i32 {
    let entries = statistics(e, row, column);
    fn_00657a30(e, entries).wrapping_sub(1)
}

// Translated from 006521f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::InitFaceGenCoord` (Xbox PDB; cdecl, one word): sizes a coordinate
/// block's matrices 50 by 1, 30 by 1 (+0x20) and 50 by 1 (+0x40); nothing for null.
pub fn bs_face_gen_manager_init_face_gen_coord(e: &mut Engine, block: u32) {
    if block == 0 {
        return;
    }
    e.call(MATRIX_RESIZE, &args![block, 0x32u32, 1u32]);
    e.call(
        MATRIX_RESIZE,
        &args![block.wrapping_add(0x20), 0x1eu32, 1u32],
    );
    e.call(
        MATRIX_RESIZE,
        &args![block.wrapping_add(0x40), 0x32u32, 1u32],
    );
}

/// A matrix's row (`0060b370`, `thiscall`: a 8-byte result slot and the row) and that row's
/// element (`00629ab0`, `thiscall`, one word): the element's address.
fn matrix_element(e: &mut Engine, matrix: u32, row: u32, column: u32) -> u32 {
    e.with_stack(8, |e, slot| {
        let line = e.call(0x0060_b370, &args![matrix, slot, row]).u32();
        e.call(0x0062_9ab0, &args![line, column]).u32()
    })
}

// Translated from 00652230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00652230 - the FS:[0] exception frame (C++ unwinding of the stack matrix) is not translated
/// Unnamed in the engine map: statistic `index` of the coordinate block `block` for `row`,
/// `column` (cdecl, four words; the result in `ST0`): 0.0 for a null block or an index past
/// the manager's entries; otherwise entry `index` (`00657a50`) times the block's matrix
/// (`00658360` into a stack matrix), whose element (0, 0) is answered (the stack matrix then
/// destroyed, `00449680`).
pub fn fn_00652230(e: &mut Engine, block: u32, row: u32, column: u32, index: u32) -> f32 {
    let mut result = 0.0f32.to_bits();
    if block != 0 {
        let entries = statistics(e, row, column);
        if index < fn_00657a30(e, entries) as u32 {
            let matrix = block
                .wrapping_add(row.wrapping_mul(0x40))
                .wrapping_add(column.wrapping_mul(0x20));
            let entries = statistics(e, row, column);
            let entry = fn_00657a50(e, entries, index);
            e.with_stack(0x20, |e, product| {
                let product = fn_00658360(e, Ptr::new(entry), product, Ptr::new(matrix));
                let element = matrix_element(e, product.addr(), 0, 0);
                result = x87_quiet_bits(e.mem.u32(element));
                e.call(ELEMENT_DESTRUCT_00449680, &args![product]);
            });
        }
    }
    f32::from_bits(result)
}

// Translated from 00652440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00aabdb0` on the manager's +0x118 with 0, `a2`, `a3` and
/// `a1` (cdecl, three words), whose `ST0` it leaves as its result.
pub fn fn_00652440(e: &mut Engine, a1: u32, a2: u32, a3: u32) -> f64 {
    let manager = bs_face_gen_manager_get_instance(e);
    e.call(
        0x00aa_bdb0,
        &args![manager.wrapping_add(0x118), 0u32, a2, a3, a1],
    )
    .f64()
}

/// The coordinate scaling of `00652af0`: when `limit` is above 0.0 and below the square root
/// (`004019b0`) of the matrix's length (`006588a0`) over `rows`, the matrix is scaled
/// (`00658650`) by `limit` over that root.
fn limit_matrix(e: &mut Engine, matrix: u32, rows: u32, limit: u32) {
    let zero: f64 = e.global(0x0101_2060);
    if !(x87_load32(limit) > zero) {
        return;
    }
    let length = e.call(0x0065_88a0, &args![matrix]).f64();
    let quotient = x87_store32(x87_div(length, f64::from(rows)));
    let root = e.call(0x0040_19b0, &args![quotient]).f64();
    let root = x87_store32(root);
    if x87_load32(limit) < x87_load32(root) {
        let scale = x87_store32(x87_div(x87_load32(limit), x87_m32(root)));
        e.call(0x0065_8650, &args![matrix, scale]);
    }
}

// Translated from 00652af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: combines two coordinate blocks into `destination` (cdecl, five
/// words: the blocks, the destination, a byte and a `float` limit). Nothing without a
/// destination or without both sources; with one source it is copied
/// (`BSFaceGenManager::CopyFaceGenCoord` with 0, 0). Otherwise each of the 2 x 2 matrices
/// takes the first source's size (or 0 by 0 when that has none); row 1 with the byte set
/// takes the first source's values, the others the element-wise sums of both (`006578b0`,
/// `006578e0` after `0044a0a0`), each then limited (`limit_matrix`).
pub fn fn_00652af0(
    e: &mut Engine,
    first: u32,
    second: u32,
    destination: u32,
    copy_row_1: u8,
    limit: f32,
) {
    if destination == 0 || (first == 0 && second == 0) {
        return;
    }
    if first == 0 {
        bs_face_gen_manager_copy_face_gen_coord(e, second, destination, 0, 0);
        return;
    }
    if second == 0 {
        bs_face_gen_manager_copy_face_gen_coord(e, first, destination, 0, 0);
        return;
    }
    let limit = limit.to_bits();
    for row in 0..2u32 {
        for column in 0..2u32 {
            let offset = row * 0x40 + column * 0x20;
            let from = first.wrapping_add(offset);
            let other = second.wrapping_add(offset);
            let to = destination.wrapping_add(offset);
            let rows = e.call(MATRIX_FIRST_DIMENSION, &args![from]).u32();
            let columns = e.call(MATRIX_SECOND_DIMENSION, &args![from]).u32();
            if rows == 0 || columns == 0 {
                e.call(MATRIX_RESIZE, &args![to, 0u32, 0u32]);
                continue;
            }
            e.call(MATRIX_RESIZE, &args![to, rows, columns]);
            if row == 1 && copy_row_1 != 0 {
                fr2_matrix_vtc_operator_assign(e, Ptr::new(to), Ptr::new(from));
                limit_matrix(e, to, rows, limit);
            } else {
                let count = rows.wrapping_mul(columns);
                e.call(0x0044_a0a0, &args![to]);
                for index in 0..count {
                    let a = fn_006578b0(e, Ptr::new(from), index);
                    let a = x87_quiet_bits(e.mem.u32(a));
                    let b = fn_006578b0(e, Ptr::new(other), index);
                    let b = x87_quiet_bits(e.mem.u32(b));
                    let sum = x87_store32(x87_add(x87_load32(a), x87_m32(b)));
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), sum);
                        fn_006578e0(e, Ptr::new(to), slot);
                    });
                }
                limit_matrix(e, to, rows, limit);
            }
        }
    }
}

/// `00607310` (cdecl, one word): the object whose virtual slot 0x94 these getters read.
const OBJECT_OF_00607310: u32 = 0x0060_7310;
/// The two lists `00653230` and `00653300` search (`011d5bb8`, `011d5ba8`).
const LIST_011D5BB8: u32 = 0x011d_5bb8;
const LIST_011D5BA8: u32 = 0x011d_5ba8;
/// The extra data name `00653430` asks for (`011d5b8c`) and `NiObjectNET::GetExtraData`
/// (`00a5bdd0`, `thiscall`, one word).
const EXTRA_DATA_NAME: u32 = 0x011d_5b8c;
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// The geometry's data (`005495f0`, `thiscall`, no stack words) and its calls
/// (`NiGeometryData::MarkAsChanged` `00a67090` and `SetConsistency` `00a67050`, one word each;
/// `00a67360`, `00a67410`, one word; `00a673d0`).
const GEOMETRY_DATA: u32 = 0x0054_95f0;
const MARK_AS_CHANGED: u32 = 0x00a6_7090;
const SET_CONSISTENCY: u32 = 0x00a6_7050;
/// `strcpy_s` (`00406d30`, cdecl: buffer, size, text), `strrchr` (`0040ab30`), `sprintf_s`
/// (`00406d00`, cdecl: buffer, size, format, words), the string set (`004037f0`, `thiscall`:
/// text, 0), the string format (`00406f60`, cdecl: string, format, words) and the "NoHat" /
/// "Hat" suffixes (`0119b734`).
const STRCPY_S: u32 = 0x0040_6d30;
const STRRCHR: u32 = 0x0040_ab30;
const SPRINTF_S: u32 = 0x0040_6d00;
const STRING_SET: u32 = 0x0040_37f0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const HAT_SUFFIXES: u32 = 0x0119_b734;
/// The model loader (`011c3b3c`) and its queues (`ModelLoader::QueueTexture` `004436c0`,
/// three words; `00443d30`, seven words; `ModelLoader::QueueEGMFile` `00447bf0` and
/// `ModelLoader::QueueTRIFile` `00448080`, four words).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const QUEUE_TEXTURE: u32 = 0x0044_36c0;
const QUEUE_MODEL: u32 = 0x0044_3d30;
const QUEUE_EGM_FILE: u32 = 0x0044_7bf0;
const QUEUE_TRI_FILE: u32 = 0x0044_8080;
/// `"Textures\"` (`0101d7fc`), `strcat_s` (`00406d50`, cdecl: buffer, size, text) and a
/// texture's file name (`00408da0`, `thiscall`).
const TEXTURES_FOLDER: u32 = 0x0101_d7fc;
const STRCAT_S: u32 = 0x0040_6d50;
const TEXTURE_NAME: u32 = 0x0040_8da0;
/// `_invalid_parameter_noinfo` (`00ec7c56`), called on an index past the end.
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// The vector size and capacity (`00662820`, `0060b860`, `thiscall`) of the `float` vectors.
const FLOAT_VECTOR_SIZE: u32 = 0x0066_2820;
/// `0064df10` (`thiscall` on the allocator: block, count): the vectors' deallocation; the
/// `std::_Xlength_error` thrower `0065f640` (no arguments).
const VECTOR_DEALLOCATE: u32 = 0x0064_df10;
const LENGTH_ERROR: u32 = 0x0065_f640;

// Translated from 006530e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the virtual slot 0x94 of `00607310`'s object for `form` (cdecl,
/// one word), 0 when there is no object.
pub fn fn_006530e0(e: &mut Engine, form: u32) -> u32 {
    let object = e.call(OBJECT_OF_00607310, &args![form]).u32();
    if object == 0 {
        return 0;
    }
    e.vcall(object, 0x94, &args![]).u32()
}

// Translated from 00653120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00653180` on the object's slot 0x94 result for `form` (cdecl,
/// one word); 0 without an object or a result.
pub fn fn_00653120(e: &mut Engine, form: u32) -> u32 {
    let object = e.call(OBJECT_OF_00607310, &args![form]).u32();
    if object == 0 || e.vcall(object, 0x94, &args![]).u32() == 0 {
        return 0;
    }
    let inner = e.vcall(object, 0x94, &args![]).u32();
    fn_00653180(e, Ptr::new(inner))
}

// Translated from 00653180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00653180 - the FS:[0] exception frame (C++ unwinding of the stack NiPointer) is not translated
/// Unnamed in the engine map: the object held by the `NiPointer` at +0x1c of the object at
/// +8 (`thiscall`, no stack words); without one, of a null stack `NiPointer` (constructed and
/// destroyed around the read).
pub fn fn_00653180(e: &mut Engine, this: Ptr) -> u32 {
    let held = e.mem.u32(this.addr().wrapping_add(8));
    if held != 0 {
        return pointer_get(e, held.wrapping_add(0x1c));
    }
    e.with_stack(4, |e, slot| {
        let pointer = e.call(POINTER_CONSTRUCT, &args![slot, 0u32]).u32();
        let object = pointer_get(e, pointer);
        e.call(POINTER_DESTRUCT, &args![slot]);
        object
    })
}

// Translated from 00653230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the list at `011d5bb8` holds `00653120`'s object for
/// `form` (`00653270`; cdecl, one word); 0 without one.
pub fn fn_00653230(e: &mut Engine, form: u32) -> u32 {
    let object = fn_00653120(e, form);
    if object == 0 {
        return 0;
    }
    fn_00653270(e, LIST_011D5BB8, object)
}

// Translated from 00653270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00653290` on `object` with `list` (cdecl, two words), 0 for a
/// null object.
pub fn fn_00653270(e: &mut Engine, list: u32, object: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    fn_00653290(e, Ptr::new(object), list).addr()
}

// Translated from 006532c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether `wanted` is in the chain that starts at the object's
/// virtual slot 8 and goes on through `00726070` (`thiscall`, one word).
pub fn fn_006532c0(e: &mut Engine, this: Ptr, wanted: u32) -> bool {
    let mut link = e.vcall(this.addr(), 8, &args![]).u32();
    while link != 0 {
        if link == wanted {
            return true;
        }
        link = e.call(0x0072_6070, &args![link]).u32();
    }
    false
}

// Translated from 00653300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00653230` with the list at `011d5ba8` (cdecl, one word).
pub fn fn_00653300(e: &mut Engine, form: u32) -> u32 {
    let object = fn_00653120(e, form);
    if object == 0 {
        return 0;
    }
    fn_00653270(e, LIST_011D5BA8, object)
}

// Translated from 00653340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: applies the vertex offsets of `geometry`'s extra data
/// (`00653430`) through the three-word cursor `cursor` (cdecl, two words). Nothing without
/// the extra data or without the cursor's object (`00559450`). A copy of the cursor walks the
/// extra data's offsets (`0084e3a0`, 12 bytes each) for as many vertices as the geometry has
/// (`00456650`, 16 bits): each is stored (`006533f0`) and the copy advanced (`00657be0`). A
/// geometry with data (`0049ec60`) then marks it changed (`MarkAsChanged` with 1).
pub fn fn_00653340(e: &mut Engine, geometry: u32, cursor: u32) {
    let extra = fn_00653430(e, geometry);
    if extra == 0 {
        return;
    }
    if pointer_get(e, cursor) == 0 {
        return;
    }
    e.with_stack(12, |e, copy| {
        let copy = copy.addr();
        for word in 0..3u32 {
            let value = e.mem.u32(cursor.wrapping_add(word * 4));
            e.mem.set_u32(copy + word * 4, value);
        }
        let mut index = 0u32;
        let mut offset = e.call(0x0084_e3a0, &args![extra]).u32();
        loop {
            let count = u32::from(e.call(0x0045_6650, &args![geometry]).u16());
            if index >= count {
                break;
            }
            fn_006533f0(e, Ptr::new(copy), offset);
            fn_00657be0(e, Ptr::new(copy));
            index = index.wrapping_add(1);
            offset = offset.wrapping_add(0xc);
        }
    });
    if e.call(0x0049_ec60, &args![geometry]).u32() != 0 {
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        e.call(MARK_AS_CHANGED, &args![data, 1u32]);
    }
}

// Translated from 006533f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies three words from `source` to where the cursor's first
/// word points (`thiscall`, one word).
pub fn fn_006533f0(e: &mut Engine, this: Ptr, source: u32) {
    let target = e.mem.u32(this.addr());
    for word in 0..3u32 {
        let value = e.mem.u32(source.wrapping_add(word * 4));
        e.mem.set_u32(target.wrapping_add(word * 4), value);
    }
}

// Translated from 00653430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `object`'s extra data named by `011d5b8c` (`00653460`,
/// `NiObjectNET::GetExtraData`), 0 for a null object. cdecl with one word (`ECX`, which the
/// callers leave set, is not read).
pub fn fn_00653430(e: &mut Engine, object: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    let name = fn_00653460(e);
    e.call(GET_EXTRA_DATA, &args![object, name]).u32()
}

// Translated from 00653460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the extra data name at `011d5b8c`. No arguments.
pub fn fn_00653460(e: &mut Engine) -> u32 {
    e.mem.u32(EXTRA_DATA_NAME)
}

// Translated from 00653470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: applies `00653300`'s object to `geometry` with `weight` (cdecl,
/// two words). Nothing for a null geometry, a weight below 0.0 (a NaN goes on, as the exe's
/// parity test lets it), no object, or when the geometry's data refuses (`00a67360` with 1).
/// Otherwise a cursor (`00534550`) is filled by the data (`00a67410`), the vertex offsets are
/// applied (`00653340`), a cursor with an object gets the object's `00662460` (geometry, 0,
/// weight) and the data's `00a673d0` runs.
pub fn fn_00653470(e: &mut Engine, geometry: u32, weight: f32) {
    if geometry == 0 {
        return;
    }
    let zero: f64 = e.global(0x0101_2060);
    if x87_load32(weight.to_bits()) < zero {
        return;
    }
    let object = fn_00653300(e, geometry);
    if object == 0 {
        return;
    }
    let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
    if e.call(0x00a6_7360, &args![data, 1u32]).u8() == 0 {
        return;
    }
    e.with_stack(0x10, |e, cursor| {
        let cursor = cursor.addr();
        e.call(0x0053_4550, &args![cursor]);
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        e.call(0x00a6_7410, &args![data, cursor]);
        fn_00653340(e, geometry, cursor);
        if pointer_get(e, cursor) != 0 {
            let weight = x87_quiet_bits(weight.to_bits());
            e.call(0x0066_2460, &args![object, geometry, 0u32, weight]);
        }
    });
    let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
    e.call(0x00a6_73d0, &args![data]);
}

/// The shared body of the `GetAs...File` functions: the file name `name` without its
/// extension (`strcpy_s`, `strrchr`) is formatted into a second buffer with `with_hat` (and
/// the "NoHat"/"Hat" suffix of `hat`, for 0 and 1) or `plain`, set into the string `result`
/// (`004037f0`), whose text is answered; 0 for a null name or a name without a dot.
fn file_for(
    e: &mut Engine,
    result: u32,
    name: u32,
    hat: Option<i32>,
    with_hat: u32,
    plain: u32,
) -> u32 {
    if name == 0 {
        return 0;
    }
    e.with_stack(0x104, |e, stem| {
        let stem = stem.addr();
        e.call(STRCPY_S, &args![stem, 0x104u32, name]);
        let dot = e.call(STRRCHR, &args![stem, 0x2eu32]).u32();
        if dot == 0 {
            return 0;
        }
        e.mem.set_u8(dot, 0);
        e.with_stack(0x104, |e, file| {
            let file = file.addr();
            match hat {
                Some(index) if (0..=1).contains(&index) => {
                    let suffix = e
                        .mem
                        .u32(HAT_SUFFIXES.wrapping_add((index as u32).wrapping_mul(4)));
                    e.call(SPRINTF_S, &args![file, 0x104u32, with_hat, stem, suffix]);
                }
                _ => {
                    e.call(SPRINTF_S, &args![file, 0x104u32, plain, stem]);
                }
            }
            e.call(STRING_SET, &args![result, file, 0u32]);
            pointer_get(e, result)
        })
    })
}

// Translated from 00653520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00653520 - the /GS cookie check (00ec408c) of its stack buffers is not translated
/// `BSFaceGenManager::GetAsEGMFile` (Xbox PDB; cdecl: the result string, a file name and the
/// hat index): `"%s%s.egm"` with the "NoHat"/"Hat" suffix for 0 and 1, else `"%s.egm"`; the
/// result's text, or 0.
pub fn bs_face_gen_manager_get_as_egm_file(
    e: &mut Engine,
    result: u32,
    name: u32,
    hat: i32,
) -> u32 {
    file_for(e, result, name, Some(hat), 0x0106_5780, 0x0106_5778)
}

// Translated from 006536d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006536d0 - the /GS cookie check (00ec408c) of its stack buffers is not translated
/// `BSFaceGenManager::GetAsTRIFile` (Xbox PDB; cdecl: the result string, a file name and the
/// hat index): `"%s%s.tri"` or `"%s.tri"`, as `GetAsEGMFile`.
pub fn bs_face_gen_manager_get_as_tri_file(
    e: &mut Engine,
    result: u32,
    name: u32,
    hat: i32,
) -> u32 {
    file_for(e, result, name, Some(hat), 0x0106_579c, 0x0106_5794)
}

// Translated from 006537d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006537d0 - the /GS cookie check (00ec408c) of its stack buffers is not translated
/// Unnamed in the engine map: the `"%s.egt"` file for `name` (cdecl: the result string and a
/// file name), as `GetAsEGMFile` without a hat.
pub fn fn_006537d0(e: &mut Engine, result: u32, name: u32) -> u32 {
    file_for(e, result, name, None, 0, 0x0106_57a8)
}

// Translated from 00653880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00653880 - the /GS cookie check (00ec408c) of its stack buffers is not translated
/// `BSFaceGenManager::GetAsNearestDetailDDSFile` (Xbox PDB; cdecl: the result string, the
/// sex, the age byte and a file name). 0 for a null name, an age above 100 or a sex of 2 or
/// more, or a name without a dot. The result is formatted `"Textures\%s%c%d.ddx"` (`M`, or `F`
/// for sex 1, and the age); when that file does not exist (`00afd270` splits it, `00af6540`
/// looks it up) the `.dds` files from the age rounded down to tens are tried, ten by ten down
/// to 0; the first found is answered (the result's text), else the result is emptied and 0
/// answered.
pub fn bs_face_gen_manager_get_as_nearest_detail_dds_file(
    e: &mut Engine,
    result: u32,
    sex: i32,
    age: u8,
    name: u32,
) -> u32 {
    let mut age = i32::from(age);
    let mut letter = b'M';
    if name == 0 || age > 100 || sex >= 2 {
        return 0;
    }
    e.with_stack(0x104, |e, stem| {
        let stem = stem.addr();
        e.call(STRCPY_S, &args![stem, 0x104u32, name]);
        let dot = e.call(STRRCHR, &args![stem, 0x2eu32]).u32();
        if dot == 0 {
            return 0;
        }
        e.mem.set_u8(dot, 0);
        if sex == 1 {
            letter = b'F';
        }
        let letter = letter as i8 as i32;
        e.call(
            STRING_FORMAT,
            &args![result, 0x0106_57c4u32, stem, letter, age],
        );
        e.with_stack(0x14, |e, parts| {
            let directory = parts.addr();
            let file = directory + 0xc;
            e.call(0x0068_15c0, &args![directory]);
            e.call(0x0068_15c0, &args![file]);
            let text = pointer_get(e, result);
            e.call(0x00af_d270, &args![text, directory, file]);
            let text = pointer_get(e, result);
            if e.call(0x00af_6540, &args![1u32, directory, file, text])
                .u32()
                != 0
            {
                return pointer_get(e, result);
            }
            age = (age / 10) * 10;
            loop {
                e.call(
                    STRING_FORMAT,
                    &args![result, 0x0106_57b0u32, stem, letter, age],
                );
                let text = pointer_get(e, result);
                if e.call(0x00af_6540, &args![1u32, directory, file, text])
                    .u32()
                    != 0
                {
                    return pointer_get(e, result);
                }
                age = age.wrapping_sub(10);
                if age < 0 {
                    break;
                }
            }
            e.call(STRING_SET, &args![result, 0x0101_1584u32, 0u32]);
            0
        })
    })
}

// Translated from 00653cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: maps `kind` 1..7 to 0, 0, 1, 3, 2, 4, 13 (the compiler's jump
/// table `00653d48`), anything else to -1 (cdecl, one word).
pub fn fn_00653cf0(_e: &mut Engine, kind: u32) -> i32 {
    match kind.wrapping_sub(1) {
        0 | 1 => 0,
        2 => 1,
        3 => 3,
        4 => 2,
        5 => 4,
        6 => 0xd,
        _ => -1,
    }
}

// Translated from 00653d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: splits `b - a` (computed `a * -1.0 + b`, the `double` at
/// `0101a6b0`) into `below` and `above` (cdecl, four words): a difference below 0.0 (or a
/// NaN, which the exe's `C0` test sends the same way) goes negated into `below` with 0.0 in
/// `above`; otherwise 0.0 in `below` and the difference in `above`.
pub fn fn_00653d70(e: &mut Engine, a: f32, b: f32, below: Ptr, above: Ptr) {
    let minus_one: f64 = e.global(0x0101_a6b0);
    let zero: f64 = e.global(0x0101_2060);
    let difference = x87_store32(x87_add(
        x87_mul(x87_load32(a.to_bits()), minus_one),
        x87_m32(b.to_bits()),
    ));
    let value = x87_load32(difference);
    if value < zero || value.is_nan() {
        let negated = x87_store32(x87_mul(value, minus_one));
        e.mem.set_u32(below.addr(), negated);
        e.mem.set_u32(above.addr(), 0.0f32.to_bits());
    } else {
        e.mem.set_u32(below.addr(), 0.0f32.to_bits());
        e.mem.set_u32(above.addr(), difference);
    }
}

// Translated from 00653dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills `count` floats at `values` (cdecl, three words): with
/// `clear` set all bytes 0 (`00403d30`), else each with the `float` at `01016970`. Nothing
/// for a null buffer or no count.
pub fn fn_00653dc0(e: &mut Engine, values: u32, count: u32, clear: u8) {
    if values == 0 || count == 0 {
        return;
    }
    if clear != 0 {
        e.call(FILL_00403D30, &args![values, 0u32, count << 2]);
        return;
    }
    for index in 0..count {
        let fill = x87_quiet_bits(e.mem.u32(0x0101_6970));
        e.mem.set_u32(values.wrapping_add(index * 4), fill);
    }
}

// Translated from 00653e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::SetToZero` (Xbox PDB; cdecl: the floats and their count): clears them
/// (`00403d30`); nothing for a null buffer or no count.
pub fn bs_face_gen_manager_set_to_zero(e: &mut Engine, values: u32, count: u32) {
    if values == 0 || count == 0 {
        return;
    }
    e.call(FILL_00403D30, &args![values, 0u32, count << 2]);
}

// Translated from 00657010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::PrecacheFaceGeometry` (Xbox PDB; cdecl, three words, the second never
/// read): the object's geometry (virtual slot 0x1c) with data gets `004410f0` with `a3` and
/// `SetConsistency(0)` on its data; a geometry without `0043fad0` that has `004030b0`'s object
/// gives it to the renderer (`0043c4b0`; slot 0xec with the geometry, 0, 0 and the object's
/// slot 0x118 for the geometry's property 3), then the renderer's `00e74120`.
pub fn bs_face_gen_manager_precache_face_geometry(
    e: &mut Engine,
    object: u32,
    _unused_2: u32,
    a3: u32,
) {
    if object == 0 {
        return;
    }
    let geometry = e.vcall(object, 0x1c, &args![]).u32();
    if geometry == 0 || e.call(GEOMETRY_DATA, &args![geometry]).u32() == 0 {
        return;
    }
    let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
    e.call(0x0044_10f0, &args![data, a3]);
    let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
    e.call(SET_CONSISTENCY, &args![data, 0u32]);
    if e.call(0x0043_fad0, &args![geometry]).u32() != 0 {
        return;
    }
    let shader = e.call(0x0040_30b0, &args![geometry]).u32();
    if shader == 0 {
        return;
    }
    let renderer = e.call(0x0043_c4b0, &args![]).u32();
    let property = e.call(0x00a5_9d30, &args![geometry, 3u32]).u32();
    let value = e.vcall(shader, 0x118, &args![geometry, property]).u32();
    e.vcall(renderer, 0xec, &args![geometry, 0u32, 0u32, value]);
    let renderer = e.call(0x0043_c4b0, &args![]).u32();
    e.call(0x00e7_4120, &args![renderer]);
}

// Translated from 006570e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006570e0 - the /GS cookie check (00ec408c) of its stack buffers is not translated
/// `BSFaceGenManager::ClearBodyTexturesFromPalette` (Xbox PDB; cdecl, one word: the NPC): the
/// race's (`004ac110`) body texture name (`TESRace::GetBodyModTextureName`) and file name
/// (`TESRace::GetBodyModTextureFileName`) into two stack buffers, then the file is removed
/// from the texture palette (`BSTexturePalette::RemoveTexture`).
pub fn bs_face_gen_manager_clear_body_textures_from_palette(e: &mut Engine, npc: u32) {
    e.with_stack(0x208, |e, buffers| {
        let name = buffers.addr();
        let file = name + 0x104;
        let race = e.call(0x004a_c110, &args![npc]).u32();
        e.call(0x0061_47e0, &args![race, npc, name]);
        let race = e.call(0x004a_c110, &args![npc]).u32();
        e.call(0x0061_48f0, &args![race, npc, name, file]);
        e.call(0x00a6_1fb0, &args![file]);
    });
}

/// The NPC's face-gen data (`005dd590` constructs it, `005dd6f0` destroys it): 0xf0 bytes.
const FACE_GEN_DATA_SIZE: u32 = 0xf0;

// Translated from 00657150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00657150 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: queues an NPC's face textures and models with the model loader
/// (`011c3b3c`) (cdecl: the NPC and two words passed on to every queue), inside the
/// memory-context scope (0x37, line 0xe38). The race's face-gen data for the NPC's sex
/// (`TESRace::GetFaceGenData` into a stack copy) gives the age byte (`00652440`, `00406ce0`,
/// `_ftol2`), the eyes texture (or `EyeDefault.dds`), the hair model, texture and EGM file,
/// the head part texture (`TESNPC::GetHeadPartModTextureFileName`); each head part with a
/// texture (but 6 and 7) its texture, its nearest age detail texture for part 0
/// (`GetAsNearestDetailDDSFile`), its `0048e660` texture and, for parts 0 and 1 with the
/// data's byte +0xd4, its `0048e6c0` texture; each head part model and each listed extra part
/// its model, EGM and TRI files.
pub fn fn_00657150(e: &mut Engine, npc: u32, a2: u32, a3: u32) {
    in_scope(e, 0x37, 0xe38, |e| {
        e.with_stack(FACE_GEN_DATA_SIZE, |e, data| {
            let data = data.addr();
            e.call(0x005d_d590, &args![data]);
            let sex = e.call(0x005f_0cc0, &args![npc]).u32();
            let race = e.call(0x004a_c110, &args![npc]).u32();
            e.call(0x0061_41f0, &args![race, npc, data, sex, 1u32]);
            // The exe tests the stack copy's address, which is never null.
            if data == 0 {
                e.call(0x005d_d6f0, &args![data]);
                return;
            }
            queue_face(e, npc, data, a2, a3);
            e.call(0x005d_d6f0, &args![data]);
        });
    });
}

/// `ModelLoader::QueueTexture` on the model loader.
fn queue_texture(e: &mut Engine, file: u32, a2: u32, a3: u32) {
    let loader = e.mem.u32(MODEL_LOADER);
    e.call(QUEUE_TEXTURE, &args![loader, file, a2, a3]);
}

/// `"Textures\"` followed by the texture's file name (`00408da0` on `texture`) in `buffer`.
fn textures_path(e: &mut Engine, buffer: u32, texture: u32) {
    e.call(STRCPY_S, &args![buffer, 0x104u32, TEXTURES_FOLDER]);
    let name = e.call(TEXTURE_NAME, &args![texture]).u32();
    e.call(STRCAT_S, &args![buffer, 0x104u32, name]);
}

/// The model, EGM and TRI queues for a model at `model` with the part index `part`.
fn queue_model_files(e: &mut Engine, model: u32, a2: u32, a3: u32, part: u32) {
    let loader = e.mem.u32(MODEL_LOADER);
    e.call(
        QUEUE_MODEL,
        &args![loader, model, a2, a3, 0u32, 0u32, 0u32, 1u32],
    );
    let loader = e.mem.u32(MODEL_LOADER);
    e.call(QUEUE_EGM_FILE, &args![loader, model, a2, a3, part]);
    let loader = e.mem.u32(MODEL_LOADER);
    e.call(QUEUE_TRI_FILE, &args![loader, model, a2, a3, part]);
}

/// The model at +0x24 of `form` (0 for a null form), as the exe computes it each time.
fn model_of(form: u32) -> u32 {
    if form != 0 {
        form.wrapping_add(0x24)
    } else {
        0
    }
}

/// The body of `00657150` once the face-gen data is read.
fn queue_face(e: &mut Engine, npc: u32, data: u32, a2: u32, a3: u32) {
    let value = fn_00652440(e, data, 0, 0);
    let value = x87_store32(value);
    let rounded = e.call(0x0040_6ce0, &args![value]).f64();
    let age = e.call(0x00ec_62c0, &args![rounded]).u8();
    e.with_stack(0x104 * 6 + 8, |e, buffers| {
        let eyes_buffer = buffers.addr();
        let hair_buffer = eyes_buffer + 0x104;
        let part_buffer = hair_buffer + 0x104;
        let first_extra = part_buffer + 0x104;
        let second_extra = first_extra + 0x104;
        let head_buffer = second_extra + 0x104;
        let string = head_buffer + 0x104;

        let eyes = e.mem.u32(data + 0x8c);
        if eyes != 0 {
            textures_path(e, eyes_buffer, eyes.wrapping_add(0x24));
            queue_texture(e, eyes_buffer, a2, a3);
        } else {
            queue_texture(e, 0x0106_5804, a2, a3);
        }

        if e.mem.u32(data + 0x80) != 0 {
            let model = model_of(e.mem.u32(data + 0x80));
            let loader = e.mem.u32(MODEL_LOADER);
            e.call(
                QUEUE_MODEL,
                &args![loader, model, a2, a3, 0u32, 0u32, 0u32, 1u32],
            );
            let hair = e.mem.u32(data + 0x80);
            textures_path(e, hair_buffer, hair.wrapping_add(0x3c));
            queue_texture(e, hair_buffer, a2, a3);
            let model = model_of(e.mem.u32(data + 0x80));
            let loader = e.mem.u32(MODEL_LOADER);
            e.call(QUEUE_EGM_FILE, &args![loader, model, a2, a3, 9u32]);
        }

        if e.call(0x0060_4310, &args![npc, 0u32, head_buffer, 0x104u32])
            .bool()
        {
            queue_texture(e, head_buffer, a2, a3);
        }

        let models = data + 0x94;
        let textures = data + 0xa4;
        let slot = |e: &mut Engine, array: u32, index: u32| {
            let at = e.call(0x0087_7a30, &args![array, index]).u32();
            e.mem.u32(at)
        };
        let mut index = 0u32;
        while index < 8 {
            if index == 6
                || index == 7
                || slot(e, models, index) == 0
                || slot(e, textures, index) == 0
            {
                index += 1;
                continue;
            }
            let kind = if index == 0 || index == 1 { 0xe } else { 1 };
            e.call(0x0040_37b0, &args![string]);
            let texture = slot(e, textures, index);
            textures_path(e, part_buffer, texture);
            queue_texture(e, part_buffer, a2, a3);
            if index == 0 && (age as i8) >= 0 {
                let texture = slot(e, textures, index);
                let name = e.call(TEXTURE_NAME, &args![texture]).u32();
                let sex = e.mem.u32(data + 0x90) as i32;
                if bs_face_gen_manager_get_as_nearest_detail_dds_file(e, string, sex, age, name)
                    != 0
                {
                    let text = pointer_get(e, string);
                    e.call(STRCPY_S, &args![part_buffer, 0x104u32, text]);
                    queue_texture(e, part_buffer, a2, a3);
                }
            }
            e.call(0x0048_e660, &args![part_buffer, first_extra]);
            queue_texture(e, first_extra, a2, a3);
            if e.mem.u8(data + 0xd4) != 0 && kind == 0xe {
                e.call(0x0048_e6c0, &args![part_buffer, second_extra]);
                queue_texture(e, second_extra, a2, a3);
            }
            e.call(0x0040_37d0, &args![string]);
            index += 1;
        }

        let mut index = 0u32;
        while index < 8 {
            let part = slot(e, models, index);
            if part != 0 && e.call(0x0048_cee0, &args![part]).u32() != 0 {
                queue_model_files(e, part, a2, a3, index);
            }
            index += 1;
        }

        // The extra parts' list from +0xe4, `index` staying at 8.
        let mut link = data + 0xe4;
        while link != 0 {
            let at = e.call(0x0068_15c0, &args![link]).u32();
            let part = e.mem.u32(at);
            if part != 0 && e.call(0x0048_cee0, &args![part.wrapping_add(0x24)]).u32() != 0 {
                let loader = e.mem.u32(MODEL_LOADER);
                e.call(
                    QUEUE_MODEL,
                    &args![loader, model_of(part), a2, a3, 0u32, 0u32, 0u32, 1u32],
                );
                let loader = e.mem.u32(MODEL_LOADER);
                e.call(
                    QUEUE_EGM_FILE,
                    &args![loader, model_of(part), a2, a3, index],
                );
                let loader = e.mem.u32(MODEL_LOADER);
                e.call(
                    QUEUE_TRI_FILE,
                    &args![loader, model_of(part), a2, a3, index],
                );
            }
            link = e.call(0x0072_6070, &args![link]).u32();
        }
    });
}

// Translated from 00657820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `"FaceGenHair"` followed by the "NoHat"/"Hat" suffix of `hat`
/// (the first one outside 0 and 1) in `buffer` (`004046f0`, `strcat` `00ec6380`; cdecl, two
/// words). Answers `buffer`.
pub fn fn_00657820(e: &mut Engine, buffer: u32, hat: i32) -> u32 {
    e.call(0x0040_46f0, &args![buffer, 0x0104_aad0u32]);
    let suffix = if hat > -1 && hat < 2 {
        e.mem
            .u32(HAT_SUFFIXES.wrapping_add((hat as u32).wrapping_mul(4)))
    } else {
        e.mem.u32(HAT_SUFFIXES)
    };
    e.call(0x00ec_6380, &args![buffer, suffix]);
    buffer
}

// Translated from 006578b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the address of `float` `index` of the vector (data at +0xc)
/// (`thiscall`, one word); an index past the size (`00662820`) first reports
/// `_invalid_parameter_noinfo`.
pub fn fn_006578b0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let size = e.call(FLOAT_VECTOR_SIZE, &args![this]).u32();
    if index >= size {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem
        .u32(this.addr().wrapping_add(0xc))
        .wrapping_add(index.wrapping_mul(4))
}

// Translated from 006578e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: appends the `float` at `value` to the vector (`thiscall`, one
/// word): within the capacity (`0060b860`) by `0060b8a0` at the end (+0x10), else through an
/// insert at the end (`0065f550` gives it, `00657c50`).
pub fn fn_006578e0(e: &mut Engine, this: Ptr, value: Ptr) {
    let base = this.addr();
    let size = e.call(FLOAT_VECTOR_SIZE, &args![this]).u32();
    let capacity = e.call(0x0060_b860, &args![this]).u32();
    if size < capacity {
        let end = e.mem.u32(base.wrapping_add(0x10));
        let end = e.call(0x0060_b8a0, &args![this, end, 1u32, value]).u32();
        e.mem.set_u32(base.wrapping_add(0x10), end);
        return;
    }
    e.with_stack(0x10, |e, slots| {
        let end_slot = slots.addr();
        let result = end_slot + 8;
        let end = e.call(0x0065_f550, &args![this, end_slot]).u32();
        let first = e.mem.u32(end);
        let second = e.mem.u32(end.wrapping_add(4));
        fn_00657c50(e, this, Ptr::new(result), first, second, value.addr());
    });
}

/// The constructor shape of `00657960`, `00657a80` and `00657b50`: an allocator word made by
/// `006815c0` on a stack word, handed to `0065d3a0`, then `reserve(0)` (`reserve`).
fn construct_vector(e: &mut Engine, this: Ptr, reserve: fn(&mut Engine, Ptr, u32) -> u8) -> Ptr {
    let allocator = e.with_stack(4, |e, slot| {
        e.call(0x0068_15c0, &args![slot]);
        e.mem.u32(slot.addr())
    });
    e.call(0x0065_d3a0, &args![this, allocator]);
    reserve(e, this, 0);
    this
}

// Translated from 00657960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00657960 - the FS:[0] exception frame (C++ unwinding during construction) is not translated
/// Unnamed in the engine map: a vector constructor (`thiscall`, no stack words): the
/// allocator (`006815c0` into the argument word, `0065d3a0`) and `00657cd0` with 0. Answers
/// `this`.
pub fn fn_00657960(e: &mut Engine, this: Ptr) -> Ptr {
    construct_vector(e, this, fn_00657cd0)
}

// Translated from 006579d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006579d0 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the destructor of `00657960`'s vector (`thiscall`, no stack
/// words): `00657d50`, then `0065cfb0`.
pub fn fn_006579d0(e: &mut Engine, this: Ptr) {
    fn_00657d50(e, this);
    e.call(0x0065_cfb0, &args![this]);
}

/// `(end - begin) / size` of a vector (+0xc begin, +0x10 end), signed as the exe's `IDIV`.
fn vector_count(e: &Engine, this: Ptr, size: i32) -> i32 {
    let end = e.mem.i32(this.addr().wrapping_add(0x10));
    let begin = e.mem.i32(this.addr().wrapping_add(0xc));
    end.wrapping_sub(begin).wrapping_div(size)
}

// Translated from 00657a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the number of 0x3c-byte elements of the vector (`thiscall`, no
/// stack words).
pub fn fn_00657a30(e: &mut Engine, this: Ptr) -> i32 {
    vector_count(e, this, 0x3c)
}

// Translated from 00657a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the address of 0x3c-byte element `index` (`thiscall`, one word);
/// an index past the count first reports `_invalid_parameter_noinfo`.
pub fn fn_00657a50(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index >= fn_00657a30(e, this) as u32 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    index
        .wrapping_mul(0x3c)
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(0xc)))
}

// Translated from 00657a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00657a80 - the FS:[0] exception frame (C++ unwinding during construction) is not translated
/// Unnamed in the engine map: the 0x88-byte elements' vector constructor (`thiscall`, no stack
/// words): as `00657960` with `00657dd0`. Answers `this`.
pub fn fn_00657a80(e: &mut Engine, this: Ptr) -> Ptr {
    construct_vector(e, this, fn_00657dd0)
}

// Translated from 00657af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00657af0 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the destructor of `00657a80`'s vector (`thiscall`, no stack
/// words): `00657e60`, then `0065cfb0`.
pub fn fn_00657af0(e: &mut Engine, this: Ptr) {
    fn_00657e60(e, this);
    e.call(0x0065_cfb0, &args![this]);
}

// Translated from 00657b50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00657b50 - the FS:[0] exception frame (C++ unwinding during construction) is not translated
/// Unnamed in the engine map: the byte vector's constructor (`thiscall`, no stack words): as
/// `00657960` with `00657ee0`. Answers `this`.
pub fn fn_00657b50(e: &mut Engine, this: Ptr) -> Ptr {
    construct_vector(e, this, fn_00657ee0)
}

// Translated from 00657bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the number of 6-byte elements of the vector (`thiscall`, no
/// stack words).
pub fn fn_00657bc0(e: &mut Engine, this: Ptr) -> i32 {
    vector_count(e, this, 6)
}

// Translated from 00657be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: advances a cursor (+0) by its stride (+4) (`thiscall`, no stack
/// words).
pub fn fn_00657be0(e: &mut Engine, this: Ptr) {
    let at = e.mem.u32(this.addr());
    let stride = e.mem.u32(this.addr().wrapping_add(4));
    e.mem.set_u32(this.addr(), at.wrapping_add(stride));
}

// Translated from 00657c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the number of 0x1c-byte elements of the vector (`thiscall`, no
/// stack words).
pub fn fn_00657c00(e: &mut Engine, this: Ptr) -> i32 {
    vector_count(e, this, 0x1c)
}

// Translated from 00657c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the address of 0x1c-byte element `index` (`thiscall`, one word);
/// an index past the count first reports `_invalid_parameter_noinfo`.
pub fn fn_00657c20(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index >= fn_00657c00(e, this) as u32 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    index
        .wrapping_mul(0x1c)
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(0xc)))
}

// Translated from 00657c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` vector's insert of one value before `where`
/// (`thiscall`, four words: the result iterator, the two words of `where`, the value's
/// address). The offset of `where` from the beginning (`0044a170`, `006580b0`; 0 for an empty
/// vector) is kept, `0060b4d0` inserts, and the result is the beginning moved by that offset
/// (`0060b820`). Answers `result`.
pub fn fn_00657c50(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    where_first: u32,
    where_second: u32,
    value: u32,
) -> Ptr {
    e.with_stack(0x18, |e, slots| {
        let place = slots.addr();
        let begin_slot = place + 8;
        let second_begin = place + 0x10;
        e.mem.set_u32(place, where_first);
        e.mem.set_u32(place + 4, where_second);
        let offset = if e.call(FLOAT_VECTOR_SIZE, &args![this]).u32() == 0 {
            0
        } else {
            let begin = e.call(0x0044_a170, &args![this, begin_slot]).u32();
            fn_006580b0(e, Ptr::new(place), Ptr::new(begin))
        };
        let where_first = e.mem.u32(place);
        let where_second = e.mem.u32(place + 4);
        e.call(
            0x0060_b4d0,
            &args![this, where_first, where_second, 1u32, value],
        );
        let begin = e.call(0x0044_a170, &args![this, second_begin]).u32();
        e.call(0x0060_b820, &args![begin, result, offset]);
    });
    result
}

/// Clears a vector's begin, end and capacity end (+0xc, +0x10, +0x14).
fn clear_vector(e: &mut Engine, this: Ptr) {
    for offset in [0xcu32, 0x10, 0x14] {
        e.mem.set_u32(this.addr().wrapping_add(offset), 0);
    }
}

/// The rest of the `reserve` shape of `00657cd0`, `00657dd0` and `00657ee0` for a non-zero
/// count: the length error (`0065f640`) when it is above `max_size`, else `allocate` on the
/// allocator (+8) and the three words set. Answers 1.
fn reserve(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    max_size: u32,
    allocate: u32,
    element: u32,
) -> u8 {
    let base = this.addr();
    if max_size < count {
        e.call(LENGTH_ERROR, &args![]);
    } else {
        let block = e.call(allocate, &args![base.wrapping_add(8), count]).u32();
        e.mem.set_u32(base.wrapping_add(0xc), block);
        let begin = e.mem.u32(base.wrapping_add(0xc));
        e.mem.set_u32(base.wrapping_add(0x10), begin);
        let begin = e.mem.u32(base.wrapping_add(0xc));
        e.mem.set_u32(
            base.wrapping_add(0x14),
            count.wrapping_mul(element).wrapping_add(begin),
        );
    }
    1
}

// Translated from 00657cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x3c-byte elements' vector reserve (`thiscall`, one word:
/// the count): the maximum from `00657fd0`, the allocation `00658020`. Answers 0 for no count,
/// else 1.
pub fn fn_00657cd0(e: &mut Engine, this: Ptr, count: u32) -> u8 {
    clear_vector(e, this);
    if count == 0 {
        return 0;
    }
    let max_size = fn_00657fd0(e, this);
    reserve(e, this, count, max_size, 0x0065_8020, 0x3c)
}

/// The release shape of `00657d50`, `00657e60` and `00657f60`: with a block (+0xc), `destroy`
/// on the range (+0xc to +0x10) and the allocator's (+8) deallocation (`0064df10`) of the
/// capacity (+0x14 - +0xc, over `element` when it is above 1); the three words cleared.
fn release_vector(
    e: &mut Engine,
    this: Ptr,
    destroy: impl FnOnce(&mut Engine, u32, u32),
    element: i32,
) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0xc)) != 0 {
        let last = e.mem.u32(base.wrapping_add(0x10));
        let first = e.mem.u32(base.wrapping_add(0xc));
        destroy(e, first, last);
        let end = e.mem.i32(base.wrapping_add(0x14));
        let begin = e.mem.i32(base.wrapping_add(0xc));
        let bytes = end.wrapping_sub(begin);
        let count = if element > 1 {
            bytes.wrapping_div(element)
        } else {
            bytes
        };
        let block = e.mem.u32(base.wrapping_add(0xc));
        e.call(
            VECTOR_DEALLOCATE,
            &args![base.wrapping_add(8), block, count],
        );
    }
    clear_vector(e, this);
}

// Translated from 00657d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: releases the 0x3c-byte elements' vector (`thiscall`, no stack
/// words): `00657ff0` on the range and the deallocation of the capacity in elements.
pub fn fn_00657d50(e: &mut Engine, this: Ptr) {
    release_vector(
        e,
        this,
        |e, first, last| fn_00657ff0(e, this, first, last),
        0x3c,
    );
}

// Translated from 00657dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x88-byte elements' vector reserve (`thiscall`, one word):
/// the maximum from `00658040`, the allocation `00658090`. Answers 0 for no count, else 1.
pub fn fn_00657dd0(e: &mut Engine, this: Ptr, count: u32) -> u8 {
    clear_vector(e, this);
    if count == 0 {
        return 0;
    }
    let max_size = fn_00658040(e, this);
    reserve(e, this, count, max_size, 0x0065_8090, 0x88)
}

// Translated from 00657e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: releases the 0x88-byte elements' vector (`thiscall`, no stack
/// words): `00658060` on the range and the deallocation of the capacity in elements.
pub fn fn_00657e60(e: &mut Engine, this: Ptr) {
    release_vector(
        e,
        this,
        |e, first, last| {
            e.call(0x0065_8060, &args![this, first, last]);
        },
        0x88,
    );
}

// Translated from 00657ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte vector's reserve (`thiscall`, one word): the maximum
/// from `0065f6e0`, the allocation `0065f7a0`. Answers 0 for no count, else 1.
pub fn fn_00657ee0(e: &mut Engine, this: Ptr, count: u32) -> u8 {
    clear_vector(e, this);
    if count == 0 {
        return 0;
    }
    let max_size = e.call(0x0065_f6e0, &args![this]).u32();
    reserve(e, this, count, max_size, 0x0065_f7a0, 1)
}

// Translated from 00657f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: releases the byte vector (`thiscall`, no stack words):
/// `0065f2b0` on the range and the deallocation of the capacity in bytes.
pub fn fn_00657f60(e: &mut Engine, this: Ptr) {
    release_vector(
        e,
        this,
        |e, first, last| {
            e.call(0x0065_f2b0, &args![this, first, last]);
        },
        1,
    );
}

// Translated from 00657ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destroys the range `first`..`last` with the vector's allocator
/// (+8) (`00658bd0`; `thiscall`, two words).
pub fn fn_00657ff0(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    fn_00658bd0(e, first, last, this.addr().wrapping_add(8));
}

// Translated from 006580b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the distance in words from the iterator `other` to this one
/// (`thiscall`, one word): `(this+4 - other+4) >> 2` (arithmetic), after
/// `_invalid_parameter_noinfo` when this one is not valid (`005ca4f0`) or not compatible
/// (`004b0460`).
pub fn fn_006580b0(e: &mut Engine, this: Ptr, other: Ptr) -> i32 {
    if !e.call(0x005c_a4f0, &args![this]).bool() || !e.call(0x004b_0460, &args![this, other]).bool()
    {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let mine = e.mem.i32(this.addr().wrapping_add(4));
    let theirs = e.mem.i32(other.addr().wrapping_add(4));
    mine.wrapping_sub(theirs) >> 2
}

// Translated from 006581e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FR2MatrixVTC<float>::operator=` (Xbox PDB name; `thiscall`, one word: the source): unless
/// it is itself, takes the source's dimensions (+0x18, +0x1c), sizes its storage
/// (`0060b3f0` with their product) and copies the values (`00401460`, product times 4 bytes,
/// from element 0 of each, `006578b0`). Answers `this`.
pub fn fr2_matrix_vtc_operator_assign(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    if this == source {
        return this;
    }
    let base = this.addr();
    let rows = e.mem.u32(source.addr().wrapping_add(0x18));
    e.mem.set_u32(base.wrapping_add(0x18), rows);
    let columns = e.mem.u32(source.addr().wrapping_add(0x1c));
    e.mem.set_u32(base.wrapping_add(0x1c), columns);
    let count = e
        .mem
        .u32(base.wrapping_add(0x18))
        .wrapping_mul(e.mem.u32(base.wrapping_add(0x1c)));
    e.call(0x0060_b3f0, &args![this, count]);
    let bytes = e
        .mem
        .u32(base.wrapping_add(0x18))
        .wrapping_mul(e.mem.u32(base.wrapping_add(0x1c)))
        << 2;
    let from = fn_006578b0(e, source, 0);
    let to = fn_006578b0(e, this, 0);
    e.call(0x0040_1460, &args![to, from, bytes]);
    this
}

// Translated from 00658260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00658260 - the FS:[0] exception frame (C++ unwinding of the stack matrix) is not translated
/// The engine map's `FR2MatrixVTC<float>::operatorP` (`thiscall`, two words: the result
/// matrix and a `float`): a stack matrix of the same size (`00659000`) gets each value times
/// `factor` (iterators `0065fd10`/`0065fd40`/`0065dfd0` over this one, `0044a170` over the
/// copy, `0064dda0`/`0064dbd0` the elements, `00659100`/`006590c0` the steps), is copied into
/// `result` (`00658f30`) and destroyed (`00449680`). Answers `result`.
pub fn fn_00658260(e: &mut Engine, this: Ptr, result: Ptr, factor: f32) -> Ptr {
    let base = this.addr();
    e.with_stack(0x50, |e, slots| {
        let copy = slots.addr();
        let source_it = copy + 0x20;
        let copy_it = copy + 0x28;
        let end_slot = copy + 0x30;
        let step_a = copy + 0x38;
        let step_b = copy + 0x40;
        let rows = e.mem.u32(base.wrapping_add(0x18));
        let columns = e.mem.u32(base.wrapping_add(0x1c));
        e.call(0x0065_9000, &args![copy, rows, columns]);
        e.call(0x0065_fd10, &args![this, source_it]);
        e.call(0x0044_a170, &args![copy, copy_it]);
        loop {
            let end = e.call(0x0065_fd40, &args![this, end_slot]).u32();
            if !e.call(0x0065_dfd0, &args![source_it, end]).bool() {
                break;
            }
            let from = e.call(0x0064_dda0, &args![source_it]).u32();
            let product = x87_store32(x87_mul(
                x87_load32(e.mem.u32(from)),
                x87_m32(factor.to_bits()),
            ));
            let to = e.call(0x0064_dbd0, &args![copy_it]).u32();
            e.mem.set_u32(to, product);
            e.call(0x0065_9100, &args![source_it, step_a, 0u32]);
            e.call(0x0065_90c0, &args![copy_it, step_b, 0u32]);
        }
        e.call(0x0065_8f30, &args![result, copy]);
        e.call(ELEMENT_DESTRUCT_00449680, &args![copy]);
    });
    result
}

// Translated from 00658360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00658360 - the FS:[0] exception frame (C++ unwinding of the stack matrix) is not translated
/// Unnamed in the engine map: the matrix product of this one and `other` into `result`
/// (`thiscall`, two words). Mismatched inner sizes report `fr2Assert` (`00aafb90` with
/// `matrixVT.hpp` and line 0x113). A stack matrix of this one's rows by the other's columns
/// (`00659000`) gets each element (`0060b370`, `00629ab0`) set to 0.0 and then the sum of the
/// products, in the exe's `FLD`, `FMUL`, `FADD`, `FSTP` order; it is copied into `result`
/// (`00658f30`) and destroyed (`00449680`). Answers `result`.
pub fn fn_00658360(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let a = this.addr();
    let b = other.addr();
    if e.mem.u32(a.wrapping_add(0x1c)) != e.mem.u32(b.wrapping_add(0x18)) {
        e.call(0x00aa_fb90, &args![0x0106_5830u32, 0x113u32]);
    }
    e.with_stack(0x20, |e, product| {
        let product = product.addr();
        let rows = e.mem.u32(a.wrapping_add(0x18));
        let columns = e.mem.u32(b.wrapping_add(0x1c));
        e.call(0x0065_9000, &args![product, rows, columns]);
        let mut row = 0u32;
        while row < e.mem.u32(a.wrapping_add(0x18)) {
            let mut column = 0u32;
            while column < e.mem.u32(b.wrapping_add(0x1c)) {
                let cell = matrix_element(e, product, row, column);
                e.mem.set_u32(cell, 0.0f32.to_bits());
                let mut inner = 0u32;
                while inner < e.mem.u32(a.wrapping_add(0x1c)) {
                    let cell = matrix_element(e, product, row, column);
                    let left = matrix_element(e, a, row, inner);
                    let right = matrix_element(e, b, inner, column);
                    let sum = x87_add(
                        x87_mul(x87_load32(e.mem.u32(left)), x87_m32(e.mem.u32(right))),
                        x87_m32(e.mem.u32(cell)),
                    );
                    e.mem.set_u32(cell, x87_store32(sum));
                    inner += 1;
                }
                column += 1;
            }
            row += 1;
        }
        e.call(0x0065_8f30, &args![result, product]);
        e.call(ELEMENT_DESTRUCT_00449680, &args![product]);
    });
    result
}

// Translated from 00658950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00658950 - the FS:[0] exception frame (C++ unwinding of the by-value NiPointer) is not translated
/// Unnamed in the engine map: the string map's `SetAt` (`thiscall`, two words: the key and an
/// `NiPointer` by value, its word here). The key's bucket (virtual slot 4) is searched with
/// the map's key compare (slot 8); a match takes the key (unless the map copies keys, +0x10)
/// and the value (`NiPointer` assignment at the item's +8). Otherwise a new item (`006c63e0`)
/// gets the key and a copy of the value (slot 0xc) and goes first in the bucket; the count
/// (+0xc) grows. The by-value pointer is destroyed at the end.
pub fn fn_00658950(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let base = this.addr();
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        e.mem.set_u32(argument, value);
        let bucket = e.vcall(base, 4, &args![key]).u32();
        let buckets = e.mem.u32(base.wrapping_add(8));
        let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
        while item != 0 {
            let item_key = e.mem.u32(item.wrapping_add(4));
            if e.vcall(base, 8, &args![key, item_key]).bool() {
                if e.mem.u8(base.wrapping_add(0x10)) == 0 {
                    e.mem.set_u32(item.wrapping_add(4), key);
                }
                e.call(
                    POINTER_ASSIGN_POINTER,
                    &args![item.wrapping_add(8), argument],
                );
                e.call(POINTER_DESTRUCT, &args![argument]);
                return;
            }
            item = e.mem.u32(item);
        }
        let item = e.call(0x006c_63e0, &args![this]).u32();
        let copy = e.with_stack(4, |e, slot| {
            e.call(POINTER_COPY, &args![slot, argument]);
            e.mem.u32(slot.addr())
        });
        e.vcall(base, 0xc, &args![item, key, copy]);
        let buckets = e.mem.u32(base.wrapping_add(8));
        let slot = buckets.wrapping_add(bucket.wrapping_mul(4));
        let head = e.mem.u32(slot);
        e.mem.set_u32(item, head);
        let buckets = e.mem.u32(base.wrapping_add(8));
        e.mem
            .set_u32(buckets.wrapping_add(bucket.wrapping_mul(4)), item);
        let count = e.mem.u32(base.wrapping_add(0xc));
        e.mem.set_u32(base.wrapping_add(0xc), count.wrapping_add(1));
        e.call(POINTER_DESTRUCT, &args![argument]);
    });
}

// Translated from 00658ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00658ad0 - the FS:[0] exception frame (C++ unwinding of the by-value NiPointer) is not translated
/// `NiTStringTemplateMap<NiTMap<char const *, NiPointer<BSFaceGenModelMap::Entry>>, ...>::SetValue`
/// (engine map name; `thiscall`, three words: the item, the key and an `NiPointer` by value).
/// A map that copies keys (+0x10) stores a copy (`strlen`, `00aa1070`, `00404d80`), else the
/// key itself, at the item's +4; the value goes into the item's +8 (`NiPointer` assignment)
/// and the by-value pointer is destroyed.
pub fn ni_t_string_template_map_set_value(
    e: &mut Engine,
    this: Ptr,
    item: u32,
    key: u32,
    value: u32,
) {
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        e.mem.set_u32(argument, value);
        if e.mem.u8(this.addr().wrapping_add(0x10)) != 0 {
            let length = e.call(0x00ec_6130, &args![key]).u32().wrapping_add(1);
            let copy = e.call(0x00aa_1070, &args![length]).u32();
            e.mem.set_u32(item.wrapping_add(4), copy);
            let copy = e.mem.u32(item.wrapping_add(4));
            e.call(0x0040_4d80, &args![copy, length, key]);
        } else {
            e.mem.set_u32(item.wrapping_add(4), key);
        }
        e.call(
            POINTER_ASSIGN_POINTER,
            &args![item.wrapping_add(8), argument],
        );
        e.call(POINTER_DESTRUCT, &args![argument]);
    });
}

// Translated from 00658b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: frees an item's key (+4, `00aa10f0`) when the map copies keys
/// (+0x10) (`thiscall`, one word: the item).
pub fn fn_00658b80(e: &mut Engine, this: Ptr, item: u32) {
    if e.mem.u8(this.addr().wrapping_add(0x10)) != 0 {
        let key = e.mem.u32(item.wrapping_add(4));
        e.call(0x00aa_10f0, &args![key]);
    }
}

// Translated from 00658bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `_Destroy_range` of `first`..`last` with `allocator` (cdecl,
/// three words): the iterator category (`0065e750` on the two argument words) and
/// `00659260`.
pub fn fn_00658bd0(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    e.with_stack(8, |e, words| {
        let words = words.addr();
        e.mem.set_u32(words, first);
        e.mem.set_u32(words + 4, last);
        // ECX is the first argument word's address (00658bd8, LEA ECX,[EBP+8]): 0065e750's
        // answer is its top byte, before the two pushed words.
        let category = e.call(0x0065_e750, &args![words, words, words + 4]).u8();
        let first = e.mem.u32(words);
        let last = e.mem.u32(words + 4);
        fn_00659260(e, first, last, allocator, u32::from(category));
    });
}

// Translated from 00658c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: allocates `count` 0x3c-byte elements (`00401000`; cdecl, one
/// word); a count whose size overflows throws `std::bad_alloc` (`0044d6c0`,
/// `_CxxThrowException` `00ec7894` with `01140ef8`). A count of 0 allocates 0 bytes.
pub fn fn_00658c10(e: &mut Engine, count: u32) -> u32 {
    // (The exe stores 0 back into a count of 0.)
    if count != 0 && u32::MAX / count < 0x3c {
        e.with_stack(0xc, |e, exception| {
            e.call(0x0044_d6c0, &args![exception, 0u32]);
            e.call(0x00ec_7894, &args![exception, 0x0114_0ef8u32]);
        });
    }
    e.call(ALLOCATE_PLAIN, &args![count.wrapping_mul(0x3c)])
        .u32()
}

// Translated from 00658d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: forwards to the destructor body `00658d50` (`thiscall`, no
/// stack words).
pub fn fn_00658d30(e: &mut Engine, this: Ptr) {
    fn_00658d50(e, this);
}

// Translated from 00658d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00658d50 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the string map's destructor body (`thiscall`, no stack words;
/// virtual table `01065890`): a map that copies keys (+0x10) frees each item's key (+4,
/// `00aa10f0`) over all buckets (+4 count, +8 buckets), then the base destructor `00658e10`.
pub fn fn_00658d50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0106_5890);
    if e.mem.u8(base.wrapping_add(0x10)) != 0 {
        let mut bucket = 0u32;
        while bucket < e.mem.u32(base.wrapping_add(4)) {
            let buckets = e.mem.u32(base.wrapping_add(8));
            let mut item = e.mem.u32(buckets.wrapping_add(bucket * 4));
            while item != 0 {
                let next = e.mem.u32(item);
                let key = e.mem.u32(item.wrapping_add(4));
                e.call(0x00aa_10f0, &args![key]);
                item = next;
            }
            bucket += 1;
        }
    }
    fn_00658e10(e, this);
}

// Translated from 00658e10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00658e10 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map: the next base's destructor body (`thiscall`, no stack words;
/// virtual table `010658b0`): the map's clean-up (`00438af0`), then `00658e70`.
pub fn fn_00658e10(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_58b0);
    e.call(MAP_CLEAN_UP, &args![this]);
    fn_00658e70(e, this);
}

// Translated from 00658e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the map base's destructor body (`thiscall`, no stack words;
/// virtual table `010658d0`): the clean-up (`00438af0`), then the bucket array (+8) freed
/// (`00aa10f0`).
pub fn fn_00658e70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_58d0);
    e.call(MAP_CLEAN_UP, &args![this]);
    let buckets = e.mem.u32(this.addr().wrapping_add(8));
    e.call(0x00aa_10f0, &args![buckets]);
}

// Translated from 00659260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `_Destroy_range` for 0x3c-byte elements (cdecl, four words; the
/// iterator category is not read): each element from `first` to `last` destroyed by the
/// allocator (`006592f0`).
pub fn fn_00659260(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    let mut element = first;
    while element != last {
        e.call(0x0065_92f0, &args![allocator, element]);
        element = element.wrapping_add(0x3c);
    }
}

// Translated from 00659ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a model's constructor (`thiscall`, no stack words): the base
/// constructor `004968b0`, its virtual table `01065af4`, +8 and +0xc cleared. Answers `this`.
pub fn fn_00659ae0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(ENTRY_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, 0x0106_5af4);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    this
}

// Translated from 00652130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the face-gen lock's depth counter (`011d59dc`). No arguments.
pub fn fn_00652130(e: &mut Engine) -> u32 {
    e.global(FACE_GEN_LOCK_DEPTH)
}

/// `BSStringT` (8 bytes): the constructor (`004037b0`) and destructor
/// (`004037d0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `strchr` (`0043b9d0`, cdecl: text, character).
const STRCHR: u32 = 0x0043_b9d0;
/// `"FACEGEN:\t%3d\t%s\t%iKB\t%iKB\r\n"`, `"%s.nif"`, `"%s_50.NIF"` and
/// `"Meshes\%s"`.
const MODEL_LINE_FORMAT: u32 = 0x0106_567c;
const NIF_FORMAT: u32 = 0x0106_578c;
const LOW_LOD_NIF_FORMAT: u32 = 0x0106_57d8;
const MESHES_FORMAT: u32 = 0x0101_9710;
/// The model map's position walk: `004b9ba0` (the first position) and
/// `NiTMapBase<..>::GetNext` (engine map name of `00559120`, `RET 0xC`:
/// position, key and `NiPointer` out); `00726070` (the word at +4).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x0055_9120;
const WORD_AT_4: u32 = 0x0072_6070;
/// The coordinate block's statistics index (`011d59f0`) and the direction
/// byte the debug sweep flips (`011d59e1`).
const SWEEP_INDEX: u32 = 0x011d_59f0;
const SWEEP_DOWN: u32 = 0x011d_59e1;
/// 0.1 (`0101ffa0`), 10.0 (`01020758`) and -10.0 (`01065770`), `double`s.
const SWEEP_STEP: u32 = 0x0101_ffa0;
const SWEEP_TOP: u32 = 0x0102_0758;
const SWEEP_BOTTOM: u32 = 0x0106_5770;
/// 0.0 (`01012060`) and 1.0 (`01012070`), `double`s.
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `fr2Assert` (`00aafb90`, cdecl: file, line) and `"d:\_fallout3\...\matrixVT.hpp"`.
const FR2_ASSERT: u32 = 0x00aa_fb90;
const MATRIX_VT_FILE: u32 = 0x0106_5830;

// Translated from 0064fe80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064fe80 - the FS:[0] exception frame (C++ unwinding of the stack NiPointer and string) is not translated
/// `BSFaceGenModelMap::OutputModelMapContents` (Xbox PDB; `RET 4`: an
/// optional output). Walks the map (+4) through a stack `NiPointer`; for
/// each entry with a model, formats `"FACEGEN:\t%3d\t%s\t%iKB\t%iKB\r\n"`
/// with the model's +4 word minus 1, the key, and the `0065c110` and
/// `0065c490` sizes over 1024 (the entry's model at +8), prints it
/// (`005b5e40`) and, with an output, gives it its vtable slot +0x38 with 1.
pub fn bs_face_gen_model_map_output_model_map_contents(e: &mut Engine, this: Ptr, output: u32) {
    let map = this.addr() + 4;
    // The locals: string (-0x20, 8 bytes), NiPointer (-0x18), key (-0x14),
    // position (-0x10).
    e.with_stack(0x14, |e, block| {
        let string = block.addr();
        let pointer = string + 8;
        let key = string + 0xc;
        let position = string + 0x10;
        let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
        e.mem.set_u32(position, first);
        e.call(POINTER_CONSTRUCT, &args![pointer, 0u32]);
        while e.mem.u32(position) != 0 {
            e.call(MAP_GET_NEXT, &args![map, position, key, pointer]);
            if e.call(POINTER_GET, &args![pointer]).u32() == 0 {
                continue;
            }
            e.call(STRING_CONSTRUCT, &args![string]);
            let model = |e: &mut Engine| {
                let entry = e.call(POINTER_GET, &args![pointer]).u32();
                e.call(POINTER_GET, &args![entry.wrapping_add(8)]).u32()
            };
            let first = model(e);
            let first = e.call(0x0065_c490, &args![first]).u32() >> 10;
            let second = model(e);
            let second = e.call(0x0065_c110, &args![second]).u32() >> 10;
            let name = e.mem.u32(key);
            let third = model(e);
            let count = e.call(WORD_AT_4, &args![third]).u32().wrapping_sub(1);
            e.call(
                STRING_FORMAT,
                &args![string, MODEL_LINE_FORMAT, count, name, second, first],
            );
            let text = e.call(POINTER_GET, &args![string]).u32();
            e.call(PRINT_005B5E40, &args![text]);
            if output != 0 {
                e.vcall(output, 0x38, &args![string, 1u32]);
            }
            e.call(STRING_DESTRUCT, &args![string]);
        }
        e.call(POINTER_DESTRUCT, &args![pointer]);
    });
}

// Translated from 00652320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00652320 - the FS:[0] exception frame (C++ unwinding of the two stack matrices) is not translated
/// Sets statistic `index` of the coordinate block's matrix (`row`,
/// `column`) to `value` (cdecl, five words; the engine map has no name):
/// for a block and an index below the statistics' count (`00657a30`), the
/// statistic's matrix (`006587a0` of the statistics entry into a stack
/// matrix) scaled by `value` minus the current statistic ([`fn_00652230`];
/// `FR2MatrixVTC<float>::operator*`, [`fn_00658260`]) is added to the
/// block's matrix (`006586c0`); both stack matrices are destroyed
/// (`00449680`).
pub fn fn_00652320(e: &mut Engine, block: u32, row: u32, column: u32, index: u32, value: f32) {
    if block == 0 {
        return;
    }
    let entries = statistics(e, row, column);
    // JNC: an unsigned compare.
    if index >= fn_00657a30(e, entries) as u32 {
        return;
    }
    let current = fn_00652230(e, block, row, column, index).to_bits();
    let difference = x87_store32(x87_sub(x87_load32(value.to_bits()), x87_m32(current)));
    // The stack matrices: the product at -0x54 and the statistic at -0x34.
    e.with_stack(0x40, |e, matrices| {
        let product = matrices.addr();
        let statistic = product + 0x20;
        let entries = statistics(e, row, column);
        let entry = fn_00657a50(e, entries, index);
        let made = e.call(0x0065_87a0, &args![entry, statistic]).u32();
        let scaled = fn_00658260(
            e,
            Ptr::new(made),
            Ptr::new(product),
            f32::from_bits(x87_quiet_bits(difference)),
        );
        let matrix = block
            .wrapping_add(row.wrapping_shl(6))
            .wrapping_add(column.wrapping_shl(5));
        e.call(0x0065_86c0, &args![matrix, scaled]);
        e.call(ELEMENT_DESTRUCT_00449680, &args![product]);
        e.call(ELEMENT_DESTRUCT_00449680, &args![statistic]);
    });
}

// Translated from 00652470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets one of two values (cdecl, four words: a key, the slot, a second
/// key, the value; the engine map has no name): two floats on the stack,
/// the slot's being the value (`FLD`/`FSTP`) and the other's `00652440`'s
/// answer for its index, given to [`fn_006524e0`]; that call's `EAX` is the
/// answer.
pub fn fn_00652470(e: &mut Engine, first: u32, slot: u32, second: u32, value: f32) -> u32 {
    e.with_stack(8, |e, values| {
        for index in 0..2u32 {
            let word = if index == slot {
                x87_quiet_bits(value.to_bits())
            } else {
                x87_store32(fn_00652440(e, first, index, second))
            };
            e.mem.set_u32(values.addr() + 4 * index, word);
        }
        fn_006524e0(e, first, second, values.addr())
    })
}

// Translated from 006524e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00aac170` on the manager's +0x118 with 0, the second and third words
/// and the first (cdecl, three words; the engine map has no name); its
/// `EAX` is the answer.
pub fn fn_006524e0(e: &mut Engine, first: u32, second: u32, third: u32) -> u32 {
    let manager = bs_face_gen_manager_get_instance(e);
    e.call(
        0x00aa_c170,
        &args![manager.wrapping_add(0x118), 0u32, second, third, first],
    )
    .u32()
}

// Translated from 00652510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00652510 - the FS:[0] exception frame (C++ unwinding of the three stack coordinate blocks) is not translated
/// A debug sweep of an actor's face (cdecl, one word: the actor; the
/// engine map has no name). With the base form (`007af430`) and vtable
/// +0x1D0 (then its +0xC, a value nothing reads), three stack blocks of four
/// matrices are made; the first two take the form's face coordinates
/// (`TESNPC::GetFaceCoord`). The statistic at `011d59f0` of the first moves
/// by 0.1 up or down (`011d59e1`; turning at 10 and at -10 or below) and is
/// set in the second ([`fn_00652320`]); the third takes the difference
/// ([`fn_00652e00`]). Over two lists (vtable +0x1AC, then +0x1B0) each
/// item's +0x1C object with an `00607310` object whose +0x94 answers
/// something gets the third block (`0065a970`). Last [`fn_00652e00`] runs on
/// `005d9fb0(00726070(form + 0x10C), sex)`, the second block and
/// `00601800`'s answer, with the 0 the exe pushed earlier as its fourth
/// word; the blocks are destroyed.
pub fn fn_00652510(e: &mut Engine, actor: u32) {
    let form = e.call(0x007a_f430, &args![actor]).u32();
    let object = e.vcall(actor, 0x1d0, &args![]).u32();
    if object != 0 {
        e.vcall(object, 0xc, &args![]);
    }
    // The blocks: the first at -0x11C, the second at -0x1A4, the third at
    // -0x8C (4 matrices of 0x20 bytes each).
    e.with_stack(0x180, |e, blocks| {
        let first = blocks.addr();
        let second = first + 0x80;
        let third = first + 0x100;
        for block in [first, second, third] {
            e.call(
                VECTOR_CONSTRUCT,
                &args![
                    block,
                    0x20u32,
                    4u32,
                    ELEMENT_CONSTRUCT_00449610,
                    ELEMENT_DESTRUCT_00449680
                ],
            );
        }
        e.call(0x0060_3ad0, &args![form, first]);
        e.call(0x0060_3ad0, &args![form, second]);
        let index = e.global::<u32>(SWEEP_INDEX);
        let mut value = fn_00652230(e, first, 0, 0, index).to_bits();
        if e.global::<u8>(SWEEP_DOWN) != 0 {
            value = x87_store32(x87_add(x87_load32(value), e.global::<f64>(SWEEP_STEP)));
            // FCOMP; TEST AH,1; JNZ: turns once no longer below 10.
            if !(x87_load32(value) < e.global::<f64>(SWEEP_TOP)) {
                e.set_global(SWEEP_DOWN, 0u8);
            }
        } else {
            value = x87_store32(x87_sub(x87_load32(value), e.global::<f64>(SWEEP_STEP)));
            // FCOMP; TEST AH,0x41; JP: turns at -10 or below (not NaN).
            if x87_load32(value) <= e.global::<f64>(SWEEP_BOTTOM) {
                e.set_global(SWEEP_DOWN, 1u8);
            }
        }
        let index = e.global::<u32>(SWEEP_INDEX);
        fn_00652320(
            e,
            second,
            0,
            0,
            index,
            f32::from_bits(x87_quiet_bits(value)),
        );
        fn_00652e00(e, first, second, third, 0);
        let mut list = e.vcall(actor, 0x1ac, &args![0u32]).u32();
        for _ in 0..2 {
            let count = e.call(0x0043_b480, &args![list]).u32();
            let mut item = 0u32;
            while item < count {
                let entry = e.call(0x0043_b4a0, &args![list, item]).u32();
                let model = if entry == 0 {
                    0
                } else {
                    e.vcall(entry, 0x1c, &args![]).u32()
                };
                if model != 0 {
                    let owner = e.call(OBJECT_OF_00607310, &args![model]).u32();
                    let target = if owner == 0 {
                        0
                    } else {
                        e.vcall(owner, 0x94, &args![]).u32()
                    };
                    if target != 0 {
                        e.call(0x0065_a970, &args![target, third, model, 0u32]);
                    }
                }
                item = item.wrapping_add(1);
            }
            list = e.vcall(actor, 0x1b0, &args![0u32]).u32();
        }
        let face = e.call(0x0060_1800, &args![form]).u32();
        let sex = e.call(0x005f_0cc0, &args![form]).u32();
        let holder = e.call(WORD_AT_4, &args![form.wrapping_add(0x10c)]).u32();
        let source = e.call(0x005d_9fb0, &args![holder, sex]).u32();
        fn_00652e00(e, source, second, face, 0);
        for block in [third, second, first] {
            e.call(
                VECTOR_DESTRUCT,
                &args![block, 0x20u32, 4u32, ELEMENT_DESTRUCT_00449680],
            );
        }
    });
}

// Translated from 00652900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two coordinate blocks differ (cdecl, two words; the engine map
/// has no name): true for two null blocks, false when only one is null,
/// otherwise whether any of the four matrix pairs differs
/// ([`fn_00657880`]).
pub fn fn_00652900(e: &mut Engine, first: u32, second: u32) -> bool {
    if first == 0 && second == 0 {
        return true;
    }
    if first == 0 || second == 0 {
        return false;
    }
    for row in 0..2u32 {
        for column in 0..2u32 {
            let offset = row.wrapping_shl(6).wrapping_add(column.wrapping_shl(5));
            let this = Ptr::new(first.wrapping_add(offset));
            if fn_00657880(e, this, second.wrapping_add(offset)) {
                return true;
            }
        }
    }
    false
}

// Translated from 00652e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The difference of two coordinate blocks (cdecl, four words: two blocks,
/// the result block, a flag; the engine map has no name): nothing unless
/// all three are set. Each result matrix takes the first block's
/// dimensions (`std::vector<float>::resize`, 0 by 0 when one is 0); row 1
/// with the flag copies the first block's matrix
/// (`FR2MatrixVTC<float>::operator=`), otherwise the result is emptied
/// (`0044a0a0`) and gets `second - first` for each value ([`fn_006578e0`]).
pub fn fn_00652e00(e: &mut Engine, first: u32, second: u32, result: u32, flag: u8) {
    if first == 0 || second == 0 || result == 0 {
        return;
    }
    for row in 0..2u32 {
        for column in 0..2u32 {
            let offset = row.wrapping_shl(6).wrapping_add(column.wrapping_shl(5));
            let (a, b, out) = (
                first.wrapping_add(offset),
                second.wrapping_add(offset),
                result.wrapping_add(offset),
            );
            let rows = e.call(MATRIX_FIRST_DIMENSION, &args![a]).u32();
            let columns = e.call(MATRIX_SECOND_DIMENSION, &args![a]).u32();
            if rows == 0 || columns == 0 {
                e.call(MATRIX_RESIZE, &args![out, 0u32, 0u32]);
                continue;
            }
            e.call(MATRIX_RESIZE, &args![out, rows, columns]);
            if row == 1 && flag != 0 {
                fr2_matrix_vtc_operator_assign(e, Ptr::new(out), Ptr::new(a));
                continue;
            }
            let count = rows.wrapping_mul(columns);
            e.call(0x0044_a0a0, &args![out]);
            let mut index = 0u32;
            while index < count {
                let from = fn_006578b0(e, Ptr::new(a), index);
                let low = x87_quiet_bits(e.mem.u32(from));
                let to = fn_006578b0(e, Ptr::new(b), index);
                let high = x87_quiet_bits(e.mem.u32(to));
                let difference = x87_store32(x87_sub(x87_load32(high), x87_m32(low)));
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), difference);
                    fn_006578e0(e, Ptr::new(out), slot);
                });
                index = index.wrapping_add(1);
            }
        }
    }
}

// Translated from 00652fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00aad6b0` on the manager's +0x118 with the float (`FLD`/`FSTP`) and
/// the two words (cdecl, three words; the engine map has no name), when
/// both words are set. Nothing is answered (the other path leaves `EAX`
/// as it was).
pub fn fn_00652fc0(e: &mut Engine, value: f32, first: u32, second: u32) {
    if second == 0 || first == 0 {
        return;
    }
    let manager = bs_face_gen_manager_get_instance(e);
    e.call(
        0x00aa_d6b0,
        &args![
            manager.wrapping_add(0x118),
            x87_quiet_bits(value.to_bits()),
            first,
            second
        ],
    );
}

// Translated from 00653000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales a coordinate block (cdecl, three words: a float, the block, a
/// flag; the engine map has no name): for a block and a value not below 0
/// and not above 1 (a NaN passes both tests), each matrix with both
/// dimensions set is scaled (`FR2MatrixVTC<float>::operator*=`,
/// `00658650`); the row and flag test the exe makes lets every matrix
/// through.
pub fn fn_00653000(e: &mut Engine, value: f32, block: u32, flag: u8) {
    if block == 0 {
        return;
    }
    let loaded = x87_load32(value.to_bits());
    // FCOMP 0.0; TEST AH,5; JNP: out when below. FCOMP 1.0; TEST AH,0x41;
    // JNZ: on when not above.
    if loaded < e.global::<f64>(ZERO_DOUBLE) || loaded > e.global::<f64>(ONE_DOUBLE) {
        return;
    }
    for row in 0..2u32 {
        for column in 0..2u32 {
            let matrix = block
                .wrapping_add(row.wrapping_shl(6))
                .wrapping_add(column.wrapping_shl(5));
            if e.call(MATRIX_FIRST_DIMENSION, &args![matrix]).u32() == 0 {
                continue;
            }
            if e.call(MATRIX_SECOND_DIMENSION, &args![matrix]).u32() == 0 {
                continue;
            }
            let scales = row == 0 || (row == 1 && flag != 0) || flag == 0;
            if scales {
                e.call(0x0065_8650, &args![matrix, x87_quiet_bits(value.to_bits())]);
            }
        }
    }
}

/// `00653620` and `00653c40`: the name cut at the last `cut` character,
/// formatted as `"%s.nif"` into a stack buffer and set into `out`; `None`
/// without that character.
fn cut_model_name(e: &mut Engine, out: u32, name: u32, cut: u32) -> Option<u32> {
    // The copy at -0x218 and the formatted name at -0x110 (0x108 each).
    e.with_stack(0x210, |e, buffers| {
        let copy = buffers.addr();
        let formatted = copy + 0x108;
        e.call(STRCPY_S, &args![copy, 0x104u32, name]);
        let at = e.call(STRRCHR, &args![copy, cut]).u32();
        if at == 0 {
            return None;
        }
        e.mem.set_u8(at, 0);
        e.call(SPRINTF_S, &args![formatted, 0x104u32, NIF_FORMAT, copy]);
        e.call(STRING_SET, &args![out, formatted, 0u32]);
        Some(e.call(POINTER_GET, &args![out]).u32())
    })
}

// Translated from 00653620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00653620 - the /GS stack cookie check (00ec408c) is not translated.
/// The model name without its extension (cdecl, two words: the out
/// string, the name; the engine map has no name): 0 without a name or a
/// `.`; otherwise the name cut at its last `.` with `.nif`, set into the
/// string, whose text is the answer.
pub fn fn_00653620(e: &mut Engine, out: u32, name: u32) -> u32 {
    if name == 0 {
        return 0;
    }
    cut_model_name(e, out, name, 0x2e).unwrap_or(0)
}

// Translated from 00653a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00653a10 - the FS:[0] exception frame (C++ unwinding of the stack string) and the /GS stack cookie check (00ec408c) are not translated
/// `BSFaceGenManager::GetAsNIFFileLowLOD` (Xbox PDB; cdecl: the out
/// string, the name). With a stack string: 0 without a name; a name with a
/// `_` is set into the out string as it is and answered; without a `.` the
/// name is answered; otherwise the name cut at its last `.` with `_50.NIF`
/// is set into the out string, `"Meshes\%s"` of it formatted into the
/// stack string and looked up (`00afd270`, `00af6540` with two stack
/// words made by `006815c0`): the out string's text when found, else 0.
pub fn bs_face_gen_manager_get_as_nif_file_low_lod(e: &mut Engine, out: u32, name: u32) -> u32 {
    // The locals: the string at -0x20, the words at -0x14 and -0x138, the
    // copy at -0x244 and the formatted name at -0x12C (0x108 each).
    e.with_stack(0x228, |e, block| {
        let string = block.addr();
        let second_word = string + 8;
        let first_word = string + 0x10;
        let copy = string + 0x18;
        let formatted = copy + 0x108;
        e.call(STRING_CONSTRUCT, &args![string]);
        let answer = if name == 0 {
            0
        } else {
            e.call(STRCPY_S, &args![copy, 0x104u32, name]);
            if e.call(STRCHR, &args![copy, 0x5fu32]).u32() != 0 {
                e.call(STRING_SET, &args![out, copy, 0u32]);
                name
            } else {
                let dot = e.call(STRRCHR, &args![copy, 0x2eu32]).u32();
                if dot == 0 {
                    name
                } else {
                    e.mem.set_u8(dot, 0);
                    e.call(
                        SPRINTF_S,
                        &args![formatted, 0x104u32, LOW_LOD_NIF_FORMAT, copy],
                    );
                    e.call(STRING_SET, &args![out, formatted, 0u32]);
                    let text = e.call(POINTER_GET, &args![out]).u32();
                    e.call(STRING_FORMAT, &args![string, MESHES_FORMAT, text]);
                    e.call(0x0068_15c0, &args![first_word]);
                    e.call(0x0068_15c0, &args![second_word]);
                    let path = e.call(POINTER_GET, &args![string]).u32();
                    e.call(0x00af_d270, &args![path, first_word, second_word]);
                    let path = e.call(POINTER_GET, &args![string]).u32();
                    let found = e
                        .call(0x00af_6540, &args![0u32, first_word, second_word, path])
                        .u32();
                    if found == 0 {
                        0
                    } else {
                        e.call(POINTER_GET, &args![out]).u32()
                    }
                }
            }
        };
        e.call(STRING_DESTRUCT, &args![string]);
        answer
    })
}

// Translated from 00653c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00653c40 - the /GS stack cookie check (00ec408c) is not translated.
/// The model name without its `_` suffix (cdecl, two words: the out
/// string, the name; the engine map has no name): 0 without a name; the
/// name itself without a `_`; otherwise the name cut at its last `_` with
/// `.nif`, set into the string, whose text is the answer.
pub fn fn_00653c40(e: &mut Engine, out: u32, name: u32) -> u32 {
    if name == 0 {
        return 0;
    }
    cut_model_name(e, out, name, 0x5f).unwrap_or(name)
}

// Translated from 00657880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two matrices differ (`RET 4`: the other; the engine map has no
/// name): not [`fn_00658160`].
pub fn fn_00657880(e: &mut Engine, this: Ptr, other: u32) -> bool {
    !fn_00658160(e, this, other)
}

// Translated from 00658020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x3C-byte elements' allocator's `allocate` (`RET 4`: the count; the
/// engine map has no name): [`fn_00658c10`] with the count and 0; its
/// `EAX` is the answer.
pub fn fn_00658020(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    fn_00658c10(e, count)
}

// Translated from 00658060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x88-byte elements' vector's destroy of `first`..`last` (`RET 8`;
/// the engine map has no name): [`fn_00658c60`] with the vector's
/// allocator (+8).
pub fn fn_00658060(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    fn_00658c60(e, first, last, this.addr().wrapping_add(8));
}

// Translated from 00658090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x88-byte elements' allocator's `allocate` (`RET 4`: the count; the
/// engine map has no name): [`fn_00658ca0`] with the count and 0; its
/// `EAX` is the answer.
pub fn fn_00658090(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    fn_00658ca0(e, count)
}

// Translated from 00658100 (decompiled, FalloutNV.exe 1.4.0.525)
/// An allocator's `max_size()` for 0x3C-byte elements: `0x4444444` (the
/// library's `count > 0 ? count : 1`, whose second arm the constant makes
/// unreachable but which is translated as written).
pub fn fn_00658100(_e: &mut Engine, _this: Ptr) -> u32 {
    let count: u32 = std::hint::black_box(0x0444_4444);
    if count > 0 {
        count
    } else {
        1
    }
}

// Translated from 00658130 (decompiled, FalloutNV.exe 1.4.0.525)
/// An allocator's `max_size()` for 0x88-byte elements: `0x1E1E1E1` (as
/// [`fn_00658100`]).
pub fn fn_00658130(_e: &mut Engine, _this: Ptr) -> u32 {
    let count: u32 = std::hint::black_box(0x01e1_e1e1);
    if count > 0 {
        count
    } else {
        1
    }
}

// Translated from 00658160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two matrices are equal (`RET 4`: the other; the engine map has
/// no name): the same dimensions (+0x18, +0x1C) and, when both are set,
/// the same `rows * 4 * columns` value bytes (`_memcmp` of the two
/// matrices' element 0, [`fn_006578b0`]).
pub fn fn_00658160(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    if e.mem.u32(other + 0x18) != e.mem.u32(base + 0x18)
        || e.mem.u32(other + 0x1c) != e.mem.u32(base + 0x1c)
    {
        return false;
    }
    if e.mem.u32(base + 0x18) == 0 || e.mem.u32(base + 0x1c) == 0 {
        return true;
    }
    let size = e
        .mem
        .u32(base + 0x18)
        .wrapping_shl(2)
        .wrapping_mul(e.mem.u32(base + 0x1c));
    let theirs = fn_006578b0(e, Ptr::new(other), 0);
    let own = fn_006578b0(e, this, 0);
    e.call(0x00ec_4835, &args![own, theirs, size]).u32() == 0
}

// Translated from 00658500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00658500 - the FS:[0] exception frame (C++ unwinding of the stack matrix) is not translated
/// `FR2MatrixVTC<float>::operator+` (Xbox PDB `operator__ov2`; `RET 8`: the
/// result, the other matrix): `fr2Assert` (`matrixVT.hpp`, line 0x16D)
/// when the dimensions differ; a stack matrix of this one's dimensions
/// (`00659000`) gets each `this + other` value through three iterators
/// (`0044a170`, `0065fd10`; `0065fd40`/`0065dfd0` the end test,
/// `0064dda0`/`0064dbd0` the values, `00659100`/`006590c0` the steps);
/// the result is copied from it (`00658f30`) and it is destroyed
/// (`00449680`). Returns the result.
pub fn fr2_matrix_vtc_operator_add(e: &mut Engine, this: Ptr, result: u32, other: u32) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 0x18) != e.mem.u32(other + 0x18)
        || e.mem.u32(base + 0x1c) != e.mem.u32(other + 0x1c)
    {
        e.call(FR2_ASSERT, &args![MATRIX_VT_FILE, 0x16du32]);
    }
    // The frame below EBP from -0x74 up: `at(x)` is `EBP - x`.
    e.with_stack(0x68, |e, frame| {
        let at = |x: u32| frame.addr() + 0x74 - x;
        let sum = at(0x3c);
        let (own, theirs, out) = (at(0x44), at(0x1c), at(0x14));
        let rows = e.mem.u32(base + 0x18);
        let columns = e.mem.u32(base + 0x1c);
        e.call(0x0065_9000, &args![sum, rows, columns]);
        e.call(0x0044_a170, &args![sum, out]);
        e.call(0x0065_fd10, &args![this, own]);
        e.call(0x0065_fd10, &args![other, theirs]);
        loop {
            let end = e.call(0x0065_fd40, &args![this, at(0x64)]).u32();
            if !e.call(0x0065_dfd0, &args![own, end]).bool() {
                break;
            }
            let a = e.call(0x0064_dda0, &args![own]).u32();
            let b = e.call(0x0064_dda0, &args![theirs]).u32();
            let value = x87_store32(x87_add(x87_load32(e.mem.u32(a)), x87_m32(e.mem.u32(b))));
            let slot = e.call(0x0064_dbd0, &args![out]).u32();
            e.mem.set_u32(slot, x87_quiet_bits(value));
            e.call(0x0065_9100, &args![own, at(0x4c), 0u32]);
            e.call(0x0065_9100, &args![theirs, at(0x54), 0u32]);
            e.call(0x0065_90c0, &args![out, at(0x5c), 0u32]);
        }
        e.call(0x0065_8f30, &args![result, sum]);
        e.call(ELEMENT_DESTRUCT_00449680, &args![sum]);
    });
    result
}

// Translated from 00658c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Destroy_range` of `first`..`last` with `allocator` for 0x88-byte
/// elements (cdecl, three words; the engine map has no name): the iterator
/// category (`0065e750` on the two argument words) and [`fn_00659290`].
pub fn fn_00658c60(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    e.with_stack(8, |e, words| {
        let words = words.addr();
        e.mem.set_u32(words, first);
        e.mem.set_u32(words + 4, last);
        let category = e.call(0x0065_e750, &args![words, words + 4]).u8();
        let first = e.mem.u32(words);
        let last = e.mem.u32(words + 4);
        fn_00659290(e, first, last, allocator, u32::from(category));
    });
}

// Translated from 00658ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `count` 0x88-byte elements (`00401000`; cdecl, one word read);
/// a count whose size overflows throws `std::bad_alloc` (`0044d6c0`,
/// `_CxxThrowException` `00ec7894` with `01140ef8`). A count of 0
/// allocates 0 bytes.
pub fn fn_00658ca0(e: &mut Engine, count: u32) -> u32 {
    // (The exe stores 0 back into a count of 0.)
    if count != 0 && u32::MAX / count < 0x88 {
        e.with_stack(0xc, |e, exception| {
            e.call(0x0044_d6c0, &args![exception, 0u32]);
            e.call(0x00ec_7894, &args![exception, 0x0114_0ef8u32]);
        });
    }
    e.call(ALLOCATE_PLAIN, &args![count.wrapping_mul(0x88)])
        .u32()
}

/// The scalar deleting destructors below: the body, then the free
/// (`00401030`) when bit 0 of the flags is set. Returns the map.
fn scalar_delete(e: &mut Engine, this: Ptr, flags: u32, body: fn(&mut Engine, Ptr)) -> Ptr {
    body(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00658d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<NiPointer<BSFaceGenModelMap::Entry>>`
/// scalar deleting destructor (engine map; `RET 4`): [`fn_00658bb0`], then
/// the free when bit 0 is set.
pub fn bst_case_insensitive_string_map_model_map_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, fn_00658bb0)
}

// Translated from 00658ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<..>,char const *,NiPointer<BSFaceGenModelMap::Entry>>`
/// scalar deleting destructor (engine map; `RET 4`): [`fn_00658e70`], then
/// the free when bit 0 is set.
pub fn ni_t_map_base_model_map_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, fn_00658e70)
}

// Translated from 00658ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<char const *,NiPointer<BSFaceGenModelMap::Entry>>` scalar
/// deleting destructor (engine map; `RET 4`): [`fn_00658e10`], then the
/// free when bit 0 is set.
pub fn ni_t_map_model_map_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, fn_00658e10)
}

// Translated from 00658f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTMap<..>,NiPointer<BSFaceGenModelMap::Entry>>`
/// scalar deleting destructor (engine map; `RET 4`): [`fn_00658d50`], then
/// the free when bit 0 is set.
pub fn ni_t_string_template_map_model_map_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, fn_00658d50)
}

// Translated from 00659290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Destroy_range` for 0x88-byte elements (cdecl, four words; the iterator
/// category is not read; the engine map has no name): each element from
/// `first` to `last` destroyed by the allocator (`00659310`).
pub fn fn_00659290(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    let mut element = first;
    while element != last {
        e.call(0x0065_9310, &args![allocator, element]);
        element = element.wrapping_add(0x88);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0064_fe80,
            bs_face_gen_model_map_output_model_map_contents(Ptr, u32)
        ),
        entry!(0x0065_2320, fn_00652320(u32, u32, u32, u32, f32)),
        entry!(0x0065_2470, fn_00652470(u32, u32, u32, f32) -> u32),
        entry!(0x0065_24e0, fn_006524e0(u32, u32, u32) -> u32),
        entry!(0x0065_2510, fn_00652510(u32)),
        entry!(0x0065_2900, fn_00652900(u32, u32) -> bool),
        entry!(0x0065_2e00, fn_00652e00(u32, u32, u32, u8)),
        entry!(0x0065_2fc0, fn_00652fc0(f32, u32, u32)),
        entry!(0x0065_3000, fn_00653000(f32, u32, u8)),
        entry!(0x0065_3620, fn_00653620(u32, u32) -> u32),
        entry!(
            0x0065_3a10,
            bs_face_gen_manager_get_as_nif_file_low_lod(u32, u32) -> u32
        ),
        entry!(0x0065_3c40, fn_00653c40(u32, u32) -> u32),
        entry!(0x0065_7880, fn_00657880(Ptr, u32) -> bool),
        entry!(0x0065_8020, fn_00658020(Ptr, u32) -> u32),
        entry!(0x0065_8060, fn_00658060(Ptr, u32, u32)),
        entry!(0x0065_8090, fn_00658090(Ptr, u32) -> u32),
        entry!(0x0065_8100, fn_00658100(Ptr) -> u32),
        entry!(0x0065_8130, fn_00658130(Ptr) -> u32),
        entry!(0x0065_8160, fn_00658160(Ptr, u32) -> bool),
        entry!(
            0x0065_8500,
            fr2_matrix_vtc_operator_add(Ptr, u32, u32) -> u32
        ),
        entry!(0x0065_8c60, fn_00658c60(u32, u32, u32)),
        entry!(0x0065_8ca0, fn_00658ca0(u32) -> u32),
        entry!(
            0x0065_8d00,
            bst_case_insensitive_string_map_model_map_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0065_8ea0,
            ni_t_map_base_model_map_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0065_8ed0,
            ni_t_map_model_map_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0065_8f00,
            ni_t_string_template_map_model_map_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0065_9290, fn_00659290(u32, u32, u32, u32)),
        entry!(0x0065_2130, fn_00652130() -> u32),
        entry!(0x00b9_fcc0, ni_t_pointer_list_base_remove(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0065_8bb0, fn_00658bb0(Ptr)),
        entry!(0x00653290, fn_00653290(Ptr, u32) -> Ptr),
        entry!(0x00486b50, is_keys_equal(Ptr, u32, u32) -> bool),
        entry!(0x00600b20, key_to_hash_index(Ptr, u32) -> u32),
        entry!(0x00659210, fn_00659210(Ptr, u8, u32) -> u32),
        entry!(0x0065_7fd0, fn_00657fd0(Ptr) -> u32),
        entry!(0x0065_8040, fn_00658040(Ptr) -> u32),
        entry!(0x006521e0, bs_face_gen_manager_get_face_gen_coord() -> u32),
        entry!(
            0x006529a0,
            bs_face_gen_manager_copy_face_gen_coord(u32, u32, u8, u8)
        ),
        entry!(0x00b9_efa0, ni_t_pointer_list_base_remove_pos(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0064_fd80, fn_0064fd80(Ptr) -> Ptr),
        entry!(0x0064_fde0, bs_face_gen_model_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0064_fe10, fn_0064fe10(Ptr)),
        entry!(0x0064_ffd0, fn_0064ffd0(Ptr, u32, u32)),
        entry!(0x0065_0100, fn_00650100(Ptr) -> Ptr),
        entry!(0x0065_0170, bs_face_gen_model_map_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0065_01a0, fn_006501a0(Ptr)),
        entry!(0x0065_0200, fn_00650200(Ptr, u32, u32) -> bool),
        entry!(0x0065_0480, fn_00650480(Ptr, u32) -> bool),
        entry!(0x0065_0630, fn_00650630(Ptr, u32) -> bool),
        entry!(0x0065_08e0, fn_006508e0(Ptr, u32, u32)),
        entry!(0x0065_0ea0, bs_face_gen_manager_bs_face_gen_manager(Ptr) -> Ptr),
        entry!(0x0065_1450, fn_00651450(Ptr) -> Ptr),
        entry!(0x0065_1520, fn_00651520(Ptr) -> Ptr),
        entry!(0x0065_15b0, fn_006515b0(Ptr)),
        entry!(0x0065_1630, fn_00651630(Ptr) -> Ptr),
        entry!(0x0065_1650, fn_00651650(Ptr)),
        entry!(0x0065_1670, fn_00651670(Ptr) -> Ptr),
        entry!(0x0065_16a0, fn_006516a0(Ptr)),
        entry!(0x0065_16c0, fn_006516c0(Ptr, u8)),
        entry!(0x0065_16e0, fn_006516e0(Ptr) -> Ptr),
        entry!(0x0065_1760, fn_00651760(Ptr)),
        entry!(0x0065_17e0, fn_006517e0(Ptr) -> Ptr),
        entry!(0x0065_1810, fn_00651810(Ptr)),
        entry!(0x0065_1830, fn_00651830(Ptr)),
        entry!(0x0065_18e0, fn_006518e0(Ptr)),
        entry!(0x0065_19f0, bs_face_gen_manager_initialize_system()),
        entry!(0x0065_1ab0, fn_00651ab0()),
        entry!(0x0065_1b00, fn_00651b00(Ptr, u32) -> Ptr),
        entry!(0x0065_1b30, bs_face_gen_manager_get_instance() -> u32),
        entry!(0x0065_1b50, fn_00651b50(u32, u32, u32, u32, u8, u32, u8) -> u32),
        entry!(0x0065_1e30, fn_00651e30()),
        entry!(0x0065_1e70, fn_00651e70(Ptr)),
        entry!(0x0065_1f40, fn_00651f40()),
        entry!(0x0065_1f70, fn_00651f70(u32) -> bool),
        entry!(0x0065_2040, fn_00652040(u32) -> bool),
        entry!(0x0065_2110, bs_face_gen_manager_get_model_cache() -> u32),
        entry!(0x0065_2140, fn_00652140()),
        entry!(0x0065_2160, fn_00652160() -> u8),
        entry!(0x0065_2190, fn_00652190()),
        entry!(0x0065_21b0, fn_006521b0(u32, u32) -> i32),
        entry!(0x0065_21f0, bs_face_gen_manager_init_face_gen_coord(u32)),
        entry!(0x0065_2230, fn_00652230(u32, u32, u32, u32) -> f32),
        entry!(0x0065_2440, fn_00652440(u32, u32, u32) -> f64),
        entry!(0x0065_2af0, fn_00652af0(u32, u32, u32, u8, f32)),
        entry!(0x0065_30e0, fn_006530e0(u32) -> u32),
        entry!(0x0065_3120, fn_00653120(u32) -> u32),
        entry!(0x0065_3180, fn_00653180(Ptr) -> u32),
        entry!(0x0065_3230, fn_00653230(u32) -> u32),
        entry!(0x0065_3270, fn_00653270(u32, u32) -> u32),
        entry!(0x0065_32c0, fn_006532c0(Ptr, u32) -> bool),
        entry!(0x0065_3300, fn_00653300(u32) -> u32),
        entry!(0x0065_3340, fn_00653340(u32, u32)),
        entry!(0x0065_33f0, fn_006533f0(Ptr, u32)),
        entry!(0x0065_3430, fn_00653430(u32) -> u32),
        entry!(0x0065_3460, fn_00653460() -> u32),
        entry!(0x0065_3470, fn_00653470(u32, f32)),
        entry!(0x0065_3520, bs_face_gen_manager_get_as_egm_file(u32, u32, i32) -> u32),
        entry!(0x0065_36d0, bs_face_gen_manager_get_as_tri_file(u32, u32, i32) -> u32),
        entry!(0x0065_37d0, fn_006537d0(u32, u32) -> u32),
        entry!(0x0065_3880, bs_face_gen_manager_get_as_nearest_detail_dds_file(u32, i32, u8, u32) -> u32),
        entry!(0x0065_3cf0, fn_00653cf0(u32) -> i32),
        entry!(0x0065_3d70, fn_00653d70(f32, f32, Ptr, Ptr)),
        entry!(0x0065_3dc0, fn_00653dc0(u32, u32, u8)),
        entry!(0x0065_3e20, bs_face_gen_manager_set_to_zero(u32, u32)),
        entry!(
            0x0065_7010,
            bs_face_gen_manager_precache_face_geometry(u32, u32, u32)
        ),
        entry!(
            0x0065_70e0,
            bs_face_gen_manager_clear_body_textures_from_palette(u32)
        ),
        entry!(0x0065_7150, fn_00657150(u32, u32, u32)),
        entry!(0x0065_7820, fn_00657820(u32, i32) -> u32),
        entry!(0x0065_78b0, fn_006578b0(Ptr, u32) -> u32),
        entry!(0x0065_78e0, fn_006578e0(Ptr, Ptr)),
        entry!(0x0065_7960, fn_00657960(Ptr) -> Ptr),
        entry!(0x0065_79d0, fn_006579d0(Ptr)),
        entry!(0x0065_7a30, fn_00657a30(Ptr) -> i32),
        entry!(0x0065_7a50, fn_00657a50(Ptr, u32) -> u32),
        entry!(0x0065_7a80, fn_00657a80(Ptr) -> Ptr),
        entry!(0x0065_7af0, fn_00657af0(Ptr)),
        entry!(0x0065_7b50, fn_00657b50(Ptr) -> Ptr),
        entry!(0x0065_7bc0, fn_00657bc0(Ptr) -> i32),
        entry!(0x0065_7be0, fn_00657be0(Ptr)),
        entry!(0x0065_7c00, fn_00657c00(Ptr) -> i32),
        entry!(0x0065_7c20, fn_00657c20(Ptr, u32) -> u32),
        entry!(0x0065_7c50, fn_00657c50(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0065_7cd0, fn_00657cd0(Ptr, u32) -> u8),
        entry!(0x0065_7d50, fn_00657d50(Ptr)),
        entry!(0x0065_7dd0, fn_00657dd0(Ptr, u32) -> u8),
        entry!(0x0065_7e60, fn_00657e60(Ptr)),
        entry!(0x0065_7ee0, fn_00657ee0(Ptr, u32) -> u8),
        entry!(0x0065_7f60, fn_00657f60(Ptr)),
        entry!(0x0065_7ff0, fn_00657ff0(Ptr, u32, u32)),
        entry!(0x0065_80b0, fn_006580b0(Ptr, Ptr) -> i32),
        entry!(0x0065_81e0, fr2_matrix_vtc_operator_assign(Ptr, Ptr) -> Ptr),
        entry!(0x0065_8260, fn_00658260(Ptr, Ptr, f32) -> Ptr),
        entry!(0x0065_8360, fn_00658360(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0065_8950, fn_00658950(Ptr, u32, u32)),
        entry!(
            0x0065_8ad0,
            ni_t_string_template_map_set_value(Ptr, u32, u32, u32)
        ),
        entry!(0x0065_8b80, fn_00658b80(Ptr, u32)),
        entry!(0x0065_8bd0, fn_00658bd0(u32, u32, u32)),
        entry!(0x0065_8c10, fn_00658c10(u32) -> u32),
        entry!(0x0065_8d30, fn_00658d30(Ptr)),
        entry!(0x0065_8d50, fn_00658d50(Ptr)),
        entry!(0x0065_8e10, fn_00658e10(Ptr)),
        entry!(0x0065_8e70, fn_00658e70(Ptr)),
        entry!(0x0065_9260, fn_00659260(u32, u32, u32, u32)),
        entry!(0x0065_9ae0, fn_00659ae0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_equal_when_the_compare_answers_zero() {
        let mut e = Engine::new();
        e.register(STRICMP_WRAPPER, |_, a| u32::from(a[0] != a[1]).into_ret());
        assert_eq!(e.call(0x00486b50, &[0, 5, 5]).u32() & 0xff, 1);
        assert_eq!(e.call(0x00486b50, &[0, 5, 6]).u32() & 0xff, 0);
    }

    #[test]
    fn hashes_the_lowered_key_modulo_the_bucket_count() {
        let mut e = Engine::new();
        let map = e.mem.alloc(8);
        e.mem.set_u32(map + 4, 37);
        let key = e.mem.alloc(8);
        e.mem.write(key, b"fAb\0");
        let mut h = 0u32;
        for c in b"fab" {
            h = h.wrapping_mul(33).wrapping_add(u32::from(*c));
        }
        assert_eq!(e.call(0x00600b20, &[map, key]).u32(), h % 37);
    }

    #[test]
    fn a_high_byte_is_sign_extended() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00659210, &[0, 0xe9, 1]).u32(),
            33u32.wrapping_sub(0x17)
        );
    }

    #[test]
    fn fn_00653290_returns_this_only_when_the_check_passes() {
        let mut e = Engine::new();
        e.register(CHECK_006532C0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let this = Ptr::<()>::new(0x5000);
        assert_eq!(e.call(0x0065_3290, &args![this, 1u32]).ptr::<()>(), this);
        assert_eq!(
            e.call(0x0065_3290, &args![this, 0u32]).ptr::<()>(),
            Ptr::<()>::NULL
        );
    }

    #[test]
    fn l68d_00657fd0_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00658100, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_7fd0, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00658100).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }

    #[test]
    fn l68d_00658040_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00658130, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_8040, &args![0x4200u32]).u32(), 0x175d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00658130).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x8]);
    }
}

#[cfg(test)]
mod coord_tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn the_coordinate_block_is_8_bytes_into_the_manager() {
        let mut e = Engine::new();
        e.register(FACE_GEN_GET_INSTANCE, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        assert_eq!(e.call(0x006521e0, &[]).u32(), 0x5008);
    }

    /// Runs the copy on blocks at 0x10000 (source) and 0x20000 (destination).
    /// The source matrix at offset `index * 0x20` has the dimensions
    /// `dimensions[index]` (rows, columns).
    fn copy(dimensions: [(u32, u32); 4], first: u32, second: u32) -> CallLog {
        let mut e = Engine::new();
        e.map(0x10000, 0x100);
        for (index, (rows, columns)) in dimensions.iter().enumerate() {
            e.mem.set_u32(0x10000 + index as u32 * 0x20 + 0x18, *rows);
            e.mem
                .set_u32(0x10000 + index as u32 * 0x20 + 0x1c, *columns);
        }
        e.register(MATRIX_FIRST_DIMENSION, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x18),
            ..Ret::default()
        });
        e.register(MATRIX_SECOND_DIMENSION, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x1c),
            ..Ret::default()
        });
        e.register(MATRIX_RESIZE, |_, _| Ret::default());
        e.register(MATRIX_ASSIGN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006529a0, &[0x10000, 0x20000, first, second]);
        e.call_log.take().unwrap()
    }

    #[test]
    fn null_blocks_do_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x006529a0, &[0, 0x20000, 1, 1]);
        e.call(0x006529a0, &[0x10000, 0, 1, 1]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn equal_flags_copy_every_matrix_with_dimensions() {
        let log = copy([(2, 3), (4, 5), (6, 7), (8, 9)], 0, 0);
        assert_eq!(
            calls_to(&log, MATRIX_RESIZE),
            vec![
                vec![0x20000, 2, 3],
                vec![0x20020, 4, 5],
                vec![0x20040, 6, 7],
                vec![0x20060, 8, 9],
            ]
        );
        assert_eq!(
            calls_to(&log, MATRIX_ASSIGN),
            vec![
                vec![0x20000, 0x10000],
                vec![0x20020, 0x10020],
                vec![0x20040, 0x10040],
                vec![0x20060, 0x10060],
            ]
        );
    }

    #[test]
    fn unequal_flags_copy_only_the_row_they_name() {
        let dims = [(2, 3); 4];
        let first = copy(dims, 1, 0);
        assert_eq!(
            calls_to(&first, MATRIX_ASSIGN),
            vec![vec![0x20000, 0x10000], vec![0x20020, 0x10020]]
        );
        let second = copy(dims, 0, 1);
        assert_eq!(
            calls_to(&second, MATRIX_ASSIGN),
            vec![vec![0x20040, 0x10040], vec![0x20060, 0x10060]]
        );
        // Only the low byte of each flag counts, and unequal bytes of two
        // set flags still copy both rows.
        let both = copy(dims, 2, 1);
        assert_eq!(calls_to(&both, MATRIX_ASSIGN).len(), 4);
        let masked = copy(dims, 0x100, 0);
        assert_eq!(calls_to(&masked, MATRIX_ASSIGN).len(), 4);
    }

    #[test]
    fn a_source_matrix_without_both_dimensions_empties_the_destination() {
        let log = copy([(2, 0), (0, 5), (0, 0), (1, 1)], 0, 0);
        assert_eq!(
            calls_to(&log, MATRIX_RESIZE),
            vec![
                vec![0x20000, 0, 0],
                vec![0x20020, 0, 0],
                vec![0x20040, 0, 0],
                vec![0x20060, 1, 1],
            ]
        );
        assert_eq!(calls_to(&log, MATRIX_ASSIGN), vec![vec![0x20060, 0x10060]]);
    }

    #[test]
    fn fn_00658bb0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0065_8d30, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0065_8bb0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0065_8d30)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_5870);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_5870,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod remove_tests {
    use super::*;

    const DELETE: u32 = 0x0ec0_0000;
    const VTABLE: u32 = 0x0ec1_0000;

    /// A list of three items holding objects with count 1, and the engine.
    fn list() -> (Engine, u32, Vec<u32>, Vec<u32>) {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        // The removal hands the item's element over to the temporary.
        e.register(REMOVE_POSITION_00B9EFA0, |e, a| {
            let item = e.mem.u32(a[2]);
            let element = e.mem.u32(item + 8);
            e.mem.set_u32(a[1], element);
            a[1].into_ret()
        });
        let objects: Vec<u32> = (0..3)
            .map(|_| {
                let object = e.mem.alloc(8);
                e.mem.set_u32(object, VTABLE);
                e.mem.set_u32(object + 4, 1);
                object
            })
            .collect();
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(0xc)).collect();
        for index in 0..3 {
            let next = if index < 2 { items[index + 1] } else { 0 };
            e.mem.set_u32(items[index], next);
            e.mem.set_u32(items[index] + 8, objects[index]);
        }
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, items[0]);
        e.call_log = Some(vec![]);
        (e, this, items, objects)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn remove_takes_the_found_item_out_and_answers_its_element() {
        let (mut e, this, _, objects) = list();
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, objects[1]);
        let result = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00b9_fcc0, &args![this, result, element]).u32(),
            result
        );
        assert_eq!(e.mem.u32(result), objects[1]);
        let removed = calls(&e, REMOVE_POSITION_00B9EFA0);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], this);
        // The result holds one more reference, the temporary's is dropped again.
        assert_eq!(e.mem.u32(objects[1] + 4), 1);
        assert!(calls(&e, DELETE).is_empty());
    }

    #[test]
    fn remove_answers_the_element_asked_for_when_it_is_not_in_the_list() {
        let (mut e, this, _, _) = list();
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, VTABLE);
        e.mem.set_u32(other + 4, 5);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, other);
        let result = e.mem.alloc(4);
        e.call(0x00b9_fcc0, &args![this, result, element]);
        assert_eq!(e.mem.u32(result), other);
        assert_eq!(e.mem.u32(other + 4), 6);
        assert!(calls(&e, REMOVE_POSITION_00B9EFA0).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        // An empty list behaves the same; a null element gives a null result.
        let empty = e.mem.alloc(0xc);
        e.mem.set_u32(element, 0);
        e.call(0x00b9_fcc0, &args![empty, result, element]);
        assert_eq!(e.mem.u32(result), 0);
    }
}

#[cfg(test)]
mod remove_pos_tests {
    use super::*;

    const DELETE: u32 = 0x0e20_0004;
    const VTABLE: u32 = 0x0e21_0000;

    /// A list of three items whose elements are reference-counted objects (count 1), a result
    /// word and a position word.
    fn engine() -> (Engine, u32, [u32; 3], [u32; 3], u32, u32) {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        for target in [
            REMOVE_HEAD_00B9E7C0,
            REMOVE_TAIL_00B9E850,
            FREE_ITEM_00E6E830,
            DELETE,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        let list = e.mem.alloc(0xc);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        let elements = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        for i in 0..3 {
            let next = if i < 2 { items[i + 1] } else { 0 };
            let previous = if i > 0 { items[i - 1] } else { 0 };
            e.mem.set_u32(items[i], next);
            e.mem.set_u32(items[i] + 4, previous);
            e.mem.set_u32(items[i] + 8, elements[i]);
            e.mem.set_u32(elements[i], VTABLE);
            e.mem.set_u32(elements[i] + 4, 1);
        }
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[2]);
        e.mem.set_u32(list + 8, 3);
        let result = e.mem.alloc(4);
        let position = e.mem.alloc(4);
        (e, list, items, elements, result, position)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn remove_pos_unlinks_a_middle_item_and_returns_its_element() {
        let (mut e, list, items, elements, result, position) = engine();
        e.mem.set_u32(position, items[1]);
        let answer = e.call(0x00b9_efa0, &args![list, result, position]).u32();
        assert_eq!(answer, result);
        assert_eq!(e.mem.u32(result), elements[1]);
        // Held by the result: one more than before, the temporary's hold gone again.
        assert_eq!(e.mem.u32(elements[1] + 4), 2);
        assert_eq!(e.mem.u32(position), items[2]);
        assert_eq!(e.mem.u32(items[0]), items[2]);
        assert_eq!(e.mem.u32(items[2] + 4), items[0]);
        assert_eq!(e.mem.u32(list + 8), 2);
        assert_eq!(calls(&e, FREE_ITEM_00E6E830), vec![vec![list, items[1]]]);
        assert!(calls(&e, DELETE).is_empty());
    }

    #[test]
    fn remove_pos_hands_the_head_and_the_tail_to_the_helpers() {
        let (mut e, list, items, _, result, position) = engine();
        e.mem.set_u32(position, items[0]);
        e.call(0x00b9_efa0, &args![list, result, position]);
        assert_eq!(e.mem.u32(position), items[1]);
        assert_eq!(calls(&e, REMOVE_HEAD_00B9E7C0), vec![vec![list, result]]);
        e.mem.set_u32(position, items[2]);
        e.call(0x00b9_efa0, &args![list, result, position]);
        assert_eq!(e.mem.u32(position), 0);
        assert_eq!(calls(&e, REMOVE_TAIL_00B9E850), vec![vec![list, result]]);
        assert_eq!(e.mem.u32(list + 8), 3);
        assert!(calls(&e, FREE_ITEM_00E6E830).is_empty());
    }

    #[test]
    fn remove_pos_with_a_null_element_holds_nothing() {
        let (mut e, list, items, _, result, position) = engine();
        e.mem.set_u32(items[1] + 8, 0);
        e.mem.set_u32(result, 0x55);
        e.mem.set_u32(position, items[1]);
        e.call(0x00b9_efa0, &args![list, result, position]);
        assert_eq!(e.mem.u32(result), 0);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }
}

#[cfg(test)]
mod b049_tests {
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

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    /// An object whose virtual table (at `vtable`) has `slots` entries, slot `i` at
    /// `vtable + 0x1000 + 4 * i`.
    fn object_with_vtable(e: &mut Engine, vtable: u32, slots: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..slots).map(|i| vtable + 0x1000 + 4 * i).collect();
        e.put_vtable(vtable, &targets);
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// An engine with the face-gen lock, the globals' pages, the `NiPointer` helpers as plain
    /// word operations and an allocation that hands out zeroed blocks.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_5000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        stub(
            &mut e,
            &[
                LOCK_TAKE,
                LOCK_RELEASE,
                POINTER_DESTRUCT,
                SCOPE_OPEN,
                SCOPE_CLOSE,
            ],
        );
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(POINTER_ASSIGN_POINTER, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(ALLOCATE_PLAIN, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e
    }

    /// A manager (0x11a4 bytes) set as the singleton, with a model map at +0x1194.
    fn with_manager(e: &mut Engine, map: u32) -> u32 {
        let manager = e.mem.alloc(0x11a4);
        e.mem.set_u32(manager + 0x1194, map);
        e.mem.set_u32(MANAGER, manager);
        manager
    }

    #[test]
    fn fn_0064fd80_constructs_the_model_map() {
        let mut e = engine();
        stub(&mut e, &[MAP_CONSTRUCT]);
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_fd80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), MODEL_MAP_VTABLE);
        assert_eq!(calls_to(&e, MAP_CONSTRUCT), vec![vec![this + 4, 0x25, 1]]);
        assert_eq!(e.mem.u32(this + 0x18), 0x1000_0000);
        assert_eq!(e.mem.u32(this + 0x1c), 0x1000_0000);
        assert_eq!(e.mem.u8(this + 0x20), 0);
    }

    #[test]
    fn model_map_scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = engine();
        stub(&mut e, &[MAP_CLEAN_UP, 0x0065_8d30, FREE]);
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0064_fde0, &args![this, 0u32]);
        assert!(calls_to(&e, FREE).is_empty());
        e.call(0x0064_fde0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this]]);
    }

    #[test]
    fn fn_0064fe10_cleans_and_destroys_the_map() {
        let mut e = engine();
        stub(&mut e, &[MAP_CLEAN_UP, 0x0065_8d30]);
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0064_fe10, &args![this]);
        assert_eq!(e.mem.u32(this), MODEL_MAP_VTABLE);
        assert_eq!(calls_to(&e, MAP_CLEAN_UP), vec![vec![this + 4]]);
        assert_eq!(e.mem.u32(this + 4), 0x0106_5870);
        assert_eq!(calls_to(&e, 0x0065_8d30), vec![vec![this + 4]]);
    }

    /// A model map whose inner map (+4) hashes every key to bucket 0 of a one-bucket table,
    /// compares keys by value (slot 8) and stores items through slot 0xc.
    fn model_map(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x24);
        e.put_vtable(0x7000, &[0, 0x7104, 0x7108, 0x710c]);
        e.register(0x7104, |_, _| 0u32.into_ret());
        e.register(0x7108, |_, a| (a[1] == a[2]).into_ret());
        e.register(0x710c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(0x006c_63e0, |e, _| e.mem.alloc(12).into_ret());
        let buckets = e.mem.alloc(4);
        e.mem.set_u32(this + 4, 0x7000);
        e.mem.set_u32(this + 8, 1);
        e.mem.set_u32(this + 0xc, buckets);
        this
    }

    #[test]
    fn fn_0064ffd0_removes_or_stores_an_entry() {
        let mut e = engine();
        stub(
            &mut e,
            &[MAP_REMOVE_AT, ENTRY_BASE_CONSTRUCT, MODEL_MAP_TOUCH],
        );
        answer(&mut e, FRAME_COUNTER, 77);
        let this = model_map(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0064_ffd0, &args![this, 0x55u32, 0u32]);
        assert_eq!(calls_to(&e, MAP_REMOVE_AT), vec![vec![this + 4, 0x55]]);
        assert_eq!(e.mem.u32(FACE_GEN_LOCK_DEPTH), 0);

        e.call_log = Some(vec![]);
        e.call(0x0064_ffd0, &args![this, 0x55u32, 0x9999u32]);
        let item = e.mem.u32(e.mem.u32(this + 0xc));
        assert_ne!(item, 0);
        assert_eq!(e.mem.u32(item + 4), 0x55);
        let entry = e.mem.u32(item + 8);
        assert_eq!(e.mem.u32(entry), MODEL_MAP_ENTRY_VTABLE);
        assert_eq!(e.mem.u32(entry + 8), 0x9999);
        assert_eq!(e.mem.u32(entry + 0xc), 77);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(calls_to(&e, MODEL_MAP_TOUCH), vec![vec![this, entry]]);
        assert_eq!(e.mem.u32(FACE_GEN_LOCK_DEPTH), 0);
    }

    #[test]
    fn fn_00650100_constructs_an_entry() {
        let mut e = engine();
        stub(&mut e, &[ENTRY_BASE_CONSTRUCT]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_0100, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, ENTRY_BASE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), MODEL_MAP_ENTRY_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn entry_scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = engine();
        stub(&mut e, &[ENTRY_BASE_DESTRUCT, FREE_SIZED]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_0170, &args![this, 2u32]).u32(), this);
        assert!(calls_to(&e, FREE_SIZED).is_empty());
        e.call(0x0065_0170, &args![this, 1u32]);
        assert_eq!(calls_to(&e, FREE_SIZED), vec![vec![this, 0x10]]);
    }

    #[test]
    fn fn_006501a0_destroys_the_model_then_the_base() {
        let mut e = engine();
        stub(&mut e, &[ENTRY_BASE_DESTRUCT]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0065_01a0, &args![this]);
        let log = e.call_log.as_ref().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
        assert_eq!(order, vec![POINTER_DESTRUCT, ENTRY_BASE_DESTRUCT]);
        assert_eq!(log[1].1, vec![this + 8]);
    }

    /// An entry holding `model`, as a lookup hands it out.
    fn entry_with(e: &mut Engine, model: u32) -> u32 {
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 8, model);
        entry
    }

    #[test]
    fn fn_00650200_looks_up_and_touches() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                MODEL_MAP_TOUCH,
                MODEL_ATTACH_0065BF80,
                MODEL_ATTACH_0065C2C0,
            ],
        );
        answer(&mut e, FRAME_COUNTER, 9);
        let entry = entry_with(&mut e, 0x4440);
        e.register_double(MAP_LOOKUP, move |e, a| {
            if a[1] == 0x55 {
                e.mem.set_u32(a[2], entry);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let this = e.mem.alloc(0x24);
        let model = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_0200, &args![this, 0x55u32, model]).bool());
        assert_eq!(e.mem.u32(model), 0x4440);
        assert_eq!(e.mem.u32(entry + 0xc), 9);
        assert_eq!(calls_to(&e, MODEL_MAP_TOUCH), vec![vec![this, entry]]);
        assert_eq!(calls_to(&e, MODEL_ATTACH_0065BF80), vec![vec![0x4440]]);
        assert_eq!(calls_to(&e, MODEL_ATTACH_0065C2C0), vec![vec![0x4440]]);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert!(!e.call(0x0065_0200, &args![this, 0x66u32, model]).bool());
        assert_eq!(e.mem.u32(FACE_GEN_LOCK_DEPTH), 0);
    }

    /// The name source doubles: the first name `0x100`, the second `0x200`; only the second
    /// is in the map.
    fn named_engine() -> (Engine, u32) {
        let mut e = engine();
        stub(
            &mut e,
            &[
                MODEL_MAP_TOUCH,
                MODEL_ATTACH_0065BF80,
                MODEL_ATTACH_0065C2C0,
            ],
        );
        answer(&mut e, FRAME_COUNTER, 9);
        answer(&mut e, NAME_FIRST, 0x100);
        answer(&mut e, NAME_SECOND, 0x200);
        answer(&mut e, STRING_LENGTH, 3);
        let entry = entry_with(&mut e, 0x4440);
        e.register_double(MAP_LOOKUP, move |e, a| {
            if a[1] == 0x200 {
                e.mem.set_u32(a[2], entry);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        (e, entry)
    }

    #[test]
    fn fn_00650480_falls_back_to_the_second_name() {
        let (mut e, entry) = named_engine();
        let this = e.mem.alloc(0x24);
        assert!(!e.call(0x0065_0480, &args![this, 0u32]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_0480, &args![this, 0x3000u32]).bool());
        let keys: Vec<u32> = calls_to(&e, MAP_LOOKUP).iter().map(|c| c[1]).collect();
        assert_eq!(keys, vec![0x100, 0x200]);
        assert_eq!(calls_to(&e, MODEL_MAP_TOUCH), vec![vec![this, entry]]);
        assert_eq!(calls_to(&e, MODEL_ATTACH_0065BF80), vec![vec![0x4440]]);
        assert_eq!(e.mem.u8(this + 0x20), 1);
    }

    #[test]
    fn fn_00650630_tries_the_second_name_first() {
        let (mut e, _) = named_engine();
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_0630, &args![this, 0x3000u32]).bool());
        let keys: Vec<u32> = calls_to(&e, MAP_LOOKUP).iter().map(|c| c[1]).collect();
        assert_eq!(keys, vec![0x200]);
        assert_eq!(calls_to(&e, MODEL_ATTACH_0065C2C0), vec![vec![0x4440]]);
        // An empty name answers false without a look-up.
        answer(&mut e, STRING_LENGTH, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0065_0630, &args![this, 0x3000u32]).bool());
        assert!(calls_to(&e, MAP_LOOKUP).is_empty());
    }

    #[test]
    fn fn_006508e0_sets_a_word_and_touches() {
        let mut e = engine();
        stub(&mut e, &[MODEL_MAP_TOUCH]);
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0065_08e0, &args![this, 1u32, 0x77u32]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x77);
        assert_eq!(calls_to(&e, MODEL_MAP_TOUCH), vec![vec![this, 0]]);
    }

    /// The constructor's callees as doubles.
    fn manager_engine() -> Engine {
        let mut e = engine();
        stub(
            &mut e,
            &[
                VECTOR_CONSTRUCT,
                VECTOR_DESTRUCT,
                0x0068_15c0,
                0x0065_d3a0,
                STD_STRING_CONSTRUCT,
                STD_STRING_DESTRUCT,
                PRINT_005B5E40,
                MATRIX_RESIZE,
                OBJECT_SET_004393C0,
                OBJECT_SET_004393E0,
                OBJECT_SET_0094DB80,
                FILL_00403D30,
                PIXEL_DATA_STEP_004F31B0,
                FIXED_STRING_DESTRUCT,
                FIXED_STRING_TEXT,
                ELEMENT_CONSTRUCT_00449610,
            ],
        );
        answer(&mut e, FAN_READ_CTL_FILE, 0);
        answer(&mut e, OBJECT_CONSTRUCT_004391C0, 0x3910);
        e.register(PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(PIXEL_DATA_PIXELS, |e, _| e.mem.alloc(0x1000).into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        let mut texture = 0x5000u32;
        e.register_double(SOURCE_TEXTURE_CREATE, move |_, _| {
            texture += 0x1000;
            texture.into_ret()
        });
        e
    }

    #[test]
    fn constructor_builds_the_parts_and_textures() {
        let mut e = manager_engine();
        let this = e.mem.alloc(0x11a4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_0ea0, &args![this]).u32(), this);
        // Both control file reads failed: the message.
        assert_eq!(calls_to(&e, FAN_READ_CTL_FILE).len(), 2);
        assert_eq!(calls_to(&e, PRINT_005B5E40), vec![vec![CTL_FAILED_MESSAGE]]);
        // No statistics: every coordinate matrix sized 0 by 0.
        let resizes = calls_to(&e, MATRIX_RESIZE);
        assert_eq!(resizes.len(), 4);
        assert_eq!(resizes[3], vec![this + 8 + 0x40 + 0x20, 0, 0]);
        assert_eq!(e.mem.u32(this + 0x1198), 0x3910);
        assert_eq!(calls_to(&e, OBJECT_SET_0094DB80), vec![vec![0x3910, 0x64]]);
        assert_eq!(e.mem.u32(this + 0x119c), 0x6000);
        assert_eq!(e.mem.u32(this + 0x11a0), 0x7000);
        let fills = calls_to(&e, FILL_00403D30);
        assert_eq!(&fills[0][1..], &[0x80, 0x1000]);
        let pixels = calls_to(&e, PIXEL_DATA_PIXELS);
        assert_eq!(pixels.len(), 2);
        let creates = calls_to(&e, SOURCE_TEXTURE_CREATE);
        assert_eq!(creates[1][2], TEXTURE_PREFERENCES_011A9598);
        assert_eq!(e.mem.u8(MANAGER_BUILT_011D59E0), 1);
        assert_eq!(e.mem.u8(creates[1][0] + 0x70), 1);
        assert_eq!(
            calls_to(&e, SCOPE_OPEN)[0][1..],
            [0x37, 1, SOURCE_FILE_NAME, 0x2b4]
        );
    }

    #[test]
    fn constructor_reads_the_control_file_once_when_it_works() {
        let mut e = manager_engine();
        answer(&mut e, FAN_READ_CTL_FILE, 1);
        let this = e.mem.alloc(0x11a4);
        e.call_log = Some(vec![]);
        e.call(0x0065_0ea0, &args![this]);
        assert_eq!(calls_to(&e, FAN_READ_CTL_FILE).len(), 1);
        assert!(calls_to(&e, PRINT_005B5E40).is_empty());
    }

    #[test]
    fn fn_00651630_and_00651650_wrap_the_element() {
        let mut e = engine();
        stub(
            &mut e,
            &[ELEMENT_CONSTRUCT_00449610, ELEMENT_DESTRUCT_00449680],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_1630, &args![0x40u32]).u32(), 0x40);
        e.call(0x0065_1650, &args![0x40u32]);
        assert_eq!(calls_to(&e, ELEMENT_CONSTRUCT_00449610), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, ELEMENT_DESTRUCT_00449680), vec![vec![0x40]]);
    }

    #[test]
    fn fn_00651650_destroys_the_element() {
        let mut e = engine();
        stub(&mut e, &[ELEMENT_DESTRUCT_00449680]);
        e.call_log = Some(vec![]);
        e.call(0x0065_1650, &args![0x44u32]);
        assert_eq!(calls_to(&e, ELEMENT_DESTRUCT_00449680), vec![vec![0x44]]);
    }

    #[test]
    fn fn_00651670_and_006516a0_build_and_destroy_pairs() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_CONSTRUCT, VECTOR_DESTRUCT]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_1670, &args![0x40u32]).u32(), 0x40);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![
                0x40,
                0x20,
                2,
                ELEMENT_CONSTRUCT_00449610,
                ELEMENT_DESTRUCT_00449680
            ]]
        );
    }

    #[test]
    fn fn_006516a0_destroys_the_pair() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_DESTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x0065_16a0, &args![0x40u32]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![0x40, 0x20, 2, ELEMENT_DESTRUCT_00449680]]
        );
    }

    #[test]
    fn fn_006516c0_sets_the_byte() {
        let mut e = engine();
        let this = e.mem.alloc(0x74);
        e.call(0x0065_16c0, &args![this, 0x1ffu32]);
        assert_eq!(e.mem.u8(this + 0x70), 0xff);
    }

    #[test]
    fn fn_006516e0_builds_matrices_and_vectors() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_CONSTRUCT, 0x0068_15c0, 0x0065_d3a0]);
        let this = e.mem.alloc(0xb8);
        e.mem.write(this, &[0xcd; 0xb8]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_16e0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, VECTOR_CONSTRUCT)[0][..3], [this + 8, 0x20, 4]);
        let allocators: Vec<u32> = calls_to(&e, 0x0065_d3a0).iter().map(|c| c[0]).collect();
        assert_eq!(allocators, vec![this + 0x88, this + 0xa0]);
        assert_eq!(e.mem.u32(this + 0x88 + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0xa0 + 0x14), 0);
    }

    #[test]
    fn fn_00651760_destroys_the_first_part() {
        let mut e = engine();
        stub(
            &mut e,
            &[0x0065_f060, 0x0065_cfb0, VECTOR_DESTRUCT, VECTOR_DEALLOCATE],
        );
        let this = e.mem.alloc(0xb8);
        e.call_log = Some(vec![]);
        e.call(0x0065_1760, &args![this]);
        assert_eq!(calls_to(&e, 0x0065_f060), vec![vec![this + 0xa0]]);
        assert_eq!(calls_to(&e, 0x0065_cfb0), vec![vec![this + 0x88]]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![this + 8, 0x20, 4, ELEMENT_DESTRUCT_00449680]]
        );
        assert!(calls_to(&e, VECTOR_DEALLOCATE).is_empty());
    }

    #[test]
    fn fn_006517e0_and_00651810_build_and_destroy_statistics() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_CONSTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x0065_17e0, &args![0x40u32]);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![0x40, 0x18, 4, 0x0065_7960, 0x0065_79d0]]
        );
    }

    #[test]
    fn fn_00651810_destroys_the_statistics() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_DESTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x0065_1810, &args![0x40u32]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![0x40, 0x18, 4, 0x0065_79d0]]
        );
    }

    #[test]
    fn fn_00651830_destroys_the_arrays_last_first() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_DESTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x0065_1830, &args![0x1000u32]);
        let arrays: Vec<u32> = calls_to(&e, VECTOR_DESTRUCT).iter().map(|c| c[0]).collect();
        assert_eq!(arrays, vec![0x1d4c, 0x1644, 0x1324, 0x1004]);
    }

    #[test]
    fn fn_00651450_builds_four_arrays() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_CONSTRUCT]);
        let this = e.mem.alloc(0x1080);
        e.mem.set_u8(this, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_1450, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this), 0);
        let arrays = calls_to(&e, VECTOR_CONSTRUCT);
        assert_eq!(arrays.len(), 4);
        assert_eq!(arrays[0], vec![this + 4, 0xa0, 5, 0x0065_1520, 0x0065_15b0]);
        assert_eq!(arrays[3][..3], [this + 0xd4c, 0x20, 0x14]);
    }

    #[test]
    fn fn_00651520_builds_five_elements() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_CONSTRUCT, ELEMENT_CONSTRUCT_00449610]);
        e.call_log = Some(vec![]);
        e.call(0x0065_1520, &args![0x1000u32]);
        let pairs: Vec<u32> = calls_to(&e, VECTOR_CONSTRUCT)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(pairs, vec![0x1000, 0x1060]);
        assert_eq!(calls_to(&e, ELEMENT_CONSTRUCT_00449610), vec![vec![0x1040]]);
    }

    #[test]
    fn fn_006515b0_destroys_five_elements() {
        let mut e = engine();
        stub(&mut e, &[VECTOR_DESTRUCT, ELEMENT_DESTRUCT_00449680]);
        e.call_log = Some(vec![]);
        e.call(0x0065_15b0, &args![0x1000u32]);
        let pairs: Vec<u32> = calls_to(&e, VECTOR_DESTRUCT).iter().map(|c| c[0]).collect();
        assert_eq!(pairs, vec![0x1060, 0x1000]);
        assert_eq!(calls_to(&e, ELEMENT_DESTRUCT_00449680), vec![vec![0x1040]]);
    }

    fn destructor_engine() -> Engine {
        let mut e = engine();
        stub(
            &mut e,
            &[0x0065_0a30, VECTOR_DESTRUCT, 0x0065_f060, 0x0065_cfb0, FREE],
        );
        e
    }

    #[test]
    fn fn_006518e0_destroys_the_manager() {
        let mut e = destructor_engine();
        let map = object_with_vtable(&mut e, 0x7000, 1, 0x24);
        e.register(0x8000, |_, _| Ret::default());
        let this = e.mem.alloc(0x11a4);
        e.mem.set_u32(this + 0x1194, map);
        e.call_log = Some(vec![]);
        e.call(0x0065_18e0, &args![this]);
        assert_eq!(calls_to(&e, 0x0065_0a30), vec![vec![map, 0]]);
        assert_eq!(calls_to(&e, 0x8000), vec![vec![map, 1]]);
        assert_eq!(e.mem.u32(this + 0x1194), 0);
        let pointers: Vec<u32> = calls_to(&e, POINTER_DESTRUCT)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(pointers, vec![this + 0x11a0, this + 0x119c, this + 0x1198]);
        assert_eq!(calls_to(&e, 0x0065_f060), vec![vec![this + 0xa0]]);
    }

    #[test]
    fn initialize_system_makes_the_manager_once() {
        let mut e = engine();
        answer(&mut e, ALLOCATE_PLAIN, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_19f0, &args![]);
        assert_eq!(calls_to(&e, ALLOCATE_PLAIN), vec![vec![0x11a4]]);
        assert_eq!(e.mem.u32(MANAGER), 0);
        assert_eq!(
            calls_to(&e, SCOPE_OPEN)[0][1..],
            [0x37, 1, SOURCE_FILE_NAME, 0x316]
        );
        with_manager(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_19f0, &args![]);
        assert!(calls_to(&e, ALLOCATE_PLAIN).is_empty());
        assert_eq!(calls_to(&e, SCOPE_CLOSE).len(), 1);
    }

    #[test]
    fn fn_00651ab0_deletes_the_manager() {
        let mut e = destructor_engine();
        e.call_log = Some(vec![]);
        e.call(0x0065_1ab0, &args![]);
        assert!(calls_to(&e, FREE).is_empty());
        let manager = with_manager(&mut e, 0);
        e.call(0x0065_1ab0, &args![]);
        assert_eq!(calls_to(&e, FREE), vec![vec![manager]]);
        assert_eq!(e.mem.u32(MANAGER), 0);
    }

    #[test]
    fn fn_00651b00_frees_only_with_bit_0() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x11a4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_1b00, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(calls_to(&e, VECTOR_DESTRUCT).len(), 6);
    }

    #[test]
    fn get_instance_initializes_when_needed() {
        let mut e = engine();
        answer(&mut e, ALLOCATE_PLAIN, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_1b30, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, ALLOCATE_PLAIN).len(), 1);
        let manager = with_manager(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_1b30, &args![]).u32(), manager);
        assert!(calls_to(&e, ALLOCATE_PLAIN).is_empty());
    }

    #[test]
    fn fn_00651b50_finds_or_makes_a_model() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                MODEL_MAP_TOUCH,
                MODEL_ATTACH_0065BF80,
                MODEL_ATTACH_0065C2C0,
                0x0065_a760,
            ],
        );
        stub(&mut e, &[ENTRY_BASE_CONSTRUCT]);
        answer(&mut e, FRAME_COUNTER, 1);
        answer(&mut e, NAME_SECOND, 0);
        let model = e.mem.alloc(0x10);
        let entry = entry_with(&mut e, model);
        e.register_double(MAP_LOOKUP, move |e, a| {
            if a[1] == 0x55 {
                e.mem.set_u32(a[2], entry);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let map = e.mem.alloc(0x24);
        with_manager(&mut e, map);
        // No keys: null.
        assert_eq!(
            e.call(
                0x0065_1b50,
                &args![0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32]
            )
            .u32(),
            0
        );
        // Found under the first key; the second key goes to `0065a760`.
        e.call_log = Some(vec![]);
        let found = e
            .call(
                0x0065_1b50,
                &args![0x55u32, 0u32, 0u32, 0x66u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(found, model);
        assert_eq!(calls_to(&e, 0x0065_a760), vec![vec![model, 0x66]]);
        // Not found, made but not built: dropped, null.
        answer(&mut e, 0x0065_9c70, 0);
        e.call_log = Some(vec![]);
        let made = e
            .call(
                0x0065_1b50,
                &args![0x77u32, 2u32, 3u32, 0u32, 1u32, 6u32, 0x107u32],
            )
            .u32();
        assert_eq!(made, 0);
        let build = &calls_to(&e, 0x0065_9c70)[0];
        assert_eq!(&build[1..], &[0x77, 2, 3, 0, 6, 7]);
        assert_eq!(e.mem.u32(build[0]), 0x0106_5af4);
        // Not found and not made: null.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x0065_1b50,
                &args![0u32, 0u32, 0u32, 0x99u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            0
        );
        assert!(calls_to(&e, 0x0065_9c70).is_empty());
    }

    #[test]
    fn fn_00651b50_makes_the_model_map_first() {
        let mut e = engine();
        stub(&mut e, &[MAP_CONSTRUCT, MODEL_MAP_TOUCH]);
        answer(&mut e, 0x0045_03f0, 0x31);
        answer(&mut e, MAP_LOOKUP, 0);
        let manager = with_manager(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x0065_1b50,
            &args![0x55u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        let map = e.mem.u32(manager + 0x1194);
        assert_ne!(map, 0);
        assert_eq!(e.mem.u32(map), MODEL_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 0x18), 0x31);
        assert_eq!(e.mem.u32(map + 0x1c), 0x31);
        let sources: Vec<u32> = calls_to(&e, 0x0045_03f0).iter().map(|c| c[0]).collect();
        assert_eq!(sources, vec![0x011d_59f4, 0x011d_5afc]);
    }

    #[test]
    fn fn_00651e30_updates_the_map_when_there_is_one() {
        let mut e = engine();
        stub(&mut e, &[0x0065_03b0]);
        e.call_log = Some(vec![]);
        e.call(0x0065_1e30, &args![]);
        assert!(calls_to(&e, 0x0065_03b0).is_empty());
        with_manager(&mut e, 0x2400);
        e.call(0x0065_1e30, &args![]);
        assert_eq!(calls_to(&e, 0x0065_03b0), vec![vec![0x2400]]);
    }

    #[test]
    fn fn_00651e70_walks_every_model() {
        let mut e = engine();
        stub(&mut e, &[0x0065_c3a0]);
        answer(&mut e, 0x004b_9ba0, 1);
        let entry = entry_with(&mut e, 0x4440);
        e.register_double(0x0055_9120, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[3], entry);
            Ret::default()
        });
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0065_1e70, &args![this]);
        assert_eq!(calls_to(&e, 0x0065_c3a0), vec![vec![0x4440]]);
        assert_eq!(calls_to(&e, 0x0055_9120)[0][0], this + 4);
        assert_eq!(e.mem.u32(FACE_GEN_LOCK_DEPTH), 0);
        assert_eq!(calls_to(&e, LOCK_TAKE).len(), 1);
    }

    #[test]
    fn fn_00651f40_walks_only_with_a_map() {
        let mut e = engine();
        answer(&mut e, 0x004b_9ba0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_1f40, &args![]);
        assert!(calls_to(&e, 0x004b_9ba0).is_empty());
        with_manager(&mut e, 0x2400);
        e.call(0x0065_1f40, &args![]);
        assert_eq!(calls_to(&e, 0x004b_9ba0), vec![vec![0x2404]]);
    }

    #[test]
    fn fn_00651f70_touches_through_the_map() {
        let (mut e, _) = named_engine();
        assert!(!e.call(0x0065_1f70, &args![0x3000u32]).bool());
        let map = e.mem.alloc(0x24);
        with_manager(&mut e, map);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_1f70, &args![0x3000u32]).bool());
        assert_eq!(e.mem.u8(map + 0x20), 1);
        assert_eq!(
            calls_to(&e, SCOPE_OPEN)[0][1..],
            [0x38, 1, SOURCE_FILE_NAME, 0x44c]
        );
    }

    #[test]
    fn fn_00652040_touches_through_the_map() {
        let (mut e, _) = named_engine();
        let map = e.mem.alloc(0x24);
        with_manager(&mut e, map);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_2040, &args![0x3000u32]).bool());
        assert_eq!(calls_to(&e, MODEL_ATTACH_0065C2C0).len(), 1);
        assert_eq!(
            calls_to(&e, SCOPE_OPEN)[0][1..],
            [0x39, 1, SOURCE_FILE_NAME, 0x45a]
        );
    }

    #[test]
    fn get_model_cache_reads_the_map() {
        let mut e = engine();
        assert_eq!(e.call(0x0065_2110, &args![]).u32(), 0);
        with_manager(&mut e, 0x2400);
        assert_eq!(e.call(0x0065_2110, &args![]).u32(), 0x2400);
    }

    #[test]
    fn fn_00652140_takes_the_lock() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0065_2140, &args![]);
        assert_eq!(calls_to(&e, LOCK_TAKE), vec![vec![FACE_GEN_LOCK, 0]]);
        assert_eq!(e.mem.u32(FACE_GEN_LOCK_DEPTH), 1);
    }

    #[test]
    fn fn_00652160_counts_only_a_taken_lock() {
        let mut e = engine();
        answer(&mut e, LOCK_TRY, 0);
        assert_eq!(e.call(0x0065_2160, &args![]).u8(), 0);
        assert_eq!(e.mem.u32(FACE_GEN_LOCK_DEPTH), 0);
        answer(&mut e, LOCK_TRY, 1);
        assert_eq!(e.call(0x0065_2160, &args![]).u8(), 1);
        assert_eq!(e.mem.u32(FACE_GEN_LOCK_DEPTH), 1);
    }

    #[test]
    fn fn_00652190_releases_the_lock() {
        let mut e = engine();
        e.mem.set_u32(FACE_GEN_LOCK_DEPTH, 2);
        e.call_log = Some(vec![]);
        e.call(0x0065_2190, &args![]);
        assert_eq!(calls_to(&e, LOCK_RELEASE), vec![vec![FACE_GEN_LOCK]]);
        assert_eq!(e.mem.u32(FACE_GEN_LOCK_DEPTH), 1);
    }

    #[test]
    fn fn_006521b0_counts_the_statistics() {
        let mut e = engine();
        let manager = with_manager(&mut e, 0);
        let vector = manager + 0xb8 + 0x30 + 0x18;
        e.mem.set_u32(vector + 0xc, 0x1000);
        e.mem.set_u32(vector + 0x10, 0x1000 + 3 * 0x3c);
        assert_eq!(e.call(0x0065_21b0, &args![1u32, 1u32]).i32(), 2);
    }

    #[test]
    fn init_face_gen_coord_sizes_three_matrices() {
        let mut e = engine();
        stub(&mut e, &[MATRIX_RESIZE]);
        e.call_log = Some(vec![]);
        e.call(0x0065_21f0, &args![0u32]);
        assert!(calls_to(&e, MATRIX_RESIZE).is_empty());
        e.call(0x0065_21f0, &args![0x1000u32]);
        assert_eq!(
            calls_to(&e, MATRIX_RESIZE),
            vec![
                vec![0x1000, 0x32, 1],
                vec![0x1020, 0x1e, 1],
                vec![0x1040, 0x32, 1]
            ]
        );
    }

    /// Matrices as the tests model them: data at +0xc, rows +0x18, columns +0x1c; the
    /// helpers `00659000` (construct), `0060b370`/`00629ab0` (element), `00658f30` (copy).
    fn matrix_engine() -> Engine {
        let mut e = engine();
        stub(&mut e, &[ELEMENT_DESTRUCT_00449680, 0x00aa_fb90]);
        e.register(0x0065_9000, |e, a| {
            let data = e.mem.alloc((a[1] * a[2] * 4).max(4));
            e.mem.set_u32(a[0] + 0xc, data);
            e.mem.set_u32(a[0] + 0x18, a[1]);
            e.mem.set_u32(a[0] + 0x1c, a[2]);
            Ret::default()
        });
        e.register(0x0060_b370, |e, a| {
            let data = e.mem.u32(a[0] + 0xc);
            let columns = e.mem.u32(a[0] + 0x1c);
            (data + a[2] * columns * 4).into_ret()
        });
        e.register(0x0062_9ab0, |_, a| (a[0] + a[1] * 4).into_ret());
        e.register(0x0065_8f30, |e, a| {
            let mut bytes = [0u8; 0x20];
            e.mem.read(a[1], &mut bytes);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e
    }

    fn matrix(e: &mut Engine, rows: u32, columns: u32, values: &[f32]) -> u32 {
        let m = e.mem.alloc(0x20);
        let data = e.mem.alloc((rows * columns * 4).max(4));
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(data + 4 * i as u32, *v);
        }
        e.mem.set_u32(m + 0xc, data);
        e.mem.set_u32(m + 0x18, rows);
        e.mem.set_u32(m + 0x1c, columns);
        m
    }

    #[test]
    fn fn_00658360_multiplies() {
        let mut e = matrix_engine();
        let a = matrix(&mut e, 1, 2, &[2.0, 3.0]);
        let b = matrix(&mut e, 2, 1, &[4.0, 5.0]);
        let result = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_8360, &args![a, result, b]).u32(), result);
        let data = e.mem.u32(result + 0xc);
        assert_eq!(e.mem.f32(data), 23.0);
        assert!(calls_to(&e, 0x00aa_fb90).is_empty());
        // Mismatched sizes assert first.
        let c = matrix(&mut e, 3, 1, &[1.0, 1.0, 1.0]);
        e.call(0x0065_8360, &args![a, result, c]);
        assert_eq!(calls_to(&e, 0x00aa_fb90), vec![vec![0x0106_5830, 0x113]]);
    }

    #[test]
    fn fn_00652230_answers_the_first_product_element() {
        let mut e = matrix_engine();
        let manager = with_manager(&mut e, 0);
        let entries = e.mem.alloc(0x3c);
        let vector = manager + 0xb8;
        e.mem.set_u32(vector + 0xc, entries);
        e.mem.set_u32(vector + 0x10, entries + 0x3c);
        let data = e.mem.alloc(4);
        e.mem.set_f32(data, 2.0);
        e.mem.set_u32(entries + 0xc, data);
        e.mem.set_u32(entries + 0x18, 1);
        e.mem.set_u32(entries + 0x1c, 1);
        let block = e.mem.alloc(0x80);
        let values = e.mem.alloc(4);
        e.mem.set_f32(values, 3.0);
        e.mem.set_u32(block + 0xc, values);
        e.mem.set_u32(block + 0x18, 1);
        e.mem.set_u32(block + 0x1c, 1);
        assert_eq!(
            e.call(0x0065_2230, &args![block, 0u32, 0u32, 0u32]).f32(),
            6.0
        );
        assert_eq!(
            e.call(0x0065_2230, &args![block, 0u32, 0u32, 1u32]).f32(),
            0.0
        );
        assert_eq!(
            e.call(0x0065_2230, &args![0u32, 0u32, 0u32, 0u32]).f32(),
            0.0
        );
    }

    #[test]
    fn fn_00652440_passes_the_result_on() {
        let mut e = engine();
        let manager = with_manager(&mut e, 0);
        e.register(0x00aa_bdb0, |_, _| Ret {
            st0: 1.25,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let value = e.call(0x0065_2440, &args![1u32, 2u32, 3u32]).f64();
        assert_eq!(value, 1.25);
        assert_eq!(
            calls_to(&e, 0x00aa_bdb0),
            vec![vec![manager + 0x118, 0, 2, 3, 1]]
        );
    }

    /// `float` vectors: size from `00662820` (data +0xc, end +0x10), the append through
    /// `0060b8a0` (capacity 64).
    fn float_vector_engine() -> Engine {
        let mut e = matrix_engine();
        stub(&mut e, &[INVALID_PARAMETER, 0x0044_a0a0, 0x0065_8650]);
        e.register(FLOAT_VECTOR_SIZE, |e, a| {
            ((e.mem.u32(a[0] + 0x10) - e.mem.u32(a[0] + 0xc)) / 4).into_ret()
        });
        answer(&mut e, 0x0060_b860, 64);
        e.register(0x0060_b8a0, |e, a| {
            let value = e.mem.u32(a[3]);
            e.mem.set_u32(a[1], value);
            (a[1] + 4).into_ret()
        });
        e
    }

    fn float_vector(e: &mut Engine, values: &[f32]) -> u32 {
        let vector = e.mem.alloc(0x20);
        let data = e.mem.alloc(64 * 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(data + 4 * i as u32, *v);
        }
        e.mem.set_u32(vector + 0xc, data);
        e.mem.set_u32(vector + 0x10, data + 4 * values.len() as u32);
        vector
    }

    #[test]
    fn fn_00652af0_sums_and_limits() {
        let mut e = float_vector_engine();
        e.register(MATRIX_FIRST_DIMENSION, |_, _| 1u32.into_ret());
        e.register(MATRIX_SECOND_DIMENSION, |_, _| 2u32.into_ret());
        stub(&mut e, &[MATRIX_RESIZE]);
        e.register(0x0065_88a0, |_, _| Ret {
            st0: 16.0,
            ..Ret::default()
        });
        e.register(0x0040_19b0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).sqrt(),
            ..Ret::default()
        });
        let first = e.mem.alloc(0x80);
        let second = e.mem.alloc(0x80);
        let destination = e.mem.alloc(0x80);
        for block in [first, second, destination] {
            for offset in [0u32, 0x20, 0x40, 0x60] {
                let values = if block == destination {
                    vec![]
                } else {
                    vec![1.0, 2.0]
                };
                let vector = float_vector(&mut e, &values);
                let mut bytes = [0u8; 0x20];
                e.mem.read(vector, &mut bytes);
                e.mem.write(block + offset, &bytes);
            }
        }
        e.call_log = Some(vec![]);
        e.call(
            0x0065_2af0,
            &args![first, second, destination, 0u32, 2.0f32],
        );
        let data = e.mem.u32(destination + 0xc);
        assert_eq!(e.mem.f32(data), 2.0);
        assert_eq!(e.mem.f32(data + 4), 4.0);
        // Length 16 over 1 row: root 4 > 2, so each matrix is scaled by 0.5.
        let scales = calls_to(&e, 0x0065_8650);
        assert_eq!(scales.len(), 4);
        assert_eq!(scales[0][1], 0.5f32.to_bits());
        // With no destination nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x0065_2af0, &args![first, second, 0u32, 0u32, 2.0f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00652af0_copies_a_single_source() {
        let mut e = engine();
        answer(&mut e, MATRIX_FIRST_DIMENSION, 0);
        stub(&mut e, &[MATRIX_RESIZE]);
        e.call_log = Some(vec![]);
        e.call(
            0x0065_2af0,
            &args![0u32, 0x1000u32, 0x2000u32, 0u32, 0.0f32],
        );
        assert_eq!(calls_to(&e, MATRIX_RESIZE)[0], vec![0x2000, 0, 0]);
        assert_eq!(calls_to(&e, MATRIX_FIRST_DIMENSION)[0], vec![0x1000]);
    }

    /// An object for `00607310` whose slot 0x94 answers an inner object whose +8 holds an
    /// `NiPointer` at +0x1c to `0x4440`.
    fn form_engine() -> Engine {
        let mut e = engine();
        let inner = e.mem.alloc(0x10);
        let held = e.mem.alloc(0x20);
        e.mem.set_u32(held + 0x1c, 0x4440);
        e.mem.set_u32(inner + 8, held);
        let object = object_with_vtable(&mut e, 0x7000, 0x30, 4);
        answer(&mut e, 0x7000 + 0x1094, inner);
        e.register_double(OBJECT_OF_00607310, move |_, a| {
            if a[0] == 0x77 {
                object.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e
    }

    #[test]
    fn fn_006530e0_answers_the_slot_result() {
        let mut e = form_engine();
        assert_ne!(e.call(0x0065_30e0, &args![0x77u32]).u32(), 0);
        assert_eq!(e.call(0x0065_30e0, &args![0x78u32]).u32(), 0);
    }

    #[test]
    fn fn_00653120_answers_the_held_object() {
        let mut e = form_engine();
        assert_eq!(e.call(0x0065_3120, &args![0x77u32]).u32(), 0x4440);
        assert_eq!(e.call(0x0065_3120, &args![0x78u32]).u32(), 0);
    }

    #[test]
    fn fn_00653180_uses_a_null_pointer_without_a_holder() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_3180, &args![this]).u32(), 0);
        assert_eq!(calls_to(&e, POINTER_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_00653230_checks_the_first_list() {
        let mut e = form_engine();
        e.register(CHECK_006532C0, |_, a| (a[1] == LIST_011D5BB8).into_ret());
        assert_eq!(e.call(0x0065_3230, &args![0x77u32]).u32(), 0x4440);
        assert_eq!(e.call(0x0065_3230, &args![0x78u32]).u32(), 0);
    }

    #[test]
    fn fn_00653300_checks_the_second_list() {
        let mut e = form_engine();
        e.register(CHECK_006532C0, |_, a| (a[1] == LIST_011D5BA8).into_ret());
        assert_eq!(e.call(0x0065_3300, &args![0x77u32]).u32(), 0x4440);
    }

    #[test]
    fn fn_00653270_needs_an_object() {
        let mut e = engine();
        answer(&mut e, CHECK_006532C0, 1);
        assert_eq!(e.call(0x0065_3270, &args![0x10u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x0065_3270, &args![0x10u32, 0x20u32]).u32(), 0x20);
    }

    #[test]
    fn fn_006532c0_walks_the_chain() {
        let mut e = engine();
        let this = object_with_vtable(&mut e, 0x7000, 4, 4);
        answer(&mut e, 0x7000 + 0x1008, 1);
        e.register(0x0072_6070, |_, a| {
            (if a[0] < 3 { a[0] + 1 } else { 0 }).into_ret()
        });
        assert!(e.call(0x0065_32c0, &args![this, 3u32]).bool());
        assert!(!e.call(0x0065_32c0, &args![this, 9u32]).bool());
    }

    #[test]
    fn fn_00653340_writes_the_offsets() {
        let mut e = engine();
        e.map(0x011d_5000, 0x1000);
        answer(&mut e, GET_EXTRA_DATA, 0x3300);
        let offsets = e.mem.alloc(24);
        for i in 0..6 {
            e.mem.set_u32(offsets + 4 * i, 10 + i);
        }
        answer(&mut e, 0x0084_e3a0, offsets);
        e.register(0x0045_6650, |_, _| 2u32.into_ret());
        answer(&mut e, 0x0049_ec60, 1);
        answer(&mut e, GEOMETRY_DATA, 0x6600);
        stub(&mut e, &[MARK_AS_CHANGED]);
        let target = e.mem.alloc(0x40);
        let cursor = e.mem.alloc(12);
        e.mem.set_u32(cursor, target);
        e.mem.set_u32(cursor + 4, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0065_3340, &args![0x1000u32, cursor]);
        assert_eq!(e.mem.u32(target), 10);
        assert_eq!(e.mem.u32(target + 0x10 + 8), 15);
        // The caller's cursor is not moved.
        assert_eq!(e.mem.u32(cursor), target);
        assert_eq!(calls_to(&e, MARK_AS_CHANGED), vec![vec![0x6600, 1]]);
        // A null cursor object stops early.
        e.mem.set_u32(cursor, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_3340, &args![0x1000u32, cursor]);
        assert!(calls_to(&e, MARK_AS_CHANGED).is_empty());
    }

    #[test]
    fn fn_006533f0_copies_three_words() {
        let mut e = engine();
        let target = e.mem.alloc(12);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, target);
        let source = e.mem.alloc(12);
        e.mem.set_u32(source + 8, 9);
        e.call(0x0065_33f0, &args![cursor, source]);
        assert_eq!(e.mem.u32(target + 8), 9);
    }

    #[test]
    fn fn_00653430_asks_for_the_named_extra_data() {
        let mut e = engine();
        e.mem.set_u32(EXTRA_DATA_NAME, 0x7777);
        answer(&mut e, GET_EXTRA_DATA, 0x3300);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_3430, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x0065_3430, &args![0x1000u32]).u32(), 0x3300);
        assert_eq!(calls_to(&e, GET_EXTRA_DATA), vec![vec![0x1000, 0x7777]]);
    }

    #[test]
    fn fn_00653460_reads_the_name() {
        let mut e = engine();
        e.mem.set_u32(EXTRA_DATA_NAME, 0x7777);
        assert_eq!(e.call(0x0065_3460, &args![]).u32(), 0x7777);
    }

    #[test]
    fn fn_00653470_applies_with_a_weight() {
        let mut e = form_engine();
        e.register(CHECK_006532C0, |_, _| 1u32.into_ret());
        answer(&mut e, GEOMETRY_DATA, 0x6600);
        answer(&mut e, 0x00a6_7360, 1);
        stub(&mut e, &[0x0053_4550, 0x00a6_73d0, 0x0066_2460]);
        e.register(0x00a6_7410, |e, a| {
            e.mem.set_u32(a[1], 0x9000);
            Ret::default()
        });
        answer(&mut e, GET_EXTRA_DATA, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_3470, &args![0x77u32, 0.5f32]);
        assert_eq!(
            calls_to(&e, 0x0066_2460),
            vec![vec![0x4440, 0x77, 0, 0.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x00a6_73d0), vec![vec![0x6600]]);
        // A negative weight does nothing; a NaN goes on.
        e.call_log = Some(vec![]);
        e.call(0x0065_3470, &args![0x77u32, -0.5f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x0065_3470, &args![0x77u32, f32::NAN]);
        assert_eq!(calls_to(&e, 0x0066_2460).len(), 1);
    }

    /// The string functions: `strcpy_s` and `strrchr` on real bytes, `sprintf_s` and the
    /// string format record their arguments.
    fn string_engine() -> Engine {
        let mut e = engine();
        e.register(STRCPY_S, |e, a| {
            let mut i = 0;
            loop {
                let byte = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, byte);
                if byte == 0 {
                    break;
                }
                i += 1;
            }
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let mut found = 0;
            let mut i = 0;
            loop {
                let byte = e.mem.u8(a[0] + i);
                if byte == 0 {
                    break;
                }
                if u32::from(byte) == a[1] {
                    found = a[0] + i;
                }
                i += 1;
            }
            found.into_ret()
        });
        stub(&mut e, &[SPRINTF_S, STRING_SET, STRING_FORMAT]);
        e.map(0x0119_b000, 0x1000);
        e.mem.set_u32(HAT_SUFFIXES, 0x1111);
        e.mem.set_u32(HAT_SUFFIXES + 4, 0x2222);
        e
    }

    fn text(e: &mut Engine, value: &str) -> u32 {
        let at = e.mem.alloc(value.len() as u32 + 1);
        e.mem.write(at, value.as_bytes());
        at
    }

    #[test]
    fn get_as_egm_file_formats_with_or_without_a_hat() {
        let mut e = string_engine();
        let name = text(&mut e, "head.nif");
        let result = e.mem.alloc(8);
        e.mem.set_u32(result, 0x8800);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0065_3520, &args![result, name, 1u32]).u32(),
            0x8800
        );
        let format = &calls_to(&e, SPRINTF_S)[0];
        assert_eq!(format[2], 0x0106_5780);
        assert_eq!(format[4], 0x2222);
        assert_eq!(e.mem.u8(format[3] + 4), 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_3520, &args![result, name, 2u32]);
        assert_eq!(calls_to(&e, SPRINTF_S)[0][2], 0x0106_5778);
        let plain = text(&mut e, "head");
        assert_eq!(e.call(0x0065_3520, &args![result, plain, 0u32]).u32(), 0);
        assert_eq!(e.call(0x0065_3520, &args![result, 0u32, 0u32]).u32(), 0);
    }

    #[test]
    fn get_as_tri_file_uses_the_tri_formats() {
        let mut e = string_engine();
        let name = text(&mut e, "head.nif");
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0065_36d0, &args![result, name, 0u32]);
        assert_eq!(calls_to(&e, SPRINTF_S)[0][2], 0x0106_579c);
        assert_eq!(calls_to(&e, SPRINTF_S)[0][4], 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x0065_36d0, &args![result, name, -1i32]);
        assert_eq!(calls_to(&e, SPRINTF_S)[0][2], 0x0106_5794);
        assert_eq!(calls_to(&e, STRING_SET)[0][0], result);
    }

    #[test]
    fn fn_006537d0_uses_the_egt_format() {
        let mut e = string_engine();
        let name = text(&mut e, "head.nif");
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0065_37d0, &args![result, name]);
        assert_eq!(calls_to(&e, SPRINTF_S)[0][2], 0x0106_57a8);
        assert_eq!(calls_to(&e, SPRINTF_S)[0].len(), 4);
    }

    #[test]
    fn nearest_detail_dds_steps_down_by_ten() {
        let mut e = string_engine();
        stub(&mut e, &[0x0068_15c0, 0x00af_d270]);
        // Only the age 20 `.dds` file exists (the second format call is age 30's).
        let mut seen = 0;
        e.register_double(0x00af_6540, move |_, _| {
            seen += 1;
            u32::from(seen == 3).into_ret()
        });
        let name = text(&mut e, "face.dds");
        let result = e.mem.alloc(8);
        e.mem.set_u32(result, 0x8800);
        e.call_log = Some(vec![]);
        let found = e.call(0x0065_3880, &args![result, 1u32, 35u32, name]).u32();
        assert_eq!(found, 0x8800);
        let formats = calls_to(&e, STRING_FORMAT);
        assert_eq!(formats[0][1], 0x0106_57c4);
        assert_eq!(formats[0][3], u32::from(b'F'));
        assert_eq!(formats[0][4], 35);
        assert_eq!(formats[1][1], 0x0106_57b0);
        assert_eq!(formats[1][4], 30);
        assert_eq!(formats[2][4], 20);
        // Nothing found: the result is emptied.
        answer(&mut e, 0x00af_6540, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0065_3880, &args![result, 0u32, 15u32, name]).u32(),
            0
        );
        let formats = calls_to(&e, STRING_FORMAT);
        assert_eq!(formats[0][3], u32::from(b'M'));
        assert_eq!(formats.len(), 3);
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![result, 0x0101_1584, 0]]);
        // Out of range.
        assert_eq!(
            e.call(0x0065_3880, &args![result, 2u32, 15u32, name]).u32(),
            0
        );
        assert_eq!(
            e.call(0x0065_3880, &args![result, 0u32, 101u32, name])
                .u32(),
            0
        );
    }

    #[test]
    fn fn_00653cf0_maps_the_kinds() {
        let mut e = engine();
        let answers: Vec<i32> = (0..9u32)
            .map(|kind| e.call(0x0065_3cf0, &args![kind]).i32())
            .collect();
        assert_eq!(answers, vec![-1, 0, 0, 1, 3, 2, 4, 13, -1]);
    }

    #[test]
    fn fn_00653d70_splits_the_difference() {
        let mut e = engine();
        e.map(0x0101_a000, 0x1000);
        e.set_global(0x0101_a6b0, -1.0f64);
        let out = e.mem.alloc(8);
        e.call(0x0065_3d70, &args![1.0f32, 3.5f32, out, out + 4]);
        assert_eq!((e.mem.f32(out), e.mem.f32(out + 4)), (0.0, 2.5));
        e.call(0x0065_3d70, &args![3.0f32, 1.0f32, out, out + 4]);
        assert_eq!((e.mem.f32(out), e.mem.f32(out + 4)), (2.0, 0.0));
        e.call(0x0065_3d70, &args![1.0f32, f32::NAN, out, out + 4]);
        assert!(e.mem.f32(out).is_nan());
        assert_eq!(e.mem.f32(out + 4), 0.0);
    }

    #[test]
    fn fn_00653dc0_fills_or_clears() {
        let mut e = engine();
        e.map(0x0101_6000, 0x1000);
        e.set_global(0x0101_6970, 7.5f32);
        stub(&mut e, &[FILL_00403D30]);
        let values = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0065_3dc0, &args![values, 2u32, 0u32]);
        assert_eq!(e.mem.f32(values + 4), 7.5);
        e.call(0x0065_3dc0, &args![values, 2u32, 1u32]);
        assert_eq!(calls_to(&e, FILL_00403D30), vec![vec![values, 0, 8]]);
        e.call(0x0065_3dc0, &args![0u32, 2u32, 1u32]);
        assert_eq!(calls_to(&e, FILL_00403D30).len(), 1);
    }

    #[test]
    fn set_to_zero_clears() {
        let mut e = engine();
        stub(&mut e, &[FILL_00403D30]);
        e.call_log = Some(vec![]);
        e.call(0x0065_3e20, &args![0x1000u32, 0u32]);
        e.call(0x0065_3e20, &args![0x1000u32, 3u32]);
        assert_eq!(calls_to(&e, FILL_00403D30), vec![vec![0x1000, 0, 12]]);
    }

    #[test]
    fn precache_face_geometry_hands_the_geometry_to_the_renderer() {
        let mut e = engine();
        let object = object_with_vtable(&mut e, 0x7000, 0x10, 4);
        answer(&mut e, 0x7000 + 0x101c, 0x2200);
        answer(&mut e, GEOMETRY_DATA, 0x6600);
        stub(&mut e, &[0x0044_10f0, SET_CONSISTENCY, 0x00e7_4120]);
        answer(&mut e, 0x0043_fad0, 0);
        let shader = object_with_vtable(&mut e, 0x9000, 0x50, 4);
        answer(&mut e, 0x0040_30b0, shader);
        answer(&mut e, 0x9000 + 0x1118, 0x4444);
        let renderer = object_with_vtable(&mut e, 0xb000, 0x40, 4);
        answer(&mut e, 0x0043_c4b0, renderer);
        answer(&mut e, 0x00a5_9d30, 0x3300);
        stub(&mut e, &[0xb000 + 0x10ec]);
        e.call_log = Some(vec![]);
        e.call(0x0065_7010, &args![object, 5u32, 6u32]);
        assert_eq!(calls_to(&e, 0x0044_10f0), vec![vec![0x6600, 6]]);
        assert_eq!(calls_to(&e, SET_CONSISTENCY), vec![vec![0x6600, 0]]);
        assert_eq!(
            calls_to(&e, 0x9000 + 0x1118),
            vec![vec![shader, 0x2200, 0x3300]]
        );
        assert_eq!(
            calls_to(&e, 0xb000 + 0x10ec),
            vec![vec![renderer, 0x2200, 0, 0, 0x4444]]
        );
        assert_eq!(calls_to(&e, 0x00e7_4120), vec![vec![renderer]]);
        // A geometry with `0043fad0` stops after the consistency.
        answer(&mut e, 0x0043_fad0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0065_7010, &args![object, 5u32, 6u32]);
        assert!(calls_to(&e, 0x0043_c4b0).is_empty());
    }

    #[test]
    fn clear_body_textures_removes_the_file() {
        let mut e = engine();
        answer(&mut e, 0x004a_c110, 0x7700);
        stub(&mut e, &[0x0061_47e0, 0x0061_48f0, 0x00a6_1fb0]);
        e.call_log = Some(vec![]);
        e.call(0x0065_70e0, &args![0x1000u32]);
        let name = calls_to(&e, 0x0061_47e0)[0].clone();
        assert_eq!(&name[..2], &[0x7700, 0x1000]);
        let file = calls_to(&e, 0x0061_48f0)[0].clone();
        assert_eq!(file[2], name[2]);
        assert_eq!(calls_to(&e, 0x00a6_1fb0), vec![vec![file[3]]]);
    }

    #[test]
    fn fn_00657150_queues_the_face() {
        let mut e = engine();
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(MODEL_LOADER, 0x1c3c);
        let manager = with_manager(&mut e, 0);
        let _ = manager;
        stub(
            &mut e,
            &[
                0x005d_d590,
                0x005d_d6f0,
                QUEUE_TEXTURE,
                QUEUE_MODEL,
                QUEUE_EGM_FILE,
                QUEUE_TRI_FILE,
                STRCPY_S,
                STRCAT_S,
                0x0040_37b0,
                0x0040_37d0,
                0x0048_e660,
                0x0048_e6c0,
            ],
        );
        answer(&mut e, 0x005f_0cc0, 1);
        answer(&mut e, 0x004a_c110, 0x7700);
        e.register(0x0061_41f0, |e, a| {
            // Eyes, hair, one head part model (0) and the extra parts' list.
            e.mem.set_u32(a[2] + 0x8c, 0x8c00);
            e.mem.set_u32(a[2] + 0x80, 0x8000);
            Ret::default()
        });
        e.register(0x00aa_bdb0, |_, _| Ret {
            st0: 30.0,
            ..Ret::default()
        });
        e.register(0x0040_6ce0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.register(0x00ec_62c0, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        answer(&mut e, TEXTURE_NAME, 0x4e00);
        answer(&mut e, 0x0060_4310, 1);
        let empty = e.mem.alloc(4);
        answer(&mut e, 0x0087_7a30, empty);
        answer(&mut e, 0x0068_15c0, empty);
        answer(&mut e, 0x0072_6070, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_7150, &args![0x1000u32, 2u32, 3u32]);
        // Eyes, hair and head part textures.
        let textures = calls_to(&e, QUEUE_TEXTURE);
        assert_eq!(textures.len(), 3);
        assert_eq!(&textures[0][2..], &[2, 3]);
        let hair_model = &calls_to(&e, QUEUE_MODEL)[0];
        assert_eq!(&hair_model[1..], &[0x8024, 2, 3, 0, 0, 0, 1]);
        assert_eq!(
            calls_to(&e, QUEUE_EGM_FILE),
            vec![vec![0x1c3c, 0x8024, 2, 3, 9]]
        );
        assert_eq!(calls_to(&e, TEXTURE_NAME), vec![vec![0x8c24], vec![0x803c]]);
        assert_eq!(calls_to(&e, 0x0061_41f0)[0][3], 1);
        assert_eq!(calls_to(&e, 0x005d_d6f0).len(), 1);
        assert_eq!(calls_to(&e, SCOPE_OPEN)[0][4], 0xe38);
    }

    #[test]
    fn fn_00657820_appends_the_hat_suffix() {
        let mut e = engine();
        e.map(0x0119_b000, 0x1000);
        e.mem.set_u32(HAT_SUFFIXES, 0x1111);
        e.mem.set_u32(HAT_SUFFIXES + 4, 0x2222);
        stub(&mut e, &[0x0040_46f0, 0x00ec_6380]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_7820, &args![0x5000u32, 1u32]).u32(), 0x5000);
        e.call(0x0065_7820, &args![0x5000u32, 5u32]);
        assert_eq!(calls_to(&e, 0x0040_46f0)[0], vec![0x5000, 0x0104_aad0]);
        assert_eq!(
            calls_to(&e, 0x00ec_6380),
            vec![vec![0x5000, 0x2222], vec![0x5000, 0x1111]]
        );
    }

    #[test]
    fn fn_006578b0_indexes_with_a_check() {
        let mut e = float_vector_engine();
        let vector = float_vector(&mut e, &[1.0, 2.0]);
        let data = e.mem.u32(vector + 0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_78b0, &args![vector, 1u32]).u32(), data + 4);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        assert_eq!(e.call(0x0065_78b0, &args![vector, 2u32]).u32(), data + 8);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn fn_006578e0_appends_or_inserts() {
        let mut e = float_vector_engine();
        let vector = float_vector(&mut e, &[1.0]);
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 9.0);
        e.call(0x0065_78e0, &args![vector, value]);
        let data = e.mem.u32(vector + 0xc);
        assert_eq!(e.mem.f32(data + 4), 9.0);
        assert_eq!(e.mem.u32(vector + 0x10), data + 8);
        // At capacity: the insert path.
        answer(&mut e, 0x0060_b860, 2);
        let end = e.mem.alloc(8);
        e.mem.set_u32(end, 0xaa);
        e.mem.set_u32(end + 4, 0xbb);
        answer(&mut e, 0x0065_f550, end);
        stub(&mut e, &[0x0060_b4d0, 0x0060_b820]);
        e.register(0x0044_a170, |_, a| a[1].into_ret());
        answer(&mut e, 0x005c_a4f0, 1);
        answer(&mut e, 0x004b_0460, 1);
        e.call_log = Some(vec![]);
        e.call(0x0065_78e0, &args![vector, value]);
        assert_eq!(calls_to(&e, 0x0060_b4d0)[0][1..], [0xaa, 0xbb, 1, value]);
    }

    #[test]
    fn vector_constructors_reserve_nothing() {
        let mut e = engine();
        stub(&mut e, &[0x0068_15c0, 0x0065_d3a0]);
        for address in [0x0065_7960u32, 0x0065_7a80, 0x0065_7b50] {
            let this = e.mem.alloc(0x18);
            e.mem.write(this, &[0xcd; 0x18]);
            assert_eq!(e.call(address, &args![this]).u32(), this);
            assert_eq!(e.mem.u32(this + 0xc), 0);
            assert_eq!(e.mem.u32(this + 0x14), 0);
        }
    }

    #[test]
    fn fn_00657960_constructs() {
        let mut e = engine();
        stub(&mut e, &[0x0068_15c0, 0x0065_d3a0]);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x0065_7960, &args![this]);
        assert_eq!(calls_to(&e, 0x0065_d3a0)[0][0], this);
    }

    #[test]
    fn fn_00657a80_constructs() {
        let mut e = engine();
        stub(&mut e, &[0x0068_15c0, 0x0065_d3a0]);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0065_7a80, &args![this]).u32(), this);
    }

    #[test]
    fn fn_00657b50_constructs() {
        let mut e = engine();
        stub(&mut e, &[0x0068_15c0, 0x0065_d3a0]);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0065_7b50, &args![this]).u32(), this);
    }

    fn release_engine() -> Engine {
        let mut e = engine();
        stub(
            &mut e,
            &[
                0x0065_cfb0,
                VECTOR_DEALLOCATE,
                0x0065_8060,
                0x0065_f2b0,
                0x0065_e750,
                0x0065_92f0,
            ],
        );
        e
    }

    fn filled_vector(e: &mut Engine, bytes: u32, capacity: u32) -> u32 {
        let this = e.mem.alloc(0x18);
        let block = e.mem.alloc(capacity.max(4));
        e.mem.set_u32(this + 0xc, block);
        e.mem.set_u32(this + 0x10, block + bytes);
        e.mem.set_u32(this + 0x14, block + capacity);
        this
    }

    #[test]
    fn fn_006579d0_releases_then_destroys() {
        let mut e = release_engine();
        let this = filled_vector(&mut e, 0x3c, 0x78);
        let block = e.mem.u32(this + 0xc);
        e.call_log = Some(vec![]);
        e.call(0x0065_79d0, &args![this]);
        assert_eq!(calls_to(&e, 0x0065_92f0), vec![vec![this + 8, block]]);
        assert_eq!(
            calls_to(&e, VECTOR_DEALLOCATE),
            vec![vec![this + 8, block, 2]]
        );
        assert_eq!(calls_to(&e, 0x0065_cfb0), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn fn_00657d50_releases() {
        let mut e = release_engine();
        let this = filled_vector(&mut e, 0, 0x3c);
        e.call_log = Some(vec![]);
        e.call(0x0065_7d50, &args![this]);
        assert_eq!(calls_to(&e, VECTOR_DEALLOCATE)[0][2], 1);
        // An empty vector deallocates nothing.
        e.call_log = Some(vec![]);
        e.call(0x0065_7d50, &args![this]);
        assert!(calls_to(&e, VECTOR_DEALLOCATE).is_empty());
    }

    #[test]
    fn fn_00657af0_releases_then_destroys() {
        let mut e = release_engine();
        let this = filled_vector(&mut e, 0x88, 0x110);
        e.call_log = Some(vec![]);
        e.call(0x0065_7af0, &args![this]);
        assert_eq!(calls_to(&e, 0x0065_8060).len(), 1);
        assert_eq!(calls_to(&e, VECTOR_DEALLOCATE)[0][2], 2);
        assert_eq!(calls_to(&e, 0x0065_cfb0), vec![vec![this]]);
    }

    #[test]
    fn fn_00657e60_releases() {
        let mut e = release_engine();
        let this = filled_vector(&mut e, 0, 0x88 * 3);
        e.call_log = Some(vec![]);
        e.call(0x0065_7e60, &args![this]);
        assert_eq!(calls_to(&e, VECTOR_DEALLOCATE)[0][2], 3);
    }

    #[test]
    fn fn_00657f60_releases_bytes() {
        let mut e = release_engine();
        let this = filled_vector(&mut e, 2, 7);
        let block = e.mem.u32(this + 0xc);
        e.call_log = Some(vec![]);
        e.call(0x0065_7f60, &args![this]);
        assert_eq!(
            calls_to(&e, 0x0065_f2b0),
            vec![vec![this, block, block + 2]]
        );
        assert_eq!(calls_to(&e, VECTOR_DEALLOCATE)[0][2], 7);
    }

    #[test]
    fn element_counts_divide_signed() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x10, 0x1000 + 0x3c * 2);
        assert_eq!(e.call(0x0065_7a30, &args![this]).i32(), 2);
        e.mem.set_u32(this + 0x10, 0x1000 - 0x3c);
        assert_eq!(e.call(0x0065_7a30, &args![this]).i32(), -1);
    }

    #[test]
    fn fn_00657bc0_counts_six_byte_elements() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x10, 0x100c);
        assert_eq!(e.call(0x0065_7bc0, &args![this]).i32(), 2);
    }

    #[test]
    fn fn_00657c00_counts_and_00657c20_indexes() {
        let mut e = engine();
        stub(&mut e, &[INVALID_PARAMETER]);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x10, 0x1000 + 0x1c * 3);
        assert_eq!(e.call(0x0065_7c00, &args![this]).i32(), 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_7c20, &args![this, 2u32]).u32(), 0x1038);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        e.call(0x0065_7c20, &args![this, 3u32]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn fn_00657c20_indexes() {
        let mut e = engine();
        stub(&mut e, &[INVALID_PARAMETER]);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x2000);
        e.mem.set_u32(this + 0x10, 0x2000 + 0x1c);
        assert_eq!(e.call(0x0065_7c20, &args![this, 0u32]).u32(), 0x2000);
    }

    #[test]
    fn fn_00657a50_indexes() {
        let mut e = engine();
        stub(&mut e, &[INVALID_PARAMETER]);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x2000);
        e.mem.set_u32(this + 0x10, 0x2000 + 0x3c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_7a50, &args![this, 0u32]).u32(), 0x2000);
        e.call(0x0065_7a50, &args![this, 1u32]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn fn_00657be0_steps_the_cursor() {
        let mut e = engine();
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, 0x100);
        e.mem.set_u32(cursor + 4, 0xc);
        e.call(0x0065_7be0, &args![cursor]);
        assert_eq!(e.mem.u32(cursor), 0x10c);
    }

    #[test]
    fn fn_00657c50_inserts_and_answers_the_position() {
        let mut e = engine();
        e.register(FLOAT_VECTOR_SIZE, |_, _| 3u32.into_ret());
        e.register(0x0044_a170, |e, a| {
            e.mem.set_u32(a[1] + 4, 0x1000);
            a[1].into_ret()
        });
        answer(&mut e, 0x005c_a4f0, 1);
        answer(&mut e, 0x004b_0460, 1);
        stub(&mut e, &[0x0060_b4d0, 0x0060_b820]);
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x0065_7c50,
                &args![0x3000u32, result, 0x77u32, 0x1008u32, 0x99u32],
            )
            .u32();
        assert_eq!(back, result);
        assert_eq!(
            calls_to(&e, 0x0060_b4d0),
            vec![vec![0x3000, 0x77, 0x1008, 1, 0x99]]
        );
        let moved = &calls_to(&e, 0x0060_b820)[0];
        assert_eq!(&moved[1..], &[result, 2]);
    }

    #[test]
    fn fn_00657cd0_reserves() {
        let mut e = engine();
        answer(&mut e, 0x0065_8100, 10);
        answer(&mut e, 0x0065_8020, 0x4000);
        stub(&mut e, &[LENGTH_ERROR]);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0065_7cd0, &args![this, 0u32]).u8(), 0);
        assert_eq!(e.call(0x0065_7cd0, &args![this, 2u32]).u8(), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0x4000);
        assert_eq!(e.mem.u32(this + 0x10), 0x4000);
        assert_eq!(e.mem.u32(this + 0x14), 0x4000 + 0x78);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_7cd0, &args![this, 11u32]).u8(), 1);
        assert_eq!(calls_to(&e, LENGTH_ERROR).len(), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn fn_00657dd0_reserves() {
        let mut e = engine();
        answer(&mut e, 0x0065_8130, 10);
        answer(&mut e, 0x0065_8090, 0x4000);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0065_7dd0, &args![this, 2u32]).u8(), 1);
        assert_eq!(e.mem.u32(this + 0x14), 0x4000 + 0x110);
    }

    #[test]
    fn fn_00657ee0_reserves() {
        let mut e = engine();
        answer(&mut e, 0x0065_f6e0, 10);
        answer(&mut e, 0x0065_f7a0, 0x4000);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0065_7ee0, &args![this, 5u32]).u8(), 1);
        assert_eq!(e.mem.u32(this + 0x14), 0x4005);
        assert_eq!(e.call(0x0065_7ee0, &args![this, 0u32]).u8(), 0);
    }

    #[test]
    fn fn_00657ff0_destroys_the_range() {
        let mut e = release_engine();
        e.call_log = Some(vec![]);
        e.call(
            0x0065_7ff0,
            &args![0x3000u32, 0x1000u32, 0x1000u32 + 0x3c * 2],
        );
        assert_eq!(
            calls_to(&e, 0x0065_92f0),
            vec![vec![0x3008, 0x1000], vec![0x3008, 0x103c]]
        );
    }

    #[test]
    fn fn_006580b0_measures_the_distance() {
        let mut e = engine();
        stub(&mut e, &[INVALID_PARAMETER]);
        answer(&mut e, 0x005c_a4f0, 1);
        answer(&mut e, 0x004b_0460, 1);
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_u32(a + 4, 0x1010);
        e.mem.set_u32(b + 4, 0x1000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_80b0, &args![a, b]).i32(), 4);
        assert_eq!(e.call(0x0065_80b0, &args![b, a]).i32(), -4);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        answer(&mut e, 0x004b_0460, 0);
        e.call(0x0065_80b0, &args![a, b]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn operator_assign_copies_size_and_values() {
        let mut e = float_vector_engine();
        stub(&mut e, &[0x0060_b3f0, 0x0040_1460]);
        let source = float_vector(&mut e, &[1.0, 2.0]);
        e.mem.set_u32(source + 0x18, 1);
        e.mem.set_u32(source + 0x1c, 2);
        let this = float_vector(&mut e, &[0.0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_81e0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        assert_eq!(calls_to(&e, 0x0060_b3f0), vec![vec![this, 2]]);
        let copy = &calls_to(&e, 0x0040_1460)[0];
        assert_eq!(copy[1], e.mem.u32(source + 0xc));
        assert_eq!(copy[2], 8);
        e.call_log = Some(vec![]);
        e.call(0x0065_81e0, &args![this, this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00658260_scales_each_value() {
        let mut e = matrix_engine();
        let source = matrix(&mut e, 1, 2, &[1.5, -2.0]);
        // Iterators: +0 position (the element address), +4 end.
        e.register(0x0065_fd10, |e, a| {
            let data = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1], data);
            Ret::default()
        });
        e.register(0x0065_fd40, |e, a| {
            let data = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1], data + 8);
            a[1].into_ret()
        });
        e.register(0x0044_a170, |e, a| {
            let data = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1], data);
            Ret::default()
        });
        e.register(0x0065_dfd0, |e, a| {
            (e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        e.register(0x0064_dda0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0064_dbd0, |e, a| e.mem.u32(a[0]).into_ret());
        for step in [0x0065_9100u32, 0x0065_90c0] {
            e.register(step, |e, a| {
                let at = e.mem.u32(a[0]);
                e.mem.set_u32(a[0], at + 4);
                Ret::default()
            });
        }
        let result = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x0065_8260, &args![source, result, 2.0f32]).u32(),
            result
        );
        let data = e.mem.u32(result + 0xc);
        assert_eq!((e.mem.f32(data), e.mem.f32(data + 4)), (3.0, -4.0));
    }

    #[test]
    fn fn_00658950_replaces_or_adds() {
        let mut e = engine();
        let map = model_map(&mut e);
        let inner = map + 4;
        e.call_log = Some(vec![]);
        e.call(0x0065_8950, &args![inner, 0x55u32, 0x1111u32]);
        let item = e.mem.u32(e.mem.u32(inner + 8));
        assert_eq!(e.mem.u32(item + 8), 0x1111);
        assert_eq!(e.mem.u32(inner + 0xc), 1);
        // Same key: the value is replaced in place, and the key taken (byte +0x10 clear).
        e.call(0x0065_8950, &args![inner, 0x55u32, 0x2222u32]);
        assert_eq!(e.mem.u32(item + 8), 0x2222);
        assert_eq!(e.mem.u32(inner + 0xc), 1);
        assert_eq!(calls_to(&e, POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn set_value_copies_the_key_when_asked() {
        let mut e = engine();
        answer(&mut e, 0x00ec_6130, 4);
        answer(&mut e, 0x00aa_1070, 0x5500);
        stub(&mut e, &[0x0040_4d80]);
        let this = e.mem.alloc(0x14);
        let item = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x0065_8ad0, &args![this, item, 0x77u32, 0x1111u32]);
        assert_eq!(e.mem.u32(item + 4), 0x77);
        assert_eq!(e.mem.u32(item + 8), 0x1111);
        e.mem.set_u8(this + 0x10, 1);
        e.call(0x0065_8ad0, &args![this, item, 0x77u32, 0x1111u32]);
        assert_eq!(e.mem.u32(item + 4), 0x5500);
        assert_eq!(calls_to(&e, 0x0040_4d80), vec![vec![0x5500, 5, 0x77]]);
    }

    #[test]
    fn fn_00658b80_frees_copied_keys() {
        let mut e = engine();
        stub(&mut e, &[0x00aa_10f0]);
        let this = e.mem.alloc(0x14);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 4, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x0065_8b80, &args![this, item]);
        assert!(calls_to(&e, 0x00aa_10f0).is_empty());
        e.mem.set_u8(this + 0x10, 1);
        e.call(0x0065_8b80, &args![this, item]);
        assert_eq!(calls_to(&e, 0x00aa_10f0), vec![vec![0x77]]);
    }

    #[test]
    fn fn_00658bd0_destroys_the_range() {
        let mut e = release_engine();
        e.call_log = Some(vec![]);
        e.call(0x0065_8bd0, &args![0x1000u32, 0x103cu32, 0x3000u32]);
        assert_eq!(calls_to(&e, 0x0065_e750).len(), 1);
        assert_eq!(calls_to(&e, 0x0065_92f0), vec![vec![0x3000, 0x1000]]);
    }

    #[test]
    fn fn_00658c10_allocates_elements() {
        let mut e = engine();
        answer(&mut e, ALLOCATE_PLAIN, 0x4000);
        stub(&mut e, &[0x0044_d6c0, 0x00ec_7894]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_8c10, &args![2u32]).u32(), 0x4000);
        assert_eq!(calls_to(&e, ALLOCATE_PLAIN), vec![vec![0x78]]);
        assert!(calls_to(&e, 0x00ec_7894).is_empty());
        e.call(0x0065_8c10, &args![0x0800_0000u32]);
        assert_eq!(calls_to(&e, 0x00ec_7894)[0][1], 0x0114_0ef8);
    }

    fn map_destructor_engine() -> Engine {
        let mut e = engine();
        stub(&mut e, &[0x00aa_10f0, MAP_CLEAN_UP]);
        e
    }

    #[test]
    fn fn_00658d30_runs_the_destructor_body() {
        let mut e = map_destructor_engine();
        let this = e.mem.alloc(0x14);
        e.call(0x0065_8d30, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0106_58d0);
    }

    #[test]
    fn fn_00658d50_frees_copied_keys_then_the_bases() {
        let mut e = map_destructor_engine();
        let this = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        let second = e.mem.alloc(12);
        e.mem.set_u32(second + 4, 0x22);
        let first = e.mem.alloc(12);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x11);
        e.mem.set_u32(buckets + 4, first);
        e.mem.set_u32(this + 4, 2);
        e.mem.set_u32(this + 8, buckets);
        e.mem.set_u8(this + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x0065_8d50, &args![this]);
        assert_eq!(
            calls_to(&e, 0x00aa_10f0),
            vec![vec![0x11], vec![0x22], vec![buckets]]
        );
        assert_eq!(calls_to(&e, MAP_CLEAN_UP).len(), 2);
    }

    #[test]
    fn fn_00658e10_cleans_then_runs_the_base() {
        let mut e = map_destructor_engine();
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x0065_8e10, &args![this]);
        assert_eq!(calls_to(&e, MAP_CLEAN_UP), vec![vec![this], vec![this]]);
    }

    #[test]
    fn fn_00658e70_frees_the_buckets() {
        let mut e = map_destructor_engine();
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, 0x5500);
        e.call_log = Some(vec![]);
        e.call(0x0065_8e70, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0106_58d0);
        assert_eq!(calls_to(&e, 0x00aa_10f0), vec![vec![0x5500]]);
    }

    #[test]
    fn fn_00659260_destroys_each_element() {
        let mut e = release_engine();
        e.call_log = Some(vec![]);
        e.call(0x0065_9260, &args![0x1000u32, 0x1078u32, 0x3000u32, 9u32]);
        assert_eq!(
            calls_to(&e, 0x0065_92f0),
            vec![vec![0x3000, 0x1000], vec![0x3000, 0x103c]]
        );
    }

    #[test]
    fn fn_00659ae0_constructs_a_model() {
        let mut e = engine();
        stub(&mut e, &[ENTRY_BASE_CONSTRUCT]);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcd; 0x10]);
        assert_eq!(e.call(0x0065_9ae0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0106_5af4);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00652130_answers_the_lock_depth() {
        let mut e = Engine::new();
        e.map(0x011d_5000, 0x1000);
        e.set_global(FACE_GEN_LOCK_DEPTH, 3u32);
        assert_eq!(e.call(0x0065_2130, &args![]).u32(), 3);
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The manager singleton (`011d59e8`), already built.
    fn manager(e: &mut Engine) -> u32 {
        e.map(0x011d_5000, 0x1000);
        let manager = e.mem.alloc(0x1200);
        e.mem.set_u32(MANAGER, manager);
        manager
    }

    fn put_text(e: &mut Engine, at: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            e.mem.set_u8(at + i as u32, *b);
        }
        e.mem.set_u8(at + text.len() as u32, 0);
    }

    fn text(e: &Engine, at: u32) -> Vec<u8> {
        let mut out = vec![];
        let mut at = at;
        while e.mem.u8(at) != 0 {
            out.push(e.mem.u8(at));
            at += 1;
        }
        out
    }

    #[test]
    fn model_map_contents_print_each_model() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        // Two entries: the first without a model.
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 8, 0x6000);
        let pointers = Rc::new(RefCell::new(vec![0u32, entry]));
        e.register(MAP_FIRST_POSITION, |_, _| ret(2));
        let p = pointers.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let left = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], left - 1);
            e.mem.set_u32(a[2], 0x7100 + left);
            e.mem.set_u32(a[3], p.borrow()[2 - left as usize]);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0065_c490, |_, _| ret(4096));
        e.register(0x0065_c110, |_, _| ret(2048));
        e.register(WORD_AT_4, |_, _| ret(3));
        e.register(STRING_FORMAT, |e, a| {
            e.mem.set_u32(a[0], 0x7300);
            Ret::default()
        });
        let output = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(output, vtable);
        e.mem.set_u32(vtable + 0x38, 0x7b00_0038);
        quiet(
            &mut e,
            &[
                POINTER_CONSTRUCT,
                POINTER_DESTRUCT,
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                PRINT_005B5E40,
                0x7b00_0038,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0064_fe80, &args![this, output]);
        let format = calls(&e, STRING_FORMAT);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1..], [MODEL_LINE_FORMAT, 2, 0x7101, 2, 4]);
        assert_eq!(calls(&e, PRINT_005B5E40), vec![vec![0x7300]]);
        assert_eq!(calls(&e, 0x7b00_0038), vec![vec![output, format[0][0], 1]]);
        assert_eq!(calls(&e, MAP_FIRST_POSITION), vec![vec![this + 4]]);
        assert_eq!(calls(&e, STRING_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, POINTER_DESTRUCT).len(), 1);
        // Without an output: no vtable call.
        e.call_log = Some(vec![]);
        e.call(0x0064_fe80, &args![this, 0u32]);
        assert!(calls(&e, 0x7b00_0038).is_empty());
    }

    #[test]
    fn statistic_set_adds_the_scaled_difference() {
        let mut e = Engine::new();
        let manager = manager(&mut e);
        let data = statistics_with(&mut e, manager + 0x30 + 0xb8, 3);
        let block = e.mem.alloc(0x80);
        let (current, cell) = statistic_doubles(&mut e);
        e.mem.set_u32(current, 0.25f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x0065_2320, &args![block, 1u32, 0u32, 2u32, 1.0f32]);
        let made = calls(&e, 0x0065_87a0);
        assert_eq!(made[0][0], data + 2 * 0x3c);
        // The statistic's one value (1.0) times 1.0 - 0.25.
        assert_eq!(e.mem.f32(cell), 0.75);
        let added = calls(&e, 0x0065_86c0);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], block + 0x40);
        let destroyed = calls(&e, ELEMENT_DESTRUCT_00449680);
        assert_eq!(
            destroyed[destroyed.len() - 2..],
            [[added[0][1]], [made[0][1]]]
        );
        // An index past the count, or no block: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0065_2320, &args![block, 1u32, 0u32, 3u32, 1.0f32]);
        e.call(0x0065_2320, &args![0u32, 1u32, 0u32, 0u32, 1.0f32]);
        assert!(calls(&e, 0x0065_87a0).is_empty());
    }

    /// A statistics vector at `entries` with `count` zeroed 0x3C-byte
    /// entries; answers their data.
    fn statistics_with(e: &mut Engine, entries: u32, count: u32) -> u32 {
        let data = e.mem.alloc(0x3c * count.max(1));
        e.mem.set_u32(entries + 0xc, data);
        e.mem.set_u32(entries + 0x10, data + 0x3c * count);
        data
    }

    /// Doubles for the statistic read (`fn_00652230`, its element through
    /// `0060b370`/`00629ab0` is `current`) and the scaling (`fn_00658260`,
    /// one value 1.0 written to `cell`).
    fn statistic_doubles(e: &mut Engine) -> (u32, u32) {
        let current = e.mem.alloc(4);
        let one = e.mem.alloc(4);
        e.mem.set_u32(one, 1.0f32.to_bits());
        let cell = e.mem.alloc(4);
        e.register(0x0060_b370, |_, a| ret(a[1]));
        e.register_double(0x0062_9ab0, move |_, _| ret(current));
        e.register(0x0065_87a0, |_, a| ret(a[1]));
        let left = Rc::new(RefCell::new(0u32));
        let l = left.clone();
        e.register_double(0x0065_fd10, move |_, _| {
            *l.borrow_mut() = 1;
            Ret::default()
        });
        let l = left;
        e.register_double(0x0065_dfd0, move |_, _| {
            let mut left = l.borrow_mut();
            let going = *left > 0;
            *left = left.saturating_sub(1);
            ret(u32::from(going))
        });
        e.register_double(0x0064_dda0, move |_, _| ret(one));
        e.register_double(0x0064_dbd0, move |_, _| ret(cell));
        quiet(
            e,
            &[
                ELEMENT_DESTRUCT_00449680,
                0x0065_86c0,
                INVALID_PARAMETER,
                0x0065_9000,
                0x0065_8f30,
                0x0044_a170,
                0x0065_fd40,
                0x0065_9100,
                0x0065_90c0,
                FR2_ASSERT,
            ],
        );
        (current, cell)
    }

    #[test]
    fn value_pair_and_controls() {
        let mut e = Engine::new();
        let manager = manager(&mut e);
        e.register(0x00aa_bdb0, |_, a| st0(f64::from(a[2]) + 0.5));
        let seen = Rc::new(RefCell::new(vec![]));
        let s = seen.clone();
        e.register_double(0x00aa_c170, move |e, a| {
            s.borrow_mut()
                .extend([e.mem.u32(a[3]), e.mem.u32(a[3] + 4)]);
            ret(0x55)
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0065_2470, &args![0x10u32, 1u32, 0x20u32, 7.0f32])
                .u32(),
            0x55
        );
        assert_eq!(*seen.borrow(), vec![0.5f32.to_bits(), 7.0f32.to_bits()]);
        let control = calls(&e, 0x00aa_c170);
        assert_eq!(control[0][..3], [manager + 0x118, 0, 0x20]);
        assert_eq!(control[0][4], 0x10);
        // 00652fc0 only with both words.
        quiet(&mut e, &[0x00aa_d6b0]);
        e.call_log = Some(vec![]);
        e.call(0x0065_2fc0, &args![2.0f32, 0u32, 5u32]);
        e.call(0x0065_2fc0, &args![2.0f32, 4u32, 0u32]);
        assert!(calls(&e, 0x00aa_d6b0).is_empty());
        e.call(0x0065_2fc0, &args![2.0f32, 4u32, 5u32]);
        assert_eq!(
            calls(&e, 0x00aa_d6b0),
            vec![vec![manager + 0x118, 2.0f32.to_bits(), 4, 5]]
        );
    }

    /// A 0x20-byte matrix with `rows` by `columns` values `values` (its
    /// storage words at +0xC and +0x10 hold the data's start and end).
    fn matrix(e: &mut Engine, at: u32, rows: u32, columns: u32, values: &[f32]) {
        let data = e.mem.alloc(4 * values.len().max(1) as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, v.to_bits());
        }
        e.mem.set_u32(at + 0xc, data);
        e.mem.set_u32(at + 0x10, data + 4 * values.len() as u32);
        e.mem.set_u32(at + 0x14, data + 4 * values.len() as u32);
        e.mem.set_u32(at + 0x18, rows);
        e.mem.set_u32(at + 0x1c, columns);
    }

    #[test]
    fn matrix_equality_and_block_comparison() {
        let mut e = Engine::new();
        let (a, b) = (e.mem.alloc(0x80), e.mem.alloc(0x80));
        for i in 0..4u32 {
            matrix(&mut e, a + 0x20 * i, 1, 2, &[1.0, 2.0]);
            matrix(&mut e, b + 0x20 * i, 1, 2, &[1.0, 2.0]);
        }
        vector_doubles(&mut e);
        e.register(0x00ec_4835, |e, x| {
            let differs = (0..x[2]).any(|i| e.mem.u8(x[0] + i) != e.mem.u8(x[1] + i));
            ret(u32::from(differs))
        });
        assert!(e.call(0x0065_8160, &args![a, b]).bool());
        assert!(!e.call(0x0065_7880, &args![a, b]).bool());
        // Equal blocks: false; both null: true; one null: false.
        assert!(!e.call(0x0065_2900, &args![a, b]).bool());
        assert!(e.call(0x0065_2900, &args![0u32, 0u32]).bool());
        assert!(!e.call(0x0065_2900, &args![a, 0u32]).bool());
        // A value of the last matrix differs.
        matrix(&mut e, b + 0x60, 1, 2, &[1.0, 3.0]);
        assert!(!e.call(0x0065_8160, &args![a + 0x60, b + 0x60]).bool());
        assert!(e.call(0x0065_2900, &args![a, b]).bool());
        // Other dimensions differ; empty matrices of equal dimensions match.
        matrix(&mut e, b, 2, 1, &[1.0, 2.0]);
        assert!(!e.call(0x0065_8160, &args![a, b]).bool());
        matrix(&mut e, a, 0, 2, &[]);
        matrix(&mut e, b, 0, 2, &[]);
        assert!(e.call(0x0065_8160, &args![a, b]).bool());
    }

    /// The `float` vector doubles: size from +0xC/+0x10, room for 16, the
    /// end insert writing the value, the clear emptying, the sizing and
    /// copy logged only.
    fn vector_doubles(e: &mut Engine) {
        e.register(FLOAT_VECTOR_SIZE, |e, a| {
            ret((e.mem.u32(a[0] + 0x10) - e.mem.u32(a[0] + 0xc)) / 4)
        });
        e.register(0x0060_b860, |_, _| ret(16));
        e.register(0x0060_b8a0, |e, a| {
            let value = e.mem.u32(a[3]);
            e.mem.set_u32(a[1], value);
            ret(a[1] + 4)
        });
        e.register(0x0044_a0a0, |e, a| {
            let begin = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[0] + 0x10, begin);
            Ret::default()
        });
        quiet(e, &[0x0060_b3f0, 0x0040_1460, INVALID_PARAMETER]);
    }

    /// Dimension doubles reading +0x18 and +0x1C; resize keeps a log only.
    fn dimension_doubles(e: &mut Engine) {
        e.register(MATRIX_FIRST_DIMENSION, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(MATRIX_SECOND_DIMENSION, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        quiet(e, &[MATRIX_RESIZE]);
    }

    #[test]
    fn block_difference_fills_the_result() {
        let mut e = Engine::new();
        let (a, b, out) = (e.mem.alloc(0x80), e.mem.alloc(0x80), e.mem.alloc(0x80));
        for i in 0..4u32 {
            matrix(&mut e, a + 0x20 * i, 1, 2, &[1.0, 2.0]);
            matrix(&mut e, b + 0x20 * i, 1, 2, &[4.0, 8.0]);
        }
        matrix(&mut e, a + 0x20, 0, 2, &[]);
        for i in 0..4u32 {
            matrix(&mut e, out + 0x20 * i, 0, 0, &[0.0, 0.0]);
        }
        dimension_doubles(&mut e);
        vector_doubles(&mut e);
        e.call_log = Some(vec![]);
        fn_00652e00(&mut e, a, b, out, 1);
        let resized = calls(&e, MATRIX_RESIZE);
        assert_eq!(resized[1], vec![out + 0x20, 0, 0]);
        assert_eq!(resized[0], vec![out, 1, 2]);
        // Row 0: emptied and refilled with b - a.
        assert_eq!(calls(&e, 0x0044_a0a0), vec![vec![out]]);
        let data = e.mem.u32(out + 0xc);
        assert_eq!([e.mem.f32(data), e.mem.f32(data + 4)], [3.0, 6.0]);
        // Row 1 with the flag: copies of the first block's matrices.
        let copies = calls(&e, 0x0040_1460);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][1], e.mem.u32(a + 0x40 + 0xc));
        // Without the flag every non-empty matrix is emptied and refilled.
        e.call_log = Some(vec![]);
        fn_00652e00(&mut e, a, b, out, 0);
        assert!(calls(&e, 0x0040_1460).is_empty());
        assert_eq!(calls(&e, 0x0044_a0a0).len(), 3);
        // A null block: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0065_2e00, &args![a, b, 0u32, 0u32]);
        assert!(calls(&e, MATRIX_RESIZE).is_empty());
    }

    #[test]
    fn block_scale_takes_values_from_zero_to_one() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        let block = e.mem.alloc(0x80);
        for i in 0..4u32 {
            matrix(&mut e, block + 0x20 * i, 1, 1, &[1.0]);
        }
        matrix(&mut e, block + 0x20, 1, 0, &[]);
        dimension_doubles(&mut e);
        quiet(&mut e, &[0x0065_8650]);
        for (value, scaled) in [
            (0.5f32, 3usize),
            (1.5, 0),
            (-0.5, 0),
            (f32::NAN, 3),
            (1.0, 3),
        ] {
            for flag in [0u32, 1] {
                e.call_log = Some(vec![]);
                e.call(0x0065_3000, &args![value, block, flag]);
                let done = calls(&e, 0x0065_8650);
                assert_eq!(done.len(), scaled);
                if scaled > 0 {
                    assert_eq!(done[0], vec![block, value.to_bits()]);
                }
            }
        }
        e.call_log = Some(vec![]);
        e.call(0x0065_3000, &args![0.5f32, 0u32, 0u32]);
        assert!(calls(&e, MATRIX_FIRST_DIMENSION).is_empty());
    }

    /// String doubles: copy, `strrchr`/`strchr` on real bytes, `sprintf_s`
    /// with one `%s` word, the set and get of a string word.
    fn string_doubles(e: &mut Engine) {
        e.register(STRCPY_S, |e, a| {
            let source = text(e, a[2]);
            put_text(e, a[0], &source);
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let bytes = text(e, a[0]);
            ret(bytes
                .iter()
                .rposition(|&b| u32::from(b) == a[1])
                .map_or(0, |i| a[0] + i as u32))
        });
        e.register(STRCHR, |e, a| {
            let bytes = text(e, a[0]);
            ret(bytes
                .iter()
                .position(|&b| u32::from(b) == a[1])
                .map_or(0, |i| a[0] + i as u32))
        });
        e.register(SPRINTF_S, |e, a| {
            let mut out = text(e, a[3]);
            out.extend_from_slice(b"+f");
            put_text(e, a[0], &out);
            Ret::default()
        });
        e.register(STRING_SET, |e, a| {
            let copy = e.mem.alloc(0x110);
            let value = text(e, a[1]);
            put_text(e, copy, &value);
            e.mem.set_u32(a[0], copy);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
    }

    #[test]
    fn model_names_are_cut_at_the_dot_or_the_underscore() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        let out = e.mem.alloc(8);
        let name = e.mem.alloc(0x20);
        put_text(&mut e, name, b"head_hat.dds");
        e.call_log = Some(vec![]);
        let answer = e.call(0x0065_3620, &args![out, name]).u32();
        assert_eq!(text(&e, answer), b"head_hat+f");
        assert_eq!(calls(&e, SPRINTF_S)[0][2], NIF_FORMAT);
        let answer = e.call(0x0065_3c40, &args![out, name]).u32();
        assert_eq!(text(&e, answer), b"head+f");
        // No dot: 0; no underscore: the name; no name: 0.
        put_text(&mut e, name, b"head");
        assert_eq!(e.call(0x0065_3620, &args![out, name]).u32(), 0);
        assert_eq!(e.call(0x0065_3c40, &args![out, name]).u32(), name);
        assert_eq!(e.call(0x0065_3620, &args![out, 0u32]).u32(), 0);
        assert_eq!(e.call(0x0065_3c40, &args![out, 0u32]).u32(), 0);
    }

    #[test]
    fn low_lod_name_is_looked_up_in_the_archives() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        quiet(
            &mut e,
            &[STRING_CONSTRUCT, STRING_DESTRUCT, 0x0068_15c0, 0x00af_d270],
        );
        e.register(STRING_FORMAT, |e, a| {
            let mut out = b"M\\".to_vec();
            out.extend(text(e, a[2]));
            let at = e.mem.alloc(0x40);
            put_text(e, at, &out);
            e.mem.set_u32(a[0], at);
            Ret::default()
        });
        let found = Rc::new(RefCell::new(1u32));
        let f = found.clone();
        e.register_double(0x00af_6540, move |_, _| ret(*f.borrow()));
        let out = e.mem.alloc(8);
        let name = e.mem.alloc(0x20);
        put_text(&mut e, name, b"head.nif");
        e.call_log = Some(vec![]);
        let answer = e.call(0x0065_3a10, &args![out, name]).u32();
        assert_eq!(text(&e, answer), b"head+f");
        assert_eq!(calls(&e, SPRINTF_S)[0][2], LOW_LOD_NIF_FORMAT);
        assert_eq!(calls(&e, STRING_FORMAT)[0][1], MESHES_FORMAT);
        let lookup = calls(&e, 0x00af_6540);
        assert_eq!(lookup[0][0], 0);
        assert_eq!(text(&e, lookup[0][3]), b"M\\head+f");
        assert_eq!(calls(&e, STRING_DESTRUCT).len(), 1);
        // Not found: 0.
        *found.borrow_mut() = 0;
        assert_eq!(e.call(0x0065_3a10, &args![out, name]).u32(), 0);
        // An underscore: set as it is, the name answered; no dot: the name.
        put_text(&mut e, name, b"head_50.nif");
        assert_eq!(e.call(0x0065_3a10, &args![out, name]).u32(), name);
        assert_eq!(text(&e, e.mem.u32(out)), b"head_50.nif");
        put_text(&mut e, name, b"head");
        assert_eq!(e.call(0x0065_3a10, &args![out, name]).u32(), name);
        assert_eq!(e.call(0x0065_3a10, &args![out, 0u32]).u32(), 0);
    }

    #[test]
    fn allocators_and_destroy_ranges() {
        let mut e = Engine::new();
        e.register(ALLOCATE_PLAIN, |_, a| ret(a[0] + 0x10));
        quiet(&mut e, &[0x0044_d6c0, 0x00ec_7894, 0x0065_9310]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0065_8090, &args![0u32, 2u32]).u32(),
            2 * 0x88 + 0x10
        );
        assert_eq!(
            e.call(0x0065_8020, &args![0u32, 2u32]).u32(),
            2 * 0x3c + 0x10
        );
        assert!(calls(&e, 0x00ec_7894).is_empty());
        e.call(0x0065_8ca0, &args![0x0200_0000u32]);
        assert_eq!(calls(&e, 0x00ec_7894)[0][1], 0x0114_0ef8);
        assert_eq!(e.call(0x0065_8100, &args![0u32]).u32(), 0x0444_4444);
        assert_eq!(e.call(0x0065_8130, &args![0u32]).u32(), 0x01e1_e1e1);
        e.register(0x0065_e750, |_, _| ret(0x1ff));
        e.call_log = Some(vec![]);
        let vector = e.mem.alloc(0x10);
        e.call(0x0065_8060, &args![vector, 0x1000u32, 0x1000u32 + 2 * 0x88]);
        assert_eq!(
            calls(&e, 0x0065_9310),
            vec![vec![vector + 8, 0x1000], vec![vector + 8, 0x1088]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0065_9290, &args![0x2000u32, 0x2000u32, 0u32, 0u32]);
        assert!(calls(&e, 0x0065_9310).is_empty());
    }

    #[test]
    fn matrix_sum_walks_three_iterators() {
        let mut e = Engine::new();
        let (a, b) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        e.mem.set_u32(a + 0x18, 1);
        e.mem.set_u32(a + 0x1c, 2);
        e.mem.set_u32(b + 0x18, 2);
        e.mem.set_u32(b + 0x1c, 1);
        let (x, y, z) = (e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(x, 1.5f32.to_bits());
        e.mem.set_u32(x + 4, 2.0f32.to_bits());
        e.mem.set_u32(y, 0.25f32.to_bits());
        e.mem.set_u32(y + 4, 0.5f32.to_bits());
        // Iterators hold an address word; their begin points them at x/y/z.
        e.register_double(0x0065_fd10, move |e, a| {
            e.mem
                .set_u32(a[1], if e.mem.u32(a[0] + 0x18) == 1 { x } else { y });
            Ret::default()
        });
        e.register_double(0x0044_a170, move |e, a| {
            e.mem.set_u32(a[1], z);
            Ret::default()
        });
        e.register_double(0x0065_fd40, move |_, _| ret(x + 8));
        e.register(0x0065_dfd0, |e, a| ret(u32::from(e.mem.u32(a[0]) != a[1])));
        e.register(0x0064_dda0, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0064_dbd0, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0065_9100, |e, a| {
            let at = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], at + 4);
            Ret::default()
        });
        e.register(0x0065_90c0, |e, a| {
            let at = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], at + 4);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x0065_9000,
                0x0065_8f30,
                ELEMENT_DESTRUCT_00449680,
                FR2_ASSERT,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_8500, &args![a, 0x7000u32, b]).u32(), 0x7000);
        assert_eq!(calls(&e, FR2_ASSERT), vec![vec![MATRIX_VT_FILE, 0x16d]]);
        assert_eq!(calls(&e, 0x0065_9000)[0][1..], [1, 2]);
        assert_eq!(e.mem.f32(z), 1.75);
        assert_eq!(e.mem.f32(z + 4), 2.5);
        let copied = calls(&e, 0x0065_8f30);
        assert_eq!(copied[0][0], 0x7000);
        assert_eq!(
            calls(&e, ELEMENT_DESTRUCT_00449680),
            vec![vec![copied[0][1]]]
        );
        // Equal dimensions: no assert.
        e.mem.set_u32(b + 0x18, 1);
        e.mem.set_u32(b + 0x1c, 2);
        e.call_log = Some(vec![]);
        e.call(0x0065_8500, &args![a, 0x7000u32, b]);
        assert!(calls(&e, FR2_ASSERT).is_empty());
    }

    #[test]
    fn model_map_scalar_deleting_destructors() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            quiet(&mut e, &[FREE, 0x0065_8d30, MAP_CLEAN_UP, 0x00aa_10f0]);
            let map = e.mem.alloc(0x14);
            e.call_log = Some(vec![]);
            for addr in [0x0065_8d00u32, 0x0065_8ea0, 0x0065_8ed0, 0x0065_8f00] {
                assert_eq!(e.call(addr, &args![map, flags]).u32(), map);
            }
            assert_eq!(calls(&e, FREE).len(), 4 * flags as usize);
        }
    }

    #[test]
    fn debug_sweep_moves_the_statistic_and_turns() {
        let mut e = Engine::new();
        let manager = manager(&mut e);
        for page in [0x0101_f000u32, 0x0102_0000, 0x0106_5000] {
            e.map(page, 0x1000);
        }
        e.set_global(SWEEP_STEP, 0.5f64);
        e.set_global(SWEEP_TOP, 10.0f64);
        e.set_global(SWEEP_BOTTOM, -10.0f64);
        e.set_global(SWEEP_INDEX, 0u32);
        // The statistics vector (0, 0): count 1.
        statistics_with(&mut e, manager + 0xb8, 1);
        let (current, _) = statistic_doubles(&mut e);
        e.mem.set_u32(current, (-9.75f32).to_bits());
        let actor = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x1e0);
        e.mem.set_u32(actor, vtable);
        e.mem.set_u32(vtable + 0x1d0, 0x7c00_01d0);
        e.mem.set_u32(vtable + 0x1ac, 0x7c00_01ac);
        e.mem.set_u32(vtable + 0x1b0, 0x7c00_01b0);
        e.register(0x7c00_01d0, |_, _| ret(0));
        e.register(0x7c00_01ac, |_, _| ret(0x5100));
        e.register(0x7c00_01b0, |_, _| ret(0x5200));
        e.register(0x0043_b480, |_, a| ret(u32::from(a[0] == 0x5100)));
        let item = e.mem.alloc(4);
        let item_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(item, item_vtable);
        e.mem.set_u32(item_vtable + 0x1c, 0x7c00_001c);
        e.register(0x7c00_001c, |_, _| ret(0x5300));
        e.register_double(0x0043_b4a0, move |_, _| ret(item));
        let owner = e.mem.alloc(4);
        let owner_vtable = e.mem.alloc(0x98);
        e.mem.set_u32(owner, owner_vtable);
        e.mem.set_u32(owner_vtable + 0x94, 0x7c00_0094);
        e.register(0x7c00_0094, |_, _| ret(0x5400));
        e.register_double(OBJECT_OF_00607310, move |_, _| ret(owner));
        e.register(0x007a_f430, |_, _| ret(0x5500));
        e.register(0x0060_1800, |_, _| ret(0x5600));
        e.register(0x005f_0cc0, |_, _| ret(1));
        e.register(WORD_AT_4, |_, _| ret(0x5700));
        e.register(0x005d_9fb0, |_, _| ret(0));
        e.register(MATRIX_FIRST_DIMENSION, |_, _| ret(0));
        e.register(MATRIX_SECOND_DIMENSION, |_, _| ret(0));
        quiet(
            &mut e,
            &[
                VECTOR_CONSTRUCT,
                VECTOR_DESTRUCT,
                0x0060_3ad0,
                0x0065_86c0,
                0x0065_a970,
                MATRIX_RESIZE,
                ELEMENT_DESTRUCT_00449680,
                INVALID_PARAMETER,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0065_2510, &args![actor]);
        // -9.75 - 0.5 = -10.25: at or below -10, the sweep turns up.
        assert_eq!(e.global::<u8>(SWEEP_DOWN), 1);
        assert_eq!(calls(&e, VECTOR_CONSTRUCT).len(), 3);
        assert_eq!(calls(&e, VECTOR_DESTRUCT).len(), 3);
        let faces = calls(&e, 0x0060_3ad0);
        assert_eq!(faces.len(), 2);
        assert!(faces.iter().all(|a| a[0] == 0x5500));
        let applied = calls(&e, 0x0065_a970);
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0][0], 0x5400);
        assert_eq!(applied[0][2..], [0x5300, 0]);
        assert_eq!(calls(&e, 0x005d_9fb0), vec![vec![0x5700, 1]]);
        assert_eq!(calls(&e, WORD_AT_4), vec![vec![0x5500 + 0x10c]]);
        // Up: -9.75 + 0.5 stays below 10.
        e.call(0x0065_2510, &args![actor]);
        assert_eq!(e.global::<u8>(SWEEP_DOWN), 1);
        e.mem.set_u32(current, 9.75f32.to_bits());
        e.call(0x0065_2510, &args![actor]);
        assert_eq!(e.global::<u8>(SWEEP_DOWN), 0);
    }
}
