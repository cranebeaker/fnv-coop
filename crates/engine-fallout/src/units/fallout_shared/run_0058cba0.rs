//! `fallout shared/run_0058cba0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (callers include `HitMe` and `00403e60`; the class is not
/// confirmed): whether any bit of `mask` is set in the word at
/// `this + 0x58` (`NEG`/`SBB`/`NEG`: 1 or 0 in `EAX`); `RET 4`.
pub fn fn_0058cba0(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0x58) & mask != 0
}

/// `0066b0d0` (`thiscall` on a smart-pointer field, one word: stores the
/// object), `00559450` (`thiscall` on the field: the object it holds),
/// `NiNew`'s allocator (`00aa13e0`, `cdecl`: size) and the node constructor
/// `00a5ecb0` (`ninode.cpp`, `thiscall`, one word).
const POINTER_SET: u32 = 0x0066_b0d0;
const POINTER_GET: u32 = 0x0055_9450;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `TESObjectREFR::GetOrientation` (`0056fa00`, engine map; `thiscall`,
/// one word: the 0x24-byte answer), `0043fa80` (`thiscall` on the node:
/// sets its rotation), `0045bb80` (`thiscall`: a reference's position) and
/// `004f00e0` (`thiscall` on the node: sets its translation).
const GET_ORIENTATION: u32 = 0x0056_fa00;
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
const REFERENCE_POSITION: u32 = 0x0045_bb80;
const NODE_SET_TRANSLATE: u32 = 0x004f_00e0;
/// `009611e0` (`thiscall`: the next object to try when slot +0x10 answers
/// zero) and the byte both target setters set (`011e07c3`).
const NEXT_TARGET: u32 = 0x0096_11e0;
const TARGET_CHANGED: u32 = 0x011e_07c3;

/// The flag word (+0x44) the two readers test.
const CAMERA_FLAGS: u32 = 0x44;

/// A new node (0xAC bytes from `00aa13e0`, built by `00a5ecb0(0)`; null
/// when the allocation fails).
fn new_node(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
}

// Translated from 0058cbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058cbc0 - the FS:[0] exception frame (freeing the new node when its constructor throws) is not translated
/// No map name (`thiscall`, one word, `RET 4`; VATS calls it with its
/// target): with bit 0x1 of the flags at +0x44, stores `target` in the
/// pointer at +0x60. Otherwise, for a target, stores a new node there,
/// gives it the orientation of the reference at +0x68 when there is one
/// (`0056fa00` into a stack block, `0043fa80`) and the target's position
/// (`0045bb80`, `004f00e0`). Sets the byte at `011e07c3` in every case.
pub fn fn_0058cbc0(e: &mut Engine, this: Ptr, target: u32) {
    let at = this.addr();
    if fn_0058ccd0(e, this) {
        e.call(POINTER_SET, &args![at + 0x60, target]);
    } else if target != 0 {
        let node = new_node(e);
        e.call(POINTER_SET, &args![at + 0x60, node]);
        let reference = e.mem.u32(at + 0x68);
        if reference != 0 {
            e.with_stack(0x24, |e, block| {
                let orientation = e.call(GET_ORIENTATION, &args![reference, block]).u32();
                let node = e.call(POINTER_GET, &args![at + 0x60]).u32();
                e.call(NODE_SET_ROTATE, &args![node, orientation]);
            });
        }
        let position = e.call(REFERENCE_POSITION, &args![target]).u32();
        let node = e.call(POINTER_GET, &args![at + 0x60]).u32();
        e.call(NODE_SET_TRANSLATE, &args![node, position]);
    }
    e.mem.set_u8(TARGET_CHANGED, 1);
}

// Translated from 0058ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x1 of the flag word at +0x44 is set.
pub fn fn_0058ccd0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + CAMERA_FLAGS) & 1 != 0
}

