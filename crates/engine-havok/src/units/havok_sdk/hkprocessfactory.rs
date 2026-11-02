//! `Havok SDK/hkprocessfactory.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dd9580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_94c0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dd9580(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_94c0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The critical section imports (`EnterCriticalSection`, `DeleteCriticalSection`,
/// `LeaveCriticalSection`; stdcall, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `hkString::strCmp` (`00cb0fa0`, cdecl: two strings; zero when equal).
const STRING_COMPARE: u32 = 0x00cb_0fa0;
/// The array reservation (`00c908c0`, cdecl: allocator, array, capacity, element size).
const RESERVE_ARRAY: u32 = 0x00c9_08c0;
/// `hkStringPtr`: the constructor (`00cb0a60`, thiscall, no stack words), the assignment of a
/// string (`00cb0c10`, thiscall, `RET 4`) and the destructor (`00cb0c00`, thiscall).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, thiscall with the spin count; answers
/// the object).
const CRITICAL_SECTION_CONSTRUCT: u32 = 0x00cf_6da0;
/// The factory's vtable and the vtable of its base class (the destructor stores both).
const FACTORY_VTABLE: u32 = 0x010d_7c90;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The size of a registered process entry (a string pointer, the creator and the id).
const ENTRY_SIZE: u32 = 0xc;

// Translated from 00dd9380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkProcessFactory::registerProcess` (Xbox PDB), thiscall with the process name and the
/// creator function (`RET 8`); answers the process id. The factory's critical section (`+0x18`)
/// is entered; the entries (array: data `+0xC`, size `+0x10`, capacity word `+0x14`, 12 bytes
/// each) are searched for the name: an entry with no name matches a null name only, a null name
/// matches nothing else, and otherwise `hkString::strCmp` decides. A found entry gives its id
/// (`+8`). Otherwise the array grows by one (capacity raised to the larger of twice itself and
/// the new size through `00c908c0`), the new entry's string pointer is constructed and takes
/// the name, the entry gets the creator (`+4`) and the next id (the counter at `+8`, which then
/// increases). The critical section is left before the id is answered.
pub fn hk_process_factory_register_process(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    creator: u32,
) -> u32 {
    let me = this.addr();
    let section = e.mem.u32(me + 0x18);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let mut entry = 0;
    let mut index: i32 = 0;
    if e.mem.i32(me + 0x10) > 0 {
        let mut offset: u32 = 0;
        loop {
            let data = e.mem.u32(me + 0xc);
            let entry_name = e.mem.u32(offset.wrapping_add(data)) & 0xffff_fffe;
            let difference = if entry_name == 0 {
                // A null name differs only from a non-null one.
                if name != 0 {
                    u32::MAX
                } else {
                    0
                }
            } else if name == 0 {
                u32::MAX
            } else {
                e.call(STRING_COMPARE, &args![entry_name, name]).u32()
            };
            if difference == 0 {
                let data = e.mem.u32(me + 0xc);
                entry = data.wrapping_add((index as u32).wrapping_mul(ENTRY_SIZE));
                break;
            }
            index += 1;
            offset = offset.wrapping_add(ENTRY_SIZE);
            if index >= e.mem.i32(me + 0x10) {
                break;
            }
        }
    }
    if entry == 0 {
        let array = me + 0xc;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = e.mem.i32(array + 4);
        let wanted = size.wrapping_add(1);
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < wanted {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if wanted >= doubled { wanted } else { doubled };
            e.call(
                RESERVE_ARRAY,
                &args![allocator, array, wanted as u32, ENTRY_SIZE],
            );
        }
        let data = e.mem.u32(array);
        let slot = data.wrapping_add((e.mem.u32(array + 4)).wrapping_mul(ENTRY_SIZE));
        if slot != 0 {
            e.call(STRING_PTR_CONSTRUCT, &args![slot]);
        }
        let data = e.mem.u32(array);
        e.mem
            .set_u32(array + 4, e.mem.u32(array + 4).wrapping_add(1));
        entry = data.wrapping_add((size as u32).wrapping_mul(ENTRY_SIZE));
        e.call(STRING_PTR_ASSIGN, &args![entry, name]);
        e.mem.set_u32(entry + 4, creator);
        let next = e.mem.u32(me + 8);
        e.mem.set_u32(entry + 8, next);
        e.mem.set_u32(me + 8, next.wrapping_add(1));
    }
    let section = e.mem.u32(me + 0x18);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    e.mem.u32(entry + 8)
}

