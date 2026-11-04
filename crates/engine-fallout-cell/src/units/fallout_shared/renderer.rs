//! `fallout shared/renderer.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is the game's `Renderer` glue: it creates the window and the
//! Gamebryo renderer (`Renderer::Init`, `004da670`), reads the `[Display]`
//! settings into the renderer's globals, probes the Direct3D device for the
//! formats the shaders need and writes `RendererInfo.txt`. The functions
//! from `004dc5c0` on are for the next session (they are called by address
//! until then).
//!
//! Conventions in this file:
//! * The globals are named by what the code does with them; the settings by
//!   the INI name the game registers for them (`"iSize W:Display"`), found in
//!   the compiler-generated initializers (`00f3fc70` and its neighbours).
//! * A setting object keeps its value at `+4`; five getters read it:
//!   `0043d4d0` (pointer to an `int`), `00408d60` (pointer to a `bool` byte),
//!   `00403e20` (pointer to a `float`), `004503f0` (the `int`),
//!   `00454af0` (the `bool`). The helpers below call exactly the one the
//!   game calls.
//! * `00559450` is the smart pointer's `get` (it returns the word at
//!   `this`); `0066b0d0` is its assignment. The renderer lives in the smart
//!   pointer at `0x011c73b4`.
//! * The Direct3D 9 object (`004dc040`) is called through its vtable in the
//!   C style: the object is both `this` and the first stack word, so the
//!   uniform form `e.vcall(object, slot, ...)` is exactly the same call.
//!   Slot `0x28` takes (adapter, device type, adapter format, usage,
//!   resource type, format) in that order and returns an `HRESULT`.
//! * x87: `FILD`/`FDIV`/`FSTP float` sequences compute in `f64` and round to
//!   `f32` at each store. `FISTP` under round-toward-zero is `as i64`.
//! * Not translated: the C++ exception frame (`FS:[0]` chain) of
//!   `Renderer::Init`; the scope guard's destructor still runs on every
//!   path.
//! * Functions of this unit that a later session translates (`004dc710`,
//!   `004dc5c0`, `004dc650`, `004dcdb0`, `004dce30`, `004dd180`, `004de100`,
//!   `004de2d0`, ...) are called by address.

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Calls outside this file (or later in this unit) ----------------------

/// Smart-pointer `get`: returns the word at `this`.
const SMART_POINTER_GET: u32 = 0x0055_9450;
/// Smart-pointer assignment (`this`, new pointer).
const SMART_POINTER_SET: u32 = 0x0066_b0d0;
/// Setting getter: pointer to the setting's `int` value (`this + 4`).
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// Setting getter: the `int` value.
const SETTING_INT_VALUE: u32 = 0x0045_03f0;
/// Setting getter: the `bool` value.
const SETTING_BOOL_VALUE: u32 = 0x0045_4af0;
/// Setting getter: pointer to the setting's `bool` byte.
const SETTING_BOOL_POINTER: u32 = 0x0040_8d60;
/// Setting getter: pointer to the setting's `float` value.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// Setting setter for an `int` setting (`this`, value).
const SETTING_INT_SET: u32 = 0x0045_ce80;
/// Setting setter for a `bool` setting (`this`, value) (`004de2d0`).
const SETTING_BOOL_SET: u32 = 0x004d_e2d0;
/// Scope guard constructor / destructor (tag, 1, source file, line).
const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
/// The scope guard is one word.
const SCOPE_GUARD_SIZE: u32 = 4;
/// The guard's tag in `Renderer::Init`.
const SCOPE_GUARD_TAG: u32 = 4;
/// `"D:\\_Fallout3\\Platforms\\Common\\Code\\Fallout Shared\\Renderer.cpp"`.
const SOURCE_FILE: u32 = 0x0102_1584;
/// Line of the scope guard in `Renderer::Init`.
const INIT_GUARD_LINE: u32 = 0x179;
/// `sprintf(buffer, size, format, ...)` (`00406d00`).
const SPRINTF: u32 = 0x0040_6d00;
/// `fopen(path, mode)` (`00ec9a47`), `fclose(file)` (`00ec9907`),
/// `fprintf(file, format, ...)` (`00ec9774`).
const FOPEN: u32 = 0x00ec_9a47;
const FCLOSE: u32 = 0x00ec_9907;
const FPRINTF: u32 = 0x00ec_9774;
/// `_stricmp` (`00ec68e4`), `strcat` (`00ec6380`), `memset` (`00ec61c0`).
const STRICMP: u32 = 0x00ec_68e4;
const STRCAT: u32 = 0x00ec_6380;
const MEMSET: u32 = 0x00ec_61c0;
/// Copies a C string into the first argument (`004046f0`).
const STRING_COPY: u32 = 0x0040_46f0;
/// Getter for a string object's characters (`00403df0`: `this->text`, null
/// for a null string).
const STRING_TEXT: u32 = 0x0040_3df0;
/// Assignment of a C string into a string setting (`005e0190`).
const STRING_SETTING_SET: u32 = 0x005e_0190;
/// Constructor of four `float`s (`00414430`: stores the four arguments at
/// `this`, returns `this`).
const FLOAT4_CONSTRUCT: u32 = 0x0041_4430;
/// `BSShaderManager::GetShaderLevelString` (Xbox PDB), `00b4f4a0`.
const SHADER_LEVEL_STRING: u32 = 0x00b4_f4a0;
/// Names of the pixel / vertex shader target for a profile index.
const SHADER_TARGET_PIXEL: u32 = 0x00b4_f380;
const SHADER_TARGET_VERTEX: u32 = 0x00b4_f3e0;
/// Configures the shader manager (`00b4f710`): two renderer words, a flag,
/// the two adapter strings and the maximum pixel shader instruction count.
const SHADER_MANAGER_CONFIGURE: u32 = 0x00b4_f710;
/// The two renderer words passed first to it.
const SHADER_MANAGER_WORD_A: u32 = 0x0118_9468;
const SHADER_MANAGER_WORD_B: u32 = 0x0118_946c;
/// Shader package number printed in the report.
const SHADER_PACKAGE: u32 = 0x00b7_faf0;
/// Whether the multisample mode is supported for a format (`004dd180`).
const MULTISAMPLE_SUPPORTED: u32 = 0x004d_d180;
/// `004dcdb0(index)`: the adapter description with that index, or null.
const ADAPTER_DESCRIPTION: u32 = 0x004d_cdb0;
/// `004dce30()`: the adapter list (created on first use).
const ADAPTER_LIST: u32 = 0x004d_ce30;
/// `004dc710(bool)`: prepares the display mode for `Renderer::Init`.
const PREPARE_DISPLAY_MODE: u32 = 0x004d_c710;
/// `004de100()`, `004de070()`, `004de0c0()`, `004de0d0()`: the Eyefinity
/// set-up; the last two return a `float` in `ST0`.
const EYEFINITY_SETUP: u32 = 0x004d_e100;
const EYEFINITY_ACTIVE: u32 = 0x004d_e070;
const EYEFINITY_WIDTH: u32 = 0x004d_e0c0;
const EYEFINITY_HEIGHT: u32 = 0x004d_e0d0;
/// The creating call of `Renderer::Init` (`00e76210`, 15 words) and the
/// objects it works with.
const CREATE_RENDERER: u32 = 0x00e7_6210;
const NEW_OBJECT: u32 = 0x00aa_13e0;
const IMAGE_CONVERTER_CONSTRUCT: u32 = 0x00a7_b110;
const IMAGE_CONVERTER_INSTALL: u32 = 0x00a7_6b60;
/// `D3DPERF_SetOptions`-style thunk (`stdcall`, one word).
const SET_PERF_OPTIONS: u32 = 0x009f_9968;
/// `00717e50`: `this + 4`.
const ADAPTER_SECOND_NAME: u32 = 0x0071_7e50;
/// The shadow filter value of `004dc0c0` (`0042f5a0`, `00403940`).
const SHADOW_FILTER_MAP: u32 = 0x0042_f5a0;
const SHADOW_FILTER_BUILD: u32 = 0x0040_3940;
/// `00647b70(a, b)` on the two actor shadow counts.
const ACTOR_SHADOW_COUNT_PAIR: u32 = 0x0064_7b70;
/// Vendor library thunks used for the multi-GPU probe, and a getter whose
/// result is not used (`0043c4b0`).
const GPU_PROBE_INIT: u32 = 0x009f_ea2a;
const GPU_PROBE_OPEN: u32 = 0x009f_ead0;
const GPU_PROBE_ENUMERATE: u32 = 0x009f_ead6;
const GPU_PROBE_NAMES_A: u32 = 0x009f_eae8;
const GPU_PROBE_NAMES_B: u32 = 0x009f_eae2;
const GPU_PROBE_QUERY: u32 = 0x009f_ebd8;
const GPU_PROBE_DEVICE_WORD: u32 = 0x0043_c4b0;
/// Imports (call slots): `CreateWindowExA`, `GetClassLongA`,
/// `GetWindowLongA`, `AdjustWindowRect`, `SetWindowPos`, `GetSystemMetrics`,
/// `LoadLibraryA`, `GetProcAddress`, `FreeLibrary`.
const IMPORT_CREATE_WINDOW_EX: u32 = 0x00fd_f2b8;
const IMPORT_GET_CLASS_LONG: u32 = 0x00fd_f2ec;
const IMPORT_GET_WINDOW_LONG: u32 = 0x00fd_f2e8;
const IMPORT_ADJUST_WINDOW_RECT: u32 = 0x00fd_f310;
const IMPORT_SET_WINDOW_POS: u32 = 0x00fd_f2a4;
const IMPORT_GET_SYSTEM_METRICS: u32 = 0x00fd_f2b0;
const IMPORT_LOAD_LIBRARY: u32 = 0x00fd_f0b0;
const IMPORT_GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
const IMPORT_FREE_LIBRARY: u32 = 0x00fd_f0a8;
/// `Renderer::Kill`'s helpers: shut-down (`00b54630`), reference count
/// (`00726070`), a no-argument call (`00461300`) and the log (`005b5e40`).
const RENDERER_SHUTDOWN: u32 = 0x00b5_4630;
const REFERENCE_COUNT: u32 = 0x0072_6070;
const KILL_NOTE: u32 = 0x0046_1300;
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// Calls of `004dc360` (display mode change).
const RENDERER_GETTER: u32 = 0x004b_c3f0;
const SHADER_ACCUMULATOR: u32 = 0x00b4_f5c0;
const ACCUMULATOR_RESET: u32 = 0x00b6_31d0;
const MOVIE_STOP: u32 = 0x00ec_25e0;
const INTERFACE_SHUTDOWN: u32 = 0x0070_2330;
const RESIZE_SWAP_CHAIN: u32 = 0x00e7_3eb0;
const WINDOW_HANDLE: u32 = 0x0044_ddc0;
const AFTER_RESIZE_A: u32 = 0x00a6_2030;
const AFTER_RESIZE_B: u32 = 0x00b5_78a0;
const SMART_POINTER_GLOBAL_GETTER: u32 = 0x0045_c670;
const AFTER_RESIZE_C: u32 = 0x00c5_2520;
const AFTER_RESIZE_D: u32 = 0x004d_c5c0;
const AFTER_RESIZE_E: u32 = 0x004d_c650;
const INTERFACE_INIT: u32 = 0x0070_2250;
const AFTER_RESIZE_F: u32 = 0x0070_22c0;
const AFTER_RESIZE_G: u32 = 0x0070_53f0;
/// The no-effect call of `004dc560` / `004dc590` (`00483710`).
const NO_EFFECT: u32 = 0x0048_3710;

// ---- Constants ------------------------------------------------------------

/// `2.0` (`double`).
const TWO: u32 = 0x0101_1590;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `0.75` (`double`): the 4:3 height-to-width ratio.
const FOUR_BY_THREE: u32 = 0x0101_de30;
/// `"yes"` / `"no"` in `RendererInfo.txt`.
const YES: u32 = 0x0102_1388;
const NO: u32 = 0x0102_1384;
/// `"%sRendererInfo.txt"`, and the `fopen` modes `"w"` and `"a"`.
const INFO_FILE_FORMAT: u32 = 0x0102_1570;
const MODE_WRITE: u32 = 0x0102_156c;
const MODE_APPEND: u32 = 0x0102_1568;
/// `"ATIMGPUD.dll"` and `"AtiQueryMgpuCount"`.
const ATI_LIBRARY: u32 = 0x0102_148c;
const ATI_QUERY_COUNT: u32 = 0x0102_1478;
/// Error texts of `Renderer::Init` (each passed to `004dc330`).
const MESSAGE_RENDERER_CREATE: u32 = 0x0102_14e4;
const MESSAGE_ADAPTER_DESC: u32 = 0x0102_14c8;
const MESSAGE_DEVICE_CAPS: u32 = 0x0102_149c;
/// `"Failed to Recreate Gamebryo Render in desired dimensions."`.
const MESSAGE_RECREATE: u32 = 0x0102_15c4;
/// `"Multisample setting [%i] is not supported ..."`.
const MESSAGE_MULTISAMPLE: u32 = 0x0102_1518;
/// `Renderer::Kill`'s two reference diagnostics.
const MESSAGE_ONE_REFERENCE: u32 = 0x0102_1128;
const MESSAGE_MANY_REFERENCES: u32 = 0x0102_10c8;
/// The text `004da570` puts around the device name (`01021184`).
const DEVICE_NAME_DELIMITER: u32 = 0x0102_1184;
/// Direct3D format numbers the code passes: `0x71`
/// (`D3DFMT_A16B16G16R16F`) and `0x16` (`D3DFMT_X8R8G8B8`).
const FORMAT_FP16_ARGB: u32 = 0x71;
const FORMAT_DISPLAY: u32 = 0x16;
/// FourCC `'ATOC'` (alpha to coverage).
const FORMAT_ALPHA_TO_COVERAGE: u32 = 0x434f_5441;
/// The `HRESULT` the code uses for "not available" (`0x88760866`).
const HRESULT_NOT_AVAILABLE: i32 = 0x8876_0866_u32 as i32;
/// Size of the `RendererInfo.txt` path buffer, and the capacity passed to
/// `sprintf` for it.
const PATH_BUFFER_SIZE: u32 = 0x10c;
const PATH_BUFFER_CAPACITY: u32 = 0x104;
/// `GetSystemMetrics` indices: `SM_CMONITORS`, `SM_CXSCREEN`, `SM_CYSCREEN`.
const SM_CMONITORS: u32 = 0x50;
const SM_CXSCREEN: u32 = 0;
const SM_CYSCREEN: u32 = 1;
/// `GetClassLongA` index `-8` and `GetWindowLongA` index `-16`.
const CLASS_LONG_INDEX: i32 = -8;
const WINDOW_STYLE_INDEX: i32 = -0x10;
/// Style of the render window (`0x50000000`).
const RENDER_WINDOW_STYLE: u32 = 0x5000_0000;
/// Flags (`0x40`) of the `SetWindowPos` calls.
const SET_WINDOW_POS_FLAGS: u32 = 0x40;

// ---- Settings ("name:section" as the game registers them) -----------------

/// `"iNumHWThreads:General"`.
const I_NUM_HW_THREADS: u32 = 0x011c_3ea4;
/// `"iNPatches:Display"`.
const I_N_PATCHES: u32 = 0x011c_7100;
/// `"bMTRendering:Display"`.
const B_MT_RENDERING: u32 = 0x011c_710c;
/// `"fEyeEnvMapLOD1:Display"`.
const F_EYE_ENV_MAP_LOD1: u32 = 0x011c_7160;
/// `"fSunlightDimmer:BlurShader"`.
const F_SUNLIGHT_DIMMER: u32 = 0x011c_7170;
/// `"iSize H:Display"`.
const I_SIZE_H: u32 = 0x011c_718c;
/// `"fGamma:Display"`.
const F_GAMMA: u32 = 0x011c_71ac;
/// `"bShadowsOnGrass:Display"`.
const B_SHADOWS_ON_GRASS: u32 = 0x011c_71dc;
/// `"iTexMipMapMinimum:Display"`.
const I_TEX_MIP_MAP_MINIMUM: u32 = 0x011c_7254;
/// `"iWaterMultiSamples:Display"`.
const I_WATER_MULTI_SAMPLES: u32 = 0x011c_7284;
/// `"sD3DDevice:Display"` (a string setting).
const S_D3D_DEVICE: u32 = 0x011c_72c0;
/// `"fDecalLOD1:Display"`.
const F_DECAL_LOD1: u32 = 0x011c_72cc;
/// `"iActorShadowCountInt:Display"`.
const I_ACTOR_SHADOW_COUNT_INT: u32 = 0x011c_72d8;
/// `"iActorShadowCountExt:Display"`.
const I_ACTOR_SHADOW_COUNT_EXT: u32 = 0x011c_72e4;
/// `"fEyeEnvMapLOD2:Display"`.
const F_EYE_ENV_MAP_LOD2: u32 = 0x011c_72f0;
/// `"fSkinnedDecalLOD1:Display"`.
const F_SKINNED_DECAL_LOD1: u32 = 0x011c_72fc;
/// `"b30GrassVS:Grass"`.
const B_30_GRASS_VS: u32 = 0x011c_7308;
/// `"fSkinnedDecalLOD2:Display"`.
const F_SKINNED_DECAL_LOD2: u32 = 0x011c_7344;
/// `"fShadowFadeTime:Display"`.
const F_SHADOW_FADE_TIME: u32 = 0x011c_735c;
/// `"fEnvMapLOD1:Display"`.
const F_ENV_MAP_LOD1: u32 = 0x011c_7374;
/// `"iAdapter:Display"`.
const I_ADAPTER: u32 = 0x011c_738c;
/// `"bAllow20HairShader:Display"`.
const B_ALLOW_20_HAIR_SHADER: u32 = 0x011c_73a4;
/// The renderer's smart pointer.
const RENDERER_POINTER: u32 = 0x011c_73b4;
/// `"fInterfaceTintB:Interface"`.
const F_INTERFACE_TINT_B: u32 = 0x011c_73d0;
/// `"iSize W:Display"`.
const I_SIZE_W: u32 = 0x011c_73dc;
/// `"fEnvMapLOD2:Display"`.
const F_ENV_MAP_LOD2: u32 = 0x011c_73e8;
/// `"iMultiSample:Display"`.
const I_MULTI_SAMPLE: u32 = 0x011c_73f4;
/// `"fShadowLODRange:Display"`.
const F_SHADOW_LOD_RANGE: u32 = 0x011c_7400;
/// `"bAllowScreenShot:Display"`.
const B_ALLOW_SCREEN_SHOT: u32 = 0x011c_7418;
/// `"fDecalLOD2:Display"`.
const F_DECAL_LOD2: u32 = 0x011c_7428;
/// `"fLODNoiseMipBias:Display"`.
const F_LOD_NOISE_MIP_BIAS: u32 = 0x011c_7464;
/// `"bUseResolvableDepth:Display"`.
const B_USE_RESOLVABLE_DEPTH: u32 = 0x011c_747c;
/// `"fSunlightDimmer:BlurShaderHDR"`.
const F_SUNLIGHT_DIMMER_HDR: u32 = 0x011c_7488;
/// `"bForce1XShaders:Display"`.
const B_FORCE_1X_SHADERS: u32 = 0x011c_74e0;
/// `"bUseRefractionShader:Display"`.
const B_USE_REFRACTION_SHADER: u32 = 0x011c_7504;
/// `"fGrassDimmer:BlurShaderHDR"`.
const F_GRASS_DIMMER: u32 = 0x011c_7510;
/// `"iShadowMapResolution:Display"`.
const I_SHADOW_MAP_RESOLUTION: u32 = 0x011c_7520;
/// `"iShadowMode:Display"`.
const I_SHADOW_MODE: u32 = 0x011c_752c;
/// `"bForcePow2Textures:Display"`.
const B_FORCE_POW2_TEXTURES: u32 = 0x011c_7538;
/// `"fShadowLODStartFade:Display"`.
const F_SHADOW_LOD_START_FADE: u32 = 0x011c_7568;
/// `"fTreeDimmer:BlurShaderHDR"`.
const F_TREE_DIMMER: u32 = 0x011c_7574;
/// `"fSpecularLODStartFade:Display"`.
const F_SPECULAR_LOD_START_FADE: u32 = 0x011c_7598;
/// `"bUseFakeFullScreenMotionBlur:Display"`.
const B_USE_FAKE_FULL_SCREEN_MOTION_BLUR: u32 = 0x011c_75b0;
/// `"iLocation X:Display"`.
const I_LOCATION_X: u32 = 0x011c_75d4;
/// `"fLandLOFadeSeconds:Display"`.
const F_LAND_LO_FADE_SECONDS: u32 = 0x011c_75e0;
/// `"fInterfaceTintR:Interface"`.
const F_INTERFACE_TINT_R: u32 = 0x011c_75f8;
/// `"bCreateShaderPackage:General"`.
const B_CREATE_SHADER_PACKAGE: u32 = 0x011c_7610;
/// `"iPresentInterval:Display"`.
const I_PRESENT_INTERVAL: u32 = 0x011c_7628;
/// `"fSpecularLODRange:Display"`.
const F_SPECULAR_LOD_RANGE: u32 = 0x011c_7634;
/// `"iLocation Y:Display"`.
const I_LOCATION_Y: u32 = 0x011c_7654;
/// `"iWaterReflectHeight:Water"`.
const I_WATER_REFLECT_HEIGHT: u32 = 0x011c_7680;
/// `"fLightLODRange:Display"`.
const F_LIGHT_LOD_RANGE: u32 = 0x011c_76b4;
/// `"bDoHighDynamicRange:BlurShaderHDR"`.
const B_DO_HIGH_DYNAMIC_RANGE: u32 = 0x011c_76c0;
/// `"iShadowFilter:Display"`.
const I_SHADOW_FILTER: u32 = 0x011c_76cc;
/// `"bEnableEyefinity:Display"`.
const B_ENABLE_EYEFINITY: u32 = 0x011c_771c;
/// `"fLightLODStartFade:Display"`.
const F_LIGHT_LOD_START_FADE: u32 = 0x011c_7740;
/// `"bUseBlurShader:BlurShader"`.
const B_USE_BLUR_SHADER: u32 = 0x011c_7768;
/// `"bLODNoiseAniso:Display"`.
const B_LOD_NOISE_ANISO: u32 = 0x011c_7774;
/// `"iWaterReflectWidth:Water"`.
const I_WATER_REFLECT_WIDTH: u32 = 0x011c_7784;
/// `"bTransparencyMultisampling:Display"`.
const B_TRANSPARENCY_MULTISAMPLING: u32 = 0x011c_7790;
/// `"iTexMipMapSkip:Display"`.
const I_TEX_MIP_MAP_SKIP: u32 = 0x011c_779c;
/// `"bFull Screen:Display"`.
const B_FULL_SCREEN: u32 = 0x011c_77b4;
/// `"fInterfaceTintG:Interface"`.
const F_INTERFACE_TINT_G: u32 = 0x011c_77cc;
/// `"bUseWaterDisplacements:Water"`.
const B_USE_WATER_DISPLACEMENTS: u32 = 0x011c_7ac4;
/// `"bUseWaterShader:Water"`.
const B_USE_WATER_SHADER: u32 = 0x011c_7b20;
/// `"bUseWaterHiRes:Water"`.
const B_USE_WATER_HI_RES: u32 = 0x011c_7b48;
/// `"bUseWaterReflections:Water"`.
const B_USE_WATER_REFLECTIONS: u32 = 0x011c_7b6c;
/// Two more smart pointers cleared by `004dc560` / `004dc590`.
const SMART_POINTER_A: u32 = 0x011d_ee84;
const SMART_POINTER_B: u32 = 0x011d_eca4;

// ---- Renderer globals -----------------------------------------------------

/// The window `Renderer::Init` was given (its first parameter) and the
/// module instance (second parameter).
const PARENT_WINDOW: u32 = 0x011c_6fc0;
const MODULE_INSTANCE: u32 = 0x011c_6fc4;
/// The window the renderer draws to (the parent itself when full screen).
const RENDER_WINDOW: u32 = 0x011c_6fbc;
/// The window class name pointer used for the render window.
const WINDOW_CLASS_NAME: u32 = 0x011a_2fe8;
/// Last error text (`004dc330`).
const ERROR_MESSAGE: u32 = 0x011c_6fc8;
/// Copy of `iPresentInterval`.
const PRESENT_INTERVAL: u32 = 0x011c_6fb0;
/// Window and back buffer width and height.
const RENDER_WIDTH: u32 = 0x0118_947c;
const RENDER_HEIGHT: u32 = 0x0118_9480;
/// Display flags (bit 2 set when running full screen).
const DISPLAY_FLAGS: u32 = 0x0118_9478;
/// The multisample type chosen (0, 1, 2, 4 or 8).
const MULTISAMPLE_TYPE: u32 = 0x011c_70cc;
/// Gamma last applied, its "changed" flag and the copy next to the shader
/// constants.
const GAMMA: u32 = 0x0118_945c;
const GAMMA_CHANGED: u32 = 0x011c_6fb8;
const GAMMA_COPY: u32 = 0x011a_d83c;
/// Set when the render window is not 4:3.
const NOT_FOUR_BY_THREE: u32 = 0x011c_70eb;
/// Byte tested before the object at `renderer + 0x288` is adjusted at the
/// end of the renderer set-up.
const RENDER_TARGET_ADJUST_FLAG: u32 = 0x011c_6fba;
/// Device capability flags written by `Renderer::Init`.
const DYNAMIC_TEXTURES: u32 = 0x011c_6fb9;
const FP16_BLENDING: u32 = 0x011c_70ea;
const FP16_FILTERING: u32 = 0x011f_941f;
const ANISOTROPIC_MIN_FILTER: u32 = 0x011f_9420;
const TRANSPARENCY_MULTISAMPLING_ACTIVE: u32 = 0x011f_9421;
const BLOOM_LIGHTING: u32 = 0x011f_9422;
const HIGH_DYNAMIC_RANGE: u32 = 0x011f_941e;
const NON_POW2_TEXTURES: u32 = 0x011f_9183;
const SHADER_30_SUPPORT: u32 = 0x011f_94a4;
const SHADER_30_LIGHTING: u32 = 0x011f_94a6;
const HAIR_SHADER_20: u32 = 0x011f_91a3;
/// Multi-GPU (SLI / CrossFire) flag, and the flag it clears.
const MULTI_GPU: u32 = 0x011f_9440;
const MULTI_GPU_CLEARED_FLAG: u32 = 0x011f_9181;
/// Refraction flag (`004dc090` sets, `004dc0a0` reads).
const REFRACTION: u32 = 0x011f_9180;
/// The 15 formats probed in `Renderer::Init`, one byte each.
const FORMAT_SUPPORT_TABLE: u32 = 0x011f_9194;
/// Result of the post-pixel-shader-blending query of `D3DFMT_X8R8G8B8`.
const POST_PIXEL_BLEND_X8R8G8B8: u32 = 0x011a_d832;
/// Flag byte at `011c70ec` and its copy; the offscreen rectangle it enables.
const OFFSCREEN_ENABLED: u32 = 0x011c_70ec;
const OFFSCREEN_ENABLED_COPY: u32 = 0x011f_9426;
const OFFSCREEN_RECT: u32 = 0x011a_d840;
const OFFSCREEN_WIDTH: u32 = 0x011f_9430;
const OFFSCREEN_HEIGHT: u32 = 0x011f_9434;
/// Copies of the display settings (see `Renderer::Init`).
const WATER_MULTI_SAMPLES: u32 = 0x011a_d810;
const WATER_REFLECT_WIDTH: u32 = 0x011a_d814;
const WATER_REFLECT_HEIGHT: u32 = 0x011a_d818;
const TEX_MIP_MAP_SKIP: u32 = 0x0126_efc4;
const TEX_MIP_MAP_MINIMUM: u32 = 0x0126_efc8;
const RESOLVABLE_DEPTH: u32 = 0x011f_94a8;
const FAKE_MOTION_BLUR: u32 = 0x011f_94a9;
const SHADOW_MAP_RESOLUTION: u32 = 0x011a_d830;
const ACTOR_SHADOW_PAIR_RESULT: u32 = 0x011f_916c;
const SHADOW_FILTER_RESULT: u32 = 0x011f_948c;
const SUNLIGHT_DIMMER: u32 = 0x011f_9190;
const TREE_DIMMER: u32 = 0x011f_918c;
const GRASS_DIMMER: u32 = 0x011f_9188;
const LIGHT_LOD_START: u32 = 0x011f_9444;
const LIGHT_LOD_END: u32 = 0x011f_9448;
const SHADOW_LOD_START: u32 = 0x011f_944c;
const SHADOW_LOD_END: u32 = 0x011f_9450;
const SPECULAR_LOD_START: u32 = 0x011f_9454;
const SPECULAR_LOD_END: u32 = 0x011f_9458;
const ENV_MAP_LOD1: u32 = 0x011f_945c;
const ENV_MAP_LOD2: u32 = 0x011f_9460;
const EYE_ENV_MAP_LOD1: u32 = 0x011f_9464;
const EYE_ENV_MAP_LOD2: u32 = 0x011f_9468;
const DECAL_LOD1: u32 = 0x011f_946c;
const DECAL_LOD2: u32 = 0x011f_9470;
const SKINNED_DECAL_LOD1: u32 = 0x011f_9474;
const SKINNED_DECAL_LOD2: u32 = 0x011f_9478;
const CREATE_SHADER_PACKAGE: u32 = 0x011f_9488;
const SHADOW_FADE_TIME: u32 = 0x011a_d834;
const MT_RENDERING: u32 = 0x011f_94b4;
const LOD_NOISE_ANISO: u32 = 0x011f_9425;
const LOD_NOISE_MIP_BIAS: u32 = 0x011f_9428;
const LAND_LO_FADE_SECONDS: u32 = 0x011a_d838;
const SHADOW_MODE: u32 = 0x011f_91b0;
const MULTISAMPLE_COPY: u32 = 0x011f_9490;
const INTERFACE_TINT: u32 = 0x011f_961c;
const GRASS_VS_30: u32 = 0x011f_948a;
/// Byte cleared by `Renderer::Init` (its readers are not in this batch).
const CLEARED_FLAG_9489: u32 = 0x011f_9489;
/// Word returned by `004dc0b0` (vendor selector: `Renderer::Init`
/// compares it with 1, 2 and 3).
const VENDOR: u32 = 0x011f_94b8;
/// Words behind `004dc060`.
const VALUE_WHEN_SET: u32 = 0x011f_91bc;
const VALUE_WHEN_CLEAR: u32 = 0x011f_91c0;
/// Remaining globals read or written by the small functions.
const FLAG_011F4508: u32 = 0x011f_4508;
const FLAG_011A9594: u32 = 0x011a_9594;
const DIRECT3D_OBJECT: u32 = 0x0126_f0d8;
const RENDERER_ARG_011C70D4: u32 = 0x011c_70d4;
const RENDERER_ARG_011C70D8: u32 = 0x011c_70d8;
const RENDERER_ARG_01189464: u32 = 0x0118_9464;
const RENDERER_ARG_011C70D0: u32 = 0x011c_70d0;
const RENDERER_ARG_01189470: u32 = 0x0118_9470;
const RENDERER_ARG_01189474: u32 = 0x0118_9474;
const TABLE_011C74B8: u32 = 0x011c_74b8;
const INFO_DIRECTORY: u32 = 0x0120_2fa0;
/// Pixel width / height requested for the display-mode change (`004dc360`).
const REQUESTED_WIDTH: u32 = 0x011c_70e0;
const REQUESTED_HEIGHT: u32 = 0x011c_70e4;
/// The window object whose `+8` is the native handle (`0044ddc0`).
const WINDOW_OBJECT: u32 = 0x011d_ea0c;
/// Movie player stopped by `004dc360` (`MoviePlayer::Stop`, Xbox PDB).
const MOVIE_PLAYER: u32 = 0x0126_fac4;

// ---- Offsets of objects this unit does not own --------------------------------
//
// The adapter description (`004dcdb0`) is read at offsets only; the game's
// class name for it is not confirmed from the exe:
//   +0x204 the adapter name (inline string, `004da650`)
//   +0x460 device information for the hardware device, +0x464 for the
//          reference device (`004dc160`)
// A device information object has a handle at `+4` (`004dc1c0` tests it).
// The renderer (the content of the smart pointer `011c73b4`) has a pointer
// at `+0x288` (`004dc020`); `004dc540` assigns through the smart pointer at
// `this + 8`.
const ADAPTER_NAME_OFFSET: u32 = 0x204;
const ADAPTER_HAL_INFO_OFFSET: u32 = 0x460;
const ADAPTER_REF_INFO_OFFSET: u32 = 0x464;
const DEVICE_INFO_HANDLE_OFFSET: u32 = 4;
const RENDERER_OBJECT_OFFSET: u32 = 0x288;
const SMART_POINTER_OFFSET: u32 = 8;
// Device capability words read by `Renderer::Init` (offsets into the object
// `004dc120` returns; they coincide with fields of `D3DCAPS9`, which is not
// asserted here).
const CAPS_DYNAMIC_TEXTURE_BITS: u32 = 0x0c;
const CAPS_TEXTURE_BITS: u32 = 0x3c;
const CAPS_FILTER_BITS: u32 = 0x40;
const CAPS_VERTEX_SHADER_VERSION: u32 = 0xc4;
const CAPS_PIXEL_SHADER_VERSION: u32 = 0xcc;
const CAPS_MAX_PS_INSTRUCTIONS_LIMIT: u32 = 0x110;
const CAPS_MAX_PS_INSTRUCTIONS: u32 = 0x118;

