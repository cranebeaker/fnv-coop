//! `BSHavok/bhkwheelconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkWheelConstraint` is a 0x10-byte constraint object built the way the other constraint
//! classes are (`bhkHingeConstraint` shares the helpers): its cinfo holder
//! (`hkWheelConstraintCinfo` in the Xbox PDB) is a 0x14-byte block (vtable `010c9af0`) whose
//! word at `+4` points at the Havok constraint data, a 0x160-byte block. The functions marked
//! `Translation incomplete` carry an `FS:[0]` exception-unwinding frame, which is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{enter_scope, havok_allocate, stream_transfer, SERIALIZABLE_KILL};
use super::bhkhingeconstraint::{
    clone_constraint, create_cinfo_block, create_data_block, new_constraint, CINFO_CLONE_DATA,
    CINFO_LOAD_BINARY, CINFO_SAVE_BINARY, CONSTRAINT_DESTRUCT,
};

/// The wheel constraint's vtable and live counter, its cinfo block's vtable, and the data
/// block's constructor (`00d4edc0`, thiscall on the block, answers it).
const WHEEL_VTABLE: u32 = 0x010c_9a14;
const WHEEL_COUNT: u32 = 0x0126_85e8;
const WHEEL_CLASSES: &[(u32, u32)] = &[(WHEEL_VTABLE, WHEEL_COUNT)];
const WHEEL_CINFO_BLOCK_VTABLE: u32 = 0x010c_9af0;
const WHEEL_DATA_CONSTRUCT: u32 = 0x00d4_edc0;
/// Size of the data block.
const DATA_SIZE: u32 = 0x160;
/// The data fields the cinfo reads and writes, as `(offset, size)` in stream order.
const STREAMED_FIELDS: [(u32, u32); 10] = [
    (0x20, 0x40),
    (0x60, 0x40),
    (0xd0, 0x30),
    (0x100, 0x30),
    (0x140, 0x10),
    (0x150, 0x10),
    (0xa4, 4),
    (0xa8, 4),
    (0xb0, 4),
    (0xb4, 4),
];

/// Moves the data fields (`STREAMED_FIELDS`) of the holder `this` between its data block and
/// the stream through the stream's endpoint at `stream + endpoint` (function at
/// `function_offset`), each with a length cell initialised to the field size and flag 1.
fn transfer_fields(e: &mut Engine, this: Ptr, stream: u32, endpoint: u32, function_offset: u32) {
    let data = e.mem.u32(this.addr() + 4);
    for (offset, size) in STREAMED_FIELDS {
        let object = e.mem.u32(stream + endpoint);
        stream_transfer(
            e,
            object,
            function_offset,
            data + offset,
            size,
            Some(size),
            1,
        );
    }
}

/// `MULPS` of a broadcast `scale` by the four floats at `address`, stored back.
fn scale_vector(e: &mut Engine, address: u32, scale: f32) {
    for lane in 0..4 {
        let value = e.mem.f32(address + 4 * lane);
        e.mem.set_f32(address + 4 * lane, scale * value);
    }
}

// Translated from 00cc5090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5090 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWheelConstraint::~bhkWheelConstraint` (Xbox PDB): installs the `bhkWheelConstraint`
/// vtable, runs `bhkSerializable::Kill`, decrements the live wheel constraint counter and runs
/// the `bhkConstraint` destructor body.
pub fn bhk_wheel_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), WHEEL_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(WHEEL_COUNT);
    e.mem.set_u32(WHEEL_COUNT, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00cc5110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWheelConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc5090`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_wheel_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_5090;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc5100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWheelConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85ec.
pub fn bhk_wheel_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85ec
}

// Translated from 00cc5130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5130 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWheelConstraint` factory (no PDB name): allocates 0x10 bytes from the game heap and
/// constructs a wheel constraint in them. Answers the object, or 0 when the allocation failed
/// (takes no arguments).
pub fn fn_00cc5130(e: &mut Engine) -> u32 {
    new_constraint(e, WHEEL_CLASSES)
}

// Translated from 00cc51c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed constructor of the wheel constraint's cinfo block (the code `CreateCinfo` inlines):
/// clears `+4`, `+0xC` and `+0x10`, sets the reference count `+8` to 1 and the vtable
/// `010c9af0`. Answers `this`.
pub fn fn_00cc51c0(e: &mut Engine, this: Ptr) -> Ptr {
    let block = this.addr();
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 1);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0);
    e.mem.set_u32(block, WHEEL_CINFO_BLOCK_VTABLE);
    this
}

