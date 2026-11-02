//! `BSHavok/bhkunaryaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkUnaryAction` (vtable `0102009c`, RTTI-confirmed) is a `bhkAction`: its Havok action is
//! at `+8` and keeps its target entity at `+0x18`; that entity's `bhk` object is at `+0xC`.
//! The construction info (`cinfo`, virtual slot `0xB4` fills a flag byte and answers it)
//! keeps the target entity at `+4`.
//!
//! The functions that run in the Havok allocation scope raise the per-thread allocation word at
//! `TLS + 0x2b4` to 0x10 for their duration. The exception-unwinding frames (`FS:[0]`) and
//! `/GS` cookie checks of the functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{add_viewer_string, enter_scope, leave_scope};

/// `bhkAction` methods: destructor body (`ECX`), `CopyCinfo` `(cinfo)` (`00c879e0`) and
/// `GetViewerStrings` `(list)`.
const ACTION_DESTRUCT: u32 = 0x00c8_7970;
const ACTION_COPY_CINFO: u32 = 0x00c8_79e0;
const ACTION_GET_VIEWER_STRINGS: u32 = 0x00c8_7be0;
/// `bhkSerializable` methods: `Kill`, `CopyMembers` `(source, process)`, `LoadBinary`,
/// `LinkObject`, `SaveBinary`.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const SERIALIZABLE_COPY_MEMBERS: u32 = 0x00c8_7600;
const SERIALIZABLE_LOAD_BINARY: u32 = 0x00c8_7650;
const SERIALIZABLE_LINK_OBJECT: u32 = 0x00c8_7670;
const SERIALIZABLE_SAVE_BINARY: u32 = 0x00c8_7850;
/// `NiStream` methods (`this` is the stream): `00a645b0` (unnamed, no arguments, result unused
/// by `LoadBinary`) and `GetObjectFromLinkID` (`00a63430`).
const STREAM_UNNAMED_READ: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// The map lookup `CopyMembers` finds the copy of the target with (`00a53c30`, `ECX` = the
/// map, `(key, out)` with the found value stored at `out`; answers whether it was found).
const MAP_LOOKUP: u32 = 0x00a5_3c30;
/// The jump thunk at `00ba87d0` (to `00a5d470`) `IsEqual` starts with: `(this, other)`.
const BASE_IS_EQUAL: u32 = 0x00ba_87d0;
/// `bhkWorld::Add_ov3` (Xbox PDB, `00c6b3c0`, `ECX` = the world, one word).
const WORLD_ADD: u32 = 0x00c6_b3c0;
/// `hkOstream` (Xbox PDB name of `00cc0500`: `ECX` = the stream object, `(buffer, size,
/// flag)`), its append-text method (`00cc01c0`, one word) and destructor (`00cc00e0`), and
/// the global error handler object at `01268360` whose virtual slot `0xC` takes `(1, id,
/// text, file, line)`.
const OSTREAM_CONSTRUCT: u32 = 0x00cc_0500;
const OSTREAM_APPEND: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
const ERROR_HANDLER: u32 = 0x0126_8360;
/// The message text, the source file name and the error id of the warning `ProcessEntity` logs
/// (the line is 200).
const WARNING_TEXT: u32 = 0x010c_5aa0;
const WARNING_FILE: u32 = 0x010c_5a5c;
const WARNING_ID: u32 = 0x234f_24fc;
/// `NiGetViewerString` for a string (`00a7d5e0`, cdecl `(name, text)`) and `NiGetViewerString`
/// for a float-like word (`00a7d330`, cdecl `(text)`); `sprintf_s` (`00406d00`, cdecl).
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const VIEWER_STRING: u32 = 0x00a7_d330;
const FORMAT_TEXT: u32 = 0x0040_6d00;
/// The vtable of `bhkUnaryAction` and its live-object count.
const VTABLE_UNARY_ACTION: u32 = 0x0102_009c;
const COUNT_UNARY_ACTION: u32 = 0x0126_8118;
/// The global holding the text `GetViewerStrings` starts with (read as a word), the format
/// of the target address and the name of its viewer string.
const CLASS_TEXT: u32 = 0x0126_811c;
const POINTER_FORMAT: u32 = 0x010c_54b0;
const TARGET_NAME: u32 = 0x010c_5ae8;

/// The construction info of `this`: virtual slot `0xB4` fills a flag byte and answers it.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32()
}

