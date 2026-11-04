//! `fallout shared/sky/sky.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Sky` (the singleton at `pInstance`) owns the sky's scene-graph roots, the
//! sky components (atmosphere, stars, sun, clouds, the two moons, the
//! precipitation), the current climate and weather and the colours and fog
//! values the weather blend produces each frame. The file also holds the
//! `SkySound` helpers and a few weather and clouds accessors the linker
//! placed between the `Sky` methods; each says whose method it is.
//!
//! Layouts and helpers are at the top; the functions follow in address
//! order. The last session translated `0063d060` to `0063ef20`, the end of the unit.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` at every
//! `float` store (docs/ENGINE_CRATE.md); the comparisons the game makes with
//! `FCOMPP` keep their NaN behaviour (a comparison with NaN is false).

#[allow(unused_imports)]
use crate::prelude::*;

// ---- callees outside this file ----

/// `NiPointer<T>::get` (`__thiscall`, no arguments): the pointer stored at
/// `this`.
pub(crate) const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<T>::operator=(T*)` (`RET 4`): releases the old object, stores
/// the new one and adds a reference to it.
pub(crate) const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>::operator!=(T*)` (`RET 4`): `*this != pointer`.
const NI_POINTER_NOT_EQUAL: u32 = 0x0052_aa80;
/// `NiPointer<T>::~NiPointer`: releases the held object.
const NI_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// `BSSimpleList` node accessors (`__thiscall`): the address of the node's
/// item (`this`) and the next node (`this + 4`); `IsEmpty` tests both words
/// of the head node, `RemoveHead` unlinks and frees the first node, `Clear`
/// empties the list and `0047 02f0` is its scalar deleting destructor.
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_SCALAR_DELETING_DESTRUCTOR: u32 = 0x0047_02f0;
/// `operator new` (`cdecl`, size), `operator delete` (`cdecl`, block) and the
/// `NiObject` allocation (`cdecl`, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const NI_OBJECT_NEW: u32 = 0x00aa_13e0;
/// `memset` (`cdecl`: destination, value, size).
const MEMSET: u32 = 0x0040_3d30;
/// Scope timer constructor (`ECX` = timer, `RET 0x10`: kind, 1, file, line)
/// and destructor.
const TIMER_SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const TIMER_SCOPE_DESTROY: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Sky\Sky.cpp"`.
const SKY_SOURCE_FILE_NAME: u32 = 0x0104_edf0;
/// `Sky::vftable`.
const SKY_VTABLE: u32 = 0x0104_edec;

/// `BSSoundHandle` constructor (`{ -1, 0, 0 }`), copy assignment (`RET 4`)
/// and (empty) destructor; `IsPlaying`, `Stop` and `Release`.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const SOUND_HANDLE_IS_PLAYING: u32 = 0x00ad_8930;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
/// The body of `SkySound`'s destructor (a `BSSoundHandle` destructor).
const SKY_SOUND_BASE_DESTRUCT: u32 = 0x0052_d240;

/// Scene-graph node visibility setter (`__thiscall`, `RET 4`, one byte; the
/// underwater code passes 1 to hide the sky nodes).
const SET_NODE_HIDDEN: u32 = 0x0045_0f90;
/// The scene-graph node of a `Sun` (`this + 0xc`, `__thiscall`).
const SUN_GET_NODE: u32 = 0x0043_b230;
/// The number of layers of a `Clouds` (`__thiscall`).
const CLOUDS_LAYER_COUNT: u32 = 0x0063_4830;

/// `Sky::UnloadAllTextures` and the function that runs when mode 2 or 3 is entered (both `__thiscall`, no arguments); `Sky::UpdateHDRValues`.
const SKY_UNLOAD_ALL_TEXTURES: u32 = 0x0063_e210;
const SKY_ENTER_VISIBLE_MODE: u32 = 0x0063_e2f0;
const SKY_UPDATE_HDR_VALUES: u32 = 0x0063_ef20;
/// Precipitation: update (`__thiscall`, `float`), constructor (0x18 bytes)
/// and `Initialize`.
const PRECIPITATION_UPDATE: u32 = 0x0063_6ed0;
const PRECIPITATION_CONSTRUCT: u32 = 0x0063_6a40;
const PRECIPITATION_INITIALIZE: u32 = 0x0063_6bb0;
/// Constructors of `Atmosphere` (0x1c bytes), `Stars` (0x10), `Sun` (0x2c)
/// and `Clouds` (0x5c).
const ATMOSPHERE_CONSTRUCT: u32 = 0x0063_3080;
const STARS_CONSTRUCT: u32 = 0x0063_fc70;
const SUN_CONSTRUCT: u32 = 0x0064_04f0;
const CLOUDS_CONSTRUCT: u32 = 0x0063_3cc0;
/// The moons' root `NiNode` constructor (0xac bytes, one argument) and the
/// `BSMultiBoundNode` constructor (0xb4 bytes).
const MOONS_ROOT_CONSTRUCT: u32 = 0x00a5_ecb0;
const MULTIBOUND_NODE_CONSTRUCT: u32 = 0x00c4_6970;
/// Calls on the sky's root node in `Initialize` (all `__thiscall`): the two
/// that take one flag word (`1`), the one taking the root's name argument,
/// `AttachProperty`, `UpdateProperties`, and the object whose virtual slot
/// `0xe8` takes the root away from its scene graph.
const ROOT_SET_FIRST_FLAG: u32 = 0x0054_68d0;
const ROOT_SET_SECOND_FLAG: u32 = 0x0049_02f0;
const ROOT_SET_NAME: u32 = 0x0044_0460;
const ROOT_ATTACH_PROPERTY: u32 = 0x0043_9410;
const ROOT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const ROOT_GET_SCENE: u32 = 0x0096_11e0;
/// The string `Initialize` passes to [`ROOT_SET_NAME`].
const ROOT_NAME_ARGUMENT: u32 = 0x011f_426c;
/// The sky's property object: constructor (0x1c bytes, `NiObject`
/// allocation) and four setters called with `6`, `7`, `1` and `1`.
const SKY_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const SKY_PROPERTY_SET_A: u32 = 0x0043_9340;
const SKY_PROPERTY_SET_B: u32 = 0x0043_9390;
const SKY_PROPERTY_SET_C: u32 = 0x0049_ed90;
const SKY_PROPERTY_SET_D: u32 = 0x0063_6000;
/// `NiPointer` constructor from a raw pointer (`00633c90`, translated in
/// `run_00633c90.rs`).
const NI_POINTER_FROM_RAW: u32 = 0x0063_3c90;
/// A setting object (`ECX` = [`SETTING_PRECIPITATION`]) whose accessor
/// returns a pointer to its byte value.
const SETTING_PRECIPITATION: u32 = 0x011c_cb8c;
const SETTING_BYTE_VALUE: u32 = 0x0040_8d60;
/// The accessor of a setting object holding a `float` (`ECX` = the setting,
/// returns a pointer to the value).
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// A global byte; when set `Initialize` stores `1.0` in the two floats.
const INITIAL_FLAG: u32 = 0x011f_941e;
const INITIAL_FLOAT_A: u32 = 0x011a_d87c;
const INITIAL_FLOAT_B: u32 = 0x011a_d880;
/// `SetMode` clears this byte when it enters mode 0 or 1 from 2, 3 or 4.
const SET_MODE_FLAG: u32 = 0x011c_cb74;

// ---- callees and data of the colour, fog and update code ----

/// `_ftol2_sse` (`00ec62c0`): truncates the float in `ST0`, passed here as an
/// `f64` argument (two words).
const FTOL: u32 = 0x00ec_62c0;
/// The player (a pointer global) and its `parentCell` accessor (`__thiscall`,
/// returns the word at `+0x40`).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
const PLAYER_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectREFR::GetRelevantWaterHeight` (`ECX` = the player, `float` in
/// `ST0`) and the cell's water height (`ECX` = cell, `float` in `ST0`).
const RELEVANT_WATER_HEIGHT: u32 = 0x0057_b0a0;
const CELL_WATER_HEIGHT: u32 = 0x0054_71e0;
/// The `NiPointer` global holding the camera and its helpers: the camera
/// (no arguments), the camera's scene node, and a node's world position
/// (`node + 0x8c`).
const CAMERA_HOLDER_GET: u32 = 0x0045_c670;
const CAMERA_NODE: u32 = 0x0055_8310;
const NODE_WORLD_POSITION: u32 = 0x0045_bb80;
/// The calendar (an object at a fixed address): `Calendar::GetHour` (`float`
/// in `ST0`) and the day count accessor (`ECX` = calendar, returns a word).
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
const CALENDAR_GET_DAYS: u32 = 0x0086_7de0;
/// A global pointer to an object whose accessor `0042ce10` returns bit 1 of
/// the word at `+0x244`.
const WORLD_STATE_POINTER: u32 = 0x011d_df38;
const WORLD_STATE_FLAG: u32 = 0x0042_ce10;
/// Steps `Sky::Update` calls on other units' code: `0063d1d0`, `0063d5e0`
/// (`__thiscall`, no arguments), the moon-phase test `006368f0` (`cdecl`,
/// the sky) and `004dce50` (`cdecl`, the colour at `+0x48`).
const SKY_UPDATE_TRANSITION: u32 = 0x0063_d1d0;
const SKY_UPDATE_UNKNOWN: u32 = 0x0063_d5e0;
const MOON_PHASE_APPLIES: u32 = 0x0063_68f0;
const FOG_COLOR_UPDATE: u32 = 0x004d_ce50;
/// `Sky::ResetWeather`.
const SKY_RESET_WEATHER: u32 = 0x0063_d060;
/// The tick counter object (`ECX`) and its reader (returns the word at
/// `+0x14`); the lightning flash duration setting.
const TICK_COUNTER: u32 = 0x011f_6394;
const TICK_COUNT_GET: u32 = 0x0082_5c00;
const FLASH_DURATION_SETTING: u32 = 0x011c_cc34;
/// Doubles in the exe's data: `0.0`, `1.0`, `0.5`, `1000.0`.
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_ONE: u32 = 0x0101_2070;
const DOUBLE_HALF: u32 = 0x0101_1588;
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;

/// `Sky` methods other units define: the sunrise begin and end and the
/// sunset begin and end hours of the current climate (`__thiscall`, `float`
/// in `ST0`). The sky caches the first and last in the globals below.
const SKY_SUNRISE_BEGIN: u32 = 0x0059_5ea0;
const SKY_SUNRISE_END: u32 = 0x0059_5f50;
const SKY_SUNSET_BEGIN: u32 = 0x0059_5fc0;
const SKY_SUNSET_END: u32 = 0x0059_6030;
const SUNRISE_BEGIN_CACHE: u32 = 0x011c_ccfc;
const SUNSET_END_CACHE: u32 = 0x011c_cd00;
/// The time-transition setting.
const TIME_SETTING: u32 = 0x011c_cbd8;
/// `max(a, b)` and `min(a, b)` of two `float`s (`cdecl`; the second argument
/// is the result when the comparison is false or unordered).
const MAX_FLOAT: u32 = 0x0040_4010;
const MIN_FLOAT: u32 = 0x0040_ebd0;
/// `clamp(&value, low, high)` (`cdecl`) and the colour clamp to at most 1
/// (`ECX` = colour).
const CLAMP_FLOAT: u32 = 0x0053_30e0;
const COLOR_CLAMP_TO_ONE: u32 = 0x00a6_9690;
/// `NiColor::NiColor(r, g, b)` (`__thiscall`, returns `this`).
const NI_COLOR_CONSTRUCT: u32 = 0x0041_6870;
/// A weather's byte table fraction (`ECX` = weather; index, high, low):
/// `table[index] / 255 * (high - low) + low`.
const WEATHER_BYTE_FRACTION: u32 = 0x004e_d230;
/// The log call for invalid climate data (`cdecl`, one string) and its
/// message.
const LOG_MASTERFILE_ERROR: u32 = 0x005b_5e40;
const TRANSITION_TIMES_MESSAGE: u32 = 0x0104_ee38;
/// The interior-cell holder global, the accessor `005f36f0` that returns its
/// interior cell (the engine map names it `ActorMover::GetPreferredMoveMode`
/// because the linker folded identical code) and the interior cell's
/// lighting colour and fog getters.
const INTERIOR_CELL_HOLDER: u32 = 0x011d_ea10;
const GET_INTERIOR_CELL: u32 = 0x005f_36f0;
const CELL_COLOR_FIRST: u32 = 0x0054_4890;
const CELL_COLOR_SECOND: u32 = 0x0054_47b0;
const CELL_COLOR_THIRD: u32 = 0x0054_4a30;
const CELL_FOG_FAR: u32 = 0x0054_4b10;
const CELL_FOG_NEAR: u32 = 0x0054_4ab0;
const CELL_FOG_POWER: u32 = 0x0054_4b70;
/// The fog default (a `float`), the interior far-distance limit (a double),
/// the factor for the near distance (a double) and the camera-relative fog
/// setting.
const DEFAULT_FOG_DISTANCE: u32 = 0x0103_17c4;
const FOG_FAR_LIMIT: u32 = 0x0103_a480;
const FOG_NEAR_FACTOR: u32 = 0x0104_ee90;
const FOG_SETTING: u32 = 0x0120_3144;
/// The water volume's fog near and far distances (`ECX` = volume), its two
/// colours, and the setting that scales the depth.
const WATER_FOG_NEAR: u32 = 0x004e_3d00;
const WATER_FOG_FAR: u32 = 0x0050_7b20;
const WATER_COLOR_A: u32 = 0x004f_9bf0;
const WATER_COLOR_B: u32 = 0x009e_e040;
const WATER_DEPTH_SCALE: u32 = 0x011c_cc60;
/// The two global flag bytes the fog code tests (getters without arguments).
const FLAG_UNDERWATER_A: u32 = 0x004e_2180;
const FLAG_UNDERWATER_B: u32 = 0x004e_3c40;
/// Lightning scale settings for sky colours 3 and 4.
const LIGHTNING_SCALE_COLOR_3: u32 = 0x011c_cbc0;
const LIGHTNING_SCALE_COLOR_4: u32 = 0x011c_cccc;
/// `Sky` accessors (`__thiscall`): the clouds (`+0x2c`), the atmosphere
/// (`+0x20`) and the sun (`+0x28`); and the `NiPointer` at `+8` of a
/// component.
const SKY_GET_CLOUDS: u32 = 0x0055_b980;
const SKY_GET_ATMOSPHERE: u32 = 0x007a_f430;
const SKY_GET_SUN: u32 = 0x0045_cd60;
const HOLDER_NODE: u32 = 0x0055_85e0;
/// `TESForm` lookup by id (`cdecl`), `__RTDynamicCast` (`cdecl`: object,
/// vfdelta, source type, target type, reference flag) and the two type
/// descriptors `Sky::SetCurrentClimate` casts between.
const LOOKUP_FORM: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_DESCRIPTOR_FORM: u32 = 0x0118_3028;
const TYPE_DESCRIPTOR_CLIMATE: u32 = 0x0118_4170;
/// `Stars::LoadGeometry` (Xbox PDB).
const STARS_LOAD_GEOMETRY: u32 = 0x0063_fdb0;
/// Sun texture loading: the node that holds the glare property (`+0x10` of
/// the sun's holder, `__thiscall`), `NiAVObject::GetProperty(type)`
/// (`RET 4`), the climate texture entry's validity test and name, the
/// `BSStringT` constructor, destructor, assignment from a `char*` and append,
/// a case-insensitive comparison (`cdecl`), the texture loader (`cdecl`:
/// path, 1, holder, 1, 0), the property's texture setter and the node flag
/// setter (`ECX` = node, one byte).
const SUN_FIRST_NODE: u32 = 0x006f_a820;
const NODE_GET_PROPERTY: u32 = 0x00a5_9d30;
const TEXTURE_ENTRY_VALID: u32 = 0x0048_cee0;
const TEXTURE_ENTRY_NAME: u32 = 0x0040_8da0;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const STRING_ASSIGN: u32 = 0x0043_8390;
const STRING_APPEND: u32 = 0x0040_4820;
const STRING_COMPARE: u32 = 0x0040_4dc0;
const TEXTURE_LOAD: u32 = 0x00b5_5840;
const PROPERTY_SET_TEXTURE: u32 = 0x0063_48e0;
const NODE_SET_FLAG_BIT_20: u32 = 0x0063_5fe0;
/// `"Sky\SunGlare.dds"` and `"Textures\Sky\SunGlareNonHDR.dds"`.
const SUN_GLARE_NAME: u32 = 0x0102_d7c4;
const SUN_GLARE_NON_HDR_PATH: u32 = 0x0104_eeb8;
/// `NiUpdateData(time, flag, flag)` constructor (`RET 0xc`) and the node call
/// that takes it (`RET 4`).
const NI_UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const NODE_UPDATE_CONTROLLERS: u32 = 0x00a5_9c60;
/// A setting object holding a word (`ECX` = setting, returns a pointer to
/// the value) and the `Moon` constructor (0x7c bytes; `RET 0x1c`: name,
/// five floats, one word).
const SETTING_WORD_VALUE: u32 = 0x0043_d4d0;
const MOON_CONSTRUCT: u32 = 0x0063_4a70;

// ---- callees and data of the weather transition, sound, texture, save and
// image-space code (`0063d060` to `0063ef20`) ----

/// `Clouds::ClearTransTextures` (Xbox PDB), `Clouds::RemoveTextures`
/// (`006349e0`), `Precipitation::FlushAll` (Xbox PDB), the unload of a `Moon`
/// (`006369a0`) and of the `Sun` (`00642940`); all `__thiscall`, no arguments.
const CLOUDS_CLEAR_TRANS_TEXTURES: u32 = 0x0063_4930;
const CLOUDS_REMOVE_TEXTURES: u32 = 0x0063_49e0;
const PRECIPITATION_FLUSH_ALL: u32 = 0x0063_7800;
const MOON_UNLOAD_TEXTURES: u32 = 0x0063_69a0;
const SUN_UNLOAD_TEXTURES: u32 = 0x0064_2940;
/// A `PlayerCharacter` method (`ECX` = the player, `RET 4`) that
/// `ForceWeather` calls with `0`.
const PLAYER_METHOD_0093A7A0: u32 = 0x0093_a7a0;
/// The weather list of a climate: `ECX` = climate + `0x30`; returns the
/// weather the climate picks (`0063d5c0` forwards to it).
const CLIMATE_PICK_WEATHER: u32 = 0x0058_27d0;
/// The climate's byte at `+0x54` (`ECX` = climate), whose complement
/// scales the time the sky waits before it picks a new weather.
const CLIMATE_UPDATE_BYTE: u32 = 0x0045_1cd0;
/// The double `1/255` (`0.003921569`).
const RECIPROCAL_255: u32 = 0x0102_31e8;
/// The `float` `0.99902...` and the `float` `0.001` that `Sky` passes as the
/// high and low ends to [`WEATHER_BYTE_FRACTION`].
const FRACTION_HIGH_SOUND: u32 = 0x0102_31e0;
const FRACTION_LOW: u32 = 0x0101_7d00;
/// The form id of the default weather list entry that `0063d1d0` looks up
/// (`004839c0`, `cdecl`) when the climate has none.
const DEFAULT_WEATHER_FORM_ID: u32 = 0x15e;
/// The type descriptor of `TESWeather`.
const TYPE_DESCRIPTOR_WEATHER: u32 = 0x0118_629c;
/// The global holding the object of the region list, and its accessor
/// (`ECX` = the global's value) that returns the list's owner; the regions are
/// the list at `+4`. `TESRegion::UpdateWeather` (Xbox PDB) is called on each.
const REGION_HOLDER: u32 = 0x011c_3f2c;
const REGION_LIST_OWNER: u32 = 0x0041_69d0;
const REGION_UPDATE_WEATHER: u32 = 0x004f_1050;
/// Two accessors that `0063d1d0` chains: of the player (`ECX` = the player
/// global's value) an object, and of that object a weather (zero when it has
/// none; the engine map names `0059bb30` `D3DTexture_LockRect` because of
/// folded code).
const PLAYER_WEATHER_OWNER: u32 = 0x0054_54d0;
const WEATHER_OF_OWNER: u32 = 0x0059_bb30;
/// Float settings read by the weather transition: the low and high ends of
/// the transition fraction and the acceleration setting.
const TRANSITION_LOW_SETTING: u32 = 0x011c_cc08;
const TRANSITION_HIGH_SETTING: u32 = 0x011c_cc88;
const TRANSITION_ACCELERATION_SETTING: u32 = 0x011c_cc28;
/// `Sky` byte at `011ccd04`: set when a sound was marked and the sound list
/// must be rebuilt.
const SOUNDS_DIRTY: u32 = 0x011c_cd04;
/// The byte at `011ccb74` counts the sounds of type 3 (see `SetMode`).
const SOUND_TYPE_3_COUNT: u32 = SET_MODE_FLAG;
/// `BSSimpleList` constructor (`ECX`, no arguments, returns `this`),
/// destructor, and `AddHead`-style insertion (`RET 4`, a pointer to the
/// item).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
const LIST_INSERT: u32 = 0x005a_e3d0;
/// A random number (`cdecl`, no arguments).
const RANDOM_NUMBER: u32 = 0x0048_7f50;
/// `BSSoundHandle` methods: `IsValid`, `Play(flag)` (`RET 4`), `GetVolume`
/// (returns `ST0`) and `SetVolume(volume)` (`RET 4`).
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_GET_VOLUME: u32 = 0x00ad_8a20;
const SOUND_HANDLE_SET_VOLUME: u32 = 0x00ad_89e0;
/// `BSAudio::QInstance` (Xbox PDB; returns the audio object) and
/// `BSAudio::GetSoundHandleByNumericID` (Xbox PDB; `ECX` = audio, `RET 0xc`:
/// the handle to fill, the id, the flags; returns the handle).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE: u32 = 0x00ad_73b0;
/// `Interface::IsInMenuMode` and `Interface::IsInGameLoadingMenuOpen` (Xbox
/// PDB; `cdecl`, no arguments).
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
const IS_LOADING_MENU_OPEN: u32 = 0x0070_5ea0;
/// `fabs` of a `float` (`cdecl`, result in `ST0`) and the double `0.01`
/// (`0.009999999776482582`).
const ABS_FLOAT: u32 = 0x0040_8840;
const VOLUME_EPSILON: u32 = 0x0101_6408;
/// `Clouds::...(layer)` accessor of the weather: the cloud texture entry of a
/// layer (`RET 4`, null when none), and the text format `"%s%s"`, the
/// formatter (`cdecl`: text, format, arguments), and the existence test of a
/// file (`cdecl`: path, 0, 0, -1).
const WEATHER_CLOUD_ENTRY: u32 = 0x0063_47a0;
const FORMAT_STRING: u32 = 0x0040_6f60;
const PATH_FORMAT: u32 = 0x0101_996c;
const FILE_EXISTS: u32 = 0x0045_6a20;
/// `"Textures\Sky\MoonShadow.dds"`.
const MOON_SHADOW_PATH: u32 = 0x0104_ed3c;
/// The global holding the save-game buffer object and its methods
/// (`ECX` = buffer): the version byte (`008df040`), `SaveNumericID` (`RET 8`:
/// pointer, size) and the raw data save (`008579b0`, `RET 8`: pointer, size).
const SAVE_BUFFER_GLOBAL: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const SAVE_DATA: u32 = 0x0085_79b0;
/// A form's id (`ECX` = form, returns a word).
const FORM_ID_OF: u32 = 0x0084_e3a0;
/// The second save format: `SaveFormID` (`RET 8`: form, 0) and data save
/// (`RET 0xc`: pointer, size, 0).
const SAVE_FORM_ID_OV2: u32 = 0x0086_5df0;
const SAVE_DATA_OV2: u32 = 0x0086_5e50;
/// The load buffer's methods: the next form id (no arguments) and the data
/// load (`RET 8`: pointer, size).
const LOAD_FORM_ID: u32 = 0x0086_48a0;
const LOAD_DATA: u32 = 0x0086_4980;
/// The image-space modifier instance: the object's size, its constructor
/// (`ECX` = block), the registration of a holder with the manager (the
/// accessor `0043b5d0`, `ECX` = the global `011dea10`'s value, returns the
/// manager; `00631540`, `RET 4`: the holder), the flag setter (`RET 4`), the
/// weight setter (`RET 4`, `float`) and the form setter (`RET 4`).
const IMAGE_SPACE_SIZE: u32 = 0x30;
const IMAGE_SPACE_CONSTRUCT: u32 = 0x0052_9490;
const IMAGE_SPACE_MANAGER: u32 = 0x0043_b5d0;
const IMAGE_SPACE_ADD: u32 = 0x0063_1540;
const IMAGE_SPACE_SET_FLAG: u32 = 0x005b_bd60;
const IMAGE_SPACE_SET_WEIGHT: u32 = 0x0063_f790;
const IMAGE_SPACE_SET_FORM: u32 = 0x0050_f9a0;
/// The weather's image-space form for a time of day (`ECX` = weather + `0x18`,
/// `RET 4`: the index) and the default form (no arguments).
const WEATHER_IMAGE_SPACE_FOR_TIME: u32 = 0x0058_22a0;
const DEFAULT_IMAGE_SPACE: u32 = 0x0053_2ff0;
/// `ECX` = the sky: whether flag 1 or 2 of `uiFlags` is set (the sound code
/// uses it as "the weather sounds apply"; `00634870`, a function of another unit).
const SOUNDS_APPLY: u32 = 0x0063_4870;

layout! {
    /// `Sky` (Xbox PDB), 0x138 bytes; the offsets used here match the PC
    /// code. `Sky::pInstance` is the singleton.
    pub struct Sky: 0x138 {
        /// `spRoot` (Xbox PDB): `NiPointer<BSMultiBoundNode>`.
        0x04 spRoot: u32,
        /// `spMoonsRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x08 spMoonsRoot: u32,
        /// `pCurrentClimate` (Xbox PDB): `TESClimate*`.
        0x0C pCurrentClimate: Ptr,
        /// `pCurrentWeather` (Xbox PDB): `TESWeather*`.
        0x10 pCurrentWeather: Ptr,
        /// `pLastWeather` (Xbox PDB): `TESWeather*`.
        0x14 pLastWeather: Ptr,
        /// `pDefaultWeather` (Xbox PDB): `TESWeather*`.
        0x18 pDefaultWeather: Ptr,
        /// `pOverrideWeather` (Xbox PDB): `TESWeather*`.
        0x1C pOverrideWeather: Ptr,
        /// `pAtmosphere` (Xbox PDB).
        0x20 pAtmosphere: Ptr,
        /// `pStars` (Xbox PDB).
        0x24 pStars: Ptr,
        /// `pSun` (Xbox PDB).
        0x28 pSun: Ptr,
        /// `pClouds` (Xbox PDB).
        0x2C pClouds: Ptr,
        /// `pMasser` (Xbox PDB): a `Moon`.
        0x30 pMasser: Ptr,
        /// `pSecunda` (Xbox PDB): a `Moon`.
        0x34 pSecunda: Ptr,
        /// `pPrecip` (Xbox PDB): the `Precipitation`.
        0x38 pPrecip: Ptr,
        /// `SkyColor` (Xbox PDB): ten `NiColor`s of 12 bytes from here
        /// ([`SKY_COLOR_STRIDE`]).
        0x3C SkyColor: u32,
        /// `WaterFogColor` (Xbox PDB).
        0xB4 WaterFogColor: u32,
        /// `SunSpecularColor` (Xbox PDB).
        0xC0 SunSpecularColor: u32,
        /// `fWindSpeed` (Xbox PDB).
        0xCC fWindSpeed: f32,
        /// `fFogDistances` (Xbox PDB): the near distance.
        0xD4 fFogNear: f32,
        /// `fFogDistances` (Xbox PDB): the far distance.
        0xD8 fFogFar: f32,
        /// `fFogPower` (Xbox PDB).
        0xE8 fFogPower: f32,
        /// `fCurrentGameHour` (Xbox PDB).
        0xEC fCurrentGameHour: f32,
        /// `fLastWeatherUpdate` (Xbox PDB).
        0xF0 fLastWeatherUpdate: f32,
        /// `fCurrentWeatherPct` (Xbox PDB).
        0xF4 fCurrentWeatherPct: f32,
        /// `eMode` (Xbox PDB): `Sky::SKY_MODE`.
        0xF8 eMode: u32,
        /// `pSkySoundList` (Xbox PDB): `BSSimpleList<SkySound*>*`.
        0xFC pSkySoundList: Ptr,
        /// `fFlash` (Xbox PDB).
        0x100 fFlash: f32,
        /// `uiFlashTime` (Xbox PDB).
        0x104 uiFlashTime: u32,
        /// `uiLastMoonPhaseUpdate` (Xbox PDB).
        0x108 uiLastMoonPhaseUpdate: u32,
        /// `fAccelBeginPct` (Xbox PDB).
        0x110 fAccelBeginPct: f32,
        /// `uiFlags` (Xbox PDB).
        0x118 uiFlags: u32,
        /// `pCurrentWeatherImageSpaceMod` (Xbox PDB): the instance for the
        /// current weather's first image-space modifier.
        0x11C pCurrentWeatherImageSpaceMod: Ptr,
        /// `pCurrentWeatherImageSpaceMod2` (Xbox PDB).
        0x120 pCurrentWeatherImageSpaceMod2: Ptr,
        /// `pLastWeatherImageSpaceMod` (Xbox PDB).
        0x124 pLastWeatherImageSpaceMod: Ptr,
        /// `pLastWeatherImageSpaceMod2` (Xbox PDB).
        0x128 pLastWeatherImageSpaceMod2: Ptr,
        /// `fHighNoon` (Xbox PDB).
        0x12C fHighNoon: f32,
    }

    /// `SkySound` (Xbox PDB), 0x1c bytes: a `BSSoundHandle` (id, flag byte,
    /// state word) and the weather sound's description.
    pub struct SkySound: 0x1C {
        /// `SndHandle` (Xbox PDB): the sound id (`-1` = none).
        0x00 SndHandle: u32,
        /// `pWeather` (Xbox PDB).
        0x0C pWeather: Ptr,
        /// `eSoundType` (Xbox PDB).
        0x10 eSoundType: u32,
        /// `uiFormID` (Xbox PDB).
        0x14 uiFormID: u32,
        /// `uiData` (Xbox PDB).
        0x18 uiData: u32,
    }
}

/// Size of one `NiColor` (three `float`s) in `Sky::SkyColor`.
pub(crate) const SKY_COLOR_STRIDE: u32 = 12;

/// Copies one `NiColor` (three words) from `from` to `to`.
fn copy_color(e: &mut Engine, to: u32, from: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// `NiPointer<T>::get` on the smart pointer stored at `slot`.
fn ni_pointer_get(e: &mut Engine, slot: u32) -> Ptr {
    e.call(NI_POINTER_GET, &args![slot]).ptr()
}

/// `NiPointer<T>::operator=` on the smart pointer stored at `slot`.
fn ni_pointer_assign(e: &mut Engine, slot: u32, value: Ptr) {
    e.call(NI_POINTER_ASSIGN, &args![slot, value]);
}

/// `delete component`: calls the object's scalar deleting destructor
/// (virtual slot 0, flag 1) when it is not null.
fn delete_component(e: &mut Engine, component: Ptr) {
    if !component.is_null() {
        e.vcall(component.addr(), 0, &args![1u32]);
    }
}

/// Runs `body` between the scope timer's constructor and destructor (`line`
/// is the source line the game passes).
fn with_timer<R>(e: &mut Engine, line: u32, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(8, |e, guard| {
        e.call(
            TIMER_SCOPE_CONSTRUCT,
            &args![guard, 0x21u32, 1u32, SKY_SOURCE_FILE_NAME, line],
        );
        let result = body(e);
        e.call(TIMER_SCOPE_DESTROY, &args![guard]);
        result
    })
}

/// `new` of a class with a constructor: allocates `size` bytes with
/// `allocator`, runs `construct` on the block unless the allocation failed
/// and returns the object (null when the block is null).
fn construct_with(
    e: &mut Engine,
    allocator: u32,
    size: u32,
    construct: u32,
    extra_arguments: &[u32],
) -> Ptr {
    let block: Ptr = e.call(allocator, &args![size]).ptr();
    if block.is_null() {
        return Ptr::NULL;
    }
    let mut words = vec![block.addr()];
    words.extend_from_slice(extra_arguments);
    e.call(construct, &words).ptr()
}

// ---- functions ----

// Translated from 00639bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkySound::SkySound` (constructor by its body; the engine map has no
/// name): makes `SndHandle` a fresh `BSSoundHandle`, copies in the handle
/// passed by value (three words), stores the weather, the sound type and the
/// form id and clears `uiData`. The compiler's exception frame is not
/// translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00639bf0(
    e: &mut Engine,
    this: Ptr<SkySound>,
    handle_id: u32,
    handle_flag: u32,
    handle_state: u32,
    weather: Ptr,
    sound_type: u32,
    form_id: u32,
) -> Ptr<SkySound> {
    e.call(SOUND_HANDLE_CONSTRUCT, &args![this]);
    e.with_stack(12, |e, copy| {
        e.mem.set_u32(copy.addr(), handle_id);
        e.mem.set_u32(copy.addr() + 4, handle_flag);
        e.mem.set_u32(copy.addr() + 8, handle_state);
        e.call(SOUND_HANDLE_ASSIGN, &args![this, copy]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![copy]);
    });
    e.set(this, SkySound::pWeather, weather);
    e.set(this, SkySound::eSoundType, sound_type);
    e.set(this, SkySound::uiFormID, form_id);
    let data = this.addr() + 0x18;
    e.call(MEMSET, &args![data, 0u32, 4u32]);
    this
}

// Translated from 00639c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed method of another class (its owner is not identified): when the
/// object's `00457fe0` value exceeds the limit `0042f5a0(0x80000000,
/// 00639ce0())`, calls `00639d00` with that value plus `extra` and returns
/// true; otherwise returns false.
pub fn fn_00639c90(e: &mut Engine, this: Ptr, extra: u32) -> bool {
    // 00457fe0 takes no `this` (00639c99 leaves ECX as is).
    let size = e.call(0x0045_7fe0, &args![]).u32();
    let limit_source = e.call(0x0063_9ce0, &args![this]).u32();
    let limit = e
        .call(0x0042_f5a0, &args![0x8000_0000u32, limit_source])
        .u32();
    if limit < size {
        e.call(0x0063_9d00, &args![this, size.wrapping_add(extra)]);
        true
    } else {
        false
    }
}

// Translated from 0063a080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor and,
/// when bit 0 of `flags` is set, frees the object. Returns `this`.
pub fn sky_scalar_deleting_destructor(e: &mut Engine, this: Ptr<Sky>, flags: u32) -> Ptr<Sky> {
    fn_0063a0b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0063a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::~Sky` (by its body; the engine map names only the deleting
/// version): stores the vtable, deletes atmosphere, stars, clouds, sun, both
/// moons and the precipitation (virtual deleting destructors), releases the
/// moons root, takes the root out of its scene graph and releases it, deletes
/// every `SkySound` of the sound list, then the list. The compiler's
/// exception frame is not translated.
pub fn fn_0063a0b0(e: &mut Engine, this: Ptr<Sky>) {
    e.mem.set_u32(this.addr(), SKY_VTABLE);
    for field in [
        Sky::pAtmosphere,
        Sky::pStars,
        Sky::pClouds,
        Sky::pSun,
        Sky::pMasser,
        Sky::pSecunda,
        Sky::pPrecip,
    ] {
        let component = e.get(this, field);
        delete_component(e, component);
    }
    ni_pointer_assign(e, this.addr() + 8, Ptr::NULL);
    let root_slot = this.addr() + 4;
    if e.call(NI_POINTER_NOT_EQUAL, &args![root_slot, 0u32]).bool() {
        let root = ni_pointer_get(e, root_slot);
        if !e.call(ROOT_GET_SCENE, &args![root]).ptr::<()>().is_null() {
            let root = ni_pointer_get(e, root_slot);
            let scene: Ptr = e.call(ROOT_GET_SCENE, &args![root]).ptr();
            let root = ni_pointer_get(e, root_slot);
            e.vcall(scene.addr(), 0xe8, &args![root]);
        }
    }
    ni_pointer_assign(e, root_slot, Ptr::NULL);
    let mut node = e.get(this, Sky::pSkySoundList);
    while !node.is_null() {
        let item_slot: Ptr = e.call(LIST_NODE_ITEM, &args![node]).ptr();
        if e.mem.u32(item_slot.addr()) == 0 {
            break;
        }
        let item_slot: Ptr = e.call(LIST_NODE_ITEM, &args![node]).ptr();
        let item: Ptr<SkySound> = Ptr::new(e.mem.u32(item_slot.addr()));
        if !item.is_null() {
            fn_0063a3c0(e, item, 1);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
    }
    let list = e.get(this, Sky::pSkySoundList);
    e.call(LIST_CLEAR, &args![list]);
    let list = e.get(this, Sky::pSkySoundList);
    if !list.is_null() {
        e.call(LIST_SCALAR_DELETING_DESTRUCTOR, &args![list, 1u32]);
    }
    e.call(NI_POINTER_DESTRUCTOR, &args![this.addr() + 8]);
    e.call(NI_POINTER_DESTRUCTOR, &args![root_slot]);
}

// Translated from 0063a3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a `SkySound` (by its use in the sky's sound
/// list): runs the `BSSoundHandle` destructor body and, when bit 0 of
/// `flags` is set, frees the object. Returns `this`.
pub fn fn_0063a3c0(e: &mut Engine, this: Ptr<SkySound>, flags: u32) -> Ptr<SkySound> {
    e.call(SKY_SOUND_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0063a3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::SetMode` (Xbox PDB): changes the sky mode (`Sky::SKY_MODE`: 0
/// `SM_NONE`, 1 `SM_INTERIOR`, 2 `SM_FAKEEXTERIOR`, 3 `SM_FULL`, 4
/// `SM_COUNT`). Entering mode 2 or 3 from 0, 1 or 4 calls `0063e2f0`, flags
/// the clouds (`0063a610`) and clears the visibility flag of the root and
/// the sun node. Entering mode 0 or 1 from 2, 3 or 4 unloads the textures,
/// sets that flag on both nodes, stops, releases and deletes every sound of
/// the list, clears the byte at `011ccb74`, stores the mode, resets the
/// precipitation and the flash. Every path ends by storing the mode and
/// calling `Sky::UpdateHDRValues`.
pub fn sky_set_mode(e: &mut Engine, this: Ptr<Sky>, mode: u32) {
    let current = e.get(this, Sky::eMode);
    if matches!(current, 0 | 1 | 4) && matches!(mode, 2 | 3) {
        e.call(SKY_ENTER_VISIBLE_MODE, &args![this]);
        let clouds = e.get(this, Sky::pClouds);
        if !clouds.is_null() {
            fn_0063a610(e, clouds);
        }
        set_root_and_sun_hidden(e, this, 0);
    } else if matches!(current, 2..=4) && matches!(mode, 0 | 1) {
        e.call(SKY_UNLOAD_ALL_TEXTURES, &args![this]);
        set_root_and_sun_hidden(e, this, 1);
        let list = e.get(this, Sky::pSkySoundList);
        while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            let slot: Ptr = e.call(LIST_NODE_ITEM, &args![list]).ptr();
            let sound = e.mem.u32(slot.addr());
            if e.call(SOUND_HANDLE_IS_PLAYING, &args![sound]).bool() {
                let slot: Ptr = e.call(LIST_NODE_ITEM, &args![list]).ptr();
                let sound = e.mem.u32(slot.addr());
                e.call(SOUND_HANDLE_STOP, &args![sound]);
            }
            let slot: Ptr = e.call(LIST_NODE_ITEM, &args![list]).ptr();
            let sound = e.mem.u32(slot.addr());
            e.call(SOUND_HANDLE_RELEASE, &args![sound]);
            let slot: Ptr = e.call(LIST_NODE_ITEM, &args![list]).ptr();
            let sound: Ptr<SkySound> = Ptr::new(e.mem.u32(slot.addr()));
            if !sound.is_null() {
                fn_0063a3c0(e, sound, 1);
            }
            e.call(LIST_REMOVE_HEAD, &args![list]);
        }
        e.set_global(SET_MODE_FLAG, 0u8);
        e.set(this, Sky::eMode, mode);
        let precipitation = e.get(this, Sky::pPrecip);
        if !precipitation.is_null() {
            e.call(PRECIPITATION_UPDATE, &args![precipitation, 0.0f32]);
        }
        e.set(this, Sky::fFlash, 0.0);
    }
    e.set(this, Sky::eMode, mode);
    e.call(SKY_UPDATE_HDR_VALUES, &args![this]);
}

/// Calls the visibility setter with `hidden` (0 or 1) on the root node and the
/// sun's node, as `SetMode` does on both of its branches.
fn set_root_and_sun_hidden(e: &mut Engine, this: Ptr<Sky>, hidden: u32) {
    let root_slot = this.addr() + 4;
    if !ni_pointer_get(e, root_slot).is_null() {
        let root = ni_pointer_get(e, root_slot);
        e.call(SET_NODE_HIDDEN, &args![root, hidden]);
    }
    let sun = e.get(this, Sky::pSun);
    if !sun.is_null() {
        let node: Ptr = e.call(SUN_GET_NODE, &args![sun]).ptr();
        e.call(SET_NODE_HIDDEN, &args![node, hidden]);
    }
}

// Translated from 0063a610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds` method (by its caller `Sky::SetMode`): sets the byte at `+0x5a`.
pub fn fn_0063a610(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x5a, 1);
}

