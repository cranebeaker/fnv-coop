//! `Havok SDK/hkpsimplecontactconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl: allocator, the array, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `hkpSimpleContactConstraintAtomUtil::allocateAtom` (`00d92510`, cdecl).
const ALLOCATE_ATOM: u32 = 0x00d9_2510;
/// The collision-response callbacks of the atoms in a contact constraint's chain (thiscall on
/// the atom): begin callbacks take five words, end callbacks six. The atom type (`u16` at
/// +0) selects them: `0x17`, `0x18`, `0x1a` and `0x1c`.
const BEGIN_CALLBACK_TYPE_17: u32 = 0x00d2_1f10;
const BEGIN_CALLBACK_TYPE_18: u32 = 0x00d2_1e00;
const BEGIN_CALLBACK_TYPE_1A: u32 = 0x00d2_1f50;
const BEGIN_CALLBACK_TYPE_1C: u32 = 0x00d2_2160;
const END_CALLBACK_TYPE_17: u32 = 0x00d2_1d10;
const END_CALLBACK_TYPE_18: u32 = 0x00d2_1d00;
const END_CALLBACK_TYPE_1A: u32 = 0x00d2_1fc0;
const END_CALLBACK_TYPE_1C: u32 = 0x00d2_2260;

// Translated from 00d927b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getConstraintInfo` (Xbox PDB): calls
/// `hkpConstraintData::getConstraintInfoUtil(atom, u16[atom + 2], info)` with the atom at +0x28.
pub fn hkp_simple_contact_constraint_data_get_constraint_info(
    e: &mut Engine,
    this: Ptr,
    info: Ptr,
) {
    let atom = e.mem.u32(this.addr() + 0x28);
    let size = u32::from(e.mem.u16(atom + 2));
    e.call(GET_CONSTRAINT_INFO_UTIL, &args![atom, size, info]);
}

// Translated from 00d927d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getRuntimeInfo` (Xbox PDB): clears the two words of the
/// runtime info `info`. Register value first: ECX (`this`) is never read; the first stack word
/// is never read either. Returns `info` (the exe leaves it in EAX).
pub fn hkp_simple_contact_constraint_data_get_runtime_info(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    info: Ptr,
) -> Ptr {
    e.mem.set_u32(info.addr(), 0);
    e.mem.set_u32(info.addr() + 4, 0);
    info
}

// Translated from 00d927f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getSolverResults` (Xbox PDB): answers zero. Register value
/// first: ECX (`this`) is never read; the stack word is never read either.
pub fn hkp_simple_contact_constraint_data_get_solver_results(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
) -> u32 {
    0
}

// Translated from 00d92800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::isValid` (Xbox PDB): stores `1` in the byte `valid` points
/// to. Register value first: ECX (`this`) is never read. Returns `valid` (the exe leaves it in
/// EAX).
pub fn hkp_simple_contact_constraint_data_is_valid(e: &mut Engine, _this: Ptr, valid: Ptr) -> Ptr {
    e.mem.set_u8(valid.addr(), 1);
    valid
}

// Translated from 00d92810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getType` (Xbox PDB): answers the constraint type `0xb`.
/// Register value first: ECX (`this`) is never read.
pub fn hkp_simple_contact_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0xb
}

// Translated from 00d92820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getSimpleConstraintContactMgr` (Xbox PDB): the contact
/// manager this data is embedded in, at `this - 0x14`.
pub fn hkp_simple_contact_constraint_data_get_simple_constraint_contact_mgr(
    _e: &mut Engine,
    this: Ptr,
) -> Ptr {
    Ptr::new(this.addr().wrapping_sub(0x14))
}

// Translated from 00d92830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::collisionResponseBeginCallback` (Xbox PDB): walks the atom
/// chain starting at the pointer at +0x10 of the object at `this + 0x24` (nothing when it is
/// null); while the atom's `u16` type is at least `0x17`, calls the begin callback of its type
/// on the atom (`0x17`: `00d21f10`, `0x18`: `00d21e00`, `0x1a`: `00d21f50`, `0x1c`:
/// `00d22160`; other types none) with the five stack words, then moves to the atom at +0x14.
pub fn hkp_simple_contact_constraint_data_collision_response_begin_callback(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
) {
    let owner = e.mem.u32(this.addr() + 0x24);
    let mut atom = e.mem.u32(owner + 0x10);
    if atom == 0 || e.mem.u16(atom) < 0x17 {
        return;
    }
    loop {
        let callback = match e.mem.u16(atom) {
            0x17 => Some(BEGIN_CALLBACK_TYPE_17),
            0x18 => Some(BEGIN_CALLBACK_TYPE_18),
            0x1a => Some(BEGIN_CALLBACK_TYPE_1A),
            0x1c => Some(BEGIN_CALLBACK_TYPE_1C),
            _ => None,
        };
        if let Some(callback) = callback {
            e.call(
                callback,
                &args![atom, word_1, word_2, word_3, word_4, word_5],
            );
        }
        atom = e.mem.u32(atom + 0x14);
        if e.mem.u16(atom) < 0x17 {
            break;
        }
    }
}

// Translated from 00d92f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::hkpSimpleContactConstraintData` (Xbox PDB, unnamed in the
/// map), `(owner, body_a, body_b)`: sets the vtable `0x010c_c934`, `1` in the `u16` at +6,
/// the empty inplace array at +0xc (data `this + 0x18`, size 0, capacity word `0x80000008`),
/// `owner` at +0x24, allocates the contact atom (`allocateAtom(1, byte[body_a + 0xd1],
/// byte[body_b + 0xd1], 0x3400 / ((sum of the bytes) * 4 + 0x34))`, the quotient as a 16-bit
/// value) into +0x28, clears its `u16` at +0x10, sets its `u16` at +0x12 to `3`, clears
/// its words +0x14 to +0x2c and stores the `u16` at atom + 2 in +0x2c. Returns `this`.
pub fn fn_00d92f20(e: &mut Engine, this: Ptr, owner: u32, body_a: u32, body_b: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x010c_c934);
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, base + 0x18);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0x8000_0008);
    e.mem.set_u32(base + 0x24, owner);
    let bytes_a = u32::from(e.mem.u8(body_a + 0xd1));
    let bytes_b = u32::from(e.mem.u8(body_b + 0xd1));
    let divisor = (bytes_b + bytes_a) * 4 + 0x34;
    let quotient = 0x3400 / divisor;
    let atom = e
        .call(
            ALLOCATE_ATOM,
            &args![1u32, bytes_a, bytes_b, quotient & 0xffff],
        )
        .u32();
    e.mem.set_u32(base + 0x28, atom);
    e.mem.set_u16(atom + 0x10, 0);
    e.mem.set_u16(atom + 0x12, 3);
    for offset in (0x14..0x30).step_by(4) {
        e.mem.set_u32(atom + offset, 0);
    }
    let size = u32::from(e.mem.u16(atom + 2));
    e.mem.set_u32(base + 0x2c, size);
    this
}

