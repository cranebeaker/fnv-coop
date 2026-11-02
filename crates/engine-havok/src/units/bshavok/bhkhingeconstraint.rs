//! `BSHavok/bhkhingeconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `bhkHingeConstraint` is a 0x10-byte `bhkConstraint` (vtable, reference count `+4`, the
//! Havok constraint's cinfo holder `+8`, the cinfo block `+0xC`). A constraint's cinfo holder
//! (`hkHingeConstraintCinfo` in the Xbox PDB) keeps its data block, 0xB0 bytes with the two
//! hinge frames (a pivot, an axle and two perpendicular axes each) from `+0x20`, at holder
//! `+4`.
//!
//! The helpers marked `pub(crate)` are shared by the sibling constraint units. The
//! exception-unwinding frames (`FS:[0]` registrations) of the functions marked
//! `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{
    enter_scope, havok_allocate, push_viewer_string, stream_transfer, CLONE_LOCK, CLONE_UNLOCK,
    HEAP_ALLOCATE, MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE, NI_POINT3_GET_VIEWER_STRING,
    SERIALIZABLE_KILL, VECTOR_TO_POINT,
};

/// `bhkConstraint` base constructor (`00c87c30`), its destructor body (`00c89640`), the copy of
/// the base members into a clone (`00c89730`, `(this, clone, context)`) and the adoption of a
/// new cinfo block (`00c89c80`, `(this, block)`).
pub(crate) const CONSTRAINT_CONSTRUCT: u32 = 0x00c8_7c30;
pub(crate) const CONSTRAINT_DESTRUCT: u32 = 0x00c8_9640;
pub(crate) const CONSTRAINT_COPY_MEMBERS: u32 = 0x00c8_9730;
pub(crate) const CONSTRAINT_ADOPT_CINFO: u32 = 0x00c8_9c80;
/// `bhkConstraintCinfo` base routines: `LoadBinary` (`00c8a230`), `SaveBinary` (`00c8a280`),
/// `CloneData` (`00c8a1b0`, `(this, data, context)`) and `GetViewerStrings` (`00c8a560`).
pub(crate) const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
pub(crate) const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
pub(crate) const CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
pub(crate) const CINFO_GET_VIEWER_STRINGS: u32 = 0x00c8_a560;
/// The base class vtable the constraint constructor installs first, and the live constraint
/// counter it bumps.
pub(crate) const CONSTRAINT_BASE_VTABLE: u32 = 0x0108_febc;
pub(crate) const CONSTRAINT_COUNT: u32 = 0x0126_8188;

/// The hinge constraint's vtable and live counter, its cinfo block's vtable, the data block's
/// constructor (`00cbe7b0`, thiscall on the block) and the data scale routine (`00cbd9e0`,
/// cdecl `(scale, data)`).
const HINGE_VTABLE: u32 = 0x0108_fddc;
const HINGE_COUNT: u32 = 0x0126_8434;
const HINGE_CLASSES: &[(u32, u32)] = &[(HINGE_VTABLE, HINGE_COUNT)];
const HINGE_CINFO_BLOCK_VTABLE: u32 = 0x0108_ff98;
const HINGE_DATA_CONSTRUCT: u32 = 0x00cb_e7b0;
const HINGE_DATA_SCALE: u32 = 0x00cb_d9e0;
/// The swap of the byte order of a hinge frame (`00c8cdb0`, cdecl, the frame's address).
const HINGE_FRAME_SWAP: u32 = 0x00c8_cdb0;
/// Size of the data block.
const DATA_SIZE: u32 = 0xb0;
/// Viewer names of the five vectors listed.
const NAME_PIVOT_IN_A: u32 = 0x010c_839c;
const NAME_PIVOT_IN_B: u32 = 0x010c_8390;
const NAME_AXLE_IN_B: u32 = 0x010c_8388;
const NAME_PERP_2_AXLE_IN_A_1: u32 = 0x010c_8378;
const NAME_PERP_2_AXLE_IN_A_2: u32 = 0x010c_8368;

