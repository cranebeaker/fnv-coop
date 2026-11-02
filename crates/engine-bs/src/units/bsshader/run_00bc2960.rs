//! `BSShader/run_00bc2960` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiNode`'s constructor (`00a5ecb0`, `thiscall`, one word: the child array's size).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The vtable this constructor sets (`010ba608`; no RTTI locator before it).
const VTABLE_010BA608: u32 = 0x010b_a608;
/// The `float`s copied to `+0xb4` (`01016410`) and `+0xb0` (`010ba600`).
const VALUE_FOR_B4_01016410: u32 = 0x0101_6410;
const VALUE_FOR_B0_010BA600: u32 = 0x010b_a600;

// Translated from 00bc2980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader`); a constructor of an `NiNode` subclass whose class
/// is not confirmed (its vtable `010ba608` has no RTTI locator). `thiscall`, no stack words;
/// answers `this`. Constructs the `NiNode` (child array size 0), copies the `float`s at
/// `01016410` to `+0xb4` and `010ba600` to `+0xb0` (`FLD`/`FSTP`: a signalling NaN comes out
/// quiet), sets the vtable and clears the bytes at `+0xad` and `+0xac`.
pub fn fn_00bc2980(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_NODE_CONSTRUCT, &args![base, 0u32]);
    let first = x87_quiet_bits(e.global::<u32>(VALUE_FOR_B4_01016410));
    e.mem.set_u32(base.wrapping_add(0xb4), first);
    e.mem.set_u32(base, VTABLE_010BA608);
    let second = x87_quiet_bits(e.global::<u32>(VALUE_FOR_B0_010BA600));
    e.mem.set_u8(base.wrapping_add(0xad), 0);
    e.mem.set_u32(base.wrapping_add(0xb0), second);
    e.mem.set_u8(base.wrapping_add(0xac), 0);
    this
}

/// The flag `00bc2960` tests and clears (`01201f70`).
const PENDING_FLAG_01201F70: u32 = 0x0120_1f70;
/// The two cdecl calls `00bc2960` makes when the flag is set (`00b4e380`,
/// in `BSShader/bsfadenode.cpp` by the engine map, and `00b6c130`).
const PENDING_CALL_B4E380: u32 = 0x00b4_e380;
const PENDING_CALL_B6C130: u32 = 0x00b6_c130;
/// The `NiNode` destructor body (`00a5e930`, thiscall).
const NI_NODE_DESTROY: u32 = 0x00a5_e930;
/// cdecl (block, size): frees an Ni object of the given size (`00aa1460`).
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// `NiNode::OnVisible` (engine map), thiscall (node, culler).
const NI_NODE_ON_VISIBLE: u32 = 0x00a5_dbe0;

// Translated from 00bc2960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl without words: when the byte at
/// `01201f70` is set, calls `00b4e380` and `00b6c130` and clears it.
pub fn fn_00bc2960(e: &mut Engine) {
    if e.global::<u8>(PENDING_FLAG_01201F70) != 0 {
        e.call(PENDING_CALL_B4E380, &args![]);
        e.call(PENDING_CALL_B6C130, &args![]);
        e.set_global(PENDING_FLAG_01201F70, 0u8);
    }
}

// Translated from 00bc29e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// (Ghidra has no function here; translated from the disassembled bytes.)
/// The scalar deleting destructor of the class `00bc2980` constructs (its
/// vtable `010ba608`; unnamed in the engine map), thiscall `(flags)`: the
/// class's destructor body writes the same fields as the constructor (the
/// float at `01016410` to `+0xb4`, the vtable, the float at `010ba600` to
/// `+0xb0`, both through `FLD`/`FSTP`, and clears the bytes at `+0xad` and
/// `+0xac`), then the `NiNode` destructor runs; with bit 0 of the flags the
/// object (0xb8 bytes) is freed. Answers `this`.
pub fn fn_00bc29e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let first = x87_quiet_bits(e.global::<u32>(VALUE_FOR_B4_01016410));
    e.mem.set_u32(base.wrapping_add(0xb4), first);
    let second = x87_quiet_bits(e.global::<u32>(VALUE_FOR_B0_010BA600));
    e.mem.set_u32(base, VTABLE_010BA608);
    e.mem.set_u32(base.wrapping_add(0xb0), second);
    e.mem.set_u8(base.wrapping_add(0xad), 0);
    e.mem.set_u8(base.wrapping_add(0xac), 0);
    e.call(NI_NODE_DESTROY, &args![base]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![base, 0xb8u32]);
    }
    this
}

