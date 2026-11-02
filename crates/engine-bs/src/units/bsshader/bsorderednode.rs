//! `BSShader/bsorderednode.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiNode::LoadBinary` (`00a5dd40`, `thiscall`, one word: the stream).
const NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
/// `NiSphere::LoadBinary` (`00a69ac0`, `thiscall` on the sphere, one word).
const SPHERE_LOAD_BINARY: u32 = 0x00a6_9ac0;
/// The stream's `+4` word above which the flag byte is in the file (compared
/// unsigned with 0x17).
const LAST_VERSION_WITHOUT_FLAG: u32 = 0x17;

use crate::units::bsshader::bsshaderproperty::{
    viewer_strings_add, VIEWER_STRING, VIEWER_STRING_FLAG,
};

// Translated from 00bc2e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOrderedNode::LoadBinary` (Xbox PDB): the node part, the bound sphere at
/// `+0xAC`; then, when the stream's `+4` word is above 0x17, one byte read
/// into `+0xBC` from the stream's input file (`+0x24c`: its read function
/// `+8`, `cdecl (file, dest, 1, &count, 1)`, the result added to its
/// position `+4`; the count word is the argument slot), else that byte is 1.
pub fn bs_ordered_node_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NODE_LOAD_BINARY, &args![this, stream]);
    e.call(SPHERE_LOAD_BINARY, &args![this.addr() + 0xac, stream]);
    if e.mem.u32(stream.addr() + 4) <= LAST_VERSION_WITHOUT_FLAG {
        e.mem.set_u8(this.addr() + 0xbc, 1);
        return;
    }
    let file = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(file + 8);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        let got = e
            .call(read, &args![file, this.addr() + 0xbc, 1u32, count, 1u32])
            .u32();
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(got));
    });
}

/// The calls the other `BSOrderedNode` methods make: the `NiNode` copy (`00c5fb60`,
/// `thiscall`: destination, clone process), the world-transform point mapping (`00524c40`,
/// `thiscall` on the transform: the output, the point), `BSShaderAccumulator::
/// StartGroupingAlphas` (`00b63500`, `thiscall`: the bound, 0) and `StopGroupingAlphas`
/// (`00b63090`), `NiNode::OnVisible` (`00a5dbe0`, `thiscall`, the culler), the default bound
/// (`011f4288`), the `BSOrderedNode` constructor (`00bc2bd0`), the allocation (`00aa13e0`),
/// `NiNode::SaveBinary` (`00a5e880`), `NiSphere::SaveBinary` (`00eb5410`, `thiscall` on the
/// sphere), `NiNode::GetViewerStrings` (`00a5ea70`), `NiBound::GetViewerString` (`00a7fb20`,
/// `thiscall` on the bound, the label) and the class-name record (`01201f84`).
const NODE_COPY_MEMBERS_00C5FB60: u32 = 0x00c5_fb60;
const TRANSFORM_POINT_00524C40: u32 = 0x0052_4c40;
const START_GROUPING_ALPHAS: u32 = 0x00b6_3500;
const STOP_GROUPING_ALPHAS: u32 = 0x00b6_3090;
const NODE_ON_VISIBLE: u32 = 0x00a5_dbe0;
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
const ORDERED_NODE_CONSTRUCTOR_00BC2BD0: u32 = 0x00bc_2bd0;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NODE_SAVE_BINARY: u32 = 0x00a5_e880;
const SPHERE_SAVE_BINARY: u32 = 0x00eb_5410;
const NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
const BOUND_GET_VIEWER_STRING: u32 = 0x00a7_fb20;
const CLASS_RTTI_01201F84: u32 = 0x0120_1f84;
/// The labels (`"bHasStaticBound"`, `"Alpha Sort Bound"`).
const LABEL_HAS_STATIC_BOUND: u32 = 0x010b_a848;
const LABEL_ALPHA_SORT_BOUND: u32 = 0x010b_a834;

