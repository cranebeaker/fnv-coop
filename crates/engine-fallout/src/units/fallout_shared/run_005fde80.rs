//! `fallout shared/run_005fde80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fe040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fe040 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x48 (0x54 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005fe040(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x005f_e160, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0104_9d0c);
    e.mem.set_u32(base + 0x18, 0x0104_9ce8);
    e.call(0x0068_0890, &args![base + 0x30]);
    e.call(0x0040_37b0, &args![base + 0x4c]);
    e.call(0x004f_15a0, &args![this, 0x48]);
    e.mem.set_u32(base + 0x40, 0x0);
    e.call(0x005f_e3b0, &args![this]);
    this
}

// Translated from 005fe030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x24 (`SUB ECX,0x24`) and jumps to `005fdb60`. `005fdb60` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_005fe030(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x005f_db60, &args![this.addr().wrapping_sub(0x24), flags])
        .u32()
}

/// `TESIdleForm`'s two vtables (RTTI names read through their locators):
/// the main one at +0 and the one at +0x18.
const IDLE_FORM_VTABLE: u32 = 0x0104_9d0c;
const IDLE_FORM_VTABLE_18: u32 = 0x0104_9ce8;
/// `TESModelAnim`'s vtable (RTTI name), stored by `005fe160` at +0.
const MODEL_ANIM_VTABLE: u32 = 0x0104_9e48;
/// `00488f50` (`ECX` = the model at +0x18): its base constructor.
const MODEL_CONSTRUCT: u32 = 0x0048_8f50;
/// `004ac110` (`ECX` = actor).
const ACTOR_004AC110: u32 = 0x004a_c110;
/// `0045bb80` (`ECX` = `004ac110`'s result).
const OBJECT_0045BB80: u32 = 0x0045_bb80;
/// `005f65d0` (`ECX` = `0045bb80`'s result, the address of a word holding
/// the idle; `RET 4`): a byte.
const OBJECT_005F65D0: u32 = 0x005f_65d0;
/// `TESActorBase::GetSex` (Xbox PDB name of `005f0cc0`; `ECX` = actor).
const GET_SEX: u32 = 0x005f_0cc0;
/// `00559450` (`ECX` = string): its characters.
const STRING_CHARACTERS: u32 = 0x0055_9450;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `00401030` (cdecl: block): frees it.
const FREE: u32 = 0x0040_1030;
/// `00401000` (cdecl: size): allocates.
const ALLOCATE: u32 = 0x0040_1000;
/// `0058b3f0` (`ECX` = memory): constructs the 0x18-byte child array.
const CHILD_ARRAY_CONSTRUCT: u32 = 0x0058_b3f0;
/// The pointer globals the idle code asks (`011c3f2c`, `011cb6a0`; the
/// second is the one `TESIdleManager`'s functions take as `ECX`).
const HOLDER_011C3F2C: u32 = 0x011c_3f2c;
const IDLE_MANAGER: u32 = 0x011c_b6a0;
/// `004226e0` and `004516b0` (`ECX` = `011c3f2c`'s object): bytes.
const HOLDER_004226E0: u32 = 0x0042_26e0;
const HOLDER_004516B0: u32 = 0x0045_16b0;
/// `00843150` (`ECX` = the idle manager): a byte.
const MANAGER_00843150: u32 = 0x0084_3150;
/// `004013e0` (`ECX` = form): a byte.
const FORM_004013E0: u32 = 0x0040_13e0;
/// `005ff610` (`ECX` = idle).
const IDLE_005FF610: u32 = 0x005f_f610;
/// `00436aa0` (`ECX` = idle): its condition list.
const CONDITIONS: u32 = 0x0043_6aa0;
/// `00680fe0` (`ECX` = condition list): clears it.
const CONDITIONS_CLEAR: u32 = 0x0068_0fe0;
/// `LowProcess::GetGenericLocation` (engine map name of `008041a0`; `ECX` =
/// idle): the word at +0x44 (the parent).
const IDLE_PARENT: u32 = 0x0080_41a0;
/// `005ff5d0` (`ECX` = parent, the idle; `RET 4`).
const PARENT_005FF5D0: u32 = 0x005f_f5d0;
/// `TESIdleManager::GetRootIdleArray_ov2` and `GetLooseIdleArray_ov2`
/// (Xbox PDB names of `006005c0` and `00600690`; `ECX` = manager, the idle;
/// `RET 4`).
const ROOT_IDLE_ARRAY: u32 = 0x0060_05c0;
const LOOSE_IDLE_ARRAY: u32 = 0x0060_0690;
/// `NiFormArray::GetIndexForForm` (Xbox PDB name of `004d6100`; `ECX` =
/// array, the form; `RET 4`): -1 when not found.
const INDEX_FOR_FORM: u32 = 0x004d_6100;
/// `NiFormArray::GetFormAt` (Xbox PDB name of `004d6170`; `ECX` = array, an
/// index; `RET 4`).
const FORM_AT: u32 = 0x004d_6170;
/// `LowProcess::GetSecondGenericLocation` (engine map name of `006733e0`;
/// `ECX` = idle): the word at +0x48 (the previous idle).
const IDLE_PREVIOUS: u32 = 0x0067_33e0;
/// `LowProcess::SetSecondGenericLocation` (engine map name of `00673400`;
/// `ECX` = idle, a word; `RET 4`).
const SET_IDLE_PREVIOUS: u32 = 0x0067_3400;
/// `00486b80` (`ECX` = array, an index; `RET 4`): removes that element.
const ARRAY_REMOVE: u32 = 0x0048_6b80;
/// `0061bdc0` (`ECX` = array).
const ARRAY_0061BDC0: u32 = 0x0061_bdc0;
/// `005ff4a0` and `005ff4d0` (`run_005ff2e0`): the idle's child count and
/// child (`ECX` = idle; `005ff4d0` takes an index, `RET 4`).
const CHILD_COUNT: u32 = 0x005f_f4a0;
const CHILD_AT: u32 = 0x005f_f4d0;
/// `LowProcess::SetGenericLocation` (engine map name of `008d8040`; `ECX` =
/// idle, a word; `RET 4`): sets the word at +0x44 (the parent).
const SET_IDLE_PARENT: u32 = 0x008d_8040;
/// `004037d0` (`ECX` = string): its destructor.
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `006808b0` (`ECX` = condition list): its destructor.
const CONDITIONS_DESTRUCT: u32 = 0x0068_08b0;
/// `0051b950` (`ECX` = the model at +0x18): its destructor.
const MODEL_DESTRUCT: u32 = 0x0051_b950;
/// `TESForm::~TESForm` (Xbox PDB name of `00483630`; `ECX` = form).
const FORM_DESTRUCT: u32 = 0x0048_3630;

