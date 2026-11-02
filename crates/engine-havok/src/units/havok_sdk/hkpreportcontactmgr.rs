//! `Havok SDK/hkpreportcontactmgr.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpReportContactMgr` (Xbox PDB; vtable `010cc86c`, 0x1C bytes) keeps the world's callback
//! owner at `+0xC`, its two entities at `+0x10` and `+0x14` and a countdown of 16 bits at
//! `+0x18` (the number of contact processings to skip). Its factory (vtable `010cc8bc`) keeps
//! a word at `+8`. The events the functions hand to the callback utilities are locals of the
//! exe's stack frames, modelled as zeroed blocks of the same size (the exe leaves what it does not
//! write uninitialised). An entity's `+0x204` (16 bits) says whether it has callbacks to run.
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, shufps, sum3, zip_with, Vec4};

/// The manager's vtable, and the factory's.
const VTABLE: u32 = 0x010c_c86c;
const FACTORY_VTABLE: u32 = 0x010c_c8bc;
/// The memory router (`00c85750`): its `+0x10` is the allocator (slot 4 allocates).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The event constructor (`00cfcba0`, `this` = the event, ten stack words, the last a float).
const EVENT_CONSTRUCT: u32 = 0x00cf_cba0;
/// `hkpWorldCallbackUtil` functions (cdecl `(world, event)`): `fireContactPointAdded`,
/// `fireContactPointRemoved`, `fireContactPointCallback`, `fireContactProcess`.
const WORLD_FIRE_ADDED: u32 = 0x00d0_1850;
const WORLD_FIRE_REMOVED: u32 = 0x00d0_1900;
const WORLD_FIRE_CALLBACK: u32 = 0x00d0_0ca0;
const WORLD_FIRE_PROCESS: u32 = 0x00d0_19b0;
/// `hkpEntityCallbackUtil` functions (cdecl `(entity, event)`): `fireContactPointAddedInternal`,
/// `fireContactPointRemovedInternal`, `fireContactPointCallbackInternal`,
/// `fireContactProcessInternal`.
const ENTITY_FIRE_ADDED: u32 = 0x00d0_0110;
const ENTITY_FIRE_REMOVED: u32 = 0x00d0_01c0;
const ENTITY_FIRE_CALLBACK: u32 = 0x00cf_ff00;
const ENTITY_FIRE_PROCESS: u32 = 0x00d0_0270;

/// Whether the entity has callbacks (`+0x204`, 16 bits, non-zero).
fn has_callbacks(e: &Engine, entity: u32) -> bool {
    e.mem.u16(entity + 0x204) != 0
}

/// The smaller of the two 16-bit values at `+0xAA` of the objects, compared unsigned.
fn smaller_word(e: &Engine, first: u32, second: u32) -> u16 {
    e.mem.u16(first + 0xaa).min(e.mem.u16(second + 0xaa))
}

// Translated from 00cfc5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpReportContactMgr` constructor (the vtable is confirmed through its RTTI; the map
/// has no name): reference count 1 (`+6`, 16 bits), `+8` = 1, the vtable, the callback owner
/// (`+0xC`), the two entities (`+0x10` = `entity_a`, `+0x14` = `entity_b`) and the countdown
/// (`+0x18`, 16 bits) set to the smaller of the entities' words at `+0xAA`. Answers `this`.
pub fn hkp_report_contact_mgr_construct(
    e: &mut Engine,
    this: Ptr,
    owner: u32,
    entity_a: Ptr,
    entity_b: Ptr,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 1);
    e.mem.set_u32(me, VTABLE);
    let countdown = smaller_word(e, entity_b.addr(), entity_a.addr());
    e.mem.set_u32(me + 0x10, entity_a.addr());
    e.mem.set_u32(me + 0x14, entity_b.addr());
    e.mem.set_u16(me + 0x18, countdown);
    e.mem.set_u32(me + 0xc, owner);
    this
}

/// The velocity of the point at offset `r` of a body: `w x r + linear`, the products in the
/// order the code multiplies them.
fn point_velocity(r: Vec4, w: Vec4, linear: Vec4) -> Vec4 {
    let first = zip_with(shufps(w, w, 0xc9), shufps(r, r, 0xd2), |a, b| a * b);
    let second = zip_with(shufps(w, w, 0xd2), shufps(r, r, 0xc9), |a, b| a * b);
    zip_with(zip_with(first, second, |a, b| a - b), linear, |a, b| a + b)
}

