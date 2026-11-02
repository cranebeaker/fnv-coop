//! `BSHavok/bhksimpleshapephantom.obj` (Xbox PDB source unit), subsystem `BSHavok`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkSimpleShapePhantom` (vtable `010305dc`, RTTI-confirmed) is a `bhkShapePhantom`; its
//! Cinfo carries a 0x40-byte transform (four 16-byte vectors) at `+0x20`.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame or
//! a `/GS` stack-cookie check (`00ec408c`), neither of which is modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkSerializable::Kill` (`00c876a0`), `bhkSerializable::CloneCritLock` / `CloneCritUnlock`
/// (`00c876f0`, `00c87700`) and `~bhkShapePhantom` (`00c9eb30`), engine map names.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
const SHAPE_PHANTOM_DESTRUCT: u32 = 0x00c9_eb30;
/// `bhkShapePhantom::GetViewerStrings` (`00c9ed30`) and `bhkWorldObject::CopyMembers`
/// (`00c85900`, thiscall: clone, process), engine map names.
const SHAPE_PHANTOM_GET_VIEWER_STRINGS: u32 = 0x00c9_ed30;
const WORLD_OBJECT_COPY_MEMBERS: u32 = 0x00c8_5900;
/// The constructor of the 0x18-byte phantom wrapper (`00c6d190`, thiscall on the new block;
/// the engine map does not name it) and the heap allocator (`00aa13e0`, cdecl: size).
const PHANTOM_CONSTRUCT: u32 = 0x00c6_d190;
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// `hkRotation::renormalize` (engine map name, `00cb2fc0`, thiscall on the rotation).
const ROTATION_RENORMALIZE: u32 = 0x00cb_2fc0;
/// The memory router (`00c85750`): its field `+0x10` is the allocator (virtual slot 4 allocates).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The constructor of the Havok simple shape phantom (`00d1f730`, thiscall on the new block,
/// three stack words: the Cinfo's `+4`, the address of the transform, the Cinfo's `+0`; the
/// engine map does not name it) and the reference count decrement of a Havok object
/// (`00c905b0`; the engine map does not name it).
const HAVOK_PHANTOM_CONSTRUCT: u32 = 0x00d1_f730;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The vtable `~bhkSimpleShapePhantom` stores and the object counter it lowers.
const SIMPLE_SHAPE_PHANTOM_VTABLE: u32 = 0x0103_05dc;
const SIMPLE_SHAPE_PHANTOM_COUNT: u32 = 0x0126_822c;
/// The class name word (`01268230`) `GetViewerStrings` passes to `NiGetViewerString`.
const CLASS_RTTI: u32 = 0x0126_8230;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

// Virtual slots (byte offsets).
/// `SethkObject`.
const VSLOT_SET_HK_OBJECT: u32 = 0x8c;
/// Answers the world of the object (unconfirmed name).
const VSLOT_GET_WORLD: u32 = 0x94;
/// `PostInit` (the Cinfo).
const VSLOT_POST_INIT: u32 = 0xbc;

/// The size of the Havok simple shape phantom block and the transform in the Cinfo.
const HAVOK_PHANTOM_SIZE: u32 = 0x170;
const CINFO_TRANSFORM: u32 = 0x20;
const TRANSFORM_SIZE: u32 = 0x40;

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

// Translated from 00c9eec0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9eec0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSimpleShapePhantom::~bhkSimpleShapePhantom` (engine map name): stores the vtable, runs
/// `Kill` (`00c876a0`), lowers the object counter (`0126822c`) and runs `~bhkShapePhantom`
/// (`00c9eb30`).
pub fn bhk_simple_shape_phantom_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SIMPLE_SHAPE_PHANTOM_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(SIMPLE_SHAPE_PHANTOM_COUNT);
    e.mem
        .set_u32(SIMPLE_SHAPE_PHANTOM_COUNT, count.wrapping_sub(1));
    e.call(SHAPE_PHANTOM_DESTRUCT, &args![this]);
}