// Translated from 00bc2c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOrderedNode::CopyMembers` (by the vtable use; unnamed in the engine map), `thiscall`
/// with two words (the destination, the clone process): the `NiNode` copy (`00c5fb60`), then
/// the static bound (`+0xac` centre, `+0xb8` radius through two `FLD`/`FSTP`s) and its flag
/// byte `+0xbc`.
pub fn fn_00bc2c50(e: &mut Engine, this: Ptr, dest: u32, process: u32) {
    let base = this.addr();
    e.call(NODE_COPY_MEMBERS_00C5FB60, &args![this, dest, process]);
    let radius = x87_quiet_bits(x87_quiet_bits(e.mem.u32(base + 0xb8)));
    let v = e.mem.u32(base + 0xac);
    e.mem.set_u32(dest.wrapping_add(0xac), v);
    let v = e.mem.u32(base + 0xb0);
    e.mem.set_u32(dest.wrapping_add(0xb0), v);
    let v = e.mem.u32(base + 0xb4);
    e.mem.set_u32(dest.wrapping_add(0xb8), radius);
    e.mem.set_u32(dest.wrapping_add(0xb4), v);
    let flag = e.mem.u8(base + 0xbc);
    e.mem.set_u8(dest.wrapping_add(0xbc), flag);
}

/// The world bound of the static bound: its centre mapped through the world transform
/// (`+0x68`, `00524c40`) and its radius times the world scale (`+0x98`, one `FMUL`), at
/// `sphere` (centre, then radius).
fn world_static_bound(e: &mut Engine, this: u32, scratch: u32, sphere: u32) {
    e.call(
        TRANSFORM_POINT_00524C40,
        &args![this + 0x68, scratch, this + 0xac],
    );
    let radius = x87_store32(x87_mul(
        x87_m32(e.mem.u32(this + 0xb8)),
        x87_m32(e.mem.u32(this + 0x98)),
    ));
    for k in 0..3u32 {
        let v = e.mem.u32(scratch + 4 * k);
        e.mem.set_u32(sphere + 4 * k, v);
    }
    e.mem.set_u32(sphere + 0xc, radius);
}

// Translated from 00bc2d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOrderedNode::OnVisible` (engine map; the body matches): `thiscall` with one word (the
/// culler). Starts grouping alphas on the culler's accumulator (`+0xc4`, `00b63500` with 0)
/// around a bound: the world static bound ([`world_static_bound`], in a local) when the flag
/// byte `+0xbc` is set, else the node's `+0x20` (the default bound `011f4288` when null);
/// then `NiNode::OnVisible` and `StopGroupingAlphas`.
pub fn bs_ordered_node_on_visible(e: &mut Engine, this: Ptr, culler: u32) {
    let base = this.addr();
    let accumulator = e.mem.u32(culler.wrapping_add(0xc4));
    e.with_stack(0x1c, |e, locals| {
        let bound = if e.mem.u8(base + 0xbc) != 0 {
            let sphere = locals.addr() + 0xc;
            world_static_bound(e, base, locals.addr(), sphere);
            sphere
        } else {
            let bound = e.mem.u32(base + 0x20);
            if bound != 0 {
                bound
            } else {
                DEFAULT_BOUND_011F4288
            }
        };
        e.call(START_GROUPING_ALPHAS, &args![accumulator, bound, 0u32]);
    });
    e.call(NODE_ON_VISIBLE, &args![this, culler]);
    e.call(STOP_GROUPING_ALPHAS, &args![accumulator]);
}

// Translated from 00bc2e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc2e00 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`CreateClone` by its body), `thiscall` with one word (the clone
/// process): a new 0xc0-byte `BSOrderedNode` (`00aa13e0`, `00bc2bd0`; null when the
/// allocation fails) and [`fn_00bc2c50`] into it. Answers it.
pub fn fn_00bc2e00(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0xc0u32]).u32();
    let copy = if block != 0 {
        e.call(ORDERED_NODE_CONSTRUCTOR_00BC2BD0, &args![block])
            .u32()
    } else {
        0
    };
    fn_00bc2c50(e, this, copy, process);
    copy
}