// Translated from 00cc51e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc51e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkWheelConstraintCinfo::CloneData` (Xbox PDB): with the allocation-scope word at 0x10,
/// uses `data` as the destination or, when it is 0, builds a new data block (0x160 bytes from
/// the current Havok allocator and its constructor `00d4edc0`). Copies the vectors at data
/// `+0x50` and `+0x90` and the floats at `+0xA4`, `+0xA8`, `+0xB0` and `+0xB4` of this holder's
/// block (the floats go through the x87 stack, which quiets signalling NaNs), then, when the
/// float at `context + 0x10` is not exactly 1.0 (a NaN counts as different), multiplies the
/// two vectors in the copy by it. Answers the base `CloneData` (`00c8a1b0`, `(this, data,
/// context)`).
pub fn hk_wheel_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    context: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let source = e.mem.u32(this.addr() + 4);
    let data = if data == 0 {
        let block = havok_allocate(e, DATA_SIZE);
        e.call(WHEEL_DATA_CONSTRUCT, &args![block]).u32()
    } else {
        data
    };
    for vector in [0x50, 0x90] {
        for lane in 0..4 {
            let value = e.mem.u32(source + vector + 4 * lane);
            e.mem.set_u32(data + vector + 4 * lane, value);
        }
    }
    for offset in [0xa4, 0xa8, 0xb0, 0xb4] {
        let value = x87_quiet_bits(e.mem.u32(source + offset));
        e.mem.set_u32(data + offset, value);
    }
    let scale = f32::from_bits(x87_quiet_bits(e.mem.u32(context + 0x10)));
    if scale != 1.0 {
        scale_vector(e, data + 0x50, scale);
        scale_vector(e, data + 0x90, scale);
    }
    let result = e.call(CINFO_CLONE_DATA, &args![this, data, context]).u32();
    e.mem.set_u32(word, saved);
    result
}

// Translated from 00cc5340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5340 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkWheelConstraintCinfo::CreateData` (Xbox PDB): builds the holder's data block when it has
/// none (`create_data_block`: 0x160 bytes and the constructor `00d4edc0`). Answers the size,
/// 0x160.
pub fn hk_wheel_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    create_data_block(e, this, DATA_SIZE, WHEEL_DATA_CONSTRUCT);
    DATA_SIZE
}

// Translated from 00cc53f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc53f0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWheelConstraint::CreateClone` (Xbox PDB): a new wheel constraint filled from `this`
/// (`clone_constraint`). Answers the clone.
pub fn bhk_wheel_constraint_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    clone_constraint(e, this, context, WHEEL_CLASSES)
}

// Translated from 00cc54d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc54d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWheelConstraint::CreateCinfo` (Xbox PDB): `create_cinfo_block` with the wheel cinfo
/// block vtable `010c9af0`. `created` is the address of the byte that receives whether a new
/// block was made.
pub fn bhk_wheel_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    create_cinfo_block(e, this, created, 0x14, WHEEL_CINFO_BLOCK_VTABLE)
}

// Translated from 00cc55a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkWheelConstraintCinfo::LoadBinary` (Xbox PDB): the base cinfo load, then the ten data
/// fields (0x40 at `+0x20` and `+0x60`, 0x30 at `+0xD0` and `+0x100`, 0x10 at `+0x140` and
/// `+0x150`, four bytes each at `+0xA4`, `+0xA8`, `+0xB0` and `+0xB4`) read through the
/// stream's reader (`stream + 0x24C`, function at `+8`).
pub fn hk_wheel_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    transfer_fields(e, this, stream, 0x24c, 8);
}

