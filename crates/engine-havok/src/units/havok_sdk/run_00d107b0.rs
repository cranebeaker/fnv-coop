//! `Havok SDK/run_00d107b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d107b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): sets the `u16` at `+6` to 1 (the reference count
/// of the `hkReferencedObject` layout), the word at `+8` to 0 and the vtable word at `+0` to
/// 0x010c_d54c. Returns `this`.
pub fn fn_00d107b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr(), 0x010c_d54c);
    this
}

// Translated from 00d107d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (class unconfirmed): sets the `u16` at `+6` to 1 (the reference count of the
/// `hkReferencedObject` layout), the word at `+8` to `0x010c_63bc` and then to `0x010c_d588`, the
/// vtable word at `+0` to `0x010c_d598` and the word at `+0xc` to `link`. Returns `this`.
pub fn fn_00d107d0(e: &mut Engine, this: Ptr, link: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0x010c_63bc);
    e.mem.set_u32(base, 0x010c_d598);
    e.mem.set_u32(base + 8, 0x010c_d588);
    e.mem.set_u32(base + 0xc, link);
    this
}

/// `hkpWorldAgentUtil::addAgent` (Xbox PDB, `00cc0f40`, cdecl `(first collidable, second
/// collidable, collision input)`).
const ADD_AGENT: u32 = 0x00cc_0f40;

// Translated from 00d10800 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (it is the broad-phase listener function that matches
/// `removeCollisionPair`, so most likely `addCollisionPair`, unconfirmed). Thiscall with one
/// stack word, a pair of broad-phase handles (two pointers, each handle giving its collidable
/// at its own address plus the signed byte at its +5). `this + 4` is the word whose `+0x78`
/// is the collision input. The collision input's table (its first word) maps the two collidables'
/// signed type bytes at +0x1a to a byte at `table + 0x1bb0 + 10 * type of the first + type of
/// the second`; when that byte is not zero, the input's word at +0x14 is set to the word at
/// `table + 0x1c30 + 64 * that byte` and `addAgent(first, second, input)` runs.
pub fn fn_00d10800(e: &mut Engine, this: Ptr, pair: Ptr) {
    let owner = e.mem.u32(this.addr() + 4);
    let pair = pair.addr();
    let first_handle = e.mem.u32(pair);
    let first = first_handle.wrapping_add(i32::from(e.mem.i8(first_handle + 5)) as u32);
    let second_handle = e.mem.u32(pair + 4);
    let second = second_handle.wrapping_add(i32::from(e.mem.i8(second_handle + 5)) as u32);
    let second_type = i32::from(e.mem.i8(second + 0x1a));
    let input = e.mem.u32(owner + 0x78);
    let first_type = i32::from(e.mem.i8(first + 0x1a));
    let table = e.mem.u32(input);
    let selector = e.mem.u8((second_type as u32)
        .wrapping_add(table)
        .wrapping_add((first_type.wrapping_mul(5) as u32).wrapping_mul(2))
        .wrapping_add(0x1bb0));
    if selector != 0 {
        let offset = (i32::from(selector as i8) as u32) << 6;
        let value = e.mem.u32(offset.wrapping_add(table).wrapping_add(0x1c30));
        e.mem.set_u32(input + 0x14, value);
        e.call(ADD_AGENT, &args![first, second, input]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_0800, fn_00d10800(Ptr, Ptr)),
        entry!(0x00d107b0, fn_00d107b0(Ptr) -> Ptr),
        entry!(0x00d1_07d0, fn_00d107d0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d107b0_sets_count_link_and_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for offset in (0..0x10).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        let result = e.call(0x00d1_07b0, &args![this]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x010c_d54c);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ffff);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use crate::prelude::*;

    #[test]
    fn fn_00d107d0_sets_count_vtables_and_link() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        let result = e.call(0x00d1_07d0, &args![this, 0x1234u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x010c_d598);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x010c_d588);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
    }
}

#[cfg(test)]
mod physics_listener_tests {
    use super::*;

    #[test]
    fn fn_00d10800_adds_an_agent_when_the_pair_table_says_so() {
        let mut e = Engine::new();
        e.map(0x0200_0000, 0x4000);
        // The table: byte at 0x1bb0 + 10 * 2 + 3 selects entry 2 (value at 0x1c30 + 0x80).
        let table = 0x0200_0000;
        e.mem.set_u8(table + 0x1bb0 + 20 + 3, 2);
        e.mem.set_u32(table + 0x1c30 + 0x80, 0xfeed);
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input, table);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x78, input);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, world);
        // Two handles, each giving its collidable 8 bytes further (byte 8 at +5).
        let mut collidables = [0u32; 2];
        let mut handles = [0u32; 2];
        for (i, type_byte) in [2i8, 3].into_iter().enumerate() {
            let block = e.mem.alloc(0x40);
            e.mem.set_i8(block + 5, 8);
            e.mem.set_i8(block + 8 + 0x1a, type_byte);
            handles[i] = block;
            collidables[i] = block + 8;
        }
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, handles[0]);
        e.mem.set_u32(pair + 4, handles[1]);
        e.register(ADD_AGENT, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00d1_0800, &args![this, pair]);
        let log = e.call_log.take().unwrap();
        let added: Vec<_> = log.iter().filter(|(a, _)| *a == ADD_AGENT).collect();
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].1, vec![collidables[0], collidables[1], input]);
        assert_eq!(e.mem.u32(input + 0x14), 0xfeed);
        // Selector byte zero: nothing happens.
        e.mem.set_u8(table + 0x1bb0 + 20 + 3, 0);
        e.mem.set_u32(input + 0x14, 0);
        e.call_log = Some(Vec::new());
        e.call(0x00d1_0800, &args![this, pair]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|(a, _)| *a != ADD_AGENT));
        assert_eq!(e.mem.u32(input + 0x14), 0);
    }
}