/// `counter += 1`, as `ADD dword ptr [counter], 1` does.
pub(crate) fn bump(e: &mut Engine, counter: u32) {
    let value = e.mem.u32(counter);
    e.mem.set_u32(counter, value.wrapping_add(1));
}

/// Constructs a constraint object in `block` (when there is one) the way the constraint
/// classes do: the base constructor, the base vtable, a zero at `+0xC` and the base counter,
/// then, for each class from the base down, its `vtable` and live `counter`. Answers `block`.
pub(crate) fn construct_constraint(e: &mut Engine, block: u32, classes: &[(u32, u32)]) -> u32 {
    if block == 0 {
        return 0;
    }
    e.call(CONSTRAINT_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, CONSTRAINT_BASE_VTABLE);
    e.mem.set_u32(block + 0xc, 0);
    bump(e, CONSTRAINT_COUNT);
    for &(vtable, counter) in classes {
        e.mem.set_u32(block, vtable);
        bump(e, counter);
    }
    block
}

/// Allocates a 0x10-byte constraint object from the game heap and constructs it
/// (`construct_constraint`); 0 when the allocation failed.
pub(crate) fn new_constraint(e: &mut Engine, classes: &[(u32, u32)]) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
    construct_constraint(e, block, classes)
}

/// The body of every constraint class's `CreateClone`: with the allocation-scope word at 0x10
/// and the clone lock held, builds a fresh object of the class and has the base class copy
/// `this` into it (`00c89730`). Answers the clone (0 when the allocation failed).
pub(crate) fn clone_constraint(
    e: &mut Engine,
    this: Ptr,
    context: u32,
    classes: &[(u32, u32)],
) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = new_constraint(e, classes);
    e.call(CONSTRAINT_COPY_MEMBERS, &args![this, clone, context]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

/// The body of every constraint class's `CreateCinfo`: with the allocation-scope word at 0x10,
/// stores whether the cinfo block (`this + 0xC`) is still missing in the byte `created`
/// points to and, when it is, allocates one (`size` bytes from the memory manager, with the
/// reference count 1 at `+8`, the class's `vtable` and the cinfo at `+4`), stores the cinfo's
/// address in `this + 0xC` and hands the block to `00c89c80` if `this + 8` is set. Answers
/// `this + 0xC`.
pub(crate) fn create_cinfo_block(
    e: &mut Engine,
    this: Ptr,
    created: u32,
    size: u32,
    vtable: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let object = this.addr();
    let missing = e.mem.u32(object + 0xc) == 0;
    e.mem.set_u8(created, u8::from(missing));
    if missing {
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
            .u32();
        let cinfo = if block != 0 {
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block, vtable);
            e.mem.set_u32(block + 4, 0);
            block + 4
        } else {
            0
        };
        e.mem.set_u32(object + 0xc, cinfo);
        if e.mem.u32(object + 8) != 0 {
            let start = if cinfo == 0 { 0 } else { cinfo - 4 };
            e.call(CONSTRAINT_ADOPT_CINFO, &args![this, start]);
        }
    }
    let result = e.mem.u32(object + 0xc);
    e.mem.set_u32(word, saved);
    result
}

