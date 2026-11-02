//! `Havok SDK/run_00d9e3f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread's memory router (`00c85750`); its `+0x10` is the allocator `00c90940` takes.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90940` (`hkarrayutil.cpp`): cdecl `(allocator, array, element size)`, makes room for one
/// more element.
const ARRAY_GROW: u32 = 0x00c9_0940;

// Translated from 00d9e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall with three stack words `(a, b, c)`, `RET 0xc`: appends a 16-byte element `(a, b, c,
/// counter)` to the array at `this + 0x20` (data, size at `+4`, capacity word at `+8`), growing
/// it through `00c90940` first when its size equals its capacity (the low 30 bits of the word),
/// where `counter` is the word at `this + 0xc`, which is then incremented. The class is
/// unconfirmed (`hkpEndOfStepCallbackUtil`'s step list).
pub fn fn_00d9e3f0(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) {
    let this = this.addr();
    let array = this.wrapping_add(0x20);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 0x10u32]);
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    let element = size.wrapping_shl(4).wrapping_add(e.mem.u32(array));
    e.mem.set_u32(element, a);
    e.mem.set_u32(element + 4, b);
    e.mem.set_u32(element + 8, c);
    let counter = e.mem.u32(this + 0xc);
    e.mem.set_u32(element + 0xc, counter);
    e.mem.set_u32(this + 0xc, counter.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_e3f0, fn_00d9e3f0(Ptr, u32, u32, u32))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    fn setup(e: &mut Engine, capacity_word: u32, size: u32) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, data);
        e.mem.set_u32(this + 0x24, size);
        e.mem.set_u32(this + 0x28, capacity_word);
        e.mem.set_u32(this + 0xc, 7);
        (this, allocator)
    }

    #[test]
    fn an_element_is_appended_with_the_running_counter() {
        let mut e = Engine::new();
        let (this, _) = setup(&mut e, 4, 1);
        e.register(ARRAY_GROW, |_, _| panic!("no growth wanted"));
        e.call(0x00d9_e3f0, &args![this, 0x11u32, 0x22u32, 0x33u32]);
        let data = e.mem.u32(this + 0x20);
        assert_eq!(e.mem.u32(this + 0x24), 2);
        assert_eq!(
            [0, 4, 8, 0xc].map(|offset| e.mem.u32(data + 0x10 + offset)),
            [0x11, 0x22, 0x33, 7]
        );
        assert_eq!(e.mem.u32(this + 0xc), 8);
        e.call(0x00d9_e3f0, &args![this, 0x44u32, 0x55u32, 0x66u32]);
        assert_eq!(e.mem.u32(data + 0x20 + 0xc), 8);
        assert_eq!(e.mem.u32(this + 0xc), 9);
    }

    #[test]
    fn a_full_array_is_grown_first_and_the_flag_bits_do_not_count_as_capacity() {
        let mut e = Engine::new();
        // Capacity 2 with a flag bit above: the size 2 equals the capacity.
        let (this, allocator) = setup(&mut e, 0x8000_0002, 2);
        let grows = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = std::rc::Rc::clone(&grows);
        e.register_double(ARRAY_GROW, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            // The grown array has a new buffer.
            let room = e.mem.alloc(0x80);
            e.mem.set_u32(a[1], room);
            Ret::default()
        });
        e.call(0x00d9_e3f0, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(*grows.borrow(), [vec![allocator, this + 0x20, 0x10]]);
        let data = e.mem.u32(this + 0x20);
        assert_eq!(e.mem.u32(data + 0x20), 1);
        assert_eq!(e.mem.u32(this + 0x24), 3);
    }
}
