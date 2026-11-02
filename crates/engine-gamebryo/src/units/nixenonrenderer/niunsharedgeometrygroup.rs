//! `NiXenonRenderer/niunsharedgeometrygroup.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiUnsharedGeometryGroup` gives every stream of every geometry a vertex buffer of its own:
//! `CreateChip` makes one through the device (`IDirect3DDevice9::CreateVertexBuffer`, slot
//! +0x68) and wraps it in a fresh 0x20-byte `NiVBChip`; `ReleaseChip` releases the buffer
//! and frees the chip.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nixenonrenderer::nigeometrybufferdata::NiGeometryBufferData;

layout! {
    /// `NiUnsharedGeometryGroup` (Xbox PDB), 0x0C bytes: an `NiGeometryGroup`.
    pub struct NiUnsharedGeometryGroup: 0x0C {
        /// `NiGeometryGroup::m_uiRefCount` (Xbox PDB).
        0x04 m_uiRefCount: u32,
        /// `NiGeometryGroup::m_pkD3DDevice` (Xbox PDB): the Direct3D device.
        0x08 m_pkD3DDevice: u32,
    }
}

layout! {
    /// `NiVBChip` (Xbox PDB), 0x20 bytes.
    pub struct NiVBChip: 0x20 {
        /// `m_uiIndex` (Xbox PDB).
        0x00 m_uiIndex: u32,
        /// `m_pkBlock` / `m_pkDynamicSet` (Xbox PDB, a union).
        0x04 m_pkBlock: u32,
        /// `m_pkVB` (Xbox PDB): the vertex buffer (a Direct3D object).
        0x08 m_pkVB: u32,
        /// `m_uiOffset` (Xbox PDB).
        0x0C m_uiOffset: u32,
        /// `m_uiLockFlags` (Xbox PDB).
        0x10 m_uiLockFlags: u32,
        /// `m_uiSize` (Xbox PDB).
        0x14 m_uiSize: u32,
        /// `m_pkNextChip` (Xbox PDB).
        0x18 m_pkNextChip: u32,
        /// `m_pkPrevChip` (Xbox PDB).
        0x1C m_pkPrevChip: u32,
    }
}

/// `NiUnsharedGeometryGroup`'s vtable, stored by the destructor.
const UNSHARED_GROUP_VTABLE: u32 = 0x010f_1008;
/// `NiUnsharedGeometryGroup::ms_pkGroup` (Xbox PDB): the one group.
const UNSHARED_GROUP_INSTANCE: u32 = 0x0126_fa34;
/// `NiGeometryGroup`'s destructor body (`00e98710`, `thiscall`, no stack words).
const GEOMETRY_GROUP_DESTRUCT: u32 = 0x00e9_8710;
/// The sized free (`00aa1460`, `cdecl`: the block and its size).
const FREE_SIZED: u32 = 0x00aa_1460;
/// `00aa13e0` (`cdecl`, the size): allocates an object.
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// The byte `CreateChip` reads to choose the pool: when it is 0 the buffer goes in pool 1
/// (`D3DPOOL_MANAGED`), otherwise pool 0 (`D3DPOOL_DEFAULT`).
const POOL_FLAG_0126F0D0: u32 = 0x0126_f0d0;
/// `IDirect3DDevice9::CreateVertexBuffer` (slot 26) and `IUnknown::Release` (slot 2).
const DEVICE_CREATE_VERTEX_BUFFER: u32 = 0x68;
const RELEASE: u32 = 0x8;

// Translated from 00e94d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnsharedGeometryGroup::` scalar deleting destructor (Xbox PDB): sets the class's
/// vtable, clears `ms_pkGroup` when it is this group, runs `NiGeometryGroup`'s destructor
/// (`00e98710`) and, when bit 0 of `flags` is set, frees the 0x0C bytes (`00aa1460`).
/// Returns `this`.
pub fn ni_unshared_geometry_group_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiUnsharedGeometryGroup>,
    flags: u32,
) -> Ptr<NiUnsharedGeometryGroup> {
    e.mem.set_u32(this.addr(), UNSHARED_GROUP_VTABLE);
    if this.addr() == e.global::<u32>(UNSHARED_GROUP_INSTANCE) {
        e.set_global(UNSHARED_GROUP_INSTANCE, 0u32);
    }
    e.call(GEOMETRY_GROUP_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0xcu32]);
    }
    this
}

