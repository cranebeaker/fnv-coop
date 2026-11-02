//! `Havok SDK/hkphavoksnapshot.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil` growth function (cdecl `(allocator, array, element size)`; no map name).
const RESERVE_MORE: u32 = 0x00c9_0940;
/// A class-derivation test (no map name; `this` = a class record, then `(out byte pointer,
/// other class)`; stores whether the other class derives from `this` and answers the out
/// pointer).
const IS_DERIVED: u32 = 0x00d4_6d80;
/// `hkClass::getName` (Xbox PDB; `this` = the class, answers its name).
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
/// `hkStringPtr::hkStringPtr`, `hkStringPtr::~hkStringPtr` and `hkStringPtr::operator=` (all
/// Xbox PDB; `this` = the string, the last one then takes the new text).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
/// `hkRefVariant::set` and `hkRefVariant::getClass` (Xbox PDB; `this` = the variant, `set` then
/// takes `(object, class)`).
const VARIANT_SET: u32 = 0x00d4_7770;
const VARIANT_GET_CLASS: u32 = 0x00d4_77a0;
/// The reference release of `hkReferencedObject` (no map name; `this` = the object).
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkSerializeUtil::saveTagfile` (Xbox PDB, cdecl `(object, class, tag, flag, writer)`;
/// answers 0 on success).
const SAVE_TAGFILE: u32 = 0x00d4_85a0;
/// `hkpPhysicsData::hkpPhysicsData`, the unnamed function that fills it from a source object
/// (`this`, then `(source, last argument)`) and `hkpPhysicsData::~hkpPhysicsData` (Xbox PDB).
const PHYSICS_DATA_CONSTRUCT: u32 = 0x00d4_c220;
const PHYSICS_DATA_POPULATE: u32 = 0x00d4_c2d0;
const PHYSICS_DATA_DESTRUCT: u32 = 0x00d4_c250;
/// The constructors of the three replacement objects of the conversion (`this` = the new block,
/// then the original object): sizes 0x70, 0x110 (`hkpStorageExtendedMeshShape`, Xbox PDB) and
/// 0x80.
const REPLACEMENT_A: u32 = 0x00d4_bab0;
const REPLACEMENT_B: u32 = 0x00d4_aa70;
const REPLACEMENT_C: u32 = 0x00d4_9750;
/// The class records the conversion tests an object's class against, and the class records an
/// object gets when it is replaced.
const CLASS_A: u32 = 0x0126_8f2c;
const CLASS_B: u32 = 0x0126_8e24;
const CLASS_C: u32 = 0x0126_8c5c;
const CLASS_D: u32 = 0x0126_8bcc;
const REPLACED_A: u32 = 0x0126_8eb4;
const REPLACED_B: u32 = 0x0126_8d4c;
const REPLACED_C: u32 = 0x0126_8c14;
/// The class records the tag file is written with (`01268b54`) and the one the physics data is
/// stored under (`01268b9c`).
const TAGFILE_CLASS: u32 = 0x0126_8b54;
const PHYSICS_DATA_CLASS: u32 = 0x0126_8b9c;
/// The vtables of the owner list of the conversion: the derived (`010c93e4`) and the base
/// (`0102e388`) one.
const LIST_VTABLE: u32 = 0x010c_93e4;
const LIST_BASE_VTABLE: u32 = 0x0102_e388;

/// Allocates `size` bytes through the memory router's allocator (slot 4) and stores the size
/// as the 16-bit word at `+4`.
fn allocate_object(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// Appends `object` to the pointer array at `array` (data `+0`, size `+4`, capacity and flags
/// `+8`), growing it through the allocator first when it is full.
fn push_back_pointer(e: &mut Engine, array: u32, object: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), object);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Whether `class` derives from the class record `of` (the out byte lives at `out`).
fn derives_from(e: &mut Engine, of: u32, out: u32, class: u32) -> bool {
    let answer = e.call(IS_DERIVED, &args![of, out, class]).u32();
    e.mem.u8(answer) != 0
}

// Translated from 00cc08f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHavokSnapshot::ConvertListener::addObjectCallback` (Xbox PDB; `this` = the listener,
/// then `object_slot` (a pointer to the object pointer) and `class_slot` (a pointer to the
/// object's class record), `RET 8`): replaces the object by a converted copy and remembers
/// the copy in the listener's pointer array (data `+8`, size `+0xC`, capacity `+0x10`). In
/// order, with the class `c` in the class slot:
///
/// 1. when `c` derives from the record at `01268f2c` and is not `01268eb4`, a 0x70-byte copy
///    (`00d4bab0`) replaces the object and the class slot becomes `01268eb4`;
/// 2. then, when the (new) class derives from `01268e24` and is not `01268d4c`, a 0x110-byte
///    `hkpStorageExtendedMeshShape` copy (`00d4aa70`) replaces the object, the class slot
///    becomes `01268d4c` and the callback ends;
/// 3. otherwise, when it derives from `01268c5c`: an object whose byte at `+0x1D` is 2 gets a
///    0x80-byte copy (`00d49750`) and the class `01268c14`; any other object is left alone;
/// 4. otherwise, when it derives from `01268bcc` and the object's owner (the pointer at `+8`)
///    points back to it (at `+0x24`), both slots are cleared.
pub fn hkp_havok_snapshot_convert_listener_add_object_callback(
    e: &mut Engine,
    this: Ptr,
    object_slot: Ptr,
    class_slot: Ptr,
) {
    let list = this.addr() + 8;
    let (object_slot, class_slot) = (object_slot.addr(), class_slot.addr());
    // The out bytes of the derivation tests live in the exe's stack frame (a local and the
    // two argument slots).
    e.with_stack(0x10, |e, scratch| {
        let scratch = scratch.addr();
        let class = e.mem.u32(class_slot);
        if derives_from(e, CLASS_A, scratch + 3, class) && e.mem.u32(class_slot) != REPLACED_A {
            let object = e.mem.u32(object_slot);
            let block = allocate_object(e, 0x70);
            let copy = e.call(REPLACEMENT_A, &args![block, object]).u32();
            push_back_pointer(e, list, copy);
            e.mem.set_u32(object_slot, copy);
            e.mem.set_u32(class_slot, REPLACED_A);
        }
        let class = e.mem.u32(class_slot);
        if derives_from(e, CLASS_B, scratch + 8, class) && e.mem.u32(class_slot) != REPLACED_B {
            let object = e.mem.u32(object_slot);
            let block = allocate_object(e, 0x110);
            let copy = e.call(REPLACEMENT_B, &args![block, object]).u32();
            push_back_pointer(e, list, copy);
            e.mem.set_u32(object_slot, copy);
            e.mem.set_u32(class_slot, REPLACED_B);
            return;
        }
        let class = e.mem.u32(class_slot);
        if derives_from(e, CLASS_C, scratch + 8, class) {
            let object = e.mem.u32(object_slot);
            if e.mem.u8(object + 0x1d) == 2 {
                let block = allocate_object(e, 0x80);
                let copy = e.call(REPLACEMENT_C, &args![block, object]).u32();
                push_back_pointer(e, list, copy);
                e.mem.set_u32(object_slot, copy);
                e.mem.set_u32(class_slot, REPLACED_C);
            }
            return;
        }
        let class = e.mem.u32(class_slot);
        if derives_from(e, CLASS_D, scratch + 0xc, class) {
            let object = e.mem.u32(object_slot);
            let owner = e.mem.u32(object + 8);
            if owner != 0 && e.mem.u32(owner + 0x24) == object {
                e.mem.set_u32(object_slot, 0);
                e.mem.set_u32(class_slot, 0);
            }
        }
    });
}

// Translated from 00cc0ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cc0ae0` (no map name / unconfirmed; `ECX` = `this`): the base destructor the
/// `ConvertListener` deleting destructor `00cc0dd0` calls. Installs the vtable `010c93e4`, calls
/// the virtual function at slot 0 with `1` (the scalar deleting destructor) on every non-null
/// pointer of the array (data `+8`, size `+0xC`, capacity and flags `+0x10`), empties it, frees
/// its block through the allocator (slot `0x10`, size `capacity * 4`) unless the capacity word
/// is negative (not owned), clears the data pointer, resets the capacity word to `0x80000000`
/// and installs the vtable `0102e388`.
pub fn fn_00cc0ae0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, LIST_VTABLE);
    if e.mem.i32(me + 0xc) > 0 {
        let mut index = 0u32;
        loop {
            let item = e.mem.u32(e.mem.u32(me + 8) + index * 4);
            if item != 0 {
                e.vcall(item, 0, &args![1u32]);
            }
            index += 1;
            if (index as i32) >= e.mem.i32(me + 0xc) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x10);
    e.mem.set_u32(me + 0xc, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 8);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
    }
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0x10, 0x8000_0000);
    e.mem.set_u32(me, LIST_BASE_VTABLE);
}

