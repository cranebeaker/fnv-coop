//! `NiXenonRenderer/nid3dshaderinterface.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The five set-up calls `InitializeShaderSystem` makes with the renderer (`cdecl`, unnamed in
/// the engine map), the global it keeps the renderer's shader library in (`0126f99c`), the
/// getter `00e91f10` (unnamed, no arguments), the program factory global (`0126fa10`) and its
/// constructor (`00e93ad0`, unnamed; `thiscall`, no stack words), the allocation `00aa13e0`,
/// the report `00a29680` and its message (`"Failed to create shader program factory!"`).
const SET_UP_00E7F920: u32 = 0x00e7_f920;
const SET_UP_00E7E8D0: u32 = 0x00e7_e8d0;
const SET_UP_00E92F70: u32 = 0x00e9_2f70;
const SET_UP_00E93F50: u32 = 0x00e9_3f50;
const SET_UP_00E7EEA0: u32 = 0x00e7_eea0;
const SHADER_LIBRARY_0126F99C: u32 = 0x0126_f99c;
const GETTER_00E91F10: u32 = 0x00e9_1f10;
const PROGRAM_FACTORY_0126FA10: u32 = 0x0126_fa10;
const PROGRAM_FACTORY_CONSTRUCT_00E93AD0: u32 = 0x00e9_3ad0;
const NEW_OBJECT: u32 = 0x00aa_13e0;
const REPORT_00A29680: u32 = 0x00a2_9680;
const FACTORY_FAILED_010EF488: u32 = 0x010e_f488;

// Translated from 00e81940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::InitializeShaderSystem` (engine map), `cdecl`, the renderer; answers
/// 0. Runs `00e7f920`, `00e7e8d0` (then keeps the renderer's shader library, +0x8b8, at
/// `0126f99c`), `00e92f70`, `00e93f50` and `00e7eea0` with the renderer, the library's virtual
/// slot 0x100, and, when `00e91f10` answers an object, its slot 0x70 with the renderer. Without
/// a program factory (`0126fa10`) one is made (0x1c bytes, `00e93ad0`); a null result reports
/// "Failed to create shader program factory!" (a failed allocation also clears the global).
pub fn ni_d3d_shader_interface_initialize_shader_system(e: &mut Engine, renderer: u32) -> u32 {
    let library = e.mem.u32(renderer.wrapping_add(0x8b8));
    e.call(SET_UP_00E7F920, &args![renderer]);
    e.call(SET_UP_00E7E8D0, &args![renderer]);
    e.mem.set_u32(SHADER_LIBRARY_0126F99C, library);
    e.call(SET_UP_00E92F70, &args![renderer]);
    e.call(SET_UP_00E93F50, &args![renderer]);
    e.call(SET_UP_00E7EEA0, &args![renderer]);
    e.vcall(library, 0x100, &args![]);
    let object = e.call(GETTER_00E91F10, &args![]).u32();
    if object != 0 {
        e.vcall(object, 0x70, &args![renderer]);
    }
    if e.mem.u32(PROGRAM_FACTORY_0126FA10) != 0 {
        return 0;
    }
    let block = e.call(NEW_OBJECT, &args![0x1cu32]).u32();
    if block == 0 {
        e.mem.set_u32(PROGRAM_FACTORY_0126FA10, 0);
        e.call(REPORT_00A29680, &args![FACTORY_FAILED_010EF488]);
        return 0;
    }
    let factory = e
        .call(PROGRAM_FACTORY_CONSTRUCT_00E93AD0, &args![block])
        .u32();
    e.mem.set_u32(PROGRAM_FACTORY_0126FA10, factory);
    if factory == 0 {
        e.call(REPORT_00A29680, &args![FACTORY_FAILED_010EF488]);
    }
    0
}

