//! `NiParticle/nipsyspositionmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::niparticle::nipsysdata::NiPSysData;

/// `00c318b0`: the jump target of `00c28b50`.
const JUMP_TARGET_00C28B50: u32 = 0x00c3_18b0;

/// `00c31950`: the jump target of `00c28b70`.
const JUMP_TARGET_00C28B70: u32 = 0x00c3_1950;

/// `00c31830`: the jump target of `00c28b80`.
const JUMP_TARGET_00C28B80: u32 = 0x00c3_1830;

// Translated from 00c28b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPositionModifier::LoadBinary` (Xbox PDB): a 5-byte jump to `00c318b0` (engine map: `NiPSysModifier::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiPSysPositionModifier@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_position_modifier_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C28B50, &args![this, stream]);
}

// Translated from 00c28b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPositionModifier::SaveBinary` (Xbox PDB): a 5-byte jump to `00c31950` (engine map: `NiPSysModifier::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiPSysPositionModifier@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_position_modifier_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C28B70, &args![this, stream]);
}

// Translated from 00c28b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPositionModifier::IsEqual` (Xbox PDB): a 5-byte jump to `00c31830` (engine map: `NiPSysModifier::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiPSysPositionModifier@@UAA_NPAVNiObject@@@Z`.
pub fn ni_p_sys_position_modifier_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C28B80, &args![this, other]).bool()
}

// Translated from 00c28b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPositionModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26b8.
pub fn ni_p_sys_position_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26b8
}

/// `00aa13e0` (`cdecl`, one word): the object allocation; `00aa1460`
/// (`cdecl`, `(block, size)`): the sized free.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiPSysModifier`'s constructor (`00c31720`), destructor (`00c31750`)
/// and member copy (`00c31780`, `thiscall`, `(clone, process)`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
/// The vtable of `NiPSysPositionModifier` (RTTI
/// `.?AVNiPSysPositionModifier@@`).
const VTABLE: u32 = 0x010b_ea54;
/// The particle data's vtable slot +0xAC (`NiPSysData::ResolveAddedParticles`
/// in the Xbox PDB), no arguments.
const RESOLVE_ADDED_PARTICLES_SLOT: u32 = 0xac;
/// `NiParticleInfo` (Xbox PDB), 0x1C bytes: `m_kVelocity` +0x00 and
/// `m_fLastUpdate` +0x14.
const INFO_SIZE: u32 = 0x1c;
const INFO_LAST_UPDATE: u32 = 0x14;

// Translated from 00c28ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPositionModifier`'s scalar deleting destructor: `NiPSysModifier`'s
/// destructor, then the 0x18-byte free when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00c28ba0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00c28bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPositionModifier::Update` (Xbox PDB): `thiscall`, `RET 8` (time,
/// particle data). Resolves the added particles (data slot +0xAC), then for
/// each active particle moves its position by its velocity times the time
/// since its last update, and sets the last update to `time`. The x87
/// operations follow the exe instruction by instruction (the elapsed time
/// and the three offsets pass through `float` stack slots).
pub fn ni_p_sys_position_modifier_update(
    e: &mut Engine,
    _this: Ptr,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    e.vcall(data.addr(), RESOLVE_ADDED_PARTICLES_SLOT, &[]);
    let infos = e.get(data, NiPSysData::m_pkParticleInfo);
    let mut i: u16 = 0;
    if i >= e.get(data, NiPSysData::m_usActive) {
        return;
    }
    let now = x87_load32(time.to_bits());
    loop {
        let k = u32::from(i);
        let vertices = e.get(data, NiPSysData::m_pkVertex);
        let info = infos + k * INFO_SIZE;
        let position = vertices + k * 0xc;
        let last = e.mem.u32(info + INFO_LAST_UPDATE);
        let elapsed = x87_store32(x87_sub(now, x87_m32(last)));
        let elapsed = x87_load32(elapsed);
        let dx = x87_store32(x87_mul(x87_load32(e.mem.u32(info)), elapsed));
        let dy = x87_store32(x87_mul(x87_load32(e.mem.u32(info + 4)), elapsed));
        let dz = x87_store32(x87_mul(elapsed, x87_m32(e.mem.u32(info + 8))));
        let x = x87_add(x87_load32(e.mem.u32(position)), x87_m32(dx));
        e.mem.set_u32(position, x87_store32(x));
        let y = x87_add(x87_load32(dy), x87_m32(e.mem.u32(position + 4)));
        e.mem.set_u32(position + 4, x87_store32(y));
        let z = x87_add(x87_load32(e.mem.u32(position + 8)), x87_m32(dz));
        e.mem.set_u32(position + 8, x87_store32(z));
        e.mem.set_u32(info + INFO_LAST_UPDATE, x87_store32(now));
        i = i.wrapping_add(1);
        if i >= e.get(data, NiPSysData::m_usActive) {
            break;
        }
    }
}

// Translated from 00c28c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPositionModifier::CreateClone` (Xbox PDB): a 0x18-byte object
/// from `00aa13e0` through `NiPSysModifier`'s constructor and this class's
/// vtable (0 when the allocation fails), then `NiPSysModifier`'s member copy
/// into it (which still runs for a null clone). Returns the clone.
pub fn ni_p_sys_position_modifier_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0x18u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        e.call(MODIFIER_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, VTABLE);
        Ptr::new(block)
    };
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

use crate::units::niparticle::nipsysmodifier::add_class_name;