// ---- Setting helpers --------------------------------------------------------

/// `*(int*)` of the pointer `0043d4d0(setting)` returns.
fn setting_int(e: &mut Engine, setting: u32) -> i32 {
    let pointer = e.call(SETTING_INT_POINTER, &args![setting]).u32();
    e.mem.i32(pointer)
}

/// `*(bool byte*)` of the pointer `00408d60(setting)` returns.
fn setting_flag(e: &mut Engine, setting: u32) -> u8 {
    let pointer = e.call(SETTING_BOOL_POINTER, &args![setting]).u32();
    e.mem.u8(pointer)
}

/// `*(float*)` of the pointer `00403e20(setting)` returns.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let pointer = e.call(SETTING_FLOAT_POINTER, &args![setting]).u32();
    e.mem.f32(pointer)
}

/// The `int` value getter (`004503f0`).
fn setting_value(e: &mut Engine, setting: u32) -> i32 {
    e.call(SETTING_INT_VALUE, &args![setting]).i32()
}

/// The `bool` value getter (`00454af0`).
fn setting_bool(e: &mut Engine, setting: u32) -> bool {
    e.call(SETTING_BOOL_VALUE, &args![setting]).bool()
}

/// `"yes"` or `"no"` as `RendererInfo.txt` prints a flag.
fn yes_no(flag: bool) -> u32 {
    if flag {
        YES
    } else {
        NO
    }
}

/// A `float` global copied from a `float` setting.
fn copy_float_setting(e: &mut Engine, destination: u32, setting: u32) {
    let value = setting_float(e, setting);
    e.set_global(destination, value);
}

/// `fprintf(file, format, value)`.
fn report(e: &mut Engine, file: u32, format: u32, value: u32) {
    e.call(FPRINTF, &args![file, format, value]);
}

/// Formats the `RendererInfo.txt` path into `buffer` and opens it.
fn open_info_file(e: &mut Engine, buffer: u32, mode: u32) -> u32 {
    let directory = fn_004dc110(e);
    e.call(
        SPRINTF,
        &args![buffer, PATH_BUFFER_CAPACITY, INFO_FILE_FORMAT, directory],
    );
    e.call(FOPEN, &args![buffer, mode]).u32()
}

/// Four words read back from the `float` quadruple `00414430` built.
fn read_float4(e: &Engine, base: u32) -> [u32; 4] {
    [
        e.mem.u32(base),
        e.mem.u32(base + 4),
        e.mem.u32(base + 8),
        e.mem.u32(base + 12),
    ]
}

/// Whether `Direct3D::CheckDeviceFormat(adapter 0, HAL, X8R8G8B8, usage,
/// resource type, format)` succeeds, as the code asks it: `smart pointer
/// get` first (its result is unused), then the object of `004dc040`.
fn check_device_format(e: &mut Engine, usage: u32, resource_type: u32, format: u32) -> i32 {
    e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]);
    let direct3d = fn_004dc040(e);
    e.vcall(
        direct3d,
        0x28,
        &args![0u32, 1u32, FORMAT_DISPLAY, usage, resource_type, format],
    )
    .i32()
}

// Translated from 004da4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Renderer::Kill` (Xbox PDB): shuts the renderer down (`00b54630`); when
/// `report_leaks` is set, logs how many other objects still hold a smart
/// pointer to the renderer; then clears the renderer's smart pointer.
pub fn renderer_kill(e: &mut Engine, report_leaks: u8) {
    e.call(RENDERER_SHUTDOWN, &args![report_leaks]);
    if report_leaks != 0 && e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32() != 0 {
        let renderer = e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32();
        let references = e.call(REFERENCE_COUNT, &args![renderer]).i32();
        if references != 1 {
            e.call(KILL_NOTE, &args![]);
            let renderer = e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32();
            let references = e.call(REFERENCE_COUNT, &args![renderer]).i32();
            if references == 2 {
                e.call(LOG_MESSAGE, &args![MESSAGE_ONE_REFERENCE]);
            } else {
                let renderer = e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32();
                let references = e.call(REFERENCE_COUNT, &args![renderer]).i32();
                e.call(
                    LOG_MESSAGE,
                    &args![MESSAGE_MANY_REFERENCES, references.wrapping_sub(1)],
                );
            }
        }
    }
    e.call(SMART_POINTER_SET, &args![RENDERER_POINTER, 0u32]);
}

// Translated from 004da570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the name of the first adapter against `sD3DDevice:Display`:
/// returns `true` when they differ. Otherwise (no adapter list, no adapter,
/// or the same name) rewrites the setting from the delimiter, the setting's
/// own text and the delimiter again, and returns `false`.
pub fn fn_004da570(e: &mut Engine) -> bool {
    if e.call(ADAPTER_LIST, &args![]).u32() != 0 {
        let adapter = e.call(ADAPTER_DESCRIPTION, &args![0u32]).u32();
        if adapter != 0 {
            let name = fn_004da650(e, Ptr::new(adapter));
            // The result is unused by the original (kept in a local).
            e.call(ADAPTER_SECOND_NAME, &args![adapter]);
            let current = e.call(STRING_TEXT, &args![S_D3D_DEVICE]).u32();
            if e.call(STRICMP, &args![current, name]).i32() != 0 {
                return true;
            }
        }
    }
    e.with_stack(0x110, |e, text| {
        e.call(STRING_COPY, &args![text, DEVICE_NAME_DELIMITER]);
        let current = e.call(STRING_TEXT, &args![S_D3D_DEVICE]).u32();
        e.call(STRCAT, &args![text, current]);
        e.call(STRCAT, &args![text, DEVICE_NAME_DELIMITER]);
        e.call(STRING_SETTING_SET, &args![S_D3D_DEVICE, text]);
    });
    false
}

// Translated from 004da650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The adapter name inside an adapter description: `this + 0x204`.
pub fn fn_004da650(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(ADAPTER_NAME_OFFSET))
}

// Translated from 004da670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Renderer::Init` (Xbox PDB): creates the render window and the Gamebryo
/// renderer for `window` / `instance`, probes the device and fills the
/// renderer's globals from the settings. Returns the renderer (the smart
/// pointer's content), or null after a failure (the reason is left in the
/// error text by `004dc330`).
///
/// The scope guard (`00404eb0`, tag 4, line `0x179` of `Renderer.cpp`) wraps
/// the whole body; its destructor runs on every return path.
pub fn renderer_init(e: &mut Engine, window: u32, instance: u32) -> Ptr {
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, SCOPE_GUARD_TAG, 1u32, SOURCE_FILE, INIT_GUARD_LINE],
        );
        let renderer = e.with_stack(PATH_BUFFER_SIZE, |e, path| {
            renderer_init_body(e, window, instance, path.addr())
        });
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
        Ptr::new(renderer)
    })
}

/// Places and sizes the window as `Renderer::Init` and `004dc360` do:
/// `AdjustWindowRect` for the client size `(right, bottom)` with the style
/// of `window` and the menu flag of the render window's class, then
/// `SetWindowPos` at (`iLocation X`, `iLocation Y`).
fn place_window(e: &mut Engine, window: u32, right: i32, bottom: i32) {
    e.with_stack(16, |e, rect| {
        e.mem.set_i32(rect.addr(), 0);
        e.mem.set_i32(rect.addr() + 4, 0);
        e.mem.set_i32(rect.addr() + 8, right);
        e.mem.set_i32(rect.addr() + 12, bottom);
        let render_window = e.global::<u32>(RENDER_WINDOW);
        let menu = e
            .call(
                IMPORT_GET_CLASS_LONG,
                &args![render_window, CLASS_LONG_INDEX],
            )
            .u32();
        let style = e
            .call(IMPORT_GET_WINDOW_LONG, &args![window, WINDOW_STYLE_INDEX])
            .u32();
        e.call(IMPORT_ADJUST_WINDOW_RECT, &args![rect, style, menu]);
        let left = e.mem.i32(rect.addr());
        let top = e.mem.i32(rect.addr() + 4);
        let right = e.mem.i32(rect.addr() + 8);
        let bottom = e.mem.i32(rect.addr() + 12);
        let width = right.wrapping_sub(left);
        let height = bottom.wrapping_sub(top);
        let y = setting_value(e, I_LOCATION_Y);
        let x = setting_value(e, I_LOCATION_X);
        e.call(
            IMPORT_SET_WINDOW_POS,
            &args![window, 0u32, x, y, width, height, SET_WINDOW_POS_FLAGS],
        );
    });
}

/// The multi-GPU probe for the vendor with selector 1 (`004dc0b0() == 1`):
/// asks the vendor library for the adapters, the names and the device state
/// and sets the multi-GPU flag when it reports more than one.
///
/// The library thunks (`009fea2a` .. `009febd8`) are named by address only;
/// what they call is not confirmed from the exe.
fn probe_multi_gpu_first_vendor(e: &mut Engine) {
    const ARRAY: u32 = 0x90;
    const NAMES_A: u32 = 0x290;
    const NAMES_B: u32 = 0x394;
    const COUNT_A: u32 = 0x498;
    const COUNT_B: u32 = 0x49c;
    const DEVICE_INFO: u32 = 0x4a0;
    e.with_stack(0x4c0, |e, block| {
        let at = |offset: u32| block.addr() + offset;
        e.call(GPU_PROBE_INIT, &args![]);
        // The first 0x8c bytes: a size word, then 0x88 zero bytes.
        e.call(MEMSET, &args![at(4), 0u32, 0x88u32]);
        e.mem.set_u32(at(0), 0x1008c);
        let mut status = e.call(GPU_PROBE_OPEN, &args![0u32, at(0)]).u32();
        if status != 0 {
            return;
        }
        // The number of successful queries is counted but never read.
        let mut successful = 0u32;
        let mut index = 0u32;
        while status == 0 && index < 0x80 {
            status = e
                .call(GPU_PROBE_ENUMERATE, &args![index, at(ARRAY + 4 * index)])
                .u32();
            if status == 0 {
                successful += 1;
            }
            index += 1;
        }
        let _ = successful;
        if e.call(GPU_PROBE_NAMES_A, &args![at(NAMES_A), at(COUNT_A)])
            .u32()
            != 0
        {
            return;
        }
        if e.call(GPU_PROBE_NAMES_B, &args![at(NAMES_B), at(COUNT_B)])
            .u32()
            != 0
        {
            return;
        }
        e.mem.set_u32(at(DEVICE_INFO), 0x1001c);
        e.call(GPU_PROBE_DEVICE_WORD, &args![]);
        let device = fn_004dc040(e);
        let status = e
            .call(GPU_PROBE_QUERY, &args![device, at(DEVICE_INFO)])
            .u32();
        let count_a = e.mem.u32(at(COUNT_A));
        let count_b = e.mem.u32(at(COUNT_B));
        let multi_gpu = if status == 0 {
            count_b > 0
        } else {
            count_a < count_b
        };
        if multi_gpu {
            e.set_global(MULTI_GPU, 1u8);
            e.set_global(MULTI_GPU_CLEARED_FLAG, 0u8);
        }
    });
}

/// The multi-GPU probe for the vendor with selector 2: loads `ATIMGPUD.dll`
/// and asks `AtiQueryMgpuCount` for the adapter count.
fn probe_multi_gpu_second_vendor(e: &mut Engine) {
    let library = e.call(IMPORT_LOAD_LIBRARY, &args![ATI_LIBRARY]).u32();
    if library == 0 {
        return;
    }
    let query = e
        .call(IMPORT_GET_PROC_ADDRESS, &args![library, ATI_QUERY_COUNT])
        .u32();
    if query != 0 && e.call(query, &args![]).i32() > 1 {
        e.set_global(MULTI_GPU, 1u8);
        e.set_global(MULTI_GPU_CLEARED_FLAG, 0u8);
    }
    e.call(IMPORT_FREE_LIBRARY, &args![library]);
}

/// The `fprintf` lines of the "Renderer Device Information" block of
/// `RendererInfo.txt`, in the game's order. The file stays open for the
/// caller to close.
fn write_device_report(
    e: &mut Engine,
    file: u32,
    names: (u32, u32),
    shader_level: u32,
    caps: u32,
    max_ps_instructions: u16,
    non_pow2: bool,
) {
    e.call(FPRINTF, &args![file, 0x0102_1450u32, names.0, names.1]);
    report(e, file, 0x0102_1434, shader_level);
    let pixel_version = e.mem.u32(caps + CAPS_PIXEL_SHADER_VERSION) & 0xffff;
    report(e, file, 0x0102_1418, pixel_version);
    let vertex_version = e.mem.u32(caps + CAPS_VERTEX_SHADER_VERSION) & 0xffff;
    report(e, file, 0x0102_13fc, vertex_version);
    let target = e.call(SHADER_TARGET_VERTEX, &args![]).u32();
    report(e, file, 0x0102_13e0, target);
    let target = e.call(SHADER_TARGET_PIXEL, &args![0u32]).u32();
    report(e, file, 0x0102_13c4, target);
    let target = e.call(SHADER_TARGET_PIXEL, &args![1u32]).u32();
    report(e, file, 0x0102_13a8, target);
    report(e, file, 0x0102_138c, max_ps_instructions as u32);
    let value = e.global::<u8>(SHADER_30_SUPPORT) != 0;
    report(e, file, 0x0102_1368, yes_no(value));
    let value = e.global::<u8>(SHADER_30_LIGHTING) != 0;
    report(e, file, 0x0102_134c, yes_no(value));
    report(e, file, 0x0102_1330, yes_no(non_pow2));
    let value = e.global::<u8>(FP16_BLENDING) != 0;
    report(e, file, 0x0102_1314, yes_no(value));
    let value = e.global::<u8>(FP16_FILTERING) != 0;
    report(e, file, 0x0102_12f8, yes_no(value));
    let value = e.global::<u8>(HIGH_DYNAMIC_RANGE) != 0;
    report(e, file, 0x0102_12dc, yes_no(value));
    let value = e.global::<u8>(BLOOM_LIGHTING) != 0;
    report(e, file, 0x0102_12c0, yes_no(value));
    let value = fn_004dc0a0(e) != 0;
    report(e, file, 0x0102_12a4, yes_no(value));
    let value = e.global::<u8>(HAIR_SHADER_20) != 0;
    report(e, file, 0x0102_1284, yes_no(value));
    let value = e.global::<u8>(MULTI_GPU) != 0;
    report(e, file, 0x0102_1268, yes_no(value));
    for (format, setting) in [
        (0x0102_124cu32, B_USE_WATER_SHADER),
        (0x0102_1230, B_USE_WATER_REFLECTIONS),
        (0x0102_1214, B_USE_WATER_DISPLACEMENTS),
        (0x0102_11f8, B_USE_WATER_HI_RES),
    ] {
        let value = setting_bool(e, setting);
        report(e, file, format, yes_no(value));
    }
    let multisample = e.global::<u32>(MULTISAMPLE_TYPE);
    report(e, file, 0x0102_11dc, multisample);
    let value = e.global::<u8>(TRANSPARENCY_MULTISAMPLING_ACTIVE) != 0;
    report(e, file, 0x0102_11c0, yes_no(value));
    let package = e.call(SHADER_PACKAGE, &args![]).u32();
    report(e, file, 0x0102_11a4, package);
    let threads = setting_int(e, I_NUM_HW_THREADS);
    report(e, file, 0x0102_1188, threads as u32);
}

/// The body of `Renderer::Init` inside the scope guard; `path` is the
/// 0x10c-byte buffer for the `RendererInfo.txt` path.
fn renderer_init_body(e: &mut Engine, window: u32, instance: u32, path: u32) -> u32 {
    e.set_global(PARENT_WINDOW, window);
    e.set_global(MODULE_INSTANCE, instance);

    // RendererInfo.txt is created empty.
    let file = open_info_file(e, path, MODE_WRITE);
    if file != 0 {
        e.call(FCLOSE, &args![file]);
    }

    let present_interval = setting_int(e, I_PRESENT_INTERVAL);
    e.set_global(PRESENT_INTERVAL, present_interval);
    let width = setting_value(e, I_SIZE_W);
    e.set_global(RENDER_WIDTH, width);
    let height = setting_value(e, I_SIZE_H);
    e.set_global(RENDER_HEIGHT, height);
    if setting_bool(e, B_ENABLE_EYEFINITY) {
        e.call(EYEFINITY_SETUP, &args![]);
    }
    if e.call(EYEFINITY_ACTIVE, &args![]).bool() {
        // FISTP to 64 bits under round-toward-zero; the low word is stored.
        let width = e.call(EYEFINITY_WIDTH, &args![]).f32();
        e.set_global(RENDER_WIDTH, (width as i64) as u32);
        let height = e.call(EYEFINITY_HEIGHT, &args![]).f32();
        e.set_global(RENDER_HEIGHT, (height as i64) as u32);
        let samples = if setting_int(e, I_MULTI_SAMPLE) < 2 {
            setting_int(e, I_MULTI_SAMPLE)
        } else {
            2
        };
        e.call(SETTING_INT_SET, &args![I_MULTI_SAMPLE, samples]);
    }

    if !e.call(PREPARE_DISPLAY_MODE, &args![1u32]).bool() {
        return 0;
    }

    // The render window: the given window itself when full screen, else a
    // child sized for the client area.
    if setting_bool(e, B_FULL_SCREEN) {
        let flags = e.global::<u32>(DISPLAY_FLAGS);
        e.set_global(DISPLAY_FLAGS, flags | 4);
        e.set_global(RENDER_WINDOW, window);
    } else {
        let height = fn_004dc200(e);
        let width = fn_004dc1f0(e);
        let class_name = e.global::<u32>(WINDOW_CLASS_NAME);
        let created = e
            .call(
                IMPORT_CREATE_WINDOW_EX,
                &args![
                    0u32,
                    class_name,
                    0u32,
                    RENDER_WINDOW_STYLE,
                    0u32,
                    0u32,
                    width,
                    height,
                    window,
                    0u32,
                    instance,
                    0u32
                ],
            )
            .u32();
        e.set_global(RENDER_WINDOW, created);
        let right = fn_004dc1f0(e);
        let bottom = fn_004dc200(e);
        place_window(e, window, right, bottom);
    }

    e.set_global(MULTISAMPLE_TYPE, 0u32);
    if setting_flag(e, B_ALLOW_SCREEN_SHOT) != 0 && e.global::<u32>(MULTISAMPLE_TYPE) == 0 {
        e.set_global(MULTISAMPLE_TYPE, 1u32);
    }

    // `true` when the window is full screen on a multi-monitor desktop and
    // the primary screen differs in size from the requested one.
    let mut size_differs = false;
    if setting_bool(e, B_FULL_SCREEN)
        && e.call(IMPORT_GET_SYSTEM_METRICS, &args![SM_CMONITORS])
            .i32()
            > 1
    {
        let screen_height = e.call(IMPORT_GET_SYSTEM_METRICS, &args![SM_CYSCREEN]).i32();
        let screen_width = e.call(IMPORT_GET_SYSTEM_METRICS, &args![SM_CXSCREEN]).i32();
        // (The game tests the height first, then the width.)
        size_differs = screen_height != fn_004dc200(e) || screen_width != fn_004dc1f0(e);
    }

    // The renderer is created from fifteen words, all of them getters of
    // this unit (the game pushes them in this order, so the call receives
    // them reversed).
    let word_1 = fn_004dc260(e);
    let word_2 = fn_004dc250(e);
    let word_3 = fn_004dc290(e);
    let word_4 = fn_004dc240(e);
    let word_5 = fn_004dc2a0(e);
    let word_6 = fn_004dc280(e);
    let word_7 = fn_004dc270(e);
    let word_8 = fn_004dc230(e);
    let word_9 = fn_004dc210(e);
    let word_10 = fn_004dc1e0(e);
    let word_11 = fn_004dc1e0(e);
    let word_12 = fn_004dc220(e);
    let word_13 = fn_004dc200(e);
    let word_14 = fn_004dc1f0(e);
    let created = e
        .call(
            CREATE_RENDERER,
            &args![
                word_14,
                word_13,
                word_12,
                word_11,
                word_10,
                word_9,
                word_8,
                word_7,
                word_6,
                word_5,
                word_4,
                word_3,
                word_2,
                word_1,
                size_differs as u32
            ],
        )
        .u32();
    e.call(SMART_POINTER_SET, &args![RENDERER_POINTER, created]);

    // Multisampling: the mode must be supported by the device.
    if setting_int(e, I_MULTI_SAMPLE) > 0 {
        let requested = setting_int(e, I_MULTI_SAMPLE);
        let supported = e
            .call(MULTISAMPLE_SUPPORTED, &args![requested, FORMAT_FP16_ARGB])
            .bool();
        if !supported && setting_flag(e, B_DO_HIGH_DYNAMIC_RANGE) != 0 {
            // Unsupported: say so in RendererInfo.txt (the file is not
            // closed afterwards, as in the game) and switch it off.
            e.call(SHADER_LEVEL_STRING, &args![]);
            let file = open_info_file(e, path, MODE_APPEND);
            if file != 0 {
                let requested = setting_int(e, I_MULTI_SAMPLE);
                report(e, file, MESSAGE_MULTISAMPLE, requested as u32);
            }
            e.call(SETTING_BOOL_SET, &args![B_TRANSPARENCY_MULTISAMPLING, 0u32]);
            e.set_global(TRANSPARENCY_MULTISAMPLING_ACTIVE, 0u8);
        } else {
            let mode = match setting_value(e, I_MULTI_SAMPLE) {
                2 | 3 => 2,
                4 => 4,
                8 => 8,
                _ => 0,
            };
            e.set_global(MULTISAMPLE_TYPE, mode);
        }
    }

    if e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32() == 0 {
        fn_004dc330(e, MESSAGE_RENDERER_CREATE);
        return 0;
    }
    let renderer = e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32();
    let table = fn_004dc2c0(e);
    e.vcall(renderer, 0xb0, &args![table]);
    if e.global::<u8>(RENDER_TARGET_ADJUST_FLAG) != 0 {
        let renderer = e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32();
        let object = fn_004dc020(e, Ptr::new(renderer));
        if !object.is_null() {
            let value = fn_004dc2b0(e);
            e.vcall(object.addr(), 0x13c, &args![value as f32]);
        }
    }

    // The image converter (a 0xa00-byte object) is installed.
    let memory = e.call(NEW_OBJECT, &args![0xa00u32]).u32();
    let converter = if memory != 0 {
        e.call(IMAGE_CONVERTER_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    e.call(IMAGE_CONVERTER_INSTALL, &args![converter]);
    fn_004dc050(e, 1);
    fn_004dc010(e, 0);

    let adapter_index = fn_004dc210(e);
    let adapter = e.call(ADAPTER_DESCRIPTION, &args![adapter_index]).u32();
    let device_name = fn_004da650(e, Ptr::new(adapter)).addr();
    let driver_name = e.call(ADAPTER_SECOND_NAME, &args![adapter]).u32();
    if adapter == 0 {
        fn_004dc330(e, MESSAGE_ADAPTER_DESC);
        return 0;
    }
    e.call(SET_PERF_OPTIONS, &args![1u32]);
    let caps = fn_004dc120(e, Ptr::new(adapter), 1).addr();
    if caps == 0 {
        fn_004dc330(e, MESSAGE_DEVICE_CAPS);
        return 0;
    }

    let dynamic_textures = setting_bool(e, B_FULL_SCREEN)
        && e.mem.u32(caps + CAPS_DYNAMIC_TEXTURE_BITS) & 0x20000 != 0;
    e.set_global(DYNAMIC_TEXTURES, dynamic_textures as u8);
    let mut max_ps_instructions = e.mem.u16(caps + CAPS_MAX_PS_INSTRUCTIONS);
    if max_ps_instructions > 0x60 && e.mem.i32(caps + CAPS_MAX_PS_INSTRUCTIONS_LIMIT) < 0x20 {
        max_ps_instructions = 0x60;
    }

    // Capability queries on the Direct3D object.
    let result = check_device_format(e, 0x80000, 3, FORMAT_FP16_ARGB);
    e.set_global(FP16_BLENDING, (result >= 0) as u8);
    let result = check_device_format(e, 0x20000, 3, FORMAT_FP16_ARGB);
    e.set_global(FP16_FILTERING, (result >= 0) as u8);

    let value = setting_int(e, I_WATER_MULTI_SAMPLES);
    e.set_global(WATER_MULTI_SAMPLES, value);
    let value = setting_int(e, I_WATER_REFLECT_WIDTH);
    e.set_global(WATER_REFLECT_WIDTH, value);
    let value = setting_int(e, I_WATER_REFLECT_HEIGHT);
    e.set_global(WATER_REFLECT_HEIGHT, value);

    let filter_caps = e.mem.u32(caps + CAPS_FILTER_BITS);
    e.set_global(ANISOTROPIC_MIN_FILTER, (filter_caps & 0x400 != 0) as u8);
    let texture_caps = e.mem.u32(caps + CAPS_TEXTURE_BITS);
    let non_pow2 = texture_caps & 2 == 0 || texture_caps & 0x100 != 0;
    let non_pow2_allowed = non_pow2 && setting_flag(e, B_FORCE_POW2_TEXTURES) == 0;
    e.set_global(NON_POW2_TEXTURES, non_pow2_allowed as u8);
    let value = setting_int(e, I_TEX_MIP_MAP_SKIP);
    e.set_global(TEX_MIP_MAP_SKIP, value);
    let value = setting_int(e, I_TEX_MIP_MAP_MINIMUM);
    e.set_global(TEX_MIP_MAP_MINIMUM, value);

    let offscreen = e.global::<u8>(OFFSCREEN_ENABLED);
    e.set_global(OFFSCREEN_ENABLED_COPY, offscreen);
    if e.global::<u8>(OFFSCREEN_ENABLED) != 0 {
        // The offscreen rectangle: the fraction of the height not covered
        // by `iSize H`, halved, as a (0, 1, 1 - f, f) quadruple.
        let full_height = e.global::<u32>(RENDER_HEIGHT) as f64;
        let requested_height = setting_int(e, I_SIZE_H) as f64;
        let two: f64 = e.global(TWO);
        let height_again = e.global::<u32>(RENDER_HEIGHT) as f64;
        let fraction = (((full_height - requested_height) / height_again) / two) as f32;
        let remainder = (1.0 - fraction as f64) as f32;
        let rect = e.with_stack(16, |e, out| {
            let result = e.call(
                FLOAT4_CONSTRUCT,
                &args![out, 0.0f32, 1.0f32, remainder, fraction],
            );
            read_float4(e, result.u32())
        });
        for (i, word) in rect.into_iter().enumerate() {
            e.set_global(OFFSCREEN_RECT + 4 * i as u32, word);
        }
        let value = setting_int(e, I_SIZE_W);
        e.set_global(OFFSCREEN_WIDTH, value);
        let value = setting_int(e, I_SIZE_H);
        e.set_global(OFFSCREEN_HEIGHT, value);
    }

    let hdr = setting_flag(e, B_DO_HIGH_DYNAMIC_RANGE) != 0 && e.global::<u8>(FP16_BLENDING) != 0;
    e.set_global(HIGH_DYNAMIC_RANGE, hdr as u8);
    let value = setting_flag(e, B_USE_RESOLVABLE_DEPTH);
    e.set_global(RESOLVABLE_DEPTH, value);
    let value = setting_flag(e, B_USE_FAKE_FULL_SCREEN_MOTION_BLUR);
    e.set_global(FAKE_MOTION_BLUR, value);
    let bloom = e.global::<u8>(HIGH_DYNAMIC_RANGE) == 0 && setting_flag(e, B_USE_BLUR_SHADER) != 0;
    e.set_global(BLOOM_LIGHTING, bloom as u8);
    let value = setting_flag(e, B_ALLOW_20_HAIR_SHADER);
    e.set_global(HAIR_SHADER_20, value);

    let force_1x_shaders = setting_flag(e, B_FORCE_1X_SHADERS);
    let word_a = e.global::<u32>(SHADER_MANAGER_WORD_A);
    let word_b = e.global::<u32>(SHADER_MANAGER_WORD_B);
    e.call(
        SHADER_MANAGER_CONFIGURE,
        &args![
            word_a,
            word_b,
            force_1x_shaders as u32,
            device_name,
            driver_name,
            max_ps_instructions as u32
        ],
    );
    if fn_004dc060(e, 0) < 5 {
        e.call(SETTING_BOOL_SET, &args![B_SHADOWS_ON_GRASS, 0u32]);
    }
    let resolution = e
        .call(SETTING_INT_POINTER, &args![I_SHADOW_MAP_RESOLUTION])
        .u32();
    let value = e.mem.u16(resolution);
    e.set_global(SHADOW_MAP_RESOLUTION, value);

    // Alpha-to-coverage support depends on the vendor selector.
    let alpha_to_coverage: i32 = match fn_004dc0b0(e) {
        1 => check_device_format(e, 0, 1, FORMAT_ALPHA_TO_COVERAGE),
        2 => 0,
        _ => HRESULT_NOT_AVAILABLE,
    };
    let transparency = alpha_to_coverage >= 0
        && setting_flag(e, B_TRANSPARENCY_MULTISAMPLING) != 0
        && setting_int(e, I_MULTI_SAMPLE) > 1;
    e.set_global(TRANSPARENCY_MULTISAMPLING_ACTIVE, transparency as u8);

    let shadow_count_exterior = setting_int(e, I_ACTOR_SHADOW_COUNT_EXT);
    let shadow_count_interior = setting_int(e, I_ACTOR_SHADOW_COUNT_INT);
    let pair = e
        .call(
            ACTOR_SHADOW_COUNT_PAIR,
            &args![shadow_count_exterior, shadow_count_interior],
        )
        .u32();
    fn_004dc100(e, pair);

    // Multi-GPU probe.
    match fn_004dc0b0(e) {
        2 => probe_multi_gpu_second_vendor(e),
        1 => probe_multi_gpu_first_vendor(e),
        _ => {
            // Both arms of the original's `== 3` test call this getter.
            fn_004dc310(e);
        }
    }
    // (The game tests a local that is always true here before the setting.)
    let refraction = setting_flag(e, B_USE_REFRACTION_SHADER) != 0;
    fn_004dc090(e, refraction as u8);

    let height = fn_004dc200(e) as f64;
    let width = fn_004dc1f0(e) as f64;
    let four_by_three: f64 = e.global(FOUR_BY_THREE);
    e.set_global(NOT_FOUR_BY_THREE, (height / width != four_by_three) as u8);

    // The 15 formats the shaders use, probed one by one.
    const FORMATS: [u32; 15] = [
        0x17, 0x18, 0x19, 0x1a, 0x51, 0x14, 0x15, 0x16, 0x72, 0x71, 0x24, 0x74, 0x32, 0x22, 0x70,
    ];
    for (i, format) in FORMATS.into_iter().enumerate() {
        let result = check_device_format(e, 0, 3, format);
        e.set_global(FORMAT_SUPPORT_TABLE + i as u32, (result >= 0) as u8);
    }
    let result = check_device_format(e, 0x80000, 3, 0x24);
    e.set_global(POST_PIXEL_BLEND_X8R8G8B8, (result >= 0) as u8);

    let gamma = setting_float(e, F_GAMMA);
    fn_004dc2d0(e, gamma);

    // RendererInfo.txt: the device information block.
    let shader_level = e.call(SHADER_LEVEL_STRING, &args![]).u32();
    let file = open_info_file(e, path, MODE_APPEND);
    if file != 0 {
        write_device_report(
            e,
            file,
            (device_name, driver_name),
            shader_level,
            caps,
            max_ps_instructions,
            non_pow2,
        );
        e.call(FCLOSE, &args![file]);
    }

    // Lighting and level-of-detail distances.
    let sunlight = if setting_flag(e, B_DO_HIGH_DYNAMIC_RANGE) != 0 {
        setting_float(e, F_SUNLIGHT_DIMMER_HDR)
    } else {
        setting_float(e, F_SUNLIGHT_DIMMER)
    };
    e.set_global(SUNLIGHT_DIMMER, sunlight);
    copy_float_setting(e, TREE_DIMMER, F_TREE_DIMMER);
    copy_float_setting(e, GRASS_DIMMER, F_GRASS_DIMMER);
    copy_float_setting(e, LIGHT_LOD_START, F_LIGHT_LOD_START_FADE);
    let start = setting_float(e, F_LIGHT_LOD_START_FADE);
    let range = setting_float(e, F_LIGHT_LOD_RANGE);
    e.set_global(LIGHT_LOD_END, (start as f64 + range as f64) as f32);
    copy_float_setting(e, SHADOW_LOD_START, F_SHADOW_LOD_START_FADE);
    let start = setting_float(e, F_SHADOW_LOD_START_FADE);
    let range = setting_float(e, F_SHADOW_LOD_RANGE);
    e.set_global(SHADOW_LOD_END, (start as f64 + range as f64) as f32);
    copy_float_setting(e, SPECULAR_LOD_START, F_SPECULAR_LOD_START_FADE);
    let start = setting_float(e, F_SPECULAR_LOD_START_FADE);
    let range = setting_float(e, F_SPECULAR_LOD_RANGE);
    e.set_global(SPECULAR_LOD_END, (start as f64 + range as f64) as f32);
    copy_float_setting(e, ENV_MAP_LOD1, F_ENV_MAP_LOD1);
    copy_float_setting(e, ENV_MAP_LOD2, F_ENV_MAP_LOD2);
    copy_float_setting(e, EYE_ENV_MAP_LOD1, F_EYE_ENV_MAP_LOD1);
    copy_float_setting(e, EYE_ENV_MAP_LOD2, F_EYE_ENV_MAP_LOD2);
    copy_float_setting(e, DECAL_LOD1, F_DECAL_LOD1);
    copy_float_setting(e, DECAL_LOD2, F_DECAL_LOD2);
    copy_float_setting(e, SKINNED_DECAL_LOD1, F_SKINNED_DECAL_LOD1);
    copy_float_setting(e, SKINNED_DECAL_LOD2, F_SKINNED_DECAL_LOD2);
    let value = setting_flag(e, B_CREATE_SHADER_PACKAGE);
    e.set_global(CREATE_SHADER_PACKAGE, value);
    copy_float_setting(e, SHADOW_FADE_TIME, F_SHADOW_FADE_TIME);
    e.set_global(CLEARED_FLAG_9489, 0u8);
    let value = setting_flag(e, B_MT_RENDERING);
    e.set_global(MT_RENDERING, value);
    let value = setting_flag(e, B_LOD_NOISE_ANISO);
    e.set_global(LOD_NOISE_ANISO, value);
    copy_float_setting(e, LOD_NOISE_MIP_BIAS, F_LOD_NOISE_MIP_BIAS);
    copy_float_setting(e, LAND_LO_FADE_SECONDS, F_LAND_LO_FADE_SECONDS);

    // `iShadowMode` outside 0..=3 means 3.
    let mode = if setting_int(e, I_SHADOW_MODE) < 0 || setting_int(e, I_SHADOW_MODE) > 3 {
        3
    } else {
        setting_int(e, I_SHADOW_MODE)
    };
    e.set_global(SHADOW_MODE, mode);
    let multisample = e.global::<u32>(MULTISAMPLE_TYPE);
    e.set_global(MULTISAMPLE_COPY, multisample);

    // The interface tint (red, green, blue, 1.0).
    let blue = setting_float(e, F_INTERFACE_TINT_B);
    let green = setting_float(e, F_INTERFACE_TINT_G);
    let red = setting_float(e, F_INTERFACE_TINT_R);
    let tint = e.with_stack(16, |e, out| {
        let result = e.call(FLOAT4_CONSTRUCT, &args![out, red, green, blue, 1.0f32]);
        read_float4(e, result.u32())
    });
    for (i, word) in tint.into_iter().enumerate() {
        e.set_global(INTERFACE_TINT + 4 * i as u32, word);
    }
    let filter = setting_int(e, I_SHADOW_FILTER);
    fn_004dc0c0(e, filter);
    let value = setting_flag(e, B_30_GRASS_VS);
    e.set_global(GRASS_VS_30, value);

    e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32()
}

// Translated from 004dc010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a flag byte in the renderer global at `011f4508`.
pub fn fn_004dc010(e: &mut Engine, value: u8) {
    e.set_global(FLAG_011F4508, value);
}

// Translated from 004dc020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object pointer at `+0x288` of the renderer.
pub fn fn_004dc020(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr().wrapping_add(RENDERER_OBJECT_OFFSET)))
}

