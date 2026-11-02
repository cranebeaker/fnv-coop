//! `BSHavok/bhksphereshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkSphereShape` (vtable `01030cdc`, RTTI-confirmed) wraps the Havok sphere shape at `+8`
//! (its radius at `+0x10`); its Cinfo (8 bytes, at `+0xc`) holds the user data at `+0` and the
//! radius at `+4`, and is made by `CreateCinfo`.
//!
//! Floats: x87 `FLD`/`FSTP` copies go through [`x87_quiet`]; the x87 arithmetic is done in
//! `f64` and rounded to `f32` where the exe stores to memory.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame,
//! which is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkSerializable::Kill` (`00c876a0`), the clone lock (`00c876f0`, `00c87700`),
/// `~bhkConvexShape` (`00c9fb00`), `bhkConvexShape::CopyCinfo` (`00c9fb70`) and
/// `bhkConvexShape::GetViewerStrings` (`00c9fc50`), `bhkShape::CopyMembers` (`00c84df0`), engine
/// map names.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
const CONVEX_DESTRUCT: u32 = 0x00c9_fb00;
const CONVEX_COPY_CINFO: u32 = 0x00c9_fb70;
const CONVEX_GET_VIEWER_STRINGS: u32 = 0x00c9_fc50;
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
/// The constructor of the sphere shape (`0094dbb0`, thiscall on the new block, answers it; the
/// engine map does not name it) and the heap allocator (`00aa13e0`, cdecl: size).
const SPHERE_SHAPE_CONSTRUCT: u32 = 0x0094_dbb0;
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// The memory manager (`011f6238`), allocating through `00aa3e40` (thiscall, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The memory router (`00c85750`): its field `+0x10` is the allocator (virtual slot 4 allocates).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The constructor of the Havok sphere (`00cd8160`, `hkpSphereShape::hkpSphereShape` in the
/// engine map; thiscall on the new block, one stack word: the radius) and the reference
/// count decrement of a Havok object (`00c905b0`; the engine map does not name it).
const HAVOK_SPHERE_CONSTRUCT: u32 = 0x00cd_8160;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `BSTestObjects::CreateSphere` (`00c56680`, engine map name, cdecl: the radius, 10, 10, 0).
const CREATE_SPHERE: u32 = 0x00c5_6680;
/// The string interning (`00a5b690`, cdecl: text; unnamed), the function that stores an
/// interned name in a geometry (`00a5b950`, thiscall on the geometry, one stack word: the
/// address of the handle; unnamed) and `_atexit` (`00ec658f`, cdecl: function).
const INTERN_STRING: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const AT_EXIT: u32 = 0x00ec_658f;
/// The display name `"bhkSphereShape"` (`010c3f88`), its interned handle (`01268290`), the
/// one-time guard (`01268294`, bit 0) and the cleanup registered with `_atexit` (`00fde1d0`).
const NAME_TEXT: u32 = 0x010c_3f88;
const NAME_HANDLE: u32 = 0x0126_8290;
const NAME_GUARD: u32 = 0x0126_8294;
const NAME_CLEANUP: u32 = 0x00fd_e1d0;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The vtable `~bhkSphereShape` stores and the object counter it lowers.
const SPHERE_SHAPE_VTABLE: u32 = 0x0103_0cdc;
const SPHERE_SHAPE_COUNT: u32 = 0x0126_8280;
/// The class name word (`01268288`) `GetViewerStrings` passes to `NiGetViewerString`.
const CLASS_RTTI: u32 = 0x0126_8288;
/// `float` default radius of a Cinfo (`010c72bc`) and the factor (`float`, `011b15c8`) the
/// display sphere's radius is scaled by.
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
const DISPLAY_SCALE: u32 = 0x011b_15c8;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

