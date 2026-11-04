//! `fallout shared/sky/sun.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Sun` (a `SkyObject`, 0x2c bytes) owns the sun's two billboards (base and glare), their
//! quads, the directional light that follows it, the glare pick and the occlusion
//! accumulator. `Update` moves the sun with the game hour between the sunrise and sunset
//! of the climate, fades it with the weathers and the glare with its occlusion.
//!
//! The code is compiled without optimisation: every `NiPointer` access and accessor is a
//! call, every `float` local goes through memory. The translations make the calls in the
//! code's order and do each x87 operation through `crate::x87` with its operands as the
//! instruction sees them (`float` locals rounded at each `FSTP`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_sky::skyobject::{
    pointer_assign, pointer_get, NI_ALLOCATE, NI_POINTER_DESTRUCT, NI_POINTER_FROM_RAW,
    NODE_SET_FLAG_004902F0,
};

/// `Sun`'s vtable (`0104f298`).
const SUN_VTABLE: u32 = 0x0104_f298;
/// `SkyObject::SkyObject` (`0063f830`), `::~SkyObject` (`0063f8d0`), `::Initialize`
/// (`0063f940`).
const SKY_OBJECT_CONSTRUCT: u32 = 0x0063_f830;
const SKY_OBJECT_DESTRUCT: u32 = 0x0063_f8d0;
const SKY_OBJECT_INITIALIZE: u32 = 0x0063_f940;
/// `operator new` (`00401000`) and `operator delete` (`00401030`) (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `BSShaderAccumulator::BSShaderAccumulator` (`00b660d0`, three words; 0x280 bytes).
const ACCUMULATOR_CONSTRUCT: u32 = 0x00b6_60d0;
/// `NiPick::~NiPick` (`00e98fa0`) and `NiPick::NiPick` (`00e98f20`, two words; 0x34
/// bytes), and the pick's four setters `Initialize` calls with 1 (`00537e90`, `00403550`,
/// `00632d20`, `00458b30`; `thiscall`, one word).
const PICK_DESTRUCT: u32 = 0x00e9_8fa0;
const PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const PICK_SET_00537E90: u32 = 0x0053_7e90;
const PICK_SET_00403550: u32 = 0x0040_3550;
const PICK_SET_00632D20: u32 = 0x0063_2d20;
const PICK_SET_00458B30: u32 = 0x0045_8b30;
/// The scope timer (`00404eb0`: kind, 1, file, line; `00404ee0` ends it) and the file name
/// it is given (`0104f2ac`, `"D:\_Fallout3\...\Sun.cpp"`).
const TIMER_SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const TIMER_SCOPE_DESTROY: u32 = 0x0040_4ee0;
const SUN_SOURCE_FILE_NAME: u32 = 0x0104_f2ac;
/// The accumulator's `float` (`00508050`, `thiscall`, in `ST0`).
const ACCUMULATOR_FLOAT: u32 = 0x0050_8050;
/// `vector constructor iterator` (`00401050`, `stdcall`: array, size, count, constructor)
/// and the element constructors it is given: `006815c0` (answers `this`; points and
/// texture coordinates) and `004a7800` (colours).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const ELEMENT_CONSTRUCT_006815C0: u32 = 0x0068_15c0;
const ELEMENT_CONSTRUCT_004A7800: u32 = 0x004a_7800;
/// `NiPoint3`/`NiColor` from three `float`s (`00416870`), `NiColorA` from four
/// (`00414430`) and `NiPoint2` from two (`00452dc0`) (`thiscall`).
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
const COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// The setting objects of the sun's sizes (`011cce7c` the base, `011cce50` the glare) and
/// their `float` accessor (`00403e20`, a pointer to the value).
const SUN_BASE_SIZE_SETTING: u32 = 0x011c_ce7c;
const SUN_GLARE_SIZE_SETTING: u32 = 0x011c_ce50;
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// `-1.0` (`double`, `0101a6b0`).
const MINUS_ONE: u32 = 0x0101_a6b0;
/// The `NiPoint3` the normals and the starting direction are copied from (`011f426c`).
const NORMAL_011F426C: u32 = 0x011f_426c;
/// The Ni allocation of the triangle lists (`00aa1070`, `cdecl`, size).
const NI_ALLOC_00AA1070: u32 = 0x00aa_1070;
/// `NiTriShape::NiTriShape` (`00a74410`, nine words; 0xc4 bytes), the billboard node's
/// constructor (`0051cf00`; 0xb4 bytes) and mode setter (`0051d000`, one word),
/// `NiDirectionalLight::NiDirectionalLight` (`00a75c20`; 0xfc bytes) (names from the
/// engine map where it has them).
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
const BILLBOARD_CONSTRUCT: u32 = 0x0051_cf00;
const BILLBOARD_SET_MODE: u32 = 0x0051_d000;
const LIGHT_CONSTRUCT: u32 = 0x00a7_5c20;
/// A node's scale setter (`00440490`, one `float`) and translation setter (`00440460`, one
/// word: the point) (`thiscall`).
const NODE_SET_SCALE: u32 = 0x0044_0490;
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
/// The light's setters (`thiscall`, one word): dimmer (`0050dd20`, a `float`), the two
/// colours (`004bc280`, `004bc2e0`), the third colour (`0050dd50`) and the rotation
/// (`0043fa80`, a matrix).
const LIGHT_SET_DIMMER: u32 = 0x0050_dd20;
const LIGHT_SET_COLOR_004BC280: u32 = 0x004b_c280;
const LIGHT_SET_COLOR_004BC2E0: u32 = 0x004b_c2e0;
const LIGHT_SET_COLOR_0050DD50: u32 = 0x0050_dd50;
const LIGHT_SET_ROTATE: u32 = 0x0043_fa80;
/// The `NiColor` the light's colours start as (`011a9b7c`).
const COLOR_011A9B7C: u32 = 0x011a_9b7c;
/// `BSShaderData::BSShaderData` (`00baf3c0`) and `BSShaderData::Convert` (`00bb0180`, two
/// words); the local data is 0xb8 bytes.
const SHADER_DATA_CONSTRUCT: u32 = 0x00ba_f3c0;
const SHADER_DATA_CONVERT: u32 = 0x00bb_0180;
/// The base update `008d0600` (`thiscall`: the sky, the time; a bare `RET 8`).
const BASE_UPDATE_008D0600: u32 = 0x008d_0600;
/// `Sky` accessors (`thiscall`): `fCurrentGameHour` (`00966a20`, `+0xec`, `ST0`), the mode
/// (`00476c90`), the colours at `+0x60` (`004ea950`), `+0x6c` (`0043b5d0`), `+0x78`
/// (`00461110`) and `SunSpecularColor` (`00514f30`, `+0xc0`), `pCurrentWeather`
/// (`0044edb0`), `pLastWeather` (`00825c00`), `fCurrentWeatherPct` (`006447d0`, `ST0`), the
/// sunrise begin (`00595ea0`) and end (`00595f50`) and sunset begin (`00595fc0`) and end
/// (`00596030`) hours (`ST0`), and the flag-4 test (`00642680`, translated here).
const SKY_GAME_HOUR: u32 = 0x0096_6a20;
const SKY_GET_MODE: u32 = 0x0047_6c90;
const SKY_COLOR_60: u32 = 0x004e_a950;
const SKY_COLOR_6C: u32 = 0x0043_b5d0;
const SKY_COLOR_78: u32 = 0x0046_1110;
const SKY_SUN_SPECULAR: u32 = 0x0051_4f30;
const SKY_CURRENT_WEATHER: u32 = 0x0044_edb0;
const SKY_LAST_WEATHER: u32 = 0x0082_5c00;
const SKY_WEATHER_PERCENT: u32 = 0x0064_47d0;
const SKY_SUNRISE_BEGIN: u32 = 0x0059_5ea0;
const SKY_SUNRISE_END: u32 = 0x0059_5f50;
const SKY_SUNSET_BEGIN: u32 = 0x0059_5fc0;
const SKY_SUNSET_END: u32 = 0x0059_6030;
/// The identity matrix (`011a9448`, nine words).
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// The pointer global (`011dea10`) whose `+0x34` (`005f36f0`) is the cell `Update`'s
/// interior branch reads and whose `+0xc` (`0084e3a0`) gates the occlusion; the cell's two
/// angles (`00544910`, `00544970`; whole numbers) and the degrees-to-radians `double`
/// (`01023128`).
const TES_011DEA10: u32 = 0x011d_ea10;
const HOLDER_CELL_005F36F0: u32 = 0x005f_36f0;
const HOLDER_VALUE_0084E3A0: u32 = 0x0084_e3a0;
const CELL_ANGLE_00544910: u32 = 0x0054_4910;
const CELL_ANGLE_00544970: u32 = 0x0054_4970;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// The matrix helpers (`thiscall`): `004a0c90` (one `float`), `NiMatrix3::MakeYRotation`
/// (`0043f850`, one `float`), `NiMatrix3::operator*` (`0043f8d0`: the result, the other),
/// the element setter `00642660` (row, column, `float`) and `004769c0` (a column, a point).
const MATRIX_ROTATION_004A0C90: u32 = 0x004a_0c90;
const MATRIX_MAKE_Y_ROTATION: u32 = 0x0043_f850;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const MATRIX_SET_ELEMENT: u32 = 0x0064_2660;
const MATRIX_SET_COLUMN: u32 = 0x0047_69c0;
/// The interior sun angle (`0104f2a8`, a `float`).
const INTERIOR_ANGLE: u32 = 0x0104_f2a8;
/// The nodes' visibility test (`006366b0`) and setter (`00450f90`, one byte: hidden).
const NODE_IS_VISIBLE: u32 = 0x0063_66b0;
const SET_HIDDEN: u32 = 0x0045_0f90;
/// `double`s: 2.0 (`01011590`), 0.5 (`01011588`), 24.0 (`010356d8`), 1.0 (`01012070`).
const TWO: u32 = 0x0101_1590;
const HALF: u32 = 0x0101_1588;
const HOURS_PER_DAY: u32 = 0x0103_56d8;
const ONE: u32 = 0x0101_2070;
/// Settings (their `float`s through `00403e20`, or `00450410` which answers the value in
/// `ST0`): the transition length (`011ccea4`), the sun's height (`011cce94`), its distance
/// (`011ccf0c`), the light's height (`011cceb4`), the glare step (`011cceec`) and the glare
/// limit (`011ccec0`).
const TRANSITION_SETTING: u32 = 0x011c_cea4;
const HEIGHT_SETTING: u32 = 0x011c_ce94;
const DISTANCE_SETTING: u32 = 0x011c_cf0c;
const LIGHT_HEIGHT_SETTING: u32 = 0x011c_ceb4;
const GLARE_STEP_SETTING: u32 = 0x011c_ceec;
const GLARE_LIMIT_SETTING: u32 = 0x011c_cec0;
const SETTING_FLOAT_IN_ST0: u32 = 0x0045_0410;
/// The weather's fade (`004ed230`, `thiscall`: 4, a `float`, a `float`; `ST0`).
const WEATHER_FADE: u32 = 0x004e_d230;
/// `NiAVObject::GetProperty` (`00a59d30`) and a property's type (`00441110`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_TYPE: u32 = 0x0044_1110;
/// The `float` function `00408860` (`cdecl`, one `float`; it calls the CRT's `00ec6cde`).
const FLOAT_FUNCTION_00408860: u32 = 0x0040_8860;
/// The camera holder (`0045c670`), its node (`00558310`) and the node's point (`0043c490`).
const CAMERA_HOLDER_GET: u32 = 0x0045_c670;
const CAMERA_NODE: u32 = 0x0055_8310;
const NODE_POINT_0043C490: u32 = 0x0043_c490;
/// A point's normalisation (`004a0c10`, `thiscall`).
const POINT_UNITIZE: u32 = 0x004a_0c10;
/// `0070ec90` (`thiscall`, `+0x64`; its answer is dropped) and `004e2180` (the byte at
/// `011c7a59`).
const HOLDER_VALUE_0070EC90: u32 = 0x0070_ec90;
const BYTE_GETTER_004E2180: u32 = 0x004e_2180;
/// The occlusion curve `004dd130` (`cdecl`, two `float`s; `ST0`) and its second argument
/// (`010162c0`, a `float`).
const OCCLUSION_CURVE: u32 = 0x004d_d130;
const OCCLUSION_EXPONENT: u32 = 0x0101_62c0;
/// `00642890` (`thiscall` on the `Sun`; clears `bDoOcclusionTests`) and `005585e0` (the
/// `NiPointer` at `+8`: the sun base node).
const CLEAR_OCCLUSION_TESTS: u32 = 0x0064_2890;
const SUN_BASE_NODE: u32 = 0x0055_85e0;