// Translated from 00dd9460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The factory constructor (fastcall; answers `this`): the 16-bit word at `+6` is 1, the vtable
/// is `010d7c90`, the id counter (`+8`), the entry array (`+0xC` data, `+0x10` size) are zero
/// and its capacity word (`+0x14`) is `0x80000000`; a critical section of 0x18 bytes is taken
/// from the memory router's allocator (slot 4) and constructed with the spin count 2000 and
/// kept at `+0x18` (null when the allocator answers 0).
pub fn hk_process_factory_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, FACTORY_VTABLE);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x14, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x18u32]).u32();
    if block != 0 {
        let section = e
            .call(CRITICAL_SECTION_CONSTRUCT, &args![block, 2000u32])
            .u32();
        e.mem.set_u32(me + 0x18, section);
    } else {
        e.mem.set_u32(me + 0x18, 0);
    }
    this
}

// Translated from 00dd94c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkProcessFactory::~hkProcessFactory` (Xbox PDB), fastcall: stores the factory vtable; a
/// critical section (`+0x18`) is deleted and returned to the allocator (slot 8, `0x18` bytes);
/// the string pointer of every entry is destructed (`00cb0c00`), the size becomes 0 and the
/// entry storage is freed (slot `0x10`, data and capacity times 12 bytes) unless the capacity
/// word is negative; the data pointer is cleared, the capacity word is `0x80000000` and the
/// base-class vtable `0102e388` is stored.
pub fn hk_process_factory_destructor(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let section = e.mem.u32(me + 0x18);
    e.mem.set_u32(me, FACTORY_VTABLE);
    if section != 0 {
        e.call(DELETE_CRITICAL_SECTION, &args![section]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![section, 0x18u32]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.i32(me + 0x10);
    let mut entry = e.mem.u32(me + 0xc);
    if size > 0 {
        let mut remaining = size;
        loop {
            e.call(STRING_PTR_DESTRUCT, &args![entry]);
            entry = entry.wrapping_add(ENTRY_SIZE);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
    let capacity = e.mem.u32(me + 0x14);
    e.mem.set_u32(me + 0x10, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0xc);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(ENTRY_SIZE);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x14, 0x8000_0000);
    e.mem.set_u32(me, BASE_VTABLE);
}

// Translated from 00dd9550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 0x1c bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10), stores
/// the size as a 16-bit word at +4 and ends with a jump to the constructor `00dd9460` (`this` in
/// `ECX`, no stack words), whose answer it returns. Ghidra made no function here; the address is stored in the data word at 011b83b0.
pub fn fn_00dd9550(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(block + 4, 0x1c);
    e.call(0x00dd_9460, &args![block]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_9380, hk_process_factory_register_process(Ptr, u32, u32) -> u32),
        entry!(0x00dd_9460, hk_process_factory_construct(Ptr) -> Ptr),
        entry!(0x00dd_94c0, hk_process_factory_destructor(Ptr)),
        entry!(0x00dd_9580, deleting_destructor_00dd9580(Ptr, u8) -> Ptr),
        entry!(0x00dd_9550, fn_00dd9550() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double, new_log, Log};
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dd9580_frees_only_when_bit_zero_is_set() {
        check(0x00dd_9580, 0x00dd_94c0);
    }

    /// The allocator slots 4 (allocate), 8 (free a block) and `0x10` (free an array).
    const ALLOCATE: u32 = 0x0710_0004;
    const FREE_BLOCK: u32 = 0x0710_0008;
    const FREE_ARRAY: u32 = 0x0710_0010;

    fn c_string(e: &mut Engine, text: &str) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(at, text.as_bytes());
        e.mem.set_u8(at + text.len() as u32, 0);
        at
    }

    fn read_c_string(e: &Engine, mut at: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        while e.mem.u8(at) != 0 {
            bytes.push(e.mem.u8(at));
            at += 1;
        }
        bytes
    }

    /// The doubles around the factory: the critical section imports, the string comparison
    /// (byte order of the two C strings), the array reservation (a fresh buffer of the requested
    /// capacity with the entries copied, capacity word set), the string pointer methods
    /// (the assignment stores the name in the entry), and the memory router whose allocator
    /// answers `block` for an allocation. All calls go to the log. Returns the allocator.
    fn factory_doubles(e: &mut Engine, log: &Log, block: u32) -> u32 {
        for address in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            STRING_PTR_CONSTRUCT,
            STRING_PTR_DESTRUCT,
            FREE_BLOCK,
            FREE_ARRAY,
        ] {
            log_double(e, log, address, 0);
        }
        log_double(e, log, ALLOCATE, block);
        let logged = Rc::clone(log);
        e.register_double(STRING_COMPARE, move |e, a| {
            logged.borrow_mut().push((STRING_COMPARE, a.to_vec()));
            let (first, second) = (read_c_string(e, a[0]), read_c_string(e, a[1]));
            let order = match first.cmp(&second) {
                std::cmp::Ordering::Less => -1i32,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            Ret {
                eax: order as u32,
                ..Ret::default()
            }
        });
        let logged = Rc::clone(log);
        e.register_double(STRING_PTR_ASSIGN, move |e, a| {
            logged.borrow_mut().push((STRING_PTR_ASSIGN, a.to_vec()));
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let logged = Rc::clone(log);
        e.register_double(RESERVE_ARRAY, move |e, a| {
            logged.borrow_mut().push((RESERVE_ARRAY, a.to_vec()));
            let (array, capacity, element) = (a[1], a[2], a[3]);
            let (old, size) = (e.mem.u32(array), e.mem.u32(array + 4));
            let new = e.mem.alloc((capacity * element).max(8));
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(new + byte, value);
            }
            e.mem.set_u32(array, new);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 4, ALLOCATE);
        e.mem.set_u32(table + 8, FREE_BLOCK);
        e.mem.set_u32(table + 0x10, FREE_ARRAY);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, log, MEMORY_ROUTER, router);
        allocator
    }

    /// A factory (0x40 bytes) with the critical section `0x5ec0` and no entries.
    fn factory(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x5ec0);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        this
    }

    /// An entry array of `entries` (name word, creator, id) in the factory.
    fn fill_entries(e: &mut Engine, this: u32, entries: &[(u32, u32, u32)]) {
        let data = e.mem.alloc(0xc * entries.len() as u32 + 8);
        for (index, (name, creator, id)) in entries.iter().enumerate() {
            let entry = data + 0xc * index as u32;
            e.mem.set_u32(entry, *name);
            e.mem.set_u32(entry + 4, *creator);
            e.mem.set_u32(entry + 8, *id);
        }
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, entries.len() as u32);
        e.mem.set_u32(this + 0x14, entries.len() as u32);
    }

    #[test]
    fn registering_new_names_grows_the_entry_array_00dd9380() {
        let mut e = engine();
        let log = new_log();
        factory_doubles(&mut e, &log, 0);
        let this = factory(&mut e);
        e.mem.set_u32(this + 8, 5);
        let (first, second) = (c_string(&mut e, "Alpha"), c_string(&mut e, "Beta"));
        assert_eq!(e.call(0x00dd_9380, &args![this, first, 0x111u32]).u32(), 5);
        let data = e.mem.u32(this + 0xc);
        // The empty array (capacity 0) was reserved for one entry; the new one was constructed
        // in place, given the name, the creator and the id.
        let allocator = calls(&log, RESERVE_ARRAY)[0][0];
        assert_eq!(
            calls(&log, RESERVE_ARRAY),
            vec![vec![allocator, this + 0xc, 1, 0xc]]
        );
        assert_eq!(calls(&log, STRING_PTR_CONSTRUCT), vec![vec![data]]);
        assert_eq!(calls(&log, STRING_PTR_ASSIGN), vec![vec![data, first]]);
        assert_eq!(e.mem.u32(data), first);
        assert_eq!(e.mem.u32(data + 4), 0x111);
        assert_eq!(e.mem.u32(data + 8), 5);
        assert_eq!(e.mem.u32(this + 8), 6);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        // The next name is compared with the first, then the array grows to two entries.
        assert_eq!(e.call(0x00dd_9380, &args![this, second, 0x222u32]).u32(), 6);
        assert_eq!(calls(&log, STRING_COMPARE), vec![vec![first, second]]);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(calls(&log, RESERVE_ARRAY)[1][2..], [2, 0xc]);
        assert_eq!(calls(&log, STRING_PTR_CONSTRUCT)[1], vec![data + 0xc]);
        assert_eq!(e.mem.u32(data), first);
        assert_eq!(e.mem.u32(data + 0xc), second);
        assert_eq!(e.mem.u32(data + 0x10), 0x222);
        assert_eq!(e.mem.u32(data + 0x14), 6);
        // The critical section was entered and left around each call.
        assert_eq!(calls(&log, ENTER_CRITICAL_SECTION), vec![vec![0x5ec0]; 2]);
        assert_eq!(calls(&log, LEAVE_CRITICAL_SECTION), vec![vec![0x5ec0]; 2]);
    }

    #[test]
    fn a_registered_name_gives_its_id_again_00dd9380() {
        let mut e = engine();
        let log = new_log();
        factory_doubles(&mut e, &log, 0);
        let this = factory(&mut e);
        let (one, two) = (c_string(&mut e, "One"), c_string(&mut e, "Two"));
        // The stored name pointer has its flag bit set; the lookup ignores it.
        fill_entries(&mut e, this, &[(one | 1, 0x10, 8), (two, 0x20, 9)]);
        e.mem.set_u32(this + 8, 10);
        let lookup = c_string(&mut e, "Two");
        assert_eq!(e.call(0x00dd_9380, &args![this, lookup, 0x999u32]).u32(), 9);
        assert_eq!(
            calls(&log, STRING_COMPARE),
            vec![vec![one, lookup], vec![two, lookup]]
        );
        assert!(calls(&log, RESERVE_ARRAY).is_empty());
        assert!(calls(&log, STRING_PTR_CONSTRUCT).is_empty());
        assert_eq!(e.mem.u32(this + 8), 10);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(data + 0xc + 4), 0x20);
        let lookup = c_string(&mut e, "One");
        assert_eq!(e.call(0x00dd_9380, &args![this, lookup, 0u32]).u32(), 8);
    }

    #[test]
    fn null_names_match_only_null_names_00dd9380() {
        let mut e = engine();
        let log = new_log();
        factory_doubles(&mut e, &log, 0);
        let this = factory(&mut e);
        let named = c_string(&mut e, "Named");
        fill_entries(&mut e, this, &[(0, 0x10, 3), (named, 0x20, 4)]);
        e.mem.set_u32(this + 8, 5);
        // A null name finds the null entry without comparing anything.
        assert_eq!(e.call(0x00dd_9380, &args![this, 0u32, 0x1u32]).u32(), 3);
        assert!(calls(&log, STRING_COMPARE).is_empty());
        // A name skips the null entry and finds its own.
        let again = c_string(&mut e, "Named");
        assert_eq!(e.call(0x00dd_9380, &args![this, again, 0x1u32]).u32(), 4);
        assert_eq!(calls(&log, STRING_COMPARE), vec![vec![named, again]]);
        // A null name differs from a named entry: with only that one, it is added.
        let other = factory(&mut e);
        fill_entries(&mut e, other, &[(named, 0x20, 4)]);
        e.mem.set_u32(other + 8, 5);
        assert_eq!(e.call(0x00dd_9380, &args![other, 0u32, 0x7u32]).u32(), 5);
        assert_eq!(calls(&log, STRING_COMPARE).len(), 1);
        assert_eq!(calls(&log, STRING_PTR_ASSIGN).len(), 1);
        assert_eq!(e.mem.u32(other + 0x10), 2);
    }

    #[test]
    fn the_constructor_takes_a_critical_section_from_the_allocator_00dd9460() {
        let mut e = engine();
        let log = new_log();
        factory_doubles(&mut e, &log, 0x7000);
        log_double(&mut e, &log, CRITICAL_SECTION_CONSTRUCT, 0x7008);
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xcd; 0x40]);
        assert_eq!(e.call(0x00dd_9460, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u16(this + 4), 0xcdcd);
        assert_eq!(e.mem.u32(this), 0x010d_7c90);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(calls(&log, ALLOCATE)[0][1], 0x18);
        assert_eq!(
            calls(&log, CRITICAL_SECTION_CONSTRUCT),
            vec![vec![0x7000, 2000]]
        );
        // The section kept is what the constructor answered.
        assert_eq!(e.mem.u32(this + 0x18), 0x7008);
    }

    #[test]
    fn the_constructor_keeps_no_section_when_the_allocator_fails_00dd9460() {
        let mut e = engine();
        let log = new_log();
        factory_doubles(&mut e, &log, 0);
        log_double(&mut e, &log, CRITICAL_SECTION_CONSTRUCT, 0x7008);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0xcdcd_cdcd);
        assert_eq!(e.call(0x00dd_9460, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert!(calls(&log, CRITICAL_SECTION_CONSTRUCT).is_empty());
    }

    #[test]
    fn the_destructor_releases_the_section_the_entries_and_the_storage_00dd94c0() {
        let mut e = engine();
        let log = new_log();
        let allocator = factory_doubles(&mut e, &log, 0);
        let this = factory(&mut e);
        fill_entries(&mut e, this, &[(0x11, 1, 0), (0x22, 2, 1), (0x33, 3, 2)]);
        e.mem.set_u32(this + 0x14, 0x4000_0005);
        let data = e.mem.u32(this + 0xc);
        e.call(0x00dd_94c0, &args![this]);
        assert_eq!(calls(&log, DELETE_CRITICAL_SECTION), vec![vec![0x5ec0]]);
        assert_eq!(calls(&log, FREE_BLOCK), vec![vec![allocator, 0x5ec0, 0x18]]);
        assert_eq!(
            calls(&log, STRING_PTR_DESTRUCT),
            vec![vec![data], vec![data + 0xc], vec![data + 0x18]]
        );
        assert_eq!(calls(&log, FREE_ARRAY), vec![vec![allocator, data, 60]]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn the_destructor_skips_what_does_not_exist_00dd94c0() {
        let mut e = engine();
        let log = new_log();
        factory_doubles(&mut e, &log, 0);
        let this = factory(&mut e);
        e.mem.set_u32(this + 0x18, 0);
        e.mem.set_u32(this + 0xc, 0x8000);
        e.call(0x00dd_94c0, &args![this]);
        assert!(calls(&log, DELETE_CRITICAL_SECTION).is_empty());
        assert!(calls(&log, FREE_BLOCK).is_empty());
        assert!(calls(&log, STRING_PTR_DESTRUCT).is_empty());
        // The capacity word has the flag bit: the storage is not the array's to free.
        assert!(calls(&log, FREE_ARRAY).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// The constructor sees the block with its size stored and its answer is returned.
    #[test]
    fn fn_00dd9550_allocates_and_hands_the_block_to_the_constructor() {
        let (mut e, sizes) = allocator_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00dd_9460, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00dd_9550, &args![]).u32(), 0x1234);
        assert_eq!(*sizes.borrow(), vec![0x1c]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0.len(), 1);
        assert_eq!(seen[0].1, 0x1c);
    }
}
