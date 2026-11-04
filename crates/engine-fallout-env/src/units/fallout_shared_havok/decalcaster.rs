//! `fallout shared/havok/decalcaster.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The decal caster keeps its phantom in a smart pointer at `+0x3a0`; every function reads it
//! through `00559450` (identical-code folding gave one body to many accessors, so the engine map
//! has no name for it) and `004ae750`.

#[allow(unused_imports)]
use crate::prelude::*;

/// Block allocator `00aa13e0` (cdecl, one size word).
const ALLOCATE_OBJECT: u32 = 0x00aa_13e0;
/// `0056e950` (Ghidra names it `bhkSphereShape::bhkSphereShape`, unconfirmed): `thiscall`,
/// `RET 8`, a `float` and a flag.
const SPHERE_SHAPE_CONSTRUCT: u32 = 0x0056_e950;
/// `0056e090` (unnamed in the engine map): `thiscall`, plain `RET`; constructs the 0x60-byte
/// record `00622d60` keeps on its stack.
const RECORD_CONSTRUCT: u32 = 0x0056_e090;
/// `0056e120` (unnamed in the engine map): `thiscall`, plain `RET`; run on `record + 0x20`.
const RECORD_MEMBER_CONSTRUCT: u32 = 0x0056_e120;
/// `0056ea90` (unnamed in the engine map): `thiscall`, plain `RET`; the record's destructor.
const RECORD_DESTRUCT: u32 = 0x0056_ea90;
/// `008c71b0` (unnamed in the engine map): `thiscall`, `RET 4`; constructs the 4-byte word
/// `00622d60` keeps on its stack.
const FILTER_CONSTRUCT: u32 = 0x008c_71b0;
/// `004a39f0` (unnamed in the engine map): `thiscall`, `RET 4`; run on that word.
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
/// `0059ce80` (unnamed in the engine map): `thiscall`, `RET 4`; run on that word.
const FILTER_SET_SOURCE: u32 = 0x0059_ce80;
/// `00559450` (unnamed in the engine map): `thiscall`, plain `RET`, no stack words: answers
/// what a smart pointer or a small word holds.
const HOLDER_GET: u32 = 0x0055_9450;
/// `004ae750` (unnamed in the engine map): `thiscall`, plain `RET`: the pointer a smart pointer
/// holds.
const POINTER_GET: u32 = 0x004a_e750;
/// `0056e2d0` (Ghidra names it `bhkSimpleShapePhantom::bhkSimpleShapePhantom`, unconfirmed):
/// `thiscall`, `RET 4`.
const PHANTOM_CONSTRUCT: u32 = 0x0056_e2d0;
/// `0066b0d0` (unnamed in the engine map): `thiscall`, `RET 4`; assigns the phantom to the smart
/// pointer at `this + 0x3a0`.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `00c82870` (Ghidra names it `bhkCollisionFilter::SetMultiLayerCollision`, unconfirmed):
/// cdecl, three words.
const SET_MULTI_LAYER_COLLISION: u32 = 0x00c8_2870;
/// `00a29680` (unnamed in the engine map): `thiscall`, plain `RET`; run on the smart pointer
/// before and after a call through the pointer it holds.
const POINTER_SCOPE: u32 = 0x00a2_9680;
/// `00cb0c10` (unnamed in the engine map): `thiscall`, `RET 4`; run on `object + 0x74`.
const NAME_ASSIGN: u32 = 0x00cb_0c10;
/// `00621290` (unnamed in the engine map): `thiscall`, plain `RET`; constructs the 0x20-byte
/// record `00622f80` keeps at `-0x40` of its frame.
const MEMBER_A_SETUP: u32 = 0x0062_1290;
/// `004a3e00` (unnamed in the engine map): cdecl, destination and source.
const COPY_RECORD: u32 = 0x004a_3e00;
/// `006815c0` (unnamed in the engine map): `thiscall`, plain `RET`.
const MEMBER_SETUP: u32 = 0x0068_15c0;
/// `00c9eba0` (unnamed in the engine map): `thiscall`, four stack words.
const CAST_THROUGH_HOLDER: u32 = 0x00c9_eba0;
/// `00621210` (unnamed in the engine map): `thiscall`, `RET 4`; takes the address of a byte.
const RESULT_HOLDER: u32 = 0x0062_1210;
/// `005609f0` (unnamed in the engine map): `thiscall`, plain `RET`; answers a byte in `AL`.
const RESULT_BYTE: u32 = 0x0056_09f0;
/// `00460140` (unnamed in the engine map): `thiscall`, plain `RET`.
const LIST_OF: u32 = 0x0046_0140;
/// `00726070` (unnamed in the engine map): `thiscall`, plain `RET`; the dword at `this + 4`.
const LIST_LINK: u32 = 0x0072_6070;
/// `006219f0` (unnamed in the engine map): `thiscall`, `RET 4`.
const RECORD_CONSUME: u32 = 0x0062_19f0;
/// `0044ddc0` (unnamed in the engine map): `thiscall`, no stack words.
const NODE_KEY: u32 = 0x0044_ddc0;
/// `00cabb00` (Ghidra names it `hkColor::rgbFromChars`, unconfirmed): cdecl, four words.
const COLOR_FROM_BYTES: u32 = 0x00ca_bb00;
/// `00c68e30` (Ghidra names it `bhkWorld::SetDebugObjectColor`, unconfirmed): `thiscall`,
/// `RET 8`.
const SET_DEBUG_COLOR: u32 = 0x00c6_8e30;
/// `00621480` (unnamed in the engine map): cdecl, one word.
const FIRST_OF: u32 = 0x0062_1480;
/// `004ae6c0` (unnamed in the engine map): `thiscall`, plain `RET`; answers a pointer.
const OWNER_OF: u32 = 0x004a_e6c0;
/// `004a3e90` (unnamed in the engine map): cdecl, one `float`; answers a value in `ST0`.
const CONVERT_VALUE: u32 = 0x004a_3e90;
/// `00621b20` (unnamed in the engine map): `thiscall`, `RET 4`; takes a `float`.
const OWNER_SET_VALUE: u32 = 0x0062_1b20;
/// `006233c0` (`run_00623370.rs`): `thiscall`, `RET 4`.
const CASTER_RESET: u32 = 0x0062_33c0;
/// The string `"DecalCaster"` the initializer names the phantom with.
const NAME_STRING: u32 = 0x0104_dcf0;
/// The double the radius of `00622f80` is compared with (0.0).
const ZERO: u32 = 0x0101_2060;