// Translated from 0058ccf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058ccf0 - the FS:[0] exception frame (freeing the new node when its constructor throws) is not translated
/// No map name (`thiscall`, one word, `RET 4`; VATS calls it with its
/// second target): without a target, clears the pointers at +0x64 and
/// +0x70 and returns. Otherwise stores the target at +0x64 when bit 0x2 of
/// the flags at +0x44 is set, else a new node placed at the target's
/// position (`0045bb80`, `004f00e0`); then stores at +0x70 the first of the
/// target and its successors (`009611e0`) whose slot +0x10 answers
/// non-zero, or the target when none does, and sets the byte at
/// `011e07c3`.
pub fn fn_0058ccf0(e: &mut Engine, this: Ptr, target: u32) {
    let at = this.addr();
    if target == 0 {
        e.call(POINTER_SET, &args![at + 0x64, 0u32]);
        e.call(POINTER_SET, &args![at + 0x70, 0u32]);
        return;
    }
    if fn_0058ce30(e, this) {
        e.call(POINTER_SET, &args![at + 0x64, target]);
    } else {
        let node = new_node(e);
        e.call(POINTER_SET, &args![at + 0x64, node]);
        let position = e.call(REFERENCE_POSITION, &args![target]).u32();
        let node = e.call(POINTER_GET, &args![at + 0x64]).u32();
        e.call(NODE_SET_TRANSLATE, &args![node, position]);
    }
    let mut object = target;
    while object != 0 && e.vcall(object, 0x10, &args![]).u32() == 0 {
        object = e.call(NEXT_TARGET, &args![object]).u32();
    }
    let chosen = if object != 0 { object } else { target };
    e.call(POINTER_SET, &args![at + 0x70, chosen]);
    e.mem.set_u8(TARGET_CHANGED, 1);
}

