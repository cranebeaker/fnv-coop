//! `fallout/misc/garbagecollector.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `GarbageCollector` (Xbox PDB) defers the destruction of objects that other
//! threads or the current frame may still be using: references, 3D objects,
//! biped and plain animations, temp effects and navmeshes are queued in
//! arrays and destroyed later by [`garbage_collector_update`], or destroyed
//! at once when the model loader's lock can be taken.
//!
//! The Xbox build has one array per kind (`ObjectGarbageA`, `Actor3DGarbageA`,
//! `BipedGarbageA`, `AnimGarbageA`, `EffectGarbageA`, `NavmeshGarbageA`). The
//! PC build has two for every kind but animations, told apart here by the
//! thread flag (a byte at +0x298 of the thread's TLS block) that decides which
//! one an `Add` queues into: `*_FLAG_SET` when the flag is set,
//! `*_FLAG_CLEAR` when it is clear. Which of the two is the Xbox name's array
//! is not known.
//!
//! Not translated: the compiler's exception-unwinding frames (`FS:[0]`
//! chains and state variables) of the `Add` overloads, `Update` and the array
//! helpers. Local objects the game keeps on its stack (the guard of
//! [`fn_008681c0`], the `NiPointer` temporaries, the pointer whose address is
//! handed to the arrays' `Add`) are 8- and 4-byte heap blocks here, freed
//! where the game's destructors or scope ends run.

#[allow(unused_imports)]
use crate::prelude::*;

/// `GarbageCollector::CritSec` (Xbox PDB), a `BSSpinLock`.
const CRIT_SEC: u32 = 0x011d_e8e0;
/// Bit set of the kinds currently being added (`1` references, `2` 3D
/// objects, `4` biped animations, `8` animations, `0x10` temp effects,
/// `0x20` navmeshes); see [`fn_008681c0`].
const ACTIVE_KINDS: u32 = 0x011d_e804;
/// Set while [`garbage_collector_clear_all`] runs.
const CLEARING_ALL: u32 = 0x011d_e958;
/// Pointer to the `ModelLoader` (the `this` of `ModelLoader::CancelReference`,
/// `00445570`, in [`garbage_collector_add`]).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// A pointer to an object whose flag word at +0x244 bit 1 `0042ce10` tests
/// (what that object is, is not known).
const FLAGGED_OBJECT: u32 = 0x011d_df38;
/// Offset of the thread flag in the TLS block.
const TLS_THREAD_FLAG: u32 = 0x298;

// The garbage arrays (`NiTObjectArray` / `BSSimpleArray`), see the module doc.
const OBJECT_GARBAGE_FLAG_SET: u32 = 0x011d_e874;
const OBJECT_GARBAGE_FLAG_CLEAR: u32 = 0x011d_e8bc;
const ACTOR_3D_GARBAGE_FLAG_SET: u32 = 0x011d_e888;
const ACTOR_3D_GARBAGE_FLAG_CLEAR: u32 = 0x011d_e848;
const BIPED_GARBAGE_FLAG_SET: u32 = 0x011d_e910;
const BIPED_GARBAGE_FLAG_CLEAR: u32 = 0x011d_e838;
/// `AnimGarbageA` (Xbox PDB): the only animation array.
const ANIM_GARBAGE: u32 = 0x011d_e808;
const EFFECT_GARBAGE_FLAG_SET: u32 = 0x011d_e828;
const EFFECT_GARBAGE_FLAG_CLEAR: u32 = 0x011d_e858;
const NAVMESH_GARBAGE_FLAG_SET: u32 = 0x011d_e924;
const NAVMESH_GARBAGE_FLAG_CLEAR: u32 = 0x011d_e898;

// Kind bits of [`ACTIVE_KINDS`].
const KIND_OBJECT: u32 = 1;
const KIND_ACTOR_3D: u32 = 2;
const KIND_BIPED: u32 = 4;
const KIND_ANIM: u32 = 8;
const KIND_EFFECT: u32 = 0x10;
const KIND_NAVMESH: u32 = 0x20;

// Scope names handed to the lock (`GarbageCollector::Add(...)` strings).
const NAME_ADD_OBJECT: u32 = 0x0108_267c;
const NAME_REMOVE_OBJECT: u32 = 0x0108_26a4;
const NAME_ADD_ANIM: u32 = 0x0108_26cc;
const NAME_ADD_EFFECT: u32 = 0x0108_26f0;
const NAME_ADD_BIPED: u32 = 0x0108_2714;
const NAME_ADD_ACTOR_3D: u32 = 0x0108_2738;
const NAME_ADD_NAVMESH: u32 = 0x0108_275c;
const NAME_UPDATE: u32 = 0x0108_2780;
const NAME_CLEAR_TEMP_EFFECTS: u32 = 0x0108_279c;
const NAME_CLEAR_ALL: u32 = 0x0108_27c4;

// Array primitives (other units, called by address).
/// `m_usSize` of an `NiTArray`.
const ARRAY_SIZE: u32 = 0x0065_8930;
/// Address of element `index` of an `NiTArray` or `BSSimpleArray`.
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `iSize` of a `BSSimpleArray`.
const SIMPLE_ARRAY_SIZE: u32 = 0x0044_ddc0;
/// Address of element `index` of a `BSSimpleArray` (calls `ARRAY_ELEMENT`).
const SIMPLE_ARRAY_ELEMENT: u32 = 0x006a_7ad0;
/// Appends a pointer (passed by address) to an `NiTArray` of raw pointers.
const ARRAY_ADD_POINTER: u32 = 0x0086_93c0;
/// Appends an `NiPointer<BSTempEffect>` (passed by address).
const ARRAY_ADD_EFFECT: u32 = 0x0086_93f0;
/// Appends an `NiPointer<NiAVObject>` (passed by address).
const ARRAY_ADD_ACTOR_3D: u32 = 0x004a_fc50;
/// Appends a navmesh pointer (passed by address) to a `BSSimpleArray`.
const SIMPLE_ARRAY_ADD: u32 = 0x007c_b2e0;
/// Empties an `NiTArray` of raw pointers (sets its sizes to zero).
const ARRAY_CLEAR: u32 = 0x005e_03d0;
/// Removes element `index` of an `NiTArray` of raw pointers by moving the
/// last one into its place (`fn_008691d0` is the PC copy for references).
const ARRAY_REMOVE_AT: u32 = 0x009e_98d0;
/// Empties a `BSSimpleArray` (`this`, `bFreeMemory`).
const SIMPLE_ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `BSSimpleArray<...>::Remove(index, count)`.
const SIMPLE_ARRAY_REMOVE: u32 = 0x006b_f8f0;

