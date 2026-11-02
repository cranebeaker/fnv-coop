//! `fallout/ai/run_008d40e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d4380 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `008d50a0`, which frees the block held at `+0`
/// through `00401030` when it is non-null and zeroes `+0`, and returns nothing. Parameters:
/// one word, `this`; the disassembly of the function ends in a plain `RET` and passes only
/// `ECX` on. `008d50a0`'s disassembly ends on the store `MOV dword ptr [ECX],0x0` with no
/// result (Ghidra types it void).
pub fn fn_008d4380(e: &mut Engine, this: Ptr) {
    e.call(0x008d_50a0, &args![this]);
}

// Translated from 008d4360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that runs `008d5020` on the object (`ECX`) and answers it. The engine map gives it
/// no name.
pub fn fn_008d4360(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x008d_5020, &args![this]);
    this
}

/// `0087f4c0` (thiscall): a word of the actor passed on to `00613a20`.
const ACTOR_0087F4C0: u32 = 0x0087_f4c0;
/// `00613a20` (thiscall: word, index; unit `tesrace.cpp` in the engine map):
/// the base form's part `index`, or null.
const BASE_PART: u32 = 0x0061_3a20;
/// `004b75a0` `BSstristr` (Xbox PDB name in the engine map; cdecl:
/// haystack, needle).
const BS_STRISTR: u32 = 0x004b_75a0;
/// `"child"`.
const CHILD_TEXT: u32 = 0x0108_6f50;
/// `008853a0` `Actor::GetHeight` (Xbox PDB name in the engine map; ST0).
const ACTOR_GET_HEIGHT: u32 = 0x0088_53a0;
/// `00567400` `TESObjectREFR::GetScale` (Xbox PDB name in the engine map;
/// ST0).
const REFERENCE_GET_SCALE: u32 = 0x0056_7400;
/// `110.0` (`double`).
const CHILD_HEIGHT_LIMIT: u32 = 0x0106_ebc8;

// Translated from 008d41f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether flag 4 of the word at +0x70 is set.
pub fn fn_008d41f0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x70)) & 4 != 0
}

// Translated from 008d40e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the actor counts as a child. With a
/// base form (vtable +0x37C): its flag 4 at +0x70 (`008d41f0`), or else
/// whether any of its first three parts (`00613a20`) has a name (part
/// vtable +0x14) containing "child" (`BSstristr`). When that is false and
/// `skip_height` is 0, an actor for which vtable +0x1D0 answers non-zero is
/// a child when its height divided by its scale, stored as a float, is
/// below 110.0.
pub fn fn_008d40e0(e: &mut Engine, this: Ptr, skip_height: u8) -> bool {
    let t = this.addr();
    let mut child = false;
    let base = e.vcall(t, 0x37c, &[]).u32();
    if base != 0 {
        child = fn_008d41f0(e, Ptr::new(base));
        if !child {
            let mut found = false;
            let mut index = 0i32;
            while !found && index < 3 {
                let word = e.call(ACTOR_0087F4C0, &args![this]).u32();
                let part = e.call(BASE_PART, &args![base, word, index]).u32();
                if part != 0 {
                    let name = e.vcall(part, 0x14, &[]).u32();
                    found = e.call(BS_STRISTR, &args![name, CHILD_TEXT]).u32() != 0;
                }
                index += 1;
            }
            child = found;
        }
    }
    if skip_height == 0 && !child && e.vcall(t, 0x1d0, &[]).u32() != 0 {
        let height = e.call(ACTOR_GET_HEIGHT, &args![this]).st0;
        // FSTP double: the height is kept as a double (ST0 already is one
        // here).
        let scale = e.call(REFERENCE_GET_SCALE, &args![this]).st0;
        let ratio = x87_m32(x87_store32(x87_div(height, scale)));
        let limit: f64 = e.global(CHILD_HEIGHT_LIMIT);
        if ratio < limit {
            child = true;
        }
    }
    child
}

