//! `fallout shared/run_0050fe50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005104f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005104f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x15 (0x90 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005104f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0048_cc50, &args![base + 0x5c]);
    e.call(0x0047_5350, &args![base + 0x68]);
    e.call(0x0051_0620, &args![base + 0x70]);
    e.mem.set_u32(base, 0x0102_9d5c);
    e.mem.set_u32(base + 0x30, 0x0102_9d44);
    e.mem.set_u32(base + 0x3c, 0x0102_9d20);
    e.mem.set_u32(base + 0x5c, 0x0102_9d0c);
    e.mem.set_u32(base + 0x68, 0x0102_9cf8);
    e.mem.set_u32(base + 0x70, 0x0102_9ce8);
    e.call(0x0040_37b0, &args![base + 0x88]);
    e.call(0x004f_15a0, &args![this, 0x15]);
    e.call(0x0051_0710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 00510620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the component at `+0x70` of the form `005104f0` builds: it stores
/// the component's base vtable (`0x0102_9ee0`) and returns the object. The engine map
/// gives it no name, so the class is unconfirmed.
pub fn fn_00510620(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0102_9ee0);
    this
}

// Translated from 00510710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of the form `005104f0` builds (form type 0x15; class unconfirmed):
/// zeroes the words at `+0x74`, `+0x78`, `+0x80`, `+0x84` and `+0x7c`, then runs
/// `00438390` on the string at `+0x88` with the text at `0x0101_1584`.
pub fn fn_00510710(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x74, 0);
    e.mem.set_u32(base + 0x78, 0);
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_u32(base + 0x7c, 0);
    e.call(0x0043_8390, &args![base + 0x88, 0x0101_1584u32]);
}