// Translated from 00e94e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnsharedGeometryGroup::CreateChip` (Xbox PDB): makes the vertex buffer of one stream of
/// `data`. Its length is the data's maximum vertex count (+0x1C) times the stream's stride
/// (0 for a stream past the count); usage 8 (`D3DUSAGE_WRITEONLY`), or 0x18 (also
/// `D3DUSAGE_SOFTWAREPROCESSING`) when the data is software-processed (+0x14); the data's FVF
/// (+0x0C); the pool from the byte at `0126f0d0`. The device's `CreateVertexBuffer` writes the
/// buffer into the slot of the `data` argument (read back whatever the call leaves there).
/// Then a 0x20-byte chip is allocated (`00aa13e0`): null gives null; otherwise every word is
/// cleared and it holds the buffer (+0x08) and the length (+0x14).
pub fn ni_unshared_geometry_group_create_chip(
    e: &mut Engine,
    this: Ptr<NiUnsharedGeometryGroup>,
    data: Ptr<NiGeometryBufferData>,
    stream: u32,
) -> Ptr<NiVBChip> {
    let vertices = e.get(data, NiGeometryBufferData::m_uiMaxVertCount);
    let stride = if stream < e.get(data, NiGeometryBufferData::m_uiStreamCount) {
        let strides = e.get(data, NiGeometryBufferData::m_puiVertexStride);
        e.mem.u32(strides.wrapping_add(stream.wrapping_mul(4)))
    } else {
        0
    };
    let length = stride.wrapping_mul(vertices);
    let usage: u32 = if e.get(data, NiGeometryBufferData::m_bSoftwareVP) != 0 {
        0x18
    } else {
        8
    };
    let device = e.get(this, NiUnsharedGeometryGroup::m_pkD3DDevice);
    let fvf = e.get(data, NiGeometryBufferData::m_uiFVF);
    let pool = u32::from(e.global::<u8>(POOL_FLAG_0126F0D0) == 0);
    // The exe reuses the `data` argument's stack slot for the buffer; it holds the device's
    // vtable before the call.
    let buffer = e.with_stack(4, |e, slot| {
        let vtable = e.mem.u32(device);
        e.mem.set_u32(slot.addr(), vtable);
        e.vcall(
            device,
            DEVICE_CREATE_VERTEX_BUFFER,
            &args![length, usage, fvf, pool, slot, 0u32],
        );
        e.mem.u32(slot.addr())
    });
    let chip: Ptr<NiVBChip> = e.call(NEW_OBJECT, &args![0x20u32]).ptr();
    if chip.is_null() {
        return Ptr::NULL;
    }
    for offset in [0x00u32, 0x04, 0x08, 0x0C, 0x14, 0x10, 0x18, 0x1C] {
        e.mem.set_u32(chip.addr() + offset, 0);
    }
    e.set(chip, NiVBChip::m_pkVB, buffer);
    e.set(chip, NiVBChip::m_uiSize, length);
    e.set(chip, NiVBChip::m_uiIndex, 0);
    e.set(chip, NiVBChip::m_pkBlock, 0);
    e.set(chip, NiVBChip::m_uiOffset, 0);
    e.set(chip, NiVBChip::m_uiLockFlags, 0);
    chip
}

// Translated from 00e94ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnsharedGeometryGroup::ReleaseChip` (Xbox PDB): for a stream within the data's count
/// whose chip is set, releases the chip's vertex buffer (when set), clears the data's chip
/// slot, clears the chip's buffer and frees the chip's 0x20 bytes (`00aa1460`). `this` is not
/// read.
pub fn ni_unshared_geometry_group_release_chip(
    e: &mut Engine,
    _this: Ptr<NiUnsharedGeometryGroup>,
    data: Ptr<NiGeometryBufferData>,
    stream: u32,
) {
    if stream >= e.get(data, NiGeometryBufferData::m_uiStreamCount) {
        return;
    }
    let chips = e.get(data, NiGeometryBufferData::m_ppkVBChip);
    let chip: Ptr<NiVBChip> = Ptr::new(e.mem.u32(chips.wrapping_add(stream.wrapping_mul(4))));
    if chip.is_null() {
        return;
    }
    let buffer = e.get(chip, NiVBChip::m_pkVB);
    if buffer != 0 {
        e.vcall(buffer, RELEASE, &[]);
    }
    if stream < e.get(data, NiGeometryBufferData::m_uiStreamCount) {
        let chips = e.get(data, NiGeometryBufferData::m_ppkVBChip);
        e.mem.set_u32(chips.wrapping_add(stream.wrapping_mul(4)), 0);
    }
    e.set(chip, NiVBChip::m_pkVB, 0);
    e.call(FREE_SIZED, &args![chip, 0x20u32]);
}

/// The memory manager (`011f6238`) and its allocation (`00aa3e40`, `thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `NiGeometryBufferData::NiGeometryBufferData` (`00e947c0`, engine map; `thiscall`, no stack
/// words; answers the object).
const BUFFER_DATA_CONSTRUCTOR: u32 = 0x00e9_47c0;
/// `NiGeometryGroup::AddObject` (`00e98730`, `nigeometrygroup.cpp`; `thiscall`, the data).
const GEOMETRY_GROUP_ADD_OBJECT: u32 = 0x00e9_8730;