// Translated from 00cfc5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReportContactMgr::addContactPointImpl` (Xbox PDB), `RET 0x18` (six words: `p1` to `p3`
/// and `p5` are handed on to the event constructor, `p4` too and its `+4` is an argument of the
/// slot-`0x14` call below, `point` is the contact point). The relative velocity of the two
/// entities' points (positions at `+0x140`, linear velocity `+0x1B0`, angular velocity
/// `+0x1C0`), dotted with the vector at `point + 0x10`, goes to the event constructor
/// (`00cfcba0`) as its float.
/// The event is then fired at the world (`+0xC`) and at each entity that has callbacks.
/// When the event's status (`+0x20`) is 1 the manager's slot `0x14` is called with
/// `(0, [p4 + 4])` and 0xFFFF is the answer; otherwise the countdown (`+0x18`) takes the
/// event's 16-bit word at `+0x32` and the answer is 0.
pub fn hkp_report_contact_mgr_add_contact_point_impl(
    e: &mut Engine,
    this: Ptr,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: Ptr,
    p5: u32,
    point: Ptr,
) -> u32 {
    let me = this.addr();
    let first = e.mem.u32(me + 0x10);
    let second = e.mem.u32(me + 0x14);
    let p = load_vec(e, point.addr());
    let r_first = zip_with(p, load_vec(e, first + 0x140), |a, b| a - b);
    let r_second = zip_with(p, load_vec(e, second + 0x140), |a, b| a - b);
    let v_second = point_velocity(
        r_second,
        load_vec(e, second + 0x1c0),
        load_vec(e, second + 0x1b0),
    );
    let v_first = point_velocity(
        r_first,
        load_vec(e, first + 0x1c0),
        load_vec(e, first + 0x1b0),
    );
    let normal = load_vec(e, point.addr() + 0x10);
    let relative = zip_with(v_second, v_first, |a, b| a - b);
    let along = sum3(zip_with(relative, normal, |a, b| a * b));
    e.with_stack(0x40, |e, event| {
        e.call(
            EVENT_CONSTRUCT,
            &args![
                event,
                0u32,
                this,
                p3,
                p4,
                p1,
                p2,
                point,
                p5,
                0u32,
                x87_quiet_bits(along.to_bits())
            ],
        );
        let world = e.mem.u32(me + 0xc);
        e.call(WORLD_FIRE_ADDED, &args![world, event]);
        if has_callbacks(e, first) {
            e.call(ENTITY_FIRE_ADDED, &args![first, event]);
        }
        if has_callbacks(e, second) {
            e.call(ENTITY_FIRE_ADDED, &args![second, event]);
        }
        if e.mem.u32(event.addr() + 0x20) == 1 {
            let key = e.mem.u32(p4.addr() + 4);
            e.vcall(me, 0x14, &args![0u32, key]);
            return 0xffff;
        }
        let countdown = e.mem.u16(event.addr() + 0x32);
        e.mem.set_u16(me + 0x18, countdown);
        0
    })
}

// Translated from 00cfc760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReportContactMgr::addToiImpl` (Xbox PDB), `RET 0x24` (nine words): clears the output
/// (`out`: bytes `+0xC`, `+0xD`, `+0xE` = 0, `+0xF` = 1), builds the event on the stack (`p1`
/// `+0x00`, `p2` `+0x04`, 0 `+0x08`, `p6` `+0x10`, `p7` `+0x14`, `out` `+0x18`, the float at
/// `fraction` `+0x1C`, 0 `+0x20`, `this` `+0x24`, `p3` `+0x28`, `p4` `+0x2C`, the float `p5`
/// `+0x30`; floats go through the x87 stack) and fires it at the world (`+0xC`) and at each
/// entity (`+0x10`, `+0x14`) that has callbacks. When the status (`+0x20`) is 1 the manager's
/// slot `0x20` is called with `([p4 + 4], out)`; otherwise the event's float goes back to
/// `fraction`. Answers the status.
pub fn hkp_report_contact_mgr_add_toi_impl(
    e: &mut Engine,
    this: Ptr,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: Ptr,
    p5: f32,
    p6: u32,
    p7: u32,
    fraction: Ptr,
    out: Ptr,
) -> u32 {
    let me = this.addr();
    e.mem.set_u8(out.addr() + 0xc, 0);
    e.mem.set_u8(out.addr() + 0xe, 0);
    e.mem.set_u8(out.addr() + 0xd, 0);
    e.mem.set_u8(out.addr() + 0xf, 1);
    let first = e.mem.u32(me + 0x10);
    let fraction_bits = x87_quiet_bits(e.mem.u32(fraction.addr()));
    let second = e.mem.u32(me + 0x14);
    e.with_stack(0x34, |e, event| {
        let at = event.addr();
        e.mem.set_u32(at, p1);
        e.mem.set_u32(at + 0x10, p6);
        e.mem.set_u32(at + 4, p2);
        e.mem.set_u32(at + 0x14, p7);
        e.mem.set_u32(at + 0x1c, fraction_bits);
        e.mem.set_u32(at + 0x28, p3);
        e.mem.set_f32(at + 0x30, x87_quiet(p5));
        e.mem.set_u32(at + 0x2c, p4.addr());
        let world = e.mem.u32(me + 0xc);
        e.mem.set_u32(at + 8, 0);
        e.mem.set_u32(at + 0x18, out.addr());
        e.mem.set_u32(at + 0x24, me);
        e.mem.set_u32(at + 0x20, 0);
        e.call(WORLD_FIRE_ADDED, &args![world, event]);
        if has_callbacks(e, first) {
            e.call(ENTITY_FIRE_ADDED, &args![first, event]);
        }
        if has_callbacks(e, second) {
            e.call(ENTITY_FIRE_ADDED, &args![second, event]);
        }
        let status = e.mem.u32(at + 0x20);
        if status == 1 {
            let key = e.mem.u32(p4.addr() + 4);
            e.vcall(me, 0x20, &args![key, out]);
            return e.mem.u32(at + 0x20);
        }
        let updated = x87_quiet_bits(e.mem.u32(at + 0x1c));
        e.mem.set_u32(fraction.addr(), updated);
        status
    })
}

// Translated from 00cfc860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReportContactMgr::removeToiImpl` (Xbox PDB), `RET 8`: builds the removal event on the
/// stack (16 bits `+0x00` = 0xFFFF, `p2` `+0x04`, the entities `+0x08`/`+0x0C`, the manager
/// `+0x14`, `p1` `+0x18`) and fires it at the world of the first entity (`entity + 8`) and at
/// each entity that has callbacks.
pub fn hkp_report_contact_mgr_remove_toi_impl(e: &mut Engine, this: Ptr, p1: u32, p2: u32) {
    let me = this.addr();
    let first = e.mem.u32(me + 0x10);
    let world = e.mem.u32(first + 8);
    let second = e.mem.u32(me + 0x14);
    e.with_stack(0x20, |e, event| {
        let at = event.addr();
        e.mem.set_u16(at, 0xffff);
        e.mem.set_u32(at + 4, p2);
        e.mem.set_u32(at + 0x14, me);
        e.mem.set_u32(at + 8, first);
        e.mem.set_u32(at + 0xc, second);
        e.mem.set_u32(at + 0x18, p1);
        e.call(WORLD_FIRE_REMOVED, &args![world, event]);
        if has_callbacks(e, first) {
            e.call(ENTITY_FIRE_REMOVED, &args![first, event]);
        }
        if has_callbacks(e, second) {
            e.call(ENTITY_FIRE_REMOVED, &args![second, event]);
        }
    });
}

