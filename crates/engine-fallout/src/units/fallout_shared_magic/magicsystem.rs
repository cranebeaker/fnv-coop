//! `fallout shared/magic/magicsystem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Thiscall on a 4-byte stack object (category, word, source file, line),
/// `RET 0x10` (`effectitem.cpp`): the scope guard `GenerateDefaultObjects`
/// opens first, and its destructor (`00404ee0`).
const SCOPE_OPEN: u32 = 0x0040_4eb0;
const SCOPE_CLOSE: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Magic\MagicSystem.cpp"`.
const SOURCE_FILE_TEXT: u32 = 0x0101_3928;
/// Cdecl (form id): the form with that id, or 0 (`tesform.cpp`).
const LOOK_UP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (CRT), cdecl (object, offset, source type, target type,
/// is-reference), and the `TESForm` type descriptor.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The allocator, cdecl (size).
const ALLOCATE: u32 = 0x0040_1000;
/// The data handler (`TESDataHandler *` at `011c3f2c`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// Thiscall on a form list (address of the form word), `RET 4`: appends it.
const LIST_ADD: u32 = 0x005a_e3d0;
/// `EffectSettingCollection::CreateDefaultEffects` (Xbox PDB), cdecl.
const CREATE_DEFAULT_EFFECTS: u32 = 0x0040_8f60;

/// One kind of default object: its RTTI type descriptor, size, constructor,
/// the data handler's accessor of its form list, and whether the code checks
/// the new object for null before using it.
#[derive(Clone, Copy)]
struct DefaultKind {
    type_descriptor: u32,
    size: u32,
    construct: u32,
    form_list: u32,
    checks_null: bool,
}

/// `TESSound` (`.?AVTESSound@@`, 0x6c bytes, `TESSound::TESSound`; the sound
/// list through `0045c650`).
const SOUND: DefaultKind = DefaultKind {
    type_descriptor: 0x0118_32fc,
    size: 0x6c,
    construct: 0x005e_2b70,
    form_list: 0x0045_c650,
    checks_null: false,
};
/// `TESEffectShader` (`.?AVTESEffectShader@@`, 0x170 bytes,
/// `TESEffectShader::TESEffectShader`; its list through `00874670`).
const EFFECT_SHADER: DefaultKind = DefaultKind {
    type_descriptor: 0x0118_3918,
    size: 0x170,
    construct: 0x0050_5c40,
    form_list: 0x0087_4670,
    checks_null: true,
};
/// `SpellItem` (`.?AVSpellItem@@`, 0x44 bytes, `SpellItem::SpellItem`; its
/// list through `0087eaa0`).
const SPELL: DefaultKind = DefaultKind {
    type_descriptor: 0x0118_3060,
    size: 0x44,
    construct: 0x0040_e160,
    form_list: 0x0087_eaa0,
    checks_null: true,
};

/// The default objects in the code's order: (form id, the global that holds
/// it, its editor id, its kind).
const DEFAULT_OBJECTS: [(u32, u32, u32, DefaultKind); 22] = [
    // "MagicFailureSoundAlteration" ... "MagicFailureSoundRestoration".
    (0x12c, 0x011c_3538, 0x0101_3908, SOUND),
    (0x12d, 0x011c_353c, 0x0101_38e8, SOUND),
    (0x12e, 0x011c_3540, 0x0101_38c8, SOUND),
    (0x12f, 0x011c_3544, 0x0101_38ac, SOUND),
    (0x130, 0x011c_3548, 0x0101_3890, SOUND),
    (0x131, 0x011c_354c, 0x0101_3870, SOUND),
    // "MagicEnchantDrawSoundAlteration" ... "MagicEnchantDrawSoundRestoration".
    (0x138, 0x011c_3550, 0x0101_3850, SOUND),
    (0x139, 0x011c_3554, 0x0101_382c, SOUND),
    (0x13a, 0x011c_3558, 0x0101_3808, SOUND),
    (0x13b, 0x011c_355c, 0x0101_37e8, SOUND),
    (0x13c, 0x011c_3560, 0x0101_37c8, SOUND),
    (0x13d, 0x011c_3564, 0x0101_37a4, SOUND),
    // "MagicEnchantHitSoundAlteration" ... "MagicEnchantHitSoundRestoration".
    (0x13e, 0x011c_3568, 0x0101_3784, SOUND),
    (0x13f, 0x011c_356c, 0x0101_3764, SOUND),
    (0x140, 0x011c_3570, 0x0101_3744, SOUND),
    (0x141, 0x011c_3574, 0x0101_3724, SOUND),
    (0x142, 0x011c_3578, 0x0101_3704, SOUND),
    (0x143, 0x011c_357c, 0x0101_36e4, SOUND),
    // "effectAbsorb", "effectReflect", "LifeDetected".
    (0x144, 0x011c_3580, 0x0101_36d4, EFFECT_SHADER),
    (0x145, 0x011c_3584, 0x0101_36c4, EFFECT_SHADER),
    (0x146, 0x011c_3588, 0x0101_36b4, EFFECT_SHADER),
    // "spPipBoyLight".
    (0x147, 0x011c_358c, 0x0101_36a4, SPELL),
];

// Translated from 0040ca50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040ca50 - the C++ exception frame (FS:[0] chain, unwind states 0 to 22 around the scope guard and each allocation) is not translated.
/// `MagicSystem::GenerateDefaultObjects` (Xbox PDB): inside a scope guard
/// (`00404eb0(0x2f, 1, source file, 0x12e)`), finds each of the 22 default
/// forms by id (`004839c0`, cast to its class) and keeps it in its global;
/// a missing one is made (allocated, constructed; for sounds the code goes
/// on through a null pointer when the allocation fails, the effect shaders
/// and the spell are skipped then), given its id (vtable +0x128 with 1), its
/// editor id (vtable +0x134) and vtable +0xc8 with 0, and added to the data
/// handler's list of its kind. Then the default effects
/// (`EffectSettingCollection::CreateDefaultEffects`).
pub fn magic_system_generate_default_objects(e: &mut Engine) {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_OPEN,
            &args![guard, 0x2fu32, 1u32, SOURCE_FILE_TEXT, 0x12eu32],
        );
        for (form_id, global, editor_id, kind) in DEFAULT_OBJECTS {
            let form = e.call(LOOK_UP_FORM, &args![form_id]).u32();
            let object = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, kind.type_descriptor, 0u32],
                )
                .u32();
            e.mem.set_u32(global, object);
            if e.mem.u32(global) != 0 {
                continue;
            }
            let block = e.call(ALLOCATE, &args![kind.size]).u32();
            let made = if block != 0 {
                e.call(kind.construct, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(global, made);
            if kind.checks_null && e.mem.u32(global) == 0 {
                continue;
            }
            let object = e.mem.u32(global);
            e.vcall(object, 0x128, &args![form_id, 1u32]);
            let object = e.mem.u32(global);
            e.vcall(object, 0x134, &args![editor_id]);
            let object = e.mem.u32(global);
            e.vcall(object, 0xc8, &args![0u32]);
            let handler = e.mem.u32(DATA_HANDLER);
            let list = e.call(kind.form_list, &args![handler]).u32();
            e.call(LIST_ADD, &args![list, global]);
        }
        e.call(CREATE_DEFAULT_EFFECTS, &args![]);
        e.call(SCOPE_CLOSE, &args![guard]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_ca50, magic_system_generate_default_objects())]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Doubles: forms with ids in `present` exist (the cast answers the form
    /// id plus `0x0100_0000`), new objects are at `0x5000 + 0x200 * n` with
    /// the vtable `0109_0000`, or null when `null_blocks` is set.
    fn engine(present: &'static [u32], null_blocks: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(DATA_HANDLER, 0x4000);
        e.register(LOOK_UP_FORM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(DYNAMIC_CAST, move |_, a| Ret {
            eax: if present.contains(&a[0]) {
                0x0100_0000 + a[0]
            } else {
                0
            },
            ..Ret::default()
        });
        let mut next = 0x5000u32;
        e.map(0x5000, 0x4000);
        e.register_double(ALLOCATE, move |_, _| {
            let block = if null_blocks { 0 } else { next };
            next += 0x200;
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        for construct in [SOUND.construct, EFFECT_SHADER.construct, SPELL.construct] {
            e.register(construct, |e, a| {
                e.mem.set_u32(a[0], 0x0109_0000);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        e.put_vtable(0x0109_0000, &[0x0050_0000; 0x50]);
        for slot in [0x128u32, 0x134, 0xc8] {
            e.mem.set_u32(0x0109_0000 + slot, 0x0050_0000 + slot);
            e.register(0x0050_0000 + slot, |_, _| Ret::default());
        }
        for list in [SOUND.form_list, EFFECT_SHADER.form_list, SPELL.form_list] {
            e.register_double(list, move |_, _| Ret {
                eax: list,
                ..Ret::default()
            });
        }
        for addr in [SCOPE_OPEN, SCOPE_CLOSE, LIST_ADD, CREATE_DEFAULT_EFFECTS] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn makes_the_missing_default_objects() {
        let mut e = engine(&[0x12d, 0x145], false);
        e.call_log = Some(vec![]);
        e.call(0x0040_ca50, &args![]);
        // 20 of 22 are made: sizes by kind, in order.
        let sizes: Vec<u32> = calls_to(&e, ALLOCATE).iter().map(|c| c[0]).collect();
        assert_eq!(sizes.len(), 20);
        assert_eq!(sizes[..2], [0x6c, 0x6c]);
        assert_eq!(sizes[17..], [0x170, 0x170, 0x44]);
        // The found ones keep the cast result.
        assert_eq!(e.mem.u32(0x011c_353c), 0x0100_012d);
        assert_eq!(e.mem.u32(0x011c_3584), 0x0100_0145);
        assert_eq!(e.mem.u32(0x011c_3538), 0x5000);
        let ids = calls_to(&e, 0x0050_0128);
        assert_eq!(ids[0], vec![0x5000, 0x12c, 1]);
        assert_eq!(ids.last().unwrap(), &vec![e.mem.u32(0x011c_358c), 0x147, 1]);
        assert_eq!(calls_to(&e, 0x0050_0134)[0], vec![0x5000, 0x0101_3908]);
        assert_eq!(calls_to(&e, 0x0050_00c8)[0], vec![0x5000, 0]);
        let adds = calls_to(&e, LIST_ADD);
        assert_eq!(adds[0], vec![SOUND.form_list, 0x011c_3538]);
        assert_eq!(adds.last().unwrap(), &vec![SPELL.form_list, 0x011c_358c]);
        assert_eq!(calls_to(&e, SOUND.form_list)[0], vec![0x4000]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST)[0],
            vec![0x12c, 0, TYPE_TES_FORM, SOUND.type_descriptor, 0]
        );
        assert_eq!(calls_to(&e, CREATE_DEFAULT_EFFECTS).len(), 1);
        let guard = calls_to(&e, SCOPE_OPEN);
        assert_eq!(guard[0][1..], [0x2f, 1, SOURCE_FILE_TEXT, 0x12e]);
        assert_eq!(calls_to(&e, SCOPE_CLOSE), vec![vec![guard[0][0]]]);
    }

    #[test]
    fn failed_allocations_skip_the_checked_kinds() {
        // Every sound exists; only the shaders and the spell are made, and
        // their allocations fail.
        const SOUNDS: &[u32] = &[
            0x12c, 0x12d, 0x12e, 0x12f, 0x130, 0x131, 0x138, 0x139, 0x13a, 0x13b, 0x13c, 0x13d,
            0x13e, 0x13f, 0x140, 0x141, 0x142, 0x143,
        ];
        let mut e = engine(SOUNDS, true);
        e.call_log = Some(vec![]);
        e.call(0x0040_ca50, &args![]);
        assert_eq!(calls_to(&e, ALLOCATE).len(), 4);
        assert!(calls_to(&e, 0x0050_0128).is_empty());
        assert!(calls_to(&e, LIST_ADD).is_empty());
        assert_eq!(e.mem.u32(0x011c_358c), 0);
        assert_eq!(calls_to(&e, CREATE_DEFAULT_EFFECTS).len(), 1);
    }
}
