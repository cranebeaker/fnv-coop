//! `(unplaced)/run_00664440` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions sit inside `fallout shared/speedtree/bstreemanager.cpp`
//! (before `BSTreeManager::GetModelArray` `006649c0`) but the Xbox PDB names
//! no unit for them. They are the `BSTreeManager` constructor, destructor
//! and singleton management. The class is declared here (its own unit file,
//! `units/fallout_shared_speedtree/bstreemanager.rs`, declares no layout
//! yet); the PC offsets are the Xbox PDB ones (0x20 bytes), checked against
//! the constructor.
//!
//! Notes for the next session:
//! - This unit is complete after `00664990`.
//! - The singleton is the pointer at `011d5c48`.
//! - Locals the game keeps on its stack (the critical-section scope guard of
//!   `00664870`, the colour temporaries of the constructor) are `with_stack`
//!   blocks.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSTreeManager` (Xbox PDB), 0x20 bytes.
    pub struct BSTreeManager: 0x20 {
        /// `pModelMap` (Xbox PDB): `NiTPointerMap<TESObjectTREE *, NiPointer<BSTreeModel> *> *`.
        0x00 pModelMap: Ptr,
        /// `spCanopyShadow` (Xbox PDB): `NiPointer<NiTexture>`.
        0x04 spCanopyShadow: Ptr,
        /// `spBillboardMatProp` (Xbox PDB): `NiPointer<NiMaterialProperty>`.
        0x08 spBillboardMatProp: Ptr,
        /// `spDistantAlphaProp` (Xbox PDB): `NiPointer<NiAlphaProperty>`.
        0x0C spDistantAlphaProp: Ptr,
        /// `spHavokFlags` (Xbox PDB): `NiPointer<BSXFlags>`.
        0x10 spHavokFlags: Ptr,
        /// `fWindSpeed` (Xbox PDB).
        0x14 fWindSpeed: f32,
        /// `bShowTrees` (Xbox PDB).
        0x18 bShowTrees: bool,
        /// `bShowLeaves` (Xbox PDB).
        0x19 bShowLeaves: bool,
        /// `bForceCompute` (Xbox PDB).
        0x1A bForceCompute: bool,
        /// `m_bForceFullLOD` (Xbox PDB).
        0x1B m_bForceFullLOD: bool,
        /// `pBackgroundLoadedTreeMap` (Xbox PDB): `LockFreeMap<TESObjectREFR *, BSTreeNode *> *`.
        0x1C pBackgroundLoadedTreeMap: Ptr,
    }
}

/// The singleton pointer (`011d5c48`).
const INSTANCE: u32 = 0x011d_5c48;
/// The `float` static `00664720` stores into (`011fd890`).
const TREE_STATIC_FLOAT: u32 = 0x011f_d890;
/// Statics the destructor clears: two `NiPointer`s (`011f95e4` through
/// `00664820`, `011f95e8` through `00451570`), the byte `01189184` (`00451520`).
const GLOBAL_POINTER_A: u32 = 0x011f_95e4;
/// `NiPointer<T>::operator=(T *)` (`0066b0d0`, thiscall).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer` destructor (`0045cec0`) and `*this` (`00559450`).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer` constructor from a pointer (`00633c90`, thiscall, one word).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// The game's allocator: `operator new` (`00401000`) and the allocator used
/// by `new NiObject` (`00aa13e0`), `operator delete` (`00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The INI setting `bForceFullLOD` (`011d5d00`) and its value accessor
/// (`00408d60`); the setting holding the tree distance (`011d5d98`) and its
/// accessor (`0043d4d0`).
const FORCE_FULL_LOD_SETTING: u32 = 0x011d_5d00;
const TREE_DISTANCE_SETTING: u32 = 0x011d_5d98;
const SETTING_VALUE_BYTE: u32 = 0x0040_8d60;
const SETTING_VALUE_WORD: u32 = 0x0043_d4d0;
/// `NiColor::NiColor(r, g, b)` (`00416870`, thiscall, `RET 0xc`).
const COLOR_CONSTRUCT: u32 = 0x0041_6870;
/// `NiMaterialProperty::NiMaterialProperty` (`00a75650`) and the setters of
/// colours the constructor calls: two no-ops (`004534f0`) and
/// `SetEmittance`-like `004bc450`.
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
const COLOR_SETTER_NOOP: u32 = 0x0045_34f0;
const COLOR_SETTER: u32 = 0x004b_c450;
/// `NiAlphaProperty` constructor (`004391c0`) and its setters (`004393c0`,
/// `004393e0`, `0094db80`, `0049ed90`).
const ALPHA_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
/// `BSXFlags` constructor (`00c42ff0`, one word).
const XFLAGS_CONSTRUCT: u32 = 0x00c4_2ff0;
/// The model map (`00665cb0(size?, hashSize, ..)`, thiscall).
const TREE_MAP_CONSTRUCT: u32 = 0x0066_5cb0;
/// Critical-section scope guard of `00664870`: constructor and destructor.
const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// The path string `BSTreeManager.cpp` the guard is given (`01066398`).
const SOURCE_PATH_STRING: u32 = 0x0106_6398;
/// `00664cd0(0)` run by the destructor first.
const CLEAR_TREES: u32 = 0x0066_4cd0;

