//! The differential test of every typed function of the lock-free map family
//! (`engine_typed::lockfree`) against its translation: here because the translations live in the part
//! crates, and this crate registers them all. Each test builds a real map in guest memory (buckets of
//! sorted nodes, an interface manager, an interface with its node slots, counted values), and runs
//! translation and typed version on it.
//!
//! What stands in: the map's virtual functions (its vtable points at test functions past `.text`),
//! the TLS slot (a guest word), and the map's lock (its entry and exit do nothing here; they are
//! still called, and the calls compare).

use engine as _;
use engine_core::prelude::*;
use engine_core::typed::diff::{Diff, Input};
use engine_typed::lockfree::{self, holder, iface, manager, map, queue};

/// Where the test's map functions are registered: past `.text`, one per vtable slot.
const VT_FN: u32 = 0x00ff_1000;
/// The words the stand-ins read, at fixed guest addresses: the interface `AllocateInterface`
/// answers, the TLS slot's value.
const WORLD: u32 = 0x0300_0000;
const W_FACE: u32 = WORLD;
const W_TLS: u32 = WORLD + 4;

/// The map lock's entry and exit, the import slots of `TlsGetValue` and `TlsSetValue`.
const LOCK_ENTER: u32 = 0x0040_fbf0;
const LOCK_LEAVE: u32 = 0x0040_fba0;
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
const TLS_ALLOC: u32 = 0x00fd_f048;
const TLS_FREE: u32 = 0x00fd_f050;
const COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
const INCREMENT: u32 = 0x00fd_f1c0;
const DECREMENT: u32 = 0x00fd_f1c4;
const CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;

/// The helpers the translations call by address that the typed versions do in place (a typed
/// callee too, so these go at every depth): the marked-pointer and iterator-flag helpers, the empty
/// function, the word getters, the slot release, the list links, the interface getter, the base
/// constructors and destructor, the TLS and bucket getters.
const INLINED: &[u32] = &[
    0x0040_fbe0,
    0x0044_b4b0,
    0x0044_d5b0,
    0x0044_d7f0,
    0x0044_ddc0,
    0x0044_de20,
    0x0044_de40,
    0x0044_de80,
    0x0044_dec0,
    0x0044_df00,
    0x0044_df20,
    0x0044_dff0,
    0x0044_e010,
    0x0044_e340,
    0x0044_edb0,
    0x0044_ee20,
    0x0044_f3e0,
    0x0052_9000,
    0x0053_7e90,
    0x0055_9450,
    0x0066_65a0,
    0x006e_bde0,
    0x006e_cba0,
    0x006e_cd40,
    0x0072_6070,
    0x0092_c870,
    0x0096_a2d0,
];

fn ret(eax: u32) -> Ret {
    Ret {
        eax,
        ..Ret::default()
    }
}

/// A map in guest memory, as the game builds one.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct World {
    map: u32,
    manager: u32,
    face: u32,
    buckets: u32,
    refs: u32,
    /// A counted object no node holds.
    spare: u32,
}

const NUM_BUCKETS: u32 = 4;

/// The most calls one run makes here is a few hundred: a version that loops stops at this many.
const MAX_CALLS: usize = 5_000;

/// A counted object with a do-nothing deleting destructor.
fn ref_object(e: &mut Engine, count: u32) -> u32 {
    let vt = e.mem.alloc(8);
    e.mem.set_u32(vt, VT_FN + 0x80);
    let p = e.mem.alloc(0xc);
    e.mem.set_u32(p, vt);
    e.mem.set_u32(p + 8, count);
    p
}

/// A node with `key` and a counted value, linked to `next`.
fn node(e: &mut Engine, key: u32, next: u32) -> u32 {
    let value = ref_object(e, 3);
    let n = e.mem.alloc(0xc);
    e.mem.set_u32(n, key);
    e.mem.set_u32(n + 4, value);
    e.mem.set_u32(n + 8, next);
    n
}

