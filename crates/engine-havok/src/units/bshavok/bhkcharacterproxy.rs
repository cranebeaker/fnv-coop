//! `BSHavok/bhkcharacterproxy.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkCharacterProxy` wraps a Havok `hkpCharacterProxy` (the PC build derives the
//! 0xD0-byte `ahkpCharacterProxy` from it, RTTI-confirmed on its vtable at `010c83e8`)
//! and keeps the `hkpCharacterProxyCinfo`-based construction info in `pInfo`. The
//! functions that allocate, construct or tear down Havok objects raise the per-thread
//! allocation word at `TLS + 0x2b4` to 0x10 for their duration and put the old value
//! back, as `bhkWorld` does.
//!
//! The exception-unwinding frames (`FS:[0]` registrations) of the functions marked
//! `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates, `+8` frees `(pointer, size)` and `+0x10` frees `(pointer, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpCharacterProxy`'s `m_shapePhantom` getter (`00cac980`, `MOV EAX,[ECX+0x30]`).
const GET_SHAPE_PHANTOM: u32 = 0x00ca_c980;
/// `hkpCharacterProxy::getPosition` (`00cac910`) and `getLinearVelocity` (`00cac950`),
/// both return a pointer to a vector.
const GET_POSITION: u32 = 0x00ca_c910;
const GET_LINEAR_VELOCITY: u32 = 0x00ca_c950;
/// Sets the cinfo on the Havok proxy (`00cac9a0`, one stack argument).
const COPY_INFO_TO_PROXY: u32 = 0x00ca_c9a0;
/// `hkpCharacterProxy` base constructor (`00cada70`, takes the cinfo) and destructor
/// (`00cadb00`).
const HKP_CHARACTER_PROXY_CONSTRUCT: u32 = 0x00ca_da70;
const HKP_CHARACTER_PROXY_DESTRUCT: u32 = 0x00ca_db00;
/// `hkpEntity` / `hkpPhantom` listener removal (`00c9c670`, `00d07c10`): the listener
/// is the single stack argument.
const ENTITY_REMOVE_LISTENER: u32 = 0x00c9_c670;
const PHANTOM_REMOVE_LISTENER: u32 = 0x00d0_7c10;
/// `hkReferencedObject::removeReference` (`00c905b0`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `bhkSerializable` base methods this unit's overrides call.
const SERIALIZABLE_LOAD_BINARY: u32 = 0x00c8_7650;
const SERIALIZABLE_LINK_OBJECT: u32 = 0x00c8_7670;
const SERIALIZABLE_SAVE_BINARY: u32 = 0x00c8_7850;
const SERIALIZABLE_COPY_MEMBERS: u32 = 0x00c8_7600;
const SERIALIZABLE_CLONE_LOCK: u32 = 0x00c8_76f0;
const SERIALIZABLE_CLONE_UNLOCK: u32 = 0x00c8_7700;
const SERIALIZABLE_GET_VIEWER_STRINGS: u32 = 0x00c8_78c0;
/// `bhkRefObject::Link` (`00c87c90`).
const REF_OBJECT_LINK: u32 = 0x00c8_7c90;
/// `NiStream` helpers: `00a645b0` (called on the stream first by `LoadBinary`) and
/// `NiStream::GetObjectFromLinkID` (`00a63430`).
const STREAM_BEFORE_LOAD: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiObject::RegisterStreamables` (`00a81fe0`).
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a8_1fe0;
/// `NiGetViewerString` (`00a7d330`, cdecl) and `NiPoint3::GetViewerString` (`00a59af0`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_POINT3_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// Copies a vector into a `NiPoint3` (`00458620`, cdecl `(destination, source)`).
const COPY_POINT: u32 = 0x0045_8620;
/// The viewer-string array's grow (`0096ad30`) and set-at (`0096ae90`) methods.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `bhkCharacterProxy`'s constructor (`00c6ce80`) and the construction-info constructor
/// (`00c6cde0`), both in `bhkcharactercontroller.obj`.
const CHARACTER_PROXY_CONSTRUCT: u32 = 0x00c6_ce80;
const CINFO_CONSTRUCT: u32 = 0x00c6_cde0;
/// The `NiRTTI` global whose viewer string heads the proxy's list, the three name
/// strings and the default vector used when there is no Havok proxy.
const PROXY_RTTI: u32 = 0x0126_8344;
const POSITION_NAME: u32 = 0x010c_8400;
const VELOCITY_NAME: u32 = 0x010c_83fc;
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
/// The Havok error/reporting object (`01268360` holds a pointer to it): slot `0x10`
/// enables or disables one message id.
const HAVOK_ERROR_OBJECT: u32 = 0x0126_8360;
const HAVOK_ERROR_SLOT_SET_ENABLED: u32 = 0x10;
const HAVOK_MESSAGE_ID: u32 = 0x6cee_9071;
/// `kernel32` `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `ahkpCharacterProxy` vtables (three, one per base) and its allocation size.
const PROXY_VTABLE: u32 = 0x010c_83e8;
const PROXY_ENTITY_LISTENER_VTABLE: u32 = 0x010c_83cc;
const PROXY_PHANTOM_LISTENER_VTABLE: u32 = 0x010c_83b4;
const PROXY_SIZE: u32 = 0xd0;
/// `bhkCharacterProxy`'s allocation size (`0x3e0`) and the cinfo's (`0x80`).
const CHARACTER_PROXY_SIZE: u32 = 0x3e0;
const CINFO_SIZE: u32 = 0x80;
/// The game heap's allocator (`00aa13e0`, cdecl).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// Float constants of the proxy's default up vector: `0.0` and `1.0`.
const ZERO: u32 = 0x0101_1d78;
const ONE: u32 = 0x0101_2068;