// Translated from 00bc2ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOrderedNode::SaveBinary` (engine map; the body matches): `thiscall`, one word (the
/// stream): `NiNode::SaveBinary`, the static bound (`NiSphere::SaveBinary` on `+0xac`) and the
/// flag byte `+0xbc` (one byte through the stream's file, `+0x250`: its write function `+0xc`,
/// `cdecl (file, source, 1, &count, 1)`, the result added to its position `+4`; the count
/// word is the argument slot).
pub fn bs_ordered_node_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(NODE_SAVE_BINARY, &args![this, stream]);
    e.call(SPHERE_SAVE_BINARY, &args![this.addr() + 0xac, stream]);
    let file = e.mem.u32(stream.wrapping_add(0x250));
    let write = e.mem.u32(file.wrapping_add(0xc));
    let written = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        e.call(write, &args![file, this.addr() + 0xbc, 1u32, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(file.wrapping_add(4));
    e.mem
        .set_u32(file.wrapping_add(4), position.wrapping_add(written));
}

// Translated from 00bc2f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOrderedNode::GetViewerStrings` (engine map; the body matches): `thiscall`, one word
/// (the strings): the `NiNode` strings, the class name, the flag byte `+0xbc`
/// (`"bHasStaticBound"`) and the world static bound ([`world_static_bound`], through
/// `NiBound::GetViewerString` with `"Alpha Sort Bound"`), each added inline.
pub fn bs_ordered_node_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let base = this.addr();
    e.call(NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI_01201F84);
    let text = e.call(VIEWER_STRING, &args![class_name]).u32();
    viewer_strings_add(e, strings, text);
    let flag = u32::from(e.mem.u8(base + 0xbc));
    let text = e
        .call(VIEWER_STRING_FLAG, &args![LABEL_HAS_STATIC_BOUND, flag])
        .u32();
    viewer_strings_add(e, strings, text);
    let text = e.with_stack(0x1c, |e, locals| {
        let sphere = locals.addr() + 0xc;
        world_static_bound(e, base, locals.addr(), sphere);
        e.call(
            BOUND_GET_VIEWER_STRING,
            &args![sphere, LABEL_ALPHA_SORT_BOUND],
        )
        .u32()
    });
    viewer_strings_add(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_2e80, bs_ordered_node_load_binary(Ptr, Ptr)),
        entry!(0x00bc_2c50, fn_00bc2c50(Ptr, u32, u32)),
        entry!(0x00bc_2d70, bs_ordered_node_on_visible(Ptr, u32)),
        entry!(0x00bc_2e00, fn_00bc2e00(Ptr, u32) -> u32),
        entry!(0x00bc_2ee0, bs_ordered_node_save_binary(Ptr, u32)),
        entry!(0x00bc_2f30, bs_ordered_node_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_binary_reads_the_flag_only_from_newer_streams() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(NODE_LOAD_BINARY, |_, _| Ret::default());
        e.register(SPHERE_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            e.mem.set_u8(a[1], 0);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 4, 0x17);
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        e.call(0x00bc_2e80, &args![this, stream]);
        assert_eq!(e.mem.u8(this + 0xbc), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(SPHERE_LOAD_BINARY, vec![this + 0xac, stream])));
        assert!(log.contains(&(NODE_LOAD_BINARY, vec![this, stream])));
        e.mem.set_u32(stream + 4, 0x18);
        e.call(0x00bc_2e80, &args![this, stream]);
        assert_eq!(e.mem.u8(this + 0xbc), 0);
        assert_eq!(e.mem.u32(file + 4), 1);
    }
}