/// The Havok action's target entity: `this`, its Havok action (`+8`), that action's `+0x18`;
/// 0 when `this` or the Havok action is missing.
fn target_entity(e: &Engine, this: Ptr) -> u32 {
    if this.is_null() {
        return 0;
    }
    let action = e.mem.u32(this.addr() + 8);
    if action == 0 {
        return 0;
    }
    e.mem.u32(action + 0x18)
}

// Translated from 00c86720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c86720 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkUnaryAction::~bhkUnaryAction` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill`, decrements the live-object count at `01268118` and runs the
/// `bhkAction` destructor body (`00c87970`).
pub fn bhk_unary_action_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_UNARY_ACTION);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_UNARY_ACTION);
    e.set_global(COUNT_UNARY_ACTION, count.wrapping_sub(1));
    e.call(ACTION_DESTRUCT, &args![this]);
}

// Translated from 00c86790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUnaryAction::LoadBinary` (Xbox PDB): calls the stream's unnamed method `00a645b0`
/// (result unused), runs `bhkSerializable::LoadBinary`, takes the cinfo and clears its `+4`.
pub fn bhk_unary_action_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(STREAM_UNNAMED_READ, &args![stream]);
    e.call(SERIALIZABLE_LOAD_BINARY, &args![this, stream]);
    let cinfo = cinfo_of(e, this);
    e.mem.set_u32(cinfo + 4, 0);
}

// Translated from 00c867d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the construction info (unnamed in the engine map), `(this, cinfo)`: `bhkAction`'s
/// `00c879e0` with the cinfo, then the Havok action's target entity (0 without a Havok
/// action) into the cinfo's `+4`.
pub fn fn_00c867d0(e: &mut Engine, this: Ptr, cinfo: u32) {
    e.call(ACTION_COPY_CINFO, &args![this, cinfo]);
    let entity = target_entity(e, this);
    e.mem.set_u32(cinfo + 4, entity);
}

// Translated from 00c86800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUnaryAction::CopyMembers` (Xbox PDB), `(this, source, process)`: takes the cinfo
/// (virtual slot `0xB4`) and its target entity's `bhk` object (`+0xC` of the entity at the
/// cinfo's `+4`, 0 without one), clears the cinfo's `+4`, and when there is such an object
/// looks it up in the map at the first word of `process` (`00a53c30`, with an out word that
/// holds `process` until the lookup writes it); when found, the found object's Havok object
/// (`+8`, 0 when the found object is null) becomes the cinfo's target. Then
/// `bhkSerializable::CopyMembers` with `(source, process)`.
pub fn bhk_unary_action_copy_members(e: &mut Engine, this: Ptr, source: u32, process: u32) {
    let cinfo = cinfo_of(e, this);
    let entity = e.mem.u32(cinfo + 4);
    let object = if entity != 0 {
        e.mem.u32(entity + 0xc)
    } else {
        0
    };
    e.mem.set_u32(cinfo + 4, 0);
    if object != 0 {
        let map = e.mem.u32(process);
        let found = e.with_stack(4, |e, out| {
            e.mem.set_u32(out.addr(), process);
            let hit = e.call(MAP_LOOKUP, &args![map, object, out]).bool();
            (hit, e.mem.u32(out.addr()))
        });
        if found.0 {
            let target = if found.1 != 0 {
                e.mem.u32(found.1 + 8)
            } else {
                0
            };
            e.mem.set_u32(cinfo + 4, target);
        }
    }
    e.call(SERIALIZABLE_COPY_MEMBERS, &args![this, source, process]);
}

// Translated from 00c86870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUnaryAction::LinkObject` (Xbox PDB): takes the cinfo, the next linked object of the
/// stream (`00a63430`) and, when there is one, stores its Havok object (`+8`) at the cinfo's
/// `+4` (the cinfo keeps its value otherwise); then `bhkSerializable::LinkObject`.
pub fn bhk_unary_action_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = cinfo_of(e, this);
    let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    if linked != 0 {
        let object = e.mem.u32(linked + 8);
        e.mem.set_u32(cinfo + 4, object);
    }
    e.call(SERIALIZABLE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00c868b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUnaryAction::SaveBinary` (Xbox PDB): hands the target entity's `bhk` object (`+0xC` of
/// the Havok action's target, 0 without one) to the stream's virtual slot `0x30`, then runs
/// `bhkSerializable::SaveBinary`.
pub fn bhk_unary_action_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let entity = target_entity(e, this);
    let object = if entity != 0 {
        e.mem.u32(entity + 0xc)
    } else {
        0
    };
    e.vcall(stream, 0x30, &args![object]);
    e.call(SERIALIZABLE_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00c868f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUnaryAction::IsEqual` (Xbox PDB): false when the base comparison (`00ba87d0`) says no;
/// otherwise whether the two actions' target entities (the Havok action's `+0x18`, 0 when
/// the action or its Havok action is missing) are the same.
pub fn bhk_unary_action_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    target_entity(e, this) == target_entity(e, other)
}

