//! `BSMain/run_00c61610` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds three groups of functions, all confirmed from the exe:
//!
//! - the `BSWArray` tag helpers (`00c61610`..`00c61830`): the pooled string
//!   `"BSW"` kept in the 4-byte cell the global at `012043a0` points to, the
//!   static RTTI at `012043a4` (`BSWArray`'s: the class whose vtable
//!   `00c613a0` stores has the RTTI name `BSWArray`), and the helpers that
//!   look a `BSWArray` extra data up by that tag;
//! - the `PArrayPoint` creator (`00c61850`) and the tag cell for the pooled
//!   string `"PAP"` (`012043ac`, `00c618f0`, `00c61990`); the vtable stored by
//!   `00c61850` has the RTTI name `PArrayPoint`;
//! - the `BSPSysArrayEmitter` constructor (`00c61a10`); the vtable it stores
//!   has the RTTI name `BSPSysArrayEmitter`.
//!
//! The engine map has no names for any of them, so the functions are named
//! by address (`fn_<addr>`) unless the body settles the role.
//!
//! x87 note: `00c61a10` copies ten `float` arguments through `FLD`/`FSTP`,
//! which turns a signalling NaN into a quiet one (the quiet bit, bit 22, is
//! set) and leaves every other `float` bit-for-bit; [`through_x87`] does the
//! same on the raw bits.

#[allow(unused_imports)]
use crate::prelude::*;

/// `012043a0`: pointer to the 4-byte cell holding the pooled `"BSW"` string
/// handle (the tag of `BSWArray` extra data).
const BSW_TAG_CELL: u32 = 0x0120_43a0;
/// `012043a4`: `BSWArray`'s static RTTI (a node with the base class's RTTI
/// pointer in its second word).
const BSW_ARRAY_RTTI: u32 = 0x0120_43a4;
/// `012043ac`: pointer to the 4-byte cell holding the pooled `"PAP"` string
/// handle (`PArrayPoint`'s tag).
const PAP_TAG_CELL: u32 = 0x0120_43ac;
/// `011f49e0`: a `NiFixedString` the `PArrayPoint` creator names its object
/// with (the address of the handle, not a cell pointer).
const PAP_NAME_STRING: u32 = 0x011f_49e0;
/// `"BSW"` (a string literal in the exe).
const BSW_TEXT: u32 = 0x010c_3610;
/// `"PAP"` (a string literal in the exe).
const PAP_TEXT: u32 = 0x010c_36b0;
/// The pooled-string handle the tag cells are compared with before the
/// string's reference count (at `handle - 8`) is released (`0109b220`).
const NO_STRING_HANDLE: u32 = 0x0109_b220;
/// `PArrayPoint`'s vtable (RTTI name `PArrayPoint`).
const PARRAYPOINT_VTABLE: u32 = 0x010c_361c;
/// `BSPSysArrayEmitter`'s vtable (RTTI name `BSPSysArrayEmitter`).
const BSPSYSARRAYEMITTER_VTABLE: u32 = 0x010c_36c4;
/// Vtable stored into the emitter's embedded array at +0x5c (RTTI name
/// `NiTPrimitiveArray<NiPointer<NiAVObject>>`).
const EMITTER_OBJECT_ARRAY_VTABLE: u32 = 0x010c_36b8;