// Translated from 00d92fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::collisionResponseEndCallback` (Xbox PDB): collects the atom
/// chain (as in the begin callback, from the pointer at +0x10 of the object at `this + 0x24`;
/// nothing when it is null) into an inplace array of sixteen pointers on the stack (grown
/// through `00c90940` when full) while the atom's `u16` type is at least `0x17`, then calls
/// the end callback of each atom's type from the last atom to the first (`0x17`: `00d21d10`,
/// `0x18`: `00d21d00`, `0x1a`: `00d21fc0`, `0x1c`: `00d22260`; other types none) with
/// `(word_1, value, word_3, word_4, word_5, word_6)`, `value` through the x87 stack; the array
/// is emptied and, when its capacity word is not negative, freed through the allocator
/// (slot `0x10`, capacity masked with `0x3fffffff`, times 4).
pub fn hkp_simple_contact_constraint_data_collision_response_end_callback(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    value: f32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
) {
    let owner = e.mem.u32(this.addr() + 0x24);
    let first = e.mem.u32(owner + 0x10);
    if first == 0 {
        return;
    }
    e.with_stack(0x4c, |e, array| {
        let list = array.addr();
        e.mem.set_u32(list, list + 0xc);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0010);
        let mut atom = first;
        if e.mem.u16(atom) >= 0x17 {
            loop {
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let capacity = e.mem.u32(list + 8) & 0x3fff_ffff;
                if e.mem.u32(list + 4) == capacity {
                    let allocator = e.mem.u32(router + 0x10);
                    e.call(ARRAY_RESERVE_MORE, &args![allocator, list, 4u32]);
                }
                let size = e.mem.u32(list + 4);
                let data = e.mem.u32(list);
                e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), atom);
                e.mem.set_u32(list + 4, size.wrapping_add(1));
                atom = e.mem.u32(atom + 0x14);
                if e.mem.u16(atom) < 0x17 {
                    break;
                }
            }
        }
        let mut index = e.mem.i32(list + 4).wrapping_sub(1);
        while index >= 0 {
            let data = e.mem.u32(list);
            let atom = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let callback = match e.mem.u16(atom) {
                0x17 => Some(END_CALLBACK_TYPE_17),
                0x18 => Some(END_CALLBACK_TYPE_18),
                0x1a => Some(END_CALLBACK_TYPE_1A),
                0x1c => Some(END_CALLBACK_TYPE_1C),
                _ => None,
            };
            if let Some(callback) = callback {
                e.call(
                    callback,
                    &args![
                        atom,
                        word_1,
                        x87_quiet(value),
                        word_3,
                        word_4,
                        word_5,
                        word_6
                    ],
                );
            }
            index -= 1;
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
    });
}

