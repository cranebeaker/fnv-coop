//! `(unplaced)/run_0065c2c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Face-gen model helpers: the EGT texture data of a model (`+0xC`, a
//! record of a file name `BSStringT` and the loaded data pointer at `+8`, as
//! the Xbox PDB's `BSFaceGenModelTextureData`), a class tag set up and torn
//! down like `BSFaceGenAnimationData`'s, and `std::vector` constructors,
//! destructors and accessors. The engine map names none of them; the
//! classes are described from the code, not confirmed.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory-context guard's constructor (`00404eb0`, four words: an id,
/// a flag, a source file name, a line) and destructor (`00404ee0`).
const CONTEXT_GUARD_ENTER: u32 = 0x0040_4eb0;
const CONTEXT_GUARD_LEAVE: u32 = 0x0040_4ee0;
/// The source-file string the EGT load passes to the guard
/// (`"D:\_Fallout3\Platforms\Common\Code\Fallout Share..."`).
const EGT_SOURCE_FILE: u32 = 0x0106_5b00;
/// The string length of a `BSStringT` (`004048e0`).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// The first word of an object (`00559450`, here the `BSStringT`'s text).
const FIRST_WORD: u32 = 0x0055_9450;
/// Loads EGT data from the named file (`00659780`, cdecl, one word).
const LOAD_EGT_DATA: u32 = 0x0065_9780;
/// Deleting destructors (`this`, flags) of the loaded EGT data
/// (`00659ab0`) and of the texture-data record (`00659c40`).
const DELETE_EGT_DATA: u32 = 0x0065_9ab0;
const DELETE_TEXTURE_DATA: u32 = 0x0065_9c40;
/// `std::vector` helpers for 0x58-byte elements: the element count
/// (`0065cea0`) and the address of element `i` (`0065cec0`), and the byte
/// size of a vector (`0065cc50`).
const VECTOR_58_SIZE: u32 = 0x0065_cea0;
const VECTOR_58_AT: u32 = 0x0065_cec0;
const VECTOR_BYTE_SIZE: u32 = 0x0065_cc50;
/// `_invalid_parameter_noinfo` style failure (`00ec7c56`, no arguments).
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// The tag `0065c570` makes and `0065c5f0` frees, and the string it is made
/// from (`"FMD"`).
const MODEL_TAG: u32 = 0x011d_5b50;
const MODEL_TAG_TEXT: u32 = 0x0106_5c8c;

layout! {
    /// A face-gen model as the code here uses it (the Xbox PDB's
    /// `BSFaceGenModel`, 0x10 bytes, has `pTextureData` at the same offset).
    pub struct FaceGenModel: 0x10 {
        /// `pTextureData` (Xbox PDB `BSFaceGenModel`):
        /// `BSFaceGenModelTextureData*`.
        0x0C pTextureData: u32,
    }

    /// The EGT texture-data record (Xbox PDB `BSFaceGenModelTextureData`,
    /// 0x0C bytes).
    pub struct FaceGenModelTextureData: 0x0C {
        /// `EGTFilename` (Xbox PDB): a `BSStringT`.
        0x00 EGTFilename: u32,
        /// `pEGTData` (Xbox PDB): `EGTData*`.
        0x08 pEGTData: u32,
    }

    /// A `std::vector` as these helpers use it (first element at `+0xC`,
    /// one past the last at `+0x10`).
    pub struct StdVector: 0x18 {
        0x0C first: u32,
        0x10 last: u32,
    }
}

// Translated from 0065c2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065c2c0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Loads a model's EGT data on demand (the Xbox PDB's
/// `BSFaceGenModel::LoadEGTData` by its body; unconfirmed): under a
/// memory-context guard (`00404eb0` with `0x39, 1, <source file>, 0x538`),
/// false without texture data; when the data isn't loaded yet, false for an
/// empty file name, else it is loaded from the name (`00659780`). True
/// when the data is (now) there.
pub fn fn_0065c2c0(e: &mut Engine, this: Ptr<FaceGenModel>) -> bool {
    e.with_stack(4, |e, guard| {
        e.call(
            CONTEXT_GUARD_ENTER,
            &args![guard, 0x39u32, 1u32, EGT_SOURCE_FILE, 0x538u32],
        );
        let texture: Ptr<FaceGenModelTextureData> =
            Ptr::new(e.get(this, FaceGenModel::pTextureData));
        let loaded = if texture.is_null() {
            false
        } else if e.get(texture, FaceGenModelTextureData::pEGTData) == 0 {
            if e.call(STRING_LENGTH, &args![texture]).u32() == 0 {
                false
            } else {
                let name = e.call(FIRST_WORD, &args![texture]).u32();
                let data = e.call(LOAD_EGT_DATA, &args![name]).u32();
                let texture: Ptr<FaceGenModelTextureData> =
                    Ptr::new(e.get(this, FaceGenModel::pTextureData));
                e.set(texture, FaceGenModelTextureData::pEGTData, data);
                true
            }
        } else {
            true
        };
        e.call(CONTEXT_GUARD_LEAVE, &args![guard]);
        loaded
    })
}

