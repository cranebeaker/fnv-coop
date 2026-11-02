//! `BSHavok/bhkhingelimitsconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `bhkHingeLimitsConstraint` is built like a hinge constraint (see `bhkhingeconstraint`);
//! its cinfo holder's data block is 0xA0 bytes: vectors from `+0x20`, and the
//! two limit floats at `+0x84` and `+0x88`. The exception-unwinding frames (`FS:[0]`
//! registrations) of the functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{
    enter_scope, havok_allocate, push_viewer_string, stream_transfer, HEAP_ALLOCATE,
    NI_GET_VIEWER_STRING_FLOAT, NI_POINT3_GET_VIEWER_STRING, SERIALIZABLE_KILL, VECTOR_TO_POINT,
};
use super::bhkhingeconstraint::{
    clone_constraint, construct_constraint, create_cinfo_block, create_data_block,
    CINFO_CLONE_DATA, CINFO_GET_VIEWER_STRINGS, CINFO_LOAD_BINARY, CINFO_SAVE_BINARY,
    CONSTRAINT_DESTRUCT,
};

/// The hinge limits constraint's vtable and live counter, its cinfo block's vtable and the
/// data block's constructor (`00d52fc0`, thiscall on the block).
const LIMITS_VTABLE: u32 = 0x010c_9f74;
const LIMITS_COUNT: u32 = 0x0126_8630;
const LIMITS_CLASSES: &[(u32, u32)] = &[(LIMITS_VTABLE, LIMITS_COUNT)];
const LIMITS_CINFO_BLOCK_VTABLE: u32 = 0x010c_a050;
const LIMITS_DATA_CONSTRUCT: u32 = 0x00d5_2fc0;
/// Size of the data block.
const DATA_SIZE: u32 = 0xa0;
/// Viewer names: the vectors (`AxleInB`, `Perp2AxleInA1`, `Perp2AxleInA2`) and the two limits
/// (`00a7d550` takes the name first, then the float).
const NAME_AXLE_IN_B: u32 = 0x010c_8388;
const NAME_PERP_2_AXLE_IN_A_1: u32 = 0x010c_8378;
const NAME_PERP_2_AXLE_IN_A_2: u32 = 0x010c_8368;
const NAME_FIRST_LIMIT: u32 = 0x010c_835c;
const NAME_SECOND_LIMIT: u32 = 0x010c_8350;

// Translated from 00cc8480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc8480 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHingeLimitsConstraint::~bhkHingeLimitsConstraint` (Xbox PDB): installs the
/// `bhkHingeLimitsConstraint` vtable, runs `bhkSerializable::Kill`, decrements the live
/// counter of this class and runs the `bhkConstraint` destructor body.
pub fn bhk_hinge_limits_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LIMITS_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(LIMITS_COUNT);
    e.mem.set_u32(LIMITS_COUNT, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00cc84f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkHingeLimitsConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8634.
pub fn bhk_hinge_limits_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8634
}

// Translated from 00cc8500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkHingeLimitsConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc8480`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_hinge_limits_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_8480;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc8520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc8520 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHingeLimitsConstraint` factory (no PDB name): allocates 0x10 bytes from the game heap and
/// constructs a hinge limits constraint in them. Answers the object, or 0 when the allocation
/// failed (takes no arguments).
pub fn fn_00cc8520(e: &mut Engine) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
    construct_constraint(e, block, LIMITS_CLASSES)
}

// Translated from 00cc85b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc85b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkHingeLimitsConstraintCinfo::CloneData` (Xbox PDB): with the allocation-scope word at
/// 0x10, uses `data` as the destination or, when it is 0, builds a new data block (0xA0 bytes
/// from the current Havok allocator and its constructor `00d52fc0`). Copies the 96 bytes from
/// data `+0x20` of this holder's block (aligned 16-byte loads and stores in the exe) and the
/// floats at `+0x84` and `+0x88` (through the x87, so a signalling NaN comes out quiet).
/// Answers the base `CloneData` (`00c8a1b0`, `(this, data, context)`).
pub fn hk_hinge_limits_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    context: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let source = e.mem.u32(this.addr() + 4);
    let data = if data == 0 {
        let block = havok_allocate(e, DATA_SIZE);
        e.call(LIMITS_DATA_CONSTRUCT, &args![block]).u32()
    } else {
        data
    };
    for offset in (0x20..0x80).step_by(4) {
        let value = e.mem.u32(source + offset);
        e.mem.set_u32(data + offset, value);
    }
    for offset in [0x84, 0x88] {
        let value = x87_quiet_bits(e.mem.u32(source + offset));
        e.mem.set_u32(data + offset, value);
    }
    let result = e.call(CINFO_CLONE_DATA, &args![this, data, context]).u32();
    e.mem.set_u32(word, saved);
    result
}

