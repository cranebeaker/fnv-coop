//! `fallout shared/havok/focollisionlistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00624050` calls on the member (`00577bf0`).
const CALLEE_00577BF0: u32 = 0x0057_7bf0;

/// `hkpContactListener`'s vtable (its RTTI name).
const CONTACT_LISTENER_VTABLE: u32 = 0x0104_dd7c;
/// `00559450`: the first word of an object (a `NiPointer`'s object).
const FIRST_WORD: u32 = 0x0055_9450;
/// `006815c0`: returns its `this`.
const SELF: u32 = 0x0068_15c0;
/// `008256d0`: whether a `BSSimpleList` node is empty (item and next 0).
const LIST_NODE_EMPTY: u32 = 0x0082_56d0;
/// `004b59f0`, `004b5a20`, `0044ddc0`, `0056f930`
/// (`TESObjectREFR::FindReferenceFor3D`, engine map): the steps from a
/// collision body to the reference it belongs to; `004b5a80` gives the
/// body's object.
const BODY_FROM_ENTITY: u32 = 0x004b_59f0;
const BODY_NODE: u32 = 0x004b_5a20;
const NODE_OBJECT: u32 = 0x0044_ddc0;
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
const BODY_OBJECT: u32 = 0x004b_5a80;
/// Settings: `fMinSoundVel:Audio` (read through `00450410`) and
/// `fPhysicsDamageSpeedMin` (through `00403e20`).
const F_MIN_SOUND_VEL: u32 = 0x011c_c270;
const F_PHYSICS_DAMAGE_SPEED_MIN: u32 = 0x011c_e75c;
/// Constants: `1.0` (`double`), `0.0` (`double`) and the `float`s the
/// listener uses when a value is missing or as a damage amount.
const ONE: u32 = 0x0101_2070;
const ZERO: u32 = 0x0101_2060;
const DEFAULT_SOUND_LEVEL: u32 = 0x0101_7870;
const DEFAULT_MASS: u32 = 0x0102_2958;
const PLAYER_DAMAGE: u32 = 0x0102_918c;
/// The player character (`PlayerCharacter *`).
const PLAYER: u32 = 0x011d_ea3c;

// Translated from 00624050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00577bf0` with `ECX` = `this + 0x4` and passes on its `float` result (`ST0`).
pub fn fn_00624050(e: &mut Engine, this: Ptr) -> f32 {
    e.call(CALLEE_00577BF0, &args![this.byte_add(0x4)]).f32()
}

// Translated from 00623570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpContactListener` base's constructor: stores its vtable
/// (`0104dd7c`) and returns `this`.
pub fn fn_00623570(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), CONTACT_LISTENER_VTABLE);
    this
}

/// The reference a collision entity belongs to: `004b59f0` on it, then
/// `004b5a20`, `0044ddc0` and `TESObjectREFR::FindReferenceFor3D`, each
/// only on a non-null result. Returns the body (`004b59f0`'s result) and
/// the reference.
fn entity_reference(e: &mut Engine, entity: u32) -> (u32, u32) {
    let body = e.call(BODY_FROM_ENTITY, &args![entity]).u32();
    let node = if body != 0 {
        e.call(BODY_NODE, &args![body]).u32()
    } else {
        0
    };
    let object = if node != 0 {
        e.call(NODE_OBJECT, &args![node]).u32()
    } else {
        0
    };
    let reference = if object != 0 {
        e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32()
    } else {
        0
    };
    (body, reference)
}