/// `00a5b950` (engine map: `NiMain/niobjectnet.cpp`): `this + 8` takes the
/// pooled string handle stored at the address passed, adjusting the strings'
/// reference counts (at `handle - 8`).
const ASSIGN_NAME_HANDLE: u32 = 0x00a5_b950;
/// `00a5bdd0` (engine map: `NiObjectNET::GetExtraData`): the extra data of
/// `this` whose name handle equals the one stored at the address passed, or
/// null.
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// `00a5b690`: interns a C string in the pooled-string table and returns
/// its handle (reference counted).
const INTERN_STRING: u32 = 0x00a5_b690;
/// `00aa3e40` (`MemoryManager::Allocate`, `ECX` = pool).
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa4060` (`MemoryManager::Deallocate`, `ECX` = pool).
const POOL_DEALLOCATE: u32 = 0x00aa_4060;
/// The pool the tag cells are allocated from (`ECX` of `00aa3e40`).
const TAG_CELL_POOL: u32 = 0x011f_6238;
/// `operator new` (`00aa13e0`, cdecl, one argument).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `operator delete` with a size (`00aa1460`, cdecl: pointer, size).
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00c613a0`: the destructor body of the `BSWArray` the scalar deleting
/// destructor `00c617e0` runs.
const BSW_ARRAY_DESTRUCTOR: u32 = 0x00c6_13a0;
/// `00a7b440`: the base constructor of the `PArrayPoint` creator (takes the
/// address of a tag handle).
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b440;
/// `00a7b330`: copies the members of one extra data into another (target,
/// cloning process).
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
/// `00c31bd0`: the base constructor of the emitter (`NiPSysVolumeEmitter`
/// in the engine map).
const VOLUME_EMITTER_CONSTRUCTOR: u32 = 0x00c3_1bd0;
/// `00c63260`: constructor of the emitter's embedded array at +0x5c.
const EMITTER_ARRAY_CONSTRUCTOR: u32 = 0x00c6_3260;
/// `00c62140` (`BSPSysArrayEmitter::SetupArray` in the Xbox PDB).
const EMITTER_SETUP_ARRAY: u32 = 0x00c6_2140;

/// A `float` argument copied with `FLD m32` / `FSTP m32`, as raw bits: the
/// x87 load quiets a signalling NaN (sets bit 22) and changes nothing else.
fn through_x87(value: f32) -> u32 {
    let bits = value.to_bits();
    let is_nan = bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0;
    if is_nan {
        bits | 0x0040_0000
    } else {
        bits
    }
}

