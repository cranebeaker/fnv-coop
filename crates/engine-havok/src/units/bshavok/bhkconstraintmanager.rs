//! `BSHavok/bhkconstraintmanager.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constraint manager maps the constraint type numbers 0 to 13 (the type of an
//! `hkpConstraintData`, virtual `+0x20`) to the 0x14-byte Cinfo object that wraps data of that
//! type: the Cinfo keeps its `hkpConstraintData` at `+4`. The numbers 4, 5 and 11 have no Cinfo.
//!
//! The exception-unwinding frames (`FS:[0]`) of the functions marked `Translation incomplete`
//! are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), allocating through `00aa3e40` (thiscall, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `hkConstraintCinfo::SetData` (engine map name, `00c89ac0`, thiscall, one argument: the data).
const CINFO_SET_DATA: u32 = 0x00c8_9ac0;
/// Reference count increment of a Havok object (`00c90510`, thiscall; the engine map does not
/// name it).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// Per-thread word the functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;
/// Size of every Cinfo object.
const CINFO_SIZE: u32 = 0x14;
/// The constructor of the Cinfo object of a constraint type (thiscall, no stack arguments, each
/// answers the object): the compiler's switch of `GetInfo`; the types 4, 5 and 11 and the numbers
/// above 13 have none. The type 3 one is `00cf4820`.
fn cinfo_constructor(constraint_type: u32) -> Option<u32> {
    Some(match constraint_type {
        0 => 0x00cb_9c30,
        1 => 0x009b_a550,
        2 => 0x00c8_8430,
        3 => 0x00cf_4820,
        6 => 0x00cc_7590,
        7 => 0x00cb_9c50,
        8 => 0x00c7_fb20,
        9 => 0x00cc_51c0,
        10 => 0x00cc_7f10,
        12 => 0x00cc_4990,
        13 => 0x00ca_9a80,
        _ => return None,
    })
}
/// The vtable `00cf4820` stores.
const TYPE_3_CINFO_VTABLE: u32 = 0x010c_c2e0;
/// The address of the name of a constraint type: the compiler's switch of `GetTypeName`; the
/// other numbers (4, 5, 11 and above 13) get [`UNKNOWN_TYPE_NAME`] (`0106f148`).
fn type_name(constraint_type: u32) -> Option<u32> {
    Some(match constraint_type {
        0 => 0x010c_c2c0,
        1 => 0x010c_c2ac,
        2 => 0x010c_c290,
        3 => 0x010c_c278,
        6 => 0x010c_c260,
        7 => 0x010c_c24c,
        8 => 0x010c_c230,
        9 => 0x010c_c21c,
        10 => 0x010c_c208,
        12 => 0x010c_c1f0,
        13 => 0x010c_c1d8,
        _ => return None,
    })
}
const UNKNOWN_TYPE_NAME: u32 = 0x0106_f148;

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

// Translated from 00cf4790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraintManager::GetTypeName` (engine map name), `(constraint_type)`: the address of
/// the name string for the constraint types 0 to 13, `0106f148` for the types 4, 5 and 11 and
/// every number above 13.
pub fn bhk_constraint_manager_get_type_name(_e: &mut Engine, constraint_type: u32) -> u32 {
    type_name(constraint_type).unwrap_or(UNKNOWN_TYPE_NAME)
}

// Translated from 00cf4820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the Cinfo object for constraint type 3 (unconfirmed identity; the exe
/// shares this shape with the other Cinfo constructors): clears `+4`, sets `+8` to 1, clears
/// `+0xc` and `+0x10` and stores the vtable `010cc2e0`. Answers the object.
pub fn fn_00cf4820(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.mem.set_u32(object + 4, 0);
    e.mem.set_u32(object + 8, 1);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    e.mem.set_u32(object, TYPE_3_CINFO_VTABLE);
    this
}