// Virtual slots (byte offsets).
/// `SethkObject`, `PostInit` (the Cinfo), `CreateCinfo` (out parameter: the address of a flag
/// byte), `CopyMembers` (clone, process) and `BuildDisplayFromhkGeometry`-style attach of the
/// node (display, flag).
const VSLOT_SET_HK_OBJECT: u32 = 0x8c;
const VSLOT_POST_INIT: u32 = 0xbc;
const VSLOT_CREATE_CINFO: u32 = 0xb4;
const VSLOT_COPY_MEMBERS: u32 = 0xc4;
const VSLOT_NODE_ATTACH: u32 = 0xdc;

/// The Havok sphere's radius (`+0x10`), the scale in the clone process (`+0x10`) and the
/// Cinfo's radius (`+4`).
const HAVOK_RADIUS: u32 = 0x10;
const PROCESS_SCALE: u32 = 0x10;
const CINFO_RADIUS: u32 = 4;
/// The size of the Havok sphere block.
const HAVOK_SPHERE_SIZE: u32 = 0x20;

layout! {
    /// `bhkSphereShape` (0x14 bytes): a `bhkShape`.
    pub struct BhkSphereShape: 0x14 {
        /// `phkObject` (Xbox PDB): the Havok sphere.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB): the Cinfo.
        0x0C pInfo: Ptr,
    }
}

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

/// An x87 `FLD dword` of the float at `address`.
fn load_quiet(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.mem.f32(address))
}

// Translated from 00ca0660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0660 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSphereShape::~bhkSphereShape` (engine map name): stores the vtable, runs `Kill`
/// (`00c876a0`), lowers the object counter (`01268280`) and runs `~bhkConvexShape` (`00c9fb00`).
pub fn bhk_sphere_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SPHERE_SHAPE_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(SPHERE_SHAPE_COUNT);
    e.mem.set_u32(SPHERE_SHAPE_COUNT, count.wrapping_sub(1));
    e.call(CONVEX_DESTRUCT, &args![this]);
}

// Translated from 00ca06d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca06d0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name (the map places it in `bhksphereshape.obj`). Allocates 0x14 bytes
/// (`operator new`, `00aa13e0`) and constructs the sphere shape in them (`0094dbb0`); answers
/// the constructor's answer, or 0 when the allocation failed.
pub fn fn_00ca06d0(e: &mut Engine) -> Ptr {
    let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    Ptr::new(e.call(SPHERE_SHAPE_CONSTRUCT, &args![block]).u32())
}

// Translated from 00ca0740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSphereShape::CopyMembers` (engine map name), `(clone, process)`: takes the shape's
/// Cinfo (`CreateCinfo`, virtual `+0xb4`), multiplies its radius (`+4`) by the float at
/// `process + 0x10` and runs `bhkShape::CopyMembers` (`00c84df0`).
pub fn bhk_sphere_shape_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, process: Ptr) {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), VSLOT_CREATE_CINFO, &args![created])
            .u32()
    });
    let scale = load_quiet(e, process.addr() + PROCESS_SCALE);
    let radius = load_quiet(e, cinfo + CINFO_RADIUS);
    e.mem.set_f32(
        cinfo + CINFO_RADIUS,
        (f64::from(scale) * f64::from(radius)) as f32,
    );
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, process]);
}

// Translated from 00ca0780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0780 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSphereShape::CreateClone` (engine map name), `(process)`: inside the
/// `TLS + 0x2b4 = 0x10` scope and the clone lock (`00c876f0`, `00c87700`), makes a new shape
/// as `00ca06d0` does (0 when the allocation fails) and runs `CopyMembers` (virtual `+0xc4`,
/// with the clone and `process`) on this one. Answers the clone.
pub fn bhk_sphere_shape_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let scope = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    let mut clone = 0;
    if block != 0 {
        clone = e.call(SPHERE_SHAPE_CONSTRUCT, &args![block]).u32();
    }
    e.vcall(this.addr(), VSLOT_COPY_MEMBERS, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, scope);
    Ptr::new(clone)
}

