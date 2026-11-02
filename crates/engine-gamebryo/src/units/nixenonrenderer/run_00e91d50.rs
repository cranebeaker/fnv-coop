//! `NiXenonRenderer/run_00e91d50` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043a010` (`thiscall`, `allocate_node`): a list node from the pool at `this`.
const ALLOCATE_NODE: u32 = 0x0043_a010;
/// `0126f0f8`: head of the list `fn_00e91d50` keeps (node: next at `+0`, previous at `+4`,
/// the number at `+8`).
const LIST_HEAD: u32 = 0x0126_f0f8;
/// `0126f0fc`: tail of the list.
const LIST_TAIL: u32 = 0x0126_f0fc;
/// `0126f100`: the node count, also the `this` of the node allocation.
const LIST_COUNT: u32 = 0x0126_f100;
/// Byte `0126fa00`: set to 1 when `fn_00e91dc0` has started.
const SETUP_STARTED: u32 = 0x0126_fa00;
/// Word `0126fa04`: how many times the registrar `fn_00e91e30` ran.
const REGISTRAR_COUNT: u32 = 0x0126_fa04;
/// `00aa2080` (`cdecl`, two words): registers a start-up function and its second word.
const REGISTER_STARTUP: u32 = 0x00aa_2080;

/// The no-argument steps `fn_00e91dc0` runs after the five numbers, in order. The last
/// one, `00e81100`, is reached by a jump.
const STEPS: [u32; 10] = [
    0x00e7_f2b0,
    0x00e8_95f0,
    0x00e8_0620,
    0x00e7_a860,
    0x00e9_2e60,
    0x00e9_3df0,
    0x00e9_1b40,
    0x00e9_1b90,
    0x00e9_1af0,
    0x00e8_1100,
];

// Translated from 00e91d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): adds `key` to the end of the list headed
/// at `0126f0f8` unless a node with that number is in it already (nodes come from
/// `0043a010` with `this` = `0126f100`, the node count).
pub fn fn_00e91d50(e: &mut Engine, key: u32) {
    let mut node = e.mem.u32(LIST_HEAD);
    if node != 0 {
        loop {
            let current = node;
            node = e.mem.u32(current);
            if key == e.mem.u32(current + 8) {
                if current != 0 {
                    return;
                }
                break;
            }
            if node == 0 {
                break;
            }
        }
    }
    let new = e.call(ALLOCATE_NODE, &args![LIST_COUNT]).u32();
    e.mem.set_u32(new + 8, key);
    e.mem.set_u32(new, 0);
    let tail = e.mem.u32(LIST_TAIL);
    e.mem.set_u32(new + 4, tail);
    let tail = e.mem.u32(LIST_TAIL);
    let count = e.mem.u32(LIST_COUNT);
    if tail != 0 {
        e.mem.set_u32(tail, new);
        e.mem.set_u32(LIST_COUNT, count.wrapping_add(1));
        e.mem.set_u32(LIST_TAIL, new);
    } else {
        e.mem.set_u32(LIST_COUNT, count.wrapping_add(1));
        e.mem.set_u32(LIST_HEAD, new);
        e.mem.set_u32(LIST_TAIL, new);
    }
}

// Translated from 00e91dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no Ghidra function; the start-up function that
/// `fn_00e91e30` registers; no arguments): once only (byte `0126fa00`), adds the numbers
/// 0x15, 0x16, 0x19, 0x18 and 0x17 to the list (`fn_00e91d50`), then runs ten steps with no
/// arguments: `00e7f2b0`, `00e895f0`, `00e80620`, `00e7a860`, `00e92e60`, `00e93df0`,
/// `00e91b40`, `00e91b90`, `00e91af0` and, by a jump, `00e81100`.
pub fn fn_00e91dc0(e: &mut Engine) {
    if e.mem.u8(SETUP_STARTED) != 0 {
        return;
    }
    e.mem.set_u8(SETUP_STARTED, 1);
    for key in [0x15u32, 0x16, 0x19, 0x18, 0x17] {
        fn_00e91d50(e, key);
    }
    for step in STEPS {
        e.call(step, &args![]);
    }
}

