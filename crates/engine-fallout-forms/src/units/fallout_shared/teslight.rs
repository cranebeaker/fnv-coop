//! `fallout shared/teslight.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The bit of the flags dword at `+8` the function tests.
const FLAG_BIT: u32 = 0x10;

// Translated from 0050d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `teslight.cpp`). `__fastcall`: whether
/// bit `0x10` of the dword at `this + 8` is set.
pub fn fn_0050d4a0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & FLAG_BIT != 0
}

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// Sets (`true`) or clears form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// Tests form flag 0x8 (`004013e0`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The size of the current chunk (`00401660`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): one word.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// Whether the file's data needs its byte order swapped (`00401680`).
const SWAP_BYTES: u32 = 0x0040_1680;
/// `TESForm::LoadData` (`004861f0`, `RET 0xC`): file, buffer, size.
const LOAD_DATA: u32 = 0x0048_61f0;
/// The address of a `Setting`'s value (`00403e20`, the setting in `ECX`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// Whether a model has a file name (`0048cee0`).
const MODEL_HAS_NAME: u32 = 0x0048_cee0;
/// The model loader (a pointer at `011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The `NiPointer` constructor (`00633c90`, `RET 4`), its object
/// (`00559450`) and its release (`0045cec0`).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_OBJECT: u32 = 0x0055_9450;
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// The Gamebryo allocation (`00aa13e0`, cdecl).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// A node's constructor (`00a5ecb0`, `RET 4`; 0xAC bytes).
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The name string's constructor (`00438170`, `RET 4`), destructor
/// (`004381b0`) and an object's name setter (`00a5b950`, `RET 4`).
const NAME_CONSTRUCT: u32 = 0x0043_8170;
const NAME_DESTRUCT: u32 = 0x0043_81b0;
const SET_NAME: u32 = 0x00a5_b950;
/// The `double` 0.0 the float tests compare with (`01012060`).
const ZERO: u32 = 0x0101_2060;
/// The `float` default for `+0xB0` (`010249d8`).
const DEFAULT_B0: u32 = 0x0102_49d8;
/// The `double` the attenuation results fall back to for a zero factor
/// (`01016408`).
const ATTENUATION_FALLBACK: u32 = 0x0101_6408;
/// The `double` 255.0 the colour bytes are divided by (`0101e568`).
const COLOR_SCALE: u32 = 0x0101_e568;
/// The `bLightAttenuation` flags byte (`011c8fa0`): bit 0 constant, bit 1
/// linear, bit 2 quadratic.
const ATTENUATION_FLAGS: u32 = 0x011c_8fa0;
/// The `bLightAttenuation` settings (their addresses): `fConstantValue`,
/// `fLinearRadiusMult`, `uLinearMethod`, `fLinearValue`,
/// `fQuadraticRadiusMult`, `uQuadraticMethod`, `fQuadraticValue`,
/// `bOutQuadInLin`.
const CONSTANT_VALUE: u32 = 0x011c_9000;
const LINEAR_RADIUS_MULT: u32 = 0x011c_8fb0;
const LINEAR_METHOD: u32 = 0x011c_9054;
const LINEAR_VALUE: u32 = 0x011c_908c;
const QUADRATIC_RADIUS_MULT: u32 = 0x011c_906c;
const QUADRATIC_METHOD: u32 = 0x011c_907c;
const QUADRATIC_VALUE: u32 = 0x011c_9040;
const OUT_QUAD_IN_LIN: u32 = 0x011c_8fc0;
/// The three attenuation setters of the light (`0050c630`, `0050c660`,
/// `0050c690`, each a `float` and `RET 4`).
const SET_CONSTANT: u32 = 0x0050_c630;
const SET_LINEAR: u32 = 0x0050_c660;
const SET_QUADRATIC: u32 = 0x0050_c690;
/// The object `005f36f0` is asked about (a pointer at `011dea10`; the
/// engine map names `005f36f0` `ActorMover::GetPreferredMoveMode` through
/// folding, and docs/ENGINE_CRATE.md notes that body returns
/// `TES::pInteriorCell`).
const TES_011DEA10: u32 = 0x011d_ea10;

/// Chunk signatures, as the words the exe compares.
const SIG_DATA: u32 = 0x4154_4144;
const SIG_DEST: u32 = 0x5453_4544;
const SIG_DSTD: u32 = 0x4454_5344;
const SIG_EDID: u32 = 0x4449_4445;
const SIG_FNAM: u32 = 0x4d41_4e46;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_ICON: u32 = 0x4e4f_4349;
const SIG_MICO: u32 = 0x4f43_494d;
const SIG_MODD: u32 = 0x4444_4f4d;
const SIG_MODL: u32 = 0x4c44_4f4d;
const SIG_MODS: u32 = 0x5344_4f4d;
const SIG_MODT: u32 = 0x5444_4f4d;
const SIG_MOSD: u32 = 0x4453_4f4d;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_SCRI: u32 = 0x4952_4353;
const SIG_SNAM: u32 = 0x4d41_4e53;

/// `FLD dword`/`FSTP dword` of a value read from memory: its bits, a
/// signalling NaN quieted.
fn copy_float(e: &Engine, at: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(at))
}

/// The value of a `Setting` (`00403e20` gives its address) as `FLD dword`
/// loads it.
fn setting_float(e: &mut Engine, setting: u32) -> f64 {
    let at = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(at))
}

// Translated from 0050c350 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`GenDynamic` calls it with its light and radius), cdecl,
/// two words: sets the light attenuation of the object `00653270` finds
/// for `key` in the map at `011f4a98` (nothing without one). Constant:
/// `fConstantValue` when bit 0 of the `bLightAttenuation` flags byte
/// (`011c8fa0`) is set, else 0.0 (`0050c630`); linear and quadratic start
/// at 0.0 (`0050c660`, `0050c690`).
///
/// With flag bit 1, or else when `005f36f0` on the object at `011dea10`
/// answers non-zero and `bOutQuadInLin` (`0050e620` with 1) is set, the
/// linear factor `t = fLinearRadiusMult * radius` (stored as a `float`) by
/// `uLinearMethod` (`0043d4d0`): 0 gives `fLinearValue`; 2 gives
/// `fLinearValue / (t * t)`; any other `fLinearValue * (1 / t)`; a `t`
/// equal to 0.0 gives the `double` at `01016408` instead (`0050c660`).
///
/// With flag bit 2, or else when `005f36f0` answers zero and
/// `bOutQuadInLin` is set, the quadratic factor
/// `q = fQuadraticRadiusMult * radius` by `uQuadraticMethod`: 0 gives
/// `fQuadraticValue`; 1 gives `fQuadraticValue` (`00450410`) times
/// `1 / radius`; any other `fQuadraticValue / (q * q)`; 0.0 gives the
/// `01016408` value (`0050c690`). Every value goes through a `float`
/// store, each x87 operation in the exe's order.
pub fn fn_0050c350(e: &mut Engine, key: u32, radius: u32) {
    let light = e.call(0x0065_3270, &args![0x011f_4a98u32, key]).u32();
    if light == 0 {
        return;
    }
    let flags = e.global::<u8>(ATTENUATION_FLAGS);
    let constant = if flags & 1 != 0 {
        let at = e.call(SETTING_VALUE, &args![CONSTANT_VALUE]).u32();
        copy_float(e, at)
    } else {
        0
    };
    e.call(SET_CONSTANT, &args![light, constant]);
    e.call(SET_LINEAR, &args![light, 0u32]);
    e.call(SET_QUADRATIC, &args![light, 0u32]);
    let zero = e.global::<f64>(ZERO);
    let linear = if e.global::<u8>(ATTENUATION_FLAGS) & 2 != 0 {
        true
    } else {
        e.call(0x005f_36f0, &args![e.global::<u32>(TES_011DEA10)])
            .u32()
            != 0
            && e.call(0x0050_e620, &args![OUT_QUAD_IN_LIN, 1u32]).bool()
    };
    if linear {
        let distance = f64::from(radius);
        let mult = setting_float(e, LINEAR_RADIUS_MULT);
        let factor = x87_store32(x87_mul(mult, distance));
        let method_at = e.call(0x0043_d4d0, &args![LINEAR_METHOD]).u32();
        let method = e.mem.u32(method_at);
        let value = if method == 0 {
            let at = e.call(SETTING_VALUE, &args![LINEAR_VALUE]).u32();
            copy_float(e, at)
        } else {
            let t = x87_load32(factor);
            let result = if t == zero {
                x87_load64(e.global::<u64>(ATTENUATION_FALLBACK))
            } else if method == 2 {
                let at = e.call(SETTING_VALUE, &args![LINEAR_VALUE]).u32();
                let square = x87_mul(t, x87_m32(factor));
                x87_div(x87_m32(e.mem.u32(at)), square)
            } else {
                let inverse = x87_div(1.0, t);
                let value = setting_float(e, LINEAR_VALUE);
                x87_mul(value, inverse)
            };
            x87_store32(result)
        };
        e.call(SET_LINEAR, &args![light, value]);
    }

    let quadratic = if e.global::<u8>(ATTENUATION_FLAGS) & 4 != 0 {
        true
    } else {
        e.call(0x005f_36f0, &args![e.global::<u32>(TES_011DEA10)])
            .u32()
            == 0
            && e.call(0x0050_e620, &args![OUT_QUAD_IN_LIN, 1u32]).bool()
    };
    if quadratic {
        let distance = f64::from(radius);
        let mult = setting_float(e, QUADRATIC_RADIUS_MULT);
        let factor = x87_store32(x87_mul(mult, distance));
        let method_at = e.call(0x0043_d4d0, &args![QUADRATIC_METHOD]).u32();
        let method = e.mem.u32(method_at);
        let value = if method == 0 {
            let at = e.call(SETTING_VALUE, &args![QUADRATIC_VALUE]).u32();
            copy_float(e, at)
        } else {
            let q = x87_load32(factor);
            let result = if q == zero {
                x87_load64(e.global::<u64>(ATTENUATION_FALLBACK))
            } else if method == 1 {
                let inverse = x87_div(1.0, f64::from(radius));
                let value = e.call(0x0045_0410, &args![QUADRATIC_VALUE]).f64();
                x87_mul(value, inverse)
            } else {
                let value = e.call(0x0045_0410, &args![QUADRATIC_VALUE]).f64();
                let square = x87_mul(q, x87_m32(factor));
                x87_div(value, square)
            };
            x87_store32(result)
        };
        e.call(SET_QUADRATIC, &args![light, value]);
    }
}