// Translated from 00cc5730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkWheelConstraintCinfo::SaveBinary` (Xbox PDB): the base cinfo save, then the same ten
/// data fields as `LoadBinary` written through the stream's writer (`stream + 0x250`,
/// function at `+0xC`).
pub fn hk_wheel_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
    transfer_fields(e, this, stream, 0x250, 0xc);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc5090, bhk_wheel_constraint_destruct(Ptr)),
        entry!(0x00cc5100, bhk_wheel_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cc5110, bhk_wheel_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc5130, fn_00cc5130() -> u32),
        entry!(0x00cc51c0, fn_00cc51c0(Ptr) -> Ptr),
        entry!(
            0x00cc51e0,
            hk_wheel_constraint_cinfo_clone_data(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00cc5340,
            hk_wheel_constraint_cinfo_create_data(Ptr) -> u32
        ),
        entry!(
            0x00cc53f0,
            bhk_wheel_constraint_create_clone(Ptr, u32) -> u32
        ),
        entry!(
            0x00cc54d0,
            bhk_wheel_constraint_create_cinfo(Ptr, u32) -> u32
        ),
        entry!(0x00cc55a0, hk_wheel_constraint_cinfo_load_binary(Ptr, u32)),
        entry!(0x00cc5730, hk_wheel_constraint_cinfo_save_binary(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::{
        CLONE_LOCK, CLONE_UNLOCK, HEAP_ALLOCATE, MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE,
        SCOPE_WORD,
    };
    use super::super::bhkhingeconstraint::testing::*;
    use super::super::bhkhingeconstraint::{
        CONSTRAINT_ADOPT_CINFO, CONSTRAINT_CONSTRUCT, CONSTRAINT_COPY_MEMBERS, CONSTRAINT_COUNT,
    };
    use super::*;

    #[test]
    fn bhk_wheel_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_5090, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc5110, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc5110, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_5090), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_wheel_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_5100, &args![]).u32(), 0x0126_85ec);
    }

    #[test]
    fn destructor_installs_the_vtable_and_counts_the_wheel_down() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[WHEEL_COUNT]);
        e.mem.set_u32(WHEEL_COUNT, 5);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_5090, &args![this]);
        assert_eq!(e.mem.u32(this), WHEEL_VTABLE);
        assert_eq!(e.mem.u32(WHEEL_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONSTRAINT_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn factory_builds_a_wheel_constraint_or_nothing() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[WHEEL_COUNT]);
        allocator_doubles(&mut e);
        let object = e.call(0x00cc_5130, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), WHEEL_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 1);
        assert_eq!(e.mem.u32(WHEEL_COUNT), 1);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, CONSTRAINT_CONSTRUCT), vec![vec![object]]);
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_5130, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(WHEEL_COUNT), 1);
    }

    #[test]
    fn cinfo_block_constructor_sets_the_count_and_the_vtable() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x14);
        for offset in (0..0x14).step_by(4) {
            e.mem.set_u32(block + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00cc_51c0, &args![block]).u32(), block);
        assert_eq!(e.mem.u32(block), WHEEL_CINFO_BLOCK_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
    }

    #[test]
    fn clone_data_copies_the_fields_and_scales_the_vectors_unless_exactly_one() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(WHEEL_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CINFO_CLONE_DATA, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let (holder, source) = holder_with_data(&mut e, 0x160);
        for lane in 0..4 {
            e.mem.set_f32(source + 0x50 + 4 * lane, 1.0 + lane as f32);
            e.mem.set_f32(source + 0x90 + 4 * lane, 10.0 + lane as f32);
        }
        // A signalling NaN in one of the floats the x87 stack copies.
        e.mem.set_u32(source + 0xa8, 0x7f80_0001);
        let context = e.mem.alloc(0x20);
        let destination = e.mem.alloc(0x160);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.mem.set_f32(context + 0x10, 1.0);
        let call_args = args![holder, destination, context];
        assert_eq!(e.call(0x00cc_51e0, &call_args).u32(), 0x55);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(e.mem.f32(destination + 0x5c), 4.0);
        assert_eq!(e.mem.f32(destination + 0x90), 10.0);
        assert_eq!(e.mem.u32(destination + 0xa4), e.mem.u32(source + 0xa4));
        assert_eq!(e.mem.u32(destination + 0xa8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(destination + 0xb4), e.mem.u32(source + 0xb4));
        // Words that are not copied stay as they were.
        assert_eq!(e.mem.u32(destination + 0xac), 0);
        assert_eq!(e.mem.u32(destination + 0xb8), 0);
        assert_eq!(
            calls_to(&e, CINFO_CLONE_DATA),
            vec![vec![holder, destination, context]]
        );
        // A scale of 2 doubles both vectors, and only them.
        e.mem.set_f32(context + 0x10, 2.0);
        e.call(0x00cc_51e0, &call_args);
        assert_eq!(e.mem.f32(destination + 0x50), 2.0);
        assert_eq!(e.mem.f32(destination + 0x5c), 8.0);
        assert_eq!(e.mem.f32(destination + 0x9c), 26.0);
        assert_eq!(e.mem.u32(destination + 0xa4), e.mem.u32(source + 0xa4));
        // A NaN scale differs from 1.0 and spreads into the vectors.
        e.mem.set_u32(context + 0x10, 0x7fc0_0000);
        e.call(0x00cc_51e0, &call_args);
        assert!(e.mem.f32(destination + 0x50).is_nan());
        // No destination: a new block comes from the Havok allocator and its constructor.
        e.mem.set_f32(context + 0x10, 1.0);
        let result = e.call(0x00cc_51e0, &args![holder, 0u32, context]).u32();
        assert_eq!(result, 0x55);
        let made = calls_to(&e, CINFO_CLONE_DATA)[3][1];
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, WHEEL_DATA_CONSTRUCT), vec![vec![made]]);
        assert_eq!(e.mem.u16(made + 4), 0x160);
        assert_eq!(e.mem.f32(made + 0x54), 2.0);
    }

    #[test]
    fn create_data_builds_the_block_once() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(WHEEL_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        let holder = e.mem.alloc(0x20);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        assert_eq!(e.call(0x00cc_5340, &args![holder]).u32(), 0x160);
        let block = calls_to(&e, WHEEL_DATA_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(holder + 4), block + 0x10);
        assert_eq!(e.mem.u16(block + 4), 0x160);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(e.call(0x00cc_5340, &args![holder]).u32(), 0x160);
        assert_eq!(calls_to(&e, WHEEL_DATA_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_builds_a_wheel_under_the_clone_lock() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[WHEEL_COUNT]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        let clone = e.call(0x00cc_53f0, &args![this, 0x1234u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), WHEEL_VTABLE);
        assert_eq!(e.mem.u32(WHEEL_COUNT), 1);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS),
            vec![vec![this, clone, 0x1234]]
        );
        assert_eq!(calls_to(&e, CLONE_LOCK).len(), 1);
        assert_eq!(calls_to(&e, CLONE_UNLOCK).len(), 1);
        // A failed allocation still goes through the copy, with a null clone.
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_53f0, &args![this, 0x1234u32]).u32(), 0);
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
        let cinfo = e.call(0x00cc_54d0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        let block = cinfo - 4;
        assert_eq!(e.mem.u32(block), WHEEL_CINFO_BLOCK_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
        assert!(calls_to(&e, CONSTRAINT_ADOPT_CINFO).is_empty());
        // Already there: reported as not created and returned unchanged.
        assert_eq!(e.call(0x00cc_54d0, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        // With a holder the new block is handed to the base routine.
        e.mem.set_u32(this + 8, 0x77);
        e.mem.set_u32(this + 0xc, 0);
        let cinfo = e.call(0x00cc_54d0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(
            calls_to(&e, CONSTRAINT_ADOPT_CINFO),
            vec![vec![this, cinfo - 4]]
        );
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // A failed allocation leaves the block missing and hands over null.
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(e.call(0x00cc_54d0, &args![this, created]).u32(), 0);
        assert_eq!(calls_to(&e, CONSTRAINT_ADOPT_CINFO)[1], vec![this, 0]);
    }

    #[test]
    fn load_binary_reads_the_ten_fields_after_the_base_load() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        let (stream, reader, _) = stream_with_endpoints(&mut e);
        let (holder, data) = holder_with_data(&mut e, 0x160);
        e.call(0x00cc_55a0, &args![holder, stream]);
        assert_eq!(calls_to(&e, CINFO_LOAD_BINARY), vec![vec![holder, stream]]);
        let reads: Vec<(u32, u32, u32, u32)> = calls_to(&e, 0x0900_0e08)
            .iter()
            .map(|c| (c[0], c[1] - data, c[2], c[4]))
            .collect();
        assert_eq!(
            reads,
            STREAMED_FIELDS
                .iter()
                .map(|&(offset, size)| (reader, offset, size, 1))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            e.mem.u32(reader + 4),
            0x40 * 2 + 0x30 * 2 + 0x10 * 2 + 4 * 4
        );
    }

    #[test]
    fn save_binary_writes_the_ten_fields_after_the_base_save() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        let (stream, _, writer) = stream_with_endpoints(&mut e);
        let (holder, data) = holder_with_data(&mut e, 0x160);
        e.call(0x00cc_5730, &args![holder, stream]);
        assert_eq!(calls_to(&e, CINFO_SAVE_BINARY), vec![vec![holder, stream]]);
        let writes: Vec<(u32, u32, u32, u32)> = calls_to(&e, 0x0900_0e0c)
            .iter()
            .map(|c| (c[0], c[1] - data, c[2], c[4]))
            .collect();
        assert_eq!(
            writes,
            STREAMED_FIELDS
                .iter()
                .map(|&(offset, size)| (writer, offset, size, 1))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            e.mem.u32(writer + 4),
            0x40 * 2 + 0x30 * 2 + 0x10 * 2 + 4 * 4
        );
    }
}
