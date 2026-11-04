//! `fallout shared/formcomponentcollection.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `memset` (cdecl: destination, value, count).
const MEMSET: u32 = 0x0040_3d30;
/// `__RTDynamicCast` (cdecl: object, vfdelta, source type, target type,
/// is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TypeDescriptor` every cast starts from (`.?AVTESForm@@`).
const TES_FORM_TYPE: u32 = 0x0118_3028;

layout! {
    /// `FormComponentCollection` (Xbox PDB), 0x98 bytes on PC as well: 38
    /// `BaseFormComponent*` slots.
    pub struct FormComponentCollection: 0x98 {
        /// `pFCArray[0]` (Xbox PDB): the form itself.
        0x00 pFCArray: Ptr,
    }
}

/// The casts `InitFormComponentArray` makes, in the exe's order: the slot
/// offset the result is stored at and the target `TypeDescriptor` (its
/// decorated name, read from the exe at descriptor + 8, in the comment).
const COMPONENT_CASTS: [(u32, u32); 37] = [
    (0x04, 0x0118_6cd4), // TESAIForm
    (0x0c, 0x0118_6cb4), // TESActorBaseData
    (0x10, 0x0118_6c90), // TESAttackDamageForm
    (0x14, 0x0118_6c74), // TESAttributes
    (0x18, 0x0118_3978), // TESBipedModelForm
    (0x1c, 0x0118_6c58), // TESContainer
    (0x20, 0x0118_38f8), // TESDescription
    (0x24, 0x0118_39b4), // TESEnchantableForm
    (0x28, 0x0118_3158), // TESFullName
    (0x2c, 0x0118_6c3c), // TESHealthForm
    (0x30, 0x0118_3218), // TESTexture
    (0x34, 0x0118_6c1c), // TESLeveledList
    (0x38, 0x0118_31e8), // TESModel
    (0x3c, 0x0118_6bfc), // TESProduceForm
    (0x40, 0x0118_6bdc), // TESQualityForm
    (0x44, 0x0118_6bc0), // TESRaceForm
    (0x48, 0x0118_3254), // TESScriptableForm
    (0x4c, 0x0118_6ba4), // TESSpellList
    (0x50, 0x0118_6b88), // TESUsesForm
    (0x54, 0x0118_6b6c), // TESValueForm
    (0x58, 0x0118_3274), // TESWeightForm
    (0x5c, 0x0118_6b50), // TESSoundFile
    (0x60, 0x0118_6b30), // TESReactionForm
    (0x08, 0x0118_584c), // TESAnimation
    (0x64, 0x0118_6b14), // TESModelList
    (0x68, 0x0118_68d4), // BGSClipRoundsForm
    (0x6c, 0x0118_6890), // BGSAmmoForm
    (0x70, 0x0118_69f8), // BGSIdleCollection
    (0x74, 0x0118_32ac), // BGSDestructibleObjectForm
    (0x78, 0x0118_6a64), // BGSRepairItemList
    (0x7c, 0x0118_3290), // BGSEquipType
    (0x80, 0x0118_68b0), // BGSBipedModelList
    (0x84, 0x0118_6ae8), // TESImageSpaceModifiableForm
    (0x88, 0x0118_6ab0), // TESImageSpaceModifiableCountForm<5>
    (0x8c, 0x0118_3234), // BGSMessageIcon
    (0x90, 0x0118_6a8c), // BGSTouchSpellForm
    (0x94, 0x0118_32d4), // BGSPickupPutdownSounds
];

// Translated from 0047c050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FormComponentCollection::FormComponentCollection` (engine map): clears
/// the component array (`0047c540`) and returns `this`.
pub fn form_component_collection_form_component_collection(
    e: &mut Engine,
    this: Ptr<FormComponentCollection>,
) -> Ptr<FormComponentCollection> {
    fn_0047c540(e, this);
    this
}

// Translated from 0047c540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the whole component array: `memset(this, 0, 0x98)`. The engine
/// map gives it no name.
pub fn fn_0047c540(e: &mut Engine, this: Ptr<FormComponentCollection>) {
    e.call(MEMSET, &args![this, 0u32, 0x98u32]);
}

// Translated from 0047c070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FormComponentCollection::InitFormComponentArray` (engine map): clears
/// the array (`0047c540`), stores `form` in slot 0 and in each other slot
/// the result of `__RTDynamicCast(form, 0, TESForm, <component type>, 0)`
/// (null when the form has no such component). `__thiscall`, one stack
/// word (`RET 4`).
pub fn form_component_collection_init_form_component_array(
    e: &mut Engine,
    this: Ptr<FormComponentCollection>,
    form: Ptr,
) {
    fn_0047c540(e, this);
    e.set(this, FormComponentCollection::pFCArray, form);
    for (offset, target) in COMPONENT_CASTS {
        let component = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TES_FORM_TYPE, target, 0u32],
            )
            .u32();
        e.mem.set_u32(this.addr() + offset, component);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0047_c050,
            form_component_collection_form_component_collection(
                Ptr<FormComponentCollection>,
            )
                -> Ptr<FormComponentCollection>
        ),
        entry!(0x0047_c540, fn_0047c540(Ptr<FormComponentCollection>)),
        entry!(
            0x0047_c070,
            form_component_collection_init_form_component_array(Ptr<FormComponentCollection>, Ptr)
        ),
    ]
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
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn fake_memset(e: &mut Engine, a: &[u32]) -> Ret {
        for i in 0..a[2] {
            e.mem.set_u8(a[0] + i, a[1] as u8);
        }
        Ret::default()
    }

    #[test]
    fn the_constructor_clears_the_array_and_returns_this() {
        let mut e = Engine::new();
        let this = e.new_object::<FormComponentCollection>();
        e.register(MEMSET, fake_memset);
        e.call_log = Some(vec![]);
        let result = e.call(0x0047_c050, &args![this]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(calls_to(&e, MEMSET), vec![vec![this.addr(), 0, 0x98]]);
    }

    #[test]
    fn clearing_sets_all_slots_to_zero() {
        let mut e = Engine::new();
        let this = e.new_object::<FormComponentCollection>();
        for i in 0..0x26 {
            e.mem.set_u32(this.addr() + 4 * i, 0xdead_0000 + i);
        }
        e.register(MEMSET, fake_memset);
        e.call(0x0047_c540, &args![this]);
        for i in 0..0x26 {
            assert_eq!(e.mem.u32(this.addr() + 4 * i), 0);
        }
    }

    #[test]
    fn init_fills_every_slot_from_the_casts_in_order() {
        let mut e = Engine::new();
        let this = e.new_object::<FormComponentCollection>();
        let form = 0x0123_4560u32;
        e.register(MEMSET, fake_memset);
        // The double returns the target descriptor plus one, or null for
        // `TESTexture` (a form without that component).
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: if a[3] == 0x0118_3218 { 0 } else { a[3] + 1 },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0047_c070, &args![this, form]);
        let casts = calls_to(&e, RT_DYNAMIC_CAST);
        assert_eq!(casts.len(), 37);
        for (call, (_, target)) in casts.iter().zip(COMPONENT_CASTS) {
            assert_eq!(call, &vec![form, 0, TES_FORM_TYPE, target, 0]);
        }
        assert_eq!(e.mem.u32(this.addr()), form);
        assert_eq!(e.mem.u32(this.addr() + 0x04), 0x0118_6cd5);
        assert_eq!(e.mem.u32(this.addr() + 0x08), 0x0118_584d);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x94), 0x0118_32d5);
        // memset ran first, on the whole array.
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log[1], (MEMSET, vec![this.addr(), 0, 0x98]));
    }
}