// Translated from 00cf4840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf4840 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintManager::GetInfo` (engine map name), `(constraint_type)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, allocates a 0x14-byte block from the memory manager and
/// constructs the Cinfo object of that constraint type in it. Answers the object; 0 for the
/// types 4, 5 and 11, for numbers above 13, and when the allocation fails.
pub fn bhk_constraint_manager_get_info(e: &mut Engine, constraint_type: u32) -> Ptr {
    let scope = enter_scope(e);
    let constructor = cinfo_constructor(constraint_type);
    let mut info = Ptr::new(0);
    if let Some(constructor) = constructor {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, CINFO_SIZE])
            .u32();
        if block != 0 {
            info = e.call(constructor, &args![block]).ptr();
        }
    }
    leave_scope(e, scope);
    info
}

// Translated from 00cf4a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf4a30 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintManager::Clone` (engine map name), `(data, process)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, takes the Cinfo for the type of the constraint data (virtual
/// `+0x20`), binds it to the data (`00c89ac0`) and has it clone the data (virtual `+4`, with 0
/// and the process). Answers the clone (the Cinfo's `+4`) with a reference added, after
/// releasing the Cinfo (the scalar deleting destructor, virtual `+0`, flag 1). Answers 0 when
/// the type has no Cinfo.
pub fn bhk_constraint_manager_clone(e: &mut Engine, data: Ptr, process: Ptr) -> Ptr {
    let scope = enter_scope(e);
    let constraint_type = e.vcall(data.addr(), 0x20, &args![]).u32();
    let info = bhk_constraint_manager_get_info(e, constraint_type).addr();
    let mut clone = 0;
    if info != 0 {
        e.call(CINFO_SET_DATA, &args![info, data]);
        e.vcall(info, 4, &args![0u32, process]);
        clone = e.mem.u32(info + 4);
        e.call(ADD_REFERENCE, &args![clone]);
        e.vcall(info, 0, &args![1u32]);
    }
    leave_scope(e, scope);
    Ptr::new(clone)
}

