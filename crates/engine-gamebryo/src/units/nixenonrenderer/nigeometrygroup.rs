//! `NiXenonRenderer/nigeometrygroup.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGeometryBufferData::RemoveIB` (engine map; thiscall, no stack words).
const BUFFER_DATA_REMOVE_IB: u32 = 0x00e8_eee0;

// Translated from 00e98730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryGroup::AddObject` (Xbox PDB; thiscall, one word): counts
/// one more object (interlocked increment of +0x04) and stores the group in
/// the buffer data's +0x08.
pub fn ni_geometry_group_add_object(e: &mut Engine, this: Ptr, data: Ptr) {
    e.call(IMPORT_INTERLOCKED_INCREMENT, &[this.addr() + 4]);
    e.mem.set_u32(data.addr() + 8, this.addr());
}

// Translated from 00e98750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryGroup::RemoveObject` (Xbox PDB; thiscall, one word): counts
/// one object less (interlocked decrement of +0x04), clears the buffer
/// data's group (+0x08), calls the group's virtual slot +0x1C with
/// (`data`, stream) for each of the data's streams (count at +0x20, read
/// once) and removes its index buffer (`NiGeometryBufferData::RemoveIB`).
pub fn ni_geometry_group_remove_object(e: &mut Engine, this: Ptr, data: Ptr) {
    e.call(IMPORT_INTERLOCKED_DECREMENT, &[this.addr() + 4]);
    let streams = e.mem.u32(data.addr() + 0x20);
    e.mem.set_u32(data.addr() + 8, 0);
    let mut stream = 0;
    while stream < streams {
        e.vcall(this.addr(), 0x1c, &[data.addr(), stream]);
        stream += 1;
    }
    e.call(BUFFER_DATA_REMOVE_IB, &[data.addr()]);
}

/// The vtable the deleting destructor installs (`010f1644`, `NiGeometryGroup`'s, with
/// `_purecall` slots).
const GEOMETRY_GROUP_VTABLE: u32 = 0x010f_1644;
/// `00aa1460` (`cdecl`, two words: block, size): the Ni sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00e98790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryGroup::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall` with one
/// stack word (the flags): installs the vtable `010f1644`, releases the object at `+0x08` when
/// set (its slot +8, `stdcall` with the object), and when bit 0 of the flags is set frees the
/// group with size 0xc (`00aa1460`). Answers `this`.
pub fn ni_geometry_group_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let held = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(this.addr(), GEOMETRY_GROUP_VTABLE);
    if held != 0 {
        e.vcall(held, 8, &[]);
    }
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0xcu32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_8730, ni_geometry_group_add_object(Ptr, Ptr)),
        entry!(0x00e9_8750, ni_geometry_group_remove_object(Ptr, Ptr)),
        entry!(
            0x00e9_8790,
            ni_geometry_group_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(BUFFER_DATA_REMOVE_IB, |_, _| Ret::default());
        e.register(0x0066_001c, |_, _| Ret::default());
        e
    }

    #[test]
    fn add_object_counts_and_links() {
        let mut e = engine();
        let group = e.mem.alloc(8);
        let data = e.mem.alloc(0x24);
        e.call(0x00e9_8730, &[group, data]);
        assert_eq!(e.mem.u32(group + 4), 1);
        assert_eq!(e.mem.u32(data + 8), group);
    }

    #[test]
    fn remove_object_releases_every_stream() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x1c, 0x0066_001c);
        let group = e.mem.alloc(8);
        e.mem.set_u32(group, vtable);
        e.mem.set_u32(group + 4, 2);
        let data = e.mem.alloc(0x24);
        e.mem.set_u32(data + 8, group);
        e.mem.set_u32(data + 0x20, 3);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8750, &[group, data]);
        let log = e.call_log.as_ref().unwrap();
        let streams: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == 0x0066_001c)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            streams,
            vec![
                vec![group, data, 0],
                vec![group, data, 1],
                vec![group, data, 2]
            ]
        );
        assert_eq!(log.last().unwrap(), &(BUFFER_DATA_REMOVE_IB, vec![data]));
        assert_eq!(e.mem.u32(group + 4), 1);
        assert_eq!(e.mem.u32(data + 8), 0);
        // No streams: only the index buffer goes.
        e.mem.set_u32(data + 0x20, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8750, &[group, data]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn deleting_destructor_releases_the_held_object() {
        let mut e = engine();
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        e.register(0x0066_0008, |_, _| Ret::default());
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, 0x0066_0008);
        let held = e.mem.alloc(4);
        e.mem.set_u32(held, vtable);
        let group = e.mem.alloc(0xc);
        e.mem.set_u32(group + 8, held);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_8790, &args![group, 1u32]).u32(), group);
        assert_eq!(e.mem.u32(group), GEOMETRY_GROUP_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0066_0008, vec![held]));
        assert_eq!(log[2], (NI_SIZED_DELETE, vec![group, 0xc]));
        e.mem.set_u32(group + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8790, &args![group, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
