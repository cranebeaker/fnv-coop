//! `fallout shared/havok/tesarrowtargetlistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005545b0` (unnamed in the engine map): `thiscall`, plain `RET`; the destructor `0062a4e0`
/// runs.
const DESTRUCT: u32 = 0x0055_45b0;
/// `00401030` (unnamed in the engine map): `cdecl`, one word; the delete of a deleting
/// destructor.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `004b5a80` (`fallout shared/globalfunc.cpp` in the engine map): `cdecl`, the entity.
const ENTITY_KEY: u32 = 0x004b_5a80;
/// `00653270` (unnamed here; `cdecl`, the map and a key): the reference for the key.
const MAP_LOOKUP: u32 = 0x0065_3270;
/// The map `0062a510` and `0062a690` look the entity up in.
const ENTITY_MAP: u32 = 0x0126_81c0;
/// `00563200` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, `RET 4`.
const REFERENCE_HAS: u32 = 0x0056_3200;
/// `bhkCharacterProxy::operator->` (engine map name of `004ae750`, possibly a folded body):
/// `thiscall`, plain `RET`.
const HELD_OBJECT: u32 = 0x004a_e750;
/// `004b5a20` (`fallout shared/globalfunc.cpp` in the engine map): `cdecl`, one word.
const OWNER_OF: u32 = 0x004b_5a20;
/// `0044ddc0` (unnamed in the engine map): `thiscall`, plain `RET`; answers a word (flags, as
/// `0062a510` tests bit 5 of it).
const WORD_OF: u32 = 0x0044_ddc0;
/// `TESObjectREFR::FindReferenceFor3D` (engine map name of `0056f930`): `cdecl`, one word; its
/// answer is not used.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `BaseProcess::GetActorPackageThatIsRunning` (engine map name of `00717e50`, possibly a folded
/// body): `thiscall` on `011e0e80`, plain `RET`; answers the list `0062a510` walks.
const LIST_OF: u32 = 0x0071_7e50;
/// The object `00717e50` is called on.
const LIST_OWNER: u32 = 0x011e_0e80;
/// `005be5c0` (`fallout shared/tesscriptfunctions.cpp` in the engine map): `thiscall`, `RET 4`;
/// the list's count.
const LIST_COUNT: u32 = 0x005b_e5c0;
/// `00968670` (`fallout/ai/playercharacter.cpp` in the engine map): `thiscall`, `RET 4`; the
/// list's element at an index.
const LIST_AT: u32 = 0x0096_8670;
/// Vtable slot of the element test the callbacks make first.
const ELEMENT_TEST_SLOT: u32 = 0x224;
/// `00444d00` (`fallout shared/modelloader.cpp` in the engine map): `thiscall`, `RET 4`; answers
/// a byte.
const HAS_FLAG: u32 = 0x0044_4d00;
/// `00401170` (unnamed in the engine map): `thiscall`, plain `RET`; answers a type number.
const TYPE_OF: u32 = 0x0040_1170;
/// `00472380` (`fallout shared/tesfile.cpp` in the engine map): `thiscall`, plain `RET`.
const STATE_OF: u32 = 0x0047_2380;
/// `009bc8f0` (`fallout/ai/combat/projectile.cpp` in the engine map): `thiscall`, plain `RET`.
const ELEMENT_RELEASE: u32 = 0x009b_c8f0;
/// `006815c0` (unnamed in the engine map): `thiscall`, plain `RET`; answers the address of the
/// held word.
const HELD_WORD: u32 = 0x0068_15c0;
/// `bhkCollisionObject::GetbhkCollisionObject` (Xbox PDB), `0043b610`: `cdecl`, one word.
const GET_COLLISION_OBJECT: u32 = 0x0043_b610;
/// `006fa820` (unnamed in the engine map): `thiscall`, plain `RET`; the rigid body of a
/// collision object.
const RIGID_BODY_OF: u32 = 0x006f_a820;
/// `008256d0` (unnamed in the engine map): `thiscall` on a list node, plain `RET`; answers a byte
/// that ends the walk.
const NODE_IS_END: u32 = 0x0082_56d0;
/// `00726070` (`fallout/interface/menumanager.cpp` in the engine map): `thiscall` on a list
/// node, plain `RET`; the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// The list of records `0062a7e0` keeps.
const RECORD_LIST: u32 = 0x011c_c408;
/// `00401000` (unnamed in the engine map): `cdecl`, the size; the allocation.
const ALLOCATE: u32 = 0x0040_1000;
/// `005ae3d0` (unnamed in the engine map): `thiscall` on the list, `RET 4`, the address of the
/// word holding the new record.
const LIST_ADD: u32 = 0x005a_e3d0;
/// `0056ded0` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, plain `RET`;
/// constructs a transform.
const TRANSFORM_CONSTRUCT: u32 = 0x0056_ded0;
/// `hkTransform::setInverse` (Xbox PDB), `00cb21e0`: `thiscall`, `RET 4`.
const TRANSFORM_SET_INVERSE: u32 = 0x00cb_21e0;
/// `hkTransform::setMul` (Xbox PDB), `00cb2240`: `thiscall`, `RET 8`.
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
/// Vtable slot of the rigid body's transform getter.
const BODY_TRANSFORM_SLOT: u32 = 0xf4;
/// Vtable slot of the rigid body's centre of mass getter.
const BODY_CENTER_SLOT: u32 = 0xec;
/// `00436aa0` (`fallout shared/extradataobjects.cpp` in the engine map): `thiscall`, plain
/// `RET`.
const TRANSFORM_TRANSLATION: u32 = 0x0043_6aa0;
/// `004a3c90` (unnamed in the engine map): `thiscall`, `RET 4`; a vector copy.
const VECTOR_COPY: u32 = 0x004a_3c90;
/// `00403e20` (`fallout shared/magic/effectitem.cpp` in the engine map): `thiscall` on
/// `011cf8b0`, plain `RET`; answers the address of a `float`.
const FLOAT_HOLDER: u32 = 0x0040_3e20;
/// The object `00403e20` is called on.
const FLOAT_OWNER: u32 = 0x011c_f8b0;
/// `005dbf20` (`fallout shared/tesscriptfunctions.cpp` in the engine map): `thiscall`, `RET 4`,
/// a `float`; a vector of that `float`.
const VECTOR_SPLAT: u32 = 0x005d_bf20;
/// `00627920` (unnamed in the engine map): `thiscall`, `RET 4`; a vector multiply in place.
const VECTOR_MUL: u32 = 0x0062_7920;
/// `00458880` (`fallout shared/tes.cpp` in the engine map): `thiscall`, `RET 8`; a vector sum.
const VECTOR_ADD: u32 = 0x0045_8880;
/// `hkpRigidBody::setCenterOfMassLocal` (Xbox PDB), `00ca85f0`: `thiscall`, `RET 4`.
const SET_CENTER_OF_MASS_LOCAL: u32 = 0x00ca_85f0;
/// `hkpEntity::activate` (Xbox PDB), `00c9c1d0`: `thiscall`, plain `RET`.
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// The address `0062aad0` answers.
const TRANSFORM_SOURCE: u32 = 0x0126_83b0;
/// `0056df00` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, plain `RET`.
const ROTATION_CONSTRUCT: u32 = 0x0056_df00;
/// `00cb2d90` (`Havok SDK/hkrotation.cpp` in the engine map): `thiscall`, `RET 4`.
const ROTATION_SET: u32 = 0x00cb_2d90;
/// `005d3d10` (`fallout shared/tesscriptfunctions.cpp` in the engine map): `thiscall`, plain
/// `RET`; the motion of a rigid body.
const MOTION_OF: u32 = 0x005d_3d10;
/// `hkpMotion::getMass` (Xbox PDB), `00c86430`: `thiscall`, plain `RET`; the mass in `ST0`.
const MOTION_GET_MASS: u32 = 0x00c8_6430;
/// `004b5400` (`fallout shared/globalfunc.cpp` in the engine map): `thiscall`, plain `RET`; a
/// `float` in `ST0`.
const FALLBACK_MASS: u32 = 0x004b_5400;