// `NiPointer` operations (other units).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const NI_POINTER_COPY_CONSTRUCT: u32 = 0x0055_9a40;
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer::operator!=` (compares the two pointers).
const NI_POINTER_NOT_EQUAL: u32 = 0x0063_1820;

/// `InterlockedExchange` (kernel32 import; the exe calls it through the
/// import table slot at this address).
const INTERLOCKED_EXCHANGE: u32 = 0x00fd_f0e4;

layout! {
    /// The 8-byte scope guard the `Add` overloads keep on their stack (no
    /// Xbox PDB type; the name is made up here): [`fn_008681c0`] builds it,
    /// [`fn_00868210`] undoes it.
    pub struct GarbageCollectorGuard: 8 {
        /// The kind bit this scope adds to [`ACTIVE_KINDS`].
        0x00 mask: u32,
        /// Whether that bit was already set when the scope began.
        0x04 bWasSet: bool,
    }

}

pub use crate::types::NiTArray;

/// Address of the thread flag byte in the current thread's TLS block.
fn thread_flag_address(e: &mut Engine) -> u32 {
    e.tls() + TLS_THREAD_FLAG
}

fn thread_flag(e: &mut Engine) -> bool {
    let address = thread_flag_address(e);
    e.mem.u8(address) != 0
}

/// Whether an `Add` overload may destroy its object on the spot: the thread
/// flag is clear, or it is set and the object at [`FLAGGED_OBJECT`] exists
/// and `0042ce10` accepts it.
fn may_destroy_in_place(e: &mut Engine) -> bool {
    if !thread_flag(e) {
        return true;
    }
    let owner: u32 = e.global(FLAGGED_OBJECT);
    owner != 0 && e.call(0x0042_ce10, &args![owner]).bool()
}

fn array_size(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

fn simple_array_size(e: &mut Engine, array: u32) -> u32 {
    e.call(SIMPLE_ARRAY_SIZE, &args![array]).u32()
}

/// The pointer stored in element `index` of an array of raw pointers.
fn array_pointer_at(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
    e.mem.u32(slot)
}

/// Appends `value` to an array of raw pointers with `add` (the arrays take
/// the address of a pointer variable).
fn add_pointer(e: &mut Engine, array: u32, add: u32, value: u32) {
    let variable = e.mem.alloc(4);
    e.mem.set_u32(variable, value);
    e.call(add, &args![array, variable]);
    e.mem.free(variable);
}

/// Wraps `object` in a temporary `NiPointer`, appends it to `array` with
/// `add`, and destroys the temporary.
fn add_ni_pointer(e: &mut Engine, array: u32, add: u32, object: u32) {
    let pointer = e.mem.alloc(4);
    e.call(NI_POINTER_CONSTRUCT, &args![pointer, object]);
    e.call(add, &args![array, pointer]);
    e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    e.mem.free(pointer);
}

fn loader(e: &Engine) -> u32 {
    e.global(MODEL_LOADER)
}

// Translated from 00418e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `NiTArray<BipedAnim *,NiTNewInterface<BipedAnim *>>::SetSize`
/// (Xbox PDB), a name the linker's folding of identical code gave it; the
/// code is a scalar deleting destructor: runs the destructor `004aae50`, and
/// frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00418e00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004a_ae50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004b0220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<BSTempEffect>::_vector_deleting_destructor_` (Xbox PDB, a name
/// from identical-code folding): 0 for a null pointer, otherwise the result
/// of `00667120(pointer, 3)`.
pub fn fn_004b0220(e: &mut Engine, pointer: Ptr) -> u32 {
    if pointer.is_null() {
        return 0;
    }
    e.call(0x0066_7120, &args![pointer, 3u32]).u32()
}

// Translated from 004dffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<BSTempEffect>,NiTNewInterface<NiPointer<BSTempEffect>>>::RemoveAll`
/// (Xbox PDB): assigns a null `NiPointer` to every slot (releasing what it
/// held), then sets the size and the element count to zero.
pub fn ni_t_array_ni_pointer_bs_temp_effect_remove_all(e: &mut Engine, this: Ptr<NiTArray>) {
    let null_pointer = e.mem.alloc(4);
    let mut index: u16 = 0;
    while (index as u32) < e.get(this, NiTArray::m_usSize) as u32 {
        e.call(NI_POINTER_CONSTRUCT, &args![null_pointer, 0u32]);
        let base = e.get(this, NiTArray::m_pBase);
        e.call(
            NI_POINTER_ASSIGN,
            &args![base.wrapping_add(index as u32 * 4), null_pointer],
        );
        e.call(NI_POINTER_DESTRUCT, &args![null_pointer]);
        index = index.wrapping_add(1);
    }
    e.mem.free(null_pointer);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
}

// Translated from 00867f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the garbage collector's lock ([`CRIT_SEC`]) under the name `name`
/// (`0040fbf0`). The game's callers also push a second argument, which this
/// function does not read.
pub fn fn_00867f50(e: &mut Engine, name: u32) {
    e.call(0x0040_fbf0, &args![CRIT_SEC, name]);
}

// Translated from 00867f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries to take the garbage collector's lock (`0078d200`); whether it did.
pub fn fn_00867f70(e: &mut Engine) -> bool {
    e.call(0x0078_d200, &args![CRIT_SEC]).bool()
}

// Translated from 00867f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the garbage collector's lock (`0040fba0`).
pub fn fn_00867f80(e: &mut Engine) {
    e.call(0x0040_fba0, &args![CRIT_SEC]);
}

// Translated from 00867f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::Add` (Xbox PDB), the `TESObjectREFR` overload: takes the
/// reference out of the world (cancels its model loading, removes it from
/// the form data structures and the terrain tree), then either destroys it
/// at once (when the model loader's lock can be taken) or queues it.
///
/// If the reference's form lookup (`0084e3a0`, `004839c0`) does not come
/// back to the reference itself, it is only destroyed.
pub fn garbage_collector_add(e: &mut Engine, object: Ptr) {
    let guard = e.new_object::<GarbageCollectorGuard>();
    fn_008681c0(e, guard, KIND_OBJECT, NAME_ADD_OBJECT);

    let form = e.call(0x0084_e3a0, &args![object]).u32();
    let found = e.call(0x0048_39c0, &args![form]).ptr::<()>();
    if found != object {
        if !object.is_null() {
            e.vcall(object.addr(), 0x10, &args![1u32]);
        }
    } else {
        let this = object.addr();
        e.vcall(this, 0xc4, &args![1u32]);
        e.call(0x0056_fcb0, &args![this]);
        if e.vcall(this, 0x100, &args![]).bool() {
            e.vcall(this, 0x434, &args![0u32]);
        }
        if e.vcall(this, 0x224, &args![]).bool() {
            e.call(0x0042_0ba0, &args![this, 0u32]);
            e.call(0x005d_e0a0, &args![this, 0x400u32, 0u32]);
        }
        let model_loader = loader(e);
        // ModelLoader::CancelReference (Xbox PDB)
        e.call(0x0044_5570, &args![model_loader, this]);
        // TESForm::RemoveFromDataStructures (Xbox PDB)
        e.call(0x0048_3c70, &args![this]);
        // BGSSaveFormBuffer::GetForm (Xbox PDB)
        let owner = e.call(0x007a_f430, &args![this]).u32();
        if e.call(0x0054_9580, &args![owner]).bool() {
            // TESObjectREFR::GetWorldSpace (Xbox PDB)
            let world_space = e.call(0x0057_5d70, &args![this]).u32();
            if world_space != 0 {
                // TESWorldSpace::GetTerrainManager (Xbox PDB)
                let terrain = e.call(0x0058_6170, &args![world_space]).u32();
                if terrain != 0 {
                    let terrain = e.call(0x0058_6170, &args![world_space]).u32();
                    // BGSTerrainManager::ReleaseTree (Xbox PDB)
                    e.call(0x006f_cf30, &args![terrain, this]);
                }
            }
        }
        e.vcall(this, 0x228, &args![0u32]);

        if may_destroy_in_place(e) {
            let model_loader = loader(e);
            if fn_00868250(e, Ptr::new(model_loader)) {
                if !object.is_null() {
                    e.vcall(this, 0x10, &args![1u32]);
                }
                e.call(0x004a_af10, &args![model_loader]);
            } else {
                add_pointer(e, OBJECT_GARBAGE_FLAG_CLEAR, ARRAY_ADD_POINTER, this);
            }
        } else {
            add_pointer(e, OBJECT_GARBAGE_FLAG_SET, ARRAY_ADD_POINTER, this);
        }
    }

    fn_00868210(e, guard);
    e.mem.free(guard.addr());
}

// Translated from 008681c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts an `Add` scope: records `mask`, takes the lock under `name`
/// ([`fn_00867f50`]), remembers whether the kind bit was already set in
/// [`ACTIVE_KINDS`], and sets it. Returns the guard.
pub fn fn_008681c0(
    e: &mut Engine,
    this: Ptr<GarbageCollectorGuard>,
    mask: u32,
    name: u32,
) -> Ptr<GarbageCollectorGuard> {
    e.set(this, GarbageCollectorGuard::mask, mask);
    fn_00867f50(e, name);
    let active: u32 = e.global(ACTIVE_KINDS);
    e.set(this, GarbageCollectorGuard::bWasSet, active & mask != 0);
    let active: u32 = e.global(ACTIVE_KINDS);
    e.set_global(ACTIVE_KINDS, active | mask);
    this
}

// Translated from 00868210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends an `Add` scope: clears the kind bit again unless it was already set
/// when the scope began, and releases the lock.
pub fn fn_00868210(e: &mut Engine, this: Ptr<GarbageCollectorGuard>) {
    if !e.get(this, GarbageCollectorGuard::bWasSet) {
        let mask = e.get(this, GarbageCollectorGuard::mask);
        let active: u32 = e.global(ACTIVE_KINDS);
        e.set_global(ACTIVE_KINDS, !mask & active);
    }
    fn_00867f80(e);
}

// Translated from 00868250 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `this` the model loader: tries to take the lock of the object its
/// first field points to ([`fn_008691b0`]).
pub fn fn_00868250(e: &mut Engine, this: Ptr) -> bool {
    let inner = e.mem.u32(this.addr());
    fn_008691b0(e, Ptr::new(inner))
}

// Translated from 00868270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::Remove` (Xbox PDB), the `TESObjectREFR` overload:
/// under the lock, removes `object` from the flag-set array and from the
/// flag-clear array (at most once from each).
pub fn garbage_collector_remove(e: &mut Engine, object: u32) {
    fn_00867f50(e, NAME_REMOVE_OBJECT);
    for array in [OBJECT_GARBAGE_FLAG_SET, OBJECT_GARBAGE_FLAG_CLEAR] {
        let count = array_size(e, array);
        for index in 0..count {
            if array_pointer_at(e, array, index) == object {
                fn_008691d0(e, Ptr::new(array), index);
                break;
            }
        }
    }
    fn_00867f80(e);
}

// Translated from 00868330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::Add` (Xbox PDB), the `Animation` overload: shuts down
/// the animation's idles and controllers, cancels its replacement KF list,
/// then destroys it at once or queues it in [`ANIM_GARBAGE`].
pub fn garbage_collector_add_ov2(e: &mut Engine, animation: Ptr) {
    let guard = e.new_object::<GarbageCollectorGuard>();
    fn_008681c0(e, guard, KIND_ANIM, NAME_ADD_ANIM);

    let animation_address = animation.addr();
    // Animation::ShutdownAllAnimIdles (Xbox PDB)
    e.call(0x0048_ff50, &args![animation_address]);
    let manager = e.call(0x0049_6940, &args![animation_address]).u32();
    if manager != 0 {
        let manager = e.call(0x0049_6940, &args![animation_address]).u32();
        // NiControllerManager::DeactivateAll (Xbox PDB)
        e.call(0x0048_fef0, &args![manager, 0.0f32]);
    }
    let node = e.call(0x0055_85e0, &args![animation_address]).u32();
    if node != 0 {
        for controller_type in [0x011f_36bc_u32, 0x011f_36e4] {
            // NiObjectNET::GetController (Xbox PDB)
            let controller = e.call(0x00a5_c570, &args![node, controller_type]).u32();
            if controller != 0 {
                // NiObjectNET::RemoveController (Xbox PDB)
                e.call(0x00a5_c480, &args![node, controller]);
            }
        }
    }
    let model_loader = loader(e);
    // ModelLoader::CancelReplacementKFList (Xbox PDB)
    e.call(0x0044_5430, &args![model_loader, animation_address]);

    if may_destroy_in_place(e) {
        if !animation.is_null() {
            e.call(0x0041_8d20, &args![animation_address, 1u32]);
        }
    } else {
        add_pointer(e, ANIM_GARBAGE, ARRAY_ADD_POINTER, animation_address);
    }

    fn_00868210(e, guard);
    e.mem.free(guard.addr());
}

// Translated from 00868490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::Add` (Xbox PDB), the `BSTempEffect` overload: queues
/// the effect (as an `NiPointer`) in the flag-set or flag-clear effect array.
pub fn garbage_collector_add_ov3(e: &mut Engine, effect: Ptr) {
    let guard = e.new_object::<GarbageCollectorGuard>();
    fn_008681c0(e, guard, KIND_EFFECT, NAME_ADD_EFFECT);

    let array = if thread_flag(e) {
        EFFECT_GARBAGE_FLAG_SET
    } else {
        EFFECT_GARBAGE_FLAG_CLEAR
    };
    add_ni_pointer(e, array, ARRAY_ADD_EFFECT, effect.addr());

    fn_00868210(e, guard);
    e.mem.free(guard.addr());
}

// Translated from 00868560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::Add` (Xbox PDB), the `BipedAnim` overload: destroys the
/// biped animation at once when the model loader's lock can be taken,
/// otherwise queues it in the flag-clear array; with the thread flag set and
/// nothing allowing an in-place destruction it goes to the flag-set array.
pub fn garbage_collector_add_ov4(e: &mut Engine, biped: Ptr) {
    let guard = e.new_object::<GarbageCollectorGuard>();
    fn_008681c0(e, guard, KIND_BIPED, NAME_ADD_BIPED);

    if may_destroy_in_place(e) {
        let model_loader = loader(e);
        if fn_00868250(e, Ptr::new(model_loader)) {
            if !biped.is_null() {
                fn_00418e00(e, biped, 1);
            }
            e.call(0x004a_af10, &args![model_loader]);
        } else {
            add_pointer(e, BIPED_GARBAGE_FLAG_CLEAR, ARRAY_ADD_POINTER, biped.addr());
        }
    } else {
        add_pointer(e, BIPED_GARBAGE_FLAG_SET, ARRAY_ADD_POINTER, biped.addr());
    }

    fn_00868210(e, guard);
    e.mem.free(guard.addr());
}

// Translated from 00868660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::Add` (Xbox PDB), the `NiAVObject` overload: marks a
/// non-null object ([`fn_00868740`]) and queues it (as an `NiPointer`) in the
/// flag-set or flag-clear 3D array.
pub fn garbage_collector_add_ov5(e: &mut Engine, object: Ptr) {
    let guard = e.new_object::<GarbageCollectorGuard>();
    fn_008681c0(e, guard, KIND_ACTOR_3D, NAME_ADD_ACTOR_3D);

    if !object.is_null() {
        fn_00868740(e, object);
    }
    let array = if thread_flag(e) {
        ACTOR_3D_GARBAGE_FLAG_SET
    } else {
        ACTOR_3D_GARBAGE_FLAG_CLEAR
    };
    add_ni_pointer(e, array, ARRAY_ADD_ACTOR_3D, object.addr());

    fn_00868210(e, guard);
    e.mem.free(guard.addr());
}

// Translated from 00868740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit 30 (`0x40000000`) of the flag word at +4 of `this` (an
/// `NiAVObject`'s flags) with an interlocked exchange ([`fn_00868770`]).
pub fn fn_00868740(e: &mut Engine, this: Ptr) {
    let flags = e.mem.u32(this.addr() + 4);
    fn_00868770(e, this.addr() + 4, flags | 0x4000_0000);
}

// Translated from 00868770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterlockedExchange(target, value)`, called through the import table
/// slot [`INTERLOCKED_EXCHANGE`] (the old value is discarded).
pub fn fn_00868770(e: &mut Engine, target: u32, value: u32) {
    e.call(INTERLOCKED_EXCHANGE, &args![target, value]);
}

// Translated from 00868790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::Add` (Xbox PDB), the `NavMeshPtr` overload: calls
/// `0040f6e0` on the navmesh (+0x1c of the pointer's target) and appends the
/// navmesh pointer `00458b50` returns to the flag-set or flag-clear navmesh
/// array.
pub fn garbage_collector_add_ov6(e: &mut Engine, navmesh_pointer: Ptr) {
    let guard = e.new_object::<GarbageCollectorGuard>();
    fn_008681c0(e, guard, KIND_NAVMESH, NAME_ADD_NAVMESH);

    let target = e.call(NI_POINTER_GET, &args![navmesh_pointer]).u32();
    e.call(0x0040_f6e0, &args![target + 0x1c]);
    let array = if thread_flag(e) {
        NAVMESH_GARBAGE_FLAG_SET
    } else {
        NAVMESH_GARBAGE_FLAG_CLEAR
    };
    let navmesh = e.call(0x0045_8b50, &args![navmesh_pointer]).u32();
    add_pointer(e, array, SIMPLE_ARRAY_ADD, navmesh);

    fn_00868210(e, guard);
    e.mem.free(guard.addr());
}

// Translated from 00868850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::Update` (Xbox PDB): once a frame, under the lock,
/// does one piece of work, the first that applies of: destroy queued
/// animations, biped animations, release 3D objects, destroy queued
/// references, release navmeshes; or move a flag-clear array into its
/// flag-set twin. A batch is limited to a count (10, 20 or 5 times 1, or
/// times 2 when `00878360` says the frame is slow); destroying needs the
/// model loader's lock, which is released afterwards.
pub fn garbage_collector_update(e: &mut Engine) {
    fn_00867f50(e, NAME_UPDATE);
    let mut multiplier: u32 = 1;
    let singleton = e.call(0x0044_6ef0, &args![]).u32();
    if e.call(0x0087_8360, &args![singleton]).bool() {
        multiplier = 2;
    }

    if array_size(e, ANIM_GARBAGE) != 0 {
        if fn_00868250(e, Ptr::new(loader(e))) {
            let mut remaining = array_size(e, ANIM_GARBAGE);
            let mut budget = multiplier * 10;
            while remaining != 0 && budget != 0 {
                remaining -= 1;
                budget -= 1;
                let animation = array_pointer_at(e, ANIM_GARBAGE, remaining);
                e.call(ARRAY_REMOVE_AT, &args![ANIM_GARBAGE, remaining]);
                if animation != 0 {
                    e.call(0x0041_8d20, &args![animation, 1u32]);
                }
            }
            let model_loader = loader(e);
            e.call(0x004a_af10, &args![model_loader]);
        }
    } else if array_size(e, BIPED_GARBAGE_FLAG_SET) != 0 {
        if fn_00868250(e, Ptr::new(loader(e))) {
            let mut remaining = array_size(e, BIPED_GARBAGE_FLAG_SET);
            let mut budget = multiplier * 10;
            while remaining != 0 && budget != 0 {
                remaining -= 1;
                budget -= 1;
                let biped = array_pointer_at(e, BIPED_GARBAGE_FLAG_SET, remaining);
                e.call(ARRAY_REMOVE_AT, &args![BIPED_GARBAGE_FLAG_SET, remaining]);
                if biped != 0 {
                    fn_00418e00(e, Ptr::new(biped), 1);
                }
            }
            let model_loader = loader(e);
            e.call(0x004a_af10, &args![model_loader]);
        }
    } else if array_size(e, BIPED_GARBAGE_FLAG_CLEAR) != 0 {
        fn_008694c0(e, BIPED_GARBAGE_FLAG_CLEAR, BIPED_GARBAGE_FLAG_SET);
        e.call(ARRAY_CLEAR, &args![BIPED_GARBAGE_FLAG_CLEAR]);
    } else if array_size(e, ACTOR_3D_GARBAGE_FLAG_SET) != 0 {
        let mut remaining = array_size(e, ACTOR_3D_GARBAGE_FLAG_SET);
        let mut budget = multiplier * 10;
        let released = e.mem.alloc(4);
        while remaining != 0 && budget != 0 {
            remaining -= 1;
            budget -= 1;
            ni_t_array_ni_pointer_ni_av_object_remove_at(
                e,
                Ptr::new(ACTOR_3D_GARBAGE_FLAG_SET),
                Ptr::new(released),
                remaining,
            );
            if e.call(0x0052_aa80, &args![released, 0u32]).bool() {
                let object = e.mem.u32(released);
                fn_00868ce0(e, Ptr::new(object));
            }
            e.call(NI_POINTER_DESTRUCT, &args![released]);
        }
        e.mem.free(released);
    } else if array_size(e, ACTOR_3D_GARBAGE_FLAG_CLEAR) != 0 {
        fn_00869420(e, ACTOR_3D_GARBAGE_FLAG_CLEAR, ACTOR_3D_GARBAGE_FLAG_SET);
        ni_t_array_ni_pointer_bs_temp_effect_remove_all(e, Ptr::new(ACTOR_3D_GARBAGE_FLAG_CLEAR));
    } else if array_size(e, OBJECT_GARBAGE_FLAG_SET) != 0 {
        if fn_00868250(e, Ptr::new(loader(e))) {
            let mut remaining = array_size(e, OBJECT_GARBAGE_FLAG_SET);
            let mut budget = multiplier * 0x14;
            let mut references_left: i32 = 1;
            while remaining != 0 && budget != 0 && references_left != 0 {
                remaining -= 1;
                budget -= 1;
                // The destructor removes the reference from the array again
                // (`garbage_collector_remove`), so element 0 is a new one
                // every time.
                let reference = array_pointer_at(e, OBJECT_GARBAGE_FLAG_SET, 0);
                if e.vcall(reference, 0x100, &args![]).bool()
                    || e.call(0x0056_4d80, &args![reference]).bool()
                {
                    references_left -= 1;
                }
                if reference != 0 {
                    e.vcall(reference, 0x10, &args![1u32]);
                }
            }
            let model_loader = loader(e);
            e.call(0x004a_af10, &args![model_loader]);
        }
    } else if array_size(e, OBJECT_GARBAGE_FLAG_CLEAR) != 0 {
        fn_008694c0(e, OBJECT_GARBAGE_FLAG_CLEAR, OBJECT_GARBAGE_FLAG_SET);
        e.call(ARRAY_CLEAR, &args![OBJECT_GARBAGE_FLAG_CLEAR]);
    } else if simple_array_size(e, NAVMESH_GARBAGE_FLAG_SET) != 0 {
        let mut remaining = simple_array_size(e, NAVMESH_GARBAGE_FLAG_SET);
        let mut budget = multiplier * 5;
        while remaining != 0 && budget != 0 {
            remaining -= 1;
            budget -= 1;
            let navmesh = array_pointer_at(e, NAVMESH_GARBAGE_FLAG_SET, 0);
            e.call(0x0040_1970, &args![navmesh + 0x1c]);
            e.call(
                SIMPLE_ARRAY_REMOVE,
                &args![NAVMESH_GARBAGE_FLAG_SET, 0u32, 1u32],
            );
        }
    } else if simple_array_size(e, NAVMESH_GARBAGE_FLAG_CLEAR) != 0 {
        fn_00869510(e, NAVMESH_GARBAGE_FLAG_CLEAR, NAVMESH_GARBAGE_FLAG_SET);
        e.call(SIMPLE_ARRAY_CLEAR, &args![NAVMESH_GARBAGE_FLAG_CLEAR, 1u32]);
    }

    fn_00867f80(e);
}

// Translated from 00868ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears bit 30 (`0x40000000`) of the flag word at +4 of `this` (the mark
/// [`fn_00868740`] sets) with an interlocked exchange.
pub fn fn_00868ce0(e: &mut Engine, this: Ptr) {
    let flags = e.mem.u32(this.addr() + 4);
    fn_00868770(e, this.addr() + 4, flags & 0xbfff_ffff);
}

// Translated from 00868d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::ClearTempEffects` (Xbox PDB): when the flag-set effect
/// array has entries and the model loader's lock can be taken, empties it
/// under the lock and releases the model loader's lock.
pub fn garbage_collector_clear_temp_effects(e: &mut Engine) {
    if array_size(e, EFFECT_GARBAGE_FLAG_SET) != 0 && fn_00868250(e, Ptr::new(loader(e))) {
        fn_00867f50(e, NAME_CLEAR_TEMP_EFFECTS);
        ni_t_array_ni_pointer_bs_temp_effect_remove_all(e, Ptr::new(EFFECT_GARBAGE_FLAG_SET));
        let model_loader = loader(e);
        e.call(0x004a_af10, &args![model_loader]);
        fn_00867f80(e);
    }
}

// Translated from 00868d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GarbageCollector::ClearAll` (Xbox PDB): destroys everything queued,
/// for every kind that is not being added right now ([`fn_00869180`]). Not
/// re-entrant ([`CLEARING_ALL`]). With `try_lock` set the lock is only
/// tried (and nothing happens when it is busy), otherwise it is taken.
/// Runs with the thread flag cleared and restores it afterwards; when it was
/// set, the flag-clear arrays are then moved into the flag-set ones.
pub fn garbage_collector_clear_all(e: &mut Engine, try_lock: u8) {
    if e.global::<u8>(CLEARING_ALL) != 0 {
        return;
    }
    let mut locked = true;
    if try_lock != 0 {
        locked = fn_00867f70(e);
    } else {
        fn_00867f50(e, NAME_CLEAR_ALL);
    }
    if !locked {
        return;
    }
    e.set_global(CLEARING_ALL, 1u8);
    let flag_address = thread_flag_address(e);
    let saved_flag = e.mem.u8(flag_address);
    e.mem.set_u8(flag_address, 0);

    if !fn_00869180(e, KIND_EFFECT) && e.call(0x0078_d1f0, &args![]).bool() {
        ni_t_array_ni_pointer_bs_temp_effect_remove_all(e, Ptr::new(EFFECT_GARBAGE_FLAG_SET));
        e.call(0x0045_2530, &args![]);
    }

    if !fn_00869180(e, KIND_ANIM) && fn_00868250(e, Ptr::new(loader(e))) {
        let count = array_size(e, ANIM_GARBAGE);
        for index in 0..count {
            let animation = array_pointer_at(e, ANIM_GARBAGE, index);
            if animation != 0 {
                e.call(0x0041_8d20, &args![animation, 1u32]);
            }
        }
        e.call(ARRAY_CLEAR, &args![ANIM_GARBAGE]);
        let model_loader = loader(e);
        e.call(0x004a_af10, &args![model_loader]);
    }

    if !fn_00869180(e, KIND_BIPED) && fn_00868250(e, Ptr::new(loader(e))) {
        let count = array_size(e, BIPED_GARBAGE_FLAG_SET);
        for index in 0..count {
            let biped = array_pointer_at(e, BIPED_GARBAGE_FLAG_SET, index);
            if biped != 0 {
                fn_00418e00(e, Ptr::new(biped), 1);
            }
        }
        e.call(ARRAY_CLEAR, &args![BIPED_GARBAGE_FLAG_SET]);
        let model_loader = loader(e);
        e.call(0x004a_af10, &args![model_loader]);
    }

    if !fn_00869180(e, KIND_ACTOR_3D) {
        let mut index = 0;
        while index < array_size(e, ACTOR_3D_GARBAGE_FLAG_SET) {
            let slot = e
                .call(ARRAY_ELEMENT, &args![ACTOR_3D_GARBAGE_FLAG_SET, index])
                .u32();
            if e.call(NI_POINTER_GET, &args![slot]).u32() != 0 {
                let object = array_pointer_at(e, ACTOR_3D_GARBAGE_FLAG_SET, index);
                fn_00868ce0(e, Ptr::new(object));
            }
            index += 1;
        }
        ni_t_array_ni_pointer_bs_temp_effect_remove_all(e, Ptr::new(ACTOR_3D_GARBAGE_FLAG_SET));
    }

    if !fn_00869180(e, KIND_OBJECT) && fn_00868250(e, Ptr::new(loader(e))) {
        // Each destructor removes its reference from the array again.
        while array_size(e, OBJECT_GARBAGE_FLAG_SET) != 0 {
            let reference = array_pointer_at(e, OBJECT_GARBAGE_FLAG_SET, 0);
            if reference != 0 {
                e.vcall(reference, 0x10, &args![1u32]);
            }
        }
        e.call(ARRAY_CLEAR, &args![OBJECT_GARBAGE_FLAG_SET]);
        let model_loader = loader(e);
        e.call(0x004a_af10, &args![model_loader]);
    }

    if !fn_00869180(e, KIND_NAVMESH) {
        let mut index = 0;
        while index < simple_array_size(e, NAVMESH_GARBAGE_FLAG_SET) {
            let navmesh = array_pointer_at(e, NAVMESH_GARBAGE_FLAG_SET, index);
            e.call(0x0040_1970, &args![navmesh + 0x1c]);
            index += 1;
        }
        e.call(SIMPLE_ARRAY_CLEAR, &args![NAVMESH_GARBAGE_FLAG_SET, 1u32]);
    }

    e.mem.set_u8(flag_address, saved_flag);
    if e.mem.u8(flag_address) != 0 {
        fn_008694c0(e, OBJECT_GARBAGE_FLAG_CLEAR, OBJECT_GARBAGE_FLAG_SET);
        e.call(ARRAY_CLEAR, &args![OBJECT_GARBAGE_FLAG_CLEAR]);
        fn_00869420(e, ACTOR_3D_GARBAGE_FLAG_CLEAR, ACTOR_3D_GARBAGE_FLAG_SET);
        ni_t_array_ni_pointer_bs_temp_effect_remove_all(e, Ptr::new(ACTOR_3D_GARBAGE_FLAG_CLEAR));
        fn_008694c0(e, BIPED_GARBAGE_FLAG_CLEAR, BIPED_GARBAGE_FLAG_SET);
        e.call(ARRAY_CLEAR, &args![BIPED_GARBAGE_FLAG_CLEAR]);
        fn_00869560(e, EFFECT_GARBAGE_FLAG_CLEAR, EFFECT_GARBAGE_FLAG_SET);
        ni_t_array_ni_pointer_bs_temp_effect_remove_all(e, Ptr::new(EFFECT_GARBAGE_FLAG_CLEAR));
        fn_00869510(e, NAVMESH_GARBAGE_FLAG_CLEAR, NAVMESH_GARBAGE_FLAG_SET);
        e.call(SIMPLE_ARRAY_CLEAR, &args![NAVMESH_GARBAGE_FLAG_CLEAR, 1u32]);
    }
    e.set_global(CLEARING_ALL, 0u8);
    fn_00867f80(e);
}

// Translated from 00869180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any of the kind bits in `mask` is set in [`ACTIVE_KINDS`]
/// (that kind is being added right now).
pub fn fn_00869180(e: &mut Engine, mask: u32) -> bool {
    e.global::<u32>(ACTIVE_KINDS) & mask != 0
}

// Translated from 00869190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the thread flag in the current thread's TLS block.
pub fn fn_00869190(e: &mut Engine, value: u8) {
    let address = thread_flag_address(e);
    e.mem.set_u8(address, value);
}

// Translated from 008691b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries to take the lock at +0x20 of `this` (`0078d200`); whether it did.
pub fn fn_008691b0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0078_d200, &args![this.addr() + 0x20]).bool()
}