/// `NiPSysModifier::GetViewerStrings` (`00c319e0`, `thiscall`, one word).
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// The `NiRTTI` record of `NiPSysPositionModifier` (`012026b8`).
const RTTI: u32 = 0x0120_26b8;

// Translated from 00c28cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPositionModifier::GetViewerStrings` (Xbox PDB):
/// `NiPSysModifier::GetViewerStrings`, then the class name (`NiRTTI`
/// `012026b8`) appended.
pub fn ni_p_sys_position_modifier_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    add_class_name(e, strings.addr(), RTTI);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_8cb0,
            ni_p_sys_position_modifier_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c28b90, ni_p_sys_position_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_8b50,
            ni_p_sys_position_modifier_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c2_8b70,
            ni_p_sys_position_modifier_save_binary(Ptr, Ptr)
        ),
        entry!(0x00c2_8b80, ni_p_sys_position_modifier_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c2_8ba0, fn_00c28ba0(Ptr, u32) -> Ptr),
        entry!(
            0x00c2_8bc0,
            ni_p_sys_position_modifier_update(Ptr, f32, Ptr<NiPSysData>)
        ),
        entry!(
            0x00c2_8c60,
            ni_p_sys_position_modifier_create_clone(Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c28b50_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_8b50;
        const TARGET: u32 = 0x00c3_18b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00c28b70_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_8b70;
        const TARGET: u32 = 0x00c3_1950;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00c28b80_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_8b80;
        const TARGET: u32 = 0x00c3_1830;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn ni_p_sys_position_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_8b90, &args![]).u32(), 0x0120_26b8);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(MODIFIER_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_8ba0, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(calls_to(&e, NI_DELETE).is_empty());
        assert_eq!(e.call(0x00c2_8ba0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![0x5000, 0x18]]);
        assert_eq!(calls_to(&e, MODIFIER_DESTRUCTOR), vec![vec![0x5000]; 2]);
    }

    #[test]
    fn update_moves_each_active_particle_by_velocity_times_elapsed_time() {
        const RESOLVE: u32 = 0x0040_3000;
        let mut e = Engine::new();
        e.register(RESOLVE, |_, _| Ret::default());
        e.put_vtable(0x0090_0000, &[0; 0xb0 / 4]);
        e.mem
            .set_u32(0x0090_0000 + RESOLVE_ADDED_PARTICLES_SLOT, RESOLVE);
        let data: Ptr<NiPSysData> = e.new_object();
        e.mem.set_u32(data.addr(), 0x0090_0000);
        let infos = e.mem.alloc(3 * INFO_SIZE);
        let vertices = e.mem.alloc(3 * 0xc);
        e.set(data, NiPSysData::m_pkParticleInfo, infos);
        e.set(data, NiPSysData::m_pkVertex, vertices);
        e.set(data, NiPSysData::m_usActive, 2);
        for (k, (velocity, last)) in [([1.0f32, 2.0, -4.0], 1.0f32), ([0.5, 0.0, 8.0], 2.5)]
            .iter()
            .enumerate()
        {
            let info = infos + k as u32 * INFO_SIZE;
            for (j, v) in velocity.iter().enumerate() {
                e.mem.set_f32(info + 4 * j as u32, *v);
                e.mem
                    .set_f32(vertices + 0xc * k as u32 + 4 * j as u32, 10.0);
            }
            e.mem.set_f32(info + INFO_LAST_UPDATE, *last);
        }
        // A third, inactive particle stays.
        e.mem.set_f32(infos + 2 * INFO_SIZE + INFO_LAST_UPDATE, 7.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_8bc0, &args![0x7000u32, 3.0f32, data]);
        assert_eq!(calls_to(&e, RESOLVE), vec![vec![data.addr()]]);
        let at = |e: &Engine, k: u32, j: u32| e.mem.f32(vertices + 0xc * k + 4 * j);
        // Particle 0: elapsed 2.
        assert_eq!(
            [at(&e, 0, 0), at(&e, 0, 1), at(&e, 0, 2)],
            [12.0, 14.0, 2.0]
        );
        // Particle 1: elapsed 0.5.
        assert_eq!(
            [at(&e, 1, 0), at(&e, 1, 1), at(&e, 1, 2)],
            [10.25, 10.0, 14.0]
        );
        assert_eq!(e.mem.f32(infos + INFO_LAST_UPDATE), 3.0);
        assert_eq!(e.mem.f32(infos + INFO_SIZE + INFO_LAST_UPDATE), 3.0);
        assert_eq!(e.mem.f32(infos + 2 * INFO_SIZE + INFO_LAST_UPDATE), 7.0);

        // No active particles: only the resolve.
        e.set(data, NiPSysData::m_usActive, 0);
        e.call(0x00c2_8bc0, &args![0x7000u32, 9.0f32, data]);
        assert_eq!(e.mem.f32(infos + INFO_LAST_UPDATE), 3.0);
    }

    #[test]
    fn create_clone_constructs_sets_the_vtable_and_copies() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_8c60, &args![0x5000u32, 0x6000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x18]]);
        assert_eq!(calls_to(&e, MODIFIER_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_8c60, &args![0x5000u32, 0x6000u32]).u32(), 0);
        assert!(calls_to(&e, MODIFIER_CONSTRUCTOR).is_empty());
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0006);
        e.register(MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_8cb0, &args![0x5000u32, strings]);
        assert_eq!(
            calls_to(&e, MODIFIER_GET_VIEWER_STRINGS),
            vec![vec![0x5000, strings]]
        );
        assert_eq!(*stored.borrow(), vec![0x0abc_0006]);
    }
}