// Translated from 00bc2ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// (Ghidra has no function here; translated from the disassembled bytes.)
/// An `OnVisible` override (unnamed in the engine map; it calls
/// `NiNode::OnVisible` with `ECX` unchanged), thiscall `(culler)`: when
/// the culler's object at `+0xc4` exists, clears its byte at `+0x84` and
/// sets its word at `+0x34` to 2 around the base `OnVisible`, then sets the
/// byte to 1 and the word to 0.
pub fn fn_00bc2ae0(e: &mut Engine, this: Ptr, culler: u32) {
    let state = e.mem.u32(culler.wrapping_add(0xc4));
    if state != 0 {
        e.mem.set_u8(state + 0x84, 0);
        e.mem.set_u32(state + 0x34, 2);
    }
    e.call(NI_NODE_ON_VISIBLE, &args![this, culler]);
    if state != 0 {
        e.mem.set_u8(state + 0x84, 1);
        e.mem.set_u32(state + 0x34, 0);
    }
}

/// `NiNode::GetViewerStrings` (`00a5ea70`, engine map), the record `00bc29d0` answers
/// (`01201f74`), `NiNew`'s allocator (`00aa13e0`, cdecl) and the node member copy
/// (`00a5dc80`, thiscall on the source: clone, cloning process).
const NI_NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
const RTTI_01201F74: u32 = 0x0120_1f74;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_NODE_COPY_MEMBERS: u32 = 0x00a5_dc80;

