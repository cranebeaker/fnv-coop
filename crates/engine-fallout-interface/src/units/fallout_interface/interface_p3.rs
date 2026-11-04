//! `fallout/interface/interface.cpp` (Xbox PDB source unit), part 3: its functions from `00709b50` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::interface`]; anything public there may be used here.
//!
//! These are the last functions of the unit: setters and getters on the
//! interface manager's PC-only fields at `+0x4ac..+0x4bc` (not in the Xbox
//! PDB, whose manager ends at `0x478`), thin static wrappers over HUD and
//! map menu functions, the camera frustum setup of `00709d50`, and the
//! constructor and destructors of a `BSSimpleArray<Tile::Value *>`.

#[allow(unused_imports)]
use super::interface::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `004b7210`: the interface manager object.
const MANAGER: u32 = 0x004b_7210;
/// `004b71d0`: whether the interface manager is ready (a `bool`).
const MANAGER_IS_READY: u32 = 0x004b_71d0;
/// `HUDMainMenu` functions (`hudmainmenu.cpp`) that the static wrappers
/// forward to.
const HUD_FN_0077F2F0: u32 = 0x0077_f2f0;
/// `HUDMainMenu::SetGunScopeVisible` (Xbox PDB).
const HUD_SET_GUN_SCOPE_VISIBLE: u32 = 0x0077_f3c0;
const HUD_FN_0077F420: u32 = 0x0077_f420;
const HUD_FN_0077F490: u32 = 0x0077_f490;
const HUD_FN_0077F270: u32 = 0x0077_f270;
/// `MapMenu::SetRootMapSpace` (Xbox PDB).
const MAP_MENU_SET_ROOT_MAP_SPACE: u32 = 0x007a_1880;

/// A pointer global to an object whose byte at `+0x1fc` [`fn_00709cc0`]
/// reads (the callee `0077f270` clears the same byte, in the HUD main
/// menu).
const HUD_MAIN_MENU: u32 = 0x011d_96c0;
/// A byte global returned by [`fn_00709d10`].
const FLAG_BYTE: u32 = 0x011d_b0ce;

/// Manager fields (PC only, not in the Xbox PDB).
const MANAGER_FLOAT_A: u32 = 0x4ac;
const MANAGER_FLOAT_B: u32 = 0x4b0;
const MANAGER_BYTE: u32 = 0x4b4;
const MANAGER_WORD: u32 = 0x4b8;
/// The manager's mode word: 0, 2 and 3 are tested by the getters below.
const MANAGER_MODE: u32 = 0x4bc;

/// Two `double` factors and the upper limit (`double`) of the half angle
/// computed by [`fn_00709d50`], and the far plane (`float`).
const ANGLE_FACTOR_A: u32 = 0x0102_3128;
const ANGLE_FACTOR_B: u32 = 0x0102_90f8;
const ANGLE_LIMIT: u32 = 0x0106_ede0;
const FAR_PLANE: u32 = 0x0103_0020;

/// `00709ac0`: returns a `float` in ST0 (a scale used by the camera setup).
const CAMERA_SCALE: u32 = 0x0070_9ac0;
/// `004de070`: a renderer flag byte.
const RENDERER_FLAG: u32 = 0x004d_e070;
/// `004de0c0` / `004de0d0`: two renderer `float`s in ST0 (their quotient is
/// the aspect ratio).
const RENDERER_WIDTH: u32 = 0x004d_e0c0;
const RENDERER_HEIGHT: u32 = 0x004d_e0d0;
/// `00403e20` on the object at `01203150`: returns a pointer to a `float`.
const FLOAT_SETTING_OBJECT: u32 = 0x0120_3150;
const FLOAT_SETTING_GET: u32 = 0x0040_3e20;
/// `NiCamera::SetViewFrustum` (Xbox PDB), taking a `const NiFrustum*`.
const NI_CAMERA_SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// `NiFrustum::NiFrustum(bool ortho)` (Xbox PDB).
const NI_FRUSTUM_NEW: u32 = 0x00a7_1b70;
/// `00a6fb90` (`nicamera.cpp`), thiscall on the camera with a `float` and a
/// zero word.
const NI_CAMERA_FN_00A6FB90: u32 = 0x00a6_fb90;
/// `0057dd70`: one `float` in, a `float` in ST0 out (the frustum's tangent).
const TANGENT: u32 = 0x0057_dd70;