// Translation incomplete: 00664440 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 00664440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::BSTreeManager` (constructor, the Xbox PDB class): empty
/// `NiPointer`s, trees and leaves shown, a new white `NiMaterialProperty`
/// (three colours set to `(1, 1, 1)`), a new `NiAlphaProperty` (`004393c0(1)`,
/// `004393e0(4)`, `0094db80(0x54)`, `0049ed90(0)`), a new `BSXFlags` (3),
/// the engine's tree statics (`00b07db0(4)`, `00b084f0(1)`, `00b0d7b0(1)`),
/// the tree float static from the setting at `011d5d98`, and a new
/// background-loaded tree map (`00665cb0(2, 0x25, 0xc)`).
pub fn bs_tree_manager_bs_tree_manager(
    e: &mut Engine,
    this: Ptr<BSTreeManager>,
) -> Ptr<BSTreeManager> {
    let base = this.addr();
    for field in [0x04u32, 0x08, 0x0c, 0x10] {
        e.call(NI_POINTER_CONSTRUCT, &args![base + field, 0u32]);
    }
    e.set(this, BSTreeManager::pModelMap, Ptr::NULL);
    e.set(this, BSTreeManager::fWindSpeed, 0.0);
    e.set(this, BSTreeManager::bShowLeaves, true);
    e.set(this, BSTreeManager::bShowTrees, true);
    e.set(this, BSTreeManager::bForceCompute, false);
    let setting = e
        .call(SETTING_VALUE_BYTE, &args![FORCE_FULL_LOD_SETTING])
        .u32();
    let full_lod = e.mem.u8(setting) != 0;
    e.set(this, BSTreeManager::m_bForceFullLOD, full_lod);

    let material_memory = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    let material = if material_memory != 0 {
        e.call(MATERIAL_PROPERTY_CONSTRUCT, &args![material_memory])
            .u32()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN, &args![base + 0x08, material]);
    e.with_stack(0x24, |e, colors| {
        for (index, setter) in [COLOR_SETTER_NOOP, COLOR_SETTER_NOOP, COLOR_SETTER]
            .into_iter()
            .enumerate()
        {
            let color = colors.addr() + 0x0c * index as u32;
            let made = e
                .call(COLOR_CONSTRUCT, &args![color, 1.0f32, 1.0f32, 1.0f32])
                .u32();
            let property = e.call(NI_POINTER_GET, &args![base + 0x08]).u32();
            e.call(setter, &args![property, made]);
        }
    });

    let alpha_memory = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let alpha = if alpha_memory != 0 {
        e.call(ALPHA_PROPERTY_CONSTRUCT, &args![alpha_memory]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN, &args![base + 0x0c, alpha]);
    for (setter, value) in [
        (0x0043_93c0u32, 1u32),
        (0x0043_93e0, 4),
        (0x0094_db80, 0x54),
        (0x0049_ed90, 0),
    ] {
        let property = e.call(NI_POINTER_GET, &args![base + 0x0c]).u32();
        e.call(setter, &args![property, value]);
    }

    let flags_memory = e.call(NI_ALLOCATE, &args![0x10u32]).u32();
    let flags = if flags_memory != 0 {
        e.call(XFLAGS_CONSTRUCT, &args![flags_memory, 3u32]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN, &args![base + 0x10, flags]);

    e.call(0x00b0_7db0, &args![4u32]);
    e.call(0x00b0_84f0, &args![1u32]);
    e.call(0x00b0_d7b0, &args![1u32]);
    let distance_pointer = e
        .call(SETTING_VALUE_WORD, &args![TREE_DISTANCE_SETTING])
        .u32();
    let distance = e.mem.u32(distance_pointer);
    fn_00664720(e, distance);

    let map_memory = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    let map = if map_memory != 0 {
        e.call(
            TREE_MAP_CONSTRUCT,
            &args![map_memory, 2u32, 0x25u32, 0xcu32],
        )
        .u32()
    } else {
        0
    };
    e.set(this, BSTreeManager::pBackgroundLoadedTreeMap, Ptr::new(map));
    this
}

// Translated from 00664720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` (an unsigned integer) as a `float` into the static at
/// `011fd890` (`FILD` of the 64-bit value `value`).
pub fn fn_00664720(e: &mut Engine, value: u32) {
    e.set_global(TREE_STATIC_FLOAT, value as f32);
}

// Translation incomplete: 00664740 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 00664740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::~BSTreeManager` (destructor, the Xbox PDB class): clears
/// the trees (`00664cd0(0)`), the global `NiPointer`s (`00664820(0)`,
/// `00451570(0)`, `004f3310`), sets the byte at `01189184` (`00451520(1)`),
/// deletes the background-loaded tree map (its virtual slot 0 with `1`) and
/// destroys the four `NiPointer` members.
pub fn bs_tree_manager_destructor(e: &mut Engine, this: Ptr<BSTreeManager>) {
    let base = this.addr();
    e.call(CLEAR_TREES, &args![0u32]);
    fn_00664820(e, 0);
    e.call(0x0045_1570, &args![0u32]);
    e.call(0x0045_1520, &args![1u32]);
    e.call(0x004f_3310, &args![]);
    let map = e.get(this, BSTreeManager::pBackgroundLoadedTreeMap);
    if !map.is_null() {
        e.vcall(map.addr(), 0, &args![1u32]);
    }
    for field in [0x10u32, 0x0c, 0x08, 0x04] {
        e.call(NI_POINTER_DESTRUCT, &args![base + field]);
    }
}

// Translated from 00664820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `value` to the global `NiPointer` at `011f95e4`.
pub fn fn_00664820(e: &mut Engine, value: u32) {
    e.call(NI_POINTER_ASSIGN, &args![GLOBAL_POINTER_A, value]);
}

// Translated from 00664840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSTreeManager` singleton: created through `00664870(0)` when it does
/// not exist yet and `create` is set. Returns the singleton (or null).
pub fn fn_00664840(e: &mut Engine, create: u8) -> Ptr<BSTreeManager> {
    if e.global::<u32>(INSTANCE) == 0 && create != 0 {
        fn_00664870(e, 0);
    }
    Ptr::new(e.global::<u32>(INSTANCE))
}

// Translation incomplete: 00664870 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 00664870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the `BSTreeManager` singleton under a critical-section guard
/// (`00404eb0(0x1f, 1, "...BSTreeManager.cpp", 0xb1)`). When one exists,
/// `replace` set destroys it first (`00664940`); `replace` clear leaves it
/// alone. A new `0x20`-byte manager is built with `00664440`.
pub fn fn_00664870(e: &mut Engine, replace: u8) {
    e.with_stack(8, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, 0x1fu32, 1u32, SOURCE_PATH_STRING, 0xb1u32],
        );
        if e.global::<u32>(INSTANCE) != 0 {
            if replace == 0 {
                e.call(GUARD_DESTRUCT, &args![guard]);
                return;
            }
            fn_00664940(e);
        }
        let memory = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
        let manager = if memory != 0 {
            bs_tree_manager_bs_tree_manager(e, Ptr::new(memory)).addr()
        } else {
            0
        };
        e.set_global(INSTANCE, manager);
        e.call(GUARD_DESTRUCT, &args![guard]);
    })
}

