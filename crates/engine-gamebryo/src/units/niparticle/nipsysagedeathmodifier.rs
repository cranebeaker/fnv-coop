//! `NiParticle/nipsysagedeathmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31670`: the jump target of `00c28b60`.
const JUMP_TARGET_00C28B60: u32 = 0x00c3_1670;

// Translated from 00c28b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c31670` (engine map: `NiPSysModifier::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysAgeDeathModifier@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_age_death_modifier_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C28B60, &args![this, stream]).bool()
}

// Translated from 00c2efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2798.
pub fn ni_p_sys_age_death_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2798
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::{
    stream_read, NiPSysModifier, CLONE_MAP_LOOKUP, GET_OBJECT_FROM_LINK_ID, READ_LINK_ID,
};

layout! {
    /// `NiPSysAgeDeathModifier` (Xbox PDB), 0x20 bytes; the PC code uses the
    /// same offsets.
    pub struct NiPSysAgeDeathModifier: 0x20 {
        /// `m_bSpawnOnDeath` (Xbox PDB).
        0x18 m_bSpawnOnDeath: u8,
        /// `m_pkSpawnModifier` (Xbox PDB): `NiPSysSpawnModifier*`.
        0x1C m_pkSpawnModifier: u32,
    }
}

/// The vtable of `NiPSysAgeDeathModifier` (RTTI
/// `.?AVNiPSysAgeDeathModifier@@`, `010c0304`).
const VTABLE: u32 = 0x010c_0304;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized free.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiPSysModifier`'s constructor (`00c31720`), destructor (`00c31750`),
/// member copy (`00c31780`), `LinkObject` (`00c31650`), `ProcessClone`
/// (`00c317e0`) and `LoadBinary` (`00c318b0`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LINK_OBJECT: u32 = 0x00c3_1650;
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
/// `NiPSysData::RemoveParticle` and `NiPSysSpawnModifier::SpawnParticles`
/// (Xbox PDB vtable slot +0xa4 of each).
const REMOVE_PARTICLE: u32 = 0xa4;
const SPAWN_PARTICLES: u32 = 0xa4;

// Translated from 00c2ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::LinkObject` (Xbox PDB): `NiPSysModifier`'s
/// `LinkObject`, then the linked object becomes the spawn modifier.
pub fn ni_p_sys_age_death_modifier_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_LINK_OBJECT, &args![this, stream]);
    let spawner = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiPSysAgeDeathModifier::m_pkSpawnModifier, spawner);
}

// Translated from 00c2eff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::_scalar_deleting_destructor_` (Xbox PDB):
/// `NiPSysModifier`'s destructor, then the sized delete of 0x20 bytes when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_p_sys_age_death_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    flags: u32,
) -> Ptr<NiPSysAgeDeathModifier> {
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00c2f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::Update` (Xbox PDB): `thiscall`, `RET 8` (time,
/// particle data). Every active particle, from the last to the first, ages
/// by `time - last update` (`(time - last) + age`, stored); when its life
/// span is below the new age it is removed (data vtable slot +0xa4), after
/// the spawn modifier's `SpawnParticles` (its slot +0xa4: the time, `life
/// span + time - age`, the index word, the target) when spawning on death
/// with a spawn modifier. The index word passed is the exe's counter
/// register: it starts at `count - 1` and gains 0xffff each time round, so
/// its low half counts down and its high half counts the particles done.
pub fn ni_p_sys_age_death_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    let count = e.get(data, NiPSysData::m_usActive);
    if count == 0 {
        return;
    }
    let now = x87_load32(time.to_bits());
    let spawns = e.get(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath) != 0
        && e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier) != 0;
    let mut counter = u32::from(count);
    let mut index = u32::from(count.wrapping_sub(1));
    loop {
        let info = e.get(data, NiPSysData::m_pkParticleInfo) + (index & 0xffff) * 0x1c;
        let age = x87_load32(x87_store32(x87_add(
            x87_sub(now, x87_m32(e.mem.u32(info + 0x14))),
            x87_m32(e.mem.u32(info + 0xc)),
        )));
        e.mem.set_u32(info + 0xc, x87_store32(age));
        let life_span = x87_load32(e.mem.u32(info + 0x10));
        if life_span < age {
            if spawns {
                let left = x87_store32(x87_sub(x87_add(life_span, now), age));
                let target = e.mem.u32(this.addr() + NiPSysModifier::m_pkTarget.off);
                let spawner = e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier);
                e.vcall(
                    spawner,
                    SPAWN_PARTICLES,
                    &args![x87_store32(now), x87_quiet_bits(left), index, target],
                );
            }
            e.vcall(data.addr(), REMOVE_PARTICLE, &args![index]);
        }
        counter = counter.wrapping_add(0xffff);
        index = index.wrapping_add(0xffff);
        if counter as u16 == 0 {
            break;
        }
    }
}