/// The map's virtual functions, as the test's map answers them.
fn register_map_functions(e: &mut Engine) {
    // AllocateInterface(index): the world's interface.
    e.register(VT_FN + 0x04, |e, _| ret(e.mem.u32(W_FACE)));
    // The bucket of a key (two words, the key twice).
    e.register(VT_FN + 0x24, |e, a| {
        let buckets = e.mem.u32(a[0] + 8);
        ret(a[1] % buckets.max(1))
    });
    // A key for a new node (key, value): the key.
    e.register(VT_FN + 0x2c, |_, a| ret(a[1]));
    // Copies a key to `*out`. Key 7 is a node another thread deletes meanwhile: the copy unlinks it.
    e.register(VT_FN + 0x30, |e, a| {
        if a[1] == 7 {
            let buckets = e.mem.u32(a[0] + 0xc);
            let head = e.mem.u32(buckets + 3 * 4) & !1;
            let next = e.mem.u32(head + 8) & !1;
            e.mem.set_u32(buckets + 3 * 4, next);
        }
        e.mem.set_u32(a[2], a[1]);
        ret(0)
    });
    // The search stops at a node whose key is not below the one looked for.
    e.register(VT_FN + 0x34, |_, a| ret(u32::from(a[1] >= a[2])));
    // Equal keys.
    e.register(VT_FN + 0x38, |_, a| ret(u32::from(a[1] == a[2])));
    // IncrementCount, DecrementCount.
    e.register(VT_FN + 0x3c, |e, a| {
        let n = e.mem.u32(a[0] + 0x18);
        e.mem.set_u32(a[0] + 0x18, n.wrapping_add(1));
        ret(0)
    });
    e.register(VT_FN + 0x40, |e, a| {
        let n = e.mem.u32(a[0] + 0x18);
        e.mem.set_u32(a[0] + 0x18, n.wrapping_sub(1));
        ret(0)
    });
    // A counted object's deleting destructor.
    e.register(VT_FN + 0x80, |_, a| ret(a[0]));
    // A key's release when its node is deleted.
    e.register(VT_FN + 0x28, |_, _| ret(0));
    // The interlocked imports and TLS allocation, as Windows answers them on one thread.
    e.register(COMPARE_EXCHANGE, |e, a| {
        let old = e.mem.u32(a[0]);
        if old == a[2] {
            e.mem.set_u32(a[0], a[1]);
        }
        ret(old)
    });
    e.register(INCREMENT, |e, a| {
        let n = e.mem.u32(a[0]).wrapping_add(1);
        e.mem.set_u32(a[0], n);
        ret(n)
    });
    e.register(DECREMENT, |e, a| {
        let n = e.mem.u32(a[0]).wrapping_sub(1);
        e.mem.set_u32(a[0], n);
        ret(n)
    });
    e.register(TLS_ALLOC, |_, _| ret(7));
    e.register(TLS_FREE, |_, _| ret(1));
    e.register(CURRENT_THREAD_ID, |_, _| ret(0x1234));
    e.register(LOCK_ENTER, |_, _| ret(0));
    e.register(LOCK_LEAVE, |_, _| ret(0));
    e.register(TLS_GET_VALUE, |e, _| ret(e.mem.u32(W_TLS)));
    e.register(TLS_SET_VALUE, |e, a| {
        e.mem.set_u32(W_TLS, a[1]);
        ret(1)
    });
}

/// The world: buckets 0 empty, 1 = [1, 5], 2 = [2], 3 = [7 (deleted, unlinked by the copy), 11].
/// The interface is the thread's (the TLS slot holds it) when `tls_set`.
fn world(e: &mut Engine, tls_set: bool) -> World {
    register_map_functions(e);
    e.map(WORLD, 0x1000);
    let vt = e.mem.alloc(0x48);
    for slot in (0..0x48).step_by(4) {
        e.mem.set_u32(vt + slot, VT_FN + slot);
    }
    let map = e.mem.alloc(0x40);
    e.mem.set_u32(map, vt);
    let refs = e.mem.alloc(3 * 2 * 4);
    e.mem.set_u32(map + 4, refs);
    e.mem.set_u32(map + 8, NUM_BUCKETS);
    let buckets = e.mem.alloc(NUM_BUCKETS * 4);
    e.mem.set_u32(map + 0xc, buckets);
    e.mem.set_u32(map + 0x10, 100);
    let manager = e.mem.alloc(0x10);
    e.mem.set_u32(map + 0x14, manager);
    e.mem.set_u32(map + 0x18, 5);
    // Buckets.
    let five = node(e, 5, 0);
    let one = node(e, 1, five);
    e.mem.set_u32(buckets + 4, one);
    let two = node(e, 2, 0);
    e.mem.set_u32(buckets + 8, two);
    let eleven = node(e, 11, 0);
    let seven = node(e, 7, eleven | 1);
    e.mem.set_u32(buckets + 12, seven);
    // The interface manager: two threads, TLS slot 5.
    e.mem.set_u32(manager, 2);
    e.mem.set_u32(manager + 4, 5);
    let entries = e.mem.alloc(16);
    e.mem.set_u32(manager + 8, entries);
    e.mem.set_u32(manager + 0xc, 1);
    // The interface of thread 0.
    let face = e.mem.alloc(0x24);
    e.mem.set_u32(face, map);
    e.mem.set_u32(face + 4, refs);
    e.mem.set_u32(face + 8, refs + 4);
    e.mem.set_u32(face + 0xc, refs + 8);
    e.mem.set_u32(W_FACE, face);
    e.mem.set_u32(W_TLS, if tls_set { face } else { 0 });
    let spare = ref_object(e, 2);
    World {
        map,
        manager,
        face,
        buckets,
        refs,
        spare,
    }
}

