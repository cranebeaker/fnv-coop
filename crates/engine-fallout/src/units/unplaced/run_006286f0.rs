//! `(unplaced)/run_006286f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004a3c90` (thiscall, source): copies four words (a vector).
const VECTOR_COPY: u32 = 0x004a_3c90;
/// `00458b20`: the address of the vector at `01267e30`.
const VECTOR_AT_01267E30: u32 = 0x0045_8b20;
/// `004ae750` (thiscall): null for a null `this`, otherwise `00620b80`'s
/// result (engine map name `bhkCharacterProxy::operator*`, unconfirmed).
const OBJECT_PROXY: u32 = 0x004a_e750;
/// `00a29680`: an empty function (a lone `RET`).
const EMPTY_FUNCTION: u32 = 0x00a2_9680;
/// `00984230` (thiscall): `this + 0xb0`.
const BOUNDS_AT_B0: u32 = 0x0098_4230;
/// `006281a0` (thiscall, source): copies the two vectors of a box (min at
/// +0, max at +0x10, `004a3f10` each).
const BOX_COPY: u32 = 0x0062_81a0;
/// `00458a60` (thiscall: out mask, other): per-lane `this <= other` mask.
const LANES_LESS_EQUAL: u32 = 0x0045_8a60;
/// `00458990` (thiscall, lane bits): whether every lane in the bits is set
/// in the mask (`MOVMSKPS`).
const MASK_ALL_SET: u32 = 0x0045_8990;
/// `00622570` (thiscall, byte): stores the byte at `this` and returns
/// `this`.
const STORE_BYTE: u32 = 0x0062_2570;
/// `00437c10` (thiscall): the box constructor (two vector constructors).
const BOX_CONSTRUCT: u32 = 0x0043_7c10;
/// `006815c0` (thiscall): the vector constructor (returns `this`).
const VECTOR_CONSTRUCT: u32 = 0x0068_15c0;
/// `004589c0` (thiscall, float): sets all four lanes to the float.
const VECTOR_SPLAT: u32 = 0x0045_89c0;
/// `00628640` (thiscall: a, b): lane-wise maximum of two vectors.
const VECTOR_MAX: u32 = 0x0062_8640;
/// `006285e0` (thiscall: a, b): lane-wise minimum of two vectors.
const VECTOR_MIN: u32 = 0x0062_85e0;
/// `004a3f10` (thiscall, source): copies a vector.
const VECTOR_SET: u32 = 0x004a_3f10;
/// `004543c0` (thiscall): the cell's object (`0041b9a0` or `00451010`).
const CELL_OBJECT: u32 = 0x0045_43c0;
/// `00541c80` (thiscall): null for a null `this`, otherwise `00541cb0`.
const CELL_OBJECT_DATA: u32 = 0x0054_1c80;
/// The float `004589c0` sets the maximum to before the loop (`0104e148`).
const LOWEST_FLOAT: u32 = 0x0104_e148;
/// The float `004589c0` sets the minimum to before the loop (`0104da48`).
const HIGHEST_FLOAT: u32 = 0x0104_da48;
/// `00726070` (thiscall): the word at +4 (an array's count).
const ARRAY_COUNT: u32 = 0x0072_6070;
/// `00629ab0` (thiscall, index): the address of element `index` (data at
/// +0, four bytes each).
const ARRAY_ELEMENT: u32 = 0x0062_9ab0;
/// `00460140` (thiscall): `this + 0x10`.
const PLUS_10: u32 = 0x0046_0140;
/// `00559450` (thiscall): the word at +0.
const WORD_AT_0: u32 = 0x0055_9450;
/// `006286d0` (thiscall): the word at +0x58 (engine map name
/// `LowProcess::GetNumberOfItemsActivated`, a folded body).
const WORD_AT_58: u32 = 0x0062_86d0;
/// `007d6bb0` (thiscall): `this + 0x14`.
const PLUS_14: u32 = 0x007d_6bb0;
/// `005609f0` (thiscall): whether the byte at `this` is non-zero.
const BYTE_SET: u32 = 0x0056_09f0;
/// `0043b540` (thiscall): a word of the object (passed to `008c71b0`).
const OBJECT_TYPE_WORD: u32 = 0x0043_b540;
/// `008c71b0` (thiscall, word): stores the word at +0.
const STORE_WORD: u32 = 0x008c_71b0;
/// `0043b4d0` (thiscall): reads the stored word's type.
const WORD_TYPE: u32 = 0x0043_b4d0;
/// `0062b4e0` `TESHavokUtilities::FindCollidableRef` (Xbox PDB name in the
/// engine map; cdecl, one word).
const FIND_COLLIDABLE_REF: u32 = 0x0062_b4e0;
/// `006295e0` `LoadedAreaBound::WarpActor` (Xbox PDB name in the engine
/// map; thiscall, reference).
const WARP_ACTOR: u32 = 0x0062_95e0;
/// `009611e0` (thiscall): the element's actor word.
const ELEMENT_ACTOR: u32 = 0x0096_11e0;
/// `00624700` (thiscall): the element's inner array.
const ELEMENT_INNER_ARRAY: u32 = 0x0062_4700;
/// `004b59f0` (cdecl, one word): the reference the object belongs to.
const OBJECT_REFERENCE: u32 = 0x004b_59f0;
/// `00517670` (thiscall): a kind number of the reference (4, 5 and 7 are
/// skipped).
const REFERENCE_KIND: u32 = 0x0051_7670;
/// `006299a0` (thiscall): the local array constructor.
const LIST_CONSTRUCT: u32 = 0x0062_99a0;
/// `006299c0` (thiscall, element address): appends the word at the address.
const LIST_APPEND: u32 = 0x0062_99c0;
/// `00629f10` (thiscall: element address, start, end): the index of the
/// element, or -1.
const LIST_FIND: u32 = 0x0062_9f10;
/// `00629b40` (thiscall): the local array destructor.
const LIST_DESTRUCT: u32 = 0x0062_9b40;
/// `004b5a80` (cdecl, one word).
const REFERENCE_KEY: u32 = 0x004b_5a80;
/// `00628f90` (thiscall: key, out array).
const COLLECT_FOR_KEY: u32 = 0x0062_8f90;

/// A box on the stack: min at +0, max at +0x10 (0x20 bytes).
const BOX_SIZE: u32 = 0x20;
/// A local array (data, count, ...), 0xc bytes.
const LIST_SIZE: u32 = 0xc;

// Translated from 00628da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the word at +0x20, assigns null to the
/// six `NiPointer`s at +0x08..+0x1C (`0066b0d0`) and tears the member at
/// +0x24 down (`00438af0`).
pub fn fn_00628da0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t + 0x20, 0);
    for index in 0..6u32 {
        e.call(0x0066_b0d0, &args![t + index * 4 + 8, 0u32]);
    }
    e.call(0x0043_8af0, &args![t + 0x24]);
}

// Translated from 00628950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a box from two vectors: copies `min` to +0 and
/// `max` to +0x10 (`004a3c90`). Returns `this`.
pub fn fn_00628950(e: &mut Engine, this: Ptr, min: Ptr, max: Ptr) -> Ptr {
    e.call(VECTOR_COPY, &args![this, min]);
    e.call(VECTOR_COPY, &args![this.byte_add(0x10), max]);
    this
}

// Translated from 006288a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `this + 0x110`.
pub fn fn_006288a0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x110)
}

// Translated from 006288c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: writes the object's box to `out`: when
/// `004ae750` gives a proxy, its box at +0xB0 (between two calls of the
/// empty `00a29680`); otherwise a box made of the vector at `01267e30`
/// twice.
// Translation incomplete: 006288c0 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_006288c0(e: &mut Engine, this: Ptr, out: Ptr) {
    let proxy = e.call(OBJECT_PROXY, &args![this]).u32();
    if proxy != 0 {
        e.call(EMPTY_FUNCTION, &args![this]);
        let bounds = e.call(BOUNDS_AT_B0, &args![proxy]).u32();
        e.call(BOX_COPY, &args![out, bounds]);
        e.call(EMPTY_FUNCTION, &args![this]);
    } else {
        e.with_stack(BOX_SIZE, |e, local| {
            let first = e.call(VECTOR_AT_01267E30, &[]).u32();
            let second = e.call(VECTOR_AT_01267E30, &[]).u32();
            let made = fn_00628950(e, local, Ptr::new(second), Ptr::new(first));
            e.call(BOX_COPY, &args![out, made]);
        });
    }
}