/// Offset of the smart pointer that holds the phantom.
const PHANTOM_OFFSET: u32 = 0x3a0;

// Translated from 00622d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00622d60 - C++ exception unwinding (FS:[0] frame) not translated
/// `DecalCaster::Init` (engine map name). `thiscall`, `RET 8` (a `float` and a word), the stack
/// realigned to 16 bytes. Builds a sphere shape of the radius (0x14 bytes from `00aa13e0`,
/// `0056e950` with the radius and 1), fills the 0x60-byte record kept on the stack (its first word
/// is what `00559450` answers for a 4-byte word built with `008c71b0`, layer `0x27` through
/// `004a39f0` and `source` through `0059ce80`; its second word the pointer `004ae750` answers for
/// the shape; `0056e120` on `record + 0x20`), makes the phantom from it (0x18 bytes,
/// `0056e2d0`), stores it in the smart pointer at `this + 0x3a0` (`0066b0d0`), names the held
/// object `"DecalCaster"` (`00622f20`), sets the multi-layer collision of layer `0x27`
/// (`00c82870` with `0x1fe3f637` and `0xfffffd47`) and destroys the record.
pub fn decal_caster_init(e: &mut Engine, this: Ptr, radius: f32, source: u32) {
    let shape_block = e.call(ALLOCATE_OBJECT, &args![0x14u32]).u32();
    let shape = if shape_block == 0 {
        0
    } else {
        // The radius travels through the x87 stack (`FLD`/`FSTP`).
        e.call(
            SPHERE_SHAPE_CONSTRUCT,
            &args![shape_block, x87_quiet(radius), 1u32],
        )
        .u32()
    };
    // The exe's frame: `EBP` on a 16-byte boundary (`AND ESP,0xfffffff0`), 0xb4 bytes below it;
    // the record at `EBP-0x90` and the filter at `EBP-0x94`.
    e.with_stack(0xb4 + 0x10, |e, raw| {
        let ebp = (raw.addr() + 0xb4 + 0xf) & !0xf;
        let record = ebp - 0x90;
        e.call(RECORD_CONSTRUCT, &args![record]);
        let filter = ebp - 0x94;
        e.call(FILTER_CONSTRUCT, &args![filter, 0u32]);
        e.call(FILTER_SET_LAYER, &args![filter, 0x27u32]);
        e.call(FILTER_SET_SOURCE, &args![filter, source]);
        let info = e.call(HOLDER_GET, &args![filter]).u32();
        e.mem.set_u32(record, info);
        let shape_pointer = e.call(POINTER_GET, &args![shape]).u32();
        e.mem.set_u32(record.wrapping_add(4), shape_pointer);
        e.call(RECORD_MEMBER_CONSTRUCT, &args![record.wrapping_add(0x20)]);
        let phantom_block = e.call(ALLOCATE_OBJECT, &args![0x18u32]).u32();
        let phantom = if phantom_block == 0 {
            0
        } else {
            e.call(PHANTOM_CONSTRUCT, &args![phantom_block, record])
                .u32()
        };
        let holder = this.addr().wrapping_add(PHANTOM_OFFSET);
        e.call(POINTER_ASSIGN, &args![holder, phantom]);
        let held = e.call(HOLDER_GET, &args![holder]).u32();
        fn_00622f20(e, Ptr::new(held), NAME_STRING);
        e.call(
            SET_MULTI_LAYER_COLLISION,
            &args![0x27u32, 0x1fe3_f637u32, 0xffff_fd47u32],
        );
        e.call(RECORD_DESTRUCT, &args![record]);
    });
}

