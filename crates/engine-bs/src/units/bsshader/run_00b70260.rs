//! `BSShader/run_00b70260` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00ba92c0` (unit `bsshaderproperty.cpp`, `thiscall`, one word: the
/// stream): the shader property's `LoadBinary` body.
const SHADER_PROPERTY_LOAD_BINARY: u32 = 0x00ba_92c0;
/// `00ba8aa0` (unnamed, `thiscall`, one word) and `00ba8b10` (unit
/// `bsshaderproperty.cpp`, `thiscall`, two words): the shader-property steps
/// `00b70290` and `00b702b0` run before their virtual call.
const SHADER_PROPERTY_STEP_00BA8AA0: u32 = 0x00ba_8aa0;
const SHADER_PROPERTY_STEP_00BA8B10: u32 = 0x00ba_8b10;

// Translated from 00b70260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00ba92c0`, passing on `this`
/// and its one word (the stream).
pub fn fn_00b70260(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SHADER_PROPERTY_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00b70290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): `00ba8aa0(value)`, then
/// the object's vtable `+0xC8` with the same word.
pub fn fn_00b70290(e: &mut Engine, this: Ptr, value: u32) {
    e.call(SHADER_PROPERTY_STEP_00BA8AA0, &args![this, value]);
    e.vcall(this.addr(), 0xc8, &args![value]);
}

// Translated from 00b702b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words): `00ba8b10(first,
/// second)`, then the object's vtable `+0xC8` with the first word.
pub fn fn_00b702b0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(SHADER_PROPERTY_STEP_00BA8B10, &args![this, first, second]);
    e.vcall(this.addr(), 0xc8, &args![first]);
}

// Translated from 00b702e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a list of two words, head and
/// tail; two words: `node`, `before`): moves `node` (next at `+0`, previous
/// at `+4`) to just before `before` in the doubly linked list: nothing when
/// they are the same node; the head and tail are kept up to date, `node` is
/// unlinked from its neighbours and linked in between `before`'s previous
/// node and `before`.
pub fn fn_00b702e0(e: &mut Engine, this: Ptr, node: u32, before: u32) {
    if node == before {
        return;
    }
    let list = this.addr();
    if e.mem.u32(list) == node {
        let next = e.mem.u32(node);
        e.mem.set_u32(list, next);
    }
    if e.mem.u32(list) == before {
        e.mem.set_u32(list, node);
    }
    if e.mem.u32(list + 4) == node {
        let previous = e.mem.u32(node + 4);
        e.mem.set_u32(list + 4, previous);
    }
    let next = e.mem.u32(node);
    if next != 0 {
        let previous = e.mem.u32(node + 4);
        e.mem.set_u32(next + 4, previous);
    }
    let previous = e.mem.u32(node + 4);
    if previous != 0 {
        let next = e.mem.u32(node);
        e.mem.set_u32(previous, next);
    }
    let previous = e.mem.u32(before + 4);
    e.mem.set_u32(node + 4, previous);
    e.mem.set_u32(node, before);
    if previous != 0 {
        e.mem.set_u32(previous, node);
    }
    e.mem.set_u32(before + 4, node);
}

/// `00c9a420` (`thiscall`, one word: the other object; `bool` in `AL`): the comparison
/// `fn_00b70270` passes on (the slot +0x5c of `TallGrassShaderProperty`'s vtable, `IsEqual` in
/// the Xbox PDB's slot list).
const IS_EQUAL_00C9A420: u32 = 0x00c9_a420;