// Translated from 004dc040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The Direct3D 9 object the renderer was created with (global `0126f0d8`).
pub fn fn_004dc040(e: &mut Engine) -> u32 {
    e.global(DIRECT3D_OBJECT)
}

// Translated from 004dc050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a flag byte in the renderer global at `011a9594`.
pub fn fn_004dc050(e: &mut Engine, value: u8) {
    e.set_global(FLAG_011A9594, value);
}

// Translated from 004dc060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `011f91bc` when `selector` is non-zero, else the one
/// at `011f91c0`.
pub fn fn_004dc060(e: &mut Engine, selector: u8) -> i32 {
    if selector != 0 {
        e.global(VALUE_WHEN_SET)
    } else {
        e.global(VALUE_WHEN_CLEAR)
    }
}

// Translated from 004dc090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the refraction flag (read back by `004dc0a0`).
pub fn fn_004dc090(e: &mut Engine, value: u8) {
    e.set_global(REFRACTION, value);
}

// Translated from 004dc0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The refraction flag (set by `004dc090`).
pub fn fn_004dc0a0(e: &mut Engine) -> u8 {
    e.global(REFRACTION)
}

// Translated from 004dc0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The vendor selector word (`Renderer::Init` compares it with 1, 2 and 3).
pub fn fn_004dc0b0(e: &mut Engine) -> i32 {
    e.global(VENDOR)
}

// Translated from 004dc0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the shadow filter value: `00403940(0, 0042f5a0(k, value))` where
/// `k` is 0 when the word at `011f91c0` is below 5 and 2 otherwise; stored
/// at `011f948c`.
pub fn fn_004dc0c0(e: &mut Engine, value: i32) {
    let level: i32 = e.global(VALUE_WHEN_CLEAR);
    let kind: u32 = if level < 5 { 0 } else { 2 };
    let mapped = e.call(SHADOW_FILTER_MAP, &args![kind, value]).u32();
    let built = e.call(SHADOW_FILTER_BUILD, &args![0u32, mapped]).u32();
    e.set_global(SHADOW_FILTER_RESULT, built);
}

// Translated from 004dc100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at `011f916c` (the actor shadow count pair).
pub fn fn_004dc100(e: &mut Engine, value: u32) {
    e.set_global(ACTOR_SHADOW_PAIR_RESULT, value);
}

// Translated from 004dc110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The directory prefix of `RendererInfo.txt`: the address of the string at
/// `01202fa0`.
pub fn fn_004dc110(_e: &mut Engine) -> u32 {
    INFO_DIRECTORY
}

// Translated from 004dc120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004dc160(selector)` followed by `00717e50` (`+4`) when it found a
/// device information object, null otherwise.
pub fn fn_004dc120(e: &mut Engine, this: Ptr, selector: u32) -> Ptr {
    let info = fn_004dc160(e, this, selector);
    if info.is_null() {
        Ptr::NULL
    } else {
        e.call_as(ADAPTER_SECOND_NAME, &args![info])
    }
}

// Translated from 004dc160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The device information of an adapter description: the hardware one
/// (`+0x460`) when `selector` is 1, else the reference one (`+0x464`);
/// null unless that object is valid (`004dc1c0`).
pub fn fn_004dc160(e: &mut Engine, this: Ptr, selector: u32) -> Ptr {
    let offset = if selector == 1 {
        ADAPTER_HAL_INFO_OFFSET
    } else {
        ADAPTER_REF_INFO_OFFSET
    };
    let info = Ptr::new(e.mem.u32(this.addr().wrapping_add(offset)));
    if fn_004dc1c0(e, info) {
        info
    } else {
        Ptr::NULL
    }
}

// Translated from 004dc1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the device information object has its handle (`+4`) set.
pub fn fn_004dc1c0(e: &mut Engine, this: Ptr) -> bool {
    e.mem
        .u32(this.addr().wrapping_add(DEVICE_INFO_HANDLE_OFFSET))
        != 0
}

// Translated from 004dc1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The render window handle.
pub fn fn_004dc1e0(e: &mut Engine) -> u32 {
    e.global(RENDER_WINDOW)
}

// Translated from 004dc1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The render width.
pub fn fn_004dc1f0(e: &mut Engine) -> i32 {
    e.global(RENDER_WIDTH)
}

// Translated from 004dc200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The render height.
pub fn fn_004dc200(e: &mut Engine) -> i32 {
    e.global(RENDER_HEIGHT)
}

// Translated from 004dc210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `iAdapter:Display` setting.
pub fn fn_004dc210(e: &mut Engine) -> i32 {
    setting_value(e, I_ADAPTER)
}

// Translated from 004dc220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The display flags word.
pub fn fn_004dc220(e: &mut Engine) -> u32 {
    e.global(DISPLAY_FLAGS)
}

// Translated from 004dc230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c70d4`.
pub fn fn_004dc230(e: &mut Engine) -> u32 {
    e.global(RENDERER_ARG_011C70D4)
}

// Translated from 004dc240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c70d8`.
pub fn fn_004dc240(e: &mut Engine) -> u32 {
    e.global(RENDERER_ARG_011C70D8)
}

// Translated from 004dc250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `01189464`.
pub fn fn_004dc250(e: &mut Engine) -> u32 {
    e.global(RENDERER_ARG_01189464)
}

// Translated from 004dc260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c70d0`.
pub fn fn_004dc260(e: &mut Engine) -> u32 {
    e.global(RENDERER_ARG_011C70D0)
}

// Translated from 004dc270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `01189470`.
pub fn fn_004dc270(e: &mut Engine) -> u32 {
    e.global(RENDERER_ARG_01189470)
}

// Translated from 004dc280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `01189474`.
pub fn fn_004dc280(e: &mut Engine) -> u32 {
    e.global(RENDERER_ARG_01189474)
}

// Translated from 004dc290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The multisample type.
pub fn fn_004dc290(e: &mut Engine) -> u32 {
    e.global(MULTISAMPLE_TYPE)
}

// Translated from 004dc2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The present interval copy.
pub fn fn_004dc2a0(e: &mut Engine) -> u32 {
    e.global(PRESENT_INTERVAL)
}

// Translated from 004dc2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `iNPatches:Display` setting.
pub fn fn_004dc2b0(e: &mut Engine) -> i32 {
    setting_value(e, I_N_PATCHES)
}

// Translated from 004dc2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the table at `011c74b8`.
pub fn fn_004dc2c0(_e: &mut Engine) -> u32 {
    TABLE_011C74B8
}

// Translated from 004dc2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the gamma: when `gamma` differs from the stored one (unordered
/// counts as different) and is above `0.0`, stores it (and a copy) and marks
/// it changed.
pub fn fn_004dc2d0(e: &mut Engine, gamma: f32) {
    let stored: f32 = e.global(GAMMA);
    if stored != gamma {
        let zero: f64 = e.global(ZERO);
        if gamma as f64 > zero {
            e.set_global(GAMMA, gamma);
            e.set_global(GAMMA_CHANGED, 1u8);
            e.set_global(GAMMA_COPY, gamma);
        }
    }
}

// Translated from 004dc310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the multisample type is 2 or more.
pub fn fn_004dc310(e: &mut Engine) -> bool {
    e.global::<i32>(MULTISAMPLE_TYPE) >= 2
}

// Translated from 004dc330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records an error text: clears the first byte of the error buffer, then
/// copies `message` into it when it is not null.
pub fn fn_004dc330(e: &mut Engine, message: u32) {
    e.set_global(ERROR_MESSAGE, 0u8);
    if message != 0 {
        e.call(STRING_COPY, &args![ERROR_MESSAGE, message]);
    }
}

// Translated from 004dc360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes the display mode to `011c70e0` x `011c70e4` on a running
/// renderer: stops the movie, resizes the renderer through `00e73eb0`, moves
/// the window (or records the failure with `004dc330`) and re-initializes
/// the interface. Does nothing without a renderer or a requested size.
pub fn fn_004dc360(e: &mut Engine) {
    if e.call(RENDERER_GETTER, &args![]).u32() == 0 {
        return;
    }
    let requested_width = e.global::<i32>(REQUESTED_WIDTH);
    let requested_height = e.global::<i32>(REQUESTED_HEIGHT);
    if requested_width == 0 || requested_height == 0 {
        return;
    }
    let accumulator = e.call(SHADER_ACCUMULATOR, &args![]).u32();
    e.call(ACCUMULATOR_RESET, &args![accumulator]);
    e.call(SETTING_INT_SET, &args![I_SIZE_W, requested_width]);
    e.call(SETTING_INT_SET, &args![I_SIZE_H, requested_height]);
    // FILD / FIDIV: signed 32-bit operands.
    let ratio = requested_height as f64 / requested_width as f64;
    let four_by_three: f64 = e.global(FOUR_BY_THREE);
    e.set_global(NOT_FOUR_BY_THREE, (ratio != four_by_three) as u8);
    let movie_player = e.global::<u32>(MOVIE_PLAYER);
    e.call(MOVIE_STOP, &args![movie_player]);
    e.call(INTERFACE_SHUTDOWN, &args![]);
    fn_004dc560(e);
    fn_004dc590(e);
    let renderer = e.call(RENDERER_GETTER, &args![]).u32();
    fn_004dc540(e, Ptr::new(renderer), Ptr::NULL);
    let renderer = e.call(RENDERER_GETTER, &args![]).u32();
    let rebuilt = e
        .call(
            RESIZE_SWAP_CHAIN,
            &args![renderer, requested_width, requested_height],
        )
        .u32()
        != 0;
    if rebuilt {
        let window_object = e.global::<u32>(WINDOW_OBJECT);
        let window = e.call(WINDOW_HANDLE, &args![window_object]).u32();
        let right = setting_value(e, I_SIZE_W);
        let bottom = setting_value(e, I_SIZE_H);
        place_window(e, window, right, bottom);
    } else {
        fn_004dc330(e, MESSAGE_RECREATE);
    }
    e.call(AFTER_RESIZE_A, &args![]);
    e.call(AFTER_RESIZE_B, &args![]);
    let accumulator = e.call(SHADER_ACCUMULATOR, &args![]).u32();
    let renderer = e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32();
    fn_004dc540(e, Ptr::new(renderer), Ptr::new(accumulator));
    let object = e.call(SMART_POINTER_GLOBAL_GETTER, &args![]).u32();
    e.call(AFTER_RESIZE_C, &args![object]);
    e.call(AFTER_RESIZE_D, &args![]);
    e.call(AFTER_RESIZE_E, &args![]);
    e.call(INTERFACE_INIT, &args![0u32]);
    e.call(AFTER_RESIZE_F, &args![]);
    e.call(AFTER_RESIZE_G, &args![]);
}

// Translated from 004dc540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `value` to the smart pointer at `this + 8`.
pub fn fn_004dc540(e: &mut Engine, this: Ptr, value: Ptr) {
    let pointer = this.addr().wrapping_add(SMART_POINTER_OFFSET);
    e.call(SMART_POINTER_SET, &args![pointer, value]);
}

// Translated from 004dc560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the smart pointer at `011dee84`, calls the no-effect `00483710` on
/// it and clears the smart pointer.
pub fn fn_004dc560(e: &mut Engine) {
    let held = e.call(SMART_POINTER_GET, &args![SMART_POINTER_A]).u32();
    e.call(NO_EFFECT, &args![held]);
    e.call(SMART_POINTER_SET, &args![SMART_POINTER_A, 0u32]);
}

// Translated from 004dc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same as `004dc560` for the smart pointer at `011deca4`.
pub fn fn_004dc590(e: &mut Engine) {
    let held = e.call(SMART_POINTER_GET, &args![SMART_POINTER_B]).u32();
    e.call(NO_EFFECT, &args![held]);
    e.call(SMART_POINTER_SET, &args![SMART_POINTER_B, 0u32]);
}

/// The `Setting` base constructors for a byte (`0045cfb0`) and a float
/// (`0040e130`) value; the vtable and collection of the third settings
/// collection (`010184fc`, `0045d180`) and of the interface settings
/// (`01022738`, `004de490`).
const SETTING_CONSTRUCT_BYTE: u32 = 0x0045_cfb0;
const SETTING_CONSTRUCT_FLOAT: u32 = 0x0040_e130;
const PREF_SETTING_VTABLE: u32 = 0x0101_84fc;
const PREF_SETTINGS: u32 = 0x0045_d180;
const INTERFACE_SETTING_VTABLE: u32 = 0x0102_2738;
const INTERFACE_SETTINGS: u32 = 0x004d_e490;
/// Slot of a settings collection's vtable that adds a setting.
const SETTINGS_ADD_SLOT: u32 = 0x4;

// Translated from 004de2f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a boolean setting of the `0045d180` collection
/// (`thiscall`, `RET 8`; the value a byte, `MOVZX`): the byte base
/// constructor (`0045cfb0`), the vtable `010184fc`, then adds itself to the
/// collection (vtable slot +4). Returns `this`. The exception-unwinding frame
/// is not translated.
pub fn fn_004de2f0(e: &mut Engine, this: Ptr, name: u32, value: u8) -> Ptr {
    e.call(SETTING_CONSTRUCT_BYTE, &args![this, name, value as u32]);
    e.mem.set_u32(this.addr(), PREF_SETTING_VTABLE);
    let collection = e.call(PREF_SETTINGS, &[]).u32();
    e.vcall(collection, SETTINGS_ADD_SLOT, &args![this]);
    this
}

// Translated from 004de370 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a float setting of the `004de490` collection
/// (`thiscall`, `RET 8`): the float base constructor (`0040e130`, the value
/// passed on through the x87 stack: a signaling NaN comes out quiet), the
/// vtable `01022738`, then adds itself to the collection (vtable slot +4).
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_004de370(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    let nan = value & 0x7f80_0000 == 0x7f80_0000 && value & 0x007f_ffff != 0;
    let value = if nan { value | 0x0040_0000 } else { value };
    e.call(SETTING_CONSTRUCT_FLOAT, &args![this, name, value]);
    e.mem.set_u32(this.addr(), INTERFACE_SETTING_VTABLE);
    let collection = e.call(INTERFACE_SETTINGS, &[]).u32();
    e.vcall(collection, SETTINGS_ADD_SLOT, &args![this]);
    this
}

/// `004dd150` (Ghidra names it `powf`; no map name): cdecl (base, exponent), result in ST0.
const POWF: u32 = 0x004d_d150;

// Translated from 004dd130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the wrapper that forwards its two `float` arguments to
/// `powf` (`004dd150`) and leaves the result in ST0. Both go through an
/// `FLD`/`FSTP` pair, which quiets a signalling NaN.
pub fn fn_004dd130(e: &mut Engine, base: f32, exponent: f32) -> f32 {
    e.call(POWF, &args![x87_quiet(base), x87_quiet(exponent)])
        .f32()
}

/// The function `fn_004dcce0` calls on the member (`00658930`).
const CALLEE_00658930: u32 = 0x0065_8930;

// Translated from 004dcce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00658930` with `ECX` = `this + 0x450` and passes on its 16-bit result.
pub fn fn_004dcce0(e: &mut Engine, this: Ptr) -> u16 {
    e.call(CALLEE_00658930, &args![this.byte_add(0x450)]).u16()
}

// Translated from 004de290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`renderer.cpp`). `thiscall`, `RET 4`: when `this` is not null,
/// copies the float argument to `+4` of it (`FLD`/`FSTP`, a signalling NaN comes out quiet);
/// returns `this`.
pub fn fn_004de290(e: &mut Engine, this: Ptr, value: f32) -> Ptr {
    if !this.is_null() {
        e.mem
            .set_u32(this.addr().wrapping_add(4), x87_quiet_bits(value.to_bits()));
    }
    this
}

/// `SceneGraph::SceneGraph` (engine map, `00878610`): `thiscall`, `RET 0xc` (name, two
/// flags); 0xc0 bytes.
const SCENE_GRAPH_CONSTRUCT: u32 = 0x0087_8610;
/// `"Menu"` (`01021600`) and `"3DMenu"` (`01021608`): the two scene graphs' names.
const MENU_NAME: u32 = 0x0102_1600;
const MENU_3D_NAME: u32 = 0x0102_1608;
/// The smart pointers the two scene graphs are assigned to (`011dee84`, `011deca4`).
const MENU_SCENE_GRAPH: u32 = 0x011d_ee84;
const MENU_3D_SCENE_GRAPH: u32 = 0x011d_eca4;

/// A new `SceneGraph` (0xc0 bytes; null when the allocation fails) with `name` and the two
/// flag words, assigned to the smart pointer at `holder`.
fn make_scene_graph(e: &mut Engine, holder: u32, name: u32, first: u32, second: u32) {
    let memory = e.call(NEW_OBJECT, &args![0xc0u32]).u32();
    let graph = if memory != 0 {
        e.call(SCENE_GRAPH_CONSTRUCT, &args![memory, name, first, second])
            .u32()
    } else {
        0
    };
    e.call(SMART_POINTER_SET, &args![holder, graph]);
}

// Translated from 004dc5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`renderer.cpp`), no arguments: makes the `"Menu"` scene graph
/// (`SceneGraph::SceneGraph(name, 1, 0)` on 0xc0 bytes, null when the allocation fails) and
/// assigns it to the smart pointer at `011dee84`.
// Translation incomplete: 004dc5c0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn fn_004dc5c0(e: &mut Engine) {
    make_scene_graph(e, MENU_SCENE_GRAPH, MENU_NAME, 1, 0);
}

// Translated from 004dc650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`renderer.cpp`), no arguments: makes the `"3DMenu"` scene
/// graph (`SceneGraph::SceneGraph(name, 0, 0)` on 0xc0 bytes, null when the allocation fails)
/// and assigns it to the smart pointer at `011deca4`.
// Translation incomplete: 004dc650 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn fn_004dc650(e: &mut Engine) {
    make_scene_graph(e, MENU_3D_SCENE_GRAPH, MENU_3D_NAME, 0, 0);
}

// ---- Display-mode and adapter helpers (batch b005) ------------------------

/// The adapter list `004dce30` creates on first use (`00e69a60`) and keeps.
const ADAPTER_LIST_POINTER: u32 = 0x011c_70dc;
const ADAPTER_LIST_CREATE: u32 = 0x00e6_9a60;
/// The adapter list's mode count (`00658930`) and element access
/// (`00877a30`, the element's address) on its array at `+4`.
const ARRAY_COUNT: u32 = 0x0065_8930;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// Eyefinity flags (`004de070` returns the first, `004de080` both).
const EYEFINITY_FLAG: u32 = 0x011c_70ed;
const EYEFINITY_SECOND_FLAG: u32 = 0x011c_70ee;
/// `"bUse Shaders:Display"`; `004b9930` on `bFull Screen:Display` gives "not
/// full screen".
const B_USE_SHADERS: u32 = 0x011c_726c;
const WINDOWED_FROM_SETTING: u32 = 0x004b_9930;
/// `00446e10`: the `bFull Screen:Display` byte (through `00408d60`).
const FULL_SCREEN_MODE: u32 = 0x0044_6e10;
/// `00408840`: the absolute value of a `float` (cdecl, result in ST0).
const FLOAT_ABS: u32 = 0x0040_8840;
/// `_pow` (`00ec9ac0`, cdecl, two `double`s, result in ST0).
const POW: u32 = 0x00ec_9ac0;
/// `0.01` (the `float` widened to `double`), `1.25` and `1.3333334` (the
/// `float` widened), all `double`.
const ASPECT_TOLERANCE: u32 = 0x0101_6408;
const ASPECT_FIVE_BY_FOUR: u32 = 0x0102_1798;
const ASPECT_FOUR_BY_THREE: u32 = 0x0102_1790;
/// Error texts of `004dc710`.
const MESSAGE_NO_DEVICE: u32 = 0x0102_1610;
const MESSAGE_BAD_ADAPTER: u32 = 0x0102_1634;
const MESSAGE_NO_WINDOWED: u32 = 0x0102_17a0;
const MESSAGE_RESOLUTION: u32 = 0x0102_16f8;
const MESSAGE_SHADER_VERSIONS: u32 = 0x0102_1698;
const MESSAGE_NO_HARDWARE_TL: u32 = 0x0102_1660;
/// Device-capability words of the hardware device information (`+0x1C`
/// flags, `+0xC4` and `+0xCC` the vertex and pixel shader versions).
const CAPS_FLAGS_OFFSET: u32 = 0x1c;
const CAPS_VERTEX_SHADER_OFFSET: u32 = 0xc4;
const CAPS_PIXEL_SHADER_OFFSET: u32 = 0xcc;
/// The renderer's Direct3D device-type word (`+0x5C8`).
const RENDERER_DEVICE_TYPE_OFFSET: u32 = 0x5c8;
/// The light colour `004dce50` keeps (three `float`s).
const LIGHT_COLOR: u32 = 0x011c_74b8;

// Translated from 004dc710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Prepares the display mode for `Renderer::Init` (cdecl; its argument is
/// not read). Needs the adapter list and the adapter of `iAdapter`
/// (otherwise an error text through `004dc330` and false); a windowed game
/// needs a windowed-capable adapter (`004dcd30(1)`). Full screen, it walks
/// the adapter's display modes from the last: a mode of at least 32 bits in
/// the display format is taken at once when windowed, or when it matches
/// the requested size with an aspect ratio within 0.01 of the screen's
/// (or the screen is 5:4 and the mode 4:3); else the nearest larger mode is
/// remembered, whose size then becomes the render size (`OFFSCREEN_ENABLED`
/// set), or an error text when there is none. Then the hardware device
/// information must offer hardware T&L and the shader versions the shader
/// manager needs (`004dcd90`, `004dcd80`), whose versions it stores; with
/// shaders off (`bUse Shaders:Display`) or no device information the
/// versions are 0 and the shader manager is configured directly
/// (`00b4f710`). Returns whether the mode can be used.
pub fn fn_004dc710(e: &mut Engine, _unused_1: u32) -> u8 {
    let mut found = 0u8;
    let screen_width = e.call(IMPORT_GET_SYSTEM_METRICS, &args![SM_CXSCREEN]).u32();
    let screen_height = e.call(IMPORT_GET_SYSTEM_METRICS, &args![SM_CYSCREEN]).u32();
    let screen_aspect = x87_store32(x87_div(f64::from(screen_width), f64::from(screen_height)));
    if fn_004dce30(e) == 0 {
        fn_004dc330(e, MESSAGE_NO_DEVICE);
        return found;
    }
    let adapter_index = fn_004dc210(e) as u32;
    let adapter = fn_004dcdb0(e, adapter_index);
    if adapter == 0 {
        fn_004dc330(e, MESSAGE_BAD_ADAPTER);
        return found;
    }
    let adapter = Ptr::new(adapter);
    found = e.call(FULL_SCREEN_MODE, &[]).u8();
    if found == 0 {
        found = fn_004dcd30(e, adapter, 1);
        if found == 0 {
            fn_004dc330(e, MESSAGE_NO_WINDOWED);
        }
    }
    if found != 0 {
        found = 0;
        let mut index = u32::from(fn_004dcce0(e, adapter));
        let mut best = index.wrapping_add(1);
        // A local the code compares mode heights with; never set but to 0.
        let floor = 0u32;
        let tolerance = e.global::<f64>(ASPECT_TOLERANCE);
        let near = |e: &mut Engine, bits: u32| -> bool {
            let distance = e.call(FLOAT_ABS, &args![x87_quiet_bits(bits)]).st0;
            distance < tolerance
        };
        loop {
            let previous = index;
            index = index.wrapping_sub(1);
            if previous == 0 || found != 0 {
                break;
            }
            let mode = fn_004dcd00(e, adapter, index);
            if e.mem.u32(mode.wrapping_add(8)) < 0x20 {
                continue;
            }
            let format = fn_004dc270(e);
            let format = fn_004dcbe0(e, format);
            if e.mem.u32(mode.wrapping_add(0xc)) != format {
                continue;
            }
            if e.call(FULL_SCREEN_MODE, &[]).u8() == 0 {
                found = 1;
                break;
            }
            let mode_width = e.mem.u32(mode);
            let mode_height = e.mem.u32(mode.wrapping_add(4));
            let aspect = x87_store32(x87_div(f64::from(mode_width), f64::from(mode_height)));
            let five_by_four = e.global::<f64>(ASPECT_FIVE_BY_FOUR);
            let four_by_three = e.global::<f64>(ASPECT_FOUR_BY_THREE);
            let difference = x87_store32(x87_sub(x87_load32(aspect), x87_m32(screen_aspect)));
            let matched = if near(e, difference) {
                true
            } else {
                let difference = x87_store32(x87_sub(x87_load32(screen_aspect), five_by_four));
                if near(e, difference) {
                    let difference = x87_store32(x87_sub(x87_load32(aspect), four_by_three));
                    near(e, difference)
                } else {
                    false
                }
            };
            if matched {
                if fn_004dc1f0(e) as u32 == e.mem.u32(mode)
                    && fn_004dc200(e) as u32 == e.mem.u32(mode.wrapping_add(4))
                {
                    found = 1;
                    break;
                }
                if e.call(FULL_SCREEN_MODE, &[]).u8() != 0
                    && fn_004dc1f0(e) as u32 == e.mem.u32(mode)
                {
                    let difference = x87_store32(x87_sub(x87_load32(aspect), five_by_four));
                    let other_ratio = if near(e, difference) {
                        let difference = x87_store32(x87_sub(x87_load32(aspect), four_by_three));
                        !near(e, difference)
                    } else {
                        true
                    };
                    if other_ratio
                        && e.mem.u32(mode.wrapping_add(4)) > fn_004dc200(e) as u32
                        && (best > u32::from(fn_004dcce0(e, adapter))
                            || e.mem.u32(mode.wrapping_add(4)) > floor)
                    {
                        best = index;
                    }
                }
            } else if fn_004dc1f0(e) as u32 == e.mem.u32(mode)
                && fn_004dc200(e) <= e.mem.u32(mode.wrapping_add(4)) as i32
                && best > u32::from(fn_004dcce0(e, adapter))
            {
                best = index;
            }
        }
        if found == 0 && best > u32::from(fn_004dcce0(e, adapter)) {
            fn_004dc330(e, MESSAGE_RESOLUTION);
        } else if found == 0 && best <= u32::from(fn_004dcce0(e, adapter)) {
            let mode = fn_004dcd00(e, adapter, best);
            let width = e.mem.u32(mode);
            e.set_global(RENDER_WIDTH, width);
            let mode = fn_004dcd00(e, adapter, best);
            let height = e.mem.u32(mode.wrapping_add(4));
            e.set_global(RENDER_HEIGHT, height);
            found = 1;
            e.set_global(OFFSCREEN_ENABLED, 1u8);
        }
    }
    if found != 0 {
        found = 0;
        let caps = fn_004dc120(e, adapter, 1).addr();
        if caps != 0 && fn_004dcda0(e) as u8 != 0 {
            let flags = e.mem.u32(caps.wrapping_add(CAPS_FLAGS_OFFSET));
            found = u8::from(flags & 0x1_0000 != 0);
            if found != 0 {
                let vertex = e.mem.u32(caps.wrapping_add(CAPS_VERTEX_SHADER_OFFSET)) & 0xffff;
                let ok = if vertex < fn_004dcd90(e) {
                    false
                } else {
                    let pixel = e.mem.u32(caps.wrapping_add(CAPS_PIXEL_SHADER_OFFSET)) & 0xffff;
                    pixel >= fn_004dcd80(e)
                };
                found = u8::from(ok);
                if ok {
                    let vertex = e.mem.u32(caps.wrapping_add(CAPS_VERTEX_SHADER_OFFSET)) & 0xffff;
                    e.set_global(SHADER_MANAGER_WORD_B, vertex);
                    let pixel = e.mem.u32(caps.wrapping_add(CAPS_PIXEL_SHADER_OFFSET)) & 0xffff;
                    e.set_global(SHADER_MANAGER_WORD_A, pixel);
                } else {
                    fn_004dc330(e, MESSAGE_SHADER_VERSIONS);
                }
            } else {
                fn_004dc330(e, MESSAGE_NO_HARDWARE_TL);
            }
            let flags = e.mem.u32(caps.wrapping_add(CAPS_FLAGS_OFFSET));
            let patches = u32::from(flags & 0x100_0000 != 0);
            let many = u32::from(fn_004dc2b0(e) > 1);
            e.set_global(RENDER_TARGET_ADJUST_FLAG, (patches & many) as u8);
        } else {
            e.set_global(SHADER_MANAGER_WORD_B, 0u32);
            e.set_global(SHADER_MANAGER_WORD_A, 0u32);
            let force = e
                .call(SETTING_BOOL_POINTER, &args![B_FORCE_1X_SHADERS])
                .u32();
            let force = u32::from(e.mem.u8(force));
            let word_b: u32 = e.global(SHADER_MANAGER_WORD_B);
            let word_a: u32 = e.global(SHADER_MANAGER_WORD_A);
            e.call(
                SHADER_MANAGER_CONFIGURE,
                &args![word_a, word_b, force, 0u32, 0u32, 0u32],
            );
            if fn_004dcda0(e) as u8 == 0 {
                found = 1;
            }
        }
    }
    found
}

// Translated from 004dcbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The Direct3D format of a display-format index (cdecl; the exe's jump
/// table at 004dcc8c): 1..=11 give `0x14..=0x1E`, 12 `0x6F`, 13 `0x70`, 14
/// `0x24`, 15..=18 `0x71..=0x74`; 0 and anything above 18 give 0.
pub fn fn_004dcbe0(_e: &mut Engine, index: u32) -> u32 {
    match index {
        0 => 0,
        1..=11 => 0x13 + index,
        12 => 0x6f,
        13 => 0x70,
        14 => 0x24,
        15..=18 => 0x62 + index,
        _ => 0,
    }
}

