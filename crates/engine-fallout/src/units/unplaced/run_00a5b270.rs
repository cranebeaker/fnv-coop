//! `(unplaced)/run_00a5b270` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The run holds the global string table behind `NiFixedString` handles.
//! The engine map names none of its functions; the class is identified by
//! its layout: the Xbox PDB's `NiGlobalStringTable` is 0x2100 bytes with
//! `m_kHashArray` (`NiTPrimitiveArray<char *>` buckets) at +0 and
//! `m_kCriticalSection` at +0x2080, and the code here builds 0x201 buckets
//! of 0x10 bytes whose vtable `01021024` has the RTTI name
//! `NiTPrimitiveArray<char *>` (the destructor stores `0102102c`,
//! `NiTArray<char *, NiTMallocInterface<char *>>`), allocates 0x2100 bytes
//! and keeps the critical section at +0x2080. The method names are not
//! confirmed, so the functions are `fn_<address>`.
//!
//! A string handle points at the characters; the word at `handle - 8` is
//! its reference count and the word at `handle - 4` its length.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

layout! {
    /// `NiGlobalStringTable` (Xbox PDB, 0x2100 bytes): `m_kHashArray`, 0x201
    /// `NiTPrimitiveArray<char *>` of 0x10 bytes at +0, and
    /// `m_kCriticalSection` at +0x2080 (see [`CRITICAL_SECTION`]).
    pub struct NiGlobalStringTable: 0x2100 {}
}

/// Number of hash buckets (the hash is taken modulo this).
const BUCKET_COUNT: u32 = 0x201;
/// Size of one bucket (`NiTPrimitiveArray<char *>`).
const BUCKET_SIZE: u32 = 0x10;
/// Offset of `m_kCriticalSection` (Xbox PDB) in the table.
const CRITICAL_SECTION: u32 = 0x2080;
/// Size the table is allocated with.
const TABLE_SIZE: u32 = 0x2100;
/// Grow-by count the constructor gives every bucket after setting up the
/// critical section.
const BUCKET_GROW_BY: u16 = 10;

/// The word holding the table (null before `00a5b630` and after
/// `00a5b660`).
pub const STRING_TABLE: u32 = 0x011f_42c0;
/// Counter incremented by every insertion.
const INSERTED_STRINGS: u32 = 0x011f_42c4;
/// Counter incremented by an insertion into a bucket that already had
/// entries.
const BUCKET_COLLISIONS: u32 = 0x011f_42c8;

/// Vtable `NiTPrimitiveArray<char *>` (RTTI) stored by the constructor.
const PRIMITIVE_ARRAY_VTABLE: u32 = 0x0102_1024;
/// Vtable `NiTArray<char *, NiTMallocInterface<char *>>` (RTTI) stored by
/// the destructor before it frees a bucket's storage.
const MALLOC_ARRAY_VTABLE: u32 = 0x0102_102c;

/// `MemoryManager` singleton and its `Allocate` / `Deallocate` (Xbox PDB).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// Gamebryo allocation through the allocator object (`00aa1070`, size) and
/// release (`00aa10f0`, block), both cdecl.
const GAMEBRYO_ALLOCATE: u32 = 0x00aa_1070;
const GAMEBRYO_FREE: u32 = 0x00aa_10f0;
/// `NiTArray<char *>` storage growth (`this`, new capacity) and element
/// store (`this`, index, address of the value).
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// CRT `memcpy` (cdecl).
const MEMCPY: u32 = 0x00ec_44d0;

/// Import slots (KERNEL32).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

/// The per-thread word at `TLS + 0x2b4` that insertions set to 8 for their
/// duration (the allocation context), restoring the old value after.
const TLS_CONTEXT: u32 = 0x2b4;
const STRING_CONTEXT: u32 = 8;

/// Bucket `index` of the table.
fn bucket(table: Ptr<NiGlobalStringTable>, index: u32) -> Ptr<NiTArray> {
    Ptr::new(table.addr().wrapping_add(index.wrapping_mul(BUCKET_SIZE)))
}