// Translated from 00ca0840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0840 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSphereShape::CreateCinfo` (engine map name), `(created)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, writes whether there was no Cinfo yet to the byte at `created`;
/// if so, allocates an 8-byte Cinfo from the memory manager (`00aa3e40`) holding zero user
/// data and the default radius (`010c72bc`), stores it at `+0xc` and, when the shape has its
/// Havok sphere (`+8`), runs `CopyCinfo` (`00c9fb70`) on it. Answers the stored Cinfo.
pub fn bhk_sphere_shape_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkSphereShape>,
    created: Ptr,
) -> Ptr {
    let scope = enter_scope(e);
    let is_new = e.get(this, BhkSphereShape::pInfo).addr() == 0;
    e.mem.set_u8(created.addr(), is_new as u8);
    if is_new {
        let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            let radius = load_quiet(e, DEFAULT_RADIUS);
            e.mem.set_f32(block + CINFO_RADIUS, radius);
        }
        e.set(this, BhkSphereShape::pInfo, Ptr::new(block));
        if e.get(this, BhkSphereShape::phkObject).addr() != 0 {
            e.call(CONVEX_COPY_CINFO, &args![this, block]);
        }
    }
    let cinfo = e.get(this, BhkSphereShape::pInfo);
    leave_scope(e, scope);
    cinfo
}

// Translated from 00ca0900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0900 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSphereShape::Init` (engine map name), `(cinfo)`: inside the `TLS + 0x2b4 = 0x10` scope,
/// allocates the 0x20-byte Havok sphere from the memory router's allocator (virtual slot 4)
/// with its 16-bit size stored at `+4`, constructs it with `00cd8160` on the Cinfo's radius
/// (`+4`, through `FLD`/`FSTP`), hands it to `SethkObject` (virtual `+0x8c`), drops the
/// construction's reference (`00c905b0`) and runs the virtual at `+0xbc` with the Cinfo.
pub fn bhk_sphere_shape_init(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    let scope = enter_scope(e);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![HAVOK_SPHERE_SIZE]).u32();
    e.mem.set_u16(block + 4, HAVOK_SPHERE_SIZE as u16);
    let radius = x87_quiet_bits(e.mem.u32(cinfo.addr() + CINFO_RADIUS));
    let sphere = e.call(HAVOK_SPHERE_CONSTRUCT, &args![block, radius]).u32();
    e.vcall(this.addr(), VSLOT_SET_HK_OBJECT, &args![sphere]);
    e.call(REMOVE_REFERENCE, &args![sphere]);
    e.vcall(this.addr(), VSLOT_POST_INIT, &args![cinfo]);
    leave_scope(e, scope);
}

// Translated from 00ca09e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca09e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSphereShape::BuildDisplayGeometry` (engine map name), `(node, _unused_2)`: takes the
/// radius of the Havok sphere (`+8`, at `+0x10`; the default radius `010c72bc` when `this` or
/// the sphere is null), multiplies it by the float at `011b15c8` (rounded to a float) and
/// makes a display sphere with `BSTestObjects::CreateSphere` (`00c56680`: the scaled radius,
/// 10, 10, 0). If that worked: on the first use interns the name `"bhkSphereShape"` into
/// `01268290` (guard bit 0 of `01268294`, `_atexit(00fde1d0)`), stores the handle in the
/// sphere (`00a5b950`) and attaches the sphere to `node` (virtual `+0xdc`: the sphere, 1).
pub fn bhk_sphere_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    _unused_2: u32,
) {
    let mut havok = 0;
    if this.addr() != 0 {
        havok = e.mem.u32(this.addr() + 8);
    }
    let radius = if havok != 0 {
        load_quiet(e, havok + HAVOK_RADIUS)
    } else {
        load_quiet(e, DEFAULT_RADIUS)
    };
    let scale = load_quiet(e, DISPLAY_SCALE);
    let scaled = x87_quiet((f64::from(radius) * f64::from(scale)) as f32);
    let sphere = e
        .call(CREATE_SPHERE, &args![scaled.to_bits(), 10u32, 10u32, 0u32])
        .u32();
    if sphere == 0 {
        return;
    }
    let guard: u32 = e.global(NAME_GUARD);
    if guard & 1 == 0 {
        e.set_global(NAME_GUARD, guard | 1);
        let handle = e.call(INTERN_STRING, &args![NAME_TEXT]).u32();
        e.set_global(NAME_HANDLE, handle);
        e.call(AT_EXIT, &args![NAME_CLEANUP]);
    }
    e.call(SET_NAME, &args![sphere, NAME_HANDLE]);
    e.vcall(node.addr(), VSLOT_NODE_ATTACH, &args![sphere, 1u32]);
}