// Translated from 004dcd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of display mode `index` of an adapter description (its
/// array at `+0x450`, `00877a30`, read through the element).
pub fn fn_004dcd00(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let element = e
        .call(
            ARRAY_ELEMENT,
            &args![this.addr().wrapping_add(0x450), index],
        )
        .u32();
    e.mem.u32(element)
}

// Translated from 004dcd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the device information `selector` of an adapter description
/// (`004dc160`) exists and allows windowed rendering (`004dcd60`).
pub fn fn_004dcd30(e: &mut Engine, this: Ptr, selector: u32) -> u8 {
    let info = fn_004dc160(e, this, selector);
    if info.is_null() {
        return 0;
    }
    fn_004dcd60(e, info)
}

// Translated from 004dcd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x140` of a device information object (read by
/// `004dcd30` as "windowed rendering possible").
pub fn fn_004dcd60(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x140))
}

// Translated from 004dcd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `01189468` (the pixel shader version the shader manager
/// needs / was given).
pub fn fn_004dcd80(e: &mut Engine) -> u32 {
    e.global(SHADER_MANAGER_WORD_A)
}

// Translated from 004dcd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0118946c` (the vertex shader version the shader manager
/// needs / was given).
pub fn fn_004dcd90(e: &mut Engine) -> u32 {
    e.global(SHADER_MANAGER_WORD_B)
}

// Translated from 004dcda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bUse Shaders:Display` setting (`00454af0`; its result register
/// passed on).
pub fn fn_004dcda0(e: &mut Engine) -> u32 {
    e.call(SETTING_BOOL_VALUE, &args![B_USE_SHADERS]).u32()
}

// Translated from 004dcdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The adapter description with `index` (cdecl), or null when there is no
/// adapter list or `index` is not below its count (the list's first word).
pub fn fn_004dcdb0(e: &mut Engine, index: u32) -> u32 {
    if fn_004dce30(e) == 0 {
        return 0;
    }
    let list: u32 = e.global(ADAPTER_LIST_POINTER);
    let count = e.call(SMART_POINTER_GET, &args![list]).u32();
    if index >= count {
        return 0;
    }
    let list: u32 = e.global(ADAPTER_LIST_POINTER);
    fn_004dcdf0(e, Ptr::new(list), index)
}

// Translated from 004dcdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of the adapter list's array at `+4` (`00658930` count,
/// `00877a30` element), or null past the end.
pub fn fn_004dcdf0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let array = this.addr().wrapping_add(4);
    if index >= e.call(ARRAY_COUNT, &args![array]).u32() {
        return 0;
    }
    let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
    e.mem.u32(element)
}

// Translated from 004dce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The adapter list, created on first use (`00e69a60`) and kept at
/// `011c70dc`.
pub fn fn_004dce30(e: &mut Engine) -> u32 {
    if e.global::<u32>(ADAPTER_LIST_POINTER) == 0 {
        let list = e.call(ADAPTER_LIST_CREATE, &[]).u32();
        e.set_global(ADAPTER_LIST_POINTER, list);
    }
    e.global(ADAPTER_LIST_POINTER)
}

// Translated from 004dce50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a light colour (three words from `color`, cdecl) at `011c74b8`
/// and, when there is a renderer object (`004bc3f0`), passes it with 0 as
/// the fourth component to the object's vtable +0xAC (a four-`float`
/// local built by `00414430`); returns whether it did.
pub fn fn_004dce50(e: &mut Engine, color: Ptr) -> bool {
    for i in 0..3 {
        let word = e.mem.u32(color.addr().wrapping_add(4 * i));
        e.set_global(LIGHT_COLOR + 4 * i, word);
    }
    if e.call(RENDERER_GETTER, &[]).u32() == 0 {
        return false;
    }
    e.with_stack(0x10, |e, local| {
        e.call(
            FLOAT4_CONSTRUCT,
            &args![
                local,
                FLOAT_ZERO_BITS,
                FLOAT_ZERO_BITS,
                FLOAT_ZERO_BITS,
                FLOAT_ZERO_BITS
            ],
        );
        for i in 0..3 {
            let word = x87_quiet_bits(e.global(LIGHT_COLOR + 4 * i));
            e.mem.set_u32(local.addr() + 4 * i, word);
        }
        e.mem.set_u32(local.addr() + 0xc, FLOAT_ZERO_BITS);
        let renderer = e.call(RENDERER_GETTER, &[]).u32();
        e.vcall(renderer, 0xac, &args![local]);
    });
    true
}

/// `FLDZ` pushed as a `float`.
const FLOAT_ZERO_BITS: u32 = 0;

// Translated from 004dd150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `powf` (Ghidra's name; no map name; cdecl): both `float`s widened to
/// `double` for `_pow` (`00ec9ac0`), the result stored to a `float` and
/// returned in ST0.
pub fn fn_004dd150(e: &mut Engine, base: f32, exponent: f32) -> f32 {
    let base = x87_load32(base.to_bits());
    let exponent = x87_load32(exponent.to_bits());
    let result = e.call(POW, &args![base, exponent]).st0;
    f32::from_bits(x87_quiet_bits(x87_store32(result)))
}

// Translated from 004dd180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `multisample` is supported (cdecl `(multisample, format)`): false
/// without a renderer; otherwise the Direct3D object's
/// `CheckDeviceMultiSampleType` (vtable +0x2C, C style) for `iAdapter`, the
/// renderer's device type, `format` (or, when 0, the display format of the
/// index at `01189470`), windowed (`bFull Screen:Display` off) and no quality
/// pointer must succeed.
pub fn fn_004dd180(e: &mut Engine, multisample: u32, format: u32) -> bool {
    if e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32() == 0 {
        return false;
    }
    let format = if format != 0 {
        format
    } else {
        let index = fn_004dc270(e);
        fn_004dcbe0(e, index)
    };
    e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]);
    let direct3d = fn_004dc040(e);
    let windowed = u32::from(e.call(WINDOWED_FROM_SETTING, &args![B_FULL_SCREEN]).u8());
    let renderer = e.call(SMART_POINTER_GET, &args![RENDERER_POINTER]).u32();
    let device_type = fn_004dd220(e, Ptr::new(renderer));
    let adapter = fn_004dc210(e) as u32;
    let result = e
        .vcall(
            direct3d,
            0x2c,
            &args![adapter, device_type, format, windowed, multisample, 0u32],
        )
        .i32();
    result >= 0
}

// Translated from 004dd220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The renderer's Direct3D device-type word (`+0x5C8`).
pub fn fn_004dd220(e: &mut Engine, this: Ptr) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add(RENDERER_DEVICE_TYPE_OFFSET))
}

// Translated from 004de070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The Eyefinity flag byte at `011c70ed`.
pub fn fn_004de070(e: &mut Engine) -> u8 {
    e.global(EYEFINITY_FLAG)
}

// Translated from 004de080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether both Eyefinity flag bytes (`011c70ed`, `011c70ee`) are set.
pub fn fn_004de080(e: &mut Engine) -> u8 {
    u8::from(e.global::<u8>(EYEFINITY_FLAG) != 0 && e.global::<u8>(EYEFINITY_SECOND_FLAG) != 0)
}

// ---- The Eyefinity query (batch b000s) -------------------------------------

/// `004dd240` (this unit, called by address until it is translated): the
/// address the query gives `ADL_Main_Control_Create` as its callback, and
/// that it calls itself (`cdecl`, a size) for the blocks it fills.
const ADL_ALLOCATE: u32 = 0x004d_d240;
/// `004dd270` (this unit, called by address until it is translated;
/// `stdcall`, the address of a pointer word): when the word is not null it
/// passes the word's block to `00401030` and clears the word.
const ADL_FREE_SLOT: u32 = 0x004d_d270;
/// `00403d30` (`cdecl`: block, value, size), the fill the query uses on the
/// blocks it gets.
const FILL_BYTES: u32 = 0x0040_3d30;
/// `"atiadlxx.dll"`, and `"atiadlxy.dll"` when that one does not load.
const ADL_LIBRARY: u32 = 0x0102_18dc;
const ADL_LIBRARY_FALLBACK: u32 = 0x0102_18cc;
/// The names the query looks up in the library, in its order (read in
/// Ghidra).
const ADL_MAIN_CONTROL_CREATE: u32 = 0x0102_18b4;
const ADL_MAIN_CONTROL_DESTROY: u32 = 0x0102_1898;
const ADL_ADAPTER_NUMBER_OF_ADAPTERS_GET: u32 = 0x0102_1874;
const ADL_ADAPTER_ADAPTER_INFO_GET: u32 = 0x0102_1858;
const ADL_DISPLAY_SLS_MAP_INDEX_GET: u32 = 0x0102_183c;
const ADL_DISPLAY_SLS_MAP_CONFIG_GET: u32 = 0x0102_181c;
const ADL_DISPLAY_MODES_GET: u32 = 0x0102_1804;
const ADL_DISPLAY_DISPLAY_MAP_CONFIG_GET: u32 = 0x0102_17e0;
/// Bytes of one adapter record the adapter-info call fills: the adapter's
/// index word at +4, the display index compared at +0x620.
const ADAPTER_RECORD_SIZE: u32 = 0x624;
/// Bytes of one entry of the array the query fills for its caller.
const DISPLAY_ENTRY_SIZE: u32 = 0x2c;
/// The query's stack frame below its frame pointer (`SUB ESP,0x10c`); the
/// slots the code passes by address, named by their offset below the frame
/// pointer as the disassembly addresses them.
const QUERY_FRAME: u32 = 0x10c;
const SLOT_ADAPTER_COUNT: u32 = 0x1c;
const SLOT_ADAPTER_RECORDS: u32 = 0x74;
const SLOT_DISPLAY_MAP_COUNT: u32 = 0x34;
const SLOT_DISPLAY_MAPS: u32 = 0x18;
const SLOT_TARGET_COUNT: u32 = 0xf8;
const SLOT_TARGETS: u32 = 0x10;
const SLOT_SLS_MAP_INDEX: u32 = 0x40;
/// The 0x50-byte SLS map record (its first word cleared by the code, the
/// other 0x4c bytes by `memset`); its words +0x10 (the rows: they divide
/// the height) and +0x14 (the columns: they divide the width).
const SLOT_SLS_MAP: u32 = 0xf0;
const SLOT_SLS_MAP_ROWS: u32 = 0xe0;
const SLOT_SLS_MAP_COLUMNS: u32 = 0xdc;
const SLOT_SLS_TARGET_COUNT: u32 = 0x64;
const SLOT_SLS_TARGETS: u32 = 0x98;
const SLOT_NATIVE_MODE_COUNT: u32 = 0x94;
const SLOT_NATIVE_MODES: u32 = 0x54;
const SLOT_BEZEL_MODE_COUNT: u32 = 0x60;
const SLOT_BEZEL_MODES: u32 = 0x84;
const SLOT_TRANSIENT_MODE_COUNT: u32 = 0x0c;
const SLOT_TRANSIENT_MODES: u32 = 0x3c;
const SLOT_OFFSET_COUNT: u32 = 0x5c;
const SLOT_OFFSETS: u32 = 0x2c;
const SLOT_MODE_COUNT: u32 = 0x08;
const SLOT_MODES: u32 = 0x70;

// Translated from 004dd2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`renderer.cpp`): `cdecl`, the address of a
/// pointer word. Hands it to `004dd270` (which releases the word's block and
/// clears it) and answers 1.
pub fn fn_004dd2a0(e: &mut Engine, slot: Ptr) -> u32 {
    e.call(ADL_FREE_SLOT, &args![slot]);
    1
}

/// One bit per display target, as `SHL reg,CL` makes it (the count taken
/// modulo 32).
fn target_bit(target: i32) -> u32 {
    1u32.wrapping_shl(target as u32)
}

// Translated from 004dd2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`renderer.cpp`): the Eyefinity (ADL) query for
/// one display, `cdecl`: display index, a 6-word description, the target
/// count word and the word that receives the entry array. Answers 1 or 0.
///
/// A null description, count or array word answers 0. The ADL library
/// (`atiadlxx.dll`, else `atiadlxy.dll`) is loaded and its eight entry
/// points looked up; a missing library or entry point, a failing
/// `ADL_Main_Control_Create` (given `004dd240` and 1),
/// `ADL_Adapter_NumberOfAdapters_Get` or `ADL_Adapter_AdapterInfo_Get`, no
/// adapters or no adapter block answer 0 at once (nothing released). The
/// adapter whose record has the display index gets its display map
/// (`ADL_Display_DisplayMapConfig_Get`, failing: result 0); with at least two
/// targets its SLS map (`ADL_Display_SLSMapIndex_Get`; failing, or fewer
/// targets, ends the search keeping result 1) and the map's configuration
/// (`ADL_Display_SLSMapConfig_Get`, failing: 0). The target count must be the
/// map's rows times columns (else 0). Per target, its modes
/// (`ADL_Display_Modes_Get`; failing marks the target failed) are matched
/// against the native modes, then the bezel modes (which sets the bezel
/// flag); a matched target gets its entry (the array allocated and cleared on
/// first use): its SLS column and row, its size (the mode's size divided by
/// columns and rows, swapped when the mode's orientation is neither 0 nor
/// 180), its position, its visible rectangle (from the SLS offsets for bezel
/// modes, else the same), and +0x28 set for the target at the middle-left
/// column (1 for 3 or 4 columns, 2 for 5 or 6, else 0) of the last row. Then
/// the description (1, columns, rows, the width and height, 1 at +0x14 for
/// bezel modes) and the count are written. Failed targets get entries
/// derived from the first target that did not fail. Every ADL block is
/// released (`004dd270`), `ADL_Main_Control_Destroy` runs (looked up again)
/// and the library is freed; on failure the array is released, the count
/// cleared and the description's first word cleared.
///
/// The "found" word (`EBP-4`), the rotated and bezel flags are never reset
/// between targets, and the SLS-target and bezel-mode indexes keep their
/// last values, as in the exe.
pub fn fn_004dd2c0(
    e: &mut Engine,
    display_index: u32,
    info: Ptr,
    count_out: Ptr,
    entries_out: Ptr,
) -> i32 {
    e.with_stack(QUERY_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + QUERY_FRAME - offset;
        let word = |e: &Engine, offset: u32| e.mem.u32(at(offset));
        // The code clears every local first; the frame is zero already except
        // for the SLS map record, which it clears with memset.
        e.call(MEMSET, &args![at(SLOT_SLS_MAP - 4), 0u32, 0x4cu32]);
        if info.is_null() || count_out.is_null() || entries_out.is_null() {
            return 0;
        }
        let mut module = e.call(IMPORT_LOAD_LIBRARY, &args![ADL_LIBRARY]).u32();
        if module == 0 {
            module = e
                .call(IMPORT_LOAD_LIBRARY, &args![ADL_LIBRARY_FALLBACK])
                .u32();
        }
        if module == 0 {
            return 0;
        }
        let look_up =
            |e: &mut Engine, name: u32| e.call(IMPORT_GET_PROC_ADDRESS, &args![module, name]).u32();
        let create = look_up(e, ADL_MAIN_CONTROL_CREATE);
        let destroy = look_up(e, ADL_MAIN_CONTROL_DESTROY);
        let number_of_adapters = look_up(e, ADL_ADAPTER_NUMBER_OF_ADAPTERS_GET);
        let adapter_info = look_up(e, ADL_ADAPTER_ADAPTER_INFO_GET);
        let sls_map_index = look_up(e, ADL_DISPLAY_SLS_MAP_INDEX_GET);
        let sls_map_config = look_up(e, ADL_DISPLAY_SLS_MAP_CONFIG_GET);
        let display_modes = look_up(e, ADL_DISPLAY_MODES_GET);
        let display_map_config = look_up(e, ADL_DISPLAY_DISPLAY_MAP_CONFIG_GET);
        if [
            create,
            destroy,
            number_of_adapters,
            adapter_info,
            sls_map_index,
            sls_map_config,
            display_modes,
            display_map_config,
        ]
        .contains(&0)
        {
            return 0;
        }
        if e.call(create, &args![ADL_ALLOCATE, 1u32]).u32() != 0 {
            return 0;
        }
        if e.call(number_of_adapters, &args![at(SLOT_ADAPTER_COUNT)])
            .u32()
            != 0
        {
            return 0;
        }
        if word(e, SLOT_ADAPTER_COUNT) == 0 {
            return 0;
        }
        let records_size = word(e, SLOT_ADAPTER_COUNT).wrapping_mul(ADAPTER_RECORD_SIZE);
        let records = e.call(ADL_ALLOCATE, &args![records_size]).u32();
        e.mem.set_u32(at(SLOT_ADAPTER_RECORDS), records);
        if records == 0 {
            return 0;
        }
        let records_size = word(e, SLOT_ADAPTER_COUNT).wrapping_mul(ADAPTER_RECORD_SIZE);
        e.call(FILL_BYTES, &args![records, 0u32, records_size]);
        let records_size = word(e, SLOT_ADAPTER_COUNT).wrapping_mul(ADAPTER_RECORD_SIZE);
        if e.call(adapter_info, &args![records, records_size]).u32() != 0 {
            return 0;
        }

        let mut result: i32 = 1;
        let mut found = 0u32;
        let mut bezel = 0u32;
        let mut rotated = 0u32;
        let mut matched = 0u32;
        let mut failed = 0u32;
        let mut width = 0u32;
        let mut height = 0u32;
        let mut middle_column = 0u32;
        let mut last_row = 0u32;
        let mut entry = 0u32;
        let mut sls_target: i32;
        let mut bezel_mode: i32 = 0;
        let target_display = |e: &Engine, target: i32| {
            e.mem
                .u32(word(e, SLOT_TARGETS).wrapping_add((target as u32).wrapping_mul(0x1c)))
        };
        let sls_record = |e: &Engine, index: i32| {
            word(e, SLOT_SLS_TARGETS).wrapping_add((index as u32).wrapping_mul(0x70))
        };
        let columns = |e: &Engine| word(e, SLOT_SLS_MAP_COLUMNS);
        let rows = |e: &Engine| word(e, SLOT_SLS_MAP_ROWS);

        'search: {
            let mut adapter_index: i32 = 0;
            while adapter_index < word(e, SLOT_ADAPTER_COUNT) as i32 {
                let record = word(e, SLOT_ADAPTER_RECORDS)
                    .wrapping_add((adapter_index as u32).wrapping_mul(ADAPTER_RECORD_SIZE));
                if e.mem.u32(record.wrapping_add(0x620)) != display_index {
                    adapter_index += 1;
                    continue;
                }
                let adapter = e.mem.u32(record.wrapping_add(4));
                let status = e
                    .call(
                        display_map_config,
                        &args![
                            adapter,
                            at(SLOT_DISPLAY_MAP_COUNT),
                            at(SLOT_DISPLAY_MAPS),
                            at(SLOT_TARGET_COUNT),
                            at(SLOT_TARGETS),
                            1u32
                        ],
                    )
                    .u32();
                if status != 0 {
                    result = 0;
                    break 'search;
                }
                if (word(e, SLOT_TARGET_COUNT) as i32) < 2 {
                    break 'search;
                }
                let status = e
                    .call(
                        sls_map_index,
                        &args![
                            adapter,
                            word(e, SLOT_TARGET_COUNT),
                            word(e, SLOT_TARGETS),
                            at(SLOT_SLS_MAP_INDEX)
                        ],
                    )
                    .u32();
                if status != 0 {
                    break 'search;
                }
                let status = e
                    .call(
                        sls_map_config,
                        &args![
                            adapter,
                            word(e, SLOT_SLS_MAP_INDEX),
                            at(SLOT_SLS_MAP),
                            at(SLOT_SLS_TARGET_COUNT),
                            at(SLOT_SLS_TARGETS),
                            at(SLOT_NATIVE_MODE_COUNT),
                            at(SLOT_NATIVE_MODES),
                            at(SLOT_BEZEL_MODE_COUNT),
                            at(SLOT_BEZEL_MODES),
                            at(SLOT_TRANSIENT_MODE_COUNT),
                            at(SLOT_TRANSIENT_MODES),
                            at(SLOT_OFFSET_COUNT),
                            at(SLOT_OFFSETS),
                            2u32
                        ],
                    )
                    .u32();
                if status != 0 {
                    result = 0;
                    break 'search;
                }
                if word(e, SLOT_TARGET_COUNT) != columns(e).wrapping_mul(rows(e)) {
                    result = 0;
                    break 'search;
                }
                // The switch on columns - 3 (jump table at 004de060, read in
                // Ghidra): 0 and 1 give 1, 2 and 3 give 2, anything else 0.
                middle_column = match columns(e).wrapping_sub(3) {
                    0 | 1 => 1,
                    2 | 3 => 2,
                    _ => 0,
                };
                last_row = rows(e).wrapping_sub(1);
                'targets: {
                    let mut target: i32 = 0;
                    while target < word(e, SLOT_TARGET_COUNT) as i32 {
                        let bit = target_bit(target);
                        e.call(ADL_FREE_SLOT, &args![at(SLOT_MODES)]);
                        let display = target_display(e, target);
                        let status = e
                            .call(
                                display_modes,
                                &args![adapter, display, at(SLOT_MODE_COUNT), at(SLOT_MODES)],
                            )
                            .u32();
                        if status != 0 {
                            failed |= bit;
                            target += 1;
                            continue;
                        }
                        let modes = word(e, SLOT_MODES);
                        let mode_width = |e: &Engine| e.mem.u32(modes.wrapping_add(0x1c));
                        let mode_height = |e: &Engine| e.mem.u32(modes.wrapping_add(0x20));
                        let mut native: i32 = 0;
                        while native < word(e, SLOT_NATIVE_MODE_COUNT) as i32 {
                            let record = word(e, SLOT_NATIVE_MODES)
                                .wrapping_add((native as u32).wrapping_mul(0x50));
                            if mode_width(e) == e.mem.u32(record.wrapping_add(0x28))
                                && mode_height(e) == e.mem.u32(record.wrapping_add(0x2c))
                            {
                                matched |= bit;
                                break;
                            }
                            native += 1;
                        }
                        if matched & bit == 0 {
                            bezel_mode = 0;
                            while bezel_mode < word(e, SLOT_BEZEL_MODE_COUNT) as i32 {
                                let record = word(e, SLOT_BEZEL_MODES)
                                    .wrapping_add((bezel_mode as u32).wrapping_mul(0x58));
                                if mode_width(e) == e.mem.u32(record.wrapping_add(0x28))
                                    && mode_height(e) == e.mem.u32(record.wrapping_add(0x2c))
                                {
                                    matched |= bit;
                                    bezel = 1;
                                    break;
                                }
                                bezel_mode += 1;
                            }
                        }
                        if matched & bit != 0 {
                            if e.mem.u32(entries_out.addr()) == 0 {
                                let size =
                                    word(e, SLOT_TARGET_COUNT).wrapping_mul(DISPLAY_ENTRY_SIZE);
                                let block = e.call(ADL_ALLOCATE, &args![size]).u32();
                                e.mem.set_u32(entries_out.addr(), block);
                                if e.mem.u32(entries_out.addr()) == 0 {
                                    result = 0;
                                    break 'targets;
                                }
                                let size =
                                    word(e, SLOT_TARGET_COUNT).wrapping_mul(DISPLAY_ENTRY_SIZE);
                                let block = e.mem.u32(entries_out.addr());
                                e.call(FILL_BYTES, &args![block, 0u32, size]);
                            }
                            sls_target = 0;
                            while sls_target < word(e, SLOT_SLS_TARGET_COUNT) as i32 {
                                let record = sls_record(e, sls_target);
                                if target_display(e, target) == e.mem.u32(record.wrapping_add(8)) {
                                    found = 1;
                                    break;
                                }
                                sls_target += 1;
                            }
                            if found == 0 {
                                result = 0;
                                break 'targets;
                            }
                            width = mode_width(e);
                            height = mode_height(e);
                            let orientation = e.mem.u32(modes.wrapping_add(0x2c));
                            if orientation != 0 && orientation != 0xb4 {
                                rotated = 1;
                            }
                            entry = e
                                .mem
                                .u32(entries_out.addr())
                                .wrapping_add((target as u32).wrapping_mul(DISPLAY_ENTRY_SIZE));
                            let record = sls_record(e, sls_target);
                            let column = e.mem.u32(record.wrapping_add(0x24));
                            e.mem.set_u32(entry, column);
                            let row = e.mem.u32(record.wrapping_add(0x28));
                            e.mem.set_u32(entry.wrapping_add(4), row);
                            let (across, down) = if rotated == 0 {
                                (mode_width(e), mode_height(e))
                            } else {
                                (mode_height(e), mode_width(e))
                            };
                            let tile_width = (across as i32) / (columns(e) as i32);
                            e.mem.set_u32(entry.wrapping_add(0x10), tile_width as u32);
                            let tile_height = (down as i32) / (rows(e) as i32);
                            e.mem.set_u32(entry.wrapping_add(0x14), tile_height as u32);
                            let x = e
                                .mem
                                .u32(entry.wrapping_add(0x10))
                                .wrapping_mul(e.mem.u32(record.wrapping_add(0x24)));
                            e.mem.set_u32(entry.wrapping_add(8), x);
                            let y = e
                                .mem
                                .u32(entry.wrapping_add(0x14))
                                .wrapping_mul(e.mem.u32(record.wrapping_add(0x28)));
                            e.mem.set_u32(entry.wrapping_add(0xc), y);
                            if bezel == 1 {
                                let bezel_record = word(e, SLOT_BEZEL_MODES)
                                    .wrapping_add((bezel_mode as u32).wrapping_mul(0x58));
                                let mut offset: i32 = 0;
                                while offset < word(e, SLOT_OFFSET_COUNT) as i32 {
                                    let record = word(e, SLOT_OFFSETS)
                                        .wrapping_add((offset as u32).wrapping_mul(0x34));
                                    if e.mem.u32(bezel_record.wrapping_add(8))
                                        == e.mem.u32(record.wrapping_add(0x18))
                                        && target_display(e, target)
                                            == e.mem.u32(record.wrapping_add(8))
                                    {
                                        found = 1;
                                        break;
                                    }
                                    offset += 1;
                                }
                                if found == 0 {
                                    result = 0;
                                    break 'targets;
                                }
                                let record = word(e, SLOT_OFFSETS)
                                    .wrapping_add((offset as u32).wrapping_mul(0x34));
                                for (to, from) in
                                    [(0x20, 0x24), (0x24, 0x28), (0x18, 0x1c), (0x1c, 0x20)]
                                {
                                    let value = e.mem.u32(record.wrapping_add(from));
                                    e.mem.set_u32(entry.wrapping_add(to), value);
                                }
                            } else {
                                for i in 0..4 {
                                    let value = e.mem.u32(entry.wrapping_add(8 + 4 * i));
                                    e.mem.set_u32(entry.wrapping_add(0x18 + 4 * i), value);
                                }
                            }
                            let record = sls_record(e, sls_target);
                            if middle_column == e.mem.u32(record.wrapping_add(0x24))
                                && last_row == e.mem.u32(record.wrapping_add(0x28))
                            {
                                e.mem.set_u32(entry.wrapping_add(0x28), 1);
                            }
                        }
                        target += 1;
                    }
                }
                if matched != 0 && result == 1 {
                    let info = info.addr();
                    e.mem.set_u32(info, 1);
                    e.mem.set_u32(info.wrapping_add(4), columns(e));
                    e.mem.set_u32(info.wrapping_add(8), rows(e));
                    if rotated == 0 {
                        e.mem.set_u32(info.wrapping_add(0xc), width);
                        e.mem.set_u32(info.wrapping_add(0x10), height);
                    } else {
                        e.mem.set_u32(info.wrapping_add(0xc), height);
                        e.mem.set_u32(info.wrapping_add(0x10), width);
                    }
                    if bezel == 1 {
                        e.mem.set_u32(info.wrapping_add(0x14), 1);
                    }
                    e.mem.set_u32(count_out.addr(), word(e, SLOT_TARGET_COUNT));
                }
                break 'search;
            }
        }

        if matched != 0 && failed != 0 {
            let mut reference = 0u32;
            let mut target: i32 = 0;
            while target < word(e, SLOT_TARGET_COUNT) as i32 {
                if target_bit(target) & failed == 0 {
                    reference = e
                        .mem
                        .u32(entries_out.addr())
                        .wrapping_add((target as u32).wrapping_mul(DISPLAY_ENTRY_SIZE));
                    break;
                }
                target += 1;
            }
            if entry == 0 {
                result = 0;
            } else {
                let reference_word =
                    |e: &Engine, offset: u32| e.mem.u32(reference.wrapping_add(offset));
                let mut target: i32 = 0;
                while target < word(e, SLOT_TARGET_COUNT) as i32 {
                    if target_bit(target) & failed == 0 {
                        target += 1;
                        continue;
                    }
                    sls_target = 0;
                    while sls_target < word(e, SLOT_SLS_TARGET_COUNT) as i32 {
                        let record = sls_record(e, sls_target);
                        if target_display(e, target) == e.mem.u32(record.wrapping_add(8)) {
                            found = 1;
                            break;
                        }
                        sls_target += 1;
                    }
                    if found == 0 {
                        result = 0;
                        break;
                    }
                    entry = e
                        .mem
                        .u32(entries_out.addr())
                        .wrapping_add((target as u32).wrapping_mul(DISPLAY_ENTRY_SIZE));
                    let record = sls_record(e, sls_target);
                    let column = e.mem.u32(record.wrapping_add(0x24));
                    e.mem.set_u32(entry, column);
                    let row = e.mem.u32(record.wrapping_add(0x28));
                    e.mem.set_u32(entry.wrapping_add(4), row);
                    let value = reference_word(e, 0x10);
                    e.mem.set_u32(entry.wrapping_add(0x10), value);
                    let value = reference_word(e, 0x14);
                    e.mem.set_u32(entry.wrapping_add(0x14), value);
                    let x =
                        reference_word(e, 0x10).wrapping_mul(e.mem.u32(record.wrapping_add(0x24)));
                    e.mem.set_u32(entry.wrapping_add(8), x);
                    let y =
                        reference_word(e, 0x14).wrapping_mul(e.mem.u32(record.wrapping_add(0x28)));
                    e.mem.set_u32(entry.wrapping_add(0xc), y);
                    if bezel == 1 {
                        let value = reference_word(e, 0x20);
                        e.mem.set_u32(entry.wrapping_add(0x20), value);
                        let value = reference_word(e, 0x24);
                        e.mem.set_u32(entry.wrapping_add(0x24), value);
                        let column = e.mem.u32(record.wrapping_add(0x24));
                        let visible_x = if column == 0 {
                            0
                        } else if column == columns(e).wrapping_sub(1) {
                            width.wrapping_sub(reference_word(e, 0x20))
                        } else {
                            let spare =
                                reference_word(e, 0x10).wrapping_sub(reference_word(e, 0x20));
                            (((spare as i32) / 2) as u32)
                                .wrapping_add(e.mem.u32(entry.wrapping_add(8)))
                        };
                        e.mem.set_u32(entry.wrapping_add(0x18), visible_x);
                        let row = e.mem.u32(record.wrapping_add(0x28));
                        let visible_y = if row == 0 {
                            0
                        } else if row == rows(e).wrapping_sub(1) {
                            height.wrapping_sub(reference_word(e, 0x24))
                        } else {
                            let spare =
                                reference_word(e, 0x14).wrapping_sub(reference_word(e, 0x24));
                            (((spare as i32) / 2) as u32)
                                .wrapping_add(e.mem.u32(entry.wrapping_add(0xc)))
                        };
                        e.mem.set_u32(entry.wrapping_add(0x1c), visible_y);
                    } else {
                        for i in 0..4 {
                            let value = e.mem.u32(entry.wrapping_add(8 + 4 * i));
                            e.mem.set_u32(entry.wrapping_add(0x18 + 4 * i), value);
                        }
                    }
                    if middle_column == e.mem.u32(record.wrapping_add(0x24))
                        && last_row == e.mem.u32(record.wrapping_add(0x28))
                    {
                        e.mem.set_u32(entry.wrapping_add(0x28), 1);
                    }
                    target += 1;
                }
            }
        }

        for slot in [
            SLOT_MODES,
            SLOT_SLS_TARGETS,
            SLOT_NATIVE_MODES,
            SLOT_BEZEL_MODES,
            SLOT_TRANSIENT_MODES,
            SLOT_OFFSETS,
            SLOT_DISPLAY_MAPS,
            SLOT_TARGETS,
            SLOT_ADAPTER_RECORDS,
        ] {
            e.call(ADL_FREE_SLOT, &args![at(slot)]);
        }
        let destroy = e
            .call(
                IMPORT_GET_PROC_ADDRESS,
                &args![module, ADL_MAIN_CONTROL_DESTROY],
            )
            .u32();
        if destroy != 0 {
            e.call(destroy, &args![]);
        }
        e.call(IMPORT_FREE_LIBRARY, &args![module]);
        if result == 0 {
            fn_004dd2a0(e, entries_out);
            e.mem.set_u32(count_out.addr(), 0);
            e.call(FILL_BYTES, &args![info, 0u32, 4u32]);
        }
        result
    })
}