// Translated from 00623640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Works off the queued collision damage in the `BSSimpleList` at `+4`:
/// takes each record off the list (`0063f7b0`); a record with a body and a
/// positive amount (`+4`) finds its reference (`bhkCharacterProxy`'s
/// operator `004ae750`, `004b5a20`, `0044ddc0`,
/// `TESObjectREFR::FindReferenceFor3D`), tells it about the cause (its
/// vtable +0x164 with the record's `+8` object) and applies the amount:
/// directly (vtable +0x144 with the amount and 0) unless `008c7aa0` says
/// to queue it with the task queue (`0087af50` on the queue from
/// `004537b0`, with the reference, the amount and 0). A reference that is
/// flagged (`00477ba0`) and whose cause is the player gets the player's
/// `00963eb0` (10, the `float` at `0102918c`, 0). Each record is then
/// deleted (`00623820` with flag 1).
pub fn fn_00623640(e: &mut Engine, this: Ptr) {
    let list = this.addr().wrapping_add(4);
    // The code tests the (unchanging) list address for null before every
    // round.
    if list == 0 {
        return;
    }
    loop {
        if e.call(LIST_NODE_EMPTY, &args![list]).u8() != 0 {
            break;
        }
        let slot = e.call(SELF, &args![list]).u32();
        let record = e.mem.u32(slot);
        e.call(0x0063_f7b0, &args![list]);
        if e.call(FIRST_WORD, &args![record]).u32() != 0
            && x87_load32(e.mem.u32(record.wrapping_add(4))) > e.global::<f64>(ZERO)
        {
            let proxy = e.call(FIRST_WORD, &args![record]).u32();
            let entity = e.call(0x004a_e750, &args![proxy]).u32();
            let node = if entity != 0 {
                e.call(BODY_NODE, &args![entity]).u32()
            } else {
                0
            };
            let object = if node != 0 {
                e.call(NODE_OBJECT, &args![node]).u32()
            } else {
                0
            };
            let reference = if object != 0 {
                e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32()
            } else {
                0
            };
            if reference != 0 {
                let cause = e.call(FIRST_WORD, &args![record.wrapping_add(8)]).u32();
                e.vcall(reference, 0x164, &args![cause]);
                let amount = x87_quiet_bits(e.mem.u32(record.wrapping_add(4)));
                if e.call(0x008c_7aa0, &[]).u8() == 0 {
                    e.vcall(reference, 0x144, &args![amount, 0u32]);
                } else {
                    let queue = e.call(0x0045_37b0, &[]).u32();
                    e.call(0x0087_af50, &args![queue, reference, amount, 0u32]);
                }
                if e.call(0x0047_7ba0, &args![reference]).u8() != 0
                    && e.call(FIRST_WORD, &args![record.wrapping_add(8)]).u32() != 0
                {
                    let cause = e.call(FIRST_WORD, &args![record.wrapping_add(8)]).u32();
                    let cause_object = e.call(FIRST_WORD, &args![cause]).u32();
                    let player: u32 = e.global(PLAYER);
                    if cause_object == player {
                        let damage = x87_quiet_bits(e.mem.u32(PLAYER_DAMAGE));
                        e.call(0x0096_3eb0, &args![player, 0xau32, damage, 0u32]);
                    }
                }
            }
        }
        if record != 0 {
            e.call(0x0062_3820, &args![record, 1u32]);
        }
    }
}

/// One direction of `fn_006238b0`: queues damage for `target` when it is a
/// live, unflagged reference (`00452370`, `00477ba0`) that isn't excluded
/// (its vtable +0x224, and the other side's) and the impact (from the other
/// body's mass, `004b4ea0`, or the `float` at `01022958` for a zero mass,
/// with the speed, through `0062be90`) is positive: a 0x0C-byte record
/// (`00623bd0`) holding the body's object (`004b5a80`), the amount and the
/// other side's cause (its vtable +0x168), added to the list at `+4`
/// (`005ae3d0`).
fn queue_damage(
    e: &mut Engine,
    this: Ptr,
    speed: u32,
    target: u32,
    other: u32,
    mass_body: u32,
    object_body: u32,
) {
    if target == 0
        || e.call(0x0045_2370, &args![target]).u8() == 0
        || e.call(0x0047_7ba0, &args![target]).u8() != 0
        || e.vcall(target, 0x224, &[]).u8() != 0
    {
        return;
    }
    if other != 0 && e.vcall(other, 0x224, &[]).u8() != 0 {
        return;
    }
    let mut mass = x87_store32(e.call(0x004b_4ea0, &args![mass_body]).st0);
    if x87_load32(mass) == e.global::<f64>(ZERO) {
        mass = x87_quiet_bits(e.mem.u32(DEFAULT_MASS));
    }
    let amount = x87_store32(
        e.call(
            0x0062_be90,
            &args![x87_quiet_bits(mass), x87_quiet_bits(speed)],
        )
        .st0,
    );
    if x87_load32(amount) <= e.global::<f64>(ZERO) || x87_load32(amount).is_nan() {
        return;
    }
    let block = e.call(0x0040_1000, &args![0xcu32]).u32();
    let record = if block != 0 {
        e.call(0x0062_3bd0, &args![block]).u32()
    } else {
        0
    };
    let object = e.call(BODY_OBJECT, &args![object_body]).u32();
    e.call(0x0066_b0d0, &args![record, object]);
    e.mem
        .set_u32(record.wrapping_add(4), x87_quiet_bits(amount));
    let cause = if other != 0 {
        e.vcall(other, 0x168, &[]).u32()
    } else {
        0
    };
    e.call(0x0042_f780, &args![record.wrapping_add(8), cause]);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), record);
        e.call(0x005a_e3d0, &args![this.addr().wrapping_add(4), local]);
    });
}