// Translated from 00e94de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnsharedGeometryGroup::AddObject` (Xbox PDB, engine map), `thiscall`, one word: when the
/// object's +0x1C is 0, makes a buffer data (0x54 bytes from the memory manager,
/// constructed), sets its +0x3C to 4, stores it at the object's +0x1C, sets its `m_uiFlags`
/// (+0x04) to `0x01400000` and adds it to the group (`NiGeometryGroup::AddObject`). A failed
/// allocation leaves the data null and the exe writes through it anyway.
pub fn ni_unshared_geometry_group_add_object(
    e: &mut Engine,
    this: Ptr<NiUnsharedGeometryGroup>,
    object: Ptr,
) {
    let object = object.addr();
    if e.mem.u32(object + 0x1c) != 0 {
        return;
    }
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x54u32])
        .u32();
    let data = if block != 0 {
        e.call(BUFFER_DATA_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(data.wrapping_add(0x3c), 4);
    e.mem.set_u32(object + 0x1c, data);
    e.mem.set_u32(data.wrapping_add(4), 0x0140_0000);
    e.call(GEOMETRY_GROUP_ADD_OBJECT, &args![this, data]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_4de0,
            ni_unshared_geometry_group_add_object(Ptr<NiUnsharedGeometryGroup>, Ptr)
        ),
        entry!(
            0x00e9_4d30,
            ni_unshared_geometry_group_scalar_deleting_destructor(
                Ptr<NiUnsharedGeometryGroup>,
                u32,
            )
                -> Ptr<NiUnsharedGeometryGroup>
        ),
        entry!(
            0x00e9_4e30,
            ni_unshared_geometry_group_create_chip(
                Ptr<NiUnsharedGeometryGroup>,
                Ptr<NiGeometryBufferData>,
                u32,
            ) -> Ptr<NiVBChip>
        ),
        entry!(
            0x00e9_4ee0,
            ni_unshared_geometry_group_release_chip(
                Ptr<NiUnsharedGeometryGroup>,
                Ptr<NiGeometryBufferData>,
                u32,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const CREATE_VB_FN: u32 = 0x0040_2000;
    const RELEASE_FN: u32 = 0x0040_2010;
    const DEVICE_VTABLE: u32 = 0x0090_2000;
    const BUFFER_VTABLE: u32 = 0x0090_3000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(GEOMETRY_GROUP_DESTRUCT, |_, _| Ret::default());
        e.register(FREE_SIZED, |_, _| Ret::default());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        // CreateVertexBuffer: writes 0xb0f into the out slot (word 5).
        e.register(CREATE_VB_FN, |e, a| {
            e.mem.set_u32(a[5], 0xb0f);
            Ret::default()
        });
        e.register(RELEASE_FN, |_, _| Ret::default());
        let mut device_slots = vec![0u32; 27];
        device_slots[26] = CREATE_VB_FN;
        e.put_vtable(DEVICE_VTABLE, &device_slots);
        e.put_vtable(BUFFER_VTABLE, &[0, 0, RELEASE_FN]);
        e
    }

    fn data_with_strides(e: &mut Engine, strides: &[u32]) -> Ptr<NiGeometryBufferData> {
        let data: Ptr<NiGeometryBufferData> = e.new_object();
        let n = strides.len() as u32;
        let block = e.mem.alloc(4 * n.max(1));
        let chips = e.mem.alloc(4 * n.max(1));
        for (i, s) in strides.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *s);
        }
        e.set(data, NiGeometryBufferData::m_uiStreamCount, n);
        e.set(data, NiGeometryBufferData::m_puiVertexStride, block);
        e.set(data, NiGeometryBufferData::m_ppkVBChip, chips);
        data
    }

    #[test]
    fn destructor_clears_the_instance_and_frees_with_bit_0() {
        let mut e = engine();
        let group: Ptr<NiUnsharedGeometryGroup> = e.new_object();
        e.set_global(UNSHARED_GROUP_INSTANCE, group.addr());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_4d30, &args![group, 1u32]).u32(), group.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(UNSHARED_GROUP_INSTANCE), 0);
        assert_eq!(e.mem.u32(group.addr()), UNSHARED_GROUP_VTABLE);
        assert_eq!(
            calls_to(&log, GEOMETRY_GROUP_DESTRUCT),
            vec![vec![group.addr()]]
        );
        assert_eq!(calls_to(&log, FREE_SIZED), vec![vec![group.addr(), 0xc]]);

        // Another group: the instance stays, and no free without bit 0.
        let other: Ptr<NiUnsharedGeometryGroup> = e.new_object();
        e.set_global(UNSHARED_GROUP_INSTANCE, group.addr());
        e.call_log = Some(vec![]);
        e.call(0x00e9_4d30, &args![other, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(UNSHARED_GROUP_INSTANCE), group.addr());
        assert!(calls_to(&log, FREE_SIZED).is_empty());
    }

    #[test]
    fn create_chip_sizes_the_buffer_and_fills_the_chip() {
        let mut e = engine();
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let group: Ptr<NiUnsharedGeometryGroup> = e.new_object();
        e.set(group, NiUnsharedGeometryGroup::m_pkD3DDevice, device);
        let data = data_with_strides(&mut e, &[12, 32]);
        e.set(data, NiGeometryBufferData::m_uiMaxVertCount, 10);
        e.set(data, NiGeometryBufferData::m_uiFVF, 0x142);
        e.call_log = Some(vec![]);
        let chip: Ptr<NiVBChip> = e.call(0x00e9_4e30, &args![group, data, 1u32]).ptr();
        let log = e.call_log.take().unwrap();
        let create = calls_to(&log, CREATE_VB_FN);
        assert_eq!(create.len(), 1);
        // Device, length, usage, FVF, pool (the flag byte is 0: managed), out slot, 0.
        assert_eq!(&create[0][..5], &[device, 320, 8, 0x142, 1]);
        assert_eq!(create[0][6], 0);
        assert_eq!(e.get(chip, NiVBChip::m_pkVB), 0xb0f);
        assert_eq!(e.get(chip, NiVBChip::m_uiSize), 320);
        assert_eq!(e.get(chip, NiVBChip::m_pkNextChip), 0);

        // Software processing, a stream past the count, and the pool flag set.
        e.set(data, NiGeometryBufferData::m_bSoftwareVP, 1);
        e.set_global(POOL_FLAG_0126F0D0, 1u8);
        e.call_log = Some(vec![]);
        let chip: Ptr<NiVBChip> = e.call(0x00e9_4e30, &args![group, data, 5u32]).ptr();
        let log = e.call_log.take().unwrap();
        let create = calls_to(&log, CREATE_VB_FN);
        assert_eq!(&create[0][..5], &[device, 0, 0x18, 0x142, 0]);
        assert_eq!(e.get(chip, NiVBChip::m_uiSize), 0);

        // A failed allocation gives null.
        e.register(NEW_OBJECT, |_, _| Ret::default());
        assert!(e
            .call(0x00e9_4e30, &args![group, data, 0u32])
            .ptr::<()>()
            .is_null());
    }

    #[test]
    fn release_chip_releases_the_buffer_and_frees_the_chip() {
        let mut e = engine();
        let group: Ptr<NiUnsharedGeometryGroup> = e.new_object();
        let data = data_with_strides(&mut e, &[12, 32]);
        let chip: Ptr<NiVBChip> = e.new_object();
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, BUFFER_VTABLE);
        e.set(chip, NiVBChip::m_pkVB, buffer);
        let chips = e.get(data, NiGeometryBufferData::m_ppkVBChip);
        e.mem.set_u32(chips + 4, chip.addr());
        e.call_log = Some(vec![]);
        e.call(0x00e9_4ee0, &args![group, data, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE_FN), vec![vec![buffer]]);
        assert_eq!(calls_to(&log, FREE_SIZED), vec![vec![chip.addr(), 0x20]]);
        assert_eq!(e.mem.u32(chips + 4), 0);
        assert_eq!(e.get(chip, NiVBChip::m_pkVB), 0);

        // An empty slot and a stream past the count do nothing.
        e.call_log = Some(vec![]);
        e.call(0x00e9_4ee0, &args![group, data, 1u32]);
        e.call(0x00e9_4ee0, &args![group, data, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);

        // A chip without a buffer is still freed.
        let bare: Ptr<NiVBChip> = e.new_object();
        e.mem.set_u32(chips, bare.addr());
        e.call_log = Some(vec![]);
        e.call(0x00e9_4ee0, &args![group, data, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, RELEASE_FN).is_empty());
        assert_eq!(calls_to(&log, FREE_SIZED), vec![vec![bare.addr(), 0x20]]);
    }

    #[test]
    fn add_object_makes_and_stores_a_data() {
        let mut e = engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(BUFFER_DATA_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(GEOMETRY_GROUP_ADD_OBJECT, |_, _| Ret::default());
        let group = e.mem.alloc(0xc);
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4de0, &args![group, object]);
        let data = e.mem.u32(object + 0x1c);
        assert_eq!(e.mem.u32(data + 0x3c), 4);
        assert_eq!(e.mem.u32(data + 4), 0x0140_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x54]]
        );
        assert_eq!(
            calls_to(&log, GEOMETRY_GROUP_ADD_OBJECT),
            vec![vec![group, data]]
        );
        // An object that has one is left alone.
        e.call_log = Some(vec![]);
        e.call(0x00e9_4de0, &args![group, object]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
