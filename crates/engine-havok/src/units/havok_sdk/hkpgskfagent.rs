//! `Havok SDK/hkpgskfagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::{profile_start_record, LABEL_END};
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// The memory router (no map name): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkGskManifold_cleanup` (Xbox PDB, cdecl `(manifold, motion, flag)`) and
/// `hkGskManifold_removePoint` (Xbox PDB; register input `EDX` = the manifold, no other input).
const GSK_MANIFOLD_CLEANUP: u32 = 0x00da_d640;
const GSK_MANIFOLD_REMOVE_POINT: u32 = 0x00da_e220;
/// `hkGskAgentUtil_processCollisionNoTim` (Xbox PDB, cdecl, seven arguments).
const GSK_AGENT_UTIL_PROCESS_COLLISION_NO_TIM: u32 = 0x00db_4180;
/// `hkTransform::setMulInverseMul` (Xbox PDB; `this` = the result transform, then the two
/// transforms).
const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// The collision dispatcher's agent registration (no map name; `this` = dispatcher, then
/// `(agent function entry, shape type a, shape type b)`).
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The static agent functions the table entry holds: `hkpGskBaseAgent::staticGetPenetrations`,
/// `hkpGskBaseAgent::staticGetClosestPoints` and
/// `hkpIterativeLinearCastAgent::staticLinearCast` (Xbox PDB), and the creation function of
/// this unit (`00dacc00`).
const STATIC_GET_PENETRATIONS: u32 = 0x00da_c2b0;
const STATIC_GET_CLOSEST_POINTS: u32 = 0x00da_c450;
const STATIC_LINEAR_CAST: u32 = 0x00da_f8e0;
const CREATE_AGENT_FUNCTION: u32 = 0x00da_cc00;
/// The shape type both sides of the registration have.
const CONVEX_SHAPE_TYPE: u32 = 0x12;
/// The profiling labels `"LtGskAgent"`, `"StTim"` and `"StGsk"`.
const LABEL_GSK_AGENT: u32 = 0x010d_525c;
const LABEL_TIM: u32 = 0x010c_eb7c;
const LABEL_GSK: u32 = 0x010c_eca0;

/// `hkpGskBaseAgent::hkpGskBaseAgent` (Xbox PDB, `00dabf10`, `this`, three words).
const GSK_BASE_AGENT_CONSTRUCT: u32 = 0x00da_bf10;
/// `hkpGskfAgent`'s vtable (RTTI confirmed).
const GSKF_AGENT_VTABLE: u32 = 0x010d_5224;

// Translated from 00dacbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskfAgent::hkpGskfAgent` (Xbox PDB): the base agent constructor with the three
/// words, then the vtable and a zero word at +0x30. Returns `this`.
pub fn hkp_gskf_agent_construct(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
) -> Ptr {
    e.call(GSK_BASE_AGENT_CONSTRUCT, &args![this, first, second, third]);
    e.mem.set_u32(this.addr(), GSKF_AGENT_VTABLE);
    e.mem.set_u32(this.addr() + 0x30, 0);
    this
}

// Translated from 00dacaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskfAgent::cleanup` (Xbox PDB): cleans up the manifold (`this + 0x30`, with the motion
/// `[this + 8]` and `flag`, `hkGskManifold_cleanup`), then deletes the agent through its
/// destructor (vtable slot 0, argument 1).
pub fn hkp_gskf_agent_cleanup(e: &mut Engine, this: Ptr, flag: u32) {
    let me = this.addr();
    let motion = e.mem.u32(me + 8);
    e.call(GSK_MANIFOLD_CLEANUP, &args![me + 0x30, motion, flag]);
    e.vcall(me, 0, &args![1u32]);
}

// Translated from 00dacc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The agent creation function of the registered table entry (no map name; cdecl, the third
/// argument is not read): takes the memory for the agent from the allocator (slot 4) and
/// builds it. With a non-zero `fourth` that is a `hkpGskfAgent` of 0x80 bytes
/// (`hkpGskfAgent::hkpGskfAgent(first, second, fourth)`), else a `hkpGskBaseAgent` of 0x30
/// bytes (`hkpGskBaseAgent::hkpGskBaseAgent(first, second, 0)`); the 16-bit size field at
/// `+4` is set to the size. Returns what the constructor returns.
pub fn fn_00dacc00(e: &mut Engine, first: u32, second: u32, _unused_1: u32, fourth: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if fourth != 0 {
        let agent = e.vcall(allocator, 4, &args![0x80u32]).u32();
        e.mem.set_u16(agent + 4, 0x80);
        hkp_gskf_agent_construct(e, Ptr::new(agent), first, second, fourth).addr()
    } else {
        let agent = e.vcall(allocator, 4, &args![0x30u32]).u32();
        e.mem.set_u16(agent + 4, 0x30);
        e.call(GSK_BASE_AGENT_CONSTRUCT, &args![agent, first, second, 0u32])
            .u32()
    }
}