// Translated from 00cf4af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf4af0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintManager::LoadBinary` (engine map name), `(stream)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, reads the constraint type (4 bytes through the stream's input
/// object, `stream + 0x24c`, function pointer at `+8` called as
/// `(input, buffer, size, &size, 1)`, advancing the object's position at `+4`), takes the
/// Cinfo for it, has it load (virtual `+8`, with the stream) and answers its data (`+4`) with
/// a reference added, after releasing the Cinfo as [`bhk_constraint_manager_clone`] does.
/// Answers 0 when the type has no Cinfo.
pub fn bhk_constraint_manager_load_binary(e: &mut Engine, stream: Ptr) -> Ptr {
    let scope = enter_scope(e);
    let input = e.mem.u32(stream.addr() + 0x24c);
    let reader = e.mem.u32(input + 8);
    // The stack holds the size cell (the stream parameter's slot, overwritten with 4) and the
    // type the read fills.
    let constraint_type = e.with_stack(8, |e, cells| {
        let size_slot = cells.addr();
        let type_slot = size_slot + 4;
        e.mem.set_u32(size_slot, 4);
        let read = e
            .call(reader, &args![input, type_slot, 4u32, size_slot, 1u32])
            .u32();
        let position = e.mem.u32(input + 4);
        e.mem.set_u32(input + 4, position.wrapping_add(read));
        e.mem.u32(type_slot)
    });
    let info = bhk_constraint_manager_get_info(e, constraint_type).addr();
    let mut data = 0;
    if info != 0 {
        e.vcall(info, 8, &args![stream]);
        data = e.mem.u32(info + 4);
        e.call(ADD_REFERENCE, &args![data]);
        e.vcall(info, 0, &args![1u32]);
    }
    leave_scope(e, scope);
    Ptr::new(data)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_4790, bhk_constraint_manager_get_type_name(u32) -> u32),
        entry!(0x00cf_4820, fn_00cf4820(Ptr) -> Ptr),
        entry!(0x00cf_4840, bhk_constraint_manager_get_info(u32) -> Ptr),
        entry!(0x00cf_4a30, bhk_constraint_manager_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00cf_4af0, bhk_constraint_manager_load_binary(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INFO_VTABLE: u32 = TYPE_3_CINFO_VTABLE;
    const DESTRUCT_SLOT: u32 = 0x7000_0000;
    const CLONE_SLOT: u32 = 0x7000_0004;
    const LOAD_SLOT: u32 = 0x7000_0008;
    const DATA_VTABLE: u32 = 0x7001_1000;
    const DATA_TYPE_SLOT: u32 = 0x7000_0020;
    const READER: u32 = 0x7000_0100;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// The memory manager double hands out zeroed blocks of the size asked.
    fn install_allocator(e: &mut Engine) {
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
    }

    /// The Cinfo vtable of type 3: destructor (`+0`), clone (`+4`) and load (`+8`) slots. The
    /// clone slot stores a data object at `+4` of the Cinfo; the load slot does the same.
    fn install_info_vtable(e: &mut Engine) {
        e.put_vtable(INFO_VTABLE, &[DESTRUCT_SLOT, CLONE_SLOT, LOAD_SLOT]);
        e.register(DESTRUCT_SLOT, |_, _| Ret::default());
        e.register(CLONE_SLOT, |e, a| {
            let data = e.mem.alloc(0x20);
            e.mem.set_u32(a[0] + 4, data);
            Ret::default()
        });
        e.register(LOAD_SLOT, |e, a| {
            let data = e.mem.alloc(0x20);
            e.mem.set_u32(a[0] + 4, data);
            Ret::default()
        });
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(CINFO_SET_DATA, |_, _| Ret::default());
    }

    #[test]
    fn bhk_constraint_manager_get_type_name_names_the_constraint_types() {
        let mut e = Engine::new();
        let expected = [
            (0u32, 0x010c_c2c0u32),
            (1, 0x010c_c2ac),
            (2, 0x010c_c290),
            (3, 0x010c_c278),
            (4, 0x0106_f148),
            (5, 0x0106_f148),
            (6, 0x010c_c260),
            (7, 0x010c_c24c),
            (8, 0x010c_c230),
            (9, 0x010c_c21c),
            (10, 0x010c_c208),
            (11, 0x0106_f148),
            (12, 0x010c_c1f0),
            (13, 0x010c_c1d8),
            (14, 0x0106_f148),
            (0xffff_ffff, 0x0106_f148),
        ];
        for (number, name) in expected {
            assert_eq!(e.call(0x00cf_4790, &args![number]).u32(), name);
        }
    }

    #[test]
    fn fn_00cf4820_initialises_the_type_3_cinfo() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x14);
        for offset in [4, 8, 0xc, 0x10] {
            e.mem.set_u32(object + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00cf_4820, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x010c_c2e0);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 1);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 0x10), 0);
    }

    #[test]
    fn bhk_constraint_manager_get_info_constructs_the_cinfo_of_the_type() {
        let mut e = logged_engine();
        install_allocator(&mut e);
        for constructor in (0..=13).filter_map(cinfo_constructor) {
            if constructor != 0x00cf_4820 {
                e.register(constructor, |_, a| a[0].into_ret());
            }
        }
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        for number in 0..=13u32 {
            let Some(constructor) = cinfo_constructor(number) else {
                continue;
            };
            clear_log(&mut e);
            let object = e.call(0x00cf_4840, &args![number]).u32();
            assert_ne!(object, 0);
            assert_eq!(
                calls_to(&e, MEMORY_ALLOCATE),
                vec![vec![MEMORY_MANAGER, 0x14]]
            );
            assert_eq!(calls_to(&e, constructor).len(), 1);
            assert_eq!(calls_to(&e, constructor)[0][0], object);
            assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        }
        // The type 3 one really runs.
        let object = e.call(0x00cf_4840, &args![3u32]).u32();
        assert_eq!(e.mem.u32(object), TYPE_3_CINFO_VTABLE);
        // No Cinfo for 4, 5, 11 and the numbers above 13: no allocation either.
        for number in [4u32, 5, 11, 14, 0xffff_ffff] {
            clear_log(&mut e);
            assert_eq!(e.call(0x00cf_4840, &args![number]).u32(), 0);
            assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
            assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        }
        // A failed allocation answers 0 without constructing.
        clear_log(&mut e);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00cf_4840, &args![0u32]).u32(), 0);
        assert!(calls_to(&e, 0x00cb_9c30).is_empty());
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    fn clear_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    #[test]
    fn bhk_constraint_manager_clone_clones_the_data_through_the_cinfo() {
        let mut e = logged_engine();
        install_allocator(&mut e);
        install_info_vtable(&mut e);
        e.put_vtable(DATA_VTABLE, &[0, 0, 0, 0, 0, 0, 0, 0, DATA_TYPE_SLOT]);
        e.register(DATA_TYPE_SLOT, |_, _| 3u32.into_ret());
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data, DATA_VTABLE);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let clone = e.call(0x00cf_4a30, &args![data, 0x9000u32]).u32();
        let info = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.0 == CLONE_SLOT)
            .unwrap()
            .1[0];
        assert_eq!(clone, e.mem.u32(info + 4));
        assert_ne!(clone, 0);
        assert_eq!(calls_to(&e, DATA_TYPE_SLOT), vec![vec![data]]);
        assert_eq!(calls_to(&e, CINFO_SET_DATA), vec![vec![info, data]]);
        assert_eq!(calls_to(&e, CLONE_SLOT), vec![vec![info, 0, 0x9000]]);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![clone]]);
        assert_eq!(calls_to(&e, DESTRUCT_SLOT), vec![vec![info, 1]]);
        let order = vec![
            DATA_TYPE_SLOT,
            MEMORY_ALLOCATE,
            CINFO_SET_DATA,
            CLONE_SLOT,
            ADD_REFERENCE,
            DESTRUCT_SLOT,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // A data type without Cinfo answers 0.
        clear_log(&mut e);
        e.register(DATA_TYPE_SLOT, |_, _| 5u32.into_ret());
        assert_eq!(e.call(0x00cf_4a30, &args![data, 0x9000u32]).u32(), 0);
        assert!(calls_to(&e, CINFO_SET_DATA).is_empty());
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_constraint_manager_load_binary_loads_the_data_of_the_read_type() {
        let mut e = logged_engine();
        install_allocator(&mut e);
        install_info_vtable(&mut e);
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, READER);
        e.mem.set_u32(input + 0x10, 3);
        e.register(READER, |e, a| {
            let value = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[1], value);
            let size_cell = e.mem.u32(a[3]);
            e.mem.set_u32(a[0] + 0x14, size_cell);
            a[2].into_ret()
        });
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let data = e.call(0x00cf_4af0, &args![stream]).u32();
        let info = calls_to(&e, LOAD_SLOT)[0][0];
        assert_eq!(calls_to(&e, LOAD_SLOT), vec![vec![info, stream]]);
        assert_eq!(data, e.mem.u32(info + 4));
        assert_ne!(data, 0);
        let reads = calls_to(&e, READER);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0][0], input);
        assert_eq!(reads[0][2], 4);
        assert_eq!(reads[0][4], 1);
        assert_eq!(e.mem.u32(input + 0x14), 4);
        assert_eq!(e.mem.u32(input + 4), 4);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![data]]);
        assert_eq!(calls_to(&e, DESTRUCT_SLOT), vec![vec![info, 1]]);
        let order = vec![
            READER,
            MEMORY_ALLOCATE,
            LOAD_SLOT,
            ADD_REFERENCE,
            DESTRUCT_SLOT,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // A type without Cinfo (the reader answers 5) loads nothing and answers 0.
        clear_log(&mut e);
        e.mem.set_u32(input + 0x10, 5);
        assert_eq!(e.call(0x00cf_4af0, &args![stream]).u32(), 0);
        assert!(calls_to(&e, LOAD_SLOT).is_empty());
        assert_eq!(e.mem.u32(input + 4), 8);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}