// Translated from 0058ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x2 of the flag word at +0x44 is set.
pub fn fn_0058ce30(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + CAMERA_FLAGS) & 2 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_cba0, fn_0058cba0(Ptr, u32) -> bool),
        entry!(0x0058_cbc0, fn_0058cbc0(Ptr, u32)),
        entry!(0x0058_ccd0, fn_0058ccd0(Ptr) -> bool),
        entry!(0x0058_ccf0, fn_0058ccf0(Ptr, u32)),
        entry!(0x0058_ce30, fn_0058ce30(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_bit_of_the_mask_in_the_word_at_0x58() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 0x58, 0x8000_0010);
        for (mask, expected) in [(0x10, 1), (0x8000_0000, 1), (0x0f, 0), (0, 0)] {
            assert_eq!(e.call(0x0058_cba0, &[this, mask]).u32(), expected);
        }
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    const TARGETS: [u32; 7] = [
        POINTER_SET,
        NI_ALLOCATE,
        NODE_CONSTRUCT,
        GET_ORIENTATION,
        NODE_SET_ROTATE,
        NODE_SET_TRANSLATE,
        NEXT_TARGET,
    ];

    /// Pointer fields hold their object; the allocator answers 0x8000, the
    /// node constructor its block, positions the reference plus 0x10, the
    /// orientation its block. Objects answer the word at +4 at slot +0x10
    /// and their successor is the word at +8.
    fn setup(e: &mut Engine) {
        e.map(0x011e_0000, 0x1000);
        e.register(POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(NI_ALLOCATE, |_, _| Ret {
            eax: 0x8000,
            ..Ret::default()
        });
        e.register(NODE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(GET_ORIENTATION, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(REFERENCE_POSITION, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        e.register(NODE_SET_ROTATE, |_, _| Ret::default());
        e.register(NODE_SET_TRANSLATE, |_, _| Ret::default());
        e.register(NEXT_TARGET, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(0x0090_0010, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
    }

    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0090_0010);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn fn_0058cbc0_sets_the_view_target() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = e.mem.alloc(0x80);
        let target = object(&mut e);
        // The flag: the target as is.
        e.mem.set_u32(this + 0x44, 1);
        e.call_log = Some(vec![]);
        e.call(0x0058_cbc0, &[this, target]);
        assert_eq!(
            calls_to(&mut e, &TARGETS),
            vec![(POINTER_SET, vec![this + 0x60, target])]
        );
        assert_eq!(e.mem.u8(TARGET_CHANGED), 1);
        // No flag, no target: nothing but the byte.
        e.mem.set_u32(this + 0x44, 2);
        e.mem.set_u8(TARGET_CHANGED, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_cbc0, &[this, 0]);
        assert!(calls_to(&mut e, &TARGETS).is_empty());
        assert_eq!(e.mem.u8(TARGET_CHANGED), 1);
        // A new node, without and with the reference at +0x68.
        e.call_log = Some(vec![]);
        e.call(0x0058_cbc0, &[this, target]);
        assert_eq!(
            calls_to(&mut e, &TARGETS),
            vec![
                (NI_ALLOCATE, vec![0xac]),
                (NODE_CONSTRUCT, vec![0x8000, 0]),
                (POINTER_SET, vec![this + 0x60, 0x8000]),
                (NODE_SET_TRANSLATE, vec![0x8000, target + 0x10]),
            ]
        );
        e.mem.set_u32(this + 0x68, 0x6800);
        e.call_log = Some(vec![]);
        e.call(0x0058_cbc0, &[this, target]);
        let log = calls_to(&mut e, &TARGETS);
        let block = log[3].1[1];
        assert_eq!(log[3], (GET_ORIENTATION, vec![0x6800, block]));
        assert_eq!(log[4], (NODE_SET_ROTATE, vec![0x8000, block]));
        assert_eq!(log[5], (NODE_SET_TRANSLATE, vec![0x8000, target + 0x10]));
    }

    #[test]
    fn fn_0058ccf0_sets_the_second_target() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = e.mem.alloc(0x80);
        let first = object(&mut e);
        let second = object(&mut e);
        e.mem.set_u32(first + 8, second);
        e.call_log = Some(vec![]);
        e.call(0x0058_ccf0, &[this, 0]);
        assert_eq!(
            calls_to(&mut e, &TARGETS),
            vec![
                (POINTER_SET, vec![this + 0x64, 0]),
                (POINTER_SET, vec![this + 0x70, 0]),
            ]
        );
        assert_eq!(e.mem.u8(TARGET_CHANGED), 0);
        // The flag, no answering object: the target at +0x70.
        e.mem.set_u32(this + 0x44, 2);
        e.call_log = Some(vec![]);
        e.call(0x0058_ccf0, &[this, first]);
        assert_eq!(
            calls_to(&mut e, &TARGETS),
            vec![
                (POINTER_SET, vec![this + 0x64, first]),
                (NEXT_TARGET, vec![first]),
                (NEXT_TARGET, vec![second]),
                (POINTER_SET, vec![this + 0x70, first]),
            ]
        );
        assert_eq!(e.mem.u8(TARGET_CHANGED), 1);
        // No flag: a new node; the successor answers.
        e.mem.set_u32(this + 0x44, 1);
        e.mem.set_u32(second + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0058_ccf0, &[this, first]);
        assert_eq!(
            calls_to(&mut e, &TARGETS),
            vec![
                (NI_ALLOCATE, vec![0xac]),
                (NODE_CONSTRUCT, vec![0x8000, 0]),
                (POINTER_SET, vec![this + 0x64, 0x8000]),
                (NODE_SET_TRANSLATE, vec![0x8000, first + 0x10]),
                (NEXT_TARGET, vec![first]),
                (POINTER_SET, vec![this + 0x70, second]),
            ]
        );
    }

    #[test]
    fn fn_0058ccd0_and_0058ce30_read_the_flags() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        for (flags, one, two) in [
            (0u32, false, false),
            (1, true, false),
            (0x8000_0002, false, true),
        ] {
            e.mem.set_u32(this + 0x44, flags);
            assert_eq!(e.call(0x0058_ccd0, &[this]).bool(), one);
            assert_eq!(e.call(0x0058_ce30, &[this]).bool(), two);
        }
    }
}