// Translated from 0050ca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLIGH::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), the components' `InitItem` with the light (`0048cd90` on
/// `+0x78`, `00478be0` on `+0x94`, `TESModelTextureSwap::InitItem`
/// `0048aa80` on `+0x3C`); a sound id at `+0xB8` gets the compile index of
/// the light's file (`00485d50` with `TESForm::GetFile(-1)`), is looked up
/// (`004839c0`) and cast to `011832fc` (`__RTDynamicCast`); a missing one is
/// reported ("MASTERFILE: Unable to find sound (%08X) on object '%s'.",
/// with the id and vtable slot 0x130's answer). Finally form flag 0x8 is
/// set.
pub fn tes_object_ligh_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.call(0x0048_cd90, &args![base + 0x78, this]);
    e.call(0x0047_8be0, &args![base + 0x94, this]);
    e.call(0x0048_aa80, &args![base + 0x3c, this]);
    if e.mem.u32(base + 0xb8) != 0 {
        e.with_stack(4, |e, id_at| {
            let id = e.mem.u32(base + 0xb8);
            e.mem.set_u32(id_at.addr(), id);
            let file = e.call(0x0048_4e60, &args![this, u32::MAX]).u32();
            e.call(0x0048_5d50, &args![id_at, file]);
            let id = e.mem.u32(id_at.addr());
            let form = e.call(0x0048_39c0, &args![id]).u32();
            let sound = e
                .call(
                    0x00ec_43fb,
                    &args![form, 0u32, 0x0118_3028u32, 0x0118_32fcu32, 0u32],
                )
                .u32();
            e.mem.set_u32(base + 0xb8, sound);
            if e.mem.u32(base + 0xb8) == 0 {
                let name = e.vcall(base, 0x130, &args![]).u32();
                let id = e.mem.u32(id_at.addr());
                e.call(0x005b_5e40, &args![0x0102_9064u32, id, name]);
            }
        });
    }
    e.call(SET_FORM_FLAG_8, &args![this, true]);
}

// Translated from 0050caf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050caf0 - the /GS cookie check (00ec408c) is not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// `TESObjectLIGH`'s `Load` (`RET 4`): false unless the record is of form
/// type 0x1E. Then `TESForm::LoadForm` (Xbox PDB, `00485110`), form flag
/// 0x8 cleared, the `float`s at `+0xB0` and `+0xAC` set to 0.0, and every
/// chunk of `file` (a component address is null when `this` is, as in the
/// exe): `EDID` (vtable slot 0x134, at most 0x200 bytes), `OBND` (slot
/// 0xE0), `FULL` (`00487050` on `+0x30`), `MODL`/`MODD`/`MOSD`/`MODT`
/// (`TESModel::LoadModelChunk`, `004892d0`, on `+0x3C`), `MODS` (read whole
/// into a stack buffer, unpacked by `0048a7b0` on `+0x3C` with its size
/// and the form version `00403570`), `ICON` (`0048e3d0` on `+0x5C`),
/// `MICO` (`0047a290` on `+0x68`), `DSTD`/`DEST` (`004781e0` on `+0x94`),
/// `SCRI` (a cleared word given to `006ecd40`, then `0048cd90` with the
/// light, on `+0x78`), `DATA` (0x18 bytes into `+0x9C` by
/// `TESForm::LoadData`, `004861f0`), `FNAM` (a word straight into
/// `+0xB4`), `SNAM` (a cleared word stored at `+0xB8`). After a `DATA`
/// chunk the data is swapped (`00693dc0` on `+0x9C`) when the file says so
/// (`00401680`), a `+0xB0` equal to 0.0 becomes the `float` at `010249d8`
/// and a `+0xAC` equal to 0.0 becomes 1.0. Returns true.
pub fn fn_0050caf0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let component = |offset: u32| if base != 0 { base + offset } else { 0 };
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x1e {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, false]);
    e.mem.set_u32(base + 0xb0, 0.0f32.to_bits());
    e.mem.set_u32(base + 0xac, 0.0f32.to_bits());
    e.with_stack(4, |e, word_at| loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_FULL => {
                e.call(0x0048_7050, &args![component(0x30), file]);
            }
            SIG_MOSD | SIG_MODD | SIG_MODL | SIG_MODT => {
                e.call(0x0048_92d0, &args![component(0x3c), file]);
            }
            SIG_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            SIG_DATA => {
                e.call(LOAD_DATA, &args![this, file, base + 0x9c, 0x18u32]);
            }
            SIG_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SIG_DSTD | SIG_DEST => {
                e.call(0x0047_81e0, &args![component(0x94), file]);
            }
            SIG_SCRI => {
                e.mem.set_u32(word_at.addr(), 0);
                e.call(GET_CHUNK_DATA, &args![file, word_at]);
                let script = e.mem.u32(word_at.addr());
                e.call(0x006e_cd40, &args![base + 0x78, script]);
                e.call(0x0048_cd90, &args![base + 0x78, this]);
            }
            SIG_MICO => {
                e.call(0x0047_a290, &args![component(0x68), file]);
            }
            SIG_FNAM => {
                e.call(GET_CHUNK_DATA, &args![file, base + 0xb4]);
            }
            SIG_SNAM => {
                e.mem.set_u32(word_at.addr(), 0);
                e.call(GET_CHUNK_DATA, &args![file, word_at]);
                let sound = e.mem.u32(word_at.addr());
                e.mem.set_u32(base + 0xb8, sound);
            }
            SIG_ICON => {
                e.call(0x0048_e3d0, &args![component(0x5c), file]);
            }
            SIG_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                    let version = e.call(0x0040_3570, &args![file]).u16();
                    e.call(
                        0x0048_a7b0,
                        &args![base + 0x3c, buffer, size, u32::from(version)],
                    );
                });
            }
            _ => {}
        }
        if chunk == SIG_DATA {
            if e.call(SWAP_BYTES, &args![file]).bool() {
                e.call(0x0069_3dc0, &args![base + 0x9c]);
            }
            let zero = e.global::<f64>(ZERO);
            if x87_load32(e.mem.u32(base + 0xb0)) == zero {
                let value = copy_float(e, DEFAULT_B0);
                e.mem.set_u32(base + 0xb0, value);
            }
            if x87_load32(e.mem.u32(base + 0xac)) == zero {
                e.mem.set_u32(base + 0xac, 1.0f32.to_bits());
            }
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    });
    true
}

// Translated from 0050d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the light's data (`this + 0x9C`).
pub fn fn_0050d100(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x9c)
}

// Translated from 0050d1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x8 of the light's flags word (`+0xA8`) is set.
pub fn fn_0050d1d0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0xa8) & 0x8 != 0
}

// Translated from 0050d1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x40 of the light's flags word (`+0xA8`) is set.
pub fn fn_0050d1f0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0xa8) & 0x40 != 0
}

// Translated from 0050d210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x80 of the light's flags word (`+0xA8`) is set.
pub fn fn_0050d210(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0xa8) & 0x80 != 0
}

// Translated from 0050d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x100 of the light's flags word (`+0xA8`) is set.
pub fn fn_0050d230(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0xa8) & 0x100 != 0
}

// Translated from 0050d250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050d250 - the exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated; the __alloca_probe_16 (00ec5ec0) buffer is a with_stack block (it is not used afterwards)
/// `TESObjectLIGH::Clone3D` (Xbox PDB), `reference` the light's reference
/// (`RET 4`): the model vtable slot 0x174 gives for it, held in a stack
/// `NiPointer` (`00633c90`; an empty one is made first); none gives null.
/// Otherwise a stack buffer of `00474cb0`'s answer + 0x10 bytes is taken;
/// without a model file name (`0048cee0` on `+0x3C`) the model's vtable
/// slot 0x10 gives a node, which (when not null) is named after the light
/// (slot 0x130, through a stack name `00438170`/`004381b0` and
/// `00a5b950`). A light with flag bit 0x10 (`0050d4a0`) and a reference
/// runs `00693ef0` with 1 on the reference. A cloning process (`004ad050`)
/// with the reference's scale (`TESObjectREFR::GetScale`, `00567400`, 1.0
/// without one) then clones the model (`NiObject::Clone`, `00a5d2c0`) when
/// it has a file name; otherwise, without a node yet, a new `BSFadeNode`
/// (0xE4 bytes, `00aa13e0`, `00b4e150`) is the node. `GenDynamic`
/// (`0050d810` with 0) adds the light to it, `NiAVObject::UpdateProperties`
/// (`00a5a040`) runs, the cloning process (`004ad270`) and both pointers
/// (`0045cec0`) are released, and the node is returned.
pub fn tes_object_ligh_clone_3d(e: &mut Engine, this: Ptr, reference: u32) -> u32 {
    let base = this.addr();
    e.with_stack(0x30, |e, frame| {
        // `[EBP-0x10]`, `[EBP-0x14]`, the name at `[EBP-0x48]` and the
        // cloning process at `[EBP-0x40]`.
        let empty_at = frame;
        let model_at = frame.byte_add(4);
        let name_at = frame.byte_add(8);
        let process_at = frame.byte_add(0x10);
        e.call(POINTER_CONSTRUCT, &args![empty_at, 0u32]);
        let model = e.vcall(base, 0x174, &args![reference]).u32();
        e.call(POINTER_CONSTRUCT, &args![model_at, model]);
        if e.call(POINTER_OBJECT, &args![model_at]).u32() == 0 {
            e.call(POINTER_RELEASE, &args![model_at]);
            e.call(POINTER_RELEASE, &args![empty_at]);
            return 0;
        }
        let size = e.call(0x0047_4cb0, &args![this]).u32().wrapping_add(0x10);
        e.with_stack(size, |e, _buffer| {
            let mut node = 0u32;
            if e.call(MODEL_HAS_NAME, &args![base + 0x3c]).u32() == 0 {
                let object = e.call(POINTER_OBJECT, &args![model_at]).u32();
                node = e.vcall(object, 0x10, &args![]).u32();
                if node != 0 {
                    let light_name = e.vcall(base, 0x130, &args![]).u32();
                    let name = e.call(NAME_CONSTRUCT, &args![name_at, light_name]).u32();
                    e.call(SET_NAME, &args![node, name]);
                    e.call(NAME_DESTRUCT, &args![name_at]);
                }
            }
            if fn_0050d4a0(e, this) && reference != 0 {
                e.call(0x0069_3ef0, &args![reference, 1u32]);
            }
            let mut scale = 1.0f32.to_bits();
            if reference != 0 {
                scale = x87_store32(e.call(0x0056_7400, &args![reference]).f64());
            }
            e.call(0x004a_d050, &args![process_at, x87_quiet_bits(scale)]);
            if e.call(MODEL_HAS_NAME, &args![base + 0x3c]).u32() != 0 {
                let object = e.call(POINTER_OBJECT, &args![model_at]).u32();
                node = e.call(0x00a5_d2c0, &args![object, process_at]).u32();
            } else if node == 0 {
                let block = e.call(NI_ALLOCATE, &args![0xe4u32]).u32();
                node = if block != 0 {
                    e.call(0x00b4_e150, &args![block]).u32()
                } else {
                    0
                };
            }
            tes_object_ligh_gen_dynamic(e, this, reference, node, false);
            e.call(0x00a5_a040, &args![node]);
            e.call(0x004a_d270, &args![process_at]);
            e.call(POINTER_RELEASE, &args![model_at]);
            e.call(POINTER_RELEASE, &args![empty_at]);
            node
        })
    })
}