/// Releases one reference to a pooled string handle: nothing for the
/// no-string handle, otherwise `InterlockedDecrement(handle - 8)`.
fn release_tag_handle(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(NO_STRING_HANDLE) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// Frees a tag cell (global `global`): releases its string handle, hands
/// the cell back to the pool and clears the global. `00c61760` and
/// `00c61990` are this for the two tags.
fn destroy_tag_cell(e: &mut Engine, global: u32) {
    let cell = e.global::<u32>(global);
    if cell != 0 {
        let handle = e.mem.u32(cell);
        release_tag_handle(e, handle);
        // `MOV ECX,1; AND ECX,1; JZ`: the deleting-destructor flag is a
        // compile-time 1, so the cell is always freed.
        e.call(POOL_DEALLOCATE, &args![TAG_CELL_POOL, cell]);
    }
    e.set_global::<u32>(global, 0);
}

// Translated from 00c61610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: names the object `this` with the `"BSW"`
/// tag, by handing the tag cell to `00a5b950`.
pub fn fn_00c61610(e: &mut Engine, this: Ptr) {
    let cell = e.global::<u32>(BSW_TAG_CELL);
    e.call(ASSIGN_NAME_HANDLE, &args![this, cell]);
}

// Translated from 00c61630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: finds the extra data of `object` named by the
/// `"BSW"` tag (`GetExtraData`) and returns it only when its RTTI (virtual
/// slot +8) is `BSWArray`'s (`012043a4`) or derives from it, else null.
/// Null `object` gives null.
pub fn fn_00c61630(e: &mut Engine, object: Ptr) -> Ptr {
    if object.is_null() {
        return Ptr::NULL;
    }
    let tag = e.global::<u32>(BSW_TAG_CELL);
    let extra = e.call(GET_EXTRA_DATA, &args![object, tag]).u32();
    if extra == 0 {
        return Ptr::NULL;
    }
    let mut rtti = e.vcall(extra, 8, &[]).u32();
    while rtti != 0 {
        if rtti == BSW_ARRAY_RTTI {
            return Ptr::new(extra);
        }
        // The next RTTI node is the base class's (second word).
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    Ptr::NULL
}

// Translated from 00c616c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: creates the `"BSW"` tag cell. Allocates four
/// bytes from the pool at `011f6238`, stores the interned `"BSW"` handle in
/// it (the compiler left a dead `else` for a null string literal) and
/// publishes the cell in `012043a0`; a failed allocation publishes null.
// Translation incomplete: 00c616c0 - SEH exception unwinding is unsupported.
pub fn fn_00c616c0(e: &mut Engine) {
    let cell = e.call(POOL_ALLOCATE, &args![TAG_CELL_POOL, 4u32]).u32();
    if cell != 0 {
        let handle = e.call(INTERN_STRING, &args![BSW_TEXT]).u32();
        e.mem.set_u32(cell, handle);
    }
    e.set_global::<u32>(BSW_TAG_CELL, cell);
}

// Translated from 00c61760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destroys the `"BSW"` tag cell and clears
/// `012043a0`.
pub fn fn_00c61760(e: &mut Engine) {
    destroy_tag_cell(e, BSW_TAG_CELL);
}

// Translated from 00c617e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BSWArray` (not named in the engine map; the
/// class is the one whose vtable the body `00c613a0` stores): runs
/// `00c613a0` and, when bit 0 of `flags` is set, frees the 0x14 bytes.
/// Returns `this`.
pub fn bs_w_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BSW_ARRAY_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00c61810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns the address of `BSWArray`'s static
/// RTTI (`012043a4`).
pub fn fn_00c61810(_e: &mut Engine) -> u32 {
    BSW_ARRAY_RTTI
}

// Translated from 00c61820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns `this`; the one stack word is never
/// read.
pub fn fn_00c61820(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

// Translated from 00c61830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: forwards `(target, cloning_process)` to the
/// extra-data member copy `00a7b330` with `this` as the source.
pub fn fn_00c61830(e: &mut Engine, this: Ptr, target: Ptr, cloning_process: u32) {
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, target, cloning_process],
    );
}

// Translated from 00c61850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: creates a `PArrayPoint` (0xc bytes, `operator
/// new`), builds its base with the `"PAP"` tag cell's address (`00a7b440`),
/// stores the `PArrayPoint` vtable, names it with the `NiFixedString` at
/// `011f49e0` (`00a5b950`) and returns it. The exe does not test the
/// allocation result before the name assignment, nor does this.
// Translation incomplete: 00c61850 - SEH exception unwinding is unsupported.
pub fn fn_00c61850(e: &mut Engine) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if object != 0 {
        let tag = e.global::<u32>(PAP_TAG_CELL);
        e.call(EXTRA_DATA_CONSTRUCTOR, &args![object, tag]);
        e.mem.set_u32(object, PARRAYPOINT_VTABLE);
    }
    e.call(ASSIGN_NAME_HANDLE, &args![object, PAP_NAME_STRING]);
    Ptr::new(object)
}

// Translated from 00c618f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: creates the `"PAP"` tag cell the way
/// `00c616c0` creates the `"BSW"` one, and publishes it in `012043ac`.
// Translation incomplete: 00c618f0 - SEH exception unwinding is unsupported.
pub fn fn_00c618f0(e: &mut Engine) {
    let cell = e.call(POOL_ALLOCATE, &args![TAG_CELL_POOL, 4u32]).u32();
    if cell != 0 {
        let handle = e.call(INTERN_STRING, &args![PAP_TEXT]).u32();
        e.mem.set_u32(cell, handle);
    }
    e.set_global::<u32>(PAP_TAG_CELL, cell);
}

// Translated from 00c61990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destroys the `"PAP"` tag cell and clears
/// `012043ac`.
pub fn fn_00c61990(e: &mut Engine) {
    destroy_tag_cell(e, PAP_TAG_CELL);
}