// Translated from 005fdf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of the byte at `this + 0x48` is clear. The engine map
/// gives it no name.
pub fn fn_005fdf60(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x48) & 2 == 0
}

// Translated from 005fdf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the byte at `this + 0x48` is clear. The engine map
/// gives it no name.
pub fn fn_005fdf80(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x48) & 4 == 0
}

// Translated from 005fdfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `this` suits an actor (one word; `RET 4`): false for a null
/// actor or when `004ac110` gives nothing; otherwise `005f65d0` (on
/// `0045bb80` of `004ac110`'s result, with a stack word holding `this`)
/// must report true, and then, by the actor's sex (`TESActorBase::GetSex`),
/// 0 needs `005fdf60` and 1 `005fdf80` to report true; any other sex is
/// false. The engine map gives it no name.
pub fn fn_005fdfa0(e: &mut Engine, this: Ptr, actor: Ptr) -> bool {
    if actor.is_null() || e.call(ACTOR_004AC110, &args![actor]).u32() == 0 {
        return false;
    }
    let suits = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), this.addr());
        let first = e.call(ACTOR_004AC110, &args![actor]).u32();
        let second = e.call(OBJECT_0045BB80, &args![first]).u32();
        e.call(OBJECT_005F65D0, &args![second, slot]).bool()
    });
    if !suits {
        return false;
    }
    match e.call(GET_SEX, &args![actor]).u32() {
        0 => fn_005fdf60(e, this),
        1 => fn_005fdf80(e, this),
        _ => false,
    }
}

// Translated from 005fe0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The characters of the string at `this + 0x4c` (`00559450`), or the empty
/// string (`01011584`) when they are null. The engine map gives it no name.
pub fn fn_005fe0f0(e: &mut Engine, this: Ptr) -> u32 {
    let string = this.addr() + 0x4c;
    if e.call(STRING_CHARACTERS, &args![string]).u32() == 0 {
        return EMPTY_STRING;
    }
    e.call(STRING_CHARACTERS, &args![string]).u32()
}

// Translated from 005fe130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape: the destructor body `005fe180`, then
/// `this` freed (`00401030`) when bit 0 of the flags is set; `RET 4`.
/// Returns `this`. The engine map gives it no name.
pub fn fn_005fe130(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005fe180(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 005fe160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the model part (+0x18 of the idle, as `005fe040` passes it):
/// its base (`00488f50`), then the `TESModelAnim` vtable (RTTI). Returns
/// `this`. The engine map gives it no name.
pub fn fn_005fe160(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MODEL_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), MODEL_ANIM_VTABLE);
    this
}

/// Removes `idle` from `array` when it is in it: `00486b80` with its index
/// and `0061bdc0`. With `fix_next`, the element after it, when there is
/// one, first gets the idle's previous idle (`006733e0`, `00673400`).
fn remove_from(e: &mut Engine, array: u32, idle: Ptr, fix_next: bool) {
    if array == 0 {
        return;
    }
    let index = e.call(INDEX_FOR_FORM, &args![array, idle]).u32();
    if index == 0xffff_ffff {
        return;
    }
    if fix_next {
        let next = e.call(FORM_AT, &args![array, index.wrapping_add(1)]).u32();
        if next != 0 {
            let previous = e.call(IDLE_PREVIOUS, &args![idle]).u32();
            e.call(SET_IDLE_PREVIOUS, &args![next, previous]);
        }
    }
    e.call(ARRAY_REMOVE, &args![array, index]);
    e.call(ARRAY_0061BDC0, &args![array]);
}

