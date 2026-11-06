//! The typed migration's seam as a crate above the core uses it: a family it provides itself,
//! `typed_call!` and the differential test, through their public paths only.

#![cfg(feature = "test-support")]

use engine_core::prelude::*;
use engine_core::typed::diff::{Diff, Input};
use engine_core::typed::{provide, Family, Threads};

engine_core::assemble!();

const FAMILY: FamilyId = FamilyId::new(7);
const LEAF: u32 = 0x00ff_1000;
const COUNTER: u32 = 0x0200_0000;

fn leaf(e: &mut Engine, x: u32) -> u32 {
    e.mem.set_u32(COUNTER, e.mem.u32(COUNTER) + 1);
    x + 1
}

fn caller(e: &mut Engine, x: u32) -> u32 {
    typed_call!(FAMILY, 0x00ff_1000, leaf(e, x))
}

static ABOVE: Family = Family {
    name: "above_the_core",
    id: FAMILY,
    past_gate: true,
    threads: Threads::Both,
    funcs: || vec![entry!(0x00ff_1000, leaf(u32) -> u32)],
};

/// A family past its gate is on after `Engine::new` and goes direct; a double makes it go by
/// address, and the double is what runs.
#[test]
fn a_family_from_above_goes_direct_until_doubled() {
    if std::env::var_os("NV_TYPED").is_some() {
        return;
    }
    provide(&ABOVE);
    let mut e = Engine::new();
    e.map(COUNTER, 4);
    assert!(e.is_registered(LEAF));
    assert!(e.typed_direct(FAMILY, LEAF));
    assert_eq!(caller(&mut e, 1), 2);
    assert_eq!(e.mem.u32(COUNTER), 1);
    assert_eq!(e.call(LEAF, &[1]).u32(), 2);
    assert_eq!(e.mem.u32(COUNTER), 2);

    e.register(LEAF, |_, a| Ret {
        eax: a[0] + 100,
        ..Ret::default()
    });
    assert!(!e.typed_direct(FAMILY, LEAF));
    assert_eq!(caller(&mut e, 1), 101);
    assert_eq!(e.mem.u32(COUNTER), 2);
}

#[test]
fn the_differential_test_works_from_another_crate() {
    fn half(_: &mut Engine, x: f32) -> f32 {
        x * 0.5
    }
    fn half_again(_: &mut Engine, x: f32) -> f32 {
        x / 2.0
    }
    fn half_wrong(_: &mut Engine, x: f32) -> f32 {
        x * 0.4
    }
    let translation = entry!(0x00ff_2000, half(f32) -> f32).1;
    let same = Diff::new(entry!(0x00ff_2000, half_again(f32) -> f32))
        .translation(translation)
        .inputs(&[Input::F32])
        .run();
    same.assert_same();
    let wrong = Diff::new(entry!(0x00ff_2000, half_wrong(f32) -> f32))
        .translation(translation)
        .inputs(&[Input::F32])
        .run();
    assert!(!wrong.is_same());
}

/// A crate above the core names as many families as its call tree reaches.
#[test]
fn two_families_go_direct_from_another_crate() {
    const CALLER_ID: FamilyId = FamilyId::new(8);
    const LEAF_ID: FamilyId = FamilyId::new(9);
    fn leaf(_: &mut Engine, x: u32) -> u32 {
        x + 1
    }
    fn caller(e: &mut Engine, x: u32) -> u32 {
        let got: u32 = typed_call!(LEAF_ID, 0x00ff_3000, leaf(e, x));
        got * 2
    }
    fn caller_translated(_: &mut Engine, x: u32) -> u32 {
        (x + 1) * 2
    }
    static LEAVES: Family = Family {
        name: "seam_leaves",
        id: LEAF_ID,
        past_gate: false,
        threads: Threads::Both,
        funcs: || vec![entry!(0x00ff_3000, leaf(u32) -> u32)],
    };
    static CALLERS: Family = Family {
        name: "seam_callers",
        id: CALLER_ID,
        past_gate: false,
        threads: Threads::Both,
        funcs: || vec![entry!(0x00ff_3010, caller(u32) -> u32)],
    };
    let report = Diff::new(entry!(0x00ff_3010, caller(u32) -> u32))
        .translation(entry!(0x00ff_3010, caller_translated(u32) -> u32).1)
        .family(&CALLERS)
        .family(&LEAVES)
        .translation_calls_plainly(&[0x00ff_3000])
        .inputs(&[Input::Word])
        .setup(|e, case| {
            let (addr, f) = entry!(0x00ff_3000, leaf(u32) -> u32);
            e.register(addr, f);
            case.words().iter().map(|w| w & 0xffff).collect()
        })
        .run();
    report.assert_same();
    assert_eq!(report.direct_calls, report.runs);
}