// Translated from 008691d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes element `index` of an `NiTArray` of raw pointers by moving the
/// last element into its place, and returns the removed pointer (0 for an
/// index past the end). The element count drops when the removed pointer
/// was not null.
pub fn fn_008691d0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let array = this.cast::<NiTArray>();
    if index >= e.get(array, NiTArray::m_usSize) as u32 {
        return 0;
    }
    let new_size = e.get(array, NiTArray::m_usSize).wrapping_sub(1);
    e.set(array, NiTArray::m_usSize, new_size);
    let base = e.get(array, NiTArray::m_pBase);
    let removed = e.mem.u32(base.wrapping_add(index * 4));
    let last = e.mem.u32(base.wrapping_add(new_size as u32 * 4));
    e.mem.set_u32(base.wrapping_add(index * 4), last);
    e.mem.set_u32(base.wrapping_add(new_size as u32 * 4), 0);
    if removed != 0 {
        let count = e.get(array, NiTArray::m_usESize).wrapping_sub(1);
        e.set(array, NiTArray::m_usESize, count);
    }
    removed
}

// Translated from 00869260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiAVObject>,NiTNewInterface<NiPointer<NiAVObject>>>::RemoveAt`
/// (Xbox PDB): moves the `NiPointer` at `index` into `result` and leaves a
/// null one in its slot (a null `NiPointer` in `result` for an index past
/// the end). The element count drops when the slot held a non-null pointer,
/// and the size drops when it was the last slot. Returns `result`.
pub fn ni_t_array_ni_pointer_ni_av_object_remove_at(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    index: u32,
) -> Ptr {
    let array = this.cast::<NiTArray>();
    if index >= e.get(array, NiTArray::m_usSize) as u32 {
        e.call(NI_POINTER_CONSTRUCT, &args![result, 0u32]);
        return result;
    }
    let base = e.get(array, NiTArray::m_pBase);
    let slot = base.wrapping_add(index * 4);
    let taken = e.mem.alloc(4);
    let null_pointer = e.mem.alloc(4);
    e.call(NI_POINTER_COPY_CONSTRUCT, &args![taken, slot]);
    e.call(NI_POINTER_CONSTRUCT, &args![null_pointer, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![slot, null_pointer]);
    e.call(NI_POINTER_DESTRUCT, &args![null_pointer]);
    e.call(NI_POINTER_CONSTRUCT, &args![null_pointer, 0u32]);
    let was_set = e
        .call(NI_POINTER_NOT_EQUAL, &args![taken, null_pointer])
        .bool();
    e.call(NI_POINTER_DESTRUCT, &args![null_pointer]);
    if was_set {
        let count = e.get(array, NiTArray::m_usESize).wrapping_sub(1);
        e.set(array, NiTArray::m_usESize, count);
    }
    let size = e.get(array, NiTArray::m_usSize);
    if index == (size as u32).wrapping_sub(1) & 0xffff {
        e.set(array, NiTArray::m_usSize, size.wrapping_sub(1));
    }
    e.call(NI_POINTER_COPY_CONSTRUCT, &args![result, taken]);
    e.call(NI_POINTER_DESTRUCT, &args![taken]);
    e.mem.free(null_pointer);
    e.mem.free(taken);
    result
}

// Translated from 00869420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends every non-null `NiPointer<NiAVObject>` of the array `source` to
/// the array `destination` (through a temporary copy of each).
pub fn fn_00869420(e: &mut Engine, source: u32, destination: u32) {
    let mut index = 0;
    while index < array_size(e, source) {
        let slot = e.call(ARRAY_ELEMENT, &args![source, index]).u32();
        let copy = e.mem.alloc(4);
        e.call(NI_POINTER_COPY_CONSTRUCT, &args![copy, slot]);
        if e.call(NI_POINTER_GET, &args![copy]).u32() != 0 {
            e.call(ARRAY_ADD_ACTOR_3D, &args![destination, copy]);
        }
        e.call(NI_POINTER_DESTRUCT, &args![copy]);
        e.mem.free(copy);
        index += 1;
    }
}

// Translated from 008694c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends every non-null pointer of the array `source` to the array
/// `destination`.
pub fn fn_008694c0(e: &mut Engine, source: u32, destination: u32) {
    let mut index = 0;
    while index < array_size(e, source) {
        let value = array_pointer_at(e, source, index);
        if value != 0 {
            add_pointer(e, destination, ARRAY_ADD_POINTER, value);
        }
        index += 1;
    }
}

// Translated from 00869510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends every non-null navmesh pointer of the `BSSimpleArray` `source`
/// to the `BSSimpleArray` `destination`.
pub fn fn_00869510(e: &mut Engine, source: u32, destination: u32) {
    let mut index = 0;
    while index < simple_array_size(e, source) {
        let slot = e.call(SIMPLE_ARRAY_ELEMENT, &args![source, index]).u32();
        let value = e.mem.u32(slot);
        if value != 0 {
            add_pointer(e, destination, SIMPLE_ARRAY_ADD, value);
        }
        index += 1;
    }
}

// Translated from 00869560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends every non-null `NiPointer<BSTempEffect>` of the array `source` to
/// the array `destination` (through a temporary copy of each).
pub fn fn_00869560(e: &mut Engine, source: u32, destination: u32) {
    let mut index = 0;
    while index < array_size(e, source) {
        let slot = e.call(ARRAY_ELEMENT, &args![source, index]).u32();
        let copy = e.mem.alloc(4);
        e.call(NI_POINTER_COPY_CONSTRUCT, &args![copy, slot]);
        if e.call(NI_POINTER_GET, &args![copy]).u32() != 0 {
            e.call(ARRAY_ADD_EFFECT, &args![destination, copy]);
        }
        e.call(NI_POINTER_DESTRUCT, &args![copy]);
        e.mem.free(copy);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00418e00, fn_00418e00(Ptr, u32) -> Ptr),
        entry!(0x004b0220, fn_004b0220(Ptr) -> u32),
        entry!(
            0x004dffa0,
            ni_t_array_ni_pointer_bs_temp_effect_remove_all(Ptr<NiTArray>)
        ),
        entry!(0x00867f50, fn_00867f50(u32)),
        entry!(0x00867f70, fn_00867f70() -> bool),
        entry!(0x00867f80, fn_00867f80()),
        entry!(0x00867f90, garbage_collector_add(Ptr)),
        entry!(
            0x008681c0,
            fn_008681c0(Ptr<GarbageCollectorGuard>, u32, u32) -> Ptr<GarbageCollectorGuard>
        ),
        entry!(0x00868210, fn_00868210(Ptr<GarbageCollectorGuard>)),
        entry!(0x00868250, fn_00868250(Ptr) -> bool),
        entry!(0x00868270, garbage_collector_remove(u32)),
        entry!(0x00868330, garbage_collector_add_ov2(Ptr)),
        entry!(0x00868490, garbage_collector_add_ov3(Ptr)),
        entry!(0x00868560, garbage_collector_add_ov4(Ptr)),
        entry!(0x00868660, garbage_collector_add_ov5(Ptr)),
        entry!(0x00868740, fn_00868740(Ptr)),
        entry!(0x00868770, fn_00868770(u32, u32)),
        entry!(0x00868790, garbage_collector_add_ov6(Ptr)),
        entry!(0x00868850, garbage_collector_update()),
        entry!(0x00868ce0, fn_00868ce0(Ptr)),
        entry!(0x00868d10, garbage_collector_clear_temp_effects()),
        entry!(0x00868d70, garbage_collector_clear_all(u8)),
        entry!(0x00869180, fn_00869180(u32) -> bool),
        entry!(0x00869190, fn_00869190(u8)),
        entry!(0x008691b0, fn_008691b0(Ptr) -> bool),
        entry!(0x008691d0, fn_008691d0(Ptr, u32) -> u32),
        entry!(0x00869260, ni_t_array_ni_pointer_ni_av_object_remove_at(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00869420, fn_00869420(u32, u32)),
        entry!(0x008694c0, fn_008694c0(u32, u32)),
        entry!(0x00869510, fn_00869510(u32, u32)),
        entry!(0x00869560, fn_00869560(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test control block: what the doubles of the callees answer.
    const STATE: u32 = 0x0200_0000;
    /// `0078d200` (try the lock): non-zero when the lock can be taken.
    const LOCK_AVAILABLE: u32 = STATE;
    /// `0042ce10`: non-zero when the flagged object passes.
    const FLAGGED_OBJECT_PASSES: u32 = STATE + 4;
    /// `004839c0`: non-zero when the form lookup finds the reference again.
    const LOOKUP_FINDS_REFERENCE: u32 = STATE + 8;
    /// `00878360`: non-zero for a slow frame.
    const SLOW_FRAME: u32 = STATE + 12;
    /// The one `NiAVObject` pointer `0052aa80` accepts.
    const ACCEPTED_3D_OBJECT: u32 = STATE + 16;
    /// Object vtable (slots 0x10 destructor, 0x100 qualifies).
    const VTABLE: u32 = 0x0201_0000;
    const DESTRUCTOR_TARGET: u32 = 0x0300_0010;
    const QUALIFIES_TARGET: u32 = 0x0300_0100;
    const ZERO_TARGET: u32 = 0x0300_0000;
    /// Storage behind every array the tests build.
    const CAPACITY: u32 = 64;

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    /// An engine with the game's globals mapped and a double for every
    /// callee outside this unit.
    fn gc_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011c_3000,
            0x011d_d000,
            0x011d_e000,
            STATE,
            VTABLE,
            0x0300_0000,
        ] {
            e.map(page, 0x1000);
        }
        // Callees that only have to exist.
        for address in [
            0x0040_fbf0u32,
            0x0040_fba0,
            0x0048_ff50,
            0x0048_fef0,
            0x0056_fcb0,
            0x0042_0ba0,
            0x005d_e0a0,
            0x0044_5570,
            0x0044_5430,
            0x0048_3c70,
            0x006f_cf30,
            0x00a5_c480,
            0x0041_8d20,
            0x004a_af10,
            0x0040_f6e0,
            0x0040_1970,
            0x0045_2530,
            0x004a_ae50,
            0x0066_7120,
            0x0052_aa80,
            0x0056_4d80,
            0x0049_6940,
            0x0055_85e0,
            0x00a5_c570,
            0x007a_f430,
            0x0054_9580,
            0x0057_5d70,
            0x0058_6170,
            ZERO_TARGET,
        ] {
            stub(&mut e, address);
        }
        e.register(0x0078_d200, |e, _| returns(e.mem.u32(LOCK_AVAILABLE)));
        e.register(0x0078_d1f0, |e, _| returns(e.mem.u32(LOCK_AVAILABLE)));
        e.register(0x0042_ce10, |e, _| {
            returns(e.mem.u32(FLAGGED_OBJECT_PASSES))
        });
        e.register(0x0087_8360, |e, _| returns(e.mem.u32(SLOW_FRAME)));
        e.register(0x0044_6ef0, |_, _| returns(0x011d_effc));
        e.register(0x0052_aa80, |e, a| {
            returns((e.mem.u32(a[0]) == e.mem.u32(ACCEPTED_3D_OBJECT)) as u32)
        });
        e.register(0x0084_e3a0, |_, a| returns(a[0]));
        e.register(0x0048_39c0, |e, a| {
            returns(if e.mem.u32(LOOKUP_FINDS_REFERENCE) != 0 {
                a[0]
            } else {
                a[0] + 4
            })
        });
        // Array primitives.
        e.register(ARRAY_SIZE, |e, a| returns(e.mem.u16(a[0] + 0xa) as u32));
        e.register(SIMPLE_ARRAY_SIZE, |e, a| returns(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT, |e, a| {
            returns(e.mem.u32(a[0] + 4) + a[1] * 4)
        });
        e.register(SIMPLE_ARRAY_ELEMENT, |e, a| {
            returns(e.mem.u32(a[0] + 4) + a[1] * 4)
        });
        for address in [ARRAY_ADD_POINTER, ARRAY_ADD_EFFECT, ARRAY_ADD_ACTOR_3D] {
            e.register(address, |e, a| {
                let value = e.mem.u32(a[1]);
                let size = e.mem.u16(a[0] + 0xa);
                let base = e.mem.u32(a[0] + 4);
                e.mem.set_u32(base + size as u32 * 4, value);
                e.mem.set_u16(a[0] + 0xa, size + 1);
                let count = e.mem.u16(a[0] + 0xc);
                e.mem.set_u16(a[0] + 0xc, count + 1);
                Ret::default()
            });
        }
        e.register(SIMPLE_ARRAY_ADD, |e, a| {
            let value = e.mem.u32(a[1]);
            let size = e.mem.u32(a[0] + 8);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + size * 4, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(ARRAY_CLEAR, |e, a| {
            e.mem.set_u16(a[0] + 0xa, 0);
            e.mem.set_u16(a[0] + 0xc, 0);
            Ret::default()
        });
        e.register(ARRAY_REMOVE_AT, |e, a| {
            returns(fn_008691d0(e, Ptr::new(a[0]), a[1]))
        });
        e.register(SIMPLE_ARRAY_CLEAR, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(SIMPLE_ARRAY_REMOVE, |e, a| {
            // Remove(index, 1): shift the tail down.
            let size = e.mem.u32(a[0] + 8);
            let base = e.mem.u32(a[0] + 4);
            for i in a[1]..size - 1 {
                let next = e.mem.u32(base + (i + 1) * 4);
                e.mem.set_u32(base + i * 4, next);
            }
            e.mem.set_u32(a[0] + 8, size - 1);
            Ret::default()
        });
        // NiPointer operations on a 4-byte pointer variable.
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            returns(a[0])
        });
        e.register(NI_POINTER_DESTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(NI_POINTER_COPY_CONSTRUCT, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            returns(a[0])
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            returns(a[0])
        });
        e.register(NI_POINTER_GET, |e, a| returns(e.mem.u32(a[0])));
        e.register(NI_POINTER_NOT_EQUAL, |e, a| {
            returns((e.mem.u32(a[0]) != e.mem.u32(a[1])) as u32)
        });
        e.register(INTERLOCKED_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], a[1]);
            returns(old)
        });
        // Objects: the destructor removes the reference from the arrays, as
        // the game's does; slot 0x100 answers the value stored at +8.
        e.register(DESTRUCTOR_TARGET, |e, a| {
            garbage_collector_remove(e, a[0]);
            Ret::default()
        });
        e.register(QUALIFIES_TARGET, |e, a| returns(e.mem.u32(a[0] + 8)));
        let mut slots = vec![ZERO_TARGET; 0x111];
        slots[0x10 / 4] = DESTRUCTOR_TARGET;
        slots[0x100 / 4] = QUALIFIES_TARGET;
        e.put_vtable(VTABLE, &slots);

        // The model loader and the object behind its first field, whose
        // +0x20 is the lock the code tries.
        let inner = e.mem.alloc(0x40);
        let model_loader = e.mem.alloc(8);
        e.mem.set_u32(model_loader, inner);
        e.set_global(MODEL_LOADER, model_loader);
        e
    }

    /// A game object with the test vtable; `qualifies` is its slot 0x100
    /// answer (stored at +8).
    fn make_object(e: &mut Engine, qualifies: bool) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 8, qualifies as u32);
        object
    }

    /// An `NiTArray` at `address` (a game global) holding `values`.
    fn make_array(e: &mut Engine, address: u32, values: &[u32]) {
        let base = e.mem.alloc(CAPACITY * 4);
        e.mem.set_u32(address + 4, base);
        e.mem.set_u16(address + 8, CAPACITY as u16);
        e.mem.set_u16(address + 0xa, values.len() as u16);
        let non_null = values.iter().filter(|v| **v != 0).count();
        e.mem.set_u16(address + 0xc, non_null as u16);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *value);
        }
    }

    /// A `BSSimpleArray` at `address` holding `values`.
    fn make_simple_array(e: &mut Engine, address: u32, values: &[u32]) {
        let base = e.mem.alloc(CAPACITY * 4);
        e.mem.set_u32(address + 4, base);
        e.mem.set_u32(address + 8, values.len() as u32);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *value);
        }
    }

    fn contents(e: &Engine, address: u32) -> Vec<u32> {
        let base = e.mem.u32(address + 4);
        (0..e.mem.u16(address + 0xa) as u32)
            .map(|i| e.mem.u32(base + i * 4))
            .collect()
    }

    fn simple_contents(e: &Engine, address: u32) -> Vec<u32> {
        let base = e.mem.u32(address + 4);
        (0..e.mem.u32(address + 8))
            .map(|i| e.mem.u32(base + i * 4))
            .collect()
    }

    /// Every array empty.
    fn empty_arrays(e: &mut Engine) {
        for address in [
            OBJECT_GARBAGE_FLAG_SET,
            OBJECT_GARBAGE_FLAG_CLEAR,
            ACTOR_3D_GARBAGE_FLAG_SET,
            ACTOR_3D_GARBAGE_FLAG_CLEAR,
            BIPED_GARBAGE_FLAG_SET,
            BIPED_GARBAGE_FLAG_CLEAR,
            ANIM_GARBAGE,
            EFFECT_GARBAGE_FLAG_SET,
            EFFECT_GARBAGE_FLAG_CLEAR,
        ] {
            make_array(e, address, &[]);
        }
        make_simple_array(e, NAVMESH_GARBAGE_FLAG_SET, &[]);
        make_simple_array(e, NAVMESH_GARBAGE_FLAG_CLEAR, &[]);
    }

    fn set_lock_available(e: &mut Engine, available: bool) {
        e.mem.set_u32(LOCK_AVAILABLE, available as u32);
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The argument lists of every logged call to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn first_arguments(e: &Engine, address: u32) -> Vec<u32> {
        calls_to(e, address).iter().map(|a| a[0]).collect()
    }

    fn destroyed_objects(e: &Engine) -> Vec<u32> {
        first_arguments(e, DESTRUCTOR_TARGET)
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = gc_engine();
        let object = e.mem.alloc(16);
        start_log(&mut e);
        assert_eq!(e.call(0x0041_8e00, &args![object, 0u32]).u32(), object);
        assert!(e.mem.block_size(object).is_some());
        assert_eq!(e.call(0x0041_8e00, &args![object, 1u32]).u32(), object);
        assert_eq!(e.mem.block_size(object), None);
        assert_eq!(calls_to(&e, 0x004a_ae50).len(), 2);
    }

    #[test]
    fn vector_deleting_destructor_wrapper_skips_null() {
        let mut e = gc_engine();
        e.register(0x0066_7120, |_, a| returns(a[0] + a[1]));
        assert_eq!(e.call(0x004b_0220, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x004b_0220, &args![0x1000u32]).u32(), 0x1003);
    }

    #[test]
    fn temp_effect_remove_all_nulls_every_slot() {
        let mut e = gc_engine();
        make_array(&mut e, EFFECT_GARBAGE_FLAG_SET, &[0x100, 0x200, 0x300]);
        e.call(0x004d_ffa0, &args![EFFECT_GARBAGE_FLAG_SET]);
        let base = e.mem.u32(EFFECT_GARBAGE_FLAG_SET + 4);
        assert_eq!(e.mem.u32(base), 0);
        assert_eq!(e.mem.u32(base + 4), 0);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.mem.u16(EFFECT_GARBAGE_FLAG_SET + 0xa), 0);
        assert_eq!(e.mem.u16(EFFECT_GARBAGE_FLAG_SET + 0xc), 0);
    }

    #[test]
    fn lock_wrappers_pass_the_garbage_collectors_lock() {
        let mut e = gc_engine();
        set_lock_available(&mut e, true);
        start_log(&mut e);
        e.call(0x0086_7f50, &args![NAME_UPDATE, 1u32]);
        assert!(e.call(0x0086_7f70, &args![]).bool());
        e.call(0x0086_7f80, &args![1u32]);
        assert_eq!(calls_to(&e, 0x0040_fbf0), vec![vec![CRIT_SEC, NAME_UPDATE]]);
        assert_eq!(calls_to(&e, 0x0078_d200), vec![vec![CRIT_SEC]]);
        assert_eq!(calls_to(&e, 0x0040_fba0), vec![vec![CRIT_SEC]]);
        set_lock_available(&mut e, false);
        assert!(!e.call(0x0086_7f70, &args![]).bool());
    }

    #[test]
    fn guard_sets_and_clears_its_kind_bit() {
        let mut e = gc_engine();
        start_log(&mut e);
        let guard: Ptr<GarbageCollectorGuard> = e.new_object();
        e.call(0x0086_81c0, &args![guard, KIND_ANIM, NAME_ADD_ANIM]);
        assert_eq!(e.global::<u32>(ACTIVE_KINDS), KIND_ANIM);
        assert!(!e.get(guard, GarbageCollectorGuard::bWasSet));
        assert_eq!(
            calls_to(&e, 0x0040_fbf0),
            vec![vec![CRIT_SEC, NAME_ADD_ANIM]]
        );
        e.call(0x0086_8210, &args![guard]);
        assert_eq!(e.global::<u32>(ACTIVE_KINDS), 0);
        assert_eq!(calls_to(&e, 0x0040_fba0).len(), 1);
    }

    #[test]
    fn guard_keeps_a_bit_that_was_already_set() {
        let mut e = gc_engine();
        e.set_global(ACTIVE_KINDS, KIND_ANIM | KIND_EFFECT);
        let guard: Ptr<GarbageCollectorGuard> = e.new_object();
        e.call(0x0086_81c0, &args![guard, KIND_ANIM, NAME_ADD_ANIM]);
        assert!(e.get(guard, GarbageCollectorGuard::bWasSet));
        e.call(0x0086_8210, &args![guard]);
        assert_eq!(e.global::<u32>(ACTIVE_KINDS), KIND_ANIM | KIND_EFFECT);
    }

    #[test]
    fn lock_check_goes_through_the_loaders_first_field() {
        let mut e = gc_engine();
        let loader_object = loader(&e);
        let inner = e.mem.u32(loader_object);
        set_lock_available(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x0086_8250, &args![loader_object]).bool());
        assert_eq!(calls_to(&e, 0x0078_d200), vec![vec![inner + 0x20]]);
        set_lock_available(&mut e, false);
        assert!(!e.call(0x0086_91b0, &args![inner]).bool());
    }

    #[test]
    fn add_reference_that_is_not_found_again_is_only_destroyed() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let object = make_object(&mut e, false);
        e.mem.set_u32(LOOKUP_FINDS_REFERENCE, 0);
        start_log(&mut e);
        e.call(0x0086_7f90, &args![object]);
        assert_eq!(destroyed_objects(&e), vec![object]);
        assert!(calls_to(&e, 0x0056_fcb0).is_empty());
        assert_eq!(e.global::<u32>(ACTIVE_KINDS), 0);
        assert_eq!(calls_to(&e, 0x0040_fba0).len(), 2);
    }

    #[test]
    fn add_reference_destroys_it_in_place_when_the_lock_is_free() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let object = make_object(&mut e, false);
        e.mem.set_u32(LOOKUP_FINDS_REFERENCE, 1);
        set_lock_available(&mut e, true);
        start_log(&mut e);
        e.call(0x0086_7f90, &args![object]);
        assert_eq!(destroyed_objects(&e), vec![object]);
        // Taken out of the world first.
        assert_eq!(calls_to(&e, 0x0056_fcb0), vec![vec![object]]);
        assert_eq!(calls_to(&e, 0x0048_3c70), vec![vec![object]]);
        assert_eq!(calls_to(&e, 0x0044_5570), vec![vec![loader(&e), object]]);
        // The loader's lock is released afterwards.
        assert_eq!(calls_to(&e, 0x004a_af10), vec![vec![loader(&e)]]);
        assert!(contents(&e, OBJECT_GARBAGE_FLAG_SET).is_empty());
        assert!(contents(&e, OBJECT_GARBAGE_FLAG_CLEAR).is_empty());
    }

    #[test]
    fn add_reference_queues_it_when_the_lock_is_busy() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let object = make_object(&mut e, false);
        e.mem.set_u32(LOOKUP_FINDS_REFERENCE, 1);
        set_lock_available(&mut e, false);
        start_log(&mut e);
        e.call(0x0086_7f90, &args![object]);
        assert!(destroyed_objects(&e).is_empty());
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_CLEAR), vec![object]);
    }

    #[test]
    fn add_reference_with_the_thread_flag_set_goes_to_the_flag_set_array() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let object = make_object(&mut e, false);
        e.mem.set_u32(LOOKUP_FINDS_REFERENCE, 1);
        set_lock_available(&mut e, true);
        fn_00869190(&mut e, 1);
        // The flagged object is missing: queue.
        e.call(0x0086_7f90, &args![object]);
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET), vec![object]);
        // The flagged object passes its check: destroyed in place.
        let other = make_object(&mut e, false);
        e.set_global(FLAGGED_OBJECT, 0x1234u32);
        e.mem.set_u32(FLAGGED_OBJECT_PASSES, 1);
        start_log(&mut e);
        e.call(0x0086_7f90, &args![other]);
        assert_eq!(destroyed_objects(&e), vec![other]);
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET), vec![object]);
        // The check fails: queued again.
        let third = make_object(&mut e, false);
        e.mem.set_u32(FLAGGED_OBJECT_PASSES, 0);
        e.call(0x0086_7f90, &args![third]);
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET), vec![object, third]);
    }

    #[test]
    fn remove_takes_the_reference_out_of_both_arrays_once() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        make_array(&mut e, OBJECT_GARBAGE_FLAG_SET, &[0x10, 0x20, 0x30, 0x20]);
        make_array(&mut e, OBJECT_GARBAGE_FLAG_CLEAR, &[0x20, 0x40]);
        start_log(&mut e);
        e.call(0x0086_8270, &args![0x20u32]);
        // The first match only: the last element takes its place.
        assert_eq!(
            contents(&e, OBJECT_GARBAGE_FLAG_SET),
            vec![0x10, 0x20, 0x30]
        );
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_CLEAR), vec![0x40]);
        assert_eq!(
            calls_to(&e, 0x0040_fbf0),
            vec![vec![CRIT_SEC, NAME_REMOVE_OBJECT]]
        );
        assert_eq!(calls_to(&e, 0x0040_fba0).len(), 1);
    }

    #[test]
    fn add_animation_shuts_it_down_and_destroys_or_queues_it() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let animation = 0x5000u32;
        let node = 0x6000u32;
        let manager = 0x7000u32;
        e.register(0x0049_6940, |_, _| returns(0x7000));
        e.register(0x0055_85e0, |_, _| returns(0x6000));
        e.register(0x00a5_c570, |_, a| returns(a[1] + 1));
        set_lock_available(&mut e, true);
        start_log(&mut e);
        // Thread flag clear: destroyed on the spot.
        e.call(0x0086_8330, &args![animation]);
        assert_eq!(calls_to(&e, 0x0048_ff50), vec![vec![animation]]);
        assert_eq!(
            calls_to(&e, 0x0048_fef0),
            vec![vec![manager, 0.0f32.to_bits()]]
        );
        // Both controllers are looked up and removed.
        assert_eq!(
            calls_to(&e, 0x00a5_c570),
            vec![vec![node, 0x011f_36bc], vec![node, 0x011f_36e4]]
        );
        assert_eq!(
            calls_to(&e, 0x00a5_c480),
            vec![vec![node, 0x011f_36bd], vec![node, 0x011f_36e5]]
        );
        assert_eq!(calls_to(&e, 0x0044_5430).len(), 1);
        assert_eq!(calls_to(&e, 0x0041_8d20), vec![vec![animation, 1]]);
        assert!(contents(&e, ANIM_GARBAGE).is_empty());
        // Thread flag set and no flagged object: queued.
        fn_00869190(&mut e, 1);
        e.call(0x0086_8330, &args![animation]);
        assert_eq!(contents(&e, ANIM_GARBAGE), vec![animation]);
        assert_eq!(calls_to(&e, 0x0041_8d20).len(), 1);
    }

    #[test]
    fn add_animation_without_manager_or_node_skips_them() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        start_log(&mut e);
        e.call(0x0086_8330, &args![0x5000u32]);
        assert!(calls_to(&e, 0x0048_fef0).is_empty());
        assert!(calls_to(&e, 0x00a5_c570).is_empty());
    }

    #[test]
    fn add_effect_queues_in_the_array_the_thread_flag_picks() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        e.call(0x0086_8490, &args![0x111u32]);
        assert_eq!(contents(&e, EFFECT_GARBAGE_FLAG_CLEAR), vec![0x111]);
        fn_00869190(&mut e, 1);
        e.call(0x0086_8490, &args![0x222u32]);
        assert_eq!(contents(&e, EFFECT_GARBAGE_FLAG_SET), vec![0x222]);
        assert_eq!(e.global::<u32>(ACTIVE_KINDS), 0);
    }

    #[test]
    fn add_biped_has_three_outcomes() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let biped = e.mem.alloc(16);
        // Lock free, flag clear: destroyed and the lock released.
        set_lock_available(&mut e, true);
        start_log(&mut e);
        e.call(0x0086_8560, &args![biped]);
        assert_eq!(calls_to(&e, 0x004a_ae50), vec![vec![biped]]);
        assert_eq!(e.mem.block_size(biped), None);
        assert_eq!(calls_to(&e, 0x004a_af10).len(), 1);
        // Lock busy: flag-clear array.
        set_lock_available(&mut e, false);
        e.call(0x0086_8560, &args![0x333u32]);
        assert_eq!(contents(&e, BIPED_GARBAGE_FLAG_CLEAR), vec![0x333]);
        // Thread flag set, nothing allows in-place: flag-set array.
        fn_00869190(&mut e, 1);
        e.call(0x0086_8560, &args![0x444u32]);
        assert_eq!(contents(&e, BIPED_GARBAGE_FLAG_SET), vec![0x444]);
    }

    #[test]
    fn add_3d_object_marks_it_and_queues_it() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let object = e.mem.alloc(16);
        e.mem.set_u32(object + 4, 0x0000_0123);
        e.call(0x0086_8660, &args![object]);
        assert_eq!(e.mem.u32(object + 4), 0x4000_0123);
        assert_eq!(contents(&e, ACTOR_3D_GARBAGE_FLAG_CLEAR), vec![object]);
        fn_00869190(&mut e, 1);
        // A null object is queued as it is, without the mark.
        e.call(0x0086_8660, &args![0u32]);
        assert_eq!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET), vec![0]);
    }

    #[test]
    fn add_navmesh_queues_the_pointer_the_navmesh_hands_back() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        e.register(NI_POINTER_GET, |_, _| returns(0x9000));
        e.register(0x0045_8b50, |_, a| returns(a[0] + 0x77));
        start_log(&mut e);
        e.call(0x0086_8790, &args![0x1000u32]);
        assert_eq!(calls_to(&e, 0x0040_f6e0), vec![vec![0x9000 + 0x1c]]);
        assert_eq!(
            simple_contents(&e, NAVMESH_GARBAGE_FLAG_CLEAR),
            vec![0x1077]
        );
        fn_00869190(&mut e, 1);
        e.call(0x0086_8790, &args![0x2000u32]);
        assert_eq!(simple_contents(&e, NAVMESH_GARBAGE_FLAG_SET), vec![0x2077]);
    }

    #[test]
    fn mark_and_unmark_set_and_clear_bit_30_with_an_interlocked_exchange() {
        let mut e = gc_engine();
        let object = e.mem.alloc(16);
        e.mem.set_u32(object + 4, 0x8000_0001);
        start_log(&mut e);
        e.call(0x0086_8740, &args![object]);
        assert_eq!(e.mem.u32(object + 4), 0xc000_0001);
        assert_eq!(
            calls_to(&e, INTERLOCKED_EXCHANGE),
            vec![vec![object + 4, 0xc000_0001]]
        );
        e.call(0x0086_8ce0, &args![object]);
        assert_eq!(e.mem.u32(object + 4), 0x8000_0001);
    }

    #[test]
    fn interlocked_wrapper_calls_the_import() {
        let mut e = gc_engine();
        let target = e.mem.alloc(8);
        e.call(0x0086_8770, &args![target, 7u32]);
        assert_eq!(e.mem.u32(target), 7);
    }

    #[test]
    fn update_destroys_queued_animations_up_to_the_budget() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let animations: Vec<u32> = (1..=25).map(|i| 0x1000 + i * 8).collect();
        make_array(&mut e, ANIM_GARBAGE, &animations);
        set_lock_available(&mut e, true);
        start_log(&mut e);
        e.call(0x0086_8850, &args![]);
        // 10 per normal frame, taken from the end.
        let destroyed = first_arguments(&e, 0x0041_8d20);
        let expected: Vec<u32> = animations.iter().rev().take(10).copied().collect();
        assert_eq!(destroyed, expected);
        assert_eq!(contents(&e, ANIM_GARBAGE).len(), 15);
        assert_eq!(calls_to(&e, 0x004a_af10).len(), 1);
        assert_eq!(calls_to(&e, 0x0040_fbf0), vec![vec![CRIT_SEC, NAME_UPDATE]]);
        assert_eq!(calls_to(&e, 0x0040_fba0).len(), 1);

        // A slow frame doubles the budget.
        e.mem.set_u32(SLOW_FRAME, 1);
        start_log(&mut e);
        e.call(0x0086_8850, &args![]);
        assert_eq!(calls_to(&e, 0x0041_8d20).len(), 15);
        assert!(contents(&e, ANIM_GARBAGE).is_empty());
    }

    #[test]
    fn update_does_nothing_with_animations_while_the_loader_lock_is_busy() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        make_array(&mut e, ANIM_GARBAGE, &[0x1008]);
        // Animations come first: the busy lock also holds back this one.
        make_array(&mut e, BIPED_GARBAGE_FLAG_CLEAR, &[0x2008]);
        set_lock_available(&mut e, false);
        start_log(&mut e);
        e.call(0x0086_8850, &args![]);
        assert!(calls_to(&e, 0x0041_8d20).is_empty());
        assert_eq!(contents(&e, ANIM_GARBAGE), vec![0x1008]);
        assert_eq!(contents(&e, BIPED_GARBAGE_FLAG_CLEAR), vec![0x2008]);
        assert!(calls_to(&e, 0x004a_af10).is_empty());
        assert_eq!(calls_to(&e, 0x0040_fba0).len(), 1);
    }

    #[test]
    fn update_destroys_flag_set_biped_animations() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let bipeds: Vec<u32> = (0..3).map(|_| e.mem.alloc(16)).collect();
        make_array(&mut e, BIPED_GARBAGE_FLAG_SET, &bipeds);
        // The flag-clear array would also have work; the flag-set one wins.
        make_array(&mut e, BIPED_GARBAGE_FLAG_CLEAR, &[0x77]);
        set_lock_available(&mut e, true);
        start_log(&mut e);
        e.call(0x0086_8850, &args![]);
        let destroyed = first_arguments(&e, 0x004a_ae50);
        assert_eq!(destroyed, vec![bipeds[2], bipeds[1], bipeds[0]]);
        assert!(contents(&e, BIPED_GARBAGE_FLAG_SET).is_empty());
        assert_eq!(contents(&e, BIPED_GARBAGE_FLAG_CLEAR), vec![0x77]);
    }

    #[test]
    fn update_moves_flag_clear_biped_animations_to_the_flag_set_array() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        make_array(&mut e, BIPED_GARBAGE_FLAG_CLEAR, &[0x11, 0, 0x33]);
        e.call(0x0086_8850, &args![]);
        // Null entries are not carried over.
        assert_eq!(contents(&e, BIPED_GARBAGE_FLAG_SET), vec![0x11, 0x33]);
        assert!(contents(&e, BIPED_GARBAGE_FLAG_CLEAR).is_empty());
    }

    #[test]
    fn update_releases_flag_set_3d_objects_that_qualify() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let marked = e.mem.alloc(16);
        e.mem.set_u32(marked + 4, 0x4000_0000);
        let other = e.mem.alloc(16);
        e.mem.set_u32(other + 4, 0x4000_0000);
        make_array(&mut e, ACTOR_3D_GARBAGE_FLAG_SET, &[marked, other]);
        // `0052aa80` accepts only `marked`.
        e.mem.set_u32(ACCEPTED_3D_OBJECT, marked);
        e.call(0x0086_8850, &args![]);
        assert_eq!(e.mem.u32(marked + 4), 0);
        assert_eq!(e.mem.u32(other + 4), 0x4000_0000);
        assert!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET).is_empty());
    }

    #[test]
    fn update_moves_flag_clear_3d_objects_over_and_empties_the_array() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        make_array(&mut e, ACTOR_3D_GARBAGE_FLAG_CLEAR, &[0x21, 0x22]);
        e.call(0x0086_8850, &args![]);
        assert_eq!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET), vec![0x21, 0x22]);
        assert!(contents(&e, ACTOR_3D_GARBAGE_FLAG_CLEAR).is_empty());
    }

    #[test]
    fn update_destroys_references_until_one_qualifies() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let plain: Vec<u32> = (0..3).map(|_| make_object(&mut e, false)).collect();
        let qualifying = make_object(&mut e, true);
        let fourth = make_object(&mut e, false);
        let queue = [plain[0], plain[1], qualifying, fourth, plain[2]];
        make_array(&mut e, OBJECT_GARBAGE_FLAG_SET, &queue);
        set_lock_available(&mut e, true);
        start_log(&mut e);
        e.call(0x0086_8850, &args![]);
        // Always element 0; each destructor removes its reference by moving
        // the last element to the front, so the order is p0, p2, fourth, and
        // the loop stops after the one that qualifies.
        assert_eq!(
            destroyed_objects(&e),
            vec![plain[0], plain[2], fourth, qualifying]
        );
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET), vec![plain[1]]);
        assert_eq!(calls_to(&e, 0x004a_af10).len(), 1);
    }

    #[test]
    fn update_moves_flag_clear_references_over() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        make_array(&mut e, OBJECT_GARBAGE_FLAG_CLEAR, &[0x31, 0x32]);
        e.call(0x0086_8850, &args![]);
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET), vec![0x31, 0x32]);
        assert!(contents(&e, OBJECT_GARBAGE_FLAG_CLEAR).is_empty());
    }

    #[test]
    fn update_releases_navmeshes_five_at_a_time() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        let navmeshes: Vec<u32> = (1..=8).map(|i| 0x4000 + i * 0x100).collect();
        make_simple_array(&mut e, NAVMESH_GARBAGE_FLAG_SET, &navmeshes);
        start_log(&mut e);
        e.call(0x0086_8850, &args![]);
        let released = first_arguments(&e, 0x0040_1970);
        let expected: Vec<u32> = navmeshes[..5].iter().map(|n| n + 0x1c).collect();
        assert_eq!(released, expected);
        assert_eq!(
            simple_contents(&e, NAVMESH_GARBAGE_FLAG_SET),
            navmeshes[5..].to_vec()
        );
    }

    #[test]
    fn update_moves_flag_clear_navmeshes_over() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        make_simple_array(&mut e, NAVMESH_GARBAGE_FLAG_CLEAR, &[0x51, 0, 0x53]);
        e.call(0x0086_8850, &args![]);
        assert_eq!(
            simple_contents(&e, NAVMESH_GARBAGE_FLAG_SET),
            vec![0x51, 0x53]
        );
        assert!(simple_contents(&e, NAVMESH_GARBAGE_FLAG_CLEAR).is_empty());
    }

    #[test]
    fn update_with_nothing_queued_only_takes_and_releases_the_lock() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        start_log(&mut e);
        e.call(0x0086_8850, &args![]);
        assert_eq!(calls_to(&e, 0x0040_fbf0).len(), 1);
        assert_eq!(calls_to(&e, 0x0040_fba0).len(), 1);
        assert!(calls_to(&e, 0x004a_af10).is_empty());
    }

    #[test]
    fn clear_temp_effects_needs_entries_and_the_loader_lock() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        set_lock_available(&mut e, true);
        start_log(&mut e);
        e.call(0x0086_8d10, &args![]);
        assert!(calls_to(&e, 0x0040_fbf0).is_empty());

        make_array(&mut e, EFFECT_GARBAGE_FLAG_SET, &[0x61, 0x62]);
        set_lock_available(&mut e, false);
        e.call(0x0086_8d10, &args![]);
        assert!(calls_to(&e, 0x0040_fbf0).is_empty());
        assert_eq!(contents(&e, EFFECT_GARBAGE_FLAG_SET).len(), 2);

        set_lock_available(&mut e, true);
        e.call(0x0086_8d10, &args![]);
        assert_eq!(
            calls_to(&e, 0x0040_fbf0),
            vec![vec![CRIT_SEC, NAME_CLEAR_TEMP_EFFECTS]]
        );
        assert!(contents(&e, EFFECT_GARBAGE_FLAG_SET).is_empty());
        assert_eq!(calls_to(&e, 0x004a_af10).len(), 1);
        assert_eq!(calls_to(&e, 0x0040_fba0).len(), 1);
    }

    #[test]
    fn clear_all_destroys_everything_queued() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        set_lock_available(&mut e, true);
        make_array(&mut e, ANIM_GARBAGE, &[0x1008, 0, 0x1010]);
        let bipeds = [e.mem.alloc(16)];
        make_array(&mut e, BIPED_GARBAGE_FLAG_SET, &bipeds);
        make_array(&mut e, EFFECT_GARBAGE_FLAG_SET, &[0x71]);
        let marked = e.mem.alloc(16);
        e.mem.set_u32(marked + 4, 0x4000_0000);
        make_array(&mut e, ACTOR_3D_GARBAGE_FLAG_SET, &[marked]);
        let references = [make_object(&mut e, false), make_object(&mut e, false)];
        make_array(&mut e, OBJECT_GARBAGE_FLAG_SET, &references);
        make_simple_array(&mut e, NAVMESH_GARBAGE_FLAG_SET, &[0x4000, 0x5000]);
        start_log(&mut e);
        e.call(0x0086_8d70, &args![0u8]);
        // Animations (null entries skipped), biped animation, references.
        assert_eq!(first_arguments(&e, 0x0041_8d20), vec![0x1008, 0x1010]);
        assert_eq!(first_arguments(&e, 0x004a_ae50), vec![bipeds[0]]);
        assert_eq!(destroyed_objects(&e), references.to_vec());
        // Effects emptied, 3D object unmarked and emptied, navmeshes released.
        assert!(contents(&e, EFFECT_GARBAGE_FLAG_SET).is_empty());
        assert_eq!(e.mem.u32(marked + 4), 0);
        assert!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET).is_empty());
        assert_eq!(first_arguments(&e, 0x0040_1970), vec![0x401c, 0x501c]);
        assert!(simple_contents(&e, NAVMESH_GARBAGE_FLAG_SET).is_empty());
        // The lock was taken (not tried), released, and the flag cleared.
        assert_eq!(
            calls_to(&e, 0x0040_fbf0).first(),
            Some(&vec![CRIT_SEC, NAME_CLEAR_ALL])
        );
        assert_eq!(e.global::<u8>(CLEARING_ALL), 0);
    }

    #[test]
    fn clear_all_skips_kinds_that_are_being_added() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        set_lock_available(&mut e, true);
        make_array(&mut e, ANIM_GARBAGE, &[0x1008]);
        make_array(&mut e, BIPED_GARBAGE_FLAG_SET, &[0x2008]);
        make_array(&mut e, EFFECT_GARBAGE_FLAG_SET, &[0x71]);
        make_simple_array(&mut e, NAVMESH_GARBAGE_FLAG_SET, &[0x4000]);
        e.set_global(
            ACTIVE_KINDS,
            KIND_ANIM | KIND_BIPED | KIND_EFFECT | KIND_NAVMESH,
        );
        start_log(&mut e);
        e.call(0x0086_8d70, &args![0u8]);
        assert!(calls_to(&e, 0x0041_8d20).is_empty());
        assert_eq!(contents(&e, ANIM_GARBAGE), vec![0x1008]);
        assert_eq!(contents(&e, BIPED_GARBAGE_FLAG_SET), vec![0x2008]);
        assert_eq!(contents(&e, EFFECT_GARBAGE_FLAG_SET), vec![0x71]);
        assert_eq!(simple_contents(&e, NAVMESH_GARBAGE_FLAG_SET), vec![0x4000]);
    }

    #[test]
    fn clear_all_does_nothing_when_already_running_or_the_try_fails() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        start_log(&mut e);
        e.set_global(CLEARING_ALL, 1u8);
        e.call(0x0086_8d70, &args![0u8]);
        assert!(calls_to(&e, 0x0040_fbf0).is_empty());
        e.set_global(CLEARING_ALL, 0u8);
        set_lock_available(&mut e, false);
        e.call(0x0086_8d70, &args![1u8]);
        assert_eq!(calls_to(&e, 0x0078_d200), vec![vec![CRIT_SEC]]);
        assert_eq!(e.global::<u8>(CLEARING_ALL), 0);
        assert!(calls_to(&e, 0x0040_fba0).is_empty());
        // The try succeeds: the lock is released at the end.
        set_lock_available(&mut e, true);
        e.call(0x0086_8d70, &args![1u8]);
        assert_eq!(calls_to(&e, 0x0040_fba0).len(), 1);
    }

    #[test]
    fn clear_all_restores_the_thread_flag_and_moves_the_flag_clear_arrays() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        set_lock_available(&mut e, true);
        make_array(&mut e, OBJECT_GARBAGE_FLAG_CLEAR, &[0x31]);
        make_array(&mut e, ACTOR_3D_GARBAGE_FLAG_CLEAR, &[0x32]);
        make_array(&mut e, BIPED_GARBAGE_FLAG_CLEAR, &[0x33]);
        make_array(&mut e, EFFECT_GARBAGE_FLAG_CLEAR, &[0x34]);
        make_simple_array(&mut e, NAVMESH_GARBAGE_FLAG_CLEAR, &[0x35]);
        fn_00869190(&mut e, 1);
        e.call(0x0086_8d70, &args![0u8]);
        assert!(thread_flag(&mut e));
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET), vec![0x31]);
        assert_eq!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET), vec![0x32]);
        assert_eq!(contents(&e, BIPED_GARBAGE_FLAG_SET), vec![0x33]);
        assert_eq!(contents(&e, EFFECT_GARBAGE_FLAG_SET), vec![0x34]);
        assert_eq!(simple_contents(&e, NAVMESH_GARBAGE_FLAG_SET), vec![0x35]);
        assert!(contents(&e, OBJECT_GARBAGE_FLAG_CLEAR).is_empty());
        assert!(contents(&e, EFFECT_GARBAGE_FLAG_CLEAR).is_empty());
        assert!(simple_contents(&e, NAVMESH_GARBAGE_FLAG_CLEAR).is_empty());
    }

    #[test]
    fn kind_test_reads_the_active_bits() {
        let mut e = gc_engine();
        e.set_global(ACTIVE_KINDS, KIND_BIPED);
        assert!(e.call(0x0086_9180, &args![KIND_BIPED]).bool());
        assert!(!e.call(0x0086_9180, &args![KIND_ANIM]).bool());
        assert!(e.call(0x0086_9180, &args![KIND_ANIM | KIND_BIPED]).bool());
    }

    #[test]
    fn thread_flag_setter_writes_the_tls_byte() {
        let mut e = gc_engine();
        e.call(0x0086_9190, &args![1u8]);
        let flag = e.tls() + 0x298;
        assert_eq!(e.mem.u8(flag), 1);
        e.call(0x0086_9190, &args![0u8]);
        assert_eq!(e.mem.u8(flag), 0);
    }

    #[test]
    fn remove_at_moves_the_last_element_into_the_hole() {
        let mut e = gc_engine();
        make_array(&mut e, OBJECT_GARBAGE_FLAG_SET, &[0x10, 0x20, 0x30]);
        let removed = e
            .call(0x0086_91d0, &args![OBJECT_GARBAGE_FLAG_SET, 0u32])
            .u32();
        assert_eq!(removed, 0x10);
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET), vec![0x30, 0x20]);
        assert_eq!(e.mem.u16(OBJECT_GARBAGE_FLAG_SET + 0xc), 2);
        // Past the end: nothing happens.
        let past_end = e
            .call(0x0086_91d0, &args![OBJECT_GARBAGE_FLAG_SET, 2u32])
            .u32();
        assert_eq!(past_end, 0);
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET).len(), 2);
        // A null element does not lower the element count.
        make_array(&mut e, OBJECT_GARBAGE_FLAG_CLEAR, &[0, 0x20]);
        let null_removed = e
            .call(0x0086_91d0, &args![OBJECT_GARBAGE_FLAG_CLEAR, 0u32])
            .u32();
        assert_eq!(null_removed, 0);
        assert_eq!(e.mem.u16(OBJECT_GARBAGE_FLAG_CLEAR + 0xc), 1);
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_CLEAR), vec![0x20]);
    }

    #[test]
    fn ni_pointer_remove_at_takes_the_pointer_and_leaves_a_null() {
        let mut e = gc_engine();
        make_array(&mut e, ACTOR_3D_GARBAGE_FLAG_SET, &[0x10, 0x20, 0x30]);
        let result = e.mem.alloc(4);
        let back = e.call(0x0086_9260, &args![ACTOR_3D_GARBAGE_FLAG_SET, result, 1u32]);
        assert_eq!(back.u32(), result);
        assert_eq!(e.mem.u32(result), 0x20);
        // Not the last slot: the size stays, the slot is null, one fewer element.
        assert_eq!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET), vec![0x10, 0, 0x30]);
        assert_eq!(e.mem.u16(ACTOR_3D_GARBAGE_FLAG_SET + 0xc), 2);
        // The last slot also shrinks the size.
        e.call(0x0086_9260, &args![ACTOR_3D_GARBAGE_FLAG_SET, result, 2u32]);
        assert_eq!(e.mem.u32(result), 0x30);
        assert_eq!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET), vec![0x10, 0]);
        // An index past the end gives a null pointer.
        e.call(0x0086_9260, &args![ACTOR_3D_GARBAGE_FLAG_SET, result, 9u32]);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET).len(), 2);
        // A null slot does not lower the element count.
        let count = e.mem.u16(ACTOR_3D_GARBAGE_FLAG_SET + 0xc);
        e.call(0x0086_9260, &args![ACTOR_3D_GARBAGE_FLAG_SET, result, 1u32]);
        assert_eq!(e.mem.u16(ACTOR_3D_GARBAGE_FLAG_SET + 0xc), count);
    }

    #[test]
    fn copy_helpers_skip_null_entries() {
        let mut e = gc_engine();
        empty_arrays(&mut e);
        make_array(&mut e, ACTOR_3D_GARBAGE_FLAG_CLEAR, &[0x41, 0, 0x43]);
        e.call(
            0x0086_9420,
            &args![ACTOR_3D_GARBAGE_FLAG_CLEAR, ACTOR_3D_GARBAGE_FLAG_SET],
        );
        assert_eq!(contents(&e, ACTOR_3D_GARBAGE_FLAG_SET), vec![0x41, 0x43]);

        make_array(&mut e, EFFECT_GARBAGE_FLAG_CLEAR, &[0, 0x44, 0x45]);
        e.call(
            0x0086_9560,
            &args![EFFECT_GARBAGE_FLAG_CLEAR, EFFECT_GARBAGE_FLAG_SET],
        );
        assert_eq!(contents(&e, EFFECT_GARBAGE_FLAG_SET), vec![0x44, 0x45]);

        make_array(&mut e, OBJECT_GARBAGE_FLAG_CLEAR, &[0x46, 0]);
        e.call(
            0x0086_94c0,
            &args![OBJECT_GARBAGE_FLAG_CLEAR, OBJECT_GARBAGE_FLAG_SET],
        );
        assert_eq!(contents(&e, OBJECT_GARBAGE_FLAG_SET), vec![0x46]);

        make_simple_array(&mut e, NAVMESH_GARBAGE_FLAG_CLEAR, &[0, 0x47]);
        e.call(
            0x0086_9510,
            &args![NAVMESH_GARBAGE_FLAG_CLEAR, NAVMESH_GARBAGE_FLAG_SET],
        );
        assert_eq!(simple_contents(&e, NAVMESH_GARBAGE_FLAG_SET), vec![0x47]);
    }

    // ---- the frame's model (docs/FRAME_SKELETON.md, PR 5) ----

    /// The logged callees as `world::frame::world_time` names them (the
    /// references' destructor double is their vtable slot +0x10).
    fn model_callees(e: &Engine) -> Vec<world::frame::world_time::Callee> {
        use world::frame::world_time::Callee;
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| match *a {
                DESTRUCTOR_TARGET => Callee::Virtual(0x10),
                a => Callee::Direct(a),
            })
            .collect()
    }

    /// The callees the translations call as Rust functions, so not in the
    /// log: the collector's lock and unlock, the biped's destructor, the
    /// array moves, the 3D object's unmarking and the effects' removal.
    const RUST_CALLED: [u32; 8] = [
        0x0086_7f50,
        0x0086_7f80,
        0x0041_8e00,
        0x0086_94c0,
        0x0086_8ce0,
        0x0086_9420,
        0x004d_ffa0,
        0x0086_9510,
    ];

    fn rust_called() -> Vec<world::frame::world_time::Callee> {
        RUST_CALLED
            .iter()
            .map(|&a| world::frame::world_time::Callee::Direct(a))
            .collect()
    }

    /// `GarbageCollector::Update` (`00868850`) follows the model for each
    /// queue that can come first.
    #[test]
    fn update_follows_the_frame_model() {
        use world::frame::world_time::{follows, function, Garbage, WorldState};
        let model = function(0x0086_8850).expect("modelled");
        let cases: [(Garbage, bool); 8] = [
            (Garbage::None, true),
            (Garbage::Animations, true),
            (Garbage::Animations, false),
            (Garbage::BipedsSet, true),
            (Garbage::BipedsClear, true),
            (Garbage::Objects3dClear, true),
            (Garbage::ReferencesSet, true),
            (Garbage::ReferencesClear, true),
        ];
        for (garbage, free) in cases {
            let mut e = gc_engine();
            empty_arrays(&mut e);
            match garbage {
                Garbage::Animations => make_array(&mut e, ANIM_GARBAGE, &[0x1008, 0x1010]),
                Garbage::BipedsSet => {
                    let biped = e.mem.alloc(16);
                    make_array(&mut e, BIPED_GARBAGE_FLAG_SET, &[biped]);
                }
                Garbage::BipedsClear => make_array(&mut e, BIPED_GARBAGE_FLAG_CLEAR, &[0x11]),
                Garbage::Objects3dClear => make_array(&mut e, ACTOR_3D_GARBAGE_FLAG_CLEAR, &[0x21]),
                Garbage::ReferencesSet => {
                    let reference = make_object(&mut e, true);
                    make_array(&mut e, OBJECT_GARBAGE_FLAG_SET, &[reference]);
                }
                Garbage::ReferencesClear => make_array(&mut e, OBJECT_GARBAGE_FLAG_CLEAR, &[0x46]),
                _ => {}
            }
            set_lock_available(&mut e, free);
            start_log(&mut e);
            e.call(0x0086_8850, &args![]);
            let s = WorldState {
                garbage,
                model_loader_free: free,
                ..WorldState::default()
            };
            follows(model, &s, &model_callees(&e), &rust_called())
                .unwrap_or_else(|why| panic!("{garbage:?} {free}: {why}"));
        }
    }

    /// `GarbageCollector::ClearTempEffects` (`00868d10`) follows the model.
    #[test]
    fn clear_temp_effects_follows_the_frame_model() {
        use world::frame::world_time::{follows, function, WorldState};
        let model = function(0x0086_8d10).expect("modelled");
        for (queued, free) in [(false, true), (true, false), (true, true)] {
            let mut e = gc_engine();
            empty_arrays(&mut e);
            if queued {
                make_array(&mut e, EFFECT_GARBAGE_FLAG_SET, &[0x61]);
            }
            set_lock_available(&mut e, free);
            start_log(&mut e);
            e.call(0x0086_8d10, &args![]);
            let s = WorldState {
                temp_effects: queued,
                model_loader_free: free,
                ..WorldState::default()
            };
            follows(model, &s, &model_callees(&e), &rust_called())
                .unwrap_or_else(|why| panic!("{queued} {free}: {why}"));
        }
    }
}
