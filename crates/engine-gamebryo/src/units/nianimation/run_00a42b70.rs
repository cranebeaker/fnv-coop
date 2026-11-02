//! `NiAnimation/run_00a42b70` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObjectNET::ms_RTTI` (the `NiRTTI` record whose name is
/// `"NiObjectNET"`, `0109b2e0`).
const NI_OBJECT_NET_RTTI: u32 = 0x011f_4304;
/// `NiObject::GetRTTI` (Xbox PDB vtable slot +8).
const GET_RTTI_SLOT: u32 = 0x8;
/// `NiRTTI::m_pkBaseRTTI` (Xbox PDB) +4.
const RTTI_BASE: u32 = 0x4;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8.
const STREAM_FILE_VERSION: u32 = 0xd8;
/// The first file version this conversion no longer applies to.
const CONVERSION_END_VERSION: u32 = 0x0a01_0068;
/// The conversion run on the object (`00a42b70`, `cdecl (stream, object)`).
const CONVERT: u32 = 0x00a4_2b70;

// Translated from 00a42c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl (stream, object)`): for files before
/// version `0x0A010068` and a non-null object whose class derives from
/// `NiObjectNET` (its `GetRTTI` record, vtable +8, or one of its bases at
/// +4, is `NiObjectNET`'s), runs `00a42b70` on them.
pub fn fn_00a42c80(e: &mut Engine, stream: Ptr, object: Ptr) {
    if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) >= CONVERSION_END_VERSION
        || object.is_null()
    {
        return;
    }
    let mut rtti = e.vcall(object.addr(), GET_RTTI_SLOT, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_OBJECT_NET_RTTI {
            e.call(CONVERT, &args![stream, object]);
            return;
        }
        rtti = e.mem.u32(rtti.wrapping_add(RTTI_BASE));
    }
}

/// The `NiRTTI` records (static, built at start-up) of `NiAVObject` (`011f4280`),
/// `NiRollController` (`011f3f58`), `NiLookAtController` (`011f3f98`) and `NiPathController`
/// (`011f3f80`): their initializers name them.
const AV_OBJECT_RTTI: u32 = 0x011f_4280;
const ROLL_CONTROLLER_RTTI: u32 = 0x011f_3f58;
const LOOK_AT_CONTROLLER_RTTI: u32 = 0x011f_3f98;
const PATH_CONTROLLER_RTTI: u32 = 0x011f_3f80;
/// The conversions of the three old controllers (`cdecl`, the controller): `00a427e0`,
/// `00a42570` and `00a42900`.
const CONVERT_ROLL: u32 = 0x00a4_27e0;
const CONVERT_LOOK_AT: u32 = 0x00a4_2570;
const CONVERT_PATH: u32 = 0x00a4_2900;

/// Whether `object`'s class is `rtti`'s or derives from it (its `GetRTTI` record or one of
/// the record's bases).
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, GET_RTTI_SLOT, &[]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record.wrapping_add(RTTI_BASE));
    }
    false
}