// Translated from 0065c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unloads a model's EGT data (the Xbox PDB's
/// `BSFaceGenModel::UnloadEGTData` by its body; unconfirmed): when texture
/// data and loaded data exist, deletes the data (`00659ab0`, flag 1) and
/// clears the pointer.
pub fn fn_0065c3a0(e: &mut Engine, this: Ptr<FaceGenModel>) {
    let texture: Ptr<FaceGenModelTextureData> = Ptr::new(e.get(this, FaceGenModel::pTextureData));
    if texture.is_null() || e.get(texture, FaceGenModelTextureData::pEGTData) == 0 {
        return;
    }
    let data = e.get(texture, FaceGenModelTextureData::pEGTData);
    if data != 0 {
        e.call(DELETE_EGT_DATA, &args![data, 1u32]);
    }
    let texture: Ptr<FaceGenModelTextureData> = Ptr::new(e.get(this, FaceGenModel::pTextureData));
    e.set(texture, FaceGenModelTextureData::pEGTData, 0u32);
}

// Translated from 0065c400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a model's texture data (the Xbox PDB's
/// `BSFaceGenModel::KillTextureData` by its body; unconfirmed): when it
/// exists, deletes the loaded EGT data (`00659ab0`, flag 1) if any and
/// clears it, then deletes the record (`00659c40`, flag 1) and clears
/// `pTextureData`.
pub fn fn_0065c400(e: &mut Engine, this: Ptr<FaceGenModel>) {
    let texture: Ptr<FaceGenModelTextureData> = Ptr::new(e.get(this, FaceGenModel::pTextureData));
    if texture.is_null() {
        return;
    }
    let data = e.get(texture, FaceGenModelTextureData::pEGTData);
    if data != 0 {
        e.call(DELETE_EGT_DATA, &args![data, 1u32]);
    }
    let texture: Ptr<FaceGenModelTextureData> = Ptr::new(e.get(this, FaceGenModel::pTextureData));
    e.set(texture, FaceGenModelTextureData::pEGTData, 0u32);
    let texture = e.get(this, FaceGenModel::pTextureData);
    if texture != 0 {
        e.call(DELETE_TEXTURE_DATA, &args![texture, 1u32]);
    }
    e.set(this, FaceGenModel::pTextureData, 0u32);
}

/// The address of record vector `index` (at `+4` and `+0x1C`) of a model's
/// loaded EGT data, read through the model each time as the code does.
fn egt_record_vector(e: &mut Engine, this: Ptr<FaceGenModel>, index: u32) -> u32 {
    let texture: Ptr<FaceGenModelTextureData> = Ptr::new(e.get(this, FaceGenModel::pTextureData));
    let data = e.get(texture, FaceGenModelTextureData::pEGTData);
    data.wrapping_add(index * 0x18).wrapping_add(4)
}

// Translated from 0065c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte size of a model's loaded EGT data (the Xbox PDB's
/// `BSFaceGenModel::GetEGTDataSize` by its body; unconfirmed): 0 when
/// nothing is loaded; else 0x34, plus for each of the data's two vectors of
/// 0x58-byte records (at `+4` and `+0x1C`) 0x58 per record and three times
/// the byte size of each record's vector at `+0x10` (32-bit wrapping sums).
pub fn fn_0065c490(e: &mut Engine, this: Ptr<FaceGenModel>) -> u32 {
    let mut size = 0u32;
    let texture: Ptr<FaceGenModelTextureData> = Ptr::new(e.get(this, FaceGenModel::pTextureData));
    if texture.is_null() || e.get(texture, FaceGenModelTextureData::pEGTData) == 0 {
        return size;
    }
    size = 0x34;
    for i in 0..2u32 {
        let vector = egt_record_vector(e, this, i);
        let count = e.call(VECTOR_58_SIZE, &args![vector]).u32();
        size = count.wrapping_mul(0x58).wrapping_add(size);
        let mut j = 0u32;
        loop {
            let vector = egt_record_vector(e, this, i);
            if j >= e.call(VECTOR_58_SIZE, &args![vector]).u32() {
                break;
            }
            let vector = egt_record_vector(e, this, i);
            let record = e.call(VECTOR_58_AT, &args![vector, j]).u32();
            let bytes = e
                .call(VECTOR_BYTE_SIZE, &args![record.wrapping_add(0x10)])
                .u32();
            size = bytes.wrapping_mul(3).wrapping_add(size);
            j += 1;
        }
    }
    size
}