/// A holder (a one-word slot) holding `object`.
fn holder_of(e: &mut Engine, object: u32) -> u32 {
    let h = e.mem.alloc(4);
    e.mem.set_u32(h, object);
    h
}

/// The comparisons of the functions with copies at other addresses, shared by the original and its
/// copies.
trait Cases {
    fn destructor_case(self) -> Diff;
    fn assign_case(self) -> Diff;
    fn iterate_words_case(self) -> Diff;
    fn map_find_case(self) -> Diff;
    fn remove_from_bucket_case(self) -> Diff;
    fn scan_deleted_case(self) -> Diff;
    fn search_case(self) -> Diff;
}

impl Cases for Diff {
    // With an interface manager or none.
    fn destructor_case(self) -> Diff {
        self.family(&lockfree::LOCKFREE)
            .family(&leaf::LEAVES)
            .ignore_calls(INLINED)
            .max_calls(MAX_CALLS)
            .inputs(&[Input::Flag])
            .setup(|e, case| {
                let w = world(e, true);
                if case.word(0) == 0 {
                    e.mem.set_u32(w.map + 0x14, 0);
                }
                // The destructor puts the real map's vtable in place, whose functions these engines
                // don't map: so no node to delete through it, the buckets empty.
                for k in 0..NUM_BUCKETS {
                    e.mem.set_u32(w.buckets + k * 4, 0);
                }
                vec![w.map]
            })
    }

    // 0: null, 1: one object, 2: another.
    fn assign_case(self) -> Diff {
        self.family(&lockfree::LOCKFREE)
            .family(&leaf::LEAVES)
            .ignore_calls(INLINED)
            .max_calls(MAX_CALLS)
            .inputs(&[Input::Of(&[0, 1, 2]), Input::Of(&[0, 1, 2])])
            .setup(|e, case| {
                let w = world(e, true);
                let other = ref_object(e, 7);
                let pick = |k: u32| match k {
                    0 => 0,
                    1 => w.spare,
                    _ => other,
                };
                let this = holder_of(e, pick(case.word(0)));
                let from = holder_of(e, pick(case.word(1)));
                vec![this, from]
            })
    }

    fn iterate_words_case(self) -> Diff {
        self.family(&lockfree::LOCKFREE)
            .family(&leaf::LEAVES)
            .ignore_calls(INLINED)
            .max_calls(MAX_CALLS)
            .inputs(&[
                Input::Of(&[0, 1, 2, 3]),
                Input::Of(&[0, 1, 2]),
                Input::Of(&[1, 2, 5]),
                Input::Flag,
            ])
            .setup(|e, case| {
                let w = world(e, true);
                let it = e.mem.alloc(0x10);
                e.mem.set_u32(it + 4, case.word(0));
                e.mem.set_u32(it + 8, case.word(2));
                e.mem.set_u8(it + 0xc, case.word(1) as u8);
                let key_out = e.mem.alloc(4);
                let value_out = e.mem.alloc(4);
                vec![w.map, it, key_out, value_out, case.word(3)]
            })
    }

    fn map_find_case(self) -> Diff {
        self.family(&lockfree::LOCKFREE)
            .family(&leaf::LEAVES)
            .ignore_calls(INLINED)
            .max_calls(MAX_CALLS)
            .inputs(&[Input::Of(KEYS), Input::Flag])
            .setup(|e, case| {
                let w = world(e, case.word(1) != 0);
                let value_out = holder_of(e, 0);
                vec![w.map, case.word(0), value_out]
            })
    }