// Translated from 0062a4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESArrowTargetListener::~TESArrowTargetListener` deleting form (engine map). `thiscall`,
/// `RET 4`: `005545b0`, then with bit 0 of `flags` the delete `00401030`. Answers `this`.
pub fn tes_arrow_target_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0062a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: the word held by the member at `+0x88` (`006815c0`
/// answers its address).
pub fn fn_0062a670(e: &mut Engine, this: Ptr) -> u32 {
    let held = e
        .call(HELD_WORD, &args![this.addr().wrapping_add(0x88)])
        .u32();
    e.mem.u32(held)
}

/// The body both entity callbacks share; `check_flag` adds the `0062a510` test of bit 5 of
/// `0044ddc0`'s answer.
fn release_arrows_of(e: &mut Engine, entity: u32, check_flag: bool) {
    let key = e.call(ENTITY_KEY, &args![entity]).u32();
    let reference = e.call(MAP_LOOKUP, &args![ENTITY_MAP, key]).u32();
    if reference == 0 {
        return;
    }
    if e.call(REFERENCE_HAS, &args![reference, 0x100u32]).u32() == 0 {
        return;
    }
    let held = e.call(HELD_OBJECT, &args![reference]).u32();
    let owner = e.call(OWNER_OF, &args![held]).u32();
    let word = if owner == 0 {
        0
    } else {
        e.call(WORD_OF, &args![owner]).u32()
    };
    // The answer is kept in a local the code never reads.
    e.call(FIND_REFERENCE_FOR_3D, &args![word]);
    let list = e.call(LIST_OF, &args![LIST_OWNER]).u32();
    let mut index = 0u32;
    while index < e.call(LIST_COUNT, &args![list, 0u32]).u32() {
        let element = e.call(LIST_AT, &args![list, index]).u32();
        if element != 0
            && e.vcall(element, ELEMENT_TEST_SLOT, &args![]).u8() != 0
            && element != 0
            && e.call(HAS_FLAG, &args![element, 0x80u32]).u8() != 0
            && e.call(TYPE_OF, &args![element]).u32() == 0x3d
            && (!check_flag || e.call(WORD_OF, &args![element]).u32() & 0x20 == 0)
        {
            let target = fn_0062a670(e, Ptr::new(element));
            if e.mem.u32(target.wrapping_add(0x1c)) == entity
                && e.call(STATE_OF, &args![element]).u32() == 4
            {
                e.call(ELEMENT_RELEASE, &args![element]);
            }
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0062a510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESArrowTargetListener::entityRemovedCallback` (engine map). `thiscall`, `RET 4` (`this` is
/// not read). Finds the reference of `entity` (`004b5a80`, then `00653270` on `012681c0`); when
/// it has `00563200(0x100)`, runs `0056f930` on what `0044ddc0` answers for its owner
/// (`004ae750`, `004b5a20`; 0 without one) and walks the list `00717e50` answers on `011e0e80`
/// (count `005be5c0` re-read each step, elements `00968670`). Each element that passes its vtable
/// slot `+0x224`, has `00444d00(0x80)`, is of type 0x3d (`00401170`), has bit 5 of `0044ddc0`
/// clear, targets `entity` (the word at `+0x1c` of what `0062a670` answers) and is in state 4
/// (`00472380`) is released (`009bc8f0`).
pub fn tes_arrow_target_listener_entity_removed_callback(e: &mut Engine, _this: Ptr, entity: u32) {
    release_arrows_of(e, entity, true);
}

// Translated from 0062a690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESArrowTargetListener::entityDeletedCallback` (engine map). `thiscall`, `RET 4` (`this` is
/// not read). The same walk as `entityRemovedCallback` (`0062a510`) without the bit 5 test.
pub fn tes_arrow_target_listener_entity_deleted_callback(e: &mut Engine, _this: Ptr, entity: u32) {
    release_arrows_of(e, entity, false);
}

// Translated from 0062aad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. No arguments read: answers the address `012683b0`.
pub fn fn_0062aad0(_e: &mut Engine) -> u32 {
    TRANSFORM_SOURCE
}

// Translated from 0062aae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. Constructs a transform: the rotation
/// (`0056df00`), the translation at `+0x30` copied from `translation` (`004a3c90`), the rotation
/// set from `rotation` (`00cb2d90`). Answers `this`.
pub fn fn_0062aae0(e: &mut Engine, this: Ptr, rotation: u32, translation: u32) -> Ptr {
    e.call(ROTATION_CONSTRUCT, &args![this]);
    e.call(
        VECTOR_COPY,
        &args![this.addr().wrapping_add(0x30), translation],
    );
    e.call(ROTATION_SET, &args![this, rotation]);
    this
}

// Translated from 0062ab20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetDynamicMass` (engine map). `fastcall`: the mass of the motion
/// (`005d3d10`, `hkpMotion::getMass`), or `004b5400` without one, rounded to a `float` and
/// answered in `ST0`.
pub fn bhk_rigid_body_get_dynamic_mass(e: &mut Engine, this: Ptr) -> f64 {
    let motion = e.call(MOTION_OF, &args![this]).u32();
    let mass = if motion == 0 {
        e.call(FALLBACK_MASS, &args![this]).f64()
    } else {
        e.call(MOTION_GET_MASS, &args![motion]).f64()
    };
    x87_load32(x87_store32(mass))
}

// Translated from 0062a7e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062a7e0 - /GS cookie check (00ec408c) not translated
/// Unnamed in the engine map. `cdecl`, two words (`source`, `object`). For the rigid body of
/// `object`'s collision object (`0043b610`, `006fa820`): finds its record in the list `011cc408`
/// (nodes `008256d0` / `006815c0` / `00726070`; a record's first word is its body) or makes one
/// (0x10 bytes from `00401000`: body, `object`, the dynamic mass `0062ab20`, 0; added through
/// `005ae3d0`). Then builds the transform from `source` (`0062aae0` with `012683b0`) relative to
/// the body's (vtable `+0xf4`, `setInverse`, `setMul`), and moves the body's centre of mass to
/// the mass-weighted mean of its own (vtable `+0xec`, weighted by the record's mass) and that
/// transform's translation (weighted by the `float` `00403e20` answers on `011cf8b0`): vectors
/// through `005dbf20`, `00627920`, `00458880`, divided by `1 / (mass + weight)`;
/// `setCenterOfMassLocal` and `activate` on the body's Havok object (`004ae750`). The record's
/// mass grows by the weight and its count by one.
pub fn fn_0062a7e0(e: &mut Engine, source: u32, object: u32) {
    let collision = if object == 0 {
        0
    } else {
        e.call(GET_COLLISION_OBJECT, &args![object]).u32()
    };
    let body = if collision == 0 {
        0
    } else {
        e.call(RIGID_BODY_OF, &args![collision]).u32()
    };
    if body == 0 {
        return;
    }
    // The frame below `EBP`: 0x1a0 bytes, a local at `EBP - x` is at `frame + 0x1a0 - x`.
    e.with_stack(0x1a0, |e, frame| {
        let at = |x: u32| frame.addr().wrapping_add(0x1a0).wrapping_sub(x);
        let record_word = at(0x18);
        e.mem.set_u32(record_word, 0);
        if e.call(NODE_IS_END, &args![RECORD_LIST]).u8() == 0 {
            let mut node = RECORD_LIST;
            while node != 0
                && e.call(NODE_IS_END, &args![node]).u8() == 0
                && e.mem.u32(record_word) == 0
            {
                let item = e.call(HELD_WORD, &args![node]).u32();
                let record = e.mem.u32(item);
                if record != 0 && e.mem.u32(record) == body {
                    e.mem.set_u32(record_word, record);
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        }
        if e.mem.u32(record_word) == 0 {
            let record = e.call(ALLOCATE, &args![0x10u32]).u32();
            e.mem.set_u32(record_word, record);
            e.mem.set_u32(record, body);
            e.mem.set_u32(record.wrapping_add(4), object);
            let mass = bhk_rigid_body_get_dynamic_mass(e, Ptr::new(body));
            e.mem.set_u32(record.wrapping_add(8), x87_store32(mass));
            e.mem.set_u32(record.wrapping_add(0xc), 0);
            e.call(LIST_ADD, &args![RECORD_LIST, record_word]);
        }
        let record = e.mem.u32(record_word);
        let mass = x87_quiet_bits(e.mem.u32(record.wrapping_add(8)));
        let body_transform = at(0x70);
        let inverse = at(0xb0);
        let source_transform = at(0xf0);
        let relative = at(0x130);
        let translation = at(0x140);
        let weight_vector = at(0x150);
        let center = at(0x160);
        let mass_vector = at(0x170);
        let sum = at(0x180);
        let scale_vector = at(0x1a0);
        e.call(TRANSFORM_CONSTRUCT, &args![body_transform]);
        e.call(TRANSFORM_CONSTRUCT, &args![inverse]);
        let rotation = fn_0062aad0(e);
        fn_0062aae0(e, Ptr::new(source_transform), rotation, source);
        e.call(TRANSFORM_CONSTRUCT, &args![relative]);
        e.vcall(body, BODY_TRANSFORM_SLOT, &args![body_transform]);
        e.call(TRANSFORM_SET_INVERSE, &args![inverse, body_transform]);
        e.call(
            TRANSFORM_SET_MUL,
            &args![relative, inverse, source_transform],
        );
        let offset = e.call(TRANSFORM_TRANSLATION, &args![relative]).u32();
        e.call(VECTOR_COPY, &args![translation, offset]);
        let weight = e.call(FLOAT_HOLDER, &args![FLOAT_OWNER]).u32();
        let weight_bits = x87_quiet_bits(e.mem.u32(weight));
        e.call(VECTOR_SPLAT, &args![weight_vector, weight_bits]);
        e.call(VECTOR_MUL, &args![translation, weight_vector]);
        e.call(HELD_WORD, &args![center]);
        e.vcall(body, BODY_CENTER_SLOT, &args![center]);
        e.call(VECTOR_SPLAT, &args![mass_vector, mass]);
        e.call(VECTOR_MUL, &args![center, mass_vector]);
        e.call(HELD_WORD, &args![sum]);
        e.call(VECTOR_ADD, &args![sum, center, translation]);
        let weight = e.call(FLOAT_HOLDER, &args![FLOAT_OWNER]).u32();
        let total = x87_add(x87_load32(mass), x87_m32(e.mem.u32(weight)));
        let inverse_total = x87_store32(x87_div(1.0, total));
        e.call(
            VECTOR_SPLAT,
            &args![scale_vector, x87_quiet_bits(inverse_total)],
        );
        e.call(VECTOR_MUL, &args![sum, scale_vector]);
        let havok_body = e.call(HELD_OBJECT, &args![body]).u32();
        e.call(SET_CENTER_OF_MASS_LOCAL, &args![havok_body, sum]);
        let havok_body = e.call(HELD_OBJECT, &args![body]).u32();
        e.call(ENTITY_ACTIVATE, &args![havok_body]);
        let weight = e.call(FLOAT_HOLDER, &args![FLOAT_OWNER]).u32();
        let total = x87_add(x87_load32(mass), x87_m32(e.mem.u32(weight)));
        let record = e.mem.u32(record_word);
        e.mem.set_u32(record.wrapping_add(8), x87_store32(total));
        let count = e.mem.u32(record.wrapping_add(0xc));
        e.mem
            .set_u32(record.wrapping_add(0xc), count.wrapping_add(1));
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0062_a4e0,
            tes_arrow_target_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0062_a510,
            tes_arrow_target_listener_entity_removed_callback(Ptr, u32)
        ),
        entry!(0x0062_a670, fn_0062a670(Ptr) -> u32),
        entry!(
            0x0062_a690,
            tes_arrow_target_listener_entity_deleted_callback(Ptr, u32)
        ),
        entry!(0x0062_a7e0, fn_0062a7e0(u32, u32)),
        entry!(0x0062_aad0, fn_0062aad0() -> u32),
        entry!(0x0062_aae0, fn_0062aae0(Ptr, u32, u32) -> Ptr),
        entry!(0x0062_ab20, bhk_rigid_body_get_dynamic_mass(Ptr) -> f64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn at_0062a4e0_deletes_only_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_a4e0, &args![0x4000u32, 0u32]).u32(), 0x4000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0062_a4e0, &args![0x4000u32, 1u32]);
        assert_eq!(calls(&e, DESTRUCT).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x4000]]);
    }

    #[test]
    fn at_0062a670_reads_the_held_word() {
        let mut e = Engine::new();
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x7777);
        e.register_double(HELD_WORD, move |_, _| cell.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_a670, &args![0x4000u32]).u32(), 0x7777);
        assert_eq!(calls(&e, HELD_WORD), vec![vec![0x4088]]);
    }

    const ENTITY: u32 = 0x00ab_cdef;

    /// Elements 0x1000 + 0x10 * i (i < 6); each one's target block is at `targets + 0x20 * i`. Element
    /// 0x1000 matches everything; 0x1010 fails the slot test; 0x1020 has the wrong type; 0x1030 has
    /// bit 5 set; 0x1040 targets another entity; 0x1050 is in state 3. Index 6 answers no element.
    fn callback_doubles(e: &mut Engine, has: u32) {
        let targets = e.mem.alloc(0x20 * 6);
        for i in 0..6u32 {
            let target = if i == 4 { 1 } else { ENTITY };
            e.mem.set_u32(targets + 0x20 * i + 0x1c, target);
        }
        let vtable = e.mem.alloc(0x228);
        e.mem.set_u32(vtable + ELEMENT_TEST_SLOT, 0x0700_0224);
        e.register(0x0700_0224, |_, a| (a[0] != 0x1010).into_ret());
        e.map(0x1000, 0x1000);
        for i in 0..7u32 {
            e.mem.set_u32(0x1000 + 0x10 * i, vtable);
        }
        e.register(ENTITY_KEY, |_, _| 0x55u32.into_ret());
        e.register(MAP_LOOKUP, |_, _| 0x9000u32.into_ret());
        e.register_double(REFERENCE_HAS, move |_, _| has.into_ret());
        e.register(HELD_OBJECT, |_, _| 0x9100u32.into_ret());
        e.register(OWNER_OF, |_, _| 0x9200u32.into_ret());
        e.register(WORD_OF, |_, a| {
            if a[0] == 0x1030 { 0x20u32 } else { 0x0fu32 }.into_ret()
        });
        e.register(FIND_REFERENCE_FOR_3D, |_, _| Ret::default());
        e.register(LIST_OF, |_, _| 0x9300u32.into_ret());
        e.register(LIST_COUNT, |_, _| 7u32.into_ret());
        e.register(LIST_AT, |_, a| {
            if a[1] == 6 { 0 } else { 0x1000 + 0x10 * a[1] }.into_ret()
        });
        e.register(HAS_FLAG, |_, _| 1u32.into_ret());
        e.register(TYPE_OF, |_, a| {
            if a[0] == 0x1020 { 0x3cu32 } else { 0x3du32 }.into_ret()
        });
        e.register_double(HELD_WORD, move |e, a| {
            let element = a[0] - 0x88;
            let cell = element + 4;
            e.mem
                .set_u32(cell, targets + 0x20 * ((element - 0x1000) / 0x10));
            cell.into_ret()
        });
        e.register(STATE_OF, |_, a| {
            if a[0] == 0x1050 { 3u32 } else { 4u32 }.into_ret()
        });
        e.register(ELEMENT_RELEASE, |_, _| Ret::default());
    }

    #[test]
    fn at_0062a510_releases_only_the_matching_arrow() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        callback_doubles(&mut e, 1);
        e.call(0x0062_a510, &args![0x4000u32, ENTITY]);
        assert_eq!(calls(&e, ENTITY_KEY), vec![vec![ENTITY]]);
        assert_eq!(calls(&e, MAP_LOOKUP), vec![vec![ENTITY_MAP, 0x55]]);
        assert_eq!(calls(&e, REFERENCE_HAS), vec![vec![0x9000, 0x100]]);
        assert_eq!(calls(&e, FIND_REFERENCE_FOR_3D), vec![vec![0x0f]]);
        assert_eq!(calls(&e, LIST_OF), vec![vec![LIST_OWNER]]);
        assert_eq!(calls(&e, LIST_COUNT).len(), 8, "re-read each step");
        assert_eq!(calls(&e, ELEMENT_RELEASE), vec![vec![0x1000]]);
    }

    #[test]
    fn at_0062a690_does_not_test_bit_5() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        callback_doubles(&mut e, 1);
        e.call(0x0062_a690, &args![0x4000u32, ENTITY]);
        assert_eq!(calls(&e, ELEMENT_RELEASE), vec![vec![0x1000], vec![0x1030]]);
        assert_eq!(calls(&e, WORD_OF), vec![vec![0x9200]]);
    }

    #[test]
    fn at_0062a510_stops_without_the_reference_or_its_flag() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        callback_doubles(&mut e, 0);
        e.call(0x0062_a510, &args![0x4000u32, ENTITY]);
        assert!(calls(&e, HELD_OBJECT).is_empty());
        e.register(MAP_LOOKUP, |_, _| 0u32.into_ret());
        e.call(0x0062_a690, &args![0x4000u32, ENTITY]);
        assert_eq!(calls(&e, REFERENCE_HAS).len(), 1);
        assert!(calls(&e, LIST_OF).is_empty());
    }

    #[test]
    fn at_0062a510_hands_zero_to_find_reference_without_an_owner() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        callback_doubles(&mut e, 1);
        e.register(OWNER_OF, |_, _| 0u32.into_ret());
        e.register(LIST_COUNT, |_, _| 0u32.into_ret());
        e.call(0x0062_a510, &args![0x4000u32, ENTITY]);
        assert_eq!(calls(&e, FIND_REFERENCE_FOR_3D), vec![vec![0]]);
        assert!(calls(&e, WORD_OF).is_empty());
    }

    #[test]
    fn at_0062aad0_and_0062aae0_build_the_source_transform() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_aad0, &args![]).u32(), TRANSFORM_SOURCE);
        e.register(ROTATION_CONSTRUCT, |_, _| Ret::default());
        e.register(VECTOR_COPY, |_, _| Ret::default());
        e.register(ROTATION_SET, |_, _| Ret::default());
        let answer = e.call(0x0062_aae0, &args![0x4000u32, 0x11u32, 0x22u32]);
        assert_eq!(answer.u32(), 0x4000);
        assert_eq!(calls(&e, ROTATION_CONSTRUCT), vec![vec![0x4000]]);
        assert_eq!(calls(&e, VECTOR_COPY), vec![vec![0x4030, 0x22]]);
        assert_eq!(calls(&e, ROTATION_SET), vec![vec![0x4000, 0x11]]);
    }

    #[test]
    fn at_0062ab20_takes_the_motion_mass_or_the_fallback() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MOTION_OF, |_, _| 0x5000u32.into_ret());
        e.register(MOTION_GET_MASS, |_, _| 0.1f64.into_ret());
        e.register(FALLBACK_MASS, |_, _| 7.0f64.into_ret());
        let mass = e.call(0x0062_ab20, &args![0x4000u32]).f64();
        assert_eq!(mass, f64::from(0.1f32), "rounded to a float");
        assert_eq!(calls(&e, MOTION_GET_MASS), vec![vec![0x5000]]);
        e.register(MOTION_OF, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0062_ab20, &args![0x4000u32]).f64(), 7.0);
        assert_eq!(calls(&e, FALLBACK_MASS), vec![vec![0x4000]]);
    }

    #[test]
    fn at_0062a7e0_does_nothing_without_a_rigid_body() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(GET_COLLISION_OBJECT, |_, _| 0x5000u32.into_ret());
        e.register(RIGID_BODY_OF, |_, _| 0u32.into_ret());
        e.call(0x0062_a7e0, &args![0x1u32, 0u32]);
        assert!(calls(&e, GET_COLLISION_OBJECT).is_empty());
        e.call(0x0062_a7e0, &args![0x1u32, 0x6000u32]);
        assert_eq!(calls(&e, RIGID_BODY_OF), vec![vec![0x5000]]);
        assert!(calls(&e, NODE_IS_END).is_empty());
    }

    /// A body at `0x6000` with its vtable; the weight `float` is 2.0.
    fn mass_doubles(e: &mut Engine) -> u32 {
        e.register(GET_COLLISION_OBJECT, |_, _| 0x5000u32.into_ret());
        e.register(RIGID_BODY_OF, |_, _| 0x6000u32.into_ret());
        e.map(0x6000, 0x100);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + BODY_TRANSFORM_SLOT, 0x0700_00f4);
        e.mem.set_u32(vtable + BODY_CENTER_SLOT, 0x0700_00ec);
        e.mem.set_u32(0x6000, vtable);
        e.register(0x0700_00f4, |_, _| Ret::default());
        e.register(0x0700_00ec, |_, _| Ret::default());
        for addr in [
            TRANSFORM_CONSTRUCT,
            TRANSFORM_SET_INVERSE,
            TRANSFORM_SET_MUL,
            VECTOR_COPY,
            VECTOR_SPLAT,
            VECTOR_MUL,
            VECTOR_ADD,
            SET_CENTER_OF_MASS_LOCAL,
            ENTITY_ACTIVATE,
            ROTATION_CONSTRUCT,
            ROTATION_SET,
            LIST_ADD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(TRANSFORM_TRANSLATION, |_, _| 0x7700u32.into_ret());
        e.register(HELD_OBJECT, |_, _| 0x6600u32.into_ret());
        let weight = e.mem.alloc(4);
        e.mem.set_u32(weight, 2.0f32.to_bits());
        e.register_double(FLOAT_HOLDER, move |_, _| weight.into_ret());
        e.register(MOTION_OF, |_, _| 0u32.into_ret());
        e.register(FALLBACK_MASS, |_, _| 6.0f64.into_ret());
        vtable
    }

    #[test]
    fn at_0062a7e0_makes_a_record_and_moves_the_center_of_mass() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        mass_doubles(&mut e);
        // An empty record list.
        e.register(NODE_IS_END, |_, _| 1u32.into_ret());
        e.register(HELD_WORD, |_, _| Ret::default());
        let record = e.mem.alloc(0x10);
        e.register_double(ALLOCATE, move |_, _| record.into_ret());
        e.call(0x0062_a7e0, &args![0x1234u32, 0x8800u32]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(record), 0x6000);
        assert_eq!(e.mem.u32(record + 4), 0x8800);
        assert_eq!(f32::from_bits(e.mem.u32(record + 8)), 8.0, "6 + 2");
        assert_eq!(e.mem.u32(record + 0xc), 1);
        let added = calls(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], RECORD_LIST);
        let source = calls(&e, VECTOR_COPY)[0].clone();
        assert_eq!(source[1], 0x1234, "the source's translation");
        let splats: Vec<u32> = calls(&e, VECTOR_SPLAT).iter().map(|a| a[1]).collect();
        assert_eq!(
            splats,
            vec![2.0f32.to_bits(), 6.0f32.to_bits(), 0.125f32.to_bits()]
        );
        let sum = calls(&e, VECTOR_ADD)[0][0];
        assert_eq!(calls(&e, SET_CENTER_OF_MASS_LOCAL), vec![vec![0x6600, sum]]);
        assert_eq!(calls(&e, ENTITY_ACTIVATE), vec![vec![0x6600]]);
        assert_eq!(
            calls(&e, ROTATION_SET),
            vec![vec![calls(&e, ROTATION_CONSTRUCT)[0][0], TRANSFORM_SOURCE]]
        );
    }

    #[test]
    fn at_0062a7e0_reuses_the_record_of_the_body() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        mass_doubles(&mut e);
        // Nodes 0x011cc408, then 0x200 (other body), then 0x300 (this body), then the end.
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other, 0x6100);
        let mine = e.mem.alloc(0x10);
        e.mem.set_u32(mine, 0x6000);
        e.mem.set_u32(mine + 8, 1.0f32.to_bits());
        e.mem.set_u32(mine + 0xc, 4);
        let cells = e.mem.alloc(0x10);
        e.mem.set_u32(cells, 0);
        e.mem.set_u32(cells + 4, other);
        e.mem.set_u32(cells + 8, mine);
        e.register(NODE_IS_END, |_, a| (a[0] == 0x400).into_ret());
        e.register(NODE_NEXT, |_, a| {
            match a[0] {
                RECORD_LIST => 0x200u32,
                0x200 => 0x300,
                _ => 0x400,
            }
            .into_ret()
        });
        e.register_double(HELD_WORD, move |_, a| {
            match a[0] {
                RECORD_LIST => cells,
                0x200 => cells + 4,
                0x300 => cells + 8,
                _ => cells + 0xc,
            }
            .into_ret()
        });
        e.call(0x0062_a7e0, &args![0x1234u32, 0x8800u32]);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert!(calls(&e, LIST_ADD).is_empty());
        assert_eq!(
            calls(&e, NODE_NEXT),
            vec![vec![RECORD_LIST], vec![0x200], vec![0x300]],
            "the walk stops once found"
        );
        assert_eq!(f32::from_bits(e.mem.u32(mine + 8)), 3.0);
        assert_eq!(e.mem.u32(mine + 0xc), 5);
        let splats: Vec<u32> = calls(&e, VECTOR_SPLAT).iter().map(|a| a[1]).collect();
        assert_eq!(splats[2], (1.0f32 / 3.0).to_bits());
    }
}