// Translated from 005fe180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fe180 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESIdleForm`'s destructor body (the vtables it stores are that class's
/// by RTTI): the vtables, `005fe400`; unless `004226e0` on `011c3f2c`'s
/// object reports true, the idle leaves its parent (`005ff5d0` on the
/// parent, `008041a0`) or, without one, the manager's root array (the next
/// element taking its previous idle) and loose array (`006005c0`,
/// `00600690` on `011cb6a0`). With a child array (+0x40): each child
/// (`005ff4a0`, `005ff4d0`) gets parent 0 (`008d8040`) and is deleted (its
/// slot `0x10` with 1), then the array is deleted (its slot 0 with 1).
/// Last the string at +0x4c, the conditions at +0x30, the model at +0x18
/// and the `TESForm` base are destroyed. The engine map gives it no name.
pub fn fn_005fe180(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, IDLE_FORM_VTABLE);
    e.mem.set_u32(base + 0x18, IDLE_FORM_VTABLE_18);
    fn_005fe400(e, this);
    let holder = e.mem.u32(HOLDER_011C3F2C);
    if !e.call(HOLDER_004226E0, &args![holder]).bool() {
        if e.call(IDLE_PARENT, &args![this]).u32() != 0 {
            let parent = e.call(IDLE_PARENT, &args![this]).u32();
            e.call(PARENT_005FF5D0, &args![parent, this]);
        } else {
            let manager = e.mem.u32(IDLE_MANAGER);
            let root = e.call(ROOT_IDLE_ARRAY, &args![manager, this]).u32();
            remove_from(e, root, this, true);
            let manager = e.mem.u32(IDLE_MANAGER);
            let loose = e.call(LOOSE_IDLE_ARRAY, &args![manager, this]).u32();
            remove_from(e, loose, this, false);
        }
    }
    if e.mem.u32(base + 0x40) != 0 {
        let count = e.call(CHILD_COUNT, &args![this]).u32();
        if count != 0 {
            let mut index = 0u32;
            while index < count {
                let child = e.call(CHILD_AT, &args![this, index]).u32();
                e.call(SET_IDLE_PARENT, &args![child, 0u32]);
                if child != 0 {
                    e.vcall(child, 0x10, &args![1u32]);
                }
                index += 1;
            }
        }
        let children = e.mem.u32(base + 0x40);
        if children != 0 {
            e.vcall(children, 0, &args![1u32]);
        }
    }
    e.call(STRING_DESTRUCT, &args![base + 0x4c]);
    e.call(CONDITIONS_DESTRUCT, &args![base + 0x30]);
    e.call(MODEL_DESTRUCT, &args![base + 0x18]);
    e.call(FORM_DESTRUCT, &args![this]);
}

// Translated from 005fe3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the idle's data: the byte at +0x38 is 7, the bytes at +0x39,
/// +0x3a and +0x3e and the word at +0x3c are 0, and the words at +0x44 and
/// +0x48 are 0. The engine map gives it no name.
pub fn fn_005fe3b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x38, 7);
    e.mem.set_u8(base + 0x39, 0);
    e.mem.set_u8(base + 0x3a, 0);
    e.mem.set_u16(base + 0x3c, 0);
    e.mem.set_u8(base + 0x3e, 0);
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x48, 0);
}

// Translated from 005fe400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `005ff610` on the idle when `004516b0` on `011c3f2c`'s object reports
/// true, `004226e0` on it false, `00843150` on the idle manager
/// (`011cb6a0`) true and `004013e0` on the idle true; then the condition
/// list (`00436aa0`) is cleared (`00680fe0`). The engine map gives it no
/// name.
pub fn fn_005fe400(e: &mut Engine, this: Ptr) {
    let holder = e.mem.u32(HOLDER_011C3F2C);
    if e.call(HOLDER_004516B0, &args![holder]).bool() {
        let holder = e.mem.u32(HOLDER_011C3F2C);
        if !e.call(HOLDER_004226E0, &args![holder]).bool() {
            let manager = e.mem.u32(IDLE_MANAGER);
            if e.call(MANAGER_00843150, &args![manager]).bool()
                && e.call(FORM_004013E0, &args![this]).bool()
            {
                e.call(IDLE_005FF610, &args![this]);
            }
        }
    }
    let conditions = e.call(CONDITIONS, &args![this]).u32();
    e.call(CONDITIONS_CLEAR, &args![conditions]);
}

// Translated from 005fe770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fe770 - the exception-unwinding frame (FS:[0]) around the allocation is not translated.
/// The idle's child array (+0x40), made on first use: 0x18 bytes
/// (`00401000`) constructed by `0058b3f0` (0 when the allocation fails).
/// The engine map gives it no name.
pub fn fn_005fe770(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 0x40) != 0 {
        return e.mem.u32(base + 0x40);
    }
    let block = e.call(ALLOCATE, &args![0x18u32]).u32();
    let children = if block != 0 {
        e.call(CHILD_ARRAY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x40, children);
    children
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl), the source type descriptor and
/// the targets: `TESHair`'s (`011863a0`) and `TESIdleForm`'s (`01186a18`),
/// both by their RTTI names.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const HAIR_TYPE: u32 = 0x0118_63a0;
const IDLE_FORM_TYPE: u32 = 0x0118_6a18;