// Translated from 00cfc8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReportContactMgr::removeContactPointImpl` (Xbox PDB), `RET 8`: builds the removal event
/// on the stack (`id`, 16 bits, `+0x00`; 0 `+0x04`; the entities `+0x08`/`+0x0C`; the manager
/// `+0x14`; `p2` `+0x18`) and fires it at the world (`+0xC`) and at each entity that has
/// callbacks.
pub fn hkp_report_contact_mgr_remove_contact_point_impl(
    e: &mut Engine,
    this: Ptr,
    id: u16,
    p2: u32,
) {
    let me = this.addr();
    let first = e.mem.u32(me + 0x10);
    let second = e.mem.u32(me + 0x14);
    e.with_stack(0x20, |e, event| {
        let at = event.addr();
        e.mem.set_u16(at, id);
        e.mem.set_u32(at + 0x14, me);
        let world = e.mem.u32(me + 0xc);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, first);
        e.mem.set_u32(at + 0xc, second);
        e.mem.set_u32(at + 0x18, p2);
        e.call(WORLD_FIRE_REMOVED, &args![world, event]);
        if has_callbacks(e, first) {
            e.call(ENTITY_FIRE_REMOVED, &args![first, event]);
        }
        if has_callbacks(e, second) {
            e.call(ENTITY_FIRE_REMOVED, &args![second, event]);
        }
    });
}

// Translated from 00cfc960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReportContactMgr::confirmToi` (Xbox PDB), `RET 0xC`: `this` and the third word are not
/// read. `toi` is a record whose `+0xC` and `+0x10` are the two entities (the first is also
/// the world's owner: its `+8` is the world). The event built on the stack has: 2 `+0x00`,
/// `toi[+0xC]` `+0x04`, `toi[+0x10]` `+0x08`, `toi[+0x14]` `+0x0C`, 0 `+0x10`, `toi + 0x50`
/// `+0x14`, `toi + 0x18` `+0x18`, three zero bytes from `+0x1C`, `toi + 4` `+0x20`, the
/// address of the second argument word `+0x24`, `toi + 0x2C` `+0x28`, 0 `+0x2C`, 0 `+0x30`.
/// It is fired at the world of the first entity (`00d00ca0`), then at the first entity with its
/// first word set to 0 and at the second with it set to 1, each only when it has callbacks.
pub fn hkp_report_contact_mgr_confirm_toi(
    e: &mut Engine,
    _this: Ptr,
    toi: Ptr,
    p2: u32,
    unused_2: u32,
) {
    let record = toi.addr();
    let first = e.mem.u32(record + 0xc);
    let second = e.mem.u32(record + 0x10);
    let third = e.mem.u32(record + 0x14);
    // The second and third argument words sit next to each other on the stack.
    e.with_stack(8, |e, words| {
        e.mem.set_u32(words.addr(), p2);
        e.mem.set_u32(words.addr() + 4, unused_2);
        e.with_stack(0x34, |e, event| {
            let at = event.addr();
            e.mem.set_u32(at + 8, second);
            e.mem.set_u32(at + 0x14, record + 0x50);
            e.mem.set_u32(at + 0x18, record + 0x18);
            e.mem.set_u32(at + 0xc, third);
            e.mem.set_u32(at + 0x20, record + 4);
            e.mem.set_u32(at + 0x24, words.addr());
            e.mem.set_u32(at + 4, first);
            e.mem.set_u32(at + 0x10, 0);
            e.mem.set_u8(at + 0x1c, 0);
            e.mem.set_u8(at + 0x1d, 0);
            e.mem.set_u8(at + 0x1e, 0);
            e.mem.set_u32(at, 2);
            e.mem.set_u32(at + 0x28, record + 0x2c);
            e.mem.set_u32(at + 0x2c, 0);
            e.mem.set_u32(at + 0x30, 0);
            let world = e.mem.u32(first + 8);
            e.call(WORLD_FIRE_CALLBACK, &args![world, event]);
            e.mem.set_u32(at, 0);
            if has_callbacks(e, first) {
                e.call(ENTITY_FIRE_CALLBACK, &args![first, event]);
            }
            e.mem.set_u32(at, 1);
            if has_callbacks(e, second) {
                e.call(ENTITY_FIRE_CALLBACK, &args![second, event]);
            }
        });
    });
}

// Translated from 00cfca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the manager's factory (the map has no name): reference count 1 (`+6`,
/// 16 bits), the factory vtable, `owner` at `+8`. Answers `this`.
pub fn hkp_report_contact_mgr_factory_construct(e: &mut Engine, this: Ptr, owner: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), FACTORY_VTABLE);
    e.mem.set_u32(this.addr() + 8, owner);
    this
}

// Translated from 00cfca40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReportContactMgr::Factory::createContactMgr` (Xbox PDB), `RET 0xC`: the two entities are
/// each the first argument plus the signed byte at its `+0x10`; the manager is a 0x1C-byte
/// block from the memory router's allocator (slot 4) whose size field (`+4`, 16 bits) is set
/// to 0x1C and which the constructor (`00cfc5a0`) builds with the factory's `+8`. The third
/// argument word is not read. Answers the manager.
pub fn hkp_report_contact_mgr_factory_create_contact_mgr(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    _unused_2: u32,
) -> Ptr {
    let offset_first = i32::from(e.mem.i8(first.addr() + 0x10)) as u32;
    let entity_a = first.addr().wrapping_add(offset_first);
    let offset_second = i32::from(e.mem.i8(second.addr() + 0x10)) as u32;
    let entity_b = second.addr().wrapping_add(offset_second);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(block + 4, 0x1c);
    let owner = e.mem.u32(this.addr() + 8);
    hkp_report_contact_mgr_construct(
        e,
        Ptr::new(block),
        owner,
        Ptr::new(entity_a),
        Ptr::new(entity_b),
    )
}