// Translated from 00622f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. Asks the smart pointer `this` for its pointer
/// (`004ae750`); when there is one, brackets `00622f60` (given `name`) on it with `00a29680` on
/// `this` before and after.
pub fn fn_00622f20(e: &mut Engine, this: Ptr, name: u32) {
    let held = e.call(POINTER_GET, &args![this]).u32();
    if held != 0 {
        e.call(POINTER_SCOPE, &args![this]);
        fn_00622f60(e, Ptr::new(held), name);
        e.call(POINTER_SCOPE, &args![this]);
    }
}

// Translated from 00622f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: hands `value` to `00cb0c10` on `this + 0x74`.
pub fn fn_00622f60(e: &mut Engine, this: Ptr, value: u32) {
    e.call(NAME_ASSIGN, &args![this.addr().wrapping_add(0x74), value]);
}

// Translated from 00623070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`, plain `RET`: asks `00460140` on `this` and answers what
/// `00726070` says on its result (the exe leaves that value in `EAX`).
pub fn fn_00623070(e: &mut Engine, this: Ptr) -> u32 {
    let list = e.call(LIST_OF, &args![this]).u32();
    e.call(LIST_LINK, &args![list]).u32()
}

// Translated from 00622f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00622f80 - /GS stack cookie check (00ec408c) not translated
/// Unnamed in the engine map. `thiscall`, `RET 0xC`, the stack realigned to 16 bytes. When
/// `value` is not 0.0 (the compare is ordered: a NaN counts as not zero), hands it to
/// `00623310`; then runs `006233c0` with 0 and the object's vtable slot `+8`. When the smart
/// pointer at `this + 0x3a0` holds something, builds two 0x20-byte records on the stack
/// (`006815c0`, `00621290`), copies `first` and `second` into them (`004a3e00`), runs `00c9eba0`
/// on the held object with both records, `this` and 0, and asks `005609f0` on what `00621210`
/// answers for a stack byte (the byte it gets is not read afterwards). Ends with `00623070` on
/// `this`; its answer is left in `EAX` (returned here).
pub fn fn_00622f80(e: &mut Engine, this: Ptr, first: u32, second: u32, value: f32) -> u32 {
    // The stack frame: `EBP` on a 16-byte boundary (`AND ESP,0xfffffff0`), 0x50 bytes below it;
    // `frame` is `EBP-0x44`: the byte at `-0x41`, the record at `-0x40` and the record at `-0x20`.
    e.with_stack(0x50 + 0x10, |e, raw| {
        let frame = ((raw.addr() + 0x50 + 0xf) & !0xf) - 0x44;
        let second_record = frame.wrapping_add(4);
        let first_record = frame.wrapping_add(0x24);
        let zero: f64 = e.global(ZERO);
        if f64::from(x87_quiet(value)) != zero {
            fn_00623310(e, this, value);
        }
        e.call(CASTER_RESET, &args![this, 0u32]);
        e.vcall(this.addr(), 8, &args![]);
        let holder = this.addr().wrapping_add(PHANTOM_OFFSET);
        if e.call(HOLDER_GET, &args![holder]).u32() != 0 {
            e.call(MEMBER_SETUP, &args![first_record]);
            e.call(MEMBER_A_SETUP, &args![second_record]);
            e.call(COPY_RECORD, &args![first_record, first]);
            e.call(COPY_RECORD, &args![second_record, second]);
            let held = e.call(HOLDER_GET, &args![holder]).u32();
            e.call(
                CAST_THROUGH_HOLDER,
                &args![held, first_record, second_record, this, 0u32],
            );
            let result = e
                .call(RESULT_HOLDER, &args![this, frame.wrapping_add(3)])
                .u32();
            // The byte the exe keeps in `-9` is never read again.
            e.call(RESULT_BYTE, &args![result]);
        }
        fn_00623070(e, this)
    })
}