/// `EnumDisplayDevicesA` (import slot) and `printf` (`00ec9d4a`, `cdecl`).
const IMPORT_ENUM_DISPLAY_DEVICES: u32 = 0x00fd_f2f0;
const PRINTF: u32 = 0x00ec_9d4a;
/// `"Eyefinity configuration query failed for display index %i.\n"`.
const EYEFINITY_QUERY_FAILED: u32 = 0x0102_18ec;
/// The byte `004de100` requires besides an Eyefinity description.
const EYEFINITY_GATE_01189484: u32 = 0x0118_9484;
/// The `float`s `004de100` sets: the description's width and height, their
/// ratio, the copies of width and height (the width divided by 3 for three
/// columns of one row when `011c70ee` is set), and the word that also gets
/// that third.
const EYEFINITY_TOTAL_WIDTH_0118948C: u32 = 0x0118_948c;
const EYEFINITY_TOTAL_HEIGHT_01189490: u32 = 0x0118_9490;
const EYEFINITY_RATIO_01189488: u32 = 0x0118_9488;
const EYEFINITY_VIEW_WIDTH_01189494: u32 = 0x0118_9494;
const EYEFINITY_VIEW_HEIGHT_01189498: u32 = 0x0118_9498;
const EYEFINITY_THIRD_WIDTH_011C70F0: u32 = 0x011c_70f0;
/// The flag set instead of `011c70ee` when the first entry's height
/// (+0x14) is above its width (+0x10).
const EYEFINITY_TALL_FLAG_011C70EF: u32 = 0x011c_70ef;
/// The `double` 3.0 the width is divided by (`01021928`).
const THREE_01021928: u32 = 0x0102_1928;
/// `004de100`'s frame below its frame pointer (`SUB ESP,0x1d8`), the
/// `DISPLAY_DEVICEA` at its bottom (`cb` 0x1a8, `StateFlags` at +0xa4), the
/// description (6 words at `EBP-0x24`), the count (`EBP-8`) and the entry
/// array word (`EBP-0xc`).
const SETUP_FRAME: u32 = 0x1d8;
const SETUP_DISPLAY_DEVICE: u32 = 0x1d8;
const SETUP_INFO: u32 = 0x24;
const SETUP_COUNT: u32 = 0x08;
const SETUP_ENTRIES: u32 = 0x0c;

// Translated from 004de100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`renderer.cpp`): the Eyefinity set-up
/// `Renderer::Init` runs when `bEnableEyefinity` is set. The display index
/// starts at -1 and, being -1, is replaced by the first display device
/// (`EnumDisplayDevicesA(NULL, index, &device, 0)`, `cb` 0x1a8) whose state
/// flags have bit 2, or stays -1 when the enumeration ends first. Then the
/// query [`fn_004dd2c0`] runs for it. On success, when the description's
/// first word is 1 and the byte at `01189484` is set: width and height as
/// `float`s (`FILD`/`FSTP`), their copies, the ratio width / height, the flag
/// `011c70ed`, `011c70ef` when the first entry is taller than wide else
/// `011c70ee`, and with `011c70ee` set and a grid of 3 columns and 1 row the
/// view width becomes width / 3.0 (also stored at `011c70f0`) and the view
/// height is copied again. The entry array is released ([`fn_004dd2a0`]).
/// On failure `printf` reports the display index.
// Translation incomplete: 004de100 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_004de100(e: &mut Engine) {
    e.with_stack(SETUP_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + SETUP_FRAME - offset;
        let mut display = u32::MAX;
        // The exe tests the index it has just set to -1.
        if display == u32::MAX {
            let mut index = 0u32;
            let device = at(SETUP_DISPLAY_DEVICE);
            e.mem.set_u32(device, 0x1a8);
            loop {
                let more = e
                    .call(
                        IMPORT_ENUM_DISPLAY_DEVICES,
                        &args![0u32, index, device, 0u32],
                    )
                    .u32();
                if more == 0 {
                    break;
                }
                if e.mem.u32(device.wrapping_add(0xa4)) & 4 != 0 {
                    display = index;
                    break;
                }
                index = index.wrapping_add(1);
            }
        }
        let info = at(SETUP_INFO);
        let result = fn_004dd2c0(
            e,
            display,
            Ptr::new(info),
            Ptr::new(at(SETUP_COUNT)),
            Ptr::new(at(SETUP_ENTRIES)),
        );
        if result != 1 {
            e.call(PRINTF, &args![EYEFINITY_QUERY_FAILED, display]);
            return;
        }
        if e.mem.u32(info) == 1 && e.global::<u8>(EYEFINITY_GATE_01189484) != 0 {
            let total_width = x87_store32(f64::from(e.mem.u32(info.wrapping_add(0xc)) as i32));
            e.set_global(EYEFINITY_TOTAL_WIDTH_0118948C, total_width);
            let total_height = x87_store32(f64::from(e.mem.u32(info.wrapping_add(0x10)) as i32));
            e.set_global(EYEFINITY_TOTAL_HEIGHT_01189490, total_height);
            let copy = x87_quiet_bits(e.global::<u32>(EYEFINITY_TOTAL_WIDTH_0118948C));
            e.set_global(EYEFINITY_VIEW_WIDTH_01189494, copy);
            let copy = x87_quiet_bits(e.global::<u32>(EYEFINITY_TOTAL_HEIGHT_01189490));
            e.set_global(EYEFINITY_VIEW_HEIGHT_01189498, copy);
            let ratio = x87_div(
                x87_load32(e.global::<u32>(EYEFINITY_TOTAL_WIDTH_0118948C)),
                x87_m32(e.global::<u32>(EYEFINITY_TOTAL_HEIGHT_01189490)),
            );
            e.set_global(EYEFINITY_RATIO_01189488, x87_store32(ratio));
            e.set_global(EYEFINITY_FLAG, 1u8);
            let entries = e.mem.u32(at(SETUP_ENTRIES));
            let entry_height = e.mem.u32(entries.wrapping_add(0x14)) as i32;
            let entry_width = e.mem.u32(entries.wrapping_add(0x10)) as i32;
            if entry_height > entry_width {
                e.set_global(EYEFINITY_TALL_FLAG_011C70EF, 1u8);
            } else {
                e.set_global(EYEFINITY_SECOND_FLAG, 1u8);
            }
            if e.global::<u8>(EYEFINITY_SECOND_FLAG) != 0
                && e.mem.u32(info.wrapping_add(4)) == 3
                && e.mem.u32(info.wrapping_add(8)) == 1
            {
                let third = x87_div(
                    x87_load32(e.global::<u32>(EYEFINITY_TOTAL_WIDTH_0118948C)),
                    e.global::<f64>(THREE_01021928),
                );
                e.set_global(EYEFINITY_VIEW_WIDTH_01189494, x87_store32(third));
                let copy = x87_quiet_bits(e.global::<u32>(EYEFINITY_TOTAL_HEIGHT_01189490));
                e.set_global(EYEFINITY_VIEW_HEIGHT_01189498, copy);
                let copy = x87_quiet_bits(e.global::<u32>(EYEFINITY_VIEW_WIDTH_01189494));
                e.set_global(EYEFINITY_THIRD_WIDTH_011C70F0, copy);
            }
        }
        fn_004dd2a0(e, Ptr::new(at(SETUP_ENTRIES)));
    })
}

// Translated from 004de0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the `float` at `01189494` (the view width `004de100` sets), loaded with
/// `FLD dword` and returned in ST0 (a signalling NaN comes out quiet).
pub fn fn_004de0e0(e: &mut Engine) -> f32 {
    x87_quiet(e.global::<f32>(EYEFINITY_VIEW_WIDTH_01189494))
}

/// `004ed780` (no map name): thiscall on the `fGamma:Display` setting (`011c71ac`), one
/// `float` word.
const GAMMA_SETTING_SET_004ED780: u32 = 0x004e_d780;
/// `255.0` (`double`): the gamma ramp's step divisor.
const RAMP_STEPS_0101E568: u32 = 0x0101_e568;
/// `65535.0` (`double`): the ramp's full scale.
const RAMP_SCALE_010217D0: u32 = 0x0102_17d0;
/// `0.5` (`double`): added before the truncating store.
const HALF_01011588: u32 = 0x0101_1588;
/// The gamma ramp the code builds on its stack: 256 `u16` each for red, green and blue,
/// one after the other (0x600 bytes).
const GAMMA_RAMP_SIZE: u32 = 0x600;

/// One entry of the gamma ramp: `004dd130(i / 255.0, gamma) * 65535.0 + 0.5`, truncated
/// (`FISTP` under round toward zero) and kept as its low 16 bits.
fn gamma_ramp_entry(e: &mut Engine, index: u16) -> u16 {
    let gamma = f32::from_bits(x87_quiet_bits(e.global::<u32>(GAMMA)));
    let steps: f64 = e.global(RAMP_STEPS_0101E568);
    let level = x87_store32(x87_div(f64::from(index), steps));
    let curve = fn_004dd130(e, f32::from_bits(x87_quiet_bits(level)), gamma);
    let scale: f64 = e.global(RAMP_SCALE_010217D0);
    let half: f64 = e.global(HALF_01011588);
    let value = x87_add(x87_mul(f64::from(curve), scale), half);
    crate::x87::fistp_chop(value) as u16
}

// Translated from 004dcef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004dcef0 - the /GS stack cookie (011c16bc, checked by 00ec408c) is not translated
/// No map name: applies the gamma. Clears the gamma-changed byte (`011c6fb8`), hands the
/// gamma (`0118945c`) to the `fGamma:Display` setting (`004ed780`), builds the 256-entry
/// red, green and blue ramps on its stack (each entry computed three times, once per
/// colour, by the same expression), then gives the ramp to the renderer's Direct3D device
/// (`004dc020` of the renderer in `011c73b4`) through its vtable +0x54 with (0, 1, ramp),
/// in the COM form (the device is also the first stack word).
pub fn fn_004dcef0(e: &mut Engine) {
    e.set_global(GAMMA_CHANGED, 0u8);
    let gamma = x87_quiet_bits(e.global::<u32>(GAMMA));
    e.call(GAMMA_SETTING_SET_004ED780, &args![F_GAMMA, gamma]);
    e.with_stack(GAMMA_RAMP_SIZE, |e, ramp| {
        let base = ramp.addr();
        let mut index = 0u16;
        while u32::from(index) < 0x100 {
            let slot = 2 * u32::from(index);
            let red = gamma_ramp_entry(e, index);
            e.mem.set_u16(base + slot, red);
            let green = gamma_ramp_entry(e, index);
            e.mem.set_u16(base + 0x200 + slot, green);
            let blue = gamma_ramp_entry(e, index);
            e.mem.set_u16(base + 0x400 + slot, blue);
            index = index.wrapping_add(1);
        }
        let renderer = e
            .call(SMART_POINTER_GET, &args![RENDERER_POINTER])
            .ptr::<()>();
        let device = fn_004dc020(e, renderer);
        e.vcall(device.addr(), 0x54, &args![0u32, 1u32, ramp]);
    });
}

/// The allocator (`00401000`, cdecl, one word) and the free (`00401030`).
const ALLOCATE_00401000: u32 = 0x0040_1000;
const FREE_00401030: u32 = 0x0040_1030;
/// The destructor body of `SettingT<INIPrefSettingCollection>` (engine map
/// name of its scalar deleting destructor, `004de3f0`).
const PREF_SETTING_DESTRUCTOR_004DE420: u32 = 0x004d_e420;

// Translated from 004dc6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The window position settings (cdecl, two words; the engine map has no
/// name): unless the game runs full screen (`00446e10`), `iLocation X` and
/// `iLocation Y` (`011c75d4`, `011c7654`) take the two words (`0045ce80`).
pub fn fn_004dc6e0(e: &mut Engine, x: u32, y: u32) {
    if e.call(FULL_SCREEN_MODE, &args![]).bool() {
        return;
    }
    e.call(SETTING_INT_SET, &args![I_LOCATION_X, x]);
    e.call(SETTING_INT_SET, &args![I_LOCATION_Y, y]);
}

// Translated from 004dd240 (decompiled, FalloutNV.exe 1.4.0.525)
/// An allocation callback (stdcall, one word: the size; the engine map has
/// no name): the allocator's block (`00401000`).
pub fn fn_004dd240(e: &mut Engine, size: u32) -> u32 {
    e.call(ALLOCATE_00401000, &args![size]).u32()
}

// Translated from 004dd270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The matching free callback (stdcall, one word: the address of a block
/// pointer; the engine map has no name): frees the block (`00401030`) and
/// clears the pointer when it is set.
pub fn fn_004dd270(e: &mut Engine, block: Ptr) {
    let address = e.mem.u32(block.addr());
    if address != 0 {
        e.call(FREE_00401030, &args![address]);
        e.mem.set_u32(block.addr(), 0);
    }
}