    fn remove_from_bucket_case(self) -> Diff {
        self.family(&lockfree::LOCKFREE)
            .family(&leaf::LEAVES)
            .ignore_calls(INLINED)
            .max_calls(MAX_CALLS)
            .inputs(&[Input::Of(&[0, 1, 2, 3]), Input::Of(KEYS), Input::Flag])
            .setup(|e, case| {
                let w = world(e, true);
                if case.word(2) != 0 {
                    e.mem.set_u32(w.face + 0x20, 99);
                }
                vec![w.face, case.word(0), case.word(1)]
            })
    }

    // A waiting list of three nodes; none, the middle one, or all named by a referenced-node slot.
    fn scan_deleted_case(self) -> Diff {
        self.family(&lockfree::LOCKFREE)
            .family(&leaf::LEAVES)
            .ignore_calls(INLINED)
            .max_calls(MAX_CALLS)
            .inputs(&[Input::Of(&[0, 1, 2])])
            .setup(|e, case| {
                let w = world(e, true);
                let third = node(e, 40, 0);
                let second = node(e, 41, 0);
                let first = node(e, 42, 0);
                // Linked through their value words, as the retirement links them.
                e.mem.set_u32(first + 4, second);
                e.mem.set_u32(second + 4, third);
                e.mem.set_u32(third + 4, 0);
                e.mem.set_u32(w.face + 0x1c, first);
                e.mem.set_u32(w.face + 0x20, 3);
                match case.word(0) {
                    1 => e.mem.set_u32(w.refs + 4, second),
                    2 => {
                        e.mem.set_u32(w.refs, first);
                        e.mem.set_u32(w.refs + 12, second);
                        e.mem.set_u32(w.refs + 20, third);
                    }
                    _ => {}
                }
                vec![w.face]
            })
    }

    // Every bucket and key; with the interface's waiting list one short of a full batch, or not, so
    // the retirement of the deleted node in bucket 3 also scans.
    fn search_case(self) -> Diff {
        self.family(&lockfree::LOCKFREE)
            .family(&leaf::LEAVES)
            .ignore_calls(INLINED)
            .max_calls(MAX_CALLS)
            .inputs(&[Input::Of(&[0, 1, 2, 3]), Input::Of(KEYS), Input::Flag])
            .setup(|e, case| {
                let w = world(e, true);
                if case.word(2) != 0 {
                    e.mem.set_u32(w.face + 0x20, 99);
                }
                vec![w.face, case.word(0), case.word(1)]
            })
    }
}

#[test]
fn holder_assign() {
    Diff::new(entry!(0x0092_c820, holder::assign(Ptr<_>, Ptr<_>) -> Ptr))
        .assign_case()
        .run()
        .assert_same();
}