// Translated from 00623090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. When the smart pointer at `this + 0x3a0`
/// holds something: runs `006230f0` (with `first`) on it, the vtable slot `+8` of `second`, and
/// `00623150` (with `second`) on the held object.
pub fn fn_00623090(e: &mut Engine, this: Ptr, first: u32, second: Ptr) {
    let holder = this.addr().wrapping_add(PHANTOM_OFFSET);
    if e.call(HOLDER_GET, &args![holder]).u32() != 0 {
        let held = e.call(HOLDER_GET, &args![holder]).u32();
        fn_006230f0(e, Ptr::new(held), first);
        e.vcall(second.addr(), 8, &args![]);
        let held = e.call(HOLDER_GET, &args![holder]).u32();
        fn_00623150(e, Ptr::new(held), second.addr());
    }
}

// Translated from 006230f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006230f0 - /GS stack cookie check (00ec408c) not translated
/// Unnamed in the engine map. `thiscall`, `RET 4`, the stack realigned to 16 bytes. Builds a
/// 0x1c-byte record on the stack (`006815c0`), copies `source` into it (`004a3e00`) and hands
/// what that answers to `006219f0` on `this`.
pub fn fn_006230f0(e: &mut Engine, this: Ptr, source: u32) {
    // The record at `EBP-0x20`, `EBP` on a 16-byte boundary (`AND ESP,0xfffffff0`, `SUB ESP,0x20`).
    e.with_stack(0x20 + 0x10, |e, raw| {
        let record = ((raw.addr() + 0x20 + 0xf) & !0xf) - 0x20;
        e.call(MEMBER_SETUP, &args![record]);
        let copied = e.call(COPY_RECORD, &args![record, source]).u32();
        e.call(RECORD_CONSUME, &args![this, copied]);
    });
}

// Translated from 00623150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. Asks the smart pointer `this` for its pointer
/// (`004ae750`); when there is one and its key (`0044ddc0`) is not zero, calls the pointer's
/// vtable slot `+0x44` with `target` and 0, bracketed by `00a29680` on `this`.
pub fn fn_00623150(e: &mut Engine, this: Ptr, target: u32) {
    let held = e.call(POINTER_GET, &args![this]).u32();
    if held != 0 && e.call(NODE_KEY, &args![held]).u32() != 0 {
        e.call(POINTER_SCOPE, &args![this]);
        e.vcall(held, 0x44, &args![target, 0u32]);
        e.call(POINTER_SCOPE, &args![this]);
    }
}

// Translated from 006231a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DecalCaster::Add` (engine map name). `thiscall`, `RET 4`. When the smart pointer at
/// `this + 0x3a0` holds something and `00623230` does not already answer `target`, calls the
/// held object's vtable slot `+0x9c` with `target`; for a non-null `target` it then makes a colour
/// (`00cabb00` with `0xff`, `0x10`, `0x10`, `0x19`) and hands the held object and the colour to
/// `00c68e30` on `target`.
pub fn decal_caster_add(e: &mut Engine, this: Ptr, target: u32) {
    let holder = this.addr().wrapping_add(PHANTOM_OFFSET);
    if e.call(HOLDER_GET, &args![holder]).u32() != 0 && fn_00623230(e, this) != target {
        let held = e.call(HOLDER_GET, &args![holder]).u32();
        e.vcall(held, 0x9c, &args![target]);
        if target != 0 {
            let color = e
                .call(COLOR_FROM_BYTES, &args![0xffu32, 0x10u32, 0x10u32, 0x19u32])
                .u32();
            let held = e.call(HOLDER_GET, &args![holder]).u32();
            e.call(SET_DEBUG_COLOR, &args![target, held, color]);
        }
    }
}

// Translated from 00623230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, plain `RET`. When the smart pointer at `this + 0x3a0`
/// holds something, calls the held object's vtable slot `+0x94` and answers what `00621480` says
/// about its result; otherwise answers 0.
pub fn fn_00623230(e: &mut Engine, this: Ptr) -> u32 {
    let holder = this.addr().wrapping_add(PHANTOM_OFFSET);
    let mut answer = 0;
    if e.call(HOLDER_GET, &args![holder]).u32() != 0 {
        let held = e.call(HOLDER_GET, &args![holder]).u32();
        let value = e.vcall(held, 0x94, &args![]).u32();
        answer = e.call(FIRST_OF, &args![value]).u32();
    }
    answer
}