// Translated from 00c2f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::CreateClone` (Xbox PDB): a 0x20-byte object from
/// `00aa13e0` (constructed: the `NiPSysModifier` constructor, the vtable
/// `010c0304`, no spawning, no spawn modifier; 0 when the allocation
/// fails), the `NiPSysModifier` member copy into it (it still runs into a
/// null clone), then the spawn-on-death flag. Returns the clone.
pub fn ni_p_sys_age_death_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    process: Ptr,
) -> Ptr<NiPSysAgeDeathModifier> {
    let clone = Ptr::<NiPSysAgeDeathModifier>::new(e.call(NI_NEW, &args![0x20u32]).u32());
    if !clone.is_null() {
        e.call(MODIFIER_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(clone.addr(), VTABLE);
        e.set(clone, NiPSysAgeDeathModifier::m_bSpawnOnDeath, 0);
        e.set(clone, NiPSysAgeDeathModifier::m_pkSpawnModifier, 0);
    }
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let spawn = e.get(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath);
    e.set(clone, NiPSysAgeDeathModifier::m_bSpawnOnDeath, spawn);
    clone
}

// Translated from 00c2f1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::ProcessClone` (Xbox PDB): `NiPSysModifier`'s
/// `ProcessClone`, then the clone of `this` from the process's map (into a
/// stack word cleared first); with a spawn modifier, the clone's spawn
/// modifier is the modifier's clone when the map has one (the lookup
/// answers true), else the same modifier.
pub fn ni_p_sys_age_death_modifier_process_clone(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    process: Ptr,
) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(process.addr());
        e.mem.set_u32(slot.addr(), 0);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = Ptr::<NiPSysAgeDeathModifier>::new(e.mem.u32(slot.addr()));
        let spawner = e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier);
        if spawner == 0 {
            return;
        }
        let map = e.mem.u32(process.addr());
        if e.call(CLONE_MAP_LOOKUP, &args![map, spawner, slot]).bool() {
            let spawner_clone = e.mem.u32(slot.addr());
            e.set(
                clone,
                NiPSysAgeDeathModifier::m_pkSpawnModifier,
                spawner_clone,
            );
            return;
        }
        let spawner = e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier);
        e.set(clone, NiPSysAgeDeathModifier::m_pkSpawnModifier, spawner);
    });
}

// Translated from 00c2f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, the spawn-on-death flag (one byte read into the stream
/// argument's stack word; the component-size word is a local), then a link
/// id (`00a645b0`, the spawn modifier).
pub fn ni_p_sys_age_death_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.with_stack(8, |e, frame| {
        let size_local = frame.addr();
        let argument_slot = frame.addr() + 4;
        e.mem.set_u32(argument_slot, s);
        stream_read(e, s, argument_slot, 1, 1, size_local);
        let spawn = (e.mem.u8(argument_slot) != 0) as u8;
        e.set(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath, spawn);
    });
    e.call(READ_LINK_ID, &args![s]);
}

/// `NiPSysModifier`'s `IsEqual` (`00c31830`), `SaveBinary` (`00c31950`) and
/// `GetViewerStrings` (`00c319e0`), `thiscall` with one word each.
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// The stream's output binary stream (+0x250) and its write function (word +0xc, `cdecl`:
/// the object, the buffer, the count, the address of the component size and the component
/// count); the bytes written are added to the object's word +4.
const STREAM_OUTPUT_BINARY_STREAM: u32 = 0x250;
/// The class's RTTI record (its first word the class name), the viewer string makers
/// (`cdecl`: `NiGetViewerString` `00a7d330`, of a byte `00a7d370`, of a string `00a7d5e0`),
/// the string array's grow (`0096ad30`) and set-at (`0096ae90`), and the names
/// (`Spawn on Death`, `Spawn Modifier`, `None`).
const AGE_DEATH_MODIFIER_RTTI: u32 = 0x0120_2798;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL_00A7D370: u32 = 0x00a7_d370;
const VIEWER_STRING_TEXT_00A7D5E0: u32 = 0x00a7_d5e0;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const SPAWN_ON_DEATH_NAME: u32 = 0x010c_03b8;
const SPAWN_MODIFIER_NAME: u32 = 0x010c_03a8;
const NONE_TEXT: u32 = 0x0102_909c;

