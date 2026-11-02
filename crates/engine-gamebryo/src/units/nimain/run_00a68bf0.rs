//! `NiMain/run_00a68bf0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::NiAVObject;

/// `NiAVObject::m_kLocal` (+0x34) and `m_kWorld` (+0x68): `NiTransform`s of 0x34 bytes (13
/// words) on PC.
const AV_OBJECT_LOCAL: u32 = 0x34;
const AV_OBJECT_WORLD: u32 = 0x68;
const TRANSFORM_WORDS: u32 = 13;
/// The flag (bit 9 of `m_uFlags`) that makes the object take its parent's world transform
/// unchanged.
const FLAG_WORLD_IS_PARENTS: u32 = 1 << 9;
/// `0062c250` (`__thiscall` on a transform, two words: the result, the other transform):
/// `NiTransform::operator*`'s shape, answering the result.
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;

/// Copies a transform (13 words, `MOVSD.REP`).
fn copy_transform(e: &mut Engine, to: u32, from: u32) {
    for i in 0..TRANSFORM_WORDS {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 00a68bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall`, no words; the body `NiAVObject::UpdateWorldData` runs without a
/// collision object): the world transform (+0x68) becomes the local one (+0x34) without a
/// parent, the parent's world transform when bit 9 of the flags is set, and otherwise the
/// parent's world transform times the local one (`0062c250` into a stack temporary), copied
/// in.
pub fn fn_00a68bf0(e: &mut Engine, this: Ptr<NiAVObject>) {
    let base = this.addr();
    let parent = e.get(this, NiAVObject::m_pkParent);
    if parent == 0 {
        copy_transform(e, base + AV_OBJECT_WORLD, base + AV_OBJECT_LOCAL);
        return;
    }
    if e.get(this, NiAVObject::m_uFlags) & FLAG_WORLD_IS_PARENTS != 0 {
        copy_transform(e, base + AV_OBJECT_WORLD, parent + AV_OBJECT_WORLD);
        return;
    }
    e.with_stack(0x34, |e, product| {
        let result = e
            .call(
                TRANSFORM_MULTIPLY,
                &args![parent + AV_OBJECT_WORLD, product, base + AV_OBJECT_LOCAL],
            )
            .u32();
        copy_transform(e, base + AV_OBJECT_WORLD, result);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a68bf0, fn_00a68bf0(Ptr<NiAVObject>))]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn transform(e: &mut Engine, object: u32, at: u32, first: u32) {
        for i in 0..TRANSFORM_WORDS {
            e.mem.set_u32(object + at + 4 * i, first + i);
        }
    }

    fn world(e: &Engine, object: u32) -> Vec<u32> {
        (0..TRANSFORM_WORDS)
            .map(|i| e.mem.u32(object + AV_OBJECT_WORLD + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00a68bf0_takes_the_local_parent_or_product_transform() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x9c);
        let parent = e.mem.alloc(0x9c);
        transform(&mut e, object, AV_OBJECT_LOCAL, 0x100);
        transform(&mut e, parent, AV_OBJECT_WORLD, 0x200);
        e.call(0x00a68bf0, &args![object]);
        assert_eq!(world(&e, object), (0x100..0x10d).collect::<Vec<u32>>());
        e.mem.set_u32(object + 0x18, parent);
        e.mem.set_u32(object + 0x30, FLAG_WORLD_IS_PARENTS);
        e.call(0x00a68bf0, &args![object]);
        assert_eq!(world(&e, object), (0x200..0x20d).collect::<Vec<u32>>());
        e.mem.set_u32(object + 0x30, 0);
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x100);
            for i in 0..13 {
                e.mem.set_u32(a[1] + 4 * i, 0x300 + i);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.call(0x00a68bf0, &args![object]);
        assert_eq!(world(&e, object), (0x300..0x30d).collect::<Vec<u32>>());
    }
}