layout! {
    /// `bhkCharacterProxy` (Xbox PDB), 0x3E0 bytes; the fields this unit uses.
    pub struct BhkCharacterProxy: 0x3e0 {
        /// `phkObject` (Xbox PDB): the `ahkpCharacterProxy`.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB): the `bhkCharacterProxyCinfo`.
        0x0c pInfo: Ptr,
    }

    /// `ahkpCharacterProxy` (Xbox PDB), the 0xD0-byte `hkpCharacterProxy`; PC offsets
    /// as read by this unit.
    pub struct AhkpCharacterProxy: 0xd0 {
        /// First vtable word.
        0x00 vtable: u32,
        /// `m_memSizeAndFlags` (Xbox PDB): the allocation size the deleting destructor
        /// frees.
        0x04 m_memSizeAndFlags: u16,
        /// `hkpEntityListener` base vtable.
        0x08 entityListenerVtable: u32,
        /// `hkpPhantomListener` base vtable.
        0x0c phantomListenerVtable: u32,
        /// `m_shapePhantom` (Xbox PDB).
        0x30 m_shapePhantom: Ptr,
        /// `m_up` (Xbox PDB), a four-float `hkVector4`.
        0x40 m_up: [f32; 4],
        /// `m_bodies.m_data` (Xbox PDB).
        0x8c m_bodies: Ptr,
        /// `m_bodies.m_size` (Xbox PDB).
        0x90 m_bodiesSize: i32,
        /// `m_phantoms.m_data` (Xbox PDB).
        0x98 m_phantoms: Ptr,
        /// `m_phantoms.m_size` (Xbox PDB).
        0x9c m_phantomsSize: i32,
        /// `m_userData` (Xbox PDB): the owning `bhkCharacterProxy`.
        0xc0 m_userData: Ptr,
    }
}

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// The Havok allocator of the current memory router (`router + 0x10`).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// `hkpCharacterProxy::m_shapePhantom` of the Havok proxy `hk_object`.
fn shape_phantom(e: &mut Engine, hk_object: u32) -> u32 {
    e.call(GET_SHAPE_PHANTOM, &args![hk_object]).u32()
}

// Translated from 00cabec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ahkpCharacterProxy` (Xbox PDB class, RTTI of vtable `010c83e8`
/// confirmed; the function has no PDB name): runs the `hkpCharacterProxy` base
/// constructor with the cinfo, installs the three vtables, clears the owner at `+0xC0`
/// and sets `m_up` to `(0, 0, 1, 0)` from the exe's `0.0` and `1.0`. Returns `this`.
pub fn ahkp_character_proxy_construct(
    e: &mut Engine,
    this: Ptr<AhkpCharacterProxy>,
    cinfo: Ptr,
) -> Ptr<AhkpCharacterProxy> {
    e.call(HKP_CHARACTER_PROXY_CONSTRUCT, &args![this, cinfo]);
    let zero = e.global::<f32>(ZERO);
    let one = e.global::<f32>(ONE);
    e.set(this, AhkpCharacterProxy::vtable, PROXY_VTABLE);
    e.set(
        this,
        AhkpCharacterProxy::entityListenerVtable,
        PROXY_ENTITY_LISTENER_VTABLE,
    );
    e.set(
        this,
        AhkpCharacterProxy::phantomListenerVtable,
        PROXY_PHANTOM_LISTENER_VTABLE,
    );
    e.set(this, AhkpCharacterProxy::m_userData, Ptr::NULL);
    for (lane, value) in [zero, zero, one, zero].into_iter().enumerate() {
        e.mem.set_f32(
            this.addr() + AhkpCharacterProxy::m_up.off + 4 * lane as u32,
            value,
        );
    }
    this
}

// Translated from 00cabff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::LoadBinary` (Xbox PDB): prepares the stream, loads the
/// `bhkSerializable` part, then gets the cinfo through virtual slot `0xB4`
/// (`CreateCinfo`; its out flag is the stack word that held the stream) and clears its
/// `m_shapePhantom` (`+0x48`).
pub fn bhk_character_proxy_load_binary(e: &mut Engine, this: Ptr<BhkCharacterProxy>, stream: u32) {
    e.call(STREAM_BEFORE_LOAD, &args![stream]);
    e.call(SERIALIZABLE_LOAD_BINARY, &args![this, stream]);
    let info = e
        .with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), stream);
            e.vcall(this.addr(), 0xb4, &args![slot])
        })
        .u32();
    e.mem.set_u32(info + 0x48, 0);
}