// Translated from 00c86940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c86940 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled, and the three upper bytes of the dword holding the stream's flag byte are stale register contents (taken as 0).
/// `bhkUnaryAction::ProcessEntity` (Xbox PDB), `(this, world)` (the world is the `bhkWorld`
/// `Add` passes; unconfirmed here): in the Havok allocation scope, when the Havok action has a
/// target entity with a `bhk` object `target` (the entity's `+0xC`) whose virtual slot `0x94`
/// answers 0 and the world's `+0x18` is 0: calls `target`'s virtual slot `0x9c` with the
/// world, and, when `target`'s flag byte `+0x10` has neither bit 4 nor bit 5, logs a warning
/// and calls `target`'s virtual slot `0xC8` with the world. The warning ("A unary action is
/// being added to the world before its target entities.\n") is written to an `hkOstream`
/// (`00cc0500`) of 0x200 bytes on the stack and handed to the error handler object at
/// `01268360` (virtual slot `0xC`, `(1, 0x234f24fc, text, file name, 200)`).
pub fn bhk_unary_action_process_entity(e: &mut Engine, this: Ptr, world: u32) {
    let old = enter_scope(e);
    let entity = target_entity(e, this);
    if entity != 0 {
        let target = e.mem.u32(entity + 0xc);
        if target != 0 {
            let answer = e.vcall(target, 0x94, &args![]).u32();
            if answer == 0 && e.mem.u32(world + 0x18) == 0 {
                e.vcall(target, 0x9c, &args![world]);
                if e.mem.u8(target + 0x10) & 0x30 == 0 {
                    e.with_stack(0x210, |e, memory| {
                        let stream = memory.addr();
                        let buffer = stream + 0x10;
                        e.call(OSTREAM_CONSTRUCT, &args![stream, buffer, 0x200u32, 1u32]);
                        e.call(OSTREAM_APPEND, &args![stream, WARNING_TEXT]);
                        let handler = e.global::<u32>(ERROR_HANDLER);
                        e.vcall(
                            handler,
                            0xc,
                            &args![1u32, WARNING_ID, buffer, WARNING_FILE, 200u32],
                        );
                        e.call(OSTREAM_DESTRUCT, &args![stream]);
                    });
                    e.vcall(target, 0xc8, &args![world]);
                }
            }
        }
    }
    leave_scope(e, old);
}

// Translated from 00c86ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c86ab0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkUnaryAction::Add` (Xbox PDB), `(this, world)` (a `bhkWorld`, from the callee
/// `00c6b3c0`): in the Havok allocation scope, when `this` has a Havok action (`+8`): with no
/// world calls virtual slot `0xA0` and answers false; otherwise, unless virtual slot `0x94`
/// already answers the world's `+8`, calls virtual slot `0xA0`, `ProcessEntity` (`00c86940`)
/// with the world and the world's `Add_ov3` (`00c6b3c0`) with the Havok action, and answers
/// true.
pub fn bhk_unary_action_add(e: &mut Engine, this: Ptr, world: Ptr) -> bool {
    let old = enter_scope(e);
    let action = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    let mut added = false;
    if action != 0 {
        if world.is_null() {
            e.vcall(this.addr(), 0xa0, &args![]);
        } else {
            let current = e.vcall(this.addr(), 0x94, &args![]).u32();
            if current != e.mem.u32(world.addr() + 8) {
                e.vcall(this.addr(), 0xa0, &args![]);
                bhk_unary_action_process_entity(e, this, world.addr());
                e.call(WORLD_ADD, &args![world, action]);
                added = true;
            }
        }
    }
    leave_scope(e, old);
    added
}

