//! `fallout/misc/scenegraph.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `SceneGraph` derives from `BSSceneGraph` (constructed by `00c517b0`,
//! destroyed by `00c51df0`). The PC object is 0xc0 bytes (the size the
//! scalar deleting destructor frees); the Xbox layout (`pdbdump type
//! SceneGraph`, 0xf0 bytes) does not apply past the base class, so only the
//! one byte the code reads is declared, unnamed.

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSSceneGraph::BSSceneGraph` (Xbox PDB), thiscall with three stack words.
const BS_SCENE_GRAPH_CONSTRUCT: u32 = 0x00c5_17b0;
/// The base class destructor body (`00c51df0`).
const BS_SCENE_GRAPH_DESTRUCT: u32 = 0x00c5_1df0;
/// `SceneGraph`'s vtable, installed by the constructor and the destructor.
const SCENE_GRAPH_VTABLE: u32 = 0x0108_3b5c;
/// `SceneGraph::ms_RTTI` (a `NiRTTI` in `.data`).
const SCENE_GRAPH_RTTI: u32 = 0x011d_f0c0;
/// `operator delete` with a size (`00aa1460`, cdecl: pointer, size).
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;
/// Size of a `SceneGraph` on the PC.
const SCENE_GRAPH_SIZE: u32 = 0xc0;

/// `fLODFarDistance` (a `const float`, Xbox PDB name): returned as is when
/// the byte at `+0xb8` is set.
const FIXED_FAR_DISTANCE: u32 = 0x0108_3c64;
/// A `float` in the exe's constants: the far distance when there is no
/// interior cell or the cell distance is small, and the cap otherwise.
const DEFAULT_FAR_DISTANCE: u32 = 0x010c_26d8;
/// A `double` threshold for the cell's distance (`0x01012060`).
const CELL_DISTANCE_THRESHOLD: u32 = 0x0101_2060;
/// The `TES` singleton pointer (`011dea10`).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `TES::GetWorldSpace` (Xbox PDB): thiscall on the `TES`.
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `TESWorldSpace::GetTerrainManager` (Xbox PDB).
const WORLD_SPACE_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
/// `00454b70`: a test on the object `00586170` returns (a byte).
const TERRAIN_MANAGER_TEST: u32 = 0x0045_4b70;
/// `005f36f0` (named `ActorMover::GetPreferredMoveMode` in the map because
/// of code folding; its body returns `TES::pInteriorCell`): thiscall on
/// the `TES`.
const TES_INTERIOR_CELL: u32 = 0x005f_36f0;
/// `MiddleHighProcess::GetSavedAcquireObject` (name from the map, a folded
/// body; here it is called on the `TES`).
const TES_FOLDED_GETTER: u32 = 0x008d_8520;
/// `00476c90`: a test on what `008d8520` returned (an integer).
const FOLDED_GETTER_TEST: u32 = 0x0047_6c90;
/// `00544bd0`: thiscall on the cell; returns a `float` in ST0.
const CELL_DISTANCE: u32 = 0x0054_4bd0;
/// `00403e20`: thiscall on a setting object (`ECX`); returns the address of
/// its `float` value.
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// Three `float` settings (objects whose value `00403e20` locates): the one
/// that replaces a large cell distance, and the two `004b3ab0` gets as its
/// first and last parameters. Their game names are unconfirmed.
const SETTING_A: u32 = 0x0120_312c;
const SETTING_B: u32 = 0x0120_3138;
const SETTING_C: u32 = 0x0120_3120;
/// `004b3ab0`: cdecl, five `float`s (`SETTING_C`'s value, the distance, 0.0,
/// 1.0, `SETTING_B`'s value); returns a `float` in ST0.
const COMBINE: u32 = 0x004b_3ab0;

layout! {
    /// `SceneGraph` (0xc0 bytes on the PC); only the byte `008786b0` tests
    /// is declared.
    pub struct SceneGraph: 0xc0 {
        /// Tested by `GetFarDistance`; unconfirmed meaning.
        0xb8 flag_b8: u8,
    }
}

// Translated from 00878610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SceneGraph::SceneGraph` (Xbox PDB): runs `BSSceneGraph`'s constructor
/// (`00c517b0`) with the three stack words, then installs `SceneGraph`'s
/// vtable. Returns `this`.
pub fn scene_graph_construct(
    e: &mut Engine,
    this: Ptr<SceneGraph>,
    first: u32,
    flag: u8,
    third: u32,
) -> Ptr<SceneGraph> {
    e.call(BS_SCENE_GRAPH_CONSTRUCT, &args![this, first, flag, third]);
    e.mem.set_u32(this.addr(), SCENE_GRAPH_VTABLE);
    this
}

