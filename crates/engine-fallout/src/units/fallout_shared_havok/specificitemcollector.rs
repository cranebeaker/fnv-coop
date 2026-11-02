//! `fallout shared/havok/specificitemcollector.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! x87 note: the hit-fraction test is a `FCOMPP` followed by a test of the
//! C0 flag, which is set for "less than" and for unordered (NaN) operands,
//! so the early-out is taken for NaN as well.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `SpecificItemCollector` (Xbox PDB), 0x80 bytes: an
    /// `hkpClosestRayHitCollector` at `+0` (vtable, `m_earlyOutHitFraction`
    /// at `+4`, the ray hit at `+0x10`), then the filter and the least hit
    /// fraction. The PC code reads both at the same offsets as the PDB.
    pub struct SpecificItemCollector: 0x80 {
        /// `iFilter` (Xbox PDB): a `CFilter`, one word.
        0x70 iFilter: u32,
        /// `fMinHitFraction` (Xbox PDB).
        0x74 fMinHitFraction: f32,
    }
}

/// `00624020(body)`: follows the word at `+0x0c` to the last object.
const ROOT_OF: u32 = 0x0062_4020;
/// `0043b540(root)`: `0044ddc0` on `root + 0x14`, the word at `root + 0x1c`
/// (the collision filter of the root body).
const FILTER_WORD_OF: u32 = 0x0043_b540;
/// The `CFilter` constructor (`008c71b0(this, word)`, `RET 4`; the engine
/// map names it `BGSChangeFlags::IsForcedChange`, a folded match).
const FILTER_CONSTRUCT: u32 = 0x008c_71b0;
/// `CFilter` layer: the low 7 bits of the word (`0043b4d0`).
const FILTER_LAYER: u32 = 0x0043_b4d0;
/// `CFilter` group: the high 16 bits of the word (`004a3a20`).
const FILTER_GROUP: u32 = 0x004a_3a20;
/// `hkpClosestRayHitCollector::addRayHit` (engine map name of `00ca9440`;
/// `this`, then `(body, hitInfo)`, `RET 8`).
const CLOSEST_ADD_RAY_HIT: u32 = 0x00ca_9440;
/// Offset of the hit fraction in the hit information.
const HIT_FRACTION: u32 = 0x10;