// Translated from 0063a630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::Initialize` (Xbox PDB): builds the sky. Takes the root node from
/// `root`, or makes a `BSMultiBoundNode` when `root` is null, names it, then
/// replaces the atmosphere (initialized with the root and `atmosphere_argument`),
/// the stars, the sun, the moons root (attached to the root), the
/// precipitation (only when the setting at `011ccb8c` is on), and the clouds,
/// and gives the root the sky's property object. The compiler's exception
/// frame is not translated.
pub fn sky_initialize(e: &mut Engine, this: Ptr<Sky>, root: Ptr, atmosphere_argument: u32) {
    with_timer(e, 0x152, |e| {
        let root_slot = this.addr() + 4;
        if e.call(NI_POINTER_NOT_EQUAL, &args![root_slot, 0u32]).bool() {
            let current = ni_pointer_get(e, root_slot);
            if !e
                .call(ROOT_GET_SCENE, &args![current])
                .ptr::<()>()
                .is_null()
            {
                let current = ni_pointer_get(e, root_slot);
                let scene: Ptr = e.call(ROOT_GET_SCENE, &args![current]).ptr();
                let current = ni_pointer_get(e, root_slot);
                e.vcall(scene.addr(), 0xe8, &args![current]);
            }
            ni_pointer_assign(e, root_slot, Ptr::NULL);
        }
        if !root.is_null() {
            ni_pointer_assign(e, root_slot, root);
        } else {
            let node = construct_with(e, NI_OBJECT_NEW, 0xb4, MULTIBOUND_NODE_CONSTRUCT, &[]);
            ni_pointer_assign(e, root_slot, node);
            let current = ni_pointer_get(e, root_slot);
            e.call(ROOT_SET_FIRST_FLAG, &args![current, 1u32]);
            let current = ni_pointer_get(e, root_slot);
            e.call(ROOT_SET_SECOND_FLAG, &args![current, 1u32]);
        }
        let current = ni_pointer_get(e, root_slot);
        e.call(ROOT_SET_NAME, &args![current, ROOT_NAME_ARGUMENT]);

        // Atmosphere: initialized with the root and the caller's argument.
        let old = e.get(this, Sky::pAtmosphere);
        delete_component(e, old);
        let atmosphere = construct_with(e, OPERATOR_NEW, 0x1c, ATMOSPHERE_CONSTRUCT, &[]);
        e.set(this, Sky::pAtmosphere, atmosphere);
        let current = ni_pointer_get(e, root_slot);
        e.vcall(
            atmosphere.addr(),
            0x10,
            &args![current, atmosphere_argument],
        );

        // Stars, sun: initialized with the root.
        let old = e.get(this, Sky::pStars);
        delete_component(e, old);
        let stars = construct_with(e, OPERATOR_NEW, 0x10, STARS_CONSTRUCT, &[]);
        e.set(this, Sky::pStars, stars);
        let current = ni_pointer_get(e, root_slot);
        e.vcall(stars.addr(), 8, &args![current]);

        let old = e.get(this, Sky::pSun);
        delete_component(e, old);
        let sun = construct_with(e, OPERATOR_NEW, 0x2c, SUN_CONSTRUCT, &[]);
        e.set(this, Sky::pSun, sun);
        let current = ni_pointer_get(e, root_slot);
        e.vcall(sun.addr(), 8, &args![current]);

        // The moons' root node, attached to the root (slot 0xdc).
        let moons = construct_with(e, NI_OBJECT_NEW, 0xac, MOONS_ROOT_CONSTRUCT, &[0]);
        ni_pointer_assign(e, this.addr() + 8, moons);
        let current = ni_pointer_get(e, root_slot);
        let moons = ni_pointer_get(e, this.addr() + 8);
        e.vcall(current.addr(), 0xdc, &args![moons, 1u32]);

        // Precipitation exists only when the setting is on.
        let old = e.get(this, Sky::pPrecip);
        delete_component(e, old);
        let setting: Ptr = e
            .call(SETTING_BYTE_VALUE, &args![SETTING_PRECIPITATION])
            .ptr();
        if e.mem.u8(setting.addr()) != 0 {
            let precipitation = construct_with(e, OPERATOR_NEW, 0x18, PRECIPITATION_CONSTRUCT, &[]);
            e.set(this, Sky::pPrecip, precipitation);
            e.call(PRECIPITATION_INITIALIZE, &args![precipitation]);
        } else {
            e.set(this, Sky::pPrecip, Ptr::NULL);
        }

        let old = e.get(this, Sky::pClouds);
        delete_component(e, old);
        let clouds = construct_with(e, OPERATOR_NEW, 0x5c, CLOUDS_CONSTRUCT, &[]);
        e.set(this, Sky::pClouds, clouds);
        let current = ni_pointer_get(e, root_slot);
        e.vcall(clouds.addr(), 8, &args![current]);

        // The property object, held in a `NiPointer` on the stack, is
        // configured and attached to the root.
        e.with_stack(4, |e, holder| {
            let property = construct_with(e, NI_OBJECT_NEW, 0x1c, SKY_PROPERTY_CONSTRUCT, &[]);
            e.call(NI_POINTER_FROM_RAW, &args![holder, property]);
            for (setter, value) in [
                (SKY_PROPERTY_SET_A, 6u32),
                (SKY_PROPERTY_SET_B, 7),
                (SKY_PROPERTY_SET_C, 1),
                (SKY_PROPERTY_SET_D, 1),
            ] {
                let held = ni_pointer_get(e, holder.addr());
                e.call(setter, &args![held, value]);
            }
            let held = ni_pointer_get(e, holder.addr());
            let current = ni_pointer_get(e, root_slot);
            e.call(ROOT_ATTACH_PROPERTY, &args![current, held]);
            ni_pointer_assign(e, holder.addr(), Ptr::NULL);
            e.call(NI_POINTER_DESTRUCTOR, &args![holder]);
        });

        if e.global::<u8>(INITIAL_FLAG) != 0 {
            e.set_global(INITIAL_FLOAT_A, 1.0f32);
            e.set_global(INITIAL_FLOAT_B, 1.0f32);
        }
        let current = ni_pointer_get(e, root_slot);
        e.call(ROOT_UPDATE_PROPERTIES, &args![current]);
    });
}

// Translated from 0063ac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::Update` (Xbox PDB): the per-frame update. Does nothing when the
/// player has no parent cell. Otherwise reads the hour from the calendar,
/// sets flag 4 when the camera is below the water, and in mode 1 without the
/// world flag runs the short update `0063b0a0`. Else it updates the weather
/// (`0063ca20`, `0063d1d0`), the colours (`0063b120`), the fog (`0063bce0`),
/// the wind (`0063c490`) and `0063d5e0`, updates atmosphere, stars, clouds
/// and sun, the moon phases when the day changed or the world flag asks for
/// it, both moons and the precipitation, fades the lightning flash, calls
/// `004dce50` on the colour at `+0x48` and clears flags 1 and 2.
pub fn sky_update(e: &mut Engine, this: Ptr<Sky>, delta: f32) {
    with_timer(e, 0x1d8, |e| {
        let player: u32 = e.global(PLAYER_CHARACTER);
        if e.call(PLAYER_PARENT_CELL, &args![player]).u32() == 0 {
            return;
        }
        let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f32();
        e.set(this, Sky::fCurrentGameHour, hour);

        // Flag 4: the camera is below the water of the player's cell.
        let player: u32 = e.global(PLAYER_CHARACTER);
        let mut flags = e.get(this, Sky::uiFlags);
        let mut underwater = false;
        if player != 0 && e.call(PLAYER_PARENT_CELL, &args![player]).u32() != 0 {
            let camera: Ptr = e.call(CAMERA_HOLDER_GET, &[]).ptr();
            let node: Ptr = e.call(CAMERA_NODE, &args![camera]).ptr();
            let position: Ptr = e.call(NODE_WORLD_POSITION, &args![node]).ptr();
            let water = e.call(RELEVANT_WATER_HEIGHT, &args![player]).f64();
            let camera_height = e.mem.f32(position.addr() + 8) as f64;
            underwater = camera_height < water;
        }
        flags = if underwater { flags | 4 } else { flags & !4 };
        e.set(this, Sky::uiFlags, flags);

        let world_flag: u32 = e.global(WORLD_STATE_POINTER);
        if e.get(this, Sky::eMode) == 1 && !e.call(WORLD_STATE_FLAG, &args![world_flag]).bool() {
            fn_0063b0a0(e, this, delta);
            return;
        }
        fn_0063ca20(e, this);
        let world_flag: u32 = e.global(WORLD_STATE_POINTER);
        if e.call(WORLD_STATE_FLAG, &args![world_flag]).bool() {
            let flags = e.get(this, Sky::uiFlags);
            e.set(this, Sky::uiFlags, flags | 1);
        }
        e.call(SKY_UPDATE_TRANSITION, &args![this]);
        fn_0063b120(e, this);
        fn_0063bce0(e, this);
        fn_0063c490(e, this);
        e.call(SKY_UPDATE_UNKNOWN, &args![this]);
        for field in [Sky::pAtmosphere, Sky::pStars, Sky::pClouds, Sky::pSun] {
            update_component(e, this, field, delta);
        }

        // Moon phases: when the day count moved on (and flag 0x20 is set),
        // or the world flag is set.
        let flags = e.get(this, Sky::uiFlags);
        let day_changed = flags & 0x20 != 0 && {
            let days = e.call(CALENDAR_GET_DAYS, &args![CALENDAR]).u32();
            e.get(this, Sky::uiLastMoonPhaseUpdate) < days
        };
        let world_flag: u32 = e.global(WORLD_STATE_POINTER);
        if day_changed || e.call(WORLD_STATE_FLAG, &args![world_flag]).bool() {
            let phase_applies = e.call(MOON_PHASE_APPLIES, &args![this]).bool();
            for field in [Sky::pMasser, Sky::pSecunda] {
                let moon = e.get(this, field);
                if !moon.is_null() && phase_applies && e.mem.u32(moon.addr() + 0x70) == 0 {
                    let world_flag: u32 = e.global(WORLD_STATE_POINTER);
                    let on = e.call(WORLD_STATE_FLAG, &args![world_flag]).bool();
                    e.mem.set_u32(moon.addr() + 0x70, on as u32 + 1);
                }
            }
            let days = e.call(CALENDAR_GET_DAYS, &args![CALENDAR]).u32();
            e.set(this, Sky::uiLastMoonPhaseUpdate, days);
        }
        for field in [Sky::pMasser, Sky::pSecunda] {
            update_component(e, this, field, delta);
        }
        let precipitation = e.get(this, Sky::pPrecip);
        if !precipitation.is_null() {
            e.call(PRECIPITATION_UPDATE, &args![precipitation, delta]);
        }

        // The lightning flash fades over its duration (a setting, seconds).
        let zero: f64 = e.global(DOUBLE_ZERO);
        if e.get(this, Sky::fFlash) as f64 > zero {
            let now = e.call(TICK_COUNT_GET, &args![TICK_COUNTER]).u32();
            let elapsed = now.wrapping_sub(e.get(this, Sky::uiFlashTime)) as f64;
            let duration: Ptr = e
                .call(SETTING_FLOAT_VALUE, &args![FLASH_DURATION_SETTING])
                .ptr();
            let duration = e.mem.f32(duration.addr()) as f64;
            let thousand: f64 = e.global(THOUSAND_DOUBLE);
            let remaining = 1.0 - elapsed / (duration * thousand);
            e.set(this, Sky::fFlash, remaining as f32);
        }
        if (e.get(this, Sky::fFlash) as f64) < zero {
            e.set(this, Sky::fFlash, 0.0);
        }
        e.call(FOG_COLOR_UPDATE, &args![this.addr() + 0x48]);
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags & !3);
    });
}

/// Calls virtual slot `0xc` (the per-frame update, `Sky*` and `float`) of the
/// component stored in `field`, when there is one.
fn update_component(e: &mut Engine, this: Ptr<Sky>, field: Field<Sky, Ptr>, delta: f32) {
    let component = e.get(this, field);
    if !component.is_null() {
        e.vcall(component.addr(), 0xc, &args![this, delta]);
    }
}

// Translated from 0063b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The short update `Sky::Update` runs in mode 1 without the world flag:
/// colours (`0063b120`), fog (`0063bce0`), then the atmosphere and the sun
/// (virtual slot `0xc`), and `004dce50` on the colour at `+0x48`.
pub fn fn_0063b0a0(e: &mut Engine, this: Ptr<Sky>, delta: f32) {
    fn_0063b120(e, this);
    fn_0063bce0(e, this);
    for field in [Sky::pAtmosphere, Sky::pSun] {
        update_component(e, this, field, delta);
    }
    e.call(FOG_COLOR_UPDATE, &args![this.addr() + 0x48]);
}

// Translated from 0063b120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes the ten sky colours (`SkyColor`, 12 bytes each from `+0x3c`) and
/// the sun specular colour at `+0xc0` for the current mode.
///
/// Mode 0 (`SM_NONE`): fixed colours. Mode 1 (`SM_INTERIOR`) with an
/// interior cell (`011dea10` is the cell holder, `005f36f0` returns its
/// interior cell): the cell's three lighting colours (`00544890`,
/// `005447b0`, `00544a30`). Otherwise the weather blend: `0063b630` picks
/// the two time-of-day sets and their weights, and every colour is mixed
/// by `0063c690` from the weather's colours (`0063bab0`; for the clouds
/// `0063bb70`), with lightning added for colours 3 and 4 when there is no
/// last weather. The colours are then copied down the chain the game keeps
/// them in (`+0xc0`, `+0x9c`, `+0x78`, `+0x3c`, `+0x48`, `+0x90`).
pub fn fn_0063b120(e: &mut Engine, this: Ptr<Sky>) {
    let mode = e.get(this, Sky::eMode);
    let clouds = e.get(this, Sky::pClouds);
    let sky = this.addr();
    let interior_cell = |e: &mut Engine| -> Ptr {
        let holder: u32 = e.global(INTERIOR_CELL_HOLDER);
        if holder == 0 {
            return Ptr::NULL;
        }
        e.call(GET_INTERIOR_CELL, &args![holder]).ptr()
    };
    if mode == 0 {
        let temp = e.mem.alloc(12);
        let (a, b): (f32, f32) = (e.global(0x0101_ffc0), e.global(0x0101_9de0));
        e.call(NI_COLOR_CONSTRUCT, &args![temp, a, b, b]);
        copy_color(e, sky + 0x6c, temp);
        let (a, c): (f32, f32) = (e.global(0x0102_1f44), e.global(0x0104_ee30));
        e.call(NI_COLOR_CONSTRUCT, &args![temp, a, a, c]);
        copy_color(e, sky + 0x60, temp);
        if !clouds.is_null() {
            let mut layer = 0;
            while layer < e.call(CLOUDS_LAYER_COUNT, &args![clouds]).i32() {
                let value: f32 = e.global(0x0101_e2bc);
                e.call(NI_COLOR_CONSTRUCT, &args![temp, value, value, value]);
                fn_0063b600(e, clouds, layer, Ptr::new(temp));
                layer += 1;
            }
        }
        e.mem.free(temp);
        copy_color(e, sky + 0xc0, 0x011f_4998);
        copy_color(e, sky + 0x9c, sky + 0xc0);
        copy_color(e, sky + 0x78, sky + 0x9c);
        copy_color(e, sky + 0x3c, sky + 0x78);
        copy_color(e, sky + 0x48, sky + 0x3c);
        copy_color(e, sky + 0x90, sky + 0x48);
    } else if mode == 1 && !interior_cell(e).is_null() {
        let cell = interior_cell(e);
        e.call(CELL_COLOR_FIRST, &args![cell, sky + 0x6c]);
        let cell = interior_cell(e);
        e.call(CELL_COLOR_SECOND, &args![cell, sky + 0x60]);
        let cell = interior_cell(e);
        e.call(CELL_COLOR_THIRD, &args![cell, sky + 0x48]);
        copy_color(e, sky + 0xc0, sky + 0x48);
        copy_color(e, sky + 0x9c, sky + 0xc0);
        copy_color(e, sky + 0x78, sky + 0x9c);
        copy_color(e, sky + 0x3c, sky + 0x78);
        copy_color(e, sky + 0x90, sky + 0x3c);
        if !clouds.is_null() {
            let mut layer = 0;
            while layer < e.call(CLOUDS_LAYER_COUNT, &args![clouds]).i32() {
                fn_0063b600(e, clouds, layer, Ptr::new(sky + 0x48));
                layer += 1;
            }
        }
    } else {
        let weather = e.get(this, Sky::pCurrentWeather);
        let last_weather = e.get(this, Sky::pLastWeather);
        let percent = e.get(this, Sky::fCurrentWeatherPct);
        let flash = e.get(this, Sky::fFlash);
        let sample = e.mem.alloc(0x20);
        let first_index = e.mem.alloc(4);
        let second_index = e.mem.alloc(4);
        fn_0063b630(
            e,
            this,
            Ptr::new(sample),
            weather,
            percent.to_bits(),
            Ptr::new(first_index),
            Ptr::new(second_index),
        );
        for color in 0..10u32 {
            fn_0063bab0(
                e,
                this,
                Ptr::new(sample),
                weather,
                last_weather,
                color as i32,
                Ptr::new(first_index),
                Ptr::new(second_index),
            );
            let mut lightning = 0.0f32;
            if last_weather.is_null() {
                if color == 4 {
                    lightning = lightning_scaled(e, flash, LIGHTNING_SCALE_COLOR_4);
                } else if color == 3 {
                    lightning = lightning_scaled(e, flash, LIGHTNING_SCALE_COLOR_3);
                }
            }
            fn_0063c690(
                e,
                this,
                Ptr::new(sky + 0x3c + color * SKY_COLOR_STRIDE),
                Ptr::new(sample),
                lightning,
            );
        }
        if !clouds.is_null() {
            let temp = e.mem.alloc(12);
            let mut layer = 0;
            while layer < e.call(CLOUDS_LAYER_COUNT, &args![clouds]).i32() {
                fn_0063bb70(
                    e,
                    this,
                    Ptr::new(sample),
                    weather,
                    last_weather,
                    layer,
                    Ptr::new(first_index),
                    Ptr::new(second_index),
                );
                let mut lightning = 0.0f32;
                if last_weather.is_null() && layer == 0 {
                    lightning = flash;
                }
                e.call(NI_COLOR_CONSTRUCT, &args![temp, 0.0f32, 0.0f32, 0.0f32]);
                fn_0063c690(e, this, Ptr::new(temp), Ptr::new(sample), lightning);
                fn_0063b600(e, clouds, layer, Ptr::new(temp));
                layer += 1;
            }
            e.mem.free(temp);
        }
        e.mem.free(sample);
        e.mem.free(first_index);
        e.mem.free(second_index);
        copy_color(e, sky + 0xc0, sky + 0x6c);
    }
}

/// `flash` times the `float` setting object at `setting` (the lightning's
/// contribution to colour 3 or 4).
fn lightning_scaled(e: &mut Engine, flash: f32, setting: u32) -> f32 {
    let value: Ptr = e.call(SETTING_FLOAT_VALUE, &args![setting]).ptr();
    (flash as f64 * e.mem.f32(value.addr()) as f64) as f32
}

// Translated from 0063b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds` method (by its caller `Sky::fn_0063b120`): copies the colour at
/// `color` into the layer's slot at `+0x28 + 12 * index`.
pub fn fn_0063b600(e: &mut Engine, this: Ptr, index: i32, color: Ptr) {
    let slot = this
        .addr()
        .wrapping_add(0x28)
        .wrapping_add((index as u32).wrapping_mul(SKY_COLOR_STRIDE));
    copy_color(e, slot, color.addr());
}

// Translated from 0063b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the two time-of-day colour sets for the current hour and their
/// weights. `sunrise_begin`, `sunrise_end`, `sunset_begin` and `sunset_end`
/// (`0063b9b0`, `00595f50`, `00595fc0`, `0063ba30`) divide the day into
/// night (index 3), sunrise (0 to 1), day (1) and sunset (4 to 2); `high_noon`
/// (`+0x12c`) splits the day. Writes the indices into `first_index_out` and
/// `second_index_out`, and into `sample` (`+0x10` to `+0x1c`) the four
/// weights: the blend towards the second set and its complement, each scaled
/// by the weather percentage (`+0xf4`) and its complement. Does nothing when
/// `weather` is null. An hour outside every range logs the master file data
/// error `"MASTERFILE: Data error detected--Transition times stored in
/// climate data are invalid."` (`005b5e40`) and uses set 1 for both. The
/// third word is passed but not read.
pub fn fn_0063b630(
    e: &mut Engine,
    this: Ptr<Sky>,
    sample: Ptr,
    weather: Ptr,
    _unused_2: u32,
    first_index_out: Ptr,
    second_index_out: Ptr,
) {
    if weather.is_null() {
        return;
    }
    let sunrise_begin = fn_0063b9b0(e, this);
    let sunrise_end = e.call(SKY_SUNRISE_END, &args![this]).f32();
    let sunset_begin = e.call(SKY_SUNSET_BEGIN, &args![this]).f32();
    let sunset_end = fn_0063ba30(e, this);
    let hour = e.get(this, Sky::fCurrentGameHour);
    let high_noon = e.get(this, Sky::fHighNoon);
    let (a, b, c, d) = (sunrise_begin, sunrise_end, sunset_begin, sunset_end);
    let first = first_index_out.addr();
    let second = second_index_out.addr();
    let blend: f32;
    if a < hour && hour < b {
        e.mem.set_u32(first, 0);
        let half = ((b as f64 - a as f64) * e.global::<f64>(DOUBLE_HALF)) as f32;
        let middle = (a as f64 + half as f64) as f32;
        if middle > hour {
            blend = (1.0 - (middle as f64 - hour as f64) / half as f64) as f32;
            e.mem.set_u32(second, 3);
        } else {
            blend = (1.0 - (hour as f64 - middle as f64) / half as f64) as f32;
            e.mem.set_u32(second, 1);
        }
    } else if b <= hour && hour <= high_noon {
        // 0063b72f FCOMPP; TEST AH,0x41; JP: an hour equal to the sunrise end comes here.
        e.mem.set_u32(second, 1);
        e.mem.set_u32(first, 4);
        let span = (high_noon as f64 - b as f64) as f32;
        blend = (1.0 - (high_noon as f64 - hour as f64) / span as f64) as f32;
    } else if high_noon <= hour && hour <= c {
        e.mem.set_u32(second, 4);
        e.mem.set_u32(first, 1);
        let span = (c as f64 - high_noon as f64) as f32;
        blend = (1.0 - (c as f64 - hour as f64) / span as f64) as f32;
    } else if c < hour && hour < d {
        e.mem.set_u32(first, 2);
        let half = ((d as f64 - c as f64) * e.global::<f64>(DOUBLE_HALF)) as f32;
        let middle = (c as f64 + half as f64) as f32;
        if middle > hour {
            blend = (1.0 - (middle as f64 - hour as f64) / half as f64) as f32;
            e.mem.set_u32(second, 1);
        } else {
            blend = (1.0 - (hour as f64 - middle as f64) / half as f64) as f32;
            e.mem.set_u32(second, 3);
        }
    } else if d <= hour || a >= hour {
        // Night.
        e.mem.set_u32(second, 3);
        e.mem.set_u32(first, 3);
        blend = 1.0;
    } else {
        e.call(LOG_MASTERFILE_ERROR, &args![TRANSITION_TIMES_MESSAGE]);
        e.mem.set_u32(second, 1);
        e.mem.set_u32(first, 1);
        blend = 1.0;
    }
    let percent = e.get(this, Sky::fCurrentWeatherPct);
    let complement = (1.0 - blend as f64) as f32;
    e.mem.set_f32(sample.addr() + 0x10, blend);
    e.mem.set_f32(sample.addr() + 0x14, complement);
    e.mem.set_f32(sample.addr() + 0x18, blend);
    e.mem.set_f32(sample.addr() + 0x1c, complement);
    let first_scaled = (blend as f64 * percent as f64) as f32;
    let second_scaled = (complement as f64 * percent as f64) as f32;
    e.mem.set_f32(sample.addr() + 0x10, first_scaled);
    e.mem.set_f32(sample.addr() + 0x14, second_scaled);
    let rest = 1.0 - percent as f64;
    let third = (rest * e.mem.f32(sample.addr() + 0x18) as f64) as f32;
    e.mem.set_f32(sample.addr() + 0x18, third);
    let fourth = (rest * e.mem.f32(sample.addr() + 0x1c) as f64) as f32;
    e.mem.set_f32(sample.addr() + 0x1c, fourth);
}

// Translated from 0063b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sunrise begin hour as the sky uses it: when flag `0x1000` is set,
/// recomputes it as `max(0, Sky::GetSunriseBegin (00595ea0) - fTimeTransition)`
/// into the cache at `011cccfc` and clears the flag; returns the cache.
pub fn fn_0063b9b0(e: &mut Engine, this: Ptr<Sky>) -> f32 {
    let flags = e.get(this, Sky::uiFlags);
    if flags & 0x1000 != 0 {
        let begin = e.call(SKY_SUNRISE_BEGIN, &args![this]).f64();
        let setting: Ptr = e.call(SETTING_FLOAT_VALUE, &args![TIME_SETTING]).ptr();
        let margin = e.mem.f32(setting.addr()) as f64;
        let value = (begin - margin) as f32;
        let clamped = e.call(MAX_FLOAT, &args![0.0f32, value]).f32();
        e.set_global(SUNRISE_BEGIN_CACHE, clamped);
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags & 0xffff_efff);
    }
    e.global(SUNRISE_BEGIN_CACHE)
}

// Translated from 0063ba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sunset end hour as the sky uses it: when flag `0x2000` is set,
/// recomputes it as `min(Sky sunset end (00596030) + fTimeTransition,
/// [0104ede4])` into the cache at `011ccd00` and clears the flag; returns the
/// cache.
pub fn fn_0063ba30(e: &mut Engine, this: Ptr<Sky>) -> f32 {
    let flags = e.get(this, Sky::uiFlags);
    if flags & 0x2000 != 0 {
        let end = e.call(SKY_SUNSET_END, &args![this]).f64();
        let setting: Ptr = e.call(SETTING_FLOAT_VALUE, &args![TIME_SETTING]).ptr();
        let margin = e.mem.f32(setting.addr()) as f64;
        let value = (margin + end) as f32;
        let upper: f32 = e.global(0x0104_ede4);
        let clamped = e.call(MIN_FLOAT, &args![upper, value]).f32();
        e.set_global(SUNSET_END_CACHE, clamped);
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags & 0xffff_dfff);
    }
    e.global(SUNSET_END_CACHE)
}

// Translated from 0063bab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads four colours of the weathers `weather_a` and `weather_b` for the
/// time-of-day `index`: `out[0]`/`out[1]` from `weather_a` at the indices
/// stored at `first_index`/`second_index`, `out[2]`/`out[3]` the same from
/// `weather_b` (0 when there is none). Does nothing when `weather_a` is null.
/// The `__thiscall` receiver is not read.
#[allow(clippy::too_many_arguments)]
pub fn fn_0063bab0(
    e: &mut Engine,
    _unused_this: Ptr<Sky>,
    out: Ptr,
    weather_a: Ptr,
    weather_b: Ptr,
    index: i32,
    first_index: Ptr,
    second_index: Ptr,
) {
    if weather_a.is_null() {
        return;
    }
    let first = e.mem.i32(first_index.addr());
    let value = fn_0063bb40(e, weather_a, index, first);
    e.mem.set_u32(out.addr(), value);
    let second = e.mem.i32(second_index.addr());
    let value = fn_0063bb40(e, weather_a, index, second);
    e.mem.set_u32(out.addr() + 4, value);
    if weather_b.is_null() {
        e.mem.set_u32(out.addr() + 0xc, 0);
        e.mem.set_u32(out.addr() + 8, 0);
    } else {
        let first = e.mem.i32(first_index.addr());
        let value = fn_0063bb40(e, weather_b, index, first);
        e.mem.set_u32(out.addr() + 8, value);
        let second = e.mem.i32(second_index.addr());
        let value = fn_0063bb40(e, weather_b, index, second);
        e.mem.set_u32(out.addr() + 0xc, value);
    }
}

// Translated from 0063bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather` colour accessor (by its caller): the packed colour at
/// `weather + 0x108 + 0x18 * index + 4 * time_of_day`.
pub fn fn_0063bb40(e: &mut Engine, this: Ptr, index: i32, time_of_day: i32) -> u32 {
    let row = this
        .addr()
        .wrapping_add((index as u32).wrapping_mul(0x18))
        .wrapping_add(0x108);
    e.mem
        .u32(row.wrapping_add((time_of_day as u32).wrapping_mul(4)))
}

// Translated from 0063bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_0063bab0`] for the cloud layers (colours through
/// [`fn_0063bc60`]); when there is a `weather_b` a colour that is 0 is
/// replaced by the other weather's colour at the same position. The
/// `__thiscall` receiver is not read.
#[allow(clippy::too_many_arguments)]
pub fn fn_0063bb70(
    e: &mut Engine,
    _unused_this: Ptr<Sky>,
    out: Ptr,
    weather_a: Ptr,
    weather_b: Ptr,
    layer: i32,
    first_index: Ptr,
    second_index: Ptr,
) {
    if weather_a.is_null() {
        return;
    }
    let first = e.mem.i32(first_index.addr());
    let value = fn_0063bc60(e, weather_a, layer, first);
    e.mem.set_u32(out.addr(), value);
    let second = e.mem.i32(second_index.addr());
    let value = fn_0063bc60(e, weather_a, layer, second);
    e.mem.set_u32(out.addr() + 4, value);
    if weather_b.is_null() {
        e.mem.set_u32(out.addr() + 0xc, 0);
        e.mem.set_u32(out.addr() + 8, 0);
        return;
    }
    let first = e.mem.i32(first_index.addr());
    let value = fn_0063bc60(e, weather_b, layer, first);
    e.mem.set_u32(out.addr() + 8, value);
    let second = e.mem.i32(second_index.addr());
    let value = fn_0063bc60(e, weather_b, layer, second);
    e.mem.set_u32(out.addr() + 0xc, value);
    for (target, other) in [(0u32, 8u32), (4, 0xc), (8, 0), (0xc, 4)] {
        if e.mem.u32(out.addr() + target) == 0 {
            let value = e.mem.u32(out.addr() + other);
            e.mem.set_u32(out.addr() + target, value);
        }
    }
}

// Translated from 0063bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather` cloud colour accessor (by its caller): the packed colour of
/// cloud layer `layer` for `time_of_day` (the layer count is at `+0x368`,
/// the colours at `+0x68 + 0x18 * layer`; a layer past the count uses layer
/// 0). Without layers, or before the colours are known, the built-in
/// colours: `0x9b9b9b` for time 1, `0x1c140f` for 3, `0x5b4b46` for 0 and
/// `0x6c5953` otherwise.
pub fn fn_0063bc60(e: &mut Engine, this: Ptr, layer: i32, time_of_day: i32) -> u32 {
    let mut color = match time_of_day {
        1 => 0x9b9b9b,
        3 => 0x1c140f,
        0 => 0x5b4b46,
        _ => 0x6c5953,
    };
    let count = e.mem.i32(this.addr() + 0x368);
    if count > 0 {
        let layer = if layer >= count { 0 } else { layer };
        let row = this
            .addr()
            .wrapping_add((layer as u32).wrapping_mul(0x18))
            .wrapping_add(0x68);
        color = e
            .mem
            .u32(row.wrapping_add((time_of_day as u32).wrapping_mul(4)));
    }
    color
}

// Translated from 0063bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes the fog near and far distances (`+0xd4`, `+0xd8`) and the fog
/// power (`+0xe8`).
///
/// When one of the underwater flags (`004e2180`, `004e3c40`) is set and the
/// camera is below the water (or the second flag is set) and a water volume
/// is known (`0063c470`), the fog comes from the water volume
/// (`004e3d00`, `00507b20`), the water colour feeds `0063c690` into the colour
/// at `+0x48`, and in modes 2 and 3 the sky nodes are hidden once the camera
/// is more than half way below the surface. Otherwise: with a current weather
/// in mode 2 or 3, the fog is the hour-weighted mix of the weather's fog
/// values (`0063c420`; plus the last weather's, scaled by the weather
/// percentage); in mode 1 with an interior cell the cell's fog (`00544b10`,
/// `00544ab0`, `00544b70`, with the far distance clamped to 163840.0); else
/// the default `[010317c4]` for both distances and power 1.0. Unless
/// `0063c460` is set the far distance is then pulled to the camera's far
/// plane (virtual slot `0x100` of the camera, `fFogSetting`) and the near
/// distance rescaled with it.
pub fn fn_0063bce0(e: &mut Engine, this: Ptr<Sky>) {
    let water = fn_0063c470(e);
    if e.call(FLAG_UNDERWATER_A, &[]).bool() || e.call(FLAG_UNDERWATER_B, &[]).bool() {
        let camera: Ptr = e.call(CAMERA_HOLDER_GET, &[]).ptr();
        let node: Ptr = e.call(CAMERA_NODE, &args![camera]).ptr();
        let position: Ptr = e.call(NODE_WORLD_POSITION, &args![node]).ptr();
        let camera_height = e.mem.f32(position.addr() + 8);
        if water != 0 {
            let mut below = true;
            if !e.call(FLAG_UNDERWATER_B, &[]).bool() {
                let player: u32 = e.global(PLAYER_CHARACTER);
                let cell: Ptr = e.call(PLAYER_PARENT_CELL, &args![player]).ptr();
                let surface = e.call(CELL_WATER_HEIGHT, &args![cell]).f64();
                below = (camera_height as f64) < surface;
            }
            if below {
                fog_from_water(e, this, water, camera_height);
                return;
            }
        }
    }
    let weather = e.get(this, Sky::pCurrentWeather);
    let mode = e.get(this, Sky::eMode);
    if !weather.is_null() && (mode == 3 || mode == 2) {
        fog_from_weather(e, this, weather);
    } else {
        let holder: u32 = e.global(INTERIOR_CELL_HOLDER);
        if mode == 1
            && holder != 0
            && !e
                .call(GET_INTERIOR_CELL, &args![holder])
                .ptr::<()>()
                .is_null()
        {
            let far_default: f32 = e.global(DEFAULT_FOG_DISTANCE);
            let cell: Ptr = e.call(GET_INTERIOR_CELL, &args![holder]).ptr();
            let mut far = e.call(CELL_FOG_FAR, &args![cell]).f32();
            let zero: f64 = e.global(DOUBLE_ZERO);
            let limit: f64 = e.global(FOG_FAR_LIMIT);
            if far as f64 <= zero || far as f64 > limit {
                far = far_default;
            }
            let cell: Ptr = e.call(GET_INTERIOR_CELL, &args![holder]).ptr();
            let mut near = e.call(CELL_FOG_NEAR, &args![cell]).f32();
            if near as f64 <= zero || far < near {
                let factor: f64 = e.global(FOG_NEAR_FACTOR);
                near = (far as f64 * factor) as f32;
            }
            e.set(this, Sky::fFogNear, near);
            e.set(this, Sky::fFogFar, far);
            let cell: Ptr = e.call(GET_INTERIOR_CELL, &args![holder]).ptr();
            let power = e.call(CELL_FOG_POWER, &args![cell]).f32();
            e.set(this, Sky::fFogPower, power);
        } else {
            let default: f32 = e.global(DEFAULT_FOG_DISTANCE);
            e.set(this, Sky::fFogNear, default);
            e.set(this, Sky::fFogFar, default);
            e.set(this, Sky::fFogPower, 1.0);
        }
    }
    if !fn_0063c460(e) {
        let camera: Ptr = e.call(CAMERA_HOLDER_GET, &[]).ptr();
        let camera_far = e.vcall(camera.addr(), 0x100, &[]).f32();
        let setting: Ptr = e.call(SETTING_FLOAT_VALUE, &args![FOG_SETTING]).ptr();
        let near_limit = e.mem.f32(setting.addr());
        let far = e.get(this, Sky::fFogFar);
        let near = e.get(this, Sky::fFogNear);
        let ratio = ((far as f64 - camera_far as f64) / (far as f64 - near_limit as f64)) as f32;
        let pulled = (near as f64 - (near as f64 - near_limit as f64) * ratio as f64) as f32;
        e.set(this, Sky::fFogNear, pulled);
        e.set(this, Sky::fFogFar, camera_far);
        let result = e.call(MIN_FLOAT, &args![camera_far, pulled]).f32();
        e.set(this, Sky::fFogNear, result);
    }
}