// Translated from 005fde80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHair`'s copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 011863a0, 0)`; nothing without it, otherwise
/// `TESForm::CopyAllComponents` given the uncast source and the flags
/// byte at +0x48.
pub fn fn_005fde80(e: &mut Engine, this: Ptr, source: u32) {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, HAIR_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, source]);
    let flags = e.mem.u8(other + 0x48);
    e.mem.set_u8(this.addr() + 0x48, flags);
}

// Translated from 005fded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHair`'s compare (`RET 4`: the other form; true when they differ;
/// the engine map has no name): true when the cast as in `005fde80`
/// fails, `TESForm::CompareAllComponents` answers true or the flags bytes
/// at +0x48 differ.
pub fn fn_005fded0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, HAIR_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    e.mem.i8(other + 0x48) != e.mem.i8(this.addr() + 0x48)
}

// Translated from 005fdf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x1 of `TESHair`'s flags byte (+0x48) is set.
pub fn fn_005fdf40(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr() + 0x48)) & 1 != 0
}

// Translated from 005fe470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleForm`'s copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 01186a18, 0)`; nothing without it. Otherwise
/// `TESForm::CopyAllComponents` given the cast source, the conditions
/// (`00436aa0` on each) copied by `00680a70(source's, 0)` on this one's,
/// the byte `005ff200` answers kept, the eight bytes at +0x38 copied, and
/// that byte handed to `005ff220` unless `004077c0` answers true.
pub fn fn_005fe470(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, IDLE_FORM_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, other]);
    let theirs = e.call(CONDITIONS, &args![other]).u32();
    let mine = e.call(CONDITIONS, &args![this]).u32();
    e.call(0x0068_0a70, &args![mine, theirs, 0u32]);
    let kept = e.call(0x005f_f200, &args![this]).u8();
    let low = e.mem.u32(other + 0x38);
    let high = e.mem.u32(other + 0x3c);
    e.mem.set_u32(base + 0x38, low);
    e.mem.set_u32(base + 0x3c, high);
    if !e.call(TEST_004077C0, &args![this]).bool() {
        e.call(0x005f_f220, &args![this, u32::from(kept)]);
    }
}

/// `004077c0` (thiscall, no stack words), the test `005fe470` makes
/// before `005ff220`.
const TEST_004077C0: u32 = 0x0040_77c0;

// Translated from 005fe510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleForm`'s compare (`RET 4`: the other form; true when they
/// differ; the engine map has no name): true when the cast as in
/// `005fe470` fails, `TESForm::CompareAllComponents` answers true,
/// `00680b70(other's conditions)` on this one's (`00436aa0`) answers true,
/// or the bytes at +0x38, +0x39, +0x3A, the `u16` at +0x3C or the byte at
/// +0x3E differ.
pub fn fn_005fe510(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, IDLE_FORM_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    let theirs = e.call(CONDITIONS, &args![other]).u32();
    let mine = e.call(CONDITIONS, &args![this]).u32();
    if e.call(0x0068_0b70, &args![mine, theirs]).bool() {
        return true;
    }
    for offset in [0x38u32, 0x39, 0x3a] {
        if e.mem.u8(base + offset) != e.mem.u8(other + offset) {
            return true;
        }
    }
    e.mem.u16(base + 0x3c) != e.mem.u16(other + 0x3c)
        || e.mem.u8(base + 0x3e) != e.mem.u8(other + 0x3e)
}

