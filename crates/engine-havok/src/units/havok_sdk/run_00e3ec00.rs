//! `Havok SDK/run_00e3ec00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`; its `+0x10` is the heap
/// allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`): cdecl, `(allocator, array, element size)`.
const RESERVE_MORE: u32 = 0x00c9_0940;

// Translated from 00e3ec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall (`this` in `ECX`): the number of nodes below `this`. `this + 0x2c` is the number of
/// children (signed; none when it is not above 0) and `*(this + 0x28)` the array of child
/// pointers; the result is the child count plus the same result for every child.
pub fn fn_00e3ec00(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    let mut total = e.mem.i32(this.wrapping_add(0x2c));
    if total <= 0 {
        return total as u32;
    }
    let mut index = 0u32;
    loop {
        let children = e.mem.u32(this.wrapping_add(0x28));
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        let below = fn_00e3ec00(e, Ptr::new(child));
        index += 1;
        total = total.wrapping_add(below as i32);
        if (index as i32) >= e.mem.i32(this.wrapping_add(0x2c)) {
            break;
        }
    }
    total as u32
}

// Translated from 00e3ed00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: finds the path from `this` down to `target` in the same kind of tree as
/// `00e3ec00`. `path` is an `hkArray` of node pointers (`data`, `size`, `capacity | flags`).
/// `this` is pushed (growing the array through `00c90940` when it is full: size equal to the
/// capacity bits). Returns 0 when `this` is `target` (the path stays), else the children are
/// tried in turn: a child that returns 0 makes this return 0 too, and when none does the node is
/// popped again and the last child result (1 for a node without children) is returned.
pub fn fn_00e3ed00(e: &mut Engine, this: Ptr, target: u32, path: u32) -> u32 {
    let this = this.addr();
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let capacity = e.mem.u32(path.wrapping_add(8)) & 0x3fff_ffff;
    if e.mem.u32(path.wrapping_add(4)) == capacity {
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        e.call(RESERVE_MORE, &args![allocator, path, 4u32]);
    }
    let size = e.mem.u32(path.wrapping_add(4));
    let data = e.mem.u32(path);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), this);
    let size = e.mem.u32(path.wrapping_add(4));
    e.mem.set_u32(path.wrapping_add(4), size.wrapping_add(1));
    if this == target {
        return 0;
    }
    let mut result = 1;
    if e.mem.i32(this.wrapping_add(0x2c)) > 0 {
        let mut index = 0u32;
        loop {
            let children = e.mem.u32(this.wrapping_add(0x28));
            let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
            result = fn_00e3ed00(e, Ptr::new(child), target, path);
            if result == 0 {
                return 0;
            }
            index += 1;
            if (index as i32) >= e.mem.i32(this.wrapping_add(0x2c)) {
                break;
            }
        }
    }
    let size = e.mem.u32(path.wrapping_add(4));
    e.mem.set_u32(path.wrapping_add(4), size.wrapping_sub(1));
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_ec00, fn_00e3ec00(Ptr) -> u32),
        entry!(0x00e3_ed00, fn_00e3ed00(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// A node with the given children.
    fn node(e: &mut Engine, children: &[u32]) -> u32 {
        let node = e.mem.alloc(0x40);
        let list = e.mem.alloc(4 * children.len() as u32 + 4);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, *child);
        }
        e.mem.set_u32(node + 0x28, list);
        e.mem.set_u32(node + 0x2c, children.len() as u32);
        node
    }

    /// `A(B, C(D))`: returns `[a, b, c, d]`.
    fn tree(e: &mut Engine) -> [u32; 4] {
        let d = node(e, &[]);
        let b = node(e, &[]);
        let c = node(e, &[d]);
        let a = node(e, &[b, c]);
        [a, b, c, d]
    }

    #[test]
    fn counts_every_node_below() {
        let mut e = engine();
        let [a, b, c, _] = tree(&mut e);
        assert_eq!(e.call(0x00e3_ec00, &args![a]).u32(), 3);
        assert_eq!(e.call(0x00e3_ec00, &args![c]).u32(), 1);
        assert_eq!(e.call(0x00e3_ec00, &args![b]).u32(), 0);
        // A negative count is returned as it is.
        e.mem.set_u32(b + 0x2c, 0xffff_fffe);
        assert_eq!(e.call(0x00e3_ec00, &args![b]).u32(), 0xffff_fffe);
    }

    /// The path array `{ data, size, capacity }` with room for `capacity` entries.
    fn path_array(e: &mut Engine, capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(4 * capacity + 4);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    fn router(e: &mut Engine) {
        e.mem.set_u32(0x0126_0010, 0x4444);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
    }

    fn entries(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn path_to_a_node_stays_in_the_array() {
        let mut e = engine();
        router(&mut e);
        let [a, _, c, d] = tree(&mut e);
        let array = path_array(&mut e, 8);
        assert_eq!(e.call(0x00e3_ed00, &args![a, d, array]).u32(), 0);
        assert_eq!(entries(&e, array), vec![a, c, d]);
    }

    #[test]
    fn path_to_the_node_itself_is_one_entry() {
        let mut e = engine();
        router(&mut e);
        let [a, ..] = tree(&mut e);
        let array = path_array(&mut e, 8);
        assert_eq!(e.call(0x00e3_ed00, &args![a, a, array]).u32(), 0);
        assert_eq!(entries(&e, array), vec![a]);
    }

    #[test]
    fn missing_target_pops_every_node_again() {
        let mut e = engine();
        router(&mut e);
        let [a, ..] = tree(&mut e);
        let array = path_array(&mut e, 8);
        assert_eq!(e.call(0x00e3_ed00, &args![a, 0x99u32, array]).u32(), 1);
        assert_eq!(entries(&e, array), Vec::<u32>::new());
        // A node without children returns 1 at once.
        let leaf = node(&mut e, &[]);
        assert_eq!(e.call(0x00e3_ed00, &args![leaf, 0x99u32, array]).u32(), 1);
    }

    #[test]
    fn full_array_is_grown_through_the_array_utility() {
        let mut e = engine();
        router(&mut e);
        let grown: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&grown);
        e.register_double(RESERVE_MORE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let data = e.mem.alloc(64);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let [a, ..] = tree(&mut e);
        let array = path_array(&mut e, 0);
        assert_eq!(e.call(0x00e3_ed00, &args![a, a, array]).u32(), 0);
        assert_eq!(*grown.borrow(), vec![vec![0x4444, array, 4]]);
        assert_eq!(entries(&e, array), vec![a]);
    }
}