/// The underwater branch of [`fn_0063bce0`]: the fog of the water volume at
/// `water`, the water colour fed to `0063c690`, and (modes 2 and 3) the sky
/// nodes hidden when the camera is more than half way below the surface.
fn fog_from_water(e: &mut Engine, this: Ptr<Sky>, water: u32, camera_height: f32) {
    let near = e.call(WATER_FOG_NEAR, &args![water]).f32();
    e.set(this, Sky::fFogNear, near);
    let far = e.call(WATER_FOG_FAR, &args![water]).f32();
    e.set(this, Sky::fFogFar, far);
    e.set(this, Sky::fFogPower, 1.0);
    let mut surface = fn_0063c480(e);
    let player: u32 = e.global(PLAYER_CHARACTER);
    let cell: Ptr = e.call(PLAYER_PARENT_CELL, &args![player]).ptr();
    let cell_surface = e.call(CELL_WATER_HEIGHT, &args![cell]).f64();
    if (camera_height as f64) < cell_surface {
        let cell: Ptr = e.call(PLAYER_PARENT_CELL, &args![player]).ptr();
        surface = e.call(CELL_WATER_HEIGHT, &args![cell]).f32();
    }
    let depth_scale: f32 = e.global(WATER_DEPTH_SCALE);
    let mut depth = ((surface as f64 - camera_height as f64) * depth_scale as f64) as f32;
    let limit: f64 = e.global(DOUBLE_ONE);
    if depth as f64 > limit {
        depth = 1.0;
    }
    e.with_stack(0x20, |e, sample| {
        let first = e.call(WATER_COLOR_A, &args![water]).u32();
        let second = e.call(WATER_COLOR_B, &args![water]).u32();
        e.mem.set_u32(sample.addr(), first);
        e.mem.set_u32(sample.addr() + 4, second);
        e.mem.set_u32(sample.addr() + 8, 0);
        e.mem.set_u32(sample.addr() + 12, 0);
        e.mem
            .set_f32(sample.addr() + 0x10, (1.0 - depth as f64) as f32);
        e.mem.set_f32(sample.addr() + 0x14, depth);
        e.mem.set_f32(sample.addr() + 0x18, 0.0);
        e.mem.set_f32(sample.addr() + 0x1c, 0.0);
        fn_0063c690(e, this, Ptr::new(this.addr() + 0x48), sample, 0.0);
    });
    let mode = e.get(this, Sky::eMode);
    if mode == 3 || mode == 2 {
        let half: f64 = e.global(DOUBLE_HALF);
        let hide = depth as f64 > half;
        let clouds = ni_pointer_get_clouds(e, this);
        let layers = e.call(CLOUDS_LAYER_COUNT, &args![clouds]).i32();
        for layer in 0..layers {
            let clouds = ni_pointer_get_clouds(e, this);
            let node = fn_0063c440(e, clouds, layer);
            e.call(SET_NODE_HIDDEN, &args![node, hide as u32]);
        }
        let atmosphere: Ptr = e.call(SKY_GET_ATMOSPHERE, &args![this]).ptr();
        let node: Ptr = e.call(HOLDER_NODE, &args![atmosphere]).ptr();
        e.call(SET_NODE_HIDDEN, &args![node, hide as u32]);
        let sun: Ptr = e.call(SKY_GET_SUN, &args![this]).ptr();
        let node: Ptr = e.call(HOLDER_NODE, &args![sun]).ptr();
        e.call(SET_NODE_HIDDEN, &args![node, hide as u32]);
        let sun: Ptr = e.call(SKY_GET_SUN, &args![this]).ptr();
        let node: Ptr = e.call(SUN_GET_NODE, &args![sun]).ptr();
        e.call(SET_NODE_HIDDEN, &args![node, hide as u32]);
        let root = ni_pointer_get(e, this.addr() + 4);
        e.call(SET_NODE_HIDDEN, &args![root, hide as u32]);
    }
}

/// `Sky::GetClouds` (`0055b980`, returns `+0x2c`).
fn ni_pointer_get_clouds(e: &mut Engine, this: Ptr<Sky>) -> Ptr {
    e.call(SKY_GET_CLOUDS, &args![this]).ptr()
}

/// The weather branch of [`fn_0063bce0`]: the fog values mixed from the
/// weather's fog table by the hour, and by the last weather and the weather
/// percentage.
fn fog_from_weather(e: &mut Engine, this: Ptr<Sky>, weather: Ptr) {
    let sunrise_begin = fn_0063b9b0(e, this);
    let sunrise_end = e.call(SKY_SUNRISE_END, &args![this]).f32();
    let sunset_begin = e.call(SKY_SUNSET_BEGIN, &args![this]).f32();
    let sunset_end = fn_0063ba30(e, this);
    let hour = e.get(this, Sky::fCurrentGameHour);
    let blend: f32 = if sunrise_begin < hour && hour < sunrise_end {
        ((hour as f64 - sunrise_begin as f64) / (sunrise_end as f64 - sunrise_begin as f64)) as f32
    } else if sunrise_end <= hour && hour <= sunset_begin {
        1.0
    } else if sunset_begin < hour && hour < sunset_end {
        ((sunset_end as f64 - hour as f64) / (sunset_end as f64 - sunset_begin as f64)) as f32
    } else {
        0.0
    };
    let complement = (1.0 - blend as f64) as f32;
    let mix = |e: &mut Engine, from: Ptr, day: i32, night: i32| -> f64 {
        let first = fn_0063c420(e, from, day);
        let first = first as f64 * blend as f64;
        let second = fn_0063c420(e, from, night);
        second as f64 * complement as f64 + first
    };
    let fields = [
        (Sky::fFogNear, 0, 2),
        (Sky::fFogFar, 1, 3),
        (Sky::fFogPower, 4, 5),
    ];
    for (field, day, night) in fields {
        let value = mix(e, weather, day, night) as f32;
        e.set(this, field, value);
    }
    let last_weather = e.get(this, Sky::pLastWeather);
    if !last_weather.is_null() {
        for (field, day, night) in fields {
            let percent = e.get(this, Sky::fCurrentWeatherPct);
            let current = e.get(this, field);
            let scaled = (current as f64 * percent as f64) as f32;
            e.set(this, field, scaled);
            let rest = 1.0 - e.get(this, Sky::fCurrentWeatherPct) as f64;
            let last_value = mix(e, last_weather, day, night);
            let result = last_value * rest + e.get(this, field) as f64;
            e.set(this, field, result as f32);
        }
    }
}

// Translated from 0063c420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather` fog accessor (by its caller): the `float` at
/// `weather + 0xf0 + 4 * index`.
pub fn fn_0063c420(e: &mut Engine, this: Ptr, index: i32) -> f32 {
    e.mem.f32(
        this.addr()
            .wrapping_add((index as u32).wrapping_mul(4))
            .wrapping_add(0xf0),
    )
}

// Translated from 0063c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds` accessor (by its caller): the node pointer stored at
/// `clouds + 8 + 4 * index` (`NiPointer::get`).
pub fn fn_0063c440(e: &mut Engine, this: Ptr, index: i32) -> Ptr {
    let slot = this
        .addr()
        .wrapping_add((index as u32).wrapping_mul(4))
        .wrapping_add(8);
    ni_pointer_get(e, slot)
}

// Translated from 0063c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `0119f188`.
pub fn fn_0063c460(e: &mut Engine) -> bool {
    e.global(0x0119_f188)
}

// Translated from 0063c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `011c7a3c` (the water volume the fog code reads).
pub fn fn_0063c470(e: &mut Engine) -> u32 {
    e.global(0x011c_7a3c)
}

// Translated from 0063c480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `float` at `011c7a5c`.
pub fn fn_0063c480(e: &mut Engine) -> f32 {
    e.global(0x011c_7a5c)
}

// Translated from 0063c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The wind: in modes 2 and 3 with a weather, `fWindSpeed` is the weather's
/// wind speed (`004ed230`, byte 0 of its table scaled to 0..1), mixed with
/// the last weather's by the weather percentage; then it publishes the speed
/// (`0063c660`) and two values from the time of day (`0063c640` gets the
/// words at `011a9b50`; `0063c670` gets 15 times the lookup sine of the
/// phase and 15 times the sine of 0.7 of it). The compiler's no-op call of
/// `006815c0` on a local is left out.
pub fn fn_0063c490(e: &mut Engine, this: Ptr<Sky>) {
    let weather = e.get(this, Sky::pCurrentWeather);
    let mode = e.get(this, Sky::eMode);
    if weather.is_null() || !(mode == 3 || mode == 2) {
        return;
    }
    let speed = e
        .call(WEATHER_BYTE_FRACTION, &args![weather, 0u32, 1.0f32, 0.0f32])
        .f32();
    e.set(this, Sky::fWindSpeed, speed);
    let last_weather = e.get(this, Sky::pLastWeather);
    if !last_weather.is_null() {
        let percent = e.get(this, Sky::fCurrentWeatherPct);
        let scaled = (speed as f64 * percent as f64) as f32;
        e.set(this, Sky::fWindSpeed, scaled);
        let rest = 1.0 - e.get(this, Sky::fCurrentWeatherPct) as f64;
        let last_speed = e
            .call(
                WEATHER_BYTE_FRACTION,
                &args![last_weather, 0u32, 1.0f32, 0.0f32],
            )
            .f64();
        let mixed = last_speed * rest + scaled as f64;
        e.set(this, Sky::fWindSpeed, mixed as f32);
    }
    let (first, second): (u32, u32) = (e.global(0x011a_9b50), e.global(0x011a_9b54));
    fn_0063c640(e, first, second);
    let speed = e.get(this, Sky::fWindSpeed);
    fn_0063c660(e, speed);
    let fifteen: f32 = e.global(0x0101_e580);
    let seconds = e.call(0x0045_2e70, &args![0u32]).f64();
    let hour_seconds: f64 = e.global(0x0101_2640);
    let two_pi: f64 = e.global(0x0104_eea0);
    let phase = (seconds / hour_seconds * two_pi * fifteen as f64) as f32;
    let sine = fn_0063c600(e, phase);
    let x = (sine as f64 * fifteen as f64) as f32;
    let factor: f64 = e.global(0x0104_ee98);
    let shifted = (phase as f64 * factor) as f32;
    let other = e.call(0x0057_e960, &args![shifted]).f32();
    let y = (other as f64 * fifteen as f64) as f32;
    fn_0063c670(e, x, y);
}

// Translated from 0063c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sine lookup (by its callers): `table[(int)(k / [011ccc50] * angle) & 0x1ff]`
/// with `k = [01028338]` and the table of 512 `float`s at `011f52e0`.
pub fn fn_0063c600(e: &mut Engine, angle: f32) -> f32 {
    let divisor: f32 = e.global(0x011c_cc50);
    let numerator: f64 = e.global(0x0102_8338);
    let scaled = numerator / divisor as f64 * angle as f64;
    let index = e.call(FTOL, &args![scaled]).u32() & 0x1ff;
    e.global(0x011f_52e0 + 4 * index)
}

// Translated from 0063c640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores its two words in the globals `011f95dc` and `011f95e0`.
pub fn fn_0063c640(e: &mut Engine, first: u32, second: u32) {
    e.set_global(0x011f_95dc, first);
    e.set_global(0x011f_95e0, second);
}

// Translated from 0063c660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores its `float` in the global `011ad824`.
pub fn fn_0063c660(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d824, value);
}

// Translated from 0063c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores its two `float`s in the globals `011fd884` and `011fd888`.
pub fn fn_0063c670(e: &mut Engine, first: f32, second: f32) {
    e.set_global(0x011f_d884, first);
    e.set_global(0x011f_d888, second);
}

// Translated from 0063c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Mixes a colour (`Sky` method; `this` supplies the current weather at
/// `+0x10`). `source` holds four packed colours (`0xBBGGRR`, one per word)
/// followed by their four `float` weights: each output channel of `dest` is
/// the weighted sum of that channel's bytes divided by 255. `lightning` is
/// clamped to at most 1.0; when it is positive it is added to every channel
/// and, if there is a weather, each channel is then clamped to
/// `[0, weather byte fraction (table bytes 0xc, 0xd, 0xe)]`; otherwise (or
/// without a weather) every channel is clamped to at most 1.0 (`00a69690`).
pub fn fn_0063c690(e: &mut Engine, this: Ptr<Sky>, dest: Ptr, source: Ptr, lightning: f32) {
    let scale: f64 = e.global(0x0102_31e8);
    for channel in 0..3u32 {
        let mut sum = 0.0f32;
        for j in 0..4u32 {
            let packed = e.mem.u32(source.addr() + 4 * j);
            let byte = (packed >> (8 * channel)) & 0xff;
            let weight = e.mem.f32(source.addr() + 0x10 + 4 * j);
            sum = (byte as f64 * weight as f64 + sum as f64) as f32;
        }
        e.mem
            .set_f32(dest.addr() + 4 * channel, (sum as f64 * scale) as f32);
    }
    let lightning = e.call(MIN_FLOAT, &args![1.0f32, lightning]).f32();
    let zero: f64 = e.global(DOUBLE_ZERO);
    if lightning as f64 > zero {
        e.with_stack(12, |e, tint| {
            e.call(
                NI_COLOR_CONSTRUCT,
                &args![tint, lightning, lightning, lightning],
            );
            fn_0063c8a0(e, dest, tint);
        });
        let weather = e.get(this, Sky::pCurrentWeather);
        if !weather.is_null() {
            for channel in 0..3u32 {
                let limit = e
                    .call(
                        WEATHER_BYTE_FRACTION,
                        &args![weather, 0xcu32 + channel, 1.0f32, 0.0f32],
                    )
                    .f32();
                e.call(
                    CLAMP_FLOAT,
                    &args![dest.addr() + 4 * channel, 0.0f32, limit],
                );
            }
            return;
        }
    }
    e.call(COLOR_CLAMP_TO_ONE, &args![dest]);
}

// Translated from 0063c8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColor::operator+=` (by its use): adds `other`'s three `float`s to
/// `this` and returns `this`.
pub fn fn_0063c8a0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    for channel in 0..3u32 {
        let sum = e.mem.f32(this.addr() + 4 * channel) as f64
            + e.mem.f32(other.addr() + 4 * channel) as f64;
        e.mem.set_f32(this.addr() + 4 * channel, sum as f32);
    }
    this
}

// Translated from 0063c8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::SetCurrentClimate` (Xbox PDB): when `climate` is null and there is
/// no current climate (or `force` is set), looks up the form with id `0x15f`
/// (`004839c0`) and casts it to a climate with `__RTDynamicCast`. A non-null
/// climate that differs from the current one (or any with `force`) becomes
/// current: flags `0x3f00` are set (`0063c9f0`), the weather is reset
/// (`Sky::ResetWeather`, `0063d060`) and flag `0x40` is set. The compiler's
/// exception frame is not translated.
pub fn sky_set_current_climate(e: &mut Engine, this: Ptr<Sky>, climate: Ptr, force: bool) {
    with_timer(e, 0x5b8, |e| {
        let mut climate = climate;
        if climate.is_null() && (e.get(this, Sky::pCurrentClimate).is_null() || force) {
            let form: Ptr = e.call(LOOKUP_FORM, &args![0x15fu32]).ptr();
            climate = e
                .call(
                    DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        TYPE_DESCRIPTOR_FORM,
                        TYPE_DESCRIPTOR_CLIMATE,
                        0u32
                    ],
                )
                .ptr();
        }
        if climate.is_null() {
            return;
        }
        if e.get(this, Sky::pCurrentClimate) != climate || force {
            e.set(this, Sky::pCurrentClimate, climate);
            fn_0063c9f0(e, this);
            e.call(SKY_RESET_WEATHER, &args![this]);
            let flags = e.get(this, Sky::uiFlags);
            e.set(this, Sky::uiFlags, flags | 0x40);
        }
    });
}

// Translated from 0063c9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets flags `0x3f00` of the sky (marks every cached climate time stale).
pub fn fn_0063c9f0(e: &mut Engine, this: Ptr<Sky>) {
    let flags = e.get(this, Sky::uiFlags);
    e.set(this, Sky::uiFlags, flags | 0x3f00);
}

// Translated from 0063ca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the climate-dependent parts of the sky when flag `0x40` is set
/// (`Sky::SetCurrentClimate` sets it), then clears the flag. In modes 2 and
/// 3: loads the stars' geometry (`Stars::LoadGeometry`, `0063fdb0`, with the
/// value of `0063cfe0` on the climate); loads the sun's two textures (the
/// climate's entries 0 and 1 of `0063cfa0`; a name equal to
/// `"Sky\SunGlare.dds"` with the flag byte `011f941e` clear is replaced by
/// `"Textures\Sky\SunGlareNonHDR.dds"`) into the properties of the sun's
/// nodes and marks both nodes by whether a texture loaded (`00635fe0`). Then
/// creates the "Masser" and "Secunda" moons (`Moon::Moon`, `00634a70`) when
/// the climate's byte at `+0x55` has bit `0x80` / `0x40` set and deletes them
/// when it has not; updates the root's properties and its controllers
/// (`00a59c60` with a zeroed `NiUpdateData`). The compiler's exception frame
/// is not translated.
pub fn fn_0063ca20(e: &mut Engine, this: Ptr<Sky>) {
    if e.get(this, Sky::uiFlags) & 0x40 == 0 {
        return;
    }
    let mode = e.get(this, Sky::eMode);
    let climate = e.get(this, Sky::pCurrentClimate);
    let stars = e.get(this, Sky::pStars);
    if !stars.is_null() && (mode == 3 || mode == 2) {
        let geometry = fn_0063cfe0(e, climate);
        e.call(STARS_LOAD_GEOMETRY, &args![stars, geometry]);
    }
    let sun = e.get(this, Sky::pSun);
    if !sun.is_null() && (mode == 3 || mode == 2) {
        load_sun_textures(e, sun, climate);
    }
    let wanted = fn_0063d000(e, climate);
    update_moon(e, this, Sky::pMasser, wanted, MASSER_MOON);
    let wanted = fn_0063d020(e, climate);
    update_moon(e, this, Sky::pSecunda, wanted, SECUNDA_MOON);
    let root = ni_pointer_get(e, this.addr() + 4);
    e.call(ROOT_UPDATE_PROPERTIES, &args![root]);
    e.with_stack(12, |e, update_data| {
        e.call(
            NI_UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        let root = ni_pointer_get(e, this.addr() + 4);
        e.call(NODE_UPDATE_CONTROLLERS, &args![root, update_data]);
    });
    let flags = e.get(this, Sky::uiFlags);
    e.set(this, Sky::uiFlags, flags & 0xffff_ffbf);
}

/// Builds the sun's two textures from the climate's entries 0 and 1 and
/// attaches them to the properties of the sun's nodes (the node at `+0x10`
/// and the node at `+0x14` of the sun's holder), then marks the sun's nodes
/// by whether a texture was found. `texture` is not reset between the two
/// loads, as in the game.
fn load_sun_textures(e: &mut Engine, sun: Ptr, climate: Ptr) {
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCT, &args![text]);
        let node: Ptr = e.call(SUN_FIRST_NODE, &args![sun]).ptr();
        let property: Ptr = e.call(NODE_GET_PROPERTY, &args![node, 3u32]).ptr();
        let mut texture = Ptr::NULL;
        let entry = fn_0063cfa0(e, climate, 0);
        if !entry.is_null() && e.call(TEXTURE_ENTRY_VALID, &args![entry]).u32() != 0 {
            e.with_stack(4, |e, holder| {
                e.call(NI_POINTER_FROM_RAW, &args![holder, 0u32]);
                let prefix = e.vcall(entry.addr(), 0x18, &[]).u32();
                e.call(STRING_ASSIGN, &args![text, prefix]);
                let name = e.call(TEXTURE_ENTRY_NAME, &args![entry]).u32();
                e.call(STRING_APPEND, &args![text, name]);
                texture = load_texture(e, text, holder, property);
                e.call(NI_POINTER_DESTRUCTOR, &args![holder]);
            });
        }
        let found = (!texture.is_null()) as u32;
        let node: Ptr = e.call(HOLDER_NODE, &args![sun]).ptr();
        e.call(NODE_SET_FLAG_BIT_20, &args![node, found]);

        let node = fn_0063d040(e, sun);
        let property: Ptr = e.call(NODE_GET_PROPERTY, &args![node, 3u32]).ptr();
        let entry = fn_0063cfa0(e, climate, 1);
        if !entry.is_null() && e.call(TEXTURE_ENTRY_VALID, &args![entry]).u32() != 0 {
            let hdr_off = e.global::<u8>(INITIAL_FLAG) == 0;
            let mut use_default = false;
            if hdr_off {
                let name = e.call(TEXTURE_ENTRY_NAME, &args![entry]).u32();
                use_default = e.call(STRING_COMPARE, &args![name, SUN_GLARE_NAME]).u32() == 0;
            }
            if use_default {
                e.call(STRING_ASSIGN, &args![text, SUN_GLARE_NON_HDR_PATH]);
            } else {
                let prefix = e.vcall(entry.addr(), 0x18, &[]).u32();
                e.call(STRING_ASSIGN, &args![text, prefix]);
                let name = e.call(TEXTURE_ENTRY_NAME, &args![entry]).u32();
                e.call(STRING_APPEND, &args![text, name]);
            }
            e.with_stack(4, |e, holder| {
                e.call(NI_POINTER_FROM_RAW, &args![holder, 0u32]);
                texture = load_texture(e, text, holder, property);
                e.call(NI_POINTER_DESTRUCTOR, &args![holder]);
            });
        }
        let found = (!texture.is_null()) as u32;
        let node: Ptr = e.call(SUN_GET_NODE, &args![sun]).ptr();
        e.call(NODE_SET_FLAG_BIT_20, &args![node, found]);
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

/// Loads the texture named by the string at `text` into the `NiPointer` at
/// `holder` (`BSShaderManager::GetTexture`-style loader `00b55840`) and, when
/// one was found, sets it in `property` (`006348e0`). Returns the texture.
fn load_texture(e: &mut Engine, text: Ptr, holder: Ptr, property: Ptr) -> Ptr {
    let path = ni_pointer_get(e, text.addr());
    e.call(TEXTURE_LOAD, &args![path, 1u32, holder, 1u32, 0u32]);
    let texture = ni_pointer_get(e, holder.addr());
    if !texture.is_null() {
        e.call(PROPERTY_SET_TEXTURE, &args![property, texture]);
    }
    texture
}

/// How `fn_0063ca20` builds one moon: its name and the six settings
/// (a word setting and five `float` settings, in the order the game reads
/// them).
struct MoonSettings {
    name: u32,
    word_setting: u32,
    float_settings: [u32; 5],
}

const MASSER_MOON: MoonSettings = MoonSettings {
    name: 0x0104_eeb0,
    word_setting: 0x011c_cc18,
    float_settings: [
        0x011c_cc54,
        0x011c_cbfc,
        0x011c_cba4,
        0x011c_cc94,
        0x011c_ccb8,
    ],
};

const SECUNDA_MOON: MoonSettings = MoonSettings {
    name: 0x0104_eea8,
    word_setting: 0x011c_ccec,
    float_settings: [
        0x011c_cc7c,
        0x011c_cbcc,
        0x011c_cc64,
        0x011c_cbe8,
        0x011c_cb98,
    ],
};

/// Creates the moon stored in `field` (when `wanted` and there is none: a
/// `Moon` of 0x7c bytes built with the settings, then initialized through
/// virtual slot `0x10` with the moons root and its name) or deletes it (when
/// not `wanted`).
fn update_moon(
    e: &mut Engine,
    this: Ptr<Sky>,
    field: Field<Sky, Ptr>,
    wanted: bool,
    settings: MoonSettings,
) {
    if wanted && e.get(this, field).is_null() {
        let block: Ptr = e.call(OPERATOR_NEW, &args![0x7cu32]).ptr();
        let moon = if block.is_null() {
            Ptr::NULL
        } else {
            let word_slot: Ptr = e
                .call(SETTING_WORD_VALUE, &args![settings.word_setting])
                .ptr();
            let word = e.mem.u32(word_slot.addr());
            let mut values = [0.0f32; 5];
            for (value, setting) in values.iter_mut().zip(settings.float_settings) {
                let slot: Ptr = e.call(SETTING_FLOAT_VALUE, &args![setting]).ptr();
                *value = e.mem.f32(slot.addr());
            }
            e.call(
                MOON_CONSTRUCT,
                &args![
                    block,
                    settings.name,
                    values[4],
                    values[3],
                    values[2],
                    values[1],
                    values[0],
                    word
                ],
            )
            .ptr()
        };
        e.set(this, field, moon);
        let moons_root = ni_pointer_get(e, this.addr() + 8);
        e.vcall(moon.addr(), 0x10, &args![moons_root, settings.name]);
    } else if !wanted {
        let moon = e.get(this, field);
        delete_component(e, moon);
        e.set(this, field, Ptr::NULL);
    }
}

// Translated from 0063cfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Climate accessor (by its caller): the address of entry `index` (12 bytes
/// each from `+0x38`) when `index < 2`, else null.
pub fn fn_0063cfa0(_e: &mut Engine, this: Ptr, index: i32) -> Ptr {
    if index < 2 {
        Ptr::new(
            this.addr()
                .wrapping_add((index as u32).wrapping_mul(12))
                .wrapping_add(0x38),
        )
    } else {
        Ptr::NULL
    }
}

// Translated from 0063cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Climate method (by its caller): calls virtual slot `0x14` of the object
/// embedded at `+0x18` and returns its result.
pub fn fn_0063cfe0(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr() + 0x18, 0x14, &[]).u32()
}

// Translated from 0063d000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Climate accessor: whether bit `0x80` of the byte at `+0x55` is set (the
/// climate has Masser).
pub fn fn_0063d000(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x55) & 0x80 != 0
}

// Translated from 0063d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Climate accessor: whether bit `0x40` of the byte at `+0x55` is set (the
/// climate has Secunda).
pub fn fn_0063d020(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x55) & 0x40 != 0
}

// Translated from 0063d040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sun` accessor (by its caller): the `NiPointer` stored at `+0x14`.
pub fn fn_0063d040(e: &mut Engine, this: Ptr) -> Ptr {
    ni_pointer_get(e, this.addr() + 0x14)
}

// ---- the weather, sound, texture, save and image-space functions ----

/// The item of a `BSSimpleList` node (`LIST_NODE_ITEM` returns its address).
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let slot: Ptr = e.call(LIST_NODE_ITEM, &args![node]).ptr();
    e.mem.u32(slot.addr())
}

/// The next node of a `BSSimpleList` node (zero at the end).
fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NODE_NEXT, &args![node]).u32()
}

/// Inserts `item` into the list at `list` (`LIST_INSERT` takes the address of
/// a local holding the item).
fn list_insert(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_INSERT, &args![list, slot]);
    });
}

/// Clears the clouds' transition textures when the sky has clouds.
fn clear_cloud_transitions(e: &mut Engine, this: Ptr<Sky>) {
    let clouds = e.get(this, Sky::pClouds);
    if !clouds.is_null() {
        e.call(CLOUDS_CLEAR_TRANS_TEXTURES, &args![clouds]);
    }
}

/// Flushes the precipitation when the sky has one.
fn flush_precipitation(e: &mut Engine, this: Ptr<Sky>) {
    let precipitation = e.get(this, Sky::pPrecip);
    if !precipitation.is_null() {
        e.call(PRECIPITATION_FLUSH_ALL, &args![precipitation]);
    }
}

// Translated from 0063d060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::ResetWeather` (Xbox PDB): sets flag 1, forgets the override, last and
/// current weather, resets the acceleration (`0063e860(false)`), clears the
/// clouds' transition textures and flushes the precipitation (when they
/// exist) and recomputes the image-space values.
pub fn sky_reset_weather(e: &mut Engine, this: Ptr<Sky>) {
    let flags = e.get(this, Sky::uiFlags);
    e.set(this, Sky::uiFlags, flags | 1);
    e.set(this, Sky::pOverrideWeather, Ptr::NULL);
    e.set(this, Sky::pLastWeather, Ptr::NULL);
    e.set(this, Sky::pCurrentWeather, Ptr::NULL);
    fn_0063e860(e, this, false);
    clear_cloud_transitions(e, this);
    flush_precipitation(e, this);
    sky_update_hdr_values(e, this);
}

// Translated from 0063d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::ForceWeather` (Xbox PDB): makes `weather` the current weather at
/// once. With `override_weather` clear it becomes the default weather and the
/// override is cleared; with it set the weather becomes the override and the
/// default is cleared. The last weather is cleared, the acceleration reset
/// (`0063e860(false)`), the blend is 1.0 and the last update hour is the
/// current hour, the player gets its `0093a7a0(0)` call, flag 2 is set and
/// flag 1 cleared, the image-space values are recomputed, and the clouds'
/// transition textures and the precipitation are cleared.
pub fn sky_force_weather(e: &mut Engine, this: Ptr<Sky>, weather: Ptr, override_weather: bool) {
    if override_weather {
        e.set(this, Sky::pOverrideWeather, weather);
        e.set(this, Sky::pCurrentWeather, weather);
        e.set(this, Sky::pDefaultWeather, Ptr::NULL);
    } else {
        e.set(this, Sky::pDefaultWeather, weather);
        e.set(this, Sky::pCurrentWeather, weather);
        e.set(this, Sky::pOverrideWeather, Ptr::NULL);
    }
    e.set(this, Sky::pLastWeather, Ptr::NULL);
    fn_0063e860(e, this, false);
    e.set(this, Sky::fCurrentWeatherPct, 1.0);
    let hour = e.get(this, Sky::fCurrentGameHour);
    e.set(this, Sky::fLastWeatherUpdate, hour);
    let player: u32 = e.global(PLAYER_CHARACTER);
    e.call(PLAYER_METHOD_0093A7A0, &args![player, 0u32]);
    let flags = e.get(this, Sky::uiFlags);
    e.set(this, Sky::uiFlags, flags | 2);
    let flags = e.get(this, Sky::uiFlags);
    e.set(this, Sky::uiFlags, flags & !1);
    sky_update_hdr_values(e, this);
    clear_cloud_transitions(e, this);
    flush_precipitation(e, this);
}

/// Hours between the last weather update and now: the hour difference, plus 24
/// when the last update hour is later than the current hour (a day wrapped).
/// The `float` compare keeps its NaN behaviour (no wrap).
fn hours_since_last_update(e: &mut Engine, this: Ptr<Sky>) -> f64 {
    let hour = e.get(this, Sky::fCurrentGameHour);
    let last = e.get(this, Sky::fLastWeatherUpdate);
    let wrap = if last > hour { 24.0 } else { 0.0 };
    (wrap + hour as f64) - last as f64
}

// Translated from 0063d1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Weather transition update (a `Sky` method the map leaves unnamed; it is
/// called from `Sky::Update`). Does nothing without a climate. When flag 1 is
/// clear and there is a default weather it only continues when the time since
/// the last update exceeds a limit from the climate's byte (`(255 - byte) *
/// 22 / 255 + 1` hours) while there is no last weather. It then asks the
/// climate for a weather (`0063d5c0`; the form with id `0x15e` cast to a
/// weather when the climate has none) and runs `TESRegion::UpdateWeather` on
/// every region of the region list. Then it chooses the weather to show: the
/// default weather (replaced by the player's area weather outside mode 2)
/// when there is no override and the world flag does not apply, else the
/// override (so with the world flag set and a current weather only an
/// override can change the weather). A weather that differs from the current one
/// becomes current (the current one becomes the last, or the last is cleared
/// and the precipitation flushed in fast travel or mode 1) and flag 1 is set;
/// otherwise flag 1 is cleared. Then `fCurrentWeatherPct` is 0 without a
/// weather, 1 without a last weather, else the hours since the last update
/// divided by the transition length (`WEATHER_BYTE_FRACTION`, index 3, with the
/// two transition settings); with flag 8 it is re-scaled by the acceleration
/// setting. A blend above 1 ends the transition. It ends with
/// `Sky::UpdateHDRValues`.
pub fn fn_0063d1d0(e: &mut Engine, this: Ptr<Sky>) {
    let climate = e.get(this, Sky::pCurrentClimate);
    if climate.is_null() {
        return;
    }
    let mut refresh = true;
    if e.get(this, Sky::uiFlags) & 1 == 0 && !e.get(this, Sky::pDefaultWeather).is_null() {
        let elapsed = hours_since_last_update(e, this);
        let byte = e.call(CLIMATE_UPDATE_BYTE, &args![climate]).u8() as u32;
        let scale: f64 = e.global(RECIPROCAL_255);
        let limit = ((0xff - byte) * 0x16) as i32 as f64 * scale + 1.0;
        // Both the comparison and the last-weather test must hold.
        refresh = limit < elapsed && e.get(this, Sky::pLastWeather).is_null();
    }
    if refresh {
        let climate = e.get(this, Sky::pCurrentClimate);
        let weather = fn_0063d5c0(e, climate);
        e.set(this, Sky::pDefaultWeather, weather);
        if e.get(this, Sky::pDefaultWeather).is_null() {
            let form: Ptr = e.call(LOOKUP_FORM, &args![DEFAULT_WEATHER_FORM_ID]).ptr();
            let weather = e
                .call(
                    DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        TYPE_DESCRIPTOR_FORM,
                        TYPE_DESCRIPTOR_WEATHER,
                        0u32
                    ],
                )
                .ptr();
            e.set(this, Sky::pDefaultWeather, weather);
        }
        let holder: u32 = e.global(REGION_HOLDER);
        let owner: Ptr = e.call(REGION_LIST_OWNER, &args![holder]).ptr();
        let mut node = if owner.is_null() { 0 } else { owner.addr() + 4 };
        while node != 0 {
            if list_item(e, node) == 0 {
                break;
            }
            let region = list_item(e, node);
            e.call(REGION_UPDATE_WEATHER, &args![region]);
            node = list_next(e, node);
        }
    }

    // Which weather to show.
    let mut use_default = e.get(this, Sky::pOverrideWeather).is_null();
    if !e.get(this, Sky::pCurrentWeather).is_null() {
        let state: u32 = e.global(WORLD_STATE_POINTER);
        if e.call(WORLD_STATE_FLAG, &args![state]).bool() {
            use_default = false;
        }
    }
    let wanted: Ptr = if use_default {
        let mut chosen = e.get(this, Sky::pDefaultWeather);
        let player: u32 = e.global(PLAYER_CHARACTER);
        let owner: u32 = e.call(PLAYER_WEATHER_OWNER, &args![player]).u32();
        if owner != 0
            && e.call(WEATHER_OF_OWNER, &args![owner]).u32() != 0
            && e.get(this, Sky::eMode) != 2
        {
            chosen = e.call(WEATHER_OF_OWNER, &args![owner]).ptr();
        }
        chosen
    } else {
        e.get(this, Sky::pOverrideWeather)
    };

    let unchanged = wanted.is_null()
        || (!e.get(this, Sky::pLastWeather).is_null() && e.get(this, Sky::uiFlags) & 0x10 == 0)
        || wanted == e.get(this, Sky::pCurrentWeather);
    if unchanged {
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags & !1);
    } else {
        if e.get(this, Sky::uiFlags) & 0x10 == 0 && e.get(this, Sky::eMode) != 1 {
            let current = e.get(this, Sky::pCurrentWeather);
            e.set(this, Sky::pLastWeather, current);
        } else {
            e.set(this, Sky::pLastWeather, Ptr::NULL);
            flush_precipitation(e, this);
        }
        e.set(this, Sky::pCurrentWeather, wanted);
        let state: u32 = e.global(WORLD_STATE_POINTER);
        if !e.call(WORLD_STATE_FLAG, &args![state]).bool() {
            let hour = e.get(this, Sky::fCurrentGameHour);
            e.set(this, Sky::fLastWeatherUpdate, hour);
        }
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags | 1);
    }

    // The blend between the last and the current weather.
    if e.get(this, Sky::pCurrentWeather).is_null() {
        e.set(this, Sky::fCurrentWeatherPct, 0.0);
    } else if e.get(this, Sky::pLastWeather).is_null() {
        e.set(this, Sky::fCurrentWeatherPct, 1.0);
    } else {
        let elapsed = hours_since_last_update(e, this);
        let low_slot: Ptr = e
            .call(SETTING_FLOAT_VALUE, &args![TRANSITION_LOW_SETTING])
            .ptr();
        let low = e.mem.f32(low_slot.addr());
        let high_slot: Ptr = e
            .call(SETTING_FLOAT_VALUE, &args![TRANSITION_HIGH_SETTING])
            .ptr();
        let high = e.mem.f32(high_slot.addr());
        let current = e.get(this, Sky::pCurrentWeather);
        let length = e
            .call(WEATHER_BYTE_FRACTION, &args![current, 3u32, high, low])
            .f32();
        e.set(
            this,
            Sky::fCurrentWeatherPct,
            (elapsed / length as f64) as f32,
        );
    }
    if e.get(this, Sky::uiFlags) & 8 != 0 {
        let pct = e.get(this, Sky::fCurrentWeatherPct);
        let begin = e.get(this, Sky::fAccelBeginPct);
        let diff = pct as f64 - begin as f64;
        let slot: Ptr = e
            .call(SETTING_FLOAT_VALUE, &args![TRANSITION_ACCELERATION_SETTING])
            .ptr();
        let acceleration = e.mem.f32(slot.addr());
        let begin = e.get(this, Sky::fAccelBeginPct);
        e.set(
            this,
            Sky::fCurrentWeatherPct,
            ((acceleration as f64 + 1.0) * diff + begin as f64) as f32,
        );
    }
    let one: f64 = e.global(DOUBLE_ONE);
    if e.get(this, Sky::fCurrentWeatherPct) as f64 > one {
        e.set(this, Sky::pLastWeather, Ptr::NULL);
        e.set(this, Sky::fCurrentWeatherPct, 1.0);
        fn_0063e860(e, this, false);
    }
    sky_update_hdr_values(e, this);
}

// Translated from 0063d5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESClimate` method (by its caller): forwards the climate's weather list
/// (embedded at `+0x30`) to `005827d0`, which picks a weather; returns it.
pub fn fn_0063d5c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CLIMATE_PICK_WEATHER, &args![this.addr() + 0x30])
        .ptr()
}

/// Rebuilds the sky's sound list for `fn_0063d5e0` (the sounds were marked by
/// `fn_0063dd90`): a new list receives the sounds that are not marked, the
/// marked ones are released and deleted (the type-3 count goes down), the old
/// list is cleared and deleted, and the new list becomes `pSkySoundList`. The
/// local list the game builds and then walks is always empty, so the walk over
/// it stops at once; it is kept as the game has it. Returns the new list.
fn rebuild_sound_list(e: &mut Engine, this: Ptr<Sky>) -> u32 {
    let old_head = e.get(this, Sky::pSkySoundList).addr();
    let block: Ptr = e.call(OPERATOR_NEW, &args![8u32]).ptr();
    let new_list = if block.is_null() {
        0
    } else {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    };
    e.with_stack(8, |e, scratch| {
        e.call(LIST_CONSTRUCT, &args![scratch]);
        let mut node = old_head;
        while node != 0 {
            let sound = list_item(e, node);
            if sound == 0 {
                break;
            }
            if fn_0063dd70(e, Ptr::new(sound)) {
                if e.mem.u32(sound + 0x10) == 3 {
                    let count: u8 = e.global(SOUND_TYPE_3_COUNT);
                    e.set_global(SOUND_TYPE_3_COUNT, count.wrapping_sub(1));
                }
                e.call(SOUND_HANDLE_RELEASE, &args![sound]);
                fn_0063a3c0(e, Ptr::new(sound), 1);
            } else {
                list_insert(e, new_list, sound);
            }
            node = list_next(e, node);
        }

        let mut walk = scratch.addr();
        while walk != 0 {
            let sound = list_item(e, walk);
            if sound == 0 {
                break;
            }
            walk = list_next(e, walk);
            if e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
                let mut found = false;
                let mut other = new_list;
                while !found && other != 0 {
                    let candidate = list_item(e, other);
                    if candidate == 0 {
                        break;
                    }
                    if e.mem.u32(candidate + 0x14) == e.mem.u32(sound + 0x14) {
                        found = true;
                    }
                    other = list_next(e, other);
                }
                if !found {
                    e.call(SOUND_HANDLE_RELEASE, &args![sound]);
                    fn_0063a3c0(e, Ptr::new(sound), 1);
                }
            } else {
                fn_0063a3c0(e, Ptr::new(sound), 1);
            }
        }
        e.call(LIST_CLEAR, &args![scratch]);
        let list = e.get(this, Sky::pSkySoundList);
        e.call(LIST_CLEAR, &args![list]);
        let list = e.get(this, Sky::pSkySoundList);
        if !list.is_null() {
            e.call(LIST_SCALAR_DELETING_DESTRUCTOR, &args![list, 1u32]);
        }
        e.set(this, Sky::pSkySoundList, Ptr::new(new_list));
        e.set_global(SOUNDS_DIRTY, 0u8);
        e.call(LIST_DESTRUCT, &args![scratch]);
    });
    new_list
}