// Translated from 004de0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The Eyefinity total width (`float` at `0118948c`) in `ST0` (no map
/// name; a signalling NaN comes out quiet).
pub fn fn_004de0c0(e: &mut Engine) -> f32 {
    let bits = e.global::<u32>(EYEFINITY_TOTAL_WIDTH_0118948C);
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 004de0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The Eyefinity total height (`float` at `01189490`) in `ST0` (no map
/// name).
pub fn fn_004de0d0(e: &mut Engine) -> f32 {
    let bits = e.global::<u32>(EYEFINITY_TOTAL_HEIGHT_01189490);
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 004de0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The Eyefinity view height (`float` at `01189498`) in `ST0` (no map
/// name).
pub fn fn_004de0f0(e: &mut Engine) -> f32 {
    let bits = e.global::<u32>(EYEFINITY_VIEW_HEIGHT_01189498);
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 004de2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A bool setting's store (`ECX` the setting, `RET 4`: the byte; the
/// engine map has no name): the byte at +4 of a non-null setting. Returns
/// the setting.
pub fn fn_004de2b0(e: &mut Engine, this: Ptr, value: u8) -> Ptr {
    if !this.is_null() {
        e.mem.set_u8(this.addr() + 4, value);
    }
    this
}

// Translated from 004de2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The bool setting set (`ECX` the setting, `RET 4`: the byte; the engine
/// map has no name): [`fn_004de2b0`] with the byte. Returns the setting.
pub fn fn_004de2d0(e: &mut Engine, this: Ptr, value: u8) -> Ptr {
    fn_004de2b0(e, this, value);
    this
}

// Translated from 004de3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingT<INIPrefSettingCollection>` scalar deleting destructor (engine
/// map name; `RET 4`: the flags): the destructor body (`004de420`), then
/// the free (`00401030`) when bit 0 is set. Returns the setting.
pub fn setting_t_ini_pref_setting_collection_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(PREF_SETTING_DESTRUCTOR_004DE420, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_00401030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004d_c6e0, fn_004dc6e0(u32, u32)),
        entry!(0x004d_d240, fn_004dd240(u32) -> u32),
        entry!(0x004d_d270, fn_004dd270(Ptr)),
        entry!(0x004d_e0c0, fn_004de0c0() -> f32),
        entry!(0x004d_e0d0, fn_004de0d0() -> f32),
        entry!(0x004d_e0f0, fn_004de0f0() -> f32),
        entry!(0x004d_e2b0, fn_004de2b0(Ptr, u8) -> Ptr),
        entry!(0x004d_e2d0, fn_004de2d0(Ptr, u8) -> Ptr),
        entry!(
            0x004d_e3f0,
            setting_t_ini_pref_setting_collection_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004d_e0e0, fn_004de0e0() -> f32),
        entry!(0x004d_cef0, fn_004dcef0()),
        entry!(0x004d_c5c0, fn_004dc5c0()),
        entry!(0x004d_c650, fn_004dc650()),
        entry!(0x004d_e290, fn_004de290(Ptr, f32) -> Ptr),
        entry!(0x004de2f0, fn_004de2f0(Ptr, u32, u8) -> Ptr),
        entry!(0x004de370, fn_004de370(Ptr, u32, u32) -> Ptr),
        entry!(0x004da4d0, renderer_kill(u8)),
        entry!(0x004da570, fn_004da570() -> bool),
        entry!(0x004da650, fn_004da650(Ptr) -> Ptr),
        entry!(0x004da670, renderer_init(u32, u32) -> Ptr),
        entry!(0x004dc010, fn_004dc010(u8)),
        entry!(0x004dc020, fn_004dc020(Ptr) -> Ptr),
        entry!(0x004dc040, fn_004dc040() -> u32),
        entry!(0x004dc050, fn_004dc050(u8)),
        entry!(0x004dc060, fn_004dc060(u8) -> i32),
        entry!(0x004dc090, fn_004dc090(u8)),
        entry!(0x004dc0a0, fn_004dc0a0() -> u8),
        entry!(0x004dc0b0, fn_004dc0b0() -> i32),
        entry!(0x004dc0c0, fn_004dc0c0(i32)),
        entry!(0x004dc100, fn_004dc100(u32)),
        entry!(0x004dc110, fn_004dc110() -> u32),
        entry!(0x004dc120, fn_004dc120(Ptr, u32) -> Ptr),
        entry!(0x004dc160, fn_004dc160(Ptr, u32) -> Ptr),
        entry!(0x004dc1c0, fn_004dc1c0(Ptr) -> bool),
        entry!(0x004dc1e0, fn_004dc1e0() -> u32),
        entry!(0x004dc1f0, fn_004dc1f0() -> i32),
        entry!(0x004dc200, fn_004dc200() -> i32),
        entry!(0x004dc210, fn_004dc210() -> i32),
        entry!(0x004dc220, fn_004dc220() -> u32),
        entry!(0x004dc230, fn_004dc230() -> u32),
        entry!(0x004dc240, fn_004dc240() -> u32),
        entry!(0x004dc250, fn_004dc250() -> u32),
        entry!(0x004dc260, fn_004dc260() -> u32),
        entry!(0x004dc270, fn_004dc270() -> u32),
        entry!(0x004dc280, fn_004dc280() -> u32),
        entry!(0x004dc290, fn_004dc290() -> u32),
        entry!(0x004dc2a0, fn_004dc2a0() -> u32),
        entry!(0x004dc2b0, fn_004dc2b0() -> i32),
        entry!(0x004dc2c0, fn_004dc2c0() -> u32),
        entry!(0x004dc2d0, fn_004dc2d0(f32)),
        entry!(0x004dc310, fn_004dc310() -> bool),
        entry!(0x004dc330, fn_004dc330(u32)),
        entry!(0x004dc360, fn_004dc360()),
        entry!(0x004dc540, fn_004dc540(Ptr, Ptr)),
        entry!(0x004dc560, fn_004dc560()),
        entry!(0x004dc590, fn_004dc590()),
        entry!(0x004dd130, fn_004dd130(f32, f32) -> f32),
        entry!(0x004d_cce0, fn_004dcce0(Ptr) -> u16),
        entry!(0x004d_c710, fn_004dc710(u32) -> u8),
        entry!(0x004d_cbe0, fn_004dcbe0(u32) -> u32),
        entry!(0x004d_cd00, fn_004dcd00(Ptr, u32) -> u32),
        entry!(0x004d_cd30, fn_004dcd30(Ptr, u32) -> u8),
        entry!(0x004d_cd60, fn_004dcd60(Ptr) -> u8),
        entry!(0x004d_cd80, fn_004dcd80() -> u32),
        entry!(0x004d_cd90, fn_004dcd90() -> u32),
        entry!(0x004d_cda0, fn_004dcda0() -> u32),
        entry!(0x004d_cdb0, fn_004dcdb0(u32) -> u32),
        entry!(0x004d_cdf0, fn_004dcdf0(Ptr, u32) -> u32),
        entry!(0x004d_ce30, fn_004dce30() -> u32),
        entry!(0x004d_ce50, fn_004dce50(Ptr) -> bool),
        entry!(0x004d_d150, fn_004dd150(f32, f32) -> f32),
        entry!(0x004d_d180, fn_004dd180(u32, u32) -> bool),
        entry!(0x004d_d220, fn_004dd220(Ptr) -> u32),
        entry!(0x004d_e070, fn_004de070() -> u8),
        entry!(0x004d_e080, fn_004de080() -> u8),
        entry!(0x004d_d2a0, fn_004dd2a0(Ptr) -> u32),
        entry!(0x004d_d2c0, fn_004dd2c0(u32, Ptr, Ptr, Ptr) -> i32),
        entry!(0x004d_e100, fn_004de100()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interface_and_third_collection_settings_join_their_collections() {
        let mut e = Engine::new();
        let c = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(c, 0x0090_0000);
        e.register_double(PREF_SETTINGS, move |_, _| c.into_ret());
        e.register_double(INTERFACE_SETTINGS, move |_, _| c.into_ret());
        e.register(0x0090_1000, |_, _| Ret::default());
        let b = e.mem.alloc(0xc);
        e.call(0x004de2f0, &[b, 0x50, 1]);
        assert_eq!(e.mem.u32(b), PREF_SETTING_VTABLE);
        assert_eq!(e.mem.u8(b + 4), 1);
        let f = e.mem.alloc(0xc);
        e.call(0x004de370, &[f, 0x60, 0x7f80_0004]);
        assert_eq!(e.mem.u32(f), INTERFACE_SETTING_VTABLE);
        assert_eq!(e.mem.u32(f + 4), 0x7fc0_0004);
    }
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Pages holding the globals, settings and constants the code touches.
    const PAGES: [u32; 17] = [
        0x0101_1000,
        0x0101_2000,
        0x0101_d000,
        0x0102_1000,
        0x0118_9000,
        0x011a_2000,
        0x011a_9000,
        0x011a_d000,
        0x011c_3000,
        0x011c_6000,
        0x011c_7000,
        0x011d_e000,
        0x011f_4000,
        0x011f_9000,
        0x0120_2000,
        0x0126_e000,
        0x0126_f000,
    ];

    /// An engine with the globals mapped, the constants the code reads from
    /// the exe set, and doubles for the setting getters and setters (a
    /// setting keeps its value at `+4`) and the smart pointer.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in PAGES {
            e.map(page, 0x1000);
        }
        e.set_global(TWO, 2.0f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(FOUR_BY_THREE, 0.75f64);
        e.register(SETTING_INT_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_BOOL_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_FLOAT_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_INT_VALUE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SETTING_BOOL_VALUE, |e, a| {
            (e.mem.u8(a[0] + 4) != 0).into_ret()
        });
        e.register(SETTING_INT_SET, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(SETTING_BOOL_SET, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        e.register(SMART_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SMART_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(ADAPTER_SECOND_NAME, |_, a| (a[0] + 4).into_ret());
        e
    }

    /// A double that returns `value` in `EAX`.
    fn returning(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// Doubles that do nothing and return 0.
    fn ignoring(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            returning(e, *addr, 0);
        }
    }

    /// The log entries for `addr`, in order.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn set_setting_int(e: &mut Engine, setting: u32, value: i32) {
        e.mem.set_i32(setting + 4, value);
    }

    fn set_setting_flag(e: &mut Engine, setting: u32, value: bool) {
        e.mem.set_u8(setting + 4, value as u8);
    }

    fn set_setting_float(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    macro_rules! global_getter_test {
        ($name:ident, $addr:literal, $global:expr) => {
            #[test]
            fn $name() {
                let mut e = rig();
                e.set_global($global, 0x1234_5678u32);
                assert_eq!(e.call($addr, &args![]).u32(), 0x1234_5678);
            }
        };
    }

    // ---- Renderer::Kill ------------------------------------------------

    /// Doubles for `Renderer::Kill`: the reference count is `references`.
    fn kill_rig(references: i32) -> Engine {
        let mut e = rig();
        ignoring(&mut e, &[RENDERER_SHUTDOWN, KILL_NOTE, LOG_MESSAGE]);
        returning(&mut e, REFERENCE_COUNT, references as u32);
        e.mem.set_u32(RENDERER_POINTER, 0x4000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn kill_without_report_only_shuts_down_and_clears_the_pointer() {
        let mut e = kill_rig(5);
        e.call(0x004d_a4d0, &args![0u8]);
        assert_eq!(calls_to(&e, RENDERER_SHUTDOWN), vec![vec![0]]);
        assert!(calls_to(&e, LOG_MESSAGE).is_empty());
        assert!(calls_to(&e, REFERENCE_COUNT).is_empty());
        assert_eq!(e.mem.u32(RENDERER_POINTER), 0);
    }

    #[test]
    fn kill_with_the_only_reference_logs_nothing() {
        let mut e = kill_rig(1);
        e.call(0x004d_a4d0, &args![1u8]);
        assert_eq!(calls_to(&e, RENDERER_SHUTDOWN), vec![vec![1]]);
        assert!(calls_to(&e, KILL_NOTE).is_empty());
        assert!(calls_to(&e, LOG_MESSAGE).is_empty());
        assert_eq!(e.mem.u32(RENDERER_POINTER), 0);
    }

    #[test]
    fn kill_with_one_extra_reference_logs_the_singular_message() {
        let mut e = kill_rig(2);
        e.call(0x004d_a4d0, &args![1u8]);
        assert_eq!(calls_to(&e, KILL_NOTE).len(), 1);
        assert_eq!(calls_to(&e, LOG_MESSAGE), vec![vec![MESSAGE_ONE_REFERENCE]]);
    }

    #[test]
    fn kill_with_many_references_logs_the_count_minus_one() {
        let mut e = kill_rig(6);
        e.call(0x004d_a4d0, &args![1u8]);
        assert_eq!(
            calls_to(&e, LOG_MESSAGE),
            vec![vec![MESSAGE_MANY_REFERENCES, 5]]
        );
    }

    #[test]
    fn kill_without_a_renderer_skips_the_report() {
        let mut e = kill_rig(6);
        e.mem.set_u32(RENDERER_POINTER, 0);
        e.call(0x004d_a4d0, &args![1u8]);
        assert!(calls_to(&e, REFERENCE_COUNT).is_empty());
        assert!(calls_to(&e, LOG_MESSAGE).is_empty());
    }

    // ---- 004da570 -----------------------------------------------------

    /// Doubles for `004da570`: an adapter whose name is `adapter_name`
    /// (when `with_adapter`), and a setting text `setting_text`.
    fn device_rig(with_adapter: bool, adapter_name: &str, setting_text: &str) -> (Engine, u32) {
        let mut e = rig();
        let adapter = e.mem.alloc(0x500);
        e.mem
            .set_cstr(adapter + ADAPTER_NAME_OFFSET, adapter_name.as_bytes());
        let text = e.mem.alloc(0x40);
        e.mem.set_cstr(text, setting_text.as_bytes());
        returning(&mut e, ADAPTER_LIST, 1);
        returning(
            &mut e,
            ADAPTER_DESCRIPTION,
            if with_adapter { adapter } else { 0 },
        );
        returning(&mut e, STRING_TEXT, text);
        e.register(STRICMP, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            (left.cmp(&right) as i32).into_ret()
        });
        ignoring(&mut e, &[STRING_COPY, STRCAT, STRING_SETTING_SET]);
        e.call_log = Some(vec![]);
        (e, adapter)
    }

    #[test]
    fn device_check_reports_a_different_adapter_name() {
        let (mut e, adapter) = device_rig(true, "NVIDIA GeForce", "AMD Radeon");
        assert!(e.call(0x004d_a570, &args![]).bool());
        assert_eq!(calls_to(&e, ADAPTER_SECOND_NAME), vec![vec![adapter]]);
        assert!(calls_to(&e, STRING_SETTING_SET).is_empty());
    }

    #[test]
    fn device_check_rewrites_the_setting_for_the_same_name() {
        let (mut e, _) = device_rig(true, "NVIDIA GeForce", "nvidia geforce");
        assert!(!e.call(0x004d_a570, &args![]).bool());
        let copy = calls_to(&e, STRING_COPY);
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0][1], DEVICE_NAME_DELIMITER);
        let appended = calls_to(&e, STRCAT);
        assert_eq!(appended.len(), 2);
        assert_eq!(appended[1][1], DEVICE_NAME_DELIMITER);
        let set = calls_to(&e, STRING_SETTING_SET);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][0], S_D3D_DEVICE);
        assert_eq!(set[0][1], copy[0][0]);
    }

    #[test]
    fn device_check_rewrites_the_setting_without_an_adapter() {
        let (mut e, _) = device_rig(false, "", "anything");
        assert!(!e.call(0x004d_a570, &args![]).bool());
        assert_eq!(calls_to(&e, STRING_SETTING_SET).len(), 1);
        assert!(calls_to(&e, STRICMP).is_empty());
    }

    #[test]
    fn device_check_rewrites_the_setting_without_an_adapter_list() {
        let (mut e, _) = device_rig(true, "x", "y");
        returning(&mut e, ADAPTER_LIST, 0);
        assert!(!e.call(0x004d_a570, &args![]).bool());
        assert!(calls_to(&e, ADAPTER_DESCRIPTION).is_empty());
        assert_eq!(calls_to(&e, STRING_SETTING_SET).len(), 1);
    }

    #[test]
    fn adapter_name_is_inside_the_description() {
        let mut e = rig();
        assert_eq!(e.call(0x004d_a650, &args![0x1000u32]).u32(), 0x1204);
    }

    // ---- Renderer::Init -----------------------------------------------

    const FILE_HANDLE: u32 = 0x7777;
    const VTABLE_RENDERER: u32 = 0x0300_0000;
    const VTABLE_DIRECT3D: u32 = 0x0300_1000;
    const RENDERER_SET_TABLE: u32 = 0x0400_00b0;
    const RENDERER_OBJECT_METHOD: u32 = 0x0400_013c;
    const CHECK_DEVICE_FORMAT: u32 = 0x0400_0028;
    const INIT: u32 = 0x004d_a670;

    /// What `init_rig` hands to the tests.
    struct InitRig {
        e: Engine,
        renderer: u32,
        adapter: u32,
        caps: u32,
        /// `(usage, resource type, format)` of every `CheckDeviceFormat`.
        queries: Rc<RefCell<Vec<(u32, u32, u32)>>>,
    }

    /// Doubles for every function `Renderer::Init` calls outside this file
    /// (and the later part of the unit), with a full-screen configuration
    /// that succeeds: 640x480, no multisampling, a device that supports
    /// every format.
    fn init_rig() -> InitRig {
        let mut e = rig();
        ignoring(
            &mut e,
            &[
                SCOPE_GUARD_CTOR,
                SCOPE_GUARD_DTOR,
                SPRINTF,
                FCLOSE,
                EYEFINITY_SETUP,
                EYEFINITY_ACTIVE,
                IMPORT_ADJUST_WINDOW_RECT,
                IMPORT_SET_WINDOW_POS,
                SHADER_LEVEL_STRING,
                SET_PERF_OPTIONS,
                IMAGE_CONVERTER_INSTALL,
                SHADER_MANAGER_CONFIGURE,
                SHADOW_FILTER_MAP,
                SHADOW_FILTER_BUILD,
                SHADER_TARGET_PIXEL,
                SHADER_TARGET_VERTEX,
                SHADER_PACKAGE,
                IMPORT_GET_CLASS_LONG,
                IMPORT_GET_WINDOW_LONG,
                NEW_OBJECT,
                MULTISAMPLE_SUPPORTED,
                FPRINTF,
                STRING_COPY,
            ],
        );
        returning(&mut e, PREPARE_DISPLAY_MODE, 1);
        returning(&mut e, FOPEN, FILE_HANDLE);
        returning(&mut e, IMPORT_GET_SYSTEM_METRICS, 1);
        returning(&mut e, ACTOR_SHADOW_COUNT_PAIR, 0x0201);
        e.register(FLOAT4_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });

        let renderer = e.mem.alloc(0x300);
        e.put_vtable(VTABLE_RENDERER, &[0; 80]);
        e.mem.set_u32(VTABLE_RENDERER + 0xb0, RENDERER_SET_TABLE);
        e.mem
            .set_u32(VTABLE_RENDERER + 0x13c, RENDERER_OBJECT_METHOD);
        e.mem.set_u32(renderer, VTABLE_RENDERER);
        ignoring(&mut e, &[RENDERER_SET_TABLE, RENDERER_OBJECT_METHOD]);
        returning(&mut e, CREATE_RENDERER, renderer);

        // The Direct3D object: every query succeeds and is recorded.
        let direct3d = e.mem.alloc(8);
        e.put_vtable(VTABLE_DIRECT3D, &[0; 20]);
        e.mem.set_u32(VTABLE_DIRECT3D + 0x28, CHECK_DEVICE_FORMAT);
        e.mem.set_u32(direct3d, VTABLE_DIRECT3D);
        e.set_global(DIRECT3D_OBJECT, direct3d);
        let queries = Rc::new(RefCell::new(Vec::new()));
        let recorded = queries.clone();
        e.register_double(CHECK_DEVICE_FORMAT, move |_, a| {
            // this, adapter, device type, adapter format, usage, type, format
            assert_eq!(&a[1..4], &[0, 1, FORMAT_DISPLAY]);
            recorded.borrow_mut().push((a[4], a[5], a[6]));
            Ret::default()
        });

        // The adapter description with a hardware device information object
        // whose capabilities follow its handle word.
        let adapter = e.mem.alloc(0x500);
        let info = e.mem.alloc(0x140);
        e.mem.set_u32(info + 4, 1);
        e.mem.set_u32(adapter + ADAPTER_HAL_INFO_OFFSET, info);
        returning(&mut e, ADAPTER_DESCRIPTION, adapter);
        let caps = info + 4;
        e.mem.set_u32(caps + CAPS_MAX_PS_INSTRUCTIONS, 0x200);
        e.mem.set_u32(caps + CAPS_MAX_PS_INSTRUCTIONS_LIMIT, 0x100);
        e.mem.set_u32(caps + CAPS_PIXEL_SHADER_VERSION, 0xffff_0300);
        e.mem
            .set_u32(caps + CAPS_VERTEX_SHADER_VERSION, 0xffff_0200);

        set_setting_int(&mut e, I_SIZE_W, 640);
        set_setting_int(&mut e, I_SIZE_H, 480);
        set_setting_flag(&mut e, B_FULL_SCREEN, true);
        e.call_log = Some(vec![]);
        InitRig {
            e,
            renderer,
            adapter,
            caps,
            queries,
        }
    }

    /// Runs `Renderer::Init` with window `1` and instance `2`.
    fn run_init(rig: &mut InitRig) -> u32 {
        rig.e.call(INIT, &args![1u32, 2u32]).u32()
    }

    #[test]
    fn init_returns_null_when_the_display_mode_cannot_be_prepared() {
        let mut rig = init_rig();
        returning(&mut rig.e, PREPARE_DISPLAY_MODE, 0);
        let result = rig.e.call(INIT, &args![0x111u32, 0x222u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(rig.e.global::<u32>(PARENT_WINDOW), 0x111);
        assert_eq!(rig.e.global::<u32>(MODULE_INSTANCE), 0x222);
        // The guard is built with tag 4, line 0x179, and destroyed again.
        let ctor = calls_to(&rig.e, SCOPE_GUARD_CTOR);
        assert_eq!(&ctor[0][1..], &[4, 1, SOURCE_FILE, 0x179]);
        assert_eq!(calls_to(&rig.e, SCOPE_GUARD_DTOR), vec![vec![ctor[0][0]]]);
        assert!(calls_to(&rig.e, CREATE_RENDERER).is_empty());
        // RendererInfo.txt was created empty ("w") and closed.
        assert_eq!(calls_to(&rig.e, FOPEN)[0][1], MODE_WRITE);
        assert_eq!(calls_to(&rig.e, FCLOSE), vec![vec![FILE_HANDLE]]);
    }

    #[test]
    fn init_uses_the_given_window_when_full_screen() {
        let mut rig = init_rig();
        set_setting_int(&mut rig.e, I_PRESENT_INTERVAL, 1);
        let result = rig.e.call(INIT, &args![0x111u32, 0x222u32]).u32();
        assert_eq!(result, rig.renderer);
        assert_eq!(rig.e.mem.u32(RENDERER_POINTER), rig.renderer);
        assert_eq!(rig.e.global::<u32>(RENDER_WIDTH), 640);
        assert_eq!(rig.e.global::<u32>(RENDER_HEIGHT), 480);
        assert_eq!(rig.e.global::<u32>(PRESENT_INTERVAL), 1);
        assert_eq!(rig.e.global::<u32>(DISPLAY_FLAGS), 4);
        assert_eq!(rig.e.global::<u32>(RENDER_WINDOW), 0x111);
        assert_eq!(rig.e.global::<u8>(NOT_FOUR_BY_THREE), 0);
        assert!(calls_to(&rig.e, IMPORT_CREATE_WINDOW_EX).is_empty());
        // The renderer is created from the fifteen getter words, reversed.
        let create = calls_to(&rig.e, CREATE_RENDERER);
        assert_eq!(create[0].len(), 15);
        assert_eq!(&create[0][..2], &[640, 480]);
        assert_eq!(create[0][14], 0);
        // The renderer's slot 0xb0 gets the table of `011c74b8`.
        assert_eq!(
            calls_to(&rig.e, RENDERER_SET_TABLE),
            vec![vec![rig.renderer, TABLE_011C74B8]]
        );
    }

    #[test]
    fn init_creates_a_child_window_when_not_full_screen() {
        let mut rig = init_rig();
        set_setting_flag(&mut rig.e, B_FULL_SCREEN, false);
        set_setting_int(&mut rig.e, I_LOCATION_X, 30);
        set_setting_int(&mut rig.e, I_LOCATION_Y, 40);
        rig.e.set_global(WINDOW_CLASS_NAME, 0x5555u32);
        returning(&mut rig.e, IMPORT_CREATE_WINDOW_EX, 0x999);
        returning(&mut rig.e, IMPORT_GET_CLASS_LONG, 1);
        returning(&mut rig.e, IMPORT_GET_WINDOW_LONG, 0xcafe);
        rig.e.register(IMPORT_ADJUST_WINDOW_RECT, |e, a| {
            // The frame adds 16 x 39 pixels around the client area.
            e.mem.set_i32(a[0] + 8, e.mem.i32(a[0] + 8) + 16);
            e.mem.set_i32(a[0] + 12, e.mem.i32(a[0] + 12) + 39);
            Ret::default()
        });
        rig.e.call(INIT, &args![0x111u32, 0x222u32]);
        let create = calls_to(&rig.e, IMPORT_CREATE_WINDOW_EX);
        assert_eq!(
            create[0],
            vec![
                0,
                0x5555,
                0,
                0x5000_0000,
                0,
                0,
                640,
                480,
                0x111,
                0,
                0x222,
                0
            ]
        );
        assert_eq!(rig.e.global::<u32>(RENDER_WINDOW), 0x999);
        assert_eq!(rig.e.global::<u32>(DISPLAY_FLAGS), 0);
        assert_eq!(
            calls_to(&rig.e, IMPORT_GET_CLASS_LONG)[0],
            vec![0x999, -8i32 as u32]
        );
        assert_eq!(
            calls_to(&rig.e, IMPORT_GET_WINDOW_LONG)[0],
            vec![0x111, -16i32 as u32]
        );
        let adjust = calls_to(&rig.e, IMPORT_ADJUST_WINDOW_RECT);
        assert_eq!(&adjust[0][1..], &[0xcafe, 1]);
        let position = calls_to(&rig.e, IMPORT_SET_WINDOW_POS);
        assert_eq!(position[0], vec![0x111, 0, 30, 40, 656, 519, 0x40]);
    }

    #[test]
    fn init_reports_a_size_mismatch_on_a_multi_monitor_desktop() {
        let mut rig = init_rig();
        rig.e.register(IMPORT_GET_SYSTEM_METRICS, |_, a| {
            // monitors, screen height, screen width
            match a[0] {
                0x50 => 2u32.into_ret(),
                1 => 480u32.into_ret(),
                _ => 800u32.into_ret(),
            }
        });
        run_init(&mut rig);
        let create = calls_to(&rig.e, CREATE_RENDERER);
        assert_eq!(create[0][14], 1);
    }

    #[test]
    fn init_takes_the_eyefinity_size_and_limits_multisampling() {
        let mut rig = init_rig();
        returning(&mut rig.e, EYEFINITY_ACTIVE, 1);
        set_setting_flag(&mut rig.e, B_ENABLE_EYEFINITY, true);
        rig.e.register(EYEFINITY_WIDTH, |_, _| Ret {
            st0: 5760.9,
            ..Ret::default()
        });
        rig.e.register(EYEFINITY_HEIGHT, |_, _| Ret {
            st0: 1080.0,
            ..Ret::default()
        });
        set_setting_int(&mut rig.e, I_MULTI_SAMPLE, 8);
        run_init(&mut rig);
        assert_eq!(calls_to(&rig.e, EYEFINITY_SETUP).len(), 1);
        // FISTP truncates toward zero.
        assert_eq!(rig.e.global::<u32>(RENDER_WIDTH), 5760);
        assert_eq!(rig.e.global::<u32>(RENDER_HEIGHT), 1080);
        assert_eq!(
            calls_to(&rig.e, SETTING_INT_SET)[0],
            vec![I_MULTI_SAMPLE, 2]
        );
    }

    #[test]
    fn init_keeps_a_low_multisample_setting_for_eyefinity() {
        let mut rig = init_rig();
        returning(&mut rig.e, EYEFINITY_ACTIVE, 1);
        rig.e.register(EYEFINITY_WIDTH, |_, _| Ret::default());
        rig.e.register(EYEFINITY_HEIGHT, |_, _| Ret::default());
        set_setting_int(&mut rig.e, I_MULTI_SAMPLE, 1);
        run_init(&mut rig);
        assert_eq!(
            calls_to(&rig.e, SETTING_INT_SET)[0],
            vec![I_MULTI_SAMPLE, 1]
        );
    }

    #[test]
    fn init_maps_the_supported_multisample_setting_to_a_type() {
        for (setting, expected) in [(2, 2), (3, 2), (4, 4), (5, 0), (8, 8), (16, 0)] {
            let mut rig = init_rig();
            set_setting_int(&mut rig.e, I_MULTI_SAMPLE, setting);
            returning(&mut rig.e, MULTISAMPLE_SUPPORTED, 1);
            run_init(&mut rig);
            assert_eq!(
                rig.e.global::<u32>(MULTISAMPLE_TYPE),
                expected,
                "setting {setting}"
            );
            assert_eq!(
                calls_to(&rig.e, MULTISAMPLE_SUPPORTED)[0],
                vec![setting as u32, 0x71]
            );
        }
    }

    #[test]
    fn init_switches_unsupported_multisampling_off_and_says_so() {
        let mut rig = init_rig();
        set_setting_int(&mut rig.e, I_MULTI_SAMPLE, 4);
        set_setting_flag(&mut rig.e, B_DO_HIGH_DYNAMIC_RANGE, true);
        set_setting_flag(&mut rig.e, B_TRANSPARENCY_MULTISAMPLING, true);
        returning(&mut rig.e, MULTISAMPLE_SUPPORTED, 0);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u32>(MULTISAMPLE_TYPE), 0);
        let modes: Vec<u32> = calls_to(&rig.e, FOPEN).iter().map(|c| c[1]).collect();
        assert_eq!(modes, vec![MODE_WRITE, MODE_APPEND, MODE_APPEND]);
        let message = calls_to(&rig.e, FPRINTF)[0].clone();
        assert_eq!(message, vec![FILE_HANDLE, MESSAGE_MULTISAMPLE, 4]);
        assert_eq!(
            calls_to(&rig.e, SETTING_BOOL_SET)[0],
            vec![B_TRANSPARENCY_MULTISAMPLING, 0]
        );
    }

    #[test]
    fn init_fails_when_the_renderer_was_not_created() {
        let mut rig = init_rig();
        returning(&mut rig.e, CREATE_RENDERER, 0);
        let result = run_init(&mut rig);
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&rig.e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_RENDERER_CREATE]]
        );
        assert_eq!(calls_to(&rig.e, SCOPE_GUARD_DTOR).len(), 1);
    }

    #[test]
    fn init_adjusts_the_renderer_object_when_asked() {
        let mut rig = init_rig();
        rig.e.set_global(RENDER_TARGET_ADJUST_FLAG, 1u8);
        set_setting_int(&mut rig.e, I_N_PATCHES, 7);
        let object = rig.e.mem.alloc(8);
        rig.e
            .mem
            .set_u32(rig.renderer + RENDERER_OBJECT_OFFSET, object);
        rig.e.mem.set_u32(object, VTABLE_RENDERER);
        run_init(&mut rig);
        let calls = calls_to(&rig.e, RENDERER_OBJECT_METHOD);
        assert_eq!(calls, vec![vec![object, 7.0f32.to_bits()]]);
    }

    #[test]
    fn init_skips_the_renderer_object_without_one() {
        let mut rig = init_rig();
        rig.e.set_global(RENDER_TARGET_ADJUST_FLAG, 1u8);
        run_init(&mut rig);
        assert!(calls_to(&rig.e, RENDERER_OBJECT_METHOD).is_empty());
    }

    #[test]
    fn init_installs_the_image_converter() {
        let mut rig = init_rig();
        returning(&mut rig.e, NEW_OBJECT, 0x6000);
        returning(&mut rig.e, IMAGE_CONVERTER_CONSTRUCT, 0x6008);
        run_init(&mut rig);
        assert_eq!(calls_to(&rig.e, NEW_OBJECT), vec![vec![0xa00]]);
        assert_eq!(
            calls_to(&rig.e, IMAGE_CONVERTER_CONSTRUCT),
            vec![vec![0x6000]]
        );
        assert_eq!(
            calls_to(&rig.e, IMAGE_CONVERTER_INSTALL),
            vec![vec![0x6008]]
        );
        assert_eq!(rig.e.global::<u8>(FLAG_011A9594), 1);
        assert_eq!(rig.e.global::<u8>(FLAG_011F4508), 0);
    }

    #[test]
    fn init_installs_a_null_converter_when_allocation_fails() {
        let mut rig = init_rig();
        run_init(&mut rig);
        assert!(calls_to(&rig.e, IMAGE_CONVERTER_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&rig.e, IMAGE_CONVERTER_INSTALL), vec![vec![0]]);
    }

    #[test]
    fn init_fails_without_an_adapter() {
        let mut rig = init_rig();
        returning(&mut rig.e, ADAPTER_DESCRIPTION, 0);
        let result = run_init(&mut rig);
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&rig.e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_ADAPTER_DESC]]
        );
        assert!(calls_to(&rig.e, SET_PERF_OPTIONS).is_empty());
    }

    #[test]
    fn init_fails_without_device_capabilities() {
        let mut rig = init_rig();
        rig.e.mem.set_u32(rig.adapter + ADAPTER_HAL_INFO_OFFSET, 0);
        // A null info object has its handle read at address 4, which must
        // exist in this test.
        rig.e.map(0, 0x1000);
        let result = run_init(&mut rig);
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&rig.e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_DEVICE_CAPS]]
        );
        assert_eq!(calls_to(&rig.e, SET_PERF_OPTIONS), vec![vec![1]]);
    }

    #[test]
    fn init_reads_the_capabilities() {
        let mut rig = init_rig();
        let caps = rig.caps;
        rig.e.mem.set_u32(caps + CAPS_DYNAMIC_TEXTURE_BITS, 0x20000);
        rig.e.mem.set_u32(caps + CAPS_FILTER_BITS, 0x400);
        // Power-of-two only (bit 2) without the conditional bit.
        rig.e.mem.set_u32(caps + CAPS_TEXTURE_BITS, 2);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u8>(DYNAMIC_TEXTURES), 1);
        assert_eq!(rig.e.global::<u8>(ANISOTROPIC_MIN_FILTER), 1);
        assert_eq!(rig.e.global::<u8>(NON_POW2_TEXTURES), 0);
        // 0x200 instructions with a limit of 0x100 stay as reported.
        let configure = calls_to(&rig.e, SHADER_MANAGER_CONFIGURE);
        assert_eq!(configure[0][5], 0x200);
    }

    #[test]
    fn init_limits_the_pixel_shader_instructions_on_small_devices() {
        let mut rig = init_rig();
        let caps = rig.caps;
        rig.e
            .mem
            .set_u32(caps + CAPS_MAX_PS_INSTRUCTIONS_LIMIT, 0x10);
        run_init(&mut rig);
        let configure = calls_to(&rig.e, SHADER_MANAGER_CONFIGURE);
        assert_eq!(configure[0][5], 0x60);
    }

    #[test]
    fn init_allows_non_power_of_two_textures_unless_forced_off() {
        for (texture_bits, forced, expected) in [
            (0u32, false, 1u8),
            (2, false, 0),
            (0x102, false, 1),
            (0, true, 0),
        ] {
            let mut rig = init_rig();
            let caps = rig.caps;
            rig.e.mem.set_u32(caps + CAPS_TEXTURE_BITS, texture_bits);
            set_setting_flag(&mut rig.e, B_FORCE_POW2_TEXTURES, forced);
            run_init(&mut rig);
            assert_eq!(rig.e.global::<u8>(NON_POW2_TEXTURES), expected);
        }
    }

    #[test]
    fn init_probes_the_device_formats_in_order() {
        let mut rig = init_rig();
        run_init(&mut rig);
        let queries = rig.queries.borrow().clone();
        // FP16 blending and filtering, 15 formats, X8R8G8B8 blending.
        assert_eq!(queries.len(), 18);
        assert_eq!(queries[0], (0x80000, 3, 0x71));
        assert_eq!(queries[1], (0x20000, 3, 0x71));
        let formats: Vec<u32> = queries[2..17].iter().map(|q| q.2).collect();
        assert_eq!(
            formats,
            vec![
                0x17, 0x18, 0x19, 0x1a, 0x51, 0x14, 0x15, 0x16, 0x72, 0x71, 0x24, 0x74, 0x32, 0x22,
                0x70
            ]
        );
        assert_eq!(queries[17], (0x80000, 3, 0x24));
        // Every query succeeded (the double returns 0).
        assert_eq!(rig.e.global::<u8>(FP16_BLENDING), 1);
        assert_eq!(rig.e.global::<u8>(FP16_FILTERING), 1);
        for i in 0..15 {
            assert_eq!(rig.e.global::<u8>(FORMAT_SUPPORT_TABLE + i), 1);
        }
        assert_eq!(rig.e.global::<u8>(POST_PIXEL_BLEND_X8R8G8B8), 1);
    }

    #[test]
    fn init_marks_unsupported_formats() {
        let mut rig = init_rig();
        rig.e.register_double(CHECK_DEVICE_FORMAT, |_, a| Ret {
            // Fails for the format 0x18 and for the blending usage.
            eax: if a[6] == 0x18 || a[4] == 0x80000 {
                0x8876_0866
            } else {
                0
            },
            ..Ret::default()
        });
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u8>(FORMAT_SUPPORT_TABLE), 1);
        assert_eq!(rig.e.global::<u8>(FORMAT_SUPPORT_TABLE + 1), 0);
        assert_eq!(rig.e.global::<u8>(FP16_BLENDING), 0);
        assert_eq!(rig.e.global::<u8>(FP16_FILTERING), 1);
        assert_eq!(rig.e.global::<u8>(POST_PIXEL_BLEND_X8R8G8B8), 0);
    }

    #[test]
    fn init_enables_hdr_and_bloom_from_the_settings() {
        let mut rig = init_rig();
        set_setting_flag(&mut rig.e, B_DO_HIGH_DYNAMIC_RANGE, true);
        set_setting_flag(&mut rig.e, B_USE_BLUR_SHADER, true);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u8>(HIGH_DYNAMIC_RANGE), 1);
        // Bloom only when HDR is off.
        assert_eq!(rig.e.global::<u8>(BLOOM_LIGHTING), 0);
        let mut rig = init_rig();
        set_setting_flag(&mut rig.e, B_USE_BLUR_SHADER, true);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u8>(HIGH_DYNAMIC_RANGE), 0);
        assert_eq!(rig.e.global::<u8>(BLOOM_LIGHTING), 1);
    }

    #[test]
    fn init_computes_the_offscreen_rectangle() {
        let mut rig = init_rig();
        rig.e.set_global(OFFSCREEN_ENABLED, 1u8);
        // `iSize H` read through its pointer is 360, while the value the
        // start of `Renderer::Init` takes (through the value getter) is 480:
        // fraction = (480 - 360) / 480 / 2.
        let cell = rig.e.mem.alloc(4);
        rig.e.mem.set_i32(cell, 360);
        rig.e.register_double(SETTING_INT_POINTER, move |_, a| {
            (if a[0] == I_SIZE_H { cell } else { a[0] + 4 }).into_ret()
        });
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u8>(OFFSCREEN_ENABLED_COPY), 1);
        assert_eq!(rig.e.global::<f32>(OFFSCREEN_RECT), 0.0);
        assert_eq!(rig.e.global::<f32>(OFFSCREEN_RECT + 4), 1.0);
        assert_eq!(rig.e.global::<f32>(OFFSCREEN_RECT + 8), 0.875);
        assert_eq!(rig.e.global::<f32>(OFFSCREEN_RECT + 12), 0.125);
        assert_eq!(rig.e.global::<u32>(OFFSCREEN_WIDTH), 640);
        assert_eq!(rig.e.global::<u32>(OFFSCREEN_HEIGHT), 360);
    }

    #[test]
    fn init_leaves_the_offscreen_rectangle_when_disabled() {
        let mut rig = init_rig();
        rig.e.set_global(OFFSCREEN_RECT + 8, 5.0f32);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u8>(OFFSCREEN_ENABLED_COPY), 0);
        assert_eq!(rig.e.global::<f32>(OFFSCREEN_RECT + 8), 5.0);
        assert_eq!(rig.e.global::<u32>(OFFSCREEN_WIDTH), 0);
    }

    #[test]
    fn init_decides_transparency_multisampling() {
        for (vendor, hr_fails, flag, samples, expected) in [
            (1i32, false, true, 4, 1u8),
            (1, true, true, 4, 0),
            (2, true, true, 4, 1),
            (3, false, true, 4, 0),
            (1, false, false, 4, 0),
            (1, false, true, 1, 0),
        ] {
            let mut rig = init_rig();
            rig.e.set_global(VENDOR, vendor);
            set_setting_flag(&mut rig.e, B_TRANSPARENCY_MULTISAMPLING, flag);
            set_setting_int(&mut rig.e, I_MULTI_SAMPLE, samples);
            returning(&mut rig.e, MULTISAMPLE_SUPPORTED, 1);
            rig.e.register_double(CHECK_DEVICE_FORMAT, move |_, a| Ret {
                eax: if hr_fails && a[6] == FORMAT_ALPHA_TO_COVERAGE {
                    0x8876_0866
                } else {
                    0
                },
                ..Ret::default()
            });
            // The vendor probes are not under test here.
            ignoring(&mut rig.e, &[GPU_PROBE_INIT, GPU_PROBE_DEVICE_WORD, MEMSET]);
            returning(&mut rig.e, GPU_PROBE_OPEN, 1);
            returning(&mut rig.e, IMPORT_LOAD_LIBRARY, 0);
            run_init(&mut rig);
            assert_eq!(
                rig.e.global::<u8>(TRANSPARENCY_MULTISAMPLING_ACTIVE),
                expected,
                "vendor {vendor} fails {hr_fails} flag {flag} samples {samples}"
            );
        }
    }

    #[test]
    fn init_asks_for_alpha_to_coverage_only_for_the_first_vendor() {
        let mut rig = init_rig();
        rig.e.set_global(VENDOR, 1i32);
        ignoring(&mut rig.e, &[GPU_PROBE_INIT, GPU_PROBE_DEVICE_WORD, MEMSET]);
        returning(&mut rig.e, GPU_PROBE_OPEN, 1);
        run_init(&mut rig);
        let queries = rig.queries.borrow().clone();
        assert!(queries.contains(&(0, 1, FORMAT_ALPHA_TO_COVERAGE)));
        let mut rig = init_rig();
        run_init(&mut rig);
        assert!(!rig
            .queries
            .borrow()
            .iter()
            .any(|q| q.2 == FORMAT_ALPHA_TO_COVERAGE));
    }

    #[test]
    fn init_passes_the_adapter_names_to_the_shader_manager() {
        let mut rig = init_rig();
        rig.e.set_global(SHADER_MANAGER_WORD_A, 11u32);
        rig.e.set_global(SHADER_MANAGER_WORD_B, 22u32);
        set_setting_flag(&mut rig.e, B_FORCE_1X_SHADERS, true);
        run_init(&mut rig);
        let configure = calls_to(&rig.e, SHADER_MANAGER_CONFIGURE);
        assert_eq!(
            configure[0],
            vec![11, 22, 1, rig.adapter + 0x204, rig.adapter + 4, 0x200]
        );
    }

    #[test]
    fn init_turns_grass_shadows_off_on_low_shader_levels() {
        let mut rig = init_rig();
        rig.e.set_global(VALUE_WHEN_CLEAR, 4i32);
        set_setting_flag(&mut rig.e, B_SHADOWS_ON_GRASS, true);
        run_init(&mut rig);
        assert_eq!(rig.e.mem.u8(B_SHADOWS_ON_GRASS + 4), 0);
        let mut rig = init_rig();
        rig.e.set_global(VALUE_WHEN_CLEAR, 5i32);
        set_setting_flag(&mut rig.e, B_SHADOWS_ON_GRASS, true);
        run_init(&mut rig);
        assert_eq!(rig.e.mem.u8(B_SHADOWS_ON_GRASS + 4), 1);
    }

    #[test]
    fn init_flags_a_window_that_is_not_four_by_three() {
        let mut rig = init_rig();
        set_setting_int(&mut rig.e, I_SIZE_W, 1920);
        set_setting_int(&mut rig.e, I_SIZE_H, 1080);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u8>(NOT_FOUR_BY_THREE), 1);
    }

    #[test]
    fn init_stores_the_shadow_settings() {
        let mut rig = init_rig();
        set_setting_int(&mut rig.e, I_SHADOW_MAP_RESOLUTION, 0x1234_0400);
        set_setting_int(&mut rig.e, I_ACTOR_SHADOW_COUNT_EXT, 2);
        set_setting_int(&mut rig.e, I_ACTOR_SHADOW_COUNT_INT, 4);
        set_setting_int(&mut rig.e, I_SHADOW_FILTER, 3);
        run_init(&mut rig);
        // Only the low word of the setting is stored.
        assert_eq!(rig.e.global::<u16>(SHADOW_MAP_RESOLUTION), 0x400);
        // The exterior count is the pair's first argument.
        assert_eq!(calls_to(&rig.e, ACTOR_SHADOW_COUNT_PAIR), vec![vec![2, 4]]);
        assert_eq!(rig.e.global::<u32>(ACTOR_SHADOW_PAIR_RESULT), 0x0201);
        assert_eq!(calls_to(&rig.e, SHADOW_FILTER_MAP), vec![vec![0, 3]]);
    }

    #[test]
    fn init_clamps_the_shadow_mode() {
        for (setting, expected) in [(-1, 3u32), (0, 0), (2, 2), (3, 3), (4, 3)] {
            let mut rig = init_rig();
            set_setting_int(&mut rig.e, I_SHADOW_MODE, setting);
            run_init(&mut rig);
            assert_eq!(
                rig.e.global::<u32>(SHADOW_MODE),
                expected,
                "setting {setting}"
            );
        }
    }

    #[test]
    fn init_copies_the_float_settings_and_adds_the_ranges() {
        let mut rig = init_rig();
        set_setting_float(&mut rig.e, F_LIGHT_LOD_START_FADE, 1000.0);
        set_setting_float(&mut rig.e, F_LIGHT_LOD_RANGE, 500.5);
        set_setting_float(&mut rig.e, F_SHADOW_LOD_START_FADE, 200.0);
        set_setting_float(&mut rig.e, F_SHADOW_LOD_RANGE, 50.25);
        set_setting_float(&mut rig.e, F_SPECULAR_LOD_START_FADE, 300.0);
        set_setting_float(&mut rig.e, F_SPECULAR_LOD_RANGE, 75.0);
        set_setting_float(&mut rig.e, F_TREE_DIMMER, 1.2);
        set_setting_float(&mut rig.e, F_GRASS_DIMMER, 1.3);
        set_setting_float(&mut rig.e, F_ENV_MAP_LOD1, 1500.0);
        set_setting_float(&mut rig.e, F_SKINNED_DECAL_LOD2, 800.0);
        set_setting_float(&mut rig.e, F_LAND_LO_FADE_SECONDS, 15.0);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<f32>(LIGHT_LOD_START), 1000.0);
        assert_eq!(rig.e.global::<f32>(LIGHT_LOD_END), 1500.5);
        assert_eq!(rig.e.global::<f32>(SHADOW_LOD_START), 200.0);
        assert_eq!(rig.e.global::<f32>(SHADOW_LOD_END), 250.25);
        assert_eq!(rig.e.global::<f32>(SPECULAR_LOD_START), 300.0);
        assert_eq!(rig.e.global::<f32>(SPECULAR_LOD_END), 375.0);
        assert_eq!(rig.e.global::<f32>(TREE_DIMMER), 1.2);
        assert_eq!(rig.e.global::<f32>(GRASS_DIMMER), 1.3);
        assert_eq!(rig.e.global::<f32>(ENV_MAP_LOD1), 1500.0);
        assert_eq!(rig.e.global::<f32>(SKINNED_DECAL_LOD2), 800.0);
        assert_eq!(rig.e.global::<f32>(LAND_LO_FADE_SECONDS), 15.0);
    }

    #[test]
    fn init_picks_the_sunlight_dimmer_by_hdr_setting() {
        let mut rig = init_rig();
        set_setting_float(&mut rig.e, F_SUNLIGHT_DIMMER, 1.0);
        set_setting_float(&mut rig.e, F_SUNLIGHT_DIMMER_HDR, 1.3);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<f32>(SUNLIGHT_DIMMER), 1.0);
        let mut rig = init_rig();
        set_setting_float(&mut rig.e, F_SUNLIGHT_DIMMER, 1.0);
        set_setting_float(&mut rig.e, F_SUNLIGHT_DIMMER_HDR, 1.3);
        set_setting_flag(&mut rig.e, B_DO_HIGH_DYNAMIC_RANGE, true);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<f32>(SUNLIGHT_DIMMER), 1.3);
    }

    #[test]
    fn init_builds_the_interface_tint() {
        let mut rig = init_rig();
        set_setting_float(&mut rig.e, F_INTERFACE_TINT_R, 0.25);
        set_setting_float(&mut rig.e, F_INTERFACE_TINT_G, 0.5);
        set_setting_float(&mut rig.e, F_INTERFACE_TINT_B, 0.75);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<f32>(INTERFACE_TINT), 0.25);
        assert_eq!(rig.e.global::<f32>(INTERFACE_TINT + 4), 0.5);
        assert_eq!(rig.e.global::<f32>(INTERFACE_TINT + 8), 0.75);
        assert_eq!(rig.e.global::<f32>(INTERFACE_TINT + 12), 1.0);
    }

    #[test]
    fn init_copies_the_remaining_settings() {
        let mut rig = init_rig();
        set_setting_flag(&mut rig.e, B_CREATE_SHADER_PACKAGE, true);
        set_setting_flag(&mut rig.e, B_MT_RENDERING, true);
        set_setting_flag(&mut rig.e, B_LOD_NOISE_ANISO, true);
        set_setting_flag(&mut rig.e, B_30_GRASS_VS, true);
        set_setting_flag(&mut rig.e, B_USE_RESOLVABLE_DEPTH, true);
        set_setting_flag(&mut rig.e, B_USE_FAKE_FULL_SCREEN_MOTION_BLUR, true);
        set_setting_flag(&mut rig.e, B_ALLOW_20_HAIR_SHADER, true);
        set_setting_flag(&mut rig.e, B_USE_REFRACTION_SHADER, true);
        set_setting_int(&mut rig.e, I_TEX_MIP_MAP_SKIP, 2);
        set_setting_int(&mut rig.e, I_TEX_MIP_MAP_MINIMUM, 1);
        set_setting_int(&mut rig.e, I_WATER_MULTI_SAMPLES, 4);
        set_setting_int(&mut rig.e, I_WATER_REFLECT_WIDTH, 512);
        set_setting_int(&mut rig.e, I_WATER_REFLECT_HEIGHT, 256);
        set_setting_float(&mut rig.e, F_GAMMA, 1.5);
        rig.e.set_global(GAMMA, 1.0f32);
        rig.e.set_global(CLEARED_FLAG_9489, 5u8);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u8>(CREATE_SHADER_PACKAGE), 1);
        assert_eq!(rig.e.global::<u8>(MT_RENDERING), 1);
        assert_eq!(rig.e.global::<u8>(LOD_NOISE_ANISO), 1);
        assert_eq!(rig.e.global::<u8>(GRASS_VS_30), 1);
        assert_eq!(rig.e.global::<u8>(RESOLVABLE_DEPTH), 1);
        assert_eq!(rig.e.global::<u8>(FAKE_MOTION_BLUR), 1);
        assert_eq!(rig.e.global::<u8>(HAIR_SHADER_20), 1);
        assert_eq!(rig.e.global::<u8>(REFRACTION), 1);
        assert_eq!(rig.e.global::<u32>(TEX_MIP_MAP_SKIP), 2);
        assert_eq!(rig.e.global::<u32>(TEX_MIP_MAP_MINIMUM), 1);
        assert_eq!(rig.e.global::<u32>(WATER_MULTI_SAMPLES), 4);
        assert_eq!(rig.e.global::<u32>(WATER_REFLECT_WIDTH), 512);
        assert_eq!(rig.e.global::<u32>(WATER_REFLECT_HEIGHT), 256);
        assert_eq!(rig.e.global::<f32>(GAMMA), 1.5);
        assert_eq!(rig.e.global::<u8>(CLEARED_FLAG_9489), 0);
    }

    #[test]
    fn init_copies_the_multisample_type() {
        let mut rig = init_rig();
        set_setting_flag(&mut rig.e, B_ALLOW_SCREEN_SHOT, true);
        run_init(&mut rig);
        assert_eq!(rig.e.global::<u32>(MULTISAMPLE_TYPE), 1);
        assert_eq!(rig.e.global::<u32>(MULTISAMPLE_COPY), 1);
    }

    #[test]
    fn init_writes_the_device_report_in_order() {
        let mut rig = init_rig();
        set_setting_int(&mut rig.e, I_NUM_HW_THREADS, 6);
        set_setting_flag(&mut rig.e, B_USE_WATER_SHADER, true);
        rig.e.set_global(SHADER_30_SUPPORT, 1u8);
        returning(&mut rig.e, SHADER_LEVEL_STRING, 0x5100);
        returning(&mut rig.e, SHADER_TARGET_VERTEX, 0x5200);
        returning(&mut rig.e, SHADER_PACKAGE, 3);
        run_init(&mut rig);
        let modes: Vec<u32> = calls_to(&rig.e, FOPEN).iter().map(|c| c[1]).collect();
        assert_eq!(modes, vec![MODE_WRITE, MODE_APPEND]);
        let lines = calls_to(&rig.e, FPRINTF);
        assert_eq!(lines.len(), 26);
        assert!(lines.iter().all(|line| line[0] == FILE_HANDLE));
        // Device and driver names.
        assert_eq!(lines[0][2..], [rig.adapter + 0x204, rig.adapter + 4]);
        assert_eq!(lines[1], vec![FILE_HANDLE, 0x0102_1434, 0x5100]);
        // The pixel and vertex shader versions are masked to 16 bits.
        assert_eq!(lines[2][2], 0x0300);
        assert_eq!(lines[3][2], 0x0200);
        assert_eq!(lines[4][2], 0x5200);
        assert_eq!(lines[7][2], 0x200);
        assert_eq!(lines[8][2], YES);
        assert_eq!(lines[9][2], NO);
        // Water shader ("yes"), then the other three water lines ("no").
        assert_eq!(lines[18][2], YES);
        assert_eq!(lines[19][2], NO);
        assert_eq!(lines[24][2], 3);
        assert_eq!(lines[25][2], 6);
        assert_eq!(calls_to(&rig.e, FCLOSE).len(), 2);
    }

    #[test]
    fn init_skips_the_report_when_the_file_cannot_be_opened() {
        let mut rig = init_rig();
        returning(&mut rig.e, FOPEN, 0);
        run_init(&mut rig);
        assert!(calls_to(&rig.e, FPRINTF).is_empty());
        assert!(calls_to(&rig.e, FCLOSE).is_empty());
    }

    // ---- The multi-GPU probes -----------------------------------------

    #[test]
    fn init_uses_the_ati_probe_for_vendor_two() {
        let mut rig = init_rig();
        rig.e.set_global(VENDOR, 2i32);
        returning(&mut rig.e, IMPORT_LOAD_LIBRARY, 0x8000);
        returning(&mut rig.e, IMPORT_GET_PROC_ADDRESS, 0x0500_0000);
        returning(&mut rig.e, 0x0500_0000, 2);
        ignoring(&mut rig.e, &[IMPORT_FREE_LIBRARY]);
        rig.e.set_global(MULTI_GPU_CLEARED_FLAG, 1u8);
        run_init(&mut rig);
        assert_eq!(
            calls_to(&rig.e, IMPORT_LOAD_LIBRARY),
            vec![vec![ATI_LIBRARY]]
        );
        assert_eq!(
            calls_to(&rig.e, IMPORT_GET_PROC_ADDRESS),
            vec![vec![0x8000, ATI_QUERY_COUNT]]
        );
        assert_eq!(rig.e.global::<u8>(MULTI_GPU), 1);
        assert_eq!(rig.e.global::<u8>(MULTI_GPU_CLEARED_FLAG), 0);
        assert_eq!(calls_to(&rig.e, IMPORT_FREE_LIBRARY), vec![vec![0x8000]]);
    }

    #[test]
    fn ati_probe_with_one_adapter_leaves_the_flag_alone() {
        let mut e = rig();
        returning(&mut e, IMPORT_LOAD_LIBRARY, 0x8000);
        returning(&mut e, IMPORT_GET_PROC_ADDRESS, 0x0500_0000);
        returning(&mut e, 0x0500_0000, 1);
        ignoring(&mut e, &[IMPORT_FREE_LIBRARY]);
        probe_multi_gpu_second_vendor(&mut e);
        assert_eq!(e.global::<u8>(MULTI_GPU), 0);
    }

    #[test]
    fn ati_probe_without_the_library_does_nothing() {
        let mut e = rig();
        returning(&mut e, IMPORT_LOAD_LIBRARY, 0);
        e.call_log = Some(vec![]);
        probe_multi_gpu_second_vendor(&mut e);
        assert!(calls_to(&e, IMPORT_GET_PROC_ADDRESS).is_empty());
        assert!(calls_to(&e, IMPORT_FREE_LIBRARY).is_empty());
    }

    #[test]
    fn ati_probe_without_the_entry_point_still_frees_the_library() {
        let mut e = rig();
        returning(&mut e, IMPORT_LOAD_LIBRARY, 0x8000);
        returning(&mut e, IMPORT_GET_PROC_ADDRESS, 0);
        ignoring(&mut e, &[IMPORT_FREE_LIBRARY]);
        e.call_log = Some(vec![]);
        probe_multi_gpu_second_vendor(&mut e);
        assert_eq!(calls_to(&e, IMPORT_FREE_LIBRARY), vec![vec![0x8000]]);
        assert_eq!(e.global::<u8>(MULTI_GPU), 0);
    }

    /// Doubles for the first vendor's probe: `count_a`, `count_b` are the
    /// numbers the two name queries report, `query_status` the device
    /// query's status.
    fn first_vendor_rig(count_a: u32, count_b: u32, query_status: u32) -> Engine {
        let mut e = rig();
        ignoring(&mut e, &[GPU_PROBE_INIT, GPU_PROBE_DEVICE_WORD]);
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        returning(&mut e, GPU_PROBE_OPEN, 0);
        e.register(GPU_PROBE_ENUMERATE, |e, a| {
            // Four adapters answer, the fifth call fails.
            e.mem.set_u32(a[1], 0x100 + a[0]);
            (if a[0] < 4 { 0u32 } else { 1u32 }).into_ret()
        });
        e.register_double(GPU_PROBE_NAMES_A, move |e, a| {
            e.mem.set_u32(a[1], count_a);
            Ret::default()
        });
        e.register_double(GPU_PROBE_NAMES_B, move |e, a| {
            e.mem.set_u32(a[1], count_b);
            Ret::default()
        });
        returning(&mut e, GPU_PROBE_QUERY, query_status);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn first_vendor_probe_sets_the_flag_when_the_query_succeeds_with_a_count() {
        let mut e = first_vendor_rig(1, 2, 0);
        probe_multi_gpu_first_vendor(&mut e);
        assert_eq!(e.global::<u8>(MULTI_GPU), 1);
        // The size word is set and the enumeration stops at the failing
        // adapter.
        let open = calls_to(&e, GPU_PROBE_OPEN);
        assert_eq!(open[0][0], 0);
        assert_eq!(e.mem.u32(open[0][1]), 0x1008c);
        assert_eq!(calls_to(&e, GPU_PROBE_ENUMERATE).len(), 5);
    }

    #[test]
    fn first_vendor_probe_leaves_the_flag_when_the_query_succeeds_with_no_count() {
        let mut e = first_vendor_rig(1, 0, 0);
        probe_multi_gpu_first_vendor(&mut e);
        assert_eq!(e.global::<u8>(MULTI_GPU), 0);
    }

    #[test]
    fn first_vendor_probe_compares_the_counts_when_the_query_fails() {
        let mut e = first_vendor_rig(1, 2, 5);
        probe_multi_gpu_first_vendor(&mut e);
        assert_eq!(e.global::<u8>(MULTI_GPU), 1);
        let mut e = first_vendor_rig(2, 2, 5);
        probe_multi_gpu_first_vendor(&mut e);
        assert_eq!(e.global::<u8>(MULTI_GPU), 0);
    }

    #[test]
    fn first_vendor_probe_stops_when_the_adapters_cannot_be_opened() {
        let mut e = first_vendor_rig(1, 2, 0);
        returning(&mut e, GPU_PROBE_OPEN, 3);
        probe_multi_gpu_first_vendor(&mut e);
        assert!(calls_to(&e, GPU_PROBE_ENUMERATE).is_empty());
        assert_eq!(e.global::<u8>(MULTI_GPU), 0);
    }

    #[test]
    fn first_vendor_probe_stops_when_a_name_query_fails() {
        let mut e = first_vendor_rig(1, 2, 0);
        returning(&mut e, GPU_PROBE_NAMES_A, 1);
        probe_multi_gpu_first_vendor(&mut e);
        assert!(calls_to(&e, GPU_PROBE_NAMES_B).is_empty());
        assert_eq!(e.global::<u8>(MULTI_GPU), 0);
        let mut e = first_vendor_rig(1, 2, 0);
        returning(&mut e, GPU_PROBE_NAMES_B, 1);
        probe_multi_gpu_first_vendor(&mut e);
        assert!(calls_to(&e, GPU_PROBE_QUERY).is_empty());
    }

    #[test]
    fn init_uses_the_first_vendor_probe_for_vendor_one() {
        let mut rig = init_rig();
        rig.e.set_global(VENDOR, 1i32);
        ignoring(&mut rig.e, &[GPU_PROBE_INIT, GPU_PROBE_DEVICE_WORD, MEMSET]);
        returning(&mut rig.e, GPU_PROBE_OPEN, 9);
        run_init(&mut rig);
        assert_eq!(calls_to(&rig.e, GPU_PROBE_OPEN).len(), 1);
    }

    // ---- The small functions --------------------------------------------

    #[test]
    fn flag_setters_store_the_byte() {
        let mut e = rig();
        e.call(0x004d_c010, &args![7u8]);
        assert_eq!(e.global::<u8>(FLAG_011F4508), 7);
    }

    #[test]
    fn second_flag_setter_stores_the_byte() {
        let mut e = rig();
        e.call(0x004d_c050, &args![9u8]);
        assert_eq!(e.global::<u8>(FLAG_011A9594), 9);
    }

    #[test]
    fn refraction_flag_round_trips() {
        let mut e = rig();
        e.call(0x004d_c090, &args![1u8]);
        assert_eq!(e.global::<u8>(REFRACTION), 1);
        assert_eq!(e.call(0x004d_c0a0, &args![]).u8(), 1);
        e.call(0x004d_c090, &args![0u8]);
        assert_eq!(e.call(0x004d_c0a0, &args![]).u8(), 0);
    }

    #[test]
    fn renderer_object_is_read_at_0x288() {
        let mut e = rig();
        let renderer = e.mem.alloc(0x300);
        e.mem.set_u32(renderer + 0x288, 0xbeef);
        assert_eq!(e.call(0x004d_c020, &args![renderer]).u32(), 0xbeef);
    }

    global_getter_test!(direct3d_object_getter, 0x004d_c040, DIRECT3D_OBJECT);
    global_getter_test!(vendor_getter, 0x004d_c0b0, VENDOR);
    global_getter_test!(render_window_getter, 0x004d_c1e0, RENDER_WINDOW);
    global_getter_test!(render_width_getter, 0x004d_c1f0, RENDER_WIDTH);
    global_getter_test!(render_height_getter, 0x004d_c200, RENDER_HEIGHT);
    global_getter_test!(display_flags_getter, 0x004d_c220, DISPLAY_FLAGS);
    global_getter_test!(arg_011c70d4_getter, 0x004d_c230, RENDERER_ARG_011C70D4);
    global_getter_test!(arg_011c70d8_getter, 0x004d_c240, RENDERER_ARG_011C70D8);
    global_getter_test!(arg_01189464_getter, 0x004d_c250, RENDERER_ARG_01189464);
    global_getter_test!(arg_011c70d0_getter, 0x004d_c260, RENDERER_ARG_011C70D0);
    global_getter_test!(arg_01189470_getter, 0x004d_c270, RENDERER_ARG_01189470);
    global_getter_test!(arg_01189474_getter, 0x004d_c280, RENDERER_ARG_01189474);
    global_getter_test!(multisample_type_getter, 0x004d_c290, MULTISAMPLE_TYPE);
    global_getter_test!(present_interval_getter, 0x004d_c2a0, PRESENT_INTERVAL);

    #[test]
    fn selector_picks_between_two_words() {
        let mut e = rig();
        e.set_global(VALUE_WHEN_SET, 11i32);
        e.set_global(VALUE_WHEN_CLEAR, 22i32);
        assert_eq!(e.call(0x004d_c060, &args![1u8]).i32(), 11);
        assert_eq!(e.call(0x004d_c060, &args![0xffu8]).i32(), 11);
        assert_eq!(e.call(0x004d_c060, &args![0u8]).i32(), 22);
    }

    #[test]
    fn shadow_filter_uses_kind_zero_below_level_five() {
        for (level, kind) in [(4i32, 0u32), (5, 2), (-3, 0), (9, 2)] {
            let mut e = rig();
            e.set_global(VALUE_WHEN_CLEAR, level);
            returning(&mut e, SHADOW_FILTER_MAP, 0x55);
            returning(&mut e, SHADOW_FILTER_BUILD, 0x66);
            e.call_log = Some(vec![]);
            e.call(0x004d_c0c0, &args![3i32]);
            assert_eq!(calls_to(&e, SHADOW_FILTER_MAP), vec![vec![kind, 3]]);
            assert_eq!(calls_to(&e, SHADOW_FILTER_BUILD), vec![vec![0, 0x55]]);
            assert_eq!(e.global::<u32>(SHADOW_FILTER_RESULT), 0x66);
        }
    }

    #[test]
    fn actor_shadow_pair_is_stored() {
        let mut e = rig();
        e.call(0x004d_c100, &args![0x0402u32]);
        assert_eq!(e.global::<u32>(ACTOR_SHADOW_PAIR_RESULT), 0x0402);
    }

    #[test]
    fn info_directory_is_a_fixed_string_address() {
        let mut e = rig();
        assert_eq!(e.call(0x004d_c110, &args![]).u32(), 0x0120_2fa0);
    }

    /// An adapter description whose hardware / reference device information
    /// objects have the given handles.
    fn adapter_with_infos(e: &mut Engine, hal_handle: u32, ref_handle: u32) -> (u32, u32, u32) {
        let adapter = e.mem.alloc(0x500);
        let hal = e.mem.alloc(0x20);
        let reference = e.mem.alloc(0x20);
        e.mem.set_u32(hal + 4, hal_handle);
        e.mem.set_u32(reference + 4, ref_handle);
        e.mem.set_u32(adapter + 0x460, hal);
        e.mem.set_u32(adapter + 0x464, reference);
        (adapter, hal, reference)
    }

    #[test]
    fn device_info_selects_the_hardware_or_reference_object() {
        let mut e = rig();
        let (adapter, hal, reference) = adapter_with_infos(&mut e, 1, 1);
        assert_eq!(e.call(0x004d_c160, &args![adapter, 1u32]).u32(), hal);
        assert_eq!(e.call(0x004d_c160, &args![adapter, 0u32]).u32(), reference);
        assert_eq!(e.call(0x004d_c160, &args![adapter, 2u32]).u32(), reference);
    }

    #[test]
    fn device_info_is_null_without_a_handle() {
        let mut e = rig();
        let (adapter, _, _) = adapter_with_infos(&mut e, 0, 0);
        assert_eq!(e.call(0x004d_c160, &args![adapter, 1u32]).u32(), 0);
        assert_eq!(e.call(0x004d_c160, &args![adapter, 0u32]).u32(), 0);
    }

    #[test]
    fn device_caps_are_the_info_object_plus_four() {
        let mut e = rig();
        let (adapter, hal, _) = adapter_with_infos(&mut e, 1, 0);
        assert_eq!(e.call(0x004d_c120, &args![adapter, 1u32]).u32(), hal + 4);
        assert_eq!(e.call(0x004d_c120, &args![adapter, 0u32]).u32(), 0);
    }

    #[test]
    fn device_info_validity_is_the_handle() {
        let mut e = rig();
        let info = e.mem.alloc(0x10);
        assert!(!e.call(0x004d_c1c0, &args![info]).bool());
        e.mem.set_u32(info + 4, 3);
        assert!(e.call(0x004d_c1c0, &args![info]).bool());
    }

    #[test]
    fn adapter_index_is_a_setting() {
        let mut e = rig();
        set_setting_int(&mut e, I_ADAPTER, 2);
        assert_eq!(e.call(0x004d_c210, &args![]).i32(), 2);
    }

    #[test]
    fn patch_count_is_a_setting() {
        let mut e = rig();
        set_setting_int(&mut e, I_N_PATCHES, 9);
        assert_eq!(e.call(0x004d_c2b0, &args![]).i32(), 9);
    }

    #[test]
    fn table_getter_returns_the_table_address() {
        let mut e = rig();
        assert_eq!(e.call(0x004d_c2c0, &args![]).u32(), 0x011c_74b8);
    }

    #[test]
    fn gamma_is_stored_when_it_changes_and_is_positive() {
        let mut e = rig();
        e.set_global(GAMMA, 1.0f32);
        e.call(0x004d_c2d0, &args![1.5f32]);
        assert_eq!(e.global::<f32>(GAMMA), 1.5);
        assert_eq!(e.global::<u8>(GAMMA_CHANGED), 1);
        assert_eq!(e.global::<f32>(GAMMA_COPY), 1.5);
    }

    #[test]
    fn gamma_is_ignored_when_unchanged_or_not_positive() {
        let mut e = rig();
        e.set_global(GAMMA, 1.0f32);
        e.call(0x004d_c2d0, &args![1.0f32]);
        e.call(0x004d_c2d0, &args![0.0f32]);
        e.call(0x004d_c2d0, &args![-2.0f32]);
        e.call(0x004d_c2d0, &args![f32::NAN]);
        assert_eq!(e.global::<f32>(GAMMA), 1.0);
        assert_eq!(e.global::<u8>(GAMMA_CHANGED), 0);
        assert_eq!(e.global::<f32>(GAMMA_COPY), 0.0);
    }

    #[test]
    fn multisample_check_is_signed_two_or_more() {
        let mut e = rig();
        for (value, expected) in [(0i32, false), (1, false), (2, true), (8, true), (-1, false)] {
            e.set_global(MULTISAMPLE_TYPE, value);
            assert_eq!(e.call(0x004d_c310, &args![]).bool(), expected, "{value}");
        }
    }

    #[test]
    fn error_text_is_cleared_then_copied() {
        let mut e = rig();
        ignoring(&mut e, &[STRING_COPY]);
        e.set_global(ERROR_MESSAGE, 0x41u8);
        e.call_log = Some(vec![]);
        e.call(0x004d_c330, &args![0u32]);
        assert_eq!(e.global::<u8>(ERROR_MESSAGE), 0);
        assert!(calls_to(&e, STRING_COPY).is_empty());
        e.call(0x004d_c330, &args![0x1234u32]);
        assert_eq!(calls_to(&e, STRING_COPY), vec![vec![ERROR_MESSAGE, 0x1234]]);
    }

    // ---- Display mode change -------------------------------------------

    /// Doubles for `004dc360` with a requested size of 800x600 and a
    /// renderer (smart pointer content `0x4400`) present.
    fn mode_rig(rebuilt: bool) -> Engine {
        let mut e = rig();
        e.map(0x4000, 0x1000);
        ignoring(
            &mut e,
            &[
                ACCUMULATOR_RESET,
                MOVIE_STOP,
                INTERFACE_SHUTDOWN,
                NO_EFFECT,
                AFTER_RESIZE_A,
                AFTER_RESIZE_B,
                SMART_POINTER_GLOBAL_GETTER,
                AFTER_RESIZE_C,
                AFTER_RESIZE_D,
                AFTER_RESIZE_E,
                INTERFACE_INIT,
                AFTER_RESIZE_F,
                AFTER_RESIZE_G,
                IMPORT_GET_CLASS_LONG,
                IMPORT_GET_WINDOW_LONG,
                IMPORT_ADJUST_WINDOW_RECT,
                IMPORT_SET_WINDOW_POS,
                STRING_COPY,
            ],
        );
        returning(&mut e, RENDERER_GETTER, 0x4400);
        returning(&mut e, RESIZE_SWAP_CHAIN, rebuilt as u32);
        returning(&mut e, WINDOW_HANDLE, 0x9900);
        returning(&mut e, SHADER_ACCUMULATOR, 0x7700);
        e.mem.set_u32(RENDERER_POINTER, 0x4400);
        e.set_global(REQUESTED_WIDTH, 800i32);
        e.set_global(REQUESTED_HEIGHT, 600i32);
        e.set_global(WINDOW_OBJECT, 0x8800u32);
        e.set_global(MOVIE_PLAYER, 0x6600u32);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn mode_change_resizes_the_renderer_and_moves_the_window() {
        let mut e = mode_rig(true);
        set_setting_int(&mut e, I_LOCATION_X, 5);
        set_setting_int(&mut e, I_LOCATION_Y, 6);
        e.mem.set_u32(SMART_POINTER_A, 0xaaaa);
        e.call(0x004d_c360, &args![]);
        assert_eq!(calls_to(&e, ACCUMULATOR_RESET), vec![vec![0x7700]]);
        assert_eq!(
            calls_to(&e, SETTING_INT_SET),
            vec![vec![I_SIZE_W, 800], vec![I_SIZE_H, 600]]
        );
        assert_eq!(e.global::<u8>(NOT_FOUR_BY_THREE), 0);
        assert_eq!(calls_to(&e, MOVIE_STOP), vec![vec![0x6600]]);
        // The two smart pointers are cleared, the renderer's own pointer at
        // +8 is reset, and after the resize set to the accumulator.
        assert_eq!(e.mem.u32(SMART_POINTER_A), 0);
        let assigned = calls_to(&e, SMART_POINTER_SET);
        assert_eq!(assigned[2], vec![0x4408, 0]);
        assert_eq!(assigned[3], vec![0x4408, 0x7700]);
        assert_eq!(
            calls_to(&e, RESIZE_SWAP_CHAIN),
            vec![vec![0x4400, 800, 600]]
        );
        assert_eq!(calls_to(&e, IMPORT_GET_WINDOW_LONG)[0][0], 0x9900);
        assert_eq!(
            calls_to(&e, IMPORT_SET_WINDOW_POS),
            vec![vec![0x9900, 0, 5, 6, 800, 600, 0x40]]
        );
        assert!(calls_to(&e, STRING_COPY).is_empty());
        assert_eq!(calls_to(&e, INTERFACE_INIT), vec![vec![0]]);
        assert_eq!(calls_to(&e, AFTER_RESIZE_G).len(), 1);
    }

    #[test]
    fn mode_change_records_a_failed_rebuild() {
        let mut e = mode_rig(false);
        e.call(0x004d_c360, &args![]);
        assert_eq!(
            calls_to(&e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_RECREATE]]
        );
        assert!(calls_to(&e, IMPORT_SET_WINDOW_POS).is_empty());
        // The rest of the re-initialization still runs.
        assert_eq!(calls_to(&e, AFTER_RESIZE_A).len(), 1);
    }

    #[test]
    fn mode_change_flags_a_size_that_is_not_four_by_three() {
        let mut e = mode_rig(true);
        e.set_global(REQUESTED_WIDTH, 1280i32);
        e.set_global(REQUESTED_HEIGHT, 720i32);
        e.call(0x004d_c360, &args![]);
        assert_eq!(e.global::<u8>(NOT_FOUR_BY_THREE), 1);
    }

    #[test]
    fn mode_change_does_nothing_without_a_renderer_or_size() {
        let mut e = mode_rig(true);
        returning(&mut e, RENDERER_GETTER, 0);
        e.call(0x004d_c360, &args![]);
        assert_eq!(calls_to(&e, RENDERER_GETTER).len(), 1);
        assert!(calls_to(&e, SHADER_ACCUMULATOR).is_empty());
        let mut e = mode_rig(true);
        e.set_global(REQUESTED_HEIGHT, 0i32);
        e.call(0x004d_c360, &args![]);
        assert!(calls_to(&e, SHADER_ACCUMULATOR).is_empty());
        let mut e = mode_rig(true);
        e.set_global(REQUESTED_WIDTH, 0i32);
        e.call(0x004d_c360, &args![]);
        assert!(calls_to(&e, SHADER_ACCUMULATOR).is_empty());
    }

    #[test]
    fn smart_pointer_assignment_goes_through_this_plus_eight() {
        let mut e = rig();
        e.call_log = Some(vec![]);
        e.call(0x004d_c540, &args![SMART_POINTER_A - 8, 0x2000u32]);
        assert_eq!(
            calls_to(&e, SMART_POINTER_SET),
            vec![vec![SMART_POINTER_A, 0x2000]]
        );
        assert_eq!(e.mem.u32(SMART_POINTER_A), 0x2000);
    }

    #[test]
    fn the_first_smart_pointer_is_cleared() {
        let mut e = rig();
        ignoring(&mut e, &[NO_EFFECT]);
        e.mem.set_u32(SMART_POINTER_A, 0xaaaa);
        e.mem.set_u32(SMART_POINTER_B, 0xbbbb);
        e.call_log = Some(vec![]);
        e.call(0x004d_c560, &args![]);
        assert_eq!(calls_to(&e, NO_EFFECT), vec![vec![0xaaaa]]);
        assert_eq!(e.mem.u32(SMART_POINTER_A), 0);
        assert_eq!(e.mem.u32(SMART_POINTER_B), 0xbbbb);
    }

    #[test]
    fn the_second_smart_pointer_is_cleared() {
        let mut e = rig();
        ignoring(&mut e, &[NO_EFFECT]);
        e.mem.set_u32(SMART_POINTER_A, 0xaaaa);
        e.mem.set_u32(SMART_POINTER_B, 0xbbbb);
        e.call_log = Some(vec![]);
        e.call(0x004d_c590, &args![]);
        assert_eq!(calls_to(&e, NO_EFFECT), vec![vec![0xbbbb]]);
        assert_eq!(e.mem.u32(SMART_POINTER_B), 0);
        assert_eq!(e.mem.u32(SMART_POINTER_A), 0xaaaa);
    }

    #[test]
    fn pow_wrapper_forwards_both_floats_and_returns_the_result() {
        let mut e = Engine::new();
        e.register(POWF, |_, a| {
            f32::from_bits(a[0]).powf(f32::from_bits(a[1])).into_ret()
        });
        assert_eq!(e.call(0x004d_d130, &args![2.0f32, 10.0f32]).f32(), 1024.0);
    }

    #[test]
    fn pow_wrapper_quiets_a_signalling_nan_argument() {
        let mut e = Engine::new();
        e.register(POWF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x004d_d130, &args![f32::from_bits(0x7f80_0001), 1.0f32]);
        let log = e.call_log.take().unwrap();
        let forwarded = log.iter().find(|(addr, _)| *addr == POWF).unwrap();
        assert_eq!(forwarded.1, vec![0x7fc0_0001, 1.0f32.to_bits()]);
    }

    #[test]
    fn l68d_004dcce0_passes_this_plus_450_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00658930, |_, a| Ret {
            eax: a[0] | 0x5_0000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_cce0, &args![0x4100u32]).u16(), 0x4550);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00658930).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x450]);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_004de290_stores_the_float_and_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x004d_e290, &args![this, 1.5f32]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        e.call(0x004d_e290, &args![this, f32::from_bits(0xff80_0001)]);
        assert_eq!(e.mem.u32(this + 4), 0xffc0_0001);
    }

    #[test]
    fn at_004de290_leaves_a_null_this_alone() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004d_e290, &args![0u32, 1.5f32]).u32(), 0);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    fn rig(allocation_fails: bool) -> Engine {
        let mut e = Engine::new();
        e.register_double(NEW_OBJECT, move |e, a| {
            if allocation_fails {
                0u32.into_ret()
            } else {
                e.mem.alloc(a[0]).into_ret()
            }
        });
        e.register(SCENE_GRAPH_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SMART_POINTER_SET, |_, _| Ret::default());
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_004dc5c0_makes_the_menu_scene_graph() {
        let mut e = rig(false);
        e.call_log = Some(vec![]);
        e.call(0x004d_c5c0, &args![]);
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![0xc0]]);
        let graph = calls(&e, SCENE_GRAPH_CONSTRUCT);
        assert_eq!(&graph[0][1..], &[MENU_NAME, 1, 0]);
        assert_eq!(
            calls(&e, SMART_POINTER_SET),
            vec![vec![MENU_SCENE_GRAPH, graph[0][0]]]
        );
    }

    #[test]
    fn fn_004dc650_makes_the_3d_menu_scene_graph() {
        let mut e = rig(false);
        e.call_log = Some(vec![]);
        e.call(0x004d_c650, &args![]);
        let graph = calls(&e, SCENE_GRAPH_CONSTRUCT);
        assert_eq!(&graph[0][1..], &[MENU_3D_NAME, 0, 0]);
        assert_eq!(
            calls(&e, SMART_POINTER_SET),
            vec![vec![MENU_3D_SCENE_GRAPH, graph[0][0]]]
        );
    }

    #[test]
    fn scene_graphs_assign_null_when_the_allocation_fails() {
        let mut e = rig(true);
        e.call_log = Some(vec![]);
        e.call(0x004d_c5c0, &args![]);
        e.call(0x004d_c650, &args![]);
        assert!(calls(&e, SCENE_GRAPH_CONSTRUCT).is_empty());
        assert_eq!(
            calls(&e, SMART_POINTER_SET),
            vec![vec![MENU_SCENE_GRAPH, 0], vec![MENU_3D_SCENE_GRAPH, 0]]
        );
    }
}