// Translated from 008d4210 (decompiled, FalloutNV.exe 1.4.0.525)
/// After `008c1270`, slots 0x440 and 0x43C (their floats dropped) and,
/// with 3D (slot 0x1D0) and a process, the process gets the three nodes it
/// lacks: when its slot 0x790 / 0x798 answers 0 the node named by
/// `008d3f70` / `008d3f80` (`004ade00` on the 3D) is looked up in
/// `011d5bf8` (`00653270`) and handed to slot 0x794 / 0x79C; when slot
/// 0x7A0 answers 0 the node of `008d3f90` goes straight to slot 0x7A4.
pub fn fn_008d4210(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(0x008c_1270, &args![this]);
    e.vcall(at, 0x440, &[]);
    e.vcall(at, 0x43c, &[]);
    let node = e.vcall(at, 0x1d0, &[]).u32();
    if node == 0 || e.mem.u32(at + 0x68) == 0 {
        return;
    }
    for (test, set, name, look_up) in [
        (0x790, 0x794, 0x008d_3f70, true),
        (0x798, 0x79c, 0x008d_3f80, true),
        (0x7a0, 0x7a4, 0x008d_3f90, false),
    ] {
        let process = e.mem.u32(at + 0x68);
        if e.vcall(process, test, &[]).u32() != 0 {
            continue;
        }
        let text = e.call(name, &[]).u32();
        let mut found = e.call(0x004a_de00, &args![node, text]).u32();
        if look_up {
            found = e.call(0x0065_3270, &args![0x011d_5bf8u32, found]).u32();
        }
        let process = e.mem.u32(at + 0x68);
        e.vcall(process, set, &args![found]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_4210, fn_008d4210(Ptr)),
        entry!(0x008d_4380, fn_008d4380(Ptr)),
        entry!(0x008d_4360, fn_008d4360(Ptr) -> Ptr),
        entry!(0x008d_41f0, fn_008d41f0(Ptr) -> bool),
        entry!(0x008d_40e0, fn_008d40e0(Ptr, u8) -> bool),
    ]
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_008d4380_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x008d_50a0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_4380, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x008d_50a0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_008d4360_runs_the_base_initialiser() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x008d_4360,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x008d_5020, &[A::T(0)])],
            stores: &[],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An actor (vtable at `0x9100_0000`) whose base form is the word at
    /// +0x10, whose vtable +0x1D0 answers the word at +0x14, height the
    /// float at +0x18, scale the float at +0x1C; part `i` of the base is the
    /// word at base +0x10 + 4i, whose name is its word at +4.
    fn child_doubles(e: &mut Engine) {
        e.put_vtable(0x9100_0000, &[0; 0x80]);
        e.mem.set_u32(0x9100_0000 + 0x37c, 0x7000_037c);
        e.mem.set_u32(0x9100_0000 + 0x1d0, 0x7000_01d0);
        e.mem.set_u32(0x9100_0000 + 0x14, 0x7000_0014);
        e.register(0x7000_037c, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
        e.register(0x7000_01d0, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x14),
            ..Ret::default()
        });
        e.register(0x7000_0014, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(ACTOR_0087F4C0, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        e.register(BASE_PART, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10 + 4 * a[2]),
            ..Ret::default()
        });
        // The needle is "child"; a name is "child" when its word is 1.
        e.register(BS_STRISTR, |_, a| Ret {
            eax: (a[0] == 1) as u32,
            ..Ret::default()
        });
        e.register(ACTOR_GET_HEIGHT, |e, a| Ret {
            st0: e.mem.f32(a[0] + 0x18) as f64,
            ..Ret::default()
        });
        e.register(REFERENCE_GET_SCALE, |e, a| Ret {
            st0: e.mem.f32(a[0] + 0x1c) as f64,
            ..Ret::default()
        });
        e.map(CHILD_HEIGHT_LIMIT, 8);
        e.set_global(CHILD_HEIGHT_LIMIT, 110.0f64);
    }

    fn actor(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x9100_0000);
        let base = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x10, base);
        e.mem.set_f32(this + 0x1c, 1.0);
        e.mem.set_f32(this + 0x18, 200.0);
        (this, base)
    }

    #[test]
    fn fn_008d41f0_tests_flag_4_at_70() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x74);
        assert!(!e.call(0x008d_41f0, &args![this]).bool());
        e.mem.set_u32(this + 0x70, 4);
        assert!(e.call(0x008d_41f0, &args![this]).bool());
    }

    #[test]
    fn fn_008d40e0_checks_the_flag_the_part_names_and_the_height() {
        let mut e = Engine::new();
        child_doubles(&mut e);
        let (this, base) = actor(&mut e);
        assert!(!e.call(0x008d_40e0, &args![this, 0u32]).bool());
        e.mem.set_u32(base + 0x70, 4);
        assert!(e.call(0x008d_40e0, &args![this, 0u32]).bool(), "base flag");
        e.mem.set_u32(base + 0x70, 0);
        let part = e.mem.alloc(8);
        e.mem.set_u32(part, 0x9100_0000);
        e.mem.set_u32(part + 4, 1);
        e.mem.set_u32(base + 0x18, part);
        assert!(e.call(0x008d_40e0, &args![this, 1u32]).bool(), "third part");
        e.mem.set_u32(base + 0x18, 0);
        // Height test: 100 / 1 < 110 only when vtable +0x1D0 answers.
        e.mem.set_f32(this + 0x18, 100.0);
        assert!(!e.call(0x008d_40e0, &args![this, 0u32]).bool());
        e.mem.set_u32(this + 0x14, 1);
        assert!(e.call(0x008d_40e0, &args![this, 0u32]).bool());
        assert!(!e.call(0x008d_40e0, &args![this, 1u32]).bool(), "skipped");
        e.mem.set_f32(this + 0x18, 220.0);
        e.mem.set_f32(this + 0x1c, 2.0);
        assert!(
            !e.call(0x008d_40e0, &args![this, 0u32]).bool(),
            "110 is not below"
        );
        e.mem.set_f32(this + 0x1c, 0.0);
        e.mem.set_f32(this + 0x18, 0.0);
        assert!(!e.call(0x008d_40e0, &args![this, 0u32]).bool(), "NaN");
        e.mem.set_u32(this + 0x10, 0);
        e.mem.set_f32(this + 0x18, 10.0);
        e.mem.set_f32(this + 0x1c, 1.0);
        assert!(
            e.call(0x008d_40e0, &args![this, 0u32]).bool(),
            "no base form"
        );
    }
}