/// The weather the sound of `sound` is not for: the last weather when the
/// sound belongs to the current weather, else the current one.
fn other_weather(e: &mut Engine, this: Ptr<Sky>, sound: Ptr<SkySound>) -> Ptr {
    if e.get(sound, SkySound::pWeather) == e.get(this, Sky::pCurrentWeather) {
        e.get(this, Sky::pLastWeather)
    } else {
        e.get(this, Sky::pCurrentWeather)
    }
}

/// `(pct - start) / (1 - start)` of two `float`s, rounded to `float`: how far
/// past `start` the blend is.
fn rise_fraction(pct: f32, start: f32) -> f32 {
    ((pct as f64 - start as f64) / (1.0 - start as f64)) as f32
}

/// `1 - pct / end` of two `float`s, rounded to `float`: how far the blend is
/// from `end` when it falls.
fn fall_fraction(pct: f32, end: f32) -> f32 {
    (1.0 - pct as f64 / end as f64) as f32
}

// Translated from 0063d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sky's weather sounds update (a `Sky` method the map leaves unnamed;
/// called from `Sky::Update`). When the dirty byte `011ccd04` is set the sound
/// list is rebuilt without the marked sounds. When flag 1 or 2 is set the
/// current weather's sounds are added (`0063ddb0`). Then every valid sound is
/// handled: in modes 2 and 3 a sound of the current or last weather is
/// played at a volume that follows the blend `fCurrentWeatherPct` (a
/// type-3 sound starts at random, once in `byte * count` ticks, through
/// `WEATHER_BYTE_FRACTION` indices 8 and 9 and records the flash time in
/// `fFlash`; the others fade through the indices 6 and 7 or by the blend
/// itself); any other sound is stopped unless the other weather has the same
/// sound, and marked for removal. The compiler's exception frame is not
/// translated. A type-3 sound whose divisor `byte * count` is zero makes the
/// game fault on the division; here it panics.
pub fn fn_0063d5e0(e: &mut Engine, this: Ptr<Sky>) {
    let mut node = e.get(this, Sky::pSkySoundList).addr();
    if e.global::<u8>(SOUNDS_DIRTY) != 0 {
        node = rebuild_sound_list(e, this);
    }
    if e.call(SOUNDS_APPLY, &args![this]).bool() {
        let weather = e.get(this, Sky::pCurrentWeather);
        fn_0063ddb0(e, this, weather);
    }
    while node != 0 {
        let sound_address = list_item(e, node);
        if sound_address == 0 {
            break;
        }
        let sound: Ptr<SkySound> = Ptr::new(sound_address);
        if !e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
            node = list_next(e, node);
            continue;
        }
        let mode = e.get(this, Sky::eMode);
        let sound_weather = e.get(sound, SkySound::pWeather);
        if (mode == 3 || mode == 2)
            && (sound_weather == e.get(this, Sky::pCurrentWeather)
                || sound_weather == e.get(this, Sky::pLastWeather))
        {
            if e.get(sound, SkySound::eSoundType) == 3 {
                random_weather_sound(e, this, sound);
            } else {
                blended_weather_sound(e, this, sound);
            }
        } else {
            if e.call(SOUND_HANDLE_IS_PLAYING, &args![sound]).bool() {
                let other = other_weather(e, this, sound);
                let form_id = e.get(sound, SkySound::uiFormID);
                if !fn_0063e060(e, this, form_id, other) {
                    e.call(SOUND_HANDLE_STOP, &args![sound]);
                }
            }
            fn_0063dd90(e, sound);
            e.set_global(SOUNDS_DIRTY, 1u8);
        }
        node = list_next(e, node);
    }
}

/// The type-3 (random, thunder-like) sound of `fn_0063d5e0`: when it is not
/// playing and `fn_00639c90(sound, 0x1e)` accepts, it starts with probability
/// `1 / (weather byte at +0xea * count)` per call, at a volume from the blend,
/// and records the flash volume and time.
fn random_weather_sound(e: &mut Engine, this: Ptr<Sky>, sound: Ptr<SkySound>) {
    if e.call(SOUND_HANDLE_IS_PLAYING, &args![sound]).bool() || !fn_00639c90(e, sound.cast(), 0x1e)
    {
        return;
    }
    let random = e.call(RANDOM_NUMBER, &[]).u32();
    let weather = e.get(sound, SkySound::pWeather);
    let byte = fn_0063dd50(e, weather, 10) as u32;
    let count = e.global::<u8>(SOUND_TYPE_3_COUNT) as u32;
    if random % (byte * count) != 0 {
        return;
    }
    let pct = e.get(this, Sky::fCurrentWeatherPct);
    let current = e.get(this, Sky::pCurrentWeather);
    let volume = if e.get(sound, SkySound::pWeather) == current {
        let high: f32 = e.global(FRACTION_HIGH_SOUND);
        let start = e
            .call(WEATHER_BYTE_FRACTION, &args![current, 8u32, high, 0.0f32])
            .f32();
        if start > pct {
            0.0
        } else {
            rise_fraction(pct, start)
        }
    } else {
        let last = e.get(this, Sky::pLastWeather);
        let low: f32 = e.global(FRACTION_LOW);
        let end = e
            .call(WEATHER_BYTE_FRACTION, &args![last, 9u32, 1.0f32, low])
            .f32();
        if end > pct {
            fall_fraction(pct, end)
        } else {
            0.0
        }
    };
    let zero: f64 = e.global(DOUBLE_ZERO);
    if volume as f64 != zero {
        sky_sound_play(e, sound, volume);
        e.set(this, Sky::fFlash, volume);
        let ticks = e.call(TICK_COUNT_GET, &args![TICK_COUNTER]).u32();
        e.set(this, Sky::uiFlashTime, ticks);
    }
}

/// The other sound types of `fn_0063d5e0`: computes the volume of the sound
/// (`volume`) and of the same sound in the other weather (`other_volume`) from
/// the blend, and plays the sound at `volume` unless the other weather has
/// the sound and its volume is higher.
fn blended_weather_sound(e: &mut Engine, this: Ptr<Sky>, sound: Ptr<SkySound>) {
    let pct = e.get(this, Sky::fCurrentWeatherPct);
    let current = e.get(this, Sky::pCurrentWeather);
    let own = e.get(sound, SkySound::pWeather) == current;
    let (volume, other_volume);
    if e.get(sound, SkySound::eSoundType) == 1 {
        let high: f32 = e.global(FRACTION_HIGH_SOUND);
        let rise = e
            .call(WEATHER_BYTE_FRACTION, &args![current, 6u32, high, 0.0f32])
            .f32();
        let last = e.get(this, Sky::pLastWeather);
        let fall = if last.is_null() {
            0.0
        } else {
            let low: f32 = e.global(FRACTION_LOW);
            e.call(WEATHER_BYTE_FRACTION, &args![last, 7u32, 1.0f32, low])
                .f32()
        };
        let rising = if rise > pct {
            0.0
        } else {
            rise_fraction(pct, rise)
        };
        let falling = if fall > pct {
            fall_fraction(pct, fall)
        } else {
            0.0
        };
        if own {
            volume = rising;
            other_volume = falling;
        } else {
            volume = falling;
            other_volume = rising;
        }
    } else if own {
        volume = pct;
        other_volume = (1.0 - pct as f64) as f32;
    } else {
        volume = (1.0 - pct as f64) as f32;
        other_volume = pct;
    }
    let other = other_weather(e, this, sound);
    let form_id = e.get(sound, SkySound::uiFormID);
    if !fn_0063e060(e, this, form_id, other) || other_volume <= volume {
        sky_sound_play(e, sound, volume);
    }
}

// Translated from 0063dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather` method (by its caller): the byte at `+0xe0 + index`.
pub fn fn_0063dd50(e: &mut Engine, this: Ptr, index: i32) -> u8 {
    e.mem
        .u8(this.addr().wrapping_add(index as u32).wrapping_add(0xe0))
}

// Translated from 0063dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkySound` method (by its caller): whether bit 31 of `uiData` is set (the
/// sound is marked for removal).
pub fn fn_0063dd70(e: &mut Engine, this: Ptr<SkySound>) -> bool {
    e.get(this, SkySound::uiData) & 0x8000_0000 != 0
}

// Translated from 0063dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkySound` method (by its caller): sets bit 31 of `uiData` (marks the
/// sound for removal).
pub fn fn_0063dd90(e: &mut Engine, this: Ptr<SkySound>) {
    let data = e.get(this, SkySound::uiData);
    e.set(this, SkySound::uiData, data | 0x8000_0000);
}

// Translated from 0063ddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the sounds of `weather` (the list at `+0x1f8`, items `{ form id, sound
/// type }`) to the sky's sound list unless a sound with the same form id,
/// weather and type is already there. Each sound gets its handle (obtained
/// with `BSAudio::GetSoundHandleByNumericID` and flags `0x21`, plus `0x10`
/// for a type other than 3; skipped for a type-1 sound while the precipitation
/// setting is off); a sound that is already in the list only releases the
/// handle, a new one becomes a `SkySound` (`00639bf0`) inserted in the list
/// (the type-3 count goes up for type 3). The compiler's exception frame is
/// not translated.
pub fn fn_0063ddb0(e: &mut Engine, this: Ptr<Sky>, weather: Ptr) {
    if weather.is_null() {
        return;
    }
    let mut node = fn_0063dff0(e, weather).addr();
    while node != 0 {
        let description = list_item(e, node);
        if description == 0 {
            break;
        }
        e.with_stack(12, |e, handle| {
            e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
            let setting: Ptr = e
                .call(SETTING_BYTE_VALUE, &args![SETTING_PRECIPITATION])
                .ptr();
            let sound_type = e.mem.u32(description + 4);
            if e.mem.u8(setting.addr()) != 0 || sound_type != 1 {
                let mut flags = 0x21u32;
                if sound_type != 3 {
                    flags |= 0x10;
                }
                e.with_stack(12, |e, request| {
                    let audio: u32 = e.call(AUDIO_INSTANCE, &[]).u32();
                    let id = e.mem.u32(description);
                    let filled = e
                        .call(AUDIO_GET_SOUND_HANDLE, &args![audio, request, id, flags])
                        .u32();
                    e.call(SOUND_HANDLE_ASSIGN, &args![handle, filled]);
                    e.call(SOUND_HANDLE_DESTRUCT, &args![request]);
                });
            }
            let form_id = e.mem.u32(description);
            let mut found = false;
            let mut other = e.get(this, Sky::pSkySoundList).addr();
            while !found && other != 0 {
                let existing = list_item(e, other);
                if existing == 0 {
                    break;
                }
                found = e.mem.u32(existing + 0x14) == form_id
                    && e.mem.u32(existing + 0xc) == weather.addr()
                    && e.mem.u32(existing + 0x10) == sound_type;
                other = list_next(e, other);
            }
            if !found {
                let block: Ptr = e.call(OPERATOR_NEW, &args![0x1cu32]).ptr();
                let sound: Ptr<SkySound> = if block.is_null() {
                    Ptr::NULL
                } else {
                    e.with_stack(12, |e, copy| {
                        e.call(SOUND_HANDLE_ASSIGN, &args![copy, handle]);
                        let id = e.mem.u32(copy.addr());
                        let flag = e.mem.u32(copy.addr() + 4);
                        let state = e.mem.u32(copy.addr() + 8);
                        fn_00639bf0(
                            e,
                            block.cast(),
                            id,
                            flag,
                            state,
                            weather,
                            sound_type,
                            form_id,
                        )
                    })
                };
                let list = e.get(this, Sky::pSkySoundList).addr();
                list_insert(e, list, sound.addr());
                if sound_type == 3 {
                    let count: u8 = e.global(SOUND_TYPE_3_COUNT);
                    e.set_global(SOUND_TYPE_3_COUNT, count.wrapping_add(1));
                }
            } else {
                e.call(SOUND_HANDLE_RELEASE, &args![handle]);
            }
            node = list_next(e, node);
            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
        });
    }
}

// Translated from 0063dff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather` method (by its caller): the address of the weather sound list
/// embedded at `+0x1f8`.
pub fn fn_0063dff0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x1f8))
}

// Translated from 0063e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stops every sound of the sky's sound list (`BSSoundHandle::Stop`), up to
/// the first empty node.
pub fn fn_0063e010(e: &mut Engine, this: Ptr<Sky>) {
    let mut node = e.get(this, Sky::pSkySoundList).addr();
    while node != 0 {
        if list_item(e, node) == 0 {
            break;
        }
        let sound = list_item(e, node);
        e.call(SOUND_HANDLE_STOP, &args![sound]);
        node = list_next(e, node);
    }
}

// Translated from 0063e060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the sky's sound list has a sound with form id `form_id` that
/// belongs to `weather`; the walk stops at the first empty node.
pub fn fn_0063e060(e: &mut Engine, this: Ptr<Sky>, form_id: u32, weather: Ptr) -> bool {
    let mut node = e.get(this, Sky::pSkySoundList).addr();
    while node != 0 {
        if list_item(e, node) == 0 {
            return false;
        }
        // The game reads the node's item again for each test.
        let sound: Ptr<SkySound> = Ptr::new(list_item(e, node));
        if e.get(sound, SkySound::uiFormID) == form_id {
            let sound: Ptr<SkySound> = Ptr::new(list_item(e, node));
            if e.get(sound, SkySound::pWeather) == weather {
                return true;
            }
        }
        node = list_next(e, node);
    }
    false
}

// Translated from 0063e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkySound::Play` (Xbox PDB): does nothing in menu mode or while the game
/// loading menu is open. Otherwise, a sound whose handle is no longer valid
/// gets a new handle (`GetSoundHandleByNumericID` with flags `0x20`); a type-3
/// sound that is not playing starts (`Play(0)`), another type starts (`Play(1)`)
/// when `fn_00639c90(1000)` accepts and it is not playing; then, when the
/// handle's volume differs from `volume` by more than 0.01, the volume is set.
/// The compiler's exception frame is not translated.
pub fn sky_sound_play(e: &mut Engine, this: Ptr<SkySound>, volume: f32) {
    if e.call(IS_IN_MENU_MODE, &[]).bool() || e.call(IS_LOADING_MENU_OPEN, &[]).bool() {
        return;
    }
    if !e.call(SOUND_HANDLE_IS_VALID, &args![this]).bool() {
        let id = ni_pointer_get(e, this.addr()).addr();
        e.with_stack(12, |e, request| {
            let audio: u32 = e.call(AUDIO_INSTANCE, &[]).u32();
            e.call(AUDIO_GET_SOUND_HANDLE, &args![audio, request, id, 0x20u32]);
            e.call(SOUND_HANDLE_ASSIGN, &args![this, request]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![request]);
        });
    }
    if e.get(this, SkySound::eSoundType) == 3 {
        if !e.call(SOUND_HANDLE_IS_PLAYING, &args![this]).bool() {
            e.call(SOUND_HANDLE_PLAY, &args![this, 0u32]);
        }
    } else if fn_00639c90(e, this.cast(), 1000)
        && !e.call(SOUND_HANDLE_IS_PLAYING, &args![this]).bool()
    {
        e.call(SOUND_HANDLE_PLAY, &args![this, 1u32]);
    }
    let current = e.call(SOUND_HANDLE_GET_VOLUME, &args![this]).f32();
    let difference = (current as f64 - volume as f64) as f32;
    let magnitude = e.call(ABS_FLOAT, &args![difference]).f64();
    let epsilon: f64 = e.global(VOLUME_EPSILON);
    if magnitude > epsilon {
        e.call(SOUND_HANDLE_SET_VOLUME, &args![this, volume]);
    }
}

// Translated from 0063e210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::UnloadAllTextures` (Xbox PDB): removes the clouds' textures and
/// transition textures, unloads both moons and the sun, and releases the stars'
/// node (`0063e290`); each only when it exists.
pub fn sky_unload_all_textures(e: &mut Engine, this: Ptr<Sky>) {
    let clouds = e.get(this, Sky::pClouds);
    if !clouds.is_null() {
        e.call(CLOUDS_REMOVE_TEXTURES, &args![clouds]);
        let clouds = e.get(this, Sky::pClouds);
        e.call(CLOUDS_CLEAR_TRANS_TEXTURES, &args![clouds]);
    }
    for field in [Sky::pMasser, Sky::pSecunda] {
        let moon = e.get(this, field);
        if !moon.is_null() {
            let moon = e.get(this, field);
            e.call(MOON_UNLOAD_TEXTURES, &args![moon]);
        }
    }
    let sun = e.get(this, Sky::pSun);
    if !sun.is_null() {
        e.call(SUN_UNLOAD_TEXTURES, &args![sun]);
    }
    let stars = e.get(this, Sky::pStars);
    if !stars.is_null() {
        fn_0063e290(e, stars);
    }
}

// Translated from 0063e290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Stars` method (by its caller `Sky::UnloadAllTextures`): when the
/// `NiPointer` at `+4` holds an object, calls its virtual slot `0xe8` with the
/// object of the `NiPointer` at `+8`; then clears the `NiPointer` at `+8`.
pub fn fn_0063e290(e: &mut Engine, this: Ptr) {
    if !ni_pointer_get(e, this.addr() + 4).is_null() {
        let object = ni_pointer_get(e, this.addr() + 4);
        let argument = ni_pointer_get(e, this.addr() + 8);
        e.vcall(object.addr(), 0xe8, &args![argument]);
    }
    ni_pointer_assign(e, this.addr() + 8, Ptr::NULL);
}

/// One layer of the current weather's cloud textures for `fn_0063e2f0`: when
/// the weather has a valid texture entry for `layer`, formats its path
/// (`"%s%s"`: the entry's virtual slot `0x18` result and its name) and, when
/// the file exists and the layer's node has the properties, loads the texture
/// into the layer's type-5 property's texture slot and the type-3 property.
fn load_cloud_layer(e: &mut Engine, this: Ptr<Sky>, layer: i32) {
    let weather = e.get(this, Sky::pCurrentWeather);
    if e.call(WEATHER_CLOUD_ENTRY, &args![weather, layer])
        .ptr::<()>()
        .is_null()
    {
        return;
    }
    let weather = e.get(this, Sky::pCurrentWeather);
    let entry: Ptr = e.call(WEATHER_CLOUD_ENTRY, &args![weather, layer]).ptr();
    if e.call(TEXTURE_ENTRY_VALID, &args![entry]).u32() == 0 {
        return;
    }
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCT, &args![text]);
        let weather = e.get(this, Sky::pCurrentWeather);
        let first: Ptr = e.call(WEATHER_CLOUD_ENTRY, &args![weather, layer]).ptr();
        let weather = e.get(this, Sky::pCurrentWeather);
        let second: Ptr = e.call(WEATHER_CLOUD_ENTRY, &args![weather, layer]).ptr();
        let name = e.call(TEXTURE_ENTRY_NAME, &args![second]).u32();
        let prefix = e.vcall(first.addr(), 0x18, &[]).u32();
        e.call(FORMAT_STRING, &args![text, PATH_FORMAT, prefix, name]);
        let path = ni_pointer_get(e, text.addr());
        if e.call(FILE_EXISTS, &args![path, 0u32, 0u32, 0xffff_ffffu32])
            .u32()
            != 0
        {
            let clouds = e.get(this, Sky::pClouds);
            let node = fn_0063c440(e, clouds, layer);
            let property: Ptr = e.call(NODE_GET_PROPERTY, &args![node, 5u32]).ptr();
            if !property.is_null() {
                e.with_stack(4, |e, holder| {
                    e.call(NI_POINTER_FROM_RAW, &args![holder, 0u32]);
                    let path = ni_pointer_get(e, text.addr());
                    e.call(TEXTURE_LOAD, &args![path, 1u32, holder, 1u32, 0u32]);
                    let texture = ni_pointer_get(e, holder.addr());
                    let clouds = e.get(this, Sky::pClouds);
                    let node = fn_0063c440(e, clouds, layer);
                    let property: Ptr = e.call(NODE_GET_PROPERTY, &args![node, 3u32]).ptr();
                    e.call(PROPERTY_SET_TEXTURE, &args![property, texture]);
                    e.call(NI_POINTER_DESTRUCTOR, &args![holder]);
                });
            }
        }
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

// Translated from 0063e2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the textures of the visible sky (a `Sky` method the map leaves
/// unnamed; `Sky::SetMode` runs it when mode 2 or 3 is entered). With clouds, a
/// current weather and the world flag clear, loads each cloud layer's texture
/// from the weather (`"%s%s"` path of the entry, when the file exists). Each
/// moon gets `"Textures\Sky\MoonShadow.dds"` as its shadow texture, its node
/// marked by whether it loaded, and its state word set to 2. With a sun and a
/// climate, loads the sun's two textures from the climate (both entries use
/// index 0 here; the glare name `"Sky\SunGlare.dds"` becomes
/// `"Textures\Sky\SunGlareNonHDR.dds"` while the flag byte `011f941e` is
/// clear) into the properties of the sun's nodes and marks the nodes by
/// whether a texture loaded (the flag is not reset between the two loads, as
/// in the game). With stars and a climate, loads the stars' geometry. The
/// compiler's exception frame is not translated.
pub fn fn_0063e2f0(e: &mut Engine, this: Ptr<Sky>) {
    with_timer(e, 0x8dc, |e| {
        let clouds = e.get(this, Sky::pClouds);
        if !clouds.is_null() && !e.get(this, Sky::pCurrentWeather).is_null() {
            let state: u32 = e.global(WORLD_STATE_POINTER);
            if !e.call(WORLD_STATE_FLAG, &args![state]).bool() {
                let mut layer = 0i32;
                loop {
                    let clouds = e.get(this, Sky::pClouds);
                    let count = e.call(CLOUDS_LAYER_COUNT, &args![clouds]).i32();
                    if layer >= count {
                        break;
                    }
                    load_cloud_layer(e, this, layer);
                    layer += 1;
                }
            }
        }

        for field in [Sky::pMasser, Sky::pSecunda] {
            let moon = e.get(this, field);
            if moon.is_null() {
                continue;
            }
            let node = ni_pointer_get(e, moon.addr() + 0x14);
            let property: Ptr = e.call(NODE_GET_PROPERTY, &args![node, 3u32]).ptr();
            e.with_stack(4, |e, holder| {
                e.call(NI_POINTER_FROM_RAW, &args![holder, 0u32]);
                e.call(
                    TEXTURE_LOAD,
                    &args![MOON_SHADOW_PATH, 1u32, holder, 1u32, 0u32],
                );
                let texture = ni_pointer_get(e, holder.addr());
                if !texture.is_null() {
                    e.call(PROPERTY_SET_TEXTURE, &args![property, texture]);
                }
                let node = ni_pointer_get(e, moon.addr() + 0x14);
                e.call(
                    NODE_SET_FLAG_BIT_20,
                    &args![node, (!texture.is_null()) as u32],
                );
                let moon = e.get(this, field);
                e.mem.set_u32(moon.addr() + 0x70, 2);
                e.call(NI_POINTER_DESTRUCTOR, &args![holder]);
            });
        }

        let sun = e.get(this, Sky::pSun);
        if !sun.is_null() && !e.get(this, Sky::pCurrentClimate).is_null() {
            load_sun_textures_at_entry_zero(e, this, sun);
        }

        let stars = e.get(this, Sky::pStars);
        let climate = e.get(this, Sky::pCurrentClimate);
        if !stars.is_null() && !climate.is_null() {
            let geometry = fn_0063cfe0(e, climate);
            e.call(STARS_LOAD_GEOMETRY, &args![stars, geometry]);
        }
    });
}

/// The sun part of `fn_0063e2f0`.
fn load_sun_textures_at_entry_zero(e: &mut Engine, this: Ptr<Sky>, sun: Ptr) {
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCT, &args![text]);
        let node: Ptr = e.call(SUN_FIRST_NODE, &args![sun]).ptr();
        let glare_property: Ptr = e.call(NODE_GET_PROPERTY, &args![node, 3u32]).ptr();
        let mut texture = Ptr::NULL;
        e.with_stack(4, |e, holder| {
            e.call(NI_POINTER_FROM_RAW, &args![holder, 0u32]);
            let climate = e.get(this, Sky::pCurrentClimate);
            let entry = fn_0063cfa0(e, climate, 0);
            if !entry.is_null() && e.call(TEXTURE_ENTRY_VALID, &args![entry]).u32() != 0 {
                let prefix = e.vcall(entry.addr(), 0x18, &[]).u32();
                e.call(STRING_ASSIGN, &args![text, prefix]);
                let name = e.call(TEXTURE_ENTRY_NAME, &args![entry]).u32();
                e.call(STRING_APPEND, &args![text, name]);
                texture = load_texture(e, text, holder, glare_property);
            }
            let node: Ptr = e.call(HOLDER_NODE, &args![sun]).ptr();
            e.call(
                NODE_SET_FLAG_BIT_20,
                &args![node, (!texture.is_null()) as u32],
            );

            let node = fn_0063d040(e, sun);
            let property: Ptr = e.call(NODE_GET_PROPERTY, &args![node, 3u32]).ptr();
            let climate = e.get(this, Sky::pCurrentClimate);
            let entry = fn_0063cfa0(e, climate, 0);
            if !entry.is_null() && e.call(TEXTURE_ENTRY_VALID, &args![entry]).u32() != 0 {
                let mut use_default = false;
                if e.global::<u8>(INITIAL_FLAG) == 0 {
                    let name = e.call(TEXTURE_ENTRY_NAME, &args![entry]).u32();
                    use_default = e.call(STRING_COMPARE, &args![name, SUN_GLARE_NAME]).u32() == 0;
                }
                if use_default {
                    e.call(STRING_ASSIGN, &args![text, SUN_GLARE_NON_HDR_PATH]);
                } else {
                    let prefix = e.vcall(entry.addr(), 0x18, &[]).u32();
                    e.call(STRING_ASSIGN, &args![text, prefix]);
                    let name = e.call(TEXTURE_ENTRY_NAME, &args![entry]).u32();
                    e.call(STRING_APPEND, &args![text, name]);
                }
                e.with_stack(4, |e, second_holder| {
                    e.call(NI_POINTER_FROM_RAW, &args![second_holder, 0u32]);
                    texture = load_texture(e, text, second_holder, property);
                    e.call(NI_POINTER_DESTRUCTOR, &args![second_holder]);
                });
            }
            let node: Ptr = e.call(SUN_GET_NODE, &args![sun]).ptr();
            e.call(
                NODE_SET_FLAG_BIT_20,
                &args![node, (!texture.is_null()) as u32],
            );
            e.call(NI_POINTER_DESTRUCTOR, &args![holder]);
        });
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

// Translated from 0063e860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts or ends the weather blend acceleration (a `Sky` method the map
/// leaves unnamed). With `start` set and a last weather, flag 8 not yet set:
/// sets flag 8 and remembers the current blend in `fAccelBeginPct`. Without
/// `start` or without a last weather: clears flag 8 and `fAccelBeginPct`.
pub fn fn_0063e860(e: &mut Engine, this: Ptr<Sky>, start: bool) {
    let has_last = !e.get(this, Sky::pLastWeather).is_null();
    if start && has_last && e.get(this, Sky::uiFlags) & 8 == 0 {
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags | 8);
        let pct = e.get(this, Sky::fCurrentWeatherPct);
        e.set(this, Sky::fAccelBeginPct, pct);
    } else if !start || !has_last {
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags & !8);
        e.set(this, Sky::fAccelBeginPct, 0.0);
    }
}

// Translated from 0063e8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::SetFastTravel` (Xbox PDB): with `fast_travel` set, forgets the
/// override weather and sets flag `0x10`; otherwise clears flag `0x10`.
pub fn sky_set_fast_travel(e: &mut Engine, this: Ptr<Sky>, fast_travel: bool) {
    if fast_travel {
        e.set(this, Sky::pOverrideWeather, Ptr::NULL);
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags | 0x10);
    } else {
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, flags & !0x10);
    }
}

/// The save format version (`008df040` on the save buffer global).
fn save_version(e: &mut Engine) -> u32 {
    let buffer: u32 = e.global(SAVE_BUFFER_GLOBAL);
    e.call(SAVE_VERSION, &args![buffer]).u8() as u32
}

// Translated from 0063e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size in bytes of the sky's save data (a `Sky` method the map leaves
/// unnamed; `this` is not read): 28 bytes, plus 4 from save version `0x5d`
/// (the override weather's id) and 8 from version `0x69` (the flags and the
/// acceleration start). The compiler's additions of 4 are summed.
pub fn fn_0063e940(e: &mut Engine, _this: Ptr) -> u16 {
    let mut size: u16 = 12;
    if save_version(e) >= 0x5d {
        size += 4;
    }
    size += 16;
    if save_version(e) >= 0x69 {
        size += 8;
    }
    size
}

/// Saves the id of `form` (zero when null) through `SaveNumericID`.
fn save_form_id(e: &mut Engine, form: Ptr) {
    e.with_stack(4, |e, slot| {
        let id = if form.is_null() {
            0
        } else {
            e.call(FORM_ID_OF, &args![form]).u32()
        };
        e.mem.set_u32(slot.addr(), id);
        let buffer: u32 = e.global(SAVE_BUFFER_GLOBAL);
        e.call(SAVE_NUMERIC_ID, &args![buffer, slot, 4u32]);
    });
}

// Translated from 0063e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::SaveGame` (Xbox PDB): writes the current, last and default weather
/// ids, the override weather id (from save version `0x5d`), the hour, last
/// update hour, blend and mode, and from version `0x69` the flags and the
/// acceleration start.
pub fn sky_save_game(e: &mut Engine, this: Ptr<Sky>) {
    for field in [
        Sky::pCurrentWeather,
        Sky::pLastWeather,
        Sky::pDefaultWeather,
    ] {
        let weather = e.get(this, field);
        save_form_id(e, weather);
    }
    if save_version(e) >= 0x5d {
        let weather = e.get(this, Sky::pOverrideWeather);
        save_form_id(e, weather);
    }
    let buffer: u32 = e.global(SAVE_BUFFER_GLOBAL);
    for offset in [0xecu32, 0xf0, 0xf4, 0xf8] {
        e.call(SAVE_DATA, &args![buffer, this.addr() + offset, 4u32]);
    }
    if save_version(e) >= 0x69 {
        let buffer: u32 = e.global(SAVE_BUFFER_GLOBAL);
        for offset in [0x118u32, 0x110] {
            e.call(SAVE_DATA, &args![buffer, this.addr() + offset, 4u32]);
        }
    }
}

// Translated from 0063eb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::SaveGame_ov2` (Xbox PDB): writes the four weather ids (current, last,
/// default, override) and the fields at `+0xec`, `+0xf0`, `+0xf4`, `+0x118`,
/// `+0x110`, `+0xb4`, `+0xb8`, `+0xbc`, `+0xe4`, `+0xe8` and `+0xf8` (4 bytes
/// each) into the save buffer `buffer`.
pub fn sky_save_game_ov2(e: &mut Engine, this: Ptr<Sky>, buffer: Ptr) {
    for field in [
        Sky::pCurrentWeather,
        Sky::pLastWeather,
        Sky::pDefaultWeather,
        Sky::pOverrideWeather,
    ] {
        let weather = e.get(this, field);
        e.call(SAVE_FORM_ID_OV2, &args![buffer, weather, 0u32]);
    }
    for offset in [
        0xecu32, 0xf0, 0xf4, 0x118, 0x110, 0xb4, 0xb8, 0xbc, 0xe4, 0xe8, 0xf8,
    ] {
        e.call(
            SAVE_DATA_OV2,
            &args![buffer, this.addr() + offset, 4u32, 0u32],
        );
    }
}

/// Reads a weather from the load buffer: the saved form id, looked up
/// (`004839c0`) and cast from `TESForm` to `TESWeather`.
fn load_weather(e: &mut Engine, buffer: Ptr) -> Ptr {
    let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form: Ptr = e.call(LOOKUP_FORM, &args![id]).ptr();
    e.call(
        DYNAMIC_CAST,
        &args![
            form,
            0u32,
            TYPE_DESCRIPTOR_FORM,
            TYPE_DESCRIPTOR_WEATHER,
            0u32
        ],
    )
    .ptr()
}

// Translated from 0063ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::LoadGame` (Xbox PDB): does nothing while the world flag `0x40` of the
/// world state is set (`0063ef00`). Otherwise flushes the precipitation,
/// clears the clouds' transition textures and flags the clouds (`0063a610`),
/// reads the four weathers (current, last, default, override), the hour, last
/// update hour and blend, the flags (only bit 8 is taken), the acceleration
/// start, the colour at `+0xb4` (12 bytes before buffer version `0x12`, three
/// words from it; the version is the buffer's virtual slot 0), and the fields
/// at `+0xe4`, `+0xe8` and `+0xf8`, then runs `Sky::Update(0.0)` and
/// `Sky::UpdateHDRValues`.
pub fn sky_load_game(e: &mut Engine, this: Ptr<Sky>, buffer: Ptr) {
    let state: u32 = e.global(WORLD_STATE_POINTER);
    if fn_0063ef00(e, Ptr::new(state)) {
        return;
    }
    flush_precipitation(e, this);
    let clouds = e.get(this, Sky::pClouds);
    if !clouds.is_null() {
        e.call(CLOUDS_CLEAR_TRANS_TEXTURES, &args![clouds]);
        let clouds = e.get(this, Sky::pClouds);
        fn_0063a610(e, clouds);
    }
    for field in [
        Sky::pCurrentWeather,
        Sky::pLastWeather,
        Sky::pDefaultWeather,
        Sky::pOverrideWeather,
    ] {
        let weather = load_weather(e, buffer);
        e.set(this, field, weather);
    }
    for offset in [0xecu32, 0xf0, 0xf4] {
        e.call(LOAD_DATA, &args![buffer, this.addr() + offset, 4u32]);
    }
    e.with_stack(4, |e, saved_flags| {
        e.call(LOAD_DATA, &args![buffer, saved_flags, 4u32]);
        let saved = e.mem.u32(saved_flags.addr());
        let flags = e.get(this, Sky::uiFlags);
        e.set(this, Sky::uiFlags, (flags & !8) | (saved & 8));
    });
    e.call(LOAD_DATA, &args![buffer, this.addr() + 0x110, 4u32]);
    if e.vcall(buffer.addr(), 0, &[]).u8() >= 0x12 {
        for offset in [0xb4u32, 0xb8, 0xbc] {
            e.call(LOAD_DATA, &args![buffer, this.addr() + offset, 4u32]);
        }
    } else {
        e.call(LOAD_DATA, &args![buffer, this.addr() + 0xb4, 0xcu32]);
    }
    for offset in [0xe4u32, 0xe8, 0xf8] {
        e.call(LOAD_DATA, &args![buffer, this.addr() + offset, 4u32]);
    }
    sky_update(e, this, 0.0);
    sky_update_hdr_values(e, this);
}

// Translated from 0063ef00 (decompiled, FalloutNV.exe 1.4.0.525)
/// World state accessor (by its caller): whether bit `0x40` of the word at
/// `+0x244` is set.
pub fn fn_0063ef00(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x244)) & 0x40 != 0
}

/// Creates one image-space modifier instance (`0x30` bytes, `NiObject`
/// allocation and constructor).
fn new_image_space_instance(e: &mut Engine) -> Ptr {
    let block: Ptr = e.call(NI_OBJECT_NEW, &args![IMAGE_SPACE_SIZE]).ptr();
    if block.is_null() {
        Ptr::NULL
    } else {
        e.call(IMAGE_SPACE_CONSTRUCT, &args![block]).ptr()
    }
}

/// Registers `instance` with the image-space manager through a temporary
/// `NiPointer` holder.
fn register_image_space_instance(e: &mut Engine, instance: Ptr) {
    e.with_stack(4, |e, holder| {
        e.call(NI_POINTER_FROM_RAW, &args![holder, instance]);
        let owner: u32 = e.global(INTERIOR_CELL_HOLDER);
        let manager: Ptr = e.call(IMAGE_SPACE_MANAGER, &args![owner]).ptr();
        e.call(IMAGE_SPACE_ADD, &args![manager, holder]);
        e.call(NI_POINTER_DESTRUCTOR, &args![holder]);
    });
}

/// The weather's image-space form for the time-of-day index `index`, or the
/// default form when it has none (or there is no weather).
fn weather_image_space(e: &mut Engine, weather: Ptr, index: u32) -> Ptr {
    let mut form = Ptr::NULL;
    if !weather.is_null() {
        form = e
            .call(
                WEATHER_IMAGE_SPACE_FOR_TIME,
                &args![weather.addr() + 0x18, index],
            )
            .ptr();
    }
    if form.is_null() {
        form = e.call(DEFAULT_IMAGE_SPACE, &[]).ptr();
    }
    form
}

/// Sets `instance`'s weight to `weight`, and, when there is a form, its form.
fn drive_image_space(e: &mut Engine, instance: Ptr, weight: f32, form: Ptr) {
    if !form.is_null() {
        e.call(IMAGE_SPACE_SET_WEIGHT, &args![instance, weight]);
        e.call(IMAGE_SPACE_SET_FORM, &args![instance, form]);
    }
}