// Translated from 00cc0b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cc0b50` (no map name / unconfirmed; cdecl, seven arguments): writes `object` as a tag
/// file with `hkSerializeUtil::saveTagfile(object, class, tag, flag != 0, writer)`, where a
/// null `writer` is replaced by a local list object (vtable `010c93e4`, reference count 1,
/// empty array), stores 1 at `out` when that succeeded (answered 0) and 0 otherwise, destroys
/// the local list (`fn_00cc0ae0`) and answers `out`. The sixth argument is not used.
pub fn fn_00cc0b50(
    e: &mut Engine,
    out: Ptr,
    object: u32,
    class: u32,
    tag: u32,
    flag: u8,
    _unused: u32,
    writer: u32,
) -> Ptr {
    e.with_stack(0x14, |e, local| {
        let local = local.addr();
        e.mem.set_u16(local + 6, 1);
        e.mem.set_u32(local, LIST_VTABLE);
        e.mem.set_u32(local + 8, 0);
        e.mem.set_u32(local + 0xc, 0);
        e.mem.set_u32(local + 0x10, 0x8000_0000);
        let writer = if writer == 0 { local } else { writer };
        let status = e
            .call(
                SAVE_TAGFILE,
                &args![object, class, tag, u32::from(flag != 0), writer],
            )
            .u32();
        e.mem.set_u8(out.addr(), u8::from(status == 0));
        fn_00cc0ae0(e, Ptr::new(local));
        out
    })
}