// Translated from 0050d4c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050d4c0 - the exception-unwinding frame (FS:[0]) is not translated.
/// No map name, `RET 4`: with a model name (vtable slot 0x14 of `+0x3C`)
/// that is not empty (`0044a670`), the object `0050fe50` gives for
/// `argument`; with an empty one a new node (0xAC bytes, `00aa13e0`,
/// `00a5ecb0` with 0; null when the allocation fails). An object that is
/// not a node (its vtable slot 0xC answers 0) is put under a new node
/// (vtable slot 0xDC of the new node with the object and 1), which is
/// returned instead.
pub fn fn_0050d4c0(e: &mut Engine, this: Ptr, argument: u32) -> u32 {
    let base = this.addr();
    let new_node = |e: &mut Engine| {
        let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
        if block != 0 {
            e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
        } else {
            0
        }
    };
    let name = e.vcall(base + 0x3c, 0x14, &args![]).u32();
    let mut object = if e.call(0x0044_a670, &args![name]).u32() != 0 {
        e.call(0x0050_fe50, &args![this, argument]).u32()
    } else {
        new_node(e)
    };
    if object != 0 && e.vcall(object, 0xc, &args![]).u32() == 0 {
        let node = new_node(e);
        e.vcall(node, 0xdc, &args![object, 1u32]);
        object = node;
    }
    object
}

