//! `(unplaced)/run_005f8bf0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Callees of `fn_005f8bf0`: `0047cdb0` and `0047ce10` (`thiscall` on `this + 0x30`, no
/// arguments / `(reference)`), `0056afc0` (`thiscall` on the reference, no arguments),
/// `007af430` (`thiscall` on the reference; the map's name `BGSSaveFormBuffer::GetForm`
/// is a folded one) and `TESBoundObject::Clone3D` (`thiscall`, `(reference, 0)`).
const MEMBER_TEST_0047CDB0: u32 = 0x0047_cdb0;
const MEMBER_ADD_0047CE10: u32 = 0x0047_ce10;
const REFERENCE_TEST_0056AFC0: u32 = 0x0056_afc0;
const REFERENCE_GET_007AF430: u32 = 0x007a_f430;
const BOUND_OBJECT_CLONE_3D: u32 = 0x0050_ee50;
/// Callees of `fn_005f8c70`: the list-node walk (`008256d0` "stop here", `006815c0` the
/// item, `00726070` the next node; `thiscall` on the node), the C-string copy `00406d30`
/// (`cdecl`, `(destination, size, source)`), the last-character search `0040ab30` (`cdecl`,
/// `(text, character)`), `0045a5e0` (`thiscall` on the object at `011c3b3c`, `(text)`) and
/// `0050fbf0` (`thiscall`, `(argument)`).
const NODE_STOP_008256D0: u32 = 0x0082_56d0;
const NODE_ITEM_006815C0: u32 = 0x0068_15c0;
const NODE_FOLLOWING_00726070: u32 = 0x0072_6070;
const STRING_COPY_00406D30: u32 = 0x0040_6d30;
const STRING_FIND_LAST_0040AB30: u32 = 0x0040_ab30;
const PATH_SINK_OBJECT: u32 = 0x011c_3b3c;
const PATH_SINK_0045A5E0: u32 = 0x0045_a5e0;
const BASE_0050FBF0: u32 = 0x0050_fbf0;

// Translated from 005f8bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(reference)`): when `0047cdb0` on `this + 0x30` answers
/// true, the reference is not null and `0056afc0` on it answers false, the reference goes to
/// `0047ce10` on `this + 0x30` and the object `007af430` answers for it gets its virtual
/// slot +0x178 with the reference. Otherwise `TESBoundObject::Clone3D(reference, 0)`.
/// Answers the `EAX` of whichever call ran last: the exe ends on it. The function is slot
/// `+0x178` of `TESCreature`'s vtable, the `TESBoundObject` slot that the Xbox PDB names
/// `Clone3D` (`NiAVObject*`; the engine map gives `TESObjectLIGH::Clone3D` for a sibling).
pub fn fn_005f8bf0(e: &mut Engine, this: Ptr, reference: Ptr) -> u32 {
    let member = this.addr().wrapping_add(0x30);
    if e.call(MEMBER_TEST_0047CDB0, &args![member]).bool()
        && !reference.is_null()
        && !e.call(REFERENCE_TEST_0056AFC0, &args![reference]).bool()
    {
        e.call(MEMBER_ADD_0047CE10, &args![member, reference]);
        let object = e.call(REFERENCE_GET_007AF430, &args![reference]).u32();
        return e.vcall(object, 0x178, &args![reference]).u32();
    }
    e.call(BOUND_OBJECT_CLONE_3D, &args![this, reference, 0u32])
        .u32()
}

