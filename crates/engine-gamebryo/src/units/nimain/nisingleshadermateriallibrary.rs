//! `NiMain/nisingleshadermateriallibrary.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{
    acquire_fixed_string, interlocked_increment, release_fixed_string, EMPTY_FIXED_STRING,
    GET_FIXED_STRING,
};

/// `011f4400`: a counted word `00a8fe70` increments for the object it sets up.
const LIBRARY_COUNT: u32 = 0x011f_4400;
/// `011f5e44`: the global array of pointers `00a8fe70` appends to: the data pointer, then
/// (`+4`) the capacity at `011f5e48` and (`+8`) the count at `011f5e4c`.
const LIBRARY_LIST: u32 = 0x011f_5e44;
/// `00a311e0` (`thiscall`, `RET 4`, engine map `NiTSet<...>::Realloc`): gives the array
/// the capacity given, keeping its entries.
const LIST_REALLOC: u32 = 0x00a3_11e0;

// Translated from 00a8fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::NiSingleShaderMaterialLibrary` (Xbox PDB; `thiscall`, no
/// stack word): makes the fixed-string handle for the literal at `0109f7d0`, runs
/// `fn_00a8fe70` with `this` and the address of that handle, releases the handle, clears
/// the words at `+0xc` and `+0x10`, sets the vtable `0109f7b4` and returns `this`.
pub fn ni_single_shader_material_library_ni_single_shader_material_library(
    e: &mut Engine,
    this: Ptr,
) -> Ptr {
    let name = e.call(GET_FIXED_STRING, &args![0x0109_f7d0u32]).u32();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), name);
        fn_00a8fe70(e, this, local);
        let handle = e.mem.u32(local.addr());
        release_fixed_string(e, handle);
    });
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr(), 0x0109_f7b4);
    this
}

// Translated from 00a8fe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nisingleshadermateriallibrary.cpp`; `thiscall`, `RET 4`):
/// sets the vtable `0101dce4` and the word at `+4` to 0, counts `011f4400`, sets the vtable
/// `0109f7f4`, starts the handle at `+8` as the empty string's and replaces it (counted, the
/// old one released) by the handle `name` points to when that differs, then appends `this`
/// to the global array at `011f5e44`, first growing the array through `00a311e0` to twice
/// its capacity (1 when the capacity is 0) when it is full. Returns `this`.
pub fn fn_00a8fe70(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0101_dce4);
    e.mem.set_u32(base + 4, 0);
    interlocked_increment(e, LIBRARY_COUNT);
    e.mem.set_u32(base, 0x0109_f7f4);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(base + 8, empty);
    let handle = e.mem.u32(name.addr());
    if empty != handle {
        acquire_fixed_string(e, handle);
        let current = e.mem.u32(base + 8);
        release_fixed_string(e, current);
        e.mem.set_u32(base + 8, handle);
    }
    let capacity = e.mem.u32(LIBRARY_LIST + 4);
    if e.mem.u32(LIBRARY_LIST + 8) == capacity {
        let grown = if capacity == 0 {
            1
        } else {
            capacity.wrapping_add(capacity)
        };
        e.call(LIST_REALLOC, &args![LIBRARY_LIST, grown]);
    }
    let data = e.mem.u32(LIBRARY_LIST);
    let count = e.mem.u32(LIBRARY_LIST + 8);
    e.mem
        .set_u32(data.wrapping_add(count.wrapping_mul(4)), base);
    e.mem.set_u32(LIBRARY_LIST + 8, count.wrapping_add(1));
    this
}

/// `00a8fdb0` (`thiscall`, no stack word): the material-library base destructor.
const LIBRARY_BASE_DESTRUCTOR: u32 = 0x00a8_fdb0;
/// `00aa1460` (`cdecl`, two words: the block and its size): frees a sized block.
const FREE_SIZED: u32 = 0x00aa_1460;