// Translated from 00623310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`, a `float`. When the smart pointer at
/// `this + 0x3a0` holds something whose owner (`004ae6c0`) is not null, converts the value
/// (`004a3e90`, the answer in `ST0`) and hands it, as a `float`, to `00621b20` on the owner.
pub fn fn_00623310(e: &mut Engine, this: Ptr, value: f32) {
    let holder = this.addr().wrapping_add(PHANTOM_OFFSET);
    if e.call(HOLDER_GET, &args![holder]).u32() != 0 {
        let held = e.call(HOLDER_GET, &args![holder]).u32();
        let owner = e.call(OWNER_OF, &args![held]).u32();
        if owner != 0 {
            let converted = e.call(CONVERT_VALUE, &args![x87_quiet(value)]).f64();
            e.call(OWNER_SET_VALUE, &args![owner, x87_store32(converted)]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_2d60, decal_caster_init(Ptr, f32, u32)),
        entry!(0x0062_2f20, fn_00622f20(Ptr, u32)),
        entry!(0x0062_2f60, fn_00622f60(Ptr, u32)),
        entry!(0x0062_2f80, fn_00622f80(Ptr, u32, u32, f32) -> u32),
        entry!(0x0062_3070, fn_00623070(Ptr) -> u32),
        entry!(0x0062_3090, fn_00623090(Ptr, u32, Ptr)),
        entry!(0x0062_30f0, fn_006230f0(Ptr, u32)),
        entry!(0x0062_3150, fn_00623150(Ptr, u32)),
        entry!(0x0062_31a0, decal_caster_add(Ptr, u32)),
        entry!(0x0062_3230, fn_00623230(Ptr) -> u32),
        entry!(0x0062_3310, fn_00623310(Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const HELD: u32 = 0x0600_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Every address of the unit's callees answers 0 and does nothing, the constant `0.0` is
    /// mapped, and the calls are logged.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for addr in [
            ALLOCATE_OBJECT,
            SPHERE_SHAPE_CONSTRUCT,
            RECORD_CONSTRUCT,
            RECORD_MEMBER_CONSTRUCT,
            RECORD_DESTRUCT,
            FILTER_CONSTRUCT,
            FILTER_SET_LAYER,
            FILTER_SET_SOURCE,
            HOLDER_GET,
            POINTER_GET,
            PHANTOM_CONSTRUCT,
            POINTER_ASSIGN,
            SET_MULTI_LAYER_COLLISION,
            POINTER_SCOPE,
            NAME_ASSIGN,
            MEMBER_A_SETUP,
            COPY_RECORD,
            MEMBER_SETUP,
            CAST_THROUGH_HOLDER,
            RESULT_HOLDER,
            RESULT_BYTE,
            LIST_OF,
            LIST_LINK,
            RECORD_CONSUME,
            NODE_KEY,
            COLOR_FROM_BYTES,
            SET_DEBUG_COLOR,
            FIRST_OF,
            OWNER_OF,
            CONVERT_VALUE,
            OWNER_SET_VALUE,
            CASTER_RESET,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(ZERO & !0xfff, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// An object whose vtable has the given slots (byte offset, function address).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
            e.register(*target, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The addresses of the logged calls, in order, without the top-level one.
    fn order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|(a, _)| *a)
            .collect()
    }

    #[test]
    fn at_00622d60_builds_the_phantom_and_names_it() {
        let mut e = rig();
        e.register(ALLOCATE_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SPHERE_SHAPE_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        e.register(HOLDER_GET, |_, a| (a[0] ^ 0x8000_0000).into_ret());
        e.register(POINTER_GET, |_, a| (a[0] + 2).into_ret());
        let record_seen = Rc::new(RefCell::new(vec![]));
        let seen = Rc::clone(&record_seen);
        e.register_double(PHANTOM_CONSTRUCT, move |e, a| {
            let mut s = seen.borrow_mut();
            s.push(e.mem.u32(a[1]));
            s.push(e.mem.u32(a[1] + 4));
            (a[0] + 3).into_ret()
        });
        let this = e.mem.alloc(0x400);
        e.call(0x0062_2d60, &args![this, 2.5f32, 0x77u32]);
        let sphere = calls(&e, SPHERE_SHAPE_CONSTRUCT);
        assert_eq!(sphere.len(), 1);
        assert_eq!(&sphere[0][1..], &[2.5f32.to_bits(), 1]);
        let shape = sphere[0][0] + 1;
        let filter = calls(&e, FILTER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, FILTER_CONSTRUCT), vec![vec![filter, 0]]);
        assert_eq!(calls(&e, FILTER_SET_LAYER), vec![vec![filter, 0x27]]);
        assert_eq!(calls(&e, FILTER_SET_SOURCE), vec![vec![filter, 0x77]]);
        // The record: first word what the filter word answers, second the shape's pointer.
        assert_eq!(*record_seen.borrow(), vec![filter ^ 0x8000_0000, shape + 2]);
        let record = calls(&e, RECORD_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, RECORD_MEMBER_CONSTRUCT),
            vec![vec![record + 0x20]]
        );
        assert_eq!(calls(&e, RECORD_DESTRUCT), vec![vec![record]]);
        let phantom = calls(&e, PHANTOM_CONSTRUCT)[0][0] + 3;
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![this + 0x3a0, phantom]]);
        let allocations: Vec<u32> = calls(&e, ALLOCATE_OBJECT).iter().map(|w| w[0]).collect();
        assert_eq!(allocations, vec![0x14, 0x18]);
        // The held object (what the pointer answers) is the one that gets the name.
        let held = (this + 0x3a0) ^ 0x8000_0000;
        assert_eq!(calls(&e, POINTER_GET)[1], vec![held]);
        assert_eq!(
            calls(&e, SET_MULTI_LAYER_COLLISION),
            vec![vec![0x27, 0x1fe3_f637, 0xffff_fd47]]
        );
    }

    #[test]
    fn at_00622d60_goes_on_with_null_shape_and_phantom_when_allocation_fails() {
        let mut e = rig();
        e.call(0x0062_2d60, &args![0x5000u32, 1.0f32, 0u32]);
        assert!(calls(&e, SPHERE_SHAPE_CONSTRUCT).is_empty());
        assert!(calls(&e, PHANTOM_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, POINTER_GET)[0], vec![0]);
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![0x5000 + 0x3a0, 0]]);
        assert_eq!(calls(&e, RECORD_DESTRUCT).len(), 1);
    }

    #[test]
    fn at_00622d60_orders_the_calls_as_the_exe_does() {
        let mut e = rig();
        e.register(ALLOCATE_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call(0x0062_2d60, &args![0x5000u32, 1.0f32, 0u32]);
        let seq = order(&e);
        let expected = [
            ALLOCATE_OBJECT,
            SPHERE_SHAPE_CONSTRUCT,
            RECORD_CONSTRUCT,
            FILTER_CONSTRUCT,
            FILTER_SET_LAYER,
            FILTER_SET_SOURCE,
            HOLDER_GET,
            POINTER_GET,
            RECORD_MEMBER_CONSTRUCT,
            ALLOCATE_OBJECT,
            PHANTOM_CONSTRUCT,
            POINTER_ASSIGN,
            HOLDER_GET,
            POINTER_GET,
            SET_MULTI_LAYER_COLLISION,
            RECORD_DESTRUCT,
        ];
        assert_eq!(seq, expected);
    }

    #[test]
    fn at_00622f20_brackets_the_assignment_with_the_scope_calls() {
        let mut e = rig();
        e.call(0x0062_2f20, &args![0x4000u32, 0x99u32]);
        assert_eq!(order(&e), vec![POINTER_GET]);
        e.register(POINTER_GET, |_, _| 0x7000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_2f20, &args![0x4000u32, 0x99u32]);
        assert_eq!(
            order(&e),
            vec![POINTER_GET, POINTER_SCOPE, NAME_ASSIGN, POINTER_SCOPE]
        );
        assert_eq!(calls(&e, NAME_ASSIGN), vec![vec![0x7074, 0x99]]);
        assert_eq!(calls(&e, POINTER_SCOPE), vec![vec![0x4000], vec![0x4000]]);
    }

    #[test]
    fn at_00622f60_assigns_the_name_at_plus_0x74() {
        let mut e = rig();
        e.call(0x0062_2f60, &args![0x1000u32, 0x55u32]);
        assert_eq!(calls(&e, NAME_ASSIGN), vec![vec![0x1074, 0x55]]);
    }

    #[test]
    fn at_00623070_answers_the_link_of_the_list() {
        let mut e = rig();
        e.register(LIST_OF, |_, a| (a[0] + 0x10).into_ret());
        e.register(LIST_LINK, |_, a| (a[0] + 0x20).into_ret());
        assert_eq!(e.call(0x0062_3070, &args![0x1000u32]).u32(), 0x1030);
        assert_eq!(calls(&e, LIST_LINK), vec![vec![0x1010]]);
    }

    #[test]
    fn at_00622f80_skips_the_scale_for_zero_but_not_for_nan() {
        let mut e = rig();
        let this = object(&mut e, &[(8, 0x0600_1008)]);
        e.call(0x0062_2f80, &args![this, 1u32, 2u32, 0.0f32]);
        // `00623310` asks the holder first; with 0.0 that never happens before the reset.
        assert_eq!(
            calls(&e, CASTER_RESET),
            vec![vec![this, 0]],
            "the reset runs either way"
        );
        assert_eq!(order(&e)[0], CASTER_RESET);
        let mut e = rig();
        let this = object(&mut e, &[(8, 0x0600_1008)]);
        e.call(0x0062_2f80, &args![this, 1u32, 2u32, f32::NAN]);
        assert_eq!(order(&e)[0], HOLDER_GET, "the NaN went to 00623310 first");
        let mut e = rig();
        let this = object(&mut e, &[(8, 0x0600_1008)]);
        e.call(0x0062_2f80, &args![this, 1u32, 2u32, 3.0f32]);
        assert_eq!(order(&e)[0], HOLDER_GET);
    }

    #[test]
    fn at_00622f80_casts_through_the_holder_with_two_copied_records() {
        let mut e = rig();
        let this = object(&mut e, &[(8, 0x0600_1008)]);
        e.register(HOLDER_GET, |_, _| HELD.into_ret());
        e.register(LIST_OF, |_, _| 0x40u32.into_ret());
        e.register(LIST_LINK, |_, _| 0x1234u32.into_ret());
        e.register(RESULT_HOLDER, |_, _| 0x66u32.into_ret());
        let answer = e
            .call(0x0062_2f80, &args![this, 0x11u32, 0x22u32, 0.0f32])
            .u32();
        assert_eq!(answer, 0x1234);
        let first = calls(&e, MEMBER_SETUP)[0][0];
        let second = calls(&e, MEMBER_A_SETUP)[0][0];
        assert_eq!(
            second + 0x20,
            first,
            "the records sit 0x20 apart on the frame"
        );
        assert_eq!(
            calls(&e, COPY_RECORD),
            vec![vec![first, 0x11], vec![second, 0x22]]
        );
        assert_eq!(
            calls(&e, CAST_THROUGH_HOLDER),
            vec![vec![HELD, first, second, this, 0]]
        );
        let flag = calls(&e, RESULT_HOLDER)[0][1];
        assert_eq!(calls(&e, RESULT_HOLDER), vec![vec![this, flag]]);
        assert!(flag < second, "the byte sits below the records");
        assert_eq!(calls(&e, RESULT_BYTE), vec![vec![0x66]]);
        assert_eq!(order(&e).last(), Some(&LIST_LINK));
    }

    #[test]
    fn at_00623090_runs_the_three_steps_on_the_held_object() {
        let mut e = rig();
        let second = object(&mut e, &[(8, 0x0600_2008)]);
        e.call(0x0062_3090, &args![0x4000u32, 0x31u32, second]);
        assert_eq!(order(&e), vec![HOLDER_GET]);
        e.register(HOLDER_GET, |_, _| HELD.into_ret());
        e.register(POINTER_GET, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_3090, &args![0x4000u32, 0x31u32, second]);
        assert_eq!(
            calls(&e, HOLDER_GET),
            vec![vec![0x43a0], vec![0x43a0], vec![0x43a0]]
        );
        assert_eq!(calls(&e, 0x0600_2008), vec![vec![second]]);
        // Its first step is 006230f0 on the held object with the first word.
        assert_eq!(calls(&e, MEMBER_SETUP).len(), 1);
        assert_eq!(calls(&e, RECORD_CONSUME)[0][0], HELD);
        // Its last step is 00623150 on the held object: the pointer answer is null, so no call.
        assert_eq!(calls(&e, POINTER_GET), vec![vec![HELD]]);
    }

    #[test]
    fn at_006230f0_copies_the_source_and_consumes_the_copy() {
        let mut e = rig();
        e.register(COPY_RECORD, |_, a| (a[1] + 1).into_ret());
        e.call(0x0062_30f0, &args![0x4000u32, 0x31u32]);
        let record = calls(&e, MEMBER_SETUP)[0][0];
        assert_eq!(calls(&e, COPY_RECORD), vec![vec![record, 0x31]]);
        assert_eq!(calls(&e, RECORD_CONSUME), vec![vec![0x4000, 0x32]]);
    }

    #[test]
    fn at_00623150_calls_slot_0x44_only_for_a_pointer_with_a_key() {
        let mut e = rig();
        let held = object(&mut e, &[(0x44, 0x0600_3044)]);
        e.register_double(POINTER_GET, move |_, _| held.into_ret());
        // A zero key: nothing happens.
        e.call(0x0062_3150, &args![0x4000u32, 0x9u32]);
        assert!(calls(&e, 0x0600_3044).is_empty());
        e.register(NODE_KEY, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_3150, &args![0x4000u32, 0x9u32]);
        assert_eq!(calls(&e, 0x0600_3044), vec![vec![held, 9, 0]]);
        assert_eq!(calls(&e, POINTER_SCOPE).len(), 2);
        // No pointer at all.
        e.register(POINTER_GET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_3150, &args![0x4000u32, 0x9u32]);
        assert!(calls(&e, NODE_KEY).is_empty());
    }

    #[test]
    fn at_006231a0_changes_the_target_and_colours_a_new_one() {
        let mut e = rig();
        let held = object(&mut e, &[(0x9c, 0x0600_309c), (0x94, 0x0600_3094)]);
        // Nothing held: nothing happens.
        e.call(0x0062_31a0, &args![0x4000u32, 0x77u32]);
        assert_eq!(order(&e), vec![HOLDER_GET]);
        e.register_double(HOLDER_GET, move |_, _| held.into_ret());
        // Already the current target (00623230 answers 0x77 through 00621480).
        e.register(FIRST_OF, |_, _| 0x77u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_31a0, &args![0x4000u32, 0x77u32]);
        assert!(calls(&e, 0x0600_309c).is_empty());
        // A different target: slot 0x9c, then the colour and the debug colour.
        e.register(COLOR_FROM_BYTES, |_, _| 0xc0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_31a0, &args![0x4000u32, 0x88u32]);
        assert_eq!(calls(&e, 0x0600_309c), vec![vec![held, 0x88]]);
        assert_eq!(
            calls(&e, COLOR_FROM_BYTES),
            vec![vec![0xff, 0x10, 0x10, 0x19]]
        );
        assert_eq!(calls(&e, SET_DEBUG_COLOR), vec![vec![0x88, held, 0xc0]]);
        // A null target is set but not coloured.
        e.call_log = Some(vec![]);
        e.call(0x0062_31a0, &args![0x4000u32, 0u32]);
        assert_eq!(calls(&e, 0x0600_309c), vec![vec![held, 0]]);
        assert!(calls(&e, COLOR_FROM_BYTES).is_empty());
    }

    #[test]
    fn at_00623230_answers_the_first_of_the_held_objects_value() {
        let mut e = rig();
        assert_eq!(e.call(0x0062_3230, &args![0x4000u32]).u32(), 0);
        let held = object(&mut e, &[(0x94, 0x0600_3094)]);
        e.register_double(HOLDER_GET, move |_, _| held.into_ret());
        e.register(0x0600_3094, |_, _| 0x55u32.into_ret());
        e.register(FIRST_OF, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x0062_3230, &args![0x4000u32]).u32(), 0x56);
    }

    #[test]
    fn at_00623310_stores_the_converted_value_as_a_float() {
        let mut e = rig();
        e.call(0x0062_3310, &args![0x4000u32, 1.0f32]);
        assert!(calls(&e, OWNER_OF).is_empty());
        e.register(HOLDER_GET, |_, _| HELD.into_ret());
        e.call(0x0062_3310, &args![0x4000u32, 1.0f32]);
        assert!(
            calls(&e, CONVERT_VALUE).is_empty(),
            "no owner, no conversion"
        );
        e.register(OWNER_OF, |_, _| 0x7000u32.into_ret());
        e.register(CONVERT_VALUE, |_, a| {
            (f64::from(f32::from_bits(a[0])) * 2.0).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0062_3310, &args![0x4000u32, 1.5f32]);
        assert_eq!(calls(&e, CONVERT_VALUE), vec![vec![1.5f32.to_bits()]]);
        assert_eq!(
            calls(&e, OWNER_SET_VALUE),
            vec![vec![0x7000, 3.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, OWNER_OF), vec![vec![HELD]]);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on the running thread takes, with the call log
    /// switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        let frame = e.next_stack_local(size);
        e.call_log = Some(vec![]);
        frame
    }

    /// `EBP` of an exe frame that aligns it to 16 bytes, `below` bytes of locals under it.
    fn aligned_ebp(raw: u32, below: u32) -> u32 {
        (raw + below + 0xf) & !0xf
    }

    #[test]
    fn the_decal_caster_frames_put_their_locals_below_an_aligned_ebp() {
        // 00622d60: the record at EBP-0x90, the filter at EBP-0x94.
        let mut e = rig();
        let raw = next_frame(&mut e, 0xb4 + 0x10);
        e.call(0x0062_2d60, &args![0x5000u32, 1.0f32, 0u32]);
        let ebp = aligned_ebp(raw, 0xb4);
        assert_eq!(calls(&e, RECORD_CONSTRUCT), vec![vec![ebp - 0x90]]);
        assert_eq!(calls(&e, FILTER_CONSTRUCT)[0][0], ebp - 0x94);
        // 00622f80: the records at EBP-0x20 and EBP-0x40, the byte at EBP-0x41.
        let this = object(&mut e, &[(8, 0x0600_1008)]);
        e.register(HOLDER_GET, |_, _| HELD.into_ret());
        let raw = next_frame(&mut e, 0x50 + 0x10);
        e.call(0x0062_2f80, &args![this, 0x11u32, 0x22u32, 0.0f32]);
        let ebp = aligned_ebp(raw, 0x50);
        assert_eq!(calls(&e, MEMBER_SETUP), vec![vec![ebp - 0x20]]);
        assert_eq!(calls(&e, MEMBER_A_SETUP), vec![vec![ebp - 0x40]]);
        assert_eq!(calls(&e, RESULT_HOLDER), vec![vec![this, ebp - 0x41]]);
        // 006230f0: the record at EBP-0x20.
        let raw = next_frame(&mut e, 0x20 + 0x10);
        e.call(0x0062_30f0, &args![0x4000u32, 0x31u32]);
        assert_eq!(
            calls(&e, MEMBER_SETUP),
            vec![vec![aligned_ebp(raw, 0x20) - 0x20]]
        );
    }
}