// Translated from 00cc0bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHavokSnapshot::saveUnderRootLevel` (Xbox PDB; cdecl, six arguments): builds a one-entry
/// array of 12-byte named variants (name string, class-name string, `hkRefVariant`) holding
/// `object` under `class` (the name is the class's name, the second string the name of the
/// variant's own class, or none), writes it with `fn_00cc0b50(out, &array, 01268b54, tag,
/// flag, last, 0)`, then releases the variants and the strings of the array and frees its block
/// (slot `0x10`, `capacity * 12`) unless the capacity word is negative. Answers `out`.
pub fn hkp_havok_snapshot_save_under_root_level(
    e: &mut Engine,
    out: Ptr,
    object: u32,
    class: u32,
    tag: u32,
    flag: u32,
    last: u32,
) -> Ptr {
    e.with_stack(0xc, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
        if e.mem.u32(array + 4) == capacity {
            let allocator = e.mem.u32(router + 0x10);
            e.call(RESERVE_MORE, &args![allocator, array, 0xcu32]);
        }
        let size = e.mem.u32(array + 4);
        let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(12));
        if slot != 0 {
            e.call(STRING_PTR_CONSTRUCT, &args![slot]);
            e.call(STRING_PTR_CONSTRUCT, &args![slot + 4]);
            e.mem.set_u32(slot + 8, 0);
        }
        let size = e.mem.u32(array + 4);
        let element = e.mem.u32(array).wrapping_add(size.wrapping_mul(12));
        e.mem.set_u32(array + 4, size.wrapping_add(1));
        let name = e.call(CLASS_GET_NAME, &args![class]).u32();
        e.call(STRING_PTR_ASSIGN, &args![element, name]);
        let variant = element + 8;
        e.call(VARIANT_SET, &args![variant, object, class]);
        let variant_class = e.call(VARIANT_GET_CLASS, &args![variant]).u32();
        let class_name = if variant_class == 0 {
            0
        } else {
            let variant_class = e.call(VARIANT_GET_CLASS, &args![variant]).u32();
            e.call(CLASS_GET_NAME, &args![variant_class]).u32()
        };
        e.call(STRING_PTR_ASSIGN, &args![element + 4, class_name]);
        fn_00cc0b50(e, out, array, TAGFILE_CLASS, tag, flag as u8, last, 0);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let count = e.mem.i32(array + 4);
        if count > 0 {
            let mut entry = e.mem.u32(array) + 8;
            for _ in 0..count {
                let variant = e.mem.u32(entry);
                if variant != 0 {
                    e.call(RELEASE_REFERENCE, &args![variant]);
                }
                e.mem.set_u32(entry, 0);
                e.call(STRING_PTR_DESTRUCT, &args![entry - 4]);
                e.call(STRING_PTR_DESTRUCT, &args![entry - 8]);
                entry += 0xc;
            }
        }
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 12]);
        }
        out
    })
}

// Translated from 00cc0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cc0d30` (no map name / unconfirmed; cdecl, five arguments): `saveUnderRootLevel`
/// with the class record `01268b9c`: `(out, object, 01268b9c, tag, flag, last)`. Answers `out`.
pub fn fn_00cc0d30(e: &mut Engine, out: Ptr, object: u32, tag: u32, flag: u32, last: u32) -> Ptr {
    hkp_havok_snapshot_save_under_root_level(e, out, object, PHYSICS_DATA_CLASS, tag, flag, last);
    out
}