#[cfg(test)]
mod display_mode_tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn mapped() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x0102_1000,
            0x0118_9000,
            0x011c_6000,
            0x011c_7000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
        e
    }

    #[test]
    fn display_format_table() {
        let mut e = Engine::new();
        let formats: Vec<u32> = (0..20)
            .map(|i| e.call(0x004d_cbe0, &args![i as u32]).u32())
            .collect();
        assert_eq!(
            formats,
            vec![
                0, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x6f, 0x70,
                0x24, 0x71, 0x72, 0x73, 0x74, 0
            ]
        );
    }

    #[test]
    fn small_getters_read_their_globals_and_fields() {
        let mut e = mapped();
        e.set_global(SHADER_MANAGER_WORD_A, 0x300u32);
        e.set_global(SHADER_MANAGER_WORD_B, 0x200u32);
        e.set_global(EYEFINITY_FLAG, 1u8);
        assert_eq!(e.call(0x004d_cd80, &[]).u32(), 0x300);
        assert_eq!(e.call(0x004d_cd90, &[]).u32(), 0x200);
        assert_eq!(e.call(0x004d_e070, &[]).u8(), 1);
        assert_eq!(e.call(0x004d_e080, &[]).u8(), 0);
        e.set_global(EYEFINITY_SECOND_FLAG, 2u8);
        assert_eq!(e.call(0x004d_e080, &[]).u8(), 1);
        let object = e.mem.alloc(0x600);
        e.mem.set_u32(object + 0x5c8, 7);
        e.mem.set_u8(object + 0x140, 9);
        assert_eq!(e.call(0x004d_d220, &args![object]).u32(), 7);
        assert_eq!(e.call(0x004d_cd60, &args![object]).u8(), 9);
        e.register(SETTING_BOOL_VALUE, |_, a| (a[0] & 0xff).into_ret());
        assert_eq!(e.call(0x004d_cda0, &[]).u32(), B_USE_SHADERS & 0xff);
    }

    #[test]
    fn adapter_list_is_made_once_and_indexed() {
        let mut e = mapped();
        // A list: count, then an array whose element `i` holds `0x100 + i`.
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, 2);
        e.register_double(ADAPTER_LIST_CREATE, move |_, _| list.into_ret());
        e.register(SMART_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARRAY_COUNT, |_, _| 2u32.into_ret());
        e.register(ARRAY_ELEMENT, |e, a| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, 0x100 + a[1]);
            slot.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_ce30, &[]).u32(), list);
        assert_eq!(e.call(0x004d_ce30, &[]).u32(), list);
        assert_eq!(calls_to(&e, ADAPTER_LIST_CREATE).len(), 1);
        assert_eq!(e.call(0x004d_cdb0, &args![1u32]).u32(), 0x101);
        assert_eq!(e.call(0x004d_cdb0, &args![2u32]).u32(), 0);
        assert_eq!(e.call(0x004d_cdf0, &args![list, 5u32]).u32(), 0);
        // No list at all.
        let mut e = mapped();
        e.register(ADAPTER_LIST_CREATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x004d_cdb0, &args![0u32]).u32(), 0);
    }

    #[test]
    fn display_modes_and_windowed_support() {
        let mut e = mapped();
        let adapter = e.mem.alloc(0x470);
        let info = e.mem.alloc(0x150);
        e.mem.set_u32(info + 4, 1);
        e.mem.set_u8(info + 0x140, 1);
        e.mem.set_u32(adapter + 0x460, info);
        e.register(ARRAY_ELEMENT, |e, a| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, a[0] + a[1]);
            slot.into_ret()
        });
        assert_eq!(
            e.call(0x004d_cd00, &args![adapter, 3u32]).u32(),
            adapter + 0x453
        );
        assert_eq!(e.call(0x004d_cd30, &args![adapter, 1u32]).u8(), 1);
        // The reference device information has no handle.
        let reference = e.mem.alloc(0x150);
        e.mem.set_u32(adapter + 0x464, reference);
        assert_eq!(e.call(0x004d_cd30, &args![adapter, 0u32]).u8(), 0);
    }

    #[test]
    fn light_color_goes_to_the_renderer_object() {
        let mut e = mapped();
        let color = e.mem.alloc(12);
        for (i, value) in [0.25f32, 0.5, 0.75].iter().enumerate() {
            e.mem.set_f32(color + 4 * i as u32, *value);
        }
        e.register(RENDERER_GETTER, |_, _| 0u32.into_ret());
        assert!(!e.call(0x004d_ce50, &args![color]).bool());
        assert_eq!(e.mem.f32(LIGHT_COLOR + 8), 0.75);
        let object = e.mem.alloc(4);
        e.put_vtable(0x0090_0000, &[0; 0x2c]);
        e.mem.set_u32(0x0090_00ac, 0x0091_0000);
        e.mem.set_u32(object, 0x0090_0000);
        e.register_double(RENDERER_GETTER, move |_, _| object.into_ret());
        e.register(FLOAT4_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(0x0091_0000, |e, a| {
            let values = e.mem.bytes(a[1], 16);
            e.mem.write(0x011c_7f00, &values);
            Ret::default()
        });
        assert!(e.call(0x004d_ce50, &args![color]).bool());
        assert_eq!(e.mem.f32(0x011c_7f00), 0.25);
        assert_eq!(e.mem.f32(0x011c_7f08), 0.75);
        assert_eq!(e.mem.u32(0x011c_7f0c), 0);
    }

    #[test]
    fn powf_goes_through_doubles() {
        let mut e = Engine::new();
        e.register(POW, |_, a| {
            let base = f64::take(a, &mut 0);
            let exponent = f64::take(&a[2..], &mut 0);
            base.powf(exponent).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_d150, &args![2.0f32, 10.0f32]).f32(), 1024.0);
        assert_eq!(calls_to(&e, POW), vec![args![2.0f64, 10.0f64]]);
    }

    #[test]
    fn multisample_check_asks_direct3d() {
        let mut e = mapped();
        let renderer = e.mem.alloc(0x600);
        e.mem.set_u32(renderer + 0x5c8, 1);
        e.set_global(RENDERER_POINTER, renderer);
        e.register(SMART_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WINDOWED_FROM_SETTING, |_, _| 1u32.into_ret());
        e.register(SETTING_INT_VALUE, |_, _| 2u32.into_ret());
        let direct3d = e.mem.alloc(4);
        e.put_vtable(0x0090_0000, &[0; 0x0c]);
        e.mem.set_u32(0x0090_002c, 0x0091_0000);
        e.mem.set_u32(direct3d, 0x0090_0000);
        e.set_global(DIRECT3D_OBJECT, direct3d);
        e.register(0x0091_0000, |_, a| {
            u32::from(a[5] == 8).wrapping_sub(1).into_ret()
        });
        e.set_global(RENDERER_ARG_01189470, 3u32);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004d_d180, &args![4u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, 0x0091_0000),
            vec![vec![direct3d, 2, 1, 0x16, 1, 4, 0]]
        );
        assert!(e.call(0x004d_d180, &args![8u32, 0x71u32]).bool());
        e.set_global(RENDERER_POINTER, 0u32);
        assert!(!e.call(0x004d_d180, &args![8u32, 0x71u32]).bool());
    }

    /// The display-format index whose format (`0x16`) the modes use.
    const FORMAT_INDEX: u32 = 3;

    /// A rig for `004dc710`: screen 1600x1200, one adapter whose display
    /// modes are `modes` (width, height, bits, format), the requested size
    /// `request`, a hardware device information with hardware T&L and
    /// shader versions 0x200 / 0x300 (both needed).
    fn mode_rig(
        modes: &[(u32, u32, u32, u32)],
        full_screen: u8,
        request: (u32, u32),
    ) -> (Engine, u32) {
        let mut e = mapped();
        e.mem.set_f64(ASPECT_TOLERANCE, f64::from(0.01f32));
        e.mem.set_f64(ASPECT_FIVE_BY_FOUR, 1.25);
        e.mem
            .set_f64(ASPECT_FOUR_BY_THREE, f64::from(1.333_333_4f32));
        e.register(IMPORT_GET_SYSTEM_METRICS, |_, a| {
            if a[0] == SM_CXSCREEN { 1600u32 } else { 1200 }.into_ret()
        });
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 1);
        e.set_global(ADAPTER_LIST_POINTER, list);
        let adapter = e.mem.alloc(0x470);
        let info = e.mem.alloc(0x150);
        e.mem.set_u32(info + 4, 1);
        e.mem.set_u8(info + 0x140, 1);
        e.mem.set_u32(adapter + 0x460, info);
        let caps = e.mem.alloc(0x100);
        e.mem.set_u32(caps + 0x1c, 0x0001_0000);
        e.mem.set_u32(caps + 0xc4, 0xfffe_0200);
        e.mem.set_u32(caps + 0xcc, 0x0000_0300);
        let list_slot = e.mem.alloc(4);
        e.mem.set_u32(list_slot, adapter);
        let mode_slots = e.mem.alloc(4 * modes.len() as u32);
        for (i, (width, height, bits, format)) in modes.iter().enumerate() {
            let mode = e.mem.alloc(0x10);
            e.mem.set_u32(mode, *width);
            e.mem.set_u32(mode + 4, *height);
            e.mem.set_u32(mode + 8, *bits);
            e.mem.set_u32(mode + 0xc, *format);
            e.mem.set_u32(mode_slots + 4 * i as u32, mode);
        }
        let count = modes.len() as u32;
        e.register_double(ARRAY_COUNT, move |_, a| {
            if a[0] == adapter + 0x450 { count } else { 1 }.into_ret()
        });
        e.register_double(ARRAY_ELEMENT, move |_, a| {
            if a[0] == adapter + 0x450 {
                mode_slots + 4 * a[1]
            } else {
                list_slot
            }
            .into_ret()
        });
        e.register(SMART_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SETTING_INT_VALUE, |_, a| {
            u32::from(a[0] == I_N_PATCHES).wrapping_mul(2).into_ret()
        });
        e.register_double(FULL_SCREEN_MODE, move |_, _| full_screen.into_ret());
        e.register(FLOAT_ABS, |_, a| {
            f64::from(f32::from_bits(a[0]).abs()).into_ret()
        });
        e.register_double(ADAPTER_SECOND_NAME, move |_, _| caps.into_ret());
        e.register(SETTING_BOOL_VALUE, |_, _| 1u32.into_ret());
        e.register(STRING_COPY, |_, _| Ret::default());
        e.set_global(RENDERER_ARG_01189470, FORMAT_INDEX);
        e.set_global(SHADER_MANAGER_WORD_B, 0x200u32);
        e.set_global(SHADER_MANAGER_WORD_A, 0x300u32);
        e.set_global(RENDER_WIDTH, request.0);
        e.set_global(RENDER_HEIGHT, request.1);
        (e, caps)
    }

    const MODES: [(u32, u32, u32, u32); 3] = [
        (800, 600, 32, 0x16),
        (1024, 768, 32, 0x16),
        (1280, 1024, 32, 0x16),
    ];

    #[test]
    fn display_mode_takes_the_requested_size() {
        let (mut e, _) = mode_rig(&MODES, 1, (1024, 768));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_c710, &args![1u32]).u8(), 1);
        assert!(calls_to(&e, STRING_COPY).is_empty());
        assert_eq!(e.global::<u32>(SHADER_MANAGER_WORD_B), 0x200);
        assert_eq!(e.global::<u32>(SHADER_MANAGER_WORD_A), 0x300);
        assert_eq!(e.global::<u8>(OFFSCREEN_ENABLED), 0);
        assert_eq!(e.global::<u8>(RENDER_TARGET_ADJUST_FLAG), 0);
    }

    #[test]
    fn display_mode_falls_back_to_a_taller_mode() {
        let (mut e, _) = mode_rig(&MODES, 1, (1024, 700));
        assert_eq!(e.call(0x004d_c710, &args![1u32]).u8(), 1);
        assert_eq!(e.global::<u32>(RENDER_WIDTH), 1024);
        assert_eq!(e.global::<u32>(RENDER_HEIGHT), 768);
        assert_eq!(e.global::<u8>(OFFSCREEN_ENABLED), 1);
    }

    #[test]
    fn display_mode_without_a_usable_size_fails() {
        let (mut e, _) = mode_rig(&MODES, 1, (1000, 700));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_c710, &args![1u32]).u8(), 0);
        assert_eq!(
            calls_to(&e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_RESOLUTION]]
        );
    }

    #[test]
    fn windowed_takes_the_last_full_colour_mode() {
        let modes = [
            (800, 600, 32, 0x16),
            (1024, 768, 16, 0x16),
            (640, 480, 32, 0x15),
        ];
        let (mut e, caps) = mode_rig(&modes, 0, (1, 1));
        // No hardware T&L: an error and false; the patch flag needs both.
        e.mem.set_u32(caps + 0x1c, 0x0100_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_c710, &args![1u32]).u8(), 0);
        assert_eq!(
            calls_to(&e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_NO_HARDWARE_TL]]
        );
        assert_eq!(e.global::<u8>(RENDER_TARGET_ADJUST_FLAG), 1);
        // The list entry, then all three modes from the last.
        assert_eq!(calls_to(&e, ARRAY_ELEMENT).len(), 1 + 3);
    }

    #[test]
    fn old_shaders_are_refused_and_shaders_off_configures_the_manager() {
        let (mut e, caps) = mode_rig(&MODES, 1, (1024, 768));
        e.mem.set_u32(caps + 0xcc, 0x0000_0200);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_c710, &args![1u32]).u8(), 0);
        assert_eq!(
            calls_to(&e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_SHADER_VERSIONS]]
        );
        let (mut e, _) = mode_rig(&MODES, 1, (1024, 768));
        e.register(SETTING_BOOL_VALUE, |_, _| 0u32.into_ret());
        let force = e.mem.alloc(1);
        e.mem.set_u8(force, 1);
        e.register_double(SETTING_BOOL_POINTER, move |_, _| force.into_ret());
        e.register(SHADER_MANAGER_CONFIGURE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_c710, &args![1u32]).u8(), 1);
        assert_eq!(
            calls_to(&e, SHADER_MANAGER_CONFIGURE),
            vec![vec![0, 0, 1, 0, 0, 0]]
        );
        assert_eq!(e.global::<u32>(SHADER_MANAGER_WORD_B), 0);
    }

    #[test]
    fn display_mode_needs_an_adapter() {
        let mut e = mapped();
        e.register(IMPORT_GET_SYSTEM_METRICS, |_, _| 800u32.into_ret());
        e.register(ADAPTER_LIST_CREATE, |_, _| 0u32.into_ret());
        e.register(STRING_COPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_c710, &args![1u32]).u8(), 0);
        assert_eq!(
            calls_to(&e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_NO_DEVICE]]
        );
        // A list without the adapter.
        let (mut e, _) = mode_rig(&MODES, 1, (1024, 768));
        e.register(SETTING_INT_VALUE, |_, _| 5u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_c710, &args![1u32]).u8(), 0);
        assert_eq!(
            calls_to(&e, STRING_COPY),
            vec![vec![ERROR_MESSAGE, MESSAGE_BAD_ADAPTER]]
        );
    }
}