// Translated from 00e818d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::SetD3DRenderer` (engine map), `thiscall` with the renderer: keeps it
/// at +0x18. With a renderer, the device held at +0x14 is released (its slot 8, `stdcall`) and
/// replaced by the renderer's device (+0x288; counted through its slot 4 when not null), and
/// the renderer's shader library (+0x8b8) is kept at +0x1c. Without one +0x1c is cleared, the
/// held device released and +0x14 cleared.
pub fn ni_d3d_shader_interface_set_d3d_renderer(e: &mut Engine, this: Ptr, renderer: u32) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0x18), renderer);
    if renderer != 0 {
        let device = e.mem.u32(renderer.wrapping_add(0x288));
        let held = e.mem.u32(base.wrapping_add(0x14));
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(base.wrapping_add(0x14), device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        let library = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.mem.set_u32(base.wrapping_add(0x1c), library);
        return;
    }
    let held = e.mem.u32(base.wrapping_add(0x14));
    e.mem.set_u32(base.wrapping_add(0x1c), 0);
    if held != 0 {
        e.vcall(held, 8, &args![]);
    }
    e.mem.set_u32(base.wrapping_add(0x14), 0);
}

/// What `00e819f0` runs: the shader factory's clean-up `00aa0000` (no arguments) and the
/// program factory's `00e92ec0` (unnamed, no arguments).
const FACTORY_CLEAN_UP_00AA0000: u32 = 0x00aa_0000;
const PROGRAM_FACTORY_00E92EC0: u32 = 0x00e9_2ec0;

// Translated from 00e819f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderinterface.cpp`): the shader system's shut-down, no
/// arguments, answers 0. When `00e91f10` answers an object: `00aa0000`, then the object's
/// virtual slot 0x70 with 0. Then `00e7f920` and `00e7e8d0` with 0, the shader library global
/// `0126f99c` cleared, `00e92ec0`, and `00e92f70`, `00e93f50`, `00e7eea0` with 0 (the same
/// set-up calls `InitializeShaderSystem` makes with the renderer).
pub fn fn_00e819f0(e: &mut Engine) -> u32 {
    let object = e.call(GETTER_00E91F10, &args![]).u32();
    if object != 0 {
        e.call(FACTORY_CLEAN_UP_00AA0000, &args![]);
        e.vcall(object, 0x70, &args![0u32]);
    }
    e.call(SET_UP_00E7F920, &args![0u32]);
    e.call(SET_UP_00E7E8D0, &args![0u32]);
    e.mem.set_u32(SHADER_LIBRARY_0126F99C, 0);
    e.call(PROGRAM_FACTORY_00E92EC0, &args![]);
    e.call(SET_UP_00E92F70, &args![0u32]);
    e.call(SET_UP_00E93F50, &args![0u32]);
    e.call(SET_UP_00E7EEA0, &args![0u32]);
    0
}

/// The interface's table (`010ef39c`, RTTI `.?AVNiD3DShaderInterface@@`) and the `NiShader`
/// base destructor `00a8f490` (unit `nishader.cpp`; `thiscall`, no stack words).
const SHADER_INTERFACE_VTABLE_010EF39C: u32 = 0x010e_f39c;
const NI_SHADER_DESTRUCT_00A8F490: u32 = 0x00a8_f490;

// Translated from 00e81a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderinterface.cpp`): the `NiD3DShaderInterface`
/// destructor, `thiscall`, no stack words. Stores the table `010ef39c`, clears the renderer
/// (+0x18) and the library (+0x1c), releases the held device (+0x14, slot 8) and clears it,
/// then runs the `NiShader` destructor `00a8f490` (a tail jump).
pub fn fn_00e81a50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let held = e.mem.u32(base.wrapping_add(0x14));
    e.mem.set_u32(base, SHADER_INTERFACE_VTABLE_010EF39C);
    e.mem.set_u32(base.wrapping_add(0x18), 0);
    e.mem.set_u32(base.wrapping_add(0x1c), 0);
    if held != 0 {
        e.vcall(held, 8, &args![]);
    }
    e.mem.set_u32(base.wrapping_add(0x14), 0);
    e.call(NI_SHADER_DESTRUCT_00A8F490, &args![base]);
}

/// The renderer global (`011f4748`) and `NiShader::Initialize` (`00a8f3d0`, engine map;
/// `thiscall`, no stack words).
const RENDERER_011F4748: u32 = 0x011f_4748;
const NI_SHADER_INITIALIZE_00A8F3D0: u32 = 0x00a8_f3d0;