fn critical_section(table: Ptr<NiGlobalStringTable>) -> u32 {
    table.addr().wrapping_add(CRITICAL_SECTION)
}

// Translated from 00a5b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The table's string hash (stdcall, `RET 8`): with `h = 0`, the first
/// `length & !3` characters four at a time and then every character up to
/// the terminating zero are folded in as `h = h * 33 + c` (characters
/// sign-extended, 32-bit wrap-around); the result is `h % 0x201`.
pub fn fn_00a5b270(e: &mut Engine, string: Ptr, length: u32) -> u32 {
    let mut p = string.addr();
    let mut hash: u32 = 0;
    let whole = length & !3;
    if whole != 0 {
        for _ in 0..whole / 4 {
            for k in 0..4 {
                let c = e.mem.i8(p + k) as i32 as u32;
                hash = hash.wrapping_mul(33).wrapping_add(c);
            }
            p = p.wrapping_add(4);
        }
    }
    loop {
        let c = e.mem.i8(p);
        if c == 0 {
            break;
        }
        hash = hash.wrapping_mul(33).wrapping_add(c as i32 as u32);
        p = p.wrapping_add(1);
    }
    hash % BUCKET_COUNT
}

/// The character comparison the find inlines (`strcmp`'s loop): true when
/// the two zero-terminated strings are equal.
fn same_characters(e: &Engine, a: u32, b: u32) -> bool {
    let mut k = 0u32;
    loop {
        let x = e.mem.u8(a.wrapping_add(k));
        if x != e.mem.u8(b.wrapping_add(k)) {
            return false;
        }
        if x == 0 {
            return true;
        }
        k = k.wrapping_add(1);
    }
}

