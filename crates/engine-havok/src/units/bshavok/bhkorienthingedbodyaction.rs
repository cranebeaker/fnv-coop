//! `BSHavok/bhkorienthingedbodyaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkOrientHingedBodyAction` (0x10 bytes) wraps a Havok action: `+8` holds the Havok object and
//! `+0xC` the construction info (`pInfo`), a 0x50-byte block (16-byte aligned, the shift in the
//! byte before it) with two vectors at `+0x10` and `+0x20` and two floats at `+0x30` and `+0x34`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhknicollisionobject::{enter_context, push_viewer_string};
use super::bhknitristripsshape::havok_allocator;

/// The class's own vtable (`010c8914`) and the vtables of the base classes its inlined
/// constructors store on the way (`bhkSerializable` `0102016c`, `bhkAction` `0102009c`).
const VTABLE: u32 = 0x010c_8914;
const BASE_VTABLE_SERIALIZABLE: u32 = 0x0102_016c;
const BASE_VTABLE_ACTION: u32 = 0x0102_009c;
/// The static object counts the constructors increment: the base classes' (`01268154`,
/// `01268118`) and the class's own (`012683e8`, which the destructor decrements).
const SERIALIZABLE_COUNT: u32 = 0x0126_8154;
const ACTION_COUNT: u32 = 0x0126_8118;
const OBJECT_COUNT: u32 = 0x0126_83e8;
/// The word at `012683ec` that `GetViewerStrings` shows as the class name (an `NiRTTI` pointer).
const CLASS_RTTI: u32 = 0x0126_83ec;
/// The 16 bytes at `01267e30` that a new construction info gets for both of its vectors, and the
/// floats at `01016248` and `0101e2bc` it gets for `+0x30` and `+0x34`.
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
const DEFAULT_FLOAT_30: u32 = 0x0101_6248;
const DEFAULT_FLOAT_34: u32 = 0x0101_e2bc;

/// Helpers of the Havok / game heap.
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

/// Base class methods: the `bhkRefObject` constructor (`00c87c30`), the `bhkSerializable` kill
/// (`00c876a0`), the base destructor body (`00c86720`), the clone lock and unlock (`00c876f0`,
/// `00c87700`), `CopyMembers` (`00c86800`), `CopyCinfo` (`00c867d0`), `LoadBinary` (`00c86790`),
/// `SaveBinary` (`00c868b0`) and the viewer strings base (`00c86b80`).
const REF_OBJECT_CONSTRUCTOR: u32 = 0x00c8_7c30;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const BASE_DESTRUCTOR_BODY: u32 = 0x00c8_6720;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
const BASE_COPY_MEMBERS: u32 = 0x00c8_6800;
const BASE_COPY_CINFO: u32 = 0x00c8_67d0;
const BASE_LOAD_BINARY: u32 = 0x00c8_6790;
const BASE_SAVE_BINARY: u32 = 0x00c8_68b0;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00c8_6b80;
/// The Havok action's constructor (`00d2acf0`, six stack words) and the release of the
/// reference `Init` holds on it (`00c905b0`).
const HAVOK_ACTION_CONSTRUCT: u32 = 0x00d2_acf0;
const HAVOK_RELEASE: u32 = 0x00c9_05b0;
/// `NiGetViewerString` for a class name (`00a7d330`, cdecl).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;

layout! {
    /// `bhkOrientHingedBodyAction` (Xbox PDB), 0x10 bytes.
    pub struct BhkOrientHingedBodyAction: 0x10 {
        /// `phkObject` (Xbox PDB, `bhkRefObject`): the Havok action.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB, `bhkSerializable`): the construction info.
        0x0c pInfo: Ptr,
    }
}

/// Byte-swaps the ten words at `+0x10..+0x38` of the construction info.
fn swap_cinfo_words(e: &mut Engine, cinfo: u32) {
    for index in 0..10 {
        let address = cinfo + 0x10 + 4 * index;
        let word = e.mem.u32(address);
        e.mem.set_u32(address, word.swap_bytes());
    }
}