// Translated from 00664940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the singleton (scalar deleting destructor `00664990(1)`) and
/// clears the pointer.
pub fn fn_00664940(e: &mut Engine) {
    let manager = e.global::<u32>(INSTANCE);
    if manager != 0 {
        if manager != 0 {
            fn_00664990(e, Ptr::new(manager), 1);
        }
        e.set_global(INSTANCE, 0u32);
    }
}

// Translated from 00664990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::__scalar_deleting_destructor_`: runs the destructor and,
/// when bit 0 of `flags` is set, deletes the memory. Returns `this`.
pub fn fn_00664990(e: &mut Engine, this: Ptr<BSTreeManager>, flags: u32) -> Ptr<BSTreeManager> {
    bs_tree_manager_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00664440,
            bs_tree_manager_bs_tree_manager(Ptr<BSTreeManager>) -> Ptr<BSTreeManager>
        ),
        entry!(0x00664720, fn_00664720(u32)),
        entry!(0x00664740, bs_tree_manager_destructor(Ptr<BSTreeManager>)),
        entry!(0x00664820, fn_00664820(u32)),
        entry!(0x00664840, fn_00664840(u8) -> Ptr<BSTreeManager>),
        entry!(0x00664870, fn_00664870(u8)),
        entry!(0x00664940, fn_00664940()),
        entry!(
            0x00664990,
            fn_00664990(Ptr<BSTreeManager>, u32) -> Ptr<BSTreeManager>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Doubles that record every call (address and words), answering zero.
    fn recording(e: &mut Engine, addrs: &[u32]) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for a in addrs {
            let a = *a;
            let log = log.clone();
            e.register_double(a, move |_, w| {
                log.borrow_mut().push((a, w.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Everything the destructor calls.
    const DESTRUCTOR_CALLEES: [u32; 6] = [
        CLEAR_TREES,
        NI_POINTER_ASSIGN,
        0x0045_1570,
        0x0045_1520,
        0x004f_3310,
        NI_POINTER_DESTRUCT,
    ];

    #[test]
    fn constructor_builds_the_properties_and_the_map() {
        let mut e = Engine::new();
        let log = recording(
            &mut e,
            &[
                NI_POINTER_CONSTRUCT,
                NI_POINTER_ASSIGN,
                0x0043_93c0,
                0x0043_93e0,
                0x0094_db80,
                0x0049_ed90,
                0x00b0_7db0,
                0x00b0_84f0,
                0x00b0_d7b0,
                COLOR_SETTER_NOOP,
                COLOR_SETTER,
            ],
        );
        // The settings: a full-LOD byte of 1, the tree distance word 100.
        let setting_byte = e.mem.alloc(8);
        e.mem.set_u8(setting_byte, 1);
        let setting_word = e.mem.alloc(8);
        e.mem.set_u32(setting_word, 100);
        e.register_double(SETTING_VALUE_BYTE, move |_, _| ret(setting_byte));
        e.register_double(SETTING_VALUE_WORD, move |_, _| ret(setting_word));
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MATERIAL_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ALPHA_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(XFLAGS_CONSTRUCT, |_, a| ret(a[0]));
        e.register(TREE_MAP_CONSTRUCT, |_, a| ret(a[0] + 0x100));
        e.register(COLOR_CONSTRUCT, |e, a| {
            for lane in 0..3 {
                e.mem.set_u32(a[0] + 4 * lane, a[1 + lane as usize]);
            }
            ret(a[0])
        });
        e.register(NI_POINTER_GET, |_, a| ret(a[0] + 0x1000));
        e.map(TREE_STATIC_FLOAT, 8);
        let this = e.new_object::<BSTreeManager>();
        e.mem.write(this.addr(), &[0xaa; 0x20]);
        let result = e.call(0x0066_4440, &args![this]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(e.get(this, BSTreeManager::pModelMap), Ptr::NULL);
        assert_eq!(e.get(this, BSTreeManager::fWindSpeed), 0.0);
        assert!(e.get(this, BSTreeManager::bShowTrees));
        assert!(e.get(this, BSTreeManager::bShowLeaves));
        assert!(!e.get(this, BSTreeManager::bForceCompute));
        assert!(e.get(this, BSTreeManager::m_bForceFullLOD));
        let map = e.get(this, BSTreeManager::pBackgroundLoadedTreeMap);
        assert_ne!(map, Ptr::NULL);
        assert_eq!(e.global::<f32>(TREE_STATIC_FLOAT), 100.0);
        // The four NiPointer members are built empty.
        let constructs = calls_to(&log, NI_POINTER_CONSTRUCT);
        let base = this.addr();
        assert_eq!(
            constructs,
            vec![
                vec![base + 4, 0],
                vec![base + 8, 0],
                vec![base + 0xc, 0],
                vec![base + 0x10, 0]
            ]
        );
        // Two no-op colour setters then the real one, each given a white colour.
        assert_eq!(calls_to(&log, COLOR_SETTER_NOOP).len(), 2);
        let setter = calls_to(&log, COLOR_SETTER);
        assert_eq!(setter.len(), 1);
        let color = setter[0][1];
        assert_eq!(e.mem.f32(color), 1.0);
        assert_eq!(e.mem.f32(color + 8), 1.0);
        // The alpha property setters get 1, 4, 0x54, 0.
        let values: Vec<u32> = [0x0043_93c0u32, 0x0043_93e0, 0x0094_db80, 0x0049_ed90]
            .iter()
            .map(|a| calls_to(&log, *a)[0][1])
            .collect();
        assert_eq!(values, vec![1, 4, 0x54, 0]);
        assert_eq!(calls_to(&log, 0x00b0_7db0), vec![vec![4]]);
        assert_eq!(calls_to(&log, 0x00b0_84f0), vec![vec![1]]);
        // Three property pointers were assigned (material, alpha, flags).
        let assigns = calls_to(&log, NI_POINTER_ASSIGN);
        assert_eq!(assigns.len(), 3);
        assert_eq!(assigns[0][0], base + 8);
        assert_eq!(assigns[1][0], base + 0xc);
        assert_eq!(assigns[2][0], base + 0x10);
    }

    #[test]
    fn constructor_leaves_null_pointers_when_allocation_fails() {
        let mut e = Engine::new();
        recording(
            &mut e,
            &[
                NI_POINTER_CONSTRUCT,
                NI_POINTER_ASSIGN,
                0x0043_93c0,
                0x0043_93e0,
                0x0094_db80,
                0x0049_ed90,
                0x00b0_7db0,
                0x00b0_84f0,
                0x00b0_d7b0,
                COLOR_SETTER_NOOP,
                COLOR_SETTER,
                COLOR_CONSTRUCT,
            ],
        );
        let setting = e.mem.alloc(8);
        e.register_double(SETTING_VALUE_BYTE, move |_, _| ret(setting));
        e.register_double(SETTING_VALUE_WORD, move |_, _| ret(setting));
        e.register(NI_ALLOCATE, |_, _| ret(0));
        e.register(OPERATOR_NEW, |_, _| ret(0));
        e.register(NI_POINTER_GET, |_, _| ret(0));
        e.map(TREE_STATIC_FLOAT, 8);
        let this = e.new_object::<BSTreeManager>();
        e.mem.set_u32(this.addr() + 0x1c, 0xdead);
        bs_tree_manager_bs_tree_manager(&mut e, this);
        assert_eq!(
            e.get(this, BSTreeManager::pBackgroundLoadedTreeMap),
            Ptr::NULL
        );
        assert!(!e.get(this, BSTreeManager::m_bForceFullLOD));
    }

    #[test]
    fn the_float_static_is_the_unsigned_value() {
        let mut e = Engine::new();
        e.map(TREE_STATIC_FLOAT, 8);
        e.call(0x0066_4720, &args![0xffff_ffffu32]);
        assert_eq!(e.global::<f32>(TREE_STATIC_FLOAT), 4294967296.0);
        e.call(0x0066_4720, &args![7u32]);
        assert_eq!(e.global::<f32>(TREE_STATIC_FLOAT), 7.0);
    }

    #[test]
    fn destructor_clears_statics_deletes_the_map_and_destroys_members() {
        let mut e = Engine::new();
        let log = recording(&mut e, &DESTRUCTOR_CALLEES);
        let map_deleter = recording(&mut e, &[0x0060_c600]);
        let mut slots = vec![0u32; 4];
        slots[0] = 0x0060_c600;
        e.put_vtable(0x0a00_4000, &slots);
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, 0x0a00_4000);
        let this = e.new_object::<BSTreeManager>();
        e.set(this, BSTreeManager::pBackgroundLoadedTreeMap, Ptr::new(map));
        e.call(0x0066_4740, &args![this]);
        let base = this.addr();
        assert_eq!(calls_to(&log, CLEAR_TREES), vec![vec![0]]);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![GLOBAL_POINTER_A, 0]]
        );
        assert_eq!(calls_to(&log, 0x0045_1570), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x0045_1520), vec![vec![1]]);
        assert_eq!(calls_to(&map_deleter, 0x0060_c600), vec![vec![map, 1]]);
        assert_eq!(
            calls_to(&log, NI_POINTER_DESTRUCT),
            vec![
                vec![base + 0x10],
                vec![base + 0xc],
                vec![base + 8],
                vec![base + 4]
            ]
        );
    }

    #[test]
    fn destructor_without_a_map_skips_the_delete() {
        let mut e = Engine::new();
        recording(&mut e, &DESTRUCTOR_CALLEES);
        let this = e.new_object::<BSTreeManager>();
        // No map: nothing is called through a vtable (this would panic).
        bs_tree_manager_destructor(&mut e, this);
    }

    #[test]
    fn global_pointer_assignment() {
        let mut e = Engine::new();
        let log = recording(&mut e, &[NI_POINTER_ASSIGN]);
        e.call(0x0066_4820, &args![0x1234u32]);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![GLOBAL_POINTER_A, 0x1234]]
        );
    }

    /// Doubles for everything the constructor calls, answering zero or an
    /// allocation.
    fn quiet_constructor(e: &mut Engine) {
        for a in [
            NI_POINTER_CONSTRUCT,
            NI_POINTER_ASSIGN,
            0x0043_93c0u32,
            0x0043_93e0,
            0x0094_db80,
            0x0049_ed90,
            0x00b0_7db0,
            0x00b0_84f0,
            0x00b0_d7b0,
            COLOR_SETTER_NOOP,
            COLOR_SETTER,
            COLOR_CONSTRUCT,
            NI_POINTER_GET,
            MATERIAL_PROPERTY_CONSTRUCT,
            ALPHA_PROPERTY_CONSTRUCT,
            XFLAGS_CONSTRUCT,
            TREE_MAP_CONSTRUCT,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        let setting = e.mem.alloc(8);
        e.register_double(SETTING_VALUE_BYTE, move |_, _| ret(setting));
        e.register_double(SETTING_VALUE_WORD, move |_, _| ret(setting));
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.map(TREE_STATIC_FLOAT, 8);
    }

    /// `OPERATOR_NEW` that counts the 0x20-byte requests (the managers).
    fn counting_allocator(e: &mut Engine) -> Rc<RefCell<u32>> {
        let created = Rc::new(RefCell::new(0u32));
        let count = created.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            if a[0] == 0x20 {
                *count.borrow_mut() += 1;
            }
            ret(e.mem.alloc(a[0]))
        });
        created
    }

    #[test]
    fn singleton_accessor_creates_only_when_asked() {
        let mut e = Engine::new();
        e.map(INSTANCE, 4);
        quiet_constructor(&mut e);
        e.register(GUARD_CONSTRUCT, |_, _| Ret::default());
        e.register(GUARD_DESTRUCT, |_, _| Ret::default());
        let created = counting_allocator(&mut e);
        assert_eq!(fn_00664840(&mut e, 0), Ptr::NULL);
        assert_eq!(*created.borrow(), 0);
        let manager = fn_00664840(&mut e, 1);
        assert_ne!(manager, Ptr::NULL);
        assert_eq!(*created.borrow(), 1);
        // Already there: returned as is, nothing else created.
        assert_eq!(e.call(0x0066_4840, &args![1u32]).u32(), manager.addr());
        assert_eq!(*created.borrow(), 1);
    }

    #[test]
    fn create_keeps_an_existing_singleton_unless_replace_is_set() {
        let mut e = Engine::new();
        e.map(INSTANCE, 4);
        let guard = recording(&mut e, &[GUARD_CONSTRUCT, GUARD_DESTRUCT]);
        let deleted = recording(&mut e, &[OPERATOR_DELETE]);
        recording(&mut e, &DESTRUCTOR_CALLEES);
        quiet_constructor(&mut e);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let old = e.mem.alloc(0x20);
        e.set_global(INSTANCE, old);
        e.call(0x0066_4870, &args![0u32]);
        assert_eq!(e.global::<u32>(INSTANCE), old);
        assert_eq!(
            calls_to(&guard, GUARD_CONSTRUCT)[0][1..],
            [0x1f, 1, SOURCE_PATH_STRING, 0xb1]
        );
        assert_eq!(calls_to(&guard, GUARD_DESTRUCT).len(), 1);
        assert!(calls_to(&deleted, OPERATOR_DELETE).is_empty());
        // `replace` destroys the old one and builds a new one.
        e.call(0x0066_4870, &args![1u32]);
        assert_ne!(e.global::<u32>(INSTANCE), old);
        assert_ne!(e.global::<u32>(INSTANCE), 0);
        assert_eq!(calls_to(&deleted, OPERATOR_DELETE), vec![vec![old]]);
        assert_eq!(calls_to(&guard, GUARD_DESTRUCT).len(), 2);
    }

    #[test]
    fn a_failed_allocation_leaves_a_null_singleton() {
        let mut e = Engine::new();
        e.map(INSTANCE, 4);
        recording(&mut e, &[GUARD_CONSTRUCT, GUARD_DESTRUCT]);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        e.set_global(INSTANCE, 0u32);
        fn_00664870(&mut e, 0);
        assert_eq!(e.global::<u32>(INSTANCE), 0);
    }

    #[test]
    fn destroy_clears_the_singleton_and_ignores_none() {
        let mut e = Engine::new();
        e.map(INSTANCE, 4);
        let deleted = recording(&mut e, &[OPERATOR_DELETE]);
        recording(&mut e, &DESTRUCTOR_CALLEES);
        fn_00664940(&mut e);
        assert!(calls_to(&deleted, OPERATOR_DELETE).is_empty());
        let manager = e.mem.alloc(0x20);
        e.set_global(INSTANCE, manager);
        e.call(0x0066_4940, &args![]);
        assert_eq!(e.global::<u32>(INSTANCE), 0);
        assert_eq!(calls_to(&deleted, OPERATOR_DELETE), vec![vec![manager]]);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_with_bit_zero() {
        let mut e = Engine::new();
        let deleted = recording(&mut e, &[OPERATOR_DELETE]);
        recording(&mut e, &DESTRUCTOR_CALLEES);
        let this = e.new_object::<BSTreeManager>();
        assert_eq!(e.call(0x0066_4990, &args![this, 0u32]).u32(), this.addr());
        assert!(calls_to(&deleted, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0066_4990, &args![this, 3u32]).u32(), this.addr());
        assert_eq!(calls_to(&deleted, OPERATOR_DELETE), vec![vec![this.addr()]]);
    }
}