// Translated from 00628cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `this = a & b`, four words (`ANDPS`).
// Translation incomplete: 00628cc0 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00628cc0(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let mut lanes = [0u32; 4];
    for (lane, value) in lanes.iter_mut().enumerate() {
        let offset = lane as u32 * 4;
        *value =
            e.mem.u32(a.addr().wrapping_add(offset)) & e.mem.u32(b.addr().wrapping_add(offset));
    }
    for (lane, value) in lanes.iter().enumerate() {
        e.mem
            .set_u32(this.addr().wrapping_add(lane as u32 * 4), *value);
    }
}

// Translated from 00628c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the boxes `this` and `other` overlap
/// on x, y and z (`this.min <= other.max` and `other.min <= this.max` in
/// lanes 0-2, `00458990(7)`); returns that callee's `EAX`.
// Translation incomplete: 00628c50 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00628c50(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    e.with_stack(0x30, |e, locals| {
        let low = locals;
        let high = locals.byte_add(0x10);
        let both = locals.byte_add(0x20);
        e.call(LANES_LESS_EQUAL, &args![this, low, other.byte_add(0x10)]);
        e.call(LANES_LESS_EQUAL, &args![other, high, this.byte_add(0x10)]);
        fn_00628cc0(e, both, low, high);
        e.call(MASK_ALL_SET, &args![both, 7u32]).u32()
    })
}

// Translated from 00628d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the box `this` contains the box
/// `other` on x, y and z (`this.min <= other.min` and `other.max <=
/// this.max`, `00458990(7)`), stored as a byte at `out` (`00622570`).
/// Returns `out`.
// Translation incomplete: 00628d10 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00628d10(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    let inside = e.with_stack(0x30, |e, locals| {
        let low = locals;
        let high = locals.byte_add(0x10);
        let both = locals.byte_add(0x20);
        e.call(LANES_LESS_EQUAL, &args![this, low, other]);
        e.call(
            LANES_LESS_EQUAL,
            &args![other.byte_add(0x10), high, this.byte_add(0x10)],
        );
        fn_00628cc0(e, both, low, high);
        e.call(MASK_ALL_SET, &args![both, 7u32]).u32() != 0
    });
    e.call(STORE_BYTE, &args![out, inside as u32]);
    out
}

/// The test `00628df0` and `00628980` make for an object's box (`object +
/// 0x14`, filled through the actor word's object's vtable +0x3C): true when
/// `bounds` contains it (`00628d10`) and overlaps it (`00628c50`).
fn object_inside(e: &mut Engine, bounds: Ptr, object: u32, owner: u32, object_box: Ptr) -> bool {
    let target = e.call(WORD_AT_58, &args![owner]).u32();
    let box_source = e.call(PLUS_14, &args![object]).u32();
    e.vcall(target, 0x3c, &args![box_source, object_box]);
    let inside = e.with_stack(4, |e, flag| {
        let result = fn_00628d10(e, bounds, flag, object_box);
        e.call(BYTE_SET, &args![result]).bool()
    });
    inside && fn_00628c50(e, bounds, object_box) != 0
}

// Translated from 00628df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: for each object of the array `objects` (its
/// element +0x10) whose word at +0 is set and whose box is not inside
/// `bounds` (see `object_inside`): when its type word (`0043b540` stored
/// through `008c71b0`, read by `0043b4d0`) is 0x1E and it has a collidable
/// reference (`TESHavokUtilities::FindCollidableRef`) for which vtable
/// +0x100 answers true, warps that reference (`LoadedAreaBound::WarpActor`).
// Translation incomplete: 00628df0 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00628df0(e: &mut Engine, this: Ptr, bounds: Ptr, objects: Ptr, owner: u32) {
    e.with_stack(BOX_SIZE + 4, |e, locals| {
        let object_box = locals;
        let type_word = locals.byte_add(BOX_SIZE);
        e.call(BOX_CONSTRUCT, &args![object_box]);
        let mut index = 0i32;
        while index < e.call(ARRAY_COUNT, &args![objects]).i32() {
            let element = e.call(ARRAY_ELEMENT, &args![objects, index]).u32();
            let item = e.mem.u32(element);
            let object = e.call(PLUS_10, &args![item]).u32();
            if e.call(WORD_AT_0, &args![object]).u32() != 0
                && !object_inside(e, bounds, object, owner, object_box)
            {
                let word = e.call(OBJECT_TYPE_WORD, &args![object]).u32();
                e.call(STORE_WORD, &args![type_word, word]);
                if e.call(WORD_TYPE, &args![type_word]).u32() == 0x1e {
                    let reference = e.call(FIND_COLLIDABLE_REF, &args![object]).u32();
                    if reference != 0 && e.vcall(reference, 0x100, &[]).bool() {
                        e.call(WARP_ACTOR, &args![this, reference]);
                    }
                }
            }
            index += 1;
        }
    });
}

// Translated from 00628980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: for each element of `groups` (its actor word
/// `009611e0` and inner array `00624700`), gathers the references
/// (`004b59f0`) of the inner array's objects (element +0x10, word at +0 set)
/// whose box is not inside `bounds` and whose kind (`00517670`) is not 4, 5
/// or 7, into a local array; then, for each gathered reference not yet in a
/// second local array (`00629f10` gives -1), collects the references for its
/// key (`004b5a80`, `00628f90`) into a third array and appends them all to
/// the second. The three arrays are destroyed with `00629b40`.
// Translation incomplete: 00628980 - SEH exception unwinding and the /GS stack-cookie check (00ec408c) are not translated.
pub fn fn_00628980(e: &mut Engine, this: Ptr, bounds: Ptr, groups: Ptr) {
    e.with_stack(BOX_SIZE + 4 + 4 + LIST_SIZE * 3, |e, locals| {
        let object_box = locals;
        let reference_slot = locals.byte_add(BOX_SIZE).addr();
        let gathered = locals.byte_add(BOX_SIZE + 8);
        let done = locals.byte_add(BOX_SIZE + 8 + LIST_SIZE);
        let for_key = locals.byte_add(BOX_SIZE + 8 + 2 * LIST_SIZE);
        e.call(BOX_CONSTRUCT, &args![object_box]);
        e.call(LIST_CONSTRUCT, &args![gathered]);
        let mut group = 0i32;
        while group < e.call(ARRAY_COUNT, &args![groups]).i32() {
            let element = e.call(ARRAY_ELEMENT, &args![groups, group]).u32();
            let item = e.mem.u32(element);
            let actor = e.call(ELEMENT_ACTOR, &args![item]).u32();
            let element = e.call(ARRAY_ELEMENT, &args![groups, group]).u32();
            let item = e.mem.u32(element);
            let inner = e.call(ELEMENT_INNER_ARRAY, &args![item]).u32();
            let mut index = 0i32;
            while index < e.call(ARRAY_COUNT, &args![inner]).i32() {
                let element = e.call(ARRAY_ELEMENT, &args![inner, index]).u32();
                let item = e.mem.u32(element);
                let object = e.call(PLUS_10, &args![item]).u32();
                if e.call(WORD_AT_0, &args![object]).u32() != 0
                    && !object_inside(e, bounds, object, actor, object_box)
                {
                    let reference = e.call(OBJECT_REFERENCE, &args![object]).u32();
                    e.mem.set_u32(reference_slot, reference);
                    if reference != 0
                        && e.call(REFERENCE_KIND, &args![reference]).u32() != 4
                        && e.call(REFERENCE_KIND, &args![reference]).u32() != 5
                        && e.call(REFERENCE_KIND, &args![reference]).u32() != 7
                    {
                        e.call(LIST_APPEND, &args![gathered, reference_slot]);
                    }
                }
                index += 1;
            }
            group += 1;
        }
        e.call(LIST_CONSTRUCT, &args![done]);
        let mut index = 0i32;
        while index < e.call(ARRAY_COUNT, &args![gathered]).i32() {
            let element = e.call(ARRAY_ELEMENT, &args![gathered, index]).u32();
            let found = e
                .call(LIST_FIND, &args![done, element, 0u32, 0xffff_ffffu32])
                .i32();
            if found == -1 {
                let element = e.call(ARRAY_ELEMENT, &args![gathered, index]).u32();
                let reference = e.mem.u32(element);
                let key = e.call(REFERENCE_KEY, &args![reference]).u32();
                e.call(LIST_CONSTRUCT, &args![for_key]);
                e.call(COLLECT_FOR_KEY, &args![this, key, for_key]);
                let mut added = 0i32;
                while added < e.call(ARRAY_COUNT, &args![for_key]).i32() {
                    let element = e.call(ARRAY_ELEMENT, &args![for_key, added]).u32();
                    e.call(LIST_APPEND, &args![done, element]);
                    added += 1;
                }
                e.call(LIST_DESTRUCT, &args![for_key]);
            }
            index += 1;
        }
        e.call(LIST_DESTRUCT, &args![done]);
        e.call(LIST_DESTRUCT, &args![gathered]);
    });
}