// Translated from 0050d600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `RET 4` (the stack word is not read): with a model file
/// name (`0048cee0` on `+0x3C`), hands the model's name (its vtable slot
/// 0x14) to `0045a5e0` on the model loader at `011c3b3c`.
pub fn fn_0050d600(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let base = this.addr();
    if e.call(MODEL_HAS_NAME, &args![base + 0x3c]).u32() != 0 {
        let name = e.vcall(base + 0x3c, 0x14, &args![]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(0x0045_a5e0, &args![loader, name]);
    }
}

// Translated from 0050d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050d6d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// No map name, cdecl, one word: searches `node`'s children (count
/// `0043b480`, child `i` `0043b4a0`, each held in a stack `NiPointer`,
/// `00633c90`, released by `0045cec0`) depth first for the first one
/// `0043b300` accepts for the type at `011f4a28`: a child that is a node
/// (its vtable slot 0xC answers non-zero) is searched in turn; null when
/// none is found or `node` is null.
pub fn fn_0050d6d0(e: &mut Engine, node: u32) -> u32 {
    if node == 0 {
        return 0;
    }
    let mut found = 0u32;
    let mut index = 0u32;
    while index < e.call(0x0043_b480, &args![node]).u32() {
        let child = e.call(0x0043_b4a0, &args![node, index]).u32();
        let result = e.with_stack(4, |e, pointer| {
            e.call(POINTER_CONSTRUCT, &args![pointer, child]);
            let mut result = None;
            if e.call(POINTER_OBJECT, &args![pointer]).u32() != 0 {
                let object = e.call(POINTER_OBJECT, &args![pointer]).u32();
                if e.vcall(object, 0xc, &args![]).u32() != 0 {
                    let object = e.call(POINTER_OBJECT, &args![pointer]).u32();
                    found = fn_0050d6d0(e, object);
                    if found != 0 {
                        result = Some(found);
                    }
                } else {
                    let object = e.call(POINTER_OBJECT, &args![pointer]).u32();
                    if e.call(0x0043_b300, &args![0x011f_4a28u32, object]).bool() {
                        result = Some(e.call(POINTER_OBJECT, &args![pointer]).u32());
                    }
                }
            }
            e.call(POINTER_RELEASE, &args![pointer]);
            result
        });
        if let Some(result) = result {
            return result;
        }
        index += 1;
    }
    found
}

// Translated from 0050d810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050d810 - the exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// `TESObjectLIGH::GenDynamic` (Xbox PDB), `RET 0xC`: adds the light to
/// `node` for `reference` and returns the light object (null without a
/// node).
///
/// With a reference: its base form (`007af430`); for a light without flag
/// 0x20 (`+0xA8`), only when `008d6f30` answers non-zero and the height at
/// `+8` of vtable slot 0x1F4's answer is below the reference's water
/// height (`TESObjectREFR::GetRelevantWaterHeight`, `0057b0a0`; `FCOMPP`,
/// so not for NaN), and always with the flag: a base form of type 0x1E
/// with addon nodes (`TESObjectREFR::HasAddonNodes`, `00578400`) gets
/// `TESForm::SetFireOff` (`00484a70` with 1) and
/// `TESObjectLIGH::StopParticlesRecursive` (`0050d640` on the node); then
/// a reference whose vtable slot 0x100 answers false, for a light with
/// flag 0x20, returns null.
///
/// The colour (`00416870` on a stack colour with three 0.0) is the three
/// low bytes of `+0xA4` over 255.0 (`0101e568`), or `004a0bd0`'s answer
/// with flag 4. The parent is vtable slot 0xC of what the node's slot 0x9C
/// gives for `0050de10`'s word, or the node; the light is the first one
/// under it (`0050d6d0`), else a new one (0xFC bytes, `00aa13e0`,
/// `00a7d670`) named "<name> PtLight" (`00406d00`, `00438170`, `00a5b950`)
/// attached to the parent (its slot 0xDC with 1). A reference gets it
/// (`00572610`); `004bc280` and `0050dd50` run with `011f4998`; the radius
/// (`+0xA0`, or `TESObjectREFR::GetRadius` `00568cb0` with a reference, as
/// a `float`) goes three times into `00416870` and that to `0050dd50`; the
/// colour to `004bc2e0`. The node's chain (`0043b230`, `004a8a90`) is
/// walked for one whose vtable slot 0x9C answers true, which makes
/// `0050ddb0` run with 1. Without a reference, or when its lit-water list
/// (`ExtraDataList::GetLitWaterRefs`, `0041f810`) is missing or its first
/// node is empty (`008256d0`), the light is added to the shadow scene node
/// (`00450b80` with 0, `ShadowSceneNode::AddLight` `00b5c940`) with 1 when
/// `force` is set, else `0050dd90`'s answer. Finally a light gets
/// `0050c350` with the radius truncated (`FISTP qword` toward zero) and
/// `0050dd20` with the `float` at `+0xB4`.
pub fn tes_object_ligh_gen_dynamic(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    node: u32,
    force: bool,
) -> u32 {
    let base = this.addr();
    if node == 0 {
        return 0;
    }
    if reference != 0 {
        let form = e.call(0x007a_f430, &args![reference]).u32();
        let fires = if e.mem.u32(base + 0xa8) & 0x20 != 0 {
            true
        } else if e.call(0x008d_6f30, &args![reference]).u32() == 0 {
            false
        } else {
            let place = e.vcall(reference, 0x1f4, &args![]).u32();
            let water = e.call(0x0057_b0a0, &args![reference]).f64();
            let height = x87_load32(e.mem.u32(place + 8));
            height < water
        };
        if fires {
            if e.call(0x0040_1170, &args![form]).u32() == 0x1e
                && e.call(0x0057_8400, &args![node]).bool()
            {
                e.call(0x0048_4a70, &args![reference, 1u32]);
                e.call(0x0050_d640, &args![node]);
            }
            if !e.vcall(reference, 0x100, &args![]).bool() && e.mem.u32(base + 0xa8) & 0x20 != 0 {
                return 0;
            }
        }
    }
    e.with_stack(0x140, |e, frame| {
        // The colour (`[EBP-0x20]`), `004a0bd0`'s colour, the name text
        // (0x104 bytes), the name string and the radius vector.
        let color = frame;
        let other_color = frame.byte_add(0xc);
        let text = frame.byte_add(0x18);
        let name_at = frame.byte_add(0x11c);
        let radius_at = frame.byte_add(0x124);
        e.call(0x0041_6870, &args![color, 0u32, 0u32, 0u32]);
        let packed = e.mem.u32(base + 0xa4);
        let scale = e.global::<f64>(COLOR_SCALE);
        for (i, byte) in [packed & 0xff, (packed >> 8) & 0xff, (packed >> 16) & 0xff]
            .into_iter()
            .enumerate()
        {
            let channel = x87_store32(x87_div(f64::from(byte), scale));
            e.mem.set_u32(color.addr() + 4 * i as u32, channel);
        }
        if e.mem.u32(base + 0xa8) & 4 != 0 {
            let source = e.call(0x004a_0bd0, &args![color, other_color]).u32();
            for i in 0..3 {
                let word = e.mem.u32(source + 4 * i);
                e.mem.set_u32(color.addr() + 4 * i, word);
            }
        }
        let mut parent = 0u32;
        let key = fn_0050de10(e);
        let holder = e.vcall(node, 0x9c, &args![key]).u32();
        if holder != 0 {
            parent = e.vcall(holder, 0xc, &args![]).u32();
        }
        if parent == 0 {
            parent = node;
        }
        let mut light = fn_0050d6d0(e, parent);
        if light == 0 {
            let light_name = e.vcall(base, 0x130, &args![]).u32();
            e.call(
                0x0040_6d00,
                &args![text, 0x104u32, 0x0102_90a4u32, light_name],
            );
            let block = e.call(NI_ALLOCATE, &args![0xfcu32]).u32();
            light = if block != 0 {
                e.call(0x00a7_d670, &args![block]).u32()
            } else {
                0
            };
            let name = e.call(NAME_CONSTRUCT, &args![name_at, text]).u32();
            e.call(SET_NAME, &args![light, name]);
            e.call(NAME_DESTRUCT, &args![name_at]);
            e.vcall(parent, 0xdc, &args![light, 1u32]);
        }
        if reference != 0 {
            e.call(0x0057_2610, &args![reference, light]);
        }
        e.call(0x004b_c280, &args![light, 0x011f_4998u32]);
        e.call(0x0050_dd50, &args![light, 0x011f_4998u32]);
        let mut radius = x87_store32(f64::from(e.mem.u32(base + 0xa0)));
        if reference != 0 {
            radius = x87_store32(e.call(0x0056_8cb0, &args![reference]).f64());
        }
        let pushed = x87_quiet_bits(radius);
        let vector = e
            .call(0x0041_6870, &args![radius_at, pushed, pushed, pushed])
            .u32();
        e.call(0x0050_dd50, &args![light, vector]);
        e.call(0x004b_c2e0, &args![light, color]);
        let mut walk = e.call(0x0043_b230, &args![node]).u32();
        while walk != 0 {
            if e.vcall(walk, 0x9c, &args![]).bool() {
                e.call(0x0050_ddb0, &args![this, 1u32]);
                break;
            }
            walk = e.call(0x004a_8a90, &args![walk]).u32();
        }
        let lit_water = |e: &mut Engine| {
            let extra = e.call(0x005d_43c0, &args![reference]).u32();
            e.call(0x0041_f810, &args![extra]).u32()
        };
        let add = if reference == 0 || lit_water(e) == 0 {
            true
        } else if lit_water(e) == 0 {
            false
        } else {
            let list = lit_water(e);
            e.call(0x0082_56d0, &args![list]).bool()
        };
        if add {
            let cast = if force {
                1u8
            } else {
                e.call(0x0050_dd90, &args![this]).u8()
            };
            let scene = e.call(0x0045_0b80, &args![0u32]).u32();
            e.call(0x00b5_c940, &args![scene, light, u32::from(cast)]);
        }
        if light != 0 {
            let whole = crate::units::fallout_shared::bgsterminal::fistp_truncated_qword(
                x87_load32(radius),
            );
            fn_0050c350(e, light, whole as u32);
            let extra = copy_float(e, base + 0xb4);
            e.call(0x0050_dd20, &args![light, extra]);
        }
        light
    })
}

// Translated from 0050de10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c620c` (`GenDynamic` asks the node's vtable slot 0x9C
/// for it).
pub fn fn_0050de10(e: &mut Engine) -> u32 {
    e.global::<u32>(0x011c_620c)
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESModelTextureSwap::Save`
/// (`0048a520`, three chunk tags), `TESScriptableForm::Save` (`0048cca0`),
/// `BGSDestructibleObjectForm::Save` (`00477f20`), `TESFullName::Save`
/// (`00487010`), `TESTexture::Save` (`0048e370`, a tag) and
/// `BGSMessageIcon::Save` (`0047a240`, a tag).
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
/// `00401500`: the byte at `011c54ba` (whether saved words are
/// byte-swapped); `00693dc0` (`thiscall`): byte-swaps the light's data.
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
const SWAP_LIGHT_DATA: u32 = 0x0069_3dc0;
/// `TESForm::__SaveData` (`00485e20`, `thiscall`, `RET 8`: data, size).
const SAVE_DATA: u32 = 0x0048_5e20;
/// `00485910` (cdecl(chunk, word)): adds a one-word chunk.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The form-id getter (`0084e3a0`, `thiscall`: the word at +0xc).
const FORM_ID: u32 = 0x0084_e3a0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors `TESForm` (`01183028`) and `TESObjectLIGH` (`01183938`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_LIGH: u32 = 0x0118_3938;
/// `TESForm::CopyAllComponents` (`004851b0`, `RET 4`) and
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// The light's accessors (`thiscall`): the float at +0xb4 in `ST0`
/// (`006d2c40`), its setter (`006e5ee0`, `RET 4`), the word at +0xb8
/// (`004036b0`) and its setter (`0050c9e0`, `RET 4`).
const FADE_VALUE: u32 = 0x006d_2c40;
const SET_FADE_VALUE: u32 = 0x006e_5ee0;
const SOUND_WORD: u32 = 0x0040_36b0;
const SET_SOUND_WORD: u32 = 0x0050_c9e0;
/// `_memcmp` (`00ec4835`, cdecl).
const MEMCMP: u32 = 0x00ec_4835;
/// `NiNode` helpers (`thiscall` on the node): the child count (`0043b480`)
/// and the child at an index (`0043b4a0`, `RET 4`); `NiAVObject::GetProperty`
/// (`00a59d30`, `RET 4`: the property type); `004bc450` (`thiscall` on the
/// property, `RET 4`) with the address `011f4998`.
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_CALL: u32 = 0x004b_c450;
const PROPERTY_ARGUMENT: u32 = 0x011f_4998;

// Translated from 0050cf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLIGH::Save` (engine map; no stack words): `StartForm`, the
/// model with texture swaps (`MODL`, `MODT`, `MODS`), the script and the
/// destructible data; with flag bit 1 of the word at +0xa8 the full name,
/// the icon (`ICON`) and the message icon (`MICO`), else clears flag bit 5
/// (0x20) there and sets the word at +0x9c to -1. Then the 0x18 bytes of
/// data at +0x9c (`__SaveData`), byte-swapped before and back after
/// (`00693dc0`) when `00401500` says so; the float at +0xb4 as chunk `FNAM`
/// (`FLD`/`FSTP` copy); with a sound (+0xb8) its form id as chunk `SNAM`;
/// `CloseForm`.
pub fn tes_object_ligh_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![base + 0x3c, SIG_MODL, SIG_MODT, SIG_MODS],
    );
    e.call(SCRIPTABLE_FORM_SAVE, &args![base + 0x78]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + 0x94]);
    if e.mem.u32(base + 0xa8) & 2 != 0 {
        e.call(FULL_NAME_SAVE, &args![base + 0x30]);
        e.call(TEXTURE_SAVE, &args![base + 0x5c, SIG_ICON]);
        e.call(MESSAGE_ICON_SAVE, &args![base + 0x68, SIG_MICO]);
    } else {
        let flags = e.mem.u32(base + 0xa8) & 0xffff_ffdf;
        e.mem.set_u32(base + 0xa8, flags);
        e.mem.set_u32(base + 0x9c, 0xffff_ffff);
    }
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_LIGHT_DATA, &args![base + 0x9c]);
    }
    e.call(SAVE_DATA, &args![this, base + 0x9c, 0x18u32]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_LIGHT_DATA, &args![base + 0x9c]);
    }
    let fade = copy_float(e, base + 0xb4);
    e.call(ADD_CHUNK_WORD, &args![SIG_FNAM, fade]);
    let sound = e.mem.u32(base + 0xb8);
    if sound != 0 {
        let id = e.call(FORM_ID, &args![sound]).u32();
        e.call(ADD_CHUNK_WORD, &args![SIG_SNAM, id]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 0050d060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The light's copy (engine map: no name; `RET 4`): when `other` is a
/// `TESObjectLIGH`, copies all components (`TESForm::CopyAllComponents`
/// with `other`), the 0x18 bytes of data (+0x9c), the float at +0xb4
/// (through `ST0`, stored with `FSTP dword`) and the word at +0xb8.
pub fn fn_0050d060(e: &mut Engine, this: Ptr, other: u32) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_LIGH, 0u32],
        )
        .u32();
    if source == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    let data = fn_0050d100(e, Ptr::new(source));
    for offset in (0..0x18).step_by(4) {
        let word = e.mem.u32(data + offset);
        e.mem.set_u32(this.addr() + 0x9c + offset, word);
    }
    let fade = e.call(FADE_VALUE, &args![source]).f64();
    e.call(SET_FADE_VALUE, &args![this, x87_store32(fade)]);
    let sound = e.call(SOUND_WORD, &args![source]).u32();
    e.call(SET_SOUND_WORD, &args![this, sound]);
}

// Translated from 0050d120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The light's compare (engine map: no name; `RET 4`): true when the two
/// differ: when `other` is no `TESObjectLIGH`; when
/// `TESForm::CompareAllComponents` (with the cast form) says so; when the
/// words at +0xb8 differ; when the 0x18 bytes of data differ (`_memcmp`);
/// when the floats at +0xb4 compare unequal (`FUCOMPP`, a NaN included).
pub fn fn_0050d120(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_LIGH, 0u32],
        )
        .u32();
    if source == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).u32() as u8 != 0 {
        return true;
    }
    let sound = e.call(SOUND_WORD, &args![source]).u32();
    if e.mem.u32(this.addr() + 0xb8) != sound {
        return true;
    }
    let data = fn_0050d100(e, Ptr::new(source));
    if e.call(MEMCMP, &args![this.addr() + 0x9c, data, 0x18u32])
        .u32()
        != 0
    {
        return true;
    }
    let mine = e.call(FADE_VALUE, &args![this]).f64();
    let theirs = e.call(FADE_VALUE, &args![source]).f64();
    // FUCOMPP then `TEST AH,0x44; JNP`: only "equal" (C3 alone) answers false;
    // unordered (a NaN) answers true, as `!=` does.
    mine != theirs
}

