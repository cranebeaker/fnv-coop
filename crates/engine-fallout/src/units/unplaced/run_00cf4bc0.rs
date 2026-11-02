//! `(unplaced)/run_00cf4bc0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkConstraintManager::GetInfo` (engine map, `00cf4840`; cdecl: the constraint type).
const CONSTRAINT_MANAGER_GET_INFO: u32 = 0x00cf_4840;
/// `hkConstraintCinfo::SetData` (engine map, `00c89ac0`; thiscall: the constraint data).
const CONSTRAINT_CINFO_SET_DATA: u32 = 0x00c8_9ac0;
/// The word of the thread's TLS block (`e.tls() + 0x2b4`) set to 0x10 while it runs.
const TLS_CONTEXT: u32 = 0x2b4;

// Translated from 00cf4bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf4bc0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f34838, state variable) is not translated
/// Unnamed in the engine map (cdecl; called from two `SaveBinary` bodies): writes a
/// constraint's data to `stream`. With the thread's TLS word +0x2b4 set to 0x10 (restored
/// after): the type from `data`'s vtable slot +0x20, an info object for it
/// (`bhkConstraintManager::GetInfo`) filled from `data` (`hkConstraintCinfo::SetData`), the
/// type written (4 bytes, one component) through the write function at +0xc of the stream's
/// binary stream (+0x250; its answer added to that object's +4), the info object's vtable
/// slot +0xc with the stream, then its slot +0 with 1 (its deleting destructor shape).
pub fn fn_00cf4bc0(e: &mut Engine, stream: Ptr, data: Ptr) {
    let context = e.tls() + TLS_CONTEXT;
    let saved = e.mem.u32(context);
    e.mem.set_u32(context, 0x10);
    e.with_stack(8, |e, locals| {
        // +0 the type, +4 the component-size word (the exe reuses its second argument's slot).
        let kind_slot = locals.addr();
        let sizes = kind_slot + 4;
        let kind = e.vcall(data.addr(), 0x20, &args![]).u32();
        e.mem.set_u32(kind_slot, kind);
        let info = e.call(CONSTRAINT_MANAGER_GET_INFO, &args![kind]).u32();
        e.call(CONSTRAINT_CINFO_SET_DATA, &args![info, data]);
        let binary = e.mem.u32(stream.addr() + 0x250);
        let write = e.mem.u32(binary + 0xc);
        e.mem.set_u32(sizes, 4);
        let written = e
            .call(write, &args![binary, kind_slot, 4u32, sizes, 1u32])
            .u32();
        let total = e.mem.u32(binary + 4);
        e.mem.set_u32(binary + 4, total.wrapping_add(written));
        e.vcall(info, 0xc, &args![stream]);
        e.vcall(info, 0x0, &args![1u32]);
    });
    e.mem.set_u32(context, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf_4bc0, fn_00cf4bc0(Ptr, Ptr))]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn writes_the_type_and_saves_through_the_info_object() {
        let mut e = Engine::new();
        let data_vtable = e.mem.alloc(0x24);
        e.mem.set_u32(data_vtable + 0x20, 0x0079_0020);
        e.register(0x0079_0020, |e, _| {
            let context = e.tls() + TLS_CONTEXT;
            assert_eq!(e.mem.u32(context), 0x10);
            7u32.into_ret()
        });
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, data_vtable);
        let info_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(info_vtable, 0x0078_0000);
        e.mem.set_u32(info_vtable + 0xc, 0x0078_000c);
        e.register(0x0078_0000, |_, _| Ret::default());
        e.register(0x0078_000c, |_, _| Ret::default());
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, info_vtable);
        e.register_double(CONSTRAINT_MANAGER_GET_INFO, move |_, _| info.into_ret());
        e.register(CONSTRAINT_CINFO_SET_DATA, |_, _| Ret::default());
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 4, 100);
        e.mem.set_u32(binary + 0xc, 0x0077_0000);
        e.register(0x0077_0000, |e, a| {
            assert_eq!((e.mem.u32(a[1]), a[2], e.mem.u32(a[3]), a[4]), (7, 4, 4, 1));
            4u32.into_ret()
        });
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, binary);
        let context = e.tls() + TLS_CONTEXT;
        e.mem.set_u32(context, 3);
        e.call_log = Some(vec![]);
        e.call(0x00cf_4bc0, &args![stream, data]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x0079_0020,
                CONSTRAINT_MANAGER_GET_INFO,
                CONSTRAINT_CINFO_SET_DATA,
                0x0077_0000,
                0x0078_000c,
                0x0078_0000
            ]
        );
        assert_eq!(log[2].1, vec![7]);
        assert_eq!(log[3].1, vec![info, data]);
        assert_eq!(log[5].1, vec![info, stream]);
        assert_eq!(log[6].1, vec![info, 1]);
        assert_eq!(e.mem.u32(binary + 4), 104);
        assert_eq!(e.mem.u32(context), 3);
    }
}