// Translated from 00dacc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an agent function table entry (no map name; cdecl): the creation function
/// (`00dacc00`), `hkpGskBaseAgent::staticGetPenetrations`,
/// `hkpGskBaseAgent::staticGetClosestPoints`, `hkpIterativeLinearCastAgent::staticLinearCast`
/// and the two flag bytes (`+0x10`, `+0x11`) cleared.
pub fn fn_00dacc70(e: &mut Engine, entry: Ptr) {
    let entry = entry.addr();
    e.mem.set_u32(entry, CREATE_AGENT_FUNCTION);
    e.mem.set_u32(entry + 4, STATIC_GET_PENETRATIONS);
    e.mem.set_u32(entry + 8, STATIC_GET_CLOSEST_POINTS);
    e.mem.set_u32(entry + 0xc, STATIC_LINEAR_CAST);
    e.mem.set_u8(entry + 0x10, 0);
    e.mem.set_u8(entry + 0x11, 0);
}

// Translated from 00dacca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the Gskf agent with the collision dispatcher (no map name; cdecl, `dispatcher`
/// is the one argument): builds a function table entry (`00dacc70`) on the stack and passes it
/// to the dispatcher's registration with the shape types `0x12` and `0x12`.
pub fn fn_00dacca0(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x14, |e, entry| {
        e.mem.set_u8(entry.addr() + 0x10, 0);
        e.mem.set_u8(entry.addr() + 0x11, 0);
        fn_00dacc70(e, entry);
        e.call(
            REGISTER_AGENT,
            &args![dispatcher, entry, CONVEX_SHAPE_TYPE, CONVEX_SHAPE_TYPE],
        );
    });
}

// Translated from 00daccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskfAgent::processCollisionNoTim` (Xbox PDB): sets up the collision description the
/// util function reads from the stack (the two bodies, 0, the input and the motion `[this +
/// 8]`, followed 0x20 bytes on by the transform `hkTransform::setMulInverseMul` makes from
/// the bodies' transforms `[body + 8]`) and runs `hkGskAgentUtil_processCollisionNoTim` with
/// it, two scratch areas, the agent's caches (`this + 0xc`, manifold `this + 0x30`, `this +
/// 0x20`) and `output`.
pub fn hkp_gskf_agent_process_collision_no_tim(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
) {
    let me = this.addr();
    // The exe's frame: two scratch areas (4 and 12 bytes), the collision description
    // (0x14 bytes) and, 0x20 bytes after its start, the transform (0x40 bytes).
    e.with_stack(0x70, |e, frame| {
        let scratch = frame.addr();
        let collision = scratch + 0x10;
        let transform = scratch + 0x30;
        let motion = e.mem.u32(me + 8);
        e.mem.set_u32(collision + 0x10, motion);
        e.mem.set_u32(collision + 0xc, input);
        e.mem.set_u32(collision + 8, 0);
        e.mem.set_u32(collision, body_a);
        e.mem.set_u32(collision + 4, body_b);
        let transform_a = e.mem.u32(body_a + 8);
        let transform_b = e.mem.u32(body_b + 8);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, transform_a, transform_b],
        );
        e.call(
            GSK_AGENT_UTIL_PROCESS_COLLISION_NO_TIM,
            &args![
                collision,
                scratch,
                scratch + 4,
                me + 0xc,
                me + 0x30,
                me + 0x20,
                output
            ],
        );
    });
}