// Translated from 00e91e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no Ghidra function; `thiscall`, no stack word; the registrar
/// the static initializers run): counts its runs in `0126fa04`, and on the first one
/// registers `fn_00e91dc0` with `00e91c80` through `00aa2080`. Returns `this`.
pub fn fn_00e91e30(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.mem.u32(REGISTRAR_COUNT);
    e.mem.set_u32(REGISTRAR_COUNT, count.wrapping_add(1));
    if count == 0 {
        e.call(REGISTER_STARTUP, &args![0x00e9_1dc0u32, 0x00e9_1c80u32]);
    }
    this
}

/// `00e80690` (unit `nid3dshader.cpp`, unnamed in the engine map; no arguments): the first
/// step of `fn_00e91c80`.
const SHUTDOWN_FIRST_00E80690: u32 = 0x00e8_0690;
/// The three words `fn_00e91c80` empties, in its order (`0126fa3c`, `0126fa54`, `0126fa50`):
/// each object is handed to `00e93260` (`cdecl`, one word) and, read again, deleted through
/// its vtable slot 0 with 1.
const SHUTDOWN_OBJECTS: [u32; 3] = [0x0126_fa3c, 0x0126_fa54, 0x0126_fa50];
const RELEASE_00E93260: u32 = 0x00e9_3260;
/// `00e93f50` (`cdecl`, one word; given 0).
const SET_00E93F50: u32 = 0x00e9_3f50;
/// The no-argument steps after it, in order; `00e6ec20` is reached by a jump.
const SHUTDOWN_STEPS: [u32; 7] = [
    0x00e9_33d0,
    0x00e9_2080,
    0x00e7_a6a0,
    0x00e8_0630,
    0x00e7_f610,
    0x00e8_9740,
    0x00e6_ec20,
];

