//! `BSShader/run_00bc2bd0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSNiNode::BSNiNode` (`00c5f570`, `thiscall`, no words).
const BS_NI_NODE_CONSTRUCT: u32 = 0x00c5_f570;
/// `00c5faa0` (unit `bsninode.obj`, `thiscall`, no words): the `BSNiNode`
/// destructor body.
const BS_NI_NODE_DESTRUCTOR: u32 = 0x00c5_faa0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// The vtable both functions store (`BSOrderedNode`, the class `00bc2cb0` creates).
const NODE_VTABLE: u32 = 0x010b_a730;
/// The vector the constructor copies to `+0xAC` (`011f426c`).
const ORIGIN: u32 = 0x011f_426c;
/// The object's size.
const NODE_SIZE: u32 = 0xc0;

// Translated from 00bc2bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, no words): the `BSOrderedNode`
/// constructor (`00bc2cb0`, the loader table's `BSOrderedNode` creation
/// function, constructs with it): `BSNiNode::BSNiNode`, the vtable
/// `010ba730`, the three words at `011f426c` to `+0xAC`, 0.0 at `+0xB8`
/// (`FLDZ`), the byte `+0xBC` cleared. Answers `this`.
pub fn fn_00bc2bd0(e: &mut Engine, this: Ptr) -> Ptr {
    let a = this.addr();
    e.call(BS_NI_NODE_CONSTRUCT, &args![this]);
    e.mem.set_u32(a, NODE_VTABLE);
    for i in 0..3 {
        let word = e.mem.u32(ORIGIN + 4 * i);
        e.mem.set_u32(a + 0xac + 4 * i, word);
    }
    e.mem.set_u32(a + 0xb8, 0);
    e.mem.set_u8(a + 0xbc, 0);
    this
}

// Translated from 00bc2c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): the scalar deleting
/// destructor of the same class: its vtable, the `BSNiNode` destructor body
/// (`00c5faa0`), then the sized delete of 0xC0 bytes when bit 0 of `flags` is
/// set. Answers `this`.
pub fn fn_00bc2c20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NODE_VTABLE);
    e.call(BS_NI_NODE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, NODE_SIZE]);
    }
    this
}

/// The record `00bc2bc0` answers (`01201f84`).
const RTTI_01201F84: u32 = 0x0120_1f84;

// Translated from 00bc2bc0 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// The `GetRTTI` of the vtable at `010ba730` (its slot +8; the class [`fn_00bc2bd0`]
/// constructs): the address of the record at `01201f84`.
pub fn fn_00bc2bc0(_e: &mut Engine) -> Ptr {
    Ptr::new(RTTI_01201F84)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_2bc0, fn_00bc2bc0() -> Ptr),
        entry!(0x00bc_2bd0, fn_00bc2bd0(Ptr) -> Ptr),
        entry!(0x00bc_2c20, fn_00bc2c20(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bc2bd0_constructs_the_node() {
        let mut e = Engine::new();
        e.register(BS_NI_NODE_CONSTRUCT, |_, _| Ret::default());
        e.map(ORIGIN, 0xc);
        e.mem.set_f32(ORIGIN + 4, 2.0);
        let this = e.mem.alloc(NODE_SIZE);
        e.mem.set_u8(this + 0xbc, 1);
        e.mem.set_f32(this + 0xb8, 5.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_2bd0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NODE_VTABLE);
        assert_eq!(e.mem.f32(this + 0xb0), 2.0);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert_eq!(e.mem.u8(this + 0xbc), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BS_NI_NODE_CONSTRUCT, vec![this])));
    }

    #[test]
    fn fn_00bc2c20_destroys_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(BS_NI_NODE_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(NODE_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_2c20, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NODE_VTABLE);
        e.call(0x00bc_2c20, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == NI_SIZED_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![this, NODE_SIZE]);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bc_2bc0, &args![]).u32(), RTTI_01201F84);
    }
}