// Translated from 005fe600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a copy of an idle for a reference (`RET 8`: a byte, the
/// reference; the engine map has no name): `004867a0(byte, reference)` on
/// `this`, through `__RTDynamicCast` to `TESIdleForm` (`01186a18`). The
/// copy's parent (`008d8040`) is what `00853130(+0x44, &found)` on the
/// reference finds, or the word at +0x44 when it answers false or 0; its
/// previous idle (`00673400`) likewise from +0x48. With an array at +0x40:
/// `0061bdc0` on it, its count (`0084e3a0`), the copy's array
/// ([`fn_005fe770`]) sized to it (`00864280`), and each element's vtable
/// slot 0x40 (the byte, the reference) stored in the copy's array
/// (`0061bd20(index, &result)`). Answers the copy.
pub fn fn_005fe600(e: &mut Engine, this: Ptr, flag: u8, reference: u32) -> u32 {
    let base = this.addr();
    let made = e
        .call(0x0048_67a0, &args![this, u32::from(flag), reference])
        .u32();
    let copy = e
        .call(
            DYNAMIC_CAST,
            &args![made, 0u32, CAST_SOURCE, IDLE_FORM_TYPE, 0u32],
        )
        .u32();
    for (offset, setter) in [(0x44u32, SET_IDLE_PARENT), (0x48, SET_IDLE_PREVIOUS)] {
        let found = e.with_stack(4, |e, found| {
            let field = e.mem.u32(base + offset);
            let ok = e.call(0x0085_3130, &args![reference, field, found]).bool();
            let value = e.mem.u32(found.addr());
            if !ok || value == 0 {
                e.mem.u32(base + offset)
            } else {
                value
            }
        });
        e.call(setter, &args![copy, found]);
    }
    let array = e.mem.u32(base + 0x40);
    if array != 0 {
        e.call(ARRAY_0061BDC0, &args![array]);
        let array = e.mem.u32(base + 0x40);
        let count = e.call(0x0084_e3a0, &args![array]).u32();
        let target = fn_005fe770(e, Ptr::new(copy));
        e.call(0x0086_4280, &args![target, count]);
        let mut index = 0u32;
        while index < count {
            let array = e.mem.u32(base + 0x40);
            let slot = e.call(0x0087_7a30, &args![array, index]).u32();
            let element = e.mem.u32(slot);
            let result = e
                .vcall(element, 0x40, &args![u32::from(flag), reference])
                .u32();
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), result);
                e.call(0x0061_bd20, &args![target, index, local]);
            });
            index = index.wrapping_add(1);
        }
    }
    copy
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_de80, fn_005fde80(Ptr, u32)),
        entry!(0x005f_ded0, fn_005fded0(Ptr, u32) -> bool),
        entry!(0x005f_df40, fn_005fdf40(Ptr) -> bool),
        entry!(0x005f_e470, fn_005fe470(Ptr, u32)),
        entry!(0x005f_e510, fn_005fe510(Ptr, u32) -> bool),
        entry!(0x005f_e600, fn_005fe600(Ptr, u8, u32) -> u32),
        entry!(0x005f_e040, fn_005fe040(Ptr) -> Ptr),
        entry!(0x005f_e030, fn_005fe030(Ptr, u32) -> u32),
        entry!(0x005f_df60, fn_005fdf60(Ptr) -> bool),
        entry!(0x005f_df80, fn_005fdf80(Ptr) -> bool),
        entry!(0x005f_dfa0, fn_005fdfa0(Ptr, Ptr) -> bool),
        entry!(0x005f_e0f0, fn_005fe0f0(Ptr) -> u32),
        entry!(0x005f_e130, fn_005fe130(Ptr, u32) -> Ptr),
        entry!(0x005f_e160, fn_005fe160(Ptr) -> Ptr),
        entry!(0x005f_e180, fn_005fe180(Ptr)),
        entry!(0x005f_e3b0, fn_005fe3b0(Ptr)),
        entry!(0x005f_e400, fn_005fe400(Ptr)),
        entry!(0x005f_e770, fn_005fe770(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005fe040_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_e040,
            size: 0x54,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x005f_e160, &[A::T(0x18)]),
                (0x0068_0890, &[A::T(0x30)]),
                (0x0040_37b0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x48)]),
                (0x005f_e3b0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_9d0c),
                (0x18, 4, 0x0104_9ce8),
                (0x40, 4, 0x0),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_005fe030() {
        assert_eq!(
            check(0x005f_e030, 0x005f_db60, 0x24, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod b027_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(zero: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in zero {
            e.register(*addr, |_, _| ret(0));
        }
        e.map(HOLDER_011C3F2C, 4);
        e.map(IDLE_MANAGER, 4);
        e.mem.set_u32(HOLDER_011C3F2C, 0x6100);
        e.mem.set_u32(IDLE_MANAGER, 0x6200);
        e
    }

    #[test]
    fn sex_flags_and_suitability() {
        let mut e = engine(&[ACTOR_004AC110, OBJECT_0045BB80, OBJECT_005F65D0, GET_SEX]);
        let idle = e.mem.alloc(0x50);
        assert!(e.call(0x005f_df60, &args![idle]).bool());
        assert!(e.call(0x005f_df80, &args![idle]).bool());
        e.mem.set_u8(idle + 0x48, 2);
        assert!(!e.call(0x005f_df60, &args![idle]).bool());
        assert!(e.call(0x005f_df80, &args![idle]).bool());
        // Without an actor or its `004ac110` nothing suits.
        assert!(!e.call(0x005f_dfa0, &args![idle, 0u32]).bool());
        assert!(!e.call(0x005f_dfa0, &args![idle, 0x4000u32]).bool());
        e.register(ACTOR_004AC110, |_, _| ret(0x5100));
        e.register(OBJECT_0045BB80, |_, _| ret(0x5200));
        e.register(OBJECT_005F65D0, |e, a| ret(u32::from(e.mem.u32(a[1]) != 0)));
        let (result, log) = run(&mut e, 0x005f_dfa0, &[idle, 0x4000]);
        // Sex 0 needs bit 1 clear: it is set.
        assert!(!result.bool());
        assert_eq!(calls(&log, OBJECT_0045BB80), vec![vec![0x5100]]);
        assert_eq!(calls(&log, OBJECT_005F65D0)[0][0], 0x5200);
        assert_eq!(calls(&log, GET_SEX), vec![vec![0x4000]]);
        e.register(GET_SEX, |_, _| ret(1));
        assert!(e.call(0x005f_dfa0, &args![idle, 0x4000u32]).bool());
        e.register(GET_SEX, |_, _| ret(2));
        assert!(!e.call(0x005f_dfa0, &args![idle, 0x4000u32]).bool());
        e.register(OBJECT_005F65D0, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x005f_dfa0, &[idle, 0x4000]);
        assert!(calls(&log, GET_SEX).is_empty());
    }

    #[test]
    fn name_falls_back_to_the_empty_string() {
        let mut e = engine(&[STRING_CHARACTERS]);
        assert_eq!(e.call(0x005f_e0f0, &args![0x4000u32]).u32(), EMPTY_STRING);
        e.register(STRING_CHARACTERS, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x005f_e0f0, &args![0x4000u32]).u32(), 0x404d);
    }

    #[test]
    fn model_part_and_data_start_values() {
        let mut e = engine(&[MODEL_CONSTRUCT]);
        let model = e.mem.alloc(0x18);
        let (result, log) = run(&mut e, 0x005f_e160, &[model]);
        assert_eq!(result.u32(), model);
        assert_eq!(log, vec![(MODEL_CONSTRUCT, vec![model])]);
        assert_eq!(e.mem.u32(model), MODEL_ANIM_VTABLE);
        let idle = e.mem.alloc(0x50);
        for i in 0xe..0x13 {
            e.mem.set_u32(idle + 4 * i, 0xffff_ffff);
        }
        e.call(0x005f_e3b0, &args![idle]);
        let words: Vec<u32> = (0xe..0x13).map(|i| e.mem.u32(idle + 4 * i)).collect();
        assert_eq!(words, vec![0xff00_0007, 0xff00_0000, 0xffff_ffff, 0, 0]);
    }

    #[test]
    fn child_array_is_made_once() {
        let mut e = engine(&[]);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(CHILD_ARRAY_CONSTRUCT, |_, a| ret(a[0]));
        let idle = e.mem.alloc(0x50);
        let (first, log) = run(&mut e, 0x005f_e770, &[idle]);
        assert_eq!(calls(&log, ALLOCATE), vec![vec![0x18]]);
        assert_eq!(e.mem.u32(idle + 0x40), first.u32());
        let (again, log) = run(&mut e, 0x005f_e770, &[idle]);
        assert_eq!(again.u32(), first.u32());
        assert!(log.is_empty());
        e.mem.set_u32(idle + 0x40, 0);
        e.register(ALLOCATE, |_, _| ret(0));
        assert_eq!(e.call(0x005f_e770, &args![idle]).u32(), 0);
    }

    #[test]
    fn unload_check_runs_005ff610_only_when_all_agree() {
        let zero = [
            HOLDER_004516B0,
            HOLDER_004226E0,
            MANAGER_00843150,
            FORM_004013E0,
            IDLE_005FF610,
            CONDITIONS,
            CONDITIONS_CLEAR,
        ];
        let mut e = engine(&zero);
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x30));
        let (_, log) = run(&mut e, 0x005f_e400, &[0x4000]);
        assert_eq!(
            log,
            vec![
                (HOLDER_004516B0, vec![0x6100]),
                (CONDITIONS, vec![0x4000]),
                (CONDITIONS_CLEAR, vec![0x4030]),
            ]
        );
        for addr in [HOLDER_004516B0, MANAGER_00843150, FORM_004013E0] {
            e.register(addr, |_, _| ret(1));
        }
        let (_, log) = run(&mut e, 0x005f_e400, &[0x4000]);
        assert_eq!(calls(&log, MANAGER_00843150), vec![vec![0x6200]]);
        assert_eq!(calls(&log, IDLE_005FF610), vec![vec![0x4000]]);
        e.register(HOLDER_004226E0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x005f_e400, &[0x4000]);
        assert!(calls(&log, IDLE_005FF610).is_empty());
    }

    const DESTRUCT_CALLEES: [u32; 20] = [
        HOLDER_004516B0,
        HOLDER_004226E0,
        CONDITIONS,
        CONDITIONS_CLEAR,
        IDLE_PARENT,
        PARENT_005FF5D0,
        ROOT_IDLE_ARRAY,
        LOOSE_IDLE_ARRAY,
        INDEX_FOR_FORM,
        FORM_AT,
        IDLE_PREVIOUS,
        SET_IDLE_PREVIOUS,
        ARRAY_REMOVE,
        ARRAY_0061BDC0,
        CHILD_COUNT,
        CHILD_AT,
        SET_IDLE_PARENT,
        STRING_DESTRUCT,
        CONDITIONS_DESTRUCT,
        MODEL_DESTRUCT,
    ];

    #[test]
    fn destructor_leaves_the_root_and_loose_arrays() {
        let mut e = engine(&DESTRUCT_CALLEES);
        e.register(FORM_DESTRUCT, |_, _| ret(0));
        e.register(ROOT_IDLE_ARRAY, |_, _| ret(0x7100));
        e.register(LOOSE_IDLE_ARRAY, |_, _| ret(0x7200));
        e.register(INDEX_FOR_FORM, |_, a| {
            ret(if a[0] == 0x7100 { 3 } else { 0 })
        });
        e.register(FORM_AT, |_, _| ret(0x7300));
        e.register(IDLE_PREVIOUS, |_, _| ret(0x7400));
        let idle = e.mem.alloc(0x54);
        let (_, log) = run(&mut e, 0x005f_e180, &[idle]);
        assert_eq!(e.mem.u32(idle), IDLE_FORM_VTABLE);
        assert_eq!(e.mem.u32(idle + 0x18), IDLE_FORM_VTABLE_18);
        assert_eq!(calls(&log, ROOT_IDLE_ARRAY), vec![vec![0x6200, idle]]);
        assert_eq!(calls(&log, FORM_AT), vec![vec![0x7100, 4]]);
        assert_eq!(calls(&log, SET_IDLE_PREVIOUS), vec![vec![0x7300, 0x7400]]);
        assert_eq!(
            calls(&log, ARRAY_REMOVE),
            vec![vec![0x7100, 3], vec![0x7200, 0]]
        );
        assert_eq!(
            calls(&log, ARRAY_0061BDC0),
            vec![vec![0x7100], vec![0x7200]]
        );
        assert!(calls(&log, CHILD_COUNT).is_empty());
        let tail: Log = log
            .iter()
            .filter(|(a, _)| {
                [
                    STRING_DESTRUCT,
                    CONDITIONS_DESTRUCT,
                    MODEL_DESTRUCT,
                    FORM_DESTRUCT,
                ]
                .contains(a)
            })
            .cloned()
            .collect();
        assert_eq!(
            tail,
            vec![
                (STRING_DESTRUCT, vec![idle + 0x4c]),
                (CONDITIONS_DESTRUCT, vec![idle + 0x30]),
                (MODEL_DESTRUCT, vec![idle + 0x18]),
                (FORM_DESTRUCT, vec![idle]),
            ]
        );
        // Not in an array (-1): nothing removed.
        e.register(INDEX_FOR_FORM, |_, _| ret(0xffff_ffff));
        let (_, log) = run(&mut e, 0x005f_e180, &[idle]);
        assert!(calls(&log, ARRAY_REMOVE).is_empty());
    }

    #[test]
    fn destructor_leaves_its_parent_and_deletes_its_children() {
        let mut e = engine(&DESTRUCT_CALLEES);
        e.register(FORM_DESTRUCT, |_, _| ret(0));
        e.register(IDLE_PARENT, |_, _| ret(0x7500));
        e.register(0x0f00_1000, |_, _| ret(0));
        e.register(0x0f00_1010, |_, _| ret(0));
        e.put_vtable(0x0f00_0000, &[0x0f00_1000, 0, 0, 0, 0x0f00_1010]);
        let child = e.mem.alloc(4);
        e.mem.set_u32(child, 0x0f00_0000);
        let children = e.mem.alloc(4);
        e.mem.set_u32(children, 0x0f00_0000);
        e.register(CHILD_COUNT, |_, _| ret(2));
        e.register_double(CHILD_AT, move |_, a| ret(if a[1] == 0 { child } else { 0 }));
        let idle = e.mem.alloc(0x54);
        e.mem.set_u32(idle + 0x40, children);
        let (_, log) = run(&mut e, 0x005f_e180, &[idle]);
        assert_eq!(calls(&log, PARENT_005FF5D0), vec![vec![0x7500, idle]]);
        assert!(calls(&log, ROOT_IDLE_ARRAY).is_empty());
        assert_eq!(
            calls(&log, SET_IDLE_PARENT),
            vec![vec![child, 0], vec![0, 0]]
        );
        assert_eq!(calls(&log, 0x0f00_1010), vec![vec![child, 1]]);
        assert_eq!(calls(&log, 0x0f00_1000), vec![vec![children, 1]]);
        // While unloading (`004226e0` true) the idle keeps its links.
        e.register(HOLDER_004226E0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x005f_e180, &[idle]);
        assert!(calls(&log, IDLE_PARENT).is_empty());
    }

    #[test]
    fn deleting_destructor_frees_on_bit_zero() {
        let mut e = engine(&DESTRUCT_CALLEES);
        e.register(FORM_DESTRUCT, |_, _| ret(0));
        e.register(FREE, |_, _| ret(0));
        let idle = e.mem.alloc(0x54);
        let (result, log) = run(&mut e, 0x005f_e130, &[idle, 2]);
        assert_eq!(result.u32(), idle);
        assert!(calls(&log, FREE).is_empty());
        assert_eq!(calls(&log, FORM_DESTRUCT), vec![vec![idle]]);
        let (_, log) = run(&mut e, 0x005f_e130, &[idle, 3]);
        assert_eq!(calls(&log, FREE), vec![vec![idle]]);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn hair_copy_compare_and_flag() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| ret(if a[0] == 1 { 0 } else { a[0] }));
        e.register(0x0048_51b0, |_, _| Ret::default());
        e.register(0x0048_5270, |_, _| ret(0));
        let this = e.mem.alloc(0x50);
        let other = e.mem.alloc(0x50);
        e.mem.set_u8(other + 0x48, 0x81);
        assert!(e.call(0x005f_ded0, &args![this, other]).bool());
        e.call_log = Some(vec![]);
        e.call(0x005f_de80, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, CAST_SOURCE, HAIR_TYPE, 0]]
        );
        assert_eq!(e.mem.u8(this + 0x48), 0x81);
        assert!(!e.call(0x005f_ded0, &args![this, other]).bool());
        assert!(e.call(0x005f_ded0, &args![this, 1u32]).bool());
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x005f_ded0, &args![this, other]).bool());
        assert!(e.call(0x005f_df40, &args![this]).bool());
        e.mem.set_u8(this + 0x48, 0x80);
        assert!(!e.call(0x005f_df40, &args![this]).bool());
        e.call_log = Some(vec![]);
        e.call(0x005f_de80, &args![this, 1u32]);
        assert!(calls(&e, 0x0048_51b0).is_empty());
    }

    #[test]
    fn idle_copy_and_compare() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| ret(if a[0] == 1 { 0 } else { a[0] }));
        e.register(0x0048_51b0, |_, _| Ret::default());
        e.register(0x0048_5270, |_, _| ret(0));
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x10));
        e.register(0x0068_0a70, |_, _| Ret::default());
        e.register(0x0068_0b70, |_, _| ret(0));
        e.register(0x005f_f200, |_, _| ret(0x1ab));
        e.register(0x005f_f220, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        let other = e.mem.alloc(0x50);
        e.mem.set_u32(other + 0x38, 0x0403_0201);
        e.mem.set_u32(other + 0x3c, 0x0007_0506);
        for (temporary, kept) in [(0u32, 1usize), (1, 0)] {
            e.register_double(TEST_004077C0, move |_, _| ret(temporary));
            e.call_log = Some(vec![]);
            e.call(0x005f_e470, &args![this, other]);
            assert_eq!(calls(&e, 0x0048_51b0), vec![vec![this, other]]);
            assert_eq!(
                calls(&e, 0x0068_0a70),
                vec![vec![this + 0x10, other + 0x10, 0]]
            );
            assert_eq!(e.mem.u32(this + 0x38), 0x0403_0201);
            assert_eq!(e.mem.u32(this + 0x3c), 0x0007_0506);
            let handed = calls(&e, 0x005f_f220);
            assert_eq!(handed.len(), kept);
            if kept == 1 {
                assert_eq!(handed[0], vec![this, 0xab]);
            }
        }
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005f_e510, &args![this, other]).bool());
        assert_eq!(
            calls(&e, 0x0068_0b70),
            vec![vec![this + 0x10, other + 0x10]]
        );
        for offset in [0x38u32, 0x39, 0x3a, 0x3c, 0x3d, 0x3e] {
            let old = e.mem.u8(this + offset);
            e.mem.set_u8(this + offset, old ^ 0x40);
            assert!(
                e.call(0x005f_e510, &args![this, other]).bool(),
                "{offset:x}"
            );
            e.mem.set_u8(this + offset, old);
        }
        // The byte at +0x3F is not compared.
        e.mem.set_u8(this + 0x3f, 9);
        assert!(!e.call(0x005f_e510, &args![this, other]).bool());
        e.register(0x0068_0b70, |_, _| ret(1));
        assert!(e.call(0x005f_e510, &args![this, other]).bool());
        assert!(e.call(0x005f_e510, &args![this, 1u32]).bool());
        e.register(0x0068_0b70, |_, _| ret(0));
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x005f_e510, &args![this, other]).bool());
    }

    #[test]
    fn fn_005fe600_copies_the_idle_for_a_reference() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        let copy = e.mem.alloc(0x50);
        e.register_double(0x0048_67a0, move |_, _| ret(copy));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        // The reference finds a replacement for the parent only.
        e.register(0x0085_3130, |e, a| {
            if a[1] == 0x4400 {
                e.mem.set_u32(a[2], 0x4401);
            }
            ret(1)
        });
        e.register(SET_IDLE_PARENT, |_, _| Ret::default());
        e.register(SET_IDLE_PREVIOUS, |_, _| Ret::default());
        e.mem.set_u32(this + 0x44, 0x4400);
        e.mem.set_u32(this + 0x48, 0x4800);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x005f_e600, &args![this, 0x101u32, 0x9000u32]).u32(),
            copy
        );
        assert_eq!(calls(&e, 0x0048_67a0), vec![vec![this, 1, 0x9000]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![copy, 0, CAST_SOURCE, IDLE_FORM_TYPE, 0]]
        );
        assert_eq!(calls(&e, SET_IDLE_PARENT), vec![vec![copy, 0x4401]]);
        assert_eq!(calls(&e, SET_IDLE_PREVIOUS), vec![vec![copy, 0x4800]]);

        // With children: each one's slot 0x40 goes into the copy's array.
        let vtable = e.mem.alloc(0x44);
        e.mem.set_u32(vtable + 0x40, 0x7000_0040);
        e.register(0x7000_0040, |_, a| ret(a[0] + a[1] + a[2]));
        let child = e.mem.alloc(4);
        e.mem.set_u32(child, vtable);
        let children = e.mem.alloc(0x10);
        let data = e.mem.alloc(8);
        e.mem.set_u32(children, data);
        e.mem.set_u32(children + 0xc, 2);
        e.mem.set_u32(data, child);
        e.mem.set_u32(data + 4, child);
        e.mem.set_u32(this + 0x40, children);
        let target = e.mem.alloc(0x18);
        e.mem.set_u32(copy + 0x40, target);
        e.register(ARRAY_0061BDC0, |_, _| Ret::default());
        e.register(0x0084_e3a0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(0x0086_4280, |_, _| Ret::default());
        e.register(0x0087_7a30, |e, a| ret(e.mem.u32(a[0]) + 4 * a[1]));
        let stored = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = stored.clone();
        e.register_double(0x0061_bd20, move |e, a| {
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x005f_e600, &args![this, 2u32, 0x9000u32]);
        assert_eq!(calls(&e, 0x0086_4280), vec![vec![target, 2]]);
        let value = child + 2 + 0x9000;
        assert_eq!(
            *stored.borrow(),
            vec![(target, 0, value), (target, 1, value)]
        );
    }
}
