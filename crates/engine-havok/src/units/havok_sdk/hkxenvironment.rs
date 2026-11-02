//! `Havok SDK/hkxenvironment.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e3f8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e3_f820`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e3f8a0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e3_f820, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArrayUtil` growth routine (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkString::strCasecmp(a, b)` (`00cb0fe0`, cdecl).
const STRING_CASECMP: u32 = 0x00cb_0fe0;
/// `hkStringPtr`: the constructor (`00cb0a60`), the destructor (`00cb0c00`), the assignment from
/// a `const char *` (`00cb0c10`, one stack word) and the copy constructor (`00cb0bd0`, one stack
/// word), all `thiscall`.
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN_TEXT: u32 = 0x00cb_0c10;
const STRING_PTR_COPY_CONSTRUCT: u32 = 0x00cb_0bd0;
/// The base class's vtable (`0102e388`) and the one `00e3f510` writes (`010ebab8`).
const BASE_VTABLE: u32 = 0x0102_e388;
const ENVIRONMENT_VTABLE: u32 = 0x010e_bab8;

// Translated from 00e3f230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxEnvironment::findVariableByName` (Xbox PDB), `thiscall`, `RET 4` (`name`): the index of
/// the first of the variables (data at `+8`, count at `+0xc`, 8 bytes each: name and value
/// `hkStringPtr`s) whose name (the word with its low bit cleared) equals `name`
/// case-insensitively (`hkString::strCasecmp`, `00cb0fe0`); -1 when there is none.
pub fn hkx_environment_find_variable_by_name(e: &mut Engine, this: Ptr, name: u32) -> i32 {
    let base = this.addr();
    let mut index = 0i32;
    while index < e.mem.i32(base + 0xc) {
        let data = e.mem.u32(base + 8);
        let entry_name = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(8))) & 0xffff_fffe;
        if e.call(STRING_CASECMP, &args![entry_name, name]).u32() == 0 {
            return index;
        }
        index += 1;
    }
    -1
}

// Translated from 00e3f370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): destroys the name and value
/// `hkStringPtr`s of every variable (`00cb0c00`, value first) and sets the count at `+0xc` to
/// zero.
pub fn fn_00e3f370(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = e.mem.i32(base + 0xc);
    let mut variable = e.mem.u32(base + 8);
    let mut left = count;
    while left > 0 {
        e.call(STRING_PTR_DESTRUCT, &args![variable.wrapping_add(4)]);
        e.call(STRING_PTR_DESTRUCT, &args![variable]);
        variable = variable.wrapping_add(8);
        left -= 1;
    }
    e.mem.set_u32(base + 0xc, 0);
}