// Translated from 00c61a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter` constructor (the engine map has no name; the vtable
/// the body stores has the RTTI name `BSPSysArrayEmitter`). Forwards 16
/// words to the base constructor (`00c31bd0`): the two leading words and
/// `float_14`..`float_28`, `word_2c`..`word_38`, `float_3c`..`float_48`, the
/// ten floats copied through `FLD`/`FSTP` ([`through_x87`]) and the four
/// words copied bit for bit. `array_type` is not passed on; it is stored at
/// +0x58 (`eType` in the Xbox PDB). Then it stores the emitter vtable,
/// constructs the embedded array at +0x5c (`00c63260(0, 1)`), stores that
/// array's vtable and runs `00c62140`. Returns `this`.
// Translation incomplete: 00c61a10 - SEH exception unwinding is unsupported.
#[allow(clippy::too_many_arguments)]
pub fn bs_psys_array_emitter_constructor(
    e: &mut Engine,
    this: Ptr,
    word_08: u32,
    word_0c: u32,
    array_type: u32,
    float_14: f32,
    float_18: f32,
    float_1c: f32,
    float_20: f32,
    float_24: f32,
    float_28: f32,
    word_2c: u32,
    word_30: u32,
    word_34: u32,
    word_38: u32,
    float_3c: f32,
    float_40: f32,
    float_44: f32,
    float_48: f32,
) -> Ptr {
    e.call(
        VOLUME_EMITTER_CONSTRUCTOR,
        &args![
            this,
            word_08,
            word_0c,
            through_x87(float_14),
            through_x87(float_18),
            through_x87(float_1c),
            through_x87(float_20),
            through_x87(float_24),
            through_x87(float_28),
            word_2c,
            word_30,
            word_34,
            word_38,
            through_x87(float_3c),
            through_x87(float_40),
            through_x87(float_44),
            through_x87(float_48),
        ],
    );
    e.mem.set_u32(this.addr(), BSPSYSARRAYEMITTER_VTABLE);
    // BSPSysArrayEmitter::eType (Xbox PDB) +0x58
    e.mem.set_u32(this.addr() + 0x58, array_type);
    // BSPSysArrayEmitter::aObjects (Xbox PDB) +0x5c
    let objects = this.addr() + 0x5c;
    e.call(EMITTER_ARRAY_CONSTRUCTOR, &args![objects, 0u32, 1u32]);
    e.mem.set_u32(objects, EMITTER_OBJECT_ARRAY_VTABLE);
    e.call(EMITTER_SETUP_ARRAY, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c61610, fn_00c61610(Ptr)),
        entry!(0x00c61630, fn_00c61630(Ptr) -> Ptr),
        entry!(0x00c616c0, fn_00c616c0()),
        entry!(0x00c61760, fn_00c61760()),
        entry!(0x00c617e0, bs_w_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c61810, fn_00c61810() -> u32),
        entry!(0x00c61820, fn_00c61820(Ptr, u32) -> Ptr),
        entry!(0x00c61830, fn_00c61830(Ptr, Ptr, u32)),
        entry!(0x00c61850, fn_00c61850() -> Ptr),
        entry!(0x00c618f0, fn_00c618f0()),
        entry!(0x00c61990, fn_00c61990()),
        entry!(
            0x00c61a10,
            bs_psys_array_emitter_constructor(
                Ptr, u32, u32, u32, f32, f32, f32, f32, f32, f32, u32, u32, u32, u32, f32, f32,
                f32, f32
            ) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Maps the tag globals, the no-string handle and the RTTI node.
    fn world() -> Engine {
        let mut e = Engine::new();
        e.map(0x0120_4000, 0x100);
        e.map(NO_STRING_HANDLE, 4);
        e
    }

    fn with_interlocked(e: &mut Engine) {
        let imports = [
            crate::exe::Import {
                slot: 0x00fd_f1c0,
                dll: "KERNEL32.dll".into(),
                name: "InterlockedIncrement".into(),
            },
            crate::exe::Import {
                slot: INTERLOCKED_DECREMENT,
                dll: "KERNEL32.dll".into(),
                name: "InterlockedDecrement".into(),
            },
        ];
        assert_eq!(e.register_imports(&imports), 2);
    }

    #[test]
    fn test_fn_00c61610_names_the_object_with_the_bsw_cell() {
        let mut e = world();
        stub(&mut e, &[ASSIGN_NAME_HANDLE]);
        e.set_global::<u32>(BSW_TAG_CELL, 0x1234_5670);
        e.call_log = Some(vec![]);
        e.call(0x00c61610, &args![0x4000_0000u32]);
        assert_eq!(
            calls_to(&e, ASSIGN_NAME_HANDLE),
            vec![vec![0x4000_0000, 0x1234_5670]]
        );
    }

    #[test]
    fn test_fn_00c61630_finds_the_extra_data_by_rtti() {
        let mut e = world();
        let object = 0x4000_0000u32;
        e.set_global::<u32>(BSW_TAG_CELL, 0x0120_4040);
        // Extra data whose slot +8 is the real `00c61810` (BSWArray's RTTI).
        let extra = e.mem.alloc(8);
        e.put_vtable(0x4100_0000, &[0, 0, 0x00c6_1810]);
        e.mem.set_u32(extra, 0x4100_0000);
        // Extra data whose RTTI derives from BSWArray's.
        let derived = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0);
        e.mem.set_u32(node + 4, BSW_ARRAY_RTTI);
        e.put_vtable(0x4100_1000, &[0, 0, 0x4200_0000]);
        e.mem.set_u32(derived, 0x4100_1000);
        e.register_double(0x4200_0000, move |_, _| Ret {
            eax: node,
            ..Ret::default()
        });
        // Extra data of an unrelated class (no base).
        let other = e.mem.alloc(8);
        let unrelated = e.mem.alloc(8);
        e.put_vtable(0x4100_2000, &[0, 0, 0x4200_0010]);
        e.mem.set_u32(other, 0x4100_2000);
        e.register_double(0x4200_0010, move |_, _| Ret {
            eax: unrelated,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        for (found, expected) in [(0, 0), (extra, extra), (derived, derived), (other, 0)] {
            e.register_double(GET_EXTRA_DATA, move |_, _| Ret {
                eax: found,
                ..Ret::default()
            });
            let r = e.call(0x00c61630, &args![object]).u32();
            assert_eq!(r, expected);
        }
        assert_eq!(
            calls_to(&e, GET_EXTRA_DATA)[0],
            vec![object, 0x0120_4040],
            "the tag cell is the name argument"
        );
        assert_eq!(e.call(0x00c61630, &args![0u32]).u32(), 0, "null object");
        assert_eq!(calls_to(&e, GET_EXTRA_DATA).len(), 4, "no lookup for null");
    }

    #[test]
    fn test_fn_00c616c0_creates_the_bsw_cell() {
        let mut e = world();
        e.register(POOL_ALLOCATE, |e, a| {
            assert_eq!(a, [TAG_CELL_POOL, 4]);
            Ret {
                eax: e.mem.alloc(4),
                ..Ret::default()
            }
        });
        e.register(INTERN_STRING, |_, a| {
            assert_eq!(a, [BSW_TEXT]);
            Ret {
                eax: 0x5000_0008,
                ..Ret::default()
            }
        });
        e.call(0x00c616c0, &args![]);
        let cell = e.global::<u32>(BSW_TAG_CELL);
        assert_ne!(cell, 0);
        assert_eq!(e.mem.u32(cell), 0x5000_0008);
        // A failed allocation publishes null and interns nothing.
        e.register(POOL_ALLOCATE, |_, _| Ret::default());
        e.register(INTERN_STRING, |_, _| panic!("no string without a cell"));
        e.call(0x00c616c0, &args![]);
        assert_eq!(e.global::<u32>(BSW_TAG_CELL), 0);
    }

    #[test]
    fn test_fn_00c61760_destroys_the_bsw_cell() {
        let mut e = world();
        with_interlocked(&mut e);
        stub(&mut e, &[POOL_DEALLOCATE]);
        let cell = e.mem.alloc(4);
        let string = e.mem.alloc(16);
        let handle = string + 8;
        e.mem.set_i32(string, 3);
        e.mem.set_u32(cell, handle);
        e.set_global::<u32>(BSW_TAG_CELL, cell);
        e.call_log = Some(vec![]);
        e.call(0x00c61760, &args![]);
        assert_eq!(e.mem.i32(string), 2, "the string loses one reference");
        assert_eq!(
            calls_to(&e, POOL_DEALLOCATE),
            vec![vec![TAG_CELL_POOL, cell]]
        );
        assert_eq!(e.global::<u32>(BSW_TAG_CELL), 0);
        // The no-string handle keeps its count; a null global frees nothing.
        e.mem.set_u32(cell, 0);
        e.set_global::<u32>(BSW_TAG_CELL, cell);
        e.call_log = Some(vec![]);
        e.call(0x00c61760, &args![]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(calls_to(&e, POOL_DEALLOCATE).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00c61760, &args![]);
        assert!(calls_to(&e, POOL_DEALLOCATE).is_empty());
        assert_eq!(e.global::<u32>(BSW_TAG_CELL), 0);
    }

    #[test]
    fn test_bs_w_array_scalar_deleting_destructor() {
        let mut e = Engine::new();
        stub(&mut e, &[BSW_ARRAY_DESTRUCTOR, OPERATOR_DELETE_SIZED]);
        e.call_log = Some(vec![]);
        let r = e.call(0x00c617e0, &args![0x4000_0000u32, 0u32]);
        assert_eq!(r.u32(), 0x4000_0000, "returns this");
        assert!(calls_to(&e, OPERATOR_DELETE_SIZED).is_empty());
        e.call(0x00c617e0, &args![0x4000_0000u32, 1u32]);
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE_SIZED),
            vec![vec![0x4000_0000, 0x14]]
        );
        assert_eq!(calls_to(&e, BSW_ARRAY_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn test_fn_00c61810_returns_the_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c61810, &args![]).u32(), 0x0120_43a4);
    }

    #[test]
    fn test_fn_00c61820_returns_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c61820, &args![0x77u32, 0x99u32]).u32(), 0x77);
    }

    #[test]
    fn test_fn_00c61830_forwards_to_the_member_copy() {
        let mut e = Engine::new();
        stub(&mut e, &[EXTRA_DATA_COPY_MEMBERS]);
        e.call_log = Some(vec![]);
        e.call(0x00c61830, &args![1u32, 2u32, 3u32]);
        assert_eq!(calls_to(&e, EXTRA_DATA_COPY_MEMBERS), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_fn_00c61850_creates_a_parraypoint() {
        let mut e = world();
        e.set_global::<u32>(PAP_TAG_CELL, 0x0120_4050);
        let block = e.mem.alloc(0xc);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a, [0xc]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        stub(&mut e, &[EXTRA_DATA_CONSTRUCTOR, ASSIGN_NAME_HANDLE]);
        e.call_log = Some(vec![]);
        let r = e.call(0x00c61850, &args![]).u32();
        assert_eq!(r, block);
        assert_eq!(e.mem.u32(block), PARRAYPOINT_VTABLE);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_CONSTRUCTOR),
            vec![vec![block, 0x0120_4050]]
        );
        assert_eq!(
            calls_to(&e, ASSIGN_NAME_HANDLE),
            vec![vec![block, PAP_NAME_STRING]]
        );
    }

    #[test]
    fn test_fn_00c61850_without_memory_still_names_null() {
        let mut e = world();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, _| panic!("no object, no base"));
        stub(&mut e, &[ASSIGN_NAME_HANDLE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c61850, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, ASSIGN_NAME_HANDLE),
            vec![vec![0, PAP_NAME_STRING]]
        );
    }

    #[test]
    fn test_fn_00c618f0_creates_the_pap_cell() {
        let mut e = world();
        e.register(POOL_ALLOCATE, |e, a| {
            assert_eq!(a, [TAG_CELL_POOL, 4]);
            Ret {
                eax: e.mem.alloc(4),
                ..Ret::default()
            }
        });
        e.register(INTERN_STRING, |_, a| {
            assert_eq!(a, [PAP_TEXT]);
            Ret {
                eax: 0x5000_0010,
                ..Ret::default()
            }
        });
        e.call(0x00c618f0, &args![]);
        let cell = e.global::<u32>(PAP_TAG_CELL);
        assert_ne!(cell, 0);
        assert_eq!(e.mem.u32(cell), 0x5000_0010);
        e.register(POOL_ALLOCATE, |_, _| Ret::default());
        e.call(0x00c618f0, &args![]);
        assert_eq!(e.global::<u32>(PAP_TAG_CELL), 0);
    }

    #[test]
    fn test_fn_00c61990_destroys_the_pap_cell() {
        let mut e = world();
        with_interlocked(&mut e);
        stub(&mut e, &[POOL_DEALLOCATE]);
        let cell = e.mem.alloc(4);
        let string = e.mem.alloc(16);
        e.mem.set_i32(string, 1);
        e.mem.set_u32(cell, string + 8);
        e.set_global::<u32>(PAP_TAG_CELL, cell);
        e.call_log = Some(vec![]);
        e.call(0x00c61990, &args![]);
        assert_eq!(e.mem.i32(string), 0);
        assert_eq!(
            calls_to(&e, POOL_DEALLOCATE),
            vec![vec![TAG_CELL_POOL, cell]]
        );
        assert_eq!(e.global::<u32>(PAP_TAG_CELL), 0);
        e.call_log = Some(vec![]);
        e.call(0x00c61990, &args![]);
        assert!(calls_to(&e, POOL_DEALLOCATE).is_empty());
    }

    #[test]
    fn test_through_x87_quiets_only_signalling_nans() {
        assert_eq!(through_x87(f32::from_bits(0x7f80_0001)), 0x7fc0_0001);
        assert_eq!(through_x87(f32::from_bits(0xff80_0001)), 0xffc0_0001);
        assert_eq!(through_x87(f32::from_bits(0x7fc0_1234)), 0x7fc0_1234);
        assert_eq!(through_x87(f32::INFINITY), 0x7f80_0000);
        assert_eq!(through_x87(-0.0), 0x8000_0000);
        assert_eq!(through_x87(f32::from_bits(1)), 1, "denormals are kept");
        assert_eq!(through_x87(1.5), 1.5f32.to_bits());
    }

    #[test]
    fn test_bs_psys_array_emitter_constructor() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                VOLUME_EMITTER_CONSTRUCTOR,
                EMITTER_ARRAY_CONSTRUCTOR,
                EMITTER_SETUP_ARRAY,
            ],
        );
        let this = e.mem.alloc(0x6c);
        let snan = f32::from_bits(0x7f80_0001);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            0x11u32,
            0x22u32,
            7u32,
            1.0f32,
            snan,
            3.0f32,
            4.0f32,
            5.0f32,
            6.0f32,
            0x7f80_0001u32,
            0x2du32,
            0x2eu32,
            0x2fu32,
            -0.0f32,
            f32::from_bits(0x7f80_0002),
            9.0f32,
            f32::from_bits(0xff80_0003)
        ];
        let r = e.call(0x00c61a10, &words);
        assert_eq!(r.u32(), this);
        let base = calls_to(&e, VOLUME_EMITTER_CONSTRUCTOR);
        assert_eq!(
            base,
            vec![vec![
                this,
                0x11,
                0x22,
                1.0f32.to_bits(),
                0x7fc0_0001,
                3.0f32.to_bits(),
                4.0f32.to_bits(),
                5.0f32.to_bits(),
                6.0f32.to_bits(),
                0x7f80_0001,
                0x2d,
                0x2e,
                0x2f,
                0x8000_0000,
                0x7fc0_0002,
                9.0f32.to_bits(),
                0xffc0_0003,
            ]],
            "ten floats are quieted, the four raw words are not"
        );
        assert_eq!(e.mem.u32(this), BSPSYSARRAYEMITTER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x58), 7);
        assert_eq!(e.mem.u32(this + 0x5c), EMITTER_OBJECT_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, EMITTER_ARRAY_CONSTRUCTOR),
            vec![vec![this + 0x5c, 0, 1]]
        );
        assert_eq!(calls_to(&e, EMITTER_SETUP_ARRAY), vec![vec![this]]);
    }
}