layout! {
    /// `Sun` (Xbox PDB), 0x2c bytes (`Sky::Initialize` allocates 0x2c).
    pub struct Sun: 0x2C {
        /// `SkyObject::spRoot` (Xbox PDB).
        0x04 spRoot: u32,
        /// `spSunBaseNode` (Xbox PDB): `NiPointer<NiBillboardNode>`.
        0x08 spSunBaseNode: u32,
        /// `spSunGlareNode` (Xbox PDB): `NiPointer<NiBillboardNode>`.
        0x0C spSunGlareNode: u32,
        /// `spSunBase` (Xbox PDB): `NiPointer<NiTriShape>`.
        0x10 spSunBase: u32,
        /// `spSunGlare` (Xbox PDB): `NiPointer<NiTriShape>`.
        0x14 spSunGlare: u32,
        /// `pSunGlarePick` (Xbox PDB): `NiPick*`.
        0x18 pSunGlarePick: u32,
        /// `spLight` (Xbox PDB): `NiPointer<NiDirectionalLight>`.
        0x1C spLight: u32,
        /// `fGlareScale` (Xbox PDB).
        0x20 fGlareScale: f32,
        /// `bDoOcclusionTests` (Xbox PDB).
        0x24 bDoOcclusionTests: bool,
        /// `spSunAccumulator` (Xbox PDB): `NiPointer<BSShaderAccumulator>`.
        0x28 spSunAccumulator: u32,
    }
}

/// A `float` loaded with `FLD dword` and pushed or stored with `FSTP dword`.
fn copied_float(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

/// The value of a setting through `00403e20`.
fn setting_value(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

// Translated from 006404f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sun::Sun` (engine map; `thiscall`, no stack words): `SkyObject::SkyObject`, the vtable,
/// the six `NiPointer`s constructed from 0, the root and five of them assigned 0, no pick,
/// glare scale 0, no occlusion tests, and a new accumulator (0x280 bytes,
/// `BSShaderAccumulator(0x64, 1, 1)`; 0 when the allocation fails) as `spSunAccumulator`.
/// Answers `this`.
// Translation incomplete: 006404f0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the base, the pointers and the allocation) is not translated.
pub fn sun_sun(e: &mut Engine, this: Ptr<Sun>) -> Ptr<Sun> {
    let base = this.addr();
    e.call(SKY_OBJECT_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SUN_VTABLE);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x1c, 0x28] {
        e.call(NI_POINTER_FROM_RAW, &args![base + offset, 0u32]);
    }
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x1c] {
        pointer_assign(e, base + offset, 0);
    }
    e.set(this, Sun::pSunGlarePick, 0);
    e.set(this, Sun::fGlareScale, 0.0);
    e.set(this, Sun::bDoOcclusionTests, false);
    let block = e.call(NI_ALLOCATE, &args![0x280u32]).u32();
    let accumulator = if block != 0 {
        e.call(ACCUMULATOR_CONSTRUCT, &args![block, 0x64u32, 1u32, 1u32])
            .u32()
    } else {
        0
    };
    pointer_assign(e, base + 0x28, accumulator);
    this
}

// Translated from 00640670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sun`'s scalar deleting destructor (engine map): the destructor (`006406a0`), then
/// `operator delete` when `flags & 1`. Answers `this`.
pub fn sun_scalar_deleting_destructor(e: &mut Engine, this: Ptr<Sun>, flags: u32) -> Ptr<Sun> {
    fn_006406a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006406a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sun`'s destructor (unnamed in the engine map; the scalar deleting destructor runs it;
/// `thiscall`, no stack words): the vtable, the root and five pointers assigned 0, the pick
/// deleted (`006407e0` with 1) when there is one (the word is left as it is), the
/// accumulator assigned 0, the six `NiPointer` destructors, then `SkyObject::~SkyObject`.
// Translation incomplete: 006406a0 - the C++ exception frame (FS:[0] chain, unwind states 6 to 0 for the members) is not translated.
pub fn fn_006406a0(e: &mut Engine, this: Ptr<Sun>) {
    let base = this.addr();
    e.mem.set_u32(base, SUN_VTABLE);
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x1c] {
        pointer_assign(e, base + offset, 0);
    }
    let pick = e.get(this, Sun::pSunGlarePick);
    if pick != 0 {
        fn_006407e0(e, Ptr::new(pick), 1);
    }
    pointer_assign(e, base + 0x28, 0);
    for offset in [0x28u32, 0x1c, 0x14, 0x10, 0xc, 8] {
        e.call(NI_POINTER_DESTRUCT, &args![base + offset]);
    }
    e.call(SKY_OBJECT_DESTRUCT, &args![base]);
}

// Translated from 006407e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPick`'s scalar deleting destructor as compiled here (unnamed in the engine map):
/// `NiPick::~NiPick`, then `operator delete` when `flags & 1`. Answers `this`.
pub fn fn_006407e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(PICK_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// One coordinate `Initialize` pushes: 0, a setting's value, or the value times -1.
#[derive(Clone, Copy)]
enum Coordinate {
    Zero,
    Size(u32),
    MinusSize(u32),
}

/// A coordinate's pushed word, its setting read through `00403e20` as the code does.
fn coordinate(e: &mut Engine, which: Coordinate) -> u32 {
    match which {
        Coordinate::Zero => 0.0f32.to_bits(),
        Coordinate::Size(setting) => {
            let value = setting_value(e, setting);
            x87_quiet_bits(value)
        }
        Coordinate::MinusSize(setting) => {
            let value = setting_value(e, setting);
            let minus_one = e.global::<f64>(MINUS_ONE);
            x87_quiet_bits(x87_store32(x87_mul(x87_m32(value), minus_one)))
        }
    }
}

/// An array of four elements of `size` bytes from `operator new` and the vector
/// constructor (0 when the allocation fails).
fn new_array(e: &mut Engine, size: u32, constructor: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size * 4]).u32();
    if block == 0 {
        return 0;
    }
    e.call(VECTOR_CONSTRUCT, &args![block, size, 4u32, constructor]);
    block
}