// Translated from 00c86b80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c86b80 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkUnaryAction::GetViewerStrings` (Xbox PDB): in the Havok allocation scope runs
/// `bhkAction::GetViewerStrings` (`00c87be0`), appends the viewer string of the text in the
/// global at `0126811c`, then the target entity's address (0 without one) formatted by
/// `sprintf_s` with the format at `010c54b0` into a 0x20-byte buffer, as a viewer string
/// under the name at `010c5ae8`.
pub fn bhk_unary_action_get_viewer_strings(e: &mut Engine, this: Ptr, list: u32) {
    let old = enter_scope(e);
    e.call(ACTION_GET_VIEWER_STRINGS, &args![this, list]);
    let class_text = e.global::<u32>(CLASS_TEXT);
    let text = e.call(VIEWER_STRING, &args![class_text]).u32();
    add_viewer_string(e, list, text);
    let entity = target_entity(e, this);
    let text = e.with_stack(0x20, |e, buffer| {
        e.call(FORMAT_TEXT, &args![buffer, 0x20u32, POINTER_FORMAT, entity]);
        e.call(VIEWER_STRING_TEXT, &args![TARGET_NAME, buffer])
            .u32()
    });
    add_viewer_string(e, list, text);
    leave_scope(e, old);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c86720, bhk_unary_action_destructor(Ptr)),
        entry!(0x00c86790, bhk_unary_action_load_binary(Ptr, u32)),
        entry!(0x00c867d0, fn_00c867d0(Ptr, u32)),
        entry!(0x00c86800, bhk_unary_action_copy_members(Ptr, u32, u32)),
        entry!(0x00c86870, bhk_unary_action_link_object(Ptr, u32)),
        entry!(0x00c868b0, bhk_unary_action_save_binary(Ptr, u32)),
        entry!(0x00c868f0, bhk_unary_action_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c86940, bhk_unary_action_process_entity(Ptr, u32)),
        entry!(0x00c86ab0, bhk_unary_action_add(Ptr, Ptr) -> bool),
        entry!(0x00c86b80, bhk_unary_action_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// An object whose vtable (a fresh block) has the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn scope(e: &mut Engine) -> u32 {
        let word = e.tls() + 0x2b4;
        e.mem.u32(word)
    }

    /// Starts the allocation scope word at a recognisable value, so its restore is visible.
    fn mark_scope(e: &mut Engine) {
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
    }

    /// An action whose slot `0xB4` answers its `+0xC` as the cinfo, with a Havok action whose
    /// target entity (`+0x18`) is `entity`. Returns `(this, cinfo, havok action)`.
    fn action(e: &mut Engine, entity: u32) -> (u32, u32, u32) {
        let this = object(e, &[(0xb4, 0x0900_0100)]);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        let cinfo = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, cinfo);
        let havok = e.mem.alloc(0x40);
        e.mem.set_u32(havok + 0x18, entity);
        e.mem.set_u32(this + 8, havok);
        (this, cinfo, havok)
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_UNARY_ACTION, 9u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, ACTION_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00c8_6720, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_UNARY_ACTION);
        assert_eq!(e.global::<u32>(COUNT_UNARY_ACTION), 8);
        assert_eq!(
            order(&e),
            vec![0x00c8_6720, SERIALIZABLE_KILL, ACTION_DESTRUCT]
        );
    }

    #[test]
    fn load_binary_clears_the_target() {
        let mut e = Engine::new();
        quiet(&mut e, &[STREAM_UNNAMED_READ, SERIALIZABLE_LOAD_BINARY]);
        let (this, cinfo, _) = action(&mut e, 0x5555);
        e.mem.set_u32(cinfo + 4, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x00c8_6790, &args![this, 0x3000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(
            order(&e)[..3],
            [0x00c8_6790, STREAM_UNNAMED_READ, SERIALIZABLE_LOAD_BINARY]
        );
        assert_eq!(calls_to(&e, STREAM_UNNAMED_READ), vec![vec![0x3000]]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_LOAD_BINARY),
            vec![vec![this, 0x3000]]
        );
    }

    #[test]
    fn the_cinfo_fill_takes_the_target_entity() {
        let mut e = Engine::new();
        quiet(&mut e, &[ACTION_COPY_CINFO]);
        let (this, _, havok) = action(&mut e, 0x5555);
        let cinfo = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00c8_67d0, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x5555);
        assert_eq!(calls_to(&e, ACTION_COPY_CINFO), vec![vec![this, cinfo]]);
        // No Havok action, or no action at all: 0.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_67d0, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        e.mem.set_u32(cinfo + 4, 9);
        e.mem.set_u32(this + 8, havok);
        e.call(0x00c8_67d0, &args![0u32, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
    }

    #[test]
    fn copy_members_looks_the_target_up_in_the_process_map() {
        let mut e = Engine::new();
        quiet(&mut e, &[SERIALIZABLE_COPY_MEMBERS]);
        let (this, cinfo, _) = action(&mut e, 0);
        let entity = e.mem.alloc(0x20);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 0xc, object);
        e.mem.set_u32(cinfo + 4, entity);
        let found = e.mem.alloc(0x20);
        e.mem.set_u32(found + 8, 0xfa11);
        // The lookup writes the found object into the out word.
        e.register_double(MAP_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], found);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let process = e.mem.alloc(0x20);
        e.mem.set_u32(process, 0x3333);
        e.call_log = Some(vec![]);
        e.call(0x00c8_6800, &args![this, 0x9999u32, process]);
        let lookups = calls_to(&e, MAP_LOOKUP);
        assert_eq!(lookups.len(), 1);
        assert_eq!(lookups[0][..2], [0x3333, object]);
        assert_eq!(e.mem.u32(cinfo + 4), 0xfa11);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_COPY_MEMBERS),
            vec![vec![this, 0x9999, process]]
        );
        // Not found: the target stays cleared.
        e.register(MAP_LOOKUP, |_, _| Ret::default());
        e.mem.set_u32(cinfo + 4, entity);
        e.call(0x00c8_6800, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        // Found with the out word left as it was: the process word is the "object" read.
        e.register(MAP_LOOKUP, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let trailer = e.mem.alloc(0x20);
        e.mem.set_u32(trailer + 8, 0x7a11);
        let process = trailer;
        e.mem.set_u32(process, 0x3333);
        e.mem.set_u32(cinfo + 4, entity);
        e.call(0x00c8_6800, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7a11);
        // No entity object: no lookup.
        e.mem.set_u32(cinfo + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_6800, &args![this, 0x9999u32, process]);
        assert!(calls_to(&e, MAP_LOOKUP).is_empty());
        assert_eq!(calls_to(&e, SERIALIZABLE_COPY_MEMBERS).len(), 1);
    }

    #[test]
    fn link_object_keeps_the_target_without_a_linked_object() {
        let mut e = Engine::new();
        quiet(&mut e, &[SERIALIZABLE_LINK_OBJECT]);
        let (this, cinfo, _) = action(&mut e, 0);
        e.mem.set_u32(cinfo + 4, 0x1234);
        let linked = e.mem.alloc(0x20);
        e.mem.set_u32(linked + 8, 0xbee);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, {
            let mut answers = vec![0, linked];
            move |_, _| Ret {
                eax: answers.pop().unwrap(),
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00c8_6870, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0xbee);
        e.call(0x00c8_6870, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0xbee);
        assert_eq!(
            calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID),
            vec![vec![0x5000]; 2]
        );
        assert_eq!(
            calls_to(&e, SERIALIZABLE_LINK_OBJECT),
            vec![vec![this, 0x5000]; 2]
        );
    }

    #[test]
    fn save_binary_hands_the_target_object_to_the_stream() {
        let mut e = Engine::new();
        quiet(&mut e, &[SERIALIZABLE_SAVE_BINARY, 0x0900_0150]);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 0xc, 0xc0de);
        let (this, _, havok) = action(&mut e, entity);
        let stream = object(&mut e, &[(0x30, 0x0900_0150)]);
        e.call_log = Some(vec![]);
        e.call(0x00c8_68b0, &args![this, stream]);
        assert_eq!(
            order(&e),
            vec![0x00c8_68b0, 0x0900_0150, SERIALIZABLE_SAVE_BINARY]
        );
        assert_eq!(calls_to(&e, 0x0900_0150), vec![vec![stream, 0xc0de]]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        // No entity, no Havok action: the stream gets 0.
        e.mem.set_u32(havok + 0x18, 0);
        e.call(0x00c8_68b0, &args![this, stream]);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_68b0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, 0x0900_0150),
            vec![vec![stream, 0xc0de], vec![stream, 0], vec![stream, 0]]
        );
    }

    #[test]
    fn is_equal_compares_the_target_entities_after_the_base() {
        let mut e = Engine::new();
        e.register(BASE_IS_EQUAL, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x50),
            ..Ret::default()
        });
        let (first, _, _) = action(&mut e, 0x5555);
        let (second, _, _) = action(&mut e, 0x5555);
        let (third, _, _) = action(&mut e, 0x6666);
        for object in [first, second, third] {
            e.mem.set_u32(object + 0x50, 1);
        }
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c8_68f0, &args![first, second]).bool());
        assert!(!e.call(0x00c8_68f0, &args![first, third]).bool());
        // The base says no: no further look.
        e.mem.set_u32(second + 0x50, 0);
        assert!(!e.call(0x00c8_68f0, &args![first, second]).bool());
        e.mem.set_u32(second + 0x50, 1);
        // A missing Havok action reads as no target.
        let (bare, _, _) = action(&mut e, 0);
        e.mem.set_u32(bare + 0x50, 1);
        assert!(e.call(0x00c8_68f0, &args![bare, bare]).bool());
        e.mem.set_u32(bare + 8, 0);
        assert!(!e.call(0x00c8_68f0, &args![first, bare]).bool());
        assert!(e.call(0x00c8_68f0, &args![bare, bare]).bool());
        assert_eq!(calls_to(&e, BASE_IS_EQUAL).len(), 6);
    }

    /// A target `bhk` object: slot `0x94` answers its `+0x60`, slots `0x9c` and `0xc8` record,
    /// flag byte at `+0x10`.
    fn target(e: &mut Engine) -> u32 {
        let target = object(
            e,
            &[
                (0x94, 0x0900_0200),
                (0x9c, 0x0900_0201),
                (0xc8, 0x0900_0202),
            ],
        );
        e.register(0x0900_0200, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x60),
            ..Ret::default()
        });
        quiet(e, &[0x0900_0201, 0x0900_0202]);
        target
    }

    #[test]
    fn process_entity_warns_when_the_world_is_new_and_the_target_is_not_flagged() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        let handler = object(&mut e, &[(0xc, 0x0900_0210)]);
        e.set_global(ERROR_HANDLER, handler);
        quiet(
            &mut e,
            &[
                0x0900_0210,
                OSTREAM_CONSTRUCT,
                OSTREAM_APPEND,
                OSTREAM_DESTRUCT,
            ],
        );
        let target = target(&mut e);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 0xc, target);
        let (this, _, _) = action(&mut e, entity);
        let world = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00c8_6940, &args![this, world]);
        assert_eq!(
            order(&e),
            vec![
                0x00c8_6940,
                0x0900_0200,
                0x0900_0201,
                OSTREAM_CONSTRUCT,
                OSTREAM_APPEND,
                0x0900_0210,
                OSTREAM_DESTRUCT,
                0x0900_0202
            ]
        );
        assert_eq!(calls_to(&e, 0x0900_0201), vec![vec![target, world]]);
        assert_eq!(calls_to(&e, 0x0900_0202), vec![vec![target, world]]);
        let construct = calls_to(&e, OSTREAM_CONSTRUCT);
        let stream = construct[0][0];
        let buffer = construct[0][1];
        assert_eq!(construct, vec![vec![stream, buffer, 0x200, 1]]);
        assert_eq!(buffer, stream + 0x10);
        assert_eq!(
            calls_to(&e, OSTREAM_APPEND),
            vec![vec![stream, WARNING_TEXT]]
        );
        assert_eq!(
            calls_to(&e, 0x0900_0210),
            vec![vec![handler, 1, WARNING_ID, buffer, WARNING_FILE, 200]]
        );
        assert_eq!(calls_to(&e, OSTREAM_DESTRUCT), vec![vec![stream]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn process_entity_stays_quiet_for_flagged_busy_or_missing_targets() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        quiet(
            &mut e,
            &[OSTREAM_CONSTRUCT, OSTREAM_APPEND, OSTREAM_DESTRUCT],
        );
        let target = target(&mut e);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 0xc, target);
        let (this, _, havok) = action(&mut e, entity);
        let world = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        // A flagged target (bit 4 or 5): slot 0x9c runs, nothing else.
        for flag in [0x10u8, 0x20, 0x30] {
            e.mem.set_u8(target + 0x10, flag);
            e.call(0x00c8_6940, &args![this, world]);
        }
        assert_eq!(calls_to(&e, 0x0900_0201).len(), 3);
        assert!(calls_to(&e, 0x0900_0202).is_empty());
        assert!(calls_to(&e, OSTREAM_CONSTRUCT).is_empty());
        // Another flag bit alone does not silence it (the handler global is 0 here).
        // A busy target (slot 0x94 non-zero) or a world with a non-zero +0x18: nothing.
        e.mem.set_u8(target + 0x10, 0);
        e.mem.set_u32(target + 0x60, 1);
        e.call(0x00c8_6940, &args![this, world]);
        e.mem.set_u32(target + 0x60, 0);
        e.mem.set_u32(world + 0x18, 2);
        e.call(0x00c8_6940, &args![this, world]);
        assert_eq!(calls_to(&e, 0x0900_0201).len(), 3);
        // No target object, no entity, no Havok action, no action: nothing, scope restored.
        e.mem.set_u32(entity + 0xc, 0);
        e.call(0x00c8_6940, &args![this, world]);
        e.mem.set_u32(havok + 0x18, 0);
        e.call(0x00c8_6940, &args![this, world]);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_6940, &args![this, world]);
        e.call(0x00c8_6940, &args![0u32, world]);
        assert_eq!(calls_to(&e, 0x0900_0200).len(), 5);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn add_processes_the_entity_and_adds_to_the_world() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        let this = object(
            &mut e,
            &[
                (0x94, 0x0900_0300),
                (0xa0, 0x0900_0301),
                (0xb4, 0x0900_0100),
            ],
        );
        // Slot 0x94 answers the world it is in (+0x60).
        e.register(0x0900_0300, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x60),
            ..Ret::default()
        });
        quiet(&mut e, &[0x0900_0301, WORLD_ADD]);
        let havok = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, havok);
        let world = e.mem.alloc(0x40);
        e.mem.set_u32(world + 8, 0xa11);
        e.mem.set_u32(this + 0x60, 0xb22);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c8_6ab0, &args![this, world]).bool());
        assert_eq!(
            order(&e),
            vec![0x00c8_6ab0, 0x0900_0300, 0x0900_0301, WORLD_ADD]
        );
        assert_eq!(calls_to(&e, WORLD_ADD), vec![vec![world, havok]]);
        assert_eq!(scope(&mut e), 0x77);
        // Already in that world: nothing but the query.
        e.mem.set_u32(this + 0x60, 0xa11);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c8_6ab0, &args![this, world]).bool());
        assert_eq!(order(&e), vec![0x00c8_6ab0, 0x0900_0300]);
        // No world: only slot 0xA0.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c8_6ab0, &args![this, 0u32]).bool());
        assert_eq!(order(&e), vec![0x00c8_6ab0, 0x0900_0301]);
        // No Havok action: nothing at all.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c8_6ab0, &args![this, world]).bool());
        assert_eq!(order(&e), vec![0x00c8_6ab0]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_text_and_the_target_address() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        e.set_global(CLASS_TEXT, 0x1111u32);
        quiet(
            &mut e,
            &[ACTION_GET_VIEWER_STRINGS, 0x0096_ad30, FORMAT_TEXT],
        );
        e.register(VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(VIEWER_STRING_TEXT, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        // The list stores each added text at its index.
        e.register(0x0096_ae90, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], text);
            let count = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, count + 1);
            Ret::default()
        });
        let (this, _, _) = action(&mut e, 0x5555);
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00c8_6b80, &args![this, list]);
        assert_eq!(e.mem.u32(list + 0x100), 0x1112);
        let formats = calls_to(&e, FORMAT_TEXT);
        let buffer = formats[0][0];
        assert_eq!(formats, vec![vec![buffer, 0x20, POINTER_FORMAT, 0x5555]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_TEXT),
            vec![vec![TARGET_NAME, buffer]]
        );
        assert_eq!(e.mem.u32(list + 0x104), buffer);
        assert_eq!(
            calls_to(&e, ACTION_GET_VIEWER_STRINGS),
            vec![vec![this, list]]
        );
        assert_eq!(scope(&mut e), 0x77);
        // No Havok action: the address formatted is 0.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_6b80, &args![this, list]);
        assert_eq!(calls_to(&e, FORMAT_TEXT)[0][3], 0);
    }
}