// Translated from 00c9ef30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9ef30 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name (the map places it in `bhksimpleshapephantom.obj`). Allocates 0x18 bytes
/// (`operator new`, `00aa13e0`) and constructs the wrapper in them (`00c6d190`); answers the
/// constructor's answer, or 0 when the allocation failed.
pub fn fn_00c9ef30(e: &mut Engine) -> Ptr {
    let block = e.call(HEAP_ALLOCATE, &args![0x18u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    Ptr::new(e.call(PHANTOM_CONSTRUCT, &args![block]).u32())
}

// Translated from 00c9efa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9efa0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSimpleShapePhantom::CreateClone` (engine map name), `(process)`: inside the
/// `TLS + 0x2b4 = 0x10` scope and the clone lock (`00c876f0`, `00c87700`), makes a new wrapper
/// as `00c9ef30` does (0 when the allocation fails) and runs `bhkWorldObject::CopyMembers`
/// (`00c85900`) on this object with the clone and `process`. Answers the clone.
pub fn bhk_simple_shape_phantom_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let scope = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let block = e.call(HEAP_ALLOCATE, &args![0x18u32]).u32();
    let mut clone = 0;
    if block != 0 {
        clone = e.call(PHANTOM_CONSTRUCT, &args![block]).u32();
    }
    e.call(WORLD_OBJECT_COPY_MEMBERS, &args![this, clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, scope);
    Ptr::new(clone)
}

// Translated from 00c9f050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f050 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkSimpleShapePhantom::Init` (engine map name), `(cinfo)`: inside the `TLS + 0x2b4 = 0x10`
/// scope, copies the Cinfo's transform (the 16 floats at `+0x20`, bit for bit) to a local one
/// and renormalizes its rotation (`00cb2fc0`); has the allocator of the memory router
/// (`00c85750`, `+0x10`, virtual slot 4) allocate 0x170 bytes, stores 0x170 in the 16-bit word
/// at `+4` of the block and constructs the Havok phantom in it (`00d1f730`, with the Cinfo's
/// `+4`, the local transform and the Cinfo's `+0`); sets it as the Havok object (virtual
/// `+0x8c`), drops the reference the construction left (`00c905b0`) and runs the virtual at
/// `+0xbc` with the Cinfo.
pub fn bhk_simple_shape_phantom_init(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    let scope = enter_scope(e);
    e.with_stack(TRANSFORM_SIZE, |e, transform| {
        for word in 0..TRANSFORM_SIZE / 4 {
            let value = e.mem.u32(cinfo.addr() + CINFO_TRANSFORM + 4 * word);
            e.mem.set_u32(transform.addr() + 4 * word, value);
        }
        e.call(ROTATION_RENORMALIZE, &args![transform]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let block = e.vcall(allocator, 4, &args![HAVOK_PHANTOM_SIZE]).u32();
        e.mem.set_u16(block + 4, HAVOK_PHANTOM_SIZE as u16);
        let first = e.mem.u32(cinfo.addr() + 4);
        let second = e.mem.u32(cinfo.addr());
        let object = e
            .call(
                HAVOK_PHANTOM_CONSTRUCT,
                &args![block, first, transform, second],
            )
            .u32();
        e.vcall(this.addr(), VSLOT_SET_HK_OBJECT, &args![object]);
        e.call(REMOVE_REFERENCE, &args![object]);
        e.vcall(this.addr(), VSLOT_POST_INIT, &args![cinfo]);
    });
    leave_scope(e, scope);
}

// Translated from 00c9f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhksimpleshapephantom.obj`). `(list)`: takes the world
/// (virtual `+0x94`), its `+0x78` (the dispatcher) and the dispatcher's first word (the
/// table); then, for each element of the array at `+0x160` of the Havok object (`this + 8`),
/// counted by `+0x164`, from the last to the first, as long as the two words at `list + 0x10`
/// and `list + 0x14` differ, calls the handler the table names, cdecl with the Havok
/// object's interface (`+0x10`), the element, the dispatcher and `list`: the handler is the
/// word at `table + 0x9a8 + 20 * kind`, where `kind` is the byte at
/// `table + ((a + 0xd) << 5) + b`, `a` the word at `+0xc` of what the interface's first word
/// points to and `b` the word at `+0xc` of what the element's first word points to.
pub fn fn_00c9f1e0(e: &mut Engine, this: Ptr, list: Ptr) {
    let world = e.vcall(this.addr(), VSLOT_GET_WORLD, &args![]).u32();
    let dispatcher = e.mem.u32(world + 0x78);
    let table = e.mem.u32(dispatcher);
    let object = e.mem.u32(this.addr() + 8);
    let interface = if object != 0 { object + 0x10 } else { 0 };
    let mut index = e.mem.u32(object + 0x164).wrapping_sub(1);
    while (index as i32) >= 0 {
        if e.mem.u32(list.addr() + 0x14) == e.mem.u32(list.addr() + 0x10) {
            break;
        }
        let elements = e.mem.u32(object + 0x160);
        let interface_class = e.mem.u32(interface);
        let first = e.mem.u32(interface_class + 0xc);
        let element = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        let element_class = e.mem.u32(element);
        let key = first
            .wrapping_add(0xd)
            .wrapping_shl(5)
            .wrapping_add(e.mem.u32(element_class + 0xc));
        let kind = u32::from(e.mem.u8(table.wrapping_add(key)));
        let handler = e.mem.u32(
            table
                .wrapping_add(kind.wrapping_mul(20))
                .wrapping_add(0x9a8),
        );
        e.call(handler, &args![interface, element, dispatcher, list]);
        index = index.wrapping_sub(1);
    }
}

// Translated from 00c9f270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f270 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSimpleShapePhantom::GetViewerStrings` (engine map name), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the shape phantom's strings (`00c9ed30`) and the text
/// `NiGetViewerString` makes of the word at `01268230`, stored in the string array (grown by
/// `0096ad30` when its size, `+0xa`, has reached its capacity, `+8`; stored by `0096ae90`).
pub fn bhk_simple_shape_phantom_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let scope = enter_scope(e);
    e.call(SHAPE_PHANTOM_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CLASS_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    let array = strings.addr();
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
    leave_scope(e, scope);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9eec0, bhk_simple_shape_phantom_destructor(Ptr)),
        entry!(0x00c9ef30, fn_00c9ef30() -> Ptr),
        entry!(
            0x00c9efa0,
            bhk_simple_shape_phantom_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00c9f050, bhk_simple_shape_phantom_init(Ptr, Ptr)),
        entry!(0x00c9f1e0, fn_00c9f1e0(Ptr, Ptr)),
        entry!(
            0x00c9f270,
            bhk_simple_shape_phantom_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const GET_WORLD_SLOT: u32 = 0x7000_0094;
    const SET_HK_OBJECT_SLOT: u32 = 0x7000_008c;
    const POST_INIT_SLOT: u32 = 0x7000_00bc;
    const ALLOC_SLOT: u32 = 0x7000_0004;
    const OBJECT_VTABLE: u32 = 0x7001_1000;
    const ALLOCATOR_VTABLE: u32 = 0x7001_2000;
    const ROUTER_OBJECT: u32 = 0x7100_0000;
    const HANDLER_ONE: u32 = 0x7000_2001;
    const HANDLER_TWO: u32 = 0x7000_2002;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// An object with the vtable slots the unit calls.
    fn object(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 0x30];
        slots[0x8c / 4] = SET_HK_OBJECT_SLOT;
        slots[0x94 / 4] = GET_WORLD_SLOT;
        slots[0xbc / 4] = POST_INIT_SLOT;
        e.put_vtable(OBJECT_VTABLE, &slots);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, OBJECT_VTABLE);
        this
    }

    #[test]
    fn bhk_simple_shape_phantom_destructor_runs_kill_lowers_the_counter_and_runs_the_base() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(SHAPE_PHANTOM_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(SIMPLE_SHAPE_PHANTOM_COUNT, 7);
        e.call(0x00c9_eec0, &args![this]);
        assert_eq!(e.mem.u32(this), SIMPLE_SHAPE_PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(SIMPLE_SHAPE_PHANTOM_COUNT), 6);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, SHAPE_PHANTOM_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, SHAPE_PHANTOM_DESTRUCT]),
            vec![SERIALIZABLE_KILL, SHAPE_PHANTOM_DESTRUCT]
        );
    }

    #[test]
    fn fn_00c9ef30_constructs_the_wrapper_in_a_new_block() {
        let mut e = logged_engine();
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PHANTOM_CONSTRUCT, |_, a| (a[0] + 0x100).into_ret());
        let result = e.call(0x00c9_ef30, &args![]).u32();
        let constructed = calls_to(&e, PHANTOM_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        assert_eq!(result, constructed[0][0] + 0x100);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x18]]);
    }

    #[test]
    fn fn_00c9ef30_answers_zero_when_the_allocation_fails() {
        let mut e = logged_engine();
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        e.register(PHANTOM_CONSTRUCT, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00c9_ef30, &args![]).u32(), 0);
        assert!(calls_to(&e, PHANTOM_CONSTRUCT).is_empty());
    }

    #[test]
    fn bhk_simple_shape_phantom_create_clone_copies_the_members_under_the_clone_lock() {
        let mut e = logged_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PHANTOM_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(WORLD_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let clone = e.call(0x00c9_efa0, &args![this, 0x9999u32]).u32();
        assert_eq!(calls_to(&e, PHANTOM_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_COPY_MEMBERS),
            vec![vec![this, clone, 0x9999]]
        );
        assert_eq!(
            order_of(
                &e,
                &[
                    CLONE_LOCK,
                    HEAP_ALLOCATE,
                    WORLD_OBJECT_COPY_MEMBERS,
                    CLONE_UNLOCK
                ]
            ),
            vec![
                CLONE_LOCK,
                HEAP_ALLOCATE,
                WORLD_OBJECT_COPY_MEMBERS,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_simple_shape_phantom_create_clone_copies_into_a_null_clone_when_the_allocation_fails() {
        let mut e = logged_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        e.register(WORLD_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let clone = e.call(0x00c9_efa0, &args![this, 0x9999u32]).u32();
        assert_eq!(clone, 0);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_COPY_MEMBERS),
            vec![vec![this, 0, 0x9999]]
        );
    }

    #[test]
    fn bhk_simple_shape_phantom_init_builds_the_havok_phantom_from_the_cinfo() {
        let mut e = logged_engine();
        e.register(ROTATION_RENORMALIZE, |e, a| {
            e.mem.set_u32(a[0], 0x5555);
            Ret::default()
        });
        let mut allocator_slots = vec![0u32; 2];
        allocator_slots[1] = ALLOC_SLOT;
        e.put_vtable(ALLOCATOR_VTABLE, &allocator_slots);
        e.map(ROUTER_OBJECT, 0x40);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, ROUTER_OBJECT + 0x20);
        e.mem.set_u32(ROUTER_OBJECT + 0x20, ALLOCATOR_VTABLE);
        e.register(MEMORY_ROUTER, |_, _| ROUTER_OBJECT.into_ret());
        e.register(ALLOC_SLOT, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(HAVOK_PHANTOM_CONSTRUCT, |e, a| {
            for word in 0..16 {
                let value = e.mem.u32(a[2] + 4 * word);
                e.mem.set_u32(a[0] + 0x100 + 4 * word, value);
            }
            a[0].into_ret()
        });
        e.register(SET_HK_OBJECT_SLOT, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(POST_INIT_SLOT, |_, _| Ret::default());
        let this = object(&mut e);
        let cinfo = e.mem.alloc(0x70);
        e.mem.set_u32(cinfo, 0xaaaa);
        e.mem.set_u32(cinfo + 4, 0xbbbb);
        for word in 0..16u32 {
            e.mem.set_u32(cinfo + 0x20 + 4 * word, 0x100 + word);
        }
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00c9_f050, &args![this, cinfo]);
        let constructed = calls_to(&e, HAVOK_PHANTOM_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        let block = constructed[0][0];
        assert_eq!(constructed[0][1], 0xbbbb);
        assert_eq!(constructed[0][3], 0xaaaa);
        assert_eq!(e.mem.u16(block + 4), 0x170);
        // The local transform: renormalized in place after the bit-exact copy.
        assert_eq!(e.mem.u32(block + 0x100), 0x5555);
        let copied: Vec<u32> = (1..16).map(|w| e.mem.u32(block + 0x100 + 4 * w)).collect();
        assert_eq!(copied, (0x101..0x110).collect::<Vec<u32>>());
        assert_eq!(calls_to(&e, ALLOC_SLOT).len(), 1);
        assert_eq!(calls_to(&e, ALLOC_SLOT)[0][1], 0x170);
        assert_eq!(calls_to(&e, SET_HK_OBJECT_SLOT), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, cinfo]]);
        assert_eq!(
            order_of(
                &e,
                &[
                    ROTATION_RENORMALIZE,
                    ALLOC_SLOT,
                    HAVOK_PHANTOM_CONSTRUCT,
                    SET_HK_OBJECT_SLOT,
                    REMOVE_REFERENCE,
                    POST_INIT_SLOT
                ]
            ),
            vec![
                ROTATION_RENORMALIZE,
                ALLOC_SLOT,
                HAVOK_PHANTOM_CONSTRUCT,
                SET_HK_OBJECT_SLOT,
                REMOVE_REFERENCE,
                POST_INIT_SLOT
            ]
        );
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    /// A phantom whose Havok object holds two elements; the handlers `HANDLER_ONE` and
    /// `HANDLER_TWO` are what the table names for them. Answers the phantom, the Havok
    /// object, the elements, the dispatcher and a list with differing begin and end words.
    fn dispatch_setup(e: &mut Engine) -> (u32, u32, [u32; 2], u32, u32) {
        let table = e.mem.alloc(0xb00);
        let dispatcher = e.mem.alloc(0x10);
        e.mem.set_u32(dispatcher, table);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x78, dispatcher);
        e.register_double(GET_WORLD_SLOT, move |_, _| world.into_ret());
        let this = object(e);
        let havok = e.mem.alloc(0x180);
        e.mem.set_u32(this + 8, havok);
        // The interface (`havok + 0x10`) points at a structure whose `+0xc` is 1.
        let interface_class = e.mem.alloc(0x20);
        e.mem.set_u32(interface_class + 0xc, 1);
        e.mem.set_u32(havok + 0x10, interface_class);
        let list = e.mem.alloc(8);
        e.mem.set_u32(havok + 0x160, list);
        e.mem.set_u32(havok + 0x164, 2);
        let mut elements = [0u32; 2];
        for (n, kind) in [(0usize, 1u8), (1, 2)] {
            let class = e.mem.alloc(0x20);
            e.mem.set_u32(class + 0xc, 2 + n as u32);
            let element = e.mem.alloc(0x10);
            e.mem.set_u32(element, class);
            e.mem.set_u32(list + 4 * n as u32, element);
            elements[n] = element;
            let key = ((1 + 0xd) << 5) + 2 + n as u32;
            e.mem.set_u8(table + key, kind);
            let handler = [HANDLER_ONE, HANDLER_TWO][n];
            e.mem.set_u32(table + 0x9a8 + 20 * u32::from(kind), handler);
        }
        let argument = e.mem.alloc(0x20);
        e.mem.set_u32(argument + 0x10, 1);
        e.mem.set_u32(argument + 0x14, 2);
        (this, havok, elements, dispatcher, argument)
    }

    #[test]
    fn fn_00c9f1e0_calls_the_handler_of_each_element_from_the_last() {
        let mut e = logged_engine();
        e.register(HANDLER_ONE, |_, _| Ret::default());
        e.register(HANDLER_TWO, |_, _| Ret::default());
        let (this, havok, elements, dispatcher, argument) = dispatch_setup(&mut e);
        e.call(0x00c9_f1e0, &args![this, argument]);
        assert_eq!(
            calls_to(&e, HANDLER_TWO),
            vec![vec![havok + 0x10, elements[1], dispatcher, argument]]
        );
        assert_eq!(
            calls_to(&e, HANDLER_ONE),
            vec![vec![havok + 0x10, elements[0], dispatcher, argument]]
        );
        assert_eq!(
            order_of(&e, &[HANDLER_ONE, HANDLER_TWO]),
            vec![HANDLER_TWO, HANDLER_ONE]
        );
    }

    #[test]
    fn fn_00c9f1e0_does_nothing_for_an_empty_list_or_a_zero_count() {
        let mut e = logged_engine();
        e.register(HANDLER_ONE, |_, _| Ret::default());
        e.register(HANDLER_TWO, |_, _| Ret::default());
        let (this, havok, _, _, argument) = dispatch_setup(&mut e);
        e.mem.set_u32(argument + 0x14, 1);
        e.call(0x00c9_f1e0, &args![this, argument]);
        assert!(calls_to(&e, HANDLER_ONE).is_empty());
        assert!(calls_to(&e, HANDLER_TWO).is_empty());
        e.mem.set_u32(argument + 0x14, 2);
        e.mem.set_u32(havok + 0x164, 0);
        e.call(0x00c9_f1e0, &args![this, argument]);
        assert!(calls_to(&e, HANDLER_ONE).is_empty());
        assert!(calls_to(&e, HANDLER_TWO).is_empty());
    }

    #[test]
    fn bhk_simple_shape_phantom_get_viewer_strings_stores_the_class_name() {
        let mut e = logged_engine();
        e.register(SHAPE_PHANTOM_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            Ret::default()
        });
        e.mem.set_u32(CLASS_RTTI, 0x4242);
        let this = e.mem.alloc(0x20);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00c9_f270, &args![this, array]);
        assert_eq!(
            calls_to(&e, SHAPE_PHANTOM_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4242]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}