/// The vtable of the `BSSimpleArray<Tile::Value *>` of `00709e80` (RTTI
/// `.?AV?$BSSimpleArray@PAUValue@Tile@@$07@@`).
const SIMPLE_ARRAY_VTABLE: u32 = 0x0106_edec;
/// The base initialization the constructor calls, the storage release the
/// destructor calls, and the cdecl deallocation of the deleting destructor.
const ARRAY_INIT: u32 = 0x006b_3eb0;
const ARRAY_RELEASE: u32 = 0x0084_54f0;
const FREE: u32 = 0x0040_1030;

/// The manager object.
fn manager(e: &mut Engine) -> u32 {
    e.call(MANAGER, &[]).u32()
}

// Translated from 00709b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper (cdecl): [`fn_00709b80`] on the interface manager.
pub fn fn_00709b50(e: &mut Engine, flag: u8, first: f32, second: f32, word: u32) {
    let this = manager(e);
    fn_00709b80(e, this, flag, first, second, word);
}

// Translated from 00709b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the four arguments in the manager's PC-only fields `+0x4b4`
/// (byte), `+0x4ac`, `+0x4b0` (floats) and `+0x4b8` (word).
pub fn fn_00709b80(e: &mut Engine, this: u32, flag: u8, first: f32, second: f32, word: u32) {
    e.mem.set_u8(this + MANAGER_BYTE, flag);
    e.mem.set_f32(this + MANAGER_FLOAT_A, first);
    e.mem.set_f32(this + MANAGER_FLOAT_B, second);
    e.mem.set_u32(this + MANAGER_WORD, word);
}

// Translated from 00709bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsPipboyOpening` (Xbox PDB): the manager's mode word is 2.
pub fn interface_is_pipboy_opening(e: &mut Engine) -> bool {
    let this = manager(e);
    e.mem.u32(this + MANAGER_MODE) == 2
}

// Translated from 00709be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's mode word is 0.
pub fn fn_00709be0(e: &mut Engine) -> bool {
    let this = manager(e);
    e.mem.u32(this + MANAGER_MODE) == 0
}

// Translated from 00709c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's mode word is 3.
pub fn fn_00709c00(e: &mut Engine) -> bool {
    let this = manager(e);
    e.mem.u32(this + MANAGER_MODE) == 3
}

// Translated from 00709c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper (cdecl): forwards its argument to `0077f2f0`.
pub fn fn_00709c20(e: &mut Engine, argument: u32) {
    e.call(HUD_FN_0077F2F0, &args![argument]);
}

// Translated from 00709c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetGunScopeVisible` (Xbox PDB): forwards the flag (as a
/// zero-extended word) to `HUDMainMenu::SetGunScopeVisible`.
pub fn interface_set_gun_scope_visible(e: &mut Engine, visible: u8) {
    e.call(HUD_SET_GUN_SCOPE_VISIBLE, &args![visible]);
}

// Translated from 00709c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper (cdecl): forwards its byte argument to `0077f420`.
pub fn fn_00709c60(e: &mut Engine, flag: u8) {
    e.call(HUD_FN_0077F420, &args![flag]);
}

// Translated from 00709c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper (cdecl): forwards its byte argument to `0077f490`.
pub fn fn_00709c80(e: &mut Engine, flag: u8) {
    e.call(HUD_FN_0077F490, &args![flag]);
}

// Translated from 00709ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: calls `0077f270`.
pub fn fn_00709ca0(e: &mut Engine) {
    e.call(HUD_FN_0077F270, &[]);
}

// Translated from 00709cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: [`fn_00709cc0`].
pub fn fn_00709cb0(e: &mut Engine) -> bool {
    fn_00709cc0(e)
}

// Translated from 00709cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object at `011d96c0` exists and its byte at `+0x1fc` is set.
pub fn fn_00709cc0(e: &mut Engine) -> bool {
    let menu = e.global::<u32>(HUD_MAIN_MENU);
    menu != 0 && e.mem.u8(menu + 0x1fc) != 0
}