// Translated from 00878640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SceneGraph::GetRTTI` (Xbox PDB): the address of `SceneGraph::ms_RTTI`.
pub fn scene_graph_get_rtti(_e: &mut Engine, _this: Ptr<SceneGraph>) -> u32 {
    SCENE_GRAPH_RTTI
}

// Translated from 00878650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SceneGraph::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body (`00878690`) and, when bit 0 of `flags` is set, frees
/// the object (0xc0 bytes). Returns `this`.
pub fn scene_graph_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<SceneGraph>,
    flags: u32,
) -> Ptr<SceneGraph> {
    fn_00878690(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, SCENE_GRAPH_SIZE]);
    }
    this
}

// Translated from 00878690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body: installs `SceneGraph`'s vtable and runs the base
/// class destructor body (`00c51df0`).
pub fn fn_00878690(e: &mut Engine, this: Ptr<SceneGraph>) {
    e.mem.set_u32(this.addr(), SCENE_GRAPH_VTABLE);
    e.call(BS_SCENE_GRAPH_DESTRUCT, &args![this]);
}

/// The float at the address a setting's accessor (`00403e20`) returns for
/// `setting`, copied the way `FLD`/`FSTP` copy it.
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let address = e.call(SETTING_FLOAT_ADDRESS, &args![setting]).u32();
    f32::from_bits(x87_quiet_bits(e.mem.u32(address)))
}

/// The distance of the interior cell, when there is one: the cell
/// accessor, a check that the object `008d8520` returns passes
/// `00476c90 == 1`, and the float `00544bd0` returns (stored as a `float`).
/// Whether a `float` compare against the threshold leaves the "above" path of
/// the code: it does for a value above it and for a NaN (the x87 compare is
/// unordered), not for a value at or below it.
fn above_or_unordered(value: f32, threshold: f64) -> bool {
    value as f64 > threshold || value.is_nan()
}

fn interior_cell_distance(e: &mut Engine, tes: u32) -> Option<f32> {
    if tes == 0 || e.call(TES_INTERIOR_CELL, &args![tes]).u32() == 0 {
        return None;
    }
    let object = e.call(TES_FOLDED_GETTER, &args![tes]).u32();
    if e.call(FOLDED_GETTER_TEST, &args![object]).u32() != 1 {
        return None;
    }
    let cell = e.call(TES_INTERIOR_CELL, &args![tes]).u32();
    Some(e.call(CELL_DISTANCE, &args![cell]).f32())
}