// Translated from 00a8fd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::scalar deleting destructor` (Xbox PDB name in the engine
/// map; `thiscall`, `RET 4`): sets the vtable `0109f7b4`, clears the words at +0xc and
/// +0x10, runs the base destructor `00a8fdb0`, and for bit 0 of `flags` frees the 0x14 bytes
/// (`00aa1460`). Returns `this`.
pub fn ni_single_shader_material_library_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0109_f7b4);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.call(LIBRARY_BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00a8fcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nisingleshadermateriallibrary.cpp`, no arguments; one of
/// the `NiMain` shutdown steps of `00a8a280`): when `011f5e40` holds the library `00a8fd80`
/// constructed, calls its vtable slot 0 with 1 (`thiscall`); then clears the pointer.
pub fn fn_00a8fcc0(e: &mut Engine) {
    let object = e.mem.u32(LIBRARY_SLOT);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.mem.set_u32(LIBRARY_SLOT, 0);
}

/// `011f5e40`: where `00a8fd80` keeps the library it constructs.
const LIBRARY_SLOT: u32 = 0x011f_5e40;

/// `NiShaderFactory::GetInstance` (`00a9fff0`, no words), the factory's virtuals +0x38 (the
/// first shader library) and +0x3c (the next one), and the library's +0x14 (its
/// `NiShaderLibraryDesc`); `NiShaderLibraryDesc::GetShaderCount` (`00a9ffd0`) and
/// `GetShaderDesc` (`00a9ffe0`, one word: the index), `__thiscall`; the shader description's
/// virtual +0x8 (its name); `NiSingleShaderMaterial::Create` (`00aa08d0`, cdecl: name handle,
/// a flag).
const SHADER_FACTORY_GET_INSTANCE: u32 = 0x00a9_fff0;
const FACTORY_FIRST_LIBRARY_SLOT: u32 = 0x38;
const FACTORY_NEXT_LIBRARY_SLOT: u32 = 0x3c;
const LIBRARY_DESC_SLOT: u32 = 0x14;
const LIBRARY_DESC_GET_SHADER_COUNT: u32 = 0x00a9_ffd0;
const LIBRARY_DESC_GET_SHADER_DESC: u32 = 0x00a9_ffe0;
const SHADER_DESC_NAME_SLOT: u32 = 0x8;
const SINGLE_SHADER_MATERIAL_CREATE: u32 = 0x00aa_08d0;
/// The library's current shader library (+0xc) and the index into it (+0x10).
const CURRENT_LIBRARY: u32 = 0xc;
const CURRENT_INDEX: u32 = 0x10;

fn shader_count(e: &mut Engine, desc: u32) -> u32 {
    e.call(LIBRARY_DESC_GET_SHADER_COUNT, &args![desc]).u32()
}

fn library_desc(e: &mut Engine, library: u32) -> u32 {
    e.vcall(library, LIBRARY_DESC_SLOT, &args![]).u32()
}

// Translated from 00a8fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::GetFirstMaterialDesc` (Xbox PDB): the current library
/// (+0xc) becomes the factory's first one (index +0x10 cleared); none answers null. Libraries
/// without shaders are skipped (each next one's description read without a null test); the
/// answer is the first shader description of the library found, or null when it has no
/// description or no shaders.
pub fn ni_single_shader_material_library_get_first_material_desc(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let factory = e.call(SHADER_FACTORY_GET_INSTANCE, &args![]).u32();
    let library = e.vcall(factory, FACTORY_FIRST_LIBRARY_SLOT, &args![]).u32();
    e.mem.set_u32(base + CURRENT_LIBRARY, library);
    e.mem.set_u32(base + CURRENT_INDEX, 0);
    if library == 0 {
        return Ptr::NULL;
    }
    let mut desc = library_desc(e, library);
    while shader_count(e, desc) == 0 {
        let library = e.vcall(factory, FACTORY_NEXT_LIBRARY_SLOT, &args![]).u32();
        e.mem.set_u32(base + CURRENT_LIBRARY, library);
        e.mem.set_u32(base + CURRENT_INDEX, 0);
        desc = library_desc(e, library);
    }
    if desc == 0 || shader_count(e, desc) == 0 {
        return Ptr::NULL;
    }
    let index = e.mem.u32(base + CURRENT_INDEX);
    e.call(LIBRARY_DESC_GET_SHADER_DESC, &args![desc, index])
        .ptr()
}

// Translated from 00a8fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::GetNextMaterialDesc` (Xbox PDB): the factory's libraries
/// are walked up to the current one (+0xc) and the index (+0x10) goes up; when the current
/// library's description lacks that index (or is null), the next libraries are tried from
/// index 0 until one has shaders or none is left. The answer is the description at the index,
/// or null.
pub fn ni_single_shader_material_library_get_next_material_desc(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let factory = e.call(SHADER_FACTORY_GET_INSTANCE, &args![]).u32();
    let mut library = e.vcall(factory, FACTORY_FIRST_LIBRARY_SLOT, &args![]).u32();
    while library != e.mem.u32(base + CURRENT_LIBRARY) {
        library = e.vcall(factory, FACTORY_NEXT_LIBRARY_SLOT, &args![]).u32();
    }
    let index = e.mem.u32(base + CURRENT_INDEX).wrapping_add(1);
    e.mem.set_u32(base + CURRENT_INDEX, index);
    if library == 0 {
        return Ptr::NULL;
    }
    let mut desc;
    loop {
        desc = library_desc(e, library);
        if desc != 0 && shader_count(e, desc) > e.mem.u32(base + CURRENT_INDEX) {
            break;
        }
        library = e.vcall(factory, FACTORY_NEXT_LIBRARY_SLOT, &args![]).u32();
        e.mem.set_u32(base + CURRENT_LIBRARY, library);
        e.mem.set_u32(base + CURRENT_INDEX, 0);
        if library == 0 {
            break;
        }
    }
    if e.mem.u32(base + CURRENT_LIBRARY) == 0 || desc == 0 {
        return Ptr::NULL;
    }
    let index = e.mem.u32(base + CURRENT_INDEX);
    if shader_count(e, desc) <= index {
        return Ptr::NULL;
    }
    e.call(LIBRARY_DESC_GET_SHADER_DESC, &args![desc, index])
        .ptr()
}

// Translated from 00a8fbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::GetMaterialName` (Xbox PDB; `__thiscall`, `this` unread,
/// one word: the index over all libraries): walks the factory's libraries counting their
/// shaders; the library holding the index answers its description's name (the description's
/// virtual +0x8); null when the index is past them all.
pub fn ni_single_shader_material_library_get_material_name(
    e: &mut Engine,
    _unused_0: Ptr,
    index: u32,
) -> Ptr {
    let mut total = 0u32;
    let factory = e.call(SHADER_FACTORY_GET_INSTANCE, &args![]).u32();
    let mut library = e.vcall(factory, FACTORY_FIRST_LIBRARY_SLOT, &args![]).u32();
    while library != 0 {
        let desc = library_desc(e, library);
        if desc != 0 {
            if shader_count(e, desc).wrapping_add(total) > index {
                let shader = e
                    .call(
                        LIBRARY_DESC_GET_SHADER_DESC,
                        &args![desc, index.wrapping_sub(total)],
                    )
                    .u32();
                return e.vcall(shader, SHADER_DESC_NAME_SLOT, &args![]).ptr();
            }
            total = total.wrapping_add(shader_count(e, desc));
        }
        library = e.vcall(factory, FACTORY_NEXT_LIBRARY_SLOT, &args![]).u32();
    }
    Ptr::NULL
}

// Translated from 00a8fc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::GetMaterialCount` (Xbox PDB; `this` unread): the shaders of
/// all the factory's libraries (those with a description); 0 without a factory.
pub fn ni_single_shader_material_library_get_material_count(e: &mut Engine, _unused_0: Ptr) -> u32 {
    let factory = e.call(SHADER_FACTORY_GET_INSTANCE, &args![]).u32();
    if factory == 0 {
        return factory;
    }
    let mut total = 0u32;
    let mut library = e.vcall(factory, FACTORY_FIRST_LIBRARY_SLOT, &args![]).u32();
    while library != 0 {
        let desc = library_desc(e, library);
        if desc != 0 {
            total = total.wrapping_add(shader_count(e, desc));
        }
        library = e.vcall(factory, FACTORY_NEXT_LIBRARY_SLOT, &args![]).u32();
    }
    total
}

// Translated from 00a8fd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::GetMaterial` (Xbox PDB; `__thiscall`, `this` unread, one
/// word: the address of the name): `NiSingleShaderMaterial::Create` (`00aa08d0`) with the name
/// handle and 1.
pub fn ni_single_shader_material_library_get_material(
    e: &mut Engine,
    _unused_0: Ptr,
    name: Ptr,
) -> Ptr {
    let handle = e.mem.u32(name.addr());
    e.call(SINGLE_SHADER_MATERIAL_CREATE, &args![handle, 1u32])
        .ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_fce0, ni_single_shader_material_library_ni_single_shader_material_library(Ptr) -> Ptr),
        entry!(0x00a8_fe70, fn_00a8fe70(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a8_fd30,
            ni_single_shader_material_library_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a8_fcc0, fn_00a8fcc0()),
        entry!(
            0x00a8_fac0,
            ni_single_shader_material_library_get_first_material_desc(Ptr) -> Ptr
        ),
        entry!(
            0x00a8_fb50,
            ni_single_shader_material_library_get_next_material_desc(Ptr) -> Ptr
        ),
        entry!(
            0x00a8_fbf0,
            ni_single_shader_material_library_get_material_name(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a8_fc70,
            ni_single_shader_material_library_get_material_count(Ptr) -> u32
        ),
        entry!(
            0x00a8_fd60,
            ni_single_shader_material_library_get_material(Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    /// An engine whose array has room for `capacity` entries (`count` used); the realloc
    /// double doubles the data block and records the capacity asked for.
    fn engine(capacity: u32, count: u32) -> Engine {
        let mut e = engine_with_counting();
        let data = if capacity == 0 {
            0
        } else {
            e.mem.alloc(capacity * 4)
        };
        e.mem.set_u32(LIBRARY_LIST, data);
        e.mem.set_u32(LIBRARY_LIST + 4, capacity);
        e.mem.set_u32(LIBRARY_LIST + 8, count);
        e.register(LIST_REALLOC, |e, a| {
            assert_eq!(a[0], LIBRARY_LIST);
            let old = e.mem.u32(LIBRARY_LIST);
            let count = e.mem.u32(LIBRARY_LIST + 8);
            let new = e.mem.alloc(a[1] * 4);
            for index in 0..count {
                let entry = e.mem.u32(old + 4 * index);
                e.mem.set_u32(new + 4 * index, entry);
            }
            e.mem.set_u32(LIBRARY_LIST, new);
            e.mem.set_u32(LIBRARY_LIST + 4, a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn appends_the_library_to_the_list_with_room() {
        let mut e = engine(4, 1);
        e.mem.set_u32(LIBRARY_COUNT, 5);
        let this = e.mem.alloc(0x20);
        let name = e.mem.alloc(4);
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(name, handle);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_fe70, &args![this, name]).u32(), this);
        assert!(calls_to(&e, LIST_REALLOC).is_empty());
        assert_eq!(e.mem.u32(this), 0x0109_f7f4);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), handle);
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(e.mem.u32(LIBRARY_COUNT), 6);
        let data = e.mem.u32(LIBRARY_LIST);
        assert_eq!(e.mem.u32(data + 4), this);
        assert_eq!(e.mem.u32(LIBRARY_LIST + 8), 2);
    }

    #[test]
    fn grows_a_full_list_to_twice_its_capacity_or_one() {
        for (capacity, grown) in [(0u32, 1u32), (2, 4)] {
            let mut e = engine(capacity, capacity);
            let this = e.mem.alloc(0x20);
            let name = e.mem.alloc(4);
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            e.mem.set_u32(name, empty);
            e.call_log = Some(vec![]);
            e.call(0x00a8_fe70, &args![this, name]);
            assert_eq!(calls_to(&e, LIST_REALLOC), vec![vec![LIBRARY_LIST, grown]]);
            // The empty handle is kept as it is.
            assert_eq!(e.mem.u32(this + 8), empty);
            let data = e.mem.u32(LIBRARY_LIST);
            assert_eq!(e.mem.u32(data + 4 * capacity), this);
            assert_eq!(e.mem.u32(LIBRARY_LIST + 8), capacity + 1);
        }
    }

    #[test]
    fn constructor_sets_up_with_the_string_handle_and_releases_it() {
        let mut e = engine(4, 0);
        let this = e.mem.alloc(0x20);
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(handle - 8, 1);
        e.register_double(GET_FIXED_STRING, move |_, a| {
            assert_eq!(a, &[0x0109_f7d0]);
            handle.into_ret()
        });
        e.mem.set_u32(this + 0xc, 0x77);
        assert_eq!(e.call(0x00a8_fce0, &args![this]).u32(), this);
        // Counted once by the library at +8, once released by the constructor.
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(e.mem.u32(this + 8), handle);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this), 0x0109_f7b4);
        let data = e.mem.u32(LIBRARY_LIST);
        assert_eq!(e.mem.u32(data), this);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn library_destructor_clears_runs_the_base_and_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(LIBRARY_BASE_DESTRUCTOR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0x0109_f7b4);
            Ret::default()
        });
        e.register(FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        for flags in [1u32, 2] {
            e.mem.set_u32(this + 0xc, 5);
            e.mem.set_u32(this + 0x10, 6);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a8_fd30, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(LIBRARY_BASE_DESTRUCTOR, vec![this])];
            if flags & 1 != 0 {
                expected.push((FREE_SIZED, vec![this, 0x14]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(this + 0xc), 0);
            assert_eq!(e.mem.u32(this + 0x10), 0);
        }
    }

    #[test]
    fn fn_00a8fcc0_destroys_the_library_and_clears_the_pointer() {
        let mut e = Engine::new();
        e.map(LIBRARY_SLOT & !0xfff, 0x1000);
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        e.mem.set_u32(LIBRARY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a8_fcc0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0150_1000, vec![object, 1])]);
        assert_eq!(e.mem.u32(LIBRARY_SLOT), 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_fcc0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    use std::cell::Cell;
    use std::rc::Rc;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    const FIRST_DOUBLE: u32 = 0x0070_f000;
    const NEXT_DOUBLE: u32 = 0x0070_f010;
    const DESC_DOUBLE: u32 = 0x0070_f020;
    const NAME_DOUBLE: u32 = 0x0070_f030;

    /// A factory whose libraries hold `counts` shaders each (`None`: no description); each
    /// shader description is a word whose name double answers it plus 0x100.
    fn factory(e: &mut Engine, counts: &[Option<u16>]) -> (u32, Vec<u32>) {
        let factory = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(factory, vtable);
        e.mem
            .set_u32(vtable + FACTORY_FIRST_LIBRARY_SLOT, FIRST_DOUBLE);
        e.mem
            .set_u32(vtable + FACTORY_NEXT_LIBRARY_SLOT, NEXT_DOUBLE);
        let library_table = e.mem.alloc(0x20);
        e.mem
            .set_u32(library_table + LIBRARY_DESC_SLOT, DESC_DOUBLE);
        let shader_table = e.mem.alloc(0x10);
        e.mem
            .set_u32(shader_table + SHADER_DESC_NAME_SLOT, NAME_DOUBLE);
        let mut libraries = vec![];
        for count in counts {
            let library = e.mem.alloc(8);
            e.mem.set_u32(library, library_table);
            if let Some(count) = count {
                let desc = e.mem.alloc(0x48);
                let array = e.mem.alloc(4 * u32::from(*count) + 4);
                for i in 0..u32::from(*count) {
                    let shader = e.mem.alloc(4);
                    e.mem.set_u32(shader, shader_table);
                    e.mem.set_u32(array + 4 * i, shader);
                }
                e.mem.set_u32(desc + 0x3c, array);
                e.mem.set_u16(desc + 0x44, *count);
                e.mem.set_u32(library + 4, desc);
            }
            libraries.push(library);
        }
        e.register_double(SHADER_FACTORY_GET_INSTANCE, move |_, _| word(factory));
        let at = Rc::new(Cell::new(0usize));
        let (first_at, next_at) = (at.clone(), at);
        let first = libraries.clone();
        e.register_double(FIRST_DOUBLE, move |_, _| {
            first_at.set(0);
            word(first.first().copied().unwrap_or(0))
        });
        let next = libraries.clone();
        e.register_double(NEXT_DOUBLE, move |_, _| {
            next_at.set(next_at.get() + 1);
            word(next.get(next_at.get()).copied().unwrap_or(0))
        });
        e.register(DESC_DOUBLE, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(NAME_DOUBLE, |_, a| word(a[0] + 0x100));
        (factory, libraries)
    }

    fn shader(e: &Engine, library: u32, index: u32) -> u32 {
        let desc = e.mem.u32(library + 4);
        e.mem.u32(e.mem.u32(desc + 0x3c) + 4 * index)
    }

    #[test]
    fn first_and_next_walk_the_libraries() {
        let mut e = Engine::new();
        let (_, libraries) = factory(&mut e, &[Some(0), Some(2), None, Some(1)]);
        let this = e.mem.alloc(0x14);
        let first = e.call(0x00a8_fac0, &args![this]).u32();
        assert_eq!(first, shader(&e, libraries[1], 0));
        assert_eq!(e.mem.u32(this + CURRENT_LIBRARY), libraries[1]);
        let second = e.call(0x00a8_fb50, &args![this]).u32();
        assert_eq!(second, shader(&e, libraries[1], 1));
        // Past library 1: the one without a description is skipped.
        let third = e.call(0x00a8_fb50, &args![this]).u32();
        assert_eq!(third, shader(&e, libraries[3], 0));
        assert_eq!(e.mem.u32(this + CURRENT_INDEX), 0);
        assert_eq!(e.call(0x00a8_fb50, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + CURRENT_LIBRARY), 0);
    }

    #[test]
    fn first_without_libraries_is_null() {
        let mut e = Engine::new();
        factory(&mut e, &[]);
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00a8_fac0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + CURRENT_LIBRARY), 0);
    }

    #[test]
    fn names_and_counts_over_all_libraries() {
        let mut e = Engine::new();
        let (_, libraries) = factory(&mut e, &[Some(2), None, Some(3)]);
        assert_eq!(e.call(0x00a8_fc70, &args![0u32]).u32(), 5);
        let name = e.call(0x00a8_fbf0, &args![0u32, 3u32]).u32();
        assert_eq!(name, shader(&e, libraries[2], 1) + 0x100);
        let name = e.call(0x00a8_fbf0, &args![0u32, 1u32]).u32();
        assert_eq!(name, shader(&e, libraries[0], 1) + 0x100);
        assert_eq!(e.call(0x00a8_fbf0, &args![0u32, 5u32]).u32(), 0);
        e.register(SHADER_FACTORY_GET_INSTANCE, |_, _| word(0));
        assert_eq!(e.call(0x00a8_fc70, &args![0u32]).u32(), 0);
    }

    #[test]
    fn get_material_creates_from_the_name() {
        let mut e = Engine::new();
        e.register(SINGLE_SHADER_MATERIAL_CREATE, |_, a| word(a[0] + a[1]));
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x4000);
        assert_eq!(e.call(0x00a8_fd60, &args![0u32, name]).u32(), 0x4001);
    }
}