// Translated from 00e81a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::Initialize` (engine map), `thiscall`, no stack words. Without a
/// renderer in `011f4748` answers 0 (the whole of EAX is the null global with AL cleared);
/// otherwise sets it (`SetD3DRenderer`) and answers what `NiShader::Initialize` (`00a8f3d0`,
/// a tail jump) answers.
pub fn ni_d3d_shader_interface_initialize(e: &mut Engine, this: Ptr) -> u32 {
    let renderer = e.mem.u32(RENDERER_011F4748);
    if renderer == 0 {
        return 0;
    }
    ni_d3d_shader_interface_set_d3d_renderer(e, this, renderer);
    e.call(NI_SHADER_INITIALIZE_00A8F3D0, &args![this]).u32()
}

// Translated from 00b79d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::GetIsBestImplementation` (engine map; the vtable slot +0xe4 the Xbox
/// PDB names so), `thiscall`: the byte `m_bIsBestImplementation` (+0x20, Xbox PDB).
pub fn ni_d3d_shader_interface_get_is_best_implementation(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x20))
}

// Translated from 00b79da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::SetIsBestImplementation` (engine map), `thiscall` with one stack word:
/// its low byte goes to `m_bIsBestImplementation` (+0x20, Xbox PDB).
pub fn ni_d3d_shader_interface_set_is_best_implementation(e: &mut Engine, this: Ptr, best: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x20), best);
}

// Translated from 00eea540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::SetD3DRenderState` (engine map), `thiscall` with one stack word:
/// stores it in `m_pkD3DRenderState` (+0x1c, Xbox PDB).
pub fn ni_d3d_shader_interface_set_d3d_render_state(e: &mut Engine, this: Ptr, state: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x1c), state);
}

/// `NiD3DShaderInterface`'s RTTI record (`0126f75c`).
const SHADER_INTERFACE_RTTI_0126F75C: u32 = 0x0126_f75c;

// Translated from 00e818c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::GetRTTI` (engine map), `thiscall`, `this` not read: the RTTI record
/// at `0126f75c`.
pub fn ni_d3d_shader_interface_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    SHADER_INTERFACE_RTTI_0126F75C
}