#[test]
fn interface_for_thread() {
    // The TLS slot holds the interface, or not (then one is made: within the limit or past it).
    Diff::new(entry!(
        0x0044_d5c0,
        manager::interface_for_thread(Ptr<_>, Ptr<_>) -> Ptr
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[Input::Flag, Input::Of(&[0, 1, 2, 0xffff_ffff])])
    .setup(|e, case| {
        let w = world(e, case.word(0) != 0);
        e.mem.set_u32(w.manager + 0xc, case.word(1));
        vec![w.manager, w.map]
    })
    .run()
    .assert_same();
}

#[test]
fn make_interface() {
    Diff::new(entry!(
        0x0044_e3a0,
        manager::make_interface(Ptr<_>, Ptr<_>) -> Ptr
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[Input::Of(&[0, 1, 2, 0xffff_ffff]), Input::Of(&[0, 1, 2])])
    .setup(|e, case| {
        let w = world(e, false);
        e.mem.set_u32(w.manager + 0xc, case.word(0));
        e.mem.set_u32(w.manager, case.word(1));
        vec![w.manager, w.map]
    })
    .run()
    .assert_same();
}

#[test]
fn first_in_bucket() {
    // Bucket 0 empty, 1 and 2 with nodes, 3 with a node deleted meanwhile; the value holder empty,
    // holding the node's value, or another object.
    Diff::new(entry!(
        0x0044_e550,
        iface::first_in_bucket(Ptr<_>, u32, Ptr<_>, Ptr<_>) -> bool
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[Input::Of(&[0, 1, 2, 3]), Input::Of(&[0, 1, 2])])
    .setup(|e, case| {
        let w = world(e, true);
        let key_out = e.mem.alloc(4);
        let bucket = case.word(0);
        let head = e.mem.u32(w.buckets + bucket * 4);
        let held = match case.word(1) {
            0 => 0,
            1 if head != 0 => e.mem.u32(head + 4),
            _ => w.spare,
        };
        let value_out = holder_of(e, held);
        vec![w.face, bucket, key_out, value_out]
    })
    .run()
    .assert_same();
}

#[test]
fn iterate() {
    // The iterator at each bucket, with no last key, a last key, or at its end; the last key one of
    // the map's or none; keep going or not.
    Diff::new(entry!(
        0x0044_b8d0,
        map::iterate(Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>, u8) -> bool
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[
        Input::Of(&[0, 1, 2, 3]),
        Input::Of(&[0, 1, 2]),
        Input::Of(&[1, 2, 5]),
        Input::Flag,
    ])
    .setup(|e, case| {
        let w = world(e, true);
        let it = e.mem.alloc(0x10);
        e.mem.set_u32(it + 4, case.word(0));
        e.mem.set_u32(it + 8, case.word(2));
        e.mem.set_u8(it + 0xc, case.word(1) as u8);
        let key_out = e.mem.alloc(4);
        let value_out = holder_of(e, 0);
        vec![w.map, it, key_out, value_out, case.word(3)]
    })
    .run()
    .assert_same();
}

/// The keys looked for: below, at and past each node of each bucket.
const KEYS: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 11, 12];

#[test]
fn search() {
    Diff::new(entry!(0x0052_9030, iface::search(Ptr<_>, u32, u32) -> u32))
        .search_case()
        .run()
        .assert_same();
}

#[test]
fn search_string_map() {
    Diff::new(entry!(
        0x0066_62a0,
        iface::search_string_map(Ptr<_>, u32, u32) -> u32
    ))
    .search_case()
    .run()
    .assert_same();
}

#[test]
fn retire() {
    // The waiting list empty or not; the count well below, one short of, or past the batch.
    Diff::new(entry!(0x0052_92a0, iface::retire(Ptr<_>, Ptr<_>)))
        .family(&lockfree::LOCKFREE)
        .family(&leaf::LEAVES)
        .ignore_calls(INLINED)
        .max_calls(MAX_CALLS)
        .inputs(&[Input::Flag, Input::Of(&[0, 99, 100])])
        .setup(|e, case| {
            let w = world(e, true);
            // The deleted node of bucket 3, unlinked.
            let seven = e.mem.u32(w.buckets + 12);
            let eleven = e.mem.u32(seven + 8) & !1;
            e.mem.set_u32(w.buckets + 12, eleven);
            if case.word(0) != 0 {
                let earlier = node(e, 30, 1);
                e.mem.set_u32(earlier + 4, 0);
                e.mem.set_u32(w.face + 0x1c, earlier);
            }
            e.mem.set_u32(w.face + 0x20, case.word(1));
            vec![w.face, seven]
        })
        .run()
        .assert_same();
}

#[test]
fn next_in_bucket() {
    // Every bucket and key; the value holder empty or holding another object.
    Diff::new(entry!(
        0x0044_e680,
        iface::next_in_bucket(Ptr<_>, u32, Ptr<_>, Ptr<_>) -> bool
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[Input::Of(&[0, 1, 2, 3]), Input::Of(KEYS), Input::Flag])
    .setup(|e, case| {
        let w = world(e, true);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, case.word(1));
        let held = if case.word(2) != 0 { w.spare } else { 0 };
        let value_out = holder_of(e, held);
        vec![w.face, case.word(0), key, value_out]
    })
    .run()
    .assert_same();
}

#[test]
fn remove_from_bucket() {
    Diff::new(entry!(
        0x0052_8ce0,
        iface::remove_from_bucket(Ptr<_>, u32, u32) -> bool
    ))
    .remove_from_bucket_case()
    .run()
    .assert_same();
}

#[test]
fn remove_from_bucket_string_map() {
    Diff::new(entry!(
        0x0066_6080,
        iface::remove_from_bucket_string_map(Ptr<_>, u32, u32) -> bool
    ))
    .remove_from_bucket_case()
    .run()
    .assert_same();
}

#[test]
fn remove() {
    Diff::new(entry!(0x0052_8bb0, iface::remove(Ptr<_>, u32) -> bool))
        .family(&lockfree::LOCKFREE)
        .family(&leaf::LEAVES)
        .ignore_calls(INLINED)
        .max_calls(MAX_CALLS)
        .inputs(&[Input::Of(KEYS), Input::Flag])
        .setup(|e, case| {
            let w = world(e, true);
            if case.word(1) != 0 {
                e.mem.set_u32(w.face + 0x20, 99);
            }
            vec![w.face, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn first_word_in_bucket() {
    Diff::new(entry!(
        0x0044_ea60,
        iface::first_word_in_bucket(Ptr<_>, u32, Ptr<_>, Ptr<_>) -> bool
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[Input::Of(&[0, 1, 2, 3])])
    .setup(|e, case| {
        let w = world(e, true);
        let key_out = e.mem.alloc(4);
        let value_out = e.mem.alloc(4);
        e.mem.set_u32(value_out, 0x5a5a_5a5a);
        vec![w.face, case.word(0), key_out, value_out]
    })
    .run()
    .assert_same();
}

#[test]
fn next_word_in_bucket() {
    Diff::new(entry!(
        0x0044_eb80,
        iface::next_word_in_bucket(Ptr<_>, u32, Ptr<_>, Ptr<_>) -> bool
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[Input::Of(&[0, 1, 2, 3]), Input::Of(KEYS)])
    .setup(|e, case| {
        let w = world(e, true);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, case.word(1));
        let value_out = e.mem.alloc(4);
        e.mem.set_u32(value_out, 0x5a5a_5a5a);
        vec![w.face, case.word(0), key, value_out]
    })
    .run()
    .assert_same();
}

#[test]
fn find_value() {
    // Each bucket, against the value of each of its nodes, a value nobody holds, and 0.
    Diff::new(entry!(
        0x0044_e820,
        iface::find_value(Ptr<_>, u32, Ptr<_>) -> bool
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[
        Input::Of(&[0, 1, 2, 3]),
        Input::Of(&[0, 1, 2, 3]),
        Input::Flag,
    ])
    .setup(|e, case| {
        let w = world(e, true);
        if case.word(2) != 0 {
            e.mem.set_u32(w.face + 0x20, 99);
        }
        // The value of the bucket's nth node (following the links, marks cleared), else
        // the spare object, else 0.
        let mut n = e.mem.u32(w.buckets + case.word(0) * 4) & !1;
        for _ in 0..case.word(1) {
            if n != 0 {
                n = e.mem.u32(n + 8) & !1;
            }
        }
        let wanted = match (n, case.word(1)) {
            (0, 3) => 0,
            (0, _) => w.spare,
            (n, _) => e.mem.u32(n + 4),
        };
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, wanted);
        vec![w.face, case.word(0), value]
    })
    .run()
    .assert_same();
}

#[test]
fn find() {
    Diff::new(entry!(0x0044_c8e0, iface::find(Ptr<_>, u32, Ptr<_>) -> u8))
        .family(&lockfree::LOCKFREE)
        .family(&leaf::LEAVES)
        .ignore_calls(INLINED)
        .max_calls(MAX_CALLS)
        .inputs(&[Input::Of(KEYS), Input::Flag])
        .setup(|e, case| {
            let w = world(e, true);
            let held = if case.word(1) != 0 { w.spare } else { 0 };
            let value_out = holder_of(e, held);
            vec![w.face, case.word(0), value_out]
        })
        .run()
        .assert_same();
}

#[test]
fn insert() {
    // Every key (new, present, deleted), a plain or counted value, replacing or not.
    Diff::new(entry!(
        0x0044_c000,
        iface::insert(Ptr<_>, u32, u32, u8) -> u32
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .local_at(0x0040_4eb0, 0, 4)
    .local_at(0x0040_4ee0, 0, 4)
    .inputs(&[Input::Of(KEYS), Input::Flag, Input::Flag])
    .setup(|e, case| {
        let w = world(e, true);
        let value = if case.word(1) != 0 { w.spare } else { 0 };
        let holder = holder_of(e, value);
        vec![w.face, case.word(0), holder, case.word(2)]
    })
    .run()
    .assert_same();
}

#[test]
fn map_find() {
    Diff::new(entry!(0x0044_bd60, map::find(Ptr<_>, u32, Ptr<_>) -> u8))
        .map_find_case()
        .run()
        .assert_same();
}

#[test]
fn map_find_word() {
    Diff::new(entry!(
        0x0044_9530,
        map::find_word(Ptr<_>, u32, Ptr<_>) -> u8
    ))
    .map_find_case()
    .run()
    .assert_same();
}

#[test]
fn insert_in_bucket() {
    // Each key (its bucket from the map's rule), a counted value or an empty holder, replacing or
    // not. The scope's local is the typed version's only one: compared as a local.
    Diff::new(entry!(
        0x0044_d070,
        iface::insert_in_bucket(Ptr<_>, u32, u32, Ptr<_>, u8) -> bool
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .local_at(0x0040_4eb0, 0, 4)
    .local_at(0x0040_4ee0, 0, 4)
    .inputs(&[Input::Of(KEYS), Input::Flag, Input::Flag])
    .setup(|e, case| {
        let w = world(e, true);
        let value = if case.word(1) != 0 { w.spare } else { 0 };
        let holder = holder_of(e, value);
        vec![
            w.face,
            case.word(0) % NUM_BUCKETS,
            case.word(0),
            holder,
            case.word(2),
        ]
    })
    .run()
    .assert_same();
}

#[test]
fn insert_word_in_bucket() {
    Diff::new(entry!(
        0x0044_d240,
        iface::insert_word_in_bucket(Ptr<_>, u32, u32, Ptr<_>, u8) -> bool
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .local_at(0x0040_4eb0, 0, 4)
    .local_at(0x0040_4ee0, 0, 4)
    .inputs(&[Input::Of(KEYS), Input::Flag])
    .setup(|e, case| {
        let w = world(e, true);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0x1234_5678);
        vec![
            w.face,
            case.word(0) % NUM_BUCKETS,
            case.word(0),
            word,
            case.word(1),
        ]
    })
    .run()
    .assert_same();
}

#[test]
fn iterate_words() {
    Diff::new(entry!(
        0x0044_c640,
        map::iterate_words(Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>, u8) -> bool
    ))
    .iterate_words_case()
    .run()
    .assert_same();
}

#[test]
fn iterate_lip_task_map() {
    Diff::new(entry!(
        0x0090_6570,
        map::iterate_lip_task_map(Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>, u8) -> bool
    ))
    .iterate_words_case()
    .run()
    .assert_same();
}

#[test]
fn clear() {
    // With an interface manager or none; keeping none or making a new one.
    Diff::new(entry!(0x0044_c480, map::clear(Ptr<_>, u8)))
        .family(&lockfree::LOCKFREE)
        .family(&leaf::LEAVES)
        .ignore_calls(INLINED)
        .max_calls(MAX_CALLS)
        .inputs(&[Input::Flag, Input::Flag])
        .setup(|e, case| {
            let w = world(e, true);
            if case.word(0) == 0 {
                e.mem.set_u32(w.map + 0x14, 0);
            }
            vec![w.map, case.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn construct_interface_manager() {
    Diff::new(entry!(0x006c_73b0, manager::construct(Ptr<_>, u32) -> Ptr))
        .family(&lockfree::LOCKFREE)
        .family(&leaf::LEAVES)
        .ignore_calls(INLINED)
        .max_calls(MAX_CALLS)
        .inputs(&[Input::Of(&[0, 1, 2, 5])])
        .setup(|e, case| {
            register_map_functions(e);
            let manager = e.mem.alloc(0x10);
            for k in 0..4 {
                e.mem.set_u32(manager + k * 4, 0xdddd_dddd);
            }
            vec![manager, case.word(0)]
        })
        .run()
        .assert_same();
}

/// A map constructor's comparison: max threads, buckets and batch size at their edges, over a map
/// filled with a pattern.
trait ConstructorCase {
    fn constructor_case(self) -> Diff;
}

impl ConstructorCase for Diff {
    fn constructor_case(self) -> Diff {
        self.family(&lockfree::LOCKFREE)
            .family(&leaf::LEAVES)
            .ignore_calls(INLINED)
            // The scope object: one translation keeps it in a bigger frame.
            .local_at(0x0040_4eb0, 0, 4)
            .local_at(0x0040_4ee0, 0, 4)
            .max_calls(MAX_CALLS)
            .inputs(&[
                Input::Of(&[0, 1, 2]),
                Input::Of(&[0, 1, 4]),
                Input::Of(&[0, 100]),
            ])
            .setup(|e, case| {
                register_map_functions(e);
                let map = e.mem.alloc(0x40);
                for k in 0..0x10 {
                    e.mem.set_u32(map + k * 4, 0xdddd_dddd);
                }
                vec![map, case.word(0), case.word(1), case.word(2)]
            })
    }
}

#[test]
fn construct_map() {
    Diff::new(entry!(
        0x0044_b750,
        map::construct(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_anim_idle_map() {
    Diff::new(entry!(
        0x0044_ba10,
        map::construct_anim_idle_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_replacement_kf_list_map() {
    Diff::new(entry!(
        0x0044_bbe0,
        map::construct_replacement_kf_list_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_helmet_map() {
    Diff::new(entry!(
        0x0044_be80,
        map::construct_helmet_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_model_map() {
    Diff::new(entry!(
        0x0044_c040,
        map::construct_model_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_kf_model_map() {
    Diff::new(entry!(
        0x0044_c270,
        map::construct_kf_model_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_texture_map() {
    Diff::new(entry!(
        0x0044_c760,
        map::construct_texture_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_loaded_file_map() {
    Diff::new(entry!(
        0x0044_c920,
        map::construct_loaded_file_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_cell_task_map() {
    Diff::new(entry!(
        0x0052_89e0,
        map::construct_cell_task_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_tree_node_map() {
    Diff::new(entry!(
        0x0066_5cb0,
        map::construct_tree_node_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_pathing_task_map() {
    Diff::new(entry!(
        0x006e_c660,
        map::construct_pathing_task_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn construct_lip_task_map() {
    Diff::new(entry!(
        0x0090_6160,
        map::construct_lip_task_map(Ptr<_>, u32, u32, u32) -> Ptr
    ))
    .constructor_case()
    .run()
    .assert_same();
}

#[test]
fn queue_allocate_interface() {
    Diff::new(entry!(
        0x0044_b660,
        queue::allocate_interface(Ptr<_>, u32) -> u32
    ))
    .family(&lockfree::LOCKFREE)
    .family(&leaf::LEAVES)
    .ignore_calls(INLINED)
    .max_calls(MAX_CALLS)
    .inputs(&[Input::Of(&[0, 1, 2, 0x8000_0001])])
    .setup(|e, case| {
        let queue = e.mem.alloc(0x1c);
        let slots = e.mem.alloc(2 * 3 * 4);
        e.mem.set_u32(queue + 0x10, slots);
        vec![queue, case.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn scan_deleted() {
    Diff::new(entry!(0x0044_f220, iface::scan_deleted(Ptr<_>)))
        .scan_deleted_case()
        .run()
        .assert_same();
}

#[test]
fn scan_deleted_releasing() {
    Diff::new(entry!(0x0044_f070, iface::scan_deleted_releasing(Ptr<_>)))
        .scan_deleted_case()
        .run()
        .assert_same();
}

#[test]
fn destroy() {
    Diff::new(entry!(0x0044_c3f0, map::destroy(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_model_map() {
    Diff::new(entry!(0x0044_c1e0, map::destroy_model_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_loaded_file_map() {
    Diff::new(entry!(0x0044_cae0, map::destroy_loaded_file_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_tree_node_map() {
    Diff::new(entry!(0x0066_61e0, map::destroy_tree_node_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_pathing_task_map() {
    Diff::new(entry!(0x006e_c870, map::destroy_pathing_task_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_queued_reference_map() {
    Diff::new(entry!(
        0x0044_d8f0,
        map::destroy_queued_reference_map(Ptr<_>)
    ))
    .destructor_case()
    .run()
    .assert_same();
}

#[test]
fn destroy_anim_idle_map() {
    Diff::new(entry!(0x0044_d9b0, map::destroy_anim_idle_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_replacement_kf_list_map() {
    Diff::new(entry!(
        0x0044_da70,
        map::destroy_replacement_kf_list_map(Ptr<_>)
    ))
    .destructor_case()
    .run()
    .assert_same();
}

#[test]
fn destroy_helmet_map() {
    Diff::new(entry!(0x0044_db30, map::destroy_helmet_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_texture_map() {
    Diff::new(entry!(0x0044_dc50, map::destroy_texture_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_cell_task_map() {
    Diff::new(entry!(0x0052_8be0, map::destroy_cell_task_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn destroy_lip_task_map() {
    Diff::new(entry!(0x0090_64e0, map::destroy_lip_task_map(Ptr<_>)))
        .destructor_case()
        .run()
        .assert_same();
}

#[test]
fn interface_of() {
    // The TLS slot holds the interface, or not (one is made).
    Diff::new(entry!(0x0044_9f80, map::interface_of(Ptr<_>) -> Ptr))
        .family(&lockfree::LOCKFREE)
        .family(&leaf::LEAVES)
        .ignore_calls(INLINED)
        .max_calls(MAX_CALLS)
        .inputs(&[Input::Flag])
        .setup(|e, case| {
            let w = world(e, case.word(0) != 0);
            vec![w.map]
        })
        .run()
        .assert_same();
}