// Translated from 00cc86c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc86c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkHingeLimitsConstraintCinfo::CreateData` (Xbox PDB): builds the holder's data block when
/// it has none (0xA0 bytes and the constructor `00d52fc0`). Answers the size, 0xA0.
pub fn hk_hinge_limits_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    create_data_block(e, this, DATA_SIZE, LIMITS_DATA_CONSTRUCT);
    DATA_SIZE
}

// Translated from 00cc8770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc8770 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHingeLimitsConstraint::CreateClone` (Xbox PDB): a new hinge limits constraint filled
/// from `this` (`clone_constraint`). Answers the clone.
pub fn bhk_hinge_limits_constraint_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    clone_constraint(e, this, context, LIMITS_CLASSES)
}

// Translated from 00cc8850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc8850 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkHingeLimitsConstraint::CreateCinfo` (Xbox PDB): `create_cinfo_block` with the cinfo
/// block vtable `010ca050`. `created` is the address of the byte that receives whether a new
/// block was made.
pub fn bhk_hinge_limits_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    create_cinfo_block(e, this, created, 0x14, LIMITS_CINFO_BLOCK_VTABLE)
}

// Translated from 00cc8950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHingeLimitsConstraintCinfo::LoadBinary` (Xbox PDB): the base cinfo load, then four reads
/// through the stream's reader (`stream + 0x24C`, function at `+8`, length cell initialised to
/// the size, flag 1): 0x30 bytes at data `+0x20`, 0x30 at `+0x50`, 4 at `+0x84` and 4 at
/// `+0x88`.
pub fn hk_hinge_limits_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for (offset, size) in [(0x20, 0x30), (0x50, 0x30), (0x84, 4), (0x88, 4)] {
        let reader = e.mem.u32(stream + 0x24c);
        stream_transfer(e, reader, 8, data + offset, size, Some(size), 1);
    }
}