// Translated from 0062a320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpecificItemCollector::addRayHit` (Xbox PDB name): finds the root of
/// the hit body's parent chain (`00624020`); when there is one, builds a
/// `CFilter` from the root's collision filter word and, if its layer equals
/// the collector's layer, requires its group to equal the collector's group
/// too (a hit whose layer is the same and group differs is dropped).
/// Then, unless the hit's fraction is less than `fMinHitFraction` (or
/// either is a NaN), passes the hit on to the closest-hit collector
/// (`00ca9440`). `body` and `hit_info` are the two stack words.
pub fn specific_item_collector_add_ray_hit(
    e: &mut Engine,
    this: Ptr<SpecificItemCollector>,
    body: u32,
    hit_info: u32,
) {
    let root = e.call(ROOT_OF, &args![body]).u32();
    if root == 0 {
        return;
    }
    let word = e.call(FILTER_WORD_OF, &args![root]).u32();
    let own_filter = this.addr() + SpecificItemCollector::iFilter.off;
    let keep = e.with_stack(4, |e, filter| {
        e.call(FILTER_CONSTRUCT, &args![filter, word]);
        let layer = e.call(FILTER_LAYER, &args![filter]).u32();
        let own_layer = e.call(FILTER_LAYER, &args![own_filter]).u32();
        if layer == own_layer {
            let group = e.call(FILTER_GROUP, &args![filter]).u32();
            let own_group = e.call(FILTER_GROUP, &args![own_filter]).u32();
            if group != own_group {
                return false;
            }
        }
        true
    });
    if !keep {
        return;
    }
    let minimum = e.get(this, SpecificItemCollector::fMinHitFraction);
    let fraction = e.mem.f32(hit_info + HIT_FRACTION);
    if fraction >= minimum {
        e.call(CLOSEST_ADD_RAY_HIT, &args![this, body, hit_info]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0062_a320,
        specific_item_collector_add_ray_hit(Ptr<SpecificItemCollector>, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: u32 = 0x0610_0000;

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A collector with filter word `own_filter` and least fraction
    /// `minimum`; the root of every body is `root` and its filter word
    /// `root_word`. `FILTER_CONSTRUCT` stores the word like the real one,
    /// the layer and group getters read it like the real ones.
    fn setup(
        own_filter: u32,
        minimum: f32,
        root: u32,
        root_word: u32,
    ) -> (Engine, Ptr<SpecificItemCollector>) {
        let mut e = Engine::new();
        let this = e.new_object::<SpecificItemCollector>();
        e.set(this, SpecificItemCollector::iFilter, own_filter);
        e.set(this, SpecificItemCollector::fMinHitFraction, minimum);
        e.register_double(ROOT_OF, move |_, _| root.into_ret());
        e.register_double(FILTER_WORD_OF, move |_, _| root_word.into_ret());
        e.register(FILTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(FILTER_LAYER, |e, a| (e.mem.u32(a[0]) & 0x7f).into_ret());
        e.register(FILTER_GROUP, |e, a| (e.mem.u32(a[0]) >> 16).into_ret());
        e.register(CLOSEST_ADD_RAY_HIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, this)
    }

    fn hit(e: &mut Engine, fraction: f32) -> u32 {
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0x10, fraction);
        info
    }

    #[test]
    fn no_root_drops_the_hit() {
        let (mut e, this) = setup(0, 0.0, 0, 0);
        let info = hit(&mut e, 0.5);
        e.call(0x0062_a320, &args![this, BODY, info]);
        assert_eq!(logged(&e, ROOT_OF), vec![vec![BODY]]);
        assert!(logged(&e, FILTER_WORD_OF).is_empty());
        assert!(logged(&e, CLOSEST_ADD_RAY_HIT).is_empty());
    }

    #[test]
    fn a_different_layer_forwards_whatever_the_group() {
        // Own layer 3, group 5; root layer 4, group 9.
        let (mut e, this) = setup(0x0005_0003, 0.25, BODY + 0x40, 0x0009_0004);
        let info = hit(&mut e, 0.5);
        e.call(0x0062_a320, &args![this, BODY, info]);
        assert_eq!(logged(&e, FILTER_WORD_OF), vec![vec![BODY + 0x40]]);
        assert!(logged(&e, FILTER_GROUP).is_empty());
        assert_eq!(
            logged(&e, CLOSEST_ADD_RAY_HIT),
            vec![vec![this.addr(), BODY, info]]
        );
        // The filter object is the local, built from the root's word, and
        // the collector's own filter is the one at +0x70.
        let built = logged(&e, FILTER_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], 0x0009_0004);
        let layers = logged(&e, FILTER_LAYER);
        assert_eq!(layers, vec![vec![built[0][0]], vec![this.addr() + 0x70]]);
    }

    #[test]
    fn the_same_layer_needs_the_same_group() {
        let (mut e, this) = setup(0x0005_0003, 0.25, BODY + 0x40, 0x0009_0003);
        let info = hit(&mut e, 0.5);
        e.call(0x0062_a320, &args![this, BODY, info]);
        assert!(logged(&e, CLOSEST_ADD_RAY_HIT).is_empty());
        let groups = logged(&e, FILTER_GROUP);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[1], vec![this.addr() + 0x70]);

        let (mut e, this) = setup(0x0005_0003, 0.25, BODY + 0x40, 0x0005_0003);
        let info = hit(&mut e, 0.5);
        e.call(0x0062_a320, &args![this, BODY, info]);
        assert_eq!(logged(&e, CLOSEST_ADD_RAY_HIT).len(), 1);
    }

    #[test]
    fn hit_fraction_below_the_minimum_is_dropped() {
        let (mut e, this) = setup(0, 0.5, BODY + 0x40, 0x0001_0001);
        let info = hit(&mut e, 0.25);
        e.call(0x0062_a320, &args![this, BODY, info]);
        assert!(logged(&e, CLOSEST_ADD_RAY_HIT).is_empty());
    }

    #[test]
    fn hit_fraction_equal_to_the_minimum_is_forwarded() {
        let (mut e, this) = setup(0, 0.5, BODY + 0x40, 0x0001_0001);
        let info = hit(&mut e, 0.5);
        e.call(0x0062_a320, &args![this, BODY, info]);
        assert_eq!(logged(&e, CLOSEST_ADD_RAY_HIT).len(), 1);
    }

    #[test]
    fn nan_on_either_side_is_dropped() {
        let (mut e, this) = setup(0, f32::NAN, BODY + 0x40, 0x0001_0001);
        let info = hit(&mut e, 0.5);
        e.call(0x0062_a320, &args![this, BODY, info]);
        assert!(logged(&e, CLOSEST_ADD_RAY_HIT).is_empty());

        let (mut e, this) = setup(0, 0.5, BODY + 0x40, 0x0001_0001);
        let info = hit(&mut e, f32::NAN);
        e.call(0x0062_a320, &args![this, BODY, info]);
        assert!(logged(&e, CLOSEST_ADD_RAY_HIT).is_empty());
    }
}