// Translated from 00d93140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDynamicsCpIdMgr::newId` (Xbox PDB): hands out an id for `value`. Looks from the last
/// entry of the byte table (data +0, size +4) down for a free entry (`0xff`) and stores `value`
/// there; without one, grows the table when it is full (`00c90940(allocator, table, 1)`),
/// appends `value` and answers the new last index. Answers the index.
pub fn hkp_dynamics_cp_id_mgr_new_id(e: &mut Engine, this: Ptr, value: u8) -> i32 {
    let base = this.addr();
    let mut index = e.mem.i32(base + 4).wrapping_sub(1);
    if index >= 0 {
        let table = e.mem.u32(base);
        loop {
            let slot = table.wrapping_add(index as u32);
            if e.mem.u8(slot) == 0xff {
                e.mem.set_u8(slot, value);
                return index;
            }
            index = index.wrapping_sub(1);
            if index < 0 {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(base + 8) & 0x3fff_ffff;
    if e.mem.u32(base + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, this, 1u32]);
    }
    let slot = e.mem.u32(base).wrapping_add(e.mem.u32(base + 4));
    e.mem.set_u8(slot, value);
    let size = e.mem.u32(base + 4).wrapping_add(1);
    e.mem.set_u32(base + 4, size);
    size.wrapping_sub(1) as i32
}

/// `hkpSimpleContactConstraintAtomUtil::expandOne` (`00d92690`, cdecl `(atom, flag byte pointer)`,
/// answers the atom, possibly moved; the flag byte is set when it was), `hkpConstraintOwner::
/// addConstraintInfo` (`00cb74d0`, `this` = the owner, `(constraint, info)`) and
/// `hkpWorldConstraintUtil::updateFatherOfMovedAtom` (`00d275a0`, cdecl `(constraint, old atom,
/// new atom, size)`).
const EXPAND_ONE: u32 = 0x00d9_2690;
const OWNER_ADD_CONSTRAINT_INFO: u32 = 0x00cb_74d0;
const UPDATE_FATHER_OF_MOVED_ATOM: u32 = 0x00d2_75a0;

// Translated from 00d92df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::allocateContactPoint` (Xbox PDB), thiscall with three stack
/// words `(owner, point out, properties out)`: sets bit 2 of the atom flags at +0x10 of the atom
/// at `this + 0x28`, makes room for one more contact point there (`expandOne`), clears the new
/// point properties' first two words and sets their flag byte at +0xf to 1 (3 when there is an
/// earlier point whose flag byte has bit 1 clear and whose byte at +0xe is 0), stores the
/// addresses of the new contact point (atom + 0x30 + 0x20 * index) and of its properties in the
/// two outputs, tells the owner the constraint grew (`addConstraintInfo`, four words: the
/// bytes of the larger runtime block, the size delta of the atom and two counters, derived from
/// the old count), fixes up the constraint when the atom moved and returns the id
/// (`hkpDynamicsCpIdMgr::newId` of the id manager at `this + 0xc`).
pub fn hkp_simple_contact_constraint_data_allocate_contact_point(
    e: &mut Engine,
    this: Ptr,
    owner: Ptr,
    point_out: Ptr,
    properties_out: Ptr,
) -> i32 {
    // The exe's frame (`SUB ESP,0x18`, four pushes): 0x28 bytes, the moved flag at +0x13
    // (00d92e08), the constraint info at +0x18 (00d92e8d..00d92ec1).
    e.with_stack(0x28, |e, frame| {
        let frame = frame.addr();
        let data = this.addr();
        let atom = e.mem.u32(data + 0x28);
        let flags = e.mem.u16(atom + 0x10);
        e.mem.set_u16(atom + 0x10, flags | 4);
        let count = u32::from(e.mem.u16(atom + 4));
        let old_atom = e.mem.u32(data + 0x28);
        let moved = frame + 0x13;
        let new_atom = e.call(EXPAND_ONE, &args![old_atom, moved]).u32();
        e.mem.set_u32(data + 0x28, new_atom);
        let stride = u32::from(e.mem.u8(new_atom + 0xa));
        let points = (u32::from(e.mem.u16(new_atom + 6)) << 5).wrapping_add(new_atom);
        let properties = stride
            .wrapping_mul(count)
            .wrapping_add(points)
            .wrapping_add(0x30);
        let point = (count << 5).wrapping_add(new_atom).wrapping_add(0x30);
        let previous = properties.wrapping_sub(stride);
        e.mem.set_u32(properties, 0);
        e.mem.set_u32(properties + 4, 0);
        e.mem.set_u8(properties + 0xf, 1);
        if (count as i32) > 0 && e.mem.u8(previous + 0xf) & 2 == 0 && e.mem.u8(previous + 0xe) == 0
        {
            e.mem.set_u8(properties + 0xf, 3);
        }
        e.mem.set_u32(point_out.addr(), point);
        e.mem.set_u32(properties_out.addr(), properties);
        let next = count.wrapping_add(1);
        let odd = next & 1;
        let mut info = [
            (((next as i32) >> 1) as u32)
                .wrapping_mul(0x70)
                .wrapping_add(odd * 0x30)
                .wrapping_add(0xa0),
            0,
            0,
            0,
        ];
        if count == 1 {
            info[1] = 0x20;
            info[2] = 1;
            info[3] = 1;
        }
        info[2] += 1;
        info[1] += if odd != 0 { 0x30 } else { 0x40 };
        info[3] += 1;
        let info_at = frame + 0x18;
        for (i, word) in info.into_iter().enumerate() {
            e.mem.set_u32(info_at + 4 * i as u32, word);
        }
        let constraint = e.mem.u32(data + 0x24);
        e.call(
            OWNER_ADD_CONSTRAINT_INFO,
            &args![owner, constraint, info_at],
        );
        if e.mem.u8(moved) != 0 {
            let size = u32::from(e.mem.u16(new_atom + 2));
            let current = e.mem.u32(data + 0x28);
            e.call(
                UPDATE_FATHER_OF_MOVED_ATOM,
                &args![constraint, old_atom, current, size],
            );
            e.mem
                .set_u32(data + 0x2c, u32::from(e.mem.u16(new_atom + 2)));
        }
        hkp_dynamics_cp_id_mgr_new_id(e, Ptr::new(data + 0xc), count as u8)
    })
}

/// Callees of `fireCallbacks`: the impulse estimate `00df2510` (cdecl `(entity a, entity b,
/// motion a + 0x40, motion b + 0x40, contact point)`, answers `ST0`),
/// `hkpWorldCallbackUtil::fireContactPointCallback` (`00d00ca0`, cdecl `(world, event)`),
/// `hkpEntityCallbackUtil::fireContactPointCallbackInternal` (`00cfff00`, cdecl `(entity,
/// event)`), `hkpSimpleCollisionResponse::solveSingleContact2` (`00d9ee50`, cdecl, eight
/// words) and `hkpWorldCallbackUtil::fireContactImpulseLimitBreached` (`00d01700`, cdecl
/// `(world, event, count)`).
const ESTIMATE_IMPULSE: u32 = 0x00df_2510;
const FIRE_CONTACT_POINT_CALLBACK: u32 = 0x00d0_0ca0;
const FIRE_ENTITY_CONTACT_POINT_CALLBACK: u32 = 0x00cf_ff00;
const SOLVE_SINGLE_CONTACT: u32 = 0x00d9_ee50;
const FIRE_CONTACT_IMPULSE_LIMIT_BREACHED: u32 = 0x00d0_1700;
/// The constants of the contact response: `1/128` (`double`, `010cc8d0`), the restitution
/// limit (`float`, `01017870`), the small `double` added to the inverse masses (`0109cc78`),
/// the two `double` factors (`010cc8d8`, `010d3dc0`), the `float` stored in a contact's `+0x1c`
/// when its properties have bit 3 (`010cc3b0`), the default maximum impulse (`float`,
/// `0104da48`) and the table of `float` limits indexed by the property bytes (`010c7948`).
const RESTITUTION_SCALE: u32 = 0x010c_c8d0;
const RESTITUTION_LIMIT: u32 = 0x0101_7870;
const INVERSE_MASS_EPSILON: u32 = 0x0109_cc78;
const IMPULSE_FACTOR: u32 = 0x010c_c8d8;
const VELOCITY_FACTOR: u32 = 0x010d_3dc0;
const DISABLED_CONTACT_VALUE: u32 = 0x010c_c3b0;
const DEFAULT_MAX_IMPULSE: u32 = 0x0104_da48;
const LIMIT_TABLE: u32 = 0x010c_7948;

/// What `fireCallbacks` passes to every contact point event.
struct ContactEventContext {
    /// The stack frame: the event at `+0x6c`, the impulse at `+0x30`, the solver's parameters
    /// at `+0x6c` too and its output at `+0x4c`, the breach event at `+0x50` (the exe's
    /// offsets).
    frame: u32,
    data: Ptr,
    world: u32,
    entity_a: u32,
    entity_b: u32,
    word: u32,
    motion_a: u32,
    motion_b: u32,
}

/// Fills the contact point event at `frame + 0x6c` (type 2, the two entities, the contact
/// manager, `word`, the contact, its properties, `flag`, whether this is the first and the last
/// contact handled, `impulse`, 0, the contact's `+0x14`, the two motions) and fires it to the
/// world (`00d00ca0`), then as type 0 to entity a and as type 1 to entity b when their
/// listener counts (`u16` at `+0x204`) are not zero (`00cfff00`).
fn fire_contact_point(
    e: &mut Engine,
    context: &ContactEventContext,
    contact: (u32, u32),
    order: (bool, bool),
    flag: u8,
    impulse: u32,
) {
    let (point, properties) = contact;
    let (first, last) = order;
    let event = context.frame + 0x6c;
    e.mem.set_u32(event, 2);
    let manager =
        hkp_simple_contact_constraint_data_get_simple_constraint_contact_mgr(e, context.data);
    e.mem.set_u32(event + 0xc, manager.addr());
    e.mem.set_u32(event + 8, context.entity_b);
    e.mem.set_u32(event + 0x10, context.word);
    e.mem.set_u32(event + 0x14, point);
    e.mem.set_u8(event + 0x1c, flag);
    e.mem.set_u8(event + 0x1d, first as u8);
    e.mem.set_u32(event + 0x28, properties.wrapping_add(0x14));
    e.mem.set_u32(event + 0x2c, context.motion_a);
    e.mem.set_u8(event + 0x1e, last as u8);
    e.mem.set_u32(event + 0x20, impulse);
    e.mem.set_u32(event + 4, context.entity_a);
    e.mem.set_u32(event + 0x18, properties);
    e.mem.set_u32(event + 0x24, 0);
    e.mem.set_u32(event + 0x30, context.motion_b);
    e.call(FIRE_CONTACT_POINT_CALLBACK, &args![context.world, event]);
    e.mem.set_u32(event, 0);
    if e.mem.u16(context.entity_a + 0x204) != 0 {
        e.call(
            FIRE_ENTITY_CONTACT_POINT_CALLBACK,
            &args![context.entity_a, event],
        );
    }
    e.mem.set_u32(event, 1);
    if e.mem.u16(context.entity_b + 0x204) != 0 {
        e.call(
            FIRE_ENTITY_CONTACT_POINT_CALLBACK,
            &args![context.entity_b, event],
        );
    }
}

/// The response to one contact after its callbacks (the exe from `00d92abe`): `info` the
/// second argument, `impulse_bits` the estimated impulse stored at `frame + 0x30`.
fn respond_to_contact(
    e: &mut Engine,
    context: &ContactEventContext,
    info: u32,
    contact: (u32, u32),
    impulse_bits: u32,
) {
    let (point, properties) = contact;
    let frame = context.frame;
    let flags = properties + 0xf;
    if e.mem.u8(flags) & 8 != 0 {
        let value = x87_quiet_bits(e.mem.u32(DISABLED_CONTACT_VALUE));
        e.mem.set_u32(point + 0x1c, value);
        e.mem.set_u32(properties, 0);
        e.mem.set_u32(properties + 4, 0);
        e.mem.set_u32(properties + 0x10, 0);
        e.mem.set_u8(flags, e.mem.u8(flags) & 0xfe);
        return;
    }
    let restitution_byte = e.mem.u8(properties + 0xd);
    let collision_input = e.mem.u32(context.world + 0x78);
    let tolerances = e.mem.u32(collision_input + 0x68);
    let restitution_bits = x87_store32(x87_mul(
        f64::from(i32::from(restitution_byte)),
        e.mem.f64(RESTITUTION_SCALE),
    ));
    let restitution = x87_load32(restitution_bits);
    let impulse = x87_load32(impulse_bits);
    let negated_tolerance = -x87_load32(e.mem.u32(tolerances + 0x70));
    // `FCOMP` with `TEST AH,0x41` and then `TEST AH,5` / `JP`: the solver runs only when the
    // negated tolerance is above the impulse and the restitution above the limit (both
    // ordered).
    let solve =
        negated_tolerance > impulse && x87_load32(e.mem.u32(RESTITUTION_LIMIT)) < restitution;
    if solve {
        let parameters = frame + 0x6c;
        let limit_index = u32::from(e.mem.u8(properties + 0xc));
        let limit = x87_quiet_bits(e.mem.u32(LIMIT_TABLE + 4 * limit_index));
        e.mem.set_u32(parameters, limit);
        e.mem.set_u32(parameters + 4, x87_store32(restitution));
        e.mem.set_u32(parameters + 0x10, 0);
        e.mem.set_u32(parameters + 0x14, 0);
        e.mem.set_u32(parameters + 0xc, x87_store32(impulse));
        let maximum = x87_quiet_bits(e.mem.u32(DEFAULT_MAX_IMPULSE));
        e.mem.set_u32(parameters + 8, maximum);
        let maximum_index = u32::from(e.mem.u8(properties + 0xe));
        if maximum_index != 0 {
            let maximum = x87_quiet_bits(e.mem.u32(LIMIT_TABLE + 4 * maximum_index));
            e.mem.set_u32(parameters + 8, maximum);
        }
        let (motion_a, motion_b) = (e.mem.u32(info + 0x30), e.mem.u32(info + 0x34));
        e.call(
            SOLVE_SINGLE_CONTACT,
            &args![
                context.data,
                point,
                parameters,
                context.entity_a,
                context.entity_b,
                motion_a,
                motion_b,
                frame + 0x4c
            ],
        );
        if e.mem.u8(frame + 0x84) != 0 {
            let event = frame + 0x50;
            e.mem.set_u16(event, 1);
            let owner = e.mem.u32(info + 0x60);
            e.mem.set_u32(event + 4, owner);
            e.mem.set_u16(event + 2, 1);
            e.mem.set_u32(event + 0xc, point);
            e.mem.set_u32(event + 8, properties);
            e.call(
                FIRE_CONTACT_IMPULSE_LIMIT_BREACHED,
                &args![context.world, event, 1u32],
            );
        }
        e.mem.set_u32(properties, 0);
        e.mem.set_u32(properties + 0x10, 0);
        e.mem.set_u8(flags, e.mem.u8(flags) & 0xfe);
        return;
    }
    // The inverse masses are the fourth lanes of the entities' `+0x1a0` vectors.
    let inverse_mass_b = e.mem.u32(context.entity_b + 0x1ac);
    let inverse_mass_a = e.mem.u32(context.entity_a + 0x1ac);
    let sum = x87_store32(x87_add(x87_load32(inverse_mass_b), x87_m32(inverse_mass_a)));
    let sum = x87_add(x87_load32(sum), e.mem.f64(INVERSE_MASS_EPSILON));
    let inverse = x87_store32(x87_div(1.0, sum));
    let scaled = x87_mul(x87_load32(inverse), e.mem.f64(IMPULSE_FACTOR));
    let scaled = x87_mul(scaled, impulse);
    let bounce = x87_add(1.0, restitution);
    let first = x87_mul(scaled, bounce);
    e.mem.set_u32(properties, x87_store32(first));
    let velocity = x87_mul(impulse, restitution);
    let velocity = x87_store32(x87_mul(velocity, x87_m32(e.mem.u32(info))));
    let velocity = x87_store32(x87_mul(x87_load32(velocity), e.mem.f64(VELOCITY_FACTOR)));
    e.mem
        .set_u32(properties + 4, x87_store32(x87_load32(velocity)));
    // `FLDZ` / `FCOM ST2` / `TEST AH,5` / `JP`: only an ordered positive restitution adds
    // the contact's `+0x1c`.
    let last = if 0.0 < restitution {
        let total = x87_store32(x87_add(
            x87_load32(velocity),
            x87_m32(e.mem.u32(point + 0x1c)),
        ));
        x87_store32(x87_load32(total))
    } else {
        x87_store32(x87_load32(x87_store32(0.0)))
    };
    e.mem.set_u32(properties + 0x10, last);
    e.mem.set_u8(flags, e.mem.u8(flags) & 0xfe);
}

// Translated from 00d92900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleContactConstraintData_fireCallbacks` (Xbox PDB name per the engine map), cdecl
/// `(data, info, atom, word)`. With `owner = info[+0x60]`, the entities `owner[+0x14]` (a) and
/// `owner[+0x18]` (b) and the world `a[+8]`, it walks the atom's contacts (count `u16` at
/// `+4`, the contact points from `+0x30` at 0x20 bytes each, their properties from the point
/// array's entry `u16 [+6]`, `[+0xa]` bytes apart), the last index first. A contact whose
/// properties have bit 0 of `+0xf` gets an impulse estimate (`00df2510`), its contact point
/// event (flag byte = bit 3 of `owner[+0x30]`'s byte `+0x12`, the impulse's address) and its
/// response: with bit 3 of `+0xf` the contact is disabled (`+0x1c` of the point from
/// `010cc3b0`, three floats zeroed); else when the negated tolerance (`[world[+0x78] + 0x68]
/// + 0x70`) is above the impulse and the restitution (`+0xd` / 128) above the limit, the
/// single-contact solver runs (with the limits from the table at `010c7948`) and, when it
/// reports a breach (its byte at `+0x18`), the impulse-limit-breached event fires; otherwise
/// the impulse and the separating velocity are written from the inverse masses (the fourth
/// lanes at `+0x1a0`). The other contacts only get an event (flag byte 1, no impulse) when the
/// bit of `owner[+0x30]` is set. After a contact's callbacks, when an earlier contact was
/// handled and its byte `+0xe` is set, bit 1 of `+0xf` is cleared. Last, bits 0 and 3 of
/// `owner[+0x30]`'s byte `+0x12` are cleared.
pub fn hk_simple_contact_constraint_data_fire_callbacks(
    e: &mut Engine,
    data: Ptr,
    info: Ptr,
    atom: Ptr,
    word: u32,
) {
    let (info, atom) = (info.addr(), atom.addr());
    let owner = e.mem.u32(info + 0x60);
    let entity_b = e.mem.u32(owner + 0x18);
    let entity_a = e.mem.u32(owner + 0x14);
    let world = e.mem.u32(entity_a + 8);
    let motion_a = e.mem.u32(info + 0x30);
    let motion_b = e.mem.u32(info + 0x34);
    let notify = e.mem.u8(e.mem.u32(owner + 0x30) + 0x12) & 8 != 0;
    let stride = u32::from(e.mem.u8(atom + 0xa));
    let mut point = atom + 0x30;
    let mut properties = (u32::from(e.mem.u16(atom + 6)) << 5)
        .wrapping_add(atom)
        .wrapping_add(0x30);
    let last = i32::from(e.mem.u16(atom + 4)) - 1;
    e.with_stack(0xa0, |e, frame| {
        let context = ContactEventContext {
            frame: frame.addr(),
            data,
            world,
            entity_a,
            entity_b,
            word,
            motion_a,
            motion_b,
        };
        let mut previous = 0u32;
        let mut index = last;
        while index >= 0 {
            let contact = (point, properties);
            let order = (index == last, index == 0);
            let active = e.mem.u8(properties + 0xf) & 1 != 0;
            let mut impulse = None;
            if active {
                let motion_a = e.mem.u32(info + 0x30);
                let motion_b = e.mem.u32(info + 0x34);
                let estimate = e
                    .call(
                        ESTIMATE_IMPULSE,
                        &args![
                            entity_a,
                            entity_b,
                            motion_a.wrapping_add(0x40),
                            motion_b.wrapping_add(0x40),
                            point
                        ],
                    )
                    .f64();
                let slot = context.frame + 0x30;
                e.mem.set_u32(slot, x87_store32(estimate));
                fire_contact_point(e, &context, contact, order, notify as u8, slot);
                impulse = Some(slot);
            } else if notify {
                fire_contact_point(e, &context, contact, order, 1, 0);
            }
            if (active || notify) && index < last && e.mem.u8(previous + 0xe) != 0 {
                let flags = e.mem.u8(properties + 0xf) & 0xfd;
                e.mem.set_u8(properties + 0xf, flags);
            }
            if let Some(slot) = impulse {
                // The response reads the impulse back after the callbacks had its address.
                let bits = e.mem.u32(slot);
                respond_to_contact(e, &context, info, contact, bits);
            }
            index -= 1;
            point = point.wrapping_add(0x20);
            previous = properties;
            properties = properties.wrapping_add(stride);
        }
    });
    let bits = e.mem.u32(owner + 0x30) + 0x12;
    e.mem.set_u8(bits, e.mem.u8(bits) & 0xf6);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9_2df0, hkp_simple_contact_constraint_data_allocate_contact_point(Ptr, Ptr, Ptr, Ptr) -> i32),
        entry!(
            0x00d9_2900,
            hk_simple_contact_constraint_data_fire_callbacks(Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d9_27b0,
            hkp_simple_contact_constraint_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(0x00d9_27d0, hkp_simple_contact_constraint_data_get_runtime_info(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00d9_27f0, hkp_simple_contact_constraint_data_get_solver_results(Ptr, u32) -> u32),
        entry!(0x00d9_2800, hkp_simple_contact_constraint_data_is_valid(Ptr, Ptr) -> Ptr),
        entry!(0x00d9_2810, hkp_simple_contact_constraint_data_get_type(Ptr) -> u32),
        entry!(0x00d9_2820, hkp_simple_contact_constraint_data_get_simple_constraint_contact_mgr(Ptr) -> Ptr),
        entry!(
            0x00d9_2830,
            hkp_simple_contact_constraint_data_collision_response_begin_callback(
                Ptr, u32, u32, u32, u32, u32
            )
        ),
        entry!(0x00d9_2f20, fn_00d92f20(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x00d9_2fd0,
            hkp_simple_contact_constraint_data_collision_response_end_callback(
                Ptr, u32, f32, u32, u32, u32, u32
            )
        ),
        entry!(0x00d9_3140, hkp_dynamics_cp_id_mgr_new_id(Ptr, u8) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn new_log() -> EventLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A double at `address` that logs `[address, arguments...]`.
    fn log_calls(e: &mut Engine, address: u32, log: &EventLog) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            let mut entry = vec![address];
            entry.extend_from_slice(a);
            log.borrow_mut().push(entry);
            Ret::default()
        });
    }

    #[test]
    fn the_constraint_info_comes_from_the_atom() {
        let mut e = Engine::new();
        let log = new_log();
        log_calls(&mut e, GET_CONSTRAINT_INFO_UTIL, &log);
        let this = e.mem.alloc(0x40);
        let atom = e.mem.alloc(0x20);
        e.mem.set_u16(atom + 2, 0x321);
        e.mem.set_u32(this + 0x28, atom);
        e.call(0x00d9_27b0, &args![this, 0x5555u32]);
        assert_eq!(
            *log.borrow(),
            vec![vec![GET_CONSTRAINT_INFO_UTIL, atom, 0x321, 0x5555]]
        );
    }

    #[test]
    fn the_small_virtual_functions_answer_constants() {
        let mut e = Engine::new();
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info, 7);
        e.mem.set_u32(info + 4, 8);
        let got = e.call(0x00d9_27d0, &args![0x1000u32, 1u32, info]).u32();
        assert_eq!(got, info);
        assert_eq!(e.mem.u32(info), 0);
        assert_eq!(e.mem.u32(info + 4), 0);
        assert_eq!(e.call(0x00d9_27f0, &args![0x1000u32, 1u32]).u32(), 0);
        let valid = e.mem.alloc(0x10);
        let got = e.call(0x00d9_2800, &args![0x1000u32, valid]).u32();
        assert_eq!(got, valid);
        assert_eq!(e.mem.u8(valid), 1);
        assert_eq!(e.call(0x00d9_2810, &args![0x1000u32]).u32(), 0xb);
        assert_eq!(e.call(0x00d9_2820, &args![0x1014u32]).u32(), 0x1000);
    }

    /// A data object whose owner's chain has atoms of the given types (the last one ends the
    /// chain); returns `(this, atoms)`.
    fn chain(e: &mut Engine, types: &[u16]) -> (u32, Vec<u32>) {
        let this = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, owner);
        let atoms: Vec<u32> = types
            .iter()
            .map(|atom_type| {
                let atom = e.mem.alloc(0x20);
                e.mem.set_u16(atom, *atom_type);
                atom
            })
            .collect();
        for pair in atoms.windows(2) {
            e.mem.set_u32(pair[0] + 0x14, pair[1]);
        }
        e.mem.set_u32(owner + 0x10, atoms[0]);
        (this, atoms)
    }

    #[test]
    fn the_begin_callback_visits_the_chain_in_order() {
        let mut e = Engine::new();
        let log = new_log();
        for address in [
            BEGIN_CALLBACK_TYPE_17,
            BEGIN_CALLBACK_TYPE_18,
            BEGIN_CALLBACK_TYPE_1A,
            BEGIN_CALLBACK_TYPE_1C,
        ] {
            log_calls(&mut e, address, &log);
        }
        let (this, atoms) = chain(&mut e, &[0x1c, 0x19, 0x18, 0x1b, 0x17, 0x1a, 0x16]);
        e.call(0x00d9_2830, &args![this, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![BEGIN_CALLBACK_TYPE_1C, atoms[0], 1, 2, 3, 4, 5],
                vec![BEGIN_CALLBACK_TYPE_18, atoms[2], 1, 2, 3, 4, 5],
                vec![BEGIN_CALLBACK_TYPE_17, atoms[4], 1, 2, 3, 4, 5],
                vec![BEGIN_CALLBACK_TYPE_1A, atoms[5], 1, 2, 3, 4, 5],
            ]
        );
        // An empty chain, or one whose first atom is below 0x17, calls nothing.
        log.borrow_mut().clear();
        let (this, _) = chain(&mut e, &[0x16]);
        e.call(0x00d9_2830, &args![this, 1u32, 2u32, 3u32, 4u32, 5u32]);
        let owner = e.mem.u32(this + 0x24);
        e.mem.set_u32(owner + 0x10, 0);
        e.call(0x00d9_2830, &args![this, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_constructor_allocates_the_atom_and_sizes_it_from_the_bodies() {
        let mut e = Engine::new();
        let log = new_log();
        let atom = e.mem.alloc(0x40);
        e.mem.set_u16(atom + 2, 0x222);
        e.mem.set_u32(atom + 0x2c, 0xffff_ffff);
        let allocate_log = Rc::clone(&log);
        e.register_double(ALLOCATE_ATOM, move |_, a| {
            allocate_log.borrow_mut().push(a.to_vec());
            Ret {
                eax: atom,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x40);
        let body_a = e.mem.alloc(0x100);
        let body_b = e.mem.alloc(0x100);
        e.mem.set_u8(body_a + 0xd1, 3);
        e.mem.set_u8(body_b + 0xd1, 5);
        let got = e
            .call(0x00d9_2f20, &args![this, 0x7000u32, body_a, body_b])
            .u32();
        assert_eq!(got, this);
        // 0x3400 / ((3 + 5) * 4 + 0x34) = 13312 / 84 = 158.
        assert_eq!(*log.borrow(), vec![vec![1, 3, 5, 158]]);
        assert_eq!(e.mem.u32(this), 0x010c_c934);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), this + 0x18);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0008);
        assert_eq!(e.mem.u32(this + 0x24), 0x7000);
        assert_eq!(e.mem.u32(this + 0x28), atom);
        assert_eq!(e.mem.u32(this + 0x2c), 0x222);
        assert_eq!(e.mem.u16(atom + 0x12), 3);
        assert_eq!(e.mem.u16(atom + 0x10), 0);
        assert_eq!(e.mem.u32(atom + 0x2c), 0);
    }

    const FREE_DOUBLE: u32 = 0x0900_0051;

    /// Memory router with an allocator whose slot `0x10` logs `[0x10, data, size]`.
    fn install_allocator(e: &mut Engine, log: &EventLog) -> u32 {
        let mut slots = vec![0u32; 8];
        slots[4] = FREE_DOUBLE;
        e.put_vtable(0x0900_0400, &slots);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, 0x0900_0400);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(log);
        e.register_double(FREE_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(vec![0x10, a[1], a[2]]);
            Ret::default()
        });
        allocator
    }

    #[test]
    fn the_end_callback_visits_the_chain_backwards_with_the_float() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log);
        for address in [
            END_CALLBACK_TYPE_17,
            END_CALLBACK_TYPE_18,
            END_CALLBACK_TYPE_1A,
            END_CALLBACK_TYPE_1C,
        ] {
            log_calls(&mut e, address, &log);
        }
        let (this, atoms) = chain(&mut e, &[0x1c, 0x19, 0x18, 0x17, 0x1a, 0x10]);
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(
            0x00d9_2fd0,
            &args![this, 1u32, signalling, 3u32, 4u32, 5u32, 6u32],
        );
        let quiet = 0x7fc0_0001;
        assert_eq!(
            *log.borrow(),
            vec![
                vec![END_CALLBACK_TYPE_1A, atoms[4], 1, quiet, 3, 4, 5, 6],
                vec![END_CALLBACK_TYPE_17, atoms[3], 1, quiet, 3, 4, 5, 6],
                vec![END_CALLBACK_TYPE_18, atoms[2], 1, quiet, 3, 4, 5, 6],
                vec![END_CALLBACK_TYPE_1C, atoms[0], 1, quiet, 3, 4, 5, 6],
            ]
        );
        // No chain: nothing is called.
        log.borrow_mut().clear();
        let owner = e.mem.u32(this + 0x24);
        e.mem.set_u32(owner + 0x10, 0);
        e.call(
            0x00d9_2fd0,
            &args![this, 1u32, 2.0f32, 3u32, 4u32, 5u32, 6u32],
        );
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn a_long_chain_grows_the_pointer_array_and_frees_it() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log);
        e.register_double(END_CALLBACK_TYPE_17, |_, _| Ret::default());
        let grown = Rc::new(RefCell::new(0u32));
        let grown_in_double = Rc::clone(&grown);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            let buffer = e.mem.alloc(0x100);
            let old = e.mem.u32(a[1]);
            for word in 0..16 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(buffer + 4 * word, value);
            }
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, 0x4000_0040);
            *grown_in_double.borrow_mut() = buffer;
            Ret::default()
        });
        let mut types = vec![0x17u16; 17];
        types.push(0x10);
        let (this, _) = chain(&mut e, &types);
        e.call(
            0x00d9_2fd0,
            &args![this, 1u32, 2.0f32, 3u32, 4u32, 5u32, 6u32],
        );
        assert_ne!(*grown.borrow(), 0);
        assert_eq!(*log.borrow(), vec![vec![0x10, *grown.borrow(), 0x100]]);
    }

    #[test]
    fn a_new_id_reuses_the_last_free_entry_or_appends() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log);
        let table = e.mem.alloc(0x10);
        for (index, value) in [1u8, 0xff, 5, 0xff].into_iter().enumerate() {
            e.mem.set_u8(table + index as u32, value);
        }
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + 4, 4);
        e.mem.set_u32(this + 8, 0x4000_0004);
        assert_eq!(e.call(0x00d9_3140, &args![this, 9u32]).i32(), 3);
        assert_eq!(e.call(0x00d9_3140, &args![this, 8u32]).i32(), 1);
        assert_eq!(e.mem.u8(table + 3), 9);
        assert_eq!(e.mem.u8(table + 1), 8);
        // Full: the table grows (through the double) and the value is appended.
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            let bigger = e.mem.alloc(0x20);
            let old = e.mem.u32(a[1]);
            for byte in 0..4 {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(bigger + byte, value);
            }
            e.mem.set_u32(a[1], bigger);
            e.mem.set_u32(a[1] + 8, 0x4000_0010);
            Ret::default()
        });
        assert_eq!(e.call(0x00d9_3140, &args![this, 7u32]).i32(), 4);
        let table = e.mem.u32(this);
        assert_eq!(e.mem.u8(table + 4), 7);
        assert_eq!(e.mem.u32(this + 4), 5);
        // Room is left: no growth call is needed for the next one.
        assert_eq!(e.call(0x00d9_3140, &args![this, 6u32]).i32(), 5);
        // An empty table appends too.
        e.mem.set_u32(this + 4, 0);
        assert_eq!(e.call(0x00d9_3140, &args![this, 2u32]).i32(), 0);
    }
}