// Translated from 006286f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: computes the box around the six objects at
/// +0x08..+0x1C (each one's box from `006288c0`, starting from max = the
/// float at `0104e148` and min = the float at `0104da48` in every lane);
/// then, when the cell at +0x20 has an object (`004543c0`, `00541c80`),
/// checks its objects at +0x110 against the box (`00628df0`) and its two
/// arrays at +0x28 and +0x34 (`00628980`).
// Translation incomplete: 006286f0 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_006286f0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    // Locals: the box (+0), max (+0x20), min (+0x30), an object's box (+0x40).
    e.with_stack(0x60, |e, locals| {
        let bounds = locals;
        let max = locals.byte_add(0x20);
        let min = locals.byte_add(0x30);
        let object_box = locals.byte_add(0x40);
        e.call(BOX_CONSTRUCT, &args![bounds]);
        e.call(VECTOR_CONSTRUCT, &args![min]);
        e.call(VECTOR_CONSTRUCT, &args![max]);
        let lowest = x87_quiet_bits(e.global::<u32>(LOWEST_FLOAT));
        e.call(VECTOR_SPLAT, &args![max, lowest]);
        let highest = x87_quiet_bits(e.global::<u32>(HIGHEST_FLOAT));
        e.call(VECTOR_SPLAT, &args![min, highest]);
        for index in 0..6u32 {
            e.call(BOX_CONSTRUCT, &args![object_box]);
            let object = e.call(WORD_AT_0, &args![t + index * 4 + 8]).u32();
            fn_006288c0(e, Ptr::new(object), object_box);
            e.call(VECTOR_MAX, &args![max, max, object_box.byte_add(0x10)]);
            e.call(VECTOR_MIN, &args![min, min, object_box]);
        }
        e.call(VECTOR_SET, &args![bounds, min]);
        e.call(VECTOR_SET, &args![bounds.byte_add(0x10), max]);
        let cell = e.mem.u32(t + 0x20);
        let cell_object = e.call(CELL_OBJECT, &args![cell]).u32();
        let data = if cell_object != 0 {
            e.call(CELL_OBJECT_DATA, &args![cell_object]).u32()
        } else {
            0
        };
        if data != 0 {
            let first_array = data + 0x28;
            let second_array = data + 0x34;
            let objects = fn_006288a0(e, Ptr::new(data));
            fn_00628df0(e, this, bounds, objects, data);
            fn_00628980(e, this, bounds, Ptr::new(first_array));
            fn_00628980(e, this, bounds, Ptr::new(second_array));
        }
    });
}

/// The callees of [`fn_00628f90`] not named above, each by what it is given (none is
/// confirmed beyond the engine map's unit or name):
/// `005631e0` (thiscall on the reference, one word: 0x20; answers a byte),
/// `0043b560` (thiscall on the reference) whose answer goes to `0062b4e0`,
/// `0056ded0` (thiscall: a 0x48-byte local's constructor), the reference's vtable slot
/// +0xF4 (one word: that local), `00476a80` (thiscall: a 0x34-byte local's constructor),
/// `00624ad0` (cdecl: that local and the first), `LoadedAreaBound::FindWarpLocation`
/// (`006296d0`; thiscall: the collidable reference and two vector addresses),
/// `00439ef0` and `00439e90` (thiscall on a vector: the result's address and another
/// vector), `00620b80` (thiscall on the reference), `00c9c380` (thiscall: a count) and
/// `00c9c390` (thiscall, one word: an index), `0084e3a0` (thiscall), slot +0x20 of what it
/// answers (no words), `00c809e0` (cdecl: the entity and the local array), `006295a0`
/// (thiscall on a box), the reference's slot +0x94 (no words), slot +0x3C of what
/// `006286d0` answers (two words), `00629560` (thiscall on a box, one word),
/// `004a51b0` and `00560d30` (thiscall, one word: 2; answer a component's address),
/// `005610f0`, `00561690` and `005617c0` (thiscall on a reference, one word),
/// `005532a0` (thiscall on a local, four floats), the collidable reference's slots +0x100
/// (no words, a byte) and +0x1D0 (no words), `00931ed0` (thiscall, one word),
/// `00ca2ad0` (thiscall on the word at +0xB0 of the collidable reference, two words),
/// `bhkRigidBody::GetAabbLocal` (`00c8d390`; thiscall on the reference, one word),
/// `004587d0` (cdecl, a float; answers in `ST0`) and `00408840` (cdecl, a float; answers in
/// `ST0`).
const REFERENCE_FLAG_TEST: u32 = 0x0056_31e0;
const REFERENCE_MODEL_WORD: u32 = 0x0043_b560;
const LOCAL_48_CONSTRUCT: u32 = 0x0056_ded0;
const LOCAL_34_CONSTRUCT: u32 = 0x0047_6a80;
const LOCAL_FROM_48: u32 = 0x0062_4ad0;
const FIND_WARP_LOCATION: u32 = 0x0062_96d0;
const POINT_OPERATION_00439EF0: u32 = 0x0043_9ef0;
const POINT_OPERATION_00439E90: u32 = 0x0043_9e90;
const REFERENCE_ENTITY: u32 = 0x0062_0b80;
const ENTITY_COUNT: u32 = 0x00c9_c380;
const ENTITY_AT: u32 = 0x00c9_c390;
const ENTITY_OBJECT: u32 = 0x0084_e3a0;
const ENTITY_COLLECT: u32 = 0x00c8_09e0;
const BOX_RESET: u32 = 0x0062_95a0;
const BOX_ADD: u32 = 0x0062_9560;
const VECTOR_COMPONENT_004A51B0: u32 = 0x004a_51b0;
const VECTOR_COMPONENT_00560D30: u32 = 0x0056_0d30;
const REFERENCE_SET_POSITION: u32 = 0x0056_10f0;
const REFERENCE_USE_005617C0: u32 = 0x0056_17c0;
const REFERENCE_USE_00561690: u32 = 0x0056_1690;
const LOCAL_FROM_FLOATS: u32 = 0x0055_32a0;
const WORD_GETTER_00931ED0: u32 = 0x0093_1ed0;
const COLLIDABLE_UPDATE: u32 = 0x00ca_2ad0;
const GET_AABB_LOCAL: u32 = 0x00c8_d390;
const FLOAT_FUNCTION_004587D0: u32 = 0x0045_87d0;
const FLOAT_FUNCTION_00408840: u32 = 0x0040_8840;
/// The `float` `005532a0` is given as its third word (`01013ea4`, through `FLD`/`FSTP`).
const SMALL_CONSTANT_01013EA4: u32 = 0x0101_3ea4;
/// The kind slot +0x20 answers that does not count as found.
const KIND_NOT_FOUND: u32 = 0xb;