// Translated from 0050d640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLIGH::StopParticlesRecursive` (engine map; cdecl, the node):
/// for each child of `node` (the count asked again each time), skipping
/// null ones: a child whose vtable slot `+0xc` answers non-zero is walked
/// the same way; otherwise its property of type 2 (`GetProperty`), when it
/// has one, gets `004bc450` with the address `011f4998`.
pub fn tes_object_ligh_stop_particles_recursive(e: &mut Engine, node: u32) {
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![node]).u32() {
        let child = e.call(CHILD_AT, &args![node, index]).u32();
        if child != 0 {
            if e.vcall(child, 0xc, &[]).u32() != 0 {
                tes_object_ligh_stop_particles_recursive(e, child);
            } else {
                let property = e.call(GET_PROPERTY, &args![child, 2u32]).u32();
                if property != 0 {
                    e.call(PROPERTY_CALL, &args![property, PROPERTY_ARGUMENT]);
                }
            }
        }
        index += 1;
    }
}

// Translated from 0050ddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on` non-zero) or clears flag bit 0 of the light's flags word at
/// +0xa8 (engine map: no name; `RET 4`, a byte).
pub fn fn_0050ddb0(e: &mut Engine, this: Ptr, on: u8) {
    let flags = e.mem.u32(this.addr() + 0xa8);
    let flags = if on != 0 {
        flags | 1
    } else {
        flags & 0xffff_fffe
    };
    e.mem.set_u32(this.addr() + 0xa8, flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050d4a0, fn_0050d4a0(Ptr) -> bool),
        entry!(0x0050_c350, fn_0050c350(u32, u32)),
        entry!(0x0050_ca00, tes_object_ligh_init_item(Ptr)),
        entry!(0x0050_caf0, fn_0050caf0(Ptr, Ptr) -> bool),
        entry!(0x0050_d100, fn_0050d100(Ptr) -> u32),
        entry!(0x0050_d1d0, fn_0050d1d0(Ptr) -> bool),
        entry!(0x0050_d1f0, fn_0050d1f0(Ptr) -> bool),
        entry!(0x0050_d210, fn_0050d210(Ptr) -> bool),
        entry!(0x0050_d230, fn_0050d230(Ptr) -> bool),
        entry!(0x0050_d250, tes_object_ligh_clone_3d(Ptr, u32) -> u32),
        entry!(0x0050_d4c0, fn_0050d4c0(Ptr, u32) -> u32),
        entry!(0x0050_d600, fn_0050d600(Ptr, u32)),
        entry!(0x0050_d6d0, fn_0050d6d0(u32) -> u32),
        entry!(
            0x0050_d810,
            tes_object_ligh_gen_dynamic(Ptr, u32, u32, bool) -> u32
        ),
        entry!(0x0050_de10, fn_0050de10() -> u32),
        entry!(0x0050_cf20, tes_object_ligh_save(Ptr)),
        entry!(0x0050_d060, fn_0050d060(Ptr, u32)),
        entry!(0x0050_d120, fn_0050d120(Ptr, u32) -> bool),
        entry!(0x0050_d640, tes_object_ligh_stop_particles_recursive(u32)),
        entry!(0x0050_ddb0, fn_0050ddb0(Ptr, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_bit_0x10_of_the_dword_at_8() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(16));
        e.mem.set_u32(this.addr() + 8, 0xffff_ffef);
        assert!(!e.call(0x0050_d4a0, &args![this]).bool());
        e.mem.set_u32(this.addr() + 8, 0x10);
        assert!(e.call(0x0050_d4a0, &args![this]).bool());
    }
}