/// The body of every cinfo holder's `CreateData`: with the allocation-scope word at 0x10,
/// builds the data block (`size` bytes from the current Havok allocator, then its constructor
/// `construct`, whose answer is kept) when the holder (`+4`) has none yet.
pub(crate) fn create_data_block(e: &mut Engine, this: Ptr, size: u32, construct: u32) {
    let (word, saved) = enter_scope(e);
    if e.mem.u32(this.addr() + 4) == 0 {
        let block = havok_allocate(e, size);
        let data = e.call(construct, &args![block]).u32();
        e.mem.set_u32(this.addr() + 4, data);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cbd970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbd970 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHingeConstraint::~bhkHingeConstraint` (Xbox PDB): installs the `bhkHingeConstraint`
/// vtable, runs `bhkSerializable::Kill`, decrements the live hinge constraint counter and runs
/// the `bhkConstraint` destructor body.
pub fn bhk_hinge_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), HINGE_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(HINGE_COUNT);
    e.mem.set_u32(HINGE_COUNT, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00cbda30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHingeConstraintCinfo::LoadBinary` (Xbox PDB): the base cinfo load, then the two hinge
/// frames (0x40 bytes each, at data `+0x20` and `+0x60`) read through the stream's reader
/// (`stream + 0x24C`, function at `+8`, no length cell, flag 0). When the stream's byte
/// (`+0x1E5`) is not 1 the four words at `+0x30` of each frame are byte-swapped and the frame
/// goes through `00c8cdb0`, the first frame before the second.
pub fn hk_hinge_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for frame in [data + 0x20, data + 0x60] {
        let reader = e.mem.u32(stream + 0x24c);
        stream_transfer(e, reader, 8, frame, 0x40, None, 0);
    }
    if e.mem.u8(stream + 0x1e5) != 1 {
        for frame in [data + 0x20, data + 0x60] {
            for offset in [0x30, 0x34, 0x38, 0x3c] {
                let word = e.mem.u32(frame + offset);
                e.mem.set_u32(frame + offset, word.swap_bytes());
            }
            e.call(HINGE_FRAME_SWAP, &args![frame]);
        }
    }
}

// Translated from 00cbdae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbdae0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHingeConstraint` factory (no PDB name): allocates 0x10 bytes from the game heap and
/// constructs a hinge constraint in them. Answers the object, or 0 when the allocation failed
/// (takes no arguments).
pub fn fn_00cbdae0(e: &mut Engine) -> u32 {
    new_constraint(e, HINGE_CLASSES)
}

// Translated from 00cbdb70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbdb70 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkHingeConstraintCinfo::CloneData` (Xbox PDB): with the allocation-scope word at 0x10,
/// uses `data` as the destination or, when it is 0, builds a new data block (0xB0 bytes from the
/// current Havok allocator and its constructor `00cbe7b0`). Copies the 128 bytes from data
/// `+0x20` of this holder's block (aligned 16-byte loads and stores in the exe), then, when
/// the float at `context + 0x10` is not exactly 1.0 (a NaN counts as different), scales the
/// copy (`00cbd9e0`, cdecl, with the float's bit pattern first). Answers the base
/// `CloneData` (`00c8a1b0`, `(this, data, context)`).
pub fn hk_hinge_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    context: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let source = e.mem.u32(this.addr() + 4);
    let data = if data == 0 {
        let block = havok_allocate(e, DATA_SIZE);
        e.call(HINGE_DATA_CONSTRUCT, &args![block]).u32()
    } else {
        data
    };
    for offset in (0x20..0xa0).step_by(4) {
        let value = e.mem.u32(source + offset);
        e.mem.set_u32(data + offset, value);
    }
    let scale_bits = x87_quiet_bits(e.mem.u32(context + 0x10));
    if f32::from_bits(scale_bits) != 1.0 {
        e.call(HINGE_DATA_SCALE, &args![scale_bits, data]);
    }
    let result = e.call(CINFO_CLONE_DATA, &args![this, data, context]).u32();
    e.mem.set_u32(word, saved);
    result
}

// Translated from 00cbdcb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbdcb0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkHingeConstraintCinfo::CreateData` (Xbox PDB): builds the holder's data block when it has
/// none (`create_data_block`: 0xB0 bytes and the constructor `00cbe7b0`). Answers the size,
/// 0xB0.
pub fn hk_hinge_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    create_data_block(e, this, DATA_SIZE, HINGE_DATA_CONSTRUCT);
    DATA_SIZE
}

// Translated from 00cbdd60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbdd60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHingeConstraint::CreateClone` (Xbox PDB): a new hinge constraint filled from `this`
/// (`clone_constraint`). Answers the clone.
pub fn bhk_hinge_constraint_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    clone_constraint(e, this, context, HINGE_CLASSES)
}