// Translated from 00709d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: [`fn_00709d10`].
pub fn fn_00709d00(e: &mut Engine) -> u8 {
    fn_00709d10(e)
}

// Translated from 00709d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte global at `011db0ce`.
pub fn fn_00709d10(e: &mut Engine) -> u8 {
    e.global::<u8>(FLAG_BYTE)
}

// Translated from 00709d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetRootMapSpace` (Xbox PDB): forwards its argument to
/// `MapMenu::SetRootMapSpace`.
pub fn interface_set_root_map_space(e: &mut Engine, space: u32) {
    e.call(MAP_MENU_SET_ROOT_MAP_SPACE, &args![space]);
}

// Translated from 00709d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: the result of `004b71d0` (whether the manager is ready).
pub fn fn_00709d40(e: &mut Engine) -> bool {
    e.call(MANAGER_IS_READY, &[]).bool()
}

// Translated from 00709d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up a camera's view frustum. Does nothing when `camera` is null or
/// the renderer flag byte (`004de070`) is zero.
///
/// With `s = 00709ac0()`, `00a6fb90(camera, s, 0)` runs first. The aspect
/// ratio is `004de0c0() / 004de0d0()`. The half angle is `scale * (the
/// float at 00403e20 on 01203150) * k1 * k2` (`k1`, `k2` the doubles at
/// `01023128` and `010290f8`), replaced by the double at `0106ede0` unless
/// it is smaller (so also when it is NaN). With `t = 0057dd70(angle)` the
/// frustum is `left = -t*s*aspect`, `right = t*s*aspect`, `top = t*s`,
/// `bottom = -t*s`, `near = 1.0`, `far` = the float at `01030020`,
/// perspective; `NiCamera::SetViewFrustum` receives it.
pub fn fn_00709d50(e: &mut Engine, camera: u32, scale: f32) {
    if camera == 0 {
        return;
    }
    if e.call(RENDERER_FLAG, &[]).u8() == 0 {
        return;
    }
    let unit = e.call(CAMERA_SCALE, &[]).f32();
    e.call(NI_CAMERA_FN_00A6FB90, &args![camera, unit, 0u32]);
    let aspect = (e.call(RENDERER_WIDTH, &[]).f64() / e.call(RENDERER_HEIGHT, &[]).f64()) as f32;
    let factor_a = e.global::<f64>(ANGLE_FACTOR_A);
    let factor_b = e.global::<f64>(ANGLE_FACTOR_B);
    let limit = e.global::<f64>(ANGLE_LIMIT);
    let setting = e
        .call(FLOAT_SETTING_GET, &args![FLOAT_SETTING_OBJECT])
        .u32();
    let mut angle = scale as f64 * e.mem.f32(setting) as f64 * factor_a * factor_b;
    if angle < limit {
        let setting = e
            .call(FLOAT_SETTING_GET, &args![FLOAT_SETTING_OBJECT])
            .u32();
        angle = scale as f64 * e.mem.f32(setting) as f64 * factor_a * factor_b;
    } else {
        angle = limit;
    }
    let angle = angle as f32;
    let (unit, aspect) = (unit as f64, aspect as f64);
    e.with_stack(0x1c, |e, frustum| {
        let f = frustum.addr();
        e.call(NI_FRUSTUM_NEW, &args![f, 0u32]);
        // NiFrustum: +0 left, +4 right, +8 top, +0xc bottom, +0x10 near,
        // +0x14 far, +0x18 ortho (byte).
        let tangent = e.call(TANGENT, &args![angle]).f32() as f64;
        e.mem.set_f32(f, (-tangent * unit * aspect) as f32);
        let tangent = e.call(TANGENT, &args![angle]).f32() as f64;
        e.mem.set_f32(f + 4, (tangent * unit * aspect) as f32);
        let tangent = e.call(TANGENT, &args![angle]).f32() as f64;
        e.mem.set_f32(f + 0xc, (-tangent * unit) as f32);
        let tangent = e.call(TANGENT, &args![angle]).f32() as f64;
        e.mem.set_f32(f + 8, (tangent * unit) as f32);
        e.mem.set_u8(f + 0x18, 0);
        e.mem.set_f32(f + 0x10, 1.0);
        let far = e.global::<f32>(FAR_PLANE);
        e.mem.set_f32(f + 0x14, far);
        e.call(NI_CAMERA_SET_VIEW_FRUSTUM, &args![camera, f]);
    });
}