#[cfg(test)]
mod b023_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn small_accessors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        assert_eq!(e.call(0x0050_d100, &[this]).u32(), this + 0x9c);
        for (addr, bit) in [
            (0x0050_d1d0, 0x8u32),
            (0x0050_d1f0, 0x40),
            (0x0050_d210, 0x80),
            (0x0050_d230, 0x100),
        ] {
            e.mem.set_u32(this + 0xa8, !bit);
            assert!(!e.call(addr, &[this]).bool());
            e.mem.set_u32(this + 0xa8, bit);
            assert!(e.call(addr, &[this]).bool());
        }
        e.map(0x011c_6000, 0x1000);
        e.set_global(0x011c_620c, 0x77u32);
        assert_eq!(e.call(0x0050_de10, &[]).u32(), 0x77);
    }

    /// Settings for the attenuation: each setting's value at `+4`.
    fn attenuation_doubles(e: &mut Engine, flags: u8, linear: u32, quadratic: u32) {
        e.map(0x011c_8000, 0x2000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ATTENUATION_FALLBACK, 0.5f64);
        e.set_global(ATTENUATION_FLAGS, flags);
        e.set_global(TES_011DEA10, 0x0abc_0000u32);
        for (s, v) in [
            (CONSTANT_VALUE, 0.25f32),
            (LINEAR_RADIUS_MULT, 2.0),
            (LINEAR_VALUE, 3.0),
            (QUADRATIC_RADIUS_MULT, 0.5),
            (QUADRATIC_VALUE, 8.0),
        ] {
            e.mem.set_f32(s + 4, v);
        }
        e.mem.set_u32(LINEAR_METHOD + 4, linear);
        e.mem.set_u32(QUADRATIC_METHOD + 4, quadratic);
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(0x0043_d4d0, |_, a| ret(a[0] + 4));
        e.register(0x0045_0410, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e.register(0x0065_3270, |_, a| {
            ret(if a[1] == 0 { 0 } else { 0x5000 + a[1] })
        });
        quiet(e, &[SET_CONSTANT, SET_LINEAR, SET_QUADRATIC]);
    }

    fn values(log: &[(u32, Vec<u32>)], setter: u32) -> Vec<f32> {
        calls_to(log, setter)
            .iter()
            .map(|w| f32::from_bits(w[1]))
            .collect()
    }

    #[test]
    fn attenuation_by_method() {
        let mut e = Engine::new();
        attenuation_doubles(&mut e, 1 | 2 | 4, 2, 7);
        e.call_log = Some(vec![]);
        e.call(0x0050_c350, &[0, 4]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        // Radius 4: t = 8, linear method 2: 3 / 64; q = 2, quadratic
        // (other) method: 8 / 4.
        e.call_log = Some(vec![]);
        e.call(0x0050_c350, &[1, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0065_3270), vec![vec![0x011f_4a98, 1]]);
        assert_eq!(values(&log, SET_CONSTANT), vec![0.25]);
        assert_eq!(values(&log, SET_LINEAR), vec![0.0, 3.0 / 64.0]);
        assert_eq!(values(&log, SET_QUADRATIC), vec![0.0, 2.0]);
        assert_eq!(calls_to(&log, SET_LINEAR)[0][0], 0x5001);
        // Linear method 1 (other): 3 * (1 / 8); quadratic method 1:
        // 8 * (1 / radius); method 0: the values themselves.
        e.mem.set_u32(LINEAR_METHOD + 4, 1);
        e.mem.set_u32(QUADRATIC_METHOD + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0050_c350, &[1, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(values(&log, SET_LINEAR)[1], 0.375);
        assert_eq!(values(&log, SET_QUADRATIC)[1], 2.0);
        e.mem.set_u32(LINEAR_METHOD + 4, 0);
        e.mem.set_u32(QUADRATIC_METHOD + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_c350, &[1, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(values(&log, SET_LINEAR)[1], 3.0);
        assert_eq!(values(&log, SET_QUADRATIC)[1], 8.0);
        // A radius of 0 makes both factors 0: the fallback.
        e.mem.set_u32(LINEAR_METHOD + 4, 2);
        e.mem.set_u32(QUADRATIC_METHOD + 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x0050_c350, &[1, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(values(&log, SET_LINEAR)[1], 0.5);
        assert_eq!(values(&log, SET_QUADRATIC)[1], 0.5);
    }

    #[test]
    fn attenuation_without_the_flags() {
        let mut e = Engine::new();
        attenuation_doubles(&mut e, 0, 0, 0);
        // 005f36f0 answers non-zero: linear only, when bOutQuadInLin is
        // set; quadratic never.
        e.register(0x005f_36f0, |_, _| ret(0x99));
        e.register(0x0050_e620, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0050_c350, &[1, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(values(&log, SET_CONSTANT), vec![0.0]);
        assert_eq!(values(&log, SET_LINEAR), vec![0.0, 3.0]);
        assert_eq!(values(&log, SET_QUADRATIC), vec![0.0]);
        assert_eq!(calls_to(&log, 0x005f_36f0)[0], vec![0x0abc_0000]);
        assert_eq!(calls_to(&log, 0x0050_e620)[0], vec![OUT_QUAD_IN_LIN, 1]);
        // It answers zero: quadratic only.
        e.register(0x005f_36f0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0050_c350, &[1, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(values(&log, SET_LINEAR), vec![0.0]);
        assert_eq!(values(&log, SET_QUADRATIC), vec![0.0, 8.0]);
        // bOutQuadInLin not set (AL 0): neither.
        e.register(0x0050_e620, |_, _| ret(0x100));
        e.call_log = Some(vec![]);
        e.call(0x0050_c350, &[1, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(values(&log, SET_LINEAR), vec![0.0]);
        assert_eq!(values(&log, SET_QUADRATIC), vec![0.0]);
    }

    /// A light whose vtable has slot 0x130 at `0x7000_0030` (answering
    /// `0x0102_0304`), 0x134 at `0x7000_0000`, 0xE0 at `0x7000_0004` and
    /// 0x174 at `0x7000_0008`; its model (`+0x3C`) has slot 0x14 at
    /// `0x7000_0014`.
    fn light(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 0x178 / 4];
        slots[0x130 / 4] = 0x7000_0030;
        slots[0x134 / 4] = 0x7000_0000;
        slots[0xe0 / 4] = 0x7000_0004;
        slots[0x174 / 4] = 0x7000_0008;
        e.put_vtable(0x0600_0000, &slots);
        e.put_vtable(0x0610_0000, &[0, 0, 0, 0, 0, 0x7000_0014]);
        e.register(0x7000_0030, |_, _| ret(0x0102_0304));
        quiet(e, &[0x7000_0000, 0x7000_0004]);
        let this = e.mem.alloc(0xc8);
        e.mem.set_u32(this, 0x0600_0000);
        e.mem.set_u32(this + 0x3c, 0x0610_0000);
        this
    }

    #[test]
    fn init_item_resolves_the_sound() {
        let mut e = Engine::new();
        let this = light(&mut e);
        e.register(FORM_FLAG_8_SET, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0050_ca00, &[this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        e.register(FORM_FLAG_8_SET, |_, _| ret(0));
        quiet(
            &mut e,
            &[
                0x0048_cd90,
                0x0047_8be0,
                0x0048_aa80,
                0x005b_5e40,
                SET_FORM_FLAG_8,
            ],
        );
        e.register(0x0048_4e60, |_, _| ret(0x0400_0000));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x100);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| {
            ret(if a[0] == 0x101 { 0x0333_0000 } else { 0 })
        });
        e.register(0x00ec_43fb, |_, a| ret(a[0]));
        // No sound: no lookup.
        e.call_log = Some(vec![]);
        e.call(0x0050_ca00, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0048_39c0).is_empty());
        assert_eq!(calls_to(&log, 0x0048_cd90), vec![vec![this + 0x78, this]]);
        assert_eq!(calls_to(&log, 0x0047_8be0), vec![vec![this + 0x94, this]]);
        assert_eq!(calls_to(&log, 0x0048_aa80), vec![vec![this + 0x3c, this]]);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this, 1]]);
        // A sound that resolves, and one that does not.
        e.mem.set_u32(this + 0xb8, 1);
        e.call_log = Some(vec![]);
        e.call(0x0050_ca00, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xb8), 0x0333_0000);
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![vec![0x0333_0000, 0, 0x0118_3028, 0x0118_32fc, 0]]
        );
        e.mem.set_u32(this + 0xb8, 2);
        e.call_log = Some(vec![]);
        e.call(0x0050_ca00, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![0x0102_9064, 0x102, 0x0102_0304]]
        );
    }

    #[test]
    fn load_reads_the_light_record() {
        let mut e = Engine::new();
        let this = light(&mut e);
        let file = 0x0500_0000u32;
        e.register(GET_TES_FORM, |_, _| ret(0x1d));
        assert!(!e.call(0x0050_caf0, &[this, file]).bool());
        e.register(GET_TES_FORM, |_, _| ret(0x1e));
        let mut chunks = vec![
            SIG_EDID, SIG_OBND, SIG_FULL, SIG_MODL, SIG_MODD, SIG_MOSD, SIG_MODT, SIG_MODS,
            SIG_ICON, SIG_MICO, SIG_DSTD, SIG_DEST, SIG_SCRI, SIG_FNAM, SIG_SNAM, SIG_DATA,
            SIG_DATA, 0x1234,
        ];
        chunks.reverse();
        let chunks = Rc::new(RefCell::new(chunks));
        let c = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(c.borrow_mut().pop().unwrap_or(0))
        });
        let c = chunks.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            ret(u32::from(!c.borrow().is_empty()))
        });
        e.register(CHUNK_SIZE, |_, _| ret(8));
        let n = Rc::new(RefCell::new(0x500u32));
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let mut k = n.borrow_mut();
            *k += 1;
            e.mem.set_u32(a[1], *k);
            Ret::default()
        });
        e.register(0x0040_3570, |_, _| ret(0x000e));
        // The first DATA leaves +0xB0 and +0xAC at 0; the second sets them.
        let data = Rc::new(RefCell::new(0u32));
        let d = data.clone();
        e.register_double(LOAD_DATA, move |e, a| {
            let mut k = d.borrow_mut();
            *k += 1;
            if *k == 2 {
                e.mem.set_f32(a[2] + 0x14, 3.0);
                e.mem.set_f32(a[2] + 0x10, 4.0);
            }
            Ret::default()
        });
        e.register(SWAP_BYTES, |_, _| ret(1));
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.map(0x0102_4000, 0x1000);
        e.set_global(DEFAULT_B0, 0.125f32);
        quiet(
            &mut e,
            &[
                LOAD_FORM,
                SET_FORM_FLAG_8,
                GET_CHUNK_DATA_SIZED,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x0048_e3d0,
                0x0047_a290,
                0x0047_81e0,
                0x006e_cd40,
                0x0048_cd90,
                0x0069_3dc0,
            ],
        );
        e.mem.set_f32(this + 0xb0, 9.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_caf0, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOAD_FORM), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this, 0]]);
        let sized = calls_to(&log, GET_CHUNK_DATA_SIZED);
        assert_eq!((sized[0][2], sized[1][2]), (0x200, 8));
        assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, 0x0048_7050), vec![vec![this + 0x30, file]]);
        assert_eq!(
            calls_to(&log, 0x0048_92d0),
            vec![vec![this + 0x3c, file]; 4]
        );
        assert_eq!(
            calls_to(&log, 0x0048_a7b0),
            vec![vec![this + 0x3c, sized[1][1], 8, 0xe]]
        );
        assert_eq!(calls_to(&log, 0x0048_e3d0), vec![vec![this + 0x5c, file]]);
        assert_eq!(calls_to(&log, 0x0047_a290), vec![vec![this + 0x68, file]]);
        assert_eq!(
            calls_to(&log, 0x0047_81e0),
            vec![vec![this + 0x94, file]; 2]
        );
        assert_eq!(calls_to(&log, 0x006e_cd40), vec![vec![this + 0x78, 0x501]]);
        assert_eq!(calls_to(&log, 0x0048_cd90), vec![vec![this + 0x78, this]]);
        assert_eq!(e.mem.u32(this + 0xb4), 0x502);
        assert_eq!(e.mem.u32(this + 0xb8), 0x503);
        assert_eq!(
            calls_to(&log, LOAD_DATA),
            vec![vec![this, file, this + 0x9c, 0x18]; 2]
        );
        assert_eq!(calls_to(&log, 0x0069_3dc0), vec![vec![this + 0x9c]; 2]);
        // The second DATA set values that are not zero: kept.
        assert_eq!(e.mem.f32(this + 0xb0), 3.0);
        assert_eq!(e.mem.f32(this + 0xac), 4.0);
        // A DATA chunk leaving zeros: the defaults.
        chunks.borrow_mut().push(SIG_DATA);
        e.call(0x0050_caf0, &[this, file]);
        assert_eq!(e.mem.f32(this + 0xb0), 0.125);
        assert_eq!(e.mem.f32(this + 0xac), 1.0);
    }

    #[test]
    fn model_queue_and_node_wrapping() {
        let mut e = Engine::new();
        let this = light(&mut e);
        e.register(0x7000_0014, |_, _| ret(0x0bbb_0000));
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.set_global(MODEL_LOADER, 0x0777_0000u32);
        e.register(0x0045_a5e0, |_, _| Ret::default());
        e.register(MODEL_HAS_NAME, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0050_d600, &[this, 0x99]);
        assert!(calls_to(e.call_log.as_ref().unwrap(), 0x0045_a5e0).is_empty());
        e.register(MODEL_HAS_NAME, |_, _| ret(1));
        e.call(0x0050_d600, &[this, 0x99]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0045_a5e0),
            vec![vec![0x0777_0000, 0x0bbb_0000]]
        );

        // 0050d4c0: a named model gives 0050fe50's object; objects answer
        // their word at +4 to slot 0xC.
        e.put_vtable(0x0620_0000, &[0, 0, 0, 0x7000_0040]);
        e.register(0x7000_0040, |e, a| ret(e.mem.u32(a[0] + 4)));
        let mut node_slots = vec![0u32; 0xe0 / 4];
        node_slots[3] = 0x7000_0040;
        node_slots[0xdc / 4] = 0x7000_0044;
        e.put_vtable(0x0630_0000, &node_slots);
        e.register(0x7000_0044, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0630_0000);
            e.mem.set_u32(a[0] + 4, 1);
            ret(a[0])
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0620_0000);
        e.register_double(0x0050_fe50, move |_, _| ret(object));
        e.register(0x0044_a670, |_, _| ret(3));
        e.call_log = Some(vec![]);
        let wrapped = e.call(0x0050_d4c0, &[this, 0x42]).u32();
        let log = e.call_log.take().unwrap();
        assert_ne!(wrapped, object);
        assert_eq!(calls_to(&log, 0x0050_fe50), vec![vec![this, 0x42]]);
        assert_eq!(calls_to(&log, 0x7000_0044), vec![vec![wrapped, object, 1]]);
        // A node object stays; an empty name makes a new node.
        e.mem.set_u32(object + 4, 1);
        assert_eq!(e.call(0x0050_d4c0, &[this, 0x42]).u32(), object);
        e.register(0x0044_a670, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let fresh = e.call(0x0050_d4c0, &[this, 0x42]).u32();
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0050_fe50).is_empty());
        assert_eq!(calls_to(&log, NODE_CONSTRUCT), vec![vec![fresh, 0]]);
        // A failed allocation gives null.
        e.register(NI_ALLOCATE, |_, _| ret(0));
        assert_eq!(e.call(0x0050_d4c0, &[this, 0x42]).u32(), 0);
    }

    /// A scene graph for `0050d6d0`: objects are `[vtable, kind, child
    /// count, children...]`, kind 1 a node; `0043b300` accepts kind 2.
    fn tree_doubles(e: &mut Engine) {
        e.put_vtable(0x0640_0000, &[0, 0, 0, 0x7000_0050]);
        e.register(0x7000_0050, |e, a| ret(u32::from(e.mem.u32(a[0] + 4) == 1)));
        e.register(0x0043_b480, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(0x0043_b4a0, |e, a| ret(e.mem.u32(a[0] + 12 + 4 * a[1])));
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_OBJECT, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_RELEASE, |_, _| Ret::default());
        e.register(0x0043_b300, |e, a| {
            assert_eq!(a[0], 0x011f_4a28);
            ret(u32::from(e.mem.u32(a[1] + 4) == 2))
        });
    }

    fn object(e: &mut Engine, kind: u32, children: &[u32]) -> u32 {
        let p = e.mem.alloc(12 + 4 * children.len() as u32);
        e.mem.set_u32(p, 0x0640_0000);
        e.mem.set_u32(p + 4, kind);
        e.mem.set_u32(p + 8, children.len() as u32);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(p + 12 + 4 * i as u32, *c);
        }
        p
    }

    #[test]
    fn finds_the_first_light_depth_first() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        assert_eq!(e.call(0x0050_d6d0, &[0]).u32(), 0);
        let other = object(&mut e, 3, &[]);
        let deep_light = object(&mut e, 2, &[]);
        let inner = object(&mut e, 1, &[other, deep_light]);
        let late_light = object(&mut e, 2, &[]);
        let empty = object(&mut e, 1, &[]);
        let root = object(&mut e, 1, &[0, empty, inner, late_light]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_d6d0, &[root]).u32(), deep_light);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, POINTER_CONSTRUCT).len(),
            calls_to(&log, POINTER_RELEASE).len()
        );
        let none = object(&mut e, 1, &[other, empty]);
        assert_eq!(e.call(0x0050_d6d0, &[none]).u32(), 0);
    }

    /// Doubles for `GenDynamic`; returns a light object to find.
    fn gen_doubles(e: &mut Engine) -> u32 {
        tree_doubles(e);
        let found = object(e, 2, &[]);
        // The node's vtable: slot 0x9C gives no holder.
        let mut slots = vec![0u32; 0xe0 / 4];
        slots[3] = 0x7000_0050;
        slots[0x9c / 4] = 0x7000_0060;
        slots[0xdc / 4] = 0x7000_0064;
        e.put_vtable(0x0650_0000, &slots);
        e.register(0x7000_0060, |_, _| ret(0));
        quiet(
            e,
            &[
                0x7000_0064,
                0x0041_6870,
                0x0057_2610,
                0x004b_c280,
                0x0050_dd50,
                0x004b_c2e0,
                0x0050_ddb0,
                0x00b5_c940,
                0x0050_dd20,
                0x0065_3270,
            ],
        );
        e.register(0x0043_b230, |_, _| ret(0));
        e.register(0x0050_dd90, |_, _| ret(0x0107));
        e.register(0x0045_0b80, |_, _| ret(0x0888_0000));
        e.map(0x011c_6000, 0x1000);
        e.set_global(0x011c_620c, 0x55u32);
        e.map(0x0101_e000, 0x1000);
        e.set_global(COLOR_SCALE, 255.0f64);
        found
    }

    fn node_holding(e: &mut Engine, child: u32) -> u32 {
        let node = e.mem.alloc(16);
        e.mem.set_u32(node, 0x0650_0000);
        e.mem.set_u32(node + 4, 1);
        e.mem.set_u32(node + 8, 1);
        e.mem.set_u32(node + 12, child);
        node
    }

    #[test]
    fn gen_dynamic_without_a_reference() {
        let mut e = Engine::new();
        let this = light(&mut e);
        let found = gen_doubles(&mut e);
        assert_eq!(e.call(0x0050_d810, &[this, 0, 0, 0]).u32(), 0);
        let node = node_holding(&mut e, found);
        e.mem.set_u32(this + 0xa4, 0x00ff_3300);
        e.mem.set_u32(this + 0xa0, 300);
        e.mem.set_f32(this + 0xb4, 0.75);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_d810, &[this, 0, node, 0]).u32(), found);
        let log = e.call_log.take().unwrap();
        let color = calls_to(&log, 0x0041_6870)[0][0];
        assert_eq!(calls_to(&log, 0x0041_6870)[0][1..], [0, 0, 0]);
        assert_eq!(e.mem.f32(color), 0.0);
        assert_eq!(e.mem.u32(color + 4), x87_store32(51.0 / 255.0));
        assert_eq!(e.mem.f32(color + 8), 1.0);
        assert_eq!(calls_to(&log, 0x7000_0060), vec![vec![node, 0x55]]);
        assert!(calls_to(&log, 0x0057_2610).is_empty());
        assert_eq!(calls_to(&log, 0x004b_c280), vec![vec![found, 0x011f_4998]]);
        let radius = calls_to(&log, 0x0041_6870)[1].clone();
        assert_eq!(radius[1..], [300f32.to_bits(); 3]);
        assert_eq!(calls_to(&log, 0x004b_c2e0), vec![vec![found, color]]);
        assert_eq!(
            calls_to(&log, 0x00b5_c940),
            vec![vec![0x0888_0000, found, 7]]
        );
        assert_eq!(calls_to(&log, 0x0065_3270), vec![vec![0x011f_4a98, found]]);
        assert_eq!(
            calls_to(&log, 0x0050_dd20),
            vec![vec![found, 0.75f32.to_bits()]]
        );
        // Forced: 1 without asking 0050dd90.
        e.call_log = Some(vec![]);
        e.call(0x0050_d810, &[this, 0, node, 1]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0050_dd90).is_empty());
        assert_eq!(calls_to(&log, 0x00b5_c940)[0][2], 1);
    }

    #[test]
    fn gen_dynamic_makes_a_named_light_for_a_reference() {
        let mut e = Engine::new();
        let this = light(&mut e);
        gen_doubles(&mut e);
        let empty = object(&mut e, 1, &[]);
        let node = node_holding(&mut e, empty);
        // The reference: slot 0x100 answers 1, 0x1F4 a place at height 5.
        let mut slots = vec![0u32; 0x1f8 / 4];
        slots[0x100 / 4] = 0x7000_0070;
        slots[0x1f4 / 4] = 0x7000_0074;
        e.put_vtable(0x0660_0000, &slots);
        let place = e.mem.alloc(12);
        e.mem.set_f32(place + 8, 5.0);
        e.register(0x7000_0070, |_, _| ret(1));
        e.register_double(0x7000_0074, move |_, _| ret(place));
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x0660_0000);
        e.register(0x007a_f430, |_, _| ret(0x0444_0000));
        e.register(0x008d_6f30, |_, _| ret(1));
        e.register(0x0057_b0a0, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(0x0040_1170, |_, _| ret(0x1e));
        e.register(0x0057_8400, |_, _| ret(1));
        quiet(
            &mut e,
            &[
                0x0048_4a70,
                0x0050_d640,
                0x0040_6d00,
                NAME_DESTRUCT,
                SET_NAME,
            ],
        );
        e.register(NAME_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x00a7_d670, |_, a| ret(a[0] + 1));
        e.register(0x0056_8cb0, |_, _| Ret {
            st0: 2.75,
            ..Ret::default()
        });
        // The lit-water list exists and is not empty: not added.
        e.register(0x005d_43c0, |_, a| ret(a[0] + 0x100));
        e.register(0x0041_f810, |_, _| ret(0x0999_0000));
        e.register(0x0082_56d0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let made = e.call(0x0050_d810, &[this, reference, node, 0]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_4a70), vec![vec![reference, 1]]);
        assert_eq!(calls_to(&log, 0x0050_d640), vec![vec![node]]);
        let text = calls_to(&log, 0x0040_6d00);
        assert_eq!(text[0][1..], [0x104, 0x0102_90a4, 0x0102_0304]);
        assert_eq!(calls_to(&log, SET_NAME)[0][0], made);
        assert_eq!(calls_to(&log, 0x7000_0064), vec![vec![node, made, 1]]);
        assert_eq!(calls_to(&log, 0x0057_2610), vec![vec![reference, made]]);
        assert_eq!(calls_to(&log, 0x0041_6870)[1][1], 2.75f32.to_bits());
        assert!(calls_to(&log, 0x00b5_c940).is_empty());
        assert_eq!(calls_to(&log, 0x0065_3270), vec![vec![0x011f_4a98, made]]);
        // A height not below the water (and no flag 0x20) skips the fire
        // test; an empty lit-water list adds the light.
        e.mem.set_f32(place + 8, 10.0);
        e.register(0x0082_56d0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0050_d810, &[this, reference, node, 0]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0040_1170).is_empty());
        assert_eq!(calls_to(&log, 0x00b5_c940).len(), 1);
        // Flag 0x20 and a reference whose slot 0x100 answers false: null.
        e.mem.set_u32(this + 0xa8, 0x20);
        e.register(0x7000_0070, |_, _| ret(0x100));
        assert_eq!(e.call(0x0050_d810, &[this, reference, node, 0]).u32(), 0);
    }

    #[test]
    fn clone_3d_builds_the_node() {
        let mut e = Engine::new();
        let this = light(&mut e);
        let found = gen_doubles(&mut e);
        // The model reference answers null first.
        e.register(0x7000_0008, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_d250, &[this, 0x0abc_0000]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, POINTER_RELEASE).len(), 2);
        assert_eq!(calls_to(&log, 0x7000_0008), vec![vec![this, 0x0abc_0000]]);

        // A model whose slot 0x10 gives a node; no model file name.
        let node = node_holding(&mut e, found);
        let mut model_slots = vec![0u32; 5];
        model_slots[4] = 0x7000_0080;
        e.put_vtable(0x0670_0000, &model_slots);
        let model = e.mem.alloc(8);
        e.mem.set_u32(model, 0x0670_0000);
        e.register_double(0x7000_0008, move |_, _| ret(model));
        e.register_double(0x7000_0080, move |_, _| ret(node));
        e.register(0x0047_4cb0, |_, _| ret(0x20));
        e.register(MODEL_HAS_NAME, |_, _| ret(0));
        e.register(NAME_CONSTRUCT, |_, a| ret(a[0]));
        quiet(
            &mut e,
            &[
                NAME_DESTRUCT,
                SET_NAME,
                0x0069_3ef0,
                0x004a_d050,
                0x004a_d270,
                0x00a5_a040,
            ],
        );
        e.register(0x0056_7400, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        // With a reference GenDynamic stops early: flag 0x20, a base form
        // of another type, slot 0x100 answering false.
        e.mem.set_u32(this + 0x08, 0x10);
        e.mem.set_u32(this + 0xa8, 0x20);
        e.register(0x007a_f430, |_, _| ret(0));
        e.register(0x0040_1170, |_, _| ret(0));
        let mut ref_slots = vec![0u32; 0x104 / 4];
        ref_slots[0x100 / 4] = 0x7000_0090;
        e.put_vtable(0x0680_0000, &ref_slots);
        e.register(0x7000_0090, |_, _| ret(0));
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x0680_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_d250, &[this, reference]).u32(), node);
        let log = e.call_log.take().unwrap();
        let name = calls_to(&log, NAME_CONSTRUCT)[0].clone();
        assert_eq!(name[1], 0x0102_0304);
        assert_eq!(calls_to(&log, SET_NAME)[0], vec![node, name[0]]);
        assert_eq!(calls_to(&log, 0x0069_3ef0), vec![vec![reference, 1]]);
        assert_eq!(calls_to(&log, 0x004a_d050)[0][1], 1.5f32.to_bits());
        assert_eq!(calls_to(&log, 0x00a5_a040), vec![vec![node]]);
        assert_eq!(calls_to(&log, POINTER_RELEASE).len(), 2);
        assert_eq!(calls_to(&log, 0x004a_d270).len(), 1);

        // With a model file name the model is cloned; without a reference
        // the scale is 1.0 and 00693ef0 does not run (GenDynamic runs in
        // full on the node).
        e.register(MODEL_HAS_NAME, |_, _| ret(1));
        e.register_double(0x00a5_d2c0, move |_, a| {
            assert_ne!(a[1], 0);
            ret(node)
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_d250, &[this, 0]).u32(), node);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00a5_d2c0)[0][0], model);
        assert!(calls_to(&log, 0x0069_3ef0).is_empty());
        assert_eq!(calls_to(&log, 0x004a_d050)[0][1], 1.0f32.to_bits());
        assert!(calls_to(&log, SET_NAME).is_empty());
        assert_eq!(calls_to(&log, 0x00b5_c940)[0][1], found);

        // No name and no node from the model: a new fade node.
        e.register(MODEL_HAS_NAME, |_, _| ret(0));
        e.register(0x7000_0080, |_, _| ret(0));
        e.register(NI_ALLOCATE, |_, _| ret(0x0c00_0000));
        e.register_double(0x00b4_e150, move |_, a| {
            assert_eq!(a[0], 0x0c00_0000);
            ret(node)
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_d250, &[this, 0]).u32(), node);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NI_ALLOCATE), vec![vec![0xe4]]);
    }
}