// Translated from 00e3f3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxEnvironment::setVariable` (Xbox PDB), `thiscall`, `RET 8` (`name`, `value`): sets or
/// removes a variable. A null `value` removes the variable `findVariableByName` finds (1 is
/// returned when there is none): its two `hkStringPtr`s are destroyed, the count goes down by
/// one and, unless the removed one was the last, the last variable's two words are copied
/// over it. With a value, an existing variable's value `hkStringPtr` is assigned the text
/// (`00cb0c10`); otherwise a new variable is added: two temporary `hkStringPtr`s hold the name
/// and the value, the array grows when full (`00c90940`, element size 8), they are
/// copy-constructed into the new entry (`00cb0bd0`), the count goes up and the temporaries
/// are destroyed. Returns 0 (or 1 as said).
pub fn hkx_environment_set_variable(e: &mut Engine, this: Ptr, name: u32, value: u32) -> u32 {
    let base = this.addr();
    let index = hkx_environment_find_variable_by_name(e, this, name);
    if value == 0 {
        if index == -1 {
            return 1;
        }
        let entry = e
            .mem
            .u32(base + 8)
            .wrapping_add((index as u32).wrapping_mul(8));
        e.call(STRING_PTR_DESTRUCT, &args![entry.wrapping_add(4)]);
        e.call(STRING_PTR_DESTRUCT, &args![entry]);
        let count = e.mem.u32(base + 0xc).wrapping_sub(1);
        e.mem.set_u32(base + 0xc, count);
        if count as i32 != index {
            let data = e.mem.u32(base + 8);
            let last = data.wrapping_add(count.wrapping_mul(8));
            let target = data.wrapping_add((index as u32).wrapping_mul(8));
            for word in 0..2 {
                let moved = e.mem.u32(last + 4 * word);
                e.mem.set_u32(target + 4 * word, moved);
            }
        }
        return 0;
    }
    if index != -1 {
        let entry = e
            .mem
            .u32(base + 8)
            .wrapping_add((index as u32).wrapping_mul(8));
        e.call(STRING_PTR_ASSIGN_TEXT, &args![entry.wrapping_add(4), value]);
        return 0;
    }
    e.with_stack(8, |e, temporaries| {
        let name_temporary = temporaries.addr();
        let value_temporary = temporaries.addr() + 4;
        e.call(STRING_PTR_CONSTRUCT, &args![name_temporary]);
        e.call(STRING_PTR_CONSTRUCT, &args![value_temporary]);
        e.call(STRING_PTR_ASSIGN_TEXT, &args![name_temporary, name]);
        e.call(STRING_PTR_ASSIGN_TEXT, &args![value_temporary, value]);
        let array = base + 8;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
        if e.mem.u32(array + 4) == capacity {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 8u32]);
        }
        let size = e.mem.u32(array + 4);
        let entry = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
        if entry != 0 {
            e.call(STRING_PTR_COPY_CONSTRUCT, &args![entry, name_temporary]);
            e.call(
                STRING_PTR_COPY_CONSTRUCT,
                &args![entry.wrapping_add(4), value_temporary],
            );
        }
        e.mem.set_u32(array + 4, size.wrapping_add(1));
        e.call(STRING_PTR_DESTRUCT, &args![value_temporary]);
        e.call(STRING_PTR_DESTRUCT, &args![name_temporary]);
    });
    0
}

// Translated from 00e3f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: one word the code never reads): stores the
/// vtable `010ebab8` at `this` and returns `this`.
pub fn fn_00e3f510(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ENVIRONMENT_VTABLE);
    this
}