/// The sized free `00aa1460` (`cdecl`: block, size).
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e81ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::scalar deleting destructor` (engine map), `thiscall` with the flags:
/// the destructor inline (the same steps as [`fn_00e81a50`]), then with bit 0 the 0x24 bytes
/// freed (`00aa1460`). Answers `this`.
pub fn ni_d3d_shader_interface_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e81a50(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x24u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b7_9d90,
            ni_d3d_shader_interface_get_is_best_implementation(Ptr) -> u8
        ),
        entry!(
            0x00b7_9da0,
            ni_d3d_shader_interface_set_is_best_implementation(Ptr, u8)
        ),
        entry!(
            0x00ee_a540,
            ni_d3d_shader_interface_set_d3d_render_state(Ptr, u32)
        ),
        entry!(0x00e8_18c0, ni_d3d_shader_interface_get_rtti(Ptr) -> u32),
        entry!(
            0x00e8_1ab0,
            ni_d3d_shader_interface_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e8_1940,
            ni_d3d_shader_interface_initialize_shader_system(u32) -> u32
        ),
        entry!(
            0x00e8_18d0,
            ni_d3d_shader_interface_set_d3d_renderer(Ptr, u32)
        ),
        entry!(0x00e8_19f0, fn_00e819f0() -> u32),
        entry!(0x00e8_1a50, fn_00e81a50(Ptr)),
        entry!(0x00e8_1a90, ni_d3d_shader_interface_initialize(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(factory: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let mut slots = vec![0u32; 0x41];
        slots[0x40] = 0x6200;
        slots[0x1c] = 0x6300;
        e.put_vtable(0x6000, &slots);
        for no_op in [
            SET_UP_00E7F920,
            SET_UP_00E7E8D0,
            SET_UP_00E92F70,
            SET_UP_00E93F50,
            SET_UP_00E7EEA0,
            REPORT_00A29680,
            0x6200,
            0x6300,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        let library = e.mem.alloc(4);
        e.mem.set_u32(library, 0x6000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x8b8, library);
        e.register_double(GETTER_00E91F10, move |_, _| library.into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(PROGRAM_FACTORY_CONSTRUCT_00E93AD0, move |_, _| {
            factory.into_ret()
        });
        e.call_log = Some(vec![]);
        (e, renderer, library)
    }

    #[test]
    fn initialize_shader_system_sets_up_and_makes_the_factory() {
        let (mut e, renderer, library) = rig(0x7777);
        assert_eq!(e.call(0x00e8_1940, &args![renderer]).u32(), 0);
        assert_eq!(e.mem.u32(SHADER_LIBRARY_0126F99C), library);
        assert_eq!(calls(&e, 0x6200), vec![vec![library]]);
        assert_eq!(calls(&e, 0x6300), vec![vec![library, renderer]]);
        assert_eq!(e.mem.u32(PROGRAM_FACTORY_0126FA10), 0x7777);
        assert!(calls(&e, REPORT_00A29680).is_empty());
        // A factory already there: nothing made.
        e.call_log = Some(vec![]);
        e.call(0x00e8_1940, &args![renderer]);
        assert!(calls(&e, NEW_OBJECT).is_empty());
    }

    #[test]
    fn initialize_shader_system_reports_a_failed_factory() {
        let (mut e, renderer, _) = rig(0);
        e.call(0x00e8_1940, &args![renderer]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![FACTORY_FAILED_010EF488]]
        );
        e.register(NEW_OBJECT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e8_1940, &args![renderer]);
        assert!(calls(&e, PROGRAM_FACTORY_CONSTRUCT_00E93AD0).is_empty());
        assert_eq!(calls(&e, REPORT_00A29680).len(), 1);
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const ADD_REF: u32 = 0x0e20_0004;
    const RELEASE: u32 = 0x0e20_0008;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with a counted device (`AddRef`/`Release` doubles) and a renderer holding it.
    fn rig() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(ADD_REF, |_, _| Ret::default());
        e.register(RELEASE, |_, _| Ret::default());
        e.put_vtable(0x0e21_0000, &[0, ADD_REF, RELEASE]);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x0e21_0000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x4444);
        (e, renderer, device)
    }

    #[test]
    fn set_d3d_renderer_swaps_the_device_and_keeps_the_library() {
        let (mut e, renderer, device) = rig();
        let this = e.mem.alloc(0x24);
        let old = e.mem.alloc(4);
        e.mem.set_u32(old, 0x0e21_0000);
        e.mem.set_u32(this + 0x14, old);
        e.call_log = Some(vec![]);
        e.call(0x00e8_18d0, &args![this, renderer]);
        assert_eq!(calls(&e, RELEASE), vec![vec![old]]);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
        assert_eq!(e.mem.u32(this + 0x14), device);
        assert_eq!(e.mem.u32(this + 0x18), renderer);
        assert_eq!(e.mem.u32(this + 0x1c), 0x4444);
        // No renderer: everything cleared and the device released.
        e.call_log = Some(vec![]);
        e.call(0x00e8_18d0, &args![this, 0u32]);
        assert_eq!(calls(&e, RELEASE), vec![vec![device]]);
        assert_eq!(e.mem.bytes(this + 0x14, 0xc), vec![0; 0xc]);
    }

    #[test]
    fn fn_00e819f0_shuts_the_system_down() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(SHADER_LIBRARY_0126F99C, 9);
        let mut slots = vec![0u32; 0x1d];
        slots[0x1c] = 0x0e22_0070;
        e.put_vtable(0x0e23_0000, &slots);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0e23_0000);
        for no_op in [
            FACTORY_CLEAN_UP_00AA0000,
            0x0e22_0070,
            SET_UP_00E7F920,
            SET_UP_00E7E8D0,
            PROGRAM_FACTORY_00E92EC0,
            SET_UP_00E92F70,
            SET_UP_00E93F50,
            SET_UP_00E7EEA0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register_double(GETTER_00E91F10, move |_, _| object.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_19f0, &args![]).u32(), 0);
        let order: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.0)
            .collect();
        assert_eq!(
            order,
            vec![
                GETTER_00E91F10,
                FACTORY_CLEAN_UP_00AA0000,
                0x0e22_0070,
                SET_UP_00E7F920,
                SET_UP_00E7E8D0,
                PROGRAM_FACTORY_00E92EC0,
                SET_UP_00E92F70,
                SET_UP_00E93F50,
                SET_UP_00E7EEA0
            ]
        );
        assert_eq!(e.mem.u32(SHADER_LIBRARY_0126F99C), 0);
        // Without an object neither the clean-up nor the slot runs.
        e.register(GETTER_00E91F10, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e8_19f0, &args![]);
        assert!(calls(&e, FACTORY_CLEAN_UP_00AA0000).is_empty());
    }

    #[test]
    fn fn_00e81a50_releases_and_runs_the_base_destructor() {
        let (mut e, _, device) = rig();
        e.register(NI_SHADER_DESTRUCT_00A8F490, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x14, device);
        e.mem.set_u32(this + 0x18, 1);
        e.mem.set_u32(this + 0x1c, 2);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1a50, &args![this]);
        assert_eq!(calls(&e, RELEASE), vec![vec![device]]);
        assert_eq!(calls(&e, NI_SHADER_DESTRUCT_00A8F490), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), SHADER_INTERFACE_VTABLE_010EF39C);
        assert_eq!(e.mem.bytes(this + 0x14, 0xc), vec![0; 0xc]);
    }

    #[test]
    fn initialize_needs_the_renderer() {
        let (mut e, renderer, device) = rig();
        e.map(0x011f_4000, 0x1000);
        e.register(NI_SHADER_INITIALIZE_00A8F3D0, |_, _| 0x101u32.into_ret());
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_1a90, &args![this]).u32(), 0);
        assert!(calls(&e, NI_SHADER_INITIALIZE_00A8F3D0).is_empty());
        e.mem.set_u32(RENDERER_011F4748, renderer);
        assert_eq!(e.call(0x00e8_1a90, &args![this]).u32(), 0x101);
        assert_eq!(e.mem.u32(this + 0x14), device);
        assert_eq!(calls(&e, NI_SHADER_INITIALIZE_00A8F3D0), vec![vec![this]]);
    }
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    #[test]
    fn set_is_best_implementation_stores_the_low_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.call(0x00b7_9da0, &args![this, 0xffff_ff01u32]);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(e.mem.u8(this + 0x21), 0);
    }

    #[test]
    fn get_is_best_implementation_reads_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.mem.set_u8(this + 0x20, 0x5a);
        assert_eq!(e.call(0x00b7_9d90, &args![this]).u8(), 0x5a);
    }

    #[test]
    fn set_d3d_render_state_stores_the_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.call(0x00ee_a540, &args![this, 0x0e00_1234u32]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0e00_1234);
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e8_18c0, &args![0u32]).u32(), 0x0126_f75c);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_zero() {
        for (flags, frees) in [(1u32, true), (2, false)] {
            let mut e = Engine::new();
            e.put_vtable(0x0e00_0000, &[0, 0, 0x0e01_0008]);
            e.register(0x0e01_0008, |_, _| Ret::default());
            e.register(NI_SHADER_DESTRUCT_00A8F490, |_, _| Ret::default());
            e.register(FREE_SIZED_00AA1460, |_, _| Ret::default());
            let device = e.mem.alloc(4);
            e.mem.set_u32(device, 0x0e00_0000);
            let this = e.mem.alloc(0x24);
            e.mem.set_u32(this + 0x14, device);
            e.mem.set_u32(this + 0x18, 0x55);
            e.mem.set_u32(this + 0x1c, 0x66);
            e.call_log = Some(vec![]);
            let result = e.call(0x00e8_1ab0, &args![this, flags]).u32();
            assert_eq!(result, this);
            assert_eq!(e.mem.u32(this), SHADER_INTERFACE_VTABLE_010EF39C);
            for offset in [0x14u32, 0x18, 0x1c] {
                assert_eq!(e.mem.u32(this + offset), 0);
            }
            let log = e.call_log.take().unwrap();
            let picked = |addr: u32| -> Vec<Vec<u32>> {
                log.iter()
                    .filter(|c| c.0 == addr)
                    .map(|c| c.1.clone())
                    .collect()
            };
            assert_eq!(picked(0x0e01_0008), vec![vec![device]]);
            assert_eq!(picked(NI_SHADER_DESTRUCT_00A8F490), vec![vec![this]]);
            let freed = if frees {
                vec![vec![this, 0x24]]
            } else {
                vec![]
            };
            assert_eq!(picked(FREE_SIZED_00AA1460), freed);
        }
    }
}