// Translated from 0065c570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065c570 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Makes the class tag at `011d5b50` (the Xbox PDB's
/// `BSFaceGenModelExtraData::InitSDM` by its body and string; unconfirmed):
/// allocates 4 bytes and, when that succeeds, constructs them with
/// `00438170` from `"FMD"`; stores the result (or null).
pub fn fn_0065c570(e: &mut Engine) {
    let block = e.call(0x0040_1000, &args![4u32]).u32();
    let tag = if block == 0 {
        0
    } else {
        e.call(0x0043_8170, &args![block, MODEL_TAG_TEXT]).u32()
    };
    e.set_global(MODEL_TAG, tag);
}

// Translated from 0065c5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the class tag at `011d5b50` (the counterpart of `0065c570`):
/// deletes it when set (`004b9900` with flag 1) and clears it.
pub fn fn_0065c5f0(e: &mut Engine) {
    let tag: u32 = e.global(MODEL_TAG);
    if tag != 0 {
        e.call(0x004b_9900, &args![tag, 1u32]);
    }
    e.set_global(MODEL_TAG, 0u32);
}

/// The shared body of the two vector constructors: an allocator made on the
/// stack in the argument slot (`006815c0` on the slot's address, which
/// returns it), the base constructed with that word (`0065d3a0`; the slot
/// holds what the code pushed, ECX, which is `this`), then the storage set
/// up by `init` with 0. Returns `this`.
fn construct_vector(e: &mut Engine, this: Ptr<StdVector>, init: u32) -> Ptr<StdVector> {
    let allocator = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), this.addr());
        e.call(0x0068_15c0, &args![slot]);
        e.mem.u32(slot.addr())
    });
    e.call(0x0065_d3a0, &args![this, allocator]);
    e.call(init, &args![this, 0u32]);
    this
}

// Translated from 0065c630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065c630 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// A `std::vector` constructor (empty; element type unconfirmed): see
/// [`construct_vector`], with `0065cfd0` setting up the storage.
pub fn fn_0065c630(e: &mut Engine, this: Ptr<StdVector>) -> Ptr<StdVector> {
    construct_vector(e, this, 0x0065_cfd0)
}

// Translated from 0065c6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065c6a0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// A `std::vector` destructor (the counterpart of `0065c630`): frees the
/// storage (`0065d050`), then the base (`0065cfb0`).
pub fn fn_0065c6a0(e: &mut Engine, this: Ptr<StdVector>) {
    e.call(0x0065_d050, &args![this]);
    e.call(0x0065_cfb0, &args![this]);
}

// Translated from 0065c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector::size` for 12-byte elements: `(last - first) / 12`, a
/// signed division (`IDIV`, truncating toward zero).
pub fn fn_0065c700(e: &mut Engine, this: Ptr<StdVector>) -> i32 {
    let first = e.get(this, StdVector::first);
    let last = e.get(this, StdVector::last);
    (last.wrapping_sub(first) as i32) / 12
}

// Translated from 0065c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector` element access for 12-byte elements: reports an invalid
/// parameter (`00ec7c56`) when `index` is not below the size (unsigned
/// compare), then returns `first + index * 12` either way.
pub fn fn_0065c720(e: &mut Engine, this: Ptr<StdVector>, index: u32) -> u32 {
    let size = fn_0065c700(e, this) as u32;
    if index >= size {
        e.call(INVALID_PARAMETER, &[]);
    }
    index
        .wrapping_mul(12)
        .wrapping_add(e.get(this, StdVector::first))
}

// Translated from 0065c750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065c750 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// A `std::vector` constructor (empty; `BSFaceGenImage` uses it for its
/// `UncompressedImage`): see [`construct_vector`], with `0065d0d0` setting
/// up the storage.
pub fn fn_0065c750(e: &mut Engine, this: Ptr<StdVector>) -> Ptr<StdVector> {
    construct_vector(e, this, 0x0065_d0d0)
}