/// Four quad corners of `setting`'s size, each built through `00416870` from its
/// coordinates in push order (z, y, x) and copied into the array.
fn quad_points(e: &mut Engine, setting: u32) -> u32 {
    use Coordinate::{MinusSize, Size, Zero};
    let points = new_array(e, 0xc, ELEMENT_CONSTRUCT_006815C0);
    let corners = [
        [Zero, Size(setting), MinusSize(setting)],
        [Zero, MinusSize(setting), MinusSize(setting)],
        [Zero, Size(setting), Size(setting)],
        [Zero, MinusSize(setting), Size(setting)],
    ];
    for (index, corner) in corners.into_iter().enumerate() {
        let z = coordinate(e, corner[0]);
        let y = coordinate(e, corner[1]);
        let x = coordinate(e, corner[2]);
        e.with_stack(12, |e, local| {
            let made = e
                .call(POINT3_CONSTRUCT, &args![local.addr(), x, y, z])
                .u32();
            for word in 0..3 {
                let value = e.mem.u32(made + 4 * word);
                e.mem.set_u32(points + 0xc * index as u32 + 4 * word, value);
            }
        });
    }
    points
}

/// Four normals, each the three words at `011f426c`.
fn quad_normals(e: &mut Engine) -> u32 {
    let normals = new_array(e, 0xc, ELEMENT_CONSTRUCT_006815C0);
    for index in 0..4u32 {
        for word in 0..3 {
            let value = e.mem.u32(NORMAL_011F426C + 4 * word);
            e.mem.set_u32(normals + 0xc * index + 4 * word, value);
        }
    }
    normals
}

/// Four colours, each the four words of `color`.
fn quad_colors(e: &mut Engine, color: u32) -> u32 {
    let colors = new_array(e, 0x10, ELEMENT_CONSTRUCT_004A7800);
    for index in 0..4u32 {
        for word in 0..4 {
            let value = e.mem.u32(color + 4 * word);
            e.mem.set_u32(colors + 0x10 * index + 4 * word, value);
        }
    }
    colors
}

/// Four texture coordinates (0,0), (0,1), (1,0), (1,1), each through `00452dc0`.
fn quad_texture_coordinates(e: &mut Engine) -> u32 {
    let coordinates = new_array(e, 8, ELEMENT_CONSTRUCT_006815C0);
    let (zero, one) = (0.0f32.to_bits(), 1.0f32.to_bits());
    for (index, (u, v)) in [(zero, zero), (zero, one), (one, zero), (one, one)]
        .into_iter()
        .enumerate()
    {
        e.with_stack(8, |e, local| {
            let made = e.call(POINT2_CONSTRUCT, &args![local.addr(), u, v]).u32();
            for word in 0..2 {
                let value = e.mem.u32(made + 4 * word);
                e.mem
                    .set_u32(coordinates + 8 * index as u32 + 4 * word, value);
            }
        });
    }
    coordinates
}

/// The two triangles (0, 1, 2) and (2, 1, 3) in a 12-byte `00aa1070` block.
fn quad_triangles(e: &mut Engine) -> u32 {
    let triangles = e.call(NI_ALLOC_00AA1070, &args![0xcu32]).u32();
    for (index, value) in [0u16, 1, 2, 2, 1, 3].into_iter().enumerate() {
        e.mem.set_u16(triangles + 2 * index as u32, value);
    }
    triangles
}

/// A quad shape (`NiTriShape`, 0xc4 bytes; 0 when the allocation fails).
fn quad_shape(e: &mut Engine, arrays: [u32; 5]) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    let [points, normals, colors, coordinates, triangles] = arrays;
    e.call(
        TRI_SHAPE_CONSTRUCT,
        &args![
            block,
            4u32,
            points,
            normals,
            colors,
            coordinates,
            1u32,
            0u32,
            2u32,
            triangles
        ],
    )
    .u32()
}

/// A billboard node (0xb4 bytes; 0 when the allocation fails).
fn billboard(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0xb4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(BILLBOARD_CONSTRUCT, &args![block]).u32()
}