// Translated from 00cc0d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHavokSnapshot::save` (Xbox PDB; cdecl, six arguments): with a null `source` or a null
/// `tag` stores 0 at `out`. Otherwise builds a local `hkpPhysicsData` (0x18 bytes), fills it
/// from `source` (`00d4c2d0(data, source, extra)`), saves it with `fn_00cc0d30(&status, data,
/// tag, flag, last)` (the status byte lives in the stack slot of the second argument),
/// destroys the data and stores the status byte at `out`. Both paths leave `out` in `EAX`.
pub fn hkp_havok_snapshot_save(
    e: &mut Engine,
    out: Ptr,
    source: u32,
    tag: u32,
    flag: u32,
    last: u32,
    extra: u32,
) -> Ptr {
    if source == 0 || tag == 0 {
        e.mem.set_u8(out.addr(), 0);
        return out;
    }
    e.with_stack(0x1c, |e, frame| {
        let data = frame.addr();
        let slot = data + 0x18;
        e.mem.set_u32(slot, source);
        e.call(PHYSICS_DATA_CONSTRUCT, &args![data]);
        e.call(PHYSICS_DATA_POPULATE, &args![data, source, extra]);
        let status = fn_00cc0d30(e, Ptr::new(slot), data, tag, flag, last);
        let status = e.mem.u8(status.addr());
        e.call(PHYSICS_DATA_DESTRUCT, &args![data]);
        e.mem.set_u8(out.addr(), status);
    });
    out
}