// Translated from 005105f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00510640` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005105f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_0640, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00510080 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, no stack words): takes the object's node
/// (slot +0x174 with 0), runs `NiAVObject::UpdateProperties` (`00a5a040`,
/// engine map) on it, builds a three-`float` point of zeros on the stack
/// (`0043d410`, the first given by `FLDZ`/`FSTP`) and passes it to
/// `00a59c60` on the node. Returns true (`MOV AL,1`).
pub fn fn_00510080(e: &mut Engine, this: Ptr) -> bool {
    let node = e.vcall(this.addr(), 0x174, &args![0u32]).u32();
    e.call(0x00a5_a040, &args![node]);
    e.with_stack(0xc, |e, point| {
        e.call(0x0043_d410, &args![point, 0.0f32.to_bits(), 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![node, point]);
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_04f0, fn_005104f0(Ptr) -> Ptr),
        entry!(0x0051_0620, fn_00510620(Ptr) -> Ptr),
        entry!(0x0051_0710, fn_00510710(Ptr)),
        entry!(0x0051_05f0, fn_005105f0(Ptr, u32) -> Ptr),
        entry!(0x0050_fe50, fn_0050fe50(Ptr, Ptr) -> u32),
        entry!(0x0051_00d0, fn_005100d0(Ptr, u32) -> Ptr),
        entry!(0x0051_0110, fn_00510110(Ptr)),
        entry!(0x0051_0130, fn_00510130(Ptr, u32)),
        entry!(0x0051_01e0, fn_005101e0(Ptr, u32, u32) -> u32),
        entry!(0x0051_02b0, fn_005102b0(Ptr, u32) -> u32),
        entry!(0x0051_02e0, fn_005102e0(Ptr)),
        entry!(0x0051_0360, fn_00510360(Ptr, u32) -> u32),
        entry!(0x0051_0380, fn_00510380(Ptr, u32) -> u32),
        entry!(0x0051_03a0, fn_005103a0(Ptr, u32, u32) -> u32),
        entry!(0x0051_03d0, fn_005103d0(Ptr, u32, u32, u32) -> u32),
        entry!(0x0051_04b0, fn_005104b0(u32, u32, u32) -> u32),
        entry!(0x0051_04d0, fn_005104d0(u32, u32, u32) -> u32),
        entry!(0x0051_0640, fn_00510640(Ptr)),
        entry!(0x0051_0080, fn_00510080(Ptr) -> bool),
    ]
}

/// The model path of an object for a reference (`0050fd90`,
/// thiscall(reference)).
const OBJECT_MODEL_PATH: u32 = 0x0050_fd90;
/// `00444ed0` (thiscall on the reference, bool).
const REFERENCE_TEST_444ED0: u32 = 0x0044_4ed0;
/// The reference's base form (`007af430`, the engine map's
/// `BGSSaveFormBuffer::GetForm` on folded code); also a list node's next
/// node in the list functions below.
const GET_7AF430: u32 = 0x007a_f430;
/// The form type (`00401170`, thiscall).
const FORM_TYPE: u32 = 0x0040_1170;
/// The form type for which the model is not loaded but faded in (0x1E).
const FADE_FORM_TYPE: u32 = 0x1e;
/// `NiPointer` construction from a raw pointer (`00633c90`,
/// thiscall(raw)), its pointer (`00559450`) and destructor (`0040c110`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_DESTRUCT: u32 = 0x0040_c110;
/// The model loader singleton (a pointer global).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `ModelLoader::FindModel` (engine map; thiscall(path, `NiPointer` out)).
const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
/// `ModelLoader::LoadFile` (engine map; thiscall(path, LOD multiplier, 1, 0,
/// 0, 0)).
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
/// `0092c870` (thiscall on the found model).
const MODEL_CALL_92C870: u32 = 0x0092_c870;
/// `0043b230` (thiscall on the found model): the node it gives.
const MODEL_NODE_43B230: u32 = 0x0043_b230;
/// The counter `0050fe50` increments for each scaled model it finds.
const SCALED_MODEL_COUNT: u32 = 0x011c_90ac;
/// The allocator `00aa13e0` (cdecl(size)) and `BSFadeNode::BSFadeNode`
/// (engine map) on its 0xE4-byte block.
const ALLOCATE_AA13E0: u32 = 0x00aa_13e0;
const BS_FADE_NODE_CONSTRUCT: u32 = 0x00b4_e150;
const BS_FADE_NODE_SIZE: u32 = 0xe4;
/// `TES::GetLODMult` (engine map; cdecl(object)).
const TES_GET_LOD_MULT: u32 = 0x0045_c6b0;
/// `TESObjectREFR::GetScale` (engine map; ST0).
const REFERENCE_GET_SCALE: u32 = 0x0056_7400;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `TESObject::GetScaledModelFileName` (engine map; thiscall(reference,
/// buffer)).
const GET_SCALED_MODEL_FILE_NAME: u32 = 0x0050_fdf0;
/// The scaled file name buffer on the stack.
const FILE_NAME_SIZE: u32 = 0x108;

/// List node calls (thiscall on a node): the previous node (`00441110`, the
/// engine map's `PathingLocation::GetWorldspace` on folded code), the next
/// node (`007af430`), setting the previous (`0050f9a0`) and the next
/// (`0050f9c0`) node, and setting the node's list (`00984f60`).
const NODE_PREVIOUS: u32 = 0x0044_1110;
const NODE_NEXT: u32 = GET_7AF430;
const NODE_SET_PREVIOUS: u32 = 0x0050_f9a0;
const NODE_SET_NEXT: u32 = 0x0050_f9c0;
const NODE_SET_LIST: u32 = 0x0098_4f60;
/// The list compared against in `005101e0` (`00726070` on the object at
/// `011c3f2c`).
const LIST_HOLDER: u32 = 0x011c_3f2c;
const HOLDER_LIST: u32 = 0x0072_6070;

/// String helpers: the length of a C string (`0044a670`, cdecl), the
/// string's characters (`0044a970`, thiscall), `memcmp` and `memchr`.
const C_STRING_LENGTH: u32 = 0x0044_a670;
const STRING_DATA: u32 = 0x0044_a970;
const MEMCMP: u32 = 0x00ec_4835;
const MEMCHR: u32 = 0x00ec_a2e0;
/// The "not found" position (`npos`, a word in `.rdata`).
const NOT_FOUND: u32 = 0x0101_73f0;
/// The string's length (`+0x14`).
const STRING_SIZE: u32 = 0x14;

// Translated from 0050fe50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050fe50 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// The object's 3D for a reference (thiscall(reference)): with the model
/// path (`0050fd90`), for a reference that `00444ed0` accepts and whose base
/// form is not of type 0x1E, the model is looked up (`FindModel`, its
/// `0092c870` when found) and a new `BSFadeNode` is the result (null when
/// the 0xE4-byte allocation fails). Otherwise, with the LOD multiplier, a
/// reference whose scale is not 1.0 (NaN included) looks up the scaled
/// model (`GetScaledModelFileName` into a stack buffer): when found, its
/// `0092c870` runs, its node (`0043b230`) is the result and the counter at
/// `011c90ac` grows. Without a result, `ModelLoader::LoadFile(path, LOD
/// multiplier, 1, 0, 0, 0)`.
pub fn fn_0050fe50(e: &mut Engine, this: Ptr, reference: Ptr) -> u32 {
    let path = e.call(OBJECT_MODEL_PATH, &args![this, reference]).u32();
    let mut result = 0u32;
    if !reference.is_null() && e.call(REFERENCE_TEST_444ED0, &args![reference]).bool() {
        let form = e.call(GET_7AF430, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![form]).u32() != FADE_FORM_TYPE {
            return e.with_stack(4, |e, model| {
                e.call(NI_POINTER_CONSTRUCT, &args![model, 0u32]);
                let loader = e.global::<u32>(MODEL_LOADER);
                e.call(MODEL_LOADER_FIND_MODEL, &args![loader, path, model]);
                if e.call(NI_POINTER_GET, &args![model]).u32() != 0 {
                    let found = e.call(NI_POINTER_GET, &args![model]).u32();
                    e.call(MODEL_CALL_92C870, &args![found]);
                }
                let block = e.call(ALLOCATE_AA13E0, &args![BS_FADE_NODE_SIZE]).u32();
                let node = if block == 0 {
                    0
                } else {
                    e.call(BS_FADE_NODE_CONSTRUCT, &args![block]).u32()
                };
                e.call(NI_POINTER_DESTRUCT, &args![model]);
                node
            });
        }
    }
    let lod = e.call(TES_GET_LOD_MULT, &args![this]).u32();
    if !reference.is_null() {
        let scale = e.call(REFERENCE_GET_SCALE, &args![reference]).f64();
        let one: f64 = e.global(ONE);
        // `FCOMP; TEST AH,0x44; JNP`: only an (ordered) equal skips.
        if scale != one {
            e.with_stack(FILE_NAME_SIZE, |e, name| {
                e.call(GET_SCALED_MODEL_FILE_NAME, &args![this, reference, name]);
                e.with_stack(4, |e, model| {
                    e.call(NI_POINTER_CONSTRUCT, &args![model, 0u32]);
                    let loader = e.global::<u32>(MODEL_LOADER);
                    e.call(MODEL_LOADER_FIND_MODEL, &args![loader, name, model]);
                    if e.call(NI_POINTER_GET, &args![model]).u32() != 0 {
                        let found = e.call(NI_POINTER_GET, &args![model]).u32();
                        e.call(MODEL_CALL_92C870, &args![found]);
                        let found = e.call(NI_POINTER_GET, &args![model]).u32();
                        result = e.call(MODEL_NODE_43B230, &args![found]).u32();
                        let count = e.global::<u32>(SCALED_MODEL_COUNT).wrapping_add(1);
                        e.set_global(SCALED_MODEL_COUNT, count);
                    }
                    e.call(NI_POINTER_DESTRUCT, &args![model]);
                });
            });
        }
    }
    if result == 0 {
        let loader = e.global::<u32>(MODEL_LOADER);
        result = e
            .call(
                MODEL_LOADER_LOAD_FILE,
                &args![loader, path, lod, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
    }
    result
}

// Translated from 005100d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the four words of `this` (the list's count, head, tail and one
/// more) and returns `this`; the one stack word is not read.
pub fn fn_005100d0(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    for offset in [0u32, 4, 8, 0xc] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    this
}

// Translated from 00510110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list (`005102e0`).
pub fn fn_00510110(e: &mut Engine, this: Ptr) {
    fn_005102e0(e, this);
}

// Translated from 00510130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unlinks `node` from the list (count `+0`, head `+4`, tail `+8`): the head
/// moves to its next node and the tail to its previous one when it is
/// either, its neighbours are linked to each other, its list is cleared
/// (`00984f60(0)`) and the count drops.
pub fn fn_00510130(e: &mut Engine, this: Ptr, node: u32) {
    let base = this.addr();
    if node == e.mem.u32(base + 4) {
        let head = e.mem.u32(base + 4);
        let next = e.call(NODE_NEXT, &args![head]).u32();
        e.mem.set_u32(base + 4, next);
    }
    if node == e.mem.u32(base + 8) {
        let tail = e.mem.u32(base + 8);
        let previous = e.call(NODE_PREVIOUS, &args![tail]).u32();
        e.mem.set_u32(base + 8, previous);
    }
    let previous = e.call(NODE_PREVIOUS, &args![node]).u32();
    if previous != 0 {
        let next = e.call(NODE_NEXT, &args![node]).u32();
        e.call(NODE_SET_NEXT, &args![previous, next]);
    }
    let next = e.call(NODE_NEXT, &args![node]).u32();
    if next != 0 {
        let previous = e.call(NODE_PREVIOUS, &args![node]).u32();
        e.call(NODE_SET_PREVIOUS, &args![next, previous]);
    }
    e.call(NODE_SET_LIST, &args![node, 0u32]);
    let count = e.mem.u32(base).wrapping_sub(1);
    e.mem.set_u32(base, count);
}

// Translated from 005101e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links `node` into the list after `previous` (null: at the front) and
/// returns it. When this list is the one `00726070` gives for the object at
/// `011c3f2c`, `00401170` runs on the node first (its result unused). The
/// node's previous is `previous`; its next is `previous`'s next (null without
/// `previous`), which then points back at it, and `previous` points at it;
/// it becomes the head when it has no previous and the tail when it has no
/// next; the count grows and the node's list is set (`00984f60(this)`).
pub fn fn_005101e0(e: &mut Engine, this: Ptr, previous: u32, node: u32) -> u32 {
    let holder = e.global::<u32>(LIST_HOLDER);
    let list = e.call(HOLDER_LIST, &args![holder]).u32();
    if this.addr() == list {
        e.call(FORM_TYPE, &args![node]);
    }
    e.call(NODE_SET_PREVIOUS, &args![node, previous]);
    if previous != 0 {
        let next = e.call(NODE_NEXT, &args![previous]).u32();
        e.call(NODE_SET_NEXT, &args![node, next]);
        if e.call(NODE_NEXT, &args![previous]).u32() != 0 {
            let next = e.call(NODE_NEXT, &args![previous]).u32();
            e.call(NODE_SET_PREVIOUS, &args![next, node]);
        }
        e.call(NODE_SET_NEXT, &args![previous, node]);
    } else {
        e.call(NODE_SET_NEXT, &args![node, 0u32]);
    }
    let base = this.addr();
    if fn_00510360(e, this, node) == 0 {
        e.mem.set_u32(base + 8, node);
    }
    if fn_00510380(e, this, node) == 0 {
        e.mem.set_u32(base + 4, node);
    }
    let count = e.mem.u32(base).wrapping_add(1);
    e.mem.set_u32(base, count);
    e.call(NODE_SET_LIST, &args![node, this]);
    node
}

// Translated from 005102b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `node` after the tail (`005101e0(tail, node)`) and returns it.
pub fn fn_005102b0(e: &mut Engine, this: Ptr, node: u32) -> u32 {
    let tail = e.mem.u32(this.addr() + 8);
    fn_005101e0(e, this, tail, node)
}

// Translated from 005102e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every node from the tail back (each one's virtual `+0x10` with
/// 1, after reading its previous node), then clears the count, head and
/// tail.
pub fn fn_005102e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    while e.mem.u32(base + 8) != 0 {
        let tail = e.mem.u32(base + 8);
        let previous = e.call(NODE_PREVIOUS, &args![tail]).u32();
        let node = e.mem.u32(base + 8);
        if node != 0 {
            e.vcall(node, 0x10, &args![1u32]);
        }
        e.mem.set_u32(base + 8, previous);
    }
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
}

// Translated from 00510360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The next node of `node` (`007af430`); `this` is not read.
pub fn fn_00510360(e: &mut Engine, _this: Ptr, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

// Translated from 00510380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The previous node of `node` (`00441110`); `this` is not read.
pub fn fn_00510380(e: &mut Engine, _this: Ptr, node: u32) -> u32 {
    e.call(NODE_PREVIOUS, &args![node]).u32()
}

// Translated from 005103a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the C string `needle` in the string at `this` from `position`
/// (`005103d0` with its length, `0044a670`).
pub fn fn_005103a0(e: &mut Engine, this: Ptr, needle: u32, position: u32) -> u32 {
    let length = e.call(C_STRING_LENGTH, &args![needle]).u32();
    fn_005103d0(e, this, needle, position, length)
}

// Translated from 005103d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the `length` characters at `needle` in the string at `this` (its
/// size at `+0x14`, its characters from `0044a970`) from `position`: an
/// empty needle is found at `position` when that is not past the end;
/// otherwise each occurrence of the needle's first character (`memchr`
/// over what is left) is compared (`memcmp`) until one matches, giving its
/// index; not found (or `position` at or past the end, or a needle longer
/// than what is left) gives the word at `010173f0` (`npos`).
pub fn fn_005103d0(e: &mut Engine, this: Ptr, needle: u32, position: u32, length: u32) -> u32 {
    let size = e.mem.u32(this.addr() + STRING_SIZE);
    if length == 0 && position <= size {
        return position;
    }
    if position < size {
        let mut left = size.wrapping_sub(position);
        if length <= left {
            left = left.wrapping_sub(length.wrapping_sub(1));
            let mut start = e
                .call(STRING_DATA, &args![this])
                .u32()
                .wrapping_add(position);
            loop {
                let found = fn_005104d0(e, start, left, needle);
                if found == 0 {
                    break;
                }
                if fn_005104b0(e, found, needle, length) == 0 {
                    let data = e.call(STRING_DATA, &args![this]).u32();
                    return found.wrapping_sub(data);
                }
                left = left.wrapping_sub(found.wrapping_sub(start).wrapping_add(1));
                start = found.wrapping_add(1);
            }
        }
    }
    e.global::<u32>(NOT_FOUND)
}

// Translated from 005104b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `memcmp(a, b, count)` (cdecl), its result passed through.
pub fn fn_005104b0(e: &mut Engine, a: u32, b: u32, count: u32) -> u32 {
    e.call(MEMCMP, &args![a, b, count]).u32()
}

// Translated from 005104d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `memchr(start, *character, count)` (cdecl; the character read as a
/// signed byte), its result passed through.
pub fn fn_005104d0(e: &mut Engine, start: u32, count: u32, character: u32) -> u32 {
    let value = e.mem.u8(character) as i8 as i32;
    e.call(MEMCHR, &args![start, value, count]).u32()
}

// Translated from 00510640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00510640 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `TESObjectACTI`'s destructor body (RTTI of its vtable `01029d5c`): its
/// six vtables stored again, then the components' destructors in reverse:
/// the string at `+0x88` (`004037d0`), `+0x68` (`00475380`), `+0x3C`
/// (`0048a490`), `+0x30` (`00591300`), and the base (`0050f740`).
pub fn fn_00510640(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_9d5c);
    e.mem.set_u32(base + 0x30, 0x0102_9d44);
    e.mem.set_u32(base + 0x3c, 0x0102_9d20);
    e.mem.set_u32(base + 0x5c, 0x0102_9d0c);
    e.mem.set_u32(base + 0x68, 0x0102_9cf8);
    e.mem.set_u32(base + 0x70, 0x0102_9ce8);
    e.call(0x0040_37d0, &args![base + 0x88]);
    e.call(0x0047_5380, &args![base + 0x68]);
    e.call(0x0048_a490, &args![base + 0x3c]);
    e.call(0x0059_1300, &args![base + 0x30]);
    e.call(0x0050_f740, &args![this]);
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005104f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_04f0,
            size: 0x90,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0048_cc50, &[A::T(0x5c)]),
                (0x0047_5350, &[A::T(0x68)]),
                (0x0051_0620, &[A::T(0x70)]),
                (0x0040_37b0, &[A::T(0x88)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x15)]),
                (0x0051_0710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_9d5c),
                (0x30, 4, 0x0102_9d44),
                (0x3c, 4, 0x0102_9d20),
                (0x5c, 4, 0x0102_9d0c),
                (0x68, 4, 0x0102_9cf8),
                (0x70, 4, 0x0102_9ce8),
            ],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test;

    #[test]
    fn fn_00510620_stores_the_vtable() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_0620,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[],
            stores: &[(0x0, 4, 0x0102_9ee0)],
        });
    }

    #[test]
    fn fn_00510710_zeroes_the_fields_and_sets_the_string() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xcd; 0x90]);
        e.register(0x0043_8390, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_0710, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0043_8390, vec![this + 0x88, 0x0101_1584])]);
        for offset in [0x74, 0x78, 0x7c, 0x80, 0x84] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x70), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 0x88), 0xcdcd_cdcd);
    }
    #[test]
    fn at_005105f0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_0640, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_05f0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_05f0)
                .collect();
            let mut expected = vec![(0x0051_0640, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b016_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    const REFERENCE: u32 = 0x0aaa_0000;

    fn model_engine() -> Engine {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                REFERENCE_TEST_444ED0,
                GET_7AF430,
                FORM_TYPE,
                NI_POINTER_CONSTRUCT,
                NI_POINTER_DESTRUCT,
                MODEL_LOADER_FIND_MODEL,
                MODEL_CALL_92C870,
                GET_SCALED_MODEL_FILE_NAME,
            ],
        );
        e.register(OBJECT_MODEL_PATH, |_, _| 0x0bad_0000u32.into_ret());
        e.register(TES_GET_LOD_MULT, |_, _| 0x4000_0000u32.into_ret());
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MODEL_NODE_43B230, |_, a| (a[0] + 1).into_ret());
        e.register(MODEL_LOADER_LOAD_FILE, |_, _| 0x0dd0_0000u32.into_ret());
        e.register(ALLOCATE_AA13E0, |_, _| 0x0eee_0000u32.into_ret());
        e.register(BS_FADE_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(REFERENCE_GET_SCALE, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.map(0x0101_2000, 0x1000);
        e.set_global(ONE, 1.0f64);
        e.map(0x011c_3000, 0x7000);
        e.set_global(MODEL_LOADER, 0x0ccc_0000u32);
        e
    }

    #[test]
    fn model_loads_the_file_without_a_reference_or_at_scale_1() {
        let mut e = model_engine();
        let this = e.mem.alloc(0x90);
        for reference in [0u32, REFERENCE] {
            e.call_log = Some(vec![]);
            let node = e.call(0x0050_fe50, &args![this, reference]).u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(node, 0x0dd0_0000);
            assert_eq!(
                calls(&log, MODEL_LOADER_LOAD_FILE),
                vec![vec![0x0ccc_0000, 0x0bad_0000, 0x4000_0000, 1, 0, 0, 0]]
            );
            assert!(calls(&log, GET_SCALED_MODEL_FILE_NAME).is_empty());
        }
    }

    #[test]
    fn model_fades_in_an_accepted_reference() {
        let mut e = model_engine();
        e.register(REFERENCE_TEST_444ED0, |_, _| 1u32.into_ret());
        e.register(MODEL_LOADER_FIND_MODEL, |e, a| {
            e.mem.set_u32(a[2], 0x0f0f_0000);
            Ret::default()
        });
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        let node = e.call(0x0050_fe50, &args![this, REFERENCE]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(node, 0x0eee_0000);
        assert_eq!(calls(&log, MODEL_CALL_92C870), vec![vec![0x0f0f_0000]]);
        assert_eq!(calls(&log, ALLOCATE_AA13E0), vec![vec![0xe4]]);
        assert!(calls(&log, TES_GET_LOD_MULT).is_empty());
        // A base form of type 0x1E takes the load path.
        e.register(FORM_TYPE, |_, _| 0x1eu32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_fe50, &args![this, REFERENCE]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, ALLOCATE_AA13E0).is_empty());
        assert_eq!(calls(&log, MODEL_LOADER_LOAD_FILE).len(), 1);
    }

    #[test]
    fn model_uses_a_found_scaled_model() {
        let mut e = model_engine();
        e.register(REFERENCE_GET_SCALE, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(MODEL_LOADER_FIND_MODEL, |e, a| {
            e.mem.set_u32(a[2], 0x0f0f_0000);
            Ret::default()
        });
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        let node = e.call(0x0050_fe50, &args![this, REFERENCE]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(node, 0x0f0f_0001);
        let name = calls(&log, GET_SCALED_MODEL_FILE_NAME)[0][2];
        assert_eq!(calls(&log, MODEL_LOADER_FIND_MODEL)[0][1], name);
        assert_eq!(e.global::<u32>(SCALED_MODEL_COUNT), 1);
        assert!(calls(&log, MODEL_LOADER_LOAD_FILE).is_empty());
        // NaN is not 1.0 either; nothing found: the file is loaded.
        e.register(REFERENCE_GET_SCALE, |_, _| Ret {
            st0: f64::NAN,
            ..Ret::default()
        });
        e.register(MODEL_LOADER_FIND_MODEL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_fe50, &args![this, REFERENCE]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GET_SCALED_MODEL_FILE_NAME).len(), 1);
        assert_eq!(calls(&log, MODEL_LOADER_LOAD_FILE).len(), 1);
    }

    /// A model of the node calls: each node is 0x10 bytes with its previous
    /// at `+0`, its next at `+4` and its list at `+8`.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_PREVIOUS, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_SET_PREVIOUS, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NODE_SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(NODE_SET_LIST, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(FORM_TYPE, |_, _| Ret::default());
        e.register(HOLDER_LIST, |_, a| a[0].into_ret());
        e.map(0x011c_3000, 0x1000);
        e
    }

    #[test]
    fn list_links_appends_and_unlinks() {
        let mut e = list_engine();
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list + 0xc, 7);
        let back = e.call(0x0051_00d0, &args![list, 0x99u32]).u32();
        assert_eq!(back, list);
        assert_eq!(e.mem.u32(list + 0xc), 0);
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        let c = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0051_02b0, &args![list, a]).u32(), a);
        e.call(0x0051_02b0, &args![list, c]);
        // b between a and c.
        e.call(0x0051_01e0, &args![list, a, b]);
        assert_eq!(e.mem.u32(list), 3);
        assert_eq!(e.mem.u32(list + 4), a);
        assert_eq!(e.mem.u32(list + 8), c);
        assert_eq!(e.mem.u32(a + 4), b);
        assert_eq!(e.mem.u32(b), a);
        assert_eq!(e.mem.u32(b + 4), c);
        assert_eq!(e.mem.u32(c), b);
        assert_eq!(e.mem.u32(b + 8), list);
        // Unlink b, then the tail c.
        e.call(0x0051_0130, &args![list, b]);
        assert_eq!(e.mem.u32(a + 4), c);
        assert_eq!(e.mem.u32(c), a);
        assert_eq!(e.mem.u32(b + 8), 0);
        e.call(0x0051_0130, &args![list, c]);
        assert_eq!(e.mem.u32(list + 4), a);
        assert_eq!(e.mem.u32(list + 8), a);
        assert_eq!(e.mem.u32(list), 1);
        assert_eq!(e.call(0x0051_0360, &args![list, a]).u32(), 0);
        assert_eq!(e.call(0x0051_0380, &args![list, c]).u32(), a);
        // The holder's list: 00401170 runs on the node; without a previous
        // node it is the head and (its next cleared) the tail.
        let other = e.mem.alloc(0x10);
        e.set_global(LIST_HOLDER, other);
        let d = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0051_01e0, &args![other, 0u32, d]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FORM_TYPE), vec![vec![d]]);
        assert_eq!(e.mem.u32(other + 4), d);
        assert_eq!(e.mem.u32(other + 8), d);
        assert_eq!(e.mem.u32(other), 1);
    }

    #[test]
    fn list_clear_deletes_from_the_tail() {
        let mut e = list_engine();
        e.register(0x0012_3410, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x14);
        e.mem.set_u32(vtable + 0x10, 0x0012_3410);
        let list = e.mem.alloc(0x10);
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        e.call(0x0051_02b0, &args![list, a]);
        e.call(0x0051_02b0, &args![list, b]);
        // The nodes' first word becomes their vtable: from here on the
        // previous node is kept at `+0xC`.
        e.register(NODE_PREVIOUS, |e, w| e.mem.u32(w[0] + 0xc).into_ret());
        e.mem.set_u32(b + 0xc, a);
        e.mem.set_u32(a, vtable);
        e.mem.set_u32(b, vtable);
        e.call_log = Some(vec![]);
        e.call(0x0051_0110, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0012_3410), vec![vec![b, 1], vec![a, 1]]);
        for offset in [0u32, 4, 8] {
            assert_eq!(e.mem.u32(list + offset), 0);
        }
    }

    fn string_engine(text: &[u8]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(STRING_DATA, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(C_STRING_LENGTH, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        e.register(MEMCMP, |e, a| {
            let x = e.mem.bytes(a[0], a[2]);
            let y = e.mem.bytes(a[1], a[2]);
            ((x != y) as u32).into_ret()
        });
        e.register(MEMCHR, |e, a| {
            let wanted = a[1] as u8;
            for i in 0..a[2] {
                if e.mem.u8(a[0] + i) == wanted {
                    return (a[0] + i).into_ret();
                }
            }
            Ret::default()
        });
        e.map(0x0101_7000, 0x1000);
        e.set_global(NOT_FOUND, u32::MAX);
        let chars = e.mem.alloc(0x40);
        e.mem.write(chars, text);
        let string = e.mem.alloc(0x1c);
        e.mem.set_u32(string + 4, chars);
        e.mem.set_u32(string + STRING_SIZE, text.len() as u32);
        (e, string)
    }

    #[test]
    fn find_locates_a_substring() {
        let (mut e, string) = string_engine(b"meshes\\armor\\arm.nif");
        let needle = e.mem.alloc(8);
        e.mem.write(needle, b"arm.\0");
        assert_eq!(e.call(0x0051_03a0, &args![string, needle, 0u32]).u32(), 13);
        e.mem.write(needle, b"\\a\0");
        assert_eq!(e.call(0x0051_03a0, &args![string, needle, 0u32]).u32(), 6);
        assert_eq!(e.call(0x0051_03a0, &args![string, needle, 7u32]).u32(), 12);
        e.mem.write(needle, b"xyz\0");
        assert_eq!(
            e.call(0x0051_03a0, &args![string, needle, 0u32]).u32(),
            u32::MAX
        );
        // An empty needle: the position, unless past the end.
        assert_eq!(
            e.call(0x0051_03d0, &args![string, needle, 20u32, 0u32])
                .u32(),
            20
        );
        assert_eq!(
            e.call(0x0051_03d0, &args![string, needle, 21u32, 0u32])
                .u32(),
            u32::MAX
        );
        // A needle longer than what is left.
        assert_eq!(
            e.call(0x0051_03d0, &args![string, needle, 18u32, 3u32])
                .u32(),
            u32::MAX
        );
    }

    #[test]
    fn memory_helpers_pass_their_arguments() {
        let mut e = Engine::new();
        e.register(MEMCMP, |_, a| (a[0] + a[1] + a[2]).into_ret());
        e.register(MEMCHR, |_, a| (a[1]).into_ret());
        assert_eq!(e.call(0x0051_04b0, &args![1u32, 2u32, 3u32]).u32(), 6);
        let character = e.mem.alloc(4);
        e.mem.set_u32(character, 0xff);
        // The character is sign-extended.
        assert_eq!(
            e.call(0x0051_04d0, &args![0x10u32, 4u32, character]).u32(),
            u32::MAX
        );
    }

    #[test]
    fn activator_destructor_runs_the_component_destructors() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                0x0040_37d0,
                0x0047_5380,
                0x0048_a490,
                0x0059_1300,
                0x0050_f740,
            ],
        );
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x0051_0640, &args![this]);
        let log = e.call_log.take().unwrap();
        let order: Vec<_> = log[1..].iter().map(|(a, w)| (*a, w[0])).collect();
        assert_eq!(
            order,
            vec![
                (0x0040_37d0, this + 0x88),
                (0x0047_5380, this + 0x68),
                (0x0048_a490, this + 0x3c),
                (0x0059_1300, this + 0x30),
                (0x0050_f740, this),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_9d5c);
        assert_eq!(e.mem.u32(this + 0x70), 0x0102_9ce8);
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_00510080_updates_the_node_and_sets_a_zero_point() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x200);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x174, 0x0090_0174);
        e.mem.set_u32(this + 4, node);
        e.register(0x0090_0174, |e, a| {
            assert_eq!(a[1], 0);
            Ret {
                eax: e.mem.u32(a[0] + 4),
                ..Ret::default()
            }
        });
        e.register(0x00a5_a040, |_, _| Ret::default());
        e.register(0x0043_d410, |_, _| Ret::default());
        e.register(0x00a5_9c60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_0080, &[this]).bool());
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| [0x00a5_a040, 0x0043_d410, 0x00a5_9c60].contains(a))
            .collect();
        assert_eq!(log.len(), 3);
        assert_eq!(log[0], (0x00a5_a040, vec![node]));
        let point = log[1].1[0];
        assert_eq!(log[1].1[1..], [0, 0, 0]);
        assert_eq!(log[2], (0x00a5_9c60, vec![node, point]));
    }
}