// Translated from 00e91c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no Ghidra function, read from its bytes; no arguments): the
/// shut-down function `fn_00e91e30` registers beside `fn_00e91dc0`. Only when the start-up
/// ran (byte `0126fa00` set): clears that byte, runs `00e80690`, empties the words
/// `0126fa3c`, `0126fa54` and `0126fa50` (each object, when not null, handed to `00e93260`
/// and then, when still not null, deleted through its vtable slot 0 with 1; the word
/// cleared), runs `00e93f50` with 0 and then `00e933d0`, `00e92080`, `00e7a6a0`, `00e80630`,
/// `00e7f610`, `00e89740` and, by a jump, `00e6ec20`.
pub fn fn_00e91c80(e: &mut Engine) {
    if e.mem.u8(SETUP_STARTED) == 0 {
        return;
    }
    e.mem.set_u8(SETUP_STARTED, 0);
    e.call(SHUTDOWN_FIRST_00E80690, &args![]);
    for word in SHUTDOWN_OBJECTS {
        let object = e.mem.u32(word);
        if object != 0 {
            e.call(RELEASE_00E93260, &args![object]);
            let object = e.mem.u32(word);
            if object != 0 {
                e.vcall(object, 0, &args![1u32]);
            }
        }
        e.mem.set_u32(word, 0);
    }
    e.call(SET_00E93F50, &args![0u32]);
    for step in SHUTDOWN_STEPS {
        e.call(step, &args![]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_1d50, fn_00e91d50(u32)),
        entry!(0x00e9_1dc0, fn_00e91dc0()),
        entry!(0x00e9_1e30, fn_00e91e30(Ptr) -> Ptr),
        entry!(0x00e9_1c80, fn_00e91c80()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.register(ALLOCATE_NODE, |e, a| {
            assert_eq!(a, &[LIST_COUNT]);
            e.mem.alloc(0x10).into_ret()
        });
        e
    }

    fn keys(e: &Engine) -> Vec<u32> {
        let mut out = vec![];
        let mut node = e.mem.u32(LIST_HEAD);
        while node != 0 {
            out.push(e.mem.u32(node + 8));
            node = e.mem.u32(node);
        }
        out
    }

    #[test]
    fn list_gets_each_number_once_in_order() {
        let mut e = engine();
        for key in [5u32, 7, 5, 9, 7] {
            e.call(0x00e9_1d50, &args![key]);
        }
        assert_eq!(keys(&e), vec![5, 7, 9]);
        assert_eq!(e.mem.u32(LIST_COUNT), 3);
        let tail = e.mem.u32(LIST_TAIL);
        assert_eq!(e.mem.u32(tail + 8), 9);
        let previous = e.mem.u32(tail + 4);
        assert_eq!(e.mem.u32(previous + 8), 7);
        assert_eq!(e.mem.u32(e.mem.u32(LIST_HEAD) + 4), 0);
    }

    #[test]
    fn start_up_adds_five_numbers_and_runs_the_steps_once() {
        let mut e = engine();
        for step in STEPS {
            e.register(step, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00e9_1dc0, &args![]);
        assert_eq!(keys(&e), vec![0x15, 0x16, 0x19, 0x18, 0x17]);
        let order: Vec<u32> = e
            .call_log
            .clone()
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .filter(|a| STEPS.contains(a))
            .collect();
        assert_eq!(order, STEPS.to_vec());
        e.call_log = Some(vec![]);
        e.call(0x00e9_1dc0, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }

    #[test]
    fn registrar_registers_the_start_up_function_once() {
        let mut e = engine();
        e.register(REGISTER_STARTUP, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_1e30, &args![this]).u32(), this);
        assert_eq!(e.call(0x00e9_1e30, &args![this]).u32(), this);
        let calls: Vec<_> = e
            .call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == REGISTER_STARTUP)
            .collect();
        assert_eq!(
            calls,
            vec![(REGISTER_STARTUP, vec![0x00e9_1dc0, 0x00e9_1c80])]
        );
        assert_eq!(e.mem.u32(REGISTRAR_COUNT), 2);
    }
}

#[cfg(test)]
mod shut_down_tests {
    use super::*;

    const DELETE: u32 = 0x0e40_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        let mut callees = vec![SHUTDOWN_FIRST_00E80690, SET_00E93F50, DELETE];
        callees.extend(SHUTDOWN_STEPS);
        for callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
        // The release empties `0126fa54`'s word: that object is not deleted.
        e.register(RELEASE_00E93260, |e, a| {
            if e.mem.u32(0x0126_fa54) == a[0] {
                e.mem.set_u32(0x0126_fa54, 0);
            }
            Ret::default()
        });
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, DELETE);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn fn_00e91c80_shuts_down_once_started() {
        let mut e = engine();
        e.mem.set_u8(SETUP_STARTED, 1);
        let first = object(&mut e);
        let second = object(&mut e);
        e.mem.set_u32(0x0126_fa3c, first);
        e.mem.set_u32(0x0126_fa54, second);
        e.call_log = Some(vec![]);
        e.call(0x00e9_1c80, &args![]);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![
            (SHUTDOWN_FIRST_00E80690, vec![]),
            (RELEASE_00E93260, vec![first]),
            (DELETE, vec![first, 1]),
            (RELEASE_00E93260, vec![second]),
            (SET_00E93F50, vec![0]),
        ];
        expected.extend(SHUTDOWN_STEPS.iter().map(|step| (*step, vec![])));
        assert_eq!(log[1..].to_vec(), expected);
        assert_eq!(e.mem.u8(SETUP_STARTED), 0);
        for word in SHUTDOWN_OBJECTS {
            assert_eq!(e.mem.u32(word), 0);
        }

        // Not started (the byte now clear): nothing runs.
        e.call_log = Some(vec![]);
        e.call(0x00e9_1c80, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