// Translated from 00a5b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a string in the table (`this` in ECX, `RET 8`): inside the
/// critical section, walks the first `m_usSize` slots of the string's
/// bucket and returns the entry that is the same pointer, or a non-null
/// entry whose stored length (`entry - 4`) is `length` and whose characters
/// equal the string's. Returns null when there is none.
pub fn fn_00a5b300(
    e: &mut Engine,
    this: Ptr<NiGlobalStringTable>,
    string: Ptr,
    length: u32,
) -> Ptr {
    let index = fn_00a5b270(e, string, length);
    let section = critical_section(this);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let slots = bucket(this, index);
    let size = e.get(slots, NiTArray::m_usSize) as u32;
    let mut found = 0u32;
    if size != 0 {
        let mut slot = e.get(slots, NiTArray::m_pBase);
        for _ in 0..size {
            let entry = e.mem.u32(slot);
            if entry == string.addr() {
                found = entry;
                break;
            }
            if entry != 0
                && e.mem.u32(entry.wrapping_sub(4)) == length
                && same_characters(e, entry, string.addr())
            {
                found = entry;
                break;
            }
            slot = slot.wrapping_add(4);
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    Ptr::new(found)
}

// Translated from 00a5b3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the unused strings (`this` in ECX): inside the critical
/// section, every non-null entry of every bucket whose reference count
/// (`entry - 8`) is 0 is freed (`00aa10f0` on `entry - 8`) and its slot
/// cleared as `NiTArray::SetAt(i, 0)` does (inlined: `m_usSize` becomes
/// `i + 1` when `i` is past it, otherwise `m_usESize` drops by one when the
/// slot still holds a value). Each bucket's slot count is read once before
/// its walk.
pub fn fn_00a5b3c0(e: &mut Engine, this: Ptr<NiGlobalStringTable>) {
    let section = critical_section(this);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    for index in 0..BUCKET_COUNT {
        let slots = bucket(this, index);
        let size = e.get(slots, NiTArray::m_usSize) as u32;
        for i in 0..size {
            let base = e.get(slots, NiTArray::m_pBase);
            let entry = e.mem.u32(base.wrapping_add(i.wrapping_mul(4)));
            if entry == 0 {
                continue;
            }
            let block = entry.wrapping_sub(8);
            if e.mem.u32(block) != 0 {
                continue;
            }
            e.call(GAMEBRYO_FREE, &args![block]);
            if i >= e.get(slots, NiTArray::m_usSize) as u32 {
                e.set(slots, NiTArray::m_usSize, i.wrapping_add(1) as u16);
            } else {
                let base = e.get(slots, NiTArray::m_pBase);
                if e.mem.u32(base.wrapping_add(i.wrapping_mul(4))) != 0 {
                    let entries = e.get(slots, NiTArray::m_usESize);
                    e.set(slots, NiTArray::m_usESize, entries.wrapping_add(0xffff));
                }
            }
            let base = e.get(slots, NiTArray::m_pBase);
            e.mem.set_u32(base.wrapping_add(i.wrapping_mul(4)), 0);
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00a5b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the unused strings of the table in `011f42c0` (`00a5b3c0` on
/// it, a tail jump).
pub fn fn_00a5b460(e: &mut Engine) {
    let table = Ptr::new(e.mem.u32(STRING_TABLE));
    fn_00a5b3c0(e, table);
}

// Translated from 00a5b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The table's destructor (`this` in ECX): deletes the critical section,
/// then, from the last bucket to the first, stores the
/// `NiTArray<char *, NiTMallocInterface<char *>>` vtable (`0102102c`) and
/// frees the bucket's storage (`00aa10f0` on `m_pBase`, read before the
/// vtable store). The strings themselves are not freed.
pub fn fn_00a5b470(e: &mut Engine, this: Ptr<NiGlobalStringTable>) {
    let section = critical_section(this);
    e.call(DELETE_CRITICAL_SECTION, &args![section]);
    for index in (0..BUCKET_COUNT).rev() {
        let slots = bucket(this, index);
        let base = e.get(slots, NiTArray::m_pBase);
        e.mem.set_u32(slots.addr(), MALLOC_ARRAY_VTABLE);
        e.call(GAMEBRYO_FREE, &args![base]);
    }
}

// Translated from 00a5b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an element to a `NiTArray<char *>` (`this` in ECX, `RET 4`; the
/// argument is the address of the value), the shape of
/// `NiTArray::AddFirstEmpty`: a null value returns -1 (`0xffffffff`);
/// otherwise the first empty slot below `m_usSize` takes it (`m_usESize`
/// grows by one) and its index is returned; with no empty slot, the
/// storage grows to `m_usGrowBy + m_usSize` (`0096ad30`) when `m_usSize`
/// has reached `m_usMaxSize`, the value is stored at index `m_usSize`
/// (`0096ae90`, index and address of the value) and that index is
/// returned. `m_usSize` is read once, at the start.
pub fn fn_00a5b4b0(e: &mut Engine, this: Ptr<NiTArray>, value: Ptr) -> u32 {
    let element = e.mem.u32(value.addr());
    if element == 0 {
        return 0xffff_ffff;
    }
    let size = e.get(this, NiTArray::m_usSize) as u32;
    let base = e.get(this, NiTArray::m_pBase);
    for index in 0..size {
        let slot = base.wrapping_add(index.wrapping_mul(4));
        if e.mem.u32(slot) == 0 {
            e.mem.set_u32(slot, element);
            let entries = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, entries.wrapping_add(1));
            return index;
        }
    }
    if size >= e.get(this, NiTArray::m_usMaxSize) as u32 {
        let grow_by = e.get(this, NiTArray::m_usGrowBy) as u32;
        e.call(ARRAY_SET_SIZE, &args![this, grow_by.wrapping_add(size)]);
    }
    e.call(ARRAY_SET_AT, &args![this, size, value]);
    size
}

// Translated from 00a5b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The table's constructor (`this` in ECX, returns `this`): every bucket
/// becomes an empty `NiTPrimitiveArray<char *>` (vtable `01021024`, null
/// storage, capacity, size and entry count 0, grow-by 1), the critical
/// section is initialized, and then every bucket's grow-by is set to 10.
pub fn fn_00a5b530(e: &mut Engine, this: Ptr<NiGlobalStringTable>) -> Ptr<NiGlobalStringTable> {
    for index in 0..BUCKET_COUNT {
        let slots = bucket(this, index);
        e.set(slots, NiTArray::m_usMaxSize, 0);
        e.set(slots, NiTArray::m_usGrowBy, 1);
        e.set(slots, NiTArray::m_usSize, 0);
        e.set(slots, NiTArray::m_usESize, 0);
        e.set(slots, NiTArray::m_pBase, 0);
        e.mem.set_u32(slots.addr(), PRIMITIVE_ARRAY_VTABLE);
    }
    let section = critical_section(this);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![section]);
    for index in 0..BUCKET_COUNT {
        e.set(bucket(this, index), NiTArray::m_usGrowBy, BUCKET_GROW_BY);
    }
    this
}

// Translated from 00a5b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts a string into the table (`this` in ECX, `RET 8`; arguments: the
/// address of the string handle, its length). With the thread's allocation
/// context (`TLS + 0x2b4`) set to 8: inside the critical section, counts a
/// collision (`011f42c8`) when the string's bucket already has entries,
/// adds the handle to the bucket (`00a5b4b0`) and counts the insertion
/// (`011f42c4`); then the context is restored.
pub fn fn_00a5b5a0(e: &mut Engine, this: Ptr<NiGlobalStringTable>, handle: Ptr, length: u32) {
    let context = e.tls().wrapping_add(TLS_CONTEXT);
    let saved = e.mem.u32(context);
    e.mem.set_u32(context, STRING_CONTEXT);
    let string = Ptr::new(e.mem.u32(handle.addr()));
    let index = fn_00a5b270(e, string, length);
    let section = critical_section(this);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let slots = bucket(this, index);
    if e.get(slots, NiTArray::m_usESize) != 0 {
        let collisions = e.mem.u32(BUCKET_COLLISIONS);
        e.mem.set_u32(BUCKET_COLLISIONS, collisions.wrapping_add(1));
    }
    fn_00a5b4b0(e, slots, handle);
    let inserted = e.mem.u32(INSERTED_STRINGS);
    e.mem.set_u32(INSERTED_STRINGS, inserted.wrapping_add(1));
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    e.mem.set_u32(context, saved);
}

// Translated from 00a5b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the table: allocates 0x2100 bytes (`MemoryManager::Allocate`)
/// and stores the constructed table (`00a5b530`) in `011f42c0`, or null
/// when the allocation failed.
pub fn fn_00a5b630(e: &mut Engine) {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, TABLE_SIZE])
        .u32();
    let table = if block != 0 {
        fn_00a5b530(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.mem.set_u32(STRING_TABLE, table);
}

// Translated from 00a5b660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the table: a non-null table in `011f42c0` is destructed
/// (`00a5b470`) and freed (`MemoryManager::Deallocate`); the word is then
/// cleared.
pub fn fn_00a5b660(e: &mut Engine) {
    let table = e.mem.u32(STRING_TABLE);
    if table != 0 {
        fn_00a5b470(e, Ptr::new(table));
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, table]);
    }
    e.mem.set_u32(STRING_TABLE, 0);
}

// Translated from 00a5b690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the table's handle for a string (cdecl), adding the string when
/// it is not there: null for a null string. Inside the table's critical
/// section (the table read from `011f42c0` at each use), a string found by
/// `00a5b300` gets its reference count (`handle - 8`) incremented
/// (`InterlockedIncrement`) and is returned. Otherwise, with the thread's
/// allocation context (`TLS + 0x2b4`) set to 8, a block of `length + 9`
/// bytes rounded up to a multiple of 4 is allocated (`00aa1070`), its
/// reference count set to 1 and its length to `length`, the characters and
/// the terminating zero copied to `block + 8` (`memcpy`), the new handle
/// inserted (`00a5b5a0`, with the address of the argument slot that now
/// holds the handle), and the handle returned after the context is
/// restored.
pub fn fn_00a5b690(e: &mut Engine, string: Ptr) -> Ptr {
    if string.is_null() {
        return Ptr::new(0);
    }
    let mut length = 0u32;
    while e.mem.u8(string.addr().wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    let section = e.mem.u32(STRING_TABLE).wrapping_add(CRITICAL_SECTION);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let table = Ptr::new(e.mem.u32(STRING_TABLE));
    let found = fn_00a5b300(e, table, string, length);
    if !found.is_null() {
        e.call(INTERLOCKED_INCREMENT, &args![found.addr().wrapping_sub(8)]);
        let section = e.mem.u32(STRING_TABLE).wrapping_add(CRITICAL_SECTION);
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return found;
    }
    let context = e.tls().wrapping_add(TLS_CONTEXT);
    let saved = e.mem.u32(context);
    let mut size = length.wrapping_add(9);
    e.mem.set_u32(context, STRING_CONTEXT);
    if size & 3 != 0 {
        size = size.wrapping_add(4 - (size & 3));
    }
    let block = e.call(GAMEBRYO_ALLOCATE, &args![size]).u32();
    let handle = block.wrapping_add(8);
    e.mem.set_u32(block, 1);
    e.mem.set_u32(block.wrapping_add(4), length);
    e.call(MEMCPY, &args![handle, string, length.wrapping_add(1)]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), handle);
        let table = Ptr::new(e.mem.u32(STRING_TABLE));
        fn_00a5b5a0(e, table, slot, length);
    });
    let section = e.mem.u32(STRING_TABLE).wrapping_add(CRITICAL_SECTION);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    e.mem.set_u32(context, saved);
    Ptr::new(handle)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5b270, fn_00a5b270(Ptr, u32) -> u32),
        entry!(
            0x00a5b300,
            fn_00a5b300(Ptr<NiGlobalStringTable>, Ptr, u32) -> Ptr
        ),
        entry!(0x00a5b3c0, fn_00a5b3c0(Ptr<NiGlobalStringTable>)),
        entry!(0x00a5b460, fn_00a5b460()),
        entry!(0x00a5b470, fn_00a5b470(Ptr<NiGlobalStringTable>)),
        entry!(0x00a5b4b0, fn_00a5b4b0(Ptr<NiTArray>, Ptr) -> u32),
        entry!(
            0x00a5b530,
            fn_00a5b530(Ptr<NiGlobalStringTable>) -> Ptr<NiGlobalStringTable>
        ),
        entry!(0x00a5b5a0, fn_00a5b5a0(Ptr<NiGlobalStringTable>, Ptr, u32)),
        entry!(0x00a5b630, fn_00a5b630()),
        entry!(0x00a5b660, fn_00a5b660()),
        entry!(0x00a5b690, fn_00a5b690(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An engine with the globals page mapped and the KERNEL32 imports
    /// answered by doubles that do nothing.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        for slot in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            INITIALIZE_CRITICAL_SECTION,
        ] {
            e.register(slot, nop);
        }
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn c_string(e: &mut Engine, text: &[u8]) -> u32 {
        let p = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(p, text);
        p
    }

    /// A pooled string: count, length, characters.
    fn handle(e: &mut Engine, text: &[u8], count: u32) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 9);
        e.mem.set_u32(block, count);
        e.mem.set_u32(block + 4, text.len() as u32);
        e.mem.write(block + 8, text);
        block + 8
    }

    fn reference_hash(text: &[u8]) -> u32 {
        let mut h: u32 = 0;
        for &c in text {
            h = h.wrapping_mul(33).wrapping_add(c as i8 as i32 as u32);
        }
        h % 0x201
    }

    /// A constructed table (through the translated constructor).
    fn table(e: &mut Engine) -> Ptr<NiGlobalStringTable> {
        let t = e.new_object::<NiGlobalStringTable>();
        fn_00a5b530(e, t)
    }

    /// Gives bucket `index` storage of `capacity` slots holding `entries`.
    fn fill_bucket(
        e: &mut Engine,
        t: Ptr<NiGlobalStringTable>,
        index: u32,
        capacity: u16,
        entries: &[u32],
    ) -> u32 {
        let slots = bucket(t, index);
        let base = e.mem.alloc(capacity as u32 * 4);
        for (i, v) in entries.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *v);
        }
        e.set(slots, NiTArray::m_pBase, base);
        e.set(slots, NiTArray::m_usMaxSize, capacity);
        e.set(slots, NiTArray::m_usSize, entries.len() as u16);
        let used = entries.iter().filter(|v| **v != 0).count() as u16;
        e.set(slots, NiTArray::m_usESize, used);
        base
    }

    #[test]
    fn hash_folds_the_characters_modulo_0x201() {
        let mut e = engine();
        for text in [&b""[..], b"a", b"ab", b"abcdefg", b"\x80\xffxyz", b"NiNode"] {
            let s = c_string(&mut e, text);
            let r = e.call(0x00a5_b270, &args![s, text.len() as u32]).u32();
            assert_eq!(r, reference_hash(text), "{text:?}");
        }
        assert_eq!(reference_hash(b"ab"), 221);
        // The four-character groups run on the length even past a zero.
        let s = c_string(&mut e, b"a\0bc");
        let r = e.call(0x00a5_b270, &args![s, 4u32]).u32();
        let expected = (((97u32 * 33) * 33 + 98) * 33 + 99) % 0x201;
        assert_eq!(r, expected);
    }

    #[test]
    fn find_matches_by_pointer_or_by_length_and_characters() {
        let mut e = engine();
        let t = table(&mut e);
        let wanted = c_string(&mut e, b"Bip01");
        let index = reference_hash(b"Bip01");
        let other_length = handle(&mut e, b"Bip0", 1);
        e.mem.set_u32(other_length - 4, 5);
        let same = handle(&mut e, b"Bip01", 1);
        fill_bucket(&mut e, t, index, 4, &[0, other_length, same]);
        let r = e.call(0x00a5_b300, &args![t, wanted, 5u32]).u32();
        assert_eq!(r, same);
        // Wrong length: not found.
        let r = e.call(0x00a5_b300, &args![t, wanted, 4u32]).u32();
        assert_eq!(r, 0);
        // The same pointer is found although its stored length (5) differs.
        fill_bucket(&mut e, t, reference_hash(b"Bip0"), 2, &[other_length]);
        let r = e.call(0x00a5_b300, &args![t, other_length, 4u32]).u32();
        assert_eq!(r, other_length);
    }

    #[test]
    fn garbage_collect_frees_unreferenced_strings() {
        let mut e = engine();
        e.register(GAMEBRYO_FREE, nop);
        let t = table(&mut e);
        let kept = handle(&mut e, b"kept", 2);
        let dead = handle(&mut e, b"dead", 0);
        let base = fill_bucket(&mut e, t, 7, 4, &[kept, 0, dead]);
        e.mem.set_u32(STRING_TABLE, t.addr());
        e.call_log = Some(vec![]);
        e.call(0x00a5_b460, &[]);
        assert_eq!(calls(&e, GAMEBRYO_FREE), vec![vec![dead - 8]]);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.mem.u32(base), kept);
        let slots = bucket(t, 7);
        assert_eq!(e.get(slots, NiTArray::m_usESize), 1);
        assert_eq!(e.get(slots, NiTArray::m_usSize), 3);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn destructor_frees_every_bucket_last_first() {
        let mut e = engine();
        e.register(GAMEBRYO_FREE, nop);
        let t = table(&mut e);
        let base = fill_bucket(&mut e, t, 0x200, 2, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00a5_b470, &args![t]);
        let frees = calls(&e, GAMEBRYO_FREE);
        assert_eq!(frees.len(), 0x201);
        assert_eq!(frees[0], vec![base]);
        assert_eq!(frees[0x200], vec![0]);
        assert_eq!(e.mem.u32(t.addr()), MALLOC_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(t.addr() + 0x2000), MALLOC_ARRAY_VTABLE);
        assert_eq!(
            calls(&e, DELETE_CRITICAL_SECTION),
            vec![vec![t.addr() + 0x2080]]
        );
    }

    #[test]
    fn add_first_empty_reuses_a_hole_or_appends() {
        let mut e = engine();
        e.register(ARRAY_SET_SIZE, nop);
        e.register(ARRAY_SET_AT, nop);
        let t = table(&mut e);
        let slots = bucket(t, 3);
        let value = e.mem.alloc(4);
        // Null value.
        let r = e.call(0x00a5_b4b0, &args![slots, value]).u32();
        assert_eq!(r, 0xffff_ffff);
        e.mem.set_u32(value, 0x1234);
        // A hole at index 1.
        let base = fill_bucket(&mut e, t, 3, 4, &[5, 0, 6]);
        let r = e.call(0x00a5_b4b0, &args![slots, value]).u32();
        assert_eq!(r, 1);
        assert_eq!(e.mem.u32(base + 4), 0x1234);
        assert_eq!(e.get(slots, NiTArray::m_usESize), 3);
        // Full and at capacity: grows by grow-by + size, then stores at size.
        fill_bucket(&mut e, t, 3, 2, &[5, 6]);
        e.call_log = Some(vec![]);
        let r = e.call(0x00a5_b4b0, &args![slots, value]).u32();
        assert_eq!(r, 2);
        assert_eq!(calls(&e, ARRAY_SET_SIZE), vec![vec![slots.addr(), 12]]);
        assert_eq!(calls(&e, ARRAY_SET_AT), vec![vec![slots.addr(), 2, value]]);
        // Room left: no growth.
        fill_bucket(&mut e, t, 3, 4, &[5, 6]);
        e.call_log = Some(vec![]);
        e.call(0x00a5_b4b0, &args![slots, value]);
        assert!(calls(&e, ARRAY_SET_SIZE).is_empty());
    }

    #[test]
    fn constructor_sets_up_every_bucket() {
        let mut e = engine();
        let t = e.new_object::<NiGlobalStringTable>();
        e.mem.set_u32(t.addr() + 4, 0xdead);
        e.call_log = Some(vec![]);
        let r = e.call(0x00a5_b530, &args![t]).u32();
        assert_eq!(r, t.addr());
        for index in [0, 0x100, 0x200] {
            let slots = bucket(t, index);
            assert_eq!(e.mem.u32(slots.addr()), PRIMITIVE_ARRAY_VTABLE);
            assert_eq!(e.get(slots, NiTArray::m_pBase), 0);
            assert_eq!(e.get(slots, NiTArray::m_usGrowBy), 10);
        }
        assert_eq!(
            calls(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![t.addr() + 0x2080]]
        );
    }

    #[test]
    fn insert_counts_collisions_and_restores_the_context() {
        let mut e = engine();
        e.register(ARRAY_SET_SIZE, nop);
        e.register(ARRAY_SET_AT, nop);
        let t = table(&mut e);
        let s = handle(&mut e, b"abc", 1);
        let index = reference_hash(b"abc");
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, s);
        let context = e.tls() + TLS_CONTEXT;
        e.mem.set_u32(context, 3);
        let base = fill_bucket(&mut e, t, index, 4, &[0x77, 0]);
        e.call(0x00a5_b5a0, &args![t, slot, 3u32]);
        assert_eq!(e.mem.u32(base + 4), s);
        assert_eq!(e.mem.u32(INSERTED_STRINGS), 1);
        assert_eq!(e.mem.u32(BUCKET_COLLISIONS), 1);
        assert_eq!(e.mem.u32(context), 3);
        // Into an empty bucket: no collision.
        let s2 = handle(&mut e, b"xyz", 1);
        e.mem.set_u32(slot, s2);
        fill_bucket(&mut e, t, reference_hash(b"xyz"), 4, &[]);
        e.call(0x00a5_b5a0, &args![t, slot, 3u32]);
        assert_eq!(e.mem.u32(INSERTED_STRINGS), 2);
        assert_eq!(e.mem.u32(BUCKET_COLLISIONS), 1);
    }

    #[test]
    fn create_and_destroy_keep_the_table_word() {
        let mut e = engine();
        e.register(GAMEBRYO_FREE, nop);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(MEMORY_MANAGER_DEALLOCATE, nop);
        e.call_log = Some(vec![]);
        e.call(0x00a5_b630, &[]);
        let t = e.mem.u32(STRING_TABLE);
        assert_ne!(t, 0);
        assert_eq!(e.mem.u32(t), PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x2100]]
        );
        e.call(0x00a5_b660, &[]);
        assert_eq!(e.mem.u32(STRING_TABLE), 0);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, t]]
        );
        // A failed allocation leaves null; destroying null frees nothing.
        e.register(MEMORY_MANAGER_ALLOCATE, nop);
        e.call(0x00a5_b630, &[]);
        assert_eq!(e.mem.u32(STRING_TABLE), 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_b660, &[]);
        assert!(calls(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
    }

    #[test]
    fn add_string_reuses_or_creates_the_handle() {
        let mut e = engine();
        let t = table(&mut e);
        e.mem.set_u32(STRING_TABLE, t.addr());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        let sizes: CallLog = Default::default();
        let log = sizes.clone();
        e.register_double(GAMEBRYO_ALLOCATE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: e.mem.alloc(a[0]),
                ..Ret::default()
            }
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(ARRAY_SET_SIZE, nop);
        e.register(ARRAY_SET_AT, |e, a| {
            // Stand-in that stores the value at the index of the storage.
            let slots = Ptr::<NiTArray>::new(a[0]);
            let base = e.get(slots, NiTArray::m_pBase);
            let base = if base == 0 {
                let b = e.mem.alloc(16);
                e.set(slots, NiTArray::m_pBase, b);
                b
            } else {
                base
            };
            let v = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], v);
            e.set(slots, NiTArray::m_usSize, a[1] as u16 + 1);
            e.set(slots, NiTArray::m_usMaxSize, 4);
            Ret::default()
        });
        assert_eq!(e.call(0x00a5_b690, &args![0u32]).u32(), 0);
        let text = c_string(&mut e, b"Scene Root");
        let first = e.call(0x00a5_b690, &args![text]).u32();
        assert_ne!(first, 0);
        assert_eq!(sizes.borrow()[0], vec![20]); // 10 + 9 rounded up to 20.
        assert_eq!(e.mem.bytes(first, 11), b"Scene Root\0".to_vec());
        assert_eq!(e.mem.u32(first - 8), 1);
        assert_eq!(e.mem.u32(first - 4), 10);
        assert_eq!(e.mem.u32(INSERTED_STRINGS), 1);
        // The second request finds it and takes a reference.
        let again = c_string(&mut e, b"Scene Root");
        let second = e.call(0x00a5_b690, &args![again]).u32();
        assert_eq!(second, first);
        assert_eq!(e.mem.u32(first - 8), 2);
        assert_eq!(sizes.borrow().len(), 1);
        // A length already a multiple of 4 after adding 9: "abc" -> 12.
        let abc = c_string(&mut e, b"abc");
        e.call(0x00a5_b690, &args![abc]);
        assert_eq!(sizes.borrow()[1], vec![12]);
    }
}