// Translated from 00a42b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl (stream, object)`): for an `NiAVObject`, each of its
/// controllers (+0xc, the next one +0x30 read first) that is an `NiRollController`,
/// `NiLookAtController` or `NiPathController` (tested in that order, the RTTI asked anew for
/// each) is converted (`00a427e0`, `00a42570`, `00a42900`); then, when vtable +0xc answers
/// non-null, the same runs on each child (array +0xa0, count +0xa6, both re-read each step).
pub fn fn_00a42b70(e: &mut Engine, stream: Ptr, object: Ptr) {
    let o = object.addr();
    if o == 0 || !is_kind_of(e, o, AV_OBJECT_RTTI) {
        return;
    }
    let mut controller = e.mem.u32(o + 0xc);
    while controller != 0 {
        let next = e.mem.u32(controller + 0x30);
        if is_kind_of(e, controller, ROLL_CONTROLLER_RTTI) {
            e.call(CONVERT_ROLL, &args![controller]);
        } else if is_kind_of(e, controller, LOOK_AT_CONTROLLER_RTTI) {
            e.call(CONVERT_LOOK_AT, &args![controller]);
        } else if is_kind_of(e, controller, PATH_CONTROLLER_RTTI) {
            e.call(CONVERT_PATH, &args![controller]);
        }
        controller = next;
    }
    if e.vcall(o, 0xc, &[]).u32() != 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(o + 0xa6)) {
            let children = e.mem.u32(o + 0xa0);
            let child = e.mem.u32(children.wrapping_add(index * 4));
            fn_00a42b70(e, stream, Ptr::new(child));
            index += 1;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a42c80, fn_00a42c80(Ptr, Ptr)),
        entry!(0x00a4_2b70, fn_00a42b70(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (version, object present, RTTI chain contains NiObjectNET) -> converts.
    #[test]
    fn converts_old_object_net_derived_objects_only() {
        const GET_RTTI: u32 = 0x7200_0000;
        for (version, present, derived, converts) in [
            (0x0a01_0067u32, true, true, true),
            (0x0a01_0068, true, true, false),
            (0x0a01_0067, false, true, false),
            (0x0a01_0067, true, false, false),
        ] {
            let mut e = Engine::new();
            e.put_vtable(0x7000_0000, &[0, 0, GET_RTTI]);
            e.map(0x7300_0000, 4);
            let base = e.mem.alloc(8);
            let own = e.mem.alloc(8);
            e.mem.set_u32(own + 4, base);
            if derived {
                e.mem.set_u32(base + 4, NI_OBJECT_NET_RTTI);
            }
            e.mem.set_u32(0x7300_0000, own);
            e.register(GET_RTTI, |e, _| Ret {
                eax: e.mem.u32(0x7300_0000),
                ..Ret::default()
            });
            e.register(CONVERT, |_, _| Ret::default());
            let stream = e.mem.alloc(0x5c4);
            e.mem.set_u32(stream + 0xd8, version);
            let object = if present { e.mem.alloc(0x18) } else { 0 };
            if present {
                e.mem.set_u32(object, 0x7000_0000);
            }
            e.call_log = Some(vec![]);
            e.call(0x00a42c80, &args![stream, object]);
            let converted: Vec<_> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 == CONVERT)
                .map(|c| c.1)
                .collect();
            let expected = if converts {
                vec![vec![stream, object]]
            } else {
                vec![]
            };
            assert_eq!(converted, expected);
        }
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    /// An object whose `GetRTTI` (the double at `get_rtti`) answers `rtti`; vtable +0xc is the
    /// double at `get_rtti + 1`, answering `node`.
    fn object(e: &mut Engine, get_rtti: u32, rtti: u32, node: u32) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, get_rtti);
        e.mem.set_u32(vtable + 0xc, get_rtti + 1);
        e.register_double(get_rtti, move |_, _| rtti.into_ret());
        e.register_double(get_rtti + 1, move |_, _| node.into_ret());
        let object = e.mem.alloc(0xb0);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn converts_old_controllers_and_children() {
        let mut e = Engine::new();
        // The RTTI records' pages (their base words read as 0).
        e.map(0x011f_3000, 0x2000);
        for address in [CONVERT_ROLL, CONVERT_LOOK_AT, CONVERT_PATH] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let unrelated = e.mem.alloc(8);
        let node = object(&mut e, 0x0347_0000, AV_OBJECT_RTTI, 1);
        let child = object(&mut e, 0x0347_0010, AV_OBJECT_RTTI, 0);
        let roll = object(&mut e, 0x0347_0020, ROLL_CONTROLLER_RTTI, 0);
        let look_at = object(&mut e, 0x0347_0030, LOOK_AT_CONTROLLER_RTTI, 0);
        let path = object(&mut e, 0x0347_0040, PATH_CONTROLLER_RTTI, 0);
        let other = object(&mut e, 0x0347_0050, unrelated, 0);
        e.mem.set_u32(node + 0xc, roll);
        e.mem.set_u32(roll + 0x30, other);
        e.mem.set_u32(other + 0x30, look_at);
        e.mem.set_u32(child + 0xc, path);
        let children = e.mem.alloc(4);
        e.mem.set_u32(children, child);
        e.mem.set_u32(node + 0xa0, children);
        e.mem.set_u16(node + 0xa6, 1);
        e.call(0x00a4_2b70, &args![0x10u32, node]);
        let log = e.call_log.take().unwrap();
        let converted: Vec<(u32, u32)> = log
            .iter()
            .filter(|c| [CONVERT_ROLL, CONVERT_LOOK_AT, CONVERT_PATH].contains(&c.0))
            .map(|c| (c.0, c.1[0]))
            .collect();
        assert_eq!(
            converted,
            vec![
                (CONVERT_ROLL, roll),
                (CONVERT_LOOK_AT, look_at),
                (CONVERT_PATH, path)
            ]
        );
        // Not an NiAVObject, or null: nothing.
        let plain = object(&mut e, 0x0347_0060, unrelated, 1);
        e.mem.set_u32(plain + 0xc, roll);
        e.call_log = Some(vec![]);
        e.call(0x00a4_2b70, &args![0x10u32, plain]);
        e.call(0x00a4_2b70, &args![0x10u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == CONVERT_ROLL));
    }
}