// Translated from 00cbde40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbde40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHingeConstraint::CreateCinfo` (Xbox PDB): `create_cinfo_block` with the hinge cinfo
/// block vtable `0108ff98`. `created` is the address of the byte that receives whether a new
/// block was made.
pub fn bhk_hinge_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    create_cinfo_block(e, this, created, 0x14, HINGE_CINFO_BLOCK_VTABLE)
}

// Translated from 00cbdf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHingeConstraintCinfo::SaveBinary` (Xbox PDB): the base cinfo save, then the two hinge
/// frames (0x40 bytes each, at data `+0x20` and `+0x60`) written through the stream's writer
/// (`stream + 0x250`, function at `+0xC`, length cell initialised to 0x40, flag 1).
pub fn hk_hinge_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for frame in [data + 0x20, data + 0x60] {
        let writer = e.mem.u32(stream + 0x250);
        stream_transfer(e, writer, 0xc, frame, 0x40, Some(0x40), 1);
    }
}

// Translated from 00cbdf80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbdf80 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkHingeConstraintCinfo::GetViewerStrings` (Xbox PDB): with the allocation-scope word at
/// 0x10, runs the base cinfo strings and, when the holder has a data block, appends five
/// `NiPoint3` strings (each vector copied into a temporary point with `004a3970`, then
/// `00a59af0` with the name): `PivotInA` (data `+0x50`), `PivotInB` (`+0x90`), `AxleInB`
/// (`+0x60`), `Perp2AxleInA1` (`+0x30`) and `Perp2AxleInA2` (`+0x40`).
pub fn hk_hinge_constraint_cinfo_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(CINFO_GET_VIEWER_STRINGS, &args![this, strings]);
    let data = e.mem.u32(this.addr() + 4);
    if data != 0 {
        e.with_stack(12, |e, point| {
            let names = [
                (0x50, NAME_PIVOT_IN_A),
                (0x90, NAME_PIVOT_IN_B),
                (0x60, NAME_AXLE_IN_B),
                (0x30, NAME_PERP_2_AXLE_IN_A_1),
                (0x40, NAME_PERP_2_AXLE_IN_A_2),
            ];
            for (offset, name) in names {
                e.call(VECTOR_TO_POINT, &args![point, data + offset]);
                let text = e
                    .call(NI_POINT3_GET_VIEWER_STRING, &args![point, name])
                    .u32();
                push_viewer_string(e, strings, text);
            }
        });
    }
    e.mem.set_u32(word, saved);
}

/// Test helpers shared with the sibling constraint units.
#[cfg(test)]
pub(crate) mod testing {
    use super::*;