// Translated from 00ca0ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0ac0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSphereShape::GetViewerStrings` (engine map name), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the convex shape's strings (`00c9fc50`) and the text
/// `NiGetViewerString` makes of the word at `01268288`, stored in the string array (grown by
/// `0096ad30` when its size, `+0xa`, has reached its capacity, `+8`; stored by `0096ae90`).
pub fn bhk_sphere_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let scope = enter_scope(e);
    e.call(CONVEX_GET_VIEWER_STRINGS, &args![this, strings]);
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
        entry!(0x00ca0660, bhk_sphere_shape_destructor(Ptr)),
        entry!(0x00ca06d0, fn_00ca06d0() -> Ptr),
        entry!(0x00ca0740, bhk_sphere_shape_copy_members(Ptr, Ptr, Ptr)),
        entry!(0x00ca0780, bhk_sphere_shape_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00ca0840,
            bhk_sphere_shape_create_cinfo(Ptr<BhkSphereShape>, Ptr) -> Ptr
        ),
        entry!(0x00ca0900, bhk_sphere_shape_init(Ptr, Ptr)),
        entry!(
            0x00ca09e0,
            bhk_sphere_shape_build_display_geometry(Ptr, Ptr, u32)
        ),
        entry!(0x00ca0ac0, bhk_sphere_shape_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_HK_OBJECT_SLOT: u32 = 0x7000_008c;
    const CREATE_CINFO_SLOT: u32 = 0x7000_00b4;
    const POST_INIT_SLOT: u32 = 0x7000_00bc;
    const COPY_MEMBERS_SLOT: u32 = 0x7000_00c4;
    const NODE_ATTACH_SLOT: u32 = 0x7000_00dc;
    const ALLOC_SLOT: u32 = 0x7000_0004;
    const SHAPE_TABLE: u32 = 0x7001_1000;
    const ALLOCATOR_VTABLE: u32 = 0x7001_2000;
    const NODE_TABLE: u32 = 0x7001_3000;
    const ROUTER_OBJECT: u32 = 0x7100_0000;

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

    /// A sphere shape with the vtable slots the unit calls.
    fn shape(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 0x38];
        slots[0x8c / 4] = SET_HK_OBJECT_SLOT;
        slots[0xb4 / 4] = CREATE_CINFO_SLOT;
        slots[0xbc / 4] = POST_INIT_SLOT;
        slots[0xc4 / 4] = COPY_MEMBERS_SLOT;
        e.put_vtable(SHAPE_TABLE, &slots);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, SHAPE_TABLE);
        this
    }

    #[test]
    fn bhk_sphere_shape_destructor_runs_kill_lowers_the_counter_and_runs_the_convex_destructor() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(CONVEX_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(SPHERE_SHAPE_COUNT, 3);
        e.call(0x00ca_0660, &args![this]);
        assert_eq!(e.mem.u32(this), SPHERE_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(SPHERE_SHAPE_COUNT), 2);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONVEX_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, CONVEX_DESTRUCT]),
            vec![SERIALIZABLE_KILL, CONVEX_DESTRUCT]
        );
    }

    #[test]
    fn fn_00ca06d0_constructs_the_shape_in_a_new_block() {
        let mut e = logged_engine();
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SPHERE_SHAPE_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        let result = e.call(0x00ca_06d0, &args![]).u32();
        let constructed = calls_to(&e, SPHERE_SHAPE_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        assert_eq!(result, constructed[0][0] + 1);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x14]]);
    }

    #[test]
    fn fn_00ca06d0_answers_zero_when_the_allocation_fails() {
        let mut e = logged_engine();
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        e.register(SPHERE_SHAPE_CONSTRUCT, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00ca_06d0, &args![]).u32(), 0);
        assert!(calls_to(&e, SPHERE_SHAPE_CONSTRUCT).is_empty());
    }

    #[test]
    fn bhk_sphere_shape_copy_members_scales_the_radius_of_the_cinfo() {
        let mut e = logged_engine();
        e.register(SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        let cinfo = e.mem.alloc(8);
        e.mem.set_f32(cinfo + 4, 1.5);
        e.register_double(CREATE_CINFO_SLOT, move |_, _| cinfo.into_ret());
        let this = shape(&mut e);
        let clone = e.mem.alloc(0x14);
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 4.0);
        e.call(0x00ca_0740, &args![this, clone, process]);
        assert_eq!(e.mem.f32(cinfo + 4), 6.0);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, clone, process]]
        );
        assert_eq!(calls_to(&e, CREATE_CINFO_SLOT).len(), 1);
    }

    #[test]
    fn bhk_sphere_shape_copy_members_quiets_a_signalling_scale() {
        let mut e = logged_engine();
        e.register(SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        let cinfo = e.mem.alloc(8);
        e.mem.set_f32(cinfo + 4, 1.5);
        e.register_double(CREATE_CINFO_SLOT, move |_, _| cinfo.into_ret());
        let this = shape(&mut e);
        let process = e.mem.alloc(0x20);
        e.mem.set_u32(process + 0x10, 0x7f80_0001);
        e.call(0x00ca_0740, &args![this, 0u32, process]);
        assert!(e.mem.f32(cinfo + 4).is_nan());
    }

    #[test]
    fn bhk_sphere_shape_create_clone_copies_the_members_under_the_clone_lock() {
        let mut e = logged_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SPHERE_SHAPE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(COPY_MEMBERS_SLOT, |_, _| Ret::default());
        let this = shape(&mut e);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let clone = e.call(0x00ca_0780, &args![this, 0x9999u32]).u32();
        assert_eq!(calls_to(&e, SPHERE_SHAPE_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, COPY_MEMBERS_SLOT),
            vec![vec![this, clone, 0x9999]]
        );
        assert_eq!(
            order_of(
                &e,
                &[CLONE_LOCK, HEAP_ALLOCATE, COPY_MEMBERS_SLOT, CLONE_UNLOCK]
            ),
            vec![CLONE_LOCK, HEAP_ALLOCATE, COPY_MEMBERS_SLOT, CLONE_UNLOCK]
        );
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_sphere_shape_create_clone_copies_into_null_when_the_allocation_fails() {
        let mut e = logged_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        e.register(COPY_MEMBERS_SLOT, |_, _| Ret::default());
        let this = shape(&mut e);
        let clone = e.call(0x00ca_0780, &args![this, 0x9999u32]).u32();
        assert_eq!(clone, 0);
        assert_eq!(calls_to(&e, COPY_MEMBERS_SLOT), vec![vec![this, 0, 0x9999]]);
    }

    #[test]
    fn bhk_sphere_shape_create_cinfo_builds_the_cinfo_once() {
        let mut e = logged_engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(CONVEX_COPY_CINFO, |_, _| Ret::default());
        e.mem.set_f32(DEFAULT_RADIUS, 1.25);
        let this = shape(&mut e);
        let created = e.mem.alloc(4);
        e.mem.set_u8(created + 1, 0x55);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let cinfo = e.call(0x00ca_0840, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u8(created + 1), 0x55);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo), 0);
        assert_eq!(e.mem.f32(cinfo + 4), 1.25);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 8]]);
        // Without a Havok sphere the Cinfo is not filled in.
        assert!(calls_to(&e, CONVEX_COPY_CINFO).is_empty());
        let again = e.call(0x00ca_0840, &args![this, created]).u32();
        assert_eq!(again, cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_sphere_shape_create_cinfo_copies_the_havok_sphere_into_the_new_cinfo() {
        let mut e = logged_engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(CONVEX_COPY_CINFO, |_, _| Ret::default());
        let this = shape(&mut e);
        e.mem.set_u32(this + 8, 0x4000);
        let created = e.mem.alloc(4);
        let cinfo = e.call(0x00ca_0840, &args![this, created]).u32();
        assert_eq!(calls_to(&e, CONVEX_COPY_CINFO), vec![vec![this, cinfo]]);
    }

    #[test]
    fn bhk_sphere_shape_create_cinfo_stores_a_null_cinfo_when_the_allocation_fails() {
        let mut e = logged_engine();
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.register(CONVEX_COPY_CINFO, |_, _| Ret::default());
        let this = shape(&mut e);
        e.mem.set_u32(this + 8, 0x4000);
        let created = e.mem.alloc(4);
        let cinfo = e.call(0x00ca_0840, &args![this, created]).u32();
        assert_eq!(cinfo, 0);
        assert_eq!(calls_to(&e, CONVEX_COPY_CINFO), vec![vec![this, 0]]);
    }

    #[test]
    fn bhk_sphere_shape_init_builds_the_havok_sphere_from_the_cinfo() {
        let mut e = logged_engine();
        e.put_vtable(ALLOCATOR_VTABLE, &[0, ALLOC_SLOT]);
        e.map(ROUTER_OBJECT, 0x40);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, ROUTER_OBJECT + 0x20);
        e.mem.set_u32(ROUTER_OBJECT + 0x20, ALLOCATOR_VTABLE);
        e.register(MEMORY_ROUTER, |_, _| ROUTER_OBJECT.into_ret());
        e.register(ALLOC_SLOT, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(HAVOK_SPHERE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SET_HK_OBJECT_SLOT, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(POST_INIT_SLOT, |_, _| Ret::default());
        let this = shape(&mut e);
        let cinfo = e.mem.alloc(8);
        e.mem.set_u32(cinfo + 4, 0x7f80_0001);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00ca_0900, &args![this, cinfo]);
        let constructed = calls_to(&e, HAVOK_SPHERE_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        let block = constructed[0][0];
        assert_eq!(constructed[0][1], 0x7fc0_0001);
        assert_eq!(e.mem.u16(block + 4), 0x20);
        assert_eq!(calls_to(&e, ALLOC_SLOT).len(), 1);
        assert_eq!(calls_to(&e, ALLOC_SLOT)[0][1], 0x20);
        assert_eq!(calls_to(&e, SET_HK_OBJECT_SLOT), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, cinfo]]);
        assert_eq!(
            order_of(
                &e,
                &[
                    ALLOC_SLOT,
                    HAVOK_SPHERE_CONSTRUCT,
                    SET_HK_OBJECT_SLOT,
                    REMOVE_REFERENCE,
                    POST_INIT_SLOT
                ]
            ),
            vec![
                ALLOC_SLOT,
                HAVOK_SPHERE_CONSTRUCT,
                SET_HK_OBJECT_SLOT,
                REMOVE_REFERENCE,
                POST_INIT_SLOT
            ]
        );
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    /// Doubles for the display sphere: `CreateSphere` answers `sphere`, the node has the attach
    /// slot.
    fn display_engine(sphere: u32) -> (Engine, u32) {
        let mut e = logged_engine();
        e.register_double(CREATE_SPHERE, move |_, _| sphere.into_ret());
        e.register(INTERN_STRING, |_, _| 0x7ee0u32.into_ret());
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(AT_EXIT, |_, _| Ret::default());
        e.register(NODE_ATTACH_SLOT, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x38];
        slots[0xdc / 4] = NODE_ATTACH_SLOT;
        e.put_vtable(NODE_TABLE, &slots);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, NODE_TABLE);
        e.mem.set_f32(DISPLAY_SCALE, 0.5);
        e.mem.set_f32(DEFAULT_RADIUS, 3.0);
        (e, node)
    }

    #[test]
    fn bhk_sphere_shape_build_display_geometry_names_and_attaches_the_sphere() {
        let (mut e, node) = display_engine(0x6600);
        let this = e.mem.alloc(0x14);
        let havok = e.mem.alloc(0x20);
        e.mem.set_f32(havok + 0x10, 2.0);
        e.mem.set_u32(this + 8, havok);
        e.call(0x00ca_09e0, &args![this, node, 0xdead_beefu32]);
        assert_eq!(
            calls_to(&e, CREATE_SPHERE),
            vec![vec![1.0f32.to_bits(), 10, 10, 0]]
        );
        assert_eq!(calls_to(&e, INTERN_STRING), vec![vec![NAME_TEXT]]);
        assert_eq!(e.global::<u32>(NAME_HANDLE), 0x7ee0);
        assert_eq!(e.global::<u32>(NAME_GUARD) & 1, 1);
        assert_eq!(calls_to(&e, AT_EXIT), vec![vec![NAME_CLEANUP]]);
        assert_eq!(calls_to(&e, SET_NAME), vec![vec![0x6600, NAME_HANDLE]]);
        assert_eq!(calls_to(&e, NODE_ATTACH_SLOT), vec![vec![node, 0x6600, 1]]);
        // The second time the name is already interned.
        e.call(0x00ca_09e0, &args![this, node, 0u32]);
        assert_eq!(calls_to(&e, INTERN_STRING).len(), 1);
        assert_eq!(calls_to(&e, AT_EXIT).len(), 1);
        assert_eq!(calls_to(&e, SET_NAME).len(), 2);
        assert_eq!(calls_to(&e, NODE_ATTACH_SLOT).len(), 2);
    }

    #[test]
    fn bhk_sphere_shape_build_display_geometry_uses_the_default_radius_without_a_sphere() {
        let (mut e, node) = display_engine(0x6600);
        let this = e.mem.alloc(0x14);
        e.call(0x00ca_09e0, &args![this, node, 0u32]);
        e.call(0x00ca_09e0, &args![0u32, node, 0u32]);
        assert_eq!(
            calls_to(&e, CREATE_SPHERE),
            vec![vec![1.5f32.to_bits(), 10, 10, 0]; 2]
        );
    }

    #[test]
    fn bhk_sphere_shape_build_display_geometry_does_nothing_more_without_a_display_sphere() {
        let (mut e, node) = display_engine(0);
        let this = e.mem.alloc(0x14);
        e.call(0x00ca_09e0, &args![this, node, 0u32]);
        assert_eq!(calls_to(&e, CREATE_SPHERE).len(), 1);
        assert!(calls_to(&e, INTERN_STRING).is_empty());
        assert!(calls_to(&e, SET_NAME).is_empty());
        assert!(calls_to(&e, NODE_ATTACH_SLOT).is_empty());
        assert_eq!(e.global::<u32>(NAME_GUARD) & 1, 0);
    }

    #[test]
    fn bhk_sphere_shape_get_viewer_strings_stores_the_class_name() {
        let mut e = logged_engine();
        e.register(CONVEX_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            Ret::default()
        });
        e.mem.set_u32(CLASS_RTTI, 0x4242);
        let this = e.mem.alloc(0x14);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00ca_0ac0, &args![this, array]);
        assert_eq!(
            calls_to(&e, CONVEX_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4242]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}