// Translated from 00e3f820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxEnvironment::~hkxEnvironment` (Xbox PDB), `thiscall`, no stack words: for each of the
/// variables (count at `+0xc`, read after the memory router's allocator) destroys the value
/// then the name `hkStringPtr` (`00cb0c00`), clears the count, gives the array back
/// (`bufFree(data, (capacity & 0x3fffffff) * 8)`) unless its capacity word at `+0x10` is
/// negative, clears the data pointer, sets the capacity word to `0x80000000` and the vtable to
/// the base class's (`0102e388`).
pub fn hkx_environment_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.i32(base + 0xc);
    let mut variable = e.mem.u32(base + 8);
    let mut left = count;
    while left > 0 {
        e.call(STRING_PTR_DESTRUCT, &args![variable.wrapping_add(4)]);
        e.call(STRING_PTR_DESTRUCT, &args![variable]);
        variable = variable.wrapping_add(8);
        left -= 1;
    }
    let capacity = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0xc, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 8);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(8)],
        );
    }
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, 0x8000_0000);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_f8a0, deleting_destructor_00e3f8a0(Ptr, u8) -> Ptr),
        entry!(0x00e3f230, hkx_environment_find_variable_by_name(Ptr, u32) -> i32),
        entry!(0x00e3f370, fn_00e3f370(Ptr)),
        entry!(0x00e3f3b0, hkx_environment_set_variable(Ptr, u32, u32) -> u32),
        entry!(0x00e3f510, fn_00e3f510(Ptr, u32) -> Ptr),
        entry!(0x00e3f820, hkx_environment_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn deleting_destructor_00e3f8a0_frees_only_when_bit_zero_is_set() {
        check(0x00e3_f8a0, 0x00e3_f820);
    }

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Registers a double at `addr` that logs its argument words and answers `answer`.
    fn log_calls(e: &mut Engine, addr: u32, answer: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let kept = log.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    const ALLOCATOR_OBJECT: u32 = 0x0060_0000;
    const ALLOCATOR_TABLE: u32 = 0x0060_1000;
    const DELETE_DOUBLE: u32 = 0x0060_2008;
    const FREE_DOUBLE: u32 = 0x0060_2010;

    /// A memory router (`00c85750`) whose allocator logs `[block, size]` for slot `+0x10`
    /// (`bufFree`, the first log) and for slot `+8` (the second log).
    fn install_memory(e: &mut Engine) -> (Log, Log) {
        e.map(ALLOCATOR_OBJECT, 0x100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR_OBJECT);
        e.mem.set_u32(ALLOCATOR_OBJECT, ALLOCATOR_TABLE);
        let mut table = vec![0u32; 5];
        table[2] = DELETE_DOUBLE;
        table[4] = FREE_DOUBLE;
        e.put_vtable(ALLOCATOR_TABLE, &table);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees: Log = Rc::new(RefCell::new(Vec::new()));
        let deletes: Log = Rc::new(RefCell::new(Vec::new()));
        let log = frees.clone();
        e.register_double(FREE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        let log = deletes.clone();
        e.register_double(DELETE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        (frees, deletes)
    }

    /// A case-insensitive `hkString::strCasecmp` double over C strings in memory.
    fn install_casecmp(e: &mut Engine) {
        e.register(STRING_CASECMP, |e, a| {
            let left: Vec<u8> = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right: Vec<u8> = e.mem.cstr(a[1]).to_ascii_lowercase();
            Ret {
                eax: u32::from(left != right),
                ..Ret::default()
            }
        });
    }

    fn put_text(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, text);
        at
    }

    /// An environment with the variables `names[i]` = `values[i]` (each name and value word
    /// is a C string pointer; the variables sit in an array at +8 with count +0xc).
    fn environment(e: &mut Engine, names: &[&str]) -> (u32, u32) {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8 * 4);
        for (index, name) in names.iter().enumerate() {
            let text = put_text(e, name.as_bytes());
            // The low bit of a hkStringPtr word is a flag: it is masked off.
            e.mem.set_u32(data + 8 * index as u32, text | 1);
            e.mem
                .set_u32(data + 8 * index as u32 + 4, 0x7000 + index as u32);
        }
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, names.len() as u32);
        e.mem.set_u32(this + 0x10, 4);
        (this, data)
    }

    #[test]
    fn find_variable_matches_names_ignoring_case_and_the_flag_bit() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        let (this, _) = environment(&mut e, &["Alpha", "Beta", "Gamma"]);
        let beta = put_text(&mut e, b"BETA");
        let missing = put_text(&mut e, b"delta");
        assert_eq!(e.call(0x00e3_f230, &args![this, beta]).i32(), 1);
        assert_eq!(e.call(0x00e3_f230, &args![this, missing]).i32(), -1);
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(e.call(0x00e3_f230, &args![this, beta]).i32(), -1);
    }

    #[test]
    fn the_clear_destroys_value_then_name_of_each_variable() {
        let mut e = Engine::new();
        let strings = log_calls(&mut e, STRING_PTR_DESTRUCT, 0);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(16);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, 2);
        e.call(0x00e3_f370, &args![this]);
        assert_eq!(
            *strings.borrow(),
            vec![vec![data + 4], vec![data], vec![data + 12], vec![data + 8]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn set_variable_with_no_value_removes_it_and_moves_the_last_one_in() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        let strings = log_calls(&mut e, STRING_PTR_DESTRUCT, 0);
        let (this, data) = environment(&mut e, &["a", "b", "c"]);
        let name = put_text(&mut e, b"A");
        // Removing a: its two strings are destroyed, the count goes to 2 and the last
        // variable (c) moves into slot 0.
        assert_eq!(e.call(0x00e3_f3b0, &args![this, name, 0u32]).u32(), 0);
        assert_eq!(*strings.borrow(), vec![vec![data + 4], vec![data]]);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(data + 4), 0x7002);
        // Removing the (now) last one does not move anything.
        let last = put_text(&mut e, b"b");
        assert_eq!(e.call(0x00e3_f3b0, &args![this, last, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        // A name that is not there: 1.
        let missing = put_text(&mut e, b"zz");
        assert_eq!(e.call(0x00e3_f3b0, &args![this, missing, 0u32]).u32(), 1);
    }

    #[test]
    fn set_variable_assigns_an_existing_value() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        let assigned = log_calls(&mut e, STRING_PTR_ASSIGN_TEXT, 0);
        let (this, data) = environment(&mut e, &["a", "b"]);
        let name = put_text(&mut e, b"B");
        let value = put_text(&mut e, b"v");
        assert_eq!(e.call(0x00e3_f3b0, &args![this, name, value]).u32(), 0);
        assert_eq!(*assigned.borrow(), vec![vec![data + 8 + 4, value]]);
        assert_eq!(e.mem.u32(this + 0xc), 2);
    }

    #[test]
    fn set_variable_adds_a_new_variable_through_temporaries() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        install_memory(&mut e);
        let events = Rc::new(RefCell::new(Vec::new()));
        for (address, tag) in [
            (STRING_PTR_CONSTRUCT, "construct"),
            (STRING_PTR_ASSIGN_TEXT, "assign"),
            (STRING_PTR_COPY_CONSTRUCT, "copy"),
            (STRING_PTR_DESTRUCT, "destroy"),
            (ARRAY_RESERVE_MORE, "grow"),
        ] {
            let kept = events.clone();
            e.register_double(address, move |_, a| {
                kept.borrow_mut().push((tag, a.to_vec()));
                Ret::default()
            });
        }
        let (this, data) = environment(&mut e, &["a"]);
        // The capacity is 4: no growth for the second variable.
        let name = put_text(&mut e, b"new");
        let value = put_text(&mut e, b"val");
        assert_eq!(e.call(0x00e3_f3b0, &args![this, name, value]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let events_now = events.borrow();
        let tags: Vec<&str> = events_now.iter().map(|(tag, _)| *tag).collect();
        assert_eq!(
            tags,
            vec![
                "construct",
                "construct",
                "assign",
                "assign",
                "copy",
                "copy",
                "destroy",
                "destroy"
            ]
        );
        // The temporaries are consecutive words; the copies go into the new entry.
        let temp_name = events_now[0].1[0];
        let temp_value = events_now[1].1[0];
        assert_eq!(temp_value, temp_name + 4);
        assert_eq!(events_now[2].1, vec![temp_name, name]);
        assert_eq!(events_now[3].1, vec![temp_value, value]);
        assert_eq!(events_now[4].1, vec![data + 8, temp_name]);
        assert_eq!(events_now[5].1, vec![data + 12, temp_value]);
        assert_eq!(events_now[6].1, vec![temp_value]);
        assert_eq!(events_now[7].1, vec![temp_name]);
    }

    #[test]
    fn set_variable_grows_a_full_array_first() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        install_memory(&mut e);
        for address in [
            STRING_PTR_CONSTRUCT,
            STRING_PTR_ASSIGN_TEXT,
            STRING_PTR_COPY_CONSTRUCT,
            STRING_PTR_DESTRUCT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let grown = log_calls(&mut e, ARRAY_RESERVE_MORE, 0);
        let (this, _) = environment(&mut e, &["a"]);
        // Capacity 1 (masked): the array is full.
        e.mem.set_u32(this + 0x10, 0x4000_0001);
        let name = put_text(&mut e, b"new");
        let value = put_text(&mut e, b"val");
        e.call(0x00e3_f3b0, &args![this, name, value]);
        assert_eq!(*grown.borrow(), vec![vec![ALLOCATOR_OBJECT, this + 8, 8]]);
    }

    #[test]
    fn the_second_constructor_stores_its_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00e3_f510, &args![this, 0x55u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ENVIRONMENT_VTABLE);
    }

    #[test]
    fn the_destructor_destroys_the_strings_and_gives_the_array_back() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let strings = log_calls(&mut e, STRING_PTR_DESTRUCT, 0);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(16);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 0x4000_0005);
        e.call(0x00e3_f820, &args![this]);
        assert_eq!(
            *strings.borrow(),
            vec![vec![data + 4], vec![data], vec![data + 12], vec![data + 8]]
        );
        assert_eq!(*frees.borrow(), vec![vec![data, 40]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }
}
