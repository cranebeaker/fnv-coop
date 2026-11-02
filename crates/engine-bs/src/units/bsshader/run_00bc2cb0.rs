//! `BSShader/run_00bc2cb0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00bc2bd0` (unit `run_00bc2bd0`, `ECX`): the constructor of the 0xC0-byte
/// node.
const NODE_CONSTRUCT: u32 = 0x00bc_2bd0;

// Translated from 00bc2cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc2cb0 - the FS:[0] exception frame (state -1, then 0 around the constructor) and its unwind handler 00f2c0f2 are not translated
/// A creation function (the `NiStream` loader table in `bsshadermanager.rs` lists
/// it for `BSOrderedNode`; `cdecl`, no words): 0xC0 bytes constructed by
/// `00bc2bd0`, or null when the allocation fails. The frame's cookie word
/// (`011c16bc ^ ESP`) is only stored for the handler, never checked.
pub fn fn_00bc2cb0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0xc0u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(NODE_CONSTRUCT, &args![block]).ptr()
}

/// `NiNode::IsEqual` (`00a5e600`, thiscall: other object) and the comparison of four
/// `float`s (`00a92660`, thiscall on the first: the address of the second; answers a byte).
const NI_NODE_IS_EQUAL: u32 = 0x00a5_e600;
const FLOAT4_EQUAL_00A92660: u32 = 0x00a9_2660;

// Translated from 00bc2d20 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// The `IsEqual` of the vtable at `010ba730` (its slot +0x5c; the node `00bc2bd0` constructs;
/// thiscall, `RET 4`): false when `NiNode::IsEqual` is or the bytes at +0xbc differ; with
/// that byte set, the comparison of the four `float`s at +0xac (`00a92660`); else true.
pub fn fn_00bc2d20(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if e.call(NI_NODE_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    let flag = e.mem.u8(this.addr().wrapping_add(0xbc));
    if flag != e.mem.u8(other.addr().wrapping_add(0xbc)) {
        return false;
    }
    if flag == 0 {
        return true;
    }
    e.call(
        FLOAT4_EQUAL_00A92660,
        &args![
            this.addr().wrapping_add(0xac),
            other.addr().wrapping_add(0xac)
        ],
    )
    .u8()
        != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_2cb0, fn_00bc2cb0() -> Ptr),
        entry!(0x00bc_2d20, fn_00bc2d20(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bc2cb0_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(NODE_CONSTRUCT, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00bc_2cb0, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00bc_2cb0, &args![]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![0xc0]));
        assert_eq!(log[2], (NODE_CONSTRUCT, vec![made - 1]));
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    #[test]
    fn is_equal_compares_the_base_the_flag_and_the_floats() {
        const ANSWERS: u32 = 0x03f0_0000;
        let mut e = Engine::new();
        e.mem.map(ANSWERS, 8);
        e.mem.set_u32(ANSWERS, 1);
        e.mem.set_u32(ANSWERS + 4, 0);
        e.register(NI_NODE_IS_EQUAL, |e, _| e.mem.u32(ANSWERS).into_ret());
        e.register(FLOAT4_EQUAL_00A92660, |e, _| {
            e.mem.u32(ANSWERS + 4).into_ret()
        });
        let a = e.mem.alloc(0xc0);
        let b = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        // Both flags clear: equal without the float comparison.
        assert!(e.call(0x00bc_2d20, &args![a, b]).bool());
        e.mem.set_u8(a + 0xbc, 1);
        assert!(!e.call(0x00bc_2d20, &args![a, b]).bool());
        e.mem.set_u8(b + 0xbc, 1);
        assert!(!e.call(0x00bc_2d20, &args![a, b]).bool());
        e.mem.set_u32(ANSWERS + 4, 1);
        assert!(e.call(0x00bc_2d20, &args![a, b]).bool());
        let log = e.call_log.take().unwrap();
        let compares: Vec<_> = log
            .iter()
            .filter(|c| c.0 == FLOAT4_EQUAL_00A92660)
            .collect();
        assert_eq!(compares.len(), 2);
        assert_eq!(compares[0].1, vec![a + 0xac, b + 0xac]);
        e.mem.set_u32(ANSWERS, 0);
        assert!(!e.call(0x00bc_2d20, &args![a, b]).bool());
    }
}