// Translated from 00c2ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::IsEqual` (Xbox PDB, `thiscall`, one word: the other modifier):
/// `NiPSysModifier::IsEqual`, the same `m_bSpawnOnDeath`, and either no spawn modifier on
/// both or both set with the other's spawn modifier's virtual slot +0x5c (with this one's)
/// answering non-zero.
pub fn ni_p_sys_age_death_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    other: Ptr<NiPSysAgeDeathModifier>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(other, NiPSysAgeDeathModifier::m_bSpawnOnDeath)
        != e.get(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath)
    {
        return false;
    }
    let theirs = e.get(other, NiPSysAgeDeathModifier::m_pkSpawnModifier);
    let mine_set = e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier) != 0;
    if (theirs != 0) != mine_set {
        return false;
    }
    if theirs == 0 {
        return true;
    }
    let mine = e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier);
    // Reached only with both set, so this test always passes.
    if mine == 0 {
        return true;
    }
    e.vcall(theirs, 0x5c, &args![mine]).bool()
}

// Translated from 00c2f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::SaveBinary` (Xbox PDB, `thiscall`, one word: the stream):
/// `NiPSysModifier::SaveBinary`, `m_bSpawnOnDeath` (one byte, written from the low byte of the
/// argument's stack word; the component size in the frame's local word) through the stream's
/// output binary stream, then the spawn modifier's link ID (the stream's virtual slot +0x30).
pub fn ni_p_sys_age_death_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_SAVE_BINARY, &args![this, stream]);
    e.with_stack(8, |e, frame| {
        let (size_slot, argument_slot) = (frame.addr(), frame.addr() + 4);
        let flag = e.get(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath);
        let binary = e.mem.u32(stream.addr() + STREAM_OUTPUT_BINARY_STREAM);
        e.mem.set_u32(argument_slot, stream.addr());
        e.mem.set_u8(argument_slot, flag);
        e.mem.set_u32(size_slot, 1);
        let write = e.mem.u32(binary + 0xc);
        let written = e
            .call(write, &args![binary, argument_slot, 1u32, size_slot, 1u32])
            .u32();
        let total = e.mem.u32(binary + 4).wrapping_add(written);
        e.mem.set_u32(binary + 4, total);
    });
    let spawn = e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier);
    e.vcall(stream.addr(), 0x30, &args![spawn]);
}