#[cfg(test)]
mod allocate_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A data object with an atom holding `count` contact points (points area of four, stride
    /// 0x10), an id table with a free entry at the end and a recording `expandOne` that leaves
    /// the atom where it is.
    fn scene(count: u16) -> (Engine, Seen, u32, u32) {
        let mut e = Engine::new();
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let data = e.mem.alloc(0x40);
        let atom = e.mem.alloc(0x400);
        e.mem.set_u16(atom + 2, 0x300);
        e.mem.set_u16(atom + 4, count);
        e.mem.set_u16(atom + 6, 4);
        e.mem.set_u8(atom + 0xa, 0x10);
        e.mem.set_u32(data + 0x28, atom);
        e.mem.set_u32(data + 0x24, 0x7777);
        let table = e.mem.alloc(8);
        e.mem.set_u8(table, 0);
        e.mem.set_u8(table + 1, 0xff);
        e.mem.set_u32(data + 0xc, table);
        e.mem.set_u32(data + 0x10, 2);
        e.mem.set_u32(data + 0x14, 8);
        let log = Rc::clone(&seen);
        e.register_double(EXPAND_ONE, move |_, a| {
            log.borrow_mut().push((EXPAND_ONE, a.to_vec()));
            a[0].into_ret()
        });
        for address in [OWNER_ADD_CONSTRAINT_INFO, UPDATE_FATHER_OF_MOVED_ATOM] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                if address == OWNER_ADD_CONSTRAINT_INFO {
                    words.extend((0..4).map(|i| e.mem.u32(a[2] + 4 * i)));
                }
                log.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        (e, seen, data, atom)
    }

    fn calls(seen: &Seen, address: u32) -> Vec<Vec<u32>> {
        seen.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn the_first_point_gets_properties_and_a_runtime_size_of_one_point() {
        let (mut e, seen, data, atom) = scene(0);
        let outputs = e.mem.alloc(8);
        let id = e
            .call(0x00d9_2df0, &args![data, 0x5000u32, outputs, outputs + 4])
            .i32();
        // The free entry at the end of the id table is taken; the id is its index.
        assert_eq!(id, 1);
        let point = e.mem.u32(outputs);
        let properties = e.mem.u32(outputs + 4);
        assert_eq!(point, atom + 0x30);
        assert_eq!(properties, atom + 0x30 + 0x80);
        assert_eq!(e.mem.u8(properties + 0xf), 1);
        assert_eq!(e.mem.u16(atom + 0x10) & 4, 4);
        // One point: (1 >> 1) * 0x70 + 0xa0 + 0x30, then 0x30 for the odd point and 1, 1.
        let call = &calls(&seen, OWNER_ADD_CONSTRAINT_INFO)[0];
        assert_eq!(call[..2], [0x5000, 0x7777]);
        assert_eq!(call[3..], [0xd0, 0x30, 1, 1]);
        assert!(calls(&seen, UPDATE_FATHER_OF_MOVED_ATOM).is_empty());
    }

    #[test]
    fn the_moved_flag_and_info_sit_in_the_exe_frame() {
        let (mut e, seen, data, _atom) = scene(0);
        let outputs = e.mem.alloc(8);
        let frame = e.next_stack_local(0x28);
        e.call(0x00d9_2df0, &args![data, 0x5000u32, outputs, outputs + 4]);
        assert_eq!(calls(&seen, EXPAND_ONE)[0][1], frame + 0x13);
        assert_eq!(calls(&seen, OWNER_ADD_CONSTRAINT_INFO)[0][2], frame + 0x18);
    }

    #[test]
    fn a_later_point_is_chained_to_the_previous_one() {
        let (mut e, seen, data, atom) = scene(1);
        let outputs = e.mem.alloc(8);
        // The previous point's flags: bit 1 clear and byte 0xe zero: the new one gets 3.
        e.mem.set_u8(atom + 0x30 + 0x80 + 0xf, 0);
        e.mem.set_u8(atom + 0x30 + 0x80 + 0xe, 0);
        e.call(0x00d9_2df0, &args![data, 0x5000u32, outputs, outputs + 4]);
        let properties = e.mem.u32(outputs + 4);
        assert_eq!(properties, atom + 0x30 + 0x80 + 0x10);
        assert_eq!(e.mem.u8(properties + 0xf), 3);
        assert_eq!(e.mem.u32(outputs), atom + 0x30 + 0x20);
        // Two points after this one: (2 >> 1) * 0x70 + 0xa0, the first point's extra 0x20.
        let info = &calls(&seen, OWNER_ADD_CONSTRAINT_INFO)[0][3..];
        assert_eq!(info, [0x70 + 0xa0, 0x20 + 0x40, 2, 2]);
        // A previous point with bit 1 set keeps flag 1.
        let (mut e, _, data, atom) = scene(1);
        e.mem.set_u8(atom + 0x30 + 0x80 + 0xf, 2);
        let outputs = e.mem.alloc(8);
        e.call(0x00d9_2df0, &args![data, 0x5000u32, outputs, outputs + 4]);
        assert_eq!(e.mem.u8(e.mem.u32(outputs + 4) + 0xf), 1);
    }

    #[test]
    fn a_moved_atom_is_reported_to_the_constraint() {
        let (mut e, seen, data, atom) = scene(0);
        let moved = e.mem.alloc(0x400);
        e.mem.set_u16(moved + 2, 0x321);
        e.mem.set_u16(moved + 6, 4);
        e.mem.set_u8(moved + 0xa, 0x10);
        let log = Rc::clone(&seen);
        e.register_double(EXPAND_ONE, move |e, a| {
            log.borrow_mut().push((EXPAND_ONE, a.to_vec()));
            e.mem.set_u8(a[1], 1);
            moved.into_ret()
        });
        let outputs = e.mem.alloc(8);
        e.call(0x00d9_2df0, &args![data, 0x5000u32, outputs, outputs + 4]);
        assert_eq!(e.mem.u32(data + 0x28), moved);
        assert_eq!(
            calls(&seen, UPDATE_FATHER_OF_MOVED_ATOM),
            vec![vec![0x7777, atom, moved, 0x321]]
        );
        assert_eq!(e.mem.u32(data + 0x2c), 0x321);
    }
}

#[cfg(test)]
mod fire_callbacks_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    struct Rig {
        e: Engine,
        log: CallLog,
        data: u32,
        info: u32,
        atom: u32,
        owner_flags: u32,
        entity_a: u32,
        entity_b: u32,
    }

    /// The contact point `index` (in the order handled) and its properties.
    fn contact(r: &Rig, index: u32) -> (u32, u32) {
        (r.atom + 0x30 + 0x20 * index, r.atom + 0x90 + 0x18 * index)
    }

    /// A contact constraint with `count` contacts (properties from entry 3 of the point
    /// array, 0x18 bytes apart), the impulse double answering `impulse`, the solver double
    /// writing `breach` to its parameters' `+0x18`, and the exe constants the response reads
    /// (exact ones: the inverse-mass epsilon 1, the factors -0.25 and -1.5).
    fn rig(count: u16, owner_bits: u8, impulse: f64, breach: u8) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(RESTITUTION_SCALE, 1.0 / 128.0);
        e.mem.set_f32(RESTITUTION_LIMIT, 0.3);
        e.mem.set_f64(INVERSE_MASS_EPSILON, 1.0);
        e.mem.set_f64(IMPULSE_FACTOR, -0.25);
        e.mem.set_f64(VELOCITY_FACTOR, -1.5);
        e.mem.set_u32(DISABLED_CONTACT_VALUE, 0x7f80_0005);
        e.mem.set_f32(DEFAULT_MAX_IMPULSE, 1000.0);
        for index in 0..4u32 {
            e.mem.set_f32(LIMIT_TABLE + 4 * index, 10.0 * index as f32);
        }
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        e.register_double(ESTIMATE_IMPULSE, {
            let log = Rc::clone(&log);
            move |_, a| {
                let mut entry = vec![ESTIMATE_IMPULSE];
                entry.extend_from_slice(a);
                log.borrow_mut().push(entry);
                Ret {
                    st0: impulse,
                    ..Ret::default()
                }
            }
        });
        e.register_double(FIRE_CONTACT_POINT_CALLBACK, {
            let log = Rc::clone(&log);
            move |e, a| {
                let mut entry = vec![FIRE_CONTACT_POINT_CALLBACK, a[0]];
                entry.extend((0..13).map(|word| e.mem.u32(a[1] + 4 * word)));
                // The impulse the event points at, when it points at one.
                let impulse = e.mem.u32(a[1] + 0x20);
                entry.push(if impulse == 0 { 0 } else { e.mem.u32(impulse) });
                log.borrow_mut().push(entry);
                Ret::default()
            }
        });
        e.register_double(FIRE_ENTITY_CONTACT_POINT_CALLBACK, {
            let log = Rc::clone(&log);
            move |e, a| {
                let kind = e.mem.u32(a[1]);
                log.borrow_mut()
                    .push(vec![FIRE_ENTITY_CONTACT_POINT_CALLBACK, a[0], kind]);
                Ret::default()
            }
        });
        e.register_double(SOLVE_SINGLE_CONTACT, {
            let log = Rc::clone(&log);
            move |e, a| {
                let mut entry = vec![SOLVE_SINGLE_CONTACT];
                entry.extend_from_slice(a);
                entry.extend((0..6).map(|word| e.mem.u32(a[2] + 4 * word)));
                log.borrow_mut().push(entry);
                e.mem.set_u8(a[2] + 0x18, breach);
                Ret::default()
            }
        });
        e.register_double(FIRE_CONTACT_IMPULSE_LIMIT_BREACHED, {
            let log = Rc::clone(&log);
            move |e, a| {
                let event = a[1];
                log.borrow_mut().push(vec![
                    FIRE_CONTACT_IMPULSE_LIMIT_BREACHED,
                    a[0],
                    u32::from(e.mem.u16(event)),
                    u32::from(e.mem.u16(event + 2)),
                    e.mem.u32(event + 4),
                    e.mem.u32(event + 8),
                    e.mem.u32(event + 0xc),
                    a[2],
                ]);
                Ret::default()
            }
        });
        let tolerances = e.mem.alloc(0x80);
        e.mem.set_f32(tolerances + 0x70, 1.0);
        let collision_input = e.mem.alloc(0x80);
        e.mem.set_u32(collision_input + 0x68, tolerances);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x78, collision_input);
        let entity_a = e.mem.alloc(0x210);
        let entity_b = e.mem.alloc(0x210);
        e.mem.set_u32(entity_a + 8, world);
        e.mem.set_u16(entity_a + 0x204, 1);
        e.mem.set_f32(entity_a + 0x1ac, 1.0);
        e.mem.set_f32(entity_b + 0x1ac, 2.0);
        let owner_flags = e.mem.alloc(0x20);
        e.mem.set_u8(owner_flags + 0x12, owner_bits);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x14, entity_a);
        e.mem.set_u32(owner + 0x18, entity_b);
        e.mem.set_u32(owner + 0x30, owner_flags);
        let info = e.mem.alloc(0x70);
        e.mem.set_f32(info, 2.0);
        e.mem.set_u32(info + 0x30, 0x3000);
        e.mem.set_u32(info + 0x34, 0x3400);
        e.mem.set_u32(info + 0x60, owner);
        let atom = e.mem.alloc(0x100);
        e.mem.set_u16(atom + 4, count);
        e.mem.set_u16(atom + 6, 3);
        e.mem.set_u8(atom + 0xa, 0x18);
        let data = e.mem.alloc(0x40);
        Rig {
            e,
            log,
            data,
            info,
            atom,
            owner_flags,
            entity_a,
            entity_b,
        }
    }

    fn fire(r: &mut Rig) {
        let (data, info, atom) = (r.data, r.info, r.atom);
        r.e.call(0x00d9_2900, &args![data, info, atom, 0x77u32]);
    }

    fn logged(r: &Rig, address: u32) -> Vec<Vec<u32>> {
        r.log
            .borrow()
            .iter()
            .filter(|entry| entry[0] == address)
            .map(|entry| entry[1..].to_vec())
            .collect()
    }

    #[test]
    fn fire_callbacks_estimates_disables_and_notifies_the_contacts() {
        let mut r = rig(3, 0x0f, 2.0, 0);
        let (point0, properties0) = contact(&r, 0);
        let (point1, properties1) = contact(&r, 1);
        let (point2, properties2) = contact(&r, 2);
        // The first contact: active, restitution 64 / 128, its `+0xe` set for the next one.
        r.e.mem.set_u8(properties0 + 0xf, 1);
        r.e.mem.set_u8(properties0 + 0xd, 0x40);
        r.e.mem.set_u8(properties0 + 0xe, 1);
        r.e.mem.set_f32(point0 + 0x1c, 0.5);
        // The second: active and disabled (bit 3), bit 1 set; the third: inactive.
        r.e.mem.set_u8(properties1 + 0xf, 0x0b);
        r.e.mem.set_u8(properties2 + 0xf, 0x02);
        fire(&mut r);
        // Impulse 2, restitution 0.5: inverse 1 / (2 + 1 + 1) = 0.25, so the impulse is
        // 0.25 * -0.25 * 2 * 1.5 and the velocity 2 * 0.5 * 2 * -1.5, plus the point's 0.5.
        assert_eq!(r.e.mem.f32(properties0), -0.1875);
        assert_eq!(r.e.mem.f32(properties0 + 4), -3.0);
        assert_eq!(r.e.mem.f32(properties0 + 0x10), -2.5);
        assert_eq!(r.e.mem.u8(properties0 + 0xf), 0);
        // The disabled contact: bit 1 cleared by the previous one's `+0xe`, the floats zeroed.
        assert_eq!(r.e.mem.u32(point1 + 0x1c), 0x7fc0_0005);
        assert_eq!(r.e.mem.u8(properties1 + 0xf), 0x08);
        for offset in [0, 4, 0x10] {
            assert_eq!(r.e.mem.u32(properties1 + offset), 0);
        }
        assert_eq!(r.e.mem.u8(properties2 + 0xf), 0x02);
        // Bits 0 and 3 of the owner's byte are cleared.
        assert_eq!(r.e.mem.u8(r.owner_flags + 0x12), 0x06);
        assert_eq!(
            logged(&r, ESTIMATE_IMPULSE),
            vec![
                vec![r.entity_a, r.entity_b, 0x3040, 0x3440, point0],
                vec![r.entity_a, r.entity_b, 0x3040, 0x3440, point1],
            ]
        );
        let world = r.e.mem.u32(r.entity_a + 8);
        let manager = r.data - 0x14;
        let events = logged(&r, FIRE_CONTACT_POINT_CALLBACK);
        assert_eq!(events.len(), 3);
        let expect = |point: u32, properties: u32, bytes: u32, impulse: u32| {
            vec![
                world,
                2,
                r.entity_a,
                r.entity_b,
                manager,
                0x77,
                point,
                properties,
                bytes,
                impulse,
                0,
                properties + 0x14,
                0x3000,
                0x3400,
            ]
        };
        // The flag byte (the owner's bit 3), whether first, whether last.
        assert_eq!(events[0][..9], expect(point0, properties0, 0x0101, 0)[..9]);
        assert_ne!(events[0][9], 0);
        assert_eq!(events[0][10..14], expect(point0, properties0, 0, 0)[10..14]);
        assert_eq!(events[0][14], 2.0f32.to_bits());
        assert_eq!(events[1][..9], expect(point1, properties1, 0x0001, 0)[..9]);
        assert_eq!(
            &events[2][..14],
            expect(point2, properties2, 0x0001_0001, 0).as_slice()
        );
        // Entity a (one listener) gets types 0; entity b (none) is skipped.
        assert_eq!(
            logged(&r, FIRE_ENTITY_CONTACT_POINT_CALLBACK),
            vec![vec![r.entity_a, 0]; 3]
        );
        assert!(logged(&r, SOLVE_SINGLE_CONTACT).is_empty());
    }

    #[test]
    fn fire_callbacks_solves_a_bouncing_contact_and_reports_the_breach() {
        let mut r = rig(1, 0, -2.0, 1);
        let (point, properties) = contact(&r, 0);
        r.e.mem.set_u8(properties + 0xf, 1);
        r.e.mem.set_u8(properties + 0xd, 0x50);
        r.e.mem.set_u8(properties + 0xc, 2);
        r.e.mem.set_u8(properties + 0xe, 3);
        r.e.mem.set_u32(properties + 4, 0x1234);
        r.e.mem.set_f32(properties, 9.0);
        r.e.mem.set_f32(properties + 0x10, 9.0);
        fire(&mut r);
        let solve = &logged(&r, SOLVE_SINGLE_CONTACT)[0];
        assert_eq!(solve[0], r.data);
        assert_eq!(solve[1], point);
        assert_eq!(&solve[3..7], &[r.entity_a, r.entity_b, 0x3000, 0x3400]);
        // The limit (table entry 2), the restitution 80 / 128, the maximum (table entry 3),
        // the impulse and two zeros.
        assert_eq!(
            &solve[8..14],
            &[
                20.0f32.to_bits(),
                0.625f32.to_bits(),
                30.0f32.to_bits(),
                (-2.0f32).to_bits(),
                0,
                0
            ]
        );
        let owner = r.e.mem.u32(r.info + 0x60);
        let world = r.e.mem.u32(r.entity_a + 8);
        assert_eq!(
            logged(&r, FIRE_CONTACT_IMPULSE_LIMIT_BREACHED),
            vec![vec![world, 1, 1, owner, properties, point, 1]]
        );
        assert_eq!(r.e.mem.u32(properties), 0);
        assert_eq!(r.e.mem.u32(properties + 0x10), 0);
        assert_eq!(r.e.mem.u32(properties + 4), 0x1234);
        assert_eq!(r.e.mem.u8(properties + 0xf), 0);
        // Without the owner's bit 3 the event's flag byte is 0.
        let events = logged(&r, FIRE_CONTACT_POINT_CALLBACK);
        assert_eq!(events[0][8], 0x0001_0100);
        // Without a breach, and with no maximum index, the default maximum is used.
        let mut r = rig(1, 0, -2.0, 0);
        let (_, properties) = contact(&r, 0);
        r.e.mem.set_u8(properties + 0xf, 1);
        r.e.mem.set_u8(properties + 0xd, 0x50);
        fire(&mut r);
        assert_eq!(logged(&r, SOLVE_SINGLE_CONTACT)[0][10], 1000.0f32.to_bits());
        assert!(logged(&r, FIRE_CONTACT_IMPULSE_LIMIT_BREACHED).is_empty());
    }

    #[test]
    fn fire_callbacks_without_active_contacts_or_notification_only_clears_the_bits() {
        let mut r = rig(2, 0x01, 2.0, 0);
        fire(&mut r);
        assert!(r.log.borrow().is_empty());
        assert_eq!(r.e.mem.u8(r.owner_flags + 0x12), 0);
        // A low restitution takes the response path even when the tolerance is passed.
        let mut r = rig(1, 0, -2.0, 0);
        let (_, properties) = contact(&r, 0);
        r.e.mem.set_u8(properties + 0xf, 1);
        r.e.mem.set_u8(properties + 0xd, 0x20);
        fire(&mut r);
        assert!(logged(&r, SOLVE_SINGLE_CONTACT).is_empty());
        // 0.25 * -0.25 * -2 * 1.25.
        assert_eq!(r.e.mem.f32(properties), 0.15625);
        let mut r = rig(0, 0x08, 2.0, 0);
        fire(&mut r);
        assert!(r.log.borrow().is_empty());
        assert_eq!(r.e.mem.u8(r.owner_flags + 0x12), 0);
    }
}
