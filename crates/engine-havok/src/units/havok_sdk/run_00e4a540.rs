//! `Havok SDK/run_00e4a540` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::store_vec;

/// `00e3ed00` (tree path search): thiscall on the node, `(target, path)`, `RET 8`; returns 0 when
/// the path was found.
const FIND_PATH: u32 = 0x00e3_ed00;
/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`; its `+0x10` is the heap
/// allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00de8e50`: thiscall on the matrix, one word (a transform), `RET 4`.
const APPLY_TRANSFORM: u32 = 0x00de_8e50;

// Translated from 00e4a540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: a tail call. Runs `00e3ed00` on the node at `*(this + 0x14)` with `target`
/// and `path` and returns its result.
pub fn fn_00e4a540(e: &mut Engine, this: Ptr, target: u32, path: u32) -> u32 {
    let node = e.mem.u32(this.addr().wrapping_add(0x14));
    e.call(FIND_PATH, &args![node, target, path]).u32()
}

/// Empties the path array on the stack of `00e4a5a0` and frees its block when the array owns one
/// (the sign bit of the capacity word is clear), as the exe does at both of its exits.
fn release_path(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let flags = e.mem.u32(array.wrapping_add(8));
    e.mem.set_u32(array.wrapping_add(4), 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (flags & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
}

// Translated from 00e4a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: the combined transform along the path to `target`. A path array (an
/// `hkArray` on the stack with an inline block for 32 entries, unowned) is filled by `00e4a540`;
/// when that returns nonzero the array is released and 1 is returned. Otherwise the 4x4 matrix
/// at `out` is set to the identity (the zero vectors, then `1.0` in the diagonal words at `+0`,
/// `+0x14`, `+0x28` and `+0x3c`) and, for each node of the path, `00de8e50` runs on `out` with
/// the transform of index `limit` (or the last one, `count - 1`, when `limit` is not below the
/// node's count at `+0x20`) in the node's array of 64-byte transforms at `+0x1c`; the array is
/// released and 0 returned.
pub fn fn_00e4a5a0(e: &mut Engine, this: Ptr, target: u32, out: u32, limit: i32) -> u32 {
    e.with_stack(0xa0, |e, frame| {
        let array = frame.addr();
        e.mem.set_u32(array, array.wrapping_add(0xc));
        e.mem.set_u32(array.wrapping_add(4), 0);
        e.mem.set_u32(array.wrapping_add(8), 0x8000_0020);
        let not_found = fn_00e4a540(e, this, target, array);
        if not_found != 0 {
            release_path(e, array);
            return 1;
        }
        for row in 0..4u32 {
            store_vec(e, out.wrapping_add(16 * row), [0.0; 4]);
        }
        for diagonal in [0u32, 0x14, 0x28, 0x3c] {
            e.mem.set_u32(out.wrapping_add(diagonal), 0x3f80_0000);
        }
        let mut index = 0i32;
        if e.mem.i32(array.wrapping_add(4)) > 0 {
            loop {
                let nodes = e.mem.u32(array);
                let node = e
                    .mem
                    .u32(nodes.wrapping_add((index as u32).wrapping_mul(4)));
                let count = e.mem.i32(node.wrapping_add(0x20));
                let chosen = if limit >= count {
                    count.wrapping_sub(1)
                } else {
                    limit
                };
                let transform = (chosen as u32)
                    .wrapping_shl(6)
                    .wrapping_add(e.mem.u32(node.wrapping_add(0x1c)));
                e.call(APPLY_TRANSFORM, &args![out, transform]);
                index += 1;
                if index >= e.mem.i32(array.wrapping_add(4)) {
                    break;
                }
            }
        }
        release_path(e, array);
        0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_a540, fn_00e4a540(Ptr, u32, u32) -> u32),
        entry!(0x00e4_a5a0, fn_00e4a5a0(Ptr, u32, u32, i32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00d6bf60::load_vec;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e.mem.set_u32(0x0126_0010, 0x0126_0100);
        e.mem.set_u32(0x0126_0100, 0x0126_0200);
        e.put_vtable(0x0126_0200, &[0, 0, 0, 0, 0x0ff0_0010]);
        e
    }

    fn record(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let address = *address;
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    #[test]
    fn tail_call_uses_the_node_at_0x14() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(FIND_PATH, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 7,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5000);
        assert_eq!(e.call(0x00e4_a540, &args![this, 0x11u32, 0x22u32]).u32(), 7);
        assert_eq!(*seen.borrow(), vec![vec![0x5000, 0x11, 0x22]]);
    }

    /// A node with `count` transforms of 64 bytes, the first word of transform `k` being `k + 1`.
    fn node(e: &mut Engine, count: u32) -> u32 {
        let node = e.mem.alloc(0x30);
        let transforms = e.mem.alloc(0x40 * count);
        for index in 0..count {
            e.mem.set_u32(transforms + 0x40 * index, index + 1);
        }
        e.mem.set_u32(node + 0x1c, transforms);
        e.mem.set_u32(node + 0x20, count);
        node
    }

    /// The path search double: pushes `nodes` onto the path array and answers `answer`.
    fn path_double(e: &mut Engine, nodes: Vec<u32>, answer: u32) {
        e.register_double(FIND_PATH, move |e, a| {
            let array = a[2];
            let data = e.mem.u32(array);
            for (index, node) in nodes.iter().enumerate() {
                e.mem.set_u32(data + 4 * index as u32, *node);
            }
            e.mem.set_u32(array + 4, nodes.len() as u32);
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn found_path_applies_the_clamped_transforms_in_order() {
        let mut e = engine();
        let applied = record(&mut e, &[APPLY_TRANSFORM]);
        let first = node(&mut e, 3);
        let second = node(&mut e, 1);
        path_double(&mut e, vec![first, second], 0);
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(0x40);
        for word in 0..16u32 {
            e.mem.set_u32(out + 4 * word, 0xffff_ffff);
        }
        assert_eq!(
            e.call(0x00e4_a5a0, &args![this, 0x77u32, out, 1i32]).u32(),
            0
        );
        // Identity matrix.
        assert_eq!(load_vec(&e, out), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x30), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(e.mem.f32(out + 0x14), 1.0);
        assert_eq!(e.mem.f32(out + 0x28), 1.0);
        let calls = applied.borrow();
        assert_eq!(calls.len(), 2);
        let first_transform = e.mem.u32(first + 0x1c) + 0x40;
        let second_transform = e.mem.u32(second + 0x1c);
        assert_eq!(calls[0].1, vec![out, first_transform]);
        // The second node has one transform: index 1 is not below its count, so the last one.
        assert_eq!(calls[1].1, vec![out, second_transform]);
    }

    #[test]
    fn missing_path_returns_one_and_frees_a_grown_array() {
        let mut e = engine();
        let applied = record(&mut e, &[APPLY_TRANSFORM, 0x0ff0_0010]);
        // The search double replaces the path block by an owned one of 8 entries.
        e.register_double(FIND_PATH, |e, a| {
            let data = e.mem.alloc(64);
            e.mem.set_u32(a[2], data);
            e.mem.set_u32(a[2] + 8, 8);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x00e4_a5a0, &args![this, 0x77u32, out, 0i32]).u32(),
            1
        );
        let calls = applied.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, 0x0ff0_0010);
        assert_eq!(calls[0].1[0], 0x0126_0100);
        assert_eq!(calls[0].1[2], 32);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn empty_path_leaves_the_identity_and_the_inline_block_unfreed() {
        let mut e = engine();
        let applied = record(&mut e, &[APPLY_TRANSFORM, 0x0ff0_0010]);
        path_double(&mut e, vec![], 0);
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x00e4_a5a0, &args![this, 0x77u32, out, 0i32]).u32(),
            0
        );
        assert!(applied.borrow().is_empty());
        assert_eq!(e.mem.f32(out + 0x3c), 1.0);
    }
}