// Translated from 006238b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006238b0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Queues collision damage for both sides of a contact: the impact speed
/// (`00408820` on `speed`) must reach `fPhysicsDamageSpeedMin` (a NaN
/// passes); then the first reference takes damage weighed by the second
/// body's mass, and the second by the first's (see `queue_damage`).
pub fn fn_006238b0(
    e: &mut Engine,
    this: Ptr,
    speed: f32,
    first_body: u32,
    second_body: u32,
    first: u32,
    second: u32,
) {
    let speed = x87_store32(
        e.call(0x0040_8820, &args![x87_quiet_bits(speed.to_bits())])
            .st0,
    );
    let minimum = e
        .call(0x0040_3e20, &args![F_PHYSICS_DAMAGE_SPEED_MIN])
        .u32();
    let minimum = x87_load32(e.mem.u32(minimum));
    if minimum > x87_load32(speed) {
        return;
    }
    queue_damage(e, this, speed, first, second, second_body, first_body);
    queue_damage(e, this, speed, second, first, first_body, second_body);
}

// Translated from 00623cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOCollisionListener::contactPointAddedCallback` (Xbox PDB, engine map):
/// skips events with a word at `+0x20`. The approach speed (`+0x1C`
/// through `004587d0` and `00408820`) at least `fMinSoundVel:Audio` and 1
/// plays a collision sound (`ImpactMixer::PlayCollisionSound` on a 0x24-byte
/// record: `0x0C` bytes set up by `006240d0` and `00458620` from the
/// contact point `+0x10`, the speed, the sound level from the property
/// `+0x18` (`fn_00624050`) or the `float` at `01017870`, each body's
/// material (`0062b150`), their product of `material + 1`, and each side's
/// object for single-body entities), unless both entities' roots
/// (`00624020`) are of the same type (`004a3a20` on `008c71b0` of
/// `0043b540`) that `00624070` excludes, or one side is excluded with
/// `006240b0` giving 0x11. In every case it then queues the collision
/// damage (`fn_006238b0`) and offers the event to both references
/// (`fn_006240f0`).
pub fn fo_collision_listener_contact_point_added_callback(e: &mut Engine, this: Ptr, event: Ptr) {
    let event = event.addr();
    if e.mem.u32(event.wrapping_add(0x20)) != 0 {
        return;
    }
    e.with_stack(0x24, |e, sound| {
        // The 0x24 bytes at [EBP-0x24]: the sound record.
        let sound = sound.addr();
        e.call(0x0062_40d0, &args![sound]);
        let approach = x87_quiet_bits(e.mem.u32(event.wrapping_add(0x1c)));
        let scaled = x87_store32(e.call(0x0045_87d0, &args![approach]).st0);
        let speed = x87_store32(e.call(0x0040_8820, &args![scaled]).st0);
        e.mem.set_u32(sound + 0xc, speed);
        let first_entity = e.mem.u32(event);
        let first_root = e.call(0x0062_4020, &args![first_entity]).u32();
        let second_entity = e.mem.u32(event.wrapping_add(4));
        let second_root = e.call(0x0062_4020, &args![second_entity]).u32();
        let minimum = e.call(0x0045_0410, &args![F_MIN_SOUND_VEL]).st0;
        let speed = x87_load32(e.mem.u32(sound + 0xc));
        let loud_enough = speed >= minimum && speed >= e.global::<f64>(ONE);
        let mut play = loud_enough;
        if loud_enough && first_root != 0 && second_root != 0 {
            play = e.with_stack(8, |e, flags| {
                let (first_flags, second_flags) = (flags.addr(), flags.addr() + 4);
                let kind = e.call(0x0043_b540, &args![first_root]).u32();
                e.call(0x008c_71b0, &args![first_flags, kind]);
                let kind = e.call(0x0043_b540, &args![second_root]).u32();
                e.call(0x008c_71b0, &args![second_flags, kind]);
                let first_kind = e.call(0x004a_3a20, &args![first_flags]).u32();
                let second_kind = e.call(0x004a_3a20, &args![second_flags]).u32();
                if first_kind == second_kind {
                    e.call(0x0062_4070, &args![first_flags]).u8() == 0
                } else if e.call(0x0062_4070, &args![first_flags]).u8() != 0 {
                    e.call(0x0062_40b0, &args![first_flags]).u32() != 0x11
                } else if e.call(0x0062_4070, &args![second_flags]).u8() != 0 {
                    e.call(0x0062_40b0, &args![second_flags]).u32() != 0x11
                } else {
                    true
                }
            });
            if !play {
                // The exe returns here, skipping the damage and the offers.
                return;
            }
        }
        if play {
            let point = e.mem.u32(event.wrapping_add(0x10));
            let point = e.call(SELF, &args![point]).u32();
            e.call(0x0045_8620, &args![sound, point]);
            let property = e.mem.u32(event.wrapping_add(0x18));
            let level = if property != 0 {
                let level = fn_00624050(e, Ptr::new(property.wrapping_add(8)));
                x87_store32(f64::from(level))
            } else {
                x87_quiet_bits(e.mem.u32(DEFAULT_SOUND_LEVEL))
            };
            e.mem.set_u32(sound + 0x10, x87_quiet_bits(level));
            let first_entity = e.mem.u32(event);
            let first_material = e.call(0x0062_b150, &args![first_entity, sound]).u8();
            e.mem.set_u8(sound + 0x14, first_material);
            let second_entity = e.mem.u32(event.wrapping_add(4));
            let second_material = e.call(0x0062_b150, &args![second_entity, sound]).u8();
            e.mem.set_u8(sound + 0x15, second_material);
            e.mem.set_u32(sound + 0x1c, 0);
            e.mem.set_u32(sound + 0x20, 0);
            for (root, offset) in [(first_root, 0x1c), (second_root, 0x20)] {
                if root != 0 && e.call(0x004b_5980, &args![root]).u32() == 1 {
                    let body = e.call(BODY_FROM_ENTITY, &args![root]).u32();
                    if body != 0 {
                        let object = e.call(BODY_OBJECT, &args![body]).u32();
                        e.mem.set_u32(sound + offset, object);
                    }
                }
            }
            let first = (i32::from(e.mem.u8(sound + 0x14) as i8)).wrapping_add(1);
            let second = (i32::from(e.mem.u8(sound + 0x15) as i8)).wrapping_add(1);
            e.mem
                .set_u32(sound + 0x18, first.wrapping_mul(second) as u32);
            e.call(0x0083_7550, &args![sound]);
        }
        let (first_body, first) = entity_reference(e, first_root);
        let (second_body, second) = entity_reference(e, second_root);
        let approach = x87_quiet_bits(e.mem.u32(event.wrapping_add(0x1c)));
        fn_006238b0(
            e,
            this,
            f32::from_bits(approach),
            first_body,
            second_body,
            first,
            second,
        );
        fn_006240f0(e, this, first, Ptr::new(event));
        fn_006240f0(e, this, second, Ptr::new(event));
    });
}