// Translated from 00cac030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::LinkObject` (Xbox PDB): looks the linked object up in the
/// stream, gets the cinfo through virtual slot `0xB4` and, when the object exists,
/// stores its Havok object (`+8`) as the cinfo's `m_shapePhantom` (`+0x48`); then the
/// `bhkSerializable` link.
pub fn bhk_character_proxy_link_object(e: &mut Engine, this: Ptr<BhkCharacterProxy>, stream: u32) {
    let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let info = e
        .with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), stream);
            e.vcall(this.addr(), 0xb4, &args![slot])
        })
        .u32();
    if linked != 0 {
        let hk_object = e.mem.u32(linked + 8);
        e.mem.set_u32(info + 0x48, hk_object);
    }
    e.call(SERIALIZABLE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00cac070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::RegisterStreamables` (Xbox PDB; a `bool` there): when the Havok
/// proxy exists and its shape phantom has an owner (`phantom + 0xC`), registers that owner
/// through its virtual slot `0x54`; then `NiObject::RegisterStreamables`, whose value it
/// answers (the exe ends on that call and leaves its `EAX`).
pub fn bhk_character_proxy_register_streamables(
    e: &mut Engine,
    this: Ptr<BhkCharacterProxy>,
    stream: u32,
) -> u32 {
    let hk_object = e.get(this, BhkCharacterProxy::phkObject).addr();
    if hk_object != 0 {
        let phantom = shape_phantom(e, hk_object);
        if phantom != 0 {
            let owner = e.mem.u32(phantom + 0xc);
            if owner != 0 {
                e.vcall(owner, 0x54, &args![stream]);
            }
        }
    }
    e.call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00cac0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::SaveBinary` (Xbox PDB): writes the shape phantom's owner (or 0)
/// through the stream's virtual slot `0x30`, then the `bhkSerializable` part.
pub fn bhk_character_proxy_save_binary(e: &mut Engine, this: Ptr<BhkCharacterProxy>, stream: u32) {
    let hk_object = e.get(this, BhkCharacterProxy::phkObject).addr();
    let mut owner = 0;
    if hk_object != 0 {
        let phantom = shape_phantom(e, hk_object);
        if phantom != 0 {
            owner = e.mem.u32(phantom + 0xc);
        }
    }
    e.vcall(stream, 0x30, &args![owner]);
    e.call(SERIALIZABLE_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00cac0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::GethkWorld` (Xbox PDB): the world (`+8`) of the Havok proxy's
/// shape phantom, or 0.
pub fn bhk_character_proxy_get_hk_world(e: &mut Engine, this: Ptr<BhkCharacterProxy>) -> u32 {
    let hk_object = e.get(this, BhkCharacterProxy::phkObject).addr();
    if hk_object != 0 {
        let phantom = shape_phantom(e, hk_object);
        if phantom != 0 {
            return e.mem.u32(phantom + 8);
        }
    }
    0
}

// Translated from 00cac110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::UnMarkWrite` (Xbox PDB): a tail jump to the object's virtual
/// slot `0x98`; returns that call's `EAX`.
pub fn bhk_character_proxy_un_mark_write(e: &mut Engine, this: Ptr<BhkCharacterProxy>) -> u32 {
    e.vcall(this.addr(), 0x98, &args![]).u32()
}

// Translated from 00cac120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `ahkpCharacterProxy` (first slot of its vtable): the
/// `hkpCharacterProxy` destructor, then, when bit 0 of `flags` is set, frees the
/// block with the size kept in `m_memSizeAndFlags` through the allocator's slot `8`.
/// Returns `this`.
pub fn ahkp_character_proxy_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AhkpCharacterProxy>,
    flags: u8,
) -> Ptr<AhkpCharacterProxy> {
    e.call(HKP_CHARACTER_PROXY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        let heap = allocator(e);
        let size = e.get(this, AhkpCharacterProxy::m_memSizeAndFlags);
        e.vcall(heap, 8, &args![this, u32::from(size)]);
    }
    this
}

// Translated from 00cac150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cac150 - the FS:[0] exception-unwinding frame is not modeled.
/// Detaches the `hkpCharacterProxy` `this` from everything it listens to: removes its
/// entity listener (`this + 8`) from every body in `m_bodies` and its phantom listener
/// (`this + 0xC`) from every phantom in `m_phantoms`, then empties both arrays. The
/// allocation-scope word is 0x10 meanwhile.
pub fn fn_00cac150(e: &mut Engine, this: Ptr<AhkpCharacterProxy>) {
    let (word, saved) = enter_scope(e);
    let mut index = 0;
    while index < e.get(this, AhkpCharacterProxy::m_bodiesSize) {
        let bodies = e.get(this, AhkpCharacterProxy::m_bodies).addr();
        let body = e.mem.u32(bodies + 4 * index as u32);
        e.call(ENTITY_REMOVE_LISTENER, &args![body, this.addr() + 8]);
        index += 1;
    }
    e.set(this, AhkpCharacterProxy::m_bodiesSize, 0);
    let mut index = 0;
    while index < e.get(this, AhkpCharacterProxy::m_phantomsSize) {
        let phantoms = e.get(this, AhkpCharacterProxy::m_phantoms).addr();
        let phantom = e.mem.u32(phantoms + 4 * index as u32);
        e.call(PHANTOM_REMOVE_LISTENER, &args![phantom, this.addr() + 0xc]);
        index += 1;
    }
    e.set(this, AhkpCharacterProxy::m_phantomsSize, 0);
    e.mem.set_u32(word, saved);
}

// Translated from 00cac250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cac250 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterProxy::CreateClone` (Xbox PDB): under the clone lock, allocates a
/// 0x3E0-byte object and constructs it (`00c6ce80`; no construction when the
/// allocation failed), then has `this` copy its members into the clone (the argument
/// is passed on) and releases the lock. Returns the clone.
pub fn bhk_character_proxy_create_clone(
    e: &mut Engine,
    this: Ptr<BhkCharacterProxy>,
    cloning: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let mut clone = 0;
    e.call(SERIALIZABLE_CLONE_LOCK, &args![]);
    let block = e.call(HEAP_ALLOCATE, &args![CHARACTER_PROXY_SIZE]).u32();
    if block != 0 {
        clone = e.call(CHARACTER_PROXY_CONSTRUCT, &args![block]).u32();
    }
    e.call(SERIALIZABLE_COPY_MEMBERS, &args![this, clone, cloning]);
    e.call(SERIALIZABLE_CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

/// Calls the Havok error object's `setEnabled`-style slot `0x10` for the message id
/// the proxy construction silences (`enabled` is a `hkBool`).
fn set_message_enabled(e: &mut Engine, enabled: bool) {
    let object = e.global::<u32>(HAVOK_ERROR_OBJECT);
    e.vcall(
        object,
        HAVOK_ERROR_SLOT_SET_ENABLED,
        &args![HAVOK_MESSAGE_ID, u32::from(enabled)],
    );
}

// Translated from 00cac300 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cac300 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterProxy::Init` (Xbox PDB): with the Havok message `0x6cee9071` disabled
/// around the construction, allocates the 0xD0-byte `ahkpCharacterProxy` through the
/// router's allocator, constructs it from `cinfo`, hands it to the virtual slot `0x8C`
/// (set the Havok object) of `this`, drops the construction reference
/// (`removeReference`), then calls virtual slot `0xBC` with `cinfo`.
pub fn bhk_character_proxy_init(e: &mut Engine, this: Ptr<BhkCharacterProxy>, cinfo: Ptr) {
    let (word, saved) = enter_scope(e);
    set_message_enabled(e, false);
    let heap = allocator(e);
    let block = e.vcall(heap, 4, &args![PROXY_SIZE]).u32();
    e.mem.set_u16(block + 4, PROXY_SIZE as u16);
    let proxy = ahkp_character_proxy_construct(e, Ptr::new(block), cinfo);
    set_message_enabled(e, true);
    e.vcall(this.addr(), 0x8c, &args![proxy]);
    e.call(REMOVE_REFERENCE, &args![proxy]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cac410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cac410 - the FS:[0] exception-unwinding frame is not modeled.
/// Drops one reference of the shape phantom's owner (`phantom + 0xC`) of the Havok
/// proxy, if any: `InterlockedDecrement` on its count at `+4`, and its virtual slot `4`
/// (the delete) when the count reached zero. (No PDB name.)
pub fn fn_00cac410(e: &mut Engine, this: Ptr<BhkCharacterProxy>) {
    let (word, saved) = enter_scope(e);
    let hk_object = e.get(this, BhkCharacterProxy::phkObject).addr();
    if hk_object != 0 {
        let phantom = shape_phantom(e, hk_object);
        if phantom != 0 {
            let owner = e.mem.u32(phantom + 0xc);
            if owner != 0 && e.call(INTERLOCKED_DECREMENT, &args![owner + 4]).u32() == 0 {
                e.vcall(owner, 4, &args![]);
            }
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cac4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::Link` (Xbox PDB): when `linked` (low byte) is non-zero takes a
/// reference on the shape phantom's owner (`InterlockedIncrement` at `+4`), otherwise
/// releases one (`00cac410`); then points the Havok proxy's `m_userData` (`+0xC0`) at
/// `this` (linked) or null (unlinked) and runs `bhkRefObject::Link`.
pub fn bhk_character_proxy_link(e: &mut Engine, this: Ptr<BhkCharacterProxy>, linked: u32) {
    let hk_object = if this.is_null() {
        0
    } else {
        e.get(this, BhkCharacterProxy::phkObject).addr()
    };
    if linked as u8 != 0 {
        let held = e.get(this, BhkCharacterProxy::phkObject).addr();
        if held != 0 {
            let phantom = shape_phantom(e, held);
            if phantom != 0 {
                let owner = e.mem.u32(phantom + 0xc);
                if owner != 0 {
                    e.call(INTERLOCKED_INCREMENT, &args![owner + 4]);
                }
            }
        }
    } else {
        fn_00cac410(e, this);
    }
    if hk_object != 0 {
        let owner_pointer = if linked as u8 != 0 { this.addr() } else { 0 };
        e.mem.set_u32(hk_object + 0xc0, owner_pointer);
    }
    e.call(REF_OBJECT_LINK, &args![this, linked]);
}

// Translated from 00cac520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cac520 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterProxy::CreateCinfo` (Xbox PDB): writes into the byte at `created`
/// whether `pInfo` was empty; if so, allocates the 0x80-byte cinfo through the router's
/// allocator, constructs it when the allocation succeeded (`00c6cde0`; the pointer kept
/// is the allocation), stores it in `pInfo` and, when the Havok proxy exists, passes it
/// to the proxy (`00cac9a0`). Returns `pInfo`.
pub fn bhk_character_proxy_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkCharacterProxy>,
    created: Ptr,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let empty = e.get(this, BhkCharacterProxy::pInfo).is_null();
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let heap = allocator(e);
        let info = e.vcall(heap, 4, &args![CINFO_SIZE]).u32();
        if info != 0 {
            e.call(CINFO_CONSTRUCT, &args![info]);
        }
        e.set(this, BhkCharacterProxy::pInfo, Ptr::<()>::new(info));
        let hk_object = e.get(this, BhkCharacterProxy::phkObject).addr();
        if hk_object != 0 {
            e.call(COPY_INFO_TO_PROXY, &args![hk_object, info]);
        }
    }
    let info = e.get(this, BhkCharacterProxy::pInfo).addr();
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00cac5e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cac5e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterProxy::KillCinfo` (Xbox PDB): when the byte `destroy` is non-zero,
/// frees the cinfo (0x80 bytes, allocator slot `8`) if there is one and clears `pInfo`.
pub fn bhk_character_proxy_kill_cinfo(e: &mut Engine, this: Ptr<BhkCharacterProxy>, destroy: u32) {
    let (word, saved) = enter_scope(e);
    if destroy as u8 != 0 {
        let info = e.get(this, BhkCharacterProxy::pInfo).addr();
        if info != 0 {
            let heap = allocator(e);
            e.vcall(heap, 8, &args![info, CINFO_SIZE]);
        }
        e.set(this, BhkCharacterProxy::pInfo, Ptr::NULL);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cac680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Teardown sequence of the proxy (no PDB name): virtual slot `0x98`, then
/// `00cac150` on the Havok proxy (`+8`), slot `0x98` again, then a tail call to slot
/// `8` of the sub-object at `this + 0x10` (the `m_ignoredCollisionStartCollector`,
/// whose vtable is its own); returns that call's `EAX`.
pub fn fn_00cac680(e: &mut Engine, this: Ptr<BhkCharacterProxy>) -> u32 {
    e.vcall(this.addr(), 0x98, &args![]);
    let hk_object = e.get(this, BhkCharacterProxy::phkObject);
    fn_00cac150(e, Ptr::new(hk_object.addr()));
    e.vcall(this.addr(), 0x98, &args![]);
    e.vcall(this.addr() + 0x10, 8, &args![]).u32()
}

/// Appends the viewer string `text` to the array `strings` (size and capacity are
/// 16-bit fields at `+0xA` and `+8`; `+0xE` is the grow step): grows it when full,
/// then sets the element at the old size.
fn push_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00cac6b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cac6b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterProxy::GetViewerStrings` (Xbox PDB): after the `bhkSerializable`
/// strings, appends the class's viewer string, the position and the linear velocity
/// of the Havok proxy as `NiPoint3` viewer strings (the position is the default vector
/// `01267e30` without a Havok proxy; the velocity is only refreshed with one, so
/// without it the previous `NiPoint3` value is shown again). The proxy's virtual slot
/// `0x94` brackets each read of the Havok proxy.
pub fn bhk_character_proxy_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkCharacterProxy>,
    strings: u32,
) {
    let (word, saved) = enter_scope(e);
    e.call(SERIALIZABLE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(PROXY_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    e.with_stack(12, |e, point| {
        let mut source = DEFAULT_VECTOR;
        let hk_object = e.get(this, BhkCharacterProxy::phkObject).addr();
        if hk_object != 0 {
            e.vcall(this.addr(), 0x94, &args![]);
            source = e.call(GET_POSITION, &args![hk_object]).u32();
            e.vcall(this.addr(), 0x94, &args![]);
        }
        e.call(COPY_POINT, &args![point, source]);
        let text = e
            .call(NI_POINT3_GET_VIEWER_STRING, &args![point, POSITION_NAME])
            .u32();
        push_viewer_string(e, strings, text);
        let hk_object = e.get(this, BhkCharacterProxy::phkObject).addr();
        if hk_object != 0 {
            e.vcall(this.addr(), 0x94, &args![]);
            let velocity = e.call(GET_LINEAR_VELOCITY, &args![hk_object]).u32();
            e.call(COPY_POINT, &args![point, velocity]);
            e.vcall(this.addr(), 0x94, &args![]);
        }
        let text = e
            .call(NI_POINT3_GET_VIEWER_STRING, &args![point, VELOCITY_NAME])
            .u32();
        push_viewer_string(e, strings, text);
    });
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ca_bec0,
            ahkp_character_proxy_construct(Ptr<AhkpCharacterProxy>, Ptr) -> Ptr<AhkpCharacterProxy>
        ),
        entry!(
            0x00ca_bff0,
            bhk_character_proxy_load_binary(Ptr<BhkCharacterProxy>, u32)
        ),
        entry!(
            0x00ca_c030,
            bhk_character_proxy_link_object(Ptr<BhkCharacterProxy>, u32)
        ),
        entry!(
            0x00ca_c070,
            bhk_character_proxy_register_streamables(Ptr<BhkCharacterProxy>, u32) -> u32
        ),
        entry!(
            0x00ca_c0b0,
            bhk_character_proxy_save_binary(Ptr<BhkCharacterProxy>, u32)
        ),
        entry!(
            0x00ca_c0f0,
            bhk_character_proxy_get_hk_world(Ptr<BhkCharacterProxy>) -> u32
        ),
        entry!(
            0x00ca_c110,
            bhk_character_proxy_un_mark_write(Ptr<BhkCharacterProxy>) -> u32
        ),
        entry!(
            0x00ca_c120,
            ahkp_character_proxy_scalar_deleting_destructor(
                Ptr<AhkpCharacterProxy>,
                u8,
            ) -> Ptr<AhkpCharacterProxy>
        ),
        entry!(0x00ca_c150, fn_00cac150(Ptr<AhkpCharacterProxy>)),
        entry!(
            0x00ca_c250,
            bhk_character_proxy_create_clone(Ptr<BhkCharacterProxy>, u32) -> u32
        ),
        entry!(
            0x00ca_c300,
            bhk_character_proxy_init(Ptr<BhkCharacterProxy>, Ptr)
        ),
        entry!(0x00ca_c410, fn_00cac410(Ptr<BhkCharacterProxy>)),
        entry!(
            0x00ca_c4b0,
            bhk_character_proxy_link(Ptr<BhkCharacterProxy>, u32)
        ),
        entry!(
            0x00ca_c520,
            bhk_character_proxy_create_cinfo(Ptr<BhkCharacterProxy>, Ptr) -> u32
        ),
        entry!(
            0x00ca_c5e0,
            bhk_character_proxy_kill_cinfo(Ptr<BhkCharacterProxy>, u32)
        ),
        entry!(0x00ca_c680, fn_00cac680(Ptr<BhkCharacterProxy>) -> u32),
        entry!(
            0x00ca_c6b0,
            bhk_character_proxy_get_viewer_strings(Ptr<BhkCharacterProxy>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The addresses of the calls recorded so far, in order.
    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    /// The argument words of every logged call to `address`.
    fn args_of(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object whose vtable (at `vtable`) has the given `(byte offset, target)` slots.
    fn object_with_slots(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let mut table = vec![0u32; 0x40];
        for (offset, target) in slots {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &table);
        let object = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A fake memory router whose allocator vtable slots are the doubles `0x0900_0010`
    /// (allocate), `..11` (free), `..12` (sized allocate) and `..13` (free with bytes).
    fn install_heap(e: &mut Engine) {
        let allocator = object_with_slots(
            e,
            0x0200_0100,
            &[
                (4, 0x0900_0010),
                (8, 0x0900_0011),
                (0xc, 0x0900_0012),
                (0x10, 0x0900_0013),
            ],
        );
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register_double(0x0900_0010, |e, a| ret(e.mem.alloc(a[1])));
        e.register(0x0900_0011, |_, _| Ret::default());
        e.register_double(0x0900_0012, |e, a| {
            let requested = e.mem.u32(a[1]);
            ret(e.mem.alloc(requested))
        });
        e.register(0x0900_0013, |_, _| Ret::default());
    }

    /// A proxy whose Havok object `hk` has a shape phantom with an owner at `+0xC` and a
    /// world at `+8`. Returns `(proxy, hk, owner)`.
    fn proxy_with_phantom(e: &mut Engine) -> (Ptr<BhkCharacterProxy>, u32, u32) {
        let proxy = e.new_object::<BhkCharacterProxy>();
        let hk = e.mem.alloc(0xd0);
        let phantom = e.mem.alloc(0x20);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(phantom + 0xc, owner);
        e.mem.set_u32(phantom + 8, 0x7777_0000);
        e.set(proxy, BhkCharacterProxy::phkObject, Ptr::new(hk));
        e.register_double(GET_SHAPE_PHANTOM, move |_, a| {
            assert_eq!(a, &[hk]);
            ret(phantom)
        });
        (proxy, hk, owner)
    }

    #[test]
    fn construction_installs_the_vtables_and_the_up_vector() {
        let mut e = engine();
        e.register(HKP_CHARACTER_PROXY_CONSTRUCT, |_, _| ret(0));
        e.set_global(ZERO, 0.25f32);
        e.set_global(ONE, 3.0f32);
        let proxy = e.new_object::<AhkpCharacterProxy>();
        e.mem.set_u32(proxy.addr() + 0xc0, 0x1234);
        e.call_log = Some(vec![]);
        let result = e.call(0x00ca_bec0, &args![proxy, 0x7000u32]).ptr::<()>();
        assert_eq!(result.addr(), proxy.addr());
        assert_eq!(
            args_of(&e, HKP_CHARACTER_PROXY_CONSTRUCT),
            [[proxy.addr(), 0x7000]]
        );
        assert_eq!(e.mem.u32(proxy.addr()), 0x010c_83e8);
        assert_eq!(e.mem.u32(proxy.addr() + 8), 0x010c_83cc);
        assert_eq!(e.mem.u32(proxy.addr() + 0xc), 0x010c_83b4);
        assert_eq!(e.mem.u32(proxy.addr() + 0xc0), 0);
        assert_eq!(
            [0, 4, 8, 12].map(|lane| e.mem.f32(proxy.addr() + 0x40 + lane)),
            [0.25, 0.25, 3.0, 0.25]
        );
    }

    #[test]
    fn load_binary_clears_the_shape_phantom_of_the_cinfo() {
        let mut e = engine();
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x48, 0x77);
        let proxy = object_with_slots(&mut e, 0x0200_0000, &[(0xb4, 0x0900_0001)]);
        e.register(STREAM_BEFORE_LOAD, |_, _| Ret::default());
        e.register(SERIALIZABLE_LOAD_BINARY, |_, _| Ret::default());
        e.register_double(0x0900_0001, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x5000, "the slot word holds the stream");
            ret(info)
        });
        e.call_log = Some(vec![]);
        e.call(0x00ca_bff0, &args![proxy, 0x5000u32]);
        assert_eq!(e.mem.u32(info + 0x48), 0);
        assert_eq!(
            order(&e)[1..],
            [STREAM_BEFORE_LOAD, SERIALIZABLE_LOAD_BINARY, 0x0900_0001]
        );
        assert_eq!(args_of(&e, SERIALIZABLE_LOAD_BINARY), [[proxy, 0x5000]]);
    }

    #[test]
    fn link_object_stores_the_linked_havok_object() {
        let mut e = engine();
        let info = e.mem.alloc(0x80);
        let proxy = object_with_slots(&mut e, 0x0200_0000, &[(0xb4, 0x0900_0001)]);
        let linked = e.mem.alloc(0x10);
        e.mem.set_u32(linked + 8, 0xabc0);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, a| {
            assert_eq!(a, &[0x5000]);
            ret(linked)
        });
        e.register_double(0x0900_0001, move |_, _| ret(info));
        e.register(SERIALIZABLE_LINK_OBJECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ca_c030, &args![proxy, 0x5000u32]);
        assert_eq!(e.mem.u32(info + 0x48), 0xabc0);
        assert_eq!(args_of(&e, SERIALIZABLE_LINK_OBJECT), [[proxy, 0x5000]]);
        // No object for the link id: the cinfo is left alone.
        e.mem.set_u32(info + 0x48, 5);
        e.register(STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| ret(0));
        e.call(0x00ca_c030, &args![proxy, 0x5000u32]);
        assert_eq!(e.mem.u32(info + 0x48), 5);
    }

    #[test]
    fn register_streamables_registers_the_phantom_owner() {
        let mut e = engine();
        let (proxy, _, owner) = proxy_with_phantom(&mut e);
        let mut table = vec![0u32; 0x20];
        table[0x54 / 4] = 0x0900_0020;
        e.put_vtable(0x0200_0200, &table);
        e.mem.set_u32(owner, 0x0200_0200);
        e.register(0x0900_0020, |_, _| Ret::default());
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ca_c070, &args![proxy, 0x5000u32]);
        assert_eq!(args_of(&e, 0x0900_0020), [[owner, 0x5000]]);
        assert_eq!(
            args_of(&e, NI_OBJECT_REGISTER_STREAMABLES),
            [[proxy.addr(), 0x5000]]
        );
        // Without a Havok object only the base registration runs.
        e.set(proxy, BhkCharacterProxy::phkObject, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x00ca_c070, &args![proxy, 0x5000u32]);
        assert!(args_of(&e, 0x0900_0020).is_empty());
        assert_eq!(args_of(&e, NI_OBJECT_REGISTER_STREAMABLES).len(), 1);
    }

    #[test]
    fn save_binary_writes_the_phantom_owner_or_zero() {
        let mut e = engine();
        let (proxy, _, owner) = proxy_with_phantom(&mut e);
        let stream = object_with_slots(&mut e, 0x0200_0300, &[(0x30, 0x0900_0030)]);
        e.register(0x0900_0030, |_, _| Ret::default());
        e.register(SERIALIZABLE_SAVE_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ca_c0b0, &args![proxy, stream]);
        assert_eq!(args_of(&e, 0x0900_0030), [[stream, owner]]);
        assert_eq!(
            args_of(&e, SERIALIZABLE_SAVE_BINARY),
            [[proxy.addr(), stream]]
        );
        e.set(proxy, BhkCharacterProxy::phkObject, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x00ca_c0b0, &args![proxy, stream]);
        assert_eq!(args_of(&e, 0x0900_0030), [[stream, 0]]);
    }

    #[test]
    fn get_hk_world_reads_the_phantom_world() {
        let mut e = engine();
        let (proxy, _, _) = proxy_with_phantom(&mut e);
        assert_eq!(e.call(0x00ca_c0f0, &args![proxy]).u32(), 0x7777_0000);
        e.set(proxy, BhkCharacterProxy::phkObject, Ptr::NULL);
        assert_eq!(e.call(0x00ca_c0f0, &args![proxy]).u32(), 0);
    }

    #[test]
    fn un_mark_write_tail_calls_slot_0x98() {
        let mut e = engine();
        let proxy = object_with_slots(&mut e, 0x0200_0000, &[(0x98, 0x0900_0001)]);
        e.register_double(0x0900_0001, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x00ca_c110, &args![proxy]).u32(), proxy + 1);
    }

    #[test]
    fn deleting_destructor_frees_with_the_stored_size_only_on_request() {
        let mut e = engine();
        install_heap(&mut e);
        e.register(HKP_CHARACTER_PROXY_DESTRUCT, |_, _| Ret::default());
        let proxy = e.new_object::<AhkpCharacterProxy>();
        e.set(proxy, AhkpCharacterProxy::m_memSizeAndFlags, 0xd0u16);
        e.call_log = Some(vec![]);
        let result = e.call(0x00ca_c120, &args![proxy, 0u8]).ptr::<()>();
        assert_eq!(result.addr(), proxy.addr());
        assert!(args_of(&e, 0x0900_0011).is_empty());
        e.call(0x00ca_c120, &args![proxy, 1u8]);
        assert_eq!(args_of(&e, 0x0900_0011).len(), 1);
        assert_eq!(args_of(&e, 0x0900_0011)[0][1..], [proxy.addr(), 0xd0]);
        assert_eq!(args_of(&e, HKP_CHARACTER_PROXY_DESTRUCT).len(), 2);
    }

    #[test]
    fn detaching_removes_the_listeners_and_empties_both_arrays() {
        let mut e = engine();
        let proxy = e.new_object::<AhkpCharacterProxy>();
        let bodies = e.mem.alloc(8);
        e.mem.set_u32(bodies, 0x1000);
        e.mem.set_u32(bodies + 4, 0x2000);
        let phantoms = e.mem.alloc(4);
        e.mem.set_u32(phantoms, 0x3000);
        e.set(proxy, AhkpCharacterProxy::m_bodies, Ptr::new(bodies));
        e.set(proxy, AhkpCharacterProxy::m_bodiesSize, 2);
        e.set(proxy, AhkpCharacterProxy::m_phantoms, Ptr::new(phantoms));
        e.set(proxy, AhkpCharacterProxy::m_phantomsSize, 1);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x55);
        e.register_double(ENTITY_REMOVE_LISTENER, move |e, _| {
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        e.register(PHANTOM_REMOVE_LISTENER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ca_c150, &args![proxy]);
        assert_eq!(
            args_of(&e, ENTITY_REMOVE_LISTENER),
            [[0x1000, proxy.addr() + 8], [0x2000, proxy.addr() + 8]]
        );
        assert_eq!(
            args_of(&e, PHANTOM_REMOVE_LISTENER),
            [[0x3000, proxy.addr() + 0xc]]
        );
        assert_eq!(e.get(proxy, AhkpCharacterProxy::m_bodiesSize), 0);
        assert_eq!(e.get(proxy, AhkpCharacterProxy::m_phantomsSize), 0);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn clone_copies_members_into_a_new_object_under_the_clone_lock() {
        let mut e = engine();
        let block = e.mem.alloc(0x3e0);
        e.register(SERIALIZABLE_CLONE_LOCK, |_, _| Ret::default());
        e.register(SERIALIZABLE_CLONE_UNLOCK, |_, _| Ret::default());
        e.register_double(HEAP_ALLOCATE, move |_, a| {
            assert_eq!(a, &[0x3e0]);
            ret(block)
        });
        e.register(CHARACTER_PROXY_CONSTRUCT, |_, a| ret(a[0] + 0x10));
        e.register(SERIALIZABLE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00ca_c250, &args![0x1111u32, 0x2222u32]).u32();
        assert_eq!(clone, block + 0x10);
        assert_eq!(
            order(&e)[1..],
            [
                SERIALIZABLE_CLONE_LOCK,
                HEAP_ALLOCATE,
                CHARACTER_PROXY_CONSTRUCT,
                SERIALIZABLE_COPY_MEMBERS,
                SERIALIZABLE_CLONE_UNLOCK
            ]
        );
        assert_eq!(
            args_of(&e, SERIALIZABLE_COPY_MEMBERS),
            [[0x1111, block + 0x10, 0x2222]]
        );
        // A failed allocation skips the constructor and copies into null.
        e.register(HEAP_ALLOCATE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ca_c250, &args![0x1111u32, 0x2222u32]).u32(), 0);
        assert!(args_of(&e, CHARACTER_PROXY_CONSTRUCT).is_empty());
        assert_eq!(
            args_of(&e, SERIALIZABLE_COPY_MEMBERS),
            [[0x1111, 0, 0x2222]]
        );
    }

    #[test]
    fn init_builds_the_havok_proxy_between_the_message_toggles() {
        let mut e = engine();
        install_heap(&mut e);
        let reporter = object_with_slots(&mut e, 0x0200_0400, &[(0x10, 0x0900_0040)]);
        e.set_global(HAVOK_ERROR_OBJECT, reporter);
        e.register(0x0900_0040, |_, _| Ret::default());
        let proxy = object_with_slots(
            &mut e,
            0x0200_0000,
            &[(0x8c, 0x0900_0041), (0xbc, 0x0900_0042)],
        );
        e.register(0x0900_0041, |_, _| Ret::default());
        e.register(0x0900_0042, |_, _| Ret::default());
        e.register_double(HKP_CHARACTER_PROXY_CONSTRUCT, |e, a| {
            assert_eq!(e.mem.u16(a[0] + 4), 0xd0);
            let word = e.tls() + 0x2b4;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x66);
        e.call_log = Some(vec![]);
        e.call(0x00ca_c300, &args![proxy, 0x9000u32]);
        let log = order(&e);
        assert_eq!(args_of(&e, 0x0900_0010)[0][1], 0xd0);
        assert_eq!(
            args_of(&e, 0x0900_0040)
                .iter()
                .map(|c| c[1..].to_vec())
                .collect::<Vec<_>>(),
            [[0x6cee_9071, 0], [0x6cee_9071, 1]]
        );
        let proxy_object = args_of(&e, 0x0900_0041)[0][1];
        assert_eq!(args_of(&e, REMOVE_REFERENCE), [[proxy_object]]);
        assert_eq!(args_of(&e, 0x0900_0042), [[proxy, 0x9000]]);
        assert_eq!(args_of(&e, HKP_CHARACTER_PROXY_CONSTRUCT)[0][1], 0x9000);
        let position = |address| log.iter().position(|a| *a == address).unwrap();
        assert!(position(0x0900_0040) < position(HKP_CHARACTER_PROXY_CONSTRUCT));
        assert!(position(HKP_CHARACTER_PROXY_CONSTRUCT) < position(0x0900_0041));
        assert!(position(0x0900_0041) < position(REMOVE_REFERENCE));
        assert!(position(REMOVE_REFERENCE) < position(0x0900_0042));
        assert_eq!(e.mem.u32(word), 0x66);
    }

    #[test]
    fn releasing_the_owner_deletes_it_when_the_count_reaches_zero() {
        let mut e = engine();
        let (proxy, _, owner) = proxy_with_phantom(&mut e);
        e.put_vtable(0x0200_0200, &[0, 0x0900_0050]);
        e.mem.set_u32(owner, 0x0200_0200);
        e.register(0x0900_0050, |_, _| Ret::default());
        let remaining = Rc::new(RefCell::new(1u32));
        let shared = remaining.clone();
        e.register_double(INTERLOCKED_DECREMENT, move |_, a| {
            assert_eq!(a, &[owner + 4]);
            ret(*shared.borrow())
        });
        e.call_log = Some(vec![]);
        e.call(0x00ca_c410, &args![proxy]);
        assert!(args_of(&e, 0x0900_0050).is_empty());
        *remaining.borrow_mut() = 0;
        e.call(0x00ca_c410, &args![proxy]);
        assert_eq!(args_of(&e, 0x0900_0050), [[owner]]);
    }

    #[test]
    fn link_takes_or_drops_a_reference_and_points_the_havok_proxy_at_us() {
        let mut e = engine();
        let (proxy, hk, owner) = proxy_with_phantom(&mut e);
        e.register(REF_OBJECT_LINK, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |_, _| ret(1));
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(5));
        e.mem.set_u32(hk + 0xc0, 0xdead);
        e.call_log = Some(vec![]);
        e.call(0x00ca_c4b0, &args![proxy, 1u32]);
        assert_eq!(args_of(&e, INTERLOCKED_INCREMENT), [[owner + 4]]);
        assert_eq!(e.mem.u32(hk + 0xc0), proxy.addr());
        assert_eq!(args_of(&e, REF_OBJECT_LINK), [[proxy.addr(), 1]]);
        e.call_log = Some(vec![]);
        e.call(0x00ca_c4b0, &args![proxy, 0u32]);
        assert_eq!(args_of(&e, INTERLOCKED_DECREMENT), [[owner + 4]]);
        assert_eq!(e.mem.u32(hk + 0xc0), 0);
    }

    #[test]
    fn create_cinfo_allocates_once_and_hands_it_to_the_havok_proxy() {
        let mut e = engine();
        install_heap(&mut e);
        let (proxy, hk, _) = proxy_with_phantom(&mut e);
        e.register(CINFO_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_INFO_TO_PROXY, |_, _| Ret::default());
        let created = e.mem.alloc(4);
        e.mem.set_u32(created, 0xffff_ffff);
        e.call_log = Some(vec![]);
        let info = e.call(0x00ca_c520, &args![proxy, created]).u32();
        assert_ne!(info, 0);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.get(proxy, BhkCharacterProxy::pInfo).addr(), info);
        assert_eq!(args_of(&e, 0x0900_0010)[0][1], 0x80);
        assert_eq!(args_of(&e, CINFO_CONSTRUCT), [[info]]);
        assert_eq!(args_of(&e, COPY_INFO_TO_PROXY), [[hk, info]]);
        // The second request reuses it and constructs nothing.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ca_c520, &args![proxy, created]).u32(), info);
        assert_eq!(e.mem.u8(created), 0);
        assert!(args_of(&e, CINFO_CONSTRUCT).is_empty());
    }

    #[test]
    fn kill_cinfo_frees_the_cinfo_only_when_asked() {
        let mut e = engine();
        install_heap(&mut e);
        let proxy = e.new_object::<BhkCharacterProxy>();
        e.set(proxy, BhkCharacterProxy::pInfo, Ptr::new(0x4000));
        e.call_log = Some(vec![]);
        e.call(0x00ca_c5e0, &args![proxy, 0u32]);
        assert_eq!(e.get(proxy, BhkCharacterProxy::pInfo).addr(), 0x4000);
        e.call(0x00ca_c5e0, &args![proxy, 1u32]);
        assert_eq!(args_of(&e, 0x0900_0011)[0][1..], [0x4000, 0x80]);
        assert!(e.get(proxy, BhkCharacterProxy::pInfo).is_null());
        // No cinfo left: nothing more is freed.
        e.call(0x00ca_c5e0, &args![proxy, 1u32]);
        assert_eq!(args_of(&e, 0x0900_0011).len(), 1);
    }

    #[test]
    fn teardown_brackets_the_detach_with_slot_0x98_and_ends_in_the_collector() {
        let mut e = engine();
        let proxy = object_with_slots(&mut e, 0x0200_0000, &[(0x98, 0x0900_0060)]);
        let hk = e.mem.alloc(0xd0);
        e.mem.set_u32(proxy + 8, hk);
        // The sub-object at +0x10 starts with its own vtable word.
        let mut table = vec![0u32; 8];
        table[2] = 0x0900_0061;
        e.put_vtable(0x0200_0500, &table);
        e.mem.set_u32(proxy + 0x10, 0x0200_0500);
        e.register(0x0900_0060, |_, _| Ret::default());
        e.register_double(0x0900_0061, |_, a| ret(a[0] + 2));
        e.call_log = Some(vec![]);
        let result = e.call(0x00ca_c680, &args![proxy]).u32();
        assert_eq!(result, proxy + 0x10 + 2);
        assert_eq!(order(&e)[1..], [0x0900_0060, 0x0900_0060, 0x0900_0061]);
    }

    #[test]
    fn viewer_strings_append_the_class_position_and_velocity() {
        let mut e = engine();
        let proxy = object_with_slots(&mut e, 0x0200_0000, &[(0x94, 0x0900_0070)]);
        let hk = e.mem.alloc(0xd0);
        e.mem.set_u32(proxy + 8, hk);
        e.register(0x0900_0070, |_, _| Ret::default());
        e.set_global(PROXY_RTTI, 0x1111u32);
        let strings = e.mem.alloc(0x20);
        e.mem.set_u16(strings + 8, 8);
        e.mem.set_u16(strings + 0xe, 4);
        let stored = Rc::new(RefCell::new(Vec::new()));
        let sink = stored.clone();
        e.register(SERIALIZABLE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register_double(NI_GET_VIEWER_STRING, |_, a| {
            assert_eq!(a, &[0x1111]);
            ret(0xa0)
        });
        e.register(GET_POSITION, |_, _| ret(0x0100_1000));
        e.register(GET_LINEAR_VELOCITY, |_, _| ret(0x0100_2000));
        e.register(COPY_POINT, |_, _| Ret::default());
        e.register(NI_POINT3_GET_VIEWER_STRING, |_, a| ret(a[1]));
        e.register_double(STRINGS_SET_AT, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.u32(a[2])));
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ca_c6b0, &args![proxy, strings]);
        assert_eq!(
            *stored.borrow(),
            [(0, 0xa0), (1, POSITION_NAME), (2, VELOCITY_NAME)]
        );
        let sources = |e: &Engine| {
            args_of(e, COPY_POINT)
                .iter()
                .map(|c| c[1])
                .collect::<Vec<_>>()
        };
        assert_eq!(sources(&e), [0x0100_1000, 0x0100_2000]);
        // Without a Havok proxy the position comes from the default vector and the
        // velocity copy is skipped.
        e.mem.set_u32(proxy + 8, 0);
        e.mem.set_u16(strings + 0xa, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ca_c6b0, &args![proxy, strings]);
        assert_eq!(sources(&e), [DEFAULT_VECTOR]);
    }

    #[test]
    fn register_streamables_answers_the_base_registration() {
        let mut e = engine();
        let (proxy, _, _) = proxy_with_phantom(&mut e);
        e.set(proxy, BhkCharacterProxy::phkObject, Ptr::NULL);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00ca_c070, &args![proxy, 0x5000u32]).u32(), 1);
    }
}