#[cfg(test)]
mod n012_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A node with a static bound (centre 1, 2, 3, radius 2.0) and a world scale of 1.5; the
    /// transform double offsets the centre by 10.
    fn node(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xc0);
        for (k, v) in [1.0f32, 2.0, 3.0, 2.0].iter().enumerate() {
            e.mem.set_f32(this + 0xac + 4 * k as u32, *v);
        }
        e.mem.set_f32(this + 0x98, 1.5);
        e.register(TRANSFORM_POINT_00524C40, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[2] + 4 * k) + 10.0;
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            Ret::default()
        });
        this
    }

    #[test]
    fn copy_members_copies_the_static_bound() {
        let mut e = Engine::new();
        e.register(NODE_COPY_MEMBERS_00C5FB60, |_, _| Ret::default());
        let this = node(&mut e);
        e.mem.set_u32(this + 0xb8, 0x7f80_0001);
        e.mem.set_u8(this + 0xbc, 1);
        let dest = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        e.call(0x00bc_2c50, &args![this, dest, 9u32]);
        assert_eq!(
            calls(&e, NODE_COPY_MEMBERS_00C5FB60),
            vec![vec![this, dest, 9]]
        );
        assert_eq!(e.mem.f32(dest + 0xb0), 2.0);
        assert_eq!(e.mem.u32(dest + 0xb8), 0x7fc0_0001);
        assert_eq!(e.mem.u8(dest + 0xbc), 1);
    }

    #[test]
    fn on_visible_groups_alphas_around_the_chosen_bound() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.register(START_GROUPING_ALPHAS, |e, a| {
            assert_eq!(a[2], 0);
            if a[1] < 0x2000_0000 {
                return Ret::default();
            }
            // The local world sphere: centre x 1 + 10, radius 2 * 1.5.
            assert_eq!((e.mem.f32(a[1]), e.mem.f32(a[1] + 0xc)), (11.0, 3.0));
            Ret::default()
        });
        e.register(NODE_ON_VISIBLE, |_, _| Ret::default());
        e.register(STOP_GROUPING_ALPHAS, |_, _| Ret::default());
        let this = node(&mut e);
        let culler = e.mem.alloc(0xd0);
        e.mem.set_u32(culler + 0xc4, 0xacc);
        e.mem.set_u8(this + 0xbc, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bc_2d70, &args![this, culler]);
        e.mem.set_u8(this + 0xbc, 0);
        e.call(0x00bc_2d70, &args![this, culler]);
        e.mem.set_u32(this + 0x20, 0x4440);
        e.call(0x00bc_2d70, &args![this, culler]);
        let starts = calls(&e, START_GROUPING_ALPHAS);
        assert_eq!(starts[0][0], 0xacc);
        assert_eq!(starts[1][1], DEFAULT_BOUND_011F4288);
        assert_eq!(starts[2][1], 0x4440);
        assert_eq!(calls(&e, NODE_ON_VISIBLE), vec![vec![this, culler]; 3]);
        assert_eq!(calls(&e, STOP_GROUPING_ALPHAS), vec![vec![0xacc]; 3]);
        let sphere_log = e.call_log.as_ref().unwrap().iter().any(|(a, w)| {
            *a == TRANSFORM_POINT_00524C40 && w[0] == this + 0x68 && w[2] == this + 0xac
        });
        assert!(sphere_log);
    }

    #[test]
    fn world_static_bound_scales_the_radius() {
        let mut e = Engine::new();
        let this = node(&mut e);
        let scratch = e.mem.alloc(0x1c);
        world_static_bound(&mut e, this, scratch, scratch + 0xc);
        assert_eq!(e.mem.f32(scratch + 0xc), 11.0);
        assert_eq!(e.mem.f32(scratch + 0x14), 13.0);
        assert_eq!(e.mem.f32(scratch + 0x18), 3.0);
    }

    #[test]
    fn create_clone_copies_into_a_new_node() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ORDERED_NODE_CONSTRUCTOR_00BC2BD0, |_, a| a[0].into_ret());
        e.register(NODE_COPY_MEMBERS_00C5FB60, |_, _| Ret::default());
        let this = node(&mut e);
        e.call_log = Some(vec![]);
        let copy = e.call(0x00bc_2e00, &args![this, 5u32]).u32();
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0xc0]]);
        assert_eq!(
            calls(&e, NODE_COPY_MEMBERS_00C5FB60),
            vec![vec![this, copy, 5]]
        );
        assert_eq!(e.mem.f32(copy + 0xb8), 2.0);
    }

    #[test]
    fn save_binary_writes_the_flag_after_the_sphere() {
        const WRITE: u32 = 0x0100_0020;
        let mut e = Engine::new();
        e.register(NODE_SAVE_BINARY, |_, _| Ret::default());
        e.register(SPHERE_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            1u32.into_ret()
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, file);
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        e.call(0x00bc_2ee0, &args![this, stream]);
        assert_eq!(
            calls(&e, SPHERE_SAVE_BINARY),
            vec![vec![this + 0xac, stream]]
        );
        assert_eq!(calls(&e, WRITE)[0][1], this + 0xbc);
        assert_eq!(e.mem.u32(file + 4), 1);
    }

    #[test]
    fn get_viewer_strings_adds_flag_and_bound() {
        let mut e = Engine::new();
        e.map(0x0120_1000, 0x1000);
        e.mem.set_u32(CLASS_RTTI_01201F84, 0x00c1_a500);
        e.register(NODE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, a| a[0].into_ret());
        e.register(VIEWER_STRING_FLAG, |_, a| (0x100 + a[1]).into_ret());
        e.register(BOUND_GET_VIEWER_STRING, |e, a| {
            assert_eq!(a[1], LABEL_ALPHA_SORT_BOUND);
            (e.mem.f32(a[0] + 0xc) as u32).into_ret()
        });
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let v = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], v);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let this = node(&mut e);
        e.mem.set_u8(this + 0xbc, 1);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x20);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00bc_2f30, &args![this, strings]);
        assert_eq!(
            (e.mem.u32(base), e.mem.u32(base + 4), e.mem.u32(base + 8)),
            (0x00c1_a500, 0x101, 3)
        );
    }
}