// Translated from 00b70270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x5c of `DistantLODShaderProperty`'s vtable `010adad0` and
/// four others (`IsEqual` in the Xbox PDB's slot list; `thiscall`, one word: the other object).
/// Answers whether `00c9a420` with `this` and the word answers true (`TEST AL,AL; SETNZ AL`).
pub fn fn_00b70270(e: &mut Engine, this: Ptr, other: u32) -> bool {
    e.call(IS_EQUAL_00C9A420, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_0270, fn_00b70270(Ptr, u32) -> bool),
        entry!(0x00b7_0260, fn_00b70260(Ptr, Ptr)),
        entry!(0x00b7_0290, fn_00b70290(Ptr, u32)),
        entry!(0x00b7_02b0, fn_00b702b0(Ptr, u32, u32)),
        entry!(0x00b7_02e0, fn_00b702e0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00b70260_jumps_to_the_load_body() {
        let mut e = Engine::new();
        e.register(SHADER_PROPERTY_LOAD_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b7_0260, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_LOAD_BINARY),
            vec![vec![0x1000, 0x2000]]
        );
    }

    fn object_with_slot(e: &mut Engine, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xc8, target);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn fn_00b70290_runs_the_step_then_the_virtual_call() {
        const SLOT: u32 = 0x0100_00c8;
        let mut e = Engine::new();
        e.register(SHADER_PROPERTY_STEP_00BA8AA0, |_, _| Ret::default());
        e.register(SLOT, |_, _| Ret::default());
        let this = object_with_slot(&mut e, SLOT);
        e.call_log = Some(vec![]);
        e.call(0x00b7_0290, &args![this, 7u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x00b7_0290, vec![this, 7]),
                (SHADER_PROPERTY_STEP_00BA8AA0, vec![this, 7]),
                (SLOT, vec![this, 7]),
            ]
        );
    }

    #[test]
    fn fn_00b702b0_passes_both_words_then_the_first() {
        const SLOT: u32 = 0x0100_00c8;
        let mut e = Engine::new();
        e.register(SHADER_PROPERTY_STEP_00BA8B10, |_, _| Ret::default());
        e.register(SLOT, |_, _| Ret::default());
        let this = object_with_slot(&mut e, SLOT);
        e.call_log = Some(vec![]);
        e.call(0x00b7_02b0, &args![this, 7u32, 9u32]);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_STEP_00BA8B10),
            vec![vec![this, 7, 9]]
        );
        assert_eq!(calls_to(&e, SLOT), vec![vec![this, 7]]);
    }

    #[test]
    fn fn_00b702e0_moves_a_node_before_another() {
        let mut e = Engine::new();
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(8)).collect();
        let (a, b, c) = (nodes[0], nodes[1], nodes[2]);
        // a <-> b <-> c
        e.mem.set_u32(a, b);
        e.mem.set_u32(b + 4, a);
        e.mem.set_u32(b, c);
        e.mem.set_u32(c + 4, b);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, a);
        e.mem.set_u32(list + 4, c);
        // The same node: nothing.
        e.call(0x00b7_02e0, &args![list, a, a]);
        assert_eq!(e.mem.u32(list), a);
        // c before a: c <-> a <-> b.
        e.call(0x00b7_02e0, &args![list, c, a]);
        assert_eq!(e.mem.u32(list), c);
        assert_eq!(e.mem.u32(list + 4), b);
        assert_eq!(e.mem.u32(c), a);
        assert_eq!(e.mem.u32(c + 4), 0);
        assert_eq!(e.mem.u32(a + 4), c);
        assert_eq!(e.mem.u32(b), 0);
        // b before a (the head moves off nothing): c <-> b <-> a.
        e.call(0x00b7_02e0, &args![list, b, a]);
        assert_eq!(e.mem.u32(c), b);
        assert_eq!(e.mem.u32(b + 4), c);
        assert_eq!(e.mem.u32(b), a);
        assert_eq!(e.mem.u32(list + 4), a);
        assert_eq!(e.mem.u32(a), 0);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00b70270_tests_the_low_byte() {
        let mut e = Engine::new();
        e.register(IS_EQUAL_00C9A420, |_, a| (a[1] << 8).into_ret());
        e.call_log = Some(vec![]);
        // A non-zero word whose low byte is 0 is false.
        assert_eq!(e.call(0x00b7_0270, &args![0x10u32, 1u32]).u32(), 0);
        e.register(IS_EQUAL_00C9A420, |_, a| a[1].into_ret());
        assert_eq!(e.call(0x00b7_0270, &args![0x10u32, 0x7u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(IS_EQUAL_00C9A420, vec![0x10, 7])));
    }
}