#[cfg(test)]
mod n028_tests {
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

    fn object(e: &mut Engine, size: u32, base: u32) -> u32 {
        let this = e.mem.alloc(size);
        let table = e.mem.alloc(0x800);
        for slot in (0..0x800).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
        }
        e.mem.set_u32(this, table);
        this
    }

    #[test]
    fn fn_008d4210_fills_the_missing_nodes() {
        let mut e = Engine::new();
        let this = object(&mut e, 0x200, 0x7100_0000);
        let process = object(&mut e, 0x100, 0x7300_0000);
        e.mem.set_u32(this + 0x68, process);
        for callee in [
            0x008c_1270,
            0x7100_0440,
            0x7100_043c,
            0x7300_0794,
            0x7300_079c,
            0x7300_07a4,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(0x7100_01d0, |_, _| Ret {
            eax: 0x3d00,
            ..Ret::default()
        });
        e.register(0x7300_0790, |_, _| Ret::default());
        e.register(0x7300_0798, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(0x7300_07a0, |_, _| Ret::default());
        e.register(0x008d_3f70, |_, _| Ret {
            eax: 0x70,
            ..Ret::default()
        });
        e.register(0x008d_3f80, |_, _| Ret {
            eax: 0x80,
            ..Ret::default()
        });
        e.register(0x008d_3f90, |_, _| Ret {
            eax: 0x90,
            ..Ret::default()
        });
        e.register(0x004a_de00, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register(0x0065_3270, |_, a| Ret {
            eax: a[1] + 0x100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x008d_4210, &[this]);
        assert_eq!(calls_to(&e, 0x7300_0794), vec![vec![process, 0x171]]);
        assert!(calls_to(&e, 0x7300_079c).is_empty());
        assert_eq!(calls_to(&e, 0x7300_07a4), vec![vec![process, 0x91]]);
        assert_eq!(calls_to(&e, 0x0065_3270), vec![vec![0x011d_5bf8, 0x71]]);
        // Without a process only the first calls run.
        e.mem.set_u32(this + 0x68, 0);
        e.call_log = Some(vec![]);
        e.call(0x008d_4210, &[this]);
        assert!(calls_to(&e, 0x7300_0790).is_empty());
        assert_eq!(calls_to(&e, 0x7100_01d0).len(), 1);
    }
}