// Translated from 00cc0dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cc_0ae0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cc0dd0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cc_0ae0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cc_08f0,
            hkp_havok_snapshot_convert_listener_add_object_callback(Ptr, Ptr, Ptr)
        ),
        entry!(0x00cc_0ae0, fn_00cc0ae0(Ptr)),
        entry!(
            0x00cc_0b50,
            fn_00cc0b50(Ptr, u32, u32, u32, u8, u32, u32) -> Ptr
        ),
        entry!(
            0x00cc_0bc0,
            hkp_havok_snapshot_save_under_root_level(Ptr, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00cc_0d30, fn_00cc0d30(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(
            0x00cc_0d60,
            hkp_havok_snapshot_save(Ptr, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00cc_0dd0, deleting_destructor_00cc0dd0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, log_double, logging_object, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00cc0dd0_frees_only_when_bit_zero_is_set() {
        check(0x00cc_0dd0, 0x00cc_0ae0);
    }

    const ALLOC_DOUBLE: u32 = 0x0900_0021;
    const FREE_DOUBLE: u32 = 0x0900_0022;
    /// What the doubles of the three replacement constructors answer.
    const NEW_OBJECTS: [u32; 3] = [0x0a00_0001, 0x0a00_0002, 0x0a00_0003];

    /// A memory router whose allocator hands out zeroed blocks (slot 4) and logs the frees
    /// (slot `0x10`); both are logged. Answers the allocator.
    fn memory(e: &mut Engine, log: &Log) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOC_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(ALLOC_DOUBLE, move |e, a| {
            sink.borrow_mut().push((ALLOC_DOUBLE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        log_double(e, log, FREE_DOUBLE, 0);
        allocator
    }

    /// The addresses of the logged calls, in order.
    fn order(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(address, _)| *address).collect()
    }

    #[test]
    fn the_list_destructor_destroys_its_items_and_frees_an_owned_block() {
        let cases = [
            (3u32, 0x10u32, true),
            (3, 0x8000_0010, false),
            (0, 0x10, true),
        ];
        for (size, capacity, freed) in cases {
            let mut e = engine();
            let log = new_log();
            let allocator = memory(&mut e, &log);
            let first = logging_object(&mut e, &log, 0x00e2_0000);
            let second = logging_object(&mut e, &log, 0x00e3_0000);
            let data = e.mem.alloc(0x40);
            for (index, item) in [first, 0, second].into_iter().enumerate() {
                e.mem.set_u32(data + 4 * index as u32, item);
            }
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 8, data);
            e.mem.set_u32(this + 0xc, size);
            e.mem.set_u32(this + 0x10, capacity);
            e.call(0x00cc_0ae0, &args![this]);
            let destroyed = size != 0;
            let expected = |item: u32| {
                if destroyed {
                    vec![vec![item, 1]]
                } else {
                    vec![]
                }
            };
            assert_eq!(calls(&log, 0x00e2_0000), expected(first));
            assert_eq!(calls(&log, 0x00e3_0000), expected(second));
            let freed_calls = if freed {
                vec![vec![allocator, data, 0x40]]
            } else {
                vec![]
            };
            assert_eq!(calls(&log, FREE_DOUBLE), freed_calls);
            assert_eq!(e.mem.u32(this), LIST_BASE_VTABLE);
            assert_eq!(e.mem.u32(this + 8), 0);
            assert_eq!(e.mem.u32(this + 0xc), 0);
            assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        }
    }

    #[test]
    fn the_tag_file_writer_falls_back_to_a_local_list() {
        let cases = [(0u32, 0u32, 5u32, 1u32, 1u8), (0x1234, 3, 0x100, 0, 0)];
        for (writer, status, flag, expected_flag, expected_out) in cases {
            let mut e = engine();
            let log = new_log();
            memory(&mut e, &log);
            let snapshot: Rc<RefCell<Vec<u32>>> = Rc::default();
            let sink = Rc::clone(&snapshot);
            let call_log = Rc::clone(&log);
            e.register_double(SAVE_TAGFILE, move |e, a| {
                call_log.borrow_mut().push((SAVE_TAGFILE, a.to_vec()));
                if a[4] != 0x1234 {
                    *sink.borrow_mut() = vec![
                        e.mem.u32(a[4]),
                        u32::from(e.mem.u16(a[4] + 6)),
                        e.mem.u32(a[4] + 8),
                        e.mem.u32(a[4] + 0xc),
                        e.mem.u32(a[4] + 0x10),
                    ];
                }
                Ret {
                    eax: status,
                    ..Ret::default()
                }
            });
            let out = e.mem.alloc(4);
            e.mem.set_u32(out, 0x5555_5555);
            let args = args![out, 0x10u32, 0x20u32, 0x30u32, flag, 0x99u32, writer];
            assert_eq!(e.call(0x00cc_0b50, &args).u32(), out);
            let seen = calls(&log, SAVE_TAGFILE);
            assert_eq!(seen.len(), 1);
            assert_eq!(seen[0][..4], [0x10, 0x20, 0x30, expected_flag]);
            if writer == 0 {
                assert_ne!(seen[0][4], 0);
                assert_eq!(*snapshot.borrow(), vec![LIST_VTABLE, 1, 0, 0, 0x8000_0000]);
            } else {
                assert_eq!(seen[0][4], writer);
            }
            assert_eq!(e.mem.u8(out), expected_out);
            assert_eq!(e.mem.u8(out + 1), 0x55);
        }
    }

    /// Doubles for everything the root-level save calls. The array "grows" to `data` (room for
    /// four 12-byte entries); `status` is what `saveTagfile` answers and `variant_class` what
    /// the variant's `getClass` answers. Answers the allocator.
    fn writer_doubles(
        e: &mut Engine,
        log: &Log,
        data: u32,
        status: u32,
        variant_class: u32,
    ) -> u32 {
        let allocator = memory(e, log);
        let sink = Rc::clone(log);
        e.register_double(RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((RESERVE_MORE, a.to_vec()));
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        for address in [
            STRING_PTR_CONSTRUCT,
            STRING_PTR_DESTRUCT,
            STRING_PTR_ASSIGN,
            RELEASE_REFERENCE,
            PHYSICS_DATA_CONSTRUCT,
            PHYSICS_DATA_POPULATE,
            PHYSICS_DATA_DESTRUCT,
        ] {
            log_double(e, log, address, 0);
        }
        let sink = Rc::clone(log);
        e.register_double(CLASS_GET_NAME, move |e, a| {
            sink.borrow_mut().push((CLASS_GET_NAME, a.to_vec()));
            Ret {
                eax: e.mem.u32(a[0]),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(log);
        e.register_double(VARIANT_SET, move |e, a| {
            sink.borrow_mut().push((VARIANT_SET, a.to_vec()));
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        log_double(e, log, VARIANT_GET_CLASS, variant_class);
        log_double(e, log, SAVE_TAGFILE, status);
        allocator
    }

    /// A class record whose name pointer (its first word) is `name`.
    fn class_record(e: &mut Engine, name: u32) -> u32 {
        let class = e.mem.alloc(0x10);
        e.mem.set_u32(class, name);
        class
    }

    #[test]
    fn save_under_root_level_stores_the_object_under_its_class_and_cleans_up() {
        let mut e = engine();
        let log = new_log();
        let data = e.mem.alloc(0x30);
        let class = class_record(&mut e, 0x4444);
        let variant_class = class_record(&mut e, 0x5555);
        let allocator = writer_doubles(&mut e, &log, data, 0, variant_class);
        let out = e.mem.alloc(4);
        let args = args![out, 0x10u32, class, 0x11u32, 5u32, 0x22u32];
        assert_eq!(e.call(0x00cc_0bc0, &args).u32(), out);
        let array = calls(&log, RESERVE_MORE)[0][1];
        assert_eq!(calls(&log, RESERVE_MORE), vec![vec![allocator, array, 0xc]]);
        assert_eq!(
            order(&log),
            vec![
                RESERVE_MORE,
                STRING_PTR_CONSTRUCT,
                STRING_PTR_CONSTRUCT,
                CLASS_GET_NAME,
                STRING_PTR_ASSIGN,
                VARIANT_SET,
                VARIANT_GET_CLASS,
                VARIANT_GET_CLASS,
                CLASS_GET_NAME,
                STRING_PTR_ASSIGN,
                SAVE_TAGFILE,
                RELEASE_REFERENCE,
                STRING_PTR_DESTRUCT,
                STRING_PTR_DESTRUCT,
                FREE_DOUBLE
            ]
        );
        assert_eq!(
            calls(&log, STRING_PTR_CONSTRUCT),
            vec![vec![data], vec![data + 4]]
        );
        assert_eq!(
            calls(&log, STRING_PTR_ASSIGN),
            vec![vec![data, 0x4444], vec![data + 4, 0x5555]]
        );
        assert_eq!(calls(&log, VARIANT_SET), vec![vec![data + 8, 0x10, class]]);
        assert_eq!(
            calls(&log, CLASS_GET_NAME),
            vec![vec![class], vec![variant_class]]
        );
        assert_eq!(
            calls(&log, SAVE_TAGFILE)[0][..4],
            [array, TAGFILE_CLASS, 0x11, 1]
        );
        assert_eq!(calls(&log, RELEASE_REFERENCE), vec![vec![0x10]]);
        assert_eq!(
            calls(&log, STRING_PTR_DESTRUCT),
            vec![vec![data + 4], vec![data]]
        );
        assert_eq!(calls(&log, FREE_DOUBLE), vec![vec![allocator, data, 0x30]]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u32(data + 8), 0);
    }

    #[test]
    fn save_under_root_level_without_a_variant_class_or_object_names_nothing() {
        let mut e = engine();
        let log = new_log();
        let data = e.mem.alloc(0x30);
        let class = class_record(&mut e, 0x4444);
        writer_doubles(&mut e, &log, data, 7, 0);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        let args = args![out, 0u32, class, 0x11u32, 0u32, 0x22u32];
        assert_eq!(e.call(0x00cc_0bc0, &args).u32(), out);
        assert_eq!(
            order(&log),
            vec![
                RESERVE_MORE,
                STRING_PTR_CONSTRUCT,
                STRING_PTR_CONSTRUCT,
                CLASS_GET_NAME,
                STRING_PTR_ASSIGN,
                VARIANT_SET,
                VARIANT_GET_CLASS,
                STRING_PTR_ASSIGN,
                SAVE_TAGFILE,
                STRING_PTR_DESTRUCT,
                STRING_PTR_DESTRUCT,
                FREE_DOUBLE
            ]
        );
        assert_eq!(
            calls(&log, STRING_PTR_ASSIGN),
            vec![vec![data, 0x4444], vec![data + 4, 0]]
        );
        assert_eq!(calls(&log, SAVE_TAGFILE)[0][3], 0);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn the_physics_data_root_level_save_uses_the_physics_data_class() {
        let mut e = engine();
        let log = new_log();
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(PHYSICS_DATA_CLASS, 0x6666);
        writer_doubles(&mut e, &log, data, 0, 0);
        let out = e.mem.alloc(4);
        let args = args![out, 0x10u32, 0x11u32, 5u32, 0x22u32];
        assert_eq!(e.call(0x00cc_0d30, &args).u32(), out);
        assert_eq!(
            calls(&log, VARIANT_SET),
            vec![vec![data + 8, 0x10, PHYSICS_DATA_CLASS]]
        );
        assert_eq!(calls(&log, STRING_PTR_ASSIGN)[0], vec![data, 0x6666]);
        let saved = &calls(&log, SAVE_TAGFILE)[0];
        assert_eq!(saved[1..4], [TAGFILE_CLASS, 0x11, 1]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn save_builds_the_physics_data_saves_it_and_destroys_it() {
        for (status, expected) in [(0u32, 1u8), (4, 0)] {
            let mut e = engine();
            let log = new_log();
            let data = e.mem.alloc(0x30);
            writer_doubles(&mut e, &log, data, status, 0);
            let out = e.mem.alloc(4);
            let args = args![out, 0x7000u32, 0x11u32, 5u32, 0x22u32, 0x33u32];
            assert_eq!(e.call(0x00cc_0d60, &args).u32(), out);
            let built = calls(&log, PHYSICS_DATA_CONSTRUCT);
            let physics = built[0][0];
            assert_eq!(
                calls(&log, PHYSICS_DATA_POPULATE),
                vec![vec![physics, 0x7000, 0x33]]
            );
            assert_eq!(calls(&log, PHYSICS_DATA_DESTRUCT), vec![vec![physics]]);
            let sequence = order(&log);
            assert_eq!(
                sequence[..2],
                [PHYSICS_DATA_CONSTRUCT, PHYSICS_DATA_POPULATE]
            );
            assert_eq!(sequence.last(), Some(&PHYSICS_DATA_DESTRUCT));
            assert_eq!(
                calls(&log, VARIANT_SET),
                vec![vec![data + 8, physics, PHYSICS_DATA_CLASS]]
            );
            assert_eq!(calls(&log, SAVE_TAGFILE)[0][2..4], [0x11, 1]);
            assert_eq!(e.mem.u8(out), expected);
        }
    }

    #[test]
    fn save_without_a_source_or_a_tag_stores_zero_and_does_nothing_else() {
        for (source, tag) in [(0u32, 0x11u32), (0x7000, 0)] {
            let mut e = engine();
            let log = new_log();
            let data = e.mem.alloc(0x30);
            writer_doubles(&mut e, &log, data, 0, 0);
            let out = e.mem.alloc(4);
            e.mem.set_u8(out, 0xff);
            let args = args![out, source, tag, 5u32, 0x22u32, 0x33u32];
            assert_eq!(e.call(0x00cc_0d60, &args).u32(), out);
            assert_eq!(e.mem.u8(out), 0);
            assert!(log.borrow().is_empty());
        }
    }

    /// Doubles for the conversion: `answers` are what the derivation test says for the four
    /// class records (in the order A to D). Answers the allocator.
    fn listener_doubles(e: &mut Engine, log: &Log, answers: [bool; 4]) -> u32 {
        let allocator = memory(e, log);
        let sink = Rc::clone(log);
        e.register_double(IS_DERIVED, move |e, a| {
            sink.borrow_mut().push((IS_DERIVED, a.to_vec()));
            let index = [CLASS_A, CLASS_B, CLASS_C, CLASS_D]
                .iter()
                .position(|class| *class == a[0]);
            e.mem
                .set_u8(a[1], u8::from(matches!(index, Some(i) if answers[i])));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        for (address, result) in [REPLACEMENT_A, REPLACEMENT_B, REPLACEMENT_C]
            .into_iter()
            .zip(NEW_OBJECTS)
        {
            log_double(e, log, address, result);
        }
        let sink = Rc::clone(log);
        e.register_double(RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((RESERVE_MORE, a.to_vec()));
            let data = e.mem.alloc(0x40);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        allocator
    }

    /// A listener and the slots for `object` and `class`: `(this, object slot, class slot)`.
    fn listener(e: &mut Engine, object: u32, class: u32) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x20);
        let object_slot = e.mem.alloc(4);
        e.mem.set_u32(object_slot, object);
        let class_slot = e.mem.alloc(4);
        e.mem.set_u32(class_slot, class);
        (this, object_slot, class_slot)
    }

    /// The class records of the derivation tests and the classes they were asked about.
    fn derivation_tests(log: &Log) -> (Vec<u32>, Vec<u32>) {
        let tests = calls(log, IS_DERIVED);
        (
            tests.iter().map(|test| test[0]).collect(),
            tests.iter().map(|test| test[2]).collect(),
        )
    }

    #[test]
    fn a_first_kind_object_is_replaced_by_the_0x70_copy() {
        let mut e = engine();
        let log = new_log();
        let allocator = listener_doubles(&mut e, &log, [true, false, false, false]);
        let (this, object_slot, class_slot) = listener(&mut e, 0x0bad_0001, 0x1111);
        e.call(0x00cc_08f0, &args![this, object_slot, class_slot]);
        assert_eq!(calls(&log, ALLOC_DOUBLE), vec![vec![allocator, 0x70]]);
        let built = calls(&log, REPLACEMENT_A);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], 0x0bad_0001);
        assert_eq!(e.mem.u16(built[0][0] + 4), 0x70);
        assert_eq!(e.mem.u32(object_slot), NEW_OBJECTS[0]);
        assert_eq!(e.mem.u32(class_slot), REPLACED_A);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(e.mem.u32(this + 8)), NEW_OBJECTS[0]);
        // All four tests ran, the later ones against the new class.
        let (records, classes) = derivation_tests(&log);
        assert_eq!(records, vec![CLASS_A, CLASS_B, CLASS_C, CLASS_D]);
        assert_eq!(classes, vec![0x1111, REPLACED_A, REPLACED_A, REPLACED_A]);
    }

    #[test]
    fn an_object_that_already_has_the_replacement_class_is_left_alone() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log, [true, false, false, false]);
        let (this, object_slot, class_slot) = listener(&mut e, 0x0bad_0001, REPLACED_A);
        e.call(0x00cc_08f0, &args![this, object_slot, class_slot]);
        assert!(calls(&log, ALLOC_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(object_slot), 0x0bad_0001);
        assert_eq!(e.mem.u32(class_slot), REPLACED_A);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn a_second_kind_object_gets_the_0x110_mesh_shape_copy_and_the_callback_ends() {
        let mut e = engine();
        let log = new_log();
        let allocator = listener_doubles(&mut e, &log, [false, true, true, true]);
        let (this, object_slot, class_slot) = listener(&mut e, 0x0bad_0002, 0x2222);
        e.call(0x00cc_08f0, &args![this, object_slot, class_slot]);
        assert_eq!(calls(&log, ALLOC_DOUBLE), vec![vec![allocator, 0x110]]);
        let built = calls(&log, REPLACEMENT_B);
        assert_eq!(built[0][1], 0x0bad_0002);
        assert_eq!(e.mem.u16(built[0][0] + 4), 0x110);
        assert_eq!(e.mem.u32(object_slot), NEW_OBJECTS[1]);
        assert_eq!(e.mem.u32(class_slot), REPLACED_B);
        assert_eq!(e.mem.u32(e.mem.u32(this + 8)), NEW_OBJECTS[1]);
        let (records, _) = derivation_tests(&log);
        assert_eq!(records, vec![CLASS_A, CLASS_B]);
    }

    #[test]
    fn a_third_kind_object_is_copied_only_when_its_byte_is_two() {
        for (byte, copied) in [(2u8, true), (3, false)] {
            let mut e = engine();
            let log = new_log();
            let allocator = listener_doubles(&mut e, &log, [false, true, true, true]);
            let object = e.mem.alloc(0x40);
            e.mem.set_u8(object + 0x1d, byte);
            // The class already is the second replacement class, so the second test is skipped.
            let (this, object_slot, class_slot) = listener(&mut e, object, REPLACED_B);
            e.call(0x00cc_08f0, &args![this, object_slot, class_slot]);
            let (records, _) = derivation_tests(&log);
            assert_eq!(records, vec![CLASS_A, CLASS_B, CLASS_C]);
            if copied {
                assert_eq!(calls(&log, ALLOC_DOUBLE), vec![vec![allocator, 0x80]]);
                let built = calls(&log, REPLACEMENT_C);
                assert_eq!(built[0][1], object);
                assert_eq!(e.mem.u16(built[0][0] + 4), 0x80);
                assert_eq!(e.mem.u32(object_slot), NEW_OBJECTS[2]);
                assert_eq!(e.mem.u32(class_slot), REPLACED_C);
                assert_eq!(e.mem.u32(this + 0xc), 1);
            } else {
                assert!(calls(&log, ALLOC_DOUBLE).is_empty());
                assert_eq!(e.mem.u32(object_slot), object);
                assert_eq!(e.mem.u32(class_slot), REPLACED_B);
            }
        }
    }

    #[test]
    fn a_fourth_kind_object_is_forgotten_when_its_owner_points_back_to_it() {
        for (back_pointer, forgotten) in [(true, true), (false, false)] {
            let mut e = engine();
            let log = new_log();
            listener_doubles(&mut e, &log, [false, false, false, true]);
            let object = e.mem.alloc(0x40);
            let owner = e.mem.alloc(0x40);
            e.mem.set_u32(object + 8, owner);
            e.mem
                .set_u32(owner + 0x24, if back_pointer { object } else { 1 });
            let (this, object_slot, class_slot) = listener(&mut e, object, 0x3333);
            e.call(0x00cc_08f0, &args![this, object_slot, class_slot]);
            let (records, _) = derivation_tests(&log);
            assert_eq!(records, vec![CLASS_A, CLASS_B, CLASS_C, CLASS_D]);
            assert_eq!(e.mem.u32(object_slot), if forgotten { 0 } else { object });
            assert_eq!(e.mem.u32(class_slot), if forgotten { 0 } else { 0x3333 });
            assert_eq!(e.mem.u32(this + 0xc), 0);
        }
    }

    #[test]
    fn a_fourth_kind_object_without_an_owner_is_left_alone() {
        let mut e = engine();
        let log = new_log();
        listener_doubles(&mut e, &log, [false, false, false, true]);
        let object = e.mem.alloc(0x40);
        let (this, object_slot, class_slot) = listener(&mut e, object, 0x3333);
        e.call(0x00cc_08f0, &args![this, object_slot, class_slot]);
        assert_eq!(e.mem.u32(object_slot), object);
        assert_eq!(e.mem.u32(class_slot), 0x3333);
    }
}
