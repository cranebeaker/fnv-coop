//! `Havok SDK/run_00d60b40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d60b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, `RET 8`: two stack words `source` and `flag` (a byte); no map name, class
/// unconfirmed): the `u16` at `+6` = 1, the vtable word `+0` = `0x010d_14f8`, the byte at `+0xc` =
/// `flag`; makes a 0x14-byte block through slot 4 of the router's allocator (its `u16` at `+4` =
/// 0x14), builds it with `00cc06f0` on the block with `(source, 1)` and stores the answer at `+8`.
/// Returns `this`.
pub fn fn_00d60b40(e: &mut Engine, this: Ptr, source: u32, flag: u8) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, 0x010d_14f8);
    e.mem.set_u8(me + 0xc, flag);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    e.mem.set_u16(block + 4, 0x14);
    let built = e.call(0x00cc_06f0, &args![block, source, 1u32]).u32();
    e.mem.set_u32(me + 8, built);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d6_0b40, fn_00d60b40(Ptr, u32, u8) -> Ptr)]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d60b40_makes_the_block_and_stores_the_answer() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 4, 0x0900_0151);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x0900_0151, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(0x00cc_06f0, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00d6_0b40, &args![this, 0x3000u32, 0x1_0007u32])
            .u32();
        assert_eq!(result, this);
        let log = e.call_log.take().unwrap();
        let block = log[1..]
            .iter()
            .find(|(address, _)| *address == 0x00cc_06f0)
            .unwrap()
            .1[0];
        assert_eq!(
            log[1..],
            [
                (MEMORY_ROUTER, vec![]),
                (0x0900_0151, vec![allocator, 0x14]),
                (0x00cc_06f0, vec![block, 0x3000, 1]),
            ]
        );
        assert_eq!(e.mem.u16(block + 4), 0x14);
        assert_eq!(e.mem.u32(this + 8), block + 0x1000);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010d_14f8);
        assert_eq!(e.mem.u8(this + 0xc), 7);
    }
}