// Translated from 006240f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Offers a contact event to a reference: one that answers its vtable
/// +0x224 and whose +0x304 gives 3 gets `009b4950` with the event.
pub fn fn_006240f0(e: &mut Engine, _this: Ptr, reference: u32, event: Ptr) {
    if reference != 0
        && e.vcall(reference, 0x224, &[]).u8() != 0
        && e.vcall(reference, 0x304, &[]).u32() == 3
    {
        e.call(0x009b_4950, &args![reference, event]);
    }
}

/// The free (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// The destructor bodies of the `hkpContactListener` base (`006234f0`) and
/// of the next base (`006234d0`), not this task's.
const CONTACT_LISTENER_DESTRUCTOR: u32 = 0x0062_34f0;
const LISTENER_BASE_DESTRUCTOR: u32 = 0x0062_34d0;
/// `FOCollisionListener`'s vtable (RTTI `.?AVFOCollisionListener@@`).
const LISTENER_VTABLE: u32 = 0x0104_dd38;
/// The word the destructor clears (`011cc224`).
const LISTENER_WORD: u32 = 0x011c_c224;
/// `0063f7b0` (`ECX` the list): removes the list's first item;
/// `0046ffb0` (`ECX`): the list's destructor.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// The damage record's members: the `NiPointer` constructor (`00633c90`)
/// and destructor (`0045cec0`), the `NiPointer` assignment (`0066b0d0`)
/// and the cause pointer at +8 (`0042f730` constructor, `0042f760`
/// destructor, `NiPointer<ActorCause>::operator=` `0042f780`).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const CAUSE_CONSTRUCT: u32 = 0x0042_f730;
const CAUSE_DESTRUCT: u32 = 0x0042_f760;
const CAUSE_ASSIGN: u32 = 0x0042_f780;