// Translated from 00cc8a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHingeLimitsConstraintCinfo::SaveBinary` (Xbox PDB): writes 0x30 bytes at data `+0x20`,
/// 0x30 at `+0x50`, 4 at `+0x84` and 4 at `+0x88` through the stream's writer
/// (`stream + 0x250`, function at `+0xC`, length cell initialised to the size, flag 1), then
/// runs the base cinfo save.
pub fn hk_hinge_limits_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let data = e.mem.u32(this.addr() + 4);
    for (offset, size) in [(0x20, 0x30), (0x50, 0x30), (0x84, 4), (0x88, 4)] {
        let writer = e.mem.u32(stream + 0x250);
        stream_transfer(e, writer, 0xc, data + offset, size, Some(size), 1);
    }
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00cc8ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc8ad0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkHingeLimitsConstraintCinfo::GetViewerStrings` (Xbox PDB): with the allocation-scope word
/// at 0x10, runs the base cinfo strings and, when the holder has a data block, appends three
/// `NiPoint3` strings (each vector copied into a temporary point with `004a3970`, then
/// `00a59af0` with the name): `AxleInB` (data `+0x50`), `Perp2AxleInA1` (`+0x30`) and
/// `Perp2AxleInA2` (`+0x40`); then the floats at `+0x84` and `+0x88` as strings
/// (`00a7d550`, name then the float's bit pattern).
pub fn hk_hinge_limits_constraint_cinfo_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: u32,
) {
    let (word, saved) = enter_scope(e);
    e.call(CINFO_GET_VIEWER_STRINGS, &args![this, strings]);
    let data = e.mem.u32(this.addr() + 4);
    if data != 0 {
        e.with_stack(12, |e, point| {
            let names = [
                (0x50, NAME_AXLE_IN_B),
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
        for (offset, name) in [(0x84, NAME_FIRST_LIMIT), (0x88, NAME_SECOND_LIMIT)] {
            let limit = x87_quiet_bits(e.mem.u32(data + offset));
            let text = e
                .call(NI_GET_VIEWER_STRING_FLOAT, &args![name, limit])
                .u32();
            push_viewer_string(e, strings, text);
        }
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc_8480, bhk_hinge_limits_constraint_destruct(Ptr)),
        entry!(0x00cc_84f0, bhk_hinge_limits_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cc_8500, bhk_hinge_limits_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc_8520, fn_00cc8520() -> u32),
        entry!(
            0x00cc_85b0,
            hk_hinge_limits_constraint_cinfo_clone_data(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00cc_86c0,
            hk_hinge_limits_constraint_cinfo_create_data(Ptr) -> u32
        ),
        entry!(
            0x00cc_8770,
            bhk_hinge_limits_constraint_create_clone(Ptr, u32) -> u32
        ),
        entry!(
            0x00cc_8850,
            bhk_hinge_limits_constraint_create_cinfo(Ptr, u32) -> u32
        ),
        entry!(
            0x00cc_8950,
            hk_hinge_limits_constraint_cinfo_load_binary(Ptr, u32)
        ),
        entry!(
            0x00cc_8a10,
            hk_hinge_limits_constraint_cinfo_save_binary(Ptr, u32)
        ),
        entry!(
            0x00cc_8ad0,
            hk_hinge_limits_constraint_cinfo_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::{
        MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE, SCOPE_WORD, STRINGS_SET_AT,
    };
    use super::super::bhkhingeconstraint::testing::*;
    use super::super::bhkhingeconstraint::{
        CONSTRAINT_ADOPT_CINFO, CONSTRAINT_CONSTRUCT, CONSTRAINT_COPY_MEMBERS, CONSTRAINT_COUNT,
    };
    use super::*;

    #[test]
    fn bhk_hinge_limits_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_8480, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cc8500, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc8500, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_8480), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_hinge_limits_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_84f0, &args![]).u32(), 0x0126_8634);
    }

    #[test]
    fn destructor_installs_the_vtable_and_counts_the_constraint_down() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[LIMITS_COUNT]);
        e.mem.set_u32(LIMITS_COUNT, 3);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_8480, &args![this]);
        assert_eq!(e.mem.u32(this), LIMITS_VTABLE);
        assert_eq!(e.mem.u32(LIMITS_COUNT), 2);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONSTRAINT_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn factory_builds_a_constraint_or_nothing() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[LIMITS_COUNT]);
        allocator_doubles(&mut e);
        let object = e.call(0x00cc_8520, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), LIMITS_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 1);
        assert_eq!(e.mem.u32(LIMITS_COUNT), 1);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, CONSTRAINT_CONSTRUCT), vec![vec![object]]);
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_8520, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(LIMITS_COUNT), 1);
    }

    #[test]
    fn clone_data_copies_the_vectors_and_the_limits() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(LIMITS_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CINFO_CLONE_DATA, |_, _| Ret {
            eax: 0x66,
            ..Ret::default()
        });
        let (holder, source) = holder_with_data(&mut e, 0xa0);
        // A signalling NaN limit is copied quiet.
        e.mem.set_u32(source + 0x84, 0x7f80_0001);
        e.mem.set_f32(source + 0x88, 3.5);
        let context = e.mem.alloc(0x20);
        let destination = e.mem.alloc(0xa0);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        assert_eq!(
            e.call(0x00cc_85b0, &args![holder, destination, context])
                .u32(),
            0x66
        );
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(e.mem.u32(destination + 0x20), 9);
        assert_eq!(e.mem.u32(destination + 0x7c), 0x20);
        // Past the vectors only the two limits are copied.
        assert_eq!(e.mem.u32(destination + 0x80), 0);
        assert_eq!(e.mem.u32(destination + 0x84), 0x7fc0_0001);
        assert_eq!(e.mem.f32(destination + 0x88), 3.5);
        assert_eq!(e.mem.u32(destination + 0x8c), 0);
        assert_eq!(e.mem.u32(destination + 0x1c), 0);
        assert_eq!(
            calls_to(&e, CINFO_CLONE_DATA),
            vec![vec![holder, destination, context]]
        );
        // No destination: a new 0xA0 block is made.
        e.call(0x00cc_85b0, &args![holder, 0u32, context]);
        let made = calls_to(&e, CINFO_CLONE_DATA)[1][1];
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, LIMITS_DATA_CONSTRUCT), vec![vec![made]]);
        assert_eq!(e.mem.u16(made + 4), 0xa0);
        assert_eq!(e.mem.u32(made + 0x20), 9);
    }

    #[test]
    fn create_data_builds_the_block_once() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(LIMITS_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        let holder = e.mem.alloc(0x20);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        assert_eq!(e.call(0x00cc_86c0, &args![holder]).u32(), 0xa0);
        let block = calls_to(&e, LIMITS_DATA_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(holder + 4), block + 0x10);
        assert_eq!(e.mem.u16(block + 4), 0xa0);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(e.call(0x00cc_86c0, &args![holder]).u32(), 0xa0);
        assert_eq!(calls_to(&e, LIMITS_DATA_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_builds_a_constraint_under_the_clone_lock() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[LIMITS_COUNT]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        let clone = e.call(0x00cc_8770, &args![this, 0x1234u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), LIMITS_VTABLE);
        assert_eq!(e.mem.u32(LIMITS_COUNT), 1);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS),
            vec![vec![this, clone, 0x1234]]
        );
    }

    #[test]
    fn create_cinfo_makes_the_block_only_when_missing() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let created = e.mem.alloc(4);
        e.mem.set_u32(this + 8, 0x77);
        let cinfo = e.call(0x00cc_8850, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo - 4), LIMITS_CINFO_BLOCK_VTABLE);
        assert_eq!(e.mem.u32(cinfo + 4), 1);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
        assert_eq!(
            calls_to(&e, CONSTRAINT_ADOPT_CINFO),
            vec![vec![this, cinfo - 4]]
        );
        assert_eq!(e.call(0x00cc_8850, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, CONSTRAINT_ADOPT_CINFO).len(), 1);
    }

    #[test]
    fn load_binary_reads_the_vectors_and_the_limits() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        let (stream, reader, _) = stream_with_endpoints(&mut e);
        let (holder, data) = holder_with_data(&mut e, 0xa0);
        e.call(0x00cc_8950, &args![holder, stream]);
        assert_eq!(calls_to(&e, CINFO_LOAD_BINARY), vec![vec![holder, stream]]);
        let reads: Vec<(u32, u32, u32)> = calls_to(&e, 0x0900_0e08)
            .iter()
            .map(|c| (c[1], c[2], c[4]))
            .collect();
        assert_eq!(
            reads,
            vec![
                (data + 0x20, 0x30, 1),
                (data + 0x50, 0x30, 1),
                (data + 0x84, 4, 1),
                (data + 0x88, 4, 1)
            ]
        );
        assert_eq!(e.mem.u32(reader + 4), 0x68);
    }

    #[test]
    fn save_binary_writes_before_the_base_save() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        let (stream, _, writer) = stream_with_endpoints(&mut e);
        let (holder, data) = holder_with_data(&mut e, 0xa0);
        e.call(0x00cc_8a10, &args![holder, stream]);
        let writes: Vec<(u32, u32, u32)> = calls_to(&e, 0x0900_0e0c)
            .iter()
            .map(|c| (c[1], c[2], c[4]))
            .collect();
        assert_eq!(
            writes,
            vec![
                (data + 0x20, 0x30, 1),
                (data + 0x50, 0x30, 1),
                (data + 0x84, 4, 1),
                (data + 0x88, 4, 1)
            ]
        );
        assert_eq!(e.mem.u32(writer + 4), 0x68);
        assert_eq!(calls_to(&e, CINFO_SAVE_BINARY), vec![vec![holder, stream]]);
        // The base save came after the last write.
        let log = e.call_log.as_ref().unwrap();
        let last_write = log.iter().rposition(|c| c.0 == 0x0900_0e0c).unwrap();
        let base = log.iter().position(|c| c.0 == CINFO_SAVE_BINARY).unwrap();
        assert!(base > last_write);
    }

    #[test]
    fn viewer_strings_list_three_vectors_and_two_limits() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        viewer_doubles(&mut e);
        e.register(VECTOR_TO_POINT, |_, _| Ret::default());
        e.register(NI_POINT3_GET_VIEWER_STRING, |_, a| Ret {
            eax: 0x8000 + a[1],
            ..Ret::default()
        });
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, a| Ret {
            eax: 0x9000 + a[0],
            ..Ret::default()
        });
        let (holder, data) = holder_with_data(&mut e, 0xa0);
        e.mem.set_u32(data + 0x84, 0x7f80_0001);
        let strings = strings_header(&mut e, 0, 8, 2);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00cc_8ad0, &args![holder, strings]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        let vectors: Vec<u32> = calls_to(&e, VECTOR_TO_POINT).iter().map(|c| c[1]).collect();
        assert_eq!(vectors, vec![data + 0x50, data + 0x30, data + 0x40]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![
                vec![NAME_FIRST_LIMIT, 0x7fc0_0001],
                vec![NAME_SECOND_LIMIT, 0x23]
            ]
        );
        assert_eq!(
            pushed_texts(&e, strings, 0, 5),
            vec![
                0x8000 + NAME_AXLE_IN_B,
                0x8000 + NAME_PERP_2_AXLE_IN_A_1,
                0x8000 + NAME_PERP_2_AXLE_IN_A_2,
                0x9000 + NAME_FIRST_LIMIT,
                0x9000 + NAME_SECOND_LIMIT
            ]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 5);
        // Without a data block only the base strings are listed.
        e.mem.set_u32(holder + 4, 0);
        e.call(0x00cc_8ad0, &args![holder, strings]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 5);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
    }
}