// Translated from 00709e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `BSSimpleArray<Tile::Value *>`: sets the vtable, then
/// runs the base initialization `006b3eb0(this, a, b)`. Returns `this`.
pub fn fn_00709e80(e: &mut Engine, this: u32, size: u32, count: u32) -> u32 {
    e.mem.set_u32(this, SIMPLE_ARRAY_VTABLE);
    e.call(ARRAY_INIT, &args![this, size, count]);
    this
}

// Translated from 00709eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the same array: sets the vtable, then
/// `008454f0(this, 1)` (releases the storage).
pub fn fn_00709eb0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, SIMPLE_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE, &args![this, 1u32]);
}

// Translated from 00709ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<Tile::Value_P_8>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs [`fn_00709eb0`], then frees the object when bit 0 of `flags`
/// is set. Returns `this`.
pub fn bs_simple_array_tile_value_p_8_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    fn_00709eb0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00709b50, fn_00709b50(u8, f32, f32, u32)),
        entry!(0x00709b80, fn_00709b80(u32, u8, f32, f32, u32)),
        entry!(0x00709bc0, interface_is_pipboy_opening() -> bool),
        entry!(0x00709be0, fn_00709be0() -> bool),
        entry!(0x00709c00, fn_00709c00() -> bool),
        entry!(0x00709c20, fn_00709c20(u32)),
        entry!(0x00709c40, interface_set_gun_scope_visible(u8)),
        entry!(0x00709c60, fn_00709c60(u8)),
        entry!(0x00709c80, fn_00709c80(u8)),
        entry!(0x00709ca0, fn_00709ca0()),
        entry!(0x00709cb0, fn_00709cb0() -> bool),
        entry!(0x00709cc0, fn_00709cc0() -> bool),
        entry!(0x00709d00, fn_00709d00() -> u8),
        entry!(0x00709d10, fn_00709d10() -> u8),
        entry!(0x00709d20, interface_set_root_map_space(u32)),
        entry!(0x00709d40, fn_00709d40() -> bool),
        entry!(0x00709d50, fn_00709d50(u32, f32)),
        entry!(0x00709e80, fn_00709e80(u32, u32, u32) -> u32),
        entry!(0x00709eb0, fn_00709eb0(u32)),
        entry!(
            0x00709ed0,
            bs_simple_array_tile_value_p_8_scalar_deleting_destructor(u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A call-logging engine whose manager is a fresh 0x500-byte object.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x0102_3000,
            0x0102_9000,
            0x0103_0000,
            0x0106_e000,
            0x011d_9000,
            0x011d_b000,
        ] {
            e.map(page, 0x1000);
        }
        let manager = e.mem.alloc(0x500);
        e.register(MANAGER, |e, _| Ret {
            eax: e.mem.u32(0x011d_9000),
            ..Ret::default()
        });
        e.mem.set_u32(0x011d_9000, manager);
        e.call_log = Some(vec![]);
        (e, manager)
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn manager_setter_stores_fields() {
        let (mut e, manager) = engine();
        e.call(
            0x00709b80,
            &args![manager, 0x1ffu32, 1.5f32, -2.0f32, 0xdead_beefu32],
        );
        assert_eq!(e.mem.u8(manager + 0x4b4), 0xff);
        assert_eq!(e.mem.f32(manager + 0x4ac), 1.5);
        assert_eq!(e.mem.f32(manager + 0x4b0), -2.0);
        assert_eq!(e.mem.u32(manager + 0x4b8), 0xdead_beef);
    }

    #[test]
    fn manager_setter_wrapper_uses_manager() {
        let (mut e, manager) = engine();
        e.call(0x00709b50, &args![1u32, 3.0f32, 4.0f32, 9u32]);
        assert_eq!(e.mem.u8(manager + 0x4b4), 1);
        assert_eq!(e.mem.f32(manager + 0x4ac), 3.0);
        assert_eq!(e.mem.f32(manager + 0x4b0), 4.0);
        assert_eq!(e.mem.u32(manager + 0x4b8), 9);
    }

    #[test]
    fn mode_getters() {
        let (mut e, manager) = engine();
        for (mode, zero, two, three) in [
            (0, true, false, false),
            (2, false, true, false),
            (3, false, false, true),
            (1, false, false, false),
        ] {
            e.mem.set_u32(manager + 0x4bc, mode);
            assert_eq!(e.call(0x00709be0, &[]).bool(), zero);
            assert_eq!(e.call(0x00709bc0, &[]).bool(), two);
            assert_eq!(e.call(0x00709c00, &[]).bool(), three);
        }
    }

    /// Checks a wrapper that forwards `words` to `callee`.
    fn forwards(wrapper: u32, callee: u32, words: Vec<u32>, expected: Vec<u32>) {
        let (mut e, _) = engine();
        e.register(callee, |_, _| Ret::default());
        e.call(wrapper, &words);
        assert_eq!(logged(&e, callee), vec![expected]);
    }

    #[test]
    fn hud_wrappers_forward() {
        forwards(0x00709c20, 0x0077f2f0, vec![0x1234], vec![0x1234]);
        forwards(0x00709c40, 0x0077f3c0, vec![0x1ff], vec![0xff]);
        forwards(0x00709c60, 0x0077f420, vec![0x101], vec![1]);
        forwards(0x00709c80, 0x0077f490, vec![0x100], vec![0]);
        forwards(0x00709d20, 0x007a1880, vec![77], vec![77]);
    }

    #[test]
    fn no_argument_wrapper_calls_callee() {
        let (mut e, _) = engine();
        e.register(0x0077f270, |_, _| Ret::default());
        e.call(0x00709ca0, &[]);
        assert_eq!(logged(&e, 0x0077f270), vec![Vec::<u32>::new()]);
    }

    #[test]
    fn menu_flag_needs_object_and_byte() {
        let (mut e, _) = engine();
        assert!(!e.call(0x00709cc0, &[]).bool());
        assert!(!e.call(0x00709cb0, &[]).bool());
        let menu = e.mem.alloc(0x300);
        e.mem.set_u32(0x011d_96c0, menu);
        assert!(!e.call(0x00709cc0, &[]).bool());
        e.mem.set_u8(menu + 0x1fc, 5);
        assert!(e.call(0x00709cc0, &[]).bool());
        assert!(e.call(0x00709cb0, &[]).bool());
    }

    #[test]
    fn byte_global_getters() {
        let (mut e, _) = engine();
        e.mem.set_u8(0x011d_b0ce, 0x42);
        assert_eq!(e.call(0x00709d10, &[]).u8(), 0x42);
        assert_eq!(e.call(0x00709d00, &[]).u8(), 0x42);
    }

    #[test]
    fn ready_wrapper_returns_callee_byte() {
        let (mut e, _) = engine();
        e.register(0x004b71d0, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        assert!(!e.call(0x00709d40, &[]).bool());
        e.register(0x004b71d0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(e.call(0x00709d40, &[]).bool());
    }

    /// An engine for the camera setup: doubles for every callee, returning
    /// `unit = 2`, width 800, height 400, a setting of 0.5, `tan(x) = x`.
    fn camera_engine(flag: bool, limit: f64) -> Engine {
        let (mut e, _) = engine();
        e.register(RENDERER_FLAG, |e, _| Ret {
            eax: e.mem.u8(0x011d_b000) as u32,
            ..Ret::default()
        });
        e.mem.set_u8(0x011d_b000, flag as u8);
        e.register(CAMERA_SCALE, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(RENDERER_WIDTH, |_, _| Ret {
            st0: 800.0,
            ..Ret::default()
        });
        e.register(RENDERER_HEIGHT, |_, _| Ret {
            st0: 400.0,
            ..Ret::default()
        });
        e.register(FLOAT_SETTING_GET, |e, _| {
            let p = e.mem.alloc(4);
            e.mem.set_f32(p, 0.5);
            Ret {
                eax: p,
                ..Ret::default()
            }
        });
        e.register(TANGENT, |_, a| Ret {
            st0: f32::from_bits(a[0]) as f64,
            ..Ret::default()
        });
        e.register(NI_CAMERA_FN_00A6FB90, |_, _| Ret::default());
        e.register(NI_FRUSTUM_NEW, |_, _| Ret::default());
        e.mem.set_u64(ANGLE_FACTOR_A, 1.0f64.to_bits());
        e.mem.set_u64(ANGLE_FACTOR_B, 1.0f64.to_bits());
        e.mem.set_u64(ANGLE_LIMIT, limit.to_bits());
        e.mem.set_f32(FAR_PLANE, 4000.0);
        e
    }

    #[test]
    fn camera_does_nothing_without_camera_or_flag() {
        let mut e = camera_engine(true, 10.0);
        e.call(0x00709d50, &args![0u32, 1.0f32]);
        assert!(logged(&e, NI_CAMERA_FN_00A6FB90).is_empty());
        let mut e = camera_engine(false, 10.0);
        e.call(0x00709d50, &args![0x1000u32, 1.0f32]);
        assert!(logged(&e, NI_CAMERA_FN_00A6FB90).is_empty());
    }

    /// Runs the setup and returns the frustum the camera received (left,
    /// right, top, bottom, near, far, ortho byte).
    fn frustum_of(limit: f64, scale: f32) -> ([f32; 6], u8) {
        let mut e = camera_engine(true, limit);
        e.register_double(NI_CAMERA_SET_VIEW_FRUSTUM, |e, a| {
            // Park a copy of the frustum where the test can read it.
            for i in 0..7u32 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(0x011d_b100 + 4 * i, word);
            }
            Ret::default()
        });
        e.call(0x00709d50, &args![0x1000u32, scale]);
        assert_eq!(
            logged(&e, NI_CAMERA_FN_00A6FB90),
            vec![vec![0x1000, 2.0f32.to_bits(), 0]]
        );
        assert_eq!(logged(&e, NI_FRUSTUM_NEW).len(), 1);
        let mut out = [0f32; 6];
        for (i, slot) in out.iter_mut().enumerate() {
            *slot = e.mem.f32(0x011d_b100 + 4 * i as u32);
        }
        (out, e.mem.u8(0x011d_b100 + 0x18))
    }

    #[test]
    fn camera_frustum_below_limit() {
        // angle = 1.0 * 0.5 (< 10) and tangent(x) = x; unit 2, aspect 2.
        let (f, ortho) = frustum_of(10.0, 1.0);
        assert_eq!(f, [-2.0, 2.0, 1.0, -1.0, 1.0, 4000.0]);
        assert_eq!(ortho, 0);
    }

    #[test]
    fn camera_frustum_angle_is_capped() {
        // angle = 4.0 * 0.5 = 2.0 is not below the limit 0.25, which is used.
        let (f, _) = frustum_of(0.25, 4.0);
        assert_eq!(f, [-1.0, 1.0, 0.5, -0.5, 1.0, 4000.0]);
    }

    #[test]
    fn array_constructor_and_destructors() {
        let (mut e, _) = engine();
        let array = e.mem.alloc(0x10);
        e.register(0x006b3eb0, |_, _| Ret::default());
        e.register(0x008454f0, |_, _| Ret::default());
        e.register(0x00401030, |_, _| Ret::default());
        assert_eq!(e.call(0x00709e80, &args![array, 5u32, 6u32]).u32(), array);
        assert_eq!(e.mem.u32(array), 0x0106_edec);
        assert_eq!(logged(&e, 0x006b3eb0), vec![vec![array, 5, 6]]);

        e.mem.set_u32(array, 0);
        e.call(0x00709eb0, &args![array]);
        assert_eq!(e.mem.u32(array), 0x0106_edec);
        assert_eq!(logged(&e, 0x008454f0), vec![vec![array, 1]]);

        assert_eq!(e.call(0x00709ed0, &args![array, 0u32]).u32(), array);
        assert!(logged(&e, 0x00401030).is_empty());
        assert_eq!(e.call(0x00709ed0, &args![array, 3u32]).u32(), array);
        assert_eq!(logged(&e, 0x00401030), vec![vec![array]]);
    }
}