// Translated from 00623510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContactListener` scalar deleting destructor (engine map; `RET 4`):
/// the body (`006234f0`), then the free when bit 0 is set. Returns `this`.
pub fn hkp_contact_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(CONTACT_LISTENER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00623540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The listener base's scalar deleting destructor (`RET 4`; the engine map
/// has no name): the body (`006234d0`), then the free when bit 0 is set.
/// Returns `this`.
pub fn fn_00623540(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(LISTENER_BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00623590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOCollisionListener` scalar deleting destructor (engine map; `RET 4`):
/// [`fn_006235c0`], then the free when bit 0 is set. Returns `this`.
pub fn fo_collision_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006235c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006235c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006235c0 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// The `FOCollisionListener` destructor body (`ECX`; the engine map has no
/// name): its vtable (`0104dd38`), the word at `011cc224` cleared, the
/// queued records deleted ([`fn_00623c50`]), the list at +4 destroyed
/// (`0046ffb0`) and the base's body (`006234d0`).
pub fn fn_006235c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LISTENER_VTABLE);
    e.set_global(LISTENER_WORD, 0u32);
    fn_00623c50(e, this);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
    e.call(LISTENER_BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00623820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The damage record's scalar deleting destructor (`RET 4`; the engine map
/// has no name): [`fn_00623850`], then the free when bit 0 is set. Returns
/// `this`.
pub fn fn_00623820(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00623850(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00623850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00623850 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// The damage record's destructor body (`ECX`; the engine map has no
/// name): the cause pointer at +8 (`0042f760`), then the `NiPointer` at +0
/// (`0045cec0`).
pub fn fn_00623850(e: &mut Engine, this: Ptr) {
    e.call(CAUSE_DESTRUCT, &args![this.addr() + 8]);
    e.call(POINTER_DESTRUCT, &args![this]);
}

// Translated from 00623bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00623bd0 - the FS:[0] exception frame (C++ unwinding during construction) is not translated
/// The damage record's constructor (`ECX`; the engine map has no name):
/// the `NiPointer` at +0 and the cause pointer at +8 made empty
/// (`00633c90`, `0042f730` with 0), the `NiPointer` assigned 0
/// (`0066b0d0`), the `float` at +4 0.0 and the cause assigned 0
/// (`0042f780`). Returns `this`.
pub fn fn_00623bd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(POINTER_CONSTRUCT, &args![this, 0u32]);
    e.call(CAUSE_CONSTRUCT, &args![base + 8, 0u32]);
    e.call(POINTER_ASSIGN, &args![this, 0u32]);
    e.mem.set_u32(base + 4, 0.0f32.to_bits());
    e.call(CAUSE_ASSIGN, &args![base + 8, 0u32]);
    this
}

// Translated from 00623c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the listener's record list (+4; `ECX`; the engine map has no
/// name): while the list is not empty (`008256d0`), its first item
/// (`006815c0`'s word) is deleted ([`fn_00623820`] with 1) when there is
/// one, and the head removed (`0063f7b0`).
pub fn fn_00623c50(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 4;
    while !e.call(LIST_NODE_EMPTY, &args![list]).bool() {
        let node = e.call(SELF, &args![list]).u32();
        let item = e.mem.u32(node);
        if item != 0 {
            fn_00623820(e, Ptr::new(item), 1);
        }
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0062_3510,
            hkp_contact_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0062_3540, fn_00623540(Ptr, u32) -> Ptr),
        entry!(
            0x0062_3590,
            fo_collision_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0062_35c0, fn_006235c0(Ptr)),
        entry!(0x0062_3820, fn_00623820(Ptr, u32) -> Ptr),
        entry!(0x0062_3850, fn_00623850(Ptr)),
        entry!(0x0062_3bd0, fn_00623bd0(Ptr) -> Ptr),
        entry!(0x0062_3c50, fn_00623c50(Ptr)),
        entry!(0x0062_4050, fn_00624050(Ptr) -> f32),
        entry!(0x0062_3570, fn_00623570(Ptr) -> Ptr),
        entry!(0x0062_3640, fn_00623640(Ptr)),
        entry!(0x0062_38b0, fn_006238b0(Ptr, f32, u32, u32, u32, u32)),
        entry!(
            0x0062_3cb0,
            fo_collision_listener_contact_point_added_callback(Ptr, Ptr)
        ),
        entry!(0x0062_40f0, fn_006240f0(Ptr, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00624050_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00577BF0, |_, a| Ret {
            st0: a[0] as f64 / 4.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_4050, &args![0x4100u32]).f32(), 4161.0);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00577BF0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An object whose vtable slots `slot` point at `0x7000_0000 + slot`
    /// (doubles returning 0 unless replaced).
    fn object(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x400);
        for slot in (0..0x400).step_by(4) {
            e.mem.set_u32(table + slot, 0x7000_0000 + slot);
            e.register(0x7000_0000 + slot, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, table);
        this
    }

    #[test]
    fn contact_listener_base_stores_its_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x0062_3570, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CONTACT_LISTENER_VTABLE);
    }

    #[test]
    fn contact_is_offered_to_type_three_references() {
        let mut e = Engine::new();
        let reference = object(&mut e);
        e.register(0x009b_4950, |_, _| Ret::default());
        e.register(0x7000_0224, |_, _| 1u32.into_ret());
        e.register(0x7000_0304, |_, _| 3u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_40f0, &args![0u32, reference, 0x5000u32]);
        e.call(0x0062_40f0, &args![0u32, 0u32, 0x5000u32]);
        e.register(0x7000_0304, |_, _| 2u32.into_ret());
        e.call(0x0062_40f0, &args![0u32, reference, 0x5000u32]);
        assert_eq!(calls_to(&e, 0x009b_4950), vec![vec![reference, 0x5000]]);
    }

    fn damage_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_2000, 0x0102_2000, 0x011c_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.mem.set_f32(DEFAULT_MASS, 12.5);
        e.register(0x0040_8820, |_, a| {
            f64::from(f32::from_bits(a[0])).into_ret()
        });
        e.register(0x0040_3e20, |_, a| (a[0] + 4).into_ret());
        e.register(0x0045_2370, |_, _| 1u32.into_ret());
        e.register(0x0047_7ba0, |_, _| 0u32.into_ret());
        e.register(0x004b_4ea0, |_, _| 0.0f64.into_ret());
        e.register(0x0062_be90, |_, a| {
            (f64::from(f32::from_bits(a[0])) * f64::from(f32::from_bits(a[1]))).into_ret()
        });
        e.register(0x0040_1000, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0062_3bd0, |_, a| a[0].into_ret());
        e.register(BODY_OBJECT, |_, a| (a[0] + 1).into_ret());
        e.register(0x0066_b0d0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0042_f780, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x005a_e3d0, |_, _| Ret::default());
        e
    }

    #[test]
    fn damage_is_queued_for_the_first_side() {
        let mut e = damage_engine();
        let this = e.mem.alloc(0x10);
        let first = object(&mut e);
        e.mem.set_f32(F_PHYSICS_DAMAGE_SPEED_MIN + 4, 5.0);
        e.call_log = Some(vec![]);
        e.call(
            0x0062_38b0,
            &args![this, 10.0f32, 0x100u32, 0x200u32, first, 0u32],
        );
        let added = calls_to(&e, 0x005a_e3d0);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], this + 4);
        let record = calls_to(&e, 0x0062_3bd0)[0][0];
        assert_eq!(e.mem.u32(record), 0x101);
        assert_eq!(e.mem.f32(record + 4), 125.0);
        assert_eq!(e.mem.u32(record + 8), 0);
        // The mass came from the second body.
        assert_eq!(calls_to(&e, 0x004b_4ea0), vec![vec![0x200]]);
        // Too slow: nothing.
        e.call_log = Some(vec![]);
        e.call(
            0x0062_38b0,
            &args![this, 4.0f32, 0x100u32, 0x200u32, first, 0u32],
        );
        assert!(calls_to(&e, 0x005a_e3d0).is_empty());
        // An excluded other side stops the first side, and excludes the
        // second side itself.
        let second = object(&mut e);
        e.register_double(0x7000_0224, move |_, a| {
            u32::from(a[0] == second).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x0062_38b0,
            &args![this, 10.0f32, 0x100u32, 0x200u32, first, second],
        );
        assert!(calls_to(&e, 0x005a_e3d0).is_empty());
    }

    #[test]
    fn queued_damage_is_applied_and_reported() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0102_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.mem.set_f32(PLAYER_DAMAGE, 3.0);
        e.set_global(PLAYER, 0x0abc_0000u32);
        let this = e.mem.alloc(0x10);
        let reference = object(&mut e);
        let record = e.mem.alloc(0xc);
        let cause = e.mem.alloc(4);
        e.mem.set_u32(cause, 0x0abc_0000);
        e.mem.set_u32(record, 0x0111_0000);
        e.mem.set_f32(record + 4, 2.5);
        e.mem.set_u32(record + 8, cause);
        e.mem.set_u32(this + 4, record);
        // The node empties after the first record.
        let mut empty_checks = 0;
        e.register_double(LIST_NODE_EMPTY, move |_, _| {
            empty_checks += 1;
            u32::from(empty_checks > 1).into_ret()
        });
        e.register(SELF, |_, a| a[0].into_ret());
        e.register(0x0063_f7b0, |_, _| Ret::default());
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x004a_e750, |_, _| 0x10u32.into_ret());
        e.register(BODY_NODE, |_, _| 0x20u32.into_ret());
        e.register(NODE_OBJECT, |_, _| 0x30u32.into_ret());
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, _| reference.into_ret());
        e.register(0x008c_7aa0, |_, _| 0u32.into_ret());
        e.register(0x0047_7ba0, |_, _| 1u32.into_ret());
        e.register(0x0096_3eb0, |_, _| Ret::default());
        e.register(0x0062_3820, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_3640, &args![this]);
        assert_eq!(calls_to(&e, 0x7000_0164), vec![vec![reference, cause]]);
        assert_eq!(
            calls_to(&e, 0x7000_0144),
            vec![vec![reference, 2.5f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, 0x0096_3eb0),
            vec![vec![0x0abc_0000, 0xa, 3.0f32.to_bits(), 0]]
        );
        assert_eq!(calls_to(&e, 0x0062_3820), vec![vec![record, 1]]);
    }

    fn contact_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [0x0101_2000, 0x0101_7000, 0x011c_c000, 0x011c_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE, 1.0f64);
        e.set_global(ZERO, 0.0f64);
        e.mem.set_f32(DEFAULT_SOUND_LEVEL, 0.75);
        // Damage needs a speed no contact reaches.
        e.mem.set_f32(F_PHYSICS_DAMAGE_SPEED_MIN + 4, 1.0e9);
        e.register(0x0062_40d0, |_, a| a[0].into_ret());
        e.register(0x0045_87d0, |_, a| {
            f64::from(f32::from_bits(a[0])).into_ret()
        });
        e.register(0x0040_8820, |_, a| {
            f64::from(f32::from_bits(a[0])).into_ret()
        });
        e.register(0x0062_4020, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0045_0410, |_, _| 2.0f64.into_ret());
        e.register(0x0040_3e20, |_, a| (a[0] + 4).into_ret());
        e.register(SELF, |_, a| a[0].into_ret());
        e.register(0x0045_8620, |_, _| Ret::default());
        e.register(0x0062_b150, |_, a| (a[0] & 0xff).into_ret());
        e.register(0x0083_7550, |e, a| {
            // Keep a copy of the sound record for the test.
            let copy = e.mem.bytes(a[0], 0x24);
            e.mem.write(0x011c_c800, &copy);
            Ret::default()
        });
        e.register(BODY_FROM_ENTITY, |_, _| 0u32.into_ret());
        let event = e.mem.alloc(0x24);
        (e, event)
    }

    #[test]
    fn contact_plays_a_sound_and_offers_damage() {
        let (mut e, event) = contact_engine();
        let this = e.mem.alloc(0x10);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(event, first);
        e.mem.set_u32(event + 4, second);
        e.mem.set_f32(event + 0x1c, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x0062_3cb0, &args![this, event]);
        assert_eq!(calls_to(&e, 0x0083_7550).len(), 1);
        let sound = 0x011c_c800;
        assert_eq!(e.mem.f32(sound + 0xc), 3.0);
        assert_eq!(e.mem.f32(sound + 0x10), 0.75);
        let (a, b) = (first & 0xff, second & 0xff);
        assert_eq!(u32::from(e.mem.u8(sound + 0x14)), a);
        let expected = (i32::from(a as u8 as i8) + 1) * (i32::from(b as u8 as i8) + 1);
        assert_eq!(e.mem.u32(sound + 0x18), expected as u32);
        // Both sides were looked up for damage (null roots included).
        assert_eq!(calls_to(&e, BODY_FROM_ENTITY), vec![vec![0], vec![0]]);
        // Too slow: no sound, damage still offered.
        e.mem.set_f32(event + 0x1c, 1.5);
        e.call_log = Some(vec![]);
        e.call(0x0062_3cb0, &args![this, event]);
        assert!(calls_to(&e, 0x0083_7550).is_empty());
        assert_eq!(calls_to(&e, BODY_FROM_ENTITY).len(), 2);
        // A flagged event: nothing at all.
        e.mem.set_u32(event + 0x20, 1);
        e.call_log = Some(vec![]);
        e.call(0x0062_3cb0, &args![this, event]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn contact_between_excluded_roots_returns_early() {
        let (mut e, event) = contact_engine();
        let this = e.mem.alloc(0x10);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(event, first);
        e.mem.set_u32(event + 4, second);
        e.mem.set_u32(first, 0x500);
        e.mem.set_u32(second, 0x600);
        e.mem.set_f32(event + 0x1c, 3.0);
        e.register(0x0043_b540, |_, a| a[0].into_ret());
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x004a_3a20, |_, _| 7u32.into_ret());
        e.register(0x0062_4070, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_3cb0, &args![this, event]);
        assert!(calls_to(&e, 0x0083_7550).is_empty());
        assert!(calls_to(&e, BODY_FROM_ENTITY).is_empty());
        // Different kinds, the first excluded with 0x11: also early.
        e.register(0x004a_3a20, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0062_40b0, |_, _| 0x11u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_3cb0, &args![this, event]);
        assert!(calls_to(&e, BODY_FROM_ENTITY).is_empty());
        // Not 0x11: the sound plays.
        e.register(0x0062_40b0, |_, _| 3u32.into_ret());
        e.register(0x004b_5980, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_3cb0, &args![this, event]);
        assert_eq!(calls_to(&e, 0x0083_7550).len(), 1);
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn base_scalar_deleting_destructors() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            quiet(
                &mut e,
                &[CONTACT_LISTENER_DESTRUCTOR, LISTENER_BASE_DESTRUCTOR, FREE],
            );
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0062_3510, &args![0x7000u32, flags]).u32(), 0x7000);
            assert_eq!(e.call(0x0062_3540, &args![0x7000u32, flags]).u32(), 0x7000);
            assert_eq!(calls(&e, CONTACT_LISTENER_DESTRUCTOR), vec![vec![0x7000]]);
            assert_eq!(calls(&e, LISTENER_BASE_DESTRUCTOR), vec![vec![0x7000]]);
            assert_eq!(calls(&e, FREE).len(), 2 * flags as usize);
        }
    }

    #[test]
    fn listener_destructor_deletes_the_queued_records() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.map(0x011c_c000, 0x1000);
            e.set_global(LISTENER_WORD, 5u32);
            let this = e.mem.alloc(0x10);
            // A list of two nodes: one record, then an empty item.
            let record = e.mem.alloc(0x10);
            let nodes = [record, 0u32];
            let left = std::rc::Rc::new(std::cell::RefCell::new(2usize));
            let l = left.clone();
            e.register_double(LIST_NODE_EMPTY, move |_, _| {
                u32::from(*l.borrow() == 0).into_ret()
            });
            let l = left.clone();
            let slot = e.mem.alloc(4);
            e.register_double(SELF, move |e, _| {
                e.mem.set_u32(slot, nodes[2 - *l.borrow()]);
                slot.into_ret()
            });
            let l = left;
            e.register_double(LIST_REMOVE_HEAD, move |_, _| {
                *l.borrow_mut() -= 1;
                Ret::default()
            });
            quiet(
                &mut e,
                &[
                    CAUSE_DESTRUCT,
                    POINTER_DESTRUCT,
                    FREE,
                    LIST_DESTRUCT,
                    LISTENER_BASE_DESTRUCTOR,
                ],
            );
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0062_3590, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), LISTENER_VTABLE);
            assert_eq!(e.global::<u32>(LISTENER_WORD), 0);
            assert_eq!(calls(&e, CAUSE_DESTRUCT), vec![vec![record + 8]]);
            assert_eq!(calls(&e, POINTER_DESTRUCT), vec![vec![record]]);
            assert_eq!(calls(&e, LIST_REMOVE_HEAD), vec![vec![this + 4]; 2]);
            assert_eq!(calls(&e, LIST_DESTRUCT), vec![vec![this + 4]]);
            assert_eq!(calls(&e, LISTENER_BASE_DESTRUCTOR), vec![vec![this]]);
            // The record is always freed; the listener only with bit 0.
            let mut freed = vec![vec![record]];
            if flags & 1 != 0 {
                freed.push(vec![this]);
            }
            assert_eq!(calls(&e, FREE), freed);
        }
    }

    #[test]
    fn record_constructor_and_destructor() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                POINTER_CONSTRUCT,
                CAUSE_CONSTRUCT,
                POINTER_ASSIGN,
                CAUSE_ASSIGN,
                CAUSE_DESTRUCT,
                POINTER_DESTRUCT,
                FREE,
            ],
        );
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 4, 0xffff_ffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_3bd0, &args![record]).u32(), record);
        assert_eq!(e.mem.u32(record + 4), 0);
        assert_eq!(calls(&e, POINTER_CONSTRUCT), vec![vec![record, 0]]);
        assert_eq!(calls(&e, CAUSE_CONSTRUCT), vec![vec![record + 8, 0]]);
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![record, 0]]);
        assert_eq!(calls(&e, CAUSE_ASSIGN), vec![vec![record + 8, 0]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_3820, &args![record, 0u32]).u32(), record);
        assert!(calls(&e, FREE).is_empty());
        e.call(0x0062_3850, &args![record]);
        assert_eq!(calls(&e, CAUSE_DESTRUCT).len(), 2);
    }
}