// Translated from 00640810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sun::Initialize` (engine map; `thiscall`, one word: the parent node).
/// `SkyObject::Initialize`; two quads (corners `(-s, s, 0)`, `(-s, -s, 0)`, `(s, s, 0)`,
/// `(s, -s, 0)` of the base and glare size settings, normals from `011f426c`, colour
/// `(1, 0, 0, 1)`, texture coordinates (0,0), (0,1), (1,0), (1,1), triangles 0 1 2 / 2 1 3)
/// as `spSunBase` and `spSunGlare`, each with flag 1 (`004902f0`) and in a billboard node
/// (mode 2, flag 1) as `spSunBaseNode` and `spSunGlareNode`; the glare node takes the glare
/// scale; the base node is attached to the root. A directional light (dimmer 1, colours
/// from `011a9b7c`) as `spLight`, attached to the root. A pick (`NiPick(0, 8)`, 0x34
/// bytes) as `pSunGlarePick` with its four setters at 1. Then a local `BSShaderData` with
/// lighting kind 6 converts both quads (bit 0: the shader property), sky type 0 for the base
/// and 1 for the glare.
// Translation incomplete: 00640810 - the C++ exception frame (FS:[0] chain, unwind states 0 to 13 for the allocations) and the /GS cookie check (00ec408c) are not translated.
pub fn sun_initialize(e: &mut Engine, this: Ptr<Sun>, parent: u32) {
    let base = this.addr();
    e.call(SKY_OBJECT_INITIALIZE, &args![base, parent]);
    let base_points = quad_points(e, SUN_BASE_SIZE_SETTING);
    let base_normals = quad_normals(e);
    let glare_points = quad_points(e, SUN_GLARE_SIZE_SETTING);
    let glare_normals = quad_normals(e);
    let color = e.mem.alloc(16);
    let (zero, one) = (0.0f32.to_bits(), 1.0f32.to_bits());
    e.call(COLOR_A_CONSTRUCT, &args![color, one, zero, zero, one]);
    let base_colors = quad_colors(e, color);
    let glare_colors = quad_colors(e, color);
    e.mem.free(color);
    let base_coordinates = quad_texture_coordinates(e);
    let glare_coordinates = quad_texture_coordinates(e);
    let base_triangles = quad_triangles(e);
    let glare_triangles = quad_triangles(e);

    let shape = quad_shape(
        e,
        [
            base_points,
            base_normals,
            base_colors,
            base_coordinates,
            base_triangles,
        ],
    );
    pointer_assign(e, base + 0x10, shape);
    let shape = pointer_get(e, base + 0x10);
    e.call(NODE_SET_FLAG_004902F0, &args![shape, 1u32]);
    let node = billboard(e);
    pointer_assign(e, base + 8, node);
    let node = pointer_get(e, base + 8);
    e.call(NODE_SET_FLAG_004902F0, &args![node, 1u32]);
    let node = pointer_get(e, base + 8);
    e.call(BILLBOARD_SET_MODE, &args![node, 2u32]);
    let node = pointer_get(e, base + 8);
    let shape = pointer_get(e, base + 0x10);
    e.vcall(node, 0xdc, &args![shape, 1u32]);

    let shape = quad_shape(
        e,
        [
            glare_points,
            glare_normals,
            glare_colors,
            glare_coordinates,
            glare_triangles,
        ],
    );
    pointer_assign(e, base + 0x14, shape);
    let shape = pointer_get(e, base + 0x14);
    e.call(NODE_SET_FLAG_004902F0, &args![shape, 1u32]);
    let node = billboard(e);
    pointer_assign(e, base + 0xc, node);
    let node = pointer_get(e, base + 0xc);
    e.call(NODE_SET_FLAG_004902F0, &args![node, 1u32]);
    let node = pointer_get(e, base + 0xc);
    e.call(BILLBOARD_SET_MODE, &args![node, 2u32]);
    let scale = copied_float(e, base + 0x20);
    let node = pointer_get(e, base + 0xc);
    e.call(NODE_SET_SCALE, &args![node, scale]);
    let node = pointer_get(e, base + 0xc);
    let shape = pointer_get(e, base + 0x14);
    e.vcall(node, 0xdc, &args![shape, 1u32]);

    let root = pointer_get(e, base + 4);
    let node = pointer_get(e, base + 8);
    e.vcall(root, 0xdc, &args![node, 1u32]);

    let block = e.call(NI_ALLOCATE, &args![0xfcu32]).u32();
    let light = if block != 0 {
        e.call(LIGHT_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    pointer_assign(e, base + 0x1c, light);
    let light = pointer_get(e, base + 0x1c);
    e.call(LIGHT_SET_DIMMER, &args![light, one]);
    let light = pointer_get(e, base + 0x1c);
    e.call(LIGHT_SET_COLOR_004BC280, &args![light, COLOR_011A9B7C]);
    let light = pointer_get(e, base + 0x1c);
    e.call(LIGHT_SET_COLOR_004BC2E0, &args![light, COLOR_011A9B7C]);
    let root = pointer_get(e, base + 4);
    let light = pointer_get(e, base + 0x1c);
    e.vcall(root, 0xdc, &args![light, 1u32]);

    let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
    let pick = if block != 0 {
        e.call(PICK_CONSTRUCT, &args![block, 0u32, 8u32]).u32()
    } else {
        0
    };
    e.set(this, Sun::pSunGlarePick, pick);
    for setter in [
        PICK_SET_00537E90,
        PICK_SET_00403550,
        PICK_SET_00632D20,
        PICK_SET_00458B30,
    ] {
        let pick = e.get(this, Sun::pSunGlarePick);
        e.call(setter, &args![pick, 1u32]);
    }

    e.with_stack(0xb8, |e, data| {
        let data = data.addr();
        e.call(SHADER_DATA_CONSTRUCT, &args![data]);
        e.mem.set_u32(data, 6);
        e.mem.set_u32(data + 0x90, 0);
        let shape = pointer_get(e, base + 0x10);
        e.call(SHADER_DATA_CONVERT, &args![data, shape, 1u32]);
        e.mem.set_u32(data + 0x90, 1);
        let shape = pointer_get(e, base + 0x14);
        e.call(SHADER_DATA_CONVERT, &args![data, shape, 1u32]);
    });
}

/// `Update`'s light orientation outside the exterior modes (sky modes 0 and 1): the
/// identity, turned by the current cell's two angles (degrees, `004a0c90` and the Y
/// rotation, multiplied) when the holder at `011dea10` has a cell, else with elements
/// (1, 0) and (2, 0) set to `0104f2a8`; it becomes the light's rotation, and the light takes
/// the sky's colours at `+0x60`, `+0x6c` and `SunSpecularColor`.
fn update_interior_light(e: &mut Engine, base: u32, sky: u32) {
    e.with_stack(0x24 * 3, |e, frame| {
        let matrix = frame.addr();
        let turn = matrix + 0x24;
        let product = matrix + 0x48;
        for word in 0..9 {
            let value = e.mem.u32(IDENTITY_MATRIX + 4 * word);
            e.mem.set_u32(matrix + 4 * word, value);
        }
        let holder = e.global::<u32>(TES_011DEA10);
        let cell = if holder != 0 {
            e.call(HOLDER_CELL_005F36F0, &args![holder]).u32()
        } else {
            0
        };
        if cell != 0 {
            e.call(ELEMENT_CONSTRUCT_006815C0, &args![turn]);
            let to_radians = e.global::<f64>(DEGREES_TO_RADIANS);
            let first = e.call(CELL_ANGLE_00544910, &args![cell]).u32();
            let angle = x87_store32(x87_mul(f64::from(first), to_radians));
            e.call(
                MATRIX_ROTATION_004A0C90,
                &args![matrix, x87_quiet_bits(angle)],
            );
            let second = e.call(CELL_ANGLE_00544970, &args![cell]).u32();
            let angle = x87_store32(x87_mul(f64::from(second), to_radians));
            e.call(MATRIX_MAKE_Y_ROTATION, &args![turn, x87_quiet_bits(angle)]);
            let made = e.call(MATRIX_MULTIPLY, &args![matrix, product, turn]).u32();
            for word in 0..9 {
                let value = e.mem.u32(made + 4 * word);
                e.mem.set_u32(matrix + 4 * word, value);
            }
        } else {
            for row in [1u32, 2] {
                let angle = copied_float(e, INTERIOR_ANGLE);
                e.call(MATRIX_SET_ELEMENT, &args![matrix, row, 0u32, angle]);
            }
        }
        let light = pointer_get(e, base + 0x1c);
        e.call(LIGHT_SET_ROTATE, &args![light, matrix]);
    });
    set_light_colors(e, base, sky);
}

/// The light's three colours from the sky (`+0x60`, `+0x6c`, `SunSpecularColor`).
fn set_light_colors(e: &mut Engine, base: u32, sky: u32) {
    for (getter, setter) in [
        (SKY_COLOR_60, LIGHT_SET_COLOR_004BC280),
        (SKY_COLOR_6C, LIGHT_SET_COLOR_004BC2E0),
        (SKY_SUN_SPECULAR, LIGHT_SET_COLOR_0050DD50),
    ] {
        let color = e.call(getter, &args![sky]).u32();
        let light = pointer_get(e, base + 0x1c);
        e.call(setter, &args![light, color]);
    }
}

/// A quad's sky shader property (property 3 of type 0xd), else 0, read as the code does.
fn quad_sky_property(e: &mut Engine, slot: u32) -> u32 {
    let property = |e: &mut Engine| {
        let shape = pointer_get(e, slot);
        e.call(GET_PROPERTY, &args![shape, 3u32]).u32()
    };
    let is_sky = if property(e) == 0 {
        false
    } else {
        let found = property(e);
        e.call(PROPERTY_TYPE, &args![found]).u32() == 0xd
    };
    if is_sky {
        property(e)
    } else {
        0
    }
}

/// The quad's vertex colour: the sky's colour at `+0x78` with alpha `alpha` (each word
/// read through `00461110` again).
fn set_quad_color(e: &mut Engine, property: u32, sky: u32, alpha: u32) {
    let alpha = x87_quiet_bits(alpha);
    let mut words = [0u32; 3];
    for word in [2usize, 1, 0] {
        let color = e.call(SKY_COLOR_78, &args![sky]).u32();
        words[word] = copied_float(e, color + 4 * word as u32);
    }
    e.with_stack(16, |e, local| {
        let made = e
            .call(
                COLOR_A_CONSTRUCT,
                &args![local.addr(), words[0], words[1], words[2], alpha],
            )
            .u32();
        for word in 0..4 {
            let value = e.mem.u32(made + 4 * word);
            e.mem.set_u32(property + 0x60 + 4 * word, value);
        }
    });
}

// Translated from 00641830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sun::Update` (engine map; `thiscall`, two words: the sky, the time), inside a scope
/// timer (kind 0x21, line 0x12c). The base update `008d0600`. In sky modes 0 and 1 only the
/// light is oriented (see `update_interior_light`). In modes 3 and 2:
/// - the root is hidden unless its node is visible (`006366b0`);
/// - from the sunrise and sunset middles (the averages of their begin and end hours) and
///   half the transition setting, the sun's brightness `f` is 0 before the sunrise starts
///   or after the sunset ends, rises over the sunrise, falls over the sunset and is 1
///   between (each test as the code's flag tests have it, NaN included);
/// - the base quad's vertex colour is the sky's `+0x78` colour with alpha `f`, the glare's
///   with alpha the weathers' fades (`004ed230(4, pct * f, 0)` of the current weather plus
///   `004ed230(4, (1 - pct) * f, 0)` of the last);
/// - the sun's height follows the hour (rising over the day, falling after the sunset and
///   before the sunrise) times the height setting; the root and the glare node move to
///   (height, distance setting, `00408860(height setting) - 00408860(height)`); the light
///   turns towards `(-height, distance, light height)` normalised, and takes the sky's
///   colours;
/// - when the holder at `011dea10` has its `+0xc`, the glare scale moves by the glare step
///   (the step setting times the time) towards the occlusion (`004dd130(1 - occlusion, 2)`)
///   or down to 0 below the glare limit; the glare node shows with that scale, or hides;
/// - the sun base node is hidden while the sky's flag 4 is set.
// Translation incomplete: 00641830 - the C++ exception frame (FS:[0] chain, unwind state 0 for the scope timer) is not translated.
pub fn sun_update(e: &mut Engine, this: Ptr<Sun>, sky: u32, time: f32) {
    let base = this.addr();
    let timer = e.mem.alloc(8);
    e.call(
        TIMER_SCOPE_CONSTRUCT,
        &args![timer, 0x21u32, 1u32, SUN_SOURCE_FILE_NAME, 0x12cu32],
    );
    sun_update_body(e, base, sky, time);
    e.call(TIMER_SCOPE_DESTROY, &args![timer]);
    e.mem.free(timer);
}

fn sun_update_body(e: &mut Engine, base: u32, sky: u32, time: f32) {
    // The direction local starts as the point at 011f426c.
    let mut direction = [0u32; 3];
    for (word, slot) in direction.iter_mut().enumerate() {
        *slot = e.mem.u32(NORMAL_011F426C + 4 * word as u32);
    }
    let hour = x87_store32(e.call(SKY_GAME_HOUR, &args![sky]).f64());
    e.call(
        BASE_UPDATE_008D0600,
        &args![base, sky, x87_quiet_bits(time.to_bits())],
    );
    if e.call(SKY_GET_MODE, &args![sky]).u32() == 0 {
        update_interior_light(e, base, sky);
        return;
    }
    if e.call(SKY_GET_MODE, &args![sky]).u32() == 1 {
        update_interior_light(e, base, sky);
        return;
    }
    let exterior = e.call(SKY_GET_MODE, &args![sky]).u32() == 3
        || e.call(SKY_GET_MODE, &args![sky]).u32() == 2;
    if !exterior {
        return;
    }
    let root = pointer_get(e, base + 4);
    let visible = e.call(NODE_IS_VISIBLE, &args![root]).u8();
    let root = pointer_get(e, base + 4);
    e.call(SET_HIDDEN, &args![root, (visible == 0) as u32]);

    let two = e.global::<f64>(TWO);
    let rise_end = e.call(SKY_SUNRISE_END, &args![sky]).f64();
    let rise_begin = e.call(SKY_SUNRISE_BEGIN, &args![sky]).f64();
    let rise_middle = x87_store32(x87_div(x87_add(rise_begin, rise_end), two));
    let set_end = e.call(SKY_SUNSET_END, &args![sky]).f64();
    let set_begin = e.call(SKY_SUNSET_BEGIN, &args![sky]).f64();
    let set_middle = x87_store32(x87_div(x87_add(set_begin, set_end), two));
    let transition = setting_value(e, TRANSITION_SETTING);
    let half = x87_store32(x87_mul(x87_m32(transition), e.global::<f64>(HALF)));
    let m = x87_m32;
    let rise_low = x87_store32(x87_sub(m(rise_middle), m(half)));
    let rise_high = x87_store32(x87_add(m(rise_middle), m(half)));
    let set_low = x87_store32(x87_sub(m(set_middle), m(half)));
    let set_high = x87_store32(x87_add(m(set_middle), m(half)));
    let h = m(hour);

    // The brightness.
    let mut brightness = 1.0f32.to_bits();
    // FCOMPP, TEST AH,1: the jump is taken when hour < rise_low or unordered.
    if !(h >= m(rise_low) && m(set_high) >= h) {
        brightness = 0.0f32.to_bits();
    } else if h > m(rise_low) && m(rise_high) > h {
        brightness = x87_store32(x87_div(
            x87_sub(h, m(rise_low)),
            x87_sub(m(rise_high), m(rise_low)),
        ));
    } else if h > m(set_low) && m(set_high) > h {
        let fraction = x87_div(x87_sub(h, m(set_low)), x87_sub(m(set_high), m(set_low)));
        brightness = x87_store32(x87_sub(1.0, fraction));
    }

    // The weathers' fade of the glare.
    let current = e.call(SKY_CURRENT_WEATHER, &args![sky]).u32();
    let last = e.call(SKY_LAST_WEATHER, &args![sky]).u32();
    let percent = x87_store32(e.call(SKY_WEATHER_PERCENT, &args![sky]).f64());
    let zero = 0.0f32.to_bits();
    let current_fade = if current != 0 {
        let amount = x87_store32(x87_mul(m(percent), m(brightness)));
        let fade = e
            .call(
                WEATHER_FADE,
                &args![current, 4u32, x87_quiet_bits(amount), zero],
            )
            .f64();
        x87_store32(fade)
    } else {
        x87_quiet_bits(brightness)
    };
    let last_fade = if last != 0 {
        let amount = x87_store32(x87_mul(x87_sub(1.0, m(percent)), m(brightness)));
        let fade = e
            .call(
                WEATHER_FADE,
                &args![last, 4u32, x87_quiet_bits(amount), zero],
            )
            .f64();
        x87_store32(fade)
    } else {
        zero
    };
    let glare_alpha = x87_store32(x87_add(m(current_fade), m(last_fade)));

    let property = quad_sky_property(e, base + 0x10);
    if property != 0 {
        set_quad_color(e, property, sky, brightness);
    }
    let property = quad_sky_property(e, base + 0x14);
    if property != 0 {
        set_quad_color(e, property, sky, glare_alpha);
    }

    // The sun's height.
    let mut height = 0.0f32.to_bits();
    let hours_per_day = e.global::<f64>(HOURS_PER_DAY);
    let one = e.global::<f64>(ONE);
    if h > m(rise_low) && m(set_high) > h {
        let fraction = x87_store32(x87_div(
            x87_sub(h, m(rise_low)),
            x87_sub(m(set_high), m(rise_low)),
        ));
        let factor = x87_sub(1.0, x87_mul(m(fraction), two));
        let scale = setting_value(e, HEIGHT_SETTING);
        height = x87_store32(x87_mul(m(scale), factor));
    } else if m(set_high) <= h {
        // FCOMPP of set_high against the hour, TEST AH,0x41, JP: below or equal, ordered.
        let numerator = x87_sub(h, m(set_high));
        let denominator = x87_sub(hours_per_day, x87_sub(m(set_high), m(rise_low)));
        let fraction = x87_store32(x87_div(numerator, denominator));
        let factor = x87_sub(x87_mul(m(fraction), two), one);
        let scale = setting_value(e, HEIGHT_SETTING);
        height = x87_store32(x87_mul(m(scale), factor));
    } else if m(rise_low) >= h {
        // FCOMPP of rise_low against the hour, TEST AH,1, JNZ: skipped when below or
        // unordered.
        let numerator = x87_add(x87_sub(hours_per_day, m(set_high)), h);
        let denominator = x87_sub(hours_per_day, x87_sub(m(set_high), m(rise_low)));
        let fraction = x87_store32(x87_div(numerator, denominator));
        let factor = x87_sub(x87_mul(m(fraction), two), one);
        let scale = setting_value(e, HEIGHT_SETTING);
        height = x87_store32(x87_mul(m(scale), factor));
    }
    direction[0] = x87_quiet_bits(height);
    direction[1] = x87_store32(e.call(SETTING_FLOAT_IN_ST0, &args![DISTANCE_SETTING]).f64());
    let height_setting = x87_store32(e.call(SETTING_FLOAT_IN_ST0, &args![HEIGHT_SETTING]).f64());
    let first = e
        .call(
            FLOAT_FUNCTION_00408860,
            &args![x87_quiet_bits(height_setting)],
        )
        .f64();
    let second = e
        .call(FLOAT_FUNCTION_00408860, &args![x87_quiet_bits(height)])
        .f64();
    direction[2] = x87_store32(x87_sub(first, second));

    e.with_stack(0x40, |e, frame| {
        let point = frame.addr();
        let light_point = point + 0xc;
        let matrix = point + 0x18;
        for (word, value) in direction.iter().enumerate() {
            e.mem.set_u32(point + 4 * word as u32, *value);
        }
        let root = pointer_get(e, base + 4);
        e.call(NODE_SET_TRANSLATE, &args![root, point]);
        let glare = pointer_get(e, base + 0xc);
        e.call(NODE_SET_TRANSLATE, &args![glare, point]);
        for word in 0..9 {
            let value = e.mem.u32(IDENTITY_MATRIX + 4 * word);
            e.mem.set_u32(matrix + 4 * word, value);
        }
        let z = x87_store32(
            e.call(SETTING_FLOAT_IN_ST0, &args![LIGHT_HEIGHT_SETTING])
                .f64(),
        );
        let y = x87_store32(e.call(SETTING_FLOAT_IN_ST0, &args![DISTANCE_SETTING]).f64());
        // FLD, FCHS, FSTP: the sign flipped (a NaN's too).
        let x = x87_quiet_bits(height) ^ 0x8000_0000;
        e.call(POINT3_CONSTRUCT, &args![light_point, x, y, z]);
        e.call(POINT_UNITIZE, &args![light_point]);
        e.call(MATRIX_SET_COLUMN, &args![matrix, 0u32, light_point]);
        let light = pointer_get(e, base + 0x1c);
        e.call(LIGHT_SET_ROTATE, &args![light, matrix]);
    });
    set_light_colors(e, base, sky);

    let holder = e.global::<u32>(TES_011DEA10);
    if holder != 0 && e.call(HOLDER_VALUE_0084E3A0, &args![holder]).u32() != 0 {
        update_glare(e, base, time, direction);
    }
    let hidden = fn_00642680(e, sky) != 0;
    let node = e.call(SUN_BASE_NODE, &args![base]).u32();
    e.call(SET_HIDDEN, &args![node, hidden as u32]);
}

/// `Update`'s glare: the camera node's point and the normalised direction are taken into
/// locals (nothing reads them afterwards); the occlusion is `004dd130(1 - occlusion, 2)` of
/// the accumulator's `float` (`006427c0`), and occlusion tests are switched off
/// (`00642890`). When the accumulator's value is at least 1 or the byte `004e2180` reads is
/// set, or the glare limit is above the occlusion, the glare scale drops by the step (to 0
/// and hidden when the limit is above the result); otherwise it moves up by the step
/// towards the occlusion. A glare node that is not visible is hidden.
fn update_glare(e: &mut Engine, base: u32, time: f32, direction: [u32; 3]) {
    let m = x87_m32;
    let holder = e.call(CAMERA_HOLDER_GET, &args![]).u32();
    let node = e.call(CAMERA_NODE, &args![holder]).u32();
    let point = e.call(NODE_POINT_0043C490, &args![node]).u32();
    let step = e.with_stack(0x1c, |e, frame| {
        let origin = frame.addr();
        let ray = origin + 0xc;
        for word in 0..3 {
            let value = e.mem.u32(point + 4 * word);
            e.mem.set_u32(origin + 4 * word, value);
            e.mem.set_u32(ray + 4 * word, direction[word as usize]);
        }
        let scale = setting_value(e, GLARE_STEP_SETTING);
        let step = x87_store32(x87_mul(m(scale), m(time.to_bits())));
        e.mem.set_u32(ray + 0xc, step);
        e.call(POINT_UNITIZE, &args![ray]);
        step
    });
    let this = Ptr::<Sun>::new(base);
    let value = fn_006427c0(e, this);
    // FCOMP with 1.0, TEST AH,1, JZ: at least 1 (ordered) sets the flag at once.
    let forced = if x87_load32(value.to_bits()) >= e.global::<f64>(ONE) {
        true
    } else {
        let holder = e.global::<u32>(TES_011DEA10);
        e.call(HOLDER_VALUE_0070EC90, &args![holder]);
        e.call(BYTE_GETTER_004E2180, &args![]).u8() != 0
    };
    let exponent = copied_float(e, OCCLUSION_EXPONENT);
    let value = fn_006427c0(e, this);
    let rest = x87_store32(x87_sub(1.0, x87_load32(value.to_bits())));
    let occlusion = x87_store32(
        e.call(OCCLUSION_CURVE, &args![x87_quiet_bits(rest), exponent])
            .f64(),
    );
    e.call(CLEAR_OCCLUSION_TESTS, &args![this]);
    let goes_down = forced || {
        let limit = setting_value(e, GLARE_LIMIT_SETTING);
        // FCOMPP of the limit against the occlusion, TEST AH,0x41, JNZ: above, ordered.
        m(limit) > m(occlusion)
    };
    if goes_down {
        let scale = e.mem.u32(base + 0x20);
        let lowered = x87_sub(m(scale), m(step));
        let limit = setting_value(e, GLARE_LIMIT_SETTING);
        if m(limit) > lowered {
            let glare = pointer_get(e, base + 0xc);
            e.call(SET_HIDDEN, &args![glare, 1u32]);
            e.mem.set_f32(base + 0x20, 0.0);
        } else {
            let glare = pointer_get(e, base + 0xc);
            e.call(SET_HIDDEN, &args![glare, 0u32]);
            let scale = e.mem.u32(base + 0x20);
            let lowered = x87_store32(x87_sub(m(scale), m(step)));
            e.mem.set_u32(base + 0x20, lowered);
            let scale = copied_float(e, base + 0x20);
            let glare = pointer_get(e, base + 0xc);
            e.call(NODE_SET_SCALE, &args![glare, scale]);
        }
    } else {
        let scale = e.mem.u32(base + 0x20);
        let raised = x87_add(m(scale), m(step));
        // FCOMPP of the occlusion against scale + step, TEST AH,0x41, JNP: not above.
        let reaches = m(occlusion) <= raised;
        let takes_occlusion = reaches || {
            let scale = e.mem.u32(base + 0x20);
            // TEST AH,0x41, JP: above or unordered adds the step.
            m(occlusion) <= m(scale)
        };
        if takes_occlusion {
            e.mem.set_u32(base + 0x20, x87_quiet_bits(occlusion));
        } else {
            let scale = e.mem.u32(base + 0x20);
            let raised = x87_store32(x87_add(m(scale), m(step)));
            e.mem.set_u32(base + 0x20, raised);
        }
        let glare = pointer_get(e, base + 0xc);
        e.call(SET_HIDDEN, &args![glare, 0u32]);
        let scale = copied_float(e, base + 0x20);
        let glare = pointer_get(e, base + 0xc);
        e.call(NODE_SET_SCALE, &args![glare, scale]);
    }
    let glare = pointer_get(e, base + 0xc);
    if e.call(NODE_IS_VISIBLE, &args![glare]).u8() == 0 {
        let glare = pointer_get(e, base + 0xc);
        e.call(SET_HIDDEN, &args![glare, 1u32]);
    }
}

// Translated from 00642680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `Sky`, no stack words): 1 when flag 4 of
/// `uiFlags` (`+0x118`) is set, else 0.
pub fn fn_00642680(e: &mut Engine, sky: u32) -> u32 {
    (e.mem.u32(sky + 0x118) & 4 != 0) as u32
}

// Translated from 006427c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `Sun`, no stack words), inside a scope timer
/// (kind 0x21, line 0x235): the accumulator's `float` (`00508050`) rounded to a `float`, 0
/// without an accumulator.
// Translation incomplete: 006427c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the scope timer) is not translated.
pub fn fn_006427c0(e: &mut Engine, this: Ptr<Sun>) -> f32 {
    let timer = e.mem.alloc(8);
    e.call(
        TIMER_SCOPE_CONSTRUCT,
        &args![timer, 0x21u32, 1u32, SUN_SOURCE_FILE_NAME, 0x235u32],
    );
    let accumulator = pointer_get(e, this.addr() + 0x28);
    let value = if accumulator != 0 {
        x87_store32(e.call(ACCUMULATOR_FLOAT, &args![accumulator]).f64())
    } else {
        0.0f32.to_bits()
    };
    e.call(TIMER_SCOPE_DESTROY, &args![timer]);
    e.mem.free(timer);
    f32::from_bits(value)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_04f0, sun_sun(Ptr<Sun>) -> Ptr<Sun>),
        entry!(
            0x0064_0670,
            sun_scalar_deleting_destructor(Ptr<Sun>, u32) -> Ptr<Sun>
        ),
        entry!(0x0064_06a0, fn_006406a0(Ptr<Sun>)),
        entry!(0x0064_07e0, fn_006407e0(Ptr, u32) -> Ptr),
        entry!(0x0064_0810, sun_initialize(Ptr<Sun>, u32)),
        entry!(0x0064_1830, sun_update(Ptr<Sun>, u32, f32)),
        entry!(0x0064_2680, fn_00642680(u32) -> u32),
        entry!(0x0064_27c0, fn_006427c0(Ptr<Sun>) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::fallout_shared_sky::skyobject::test_support::*;
    use crate::units::fallout_shared_sky::skyobject::NI_POINTER_ASSIGN;

    #[test]
    fn constructor_clears_the_members_and_makes_the_accumulator() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[SKY_OBJECT_CONSTRUCT]);
        e.register(NI_ALLOCATE, |_, _| answer(0x0028_0000));
        e.register(ACCUMULATOR_CONSTRUCT, |_, a| answer(a[0] + 1));
        let this = e.mem.alloc(0x2c);
        for offset in (0..0x2c).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_04f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SUN_VTABLE);
        for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x24), 0);
        assert_eq!(
            calls(&e, ACCUMULATOR_CONSTRUCT),
            vec![vec![0x0028_0000, 0x64, 1, 1]]
        );
        assert_eq!(e.mem.u32(this + 0x28), 0x0028_0001);
        assert_eq!(calls(&e, NI_POINTER_FROM_RAW).len(), 6);
        // A failed allocation leaves no accumulator.
        e.register(NI_ALLOCATE, |_, _| answer(0));
        e.call(0x0064_04f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
    }

    #[test]
    fn destructors_delete_the_pick() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                NI_POINTER_DESTRUCT,
                SKY_OBJECT_DESTRUCT,
                OPERATOR_DELETE,
                PICK_DESTRUCT,
            ],
        );
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x18, 0x0000_3400);
        e.mem.set_u32(this + 0x28, 0x0000_2800);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_0670, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SUN_VTABLE);
        assert_eq!(calls(&e, PICK_DESTRUCT), vec![vec![0x3400]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x3400], vec![this]]);
        // The pick word is left; the accumulator is cleared.
        assert_eq!(e.mem.u32(this + 0x18), 0x3400);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 6);
        assert_eq!(calls(&e, SKY_OBJECT_DESTRUCT), vec![vec![this]]);
        // Without a pick and without the delete bit.
        e.mem.set_u32(this + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x0064_0670, &args![this, 0u32]);
        assert!(calls(&e, PICK_DESTRUCT).is_empty());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        // The pick's own scalar deleting destructor.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_07e0, &args![0x3400u32, 0u32]).u32(), 0x3400);
        assert_eq!(calls(&e, PICK_DESTRUCT), vec![vec![0x3400]]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    /// Setting objects answering their `float`s through `00403e20` (a pointer) and
    /// `00450410` (`ST0`).
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        let table: Vec<(u32, u32)> = values
            .iter()
            .map(|&(setting, value)| {
                let slot = e.mem.alloc(4);
                e.mem.set_f32(slot, value);
                (setting, slot)
            })
            .collect();
        let lookup = table.clone();
        e.register_double(SETTING_FLOAT_VALUE, move |_, a| {
            answer(lookup.iter().find(|(s, _)| *s == a[0]).unwrap().1)
        });
        e.register_double(SETTING_FLOAT_IN_ST0, move |e, a| {
            let slot = table.iter().find(|(s, _)| *s == a[0]).unwrap().1;
            answer_float(e.mem.f32(slot))
        });
    }

    #[test]
    fn initialize_builds_both_quads_the_light_and_the_pick() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                SKY_OBJECT_INITIALIZE,
                VECTOR_CONSTRUCT,
                NODE_SET_FLAG_004902F0,
                BILLBOARD_SET_MODE,
                NODE_SET_SCALE,
                LIGHT_SET_DIMMER,
                LIGHT_SET_COLOR_004BC280,
                LIGHT_SET_COLOR_004BC2E0,
                PICK_SET_00537E90,
                PICK_SET_00403550,
                PICK_SET_00632D20,
                PICK_SET_00458B30,
                SHADER_DATA_CONSTRUCT,
                0x0000_7770,
            ],
        );
        settings(
            &mut e,
            &[(SUN_BASE_SIZE_SETTING, 2.0), (SUN_GLARE_SIZE_SETTING, 3.0)],
        );
        e.map(MINUS_ONE, 8);
        e.set_global(MINUS_ONE, -1.0f64);
        e.map(NORMAL_011F426C, 12);
        e.mem.set_f32(NORMAL_011F426C + 8, 1.0);
        e.register(OPERATOR_NEW, |e, a| answer(e.mem.alloc(a[0])));
        e.register(NI_ALLOC_00AA1070, |e, a| answer(e.mem.alloc(a[0])));
        for constructor in [POINT3_CONSTRUCT, POINT2_CONSTRUCT, COLOR_A_CONSTRUCT] {
            e.register(constructor, |e, a| {
                for (word, value) in a[1..].iter().enumerate() {
                    e.mem.set_u32(a[0] + 4 * word as u32, *value);
                }
                answer(a[0])
            });
        }
        // Every allocated node and shape attaches its children through 0x7770.
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            let vtable = e.mem.alloc(0xe0);
            e.mem.set_u32(vtable + 0xdc, 0x0000_7770);
            e.mem.set_u32(block, vtable);
            answer(block)
        });
        for constructor in [TRI_SHAPE_CONSTRUCT, BILLBOARD_CONSTRUCT, LIGHT_CONSTRUCT] {
            e.register(constructor, |_, a| answer(a[0]));
        }
        e.register(PICK_CONSTRUCT, |_, a| answer(a[0]));
        let seen: std::rc::Rc<std::cell::RefCell<Vec<(u32, u32)>>> = Default::default();
        let record = seen.clone();
        e.register_double(SHADER_DATA_CONVERT, move |e, a| {
            record
                .borrow_mut()
                .push((e.mem.u32(a[0]), e.mem.u32(a[0] + 0x90)));
            Ret::default()
        });
        let root = object_with(&mut e, 4, &[(0xdc, 0x0000_7770)]);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 4, root);
        e.mem.set_f32(this + 0x20, 0.75);
        e.call_log = Some(vec![]);
        e.call(0x0064_0810, &args![this, 0x55u32]);
        assert_eq!(calls(&e, SKY_OBJECT_INITIALIZE), vec![vec![this, 0x55]]);
        let shapes = calls(&e, TRI_SHAPE_CONSTRUCT);
        assert_eq!(shapes.len(), 2);
        // The base quad: corners of size 2, normals, colour (1, 0, 0, 1), coordinates.
        let base_shape = &shapes[0];
        assert_eq!(base_shape[1..2], [4]);
        assert_eq!(base_shape[6..9], [1, 0, 2]);
        let points = base_shape[2];
        let corners: Vec<f32> = (0..12).map(|i| e.mem.f32(points + 4 * i)).collect();
        assert_eq!(
            corners,
            vec![-2.0, 2.0, 0.0, -2.0, -2.0, 0.0, 2.0, 2.0, 0.0, 2.0, -2.0, 0.0]
        );
        let glare_points = shapes[1][2];
        assert_eq!(e.mem.f32(glare_points), -3.0);
        assert_eq!(e.mem.f32(base_shape[3] + 0x24 + 8), 1.0);
        let colors = base_shape[4];
        assert_eq!(
            (0..4)
                .map(|i| e.mem.f32(colors + 0x30 + 4 * i))
                .collect::<Vec<_>>(),
            vec![1.0, 0.0, 0.0, 1.0]
        );
        let coordinates = base_shape[5];
        assert_eq!(
            (0..8)
                .map(|i| e.mem.f32(coordinates + 4 * i))
                .collect::<Vec<_>>(),
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]
        );
        let triangles = base_shape[9];
        assert_eq!(
            (0..6)
                .map(|i| e.mem.u16(triangles + 2 * i))
                .collect::<Vec<_>>(),
            vec![0, 1, 2, 2, 1, 3]
        );
        assert_eq!(e.mem.u32(this + 0x10), base_shape[0]);
        assert_eq!(e.mem.u32(this + 0x14), shapes[1][0]);
        let base_node = e.mem.u32(this + 8);
        let glare_node = e.mem.u32(this + 0xc);
        assert_eq!(
            calls(&e, BILLBOARD_SET_MODE),
            vec![vec![base_node, 2], vec![glare_node, 2]]
        );
        assert_eq!(
            calls(&e, NODE_SET_SCALE),
            vec![vec![glare_node, 0.75f32.to_bits()]]
        );
        let light = e.mem.u32(this + 0x1c);
        assert_eq!(
            calls(&e, 0x0000_7770),
            vec![
                vec![base_node, base_shape[0], 1],
                vec![glare_node, shapes[1][0], 1],
                vec![root, base_node, 1],
                vec![root, light, 1]
            ]
        );
        assert_eq!(
            calls(&e, LIGHT_SET_DIMMER),
            vec![vec![light, 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, LIGHT_SET_COLOR_004BC280),
            vec![vec![light, COLOR_011A9B7C]]
        );
        let pick = e.mem.u32(this + 0x18);
        assert_eq!(calls(&e, PICK_CONSTRUCT), vec![vec![pick, 0, 8]]);
        assert_eq!(calls(&e, PICK_SET_00458B30), vec![vec![pick, 1]]);
        assert_eq!(*seen.borrow(), vec![(6, 0), (6, 1)]);
        let converts = calls(&e, SHADER_DATA_CONVERT);
        assert_eq!(converts[0][1..], [base_shape[0], 1]);
        assert_eq!(converts[1][1..], [shapes[1][0], 1]);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 5);
    }

    #[test]
    fn small_accessors() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[TIMER_SCOPE_CONSTRUCT, TIMER_SCOPE_DESTROY]);
        let sky = e.mem.alloc(0x138);
        e.mem.set_u32(sky + 0x118, 4);
        assert_eq!(e.call(0x0064_2680, &args![sky]).u32(), 1);
        e.mem.set_u32(sky + 0x118, 3);
        assert_eq!(e.call(0x0064_2680, &args![sky]).u32(), 0);
        let this = e.mem.alloc(0x2c);
        assert_eq!(e.call(0x0064_27c0, &args![this]).f32(), 0.0);
        e.mem.set_u32(this + 0x28, 0x0000_2800);
        e.register(ACCUMULATOR_FLOAT, |_, _| answer_float(0.375));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_27c0, &args![this]).f32(), 0.375);
        let timer = calls(&e, TIMER_SCOPE_CONSTRUCT);
        assert_eq!(timer[0][1..], [0x21, 1, SUN_SOURCE_FILE_NAME, 0x235]);
        assert_eq!(calls(&e, TIMER_SCOPE_DESTROY).len(), 1);
    }

    struct Update {
        e: Engine,
        this: u32,
        sky: u32,
        base_property: u32,
        glare_property: u32,
        glare_node: u32,
        base_node: u32,
        light: u32,
    }

    fn update_rig(mode: u32, hour: f32) -> Update {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                TIMER_SCOPE_CONSTRUCT,
                TIMER_SCOPE_DESTROY,
                BASE_UPDATE_008D0600,
                SET_HIDDEN,
                LIGHT_SET_ROTATE,
                LIGHT_SET_COLOR_004BC280,
                LIGHT_SET_COLOR_004BC2E0,
                LIGHT_SET_COLOR_0050DD50,
                NODE_SET_TRANSLATE,
                NODE_SET_SCALE,
                POINT_UNITIZE,
                MATRIX_SET_COLUMN,
                MATRIX_SET_ELEMENT,
                CLEAR_OCCLUSION_TESTS,
                HOLDER_VALUE_0070EC90,
            ],
        );
        let sky = e.mem.alloc(0x138);
        e.mem.set_u32(sky + 0xf8, mode);
        e.mem.set_f32(sky + 0xec, hour);
        e.mem.set_f32(sky + 0xf4, 0.25);
        for word in 0..3u32 {
            e.mem
                .set_f32(sky + 0x78 + 4 * word, 0.1 * (word + 1) as f32);
        }
        e.register(SKY_GAME_HOUR, |e, a| answer_float(e.mem.f32(a[0] + 0xec)));
        e.register(SKY_GET_MODE, |e, a| answer(e.mem.u32(a[0] + 0xf8)));
        e.register(SKY_WEATHER_PERCENT, |e, a| {
            answer_float(e.mem.f32(a[0] + 0xf4))
        });
        for (address, value) in [
            (SKY_SUNRISE_BEGIN, 5.0f32),
            (SKY_SUNRISE_END, 7.0),
            (SKY_SUNSET_BEGIN, 18.0),
            (SKY_SUNSET_END, 20.0),
        ] {
            e.register_double(address, move |_, _| answer_float(value));
        }
        for (address, offset) in [
            (SKY_COLOR_60, 0x60u32),
            (SKY_COLOR_6C, 0x6c),
            (SKY_COLOR_78, 0x78),
            (SKY_SUN_SPECULAR, 0xc0),
            (SKY_CURRENT_WEATHER, 0x10),
            (SKY_LAST_WEATHER, 0x14),
        ] {
            e.register_double(address, move |e, a| {
                if offset >= 0x60 {
                    answer(a[0] + offset)
                } else {
                    answer(e.mem.u32(a[0] + offset))
                }
            });
        }
        for (address, value) in [
            (TWO, 2.0f64),
            (HALF, 0.5),
            (HOURS_PER_DAY, 24.0),
            (ONE, 1.0),
        ] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        e.map(IDENTITY_MATRIX, 36);
        e.map(NORMAL_011F426C, 12);
        e.map(TES_011DEA10, 4);
        e.map(OCCLUSION_EXPONENT, 4);
        e.set_global(OCCLUSION_EXPONENT, 2.0f32);
        settings(
            &mut e,
            &[
                (TRANSITION_SETTING, 2.0),
                (HEIGHT_SETTING, 100.0),
                (DISTANCE_SETTING, 50.0),
                (LIGHT_HEIGHT_SETTING, 30.0),
                (GLARE_STEP_SETTING, 0.5),
                (GLARE_LIMIT_SETTING, 0.1),
            ],
        );
        // The weathers' fade answers its amount; the float function doubles its argument.
        e.register(WEATHER_FADE, |_, a| answer_float(f32::from_bits(a[2])));
        e.register(FLOAT_FUNCTION_00408860, |_, a| {
            answer_float(2.0 * f32::from_bits(a[0]))
        });
        e.register(NODE_IS_VISIBLE, |_, _| answer(1));
        e.register(GET_PROPERTY, |e, a| answer(e.mem.u32(a[0] + 0xa8)));
        e.register(PROPERTY_TYPE, |e, a| answer(e.mem.u32(a[0] + 0x1c)));
        e.register(POINT3_CONSTRUCT, |e, a| {
            for (word, value) in a[1..].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * word as u32, *value);
            }
            answer(a[0])
        });
        e.register(COLOR_A_CONSTRUCT, |e, a| {
            for (word, value) in a[1..].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * word as u32, *value);
            }
            answer(a[0])
        });
        e.register(SUN_BASE_NODE, |e, a| answer(e.mem.u32(a[0] + 8)));
        let this = e.mem.alloc(0x2c);
        let mut properties = vec![];
        for slot in [0x10u32, 0x14] {
            let property = e.mem.alloc(0x90);
            e.mem.set_u32(property + 0x1c, 0xd);
            let shape = e.mem.alloc(0xc0);
            e.mem.set_u32(shape + 0xa8, property);
            e.mem.set_u32(this + slot, shape);
            properties.push(property);
        }
        let root = e.mem.alloc(0xb0);
        let base_node = e.mem.alloc(0xb4);
        let glare_node = e.mem.alloc(0xb4);
        let light = e.mem.alloc(0xfc);
        e.mem.set_u32(this + 4, root);
        e.mem.set_u32(this + 8, base_node);
        e.mem.set_u32(this + 0xc, glare_node);
        e.mem.set_u32(this + 0x1c, light);
        let current = e.mem.alloc(0x370);
        e.mem.set_u32(sky + 0x10, current);
        Update {
            e,
            this,
            sky,
            base_property: properties[0],
            glare_property: properties[1],
            glare_node,
            base_node,
            light,
        }
    }

    fn run(u: &mut Update) {
        u.e.call_log = Some(vec![]);
        u.e.call(0x0064_1830, &args![u.this, u.sky, 0.5f32]);
    }

    #[test]
    fn update_at_noon_is_bright_and_overhead() {
        let mut u = update_rig(3, 12.5);
        run(&mut u);
        let e = &u.e;
        // Brightness 1, the glare's alpha the current weather's 0.25 * 1.
        assert_eq!(e.mem.f32(u.base_property + 0x6c), 1.0);
        assert_eq!(e.mem.f32(u.base_property + 0x60), 0.1);
        assert_eq!(e.mem.f32(u.glare_property + 0x6c), 0.25);
        // Height 0: the root moves to (0, 50, 2 * 100 - 2 * 0).
        let translate = calls(e, NODE_SET_TRANSLATE);
        assert_eq!(translate.len(), 2);
        let point = translate[0][1];
        assert_eq!(
            (0..3).map(|i| e.mem.f32(point + 4 * i)).collect::<Vec<_>>(),
            vec![0.0, 50.0, 200.0]
        );
        assert_eq!(translate[1][0], u.glare_node);
        // The light: (-0, 50, 30), as a column of the identity.
        let points = calls(e, POINT3_CONSTRUCT);
        assert_eq!(
            points[0][1..],
            [0x8000_0000, 50.0f32.to_bits(), 30.0f32.to_bits()]
        );
        assert_eq!(calls(e, LIGHT_SET_ROTATE)[0][0], u.light);
        assert_eq!(
            calls(e, LIGHT_SET_COLOR_0050DD50),
            vec![vec![u.light, u.sky + 0xc0]]
        );
        // No holder: no glare change; the base node shows (flag 4 clear).
        assert!(calls(e, NODE_SET_SCALE).is_empty());
        assert!(calls(e, SET_HIDDEN).contains(&vec![u.base_node, 0]));
    }

    #[test]
    fn update_fades_over_the_sunrise_and_is_dark_at_night() {
        let mut u = update_rig(2, 6.0);
        run(&mut u);
        assert_eq!(u.e.mem.f32(u.base_property + 0x6c), 0.5);
        let mut u = update_rig(3, 19.5);
        run(&mut u);
        assert_eq!(u.e.mem.f32(u.base_property + 0x6c), 0.25);
        for hour in [3.0f32, 22.0] {
            let mut u = update_rig(3, hour);
            run(&mut u);
            assert_eq!(u.e.mem.f32(u.base_property + 0x6c), 0.0);
            let point = calls(&u.e, NODE_SET_TRANSLATE)[0][1];
            let fraction: f32 = if hour < 5.0 { 7.0 / 9.0 } else { 2.0 / 9.0 };
            let expected = (100.0 * (2.0 * fraction as f64 - 1.0)) as f32;
            assert_eq!(u.e.mem.f32(point), expected);
        }
    }

    #[test]
    fn update_inside_orients_the_light_only() {
        let mut u = update_rig(0, 12.0);
        u.e.map(INTERIOR_ANGLE, 4);
        u.e.set_global(INTERIOR_ANGLE, -0.5f32);
        run(&mut u);
        let e = &u.e;
        let set = calls(e, MATRIX_SET_ELEMENT);
        assert_eq!(set.len(), 2);
        assert_eq!(set[0][1..], [1, 0, (-0.5f32).to_bits()]);
        assert_eq!(set[1][1..], [2, 0, (-0.5f32).to_bits()]);
        assert_eq!(calls(e, LIGHT_SET_ROTATE).len(), 1);
        assert_eq!(
            calls(e, LIGHT_SET_COLOR_004BC280),
            vec![vec![u.light, u.sky + 0x60]]
        );
        assert!(calls(e, NODE_SET_TRANSLATE).is_empty());

        // Mode 1 in a cell: the cell's angles in radians.
        let mut u = update_rig(1, 12.0);
        let holder = u.e.mem.alloc(0x40);
        u.e.set_global(TES_011DEA10, holder);
        u.e.register(HOLDER_CELL_005F36F0, |_, _| answer(0x0000_0c00));
        u.e.register(CELL_ANGLE_00544910, |_, _| answer(90));
        u.e.register(CELL_ANGLE_00544970, |_, _| answer(180));
        u.e.map(DEGREES_TO_RADIANS, 8);
        u.e.set_global(DEGREES_TO_RADIANS, 0.5f64);
        quiet(
            &mut u.e,
            &[
                ELEMENT_CONSTRUCT_006815C0,
                MATRIX_ROTATION_004A0C90,
                MATRIX_MAKE_Y_ROTATION,
            ],
        );
        u.e.register(MATRIX_MULTIPLY, |_, a| answer(a[1]));
        run(&mut u);
        let e = &u.e;
        assert_eq!(calls(e, MATRIX_ROTATION_004A0C90)[0][1], 45.0f32.to_bits());
        assert_eq!(calls(e, MATRIX_MAKE_Y_ROTATION)[0][1], 90.0f32.to_bits());
        assert!(calls(e, MATRIX_SET_ELEMENT).is_empty());
    }

    fn glare_rig(accumulator: f32, scale: f32) -> Update {
        let mut u = update_rig(3, 12.5);
        let holder = u.e.mem.alloc(0x70);
        u.e.mem.set_u32(holder + 0xc, 1);
        u.e.set_global(TES_011DEA10, holder);
        u.e.register(HOLDER_VALUE_0084E3A0, |e, a| answer(e.mem.u32(a[0] + 0xc)));
        u.e.register(CAMERA_HOLDER_GET, |_, _| answer(0x0000_0c00));
        u.e.register(CAMERA_NODE, |_, _| answer(0x0000_0c04));
        let point = u.e.mem.alloc(12);
        u.e.register_double(NODE_POINT_0043C490, move |_, _| answer(point));
        u.e.register(BYTE_GETTER_004E2180, |_, _| answer(0));
        u.e.register(OCCLUSION_CURVE, |_, _| answer_float(0.8));
        u.e.mem.set_u32(u.this + 0x28, 0x0000_2800);
        u.e.register_double(ACCUMULATOR_FLOAT, move |_, _| answer_float(accumulator));
        u.e.mem.set_f32(u.this + 0x20, scale);
        u
    }

    #[test]
    fn glare_rises_towards_the_occlusion() {
        let mut u = glare_rig(0.5, 0.25);
        run(&mut u);
        let e = &u.e;
        // Step 0.5 * 0.5; 0.25 + 0.25 is still below 0.8.
        assert_eq!(e.mem.f32(u.this + 0x20), 0.5);
        assert_eq!(
            calls(e, NODE_SET_SCALE),
            vec![vec![u.glare_node, 0.5f32.to_bits()]]
        );
        assert!(calls(e, SET_HIDDEN).contains(&vec![u.glare_node, 0]));
        let curve = calls(e, OCCLUSION_CURVE);
        assert_eq!(curve[0], vec![0.5f32.to_bits(), 2.0f32.to_bits()]);
        assert_eq!(calls(e, CLEAR_OCCLUSION_TESTS), vec![vec![u.this]]);
        // Close enough: it takes the occlusion.
        let mut u = glare_rig(0.5, 0.7);
        run(&mut u);
        assert_eq!(u.e.mem.f32(u.this + 0x20), 0.8);
    }

    #[test]
    fn glare_drops_when_forced_and_hides_below_the_limit() {
        let mut u = glare_rig(1.0, 0.25);
        run(&mut u);
        let e = &u.e;
        assert_eq!(e.mem.f32(u.this + 0x20), 0.0);
        assert!(calls(e, SET_HIDDEN).contains(&vec![u.glare_node, 1]));
        assert!(calls(e, NODE_SET_SCALE).is_empty());
        // Above the limit it only drops by the step.
        let mut u = glare_rig(1.0, 1.0);
        run(&mut u);
        assert_eq!(u.e.mem.f32(u.this + 0x20), 0.75);
        assert_eq!(
            calls(&u.e, NODE_SET_SCALE),
            vec![vec![u.glare_node, 0.75f32.to_bits()]]
        );
        // The base node hides while the sky's flag 4 is set.
        let mut u = glare_rig(1.0, 1.0);
        u.e.mem.set_u32(u.sky + 0x118, 4);
        run(&mut u);
        assert!(calls(&u.e, SET_HIDDEN).contains(&vec![u.base_node, 1]));
    }
}
