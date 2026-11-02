//! `NiXenonRenderer/nixenonrenderer.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e7_3eb0,
            fn_00e73eb0(Ptr<NiDX9Renderer>, u32, u32) -> u32
        ),
        entry!(0x00e7_a9a0, ni_xenon_renderer_get_d3d_format_string(i32) -> u32),
        entry!(0x00e6_9e50, fn_00e69e50(u32) -> u32),
        entry!(0x00e6_c550, fn_00e6c550(Ptr) -> u8),
        entry!(0x00e6_9460, ni_xenon_renderer_create_direct3d9(u8) -> i32),
        (0x00e7_2e60, fn_00e72e60_entry as AbiFn),
        entry!(0x00e6_9510, fn_00e69510(u32, Ptr, Ptr) -> u8),
        entry!(0x00e6_9c90, fn_00e69c90(u32) -> u32),
        entry!(0x00e6_9d00, fn_00e69d00(u32) -> u32),
        entry!(0x00e6_e6e0, fn_00e6e6e0(Ptr, u32)),
        entry!(0x00e7_2cf0, fn_00e72cf0(Ptr, u32, u32)),
        entry!(0x00e6_b990, fn_00e6b990(Ptr)),
        (0x00e6_bf50, fn_00e6bf50_entry as AbiFn),
        entry!(0x00e6_c240, fn_00e6c240(Ptr, u32) -> u8),
        entry!(0x00e6_ec90, fn_00e6ec90(Ptr) -> u8),
        entry!(0x00e7_45a0, fn_00e745a0(Ptr<NiDX9Renderer>, Ptr)),
        entry!(
            0x00e6_eea0,
            fn_00e6eea0(Ptr<NiDX9Renderer>, u32, u32) -> bool
        ),
        entry!(0x00e6_f310, fn_00e6f310(Ptr<NiDX9Renderer>, u32, u32)),
        entry!(0x00e7_4840, fn_00e74840(Ptr<NiDX9Renderer>, Ptr)),
        entry!(
            0x00e7_3f60,
            ni_xenon_renderer_precache_geometry(Ptr<NiDX9Renderer>, u32, u32, u32, u32) -> bool
        ),
        entry!(0x00a7_65c0, ni_pixel_format_operator_not_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_50f0, ni_vertex_color_property_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00b7_99a0, ni_d3d_utility_get_d3d_from_ni(u32, Ptr) -> u32),
        entry!(0x00e6_9410, fn_00e69410() -> bool),
        entry!(0x00e6_95e0, fn_00e695e0(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_95f0, fn_00e695f0(Ptr<NiDX9Renderer>) -> u8),
        entry!(0x00e6_9600, fn_00e69600(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_9620, fn_00e69620(Ptr<NiDX9Renderer>) -> u32),
        entry!(
            0x00e6_9640,
            fn_00e69640(Ptr<NiDX9Renderer>, u32, u32, u32) -> u32
        ),
        entry!(0x00e6_9660, fn_00e69660(Ptr<NiDX9Renderer>) -> bool),
        entry!(0x00e6_96a0, fn_00e696a0(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_9790, fn_00e69790(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_9960, fn_00e69960(Ptr<NiDX9Renderer>, u32) -> u32),
        entry!(
            0x00e6_9970,
            ni_xenon_renderer_purge_vertex_shader(Ptr<NiDX9Renderer>, u32) -> u32
        ),
        entry!(
            0x00e6_99c0,
            ni_xenon_renderer_purge_pixel_shader(Ptr<NiDX9Renderer>, u32) -> u32
        ),
        entry!(
            0x00e6_9a20,
            fn_00e69a20(Ptr<NiDX9Renderer>, u32, u32, u32) -> bool
        ),
        entry!(0x00e6_9a60, fn_00e69a60() -> u32),
        entry!(0x00e6_9b00, fn_00e69b00(Ptr, u32) -> u32),
        entry!(0x00e6_9b40, fn_00e69b40(Ptr, u32) -> u32),
        entry!(0x00e6_d740, fn_00e6d740(Ptr, Ptr)),
        entry!(0x00e6_dc40, fn_00e6dc40(Ptr<NiDX9Renderer>, Ptr) -> bool),
        entry!(
            0x00e6_dc80,
            fn_00e6dc80(Ptr<NiDX9Renderer>, Ptr, u32) -> bool
        ),
        entry!(0x00e6_dcc0, fn_00e6dcc0(Ptr<NiDX9Renderer>, Ptr) -> bool),
        entry!(0x00e6_df80, fn_00e6df80(Ptr<NiDX9Renderer>, Ptr) -> u32),
        entry!(
            0x00e6_ee00,
            fn_00e6ee00(Ptr<NiDX9Renderer>, u32, u32, u32) -> u32
        ),
        entry!(0x00e6_fa90, fn_00e6fa90(Ptr, Ptr, Ptr, u32)),
        entry!(0x00e7_1cc0, fn_00e71cc0(Ptr<NiDX9Renderer>, u32)),
        entry!(0x00e7_1cf0, fn_00e71cf0(Ptr<NiDX9Renderer>, u32)),
        entry!(
            0x00e7_2c20,
            fn_00e72c20(Ptr<NiDX9Renderer>, Ptr, u32) -> bool
        ),
        entry!(0x00e7_2e50, fn_00e72e50(Ptr) -> u32),
        entry!(0x00e7_5310, fn_00e75310(Ptr<NiDX9Renderer>, u32)),
        entry!(0x00e7_5340, fn_00e75340(Ptr<NiDX9Renderer>, u32)),
        entry!(
            0x00e7_5380,
            ni_xenon_renderer_purge_all_d3d_shaders(Ptr<NiDX9Renderer>)
        ),
        entry!(0x00e7_57d0, fn_00e757d0(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e7_5810, fn_00e75810(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e7_5840, fn_00e75840(Ptr<NiDX9Renderer>, Ptr)),
        entry!(0x00e7_58f0, fn_00e758f0(Ptr<NiDX9Renderer>, Ptr)),
        entry!(0x00e7_59c0, fn_00e759c0(Ptr<NiDX9Renderer>, Ptr)),
        entry!(
            0x00e7_62f0,
            fn_00e762f0(Ptr<NiDX9Renderer>, u32) -> Ptr<NiDX9Renderer>
        ),
        entry!(0x00e7_bc90, fn_00e7bc90(Ptr, u32, u32) -> Ptr),
        entry!(
            0x00e6_da60,
            fn_00e6da60(Ptr<NiDX9Renderer>, Ptr, Ptr, u32, u32) -> bool
        ),
        entry!(0x00e7_1e40, fn_00e71e40(Ptr<NiDX9Renderer>, u32)),
        entry!(
            0x00e7_2980,
            fn_00e72980(Ptr<NiDX9Renderer>, u32, Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e7_2aa0,
            fn_00e72aa0(Ptr<NiDX9Renderer>, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e6_9ea0,
            fn_00e69ea0(Ptr<NiDX9Renderer>, u32, u32, u32, u32, u32, u32, u16)
        ),
        entry!(
            0x00e6_ab40,
            fn_00e6ab40(Ptr<NiDX9Renderer>, u32, u32, u32, u32, u32, u32, u32, u16)
        ),
        entry!(
            0x00e6_c780,
            ni_xenon_renderer_do_set_camera_data(Ptr<NiDX9Renderer>, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00e6_cf40, fn_00e6cf40(Ptr<NiDX9Renderer>, Ptr)),
        entry!(
            0x00e7_1fe0,
            ni_xenon_renderer_pack_geometry_buffer(
                Ptr<NiDX9Renderer>,
                Ptr,
                Ptr,
                u32,
                u32,
                u8,
            ) -> bool
        ),
        entry!(0x00e7_4120, fn_00e74120(Ptr<NiDX9Renderer>)),
        entry!(0x00e7_0140, fn_00e70140(Ptr<NiDX9Renderer>, Ptr)),
        entry!(0x00a5_f200, ni_vertex_color_property_get_rtti(Ptr) -> u32),
        entry!(0x00a5_f4f0, ni_z_buffer_property_get_rtti(Ptr) -> u32),
        entry!(0x00a5_f500, ni_z_buffer_property_type(Ptr) -> u32),
        entry!(0x00e7_57b0, fn_00e757b0(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_95c0, fn_00e695c0(Ptr<NiDX9Renderer>, u32) -> u32),
        entry!(0x00e6_9610, fn_00e69610(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_9650, fn_00e69650(Ptr<NiDX9Renderer>, u32) -> bool),
        entry!(
            0x00e6_98a0,
            fn_00e698a0(Ptr<NiDX9Renderer>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e6_98d0,
            fn_00e698d0(Ptr<NiDX9Renderer>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e6_9900, fn_00e69900(Ptr<NiDX9Renderer>, u32, u32)),
        entry!(0x00e6_9920, fn_00e69920(Ptr<NiDX9Renderer>)),
        entry!(0x00e6_9a00, fn_00e69a00(Ptr<NiDX9Renderer>, u32) -> u32),
        entry!(0x00e6_9ad0, fn_00e69ad0(Ptr) -> u32),
        entry!(
            0x00e6_9b60,
            fn_00e69b60(Ptr, Ptr, Ptr, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00e6_9c00, fn_00e69c00(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_b8f0, fn_00e6b8f0(Ptr, Ptr, Ptr, f32)),
        entry!(
            0x00e6_bdc0,
            fn_00e6bdc0(Ptr<NiDX9Renderer>, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e6_c690, fn_00e6c690(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_c760, fn_00e6c760(Ptr<NiDX9Renderer>, u32) -> bool),
        entry!(0x00e6_c770, fn_00e6c770(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_d710, fn_00e6d710(Ptr<NiDX9Renderer>, u32) -> u32),
        entry!(0x00e6_dd00, fn_00e6dd00(Ptr<NiDX9Renderer>, u32) -> bool),
        entry!(0x00e6_dd40, fn_00e6dd40(Ptr<NiDX9Renderer>, u32) -> bool),
        entry!(
            0x00e6_dd90,
            ni_dx9_renderer_lock_dynamic_texture(Ptr<NiDX9Renderer>, u32, Ptr) -> u32
        ),
        entry!(
            0x00e6_de70,
            ni_dx9_renderer_unlock_dynamic_texture(Ptr<NiDX9Renderer>, u32) -> bool
        ),
        entry!(0x00e6_df60, fn_00e6df60(Ptr<NiDX9Renderer>, u32) -> u32),
        entry!(0x00e6_dfa0, fn_00e6dfa0(Ptr<NiDX9Renderer>, u32) -> u32),
        entry!(0x00e7_57c0, fn_00e757c0(Ptr<NiDX9Renderer>) -> u8),
        entry!(0x00e7_57e0, fn_00e757e0(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e7_57f0, fn_00e757f0(Ptr<NiDX9Renderer>, u32) -> bool),
        entry!(0x00e7_5820, fn_00e75820(Ptr<NiDX9Renderer>, f32)),
        entry!(0x00e7_5830, fn_00e75830(Ptr<NiDX9Renderer>) -> f64),
        entry!(0x00e7_5a20, fn_00e75a20(Ptr<NiDX9Renderer>, u32)),
        entry!(0x00e7_5a30, fn_00e75a30(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e7_5a40, fn_00e75a40(Ptr<NiDX9Renderer>) -> f64),
        entry!(0x00e7_5a50, fn_00e75a50(Ptr<NiDX9Renderer>, f32) -> u32),
        entry!(0x00e6_f100, fn_00e6f100(Ptr<NiDX9Renderer>, u32) -> bool),
        entry!(
            0x00e6_f260,
            fn_00e6f260(Ptr<NiDX9Renderer>, u32, u32) -> bool
        ),
        entry!(
            0x00e6_f4e0,
            fn_00e6f4e0(Ptr<NiDX9Renderer>, Ptr, u32) -> u32
        ),
        entry!(
            0x00e6_f6e0,
            ni_dx9_renderer_fast_copy(Ptr<NiDX9Renderer>, u32, u32, Ptr, u32, u32) -> bool
        ),
        entry!(
            0x00e6_f840,
            ni_dx9_renderer_copy(Ptr<NiDX9Renderer>, u32, u32, Ptr, Ptr, u32) -> bool
        ),
        entry!(0x00e6_f9b0, fn_00e6f9b0(Ptr<NiDX9Renderer>, u32) -> bool),
        entry!(0x00e7_2df0, fn_00e72df0(Ptr, u32) -> Ptr),
        entry!(0x00e7_2e20, fn_00e72e20(Ptr, u32) -> Ptr),
        entry!(0x00e7_44b0, fn_00e744b0(Ptr<NiDX9Renderer>, Ptr)),
        entry!(
            0x00e7_52b0,
            ni_xenon_renderer_register_d3d_shader(Ptr<NiDX9Renderer>, u32)
        ),
        entry!(0x00e7_aed0, fn_00e7aed0(Ptr, u8, u32) -> u32),
        entry!(0x00e7_b190, fn_00e7b190(Ptr, u32, u32, u32, Ptr) -> u32),
        entry!(0x00e7_11e0, fn_00e711e0(Ptr<NiDX9Renderer>)),
        entry!(
            0x00e7_18a0,
            fn_00e718a0(Ptr<NiDX9Renderer>, Ptr, u32, u32, Ptr, Ptr)
        ),
        entry!(0x00e7_0720, fn_00e70720(Ptr<NiDX9Renderer>, Ptr)),
        entry!(0x00e7_0be0, fn_00e70be0(Ptr<NiDX9Renderer>, Ptr)),
        entry!(0x00e7_23a0, fn_00e723a0(Ptr<NiDX9Renderer>, Ptr, u32)),
        entry!(0x00e7_36b0, fn_00e736b0(Ptr<NiDX9Renderer>) -> bool),
        entry!(
            0x00e7_4a40,
            ni_xenon_renderer_add_to_batch(Ptr<NiDX9Renderer>, Ptr)
        ),
    ]
}

// ---- Direct3D creation and the renderer's initialisation (batch b000t) ----

/// The Direct3D globals: the `IDirect3D9` object (`0126f0d8`), the byte set when it is an
/// `IDirect3D9Ex` (`0126f0d0`), and the two creation functions `00e69410` looks up
/// (`Direct3DCreate9` at `0126f0dc`, `Direct3DCreate9Ex` at `0126f0e0`; both `stdcall`).
const DIRECT3D: u32 = 0x0126_f0d8;
const DIRECT3D_IS_EX: u32 = 0x0126_f0d0;
const DIRECT3D_CREATE9: u32 = 0x0126_f0dc;
const DIRECT3D_CREATE9_EX: u32 = 0x0126_f0e0;
/// `00e69410` (unnamed, `nixenonrenderer.cpp`; no arguments): a nonzero byte when the
/// Direct3D library is usable.
const LOAD_DIRECT3D: u32 = 0x00e6_9410;
/// `00a29680` (unnamed; `cdecl`, a message) and the messages `CreateDirect3D9` passes it.
const REPORT_00A29680: u32 = 0x00a2_9680;
const CREATE_EX_FAILED_010ED598: u32 = 0x010e_d598;
const CREATE_EX_SUCCEEDED_010ED570: u32 = 0x010e_d570;
const CREATE_FAILED_010ED550: u32 = 0x010e_d550;
const CREATE_SUCCEEDED_010ED52C: u32 = 0x010e_d52c;
/// The SDK version both creation functions are given.
const D3D_SDK_VERSION: u32 = 0x20;

/// The format's record id (`00e7acb0`, cdecl: a D3D format) and the nearest refresh rate of a
/// mode record (`00e7af60`, `thiscall` on the record: the wanted rate), by address.
#[cfg(test)]
const RECORD_ID_00E7ACB0: u32 = 0x00e7_acb0;
#[cfg(test)]
const NEAREST_RATE_00E7AF60: u32 = 0x00e7_af60;

// Translated from 00e69460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::CreateDirect3D9` (engine map), `cdecl`, one word (its low byte: try the
/// `Ex` interface). Answers 0, or -1 on failure; with the object already made (`0126f0d8`) it
/// answers 0 at once. When the library is not usable (`00e69410`) it reports "Create D3D9
/// instance...FAILED" and answers -1. With `Direct3DCreate9Ex` found and the byte set it calls
/// it (version 0x20, an out word): a null object reports "Create D3D9Ex
/// instance...FAILED" and makes the answer -1 (the plain creation is still tried), otherwise
/// the `Ex` byte and the object are set and "...SUCCESSFUL" reported. Without an object then,
/// `Direct3DCreate9(0x20)` makes it: null reports "Create D3D9 instance...FAILED" and answers
/// -1; otherwise it reports "...SUCCESSFUL" and answers the earlier result.
pub fn ni_xenon_renderer_create_direct3d9(e: &mut Engine, try_ex: u8) -> i32 {
    let mut result = 0i32;
    if e.mem.u32(DIRECT3D) != 0 {
        return result;
    }
    if e.call(LOAD_DIRECT3D, &args![]).u8() == 0 {
        e.call(REPORT_00A29680, &args![CREATE_FAILED_010ED550]);
        return -1;
    }
    let create_ex = e.mem.u32(DIRECT3D_CREATE9_EX);
    if create_ex != 0 && try_ex != 0 {
        // The out word is the slot of the pushed ECX.
        let object = e.with_stack(4, |e, out| {
            e.call(create_ex, &args![D3D_SDK_VERSION, out]);
            e.mem.u32(out.addr())
        });
        if object == 0 {
            e.call(REPORT_00A29680, &args![CREATE_EX_FAILED_010ED598]);
            result = -1;
        } else {
            e.mem.set_u8(DIRECT3D_IS_EX, 1);
            e.mem.set_u32(DIRECT3D, object);
            e.call(REPORT_00A29680, &args![CREATE_EX_SUCCEEDED_010ED570]);
        }
    }
    if e.mem.u32(DIRECT3D) != 0 {
        return result;
    }
    let create = e.mem.u32(DIRECT3D_CREATE9);
    let object = e.call(create, &args![D3D_SDK_VERSION]).u32();
    e.mem.set_u32(DIRECT3D, object);
    if object == 0 {
        e.call(REPORT_00A29680, &args![CREATE_FAILED_010ED550]);
        return -1;
    }
    e.call(REPORT_00A29680, &args![CREATE_SUCCEEDED_010ED52C]);
    result
}

/// `IDirect3D9::CreateDevice` (vtable slot 0x40) and `IDirect3D9Ex::CreateDeviceEx` (0x50),
/// named by the interfaces' slot order; the arguments match.
const CREATE_DEVICE: u32 = 0x40;
const CREATE_DEVICE_EX: u32 = 0x50;
/// The adapter list (`0126f0e8`: element pointer +8, 16-bit count +0xe).
const ADAPTER_LIST: u32 = 0x0126_f0e8;
/// Globals the initialisation clears first (a `float` and a byte).
const ZEROED_VALUE_0126EFD4: u32 = 0x0126_efd4;
const ZEROED_FLAG_0126EFD1: u32 = 0x0126_efd1;
/// The error text (`011f4648`, 0x100 bytes, filled by `_strncpy_s` `00ec8d5f` or by
/// `00e69ad0`, `cdecl` with a message).
const ERROR_MESSAGE_011F4648: u32 = 0x011f_4648;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const SET_ERROR_00E69AD0: u32 = 0x00e6_9ad0;
/// The callees of the initialisation (`nixenonrenderer.cpp` unless noted; unnamed in the
/// engine map unless noted).
const RESET_00E69A60: u32 = 0x00e6_9a60;
const DEVICE_SETTINGS_00E69510: u32 = 0x00e6_9510;
const PRESENT_PARAMETERS_00E6BF50: u32 = 0x00e6_bf50;
const AFTER_DEVICE_00E6C240: u32 = 0x00e6_c240;
const BACK_BUFFER_00E7D6B0: u32 = 0x00e7_d6b0;
const DEPTH_STENCIL_00E7D850: u32 = 0x00e7_d850;
const RENDER_TARGET_GROUP_00EE8710: u32 = 0x00ee_8710;
const NI_POINTER_CONSTRUCT_00B6AED0: u32 = 0x00b6_aed0;
const WINDOW_TARGET_00E72CF0: u32 = 0x00e7_2cf0;
const FORMAT_00E69D00: u32 = 0x00e6_9d00;
const FORMAT_00E69C90: u32 = 0x00e6_9c90;
const TEXTURE_DEFAULTS_00E6EC90: u32 = 0x00e6_ec90;
const SHADER_LIBRARY_00E91590: u32 = 0x00e9_1590;
const INITIALIZE_SHADER_SYSTEM: u32 = 0x00e8_1940;
const NEW_OBJECT: u32 = 0x00aa_13e0;
const CONSTRUCT_00E8D0E0: u32 = 0x00e8_d0e0;
const CONSTRUCT_00E8FB20: u32 = 0x00e8_fb20;
const CONSTRUCT_00E90A80: u32 = 0x00e9_0a80;
const GEOMETRY_GROUP_MANAGER_00E88EB0: u32 = 0x00e8_8eb0;
const SET_FLAG_00E88C20: u32 = 0x00e8_8c20;
const CONSTRUCT_00E8BAE0: u32 = 0x00e8_bae0;
const NI_FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const NI_SHADER_FACTORY_GET_INSTANCE: u32 = 0x00a9_fff0;
const CONSTRUCT_00E909E0: u32 = 0x00e9_09e0;
const SET_SHADER_00E6E6E0: u32 = 0x00e6_e6e0;
/// The `float` constants of the default frustum (`01012054`, `0101e2bc`), the `float` 2^32
/// (`01099378`) and the `double` the height-to-width ratio is compared with (`0101de30`).
const FRUSTUM_SIDE_01012054: u32 = 0x0101_2054;
const FRUSTUM_NEAR_0101E2BC: u32 = 0x0101_e2bc;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const RATIO_LIMIT_0101DE30: u32 = 0x0101_de30;
/// The four words passed to the renderer's virtual slot 0x18c with the frustum and viewport.
const CAMERA_WORDS_SLOT_18C: [u32; 4] = [0x011f_426c, 0x011a_946c, 0x011a_9478, 0x011a_9484];
/// The messages and error texts of the initialisation.
const INVALID_DEVICE_TYPE_010EE20C: u32 = 0x010e_e20c;
const INVALID_DEVICE_ID_010EE1C8: u32 = 0x010e_e1c8;
const INVALID_ADAPTER_010EE1A0: u32 = 0x010e_e1a0;
const INVALID_ADAPTER_REPORT_010EE160: u32 = 0x010e_e160;
const ATTEMPTING_010EE118: u32 = 0x010e_e118;
const DEVICE_NOT_VALID_010EE0E8: u32 = 0x010e_e0e8;
const NO_DEVICE_OF_TYPE_010EE0AC: u32 = 0x010e_e0ac;
const FEWER_BACK_BUFFERS_010EE048: u32 = 0x010e_e048;
const NEXT_LOWER_TYPE_010EDFE0: u32 = 0x010e_dfe0;
const NO_HARDWARE_DEVICE_010EDFAC: u32 = 0x010e_dfac;
const NO_DEVICE_OF_ANY_TYPE_010EDF60: u32 = 0x010e_df60;
const NO_TEXTURE_DEFAULTS_010EDF10: u32 = 0x010e_df10;
const NO_REFERENCE_DEVICE_010EDEDC: u32 = 0x010e_dedc;
const NO_REFERENCE_DEVICE_REPORT_010EDE88: u32 = 0x010e_de88;
const NO_HARDWARE_OF_ANY_TYPE_010EDE30: u32 = 0x010e_de30;

/// The renderer's fields the initialisation uses: the device (+0x288), the device type
/// (+0x5c8) and behaviour flags (+0x5cc) `00e69510` fills, the adapter (+0x5c4), the adapter
/// and device descriptions found (+0x5d8, +0x5dc), and the 14 words kept from +0xa98.
const DEVICE: u32 = 0x288;
const DEVICE_TYPE: u32 = 0x5c8;
const BEHAVIOR_FLAGS: u32 = 0x5cc;
const ADAPTER: u32 = 0x5c4;
const WINDOW: u32 = 0x3bc;
const FOCUS_WINDOW: u32 = 0x3c0;
/// The initialisation's frame (`SUB ESP,0x80` and four pushed registers): the vertex-buffer
/// and depth-stencil out words (+0x10, +0x14), the `CreateDeviceEx` out words (+0x18,
/// +0x1c), the full-screen display mode (+0x24, 0x18 bytes; later the viewport), the frustum
/// (+0x3c) and the present parameters (+0x58, 0x38 bytes).
const INIT_FRAME: u32 = 0x90;
const FRAME_BLOCK_OUT: u32 = 0x10;
const FRAME_DEPTH_OUT: u32 = 0x14;
const FRAME_EX_OUT: u32 = 0x18;
const FRAME_EX_RETRY_OUT: u32 = 0x1c;
const FRAME_MODE: u32 = 0x24;
const FRAME_FRUSTUM: u32 = 0x3c;
const FRAME_PRESENT: u32 = 0x58;

/// The adapter description for the renderer's adapter (+0x5c4): the list's element when the
/// index is below its 16-bit count, else null.
fn adapter_description(e: &Engine, this: u32) -> u32 {
    let list = e.mem.u32(ADAPTER_LIST);
    let index = e.mem.u32(this + ADAPTER);
    if index < u32::from(e.mem.u16(list.wrapping_add(0xe))) {
        let elements = e.mem.u32(list.wrapping_add(8));
        e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)))
    } else {
        0
    }
}

/// The adapter description's device description for the renderer's device type: +0x460 for
/// type 1, else +0x464.
fn device_description(e: &Engine, this: u32, adapter: u32) -> u32 {
    if e.mem.u32(this + DEVICE_TYPE) == 1 {
        e.mem.u32(adapter.wrapping_add(0x460))
    } else {
        e.mem.u32(adapter.wrapping_add(0x464))
    }
}

/// `CreateDevice` or, on an `Ex` object, `CreateDeviceEx` (with the full-screen display mode
/// only when the present parameters are not windowed, +0x20, and the device through `out`);
/// answers the `HRESULT`.
fn create_device(e: &mut Engine, this: u32, frame: u32, out: u32) -> i32 {
    let direct3d = e.mem.u32(DIRECT3D);
    let present = frame + FRAME_PRESENT;
    let adapter = e.mem.u32(this + ADAPTER);
    let device_type = e.mem.u32(this + DEVICE_TYPE);
    let focus = e.mem.u32(this + FOCUS_WINDOW);
    let behavior = e.mem.u32(this + BEHAVIOR_FLAGS);
    if e.mem.u8(DIRECT3D_IS_EX) != 0 {
        let mode = if e.mem.u32(present + 0x20) != 0 {
            0
        } else {
            frame + FRAME_MODE
        };
        let result = e
            .vcall(
                direct3d,
                CREATE_DEVICE_EX,
                &args![adapter, device_type, focus, behavior, present, mode, out],
            )
            .i32();
        let device = e.mem.u32(out);
        e.mem.set_u32(this + DEVICE, device);
        result
    } else {
        e.vcall(
            direct3d,
            CREATE_DEVICE,
            &args![
                adapter,
                device_type,
                focus,
                behavior,
                present,
                this + DEVICE
            ],
        )
        .i32()
    }
}

/// A word made unsigned by `FILD` and adding the `float` 2^32 when negative.
fn unsigned_on_x87(e: &Engine, word: u32) -> f64 {
    let value = f64::from(word as i32);
    if (word as i32) < 0 {
        x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)))
    } else {
        value
    }
}

/// What ends a pass of the device loop.
enum DeviceAttempt {
    Created,
    NextType,
}

// Translated from 00e72e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`; its messages call it
/// `NiDX9Renderer::Initialize`), `thiscall` with 14 words; answers a byte, 1 on success. The
/// parameter names follow what the code does with them: the width and height go to the
/// full-screen display mode, `flags` bits 0x40 and 0x20 add behaviour flags 2 and 4, the window
/// (+0x3bc) goes to the present parameters' set-up and the window target, the focus window
/// (+0x3c0) and adapter (+0x5c4) to `CreateDevice`, the device kind to `00e69510`, and the
/// last word is the display mode's refresh rate; the others are only kept and passed on.
///
/// Keeps the 14 words at +0xa98, clears the `float` at `0126efd4` and the byte `0126efd1`,
/// runs `00e69a60`, and lets `00e69510` turn the device kind into the device type and
/// behaviour flags (failing: "Invalid 3D device type"). The adapter's description (none:
/// "Invalid Adapter") and its device description for the type (none, or without caps: "Invalid
/// 3D device type") are kept. Then, with the present parameters cleared, for each device kind
/// from the requested one: the device description must still be valid ("Requested device not
/// valid"); `00e6bf50` fills the present parameters; the device is created (`CreateDevice`, or
/// `CreateDeviceEx` with a full-screen display mode of size 0x18: width, height, refresh rate,
/// the back buffer format, scan-line ordering 1), +0xad0 is set when height / width (as
/// unsigned values on the x87) is below 0.75 or unordered; a failure clears +0x6fc and, when
/// the call changed the back buffer count, is retried once ("trying fewer backbuffers"). A
/// failed kind moves to the next one: after the reference kind (4) "Could not create requested
/// reference device"; reaching kind 4 "Could not create hardware device of any type"; a kind
/// `00e69510` refuses "Invalid 3D device type". With a device: none at +0x288 fails ("Could not
/// create hardware device"); `00e6c240` must succeed; +0x5d4 and +0x5d5 get the behaviour
/// flags' inverted bit 6 and bit 7; the back buffer and (with automatic depth-stencil) the
/// depth surface are fetched; the render target group (`00ee8710`) gets them (slots 0xa8,
/// 0xac), the window's target is set (`00e72cf0` with an `NiPointer` to the group), the format
/// of the group's first buffer is kept at +0x5d0; the texture defaults (`00e6ec90`, failing:
/// "Could not initialize texture defaults") and the shader and geometry objects are made
/// (+0x8b8 to +0x8c8, `NiD3DShaderInterface::InitializeShaderSystem`); the default frustum
/// (-1, 1, 1, -1, the near `float` at `0101e2bc`, 1) and viewport (0, 1, 1, 0) go to the
/// renderer's slot 0x18c; then `NiShaderFactory::GetInstance`, a 0x40-byte object (`00e909e0`;
/// its slot 0x4c runs even when the allocation failed) handed to `00e6e6e0`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e72e60(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    flags: u32,
    window: u32,
    focus_window: u32,
    adapter: u32,
    device_kind: u32,
    eighth: u32,
    ninth: u32,
    tenth: u32,
    eleventh: u32,
    twelfth: u32,
    thirteenth: u32,
    refresh_rate: u32,
) -> u8 {
    let this = this.addr();
    let words = [
        width,
        height,
        flags,
        window,
        focus_window,
        adapter,
        device_kind,
        eighth,
        ninth,
        tenth,
        eleventh,
        twelfth,
        thirteenth,
        refresh_rate,
    ];
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(this + 0xa98 + 4 * i as u32, *word);
    }
    // FLDZ / FSTP float.
    e.mem.set_u32(ZEROED_VALUE_0126EFD4, x87_store32(0.0));
    e.mem.set_u8(ZEROED_FLAG_0126EFD1, 0);
    e.call(RESET_00E69A60, &args![]);
    e.mem.set_u32(this + ADAPTER, adapter);
    e.mem.set_u32(this + WINDOW, window);
    e.mem.set_u32(this + FOCUS_WINDOW, focus_window);
    let device_type_at = this + DEVICE_TYPE;
    let behavior_at = this + BEHAVIOR_FLAGS;
    let report = |e: &mut Engine, message: u32| {
        e.call(REPORT_00A29680, &args![message]);
    };
    let copy_error = |e: &mut Engine, message: u32| {
        e.call(
            STRNCPY_S,
            &args![ERROR_MESSAGE_011F4648, 0x100u32, message, 0xffu32],
        );
    };
    let invalid_device = |e: &mut Engine| {
        copy_error(e, INVALID_DEVICE_TYPE_010EE20C);
        report(e, INVALID_DEVICE_ID_010EE1C8);
        0u8
    };
    if e.call(
        DEVICE_SETTINGS_00E69510,
        &args![device_kind, device_type_at, behavior_at],
    )
    .u8()
        == 0
    {
        return invalid_device(e);
    }
    if flags as u8 & 0x40 != 0 {
        let value = e.mem.u32(behavior_at) | 2;
        e.mem.set_u32(behavior_at, value);
    }
    if flags as u8 & 0x20 != 0 {
        let value = e.mem.u32(behavior_at) | 4;
        e.mem.set_u32(behavior_at, value);
    }
    let adapter_desc = adapter_description(e, this);
    e.mem.set_u32(this + 0x5d8, adapter_desc);
    if adapter_desc == 0 {
        copy_error(e, INVALID_ADAPTER_010EE1A0);
        report(e, INVALID_ADAPTER_REPORT_010EE160);
        return 0;
    }
    let device_desc = device_description(e, this, adapter_desc);
    let kept = if e.mem.u32(device_desc.wrapping_add(4)) != 0 {
        device_desc
    } else {
        0
    };
    e.mem.set_u32(this + 0x5dc, kept);
    if kept == 0 {
        return invalid_device(e);
    }

    e.with_stack(INIT_FRAME, |e, frame| {
        let frame = frame.addr();
        let present = frame + FRAME_PRESENT;
        e.call(MEMSET_ADDR, &args![present, 0u32, 0x38u32]);
        let mut kind = device_kind;
        loop {
            report(e, ATTEMPTING_010EE118);
            let adapter_desc = adapter_description(e, this);
            let device_desc = device_description(e, this, adapter_desc);
            let mut attempt = DeviceAttempt::NextType;
            if e.mem.u32(device_desc.wrapping_add(4)) == 0 || device_desc == 0 {
                copy_error(e, DEVICE_NOT_VALID_010EE0E8);
                report(e, NO_DEVICE_OF_TYPE_010EE0AC);
            } else {
                let filled = e
                    .call(
                        PRESENT_PARAMETERS_00E6BF50,
                        &args![
                            this,
                            e.mem.u32(this + WINDOW),
                            width,
                            height,
                            flags,
                            twelfth,
                            eighth,
                            ninth,
                            thirteenth,
                            eleventh,
                            refresh_rate,
                            tenth,
                            present
                        ],
                    )
                    .u8();
                if filled != 0 {
                    let back_buffers = e.mem.u32(present + 0xc);
                    let mode = frame + FRAME_MODE;
                    for i in 0..6 {
                        e.mem.set_u32(mode + 4 * i, 0);
                    }
                    let back_buffer_format = e.mem.u32(present + 8);
                    e.mem.set_u32(mode + 0xc, refresh_rate);
                    e.mem.set_u32(mode, 0x18);
                    e.mem.set_u32(mode + 8, height);
                    e.mem.set_u32(mode + 4, width);
                    e.mem.set_u32(mode + 0x14, 1);
                    e.mem.set_u32(mode + 0x10, back_buffer_format);
                    let result = create_device(e, this, frame, frame + FRAME_EX_OUT);
                    let tall = unsigned_on_x87(e, e.mem.u32(present + 4));
                    let wide = unsigned_on_x87(e, e.mem.u32(present));
                    let ratio = x87_div(tall, wide);
                    let limit = e.mem.f64(RATIO_LIMIT_0101DE30);
                    // FCOMP / FNSTSW / TEST AH,1: C0 is set below the limit or unordered.
                    let below = !(ratio >= limit);
                    e.mem.set_u8(this + 0xad0, u8::from(below));
                    if result >= 0 {
                        attempt = DeviceAttempt::Created;
                    } else {
                        e.mem.set_u8(this + 0x6fc, 0);
                        if back_buffers != e.mem.u32(present + 0xc) {
                            report(e, FEWER_BACK_BUFFERS_010EE048);
                            let retry = create_device(e, this, frame, frame + FRAME_EX_RETRY_OUT);
                            if retry >= 0 {
                                attempt = DeviceAttempt::Created;
                            }
                        }
                    }
                }
            }
            if let DeviceAttempt::Created = attempt {
                break;
            }
            if kind == 4 {
                e.call(SET_ERROR_00E69AD0, &args![NO_REFERENCE_DEVICE_010EDEDC]);
                report(e, NO_REFERENCE_DEVICE_REPORT_010EDE88);
                return 0;
            }
            kind = kind.wrapping_add(1);
            if kind == 4 {
                report(e, NO_HARDWARE_OF_ANY_TYPE_010EDE30);
                return 0;
            }
            report(e, NEXT_LOWER_TYPE_010EDFE0);
            let accepted = e
                .call(
                    DEVICE_SETTINGS_00E69510,
                    &args![kind, device_type_at, behavior_at],
                )
                .u8();
            if accepted == 0 {
                e.call(SET_ERROR_00E69AD0, &args![INVALID_DEVICE_TYPE_010EE20C]);
                report(e, INVALID_DEVICE_ID_010EE1C8);
                return 0;
            }
        }

        if e.mem.u32(this + DEVICE) == 0 {
            e.call(SET_ERROR_00E69AD0, &args![NO_HARDWARE_DEVICE_010EDFAC]);
            report(e, NO_DEVICE_OF_ANY_TYPE_010EDF60);
            return 0;
        }
        if e.call(AFTER_DEVICE_00E6C240, &args![this, present]).u8() == 0 {
            return 0;
        }
        let behavior = e.mem.u32(behavior_at);
        e.mem.set_u8(this + 0x5d4, !((behavior >> 6) as u8) & 1);
        e.mem.set_u8(this + 0x5d5, ((behavior >> 7) as u8) & 1);
        let block_out = frame + FRAME_BLOCK_OUT;
        let depth_out = frame + FRAME_DEPTH_OUT;
        e.mem.set_u32(block_out, 0);
        e.mem.set_u32(depth_out, 0);
        let device = e.mem.u32(this + DEVICE);
        let blocks = e
            .call(BACK_BUFFER_00E7D6B0, &args![device, present, block_out])
            .u32();
        if e.mem.u32(present + 0x24) != 0 {
            let device = e.mem.u32(this + DEVICE);
            e.call(DEPTH_STENCIL_00E7D850, &args![device, depth_out]);
        }
        let group = e
            .call(RENDER_TARGET_GROUP_00EE8710, &args![1u32, this])
            .u32();
        e.mem.set_u32(this + 0x884, group);
        let back_buffer = e.mem.u32(block_out);
        e.vcall(group, 0xa8, &args![back_buffer, 0u32]);
        let group = e.mem.u32(this + 0x884);
        let depth = e.mem.u32(depth_out);
        e.vcall(group, 0xac, &args![depth]);
        // An `NiPointer` to the group made in the outgoing argument slot, passed by value.
        let pointer = e.with_stack(4, |e, slot| {
            let group = e.mem.u32(this + 0x884);
            e.call(NI_POINTER_CONSTRUCT_00B6AED0, &args![slot, group]);
            e.mem.u32(slot.addr())
        });
        e.call(
            WINDOW_TARGET_00E72CF0,
            &args![this + 0x890, window, pointer],
        );
        let group = e.mem.u32(this + 0x884);
        let buffer = e.vcall(group, 0x9c, &args![0u32]).u32();
        let raw_format = e.mem.u32(buffer.wrapping_add(0xc));
        let converted = e.call(FORMAT_00E69D00, &args![raw_format]).u32();
        let format = e.call(FORMAT_00E69C90, &args![converted]).u32();
        e.mem.set_u32(this + 0x5d0, format);
        if e.call(TEXTURE_DEFAULTS_00E6EC90, &args![this]).u8() == 0 {
            report(e, NO_TEXTURE_DEFAULTS_010EDF10);
            return 0;
        }
        let library = e
            .call(SHADER_LIBRARY_00E91590, &args![this, this + 0x28c, 1u32])
            .u32();
        e.mem.set_u32(this + 0x8b8, library);
        e.call(INITIALIZE_SHADER_SYSTEM, &args![this]);
        let made = |e: &mut Engine, size: u32, construct: u32, words: &[u32]| {
            let block = e.call(NEW_OBJECT, &args![size]).u32();
            if block == 0 {
                return 0;
            }
            let mut all = vec![block];
            all.extend_from_slice(words);
            e.call(construct, &all).u32()
        };
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x100, CONSTRUCT_00E8D0E0, &[device]);
        e.mem.set_u32(this + 0x8bc, object);
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x4c, CONSTRUCT_00E8FB20, &[device]);
        e.mem.set_u32(this + 0x8c0, object);
        let object = made(e, 0x10, CONSTRUCT_00E90A80, &[this]);
        e.mem.set_u32(this + 0x8c4, object);
        let device = e.mem.u32(this + DEVICE);
        let first = e.mem.u32(this + 0x8bc);
        let manager = e
            .call(GEOMETRY_GROUP_MANAGER_00E88EB0, &args![device, first])
            .u32();
        e.mem.set_u32(this + 0x8ac, manager);
        let renderer_flags = e.vcall(this, 0xa0, &args![]).u32();
        let manager = e.mem.u32(this + 0x8ac);
        e.call(
            SET_FLAG_00E88C20,
            &args![manager, u32::from((renderer_flags >> 1) as u8 & 1)],
        );
        let manager = e.mem.u32(this + 0x8ac);
        let group_1 = e.vcall(manager, 4, &args![1u32]).u32();
        e.mem.set_u32(this + 0x8b0, group_1);
        let manager = e.mem.u32(this + 0x8ac);
        let group_2 = e.vcall(manager, 4, &args![2u32]).u32();
        e.mem.set_u32(this + 0x8b4, group_2);
        let library = e.mem.u32(this + 0x8b8);
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x23c, CONSTRUCT_00E8BAE0, &[library, device]);
        e.mem.set_u32(this + 0x8c8, object);
        if e.mem.u32(blocks.wrapping_add(0x30)) != 0 {
            let library = e.mem.u32(this + 0x8b8);
            e.vcall(library, 0x34, &args![1u32]);
        }
        let group = e.mem.u32(this + 0x884);
        e.mem.set_u32(this + 0x888, group);
        let frustum = frame + FRAME_FRUSTUM;
        e.call(NI_FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        let side = x87_quiet_bits(e.mem.u32(FRUSTUM_SIDE_01012054));
        let near = x87_quiet_bits(e.mem.u32(FRUSTUM_NEAR_0101E2BC));
        let one = x87_store32(1.0);
        let zero = x87_store32(0.0);
        for (offset, value) in [
            (0xc, side),
            (0, side),
            (8, one),
            (4, one),
            (0x10, near),
            (0x14, one),
        ] {
            e.mem.set_u32(frustum + offset, value);
        }
        let viewport = frame + FRAME_MODE;
        for (offset, value) in [(0, zero), (0xc, zero), (4, one), (8, one)] {
            e.mem.set_u32(viewport + offset, value);
        }
        let [a, b, c, d] = CAMERA_WORDS_SLOT_18C;
        e.vcall(this, 0x18c, &args![a, b, c, d, frustum, viewport]);
        e.mem.set_u32(this + 0x888, 0);
        e.mem.set_u32(this + 0x88c, 0);
        e.call(NI_SHADER_FACTORY_GET_INSTANCE, &args![]);
        let block = e.call(NEW_OBJECT, &args![0x40u32]).u32();
        let shader = if block != 0 {
            e.call(CONSTRUCT_00E909E0, &args![block]).u32()
        } else {
            0
        };
        e.vcall(shader, 0x4c, &args![]);
        e.call(SET_SHADER_00E6E6E0, &args![this, shader]);
        1
    })
}

/// `memset` (`00ec61c0`).
const MEMSET_ADDR: u32 = 0x00ec_61c0;

/// The registry entry of [`fn_00e72e60`]: `this` and its 14 words.
fn fn_00e72e60_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let w = |i: usize| words[i];
    u32::from(fn_00e72e60(
        e,
        Ptr::new(w(0)),
        w(1),
        w(2),
        w(3),
        w(4),
        w(5),
        w(6),
        w(7),
        w(8),
        w(9),
        w(10),
        w(11),
        w(12),
        w(13),
        w(14),
    ))
    .into_ret()
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    const CREATE9: u32 = 0x7200_0000;
    const CREATE9_EX: u32 = 0x7200_0010;
    const OBJECT: u32 = 0x7300_0000;
    const OBJECT_EX: u32 = 0x7300_0100;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn direct3d_rig(usable: bool, ex: u32, ex_object: u32, object: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(DIRECT3D_CREATE9, CREATE9);
        e.mem.set_u32(DIRECT3D_CREATE9_EX, ex);
        e.register_double(LOAD_DIRECT3D, move |_, _| u32::from(usable).into_ret());
        e.register(REPORT_00A29680, |_, _| Ret::default());
        e.register_double(CREATE9_EX, move |e, a| {
            e.mem.set_u32(a[1], ex_object);
            Ret::default()
        });
        e.register_double(CREATE9, move |_, _| object.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_direct3d9_prefers_ex_when_asked() {
        let mut e = direct3d_rig(true, CREATE9_EX, OBJECT_EX, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), 0);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT_EX);
        assert_eq!(e.mem.u8(DIRECT3D_IS_EX), 1);
        assert!(calls(&e, CREATE9).is_empty());
        assert_eq!(calls(&e, CREATE9_EX)[0][0], D3D_SDK_VERSION);
        // Made already: nothing more.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), 0);
        assert!(calls(&e, LOAD_DIRECT3D).is_empty());
    }

    #[test]
    fn create_direct3d9_falls_back_and_keeps_the_ex_failure() {
        let mut e = direct3d_rig(true, CREATE9_EX, 0, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![
                vec![CREATE_EX_FAILED_010ED598],
                vec![CREATE_SUCCEEDED_010ED52C]
            ]
        );
        // Without the byte (or the Ex function) only the plain creation runs.
        let mut e = direct3d_rig(true, CREATE9_EX, OBJECT_EX, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![0x100u32]).i32(), 0);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT);
        assert!(calls(&e, CREATE9_EX).is_empty());
    }

    #[test]
    fn create_direct3d9_failures() {
        let mut e = direct3d_rig(false, 0, 0, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![CREATE_FAILED_010ED550]]
        );
        let mut e = direct3d_rig(true, 0, 0, 0);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(calls(&e, CREATE9), vec![vec![D3D_SDK_VERSION]]);
    }

    // ---- 00e72e60 ----

    const D3D_VTABLE: u32 = 0x7400_0000;
    const D3D_SLOTS: u32 = 0x7500_0000;
    const DEVICE_OBJECT: u32 = 0x7600_0000;
    const RENDERER_VTABLE: u32 = 0x7410_0000;
    const RENDERER_SLOTS: u32 = 0x7510_0000;
    const GROUP_VTABLE: u32 = 0x7420_0000;
    const GROUP_SLOTS: u32 = 0x7520_0000;
    const ADAPTER_INDEX: u32 = 0;
    const WORDS: [u32; 14] = [
        640,
        480,
        0x60,
        0x11,
        0x22,
        ADAPTER_INDEX,
        1,
        8,
        9,
        10,
        11,
        12,
        13,
        60,
    ];

    /// The Direct3D and renderer doubles. `create_results` are the `CreateDevice` results in
    /// order (the last repeats); `00e6bf50` sets the back buffer count to `back_buffers`.
    struct Setup {
        ex: bool,
        create_results: Vec<i32>,
        adapter_ok: bool,
        caps: u32,
    }

    fn vtable(e: &mut Engine, at: u32, slots: u32, count: u32) {
        let targets: Vec<u32> = (0..count).map(|i| slots + 4 * i).collect();
        e.put_vtable(at, &targets);
    }

    fn init_rig(setup: Setup) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_e000, 0x2000);
        e.map(0x0101_0000, 0x10000);
        e.map(0x0109_9000, 0x1000);
        e.mem.set_u32(FRUSTUM_SIDE_01012054, (-1.0f32).to_bits());
        e.mem.set_u32(FRUSTUM_NEAR_0101E2BC, 0.1f32.to_bits());
        e.mem.set_f64(RATIO_LIMIT_0101DE30, 0.75);
        e.mem
            .set_u32(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        // The Direct3D object and its slots.
        vtable(&mut e, D3D_VTABLE, D3D_SLOTS, 0x20);
        let direct3d = e.mem.alloc(4);
        e.mem.set_u32(direct3d, D3D_VTABLE);
        e.mem.set_u32(DIRECT3D, direct3d);
        e.mem.set_u8(DIRECT3D_IS_EX, u8::from(setup.ex));
        let mut results = setup.create_results.clone().into_iter();
        let mut last = 0;
        let slot = if setup.ex {
            CREATE_DEVICE_EX
        } else {
            CREATE_DEVICE
        };
        e.register_double(D3D_SLOTS + slot, move |e, a| {
            let result = results.next().unwrap_or(last);
            last = result;
            // Present parameters: a changed back buffer count on the first failure.
            if result < 0 {
                e.mem.set_u32(a[5] + 0xc, 1);
            }
            let out = if setup.ex { a[7] } else { a[6] };
            e.mem
                .set_u32(out, if result >= 0 { DEVICE_OBJECT } else { 0 });
            (result as u32).into_ret()
        });
        // The adapter list: one adapter with a device description for type 1.
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(ADAPTER_LIST, list);
        let elements = e.mem.alloc(4);
        e.mem.set_u32(list + 8, elements);
        e.mem.set_u16(list + 0xe, 1);
        let adapter = e.mem.alloc(0x468);
        if setup.adapter_ok {
            e.mem.set_u32(elements, adapter);
        }
        let device_desc = e.mem.alloc(0x144);
        e.mem.set_u32(device_desc + 4, setup.caps);
        e.mem.set_u32(adapter + 0x460, device_desc);
        e.mem.set_u32(adapter + 0x464, device_desc);
        // Device kind -> type 1 for kinds below 3, behaviour flags 0x40.
        e.register(DEVICE_SETTINGS_00E69510, |e, a| {
            e.mem.set_u32(a[1], 1);
            e.mem.set_u32(a[2], 0x40);
            u32::from(a[0] < 3).into_ret()
        });
        e.register(PRESENT_PARAMETERS_00E6BF50, |e, a| {
            let present = a[12];
            e.mem.set_u32(present, 640);
            e.mem.set_u32(present + 4, 480);
            e.mem.set_u32(present + 8, 0x16);
            e.mem.set_u32(present + 0xc, 2);
            e.mem.set_u32(present + 0x24, 1);
            1u32.into_ret()
        });
        let blocks = e.mem.alloc(0x40);
        e.mem.set_u32(blocks + 0x30, 1);
        e.register_double(BACK_BUFFER_00E7D6B0, move |e, a| {
            e.mem.set_u32(a[2], 0x5151);
            blocks.into_ret()
        });
        e.register(DEPTH_STENCIL_00E7D850, |e, a| {
            e.mem.set_u32(a[1], 0x5252);
            Ret::default()
        });
        // The render target group.
        vtable(&mut e, GROUP_VTABLE, GROUP_SLOTS, 0x30);
        let group = e.mem.alloc(8);
        e.mem.set_u32(group, GROUP_VTABLE);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer + 0xc, 0x77);
        e.register_double(RENDER_TARGET_GROUP_00EE8710, move |_, _| group.into_ret());
        e.register_double(GROUP_SLOTS + 0x9c, move |_, _| buffer.into_ret());
        e.register(GROUP_SLOTS + 0x4c, |_, _| Ret::default());
        e.register(GROUP_SLOTS + 4, |_, a| (0x900 + a[1]).into_ret());
        for slot in [0xa8, 0xac, 0x34] {
            e.register(GROUP_SLOTS + slot, |_, _| Ret::default());
        }
        e.register(NI_POINTER_CONSTRUCT_00B6AED0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        // Every constructor answers an object with the group's table.
        for construct in [
            CONSTRUCT_00E8D0E0,
            CONSTRUCT_00E8FB20,
            CONSTRUCT_00E90A80,
            CONSTRUCT_00E8BAE0,
            CONSTRUCT_00E909E0,
        ] {
            e.register(construct, |e, a| {
                e.mem.set_u32(a[0], GROUP_VTABLE);
                a[0].into_ret()
            });
        }
        e.register_double(GEOMETRY_GROUP_MANAGER_00E88EB0, move |_, _| {
            group.into_ret()
        });
        e.register_double(SHADER_LIBRARY_00E91590, move |_, _| group.into_ret());
        for no_op in [
            RESET_00E69A60,
            STRNCPY_S,
            REPORT_00A29680,
            SET_ERROR_00E69AD0,
            WINDOW_TARGET_00E72CF0,
            INITIALIZE_SHADER_SYSTEM,
            SET_FLAG_00E88C20,
            NI_FRUSTUM_CONSTRUCT,
            NI_SHADER_FACTORY_GET_INSTANCE,
            SET_SHADER_00E6E6E0,
            MEMSET_ADDR,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(FORMAT_00E69D00, |_, a| (a[0] + 1).into_ret());
        e.register(FORMAT_00E69C90, |_, a| (a[0] + 1).into_ret());
        for ok in [AFTER_DEVICE_00E6C240, TEXTURE_DEFAULTS_00E6EC90] {
            e.register(ok, |_, _| 1u32.into_ret());
        }
        // The renderer and its slots 0xa0 and 0x18c.
        vtable(&mut e, RENDERER_VTABLE, RENDERER_SLOTS, 0x70);
        e.register(RENDERER_SLOTS + 0xa0, |_, _| 2u32.into_ret());
        e.register(RENDERER_SLOTS + 0x18c, |e, a| {
            // Frustum and viewport as the renderer receives them.
            let frustum: Vec<f32> = (0..6).map(|i| e.mem.f32(a[5] + 4 * i)).collect();
            let viewport: Vec<f32> = (0..4).map(|i| e.mem.f32(a[6] + 4 * i)).collect();
            assert_eq!(frustum, vec![-1.0, 1.0, 1.0, -1.0, 0.1, 1.0]);
            assert_eq!(viewport, vec![0.0, 1.0, 1.0, 0.0]);
            Ret::default()
        });
        let this = e.mem.alloc(0xb80);
        e.mem.set_u32(this, RENDERER_VTABLE);
        e.call_log = Some(vec![]);
        (e, this)
    }

    fn initialize(e: &mut Engine, this: u32) -> u8 {
        let mut words = vec![this];
        words.extend_from_slice(&WORDS);
        e.call(0x00e7_2e60, &words).u8()
    }

    #[test]
    fn initialize_creates_the_device_and_the_renderer_objects() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 1);
        assert_eq!(e.mem.u32(this + 0xa98), 640);
        assert_eq!(e.mem.u32(this + 0xacc), 60);
        // Behaviour flags: 0x40 from 00e69510, 2 for flags & 0x40, 4 for flags & 0x20.
        assert_eq!(e.mem.u32(this + BEHAVIOR_FLAGS), 0x46);
        assert_eq!(e.mem.u8(this + 0x5d4), 0);
        assert_eq!(e.mem.u8(this + 0x5d5), 0);
        let create = calls(&e, D3D_SLOTS + CREATE_DEVICE);
        assert_eq!(create.len(), 1);
        assert_eq!(&create[0][1..5], &[ADAPTER_INDEX, 1, 0x22, 0x46]);
        assert_eq!(create[0][6], this + DEVICE);
        assert_eq!(e.mem.u32(this + DEVICE), DEVICE_OBJECT);
        // 480 / 640 = 0.75 is not below 0.75.
        assert_eq!(e.mem.u8(this + 0xad0), 0);
        let group = e.mem.u32(this + 0x884);
        assert_eq!(calls(&e, GROUP_SLOTS + 0xa8), vec![vec![group, 0x5151, 0]]);
        assert_eq!(calls(&e, GROUP_SLOTS + 0xac), vec![vec![group, 0x5252]]);
        assert_eq!(
            calls(&e, WINDOW_TARGET_00E72CF0),
            vec![vec![this + 0x890, 0x11, group]]
        );
        assert_eq!(e.mem.u32(this + 0x5d0), 0x79);
        assert_eq!(e.mem.u32(this + 0x8b0), 0x901);
        assert_eq!(e.mem.u32(this + 0x8b4), 0x902);
        assert_eq!(calls(&e, SET_FLAG_00E88C20)[0][1], 1);
        assert_eq!(calls(&e, GROUP_SLOTS + 0x34).len(), 1);
        assert_eq!(calls(&e, RENDERER_SLOTS + 0x18c).len(), 1);
        assert_eq!(e.mem.u32(this + 0x888), 0);
        let shader = calls(&e, SET_SHADER_00E6E6E0)[0][1];
        assert_ne!(shader, 0);
        assert_eq!(calls(&e, GROUP_SLOTS + 0x4c), vec![vec![shader]]);
    }

    #[test]
    fn initialize_retries_with_fewer_back_buffers_through_create_device_ex() {
        let (mut e, this) = init_rig(Setup {
            ex: true,
            create_results: vec![-1, 0],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 1);
        let create = calls(&e, D3D_SLOTS + CREATE_DEVICE_EX);
        assert_eq!(create.len(), 2);
        // Windowed is 0 in the present parameters: the display mode is passed.
        assert_ne!(create[0][6], 0);
        assert_ne!(create[1][7], create[0][7]);
        assert_eq!(e.mem.u32(this + DEVICE), DEVICE_OBJECT);
        assert!(calls(&e, REPORT_00A29680).contains(&vec![FEWER_BACK_BUFFERS_010EE048]));
    }

    #[test]
    fn initialize_walks_the_device_kinds_down_and_fails() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![-1],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 0);
        // Kinds 1 and 2 (each with its retry: the failure changed the back buffer count),
        // then kind 3 refused.
        assert_eq!(calls(&e, D3D_SLOTS + CREATE_DEVICE).len(), 4);
        let kinds: Vec<u32> = calls(&e, DEVICE_SETTINGS_00E69510)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(kinds, vec![1, 2, 3]);
        assert_eq!(
            calls(&e, SET_ERROR_00E69AD0),
            vec![vec![INVALID_DEVICE_TYPE_010EE20C]]
        );
    }

    #[test]
    fn initialize_refuses_a_missing_adapter_or_device() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: false,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![INVALID_ADAPTER_REPORT_010EE160]]
        );
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: true,
            caps: 0,
        });
        assert_eq!(initialize(&mut e, this), 0);
        assert_eq!(e.mem.u32(this + 0x5dc), 0);
        assert_eq!(calls(&e, STRNCPY_S)[0][2], INVALID_DEVICE_TYPE_010EE20C);
        assert!(calls(&e, D3D_SLOTS + CREATE_DEVICE).is_empty());
    }
}

// ---- The renderer's set-up helpers (batch b000v) ----

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_UP: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DOWN: u32 = 0x00fd_f1c4;

/// Releases a reference-counted object as the code inlines it: its count (+4) down through
/// `InterlockedDecrement` and, at zero, its virtual slot 4 (no stack words).
fn release_object(e: &mut Engine, object: u32) {
    if e.call(IMPORT_INTERLOCKED_DOWN, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00e69510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `cdecl`: a device kind, the device type
/// word and the behaviour flags word. Clears bits 4 to 7 of the flags, then for kinds 0 to 3
/// sets device type 1 (hardware) and for kinds 4 to 6 device type 2 with the flag 0x50, 0x40,
/// 0x80, 0x20, 0x20, 0x40, 0x80 (the jump table at `00e695a4`, read in Ghidra); answers 1, or
/// 0 for any other kind.
pub fn fn_00e69510(e: &mut Engine, kind: u32, device_type: Ptr, behavior: Ptr) -> u8 {
    let flags = e.mem.u32(behavior.addr()) & 0xffff_ff0f;
    e.mem.set_u32(behavior.addr(), flags);
    let (device, flag) = match kind {
        0 => (1u32, 0x50u32),
        1 => (1, 0x40),
        2 => (1, 0x80),
        3 => (1, 0x20),
        4 => (2, 0x20),
        5 => (2, 0x40),
        6 => (2, 0x80),
        _ => return 0,
    };
    e.mem.set_u32(device_type.addr(), device);
    let flags = e.mem.u32(behavior.addr()) | flag;
    e.mem.set_u32(behavior.addr(), flags);
    1
}

// Translated from 00e69c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `cdecl`, an index: the format number for
/// indexes 1 to 11 (the jump table at `00e69cc8`, read in Ghidra): 1 gives 0x14, 2 and 3 0x16,
/// 4 0x17, 5 and 6 0x18, 7 and 11 0x1e, 8 0x1b; anything else 0.
pub fn fn_00e69c90(_e: &mut Engine, index: u32) -> u32 {
    match index {
        1 => 0x14,
        2 | 3 => 0x16,
        4 => 0x17,
        5 | 6 => 0x18,
        7 | 11 => 0x1e,
        8 => 0x1b,
        _ => 0,
    }
}

// Translated from 00e69d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `cdecl`, a format number: its index (the
/// byte map at `00e69dd8` and jump table at `00e69d88`, read in Ghidra): 0x14 to 0x1e give 1 to
/// 0xb, 0x6f 0xc, 0x70 0xd, 0x24 0xe, 0x71 to 0x74 0xf to 0x12; anything else (and anything
/// above 0x74) 0.
pub fn fn_00e69d00(_e: &mut Engine, format: u32) -> u32 {
    match format {
        0x14..=0x1e => format - 0x13,
        0x6f => 0xc,
        0x70 => 0xd,
        0x24 => 0xe,
        0x71..=0x74 => format - 0x62,
        _ => 0,
    }
}

// Translated from 00e6e6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `thiscall` with a shader: a non-null
/// shader other than the one held at +0x14 replaces it (`NiPointer` assignment: the old one
/// released, the new one counted); a null shader changes nothing.
pub fn fn_00e6e6e0(e: &mut Engine, this: Ptr, shader: u32) {
    if shader == 0 {
        return;
    }
    let slot = this.addr().wrapping_add(0x14);
    let held = e.mem.u32(slot);
    if held == shader {
        return;
    }
    if held != 0 {
        release_object(e, held);
    }
    e.mem.set_u32(slot, shader);
    e.call(IMPORT_INTERLOCKED_UP, &args![shader.wrapping_add(4)]);
}

// Translated from 00e72cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`): the window-target map's set
/// (`NiTPointerMap<HWND, NiPointer<NiRenderTargetGroup>>` by the RTTI of the renderer's map at
/// +0x890), `thiscall`: the key and an `NiPointer` passed by value (its word). The bucket is
/// the map's virtual slot 4 of the key; an item whose key equals it (slot 8) has its value
/// replaced (`NiPointer` assignment); otherwise a new item (slot 0x14) is filled through slot
/// 0xc (given the key and a counted copy of the pointer), put at the bucket's head and
/// counted (+0xc). The by-value pointer is released at the end.
pub fn fn_00e72cf0(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let map = this.addr();
    let bucket = e.vcall(map, 4, &args![key]).u32();
    let buckets = e.mem.u32(map.wrapping_add(8));
    let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(map, 8, &args![key, item_key]).u8() != 0 {
            let held = e.mem.u32(item.wrapping_add(8));
            if held != value {
                if held != 0 {
                    release_object(e, held);
                }
                e.mem.set_u32(item.wrapping_add(8), value);
                if value != 0 {
                    e.call(IMPORT_INTERLOCKED_UP, &args![value.wrapping_add(4)]);
                }
            }
            if value != 0 {
                release_object(e, value);
            }
            return;
        }
        item = e.mem.u32(item);
    }
    let new_item = e.vcall(map, 0x14, &args![]).u32();
    if value != 0 {
        e.call(IMPORT_INTERLOCKED_UP, &args![value.wrapping_add(4)]);
    }
    e.vcall(map, 0xc, &args![new_item, key, value]);
    let buckets = e.mem.u32(map.wrapping_add(8));
    let head = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    e.mem.set_u32(new_item, head);
    let buckets = e.mem.u32(map.wrapping_add(8));
    e.mem
        .set_u32(buckets.wrapping_add(bucket.wrapping_mul(4)), new_item);
    let count = e.mem.u32(map.wrapping_add(0xc)).wrapping_add(1);
    e.mem.set_u32(map.wrapping_add(0xc), count);
    if value != 0 {
        release_object(e, value);
    }
}

/// The globals `00e6b990` copies twice (six words from `011a946c`), the `float`s it loads
/// (`0101e2bc`, `01016410`), the byte table it fills (`0126efec` to `0126f04d`) and the block
/// it clears (`0126efd8`, 0x76 bytes).
const DEFAULT_WORDS_011A946C: u32 = 0x011a_946c;
const FLOAT_SOURCE_01016410: u32 = 0x0101_6410;
const BIT_TABLE_0126EFEC: u32 = 0x0126_efec;
const CLEARED_BLOCK_0126EFD8: u32 = 0x0126_efd8;

/// The byte stores `00e6b990` makes into the table at `0126efec` (offset, value), in the
/// exe's order.
const BIT_TABLE_STORES: [(u32, u8); 46] = [
    (0x00, 0x18),
    (0x01, 0x20),
    (0x02, 0x20),
    (0x03, 0x10),
    (0x04, 0x10),
    (0x05, 0x10),
    (0x06, 0x10),
    (0x07, 0x08),
    (0x08, 0x08),
    (0x09, 0x10),
    (0x0a, 0x10),
    (0x0b, 0x20),
    (0x0c, 0x20),
    (0x0d, 0x20),
    (0x0e, 0x20),
    (0x0f, 0x20),
    (0x10, 0x40),
    (0x14, 0x10),
    (0x15, 0x08),
    (0x1e, 0x08),
    (0x1f, 0x10),
    (0x20, 0x08),
    (0x28, 0x10),
    (0x29, 0x10),
    (0x2a, 0x20),
    (0x2b, 0x20),
    (0x2c, 0x20),
    (0x2f, 0x20),
    (0x32, 0x10),
    (0x33, 0x20),
    (0x35, 0x10),
    (0x37, 0x20),
    (0x39, 0x20),
    (0x3b, 0x20),
    (0x3c, 0x10),
    (0x3e, 0x20),
    (0x3f, 0x20),
    (0x3d, 0x10),
    (0x51, 0x10),
    (0x52, 0x20),
    (0x5a, 0x40),
    (0x5b, 0x10),
    (0x5c, 0x20),
    (0x5d, 0x40),
    (0x5e, 0x20),
    (0x5f, 0x40),
];
/// The last two stores of the table (`0126f04c`, `0126f04d`).
const BIT_TABLE_LAST: [(u32, u8); 2] = [(0x60, 0x80), (0x61, 0x10)];

// Translated from 00e6b990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`): the renderer's field set-up the
/// constructor ends with, `thiscall`, no stack words. Clears the device (+0x288) and the
/// 0x130 caps bytes (+0x28c, `memset`), sets many fields to 0, 1 or their defaults (+0x5e4
/// 1.0, +0x5e0 0xff808080, +0x5c8 1, the six words at `011a946c` twice from +0x634, the
/// `float`s at `0101e2bc` and `01016410` at +0x684 and +0x688, an identity matrix of 0x40
/// bytes at +0x68c, the four 0x58-byte blocks from +0x700 cleared with the words +0x860 to
/// +0x86c and the objects at +0x870 to +0x87c released, 0.0 and 1.0 at +0x94c to +0x97c,
/// +0x880 0x16), clears the 0x76 bytes at `0126efd8` (`memset`) and fills the byte table at
/// `0126efec`.
pub fn fn_00e6b990(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let set = |e: &mut Engine, offset: u32, value: u32| e.mem.set_u32(base + offset, value);
    let set_byte = |e: &mut Engine, offset: u32, value: u8| e.mem.set_u8(base + offset, value);
    let one = x87_store32(1.0);
    let zero = x87_store32(0.0);
    set(e, 0x288, 0);
    e.call(MEMSET_ADDR, &args![base + 0x28c, 0u32, 0x130u32]);
    set(e, 0x5e4, one);
    set(e, 0x3bc, 0);
    set_byte(e, 0x3c4, 0);
    for offset in [0x5c4u32, 0x3c0, 0x5cc, 0x5d0] {
        set(e, offset, 0);
    }
    set_byte(e, 0x5d4, 0);
    set_byte(e, 0x5d5, 0);
    for offset in [0x6e4u32, 0x6e8, 0x6ec, 0x6f0] {
        set(e, offset, 0);
    }
    set_byte(e, 0x6f4, 0);
    set_byte(e, 0x6f5, 0);
    set(e, 0x8a0, 0);
    set_byte(e, 0x8a4, 0);
    set_byte(e, 0x8a5, 0);
    set(e, 0x6f8, 0);
    set_byte(e, 0x6fc, 0);
    set(e, 0x5e0, 0xff80_8080);
    set(e, 0x5e8, 0);
    set(e, 0x5ec, 0);
    set_byte(e, 0x5f0, 0);
    set(e, 0x5c8, 1);
    for copy in [0x634u32, 0x64c] {
        for word in 0..6 {
            let value = e.mem.u32(DEFAULT_WORDS_011A946C + 4 * word);
            set(e, copy + 4 * word, value);
        }
    }
    let near = x87_quiet_bits(e.mem.u32(FRUSTUM_NEAR_0101E2BC));
    set(e, 0x684, near);
    let second = x87_quiet_bits(e.mem.u32(FLOAT_SOURCE_01016410));
    set(e, 0x688, second);
    e.call(MEMSET_ADDR, &args![base + 0x68c, 0u32, 0x40u32]);
    for offset in [0x6c8u32, 0x6b4, 0x6a0, 0x68c] {
        set(e, offset, one);
    }
    set(e, 0x620, 0);
    set(e, 0x624, 0);
    set(e, 0x6d4, 1);
    set(e, 0x6dc, zero);
    set(e, 0x6d8, 1);
    set(e, 0x6cc, 0);
    set(e, 0x6e0, one);
    set(e, 0x6d0, 0);
    for block in 0..4u32 {
        let start = 0x700 + 0x58 * block;
        for word in 0..0x16 {
            set(e, start + 4 * word, 0);
        }
        let slot = 0x870 + 4 * block;
        set(e, slot - 0x10, 0);
        let held = e.mem.u32(base + slot);
        if held != 0 {
            release_object(e, held);
            set(e, slot, 0);
        }
    }
    for offset in [0x94cu32, 0x95c, 0x96c] {
        set(e, offset, zero);
    }
    set(e, 0x880, 0x16);
    set(e, 0x97c, one);
    for offset in [
        0x888u32, 0x88c, 0x8bc, 0x8c0, 0x8c4, 0x8b8, 0x8c8, 0x8ac, 0x8b0, 0x8b4, 0xa80, 0xa84,
        0xa88, 0xa90,
    ] {
        set(e, offset, 0);
    }
    e.mem.set_u16(base + 0xa8c, 0);
    for offset in [0xa94u32, 0x280, 0x284] {
        set(e, offset, 0);
    }
    e.call(MEMSET_ADDR, &args![CLEARED_BLOCK_0126EFD8, 0u32, 0x76u32]);
    for (offset, value) in BIT_TABLE_STORES.iter().chain(BIT_TABLE_LAST.iter()) {
        e.mem.set_u8(BIT_TABLE_0126EFEC + offset, *value);
    }
}

/// What the present parameters' set-up calls: the frame-buffer format chooser (`00e7aed0`,
/// unnamed; `thiscall` on the device description: windowed byte, bit depth), the format for an
/// index (`004dcbe0`, `renderer.cpp`; `cdecl`), the depth/stencil chooser (`00e7b0f0`, unnamed;
/// `thiscall` on the device description: format, back buffer format, bit depth, stencil
/// flag), the multisample type of a mode word (`00b6c200`, unnamed; `cdecl`), the presentation
/// interval of a word (`00e69e50`, unnamed; `cdecl`), the full-screen mode matcher (`00e7b190`,
/// unnamed; `thiscall` on the adapter description: format, width, height, the refresh word)
/// and `IDirect3D9::CheckDeviceMultiSampleType` (slot 0x2c by the interface's order).
const FRAMEBUFFER_FORMAT_00E7AED0: u32 = 0x00e7_aed0;
const FORMAT_FOR_INDEX_004DCBE0: u32 = 0x004d_cbe0;
const DEPTH_FORMAT_00E7B0F0: u32 = 0x00e7_b0f0;
const MULTISAMPLE_00B6C200: u32 = 0x00b6_c200;
const PRESENT_INTERVAL_00E69E50: u32 = 0x00e6_9e50;
const MATCH_MODE_00E7B190: u32 = 0x00e7_b190;
const CHECK_DEVICE_MULTI_SAMPLE_TYPE: u32 = 0x2c;
const NO_FRAMEBUFFER_FORMAT_010ED800: u32 = 0x010e_d800;
const NO_FRAMEBUFFER_FORMAT_REPORT_010ED7B8: u32 = 0x010e_d7b8;
const NO_DEPTH_FORMAT_010ED778: u32 = 0x010e_d778;
const NO_DEPTH_FORMAT_REPORT_010ED730: u32 = 0x010e_d730;
const NO_ANTIALIASING_010ED6D8: u32 = 0x010e_d6d8;
const NO_FULLSCREEN_MODE_010ED69C: u32 = 0x010e_d69c;
const NO_FULLSCREEN_MODE_REPORT_010ED650: u32 = 0x010e_d650;
const CLOSE_REFRESH_RATE_010ED5F0: u32 = 0x010e_d5f0;

// Translated from 00e6bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`): the present parameters' set-up,
/// `thiscall` with 12 words (the window, width, height, flags, a mode word, the requested
/// frame-buffer format, the depth/stencil format, the back buffer count, the swap word, the
/// refresh rate, the interval word, the present parameters); answers a byte.
///
/// Flags bit 2 clear means windowed, bit 4 set a 16-bit depth (else 32). Without a requested
/// format `00e7aed0` chooses one ("No matching framebuffer format available" when none). The
/// format of that index (`004dcbe0`) is the back buffer format; without a depth/stencil format
/// `00e7b0f0` chooses one (with flags bit 3; "No matching depth/stencil format available").
/// The 0x38 bytes are cleared and filled: width, height, format, back buffer count, the
/// multisample type of the mode word (`00b6c200`; its quality the mode word's low 31 bits for
/// type 1, else 0), the swap effect (for mode words 0 and 1: 1, 2 or 3 from the swap word 1,
/// 2, 3, else 1), the window, windowed, automatic depth-stencil (flags bit 0 clear), the
/// depth/stencil format, the lockable-back-buffer flag (mode word 1), the refresh rate and the
/// interval (`00e69e50`). `CheckDeviceMultiSampleType` failing reports "Could not support
/// requested antialiasing mode" and clears the type; otherwise the quality is capped to the
/// levels - 1. The count is kept within 1 to 3. Windowed clears the refresh rate; full screen
/// matches the mode (`00e7b190`; failing: "Could not match desired fullscreen mode") and
/// reports a changed refresh rate. Windowed intervals other than 0, 1 and 0x80000000 become
/// 0x80000000; full-screen intervals the device does not support (its caps word +0x18 of the
/// device description) become 1. Answers 1.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e6bf50(
    e: &mut Engine,
    this: Ptr,
    window: u32,
    width: u32,
    height: u32,
    flags: u32,
    mode: u32,
    framebuffer: u32,
    depth_stencil: u32,
    back_buffers: u32,
    swap: u32,
    refresh_rate: u32,
    interval: u32,
    present: Ptr,
) -> u8 {
    let this = this.addr();
    let windowed = !((flags >> 2) as u8) & 1;
    let device_desc = e.mem.u32(this + 0x5dc);
    let bit_depth = if flags as u8 & 0x10 != 0 {
        0x10u32
    } else {
        0x20
    };
    let mut framebuffer = framebuffer;
    if framebuffer == 0 {
        framebuffer = e
            .call(
                FRAMEBUFFER_FORMAT_00E7AED0,
                &args![device_desc, u32::from(windowed), bit_depth],
            )
            .u32();
        if framebuffer == 0 {
            e.call(
                STRNCPY_S,
                &args![
                    ERROR_MESSAGE_011F4648,
                    0x100u32,
                    NO_FRAMEBUFFER_FORMAT_010ED800,
                    0xffu32
                ],
            );
            e.call(
                REPORT_00A29680,
                &args![NO_FRAMEBUFFER_FORMAT_REPORT_010ED7B8],
            );
            return 0;
        }
    }
    let format = e.call(FORMAT_FOR_INDEX_004DCBE0, &args![framebuffer]).u32();
    let mut depth = depth_stencil;
    if depth == 0 {
        let display_format = fn_00e69c90(e, framebuffer);
        let device_desc = e.mem.u32(this + 0x5dc);
        depth = e
            .call(
                DEPTH_FORMAT_00E7B0F0,
                &args![device_desc, display_format, format, bit_depth, flags & 8],
            )
            .u32();
        if depth == 0 {
            e.call(
                STRNCPY_S,
                &args![
                    ERROR_MESSAGE_011F4648,
                    0x100u32,
                    NO_DEPTH_FORMAT_010ED778,
                    0xffu32
                ],
            );
            e.call(REPORT_00A29680, &args![NO_DEPTH_FORMAT_REPORT_010ED730]);
            return 0;
        }
    }
    let p = present.addr();
    e.call(MEMSET_ADDR, &args![p, 0u32, 0x38u32]);
    e.mem.set_u32(p + 8, format);
    e.mem.set_u32(p, width);
    e.mem.set_u32(p + 4, height);
    let multisample = e.call(MULTISAMPLE_00B6C200, &args![mode]).u32();
    e.mem.set_u32(p + 0x10, multisample);
    let quality = if multisample == 1 {
        mode & 0x7fff_ffff
    } else {
        0
    };
    e.mem.set_u32(p + 0x14, quality);
    e.mem.set_u32(p + 0x28, depth);
    e.mem.set_u32(p + 0xc, back_buffers);
    e.mem.set_u32(p + 0x24, !flags & 1);
    e.mem.set_u32(p + 0x20, u32::from(windowed));
    let swap_effect = if mode == 0 || mode == 1 {
        match swap {
            2 => 2,
            3 => 3,
            _ => 1,
        }
    } else {
        1
    };
    e.mem.set_u32(p + 0x18, swap_effect);
    e.mem.set_u32(p + 0x30, refresh_rate);
    e.mem.set_u32(p + 0x1c, window);
    e.mem.set_u32(p + 0x2c, u32::from(mode == 1));
    let presentation = e.call(PRESENT_INTERVAL_00E69E50, &args![interval]).u32();
    e.mem.set_u32(p + 0x34, presentation);
    let checked = e.with_stack(4, |e, levels| {
        let direct3d = e.mem.u32(DIRECT3D);
        let adapter = e.mem.u32(this + ADAPTER);
        let device_type = e.mem.u32(this + DEVICE_TYPE);
        let result = e
            .vcall(
                direct3d,
                CHECK_DEVICE_MULTI_SAMPLE_TYPE,
                &args![
                    adapter,
                    device_type,
                    format,
                    u32::from(windowed),
                    multisample,
                    levels
                ],
            )
            .i32();
        (result, e.mem.u32(levels.addr()))
    });
    if checked.0 < 0 {
        e.call(REPORT_00A29680, &args![NO_ANTIALIASING_010ED6D8]);
        e.mem.set_u32(p + 0x10, 0);
    } else if checked.1 <= e.mem.u32(p + 0x14) {
        e.mem.set_u32(p + 0x14, checked.1.wrapping_sub(1));
    }
    let count = e.mem.u32(p + 0xc);
    if count == 0 {
        e.mem.set_u32(p + 0xc, 1);
    } else if count > 3 {
        e.mem.set_u32(p + 0xc, 3);
    }
    let requested_rate = e.mem.u32(p + 0x30);
    if e.mem.u32(p + 0x20) != 0 {
        e.mem.set_u32(p + 0x30, 0);
    } else {
        let format_word = e.mem.u32(p + 8);
        let index = fn_00e69d00(e, format_word);
        let display_format = fn_00e69c90(e, index);
        let adapter_desc = e.mem.u32(this + 0x5d8);
        let width = e.mem.u32(p);
        let height = e.mem.u32(p + 4);
        let matched = e
            .call(
                MATCH_MODE_00E7B190,
                &args![adapter_desc, display_format, width, height, p + 0x30],
            )
            .u32();
        if matched == 0 {
            e.call(SET_ERROR_00E69AD0, &args![NO_FULLSCREEN_MODE_010ED69C]);
            e.call(REPORT_00A29680, &args![NO_FULLSCREEN_MODE_REPORT_010ED650]);
            return 0;
        }
        if requested_rate != e.mem.u32(p + 0x30) {
            e.call(REPORT_00A29680, &args![CLOSE_REFRESH_RATE_010ED5F0]);
        }
    }
    let interval_word = e.mem.u32(p + 0x34);
    if e.mem.u32(p + 0x20) != 0 {
        if interval_word != 0 && interval_word != 0x8000_0000 && interval_word != 1 {
            e.mem.set_u32(p + 0x34, 0x8000_0000);
        }
    } else {
        let supported = e.mem.u32(device_desc.wrapping_add(0x18));
        if interval_word & supported == 0 {
            e.mem.set_u32(p + 0x34, 1);
        }
    }
    1
}

/// The caps fields, renderer fields and globals `00e6c240` fills, and its set-up calls
/// (`00e7f830`, `00e7f840`, `00e7f850`, unnamed; `cdecl`, one word).
const CAPS_SET_UP_00E7F830: u32 = 0x00e7_f830;
const CAPS_SET_UP_00E7F840: u32 = 0x00e7_f840;
const CAPS_SET_UP_00E7F850: u32 = 0x00e7_f850;
const NO_DEVICE_CAPS_010ED924: u32 = 0x010e_d924;
const NO_DEVICE_CAPS_REPORT_010ED8E0: u32 = 0x010e_d8e0;
const NO_MULTI_TEXTURE_010ED898: u32 = 0x010e_d898;
const NO_MULTI_TEXTURE_REPORT_010ED840: u32 = 0x010e_d840;
/// `IDirect3DDevice9::GetDeviceCaps` (slot 0x1c by the interface's order).
const GET_DEVICE_CAPS_SLOT: u32 = 0x1c;

// Translated from 00e6c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `thiscall` with one word it does not
/// read; answers a byte. The device's caps go to +0x28c (`GetDeviceCaps`; failing: "Could not
/// query device caps"). Fewer than two texture blend stages or simultaneous textures (+0x320,
/// +0x324) fail ("Device does not support single-pass multi-texture"). Otherwise the renderer
/// takes its limits and flags from the caps (+0x5ec from caps bits 1 and 8, +0x6e4 the
/// stream count or 4, +0x6e8 to +0x6f5, +0x8a0 to +0x8a5, the vertex and pixel shader
/// version words at +0x918 to +0x93c), passes the stage counts to `00e7f830`, `00e7f840` and
/// `00e7f850` (0x10 for pixel shader 2.0 and above), and sets the render-state defaults at
/// `0126f050` to `0126f0e4` (their values the exe's constants and caps bits 10 and 26).
pub fn fn_00e6c240(e: &mut Engine, this: Ptr, _unused_1: u32) -> u8 {
    let this = this.addr();
    let device = e.mem.u32(this + DEVICE);
    let caps = this + 0x28c;
    if e.vcall(device, GET_DEVICE_CAPS_SLOT, &args![caps]).i32() < 0 {
        e.call(
            STRNCPY_S,
            &args![
                ERROR_MESSAGE_011F4648,
                0x100u32,
                NO_DEVICE_CAPS_010ED924,
                0xffu32
            ],
        );
        e.call(REPORT_00A29680, &args![NO_DEVICE_CAPS_REPORT_010ED8E0]);
        return 0;
    }
    let field = |e: &Engine, offset: u32| e.mem.u32(this + offset);
    let stages = field(e, 0x320);
    if stages < 2 || field(e, 0x324) < 2 {
        e.call(
            STRNCPY_S,
            &args![
                ERROR_MESSAGE_011F4648,
                0x100u32,
                NO_MULTI_TEXTURE_010ED898,
                0xffu32
            ],
        );
        e.call(REPORT_00A29680, &args![NO_MULTI_TEXTURE_REPORT_010ED840]);
        return 0;
    }
    let set = |e: &mut Engine, offset: u32, value: u32| e.mem.set_u32(this + offset, value);
    let set_byte = |e: &mut Engine, offset: u32, value: u8| e.mem.set_u8(this + offset, value);
    let texture_caps = field(e, 0x2c8);
    set(e, 0x5ec, 0);
    if texture_caps & 2 == 0 {
        set(e, 0x5ec, 8);
    } else if texture_caps & 0x100 != 0 {
        set(e, 0x5ec, 4);
    }
    let streams = field(e, 0x334);
    set(e, 0x6e4, streams);
    if e.mem.u8(this + 0x5d4) != 0 && e.mem.u8(this + 0x5d5) == 0 {
        set(e, 0x6e4, 4);
    }
    if field(e, 0x6e4) >= 4 {
        let value = field(e, 0x5ec) | 2;
        set(e, 0x5ec, value);
    }
    let raster = field(e, 0x2ac);
    let vertex_version = field(e, 0x348);
    let pixel_limit = field(e, 0x350);
    set_byte(e, 0x6f4, ((texture_caps >> 16) as u8) & 1);
    let primitive = field(e, 0x298);
    set_byte(e, 0x6f5, ((primitive >> 29) as u8) & 1);
    let max_vertex = field(e, 0x37c);
    set(e, 0x8a0, max_vertex);
    set_byte(e, 0x8a4, ((raster >> 18) as u8) & 1);
    set_byte(e, 0x8a5, ((raster >> 19) as u8) & 1);
    set(e, 0x6e8, vertex_version);
    let pixel_version = field(e, 0x358);
    set(e, 0x6ec, pixel_version);
    set(e, 0x6f0, pixel_limit);
    set(e, 0x918, pixel_version);
    set(e, 0x91c, 0xffff_0000);
    set(e, 0x920, pixel_version);
    set(e, 0x924, pixel_limit);
    set(e, 0x928, 0xfffe_0000);
    set(e, 0x92c, pixel_limit);
    for offset in [0x930u32, 0x934, 0x938] {
        set(e, offset, 0xfffe_0000);
    }
    set(e, 0x93c, 2);
    e.call(CAPS_SET_UP_00E7F830, &args![stages]);
    let textures = field(e, 0x324);
    e.call(CAPS_SET_UP_00E7F840, &args![textures]);
    let samplers = if field(e, 0x6ec) >= 0xffff_0200 {
        0x10
    } else {
        field(e, 0x320)
    };
    e.call(CAPS_SET_UP_00E7F850, &args![samplers]);
    let vertex_caps = field(e, 0x2cc);
    let bit_10 = ((vertex_caps >> 10) as u8) & 1;
    e.mem.set_u8(0x0126_f0d1, bit_10);
    let bit_26 = ((field(e, 0x2cc) >> 26) as u8) & 1;
    for global in [0x0126_f058u32, 0x0126_f064, 0x0126_f0ac] {
        e.mem.set_u32(global, 0);
    }
    e.mem.set_u8(0x0126_f0d2, bit_26);
    for global in [
        0x0126_f0c4u32,
        0x0126_f0b8,
        0x0126_f0b0,
        0x0126_f0b4,
        0x0126_f0a4,
    ] {
        e.mem.set_u32(global, 3);
    }
    e.mem.set_u32(0x0126_f09c, 2 + u32::from(bit_26 != 0));
    for global in [
        0x0126_f0c8u32,
        0x0126_f0cc,
        0x0126_f0c0,
        0x0126_f0bc,
        0x0126_f054,
        0x0126_f050,
        0x0126_f078,
        0x0126_f074,
        0x0126_f07c,
        0x0126_f084,
        0x0126_f080,
        0x0126_f094,
    ] {
        e.mem.set_u32(global, 1);
    }
    for global in [
        0x0126_f060u32,
        0x0126_f05c,
        0x0126_f06c,
        0x0126_f068,
        0x0126_f070,
        0x0126_f088,
        0x0126_f090,
        0x0126_f08c,
        0x0126_f0a8,
        0x0126_f0a0,
    ] {
        e.mem.set_u32(global, 2);
    }
    e.mem.set_u32(0x0126_f098, 2 + u32::from(bit_10 != 0));
    let filter = if field(e, 0x2c8) & 0x400 != 0 {
        0x101
    } else {
        0
    };
    e.mem.set_u32(0x0126_f0e4, filter + 2);
    1
}

/// What `00e6ec90` uses: the sampler set-up `00e6dfc0` (unnamed; `thiscall`: block, a byte, a
/// count), `00e6c550` (unnamed; `thiscall`, no stack words), the pixel data constructor
/// `00a7c190` (unnamed; `thiscall`: width, height, pixel format, mipmap levels, faces), the
/// fixed-string maker `00a5b690`, the source texture maker `00a5ff10` (unnamed; `cdecl`: pixel
/// data, name, format preferences), the empty fixed string (`0109b220`) and the texture's
/// name `"Clipper Image"` (`010edbb8`).
const SAMPLER_SET_UP_00E6DFC0: u32 = 0x00e6_dfc0;
const SET_UP_00E6C550: u32 = 0x00e6_c550;
const PIXEL_DATA_CONSTRUCT_00A7C190: u32 = 0x00a7_c190;
const FIXED_STRING_MAKE_00A5B690: u32 = 0x00a5_b690;
const SOURCE_TEXTURE_00A5FF10: u32 = 0x00a5_ff10;
const EMPTY_FIXED_STRING_0109B220: u32 = 0x0109_b220;
const CLIPPER_IMAGE_010EDBB8: u32 = 0x010e_dbb8;

// Translated from 00e6ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`; the initialisation's "texture
/// defaults"), `thiscall`, no stack words; answers 1. Sets up the four sampler blocks (+0x700
/// with 0 and 3, +0x758 with 1 and 3, +0x7b0 with 0 and 5, +0x808 with 1 and 5, `00e6dfc0`),
/// runs `00e6c550`, makes a 2 by 1 pixel data (0x74 bytes, `00a7c190` with the pixel format at
/// +0x860; counted) and writes its pixels by the format's bit count (byte +1): 16 bits: bytes
/// 3 and 2 0xff, 0 and 1 0; 32 bits: bytes 3 and 2 0, 7 to 4 0xff, 0 and 1 0; others nothing.
/// The texture "Clipper Image" made from it (`00a5ff10`, preferences 6, 0, 0) is held at
/// +0x8fc (`NiPointer` assignment); the name and the pixel data are released.
pub fn fn_00e6ec90(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    for (offset, first, count) in [
        (0x700u32, 0u32, 3u32),
        (0x758, 1, 3),
        (0x7b0, 0, 5),
        (0x808, 1, 5),
    ] {
        e.call(
            SAMPLER_SET_UP_00E6DFC0,
            &args![base, base + offset, first, count],
        );
    }
    e.call(SET_UP_00E6C550, &args![base]);
    let format = e.mem.u32(base + 0x860);
    let block = e.call(NEW_OBJECT, &args![0x74u32]).u32();
    let pixels = if block != 0 {
        let made = e
            .call(
                PIXEL_DATA_CONSTRUCT_00A7C190,
                &args![block, 2u32, 1u32, format, 1u32, 1u32],
            )
            .u32();
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_UP, &args![made + 4]);
        }
        made
    } else {
        0
    };
    let data = e.mem.u32(pixels.wrapping_add(0x5c));
    let first = e
        .mem
        .u32(data)
        .wrapping_add(e.mem.u32(pixels.wrapping_add(0x50)));
    let bits = e.mem.u8(format.wrapping_add(1));
    if bits == 0x10 {
        e.mem.set_u8(first + 3, 0xff);
        e.mem.set_u8(first + 2, 0xff);
        e.mem.set_u8(first, 0);
        e.mem.set_u8(first + 1, 0);
    } else if bits == 0x20 {
        e.mem.set_u8(first + 3, 0);
        e.mem.set_u8(first + 2, 0);
        for offset in [7u32, 6, 5, 4] {
            e.mem.set_u8(first + offset, 0xff);
        }
        e.mem.set_u8(first, 0);
        e.mem.set_u8(first + 1, 0);
    }
    // The format preferences (three words) and the name's stack word.
    let texture = e.with_stack(0x10, |e, frame| {
        let preferences = frame.addr();
        let name = frame.addr() + 0xc;
        e.mem.set_u32(preferences, 6);
        e.mem.set_u32(preferences + 8, 0);
        e.mem.set_u32(preferences + 4, 0);
        let handle = e
            .call(FIXED_STRING_MAKE_00A5B690, &args![CLIPPER_IMAGE_010EDBB8])
            .u32();
        e.mem.set_u32(name, handle);
        let texture = e
            .call(SOURCE_TEXTURE_00A5FF10, &args![pixels, handle, preferences])
            .u32();
        (texture, e.mem.u32(name))
    });
    let (texture, handle) = texture;
    let held = e.mem.u32(base + 0x8fc);
    if held != texture {
        if held != 0 {
            release_object(e, held);
        }
        e.mem.set_u32(base + 0x8fc, texture);
        if texture != 0 {
            e.call(IMPORT_INTERLOCKED_UP, &args![texture + 4]);
        }
    }
    if handle != e.mem.u32(EMPTY_FIXED_STRING_0109B220) {
        e.call(IMPORT_INTERLOCKED_DOWN, &args![handle.wrapping_sub(8)]);
    }
    release_object(e, pixels);
    1
}

/// The registry entry of [`fn_00e6bf50`]: `this` and its 12 words.
fn fn_00e6bf50_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let w = |i: usize| words[i];
    u32::from(fn_00e6bf50(
        e,
        Ptr::new(w(0)),
        w(1),
        w(2),
        w(3),
        w(4),
        w(5),
        w(6),
        w(7),
        w(8),
        w(9),
        w(10),
        w(11),
        Ptr::new(w(12)),
    ))
    .into_ret()
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    const OBJECT_VTABLE: u32 = 0x6000;
    const SLOTS: u32 = 0x6100_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with a virtual table whose slot at byte offset `n` is `SLOTS + n`, the
    /// interlocked imports doubled (the decrement answering `count_after_release`).
    fn engine(count_after_release: u32) -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x80).map(|i| SLOTS + 4 * i).collect();
        e.put_vtable(OBJECT_VTABLE, &slots);
        e.register(IMPORT_INTERLOCKED_UP, |_, _| Ret::default());
        e.register_double(IMPORT_INTERLOCKED_DOWN, move |_, _| {
            count_after_release.into_ret()
        });
        e.register(SLOTS + 4, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, OBJECT_VTABLE);
        object
    }

    #[test]
    fn fn_00e69510_maps_device_kinds() {
        let mut e = Engine::new();
        let device_type = e.mem.alloc(4);
        let behavior = e.mem.alloc(4);
        let expected = [
            (1, 0x50),
            (1, 0x40),
            (1, 0x80),
            (1, 0x20),
            (2, 0x20),
            (2, 0x40),
            (2, 0x80),
        ];
        for (kind, (device, flag)) in expected.iter().enumerate() {
            e.mem.set_u32(behavior, 0xff);
            let ok = e
                .call(0x00e6_9510, &args![kind as u32, device_type, behavior])
                .u8();
            assert_eq!(ok, 1);
            assert_eq!(e.mem.u32(device_type), *device);
            assert_eq!(e.mem.u32(behavior), 0x0f | flag);
        }
        e.mem.set_u32(behavior, 0xff);
        assert_eq!(
            e.call(0x00e6_9510, &args![7u32, device_type, behavior])
                .u8(),
            0
        );
        assert_eq!(e.mem.u32(behavior), 0x0f);
    }

    #[test]
    fn format_tables() {
        let mut e = Engine::new();
        let mut index = |format: u32| e.call(0x00e6_9d00, &args![format]).u32();
        assert_eq!(index(0x14), 1);
        assert_eq!(index(0x1e), 0xb);
        assert_eq!(index(0x24), 0xe);
        assert_eq!(index(0x6f), 0xc);
        assert_eq!(index(0x70), 0xd);
        assert_eq!(index(0x71), 0xf);
        assert_eq!(index(0x74), 0x12);
        assert_eq!(index(0x75), 0);
        assert_eq!(index(0x23), 0);
        let mut e = Engine::new();
        let mut format = |i: u32| e.call(0x00e6_9c90, &args![i]).u32();
        let values: Vec<u32> = (0..13).map(&mut format).collect();
        assert_eq!(
            values,
            vec![0, 0x14, 0x16, 0x16, 0x17, 0x18, 0x18, 0x1e, 0x1b, 0, 0, 0x1e, 0]
        );
    }

    #[test]
    fn fn_00e6e6e0_replaces_the_held_shader() {
        let mut e = engine(0);
        let this = e.mem.alloc(0x20);
        let old = object(&mut e);
        let new = object(&mut e);
        e.mem.set_u32(this + 0x14, old);
        e.call(0x00e6_e6e0, &args![this, new]);
        assert_eq!(e.mem.u32(this + 0x14), new);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DOWN), vec![vec![old + 4]]);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![old]]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_UP), vec![vec![new + 4]]);
        e.call_log = Some(vec![]);
        e.call(0x00e6_e6e0, &args![this, new]);
        e.call(0x00e6_e6e0, &args![this, 0u32]);
        assert!(e.call_log.as_ref().unwrap().len() == 2);
    }

    #[test]
    fn fn_00e72cf0_sets_new_and_existing_keys() {
        let mut e = engine(1);
        let map = object(&mut e);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map + 8, buckets);
        e.register(SLOTS + 4, |_, a| (a[1] & 1).into_ret());
        e.register(SLOTS + 8, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(SLOTS + 0x14, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(SLOTS + 0xc, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        let value = object(&mut e);
        e.call(0x00e7_2cf0, &args![map, 0x11u32, value]);
        let item = e.mem.u32(buckets + 4);
        assert_eq!(e.mem.u32(item + 4), 0x11);
        assert_eq!(e.mem.u32(item + 8), value);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        // The copy counted, the by-value pointer released.
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_UP), vec![vec![value + 4]]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DOWN), vec![vec![value + 4]]);
        // The same key again with another value replaces it.
        let other = object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00e7_2cf0, &args![map, 0x11u32, other]);
        assert_eq!(e.mem.u32(item + 8), other);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DOWN),
            vec![vec![value + 4], vec![other + 4]]
        );
    }

    #[test]
    fn fn_00e6b990_sets_the_renderer_defaults() {
        let mut e = engine(0);
        e.map(0x0126_e000, 0x2000);
        e.map(0x011a_9000, 0x1000);
        e.map(0x0101_6000, 0x9000);
        e.mem.set_u32(FRUSTUM_NEAR_0101E2BC, 0.1f32.to_bits());
        e.mem.set_u32(FLOAT_SOURCE_01016410, 2.5f32.to_bits());
        for word in 0..6 {
            e.mem
                .set_u32(DEFAULT_WORDS_011A946C + 4 * word, 0x100 + word);
        }
        e.register(MEMSET_ADDR, |_, _| Ret::default());
        let this = e.mem.alloc(0xb80);
        let held = object(&mut e);
        e.mem.set_u32(this + 0x874, held);
        e.call(0x00e6_b990, &args![this]);
        assert_eq!(e.mem.f32(this + 0x5e4), 1.0);
        assert_eq!(e.mem.u32(this + 0x5e0), 0xff80_8080);
        assert_eq!(e.mem.u32(this + 0x5c8), 1);
        assert_eq!(e.mem.u32(this + 0x638), 0x101);
        assert_eq!(e.mem.u32(this + 0x660), 0x105);
        assert_eq!(e.mem.f32(this + 0x684), 0.1);
        assert_eq!(e.mem.f32(this + 0x688), 2.5);
        assert_eq!(e.mem.f32(this + 0x6b4), 1.0);
        assert_eq!(e.mem.f32(this + 0x6e0), 1.0);
        assert_eq!(e.mem.f32(this + 0x97c), 1.0);
        assert_eq!(e.mem.u32(this + 0x880), 0x16);
        assert_eq!(e.mem.u32(this + 0x874), 0);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![held]]);
        assert_eq!(e.mem.u8(BIT_TABLE_0126EFEC), 0x18);
        assert_eq!(e.mem.u8(BIT_TABLE_0126EFEC + 0x60), 0x80);
        assert_eq!(e.mem.u8(BIT_TABLE_0126EFEC + 0x61), 0x10);
        assert!(calls(&e, MEMSET_ADDR).contains(&vec![CLEARED_BLOCK_0126EFD8, 0, 0x76]));
    }

    /// A renderer for the present parameters' set-up: Direct3D at `0126f0d8` (slot 0x2c
    /// answering `levels` quality levels), a device description whose interval caps are 1,
    /// and the callees doubled.
    fn present_rig(levels: u32, mode_found: bool) -> (Engine, u32, u32) {
        let mut e = engine(1);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        let direct3d = object(&mut e);
        e.mem.set_u32(DIRECT3D, direct3d);
        e.register_double(SLOTS + CHECK_DEVICE_MULTI_SAMPLE_TYPE, move |e, a| {
            e.mem.set_u32(a[6], levels);
            (if levels == 0 { 0x8876_086cu32 } else { 0 }).into_ret()
        });
        let this = e.mem.alloc(0xb80);
        let device_desc = e.mem.alloc(0x144);
        e.mem.set_u32(device_desc + 0x18, 1);
        e.mem.set_u32(this + 0x5dc, device_desc);
        e.mem.set_u32(this + ADAPTER, 0);
        e.mem.set_u32(this + DEVICE_TYPE, 1);
        e.register(FRAMEBUFFER_FORMAT_00E7AED0, |_, _| 3u32.into_ret());
        e.register(FORMAT_FOR_INDEX_004DCBE0, |_, _| 0x16u32.into_ret());
        e.register(DEPTH_FORMAT_00E7B0F0, |_, _| 0x4bu32.into_ret());
        e.register(MULTISAMPLE_00B6C200, |_, a| a[0].into_ret());
        e.register(PRESENT_INTERVAL_00E69E50, |_, a| a[0].into_ret());
        e.register_double(MATCH_MODE_00E7B190, move |e, a| {
            e.mem.set_u32(a[4], 75);
            u32::from(mode_found).into_ret()
        });
        for no_op in [MEMSET_ADDR, REPORT_00A29680, STRNCPY_S, SET_ERROR_00E69AD0] {
            e.register(no_op, |_, _| Ret::default());
        }
        (e, this, device_desc)
    }

    fn set_up(e: &mut Engine, this: u32, flags: u32, mode: u32, interval: u32) -> (u8, Vec<u32>) {
        let present = e.mem.alloc(0x38);
        let ok = e
            .call(
                0x00e6_bf50,
                &args![
                    this, 0x99u32, 640u32, 480u32, flags, mode, 0u32, 0u32, 5u32, 2u32, 60u32,
                    interval, present
                ],
            )
            .u8();
        (ok, (0..14).map(|i| e.mem.u32(present + 4 * i)).collect())
    }

    #[test]
    fn fn_00e6bf50_fills_windowed_present_parameters() {
        let (mut e, this, _) = present_rig(4, true);
        let (ok, p) = set_up(&mut e, this, 0, 1, 7);
        assert_eq!(ok, 1);
        // width, height, format, count (capped 3), type 1, quality min(1, 4 - 1), swap 2,
        // window, windowed, auto depth, depth, lockable, refresh 0, interval 0x80000000.
        assert_eq!(
            p,
            vec![
                640,
                480,
                0x16,
                3,
                1,
                1,
                2,
                0x99,
                1,
                1,
                0x4b,
                1,
                0,
                0x8000_0000
            ]
        );
        assert_eq!(calls(&e, FRAMEBUFFER_FORMAT_00E7AED0)[0][1..], [1, 0x20]);
        assert!(calls(&e, MATCH_MODE_00E7B190).is_empty());
    }

    #[test]
    fn fn_00e6bf50_matches_the_full_screen_mode() {
        let (mut e, this, _) = present_rig(0, true);
        // Flags 4 (full screen) | 0x10 (16-bit) | 1 (no auto depth); mode word 2.
        let (ok, p) = set_up(&mut e, this, 0x15, 2, 4);
        assert_eq!(ok, 1);
        // Type 2 (refused, cleared), quality 0, swap 1, refresh matched to 75, interval
        // unsupported (caps 1) -> 1.
        assert_eq!(
            p,
            vec![640, 480, 0x16, 3, 0, 0, 1, 0x99, 0, 0, 0x4b, 0, 75, 1]
        );
        assert!(calls(&e, REPORT_00A29680).contains(&vec![NO_ANTIALIASING_010ED6D8]));
        assert!(calls(&e, REPORT_00A29680).contains(&vec![CLOSE_REFRESH_RATE_010ED5F0]));
        assert_eq!(calls(&e, FRAMEBUFFER_FORMAT_00E7AED0)[0][1..], [0, 0x10]);
        let (mut e, this, _) = present_rig(4, false);
        assert_eq!(set_up(&mut e, this, 4, 0, 1).0, 0);
        assert_eq!(
            calls(&e, SET_ERROR_00E69AD0),
            vec![vec![NO_FULLSCREEN_MODE_010ED69C]]
        );
    }

    #[test]
    fn fn_00e6bf50_refuses_without_formats() {
        let (mut e, this, _) = present_rig(4, true);
        e.register(FRAMEBUFFER_FORMAT_00E7AED0, |_, _| 0u32.into_ret());
        assert_eq!(set_up(&mut e, this, 0, 0, 0).0, 0);
        assert_eq!(calls(&e, STRNCPY_S)[0][2], NO_FRAMEBUFFER_FORMAT_010ED800);
        let (mut e, this, _) = present_rig(4, true);
        e.register(DEPTH_FORMAT_00E7B0F0, |_, _| 0u32.into_ret());
        assert_eq!(set_up(&mut e, this, 8, 0, 0).0, 0);
        assert_eq!(calls(&e, STRNCPY_S)[0][2], NO_DEPTH_FORMAT_010ED778);
        assert_eq!(
            calls(&e, DEPTH_FORMAT_00E7B0F0)[0][1..],
            [0x16, 0x16, 0x20, 8]
        );
    }

    #[test]
    fn fn_00e6c240_takes_the_device_caps() {
        let mut e = engine(1);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        let device = object(&mut e);
        e.register(SLOTS + GET_DEVICE_CAPS_SLOT, |e, a| {
            let caps = a[1] - 0x28c;
            e.mem.set_u32(caps + 0x320, 8);
            e.mem.set_u32(caps + 0x324, 8);
            e.mem.set_u32(caps + 0x2c8, 0x1_0102);
            e.mem.set_u32(caps + 0x2cc, 0x400_0400);
            e.mem.set_u32(caps + 0x334, 16);
            e.mem.set_u32(caps + 0x358, 0xffff_0300);
            0u32.into_ret()
        });
        for no_op in [
            CAPS_SET_UP_00E7F830,
            CAPS_SET_UP_00E7F840,
            CAPS_SET_UP_00E7F850,
            STRNCPY_S,
            REPORT_00A29680,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xb80);
        e.mem.set_u32(this + DEVICE, device);
        assert_eq!(e.call(0x00e6_c240, &args![this, 0u32]).u8(), 1);
        // Caps bit 1 set and 0x100 set: 4, then | 2 for 16 streams.
        assert_eq!(e.mem.u32(this + 0x5ec), 6);
        assert_eq!(e.mem.u32(this + 0x6e4), 16);
        assert_eq!(e.mem.u8(this + 0x6f4), 1);
        assert_eq!(e.mem.u32(this + 0x6ec), 0xffff_0300);
        assert_eq!(e.mem.u32(this + 0x93c), 2);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F850), vec![vec![0x10]]);
        assert_eq!(e.mem.u8(0x0126_f0d1), 1);
        assert_eq!(e.mem.u8(0x0126_f0d2), 1);
        assert_eq!(e.mem.u32(0x0126_f09c), 3);
        assert_eq!(e.mem.u32(0x0126_f098), 3);
        assert_eq!(e.mem.u32(0x0126_f0e4), 2);
        assert_eq!(e.mem.u32(0x0126_f0c4), 3);
        assert_eq!(e.mem.u32(0x0126_f050), 1);
        assert_eq!(e.mem.u32(0x0126_f060), 2);
        // One texture stage: refused.
        e.register(SLOTS + GET_DEVICE_CAPS_SLOT, |e, a| {
            e.mem.set_u32(a[1] - 0x28c + 0x320, 1);
            0u32.into_ret()
        });
        assert_eq!(e.call(0x00e6_c240, &args![this, 0u32]).u8(), 0);
        assert!(calls(&e, STRNCPY_S).contains(&vec![
            ERROR_MESSAGE_011F4648,
            0x100,
            NO_MULTI_TEXTURE_010ED898,
            0xff
        ]));
    }

    #[test]
    fn fn_00e6ec90_makes_the_clipper_texture() {
        let mut e = engine(1);
        e.map(EMPTY_FIXED_STRING_0109B220, 4);
        for no_op in [SAMPLER_SET_UP_00E6DFC0, SET_UP_00E6C550] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let pixels_buffer = e.mem.alloc(0x10);
        e.register_double(PIXEL_DATA_CONSTRUCT_00A7C190, move |e, a| {
            e.mem.set_u32(a[0], OBJECT_VTABLE);
            let table = e.mem.alloc(4);
            e.mem.set_u32(table, pixels_buffer);
            e.mem.set_u32(a[0] + 0x5c, table);
            e.mem.set_u32(a[0] + 0x50, 0);
            a[0].into_ret()
        });
        e.register(FIXED_STRING_MAKE_00A5B690, |_, _| 0x7008u32.into_ret());
        let texture = e.mem.alloc(8);
        e.register_double(SOURCE_TEXTURE_00A5FF10, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), 6);
            texture.into_ret()
        });
        let this = e.mem.alloc(0xb80);
        let format = e.mem.alloc(4);
        e.mem.set_u8(format + 1, 0x20);
        e.mem.set_u32(this + 0x860, format);
        assert_eq!(e.call(0x00e6_ec90, &args![this]).u8(), 1);
        assert_eq!(
            e.mem.bytes(pixels_buffer, 8),
            vec![0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff]
        );
        assert_eq!(e.mem.u32(this + 0x8fc), texture);
        let samplers = calls(&e, SAMPLER_SET_UP_00E6DFC0);
        assert_eq!(samplers[3], vec![this, this + 0x808, 1, 5]);
        let pixel_data = calls(&e, PIXEL_DATA_CONSTRUCT_00A7C190);
        assert_eq!(&pixel_data[0][1..], &[2, 1, format, 1, 1]);
        assert_eq!(
            calls(&e, SOURCE_TEXTURE_00A5FF10)[0][..2],
            [pixel_data[0][0], 0x7008]
        );
        // The name, then the pixel data, released.
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DOWN),
            vec![vec![0x7000], vec![pixel_data[0][0] + 4]]
        );
    }
}

// ---- Format names, intervals and the clipper data (batch b000w) ----

/// The "unknown" format string (`01015890`).
const UNKNOWN_FORMAT_01015890: u32 = 0x0101_5890;

// Translated from 00e7a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::GetD3DFormatString` (engine map), `cdecl`, a Direct3D format number: the
/// address of its `"D3DFMT_..."` name. `MET1` and the four-character codes above it (`DXT1`
/// to `DXT5`, `UYVY`, `YUY2`, `RGBG`, `GRGB`) by compare; 0x14 to 0x75 through the switch's
/// byte map at `00e7ac44` and jump table at `00e7ab7c` (read in Ghidra); everything else the
/// string at `01015890`.
pub fn ni_xenon_renderer_get_d3d_format_string(_e: &mut Engine, format: i32) -> u32 {
    if format > 0x3154_454d {
        return match format as u32 {
            0x3154_5844 => 0x010e_e908,
            0x3254_5844 => 0x010e_e914,
            0x3354_5844 => 0x010e_e8f0,
            0x3454_5844 => 0x010e_e8e4,
            0x3554_5844 => 0x010e_e8c4,
            0x3259_5559 => 0x010e_e8fc,
            0x4247_5247 => 0x010e_e8d0,
            0x4742_4752 => 0x010e_e8b0,
            0x5956_5955 => 0x010e_e8a4,
            _ => UNKNOWN_FORMAT_01015890,
        };
    }
    if format == 0x3154_454d {
        return 0x010e_e920;
    }
    match format as u32 {
        0x14 => 0x010e_ec38,
        0x15 => 0x010e_ec28,
        0x16 => 0x010e_ec18,
        0x17 => 0x010e_ec08,
        0x18 => 0x010e_ebf8,
        0x19 => 0x010e_ebe8,
        0x1a => 0x010e_ebd8,
        0x1b => 0x010e_ebc8,
        0x1c => 0x010e_ebbc,
        0x1d => 0x010e_ebac,
        0x1e => 0x010e_eb9c,
        0x1f => 0x010e_eb88,
        0x20 => 0x010e_eb78,
        0x21 => 0x010e_eb68,
        0x22 => 0x010e_eb58,
        0x23 => 0x010e_eb44,
        0x24 => 0x010e_eb30,
        0x28 => 0x010e_eb24,
        0x29 => 0x010e_eb18,
        0x32 => 0x010e_eb0c,
        0x33 => 0x010e_eb00,
        0x34 => 0x010e_eaf4,
        0x3c => 0x010e_eae8,
        0x3d => 0x010e_ead8,
        0x3e => 0x010e_eac8,
        0x3f => 0x010e_eab8,
        0x40 => 0x010e_eaa8,
        0x43 => 0x010e_ea94,
        0x46 => 0x010e_ea80,
        0x47 => 0x010e_ea74,
        0x49 => 0x010e_ea64,
        0x4b => 0x010e_ea54,
        0x4d => 0x010e_ea44,
        0x4f => 0x010e_ea34,
        0x50 => 0x010e_ea28,
        0x51 => 0x010e_e9f4,
        0x52 => 0x010e_ea10,
        0x53 => 0x010e_ea00,
        0x64 => 0x010e_e9e0,
        0x65 => 0x010e_e9d0,
        0x66 => 0x010e_e9c0,
        0x6e => 0x010e_e9ac,
        0x6f => 0x010e_e9a0,
        0x70 => 0x010e_e990,
        0x71 => 0x010e_e978,
        0x72 => 0x010e_e96c,
        0x73 => 0x010e_e95c,
        0x74 => 0x010e_e944,
        0x75 => 0x010e_e934,
        _ => UNKNOWN_FORMAT_01015890,
    }
}

// Translated from 00e69e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `cdecl`, an interval number: the
/// presentation interval flag (the jump table at `00e69e84`, read in Ghidra): 0 gives
/// 0x80000000 (immediate), 1 to 4 give 1, 2, 4 and 8; anything else 0.
pub fn fn_00e69e50(_e: &mut Engine, interval: u32) -> u32 {
    match interval {
        0 => 0x8000_0000,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        _ => 0,
    }
}

/// The pixel format's set-up `00a622d0` (unnamed; `cdecl`); the pixel data constructor is
/// `PIXEL_DATA_CONSTRUCT_00A7C190` above.
const PIXEL_FORMAT_SET_UP_00A622D0: u32 = 0x00a6_22d0;

// Translated from 00e6c550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `thiscall`, no stack words; answers a
/// byte. For each of the four sampler blocks (0x16 words each from +0x700): the first
/// non-null pixel format among its entries 1, 0, 2, 4, 3, 5 is kept at +0x860 + 4 * block;
/// with one, a 4 by 4 pixel data (0x74 bytes, `00a7c190`) is held at +0x870 + 4 * block
/// (`NiPointer` assignment) and its pixels filled with 0xff (`memset` of its size, +0x64, times
/// 16). Then without a pixel format at +0x860 it answers 0; otherwise `00a622d0` runs on it,
/// +0x880 gets its +0x10 and it answers 1.
pub fn fn_00e6c550(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    for block in 0..4u32 {
        let first = 0x16 * block;
        let holder = base + 0x870 + 4 * block;
        let format_at = holder - 0x10;
        e.mem.set_u32(format_at, 0);
        for entry in [1u32, 0, 2, 4, 3, 5] {
            let at = base + 0x700 + 4 * (entry + first);
            if e.mem.u32(at) != 0 {
                let format = e.mem.u32(at);
                e.mem.set_u32(format_at, format);
                break;
            }
        }
        let format = e.mem.u32(format_at);
        if format == 0 {
            continue;
        }
        let allocated = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        let pixels = if allocated != 0 {
            e.call(
                PIXEL_DATA_CONSTRUCT_00A7C190,
                &args![allocated, 4u32, 4u32, format, 1u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        let held = e.mem.u32(holder);
        if held != pixels {
            if held != 0 {
                release_object(e, held);
            }
            e.mem.set_u32(holder, pixels);
            if pixels != 0 {
                e.call(IMPORT_INTERLOCKED_UP, &args![pixels + 4]);
            }
        }
        let data = e.mem.u32(holder);
        let size = e.mem.u32(data.wrapping_add(0x64)) << 4;
        let table = e.mem.u32(data.wrapping_add(0x5c));
        let start = e
            .mem
            .u32(table)
            .wrapping_add(e.mem.u32(data.wrapping_add(0x50)));
        e.call(MEMSET_ADDR, &args![start, 0xffu32, size]);
    }
    let format = e.mem.u32(base + 0x860);
    if format == 0 {
        return 0;
    }
    e.call(PIXEL_FORMAT_SET_UP_00A622D0, &args![format]);
    let format = e.mem.u32(base + 0x860);
    let value = e.mem.u32(format.wrapping_add(0x10));
    e.mem.set_u32(base + 0x880, value);
    1
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn get_d3d_format_string_names_formats() {
        let mut e = Engine::new();
        let mut name = |format: u32| e.call(0x00e7_a9a0, &args![format]).u32();
        assert_eq!(name(0x14), 0x010e_ec38);
        assert_eq!(name(0x16), 0x010e_ec18);
        assert_eq!(name(0x4b), 0x010e_ea54);
        assert_eq!(name(0x75), 0x010e_e934);
        assert_eq!(name(0x25), UNKNOWN_FORMAT_01015890);
        assert_eq!(name(0x76), UNKNOWN_FORMAT_01015890);
        assert_eq!(name(0x13), UNKNOWN_FORMAT_01015890);
        assert_eq!(name(0x3154_454d), 0x010e_e920);
        assert_eq!(name(0x3154_5844), 0x010e_e908);
        assert_eq!(name(0x5956_5955), 0x010e_e8a4);
        assert_eq!(name(0x5956_5956), UNKNOWN_FORMAT_01015890);
    }

    #[test]
    fn fn_00e69e50_maps_intervals() {
        let mut e = Engine::new();
        let values: Vec<u32> = (0..6)
            .map(|i| e.call(0x00e6_9e50, &args![i as u32]).u32())
            .collect();
        assert_eq!(values, vec![0x8000_0000, 1, 2, 4, 8, 0]);
    }

    #[test]
    fn fn_00e6c550_fills_the_clipper_pixels() {
        let mut e = Engine::new();
        e.register(0x00fd_f1c0, |_, _| Ret::default());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET_ADDR, |_, _| Ret::default());
        e.register(PIXEL_FORMAT_SET_UP_00A622D0, |_, _| Ret::default());
        e.register(PIXEL_DATA_CONSTRUCT_00A7C190, |e, a| {
            let table = e.mem.alloc(4);
            e.mem.set_u32(table, 0x9000);
            e.mem.set_u32(a[0] + 0x5c, table);
            e.mem.set_u32(a[0] + 0x50, 0x10);
            e.mem.set_u32(a[0] + 0x64, 3);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0xb80);
        let format = e.mem.alloc(0x20);
        e.mem.set_u32(format + 0x10, 0x55);
        // Block 0: entries 1 empty, 0 set; block 2: entry 5 set; blocks 1 and 3 empty.
        e.mem.set_u32(this + 0x700, format);
        e.mem.set_u32(this + 0x700 + 4 * (0x2c + 5), 0x777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_c550, &args![this]).u8(), 1);
        assert_eq!(e.mem.u32(this + 0x860), format);
        assert_eq!(e.mem.u32(this + 0x864), 0);
        assert_eq!(e.mem.u32(this + 0x868), 0x777);
        assert_ne!(e.mem.u32(this + 0x870), 0);
        assert_eq!(e.mem.u32(this + 0x874), 0);
        assert_eq!(e.mem.u32(this + 0x880), 0x55);
        let fills: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMSET_ADDR)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(fills, vec![vec![0x9010, 0xff, 0x30]; 2]);
        // No pixel format in block 0: 0.
        let empty = e.mem.alloc(0xb80);
        assert_eq!(e.call(0x00e6_c550, &args![empty]).u8(), 0);
    }
}

// ---- Batch b048 (a07) ----

layout! {
    /// `NiDX9Renderer` (PC only; RTTI `.?AVNiDX9Renderer@@`), 0xB80 bytes (its deleting
    /// destructor frees 0xB80). Fields by their use in this unit.
    pub struct NiDX9Renderer: 0xB80 {
        /// A `CRITICAL_SECTION` guarding the textures' renderer data (+0x180).
        0x180 texture_lock: u32,
        /// The vertex declaration `00e696a0` makes (position, normal, colour, one UV set).
        0x280 declaration_pncu: u32,
        /// The vertex declaration `00e69790` makes (the same with a second UV float).
        0x284 declaration_pncuu: u32,
        /// `IDirect3DDevice9*`.
        0x288 device: u32,
        /// The background colour as a `D3DCOLOR` (`0xAARRGGBB`; bytes B, G, R, A).
        0x5E0 background_color: u32,
        /// The word `00e757d0` answers.
        0x5EC word_5ec: u32,
        /// The word `00e695e0` answers.
        0x8A0 word_8a0: u32,
        /// The byte `00e695f0` answers.
        0x8A4 byte_8a4: u8,
        /// The object `00e69600` answers and `00e69620` calls.
        0x884 object_884: u32,
        /// The current render target group (`00e75810` answers it; the clear and the
        /// viewport read its size).
        0x888 target_group: u32,
        /// The object `00e69960`, `00e69970` and `00e699c0` call.
        0x8B8 object_8b8: u32,
        /// The object `00e69660` calls `00e8bf80` on.
        0x8BC object_8bc: u32,
        /// The object `00e69640` calls (vtable `010f086c`).
        0x8C4 object_8c4: u32,
        /// The object `00e71cc0` calls `00e8bd40` on.
        0x8C8 object_8c8: u32,
        /// The shaders `PurgeAllD3DShaders` releases: an `NiTPointerList` (head, tail, count).
        0x90C shader_list_head: u32,
        0x910 shader_list_tail: u32,
        0x914 shader_list_count: u32,
    }
}

/// The renderer's `+0x6FC` byte: set, `00e69660` does not end the scene.
const SKIP_END_SCENE: u32 = 0x6fc;
/// The map of textures' renderer data at +0x8CC (`NiTPointerMap`; `SetAt` `00b57ea0`,
/// `RemoveAt` `00e91fd0`).
const TEXTURE_DATA_MAP: u32 = 0x8cc;

/// `IDirect3DDevice9::EndScene` (slot 42) and `CreateVertexDeclaration` (slot 86).
const DEVICE_END_SCENE: u32 = 0xa8;
const DEVICE_CREATE_VERTEX_DECLARATION: u32 = 0x158;
/// `IUnknown::Release` (slot 2).
const RELEASE: u32 = 0x8;

/// `00a29680` (`cdecl`, a format and its arguments): the debug print.
/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
/// `EnterCriticalSection` / `LeaveCriticalSection` (KERNEL32 import slots).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `LoadLibraryA` / `GetProcAddress` (KERNEL32 import slots).
const LOAD_LIBRARY_A: u32 = 0x00fd_f0b0;
const GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
/// The allocator (`NEW_OBJECT` is above): `00aa1460` (`cdecl`, block and size), `00aa1110`
/// (`cdecl`, block: free).
const SIZED_DELETE: u32 = 0x00aa_1460;
const FREE: u32 = 0x00aa_1110;
/// `NiTPointerMap::SetAt` (`00b57ea0`, `thiscall`: key, value) and `GetAt` (`00a53c30`,
/// `thiscall`: key, &value; true when found), and `RemoveAt` (`00e91fd0`, `thiscall`: key).
const MAP_SET_AT: u32 = 0x00b5_7ea0;
const MAP_GET_AT: u32 = 0x00a5_3c30;
const MAP_REMOVE_AT: u32 = 0x00e9_1fd0;

/// The `NiTPointerList` node pool's free list and its spin lock: the lock (`011c5f80`, taken
/// with `0040fbf0`, `thiscall`, one word), its count (`011c5f84`) and the free-list head
/// (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE: u32 = 0x011c_5f58;
const NODE_POOL_LOCK_TAKE: u32 = 0x0040_fbf0;

// Translated from 00a765c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelFormat::operator!=` (Xbox PDB, `operator__ov2`): `thiscall`, one word (the other
/// format). Different when the words at +4 and +8, the bytes at +0 and +1 differ, when the
/// words at +0xC differ and neither is -1, or when any of the four 0xC-byte channels from
/// +0x14 differs in its two words or its two bytes.
pub fn ni_pixel_format_operator_not_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let a = this.addr();
    let b = other.addr();
    if e.mem.u32(b.wrapping_add(4)) != e.mem.u32(a.wrapping_add(4))
        || e.mem.u8(b) != e.mem.u8(a)
        || e.mem.u32(b.wrapping_add(8)) != e.mem.u32(a.wrapping_add(8))
        || e.mem.u8(b.wrapping_add(1)) != e.mem.u8(a.wrapping_add(1))
    {
        return true;
    }
    let theirs = e.mem.u32(b.wrapping_add(0xc));
    let ours = e.mem.u32(a.wrapping_add(0xc));
    if theirs != ours && ours != 0xffff_ffff && theirs != 0xffff_ffff {
        return true;
    }
    for i in 0..4u32 {
        let channel = 0x14 + 0xc * i;
        let (x, y) = (b.wrapping_add(channel), a.wrapping_add(channel));
        if e.mem.u32(x) != e.mem.u32(y)
            || e.mem.u32(x.wrapping_add(4)) != e.mem.u32(y.wrapping_add(4))
            || e.mem.u8(x.wrapping_add(8)) != e.mem.u8(y.wrapping_add(8))
            || e.mem.u8(x.wrapping_add(9)) != e.mem.u8(y.wrapping_add(9))
        {
            return true;
        }
    }
    false
}

/// `NiProperty`'s destructor as `NiVertexColorProperty` runs it (`00a5c240`).
const PROPERTY_DESTRUCT: u32 = 0x00a5_c240;

// Translated from 00a950f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty`'s scalar deleting destructor (Xbox PDB): `thiscall`, one word
/// (flags). The destructor (`00a5c240`), then with bit 0 the block goes back (`00aa1460`,
/// 0x1C bytes); answers the object.
pub fn ni_vertex_color_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(PROPERTY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

/// `00b71a40` (`cdecl`, four words): the conversion `GetD3DFromNi` forwards to.
const D3D_FROM_NI_00B71A40: u32 = 0x00b7_1a40;

// Translated from 00b799a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DUtility::GetD3DFromNi` (Xbox PDB): `cdecl`, two words (a destination and a source).
/// Calls `00b71a40` with the destination, the source, the source + 0x24 and the source's
/// float at +0x30, and answers what it answers.
pub fn ni_d3d_utility_get_d3d_from_ni(e: &mut Engine, destination: u32, source: Ptr) -> u32 {
    let s = source.addr();
    let value = x87_quiet_bits(e.mem.u32(s.wrapping_add(0x30)));
    e.call(
        D3D_FROM_NI_00B71A40,
        &args![destination, source, s.wrapping_add(0x24), value],
    )
    .u32()
}

/// `D3D9.DLL`'s module and its two entry points (`Direct3DCreate9Ex`, `Direct3DCreate9`).
const D3D9_MODULE: u32 = 0x0126_f0d4;
/// `"D3D9.DLL"`, `"Direct3DCreate9Ex"`, `"Direct3DCreate9"`.
const D3D9_DLL_TEXT: u32 = 0x010e_d520;
const DIRECT3D_CREATE_9_EX_TEXT: u32 = 0x010e_d50c;
const DIRECT3D_CREATE_9_TEXT: u32 = 0x010e_d4fc;

// Translated from 00e69410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether `Direct3DCreate9` is at hand (no arguments). Once
/// found it answers true at once; else loads `D3D9.DLL` and looks up `Direct3DCreate9Ex` and
/// `Direct3DCreate9` (stored at `0126f0e0` / `0126f0dc`), true when the latter was found.
pub fn fn_00e69410(e: &mut Engine) -> bool {
    if e.global::<u32>(DIRECT3D_CREATE9) != 0 {
        return true;
    }
    let module = e.call(LOAD_LIBRARY_A, &args![D3D9_DLL_TEXT]).u32();
    e.set_global(D3D9_MODULE, module);
    if module != 0 {
        let create_ex = e
            .call(GET_PROC_ADDRESS, &args![module, DIRECT3D_CREATE_9_EX_TEXT])
            .u32();
        e.set_global(DIRECT3D_CREATE9_EX, create_ex);
        let module = e.global::<u32>(D3D9_MODULE);
        let create = e
            .call(GET_PROC_ADDRESS, &args![module, DIRECT3D_CREATE_9_TEXT])
            .u32();
        e.set_global(DIRECT3D_CREATE9, create);
        if create != 0 {
            return true;
        }
    }
    false
}

// Translated from 00e695e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xD8): the word at +0x8A0.
pub fn fn_00e695e0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.get(this, NiDX9Renderer::word_8a0)
}

// Translated from 00e695f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xDC): the byte at +0x8A4.
pub fn fn_00e695f0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u8 {
    e.get(this, NiDX9Renderer::byte_8a4)
}

// Translated from 00e69600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xC8): the object at +0x884.
pub fn fn_00e69600(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.get(this, NiDX9Renderer::object_884)
}

// Translated from 00e69620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xD4): the object at +0x884's virtual +0xB0
/// with 0, answered as it is.
pub fn fn_00e69620(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_884);
    e.vcall(object, 0xb0, &args![0u32]).u32()
}

// Translated from 00e69640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xE0): a jump to the object at +0x8C4's
/// virtual +0x10 (`00e90c70` for its vtable `010f086c`, three stack words) with the same
/// three words.
pub fn fn_00e69640(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    first: u32,
    second: u32,
    third: u32,
) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_8c4);
    e.vcall(object, 0x10, &args![first, second, third]).u32()
}

/// `00e8bf80` (`thiscall`, no stack words), called on the object at +0x8BC.
const OBJECT_8BC_CALL_00E8BF80: u32 = 0x00e8_bf80;

// Translated from 00e69660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x180, the slot `Do_EndFrame` has on Xbox):
/// unless the byte at +0x6FC is set, ends the scene (`IDirect3DDevice9::EndScene`; a
/// failure answers false) and calls `00e8bf80` on the object at +0x8BC; answers true.
pub fn fn_00e69660(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> bool {
    if e.mem.u8(this.addr().wrapping_add(SKIP_END_SCENE)) == 0 {
        let device = e.get(this, NiDX9Renderer::device);
        if (e.vcall(device, DEVICE_END_SCENE, &args![]).u32() as i32) < 0 {
            return false;
        }
        let object = e.get(this, NiDX9Renderer::object_8bc);
        e.call(OBJECT_8BC_CALL_00E8BF80, &args![object]);
    }
    true
}

/// One `D3DVERTEXELEMENT9` (stream, offset, type, method, usage, usage index).
type VertexElement = (u16, u16, u8, u8, u8, u8);

/// Writes the elements as the stack array the code builds and creates the declaration into
/// `slot` (`IDirect3DDevice9::CreateVertexDeclaration`); a failure clears the slot. Answers
/// the slot's declaration.
fn create_declaration(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    slot: u32,
    elements: &[VertexElement],
) -> u32 {
    let slot = this.addr().wrapping_add(slot);
    if e.mem.u32(slot) != 0 {
        return e.mem.u32(slot);
    }
    let device = e.get(this, NiDX9Renderer::device);
    let failed = e.with_stack(8 * elements.len() as u32, |e, array| {
        for (i, (stream, offset, kind, method, usage, index)) in elements.iter().enumerate() {
            let at = array.addr() + 8 * i as u32;
            e.mem.set_u16(at, *stream);
            e.mem.set_u16(at + 2, *offset);
            e.mem.set_u8(at + 4, *kind);
            e.mem.set_u8(at + 5, *method);
            e.mem.set_u8(at + 6, *usage);
            e.mem.set_u8(at + 7, *index);
        }
        (e.vcall(
            device,
            DEVICE_CREATE_VERTEX_DECLARATION,
            &args![array, slot],
        )
        .u32() as i32)
            < 0
    });
    if failed {
        e.mem.set_u32(slot, 0);
        return 0;
    }
    e.mem.u32(slot)
}

// Translated from 00e696a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the renderer's position / normal / colour / texture vertex
/// declaration, made on first use (`thiscall`, no stack words). The elements: `FLOAT3`
/// position at 0, `FLOAT3` normal at 12, `D3DCOLOR` colour at 24, `FLOAT2` texture
/// coordinate 0 at 28, then `D3DDECL_END`. A failed creation clears the slot and answers 0.
pub fn fn_00e696a0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    create_declaration(
        e,
        this,
        0x280,
        &[
            (0, 0, 2, 0, 0, 0),
            (0, 12, 2, 0, 3, 0),
            (0, 24, 4, 0, 10, 0),
            (0, 28, 1, 0, 5, 0),
            (0xff, 0, 0x11, 0, 0, 0),
        ],
    )
}

// Translated from 00e69790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: like `00e696a0` with a sixth element, a `FLOAT1` texture
/// coordinate 1 at 36 (slot +0x284).
pub fn fn_00e69790(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    create_declaration(
        e,
        this,
        0x284,
        &[
            (0, 0, 2, 0, 0, 0),
            (0, 12, 2, 0, 3, 0),
            (0, 24, 4, 0, 10, 0),
            (0, 28, 1, 0, 5, 0),
            (0, 36, 0, 0, 5, 1),
            (0xff, 0, 0x11, 0, 0, 0),
        ],
    )
}

// Translated from 00e69960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xF4): a jump to the object at +0x8B8's
/// virtual +0x54 (one stack word) with the same word.
pub fn fn_00e69960(e: &mut Engine, this: Ptr<NiDX9Renderer>, value: u32) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(object, 0x54, &args![value]).u32()
}

/// The purge both `PurgeVertexShader` and `PurgePixelShader` run: the object at +0x8B8's
/// virtual `own` with (0, 0); the shader's virtual `get` (a Direct3D object, released when
/// set); the shader's virtual `clear` with 0, whose answer is the result.
fn purge_shader(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    shader: u32,
    own: u32,
    get: u32,
    clear: u32,
) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(object, own, &args![0u32, 0u32]);
    let d3d = e.vcall(shader, get, &args![]).u32();
    if d3d != 0 {
        e.vcall(d3d, RELEASE, &args![]);
    }
    e.vcall(shader, clear, &args![0u32]).u32()
}

// Translated from 00e69970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PurgeVertexShader` (the engine map's name): `thiscall`, one word (the
/// shader). The object at +0x8B8's virtual +0x8C with (0, 0), the shader's Direct3D vertex
/// shader (virtual +0x84) released, and the shader's virtual +0x88 with 0.
pub fn ni_xenon_renderer_purge_vertex_shader(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    shader: u32,
) -> u32 {
    purge_shader(e, this, shader, 0x8c, 0x84, 0x88)
}

// Translated from 00e699c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PurgePixelShader` (the engine map's name): as `PurgeVertexShader` with
/// the virtuals +0x7C (on both objects) and +0x80.
pub fn ni_xenon_renderer_purge_pixel_shader(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    shader: u32,
) -> u32 {
    purge_shader(e, this, shader, 0x7c, 0x7c, 0x80)
}

/// `00e7d9d0` (`cdecl`, four words: the device, &first, second, third).
const DEVICE_CALL_00E7D9D0: u32 = 0x00e7_d9d0;

// Translated from 00e69a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x154): `thiscall`, three words. False for a
/// first word of 0; else `00e7d9d0` with the device, the address of the first word (its
/// stack slot) and the other two, true when that answers nonzero.
pub fn fn_00e69a20(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    first: u32,
    second: u32,
    third: u32,
) -> bool {
    if first == 0 {
        return false;
    }
    let device = e.get(this, NiDX9Renderer::device);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), first);
        e.call(DEVICE_CALL_00E7D9D0, &args![device, slot, second, third])
            .u32()
            != 0
    })
}

/// The adapter description `00e69a60` makes once (`0126f0e8`) and the argument it passes
/// (`0126f0f8`).
const ADAPTER_INFO_ARGUMENT: u32 = 0x0126_f0f8;

// Translated from 00e69a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the adapter description (no arguments), made on first use:
/// with `Direct3DCreate9` at hand a temporary `IDirect3D9` builds a 0x14-byte object
/// (`00e7bc90` with it and `0126f0f8`; 0 when the allocation failed) and is released.
pub fn fn_00e69a60(e: &mut Engine) -> u32 {
    let info = e.global::<u32>(ADAPTER_LIST);
    if info != 0 {
        return info;
    }
    if fn_00e69410(e) {
        let create = e.global::<u32>(DIRECT3D_CREATE9);
        let direct3d = e.call(create, &args![D3D_SDK_VERSION]).u32();
        if direct3d != 0 {
            let block = e.call(NEW_OBJECT, &args![0x14u32]).u32();
            if block != 0 {
                let info = fn_00e7bc90(e, Ptr::new(block), direct3d, ADAPTER_INFO_ARGUMENT);
                e.set_global(ADAPTER_LIST, info.addr());
                e.vcall(direct3d, RELEASE, &args![]);
                return e.global::<u32>(ADAPTER_LIST);
            }
            e.set_global(ADAPTER_LIST, 0u32);
            e.vcall(direct3d, RELEASE, &args![]);
        }
    }
    e.global::<u32>(ADAPTER_LIST)
}

// Translated from 00e69b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, one word (an RTTI record). Walks the chain the
/// object's virtual +0xC answers (each link's next at +4) and answers the object when the
/// record is on it, else 0.
pub fn fn_00e69b00(e: &mut Engine, this: Ptr, rtti: u32) -> u32 {
    let mut link = e.vcall(this.addr(), 0xc, &args![]).u32();
    while link != 0 {
        if link == rtti {
            return this.addr();
        }
        link = e.mem.u32(link.wrapping_add(4));
    }
    0
}

// Translated from 00e69b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, one word (an index). The element at that index of
/// the array at +0x28 when it is below the count at +0x20 (unsigned), else 0.
pub fn fn_00e69b40(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index < e.mem.u32(this.addr().wrapping_add(0x20)) {
        let array = e.mem.u32(this.addr().wrapping_add(0x28));
        e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
    } else {
        0
    }
}

// Translated from 00e6d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x104): `thiscall`, one word (an object; the
/// renderer is not read). Frees the object's block at +0x28 (`00aa1110`) and clears +0x28
/// and +0x24.
pub fn fn_00e6d740(e: &mut Engine, _this: Ptr, object: Ptr) {
    let o = object.addr();
    let block = e.mem.u32(o.wrapping_add(0x28));
    e.call(FREE, &args![block]);
    e.mem.set_u32(o.wrapping_add(0x28), 0);
    e.mem.set_u32(o.wrapping_add(0x24), 0);
}

/// `00e68ef0` (`cdecl`: texture, renderer) and `00e8abf0` (`cdecl`: texture, renderer): the
/// renderer data makers `00e6dc40` and `00e6dcc0` call.
const MAKE_TEXTURE_DATA_00E68EF0: u32 = 0x00e6_8ef0;
const MAKE_TEXTURE_DATA_00E8ABF0: u32 = 0x00e8_abf0;
/// `00e8ff60` (`cdecl`: texture, renderer, a word).
const MAKE_TEXTURE_DATA_00E8FF60: u32 = 0x00e8_ff60;

// Translated from 00e6dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x13C): `thiscall`, one word (a texture).
/// Under the lock at +0x180, makes the texture's renderer data (`00e68ef0`) unless it has
/// some at +0x24; true when it has data.
pub fn fn_00e6dc40(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr) -> bool {
    let lock = this.addr().wrapping_add(0x180);
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    let mut data = e.mem.u32(texture.addr().wrapping_add(0x24));
    if data == 0 {
        data = e
            .call(MAKE_TEXTURE_DATA_00E68EF0, &args![texture, this])
            .u32();
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
    data != 0
}

// Translated from 00e6dc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x140): `thiscall`, two words (a texture and
/// a word). Without renderer data at +0x24 makes it (`00e8ff60` with the texture, the
/// renderer and the word; none answers false) and records it in the map at +0x8CC under the
/// texture. True otherwise.
pub fn fn_00e6dc80(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr, value: u32) -> bool {
    if e.mem.u32(texture.addr().wrapping_add(0x24)) == 0 {
        let data = e
            .call(MAKE_TEXTURE_DATA_00E8FF60, &args![texture, this, value])
            .u32();
        if data == 0 {
            return false;
        }
        let map = this.addr().wrapping_add(TEXTURE_DATA_MAP);
        e.call(MAP_SET_AT, &args![map, texture, data]);
    }
    true
}

// Translated from 00e6dcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x144): `thiscall`, one word (a texture).
/// Takes the lock at +0x180; with renderer data at +0x24 answers true at once (the lock is
/// not given back on that path); else makes it (`00e8abf0`), gives the lock back and
/// answers whether it was made.
pub fn fn_00e6dcc0(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr) -> bool {
    let lock = this.addr().wrapping_add(0x180);
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    if e.mem.u32(texture.addr().wrapping_add(0x24)) != 0 {
        return true;
    }
    let data = e
        .call(MAKE_TEXTURE_DATA_00E8ABF0, &args![texture, this])
        .u32();
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
    data != 0
}

// Translated from 00e6df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x15C): `thiscall`, one word (a texture).
/// Clears the texture's renderer data (+0x24) and jumps to the map at +0x8CC's `RemoveAt`
/// (`00e91fd0`) with the texture.
pub fn fn_00e6df80(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr) -> u32 {
    e.mem.set_u32(texture.addr().wrapping_add(0x24), 0);
    let map = this.addr().wrapping_add(TEXTURE_DATA_MAP);
    e.call(MAP_REMOVE_AT, &args![map, texture]).u32()
}

/// `NiD3DShaderInterface::SetD3DRenderer` (`00e818d0`, `thiscall`, one word).
const SHADER_SET_D3D_RENDERER: u32 = 0x00e8_18d0;
/// `00e7db70` (`thiscall` on the shader list, one word: the address of the element):
/// `AddTail`.
const SHADER_LIST_ADD: u32 = 0x00e7_db70;
/// `00e7d5b0` (`thiscall` on the shader list): removes the tail node.
const SHADER_LIST_REMOVE_TAIL: u32 = 0x00e7_d5b0;

// Translated from 00e75310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, one word (a shader). Clears the shader's renderer
/// (`SetD3DRenderer(0)`) and adds it to the list at +0x90C (`00e7db70` with the address of
/// the stack word).
pub fn fn_00e75310(e: &mut Engine, this: Ptr<NiDX9Renderer>, shader: u32) {
    e.call(SHADER_SET_D3D_RENDERER, &args![shader, 0u32]);
    let list = this.addr().wrapping_add(0x90c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), shader);
        e.call(SHADER_LIST_ADD, &args![list, slot]);
    });
}

// Translated from 00e75340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: as `00e75310`, with the shader's virtual +0xCC called first.
pub fn fn_00e75340(e: &mut Engine, this: Ptr<NiDX9Renderer>, shader: u32) {
    e.vcall(shader, 0xcc, &args![]);
    e.call(SHADER_SET_D3D_RENDERER, &args![shader, 0u32]);
    let list = this.addr().wrapping_add(0x90c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), shader);
        e.call(SHADER_LIST_ADD, &args![list, slot]);
    });
}

/// The list node's return to the pool, as the code inlines it: the element cleared, the pool
/// lock taken (`0040fbf0` with 0), the node's previous cleared and the node pushed on the
/// free list, the lock count dropped (the lock cleared at 0).
fn free_list_node(e: &mut Engine, node: u32) {
    e.mem.set_u32(node.wrapping_add(8), 0);
    e.call(NODE_POOL_LOCK_TAKE, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(node.wrapping_add(4), 0);
    let free = e.global::<u32>(NODE_POOL_FREE);
    e.mem.set_u32(node, free);
    let count = e.global::<u32>(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.set_global(NODE_POOL_LOCK_COUNT, count);
    e.set_global(NODE_POOL_FREE, node);
    if count == 0 {
        e.set_global(NODE_POOL_LOCK, 0u32);
    }
}

// Translated from 00e75380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PurgeAllD3DShaders` (the engine map's name): `thiscall`, no stack
/// words. While the shader list at +0x90C is not empty: takes the head's shader, calls its
/// virtual +0xCC and `SetD3DRenderer(0)`, then removes the first node holding it (head:
/// unlinked and pooled; tail: `00e7d5b0`; middle: unlinked and pooled), the count dropping
/// for a pooled node.
pub fn ni_xenon_renderer_purge_all_d3d_shaders(e: &mut Engine, this: Ptr<NiDX9Renderer>) {
    let t = this.addr();
    let list = t.wrapping_add(0x90c);
    while e.get(this, NiDX9Renderer::shader_list_count) != 0 {
        let head = e.mem.u32(list);
        let shader = e.mem.u32(head.wrapping_add(8));
        e.vcall(shader, 0xcc, &args![]);
        e.call(SHADER_SET_D3D_RENDERER, &args![shader, 0u32]);
        let head = e.mem.u32(list);
        let mut node = head;
        let mut found = 0;
        while node != 0 {
            let next = e.mem.u32(node);
            if e.mem.u32(node.wrapping_add(8)) == shader {
                found = node;
                break;
            }
            node = next;
        }
        if found == 0 {
            continue;
        }
        if found == head {
            let next = e.mem.u32(head);
            e.mem.set_u32(list, next);
            if next != 0 {
                e.mem.set_u32(next.wrapping_add(4), 0);
            } else {
                e.mem.set_u32(list.wrapping_add(4), 0);
            }
            free_list_node(e, head);
        } else if found == e.mem.u32(list.wrapping_add(4)) {
            e.call(SHADER_LIST_REMOVE_TAIL, &args![list]);
            continue;
        } else {
            let previous = e.mem.u32(found.wrapping_add(4));
            let next = e.mem.u32(found);
            if previous != 0 {
                e.mem.set_u32(previous, next);
            }
            if next != 0 {
                e.mem.set_u32(next.wrapping_add(4), previous);
            }
            free_list_node(e, found);
        }
        let count = e.mem.u32(list.wrapping_add(8)).wrapping_sub(1);
        e.mem.set_u32(list.wrapping_add(8), count);
    }
}

// Translated from 00e757d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xA0): the word at +0x5EC.
pub fn fn_00e757d0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.get(this, NiDX9Renderer::word_5ec)
}

// Translated from 00e75810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xCC): the word at +0x888.
pub fn fn_00e75810(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.get(this, NiDX9Renderer::target_group)
}

/// `255.0` (`double`) and `1/255` (`double`, a float's value).
const SCALE_TO_BYTE: u32 = 0x0101_e568;
const SCALE_FROM_BYTE: u32 = 0x0102_31e8;

/// `FISTP` under the code's chop control word, keeping the low byte (`MOVZX` of it).
fn channel_byte(value: f64) -> u32 {
    (crate::x87::fistp_chop(value) as u32) & 0xff
}

// Translated from 00e75840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xB0, a `SetBackgroundColor`): `thiscall`,
/// one word (an `NiColor`). Each channel times 255, truncated, its low byte packed into
/// `0xFFRRGGBB` at +0x5E0.
pub fn fn_00e75840(e: &mut Engine, this: Ptr<NiDX9Renderer>, color: Ptr) {
    let c = color.addr();
    let scale = x87_load64(e.mem.u64(SCALE_TO_BYTE));
    let red = x87_mul(x87_load32(e.mem.u32(c)), scale);
    let mut packed = (channel_byte(red) | 0xffff_ff00) << 8;
    let green = x87_mul(x87_load32(e.mem.u32(c.wrapping_add(4))), scale);
    packed = (packed | channel_byte(green)) << 8;
    let blue = x87_mul(scale, x87_m32(e.mem.u32(c.wrapping_add(8))));
    packed |= channel_byte(blue);
    e.set(this, NiDX9Renderer::background_color, packed);
}

// Translated from 00e758f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xAC, a `SetBackgroundColor` for `NiColorA`):
/// `thiscall`, one word. Alpha, red, green and blue times 255, truncated, packed into
/// `0xAARRGGBB` at +0x5E0.
pub fn fn_00e758f0(e: &mut Engine, this: Ptr<NiDX9Renderer>, color: Ptr) {
    let c = color.addr();
    let scale = x87_load64(e.mem.u64(SCALE_TO_BYTE));
    let alpha = x87_mul(x87_load32(e.mem.u32(c.wrapping_add(0xc))), scale);
    let mut packed = channel_byte(alpha) << 8;
    let red = x87_mul(x87_load32(e.mem.u32(c)), scale);
    packed = (packed | channel_byte(red)) << 8;
    let green = x87_mul(x87_load32(e.mem.u32(c.wrapping_add(4))), scale);
    packed = (packed | channel_byte(green)) << 8;
    let blue = x87_mul(scale, x87_m32(e.mem.u32(c.wrapping_add(8))));
    packed |= channel_byte(blue);
    e.set(this, NiDX9Renderer::background_color, packed);
}

// Translated from 00e759c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xB4, a `GetBackgroundColor`): `thiscall`,
/// one word (an `NiColorA` to fill). Each byte of the colour at +0x5E0 times 1/255.
pub fn fn_00e759c0(e: &mut Engine, this: Ptr<NiDX9Renderer>, color: Ptr) {
    let t = this.addr();
    let c = color.addr();
    let scale = x87_load64(e.mem.u64(SCALE_FROM_BYTE));
    let alpha = f64::from(e.mem.u8(t + 0x5e3));
    e.mem
        .set_u32(c.wrapping_add(0xc), x87_store32(x87_mul(alpha, scale)));
    let red = f64::from(e.mem.u8(t + 0x5e2));
    e.mem.set_u32(c, x87_store32(x87_mul(red, scale)));
    let green = f64::from(e.mem.u8(t + 0x5e1));
    e.mem
        .set_u32(c.wrapping_add(4), x87_store32(x87_mul(green, scale)));
    let blue = f64::from(e.mem.u8(t + 0x5e0));
    e.mem
        .set_u32(c.wrapping_add(8), x87_store32(x87_mul(scale, blue)));
}

/// `NiDX9Renderer`'s destructor (`00e75a70`).
const RENDERER_DESTRUCT: u32 = 0x00e7_5a70;

// Translated from 00e762f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9Renderer`'s scalar deleting destructor (vtable slot 0): `thiscall`, one word
/// (flags). The destructor (`00e75a70`), then with bit 0 the block goes back (`00aa1460`,
/// 0xB80 bytes); answers the renderer.
pub fn fn_00e762f0(e: &mut Engine, this: Ptr<NiDX9Renderer>, flags: u32) -> Ptr<NiDX9Renderer> {
    e.call(RENDERER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xb80u32]);
    }
    this
}

/// The adapter description's vtable (`010eeca4`) and its fill (`00e7bbf0`, `thiscall`, two
/// words, in `run_00e7b200`).
const ADAPTER_INFO_VTABLE: u32 = 0x010e_eca4;
const ADAPTER_INFO_FILL: u32 = 0x00e7_bbf0;

// Translated from 00e7bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the adapter description's constructor (`thiscall`, two words:
/// the `IDirect3D9` and a word passed on). An empty array (vtable `010eeca4`, the words at
/// +0 and +8 and the counts at +0xC, +0xE, +0x10 cleared, growth 1 at +0x12), then the fill
/// (`00e7bbf0`) with both words; answers the object.
pub fn fn_00e7bc90(e: &mut Engine, this: Ptr, direct3d: u32, value: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 0x12, 1);
    e.mem.set_u16(t + 0xe, 0);
    e.mem.set_u16(t + 0xc, 0);
    e.mem.set_u16(t + 0x10, 0);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 4, ADAPTER_INFO_VTABLE);
    e.mem.set_u32(t, 0);
    e.call(ADAPTER_INFO_FILL, &args![this, direct3d, value]);
    this
}

/// `0043a010` (`thiscall`, no stack words).
const MEMBER_CALL_0043A010: u32 = 0x0043_a010;

// Translated from 00e72e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a jump to `0043a010` on the object's member at +0xC.
pub fn fn_00e72e50(e: &mut Engine, this: Ptr) -> u32 {
    e.call(MEMBER_CALL_0043A010, &args![this.addr().wrapping_add(0xc)])
        .u32()
}

/// `00653290` (`thiscall`, one word) and its argument here (`011f4a28`); `00e8bd40`
/// (`thiscall` on the object at +0x8C8, one word).
const OBJECT_LOOKUP_00653290: u32 = 0x0065_3290;
const OBJECT_LOOKUP_ARGUMENT: u32 = 0x011f_4a28;
const OBJECT_8C8_CALL_00E8BD40: u32 = 0x00e8_bd40;

// Translated from 00e71cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xF8): `thiscall`, one word (an object).
/// For a set object, what `00653290` answers for `011f4a28` on it, when set, goes to
/// `00e8bd40` on the object at +0x8C8.
pub fn fn_00e71cc0(e: &mut Engine, this: Ptr<NiDX9Renderer>, object: u32) {
    if object == 0 {
        return;
    }
    let found = e
        .call(
            OBJECT_LOOKUP_00653290,
            &args![object, OBJECT_LOOKUP_ARGUMENT],
        )
        .u32();
    if found != 0 {
        let target = e.get(this, NiDX9Renderer::object_8c8);
        e.call(OBJECT_8C8_CALL_00E8BD40, &args![target, found]);
    }
}

/// The RTTI record `00e6ee00` checks for (`0126f5d8`).
const SHADER_RTTI_0126F5D8: u32 = 0x0126_f5d8;
/// `00e7bd20` (`cdecl`, one word), `00e7b0f0` (`thiscall` on the object at +0x5DC, four
/// words: a key) and `00e7c500` (`cdecl`, one word: makes the value).
const KEY_PART_00E7BD20: u32 = 0x00e7_bd20;
const MAKE_VALUE_00E7C500: u32 = 0x00e7_c500;

// Translated from 00e6ee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x94): `thiscall`, three words. Asks the
/// object at +0x884's virtual +0xC8 (with 0) for an object and keeps it when it derives from
/// the record at `0126f5d8` (`00e69b00`); builds a key with `00e7b0f0` on the object at
/// +0x5DC from that object's word +0x28, `00e7bd20` of the first word and the other two;
/// answers the map at +0xB14's value for the key when it has a nonzero one, else makes one
/// (`00e7c500`), stores it in the map and answers it.
pub fn fn_00e6ee00(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    first: u32,
    second: u32,
    third: u32,
) -> u32 {
    let t = this.addr();
    let source = e.get(this, NiDX9Renderer::object_884);
    let object = e.vcall(source, 0xc8, &args![0u32]).u32();
    let derived = if object == 0 {
        0
    } else {
        fn_00e69b00(e, Ptr::new(object), SHADER_RTTI_0126F5D8)
    };
    let part = e.call(KEY_PART_00E7BD20, &args![first]).u32();
    let maker = e.mem.u32(t.wrapping_add(0x5dc));
    let word = e.mem.u32(derived.wrapping_add(0x28));
    let key = e
        .call(
            DEPTH_FORMAT_00E7B0F0,
            &args![maker, word, part, second, third],
        )
        .u32();
    let map = t.wrapping_add(0xb14);
    // The first argument's stack slot holds the map's answer.
    let found = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let hit = e.call(MAP_GET_AT, &args![map, key, slot]).u8() != 0;
        (hit, e.mem.u32(slot.addr()))
    });
    if found.0 && found.1 != 0 {
        return found.1;
    }
    let value = e.call(MAKE_VALUE_00E7C500, &args![key]).u32();
    e.call(MAP_SET_AT, &args![map, key, value]);
    value
}

// Translated from 00e72c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, two words (a geometry and an object or 0). False
/// when the geometry's data (+0xB8) has a word at +0x34; true when the chain +0xBC, +0xC,
/// +0xC ends in an object with a word at +0x28 (false when the chain breaks), or when the
/// renderer's virtual +0xA0 answers bit 2. Then with the object (or the geometry's +0xC0):
/// true when the word at +0x6F0 is above `0xFFFE0000` and its virtual +0xD4 answers true,
/// or when the byte at +0x5D5 is set and its virtual +0xBC answers an object whose virtual
/// +0xA8 answers true; else false.
pub fn fn_00e72c20(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr, object: u32) -> bool {
    let t = this.addr();
    let g = geometry.addr();
    let data = e.mem.u32(g.wrapping_add(0xb8));
    if e.mem.u32(data.wrapping_add(0x34)) != 0 {
        return false;
    }
    let first = e.mem.u32(g.wrapping_add(0xbc));
    if first == 0 {
        return false;
    }
    let second = e.mem.u32(first.wrapping_add(0xc));
    if second == 0 {
        return false;
    }
    let third = e.mem.u32(second.wrapping_add(0xc));
    if e.mem.u32(third.wrapping_add(0x28)) != 0 {
        return true;
    }
    if e.vcall(t, 0xa0, &args![]).u32() & 2 != 0 {
        return true;
    }
    let mut target = object;
    if target == 0 {
        target = e.mem.u32(g.wrapping_add(0xc0));
    }
    if e.mem.u32(t.wrapping_add(0x6f0)) > 0xfffe_0000
        && target != 0
        && e.vcall(target, 0xd4, &args![]).u8() != 0
    {
        return true;
    }
    if e.mem.u8(t.wrapping_add(0x5d5)) != 0
        && target != 0
        && e.vcall(target, 0xbc, &args![]).u32() != 0
    {
        let inner = e.vcall(target, 0xbc, &args![]).u32();
        if e.vcall(inner, 0xa8, &args![]).u8() != 0 {
            return true;
        }
    }
    false
}

/// The texture creators `00e6fa90` calls: `00a670c0` (`thiscall`, seven words), `00a84ca0`
/// (`thiscall` on the second word's +8), `00a7b6c0` (`thiscall`, two words), `00a760c0`
/// (`thiscall`, three words).
const TEXTURE_SET_00A670C0: u32 = 0x00a6_70c0;
const OBJECT_CALL_00A84CA0: u32 = 0x00a8_4ca0;
const TEXTURE_CALL_00A7B6C0: u32 = 0x00a7_b6c0;
const TEXTURE_CALL_00A760C0: u32 = 0x00a7_60c0;
/// `0043b300` (`cdecl`: an RTTI record, an object): whether the object is of that class;
/// the two records `00e6fa90` asks about.
const IS_KIND_OF: u32 = 0x0043_b300;
const TEXTURE_RTTI_011F4A88: u32 = 0x011f_4a88;
const TEXTURE_RTTI_011F4A70: u32 = 0x011f_4a70;

// Translated from 00e6fa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, three words (a texture, an object, a flag); the
/// renderer is not read. Only for a texture whose word +0xE has `0x4000` in bits 12..14:
/// `00a670c0` with the word +8, then (by the byte +0x38's bits 0..3) the words +0x20, +0x24,
/// +0x28, the word +0x2C and bit 0 of the word +0xC, and the top four bits of the word +0xC.
/// With the flag set and bit 5 of the byte clear, `00a84ca0` on the object's +8; with it
/// clear and bit 4 clear, `00a7b6c0` (word +0x40, 0) or `00a760c0` (word +0x44, word +0x48,
/// 0) for the texture's class (`011f4a88` / `011f4a70`).
pub fn fn_00e6fa90(e: &mut Engine, _this: Ptr, texture: Ptr, object: Ptr, flag: u32) {
    let x = texture.addr();
    if x == 0 || e.mem.u16(x.wrapping_add(0xe)) & 0x7000 != 0x4000 {
        return;
    }
    let width = u32::from(e.mem.u16(x.wrapping_add(8)));
    let bits = u32::from(e.mem.u8(x.wrapping_add(0x38)));
    let top = u32::from(e.mem.u16(x.wrapping_add(0xc))) & 0xf000;
    if bits == 0 {
        e.call(
            TEXTURE_SET_00A670C0,
            &args![texture, width, 0u32, 0u32, 0u32, 0u32, 0u32, top],
        );
    } else {
        let first = if bits & 1 != 0 {
            e.mem.u32(x.wrapping_add(0x20))
        } else {
            0
        };
        let second = if bits & 2 != 0 {
            e.mem.u32(x.wrapping_add(0x24))
        } else {
            0
        };
        let third = if bits & 4 != 0 {
            e.mem.u32(x.wrapping_add(0x28))
        } else {
            0
        };
        let (bit, fourth) = if bits & 8 != 0 {
            (
                u32::from(e.mem.u16(x.wrapping_add(0xc))) & 1,
                e.mem.u32(x.wrapping_add(0x2c)),
            )
        } else {
            (0, 0)
        };
        let top = u32::from(e.mem.u16(x.wrapping_add(0xc))) & 0xf000;
        e.call(
            TEXTURE_SET_00A670C0,
            &args![texture, width, first, second, third, fourth, bit, top],
        );
    }
    if flag != 0 {
        if bits & 0x20 == 0 {
            let target = e.mem.u32(object.addr().wrapping_add(8));
            e.call(OBJECT_CALL_00A84CA0, &args![target]);
        }
        return;
    }
    if bits & 0x10 != 0 {
        return;
    }
    if e.call(IS_KIND_OF, &args![TEXTURE_RTTI_011F4A88, texture])
        .u8()
        != 0
    {
        let value = u32::from(e.mem.u16(x.wrapping_add(0x40)));
        e.call(TEXTURE_CALL_00A7B6C0, &args![texture, value, 0u32]);
        return;
    }
    if e.call(IS_KIND_OF, &args![TEXTURE_RTTI_011F4A70, texture])
        .u8()
        != 0
    {
        let value = u32::from(e.mem.u16(x.wrapping_add(0x44)));
        let other = e.mem.u32(x.wrapping_add(0x48));
        e.call(TEXTURE_CALL_00A760C0, &args![texture, value, other, 0u32]);
    }
}

// Translated from 00e71cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xF0, the slot after `PrecacheGeometry`):
/// `thiscall`, one word (a geometry). For a geometry with data at +0x34 whose array (count
/// +0x20, elements +0x28) is not empty and starts with a set element: under the locks at
/// +0x80 and +0x100, for each element looks up its key (+4) in the map at +0x610 and takes
/// every record (records chained at +0x20) owned by this geometry out of the chain (a first
/// record's successor replaces it in the map, or the key goes), freeing each (0x24 bytes).
/// Then, for any geometry with data, the object at +0x8AC's virtual +0x1C with (geometry,
/// 0).
pub fn fn_00e71cf0(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: u32) {
    let t = this.addr();
    if geometry == 0 {
        return;
    }
    let data = e.mem.u32(geometry.wrapping_add(0x34));
    if data == 0 {
        return;
    }
    if e.mem.u32(data.wrapping_add(0x20)) != 0 && e.mem.u32(e.mem.u32(data.wrapping_add(0x28))) != 0
    {
        let (first_lock, second_lock) = (t.wrapping_add(0x80), t.wrapping_add(0x100));
        e.call(ENTER_CRITICAL_SECTION, &args![first_lock]);
        e.call(ENTER_CRITICAL_SECTION, &args![second_lock]);
        let mut index = 0u32;
        if e.mem.u32(data.wrapping_add(0x20)) != 0 {
            loop {
                // The first pass checks the index against the count (always below it); the
                // loop's own test keeps the later ones below it too.
                let array = e.mem.u32(data.wrapping_add(0x28));
                let element = e.mem.u32(array.wrapping_add(index * 4));
                let key = e.mem.u32(element.wrapping_add(4));
                unlink_records(e, t, key, 0, geometry);
                index += 1;
                if index >= e.mem.u32(data.wrapping_add(0x20)) {
                    break;
                }
            }
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![second_lock]);
        e.call(LEAVE_CRITICAL_SECTION, &args![first_lock]);
    }
    let object = e.mem.u32(t.wrapping_add(0x8ac));
    e.vcall(object, 0x1c, &args![geometry, 0u32]);
}

/// `00e8c120` (`thiscall` on the object at +0x8BC, nine words), `00e8bfa0` (`thiscall` on it,
/// two words), and the geometry data's `00e8ef10`, `00e8f010`, `00e8f040` (`thiscall`, one
/// word each) and `00e8eeb0` (`thiscall`: whether every stream it lists has its buffer).
const GROUP_PREPARE_00E8C120: u32 = 0x00e8_c120;
const GROUP_STREAM_00E8BFA0: u32 = 0x00e8_bfa0;
const DATA_SET_COUNT_00E8EF10: u32 = 0x00e8_ef10;
const DATA_SET_FIRST_00E8F010: u32 = 0x00e8_f010;
const DATA_SET_SECOND_00E8F040: u32 = 0x00e8_f040;
const DATA_IS_PACKED_00E8EEB0: u32 = 0x00e8_eeb0;
/// The cap on bit 0 of the object's word +0xC that `00e6da60` packs (`011bf558`).
const STREAM_LIMIT_011BF558: u32 = 0x011b_f558;

// Translated from 00e6da60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: prepares a geometry data's streams (`thiscall`, four words:
/// the data, the object, a partition or 0, a source or 0). Asks the source's virtual +0xB0
/// for a stream count and an object; failing that (false for a partition whose word +0x20
/// is above 4) the object at +0x8BC's `00e8c120` fills a count of 1, two words and four more
/// locals. The count goes to the data (`00e8ef10`); with a first word it goes to the data
/// (`00e8f010`) and the second into its first stream slot (when it has slots), else the
/// source's object goes to the data (`00e8f040`) and each slot gets the source's virtual
/// +0xA0 of its index. The data's word +4 packs bit 0 of the object's word +0xC (at most the
/// word at `011bf558`) into the top byte with `0x400000` (object word +0x28 set), `0x800000`
/// (word +0x24 set) and `0x200000` (+0x24 set and the word +0xC's top four bits set); with a
/// source and the byte +0x5D4 the data's byte +0x14 takes the source's virtual +0xA8. Then
/// each stream is set up (`00e8bfa0`); on a failure the ones done are undone from the
/// failing index down to 1 (the data's +8 object, virtual +0x1C) and the answer is false.
///
/// The four locals `00e8c120` fills start out as the exe leaves them (not set); here 0.
pub fn fn_00e6da60(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    data: Ptr,
    object: Ptr,
    partition: u32,
    source: u32,
) -> bool {
    let t = this.addr();
    let d = data.addr();
    let o = object.addr();
    let prepared = e.with_stack(0x24, |e, locals| {
        // +0: the count, +4 and +8: the two words, +0xC: the source's object, +0x10..+0x1C:
        // the four other locals, +0x20: the source's stack slot.
        let l = locals.addr();
        let count = l;
        let first = l + 4;
        let second = l + 8;
        let source_object = l + 0xc;
        let mut answered = false;
        if source != 0 {
            answered = e.vcall(source, 0xb0, &args![source_object, count]).u8() != 0;
        }
        if !answered {
            if partition != 0 && e.mem.u16(partition.wrapping_add(0x20)) > 4 {
                return None;
            }
            e.mem.set_u32(l + 0x20, source);
            e.mem.set_u32(count, 1);
            let group = e.get(this, NiDX9Renderer::object_8bc);
            e.call(
                GROUP_PREPARE_00E8C120,
                &args![
                    group,
                    object,
                    partition,
                    first,
                    second,
                    l + 0x1c,
                    l + 0x18,
                    l + 0x14,
                    l + 0x10,
                    l + 0x20
                ],
            );
        }
        Some((
            e.mem.u32(count),
            e.mem.u32(first),
            e.mem.u32(second),
            e.mem.u32(source_object),
        ))
    });
    let Some((count, first, second, source_object)) = prepared else {
        return false;
    };
    e.call(DATA_SET_COUNT_00E8EF10, &args![data, count]);
    if first != 0 {
        e.call(DATA_SET_FIRST_00E8F010, &args![data, first]);
        if e.mem.u32(d.wrapping_add(0x20)) > 0 {
            let slots = e.mem.u32(d.wrapping_add(0x24));
            e.mem.set_u32(slots, second);
        }
    } else {
        e.call(DATA_SET_SECOND_00E8F040, &args![data, source_object]);
        let mut index = 0u32;
        while index < count {
            let stream = e.vcall(source, 0xa0, &args![index]).u32();
            if index < e.mem.u32(d.wrapping_add(0x20)) {
                let slots = e.mem.u32(d.wrapping_add(0x24));
                e.mem.set_u32(slots.wrapping_add(index * 4), stream);
            }
            index += 1;
        }
    }
    let has_28 = e.mem.u32(o.wrapping_add(0x28)) != 0;
    let has_24 = e.mem.u32(o.wrapping_add(0x24)) != 0;
    let top_bits = has_24 && e.mem.u16(o.wrapping_add(0xc)) & 0xf000 != 0;
    let mut low = u32::from(e.mem.u16(o.wrapping_add(0xc))) & 1;
    let limit = e.global::<u32>(STREAM_LIMIT_011BF558);
    if low > limit {
        low = limit;
    }
    let mut flags = 0u32;
    if has_28 {
        flags = 0x40_0000;
    }
    if has_24 {
        flags |= 0x80_0000;
    }
    if top_bits {
        flags |= 0x20_0000;
    }
    e.mem.set_u32(d.wrapping_add(4), (low << 24) | flags);
    if source != 0 && e.mem.u8(t.wrapping_add(0x5d4)) != 0 {
        let value = e.vcall(source, 0xa8, &args![]).u8();
        e.mem.set_u8(d.wrapping_add(0x14), value);
    }
    let mut index = 0u32;
    while index < count {
        let group = e.get(this, NiDX9Renderer::object_8bc);
        if e.call(GROUP_STREAM_00E8BFA0, &args![group, data, index])
            .u8()
            == 0
        {
            while index != 0 {
                let owner = e.mem.u32(d.wrapping_add(8));
                e.vcall(owner, 0x1c, &args![data, index]);
                index -= 1;
            }
            return false;
        }
        index += 1;
    }
    true
}

/// A geometry record (0x24 bytes) that `00e72980` and `00e72aa0` chain per key in the map at
/// +0x610 (the previous head at +0x20).
const RECORD_SIZE: u32 = 0x24;

/// Records one stream key of `data` in the map at +0x610: a new record (filled by `fill`)
/// whose +0x20 links the key's previous record (when the map had one), stored under the key.
fn chain_record(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    key: u32,
    fill: impl FnOnce(&mut Engine, u32),
) {
    let record = e.call(NEW_OBJECT, &args![RECORD_SIZE]).u32();
    fill(e, record);
    let map = this.addr().wrapping_add(0x610);
    let (hit, previous) = e.with_stack(4, |e, slot| {
        let hit = e.call(MAP_GET_AT, &args![map, key, slot]).u8() != 0;
        (hit, e.mem.u32(slot.addr()))
    });
    e.mem
        .set_u32(record.wrapping_add(0x20), if hit { previous } else { 0 });
    e.call(MAP_SET_AT, &args![map, key, record]);
}

/// The key of stream `index` of `data` (+0x28's element, its word +4; the element is 0 past
/// the count at +0x20).
fn stream_key(e: &Engine, data: u32, index: u32) -> u32 {
    let element = if index < e.mem.u32(data.wrapping_add(0x20)) {
        e.mem
            .u32(e.mem.u32(data.wrapping_add(0x28)).wrapping_add(index * 4))
    } else {
        0
    };
    e.mem.u32(element.wrapping_add(4))
}

// Translated from 00e72980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: precaches one geometry's buffers (`thiscall`, eight words; the
/// first is not read). True at once when the object's data (+0x34) is packed (`00e8eeb0`).
/// Else the data takes the object's virtual +0x90 (+0x18), its word +8 (+0x1C), the third
/// and fourth words as 16-bit values (+0x40, +0x44), the fifth and sixth (+0x50, +0x4C) and
/// the seventh as a 16-bit value (+0x48); `00e6da60` prepares it (with no partition and the
/// eighth word as source; false ends here) and each of its streams is recorded under its key
/// (object, source, the data and the stream index).
pub fn fn_00e72980(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    _unused_1: u32,
    object: Ptr,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
    source: u32,
) -> bool {
    let o = object.addr();
    let data = e.mem.u32(o.wrapping_add(0x34));
    if e.call(DATA_IS_PACKED_00E8EEB0, &args![data]).u8() != 0 {
        return true;
    }
    let vertices = u32::from(e.vcall(o, 0x90, &args![]).u16());
    let word_8 = u32::from(e.mem.u16(o.wrapping_add(8)));
    e.mem.set_u32(data.wrapping_add(0x1c), word_8);
    e.mem.set_u32(data.wrapping_add(0x18), vertices);
    e.mem.set_u32(data.wrapping_add(0x50), fifth);
    e.mem.set_u32(data.wrapping_add(0x44), fourth & 0xffff);
    e.mem.set_u32(data.wrapping_add(0x40), third & 0xffff);
    e.mem.set_u32(data.wrapping_add(0x4c), sixth);
    e.mem.set_u32(data.wrapping_add(0x48), seventh & 0xffff);
    let prepared = fn_00e6da60(e, this, Ptr::new(data), object, 0, source);
    if !prepared {
        return false;
    }
    let count = e.mem.u32(data.wrapping_add(0x20));
    let mut index = 0u32;
    while index < count {
        let key = stream_key(e, data, index);
        chain_record(e, this, key, |e, record| {
            e.mem.set_u32(record, o);
            e.mem.set_u32(record.wrapping_add(0xc), source);
            for offset in [4, 8, 0x10, 0x14] {
                e.mem.set_u32(record.wrapping_add(offset), 0);
            }
            e.mem.set_u32(record.wrapping_add(0x18), data);
            e.mem.set_u32(record.wrapping_add(0x1c), index);
        });
        index += 1;
    }
    prepared
}

// Translated from 00e72aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: precaches a skinned geometry's partitions (`thiscall`, six
/// words; the first is not read). For each 0x2C-byte partition of the third word's +0xC
/// object (count +8, array +0xC) whose data (+0x28) is not packed: the data takes the
/// partition's 16-bit words +0x1C (+0x18 and +0x1C), +0x22 (+0x48; 0 becomes 1) and +0x1E
/// (+0x40, +0x44) and its words +0x14 (+0x50) and +0x18 (+0x4C); `00e6da60` prepares it with
/// the second word, the partition and the fourth word (a failure makes the answer false and
/// moves on), and each stream is recorded under its key (second, third, partition, fourth,
/// fifth and sixth words, the data, the stream index). Answers true unless a preparation
/// failed.
pub fn fn_00e72aa0(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    _unused_1: u32,
    object: u32,
    skin: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> bool {
    let instance = e.mem.u32(skin.wrapping_add(0xc));
    let partitions = e.mem.u32(instance.wrapping_add(0xc));
    let mut remaining = e.mem.u32(instance.wrapping_add(8));
    let mut result = true;
    if remaining == 0 {
        return result;
    }
    let mut at = partitions.wrapping_add(0x1c);
    loop {
        let data = e.mem.u32(at.wrapping_add(0xc));
        if e.call(DATA_IS_PACKED_00E8EEB0, &args![data]).u8() == 0 {
            let vertices = u32::from(e.mem.u16(at));
            let strips = e.mem.u16(at.wrapping_add(6));
            let bones = u32::from(e.mem.u16(at.wrapping_add(2)));
            let word_14 = e.mem.u32(at.wrapping_sub(8));
            let word_18 = e.mem.u32(at.wrapping_sub(4));
            e.mem.set_u32(data.wrapping_add(0x18), vertices);
            e.mem.set_u32(data.wrapping_add(0x1c), vertices);
            let strips = if strips == 0 { 1 } else { u32::from(strips) };
            e.mem.set_u32(data.wrapping_add(0x48), strips);
            e.mem.set_u32(data.wrapping_add(0x40), bones);
            e.mem.set_u32(data.wrapping_add(0x44), bones);
            e.mem.set_u32(data.wrapping_add(0x50), word_14);
            e.mem.set_u32(data.wrapping_add(0x4c), word_18);
            let partition = at.wrapping_sub(0x1c);
            if fn_00e6da60(e, this, Ptr::new(data), Ptr::new(object), partition, fourth) {
                let count = e.mem.u32(data.wrapping_add(0x20));
                let mut index = 0u32;
                while index < count {
                    let key = stream_key(e, data, index);
                    chain_record(e, this, key, |e, record| {
                        e.mem.set_u32(record.wrapping_add(8), partition);
                        e.mem.set_u32(record, object);
                        e.mem.set_u32(record.wrapping_add(0xc), fourth);
                        e.mem.set_u32(record.wrapping_add(0x10), fifth);
                        e.mem.set_u32(record.wrapping_add(4), skin);
                        e.mem.set_u32(record.wrapping_add(0x14), sixth);
                        e.mem.set_u32(record.wrapping_add(0x20), 0);
                        e.mem.set_u32(record.wrapping_add(0x18), data);
                        e.mem.set_u32(record.wrapping_add(0x1c), index);
                    });
                    index += 1;
                }
            } else {
                result = false;
            }
        }
        at = at.wrapping_add(0x2c);
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
    }
    result
}

/// `NiD3DGeometryGroupManager::RemoveSkinPartitionFromGroup` (`00e88d00`, `thiscall` on the
/// object at +0x8AC, one word).
const REMOVE_SKIN_PARTITION_FROM_GROUP: u32 = 0x00e8_8d00;

/// Takes out of the map at +0x610 every record under `key` whose word at `owner_offset` is
/// `owner`, as `00e71cf0` and `00e71e40` inline it: a removed first record's successor takes
/// its place in the map (or the key goes); the record is freed (0x24 bytes).
fn unlink_records(e: &mut Engine, this: u32, key: u32, owner_offset: u32, owner: u32) {
    let map = this.wrapping_add(0x610);
    let (hit, head) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let hit = e.call(MAP_GET_AT, &args![map, key, slot]).u8() != 0;
        (hit, e.mem.u32(slot.addr()))
    });
    if !hit {
        return;
    }
    let mut record = head;
    let mut previous = 0u32;
    while record != 0 {
        if e.mem.u32(record.wrapping_add(owner_offset)) == owner {
            let next;
            if previous != 0 {
                let after = e.mem.u32(record.wrapping_add(0x20));
                e.mem.set_u32(previous.wrapping_add(0x20), after);
                next = e.mem.u32(record.wrapping_add(0x20));
            } else if e.mem.u32(record.wrapping_add(0x20)) != 0 {
                let after = e.mem.u32(record.wrapping_add(0x20));
                e.call(MAP_SET_AT, &args![map, key, after]);
                next = e.mem.u32(record.wrapping_add(0x20));
            } else {
                e.call(MAP_REMOVE_AT, &args![map, key]);
                next = 0;
            }
            e.mem.set_u32(record.wrapping_add(0x20), 0);
            e.call(SIZED_DELETE, &args![record, RECORD_SIZE]);
            record = next;
        } else {
            previous = record;
            record = e.mem.u32(record.wrapping_add(0x20));
        }
    }
}

// Translated from 00e71e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x100): purges a skin instance's partitions
/// (`thiscall`, one word). Only for an instance whose partition object (+0xC) has data on its
/// first partition (+0x28): for each of the count (+8) 0x2C-byte partitions whose data has
/// streams starting with a set one, under the locks at +0x80 and +0x100 the records of that
/// partition (record +8) leave the map at +0x610 under each stream's key. Then the group
/// manager at +0x8AC removes the instance (`RemoveSkinPartitionFromGroup`).
pub fn fn_00e71e40(e: &mut Engine, this: Ptr<NiDX9Renderer>, instance: u32) {
    let t = this.addr();
    if instance == 0 {
        return;
    }
    let partitions = e.mem.u32(instance.wrapping_add(0xc));
    if partitions == 0 || e.mem.u32(partitions.wrapping_add(0x28)) == 0 {
        return;
    }
    let mut remaining = e.mem.u32(instance.wrapping_add(8));
    if remaining != 0 {
        let mut partition = partitions;
        loop {
            let data = e.mem.u32(partition.wrapping_add(0x28));
            if e.mem.u32(data.wrapping_add(0x20)) != 0
                && e.mem.u32(e.mem.u32(data.wrapping_add(0x28))) != 0
            {
                let (first_lock, second_lock) = (t.wrapping_add(0x80), t.wrapping_add(0x100));
                e.call(ENTER_CRITICAL_SECTION, &args![first_lock]);
                e.call(ENTER_CRITICAL_SECTION, &args![second_lock]);
                let mut index = 0u32;
                if e.mem.u32(data.wrapping_add(0x20)) != 0 {
                    loop {
                        let array = e.mem.u32(data.wrapping_add(0x28));
                        let element = e.mem.u32(array.wrapping_add(index * 4));
                        let key = e.mem.u32(element.wrapping_add(4));
                        unlink_records(e, t, key, 8, partition);
                        index += 1;
                        if index >= e.mem.u32(data.wrapping_add(0x20)) {
                            break;
                        }
                    }
                }
                e.call(LEAVE_CRITICAL_SECTION, &args![second_lock]);
                e.call(LEAVE_CRITICAL_SECTION, &args![first_lock]);
            }
            partition = partition.wrapping_add(0x2c);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
    let manager = e.mem.u32(t.wrapping_add(0x8ac));
    e.call(REMOVE_SKIN_PARTITION_FROM_GROUP, &args![manager, instance]);
}

/// `00e6dfc0` (`thiscall`, three words: a format table, a word, a count): fills a table of
/// formats.
/// `00a5b690` (`cdecl`, one word: a C string): an `NiFixedString` handle for the string.
/// `NiSourceTexture::Create_ov2` (`00a5ff10`, `cdecl`: pixel data, name, &format prefs).
/// `"Clipper Image"`, the clipper texture's name.
/// The `NiFixedString` handle whose count is not kept (`0109b220`).
/// The clipper texture (`NiPointer` at +0x8FC).
#[cfg(test)]
const CLIPPER_TEXTURE: u32 = 0x8fc;

/// `NiGeometry`'s RTTI record (`011f5b70`), the two classes `PrecacheGeometry` packs as
/// triangle lists (`011f4a40`) or strips (`011f4a20`), and their data's `GetTriList`
/// (`00a7b850`) and `GetStripLists` (`00a75f10`; `thiscall`).
const GEOMETRY_RTTI: u32 = 0x011f_5b70;
const TRI_SHAPE_RTTI_011F4A40: u32 = 0x011f_4a40;
const TRI_STRIPS_RTTI_011F4A20: u32 = 0x011f_4a20;
const TRI_SHAPE_DATA_GET_TRI_LIST: u32 = 0x00a7_b850;
const TRI_STRIPS_DATA_GET_STRIP_LISTS: u32 = 0x00a7_5f10;
/// `NiD3DGeometryGroupManager::AddObjectToGroup_ov2` (`00e88dc0`, `thiscall` on the manager
/// at +0x8AC, six words).
const ADD_OBJECT_TO_GROUP: u32 = 0x00e8_8dc0;
/// `"PrecacheGeometry> No shader found for object %s!\n"`.
const NO_SHADER_TEXT: u32 = 0x010e_e404;

// Translated from 00e73f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PrecacheGeometry` (the engine map's name; renderer vtable +0xEC):
/// `thiscall`, four words (the geometry, two words for the group, a source or 0). False for
/// no geometry or one that is not an `NiGeometry` (its RTTI chain); false with a message for
/// one without a shader (+0xC0). Else, under the locks at +0x80 and +0x100: whether it is
/// skinned (`00e72c20`), the source (or the shader's virtual +0xBC), the group manager at
/// +0x8AC adds it (`AddObjectToGroup`: the group at +0x8B0, the data +0xB8, the skin +0xBC,
/// the skinned flag, the two words); a skinned geometry packs its partitions (`00e72aa0`),
/// a triangle shape or strips its data (`00e72980` with the data's virtual +0xA8, its word
/// +0x40, its triangle list or strip lists, and 0/1 or the strips' words +0x48/+0x44). The
/// answer is the packing's (false for other classes).
///
/// The skinned flag is a byte local passed as a whole stack word; its upper bytes are not set
/// in the exe (here 0).
pub fn ni_xenon_renderer_precache_geometry(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    geometry: u32,
    first: u32,
    second: u32,
    source: u32,
) -> bool {
    let t = this.addr();
    if geometry == 0 {
        return false;
    }
    let mut rtti = e.vcall(geometry, 8, &args![]).u32();
    loop {
        if rtti == 0 {
            return false;
        }
        if rtti == GEOMETRY_RTTI {
            break;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    let skin = e.mem.u32(geometry.wrapping_add(0xbc));
    let shader = e.mem.u32(geometry.wrapping_add(0xc0));
    let data = e.mem.u32(geometry.wrapping_add(0xb8));
    if shader == 0 {
        let name = e.mem.u32(geometry.wrapping_add(8));
        e.call(REPORT_00A29680, &args![NO_SHADER_TEXT, name]);
        return false;
    }
    let skinned = fn_00e72c20(e, this, Ptr::new(geometry), 0);
    let first_lock = t.wrapping_add(0x80);
    e.call(ENTER_CRITICAL_SECTION, &args![first_lock]);
    let second_lock = t.wrapping_add(0x100);
    e.call(ENTER_CRITICAL_SECTION, &args![second_lock]);
    let mut source = source;
    if source == 0 {
        source = e.vcall(shader, 0xbc, &args![]).u32();
    }
    let manager = e.mem.u32(t.wrapping_add(0x8ac));
    let group = e.mem.u32(t.wrapping_add(0x8b0));
    e.call(
        ADD_OBJECT_TO_GROUP,
        &args![
            manager,
            group,
            data,
            skin,
            u32::from(skinned),
            first,
            second
        ],
    );
    let mut result = false;
    if skinned {
        result = fn_00e72aa0(e, this, geometry, data, skin, source, first, second);
    } else if e
        .call(IS_KIND_OF, &args![TRI_SHAPE_RTTI_011F4A40, geometry])
        .u8()
        != 0
    {
        let count = u32::from(e.vcall(data, 0xa8, &args![]).u16());
        let word_40 = u32::from(e.mem.u16(data.wrapping_add(0x40)));
        let list = e.call(TRI_SHAPE_DATA_GET_TRI_LIST, &args![data]).u32();
        result = fn_00e72980(
            e,
            this,
            geometry,
            Ptr::new(data),
            count,
            word_40,
            list,
            0,
            1,
            source,
        );
    } else if e
        .call(IS_KIND_OF, &args![TRI_STRIPS_RTTI_011F4A20, geometry])
        .u8()
        != 0
    {
        let count = u32::from(e.vcall(data, 0xa8, &args![]).u16());
        let word_40 = u32::from(e.mem.u16(data.wrapping_add(0x40)));
        let lists = e.call(TRI_STRIPS_DATA_GET_STRIP_LISTS, &args![data]).u32();
        let word_48 = e.mem.u32(data.wrapping_add(0x48));
        let word_44 = u32::from(e.mem.u16(data.wrapping_add(0x44)));
        result = fn_00e72980(
            e,
            this,
            geometry,
            Ptr::new(data),
            count,
            word_40,
            lists,
            word_48,
            word_44,
            source,
        );
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![second_lock]);
    e.call(LEAVE_CRITICAL_SECTION, &args![first_lock]);
    result
}

/// `_ftol2_sse` (`00ec62c0`): truncates ST0 (a leading `f64` argument) to an integer in EAX.
const FTOL: u32 = 0x00ec_62c0;
/// `4294967296.0` (`float`), added to a negative `FILD` of an unsigned value.
/// The record `00e6f310` checks the render target's depth buffer against (`0126f5ac`) and its
/// two queries (`00e7cdf0`: has a depth buffer with a stencil; `00e7ce10`: has a depth
/// buffer; `thiscall`).
const DEPTH_BUFFER_RTTI_0126F5AC: u32 = 0x0126_f5ac;
const DEPTH_HAS_STENCIL_00E7CDF0: u32 = 0x00e7_cdf0;
const DEPTH_HAS_DEPTH_00E7CE10: u32 = 0x00e7_ce10;
/// `IDirect3DDevice9::Clear` (slot 43) and `SetViewport` (slot 47).
const DEVICE_CLEAR: u32 = 0xac;
const DEVICE_SET_VIEWPORT: u32 = 0xbc;

/// `FILD` of an unsigned word as the code does it (`FILD`; plus 2^32 when negative), stored
/// as a float.
fn unsigned_to_float(e: &Engine, value: u32) -> u32 {
    let mut x = f64::from(value as i32);
    if (value as i32) < 0 {
        x = x87_add(x, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    x87_store32(x)
}

// Translated from 00e6f310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x188, `Do_ClearBuffer`'s slot on Xbox):
/// `thiscall`, two words (a rectangle of four floats or 0, the buffers to clear: bit 0 the
/// target, bit 1 the stencil, bit 2 the depth). Nothing without buffers, with the byte at
/// +0x6FC set or without a render target group (+0x888). The rectangle (left, top, right,
/// bottom as fractions) becomes a `D3DRECT` of the target's size (virtuals +0x8C, +0x90 with
/// 0): `left·w`, `(1 − top)·h`, `right·w`, `(1 − bottom)·h`, truncated; without one the whole
/// target. The depth and stencil bits count only when the group's depth buffer (virtual
/// +0xCC, of the class `0126f5ac`) has them. The viewport (+0x6CC; width and height at
/// +0x6D4, +0x6D8 from the target) is set and the device clears with the colour (+0x5E0),
/// depth (+0x5E4) and stencil (+0x5E8).
pub fn fn_00e6f310(e: &mut Engine, this: Ptr<NiDX9Renderer>, rect: u32, buffers: u32) {
    let t = this.addr();
    if buffers == 0 || e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let group = e.get(this, NiDX9Renderer::target_group);
    if group == 0 {
        return;
    }
    let width = e.vcall(group, 0x8c, &args![0u32]).u32();
    let group = e.get(this, NiDX9Renderer::target_group);
    let height = e.vcall(group, 0x90, &args![0u32]).u32();
    let corners: [u32; 4] = if rect != 0 {
        let w = x87_load32(unsigned_to_float(e, width));
        let h = x87_load32(unsigned_to_float(e, height));
        let left = x87_mul(x87_load32(e.mem.u32(rect)), w);
        let x1 = e.call(FTOL, &args![left]).u32();
        let top = x87_mul(x87_sub(1.0, x87_load32(e.mem.u32(rect + 8))), h);
        let y1 = e.call(FTOL, &args![top]).u32();
        let right = x87_mul(w, x87_load32(e.mem.u32(rect + 4)));
        let x2 = e.call(FTOL, &args![right]).u32();
        let bottom = x87_mul(x87_sub(1.0, x87_load32(e.mem.u32(rect + 0xc))), h);
        let y2 = e.call(FTOL, &args![bottom]).u32();
        [x1, y1, x2, y2]
    } else {
        [0, 0, width, height]
    };
    let mut flags = buffers & 1;
    let group = e.get(this, NiDX9Renderer::target_group);
    let depth = e.vcall(group, 0xcc, &args![]).u32();
    if depth != 0 {
        let buffer = fn_00e69b00(e, Ptr::new(depth), DEPTH_BUFFER_RTTI_0126F5AC);
        if buffer != 0 {
            if buffers & 4 != 0 && e.call(DEPTH_HAS_STENCIL_00E7CDF0, &args![buffer]).u8() != 0 {
                flags |= 2;
            }
            if buffers & 2 != 0 && e.call(DEPTH_HAS_DEPTH_00E7CE10, &args![buffer]).u8() != 0 {
                flags |= 4;
            }
        }
    }
    let group = e.get(this, NiDX9Renderer::target_group);
    let viewport_width = e.vcall(group, 0x8c, &args![0u32]).u32();
    e.mem.set_u32(t + 0x6d4, viewport_width);
    let group = e.get(this, NiDX9Renderer::target_group);
    let viewport_height = e.vcall(group, 0x90, &args![0u32]).u32();
    e.mem.set_u32(t + 0x6d8, viewport_height);
    let device = e.get(this, NiDX9Renderer::device);
    e.vcall(device, DEVICE_SET_VIEWPORT, &args![t + 0x6cc]);
    let z = x87_quiet_bits(e.mem.u32(t + 0x5e4));
    let device = e.get(this, NiDX9Renderer::device);
    let stencil = e.mem.u32(t + 0x5e8);
    let color = e.get(this, NiDX9Renderer::background_color);
    e.with_stack(0x10, |e, area| {
        for (i, v) in corners.iter().enumerate() {
            e.mem.set_u32(area.addr() + 4 * i as u32, *v);
        }
        e.vcall(
            device,
            DEVICE_CLEAR,
            &args![1u32, area, flags, color, z, stencil],
        );
    });
}

/// `IDirect3DDevice9::DrawPrimitive` (slot 81) and `DrawIndexedPrimitive` (slot 82).
const DEVICE_DRAW_PRIMITIVE: u32 = 0x144;
const DEVICE_DRAW_INDEXED_PRIMITIVE: u32 = 0x148;
/// The bound `00e74840` copies when the geometry has none (`011f4288`).
const DEFAULT_BOUND: u32 = 0x011f_4288;
/// The renderer's copy of the drawn object's bound (centre at +0x664, radius at +0x670).
const DRAWN_BOUND: u32 = 0x664;

// Translated from 00e74840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1B8): draws a packed geometry (`thiscall`,
/// one word). Nothing with the byte at +0x6FC set or for data (+0xB8) without vertices
/// (virtual +0x90). The bound (+0x20, or the one at `011f4288`) is copied to +0x664; the
/// shader's virtual +0x90 is called. A skinned geometry (`00e72c20`) draws each partition's
/// strips (or its triangle list) with `DrawIndexedPrimitive` (type 5 when the first
/// partition has strips, else 4; the partition's base vertex +0x38 of its data, its vertex
/// count, running start indices); else the renderer data (+0x34 of the geometry data) draws
/// indexed (with an index buffer at +0x34) or not (`DrawPrimitive`). Then the data's word
/// +0xE keeps only its top four bits.
pub fn fn_00e74840(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    let g = geometry.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let data = e.mem.u32(g.wrapping_add(0xb8));
    if e.vcall(data, 0x90, &args![]).u16() == 0 {
        return;
    }
    // Diagnostic only (`NV_HEAD_CHECK`, not exe behaviour): notes the geometry's skin and data.
    if super::run_00e6fbb0::head_check_enabled() {
        super::run_00e6fbb0::head_check_note_geometry(e, g);
    }
    let skin = e.mem.u32(g.wrapping_add(0xbc));
    let bound = match e.mem.u32(g.wrapping_add(0x20)) {
        0 => DEFAULT_BOUND,
        bound => bound,
    };
    let center_y = e.mem.u32(bound.wrapping_add(4));
    let center_z = e.mem.u32(bound.wrapping_add(8));
    let center_x = e.mem.u32(bound);
    let radius = e.mem.u32(bound.wrapping_add(0xc));
    let skinned = fn_00e72c20(e, this, geometry, 0);
    let render_data = e
        .mem
        .u32(e.mem.u32(g.wrapping_add(0xb8)).wrapping_add(0x34));
    let shader = e.mem.u32(g.wrapping_add(0xc0));
    e.vcall(shader, 0x90, &args![]);
    e.mem.set_u32(t + DRAWN_BOUND + 0xc, x87_quiet_bits(radius));
    e.mem.set_u32(t + DRAWN_BOUND, center_x);
    e.mem.set_u32(t + DRAWN_BOUND + 4, center_y);
    e.mem.set_u32(t + DRAWN_BOUND + 8, center_z);
    if skinned {
        let instance = e.mem.u32(skin.wrapping_add(0xc));
        let mut remaining = e.mem.u32(instance.wrapping_add(8));
        let partitions = e.mem.u32(instance.wrapping_add(0xc));
        let kind = u32::from(e.mem.u16(partitions.wrapping_add(0x22)) != 0) + 4;
        if remaining != 0 {
            let mut at = partitions.wrapping_add(0x1c);
            loop {
                let part_data = e.mem.u32(at.wrapping_add(0xc));
                let mut strip = 0u32;
                let mut start = 0u32;
                while strip < e.mem.u32(part_data.wrapping_add(0x48)) {
                    let lengths = e.mem.u32(part_data.wrapping_add(0x4c));
                    let count = if lengths != 0 {
                        u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
                    } else {
                        e.mem.u32(part_data.wrapping_add(0x40))
                    };
                    let vertices = u32::from(e.mem.u16(at));
                    let device = e.get(this, NiDX9Renderer::device);
                    let base = e.mem.u32(part_data.wrapping_add(0x38));
                    e.vcall(
                        device,
                        DEVICE_DRAW_INDEXED_PRIMITIVE,
                        &args![kind, base, 0u32, vertices, start, count],
                    );
                    strip += 1;
                    start = start.wrapping_add(count).wrapping_add(2);
                }
                at = at.wrapping_add(0x2c);
                remaining = remaining.wrapping_sub(1);
                if remaining == 0 {
                    break;
                }
            }
        }
    } else if e.mem.u32(render_data.wrapping_add(0x34)) != 0 {
        let mut strip = 0u32;
        let mut start = 0u32;
        while strip < e.mem.u32(render_data.wrapping_add(0x48)) {
            let lengths = e.mem.u32(render_data.wrapping_add(0x4c));
            let count = if lengths != 0 {
                u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
            } else {
                e.mem.u32(render_data.wrapping_add(0x40))
            };
            let vertices = e.mem.u32(render_data.wrapping_add(0x18));
            let device = e.get(this, NiDX9Renderer::device);
            let base = e.mem.u32(render_data.wrapping_add(0x38));
            let kind = e.mem.u32(render_data.wrapping_add(0x3c));
            e.vcall(
                device,
                DEVICE_DRAW_INDEXED_PRIMITIVE,
                &args![kind, base, 0u32, vertices, start, count],
            );
            strip += 1;
            start = start.wrapping_add(count).wrapping_add(2);
        }
    } else {
        let count = e.mem.u32(render_data.wrapping_add(0x40));
        let first = e.mem.u32(render_data.wrapping_add(0x38));
        let device = e.get(this, NiDX9Renderer::device);
        let kind = e.mem.u32(render_data.wrapping_add(0x3c));
        e.vcall(device, DEVICE_DRAW_PRIMITIVE, &args![kind, first, count]);
    }
    let flags = e.mem.u16(data.wrapping_add(0xe));
    e.mem.set_u16(data.wrapping_add(0xe), flags & 0xf000);
}

/// The caps `GetDeviceCaps` fills (+0x28C), as the tests name them.
#[cfg(test)]
const DEVICE_CAPS: u32 = 0x28c;

/// `00e7cc50` (`cdecl`: the device, a render-target index): clears that render target, and
/// `00e7cca0` (`cdecl`: the device): clears the depth/stencil buffer (true on success).
const CLEAR_RENDER_TARGET_00E7CC50: u32 = 0x00e7_cc50;
const CLEAR_DEPTH_STENCIL_00E7CCA0: u32 = 0x00e7_cca0;
/// The classes `00e6eea0` checks: a render-target buffer (`0126f59c`), a depth/stencil
/// buffer (`0126f5ac`) and the default one's (`0126f5d8`).
const RENDER_BUFFER_RTTI_0126F59C: u32 = 0x0126_f59c;
/// `"NiDX9Renderer::BeginUsingRenderTargetGroup> Failed - %s"` and its three reasons (an
/// output target, the depth/stencil buffer, the depth/stencil buffer set to none).
const BEGIN_GROUP_FAILED_TEXT: u32 = 0x010e_dc58;
const OUTPUT_TARGET_FAILED_TEXT: u32 = 0x010e_dc18;
const DEPTH_STENCIL_FAILED_TEXT: u32 = 0x010e_dc90;
const DEPTH_STENCIL_NULL_FAILED_TEXT: u32 = 0x010e_dbc8;

/// Whether `object`'s RTTI chain (virtual +0xC, links at +4) has `rtti`: the object, or 0.
fn kind_of(e: &mut Engine, object: u32, rtti: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    fn_00e69b00(e, Ptr::new(object), rtti)
}

/// The failure path of `00e6eea0`: back to the default group (+0x884) through the
/// renderer's own virtual +0x194, then the message.
fn begin_group_failed(e: &mut Engine, this: Ptr<NiDX9Renderer>, clear: u32, reason: u32) -> bool {
    let default = e.get(this, NiDX9Renderer::object_884);
    e.vcall(this.addr(), 0x194, &args![default, clear]);
    e.call(REPORT_00A29680, &args![BEGIN_GROUP_FAILED_TEXT, reason]);
    false
}

// Translated from 00e6eea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x194, `Do_BeginUsingRenderTargetGroup`'s
/// slot on Xbox): `thiscall`, two words (the group, the buffers to clear). True at once with
/// the byte +0x6FC set. Clears render targets 1..=3 (`00e7cc50`), sets each of the group's
/// buffers (virtual +0xA4 count, +0xC8 buffer) of the render-buffer class as target (its
/// virtual +0x5C with the device and the index) and the depth/stencil buffer (+0xCC; its
/// virtual +0x60, or `00e7cca0` without one); a failure goes back to the default group and
/// answers false with its message. Then stores the group at +0x888 (and at +0x88C when its
/// first buffer is of the default class) and clears it (the renderer's virtual +0x188 with
/// no rectangle). Answers true.
pub fn fn_00e6eea0(e: &mut Engine, this: Ptr<NiDX9Renderer>, group: u32, clear: u32) -> bool {
    let t = this.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return true;
    }
    for target in 1..=3u32 {
        let device = e.get(this, NiDX9Renderer::device);
        e.call(CLEAR_RENDER_TARGET_00E7CC50, &args![device, target]);
    }
    let mut index = 0u32;
    if e.vcall(group, 0xa4, &args![]).u32() != 0 {
        loop {
            let buffer = e.vcall(group, 0xc8, &args![index]).u32();
            let target = kind_of(e, buffer, RENDER_BUFFER_RTTI_0126F59C);
            if target != 0 {
                let device = e.get(this, NiDX9Renderer::device);
                if e.vcall(target, 0x5c, &args![device, index]).u8() == 0 {
                    return begin_group_failed(e, this, clear, OUTPUT_TARGET_FAILED_TEXT);
                }
            }
            index += 1;
            if index >= e.vcall(group, 0xa4, &args![]).u32() {
                break;
            }
        }
    }
    let depth = e.vcall(group, 0xcc, &args![]).u32();
    let depth_buffer = kind_of(e, depth, DEPTH_BUFFER_RTTI_0126F5AC);
    if depth_buffer != 0 {
        let device = e.get(this, NiDX9Renderer::device);
        if e.vcall(depth_buffer, 0x60, &args![device]).u8() == 0 {
            return begin_group_failed(e, this, clear, DEPTH_STENCIL_FAILED_TEXT);
        }
    } else {
        let device = e.get(this, NiDX9Renderer::device);
        if e.call(CLEAR_DEPTH_STENCIL_00E7CCA0, &args![device]).u8() == 0 {
            return begin_group_failed(e, this, clear, DEPTH_STENCIL_NULL_FAILED_TEXT);
        }
    }
    let first = e.vcall(group, 0xc8, &args![0u32]).u32();
    let default = kind_of(e, first, SHADER_RTTI_0126F5D8);
    e.set(this, NiDX9Renderer::target_group, group);
    if default != 0 {
        e.mem.set_u32(t + 0x88c, group);
    }
    e.vcall(t, 0x188, &args![0u32, clear]);
    true
}

/// `float` 1.0 bits (the code stores `FLD1`).
#[cfg(test)]
const ONE: u32 = 0x3f80_0000;

// Translated from 00e745a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1B4): draws a packed geometry with its
/// partial-draw records (`thiscall`, one word). As `00e74840`, with two differences: a
/// skinned partition without strip lengths draws the count at +0xC4 of the geometry's
/// virtual +0x2C object when there is one; an unskinned indexed strip with a draw object
/// (the geometry's virtual +0x28) draws its active records instead (0x14 bytes each from
/// +0xC4, active when byte +0x10 is set: start +0, count +0xC; one record with the start 0
/// and the count of the object's data word +0x40 when its byte +0xCD is set).
pub fn fn_00e745a0(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    let g = geometry.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let data = e.mem.u32(g.wrapping_add(0xb8));
    if e.vcall(data, 0x90, &args![]).u16() == 0 {
        return;
    }
    let skin = e.mem.u32(g.wrapping_add(0xbc));
    let skinned = fn_00e72c20(e, this, geometry, 0);
    let shader = e.mem.u32(g.wrapping_add(0xc0));
    let render_data = e
        .mem
        .u32(e.mem.u32(g.wrapping_add(0xb8)).wrapping_add(0x34));
    e.vcall(shader, 0x90, &args![]);
    if skinned {
        let instance = e.mem.u32(skin.wrapping_add(0xc));
        let mut remaining = e.mem.u32(instance.wrapping_add(8));
        let partitions = e.mem.u32(instance.wrapping_add(0xc));
        let kind = u32::from(e.mem.u16(partitions.wrapping_add(0x22)) != 0) + 4;
        if remaining != 0 {
            let mut at = partitions.wrapping_add(0x1c);
            loop {
                let part_data = e.mem.u32(at.wrapping_add(0xc));
                let mut strip = 0u32;
                let mut start = 0u32;
                while strip < e.mem.u32(part_data.wrapping_add(0x48)) {
                    let lengths = e.mem.u32(part_data.wrapping_add(0x4c));
                    let count = if lengths != 0 {
                        u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
                    } else {
                        let mut count = e.mem.u32(part_data.wrapping_add(0x40));
                        if e.vcall(g, 0x2c, &args![]).u32() != 0 {
                            let partial = e.vcall(g, 0x2c, &args![]).u32();
                            count = e.mem.u32(partial.wrapping_add(0xc4));
                        }
                        count
                    };
                    let vertices = u32::from(e.mem.u16(at));
                    let device = e.get(this, NiDX9Renderer::device);
                    let base = e.mem.u32(part_data.wrapping_add(0x38));
                    e.vcall(
                        device,
                        DEVICE_DRAW_INDEXED_PRIMITIVE,
                        &args![kind, base, 0u32, vertices, start, count],
                    );
                    strip += 1;
                    start = start.wrapping_add(count).wrapping_add(2);
                }
                at = at.wrapping_add(0x2c);
                remaining = remaining.wrapping_sub(1);
                if remaining == 0 {
                    break;
                }
            }
        }
    } else if e.mem.u32(render_data.wrapping_add(0x34)) != 0 {
        let mut start = 0u32;
        let mut strip = 0u32;
        while strip < e.mem.u32(render_data.wrapping_add(0x48)) {
            let lengths = e.mem.u32(render_data.wrapping_add(0x4c));
            let count = if lengths != 0 {
                u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
            } else {
                e.mem.u32(render_data.wrapping_add(0x40))
            };
            if e.vcall(g, 0x28, &args![]).u32() == 0 {
                let vertices = e.mem.u32(render_data.wrapping_add(0x18));
                let device = e.get(this, NiDX9Renderer::device);
                let base = e.mem.u32(render_data.wrapping_add(0x38));
                let kind = e.mem.u32(render_data.wrapping_add(0x3c));
                e.vcall(
                    device,
                    DEVICE_DRAW_INDEXED_PRIMITIVE,
                    &args![kind, base, 0u32, vertices, start, count],
                );
            } else {
                let draw = e.vcall(g, 0x28, &args![]).u32();
                let whole = e.mem.u8(draw.wrapping_add(0xcd)) != 0;
                let records = if whole {
                    1
                } else {
                    e.mem.u32(draw.wrapping_add(0xc8))
                };
                let mut offset = 0u32;
                let mut left = records;
                while left != 0 {
                    let array = e.mem.u32(draw.wrapping_add(0xc4));
                    let record = array.wrapping_add(offset);
                    if e.mem.u8(record.wrapping_add(0x10)) != 0 {
                        let whole = e.mem.u8(draw.wrapping_add(0xcd)) != 0;
                        let part_count = if whole {
                            let draw_data = e.mem.u32(draw.wrapping_add(0xb8));
                            u32::from(e.mem.u16(draw_data.wrapping_add(0x40)))
                        } else {
                            e.mem.u32(record.wrapping_add(0xc))
                        };
                        let part_start = if whole { 0 } else { e.mem.u32(record) };
                        let device = e.get(this, NiDX9Renderer::device);
                        let vertices = e.mem.u32(render_data.wrapping_add(0x18));
                        let base = e.mem.u32(render_data.wrapping_add(0x38));
                        let kind = e.mem.u32(render_data.wrapping_add(0x3c));
                        e.vcall(
                            device,
                            DEVICE_DRAW_INDEXED_PRIMITIVE,
                            &args![kind, base, 0u32, vertices, part_start, part_count],
                        );
                    }
                    offset = offset.wrapping_add(0x14);
                    left -= 1;
                }
            }
            start = start.wrapping_add(count).wrapping_add(2);
            strip += 1;
        }
    } else {
        let count = e.mem.u32(render_data.wrapping_add(0x40));
        let first = e.mem.u32(render_data.wrapping_add(0x38));
        let device = e.get(this, NiDX9Renderer::device);
        let kind = e.mem.u32(render_data.wrapping_add(0x3c));
        e.vcall(device, DEVICE_DRAW_PRIMITIVE, &args![kind, first, count]);
    }
    let flags = e.mem.u16(data.wrapping_add(0xe));
    e.mem.set_u16(data.wrapping_add(0xe), flags & 0xf000);
}

#[cfg(test)]
mod b048_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A vtable at `table` whose slot `n` calls the double at `base + 4 * n`.
    fn vtable(e: &mut Engine, table: u32, base: u32) {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
    }

    /// An object whose vtable slot at byte `slot` is the double `base + slot`.
    fn object(e: &mut Engine, table: u32, base: u32) -> u32 {
        vtable(e, table, base);
        let o = e.mem.alloc(0x40);
        e.mem.set_u32(o, table);
        o
    }

    fn renderer(e: &mut Engine) -> Ptr<NiDX9Renderer> {
        e.new_object::<NiDX9Renderer>()
    }

    #[test]
    fn pixel_formats_compare_their_fields_and_channels() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x44);
        let b = e.mem.alloc(0x44);
        let ne = |e: &mut Engine| e.call(0x00a7_65c0, &args![a, b]).bool();
        assert!(!ne(&mut e));
        e.mem.set_u32(b + 0xc, 0xffff_ffff);
        e.mem.set_u32(a + 0xc, 5);
        assert!(!ne(&mut e));
        e.mem.set_u32(b + 0xc, 6);
        assert!(ne(&mut e));
        e.mem.set_u32(b + 0xc, 5);
        e.mem.set_u8(b + 0x1d + 0xc * 3, 1);
        assert!(ne(&mut e));
        e.mem.set_u8(b + 0x1d + 0xc * 3, 0);
        e.mem.set_u8(b + 1, 2);
        assert!(ne(&mut e));
    }

    #[test]
    fn small_destructors_and_forwarders() {
        let mut e = Engine::new();
        e.register(PROPERTY_DESTRUCT, nop);
        e.register(SIZED_DELETE, nop);
        e.register(RENDERER_DESTRUCT, nop);
        e.register(D3D_FROM_NI_00B71A40, |_, _| 7u32.into_ret());
        e.register(MEMBER_CALL_0043A010, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_50f0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        e.call(0x00a9_50f0, &args![0x5000u32, 0u32]);
        assert_eq!(e.call(0x00e7_62f0, &args![0x6000u32, 1u32]).u32(), 0x6000);
        assert_eq!(
            calls(&e, SIZED_DELETE),
            vec![vec![0x5000, 0x1c], vec![0x6000, 0xb80]]
        );
        assert_eq!(calls(&e, PROPERTY_DESTRUCT).len(), 2);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x30, 1.5f32.to_bits());
        assert_eq!(e.call(0x00b7_99a0, &args![0x77u32, source]).u32(), 7);
        assert_eq!(
            calls(&e, D3D_FROM_NI_00B71A40),
            vec![vec![0x77, source, source + 0x24, 1.5f32.to_bits()]]
        );
        assert_eq!(e.call(0x00e7_2e50, &args![0x100u32]).u32(), 0x10c);
    }

    fn d3d9_rig(e: &mut Engine) {
        e.map(D3D9_MODULE, 0x30);
        e.map(DIRECT3D_IS_EX, 4);
        e.register(LOAD_LIBRARY_A, |_, _| 0x7700u32.into_ret());
        e.register(GET_PROC_ADDRESS, |_, a| {
            if a[1] == DIRECT3D_CREATE_9_TEXT {
                0x6200_0000u32
            } else {
                0x6200_0004
            }
            .into_ret()
        });
        e.register(REPORT_00A29680, nop);
    }

    #[test]
    fn direct3d_entry_points_load_once() {
        let mut e = Engine::new();
        d3d9_rig(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_9410, &args![]).bool());
        assert_eq!(e.global::<u32>(DIRECT3D_CREATE9), 0x6200_0000);
        assert_eq!(e.global::<u32>(DIRECT3D_CREATE9_EX), 0x6200_0004);
        assert!(e.call(0x00e6_9410, &args![]).bool());
        assert_eq!(calls(&e, LOAD_LIBRARY_A), vec![vec![D3D9_DLL_TEXT]]);
        e.set_global(DIRECT3D_CREATE9, 0u32);
        e.register(LOAD_LIBRARY_A, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e6_9410, &args![]).bool());
    }

    #[test]
    fn create_direct3d9_tries_the_ex_entry_first() {
        let mut e = Engine::new();
        d3d9_rig(&mut e);
        // Ex fails (leaves the out word 0), the plain creation works: the answer stays -1.
        e.register(0x6200_0004, nop);
        e.register(0x6200_0000, |_, _| 0xd3d0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_9460, &args![true]).i32(), -1);
        assert_eq!(e.global::<u32>(DIRECT3D), 0xd3d0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![
                vec![CREATE_EX_FAILED_010ED598],
                vec![CREATE_SUCCEEDED_010ED52C]
            ]
        );
        // Made already: 0 at once.
        assert_eq!(e.call(0x00e6_9460, &args![true]).i32(), 0);
        // Ex works.
        e.set_global(DIRECT3D, 0u32);
        e.register(0x6200_0004, |e, a| {
            e.mem.set_u32(a[1], 0xe0e0);
            Ret::default()
        });
        assert_eq!(e.call(0x00e6_9460, &args![true]).i32(), 0);
        assert_eq!(e.global::<u32>(DIRECT3D), 0xe0e0);
        assert_eq!(e.global::<u8>(DIRECT3D_IS_EX), 1);
        // Neither: -1 with the failure message.
        e.set_global(DIRECT3D, 0u32);
        e.register(0x6200_0000, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e6_9460, &args![false]).i32(), -1);
        assert_eq!(
            calls(&e, REPORT_00A29680).last().unwrap(),
            &vec![CREATE_FAILED_010ED550]
        );
    }

    #[test]
    fn mode_table_sets_the_count_and_flags() {
        let mut e = Engine::new();
        let count = e.mem.alloc(4);
        let flags = e.mem.alloc(4);
        let cases = [
            (0u32, 1u32, 0x50u32),
            (1, 1, 0x40),
            (2, 1, 0x80),
            (3, 1, 0x20),
            (4, 2, 0x20),
            (5, 2, 0x40),
            (6, 2, 0x80),
        ];
        for (mode, value, set) in cases {
            e.mem.set_u32(flags, 0xffff_ffff);
            assert!(e.call(0x00e6_9510, &args![mode, count, flags]).bool());
            assert_eq!(e.mem.u32(count), value);
            assert_eq!(e.mem.u32(flags), 0xffff_ff0f | set);
        }
        e.mem.set_u32(flags, 0xff);
        assert!(!e.call(0x00e6_9510, &args![7u32, count, flags]).bool());
        assert_eq!(e.mem.u32(flags), 0x0f);
    }

    #[test]
    fn getters_and_object_forwarders() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        e.set(r, NiDX9Renderer::word_8a0, 1);
        e.set(r, NiDX9Renderer::byte_8a4, 2);
        e.set(r, NiDX9Renderer::word_5ec, 3);
        e.set(r, NiDX9Renderer::target_group, 4);
        let o884 = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::object_884, o884);
        let o8c4 = object(&mut e, 0x6400, 0x6140_0000);
        e.set(r, NiDX9Renderer::object_8c4, o8c4);
        let o8b8 = object(&mut e, 0x6800, 0x6180_0000);
        e.set(r, NiDX9Renderer::object_8b8, o8b8);
        e.register(0x6100_00b0, |_, a| (a[1] + 9).into_ret());
        e.register(0x6140_0010, |_, a| (a[1] + a[2] + a[3]).into_ret());
        e.register(0x6180_0054, |_, a| a[1].into_ret());
        assert_eq!(e.call(0x00e6_95e0, &args![r]).u32(), 1);
        assert_eq!(e.call(0x00e6_95f0, &args![r]).u32(), 2);
        assert_eq!(e.call(0x00e7_57d0, &args![r]).u32(), 3);
        assert_eq!(e.call(0x00e7_5810, &args![r]).u32(), 4);
        assert_eq!(e.call(0x00e6_9600, &args![r]).u32(), o884);
        assert_eq!(e.call(0x00e6_9620, &args![r]).u32(), 9);
        assert_eq!(e.call(0x00e6_9640, &args![r, 1u32, 2u32, 3u32]).u32(), 6);
        assert_eq!(e.call(0x00e6_9960, &args![r, 5u32]).u32(), 5);
    }

    #[test]
    fn end_frame_ends_the_scene_unless_skipped() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let device = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::device, device);
        e.set(r, NiDX9Renderer::object_8bc, 0x8bc0);
        e.register(0x6100_0000 + DEVICE_END_SCENE, |_, _| 0u32.into_ret());
        e.register(OBJECT_8BC_CALL_00E8BF80, nop);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_9660, &args![r]).bool());
        assert_eq!(calls(&e, OBJECT_8BC_CALL_00E8BF80), vec![vec![0x8bc0]]);
        e.register(0x6100_0000 + DEVICE_END_SCENE, |_, _| {
            0x8000_4005u32.into_ret()
        });
        assert!(!e.call(0x00e6_9660, &args![r]).bool());
        e.mem.set_u8(r.addr() + SKIP_END_SCENE, 1);
        assert!(e.call(0x00e6_9660, &args![r]).bool());
        assert_eq!(calls(&e, 0x6100_0000 + DEVICE_END_SCENE).len(), 2);
        assert_eq!(calls(&e, OBJECT_8BC_CALL_00E8BF80).len(), 1);
    }

    #[test]
    fn vertex_declarations_are_made_once_from_the_stack_elements() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let device = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::device, device);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        let base = r.addr();
        e.register_double(
            0x6100_0000 + DEVICE_CREATE_VERTEX_DECLARATION,
            move |e, a| {
                let len = if a[2] == base + 0x284 { 0x30 } else { 0x28 };
                log.borrow_mut().push(e.mem.bytes(a[1], len));
                e.mem.set_u32(a[2], 0xdec1);
                Ret::default()
            },
        );
        assert_eq!(e.call(0x00e6_96a0, &args![r]).u32(), 0xdec1);
        assert_eq!(e.call(0x00e6_96a0, &args![r]).u32(), 0xdec1);
        assert_eq!(e.call(0x00e6_9790, &args![r]).u32(), 0xdec1);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(
            &seen[0][..0x28],
            &[
                0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 12, 0, 2, 0, 3, 0, 0, 0, 24, 0, 4, 0, 10, 0, 0, 0,
                28, 0, 1, 0, 5, 0, 0xff, 0, 0, 0, 0x11, 0, 0, 0
            ]
        );
        assert_eq!(
            &seen[1][0x20..0x30],
            &[0, 0, 36, 0, 0, 0, 5, 1, 0xff, 0, 0, 0, 0x11, 0, 0, 0]
        );
        // A failed creation clears the slot.
        e.mem.set_u32(r.addr() + 0x280, 0);
        e.register(0x6100_0000 + DEVICE_CREATE_VERTEX_DECLARATION, |e, a| {
            e.mem.set_u32(a[2], 0x55);
            0x8000_0000u32.into_ret()
        });
        assert_eq!(e.call(0x00e6_96a0, &args![r]).u32(), 0);
        assert_eq!(e.mem.u32(r.addr() + 0x280), 0);
    }

    #[test]
    fn purges_release_the_shaders_direct3d_objects() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let cache = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::object_8b8, cache);
        let shader = object(&mut e, 0x6400, 0x6140_0000);
        let d3d = object(&mut e, 0x6800, 0x6180_0000);
        for a in [
            0x6100_008c,
            0x6100_007c,
            0x6180_0008,
            0x6140_0088,
            0x6140_0080,
        ] {
            e.register(a, |_, _| 0x33u32.into_ret());
        }
        e.register_double(0x6140_0084, move |_, _| d3d.into_ret());
        e.register(0x6140_007c, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_9970, &args![r, shader]).u32(), 0x33);
        assert_eq!(calls(&e, 0x6100_008c), vec![vec![cache, 0, 0]]);
        assert_eq!(calls(&e, 0x6180_0008), vec![vec![d3d]]);
        assert_eq!(calls(&e, 0x6140_0088), vec![vec![shader, 0]]);
        e.call(0x00e6_99c0, &args![r, shader]);
        assert_eq!(calls(&e, 0x6100_007c), vec![vec![cache, 0, 0]]);
        assert_eq!(calls(&e, 0x6180_0008).len(), 1);
        assert_eq!(calls(&e, 0x6140_0080), vec![vec![shader, 0]]);
    }

    #[test]
    fn fn_00e69a20_passes_the_address_of_its_first_word() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        e.set(r, NiDX9Renderer::device, 0xd3d);
        e.register(DEVICE_CALL_00E7D9D0, |e, a| e.mem.u32(a[1]).into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e6_9a20, &args![r, 0u32, 2u32, 3u32]).bool());
        assert!(e.call(0x00e6_9a20, &args![r, 9u32, 2u32, 3u32]).bool());
        let c = calls(&e, DEVICE_CALL_00E7D9D0);
        assert_eq!(c.len(), 1);
        assert_eq!((c[0][0], c[0][2], c[0][3]), (0xd3d, 2, 3));
    }

    #[test]
    fn adapter_info_is_made_once_with_a_temporary_direct3d() {
        let mut e = Engine::new();
        d3d9_rig(&mut e);
        e.map(ADAPTER_LIST, 4);
        let d3d = object(&mut e, 0x6000, 0x6100_0000);
        e.register(0x6100_0008, nop);
        e.register_double(0x6200_0000, move |_, _| d3d.into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ADAPTER_INFO_FILL, nop);
        e.call_log = Some(vec![]);
        let info = e.call(0x00e6_9a60, &args![]).u32();
        assert_ne!(info, 0);
        assert_eq!(e.mem.u32(info + 4), ADAPTER_INFO_VTABLE);
        assert_eq!(e.mem.u16(info + 0x12), 1);
        assert_eq!(
            calls(&e, ADAPTER_INFO_FILL),
            vec![vec![info, d3d, ADAPTER_INFO_ARGUMENT]]
        );
        assert_eq!(calls(&e, 0x6100_0008), vec![vec![d3d]]);
        assert_eq!(e.call(0x00e6_9a60, &args![]).u32(), info);
        assert_eq!(calls(&e, NEW_OBJECT).len(), 1);
    }

    #[test]
    fn rtti_chain_and_indexed_element() {
        let mut e = Engine::new();
        let o = object(&mut e, 0x6000, 0x6100_0000);
        let link2 = e.mem.alloc(8);
        let link1 = e.mem.alloc(8);
        e.mem.set_u32(link1 + 4, link2);
        e.register_double(0x6100_000c, move |_, _| link1.into_ret());
        assert_eq!(e.call(0x00e6_9b00, &args![o, link2]).u32(), o);
        assert_eq!(e.call(0x00e6_9b00, &args![o, 0x1234u32]).u32(), 0);
        let a = e.mem.alloc(0x30);
        let elements = e.mem.alloc(8);
        e.mem.set_u32(elements + 4, 0x99);
        e.mem.set_u32(a + 0x28, elements);
        e.mem.set_u32(a + 0x20, 2);
        assert_eq!(e.call(0x00e6_9b40, &args![a, 1u32]).u32(), 0x99);
        assert_eq!(e.call(0x00e6_9b40, &args![a, 2u32]).u32(), 0);
    }

    #[test]
    fn format_tables() {
        let mut e = Engine::new();
        let c90: Vec<u32> = (0..13)
            .map(|i| e.call(0x00e6_9c90, &args![i as u32]).u32())
            .collect();
        assert_eq!(
            c90,
            vec![0, 0x14, 0x16, 0x16, 0x17, 0x18, 0x18, 0x1e, 0x1b, 0, 0, 0x1e, 0]
        );
        let d00 = |e: &mut Engine, f: u32| e.call(0x00e6_9d00, &args![f]).u32();
        assert_eq!(d00(&mut e, 20), 1);
        assert_eq!(d00(&mut e, 30), 11);
        assert_eq!(d00(&mut e, 31), 0);
        assert_eq!(d00(&mut e, 36), 14);
        assert_eq!(d00(&mut e, 111), 12);
        assert_eq!(d00(&mut e, 112), 13);
        assert_eq!(d00(&mut e, 113), 15);
        assert_eq!(d00(&mut e, 116), 18);
        assert_eq!(d00(&mut e, 117), 0);
        let e50: Vec<u32> = (0..6)
            .map(|i| e.call(0x00e6_9e50, &args![i as u32]).u32())
            .collect();
        assert_eq!(e50, vec![0x8000_0000, 1, 2, 4, 8, 0]);
        let s = |e: &mut Engine, f: u32| e.call(0x00e7_a9a0, &args![f]).u32();
        assert_eq!(s(&mut e, 21), 0x010e_ec28);
        assert_eq!(s(&mut e, 81), 0x010e_e9f4);
        assert_eq!(s(&mut e, 0x3154_454d), 0x010e_e920);
        assert_eq!(s(&mut e, 0x3154_5844), 0x010e_e908);
        assert_eq!(s(&mut e, 0x5956_5955), 0x010e_e8a4);
        assert_eq!(s(&mut e, 37), UNKNOWN_FORMAT_01015890);
        assert_eq!(s(&mut e, 0xffff_ffff), UNKNOWN_FORMAT_01015890);
    }

    #[test]
    fn texture_renderer_data() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let texture = e.mem.alloc(0x30);
        for a in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            MAP_SET_AT,
            FREE,
        ] {
            e.register(a, nop);
        }
        e.register(MAP_REMOVE_AT, |_, _| 1u32.into_ret());
        e.register(MAKE_TEXTURE_DATA_00E68EF0, |_, _| 0xda7au32.into_ret());
        e.register(MAKE_TEXTURE_DATA_00E8ABF0, |_, _| 0u32.into_ret());
        e.register(MAKE_TEXTURE_DATA_00E8FF60, |_, a| a[2].into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_dc40, &args![r, texture]).bool());
        assert!(!e.call(0x00e6_dcc0, &args![r, texture]).bool());
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 2);
        assert!(!e.call(0x00e6_dc80, &args![r, texture, 0u32]).bool());
        assert!(e.call(0x00e6_dc80, &args![r, texture, 5u32]).bool());
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![r.addr() + 0x8cc, texture, 5]]
        );
        e.mem.set_u32(texture + 0x24, 0x1);
        assert!(e.call(0x00e6_dcc0, &args![r, texture]).bool());
        // That path leaves the lock taken.
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 2);
        assert!(e.call(0x00e6_dc80, &args![r, texture, 5u32]).bool());
        assert_eq!(e.call(0x00e6_df80, &args![r, texture]).u32(), 1);
        assert_eq!(e.mem.u32(texture + 0x24), 0);
        assert_eq!(
            calls(&e, MAP_REMOVE_AT),
            vec![vec![r.addr() + 0x8cc, texture]]
        );
        e.mem.set_u32(texture + 0x28, 0xb10c);
        e.mem.set_u32(texture + 0x24, 0xb10c);
        e.call(0x00e6_d740, &args![r, texture]);
        assert_eq!(calls(&e, FREE), vec![vec![0xb10c]]);
        assert_eq!(e.mem.u32(texture + 0x28), 0);
        assert_eq!(e.mem.u32(texture + 0x24), 0);
    }

    fn ref_counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_UP, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DOWN, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn pointer_member_assignment() {
        let mut e = Engine::new();
        ref_counting(&mut e);
        let this = e.mem.alloc(0x20);
        let first = object(&mut e, 0x6000, 0x6100_0000);
        let second = object(&mut e, 0x6000, 0x6100_0000);
        e.register(0x6100_0004, nop);
        e.call_log = Some(vec![]);
        e.call(0x00e6_e6e0, &args![this, first]);
        e.call(0x00e6_e6e0, &args![this, first]);
        assert_eq!(e.mem.u32(first + 4), 1);
        e.call(0x00e6_e6e0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x14), first);
        e.call(0x00e6_e6e0, &args![this, second]);
        assert_eq!(e.mem.u32(this + 0x14), second);
        assert_eq!(calls(&e, 0x6100_0004), vec![vec![first]]);
    }

    #[test]
    fn shaders_join_and_leave_the_list() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let shader = object(&mut e, 0x6000, 0x6100_0000);
        e.register(SHADER_SET_D3D_RENDERER, nop);
        e.register(0x6100_00cc, nop);
        e.register(SHADER_LIST_ADD, |e, a| e.mem.u32(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e7_5310, &args![r, shader]);
        e.call(0x00e7_5340, &args![r, shader]);
        assert_eq!(
            calls(&e, SHADER_SET_D3D_RENDERER),
            vec![vec![shader, 0], vec![shader, 0]]
        );
        assert_eq!(calls(&e, 0x6100_00cc), vec![vec![shader]]);
        let adds = calls(&e, SHADER_LIST_ADD);
        assert_eq!(adds.len(), 2);
        assert!(adds.iter().all(|a| a[0] == r.addr() + 0x90c));
    }

    #[test]
    fn purge_all_shaders_empties_the_list_into_the_pool() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        e.map(NODE_POOL_FREE, 0x30);
        e.register(SHADER_SET_D3D_RENDERER, nop);
        e.register(NODE_POOL_LOCK_TAKE, |e, _| {
            let c = e.global::<u32>(NODE_POOL_LOCK_COUNT) + 1;
            e.set_global(NODE_POOL_LOCK_COUNT, c);
            Ret::default()
        });
        let first = object(&mut e, 0x6000, 0x6100_0000);
        let second = object(&mut e, 0x6000, 0x6100_0000);
        e.register(0x6100_00cc, nop);
        let n2 = e.mem.alloc(12);
        let n1 = e.mem.alloc(12);
        e.mem.set_u32(n1, n2);
        e.mem.set_u32(n1 + 8, first);
        e.mem.set_u32(n2 + 4, n1);
        e.mem.set_u32(n2 + 8, second);
        let list = r.addr() + 0x90c;
        e.mem.set_u32(list, n1);
        e.mem.set_u32(list + 4, n2);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00e7_5380, &args![r]);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE), n2);
        assert_eq!(e.mem.u32(n2), n1);
        assert_eq!(e.global::<u32>(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(
            calls(&e, SHADER_SET_D3D_RENDERER),
            vec![vec![first, 0], vec![second, 0]]
        );
    }

    #[test]
    fn background_colours_pack_and_unpack() {
        let mut e = Engine::new();
        e.map(SCALE_TO_BYTE, 8);
        e.map(SCALE_FROM_BYTE, 8);
        e.mem.set_u64(SCALE_TO_BYTE, 255.0f64.to_bits());
        e.mem
            .set_u64(SCALE_FROM_BYTE, f64::from(1.0f32 / 255.0).to_bits());
        let r = renderer(&mut e);
        let color = e.mem.alloc(0x10);
        for (i, v) in [1.0f32, 0.5, 0.25, 0.75].iter().enumerate() {
            e.mem.set_u32(color + 4 * i as u32, v.to_bits());
        }
        e.call(0x00e7_5840, &args![r, color]);
        assert_eq!(e.get(r, NiDX9Renderer::background_color), 0xffff_7f3f);
        e.call(0x00e7_58f0, &args![r, color]);
        assert_eq!(e.get(r, NiDX9Renderer::background_color), 0xbfff_7f3f);
        let out = e.mem.alloc(0x10);
        e.call(0x00e7_59c0, &args![r, out]);
        let k = f64::from(1.0f32 / 255.0);
        assert_eq!(e.mem.u32(out + 0xc), ((191.0 * k) as f32).to_bits());
        assert_eq!(e.mem.u32(out), ((255.0 * k) as f32).to_bits());
        assert_eq!(e.mem.u32(out + 4), ((127.0 * k) as f32).to_bits());
        assert_eq!(e.mem.u32(out + 8), ((63.0 * k) as f32).to_bits());
        // A channel out of range keeps the low byte of the integer indefinite (0).
        e.mem.set_u32(color, f32::NAN.to_bits());
        e.call(0x00e7_5840, &args![r, color]);
        assert_eq!(e.get(r, NiDX9Renderer::background_color), 0xff00_7f3f);
    }

    #[test]
    fn object_lookup_forwards_to_the_object_at_0x8c8() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        e.set(r, NiDX9Renderer::object_8c8, 0x8c80);
        e.register(OBJECT_LOOKUP_00653290, |_, a| {
            if a[0] == 1 { 0 } else { 0x77 }.into_ret()
        });
        e.register(OBJECT_8C8_CALL_00E8BD40, nop);
        e.call_log = Some(vec![]);
        e.call(0x00e7_1cc0, &args![r, 0u32]);
        e.call(0x00e7_1cc0, &args![r, 1u32]);
        e.call(0x00e7_1cc0, &args![r, 2u32]);
        assert_eq!(
            calls(&e, OBJECT_LOOKUP_00653290),
            vec![
                vec![1, OBJECT_LOOKUP_ARGUMENT],
                vec![2, OBJECT_LOOKUP_ARGUMENT]
            ]
        );
        assert_eq!(
            calls(&e, OBJECT_8C8_CALL_00E8BD40),
            vec![vec![0x8c80, 0x77]]
        );
    }

    #[test]
    fn keyed_values_are_made_once() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let source = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::object_884, source);
        let shader = object(&mut e, 0x6400, 0x6140_0000);
        e.mem.set_u32(shader + 0x28, 0x28);
        let rtti = e.mem.alloc(8);
        e.register_double(0x6100_00c8, move |_, _| shader.into_ret());
        e.register_double(0x6140_000c, move |_, _| rtti.into_ret());
        e.mem.set_u32(rtti + 4, SHADER_RTTI_0126F5D8);
        e.mem.set_u32(r.addr() + 0x5dc, 0x5dc0);
        e.register(KEY_PART_00E7BD20, |_, a| (a[0] + 1).into_ret());
        e.register(DEPTH_FORMAT_00E7B0F0, |_, _| 0x4e40u32.into_ret());
        let stored = Rc::new(RefCell::new(0u32));
        let get = stored.clone();
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], *get.borrow());
            u32::from(*get.borrow() != 0).into_ret()
        });
        let set = stored.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            *set.borrow_mut() = a[2];
            Ret::default()
        });
        e.register(MAKE_VALUE_00E7C500, |_, _| 0x7a1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e6_ee00, &args![r, 1u32, 2u32, 3u32]).u32(),
            0x7a1
        );
        assert_eq!(
            e.call(0x00e6_ee00, &args![r, 1u32, 2u32, 3u32]).u32(),
            0x7a1
        );
        assert_eq!(calls(&e, MAKE_VALUE_00E7C500), vec![vec![0x4e40]]);
        assert_eq!(
            calls(&e, DEPTH_FORMAT_00E7B0F0)[0],
            vec![0x5dc0, 0x28, 2, 2, 3]
        );
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![r.addr() + 0xb14, 0x4e40, 0x7a1]]
        );
    }

    #[test]
    fn geometry_check_branches() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        vtable(&mut e, 0x6000, 0x6100_0000);
        e.mem.set_u32(r.addr(), 0x6000);
        e.register(0x6100_00a0, |_, _| 0u32.into_ret());
        let geometry = e.mem.alloc(0xd0);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0xb8, data);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        let third = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0xbc, first);
        e.mem.set_u32(first + 0xc, second);
        e.mem.set_u32(second + 0xc, third);
        let check = |e: &mut Engine, o: u32| e.call(0x00e7_2c20, &args![r, geometry, o]).bool();
        assert!(!check(&mut e, 0));
        e.mem.set_u32(third + 0x28, 1);
        assert!(check(&mut e, 0));
        e.mem.set_u32(third + 0x28, 0);
        e.register(0x6100_00a0, |_, _| 2u32.into_ret());
        assert!(check(&mut e, 0));
        e.register(0x6100_00a0, |_, _| 0u32.into_ret());
        let target = object(&mut e, 0x6400, 0x6140_0000);
        e.register(0x6140_00d4, |_, _| 1u32.into_ret());
        assert!(!check(&mut e, target));
        e.mem.set_u32(r.addr() + 0x6f0, 0xfffe_0001);
        assert!(check(&mut e, target));
        e.mem.set_u32(r.addr() + 0x6f0, 0);
        e.mem.set_u8(r.addr() + 0x5d5, 1);
        let inner = object(&mut e, 0x6800, 0x6180_0000);
        e.register_double(0x6140_00bc, move |_, _| inner.into_ret());
        e.register(0x6180_00a8, |_, _| 1u32.into_ret());
        // The geometry's own object at +0xC0 stands in for a 0 argument.
        e.mem.set_u32(geometry + 0xc0, target);
        assert!(check(&mut e, 0));
        e.mem.set_u32(data + 0x34, 1);
        assert!(!check(&mut e, target));
    }

    #[test]
    fn default_textures_pick_formats_and_fill_their_pixels() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let t = r.addr();
        ref_counting(&mut e);
        // Group 0: index 0 and 2 set (index 0 wins over 2; 1 is empty). Group 2: index 5.
        let (f15, f16, f17) = (e.mem.alloc(0x20), e.mem.alloc(0x20), e.mem.alloc(0x20));
        e.mem.set_u32(f15 + 0x10, 0x115);
        e.mem.set_u32(t + 0x700, f15);
        e.mem.set_u32(t + 0x708, f16);
        e.mem.set_u32(t + 0x700 + 4 * (5 + 0x2c), f17);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        vtable(&mut e, 0x6000, 0x6100_0000);
        e.register(0x6100_0004, nop);
        e.register(PIXEL_DATA_CONSTRUCT_00A7C190, |e, a| {
            e.mem.set_u32(a[0], 0x6000);
            let pixels = e.mem.alloc(0x40);
            let buffers = e.mem.alloc(4);
            e.mem.set_u32(buffers, pixels);
            e.mem.set_u32(a[0] + 0x5c, buffers);
            e.mem.set_u32(a[0] + 0x50, 8);
            e.mem.set_u32(a[0] + 0x64, 2);
            a[0].into_ret()
        });
        e.register(MEMSET_ADDR, nop);
        e.register(PIXEL_FORMAT_SET_UP_00A622D0, nop);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_c550, &args![r]).bool());
        assert_eq!(e.mem.u32(t + 0x860), f15);
        assert_eq!(e.mem.u32(t + 0x864), 0);
        assert_eq!(e.mem.u32(t + 0x868), f17);
        assert_eq!(e.mem.u32(t + 0x880), 0x115);
        let constructs = calls(&e, PIXEL_DATA_CONSTRUCT_00A7C190);
        assert_eq!(constructs.len(), 2);
        assert_eq!(constructs[0][1..], [4, 4, f15, 1, 1]);
        let memsets = calls(&e, MEMSET_ADDR);
        assert_eq!((memsets[0][1], memsets[0][2]), (0xff, 0x20));
        assert_eq!(calls(&e, PIXEL_FORMAT_SET_UP_00A622D0), vec![vec![f15]]);
        // Nothing in the first group: false.
        e.mem.set_u32(t + 0x700, 0);
        e.mem.set_u32(t + 0x708, 0);
        assert!(!e.call(0x00e6_c550, &args![r]).bool());
    }

    #[test]
    fn texture_creation_by_flags_and_class() {
        let mut e = Engine::new();
        for a in [
            TEXTURE_SET_00A670C0,
            OBJECT_CALL_00A84CA0,
            TEXTURE_CALL_00A7B6C0,
            TEXTURE_CALL_00A760C0,
        ] {
            e.register(a, nop);
        }
        e.register(IS_KIND_OF, |_, a| {
            u32::from(a[0] == TEXTURE_RTTI_011F4A70).into_ret()
        });
        let texture = e.mem.alloc(0x50);
        e.mem.set_u16(texture + 0xe, 0x4000);
        e.mem.set_u16(texture + 8, 0x20);
        e.mem.set_u16(texture + 0xc, 0x3001);
        e.mem.set_u32(texture + 0x20, 0xa);
        e.mem.set_u32(texture + 0x2c, 0xd);
        e.mem.set_u16(texture + 0x44, 0x44);
        e.mem.set_u32(texture + 0x48, 0x48);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, 0x0b8);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 0u32]);
        assert_eq!(
            calls(&e, TEXTURE_SET_00A670C0),
            vec![vec![texture, 0x20, 0, 0, 0, 0, 0, 0x3000]]
        );
        assert_eq!(
            calls(&e, TEXTURE_CALL_00A760C0),
            vec![vec![texture, 0x44, 0x48, 0]]
        );
        e.mem.set_u8(texture + 0x38, 0x9);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 1u32]);
        assert_eq!(
            calls(&e, TEXTURE_SET_00A670C0)[1],
            vec![texture, 0x20, 0xa, 0, 0, 0xd, 1, 0x3000]
        );
        assert_eq!(calls(&e, OBJECT_CALL_00A84CA0), vec![vec![0x0b8]]);
        e.mem.set_u8(texture + 0x38, 0x30);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 1u32]);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 0u32]);
        assert_eq!(calls(&e, OBJECT_CALL_00A84CA0).len(), 1);
        assert_eq!(calls(&e, TEXTURE_CALL_00A760C0).len(), 1);
        // Not a 0x4000 texture: nothing.
        e.mem.set_u16(texture + 0xe, 0x3000);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 0u32]);
        assert_eq!(calls(&e, TEXTURE_SET_00A670C0).len(), 4);
    }

    #[test]
    fn geometry_purge_unlinks_its_records() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let t = r.addr();
        let tracker = object(&mut e, 0x6000, 0x6100_0000);
        e.mem.set_u32(t + 0x8ac, tracker);
        for a in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            MAP_SET_AT,
            MAP_REMOVE_AT,
            SIZED_DELETE,
            0x6100_001c,
        ] {
            e.register(a, nop);
        }
        let geometry = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0x34, data);
        let element = e.mem.alloc(8);
        e.mem.set_u32(element + 4, 0x4e1);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, element);
        e.mem.set_u32(data + 0x20, 1);
        e.mem.set_u32(data + 0x28, array);
        // Records: ours, someone else's, ours.
        let r3 = e.mem.alloc(0x24);
        let r2 = e.mem.alloc(0x24);
        let r1 = e.mem.alloc(0x24);
        e.mem.set_u32(r1, geometry);
        e.mem.set_u32(r1 + 0x20, r2);
        e.mem.set_u32(r2, other);
        e.mem.set_u32(r2 + 0x20, r3);
        e.mem.set_u32(r3, geometry);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], r1);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e7_1cf0, &args![r, geometry]);
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![t + 0x610, 0x4e1, r2]]);
        assert_eq!(e.mem.u32(r2 + 0x20), 0);
        assert_eq!(
            calls(&e, SIZED_DELETE),
            vec![vec![r1, 0x24], vec![r3, 0x24]]
        );
        assert_eq!(calls(&e, 0x6100_001c), vec![vec![tracker, geometry, 0]]);
        assert_eq!(
            calls(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![t + 0x100], vec![t + 0x80]]
        );
        // A lone record of ours: the key goes.
        e.mem.set_u32(r1 + 0x20, 0);
        e.call(0x00e7_1cf0, &args![r, geometry]);
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![t + 0x610, 0x4e1]]);
        // No data: only the tracker call.
        e.mem.set_u32(data + 0x20, 0);
        e.call(0x00e7_1cf0, &args![r, geometry]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 4);
        assert_eq!(calls(&e, 0x6100_001c).len(), 3);
    }
}

#[cfg(test)]
mod b048_buffer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    /// A renderer, a geometry data with `streams` stream slots, and doubles for the data's
    /// and the group's calls (the group's stream set-up fails at index `fail_at`).
    fn rig(e: &mut Engine, streams: u32, fail_at: u32) -> (Ptr<NiDX9Renderer>, u32) {
        let r = e.new_object::<NiDX9Renderer>();
        e.set(r, NiDX9Renderer::object_8bc, 0x8bc0);
        let data = e.mem.alloc(0x60);
        let slots = e.mem.alloc(4 * streams.max(1));
        e.mem.set_u32(data + 0x20, streams);
        e.mem.set_u32(data + 0x24, slots);
        let owner = object(e, 0x6800, 0x6180_0000, 0x10);
        e.mem.set_u32(data + 8, owner);
        e.register(0x6180_001c, nop);
        for a in [
            DATA_SET_COUNT_00E8EF10,
            DATA_SET_FIRST_00E8F010,
            DATA_SET_SECOND_00E8F040,
        ] {
            e.register(a, nop);
        }
        e.register(GROUP_PREPARE_00E8C120, |e, a| {
            e.mem.set_u32(a[3], 0xf1);
            e.mem.set_u32(a[4], 0x5ec);
            Ret::default()
        });
        e.register_double(GROUP_STREAM_00E8BFA0, move |_, a| {
            u32::from(a[2] != fail_at).into_ret()
        });
        e.map(STREAM_LIMIT_011BF558, 4);
        e.set_global(STREAM_LIMIT_011BF558, 1u32);
        (r, data)
    }

    #[test]
    fn stream_preparation_through_the_group() {
        let mut e = Engine::new();
        let (r, data) = rig(&mut e, 2, 99);
        let object = e.mem.alloc(0x30);
        e.mem.set_u16(object + 0xc, 0x1001);
        e.mem.set_u32(object + 0x24, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e6_da60, &args![r, data, object, 0u32, 0u32])
            .bool());
        let prepare = calls(&e, GROUP_PREPARE_00E8C120);
        assert_eq!(prepare.len(), 1);
        assert_eq!(&prepare[0][..3], &[0x8bc0, object, 0]);
        assert_eq!(calls(&e, DATA_SET_COUNT_00E8EF10), vec![vec![data, 1]]);
        assert_eq!(calls(&e, DATA_SET_FIRST_00E8F010), vec![vec![data, 0xf1]]);
        assert_eq!(e.mem.u32(e.mem.u32(data + 0x24)), 0x5ec);
        assert_eq!(e.mem.u32(data + 4), 0x0100_0000 | 0x80_0000 | 0x20_0000);
        assert_eq!(
            calls(&e, GROUP_STREAM_00E8BFA0),
            vec![vec![0x8bc0, data, 0]]
        );
        // A partition with more than four bones is refused before the group is asked.
        let partition = e.mem.alloc(0x30);
        e.mem.set_u16(partition + 0x20, 5);
        assert!(!e
            .call(0x00e6_da60, &args![r, data, object, partition, 0u32])
            .bool());
        assert_eq!(calls(&e, GROUP_PREPARE_00E8C120).len(), 1);
    }

    #[test]
    fn stream_preparation_through_a_source_and_its_undo() {
        let mut e = Engine::new();
        let (r, data) = rig(&mut e, 3, 2);
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 0x28, 1);
        let source = object_with_streams(&mut e);
        e.mem.set_u8(r.addr() + 0x5d4, 1);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e6_da60, &args![r, data, object, 0u32, source])
            .bool());
        assert!(calls(&e, GROUP_PREPARE_00E8C120).is_empty());
        assert_eq!(calls(&e, DATA_SET_SECOND_00E8F040), vec![vec![data, 0x50b]]);
        let slots = e.mem.u32(data + 0x24);
        assert_eq!(
            (e.mem.u32(slots), e.mem.u32(slots + 4), e.mem.u32(slots + 8)),
            (0x100, 0x101, 0x102)
        );
        assert_eq!(e.mem.u32(data + 4), 0x40_0000);
        assert_eq!(e.mem.u8(data + 0x14), 1);
        // Stream 2 failed: streams 2 and 1 are undone.
        let owner = e.mem.u32(data + 8);
        assert_eq!(
            calls(&e, 0x6180_001c),
            vec![vec![owner, data, 2], vec![owner, data, 1]]
        );
    }

    /// A source whose virtual +0xB0 answers three streams and the object 0x50B, +0xA0 the
    /// stream 0x100 + index and +0xA8 true.
    fn object_with_streams(e: &mut Engine) -> u32 {
        let source = object(e, 0x6400, 0x6140_0000, 0x10);
        e.register(0x6140_00b0, |e, a| {
            e.mem.set_u32(a[1], 0x50b);
            e.mem.set_u32(a[2], 3);
            1u32.into_ret()
        });
        e.register(0x6140_00a0, |_, a| (0x100 + a[1]).into_ret());
        e.register(0x6140_00a8, |_, _| 1u32.into_ret());
        source
    }

    fn map_rig(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32)>>> {
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let map = Rc::new(RefCell::new(Vec::<(u32, u32)>::new()));
        let get = map.clone();
        e.register_double(MAP_GET_AT, move |e, a| {
            match get.borrow().iter().find(|(k, _)| *k == a[1]) {
                Some((_, v)) => {
                    e.mem.set_u32(a[2], *v);
                    1u32.into_ret()
                }
                None => 0u32.into_ret(),
            }
        });
        let set = map.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            let mut m = set.borrow_mut();
            m.retain(|(k, _)| *k != a[1]);
            m.push((a[1], a[2]));
            Ret::default()
        });
        map
    }

    /// Stream elements with keys `0x4e0 + index`.
    fn keyed_streams(e: &mut Engine, data: u32, count: u32) {
        let array = e.mem.alloc(4 * count);
        for i in 0..count {
            let element = e.mem.alloc(8);
            e.mem.set_u32(element + 4, 0x4e0 + i);
            e.mem.set_u32(array + 4 * i, element);
        }
        e.mem.set_u32(data + 0x28, array);
    }

    #[test]
    fn geometry_precache_records_each_stream() {
        let mut e = Engine::new();
        let (r, data) = rig(&mut e, 2, 99);
        keyed_streams(&mut e, data, 2);
        let map = map_rig(&mut e);
        let object = object(&mut e, 0x6000, 0x6100_0000, 0x40);
        e.mem.set_u32(object + 0x34, data);
        e.mem.set_u16(object + 8, 0x77);
        e.register(0x6100_0090, |_, _| 0x1_0042u32.into_ret());
        e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 0u32.into_ret());
        map.borrow_mut().push((0x4e1, 0xabc));
        e.call_log = Some(vec![]);
        let words = args![
            r,
            0u32,
            object,
            0x1_0003u32,
            0x2_0004u32,
            5u32,
            6u32,
            0x3_0007u32,
            0u32
        ];
        assert!(e.call(0x00e7_2980, &words).bool());
        assert_eq!(e.mem.u32(data + 0x18), 0x42);
        assert_eq!(e.mem.u32(data + 0x1c), 0x77);
        assert_eq!(
            (
                e.mem.u32(data + 0x40),
                e.mem.u32(data + 0x44),
                e.mem.u32(data + 0x48)
            ),
            (3, 4, 7)
        );
        assert_eq!((e.mem.u32(data + 0x50), e.mem.u32(data + 0x4c)), (5, 6));
        let m = map.borrow();
        let first = m.iter().find(|(k, _)| *k == 0x4e0).unwrap().1;
        let second = m.iter().find(|(k, _)| *k == 0x4e1).unwrap().1;
        assert_eq!(e.mem.u32(first), object);
        assert_eq!(e.mem.u32(first + 0x18), data);
        assert_eq!(e.mem.u32(first + 0x20), 0);
        assert_eq!(e.mem.u32(second + 0x1c), 1);
        assert_eq!(e.mem.u32(second + 0x20), 0xabc);
        drop(m);
        // Packed already: true at once.
        e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 1u32.into_ret());
        assert!(e.call(0x00e7_2980, &words).bool());
        assert_eq!(calls(&e, 0x6100_0090).len(), 1);
    }

    #[test]
    fn skin_precache_records_each_partition_stream() {
        let mut e = Engine::new();
        let (r, data) = rig(&mut e, 1, 99);
        keyed_streams(&mut e, data, 1);
        let map = map_rig(&mut e);
        e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 0u32.into_ret());
        let partitions = e.mem.alloc(0x2c * 2);
        e.mem.set_u32(partitions + 0x28, data);
        e.mem.set_u16(partitions + 0x1c, 9);
        e.mem.set_u16(partitions + 0x1e, 4);
        e.mem.set_u32(partitions + 0x14, 0x14);
        e.mem.set_u32(partitions + 0x18, 0x18);
        // The second partition's data is packed already.
        let packed = e.mem.alloc(0x30);
        e.mem.set_u32(partitions + 0x2c + 0x28, packed);
        e.register_double(DATA_IS_PACKED_00E8EEB0, move |_, a| {
            u32::from(a[0] == packed).into_ret()
        });
        let instance = e.mem.alloc(0x10);
        e.mem.set_u32(instance + 8, 2);
        e.mem.set_u32(instance + 0xc, partitions);
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(skin + 0xc, instance);
        let object = e.mem.alloc(0x30);
        assert!(e
            .call(
                0x00e7_2aa0,
                &args![r, 0u32, object, skin, 0u32, 0x55u32, 0x66u32]
            )
            .bool());
        assert_eq!((e.mem.u32(data + 0x18), e.mem.u32(data + 0x1c)), (9, 9));
        assert_eq!(e.mem.u32(data + 0x48), 1);
        assert_eq!((e.mem.u32(data + 0x40), e.mem.u32(data + 0x44)), (4, 4));
        assert_eq!(
            (e.mem.u32(data + 0x50), e.mem.u32(data + 0x4c)),
            (0x14, 0x18)
        );
        let record = map.borrow()[0].1;
        assert_eq!(e.mem.u32(record + 8), partitions);
        assert_eq!(e.mem.u32(record + 4), skin);
        assert_eq!(
            (e.mem.u32(record + 0x10), e.mem.u32(record + 0x14)),
            (0x55, 0x66)
        );
        // A failed preparation makes the answer false.
        e.mem.set_u16(partitions + 0x20, 5);
        assert!(!e
            .call(
                0x00e7_2aa0,
                &args![r, 0u32, object, skin, 0u32, 0x55u32, 0x66u32]
            )
            .bool());
    }

    fn ref_counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_UP, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DOWN, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn pointer_map_set_at_adds_and_replaces() {
        let mut e = Engine::new();
        ref_counting(&mut e);
        let map = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map + 8, buckets);
        e.register(0x6100_0004, |_, a| (a[1] & 1).into_ret());
        e.register(0x6100_0008, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(0x6100_0014, |e, _| e.mem.alloc(12).into_ret());
        e.register(0x6100_000c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        let first = object(&mut e, 0x6400, 0x6140_0000, 0x10);
        let second = object(&mut e, 0x6400, 0x6140_0000, 0x10);
        e.register(0x6140_0004, nop);
        // The caller hands each value over with one reference of its own (the by-value copy).
        e.mem.set_u32(first + 4, 1);
        e.call(0x00e7_2cf0, &args![map, 3u32, first]);
        let item = e.mem.u32(buckets + 4);
        assert_eq!(e.mem.u32(item + 8), first);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(e.mem.u32(first + 4), 1);
        e.mem.set_u32(second + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e7_2cf0, &args![map, 3u32, second]);
        assert_eq!(e.mem.u32(item + 8), second);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(calls(&e, 0x6140_0004), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
    }

    #[test]
    fn skin_purge_unlinks_each_partitions_records() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        e.mem.set_u32(t + 0x8ac, 0x8ac0);
        for a in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            MAP_SET_AT,
            MAP_REMOVE_AT,
            SIZED_DELETE,
            REMOVE_SKIN_PARTITION_FROM_GROUP,
        ] {
            e.register(a, nop);
        }
        let data = e.mem.alloc(0x30);
        keyed_streams(&mut e, data, 1);
        e.mem.set_u32(data + 0x20, 1);
        let partitions = e.mem.alloc(0x2c);
        e.mem.set_u32(partitions + 0x28, data);
        let instance = e.mem.alloc(0x10);
        e.mem.set_u32(instance + 8, 1);
        e.mem.set_u32(instance + 0xc, partitions);
        let record = e.mem.alloc(0x24);
        e.mem.set_u32(record + 8, partitions);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], record);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e7_1e40, &args![r, instance]);
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![t + 0x610, 0x4e0]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![record, 0x24]]);
        assert_eq!(
            calls(&e, REMOVE_SKIN_PARTITION_FROM_GROUP),
            vec![vec![0x8ac0, instance]]
        );
        // Without data on the first partition nothing happens at all.
        e.mem.set_u32(partitions + 0x28, 0);
        e.call(0x00e7_1e40, &args![r, instance]);
        assert_eq!(calls(&e, REMOVE_SKIN_PARTITION_FROM_GROUP).len(), 1);
    }
}

#[cfg(test)]
mod b048_setup_tests {
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

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    fn ref_counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_UP, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DOWN, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn clipper_texture_from_a_two_texel_image() {
        let mut e = Engine::new();
        ref_counting(&mut e);
        e.map(EMPTY_FIXED_STRING_0109B220, 4);
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        let format = e.mem.alloc(0x20);
        e.mem.set_u8(format + 1, 0x20);
        e.mem.set_u32(t + 0x700, format);
        let targets: Vec<u32> = (0..8).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(0x6000, &targets);
        e.register(0x6100_0004, nop);
        e.register(SAMPLER_SET_UP_00E6DFC0, nop);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PIXEL_DATA_CONSTRUCT_00A7C190, |e, a| {
            let texels = e.mem.alloc(0x10);
            e.mem.set_u32(texels, 0x1111_1111);
            e.mem.set_u32(texels + 4, 0x2222_2222);
            let buffers = e.mem.alloc(4);
            e.mem.set_u32(buffers, texels);
            e.mem.set_u32(a[0], 0x6000);
            e.mem.set_u32(a[0] + 0x5c, buffers);
            e.mem.set_u32(a[0] + 0x64, 1);
            a[0].into_ret()
        });
        e.register(MEMSET_ADDR, nop);
        e.register(PIXEL_FORMAT_SET_UP_00A622D0, nop);
        let handle_block = e.mem.alloc(0x10);
        let handle = handle_block + 8;
        e.register_double(FIXED_STRING_MAKE_00A5B690, move |_, _| handle.into_ret());
        let texture = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        e.register_double(SOURCE_TEXTURE_00A5FF10, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), 6);
            texture.into_ret()
        });
        e.mem.set_u32(handle_block, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_ec90, &args![r]).bool());
        let fills = calls(&e, SAMPLER_SET_UP_00E6DFC0);
        assert_eq!(
            fills,
            vec![
                vec![t, t + 0x700, 0, 3],
                vec![t, t + 0x758, 1, 3],
                vec![t, t + 0x7b0, 0, 5],
                vec![t, t + 0x808, 1, 5]
            ]
        );
        let created = calls(&e, SOURCE_TEXTURE_00A5FF10);
        let pixels = created[0][0];
        assert_eq!(created[0][1], handle);
        let texels = e.mem.u32(e.mem.u32(pixels + 0x5c));
        assert_eq!(e.mem.u32(texels), 0);
        assert_eq!(e.mem.u32(texels + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(t + CLIPPER_TEXTURE), texture);
        assert_eq!(e.mem.u32(texture + 4), 1);
        // The name's count and the pixel data's reference went back down.
        assert_eq!(e.mem.u32(handle_block), 0);
        assert_eq!(e.mem.u32(pixels + 4), 0);
        assert_eq!(calls(&e, 0x6100_0004), vec![vec![pixels]]);
    }

    #[test]
    fn precache_geometry_by_class() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        e.mem.set_u32(t, 0x6000);
        e.mem.set_u32(t + 0x8ac, 0x8ac0);
        e.mem.set_u32(t + 0x8b0, 0x8b00);
        let targets: Vec<u32> = (0..0x40).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(0x6000, &targets);
        e.register(0x6100_00a0, |_, _| 0u32.into_ret());
        for a in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            ADD_OBJECT_TO_GROUP,
            REPORT_00A29680,
        ] {
            e.register(a, nop);
        }
        e.map(GEOMETRY_RTTI, 8);
        let geometry = object(&mut e, 0x6400, 0x6140_0000, 0xd0);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, GEOMETRY_RTTI);
        e.register_double(0x6140_0008, move |_, _| rtti.into_ret());
        // No shader: false with the message.
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 0u32])
            .bool());
        assert_eq!(calls(&e, REPORT_00A29680).len(), 1);
        let shader = object(&mut e, 0x6800, 0x6180_0000, 0x10);
        e.register(0x6180_00bc, |_, _| 0x5ce0u32.into_ret());
        e.mem.set_u32(geometry + 0xc0, shader);
        // A data object with renderer data (not skinned; the packing finds it packed).
        let data = object(&mut e, 0x6c00, 0x61c0_0000, 0x60);
        let render_data = e.mem.alloc(0x60);
        e.mem.set_u32(data + 0x34, render_data);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xbc, 0x5c1);
        e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 1u32.into_ret());
        e.register(IS_KIND_OF, |_, a| {
            u32::from(a[0] == TRI_SHAPE_RTTI_011F4A40).into_ret()
        });
        e.register(0x61c0_00a8, |_, _| 3u32.into_ret());
        e.register(TRI_SHAPE_DATA_GET_TRI_LIST, |_, _| 0x7215u32.into_ret());
        e.register(TRI_STRIPS_DATA_GET_STRIP_LISTS, |_, _| 0x5791u32.into_ret());
        assert!(e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 0u32])
            .bool());
        assert_eq!(
            calls(&e, ADD_OBJECT_TO_GROUP)[0],
            vec![0x8ac0, 0x8b00, data, 0x5c1, 0, 1, 2]
        );
        assert_eq!(calls(&e, TRI_SHAPE_DATA_GET_TRI_LIST), vec![vec![data]]);
        assert_eq!(
            calls(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![t + 0x100], vec![t + 0x80]]
        );
        // Strips go through their strip lists; another class packs nothing.
        e.register(IS_KIND_OF, |_, a| {
            u32::from(a[0] == TRI_STRIPS_RTTI_011F4A20).into_ret()
        });
        assert!(e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 9u32])
            .bool());
        assert_eq!(calls(&e, TRI_STRIPS_DATA_GET_STRIP_LISTS), vec![vec![data]]);
        e.register(IS_KIND_OF, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 9u32])
            .bool());
        // Not a geometry: false at once.
        e.register(0x6140_0008, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 0u32])
            .bool());
        assert_eq!(calls(&e, ADD_OBJECT_TO_GROUP).len(), 3);
    }
}

#[cfg(test)]
mod b048_draw_tests {
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

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    fn ftol(_: &mut Engine, a: &[u32]) -> Ret {
        let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
        (v.trunc() as i32).into_ret()
    }

    #[test]
    fn clear_converts_the_rectangle_and_checks_the_depth_buffer() {
        let mut e = Engine::new();
        e.map(TWO_TO_32_01099378, 4);
        e.set_global(TWO_TO_32_01099378, 4294967296.0f32.to_bits());
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        let device = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        e.set(r, NiDX9Renderer::device, device);
        let group = object(&mut e, 0x6400, 0x6140_0000, 0x10);
        e.set(r, NiDX9Renderer::target_group, group);
        e.register(0x6140_008c, |_, _| 640u32.into_ret());
        e.register(0x6140_0090, |_, _| 480u32.into_ret());
        let depth = object(&mut e, 0x6800, 0x6180_0000, 0x10);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, DEPTH_BUFFER_RTTI_0126F5AC);
        e.register_double(0x6180_000c, move |_, _| rtti.into_ret());
        e.register_double(0x6140_00cc, move |_, _| depth.into_ret());
        e.register(DEPTH_HAS_STENCIL_00E7CDF0, |_, _| 1u32.into_ret());
        e.register(DEPTH_HAS_DEPTH_00E7CE10, |_, _| 0u32.into_ret());
        e.register(FTOL, ftol);
        e.register(0x6100_0000 + DEVICE_SET_VIEWPORT, nop);
        e.register(0x6100_0000 + DEVICE_CLEAR, |e, a| {
            let r: Vec<u32> = (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            (r[0] | r[1] << 10 | r[2] << 20).into_ret()
        });
        e.set(r, NiDX9Renderer::background_color, 0xff20_4060);
        e.mem.set_u32(t + 0x5e4, 1.0f32.to_bits());
        e.mem.set_u32(t + 0x5e8, 7);
        let rect = e.mem.alloc(0x10);
        for (i, v) in [0.25f32, 0.5, 0.75, 1.0].iter().enumerate() {
            e.mem.set_u32(rect + 4 * i as u32, v.to_bits());
        }
        e.call_log = Some(vec![]);
        e.call(0x00e6_f310, &args![r, rect, 7u32]);
        let clears = calls(&e, 0x6100_0000 + DEVICE_CLEAR);
        assert_eq!(
            clears,
            vec![vec![
                device,
                1,
                clears[0][2],
                1 | 2,
                0xff20_4060,
                1.0f32.to_bits(),
                7
            ]]
        );
        assert_eq!(e.mem.u32(t + 0x6d4), 640);
        assert_eq!(e.mem.u32(t + 0x6d8), 480);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_SET_VIEWPORT),
            vec![vec![device, t + 0x6cc]]
        );
        // left 0.25·640, top (1 − 0.75)·480, right 0.5·640, bottom 0.
        let corners: Vec<u32> = calls(&e, FTOL)
            .iter()
            .map(|a| f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as u32)
            .collect();
        assert_eq!(corners, vec![160, 120, 320, 0]);
        // Nothing to clear, or skipping: no call.
        e.call(0x00e6_f310, &args![r, rect, 0u32]);
        e.mem.set_u8(t + SKIP_END_SCENE, 1);
        e.call(0x00e6_f310, &args![r, rect, 1u32]);
        assert_eq!(calls(&e, 0x6100_0000 + DEVICE_CLEAR).len(), 1);
        // Without a rectangle the whole target, without a depth buffer only the target bit.
        e.mem.set_u8(t + SKIP_END_SCENE, 0);
        e.register(0x6140_00cc, |_, _| 0u32.into_ret());
        e.call(0x00e6_f310, &args![r, 0u32, 7u32]);
        let clears = calls(&e, 0x6100_0000 + DEVICE_CLEAR);
        assert_eq!(clears[1][3], 1);
        assert_eq!(clears.len(), 2);
        assert_eq!(calls(&e, FTOL).len(), 4);
    }

    fn draw_rig(e: &mut Engine) -> (Ptr<NiDX9Renderer>, u32, u32, u32) {
        let r = e.new_object::<NiDX9Renderer>();
        let device = object(e, 0x6000, 0x6100_0000, 0x10);
        e.set(r, NiDX9Renderer::device, device);
        let targets: Vec<u32> = (0..0x40).map(|n| 0x6140_0000 + 4 * n).collect();
        e.put_vtable(0x6400, &targets);
        e.mem.set_u32(r.addr(), 0x6400);
        e.register(0x6140_00a0, |_, _| 0u32.into_ret());
        e.register(0x6100_0000 + DEVICE_DRAW_PRIMITIVE, nop);
        e.register(0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE, nop);
        let geometry = e.mem.alloc(0xd0);
        let data = object(e, 0x6800, 0x6180_0000, 0x60);
        e.register(0x6180_0090, |_, _| 4u32.into_ret());
        e.mem.set_u32(geometry + 0xb8, data);
        let shader = object(e, 0x6c00, 0x61c0_0000, 0x10);
        e.register(0x61c0_0090, nop);
        e.mem.set_u32(geometry + 0xc0, shader);
        let bound = e.mem.alloc(0x10);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_u32(bound + 4 * i as u32, v.to_bits());
        }
        e.mem.set_u32(geometry + 0x20, bound);
        let render_data = e.mem.alloc(0x60);
        e.mem.set_u32(data + 0x34, render_data);
        (r, geometry, data, render_data)
    }

    #[test]
    fn draw_without_an_index_buffer_draws_primitives() {
        let mut e = Engine::new();
        let (r, geometry, data, render_data) = draw_rig(&mut e);
        e.mem.set_u32(render_data + 0x3c, 4);
        e.mem.set_u32(render_data + 0x38, 10);
        e.mem.set_u32(render_data + 0x40, 20);
        e.mem.set_u16(data + 0xe, 0x3fff);
        e.call_log = Some(vec![]);
        e.call(0x00e7_4840, &args![r, geometry]);
        let device = e.get(r, NiDX9Renderer::device);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_PRIMITIVE),
            vec![vec![device, 4, 10, 20]]
        );
        assert_eq!(e.mem.u16(data + 0xe), 0x3000);
        assert_eq!(e.mem.u32(r.addr() + DRAWN_BOUND + 4), 2.0f32.to_bits());
        assert_eq!(e.mem.u32(r.addr() + DRAWN_BOUND + 0xc), 4.0f32.to_bits());
    }

    #[test]
    fn draw_with_an_index_buffer_draws_each_strip() {
        let mut e = Engine::new();
        let (r, geometry, _, render_data) = draw_rig(&mut e);
        e.mem.set_u32(render_data + 0x34, 1);
        e.mem.set_u32(render_data + 0x3c, 5);
        e.mem.set_u32(render_data + 0x38, 10);
        e.mem.set_u32(render_data + 0x18, 30);
        e.mem.set_u32(render_data + 0x48, 2);
        let lengths = e.mem.alloc(4);
        e.mem.set_u16(lengths, 6);
        e.mem.set_u16(lengths + 2, 5);
        e.mem.set_u32(render_data + 0x4c, lengths);
        e.call_log = Some(vec![]);
        e.call(0x00e7_4840, &args![r, geometry]);
        let device = e.get(r, NiDX9Renderer::device);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE),
            vec![
                vec![device, 5, 10, 0, 30, 0, 4],
                vec![device, 5, 10, 0, 30, 6, 3]
            ]
        );
        // Without vertices nothing is drawn.
        e.register(0x6180_0090, |_, _| 0x1_0000u32.into_ret());
        e.call(0x00e7_4840, &args![r, geometry]);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE).len(),
            2
        );
    }
}

#[cfg(test)]
mod b048_caps_tests {
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

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    #[test]
    fn device_caps_set_the_flags_and_defaults() {
        let mut e = Engine::new();
        e.map(0x0126_f050, 0x98);
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        let device = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        e.set(r, NiDX9Renderer::device, device);
        for a in [
            STRNCPY_S,
            REPORT_00A29680,
            CAPS_SET_UP_00E7F830,
            CAPS_SET_UP_00E7F840,
            CAPS_SET_UP_00E7F850,
        ] {
            e.register(a, nop);
        }
        e.register(0x6100_0000 + GET_DEVICE_CAPS_SLOT, |_, _| {
            0x8000_0000u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e6_c240, &args![r, 0u32]).bool());
        assert_eq!(
            calls(&e, STRNCPY_S),
            vec![vec![
                ERROR_MESSAGE_011F4648,
                0x100,
                NO_DEVICE_CAPS_010ED924,
                0xff
            ]]
        );
        e.register(0x6100_0000 + GET_DEVICE_CAPS_SLOT, |_, _| 0u32.into_ret());
        e.mem.set_u32(t + 0x320, 1);
        e.mem.set_u32(t + 0x324, 4);
        assert!(!e.call(0x00e6_c240, &args![r, 0u32]).bool());
        assert_eq!(
            calls(&e, REPORT_00A29680).last().unwrap(),
            &vec![NO_MULTI_TEXTURE_REPORT_010ED840]
        );
        e.mem.set_u32(t + 0x320, 8);
        e.mem.set_u32(t + 0x2c8, 0x0001_0502);
        e.mem.set_u32(t + 0x2cc, 0x0400_0400);
        e.mem.set_u32(t + 0x298, 0x2000_0000);
        e.mem.set_u32(t + 0x2ac, 0x000c_0000);
        e.mem.set_u32(t + 0x334, 4);
        e.mem.set_u32(t + 0x348, 0xfffe_0300);
        e.mem.set_u32(t + 0x350, 0xffff_0300);
        e.mem.set_u32(t + 0x358, 0xffff_0300);
        e.mem.set_u32(t + 0x37c, 0x37c);
        assert!(e.call(0x00e6_c240, &args![r, 0u32]).bool());
        assert_eq!(e.mem.u32(t + 0x5ec), 4 | 2);
        assert_eq!(e.mem.u32(t + 0x6e4), 4);
        assert_eq!(
            (
                e.mem.u8(t + 0x6f4),
                e.mem.u8(t + 0x6f5),
                e.mem.u8(t + 0x8a4),
                e.mem.u8(t + 0x8a5)
            ),
            (1, 1, 1, 1)
        );
        assert_eq!(e.mem.u32(t + 0x8a0), 0x37c);
        assert_eq!(e.mem.u32(t + 0x6ec), 0xffff_0300);
        assert_eq!(e.mem.u32(t + 0x93c), 2);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F830), vec![vec![8]]);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F840), vec![vec![4]]);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F850), vec![vec![0x10]]);
        assert_eq!(e.global::<u8>(0x0126_f0d1), 1);
        assert_eq!(e.global::<u8>(0x0126_f0d2), 1);
        assert_eq!(e.global::<u32>(0x0126_f09c), 3);
        assert_eq!(e.global::<u32>(0x0126_f098), 3);
        assert_eq!(e.global::<u32>(0x0126_f0b0), 3);
        assert_eq!(e.global::<u32>(0x0126_f088), 2);
        assert_eq!(e.global::<u32>(0x0126_f0e4), 0x103);
        // Without texture caps bit 1: 8, and a lower pixel version passes the stage count.
        e.mem.set_u32(t + 0x2c8, 0);
        e.mem.set_u32(t + 0x334, 2);
        e.mem.set_u32(t + 0x358, 0xffff_0100);
        assert!(e.call(0x00e6_c240, &args![r, 0u32]).bool());
        assert_eq!(e.mem.u32(t + 0x5ec), 8);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F850)[1], vec![8]);
        assert_eq!(e.global::<u32>(0x0126_f0e4), 2);
    }

    /// An object whose class chain (virtual +0xC) holds `rtti`.
    fn of_kind(e: &mut Engine, table: u32, base: u32, rtti: u32) -> u32 {
        let o = object(e, table, base, 0x10);
        let link = e.mem.alloc(8);
        e.mem.set_u32(link + 4, rtti);
        e.register_double(base + 0xc, move |_, _| link.into_ret());
        o
    }

    #[test]
    fn begin_using_a_group_sets_its_buffers() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        let targets: Vec<u32> = (0..0x80).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(0x6000, &targets);
        e.mem.set_u32(t, 0x6000);
        e.set(r, NiDX9Renderer::device, 0xde01);
        e.set(r, NiDX9Renderer::object_884, 0x884d);
        e.map(0x0126_f590, 0x60);
        for a in [
            CLEAR_RENDER_TARGET_00E7CC50,
            REPORT_00A29680,
            0x6100_0188,
            0x6100_0194,
        ] {
            e.register(a, nop);
        }
        e.register(CLEAR_DEPTH_STENCIL_00E7CCA0, |_, _| 1u32.into_ret());
        let buffer = of_kind(&mut e, 0x6400, 0x6140_0000, RENDER_BUFFER_RTTI_0126F59C);
        e.register(0x6140_005c, |_, _| 1u32.into_ret());
        let group = object(&mut e, 0x6800, 0x6180_0000, 0x10);
        e.register(0x6180_00a4, |_, _| 1u32.into_ret());
        e.register_double(0x6180_00c8, move |_, _| buffer.into_ret());
        e.register(0x6180_00cc, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_eea0, &args![r, group, 7u32]).bool());
        assert_eq!(
            calls(&e, CLEAR_RENDER_TARGET_00E7CC50),
            vec![vec![0xde01, 1], vec![0xde01, 2], vec![0xde01, 3]]
        );
        assert_eq!(calls(&e, 0x6140_005c), vec![vec![buffer, 0xde01, 0]]);
        assert_eq!(calls(&e, CLEAR_DEPTH_STENCIL_00E7CCA0), vec![vec![0xde01]]);
        assert_eq!(e.get(r, NiDX9Renderer::target_group), group);
        assert_eq!(e.mem.u32(t + 0x88c), 0);
        assert_eq!(calls(&e, 0x6100_0188), vec![vec![t, 0, 7]]);
        // A failing target falls back to the default group.
        e.register(0x6140_005c, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e6_eea0, &args![r, group, 7u32]).bool());
        assert_eq!(calls(&e, 0x6100_0194), vec![vec![t, 0x884d, 7]]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![BEGIN_GROUP_FAILED_TEXT, OUTPUT_TARGET_FAILED_TEXT]]
        );
        // A depth buffer that fails.
        e.register(0x6140_005c, |_, _| 1u32.into_ret());
        let depth = of_kind(&mut e, 0x6c00, 0x61c0_0000, DEPTH_BUFFER_RTTI_0126F5AC);
        e.register(0x61c0_0060, |_, _| 0u32.into_ret());
        e.register_double(0x6180_00cc, move |_, _| depth.into_ret());
        assert!(!e.call(0x00e6_eea0, &args![r, group, 7u32]).bool());
        assert_eq!(
            calls(&e, REPORT_00A29680)[1],
            vec![BEGIN_GROUP_FAILED_TEXT, DEPTH_STENCIL_FAILED_TEXT]
        );
        // Skipping: true at once.
        e.mem.set_u8(t + SKIP_END_SCENE, 1);
        assert!(e.call(0x00e6_eea0, &args![r, group, 7u32]).bool());
        assert_eq!(calls(&e, CLEAR_RENDER_TARGET_00E7CC50).len(), 9);
    }
}

#[cfg(test)]
mod b048_partial_draw_tests {
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

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    #[test]
    fn partial_draw_uses_the_active_records() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        let device = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        e.set(r, NiDX9Renderer::device, device);
        let targets: Vec<u32> = (0..0x40).map(|n| 0x6140_0000 + 4 * n).collect();
        e.put_vtable(0x6400, &targets);
        e.mem.set_u32(r.addr(), 0x6400);
        e.register(0x6140_00a0, |_, _| 0u32.into_ret());
        e.register(0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE, nop);
        let geometry = object(&mut e, 0x6800, 0x6180_0000, 0xd0);
        let data = object(&mut e, 0x6c00, 0x61c0_0000, 0x60);
        e.register(0x61c0_0090, |_, _| 4u32.into_ret());
        e.mem.set_u32(geometry + 0xb8, data);
        let shader = object(&mut e, 0x7000, 0x6200_0000, 0x10);
        e.register(0x6200_0090, nop);
        e.mem.set_u32(geometry + 0xc0, shader);
        let render_data = e.mem.alloc(0x60);
        e.mem.set_u32(data + 0x34, render_data);
        e.mem.set_u32(render_data + 0x34, 1);
        e.mem.set_u32(render_data + 0x48, 1);
        e.mem.set_u32(render_data + 0x40, 9);
        e.mem.set_u32(render_data + 0x3c, 4);
        // Two records, only the second active.
        let draw = e.mem.alloc(0xd0);
        let records = e.mem.alloc(0x28);
        e.mem.set_u32(records + 0x14, 3);
        e.mem.set_u32(records + 0x14 + 0xc, 5);
        e.mem.set_u8(records + 0x14 + 0x10, 1);
        e.mem.set_u32(draw + 0xc4, records);
        e.mem.set_u32(draw + 0xc8, 2);
        e.register_double(0x6180_0028, move |_, _| draw.into_ret());
        e.mem.set_u16(data + 0xe, 0xffff);
        e.call_log = Some(vec![]);
        e.call(0x00e7_45a0, &args![r, geometry]);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE),
            vec![vec![device, 4, 0, 0, 0, 3, 5]]
        );
        assert_eq!(e.mem.u16(data + 0xe), 0xf000);
        // Without a draw object the whole strip is drawn.
        e.register(0x6180_0028, |_, _| 0u32.into_ret());
        e.call(0x00e7_45a0, &args![r, geometry]);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE)[1],
            vec![device, 4, 0, 0, 0, 0, 9]
        );
    }

    #[test]
    fn member_set_up_clears_and_fills_the_tables() {
        let mut e = Engine::new();
        e.map(CLEARED_BLOCK_0126EFD8, 0x80);
        e.map(DEFAULT_WORDS_011A946C, 0x18);
        e.map(FRUSTUM_NEAR_0101E2BC, 4);
        e.map(FLOAT_SOURCE_01016410, 4);
        e.set_global(FRUSTUM_NEAR_0101E2BC, 2.0f32.to_bits());
        e.set_global(DEFAULT_WORDS_011A946C + 4, 0x44u32);
        e.register(MEMSET_ADDR, nop);
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        e.mem.set_u32(t + 0x700, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00e6_b990, &args![r]);
        assert_eq!(e.mem.u32(t + 0x5e4), ONE);
        assert_eq!(e.mem.u32(t + 0x5e0), 0xff80_8080);
        assert_eq!(e.mem.u32(t + 0x638), 0x44);
        assert_eq!(e.mem.u32(t + 0x650), 0x44);
        assert_eq!(e.mem.u32(t + 0x684), 2.0f32.to_bits());
        assert_eq!(e.mem.u32(t + 0x6b4), ONE);
        assert_eq!(e.mem.u32(t + 0x6e0), ONE);
        assert_eq!(e.mem.u32(t + 0x700), 0);
        assert_eq!(e.mem.u32(t + 0x880), 0x16);
        assert_eq!(e.mem.u32(t + 0x97c), ONE);
        assert_eq!(e.global::<u8>(CLEARED_BLOCK_0126EFD8 + 20), 0x18);
        assert_eq!(e.global::<u8>(CLEARED_BLOCK_0126EFD8 + 116), 0x80);
        assert_eq!(
            calls(&e, MEMSET_ADDR),
            vec![
                vec![t + DEVICE_CAPS, 0, 0x130],
                vec![t + 0x68c, 0, 0x40],
                vec![CLEARED_BLOCK_0126EFD8, 0, 0x76]
            ]
        );
    }
}

#[cfg(test)]
mod tests_mode_lookups {
    use super::*;

    #[test]
    fn the_format_lookup_checks_the_flag_byte_and_the_id() {
        let mut e = Engine::new();
        e.register(RECORD_ID_00E7ACB0, |_, a| (a[0] as i32 * 2).into_ret());
        let this = e.mem.alloc(0x140);
        let (node1, node2) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        let (r1, r2) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(this + 0x134, node1);
        e.mem.set_u32(node1, node2);
        e.mem.set_u32(node1 + 8, r1);
        e.mem.set_u32(node2 + 8, r2);
        e.mem.set_u32(r1, 21);
        e.mem.set_u8(r1 + 4, 1);
        e.mem.set_u32(r2, 22);
        e.mem.set_u8(r2 + 5, 1);
        assert_eq!(
            e.call(0x00e7_aed0, &args![this, 1u32, 42u32]).u32(),
            fn_00e69d00(&mut e, 21)
        );
        assert_eq!(e.call(0x00e7_aed0, &args![this, 1u32, 44u32]).u32(), 0);
        assert_eq!(
            e.call(0x00e7_aed0, &args![this, 0u32, 44u32]).u32(),
            fn_00e69d00(&mut e, 22)
        );
    }

    #[test]
    fn the_mode_lookup_keeps_the_last_match_and_its_rate() {
        let mut e = Engine::new();
        e.register(NEAREST_RATE_00E7AF60, |_, a| (a[1] + 1).into_ret());
        let this = e.mem.alloc(0x460);
        let table = e.mem.alloc(0xc);
        let (m1, m2) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        e.mem.set_u32(this + 0x454, table);
        e.mem.set_u16(this + 0x45a, 3);
        e.mem.set_u32(table, m1);
        e.mem.set_u32(table + 4, 0);
        e.mem.set_u32(table + 8, m2);
        for m in [m1, m2] {
            e.mem.set_u32(m, 800);
            e.mem.set_u32(m + 4, 600);
            e.mem.set_u32(m + 0xc, 22);
        }
        let rate = e.mem.alloc(4);
        e.mem.set_u32(rate, 60);
        assert_eq!(
            e.call(0x00e7_b190, &args![this, 22u32, 800u32, 600u32, rate])
                .u32(),
            m2
        );
        assert_eq!(e.mem.u32(rate), 62);
        assert_eq!(
            e.call(0x00e7_b190, &args![this, 23u32, 800u32, 600u32, rate])
                .u32(),
            0
        );
    }
}

// ---- Large x87 functions (a07) ----

/// The camera vectors the quad builders read. `Do_SetCameraData` (`00e6c780`) stores its
/// right vector at +0x64C (and +0x634) and its up vector at +0x658 (and +0x640); the Xbox
/// PDB's `m_kCamRight`, `m_kCamUp`, `m_kModelCamRight`, `m_kModelCamUp` are four `NiPoint3`
/// in the same order.
const QUAD_RIGHT: u32 = 0x64c;
const QUAD_UP: u32 = 0x658;

/// How a quad builder's instructions take the camera vectors and the positions (the two
/// builders differ only here; it decides which NaN an instruction keeps): `true` where the
/// component is the instruction's `float` memory operand (`x87_m32`, a signalling NaN
/// still signalling), `false` where it was loaded with `FLD` (quiet). Values the code
/// computed and stored are never signalling, so for them both are the same.
struct QuadOperands {
    /// The rotated corner offsets with colours: up·B, right·A, up·A, right·(−B) (x, y, z
    /// each; A and B as in [`rotated_offsets`]).
    rotated_colored: [bool; 12],
    /// The same without colours.
    rotated_plain: [bool; 12],
    /// The normal's z subtracts `up.x·right.y` with `up.x` loaded (`true`) or with
    /// `right.y` loaded (`false`).
    normal_z_up_loaded: bool,
    /// The fourth corner's x adds the position as the memory operand.
    corner3_x_memory: bool,
}

/// A camera component (`index` 0..3 of the vector at `vector`).
fn quad_camera(e: &Engine, this: u32, vector: u32, index: u32) -> u32 {
    e.mem.u32(this + vector + 4 * index)
}

/// `FLD dword` (`false`) or a `float` memory operand (`true`).
fn quad_operand(bits: u32, memory: bool) -> f64 {
    if memory {
        x87_m32(bits)
    } else {
        x87_load32(bits)
    }
}

/// `value · component`, stored as a float.
fn quad_times(value: f64, bits: u32, memory: bool) -> u32 {
    x87_store32(x87_mul(value, quad_operand(bits, memory)))
}

/// The packed colour of an `NiColorA` (`r, g, b, a`): each channel times 255.0 (`double`
/// at `0101e568`), stored as a float and converted with `FISTP` (round to nearest),
/// packed `((a << 8 | r) << 8 | g) << 8 | b`.
fn quad_color(e: &Engine, color: u32) -> u32 {
    let scale = f64::from_bits(e.mem.u64(SCALE_TO_BYTE));
    let channel = |at: u32| {
        let product = x87_store32(x87_mul(x87_load32(e.mem.u32(at)), scale));
        crate::x87::fistp(x87_load32(product)) as u32
    };
    let alpha = channel(color + 0xc);
    let red = channel(color);
    let green = channel(color + 4);
    let blue = channel(color + 8);
    (((((alpha << 8) | red) << 8) | green) << 8) | blue
}

/// The face normal both builders give every vertex: `up × right` component by component
/// (`up.z·right.y − up.y·right.z`, `right.z·up.x − up.z·right.x`, `up.y·right.x −
/// up.x·right.y`).
fn quad_normal(e: &Engine, this: u32, ops: &QuadOperands) -> [u32; 3] {
    let r = |i| quad_camera(e, this, QUAD_RIGHT, i);
    let u = |i| quad_camera(e, this, QUAD_UP, i);
    let x = x87_sub(
        x87_mul(x87_load32(u(2)), x87_m32(r(1))),
        x87_mul(x87_load32(u(1)), x87_m32(r(2))),
    );
    let y = x87_sub(
        x87_mul(x87_load32(r(2)), x87_m32(u(0))),
        x87_mul(x87_load32(u(2)), x87_m32(r(0))),
    );
    let last = if ops.normal_z_up_loaded {
        x87_mul(x87_load32(u(0)), x87_m32(r(1)))
    } else {
        x87_mul(x87_load32(r(1)), x87_m32(u(0)))
    };
    let z = x87_sub(x87_mul(x87_load32(u(1)), x87_m32(r(0))), last);
    [x87_store32(x), x87_store32(y), x87_store32(z)]
}

/// The corner offsets of a rotated quad: `FSINCOS` of the angle (sine and cosine stored as
/// floats), `A = (sin + cos)·size`, `B = (cos − sin)·size` (floats), then the first offset
/// `right·A + up·B` and the second `right·(−B) + up·A`, every product and sum a float.
fn rotated_offsets(
    e: &Engine,
    this: u32,
    angle: u32,
    size: u32,
    kinds: &[bool; 12],
) -> ([u32; 3], [u32; 3]) {
    let (sin, cos) = x87_fsincos(X87Ext::load32(angle));
    let sin = x87_load32(sin.store32());
    let cos = x87_load32(cos.store32());
    let size = x87_load32(size);
    let a = x87_load32(x87_store32(x87_mul(x87_add(sin, cos), size)));
    let b = x87_load32(x87_store32(x87_mul(x87_sub(cos, sin), size)));
    let r = |i: usize| quad_camera(e, this, QUAD_RIGHT, i as u32);
    let u = |i: usize| quad_camera(e, this, QUAD_UP, i as u32);
    let mut up_b = [0u32; 3];
    let mut right_a = [0u32; 3];
    for i in 0..3 {
        up_b[i] = quad_times(b, u(i), kinds[i]);
    }
    for i in 0..3 {
        right_a[i] = quad_times(a, r(i), kinds[3 + i]);
    }
    let mut first = [0u32; 3];
    for i in 0..3 {
        first[i] = x87_store32(x87_add(x87_load32(right_a[i]), x87_load32(up_b[i])));
    }
    let mut up_a = [0u32; 3];
    for i in 0..3 {
        up_a[i] = quad_times(a, u(i), kinds[6 + i]);
    }
    // `FCHS` of B, stored as a float.
    let minus_b = x87_load32(x87_store32(-b));
    let mut right_minus_b = [0u32; 3];
    for i in 0..3 {
        right_minus_b[i] = quad_times(minus_b, r(i), kinds[9 + i]);
    }
    let mut second = [0u32; 3];
    for i in 0..3 {
        second[i] = x87_store32(x87_add(x87_load32(right_minus_b[i]), x87_load32(up_a[i])));
    }
    (first, second)
}

/// Writes one quad: four vertices `stride` bytes apart (position, the normal, the colour,
/// two texture coordinates, then with `extra` its byte as a float). The corners are the
/// position minus the first offset (u 0, v 1), minus the second (1, 1), plus the first
/// (1, 0) and plus the second (0, 0). Each corner reads the position again, then writes the
/// vertex, then reads the byte.
#[allow(clippy::too_many_arguments)]
fn write_quad(
    e: &mut Engine,
    dest: u32,
    stride: u32,
    position: u32,
    color: u32,
    offsets: &([u32; 3], [u32; 3]),
    normal: &[u32; 3],
    extra: Option<u32>,
    corner3_x_memory: bool,
) {
    const ONE_BITS: u32 = 0x3f80_0000;
    let uv = [(0, ONE_BITS), (ONE_BITS, ONE_BITS), (ONE_BITS, 0), (0, 0)];
    for corner in 0..4u32 {
        let vertex = dest.wrapping_add(corner * stride);
        let offset = if corner % 2 == 0 {
            &offsets.0
        } else {
            &offsets.1
        };
        let mut xyz = [0u32; 3];
        for j in 0..3u32 {
            let p = e.mem.u32(position.wrapping_add(4 * j));
            let o = x87_load32(offset[j as usize]);
            xyz[j as usize] = if corner < 2 {
                x87_store32(x87_sub(x87_load32(p), o))
            } else {
                let memory = j == 1 || (corner == 3 && j == 0 && corner3_x_memory);
                x87_store32(x87_add(quad_operand(p, memory), o))
            };
        }
        e.mem.set_u32(vertex.wrapping_add(0x18), color);
        for j in 0..3u32 {
            e.mem.set_u32(vertex.wrapping_add(4 * j), xyz[j as usize]);
            e.mem
                .set_u32(vertex.wrapping_add(0xc + 4 * j), normal[j as usize]);
        }
        let (u, v) = uv[corner as usize];
        e.mem.set_u32(vertex.wrapping_add(0x1c), u);
        e.mem.set_u32(vertex.wrapping_add(0x20), v);
        if let Some(bytes) = extra {
            let value = e.mem.u8(bytes);
            e.mem
                .set_u32(vertex.wrapping_add(0x24), x87_store32(f64::from(value)));
        }
    }
}

/// The body both quad builders share (see [`fn_00e69ea0`]).
#[allow(clippy::too_many_arguments)]
fn build_quads(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    mut dest: u32,
    mut positions: u32,
    mut colors: u32,
    mut sizes: u32,
    mut scales: u32,
    mut angles: u32,
    mut extra: Option<u32>,
    count: u16,
    stride: u32,
    ops: &QuadOperands,
) {
    let t = this.addr();
    let normal = quad_normal(e, t, ops);
    let rotated = angles != 0;
    let with_colors = colors != 0;
    let (sum, difference) = if rotated {
        ([0u32; 3], [0u32; 3])
    } else {
        let r = |e: &Engine, i| quad_camera(e, t, QUAD_RIGHT, i);
        let u = |e: &Engine, i| quad_camera(e, t, QUAD_UP, i);
        let mut difference = [0u32; 3];
        for i in 0..3u32 {
            difference[i as usize] = x87_store32(x87_sub(x87_load32(u(e, i)), x87_m32(r(e, i))));
        }
        let sum = [
            x87_store32(x87_add(x87_load32(u(e, 0)), x87_m32(r(e, 0)))),
            x87_store32(x87_add(x87_load32(r(e, 1)), x87_m32(u(e, 1)))),
            x87_store32(x87_add(x87_load32(r(e, 2)), x87_m32(u(e, 2)))),
        ];
        (sum, difference)
    };
    let mut left = u32::from(count);
    while left != 0 {
        let color = if with_colors {
            quad_color(e, colors)
        } else {
            0xffff_ffff
        };
        let size = x87_store32(x87_mul(
            x87_load32(e.mem.u32(sizes)),
            x87_m32(e.mem.u32(scales)),
        ));
        let offsets = if rotated {
            let kinds = if with_colors {
                &ops.rotated_colored
            } else {
                &ops.rotated_plain
            };
            rotated_offsets(e, t, e.mem.u32(angles), size, kinds)
        } else {
            let s = x87_load32(size);
            let mut first = [0u32; 3];
            let mut second = [0u32; 3];
            for i in 0..3 {
                first[i] = x87_store32(x87_mul(x87_load32(sum[i]), s));
                second[i] = x87_store32(x87_mul(x87_load32(difference[i]), s));
            }
            (first, second)
        };
        write_quad(
            e,
            dest,
            stride,
            positions,
            color,
            &offsets,
            &normal,
            extra,
            ops.corner3_x_memory,
        );
        if with_colors {
            colors = colors.wrapping_add(0x10);
        }
        sizes = sizes.wrapping_add(4);
        scales = scales.wrapping_add(4);
        if rotated {
            angles = angles.wrapping_add(4);
        }
        positions = positions.wrapping_add(0xc);
        extra = extra.map(|bytes| bytes.wrapping_add(1));
        dest = dest.wrapping_add(4 * stride);
        left -= 1;
    }
}

// Translated from 00e69ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, seven words: the vertices, the positions, the
/// colours or 0, two size arrays, the angles or 0, the count as a `u16`). Builds a camera
/// facing quad per element: four vertices of 0x24 bytes (position, normal, `D3DCOLOR`, u, v)
/// from its position (`NiPoint3`), its size (the two sizes multiplied), the camera's right
/// (+0x64C) and up (+0x658) vectors and, with angles, its angle (`FSINCOS`). The normal is
/// `up × right` for all; the colour is the packed `NiColorA` (`0xFFFFFFFF` without colours).
/// Without angles the corner offsets are `(up + right)·size` and `(up − right)·size`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e69ea0(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    vertices: u32,
    positions: u32,
    colors: u32,
    sizes: u32,
    scales: u32,
    angles: u32,
    count: u16,
) {
    const OPS: QuadOperands = QuadOperands {
        rotated_colored: [
            true, false, false, false, false, false, false, false, true, false, false, true,
        ],
        rotated_plain: [
            false, false, false, true, false, false, false, false, true, false, true, true,
        ],
        normal_z_up_loaded: false,
        corner3_x_memory: false,
    };
    build_quads(
        e, this, vertices, positions, colors, sizes, scales, angles, None, count, 0x24, &OPS,
    );
}

// Translated from 00e6ab40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, eight words): [`fn_00e69ea0`] with one more
/// array, of bytes (the seventh word, before the count), each vertex 0x28 bytes with its
/// element's byte as a float at +0x24.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e6ab40(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    vertices: u32,
    positions: u32,
    colors: u32,
    sizes: u32,
    scales: u32,
    angles: u32,
    bytes: u32,
    count: u16,
) {
    const OPS: QuadOperands = QuadOperands {
        rotated_colored: [
            false, false, false, false, false, false, false, false, true, false, false, true,
        ],
        rotated_plain: [
            false, false, false, false, false, false, false, true, true, true, false, true,
        ],
        normal_z_up_loaded: true,
        corner3_x_memory: true,
    };
    build_quads(
        e,
        this,
        vertices,
        positions,
        colors,
        sizes,
        scales,
        angles,
        Some(bytes),
        count,
        0x28,
        &OPS,
    );
}

#[cfg(test)]
mod a07_quad_tests {
    use super::*;

    const RENDERER_VT: u32 = 0x6400;

    fn renderer(e: &mut Engine) -> Ptr<NiDX9Renderer> {
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        e.mem.set_u32(t, RENDERER_VT);
        // right (1, 0, 0), up (0, 1, 0)
        for (i, v) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0].iter().enumerate() {
            e.mem.set_u32(t + QUAD_RIGHT + 4 * i as u32, v.to_bits());
        }
        r
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let p = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(p + 4 * i as u32, v.to_bits());
        }
        p
    }

    fn vertex(e: &Engine, v: u32, n: u32) -> Vec<f32> {
        (0..n)
            .map(|i| f32::from_bits(e.mem.u32(v + 4 * i)))
            .collect()
    }

    fn scale_constant(e: &mut Engine) {
        e.map(SCALE_TO_BYTE & !0xfff, 0x1000);
        e.mem.set_u64(SCALE_TO_BYTE, 255.0f64.to_bits());
    }

    #[test]
    fn quads_without_angles_take_up_plus_and_minus_right() {
        let mut e = Engine::new();
        scale_constant(&mut e);
        let r = renderer(&mut e);
        let positions = floats(&mut e, &[10.0, 20.0, 30.0]);
        let sizes = floats(&mut e, &[2.0]);
        let scales = floats(&mut e, &[0.5]);
        let out = e.mem.alloc(0x90);
        e.call(
            0x00e6_9ea0,
            &args![r, out, positions, 0u32, sizes, scales, 0u32, 1u32],
        );
        let expect = [
            [9.0, 19.0, 30.0, 0.0, 1.0],
            [11.0, 19.0, 30.0, 1.0, 1.0],
            [11.0, 21.0, 30.0, 1.0, 0.0],
            [9.0, 21.0, 30.0, 0.0, 0.0],
        ];
        for (k, x) in expect.iter().enumerate() {
            let at = out + 0x24 * k as u32;
            let v = vertex(&e, at, 9);
            assert_eq!(&v[..3], &x[..3]);
            assert_eq!(&v[3..6], &[0.0, 0.0, 1.0]);
            assert_eq!(e.mem.u32(at + 0x18), 0xffff_ffff);
            assert_eq!(&v[7..], &x[3..]);
        }
        // With colours: r 1, g 0.5, b 0, a 1 packs to 0xFFFF8000 (127.5 rounds to even).
        let colors = floats(&mut e, &[1.0, 0.5, 0.0, 1.0]);
        e.call(
            0x00e6_9ea0,
            &args![r, out, positions, colors, sizes, scales, 0u32, 1u32],
        );
        for k in 0..4 {
            assert_eq!(e.mem.u32(out + 0x24 * k + 0x18), 0xffff_8000);
        }
        assert_eq!(vertex(&e, out + 0x24 * 3, 3), vec![9.0, 21.0, 30.0]);
        // A count of 0 writes nothing.
        let fresh = e.mem.alloc(0x90);
        e.mem.set_u32(fresh, 0x1234);
        e.call(
            0x00e6_9ea0,
            &args![r, fresh, positions, colors, sizes, scales, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(fresh), 0x1234);
    }

    #[test]
    fn quads_with_angles_rotate_the_offsets() {
        let mut e = Engine::new();
        scale_constant(&mut e);
        let r = renderer(&mut e);
        let positions = floats(&mut e, &[10.0, 20.0, 30.0, 0.0, 0.0, 0.0]);
        let sizes = floats(&mut e, &[1.0, 1.0]);
        let scales = floats(&mut e, &[1.0, 2.0]);
        // Angle 0: the offsets of the unrotated quad; angle pi: (nearly) their negation.
        let angles = floats(&mut e, &[0.0, std::f32::consts::PI]);
        let out = e.mem.alloc(0x120);
        e.call(
            0x00e6_9ea0,
            &args![r, out, positions, 0u32, sizes, scales, angles, 2u32],
        );
        assert_eq!(vertex(&e, out, 3), vec![9.0, 19.0, 30.0]);
        assert_eq!(vertex(&e, out + 0x24, 3), vec![11.0, 19.0, 30.0]);
        assert_eq!(vertex(&e, out + 0x48, 3), vec![11.0, 21.0, 30.0]);
        let second = vertex(&e, out + 0x90, 3);
        assert!((second[0] - 2.0).abs() < 1e-5 && (second[1] - 2.0).abs() < 1e-5);
        let third = vertex(&e, out + 0x90 + 0x24, 3);
        assert!((third[0] + 2.0).abs() < 1e-5 && (third[1] - 2.0).abs() < 1e-5);
        assert_eq!(e.mem.u32(out + 0x90 + 0x18), 0xffff_ffff);
        // With colours the colour is packed per element.
        let colors = floats(&mut e, &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00e6_9ea0,
            &args![r, out, positions, colors, sizes, scales, angles, 2u32],
        );
        assert_eq!(e.mem.u32(out + 0x18), 0xff00_0000);
        assert_eq!(e.mem.u32(out + 0x90 + 0x18), 0);
        assert_eq!(vertex(&e, out + 0x24, 3), vec![11.0, 19.0, 30.0]);
    }

    #[test]
    fn byte_quads_add_the_byte_as_a_float() {
        let mut e = Engine::new();
        scale_constant(&mut e);
        let r = renderer(&mut e);
        let positions = floats(&mut e, &[10.0, 20.0, 30.0, 1.0, 2.0, 3.0]);
        let sizes = floats(&mut e, &[2.0, 1.0]);
        let scales = floats(&mut e, &[0.5, 1.0]);
        let colors = floats(&mut e, &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
        let bytes = e.mem.alloc(4);
        e.mem.set_u8(bytes, 7);
        e.mem.set_u8(bytes + 1, 9);
        let out = e.mem.alloc(0x140);
        e.call(
            0x00e6_ab40,
            &args![r, out, positions, colors, sizes, scales, 0u32, bytes, 2u32],
        );
        let first = vertex(&e, out, 10);
        assert_eq!(&first[..6], &[9.0, 19.0, 30.0, 0.0, 0.0, 1.0]);
        assert_eq!(&first[7..], &[0.0, 1.0, 7.0]);
        assert_eq!(e.mem.u32(out + 0x18), 0x0000_00ff);
        assert_eq!(vertex(&e, out + 0x28 * 3, 3), vec![9.0, 21.0, 30.0]);
        assert_eq!(f32::from_bits(e.mem.u32(out + 0x28 * 3 + 0x24)), 7.0);
        assert_eq!(e.mem.u32(out + 0xa0 + 0x18), 0xffff_ffff);
        assert_eq!(vertex(&e, out + 0xa0, 3), vec![0.0, 1.0, 3.0]);
        assert_eq!(f32::from_bits(e.mem.u32(out + 0xa0 + 0x24)), 9.0);
        // With angles (0) the corners are the same.
        let angles = floats(&mut e, &[0.0, 0.0]);
        e.call(
            0x00e6_ab40,
            &args![r, out, positions, 0u32, sizes, scales, angles, bytes, 1u32],
        );
        assert_eq!(vertex(&e, out + 0x28, 3), vec![11.0, 19.0, 30.0]);
        assert_eq!(e.mem.u32(out + 0x18), 0xffff_ffff);
    }
}

/// The globals `Do_SetCameraData` reads and writes: two point copies (`011f426c` and
/// `011f474c`, three floats each) and a byte that, with the byte at `0126efd1`, selects which
/// of the location and the copy at `011f426c` becomes the view's eye.
const CAMERA_POINT_011F426C: u32 = 0x011f_426c;
const CAMERA_POINT_011F474C: u32 = 0x011f_474c;
const CAMERA_FLAG_011A9604: u32 = 0x011a_9604;
/// The `double` constants of the projection's first column and row (`01011590`, `0106d4c8`;
/// 2.0 and -2.0 by the decompiler).
const PROJECTION_SCALE_01011590: u32 = 0x0101_1590;
const PROJECTION_SCALE_0106D4C8: u32 = 0x0106_d4c8;
/// `D3DXVec4Transform` (`00ee6e34`, by its import), `D3DXVec3Normalize`,
/// `D3DXVec3TransformCoord`, `D3DXMatrixMultiply` (`stdcall`).
const D3DX_VEC4_TRANSFORM: u32 = 0x00ee_6e34;
const D3DX_VEC3_NORMALIZE: u32 = 0x00ee_6dec;
const D3DX_VEC3_TRANSFORM_COORD: u32 = 0x00ee_6df8;
const D3DX_MATRIX_MULTIPLY: u32 = 0x00ee_6de0;
/// `IDirect3DDevice9::SetTransform` (slot 44).
const DEVICE_SET_TRANSFORM: u32 = 0xb0;
/// The renderer's view (+0x980), projection (+0x9C0), view·projection (+0xA00) and inverse
/// view (+0xA40) matrices (`D3DMATRIX`, rows of four floats).
const VIEW_MATRIX: u32 = 0x980;
const PROJECTION_MATRIX: u32 = 0x9c0;
const VIEW_PROJECTION_MATRIX: u32 = 0xa00;
const INVERSE_VIEW_MATRIX: u32 = 0xa40;

/// `FISTP qword` under the code's chop control word, keeping the low word.
fn chop_low_word(value: f64) -> u32 {
    x87_fistp64_chop(X87Ext::load64(value)) as u32
}

// Translated from 00e6c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::Do_SetCameraData` (Xbox PDB; renderer vtable +0x18C): `thiscall`, six
/// words (location, direction, up, right as `NiPoint3`, an `NiFrustum`, an `NiRect<float>`
/// viewport). Nothing with the byte at +0x6FC set. The eye is the location, or with the
/// bytes at `0126efd1` and `011a9604` both set the point at `011f426c` (the other one goes
/// to `011f474c`). Builds the view matrix (+0x980: right, up, direction as columns, the
/// eye's negated dot products as the last row) and the inverse view (+0xA40), sets the view
/// transform (`SetTransform(2, ..)`), keeps right (+0x634, +0x64C) and up (+0x640, +0x658),
/// the near plane (+0x684) and the depth range (+0x688), and builds the projection
/// (+0x9C0) for an orthographic (`m_bOrtho`) or perspective frustum, its handedness by the
/// object at +0x8B8 (virtual +0x4C). With the byte at `0126efd1` set the projection's third
/// column becomes the clip plane through the point `(0, 0, [0126efd4])` facing the view's z
/// axis (`D3DXVec4Transform`, `D3DXVec3Normalize`, `D3DXVec3TransformCoord`). Then
/// `SetTransform(3, ..)`, the viewport from the render target group's size (+0x888, virtual
/// +0x8C/+0x90) and the rectangle (`SetViewport`, depths 0 to 1), the view·projection
/// (+0xA00, `D3DXMatrixMultiply`) and the near and far planes to +0x8B8's virtual +0x48.
#[allow(clippy::too_many_arguments)]
pub fn ni_xenon_renderer_do_set_camera_data(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    location: Ptr,
    direction: Ptr,
    up: Ptr,
    right: Ptr,
    frustum: Ptr,
    port: Ptr,
) {
    let t = this.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let (loc, dir, up, right, fr, port) = (
        location.addr(),
        direction.addr(),
        up.addr(),
        right.addr(),
        frustum.addr(),
        port.addr(),
    );
    let word = |e: &Engine, a: u32, i: u32| e.mem.u32(a.wrapping_add(4 * i));
    let eye: [u32; 3] =
        if e.mem.u8(ZEROED_FLAG_0126EFD1) != 0 && e.mem.u8(CAMERA_FLAG_011A9604) != 0 {
            for i in 0..3 {
                let v = word(e, CAMERA_POINT_011F426C, i);
                e.mem.set_u32(CAMERA_POINT_011F474C + 4 * i, v);
            }
            [word(e, loc, 0), word(e, loc, 1), word(e, loc, 2)]
        } else {
            for i in 0..3 {
                let v = word(e, loc, i);
                e.mem.set_u32(CAMERA_POINT_011F474C + 4 * i, v);
            }
            [
                word(e, CAMERA_POINT_011F426C, 0),
                word(e, CAMERA_POINT_011F426C, 1),
                word(e, CAMERA_POINT_011F426C, 2),
            ]
        };
    let copy = |e: &Engine, a: u32, i: u32| x87_quiet_bits(word(e, a, i));
    let view = t + VIEW_MATRIX;
    for i in 0..3 {
        let row = view + 0x10 * i;
        let values = [copy(e, right, i), copy(e, up, i), copy(e, dir, i), 0];
        for (k, v) in values.iter().enumerate() {
            e.mem.set_u32(row + 4 * k as u32, *v);
        }
    }
    let ld = |e: &Engine, a: u32, i: u32| x87_load32(word(e, a, i));
    let (ex, ey, ez) = (x87_load32(eye[0]), x87_load32(eye[1]), x87_load32(eye[2]));
    let along_right = x87_store32(x87_add(
        x87_add(x87_mul(ld(e, right, 0), ex), x87_mul(ld(e, right, 1), ey)),
        x87_mul(ld(e, right, 2), ez),
    ));
    e.mem
        .set_u32(view + 0x30, x87_store32(-x87_load32(along_right)));
    let along_up = x87_store32(x87_add(
        x87_add(x87_mul(ld(e, up, 0), ex), x87_mul(ld(e, up, 1), ey)),
        x87_mul(ld(e, up, 2), ez),
    ));
    e.mem
        .set_u32(view + 0x34, x87_store32(-x87_load32(along_up)));
    let along_dir = x87_store32(x87_add(
        x87_add(x87_mul(ld(e, dir, 1), ey), x87_mul(ld(e, dir, 0), ex)),
        x87_mul(ez, x87_m32(word(e, dir, 2))),
    ));
    e.mem
        .set_u32(view + 0x38, x87_store32(-x87_load32(along_dir)));
    const ONE_BITS: u32 = 0x3f80_0000;
    e.mem.set_u32(view + 0x3c, ONE_BITS);
    let inverse = t + INVERSE_VIEW_MATRIX;
    for (row, source) in [right, up, dir].iter().enumerate() {
        for i in 0..3 {
            let v = copy(e, *source, i);
            e.mem.set_u32(inverse + 0x10 * row as u32 + 4 * i, v);
        }
        e.mem.set_u32(inverse + 0x10 * row as u32 + 0xc, 0);
    }
    for (i, v) in eye.iter().enumerate() {
        e.mem
            .set_u32(inverse + 0x30 + 4 * i as u32, x87_quiet_bits(*v));
    }
    e.mem.set_u32(inverse + 0x3c, ONE_BITS);
    let device = e.get(this, NiDX9Renderer::device);
    e.vcall(device, DEVICE_SET_TRANSFORM, &args![2u32, view]);
    for i in 0..3 {
        let r = word(e, right, i);
        e.mem.set_u32(t + QUAD_RIGHT + 4 * i, r);
        let r = word(e, right, i);
        e.mem.set_u32(t + 0x634 + 4 * i, r);
    }
    for i in 0..3 {
        let u = word(e, up, i);
        e.mem.set_u32(t + QUAD_UP + 4 * i, u);
        let u = word(e, up, i);
        e.mem.set_u32(t + 0x640 + 4 * i, u);
    }
    // NiFrustum: left, right, top, bottom, near, far, then the byte m_bOrtho (Xbox PDB).
    e.mem.set_u32(t + 0x684, copy(e, fr, 4));
    let object = e.mem.u32(t + 0x8b8);
    let range = x87_store32(x87_sub(ld(e, fr, 5), x87_m32(word(e, fr, 4))));
    e.mem.set_u32(t + 0x688, range);
    let ortho = e.mem.u8(fr + 0x18) != 0;
    let width = x87_store32(x87_sub(ld(e, fr, 1), x87_m32(word(e, fr, 0))));
    let width_sum = x87_store32(x87_add(ld(e, fr, 0), x87_m32(word(e, fr, 1))));
    let height = x87_store32(x87_sub(ld(e, fr, 2), x87_m32(word(e, fr, 3))));
    let height_sum = x87_store32(x87_add(ld(e, fr, 3), x87_m32(word(e, fr, 2))));
    let inverse_range = x87_store32(x87_div(1.0, x87_load32(range)));
    let flipped = e.vcall(object, 0x4c, &args![]).u8() != 0;
    let scale = x87_load64(e.mem.u64(PROJECTION_SCALE_01011590));
    let w = x87_load32(width);
    let proj = t + PROJECTION_MATRIX;
    let first = if flipped {
        x87_div(f64::from_bits(e.mem.u64(PROJECTION_SCALE_0106D4C8)), w)
    } else {
        x87_div(scale, w)
    };
    e.mem.set_u32(proj, x87_store32(first));
    e.mem.set_u32(proj + 0x10, 0);
    if ortho {
        e.mem.set_u32(proj + 0x20, 0);
    }
    let sum = if flipped {
        x87_load32(width_sum)
    } else {
        -x87_load32(width_sum)
    };
    let x_offset = x87_store32(x87_div(sum, w));
    if ortho {
        e.mem.set_u32(proj + 0x30, x_offset);
    } else {
        e.mem.set_u32(proj + 0x20, x_offset);
        e.mem.set_u32(proj + 0x30, 0);
    }
    e.mem.set_u32(proj + 4, 0);
    let h = x87_load32(height);
    e.mem.set_u32(proj + 0x14, x87_store32(x87_div(scale, h)));
    if ortho {
        e.mem.set_u32(proj + 0x24, 0);
    }
    let y_offset = x87_store32(x87_div(-x87_load32(height_sum), h));
    if ortho {
        e.mem.set_u32(proj + 0x34, y_offset);
    } else {
        e.mem.set_u32(proj + 0x24, y_offset);
        e.mem.set_u32(proj + 0x34, 0);
    }
    e.mem.set_u32(proj + 8, 0);
    e.mem.set_u32(proj + 0x18, 0);
    let inv = x87_load32(inverse_range);
    if ortho {
        e.mem.set_u32(proj + 0x28, inverse_range);
        let near = x87_mul(inv, x87_m32(word(e, fr, 4)));
        e.mem.set_u32(proj + 0x38, x87_store32(-near));
        e.mem.set_u32(proj + 0xc, 0);
        e.mem.set_u32(proj + 0x1c, 0);
        e.mem.set_u32(proj + 0x2c, 0);
        e.mem.set_u32(proj + 0x3c, ONE_BITS);
    } else {
        let far = x87_mul(inv, x87_m32(word(e, fr, 5)));
        e.mem.set_u32(proj + 0x28, x87_store32(far));
        let near_far = x87_mul(ld(e, fr, 4), x87_m32(word(e, fr, 5)));
        e.mem
            .set_u32(proj + 0x38, x87_store32(-x87_mul(inv, near_far)));
        e.mem.set_u32(proj + 0xc, 0);
        e.mem.set_u32(proj + 0x1c, 0);
        e.mem.set_u32(proj + 0x2c, ONE_BITS);
        e.mem.set_u32(proj + 0x3c, 0);
    }
    e.with_stack(0x80, |e, frame| {
        let l = frame.addr();
        if e.mem.u8(ZEROED_FLAG_0126EFD1) != 0 {
            // The view's z axis (0, 0, 1, 0) and the point (0, 0, [0126efd4]).
            for (offset, value) in [
                (0x58, 0),
                (0x5c, 0),
                (0x64, 0),
                (0x28, 0),
                (0x2c, 0),
                (0x60, ONE_BITS),
            ] {
                e.mem.set_u32(l + offset, value);
            }
            let height = x87_quiet_bits(e.mem.u32(ZEROED_VALUE_0126EFD4));
            e.mem.set_u32(l + 0x30, height);
            e.call(D3DX_VEC4_TRANSFORM, &args![l + 0x48, l + 0x58, view]);
            e.call(D3DX_VEC3_NORMALIZE, &args![l + 0x48, l + 0x48]);
            e.call(D3DX_VEC3_TRANSFORM_COORD, &args![l + 0x38, l + 0x28, view]);
            let n = |e: &Engine, i: u32| x87_load32(e.mem.u32(l + 0x48 + 4 * i));
            let p = |e: &Engine, i: u32| x87_load32(e.mem.u32(l + 0x38 + 4 * i));
            let (nx, ny, nz) = (n(e, 0), n(e, 1), n(e, 2));
            let distance = x87_store32(x87_add(
                x87_add(x87_mul(p(e, 1), ny), x87_mul(p(e, 0), nx)),
                x87_mul(p(e, 2), nz),
            ));
            let plane_w = x87_store32(-x87_load32(distance));
            e.mem.set_u32(l + 0x64, plane_w);
            let negative = x87_quiet_bits(e.mem.u32(FRUSTUM_SIDE_01012054));
            let sign = |v: f64| {
                if 0.0 < v {
                    ONE_BITS
                } else if v < 0.0 {
                    negative
                } else {
                    0
                }
            };
            let sign_x = sign(nx);
            e.mem.set_u32(l + 0x1c, sign_x);
            let qx = x87_store32(x87_div(x87_load32(sign_x), x87_m32(e.mem.u32(proj))));
            e.mem.set_u32(l + 0x48, qx);
            let sign_y = sign(ny);
            e.mem.set_u32(l + 0x1c, sign_y);
            let qy = x87_store32(x87_div(x87_load32(sign_y), x87_m32(e.mem.u32(proj + 0x14))));
            e.mem.set_u32(l + 0x4c, qy);
            let qw = x87_store32(x87_div(
                x87_sub(1.0, x87_load32(e.mem.u32(proj + 0x28))),
                x87_load32(e.mem.u32(proj + 0x38)),
            ));
            e.mem.set_u32(l + 0x54, qw);
            let cw = x87_load32(plane_w);
            let dot = x87_store32(x87_add(
                x87_add(
                    x87_add(x87_mul(x87_load32(qy), ny), x87_mul(x87_load32(qx), nx)),
                    nz,
                ),
                x87_mul(x87_load32(qw), cw),
            ));
            let factor = x87_load32(x87_store32(x87_div(1.0, x87_load32(dot))));
            let column = [
                x87_store32(x87_mul(nx, factor)),
                x87_store32(x87_mul(ny, factor)),
                x87_store32(x87_mul(nz, factor)),
                x87_store32(x87_mul(factor, cw)),
            ];
            for (i, v) in column.iter().enumerate() {
                e.mem.set_u32(l + 0x38 + 4 * i as u32, *v);
            }
            for (i, v) in column.iter().enumerate() {
                e.mem.set_u32(proj + 8 + 0x10 * i as u32, *v);
            }
        }
        let device = e.get(this, NiDX9Renderer::device);
        e.vcall(device, DEVICE_SET_TRANSFORM, &args![3u32, proj]);
        let group = e.get(this, NiDX9Renderer::target_group);
        let target_width = e.vcall(group, 0x8c, &args![0u32]).u32();
        let wf = x87_load32(unsigned_to_float(e, target_width));
        let group = e.get(this, NiDX9Renderer::target_group);
        let target_height = e.vcall(group, 0x90, &args![0u32]).u32();
        let hf = x87_load32(unsigned_to_float(e, target_height));
        // NiRect<float>: left, right, top, bottom.
        let x = chop_low_word(x87_mul(wf, x87_m32(word(e, port, 0))));
        e.mem.set_u32(l + 0x68, x);
        let y = chop_low_word(x87_mul(x87_sub(1.0, ld(e, port, 2)), hf));
        e.mem.set_u32(l + 0x6c, y);
        let span = x87_sub(ld(e, port, 1), x87_m32(word(e, port, 0)));
        e.mem.set_u32(l + 0x70, chop_low_word(x87_mul(wf, span)));
        let span = x87_sub(ld(e, port, 2), x87_m32(word(e, port, 3)));
        e.mem.set_u32(l + 0x74, chop_low_word(x87_mul(hf, span)));
        e.mem.set_u32(l + 0x78, 0);
        e.mem.set_u32(l + 0x7c, ONE_BITS);
        let device = e.get(this, NiDX9Renderer::device);
        e.vcall(device, DEVICE_SET_VIEWPORT, &args![l + 0x68]);
    });
    e.call(
        D3DX_MATRIX_MULTIPLY,
        &args![t + VIEW_PROJECTION_MATRIX, view, proj],
    );
    let object = e.mem.u32(t + 0x8b8);
    let far = copy(e, fr, 5);
    let near = copy(e, fr, 4);
    e.vcall(object, 0x48, &args![near, far]);
}

#[cfg(test)]
mod a07_camera_tests {
    use super::*;

    const DEVICE_BASE: u32 = 0x6100_0000;
    const GROUP_BASE: u32 = 0x6140_0000;
    const OBJECT_BASE: u32 = 0x6180_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(0x10);
        e.mem.set_u32(o, table);
        o
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let p = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(p + 4 * i as u32, v.to_bits());
        }
        p
    }

    fn f(e: &Engine, a: u32) -> f32 {
        f32::from_bits(e.mem.u32(a))
    }

    /// A renderer with a device, a 800×600 target group and the object at +0x8B8 (its
    /// virtual +0x4C answers `flipped`), the globals mapped, and the inputs.
    fn set_up(e: &mut Engine, flipped: bool) -> (Ptr<NiDX9Renderer>, u32, [u32; 4]) {
        e.map(0x0126_e000, 0x2000);
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(0x0101_1000, 0x2000);
        e.map(0x0106_d000, 0x1000);
        e.map(TWO_TO_32_01099378 & !0xfff, 0x1000);
        e.mem.set_u64(PROJECTION_SCALE_01011590, 2.0f64.to_bits());
        e.mem
            .set_u64(PROJECTION_SCALE_0106D4C8, (-2.0f64).to_bits());
        e.mem.set_u32(FRUSTUM_SIDE_01012054, (-1.0f32).to_bits());
        for (i, v) in [4.0f32, 5.0, 6.0].iter().enumerate() {
            e.mem
                .set_u32(CAMERA_POINT_011F426C + 4 * i as u32, v.to_bits());
        }
        let r = e.new_object::<NiDX9Renderer>();
        let device = object(e, 0x6000, DEVICE_BASE);
        e.set(r, NiDX9Renderer::device, device);
        let group = object(e, 0x6400, GROUP_BASE);
        e.set(r, NiDX9Renderer::target_group, group);
        e.register(GROUP_BASE + 0x8c, |_, _| 800u32.into_ret());
        e.register(GROUP_BASE + 0x90, |_, _| 600u32.into_ret());
        let other = object(e, 0x6800, OBJECT_BASE);
        e.mem.set_u32(r.addr() + 0x8b8, other);
        if flipped {
            e.register(OBJECT_BASE + 0x4c, |_, _| 1u32.into_ret());
        } else {
            e.register(OBJECT_BASE + 0x4c, |_, _| 0u32.into_ret());
        }
        e.register(OBJECT_BASE + 0x48, nop);
        e.register(DEVICE_BASE + DEVICE_SET_TRANSFORM, nop);
        e.register(DEVICE_BASE + DEVICE_SET_VIEWPORT, nop);
        e.register(D3DX_MATRIX_MULTIPLY, nop);
        let location = floats(e, &[1.0, 2.0, 3.0]);
        let direction = floats(e, &[0.0, 0.0, 1.0]);
        let up = floats(e, &[0.0, 1.0, 0.0]);
        let right = floats(e, &[1.0, 0.0, 0.0]);
        (r, device, [location, direction, up, right])
    }

    #[test]
    fn perspective_camera_builds_the_view_projection_and_viewport() {
        let mut e = Engine::new();
        let (r, device, [location, direction, up, right]) = set_up(&mut e, false);
        let t = r.addr();
        let frustum = floats(&mut e, &[-1.0, 1.0, 1.0, -1.0, 1.0, 5.0, 0.0]);
        let port = floats(&mut e, &[0.0, 1.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(
            0x00e6_c780,
            &args![r, location, direction, up, right, frustum, port],
        );
        // The flags are clear: the eye is the point at 011f426c, the location is kept.
        assert_eq!(f(&e, CAMERA_POINT_011F474C + 8), 3.0);
        let view = t + VIEW_MATRIX;
        assert_eq!(
            (0..16).map(|i| f(&e, view + 4 * i)).collect::<Vec<_>>(),
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -4.0, -5.0, -6.0, 1.0]
        );
        assert_eq!(f(&e, t + INVERSE_VIEW_MATRIX + 0x34), 5.0);
        assert_eq!(f(&e, t + INVERSE_VIEW_MATRIX + 0x3c), 1.0);
        assert_eq!(f(&e, t + QUAD_RIGHT), 1.0);
        assert_eq!(f(&e, t + QUAD_UP + 4), 1.0);
        assert_eq!(f(&e, t + 0x640 + 4), 1.0);
        assert_eq!(f(&e, t + 0x684), 1.0);
        assert_eq!(f(&e, t + 0x688), 4.0);
        let proj = t + PROJECTION_MATRIX;
        assert_eq!(f(&e, proj), 1.0);
        assert_eq!(e.mem.u32(proj + 0x20), 0x8000_0000);
        assert_eq!(f(&e, proj + 0x14), 1.0);
        assert_eq!(e.mem.u32(proj + 0x24), 0x8000_0000);
        assert_eq!(f(&e, proj + 0x28), 1.25);
        assert_eq!(f(&e, proj + 0x38), -1.25);
        assert_eq!(f(&e, proj + 0x2c), 1.0);
        assert_eq!(f(&e, proj + 0x3c), 0.0);
        assert_eq!(
            calls(&e, DEVICE_BASE + DEVICE_SET_TRANSFORM),
            vec![vec![device, 2, view], vec![device, 3, proj]]
        );
        let viewport = calls(&e, DEVICE_BASE + DEVICE_SET_VIEWPORT);
        assert_eq!(viewport.len(), 1);
        // Its values: viewport_is_the_rectangle_of_the_target.
        assert_eq!(
            calls(&e, D3DX_MATRIX_MULTIPLY),
            vec![vec![t + VIEW_PROJECTION_MATRIX, view, proj]]
        );
        let other = e.mem.u32(t + 0x8b8);
        assert_eq!(
            calls(&e, OBJECT_BASE + 0x48),
            vec![vec![other, 1.0f32.to_bits(), 5.0f32.to_bits()]]
        );
        assert!(calls(&e, D3DX_VEC4_TRANSFORM).is_empty());
    }

    #[test]
    fn viewport_is_the_rectangle_of_the_target() {
        let mut e = Engine::new();
        let (r, _, [location, direction, up, right]) = set_up(&mut e, false);
        let frustum = floats(&mut e, &[-1.0, 1.0, 1.0, -1.0, 1.0, 5.0, 0.0]);
        let port = floats(&mut e, &[0.25, 0.75, 0.5, 0.0]);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(DEVICE_BASE + DEVICE_SET_VIEWPORT, move |e, a| {
            log.borrow_mut()
                .extend((0..6).map(|i| e.mem.u32(a[1] + 4 * i)));
            Ret::default()
        });
        e.call(
            0x00e6_c780,
            &args![r, location, direction, up, right, frustum, port],
        );
        assert_eq!(
            *seen.borrow(),
            vec![200, 300, 400, 300, 0, 1.0f32.to_bits()]
        );
    }

    #[test]
    fn oblique_orthographic_camera_replaces_the_third_column() {
        let mut e = Engine::new();
        let (r, _, [location, direction, up, right]) = set_up(&mut e, true);
        let t = r.addr();
        e.mem.set_u8(ZEROED_FLAG_0126EFD1, 1);
        e.mem.set_u8(CAMERA_FLAG_011A9604, 1);
        e.mem.set_u32(ZEROED_VALUE_0126EFD4, 7.0f32.to_bits());
        let frustum = floats(&mut e, &[-1.0, 3.0, 1.0, -1.0, 1.0, 5.0, 0.0]);
        e.mem.set_u8(frustum + 0x18, 1);
        let port = floats(&mut e, &[0.0, 1.0, 1.0, 0.0]);
        // The view's z axis transforms to (0, 0, 1); the point to (0, 0, 3).
        e.register(D3DX_VEC4_TRANSFORM, |e, a| {
            assert_eq!(e.mem.u32(a[1] + 8), 1.0f32.to_bits());
            for (i, v) in [0.0f32, 0.0, 1.0, 0.0].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, v.to_bits());
            }
            a[0].into_ret()
        });
        e.register(D3DX_VEC3_NORMALIZE, |_, a| a[0].into_ret());
        e.register(D3DX_VEC3_TRANSFORM_COORD, |e, a| {
            assert_eq!(e.mem.u32(a[1] + 8), 7.0f32.to_bits());
            for (i, v) in [0.0f32, 0.0, 3.0].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, v.to_bits());
            }
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x00e6_c780,
            &args![r, location, direction, up, right, frustum, port],
        );
        // Both flags set: the eye is the location, the point at 011f426c is copied.
        assert_eq!(f(&e, CAMERA_POINT_011F474C), 4.0);
        assert_eq!(f(&e, t + VIEW_MATRIX + 0x30), -1.0);
        let proj = t + PROJECTION_MATRIX;
        // Flipped orthographic: -2 / 4, (left + right) / 4, 2 / 2, -(top + bottom) / 2.
        assert_eq!(f(&e, proj), -0.5);
        assert_eq!(f(&e, proj + 0x30), 0.5);
        assert_eq!(f(&e, proj + 0x14), 1.0);
        assert_eq!(e.mem.u32(proj + 0x34), 0x8000_0000);
        assert_eq!(f(&e, proj + 0x3c), 1.0);
        assert_eq!(f(&e, proj + 0x2c), 0.0);
        // The plane (0, 0, 1, -3); q.w = (1 - 0.25) / -0.25 = -3; dot = 1 + 9.
        let factor = x87_store32(x87_div(1.0, 10.0));
        let ff = f64::from(f32::from_bits(factor));
        assert_eq!(e.mem.u32(proj + 8), 0);
        assert_eq!(e.mem.u32(proj + 0x18), 0);
        assert_eq!(e.mem.u32(proj + 0x28), factor);
        assert_eq!(e.mem.u32(proj + 0x38), x87_store32(ff * -3.0));
        assert_eq!(calls(&e, D3DX_VEC3_NORMALIZE).len(), 1);
    }

    #[test]
    fn nothing_while_the_scene_is_skipped() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        e.mem.set_u8(r.addr() + SKIP_END_SCENE, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e6_c780, &args![r, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

/// The `float` constants of the screen-space camera (`0102295c` -0.5, `01016248` 0.5,
/// `010ed968` 9999.0 for the depth range, `010162c0` 2.0, `010ed964` and `010ed960` about
/// ±1.0e-4 for the depth column, `01022958` 10000.0 for the far plane) and the `double`
/// `0101a6b0` (-1.0) the x offset is divided from (values as the exe holds them).
const SCREEN_VIEW_X_0102295C: u32 = 0x0102_295c;
const SCREEN_VIEW_Y_01016248: u32 = 0x0101_6248;
const SCREEN_RANGE_010ED968: u32 = 0x010e_d968;
const SCREEN_SCALE_010162C0: u32 = 0x0101_62c0;
const SCREEN_OFFSET_0101A6B0: u32 = 0x0101_a6b0;
const SCREEN_DEPTH_010ED964: u32 = 0x010e_d964;
const SCREEN_DEPTH_OFFSET_010ED960: u32 = 0x010e_d960;
const SCREEN_FAR_01022958: u32 = 0x0102_2958;

// Translated from 00e6cf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x190, `Do_SetScreenSpaceCameraData`'s slot
/// on Xbox): `thiscall`, one word (an `NiRect<float>` viewport or 0). Nothing with the byte
/// at +0x6FC set. Takes the render target group's size (+0x888, virtuals +0x8C, +0x90 with
/// 0) as floats, sets a fixed view matrix (+0x980: x kept, y and z negated by the `float`
/// -1.0 at `01012054`, translated by (-0.5, 0.5, 1)) and its inverse (+0xA40), the right
/// (1, 0, 0) and up (0, -1, 0) vectors (+0x64C/+0x634, +0x658/+0x640), the near plane 1.0
/// (+0x684) and depth range (+0x688), and a projection (+0x9C0) with the target's size
/// (`-1.0 / width` and `1 / height` as the offsets). The viewport is the rectangle of the
/// target (truncated as `00e6c780` does), or without one the whole target; depths 0 to 1.
/// Ends with near 1.0 and far 10000.0 (`01022958`) to +0x8B8's virtual +0x48.
pub fn fn_00e6cf40(e: &mut Engine, this: Ptr<NiDX9Renderer>, port: Ptr) {
    let t = this.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let group = e.get(this, NiDX9Renderer::target_group);
    let target_width = e.vcall(group, 0x8c, &args![0u32]).u32();
    let group = e.get(this, NiDX9Renderer::target_group);
    let target_height = e.vcall(group, 0x90, &args![0u32]).u32();
    let wf = unsigned_to_float(e, target_width);
    let hf = unsigned_to_float(e, target_height);
    const ONE_BITS: u32 = 0x3f80_0000;
    let constant = |e: &Engine, a: u32| x87_quiet_bits(e.mem.u32(a));
    let minus_one = constant(e, FRUSTUM_SIDE_01012054);
    let half_x = constant(e, SCREEN_VIEW_X_0102295C);
    let half_y = constant(e, SCREEN_VIEW_Y_01016248);
    let view = t + VIEW_MATRIX;
    let view_values = [
        ONE_BITS, 0, 0, 0, 0, minus_one, 0, 0, 0, 0, ONE_BITS, 0, half_x, half_y, ONE_BITS,
        ONE_BITS,
    ];
    for (i, v) in view_values.iter().enumerate() {
        e.mem.set_u32(view + 4 * i as u32, *v);
    }
    let inverse = t + INVERSE_VIEW_MATRIX;
    let inverse_values = [
        ONE_BITS, 0, 0, 0, 0, minus_one, 0, 0, 0, 0, ONE_BITS, 0, half_y, half_y, minus_one,
        ONE_BITS,
    ];
    for (i, v) in inverse_values.iter().enumerate() {
        e.mem.set_u32(inverse + 4 * i as u32, *v);
    }
    let device = e.get(this, NiDX9Renderer::device);
    e.vcall(device, DEVICE_SET_TRANSFORM, &args![2u32, view]);
    for (i, v) in [ONE_BITS, 0, 0].iter().enumerate() {
        e.mem.set_u32(t + QUAD_RIGHT + 4 * i as u32, *v);
        e.mem.set_u32(t + 0x634 + 4 * i as u32, *v);
    }
    for (i, v) in [0, minus_one, 0].iter().enumerate() {
        e.mem.set_u32(t + QUAD_UP + 4 * i as u32, *v);
        e.mem.set_u32(t + 0x640 + 4 * i as u32, *v);
    }
    e.mem.set_u32(t + 0x684, ONE_BITS);
    let range = constant(e, SCREEN_RANGE_010ED968);
    e.mem.set_u32(t + 0x688, range);
    let proj = t + PROJECTION_MATRIX;
    let scale = constant(e, SCREEN_SCALE_010162C0);
    let x_offset = x87_div(
        f64::from_bits(e.mem.u64(SCREEN_OFFSET_0101A6B0)),
        x87_load32(wf),
    );
    let y_offset = x87_div(1.0, x87_load32(hf));
    let depth = constant(e, SCREEN_DEPTH_010ED964);
    let depth_offset = constant(e, SCREEN_DEPTH_OFFSET_010ED960);
    let proj_values = [
        scale,
        0,
        0,
        0,
        0,
        scale,
        0,
        0,
        0,
        0,
        depth,
        0,
        x87_store32(x_offset),
        x87_store32(y_offset),
        depth_offset,
        ONE_BITS,
    ];
    for (i, v) in proj_values.iter().enumerate() {
        e.mem.set_u32(proj + 4 * i as u32, *v);
    }
    let device = e.get(this, NiDX9Renderer::device);
    e.vcall(device, DEVICE_SET_TRANSFORM, &args![3u32, proj]);
    let p = port.addr();
    e.with_stack(0x18, |e, viewport| {
        let v = viewport.addr();
        let rectangle = if p != 0 {
            let word = |e: &Engine, i: u32| e.mem.u32(p + 4 * i);
            let w = x87_load32(wf);
            let h = x87_load32(hf);
            let x = chop_low_word(x87_mul(w, x87_load32(word(e, 0))));
            let y = chop_low_word(x87_mul(x87_sub(1.0, x87_load32(word(e, 2))), h));
            let span = x87_sub(x87_load32(word(e, 1)), x87_m32(word(e, 0)));
            let width = chop_low_word(x87_mul(w, span));
            let span = x87_sub(x87_load32(word(e, 2)), x87_m32(word(e, 3)));
            let height = chop_low_word(x87_mul(h, span));
            [x, y, width, height]
        } else {
            [0, 0, target_width, target_height]
        };
        for (i, value) in rectangle.iter().enumerate() {
            e.mem.set_u32(v + 4 * i as u32, *value);
        }
        e.mem.set_u32(v + 0x10, 0);
        e.mem.set_u32(v + 0x14, ONE_BITS);
        let device = e.get(this, NiDX9Renderer::device);
        e.vcall(device, DEVICE_SET_VIEWPORT, &args![v]);
    });
    let far = constant(e, SCREEN_FAR_01022958);
    let object = e.mem.u32(t + 0x8b8);
    e.vcall(object, 0x48, &args![ONE_BITS, far]);
}

#[cfg(test)]
mod a07_screen_camera_tests {
    use super::*;

    const DEVICE_BASE: u32 = 0x6100_0000;
    const GROUP_BASE: u32 = 0x6140_0000;
    const OBJECT_BASE: u32 = 0x6180_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(0x10);
        e.mem.set_u32(o, table);
        o
    }

    type Seen = std::rc::Rc<std::cell::RefCell<Vec<u32>>>;

    fn set_up(e: &mut Engine) -> (Ptr<NiDX9Renderer>, u32, Seen) {
        e.map(0x0101_0000, 0x0002_0000);
        e.map(0x010e_d000, 0x1000);
        e.map(TWO_TO_32_01099378 & !0xfff, 0x1000);
        for (a, v) in [
            (FRUSTUM_SIDE_01012054, -1.0f32),
            (SCREEN_VIEW_X_0102295C, -0.5),
            (SCREEN_VIEW_Y_01016248, 0.5),
            (SCREEN_RANGE_010ED968, 9999.0),
            (SCREEN_SCALE_010162C0, 2.0),
            (SCREEN_DEPTH_010ED964, 1.0e-4),
            (SCREEN_DEPTH_OFFSET_010ED960, -1.0e-4),
            (SCREEN_FAR_01022958, 10000.0),
        ] {
            e.mem.set_u32(a, v.to_bits());
        }
        e.mem.set_u64(SCREEN_OFFSET_0101A6B0, (-1.0f64).to_bits());
        let r = e.new_object::<NiDX9Renderer>();
        let device = object(e, 0x6000, DEVICE_BASE);
        e.set(r, NiDX9Renderer::device, device);
        let group = object(e, 0x6400, GROUP_BASE);
        e.set(r, NiDX9Renderer::target_group, group);
        e.register(GROUP_BASE + 0x8c, |_, _| 800u32.into_ret());
        e.register(GROUP_BASE + 0x90, |_, _| 400u32.into_ret());
        let other = object(e, 0x6800, OBJECT_BASE);
        e.mem.set_u32(r.addr() + 0x8b8, other);
        e.register(OBJECT_BASE + 0x48, nop);
        e.register(DEVICE_BASE + DEVICE_SET_TRANSFORM, nop);
        let seen: Seen = Default::default();
        let log = seen.clone();
        e.register_double(DEVICE_BASE + DEVICE_SET_VIEWPORT, move |e, a| {
            log.borrow_mut()
                .extend((0..6).map(|i| e.mem.u32(a[1] + 4 * i)));
            Ret::default()
        });
        (r, device, seen)
    }

    fn f(e: &Engine, a: u32) -> f32 {
        f32::from_bits(e.mem.u32(a))
    }

    #[test]
    fn screen_camera_sets_the_fixed_view_and_the_target_projection() {
        let mut e = Engine::new();
        let (r, device, seen) = set_up(&mut e);
        let t = r.addr();
        e.call_log = Some(vec![]);
        e.call(0x00e6_cf40, &args![r, 0u32]);
        let view = t + VIEW_MATRIX;
        assert_eq!(
            (0..16).map(|i| f(&e, view + 4 * i)).collect::<Vec<_>>(),
            vec![1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.5, 0.5, 1.0, 1.0]
        );
        let inverse = t + INVERSE_VIEW_MATRIX;
        assert_eq!(
            (12..16).map(|i| f(&e, inverse + 4 * i)).collect::<Vec<_>>(),
            vec![0.5, 0.5, -1.0, 1.0]
        );
        assert_eq!(f(&e, t + QUAD_UP + 4), -1.0);
        assert_eq!(f(&e, t + 0x634), 1.0);
        assert_eq!(f(&e, t + 0x688), 9999.0);
        let proj = t + PROJECTION_MATRIX;
        assert_eq!(f(&e, proj), 2.0);
        assert_eq!(f(&e, proj + 0x14), 2.0);
        assert_eq!(f(&e, proj + 0x28), 1.0e-4);
        assert_eq!(f(&e, proj + 0x30), -1.0 / 800.0);
        assert_eq!(f(&e, proj + 0x34), 1.0 / 400.0);
        assert_eq!(f(&e, proj + 0x38), -1.0e-4);
        assert_eq!(f(&e, proj + 0x3c), 1.0);
        assert_eq!(
            calls(&e, DEVICE_BASE + DEVICE_SET_TRANSFORM),
            vec![vec![device, 2, view], vec![device, 3, proj]]
        );
        // Without a rectangle the viewport is the whole target.
        assert_eq!(*seen.borrow(), vec![0, 0, 800, 400, 0, 1.0f32.to_bits()]);
        let other = e.mem.u32(t + 0x8b8);
        assert_eq!(
            calls(&e, OBJECT_BASE + 0x48),
            vec![vec![other, 1.0f32.to_bits(), 10000.0f32.to_bits()]]
        );
    }

    #[test]
    fn screen_camera_viewport_takes_the_rectangle() {
        let mut e = Engine::new();
        let (r, _, seen) = set_up(&mut e);
        let port = e.mem.alloc(0x10);
        for (i, v) in [0.25f32, 0.75, 0.5, 0.0].iter().enumerate() {
            e.mem.set_u32(port + 4 * i as u32, v.to_bits());
        }
        e.call(0x00e6_cf40, &args![r, port]);
        assert_eq!(
            *seen.borrow(),
            vec![200, 200, 400, 200, 0, 1.0f32.to_bits()]
        );
        // Skipped scene: nothing.
        e.mem.set_u8(r.addr() + SKIP_END_SCENE, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e6_cf40, &args![r, port]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

/// The two RTTI records `PackGeometryBuffer` tells the data's kind by: `011f4a88` (walked
/// up the data's own RTTI chain; then the triangle list `00a7b850`, `GetTriList`) and
/// `011f4a70` (`IsKindOf`; then the strip lists `00a75f10`, `GetStripLists`).
const TRI_LIST_DATA_RTTI_011F4A88: u32 = 0x011f_4a88;
const STRIP_DATA_RTTI_011F4A70: u32 = 0x011f_4a70;
/// `NiD3DHLSLVertexShader::GetEntryPoint` (`00e8bf90`, engine map; `thiscall` on the object
/// at +0x8BC), `NiXenonVertexBufferManager::PackUntransformedVB` (`00e8c1e0`, engine map;
/// `thiscall` on it, five words), `00e8f200` (`thiscall` on the object at +0x8C0, seven
/// words: makes or refills an index buffer) and `NiGeometryBufferData::RemoveIB`
/// (`00e8eee0`, engine map; `thiscall`).
const GET_ENTRY_POINT_00E8BF90: u32 = 0x00e8_bf90;
const PACK_UNTRANSFORMED_VB_00E8C1E0: u32 = 0x00e8_c1e0;
const INDEX_BUFFER_00E8F200: u32 = 0x00e8_f200;
const REMOVE_IB_00E8EEE0: u32 = 0x00e8_eee0;

// Translated from 00e71fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PackGeometryBuffer` (Xbox PDB): `thiscall`, five words (the geometry
/// buffer data, the geometry data, an object with a word at +0x18 or 0, a stream source or
/// 0, a force byte); answers whether it packed. False without buffer data or data; true at
/// once when not forced and the buffer data is already packed (`00e8eeb0`) with the data's
/// word +0xE at `0x4000` in its bits `0x7000`. The buffer's word +4 gets the flags
/// `00e6da60` builds. The vertex count (data virtual +0x90) and the word +8 are kept, and for
/// triangle data (RTTI `011f4a88` in the data's RTTI chain) the triangle count (virtual
/// +0xA8), the index count `3·[+0x40]` and the list (`00a7b850`), for strip data
/// (`IsKindOf` `011f4a70`) the strip count +0x44, the lengths +0x48 and the index count
/// `[+0x40] + 2·strips` (`00a75f10`). The streams (the source's word +0x20, else 1) are
/// reset (each unpacked one through the buffer's +8 object, virtual +0x1C, then `00e8ef10`)
/// unless the buffer is packed with that count, which keeps the data's dirty bits (+0xE,
/// low 12 bits, with 3 when the object's +0x18 is not the entry point of +0x8BC's
/// `00e8bf90`, which then replaces it); otherwise and when forced all bits count. With any
/// bit but `0x10`, each stream is packed by the source's virtual +0xAC, or for a single
/// stream by `PackUntransformedVB`; a failure answers false. With indices and the bits
/// `0x30` the index buffer is made (`00e8f200`, false on failure) and a new one replaces the
/// old (`RemoveIB`, +0x34, +0x2C, +0x30). Then the data's dirty bits are cleared (+0xE keeps
/// `0xF000`).
pub fn ni_xenon_renderer_pack_geometry_buffer(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    buffer: Ptr,
    data: Ptr,
    object: u32,
    source: u32,
    force: u8,
) -> bool {
    let b = buffer.addr();
    let d = data.addr();
    if b == 0 || d == 0 {
        return false;
    }
    let packed = e.call(DATA_IS_PACKED_00E8EEB0, &args![buffer]).u8() != 0;
    if force == 0 && packed && e.mem.u16(d + 0xe) & 0x7000 == 0x4000 {
        return true;
    }
    let has_28 = e.mem.u32(d + 0x28) != 0;
    let has_24 = e.mem.u32(d + 0x24) != 0;
    let top_bits = has_24 && e.mem.u16(d + 0xc) & 0xf000 != 0;
    let mut low = u32::from(e.mem.u16(d + 0xc)) & 1;
    let limit = e.global::<u32>(STREAM_LIMIT_011BF558);
    if low > limit {
        low = limit;
    }
    let mut stream_flags = 0u32;
    if has_28 {
        stream_flags = 0x40_0000;
    }
    if has_24 {
        stream_flags |= 0x80_0000;
    }
    if top_bits {
        stream_flags |= 0x20_0000;
    }
    e.mem.set_u32(b + 4, (low << 24) | stream_flags);
    let vertices = u32::from(e.vcall(d, 0x90, &args![]).u16());
    let second = u32::from(e.mem.u16(d + 8));
    let mut triangles = 0u32;
    let mut base_count = 0u32;
    let mut index_count = 0u32;
    let mut list = 0u32;
    let mut lengths = 0u32;
    let mut strips = 0u32;
    let mut rtti = e.vcall(d, 8, &args![]).u32();
    let mut tri_list = false;
    while rtti != 0 {
        if rtti == TRI_LIST_DATA_RTTI_011F4A88 {
            tri_list = true;
            break;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    if tri_list {
        triangles = u32::from(e.vcall(d, 0xa8, &args![]).u16());
        base_count = u32::from(e.mem.u16(d + 0x40));
        list = e.call(TRI_SHAPE_DATA_GET_TRI_LIST, &args![data]).u32();
        lengths = 0;
        strips = 1;
        index_count = base_count * 3;
    } else if e
        .call(IS_KIND_OF, &args![STRIP_DATA_RTTI_011F4A70, data])
        .u8()
        != 0
    {
        triangles = u32::from(e.vcall(d, 0xa8, &args![]).u16());
        base_count = u32::from(e.mem.u16(d + 0x40));
        list = e.call(TRI_STRIPS_DATA_GET_STRIP_LISTS, &args![data]).u32();
        lengths = e.mem.u32(d + 0x48);
        strips = u32::from(e.mem.u16(d + 0x44));
        index_count = base_count + strips * 2;
    }
    let count = if source != 0 {
        e.mem.u32(source.wrapping_add(0x20))
    } else {
        1
    };
    let mut dirty = u32::from(e.mem.u16(d + 0xe) & 0xfff);
    if object != 0 {
        let previous = e.mem.u32(object.wrapping_add(0x18));
        let group = e.get(this, NiDX9Renderer::object_8bc);
        if previous != e.call(GET_ENTRY_POINT_00E8BF90, &args![group]).u32() {
            dirty |= 3;
            let group = e.get(this, NiDX9Renderer::object_8bc);
            let entry = e.call(GET_ENTRY_POINT_00E8BF90, &args![group]).u32();
            e.mem.set_u32(object.wrapping_add(0x18), entry);
        }
    }
    if force != 0 {
        dirty = 0xfff;
    }
    if packed {
        if count != e.mem.u32(b + 0x20) {
            e.call(DATA_SET_COUNT_00E8EF10, &args![buffer, count]);
            dirty = 0xfff;
        }
    } else {
        let owner = e.mem.u32(b + 8);
        let mut index = 0u32;
        if count > 0 {
            loop {
                e.vcall(owner, 0x1c, &args![buffer, index]);
                index += 1;
                if index >= count {
                    break;
                }
            }
        }
        e.call(DATA_SET_COUNT_00E8EF10, &args![buffer, count]);
        e.mem.set_u32(b + 0x38, 0);
        dirty = 0xfff;
    }
    e.mem.set_u32(b + 0x18, vertices);
    e.mem.set_u32(b + 0x1c, second);
    e.mem.set_u32(b + 0x4c, lengths);
    e.mem.set_u32(b + 0x44, base_count);
    e.mem.set_u32(b + 0x40, triangles);
    e.mem.set_u32(b + 0x50, list);
    e.mem.set_u32(b + 0x48, strips);
    if dirty & 0xffff_ffef != 0 {
        let mut index = 0u32;
        while index < count {
            let stream = if index < e.mem.u32(b + 0x20) {
                let slots = e.mem.u32(b + 0x28);
                e.mem.u32(slots.wrapping_add(index * 4))
            } else {
                0
            };
            let done = source != 0
                && e.vcall(
                    source,
                    0xac,
                    &args![data, object, 0u32, dirty, stream, index, 0u32, 0u32, 0u32],
                )
                .u32()
                    != 0;
            if !done {
                if count != 1 {
                    return false;
                }
                let group = e.get(this, NiDX9Renderer::object_8bc);
                let vb = e
                    .call(
                        PACK_UNTRANSFORMED_VB_00E8C1E0,
                        &args![group, data, object, dirty, stream, 0u32],
                    )
                    .u32();
                if vb == 0 {
                    return false;
                }
            }
            index += 1;
        }
    }
    if list != 0 && dirty & 0x30 != 0 {
        let dynamic = if e.mem.u8(b + 0x14) != 0 { 0x10 } else { 0 };
        let old = e.mem.u32(b + 0x34);
        let not_ex = u32::from(e.mem.u8(DIRECT3D_IS_EX) == 0);
        let made = e.with_stack(4, |e, slot| {
            let s = slot.addr();
            e.mem.set_u32(s, e.mem.u32(b + 0x30));
            let manager = e.mem.u32(this.addr() + 0x8c0);
            let made = e
                .call(
                    INDEX_BUFFER_00E8F200,
                    &args![
                        manager,
                        list,
                        index_count,
                        index_count,
                        old,
                        s,
                        not_ex,
                        dynamic
                    ],
                )
                .u32();
            (made, e.mem.u32(s))
        });
        let (index_buffer, offset) = made;
        if index_buffer == 0 {
            return false;
        }
        if old != index_buffer {
            e.call(REMOVE_IB_00E8EEE0, &args![buffer]);
            e.mem.set_u32(b + 0x34, index_buffer);
            e.mem.set_u32(b + 0x2c, index_count);
            e.mem.set_u32(b + 0x30, offset);
        }
    }
    let flags = e.mem.u16(d + 0xe);
    e.mem.set_u16(d + 0xe, flags & 0xf000);
    true
}

#[cfg(test)]
mod a07_pack_tests {
    use super::*;

    const DATA_BASE: u32 = 0x6100_0000;
    const OWNER_BASE: u32 = 0x6140_0000;
    const SOURCE_BASE: u32 = 0x6180_0000;
    const RTTI_CHAIN: u32 = 0x6000_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    struct Scene {
        renderer: Ptr<NiDX9Renderer>,
        buffer: u32,
        data: u32,
        group: u32,
        manager: u32,
    }

    /// Buffer data (one stream slot), geometry data of 9 vertices, the renderer's objects
    /// at +0x8BC and +0x8C0, and every callee doubled.
    fn set_up(e: &mut Engine, packed: bool, tri_list: bool) -> Scene {
        e.map(STREAM_LIMIT_011BF558 & !0xfff, 0x1000);
        e.mem.set_u32(STREAM_LIMIT_011BF558, 1);
        e.map(DIRECT3D_IS_EX & !0xfff, 0x1000);
        let renderer = e.new_object::<NiDX9Renderer>();
        let group = e.mem.alloc(0x10);
        e.set(renderer, NiDX9Renderer::object_8bc, group);
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(renderer.addr() + 0x8c0, manager);
        let buffer = e.mem.alloc(0x60);
        let owner = object(e, 0x6400, OWNER_BASE, 0x10);
        e.mem.set_u32(buffer + 8, owner);
        e.mem.set_u32(buffer + 0x20, 1);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, 0x5150);
        e.mem.set_u32(buffer + 0x28, slots);
        e.mem.set_u32(buffer + 0x30, 0x77);
        e.mem.set_u32(buffer + 0x34, 0x88);
        let data = object(e, 0x6800, DATA_BASE, 0x60);
        e.mem.set_u16(data + 8, 0x21);
        e.mem.set_u16(data + 0xc, 0xf001);
        e.mem.set_u32(data + 0x24, 1);
        e.mem.set_u16(data + 0x40, 4);
        e.mem.set_u16(data + 0x44, 2);
        e.mem.set_u32(data + 0x48, 0x4848);
        e.register(DATA_BASE + 0x90, |_, _| 0x1_0009u32.into_ret());
        e.register(DATA_BASE + 0xa8, |_, _| 0x2_0005u32.into_ret());
        // An RTTI chain of two records: the data's own, then 011f4a88 or another.
        let chain = RTTI_CHAIN;
        e.map(chain, 0x1000);
        e.mem.set_u32(chain + 4, chain + 0x10);
        let parent = if tri_list {
            TRI_LIST_DATA_RTTI_011F4A88
        } else {
            0
        };
        e.mem.set_u32(chain + 0x14, parent);
        e.register(DATA_BASE + 8, |_, _| RTTI_CHAIN.into_ret());
        e.register(IS_KIND_OF, |_, _| 1u32.into_ret());
        e.register(TRI_SHAPE_DATA_GET_TRI_LIST, |_, _| 0x7100u32.into_ret());
        e.register(TRI_STRIPS_DATA_GET_STRIP_LISTS, |_, _| 0x7200u32.into_ret());
        if packed {
            e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 1u32.into_ret());
        } else {
            e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 0u32.into_ret());
        }
        e.register(OWNER_BASE + 0x1c, nop);
        e.register(DATA_SET_COUNT_00E8EF10, nop);
        e.register(GET_ENTRY_POINT_00E8BF90, |_, _| 0xe1u32.into_ret());
        e.register(PACK_UNTRANSFORMED_VB_00E8C1E0, |_, _| 1u32.into_ret());
        e.register(INDEX_BUFFER_00E8F200, |e, a| {
            e.mem.set_u32(a[5], 0x99);
            0x1b00u32.into_ret()
        });
        e.register(REMOVE_IB_00E8EEE0, nop);
        Scene {
            renderer,
            buffer,
            data,
            group,
            manager,
        }
    }

    #[test]
    fn pack_needs_buffer_and_data_and_skips_packed_data() {
        let mut e = Engine::new();
        let s = set_up(&mut e, true, true);
        let (r, b, d) = (s.renderer, s.buffer, s.data);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e7_1fe0, &args![r, 0u32, d, 0u32, 0u32, 0u32])
                .u8(),
            0
        );
        assert_eq!(
            e.call(0x00e7_1fe0, &args![r, b, 0u32, 0u32, 0u32, 0u32])
                .u8(),
            0
        );
        e.mem.set_u16(d + 0xe, 0x4123);
        assert_eq!(
            e.call(0x00e7_1fe0, &args![r, b, d, 0u32, 0u32, 0u32]).u8(),
            1
        );
        assert!(calls(&e, DATA_BASE + 0x90).is_empty());
        assert_eq!(e.mem.u16(d + 0xe), 0x4123);
    }

    #[test]
    fn unpacked_triangles_are_packed_with_their_index_buffer() {
        let mut e = Engine::new();
        let s = set_up(&mut e, false, true);
        let (r, b, d) = (s.renderer, s.buffer, s.data);
        e.mem.set_u16(d + 0xe, 0x4000);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e7_1fe0, &args![r, b, d, 0u32, 0u32, 0u32]).u8(),
            1
        );
        // Bit 0 of +0xC (limit 1) on top, +0x24 set with the top bits of +0xC.
        assert_eq!(e.mem.u32(b + 4), 0x01a0_0000);
        assert_eq!(
            calls(&e, OWNER_BASE + 0x1c),
            vec![vec![e.mem.u32(b + 8), b, 0]]
        );
        assert_eq!(calls(&e, DATA_SET_COUNT_00E8EF10), vec![vec![b, 1]]);
        assert!(calls(&e, IS_KIND_OF).is_empty());
        let fields: Vec<u32> = [0x18, 0x1c, 0x40, 0x44, 0x48, 0x4c, 0x50]
            .iter()
            .map(|o| e.mem.u32(b + o))
            .collect();
        assert_eq!(fields, vec![9, 0x21, 5, 4, 1, 0, 0x7100]);
        assert_eq!(
            calls(&e, PACK_UNTRANSFORMED_VB_00E8C1E0),
            vec![vec![s.group, d, 0, 0xfff, 0x5150, 0]]
        );
        assert_eq!(
            calls(&e, INDEX_BUFFER_00E8F200)[0][..5],
            [s.manager, 0x7100, 12, 12, 0x88]
        );
        assert_eq!(calls(&e, INDEX_BUFFER_00E8F200)[0][6..], [1, 0]);
        assert_eq!(calls(&e, REMOVE_IB_00E8EEE0), vec![vec![b]]);
        assert_eq!(e.mem.u32(b + 0x34), 0x1b00);
        assert_eq!(e.mem.u32(b + 0x2c), 12);
        assert_eq!(e.mem.u32(b + 0x30), 0x99);
        assert_eq!(e.mem.u32(b + 0x38), 0);
        assert_eq!(e.mem.u16(d + 0xe), 0x4000);
    }

    #[test]
    fn packed_strips_keep_their_dirty_bits_and_pack_through_the_source() {
        let mut e = Engine::new();
        let s = set_up(&mut e, true, false);
        let (r, b, d) = (s.renderer, s.buffer, s.data);
        e.mem.set_u16(d + 0xe, 0x2010);
        let source = object(&mut e, 0x6c00, SOURCE_BASE, 0x30);
        e.mem.set_u32(source + 0x20, 1);
        e.register(SOURCE_BASE + 0xac, |_, _| 1u32.into_ret());
        // The object's entry point differs: bits 3 join, and it takes the new one.
        let shader = e.mem.alloc(0x20);
        e.mem.set_u32(shader + 0x18, 0xe0);
        // The index buffer comes back the same: nothing replaced.
        e.register(INDEX_BUFFER_00E8F200, |_, a| a[4].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e7_1fe0, &args![r, b, d, shader, source, 0u32])
                .u8(),
            1
        );
        assert_eq!(e.mem.u32(shader + 0x18), 0xe1);
        assert!(calls(&e, DATA_SET_COUNT_00E8EF10).is_empty());
        assert_eq!(
            calls(&e, SOURCE_BASE + 0xac),
            vec![vec![source, d, shader, 0, 0x13, 0x5150, 0, 0, 0, 0]]
        );
        assert!(calls(&e, PACK_UNTRANSFORMED_VB_00E8C1E0).is_empty());
        let fields: Vec<u32> = [0x40, 0x44, 0x48, 0x4c, 0x50]
            .iter()
            .map(|o| e.mem.u32(b + o))
            .collect();
        assert_eq!(fields, vec![5, 4, 2, 0x4848, 0x7200]);
        assert_eq!(calls(&e, INDEX_BUFFER_00E8F200)[0][2], 8);
        assert!(calls(&e, REMOVE_IB_00E8EEE0).is_empty());
        assert_eq!(e.mem.u16(d + 0xe), 0x2000);
    }

    #[test]
    fn pack_fails_when_several_streams_cannot_be_packed() {
        let mut e = Engine::new();
        let s = set_up(&mut e, true, true);
        let (r, b, d) = (s.renderer, s.buffer, s.data);
        e.mem.set_u16(d + 0xe, 0x0001);
        let source = object(&mut e, 0x6c00, SOURCE_BASE, 0x30);
        e.mem.set_u32(source + 0x20, 2);
        e.register(SOURCE_BASE + 0xac, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        // Forced: all bits; the count differs from the buffer's 1, so it is set.
        assert_eq!(
            e.call(0x00e7_1fe0, &args![r, b, d, 0u32, source, 1u32])
                .u8(),
            0
        );
        assert_eq!(calls(&e, DATA_SET_COUNT_00E8EF10), vec![vec![b, 2]]);
        assert_eq!(calls(&e, SOURCE_BASE + 0xac).len(), 1);
        assert_eq!(e.mem.u16(d + 0xe), 0x0001);
    }
}

/// The callees of `00e74120`: `00b601a0` (`thiscall` on the map: empties it), `00e8bff0`
/// and `00e8c0d0` (`thiscall` on the object at +0x8BC: four words, a buffer, the start, the
/// size and a word, answering the address the streams pack at; and one word, the buffer,
/// after them), `00e8c820` (`thiscall` on +0x8BC, six words: packs a partition's stream),
/// `00e6d780` (`thiscall` on the renderer, six words: packs a partition's buffer).
const MAP_REMOVE_ALL_00B601A0: u32 = 0x00b6_01a0;
const STREAMS_BEGIN_00E8BFF0: u32 = 0x00e8_bff0;
const STREAMS_END_00E8C0D0: u32 = 0x00e8_c0d0;
const PACK_PARTITION_STREAM_00E8C820: u32 = 0x00e8_c820;
const PACK_PARTITION_00E6D780: u32 = 0x00e6_d780;

/// The stream of a record's data (+0x18) at the record's index (+0x1C): the data's +0x28
/// element, or 0 past its count (+0x20).
fn record_stream(e: &Engine, record: u32) -> u32 {
    let data = e.mem.u32(record.wrapping_add(0x18));
    let index = e.mem.u32(record.wrapping_add(0x1c));
    if index < e.mem.u32(data.wrapping_add(0x20)) {
        e.mem
            .u32(e.mem.u32(data.wrapping_add(0x28)).wrapping_add(index * 4))
    } else {
        0
    }
}

// Translated from 00e74120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: packs every geometry recorded in the map at +0x610 (the
/// records `00e72980` and `00e72aa0` chain per stream key), under the locks at +0x80 and
/// +0x100 (`thiscall`, no words). It walks the map's items (hash size +0x614, buckets
/// +0x618; the next bucket through the map's virtual +4). Records under key 0 are packed one
/// by one when their stream index is 0: `PackGeometryBuffer` (no partition) or `00e6d780`
/// (with one), forced. Under any other key the records' streams span from the smallest
/// start (+0xC) to the largest end (start + size +0x14); when that span is not empty
/// `00e8bff0` gives the base for the last record's stream buffer (+8, its word +0x10), and
/// each record's stream is packed at its start plus the base, by the source's virtual +0xAC
/// or else `PackUntransformedVB` / `00e8c820` (with a partition), all bits dirty; a first
/// stream with indices remakes the data's index buffer (`00e8f200`: index counts from the
/// strip lengths, or by the primitive type +0x3C, 4: `3·` the words +0x40 and +0x44, 5:
/// each `+ 2`), then `00e8c0d0` ends the buffer. The records of key 0 and of a non-empty
/// span go on a list; when the map is done it is emptied (`00b601a0`), and each listed record whose
/// object's +0xE bits `0x7000` read `0x4000` at stream 0 has them cleared (keeping `0xF000`)
/// and goes to `00e6fa90`; every listed record is freed (0x24 bytes).
pub fn fn_00e74120(e: &mut Engine, this: Ptr<NiDX9Renderer>) {
    let t = this.addr();
    let first_lock = t + 0x80;
    let second_lock = t + 0x100;
    e.call(ENTER_CRITICAL_SECTION, &args![first_lock]);
    e.call(ENTER_CRITICAL_SECTION, &args![second_lock]);
    let map = t + 0x610;
    let mut item = 0u32;
    let size = e.mem.u32(map + 4);
    let mut bucket = 0u32;
    while bucket < size {
        let head = e.mem.u32(e.mem.u32(map + 8).wrapping_add(4 * bucket));
        if head != 0 {
            item = head;
            break;
        }
        bucket += 1;
    }
    let mut done = 0u32;
    while item != 0 {
        let key = e.mem.u32(item.wrapping_add(4));
        let first = e.mem.u32(item.wrapping_add(8));
        let mut next_item = e.mem.u32(item);
        if next_item == 0 {
            let hash = e.vcall(map, 4, &args![key]).u32();
            let mut index = hash.wrapping_add(1);
            let size = e.mem.u32(map + 4);
            while index < size {
                let head = e.mem.u32(e.mem.u32(map + 8).wrapping_add(4 * index));
                if head != 0 {
                    next_item = head;
                    break;
                }
                index += 1;
            }
        }
        if first != 0 {
            if key == 0 {
                let mut record = first;
                while record != 0 {
                    let next = e.mem.u32(record.wrapping_add(0x20));
                    if e.mem.u32(record.wrapping_add(0x1c)) == 0 {
                        let word = |e: &Engine, offset: u32| e.mem.u32(record.wrapping_add(offset));
                        let partition = word(e, 8);
                        let (data, object, skin, source) =
                            (word(e, 0x18), word(e, 0), word(e, 4), word(e, 0xc));
                        if partition != 0 {
                            e.call(
                                PACK_PARTITION_00E6D780,
                                &args![this, data, object, skin, partition, source, 1u32],
                            );
                        } else {
                            ni_xenon_renderer_pack_geometry_buffer(
                                e,
                                this,
                                Ptr::new(data),
                                Ptr::new(object),
                                skin,
                                source,
                                1,
                            );
                        }
                    }
                    e.mem.set_u32(record.wrapping_add(0x20), done);
                    done = record;
                    record = next;
                }
            } else {
                let mut start = u32::MAX;
                let mut end = 0u32;
                let mut stream = 0u32;
                let mut record = first;
                while record != 0 {
                    let next = e.mem.u32(record.wrapping_add(0x20));
                    stream = record_stream(e, record);
                    let at = e.mem.u32(stream.wrapping_add(0xc));
                    if at < start {
                        start = at;
                    }
                    let stop = at.wrapping_add(e.mem.u32(stream.wrapping_add(0x14)));
                    if stop > end {
                        end = stop;
                    }
                    record = next;
                }
                if end > start {
                    let group = e.get(this, NiDX9Renderer::object_8bc);
                    let buffer = e.mem.u32(stream.wrapping_add(8));
                    let word_10 = e.mem.u32(stream.wrapping_add(0x10));
                    let base = e
                        .call(
                            STREAMS_BEGIN_00E8BFF0,
                            &args![group, buffer, start, end - start, word_10],
                        )
                        .u32();
                    let mut record = first;
                    loop {
                        let word = |e: &Engine, offset: u32| e.mem.u32(record.wrapping_add(offset));
                        let data = word(e, 0x18);
                        let index = word(e, 0x1c);
                        let next = word(e, 0x20);
                        stream = record_stream(e, record);
                        let offset = e.mem.u32(stream.wrapping_add(0xc)).wrapping_add(base);
                        let source = word(e, 0xc);
                        let (object, skin, partition) = (word(e, 0), word(e, 4), word(e, 8));
                        let packed = source != 0
                            && e.vcall(
                                source,
                                0xac,
                                &args![
                                    object, skin, partition, 0xfffu32, stream, index, offset, 0u32,
                                    0u32
                                ],
                            )
                            .u32()
                                != 0;
                        if !packed {
                            let partition = word(e, 8);
                            let object = word(e, 0);
                            let group = e.get(this, NiDX9Renderer::object_8bc);
                            let skin = word(e, 4);
                            if partition != 0 {
                                e.call(
                                    PACK_PARTITION_STREAM_00E8C820,
                                    &args![
                                        group, object, skin, partition, 0xfffu32, stream, offset
                                    ],
                                );
                            } else {
                                e.call(
                                    PACK_UNTRANSFORMED_VB_00E8C1E0,
                                    &args![group, object, skin, 0xfffu32, stream, offset],
                                );
                            }
                        }
                        if word(e, 0x1c) == 0 && e.mem.u32(data.wrapping_add(0x50)) != 0 {
                            let list = e.mem.u32(data.wrapping_add(0x50));
                            let at = |e: &Engine, offset: u32| e.mem.u32(data.wrapping_add(offset));
                            let lengths = at(e, 0x4c);
                            let (first_count, second_count) = if lengths != 0 {
                                let mut sum = 0u32;
                                let mut strip = at(e, 0x48);
                                while strip != 0 {
                                    let length =
                                        e.mem.u16(lengths.wrapping_add(strip * 2).wrapping_sub(2));
                                    sum = sum.wrapping_add(u32::from(length));
                                    strip -= 1;
                                }
                                (sum, sum)
                            } else {
                                match at(e, 0x3c) {
                                    4 => (at(e, 0x40).wrapping_mul(3), at(e, 0x44).wrapping_mul(3)),
                                    5 => (at(e, 0x40).wrapping_add(2), at(e, 0x44).wrapping_add(2)),
                                    _ => (0, 0),
                                }
                            };
                            let dynamic = if e.mem.u8(data.wrapping_add(0x14)) != 0 {
                                0x10
                            } else {
                                0
                            };
                            let old = at(e, 0x34);
                            let not_ex = u32::from(e.mem.u8(DIRECT3D_IS_EX) == 0);
                            let slot_value = at(e, 0x30);
                            let (index_buffer, slot_value) = e.with_stack(4, |e, slot| {
                                let s = slot.addr();
                                e.mem.set_u32(s, slot_value);
                                let manager = e.mem.u32(t + 0x8c0);
                                let made = e
                                    .call(
                                        INDEX_BUFFER_00E8F200,
                                        &args![
                                            manager,
                                            list,
                                            first_count,
                                            second_count,
                                            old,
                                            s,
                                            not_ex,
                                            dynamic
                                        ],
                                    )
                                    .u32();
                                (made, e.mem.u32(s))
                            });
                            e.mem.set_u32(data.wrapping_add(0x34), index_buffer);
                            e.mem.set_u32(data.wrapping_add(0x2c), first_count);
                            e.mem.set_u32(data.wrapping_add(0x30), slot_value);
                        }
                        e.mem.set_u32(record.wrapping_add(0x20), done);
                        done = record;
                        record = next;
                        if record == 0 {
                            break;
                        }
                    }
                    let group = e.get(this, NiDX9Renderer::object_8bc);
                    let buffer = e.mem.u32(stream.wrapping_add(8));
                    e.call(STREAMS_END_00E8C0D0, &args![group, buffer]);
                }
            }
        }
        item = next_item;
    }
    e.call(MAP_REMOVE_ALL_00B601A0, &args![map]);
    let mut record = done;
    while record != 0 {
        let object = e.mem.u32(record);
        let next = e.mem.u32(record.wrapping_add(0x20));
        let flags = e.mem.u16(object.wrapping_add(0xe));
        if flags & 0x7000 == 0x4000 && e.mem.u32(record.wrapping_add(0x1c)) == 0 {
            e.mem.set_u16(object.wrapping_add(0xe), flags & 0xf000);
            let skin = e.mem.u32(record.wrapping_add(4));
            let partition = e.mem.u32(record.wrapping_add(8));
            fn_00e6fa90(e, Ptr::new(t), Ptr::new(object), Ptr::new(skin), partition);
        }
        e.call(SIZED_DELETE, &args![record, RECORD_SIZE]);
        record = next;
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![second_lock]);
    e.call(LEAVE_CRITICAL_SECTION, &args![first_lock]);
}

#[cfg(test)]
mod a07_flush_tests {
    use super::*;

    const MAP_BASE: u32 = 0x6100_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A record: object, skin, partition, source, data, stream index, next.
    fn record(e: &mut Engine, words: [u32; 7]) -> u32 {
        let r = e.mem.alloc(RECORD_SIZE);
        for (offset, value) in [0, 4, 8, 0xc, 0x18, 0x1c, 0x20].iter().zip(words) {
            e.mem.set_u32(r + offset, value);
        }
        r
    }

    fn renderer(e: &mut Engine) -> Ptr<NiDX9Renderer> {
        e.map(DIRECT3D_IS_EX & !0xfff, 0x1000);
        let r = e.new_object::<NiDX9Renderer>();
        let targets: Vec<u32> = (0..8).map(|n| MAP_BASE + 4 * n).collect();
        e.put_vtable(0x6000, &targets);
        e.mem.set_u32(r.addr() + 0x610, 0x6000);
        for addr in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            MAP_REMOVE_ALL_00B601A0,
            SIZED_DELETE,
            TEXTURE_SET_00A670C0,
            PACK_PARTITION_00E6D780,
            PACK_UNTRANSFORMED_VB_00E8C1E0,
            PACK_PARTITION_STREAM_00E8C820,
            STREAMS_END_00E8C0D0,
        ] {
            e.register(addr, nop);
        }
        e.register(STREAMS_BEGIN_00E8BFF0, |_, _| 0x1000u32.into_ret());
        e.register(INDEX_BUFFER_00E8F200, |e, a| {
            e.mem.set_u32(a[5], 0x99);
            0x1b00u32.into_ret()
        });
        e.register(MAP_BASE + 4, |_, a| u32::from(a[1] != 0).into_ret());
        r
    }

    #[test]
    fn flush_packs_every_recorded_geometry_and_frees_the_records() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let t = r.addr();
        let group = e.mem.alloc(0x10);
        e.set(r, NiDX9Renderer::object_8bc, group);
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(t + 0x8c0, manager);
        // Key 0: a partition's record, one without data (PackGeometryBuffer answers false
        // at once) and one of stream 1 (not packed).
        let first_object = e.mem.alloc(0x40);
        e.mem.set_u16(first_object + 0xe, 0x4123);
        e.mem.set_u8(first_object + 0x38, 0x30);
        let fourth = record(&mut e, [first_object, 0, 0, 0, 0x5d4, 1, 0]);
        let second = record(&mut e, [first_object, 0, 0, 0, 0, 0, fourth]);
        let first = record(&mut e, [first_object, 0x11, 0x3000, 0x13, 0x5d1, 0, second]);
        // Key 0x55: one record whose stream packs at the base plus its start.
        let stream = e.mem.alloc(0x18);
        for (offset, value) in [(8, 0xb0f), (0xc, 0x40), (0x10, 7), (0x14, 0x20)] {
            e.mem.set_u32(stream + offset, value);
        }
        let data = e.mem.alloc(0x60);
        let slots = e.mem.alloc(4);
        e.mem.set_u32(slots, stream);
        for (offset, value) in [
            (0x20, 1),
            (0x28, slots),
            (0x50, 0x9000),
            (0x3c, 4),
            (0x40, 5),
            (0x44, 6),
            (0x34, 0x88),
            (0x30, 0x77),
        ] {
            e.mem.set_u32(data + offset, value);
        }
        e.mem.set_u8(data + 0x14, 1);
        let third_object = e.mem.alloc(0x40);
        e.mem.set_u16(third_object + 0xe, 0x4abc);
        e.mem.set_u8(third_object + 0x38, 0x30);
        let third = record(&mut e, [third_object, 0x31, 0, 0, data, 0, 0]);
        // Two buckets: item (0, first) and item (0x55, third).
        let items = e.mem.alloc(0x18);
        e.mem.set_u32(items + 4, 0);
        e.mem.set_u32(items + 8, first);
        e.mem.set_u32(items + 0x10, 0x55);
        e.mem.set_u32(items + 0x14, third);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(buckets, items);
        e.mem.set_u32(buckets + 4, items + 0xc);
        e.mem.set_u32(t + 0x614, 2);
        e.mem.set_u32(t + 0x618, buckets);
        e.call_log = Some(vec![]);
        e.call(0x00e7_4120, &args![r]);
        assert_eq!(
            calls(&e, ENTER_CRITICAL_SECTION),
            vec![vec![t + 0x80], vec![t + 0x100]]
        );
        assert_eq!(
            calls(&e, MAP_BASE + 4),
            vec![vec![t + 0x610, 0], vec![t + 0x610, 0x55]]
        );
        assert_eq!(
            calls(&e, PACK_PARTITION_00E6D780),
            vec![vec![t, 0x5d1, first_object, 0x11, 0x3000, 0x13, 1]]
        );
        assert_eq!(
            calls(&e, STREAMS_BEGIN_00E8BFF0),
            vec![vec![group, 0xb0f, 0x40, 0x20, 7]]
        );
        assert_eq!(
            calls(&e, PACK_UNTRANSFORMED_VB_00E8C1E0),
            vec![vec![group, third_object, 0x31, 0xfff, stream, 0x1040]]
        );
        let index = calls(&e, INDEX_BUFFER_00E8F200);
        assert_eq!(index.len(), 1);
        assert_eq!(index[0][..5], [manager, 0x9000, 15, 18, 0x88]);
        assert_eq!(index[0][6..], [1, 0x10]);
        assert_eq!(e.mem.u32(data + 0x34), 0x1b00);
        assert_eq!(e.mem.u32(data + 0x2c), 15);
        assert_eq!(e.mem.u32(data + 0x30), 0x99);
        assert_eq!(calls(&e, STREAMS_END_00E8C0D0), vec![vec![group, 0xb0f]]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![t + 0x610]]);
        assert_eq!(
            calls(&e, SIZED_DELETE),
            vec![
                vec![third, RECORD_SIZE],
                vec![fourth, RECORD_SIZE],
                vec![second, RECORD_SIZE],
                vec![first, RECORD_SIZE]
            ]
        );
        assert_eq!(e.mem.u16(third_object + 0xe), 0x4000);
        assert_eq!(e.mem.u16(first_object + 0xe), 0x4000);
        assert_eq!(calls(&e, TEXTURE_SET_00A670C0).len(), 3);
        assert_eq!(
            calls(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![t + 0x100], vec![t + 0x80]]
        );
    }

    #[test]
    fn flush_keeps_records_whose_streams_span_nothing() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let t = r.addr();
        let stream = e.mem.alloc(0x18);
        let data = e.mem.alloc(0x60);
        let slots = e.mem.alloc(4);
        e.mem.set_u32(slots, stream);
        e.mem.set_u32(data + 0x20, 1);
        e.mem.set_u32(data + 0x28, slots);
        let only = record(&mut e, [0, 0, 0, 0, data, 0, 0]);
        let items = e.mem.alloc(0xc);
        e.mem.set_u32(items + 4, 0x55);
        e.mem.set_u32(items + 8, only);
        let buckets = e.mem.alloc(4);
        e.mem.set_u32(buckets, items);
        e.mem.set_u32(t + 0x614, 1);
        e.mem.set_u32(t + 0x618, buckets);
        e.call_log = Some(vec![]);
        e.call(0x00e7_4120, &args![r]);
        assert!(calls(&e, STREAMS_BEGIN_00E8BFF0).is_empty());
        assert!(calls(&e, SIZED_DELETE).is_empty());
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0).len(), 1);
    }
}

/// The callees of `00e70140`: `00e8f360` (`thiscall` on the object at +0x8C0, five words:
/// an index buffer for a vertex count), `00e8fc50` (`thiscall` on it, four words: an index
/// buffer of a size), `004b3ae0` (`cdecl`, three words: a point out, a point, the
/// transform's rotation; answers the out pointer).
const POINT_INDICES_00E8F360: u32 = 0x00e8_f360;
const NEW_INDEX_BUFFER_00E8FC50: u32 = 0x00e8_fc50;
const ROTATE_POINT_004B3AE0: u32 = 0x004b_3ae0;
/// `"NiDX9IndexBufferManager::PackBuffer> Failed to lock index buffer\n"` (`010eddd0`).
const LOCK_FAILED_MESSAGE_010EDDD0: u32 = 0x010e_ddd0;
/// `IDirect3DIndexBuffer9::Lock` (slot 11) and `Unlock` (slot 12).
const INDEX_BUFFER_LOCK: u32 = 0x2c;
const INDEX_BUFFER_UNLOCK: u32 = 0x30;
/// `IDirect3DDevice9::SetStreamSource` (slot 100) and `SetIndices` (slot 104).
const DEVICE_SET_STREAM_SOURCE: u32 = 0x190;
const DEVICE_SET_INDICES: u32 = 0x1a0;
/// `D3DTS_WORLD`.
const TRANSFORM_WORLD: u32 = 0x100;

// Translated from 00e70140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1BC, `Do_RenderPoints`' slot on Xbox):
/// draws a geometry's points as camera facing quads (`thiscall`, one word). Nothing with the
/// byte at +0x6FC set, or when the object at +0xC has bit 0 of its first word's byte +0x18
/// set and its third word's float +0x3C is not above 0 (a NaN draws), or for data (+0xB8)
/// without vertices (virtual +0x90). The data joins the group (`AddObjectToGroup_ov2` with
/// +0x8B4); its buffer data (+0x34) gets one stream (`00e8ef10`), four vertices and two
/// triangles per point, the type word `0x1C00000`, and the renderer the geometry's bound
/// (+0x20, else `011f4288`) at +0x664. The indices: with the data's index array (+0x44) six
/// per point (`4i, 4i+1, 4i+2, 4i, 4i+2, 4i+3` of each index, written into the buffer data's
/// index buffer, made by `00e8fc50` when missing), else `00e8f360`'s. The world transform
/// (+0x68) goes to +0x940 (`00b71a40`) and the device (`SetTransform(0x100, ..)`) and to
/// +0x8B8's virtuals +0x2C/+0x30; the camera's right and up (+0x634, +0x640) through it
/// (`004b3ae0`) give +0x64C and +0x658. With the data's byte array (+0x64) `00e6ab40` builds
/// 0x28-byte vertices (declaration `00e69790`), else `00e69ea0` 0x24-byte ones (`00e696a0`),
/// into the stream's buffer between `00e8bff0` and `00e8c0d0` (nothing without one). Then
/// the property at +0xC0 draws its passes (`DrawIndexedPrimitive` per pass) with the
/// geometry's +0x9C.
pub fn fn_00e70140(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    let g = geometry.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let owner = e.mem.u32(t + 0xc);
    if e.mem.u8(e.mem.u32(owner).wrapping_add(0x18)) & 1 != 0 {
        let limit = x87_m32(
            e.mem
                .u32(e.mem.u32(owner.wrapping_add(8)).wrapping_add(0x3c)),
        );
        // `FCOMP` of 0 with it: on only when 0 is below it or the compare is unordered.
        if limit <= 0.0 {
            return;
        }
    }
    e.with_stack(0x7c, |e, frame| {
        let f = frame.addr();
        let data = e.mem.u32(g.wrapping_add(0xb8));
        for i in 0..0xd {
            let word = e.mem.u32(g.wrapping_add(0x68 + 4 * i));
            e.mem.set_u32(f + 0x48 + 4 * i, word);
        }
        let mut bound = e.mem.u32(g.wrapping_add(0x20));
        if bound == 0 {
            bound = DEFAULT_BOUND;
        }
        for i in 0..4 {
            let word = e.mem.u32(bound + 4 * i);
            e.mem.set_u32(f + 0x38 + 4 * i, word);
        }
        let count = u32::from(e.vcall(data, 0x90, &args![]).u16());
        let second = u32::from(e.mem.u16(data.wrapping_add(8)));
        if count == 0 {
            return;
        }
        let property = e.mem.u32(g.wrapping_add(0xc0));
        let manager = e.mem.u32(t + 0x8ac);
        let group = e.mem.u32(t + 0x8b4);
        e.call(
            ADD_OBJECT_TO_GROUP,
            &args![manager, group, data, 0u32, 0u32, 0u32, 0u32],
        );
        let buffer = e.mem.u32(data.wrapping_add(0x34));
        let vertices = count * 4;
        e.call(DATA_SET_COUNT_00E8EF10, &args![buffer, 1u32]);
        let radius = x87_quiet_bits(e.mem.u32(f + 0x44));
        e.mem.set_u32(buffer.wrapping_add(0x18), vertices);
        e.mem.set_u32(buffer.wrapping_add(0x1c), second * 4);
        e.mem.set_u32(buffer.wrapping_add(0x40), count * 2);
        e.mem.set_u32(buffer.wrapping_add(0x44), second * 2);
        e.mem.set_u32(buffer.wrapping_add(0x50), 0);
        e.mem.set_u32(buffer.wrapping_add(0x4c), 0);
        e.mem.set_u32(buffer.wrapping_add(0x48), 1);
        for (i, offset) in [0x664u32, 0x668, 0x66c].iter().enumerate() {
            let word = e.mem.u32(f + 0x38 + 4 * i as u32);
            e.mem.set_u32(t + offset, word);
        }
        e.mem.set_u32(t + 0x670, radius);
        e.mem.set_u32(buffer.wrapping_add(4), 0x01c0_0000);
        let not_ex = u32::from(e.mem.u8(DIRECT3D_IS_EX) == 0);
        let index_buffer = if e.mem.u32(data.wrapping_add(0x44)) == 0 {
            let indices = e.mem.u32(t + 0x8c0);
            e.call(
                POINT_INDICES_00E8F360,
                &args![indices, count, second, 0u32, 0u32, not_ex],
            )
            .u32()
        } else {
            let mut index_buffer = e.mem.u32(buffer.wrapping_add(0x34));
            let size = second * 12;
            if index_buffer == 0 {
                let indices = e.mem.u32(t + 0x8c0);
                index_buffer = e
                    .call(
                        NEW_INDEX_BUFFER_00E8FC50,
                        &args![indices, size, 0u32, 0x65u32, not_ex],
                    )
                    .u32();
                e.mem.set_u32(buffer.wrapping_add(0x34), index_buffer);
                e.mem.set_u32(buffer.wrapping_add(0x2c), second * 6);
                e.mem.set_u32(buffer.wrapping_add(0x30), size);
                if index_buffer == 0 {
                    return;
                }
            }
            e.mem.set_u32(f + 0x1c, 0);
            let locked = e
                .vcall(
                    index_buffer,
                    INDEX_BUFFER_LOCK,
                    &args![0u32, count * 12, f + 0x1c, 0u32],
                )
                .u32() as i32;
            if locked < 0 {
                e.call(REPORT_00A29680, &args![LOCK_FAILED_MESSAGE_010EDDD0]);
                e.vcall(index_buffer, INDEX_BUFFER_UNLOCK, &args![]);
                e.vcall(index_buffer, RELEASE, &args![]);
                return;
            }
            let array = e.mem.u32(data.wrapping_add(0x44));
            let mut at = e.mem.u32(f + 0x1c);
            let mut point = 0u32;
            while point < count {
                let first = u32::from(e.mem.u16(array.wrapping_add(point * 2))) * 4;
                for value in [first, first + 1, first + 2, first, first + 2, first + 3] {
                    e.mem.set_u16(at, value as u16);
                    at = at.wrapping_add(2);
                }
                point += 1;
            }
            e.vcall(index_buffer, INDEX_BUFFER_UNLOCK, &args![]);
            index_buffer
        };
        if index_buffer == 0 {
            return;
        }
        let word = |e: &Engine, offset: u32| e.mem.u32(data.wrapping_add(offset));
        let (sizes, colors, scales, angles, positions, bytes) = (
            word(e, 0x4c),
            word(e, 0x28),
            word(e, 0x54),
            word(e, 0x5c),
            word(e, 0x20),
            word(e, 0x64),
        );
        let other = e.mem.u32(t + 0x8b8);
        e.vcall(other, 0x2c, &args![0u32]);
        let scale = x87_quiet_bits(e.mem.u32(f + 0x78));
        let world = t + 0x940;
        e.call(
            D3D_FROM_NI_00B71A40,
            &args![world, f + 0x48, f + 0x6c, scale],
        );
        let device = e.get(this, NiDX9Renderer::device);
        e.vcall(device, DEVICE_SET_TRANSFORM, &args![TRANSFORM_WORLD, world]);
        let other = e.mem.u32(t + 0x8b8);
        e.vcall(other, 0x30, &args![f + 0x48]);
        for (from, to) in [(0x634u32, QUAD_RIGHT), (0x640, QUAD_UP)] {
            let out = e
                .call(ROTATE_POINT_004B3AE0, &args![f + 0x38, t + from, f + 0x48])
                .u32();
            for i in 0..3 {
                let v = e.mem.u32(out.wrapping_add(4 * i));
                e.mem.set_u32(t + to + 4 * i, v);
            }
        }
        let (stride, declaration) = if bytes != 0 {
            (0x28u32, fn_00e69790(e, this))
        } else {
            (0x24u32, fn_00e696a0(e, this))
        };
        e.call(DATA_SET_SECOND_00E8F040, &args![buffer, declaration]);
        if e.mem.u32(buffer.wrapping_add(0x20)) > 0 {
            let strides = e.mem.u32(buffer.wrapping_add(0x24));
            e.mem.set_u32(strides, stride);
        }
        let streams = e.get(this, NiDX9Renderer::object_8bc);
        e.call(GROUP_STREAM_00E8BFA0, &args![streams, buffer, 0u32]);
        let stream = if e.mem.u32(buffer.wrapping_add(0x20)) > 0 {
            e.mem.u32(e.mem.u32(buffer.wrapping_add(0x28)))
        } else {
            0
        };
        let vertex_buffer = e.mem.u32(stream.wrapping_add(8));
        if vertex_buffer == 0 {
            return;
        }
        let start = e.mem.u32(stream.wrapping_add(0xc));
        let flags = e.mem.u32(stream.wrapping_add(0x10));
        let streams = e.get(this, NiDX9Renderer::object_8bc);
        let base = e
            .call(
                STREAMS_BEGIN_00E8BFF0,
                &args![streams, vertex_buffer, start, vertices * stride, flags],
            )
            .u32();
        if base == 0 {
            return;
        }
        let count16 = count as u16;
        if bytes != 0 {
            fn_00e6ab40(
                e, this, base, positions, colors, sizes, scales, angles, bytes, count16,
            );
        } else {
            fn_00e69ea0(
                e, this, base, positions, colors, sizes, scales, angles, count16,
            );
        }
        let streams = e.get(this, NiDX9Renderer::object_8bc);
        let vertex_buffer = e.mem.u32(stream.wrapping_add(8));
        e.call(STREAMS_END_00E8C0D0, &args![streams, vertex_buffer]);
        let passes = e.vcall(property, 0x90, &args![]).u32();
        let state = g.wrapping_add(0x9c);
        for slot in [0x78u32, 0x7c, 0x80, 0x84] {
            e.vcall(property, slot, &args![state]);
        }
        e.vcall(property, 0x88, &args![state, 1u32]);
        let other = e.mem.u32(t + 0x8b8);
        let word_10 = e.mem.u32(buffer.wrapping_add(0x10));
        e.vcall(other, 0xac, &args![word_10, 0u32]);
        let stride = if e.mem.u32(buffer.wrapping_add(0x20)) > 0 {
            e.mem.u32(e.mem.u32(buffer.wrapping_add(0x24)))
        } else {
            0
        };
        let device = e.get(this, NiDX9Renderer::device);
        let vertex_buffer = e.mem.u32(stream.wrapping_add(8));
        e.vcall(
            device,
            DEVICE_SET_STREAM_SOURCE,
            &args![0u32, vertex_buffer, 0u32, stride],
        );
        let device = e.get(this, NiDX9Renderer::device);
        e.vcall(device, DEVICE_SET_INDICES, &args![index_buffer]);
        if passes != 0 {
            loop {
                let device = e.get(this, NiDX9Renderer::device);
                let b = |e: &Engine, offset: u32| e.mem.u32(buffer.wrapping_add(offset));
                let (kind, first, vertices, triangles) =
                    (b(e, 0x3c), b(e, 0x38), b(e, 0x18), b(e, 0x40));
                e.vcall(
                    device,
                    DEVICE_DRAW_INDEXED_PRIMITIVE,
                    &args![kind, first, 0u32, vertices, 0u32, triangles],
                );
                if e.vcall(property, 0x94, &args![]).u32() == 0 {
                    break;
                }
            }
        }
        e.vcall(property, 0x8c, &args![state]);
    });
}

#[cfg(test)]
mod a07_points_tests {
    use super::*;

    const DEVICE_BASE: u32 = 0x6100_0000;
    const OTHER_BASE: u32 = 0x6140_0000;
    const DATA_BASE: u32 = 0x6180_0000;
    const PROPERTY_BASE: u32 = 0x61c0_0000;
    const INDEX_BASE: u32 = 0x6200_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let p = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(p + 4 * i as u32, v.to_bits());
        }
        p
    }

    struct Scene {
        renderer: Ptr<NiDX9Renderer>,
        device: u32,
        geometry: u32,
        data: u32,
        buffer: u32,
        strides: u32,
        streams: u32,
        vertices: u32,
        owner_flags: u32,
    }

    /// One point at (10, 20, 30) of size 2·0.5 with right (1, 0, 0) and up (0, 1, 0).
    fn set_up(e: &mut Engine) -> Scene {
        e.map(DIRECT3D_IS_EX & !0xfff, 0x1000);
        let renderer = e.new_object::<NiDX9Renderer>();
        let t = renderer.addr();
        let flags = e.mem.alloc(0x20);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner, flags);
        e.mem.set_u32(t + 0xc, owner);
        let device = object(e, 0x6000, DEVICE_BASE, 0x10);
        e.set(renderer, NiDX9Renderer::device, device);
        let other = object(e, 0x6400, OTHER_BASE, 0x10);
        e.mem.set_u32(t + 0x8b8, other);
        let streams = e.mem.alloc(0x10);
        e.set(renderer, NiDX9Renderer::object_8bc, streams);
        e.mem.set_u32(t + 0x8ac, 0x8ac0);
        e.mem.set_u32(t + 0x8b4, 0x8b40);
        e.mem.set_u32(t + 0x8c0, 0x8c00);
        e.mem.set_u32(t + 0x280, 0xdec1);
        e.mem.set_u32(t + 0x284, 0xdec2);
        for (i, v) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0].iter().enumerate() {
            e.mem.set_u32(t + 0x634 + 4 * i as u32, v.to_bits());
        }
        let geometry = e.mem.alloc(0xd0);
        let bound = floats(e, &[1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u32(geometry + 0x20, bound);
        e.mem.set_u32(geometry + 0x68 + 0x30, 1.0f32.to_bits());
        let data = object(e, 0x6800, DATA_BASE, 0x70);
        e.mem.set_u32(geometry + 0xb8, data);
        e.register(DATA_BASE + 0x90, |_, _| 1u32.into_ret());
        e.mem.set_u16(data + 8, 2);
        let positions = floats(e, &[10.0, 20.0, 30.0]);
        let sizes = floats(e, &[2.0]);
        let scales = floats(e, &[0.5]);
        e.mem.set_u32(data + 0x20, positions);
        e.mem.set_u32(data + 0x4c, sizes);
        e.mem.set_u32(data + 0x54, scales);
        let buffer = e.mem.alloc(0x60);
        e.mem.set_u32(data + 0x34, buffer);
        e.mem.set_u32(buffer + 0x20, 1);
        let strides = e.mem.alloc(4);
        e.mem.set_u32(buffer + 0x24, strides);
        let stream = e.mem.alloc(0x18);
        e.mem.set_u32(stream + 8, 0x7b00);
        e.mem.set_u32(stream + 0xc, 0x10);
        e.mem.set_u32(stream + 0x10, 3);
        let stream_list = e.mem.alloc(4);
        e.mem.set_u32(stream_list, stream);
        e.mem.set_u32(buffer + 0x28, stream_list);
        e.mem.set_u32(buffer + 0x3c, 4);
        e.mem.set_u32(buffer + 0x38, 0x38);
        let property = object(e, 0x6c00, PROPERTY_BASE, 0x10);
        e.mem.set_u32(geometry + 0xc0, property);
        e.register(PROPERTY_BASE + 0x90, |_, _| 1u32.into_ret());
        e.register(PROPERTY_BASE + 0x94, |_, _| 0u32.into_ret());
        for slot in [0x78, 0x7c, 0x80, 0x84, 0x88, 0x8c] {
            e.register(PROPERTY_BASE + slot, nop);
        }
        for slot in [0x2c, 0x30, 0xac] {
            e.register(OTHER_BASE + slot, nop);
        }
        for slot in [
            DEVICE_SET_TRANSFORM,
            DEVICE_SET_STREAM_SOURCE,
            DEVICE_SET_INDICES,
            DEVICE_DRAW_INDEXED_PRIMITIVE,
        ] {
            e.register(DEVICE_BASE + slot, nop);
        }
        for addr in [
            ADD_OBJECT_TO_GROUP,
            DATA_SET_COUNT_00E8EF10,
            D3D_FROM_NI_00B71A40,
            DATA_SET_SECOND_00E8F040,
            GROUP_STREAM_00E8BFA0,
            STREAMS_END_00E8C0D0,
            REPORT_00A29680,
        ] {
            e.register(addr, nop);
        }
        e.register(POINT_INDICES_00E8F360, |_, _| 0x1b00u32.into_ret());
        // The rotation is the identity: the point comes back as it went.
        e.register(ROTATE_POINT_004B3AE0, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        let vertices = e.mem.alloc(0xa0);
        e.register_double(STREAMS_BEGIN_00E8BFF0, move |_, _| vertices.into_ret());
        Scene {
            renderer,
            device,
            geometry,
            data,
            buffer,
            strides,
            streams,
            vertices,
            owner_flags: flags,
        }
    }

    #[test]
    fn points_build_their_quads_and_draw_each_pass() {
        let mut e = Engine::new();
        let s = set_up(&mut e);
        let (r, t) = (s.renderer, s.renderer.addr());
        e.call_log = Some(vec![]);
        e.call(0x00e7_0140, &args![r, s.geometry]);
        assert_eq!(
            calls(&e, ADD_OBJECT_TO_GROUP),
            vec![vec![0x8ac0, 0x8b40, s.data, 0, 0, 0, 0]]
        );
        let fields: Vec<u32> = [4, 0x18, 0x1c, 0x40, 0x44, 0x48, 0x4c, 0x50]
            .iter()
            .map(|o| e.mem.u32(s.buffer + o))
            .collect();
        assert_eq!(fields, vec![0x01c0_0000, 4, 8, 2, 4, 1, 0, 0]);
        assert_eq!(e.mem.u32(t + 0x664), 1.0f32.to_bits());
        assert_eq!(e.mem.u32(t + 0x670), 4.0f32.to_bits());
        assert_eq!(
            calls(&e, POINT_INDICES_00E8F360),
            vec![vec![0x8c00, 1, 2, 0, 0, 1]]
        );
        assert_eq!(
            calls(&e, DEVICE_BASE + DEVICE_SET_TRANSFORM),
            vec![vec![s.device, 0x100, t + 0x940]]
        );
        assert_eq!(e.mem.u32(t + QUAD_UP + 4), 1.0f32.to_bits());
        assert_eq!(
            calls(&e, DATA_SET_SECOND_00E8F040),
            vec![vec![s.buffer, 0xdec1]]
        );
        assert_eq!(e.mem.u32(s.strides), 0x24);
        assert_eq!(
            calls(&e, STREAMS_BEGIN_00E8BFF0),
            vec![vec![s.streams, 0x7b00, 0x10, 4 * 0x24, 3]]
        );
        // The quad as 00e69ea0 builds it.
        assert_eq!(e.mem.u32(s.vertices), 9.0f32.to_bits());
        assert_eq!(e.mem.u32(s.vertices + 0x24 * 2 + 4), 21.0f32.to_bits());
        assert_eq!(
            calls(&e, STREAMS_END_00E8C0D0),
            vec![vec![s.streams, 0x7b00]]
        );
        assert_eq!(
            calls(&e, DEVICE_BASE + DEVICE_SET_STREAM_SOURCE),
            vec![vec![s.device, 0, 0x7b00, 0, 0x24]]
        );
        assert_eq!(
            calls(&e, DEVICE_BASE + DEVICE_SET_INDICES),
            vec![vec![s.device, 0x1b00]]
        );
        assert_eq!(
            calls(&e, DEVICE_BASE + DEVICE_DRAW_INDEXED_PRIMITIVE),
            vec![vec![s.device, 4, 0x38, 0, 4, 0, 2]]
        );
        let property = e.mem.u32(s.geometry + 0xc0);
        assert_eq!(
            calls(&e, PROPERTY_BASE + 0x88),
            vec![vec![property, s.geometry + 0x9c, 1]]
        );
        assert_eq!(calls(&e, PROPERTY_BASE + 0x8c).len(), 1);
    }

    #[test]
    fn indexed_points_with_bytes_fill_the_index_buffer() {
        let mut e = Engine::new();
        let s = set_up(&mut e);
        let r = s.renderer;
        let array = e.mem.alloc(4);
        e.mem.set_u16(array, 3);
        e.mem.set_u32(s.data + 0x44, array);
        let bytes = e.mem.alloc(4);
        e.mem.set_u8(bytes, 5);
        e.mem.set_u32(s.data + 0x64, bytes);
        let index_buffer = object(&mut e, 0x7000, INDEX_BASE, 0x10);
        e.register_double(NEW_INDEX_BUFFER_00E8FC50, move |_, _| {
            index_buffer.into_ret()
        });
        let indices = e.mem.alloc(0x10);
        e.register_double(INDEX_BASE + INDEX_BUFFER_LOCK, move |e, a| {
            e.mem.set_u32(a[3], indices);
            Ret::default()
        });
        e.register(INDEX_BASE + INDEX_BUFFER_UNLOCK, nop);
        let vertices = e.mem.alloc(0xa0);
        e.register_double(STREAMS_BEGIN_00E8BFF0, move |_, _| vertices.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e7_0140, &args![r, s.geometry]);
        assert_eq!(
            calls(&e, NEW_INDEX_BUFFER_00E8FC50),
            vec![vec![0x8c00, 24, 0, 0x65, 1]]
        );
        assert_eq!(e.mem.u32(s.buffer + 0x34), index_buffer);
        assert_eq!(e.mem.u32(s.buffer + 0x2c), 12);
        assert_eq!(e.mem.u32(s.buffer + 0x30), 24);
        assert_eq!(calls(&e, INDEX_BASE + INDEX_BUFFER_LOCK)[0][1..3], [0, 12]);
        let written: Vec<u16> = (0..6).map(|i| e.mem.u16(indices + 2 * i)).collect();
        assert_eq!(written, vec![12, 13, 14, 12, 14, 15]);
        assert_eq!(calls(&e, INDEX_BASE + INDEX_BUFFER_UNLOCK).len(), 1);
        assert_eq!(
            calls(&e, DATA_SET_SECOND_00E8F040),
            vec![vec![s.buffer, 0xdec2]]
        );
        assert_eq!(e.mem.u32(s.strides), 0x28);
        assert_eq!(f32::from_bits(e.mem.u32(vertices + 0x24)), 5.0);
        assert_eq!(
            calls(&e, DEVICE_BASE + DEVICE_SET_INDICES),
            vec![vec![s.device, index_buffer]]
        );
    }

    #[test]
    fn a_failed_lock_releases_the_index_buffer() {
        let mut e = Engine::new();
        let s = set_up(&mut e);
        let array = e.mem.alloc(4);
        e.mem.set_u32(s.data + 0x44, array);
        let index_buffer = object(&mut e, 0x7000, INDEX_BASE, 0x10);
        e.mem.set_u32(s.buffer + 0x34, index_buffer);
        e.register(INDEX_BASE + INDEX_BUFFER_LOCK, |_, _| {
            0x8876_086cu32.into_ret()
        });
        e.register(INDEX_BASE + INDEX_BUFFER_UNLOCK, nop);
        e.register(INDEX_BASE + RELEASE, nop);
        e.call_log = Some(vec![]);
        e.call(0x00e7_0140, &args![s.renderer, s.geometry]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![LOCK_FAILED_MESSAGE_010EDDD0]]
        );
        assert_eq!(calls(&e, INDEX_BASE + RELEASE), vec![vec![index_buffer]]);
        assert!(calls(&e, DEVICE_BASE + DEVICE_SET_TRANSFORM).is_empty());
    }

    #[test]
    fn points_wait_for_the_owner_and_need_vertices() {
        let mut e = Engine::new();
        let s = set_up(&mut e);
        let owner = e.mem.u32(s.renderer.addr() + 0xc);
        let third = floats(&mut e, &[0.0; 16]);
        e.mem.set_u32(owner + 8, third);
        e.mem.set_u8(s.owner_flags + 0x18, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e7_0140, &args![s.renderer, s.geometry]);
        assert!(calls(&e, DATA_BASE + 0x90).is_empty());
        // A NaN draws; without vertices it stops there.
        e.mem.set_u32(third + 0x3c, 0x7fc0_0000);
        e.register(DATA_BASE + 0x90, |_, _| 0u32.into_ret());
        e.call(0x00e7_0140, &args![s.renderer, s.geometry]);
        assert_eq!(calls(&e, DATA_BASE + 0x90).len(), 1);
        assert!(calls(&e, ADD_OBJECT_TO_GROUP).is_empty());
    }
}

/// `_memcpy` (`00ec44d0`, cdecl: destination, source, count).
const COPY_BYTES_00EC44D0: u32 = 0x00ec_44d0;
/// `00e736b0` (unnamed, this unit; `thiscall`, no stack words): resets the device with the
/// presentation parameters, a nonzero byte on success.
const RESET_DEVICE_00E736B0: u32 = 0x00e7_36b0;
/// The messages `00e73eb0` gives `00a29680`.
const RESET_FAILED_010EE398: u32 = 0x010e_e398;
const RECOVER_FAILED_010EE348: u32 = 0x010e_e348;
/// The presentation parameters' size (`D3DPRESENT_PARAMETERS`, 0x38 bytes) and their place
/// in the render target group object (+0x20).
const PRESENT_PARAMETERS_SIZE: u32 = 0x38;
const PRESENT_PARAMETERS_OFFSET: u32 = 0x20;

// Translated from 00e73eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9Renderer::Recreate` by its messages; `thiscall`, two
/// words: width and height, `RET 8`). Takes the object at +0x884's virtual +0xB0 (with 0),
/// keeps its word +0x10 when that derives from the record at `0126f5d8` (`00e69b00`; null
/// otherwise and when the word is null), and works on the 0x38 bytes at +0x20 of it: saves
/// them on the stack, sets the first two words to `width` and `height` and resets the
/// device (`00e736b0`). Answers 2 when that works; otherwise reports
/// `"NiDX9Renderer::Recreate> Could not reset device ..."` (`00a29680`), puts the saved bytes
/// back and resets again: 1 when that works, else reports `"... Could not recover original
/// presentation parameters"` and answers 0.
pub fn fn_00e73eb0(e: &mut Engine, this: Ptr<NiDX9Renderer>, width: u32, height: u32) -> u32 {
    let source = e.get(this, NiDX9Renderer::object_884);
    let group = e.vcall(source, 0xb0, &args![0u32]).u32();
    let data = e.mem.u32(group.wrapping_add(0x10));
    let target = if data == 0 {
        0
    } else {
        fn_00e69b00(e, Ptr::new(data), SHADER_RTTI_0126F5D8)
    };
    let parameters = target.wrapping_add(PRESENT_PARAMETERS_OFFSET);
    e.with_stack(PRESENT_PARAMETERS_SIZE, |e, saved| {
        e.call(
            COPY_BYTES_00EC44D0,
            &args![saved, parameters, PRESENT_PARAMETERS_SIZE],
        );
        e.mem.set_u32(parameters, width);
        e.mem.set_u32(parameters.wrapping_add(4), height);
        if e.call(RESET_DEVICE_00E736B0, &args![this]).bool() {
            return 2;
        }
        e.call(REPORT_00A29680, &args![RESET_FAILED_010EE398]);
        e.call(
            COPY_BYTES_00EC44D0,
            &args![parameters, saved, PRESENT_PARAMETERS_SIZE],
        );
        if e.call(RESET_DEVICE_00E736B0, &args![this]).bool() {
            return 1;
        }
        e.call(REPORT_00A29680, &args![RECOVER_FAILED_010EE348]);
        0
    })
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// A renderer whose +0x884 object answers a group whose +0x10 word is an object of the
    /// record `0126f5d8`; the reset double answers each of `results` in turn and records
    /// the width and height it sees. Answers (engine, renderer, parameters address).
    fn recreate_rig(results: Vec<u32>) -> (Engine, Ptr<NiDX9Renderer>, u32) {
        let mut e = Engine::new();
        let this: Ptr<NiDX9Renderer> = e.new_object();
        let source = e.mem.alloc(4);
        e.put_vtable(0x0090_1000, &[0; 0x2d]);
        e.mem.set_u32(0x0090_1000 + 0xb0, 0x0090_00b0);
        e.mem.set_u32(source, 0x0090_1000);
        e.set(this, NiDX9Renderer::object_884, source);
        let group = e.mem.alloc(0x14);
        let target = e.mem.alloc(0x60);
        e.mem.set_u32(group + 0x10, target);
        e.register_double(0x0090_00b0, move |_, a| {
            assert_eq!(a[1], 0);
            group.into_ret()
        });
        // The target's `GetRTTI` (+0xC) answers the record itself.
        e.put_vtable(0x0090_2000, &[0, 0, 0, 0x0090_000c]);
        e.register(0x0090_000c, |_, _| SHADER_RTTI_0126F5D8.into_ret());
        e.mem.set_u32(target, 0x0090_2000);
        e.register(COPY_BYTES_00EC44D0, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(REPORT_00A29680, |_, _| Ret::default());
        let mut answers = results.into_iter();
        e.register_double(RESET_DEVICE_00E736B0, move |_, _| {
            answers.next().unwrap().into_ret()
        });
        let parameters = target + 0x20;
        for i in 0..14u32 {
            e.mem.set_u32(parameters + 4 * i, 100 + i);
        }
        (e, this, parameters)
    }

    fn reports(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == REPORT_00A29680)
            .map(|(_, w)| w[0])
            .collect()
    }

    #[test]
    fn recreate_answers_2_when_the_new_size_resets() {
        let (mut e, this, parameters) = recreate_rig(vec![0x1234_5601]);
        e.call_log = Some(vec![]);
        let result = e.call(0x00e7_3eb0, &args![this, 1920u32, 1080u32]).u32();
        assert_eq!(result, 2);
        assert_eq!(e.mem.u32(parameters), 1920);
        assert_eq!(e.mem.u32(parameters + 4), 1080);
        assert_eq!(e.mem.u32(parameters + 8), 102);
        assert!(reports(&e).is_empty());
    }

    #[test]
    fn recreate_answers_1_when_the_old_parameters_reset() {
        let (mut e, this, parameters) = recreate_rig(vec![0x1234_5600, 1]);
        e.call_log = Some(vec![]);
        let result = e.call(0x00e7_3eb0, &args![this, 1920u32, 1080u32]).u32();
        assert_eq!(result, 1);
        // The saved parameters are back.
        assert_eq!(e.mem.u32(parameters), 100);
        assert_eq!(e.mem.u32(parameters + 4), 101);
        assert_eq!(reports(&e), vec![RESET_FAILED_010EE398]);
    }

    #[test]
    fn recreate_answers_0_when_neither_resets() {
        let (mut e, this, _) = recreate_rig(vec![0, 0]);
        e.call_log = Some(vec![]);
        let result = e.call(0x00e7_3eb0, &args![this, 1920u32, 1080u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(
            reports(&e),
            vec![RESET_FAILED_010EE398, RECOVER_FAILED_010EE348]
        );
    }
}

// ---- Batch n084 (a02) ----

/// The `NiRTTI` records the `GetRTTI` functions answer: `NiVertexColorProperty`'s
/// (`011f4430`), `NiZBufferProperty`'s (`011f443c`) and `NiDX9Renderer`'s (`0126f0f0`);
/// their static initializers (`00fb5730`, `00fb5760`, `00fc72a0`) construct them with
/// those names.
const RTTI_NI_VERTEX_COLOR_PROPERTY: u32 = 0x011f_4430;
const RTTI_NI_Z_BUFFER_PROPERTY: u32 = 0x011f_443c;
const RTTI_NI_DX9_RENDERER: u32 = 0x0126_f0f0;
/// `00e89ed0` (`cdecl`: a word and the renderer's sampler blocks at +0x700).
const SAMPLER_HELPER_00E89ED0: u32 = 0x00e8_9ed0;
/// `00e8a9b0` (`cdecl`, the renderer).
const RENDERER_HELPER_00E8A9B0: u32 = 0x00e8_a9b0;
/// `00e8db90` (`thiscall` on a new 0x54-byte block, one stack word): its constructor.
const CONSTRUCT_00E8DB90: u32 = 0x00e8_db90;
/// `00e901a0` (`cdecl`: the texture and the renderer) and `00e904c0` (`cdecl`, the
/// texture): make a texture's renderer data.
const TEXTURE_DATA_00E901A0: u32 = 0x00e9_01a0;
const TEXTURE_DATA_00E904C0: u32 = 0x00e9_04c0;
/// `NiXenonErrorString` (`00e89d50`, engine map; `cdecl`, the `HRESULT`).
const ERROR_STRING_00E89D50: u32 = 0x00e8_9d50;
/// `NiSprintf` (`00a305b0`, engine map; `cdecl`: buffer, size, format, words).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// CRT `strcat_s` (`00ec6bd2`, `cdecl`: destination, size, source).
const STRCAT_S_00EC6BD2: u32 = 0x00ec_6bd2;
/// `D3DXGetVertexShaderProfile` and `D3DXGetPixelShaderProfile` thunks (`stdcall`, the
/// device).
const VERTEX_SHADER_PROFILE_00EE6E2E: u32 = 0x00ee_6e2e;
const PIXEL_SHADER_PROFILE_00EE6E28: u32 = 0x00ee_6e28;
/// `00a8f750` (`cdecl`, no arguments): the word `00e6bdc0` uses when its sixth is 0.
const DEFAULT_WORD_00A8F750: u32 = 0x00a8_f750;
/// `00ee8850` (`thiscall`, one stack word) on the group `00e6bdc0` is given.
const GROUP_BUFFER_00EE8850: u32 = 0x00ee_8850;
/// `NiD3DGPUProgramCache::NiD3DGPUProgramCache` (`00e8e450`, engine map; `thiscall`,
/// nine stack words).
const PROGRAM_CACHE_CONSTRUCT: u32 = 0x00e8_e450;
/// The renderer's free list of 0x10-byte nodes (`0126f0ec`; the next node at +0xC).
const FREE_NODES_0126F0EC: u32 = 0x0126_f0ec;
/// The word `00e6c760` sets and `00e6c770` answers (`0126efc0`).
const STORED_WORD_0126EFC0: u32 = 0x0126_efc0;
/// The renderer's behaviour-flag texts `00e69c00` joins (`" PURE"`, `" MT"`, `" HWVP"`,
/// `" MIXVP"`, `" SWVP"`).
const PURE_TEXT_010ED5E4: u32 = 0x010e_d5e4;
const MT_TEXT_010ED5E0: u32 = 0x010e_d5e0;
const HWVP_TEXT_010ED5D8: u32 = 0x010e_d5d8;
const MIXVP_TEXT_010ED5D0: u32 = 0x010e_d5d0;
const SWVP_TEXT_010ED5C8: u32 = 0x010e_d5c8;
/// The device type texts `00e6c690` uses (`"HAL"`, `"REF"`, `"SW"`, `"???"`) and its
/// format `"%s (%s-%s)"`.
const HAL_TEXT_010ED5BC: u32 = 0x010e_d5bc;
const REF_TEXT_010ED5C0: u32 = 0x010e_d5c0;
const SW_TEXT_010ED5C4: u32 = 0x010e_d5c4;
const UNKNOWN_TEXT_010A93F4: u32 = 0x010a_93f4;
const DESCRIPTION_FORMAT_010ED954: u32 = 0x010e_d954;
/// The failure messages of the lock and unlock functions.
const LOCK_FAILED_010ED96C: u32 = 0x010e_d96c;
const UNLOCK_FAILED_010ED9E4: u32 = 0x010e_d9e4;
const UPDATE_FAILED_010ED9A4: u32 = 0x010e_d9a4;
/// The world offset `00e6b8f0` subtracts from a translation (`011f474c`, three floats).
const WORLD_OFFSET_011F474C: u32 = 0x011f_474c;

// Translated from 00a5f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty::GetRTTI` (engine map; the record it answers is constructed with
/// that name): the address `011f4430`.
pub fn ni_vertex_color_property_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_NI_VERTEX_COLOR_PROPERTY
}

// Translated from 00a5f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiZBufferProperty::GetRTTI` (engine map; the record it answers is constructed with that
/// name): the address `011f443c`.
pub fn ni_z_buffer_property_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_NI_Z_BUFFER_PROPERTY
}

// Translated from 00a5f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiZBufferProperty::Type` (engine map): 0xB.
pub fn ni_z_buffer_property_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0xb
}

// Translated from 00e757b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x8): `NiDX9Renderer`'s `GetRTTI`, the
/// address `0126f0f0` (constructed with the name `NiDX9Renderer`).
pub fn fn_00e757b0(_e: &mut Engine, _this: Ptr<NiDX9Renderer>) -> u32 {
    RTTI_NI_DX9_RENDERER
}

// Translated from 00e695c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x98, `thiscall`, one stack word):
/// `00e89ed0(word, this + 0x700)`, answered as it is.
pub fn fn_00e695c0(e: &mut Engine, this: Ptr<NiDX9Renderer>, word: u32) -> u32 {
    e.call(
        SAMPLER_HELPER_00E89ED0,
        &args![word, this.addr().wrapping_add(0x700)],
    )
    .u32()
}

// Translated from 00e69610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xD0, no stack words): a jump to the
/// object at +0x884's virtual +0xB4 (`00aa1ac0` for the render target group's vtable
/// `011010ec`, no stack words), answered as it is.
pub fn fn_00e69610(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_884);
    e.vcall(object, 0xb4, &args![]).u32()
}

// Translated from 00e69650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x10C, `thiscall`, one stack word that is
/// not read): `00e8a9b0(this)`, then `AL` = 1 (the rest of `EAX` is the callee's).
pub fn fn_00e69650(e: &mut Engine, this: Ptr<NiDX9Renderer>, _unused_1: u32) -> bool {
    e.call(RENDERER_HELPER_00E8A9B0, &args![this]);
    true
}

/// `IDirect3DDevice9::DrawIndexedPrimitive` (vtable +0x148) with the primitive type and
/// the four words of `00e698a0`/`00e698d0` (minimum index 0), answered as it is.
fn draw_indexed(e: &mut Engine, this: Ptr<NiDX9Renderer>, primitive: u32, words: [u32; 4]) -> u32 {
    let device = e.get(this, NiDX9Renderer::device);
    e.vcall(
        device,
        0x148,
        &args![primitive, words[0], 0u32, words[1], words[2], words[3]],
    )
    .u32()
}

// Translated from 00e698a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x174, `thiscall`, four stack words):
/// `device->DrawIndexedPrimitive(4 (a triangle list), first, 0, second, third, fourth)`.
pub fn fn_00e698a0(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> u32 {
    draw_indexed(e, this, 4, [first, second, third, fourth])
}

// Translated from 00e698d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x178): `00e698a0` with primitive type 5
/// (a triangle strip).
pub fn fn_00e698d0(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> u32 {
    draw_indexed(e, this, 5, [first, second, third, fourth])
}

// Translated from 00e69900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x19C, `thiscall`, two stack words): stores
/// them at +0x628 and +0x62C.
pub fn fn_00e69900(e: &mut Engine, this: Ptr<NiDX9Renderer>, first: u32, second: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x628), first);
    e.mem.set_u32(this.addr().wrapping_add(0x62c), second);
}

// Translated from 00e69920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): moves every node of the list at
/// +0x620 (the next node at +0xC) to the front of the free list at `0126f0ec`, one by one
/// (so in reverse order), and clears +0x624 and +0x620.
pub fn fn_00e69920(e: &mut Engine, this: Ptr<NiDX9Renderer>) {
    let t = this.addr();
    let mut node = e.mem.u32(t.wrapping_add(0x620));
    while node != 0 {
        let next = e.mem.u32(node.wrapping_add(0xc));
        let free = e.mem.u32(FREE_NODES_0126F0EC);
        e.mem.set_u32(FREE_NODES_0126F0EC, node);
        e.mem.set_u32(node.wrapping_add(0xc), free);
        node = next;
    }
    e.mem.set_u32(t.wrapping_add(0x624), 0);
    e.mem.set_u32(t.wrapping_add(0x620), 0);
}

// Translated from 00e69a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x16C, `thiscall`, one stack word; `this`
/// is not read): allocates 0x54 bytes (`00aa13e0`) and, when that worked, jumps to their
/// constructor `00e8db90` with the word, answering its result; otherwise answers 0.
pub fn fn_00e69a00(e: &mut Engine, _this: Ptr<NiDX9Renderer>, word: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x54u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(CONSTRUCT_00E8DB90, &args![block, word]).u32()
}

// Translated from 00e69ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the text): copies the text into the renderer's
/// error message at `011f4648` (`_strncpy_s` with size 0x100 and count 0xFF, whose result
/// is the answer), or empties it for a null text (answering 0).
pub fn fn_00e69ad0(e: &mut Engine, text: Ptr) -> u32 {
    if text.is_null() {
        e.mem.set_u8(ERROR_MESSAGE_011F4648, 0);
        return 0;
    }
    e.call(
        STRNCPY_S,
        &args![ERROR_MESSAGE_011F4648, 0x100u32, text, 0xffu32],
    )
    .u32()
}

// Translated from 00e69b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, seven stack words; `this` is not read): writes
/// one vertex at `out`: the position's three floats, the normal's three floats, the packed
/// colour word, the floats `u` and `v`, and, unless `w` is below 0 (or a NaN: the code's
/// `FCOM` with 0 sets `C0` for both), the float `w`. Answers the address after it
/// (`out + 0x28`, or `out + 0x24` without `w`). The floats are `FLD`/`FSTP` copies.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e69b60(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    position: Ptr,
    normal: Ptr,
    color: u32,
    u: u32,
    v: u32,
    w: u32,
) -> Ptr {
    let o = out.addr();
    let copy = |e: &mut Engine, at: u32, from: u32| {
        let bits = x87_quiet_bits(e.mem.u32(from));
        e.mem.set_u32(at, bits);
    };
    for i in 0..3u32 {
        copy(
            e,
            o.wrapping_add(4 * i),
            position.addr().wrapping_add(4 * i),
        );
    }
    for i in 0..3u32 {
        copy(
            e,
            o.wrapping_add(0xc + 4 * i),
            normal.addr().wrapping_add(4 * i),
        );
    }
    e.mem.set_u32(o.wrapping_add(0x18), color);
    e.mem.set_u32(o.wrapping_add(0x1c), x87_quiet_bits(u));
    e.mem.set_u32(o.wrapping_add(0x20), x87_quiet_bits(v));
    let w_value = x87_load32(w);
    if w_value < 0.0 || w_value.is_nan() {
        return Ptr::new(o.wrapping_add(0x24));
    }
    e.mem.set_u32(o.wrapping_add(0x24), x87_quiet_bits(w));
    Ptr::new(o.wrapping_add(0x28))
}

// Translated from 00e69c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the behaviour-flag text at
/// +0x5F0 (0x20 bytes): emptied, then `" PURE"` (flag 0x10), `" MT"` (0x4), `" HWVP"`
/// (0x40), `" MIXVP"` (0x80) and `" SWVP"` (0x20) of the flags at +0x5CC appended in that
/// order (`strcat_s`). Answers the text's address.
pub fn fn_00e69c00(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    let t = this.addr();
    let text = t.wrapping_add(0x5f0);
    e.mem.set_u8(text, 0);
    for (flag, word) in [
        (0x10u8, PURE_TEXT_010ED5E4),
        (0x4, MT_TEXT_010ED5E0),
        (0x40, HWVP_TEXT_010ED5D8),
        (0x80, MIXVP_TEXT_010ED5D0),
        (0x20, SWVP_TEXT_010ED5C8),
    ] {
        if e.mem.u8(t.wrapping_add(BEHAVIOR_FLAGS)) & flag != 0 {
            e.call(STRCAT_S_00EC6BD2, &args![text, 0x20u32, word]);
        }
    }
    text
}

// Translated from 00e6b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the output matrix, a 3x3 rotation, a translation
/// and a scale): writes a 4x4 matrix row by row: each row's three rotation entries times
/// the scale (x87 `FMUL`, stored as floats; the last entry is a memory operand) followed
/// by the translation's entry, the translation entries then rewritten minus the world
/// offset at `011f474c` (x87 `FSUB`), and 1.0 at +0x3C. The words at +0x30 to +0x38 are
/// not written.
pub fn fn_00e6b8f0(e: &mut Engine, out: Ptr, rotation: Ptr, translation: Ptr, scale: f32) {
    let o = out.addr();
    let r = rotation.addr();
    let t = translation.addr();
    let s = x87_load32(scale.to_bits());
    for row in 0..3u32 {
        for column in 0..3u32 {
            let bits = e.mem.u32(r.wrapping_add(0xc * row + 4 * column));
            let entry = if row == 2 && column == 2 {
                x87_m32(bits)
            } else {
                x87_load32(bits)
            };
            let product = x87_store32(x87_mul(entry, s));
            e.mem
                .set_u32(o.wrapping_add(0x10 * row + 4 * column), product);
        }
        let copy = x87_quiet_bits(e.mem.u32(t.wrapping_add(4 * row)));
        e.mem.set_u32(o.wrapping_add(0x10 * row + 0xc), copy);
    }
    for row in 0..3u32 {
        let entry = x87_load32(e.mem.u32(t.wrapping_add(4 * row)));
        let offset = x87_m32(e.mem.u32(WORLD_OFFSET_011F474C + 4 * row));
        let moved = x87_store32(x87_sub(entry, offset));
        e.mem.set_u32(o.wrapping_add(0x10 * row + 0xc), moved);
    }
    e.mem.set_u32(o.wrapping_add(0x3c), x87_store32(1.0));
}

/// Releases an `NiFixedString` handle kept in a local: unless it is the empty string
/// (`0109b220`), its reference count (the word 8 bytes before the text) goes down by one
/// (`InterlockedDecrement`).
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING_0109B220) {
        e.call(
            IMPORT_INTERLOCKED_DOWN,
            &args![handle.wrapping_add(0xffff_fff8)],
        );
    }
}

/// One program cache of `00e6bdc0`: the profile text as an `NiFixedString` in `slot` (the
/// empty string for no profile), a new 0x48-byte `NiD3DGPUProgramCache` built with the
/// group's buffer `which` (`00ee8850`), and the slot released. Answers the cache (0 when
/// the allocation failed).
#[allow(clippy::too_many_arguments)]
fn program_cache(
    e: &mut Engine,
    group: u32,
    profile: u32,
    which: u32,
    slot: u32,
    words: [u32; 5],
    made: &mut bool,
) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x48u32]).u32();
    if block == 0 {
        return 0;
    }
    *made = true;
    let width = e.mem.u32(group.wrapping_add(8));
    let name = if profile == 0 {
        e.mem.u32(EMPTY_FIXED_STRING_0109B220)
    } else {
        e.call(FIXED_STRING_MAKE_00A5B690, &args![profile]).u32()
    };
    e.mem.set_u32(slot, name);
    let [second, third, fourth, fifth, sixth] = words;
    let buffer = e.call(GROUP_BUFFER_00EE8850, &args![group, which]).u32();
    e.call(
        PROGRAM_CACHE_CONSTRUCT,
        &args![block, buffer, sixth, which, slot, width, second, third, fifth, fourth],
    )
    .u32()
}

// Translated from 00e6bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x90, `thiscall`, six stack words: an
/// object `group` with a vtable and the five words passed on). Gets the device's vertex
/// shader profile (`00ee6e2e`); a sixth word of 0 is replaced by `00a8f750()`. Makes a
/// program cache (0x48 bytes, `NiD3DGPUProgramCache::NiD3DGPUProgramCache` `00e8e450`)
/// from the result of the group's `00ee8850(0)`, the sixth word, 0, the address of the
/// profile as an `NiFixedString` (made by `00a5b690`, or the empty string `0109b220` for
/// no profile), the group's word at +8 and the second, third, fifth and fourth words; the
/// fixed string is released when the allocation worked; the group's virtual +0x60 gets
/// the cache (0 when the allocation failed) and 0. The same again with the pixel shader
/// profile (`00ee6e28`), 2 for 0 and the fixed string kept in the first word's own stack
/// slot. Answers the second virtual call's result.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e6bdc0(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    group: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> u32 {
    let device = e.get(this, NiDX9Renderer::device);
    let profile = e.call(VERTEX_SHADER_PROFILE_00EE6E2E, &args![device]).u32();
    let sixth = if sixth == 0 {
        e.call(DEFAULT_WORD_00A8F750, &args![]).u32()
    } else {
        sixth
    };
    let words = [second, third, fourth, fifth, sixth];
    e.with_stack(8, |e, locals| {
        let slot = locals.addr();
        let first_slot = slot.wrapping_add(4);
        let mut made = false;
        let cache = program_cache(e, group, profile, 0, slot, words, &mut made);
        if made {
            let name = e.mem.u32(slot);
            release_fixed_string(e, name);
        }
        e.vcall(group, 0x60, &args![cache, 0u32]);
        let device = e.get(this, NiDX9Renderer::device);
        let profile = e.call(PIXEL_SHADER_PROFILE_00EE6E28, &args![device]).u32();
        let mut made = false;
        let cache = program_cache(e, group, profile, 2, first_slot, words, &mut made);
        if made {
            let name = e.mem.u32(first_slot);
            release_fixed_string(e, name);
        }
        e.vcall(group, 0x60, &args![cache, 2u32]).u32()
    })
}

// Translated from 00e6c690 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e6c690 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// Unnamed in the engine map (renderer vtable +0x9C, `thiscall`, no stack words): the
/// adapter description at +0x3C4 (0x200 bytes). Empties it, clears a 0x44C-byte
/// `D3DADAPTER_IDENTIFIER9` on the stack and asks Direct3D (`0126f0d8`, vtable +0x14,
/// `GetAdapterIdentifier`) for the adapter at +0x5C4 with flags 0. When that succeeds,
/// writes `"%s (%s-%s)"` (`NiSprintf`) with the identifier's description (+0x200), the
/// device type's text (+0x5C8: 1 `"HAL"`, 2 `"REF"`, 3 `"SW"`, else `"???"`) and the
/// behaviour-flag text (`00e69c00`). Answers the description's address.
pub fn fn_00e6c690(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    let t = this.addr();
    let description = t.wrapping_add(0x3c4);
    e.with_stack(0x44c, |e, identifier| {
        let identifier = identifier.addr();
        e.mem.set_u8(description, 0);
        e.call(MEMSET_ADDR, &args![identifier, 0u32, 0x44cu32]);
        let direct3d = e.mem.u32(DIRECT3D);
        let adapter = e.mem.u32(t.wrapping_add(ADAPTER));
        let result = e
            .vcall(direct3d, 0x14, &args![adapter, 0u32, identifier])
            .i32();
        if result >= 0 {
            let kind = match e.mem.u32(t.wrapping_add(DEVICE_TYPE)) {
                1 => HAL_TEXT_010ED5BC,
                2 => REF_TEXT_010ED5C0,
                3 => SW_TEXT_010ED5C4,
                _ => UNKNOWN_TEXT_010A93F4,
            };
            let flags = fn_00e69c00(e, this);
            e.call(
                NI_SPRINTF,
                &args![
                    description,
                    0x200u32,
                    DESCRIPTION_FORMAT_010ED954,
                    identifier.wrapping_add(0x200),
                    kind,
                    flags
                ],
            );
        }
        description
    })
}

// Translated from 00e6c760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xE4, `thiscall`, one stack word): stores
/// the word at `0126efc0`; `AL` = 1 (the rest of `EAX` is the word's).
pub fn fn_00e6c760(e: &mut Engine, _this: Ptr<NiDX9Renderer>, word: u32) -> bool {
    e.mem.set_u32(STORED_WORD_0126EFC0, word);
    true
}

// Translated from 00e6c770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xE8): the word at `0126efc0`.
pub fn fn_00e6c770(e: &mut Engine, _this: Ptr<NiDX9Renderer>) -> u32 {
    e.mem.u32(STORED_WORD_0126EFC0)
}

// Translated from 00e6d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xFC, `thiscall`, one stack word): for a
/// non-null object whose word at +0x1C is not 0, a jump to the object at +0x8AC's virtual
/// +0x18 (`00e88cc0` for its vtable `010ef718`, one stack word) with it, answered as it
/// is; otherwise answers the object (`EAX` still holds it).
pub fn fn_00e6d710(e: &mut Engine, this: Ptr<NiDX9Renderer>, object: u32) -> u32 {
    if object == 0 || e.mem.u32(object.wrapping_add(0x1c)) == 0 {
        return object;
    }
    let manager = e.mem.u32(this.addr().wrapping_add(0x8ac));
    e.vcall(manager, 0x18, &args![object]).u32()
}

// Translated from 00e6dd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x148, `thiscall`, one stack word, a
/// texture): a texture without renderer data (+0x24) gets it from `00e901a0(texture,
/// this)`, kept in the map at +0x8DC (`SetAt(texture, data)`, `00b57ea0`); false when that
/// gave none. Otherwise true.
pub fn fn_00e6dd00(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: u32) -> bool {
    if e.mem.u32(texture.wrapping_add(0x24)) != 0 {
        return true;
    }
    let data = e.call(TEXTURE_DATA_00E901A0, &args![texture, this]).u32();
    if data == 0 {
        return false;
    }
    e.call(
        MAP_SET_AT,
        &args![this.addr().wrapping_add(0x8dc), texture, data],
    );
    true
}

// Translated from 00e6dd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x14C, `thiscall`, one stack word, a
/// texture): true when the texture's renderer data (+0x24) exists and its virtual +0xAC
/// answers non-zero; otherwise new data from `00e904c0(texture)` is kept in the map at
/// +0x8EC (`SetAt`) and the answer is true, or false when there was none.
pub fn fn_00e6dd40(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: u32) -> bool {
    let data = e.mem.u32(texture.wrapping_add(0x24));
    if data != 0 && e.vcall(data, 0xac, &args![]).u32() != 0 {
        return true;
    }
    let data = e.call(TEXTURE_DATA_00E904C0, &args![texture]).u32();
    if data == 0 {
        return false;
    }
    e.call(
        MAP_SET_AT,
        &args![this.addr().wrapping_add(0x8ec), texture, data],
    );
    true
}

/// The Direct3D texture `00e6dd90` and `00e6de70` lock: the data's virtual +0x9C when the
/// renderer's byte at +0x6F5 is set, otherwise its virtual +0xB4.
fn locked_texture(e: &mut Engine, this: Ptr<NiDX9Renderer>, data: u32) -> u32 {
    let slot = if e.mem.u8(this.addr().wrapping_add(0x6f5)) != 0 {
        0x9c
    } else {
        0xb4
    };
    e.vcall(data, slot, &args![]).u32()
}

// Translated from 00e6dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9Renderer::LockDynamicTexture` (named by the message it reports; renderer vtable
/// +0x164, `thiscall`, two stack words: a
/// texture and where the pitch goes): locks a texture for writing. Clears the pitch; with
/// no texture, no renderer data (the texture's virtual +0xAC) or data whose virtual +0xB8
/// answers true, answers 0. Locks level 0 of the data's Direct3D texture (virtual +0x9C
/// with flags 0x2000 when the byte at +0x6F5 is set, otherwise virtual +0xB4 with flags 0;
/// none answers 0) with `LockRect` (vtable +0x4C); a failure is reported
/// (`NiXenonErrorString`, message `010ed96c`) and answers 0. Otherwise stores the pitch,
/// calls the data's virtual +0xBC with 1 and answers the bits.
pub fn ni_dx9_renderer_lock_dynamic_texture(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    texture: u32,
    pitch: Ptr,
) -> u32 {
    e.mem.set_u32(pitch.addr(), 0);
    if texture == 0 {
        return 0;
    }
    let data = e.vcall(texture, 0xac, &args![]).u32();
    if data == 0 || e.vcall(data, 0xb8, &args![]).bool() {
        return 0;
    }
    let flags = if e.mem.u8(this.addr().wrapping_add(0x6f5)) != 0 {
        0x2000u32
    } else {
        0
    };
    let surface = locked_texture(e, this, data);
    if surface == 0 {
        return 0;
    }
    e.with_stack(8, |e, locked| {
        let result = e
            .vcall(surface, 0x4c, &args![0u32, locked, 0u32, flags])
            .i32();
        if result < 0 {
            let text = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
            e.call(REPORT_00A29680, &args![LOCK_FAILED_010ED96C, text]);
            return 0;
        }
        let row_pitch = e.mem.u32(locked.addr());
        let bits = e.mem.u32(locked.addr().wrapping_add(4));
        e.mem.set_u32(pitch.addr(), row_pitch);
        e.vcall(data, 0xbc, &args![1u32]);
        bits
    })
}

// Translated from 00e6de70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9Renderer::UnLockDynamicTexture` (named by the messages it reports; renderer
/// vtable +0x168, `thiscall`, one stack word, a
/// texture): unlocks what `00e6dd90` locked. False with no texture, no renderer data or
/// data whose virtual +0xB8 answers false, or no Direct3D texture. `UnlockRect(0)` (vtable
/// +0x50); a failure is reported (message `010ed9e4`) and answers false. When the byte at
/// +0x6F5 is clear, the data's virtual +0x9C gives the texture to update (none answers
/// false) and the device's `UpdateTexture` (vtable +0x7C) copies the locked one into it; a
/// failure is reported (message `010ed9a4`) and answers false. Then the data's virtual
/// +0xBC with 0, and true.
pub fn ni_dx9_renderer_unlock_dynamic_texture(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    texture: u32,
) -> bool {
    if texture == 0 {
        return false;
    }
    let data = e.vcall(texture, 0xac, &args![]).u32();
    if data == 0 || !e.vcall(data, 0xb8, &args![]).bool() {
        return false;
    }
    let surface = locked_texture(e, this, data);
    if surface == 0 {
        return false;
    }
    let result = e.vcall(surface, 0x50, &args![0u32]).i32();
    if result < 0 {
        let text = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
        e.call(REPORT_00A29680, &args![UNLOCK_FAILED_010ED9E4, text]);
        return false;
    }
    if e.mem.u8(this.addr().wrapping_add(0x6f5)) == 0 {
        let target = e.vcall(data, 0x9c, &args![]).u32();
        if target == 0 {
            return false;
        }
        let device = e.get(this, NiDX9Renderer::device);
        let result = e.vcall(device, 0x7c, &args![surface, target]).i32();
        if result < 0 {
            let text = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
            e.call(REPORT_00A29680, &args![UPDATE_FAILED_010ED9A4, text]);
            return false;
        }
    }
    e.vcall(data, 0xbc, &args![0u32]);
    true
}

// Translated from 00e6df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x158, `thiscall`, one stack word, a
/// texture): clears its renderer data (+0x24) and jumps to `RemoveAt` (`00e91fd0`) on the
/// map at +0x8DC with it, answered as it is.
pub fn fn_00e6df60(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: u32) -> u32 {
    e.mem.set_u32(texture.wrapping_add(0x24), 0);
    e.call(
        MAP_REMOVE_AT,
        &args![this.addr().wrapping_add(0x8dc), texture],
    )
    .u32()
}

// Translated from 00e6dfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x160): `00e6df60` with the map at +0x8EC.
pub fn fn_00e6dfa0(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: u32) -> u32 {
    e.mem.set_u32(texture.wrapping_add(0x24), 0);
    e.call(
        MAP_REMOVE_AT,
        &args![this.addr().wrapping_add(0x8ec), texture],
    )
    .u32()
}

// Translated from 00e757c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x170): the byte at +0xAD0.
pub fn fn_00e757c0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0xad0))
}

// Translated from 00e757e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x11C, no stack words): a jump to the
/// object at +0x8B8's virtual +0x4C (`00e88750` for its vtable `010f088c`, no stack words,
/// a byte in `AL`), answered as it is.
pub fn fn_00e757e0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(object, 0x4c, &args![]).u32()
}

// Translated from 00e757f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x120, `thiscall`, one stack word): the
/// object at +0x8B8's virtual +0x50 with the word; `AL` = 1.
pub fn fn_00e757f0(e: &mut Engine, this: Ptr<NiDX9Renderer>, word: u32) -> bool {
    let object = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(object, 0x50, &args![word]);
    true
}

// Translated from 00e75820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xA4, `thiscall`, one stack word): stores
/// the float at +0x5E4 (an `FLD`/`FSTP` copy).
pub fn fn_00e75820(e: &mut Engine, this: Ptr<NiDX9Renderer>, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0x5e4),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 00e75830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xA8): the float at +0x5E4 (in `ST0`).
pub fn fn_00e75830(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> f64 {
    x87_load32(e.mem.u32(this.addr().wrapping_add(0x5e4)))
}

// Translated from 00e75a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xB8, `thiscall`, one stack word): stores
/// the word at +0x5E8.
pub fn fn_00e75a20(e: &mut Engine, this: Ptr<NiDX9Renderer>, word: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x5e8), word);
}

// Translated from 00e75a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xBC): the word at +0x5E8.
pub fn fn_00e75a30(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x5e8))
}

// Translated from 00e75a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x124, no stack words): a jump to the
/// object at +0x8B8's virtual +0x38 (`00e886b0` for its vtable `010f088c`, no stack
/// words, a float in `ST0`), answered as it is.
pub fn fn_00e75a40(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> f64 {
    let object = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(object, 0x38, &args![]).f64()
}

// Translated from 00e75a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x128, `thiscall`, one stack word): the
/// object at +0x8B8's virtual +0x40 with the float (an `FLD`/`FSTP` copy), answered as it
/// is.
pub fn fn_00e75a50(e: &mut Engine, this: Ptr<NiDX9Renderer>, value: f32) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(object, 0x40, &args![x87_quiet_bits(value.to_bits())])
        .u32()
}

/// The bits of each Direct3D format (`0126efd8`, a byte for each of the formats below 0x76;
/// filled by `00e6b990`).
const FORMAT_BITS_0126EFD8: u32 = 0x0126_efd8;
/// `NiRenderedCubeMap`'s `NiRTTI` (`012708e4`, constructed with that name by `00fc7950`).
const RENDERED_CUBE_MAP_RTTI_012708E4: u32 = 0x0127_08e4;
/// The `NiRTTI` `00e6f4e0` casts the group's first buffer to (`0126f59c`).
const BUFFER_RTTI_0126F59C: u32 = 0x0126_f59c;
/// The messages of `FastCopy` and `Copy`: their formats (`"NiDX9Renderer::FastCopy> Failed
/// - %s"`, `010edd38`; the `Copy` one at `010edd7c`) with `"Pixel formats do not match"`
/// (`010edd60`), `"No RendererData found"` (`010edcd8`) or `"NULL Surface found"`
/// (`010edcf0`), and the `StretchRect` failures (`010edd04`, `010edda0`).
const FAST_COPY_FAILED_010EDD38: u32 = 0x010e_dd38;
const COPY_FAILED_010EDD7C: u32 = 0x010e_dd7c;
const FORMATS_DIFFER_010EDD60: u32 = 0x010e_dd60;
const NO_RENDERER_DATA_010EDCD8: u32 = 0x010e_dcd8;
const NULL_SURFACE_010EDCF0: u32 = 0x010e_dcf0;
const FAST_COPY_STRETCH_FAILED_010EDD04: u32 = 0x010e_dd04;
const COPY_STRETCH_FAILED_010EDDA0: u32 = 0x010e_dda0;
/// `IDirect3DSurface9::GetDesc` (vtable +0x30), `LockRect` (+0x34) and `UnlockRect`
/// (+0x38); `IDirect3DDevice9::StretchRect` (+0x88).
const SURFACE_GET_DESC: u32 = 0x30;
const SURFACE_LOCK_RECT: u32 = 0x34;
const SURFACE_UNLOCK_RECT: u32 = 0x38;
const DEVICE_STRETCH_RECT: u32 = 0x88;

// Translated from 00e6f100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xC0, `thiscall`, one stack word: a render
/// target group): whether the renderer can use the group. False when the group has more
/// buffers (its virtual +0xA4) than the renderer takes (its own virtual +0xD8). Each buffer
/// (the group's virtual +0xC8 with its index) must exist and have a surface (+0x14), and
/// every earlier buffer's surface (`GetDesc`, which must succeed) must have no
/// multisampling and the width and height of this buffer's data (+0x8: +0x8, +0xC) and,
/// unless the renderer's virtual +0xDC answers true, the bits (the table at `0126efd8`, 0
/// for formats from 0x76 on) the buffer's pixel format gives (+0xC: byte +1). A depth
/// buffer (the group's virtual +0xCC) of the depth-stencil class (`0126f5ac`) must have a
/// surface (+0x14).
pub fn fn_00e6f100(e: &mut Engine, this: Ptr<NiDX9Renderer>, group: u32) -> bool {
    let count = e.vcall(group, 0xa4, &args![]).u32();
    let most = e.vcall(this.addr(), 0xd8, &args![]).u32();
    if count > most {
        return false;
    }
    let fits = e.with_stack(0x20, |e, description| {
        let d = description.addr();
        let mut i = 0u32;
        while i < count {
            let buffer = e.vcall(group, 0xc8, &args![i]).u32();
            if buffer == 0 || e.mem.u32(buffer.wrapping_add(0x14)) == 0 {
                return false;
            }
            let mut j = 0u32;
            while j < i {
                let earlier = e.vcall(group, 0xc8, &args![j]).u32();
                let surface = e.mem.u32(earlier.wrapping_add(0x14));
                if e.vcall(surface, SURFACE_GET_DESC, &args![d]).i32() < 0 {
                    return false;
                }
                // D3DSURFACE_DESC: Format +0, MultiSampleType +0x10, Width +0x18, Height +0x1C.
                if e.mem.u32(d.wrapping_add(0x10)) != 0 {
                    return false;
                }
                let data = e.mem.u32(buffer.wrapping_add(8));
                if e.mem.u32(data.wrapping_add(8)) != e.mem.u32(d.wrapping_add(0x18)) {
                    return false;
                }
                if e.mem.u32(data.wrapping_add(0xc)) != e.mem.u32(d.wrapping_add(0x1c)) {
                    return false;
                }
                if !e.vcall(this.addr(), 0xdc, &args![]).bool() {
                    let format = e.mem.u32(d);
                    let bits = if format < 0x76 {
                        e.mem.u8(FORMAT_BITS_0126EFD8.wrapping_add(format))
                    } else {
                        0
                    };
                    let pixel_format = e.mem.u32(buffer.wrapping_add(0xc));
                    if bits != e.mem.u8(pixel_format.wrapping_add(1)) {
                        return false;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        true
    });
    if !fits {
        return false;
    }
    let depth = e.vcall(group, 0xcc, &args![]).u32();
    if depth == 0 {
        return true;
    }
    let cast = fn_00e69b00(e, Ptr::new(depth), DEPTH_BUFFER_RTTI_0126F5AC);
    cast == 0 || e.mem.u32(cast.wrapping_add(0x14)) != 0
}

// Translated from 00e6f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x110, `thiscall`, two stack words: an
/// `NiRect` (left, right, top, bottom) or 0, and a render target group, 0 for the one at
/// +0x884): reads the group's first buffer back into a new `NiPixelData`. Answers 0 with
/// the byte at +0x6FC set. With a rectangle, the group's width (virtual +0x8C with 0, as an
/// unsigned `float`) must be above the left edge and its height (virtual +0x90) above the
/// bottom edge (x87 compares, `float` stores of the integers), and a right or bottom edge
/// past them is cut to the width or height; the region is right − left by bottom − top.
/// The first buffer (virtual +0xC8 with 0, cast to `0126f59c`; with none the code reads
/// address 0x14) has its surface (+0x14) described (`GetDesc`) and locked read-only
/// (`LockRect`, flags 0x10, the region as a `RECT`, or all of it); a failed lock answers 0.
/// The `NiPixelData` (0x74 bytes, `00a7c190` with the region's width and height, the
/// group's virtual +0x9C with 0, 1 and 1) gets each row copied from the lock (`memcpy` of
/// `m_puiWidth[0] * m_uiPixelStride` bytes from `m_pucPixels + m_puiOffsetInBytes[0]` on,
/// the source moving by the lock's pitch); the surface is unlocked and the pixel data
/// answered (0, unlocked, when it could not be made).
pub fn fn_00e6f4e0(e: &mut Engine, this: Ptr<NiDX9Renderer>, rect: Ptr, group: u32) -> u32 {
    if e.mem.u8(this.addr().wrapping_add(SKIP_END_SCENE)) != 0 {
        return 0;
    }
    let group = if group == 0 {
        e.get(this, NiDX9Renderer::object_884)
    } else {
        group
    };
    let width = e.vcall(group, 0x8c, &args![0u32]).u32();
    let height = e.vcall(group, 0x90, &args![0u32]).u32();
    // The frame: the lock's D3DLOCKED_RECT (+0x4: pitch, +0x8: bits), the RECT (+0xC) and
    // the D3DSURFACE_DESC (+0x1C).
    e.with_stack(0x3c, |e, frame| {
        let f = frame.addr();
        let locked = f + 4;
        let area = f + 0xc;
        let description = f + 0x1c;
        let mut region_width = width;
        let mut rows = height;
        let mut area_word = 0u32;
        if !rect.is_null() {
            let r = rect.addr();
            let left = e.mem.u32(r);
            let mut right = e.mem.u32(r.wrapping_add(4));
            let top = e.mem.u32(r.wrapping_add(8));
            let mut bottom = e.mem.u32(r.wrapping_add(0xc));
            e.mem.set_u32(area, left);
            e.mem.set_u32(area + 4, top);
            e.mem.set_u32(area + 8, right);
            e.mem.set_u32(area + 0xc, bottom);
            let width_value = x87_load32(unsigned_to_float(e, width));
            if width_value <= f64::from(left as i32) {
                return 0;
            }
            let bottom_value = x87_load32(x87_store32(f64::from(bottom as i32)));
            let height_value = x87_load32(unsigned_to_float(e, height));
            if height_value <= bottom_value {
                return 0;
            }
            if f64::from(right as i32) > width_value {
                right = width;
                e.mem.set_u32(area + 8, right);
            }
            if bottom_value > height_value {
                bottom = height;
                e.mem.set_u32(area + 0xc, bottom);
            }
            region_width = right.wrapping_sub(left);
            rows = bottom.wrapping_sub(top);
            area_word = area;
        }
        let first = e.vcall(group, 0xc8, &args![0u32]).u32();
        let buffer = if first != 0 {
            fn_00e69b00(e, Ptr::new(first), BUFFER_RTTI_0126F59C)
        } else {
            0
        };
        let surface = e.mem.u32(buffer.wrapping_add(0x14));
        e.vcall(surface, SURFACE_GET_DESC, &args![description]);
        let result = e
            .vcall(
                surface,
                SURFACE_LOCK_RECT,
                &args![locked, area_word, 0x10u32],
            )
            .i32();
        if result < 0 {
            return 0;
        }
        let block = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        let pixels = if block != 0 {
            let format = e.vcall(group, 0x9c, &args![0u32]).u32();
            e.call(
                PIXEL_DATA_CONSTRUCT_00A7C190,
                &args![block, region_width, rows, format, 1u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        if pixels == 0 {
            e.vcall(surface, SURFACE_UNLOCK_RECT, &args![]);
            return 0;
        }
        // NiPixelData (Xbox PDB): m_pucPixels +0x50, m_puiWidth +0x54,
        // m_puiOffsetInBytes +0x5C, m_uiPixelStride +0x64.
        let widths = e.mem.u32(pixels.wrapping_add(0x54));
        let offsets = e.mem.u32(pixels.wrapping_add(0x5c));
        let row_bytes = e
            .mem
            .u32(widths)
            .wrapping_mul(e.mem.u32(pixels.wrapping_add(0x64)));
        let mut destination = e
            .mem
            .u32(offsets)
            .wrapping_add(e.mem.u32(pixels.wrapping_add(0x50)));
        let mut source = e.mem.u32(locked + 4);
        let mut left_rows = rows;
        while left_rows != 0 {
            e.call(COPY_BYTES_00EC44D0, &args![destination, source, row_bytes]);
            source = source.wrapping_add(e.mem.u32(locked));
            destination = destination.wrapping_add(row_bytes);
            left_rows -= 1;
        }
        e.vcall(surface, SURFACE_UNLOCK_RECT, &args![]);
        pixels
    })
}

/// Reports a `FastCopy` or `Copy` failure: `00a29680(format, message)`.
fn copy_failed(e: &mut Engine, format: u32, message: u32) {
    e.call(REPORT_00A29680, &args![format, message]);
}

/// Writes an `NiRect` (left, right, top, bottom) at `rect` as a `RECT` (left, top, right,
/// bottom) at `at`.
fn put_rect(e: &mut Engine, at: u32, rect: u32) {
    let left = e.mem.u32(rect);
    let right = e.mem.u32(rect.wrapping_add(4));
    let top = e.mem.u32(rect.wrapping_add(8));
    let bottom = e.mem.u32(rect.wrapping_add(0xc));
    e.mem.set_u32(at, left);
    e.mem.set_u32(at + 4, top);
    e.mem.set_u32(at + 8, right);
    e.mem.set_u32(at + 0xc, bottom);
}

// Translated from 00e6f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9Renderer::FastCopy` (named by the messages it reports; renderer vtable +0x114,
/// `thiscall`, five stack words: the source and destination textures, an `NiRect` of the
/// source or 0, and the destination's left and top). False with the byte at +0x6FC set;
/// without renderer data (+0x10) on either, with pixel formats (data +0xC) that differ
/// (`NiPixelFormat::operator!=`) or without a surface (data +0x14) on either, the failure
/// is reported and false answered. The device's `StretchRect` copies the source rectangle
/// (as a `RECT`) to the same-sized one at the destination point, or the whole surface to
/// the whole surface without a rectangle, with filter 0; a failure is reported
/// (`NiXenonErrorString`) and answers false. Otherwise true.
#[allow(clippy::too_many_arguments)]
pub fn ni_dx9_renderer_fast_copy(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    source: u32,
    destination: u32,
    rect: Ptr,
    left: u32,
    top: u32,
) -> bool {
    if e.mem.u8(this.addr().wrapping_add(SKIP_END_SCENE)) != 0 {
        return false;
    }
    let from = e.mem.u32(source.wrapping_add(0x10));
    let to = e.mem.u32(destination.wrapping_add(0x10));
    if from == 0 || to == 0 {
        copy_failed(e, FAST_COPY_FAILED_010EDD38, NO_RENDERER_DATA_010EDCD8);
        return false;
    }
    let from_format = e.mem.u32(from.wrapping_add(0xc));
    let to_format = e.mem.u32(to.wrapping_add(0xc));
    if ni_pixel_format_operator_not_equal(e, Ptr::new(from_format), Ptr::new(to_format)) {
        copy_failed(e, FAST_COPY_FAILED_010EDD38, FORMATS_DIFFER_010EDD60);
        return false;
    }
    let from_surface = e.mem.u32(from.wrapping_add(0x14));
    let to_surface = e.mem.u32(to.wrapping_add(0x14));
    if from_surface == 0 || to_surface == 0 {
        copy_failed(e, FAST_COPY_FAILED_010EDD38, NULL_SURFACE_010EDCF0);
        return false;
    }
    let device = e.get(this, NiDX9Renderer::device);
    let result = e.with_stack(0x20, |e, rects| {
        let source_rect = rects.addr();
        let destination_rect = source_rect + 0x10;
        let (source_word, destination_word) = if !rect.is_null() {
            let r = rect.addr();
            put_rect(e, source_rect, r);
            let width = e
                .mem
                .u32(r.wrapping_add(4))
                .wrapping_sub(e.mem.u32(r))
                .wrapping_add(left);
            let height = e
                .mem
                .u32(r.wrapping_add(0xc))
                .wrapping_sub(e.mem.u32(r.wrapping_add(8)))
                .wrapping_add(top);
            e.mem.set_u32(destination_rect, left);
            e.mem.set_u32(destination_rect + 4, top);
            e.mem.set_u32(destination_rect + 8, width);
            e.mem.set_u32(destination_rect + 0xc, height);
            (source_rect, destination_rect)
        } else {
            (0, 0)
        };
        e.vcall(
            device,
            DEVICE_STRETCH_RECT,
            &args![
                from_surface,
                source_word,
                to_surface,
                destination_word,
                0u32
            ],
        )
        .i32()
    });
    if result < 0 {
        let text = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
        e.call(
            REPORT_00A29680,
            &args![FAST_COPY_STRETCH_FAILED_010EDD04, text],
        );
        return false;
    }
    true
}

// Translated from 00e6f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9Renderer::Copy` (named by the messages it reports; renderer vtable +0x118,
/// `thiscall`, five stack words: the source and destination textures, an `NiRect` of each
/// or 0 for the whole texture (0, width, 0, height from the texture's +0x8 and +0xC), and
/// the filter: 1 and 2 are passed as they are, anything else as 0). False with the byte at
/// +0x6FC set; without renderer data (+0x10) or a surface (data +0x14) on either, the
/// failure is reported and false answered. The device's `StretchRect` copies one
/// rectangle into the other; a failure is reported and answers false. Otherwise true.
#[allow(clippy::too_many_arguments)]
pub fn ni_dx9_renderer_copy(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    source: u32,
    destination: u32,
    source_rect: Ptr,
    destination_rect: Ptr,
    filter: u32,
) -> bool {
    if e.mem.u8(this.addr().wrapping_add(SKIP_END_SCENE)) != 0 {
        return false;
    }
    let to = e.mem.u32(destination.wrapping_add(0x10));
    let from = e.mem.u32(source.wrapping_add(0x10));
    if from == 0 || to == 0 {
        copy_failed(e, COPY_FAILED_010EDD7C, NO_RENDERER_DATA_010EDCD8);
        return false;
    }
    let from_surface = e.mem.u32(from.wrapping_add(0x14));
    let to_surface = e.mem.u32(to.wrapping_add(0x14));
    if from_surface == 0 || to_surface == 0 {
        copy_failed(e, COPY_FAILED_010EDD7C, NULL_SURFACE_010EDCF0);
        return false;
    }
    let whole = |e: &mut Engine, at: u32, texture: u32| {
        let width = e.mem.u32(texture.wrapping_add(8));
        let height = e.mem.u32(texture.wrapping_add(0xc));
        e.mem.set_u32(at, 0);
        e.mem.set_u32(at + 8, width);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 0xc, height);
    };
    let filter = match filter {
        1 => 1u32,
        2 => 2,
        _ => 0,
    };
    let device = e.get(this, NiDX9Renderer::device);
    let result = e.with_stack(0x20, |e, rects| {
        let from_rect = rects.addr();
        let to_rect = from_rect + 0x10;
        if source_rect.is_null() {
            whole(e, from_rect, source);
        } else {
            put_rect(e, from_rect, source_rect.addr());
        }
        if destination_rect.is_null() {
            whole(e, to_rect, destination);
        } else {
            put_rect(e, to_rect, destination_rect.addr());
        }
        e.vcall(
            device,
            DEVICE_STRETCH_RECT,
            &args![from_surface, from_rect, to_surface, to_rect, filter],
        )
        .i32()
    });
    if result < 0 {
        let text = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
        e.call(REPORT_00A29680, &args![COPY_STRETCH_FAILED_010EDDA0, text]);
        return false;
    }
    true
}

/// Releases the `NiPointer` at `slot` as the code does: when it holds an object, its
/// reference count (+4) goes down (`InterlockedDecrement`) and at 0 the object's virtual
/// +4 (`DeleteThis`) runs; the slot is then cleared.
fn release_pointer_slot(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object == 0 {
        return;
    }
    let left = e
        .call(IMPORT_INTERLOCKED_DOWN, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
    e.mem.set_u32(slot, 0);
}

// Translated from 00e6f9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x108, `thiscall`, one stack word, a
/// texture): drops the texture's renderer data (+0x24). With data whose virtual +0xA4
/// answers non-zero, the `NiPointer` at +0x10 of what the texture's virtual +0xA4 answers
/// is released, or, for an `NiRenderedCubeMap` (`012708e4` on the chain of its `GetRTTI`,
/// virtual +8), the one at +0x10 of each of its six faces (+0x4C to +0x60). The data
/// pointer is cleared and the data's virtual 0 (its deleting destructor) runs with 1.
/// Answers true (also without data).
pub fn fn_00e6f9b0(e: &mut Engine, _this: Ptr<NiDX9Renderer>, texture: u32) -> bool {
    let data = e.mem.u32(texture.wrapping_add(0x24));
    if data == 0 {
        return true;
    }
    if e.vcall(data, 0xa4, &args![]).u32() != 0 {
        let mut link = e.vcall(texture, 8, &args![]).u32();
        let mut cube = false;
        while link != 0 {
            if link == RENDERED_CUBE_MAP_RTTI_012708E4 {
                cube = true;
                break;
            }
            link = e.mem.u32(link.wrapping_add(4));
        }
        if cube {
            for face in 0..6u32 {
                let object = e.mem.u32(texture.wrapping_add(0x4c + 4 * face));
                release_pointer_slot(e, object.wrapping_add(0x10));
            }
        } else {
            let object = e.vcall(texture, 0xa4, &args![]).u32();
            release_pointer_slot(e, object.wrapping_add(0x10));
        }
    }
    e.mem.set_u32(texture.wrapping_add(0x24), 0);
    e.vcall(data, 0, &args![1u32]);
    true
}

/// The vtables of the two `NiTArray` instances whose scalar deleting destructors are
/// `00e72df0` (`010edaec`, RTTI `NiTArray<bool (*)(bool, void *), NiTMallocInterface<..>>`)
/// and `00e72e20` (`010edaf4`, `NiTArray<bool (*)(void *), NiTMallocInterface<..>>`).
const CALLBACK_ARRAY_VTABLE_010EDAEC: u32 = 0x010e_daec;
const CALLBACK_ARRAY_VTABLE_010EDAF4: u32 = 0x010e_daf4;
/// `NiFree` (`00aa10f0`, `cdecl`), the `NiTMallocInterface`'s deallocation.
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;
/// The memory manager object (`011f6238`) and its deallocate (`00aa4060`, `thiscall`, one
/// word).
const MEMORY_MANAGER_OBJECT_011F6238: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;
/// `00e88dc0` (`thiscall` on the geometry group manager at +0x8AC, six stack words).
const GROUP_MANAGER_00E88DC0: u32 = 0x00e8_8dc0;
/// `00e715a0` (`thiscall` on the renderer, six stack words): the unskinned draw.
const DRAW_00E715A0: u32 = 0x00e7_15a0;
/// `0043a010` (`thiscall` on a list's count, no stack words): a new list node.
const NEW_LIST_NODE_0043A010: u32 = 0x0043_a010;
/// `00e7acb0` (`cdecl`, one word) and `00e69d00` (`cdecl`, one word): the format queries of
/// `00e7aed0`.
const FORMAT_QUERY_00E7ACB0: u32 = 0x00e7_acb0;
/// `00e7af60` (`thiscall` on a mode, one stack word).
const MODE_MERGE_00E7AF60: u32 = 0x00e7_af60;

/// The scalar deleting destructor of an `NiTArray` with an `NiTMallocInterface`: the vtable,
/// `m_pBase` (+4) freed (`NiFree`), and with bit 0 of the flags `this` given back to the
/// memory manager. Answers `this`.
fn callback_array_delete(e: &mut Engine, this: Ptr, flags: u32, vtable: u32) -> Ptr {
    let t = this.addr();
    let base = e.mem.u32(t.wrapping_add(4));
    e.mem.set_u32(t, vtable);
    e.call(NI_FREE_00AA10F0, &args![base]);
    if flags & 1 != 0 {
        e.call(
            MEMORY_MANAGER_DEALLOCATE_00AA4060,
            &args![MEMORY_MANAGER_OBJECT_011F6238, this],
        );
    }
    this
}

// Translated from 00e72df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor (`thiscall`, `RET 4`: the
/// flags) of `NiTArray<bool (*)(bool, void *), NiTMallocInterface<..>>` (its vtable's RTTI).
pub fn fn_00e72df0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    callback_array_delete(e, this, flags, CALLBACK_ARRAY_VTABLE_010EDAEC)
}

// Translated from 00e72e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor (`thiscall`, `RET 4`: the
/// flags) of `NiTArray<bool (*)(void *), NiTMallocInterface<..>>` (its vtable's RTTI).
pub fn fn_00e72e20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    callback_array_delete(e, this, flags, CALLBACK_ARRAY_VTABLE_010EDAF4)
}

// Translated from 00e744b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1AC and +0x1B0, `thiscall`, one stack word:
/// a geometry). Nothing with the byte at +0x6FC set or when the geometry's data (+0xB8)
/// answers 0 from its virtual +0x90 (a 16-bit count). Copies the geometry's transform (13
/// words from +0x68) and its bound (+0x20, or `011f4288` without one: four words) into
/// locals, asks `00e72c20(geometry, 0)` whether it is skinned, gives the geometry group
/// manager (+0x8AC) `00e88dc0(+0x8B0, data, skin instance (+0xBC), that answer in the low
/// byte of the geometry's stack word, 0, 0)` and draws: skinned through `00e718a0(geometry,
/// data, skin instance, transform, bound)`, otherwise through `00e715a0(geometry, data,
/// skin instance, transform, bound, data's +0x34)`.
pub fn fn_00e744b0(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    if e.mem.u8(t.wrapping_add(SKIP_END_SCENE)) != 0 {
        return;
    }
    let g = geometry.addr();
    let data = e.mem.u32(g.wrapping_add(0xb8));
    if e.vcall(data, 0x90, &args![]).u16() == 0 {
        return;
    }
    let skin = e.mem.u32(g.wrapping_add(0xbc));
    e.with_stack(0x44, |e, frame| {
        let bound = frame.addr();
        let transform = bound + 0x10;
        for i in 0..13u32 {
            let word = e.mem.u32(g.wrapping_add(0x68 + 4 * i));
            e.mem.set_u32(transform + 4 * i, word);
        }
        let mut source = e.mem.u32(g.wrapping_add(0x20));
        if source == 0 {
            source = DEFAULT_BOUND;
        }
        for i in 0..4u32 {
            let word = e.mem.u32(source.wrapping_add(4 * i));
            e.mem.set_u32(bound + 4 * i, word);
        }
        let skinned = u8::from(fn_00e72c20(e, this, geometry, 0));
        // The answer goes into the low byte of the geometry's own stack word, which is
        // then pushed whole.
        let word = (g & 0xffff_ff00) | u32::from(skinned);
        let manager = e.mem.u32(t.wrapping_add(0x8ac));
        let group = e.mem.u32(t.wrapping_add(0x8b0));
        e.call(
            GROUP_MANAGER_00E88DC0,
            &args![manager, group, data, skin, word, 0u32, 0u32],
        );
        if skinned != 0 {
            fn_00e718a0(
                e,
                this,
                geometry,
                data,
                skin,
                Ptr::new(transform),
                Ptr::new(bound),
            );
            return;
        }
        let pass = e.mem.u32(data.wrapping_add(0x34));
        e.call(
            DRAW_00E715A0,
            &args![this, geometry, data, skin, transform, bound, pass],
        );
    });
}

// Translated from 00e752b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::RegisterD3DShader` (engine map): `thiscall`, one stack word (the
/// shader). Adds the shader at the head of the list at +0x90C (head), +0x910 (tail),
/// +0x914 (count): a new node from `0043a010` (on the count) gets the shader (+8), no
/// previous (+4) and the old head as next (+0); the old head's previous becomes the node
/// (an empty list's tail too), and the count grows by one.
pub fn ni_xenon_renderer_register_d3d_shader(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    shader: u32,
) {
    let t = this.addr();
    let count = t.wrapping_add(0x914);
    let node = e.call(NEW_LIST_NODE_0043A010, &args![count]).u32();
    e.mem.set_u32(node.wrapping_add(8), shader);
    e.mem.set_u32(node.wrapping_add(4), 0);
    let head = e.mem.u32(t.wrapping_add(0x90c));
    e.mem.set_u32(node, head);
    let head = e.mem.u32(t.wrapping_add(0x90c));
    if head != 0 {
        e.mem.set_u32(head.wrapping_add(4), node);
        let n = e.mem.u32(count).wrapping_add(1);
        e.mem.set_u32(count, n);
        e.mem.set_u32(t.wrapping_add(0x90c), node);
        return;
    }
    let n = e.mem.u32(count).wrapping_add(1);
    e.mem.set_u32(count, n);
    e.mem.set_u32(t.wrapping_add(0x910), node);
    e.mem.set_u32(t.wrapping_add(0x90c), node);
}

// Translated from 00e7aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words: a byte flag and a format):
/// walks the list at +0x134 (next at +0, data at +8); for an entry whose data is not null
/// and has the byte at +4 set (flag non-zero) or the byte at +5 set (flag 0), and whose
/// first word `00e7acb0` maps to the format, answers `00e69d00` of that word. Answers 0
/// when none does.
pub fn fn_00e7aed0(e: &mut Engine, this: Ptr, flag: u8, format: u32) -> u32 {
    let mut node = e.mem.u32(this.addr().wrapping_add(0x134));
    let byte = if flag != 0 { 4 } else { 5 };
    while node != 0 {
        let data = e.mem.u32(node.wrapping_add(8));
        node = e.mem.u32(node);
        if data != 0 && e.mem.u8(data.wrapping_add(byte)) != 0 {
            let word = e.mem.u32(data);
            if e.call(FORMAT_QUERY_00E7ACB0, &args![word]).u32() == format {
                let word = e.mem.u32(data);
                return e.call(FORMAT_00E69D00, &args![word]).u32();
            }
        }
    }
    0
}

// Translated from 00e7b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four stack words: three keys and a word by
/// address): goes through the 16-bit count (+0x45A) of entries at +0x454; for each
/// non-null entry whose +0xC, +0 and +4 equal the three keys, the word becomes
/// `00e7af60(entry, word)`. Answers the last such entry, or 0.
pub fn fn_00e7b190(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    word: Ptr,
) -> u32 {
    let t = this.addr();
    let mut found = 0u32;
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(t.wrapping_add(0x45a))) {
        let entries = e.mem.u32(t.wrapping_add(0x454));
        let entry = e.mem.u32(entries.wrapping_add(4 * i));
        if entry != 0
            && e.mem.u32(entry.wrapping_add(0xc)) == first
            && e.mem.u32(entry) == second
            && e.mem.u32(entry.wrapping_add(4)) == third
        {
            found = entry;
            let old = e.mem.u32(word.addr());
            let merged = e.call(MODE_MERGE_00E7AF60, &args![entry, old]).u32();
            e.mem.set_u32(word.addr(), merged);
        }
        i += 1;
    }
    found
}

/// The renderer's batch: the list of batched geometries at +0x620 (each node: the geometry
/// +0, a word +4, a word +8, the next node +0xC), the shader it is drawn with at +0x630
/// (an `NiPointer`) and the two words at +0x628 and +0x62C (set by `00e69900`).
const BATCH_LIST: u32 = 0x620;
const BATCH_SHADER: u32 = 0x630;
const BATCH_FIRST_WORD: u32 = 0x628;
const BATCH_SECOND_WORD: u32 = 0x62c;
/// The skin instance class `00e718a0` compares its instance's `GetRTTI` with (`011f49d8`).
const SKIN_INSTANCE_RTTI_011F49D8: u32 = 0x011f_49d8;

/// The bound a geometry is drawn with: its own (+0x20), or the default one (`011f4288`).
fn geometry_bound(e: &Engine, geometry: u32) -> u32 {
    let bound = e.mem.u32(geometry.wrapping_add(0x20));
    if bound == 0 {
        DEFAULT_BOUND
    } else {
        bound
    }
}

/// `00e711e0`'s end: the batch shader released, the two batch words cleared and the node
/// list given back (`00e69920`).
fn end_batch(e: &mut Engine, this: Ptr<NiDX9Renderer>) {
    let t = this.addr();
    release_pointer_slot(e, t.wrapping_add(BATCH_SHADER));
    e.mem.set_u32(t.wrapping_add(BATCH_FIRST_WORD), 0);
    e.mem.set_u32(t.wrapping_add(BATCH_SECOND_WORD), 0);
    fn_00e69920(e, this);
}

/// The `DrawIndexedPrimitive` strips of a pass (+0x48 strips; each strip's index count from
/// the 16-bit array at +0x4C minus 2, or +0x40 without the array), starting at index
/// 0 and moving by the count plus 2 each time: the primitive type, the base vertex
/// (+0x38), 0, the vertex count and the start.
fn draw_strips(e: &mut Engine, this: Ptr<NiDX9Renderer>, pass: u32, kind: u32, vertices: u32) {
    let mut start = 0u32;
    let mut strip = 0u32;
    while strip < e.mem.u32(pass.wrapping_add(0x48)) {
        let lengths = e.mem.u32(pass.wrapping_add(0x4c));
        let count = if lengths != 0 {
            u32::from(e.mem.u16(lengths.wrapping_add(2 * strip))).wrapping_sub(2)
        } else {
            e.mem.u32(pass.wrapping_add(0x40))
        };
        let device = e.get(this, NiDX9Renderer::device);
        let base = e.mem.u32(pass.wrapping_add(0x38));
        e.vcall(
            device,
            0x148,
            &args![kind, base, 0u32, vertices, start, count],
        );
        start = start.wrapping_add(count).wrapping_add(2);
        strip += 1;
    }
}

// Translated from 00e711e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1A0, no stack words): draws and ends the
/// batch. With no batched geometry or the byte at +0x6FC set it only ends the batch (the
/// shader at +0x630 released, the words +0x628 and +0x62C cleared, the nodes given back by
/// `00e69920`). Otherwise, with the first node's geometry, skin instance (+0xBC), word +8,
/// transform (+0x68) and bound: the shader's virtual +0x54 (a non-zero answer ends the
/// batch the same way), then +0x58; while its virtual +0x90 answers non-zero, +0x5C with
/// the first node and, for each node, +0x60, the pass from +0x6C, +0x68, the pass's strips
/// (`DrawIndexedPrimitive` with the pass's type +0x3C, base vertex +0x38 and vertex count
/// +0x18) or, without an index buffer (+0x34), one `DrawPrimitive` (type, start +0x38,
/// count +0x40), and +0x70; again while +0x94 answers non-zero. Then every batched
/// geometry's data (+0xB8) keeps only the top four bits of its 16-bit word at +0xE, the
/// shader's virtual +0x74 runs with the last geometry drawn, the batch ends and the object
/// at +0x8B8 gets its virtual +0xF8 with 0.
pub fn fn_00e711e0(e: &mut Engine, this: Ptr<NiDX9Renderer>) {
    let t = this.addr();
    let head = e.mem.u32(t.wrapping_add(BATCH_LIST));
    if head == 0 || e.mem.u8(t.wrapping_add(SKIP_END_SCENE)) != 0 {
        end_batch(e, this);
        return;
    }
    let shader_word = |e: &Engine| e.mem.u32(t.wrapping_add(BATCH_SHADER));
    let words = |e: &Engine| {
        (
            e.mem.u32(t.wrapping_add(BATCH_FIRST_WORD)),
            e.mem.u32(t.wrapping_add(BATCH_SECOND_WORD)),
        )
    };
    let mut pass = e.mem.u32(head.wrapping_add(8));
    let mut geometry = e.mem.u32(head);
    let mut skin = e.mem.u32(geometry.wrapping_add(0xbc));
    let mut last_bound = geometry_bound(e, geometry);
    let shader = shader_word(e);
    let (first, second) = words(e);
    let refused = e
        .vcall(
            shader,
            0x54,
            &args![
                geometry,
                skin,
                pass,
                first,
                second,
                geometry.wrapping_add(0x68),
                last_bound
            ],
        )
        .u32();
    if refused != 0 {
        end_batch(e, this);
        return;
    }
    let shader = shader_word(e);
    let (first, second) = words(e);
    e.vcall(
        shader,
        0x58,
        &args![
            geometry,
            skin,
            pass,
            first,
            second,
            geometry.wrapping_add(0x68),
            last_bound
        ],
    );
    let shader = shader_word(e);
    if e.vcall(shader, 0x90, &args![]).u32() != 0 {
        loop {
            let node = e.mem.u32(t.wrapping_add(BATCH_LIST));
            let first_geometry = e.mem.u32(node);
            let first_bound = geometry_bound(e, first_geometry);
            let first_word = e.mem.u32(node.wrapping_add(8));
            let shader = shader_word(e);
            let (first, second) = words(e);
            let first_skin = e.mem.u32(first_geometry.wrapping_add(0xbc));
            e.vcall(
                shader,
                0x5c,
                &args![
                    first_geometry,
                    first_skin,
                    first_word,
                    first,
                    second,
                    first_geometry.wrapping_add(0x68),
                    first_bound
                ],
            );
            let mut cursor = node;
            while cursor != 0 {
                geometry = e.mem.u32(cursor);
                let word = e.mem.u32(cursor.wrapping_add(8));
                let node_word = e.mem.u32(cursor.wrapping_add(4));
                skin = e.mem.u32(geometry.wrapping_add(0xbc));
                let bound = geometry_bound(e, geometry);
                last_bound = bound;
                let transform = geometry.wrapping_add(0x68);
                let shader = shader_word(e);
                let (first, second) = words(e);
                e.vcall(
                    shader,
                    0x60,
                    &args![geometry, skin, node_word, word, first, second, transform, bound],
                );
                let shader = shader_word(e);
                let first = e.mem.u32(t.wrapping_add(BATCH_FIRST_WORD));
                pass = e
                    .vcall(shader, 0x6c, &args![geometry, node_word, word, first])
                    .u32();
                let shader = shader_word(e);
                let (first, second) = words(e);
                e.vcall(
                    shader,
                    0x68,
                    &args![geometry, skin, node_word, pass, first, second, transform, bound],
                );
                if e.mem.u32(pass.wrapping_add(0x34)) != 0 {
                    let kind = e.mem.u32(pass.wrapping_add(0x3c));
                    let vertices = e.mem.u32(pass.wrapping_add(0x18));
                    draw_strips(e, this, pass, kind, vertices);
                } else {
                    let device = e.get(this, NiDX9Renderer::device);
                    let kind = e.mem.u32(pass.wrapping_add(0x3c));
                    let start = e.mem.u32(pass.wrapping_add(0x38));
                    let count = e.mem.u32(pass.wrapping_add(0x40));
                    e.vcall(device, 0x144, &args![kind, start, count]);
                }
                let shader = shader_word(e);
                let (first, second) = words(e);
                e.vcall(
                    shader,
                    0x70,
                    &args![geometry, skin, node_word, pass, first, second, transform, bound],
                );
                cursor = e.mem.u32(cursor.wrapping_add(0xc));
            }
            let shader = shader_word(e);
            if e.vcall(shader, 0x94, &args![]).u32() == 0 {
                break;
            }
        }
    }
    let mut node = e.mem.u32(t.wrapping_add(BATCH_LIST));
    while node != 0 {
        let batched = e.mem.u32(node);
        let data = e.mem.u32(batched.wrapping_add(0xb8));
        let flags = e.mem.u16(data.wrapping_add(0xe)) & 0xf000;
        e.mem.set_u16(data.wrapping_add(0xe), flags);
        node = e.mem.u32(node.wrapping_add(0xc));
    }
    let shader = shader_word(e);
    let (first, second) = words(e);
    e.vcall(
        shader,
        0x74,
        &args![
            geometry,
            skin,
            pass,
            first,
            second,
            geometry.wrapping_add(0x68),
            last_bound
        ],
    );
    end_batch(e, this);
    let library = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(library, 0xf8, &args![0u32]);
}

// Translated from 00e718a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, five stack words: a geometry, its data, its skin
/// instance, a transform and a bound): draws a skinned geometry partition by partition.
/// Nothing without a skin instance. The partitions are the skin partition's (instance
/// +0xC: count +8, array +0xC of 0x2C-byte partitions whose +0x28 is the first's stream);
/// the primitive type is 5 (a strip) when the first partition's 16-bit word at +0x22 is
/// non-zero, else 4. With the geometry's shader (+0xC0; for a null geometry the shader is
/// null and the call reads address 0): its virtual +0x54 (a non-zero answer stops
/// everything), then +0x58; with an
/// instance whose `GetRTTI` is `011f49d8` the per-partition flags are its +0x38 (4 bytes a
/// partition: byte 0 draws, byte 1 of the next joins it to this one). While the shader's
/// virtual +0x90 answers passes: +0x5C, the bound's centre and radius copied to +0x664 to
/// +0x670, then for each partition to draw: +0x60 on the first one, the pass from +0x6C
/// (none stops everything), +0x68 for the first and +0x64 for the later ones, its strips
/// (`DrawIndexedPrimitive` with the partition's 16-bit vertex count at +0x1C), the
/// following joined partitions drawn with their own passes (+0x6C) and +0x70; the data's
/// word at +0xE keeps only its top four bits, and again while +0x94 answers non-zero. Then
/// the shader's virtual +0x74 and the object at +0x8B8's virtual +0xF8 with 0. The words
/// at +0xC and +0x10 of the renderer go to every shader call.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e718a0(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    geometry: Ptr,
    data: u32,
    skin: u32,
    transform: Ptr,
    bound: Ptr,
) {
    let t = this.addr();
    if skin == 0 {
        return;
    }
    let geometry = geometry.addr();
    let transform = transform.addr();
    let bound = bound.addr();
    let partition = e.mem.u32(skin.wrapping_add(0xc));
    let partitions = e.mem.u32(partition.wrapping_add(0xc));
    let kind = if e.mem.u16(partitions.wrapping_add(0x22)) != 0 {
        5u32
    } else {
        4
    };
    let shader = if geometry != 0 {
        e.mem.u32(geometry.wrapping_add(0xc0))
    } else {
        // The code keeps 0 and calls through it (a read of address 0).
        0
    };
    let words = |e: &Engine| {
        (
            e.mem.u32(t.wrapping_add(0xc)),
            e.mem.u32(t.wrapping_add(0x10)),
        )
    };
    let mut stream = e.mem.u32(partitions.wrapping_add(0x28));
    let (first, second) = words(e);
    let refused = e
        .vcall(
            shader,
            0x54,
            &args![geometry, skin, stream, first, second, transform, bound],
        )
        .u32();
    if refused != 0 {
        return;
    }
    let (first, second) = words(e);
    e.vcall(
        shader,
        0x58,
        &args![geometry, skin, stream, first, second, transform, bound],
    );
    let count = e.mem.u32(partition.wrapping_add(8));
    let passes = e.vcall(shader, 0x90, &args![]).u32();
    let mut flags = 0u32;
    if e.vcall(skin, 8, &args![]).u32() == SKIN_INSTANCE_RTTI_011F49D8 {
        flags = e.mem.u32(skin.wrapping_add(0x38));
    }
    if passes != 0 {
        loop {
            let (first, second) = words(e);
            e.vcall(
                shader,
                0x5c,
                &args![geometry, skin, stream, first, second, transform, bound],
            );
            let centre_x = e.mem.u32(bound);
            let radius = x87_quiet_bits(e.mem.u32(bound.wrapping_add(0xc)));
            e.mem.set_u32(t.wrapping_add(0x664), centre_x);
            let centre_y = e.mem.u32(bound.wrapping_add(4));
            e.mem.set_u32(t.wrapping_add(0x668), centre_y);
            let centre_z = e.mem.u32(bound.wrapping_add(8));
            e.mem.set_u32(t.wrapping_add(0x670), radius);
            e.mem.set_u32(t.wrapping_add(0x66c), centre_z);
            let mut first_partition = true;
            let mut i = 0u32;
            while i < count {
                if flags != 0 && e.mem.u8(flags.wrapping_add(4 * i)) == 0 {
                    i += 1;
                    continue;
                }
                let mut element = partitions.wrapping_add(i.wrapping_mul(0x2c));
                stream = e.mem.u32(element.wrapping_add(0x28));
                let slot = if first_partition {
                    let (first, second) = words(e);
                    first_partition = false;
                    e.vcall(
                        shader,
                        0x60,
                        &args![geometry, skin, element, stream, first, second, transform, bound],
                    );
                    0x68
                } else {
                    0x64
                };
                let first = e.mem.u32(t.wrapping_add(0xc));
                stream = e
                    .vcall(shader, 0x6c, &args![geometry, element, stream, first])
                    .u32();
                if stream == 0 {
                    return;
                }
                let (first, second) = words(e);
                e.vcall(
                    shader,
                    slot,
                    &args![geometry, skin, element, stream, first, second, transform, bound],
                );
                let vertices = u32::from(e.mem.u16(element.wrapping_add(0x1c)));
                draw_strips(e, this, stream, kind, vertices);
                if flags != 0 && i.wrapping_add(1) < count {
                    let mut next = i.wrapping_add(1);
                    let mut at = element;
                    loop {
                        if e.mem.u8(flags.wrapping_add(4 * i + 5)) != 0 {
                            break;
                        }
                        next = next.wrapping_add(1);
                        i = i.wrapping_add(1);
                        at = at.wrapping_add(0x2c);
                        if e.mem.u8(flags.wrapping_add(4 * i)) != 0 {
                            let word = e.mem.u32(at.wrapping_add(0x28));
                            let first = e.mem.u32(t.wrapping_add(0xc));
                            element = at;
                            stream = e
                                .vcall(shader, 0x6c, &args![geometry, at, word, first])
                                .u32();
                            if stream == 0 {
                                return;
                            }
                            let vertices = u32::from(e.mem.u16(at.wrapping_add(0x1c)));
                            draw_strips(e, this, stream, kind, vertices);
                        }
                        if next >= count {
                            break;
                        }
                    }
                }
                let (first, second) = words(e);
                e.vcall(
                    shader,
                    0x70,
                    &args![geometry, skin, element, stream, first, second, transform, bound],
                );
                i += 1;
            }
            let flags_word = e.mem.u16(data.wrapping_add(0xe)) & 0xf000;
            e.mem.set_u16(data.wrapping_add(0xe), flags_word);
            if e.vcall(shader, 0x94, &args![]).u32() == 0 {
                break;
            }
        }
    }
    let (first, second) = words(e);
    e.vcall(
        shader,
        0x74,
        &args![geometry, skin, stream, first, second, transform, bound],
    );
    let library = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(library, 0xf8, &args![0u32]);
}

/// The calls `00e70720` and `00e70be0` make on the stream objects: `00e8ef10` (`thiscall`,
/// one word), `00e8f040` and `00e8f010` (`thiscall`, one word: the vertex declaration or
/// the FVF), and on the objects at +0x8BC and +0x8C0 `00e8bfa0` (two words), `00e8bff0`
/// (four words: lock a vertex buffer), `00e8c0d0` (one word: unlock it), `00e8fc50` (four
/// words: make an index buffer) and `00e8f520` (five words).
const STREAM_00E8EF10: u32 = 0x00e8_ef10;
const STREAM_DECLARATION_00E8F040: u32 = 0x00e8_f040;
const STREAM_FVF_00E8F010: u32 = 0x00e8_f010;
const BUFFERS_00E8BFA0: u32 = 0x00e8_bfa0;
const BUFFERS_LOCK_00E8BFF0: u32 = 0x00e8_bff0;
const BUFFERS_UNLOCK_00E8C0D0: u32 = 0x00e8_c0d0;
const INDEX_BUFFER_00E8FC50: u32 = 0x00e8_fc50;
const INDEX_BUFFER_00E8F520: u32 = 0x00e8_f520;
/// `00c25db0` (`thiscall` on the strip data, one word: a strip's vertex count, 16 bits)
/// and `00c25dd0` (`thiscall`, no stack words: the 16-bit word at +0x80).
const STRIP_LENGTH_00C25DB0: u32 = 0x00c2_5db0;
const STRIP_WORD_00C25DD0: u32 = 0x00c2_5dd0;
/// `00e6d3a0` (`thiscall`, five stack words): writes the strips' vertices and indices.
const STRIP_VERTICES_00E6D3A0: u32 = 0x00e6_d3a0;
/// `255.0` (`double`, `0101e568`), the colour channel scale.
const COLOR_SCALE_0101E568: u32 = 0x0101_e568;
/// `"... Lock failed"` reported by `00e70720` (`010eddd0`).
const INDEX_LOCK_FAILED_010EDDD0: u32 = 0x010e_ddd0;

/// The shader calls both functions make with the renderer's word at +0xC: virtuals +0x78,
/// +0x7C, +0x80 and +0x84 with it, +0x88 with it and 1.
fn shader_set_up(e: &mut Engine, this: Ptr<NiDX9Renderer>, shader: u32) {
    for slot in [0x78u32, 0x7c, 0x80, 0x84] {
        let word = e.mem.u32(this.addr().wrapping_add(0xc));
        e.vcall(shader, slot, &args![word]);
    }
    let word = e.mem.u32(this.addr().wrapping_add(0xc));
    e.vcall(shader, 0x88, &args![word, 1u32]);
}

/// The renderer's copy of the drawn bound (+0x664 to +0x670): the centre's three words and
/// the radius (an `FLD`/`FSTP` copy).
fn copy_drawn_bound(e: &mut Engine, this: Ptr<NiDX9Renderer>, bound: [u32; 4]) {
    let t = this.addr();
    e.mem
        .set_u32(t.wrapping_add(0x670), x87_quiet_bits(bound[3]));
    e.mem.set_u32(t.wrapping_add(0x664), bound[0]);
    e.mem.set_u32(t.wrapping_add(0x668), bound[1]);
    e.mem.set_u32(t.wrapping_add(0x66c), bound[2]);
}

/// The four words of a geometry's bound (+0x20, or the default `011f4288`).
fn bound_words(e: &Engine, geometry: u32) -> [u32; 4] {
    let bound = geometry_bound(e, geometry);
    [
        e.mem.u32(bound),
        e.mem.u32(bound.wrapping_add(4)),
        e.mem.u32(bound.wrapping_add(8)),
        e.mem.u32(bound.wrapping_add(0xc)),
    ]
}

/// One colour channel as the code packs it: the float times 255.0 (`double`), stored as a
/// float, loaded again and stored as an integer (`FISTP`, rounding to nearest).
fn packed_channel(e: &Engine, at: u32) -> u32 {
    let scale = f64::from_bits(e.mem.u64(COLOR_SCALE_0101E568));
    let scaled = x87_store32(x87_mul(x87_load32(e.mem.u32(at)), scale));
    crate::x87::fistp(x87_load32(scaled)) as u32
}

/// A colour of four floats (r, g, b, a) packed as `((a << 8 | r) << 8 | g) << 8 | b` with
/// whole words (the channels are not masked), the channels taken in the order a, r, g, b.
fn packed_color(e: &Engine, color: u32) -> u32 {
    let alpha = packed_channel(e, color.wrapping_add(0xc));
    let red = packed_channel(e, color);
    let green = packed_channel(e, color.wrapping_add(4));
    let blue = packed_channel(e, color.wrapping_add(8));
    (((alpha << 8 | red) << 8 | green) << 8) | blue
}

/// `count` `FLD`/`FSTP` float copies from `from` to `to`.
fn copy_floats(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let bits = x87_quiet_bits(e.mem.u32(from.wrapping_add(4 * i)));
        e.mem.set_u32(to.wrapping_add(4 * i), bits);
    }
}

// Translated from 00e70720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1C0, `thiscall`, one stack word: a strip
/// particle geometry). Nothing with the byte at +0x6FC set, or when the property state at
/// +0xC has bit 0 of its first property's byte +0x18 set and its third property's float at
/// +0x3C is not above 0 (x87 `FCOMP` with 0; a NaN goes on). Counts the strips' vertices
/// (`00c25db0` for each of the data's 16-bit strip count at +0x50; strips of at least two
/// vertices give 4 vertices each and 2 indices a vertex plus 2 between strips); nothing
/// without any or without a shader (+0xC0). Prepares the stream object at the data's +0x34
/// (`00e8ef10(1)`, the counts at +0x40, +0x44 (indices − 2), +0x18, +0x1C, one strip at
/// +0x48, +0x4C and +0x50 cleared, +0x4 = 0x1C00000), copies the bound to +0x664 and the
/// object at +0x8B8 gets its virtual +0x2C with 0 and +0x30 with the transform. With the
/// data's word at +0x64 set the vertex stride is 0x28 and the declaration `00e69790`'s,
/// otherwise 0x24 and `00e696a0`'s (each given to the object at +0x8B8's virtual +0xAC with
/// 0, then to `00e8f040`); triangle strips (+0x3C = 5). The shader's virtual +0x90 and its
/// set-up calls (+0x78 to +0x88), `00e8bfa0`, a vertex buffer locked (`00e8bff0` with 10 or
/// 9 times 4 bytes a vertex), an index buffer made when the stream has none (`00e8fc50`,
/// the count from `00c25dd0` as the code calls it), locked (vtable +0x2C; a failure is
/// reported, the buffer unlocked and released), filled with the vertex buffer by
/// `00e6d3a0`, both unlocked, and with at least one vertex written the stream counts set
/// again and one `DrawIndexedPrimitive` after `SetStreamSource` and `SetIndices`.
pub fn fn_00e70720(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    if e.mem.u8(t.wrapping_add(SKIP_END_SCENE)) != 0 {
        return;
    }
    let state = e.mem.u32(t.wrapping_add(0xc));
    let first_property = e.mem.u32(state);
    if e.mem.u8(first_property.wrapping_add(0x18)) & 1 != 0 {
        let third = e.mem.u32(state.wrapping_add(8));
        let value = x87_m32(e.mem.u32(third.wrapping_add(0x3c)));
        // FLDZ; FCOMP: C0 is set when 0 < value, and for a NaN.
        if !(0.0 < value || value.is_nan()) {
            return;
        }
    }
    let g = geometry.addr();
    let data = e.mem.u32(g.wrapping_add(0xb8));
    e.with_stack(0x34, |e, transform| {
        let transform = transform.addr();
        for i in 0..13u32 {
            let word = e.mem.u32(g.wrapping_add(0x68 + 4 * i));
            e.mem.set_u32(transform + 4 * i, word);
        }
        let bound = bound_words(e, g);
        let strips = u32::from(e.mem.u16(data.wrapping_add(0x50)));
        let wide = e.mem.u32(data.wrapping_add(0x64));
        let mut vertices = 0u32;
        let mut indices = 0u32;
        let mut joined = false;
        for strip in 0..strips {
            let length = e.call(STRIP_LENGTH_00C25DB0, &args![data, strip]).u16();
            if length >= 2 {
                let quads = u32::from(length) * 4;
                vertices = vertices.wrapping_add(quads);
                if joined {
                    indices = indices.wrapping_add(2);
                } else {
                    joined = true;
                }
                indices = indices.wrapping_add(quads).wrapping_add(2);
            }
        }
        let shader = e.mem.u32(g.wrapping_add(0xc0));
        if vertices == 0 || shader == 0 {
            return;
        }
        let manager = e.mem.u32(t.wrapping_add(0x8ac));
        let group = e.mem.u32(t.wrapping_add(0x8b4));
        e.call(
            GROUP_MANAGER_00E88DC0,
            &args![manager, group, data, 0u32, 0u32, 0u32, 0u32],
        );
        let stream = e.mem.u32(data.wrapping_add(0x34));
        e.call(STREAM_00E8EF10, &args![stream, 1u32]);
        let primitives = indices.wrapping_sub(2);
        e.mem.set_u32(stream.wrapping_add(0x40), primitives);
        e.mem.set_u32(stream.wrapping_add(0x44), primitives);
        e.mem.set_u32(stream.wrapping_add(0x18), vertices);
        e.mem.set_u32(stream.wrapping_add(0x1c), vertices);
        e.mem.set_u32(stream.wrapping_add(0x50), 0);
        e.mem.set_u32(stream.wrapping_add(0x4c), 0);
        e.mem.set_u32(stream.wrapping_add(0x48), 1);
        copy_drawn_bound(e, this, bound);
        e.mem.set_u32(stream.wrapping_add(4), 0x01c0_0000);
        let library = e.get(this, NiDX9Renderer::object_8b8);
        e.vcall(library, 0x2c, &args![0u32]);
        let library = e.get(this, NiDX9Renderer::object_8b8);
        e.vcall(library, 0x30, &args![transform]);
        let stride = if wide != 0 { 0x28u32 } else { 0x24 };
        if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
            let strides = e.mem.u32(stream.wrapping_add(0x24));
            e.mem.set_u32(strides, stride);
        }
        let make_declaration = |e: &mut Engine| {
            if wide != 0 {
                fn_00e69790(e, this)
            } else {
                fn_00e696a0(e, this)
            }
        };
        let library = e.get(this, NiDX9Renderer::object_8b8);
        let declaration = make_declaration(e);
        e.vcall(library, 0xac, &args![declaration, 0u32]);
        let declaration = make_declaration(e);
        e.call(STREAM_DECLARATION_00E8F040, &args![stream, declaration]);
        e.mem.set_u32(stream.wrapping_add(0x3c), 5);
        e.vcall(shader, 0x90, &args![]);
        shader_set_up(e, this, shader);
        let buffers = e.mem.u32(t.wrapping_add(0x8bc));
        e.call(BUFFERS_00E8BFA0, &args![buffers, stream, 0u32]);
        let list = if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
            let lists = e.mem.u32(stream.wrapping_add(0x28));
            e.mem.u32(lists)
        } else {
            0
        };
        let buffer = e.mem.u32(list.wrapping_add(8));
        if buffer == 0 {
            return;
        }
        let per_vertex = if wide != 0 {
            vertices.wrapping_mul(10)
        } else {
            vertices.wrapping_mul(9)
        };
        let bytes = per_vertex.wrapping_mul(4);
        let offset = e.mem.u32(list.wrapping_add(0xc));
        let flags = e.mem.u32(list.wrapping_add(0x10));
        let buffers = e.mem.u32(t.wrapping_add(0x8bc));
        let written_to = e
            .call(
                BUFFERS_LOCK_00E8BFF0,
                &args![buffers, buffer, offset, bytes, flags],
            )
            .u32();
        if written_to == 0 {
            return;
        }
        let index_bytes = indices.wrapping_mul(2);
        let mut index_buffer = e.mem.u32(stream.wrapping_add(0x34));
        if index_buffer == 0 {
            let mut count = 0u32;
            let mut joined = false;
            let mut left = u32::from(e.mem.u16(data.wrapping_add(8)));
            while left != 0 {
                let length = e.call(STRIP_WORD_00C25DD0, &args![data]).u16();
                if length >= 2 {
                    if joined {
                        count = count.wrapping_add(2);
                    } else {
                        joined = true;
                    }
                    count = count.wrapping_add(u32::from(length) * 4).wrapping_add(2);
                }
                left -= 1;
            }
            let not_ex = u32::from(e.mem.u8(DIRECT3D_IS_EX) == 0);
            let size = count.wrapping_mul(2);
            let maker = e.mem.u32(t.wrapping_add(0x8c0));
            index_buffer = e
                .call(
                    INDEX_BUFFER_00E8FC50,
                    &args![maker, size, 0u32, 0x65u32, not_ex],
                )
                .u32();
            e.mem.set_u32(stream.wrapping_add(0x34), index_buffer);
            e.mem.set_u32(stream.wrapping_add(0x2c), count);
            e.mem.set_u32(stream.wrapping_add(0x30), size);
        }
        e.with_stack(0xc, |e, counts| {
            let written = counts.addr();
            let index_count = written + 4;
            let locked = written + 8;
            if index_buffer == 0 {
                let buffers = e.mem.u32(t.wrapping_add(0x8bc));
                let buffer = e.mem.u32(list.wrapping_add(8));
                e.call(BUFFERS_UNLOCK_00E8C0D0, &args![buffers, buffer]);
                return;
            }
            let result = e
                .vcall(index_buffer, 0x2c, &args![0u32, index_bytes, locked, 0u32])
                .i32();
            if result < 0 {
                e.call(REPORT_00A29680, &args![INDEX_LOCK_FAILED_010EDDD0]);
                e.vcall(index_buffer, 0x30, &args![]);
                e.vcall(index_buffer, 8, &args![]);
                return;
            }
            let indices_at = e.mem.u32(locked);
            e.call(
                STRIP_VERTICES_00E6D3A0,
                &args![this, written_to, indices_at, data, written, index_count],
            );
            let buffers = e.mem.u32(t.wrapping_add(0x8bc));
            let buffer = e.mem.u32(list.wrapping_add(8));
            e.call(BUFFERS_UNLOCK_00E8C0D0, &args![buffers, buffer]);
            e.vcall(index_buffer, 0x30, &args![]);
            let drawn = e.mem.u32(written);
            if drawn < 1 {
                return;
            }
            e.mem.set_u32(stream.wrapping_add(0x18), drawn);
            e.mem.set_u32(stream.wrapping_add(0x1c), drawn);
            let primitives = e.mem.u32(index_count).wrapping_sub(2);
            e.mem.set_u32(stream.wrapping_add(0x40), primitives);
            e.mem.set_u32(stream.wrapping_add(0x44), primitives);
            e.mem.set_u32(stream.wrapping_add(0x50), 0);
            e.mem.set_u32(stream.wrapping_add(0x4c), 0);
            e.mem.set_u32(stream.wrapping_add(0x48), 1);
            let stride = if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
                let strides = e.mem.u32(stream.wrapping_add(0x24));
                e.mem.u32(strides)
            } else {
                0
            };
            let device = e.get(this, NiDX9Renderer::device);
            let buffer = e.mem.u32(list.wrapping_add(8));
            e.vcall(device, 0x190, &args![0u32, buffer, 0u32, stride]);
            let device = e.get(this, NiDX9Renderer::device);
            e.vcall(device, 0x1a0, &args![index_buffer]);
            let device = e.get(this, NiDX9Renderer::device);
            let kind = e.mem.u32(stream.wrapping_add(0x3c));
            let base = e.mem.u32(stream.wrapping_add(0x38));
            let count = e.mem.u32(stream.wrapping_add(0x18));
            let primitives = e.mem.u32(stream.wrapping_add(0x40));
            e.vcall(
                device,
                0x148,
                &args![kind, base, 0u32, count, 0u32, primitives],
            );
        });
    });
}

// Translated from 00e70be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1C4, `thiscall`, one stack word: a lines
/// geometry). Nothing with the byte at +0x6FC set or for data (+0xB8) with no vertices (its
/// 16-bit count at +8). Gives the geometry group manager `00e88dc0(+0x8B4, data, 0, 0, 0,
/// 0)`, copies the bound to +0x664, and gets the lines' index buffer from the object at
/// +0x8C0 (`00e8f520(count, data's +0x40, &index count, 0, not Ex)`; nothing without one or
/// without indices). The FVF is XYZ (stride 0xC), with the texture coordinates (+0x2C)
/// 0x102 (0x14), plus 0x40 (4 more) with the colours (+0x28). The stream object at the
/// data's +0x34 is prepared (`00e8ef10(1)`, the counts at +0x18 and +0x1C, the FVF through
/// `00e8f010`, the stride), `00e8bfa0`, the object at +0x8B8's virtual +0x9C with the
/// stream's +0xC and 0, and the vertex buffer locked (`00e8bff0`; nothing on failure) and
/// filled: each vertex's position (three floats from +0x20), its colour packed (four floats
/// a vertex) and its texture coordinates (two floats), as the FVF has them. Then the buffer
/// is unlocked, the shader (+0xC0) set up (+0x78 to +0x88), `SetStreamSource` and
/// `SetIndices`, and while the shader's passes (virtual +0x90, then +0x94) go on, one
/// `DrawIndexedPrimitive` of a line list with half the index count; finally the shader's
/// virtual +0x8C with the renderer's word at +0xC.
pub fn fn_00e70be0(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    if e.mem.u8(t.wrapping_add(SKIP_END_SCENE)) != 0 {
        return;
    }
    let g = geometry.addr();
    let data = e.mem.u32(g.wrapping_add(0xb8));
    let count = u32::from(e.mem.u16(data.wrapping_add(8)));
    if count == 0 {
        return;
    }
    let bound = bound_words(e, g);
    let manager = e.mem.u32(t.wrapping_add(0x8ac));
    let group = e.mem.u32(t.wrapping_add(0x8b4));
    e.call(
        GROUP_MANAGER_00E88DC0,
        &args![manager, group, data, 0u32, 0u32, 0u32, 0u32],
    );
    copy_drawn_bound(e, this, bound);
    let lines = e.mem.u32(data.wrapping_add(0x40));
    let mut positions = e.mem.u32(data.wrapping_add(0x20));
    let mut coordinates = e.mem.u32(data.wrapping_add(0x2c));
    let mut colors = e.mem.u32(data.wrapping_add(0x28));
    let not_ex = u32::from(e.mem.u8(DIRECT3D_IS_EX) == 0);
    e.with_stack(4, |e, index_count| {
        let index_count = index_count.addr();
        let maker = e.mem.u32(t.wrapping_add(0x8c0));
        let index_buffer = e
            .call(
                INDEX_BUFFER_00E8F520,
                &args![maker, count, lines, index_count, 0u32, not_ex],
            )
            .u32();
        if index_buffer == 0 || e.mem.u32(index_count) == 0 {
            return;
        }
        let mut fvf = 2u32;
        let mut stride = 0xcu32;
        if coordinates != 0 {
            fvf = 0x102;
            stride = 0x14;
        }
        if colors != 0 {
            fvf |= 0x40;
            stride += 4;
        }
        let shader = e.mem.u32(g.wrapping_add(0xc0));
        let stream = e.mem.u32(data.wrapping_add(0x34));
        e.call(STREAM_00E8EF10, &args![stream, 1u32]);
        e.mem.set_u32(stream.wrapping_add(0x18), count);
        e.mem.set_u32(stream.wrapping_add(0x1c), count);
        e.call(STREAM_FVF_00E8F010, &args![stream, fvf]);
        if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
            let strides = e.mem.u32(stream.wrapping_add(0x24));
            e.mem.set_u32(strides, stride);
        }
        let buffers = e.mem.u32(t.wrapping_add(0x8bc));
        e.call(BUFFERS_00E8BFA0, &args![buffers, stream, 0u32]);
        let list = if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
            let lists = e.mem.u32(stream.wrapping_add(0x28));
            e.mem.u32(lists)
        } else {
            0
        };
        let library = e.get(this, NiDX9Renderer::object_8b8);
        let key = e.mem.u32(stream.wrapping_add(0xc));
        e.vcall(library, 0x9c, &args![key, 0u32]);
        let buffer = e.mem.u32(list.wrapping_add(8));
        if buffer == 0 {
            return;
        }
        let offset = e.mem.u32(list.wrapping_add(0xc));
        let size = e.mem.u32(list.wrapping_add(0x14));
        let flags = e.mem.u32(list.wrapping_add(0x10));
        let buffers = e.mem.u32(t.wrapping_add(0x8bc));
        let mut out = e
            .call(
                BUFFERS_LOCK_00E8BFF0,
                &args![buffers, buffer, offset, size, flags],
            )
            .u32();
        if out == 0 {
            return;
        }
        for _ in 0..count {
            copy_floats(e, out, positions, 3);
            out = out.wrapping_add(0xc);
            positions = positions.wrapping_add(0xc);
            if colors != 0 {
                let packed = packed_color(e, colors);
                e.mem.set_u32(out, packed);
                out = out.wrapping_add(4);
                colors = colors.wrapping_add(0x10);
            }
            if coordinates != 0 {
                copy_floats(e, out, coordinates, 2);
                out = out.wrapping_add(8);
                coordinates = coordinates.wrapping_add(8);
            }
        }
        let buffers = e.mem.u32(t.wrapping_add(0x8bc));
        let buffer = e.mem.u32(list.wrapping_add(8));
        e.call(BUFFERS_UNLOCK_00E8C0D0, &args![buffers, buffer]);
        shader_set_up(e, this, shader);
        let passes = e.vcall(shader, 0x90, &args![]).u32();
        let stride = if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
            let strides = e.mem.u32(stream.wrapping_add(0x24));
            e.mem.u32(strides)
        } else {
            0
        };
        let device = e.get(this, NiDX9Renderer::device);
        let buffer = e.mem.u32(list.wrapping_add(8));
        e.vcall(device, 0x190, &args![0u32, buffer, 0u32, stride]);
        let device = e.get(this, NiDX9Renderer::device);
        e.vcall(device, 0x1a0, &args![index_buffer]);
        if passes != 0 {
            loop {
                let device = e.get(this, NiDX9Renderer::device);
                let primitives = e.mem.u32(index_count) >> 1;
                let vertices = e.mem.u32(stream.wrapping_add(0x18));
                let base = e.mem.u32(stream.wrapping_add(0x38));
                e.vcall(
                    device,
                    0x148,
                    &args![2u32, base, 0u32, vertices, 0u32, primitives],
                );
                if e.vcall(shader, 0x94, &args![]).u32() == 0 {
                    break;
                }
            }
        }
        let word = e.mem.u32(t.wrapping_add(0xc));
        e.vcall(shader, 0x8c, &args![word]);
    });
}

/// The screen-texture buffers the renderer keeps (`00e723a0`): positions (+0xA80, 8 bytes a
/// vertex), colours (+0xA84, 16 bytes a vertex), texture coordinates (+0xA88, 8 bytes a
/// vertex) and their 16-bit vertex capacity (+0xA8C); the 16-bit indices (+0xA90) and
/// their capacity (+0xA94).
const SCREEN_POSITIONS: u32 = 0xa80;
const SCREEN_COLORS: u32 = 0xa84;
const SCREEN_COORDINATES: u32 = 0xa88;
const SCREEN_CAPACITY: u32 = 0xa8c;
const SCREEN_INDICES: u32 = 0xa90;
const SCREEN_INDEX_CAPACITY: u32 = 0xa94;
/// The memory manager's allocate (`00aa3e40`, `thiscall`, one word).
const MEMORY_MANAGER_ALLOCATE_00AA3E40: u32 = 0x00aa_3e40;
/// The `eh vector constructor iterator` (`00401050`, `stdcall`: the array, the element
/// size, the count, the constructor) and the colour constructor it is given (`004a7800`).
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const COLOR_CONSTRUCT_004A7800: u32 = 0x004a_7800;
/// `NiAlloc` (`00aa1070`, `cdecl`, the size).
const NI_ALLOC_00AA1070: u32 = 0x00aa_1070;
/// `00e8c570` (`thiscall` on the object at +0x8BC, eight stack words): fills the stream's
/// vertex buffer from the screen-texture buffers.
const SCREEN_VERTICES_00E8C570: u32 = 0x00e8_c570;
/// `00e8f200` (`thiscall` on the object at +0x8C0, seven stack words): the index buffer.
const SCREEN_INDEX_BUFFER_00E8F200: u32 = 0x00e8_f200;
/// `00e8eee0` (`thiscall` on a stream, no stack words): drops its index buffer.
const STREAM_DROP_INDICES_00E8EEE0: u32 = 0x00e8_eee0;
/// `0.5` (`double`, `01011588`): the half-pixel offset.
const HALF_PIXEL_01011588: u32 = 0x0101_1588;

/// A 16-bit coordinate brought into `0..limit` the way the code does it: while negative
/// (as a 16-bit value) `limit` is added, while not below `limit` (signed) it is taken
/// away, both in 16 bits.
fn wrap_coordinate(value: u16, limit: u32) -> i32 {
    let mut v = value;
    while (v as i16) < 0 {
        v = v.wrapping_add(limit as u16);
    }
    while i32::from(v as i16) >= limit as i32 {
        v = v.wrapping_sub(limit as u16);
    }
    i32::from(v as i16)
}

// Translated from 00e723a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words: a screen texture and flags;
/// called by `00e74cc0`): builds a screen texture's quads. With `n` rectangles (+0x10,
/// 16 bits) there are `4n` vertices (16 bits) and `6n` indices; the stream (+0x1C) gets
/// `2n` triangles (+0x40, +0x44), the vertex counts (+0x18, +0x1C) and one strip. With bit
/// 3 of the flags, the renderer's buffers grow to the vertex count when they are too small
/// (old ones freed, new ones from the memory manager, the colours constructed by
/// `004a7800`), and when the stream's buffer (its first list entry) holds fewer vertices
/// than needed (its +0x14 over the first stride, an unsigned division) the flags become
/// 0xF. (The code then tests `flags | ~8` for zero, which never holds.) For each rectangle
/// (0x1C bytes at +8: 16-bit top +0, left +2, width +4, height +6, texture top +8, left
/// +0xA, a colour of four words +0xC) the left and top are wrapped into the current render
/// target's width and height (+0x888, virtuals +0x8C, +0x90), moved by −0.5 and the four
/// corners written (x left or left + width for corners 2 and 3, y top or top + height for
/// odd corners, the texture coordinates scaled by one over the first texture's width and
/// height, virtuals +0x94 and +0x98, all x87 then stored as floats). `00e8c570` fills the
/// stream from the buffers; when it worked and bit 3 is set, the indices (two triangles a
/// quad: 0, 1, 2, 2, 1, 3) are rebuilt when there is not room for them, and the index
/// buffer from `00e8f200` replaces the stream's (after `00e8eee0`) when it changed.
pub fn fn_00e723a0(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr, flags: u32) {
    let t = this.addr();
    let a = texture.addr();
    let rects = u32::from(e.mem.u16(a.wrapping_add(0x10)));
    let stream = e.mem.u32(a.wrapping_add(0x1c));
    let vertices = rects.wrapping_mul(4) & 0xffff;
    let indices = ((vertices * 3) as i32 / 2) as u32;
    let rebuild = (flags as u8 >> 3) & 1 != 0;
    let mut flags = flags;
    e.mem.set_u32(stream.wrapping_add(0x40), rects * 2);
    e.mem.set_u32(stream.wrapping_add(0x44), rects * 2);
    e.mem.set_u32(stream.wrapping_add(0x18), vertices);
    e.mem.set_u32(stream.wrapping_add(0x1c), vertices);
    e.mem.set_u32(stream.wrapping_add(0x50), 0);
    e.mem.set_u32(stream.wrapping_add(0x4c), 0);
    e.mem.set_u32(stream.wrapping_add(0x48), 1);
    let list = if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
        let lists = e.mem.u32(stream.wrapping_add(0x28));
        e.mem.u32(lists)
    } else {
        0
    };
    if rebuild {
        if vertices as u16 > e.mem.u16(t.wrapping_add(SCREEN_CAPACITY)) {
            for slot in [SCREEN_POSITIONS, SCREEN_COLORS, SCREEN_COORDINATES] {
                let old = e.mem.u32(t.wrapping_add(slot));
                e.call(
                    MEMORY_MANAGER_DEALLOCATE_00AA4060,
                    &args![MEMORY_MANAGER_OBJECT_011F6238, old],
                );
            }
            let positions = e
                .call(
                    MEMORY_MANAGER_ALLOCATE_00AA3E40,
                    &args![MEMORY_MANAGER_OBJECT_011F6238, vertices * 8],
                )
                .u32();
            e.mem.set_u32(t.wrapping_add(SCREEN_POSITIONS), positions);
            let colors = e
                .call(
                    MEMORY_MANAGER_ALLOCATE_00AA3E40,
                    &args![MEMORY_MANAGER_OBJECT_011F6238, vertices * 16],
                )
                .u32();
            if colors != 0 {
                e.call(
                    VECTOR_CONSTRUCT_00401050,
                    &args![colors, 0x10u32, vertices, COLOR_CONSTRUCT_004A7800],
                );
            }
            e.mem.set_u32(t.wrapping_add(SCREEN_COLORS), colors);
            let coordinates = e
                .call(
                    MEMORY_MANAGER_ALLOCATE_00AA3E40,
                    &args![MEMORY_MANAGER_OBJECT_011F6238, vertices * 8],
                )
                .u32();
            e.mem
                .set_u32(t.wrapping_add(SCREEN_COORDINATES), coordinates);
            e.mem
                .set_u16(t.wrapping_add(SCREEN_CAPACITY), vertices as u16);
        }
        if list != 0 {
            let size = e.mem.u32(list.wrapping_add(0x14));
            let stride = if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
                let strides = e.mem.u32(stream.wrapping_add(0x24));
                e.mem.u32(strides)
            } else {
                0
            };
            // DIV: a zero stride faults in the exe; the Rust division panics.
            if vertices > size / stride {
                flags = 0xf;
            }
        }
    }
    let group = e.get(this, NiDX9Renderer::target_group);
    let width = e.vcall(group, 0x8c, &args![0u32]).u32();
    let group = e.get(this, NiDX9Renderer::target_group);
    let height = e.vcall(group, 0x90, &args![0u32]).u32();
    let first_texture = |e: &Engine| {
        let holder = e.mem.u32(a.wrapping_add(0x14));
        let entry = e.mem.u32(e.mem.u32(holder.wrapping_add(0x20)));
        if entry != 0 {
            e.mem.u32(entry.wrapping_add(8))
        } else {
            0
        }
    };
    let image = first_texture(e);
    let image_width = e.vcall(image, 0x94, &args![]).u16();
    let image = first_texture(e);
    let inverse_width = x87_store32(x87_div(1.0, f64::from(image_width)));
    let image_height = e.vcall(image, 0x98, &args![]).u16();
    let inverse_height = x87_store32(x87_div(1.0, f64::from(image_height)));
    let iw = x87_load32(inverse_width);
    let ih = x87_load32(inverse_height);
    let half = x87_load64(e.mem.u64(HALF_PIXEL_01011588));
    let mut i = 0u16;
    while u32::from(i) < rects {
        let rect = e
            .mem
            .u32(a.wrapping_add(8))
            .wrapping_add(u32::from(i).wrapping_mul(0x1c));
        let left = wrap_coordinate(e.mem.u16(rect.wrapping_add(2)), width);
        let top = wrap_coordinate(e.mem.u16(rect), height);
        let w = f64::from(e.mem.u16(rect.wrapping_add(4)));
        let h = f64::from(e.mem.u16(rect.wrapping_add(6)));
        let texture_left = f64::from(e.mem.u16(rect.wrapping_add(0xa)));
        let texture_top = f64::from(e.mem.u16(rect.wrapping_add(8)));
        let x0 = x87_store32(x87_sub(f64::from(left), half));
        let y0 = x87_store32(x87_sub(f64::from(top), half));
        let x1 = x87_store32(x87_add(x87_load32(x0), w));
        let y1 = x87_store32(x87_add(x87_load32(y0), h));
        let u0 = x87_store32(x87_mul(texture_left, iw));
        let v0 = x87_store32(x87_mul(texture_top, ih));
        let u1 = x87_store32(x87_add(x87_mul(w, iw), x87_load32(u0)));
        let v1 = x87_store32(x87_add(x87_mul(h, ih), x87_load32(v0)));
        let base = u32::from(i).wrapping_mul(4);
        for corner in 0..4u32 {
            let vertex = (base.wrapping_add(corner)) & 0xffff;
            let x = if corner >> 1 == 0 { x0 } else { x1 };
            let y = if corner & 1 == 0 { y0 } else { y1 };
            let u = if corner >> 1 == 0 { u0 } else { u1 };
            let v = if corner & 1 == 0 { v0 } else { v1 };
            let positions = e.mem.u32(t.wrapping_add(SCREEN_POSITIONS));
            e.mem
                .set_u32(positions.wrapping_add(vertex * 8), x87_quiet_bits(x));
            let positions = e.mem.u32(t.wrapping_add(SCREEN_POSITIONS));
            e.mem
                .set_u32(positions.wrapping_add(vertex * 8 + 4), x87_quiet_bits(y));
            let colors = e
                .mem
                .u32(t.wrapping_add(SCREEN_COLORS))
                .wrapping_add(vertex * 16);
            for word in 0..4u32 {
                let value = e.mem.u32(rect.wrapping_add(0xc + 4 * word));
                e.mem.set_u32(colors.wrapping_add(4 * word), value);
            }
            let coordinates = e.mem.u32(t.wrapping_add(SCREEN_COORDINATES));
            e.mem
                .set_u32(coordinates.wrapping_add(vertex * 8), x87_quiet_bits(u));
            let coordinates = e.mem.u32(t.wrapping_add(SCREEN_COORDINATES));
            e.mem
                .set_u32(coordinates.wrapping_add(vertex * 8 + 4), x87_quiet_bits(v));
        }
        i = i.wrapping_add(1);
    }
    let buffers = e.mem.u32(t.wrapping_add(0x8bc));
    let positions = e.mem.u32(t.wrapping_add(SCREEN_POSITIONS));
    let colors = e.mem.u32(t.wrapping_add(SCREEN_COLORS));
    let coordinates = e.mem.u32(t.wrapping_add(SCREEN_COORDINATES));
    let filled = e
        .call(
            SCREEN_VERTICES_00E8C570,
            &args![
                buffers,
                stream,
                vertices,
                positions,
                colors,
                coordinates,
                flags,
                list,
                0u32
            ],
        )
        .u32();
    if filled == 0 || !rebuild {
        return;
    }
    if indices > e.mem.u32(t.wrapping_add(SCREEN_INDEX_CAPACITY)) {
        let old = e.mem.u32(t.wrapping_add(SCREEN_INDICES));
        e.call(NI_FREE_00AA10F0, &args![old]);
        let fresh = e.call(NI_ALLOC_00AA1070, &args![indices * 2]).u32();
        e.mem.set_u32(t.wrapping_add(SCREEN_INDICES), fresh);
        e.mem
            .set_u32(t.wrapping_add(SCREEN_INDEX_CAPACITY), indices);
        let mut at = 0u16;
        let mut corner = 0u32;
        while corner < vertices {
            let b = corner as u16;
            for value in [b, b + 1, b + 2, b + 2, b + 1, b + 3] {
                let array = e.mem.u32(t.wrapping_add(SCREEN_INDICES));
                e.mem.set_u16(array.wrapping_add(u32::from(at) * 2), value);
                at = at.wrapping_add(1);
            }
            corner += 4;
        }
    }
    let usage = if e.mem.u8(stream.wrapping_add(0x14)) != 0 {
        0x10u32
    } else {
        0
    };
    let old = e.mem.u32(stream.wrapping_add(0x34));
    let not_ex = u32::from(e.mem.u8(DIRECT3D_IS_EX) == 0);
    e.with_stack(4, |e, size| {
        let size = size.addr();
        let current = e.mem.u32(stream.wrapping_add(0x30));
        e.mem.set_u32(size, current);
        let array = e.mem.u32(t.wrapping_add(SCREEN_INDICES));
        let maker = e.mem.u32(t.wrapping_add(0x8c0));
        let fresh = e
            .call(
                SCREEN_INDEX_BUFFER_00E8F200,
                &args![maker, array, indices, indices, old, size, not_ex, usage],
            )
            .u32();
        if fresh == old {
            return;
        }
        e.call(STREAM_DROP_INDICES_00E8EEE0, &args![stream]);
        e.mem.set_u32(stream.wrapping_add(0x34), fresh);
        e.mem.set_u32(stream.wrapping_add(0x2c), indices);
        let size = e.mem.u32(size);
        e.mem.set_u32(stream.wrapping_add(0x30), size);
    });
}

/// The calls of the device reset `00e736b0`: on the objects at +0x8C8 (`00e8be20` before,
/// `00e8b170` after), +0x8C0 (`00e8f6d0`) and +0x8BC (`00e8cd60`), all `thiscall` without
/// stack words; `00e7cc50` (`cdecl`: the device and 1, 2 or 3), `00e7cec0` (`cdecl`, no
/// arguments) and `00e7cf00` (`cdecl`, the device); the map step `00e6e630` (`thiscall`:
/// the position, key and value by address).
const RESET_00E8BE20: u32 = 0x00e8_be20;
const RESET_00E8B170: u32 = 0x00e8_b170;
const RESET_00E8F6D0: u32 = 0x00e8_f6d0;
const RESET_00E8CD60: u32 = 0x00e8_cd60;
const RESET_00E7CC50: u32 = 0x00e7_cc50;
const RESET_00E7CEC0: u32 = 0x00e7_cec0;
const RESET_00E7CF00: u32 = 0x00e7_cf00;
const MAP_GET_NEXT_00E6E630: u32 = 0x00e6_e630;
/// The class the render target groups' buffers are cast to (`0126f5d8`).
const COLOR_BUFFER_RTTI_0126F5D8: u32 = 0x0126_f5d8;
/// The failure messages of the reset: a callback refused before (`010ee2e8`) or after
/// (`010ee238`), the texture defaults could not be made again (`010ee298`).
const RESET_CALLBACK_FAILED_010EE2E8: u32 = 0x010e_e2e8;
const RESET_CALLBACK_AFTER_FAILED_010EE238: u32 = 0x010e_e238;
const RESET_DEFAULTS_FAILED_010EE298: u32 = 0x010e_e298;
/// `"... no shader"` reported by `AddToBatch` with the geometry's name (`010ee438`).
const NO_BATCH_SHADER_010EE438: u32 = 0x010e_e438;

/// The first node of an `NiTPointerMap` (+4 the bucket count, +8 the buckets): the first
/// non-null bucket, or 0.
fn map_first_node(e: &Engine, map: u32) -> u32 {
    let count = e.mem.u32(map.wrapping_add(4));
    let buckets = e.mem.u32(map.wrapping_add(8));
    let mut i = 0u32;
    while i < count {
        let node = e.mem.u32(buckets.wrapping_add(4 * i));
        if node != 0 {
            return node;
        }
        i += 1;
    }
    0
}

/// The node after `node` (next +0, key +4): its next, or the first non-null bucket after
/// the key's own (the map's virtual +4 gives the key's bucket), or 0.
fn map_next_node(e: &mut Engine, map: u32, node: u32) -> u32 {
    let next = e.mem.u32(node);
    if next != 0 {
        return next;
    }
    let key = e.mem.u32(node.wrapping_add(4));
    let mut i = e.vcall(map, 4, &args![key]).u32().wrapping_add(1);
    let count = e.mem.u32(map.wrapping_add(4));
    while i < count {
        let buckets = e.mem.u32(map.wrapping_add(8));
        let found = e.mem.u32(buckets.wrapping_add(4 * i));
        if found != 0 {
            return found;
        }
        i += 1;
    }
    0
}

/// The object `object` cast to `rtti` the way the code inlines it (the chain of its virtual
/// +0xC), or 0.
fn cast_to(e: &mut Engine, object: u32, rtti: u32) -> u32 {
    let mut link = e.vcall(object, 0xc, &args![]).u32();
    while link != 0 {
        if link == rtti {
            return object;
        }
        link = e.mem.u32(link.wrapping_add(4));
    }
    0
}

/// Steps through the render target groups at +0x890 (`00e6e630`), and for each one runs
/// `each(group)` and releases the reference `00e6e630` gave (its count at +4 down, at 0
/// its virtual +4).
fn for_each_target_group(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    each: fn(&mut Engine, Ptr<NiDX9Renderer>, u32),
) {
    let map = this.addr().wrapping_add(0x890);
    let first = map_first_node(e, map);
    e.with_stack(0xc, |e, locals| {
        let position = locals.addr();
        let value = position + 4;
        let key = position + 8;
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(value, 0);
            e.call(MAP_GET_NEXT_00E6E630, &args![map, position, key, value]);
            let group = e.mem.u32(value);
            each(e, this, group);
            let left = e
                .call(IMPORT_INTERLOCKED_DOWN, &args![group.wrapping_add(4)])
                .u32();
            if left == 0 {
                e.vcall(group, 4, &args![]);
            }
        }
    });
}

/// Runs the renderer's reset callbacks (16-bit count at +0xADE, functions at +0xAD8, their
/// words at +0xAE8) with `before`, the count read at the start; answers false at the first
/// one answering false.
fn reset_callbacks(e: &mut Engine, this: Ptr<NiDX9Renderer>, count: u32, before: u32) -> bool {
    let t = this.addr();
    let mut i = 0u32;
    while i < count {
        let functions = e.mem.u32(t.wrapping_add(0xad8));
        let function = e.mem.u32(functions.wrapping_add(4 * i));
        let words = e.mem.u32(t.wrapping_add(0xae8));
        let word = e.mem.u32(words.wrapping_add(4 * i));
        if function != 0 && !e.call(function, &args![before, word]).bool() {
            return false;
        }
        i += 1;
    }
    true
}

// Translated from 00e736b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): resets the Direct3D device.
/// Before: `00e8be20`, `00e8f6d0`, `00e8cd60` on the objects at +0x8C8, +0x8C0, +0x8BC and
/// `00e7cc50(device, 1..3)`; each render target group's buffer (virtual +0xB0 with 0, its
/// +0x10 cast to `0126f5d8`) and depth buffer (virtual +0xB4, cast to `0126f5ac`) get their
/// virtual +0x54; every texture of the maps at +0x8CC and +0x8DC is purged (the renderer's
/// virtual +0x108) and its entry set to 0, the dynamic textures (+0x8EC) lose their data
/// (+0x24 cleared, the data's virtual 0 with 1); `00e7cec0`, each registered shader's
/// virtual +0xC4 (list at +0x90C), the object at +0x8B4's virtual 0, and the reset
/// callbacks with 1 (one answering false is reported and answers false). Then the device's
/// `Reset` (vtable +0x40) with the parameters at +0x20 of the default group's buffer; a
/// failure answers false. After: the reset count (+0x6F8) grows, the byte at +0x6FC is
/// cleared, the groups' buffers and depth buffers (virtual +0xCC) get their virtual +0x58
/// with the device, the texture defaults are made again (`00e6ec90`; a failure is reported
/// and answers false), the textures are made again (the renderer's virtuals +0x140 with
/// the texture and its +0x3C, +0x148, +0x14C), `00e7cf00(device)`, each shader's virtual
/// +0xC8, the object at +0x8B8's virtual +0x100, `00e8b170` and the callbacks with 0 (as
/// before, message `010ee238`). Answers true.
pub fn fn_00e736b0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> bool {
    let t = this.addr();
    let object = e.mem.u32(t.wrapping_add(0x8c8));
    e.call(RESET_00E8BE20, &args![object]);
    let object = e.mem.u32(t.wrapping_add(0x8c0));
    e.call(RESET_00E8F6D0, &args![object]);
    let object = e.mem.u32(t.wrapping_add(0x8bc));
    e.call(RESET_00E8CD60, &args![object]);
    for which in 1..=3u32 {
        let device = e.get(this, NiDX9Renderer::device);
        e.call(RESET_00E7CC50, &args![device, which]);
    }
    for_each_target_group(e, this, |e, _, group| {
        let buffer = e.vcall(group, 0xb0, &args![0u32]).u32();
        let surface = e.mem.u32(buffer.wrapping_add(0x10));
        let cast = if surface != 0 {
            cast_to(e, surface, COLOR_BUFFER_RTTI_0126F5D8)
        } else {
            0
        };
        e.vcall(cast, 0x54, &args![]);
        let depth = e.vcall(group, 0xb4, &args![]).u32();
        if depth != 0 {
            let surface = e.mem.u32(depth.wrapping_add(0x10));
            if surface != 0 {
                let cast = cast_to(e, surface, DEPTH_BUFFER_RTTI_0126F5AC);
                if cast != 0 {
                    e.vcall(cast, 0x54, &args![]);
                }
            }
        }
    });
    for map_offset in [0x8ccu32, 0x8dc] {
        let map = t.wrapping_add(map_offset);
        let mut node = map_first_node(e, map);
        while node != 0 {
            let key = e.mem.u32(node.wrapping_add(4));
            let next = map_next_node(e, map, node);
            e.vcall(t, 0x108, &args![key]);
            e.call(MAP_SET_AT, &args![map, key, 0u32]);
            node = next;
        }
    }
    let map = t.wrapping_add(0x8ec);
    let mut node = map_first_node(e, map);
    while node != 0 {
        let key = e.mem.u32(node.wrapping_add(4));
        let data = e.mem.u32(node.wrapping_add(8));
        let next = map_next_node(e, map, node);
        e.mem.set_u32(key.wrapping_add(0x24), 0);
        if data != 0 {
            e.vcall(data, 0, &args![1u32]);
        }
        e.call(MAP_SET_AT, &args![map, key, 0u32]);
        node = next;
    }
    e.call(RESET_00E7CEC0, &args![]);
    let mut node = e.mem.u32(t.wrapping_add(0x90c));
    while node != 0 {
        let shader = e.mem.u32(node.wrapping_add(8));
        node = e.mem.u32(node);
        e.vcall(shader, 0xc4, &args![]);
    }
    let object = e.mem.u32(t.wrapping_add(0x8b4));
    e.vcall(object, 0, &args![]);
    let count = u32::from(e.mem.u16(t.wrapping_add(0xade)));
    if !reset_callbacks(e, this, count, 1) {
        e.call(REPORT_00A29680, &args![RESET_CALLBACK_FAILED_010EE2E8]);
        return false;
    }
    let group = e.get(this, NiDX9Renderer::object_884);
    let buffer = e.vcall(group, 0xb0, &args![0u32]).u32();
    let surface = e.mem.u32(buffer.wrapping_add(0x10));
    let cast = if surface != 0 {
        cast_to(e, surface, COLOR_BUFFER_RTTI_0126F5D8)
    } else {
        0
    };
    let device = e.get(this, NiDX9Renderer::device);
    if e.vcall(device, 0x40, &args![cast.wrapping_add(0x20)]).i32() < 0 {
        return false;
    }
    let resets = e.mem.u32(t.wrapping_add(0x6f8)).wrapping_add(1);
    e.mem.set_u32(t.wrapping_add(0x6f8), resets);
    e.mem.set_u8(t.wrapping_add(SKIP_END_SCENE), 0);
    for_each_target_group(e, this, |e, this, group| {
        let buffer = e.vcall(group, 0xb0, &args![0u32]).u32();
        let surface = e.mem.u32(buffer.wrapping_add(0x10));
        let cast = if surface != 0 {
            cast_to(e, surface, COLOR_BUFFER_RTTI_0126F5D8)
        } else {
            0
        };
        let device = e.get(this, NiDX9Renderer::device);
        e.vcall(cast, 0x58, &args![device]);
        let depth = e.vcall(group, 0xcc, &args![]).u32();
        if depth != 0 {
            let cast = cast_to(e, depth, DEPTH_BUFFER_RTTI_0126F5AC);
            if cast != 0 {
                let device = e.get(this, NiDX9Renderer::device);
                e.vcall(cast, 0x58, &args![device]);
            }
        }
    });
    if fn_00e6ec90(e, this.cast()) == 0 {
        e.call(REPORT_00A29680, &args![RESET_DEFAULTS_FAILED_010EE298]);
        return false;
    }
    for (map_offset, slot) in [(0x8ccu32, 0x140u32), (0x8dc, 0x148), (0x8ec, 0x14c)] {
        let map = t.wrapping_add(map_offset);
        let mut node = map_first_node(e, map);
        while node != 0 {
            let key = e.mem.u32(node.wrapping_add(4));
            let next = map_next_node(e, map, node);
            if slot == 0x140 {
                let word = e.mem.u32(key.wrapping_add(0x3c));
                e.vcall(t, slot, &args![key, word]);
            } else {
                e.vcall(t, slot, &args![key]);
            }
            node = next;
        }
    }
    let device = e.get(this, NiDX9Renderer::device);
    e.call(RESET_00E7CF00, &args![device]);
    let mut node = e.mem.u32(t.wrapping_add(0x90c));
    while node != 0 {
        let shader = e.mem.u32(node.wrapping_add(8));
        node = e.mem.u32(node);
        e.vcall(shader, 0xc8, &args![]);
    }
    let library = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(library, 0x100, &args![]);
    let object = e.mem.u32(t.wrapping_add(0x8c8));
    e.call(RESET_00E8B170, &args![object]);
    if !reset_callbacks(e, this, count, 0) {
        e.call(
            REPORT_00A29680,
            &args![RESET_CALLBACK_AFTER_FAILED_010EE238],
        );
        return false;
    }
    true
}

/// A batch node (0x10 bytes): from the renderer's free list (`0126f0ec`, the next at
/// +0xC, which is cleared) or, when it is empty, new from `00aa13e0`.
fn batch_node(e: &mut Engine) -> u32 {
    let free = e.mem.u32(FREE_NODES_0126F0EC);
    if free == 0 {
        return e.call(NEW_OBJECT, &args![0x10u32]).u32();
    }
    let next = e.mem.u32(free.wrapping_add(0xc));
    e.mem.set_u32(FREE_NODES_0126F0EC, next);
    e.mem.set_u32(free.wrapping_add(0xc), 0);
    free
}

/// Appends `node` to the batch list (+0x620 head, +0x624 tail).
fn append_batch_node(e: &mut Engine, this: Ptr<NiDX9Renderer>, node: u32) {
    let t = this.addr();
    let tail = e.mem.u32(t.wrapping_add(0x624));
    if tail != 0 {
        e.mem.set_u32(tail.wrapping_add(0xc), node);
    } else {
        e.mem.set_u32(t.wrapping_add(BATCH_LIST), node);
    }
    e.mem.set_u32(t.wrapping_add(0x624), node);
}

// Translated from 00e74a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::AddToBatch` (engine map; `thiscall`, one stack word: a geometry).
/// Without a batch shader (+0x630) the geometry's shader (+0xC0) becomes it (an `NiPointer`
/// assignment: the old one released, the new one's count up); still none reports the
/// geometry's name (+8) and does nothing more. Nothing when the geometry's data (+0xB8)
/// answers 0 from its virtual +0x90 (16 bits). Asks `00e72c20(geometry, shader)` whether it
/// is skinned and gives the geometry group manager `00e88dc0(+0x8B0, data, skin instance,
/// that answer, 0, 0)` (the callee reads only the answer's low byte; the code pushes it in
/// a stack word whose other bytes it never set, passed here as 0). Then appends batch
/// nodes: for a skinned geometry one per partition of its skin partition (the geometry,
/// the partition, the partition's +0x28), otherwise one (the geometry, 0, the data's
/// +0x34).
pub fn ni_xenon_renderer_add_to_batch(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    let g = geometry.addr();
    let data = e.mem.u32(g.wrapping_add(0xb8));
    let skin = e.mem.u32(g.wrapping_add(0xbc));
    if e.mem.u32(t.wrapping_add(BATCH_SHADER)) == 0 {
        let shader = e.mem.u32(g.wrapping_add(0xc0));
        let old = e.mem.u32(t.wrapping_add(BATCH_SHADER));
        if old != shader {
            if old != 0 {
                let left = e
                    .call(IMPORT_INTERLOCKED_DOWN, &args![old.wrapping_add(4)])
                    .u32();
                if left == 0 {
                    e.vcall(old, 4, &args![]);
                }
            }
            e.mem.set_u32(t.wrapping_add(BATCH_SHADER), shader);
            if shader != 0 {
                e.call(IMPORT_INTERLOCKED_UP, &args![shader.wrapping_add(4)]);
            }
        }
        if e.mem.u32(t.wrapping_add(BATCH_SHADER)) == 0 {
            let name = e.mem.u32(g.wrapping_add(8));
            e.call(REPORT_00A29680, &args![NO_BATCH_SHADER_010EE438, name]);
            return;
        }
    }
    if e.vcall(data, 0x90, &args![]).u16() == 0 {
        return;
    }
    let shader = e.mem.u32(t.wrapping_add(BATCH_SHADER));
    let skinned = fn_00e72c20(e, this, geometry, shader);
    let manager = e.mem.u32(t.wrapping_add(0x8ac));
    let group = e.mem.u32(t.wrapping_add(0x8b0));
    e.call(
        GROUP_MANAGER_00E88DC0,
        &args![manager, group, data, skin, u32::from(skinned), 0u32, 0u32],
    );
    if skinned {
        let partition = e.mem.u32(skin.wrapping_add(0xc));
        let mut element = e.mem.u32(partition.wrapping_add(0xc));
        let count = e.mem.u32(partition.wrapping_add(8));
        for _ in 0..count {
            let node = batch_node(e);
            e.mem.set_u32(node, g);
            e.mem.set_u32(node.wrapping_add(4), element);
            let stream = e.mem.u32(element.wrapping_add(0x28));
            e.mem.set_u32(node.wrapping_add(8), stream);
            e.mem.set_u32(node.wrapping_add(0xc), 0);
            append_batch_node(e, this, node);
            element = element.wrapping_add(0x2c);
        }
        return;
    }
    let node = batch_node(e);
    e.mem.set_u32(node, g);
    e.mem.set_u32(node.wrapping_add(4), 0);
    let stream = e.mem.u32(data.wrapping_add(0x34));
    e.mem.set_u32(node.wrapping_add(8), stream);
    e.mem.set_u32(node.wrapping_add(0xc), 0);
    append_batch_node(e, this, node);
}

// Translated from 00e6f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xC4, `thiscall`, two stack words: a colour
/// buffer and a depth buffer): whether the depth buffer suits the colour buffer. False when
/// the colour buffer is wider (+8) or taller (+0xC) than the depth buffer; then both data's
/// surfaces (+0x10, then +0x14) are described (`GetDesc`, vtable +0x30; the results not
/// checked) and their multisample quality and type must match; then Direct3D's
/// `CheckDepthStencilMatch` (vtable +0x30: the adapter +0x5C4, the device type +0x5C8, the
/// adapter format +0x5D0, the colour format and the depth format) must succeed.
pub fn fn_00e6f260(e: &mut Engine, this: Ptr<NiDX9Renderer>, color: u32, depth: u32) -> bool {
    let t = this.addr();
    if e.mem.u32(color.wrapping_add(8)) > e.mem.u32(depth.wrapping_add(8)) {
        return false;
    }
    if e.mem.u32(color.wrapping_add(0xc)) > e.mem.u32(depth.wrapping_add(0xc)) {
        return false;
    }
    let color_data = e.mem.u32(color.wrapping_add(0x10));
    let color_surface = e.mem.u32(color_data.wrapping_add(0x14));
    let depth_data = e.mem.u32(depth.wrapping_add(0x10));
    e.with_stack(0x40, |e, descriptions| {
        let color_description = descriptions.addr();
        let depth_description = color_description + 0x20;
        e.vcall(color_surface, SURFACE_GET_DESC, &args![color_description]);
        let depth_surface = e.mem.u32(depth_data.wrapping_add(0x14));
        e.vcall(depth_surface, SURFACE_GET_DESC, &args![depth_description]);
        // D3DSURFACE_DESC: Format +0, MultiSampleType +0x10, MultiSampleQuality +0x14.
        if e.mem.u32(color_description + 0x14) != e.mem.u32(depth_description + 0x14) {
            return false;
        }
        if e.mem.u32(color_description + 0x10) != e.mem.u32(depth_description + 0x10) {
            return false;
        }
        let device_type = e.mem.u32(t.wrapping_add(DEVICE_TYPE));
        let adapter = e.mem.u32(t.wrapping_add(ADAPTER));
        let format = e.mem.u32(t.wrapping_add(0x5d0));
        let direct3d = e.mem.u32(DIRECT3D);
        let color_format = e.mem.u32(color_description);
        let depth_format = e.mem.u32(depth_description);
        e.vcall(
            direct3d,
            0x30,
            &args![adapter, device_type, format, color_format, depth_format],
        )
        .i32()
            >= 0
    })
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, callee: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object of `size` bytes whose vtable slot `s` holds the address `tag + s`.
    fn object(e: &mut Engine, tag: u32, size: u32) -> u32 {
        let o = e.mem.alloc(size.max(8));
        let table = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(table + slot, tag + slot);
        }
        e.mem.set_u32(o, table);
        o
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &f in callees {
            e.register(f, |_, _| Ret::default());
        }
    }

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb80);
        e.call_log = Some(vec![]);
        (e, this)
    }

    const DEVICE_TAG: u32 = 0x7100_0000;
    const LIBRARY_TAG: u32 = 0x7200_0000;
    const OBJECT_TAG: u32 = 0x7300_0000;
    const SECOND_TAG: u32 = 0x7400_0000;
    const THIRD_TAG: u32 = 0x7500_0000;

    #[test]
    fn rtti_and_type_answers() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00a5_f200, &args![this]).u32(), 0x011f_4430);
        assert_eq!(e.call(0x00a5_f4f0, &args![this]).u32(), 0x011f_443c);
        assert_eq!(e.call(0x00a5_f500, &args![this]).u32(), 0xb);
        assert_eq!(e.call(0x00e7_57b0, &args![this]).u32(), 0x0126_f0f0);
    }

    #[test]
    fn fn_00e695c0_passes_the_sampler_blocks() {
        let (mut e, this) = engine();
        e.register(SAMPLER_HELPER_00E89ED0, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x00e6_95c0, &args![this, 5u32]).u32(), 6);
        assert_eq!(
            calls(&e, SAMPLER_HELPER_00E89ED0),
            vec![vec![5, this + 0x700]]
        );
    }

    #[test]
    fn fn_00e69610_and_fn_00e69650() {
        let (mut e, this) = engine();
        let group = object(&mut e, OBJECT_TAG, 0x40);
        e.mem.set_u32(this + 0x884, group);
        e.register(OBJECT_TAG + 0xb4, |_, _| 0x55u32.into_ret());
        assert_eq!(e.call(0x00e6_9610, &args![this]).u32(), 0x55);
        quiet(&mut e, &[RENDERER_HELPER_00E8A9B0]);
        assert!(e.call(0x00e6_9650, &args![this, 9u32]).bool());
        assert_eq!(calls(&e, RENDERER_HELPER_00E8A9B0), vec![vec![this]]);
    }

    #[test]
    fn fn_00e698a0_and_fn_00e698d0_draw_indexed() {
        let (mut e, this) = engine();
        let device = object(&mut e, DEVICE_TAG, 0x10);
        e.mem.set_u32(this + 0x288, device);
        e.register(DEVICE_TAG + 0x148, |_, _| 7u32.into_ret());
        assert_eq!(
            e.call(0x00e6_98a0, &args![this, 1u32, 2u32, 3u32, 4u32])
                .u32(),
            7
        );
        e.call(0x00e6_98d0, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(
            calls(&e, DEVICE_TAG + 0x148),
            vec![
                vec![device, 4, 1, 0, 2, 3, 4],
                vec![device, 5, 1, 0, 2, 3, 4]
            ]
        );
    }

    #[test]
    fn fn_00e69900_and_fn_00e69920() {
        let (mut e, this) = engine();
        e.call(0x00e6_9900, &args![this, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u32(this + 0x628), 0x11);
        assert_eq!(e.mem.u32(this + 0x62c), 0x22);
        e.map(FREE_NODES_0126F0EC, 4);
        e.mem.set_u32(FREE_NODES_0126F0EC, 0x99);
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        e.mem.set_u32(a + 0xc, b);
        e.mem.set_u32(this + 0x620, a);
        e.mem.set_u32(this + 0x624, b);
        e.call(0x00e6_9920, &args![this]);
        assert_eq!(e.mem.u32(FREE_NODES_0126F0EC), b);
        assert_eq!(e.mem.u32(b + 0xc), a);
        assert_eq!(e.mem.u32(a + 0xc), 0x99);
        assert_eq!(e.mem.u32(this + 0x620), 0);
        assert_eq!(e.mem.u32(this + 0x624), 0);
    }

    #[test]
    fn fn_00e69a00_constructs_a_block() {
        let (mut e, this) = engine();
        e.register(NEW_OBJECT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e6_9a00, &args![this, 3u32]).u32(), 0);
        e.register(NEW_OBJECT, |_, _| 0x4000u32.into_ret());
        e.register(CONSTRUCT_00E8DB90, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x00e6_9a00, &args![this, 3u32]).u32(), 0x4003);
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![0x54], vec![0x54]]);
    }

    #[test]
    fn fn_00e69ad0_sets_or_clears_the_error() {
        let (mut e, _) = engine();
        e.map(ERROR_MESSAGE_011F4648, 0x100);
        e.mem.set_u8(ERROR_MESSAGE_011F4648, b'x');
        e.register(STRNCPY_S, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e6_9ad0, &args![0u32]).u32(), 0);
        assert_eq!(e.mem.u8(ERROR_MESSAGE_011F4648), 0);
        e.call(0x00e6_9ad0, &args![0x9000u32]);
        assert_eq!(
            calls(&e, STRNCPY_S),
            vec![vec![ERROR_MESSAGE_011F4648, 0x100, 0x9000, 0xff]]
        );
    }

    #[test]
    fn fn_00e69b60_writes_a_vertex_with_or_without_w() {
        let (mut e, _) = engine();
        let out = e.mem.alloc(0x30);
        let position = e.mem.alloc(0xc);
        let normal = e.mem.alloc(0xc);
        for i in 0..3u32 {
            e.mem
                .set_u32(position + 4 * i, (1.0f32 + i as f32).to_bits());
            e.mem.set_u32(normal + 4 * i, (4.0f32 + i as f32).to_bits());
        }
        let u = 7.0f32.to_bits();
        let v = 8.0f32.to_bits();
        let r = e
            .call(
                0x00e6_9b60,
                &args![
                    0u32,
                    out,
                    position,
                    normal,
                    0xaabb_ccddu32,
                    u,
                    v,
                    9.0f32.to_bits()
                ],
            )
            .u32();
        assert_eq!(r, out + 0x28);
        let words: Vec<u32> = (0..10).map(|i| e.mem.u32(out + 4 * i)).collect();
        let f = |x: f32| x.to_bits();
        assert_eq!(
            words,
            vec![
                f(1.0),
                f(2.0),
                f(3.0),
                f(4.0),
                f(5.0),
                f(6.0),
                0xaabb_ccdd,
                f(7.0),
                f(8.0),
                f(9.0)
            ]
        );
        let r = e
            .call(
                0x00e6_9b60,
                &args![0u32, out, position, normal, 0u32, u, v, (-1.0f32).to_bits()],
            )
            .u32();
        assert_eq!(r, out + 0x24);
        let nan = 0x7fc0_0000u32;
        let r = e
            .call(
                0x00e6_9b60,
                &args![0u32, out, position, normal, 0u32, u, v, nan],
            )
            .u32();
        assert_eq!(r, out + 0x24);
        // A negative zero is not below zero.
        let r = e
            .call(
                0x00e6_9b60,
                &args![0u32, out, position, normal, 0u32, u, v, 0x8000_0000u32],
            )
            .u32();
        assert_eq!(r, out + 0x28);
    }

    #[test]
    fn fn_00e69c00_joins_the_flag_texts() {
        let (mut e, this) = engine();
        e.register(STRCAT_S_00EC6BD2, |_, _| Ret::default());
        e.mem.set_u8(this + 0x5cc, 0x10 | 0x40 | 0x20);
        e.mem.set_u8(this + 0x5f0, b'x');
        assert_eq!(e.call(0x00e6_9c00, &args![this]).u32(), this + 0x5f0);
        assert_eq!(e.mem.u8(this + 0x5f0), 0);
        let texts: Vec<u32> = calls(&e, STRCAT_S_00EC6BD2).iter().map(|c| c[2]).collect();
        assert_eq!(
            texts,
            vec![PURE_TEXT_010ED5E4, HWVP_TEXT_010ED5D8, SWVP_TEXT_010ED5C8]
        );
    }

    #[test]
    fn fn_00e6b8f0_builds_the_scaled_matrix() {
        let (mut e, _) = engine();
        e.map(WORLD_OFFSET_011F474C, 0xc);
        for i in 0..3u32 {
            e.mem
                .set_u32(WORLD_OFFSET_011F474C + 4 * i, 0.5f32.to_bits());
        }
        let out = e.mem.alloc(0x40);
        let rotation = e.mem.alloc(0x24);
        for i in 0..9u32 {
            e.mem.set_u32(rotation + 4 * i, (i as f32).to_bits());
        }
        let translation = e.mem.alloc(0xc);
        for i in 0..3u32 {
            e.mem
                .set_u32(translation + 4 * i, (10.0f32 * (i + 1) as f32).to_bits());
        }
        e.mem.set_u32(out + 0x30, 0x1234);
        e.call(0x00e6_b8f0, &args![out, rotation, translation, 2.0f32]);
        let read = |e: &Engine, at: u32| f32::from_bits(e.mem.u32(out + at));
        assert_eq!(read(&e, 0), 0.0);
        assert_eq!(read(&e, 4), 2.0);
        assert_eq!(read(&e, 0x10), 6.0);
        assert_eq!(read(&e, 0x28), 16.0);
        assert_eq!(read(&e, 0xc), 9.5);
        assert_eq!(read(&e, 0x1c), 19.5);
        assert_eq!(read(&e, 0x2c), 29.5);
        assert_eq!(read(&e, 0x3c), 1.0);
        assert_eq!(e.mem.u32(out + 0x30), 0x1234);
    }

    #[test]
    fn fn_00e6bdc0_makes_both_program_caches() {
        let (mut e, this) = engine();
        e.map(EMPTY_FIXED_STRING_0109B220, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING_0109B220, 0x5550);
        e.register(VERTEX_SHADER_PROFILE_00EE6E2E, |_, _| 0u32.into_ret());
        e.register(PIXEL_SHADER_PROFILE_00EE6E28, |_, _| 0x9000u32.into_ret());
        e.register(DEFAULT_WORD_00A8F750, |_, _| 0x66u32.into_ret());
        e.register(NEW_OBJECT, |_, _| 0x6000u32.into_ret());
        e.register(FIXED_STRING_MAKE_00A5B690, |_, _| 0x7008u32.into_ret());
        e.register(GROUP_BUFFER_00EE8850, |_, a| (0x100 + a[1]).into_ret());
        e.register(PROGRAM_CACHE_CONSTRUCT, |_, _| 0x6100u32.into_ret());
        quiet(&mut e, &[IMPORT_INTERLOCKED_DOWN]);
        let group = object(&mut e, OBJECT_TAG, 0x10);
        e.mem.set_u32(group + 8, 0x40);
        e.register(OBJECT_TAG + 0x60, |_, a| a[2].into_ret());
        let r = e
            .call(
                0x00e6_bdc0,
                &args![this, group, 2u32, 3u32, 4u32, 5u32, 0u32],
            )
            .u32();
        assert_eq!(r, 2);
        let made = calls(&e, PROGRAM_CACHE_CONSTRUCT);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][..4], [0x6000, 0x100, 0x66, 0]);
        assert_eq!(made[0][5..], [0x40, 2, 3, 5, 4]);
        assert_eq!(made[1][..4], [0x6000, 0x102, 0x66, 2]);
        assert_eq!(
            calls(&e, GROUP_BUFFER_00EE8850),
            vec![vec![group, 0], vec![group, 2]]
        );
        assert_eq!(
            calls(&e, OBJECT_TAG + 0x60),
            vec![vec![group, 0x6100, 0], vec![group, 0x6100, 2]]
        );
        // The empty name is not released, the made one is.
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DOWN), vec![vec![0x7000]]);
    }

    #[test]
    fn fn_00e6c690_describes_the_adapter() {
        let (mut e, this) = engine();
        e.map(DIRECT3D, 4);
        let direct3d = object(&mut e, DEVICE_TAG, 0x10);
        e.mem.set_u32(DIRECT3D, direct3d);
        e.mem.set_u32(this + 0x5c4, 1);
        e.mem.set_u32(this + 0x5c8, 2);
        e.register(MEMSET_ADDR, |_, _| Ret::default());
        quiet(&mut e, &[NI_SPRINTF, STRCAT_S_00EC6BD2]);
        e.register(DEVICE_TAG + 0x14, |_, _| 0x8000_0000u32.into_ret());
        e.mem.set_u8(this + 0x3c4, b'x');
        assert_eq!(e.call(0x00e6_c690, &args![this]).u32(), this + 0x3c4);
        assert_eq!(e.mem.u8(this + 0x3c4), 0);
        assert!(calls(&e, NI_SPRINTF).is_empty());
        e.register(DEVICE_TAG + 0x14, |_, _| 0u32.into_ret());
        e.call(0x00e6_c690, &args![this]);
        let printed = calls(&e, NI_SPRINTF);
        assert_eq!(printed.len(), 1);
        let identifier = calls(&e, DEVICE_TAG + 0x14)[1][3];
        assert_eq!(
            printed[0],
            vec![
                this + 0x3c4,
                0x200,
                DESCRIPTION_FORMAT_010ED954,
                identifier + 0x200,
                REF_TEXT_010ED5C0,
                this + 0x5f0
            ]
        );
        assert_eq!(calls(&e, MEMSET_ADDR)[1][1..], [0, 0x44c]);
    }

    #[test]
    fn fn_00e6c760_and_fn_00e6c770() {
        let (mut e, this) = engine();
        e.map(STORED_WORD_0126EFC0, 4);
        assert!(e.call(0x00e6_c760, &args![this, 0x42u32]).bool());
        assert_eq!(e.call(0x00e6_c770, &args![this]).u32(), 0x42);
    }

    #[test]
    fn fn_00e6d710_passes_objects_with_a_word() {
        let (mut e, this) = engine();
        let manager = object(&mut e, OBJECT_TAG, 0x10);
        e.mem.set_u32(this + 0x8ac, manager);
        e.register(OBJECT_TAG + 0x18, |_, _| 9u32.into_ret());
        assert_eq!(e.call(0x00e6_d710, &args![this, 0u32]).u32(), 0);
        let item = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00e6_d710, &args![this, item]).u32(), item);
        e.mem.set_u32(item + 0x1c, 1);
        assert_eq!(e.call(0x00e6_d710, &args![this, item]).u32(), 9);
        assert_eq!(calls(&e, OBJECT_TAG + 0x18), vec![vec![manager, item]]);
    }

    #[test]
    fn fn_00e6dd00_and_fn_00e6dd40_make_texture_data() {
        let (mut e, this) = engine();
        quiet(&mut e, &[MAP_SET_AT]);
        let texture = e.mem.alloc(0x30);
        e.mem.set_u32(texture + 0x24, 1);
        assert!(e.call(0x00e6_dd00, &args![this, texture]).bool());
        e.mem.set_u32(texture + 0x24, 0);
        e.register(TEXTURE_DATA_00E901A0, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e6_dd00, &args![this, texture]).bool());
        e.register(TEXTURE_DATA_00E901A0, |_, _| 0x5000u32.into_ret());
        assert!(e.call(0x00e6_dd00, &args![this, texture]).bool());
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![this + 0x8dc, texture, 0x5000]]
        );
        assert_eq!(calls(&e, TEXTURE_DATA_00E901A0)[1], vec![texture, this]);
        let data = object(&mut e, OBJECT_TAG, 0x10);
        e.mem.set_u32(texture + 0x24, data);
        e.register(OBJECT_TAG + 0xac, |_, _| 1u32.into_ret());
        assert!(e.call(0x00e6_dd40, &args![this, texture]).bool());
        e.register(OBJECT_TAG + 0xac, |_, _| 0u32.into_ret());
        e.register(TEXTURE_DATA_00E904C0, |_, _| 0x5100u32.into_ret());
        assert!(e.call(0x00e6_dd40, &args![this, texture]).bool());
        assert_eq!(
            calls(&e, MAP_SET_AT)[1],
            vec![this + 0x8ec, texture, 0x5100]
        );
        e.register(TEXTURE_DATA_00E904C0, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e6_dd40, &args![this, texture]).bool());
    }

    /// A texture whose renderer data answers `locked` from its virtual +0xB8 and whose
    /// Direct3D texture locks to the pitch 0x40 and the bits 0x9000.
    fn lock_rig(locked: bool) -> (Engine, u32, u32, u32) {
        let (mut e, this) = engine();
        let texture = object(&mut e, OBJECT_TAG, 0x30);
        let data = object(&mut e, SECOND_TAG, 0x30);
        let surface = object(&mut e, THIRD_TAG, 0x30);
        let device = object(&mut e, DEVICE_TAG, 0x10);
        e.mem.set_u32(this + 0x288, device);
        e.register_double(OBJECT_TAG + 0xac, move |_, _| data.into_ret());
        let answer = u32::from(locked);
        e.register_double(SECOND_TAG + 0xb8, move |_, _| answer.into_ret());
        e.register_double(SECOND_TAG + 0x9c, move |_, _| surface.into_ret());
        e.register_double(SECOND_TAG + 0xb4, move |_, _| surface.into_ret());
        quiet(&mut e, &[SECOND_TAG + 0xbc, REPORT_00A29680]);
        e.register(THIRD_TAG + 0x4c, |e, a| {
            e.mem.set_u32(a[2], 0x40);
            e.mem.set_u32(a[2] + 4, 0x9000);
            Ret::default()
        });
        e.register(THIRD_TAG + 0x50, |_, _| Ret::default());
        e.register(DEVICE_TAG + 0x7c, |_, _| Ret::default());
        e.register(ERROR_STRING_00E89D50, |_, _| 0x77u32.into_ret());
        (e, this, texture, surface)
    }

    #[test]
    fn ni_dx9_renderer_lock_dynamic_texture_locks_level_0() {
        let (mut e, this, texture, surface) = lock_rig(false);
        let pitch = e.mem.alloc(4);
        e.mem.set_u32(pitch, 5);
        assert_eq!(e.call(0x00e6_dd90, &args![this, 0u32, pitch]).u32(), 0);
        assert_eq!(e.mem.u32(pitch), 0);
        assert_eq!(
            e.call(0x00e6_dd90, &args![this, texture, pitch]).u32(),
            0x9000
        );
        assert_eq!(e.mem.u32(pitch), 0x40);
        let lock = &calls(&e, THIRD_TAG + 0x4c)[0];
        assert_eq!([lock[0], lock[1], lock[3], lock[4]], [surface, 0, 0, 0]);
        assert_eq!(calls(&e, SECOND_TAG + 0xbc)[0][1], 1);
        e.mem.set_u8(this + 0x6f5, 1);
        e.call(0x00e6_dd90, &args![this, texture, pitch]);
        assert_eq!(calls(&e, THIRD_TAG + 0x4c)[1][4], 0x2000);
        e.register(THIRD_TAG + 0x4c, |_, _| 0x8000_0001u32.into_ret());
        assert_eq!(e.call(0x00e6_dd90, &args![this, texture, pitch]).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![LOCK_FAILED_010ED96C, 0x77]]
        );
        let (mut e, this, texture, _) = lock_rig(true);
        let pitch = e.mem.alloc(4);
        assert_eq!(e.call(0x00e6_dd90, &args![this, texture, pitch]).u32(), 0);
    }

    #[test]
    fn ni_dx9_renderer_unlock_dynamic_texture_updates_the_texture() {
        let (mut e, this, texture, surface) = lock_rig(true);
        assert!(!e.call(0x00e6_de70, &args![this, 0u32]).bool());
        assert!(e.call(0x00e6_de70, &args![this, texture]).bool());
        let device = e.mem.u32(this + 0x288);
        assert_eq!(
            calls(&e, DEVICE_TAG + 0x7c),
            vec![vec![device, surface, surface]]
        );
        assert_eq!(calls(&e, SECOND_TAG + 0xbc)[0][1], 0);
        e.mem.set_u8(this + 0x6f5, 1);
        assert!(e.call(0x00e6_de70, &args![this, texture]).bool());
        assert_eq!(calls(&e, DEVICE_TAG + 0x7c).len(), 1);
        e.register(THIRD_TAG + 0x50, |_, _| 0x8000_0000u32.into_ret());
        assert!(!e.call(0x00e6_de70, &args![this, texture]).bool());
        assert_eq!(calls(&e, REPORT_00A29680)[0][0], UNLOCK_FAILED_010ED9E4);
        let (mut e, this, texture, _) = lock_rig(false);
        assert!(!e.call(0x00e6_de70, &args![this, texture]).bool());
    }

    #[test]
    fn fn_00e6df60_and_fn_00e6dfa0_remove_texture_data() {
        let (mut e, this) = engine();
        e.register(MAP_REMOVE_AT, |_, _| 1u32.into_ret());
        let texture = e.mem.alloc(0x30);
        e.mem.set_u32(texture + 0x24, 5);
        assert_eq!(e.call(0x00e6_df60, &args![this, texture]).u32(), 1);
        assert_eq!(e.mem.u32(texture + 0x24), 0);
        e.call(0x00e6_dfa0, &args![this, texture]);
        assert_eq!(
            calls(&e, MAP_REMOVE_AT),
            vec![vec![this + 0x8dc, texture], vec![this + 0x8ec, texture]]
        );
    }

    #[test]
    fn small_accessors() {
        let (mut e, this) = engine();
        e.mem.set_u8(this + 0xad0, 3);
        assert_eq!(e.call(0x00e7_57c0, &args![this]).u8(), 3);
        let library = object(&mut e, LIBRARY_TAG, 0x10);
        e.mem.set_u32(this + 0x8b8, library);
        e.register(LIBRARY_TAG + 0x4c, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00e7_57e0, &args![this]).u32(), 1);
        quiet(&mut e, &[LIBRARY_TAG + 0x50, LIBRARY_TAG + 0x40]);
        assert!(e.call(0x00e7_57f0, &args![this, 6u32]).bool());
        assert_eq!(calls(&e, LIBRARY_TAG + 0x50), vec![vec![library, 6]]);
        let signalling = 0x7f80_0001u32;
        e.call(0x00e7_5820, &args![this, signalling]);
        assert_eq!(e.mem.u32(this + 0x5e4), 0x7fc0_0001);
        e.call(0x00e7_5820, &args![this, 2.5f32]);
        assert_eq!(e.call(0x00e7_5830, &args![this]).f64(), 2.5);
        e.call(0x00e7_5a20, &args![this, 0x33u32]);
        assert_eq!(e.call(0x00e7_5a30, &args![this]).u32(), 0x33);
        e.register(LIBRARY_TAG + 0x38, |_, _| Ret {
            st0: 1.25,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00e7_5a40, &args![this]).f64(), 1.25);
        e.call(0x00e7_5a50, &args![this, signalling]);
        assert_eq!(
            calls(&e, LIBRARY_TAG + 0x40),
            vec![vec![library, 0x7fc0_0001]]
        );
    }
}

#[cfg(test)]
mod tests_n084_draw {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, callee: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object of `size` bytes whose vtable slot `s` holds the address `tag + s`.
    fn object(e: &mut Engine, tag: u32, size: u32) -> u32 {
        let o = e.mem.alloc(size.max(8));
        let table = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(table + slot, tag + slot);
        }
        e.mem.set_u32(o, table);
        o
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &f in callees {
            e.register(f, |_, _| Ret::default());
        }
    }

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb80);
        e.call_log = Some(vec![]);
        (e, this)
    }

    const DEVICE_TAG: u32 = 0x7100_0000;
    const GROUP_TAG: u32 = 0x7300_0000;
    const BUFFER_TAG: u32 = 0x7400_0000;
    const SURFACE_TAG: u32 = 0x7500_0000;
    const DATA_TAG: u32 = 0x7700_0000;
    const RENDERER_TAG: u32 = 0x7800_0000;

    #[test]
    fn fn_00e6f100_checks_the_buffers() {
        let (mut e, this) = engine();
        let renderer = object(&mut e, RENDERER_TAG, 0xb80);
        let group = object(&mut e, GROUP_TAG, 0x40);
        let surface = object(&mut e, SURFACE_TAG, 0x10);
        let pixel_format = e.mem.alloc(4);
        e.mem.set_u8(pixel_format + 1, 32);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 8, 640);
        e.mem.set_u32(data + 0xc, 480);
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(buffer + 8, data);
        e.mem.set_u32(buffer + 0xc, pixel_format);
        e.mem.set_u32(buffer + 0x14, surface);
        e.map(FORMAT_BITS_0126EFD8, 0x76);
        e.mem.set_u8(FORMAT_BITS_0126EFD8 + 0x15, 32);
        e.register(GROUP_TAG + 0xa4, |_, _| 2u32.into_ret());
        e.register(RENDERER_TAG + 0xd8, |_, _| 4u32.into_ret());
        e.register(RENDERER_TAG + 0xdc, |_, _| 0u32.into_ret());
        e.register_double(GROUP_TAG + 0xc8, move |_, _| buffer.into_ret());
        e.register(GROUP_TAG + 0xcc, |_, _| 0u32.into_ret());
        e.register(SURFACE_TAG + 0x30, |e, a| {
            e.mem.set_u32(a[1], 0x15);
            e.mem.set_u32(a[1] + 0x10, 0);
            e.mem.set_u32(a[1] + 0x18, 640);
            e.mem.set_u32(a[1] + 0x1c, 480);
            Ret::default()
        });
        assert!(e.call(0x00e6_f100, &args![renderer, group]).bool());
        // A different size fails.
        e.mem.set_u32(data + 8, 320);
        assert!(!e.call(0x00e6_f100, &args![renderer, group]).bool());
        e.mem.set_u32(data + 8, 640);
        // Other bits fail unless the renderer's virtual +0xDC allows them.
        e.mem.set_u8(pixel_format + 1, 16);
        assert!(!e.call(0x00e6_f100, &args![renderer, group]).bool());
        e.register(RENDERER_TAG + 0xdc, |_, _| 1u32.into_ret());
        assert!(e.call(0x00e6_f100, &args![renderer, group]).bool());
        // Too many buffers.
        e.register(GROUP_TAG + 0xa4, |_, _| 5u32.into_ret());
        assert!(!e.call(0x00e6_f100, &args![renderer, group]).bool());
        // A depth buffer of the class without a surface fails.
        e.register(GROUP_TAG + 0xa4, |_, _| 1u32.into_ret());
        let depth = object(&mut e, BUFFER_TAG, 0x20);
        e.register(BUFFER_TAG + 0xc, |_, _| {
            DEPTH_BUFFER_RTTI_0126F5AC.into_ret()
        });
        e.register_double(GROUP_TAG + 0xcc, move |_, _| depth.into_ret());
        assert!(!e.call(0x00e6_f100, &args![renderer, group]).bool());
        e.mem.set_u32(depth + 0x14, 1);
        assert!(e.call(0x00e6_f100, &args![renderer, group]).bool());
        let _ = this;
    }

    #[test]
    fn fn_00e6f260_matches_the_depth_buffer() {
        let (mut e, this) = engine();
        e.map(DIRECT3D, 4);
        let direct3d = object(&mut e, DEVICE_TAG, 0x10);
        e.mem.set_u32(DIRECT3D, direct3d);
        e.mem.set_u32(this + 0x5c4, 1);
        e.mem.set_u32(this + 0x5c8, 2);
        e.mem.set_u32(this + 0x5d0, 0x16);
        let make = |e: &mut Engine, w: u32, h: u32, format: u32| {
            let surface = object(e, SURFACE_TAG + format * 0x1000, 0x10);
            e.register_double(SURFACE_TAG + format * 0x1000 + 0x30, move |e, a| {
                e.mem.set_u32(a[1], format);
                Ret::default()
            });
            let data = e.mem.alloc(0x20);
            e.mem.set_u32(data + 0x14, surface);
            let buffer = e.mem.alloc(0x20);
            e.mem.set_u32(buffer + 8, w);
            e.mem.set_u32(buffer + 0xc, h);
            e.mem.set_u32(buffer + 0x10, data);
            buffer
        };
        let color = make(&mut e, 640, 480, 0x15);
        let depth = make(&mut e, 640, 480, 0x4b);
        let small = make(&mut e, 320, 480, 0x4c);
        e.register(DEVICE_TAG + 0x30, |_, _| 0u32.into_ret());
        assert!(e.call(0x00e6_f260, &args![this, color, depth]).bool());
        assert_eq!(
            calls(&e, DEVICE_TAG + 0x30),
            vec![vec![direct3d, 1, 2, 0x16, 0x15, 0x4b]]
        );
        assert!(!e.call(0x00e6_f260, &args![this, color, small]).bool());
        e.register(DEVICE_TAG + 0x30, |_, _| 0x8000_0000u32.into_ret());
        assert!(!e.call(0x00e6_f260, &args![this, color, depth]).bool());
    }

    /// A renderer with a group (+0x884) of `w` by `h` whose first buffer casts to the colour
    /// buffer class and has a surface that locks to pitch 8 and the bits `bits`.
    fn read_back_rig(w: u32, h: u32) -> (Engine, u32, u32, u32) {
        let (mut e, this) = engine();
        let group = object(&mut e, GROUP_TAG, 0x40);
        e.mem.set_u32(this + 0x884, group);
        let buffer = object(&mut e, BUFFER_TAG, 0x20);
        e.register(BUFFER_TAG + 0xc, |_, _| BUFFER_RTTI_0126F59C.into_ret());
        let surface = object(&mut e, SURFACE_TAG, 0x10);
        e.mem.set_u32(buffer + 0x14, surface);
        e.register_double(GROUP_TAG + 0x8c, move |_, _| w.into_ret());
        e.register_double(GROUP_TAG + 0x90, move |_, _| h.into_ret());
        e.register_double(GROUP_TAG + 0xc8, move |_, _| buffer.into_ret());
        e.register(GROUP_TAG + 0x9c, |_, _| 0x4400u32.into_ret());
        let bits = e.mem.alloc(0x40);
        e.register(SURFACE_TAG + 0x30, |_, _| Ret::default());
        e.register_double(SURFACE_TAG + 0x34, move |e, a| {
            e.mem.set_u32(a[1], 8);
            e.mem.set_u32(a[1] + 4, bits);
            Ret::default()
        });
        quiet(&mut e, &[SURFACE_TAG + 0x38, COPY_BYTES_00EC44D0]);
        e.map(TWO_TO_32_01099378, 4);
        e.mem
            .set_u32(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        (e, this, surface, bits)
    }

    #[test]
    fn fn_00e6f4e0_reads_the_buffer_back() {
        let (mut e, this, surface, bits) = read_back_rig(4, 2);
        e.register(NEW_OBJECT, |_, _| 0x6000u32.into_ret());
        e.map(0x6000, 0x80);
        let widths = e.mem.alloc(4);
        let offsets = e.mem.alloc(4);
        e.mem.set_u32(widths, 4);
        e.mem.set_u32(offsets, 0x10);
        e.mem.set_u32(0x6050, 0x9000);
        e.mem.set_u32(0x6054, widths);
        e.mem.set_u32(0x605c, offsets);
        e.mem.set_u32(0x6064, 2);
        e.register(PIXEL_DATA_CONSTRUCT_00A7C190, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x00e6_f4e0, &args![this, 0u32, 0u32]).u32(), 0x6000);
        assert_eq!(
            calls(&e, PIXEL_DATA_CONSTRUCT_00A7C190),
            vec![vec![0x6000, 4, 2, 0x4400, 1, 1]]
        );
        assert_eq!(calls(&e, SURFACE_TAG + 0x34)[0][2], 0);
        assert_eq!(calls(&e, SURFACE_TAG + 0x34)[0][3], 0x10);
        assert_eq!(
            calls(&e, COPY_BYTES_00EC44D0),
            vec![vec![0x9010, bits, 8], vec![0x9018, bits + 8, 8]]
        );
        assert_eq!(calls(&e, SURFACE_TAG + 0x38), vec![vec![surface]]);
        // A rectangle past the target is cut to it.
        let rect = e.mem.alloc(0x10);
        for (i, v) in [1u32, 9, 0, 1].iter().enumerate() {
            e.mem.set_u32(rect + 4 * i as u32, *v);
        }
        e.call(0x00e6_f4e0, &args![this, rect, 0u32]);
        let made = calls(&e, PIXEL_DATA_CONSTRUCT_00A7C190);
        assert_eq!(made[1][1..3], [3, 1]);
        // A left edge at the width answers 0.
        e.mem.set_u32(rect, 4);
        assert_eq!(e.call(0x00e6_f4e0, &args![this, rect, 0u32]).u32(), 0);
        // A failed lock answers 0.
        e.register(SURFACE_TAG + 0x34, |_, _| 0x8000_0000u32.into_ret());
        assert_eq!(e.call(0x00e6_f4e0, &args![this, 0u32, 0u32]).u32(), 0);
        // With the byte at +0x6FC set nothing happens.
        e.mem.set_u8(this + 0x6fc, 1);
        assert_eq!(e.call(0x00e6_f4e0, &args![this, 0u32, 0u32]).u32(), 0);
    }

    /// A source and a destination texture with renderer data (+0x10) of the given pixel
    /// formats and surfaces.
    fn copy_rig() -> (Engine, u32, u32, u32, u32) {
        let (mut e, this) = engine();
        let device = object(&mut e, DEVICE_TAG, 0x10);
        e.mem.set_u32(this + 0x288, device);
        let texture = |e: &mut Engine, surface: u32| {
            let format = e.mem.alloc(0x44);
            let data = e.mem.alloc(0x20);
            e.mem.set_u32(data + 0xc, format);
            e.mem.set_u32(data + 0x14, surface);
            let t = e.mem.alloc(0x20);
            e.mem.set_u32(t + 8, 64);
            e.mem.set_u32(t + 0xc, 32);
            e.mem.set_u32(t + 0x10, data);
            t
        };
        let source = texture(&mut e, 0x1111);
        let destination = texture(&mut e, 0x2222);
        quiet(&mut e, &[REPORT_00A29680]);
        e.map(0x8000, 0x20);
        e.register(DEVICE_TAG + 0x88, |e, a| {
            for (k, rect) in [a[2], a[4]].iter().enumerate() {
                if *rect != 0 {
                    for i in 0..4u32 {
                        let v = e.mem.u32(rect + 4 * i);
                        e.mem.set_u32(0x8000 + 0x10 * k as u32 + 4 * i, v);
                    }
                }
            }
            Ret::default()
        });
        e.register(ERROR_STRING_00E89D50, |_, _| 0x77u32.into_ret());
        (e, this, device, source, destination)
    }

    #[test]
    fn ni_dx9_renderer_fast_copy_stretches() {
        let (mut e, this, device, source, destination) = copy_rig();
        assert!(e
            .call(
                0x00e6_f6e0,
                &args![this, source, destination, 0u32, 0u32, 0u32]
            )
            .bool());
        assert_eq!(
            calls(&e, DEVICE_TAG + 0x88),
            vec![vec![device, 0x1111, 0, 0x2222, 0, 0]]
        );
        let rect = e.mem.alloc(0x10);
        for (i, v) in [2u32, 10, 4, 8].iter().enumerate() {
            e.mem.set_u32(rect + 4 * i as u32, *v);
        }
        e.call(
            0x00e6_f6e0,
            &args![this, source, destination, rect, 5u32, 6u32],
        );
        let seen: Vec<u32> = (0..8).map(|i| e.mem.u32(0x8000 + 4 * i)).collect();
        assert_eq!(seen, vec![2, 4, 10, 8, 5, 6, 13, 10]);
        // A missing surface or different formats are reported.
        let data = e.mem.u32(destination + 0x10);
        let format = e.mem.u32(data + 0xc);
        e.mem.set_u8(format, 1);
        assert!(!e
            .call(
                0x00e6_f6e0,
                &args![this, source, destination, 0u32, 0u32, 0u32]
            )
            .bool());
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![FAST_COPY_FAILED_010EDD38, FORMATS_DIFFER_010EDD60]]
        );
        e.mem.set_u32(destination + 0x10, 0);
        assert!(!e
            .call(
                0x00e6_f6e0,
                &args![this, source, destination, 0u32, 0u32, 0u32]
            )
            .bool());
        assert_eq!(calls(&e, REPORT_00A29680)[1][1], NO_RENDERER_DATA_010EDCD8);
    }

    #[test]
    fn ni_dx9_renderer_copy_stretches_whole_textures() {
        let (mut e, this, device, source, destination) = copy_rig();
        assert!(e
            .call(
                0x00e6_f840,
                &args![this, source, destination, 0u32, 0u32, 2u32]
            )
            .bool());
        let call = calls(&e, DEVICE_TAG + 0x88)[0].clone();
        assert_eq!(
            [call[0], call[1], call[3], call[5]],
            [device, 0x1111, 0x2222, 2]
        );
        e.register(DEVICE_TAG + 0x88, |_, _| 0x8000_0000u32.into_ret());
        assert!(!e
            .call(
                0x00e6_f840,
                &args![this, source, destination, 0u32, 0u32, 7u32]
            )
            .bool());
        assert_eq!(calls(&e, DEVICE_TAG + 0x88)[1][5], 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![COPY_STRETCH_FAILED_010EDDA0, 0x77]]
        );
        let data = e.mem.u32(source + 0x10);
        e.mem.set_u32(data + 0x14, 0);
        assert!(!e
            .call(
                0x00e6_f840,
                &args![this, source, destination, 0u32, 0u32, 1u32]
            )
            .bool());
        assert_eq!(
            calls(&e, REPORT_00A29680)[1],
            vec![COPY_FAILED_010EDD7C, NULL_SURFACE_010EDCF0]
        );
    }

    #[test]
    fn fn_00e6f9b0_drops_the_renderer_data() {
        let (mut e, this) = engine();
        let texture = object(&mut e, GROUP_TAG, 0x70);
        let data = object(&mut e, DATA_TAG, 0x20);
        e.mem.set_u32(texture + 0x24, data);
        e.register(DATA_TAG + 0xa4, |_, _| 1u32.into_ret());
        quiet(&mut e, &[DATA_TAG, IMPORT_INTERLOCKED_DOWN]);
        let chain = e.mem.alloc(8);
        e.register_double(GROUP_TAG + 8, move |_, _| chain.into_ret());
        let holder = e.mem.alloc(0x20);
        let held = object(&mut e, SURFACE_TAG, 8);
        quiet(&mut e, &[SURFACE_TAG + 4]);
        e.mem.set_u32(holder + 0x10, held);
        e.register_double(GROUP_TAG + 0xa4, move |_, _| holder.into_ret());
        assert!(e.call(0x00e6_f9b0, &args![this, texture]).bool());
        assert_eq!(e.mem.u32(holder + 0x10), 0);
        assert_eq!(e.mem.u32(texture + 0x24), 0);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DOWN), vec![vec![held + 4]]);
        assert_eq!(calls(&e, DATA_TAG), vec![vec![data, 1]]);
        assert!(e.call(0x00e6_f9b0, &args![this, texture]).bool());
        assert_eq!(calls(&e, DATA_TAG).len(), 1);
    }

    #[test]
    fn callback_array_destructors() {
        let (mut e, _) = engine();
        quiet(
            &mut e,
            &[NI_FREE_00AA10F0, MEMORY_MANAGER_DEALLOCATE_00AA4060],
        );
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x3000);
        assert_eq!(e.call(0x00e7_2df0, &args![array, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), CALLBACK_ARRAY_VTABLE_010EDAEC);
        e.call(0x00e7_2e20, &args![array, 0u32]);
        assert_eq!(e.mem.u32(array), CALLBACK_ARRAY_VTABLE_010EDAF4);
        assert_eq!(
            calls(&e, NI_FREE_00AA10F0),
            vec![vec![0x3000], vec![0x3000]]
        );
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER_OBJECT_011F6238, array]]
        );
    }

    #[test]
    fn ni_xenon_renderer_register_d3d_shader_adds_at_the_head() {
        let (mut e, this) = engine();
        e.register(NEW_LIST_NODE_0043A010, |e, _| e.mem.alloc(0xc).into_ret());
        e.call(0x00e7_52b0, &args![this, 0x11u32]);
        let first = e.mem.u32(this + 0x90c);
        assert_eq!(e.mem.u32(this + 0x910), first);
        assert_eq!(e.mem.u32(first + 8), 0x11);
        e.call(0x00e7_52b0, &args![this, 0x22u32]);
        let head = e.mem.u32(this + 0x90c);
        assert_eq!(e.mem.u32(head), first);
        assert_eq!(e.mem.u32(first + 4), head);
        assert_eq!(e.mem.u32(this + 0x910), first);
        assert_eq!(e.mem.u32(this + 0x914), 2);
    }

    #[test]
    fn fn_00e7aed0_and_fn_00e7b190_find_formats_and_modes() {
        let (mut e, this) = engine();
        let entry = |e: &mut Engine, word: u32, windowed: u8, full: u8| {
            let data = e.mem.alloc(8);
            e.mem.set_u32(data, word);
            e.mem.set_u8(data + 4, windowed);
            e.mem.set_u8(data + 5, full);
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node + 8, data);
            node
        };
        let a = entry(&mut e, 0x20, 0, 1);
        let b = entry(&mut e, 0x30, 1, 0);
        e.mem.set_u32(a, b);
        e.mem.set_u32(this + 0x134, a);
        e.register(FORMAT_QUERY_00E7ACB0, |_, a| (a[0] + 1).into_ret());
        e.register(FORMAT_00E69D00, |_, a| (a[0] + 0x100).into_ret());
        assert_eq!(
            e.call(0x00e7_aed0, &args![this, 1u32, 0x31u32]).u32(),
            0x130
        );
        assert_eq!(
            e.call(0x00e7_aed0, &args![this, 0u32, 0x21u32]).u32(),
            0x120
        );
        assert_eq!(e.call(0x00e7_aed0, &args![this, 0u32, 0x31u32]).u32(), 0);
        let modes = e.mem.alloc(8);
        let mode = |e: &mut Engine, w: u32, h: u32, f: u32| {
            let m = e.mem.alloc(0x10);
            e.mem.set_u32(m, w);
            e.mem.set_u32(m + 4, h);
            e.mem.set_u32(m + 0xc, f);
            m
        };
        let m0 = mode(&mut e, 640, 480, 0x16);
        let m1 = mode(&mut e, 800, 600, 0x16);
        e.mem.set_u32(modes, m0);
        e.mem.set_u32(modes + 4, m1);
        e.mem.set_u32(this + 0x454, modes);
        e.mem.set_u16(this + 0x45a, 2);
        e.register(MODE_MERGE_00E7AF60, |_, a| (a[1] + 1).into_ret());
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 60);
        assert_eq!(
            e.call(0x00e7_b190, &args![this, 0x16u32, 800u32, 600u32, word])
                .u32(),
            m1
        );
        assert_eq!(e.mem.u32(word), 61);
        assert_eq!(calls(&e, MODE_MERGE_00E7AF60), vec![vec![m1, 60]]);
        assert_eq!(
            e.call(0x00e7_b190, &args![this, 0x17u32, 800u32, 600u32, word])
                .u32(),
            0
        );
    }
}

#[cfg(test)]
mod tests_n084_batch {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, callee: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object of `size` bytes whose vtable slot `s` holds the address `tag + s`.
    fn object(e: &mut Engine, tag: u32, size: u32) -> u32 {
        let o = e.mem.alloc(size.max(8));
        let table = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(table + slot, tag + slot);
        }
        e.mem.set_u32(o, table);
        o
    }

    /// Doubles answering 0 at `tag + slot` for every slot.
    fn quiet_table(e: &mut Engine, tag: u32) {
        for slot in (0..0x200u32).step_by(4) {
            e.register(tag + slot, |_, _| Ret::default());
        }
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &f in callees {
            e.register(f, |_, _| Ret::default());
        }
    }

    const DEVICE_TAG: u32 = 0x7100_0000;
    const LIBRARY_TAG: u32 = 0x7200_0000;
    const SHADER_TAG: u32 = 0x7300_0000;
    const DATA_TAG: u32 = 0x7400_0000;
    const MANAGER_TAG: u32 = 0x7500_0000;
    const GROUP_TAG: u32 = 0x7600_0000;

    /// A renderer with a device, a library (+0x8B8) and a manager (+0x8AC), all of whose
    /// virtuals answer 0, and a geometry with data (+0xB8), a skin instance (+0xBC), a shader
    /// (+0xC0) and a bound.
    fn rig() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb80);
        let device = object(&mut e, DEVICE_TAG, 0x10);
        let library = object(&mut e, LIBRARY_TAG, 0x10);
        let manager = object(&mut e, MANAGER_TAG, 0x10);
        e.mem.set_u32(this + 0x288, device);
        e.mem.set_u32(this + 0x8b8, library);
        e.mem.set_u32(this + 0x8ac, manager);
        for tag in [
            DEVICE_TAG,
            LIBRARY_TAG,
            SHADER_TAG,
            DATA_TAG,
            MANAGER_TAG,
            GROUP_TAG,
        ] {
            quiet_table(&mut e, tag);
        }
        let geometry = e.mem.alloc(0xd0);
        let data = object(&mut e, DATA_TAG, 0x70);
        let shader = object(&mut e, SHADER_TAG, 0x10);
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound + 0xc, 3.0f32.to_bits());
        e.mem.set_u32(geometry + 0x20, bound);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xc0, shader);
        e.call_log = Some(vec![]);
        (e, this, geometry)
    }

    #[test]
    fn fn_00e744b0_draws_unskinned_geometry() {
        let (mut e, this, geometry) = rig();
        quiet(&mut e, &[GROUP_MANAGER_00E88DC0, DRAW_00E715A0]);
        e.register(DATA_TAG + 0x90, |_, _| 3u32.into_ret());
        let data = e.mem.u32(geometry + 0xb8);
        e.mem.set_u32(data + 0x34, 0x77);
        for i in 0..13u32 {
            e.mem.set_u32(geometry + 0x68 + 4 * i, i);
        }
        e.call(0x00e7_44b0, &args![this, geometry]);
        let drawn = calls(&e, DRAW_00E715A0);
        assert_eq!(drawn.len(), 1);
        assert_eq!(drawn[0][..4], [this, geometry, data, 0]);
        assert_eq!(drawn[0][6], 0x77);
        let manager = e.mem.u32(this + 0x8ac);
        let given = &calls(&e, GROUP_MANAGER_00E88DC0)[0];
        assert_eq!(given[0], manager);
        assert_eq!(given[4] & 0xff, 0);
        assert_eq!(given[4] >> 8, geometry >> 8);
        // No count: nothing.
        e.register(DATA_TAG + 0x90, |_, _| 0u32.into_ret());
        e.call(0x00e7_44b0, &args![this, geometry]);
        assert_eq!(calls(&e, DRAW_00E715A0).len(), 1);
    }

    /// A batch of one node over `geometry` with the word +8 `pass_word`.
    fn batch(e: &mut Engine, this: u32, geometry: u32) -> u32 {
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, geometry);
        e.mem.set_u32(node + 4, 0x44);
        e.mem.set_u32(node + 8, 0x55);
        e.mem.set_u32(this + BATCH_LIST, node);
        e.mem.set_u32(this + 0x624, node);
        let shader = e.mem.u32(geometry + 0xc0);
        e.mem.set_u32(this + BATCH_SHADER, shader);
        e.mem.set_u32(shader + 4, 2);
        node
    }

    #[test]
    fn fn_00e711e0_draws_the_batch() {
        let (mut e, this, geometry) = rig();
        e.map(FREE_NODES_0126F0EC, 4);
        quiet(&mut e, &[IMPORT_INTERLOCKED_DOWN]);
        let node = batch(&mut e, this, geometry);
        let pass = e.mem.alloc(0x50);
        e.mem.set_u32(pass + 0x34, 1);
        e.mem.set_u32(pass + 0x48, 2);
        e.mem.set_u32(pass + 0x40, 6);
        e.mem.set_u32(pass + 0x3c, 5);
        e.mem.set_u32(pass + 0x18, 9);
        e.register_double(SHADER_TAG + 0x6c, move |_, _| pass.into_ret());
        let mut passes = 1;
        e.register_double(SHADER_TAG + 0x90, move |_, _| {
            let r = passes;
            passes = 0;
            r.into_ret()
        });
        let data = e.mem.u32(geometry + 0xb8);
        e.mem.set_u16(data + 0xe, 0xf123);
        e.call(0x00e7_11e0, &args![this]);
        let drawn = calls(&e, DEVICE_TAG + 0x148);
        let device = e.mem.u32(this + 0x288);
        assert_eq!(
            drawn,
            vec![
                vec![device, 5, 0, 0, 9, 0, 6],
                vec![device, 5, 0, 0, 9, 8, 6]
            ]
        );
        assert_eq!(calls(&e, SHADER_TAG + 0x60).len(), 1);
        assert_eq!(calls(&e, SHADER_TAG + 0x70).len(), 1);
        assert_eq!(calls(&e, SHADER_TAG + 0x74).len(), 1);
        assert_eq!(e.mem.u16(data + 0xe), 0xf000);
        assert_eq!(e.mem.u32(this + BATCH_SHADER), 0);
        assert_eq!(e.mem.u32(this + BATCH_LIST), 0);
        assert_eq!(e.mem.u32(FREE_NODES_0126F0EC), node);
        assert_eq!(calls(&e, LIBRARY_TAG + 0xf8).len(), 1);
    }

    #[test]
    fn fn_00e711e0_ends_a_refused_or_empty_batch() {
        let (mut e, this, geometry) = rig();
        e.map(FREE_NODES_0126F0EC, 4);
        quiet(&mut e, &[IMPORT_INTERLOCKED_DOWN]);
        e.call(0x00e7_11e0, &args![this]);
        assert!(calls(&e, SHADER_TAG + 0x54).is_empty());
        batch(&mut e, this, geometry);
        e.register(SHADER_TAG + 0x54, |_, _| 1u32.into_ret());
        e.call(0x00e7_11e0, &args![this]);
        assert!(calls(&e, SHADER_TAG + 0x58).is_empty());
        assert_eq!(e.mem.u32(this + BATCH_SHADER), 0);
        assert!(calls(&e, LIBRARY_TAG + 0xf8).is_empty());
    }

    /// A skin instance with `count` partitions of 0x2C bytes (vertex count 3 at +0x1C).
    fn skin(e: &mut Engine, count: u32) -> u32 {
        let partitions = e.mem.alloc(0x2c * count);
        for i in 0..count {
            e.mem.set_u16(partitions + 0x2c * i + 0x1c, 3);
            e.mem.set_u32(partitions + 0x2c * i + 0x28, 0x900 + i);
        }
        let partition = e.mem.alloc(0x10);
        e.mem.set_u32(partition + 8, count);
        e.mem.set_u32(partition + 0xc, partitions);
        let instance = object(e, GROUP_TAG, 0x40);
        e.mem.set_u32(instance + 0xc, partition);
        instance
    }

    #[test]
    fn fn_00e718a0_draws_each_partition() {
        let (mut e, this, geometry) = rig();
        let instance = skin(&mut e, 2);
        let pass = e.mem.alloc(0x50);
        e.mem.set_u32(pass + 0x48, 1);
        e.mem.set_u32(pass + 0x40, 4);
        e.register_double(SHADER_TAG + 0x6c, move |_, _| pass.into_ret());
        let mut passes = 1;
        e.register_double(SHADER_TAG + 0x90, move |_, _| {
            let r = passes;
            passes = 0;
            r.into_ret()
        });
        let shader = e.mem.u32(geometry + 0xc0);
        e.mem.set_u32(geometry + 0xc0, shader);
        let data = e.mem.u32(geometry + 0xb8);
        let transform = e.mem.alloc(0x34);
        let bound = e.mem.u32(geometry + 0x20);
        e.call(
            0x00e7_18a0,
            &args![this, geometry, data, instance, transform, bound],
        );
        let device = e.mem.u32(this + 0x288);
        let drawn = calls(&e, DEVICE_TAG + 0x148);
        assert_eq!(
            drawn,
            vec![
                vec![device, 4, 0, 0, 3, 0, 4],
                vec![device, 4, 0, 0, 3, 0, 4]
            ]
        );
        assert_eq!(calls(&e, SHADER_TAG + 0x60).len(), 1);
        assert_eq!(calls(&e, SHADER_TAG + 0x68).len(), 1);
        assert_eq!(calls(&e, SHADER_TAG + 0x64).len(), 1);
        assert_eq!(calls(&e, SHADER_TAG + 0x70).len(), 2);
        assert_eq!(e.mem.u32(this + 0x670), 3.0f32.to_bits());
        // No skin instance: nothing.
        e.call(
            0x00e7_18a0,
            &args![this, geometry, data, 0u32, transform, bound],
        );
        assert_eq!(calls(&e, SHADER_TAG + 0x54).len(), 1);
    }

    #[test]
    fn ni_xenon_renderer_add_to_batch_appends_nodes() {
        let (mut e, this, geometry) = rig();
        e.map(FREE_NODES_0126F0EC, 4);
        quiet(
            &mut e,
            &[
                IMPORT_INTERLOCKED_UP,
                GROUP_MANAGER_00E88DC0,
                REPORT_00A29680,
            ],
        );
        e.register(NEW_OBJECT, |e, _| e.mem.alloc(0x10).into_ret());
        e.register(DATA_TAG + 0x90, |_, _| 1u32.into_ret());
        let data = e.mem.u32(geometry + 0xb8);
        e.mem.set_u32(data + 0x34, 0x66);
        e.call(0x00e7_4a40, &args![this, geometry]);
        let shader = e.mem.u32(geometry + 0xc0);
        assert_eq!(e.mem.u32(this + BATCH_SHADER), shader);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_UP), vec![vec![shader + 4]]);
        let node = e.mem.u32(this + BATCH_LIST);
        assert_eq!(e.mem.u32(this + 0x624), node);
        assert_eq!(
            (0..4).map(|i| e.mem.u32(node + 4 * i)).collect::<Vec<_>>(),
            vec![geometry, 0, 0x66, 0]
        );
        // A skinned geometry adds a node per partition, the free list first.
        let instance = skin(&mut e, 2);
        e.mem.set_u32(geometry + 0xbc, instance);
        let free = e.mem.alloc(0x10);
        e.mem.set_u32(FREE_NODES_0126F0EC, free);
        e.mem.set_u32(data + 0x34, 0);
        e.call(0x00e7_4a40, &args![this, geometry]);
        assert_eq!(e.mem.u32(node + 0xc), free);
        assert_eq!(e.mem.u32(free + 8), 0x900);
        let last = e.mem.u32(this + 0x624);
        assert_eq!(e.mem.u32(last + 8), 0x901);
        // No shader anywhere: reported.
        let (mut e, this, geometry) = rig();
        quiet(&mut e, &[REPORT_00A29680]);
        e.mem.set_u32(geometry + 0xc0, 0);
        e.mem.set_u32(geometry + 8, 0x1234);
        e.call(0x00e7_4a40, &args![this, geometry]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![NO_BATCH_SHADER_010EE438, 0x1234]]
        );
    }
}

#[cfg(test)]
mod tests_n084_geometry {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, callee: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn object(e: &mut Engine, tag: u32, size: u32) -> u32 {
        let o = e.mem.alloc(size.max(8));
        let table = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(table + slot, tag + slot);
        }
        e.mem.set_u32(o, table);
        o
    }

    fn quiet_table(e: &mut Engine, tag: u32) {
        for slot in (0..0x200u32).step_by(4) {
            e.register(tag + slot, |_, _| Ret::default());
        }
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &f in callees {
            e.register(f, |_, _| Ret::default());
        }
    }

    const DEVICE_TAG: u32 = 0x7100_0000;
    const LIBRARY_TAG: u32 = 0x7200_0000;
    const SHADER_TAG: u32 = 0x7300_0000;
    const INDEX_TAG: u32 = 0x7400_0000;
    const GROUP_TAG: u32 = 0x7500_0000;
    const IMAGE_TAG: u32 = 0x7600_0000;
    const MANAGER_TAG: u32 = 0x7700_0000;

    /// A renderer with a device, a library and a manager whose virtuals answer 0, a
    /// stream with one buffer list entry (+0x28: the buffer 0x5000) and a stride slot, and
    /// the buffer objects the stream calls answering 0.
    fn rig() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb80);
        let device = object(&mut e, DEVICE_TAG, 0x10);
        let library = object(&mut e, LIBRARY_TAG, 0x10);
        let manager = object(&mut e, MANAGER_TAG, 0x10);
        e.mem.set_u32(this + 0x288, device);
        e.mem.set_u32(this + 0x8b8, library);
        e.mem.set_u32(this + 0x8ac, manager);
        for tag in [
            DEVICE_TAG,
            LIBRARY_TAG,
            SHADER_TAG,
            INDEX_TAG,
            GROUP_TAG,
            IMAGE_TAG,
            MANAGER_TAG,
        ] {
            quiet_table(&mut e, tag);
        }
        let stream = e.mem.alloc(0x60);
        let list = e.mem.alloc(0x18);
        e.mem.set_u32(list + 8, 0x5000);
        let lists = e.mem.alloc(4);
        e.mem.set_u32(lists, list);
        let strides = e.mem.alloc(4);
        e.mem.set_u32(stream + 0x20, 1);
        e.mem.set_u32(stream + 0x24, strides);
        e.mem.set_u32(stream + 0x28, lists);
        quiet(
            &mut e,
            &[
                GROUP_MANAGER_00E88DC0,
                STREAM_00E8EF10,
                STREAM_DECLARATION_00E8F040,
                STREAM_FVF_00E8F010,
                BUFFERS_00E8BFA0,
                BUFFERS_UNLOCK_00E8C0D0,
                REPORT_00A29680,
            ],
        );
        e.map(DIRECT3D_IS_EX, 4);
        e.map(DEFAULT_BOUND, 0x10);
        e.call_log = Some(vec![]);
        (e, this, stream, list)
    }

    #[test]
    fn fn_00e70be0_fills_and_draws_lines() {
        let (mut e, this, stream, _) = rig();
        let geometry = e.mem.alloc(0xd0);
        let data = e.mem.alloc(0x50);
        let shader = object(&mut e, SHADER_TAG, 0x10);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xc0, shader);
        e.mem.set_u16(data + 8, 2);
        e.mem.set_u32(data + 0x34, stream);
        let positions = e.mem.alloc(0x18);
        for i in 0..6u32 {
            e.mem.set_u32(positions + 4 * i, (i as f32).to_bits());
        }
        let colors = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(colors + 4 * i, 1.0f32.to_bits());
        }
        e.mem.set_u32(data + 0x20, positions);
        e.mem.set_u32(data + 0x28, colors);
        e.register(INDEX_BUFFER_00E8F520, |e, a| {
            e.mem.set_u32(a[3], 4);
            0x6600u32.into_ret()
        });
        let vertices = e.mem.alloc(0x40);
        e.register_double(BUFFERS_LOCK_00E8BFF0, move |_, _| vertices.into_ret());
        let mut passes = 1;
        e.register_double(SHADER_TAG + 0x90, move |_, _| {
            let r = passes;
            passes = 0;
            r.into_ret()
        });
        e.map(COLOR_SCALE_0101E568, 8);
        e.mem.set_u64(COLOR_SCALE_0101E568, 255.0f64.to_bits());
        e.call(0x00e7_0be0, &args![this, geometry]);
        // XYZ | DIFFUSE, 16 bytes a vertex.
        assert_eq!(calls(&e, STREAM_FVF_00E8F010), vec![vec![stream, 0x42]]);
        let strides = e.mem.u32(stream + 0x24);
        assert_eq!(e.mem.u32(strides), 0x10);
        assert_eq!(e.mem.u32(vertices + 0xc), 0xffff_ffff);
        assert_eq!(e.mem.u32(vertices + 0x10), 3.0f32.to_bits());
        let device = e.mem.u32(this + 0x288);
        assert_eq!(
            calls(&e, DEVICE_TAG + 0x148),
            vec![vec![device, 2, 0, 0, 2, 0, 2]]
        );
        assert_eq!(calls(&e, DEVICE_TAG + 0x1a0), vec![vec![device, 0x6600]]);
        assert_eq!(calls(&e, SHADER_TAG + 0x8c).len(), 1);
    }

    #[test]
    fn fn_00e70720_builds_the_strips() {
        let (mut e, this, stream, _) = rig();
        let geometry = e.mem.alloc(0xd0);
        let data = e.mem.alloc(0x70);
        let shader = object(&mut e, SHADER_TAG, 0x10);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xc0, shader);
        e.mem.set_u16(data + 0x50, 2);
        e.mem.set_u32(data + 0x34, stream);
        e.mem.set_u16(data + 8, 1);
        let state = e.mem.alloc(0x10);
        let property = e.mem.alloc(0x20);
        e.mem.set_u32(state, property);
        e.mem.set_u32(this + 0xc, state);
        e.register(STRIP_LENGTH_00C25DB0, |_, a| (2 + a[1]).into_ret());
        e.register(STRIP_WORD_00C25DD0, |_, _| 3u32.into_ret());
        e.register_double(BUFFERS_LOCK_00E8BFF0, move |_, _| 0x7000u32.into_ret());
        let index = object(&mut e, INDEX_TAG, 0x10);
        e.register_double(INDEX_BUFFER_00E8FC50, move |_, _| index.into_ret());
        e.register(STRIP_VERTICES_00E6D3A0, |e, a| {
            e.mem.set_u32(a[4], 16);
            e.mem.set_u32(a[5], 22);
            Ret::default()
        });
        e.call(0x00e7_0720, &args![this, geometry]);
        // Strips of 2 and 3 vertices: 20 vertices; 8 + 2 + 2 + 12 + 2 = 26 indices.
        let locked = calls(&e, BUFFERS_LOCK_00E8BFF0);
        assert_eq!(locked[0][3], 20 * 9 * 4);
        let made = calls(&e, INDEX_BUFFER_00E8FC50);
        assert_eq!(made[0][1..], [28, 0, 0x65, 1]);
        assert_eq!(e.mem.u32(stream + 0x34), index);
        let device = e.mem.u32(this + 0x288);
        assert_eq!(
            calls(&e, DEVICE_TAG + 0x148),
            vec![vec![device, 5, 0, 0, 16, 0, 20]]
        );
        assert_eq!(e.mem.u32(stream + 0x18), 16);
        assert_eq!(e.mem.u32(stream + 4), 0x01c0_0000);
        // The disabled check: a first property with bit 0 and a value not above 0.
        let third = e.mem.alloc(0x40);
        e.mem.set_u32(state + 8, third);
        e.mem.set_u8(property + 0x18, 1);
        e.call(0x00e7_0720, &args![this, geometry]);
        assert_eq!(calls(&e, DEVICE_TAG + 0x148).len(), 1);
    }

    #[test]
    fn fn_00e723a0_builds_the_screen_quads() {
        let (mut e, this, stream, list) = rig();
        e.mem.set_u32(list + 0x14, 0x28);
        let strides = e.mem.u32(stream + 0x24);
        e.mem.set_u32(strides, 0x14);
        let texture = e.mem.alloc(0x20);
        e.mem.set_u16(texture + 0x10, 1);
        e.mem.set_u32(texture + 0x1c, stream);
        let rect = e.mem.alloc(0x1c);
        e.mem.set_u16(rect, 10);
        e.mem.set_u16(rect + 2, 20);
        e.mem.set_u16(rect + 4, 4);
        e.mem.set_u16(rect + 6, 8);
        e.mem.set_u16(rect + 8, 2);
        e.mem.set_u16(rect + 0xa, 1);
        for i in 0..4u32 {
            e.mem.set_u32(rect + 0xc + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(texture + 8, rect);
        let image = object(&mut e, IMAGE_TAG, 0x10);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 8, image);
        let holder_list = e.mem.alloc(4);
        e.mem.set_u32(holder_list, entry);
        let holder = e.mem.alloc(0x30);
        e.mem.set_u32(holder + 0x20, holder_list);
        e.mem.set_u32(texture + 0x14, holder);
        e.register(IMAGE_TAG + 0x94, |_, _| 4u32.into_ret());
        e.register(IMAGE_TAG + 0x98, |_, _| 8u32.into_ret());
        let group = object(&mut e, GROUP_TAG, 0x10);
        e.mem.set_u32(this + 0x888, group);
        e.register(GROUP_TAG + 0x8c, |_, _| 640u32.into_ret());
        e.register(GROUP_TAG + 0x90, |_, _| 480u32.into_ret());
        e.map(HALF_PIXEL_01011588, 8);
        e.mem.set_u64(HALF_PIXEL_01011588, 0.5f64.to_bits());
        quiet(
            &mut e,
            &[
                MEMORY_MANAGER_DEALLOCATE_00AA4060,
                VECTOR_CONSTRUCT_00401050,
                NI_FREE_00AA10F0,
                STREAM_DROP_INDICES_00E8EEE0,
            ],
        );
        e.register(MEMORY_MANAGER_ALLOCATE_00AA3E40, |e, a| {
            e.mem.alloc(a[1]).into_ret()
        });
        e.register(NI_ALLOC_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SCREEN_VERTICES_00E8C570, |_, _| 1u32.into_ret());
        e.register(SCREEN_INDEX_BUFFER_00E8F200, |_, _| 0x6700u32.into_ret());
        e.call(0x00e7_23a0, &args![this, texture, 8u32]);
        let positions = e.mem.u32(this + SCREEN_POSITIONS);
        let read = |e: &Engine, at: u32| f32::from_bits(e.mem.u32(at));
        assert_eq!(read(&e, positions), 19.5);
        assert_eq!(read(&e, positions + 4), 9.5);
        assert_eq!(read(&e, positions + 0x1c), 17.5);
        let coordinates = e.mem.u32(this + SCREEN_COORDINATES);
        assert_eq!(read(&e, coordinates), 0.25);
        assert_eq!(read(&e, coordinates + 4), 0.25);
        assert_eq!(read(&e, coordinates + 0x18), 1.25);
        assert_eq!(read(&e, coordinates + 0x1c), 1.25);
        let colors = e.mem.u32(this + SCREEN_COLORS);
        assert_eq!(e.mem.u32(colors + 0x30), 0x100);
        let indices = e.mem.u32(this + SCREEN_INDICES);
        let words: Vec<u16> = (0..6).map(|i| e.mem.u16(indices + 2 * i)).collect();
        assert_eq!(words, vec![0, 1, 2, 2, 1, 3]);
        assert_eq!(e.mem.u32(stream + 0x34), 0x6700);
        assert_eq!(e.mem.u32(stream + 0x2c), 6);
        assert_eq!(e.mem.u32(stream + 0x40), 2);
        let filled = &calls(&e, SCREEN_VERTICES_00E8C570)[0];
        assert_eq!(filled[2], 4);
        assert_eq!(filled[6], 0xf);
    }

    #[test]
    fn fn_00e736b0_resets_the_device() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb80);
        let renderer_table = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(renderer_table + slot, 0x7800_0000 + slot);
            e.register(0x7800_0000 + slot, |_, _| Ret::default());
        }
        e.mem.set_u32(this, renderer_table);
        let device = object(&mut e, DEVICE_TAG, 0x10);
        e.mem.set_u32(this + 0x288, device);
        quiet_table(&mut e, DEVICE_TAG);
        let library = object(&mut e, LIBRARY_TAG, 0x10);
        e.mem.set_u32(this + 0x8b8, library);
        quiet_table(&mut e, LIBRARY_TAG);
        let object_8b4 = object(&mut e, MANAGER_TAG, 0x10);
        e.mem.set_u32(this + 0x8b4, object_8b4);
        quiet_table(&mut e, MANAGER_TAG);
        let group = object(&mut e, GROUP_TAG, 0x10);
        e.mem.set_u32(this + 0x884, group);
        quiet_table(&mut e, GROUP_TAG);
        let buffer = e.mem.alloc(0x20);
        e.register_double(GROUP_TAG + 0xb0, move |_, _| buffer.into_ret());
        quiet(
            &mut e,
            &[
                RESET_00E8BE20,
                RESET_00E8B170,
                RESET_00E8F6D0,
                RESET_00E8CD60,
                RESET_00E7CC50,
                RESET_00E7CEC0,
                RESET_00E7CF00,
                REPORT_00A29680,
                MAP_SET_AT,
                SAMPLER_SET_UP_00E6DFC0,
                SET_UP_00E6C550,
                IMPORT_INTERLOCKED_UP,
                IMPORT_INTERLOCKED_DOWN,
                FIXED_STRING_MAKE_00A5B690,
                SOURCE_TEXTURE_00A5FF10,
            ],
        );
        // What the texture defaults (`00e6ec90`, run for real) need.
        e.map(EMPTY_FIXED_STRING_0109B220, 4);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let pixels = object(&mut e, 0x7a00_0000, 0x74);
        quiet_table(&mut e, 0x7a00_0000);
        let offsets = e.mem.alloc(4);
        let bytes = e.mem.alloc(8);
        e.mem.set_u32(pixels + 0x5c, offsets);
        e.mem.set_u32(pixels + 0x50, bytes);
        e.register_double(PIXEL_DATA_CONSTRUCT_00A7C190, move |_, _| pixels.into_ret());
        let format = e.mem.alloc(4);
        e.mem.set_u32(this + 0x860, format);
        // One callback, answering whether it was given 1 (before the reset).
        let functions = e.mem.alloc(4);
        let words = e.mem.alloc(4);
        e.mem.set_u32(functions, 0x7900_0000);
        e.mem.set_u32(words, 0x77);
        e.mem.set_u32(this + 0xad8, functions);
        e.mem.set_u32(this + 0xae8, words);
        e.mem.set_u16(this + 0xade, 1);
        e.register(0x7900_0000, |_, a| u32::from(a[0] == 1).into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e7_36b0, &args![this]).bool());
        assert_eq!(calls(&e, 0x7900_0000), vec![vec![1, 0x77], vec![0, 0x77]]);
        assert_eq!(calls(&e, DEVICE_TAG + 0x40), vec![vec![device, 0x20]]);
        assert_eq!(e.mem.u32(this + 0x6f8), 1);
        assert_eq!(calls(&e, RESET_00E7CC50).len(), 3);
        assert_eq!(
            calls(&e, REPORT_00A29680).last().unwrap(),
            &vec![RESET_CALLBACK_AFTER_FAILED_010EE238]
        );
        e.register(0x7900_0000, |_, _| 1u32.into_ret());
        assert!(e.call(0x00e7_36b0, &args![this]).bool());
        assert_eq!(calls(&e, LIBRARY_TAG + 0x100).len(), 2);
        // A refusing callback stops before the reset.
        e.register(0x7900_0000, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e7_36b0, &args![this]).bool());
        assert_eq!(calls(&e, DEVICE_TAG + 0x40).len(), 2);
        assert_eq!(
            calls(&e, REPORT_00A29680).last().unwrap(),
            &vec![RESET_CALLBACK_FAILED_010EE2E8]
        );
    }
}