#[cfg(test)]
mod save_copy_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    fn light(e: &mut Engine) -> u32 {
        e.mem.alloc(0xc8)
    }

    #[test]
    fn save_writes_the_components_and_chunks() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                START_FORM,
                CLOSE_FORM,
                MODEL_TEXTURE_SWAP_SAVE,
                SCRIPTABLE_FORM_SAVE,
                DESTRUCTIBLE_SAVE,
                FULL_NAME_SAVE,
                TEXTURE_SAVE,
                MESSAGE_ICON_SAVE,
                SWAP_LIGHT_DATA,
                SAVE_DATA,
                ADD_CHUNK_WORD,
            ],
        );
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(0));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let this = light(&mut e);
        e.mem.set_u32(this + 0xa8, 0x22);
        e.mem.set_u32(this + 0xb4, 0x7f80_0001);
        let sound = e.mem.alloc(0x18);
        e.mem.set_u32(sound + 0xc, 0x0000_abcd);
        e.mem.set_u32(this + 0xb8, sound);
        let (_, log) = run(&mut e, 0x0050_cf20, &args![this]);
        assert_eq!(
            only(&log, MODEL_TEXTURE_SWAP_SAVE),
            vec![vec![this + 0x3c, SIG_MODL, SIG_MODT, SIG_MODS]]
        );
        assert_eq!(only(&log, SCRIPTABLE_FORM_SAVE), vec![vec![this + 0x78]]);
        assert_eq!(only(&log, DESTRUCTIBLE_SAVE), vec![vec![this + 0x94]]);
        assert_eq!(only(&log, FULL_NAME_SAVE), vec![vec![this + 0x30]]);
        assert_eq!(only(&log, TEXTURE_SAVE), vec![vec![this + 0x5c, SIG_ICON]]);
        assert_eq!(
            only(&log, MESSAGE_ICON_SAVE),
            vec![vec![this + 0x68, SIG_MICO]]
        );
        assert_eq!(only(&log, SAVE_DATA), vec![vec![this, this + 0x9c, 0x18]]);
        assert_eq!(
            only(&log, ADD_CHUNK_WORD),
            vec![vec![SIG_FNAM, 0x7fc0_0001], vec![SIG_SNAM, 0xabcd]]
        );
        assert!(only(&log, SWAP_LIGHT_DATA).is_empty());
        assert_eq!(only(&log, CLOSE_FORM), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0xa8), 0x22);
    }

    #[test]
    fn save_without_flag_two_clears_bit_five_and_swaps_when_asked() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                START_FORM,
                CLOSE_FORM,
                MODEL_TEXTURE_SWAP_SAVE,
                SCRIPTABLE_FORM_SAVE,
                DESTRUCTIBLE_SAVE,
                FULL_NAME_SAVE,
                SWAP_LIGHT_DATA,
                SAVE_DATA,
                ADD_CHUNK_WORD,
            ],
        );
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(0x101));
        let this = light(&mut e);
        e.mem.set_u32(this + 0xa8, 0x21);
        e.mem.set_u32(this + 0x9c, 5);
        let (_, log) = run(&mut e, 0x0050_cf20, &args![this]);
        assert_eq!(e.mem.u32(this + 0xa8), 1);
        assert_eq!(e.mem.u32(this + 0x9c), 0xffff_ffff);
        assert!(only(&log, FULL_NAME_SAVE).is_empty());
        assert_eq!(only(&log, SWAP_LIGHT_DATA).len(), 2);
        assert_eq!(only(&log, ADD_CHUNK_WORD).len(), 1);
    }

    #[test]
    fn copy_takes_data_fade_and_sound() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(FADE_VALUE, |e, a| Ret {
            st0: x87_load32(e.mem.u32(a[0] + 0xb4)),
            ..Ret::default()
        });
        e.register(SET_FADE_VALUE, |e, a| {
            e.mem.set_u32(a[0] + 0xb4, a[1]);
            Ret::default()
        });
        e.register(SOUND_WORD, |e, a| ret(e.mem.u32(a[0] + 0xb8)));
        e.register(SET_SOUND_WORD, |e, a| {
            e.mem.set_u32(a[0] + 0xb8, a[1]);
            Ret::default()
        });
        let this = light(&mut e);
        let other = light(&mut e);
        for i in 0..6 {
            e.mem.set_u32(other + 0x9c + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(other + 0xb4, 1.5f32.to_bits());
        e.mem.set_u32(other + 0xb8, 0x777);
        let (_, log) = run(&mut e, 0x0050_d060, &args![this, other]);
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        for i in 0..6 {
            assert_eq!(e.mem.u32(this + 0x9c + 4 * i), 0x100 + i);
        }
        assert_eq!(e.mem.u32(this + 0xb4), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(this + 0xb8), 0x777);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let fresh = light(&mut e);
        e.call(0x0050_d060, &args![fresh, other]);
        assert_eq!(e.mem.u32(fresh + 0xb8), 0);
    }

    #[test]
    fn compare_reports_each_difference() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0x200));
        e.register(FADE_VALUE, |e, a| Ret {
            st0: x87_load32(e.mem.u32(a[0] + 0xb4)),
            ..Ret::default()
        });
        e.register(SOUND_WORD, |e, a| ret(e.mem.u32(a[0] + 0xb8)));
        e.register(MEMCMP, |e, a| {
            let differ = (0..a[2]).any(|i| e.mem.u8(a[0] + i) != e.mem.u8(a[1] + i));
            ret(u32::from(differ))
        });
        let pair = |e: &mut Engine, change: &dyn Fn(&mut Engine, u32)| {
            let this = light(e);
            let other = light(e);
            for p in [this, other] {
                e.mem.set_u32(p + 0x9c, 3);
                e.mem.set_u32(p + 0xb4, 2.0f32.to_bits());
                e.mem.set_u32(p + 0xb8, 0x55);
            }
            change(e, other);
            e.call(0x0050_d120, &args![this, other]).bool()
        };
        assert!(!pair(&mut e, &|_, _| {}));
        assert!(pair(&mut e, &|e, o| e.mem.set_u32(o + 0xb8, 0x56)));
        assert!(pair(&mut e, &|e, o| e.mem.set_u32(o + 0xb0, 1)));
        assert!(pair(&mut e, &|e, o| e
            .mem
            .set_u32(o + 0xb4, 2.5f32.to_bits())));
        assert!(pair(&mut e, &|e, o| e
            .mem
            .set_u32(o + 0xb4, f32::NAN.to_bits())));
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(pair(&mut e, &|_, _| {}));
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(pair(&mut e, &|_, _| {}));
    }

    #[test]
    fn stop_particles_walks_nodes_and_stops_properties() {
        let mut e = Engine::new();
        // A node is [vtable, count, children...]; slot +0xc answers whether
        // it is a node (its count word is not 0xffff_ffff).
        e.register(CHILD_COUNT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 8 + 4 * a[1])));
        e.register(0x0f45_000c, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 4) != 0xffff_ffff))
        });
        e.register(GET_PROPERTY, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(PROPERTY_CALL, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 0xc, 0x0f45_000c);
        let object = |e: &mut Engine, words: &[u32]| {
            let p = e.mem.alloc(4 + 4 * words.len() as u32);
            e.mem.set_u32(p, vtable);
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(p + 4 + 4 * i as u32, *w);
            }
            p
        };
        let leaf = object(&mut e, &[0xffff_ffff, 0x6100]);
        let bare = object(&mut e, &[0xffff_ffff, 0]);
        let inner = object(&mut e, &[1, leaf]);
        let root = object(&mut e, &[4, inner, 0, bare, leaf]);
        let (_, log) = run(&mut e, 0x0050_d640, &args![root]);
        assert_eq!(
            only(&log, PROPERTY_CALL),
            vec![
                vec![0x6100, PROPERTY_ARGUMENT],
                vec![0x6100, PROPERTY_ARGUMENT]
            ]
        );
        assert_eq!(only(&log, GET_PROPERTY).len(), 3);
        assert_eq!(only(&log, 0x0050_d640).len(), 1);
    }

    #[test]
    fn flag_bit_zero_follows_the_byte() {
        let mut e = Engine::new();
        let this = light(&mut e);
        e.mem.set_u32(this + 0xa8, 0x10);
        e.call(0x0050_ddb0, &args![this, 0x100u32 + 1]);
        assert_eq!(e.mem.u32(this + 0xa8), 0x11);
        e.call(0x0050_ddb0, &args![this, 0x100u32]);
        assert_eq!(e.mem.u32(this + 0xa8), 0x10);
    }
}