// Translated from 00dacd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskfAgent::processCollision` (Xbox PDB): between the profiling records `"LtGskAgent"`
/// / `"StTim"` and the closing one, decides whether the GSK query has to run. When the
/// input's time (`input + 0x50`) differs from the agent's last one (`this + 0x18`, NaN counts
/// as different) and the input's tolerance (`input + 8`) is below the agent's remaining
/// distance (`this + 0x2c`), the distance is reduced by what the two bodies' motion can have
/// closed in the meantime (their motion states `[body + 8]`: positions `+0x40` / `+0x50`
/// weighted by the time step `input + 0x58` times `+0x5c`, plus the angular terms `+0x9c` /
/// `+0xa0`, taken along the plane `this + 0x20` with the x87 stack's double precision for the
/// angular part and `f32` SSE arithmetic for the rest). If the tolerance is still below the
/// distance the query is skipped; otherwise (and in all other cases) the profiling record
/// `"StGsk"` is written, the agent's time becomes `input + 0x54` and
/// `processCollisionNoTim` runs.
pub fn hkp_gskf_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
) {
    let me = this.addr();
    profile_start_record(e, LABEL_GSK_AGENT, LABEL_TIM);
    let mut run_query = true;
    if e.mem.f32(me + 0x18) != e.mem.f32(input + 0x50)
        && e.mem.f32(input + 8) < e.mem.f32(me + 0x2c)
    {
        let motion_a = e.mem.u32(body_a + 8);
        let motion_b = e.mem.u32(body_b + 8);
        let step = f64::from(e.mem.f32(input + 0x58));
        let weight_a = (f64::from(e.mem.f32(motion_a + 0x5c)) * step) as f32;
        let weight_b = (step * f64::from(e.mem.f32(motion_b + 0x5c))) as f32;
        let mut moved = [0.0f32; 3];
        for (lane, value) in moved.iter_mut().enumerate() {
            let offset = 4 * lane as u32;
            let a = (e.mem.f32(motion_a + 0x40 + offset) - e.mem.f32(motion_a + 0x50 + offset))
                * weight_a;
            let b = (e.mem.f32(motion_b + 0x50 + offset) - e.mem.f32(motion_b + 0x40 + offset))
                * weight_b;
            *value = a + b;
        }
        let angular_a = f64::from(e.mem.f32(motion_a + 0x9c))
            * f64::from(weight_a)
            * f64::from(e.mem.f32(motion_a + 0xa0));
        let angular_b = f64::from(e.mem.f32(motion_b + 0x9c))
            * f64::from(weight_b)
            * f64::from(e.mem.f32(motion_b + 0xa0));
        let angular = (angular_a + angular_b) as f32;
        let along: [f32; 3] =
            std::array::from_fn(|lane| e.mem.f32(me + 0x20 + 4 * lane as u32) * moved[lane]);
        let closed = (along[2] + angular) + (along[1] + along[0]);
        let remaining = (f64::from(e.mem.f32(me + 0x2c)) - f64::from(closed)) as f32;
        e.mem.set_f32(me + 0x2c, remaining);
        if e.mem.f32(input + 8) < e.mem.f32(me + 0x2c) {
            run_query = false;
        }
    }
    if run_query {
        profile_record(e, LABEL_GSK);
        let time = x87_quiet_bits(e.mem.u32(input + 0x54));
        e.mem.set_u32(me + 0x18, time);
        hkp_gskf_agent_process_collision_no_tim(e, this, body_a, body_b, input, output);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00dae340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskfAgent::removePoint` (Xbox PDB; cdecl): removes point `index` from the manifold
/// (`hkGskManifold`: counts at `+0` and `+1`, the number of points at `+2`, 8-byte points from
/// `+4`). The last point replaces it, the count drops by one and the data that follows the
/// points (`(count0 + count1 - 1) / 2 + 1` words) moves down by one point's size; then
/// `hkGskManifold_removePoint` (tail call) does the rest.
pub fn hkp_gskf_agent_remove_point(e: &mut Engine, manifold: Ptr, index: u32) {
    let m = manifold.addr();
    let count = u32::from(e.mem.u8(m + 2));
    let last = count.wrapping_sub(1);
    e.mem.set_u8(m + 2, last as u8);
    let tail = m.wrapping_add(last.wrapping_mul(8)).wrapping_add(4);
    let first_word = e.mem.u32(tail);
    let slot = m.wrapping_add(index.wrapping_mul(8));
    e.mem.set_u32(slot.wrapping_add(4), first_word);
    let second_word = e.mem.u32(tail.wrapping_add(4));
    e.mem.set_u32(slot.wrapping_add(8), second_word);
    let following = (i32::from(e.mem.u8(m + 1)) + i32::from(e.mem.u8(m)) - 1) >> 1;
    let source = m.wrapping_add(count.wrapping_mul(8)).wrapping_add(4);
    for word in 0..(following + 1).max(0) as u32 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(tail.wrapping_add(4 * word), value);
    }
    e.call(GSK_MANIFOLD_REMOVE_POINT, &args![manifold]);
}

// Translated from 00dacb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one stack word `id`, `RET 4`; descriptions, not confirmed; the
/// vtable words `010cebc0` and `010d5250` hold its address and the function Ghidra names `removePoint` jumps to it at `00d337f9`): the object keeps up to `n` 8-byte entries
/// from `this + 0x34` (`n` is the byte at `+0x32`), each with a 16-bit id at `+2`. Looks for the
/// entry whose id equals the low 16 bits of `id`; when it finds entry `i` it runs `00dae340` (cdecl)
/// on `(this + 0x30, i)` and answers what that answers. Otherwise it answers `n` (0 when `n` is
/// 0).
pub fn fn_00dacb20(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    let base = this.addr();
    let count = u32::from(e.mem.u8(base.wrapping_add(0x32)));
    if count == 0 {
        return 0;
    }
    let wanted = id as u16;
    let mut index = 0u32;
    let mut slot = base.wrapping_add(0x36);
    loop {
        if e.mem.u16(slot) == wanted {
            return e
                .call(0x00da_e340, &args![base.wrapping_add(0x30), index])
                .u32();
        }
        index += 1;
        slot = slot.wrapping_add(8);
        if index >= count {
            return index;
        }
    }
}

// Translated from 00dacb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one stack word `id`, `RET 4`; descriptions, not confirmed; the
/// vtable words `010cebc4` and `010d5254` hold its address and the function Ghidra names `commitPotential` jumps to it at `00d33819`): over the same entries as `00dacb20`, looks
/// for the first whose id is `0xffff` and stores the low 16 bits of `id` there. Answers the index
/// of that entry, or `n` when none is free (0 when `n` is 0).
pub fn fn_00dacb60(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    let base = this.addr();
    let count = u32::from(e.mem.u8(base.wrapping_add(0x32)));
    if count == 0 {
        return 0;
    }
    let mut index = 0u32;
    let mut slot = base.wrapping_add(0x36);
    loop {
        if e.mem.u16(slot) == 0xffff {
            e.mem.set_u16(
                base.wrapping_add(index.wrapping_mul(8)).wrapping_add(0x36),
                id as u16,
            );
            return index;
        }
        index += 1;
        slot = slot.wrapping_add(8);
        if index >= count {
            return index;
        }
    }
}

// Translated from 00dacba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one stack word `id`, `RET 4`; descriptions, not confirmed; the
/// vtable words `010cebc8` and `010d5258` hold its address and the function Ghidra names `createZombie` jumps to it at `00d33839`): over the same entries as `00dacb20`, looks
/// for the entry whose id equals the low 16 bits of `id` and clears its two bytes at `+0` and
/// `+1`. Answers the index of that entry, or `n` when there is none (0 when `n` is 0).
pub fn fn_00dacba0(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    let base = this.addr();
    let count = u32::from(e.mem.u8(base.wrapping_add(0x32)));
    if count == 0 {
        return 0;
    }
    let wanted = id as u16;
    let mut index = 0u32;
    let mut entry = base.wrapping_add(0x34);
    loop {
        if e.mem.u16(entry.wrapping_add(2)) == wanted {
            e.mem.set_u8(entry, 0);
            e.mem.set_u8(entry.wrapping_add(1), 0);
            return index;
        }
        index += 1;
        entry = entry.wrapping_add(8);
        if index >= count {
            return index;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dacb20, fn_00dacb20(Ptr, u32) -> u32),
        entry!(0x00dacb60, fn_00dacb60(Ptr, u32) -> u32),
        entry!(0x00dacba0, fn_00dacba0(Ptr, u32) -> u32),
        entry!(0x00da_cbd0, hkp_gskf_agent_construct(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00da_caf0, hkp_gskf_agent_cleanup(Ptr, u32)),
        entry!(0x00da_cc00, fn_00dacc00(u32, u32, u32, u32) -> u32),
        entry!(0x00da_cc70, fn_00dacc70(Ptr)),
        entry!(0x00da_cca0, fn_00dacca0(Ptr)),
        entry!(
            0x00da_ccd0,
            hkp_gskf_agent_process_collision_no_tim(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00da_cd50,
            hkp_gskf_agent_process_collision(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00da_e340, hkp_gskf_agent_remove_point(Ptr, u32)),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, log_double, monitor, new_log, Log,
    };

    /// An object with entries whose ids are `ids` (ids at `+0x36 + 8 * i`, entry bytes `0x55` at
    /// `+0x34 + 8 * i` and `+0x35 + 8 * i`) and the count byte at `+0x32`.
    fn object_with_ids_00dacb20(e: &mut Engine, ids: &[u16]) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u8(object + 0x32, ids.len() as u8);
        for (index, id) in ids.iter().enumerate() {
            let at = object + 0x34 + 8 * index as u32;
            e.mem.set_u8(at, 0x55);
            e.mem.set_u8(at + 1, 0x55);
            e.mem.set_u16(at + 2, *id);
        }
        object
    }

    #[test]
    fn fn_00dacb20_removes_the_entry_with_the_id_through_00dae340() {
        let mut e = Engine::new();
        let object = object_with_ids_00dacb20(&mut e, &[10, 20, 30]);
        e.register(0x00da_e340, |_, a| Ret {
            eax: a[1] + 100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00da_cb20, &args![object, 0x1_0014u32]).u32();
        assert_eq!(answer, 101);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00da_e340)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x00da_e340, vec![object + 0x30, 1])]);
    }

    #[test]
    fn fn_00dacb20_answers_the_count_when_the_id_is_missing() {
        let mut e = Engine::new();
        let object = object_with_ids_00dacb20(&mut e, &[10, 20, 30]);
        assert_eq!(e.call(0x00da_cb20, &args![object, 99u32]).u32(), 3);
        let empty = object_with_ids_00dacb20(&mut e, &[]);
        assert_eq!(e.call(0x00da_cb20, &args![empty, 99u32]).u32(), 0);
    }

    #[test]
    fn fn_00dacb60_stores_the_id_in_the_first_free_entry() {
        let mut e = Engine::new();
        let object = object_with_ids_00dacb20(&mut e, &[10, 0xffff, 0xffff]);
        assert_eq!(e.call(0x00da_cb60, &args![object, 0x7_0042u32]).u32(), 1);
        assert_eq!(e.mem.u16(object + 0x36 + 8), 0x42);
        assert_eq!(e.mem.u16(object + 0x36 + 16), 0xffff);
    }

    #[test]
    fn fn_00dacb60_answers_the_count_when_nothing_is_free() {
        let mut e = Engine::new();
        let object = object_with_ids_00dacb20(&mut e, &[10, 20]);
        assert_eq!(e.call(0x00da_cb60, &args![object, 42u32]).u32(), 2);
        assert_eq!(e.mem.u16(object + 0x36), 10);
        let empty = object_with_ids_00dacb20(&mut e, &[]);
        assert_eq!(e.call(0x00da_cb60, &args![empty, 42u32]).u32(), 0);
    }

    #[test]
    fn fn_00dacba0_clears_the_two_bytes_of_the_entry_with_the_id() {
        let mut e = Engine::new();
        let object = object_with_ids_00dacb20(&mut e, &[10, 20, 30]);
        assert_eq!(e.call(0x00da_cba0, &args![object, 30u32]).u32(), 2);
        assert_eq!(e.mem.u8(object + 0x34 + 16), 0);
        assert_eq!(e.mem.u8(object + 0x35 + 16), 0);
        assert_eq!(e.mem.u8(object + 0x34 + 8), 0x55);
        assert_eq!(e.mem.u16(object + 0x36 + 16), 30);
    }

    #[test]
    fn fn_00dacba0_answers_the_count_when_the_id_is_missing() {
        let mut e = Engine::new();
        let object = object_with_ids_00dacb20(&mut e, &[10, 20]);
        assert_eq!(e.call(0x00da_cba0, &args![object, 99u32]).u32(), 2);
        assert_eq!(e.mem.u8(object + 0x34), 0x55);
        let empty = object_with_ids_00dacb20(&mut e, &[]);
        assert_eq!(e.call(0x00da_cba0, &args![empty, 99u32]).u32(), 0);
    }

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    #[test]
    fn construct_runs_the_base_constructor_then_sets_the_vtable() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(GSK_BASE_AGENT_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x30, 0xdead);
        assert_eq!(
            e.call(0x00da_cbd0, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(*seen.borrow(), vec![vec![this, 1, 2, 3]]);
        assert_eq!(e.mem.u32(this), GSKF_AGENT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    const DELETE_DOUBLE: u32 = 0x0900_0100;
    const ALLOC_DOUBLE: u32 = 0x0900_0110;

    #[test]
    fn cleanup_cleans_the_manifold_then_deletes_the_agent() {
        let mut e = test_engine();
        let log = new_log();
        log_double(&mut e, &log, GSK_MANIFOLD_CLEANUP, 0);
        log_double(&mut e, &log, DELETE_DOUBLE, 0);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, DELETE_DOUBLE);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 8, 0x77);
        e.call(0x00da_caf0, &args![this, 5u32]);
        let order: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(order, vec![GSK_MANIFOLD_CLEANUP, DELETE_DOUBLE]);
        assert_eq!(
            calls(&log, GSK_MANIFOLD_CLEANUP),
            vec![vec![this + 0x30, 0x77, 5]]
        );
        assert_eq!(calls(&log, DELETE_DOUBLE), vec![vec![this, 1]]);
    }

    /// A memory router whose allocator's slot 4 hands out zeroed blocks and logs the request;
    /// answers the allocator.
    fn allocator_router(e: &mut Engine, log: &Log) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, ALLOC_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(ALLOC_DOUBLE, move |e, a| {
            sink.borrow_mut().push((ALLOC_DOUBLE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        allocator
    }

    #[test]
    fn the_creation_function_builds_a_gskf_agent_or_a_base_agent() {
        let mut e = test_engine();
        let log = new_log();
        let allocator = allocator_router(&mut e, &log);
        log_double(&mut e, &log, GSK_BASE_AGENT_CONSTRUCT, 0x1234);
        let agent = e
            .call(CREATE_AGENT_FUNCTION, &args![11u32, 22u32, 33u32, 44u32])
            .u32();
        assert_eq!(calls(&log, ALLOC_DOUBLE), vec![vec![allocator, 0x80]]);
        assert_eq!(
            calls(&log, GSK_BASE_AGENT_CONSTRUCT),
            vec![vec![agent, 11, 22, 44]]
        );
        assert_eq!(e.mem.u16(agent + 4), 0x80);
        assert_eq!(e.mem.u32(agent), GSKF_AGENT_VTABLE);
        // Without the fourth argument the base agent is built and its answer returned.
        log.borrow_mut().clear();
        let got = e
            .call(CREATE_AGENT_FUNCTION, &args![11u32, 22u32, 33u32, 0u32])
            .u32();
        assert_eq!(got, 0x1234);
        assert_eq!(calls(&log, ALLOC_DOUBLE), vec![vec![allocator, 0x30]]);
        let base = calls(&log, GSK_BASE_AGENT_CONSTRUCT);
        assert_eq!(base.len(), 1);
        assert_eq!(base[0][1..], [11, 22, 0]);
        assert_eq!(e.mem.u16(base[0][0] + 4), 0x30);
    }

    #[test]
    fn fn_00dacc70_fills_the_function_table_entry() {
        let mut e = test_engine();
        let entry = e.mem.alloc(0x14);
        for word in 0..5 {
            e.mem.set_u32(entry + 4 * word, 0xffff_ffff);
        }
        e.call(0x00da_cc70, &args![entry]);
        let words: Vec<u32> = (0..4).map(|word| e.mem.u32(entry + 4 * word)).collect();
        assert_eq!(
            words,
            vec![
                CREATE_AGENT_FUNCTION,
                STATIC_GET_PENETRATIONS,
                STATIC_GET_CLOSEST_POINTS,
                STATIC_LINEAR_CAST
            ]
        );
        assert_eq!(e.mem.bytes(entry + 0x10, 4), vec![0, 0, 0xff, 0xff]);
    }

    #[test]
    fn fn_00dacca0_registers_the_entry_for_the_convex_shape_types() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|word| e.mem.u32(a[1] + 4 * word)));
            words.push(u32::from(e.mem.u8(a[1] + 0x10)));
            words.push(u32::from(e.mem.u8(a[1] + 0x11)));
            sink.borrow_mut().push(words);
            Ret::default()
        });
        e.call(0x00da_cca0, &args![0x5000u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][0], 0x5000);
        assert_eq!(seen[0][2..4], [0x12, 0x12]);
        assert_eq!(
            seen[0][4..],
            [
                CREATE_AGENT_FUNCTION,
                STATIC_GET_PENETRATIONS,
                STATIC_GET_CLOSEST_POINTS,
                STATIC_LINEAR_CAST,
                0,
                0
            ]
        );
    }

    #[test]
    fn process_collision_no_tim_builds_the_description_and_runs_the_util() {
        let mut e = test_engine();
        let log = new_log();
        let sink = Rc::clone(&log);
        e.register_double(TRANSFORM_SET_MUL_INVERSE_MUL, move |e, a| {
            sink.borrow_mut()
                .push((TRANSFORM_SET_MUL_INVERSE_MUL, a.to_vec()));
            e.mem.set_u32(a[0], 0xabcd);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(GSK_AGENT_UTIL_PROCESS_COLLISION_NO_TIM, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..5).map(|word| e.mem.u32(a[0] + 4 * word)));
            words.push(e.mem.u32(a[0] + 0x20));
            sink.borrow_mut()
                .push((GSK_AGENT_UTIL_PROCESS_COLLISION_NO_TIM, words));
            Ret::default()
        });
        let body_a = e.mem.alloc(0x10);
        e.mem.set_u32(body_a + 8, 0x1111);
        let body_b = e.mem.alloc(0x10);
        e.mem.set_u32(body_b + 8, 0x2222);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 0x99);
        e.call(
            0x00da_ccd0,
            &args![this, body_a, body_b, 0x6000u32, 0x7000u32],
        );
        let util = calls(&log, GSK_AGENT_UTIL_PROCESS_COLLISION_NO_TIM);
        assert_eq!(util.len(), 1);
        let words = &util[0];
        let collision = words[0];
        assert_eq!(words[1], collision - 0x10);
        assert_eq!(words[2], collision - 0xc);
        assert_eq!(words[3..7], [this + 0xc, this + 0x30, this + 0x20, 0x7000]);
        assert_eq!(
            words[7..],
            [body_a, body_b, 0, 0x6000, 0x99, 0xabcd],
            "the collision description and the transform 0x20 bytes after it"
        );
        assert_eq!(
            calls(&log, TRANSFORM_SET_MUL_INVERSE_MUL),
            vec![vec![collision + 0x20, 0x1111, 0x2222]]
        );
    }

    /// A motion state with the weight at `+0x5c`, the two positions (`+0x40`, `+0x50`) and the
    /// angular factors (`+0x9c`, `+0xa0`).
    fn motion(
        e: &mut Engine,
        weight: f32,
        now: [f32; 3],
        before: [f32; 3],
        angular: [f32; 2],
    ) -> u32 {
        let motion = e.mem.alloc(0xb0);
        e.mem.set_f32(motion + 0x5c, weight);
        for (lane, (a, b)) in now.into_iter().zip(before).enumerate() {
            e.mem.set_f32(motion + 0x40 + 4 * lane as u32, a);
            e.mem.set_f32(motion + 0x50 + 4 * lane as u32, b);
        }
        e.mem.set_f32(motion + 0x9c, angular[0]);
        e.mem.set_f32(motion + 0xa0, angular[1]);
        motion
    }

    /// What `processCollision` leaves behind for an agent whose last time is `last_time` and
    /// whose remaining distance is 10, for an input with tolerance `tolerance`: whether the
    /// util ran, the remaining distance, the agent's time and the profiling labels.
    fn process_collision(last_time: f32, tolerance: f32) -> (bool, f32, f32, Vec<u32>) {
        let mut e = test_engine();
        let (stream, records) = monitor(&mut e, 0x100);
        let log = new_log();
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        log_double(&mut e, &log, GSK_AGENT_UTIL_PROCESS_COLLISION_NO_TIM, 0);
        let motion_a = motion(&mut e, 2.0, [1.0, 2.0, 3.0], [0.0; 3], [1.0, 1.0]);
        let motion_b = motion(&mut e, 0.0, [0.0; 3], [0.0; 3], [0.0; 2]);
        let body_a = e.mem.alloc(0x10);
        e.mem.set_u32(body_a + 8, motion_a);
        let body_b = e.mem.alloc(0x10);
        e.mem.set_u32(body_b + 8, motion_b);
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x18, last_time);
        e.mem.set_f32(this + 0x20, 1.0);
        e.mem.set_f32(this + 0x2c, 10.0);
        let input = e.mem.alloc(0x60);
        e.mem.set_f32(input + 8, tolerance);
        e.mem.set_f32(input + 0x50, 5.0);
        e.mem.set_f32(input + 0x54, 7.0);
        e.mem.set_f32(input + 0x58, 1.0);
        e.call(0x00da_cd50, &args![this, body_a, body_b, input, 0x7000u32]);
        assert_eq!(e.mem.u32(records), LABEL_GSK_AGENT);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_TIM);
        let end = e.mem.u32(stream + 4);
        let labels = (records + 0x10..end)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect();
        let ran = !calls(&log, GSK_AGENT_UTIL_PROCESS_COLLISION_NO_TIM).is_empty();
        (ran, e.mem.f32(this + 0x2c), e.mem.f32(this + 0x18), labels)
    }

    #[test]
    fn process_collision_skips_the_query_while_the_distance_stays_above_the_tolerance() {
        // The motion closes 2 * (1, 2, 3) * (1, 0, 0) + 2 = 4 of the distance 10.
        let (ran, remaining, time, labels) = process_collision(6.0, 0.5);
        assert!(!ran);
        assert_eq!((remaining, time), (6.0, 6.0));
        assert_eq!(labels, vec![LABEL_END]);
        // A NaN time differs from the input's too.
        let (ran, remaining, _, _) = process_collision(f32::NAN, 0.5);
        assert!(!ran);
        assert_eq!(remaining, 6.0);
    }

    #[test]
    fn process_collision_queries_when_the_distance_falls_to_the_tolerance() {
        let (ran, remaining, time, labels) = process_collision(6.0, 8.0);
        assert!(ran);
        assert_eq!((remaining, time), (6.0, 7.0));
        assert_eq!(labels, vec![LABEL_GSK, LABEL_END]);
    }

    #[test]
    fn process_collision_queries_without_reducing_when_the_tolerance_is_above_the_distance() {
        let (ran, remaining, time, labels) = process_collision(6.0, 11.0);
        assert!(ran);
        assert_eq!((remaining, time), (10.0, 7.0));
        assert_eq!(labels, vec![LABEL_GSK, LABEL_END]);
    }

    #[test]
    fn process_collision_queries_when_the_time_is_unchanged() {
        let (ran, remaining, time, labels) = process_collision(5.0, 0.5);
        assert!(ran);
        assert_eq!((remaining, time), (10.0, 7.0));
        assert_eq!(labels, vec![LABEL_GSK, LABEL_END]);
    }

    #[test]
    fn remove_point_moves_the_last_point_and_the_data_after_the_points() {
        let mut e = test_engine();
        let log = new_log();
        log_double(&mut e, &log, GSK_MANIFOLD_REMOVE_POINT, 0);
        let manifold = e.mem.alloc(0x80);
        e.mem.set_u8(manifold, 2);
        e.mem.set_u8(manifold + 1, 1);
        e.mem.set_u8(manifold + 2, 3);
        for (word, value) in [0x10, 0x11, 0x20, 0x21, 0x30, 0x31, 0xa0, 0xa1, 0xa2]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(manifold + 4 + 4 * word as u32, value);
        }
        e.call(0x00da_e340, &args![manifold, 0u32]);
        assert_eq!(e.mem.u8(manifold + 2), 2);
        let words: Vec<u32> = (0..9)
            .map(|word| e.mem.u32(manifold + 4 + 4 * word))
            .collect();
        // Point 0 is the old last point; the two words after the points moved down.
        assert_eq!(
            words,
            vec![0x30, 0x31, 0x20, 0x21, 0xa0, 0xa1, 0xa0, 0xa1, 0xa2]
        );
        assert_eq!(calls(&log, GSK_MANIFOLD_REMOVE_POINT), vec![vec![manifold]]);
        // With no counts nothing follows the points.
        e.mem.set_u8(manifold, 0);
        e.mem.set_u8(manifold + 1, 0);
        e.mem.set_u8(manifold + 2, 2);
        let before = e.mem.bytes(manifold + 0x14, 8);
        e.call(0x00da_e340, &args![manifold, 1u32]);
        assert_eq!(e.mem.u8(manifold + 2), 1);
        assert_eq!(e.mem.bytes(manifold + 0x14, 8), before);
        assert_eq!(calls(&log, GSK_MANIFOLD_REMOVE_POINT).len(), 2);
    }
}