// Translated from 005f8c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(argument)`): walks the list whose first node is
/// embedded at `this + 0x118` until a null node or one `008256d0` stops at; for each node a
/// path is built in a 0x104-character local: the name the object at `this + 0xDC` answers
/// (its virtual slot +0x14) copied in (`00406d30`), the text after its last `\` replaced by
/// the node's item's first word as a string (`006815c0`, `00406d30` with the room left), and
/// the path given to `0045a5e0` on the object at `011c3b3c`. Then `0050fbf0(argument)`.
/// Without a `\` the copy goes to address 1, as the exe's arithmetic gives.
// Translation incomplete: 005f8c70 - the /GS stack cookie check (00ec408c) is not modelled.
pub fn fn_005f8c70(e: &mut Engine, this: Ptr, argument: u32) {
    let mut node = this.addr().wrapping_add(0x118);
    e.with_stack(0x10c, |e, buffer| {
        let buffer = buffer.addr();
        while node != 0 {
            if e.call(NODE_STOP_008256D0, &args![node]).bool() {
                break;
            }
            let named = this.addr().wrapping_add(0xdc);
            let name = e.vcall(named, 0x14, &args![]).u32();
            e.call(STRING_COPY_00406D30, &args![buffer, 0x104u32, name]);
            let slash = e
                .call(STRING_FIND_LAST_0040AB30, &args![buffer, 0x5cu32])
                .u32();
            let item = e.call(NODE_ITEM_006815C0, &args![node]).u32();
            let text = e.mem.u32(item);
            let room = 0x104u32
                .wrapping_sub(slash.wrapping_sub(buffer))
                .wrapping_sub(1);
            e.call(
                STRING_COPY_00406D30,
                &args![slash.wrapping_add(1), room, text],
            );
            let sink = e.global::<u32>(PATH_SINK_OBJECT);
            e.call(PATH_SINK_0045A5E0, &args![sink, buffer]);
            node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
        }
    });
    e.call(BASE_0050FBF0, &args![this, argument]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f8bf0, fn_005f8bf0(Ptr, Ptr) -> u32),
        entry!(0x005f8c70, fn_005f8c70(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_005f8bf0_adds_the_reference_or_clones_3d() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MEMBER_TEST_0047CDB0, |_, _| answer(1));
        e.register(REFERENCE_TEST_0056AFC0, |_, a| {
            answer(u32::from(a[0] == 0x7000))
        });
        e.register(MEMBER_ADD_0047CE10, |_, _| Ret::default());
        e.register(REFERENCE_GET_007AF430, |_, _| answer(0x8000));
        e.register(BOUND_OBJECT_CLONE_3D, |_, _| Ret::default());
        e.register(0x0090_0178, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x5f];
        slots[0x5e] = 0x0090_0178;
        e.put_vtable(0x0090_1000, &slots);
        e.map(0x8000, 4);
        e.mem.set_u32(0x8000, 0x0090_1000);
        e.call(0x005f8bf0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls_at(&e, MEMBER_ADD_0047CE10),
            vec![vec![0x5030, 0x6000]]
        );
        assert_eq!(calls_at(&e, 0x0090_0178), vec![vec![0x8000, 0x6000]]);
        assert!(calls_at(&e, BOUND_OBJECT_CLONE_3D).is_empty());
        // The reference's own test answering true, a null reference, or the member test
        // answering false: Clone3D.
        e.call(0x005f8bf0, &args![0x5000u32, 0x7000u32]);
        e.call(0x005f8bf0, &args![0x5000u32, 0u32]);
        e.register(MEMBER_TEST_0047CDB0, |_, _| answer(0x100));
        e.call(0x005f8bf0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls_at(&e, BOUND_OBJECT_CLONE_3D),
            vec![
                vec![0x5000, 0x7000, 0],
                vec![0x5000, 0, 0],
                vec![0x5000, 0x6000, 0]
            ]
        );
    }

    #[test]
    fn fn_005f8c70_builds_a_path_per_node() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x120);
        // The named object at +0xDC answers the text at 0x9000.
        e.put_vtable(0x0090_1000, &[0, 0, 0, 0, 0, 0x0090_0014]);
        e.mem.set_u32(this + 0xdc, 0x0090_1000);
        e.register(0x0090_0014, |_, _| answer(0x9000));
        e.map(0x9000, 0x100);
        e.mem.set_cstr(0x9000, b"meshes\\a\\old.nif");
        e.mem.set_cstr(0x9080, b"new.nif");
        e.mem.set_u32(0x90f0, 0x9080);
        // Two nodes: the embedded one, then 0x9100 where the walk stops.
        e.register(NODE_STOP_008256D0, |_, a| answer(u32::from(a[0] == 0x9100)));
        e.register(NODE_ITEM_006815C0, |_, _| answer(0x90f0));
        e.register(NODE_FOLLOWING_00726070, |_, _| answer(0x9100));
        e.register(STRING_COPY_00406D30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_FIND_LAST_0040AB30, |e, a| {
            let text = e.mem.cstr(a[0]);
            let at = text.iter().rposition(|&c| u32::from(c) == a[1]).unwrap();
            answer(a[0] + at as u32)
        });
        e.map(PATH_SINK_OBJECT, 4);
        e.set_global::<u32>(PATH_SINK_OBJECT, 0x4444);
        e.register(PATH_SINK_0045A5E0, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"meshes\\a\\new.nif".to_vec());
            Ret::default()
        });
        e.register(BASE_0050FBF0, |_, _| Ret::default());
        e.call(0x005f8c70, &args![this, 0x77u32]);
        let sinks = calls_at(&e, PATH_SINK_0045A5E0);
        assert_eq!(sinks.len(), 1);
        assert_eq!(sinks[0][0], 0x4444);
        let copies = calls_at(&e, STRING_COPY_00406D30);
        let buffer = copies[0][0];
        assert_eq!(copies[0][1], 0x104);
        assert_eq!(copies[1], vec![buffer + 9, 0x104 - 8 - 1, 0x9080]);
        assert_eq!(calls_at(&e, BASE_0050FBF0), vec![vec![this, 0x77]]);
    }
}

#[cfg(test)]
mod dropret_tests {
    use super::*;

    #[test]
    fn fn_005f8bf0_answers_the_clone_or_the_slot_0x178_object() {
        let mut e = Engine::new();
        e.register(MEMBER_TEST_0047CDB0, |_, _| 1u32.into_ret());
        e.register(REFERENCE_TEST_0056AFC0, |_, a| {
            u32::from(a[0] == 0x7000).into_ret()
        });
        e.register(MEMBER_ADD_0047CE10, |_, _| Ret::default());
        e.register(REFERENCE_GET_007AF430, |_, _| 0x8000u32.into_ret());
        e.register(BOUND_OBJECT_CLONE_3D, |_, _| 0x00aa_0000u32.into_ret());
        e.register(0x0090_0178, |_, _| 0x00bb_0000u32.into_ret());
        let mut slots = vec![0u32; 0x5f];
        slots[0x5e] = 0x0090_0178;
        e.put_vtable(0x0090_1000, &slots);
        e.map(0x8000, 4);
        e.mem.set_u32(0x8000, 0x0090_1000);
        assert_eq!(
            e.call(0x005f8bf0, &args![0x5000u32, 0x6000u32]).u32(),
            0x00bb_0000
        );
        assert_eq!(
            e.call(0x005f8bf0, &args![0x5000u32, 0x7000u32]).u32(),
            0x00aa_0000
        );
    }
}