// Translated from 0065c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065c7c0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// A `std::vector` destructor (the counterpart of `0065c750`): frees the
/// storage (`0065d150`), then the base (`0065cfb0`).
pub fn fn_0065c7c0(e: &mut Engine, this: Ptr<StdVector>) {
    e.call(0x0065_d150, &args![this]);
    e.call(0x0065_cfb0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_c2c0, fn_0065c2c0(Ptr<FaceGenModel>) -> bool),
        entry!(0x0065_c3a0, fn_0065c3a0(Ptr<FaceGenModel>)),
        entry!(0x0065_c400, fn_0065c400(Ptr<FaceGenModel>)),
        entry!(0x0065_c490, fn_0065c490(Ptr<FaceGenModel>) -> u32),
        entry!(0x0065_c570, fn_0065c570()),
        entry!(0x0065_c5f0, fn_0065c5f0()),
        entry!(0x0065_c630, fn_0065c630(Ptr<StdVector>) -> Ptr<StdVector>),
        entry!(0x0065_c6a0, fn_0065c6a0(Ptr<StdVector>)),
        entry!(0x0065_c700, fn_0065c700(Ptr<StdVector>) -> i32),
        entry!(0x0065_c720, fn_0065c720(Ptr<StdVector>, u32) -> u32),
        entry!(0x0065_c750, fn_0065c750(Ptr<StdVector>) -> Ptr<StdVector>),
        entry!(0x0065_c7c0, fn_0065c7c0(Ptr<StdVector>)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn model_with_texture(e: &mut Engine, data: u32) -> (Ptr<FaceGenModel>, u32) {
        let model: Ptr<FaceGenModel> = e.new_object();
        let texture = e.mem.alloc(0xc);
        e.mem.set_u32(texture + 8, data);
        e.mem.set_u32(model.addr() + 0xc, texture);
        (model, texture)
    }

    #[test]
    fn load_egt_data_loads_once_from_the_file_name() {
        let mut e = Engine::new();
        e.register(CONTEXT_GUARD_ENTER, |_, a| a[0].into_ret());
        e.register(CONTEXT_GUARD_LEAVE, |_, _| Ret::default());
        e.register(STRING_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LOAD_EGT_DATA, |_, a| (a[0] + 0x100).into_ret());
        let (model, texture) = model_with_texture(&mut e, 0);
        e.call_log = Some(vec![]);
        // Empty name: false.
        assert!(!e.call(0x0065_c2c0, &args![model]).bool());
        e.mem.set_u32(texture, 0x5000);
        e.mem.set_u32(texture + 4, 3);
        assert!(e.call(0x0065_c2c0, &args![model]).bool());
        assert_eq!(e.mem.u32(texture + 8), 0x5100);
        assert!(e.call(0x0065_c2c0, &args![model]).bool());
        assert_eq!(calls_to(&e, LOAD_EGT_DATA), vec![vec![0x5000]]);
        let guards = calls_to(&e, CONTEXT_GUARD_ENTER);
        assert_eq!(guards.len(), 3);
        assert_eq!(guards[0][1..], [0x39, 1, EGT_SOURCE_FILE, 0x538]);
        assert_eq!(calls_to(&e, CONTEXT_GUARD_LEAVE).len(), 3);
        let model: Ptr<FaceGenModel> = e.new_object();
        assert!(!e.call(0x0065_c2c0, &args![model]).bool());
    }

    #[test]
    fn unload_egt_data_deletes_loaded_data() {
        let mut e = Engine::new();
        e.register(DELETE_EGT_DATA, |_, _| Ret::default());
        let (model, texture) = model_with_texture(&mut e, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0065_c3a0, &args![model]);
        e.call(0x0065_c3a0, &args![model]);
        assert_eq!(calls_to(&e, DELETE_EGT_DATA), vec![vec![0x7000, 1]]);
        assert_eq!(e.mem.u32(texture + 8), 0);
    }

    #[test]
    fn kill_texture_data_deletes_data_and_record() {
        let mut e = Engine::new();
        e.register(DELETE_EGT_DATA, |_, _| Ret::default());
        e.register(DELETE_TEXTURE_DATA, |_, _| Ret::default());
        let (model, texture) = model_with_texture(&mut e, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0065_c400, &args![model]);
        e.call(0x0065_c400, &args![model]);
        assert_eq!(calls_to(&e, DELETE_EGT_DATA), vec![vec![0x7000, 1]]);
        assert_eq!(calls_to(&e, DELETE_TEXTURE_DATA), vec![vec![texture, 1]]);
        assert_eq!(e.mem.u32(model.addr() + 0xc), 0);
        // Without loaded data only the record goes.
        let (model, texture) = model_with_texture(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_c400, &args![model]);
        assert!(calls_to(&e, DELETE_EGT_DATA).is_empty());
        assert_eq!(calls_to(&e, DELETE_TEXTURE_DATA), vec![vec![texture, 1]]);
    }

    #[test]
    fn egt_data_size_counts_records_and_their_vectors() {
        let mut e = Engine::new();
        // The two record vectors hold 2 and 1 records; each record's vector
        // is 0x10 bytes.
        e.register(VECTOR_58_SIZE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(VECTOR_58_AT, |_, a| (0x9000 + a[1] * 0x58).into_ret());
        e.register(VECTOR_BYTE_SIZE, |_, _| 0x10u32.into_ret());
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 4, 2);
        e.mem.set_u32(data + 0x1c, 1);
        let (model, _) = model_with_texture(&mut e, data);
        e.call_log = Some(vec![]);
        let size = e.call(0x0065_c490, &args![model]).u32();
        assert_eq!(size, 0x34 + 3 * 0x58 + 3 * 3 * 0x10);
        assert_eq!(
            calls_to(&e, VECTOR_BYTE_SIZE),
            vec![vec![0x9010], vec![0x9068], vec![0x9010]]
        );
        let (model, _) = model_with_texture(&mut e, 0);
        assert_eq!(e.call(0x0065_c490, &args![model]).u32(), 0);
    }

    #[test]
    fn model_tag_is_made_and_freed() {
        let mut e = Engine::new();
        e.map(0x011d_5000, 0x1000);
        e.register(0x0040_1000, |_, _| 0x4000u32.into_ret());
        e.register(0x0043_8170, |_, a| {
            assert_eq!(a[1], MODEL_TAG_TEXT);
            a[0].into_ret()
        });
        e.register(0x004b_9900, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_c570, &[]);
        assert_eq!(e.global::<u32>(MODEL_TAG), 0x4000);
        e.call(0x0065_c5f0, &[]);
        e.call(0x0065_c5f0, &[]);
        assert_eq!(calls_to(&e, 0x004b_9900), vec![vec![0x4000, 1]]);
        assert_eq!(e.global::<u32>(MODEL_TAG), 0);
        e.register(0x0040_1000, |_, _| 0u32.into_ret());
        e.call(0x0065_c570, &[]);
        assert_eq!(e.global::<u32>(MODEL_TAG), 0);
        assert_eq!(calls_to(&e, 0x0043_8170).len(), 1);
    }

    #[test]
    fn vector_constructors_and_destructors_call_their_parts() {
        let mut e = Engine::new();
        for target in [
            0x0068_15c0,
            0x0065_d3a0,
            0x0065_cfd0,
            0x0065_d0d0,
            0x0065_d050,
            0x0065_d150,
            0x0065_cfb0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        let vector: Ptr<StdVector> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_c630, &args![vector]).u32(), vector.addr());
        assert_eq!(e.call(0x0065_c750, &args![vector]).u32(), vector.addr());
        e.call(0x0065_c6a0, &args![vector]);
        e.call(0x0065_c7c0, &args![vector]);
        let v = vector.addr();
        assert_eq!(calls_to(&e, 0x0065_d3a0), vec![vec![v, v], vec![v, v]]);
        assert_eq!(calls_to(&e, 0x0065_cfd0), vec![vec![v, 0]]);
        assert_eq!(calls_to(&e, 0x0065_d0d0), vec![vec![v, 0]]);
        assert_eq!(calls_to(&e, 0x0065_d050), vec![vec![v]]);
        assert_eq!(calls_to(&e, 0x0065_d150), vec![vec![v]]);
        assert_eq!(calls_to(&e, 0x0065_cfb0), vec![vec![v], vec![v]]);
        assert_eq!(calls_to(&e, 0x0068_15c0).len(), 2);
    }

    #[test]
    fn vector_size_and_access_use_twelve_byte_elements() {
        let mut e = Engine::new();
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        let vector: Ptr<StdVector> = e.new_object();
        e.mem.set_u32(vector.addr() + 0xc, 0x1000);
        e.mem.set_u32(vector.addr() + 0x10, 0x1000 + 3 * 12);
        assert_eq!(e.call(0x0065_c700, &args![vector]).i32(), 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_c720, &args![vector, 2u32]).u32(), 0x1018);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        assert_eq!(e.call(0x0065_c720, &args![vector, 3u32]).u32(), 0x1024);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
        // A negative byte span divides toward zero.
        e.mem.set_u32(vector.addr() + 0x10, 0x1000 - 13);
        assert_eq!(e.call(0x0065_c700, &args![vector]).i32(), -1);
    }
}