// Translated from 00628f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00628f90 - the FS:[0] exception frame (unwind state 0 for the local array) and the /GS stack-cookie check (00ec408c) are not translated
/// Unnamed in the engine map (`thiscall`, `RET 8`: a reference and an array to add to).
/// Nothing for a null reference or when `005631e0(reference, 0x20)` answers non-zero.
/// Otherwise: the collidable reference (`TESHavokUtilities::FindCollidableRef` of
/// `0043b560`'s answer), the reference's data (slot +0xF4 into a local, `00476a80`,
/// `00624ad0`), a warp location (`LoadedAreaBound::FindWarpLocation` on `this` into a second
/// vector) and the offset between them (`00439ef0`). It then walks the entity `00620b80`
/// answers (`00c9c380` items, `00c9c390`, `0084e3a0`) until one's slot +0x20 answers other
/// than 0xB.
///
/// When one does: the entity's collected items (`00c809e0` into a local array) are boxed
/// (`006295a0`, then for each item the box `006286d0`'s slot +0x3C fills, added twice with
/// `00629560`), the offset's z grows by the box's height (`max.z - min.z`, x87), and each
/// item's reference (`004b5a80`) is moved by the offset (`00439e90` on its data, then
/// `005610f0`) and given two zero-filled locals (`005532a0` with 0, 0, the float at
/// `01013ea4`, 0 and with zeros; `00561690`, `005617c0`), an item whose reference is not
/// `reference` first appended to `out` (`006299c0`). Then, when the collidable reference's
/// slot +0x100 answers a byte and its word at +0xB0 is set, `00ca2ad0` on that word with
/// slot +0x1D0's answer and `00559450` of `00931ed0`'s. Last, the local array destroyed.
///
/// When none does: the reference's box (`bhkRigidBody::GetAabbLocal`), and the reference
/// moved to the warp location raised by `004587d0(max.z) + 00408840(004587d0(min.z))`
/// (x87, the first sum in double precision), with `005532a0` and `00561690` as above.
pub fn fn_00628f90(e: &mut Engine, this: Ptr, reference: u32, out: u32) {
    if reference == 0 {
        return;
    }
    if e.call(REFERENCE_FLAG_TEST, &args![reference, 0x20u32]).u8() != 0 {
        return;
    }
    // The frame: a local at [EBP - n] is at `base + 0x250 - n`.
    e.with_stack(0x250, |e, frame| {
        let at = |n: u32| frame.addr() + 0x250 - n;
        let model = e.call(REFERENCE_MODEL_WORD, &args![reference]).u32();
        let collidable = e.call(FIND_COLLIDABLE_REF, &args![model]).u32();
        e.call(VECTOR_CONSTRUCT, &args![at(0x28)]);
        e.call(LOCAL_48_CONSTRUCT, &args![at(0x70)]);
        e.vcall(reference, 0xf4, &args![at(0x70)]);
        e.call(LOCAL_34_CONSTRUCT, &args![at(0xa4)]);
        e.call(LOCAL_FROM_48, &args![at(0xa4), at(0x70)]);
        e.call(
            FIND_WARP_LOCATION,
            &args![this, collidable, at(0x80), at(0x28)],
        );
        let mut found = false;
        e.call(
            POINT_OPERATION_00439EF0,
            &args![at(0x28), at(0xb4), at(0x80)],
        );
        let entity = e.call(REFERENCE_ENTITY, &args![reference]).u32();
        let mut index = 0u32;
        while (index as i32) < e.call(ENTITY_COUNT, &args![entity]).i32() && !found {
            let item = e.call(ENTITY_AT, &args![entity, index]).u32();
            let object = e.call(ENTITY_OBJECT, &args![item]).u32();
            let kind = e.vcall(object, 0x20, &args![]).u32();
            found = kind != KIND_NOT_FOUND;
            index = index.wrapping_add(1);
        }
        if found {
            let items = at(0xcc);
            e.call(LIST_CONSTRUCT, &args![items]);
            e.call(ENTITY_COLLECT, &args![entity, items]);
            let bounds = at(0xf0);
            e.call(BOX_CONSTRUCT, &args![bounds]);
            e.call(BOX_RESET, &args![bounds]);
            let data = e.vcall(reference, 0x94, &args![]).u32();
            let mut item = 0u32;
            while (item as i32) < e.call(ARRAY_COUNT, &args![items]).i32() {
                e.call(BOX_CONSTRUCT, &args![at(0x120)]);
                let filler = e.call(WORD_AT_58, &args![data]).u32();
                let element = e.call(ARRAY_ELEMENT, &args![items, item]).u32();
                let word = e.mem.u32(element);
                let inner = e.call(PLUS_10, &args![word]).u32();
                let source = e.call(PLUS_14, &args![inner]).u32();
                e.vcall(filler, 0x3c, &args![source, at(0x120)]);
                e.call(BOX_ADD, &args![bounds, at(0x110)]);
                e.call(BOX_ADD, &args![bounds, at(0x120)]);
                item = item.wrapping_add(1);
            }
            let offset_z = e
                .call(VECTOR_COMPONENT_004A51B0, &args![at(0xb4), 2u32])
                .u32();
            let top = e
                .call(VECTOR_COMPONENT_00560D30, &args![at(0xe0), 2u32])
                .u32();
            let bottom = e
                .call(VECTOR_COMPONENT_00560D30, &args![bounds, 2u32])
                .u32();
            let height = x87_sub(x87_load32(e.mem.u32(top)), x87_m32(e.mem.u32(bottom)));
            let raised = x87_add(height, x87_m32(e.mem.u32(offset_z)));
            e.mem.set_u32(offset_z, x87_store32(raised));
            let mut item = 0u32;
            while (item as i32) < e.call(ARRAY_COUNT, &args![items]).i32() {
                let position = at(0x138);
                e.call(VECTOR_CONSTRUCT, &args![position]);
                e.call(LOCAL_48_CONSTRUCT, &args![at(0x180)]);
                e.vcall(reference, 0xf4, &args![at(0x180)]);
                e.call(LOCAL_34_CONSTRUCT, &args![at(0x1b4)]);
                e.call(LOCAL_FROM_48, &args![at(0x1b4), at(0x180)]);
                let moved = e
                    .call(
                        POINT_OPERATION_00439E90,
                        &args![at(0x190), at(0x1c0), at(0xb4)],
                    )
                    .u32();
                for word in 0..3u32 {
                    let value = e.mem.u32(moved.wrapping_add(4 * word));
                    e.mem.set_u32(position + 4 * word, value);
                }
                let element = e.call(ARRAY_ELEMENT, &args![items, item]).u32();
                let word = e.mem.u32(element);
                let actor = e.call(REFERENCE_KEY, &args![word]).u32();
                if reference != actor {
                    let element = e.call(ARRAY_ELEMENT, &args![items, item]).u32();
                    e.call(LIST_APPEND, &args![out, element]);
                }
                e.call(REFERENCE_SET_POSITION, &args![actor, position]);
                let small = x87_quiet_bits(e.global::<u32>(SMALL_CONSTANT_01013EA4));
                e.call(
                    LOCAL_FROM_FLOATS,
                    &args![at(0x1e0), 0u32, 0u32, small, 0u32],
                );
                e.call(LOCAL_FROM_FLOATS, &args![at(0x1f0), 0u32, 0u32, 0u32, 0u32]);
                e.call(REFERENCE_USE_00561690, &args![actor, at(0x1e0)]);
                e.call(REFERENCE_USE_005617C0, &args![actor, at(0x1f0)]);
                item = item.wrapping_add(1);
            }
            if collidable != 0
                && e.vcall(collidable, 0x100, &args![]).u8() != 0
                && e.mem.u32(collidable + 0xb0) != 0
            {
                let word = e
                    .call(WORD_GETTER_00931ED0, &args![collidable, at(0x1f8)])
                    .u32();
                let value = e.call(WORD_AT_0, &args![word]).u32();
                let answer = e.vcall(collidable, 0x1d0, &args![]).u32();
                let target = e.mem.u32(collidable + 0xb0);
                e.call(COLLIDABLE_UPDATE, &args![target, answer, value]);
            }
            e.call(LIST_DESTRUCT, &args![items]);
        }
        if !found {
            let bounds = at(0x220);
            e.call(BOX_CONSTRUCT, &args![bounds]);
            e.call(GET_AABB_LOCAL, &args![reference, bounds]);
            let top = e
                .call(VECTOR_COMPONENT_00560D30, &args![at(0x210), 2u32])
                .u32();
            let top = x87_quiet_bits(e.mem.u32(top));
            let first = e.call(FLOAT_FUNCTION_004587D0, &args![top]).f64();
            let bottom = e
                .call(VECTOR_COMPONENT_00560D30, &args![bounds, 2u32])
                .u32();
            let bottom = x87_quiet_bits(e.mem.u32(bottom));
            let second = e.call(FLOAT_FUNCTION_004587D0, &args![bottom]).f64();
            let second = x87_store32(second);
            let third = e.call(FLOAT_FUNCTION_00408840, &args![second]).f64();
            let lift = x87_store32(x87_add(third, first));
            let height = e
                .call(VECTOR_COMPONENT_004A51B0, &args![at(0x28), 2u32])
                .u32();
            let raised = x87_add(x87_load32(e.mem.u32(height)), x87_m32(lift));
            e.mem.set_u32(height, x87_store32(raised));
            e.call(REFERENCE_SET_POSITION, &args![reference, at(0x28)]);
            let small = x87_quiet_bits(e.global::<u32>(SMALL_CONSTANT_01013EA4));
            e.call(
                LOCAL_FROM_FLOATS,
                &args![at(0x240), 0u32, 0u32, small, 0u32],
            );
            e.call(REFERENCE_USE_00561690, &args![reference, at(0x240)]);
        }
    });
}