/// How many words the zeroing loop of `processContactImpl` clears below the contact words:
/// `span / 0x30 - 1` with the signed division the compiler does through the magic multiplier
/// `0x2AAAAAAB` (high word of the 64-bit product, shifted right by 3, plus its sign bit).
fn contact_slots(span: i32) -> i32 {
    let high = ((i64::from(span) * 0x2aaa_aaab) >> 32) as i32;
    let shifted = high >> 3;
    shifted
        .wrapping_add(((shifted as u32) >> 31) as i32)
        .wrapping_sub(1)
}

// Translated from 00cfca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpReportContactMgr::processContactImpl` (Xbox PDB), `RET 0x10` (the third word is not
/// read): the countdown (`+0x18`, 16 bits) is decremented; when it was not 0 nothing else
/// happens. Otherwise it is set to the smaller of the entities' words at `+0xAA` (the entities
/// are `first` and `second` plus the signed bytes at their `+0x10`), and the event on the stack
/// (`first` `+0x00`, `second` `+0x04`, `list` `+0x0C`, the manager at `+0x410`; the words from
/// `+0x10` to `+0x14 + 4 * (n - 1)` cleared, `n = contact_slots(list[0] - list - 0x10)`) is
/// fired at the world (`+0xC`) and at each entity that has callbacks.
///
/// The event is the exe's 0x414-byte frame; more than 256 words would overwrite its return
/// address, so that case panics.
pub fn hkp_report_contact_mgr_process_contact_impl(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    _unused_2: u32,
    list: Ptr,
) {
    let me = this.addr();
    let counter = e.mem.u16(me + 0x18);
    e.mem.set_u16(me + 0x18, counter.wrapping_sub(1));
    if counter != 0 {
        return;
    }
    let offset_first = i32::from(e.mem.i8(first.addr() + 0x10)) as u32;
    let entity_a = first.addr().wrapping_add(offset_first);
    let offset_second = i32::from(e.mem.i8(second.addr() + 0x10)) as u32;
    let entity_b = second.addr().wrapping_add(offset_second);
    let countdown = smaller_word(e, entity_a, entity_b);
    e.mem.set_u16(me + 0x18, countdown);
    let end = e.mem.u32(list.addr());
    let span = end.wrapping_sub(list.addr()).wrapping_sub(0x10) as i32;
    let slots = contact_slots(span);
    e.with_stack(0x414, |e, event| {
        let at = event.addr();
        e.mem.set_u32(at, first.addr());
        e.mem.set_u32(at + 4, second.addr());
        e.mem.set_u32(at + 0xc, list.addr());
        e.mem.set_u32(at + 0x410, me);
        if slots >= 0 {
            let mut slot = slots;
            loop {
                slot -= 1;
                let offset = 0x14i64 + 4 * i64::from(slot);
                assert!(
                    offset < 0x414,
                    "00cfca90: the contact words overrun the stack frame"
                );
                e.mem.set_u32((at as i64 + offset) as u32, 0);
                if slot < 0 {
                    break;
                }
            }
        }
        let world = e.mem.u32(me + 0xc);
        e.call(WORLD_FIRE_PROCESS, &args![world, event]);
        if has_callbacks(e, entity_a) {
            e.call(ENTITY_FIRE_PROCESS, &args![entity_a, event]);
        }
        if has_callbacks(e, entity_b) {
            e.call(ENTITY_FIRE_PROCESS, &args![entity_b, event]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf_c5a0,
            hkp_report_contact_mgr_construct(Ptr, u32, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00cf_c5f0,
            hkp_report_contact_mgr_add_contact_point_impl(Ptr, u32, u32, u32, Ptr, u32, Ptr) -> u32
        ),
        entry!(
            0x00cf_c760,
            hkp_report_contact_mgr_add_toi_impl(Ptr, u32, u32, u32, Ptr, f32, u32, u32, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00cf_c860,
            hkp_report_contact_mgr_remove_toi_impl(Ptr, u32, u32)
        ),
        entry!(
            0x00cf_c8e0,
            hkp_report_contact_mgr_remove_contact_point_impl(Ptr, u16, u32)
        ),
        entry!(
            0x00cf_c960,
            hkp_report_contact_mgr_confirm_toi(Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00cf_ca20,
            hkp_report_contact_mgr_factory_construct(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cf_ca40,
            hkp_report_contact_mgr_factory_create_contact_mgr(Ptr, Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cf_ca90,
            hkp_report_contact_mgr_process_contact_impl(Ptr, Ptr, Ptr, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use crate::units::havok_sdk::hkpsphereshape::store_vec;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A fire call the doubles saw: (address, arguments, the event bytes at the call).
    type Fired = Rc<RefCell<Vec<(u32, Vec<u32>, Vec<u8>)>>>;

    /// What the entities' `+8` holds (the world, a value only passed on), and the callback
    /// owner of the manager (`+0xC`).
    const ENTITY_WORLD: u32 = 0x5000_0001;
    const OWNER: u32 = 0x6000_0002;
    /// Slots `0x14` and `0x20` of the test manager's vtable, and the allocator's slot 4.
    const REMOVE_CONTACT_POINT_SLOT: u32 = 0x0070_0014;
    const REMOVE_TOI_SLOT: u32 = 0x0070_0020;
    const ALLOCATE: u32 = 0x0070_0004;

    /// A double at `address` that records its arguments and `size` bytes of the event its
    /// second argument points at, then writes the `patch` (offset, word) pairs into the event.
    fn fire_double(e: &mut Engine, fired: &Fired, address: u32, size: u32, patch: Vec<(u32, u32)>) {
        let log = Rc::clone(fired);
        e.register_double(address, move |e, a| {
            let bytes = e.mem.bytes(a[1], size);
            log.borrow_mut().push((address, a.to_vec(), bytes));
            for (offset, word) in &patch {
                e.mem.set_u32(a[1] + offset, *word);
            }
            Ret::default()
        });
    }

    fn word(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    }

    fn of(fired: &Fired, address: u32) -> Vec<(Vec<u32>, Vec<u8>)> {
        fired
            .borrow()
            .iter()
            .filter(|(a, _, _)| *a == address)
            .map(|(_, args, bytes)| (args.clone(), bytes.clone()))
            .collect()
    }

    /// An entity with a position, velocities, the callback word and a world.
    fn entity(e: &mut Engine, position: Vec4, linear: Vec4, angular: Vec4, callbacks: u16) -> u32 {
        let entity = e.mem.alloc(0x400);
        store_vec(e, entity + 0x140, position);
        store_vec(e, entity + 0x1b0, linear);
        store_vec(e, entity + 0x1c0, angular);
        e.mem.set_u16(entity + 0x204, callbacks);
        e.mem.set_u32(entity + 8, ENTITY_WORLD);
        entity
    }

    /// A manager over the two entities, with a vtable whose slots `0x14` and `0x20` are the
    /// test functions above.
    fn manager(e: &mut Engine, first: u32, second: u32) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x14, REMOVE_CONTACT_POINT_SLOT);
        e.mem.set_u32(table + 0x20, REMOVE_TOI_SLOT);
        let mgr = e.mem.alloc(0x20);
        e.mem.set_u32(mgr, table);
        e.mem.set_u32(mgr + 0xc, OWNER);
        e.mem.set_u32(mgr + 0x10, first);
        e.mem.set_u32(mgr + 0x14, second);
        e.mem.set_u16(mgr + 0x18, 0x7777);
        mgr
    }

    #[test]
    fn construct_takes_the_smaller_countdown_00cfc5a0() {
        for (a, b, expected) in [(5u16, 9u16, 5u16), (9, 5, 5), (7, 7, 7), (0xffff, 1, 1)] {
            let mut e = engine();
            let [entity_a, entity_b] = [0; 2].map(|_| e.mem.alloc(0x100));
            e.mem.set_u16(entity_a + 0xaa, a);
            e.mem.set_u16(entity_b + 0xaa, b);
            let this = e.mem.alloc(0x20);
            e.mem.write(this, &[0xcd; 0x20]);
            let result = e.call(0x00cf_c5a0, &args![this, 0x1234u32, entity_a, entity_b]);
            assert_eq!(result.u32(), this);
            assert_eq!(e.mem.u16(this + 6), 1);
            assert_eq!(e.mem.u32(this + 8), 1);
            assert_eq!(e.mem.u32(this), VTABLE);
            assert_eq!(e.mem.u32(this + 0xc), 0x1234);
            assert_eq!(e.mem.u32(this + 0x10), entity_a);
            assert_eq!(e.mem.u32(this + 0x14), entity_b);
            assert_eq!(e.mem.u16(this + 0x18), expected);
            assert_eq!(e.mem.u16(this + 0x1a), 0xcdcd);
        }
    }

    /// One call of `addContactPointImpl` with an event constructor double that sets the
    /// event's status and word. Answers the engine, the manager, the log, the point, the slot
    /// argument block and the result.
    fn add_contact_point(
        status: u32,
        countdown: u16,
        callbacks: (u16, u16),
    ) -> (Engine, u32, Log, u32, u32, u32) {
        let mut e = engine();
        let log = new_log();
        let sink = Rc::clone(&log);
        e.register_double(EVENT_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((EVENT_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0] + 0x20, status);
            e.mem.set_u16(a[0] + 0x32, countdown);
            Ret::default()
        });
        for address in [
            WORLD_FIRE_ADDED,
            ENTITY_FIRE_ADDED,
            REMOVE_CONTACT_POINT_SLOT,
        ] {
            log_double(&mut e, &log, address, 0);
        }
        let first = entity(
            &mut e,
            [1.0, 2.0, 3.0, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            callbacks.0,
        );
        let second = entity(
            &mut e,
            [0.0; 4],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            callbacks.1,
        );
        let mgr = manager(&mut e, first, second);
        let point = e.mem.alloc(0x30);
        store_vec(&mut e, point, [2.0, 2.0, 2.0, 0.0]);
        store_vec(&mut e, point + 0x10, [0.0, 0.6, 0.8, 0.0]);
        let slot_words = e.mem.alloc(8);
        e.mem.set_u32(slot_words + 4, 0x77);
        let result = e
            .call(
                0x00cf_c5f0,
                &args![mgr, 0x11u32, 0x22u32, 0x33u32, slot_words, 0x55u32, point],
            )
            .u32();
        (e, mgr, log, point, slot_words, result)
    }

    #[test]
    fn an_accepted_contact_point_updates_the_countdown_00cfc5f0() {
        let (e, mgr, log, point, slot_words, result) = add_contact_point(0, 0x1234, (3, 0));
        assert_eq!(result, 0);
        assert_eq!(e.mem.u16(mgr + 0x18), 0x1234);
        let built = calls(&log, EVENT_CONSTRUCT);
        assert_eq!(built.len(), 1);
        let a = &built[0];
        let event = a[0];
        assert_eq!(
            a[1..10],
            [0, mgr, 0x33, slot_words, 0x11, 0x22, point, 0x55, 0]
        );
        // The relative velocity along the normal: (-0.5, -2, 2) . (0, 0.6, 0.8).
        assert!((f32::from_bits(a[10]) - 0.4).abs() < 1e-6);
        let first = e.mem.u32(mgr + 0x10);
        assert_eq!(calls(&log, WORLD_FIRE_ADDED), vec![vec![OWNER, event]]);
        // Only the first entity has callbacks.
        assert_eq!(calls(&log, ENTITY_FIRE_ADDED), vec![vec![first, event]]);
        assert!(calls(&log, REMOVE_CONTACT_POINT_SLOT).is_empty());
    }

    #[test]
    fn a_rejected_contact_point_removes_it_and_answers_0xffff_00cfc5f0() {
        let (e, mgr, log, _, _, result) = add_contact_point(1, 0x1234, (0, 5));
        assert_eq!(result, 0xffff);
        // The countdown is left alone.
        assert_eq!(e.mem.u16(mgr + 0x18), 0x7777);
        assert_eq!(
            calls(&log, REMOVE_CONTACT_POINT_SLOT),
            vec![vec![mgr, 0, 0x77]]
        );
        let second = e.mem.u32(mgr + 0x14);
        let event = calls(&log, EVENT_CONSTRUCT)[0][0];
        assert_eq!(calls(&log, ENTITY_FIRE_ADDED), vec![vec![second, event]]);
    }

    /// One call of `addToiImpl`: the event's status and float are set by the world double.
    fn add_toi(status: u32) -> (Engine, u32, Fired, Log, [u32; 3], u32) {
        let mut e = engine();
        let fired: Fired = Rc::new(RefCell::new(Vec::new()));
        fire_double(
            &mut e,
            &fired,
            WORLD_FIRE_ADDED,
            0x34,
            vec![(0x20, status), (0x1c, 0.25f32.to_bits())],
        );
        fire_double(&mut e, &fired, ENTITY_FIRE_ADDED, 0x34, vec![]);
        let log = new_log();
        log_double(&mut e, &log, REMOVE_TOI_SLOT, 0);
        let first = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 1);
        let second = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 0);
        let mgr = manager(&mut e, first, second);
        let slot_words = e.mem.alloc(8);
        e.mem.set_u32(slot_words + 4, 0x77);
        let fraction = e.mem.alloc(4);
        e.mem.set_f32(fraction, 0.5);
        let out = e.mem.alloc(0x10);
        e.mem.write(out, &[0xcd; 0x10]);
        let result = e
            .call(
                0x00cf_c760,
                &args![
                    mgr,
                    0x11u32,
                    0x22u32,
                    0x33u32,
                    slot_words,
                    0x7f80_0001u32,
                    0x66u32,
                    0x7777u32,
                    fraction,
                    out
                ],
            )
            .u32();
        (e, mgr, fired, log, [slot_words, fraction, out], result)
    }

    #[test]
    fn the_toi_event_is_built_and_fired_at_the_world_and_the_entity_00cfc760() {
        let (e, mgr, fired, log, [slot_words, fraction, out], result) = add_toi(0);
        assert_eq!(result, 0);
        let world = of(&fired, WORLD_FIRE_ADDED);
        assert_eq!(world.len(), 1);
        let (args, event) = &world[0];
        assert_eq!(args[0], OWNER);
        let expected = [
            (0x00, 0x11),
            (0x04, 0x22),
            (0x08, 0),
            (0x10, 0x66),
            (0x14, 0x7777),
            (0x18, out),
            (0x1c, 0.5f32.to_bits()),
            (0x20, 0),
            (0x24, mgr),
            (0x28, 0x33),
            (0x2c, slot_words),
            (0x30, 0x7fc0_0001),
        ];
        for (offset, value) in expected {
            assert_eq!(word(event, offset), value, "offset {offset:#x}");
        }
        // The output flags.
        assert_eq!(
            [0xc, 0xd, 0xe, 0xf].map(|at| e.mem.u8(out + at)),
            [0, 0, 0, 1]
        );
        assert_eq!(e.mem.u8(out + 0xb), 0xcd);
        // Status 0: the float goes back; only the first entity has callbacks.
        assert_eq!(e.mem.f32(fraction), 0.25);
        let first = e.mem.u32(mgr + 0x10);
        let entities = of(&fired, ENTITY_FIRE_ADDED);
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].0[0], first);
        assert!(calls(&log, REMOVE_TOI_SLOT).is_empty());
    }

    #[test]
    fn a_status_of_one_removes_the_toi_and_keeps_the_float_00cfc760() {
        let (e, mgr, _, log, [_, fraction, out], result) = add_toi(1);
        assert_eq!(result, 1);
        assert_eq!(e.mem.f32(fraction), 0.5);
        assert_eq!(calls(&log, REMOVE_TOI_SLOT), vec![vec![mgr, 0x77, out]]);
    }

    #[test]
    fn the_answer_is_the_status_00cfc760() {
        for status in [0u32, 1, 2, 0xffff_ffff] {
            let mut e = engine();
            let fired: Fired = Rc::new(RefCell::new(Vec::new()));
            fire_double(&mut e, &fired, WORLD_FIRE_ADDED, 0x34, vec![(0x20, status)]);
            let log = new_log();
            log_double(&mut e, &log, REMOVE_TOI_SLOT, 0);
            let first = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 0);
            let second = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 0);
            let mgr = manager(&mut e, first, second);
            let slot_words = e.mem.alloc(8);
            let fraction = e.mem.alloc(4);
            let out = e.mem.alloc(0x10);
            let result = e.call(
                0x00cf_c760,
                &args![mgr, 0u32, 0u32, 0u32, slot_words, 0u32, 0u32, 0u32, fraction, out],
            );
            assert_eq!(result.u32(), status);
        }
    }

    #[test]
    fn the_remove_events_name_the_entities_and_reach_each_one_with_callbacks_00cfc860_00cfc8e0() {
        for (callbacks, expected_entities) in [((1u16, 1u16), 2usize), ((0, 0), 0), ((0, 9), 1)] {
            let mut e = engine();
            let fired: Fired = Rc::new(RefCell::new(Vec::new()));
            for address in [WORLD_FIRE_REMOVED, ENTITY_FIRE_REMOVED] {
                fire_double(&mut e, &fired, address, 0x1c, vec![]);
            }
            let first = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], callbacks.0);
            let second = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], callbacks.1);
            let mgr = manager(&mut e, first, second);
            e.call(0x00cf_c860, &args![mgr, 0xa1u32, 0xa2u32]);
            let world = of(&fired, WORLD_FIRE_REMOVED);
            assert_eq!(world.len(), 1);
            // The world is the first entity's (`+8`).
            assert_eq!(world[0].0[0], ENTITY_WORLD);
            let event = &world[0].1;
            assert_eq!(event[0..2], [0xff, 0xff]);
            assert_eq!(word(event, 4), 0xa2);
            assert_eq!(word(event, 8), first);
            assert_eq!(word(event, 0xc), second);
            assert_eq!(word(event, 0x14), mgr);
            assert_eq!(word(event, 0x18), 0xa1);
            assert_eq!(of(&fired, ENTITY_FIRE_REMOVED).len(), expected_entities);

            // `removeContactPointImpl`: the world is the manager's, the id is 16 bits.
            let fired: Fired = Rc::new(RefCell::new(Vec::new()));
            for address in [WORLD_FIRE_REMOVED, ENTITY_FIRE_REMOVED] {
                fire_double(&mut e, &fired, address, 0x1c, vec![]);
            }
            e.call(0x00cf_c8e0, &args![mgr, 0xdead_beefu32, 0xa3u32]);
            let world = of(&fired, WORLD_FIRE_REMOVED);
            assert_eq!(world[0].0[0], OWNER);
            let event = &world[0].1;
            assert_eq!(event[0..2], [0xef, 0xbe]);
            assert_eq!(word(event, 4), 0);
            assert_eq!(word(event, 8), first);
            assert_eq!(word(event, 0xc), second);
            assert_eq!(word(event, 0x14), mgr);
            assert_eq!(word(event, 0x18), 0xa3);
            let entities = of(&fired, ENTITY_FIRE_REMOVED);
            assert_eq!(entities.len(), expected_entities);
            if callbacks == (0, 9) {
                assert_eq!(entities[0].0[0], second);
            }
        }
    }

    #[test]
    fn confirm_toi_fires_the_world_then_each_entity_with_its_index_00cfc960() {
        let mut e = engine();
        let fired: Fired = Rc::new(RefCell::new(Vec::new()));
        // The world double also reads the two words the event's `+0x24` points at.
        let pointed = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&pointed);
        let log = Rc::clone(&fired);
        e.register_double(WORLD_FIRE_CALLBACK, move |e, a| {
            let bytes = e.mem.bytes(a[1], 0x34);
            let target = word(&bytes, 0x24);
            sink.borrow_mut()
                .push((e.mem.u32(target), e.mem.u32(target + 4)));
            log.borrow_mut()
                .push((WORLD_FIRE_CALLBACK, a.to_vec(), bytes));
            Ret::default()
        });
        fire_double(&mut e, &fired, ENTITY_FIRE_CALLBACK, 0x34, vec![]);
        let first = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 1);
        let second = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 2);
        let record = e.mem.alloc(0x60);
        e.mem.set_u32(record + 0xc, first);
        e.mem.set_u32(record + 0x10, second);
        e.mem.set_u32(record + 0x14, 0x1414);
        e.call(0x00cf_c960, &args![0x9999u32, record, 0x2222u32, 0x3333u32]);
        let world = of(&fired, WORLD_FIRE_CALLBACK);
        assert_eq!(world.len(), 1);
        assert_eq!(world[0].0[0], ENTITY_WORLD);
        let event = &world[0].1;
        let expected = [
            (0x00, 2),
            (0x04, first),
            (0x08, second),
            (0x0c, 0x1414),
            (0x10, 0),
            (0x14, record + 0x50),
            (0x18, record + 0x18),
            (0x20, record + 4),
            (0x28, record + 0x2c),
            (0x2c, 0),
            (0x30, 0),
        ];
        for (offset, value) in expected {
            assert_eq!(word(event, offset), value, "offset {offset:#x}");
        }
        assert_eq!(event[0x1c..0x1f], [0, 0, 0]);
        // The pointer at `+0x24` names the second and third argument words.
        assert_eq!(*pointed.borrow(), vec![(0x2222, 0x3333)]);
        // The index is 0 for the first entity and 1 for the second.
        let entities = of(&fired, ENTITY_FIRE_CALLBACK);
        assert_eq!(entities.len(), 2);
        assert_eq!((entities[0].0[0], word(&entities[0].1, 0)), (first, 0));
        assert_eq!((entities[1].0[0], word(&entities[1].1, 0)), (second, 1));
    }

    #[test]
    fn confirm_toi_skips_entities_without_callbacks_00cfc960() {
        let mut e = engine();
        let fired: Fired = Rc::new(RefCell::new(Vec::new()));
        fire_double(&mut e, &fired, WORLD_FIRE_CALLBACK, 0x34, vec![]);
        fire_double(&mut e, &fired, ENTITY_FIRE_CALLBACK, 0x34, vec![]);
        let first = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 0);
        let second = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 4);
        let record = e.mem.alloc(0x60);
        e.mem.set_u32(record + 0xc, first);
        e.mem.set_u32(record + 0x10, second);
        e.call(0x00cf_c960, &args![0u32, record, 0u32, 0u32]);
        let entities = of(&fired, ENTITY_FIRE_CALLBACK);
        assert_eq!(entities.len(), 1);
        assert_eq!((entities[0].0[0], word(&entities[0].1, 0)), (second, 1));
    }

    #[test]
    fn the_factory_is_built_with_its_owner_00cfca20() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcd; 0x10]);
        assert_eq!(e.call(0x00cf_ca20, &args![this, 0x4242u32]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), FACTORY_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x4242);
        assert_eq!(e.mem.u16(this + 4), 0xcdcd);
    }

    #[test]
    fn the_factory_builds_a_manager_over_the_entities_behind_the_collidables_00cfca40() {
        let mut e = engine();
        let log = new_log();
        let allocator_table = e.mem.alloc(0x10);
        e.mem.set_u32(allocator_table + 4, ALLOCATE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        let block = e.mem.alloc(0x1c);
        e.mem.write(block, &[0xcd; 0x1c]);
        log_double(&mut e, &log, ALLOCATE, block);
        // The collidables sit inside buffers; the entity is reached through a signed byte.
        let buffer = e.mem.alloc(0x800);
        let (first, second) = (buffer + 0x200, buffer + 0x500);
        e.mem.set_u8(first + 0x10, 8);
        e.mem.set_u8(second + 0x10, 0xf0);
        e.mem.set_u16(first + 8 + 0xaa, 4);
        e.mem.set_u16(second - 16 + 0xaa, 3);
        let factory = e.mem.alloc(0x10);
        e.mem.set_u32(factory + 8, 0x4242);
        let result = e.call(0x00cf_ca40, &args![factory, first, second, 0u32]);
        assert_eq!(result.u32(), block);
        assert_eq!(calls(&log, ALLOCATE), vec![vec![allocator, 0x1c]]);
        assert_eq!(e.mem.u16(block + 4), 0x1c);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0x4242);
        assert_eq!(e.mem.u32(block + 0x10), first + 8);
        assert_eq!(e.mem.u32(block + 0x14), second - 16);
        assert_eq!(e.mem.u16(block + 0x18), 3);
    }

    #[test]
    fn the_contact_slots_follow_the_signed_division_by_0x30() {
        assert_eq!(contact_slots(0x90), 2);
        assert_eq!(contact_slots(0x30), 0);
        assert_eq!(contact_slots(0x2f), -1);
        assert_eq!(contact_slots(0), -1);
        assert_eq!(contact_slots(-0x30), -2);
        assert_eq!(contact_slots(-0x2f), -1);
        assert_eq!(contact_slots(0x30 * 258), 257);
    }

    /// One call of `processContactImpl` with `contacts` contact slots.
    fn process_contact(
        counter: u16,
        contacts: u32,
        callbacks: (u16, u16),
    ) -> (Engine, u32, Fired, [u32; 3]) {
        let mut e = engine();
        let fired: Fired = Rc::new(RefCell::new(Vec::new()));
        fire_double(&mut e, &fired, WORLD_FIRE_PROCESS, 0x414, vec![]);
        fire_double(&mut e, &fired, ENTITY_FIRE_PROCESS, 0x414, vec![]);
        let first = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], callbacks.0);
        let second = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], callbacks.1);
        e.mem.set_u16(first + 0xaa, 9);
        e.mem.set_u16(second + 0xaa, 6);
        let mgr = manager(&mut e, first, second);
        e.mem.set_u16(mgr + 0x18, counter);
        let list = e.mem.alloc(0x10 + 0x30 * 4);
        e.mem.set_u32(list, list + 0x10 + 0x30 * contacts);
        e.call(0x00cf_ca90, &args![mgr, first, second, 0u32, list]);
        (e, mgr, fired, [first, second, list])
    }

    #[test]
    fn the_countdown_skips_the_events_until_it_runs_out_00cfca90() {
        let (e, mgr, fired, _) = process_contact(3, 1, (1, 1));
        assert_eq!(e.mem.u16(mgr + 0x18), 2);
        assert!(fired.borrow().is_empty());
    }

    #[test]
    fn at_zero_the_event_is_fired_and_the_countdown_restarts_00cfca90() {
        let (e, mgr, fired, [first, second, list]) = process_contact(0, 3, (1, 2));
        assert_eq!(e.mem.u16(mgr + 0x18), 6);
        let world = of(&fired, WORLD_FIRE_PROCESS);
        assert_eq!(world.len(), 1);
        assert_eq!(world[0].0[0], OWNER);
        let event = &world[0].1;
        assert_eq!(word(event, 0), first);
        assert_eq!(word(event, 4), second);
        assert_eq!(word(event, 0xc), list);
        assert_eq!(word(event, 0x410), mgr);
        let entities = of(&fired, ENTITY_FIRE_PROCESS);
        assert_eq!(
            entities.iter().map(|(a, _)| a[0]).collect::<Vec<_>>(),
            vec![first, second]
        );
    }

    #[test]
    fn the_countdown_wraps_below_zero_00cfca90() {
        // 0 is the value that fires; 1 counts down to 0 and does not.
        let (e, mgr, fired, _) = process_contact(1, 1, (0, 0));
        assert_eq!(e.mem.u16(mgr + 0x18), 0);
        assert!(fired.borrow().is_empty());
    }

    #[test]
    fn only_the_entities_with_callbacks_get_the_process_event_00cfca90() {
        let (_, _, fired, [_, second, _]) = process_contact(0, 0, (0, 5));
        let entities = of(&fired, ENTITY_FIRE_PROCESS);
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].0[0], second);
    }

    #[test]
    #[should_panic(expected = "00cfca90")]
    fn too_many_contact_words_overrun_the_frame_00cfca90() {
        let mut e = engine();
        let first = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 0);
        let second = entity(&mut e, [0.0; 4], [0.0; 4], [0.0; 4], 0);
        let mgr = manager(&mut e, first, second);
        e.mem.set_u16(mgr + 0x18, 0);
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list, list + 0x10 + 0x30 * 258);
        e.call(0x00cf_ca90, &args![mgr, first, second, 0u32, list]);
    }
}