// Translated from 0063ef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::UpdateHDRValues` (Xbox PDB): drives the four image-space modifier
/// instances (`+0x11c`, `+0x120` for the current weather, `+0x124`, `+0x128`
/// for the last one) from the time of day and the weather blend. The first
/// call creates the four instances (registered with the manager, flagged `1`,
/// byte `+8` set). In modes 0 and 1 all four weights are 0. Otherwise the hour
/// (`fCurrentGameHour`) is placed against the climate's transition hours
/// (sunrise begin and end, sunset begin and end, and `fHighNoon`): in dawn
/// (indices 0 then 3/1), morning (1, 4), afternoon (4, 1), dusk (2 then 3/1) it
/// has two image-space forms and a blend between them; at night it has one
/// (index 3); a hour order the data does not allow logs the "Transition times
/// stored in climate data are invalid" error and uses index 1. The current
/// weather's instances get `pct * blend` and `(1 - blend) * pct`; the last
/// weather's (when it exists and the blend is below 1) get `(1 - pct) * blend`
/// and `(1 - pct) * (1 - blend)`, else 0. A missing form falls back to the
/// default one (`00532ff0`). The compiler's exception frame is not
/// translated.
pub fn sky_update_hdr_values(e: &mut Engine, this: Ptr<Sky>) {
    if e.get(this, Sky::pCurrentWeatherImageSpaceMod).is_null() {
        let instance = new_image_space_instance(e);
        e.set(this, Sky::pCurrentWeatherImageSpaceMod, instance);
        let instance = new_image_space_instance(e);
        e.set(this, Sky::pLastWeatherImageSpaceMod, instance);
        let last = e.get(this, Sky::pLastWeatherImageSpaceMod);
        register_image_space_instance(e, last);
        let current = e.get(this, Sky::pCurrentWeatherImageSpaceMod);
        register_image_space_instance(e, current);
        let instance = new_image_space_instance(e);
        e.set(this, Sky::pCurrentWeatherImageSpaceMod2, instance);
        let instance = new_image_space_instance(e);
        e.set(this, Sky::pLastWeatherImageSpaceMod2, instance);
        let last = e.get(this, Sky::pLastWeatherImageSpaceMod2);
        register_image_space_instance(e, last);
        let current = e.get(this, Sky::pCurrentWeatherImageSpaceMod2);
        register_image_space_instance(e, current);
        for field in [
            Sky::pCurrentWeatherImageSpaceMod,
            Sky::pLastWeatherImageSpaceMod,
            Sky::pCurrentWeatherImageSpaceMod2,
            Sky::pLastWeatherImageSpaceMod2,
        ] {
            let instance = e.get(this, field);
            e.call(IMAGE_SPACE_SET_FLAG, &args![instance, 1u32]);
        }
        for field in [
            Sky::pCurrentWeatherImageSpaceMod,
            Sky::pLastWeatherImageSpaceMod,
            Sky::pCurrentWeatherImageSpaceMod2,
            Sky::pLastWeatherImageSpaceMod2,
        ] {
            let instance = e.get(this, field);
            e.mem.set_u8(instance.addr() + 8, 1);
        }
    }

    let mode = e.get(this, Sky::eMode);
    if mode == 1 || mode == 0 {
        for field in [
            Sky::pCurrentWeatherImageSpaceMod,
            Sky::pCurrentWeatherImageSpaceMod2,
            Sky::pLastWeatherImageSpaceMod,
            Sky::pLastWeatherImageSpaceMod2,
        ] {
            let instance = e.get(this, field);
            e.call(IMAGE_SPACE_SET_WEIGHT, &args![instance, 0.0f32]);
        }
        return;
    }

    let mut second = false;
    let mut blend = 1.0f32;
    let mut index_second = 0u32;
    let sunrise_begin = fn_0063b9b0(e, this);
    let sunrise_end = e.call(SKY_SUNRISE_END, &args![this]).f32();
    let sunset_begin = e.call(SKY_SUNSET_BEGIN, &args![this]).f32();
    let sunset_end = fn_0063ba30(e, this);
    let hour = e.get(this, Sky::fCurrentGameHour);
    let noon = e.get(this, Sky::fHighNoon);
    let half: f64 = e.global(DOUBLE_HALF);
    let index_first: u32;
    if sunrise_begin < hour && hour < sunrise_end {
        // Dawn: two forms, the blend peaks at the middle of the transition.
        second = true;
        index_first = 0;
        let span = ((sunrise_end as f64 - sunrise_begin as f64) * half) as f32;
        let middle = (sunrise_begin as f64 + span as f64) as f32;
        let distance = if middle > hour {
            index_second = 3;
            middle as f64 - hour as f64
        } else {
            index_second = 1;
            hour as f64 - middle as f64
        };
        blend = (1.0 - distance / span as f64) as f32;
    } else if sunrise_end < hour && noon > hour {
        second = true;
        index_first = 1;
        index_second = 4;
        let span = (noon as f64 - sunrise_end as f64) as f32;
        blend = (1.0 - (noon as f64 - hour as f64) / span as f64) as f32;
    } else if hour > noon && sunset_begin > hour {
        second = true;
        index_first = 4;
        index_second = 1;
        let span = (sunset_begin as f64 - noon as f64) as f32;
        blend = (1.0 - (sunset_begin as f64 - hour as f64) / span as f64) as f32;
    } else if sunset_begin < hour && sunset_end > hour {
        second = true;
        index_first = 2;
        let span = ((sunset_end as f64 - sunset_begin as f64) * half) as f32;
        let middle = (sunset_begin as f64 + span as f64) as f32;
        let distance = if middle > hour {
            index_second = 1;
            middle as f64 - hour as f64
        } else {
            index_second = 3;
            hour as f64 - middle as f64
        };
        blend = (1.0 - distance / span as f64) as f32;
    } else if sunset_end <= hour || sunrise_begin >= hour {
        index_first = 3;
    } else {
        e.call(LOG_MASTERFILE_ERROR, &args![TRANSITION_TIMES_MESSAGE]);
        index_first = 1;
    }

    let current = e.get(this, Sky::pCurrentWeather);
    let pct = e.get(this, Sky::fCurrentWeatherPct);
    let form = weather_image_space(e, current, index_first);
    let instance = e.get(this, Sky::pCurrentWeatherImageSpaceMod);
    drive_image_space(e, instance, (pct as f64 * blend as f64) as f32, form);
    if second {
        let current = e.get(this, Sky::pCurrentWeather);
        let form = weather_image_space(e, current, index_second);
        let instance = e.get(this, Sky::pCurrentWeatherImageSpaceMod2);
        drive_image_space(
            e,
            instance,
            ((1.0 - blend as f64) * pct as f64) as f32,
            form,
        );
    } else {
        let instance = e.get(this, Sky::pCurrentWeatherImageSpaceMod2);
        e.call(IMAGE_SPACE_SET_WEIGHT, &args![instance, 0.0f32]);
    }

    let last = e.get(this, Sky::pLastWeather);
    let one: f64 = e.global(DOUBLE_ONE);
    if !last.is_null() && (pct as f64) < one {
        let form = weather_image_space(e, last, index_first);
        let instance = e.get(this, Sky::pLastWeatherImageSpaceMod);
        drive_image_space(
            e,
            instance,
            ((1.0 - pct as f64) * blend as f64) as f32,
            form,
        );
        if second {
            let form = weather_image_space(e, last, index_second);
            let instance = e.get(this, Sky::pLastWeatherImageSpaceMod2);
            drive_image_space(
                e,
                instance,
                ((1.0 - pct as f64) * (1.0 - blend as f64)) as f32,
                form,
            );
        } else {
            let instance = e.get(this, Sky::pLastWeatherImageSpaceMod2);
            e.call(IMAGE_SPACE_SET_WEIGHT, &args![instance, 0.0f32]);
        }
    } else {
        for field in [
            Sky::pLastWeatherImageSpaceMod,
            Sky::pLastWeatherImageSpaceMod2,
        ] {
            let instance = e.get(this, field);
            e.call(IMAGE_SPACE_SET_WEIGHT, &args![instance, 0.0f32]);
        }
    }
}
// @@FUNCS

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00639bf0,
            fn_00639bf0(Ptr<SkySound>, u32, u32, u32, Ptr, u32, u32) -> Ptr<SkySound>
        ),
        entry!(0x00639c90, fn_00639c90(Ptr, u32) -> bool),
        entry!(
            0x0063a080,
            sky_scalar_deleting_destructor(Ptr<Sky>, u32) -> Ptr<Sky>
        ),
        entry!(0x0063a0b0, fn_0063a0b0(Ptr<Sky>)),
        entry!(0x0063a3c0, fn_0063a3c0(Ptr<SkySound>, u32) -> Ptr<SkySound>),
        entry!(0x0063a3f0, sky_set_mode(Ptr<Sky>, u32)),
        entry!(0x0063a610, fn_0063a610(Ptr)),
        entry!(0x0063a630, sky_initialize(Ptr<Sky>, Ptr, u32)),
        entry!(0x0063ac70, sky_update(Ptr<Sky>, f32)),
        entry!(0x0063b0a0, fn_0063b0a0(Ptr<Sky>, f32)),
        entry!(0x0063b120, fn_0063b120(Ptr<Sky>)),
        entry!(0x0063b600, fn_0063b600(Ptr, i32, Ptr)),
        entry!(0x0063b630, fn_0063b630(Ptr<Sky>, Ptr, Ptr, u32, Ptr, Ptr)),
        entry!(0x0063b9b0, fn_0063b9b0(Ptr<Sky>) -> f32),
        entry!(0x0063ba30, fn_0063ba30(Ptr<Sky>) -> f32),
        entry!(
            0x0063bab0,
            fn_0063bab0(Ptr<Sky>, Ptr, Ptr, Ptr, i32, Ptr, Ptr)
        ),
        entry!(0x0063bb40, fn_0063bb40(Ptr, i32, i32) -> u32),
        entry!(
            0x0063bb70,
            fn_0063bb70(Ptr<Sky>, Ptr, Ptr, Ptr, i32, Ptr, Ptr)
        ),
        entry!(0x0063bc60, fn_0063bc60(Ptr, i32, i32) -> u32),
        entry!(0x0063bce0, fn_0063bce0(Ptr<Sky>)),
        entry!(0x0063c420, fn_0063c420(Ptr, i32) -> f32),
        entry!(0x0063c440, fn_0063c440(Ptr, i32) -> Ptr),
        entry!(0x0063c460, fn_0063c460() -> bool),
        entry!(0x0063c470, fn_0063c470() -> u32),
        entry!(0x0063c480, fn_0063c480() -> f32),
        entry!(0x0063c490, fn_0063c490(Ptr<Sky>)),
        entry!(0x0063c600, fn_0063c600(f32) -> f32),
        entry!(0x0063c640, fn_0063c640(u32, u32)),
        entry!(0x0063c660, fn_0063c660(f32)),
        entry!(0x0063c670, fn_0063c670(f32, f32)),
        entry!(0x0063c690, fn_0063c690(Ptr<Sky>, Ptr, Ptr, f32)),
        entry!(0x0063c8a0, fn_0063c8a0(Ptr, Ptr) -> Ptr),
        entry!(0x0063c8f0, sky_set_current_climate(Ptr<Sky>, Ptr, bool)),
        entry!(0x0063c9f0, fn_0063c9f0(Ptr<Sky>)),
        entry!(0x0063ca20, fn_0063ca20(Ptr<Sky>)),
        entry!(0x0063cfa0, fn_0063cfa0(Ptr, i32) -> Ptr),
        entry!(0x0063cfe0, fn_0063cfe0(Ptr) -> u32),
        entry!(0x0063d000, fn_0063d000(Ptr) -> bool),
        entry!(0x0063d020, fn_0063d020(Ptr) -> bool),
        entry!(0x0063d040, fn_0063d040(Ptr) -> Ptr),
        entry!(0x0063d060, sky_reset_weather(Ptr<Sky>)),
        entry!(0x0063d0e0, sky_force_weather(Ptr<Sky>, Ptr, bool)),
        entry!(0x0063d1d0, fn_0063d1d0(Ptr<Sky>)),
        entry!(0x0063d5c0, fn_0063d5c0(Ptr) -> Ptr),
        entry!(0x0063d5e0, fn_0063d5e0(Ptr<Sky>)),
        entry!(0x0063dd50, fn_0063dd50(Ptr, i32) -> u8),
        entry!(0x0063dd70, fn_0063dd70(Ptr<SkySound>) -> bool),
        entry!(0x0063dd90, fn_0063dd90(Ptr<SkySound>)),
        entry!(0x0063ddb0, fn_0063ddb0(Ptr<Sky>, Ptr)),
        entry!(0x0063dff0, fn_0063dff0(Ptr) -> Ptr),
        entry!(0x0063e010, fn_0063e010(Ptr<Sky>)),
        entry!(0x0063e060, fn_0063e060(Ptr<Sky>, u32, Ptr) -> bool),
        entry!(0x0063e0d0, sky_sound_play(Ptr<SkySound>, f32)),
        entry!(0x0063e210, sky_unload_all_textures(Ptr<Sky>)),
        entry!(0x0063e290, fn_0063e290(Ptr)),
        entry!(0x0063e2f0, fn_0063e2f0(Ptr<Sky>)),
        entry!(0x0063e860, fn_0063e860(Ptr<Sky>, bool)),
        entry!(0x0063e8f0, sky_set_fast_travel(Ptr<Sky>, bool)),
        entry!(0x0063e940, fn_0063e940(Ptr) -> u16),
        entry!(0x0063e9f0, sky_save_game(Ptr<Sky>)),
        entry!(0x0063eb70, sky_save_game_ov2(Ptr<Sky>, Ptr)),
        entry!(0x0063ecb0, sky_load_game(Ptr<Sky>, Ptr)),
        entry!(0x0063ef00, fn_0063ef00(Ptr) -> bool),
        entry!(0x0063ef20, sky_update_hdr_values(Ptr<Sky>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn ret_float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    /// An engine with test doubles for the smart-pointer, list and timer
    /// helpers every sky function uses: `NiPointer::get` reads the word,
    /// `operator=` stores it (no reference counting), `operator!=` compares,
    /// list nodes are `{ item, next }`, `operator new` is the heap.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_NOT_EQUAL, |e, a| {
            ret((e.mem.u32(a[0]) != a[1]) as u32)
        });
        e.register(NI_POINTER_DESTRUCTOR, |_, _| Ret::default());
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(TIMER_SCOPE_CONSTRUCT, |_, _| Ret::default());
        e.register(TIMER_SCOPE_DESTROY, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NI_OBJECT_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e
    }

    /// Maps the pages of the exe's data the sky code reads and sets the
    /// constants it loads.
    fn with_data(mut e: Engine) -> Engine {
        e.map(0x0100_0000, 0x0010_0000);
        e.map(0x0118_0000, 0x0009_0000);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e
    }

    const VTABLE: u32 = 0x00ac_0000;
    const SLOT_BASE: u32 = 0x00aa_0000;

    /// A vtable at [`VTABLE`] whose slot at byte offset `k` is a no-op
    /// double at `SLOT_BASE + k`, so `call_log` shows which slot was called.
    fn install_vtable(e: &mut Engine) {
        let slots: Vec<u32> = (0..0x48).map(|i| SLOT_BASE + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for i in 0..0x48 {
            e.register(SLOT_BASE + 4 * i, |_, _| Ret::default());
        }
    }

    fn object_with_vtable(e: &mut Engine, size: u32) -> Ptr {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        Ptr::new(object)
    }

    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(address, _)| *address).collect()
    }

    fn new_sky(e: &mut Engine) -> Ptr<Sky> {
        e.new_object()
    }

    #[test]
    fn sky_sound_constructor_copies_the_handle_and_stores_the_description() {
        let mut e = engine();
        e.register(SOUND_HANDLE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], u32::MAX);
            ret(a[0])
        });
        e.register(SOUND_HANDLE_ASSIGN, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            ret(a[0])
        });
        e.register(SOUND_HANDLE_DESTRUCT, |_, _| Ret::default());
        e.register(MEMSET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let sound: Ptr<SkySound> = e.new_object();
        e.mem.set_u32(sound.addr() + 0x18, 0x1234);
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x0063_9bf0,
                &args![sound, 7u32, 1u32, 2u32, 0x5000u32, 3u32, 0xabcu32],
            )
            .ptr::<SkySound>();
        assert_eq!(back, sound);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            [
                0x0063_9bf0,
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCT,
                MEMSET
            ]
        );
        assert_eq!(log[4].1, [sound.addr() + 0x18, 0, 4]);
        // The handle passed by value ends up in the sound.
        assert_eq!(e.get(sound, SkySound::SndHandle), 7);
        assert_eq!(e.mem.u32(sound.addr() + 4), 1);
        assert_eq!(e.mem.u32(sound.addr() + 8), 2);
        assert_eq!(e.get(sound, SkySound::pWeather), Ptr::new(0x5000));
        assert_eq!(e.get(sound, SkySound::eSoundType), 3);
        assert_eq!(e.get(sound, SkySound::uiFormID), 0xabc);
        assert_eq!(e.get(sound, SkySound::uiData), 0);
    }

    #[test]
    fn fn_00639c90_grows_only_past_the_limit() {
        let mut e = engine();
        e.register(0x0045_7fe0, |_, _| ret(100));
        e.register(0x0063_9ce0, |_, _| ret(0x1111));
        e.register(0x0042_f5a0, |_, a| {
            assert_eq!(a, [0x8000_0000, 0x1111]);
            ret(40)
        });
        e.register(0x0063_9d00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0063_9c90, &args![0x6000u32, 5u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(0x0063_9d00, vec![0x6000, 105]));
        // A limit at or above the value: nothing happens.
        e.register(0x0042_f5a0, |_, _| ret(100));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0063_9c90, &args![0x6000u32, 5u32]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 4);
    }

    #[test]
    fn deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        e.register(LIST_CLEAR, |_, _| Ret::default());
        let sky = new_sky(&mut e);
        e.call_log = Some(vec![]);
        let back = e.call(0x0063_a080, &args![sky, 0u32]).ptr::<Sky>();
        assert_eq!(back, sky);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&OPERATOR_DELETE));
        e.call_log = Some(vec![]);
        e.call(0x0063_a080, &args![sky, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(OPERATOR_DELETE, vec![sky.addr()]));
    }

    #[test]
    fn destructor_deletes_components_in_order_and_empties_the_sound_list() {
        let mut e = engine();
        install_vtable(&mut e);
        e.register(LIST_CLEAR, |_, _| Ret::default());
        e.register(LIST_SCALAR_DELETING_DESTRUCTOR, |_, _| Ret::default());
        e.register(SKY_SOUND_BASE_DESTRUCT, |_, _| Ret::default());
        e.register(ROOT_GET_SCENE, |e, _| {
            let scene = object_with_vtable(e, 8);
            ret(scene.addr())
        });
        let sky = new_sky(&mut e);
        let fields = [
            Sky::pAtmosphere,
            Sky::pStars,
            Sky::pSun,
            Sky::pClouds,
            Sky::pMasser,
            Sky::pSecunda,
            Sky::pPrecip,
        ];
        let mut components = vec![];
        for field in fields {
            let component = object_with_vtable(&mut e, 0x10);
            e.set(sky, field, component);
            components.push(component.addr());
        }
        e.mem.set_u32(sky.addr() + 4, 0x7777);
        e.mem.set_u32(sky.addr() + 8, 0x8888);
        // Two sounds in the list: node = { item, next }.
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let (sound_a, sound_b) = (e.mem.alloc(0x1c), e.mem.alloc(0x1c));
        e.mem.set_u32(first, sound_a);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, sound_b);
        e.set(sky, Sky::pSkySoundList, Ptr::new(first));
        e.call_log = Some(vec![]);
        fn_0063a0b0(&mut e, sky);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(sky.addr()), SKY_VTABLE);
        // Slot 0 of atmosphere, stars, clouds, sun, masser, secunda, precip.
        let deletions: Vec<u32> = log
            .iter()
            .filter(|(address, _)| *address == SLOT_BASE)
            .map(|(_, args)| args[0])
            .collect();
        let expected: Vec<u32> = [0, 1, 3, 2, 4, 5, 6]
            .iter()
            .map(|index| components[*index])
            .collect();
        assert_eq!(deletions, expected);
        assert!(log
            .iter()
            .all(|(address, args)| *address != SLOT_BASE || args[1] == 1));
        // The root is taken out of its scene (slot 0xe8) and both smart
        // pointers are released.
        assert!(log
            .iter()
            .any(|(address, args)| *address == SLOT_BASE + 0xe8 && args[1] == 0x7777));
        assert_eq!(e.mem.u32(sky.addr() + 4), 0);
        assert_eq!(e.mem.u32(sky.addr() + 8), 0);
        // Both sounds are deleted, then the list is cleared and destroyed.
        let sounds: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(address, _)| *address == OPERATOR_DELETE)
            .map(|(_, args)| args)
            .collect();
        assert_eq!(sounds, [&vec![sound_a], &vec![sound_b]]);
        let tail = addresses(&log);
        assert_eq!(
            tail[tail.len() - 4..],
            [
                LIST_CLEAR,
                LIST_SCALAR_DELETING_DESTRUCTOR,
                NI_POINTER_DESTRUCTOR,
                NI_POINTER_DESTRUCTOR
            ]
        );
    }

    #[test]
    fn sky_sound_deleting_destructor_runs_the_base_and_frees() {
        let mut e = engine();
        e.register(SKY_SOUND_BASE_DESTRUCT, |_, _| Ret::default());
        let sound: Ptr<SkySound> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0063_a3c0, &args![sound, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (SKY_SOUND_BASE_DESTRUCT, vec![sound.addr()]),
                (OPERATOR_DELETE, vec![sound.addr()])
            ]
        );
        e.call_log = Some(vec![]);
        let back = e.call(0x0063_a3c0, &args![sound, 0u32]).ptr::<SkySound>();
        assert_eq!(back, sound);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn set_mode_entering_a_visible_mode_clears_the_visibility_flags() {
        let mut e = engine();
        e.register(SKY_ENTER_VISIBLE_MODE, |_, _| Ret::default());
        e.register(SET_NODE_HIDDEN, |_, _| Ret::default());
        e.register(SUN_GET_NODE, |_, _| ret(0x4444));
        e.register(SKY_UPDATE_HDR_VALUES, |_, _| Ret::default());
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 1);
        let clouds = e.mem.alloc(0x5c);
        e.set(sky, Sky::pClouds, Ptr::new(clouds));
        e.mem.set_u32(sky.addr() + 4, 0x3333);
        e.set(sky, Sky::pSun, Ptr::new(0x5555));
        e.call_log = Some(vec![]);
        e.call(0x0063_a3f0, &args![sky, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(sky, Sky::eMode), 3);
        assert_eq!(e.mem.u8(clouds + 0x5a), 1);
        assert!(log.contains(&(SET_NODE_HIDDEN, vec![0x3333, 0])));
        assert!(log.contains(&(SET_NODE_HIDDEN, vec![0x4444, 0])));
        assert_eq!(
            log.last().unwrap(),
            &(SKY_UPDATE_HDR_VALUES, vec![sky.addr()])
        );
    }

    #[test]
    fn set_mode_leaving_a_visible_mode_stops_every_sound() {
        let mut e = with_data(engine());
        e.register(SKY_UNLOAD_ALL_TEXTURES, |_, _| Ret::default());
        e.register(SET_NODE_HIDDEN, |_, _| Ret::default());
        e.register(SUN_GET_NODE, |_, _| ret(0x4444));
        e.register(SKY_UPDATE_HDR_VALUES, |_, _| Ret::default());
        e.register(SOUND_HANDLE_IS_PLAYING, |_, _| ret(1));
        e.register(SOUND_HANDLE_STOP, |_, _| Ret::default());
        e.register(SOUND_HANDLE_RELEASE, |_, _| Ret::default());
        e.register(SKY_SOUND_BASE_DESTRUCT, |_, _| Ret::default());
        e.register(PRECIPITATION_UPDATE, |_, _| Ret::default());
        // One sound in the list; the list is empty after `RemoveHead`.
        let list = e.mem.alloc(8);
        let sound = e.mem.alloc(0x1c);
        e.mem.set_u32(list, sound);
        e.register(LIST_IS_EMPTY, |e, a| ret((e.mem.u32(a[0]) == 0) as u32));
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 3);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.set(sky, Sky::pPrecip, Ptr::new(0x6666));
        e.set(sky, Sky::fFlash, 0.75);
        e.mem.set_u32(sky.addr() + 4, 0x3333);
        e.set_global(SET_MODE_FLAG, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0063_a3f0, &args![sky, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(addresses(&log)[1], SKY_UNLOAD_ALL_TEXTURES);
        assert!(log.contains(&(SET_NODE_HIDDEN, vec![0x3333, 1])));
        assert!(log.contains(&(SOUND_HANDLE_STOP, vec![sound])));
        assert!(log.contains(&(SOUND_HANDLE_RELEASE, vec![sound])));
        assert!(log.contains(&(OPERATOR_DELETE, vec![sound])));
        assert!(log.contains(&(LIST_REMOVE_HEAD, vec![list])));
        assert!(log.contains(&(PRECIPITATION_UPDATE, vec![0x6666, 0])));
        assert_eq!(e.get(sky, Sky::eMode), 0);
        assert_eq!(e.get(sky, Sky::fFlash), 0.0);
        assert_eq!(e.global::<u8>(SET_MODE_FLAG), 0);
    }

    #[test]
    fn set_mode_between_hidden_modes_only_stores_the_mode() {
        let mut e = engine();
        e.register(SKY_UPDATE_HDR_VALUES, |_, _| Ret::default());
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 0);
        e.call_log = Some(vec![]);
        e.call(0x0063_a3f0, &args![sky, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(addresses(&log), [0x0063_a3f0, SKY_UPDATE_HDR_VALUES]);
        assert_eq!(e.get(sky, Sky::eMode), 1);
    }

    #[test]
    fn clouds_flag_setter_sets_the_byte() {
        let mut e = engine();
        let clouds: Ptr = Ptr::new(e.mem.alloc(0x5c));
        e.call(0x0063_a610, &args![clouds]);
        assert_eq!(e.mem.u8(clouds.addr() + 0x5a), 1);
    }

    fn min_float(_: &mut Engine, a: &[u32]) -> Ret {
        let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
        ret_float(if second > first { first } else { second })
    }

    fn max_float(_: &mut Engine, a: &[u32]) -> Ret {
        let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
        ret_float(if second < first { first } else { second })
    }

    /// `NiColor::NiColor(r, g, b)`: stores the three floats, returns `this`.
    fn color_construct(e: &mut Engine, a: &[u32]) -> Ret {
        for channel in 0..3 {
            e.mem.set_u32(a[0] + 4 * channel, a[1 + channel as usize]);
        }
        ret(a[0])
    }

    /// Doubles for the helpers the colour mixing of `0063c690` uses.
    fn with_color_helpers(mut e: Engine) -> Engine {
        e.register(MIN_FLOAT, min_float);
        e.register(MAX_FLOAT, max_float);
        e.register(NI_COLOR_CONSTRUCT, color_construct);
        e.register(COLOR_CLAMP_TO_ONE, |e, a| {
            for channel in 0..3 {
                let value = e.mem.f32(a[0] + 4 * channel);
                if value > 1.0 {
                    e.mem.set_f32(a[0] + 4 * channel, 1.0);
                }
            }
            Ret::default()
        });
        e.register(CLAMP_FLOAT, |e, a| {
            let (low, high) = (f32::from_bits(a[1]), f32::from_bits(a[2]));
            let value = e.mem.f32(a[0]);
            if high < value {
                e.mem.set_f32(a[0], high);
            } else if value < low {
                e.mem.set_f32(a[0], low);
            }
            Ret::default()
        });
        // A float setting is its own value: the "pointer" is the address.
        e.register(SETTING_FLOAT_VALUE, |_, a| ret(a[0]));
        e
    }

    fn initialize_engine() -> Engine {
        let mut e = with_data(engine());
        install_vtable(&mut e);
        for constructor in [
            MULTIBOUND_NODE_CONSTRUCT,
            ATMOSPHERE_CONSTRUCT,
            STARS_CONSTRUCT,
            SUN_CONSTRUCT,
            CLOUDS_CONSTRUCT,
            MOONS_ROOT_CONSTRUCT,
            PRECIPITATION_CONSTRUCT,
            SKY_PROPERTY_CONSTRUCT,
        ] {
            e.register(constructor, |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                ret(a[0])
            });
        }
        for plain in [
            ROOT_SET_FIRST_FLAG,
            ROOT_SET_SECOND_FLAG,
            ROOT_SET_NAME,
            ROOT_ATTACH_PROPERTY,
            ROOT_UPDATE_PROPERTIES,
            PRECIPITATION_INITIALIZE,
            SKY_PROPERTY_SET_A,
            SKY_PROPERTY_SET_B,
            SKY_PROPERTY_SET_C,
            SKY_PROPERTY_SET_D,
        ] {
            e.register(plain, |_, _| Ret::default());
        }
        e.register(ROOT_GET_SCENE, |_, _| ret(0));
        e.register(NI_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        // The setting is its own value: the "pointer" is the address.
        e.register(SETTING_BYTE_VALUE, |_, a| ret(a[0]));
        e
    }

    #[test]
    fn initialize_builds_every_component_and_attaches_the_moons() {
        let mut e = initialize_engine();
        e.mem.set_u8(SETTING_PRECIPITATION, 1);
        let sky = new_sky(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0063_a630, &args![sky, 0u32, 0x77u32]);
        let log = e.call_log.take().unwrap();
        let root = e.mem.u32(sky.addr() + 4);
        let moons = e.mem.u32(sky.addr() + 8);
        let atmosphere = e.get(sky, Sky::pAtmosphere).addr();
        let stars = e.get(sky, Sky::pStars).addr();
        let sun = e.get(sky, Sky::pSun).addr();
        let clouds = e.get(sky, Sky::pClouds).addr();
        let precipitation = e.get(sky, Sky::pPrecip).addr();
        for value in [root, moons, atmosphere, stars, sun, clouds, precipitation] {
            assert_ne!(value, 0);
        }
        // Sizes the game allocates.
        assert_eq!(e.mem.block_size(root), Some(0xb8));
        assert_eq!(e.mem.block_size(atmosphere), Some(0x20));
        assert_eq!(e.mem.block_size(sun), Some(0x30));
        // The root is flagged and named; the components are initialized
        // with the root (slots 8 and 0x10) and the moons root is attached.
        assert!(log.contains(&(ROOT_SET_FIRST_FLAG, vec![root, 1])));
        assert!(log.contains(&(ROOT_SET_SECOND_FLAG, vec![root, 1])));
        assert!(log.contains(&(ROOT_SET_NAME, vec![root, ROOT_NAME_ARGUMENT])));
        assert!(log.contains(&(SLOT_BASE + 0x10, vec![atmosphere, root, 0x77])));
        assert!(log.contains(&(SLOT_BASE + 8, vec![stars, root])));
        assert!(log.contains(&(SLOT_BASE + 8, vec![sun, root])));
        assert!(log.contains(&(SLOT_BASE + 8, vec![clouds, root])));
        assert!(log.contains(&(SLOT_BASE + 0xdc, vec![root, moons, 1])));
        assert!(log.contains(&(PRECIPITATION_INITIALIZE, vec![precipitation])));
        // The property object gets its four settings, then the root.
        let property = log
            .iter()
            .find(|(address, _)| *address == SKY_PROPERTY_SET_A)
            .unwrap()
            .1[0];
        assert!(log.contains(&(SKY_PROPERTY_SET_A, vec![property, 6])));
        assert!(log.contains(&(SKY_PROPERTY_SET_B, vec![property, 7])));
        assert!(log.contains(&(SKY_PROPERTY_SET_C, vec![property, 1])));
        assert!(log.contains(&(SKY_PROPERTY_SET_D, vec![property, 1])));
        assert!(log.contains(&(ROOT_ATTACH_PROPERTY, vec![root, property])));
        // The last call is the scope timer's destructor.
        assert_eq!(log[log.len() - 2], (ROOT_UPDATE_PROPERTIES, vec![root]));
    }

    #[test]
    fn initialize_replaces_old_components_and_uses_the_given_root() {
        let mut e = initialize_engine();
        e.mem.set_u8(SETTING_PRECIPITATION, 0);
        e.set_global(INITIAL_FLAG, 1u8);
        let sky = new_sky(&mut e);
        let given_root = object_with_vtable(&mut e, 0xb4);
        // An old root with a scene, and an old atmosphere and precipitation.
        e.mem.set_u32(sky.addr() + 4, 0x1357);
        e.register(ROOT_GET_SCENE, |e, _| {
            let scene = object_with_vtable(e, 8);
            ret(scene.addr())
        });
        let old_atmosphere = object_with_vtable(&mut e, 0x20);
        e.set(sky, Sky::pAtmosphere, old_atmosphere);
        let old_precipitation = object_with_vtable(&mut e, 0x20);
        e.set(sky, Sky::pPrecip, old_precipitation);
        e.call_log = Some(vec![]);
        e.call(0x0063_a630, &args![sky, given_root, 0u32]);
        let log = e.call_log.take().unwrap();
        // The old root is taken out of its scene (slot 0xe8) and replaced.
        assert!(log
            .iter()
            .any(|(address, args)| *address == SLOT_BASE + 0xe8 && args[1] == 0x1357));
        assert_eq!(e.mem.u32(sky.addr() + 4), given_root.addr());
        assert!(!addresses(&log).contains(&MULTIBOUND_NODE_CONSTRUCT));
        // The old components are deleted with flag 1; no precipitation now.
        assert!(log.contains(&(SLOT_BASE, vec![old_atmosphere.addr(), 1])));
        assert!(log.contains(&(SLOT_BASE, vec![old_precipitation.addr(), 1])));
        assert!(e.get(sky, Sky::pPrecip).is_null());
        assert!(!addresses(&log).contains(&PRECIPITATION_INITIALIZE));
        assert_eq!(e.global::<f32>(INITIAL_FLOAT_A), 1.0);
        assert_eq!(e.global::<f32>(INITIAL_FLOAT_B), 1.0);
    }

    /// Doubles for everything `Sky::Update` touches outside this file, for a
    /// sky in mode 0 (fixed colours) with a player in a cell.
    fn update_engine() -> Engine {
        let mut e = with_color_helpers(with_data(engine()));
        install_vtable(&mut e);
        e.set_global(PLAYER_CHARACTER, 0x1000u32);
        e.register(PLAYER_PARENT_CELL, |_, _| ret(0x2000));
        e.register(CALENDAR_GET_HOUR, |_, _| ret_float(13.5));
        e.register(CAMERA_HOLDER_GET, |_, _| ret(0x3000));
        e.register(CAMERA_NODE, |_, _| ret(0x3100));
        e.register(NODE_WORLD_POSITION, |e, _| {
            let position = e.mem.alloc(16);
            e.mem.set_f32(position + 8, 50.0);
            ret(position)
        });
        e.register(RELEVANT_WATER_HEIGHT, |_, _| ret_float(10.0));
        e.register(WORLD_STATE_FLAG, |_, _| ret(0));
        e.register(FLAG_UNDERWATER_A, |_, _| ret(0));
        e.register(FLAG_UNDERWATER_B, |_, _| ret(0));
        e.register(SKY_UPDATE_TRANSITION, |_, _| Ret::default());
        e.register(SKY_UPDATE_UNKNOWN, |_, _| Ret::default());
        e.register(MOON_PHASE_APPLIES, |_, _| ret(1));
        e.register(FOG_COLOR_UPDATE, |_, _| Ret::default());
        e.register(CALENDAR_GET_DAYS, |_, _| ret(5));
        e.register(PRECIPITATION_UPDATE, |_, _| Ret::default());
        e.register(TICK_COUNT_GET, |_, _| ret(1500));
        // The flag byte at `0119f188` skips the camera-relative fog tail.
        e.set_global(0x0119_f188, 1u8);
        e
    }

    #[test]
    fn update_does_nothing_without_a_parent_cell() {
        let mut e = update_engine();
        e.register(PLAYER_PARENT_CELL, |_, _| ret(0));
        let sky = new_sky(&mut e);
        e.set(sky, Sky::uiFlags, 3);
        e.call_log = Some(vec![]);
        e.call(0x0063_ac70, &args![sky, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(sky, Sky::uiFlags), 3);
        assert_eq!(e.get(sky, Sky::fCurrentGameHour), 0.0);
        assert!(!addresses(&log).contains(&CALENDAR_GET_HOUR));
    }

    #[test]
    fn update_runs_the_components_and_the_moon_phases() {
        let mut e = update_engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 0);
        e.set(sky, Sky::uiFlags, 0x23);
        let atmosphere = object_with_vtable(&mut e, 0x20);
        let masser = object_with_vtable(&mut e, 0x80);
        let secunda = object_with_vtable(&mut e, 0x80);
        let precipitation = object_with_vtable(&mut e, 0x20);
        e.set(sky, Sky::pAtmosphere, atmosphere);
        e.set(sky, Sky::pMasser, masser);
        e.set(sky, Sky::pSecunda, secunda);
        e.set(sky, Sky::pPrecip, precipitation);
        // Secunda already has a phase; Masser does not.
        e.mem.set_u32(secunda.addr() + 0x70, 7);
        e.set(sky, Sky::uiLastMoonPhaseUpdate, 4);
        e.call_log = Some(vec![]);
        e.call(0x0063_ac70, &args![sky, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(sky, Sky::fCurrentGameHour), 13.5);
        // Camera at 50 above the water at 10: flag 4 is clear. Flags 1 and 2
        // are cleared at the end; the others stay.
        assert_eq!(e.get(sky, Sky::uiFlags), 0x20);
        // The day count (5) is newer than the last update (4).
        assert_eq!(e.get(sky, Sky::uiLastMoonPhaseUpdate), 5);
        assert_eq!(e.mem.u32(masser.addr() + 0x70), 1);
        assert_eq!(e.mem.u32(secunda.addr() + 0x70), 7);
        let dt = 0.5f32.to_bits();
        assert!(log.contains(&(SLOT_BASE + 0xc, vec![atmosphere.addr(), sky.addr(), dt])));
        assert!(log.contains(&(SLOT_BASE + 0xc, vec![masser.addr(), sky.addr(), dt])));
        assert!(log.contains(&(SLOT_BASE + 0xc, vec![secunda.addr(), sky.addr(), dt])));
        assert!(log.contains(&(PRECIPITATION_UPDATE, vec![precipitation.addr(), dt])));
        assert!(log.contains(&(FOG_COLOR_UPDATE, vec![sky.addr() + 0x48])));
    }

    #[test]
    fn update_sets_the_underwater_flag_and_fades_the_flash() {
        let mut e = update_engine();
        e.register(RELEVANT_WATER_HEIGHT, |_, _| ret_float(80.0));
        // The flash lasts 2 seconds; 1500 - 500 ms have passed.
        e.set_global(FLASH_DURATION_SETTING, 2.0f32);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::fFlash, 1.0);
        e.set(sky, Sky::uiFlashTime, 500);
        e.call(0x0063_ac70, &args![sky, 0.1f32]);
        assert_eq!(e.get(sky, Sky::uiFlags) & 4, 4);
        assert_eq!(e.get(sky, Sky::fFlash), 0.5);
        // Once the flash has run out it is cleared.
        e.set(sky, Sky::fFlash, 1.0);
        e.set(sky, Sky::uiFlashTime, 1500u32.wrapping_sub(2500));
        e.call(0x0063_ac70, &args![sky, 0.1f32]);
        assert_eq!(e.get(sky, Sky::fFlash), 0.0);
    }

    #[test]
    fn update_in_mode_1_without_the_world_flag_takes_the_short_path() {
        let mut e = update_engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 1);
        let atmosphere = object_with_vtable(&mut e, 0x20);
        let sun = object_with_vtable(&mut e, 0x30);
        let stars = object_with_vtable(&mut e, 0x20);
        e.set(sky, Sky::pAtmosphere, atmosphere);
        e.set(sky, Sky::pSun, sun);
        e.set(sky, Sky::pStars, stars);
        e.call_log = Some(vec![]);
        e.call(0x0063_ac70, &args![sky, 0.25f32]);
        let log = e.call_log.take().unwrap();
        let dt = 0.25f32.to_bits();
        // Only the atmosphere and the sun are updated.
        assert!(log.contains(&(SLOT_BASE + 0xc, vec![atmosphere.addr(), sky.addr(), dt])));
        assert!(log.contains(&(SLOT_BASE + 0xc, vec![sun.addr(), sky.addr(), dt])));
        assert!(!log
            .iter()
            .any(|(_, args)| args.first() == Some(&stars.addr())));
        assert!(!addresses(&log).contains(&SKY_UPDATE_TRANSITION));
        assert!(log.contains(&(FOG_COLOR_UPDATE, vec![sky.addr() + 0x48])));
    }

    #[test]
    fn short_update_updates_colours_fog_atmosphere_and_sun() {
        let mut e = update_engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 0);
        let sun = object_with_vtable(&mut e, 0x30);
        e.set(sky, Sky::pSun, sun);
        e.call_log = Some(vec![]);
        e.call(0x0063_b0a0, &args![sky, 1.5f32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(
            SLOT_BASE + 0xc,
            vec![sun.addr(), sky.addr(), 1.5f32.to_bits()]
        )));
        // Colours and fog were computed: mode 0 copies the fixed colour and
        // the fog is the default (power 1.0).
        assert_eq!(e.get(sky, Sky::fFogPower), 1.0);
        assert_eq!(
            log.last().unwrap(),
            &(FOG_COLOR_UPDATE, vec![sky.addr() + 0x48])
        );
    }

    #[test]
    fn clouds_layer_colour_is_copied_into_its_slot() {
        let mut e = engine();
        let clouds: Ptr = Ptr::new(e.mem.alloc(0x5c));
        let color = e.mem.alloc(12);
        for (i, value) in [0.25f32, 0.5, 0.75].iter().enumerate() {
            e.mem.set_f32(color + 4 * i as u32, *value);
        }
        e.call(0x0063_b600, &args![clouds, 2i32, color]);
        let slot = clouds.addr() + 0x28 + 2 * 12;
        assert_eq!(e.mem.f32(slot), 0.25);
        assert_eq!(e.mem.f32(slot + 4), 0.5);
        assert_eq!(e.mem.f32(slot + 8), 0.75);
    }

    type CallLog = Vec<(u32, Vec<u32>)>;

    /// Runs `fn_0063b630` for `hour` with sunrise 6..10, sunset 18..22, high
    /// noon 13 and a weather percentage of 0.5; returns the two indices, the
    /// four weights and the engine's call log.
    fn transition_at(hour: f32) -> (u32, u32, [f32; 4], CallLog) {
        let mut e = with_data(engine());
        e.register(SKY_SUNRISE_END, |_, _| ret_float(10.0));
        e.register(SKY_SUNSET_BEGIN, |_, _| ret_float(18.0));
        e.register(LOG_MASTERFILE_ERROR, |_, _| Ret::default());
        e.set_global(SUNRISE_BEGIN_CACHE, 6.0f32);
        e.set_global(SUNSET_END_CACHE, 22.0f32);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::fCurrentGameHour, hour);
        e.set(sky, Sky::fHighNoon, 13.0);
        e.set(sky, Sky::fCurrentWeatherPct, 0.5);
        let sample = e.mem.alloc(0x20);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(
            0x0063_b630,
            &args![sky, sample, 0x1000u32, 0u32, first, second],
        );
        let log = e.call_log.take().unwrap();
        let weights = [
            e.mem.f32(sample + 0x10),
            e.mem.f32(sample + 0x14),
            e.mem.f32(sample + 0x18),
            e.mem.f32(sample + 0x1c),
        ];
        (e.mem.u32(first), e.mem.u32(second), weights, log)
    }

    #[test]
    fn transition_sunrise_first_half_blends_from_night() {
        // Hour 6.5 is a quarter of the way through the sunrise (6..10): the
        // middle is 8, so the blend is 1 - (8 - 6.5) / 2 = 0.25 towards set 3.
        let (first, second, weights, _) = transition_at(6.5);
        assert_eq!((first, second), (0, 3));
        // Weights: blend and its complement scaled by 0.5, then by 1 - 0.5.
        assert_eq!(weights, [0.125, 0.375, 0.125, 0.375]);
    }

    #[test]
    fn transition_sunrise_second_half_blends_to_day() {
        let (first, second, weights, _) = transition_at(9.0);
        assert_eq!((first, second), (0, 1));
        assert_eq!(weights, [0.25, 0.25, 0.25, 0.25]);
    }

    #[test]
    fn transition_morning_runs_from_set_4_to_set_1() {
        // 11 is between sunrise end (10) and high noon (13): 1 - 2/3.
        let (first, second, weights, _) = transition_at(11.0);
        assert_eq!((first, second), (4, 1));
        let blend = (1.0f64 - 2.0 / 3.0) as f32;
        assert_eq!(weights[0], (blend as f64 * 0.5) as f32);
    }

    #[test]
    fn transition_afternoon_runs_from_set_1_to_set_4() {
        // 15 is between high noon (13) and sunset begin (18): 1 - 3/5.
        let (first, second, weights, _) = transition_at(15.0);
        assert_eq!((first, second), (1, 4));
        assert_eq!(weights[0], (0.4f64 * 0.5) as f32);
    }

    #[test]
    fn transition_sunset_blends_towards_night() {
        let (first, second, _, _) = transition_at(19.0);
        assert_eq!((first, second), (2, 1));
        let (first, second, weights, _) = transition_at(20.0);
        assert_eq!((first, second), (2, 3));
        // The middle of the sunset is 20: the blend is exactly 1.
        assert_eq!(weights, [0.5, 0.0, 0.5, 0.0]);
    }

    #[test]
    fn transition_at_night_uses_set_3_for_both() {
        for hour in [23.0f32, 3.0] {
            let (first, second, weights, log) = transition_at(hour);
            assert_eq!((first, second), (3, 3));
            assert_eq!(weights, [0.5, 0.0, 0.5, 0.0]);
            assert!(!addresses(&log).contains(&LOG_MASTERFILE_ERROR));
        }
    }

    #[test]
    fn transition_with_an_invalid_hour_logs_the_data_error() {
        let (first, second, weights, log) = transition_at(f32::NAN);
        assert_eq!((first, second), (1, 1));
        assert_eq!(weights, [0.5, 0.0, 0.5, 0.0]);
        assert!(log.contains(&(LOG_MASTERFILE_ERROR, vec![TRANSITION_TIMES_MESSAGE])));
    }

    #[test]
    fn transition_without_a_weather_does_nothing() {
        let mut e = with_data(engine());
        let sky = new_sky(&mut e);
        let sample = e.mem.alloc(0x20);
        let first = e.mem.alloc(4);
        e.mem.set_u32(first, 99);
        e.call_log = Some(vec![]);
        e.call(
            0x0063_b630,
            &args![sky, sample, 0u32, 0u32, first, 0x8000_0000u32],
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(first), 99);
    }

    #[test]
    fn sunrise_begin_is_cached_until_the_climate_changes() {
        let mut e = with_data(engine());
        e.register(MAX_FLOAT, max_float);
        e.register(SETTING_FLOAT_VALUE, |_, a| ret(a[0]));
        e.register(SKY_SUNRISE_BEGIN, |_, _| ret_float(7.0));
        e.set_global(TIME_SETTING, 1.0f32);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::uiFlags, 0x1000 | 0x20);
        assert_eq!(e.call(0x0063_b9b0, &args![sky]).f32(), 6.0);
        // The flag is cleared and the value cached.
        assert_eq!(e.get(sky, Sky::uiFlags), 0x20);
        assert_eq!(e.global::<f32>(SUNRISE_BEGIN_CACHE), 6.0);
        e.register(SKY_SUNRISE_BEGIN, |_, _| panic!("recomputed"));
        assert_eq!(e.call(0x0063_b9b0, &args![sky]).f32(), 6.0);
        // A start before the transition margin is clamped to 0.
        e.register(SKY_SUNRISE_BEGIN, |_, _| ret_float(0.5));
        e.set(sky, Sky::uiFlags, 0x1000);
        assert_eq!(e.call(0x0063_b9b0, &args![sky]).f32(), 0.0);
    }

    #[test]
    fn sunset_end_is_cached_and_limited() {
        let mut e = with_data(engine());
        e.register(MIN_FLOAT, min_float);
        e.register(SETTING_FLOAT_VALUE, |_, a| ret(a[0]));
        e.register(SKY_SUNSET_END, |_, _| ret_float(21.0));
        e.set_global(TIME_SETTING, 2.0f32);
        e.set_global(0x0104_ede4, 22.0f32);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::uiFlags, 0x2000);
        // 21 + 2 is limited to 22.
        assert_eq!(e.call(0x0063_ba30, &args![sky]).f32(), 22.0);
        assert_eq!(e.get(sky, Sky::uiFlags), 0);
        e.register(SKY_SUNSET_END, |_, _| ret_float(17.0));
        e.set(sky, Sky::uiFlags, 0x2000);
        assert_eq!(e.call(0x0063_ba30, &args![sky]).f32(), 19.0);
        // Without the flag the cache is returned.
        e.register(SKY_SUNSET_END, |_, _| panic!("recomputed"));
        assert_eq!(e.call(0x0063_ba30, &args![sky]).f32(), 19.0);
    }

    /// A weather-sized block whose colour rows hold `row * 100 + column`.
    fn weather_with_colors(e: &mut Engine) -> Ptr {
        let weather = e.mem.alloc(0x400);
        for row in 0..6u32 {
            for column in 0..6u32 {
                e.mem.set_u32(
                    weather + 0x108 + row * 0x18 + column * 4,
                    row * 100 + column,
                );
            }
        }
        Ptr::new(weather)
    }

    #[test]
    fn weather_colour_accessor_indexes_rows_and_columns() {
        let mut e = engine();
        let weather = weather_with_colors(&mut e);
        assert_eq!(e.call(0x0063_bb40, &args![weather, 3i32, 2i32]).u32(), 302);
        assert_eq!(e.call(0x0063_bb40, &args![weather, 0i32, 5i32]).u32(), 5);
    }

    #[test]
    fn weather_colours_are_read_for_both_weathers() {
        let mut e = engine();
        let weather_a = weather_with_colors(&mut e);
        let weather_b = weather_with_colors(&mut e);
        e.mem.set_u32(weather_b.addr() + 0x108 + 2 * 0x18 + 4, 999);
        let out = e.mem.alloc(16);
        let indices = e.mem.alloc(8);
        e.mem.set_u32(indices, 1);
        e.mem.set_u32(indices + 4, 4);
        let call = |e: &mut Engine, a: Ptr, b: Ptr| {
            e.call(
                0x0063_bab0,
                &args![0x1234u32, out, a, b, 2i32, indices, indices + 4],
            );
            [
                e.mem.u32(out),
                e.mem.u32(out + 4),
                e.mem.u32(out + 8),
                e.mem.u32(out + 12),
            ]
        };
        // Row 2, columns 1 and 4 of both weathers (weather B differs at column 1).
        assert_eq!(call(&mut e, weather_a, weather_b), [201, 204, 999, 204]);
        // Without the second weather its half is zero.
        e.mem.set_u32(out + 8, 5);
        assert_eq!(call(&mut e, weather_a, Ptr::NULL), [201, 204, 0, 0]);
        // Without the first weather nothing is written.
        e.mem.set_u32(out, 77);
        call(&mut e, Ptr::NULL, weather_b);
        assert_eq!(e.mem.u32(out), 77);
    }

    #[test]
    fn cloud_colour_accessor_uses_defaults_and_layers() {
        let mut e = engine();
        let weather: u32 = e.mem.alloc(0x400);
        // No layers: the built-in colours.
        for (time, expected) in [(1, 0x9b9b9b), (3, 0x1c140f), (0, 0x5b4b46), (2, 0x6c5953)] {
            assert_eq!(
                e.call(0x0063_bc60, &args![weather, 0i32, time]).u32(),
                expected
            );
        }
        // Two layers of colours at +0x68, 0x18 apart.
        e.mem.set_i32(weather + 0x368, 2);
        e.mem.set_u32(weather + 0x68 + 4, 0x111111);
        e.mem.set_u32(weather + 0x68 + 0x18 + 8, 0x222222);
        assert_eq!(
            e.call(0x0063_bc60, &args![weather, 0i32, 1i32]).u32(),
            0x111111
        );
        assert_eq!(
            e.call(0x0063_bc60, &args![weather, 1i32, 2i32]).u32(),
            0x222222
        );
        // A layer past the count uses layer 0.
        assert_eq!(
            e.call(0x0063_bc60, &args![weather, 7i32, 1i32]).u32(),
            0x111111
        );
    }

    #[test]
    fn cloud_colours_fall_back_to_the_other_weather() {
        let mut e = engine();
        let weather_a = e.mem.alloc(0x400);
        let weather_b = e.mem.alloc(0x400);
        for weather in [weather_a, weather_b] {
            e.mem.set_i32(weather + 0x368, 1);
        }
        // Weather A's colours: [1] = 0x10, rest 0; weather B's: [0] = 0x20.
        e.mem.set_u32(weather_a + 0x68 + 4, 0x10);
        e.mem.set_u32(weather_b + 0x68, 0x20);
        let out = e.mem.alloc(16);
        let indices = e.mem.alloc(8);
        e.mem.set_u32(indices, 0);
        e.mem.set_u32(indices + 4, 1);
        e.call(
            0x0063_bb70,
            &args![0u32, out, weather_a, weather_b, 0i32, indices, indices + 4],
        );
        // out = [A0, A1, B0, B1] = [0, 0x10, 0x20, 0]; zero entries are
        // replaced by the entry of the other weather at the same slot.
        assert_eq!(
            [
                e.mem.u32(out),
                e.mem.u32(out + 4),
                e.mem.u32(out + 8),
                e.mem.u32(out + 12)
            ],
            [0x20, 0x10, 0x20, 0x10]
        );
        // Without the second weather its half is zero and nothing is replaced.
        e.call(
            0x0063_bb70,
            &args![0u32, out, weather_a, 0u32, 0i32, indices, indices + 4],
        );
        assert_eq!(
            [
                e.mem.u32(out),
                e.mem.u32(out + 4),
                e.mem.u32(out + 8),
                e.mem.u32(out + 12)
            ],
            [0, 0x10, 0, 0]
        );
        // Without the first weather nothing is written.
        e.mem.set_u32(out, 77);
        e.call(
            0x0063_bb70,
            &args![0u32, out, 0u32, weather_b, 0i32, indices, indices + 4],
        );
        assert_eq!(e.mem.u32(out), 77);
    }

    fn color_at(e: &Engine, address: u32) -> [f32; 3] {
        [
            e.mem.f32(address),
            e.mem.f32(address + 4),
            e.mem.f32(address + 8),
        ]
    }

    fn set_color(e: &mut Engine, address: u32, color: [f32; 3]) {
        for (i, value) in color.iter().enumerate() {
            e.mem.set_f32(address + 4 * i as u32, *value);
        }
    }

    #[test]
    fn colours_in_mode_0_are_the_fixed_ones_and_chain_down() {
        let mut e = with_color_helpers(with_data(engine()));
        e.register(CLOUDS_LAYER_COUNT, |_, _| ret(2));
        for (address, value) in [
            (0x0101_ffc0, 0.9f32),
            (0x0101_9de0, 0.8),
            (0x0102_1f44, 0.3),
            (0x0104_ee30, 0.4),
            (0x0101_e2bc, 0.1),
        ] {
            e.set_global(address, value);
        }
        e.set_global(0x011f_4998, 0.25f32);
        e.set_global(0x011f_499c, 0.5f32);
        e.set_global(0x011f_49a0, 0.75f32);
        let sky = new_sky(&mut e);
        let clouds = e.mem.alloc(0x5c);
        e.set(sky, Sky::pClouds, Ptr::new(clouds));
        e.call(0x0063_b120, &args![sky]);
        let base = sky.addr();
        assert_eq!(color_at(&e, base + 0x6c), [0.9, 0.8, 0.8]);
        assert_eq!(color_at(&e, base + 0x60), [0.3, 0.3, 0.4]);
        assert_eq!(color_at(&e, clouds + 0x28), [0.1, 0.1, 0.1]);
        assert_eq!(color_at(&e, clouds + 0x34), [0.1, 0.1, 0.1]);
        // The exe's colour is copied down the chain.
        for offset in [0xc0, 0x9c, 0x78, 0x3c, 0x48, 0x90] {
            assert_eq!(
                color_at(&e, base + offset),
                [0.25, 0.5, 0.75],
                "{offset:#x}"
            );
        }
    }

    #[test]
    fn colours_in_an_interior_come_from_the_cell() {
        let mut e = with_color_helpers(with_data(engine()));
        e.register(CLOUDS_LAYER_COUNT, |_, _| ret(1));
        e.set_global(INTERIOR_CELL_HOLDER, 0x5000u32);
        e.register(GET_INTERIOR_CELL, |_, _| ret(0x6000));
        e.register(CELL_COLOR_FIRST, |e, a| {
            set_color(e, a[1], [1.0, 2.0, 3.0]);
            Ret::default()
        });
        e.register(CELL_COLOR_SECOND, |e, a| {
            set_color(e, a[1], [4.0, 5.0, 6.0]);
            Ret::default()
        });
        e.register(CELL_COLOR_THIRD, |e, a| {
            set_color(e, a[1], [7.0, 8.0, 9.0]);
            Ret::default()
        });
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 1);
        let clouds = e.mem.alloc(0x5c);
        e.set(sky, Sky::pClouds, Ptr::new(clouds));
        e.call(0x0063_b120, &args![sky]);
        let base = sky.addr();
        assert_eq!(color_at(&e, base + 0x6c), [1.0, 2.0, 3.0]);
        assert_eq!(color_at(&e, base + 0x60), [4.0, 5.0, 6.0]);
        for offset in [0x48, 0xc0, 0x9c, 0x78, 0x3c, 0x90] {
            assert_eq!(color_at(&e, base + offset), [7.0, 8.0, 9.0], "{offset:#x}");
        }
        assert_eq!(color_at(&e, clouds + 0x28), [7.0, 8.0, 9.0]);
    }

    #[test]
    fn colours_with_a_weather_mix_the_weather_table() {
        let mut e = with_color_helpers(with_data(engine()));
        e.register(SKY_SUNRISE_END, |_, _| ret_float(10.0));
        e.register(SKY_SUNSET_BEGIN, |_, _| ret_float(18.0));
        e.set_global(SUNRISE_BEGIN_CACHE, 6.0f32);
        e.set_global(SUNSET_END_CACHE, 22.0f32);
        e.set_global(0x0102_31e8, 1.0f64 / 255.0);
        // Every colour of the weather is white.
        let weather = e.mem.alloc(0x400);
        for row in 0..10 {
            for column in 0..6 {
                e.mem
                    .set_u32(weather + 0x108 + row * 0x18 + column * 4, 0x00ff_ffff);
            }
        }
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(weather));
        e.set(sky, Sky::eMode, 3);
        e.set(sky, Sky::fCurrentGameHour, 12.0);
        e.set(sky, Sky::fHighNoon, 13.0);
        e.set(sky, Sky::fCurrentWeatherPct, 1.0);
        e.call(0x0063_b120, &args![sky]);
        for color in 0..10 {
            let value = color_at(&e, sky.addr() + 0x3c + color * 12);
            for channel in value {
                assert!((channel - 1.0).abs() < 1e-5, "{color}: {value:?}");
            }
        }
        // The specular colour is a copy of colour 3.
        assert_eq!(
            color_at(&e, sky.addr() + 0xc0),
            color_at(&e, sky.addr() + 0x6c)
        );
    }

    #[test]
    fn colours_with_a_weather_add_lightning_to_colour_4_and_the_clouds() {
        let mut e = with_color_helpers(with_data(engine()));
        e.register(SKY_SUNRISE_END, |_, _| ret_float(10.0));
        e.register(SKY_SUNSET_BEGIN, |_, _| ret_float(18.0));
        e.register(CLOUDS_LAYER_COUNT, |_, _| ret(1));
        e.register(WEATHER_BYTE_FRACTION, |_, _| ret_float(2.0));
        e.set_global(SUNRISE_BEGIN_CACHE, 6.0f32);
        e.set_global(SUNSET_END_CACHE, 22.0f32);
        e.set_global(0x0102_31e8, 0.0f64);
        e.set_global(LIGHTNING_SCALE_COLOR_3, 0.5f32);
        e.set_global(LIGHTNING_SCALE_COLOR_4, 0.25f32);
        let weather = e.mem.alloc(0x400);
        let clouds = e.mem.alloc(0x5c);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pClouds, Ptr::new(clouds));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(weather));
        e.set(sky, Sky::eMode, 3);
        e.set(sky, Sky::fCurrentGameHour, 12.0);
        e.set(sky, Sky::fHighNoon, 13.0);
        e.set(sky, Sky::fFlash, 0.5);
        e.call(0x0063_b120, &args![sky]);
        // The channels are zero before lightning; colour 3 gets 0.5 * 0.5,
        // colour 4 gets 0.5 * 0.25, the first cloud layer gets the flash.
        assert_eq!(color_at(&e, sky.addr() + 0x3c + 3 * 12), [0.25; 3]);
        assert_eq!(color_at(&e, sky.addr() + 0x3c + 4 * 12), [0.125; 3]);
        assert_eq!(color_at(&e, sky.addr() + 0x3c + 2 * 12), [0.0; 3]);
        assert_eq!(color_at(&e, clouds + 0x28), [0.5; 3]);
    }

    /// Doubles for `Sky::fn_0063bce0` and a sky in mode 3 whose camera is at
    /// height 50 and whose water volume is absent.
    fn fog_engine() -> Engine {
        let mut e = with_color_helpers(with_data(engine()));
        install_vtable(&mut e);
        e.set_global(PLAYER_CHARACTER, 0x1000u32);
        e.register(PLAYER_PARENT_CELL, |_, _| ret(0x2000));
        e.register(FLAG_UNDERWATER_A, |_, _| ret(0));
        e.register(FLAG_UNDERWATER_B, |_, _| ret(0));
        e.register(CAMERA_HOLDER_GET, |e, _| {
            let camera = object_with_vtable(e, 0x20);
            ret(camera.addr())
        });
        e.register(CAMERA_NODE, |_, _| ret(0x3100));
        e.register(NODE_WORLD_POSITION, |e, _| {
            let position = e.mem.alloc(16);
            e.mem.set_f32(position + 8, 50.0);
            ret(position)
        });
        // The camera-relative tail is skipped by default.
        e.set_global(0x0119_f188, 1u8);
        e
    }

    #[test]
    fn fog_underwater_comes_from_the_water_volume() {
        let mut e = fog_engine();
        e.register(FLAG_UNDERWATER_A, |_, _| ret(1));
        e.register(CELL_WATER_HEIGHT, |_, _| ret_float(80.0));
        e.register(WATER_FOG_NEAR, |_, _| ret_float(10.0));
        e.register(WATER_FOG_FAR, |_, _| ret_float(500.0));
        e.register(WATER_COLOR_A, |_, _| ret(0x0000_00ff));
        e.register(WATER_COLOR_B, |_, _| ret(0));
        e.register(CLOUDS_LAYER_COUNT, |_, _| ret(1));
        e.register(SKY_GET_CLOUDS, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        e.register(SKY_GET_ATMOSPHERE, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(SKY_GET_SUN, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(HOLDER_NODE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SUN_GET_NODE, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(SET_NODE_HIDDEN, |_, _| Ret::default());
        e.set_global(0x011c_7a3c, 0x7000u32);
        e.set_global(0x011c_7a5c, 100.0f32);
        e.set_global(WATER_DEPTH_SCALE, 0.01f32);
        e.set_global(0x0102_31e8, 1.0f64 / 255.0);
        e.register(WATER_FOG_NEAR, |_, a| {
            assert_eq!(a[0], 0x7000);
            ret_float(10.0)
        });
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 3);
        // Components with the node pointer at +8 (+0xc for the sun).
        let (clouds, atmosphere, sun) = (e.mem.alloc(0x60), e.mem.alloc(0x30), e.mem.alloc(0x30));
        e.mem.set_u32(clouds + 8, 0xc100);
        e.mem.set_u32(atmosphere + 8, 0xa100);
        e.mem.set_u32(sun + 8, 0x5100);
        e.mem.set_u32(sun + 0xc, 0x5200);
        e.set(sky, Sky::pClouds, Ptr::new(clouds));
        e.set(sky, Sky::pSun, Ptr::new(sun));
        e.mem.set_u32(sky.addr() + 0x20, atmosphere);
        e.mem.set_u32(sky.addr() + 4, 0x4100);
        e.call_log = Some(vec![]);
        e.call(0x0063_bce0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(sky, Sky::fFogNear), 10.0);
        assert_eq!(e.get(sky, Sky::fFogFar), 500.0);
        assert_eq!(e.get(sky, Sky::fFogPower), 1.0);
        // The camera is 30 below the cell's water: depth 0.3, so colour
        // channel 0 is 0.7 (the water colour is red) and nothing is hidden.
        let color = color_at(&e, sky.addr() + 0x48);
        assert!((color[0] - 0.7).abs() < 1e-6 && color[1] == 0.0 && color[2] == 0.0);
        let hidden: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == SET_NODE_HIDDEN)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(
            hidden,
            [
                vec![0xc100, 0],
                vec![0xa100, 0],
                vec![0x5100, 0],
                vec![0x5200, 0],
                vec![0x4100, 0]
            ]
        );
        // The same, deeper than half way: everything is hidden.
        e.set_global(WATER_DEPTH_SCALE, 0.02f32);
        e.call_log = Some(vec![]);
        e.call(0x0063_bce0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .filter(|(address, _)| *address == SET_NODE_HIDDEN)
            .all(|(_, args)| args[1] == 1));
    }

    #[test]
    fn fog_from_the_weather_mixes_by_the_hour_and_the_last_weather() {
        let mut e = fog_engine();
        e.register(SKY_SUNRISE_END, |_, _| ret_float(10.0));
        e.register(SKY_SUNSET_BEGIN, |_, _| ret_float(18.0));
        e.set_global(SUNRISE_BEGIN_CACHE, 6.0f32);
        e.set_global(SUNSET_END_CACHE, 22.0f32);
        let weather = e.mem.alloc(0x400);
        let last = e.mem.alloc(0x400);
        for (index, value) in [100.0f32, 200.0, 300.0, 400.0, 2.0, 4.0].iter().enumerate() {
            e.mem.set_f32(weather + 0xf0 + 4 * index as u32, *value);
        }
        for (index, value) in [40.0f32, 0.0, 40.0, 0.0, 0.0, 0.0].iter().enumerate() {
            e.mem.set_f32(last + 0xf0 + 4 * index as u32, *value);
        }
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(weather));
        e.set(sky, Sky::eMode, 3);
        // Midday: the daytime values (indices 0, 1, 4).
        e.set(sky, Sky::fCurrentGameHour, 12.0);
        e.call(0x0063_bce0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fFogNear), 100.0);
        assert_eq!(e.get(sky, Sky::fFogFar), 200.0);
        assert_eq!(e.get(sky, Sky::fFogPower), 2.0);
        // 8 o'clock is half way through the sunrise: the mean of both.
        e.set(sky, Sky::fCurrentGameHour, 8.0);
        e.call(0x0063_bce0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fFogNear), 200.0);
        assert_eq!(e.get(sky, Sky::fFogFar), 300.0);
        assert_eq!(e.get(sky, Sky::fFogPower), 3.0);
        // Half way into a change of weather the last weather's mix (near 40,
        // others 0) is added with weight 0.5, the new weather's halved.
        e.set(sky, Sky::pLastWeather, Ptr::new(last));
        e.set(sky, Sky::fCurrentWeatherPct, 0.5);
        e.call(0x0063_bce0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fFogNear), 120.0);
        assert_eq!(e.get(sky, Sky::fFogFar), 150.0);
        assert_eq!(e.get(sky, Sky::fFogPower), 1.5);
        // At night the nighttime values apply.
        e.set(sky, Sky::pLastWeather, Ptr::NULL);
        e.set(sky, Sky::fCurrentGameHour, 23.0);
        e.call(0x0063_bce0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fFogNear), 300.0);
        assert_eq!(e.get(sky, Sky::fFogPower), 4.0);
    }

    #[test]
    fn fog_in_an_interior_comes_from_the_cell_with_limits() {
        let mut e = fog_engine();
        e.set_global(INTERIOR_CELL_HOLDER, 0x5000u32);
        e.set_global(DEFAULT_FOG_DISTANCE, 163840.0f32);
        e.set_global(FOG_FAR_LIMIT, 163840.0f64);
        e.set_global(FOG_NEAR_FACTOR, 0.5f64);
        e.register(GET_INTERIOR_CELL, |_, _| ret(0x6000));
        e.register(CELL_FOG_FAR, |_, _| ret_float(300.0));
        e.register(CELL_FOG_NEAR, |_, _| ret_float(50.0));
        e.register(CELL_FOG_POWER, |_, _| ret_float(1.5));
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 1);
        e.call(0x0063_bce0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fFogNear), 50.0);
        assert_eq!(e.get(sky, Sky::fFogFar), 300.0);
        assert_eq!(e.get(sky, Sky::fFogPower), 1.5);
        // A far distance of 0 becomes the default; a near distance of 0, or
        // past the far one, becomes the far one times the factor.
        e.register(CELL_FOG_FAR, |_, _| ret_float(0.0));
        e.register(CELL_FOG_NEAR, |_, _| ret_float(0.0));
        e.call(0x0063_bce0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fFogFar), 163840.0);
        assert_eq!(e.get(sky, Sky::fFogNear), 81920.0);
        // Past the limit also becomes the default.
        e.register(CELL_FOG_FAR, |_, _| ret_float(200000.0));
        e.register(CELL_FOG_NEAR, |_, _| ret_float(1000.0));
        e.call(0x0063_bce0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fFogFar), 163840.0);
        assert_eq!(e.get(sky, Sky::fFogNear), 1000.0);
    }

    #[test]
    fn fog_defaults_and_the_camera_tail() {
        let mut e = fog_engine();
        e.set_global(DEFAULT_FOG_DISTANCE, 1000.0f32);
        e.set_global(FOG_SETTING, 10.0f32);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 2);
        e.call(0x0063_bce0, &args![sky]);
        assert_eq!(
            (e.get(sky, Sky::fFogNear), e.get(sky, Sky::fFogFar)),
            (1000.0, 1000.0)
        );
        assert_eq!(e.get(sky, Sky::fFogPower), 1.0);
        // Without the flag byte the far distance is pulled to the camera's
        // far plane (slot 0x100 returns 400) and the near one rescaled.
        e.set_global(0x0119_f188, 0u8);
        e.register(SLOT_BASE + 0x100, |_, _| ret_float(400.0));
        e.call(0x0063_bce0, &args![sky]);
        let ratio = ((1000.0f64 - 400.0) / (1000.0 - 10.0)) as f32;
        let pulled = (1000.0f64 - (1000.0 - 10.0) * ratio as f64) as f32;
        assert_eq!(e.get(sky, Sky::fFogFar), 400.0);
        assert_eq!(e.get(sky, Sky::fFogNear), pulled.min(400.0));
    }

    #[test]
    fn weather_fog_accessor_reads_the_table() {
        let mut e = engine();
        let weather = e.mem.alloc(0x400);
        e.mem.set_f32(weather + 0xf0 + 4 * 3, 7.5);
        assert_eq!(e.call(0x0063_c420, &args![weather, 3i32]).f32(), 7.5);
    }

    #[test]
    fn clouds_layer_node_accessor_reads_the_pointer() {
        let mut e = engine();
        let clouds = e.mem.alloc(0x5c);
        e.mem.set_u32(clouds + 8 + 4 * 2, 0xbeef);
        assert_eq!(e.call(0x0063_c440, &args![clouds, 2i32]).u32(), 0xbeef);
    }

    #[test]
    fn global_accessors_return_their_globals() {
        let mut e = with_data(engine());
        e.set_global(0x0119_f188, 1u8);
        assert!(e.call(0x0063_c460, &[]).bool());
        e.set_global(0x0119_f188, 0u8);
        assert!(!e.call(0x0063_c460, &[]).bool());
        e.set_global(0x011c_7a3c, 0x1264u32);
        assert_eq!(e.call(0x0063_c470, &[]).u32(), 0x1264);
        e.set_global(0x011c_7a5c, 2.5f32);
        assert_eq!(e.call(0x0063_c480, &[]).f32(), 2.5);
    }

    fn wind_engine() -> Engine {
        let mut e = with_data(engine());
        e.register(WEATHER_BYTE_FRACTION, |_, a| {
            ret_float(if a[0] == 0x1000 { 0.6 } else { 0.2 })
        });
        e.register(0x0045_2e70, |_, _| ret_float(3600.0));
        e.register(0x0057_e960, |_, _| ret_float(0.5));
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0).trunc() as i32 as u32));
        e.set_global(0x0101_2640, 3600.0f64);
        e.set_global(0x0104_eea0, std::f64::consts::TAU);
        e.set_global(0x0101_e580, 15.0f32);
        e.set_global(0x0104_ee98, 0.7f64);
        e.set_global(0x0102_8338, 512.0f64);
        e.set_global(0x011c_cc50, 1.0f32);
        e.set_global(0x011a_9b50, 11u32);
        e.set_global(0x011a_9b54, 22u32);
        e
    }

    #[test]
    fn wind_follows_the_weather_and_publishes_the_gusts() {
        let mut e = wind_engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x1000));
        e.set(sky, Sky::eMode, 3);
        // The phase is 2 pi * 15, which indexes the table at 126.
        e.set_global(0x011f_52e0 + 4 * 126, 0.25f32);
        e.call(0x0063_c490, &args![sky]);
        assert_eq!(e.get(sky, Sky::fWindSpeed), 0.6);
        assert_eq!(e.global::<f32>(0x011a_d824), 0.6);
        assert_eq!(e.global::<u32>(0x011f_95dc), 11);
        assert_eq!(e.global::<u32>(0x011f_95e0), 22);
        assert_eq!(e.global::<f32>(0x011f_d884), 3.75);
        assert_eq!(e.global::<f32>(0x011f_d888), 7.5);
        // With a last weather the speeds are mixed by the percentage:
        // 0.6 * 0.5 + 0.2 * 0.5.
        e.set(sky, Sky::pLastWeather, Ptr::new(0x2000));
        e.set(sky, Sky::fCurrentWeatherPct, 0.5);
        e.call(0x0063_c490, &args![sky]);
        assert!((e.get(sky, Sky::fWindSpeed) - 0.4).abs() < 1e-6);
    }

    #[test]
    fn wind_does_nothing_without_a_weather_or_in_the_wrong_mode() {
        let mut e = wind_engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 3);
        e.call_log = Some(vec![]);
        e.call(0x0063_c490, &args![sky]);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x1000));
        e.set(sky, Sky::eMode, 1);
        e.call(0x0063_c490, &args![sky]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        assert_eq!(e.get(sky, Sky::fWindSpeed), 0.0);
    }

    #[test]
    fn sine_lookup_indexes_the_table() {
        let mut e = wind_engine();
        e.set_global(0x011c_cc50, 2.0f32);
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0).trunc() as i32 as u32));
        e.set_global(0x011f_52e0 + 4 * 256, 0.75f32);
        // 512 / 2 * 1.0 = 256.
        assert_eq!(e.call(0x0063_c600, &args![1.0f32]).f32(), 0.75);
        // The index wraps at 512.
        e.set_global(0x011f_52e0 + 4 * 6, 0.5f32);
        assert_eq!(e.call(0x0063_c600, &args![2.0f32 + 3.0 / 128.0]).f32(), 0.5);
    }

    #[test]
    fn wind_helpers_store_their_arguments() {
        let mut e = with_data(engine());
        e.call(0x0063_c640, &args![5u32, 6u32]);
        assert_eq!(e.global::<u32>(0x011f_95dc), 5);
        assert_eq!(e.global::<u32>(0x011f_95e0), 6);
        e.call(0x0063_c660, &args![1.5f32]);
        assert_eq!(e.global::<f32>(0x011a_d824), 1.5);
        e.call(0x0063_c670, &args![2.5f32, 3.5f32]);
        assert_eq!(e.global::<f32>(0x011f_d884), 2.5);
        assert_eq!(e.global::<f32>(0x011f_d888), 3.5);
    }

    /// A mixing source: four packed colours and four weights.
    fn mixing_source(e: &mut Engine, colors: [u32; 4], weights: [f32; 4]) -> u32 {
        let source = e.mem.alloc(0x20);
        for i in 0..4 {
            e.mem.set_u32(source + 4 * i, colors[i as usize]);
            e.mem.set_f32(source + 0x10 + 4 * i, weights[i as usize]);
        }
        source
    }

    #[test]
    fn colour_mixing_weights_each_channel_and_clamps_to_one() {
        let mut e = with_color_helpers(with_data(engine()));
        e.set_global(0x0102_31e8, 0.5f64);
        let source = mixing_source(
            &mut e,
            [0x0003_0201, 0x0006_0504, 0, 0],
            [0.5, 0.25, 0.0, 0.0],
        );
        let sky = new_sky(&mut e);
        let dest = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x0063_c690, &args![sky, dest, source, 0.0f32]);
        let log = e.call_log.take().unwrap();
        // Red: (1 * 0.5 + 4 * 0.25) * 0.5 = 0.75; green 1.125 and blue 1.5
        // are clamped to 1.
        assert_eq!(color_at(&e, dest), [0.75, 1.0, 1.0]);
        assert!(log.contains(&(COLOR_CLAMP_TO_ONE, vec![dest])));
    }

    #[test]
    fn colour_mixing_adds_lightning_and_clamps_to_the_weather() {
        let mut e = with_color_helpers(with_data(engine()));
        e.set_global(0x0102_31e8, 0.5f64);
        e.register(WEATHER_BYTE_FRACTION, |_, a| {
            assert_eq!((a[2], a[3]), (1.0f32.to_bits(), 0));
            ret_float(match a[1] {
                0xc => 0.9,
                0xd => 1.2,
                _ => 2.0,
            })
        });
        let source = mixing_source(
            &mut e,
            [0x0003_0201, 0x0006_0504, 0, 0],
            [0.5, 0.25, 0.0, 0.0],
        );
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x1000));
        let dest = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        // Lightning above 1 is limited to 1: [1.75, 2.125, 2.5], then the
        // channels are limited to the weather's 0.9, 1.2 and 2.0.
        e.call(0x0063_c690, &args![sky, dest, source, 3.0f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(color_at(&e, dest), [0.9, 1.2, 2.0]);
        assert!(!addresses(&log).contains(&COLOR_CLAMP_TO_ONE));
        // Without a weather the lightning is added and clamped to one.
        e.set(sky, Sky::pCurrentWeather, Ptr::NULL);
        e.call(0x0063_c690, &args![sky, dest, source, 0.25f32]);
        assert_eq!(color_at(&e, dest), [1.0, 1.0, 1.0]);
    }

    #[test]
    fn colour_addition_adds_each_channel_and_returns_this() {
        let mut e = engine();
        let (first, second) = (e.mem.alloc(12), e.mem.alloc(12));
        set_color(&mut e, first, [0.5, 1.0, 1.5]);
        set_color(&mut e, second, [0.25, 0.25, 0.25]);
        let back = e.call(0x0063_c8a0, &args![first, second]).u32();
        assert_eq!(back, first);
        assert_eq!(color_at(&e, first), [0.75, 1.25, 1.75]);
        assert_eq!(color_at(&e, second), [0.25, 0.25, 0.25]);
    }

    fn climate_engine() -> Engine {
        let mut e = with_data(engine());
        e.register(LOOKUP_FORM, |_, a| {
            assert_eq!(a, [0x15f]);
            ret(0x9999)
        });
        e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!(
                a,
                [0x9999, 0, TYPE_DESCRIPTOR_FORM, TYPE_DESCRIPTOR_CLIMATE, 0]
            );
            ret(0x4444)
        });
        e.register(SKY_RESET_WEATHER, |_, _| Ret::default());
        e
    }

    #[test]
    fn set_current_climate_looks_up_the_default_climate() {
        let mut e = climate_engine();
        let sky = new_sky(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0063_c8f0, &args![sky, 0u32, false]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(sky, Sky::pCurrentClimate), Ptr::new(0x4444));
        assert_eq!(e.get(sky, Sky::uiFlags), 0x3f40);
        assert!(log.contains(&(SKY_RESET_WEATHER, vec![sky.addr()])));
    }

    #[test]
    fn set_current_climate_keeps_the_same_climate_unless_forced() {
        let mut e = climate_engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        e.call(0x0063_c8f0, &args![sky, 0x1234u32, false]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
        assert_eq!(e.get(sky, Sky::uiFlags), 0);
        // A different climate is taken over.
        e.call(0x0063_c8f0, &args![sky, 0x5678u32, false]);
        assert_eq!(e.get(sky, Sky::pCurrentClimate), Ptr::new(0x5678));
        assert_eq!(e.get(sky, Sky::uiFlags), 0x3f40);
        // Forcing with no climate looks the default up again.
        e.set(sky, Sky::uiFlags, 0);
        e.call_log = Some(vec![]);
        e.call(0x0063_c8f0, &args![sky, 0u32, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(sky, Sky::pCurrentClimate), Ptr::new(0x4444));
        assert!(addresses(&log).contains(&LOOKUP_FORM));
        assert_eq!(e.get(sky, Sky::uiFlags), 0x3f40);
    }

    #[test]
    fn set_current_climate_does_nothing_when_no_climate_is_found() {
        let mut e = climate_engine();
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        let sky = new_sky(&mut e);
        e.call(0x0063_c8f0, &args![sky, 0u32, false]);
        assert!(e.get(sky, Sky::pCurrentClimate).is_null());
        assert_eq!(e.get(sky, Sky::uiFlags), 0);
        // An existing climate and no argument: nothing is looked up.
        e.set(sky, Sky::pCurrentClimate, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        e.call(0x0063_c8f0, &args![sky, 0u32, false]);
        assert!(!addresses(&e.call_log.take().unwrap()).contains(&LOOKUP_FORM));
        assert_eq!(e.get(sky, Sky::pCurrentClimate), Ptr::new(0x1234));
    }

    #[test]
    fn climate_stale_flags_are_set() {
        let mut e = engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::uiFlags, 0x41);
        e.call(0x0063_c9f0, &args![sky]);
        assert_eq!(e.get(sky, Sky::uiFlags), 0x3f41);
    }

    /// A sky in mode 3 with a sun, and a climate with two texture entries,
    /// with doubles for the texture loading.
    fn climate_update_engine() -> Engine {
        let mut e = with_color_helpers(with_data(engine()));
        install_vtable(&mut e);
        e.register(SLOT_BASE + 0x14, |_, _| ret(0x5150));
        e.register(SLOT_BASE + 0x18, |_, _| ret(0x6100));
        e.register(STARS_LOAD_GEOMETRY, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.register(STRING_ASSIGN, |_, _| Ret::default());
        e.register(STRING_APPEND, |_, _| Ret::default());
        e.register(STRING_COMPARE, |_, _| ret(1));
        e.register(SUN_FIRST_NODE, |_, _| ret(0x7100));
        e.register(NODE_GET_PROPERTY, |_, a| {
            assert_eq!(a[1], 3);
            ret(0x8100 + a[0])
        });
        e.register(TEXTURE_ENTRY_VALID, |_, _| ret(1));
        e.register(TEXTURE_ENTRY_NAME, |_, _| ret(0x6200));
        e.register(NI_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(TEXTURE_LOAD, |e, a| {
            e.mem.set_u32(a[2], 0x7700);
            Ret::default()
        });
        e.register(PROPERTY_SET_TEXTURE, |_, _| Ret::default());
        e.register(NODE_SET_FLAG_BIT_20, |_, _| Ret::default());
        e.register(HOLDER_NODE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SUN_GET_NODE, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ROOT_UPDATE_PROPERTIES, |_, _| Ret::default());
        e.register(NI_UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(NODE_UPDATE_CONTROLLERS, |_, _| Ret::default());
        e.register(SETTING_WORD_VALUE, |_, a| ret(a[0]));
        e.register(MOON_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            ret(a[0])
        });
        e
    }

    #[test]
    fn climate_update_does_nothing_without_the_flag() {
        let mut e = climate_update_engine();
        let sky = new_sky(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0063_ca20, &args![sky]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn climate_update_loads_stars_sun_textures_and_creates_the_moons() {
        let mut e = climate_update_engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::uiFlags, 0x40 | 0x100);
        e.set(sky, Sky::eMode, 3);
        // Climate: an embedded object at +0x18 with a vtable, the two texture
        // entries (objects with a vtable) at +0x38 and +0x44, and both moons.
        let climate = e.mem.alloc(0x80);
        e.mem.set_u32(climate + 0x18, VTABLE);
        e.mem.set_u32(climate + 0x38, VTABLE);
        e.mem.set_u32(climate + 0x44, VTABLE);
        e.mem.set_u8(climate + 0x55, 0xc0);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(climate));
        let stars = e.mem.alloc(0x10);
        let sun = e.mem.alloc(0x30);
        e.mem.set_u32(sun + 8, 0x5100);
        e.mem.set_u32(sun + 0xc, 0x5200);
        e.mem.set_u32(sun + 0x14, 0x5300);
        e.set(sky, Sky::pStars, Ptr::new(stars));
        e.set(sky, Sky::pSun, Ptr::new(sun));
        e.mem.set_u32(sky.addr() + 4, 0x4100);
        e.mem.set_u32(sky.addr() + 8, 0x4200);
        e.set_global(0x011c_cc54, 1.0f32);
        e.call_log = Some(vec![]);
        e.call(0x0063_ca20, &args![sky]);
        let log = e.call_log.take().unwrap();
        // The stars get the climate's geometry value.
        assert!(log.contains(&(STARS_LOAD_GEOMETRY, vec![stars, 0x5150])));
        // The first texture (entry 0, climate + 0x38) is built from the
        // entry's prefix and name, loaded and set on the sun's property.
        assert!(log.contains(&(
            STRING_ASSIGN,
            vec![
                log.iter().find(|(a, _)| *a == STRING_CONSTRUCT).unwrap().1[0],
                0x6100
            ]
        )));
        assert!(log.contains(&(
            STRING_APPEND,
            vec![
                log.iter().find(|(a, _)| *a == STRING_CONSTRUCT).unwrap().1[0],
                0x6200
            ]
        )));
        let loads = log.iter().filter(|(a, _)| *a == TEXTURE_LOAD).count();
        assert_eq!(loads, 2);
        let sets: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == PROPERTY_SET_TEXTURE)
            .collect();
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0].1[1], 0x7700);
        // Both sun nodes are flagged as having a texture.
        assert!(log.contains(&(NODE_SET_FLAG_BIT_20, vec![0x5100, 1])));
        assert!(log.contains(&(NODE_SET_FLAG_BIT_20, vec![0x5200, 1])));
        // Both moons are created with the settings and initialized with the
        // moons root and their names (slot 0x10).
        let masser = e.get(sky, Sky::pMasser);
        let secunda = e.get(sky, Sky::pSecunda);
        assert!(!masser.is_null() && !secunda.is_null());
        assert_eq!(e.mem.block_size(masser.addr()), Some(0x80));
        assert!(log.contains(&(SLOT_BASE + 0x10, vec![masser.addr(), 0x4200, 0x0104_eeb0])));
        assert!(log.contains(&(SLOT_BASE + 0x10, vec![secunda.addr(), 0x4200, 0x0104_eea8])));
        let masser_construct = log
            .iter()
            .find(|(a, args)| *a == MOON_CONSTRUCT && args[1] == 0x0104_eeb0)
            .unwrap();
        // The name, the five float settings in reverse order and the word.
        assert_eq!(masser_construct.1.len(), 8);
        assert_eq!(
            masser_construct.1[2..7].iter().filter(|w| **w == 0).count(),
            4
        );
        assert_eq!(masser_construct.1[6], 1.0f32.to_bits());
        // The flag is cleared and the root's controllers updated.
        assert_eq!(e.get(sky, Sky::uiFlags), 0x100);
        assert!(log.contains(&(ROOT_UPDATE_PROPERTIES, vec![0x4100])));
        assert!(addresses(&log).contains(&NODE_UPDATE_CONTROLLERS));
    }

    #[test]
    fn climate_update_uses_the_non_hdr_glare_texture_and_deletes_old_moons() {
        let mut e = climate_update_engine();
        e.register(STRING_COMPARE, |_, a| {
            assert_eq!((a[0], a[1]), (0x6200, SUN_GLARE_NAME));
            ret(0)
        });
        let sky = new_sky(&mut e);
        e.set(sky, Sky::uiFlags, 0x40);
        e.set(sky, Sky::eMode, 2);
        let climate = e.mem.alloc(0x80);
        e.mem.set_u32(climate + 0x38, VTABLE);
        e.mem.set_u32(climate + 0x44, VTABLE);
        e.mem.set_u8(climate + 0x55, 0);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(climate));
        let sun = e.mem.alloc(0x30);
        e.mem.set_u32(sun + 8, 0x5100);
        e.mem.set_u32(sun + 0xc, 0x5200);
        e.mem.set_u32(sun + 0x14, 0x5300);
        e.set(sky, Sky::pSun, Ptr::new(sun));
        let masser = object_with_vtable(&mut e, 0x80);
        let secunda = object_with_vtable(&mut e, 0x80);
        e.set(sky, Sky::pMasser, masser);
        e.set(sky, Sky::pSecunda, secunda);
        e.mem.set_u32(sky.addr() + 4, 0x4100);
        e.call_log = Some(vec![]);
        e.call(0x0063_ca20, &args![sky]);
        let log = e.call_log.take().unwrap();
        // Entry 1 is the glare texture: with the HDR byte clear and its name
        // equal to the glare name, the non-HDR path is used.
        assert!(log
            .iter()
            .any(|(a, args)| *a == STRING_ASSIGN && args[1] == SUN_GLARE_NON_HDR_PATH));
        // The moons are deleted (slot 0, flag 1) and cleared.
        assert!(log.contains(&(SLOT_BASE, vec![masser.addr(), 1])));
        assert!(log.contains(&(SLOT_BASE, vec![secunda.addr(), 1])));
        assert!(e.get(sky, Sky::pMasser).is_null());
        assert!(e.get(sky, Sky::pSecunda).is_null());
    }

    #[test]
    fn climate_entry_accessor_returns_the_first_two_entries() {
        let mut e = engine();
        assert_eq!(e.call(0x0063_cfa0, &args![0x1000u32, 0i32]).u32(), 0x1038);
        assert_eq!(e.call(0x0063_cfa0, &args![0x1000u32, 1i32]).u32(), 0x1044);
        assert_eq!(e.call(0x0063_cfa0, &args![0x1000u32, 2i32]).u32(), 0);
        assert_eq!(
            e.call(0x0063_cfa0, &args![0x1000u32, -1i32]).u32(),
            0x1000 + 0x38 - 12
        );
    }

    #[test]
    fn climate_virtual_accessor_calls_slot_0x14_of_the_embedded_object() {
        let mut e = engine();
        install_vtable(&mut e);
        e.register(SLOT_BASE + 0x14, |_, a| ret(a[0] + 1));
        let climate = e.mem.alloc(0x40);
        e.mem.set_u32(climate + 0x18, VTABLE);
        assert_eq!(
            e.call(0x0063_cfe0, &args![climate]).u32(),
            climate + 0x18 + 1
        );
    }

    #[test]
    fn climate_moon_flags_read_bits_of_the_byte_at_0x55() {
        let mut e = engine();
        let climate = e.mem.alloc(0x80);
        for (byte, masser, secunda) in [
            (0u8, false, false),
            (0x80, true, false),
            (0x40, false, true),
            (0xc0, true, true),
        ] {
            e.mem.set_u8(climate + 0x55, byte);
            assert_eq!(e.call(0x0063_d000, &args![climate]).bool(), masser);
            assert_eq!(e.call(0x0063_d020, &args![climate]).bool(), secunda);
        }
    }

    #[test]
    fn sun_node_accessor_reads_the_pointer_at_0x14() {
        let mut e = engine();
        let sun = e.mem.alloc(0x30);
        e.mem.set_u32(sun + 0x14, 0xabcd);
        assert_eq!(e.call(0x0063_d040, &args![sun]).u32(), 0xabcd);
    }

    // ---- tests of the weather transition, sound and texture functions ----

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Doubles for the image-space drive: the form for index `i` is
    /// `0x100 + i`, the default form is 1.
    fn hdr_doubles(e: &mut Engine) {
        e.register(IMAGE_SPACE_SET_WEIGHT, |_, _| Ret::default());
        e.register(IMAGE_SPACE_SET_FORM, |_, _| Ret::default());
        e.register(WEATHER_IMAGE_SPACE_FOR_TIME, |_, a| ret(0x100 + a[1]));
        e.register(DEFAULT_IMAGE_SPACE, |_, _| ret(1));
    }

    const INSTANCE_CURRENT: u32 = 0x7101;
    const INSTANCE_CURRENT_2: u32 = 0x7102;
    const INSTANCE_LAST: u32 = 0x7103;
    const INSTANCE_LAST_2: u32 = 0x7104;

    /// A sky in mode 0 whose four image-space instances exist (so that
    /// `Sky::UpdateHDRValues` only sets four weights of 0).
    fn sky_with_instances(e: &mut Engine) -> Ptr<Sky> {
        let sky = new_sky(e);
        e.set(
            sky,
            Sky::pCurrentWeatherImageSpaceMod,
            Ptr::new(INSTANCE_CURRENT),
        );
        e.set(
            sky,
            Sky::pCurrentWeatherImageSpaceMod2,
            Ptr::new(INSTANCE_CURRENT_2),
        );
        e.set(sky, Sky::pLastWeatherImageSpaceMod, Ptr::new(INSTANCE_LAST));
        e.set(
            sky,
            Sky::pLastWeatherImageSpaceMod2,
            Ptr::new(INSTANCE_LAST_2),
        );
        sky
    }

    /// The weights set on `instance`, in order.
    fn weights_of(log: &[(u32, Vec<u32>)], instance: u32) -> Vec<f32> {
        calls_to(log, IMAGE_SPACE_SET_WEIGHT)
            .iter()
            .filter(|args| args[0] == instance)
            .map(|args| f32::from_bits(args[1]))
            .collect()
    }

    fn weather_engine() -> Engine {
        let mut e = with_data(engine());
        hdr_doubles(&mut e);
        e.set_global(WORLD_STATE_POINTER, 0x9000u32);
        e.register(WORLD_STATE_FLAG, |_, _| ret(0));
        e.set_global(PLAYER_CHARACTER, 0x7000u32);
        e.register(PLAYER_WEATHER_OWNER, |_, _| ret(0));
        e.register(WEATHER_OF_OWNER, |_, _| ret(0));
        e.register(CLIMATE_UPDATE_BYTE, |_, _| ret(255));
        e.register(SETTING_FLOAT_VALUE, |_, a| ret(a[0]));
        e.register(PRECIPITATION_FLUSH_ALL, |_, _| Ret::default());
        e.register(CLOUDS_CLEAR_TRANS_TEXTURES, |_, _| Ret::default());
        e.set_global(REGION_HOLDER, 0x8000u32);
        e.register(REGION_LIST_OWNER, |_, _| ret(0));
        e.register(REGION_UPDATE_WEATHER, |_, _| Ret::default());
        e.register(CLIMATE_PICK_WEATHER, |_, _| ret(0x4000));
        e
    }

    #[test]
    fn reset_weather_forgets_the_weathers_and_clears_the_effects() {
        let mut e = weather_engine();
        let sky = sky_with_instances(&mut e);
        e.set(sky, Sky::uiFlags, 8);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x100));
        e.set(sky, Sky::pLastWeather, Ptr::new(0x200));
        e.set(sky, Sky::pOverrideWeather, Ptr::new(0x300));
        e.set(sky, Sky::fAccelBeginPct, 0.3);
        e.set(sky, Sky::pClouds, Ptr::new(0x5000));
        e.set(sky, Sky::pPrecip, Ptr::new(0x6000));
        e.call_log = Some(vec![]);
        e.call(0x0063_d060, &args![sky]);
        // Flag 8 (acceleration) is cleared with its start value, flag 1 set.
        assert_eq!(e.get(sky, Sky::uiFlags), 1);
        assert_eq!(e.get(sky, Sky::fAccelBeginPct), 0.0);
        assert!(e.get(sky, Sky::pCurrentWeather).is_null());
        assert!(e.get(sky, Sky::pLastWeather).is_null());
        assert!(e.get(sky, Sky::pOverrideWeather).is_null());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log)[..3],
            [
                0x0063_d060,
                CLOUDS_CLEAR_TRANS_TEXTURES,
                PRECIPITATION_FLUSH_ALL
            ]
        );
        assert!(log.contains(&(CLOUDS_CLEAR_TRANS_TEXTURES, vec![0x5000])));
        assert!(log.contains(&(PRECIPITATION_FLUSH_ALL, vec![0x6000])));
        // The image-space values are recomputed (mode 0: four weights of 0).
        assert_eq!(calls_to(&log, IMAGE_SPACE_SET_WEIGHT).len(), 4);

        // Without clouds and precipitation neither is touched.
        let sky = sky_with_instances(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0063_d060, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&CLOUDS_CLEAR_TRANS_TEXTURES));
        assert!(!addresses(&log).contains(&PRECIPITATION_FLUSH_ALL));
    }

    #[test]
    fn force_weather_makes_the_weather_default_or_override() {
        let mut e = weather_engine();
        e.register(PLAYER_METHOD_0093A7A0, |_, _| Ret::default());
        let sky = sky_with_instances(&mut e);
        e.set(sky, Sky::uiFlags, 0x9);
        e.set(sky, Sky::fCurrentGameHour, 9.5);
        e.set(sky, Sky::pLastWeather, Ptr::new(0x200));
        e.set(sky, Sky::pOverrideWeather, Ptr::new(0x300));
        e.set(sky, Sky::pClouds, Ptr::new(0x5000));
        e.call_log = Some(vec![]);
        e.call(0x0063_d0e0, &args![sky, 0x400u32, false]);
        assert_eq!(e.get(sky, Sky::pDefaultWeather).addr(), 0x400);
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x400);
        assert!(e.get(sky, Sky::pOverrideWeather).is_null());
        assert!(e.get(sky, Sky::pLastWeather).is_null());
        assert_eq!(e.get(sky, Sky::fCurrentWeatherPct), 1.0);
        assert_eq!(e.get(sky, Sky::fLastWeatherUpdate), 9.5);
        // Flag 8 cleared by the acceleration reset, flag 2 set, flag 1 cleared.
        assert_eq!(e.get(sky, Sky::uiFlags), 2);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(PLAYER_METHOD_0093A7A0, vec![0x7000, 0])));
        // The clouds are cleared after the image-space update.
        let last_weight = log
            .iter()
            .rposition(|(a, _)| *a == IMAGE_SPACE_SET_WEIGHT)
            .unwrap();
        let clear = log
            .iter()
            .position(|(a, _)| *a == CLOUDS_CLEAR_TRANS_TEXTURES)
            .unwrap();
        assert!(last_weight < clear);

        e.call(0x0063_d0e0, &args![sky, 0x500u32, true]);
        assert_eq!(e.get(sky, Sky::pOverrideWeather).addr(), 0x500);
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x500);
        assert!(e.get(sky, Sky::pDefaultWeather).is_null());
    }

    #[test]
    fn weather_update_does_nothing_without_a_climate() {
        let mut e = weather_engine();
        let sky = sky_with_instances(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0063_d1d0, &args![sky]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn weather_update_asks_the_climate_and_updates_the_regions() {
        let mut e = weather_engine();
        let sky = sky_with_instances(&mut e);
        let climate = e.mem.alloc(0x100);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(climate));
        e.set(sky, Sky::uiFlags, 1);
        e.set(sky, Sky::fCurrentGameHour, 9.0);
        // The region list: one node `{ region, next = 0 }` at `owner + 4`.
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner + 4, 0x5500);
        e.register_double(REGION_LIST_OWNER, move |_, _| ret(owner));
        e.call_log = Some(vec![]);
        e.call(0x0063_d1d0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(CLIMATE_PICK_WEATHER, vec![climate + 0x30])));
        assert!(log.contains(&(REGION_UPDATE_WEATHER, vec![0x5500])));
        assert!(!addresses(&log).contains(&LOOKUP_FORM));
        // The picked weather is the default and becomes current; there was
        // no current weather, so there is no last one and the blend is 1.
        assert_eq!(e.get(sky, Sky::pDefaultWeather).addr(), 0x4000);
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x4000);
        assert!(e.get(sky, Sky::pLastWeather).is_null());
        assert_eq!(e.get(sky, Sky::fCurrentWeatherPct), 1.0);
        assert_eq!(e.get(sky, Sky::fLastWeatherUpdate), 9.0);
        assert_eq!(e.get(sky, Sky::uiFlags), 1);
        assert_eq!(calls_to(&log, IMAGE_SPACE_SET_WEIGHT).len(), 4);

        // A climate with no weather: the form 0x15e is looked up and cast.
        e.register(CLIMATE_PICK_WEATHER, |_, _| ret(0));
        e.register(LOOKUP_FORM, |_, a| ret(0x3000 + a[0]));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] + 0x1000));
        let sky = sky_with_instances(&mut e);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(climate));
        e.set(sky, Sky::uiFlags, 1);
        e.call_log = Some(vec![]);
        e.call(0x0063_d1d0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(LOOKUP_FORM, vec![0x15e])));
        assert!(log.contains(&(
            DYNAMIC_CAST,
            vec![0x315e, 0, TYPE_DESCRIPTOR_FORM, TYPE_DESCRIPTOR_WEATHER, 0]
        )));
        assert_eq!(e.get(sky, Sky::pDefaultWeather).addr(), 0x415e);
    }

    #[test]
    fn weather_update_blends_the_last_and_current_weather() {
        let mut e = weather_engine();
        e.set_global(0x011c_cc08, 0.0f32);
        e.set_global(0x011c_cc88, 1.0f32);
        e.register(WEATHER_BYTE_FRACTION, |_, _| ret_float(4.0));
        let sky = sky_with_instances(&mut e);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(0x1000));
        e.set(sky, Sky::pDefaultWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pLastWeather, Ptr::new(0x4100));
        e.set(sky, Sky::uiFlags, 1 << 20);
        e.set(sky, Sky::fCurrentGameHour, 10.0);
        e.set(sky, Sky::fLastWeatherUpdate, 8.0);
        e.call_log = Some(vec![]);
        e.call(0x0063_d1d0, &args![sky]);
        let log = e.call_log.take().unwrap();
        // The last weather exists, so the climate is not asked again; the
        // transition length comes from the current weather (index 3, high
        // end, low end).
        assert!(!addresses(&log).contains(&CLIMATE_PICK_WEATHER));
        assert_eq!(
            calls_to(&log, WEATHER_BYTE_FRACTION),
            [[0x4000, 3, 1.0f32.to_bits(), 0.0f32.to_bits()]]
        );
        assert_eq!(e.get(sky, Sky::fCurrentWeatherPct), 0.5);
        assert_eq!(e.get(sky, Sky::uiFlags), 1 << 20);

        // The acceleration flag re-scales the blend:
        // (setting + 1) * (pct - start) + start.
        e.set_global(0x011c_cc28, 1.0f32);
        e.set(sky, Sky::uiFlags, 8);
        e.set(sky, Sky::fAccelBeginPct, 0.25);
        e.call(0x0063_d1d0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fCurrentWeatherPct), 0.75);

        // A blend above 1 ends the transition and the acceleration.
        e.set(sky, Sky::uiFlags, 0);
        e.set(sky, Sky::fCurrentGameHour, 14.0);
        e.call(0x0063_d1d0, &args![sky]);
        assert_eq!(e.get(sky, Sky::fCurrentWeatherPct), 1.0);
        assert!(e.get(sky, Sky::pLastWeather).is_null());
    }

    #[test]
    fn weather_update_wraps_the_day_and_uses_the_override() {
        let mut e = weather_engine();
        e.set_global(0x011c_cc88, 1.0f32);
        e.register(WEATHER_BYTE_FRACTION, |_, _| ret_float(4.0));
        let sky = sky_with_instances(&mut e);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(0x1000));
        e.set(sky, Sky::pDefaultWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pOverrideWeather, Ptr::new(0x4200));
        e.set(sky, Sky::fCurrentGameHour, 1.0);
        e.set(sky, Sky::fLastWeatherUpdate, 23.0);
        e.set(sky, Sky::uiFlags, 1 << 20);
        e.call(0x0063_d1d0, &args![sky]);
        // The override differs from the current weather: the current one
        // becomes the last, flag 1 is set and the update hour is now; the
        // blend is then 0 hours over the transition length.
        assert_eq!(e.get(sky, Sky::pLastWeather).addr(), 0x4000);
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x4200);
        assert_eq!(e.get(sky, Sky::fLastWeatherUpdate), 1.0);
        assert_eq!(e.get(sky, Sky::uiFlags), (1 << 20) | 1);
        assert_eq!(e.get(sky, Sky::fCurrentWeatherPct), 0.0);

        // Fast travel (flag 0x10) drops the last weather instead and
        // flushes the precipitation.
        let sky = sky_with_instances(&mut e);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(0x1000));
        e.set(sky, Sky::pDefaultWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4100));
        e.set(sky, Sky::pPrecip, Ptr::new(0x6000));
        e.set(sky, Sky::uiFlags, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0063_d1d0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(PRECIPITATION_FLUSH_ALL, vec![0x6000])));
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x4000);
        assert!(e.get(sky, Sky::pLastWeather).is_null());

        // Without an override the player's area weather replaces the
        // default one outside mode 2.
        e.register(PLAYER_WEATHER_OWNER, |_, _| ret(0x6500));
        e.register(WEATHER_OF_OWNER, |_, a| {
            assert_eq!(a[0], 0x6500);
            ret(0x4300)
        });
        let sky = sky_with_instances(&mut e);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(0x1000));
        e.set(sky, Sky::pDefaultWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4100));
        e.set(sky, Sky::uiFlags, 0x10);
        e.call(0x0063_d1d0, &args![sky]);
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x4300);
        // Mode 2 keeps the default weather.
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4100));
        e.set(sky, Sky::eMode, 2);
        // (The image-space update of a visible mode reads the climate hours.)
        e.set_global(SUNRISE_BEGIN_CACHE, 6.0f32);
        e.set_global(SUNSET_END_CACHE, 22.0f32);
        e.register(SKY_SUNRISE_END, |_, _| ret_float(8.0));
        e.register(SKY_SUNSET_BEGIN, |_, _| ret_float(18.0));
        e.call(0x0063_d1d0, &args![sky]);
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x4000);
    }

    #[test]
    fn weather_update_with_the_world_flag_only_follows_the_override() {
        let mut e = weather_engine();
        e.register(WORLD_STATE_FLAG, |_, _| ret(1));
        let sky = sky_with_instances(&mut e);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(0x1000));
        e.set(sky, Sky::pDefaultWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4100));
        e.set(sky, Sky::uiFlags, 0x10);
        e.set(sky, Sky::fCurrentGameHour, 5.0);
        // No override: the current weather stays (and flag 1 is cleared).
        e.call(0x0063_d1d0, &args![sky]);
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x4100);
        // An override replaces it; the update hour is not touched while the
        // world flag is set.
        e.set(sky, Sky::pOverrideWeather, Ptr::new(0x4200));
        e.call(0x0063_d1d0, &args![sky]);
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x4200);
        assert_eq!(e.get(sky, Sky::fLastWeatherUpdate), 0.0);
        assert_eq!(e.get(sky, Sky::uiFlags) & 1, 1);
    }

    #[test]
    fn climate_weather_forwarder_passes_the_embedded_list() {
        let mut e = engine();
        e.register(CLIMATE_PICK_WEATHER, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x0063_d5c0, &args![0x1000u32]).u32(), 0x1031);
    }

    #[test]
    fn acceleration_starts_only_with_a_last_weather() {
        let mut e = engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::fCurrentWeatherPct, 0.4);
        // No last weather: the acceleration is cleared.
        e.set(sky, Sky::uiFlags, 8);
        e.set(sky, Sky::fAccelBeginPct, 0.9);
        e.call(0x0063_e860, &args![sky, true]);
        assert_eq!(e.get(sky, Sky::uiFlags), 0);
        assert_eq!(e.get(sky, Sky::fAccelBeginPct), 0.0);
        // A last weather: starts and remembers the blend...
        e.set(sky, Sky::pLastWeather, Ptr::new(0x4100));
        e.call(0x0063_e860, &args![sky, true]);
        assert_eq!(e.get(sky, Sky::uiFlags), 8);
        assert_eq!(e.get(sky, Sky::fAccelBeginPct), 0.4);
        // ...but a running acceleration keeps its start value.
        e.set(sky, Sky::fCurrentWeatherPct, 0.7);
        e.call(0x0063_e860, &args![sky, true]);
        assert_eq!(e.get(sky, Sky::fAccelBeginPct), 0.4);
        // Without `start` it ends.
        e.call(0x0063_e860, &args![sky, false]);
        assert_eq!(e.get(sky, Sky::uiFlags), 0);
        assert_eq!(e.get(sky, Sky::fAccelBeginPct), 0.0);
    }

    #[test]
    fn fast_travel_flag_drops_the_override() {
        let mut e = engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pOverrideWeather, Ptr::new(0x300));
        e.set(sky, Sky::uiFlags, 1);
        e.call(0x0063_e8f0, &args![sky, true]);
        assert!(e.get(sky, Sky::pOverrideWeather).is_null());
        assert_eq!(e.get(sky, Sky::uiFlags), 0x11);
        e.set(sky, Sky::pOverrideWeather, Ptr::new(0x300));
        e.call(0x0063_e8f0, &args![sky, false]);
        assert_eq!(e.get(sky, Sky::pOverrideWeather).addr(), 0x300);
        assert_eq!(e.get(sky, Sky::uiFlags), 1);
    }

    // ---- sound ----

    /// A weather sound: handle id at +0, weather at +0xc, type at +0x10, form
    /// id at +0x14 and data at +0x18.
    fn make_sound(e: &mut Engine, weather: u32, sound_type: u32, form_id: u32) -> u32 {
        let sound = e.mem.alloc(0x1c);
        e.mem.set_u32(sound, 5);
        e.mem.set_u32(sound + 0xc, weather);
        e.mem.set_u32(sound + 0x10, sound_type);
        e.mem.set_u32(sound + 0x14, form_id);
        sound
    }

    /// A list of `{ item, next }` nodes; the head is returned (an empty list
    /// is one empty node).
    fn make_list(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    fn list_items(e: &Engine, head: u32) -> Vec<u32> {
        let mut items = vec![];
        let mut node = head;
        while node != 0 && e.mem.u32(node) != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    /// Doubles for the sound handles (a handle is valid when its id is not
    /// -1; the byte at +4 says it is playing) and the list helpers.
    fn sound_engine() -> Engine {
        let mut e = with_data(engine());
        e.register(SOUND_HANDLE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], u32::MAX);
            ret(a[0])
        });
        e.register(SOUND_HANDLE_ASSIGN, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            ret(a[0])
        });
        e.register(SOUND_HANDLE_DESTRUCT, |_, _| Ret::default());
        e.register(SOUND_HANDLE_RELEASE, |_, _| Ret::default());
        e.register(SOUND_HANDLE_STOP, |_, _| Ret::default());
        e.register(SOUND_HANDLE_PLAY, |_, _| Ret::default());
        e.register(SOUND_HANDLE_IS_VALID, |e, a| {
            ret((e.mem.u32(a[0]) != u32::MAX) as u32)
        });
        e.register(SOUND_HANDLE_IS_PLAYING, |e, a| {
            ret(e.mem.u8(a[0] + 4) as u32)
        });
        e.register(SKY_SOUND_BASE_DESTRUCT, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(LIST_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(LIST_SCALAR_DELETING_DESTRUCTOR, |_, _| Ret::default());
        e.register(LIST_INSERT, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) != 0 {
                while e.mem.u32(node + 4) != 0 {
                    node = e.mem.u32(node + 4);
                }
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
            e.mem.set_u32(node, item);
            Ret::default()
        });
        e.register(SOUNDS_APPLY, |_, _| ret(0));
        e.register(IS_IN_MENU_MODE, |_, _| ret(0));
        e.register(IS_LOADING_MENU_OPEN, |_, _| ret(0));
        e.register(ABS_FLOAT, |_, a| ret_float(f32::from_bits(a[0]).abs()));
        e.register(SOUND_HANDLE_GET_VOLUME, |_, _| ret_float(0.2));
        e.register(SOUND_HANDLE_SET_VOLUME, |_, _| Ret::default());
        e.register(TICK_COUNT_GET, |_, _| ret(1234));
        e.set_global(VOLUME_EPSILON, 0.01f64);
        e
    }

    /// Doubles under `fn_00639c90`: it accepts when `limit < size`.
    fn size_limit(e: &mut Engine, size: u32, limit: u32) {
        e.register_double(0x0045_7fe0, move |_, _| ret(size));
        e.register(0x0063_9ce0, |_, _| ret(0));
        e.register_double(0x0042_f5a0, move |_, _| ret(limit));
        e.register(0x0063_9d00, |_, _| Ret::default());
    }

    #[test]
    fn sounds_marked_for_removal_are_dropped_when_the_list_is_rebuilt() {
        let mut e = sound_engine();
        let weather = 0x4000;
        let marked = make_sound(&mut e, weather, 3, 7);
        e.mem.set_u32(marked + 0x18, 0x8000_0000);
        let kept = make_sound(&mut e, weather, 0, 8);
        // The kept sound has no valid handle, so the update loop skips it.
        e.mem.set_u32(kept, u32::MAX);
        let old_list = make_list(&mut e, &[marked, kept]);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pSkySoundList, Ptr::new(old_list));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(weather));
        e.set_global(SOUNDS_DIRTY, 1u8);
        e.set_global(SOUND_TYPE_3_COUNT, 2u8);
        e.call_log = Some(vec![]);
        e.call(0x0063_d5e0, &args![sky]);
        let log = e.call_log.take().unwrap();
        // The marked sound is released and deleted, the count of type-3
        // sounds goes down, the kept one moves to the new list.
        assert!(log.contains(&(SOUND_HANDLE_RELEASE, vec![marked])));
        assert!(log.contains(&(SKY_SOUND_BASE_DESTRUCT, vec![marked])));
        assert!(log.contains(&(OPERATOR_DELETE, vec![marked])));
        assert!(!log.contains(&(SOUND_HANDLE_RELEASE, vec![kept])));
        assert_eq!(e.global::<u8>(SOUND_TYPE_3_COUNT), 1);
        assert_eq!(e.global::<u8>(SOUNDS_DIRTY), 0);
        let new_list = e.get(sky, Sky::pSkySoundList).addr();
        assert_ne!(new_list, old_list);
        assert_eq!(list_items(&e, new_list), [kept]);
        // The old list is cleared and deleted (flag 1), and the scratch list
        // is cleared and destroyed.
        assert!(log.contains(&(LIST_CLEAR, vec![old_list])));
        assert!(log.contains(&(LIST_SCALAR_DELETING_DESTRUCTOR, vec![old_list, 1])));
        assert_eq!(calls_to(&log, LIST_DESTRUCT).len(), 1);
    }

    #[test]
    fn sounds_outside_the_visible_modes_are_stopped_and_marked() {
        let mut e = sound_engine();
        let sound = make_sound(&mut e, 0x4000, 0, 7);
        e.mem.set_u8(sound + 4, 1);
        let list = make_list(&mut e, &[sound]);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4100));
        e.set(sky, Sky::eMode, 1);
        e.call_log = Some(vec![]);
        e.call(0x0063_d5e0, &args![sky]);
        let log = e.call_log.take().unwrap();
        // The other weather (the current one) has no sound 7: stop.
        assert!(log.contains(&(SOUND_HANDLE_STOP, vec![sound])));
        assert_eq!(e.mem.u32(sound + 0x18), 0x8000_0000);
        assert_eq!(e.global::<u8>(SOUNDS_DIRTY), 1);

        // When the other weather has the same sound it keeps playing.
        let other = make_sound(&mut e, 0x4100, 0, 7);
        let list = make_list(&mut e, &[sound, other]);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.mem.set_u32(sound + 0x18, 0);
        e.set_global(SOUNDS_DIRTY, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x0063_d5e0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&SOUND_HANDLE_STOP));
        assert_eq!(e.mem.u32(sound + 0x18), 0x8000_0000);
    }

    #[test]
    fn weather_sounds_follow_the_blend() {
        let mut e = sound_engine();
        size_limit(&mut e, 10, 20);
        let sound = make_sound(&mut e, 0x4000, 0, 7);
        let list = make_list(&mut e, &[sound]);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4000));
        e.set(sky, Sky::eMode, 3);
        e.set(sky, Sky::fCurrentWeatherPct, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x0063_d5e0, &args![sky]);
        let log = e.call_log.take().unwrap();
        // A plain sound of the current weather plays at the blend (the
        // handle's volume 0.2 differs from 0.25 by more than 0.01).
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_SET_VOLUME),
            [[sound, 0.25f32.to_bits()]]
        );
        assert!(!addresses(&log).contains(&SOUND_HANDLE_STOP));

        // A sound of the last weather plays at 1 - blend.
        let mut e = sound_engine();
        size_limit(&mut e, 10, 20);
        let sound = make_sound(&mut e, 0x4100, 2, 7);
        let list = make_list(&mut e, &[sound]);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pLastWeather, Ptr::new(0x4100));
        e.set(sky, Sky::eMode, 2);
        e.set(sky, Sky::fCurrentWeatherPct, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x0063_d5e0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_SET_VOLUME),
            [[sound, 0.75f32.to_bits()]]
        );
    }

    #[test]
    fn type_one_sounds_fade_in_and_out_with_the_weather_fractions() {
        let mut e = sound_engine();
        size_limit(&mut e, 10, 20);
        // Index 6: the rising end (0.25); index 7: the falling start (0.5).
        e.register(WEATHER_BYTE_FRACTION, |_, a| match a[1] {
            6 => ret_float(0.25),
            7 => ret_float(0.5),
            _ => panic!("unexpected index"),
        });
        let sound = make_sound(&mut e, 0x4000, 1, 7);
        let list = make_list(&mut e, &[sound]);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4000));
        e.set(sky, Sky::pLastWeather, Ptr::new(0x4100));
        e.set(sky, Sky::eMode, 3);
        e.set(sky, Sky::fCurrentWeatherPct, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0063_d5e0, &args![sky]);
        let log = e.call_log.take().unwrap();
        let fractions = calls_to(&log, WEATHER_BYTE_FRACTION);
        assert_eq!(fractions.len(), 2);
        assert_eq!(fractions[0][..2], [0x4000, 6]);
        assert_eq!(fractions[1][..2], [0x4100, 7]);
        // (0.5 - 0.25) / (1 - 0.25) rising; the falling sound has not
        // started (0.5 is not above the blend 0.5).
        let expected = ((0.5f64 - 0.25) / (1.0 - 0.25)) as f32;
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_SET_VOLUME),
            [[sound, expected.to_bits()]]
        );
    }

    #[test]
    fn type_three_sounds_start_at_random_and_record_the_flash() {
        let mut e = sound_engine();
        let weather = e.mem.alloc(0x400);
        e.mem.set_u8(weather + 0xea, 2);
        e.register(RANDOM_NUMBER, |_, _| ret(20));
        e.set_global(FRACTION_HIGH_SOUND, 0.99f32);
        e.register(WEATHER_BYTE_FRACTION, |_, a| {
            assert_eq!(a[1], 8);
            ret_float(0.5)
        });
        size_limit(&mut e, 100, 10);
        e.set_global(SOUND_TYPE_3_COUNT, 5u8);
        let sound = make_sound(&mut e, weather, 3, 7);
        let list = make_list(&mut e, &[sound]);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.set(sky, Sky::pCurrentWeather, Ptr::new(weather));
        e.set(sky, Sky::eMode, 3);
        e.set(sky, Sky::fCurrentWeatherPct, 0.75);
        e.call_log = Some(vec![]);
        e.call(0x0063_d5e0, &args![sky]);
        let log = e.call_log.take().unwrap();
        // 20 % (2 * 5) == 0: the sound starts at (0.75 - 0.5) / (1 - 0.5);
        // the fraction is asked with index 8, the high end from the exe's
        // data and 0 as the low end.
        assert_eq!(
            calls_to(&log, WEATHER_BYTE_FRACTION),
            [[weather, 8, 0.99f32.to_bits(), 0.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, SOUND_HANDLE_PLAY), [[sound, 0]]);
        assert_eq!(e.get(sky, Sky::fFlash), 0.5);
        assert_eq!(e.get(sky, Sky::uiFlashTime), 1234);

        // A roll that is not a multiple of the divisor does nothing.
        e.register(RANDOM_NUMBER, |_, _| ret(21));
        e.set(sky, Sky::fFlash, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x0063_d5e0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&SOUND_HANDLE_PLAY));
        assert_eq!(e.get(sky, Sky::fFlash), 0.0);
    }

    #[test]
    fn weather_sound_accessors_read_and_mark() {
        let mut e = engine();
        let weather = e.mem.alloc(0x400);
        e.mem.set_u8(weather + 0xe0 + 3, 9);
        assert_eq!(e.call(0x0063_dd50, &args![weather, 3i32]).u8(), 9);
        assert_eq!(e.call(0x0063_dff0, &args![weather]).u32(), weather + 0x1f8);
        let sound = make_sound(&mut e, 0, 0, 0);
        assert!(!e.call(0x0063_dd70, &args![sound]).bool());
        e.call(0x0063_dd90, &args![sound]);
        assert!(e.call(0x0063_dd70, &args![sound]).bool());
        assert_eq!(e.mem.u32(sound + 0x18), 0x8000_0000);
    }

    #[test]
    fn weather_sounds_are_added_unless_already_listed() {
        let mut e = sound_engine();
        e.register(SETTING_BYTE_VALUE, |_, a| ret(a[0]));
        e.register(AUDIO_INSTANCE, |_, _| ret(0xa000));
        e.register(AUDIO_GET_SOUND_HANDLE, |e, a| {
            // (audio, out, id, flags): a handle with the id.
            assert_eq!(a[0], 0xa000);
            e.mem.set_u32(a[1], a[2]);
            e.mem.set_u32(a[1] + 4, a[3]);
            ret(a[1])
        });
        e.set_global(SETTING_PRECIPITATION, 1u8);
        let weather = e.mem.alloc(0x400);
        // Two descriptions: `{ form id 70, type 3 }` and `{ 71, type 1 }`.
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 70);
        e.mem.set_u32(first + 4, 3);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 71);
        e.mem.set_u32(second + 4, 1);
        let descriptions = make_list(&mut e, &[first, second]);
        // `weather + 0x1f8` is the head node itself: copy the first node.
        let node_item = e.mem.u32(descriptions);
        let node_next = e.mem.u32(descriptions + 4);
        e.mem.set_u32(weather + 0x1f8, node_item);
        e.mem.set_u32(weather + 0x1fc, node_next);
        let existing = make_sound(&mut e, weather, 1, 71);
        let list = make_list(&mut e, &[existing]);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.set_global(SOUND_TYPE_3_COUNT, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x0063_ddb0, &args![sky, weather]);
        let log = e.call_log.take().unwrap();
        // Handles are requested with 0x21 (type 3) and 0x31 (other types).
        let requests = calls_to(&log, AUDIO_GET_SOUND_HANDLE);
        assert_eq!(requests.len(), 2);
        assert_eq!((requests[0][2], requests[0][3]), (70, 0x21));
        assert_eq!((requests[1][2], requests[1][3]), (71, 0x31));
        // Sound 70 is new: a SkySound is inserted and counted; sound 71 is
        // already listed, so only its handle is released.
        let items = list_items(&e, list);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], existing);
        let added = items[1];
        assert_eq!(e.mem.u32(added), 70);
        assert_eq!(e.mem.u32(added + 0xc), weather);
        assert_eq!(e.mem.u32(added + 0x10), 3);
        assert_eq!(e.mem.u32(added + 0x14), 70);
        assert_eq!(e.global::<u8>(SOUND_TYPE_3_COUNT), 1);
        assert_eq!(calls_to(&log, SOUND_HANDLE_RELEASE).len(), 1);
        // A request and a handle per description, plus the copy that the
        // `SkySound` constructor destroys.
        assert_eq!(calls_to(&log, SOUND_HANDLE_DESTRUCT).len(), 5);

        // A type-1 sound is skipped while the precipitation setting is off.
        e.set_global(SETTING_PRECIPITATION, 0u8);
        e.mem.set_u32(weather + 0x1f8, second);
        e.mem.set_u32(weather + 0x1fc, 0);
        let list = make_list(&mut e, &[]);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.call_log = Some(vec![]);
        e.call(0x0063_ddb0, &args![sky, weather]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&AUDIO_GET_SOUND_HANDLE));
        // No weather: nothing happens at all.
        e.call_log = Some(vec![]);
        e.call(0x0063_ddb0, &args![sky, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn sound_list_searches_stop_at_the_first_empty_node() {
        let mut e = sound_engine();
        let first = make_sound(&mut e, 0x4000, 0, 7);
        let second = make_sound(&mut e, 0x4100, 0, 8);
        let list = make_list(&mut e, &[first, second]);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        assert!(e.call(0x0063_e060, &args![sky, 8u32, 0x4100u32]).bool());
        assert!(!e.call(0x0063_e060, &args![sky, 8u32, 0x4000u32]).bool());
        assert!(!e.call(0x0063_e060, &args![sky, 9u32, 0x4100u32]).bool());
        let empty = make_list(&mut e, &[]);
        e.set(sky, Sky::pSkySoundList, Ptr::new(empty));
        assert!(!e.call(0x0063_e060, &args![sky, 7u32, 0x4000u32]).bool());

        e.set(sky, Sky::pSkySoundList, Ptr::new(list));
        e.call_log = Some(vec![]);
        e.call(0x0063_e010, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SOUND_HANDLE_STOP), [[first], [second]]);
    }

    #[test]
    fn sky_sound_play_respects_menus_and_sets_the_volume() {
        let mut e = sound_engine();
        size_limit(&mut e, 10, 20);
        e.register(AUDIO_INSTANCE, |_, _| ret(0xa000));
        e.register(AUDIO_GET_SOUND_HANDLE, |e, a| {
            e.mem.set_u32(a[1], a[2]);
            ret(a[1])
        });
        let sound = make_sound(&mut e, 0x4000, 3, 7);
        // In menu mode nothing happens.
        e.register(IS_IN_MENU_MODE, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0063_e0d0, &args![sound, 0.5f32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        // While the loading menu is open nothing happens either.
        e.register(IS_IN_MENU_MODE, |_, _| ret(0));
        e.register(IS_LOADING_MENU_OPEN, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0063_e0d0, &args![sound, 0.5f32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
        e.register(IS_LOADING_MENU_OPEN, |_, _| ret(0));

        // A sound without a valid handle asks for a new one (flags 0x20,
        // the id read from the sound), a type-3 sound starts with Play(0).
        e.mem.set_u32(sound, u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x0063_e0d0, &args![sound, 0.5f32]);
        let log = e.call_log.take().unwrap();
        // NiPointer::get is the double of `engine()`: it reads the word.
        let request = &calls_to(&log, AUDIO_GET_SOUND_HANDLE)[0];
        assert_eq!((request[2], request[3]), (u32::MAX, 0x20));
        assert_eq!(calls_to(&log, SOUND_HANDLE_PLAY), [[sound, 0]]);
        // The volume 0.2 differs from 0.5 by more than 0.01.
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_SET_VOLUME),
            [[sound, 0.5f32.to_bits()]]
        );

        // A volume within 0.01 is left alone; a playing type-3 sound is not
        // started again.
        e.mem.set_u32(sound, 5);
        e.mem.set_u8(sound + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0063_e0d0, &args![sound, 0.205f32]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&SOUND_HANDLE_PLAY));
        assert!(!addresses(&log).contains(&SOUND_HANDLE_SET_VOLUME));
    }

    #[test]
    fn sky_sound_play_starts_other_types_when_allowed() {
        let mut e = sound_engine();
        let sound = make_sound(&mut e, 0x4000, 0, 7);
        // `fn_00639c90(1000)` rejects: no Play.
        size_limit(&mut e, 10, 20);
        e.call_log = Some(vec![]);
        e.call(0x0063_e0d0, &args![sound, 0.2f32]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&SOUND_HANDLE_PLAY));
        // It accepts: Play(1).
        size_limit(&mut e, 100, 10);
        e.call_log = Some(vec![]);
        e.call(0x0063_e0d0, &args![sound, 0.2f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SOUND_HANDLE_PLAY), [[sound, 1]]);
        assert!(log.contains(&(0x0063_9d00, vec![sound, 100 + 1000])));
    }

    // ---- textures, save, load and image-space values ----

    #[test]
    fn unloading_the_textures_unloads_every_component() {
        let mut e = engine();
        install_vtable(&mut e);
        for address in [
            CLOUDS_REMOVE_TEXTURES,
            CLOUDS_CLEAR_TRANS_TEXTURES,
            MOON_UNLOAD_TEXTURES,
            SUN_UNLOAD_TEXTURES,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let sky = new_sky(&mut e);
        // Without components nothing is called.
        e.call_log = Some(vec![]);
        e.call(0x0063_e210, &args![sky]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);

        e.set(sky, Sky::pClouds, Ptr::new(0x5000));
        e.set(sky, Sky::pMasser, Ptr::new(0x5100));
        e.set(sky, Sky::pSecunda, Ptr::new(0x5200));
        e.set(sky, Sky::pSun, Ptr::new(0x5300));
        // The stars: `+4` holds an object with a vtable, `+8` another one.
        let stars = e.mem.alloc(0x10);
        let object = object_with_vtable(&mut e, 0x10);
        e.mem.set_u32(stars + 4, object.addr());
        e.mem.set_u32(stars + 8, 0x5500);
        e.set(sky, Sky::pStars, Ptr::new(stars));
        e.call_log = Some(vec![]);
        e.call(0x0063_e210, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            [
                0x0063_e210,
                CLOUDS_REMOVE_TEXTURES,
                CLOUDS_CLEAR_TRANS_TEXTURES,
                MOON_UNLOAD_TEXTURES,
                MOON_UNLOAD_TEXTURES,
                SUN_UNLOAD_TEXTURES,
                NI_POINTER_GET,
                NI_POINTER_GET,
                NI_POINTER_GET,
                SLOT_BASE + 0xe8,
                NI_POINTER_ASSIGN,
            ]
        );
        assert!(log.contains(&(MOON_UNLOAD_TEXTURES, vec![0x5100])));
        assert!(log.contains(&(MOON_UNLOAD_TEXTURES, vec![0x5200])));
        // The stars' second object is passed to the first one's slot 0xe8
        // and the second pointer is cleared.
        assert!(log.contains(&(SLOT_BASE + 0xe8, vec![object.addr(), 0x5500])));
        assert_eq!(e.mem.u32(stars + 8), 0);
    }

    #[test]
    fn stars_release_does_nothing_but_clear_without_an_object() {
        let mut e = engine();
        let stars = e.mem.alloc(0x10);
        e.mem.set_u32(stars + 8, 0x5500);
        e.call_log = Some(vec![]);
        e.call(0x0063_e290, &args![stars]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            [0x0063_e290, NI_POINTER_GET, NI_POINTER_ASSIGN]
        );
        assert_eq!(e.mem.u32(stars + 8), 0);
    }

    /// The doubles of the texture loading, with a vtable whose slot 0x18
    /// returns 0x6100.
    fn texture_engine() -> Engine {
        let mut e = climate_update_engine();
        e.register(WORLD_STATE_FLAG, |_, _| ret(0));
        e.set_global(WORLD_STATE_POINTER, 0x9000u32);
        e.register(CLOUDS_LAYER_COUNT, |_, _| ret(2));
        e.register(NODE_GET_PROPERTY, |_, a| ret(0x8100 + a[0]));
        e.register(WEATHER_CLOUD_ENTRY, |e, a| {
            // Layer 0 has an entry (an object with the vtable), layer 1 none.
            assert_eq!(a[0], 0x4000);
            if a[1] == 0 {
                let entry = e.mem.alloc(0x10);
                e.mem.set_u32(entry, VTABLE);
                ret(entry)
            } else {
                ret(0)
            }
        });
        e.register(FORMAT_STRING, |_, _| Ret::default());
        e.register(FILE_EXISTS, |_, _| ret(1));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e
    }

    #[test]
    fn visible_sky_textures_are_loaded_for_clouds_moons_sun_and_stars() {
        let mut e = texture_engine();
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4000));
        // Clouds: layer nodes at `clouds + 8 + 4 * layer`.
        let clouds = e.mem.alloc(0x20);
        e.mem.set_u32(clouds + 8, 0x5800);
        e.mem.set_u32(clouds + 12, 0x5900);
        e.set(sky, Sky::pClouds, Ptr::new(clouds));
        // One moon (Masser) whose node is at `+0x14`.
        let moon = e.mem.alloc(0x80);
        e.mem.set_u32(moon + 0x14, 0x5a00);
        e.set(sky, Sky::pMasser, Ptr::new(moon));
        // The sun and the climate with its entries.
        let climate = e.mem.alloc(0x80);
        e.mem.set_u32(climate + 0x18, VTABLE);
        e.mem.set_u32(climate + 0x38, VTABLE);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(climate));
        let sun = e.mem.alloc(0x30);
        e.mem.set_u32(sun + 8, 0x5100);
        e.mem.set_u32(sun + 0xc, 0x5200);
        e.mem.set_u32(sun + 0x14, 0x5300);
        e.set(sky, Sky::pSun, Ptr::new(sun));
        let stars = e.mem.alloc(0x10);
        e.set(sky, Sky::pStars, Ptr::new(stars));
        e.call_log = Some(vec![]);
        e.call(0x0063_e2f0, &args![sky]);
        let log = e.call_log.take().unwrap();
        // The scope timer wraps everything, with the source line 0x8dc.
        assert_eq!(addresses(&log)[1], TIMER_SCOPE_CONSTRUCT);
        assert_eq!(*log[1].1.last().unwrap(), 0x8dc);
        assert_eq!(*addresses(&log).last().unwrap(), TIMER_SCOPE_DESTROY);
        // Layer 0 is formatted ("%s%s": prefix, name), exists, and loads
        // into the property types 5 and 3 of the layer's node; layer 1 has
        // no entry.
        let formats = calls_to(&log, FORMAT_STRING);
        assert_eq!(formats.len(), 1);
        assert_eq!(formats[0][1..], [PATH_FORMAT, 0x6100, 0x6200]);
        assert_eq!(calls_to(&log, FILE_EXISTS).len(), 1);
        assert!(log.contains(&(NODE_GET_PROPERTY, vec![0x5800, 5])));
        assert!(log.contains(&(NODE_GET_PROPERTY, vec![0x5800, 3])));
        assert!(!log.contains(&(NODE_GET_PROPERTY, vec![0x5900, 5])));
        // The moon shadow texture loads with the constant path; the moon's
        // node is flagged and its state word is 2.
        assert_eq!(
            calls_to(&log, TEXTURE_LOAD)
                .iter()
                .filter(|args| args[0] == MOON_SHADOW_PATH)
                .count(),
            1
        );
        assert!(log.contains(&(NODE_SET_FLAG_BIT_20, vec![0x5a00, 1])));
        assert_eq!(e.mem.u32(moon + 0x70), 2);
        // The sun: both entries are entry 0 of the climate (+0x38), so the
        // path is built twice; both sun nodes are flagged.
        assert_eq!(calls_to(&log, STRING_ASSIGN).len(), 2);
        assert!(log.contains(&(NODE_SET_FLAG_BIT_20, vec![0x5100, 1])));
        assert!(log.contains(&(NODE_SET_FLAG_BIT_20, vec![0x5200, 1])));
        // The stars get the climate's geometry value (slot 0x14).
        assert!(log.contains(&(STARS_LOAD_GEOMETRY, vec![stars, 0x5150])));
    }

    #[test]
    fn sky_textures_wait_for_the_weather_and_use_the_non_hdr_glare() {
        let mut e = texture_engine();
        let sky = new_sky(&mut e);
        // No weather and no climate: only the timer runs.
        let clouds = e.mem.alloc(0x20);
        e.set(sky, Sky::pClouds, Ptr::new(clouds));
        e.call_log = Some(vec![]);
        e.call(0x0063_e2f0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            [0x0063_e2f0, TIMER_SCOPE_CONSTRUCT, TIMER_SCOPE_DESTROY]
        );

        // With the world flag set the cloud textures are not loaded; a sun
        // entry named like the HDR glare is replaced while the flag byte is
        // clear.
        e.register(WORLD_STATE_FLAG, |_, _| ret(1));
        e.register(STRING_COMPARE, |_, a| {
            assert_eq!((a[0], a[1]), (0x6200, SUN_GLARE_NAME));
            ret(0)
        });
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4000));
        let climate = e.mem.alloc(0x80);
        e.mem.set_u32(climate + 0x18, VTABLE);
        e.mem.set_u32(climate + 0x38, VTABLE);
        e.set(sky, Sky::pCurrentClimate, Ptr::new(climate));
        let sun = e.mem.alloc(0x30);
        e.mem.set_u32(sun + 8, 0x5100);
        e.mem.set_u32(sun + 0xc, 0x5200);
        e.mem.set_u32(sun + 0x14, 0x5300);
        e.set(sky, Sky::pSun, Ptr::new(sun));
        e.set_global(INITIAL_FLAG, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x0063_e2f0, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&WEATHER_CLOUD_ENTRY));
        assert!(calls_to(&log, STRING_ASSIGN)
            .iter()
            .any(|args| args[1] == SUN_GLARE_NON_HDR_PATH));
    }

    #[test]
    fn save_size_depends_on_the_save_version() {
        let mut e = with_data(engine());
        e.set_global(SAVE_BUFFER_GLOBAL, 0x9100u32);
        for (version, size) in [(0x5cu32, 28u32), (0x5d, 32), (0x68, 32), (0x69, 40)] {
            e.register_double(SAVE_VERSION, move |_, _| ret(version));
            assert_eq!(e.call(0x0063_e940, &args![0x1000u32]).u32(), size);
        }
    }

    /// Doubles for the save buffer: the version, form ids (`form + 1`) and
    /// the recording of everything written.
    fn save_engine(version: u32) -> Engine {
        let mut e = with_data(engine());
        e.set_global(SAVE_BUFFER_GLOBAL, 0x9100u32);
        e.register_double(SAVE_VERSION, move |_, _| ret(version));
        e.register(FORM_ID_OF, |_, a| ret(a[0] + 1));
        e.register(SAVE_NUMERIC_ID, |e, a| {
            assert_eq!((a[0], a[2]), (0x9100, 4));
            let _ = e.mem.u32(a[1]);
            Ret::default()
        });
        e.register(SAVE_DATA, |_, _| Ret::default());
        e
    }

    #[test]
    fn save_game_writes_ids_and_fields_by_version() {
        let mut e = save_engine(0x5d);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x100));
        e.set(sky, Sky::pDefaultWeather, Ptr::new(0x300));
        e.set(sky, Sky::pOverrideWeather, Ptr::new(0x400));
        e.register_double(SAVE_NUMERIC_ID, |e, a| {
            assert_eq!(a[2], 4);
            let id = e.mem.u32(a[1]);
            // Record the id in the engine's memory: a word table at 0x0118_0000.
            let count = e.mem.u32(0x0118_0000);
            e.mem.set_u32(0x0118_0004 + 4 * count, id);
            e.mem.set_u32(0x0118_0000, count + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0063_e9f0, &args![sky]);
        let log = e.call_log.take().unwrap();
        // Ids: current (0x101), last (null: 0), default (0x301) and, from
        // version 0x5d, the override (0x401).
        let count = e.mem.u32(0x0118_0000);
        let ids: Vec<u32> = (0..count).map(|i| e.mem.u32(0x0118_0004 + 4 * i)).collect();
        assert_eq!(ids, [0x101, 0, 0x301, 0x401]);
        // The four plain fields, but not the flags (version 0x69).
        let fields: Vec<u32> = calls_to(&log, SAVE_DATA)
            .iter()
            .map(|args| args[1] - sky.addr())
            .collect();
        assert_eq!(fields, [0xec, 0xf0, 0xf4, 0xf8]);

        // Version 0x5c has no override id; version 0x69 saves the flags and
        // the acceleration start.
        for (version, id_count, field_count) in [(0x5cu32, 3u32, 4usize), (0x69, 4, 6)] {
            e.register_double(SAVE_VERSION, move |_, _| ret(version));
            e.mem.set_u32(0x0118_0000, 0);
            e.call_log = Some(vec![]);
            e.call(0x0063_e9f0, &args![sky]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(0x0118_0000), id_count);
            let fields: Vec<u32> = calls_to(&log, SAVE_DATA)
                .iter()
                .map(|args| args[1] - sky.addr())
                .collect();
            assert_eq!(fields.len(), field_count);
            if version == 0x69 {
                assert_eq!(fields[4..], [0x118, 0x110]);
            }
        }
    }

    #[test]
    fn save_game_ov2_writes_four_ids_and_eleven_fields() {
        let mut e = engine();
        e.register(SAVE_FORM_ID_OV2, |_, _| Ret::default());
        e.register(SAVE_DATA_OV2, |_, _| Ret::default());
        let sky = new_sky(&mut e);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x100));
        e.set(sky, Sky::pLastWeather, Ptr::new(0x200));
        e.set(sky, Sky::pDefaultWeather, Ptr::new(0x300));
        e.set(sky, Sky::pOverrideWeather, Ptr::new(0x400));
        e.call_log = Some(vec![]);
        e.call(0x0063_eb70, &args![sky, 0x9200u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_FORM_ID_OV2),
            [
                [0x9200, 0x100, 0],
                [0x9200, 0x200, 0],
                [0x9200, 0x300, 0],
                [0x9200, 0x400, 0]
            ]
        );
        let fields: Vec<u32> = calls_to(&log, SAVE_DATA_OV2)
            .iter()
            .map(|args| {
                assert_eq!((args[0], args[2], args[3]), (0x9200, 4, 0));
                args[1] - sky.addr()
            })
            .collect();
        assert_eq!(
            fields,
            [0xec, 0xf0, 0xf4, 0x118, 0x110, 0xb4, 0xb8, 0xbc, 0xe4, 0xe8, 0xf8]
        );
    }

    /// A data-load double that writes `value` to every 4-byte target outside
    /// the sky (the temporary that receives the saved flags).
    fn load_data_double(e: &mut Engine, sky: Ptr<Sky>, value: u32) {
        let sky_address = sky.addr();
        e.register_double(LOAD_DATA, move |e, a| {
            if a[2] == 4 && !(sky_address..sky_address + 0x138).contains(&a[1]) {
                e.mem.set_u32(a[1], value);
            }
            Ret::default()
        });
    }

    /// Doubles for the load buffer: the buffer (an object whose vtable slot 0
    /// returns `version`), the form ids 1, 2, ... and the lookup and cast.
    fn load_engine(version: u32) -> (Engine, u32) {
        let mut e = with_data(engine());
        install_vtable(&mut e);
        e.register_double(SLOT_BASE, move |_, _| ret(version));
        e.set_global(WORLD_STATE_POINTER, 0x9000u32);
        e.register(CLOUDS_CLEAR_TRANS_TEXTURES, |_, _| Ret::default());
        e.register(PRECIPITATION_FLUSH_ALL, |_, _| Ret::default());
        e.register(LOAD_FORM_ID, |e, _| {
            let next = e.mem.u32(0x0118_0010) + 1;
            e.mem.set_u32(0x0118_0010, next);
            ret(next)
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
        e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!(
                a[1..],
                [0, TYPE_DESCRIPTOR_FORM, TYPE_DESCRIPTOR_WEATHER, 0]
            );
            ret(a[0] + 0x1000)
        });
        hdr_doubles(&mut e);
        e.register(PLAYER_PARENT_CELL, |_, _| ret(0));
        let buffer = object_with_vtable(&mut e, 0x10).addr();
        (e, buffer)
    }

    #[test]
    fn load_game_reads_the_weathers_fields_and_updates() {
        let (mut e, buffer) = load_engine(0x12);
        let state = e.mem.alloc(0x300);
        e.set_global(WORLD_STATE_POINTER, state);
        let sky = sky_with_instances(&mut e);
        load_data_double(&mut e, sky, 0xc);
        e.set(sky, Sky::uiFlags, 0x10);
        let clouds = e.mem.alloc(0x80);
        e.set(sky, Sky::pClouds, Ptr::new(clouds));
        e.set(sky, Sky::pPrecip, Ptr::new(0x6000));
        e.call_log = Some(vec![]);
        e.call(0x0063_ecb0, &args![sky, buffer]);
        let log = e.call_log.take().unwrap();
        // The precipitation, then the clouds are cleared (and flagged).
        assert!(log.contains(&(PRECIPITATION_FLUSH_ALL, vec![0x6000])));
        assert!(log.contains(&(CLOUDS_CLEAR_TRANS_TEXTURES, vec![clouds])));
        assert_eq!(e.mem.u8(clouds + 0x5a), 1);
        // The four weathers: id + 0x1000 (lookup) + 0x1000 (cast).
        assert_eq!(e.get(sky, Sky::pCurrentWeather).addr(), 0x2001);
        assert_eq!(e.get(sky, Sky::pLastWeather).addr(), 0x2002);
        assert_eq!(e.get(sky, Sky::pDefaultWeather).addr(), 0x2003);
        assert_eq!(e.get(sky, Sky::pOverrideWeather).addr(), 0x2004);
        // Only bit 8 comes from the saved flags (the double writes 0xc).
        assert_eq!(e.get(sky, Sky::uiFlags), 0x18);
        let loads: Vec<(u32, u32)> = calls_to(&log, LOAD_DATA)
            .iter()
            .map(|args| (args[1].wrapping_sub(sky.addr()), args[2]))
            .filter(|(offset, _)| *offset < 0x138)
            .collect();
        assert_eq!(
            loads,
            [
                (0xec, 4),
                (0xf0, 4),
                (0xf4, 4),
                (0x110, 4),
                (0xb4, 4),
                (0xb8, 4),
                (0xbc, 4),
                (0xe4, 4),
                (0xe8, 4),
                (0xf8, 4)
            ]
        );
        // The update (no parent cell, so it ends at once) and the image-space
        // update (mode 0) follow.
        assert_eq!(calls_to(&log, IMAGE_SPACE_SET_WEIGHT).len(), 4);
        assert!(addresses(&log).contains(&PLAYER_PARENT_CELL));
    }

    #[test]
    fn load_game_reads_the_old_colour_in_one_block_and_obeys_the_world_flag() {
        let (mut e, buffer) = load_engine(0x11);
        let state = e.mem.alloc(0x300);
        e.set_global(WORLD_STATE_POINTER, state);
        let sky = sky_with_instances(&mut e);
        load_data_double(&mut e, sky, 0);
        e.set(sky, Sky::uiFlags, 0x8);
        e.call_log = Some(vec![]);
        e.call(0x0063_ecb0, &args![sky, buffer]);
        let log = e.call_log.take().unwrap();
        // Saved flags without bit 8 clear it; before buffer version 0x12 the
        // colour at +0xb4 is one 12-byte block.
        assert_eq!(e.get(sky, Sky::uiFlags), 0);
        assert!(calls_to(&log, LOAD_DATA)
            .iter()
            .any(|args| args[1] == sky.addr() + 0xb4 && args[2] == 12));
        assert!(!calls_to(&log, LOAD_DATA)
            .iter()
            .any(|args| args[1] == sky.addr() + 0xb8));

        // The world flag 0x40 makes the load do nothing.
        e.mem.set_u32(state + 0x244, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x0063_ecb0, &args![sky, buffer]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn world_state_flag_is_bit_0x40_of_the_word_at_0x244() {
        let mut e = engine();
        let state = e.mem.alloc(0x300);
        assert!(!e.call(0x0063_ef00, &args![state]).bool());
        e.mem.set_u32(state + 0x244, 0x40);
        assert!(e.call(0x0063_ef00, &args![state]).bool());
        e.mem.set_u32(state + 0x244, 0xffff_ffbf);
        assert!(!e.call(0x0063_ef00, &args![state]).bool());
    }

    // ---- image-space values ----

    #[test]
    fn hdr_values_create_the_four_instances_once() {
        let mut e = with_data(engine());
        hdr_doubles(&mut e);
        e.register(IMAGE_SPACE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NI_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(IMAGE_SPACE_MANAGER, |_, a| {
            assert_eq!(a[0], 0x5000);
            ret(0xaa00)
        });
        // The manager's add records the instance the holder holds (the
        // holder is a temporary) in a table at 0x0118_0104.
        e.register(IMAGE_SPACE_ADD, |e, a| {
            let count = e.mem.u32(0x0118_0100);
            let instance = e.mem.u32(a[1]);
            e.mem.set_u32(0x0118_0104 + 4 * count, instance);
            e.mem.set_u32(0x0118_0100, count + 1);
            Ret::default()
        });
        e.register(IMAGE_SPACE_SET_FLAG, |_, _| Ret::default());
        e.set_global(INTERIOR_CELL_HOLDER, 0x5000u32);
        let sky = new_sky(&mut e);
        e.set(sky, Sky::eMode, 1);
        e.call_log = Some(vec![]);
        e.call(0x0063_ef20, &args![sky]);
        let log = e.call_log.take().unwrap();
        let current = e.get(sky, Sky::pCurrentWeatherImageSpaceMod).addr();
        let last = e.get(sky, Sky::pLastWeatherImageSpaceMod).addr();
        let current_2 = e.get(sky, Sky::pCurrentWeatherImageSpaceMod2).addr();
        let last_2 = e.get(sky, Sky::pLastWeatherImageSpaceMod2).addr();
        for instance in [current, last, current_2, last_2] {
            assert_ne!(instance, 0);
            assert_eq!(e.mem.u8(instance + 8), 1);
            assert_eq!(e.mem.block_size(instance), Some(IMAGE_SPACE_SIZE));
        }
        // Registered in the game's order: last, current, last 2, current 2;
        // flagged in the order current, last, current 2, last 2.
        assert!(calls_to(&log, IMAGE_SPACE_ADD)
            .iter()
            .all(|args| args[0] == 0xaa00));
        let added: Vec<u32> = (0..e.mem.u32(0x0118_0100))
            .map(|i| e.mem.u32(0x0118_0104 + 4 * i))
            .collect();
        assert_eq!(added, [last, current, last_2, current_2]);
        let flagged: Vec<u32> = calls_to(&log, IMAGE_SPACE_SET_FLAG)
            .iter()
            .map(|args| {
                assert_eq!(args[1], 1);
                args[0]
            })
            .collect();
        assert_eq!(flagged, [current, last, current_2, last_2]);
        // Mode 1: all four weights are 0.
        for instance in [current, last, current_2, last_2] {
            assert_eq!(weights_of(&log, instance), [0.0]);
        }
        // A second call creates nothing.
        e.call_log = Some(vec![]);
        e.call(0x0063_ef20, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&IMAGE_SPACE_CONSTRUCT));
    }

    /// A sky in mode 3 at `hour` with the climate hours 6 / 8 (sunrise),
    /// noon 13, and 18 / 22 (sunset), in the weather `0x4000`.
    fn hdr_sky(e: &mut Engine, hour: f32) -> Ptr<Sky> {
        hdr_doubles(e);
        e.set_global(SUNRISE_BEGIN_CACHE, 6.0f32);
        e.set_global(SUNSET_END_CACHE, 22.0f32);
        e.register(SKY_SUNRISE_END, |_, _| ret_float(8.0));
        e.register(SKY_SUNSET_BEGIN, |_, _| ret_float(18.0));
        let sky = sky_with_instances(e);
        e.set(sky, Sky::eMode, 3);
        e.set(sky, Sky::fCurrentGameHour, hour);
        e.set(sky, Sky::fHighNoon, 13.0);
        e.set(sky, Sky::pCurrentWeather, Ptr::new(0x4000));
        e.set(sky, Sky::fCurrentWeatherPct, 1.0);
        sky
    }

    fn forms_of(log: &[(u32, Vec<u32>)], instance: u32) -> Vec<u32> {
        calls_to(log, IMAGE_SPACE_SET_FORM)
            .iter()
            .filter(|args| args[0] == instance)
            .map(|args| args[1])
            .collect()
    }

    #[test]
    fn hdr_values_blend_two_forms_through_the_day() {
        // (hour, first index, second index, blend)
        let cases = [
            (6.5f32, 0u32, 3u32, 0.5f32),
            (7.0, 0, 1, 1.0),
            (10.0, 1, 4, 0.4),
            (15.0, 4, 1, 0.4),
            (20.0, 2, 3, 1.0),
            (19.0, 2, 1, 0.5),
        ];
        for (hour, first, second, blend) in cases {
            let mut e = with_data(engine());
            let sky = hdr_sky(&mut e, hour);
            e.call_log = Some(vec![]);
            e.call(0x0063_ef20, &args![sky]);
            let log = e.call_log.take().unwrap();
            assert_eq!(forms_of(&log, INSTANCE_CURRENT), [0x100 + first], "{hour}");
            assert_eq!(
                forms_of(&log, INSTANCE_CURRENT_2),
                [0x100 + second],
                "{hour}"
            );
            // The weight of the first is pct * blend, of the second
            // (1 - blend) * pct (pct is 1 here).
            let first_weight = weights_of(&log, INSTANCE_CURRENT);
            let second_weight = weights_of(&log, INSTANCE_CURRENT_2);
            assert!((first_weight[0] - blend).abs() < 1e-6, "{hour}");
            assert!((second_weight[0] - (1.0 - blend)).abs() < 1e-6, "{hour}");
            // The forms come from the weather at +0x18, asked with the index.
            assert!(log.contains(&(WEATHER_IMAGE_SPACE_FOR_TIME, vec![0x4018, first])));
            // No last weather: its instances get 0.
            assert_eq!(weights_of(&log, INSTANCE_LAST), [0.0]);
            assert_eq!(weights_of(&log, INSTANCE_LAST_2), [0.0]);
        }
    }

    #[test]
    fn hdr_values_at_night_use_one_form_and_the_last_weather_fades() {
        let mut e = with_data(engine());
        let sky = hdr_sky(&mut e, 23.0);
        e.set(sky, Sky::pLastWeather, Ptr::new(0x4100));
        e.set(sky, Sky::fCurrentWeatherPct, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x0063_ef20, &args![sky]);
        let log = e.call_log.take().unwrap();
        // Night is index 3, a single form; the second instances get 0.
        assert_eq!(forms_of(&log, INSTANCE_CURRENT), [0x103]);
        assert!(forms_of(&log, INSTANCE_CURRENT_2).is_empty());
        assert_eq!(weights_of(&log, INSTANCE_CURRENT), [0.25]);
        assert_eq!(weights_of(&log, INSTANCE_CURRENT_2), [0.0]);
        // The last weather gets (1 - pct) * blend (blend 1), also index 3.
        assert_eq!(forms_of(&log, INSTANCE_LAST), [0x103]);
        assert_eq!(weights_of(&log, INSTANCE_LAST), [0.75]);
        assert_eq!(weights_of(&log, INSTANCE_LAST_2), [0.0]);
        assert!(log.contains(&(WEATHER_IMAGE_SPACE_FOR_TIME, vec![0x4118, 3])));

        // A blend of 1 (or more) silences the last weather.
        e.set(sky, Sky::fCurrentWeatherPct, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x0063_ef20, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert_eq!(weights_of(&log, INSTANCE_LAST), [0.0]);
        assert_eq!(weights_of(&log, INSTANCE_LAST_2), [0.0]);
        assert!(forms_of(&log, INSTANCE_LAST).is_empty());
    }

    #[test]
    fn hdr_values_blend_the_last_weather_in_the_morning() {
        let mut e = with_data(engine());
        let sky = hdr_sky(&mut e, 10.0);
        e.set(sky, Sky::pLastWeather, Ptr::new(0x4100));
        e.set(sky, Sky::fCurrentWeatherPct, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0063_ef20, &args![sky]);
        let log = e.call_log.take().unwrap();
        // blend 0.4: (1 - pct) * blend and (1 - pct) * (1 - blend).
        assert!((weights_of(&log, INSTANCE_LAST)[0] - 0.2).abs() < 1e-6);
        assert!((weights_of(&log, INSTANCE_LAST_2)[0] - 0.3).abs() < 1e-6);
        assert_eq!(forms_of(&log, INSTANCE_LAST), [0x101]);
        assert_eq!(forms_of(&log, INSTANCE_LAST_2), [0x104]);
    }

    #[test]
    fn hdr_values_fall_back_to_the_default_form_and_log_bad_climate_data() {
        let mut e = with_data(engine());
        let sky = hdr_sky(&mut e, 13.0);
        // Hour 13 is exactly noon: no interval holds it. The data error is
        // logged and index 1 is used; a weather without a form falls back to
        // the default form.
        e.register(WEATHER_IMAGE_SPACE_FOR_TIME, |_, _| ret(0));
        e.register(LOG_MASTERFILE_ERROR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0063_ef20, &args![sky]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(LOG_MASTERFILE_ERROR, vec![TRANSITION_TIMES_MESSAGE])));
        assert!(log.contains(&(WEATHER_IMAGE_SPACE_FOR_TIME, vec![0x4018, 1])));
        assert_eq!(forms_of(&log, INSTANCE_CURRENT), [1]);
    }
    // @@TESTS
}