/// What the inlined constructors of the three base classes do after the allocation: the base
/// constructor, the vtables and object counts, and the cleared `pInfo` (the class's own
/// vtable stays; its own object count is not incremented here).
fn construct(e: &mut Engine, this: u32) {
    e.call(REF_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this, BASE_VTABLE_SERIALIZABLE);
    e.mem.set_u32(this + 0xc, 0);
    let count = e.mem.u32(SERIALIZABLE_COUNT);
    e.mem.set_u32(SERIALIZABLE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this, BASE_VTABLE_ACTION);
    let count = e.mem.u32(ACTION_COUNT);
    e.mem.set_u32(ACTION_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this, VTABLE);
}

/// A `MOVAPS` copy of 16 bytes: both addresses must be 16-byte aligned, as the exe faults
/// otherwise.
pub(crate) fn copy_aligned_vector(e: &mut Engine, from: u32, to: u32) {
    assert_eq!(
        from & 0xf,
        0,
        "MOVAPS source {from:#010x} is not 16-byte aligned"
    );
    assert_eq!(
        to & 0xf,
        0,
        "MOVAPS destination {to:#010x} is not 16-byte aligned"
    );
    for word in 0..4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 00cb83f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb83f0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkOrientHingedBodyAction::~bhkOrientHingedBodyAction` (engine map): the destructor body. Sets
/// the vtable, kills the serializable base (`00c876a0`), decrements the class's object count and
/// runs the base destructor body (`00c86720`).
pub fn bhk_orient_hinged_body_action_destructor(
    e: &mut Engine,
    this: Ptr<BhkOrientHingedBodyAction>,
) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_sub(1));
    e.call(BASE_DESTRUCTOR_BODY, &args![this]);
}

// Translated from 00cb8470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkOrientHingedBodyAction::CopyMembers` (engine map): a 5-byte jump to `00c86800`
/// (the base class's `CopyMembers`, which takes the clone and the cloning process on the stack);
/// `this` and both words are passed on.
pub fn bhk_orient_hinged_body_action_copy_members(
    e: &mut Engine,
    this: Ptr<BhkOrientHingedBodyAction>,
    clone: Ptr,
    process: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, clone, process]);
}

// Translated from 00cb84a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cb84a0` (unconfirmed identity, probably `CopyCinfo`): runs the base class's
/// `00c867d0(this, cinfo)`, then, from the Havok object at `this + 8` (read as address 0 when
/// `this` is null), copies its two vectors at `+0x20` and `+0x30` to `cinfo + 0x10` and
/// `cinfo + 0x20` (16-byte aligned copies) and its floats at `+0x50` and `+0x54` to
/// `cinfo + 0x30` and `cinfo + 0x34`.
pub fn fn_00cb84a0(e: &mut Engine, this: Ptr<BhkOrientHingedBodyAction>, cinfo: Ptr) {
    e.call(BASE_COPY_CINFO, &args![this, cinfo]);
    let havok = if this.is_null() {
        0
    } else {
        e.get(this, BhkOrientHingedBodyAction::phkObject).addr()
    };
    let dest = cinfo.addr();
    copy_aligned_vector(e, havok + 0x20, dest + 0x10);
    copy_aligned_vector(e, havok + 0x30, dest + 0x20);
    let value = x87_quiet_bits(e.mem.u32(havok + 0x50));
    e.mem.set_u32(dest + 0x30, value);
    let value = x87_quiet_bits(e.mem.u32(havok + 0x54));
    e.mem.set_u32(dest + 0x34, value);
}