#[cfg(test)]
mod b000s_tests {
    use super::*;

    const MODULE: u32 = 0x7000_0000;
    /// Fake entry points the library double hands out, one per name.
    const FAKE_CREATE: u32 = 0x7100_0000;
    const FAKE_DESTROY: u32 = 0x7100_0010;
    const FAKE_ADAPTER_COUNT: u32 = 0x7100_0020;
    const FAKE_ADAPTER_INFO: u32 = 0x7100_0030;
    const FAKE_SLS_INDEX: u32 = 0x7100_0040;
    const FAKE_SLS_CONFIG: u32 = 0x7100_0050;
    const FAKE_MODES: u32 = 0x7100_0060;
    const FAKE_DISPLAY_MAP: u32 = 0x7100_0070;
    const DISPLAY_INDEX: u32 = 2;
    const ADAPTER: u32 = 7;
    const BEZEL_ID: u32 = 0x33;

    #[derive(Clone, Copy)]
    struct Scenario {
        bezel: bool,
        failing_display: u32,
        display_map_fails: bool,
        rotated: bool,
    }

    const NATIVE: Scenario = Scenario {
        bezel: false,
        failing_display: 0,
        display_map_fails: false,
        rotated: false,
    };

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn words(e: &Engine, at: u32, n: u32) -> Vec<u32> {
        (0..n).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    fn put(e: &mut Engine, at: u32, values: &[(u32, u32)]) {
        for (offset, value) in values {
            e.mem.set_u32(at + offset, *value);
        }
    }

    /// The mode the displays report: (width, height, orientation).
    fn mode_of(s: Scenario) -> (u32, u32, u32) {
        match (s.bezel, s.rotated) {
            (true, _) => (6000, 1100, 0),
            (false, true) => (1080, 5760, 90),
            (false, false) => (5760, 1080, 0),
        }
    }

    /// A three-display, one-row Eyefinity group behind the ADL doubles.
    fn rig(s: Scenario) -> Engine {
        let mut e = Engine::new();
        e.register(MEMSET, |_, _| Ret::default());
        e.register(FILL_BYTES, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(ADL_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ADL_FREE_SLOT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(IMPORT_LOAD_LIBRARY, |_, a| {
            if a[0] == ADL_LIBRARY {
                MODULE.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(IMPORT_GET_PROC_ADDRESS, |_, a| {
            match a[1] {
                ADL_MAIN_CONTROL_CREATE => FAKE_CREATE,
                ADL_MAIN_CONTROL_DESTROY => FAKE_DESTROY,
                ADL_ADAPTER_NUMBER_OF_ADAPTERS_GET => FAKE_ADAPTER_COUNT,
                ADL_ADAPTER_ADAPTER_INFO_GET => FAKE_ADAPTER_INFO,
                ADL_DISPLAY_SLS_MAP_INDEX_GET => FAKE_SLS_INDEX,
                ADL_DISPLAY_SLS_MAP_CONFIG_GET => FAKE_SLS_CONFIG,
                ADL_DISPLAY_MODES_GET => FAKE_MODES,
                ADL_DISPLAY_DISPLAY_MAP_CONFIG_GET => FAKE_DISPLAY_MAP,
                _ => 0,
            }
            .into_ret()
        });
        e.register(IMPORT_FREE_LIBRARY, |_, _| Ret::default());
        e.register(FAKE_CREATE, |_, _| 0u32.into_ret());
        e.register(FAKE_DESTROY, |_, _| 0u32.into_ret());
        e.register(FAKE_ADAPTER_COUNT, |e, a| {
            e.mem.set_u32(a[0], 1);
            0u32.into_ret()
        });
        e.register(FAKE_ADAPTER_INFO, |e, a| {
            assert_eq!(a[1], ADAPTER_RECORD_SIZE);
            e.mem.set_u32(a[0] + 4, ADAPTER);
            e.mem.set_u32(a[0] + 0x620, DISPLAY_INDEX);
            0u32.into_ret()
        });
        e.register_double(FAKE_DISPLAY_MAP, move |e, a| {
            assert_eq!((a[0], a[5]), (ADAPTER, 1));
            if s.display_map_fails {
                return 1u32.into_ret();
            }
            let targets = e.mem.alloc(3 * 0x1c);
            for t in 0..3 {
                e.mem.set_u32(targets + t * 0x1c, 10 + t);
            }
            e.mem.set_u32(a[3], 3);
            e.mem.set_u32(a[4], targets);
            0u32.into_ret()
        });
        e.register(FAKE_SLS_INDEX, |e, a| {
            assert_eq!((a[0], a[1]), (ADAPTER, 3));
            e.mem.set_u32(a[3], 5);
            0u32.into_ret()
        });
        e.register_double(FAKE_SLS_CONFIG, move |e, a| {
            assert_eq!((a[0], a[1], a[13]), (ADAPTER, 5, 2));
            put(e, a[2], &[(0x10, 1), (0x14, 3)]);
            let sls = e.mem.alloc(3 * 0x70);
            for t in 0..3 {
                put(e, sls + t * 0x70, &[(8, 10 + t), (0x24, t), (0x28, 0)]);
            }
            e.mem.set_u32(a[3], 3);
            e.mem.set_u32(a[4], sls);
            let (width, height, _) = mode_of(s);
            let native = e.mem.alloc(0x50);
            if s.bezel {
                put(e, native, &[(0x28, 1), (0x2c, 1)]);
            } else {
                put(e, native, &[(0x28, width), (0x2c, height)]);
            }
            e.mem.set_u32(a[5], 1);
            e.mem.set_u32(a[6], native);
            let bezel = e.mem.alloc(0x58);
            put(e, bezel, &[(8, BEZEL_ID), (0x28, width), (0x2c, height)]);
            e.mem.set_u32(a[7], u32::from(s.bezel));
            e.mem.set_u32(a[8], bezel);
            let offsets = e.mem.alloc(3 * 0x34);
            for t in 0..3 {
                let at = offsets + t * 0x34;
                let x = 2000 * t + 5;
                put(
                    e,
                    at,
                    &[(8, 10 + t), (0x18, BEZEL_ID), (0x1c, x), (0x20, 6)],
                );
                put(e, at, &[(0x24, 1990), (0x28, 1090)]);
            }
            e.mem.set_u32(a[11], 3);
            e.mem.set_u32(a[12], offsets);
            0u32.into_ret()
        });
        e.register_double(FAKE_MODES, move |e, a| {
            assert_eq!(a[0], ADAPTER);
            if a[1] == s.failing_display {
                return 1u32.into_ret();
            }
            let (width, height, orientation) = mode_of(s);
            let mode = e.mem.alloc(0x30);
            put(
                e,
                mode,
                &[(0x1c, width), (0x20, height), (0x2c, orientation)],
            );
            e.mem.set_u32(a[3], mode);
            0u32.into_ret()
        });
        e
    }

    /// Runs the query; answers (result, description, count, entries).
    fn query(e: &mut Engine) -> (i32, Vec<u32>, u32, u32) {
        let info = e.mem.alloc(0x18);
        let count = e.mem.alloc(4);
        let entries = e.mem.alloc(4);
        let result = e
            .call(0x004d_d2c0, &args![DISPLAY_INDEX, info, count, entries])
            .i32();
        (
            result,
            words(e, info, 6),
            e.mem.u32(count),
            e.mem.u32(entries),
        )
    }

    #[test]
    fn fn_004dd2a0_releases_the_slot_and_answers_1() {
        let mut e = Engine::new();
        e.register(ADL_FREE_SLOT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_d2a0, &args![0x1234u32]).u32(), 1);
        assert_eq!(calls(&e, ADL_FREE_SLOT), vec![vec![0x1234]]);
    }

    #[test]
    fn fn_004dd2c0_answers_0_without_outputs_or_library() {
        let mut e = rig(NATIVE);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x004d_d2c0, &args![2u32, 0x100u32, 0u32, 0x200u32])
            .i32();
        assert_eq!(result, 0);
        assert!(calls(&e, IMPORT_LOAD_LIBRARY).is_empty());
        assert_eq!(calls(&e, MEMSET).len(), 1);
        e.register(IMPORT_LOAD_LIBRARY, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(query(&mut e).0, 0);
        assert_eq!(
            calls(&e, IMPORT_LOAD_LIBRARY),
            vec![vec![ADL_LIBRARY], vec![ADL_LIBRARY_FALLBACK]]
        );
        assert!(calls(&e, IMPORT_FREE_LIBRARY).is_empty());
    }

    #[test]
    fn fn_004dd2c0_fills_the_native_grid() {
        let mut e = rig(NATIVE);
        e.call_log = Some(vec![]);
        let (result, info, count, entries) = query(&mut e);
        assert_eq!(result, 1);
        assert_eq!(info, vec![1, 3, 1, 5760, 1080, 0]);
        assert_eq!(count, 3);
        for t in 0..3 {
            let flag = u32::from(t == 1);
            let x = 1920 * t;
            assert_eq!(
                words(&e, entries + t * DISPLAY_ENTRY_SIZE, 11),
                vec![t, 0, x, 0, 1920, 1080, x, 0, 1920, 1080, flag]
            );
        }
        assert_eq!(calls(&e, FAKE_CREATE), vec![vec![ADL_ALLOCATE, 1]]);
        // One release per target before its modes, then the nine blocks.
        assert_eq!(calls(&e, ADL_FREE_SLOT).len(), 12);
        assert_eq!(calls(&e, FAKE_DESTROY).len(), 1);
        assert_eq!(calls(&e, IMPORT_FREE_LIBRARY), vec![vec![MODULE]]);
    }

    #[test]
    fn fn_004dd2c0_swaps_a_rotated_mode() {
        let mut e = rig(Scenario {
            rotated: true,
            ..NATIVE
        });
        let (result, info, _, entries) = query(&mut e);
        assert_eq!(result, 1);
        assert_eq!(info, vec![1, 3, 1, 5760, 1080, 0]);
        assert_eq!(
            words(&e, entries + DISPLAY_ENTRY_SIZE, 6),
            vec![1, 0, 1920, 0, 1920, 1080]
        );
    }

    #[test]
    fn fn_004dd2c0_derives_a_failed_bezel_target_from_the_first_good_one() {
        let mut e = rig(Scenario {
            bezel: true,
            failing_display: 12,
            ..NATIVE
        });
        let (result, info, count, entries) = query(&mut e);
        assert_eq!(result, 1);
        assert_eq!(info, vec![1, 3, 1, 6000, 1100, 1]);
        assert_eq!(count, 3);
        assert_eq!(
            words(&e, entries, 11),
            vec![0, 0, 0, 0, 2000, 1100, 5, 6, 1990, 1090, 0]
        );
        assert_eq!(
            words(&e, entries + DISPLAY_ENTRY_SIZE, 11),
            vec![1, 0, 2000, 0, 2000, 1100, 2005, 6, 1990, 1090, 1]
        );
        // The last column: its visible x is the width less the reference's
        // visible width; row 0 gives 0.
        assert_eq!(
            words(&e, entries + 2 * DISPLAY_ENTRY_SIZE, 11),
            vec![2, 0, 4000, 0, 2000, 1100, 4010, 0, 1990, 1090, 0]
        );
    }

    #[test]
    fn fn_004dd2c0_failure_clears_the_outputs() {
        let mut e = rig(Scenario {
            display_map_fails: true,
            ..NATIVE
        });
        e.call_log = Some(vec![]);
        let info = e.mem.alloc(0x18);
        e.mem.set_u32(info, 0xffff_ffff);
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 9);
        let entries = e.mem.alloc(4);
        let result = e
            .call(0x004d_d2c0, &args![DISPLAY_INDEX, info, count, entries])
            .i32();
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(count), 0);
        assert_eq!(e.mem.u32(info), 0);
        assert_eq!(calls(&e, FILL_BYTES).last(), Some(&vec![info, 0, 4]));
        assert_eq!(calls(&e, ADL_FREE_SLOT).last(), Some(&vec![entries]));
        assert_eq!(calls(&e, IMPORT_FREE_LIBRARY), vec![vec![MODULE]]);
    }

    fn map_globals(e: &mut Engine) {
        e.map(0x0118_9480, 0x20);
        e.map(0x011c_70e0, 0x20);
        e.map(THREE_01021928, 8);
        e.set_global(THREE_01021928, 3.0f64);
    }

    /// `EnumDisplayDevicesA` double: devices 0 to 2, only device 2 with
    /// state bit 2.
    fn enumerate_three(e: &mut Engine) {
        e.register(IMPORT_ENUM_DISPLAY_DEVICES, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x1a8);
            if a[1] > 2 {
                return 0u32.into_ret();
            }
            e.mem.set_u32(a[2] + 0xa4, if a[1] == 2 { 5 } else { 0 });
            1u32.into_ret()
        });
    }

    #[test]
    fn fn_004de100_reports_a_failed_query() {
        let mut e = rig(NATIVE);
        map_globals(&mut e);
        enumerate_three(&mut e);
        e.register(IMPORT_LOAD_LIBRARY, |_, _| 0u32.into_ret());
        e.register(PRINTF, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004d_e100, &args![]);
        assert_eq!(calls(&e, IMPORT_ENUM_DISPLAY_DEVICES).len(), 3);
        assert_eq!(calls(&e, PRINTF), vec![vec![EYEFINITY_QUERY_FAILED, 2]]);
        // No device with the bit: the index stays -1.
        e.register(IMPORT_ENUM_DISPLAY_DEVICES, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004d_e100, &args![]);
        assert_eq!(
            calls(&e, PRINTF),
            vec![vec![EYEFINITY_QUERY_FAILED, u32::MAX]]
        );
    }

    #[test]
    fn fn_004de100_sets_the_eyefinity_sizes() {
        let mut e = rig(NATIVE);
        map_globals(&mut e);
        enumerate_three(&mut e);
        e.set_global(EYEFINITY_GATE_01189484, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x004d_e100, &args![]);
        let float = |e: &Engine, at: u32| f32::from_bits(e.global::<u32>(at));
        assert_eq!(float(&e, EYEFINITY_TOTAL_WIDTH_0118948C), 5760.0);
        assert_eq!(float(&e, EYEFINITY_TOTAL_HEIGHT_01189490), 1080.0);
        assert_eq!(
            float(&e, EYEFINITY_RATIO_01189488),
            (5760.0f64 / 1080.0) as f32
        );
        assert_eq!(float(&e, EYEFINITY_VIEW_WIDTH_01189494), 1920.0);
        assert_eq!(float(&e, EYEFINITY_VIEW_HEIGHT_01189498), 1080.0);
        assert_eq!(float(&e, EYEFINITY_THIRD_WIDTH_011C70F0), 1920.0);
        assert_eq!(e.global::<u8>(EYEFINITY_FLAG), 1);
        assert_eq!(e.global::<u8>(EYEFINITY_SECOND_FLAG), 1);
        assert_eq!(e.global::<u8>(EYEFINITY_TALL_FLAG_011C70EF), 0);
        // The entry array is released through its slot.
        assert_eq!(calls(&e, ADL_FREE_SLOT).len(), 13);
        assert!(calls(&e, PRINTF).is_empty());
    }

    #[test]
    fn fn_004de100_without_the_gate_only_releases() {
        let mut e = rig(NATIVE);
        map_globals(&mut e);
        enumerate_three(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x004d_e100, &args![]);
        assert_eq!(e.global::<u32>(EYEFINITY_TOTAL_WIDTH_0118948C), 0);
        assert_eq!(e.global::<u8>(EYEFINITY_FLAG), 0);
        assert_eq!(calls(&e, ADL_FREE_SLOT).len(), 13);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_004de0e0_returns_the_view_width() {
        let mut e = Engine::new();
        e.map(0x0118_9000, 0x1000);
        e.set_global(EYEFINITY_VIEW_WIDTH_01189494, 1920.0f32);
        assert_eq!(e.call(0x004d_e0e0, &args![]).f32(), 1920.0);
        // A signalling NaN comes back quiet.
        e.set_global(EYEFINITY_VIEW_WIDTH_01189494, 0x7f80_0001u32);
        assert_eq!(e.call(0x004d_e0e0, &args![]).f32().to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn fn_004dcef0_builds_three_ramps_and_hands_them_to_the_device() {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0101_e000, 0x0102_1000, 0x0118_9000] {
            e.map(page, 0x1000);
        }
        e.map(0x011c_6000, 0x2000);
        e.set_global(RAMP_STEPS_0101E568, 255.0f64);
        e.set_global(RAMP_SCALE_010217D0, 65535.0f64);
        e.set_global(HALF_01011588, 0.5f64);
        e.set_global(GAMMA, 1.0f32);
        e.set_global(GAMMA_CHANGED, 1u8);
        e.register(GAMMA_SETTING_SET_004ED780, |_, _| Ret::default());
        // `powf` double: base to the exponent.
        e.register(POWF, |_, a| {
            let base = f32::from_bits(a[0]);
            let exponent = f32::from_bits(a[1]);
            base.powf(exponent).into_ret()
        });
        e.register(SMART_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        let renderer = e.mem.alloc(0x290);
        e.set_global(RENDERER_POINTER, renderer);
        let device = e.mem.alloc(4);
        e.mem.set_u32(renderer + RENDERER_OBJECT_OFFSET, device);
        let vtable = e.mem.alloc(0x58);
        e.mem.set_u32(device, vtable);
        e.mem.set_u32(vtable + 0x54, 0x0090_0054);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(0x0090_0054, move |e, a| {
            // The ramp's red, green and blue entries 0, 1, 128 and 255.
            let ramp = a[3];
            let mut words = vec![a[0], a[1], a[2]];
            for colour in 0..3u32 {
                for index in [0u32, 1, 128, 255] {
                    words.push(u32::from(e.mem.u16(ramp + colour * 0x200 + 2 * index)));
                }
            }
            *record.borrow_mut() = words;
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x004d_cef0, &args![]);
        assert_eq!(e.global::<u8>(GAMMA_CHANGED), 0);
        let log = e.call_log.take().unwrap();
        let setting: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == GAMMA_SETTING_SET_004ED780)
            .collect();
        assert_eq!(setting.len(), 1);
        assert_eq!(setting[0].1, vec![F_GAMMA, 1.0f32.to_bits()]);
        // Each entry is computed once per colour.
        assert_eq!(log.iter().filter(|(a, _)| *a == POWF).count(), 3 * 256);
        let seen = seen.borrow();
        let word = |i: u32| seen[i as usize];
        assert_eq!([word(0), word(1), word(2)], [device, 0, 1]);
        for colour in 0..3u32 {
            let at = 3 + 4 * colour;
            assert_eq!(
                [word(at), word(at + 1), word(at + 2), word(at + 3)],
                [0, 257, 32896, 65535]
            );
        }
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn window_position_is_kept_only_windowed() {
        for full_screen in [false, true] {
            let mut e = Engine::new();
            e.register_double(FULL_SCREEN_MODE, move |_, _| {
                u32::from(full_screen).into_ret()
            });
            e.register(SETTING_INT_SET, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x004d_c6e0, &args![10u32, 20u32]);
            if full_screen {
                assert!(calls(&e, SETTING_INT_SET).is_empty());
            } else {
                assert_eq!(
                    calls(&e, SETTING_INT_SET),
                    vec![vec![I_LOCATION_X, 10], vec![I_LOCATION_Y, 20]]
                );
            }
        }
    }

    #[test]
    fn allocation_callbacks_allocate_and_free() {
        let mut e = Engine::new();
        e.register(ALLOCATE_00401000, |_, a| (a[0] + 0x1000).into_ret());
        e.register(FREE_00401030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_d240, &args![0x624u32]).u32(), 0x1624);
        let slot = e.mem.alloc(4);
        e.call(0x004d_d270, &args![slot]);
        assert!(calls(&e, FREE_00401030).is_empty());
        e.mem.set_u32(slot, 0x5000);
        e.call(0x004d_d270, &args![slot]);
        assert_eq!(calls(&e, FREE_00401030), vec![vec![0x5000]]);
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn eyefinity_floats_come_back_quiet() {
        let mut e = Engine::new();
        e.map(0x0118_9000, 0x1000);
        e.set_global(EYEFINITY_TOTAL_WIDTH_0118948C, 5760.0f32);
        e.set_global(EYEFINITY_TOTAL_HEIGHT_01189490, 1080.0f32);
        e.set_global(EYEFINITY_VIEW_HEIGHT_01189498, 0x7f80_0001u32);
        assert_eq!(e.call(0x004d_e0c0, &args![]).f32(), 5760.0);
        assert_eq!(e.call(0x004d_e0d0, &args![]).f32(), 1080.0);
        assert_eq!(e.call(0x004d_e0f0, &args![]).f32().to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn bool_setting_store_and_set() {
        let mut e = Engine::new();
        let setting = e.mem.alloc(8);
        assert_eq!(
            e.call(0x004d_e2d0, &args![setting, 0x101u32]).u32(),
            setting
        );
        assert_eq!(e.mem.u8(setting + 4), 1);
        assert_eq!(e.call(0x004d_e2b0, &args![setting, 0u32]).u32(), setting);
        assert_eq!(e.mem.u8(setting + 4), 0);
        // A null setting is left alone.
        assert_eq!(e.call(0x004d_e2b0, &args![0u32, 1u32]).u32(), 0);
    }

    #[test]
    fn pref_setting_scalar_deleting_destructor() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(PREF_SETTING_DESTRUCTOR_004DE420, |_, _| Ret::default());
            e.register(FREE_00401030, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x004d_e3f0, &args![0x7000u32, flags]).u32(), 0x7000);
            assert_eq!(
                calls(&e, PREF_SETTING_DESTRUCTOR_004DE420),
                vec![vec![0x7000]]
            );
            assert_eq!(calls(&e, FREE_00401030).len(), flags as usize);
        }
    }
}