// Translated from 008786b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SceneGraph::GetFarDistance` (Xbox PDB): the fixed far distance
/// (`01083c64`) when the byte at `+0xb8` is set. Otherwise it asks the
/// `TES` singleton (`011dea10`) whether it has a world space with a terrain
/// manager passing `00454b70`:
///
/// - if so (or if there is none to ask) the result is the default
///   (`010c26d8`) unless the interior cell's distance exists, exceeds the
///   threshold (`01012060`, a `double`) and is not above the default, in
///   which case it is that distance (a NaN distance is returned as is);
/// - if not, the far distance is derived from the settings at `0120312c`,
///   `01203138` and `01203120`: the cell's distance, when it exists and
///   exceeds the threshold, is replaced by the first setting if that is
///   below it; the value then goes through `004b3ab0` with the two other
///   settings.
pub fn scene_graph_get_far_distance(e: &mut Engine, this: Ptr<SceneGraph>) -> f32 {
    if e.get(this, SceneGraph::flag_b8) != 0 {
        return f32::from_bits(x87_quiet_bits(e.global::<u32>(FIXED_FAR_DISTANCE)));
    }
    let tes = e.global::<u32>(TES_SINGLETON);
    let threshold: f64 = e.global(CELL_DISTANCE_THRESHOLD);
    let default_distance = f32::from_bits(x87_quiet_bits(e.global::<u32>(DEFAULT_FAR_DISTANCE)));
    let mut use_default_path = true;
    if tes != 0 {
        let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
        if world_space != 0 {
            let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
            let terrain = e
                .call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world_space])
                .u32();
            use_default_path = e.call(TERRAIN_MANAGER_TEST, &args![terrain]).u8() != 0;
        }
    }
    if use_default_path {
        return match interior_cell_distance(e, tes) {
            Some(distance) if above_or_unordered(distance, threshold) => {
                if default_distance < distance {
                    default_distance
                } else {
                    distance
                }
            }
            _ => default_distance,
        };
    }
    let distance = match interior_cell_distance(e, tes) {
        Some(distance) if above_or_unordered(distance, threshold) => {
            let setting = setting_value(e, SETTING_A);
            if setting < distance {
                setting_value(e, SETTING_A)
            } else {
                distance
            }
        }
        _ => setting_value(e, SETTING_A),
    };
    let setting_b = setting_value(e, SETTING_B);
    let setting_c = setting_value(e, SETTING_C);
    e.call(
        COMBINE,
        &args![setting_c, distance, 0.0f32, 1.0f32, setting_b],
    )
    .f32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00878610,
            scene_graph_construct(Ptr<SceneGraph>, u32, u8, u32) -> Ptr<SceneGraph>
        ),
        entry!(0x00878640, scene_graph_get_rtti(Ptr<SceneGraph>) -> u32),
        entry!(
            0x00878650,
            scene_graph_scalar_deleting_destructor(Ptr<SceneGraph>, u32) -> Ptr<SceneGraph>
        ),
        entry!(0x00878690, fn_00878690(Ptr<SceneGraph>)),
        entry!(
            0x008786b0,
            scene_graph_get_far_distance(Ptr<SceneGraph>) -> f32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word_ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn constructor_runs_the_base_then_installs_the_vtable() {
        let mut e = Engine::new();
        e.register(BS_SCENE_GRAPH_CONSTRUCT, |_, _| Ret::default());
        let graph = e.new_object::<SceneGraph>();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0087_8610, &[graph.addr(), 0x77, 0x1ff, 0x88])
                .u32(),
            graph.addr()
        );
        let log = e.call_log.take().unwrap();
        // The flag is a byte: only its low byte is passed on.
        assert_eq!(
            log[1],
            (
                BS_SCENE_GRAPH_CONSTRUCT,
                vec![graph.addr(), 0x77, 0xff, 0x88]
            )
        );
        assert_eq!(e.mem.u32(graph.addr()), 0x0108_3b5c);
    }

    #[test]
    fn rtti_is_the_static_one() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0087_8640, &[0x1234]).u32(), 0x011d_f0c0);
    }

    #[test]
    fn deleting_destructor_frees_only_when_asked() {
        let mut e = Engine::new();
        e.register(BS_SCENE_GRAPH_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE_SIZED, |_, _| Ret::default());
        let graph = e.new_object::<SceneGraph>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_8650, &[graph.addr(), 0]).u32(), graph.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (BS_SCENE_GRAPH_DESTRUCT, vec![graph.addr()]));
        assert_eq!(e.mem.u32(graph.addr()), 0x0108_3b5c);
        e.call_log = Some(vec![]);
        e.call(0x0087_8650, &[graph.addr(), 3]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            *log.last().unwrap(),
            (OPERATOR_DELETE_SIZED, vec![graph.addr(), 0xc0])
        );
        e.call_log = Some(vec![]);
        e.call(0x0087_8690, &[graph.addr()]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    /// A far-distance fixture: the constants, a `TES` with a world space,
    /// and doubles for every accessor. `cell_distance` is what `00544bd0`
    /// returns.
    fn far_distance_engine(cell_distance: f32) -> (Engine, Ptr<SceneGraph>) {
        let mut e = Engine::new();
        for (addr, size) in [
            (FIXED_FAR_DISTANCE, 4),
            (DEFAULT_FAR_DISTANCE, 4),
            (CELL_DISTANCE_THRESHOLD, 8),
            (TES_SINGLETON, 4),
        ] {
            e.map(addr, size);
        }
        e.mem.set_f32(FIXED_FAR_DISTANCE, 777.0);
        e.mem.set_f32(DEFAULT_FAR_DISTANCE, 100.0);
        e.mem.set_f64(CELL_DISTANCE_THRESHOLD, 10.0);
        e.set_global(TES_SINGLETON, 0x0f00_0000u32);
        // The settings: three float cells.
        let values = e.mem.alloc(12);
        e.mem.set_f32(values, 40.0);
        e.mem.set_f32(values + 4, 5.0);
        e.mem.set_f32(values + 8, 3.0);
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, a| {
            word_ret(match a[0] {
                SETTING_A => values,
                SETTING_B => values + 4,
                SETTING_C => values + 8,
                other => panic!("unexpected setting {other:#x}"),
            })
        });
        e.register(TES_GET_WORLD_SPACE, |_, _| word_ret(0x0f10_0000));
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, _| {
            word_ret(0x0f20_0000)
        });
        e.register(TERRAIN_MANAGER_TEST, |_, _| word_ret(0));
        e.register(TES_INTERIOR_CELL, |_, _| word_ret(0x0f30_0000));
        e.register(TES_FOLDED_GETTER, |_, _| word_ret(0x0f40_0000));
        e.register(FOLDED_GETTER_TEST, |_, _| word_ret(1));
        e.register_double(CELL_DISTANCE, move |_, _| cell_distance.into_ret());
        e.register(COMBINE, |_, a| {
            // Encodes the argument order (first, distance, 0.0, 1.0, last) in
            // the result: distance * 1000 + first * 10 + last.
            assert_eq!(f32::from_bits(a[2]), 0.0);
            assert_eq!(f32::from_bits(a[3]), 1.0);
            (f32::from_bits(a[1]) * 1000.0 + f32::from_bits(a[0]) * 10.0 + f32::from_bits(a[4]))
                .into_ret()
        });
        let graph = e.new_object::<SceneGraph>();
        (e, graph)
    }

    #[test]
    fn fixed_far_distance_when_the_flag_is_set() {
        let (mut e, graph) = far_distance_engine(50.0);
        e.set(graph, SceneGraph::flag_b8, 1);
        assert_eq!(e.call(0x0087_86b0, &[graph.addr()]).f32(), 777.0);
    }

    #[test]
    fn terrain_check_passing_uses_the_default_distance_path() {
        let (mut e, graph) = far_distance_engine(50.0);
        e.register(TERRAIN_MANAGER_TEST, |_, _| word_ret(1));
        // Cell distance 50 is above the threshold but not below the default (100): it wins.
        assert_eq!(e.call(0x0087_86b0, &[graph.addr()]).f32(), 50.0);
        // Above the default: the default caps it.
        let (mut e, graph) = far_distance_engine(150.0);
        e.register(TERRAIN_MANAGER_TEST, |_, _| word_ret(1));
        assert_eq!(e.call(0x0087_86b0, &[graph.addr()]).f32(), 100.0);
        // At or below the threshold: the default.
        let (mut e, graph) = far_distance_engine(10.0);
        e.register(TERRAIN_MANAGER_TEST, |_, _| word_ret(1));
        assert_eq!(e.call(0x0087_86b0, &[graph.addr()]).f32(), 100.0);
        // NaN: the compare is unordered, so the second compare keeps the NaN.
        let (mut e, graph) = far_distance_engine(f32::NAN);
        e.register(TERRAIN_MANAGER_TEST, |_, _| word_ret(1));
        assert!(e.call(0x0087_86b0, &[graph.addr()]).f32().is_nan());
    }

    #[test]
    fn no_tes_or_no_world_space_uses_the_default_distance_path() {
        let (mut e, graph) = far_distance_engine(50.0);
        e.set_global(TES_SINGLETON, 0u32);
        assert_eq!(e.call(0x0087_86b0, &[graph.addr()]).f32(), 100.0);
        let (mut e, graph) = far_distance_engine(50.0);
        e.register(TES_GET_WORLD_SPACE, |_, _| word_ret(0));
        assert_eq!(e.call(0x0087_86b0, &[graph.addr()]).f32(), 50.0);
        // No interior cell: the default.
        let (mut e, graph) = far_distance_engine(50.0);
        e.register(TES_GET_WORLD_SPACE, |_, _| word_ret(0));
        e.register(TES_INTERIOR_CELL, |_, _| word_ret(0));
        assert_eq!(e.call(0x0087_86b0, &[graph.addr()]).f32(), 100.0);
        // The folded getter's object fails its test.
        let (mut e, graph) = far_distance_engine(50.0);
        e.register(TES_GET_WORLD_SPACE, |_, _| word_ret(0));
        e.register(FOLDED_GETTER_TEST, |_, _| word_ret(2));
        assert_eq!(e.call(0x0087_86b0, &[graph.addr()]).f32(), 100.0);
    }

    #[test]
    fn settings_path_combines_the_distance_with_the_two_other_settings() {
        // Cell distance 50 is above the first setting (40): the setting replaces it.
        let (mut e, graph) = far_distance_engine(50.0);
        assert_eq!(
            e.call(0x0087_86b0, &[graph.addr()]).f32(),
            40.0 * 1000.0 + 3.0 * 10.0 + 5.0
        );
        // Cell distance 20 is below it: kept.
        let (mut e, graph) = far_distance_engine(20.0);
        assert_eq!(
            e.call(0x0087_86b0, &[graph.addr()]).f32(),
            20.0 * 1000.0 + 3.0 * 10.0 + 5.0
        );
        // At or below the threshold: the first setting.
        let (mut e, graph) = far_distance_engine(10.0);
        assert_eq!(
            e.call(0x0087_86b0, &[graph.addr()]).f32(),
            40.0 * 1000.0 + 3.0 * 10.0 + 5.0
        );
        // NaN: the compares are unordered, so the NaN itself goes on.
        let (mut e, graph) = far_distance_engine(f32::NAN);
        assert!(e.call(0x0087_86b0, &[graph.addr()]).f32().is_nan());
    }
}