// Translated from 00cb84f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb84f0 - C++ exception unwinding (SEH frame state) is not translated.
/// `fn_00cb84f0` (a constructing allocator for the class): takes 0x10 bytes from `operator new`
/// (`00aa13e0`) and, when that succeeded, constructs a `bhkOrientHingedBodyAction` in them (the
/// base constructor `00c87c30`, the base vtables and counts, `pInfo` cleared, the class's own
/// vtable; the class's own object count is not touched). Returns the object, or null.
pub fn fn_00cb84f0(e: &mut Engine) -> Ptr<BhkOrientHingedBodyAction> {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct(e, block);
    Ptr::new(block)
}

// Translated from 00cb8590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkOrientHingedBodyAction::LoadBinary` (engine map): runs the base class's `LoadBinary`
/// (`00c86790`); when the stream's byte at `+0x1E5` is not 1, takes the construction info
/// (virtual slot `+0xB4`) and byte-swaps its ten words at `+0x10..+0x38`.
pub fn bhk_orient_hinged_body_action_load_binary(
    e: &mut Engine,
    this: Ptr<BhkOrientHingedBodyAction>,
    stream: Ptr,
) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e5) != 1 {
        let cinfo = e.with_stack(4, |e, created| {
            e.vcall(this.addr(), 0xb4, &args![created]).u32()
        });
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00cb8610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkOrientHingedBodyAction::SaveBinary` (engine map): takes the construction info (virtual
/// slot `+0xB4`); when the stream's byte at `+0x1E4` is not 1 byte-swaps its ten words at
/// `+0x10..+0x38`; runs the base class's `SaveBinary` (`00c868b0`); and swaps the words back
/// when the byte, read again, is still not 1.
pub fn bhk_orient_hinged_body_action_save_binary(
    e: &mut Engine,
    this: Ptr<BhkOrientHingedBodyAction>,
    stream: Ptr,
) {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00cb86f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb86f0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkOrientHingedBodyAction::CreateClone` (engine map): with the thread block's `+0x2b4` word
/// at 0x10 and the clone lock held (`00c876f0` / `00c87700`), allocates and constructs a new
/// object as `00cb84f0` does, then runs the original's `CopyMembers` (virtual slot `+0xC8`) with
/// the new object (null when the allocation failed) and `process`. Returns the new object.
pub fn bhk_orient_hinged_body_action_create_clone(
    e: &mut Engine,
    this: Ptr<BhkOrientHingedBodyAction>,
    process: Ptr,
) -> Ptr<BhkOrientHingedBodyAction> {
    let (word, saved) = enter_context(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00cb84f0(e);
    e.vcall(this.addr(), 0xc8, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cb87e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb87e0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkOrientHingedBodyAction::CreateCinfo` (engine map): with the thread block's `+0x2b4` word
/// at 0x10, sets the byte at `created` to whether `pInfo` was empty. If it was, takes 0x50 bytes
/// aligned to 16 from the memory manager (`00aa3e40`, the shift stored in the byte before the
/// block) and makes them the construction info: the words at `+0` and `+4` zero, the 16 bytes
/// at `01267e30` copied to `+0x10` and `+0x20`, the floats at `01016248` and `0101e2bc` to `+0x30`
/// and `+0x34`; stores it in `pInfo` and, when there is a Havok object, fills it from that
/// (`fn_00cb84a0`). Returns `pInfo`.
pub fn bhk_orient_hinged_body_action_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkOrientHingedBodyAction>,
    created: Ptr,
) -> Ptr {
    let (word, saved) = enter_context(e);
    let empty = e.get(this, BhkOrientHingedBodyAction::pInfo).is_null();
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let raw = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x50u32])
            .u32();
        let float_30 = x87_quiet_bits(e.mem.u32(DEFAULT_FLOAT_30));
        let shift = 0x10 - (raw & 0xf) as u8;
        let block = raw.wrapping_add(u32::from(shift));
        e.mem.set_u8(block.wrapping_sub(1), shift);
        e.mem.set_u32(block, 0);
        e.mem.set_u32(block + 4, 0);
        copy_aligned_vector(e, DEFAULT_VECTOR, block + 0x10);
        copy_aligned_vector(e, DEFAULT_VECTOR, block + 0x20);
        e.mem.set_u32(block + 0x30, float_30);
        let float_34 = x87_quiet_bits(e.mem.u32(DEFAULT_FLOAT_34));
        e.mem.set_u32(block + 0x34, float_34);
        let has_havok_object = !e.get(this, BhkOrientHingedBodyAction::phkObject).is_null();
        e.set(this, BhkOrientHingedBodyAction::pInfo, Ptr::new(block));
        if has_havok_object {
            fn_00cb84a0(e, this, Ptr::new(block));
        }
    }
    let info = e.get(this, BhkOrientHingedBodyAction::pInfo);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00cb88d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb88d0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkOrientHingedBodyAction::Init` (engine map): with the thread block's `+0x2b4` word at 0x10,
/// takes 0x60 bytes from the Havok allocator (slot `+4` of the object at the router's `+0x10`),
/// stores 0x60 in the 16-bit word at `+4`, and constructs the Havok action in them
/// (`00d2acf0` with the word at `cinfo + 4`, the addresses `cinfo + 0x10` and `cinfo + 0x20`, the
/// floats at `cinfo + 0x30` and `+0x34` and a zero). Hands it to the action (virtual slot
/// `+0x8C`), releases the creation reference (`00c905b0`) and runs `PostInit` (slot `+0xBC`) with
/// `cinfo`.
pub fn bhk_orient_hinged_body_action_init(
    e: &mut Engine,
    this: Ptr<BhkOrientHingedBodyAction>,
    cinfo: Ptr,
) {
    let (word, saved) = enter_context(e);
    let allocator = havok_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x60u32]).u32();
    e.mem.set_u16(block + 4, 0x60);
    let info = cinfo.addr();
    let first = e.mem.u32(info + 4);
    let float_30 = x87_quiet_bits(e.mem.u32(info + 0x30));
    let float_34 = x87_quiet_bits(e.mem.u32(info + 0x34));
    let action = e
        .call(
            HAVOK_ACTION_CONSTRUCT,
            &args![
                block,
                first,
                info + 0x10,
                info + 0x20,
                float_30,
                float_34,
                0u32
            ],
        )
        .u32();
    e.vcall(this.addr(), 0x8c, &args![action]);
    e.call(HAVOK_RELEASE, &args![action]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cb89c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb89c0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkOrientHingedBodyAction::GetViewerStrings` (engine map): with the thread block's `+0x2b4`
/// word at 0x10, appends to `strings` the base class strings (`00c86b80`) and the class name
/// (`00a7d330` on the `NiRTTI` pointer at `012683ec`).
pub fn bhk_orient_hinged_body_action_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkOrientHingedBodyAction>,
    strings: Ptr,
) {
    let (word, saved) = enter_context(e);
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings.addr(), text);
    e.mem.set_u32(word, saved);
}

// Translated from 00cb8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkOrientHingedBodyAction::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cb83f0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_orient_hinged_body_action_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cb_83f0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cb8460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkOrientHingedBodyAction::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_83ec.
pub fn bhk_orient_hinged_body_action_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_83ec
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cb83f0,
            bhk_orient_hinged_body_action_destructor(Ptr<BhkOrientHingedBodyAction>)
        ),
        entry!(0x00cb8460, bhk_orient_hinged_body_action_get_r_t_t_i() -> u32),
        entry!(
            0x00cb8470,
            bhk_orient_hinged_body_action_copy_members(Ptr<BhkOrientHingedBodyAction>, Ptr, Ptr)
        ),
        entry!(0x00cb8480, bhk_orient_hinged_body_action_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cb84a0, fn_00cb84a0(Ptr<BhkOrientHingedBodyAction>, Ptr)),
        entry!(0x00cb84f0, fn_00cb84f0() -> Ptr<BhkOrientHingedBodyAction>),
        entry!(
            0x00cb8590,
            bhk_orient_hinged_body_action_load_binary(Ptr<BhkOrientHingedBodyAction>, Ptr)
        ),
        entry!(
            0x00cb8610,
            bhk_orient_hinged_body_action_save_binary(Ptr<BhkOrientHingedBodyAction>, Ptr)
        ),
        entry!(
            0x00cb86f0,
            bhk_orient_hinged_body_action_create_clone(
                Ptr<BhkOrientHingedBodyAction>,
                Ptr,
            )
                -> Ptr<BhkOrientHingedBodyAction>
        ),
        entry!(
            0x00cb87e0,
            bhk_orient_hinged_body_action_create_cinfo(Ptr<BhkOrientHingedBodyAction>, Ptr) -> Ptr
        ),
        entry!(
            0x00cb88d0,
            bhk_orient_hinged_body_action_init(Ptr<BhkOrientHingedBodyAction>, Ptr)
        ),
        entry!(
            0x00cb89c0,
            bhk_orient_hinged_body_action_get_viewer_strings(Ptr<BhkOrientHingedBodyAction>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;

    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    const SIZED_DELETE_ADDRESS: u32 = 0x00aa_1460;
    const ALLOCATOR_ROUTER: u32 = 0x00c8_5750;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Words> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with this unit's globals mapped and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(SERIALIZABLE_COUNT, 4);
        e.map(ACTION_COUNT, 4);
        e.map(OBJECT_COUNT, 0x10);
        e.map(DEFAULT_VECTOR, 0x10);
        e.map(DEFAULT_FLOAT_30, 4);
        e.map(DEFAULT_FLOAT_34, 4);
        e.call_log = Some(vec![]);
        e
    }

    /// An object of `size` bytes whose vtable (a zeroed block) holds the `(slot offset, target)`
    /// pairs.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    /// An action whose vtable has the given slots.
    fn action_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<BhkOrientHingedBodyAction> {
        Ptr::new(object_with_slots(e, 0x10, slots))
    }

    /// A 16-byte aligned block of `size` bytes.
    fn aligned_block(e: &mut Engine, size: u32) -> u32 {
        let raw = e.mem.alloc(size + 0x10);
        (raw + 0xf) & !0xf
    }

    #[test]
    fn destructor_kills_the_base_counts_down_and_runs_the_base_body() {
        let mut e = engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR_BODY, |_, _| Ret::default());
        e.mem.set_u32(OBJECT_COUNT, 5);
        let this = e.new_object::<BhkOrientHingedBodyAction>();
        e.call(0x00cb83f0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR_BODY), vec![vec![this.addr()]]);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = engine();
        e.register(0x00cb_83f0, |_, _| Ret::default());
        e.register(SIZED_DELETE_ADDRESS, |_, _| Ret::default());
        let this = 0x5000u32;
        assert_eq!(e.call(0x00cb8480, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, SIZED_DELETE_ADDRESS).is_empty());
        assert_eq!(e.call(0x00cb8480, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, SIZED_DELETE_ADDRESS), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cb_83f0), vec![vec![this]; 2]);
    }

    #[test]
    fn get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_8460, &args![]).u32(), 0x0126_83ec);
    }

    #[test]
    fn copy_members_forwards_its_three_words() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call(0x00cb8470, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![0x1000, 0x2000, 0x3000]]
        );
    }

    #[test]
    fn fn_00cb84a0_copies_the_vectors_and_the_quieted_floats() {
        let mut e = engine();
        e.register(BASE_COPY_CINFO, |_, _| Ret::default());
        let havok = aligned_block(&mut e, 0x60);
        for word in 0..4 {
            e.mem.set_u32(havok + 0x20 + 4 * word, 0x100 + word);
            e.mem.set_u32(havok + 0x30 + 4 * word, 0x200 + word);
        }
        e.mem.set_u32(havok + 0x50, 0x7f80_0001);
        e.mem.set_f32(havok + 0x54, 2.5);
        let this = action_with_slots(&mut e, &[]);
        e.set(this, BhkOrientHingedBodyAction::phkObject, Ptr::new(havok));
        let cinfo = aligned_block(&mut e, 0x50);
        e.call(0x00cb84a0, &args![this, cinfo]);
        for word in 0..4 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * word), 0x100 + word);
            assert_eq!(e.mem.u32(cinfo + 0x20 + 4 * word), 0x200 + word);
        }
        // The signalling NaN comes out quiet; the ordinary float is untouched.
        assert_eq!(e.mem.u32(cinfo + 0x30), 0x7fc0_0001);
        assert_eq!(e.mem.f32(cinfo + 0x34), 2.5);
        assert_eq!(
            calls_to(&e, BASE_COPY_CINFO),
            vec![vec![this.addr(), cinfo]]
        );
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn fn_00cb84a0_faults_on_a_misaligned_vector() {
        let mut e = engine();
        e.register(BASE_COPY_CINFO, |_, _| Ret::default());
        let havok = aligned_block(&mut e, 0x60) + 4;
        let this = action_with_slots(&mut e, &[]);
        e.set(this, BhkOrientHingedBodyAction::phkObject, Ptr::new(havok));
        let cinfo = aligned_block(&mut e, 0x50);
        e.call(0x00cb84a0, &args![this, cinfo]);
    }

    #[test]
    fn fn_00cb84f0_constructs_the_object_or_answers_null() {
        let mut e = engine();
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 0xc, 0x99);
        e.mem.set_u32(SERIALIZABLE_COUNT, 1);
        e.mem.set_u32(ACTION_COUNT, 2);
        e.mem.set_u32(OBJECT_COUNT, 7);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        assert_eq!(e.call(0x00cb84f0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(SERIALIZABLE_COUNT), 2);
        assert_eq!(e.mem.u32(ACTION_COUNT), 3);
        // The class's own count is not touched.
        assert_eq!(e.mem.u32(OBJECT_COUNT), 7);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR), vec![vec![block]]);

        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00cb84f0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR).len(), 1);
    }

    #[test]
    fn load_binary_swaps_the_ten_words_unless_the_stream_byte_is_1() {
        for (flag, swapped) in [(0u8, true), (1, false), (2, true)] {
            let mut e = engine();
            e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
            let cinfo = e.mem.alloc(0x50);
            for word in 0..10 {
                e.mem.set_u32(cinfo + 0x10 + 4 * word, 0x0102_0300 + word);
            }
            e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
            let this = action_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
            let stream = e.mem.alloc(0x260);
            e.mem.set_u8(stream + 0x1e5, flag);
            e.call(0x00cb8590, &args![this, stream]);
            for word in 0..10 {
                let expected = if swapped {
                    (0x0102_0300u32 + word).swap_bytes()
                } else {
                    0x0102_0300 + word
                };
                assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * word), expected);
            }
            // The words outside the range stay.
            assert_eq!(e.mem.u32(cinfo + 0x38), 0);
            assert_eq!(
                calls_to(&e, BASE_LOAD_BINARY),
                vec![vec![this.addr(), stream]]
            );
            assert_eq!(calls_to(&e, 0x7000_00b4).len(), usize::from(swapped));
        }
    }

    #[test]
    fn save_binary_swaps_around_the_base_save() {
        for (flag, swapped) in [(0u8, true), (1, false)] {
            let mut e = engine();
            let cinfo = e.mem.alloc(0x50);
            for word in 0..10 {
                e.mem.set_u32(cinfo + 0x10 + 4 * word, 0x0a0b_0c00 + word);
            }
            e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
            let seen = Rc::new(RefCell::new(Vec::new()));
            let during = seen.clone();
            e.register_double(BASE_SAVE_BINARY, move |e, _| {
                *during.borrow_mut() = (0..10).map(|w| e.mem.u32(cinfo + 0x10 + 4 * w)).collect();
                Ret::default()
            });
            let this = action_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
            let stream = e.mem.alloc(0x260);
            e.mem.set_u8(stream + 0x1e4, flag);
            e.call(0x00cb8610, &args![this, stream]);
            let inside: Vec<u32> = (0..10)
                .map(|w| {
                    let word = 0x0a0b_0c00u32 + w;
                    if swapped {
                        word.swap_bytes()
                    } else {
                        word
                    }
                })
                .collect();
            assert_eq!(*seen.borrow(), inside);
            // Afterwards they are back in the original order.
            for word in 0..10 {
                assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * word), 0x0a0b_0c00 + word);
            }
        }
    }

    #[test]
    fn create_clone_copies_members_into_the_new_object_under_the_lock() {
        let mut e = engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(0x7000_00c8, |_, _| Ret::default());
        let block = e.mem.alloc(0x10);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        let this = action_with_slots(&mut e, &[(0xc8, 0x7000_00c8)]);
        let clone = e.call(0x00cb86f0, &args![this, 0x7000u32]).u32();
        assert_eq!(clone, block);
        assert_eq!(
            calls_to(&e, 0x7000_00c8),
            vec![vec![this.addr(), block, 0x7000]]
        );
        assert_eq!(e.mem.u32(word), 0x42);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| [CLONE_LOCK, CLONE_UNLOCK, 0x7000_00c8].contains(a))
            .collect();
        assert_eq!(order, [CLONE_LOCK, 0x7000_00c8, CLONE_UNLOCK]);

        // A failed allocation still runs CopyMembers, with a null clone.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00cb86f0, &args![this, 0x7000u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x7000_00c8)[1], vec![this.addr(), 0, 0x7000]);
    }

    #[test]
    fn create_cinfo_builds_the_block_once_and_fills_it_from_the_havok_object() {
        let mut e = engine();
        e.register(BASE_COPY_CINFO, |_, _| Ret::default());
        // 16 bytes at DEFAULT_VECTOR and the two floats.
        for word in 0..4 {
            e.mem.set_u32(DEFAULT_VECTOR + 4 * word, 0x300 + word);
        }
        e.mem.set_u32(DEFAULT_FLOAT_30, 0x7f80_0001);
        e.mem.set_f32(DEFAULT_FLOAT_34, 0.75);
        // The memory manager hands out an address that needs a shift of 7.
        let raw = e.mem.alloc(0x80) + 0x9;
        let raw = raw - (raw & 0xf) + 9;
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| ret(raw));
        let havok = aligned_block(&mut e, 0x60);
        for word in 0..4 {
            e.mem.set_u32(havok + 0x20 + 4 * word, 0x500 + word);
            e.mem.set_u32(havok + 0x30 + 4 * word, 0x600 + word);
        }
        e.mem.set_f32(havok + 0x50, 1.5);
        e.mem.set_f32(havok + 0x54, 3.5);
        let this = action_with_slots(&mut e, &[]);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        let created = e.mem.alloc(4);

        let info = e.call(0x00cb87e0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        let block = raw + 7;
        assert_eq!(info, block);
        assert_eq!(e.mem.u8(block - 1), 7);
        assert_eq!(e.get(this, BhkOrientHingedBodyAction::pInfo).addr(), block);
        for word in 0..4 {
            assert_eq!(e.mem.u32(block + 0x10 + 4 * word), 0x300 + word);
            assert_eq!(e.mem.u32(block + 0x20 + 4 * word), 0x300 + word);
        }
        assert_eq!(e.mem.u32(block + 0x30), 0x7fc0_0001);
        assert_eq!(e.mem.f32(block + 0x34), 0.75);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(block + 4), 0);
        // No Havok object: nothing is copied from it.
        assert!(calls_to(&e, BASE_COPY_CINFO).is_empty());
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x50]]
        );
        assert_eq!(e.mem.u32(word), 0x42);

        // A second call finds the construction info and reports that it was not made.
        let again = e.call(0x00cb87e0, &args![this, created]).u32();
        assert_eq!(again, block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);

        // With a Havok object, the new block is filled from it.
        e.set(this, BhkOrientHingedBodyAction::phkObject, Ptr::new(havok));
        e.set(this, BhkOrientHingedBodyAction::pInfo, Ptr::NULL);
        let info = e.call(0x00cb87e0, &args![this, created]).u32();
        assert_eq!(e.mem.u32(info + 0x10), 0x500);
        assert_eq!(e.mem.u32(info + 0x2c), 0x603);
        assert_eq!(e.mem.f32(info + 0x30), 1.5);
        assert_eq!(e.mem.f32(info + 0x34), 3.5);
        assert_eq!(calls_to(&e, BASE_COPY_CINFO), vec![vec![this.addr(), info]]);
    }

    #[test]
    fn init_builds_the_havok_action_and_hands_it_over() {
        let mut e = engine();
        // The router's allocator object at +0x10, with slot +4 allocating.
        let allocator = object_with_slots(&mut e, 0x10, &[(4, 0x7000_0004)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(ALLOCATOR_ROUTER, move |_, _| ret(router));
        let block = e.mem.alloc(0x60);
        e.register_double(0x7000_0004, move |_, _| ret(block));
        let made = e.mem.alloc(0x10);
        e.register_double(HAVOK_ACTION_CONSTRUCT, move |_, _| ret(made));
        e.register(HAVOK_RELEASE, |_, _| Ret::default());
        e.register(0x7000_008c, |_, _| Ret::default());
        e.register(0x7000_00bc, |_, _| Ret::default());
        let cinfo = e.mem.alloc(0x50);
        e.mem.set_u32(cinfo + 4, 0xabcd);
        e.mem.set_u32(cinfo + 0x30, 0x7f80_0003);
        e.mem.set_f32(cinfo + 0x34, 4.0);
        let this = action_with_slots(&mut e, &[(0x8c, 0x7000_008c), (0xbc, 0x7000_00bc)]);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);

        e.call(0x00cb88d0, &args![this, cinfo]);

        assert_eq!(calls_to(&e, 0x7000_0004), vec![vec![allocator, 0x60]]);
        assert_eq!(e.mem.u16(block + 4), 0x60);
        assert_eq!(
            calls_to(&e, HAVOK_ACTION_CONSTRUCT),
            vec![vec![
                block,
                0xabcd,
                cinfo + 0x10,
                cinfo + 0x20,
                0x7fc0_0003,
                4.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(calls_to(&e, 0x7000_008c), vec![vec![this.addr(), made]]);
        assert_eq!(calls_to(&e, HAVOK_RELEASE), vec![vec![made]]);
        assert_eq!(calls_to(&e, 0x7000_00bc), vec![vec![this.addr(), cinfo]]);
        assert_eq!(e.mem.u32(word), 0x42);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name_after_the_base_strings() {
        let mut e = engine();
        e.mem.set_u32(CLASS_RTTI, 0x5151_0001);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        let stored = Rc::new(RefCell::new(Vec::new()));
        let seen = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        let this = action_with_slots(&mut e, &[]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 2);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);

        e.call(0x00cb89c0, &args![this, strings]);

        assert_eq!(
            calls_to(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
        assert_eq!(*stored.borrow(), vec![(2, 0xa000)]);
        assert!(calls_to(&e, STRINGS_GROW).is_empty());
        assert_eq!(e.mem.u32(word), 0x42);

        // A full array is grown first (size 4, step 3).
        e.mem.set_u16(strings + 0xa, 4);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00cb89c0, &args![this, strings]);
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 7]]);
    }
}