    /// Maps the counters and registers do-nothing doubles for the base routines the
    /// constraint classes call (the allocator is left to `allocator_doubles`).
    pub(crate) fn constraint_doubles(e: &mut Engine, counters: &[u32]) {
        e.map(CONSTRAINT_COUNT, 4);
        for &counter in counters {
            e.map(counter, 4);
        }
        for addr in [
            CONSTRAINT_CONSTRUCT,
            CONSTRAINT_DESTRUCT,
            CONSTRAINT_COPY_MEMBERS,
            CONSTRAINT_ADOPT_CINFO,
            CINFO_LOAD_BINARY,
            CINFO_SAVE_BINARY,
            CINFO_CLONE_DATA,
            CINFO_GET_VIEWER_STRINGS,
            SERIALIZABLE_KILL,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// A stream with a reader at `+0x24C` and a writer at `+0x250`, each an object whose
    /// function slot (`+8` for the reader, `+0xC` for the writer) is the fake address
    /// `0x0900_0e08` (reader) / `0x0900_0e0c` (writer); the fakes answer the requested size
    /// and are logged by the calling test.
    pub(crate) fn stream_with_endpoints(e: &mut Engine) -> (u32, u32, u32) {
        let stream = e.mem.alloc(0x260);
        let reader = e.mem.alloc(0x20);
        let writer = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 0x24c, reader);
        e.mem.set_u32(stream + 0x250, writer);
        e.mem.set_u32(reader + 8, 0x0900_0e08);
        e.mem.set_u32(writer + 0xc, 0x0900_0e0c);
        for fake in [0x0900_0e08, 0x0900_0e0c] {
            e.register(fake, |e, a| {
                // A length cell, when there is one, starts out as the size.
                if a[3] != 0 {
                    assert_eq!(e.mem.u32(a[3]), a[2]);
                }
                Ret {
                    eax: a[2],
                    ..Ret::default()
                }
            });
        }
        (stream, reader, writer)
    }

    /// A cinfo holder with a data block of `size` bytes whose words are `index + 1`.
    pub(crate) fn holder_with_data(e: &mut Engine, size: u32) -> (u32, u32) {
        let holder = e.mem.alloc(0x20);
        let data = e.mem.alloc(size);
        for offset in (0..size).step_by(4) {
            e.mem.set_u32(data + offset, offset / 4 + 1);
        }
        e.mem.set_u32(holder + 4, data);
        (holder, data)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_d970, bhk_hinge_constraint_destruct(Ptr)),
        entry!(0x00cb_da30, hk_hinge_constraint_cinfo_load_binary(Ptr, u32)),
        entry!(0x00cb_dae0, fn_00cbdae0() -> u32),
        entry!(
            0x00cb_db70,
            hk_hinge_constraint_cinfo_clone_data(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00cb_dcb0,
            hk_hinge_constraint_cinfo_create_data(Ptr) -> u32
        ),
        entry!(
            0x00cb_dd60,
            bhk_hinge_constraint_create_clone(Ptr, u32) -> u32
        ),
        entry!(
            0x00cb_de40,
            bhk_hinge_constraint_create_cinfo(Ptr, u32) -> u32
        ),
        entry!(0x00cb_df10, hk_hinge_constraint_cinfo_save_binary(Ptr, u32)),
        entry!(
            0x00cb_df80,
            hk_hinge_constraint_cinfo_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::{SCOPE_WORD, STRINGS_SET_AT};
    use super::testing::*;
    use super::*;

    #[test]
    fn destructor_installs_the_vtable_and_counts_the_hinge_down() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[HINGE_COUNT]);
        e.mem.set_u32(HINGE_COUNT, 5);
        let this = e.mem.alloc(0x10);
        e.call(0x00cb_d970, &args![this]);
        assert_eq!(e.mem.u32(this), HINGE_VTABLE);
        assert_eq!(e.mem.u32(HINGE_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONSTRAINT_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn load_binary_reads_both_frames_and_swaps_them_for_a_foreign_stream() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        e.register(HINGE_FRAME_SWAP, |_, _| Ret::default());
        let (stream, reader, _) = stream_with_endpoints(&mut e);
        let (holder, data) = holder_with_data(&mut e, 0xb0);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00cb_da30, &args![holder, stream]);
        assert_eq!(calls_to(&e, CINFO_LOAD_BINARY), vec![vec![holder, stream]]);
        assert_eq!(
            calls_to(&e, 0x0900_0e08),
            vec![
                vec![reader, data + 0x20, 0x40, 0, 0],
                vec![reader, data + 0x60, 0x40, 0, 0]
            ]
        );
        assert_eq!(e.mem.u32(reader + 4), 0x80);
        assert!(calls_to(&e, HINGE_FRAME_SWAP).is_empty());
        // A stream whose byte is not 1: the words at +0x30 of each frame are swapped.
        e.mem.set_u8(stream + 0x1e5, 0);
        e.mem.set_u32(data + 0x50, 0x1122_3344);
        e.mem.set_u32(data + 0x9c, 0xaabb_ccdd);
        e.call(0x00cb_da30, &args![holder, stream]);
        assert_eq!(e.mem.u32(data + 0x50), 0x4433_2211);
        assert_eq!(e.mem.u32(data + 0x9c), 0xddcc_bbaa);
        assert_eq!(
            calls_to(&e, HINGE_FRAME_SWAP),
            vec![vec![data + 0x20], vec![data + 0x60]]
        );
    }

    #[test]
    fn factory_builds_a_hinge_constraint_or_nothing() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[HINGE_COUNT]);
        allocator_doubles(&mut e);
        let object = e.call(0x00cb_dae0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), HINGE_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 1);
        assert_eq!(e.mem.u32(HINGE_COUNT), 1);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, CONSTRAINT_CONSTRUCT), vec![vec![object]]);
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cb_dae0, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(HINGE_COUNT), 1);
    }

    #[test]
    fn clone_data_copies_the_frames_and_scales_unless_exactly_one() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(HINGE_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(HINGE_DATA_SCALE, |_, _| Ret::default());
        e.register(CINFO_CLONE_DATA, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let (holder, _) = holder_with_data(&mut e, 0xb0);
        let context = e.mem.alloc(0x20);
        let destination = e.mem.alloc(0xb0);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.mem.set_f32(context + 0x10, 1.0);
        let call_args = args![holder, destination, context];
        assert_eq!(e.call(0x00cb_db70, &call_args).u32(), 0x55);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(e.mem.u32(destination + 0x20), 9);
        assert_eq!(e.mem.u32(destination + 0x9c), 0x28);
        assert_eq!(e.mem.u32(destination + 0x1c), 0);
        assert_eq!(e.mem.u32(destination + 0xa0), 0);
        assert!(calls_to(&e, HINGE_DATA_SCALE).is_empty());
        assert_eq!(
            calls_to(&e, CINFO_CLONE_DATA),
            vec![vec![holder, destination, context]]
        );
        // A scale of 2: the routine gets the bit pattern and the destination.
        e.mem.set_f32(context + 0x10, 2.0);
        e.call(0x00cb_db70, &call_args);
        assert_eq!(
            calls_to(&e, HINGE_DATA_SCALE),
            vec![vec![2.0f32.to_bits(), destination]]
        );
        // A signalling NaN differs from 1.0 and goes on quiet.
        e.mem.set_u32(context + 0x10, 0x7f80_0001);
        e.call(0x00cb_db70, &call_args);
        assert_eq!(
            calls_to(&e, HINGE_DATA_SCALE)[1],
            vec![0x7fc0_0001, destination]
        );
        // No destination: a new block comes from the Havok allocator and its constructor.
        e.mem.set_f32(context + 0x10, 1.0);
        let result = e.call(0x00cb_db70, &args![holder, 0u32, context]).u32();
        assert_eq!(result, 0x55);
        let made = calls_to(&e, CINFO_CLONE_DATA)[3][1];
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, HINGE_DATA_CONSTRUCT), vec![vec![made]]);
        assert_eq!(e.mem.u16(made + 4), 0xb0);
        assert_eq!(e.mem.u32(made + 0x20), 9);
    }

    #[test]
    fn create_data_builds_the_block_once() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(HINGE_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        let holder = e.mem.alloc(0x20);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        assert_eq!(e.call(0x00cb_dcb0, &args![holder]).u32(), 0xb0);
        let block = calls_to(&e, HINGE_DATA_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(holder + 4), block + 0x10);
        assert_eq!(e.mem.u16(block + 4), 0xb0);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // With a block already there nothing is built.
        assert_eq!(e.call(0x00cb_dcb0, &args![holder]).u32(), 0xb0);
        assert_eq!(calls_to(&e, HINGE_DATA_CONSTRUCT).len(), 1);
        assert_eq!(e.mem.u32(holder + 4), block + 0x10);
    }

    #[test]
    fn create_clone_builds_a_hinge_under_the_clone_lock() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[HINGE_COUNT]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        let clone = e.call(0x00cb_dd60, &args![this, 0x1234u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), HINGE_VTABLE);
        assert_eq!(e.mem.u32(HINGE_COUNT), 1);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS),
            vec![vec![this, clone, 0x1234]]
        );
        assert_eq!(calls_to(&e, CLONE_LOCK).len(), 1);
        assert_eq!(calls_to(&e, CLONE_UNLOCK).len(), 1);
        // A failed allocation still goes through the copy, with a null clone.
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cb_dd60, &args![this, 0x1234u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS)[1],
            vec![this, 0, 0x1234]
        );
    }

    #[test]
    fn create_cinfo_makes_the_block_only_when_missing() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let created = e.mem.alloc(4);
        e.mem.set_u8(created, 0xee);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        // No holder (+8 is 0): the block is made but not handed on.
        let cinfo = e.call(0x00cb_de40, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        let block = cinfo - 4;
        assert_eq!(e.mem.u32(block), HINGE_CINFO_BLOCK_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
        assert!(calls_to(&e, CONSTRAINT_ADOPT_CINFO).is_empty());
        // Already there: reported as not created and returned unchanged.
        assert_eq!(e.call(0x00cb_de40, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);
        // With a holder the new block is handed to the base routine.
        e.mem.set_u32(this + 8, 0x77);
        e.mem.set_u32(this + 0xc, 0);
        let cinfo = e.call(0x00cb_de40, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(
            calls_to(&e, CONSTRAINT_ADOPT_CINFO),
            vec![vec![this, cinfo - 4]]
        );
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // A failed allocation leaves the block missing and hands over null.
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(e.call(0x00cb_de40, &args![this, created]).u32(), 0);
        assert_eq!(calls_to(&e, CONSTRAINT_ADOPT_CINFO)[1], vec![this, 0]);
    }

    #[test]
    fn save_binary_writes_both_frames_after_the_base_save() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        let (stream, _, writer) = stream_with_endpoints(&mut e);
        let (holder, data) = holder_with_data(&mut e, 0xb0);
        e.call(0x00cb_df10, &args![holder, stream]);
        assert_eq!(calls_to(&e, CINFO_SAVE_BINARY), vec![vec![holder, stream]]);
        let writes = calls_to(&e, 0x0900_0e0c);
        assert_eq!(writes.len(), 2);
        assert_eq!(
            writes
                .iter()
                .map(|c| (c[0], c[1], c[2], c[4]))
                .collect::<Vec<_>>(),
            vec![
                (writer, data + 0x20, 0x40, 1),
                (writer, data + 0x60, 0x40, 1)
            ]
        );
        assert_eq!(e.mem.u32(writer + 4), 0x80);
    }

    #[test]
    fn viewer_strings_list_the_five_vectors() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        viewer_doubles(&mut e);
        e.register(VECTOR_TO_POINT, |_, _| Ret::default());
        e.register(NI_POINT3_GET_VIEWER_STRING, |_, a| Ret {
            eax: 0x8000 + a[1],
            ..Ret::default()
        });
        let (holder, data) = holder_with_data(&mut e, 0xb0);
        let strings = strings_header(&mut e, 0, 8, 2);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00cb_df80, &args![holder, strings]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, CINFO_GET_VIEWER_STRINGS),
            vec![vec![holder, strings]]
        );
        let vectors: Vec<u32> = calls_to(&e, VECTOR_TO_POINT).iter().map(|c| c[1]).collect();
        assert_eq!(
            vectors,
            vec![
                data + 0x50,
                data + 0x90,
                data + 0x60,
                data + 0x30,
                data + 0x40
            ]
        );
        assert_eq!(
            pushed_texts(&e, strings, 0, 5),
            vec![
                0x8000 + NAME_PIVOT_IN_A,
                0x8000 + NAME_PIVOT_IN_B,
                0x8000 + NAME_AXLE_IN_B,
                0x8000 + NAME_PERP_2_AXLE_IN_A_1,
                0x8000 + NAME_PERP_2_AXLE_IN_A_2
            ]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 5);
        // Without a data block only the base strings are listed.
        e.mem.set_u32(holder + 4, 0);
        e.call(0x00cb_df80, &args![holder, strings]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 5);
        assert_eq!(calls_to(&e, CINFO_GET_VIEWER_STRINGS).len(), 2);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
    }
}