// Translated from 00bc29c0 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// The `GetViewerStrings` of the vtable at `010ba608` (its slot +0x60): a 5-byte jump to
/// `NiNode::GetViewerStrings` (`00a5ea70`) with `this` and the string array.
pub fn fn_00bc29c0(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(NI_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
}

// Translated from 00bc29d0 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// The `GetRTTI` of the vtable at `010ba608` (its slot +8): the address of the record at
/// `01201f74`.
pub fn fn_00bc29d0(_e: &mut Engine) -> Ptr {
    Ptr::new(RTTI_01201F74)
}

// Translated from 00bc2a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc2a30 - the C++ exception frame (FS:[0] chain, unwind state 0 around the constructor) is not translated.
/// The `CreateClone` of the vtable at `010ba608` (its slot +0x48; thiscall, `RET 4`: the
/// cloning process): 0xb8 bytes (`00aa13e0`) constructed as [`fn_00bc2980`] does (the exe
/// inlines that constructor; null when the allocation fails), then the node member copy
/// (`00a5dc80`) into it (into null too). Answers the clone.
pub fn fn_00bc2a30(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0xb8u32]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00bc2980(e, Ptr::new(block))
    };
    e.call(NI_NODE_COPY_MEMBERS, &args![this, clone, cloning]);
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_29c0, fn_00bc29c0(Ptr, u32)),
        entry!(0x00bc_29d0, fn_00bc29d0() -> Ptr),
        entry!(0x00bc_2a30, fn_00bc2a30(Ptr, u32) -> Ptr),
        entry!(0x00bc_2980, fn_00bc2980(Ptr) -> Ptr),
        entry!(0x00bc2960, fn_00bc2960()),
        entry!(0x00bc29e0, fn_00bc29e0(Ptr, u32) -> Ptr),
        entry!(0x00bc2ae0, fn_00bc2ae0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod pending_tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn pending_calls_run_once_while_the_flag_is_set() {
        let mut e = Engine::new();
        e.map(0x0120_1000, 0x1000);
        e.register(PENDING_CALL_B4E380, |_, _| Ret::default());
        e.register(PENDING_CALL_B6C130, |_, _| Ret::default());
        e.set_global(PENDING_FLAG_01201F70, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x00bc_2960, &args![]);
        e.call(0x00bc_2960, &args![]);
        assert_eq!(calls(&e, PENDING_CALL_B4E380).len(), 1);
        assert_eq!(calls(&e, PENDING_CALL_B6C130).len(), 1);
        assert_eq!(e.global::<u8>(PENDING_FLAG_01201F70), 0);
    }

    #[test]
    fn deleting_destructor_resets_the_fields_and_frees_on_bit_0() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x010b_a000, 0x1000);
        e.set_global(VALUE_FOR_B4_01016410, 0x7f80_0001u32);
        e.set_global(VALUE_FOR_B0_010BA600, 2.5f32);
        e.register(NI_NODE_DESTROY, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(0xb8);
        e.mem.set_u16(this + 0xac, 0xffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_29e0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BA608);
        assert_eq!(e.mem.u32(this + 0xb4), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0xb0), 2.5);
        assert_eq!(e.mem.u16(this + 0xac), 0);
        assert_eq!(calls(&e, NI_NODE_DESTROY), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_SIZED), vec![vec![this, 0xb8]]);
        e.call_log = Some(vec![]);
        e.call(0x00bc_29e0, &args![this, 0u32]);
        assert!(calls(&e, NI_FREE_SIZED).is_empty());
    }

    #[test]
    fn on_visible_marks_the_culler_state_around_the_base() {
        let mut e = Engine::new();
        let culler = e.mem.alloc(0xc8);
        let state = e.mem.alloc(0x88);
        e.mem.set_u32(culler + 0xc4, state);
        e.register_double(NI_NODE_ON_VISIBLE, move |e, _| {
            assert_eq!(e.mem.u8(state + 0x84), 0);
            assert_eq!(e.mem.u32(state + 0x34), 2);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00bc_2ae0, &args![0x900u32, culler]);
        assert_eq!(calls(&e, NI_NODE_ON_VISIBLE), vec![vec![0x900, culler]]);
        assert_eq!(e.mem.u8(state + 0x84), 1);
        assert_eq!(e.mem.u32(state + 0x34), 0);
        e.mem.set_u32(culler + 0xc4, 0);
        e.register(NI_NODE_ON_VISIBLE, |_, _| Ret::default());
        e.call(0x00bc_2ae0, &args![0x900u32, culler]);
        assert_eq!(e.mem.u8(state + 0x84), 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bc2980_constructs_the_node_and_its_fields() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x010b_a000, 0x1000);
        e.set_global(VALUE_FOR_B4_01016410, 100.0f32);
        e.set_global(VALUE_FOR_B0_010BA600, 0x7f80_0003u32);
        e.register(NI_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xb8);
        e.mem.set_u8(this + 0xac, 0xff);
        e.mem.set_u8(this + 0xad, 0xff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_2980, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let node: Vec<_> = log.iter().filter(|c| c.0 == NI_NODE_CONSTRUCT).collect();
        assert_eq!(node[0].1, vec![this, 0]);
        assert_eq!(e.mem.u32(this), VTABLE_010BA608);
        assert_eq!(e.mem.f32(this + 0xb4), 100.0);
        assert_eq!(e.mem.u32(this + 0xb0), 0x7fc0_0003);
        assert_eq!(e.mem.u8(this + 0xac), 0);
        assert_eq!(e.mem.u8(this + 0xad), 0);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_viewer_strings_jumps_to_the_node() {
        let mut e = Engine::new();
        e.register(NI_NODE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00bc_29c0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, NI_NODE_GET_VIEWER_STRINGS),
            vec![vec![0x1000, 0x2000]]
        );
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bc_29d0, &args![]).u32(), RTTI_01201F74);
    }

    #[test]
    fn create_clone_constructs_and_copies_even_into_null() {
        let mut e = Engine::new();
        e.register(NI_NODE_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_NODE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.mem.map(VALUE_FOR_B4_01016410, 4);
        e.mem.set_f32(VALUE_FOR_B4_01016410, 100.0);
        e.mem.map(VALUE_FOR_B0_010BA600, 4);
        e.mem.set_u32(VALUE_FOR_B0_010BA600, 0x7f80_0001);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00bc_2a30, &args![0x1000u32, 0x2000u32]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0xb8]]);
        assert_eq!(calls_to(&e, NI_NODE_CONSTRUCT), vec![vec![clone, 0]]);
        assert_eq!(e.mem.u32(clone), VTABLE_010BA608);
        assert_eq!(e.mem.f32(clone + 0xb4), 100.0);
        assert_eq!(e.mem.u32(clone + 0xb0), 0x7fc0_0001);
        assert_eq!(
            calls_to(&e, NI_NODE_COPY_MEMBERS),
            vec![vec![0x1000, clone, 0x2000]]
        );
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00bc_2a30, &args![0x1000u32, 0x2000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, NI_NODE_COPY_MEMBERS)[1],
            vec![0x1000, 0, 0x2000]
        );
    }
}