/// Adds the string at `slot` to the viewer string array (grown to growth + size when full).
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step.wrapping_add(size)]);
    }
    e.call(STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00c2f2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAgeDeathModifier::GetViewerStrings` (Xbox PDB, `thiscall`, one word: the string
/// array): `NiPSysModifier::GetViewerStrings`, then the class name (RTTI `01202798`),
/// `Spawn on Death` and `Spawn Modifier` (the spawn modifier's name, its word +8, or `None`),
/// each added to the array (the string kept in the argument's stack word).
pub fn ni_p_sys_age_death_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysAgeDeathModifier>,
    strings: Ptr,
) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let name = e.global::<u32>(AGE_DEATH_MODIFIER_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let flag = u32::from(e.get(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath));
        let text = e
            .call(
                VIEWER_STRING_BOOL_00A7D370,
                &args![SPAWN_ON_DEATH_NAME, flag],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let spawn = e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier);
        let spawn_name = if spawn != 0 {
            e.mem.u32(spawn + 8)
        } else {
            NONE_TEXT
        };
        let text = e
            .call(
                VIEWER_STRING_TEXT_00A7D5E0,
                &args![SPAWN_MODIFIER_NAME, spawn_name],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_ef60,
            ni_p_sys_age_death_modifier_link_object(Ptr<NiPSysAgeDeathModifier>, Ptr)
        ),
        entry!(
            0x00c2_eff0,
            ni_p_sys_age_death_modifier_scalar_deleting_destructor(
                Ptr<NiPSysAgeDeathModifier>,
                u32,
            )
                -> Ptr<NiPSysAgeDeathModifier>
        ),
        entry!(
            0x00c2_f010,
            ni_p_sys_age_death_modifier_update(Ptr<NiPSysAgeDeathModifier>, f32, Ptr<NiPSysData>)
        ),
        entry!(
            0x00c2_f150,
            ni_p_sys_age_death_modifier_create_clone(
                Ptr<NiPSysAgeDeathModifier>,
                Ptr,
            ) -> Ptr<NiPSysAgeDeathModifier>
        ),
        entry!(
            0x00c2_f1a0,
            ni_p_sys_age_death_modifier_process_clone(Ptr<NiPSysAgeDeathModifier>, Ptr)
        ),
        entry!(
            0x00c2_f200,
            ni_p_sys_age_death_modifier_load_binary(Ptr<NiPSysAgeDeathModifier>, Ptr)
        ),
        entry!(0x00c2efe0, ni_p_sys_age_death_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c2_8b60, ni_p_sys_age_death_modifier_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_ef80,
            ni_p_sys_age_death_modifier_is_equal(
                Ptr<NiPSysAgeDeathModifier>,
                Ptr<NiPSysAgeDeathModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_f260,
            ni_p_sys_age_death_modifier_save_binary(Ptr<NiPSysAgeDeathModifier>, Ptr)
        ),
        entry!(
            0x00c2_f2c0,
            ni_p_sys_age_death_modifier_get_viewer_strings(Ptr<NiPSysAgeDeathModifier>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c28b60_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_8b60;
        const TARGET: u32 = 0x00c3_1670;
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
    fn ni_p_sys_age_death_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_efe0, &args![]).u32(), 0x0120_2798);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    fn log_of(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn link_object_takes_the_spawn_modifier() {
        let mut e = Engine::new();
        e.register(MODIFIER_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| Ret {
            eax: 0x6600,
            ..Ret::default()
        });
        let this = e.new_object::<NiPSysAgeDeathModifier>();
        e.call(0x00c2_ef60, &args![this, 0x7000u32]);
        assert_eq!(
            e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier),
            0x6600
        );
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.register(MODIFIER_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_eff0, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(log_of(&e, NI_DELETE).is_empty());
        e.call(0x00c2_eff0, &args![0x5000u32, 1u32]);
        assert_eq!(log_of(&e, NI_DELETE), vec![vec![0x5000, 0x20]]);
        assert_eq!(log_of(&e, MODIFIER_DESTRUCTOR).len(), 2);
    }

    const REMOVE_FN: u32 = 0x0f60_0000;
    const SPAWN_FN: u32 = 0x0f60_0010;

    /// Three particles with ages 0, 5 and 9 and life spans 10; the last
    /// update is 1.0.
    fn update_setup(e: &mut Engine) -> (Ptr<NiPSysAgeDeathModifier>, Ptr<NiPSysData>) {
        e.register(REMOVE_FN, |_, _| Ret::default());
        e.register(SPAWN_FN, |_, _| Ret::default());
        let data = e.new_object::<NiPSysData>();
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + REMOVE_PARTICLE, REMOVE_FN);
        e.mem.set_u32(data.addr(), vtable);
        let info = e.mem.alloc(3 * 0x1c);
        for (i, age) in [0.0f32, 5.0, 9.0].iter().enumerate() {
            let p = info + 0x1c * i as u32;
            e.mem.set_f32(p + 0xc, *age);
            e.mem.set_f32(p + 0x10, 10.0);
            e.mem.set_f32(p + 0x14, 1.0);
        }
        e.set(data, NiPSysData::m_pkParticleInfo, info);
        e.set(data, NiPSysData::m_usActive, 3);
        let this = e.new_object::<NiPSysAgeDeathModifier>();
        e.mem.set_u32(this.addr() + 0x10, 0x7700);
        let spawner = e.mem.alloc(8);
        let spawner_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(spawner_vtable + SPAWN_PARTICLES, SPAWN_FN);
        e.mem.set_u32(spawner, spawner_vtable);
        e.set(this, NiPSysAgeDeathModifier::m_pkSpawnModifier, spawner);
        (this, data)
    }

    #[test]
    fn update_ages_and_removes_the_dead_from_the_last() {
        let mut e = Engine::new();
        let (this, data) = update_setup(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_f010, &args![this, 4.0f32, data]);
        let info = e.get(data, NiPSysData::m_pkParticleInfo);
        assert_eq!(e.mem.f32(info + 0xc), 3.0);
        assert_eq!(e.mem.f32(info + 0x1c + 0xc), 8.0);
        assert_eq!(e.mem.f32(info + 0x38 + 0xc), 12.0);
        // Only particle 2 (the first visited) died; no spawning.
        assert_eq!(log_of(&e, REMOVE_FN), vec![vec![data.addr(), 2]]);
        assert!(log_of(&e, SPAWN_FN).is_empty());
    }

    #[test]
    fn update_spawns_before_removing_and_passes_the_counter_word() {
        let mut e = Engine::new();
        let (this, data) = update_setup(&mut e);
        e.set(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath, 1);
        let info = e.get(data, NiPSysData::m_pkParticleInfo);
        // Particle 0 dies too, visited third: its index word is 0x0002_0000.
        e.mem.set_f32(info + 0xc, 20.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_f010, &args![this, 4.0f32, data]);
        let spawner = e.get(this, NiPSysAgeDeathModifier::m_pkSpawnModifier);
        let spawned = log_of(&e, SPAWN_FN);
        assert_eq!(
            spawned,
            vec![
                vec![spawner, 4.0f32.to_bits(), 2.0f32.to_bits(), 2, 0x7700],
                vec![
                    spawner,
                    4.0f32.to_bits(),
                    (-9.0f32).to_bits(),
                    0x0002_0000,
                    0x7700
                ],
            ]
        );
        assert_eq!(
            log_of(&e, REMOVE_FN),
            vec![vec![data.addr(), 2], vec![data.addr(), 0x0002_0000]]
        );
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| *a == SPAWN_FN || *a == REMOVE_FN)
            .collect();
        assert_eq!(order, vec![SPAWN_FN, REMOVE_FN, SPAWN_FN, REMOVE_FN]);
    }

    #[test]
    fn create_clone_constructs_and_copies_the_flag_only() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysAgeDeathModifier>();
        e.set(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath, 1);
        e.set(this, NiPSysAgeDeathModifier::m_pkSpawnModifier, 0x6600);
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c2_f150, &args![this, 0x4000u32])
            .ptr::<NiPSysAgeDeathModifier>();
        assert_eq!(e.mem.u32(clone.addr()), VTABLE);
        assert_eq!(
            log_of(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x4000]]
        );
        assert_eq!(e.get(clone, NiPSysAgeDeathModifier::m_bSpawnOnDeath), 1);
        assert_eq!(e.get(clone, NiPSysAgeDeathModifier::m_pkSpawnModifier), 0);
    }

    #[test]
    fn process_clone_maps_or_keeps_the_spawn_modifier() {
        for (found, expected) in [(true, 0x6100u32), (false, 0x6000)] {
            let mut e = Engine::new();
            e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
            let this = e.new_object::<NiPSysAgeDeathModifier>();
            let clone = e.new_object::<NiPSysAgeDeathModifier>();
            e.set(this, NiPSysAgeDeathModifier::m_pkSpawnModifier, 0x6000);
            let map = e.mem.alloc(8);
            e.mem.set_u32(map, clone.addr());
            e.mem.set_u32(map + 4, found as u32);
            let process = e.mem.alloc(4);
            e.mem.set_u32(process, map);
            e.register(CLONE_MAP_LOOKUP, |e, a| {
                let found = if a[1] == 0x6000 {
                    let found = e.mem.u32(a[0] + 4);
                    if found != 0 {
                        e.mem.set_u32(a[2], 0x6100);
                    }
                    found
                } else {
                    let clone = e.mem.u32(a[0]);
                    e.mem.set_u32(a[2], clone);
                    1
                };
                Ret {
                    eax: found,
                    ..Ret::default()
                }
            });
            e.call(0x00c2_f1a0, &args![this, process]);
            assert_eq!(
                e.get(clone, NiPSysAgeDeathModifier::m_pkSpawnModifier),
                expected
            );
        }
    }

    #[test]
    fn load_binary_reads_the_flag_then_a_link() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        let stream = test_stream::new(&mut e, 0x1402_0007, &[1]);
        let this = e.new_object::<NiPSysAgeDeathModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c2_f200, &args![this, stream]);
        assert_eq!(e.get(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath), 1);
        assert_eq!(log_of(&e, READ_LINK_ID), vec![vec![stream]]);
    }
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object of `size` bytes whose vtable slot `o` calls `0x0e10_0000 + o`.
    fn object_with_slots(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for slot in 0..0x40u32 {
            e.mem.set_u32(vtable + 4 * slot, 0x0e10_0000 + 4 * slot);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn is_equal_compares_flag_and_spawn_modifiers() {
        let mut e = Engine::new();
        e.register(MODIFIER_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(0x0e10_005c, |_, _| 1u32.into_ret());
        let this: Ptr<NiPSysAgeDeathModifier> = e.new_object();
        let other: Ptr<NiPSysAgeDeathModifier> = e.new_object();
        // Neither has a spawn modifier.
        assert!(e.call(0x00c2_ef80, &args![this, other]).bool());
        // Only one has one.
        let theirs = object_with_slots(&mut e, 0x20);
        e.set(other, NiPSysAgeDeathModifier::m_pkSpawnModifier, theirs);
        assert!(!e.call(0x00c2_ef80, &args![this, other]).bool());
        // Both: the other's spawn modifier compares itself with this one's.
        let mine = object_with_slots(&mut e, 0x20);
        e.set(this, NiPSysAgeDeathModifier::m_pkSpawnModifier, mine);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_ef80, &args![this, other]).bool());
        assert_eq!(calls(&e, 0x0e10_005c), vec![vec![theirs, mine]]);
        e.register(0x0e10_005c, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00c2_ef80, &args![this, other]).bool());
        e.set(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath, 1);
        e.register(0x0e10_005c, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00c2_ef80, &args![this, other]).bool());
        e.set(other, NiPSysAgeDeathModifier::m_bSpawnOnDeath, 1);
        e.register(MODIFIER_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00c2_ef80, &args![this, other]).bool());
    }

    #[test]
    fn save_binary_writes_the_flag_and_link() {
        let mut e = Engine::new();
        e.register(MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0e10_0030, |_, _| Ret::default());
        e.register(0x0e20_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            u32::from(e.mem.u8(a[1])).into_ret()
        });
        let this: Ptr<NiPSysAgeDeathModifier> = e.new_object();
        e.set(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath, 1);
        e.set(this, NiPSysAgeDeathModifier::m_pkSpawnModifier, 0x77);
        let stream = object_with_slots(&mut e, 0x260);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 0xc, 0x0e20_0000);
        e.mem.set_u32(stream + STREAM_OUTPUT_BINARY_STREAM, binary);
        e.call_log = Some(vec![]);
        e.call(0x00c2_f260, &args![this, stream]);
        assert_eq!(
            calls(&e, MODIFIER_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(e.mem.u32(binary + 4), 1);
        assert_eq!(calls(&e, 0x0e20_0000)[0][2], 1);
        assert_eq!(calls(&e, 0x0e10_0030), vec![vec![stream, 0x77]]);
    }

    #[test]
    fn viewer_strings_name_the_spawn_modifier_or_none() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.set_global(AGE_DEATH_MODIFIER_RTTI, 0x0abc_0000u32);
        for addr in [MODIFIER_GET_VIEWER_STRINGS, STRINGS_GROW, STRINGS_SET_AT] {
            e.register(addr, |_, _| Ret::default());
        }
        for addr in [
            NI_GET_VIEWER_STRING,
            VIEWER_STRING_BOOL_00A7D370,
            VIEWER_STRING_TEXT_00A7D5E0,
        ] {
            e.register(addr, |_, a| a[a.len() - 1].into_ret());
        }
        let this: Ptr<NiPSysAgeDeathModifier> = e.new_object();
        let strings = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00c2_f2c0, &args![this, strings]);
        assert_eq!(
            calls(&e, VIEWER_STRING_TEXT_00A7D5E0),
            vec![vec![SPAWN_MODIFIER_NAME, NONE_TEXT]]
        );
        // Full at size 0: each add grows the array first.
        assert_eq!(calls(&e, STRINGS_GROW).len(), 3);
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 3);
        let spawn = e.mem.alloc(0x20);
        e.mem.set_u32(spawn + 8, 0x4242);
        e.set(this, NiPSysAgeDeathModifier::m_pkSpawnModifier, spawn);
        e.set(this, NiPSysAgeDeathModifier::m_bSpawnOnDeath, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_f2c0, &args![this, strings]);
        assert_eq!(
            calls(&e, VIEWER_STRING_BOOL_00A7D370),
            vec![vec![SPAWN_ON_DEATH_NAME, 1]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_TEXT_00A7D5E0),
            vec![vec![SPAWN_MODIFIER_NAME, 0x4242]]
        );
        let slot = calls(&e, STRINGS_SET_AT)[2][2];
        assert_eq!(e.mem.u32(slot), 0x4242);
    }
}