// Translated from 00628f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` nonzero) or clears ([`fn_00628f70`]) the bits of `mask` in the word at
/// +0x10 (unnamed in the engine map; `ECX` and two stack words, `RET 8`).
pub fn fn_00628f30(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    if set != 0 {
        let flags = e.mem.u32(this.addr() + 0x10) | mask;
        e.mem.set_u32(this.addr() + 0x10, flags);
    } else {
        fn_00628f70(e, this, mask);
    }
}

// Translated from 00628f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bits of `mask` in the word at +0x10 (unnamed in the engine map; `ECX` and one
/// stack word, `RET 4`).
pub fn fn_00628f70(e: &mut Engine, this: Ptr, mask: u32) {
    let flags = !mask & e.mem.u32(this.addr() + 0x10);
    e.mem.set_u32(this.addr() + 0x10, flags);
}

// Translated from 00629560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`; called by `00628f90` and
/// `FindWarpLocation`): `006285e0(this, this, value)`, then `00628640` the same way on the
/// part at +0x10.
pub fn fn_00629560(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    e.call(0x0062_85e0, &args![base, base, value]);
    e.call(0x0062_8640, &args![base + 0x10, base + 0x10, value]);
}

// Translated from 006295a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, plain `RET`; called by `00628f90` and
/// `FindWarpLocation`): `004589c0` on `this` with the `float` at `0104da48`, then on the
/// part at +0x10 with the `float` at `0104e148` (each through `FLD`/`FSTP`).
pub fn fn_006295a0(e: &mut Engine, this: Ptr) {
    let first = x87_quiet_bits(e.global::<u32>(0x0104_da48));
    e.call(0x0045_89c0, &args![this, first]);
    let second = x87_quiet_bits(e.global::<u32>(0x0104_e148));
    e.call(0x0045_89c0, &args![this.addr() + 0x10, second]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_8f30, fn_00628f30(Ptr, u32, u8)),
        entry!(0x0062_8f70, fn_00628f70(Ptr, u32)),
        entry!(0x0062_9560, fn_00629560(Ptr, u32)),
        entry!(0x0062_95a0, fn_006295a0(Ptr)),
        entry!(0x0062_8da0, fn_00628da0(Ptr)),
        entry!(0x0062_8950, fn_00628950(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0062_88a0, fn_006288a0(Ptr) -> Ptr),
        entry!(0x0062_88c0, fn_006288c0(Ptr, Ptr)),
        entry!(0x0062_8cc0, fn_00628cc0(Ptr, Ptr, Ptr)),
        entry!(0x0062_8c50, fn_00628c50(Ptr, Ptr) -> u32),
        entry!(0x0062_8d10, fn_00628d10(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0062_8df0, fn_00628df0(Ptr, Ptr, Ptr, u32)),
        entry!(0x0062_8980, fn_00628980(Ptr, Ptr, Ptr)),
        entry!(0x0062_86f0, fn_006286f0(Ptr)),
        entry!(0x0062_8f90, fn_00628f90(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lanes(e: &Engine, p: u32) -> [f32; 4] {
        [
            e.mem.f32(p),
            e.mem.f32(p + 4),
            e.mem.f32(p + 8),
            e.mem.f32(p + 12),
        ]
    }

    fn set_lanes(e: &mut Engine, p: u32, v: [f32; 4]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x);
        }
    }

    /// Doubles with the vector semantics the callees have.
    fn vector_doubles(e: &mut Engine) {
        e.register(LANES_LESS_EQUAL, |e, a| {
            let (x, y) = (lanes(e, a[0]), lanes(e, a[2]));
            for i in 0..4 {
                let m = if x[i] <= y[i] { u32::MAX } else { 0 };
                e.mem.set_u32(a[1] + 4 * i as u32, m);
            }
            Ret::default()
        });
        e.register(MASK_ALL_SET, |e, a| {
            let mut mask = 0;
            for i in 0..4 {
                mask |= (e.mem.u32(a[0] + 4 * i) >> 31) << i;
            }
            Ret {
                eax: (mask & a[1] == a[1]) as u32,
                ..Ret::default()
            }
        });
        e.register(STORE_BYTE, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(BYTE_SET, |e, a| Ret {
            eax: (e.mem.u8(a[0]) != 0) as u32,
            ..Ret::default()
        });
        e.register(BOX_COPY, |e, a| {
            let words = e.mem.bytes(a[1], 0x20);
            e.mem.write(a[0], &words);
            Ret::default()
        });
        e.register(VECTOR_SET, |e, a| {
            let words = e.mem.bytes(a[1], 0x10);
            e.mem.write(a[0], &words);
            Ret::default()
        });
        e.register(VECTOR_COPY, |e, a| {
            let words = e.mem.bytes(a[1], 0x10);
            e.mem.write(a[0], &words);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(VECTOR_SPLAT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1]);
            }
            Ret::default()
        });
        e.register(VECTOR_MAX, |e, a| {
            let (x, y) = (lanes(e, a[1]), lanes(e, a[2]));
            set_lanes(e, a[0], [0, 1, 2, 3].map(|i| x[i].max(y[i])));
            Ret::default()
        });
        e.register(VECTOR_MIN, |e, a| {
            let (x, y) = (lanes(e, a[1]), lanes(e, a[2]));
            set_lanes(e, a[0], [0, 1, 2, 3].map(|i| x[i].min(y[i])));
            Ret::default()
        });
        for addr in [BOX_CONSTRUCT, VECTOR_CONSTRUCT, EMPTY_FUNCTION] {
            e.register(addr, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
    }

    fn new_box(e: &mut Engine, min: [f32; 4], max: [f32; 4]) -> u32 {
        let b = e.mem.alloc(0x20);
        set_lanes(e, b, min);
        set_lanes(e, b + 0x10, max);
        b
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00628da0_clears_the_pointers_and_tears_down() {
        let mut e = Engine::new();
        e.register(0x0066_b0d0, |_, _| Ret::default());
        e.register(0x0043_8af0, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x20, 9);
        e.call_log = Some(vec![]);
        e.call(0x0062_8da0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        let assigned: Vec<u32> = calls(&e, 0x0066_b0d0).iter().map(|a| a[0]).collect();
        assert_eq!(
            assigned,
            (0..6).map(|i| this + 8 + 4 * i).collect::<Vec<_>>()
        );
        assert_eq!(calls(&e, 0x0043_8af0), vec![vec![this + 0x24]]);
    }

    #[test]
    fn fn_00628950_builds_a_box_from_two_vectors() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        let min = new_box(&mut e, [1.0, 2.0, 3.0, 4.0], [0.0; 4]);
        let max = new_box(&mut e, [5.0, 6.0, 7.0, 8.0], [0.0; 4]);
        assert_eq!(e.call(0x0062_8950, &args![this, min, max]).u32(), this);
        assert_eq!(lanes(&e, this), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(lanes(&e, this + 0x10), [5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn fn_006288a0_returns_this_plus_110() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0062_88a0, &args![0x1000u32]).u32(), 0x1110);
    }

    #[test]
    fn fn_006288c0_takes_the_proxy_box_or_the_fixed_vector() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        e.register(OBJECT_PROXY, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(BOUNDS_AT_B0, |_, a| Ret {
            eax: a[0] + 0xb0,
            ..Ret::default()
        });
        e.map(0x0126_7e30, 0x10);
        set_lanes(&mut e, 0x0126_7e30, [9.0, 8.0, 7.0, 6.0]);
        e.register(VECTOR_AT_01267E30, |_, _| Ret {
            eax: 0x0126_7e30,
            ..Ret::default()
        });
        let proxy = e.mem.alloc(0xd0);
        set_lanes(&mut e, proxy + 0xb0, [1.0; 4]);
        set_lanes(&mut e, proxy + 0xc0, [2.0; 4]);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, proxy);
        let out = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0062_88c0, &args![this, out]);
        assert_eq!(lanes(&e, out), [1.0; 4]);
        assert_eq!(lanes(&e, out + 0x10), [2.0; 4]);
        assert_eq!(calls(&e, EMPTY_FUNCTION).len(), 2);
        e.mem.set_u32(this, 0);
        e.call(0x0062_88c0, &args![this, out]);
        assert_eq!(lanes(&e, out), [9.0, 8.0, 7.0, 6.0]);
        assert_eq!(lanes(&e, out + 0x10), [9.0, 8.0, 7.0, 6.0]);
    }

    #[test]
    fn fn_00628cc0_ands_four_words() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(a + 4 * i, 0xff00_ff00 | i);
            e.mem.set_u32(b + 4 * i, 0x0ff0_0ff0 | (i << 4));
        }
        e.call(0x0062_8cc0, &args![this, a, b]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this + 4 * i), 0x0f00_0f00 & 0xffff_ff00);
        }
    }

    #[test]
    fn fn_00628c50_tests_overlap_on_three_axes() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let this = new_box(&mut e, [0.0; 4], [10.0; 4]);
        let near = new_box(&mut e, [5.0, 5.0, 5.0, 50.0], [15.0, 15.0, 15.0, 60.0]);
        let far = new_box(&mut e, [5.0, 11.0, 5.0, 0.0], [15.0, 15.0, 15.0, 0.0]);
        assert_eq!(e.call(0x0062_8c50, &args![this, near]).u32(), 1);
        assert_eq!(e.call(0x0062_8c50, &args![this, far]).u32(), 0);
    }

    #[test]
    fn fn_00628d10_tests_containment_into_a_byte() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let this = new_box(&mut e, [0.0; 4], [10.0; 4]);
        let inside = new_box(&mut e, [1.0, 1.0, 1.0, -5.0], [9.0, 9.0, 9.0, 50.0]);
        let across = new_box(&mut e, [1.0; 4], [9.0, 11.0, 9.0, 9.0]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x0062_8d10, &args![this, out, inside]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x0062_8d10, &args![this, out, across]);
        assert_eq!(e.mem.u8(out), 0);
    }

    /// An array (data at +0, count at +4) of `items`.
    fn array_of(e: &mut Engine, items: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * items.len() as u32 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        let array = e.mem.alloc(LIST_SIZE);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, items.len() as u32);
        array
    }

    /// Doubles for the object walks: an item's object is `item + 0x10`;
    /// the object's box (+0x14) is copied by the target's vtable +0x3C; the
    /// object word at +0 says it is live; +0x40 holds its type word, +0x44
    /// its collidable reference, +0x48 its reference, +0x4C the kind.
    fn walk_doubles(e: &mut Engine) {
        vector_doubles(e);
        e.register(ARRAY_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(ARRAY_ELEMENT, |e, a| Ret {
            eax: e.mem.u32(a[0]) + 4 * a[1],
            ..Ret::default()
        });
        e.register(PLUS_10, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        e.register(WORD_AT_0, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(WORD_AT_58, |_, _| Ret {
            eax: 0x0091_0000,
            ..Ret::default()
        });
        e.register(PLUS_14, |_, a| Ret {
            eax: a[0] + 0x14,
            ..Ret::default()
        });
        e.map(0x0091_0000, 4);
        e.put_vtable(0x0092_0000, &[0; 0x50]);
        e.mem.set_u32(0x0091_0000, 0x0092_0000);
        e.mem.set_u32(0x0092_0000 + 0x3c, 0x7000_003c);
        e.mem.set_u32(0x0092_0000 + 0x100, 0x7000_0100);
        e.register(0x7000_003c, |e, a| {
            let words = e.mem.bytes(a[1], 0x20);
            e.mem.write(a[2], &words);
            Ret::default()
        });
        e.register(0x7000_0100, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(OBJECT_TYPE_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(WORD_TYPE, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(FIND_COLLIDABLE_REF, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x44),
            ..Ret::default()
        });
        e.register(WARP_ACTOR, |_, _| Ret::default());
        e.register(OBJECT_REFERENCE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x48),
            ..Ret::default()
        });
        e.register(REFERENCE_KIND, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
    }

    /// An item whose object (+0x10) is live, with its box at object +0x14.
    fn item(e: &mut Engine, min: f32, max: f32, type_word: u32) -> u32 {
        let item = e.mem.alloc(0x70);
        let object = item + 0x10;
        e.mem.set_u32(object, 1);
        set_lanes(e, object + 0x14, [min; 4]);
        set_lanes(e, object + 0x24, [max; 4]);
        e.mem.set_u32(object + 0x40, type_word);
        let reference = e.mem.alloc(4);
        e.mem.set_u32(reference, 0x0092_0000);
        e.mem.set_u32(object + 0x44, reference);
        item
    }

    #[test]
    fn fn_00628df0_warps_collidable_references_outside_the_bounds() {
        let mut e = Engine::new();
        walk_doubles(&mut e);
        let bounds = new_box(&mut e, [0.0; 4], [10.0; 4]);
        let inside = item(&mut e, 1.0, 2.0, 0x1e);
        let outside = item(&mut e, 20.0, 30.0, 0x1e);
        let other_type = item(&mut e, 20.0, 30.0, 0x1d);
        let dead = item(&mut e, 20.0, 30.0, 0x1e);
        e.mem.set_u32(dead + 0x10, 0);
        let objects = array_of(&mut e, &[inside, outside, other_type, dead]);
        e.call_log = Some(vec![]);
        e.call(
            0x0062_8df0,
            &args![0x5000u32, bounds, objects, 0x0091_0000u32],
        );
        let warped = calls(&e, WARP_ACTOR);
        assert_eq!(warped, vec![vec![0x5000, e.mem.u32(outside + 0x54)]]);
    }

    #[test]
    fn fn_00628980_gathers_new_references_and_collects_for_each() {
        let mut e = Engine::new();
        walk_doubles(&mut e);
        let gathered = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = gathered.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            let word = e.mem.u32(a[1]);
            log.borrow_mut().push((a[0], word));
            if e.mem.u32(a[0]) == 0 {
                let data = e.mem.alloc(0x40);
                e.mem.set_u32(a[0], data);
            }
            let count = e.mem.u32(a[0] + 4);
            let data = e.mem.u32(a[0]);
            e.mem.set_u32(data + 4 * count, word);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
        // Local arrays start empty; the done list never holds the element.
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(LIST_FIND, |_, _| Ret {
            eax: u32::MAX,
            ..Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(REFERENCE_KEY, |_, a| Ret {
            eax: a[0] ^ 0xff,
            ..Ret::default()
        });
        e.register(COLLECT_FOR_KEY, |_, _| Ret::default());
        e.register(ELEMENT_ACTOR, |_, _| Ret {
            eax: 0x0091_0000,
            ..Ret::default()
        });
        e.register(ELEMENT_INNER_ARRAY, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        let bounds = new_box(&mut e, [0.0; 4], [10.0; 4]);
        let mut objects = vec![];
        for kind in [3u32, 4, 5, 7] {
            let it = item(&mut e, 20.0, 30.0, 0);
            let reference = e.mem.alloc(4);
            e.mem.set_u32(reference, kind);
            e.mem.set_u32(it + 0x58, reference);
            objects.push(it);
        }
        let inside = item(&mut e, 1.0, 2.0, 0);
        objects.push(inside);
        let inner = array_of(&mut e, &objects);
        let group = e.mem.alloc(8);
        e.mem.set_u32(group + 4, inner);
        let groups = array_of(&mut e, &[group]);
        e.call_log = Some(vec![]);
        e.call(0x0062_8980, &args![0x5000u32, bounds, groups]);
        let appended = gathered.borrow().clone();
        assert_eq!(appended.len(), 1, "only the kind-3 reference is gathered");
        let first_reference = e.mem.u32(objects[0] + 0x58);
        assert_eq!(appended[0].1, first_reference);
        assert_eq!(
            calls(&e, COLLECT_FOR_KEY)[0][..2].to_vec(),
            vec![0x5000, first_reference ^ 0xff]
        );
        assert_eq!(calls(&e, LIST_DESTRUCT).len(), 3);
    }

    #[test]
    fn fn_006286f0_bounds_the_six_objects_and_checks_the_cell() {
        let mut e = Engine::new();
        walk_doubles(&mut e);
        e.map(LOWEST_FLOAT, 4);
        e.map(HIGHEST_FLOAT, 4);
        e.set_global(LOWEST_FLOAT, 0xff7f_ffeeu32);
        e.set_global(HIGHEST_FLOAT, 0x7f7f_ffeeu32);
        e.register(OBJECT_PROXY, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(BOUNDS_AT_B0, |_, a| Ret {
            eax: a[0] + 0xb0,
            ..Ret::default()
        });
        e.register(CELL_OBJECT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CELL_OBJECT_DATA, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x30);
        for i in 0..6u32 {
            let proxy = e.mem.alloc(0xd0);
            let v = i as f32;
            set_lanes(&mut e, proxy + 0xb0, [v, -v, v, 0.0]);
            set_lanes(&mut e, proxy + 0xc0, [v + 1.0, 1.0, 2.0 * v, 0.0]);
            let holder = e.mem.alloc(4);
            e.mem.set_u32(holder, proxy);
            e.mem.set_u32(this + 8 + 4 * i, holder);
        }
        let cell = e.mem.alloc(4);
        e.mem.set_u32(this + 0x20, cell);
        e.call_log = Some(vec![]);
        e.call(0x0062_86f0, &args![this]);
        assert!(calls(&e, CELL_OBJECT_DATA).is_empty(), "no cell object");

        let copies = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = copies.clone();
        e.register_double(VECTOR_SET, move |e, a| {
            record.borrow_mut().push(lanes(e, a[1]));
            let words = e.mem.bytes(a[1], 0x10);
            e.mem.write(a[0], &words);
            Ret::default()
        });
        let cell_object = e.mem.alloc(4);
        e.mem.set_u32(cell, cell_object);
        let data = e.mem.alloc(0x120);
        e.mem.set_u32(cell_object, data);
        // Empty arrays at +0x110, +0x28 and +0x34.
        e.register(LIST_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_86f0, &args![this]);
        let counted: Vec<u32> = calls(&e, ARRAY_COUNT).iter().map(|a| a[0]).collect();
        assert_eq!(counted[0], data + 0x110);
        assert!(counted.contains(&(data + 0x28)));
        assert!(counted.contains(&(data + 0x34)));
        let set = calls(&e, VECTOR_SET);
        assert_eq!(set[1][0], set[0][0] + 0x10);
        let copied = copies.borrow().clone();
        assert_eq!(copied[0], [0.0, -5.0, 0.0, 0.0]);
        assert_eq!(copied[1], [6.0, 1.0, 10.0, 0.0]);
    }
}

#[cfg(test)]
mod lookahead2_tests {
    use super::*;

    const REFERENCE_VTABLE: u32 = 0x0e40_0000;
    const OBJECT_VTABLE: u32 = 0x0e41_0000;
    const FILLER_VTABLE: u32 = 0x0e42_0000;
    const REFERENCE_DATA: u32 = 0x0e50_0000;
    const OBJECT_KIND: u32 = 0x0e50_0004;
    const REFERENCE_SLOT_F4: u32 = 0x0e50_0008;
    const FILLER_FILL: u32 = 0x0e50_000c;
    const COLLIDABLE_TEST: u32 = 0x0e50_0010;
    const COLLIDABLE_ANSWER: u32 = 0x0e50_0014;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A rig: `reference` and `collidable` objects with vtables, the entity's objects
    /// answering `kinds` from slot +0x20, the collected items (each a word pointing at a
    /// word the reference key double answers), the vector component getters answering the
    /// component's address, `00439e90` adding, `004587d0` doubling and `00408840` adding 1.
    fn rig(kinds: Vec<u32>, keys: Vec<u32>) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x80];
        slots[0x94 / 4] = REFERENCE_DATA;
        slots[0xf4 / 4] = REFERENCE_SLOT_F4;
        slots[0x100 / 4] = COLLIDABLE_TEST;
        slots[0x1d0 / 4] = COLLIDABLE_ANSWER;
        e.put_vtable(REFERENCE_VTABLE, &slots);
        e.put_vtable(OBJECT_VTABLE, &[0, 0, 0, 0, 0, 0, 0, 0, OBJECT_KIND]);
        let mut filler = vec![0u32; 0x10];
        filler[0x3c / 4] = FILLER_FILL;
        e.put_vtable(FILLER_VTABLE, &filler);
        e.map(SMALL_CONSTANT_01013EA4, 4);
        e.set_global(SMALL_CONSTANT_01013EA4, 0.01f32.to_bits());
        let reference = e.mem.alloc(0x100);
        e.mem.set_u32(reference, REFERENCE_VTABLE);
        let collidable = e.mem.alloc(0x100);
        e.mem.set_u32(collidable, REFERENCE_VTABLE);
        e.register(REFERENCE_FLAG_TEST, |_, _| 0u32.into_ret());
        e.register(REFERENCE_MODEL_WORD, |_, _| 0x3300u32.into_ret());
        e.register_double(FIND_COLLIDABLE_REF, move |_, _| collidable.into_ret());
        for quiet in [
            VECTOR_CONSTRUCT,
            LOCAL_48_CONSTRUCT,
            LOCAL_34_CONSTRUCT,
            LOCAL_FROM_48,
            FIND_WARP_LOCATION,
            POINT_OPERATION_00439EF0,
            REFERENCE_SLOT_F4,
            BOX_CONSTRUCT,
            BOX_RESET,
            BOX_ADD,
            FILLER_FILL,
            LIST_APPEND,
            REFERENCE_SET_POSITION,
            LOCAL_FROM_FLOATS,
            REFERENCE_USE_00561690,
            REFERENCE_USE_005617C0,
            COLLIDABLE_UPDATE,
            LIST_DESTRUCT,
            GET_AABB_LOCAL,
        ] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.register(REFERENCE_ENTITY, |_, _| 0x4400u32.into_ret());
        let count = kinds.len() as u32;
        e.register_double(ENTITY_COUNT, move |_, _| count.into_ret());
        let objects: Vec<u32> = kinds
            .iter()
            .map(|kind| {
                let object = e.mem.alloc(8);
                e.mem.set_u32(object, OBJECT_VTABLE);
                e.mem.set_u32(object + 4, *kind);
                object
            })
            .collect();
        e.register_double(ENTITY_AT, move |_, a| objects[a[1] as usize].into_ret());
        e.register(ENTITY_OBJECT, |_, a| a[0].into_ret());
        e.register(OBJECT_KIND, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // The collected items: words pointing at the keys.
        let words: Vec<u32> = keys
            .iter()
            .map(|key| {
                let at = e.mem.alloc(4);
                e.mem.set_u32(at, *key);
                at
            })
            .collect();
        let data = e.mem.alloc(4 * words.len().max(1) as u32);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *w);
        }
        let item_count = words.len() as u32;
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register_double(ENTITY_COLLECT, move |e, a| {
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, item_count);
            Ret::default()
        });
        e.register(ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0]) + 4 * a[1]).into_ret()
        });
        e.register(PLUS_10, |_, a| (a[0] + 0x10).into_ret());
        e.register(PLUS_14, |_, a| (a[0] + 0x14).into_ret());
        e.register(REFERENCE_DATA, |_, _| 0x6600u32.into_ret());
        let filler = e.mem.alloc(4);
        e.mem.set_u32(filler, FILLER_VTABLE);
        e.register_double(WORD_AT_58, move |_, _| filler.into_ret());
        e.register(REFERENCE_KEY, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(VECTOR_COMPONENT_004A51B0, |_, a| {
            (a[0] + 4 * a[1]).into_ret()
        });
        e.register(VECTOR_COMPONENT_00560D30, |_, a| {
            (a[0] + 4 * a[1]).into_ret()
        });
        e.register(POINT_OPERATION_00439E90, |e, a| {
            for i in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, sum);
            }
            a[1].into_ret()
        });
        e.register(COLLIDABLE_TEST, |_, _| 1u32.into_ret());
        e.register(COLLIDABLE_ANSWER, |_, _| 0x7700u32.into_ret());
        e.register(WORD_GETTER_00931ED0, |_, a| a[1].into_ret());
        e.register(WORD_AT_0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FLOAT_FUNCTION_004587D0, |_, a| {
            st0(f64::from(f32::from_bits(a[0])) * 2.0)
        });
        e.register(FLOAT_FUNCTION_00408840, |_, a| {
            st0(f64::from(f32::from_bits(a[0])) + 1.0)
        });
        e.call_log = Some(vec![]);
        (e, reference, collidable)
    }

    #[test]
    fn nothing_for_a_null_reference_or_a_flagged_one() {
        let (mut e, reference, _) = rig(vec![], vec![]);
        e.call(0x0062_8f90, &args![0x9000u32, 0u32, 0x5500u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.register(REFERENCE_FLAG_TEST, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_8f90, &args![0x9000u32, reference, 0x5500u32]);
        // The byte answer's low byte is 0: the work goes on.
        assert_eq!(calls(&e, REFERENCE_MODEL_WORD).len(), 1);
        e.register(REFERENCE_FLAG_TEST, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_8f90, &args![0x9000u32, reference, 0x5500u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (REFERENCE_FLAG_TEST, vec![reference, 0x20]));
    }

    #[test]
    fn found_entity_moves_the_collected_references() {
        let (mut e, reference, collidable) = rig(vec![0xb, 3, 0xb], vec![0x8100, 0]);
        e.mem.set_u32(collidable + 0xb0, 0x9900);
        e.call(0x0062_8f90, &args![0x9000u32, reference, 0x5500u32]);
        // The walk stops at the second entity object.
        assert_eq!(calls(&e, ENTITY_AT).len(), 2);
        let warp = calls(&e, FIND_WARP_LOCATION);
        assert_eq!(warp[0][..2], [0x9000, collidable]);
        let fills = calls(&e, FILLER_FILL);
        assert_eq!(fills.len(), 2);
        assert_eq!(calls(&e, BOX_ADD).len(), 4);
        let positions = calls(&e, REFERENCE_SET_POSITION);
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0][0], 0x8100);
        let floats = calls(&e, LOCAL_FROM_FLOATS);
        assert_eq!(floats[0][1..], [0, 0, 0.01f32.to_bits(), 0]);
        assert_eq!(floats[1][1..], [0, 0, 0, 0]);
        assert_eq!(calls(&e, COLLIDABLE_UPDATE), vec![vec![0x9900, 0x7700, 0]]);
        assert_eq!(calls(&e, LIST_DESTRUCT).len(), 1);
        assert!(calls(&e, GET_AABB_LOCAL).is_empty());
    }

    #[test]
    fn found_entity_appends_other_references_and_raises_the_offset() {
        let (mut e, reference, _) = rig(vec![1], vec![0x8100, 0x8200]);
        // The second item's key is the reference.
        e.register_double(REFERENCE_KEY, move |e, a| {
            let key = e.mem.u32(a[0]);
            (if key == 0x8200 { reference } else { key }).into_ret()
        });
        // The box: min z 2.0 (at +8), max z 7.5 (at +0x18); the offset z starts at 1.0.
        e.register(BOX_RESET, |e, a| {
            e.mem.set_f32(a[0] + 8, 2.0);
            e.mem.set_f32(a[0] + 0x18, 7.5);
            Ret::default()
        });
        e.register(POINT_OPERATION_00439EF0, |e, a| {
            e.mem.set_f32(a[1] + 8, 1.0);
            a[1].into_ret()
        });
        e.call(0x0062_8f90, &args![0x9000u32, reference, 0x5500u32]);
        let appended = calls(&e, LIST_APPEND);
        assert_eq!(appended.len(), 1);
        assert_eq!(appended[0][0], 0x5500);
        // The moved position: the offset (z 1.0 + 5.5) added to the zeroed data.
        let moves = calls(&e, POINT_OPERATION_00439E90);
        let offset = moves[0][2];
        assert_eq!(e.mem.f32(offset + 8), 6.5);
        // The collidable's slot +0x100 answers 1 but its word at +0xB0 is 0: no update.
        assert!(calls(&e, COLLIDABLE_UPDATE).is_empty());
    }

    #[test]
    fn nothing_found_raises_the_warp_location() {
        let (mut e, reference, _) = rig(vec![0xb], vec![]);
        // The box: min z 1.5, max z 3.0.
        e.register(GET_AABB_LOCAL, |e, a| {
            e.mem.set_f32(a[1] + 8, 1.5);
            e.mem.set_f32(a[1] + 0x18, 3.0);
            Ret::default()
        });
        // The warp location's z starts at 10.0.
        e.register(VECTOR_CONSTRUCT, |e, a| {
            e.mem.set_f32(a[0] + 8, 10.0);
            a[0].into_ret()
        });
        e.call(0x0062_8f90, &args![0x9000u32, reference, 0x5500u32]);
        let set = calls(&e, REFERENCE_SET_POSITION);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][0], reference);
        // 10 + (2 * 3.0 + (2 * 1.5 + 1)) = 20.
        assert_eq!(e.mem.f32(set[0][1] + 8), 20.0);
        let floats = calls(&e, LOCAL_FROM_FLOATS);
        assert_eq!(floats.len(), 1);
        assert_eq!(
            calls(&e, REFERENCE_USE_00561690),
            vec![vec![reference, floats[0][0]]]
        );
        assert!(calls(&e, LIST_CONSTRUCT).is_empty());
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00628f30_and_00628f70_set_and_clear_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x0f);
        e.call(0x0062_8f30, &args![this, 0x30u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0x3f);
        e.call(0x0062_8f30, &args![this, 0x11u32, 0x100u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0x2e);
        e.call(0x0062_8f70, &args![this, 0x0eu32]);
        assert_eq!(e.mem.u32(this + 0x10), 0x20);
    }

    #[test]
    fn fn_00629560_and_006295a0_forward_to_both_parts() {
        let mut e = Engine::new();
        for callee in [0x0062_85e0u32, 0x0062_8640, 0x0045_89c0] {
            e.register(callee, |_, _| Ret::default());
        }
        e.map(0x0104_d000, 0x2000);
        e.set_global(0x0104_da48u32, 2.5f32.to_bits());
        e.set_global(0x0104_e148u32, 0x7f80_0001u32);
        e.call_log = Some(vec![]);
        e.call(0x0062_9560, &args![0x1000u32, 7u32]);
        e.call(0x0062_95a0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0062_85e0, vec![0x1000, 0x1000, 7]));
        assert_eq!(log[2], (0x0062_8640, vec![0x1010, 0x1010, 7]));
        assert_eq!(log[4], (0x0045_89c0, vec![0x1000, 2.5f32.to_bits()]));
        assert_eq!(log[5], (0x0045_89c0, vec![0x1010, 0x7fc0_0001]));
    }
}
