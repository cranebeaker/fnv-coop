//! `BSShader/run_00b7fa40` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The shader package: a 0x1c-byte object the shader manager's set-up (`00b57420`) keeps at
//! `011f9498`. Unnamed in the engine map; its fields as the code uses them: the package's
//! file name (+0, allocated), the package's contents (+4, allocated), and at +8 a string map
//! of shaders (the code passes +8 to `NiTStringTemplateMap<NiTPointerMap<char const*,
//! NiShader*>,NiShader*>::SetAt`, `00a642a0`): vtable (+8), bucket count (+0xc), bucket
//! array (+0x10), item count (+0x14) and a byte at +0x18.

#[allow(unused_imports)]
use crate::prelude::*;

/// The shader model setting the package index is chosen from (`011f91c0`), and the three
/// bytes that refine it (`011f94a4`, `011f941e`, `011f941f`).
const SHADER_SETTING_011F91C0: u32 = 0x011f_91c0;
const BYTE_SETTING_011F94A4: u32 = 0x011f_94a4;
const BYTE_SETTING_011F941E: u32 = 0x011f_941e;
const BYTE_SETTING_011F941F: u32 = 0x011f_941f;

// Translated from 00b7faf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`). No arguments: the shader package index for
/// the setting at `011f91c0` (the switch's jump table at `00b7fbc4`, read from the exe's code
/// in Ghidra, gives the arms): 1, 2, 3, 4 give 1, 2, 5, 8; 5 gives 3 or 4 (byte `011f94a4`
/// clear or set) when `011f941e` is clear, else 0xa + (`011f941f` != 0) or 0xc + (it != 0);
/// 6 gives 6 or 7 when `011f941e` is clear, else 0xe + .. or 0x10 + ..; 7 gives 9 when
/// `011f941e` is clear, else 0x12 + (`011f941f` != 0); anything else gives 0. The whole
/// `EAX` is set on every path.
pub fn fn_00b7faf0(e: &mut Engine) -> u32 {
    let refine = |e: &Engine, plain: u32, base: u32| -> u32 {
        if e.global::<u8>(BYTE_SETTING_011F941E) == 0 {
            plain
        } else {
            base + u32::from(e.global::<u8>(BYTE_SETTING_011F941F) != 0)
        }
    };
    match e.global::<u32>(SHADER_SETTING_011F91C0).wrapping_sub(1) {
        0 => 1,
        1 => 2,
        2 => 5,
        3 => 8,
        4 => {
            if e.global::<u8>(BYTE_SETTING_011F94A4) == 0 {
                refine(e, 3, 0xa)
            } else {
                refine(e, 4, 0xc)
            }
        }
        5 => {
            if e.global::<u8>(BYTE_SETTING_011F94A4) == 0 {
                refine(e, 6, 0xe)
            } else {
                refine(e, 7, 0x10)
            }
        }
        6 => refine(e, 9, 0x12),
        _ => 0,
    }
}

/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`,
/// size) and deallocate (`00aa4060`, `thiscall`, block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The allocation `00aa1070` (`cdecl`, size).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
/// CRT `_memset` (`00ec61c0`, `cdecl`) and `_strcpy_s` (`00ec65a6`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
const STRCPY_S: u32 = 0x00ec_65a6;
/// The formatter `00406d00` (`cdecl`: buffer, size, format, then the format's words).
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// `"Data\\Shaders\\shaderpackage%03i.sdp"` (`010af68c`).
const PACKAGE_NAME_FORMAT: u32 = 0x010a_f68c;
/// The map's vtables: the one set first (`010ade18`) and the final one (`010af6cc`); neither
/// has an RTTI locator before it.
const MAP_VTABLE_010ADE18: u32 = 0x010a_de18;
const MAP_VTABLE_010AF6CC: u32 = 0x010a_f6cc;
/// The map's bucket count (0x25) and the size `00b7ff40` allocates its buckets with (0x94).
const MAP_BUCKETS: u32 = 0x25;
const MAP_BUCKET_BYTES: u32 = 0x94;

// Translated from 00b7ff40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`): the shader package's constructor. `thiscall`,
/// `RET 4`, one word (the package index). Sets up the map at +8 (vtable `010ade18`, 0x25
/// buckets at +0xc, item count +0x14 cleared, buckets from `00aa1070(0x94)` at +0x10 cleared
/// with `_memset` for `4 * buckets` bytes, the byte at +0x18 set to 1, then the vtable
/// `010af6cc`), clears +4 and +0, and when the index is above 0 (signed) formats
/// `"Data\\Shaders\\shaderpackage%03i.sdp"` with it into a 100-byte stack buffer
/// (`00406d00`), allocates its length + 1 from the memory manager into +0 and copies it there
/// (`_strcpy_s`). Answers `this`.
// Translation incomplete: 00b7ff40 - the C++ exception frame (FS:[0] chain, unwind state 0 after the map) and the /GS cookie check (00ec408c) are not translated.
pub fn fn_00b7ff40(e: &mut Engine, this: Ptr, index: i32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(8), MAP_VTABLE_010ADE18);
    e.mem.set_u32(base.wrapping_add(0xc), MAP_BUCKETS);
    e.mem.set_u32(base.wrapping_add(0x14), 0);
    let buckets = e.call(ALLOCATE_00AA1070, &args![MAP_BUCKET_BYTES]).u32();
    let count = e.mem.u32(base.wrapping_add(0xc));
    e.mem.set_u32(base.wrapping_add(0x10), buckets);
    e.call(MEMSET, &args![buckets, 0u32, count.wrapping_mul(4)]);
    e.mem.set_u8(base.wrapping_add(0x18), 1);
    e.mem.set_u32(base.wrapping_add(8), MAP_VTABLE_010AF6CC);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.mem.set_u32(base, 0);
    if index > 0 {
        e.with_stack(100, |e, buffer| {
            e.call(
                FORMAT_00406D00,
                &args![buffer, 100u32, PACKAGE_NAME_FORMAT, index],
            );
            // The inlined `strlen`, plus one for the NUL.
            let size = (e.mem.cstr(buffer.addr()).len() as u32).wrapping_add(1);
            let name = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
            e.mem.set_u32(base, name);
            e.call(STRCPY_S, &args![name, size, buffer]);
        });
    }
    this
}

/// `FileFinder::GetFile` (engine map, `00afdf00`, `cdecl`: name, two words).
const FILE_FINDER_GET_FILE: u32 = 0x00af_df00;
/// The file's read of a whole block (`00462d80`, `thiscall`, `RET 8`: buffer, size).
const FILE_READ_00462D80: u32 = 0x0046_2d80;
/// `NiTStringTemplateMap<NiTPointerMap<char const*,NiShader*>,NiShader*>::SetAt` (engine
/// map, `00a642a0`, `thiscall`, `RET 8`: key, value).
const SHADER_MAP_SET_AT: u32 = 0x00a6_42a0;
/// The package header's version (`100`) and the header's size.
const PACKAGE_VERSION: u32 = 100;
const PACKAGE_HEADER_BYTES: u32 = 0xc;

// Translated from 00b7fe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`): loads the shader package. `thiscall`, `RET 4`,
/// one word (the file name). Answers whether every shader was read (`AL`); 0 when the name
/// is null or `FileFinder::GetFile(name, 0, 0)` (`00afdf00`) finds nothing.
///
/// With a file: vtable slot +0x20 with (0, 0); the 12-byte header (version, shader count,
/// contents size; zeroed on the stack first) read through the function pointer at the file's
/// +8 (`cdecl`: file, header, 0xc, the address of the argument's own stack slot, which the
/// code sets to 1 just before, and 1), the result added to the file's +4. When 12 bytes came
/// and the version is 100: the contents size is allocated from the memory manager into +4 and
/// read (`00462d80`); if that read answers the size, the answer becomes 1 and each shader
/// record in the contents is put into the map at +8 (`00a642a0`, the record as key and value;
/// the next record starts `0x104 + ` the record's word at +0x100 bytes later), else the
/// contents are freed and +4 cleared. The file is then deleted (vtable slot 0 with 1).
pub fn fn_00b7fe40(e: &mut Engine, this: Ptr, name: Ptr) -> bool {
    let base = this.addr();
    let mut loaded = false;
    if name.is_null() {
        return loaded;
    }
    let file = e.call(FILE_FINDER_GET_FILE, &args![name, 0u32, 0u32]).u32();
    if file == 0 {
        return loaded;
    }
    e.vcall(file, 0x20, &args![0u32, 0u32]);
    e.with_stack(PACKAGE_HEADER_BYTES + 4, |e, header| {
        let header = header.addr();
        let argument_slot = header.wrapping_add(PACKAGE_HEADER_BYTES);
        e.mem.set_u32(argument_slot, 1);
        let read = e.mem.u32(file.wrapping_add(8));
        let got = e
            .call(
                read,
                &args![file, header, PACKAGE_HEADER_BYTES, argument_slot, 1u32],
            )
            .u32();
        let position = e.mem.u32(file.wrapping_add(4)).wrapping_add(got);
        e.mem.set_u32(file.wrapping_add(4), position);
        if got == PACKAGE_HEADER_BYTES && e.mem.u32(header) == PACKAGE_VERSION {
            let size = e.mem.u32(header.wrapping_add(8));
            let contents = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
            e.mem.set_u32(base.wrapping_add(4), contents);
            let read = e
                .call(FILE_READ_00462D80, &args![file, contents, size])
                .u32();
            if read == size {
                let mut record = e.mem.u32(base.wrapping_add(4));
                loaded = true;
                let shaders = e.mem.u32(header.wrapping_add(4));
                if shaders != 0 {
                    let mut index = 0u32;
                    loop {
                        e.call(
                            SHADER_MAP_SET_AT,
                            &args![base.wrapping_add(8), record, record],
                        );
                        let length = e.mem.u32(record.wrapping_add(0x100));
                        index = index.wrapping_add(1);
                        record = record.wrapping_add(length).wrapping_add(0x104);
                        if index >= e.mem.u32(header.wrapping_add(4)) {
                            break;
                        }
                    }
                }
            } else {
                let contents = e.mem.u32(base.wrapping_add(4));
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, contents]);
                e.mem.set_u32(base.wrapping_add(4), 0);
            }
        }
    });
    e.vcall(file, 0, &args![1u32]);
    loaded
}

/// The map's clean-up `00b7fc50` (`thiscall` on +8, no stack words), jumped to at the end
/// of `00b7fd50`.
const MAP_CLEAN_UP_00B7FC50: u32 = 0x00b7_fc50;

// Translated from 00b7fd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`): the shader package's clean-up. `thiscall`, no
/// stack words: frees the file name (+0) and the contents (+4) through the memory manager
/// when not null, clearing each, then jumps to `00b7fc50` on the map at +8; answers what it
/// answers (`EAX`).
pub fn fn_00b7fd50(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    for offset in [0u32, 4] {
        let block = e.mem.u32(base.wrapping_add(offset));
        if block != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
            e.mem.set_u32(base.wrapping_add(offset), 0);
        }
    }
    e.call(MAP_CLEAN_UP_00B7FC50, &args![base.wrapping_add(8)])
        .u32()
}

/// The free `00aa10f0` (`cdecl`, block).
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// The map's base clean-up `00b7fbe0` (`thiscall`, no stack words).
const MAP_BASE_CLEAN_UP_00B7FBE0: u32 = 0x00b7_fbe0;

// Translated from 00b7fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`): the clean-up of the shader package's string
/// map (`this` is the package's +8). `thiscall`, no stack words. Sets the map's vtable
/// `010af6cc`; when the byte at +0x10 (the package's +0x18) is set, walks every bucket of the
/// array at +8 (count at +4, read again after each bucket) and, for each entry of a
/// bucket's chain (next entry at +0), frees the entry's word at +4 (`00aa10f0`). Then runs
/// the base clean-up `00b7fbe0` and answers what it answers (`EAX`).
// Translation incomplete: 00b7fc50 - the C++ exception frame (FS:[0] chain, unwind state 0 around the walk) is not translated.
pub fn fn_00b7fc50(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base, MAP_VTABLE_010AF6CC);
    if e.mem.u8(base.wrapping_add(0x10)) != 0 && e.mem.u32(base.wrapping_add(4)) != 0 {
        let mut index = 0u32;
        loop {
            let buckets = e.mem.u32(base.wrapping_add(8));
            let mut entry = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
            while entry != 0 {
                let key = e.mem.u32(entry.wrapping_add(4));
                entry = e.mem.u32(entry);
                e.call(FREE_00AA10F0, &args![key]);
            }
            index = index.wrapping_add(1);
            if index >= e.mem.u32(base.wrapping_add(4)) {
                break;
            }
        }
    }
    e.call(MAP_BASE_CLEAN_UP_00B7FBE0, &args![base]).u32()
}

/// The map's derived vtable stored first by the base clean-up (`010af6b0`; no RTTI locator
/// before it).
const MAP_VTABLE_010AF6B0: u32 = 0x010a_f6b0;
/// The hash map's `RemoveAll` (`00b601a0`, unnamed; `thiscall`, no stack words).
const MAP_REMOVE_ALL_00B601A0: u32 = 0x00b6_01a0;

// Translated from 00b7fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`): the base clean-up of the shader package's
/// string map, `thiscall`, no stack words. Sets the vtable `010af6b0` and empties the map
/// (`00b601a0`), sets the vtable `010ade18` and empties it again, then frees the bucket array
/// at +8 (`00aa10f0`) and answers what the free answers (`EAX`).
// Translation incomplete: 00b7fbe0 - the C++ exception frame (FS:[0] chain, unwind state 0 between the two vtables) is not translated.
pub fn fn_00b7fbe0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base, MAP_VTABLE_010AF6B0);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![base]);
    e.mem.set_u32(base, MAP_VTABLE_010ADE18);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]).u32()
}

use crate::units::bsshader::geometrydecalshader::shader_inline::pass_render_state;

// Translated from 00b80040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`): a pass's blend setter, `thiscall` with three
/// stack words (`RET 0xc`), the third not read. Sets the render states 0x13 and 0x14 (the
/// source and destination blend in Direct3D 9's numbering) to the two words: for each, the
/// pass's render-state block (+0x30) is made first (`00e7f5d0`) when it is null, then
/// `00e7f430` sets the state on it with the value and 1.
pub fn fn_00b80040(
    e: &mut Engine,
    this: Ptr,
    source_blend: u32,
    destination_blend: u32,
    _unused_3: u32,
) {
    pass_render_state(e, this.addr(), 0x13, source_blend);
    pass_render_state(e, this.addr(), 0x14, destination_blend);
}

/// The hash map `GetAt` on the package's shader map (`00a53c30`, unnamed; `thiscall` on the map:
/// key, address of the result word; `RET 8`).
const SHADER_MAP_GET_AT_00A53C30: u32 = 0x00a5_3c30;

// Translated from 00b7fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`): the shader package's lookup, `thiscall` with one
/// stack word (the compiled shader's name). Looks the name up in the package's string map at
/// +8 (`00a53c30`, with a stack word cleared first for the result) and answers what it left
/// there (0 when not found).
pub fn fn_00b7fac0(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let map = this.addr().wrapping_add(8);
    e.with_stack(4, |e, result| {
        let result = result.addr();
        e.mem.set_u32(result, 0);
        e.call(SHADER_MAP_GET_AT_00A53C30, &args![map, name, result]);
        e.mem.u32(result)
    })
}

/// `NiStricmp` (`00aa1360`, `cdecl`, two words): case-insensitive compare, 0 when equal.
const NI_STRICMP: u32 = 0x00aa_1360;

// Translated from 00b7fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b7fa40`): the string maps' key compare, a virtual
/// method (`thiscall`, two stack words, `RET 8`) that twenty vtables share; `this` is not
/// read. True (1) when `NiStricmp` (`00aa1360`) finds the two keys equal, else 0
/// (`NEG`/`SBB`/`INC` on its result).
pub fn fn_00b7fce0(e: &mut Engine, _this: Ptr, key1: u32, key2: u32) -> bool {
    e.call(NI_STRICMP, &args![key1, key2]).u32() == 0
}

/// `memcpy` (`00ec44d0`, `cdecl`, three words).
const MEMCPY: u32 = 0x00ec_44d0;
/// The array of package entries' grow (`00a90180`, `thiscall`, one word: the new size) and set
/// (`00ba3aa0`, `thiscall`, two words: the index and the address of the element).
const ENTRY_ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const ENTRY_ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;

// Translated from 00b7fa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 0x108-byte package entry's constructor (`thiscall`, three
/// words: a name, a block of data and its size). The size goes to `+0x100`; with a size and
/// data, a copy of the data (`size` bytes from the memory manager, `00aa3e40` on `011f6238`,
/// filled by `memcpy`) goes to `+0x104`, else `+0x104` is cleared. The 0x100-byte name at `+0`
/// is cleared (`_memset`) and, with a name, filled by `_strcpy_s` (0x100). Answers `this`.
pub fn fn_00b7fa40(e: &mut Engine, this: Ptr, name: u32, data: u32, size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x100, size);
    if size != 0 && data != 0 {
        let copy = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        let size = e.mem.u32(base + 0x100);
        e.mem.set_u32(base + 0x104, copy);
        e.call(MEMCPY, &args![copy, data, size]);
    } else {
        e.mem.set_u32(base + 0x104, 0);
    }
    e.call(MEMSET, &args![base, 0u32, 0x100u32]);
    if name != 0 {
        e.call(STRCPY_S, &args![base, 0x100u32, name]);
    }
    this
}

// Translated from 00b7fd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of the map `fn_00b7fbe0` cleans
/// (`thiscall`, one word: the flags): `fn_00b7fbe0`, then with bit 0 of the flags the map goes
/// back to the memory manager (`00aa4060` on `011f6238`). Answers `this`.
pub fn fn_00b7fd00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    fn_00b7fbe0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b7fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of the map `fn_00b7fc50` cleans
/// (`thiscall`, one word: the flags): `fn_00b7fc50`, then with bit 0 of the flags the map goes
/// back to the memory manager (`00aa4060` on `011f6238`). Answers `this`.
pub fn fn_00b7fd20(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    fn_00b7fc50(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b7fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds a package entry at the end of an `NiTArray` of entries
/// (`thiscall` on the array, three words: the name, the data and its size). 0x108 bytes from
/// the memory manager (`00aa3e40` on `011f6238`) are constructed by `fn_00b7fa40` with the
/// three words (null when the allocation gave none); when `m_usSize` is not below
/// `m_usMaxSize` the array grows to `m_usSize + m_usGrowBy` (`00a90180`), then the entry is set
/// at index `m_usSize` (`00ba3aa0` with the address of the stack slot holding it).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b7fd90 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b7fd90(e: &mut Engine, this: Ptr, name: u32, data: u32, size: u32) {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x108u32])
        .u32();
    let entry = if block != 0 {
        fn_00b7fa40(e, Ptr::new(block), name, data, size).addr()
    } else {
        0
    };
    let array: Ptr<NiTArray> = this.cast();
    let index = u32::from(e.get(array, NiTArray::m_usSize));
    let capacity = u32::from(e.get(array, NiTArray::m_usMaxSize));
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), entry);
        if index >= capacity {
            let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
            e.call(
                ENTRY_ARRAY_SET_SIZE_00A90180,
                &args![this, grow.wrapping_add(index)],
            );
        }
        e.call(ENTRY_ARRAY_SET_AT_00BA3AA0, &args![this, index, element]);
    });
}

use crate::types::NiTArray;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_fa40, fn_00b7fa40(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00b7_fd00, fn_00b7fd00(Ptr, u8) -> Ptr),
        entry!(0x00b7_fd20, fn_00b7fd20(Ptr, u8) -> Ptr),
        entry!(0x00b7_fd90, fn_00b7fd90(Ptr, u32, u32, u32)),
        entry!(0x00b7_fbe0, fn_00b7fbe0(Ptr) -> u32),
        entry!(0x00b7_faf0, fn_00b7faf0() -> u32),
        entry!(0x00b7_ff40, fn_00b7ff40(Ptr, i32) -> Ptr),
        entry!(0x00b7_fe40, fn_00b7fe40(Ptr, Ptr) -> bool),
        entry!(0x00b7_fd50, fn_00b7fd50(Ptr) -> u32),
        entry!(0x00b7_fc50, fn_00b7fc50(Ptr) -> u32),
        entry!(0x00b8_0040, fn_00b80040(Ptr, u32, u32, u32)),
        entry!(0x00b7_fac0, fn_00b7fac0(Ptr, u32) -> u32),
        entry!(0x00b7_fce0, fn_00b7fce0(Ptr, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn settings(e: &mut Engine, setting: u32, a4: u8, e1: u8, f1: u8) -> u32 {
        e.set_global(SHADER_SETTING_011F91C0, setting);
        e.set_global(BYTE_SETTING_011F94A4, a4);
        e.set_global(BYTE_SETTING_011F941E, e1);
        e.set_global(BYTE_SETTING_011F941F, f1);
        e.call(0x00b7_faf0, &args![]).u32()
    }

    #[test]
    fn fn_00b7faf0_picks_the_package_for_each_setting() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        assert_eq!(settings(&mut e, 0, 1, 1, 1), 0);
        assert_eq!(settings(&mut e, 1, 1, 1, 1), 1);
        assert_eq!(settings(&mut e, 2, 1, 1, 1), 2);
        assert_eq!(settings(&mut e, 3, 1, 1, 1), 5);
        assert_eq!(settings(&mut e, 4, 1, 1, 1), 8);
        assert_eq!(settings(&mut e, 5, 0, 0, 1), 3);
        assert_eq!(settings(&mut e, 5, 0, 1, 0), 0xa);
        assert_eq!(settings(&mut e, 5, 0, 1, 7), 0xb);
        assert_eq!(settings(&mut e, 5, 1, 0, 1), 4);
        assert_eq!(settings(&mut e, 5, 1, 1, 0), 0xc);
        assert_eq!(settings(&mut e, 5, 1, 1, 1), 0xd);
        assert_eq!(settings(&mut e, 6, 0, 0, 1), 6);
        assert_eq!(settings(&mut e, 6, 0, 1, 1), 0xf);
        assert_eq!(settings(&mut e, 6, 1, 0, 1), 7);
        assert_eq!(settings(&mut e, 6, 1, 1, 0), 0x10);
        assert_eq!(settings(&mut e, 6, 1, 1, 1), 0x11);
        assert_eq!(settings(&mut e, 7, 1, 0, 1), 9);
        assert_eq!(settings(&mut e, 7, 0, 1, 0), 0x12);
        assert_eq!(settings(&mut e, 7, 0, 1, 1), 0x13);
        assert_eq!(settings(&mut e, 8, 1, 1, 1), 0);
        assert_eq!(settings(&mut e, 0xffff_ffff, 1, 1, 1), 0);
    }

    fn package_rig() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(1)).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        // The formatter writes the index as three digits after a short prefix.
        e.register(FORMAT_00406D00, |e, a| {
            let text = format!("pkg{:03}", a[3]);
            e.mem.set_cstr(a[0], text.as_bytes());
            (text.len() as u32).into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e
    }

    #[test]
    fn fn_00b7ff40_sets_up_the_map_and_names_the_package() {
        let mut e = package_rig();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this, 0x1111);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b7_ff40, &args![this, 7u32]).u32(), this);
        assert_eq!(calls(&e, ALLOCATE_00AA1070), vec![vec![0x94]]);
        let buckets = e.mem.u32(this + 0x10);
        assert_eq!(calls(&e, MEMSET), vec![vec![buckets, 0, 0x94]]);
        assert_eq!(e.mem.u32(this + 8), MAP_VTABLE_010AF6CC);
        assert_eq!(e.mem.u32(this + 0xc), 0x25);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 4), 0);
        let format = calls(&e, FORMAT_00406D00);
        assert_eq!(format.len(), 1);
        assert_eq!(&format[0][1..], &[100, PACKAGE_NAME_FORMAT, 7]);
        // "pkg007" and its NUL.
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 7]]);
        let name = e.mem.u32(this);
        assert_eq!(e.mem.cstr(name), b"pkg007".to_vec());
        assert_eq!(calls(&e, STRCPY_S)[0][..2], [name, 7]);
    }

    #[test]
    fn fn_00b7ff40_without_a_positive_index_leaves_no_name() {
        let mut e = package_rig();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this, 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x00b7_ff40, &args![this, 0u32]);
        e.call(0x00b7_ff40, &args![this, -3i32]);
        assert!(calls(&e, FORMAT_00406D00).is_empty());
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(this), 0);
    }

    /// A file object (vtable with slot 0 and +0x20, the read pointer at +8, position at +4)
    /// whose header read answers `header_got` bytes of (version, count, size) and whose block
    /// read answers `block_got`.
    fn file_rig(version: u32, count: u32, header_got: u32, block_got: u32) -> (Engine, u32) {
        let mut e = package_rig();
        let file = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 9];
        slots[0] = 0x0f00_3000;
        slots[8] = 0x0f00_3020;
        e.put_vtable(0x0f00_1000, &slots);
        e.mem.set_u32(file, 0x0f00_1000);
        e.mem.set_u32(file + 4, 0x40);
        e.mem.set_u32(file + 8, 0x0f00_4000);
        e.register(0x0f00_3000, |_, _| Ret::default());
        e.register(0x0f00_3020, |_, _| Ret::default());
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| file.into_ret());
        e.register_double(0x0f00_4000, move |e, a| {
            assert_eq!(a[2], 0xc);
            assert_eq!(e.mem.u32(a[3]), 1);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.bytes(a[1], 12), vec![0; 12]);
            e.mem.set_u32(a[1], version);
            e.mem.set_u32(a[1] + 4, count);
            e.mem.set_u32(a[1] + 8, 0x20c);
            header_got.into_ret()
        });
        // Two records: the first 4 bytes of text after its 0x104-byte head, then the second.
        e.register_double(FILE_READ_00462D80, move |e, a| {
            e.mem.set_u32(a[1] + 0x100, 4);
            block_got.into_ret()
        });
        e.register(SHADER_MAP_SET_AT, |_, _| Ret::default());
        (e, file)
    }

    #[test]
    fn fn_00b7fe40_loads_every_shader_record() {
        let (mut e, file) = file_rig(100, 2, 12, 0x20c);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b7_fe40, &args![this, 0x5000u32]).bool());
        assert_eq!(calls(&e, FILE_FINDER_GET_FILE), vec![vec![0x5000, 0, 0]]);
        assert_eq!(calls(&e, 0x0f00_3020), vec![vec![file, 0, 0]]);
        assert_eq!(e.mem.u32(file + 4), 0x4c);
        let contents = e.mem.u32(this + 4);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x20c]]
        );
        assert_eq!(
            calls(&e, FILE_READ_00462D80),
            vec![vec![file, contents, 0x20c]]
        );
        let second = contents + 0x108;
        assert_eq!(
            calls(&e, SHADER_MAP_SET_AT),
            vec![
                vec![this + 8, contents, contents],
                vec![this + 8, second, second]
            ]
        );
        assert_eq!(calls(&e, 0x0f00_3000), vec![vec![file, 1]]);
    }

    #[test]
    fn fn_00b7fe40_frees_the_contents_after_a_short_read() {
        let (mut e, file) = file_rig(100, 2, 12, 0x20b);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00b7_fe40, &args![this, 0x5000u32]).bool());
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls(&e, SHADER_MAP_SET_AT).is_empty());
        assert_eq!(calls(&e, 0x0f00_3000), vec![vec![file, 1]]);
    }

    #[test]
    fn fn_00b7fe40_with_no_shaders_still_answers_true() {
        let (mut e, _) = file_rig(100, 0, 12, 0x20c);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b7_fe40, &args![this, 0x5000u32]).bool());
        assert!(calls(&e, SHADER_MAP_SET_AT).is_empty());
    }

    #[test]
    fn fn_00b7fe40_skips_a_bad_header_but_still_deletes_the_file() {
        for (version, got) in [(99u32, 12u32), (100, 11)] {
            let (mut e, file) = file_rig(version, 2, got, 0x20c);
            let this = e.mem.alloc(0x1c);
            e.call_log = Some(vec![]);
            assert!(!e.call(0x00b7_fe40, &args![this, 0x5000u32]).bool());
            assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
            assert_eq!(calls(&e, 0x0f00_3000), vec![vec![file, 1]]);
        }
    }

    #[test]
    fn fn_00b7fe40_without_a_name_or_a_file_answers_false() {
        let mut e = package_rig();
        e.register(FILE_FINDER_GET_FILE, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00b7_fe40, &args![this, 0u32]).bool());
        assert!(calls(&e, FILE_FINDER_GET_FILE).is_empty());
        assert!(!e.call(0x00b7_fe40, &args![this, 0x5000u32]).bool());
        assert_eq!(calls(&e, FILE_FINDER_GET_FILE).len(), 1);
    }

    #[test]
    fn fn_00b7fd50_frees_what_it_holds_then_cleans_up_the_map() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(MAP_CLEAN_UP_00B7FC50, |_, _| 0x99u32.into_ret());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this, 0x1000);
        e.mem.set_u32(this + 4, 0x2000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b7_fd50, &args![this]).u32(), 0x99);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1000], vec![MEMORY_MANAGER, 0x2000]]
        );
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(calls(&e, MAP_CLEAN_UP_00B7FC50), vec![vec![this + 8]]);

        // Nothing held: only the map's clean-up.
        e.call_log = Some(vec![]);
        e.call(0x00b7_fd50, &args![this]);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(calls(&e, MAP_CLEAN_UP_00B7FC50).len(), 1);
    }

    /// A map with three buckets: bucket 0 holds two entries (keys 0x10, 0x11), bucket 1 is
    /// empty, bucket 2 holds one (key 0x20).
    fn map_rig(owns: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(MAP_BASE_CLEAN_UP_00B7FBE0, |_, _| 0x77u32.into_ret());
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        let third = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x10);
        e.mem.set_u32(second + 4, 0x11);
        e.mem.set_u32(third + 4, 0x20);
        e.mem.set_u32(buckets, first);
        e.mem.set_u32(buckets + 8, third);
        e.mem.set_u32(map, 0x1234);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, owns);
        (e, map)
    }

    #[test]
    fn fn_00b7fc50_frees_every_key_when_the_map_owns_them() {
        let (mut e, map) = map_rig(1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b7_fc50, &args![map]).u32(), 0x77);
        assert_eq!(e.mem.u32(map), MAP_VTABLE_010AF6CC);
        assert_eq!(
            calls(&e, FREE_00AA10F0),
            vec![vec![0x10], vec![0x11], vec![0x20]]
        );
        assert_eq!(calls(&e, MAP_BASE_CLEAN_UP_00B7FBE0), vec![vec![map]]);
    }

    #[test]
    fn fn_00b7fc50_frees_nothing_when_the_map_does_not_own_its_keys() {
        let (mut e, map) = map_rig(0);
        e.call_log = Some(vec![]);
        e.call(0x00b7_fc50, &args![map]);
        assert!(calls(&e, FREE_00AA10F0).is_empty());
        assert_eq!(calls(&e, MAP_BASE_CLEAN_UP_00B7FBE0).len(), 1);
        // No buckets: nothing walked either.
        let (mut e, map) = map_rig(1);
        e.mem.set_u32(map + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b7_fc50, &args![map]);
        assert!(calls(&e, FREE_00AA10F0).is_empty());
    }
}

#[cfg(test)]
mod base_clean_up_tests {
    use super::*;

    #[test]
    fn fn_00b7fbe0_empties_the_map_under_both_vtables_and_frees_the_buckets() {
        const SEEN: u32 = 0x0e50_0000;
        let mut e = Engine::new();
        e.map(SEEN, 0x10);
        e.register(MAP_REMOVE_ALL_00B601A0, |e, a| {
            let count = e.mem.u32(SEEN);
            let vtable = e.mem.u32(a[0]);
            e.mem.set_u32(SEEN + 4 + 4 * count, vtable);
            e.mem.set_u32(SEEN, count + 1);
            Ret::default()
        });
        e.register(FREE_00AA10F0, |_, _| 0x5au32.into_ret());
        e.call_log = Some(vec![]);
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 8, 0x0e60_0000);
        assert_eq!(e.call(0x00b7_fbe0, &args![map]).u32(), 0x5a);
        assert_eq!(e.mem.u32(SEEN), 2);
        assert_eq!(e.mem.u32(SEEN + 4), MAP_VTABLE_010AF6B0);
        assert_eq!(e.mem.u32(SEEN + 8), MAP_VTABLE_010ADE18);
        assert_eq!(e.mem.u32(map), MAP_VTABLE_010ADE18);
        let log = e.call_log.take().unwrap();
        let freed: Vec<_> = log.iter().filter(|c| c.0 == FREE_00AA10F0).collect();
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0].1, vec![0x0e60_0000]);
    }
}

#[cfg(test)]
mod blend_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::{
        NEW_RENDER_STATES_00E7F5D0, SET_RENDER_STATE_00E7F430,
    };
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00b80040_sets_both_blend_states() {
        let mut e = pass_rig();
        let pass = e.mem.alloc(0x68);
        e.call(0x00b8_0040, &args![pass, 5u32, 6u32, 0u32]);
        assert_eq!(render_states(&e, pass), vec![(0x13, 5), (0x14, 6)]);
        assert_eq!(calls(&e, NEW_RENDER_STATES_00E7F5D0).len(), 1);
        // A block already there is kept.
        let block = e.mem.u32(pass + 0x30);
        e.call(0x00b8_0040, &args![pass, 2u32, 1u32, 9u32]);
        assert_eq!(e.mem.u32(pass + 0x30), block);
        assert_eq!(calls(&e, NEW_RENDER_STATES_00E7F5D0).len(), 1);
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).len(), 4);
    }
}

#[cfg(test)]
mod package_find_tests {
    use super::*;

    #[test]
    fn fn_00b7fac0_answers_what_the_map_finds() {
        let mut e = Engine::new();
        e.register(SHADER_MAP_GET_AT_00A53C30, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            if a[1] == 0x0e00_0010 {
                e.mem.set_u32(a[2], 0x0e00_0100);
            }
            u32::from(a[1] == 0x0e00_0010).into_ret()
        });
        e.call_log = Some(vec![]);
        let package = e.mem.alloc(0x1c);
        assert_eq!(
            e.call(0x00b7_fac0, &args![package, 0x0e00_0010u32]).u32(),
            0x0e00_0100
        );
        assert_eq!(
            e.call(0x00b7_fac0, &args![package, 0x0e00_0020u32]).u32(),
            0
        );
        let log = e.call_log.take().unwrap();
        let maps: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == SHADER_MAP_GET_AT_00A53C30)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(maps, vec![package + 8, package + 8]);
    }
}

#[cfg(test)]
mod b048_tests {
    use super::*;

    #[test]
    fn fn_00b7fce0_is_true_when_stricmp_answers_zero() {
        let mut e = Engine::new();
        e.register(NI_STRICMP, |_, a| {
            u32::from(a[0] != a[1]).wrapping_neg().into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b7_fce0, &args![0x5000u32, 0x10u32, 0x10u32])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00b7_fce0, &args![0x5000u32, 0x10u32, 0x20u32])
                .u32(),
            0
        );
        let log = e.call_log.take().unwrap();
        let compares: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == NI_STRICMP)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(compares, vec![vec![0x10, 0x10], vec![0x10, 0x20]]);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(STRCPY_S, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00b7fa40_copies_the_data_and_the_name() {
        let mut e = rig();
        let entry = e.mem.alloc(0x108);
        assert_eq!(
            e.call(0x00b7_fa40, &args![entry, 0x500u32, 0x600u32, 0x20u32])
                .u32(),
            entry
        );
        assert_eq!(e.mem.u32(entry + 0x100), 0x20);
        let copy = e.mem.u32(entry + 0x104);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x20]]);
        assert_eq!(calls(&e, MEMCPY), vec![vec![copy, 0x600, 0x20]]);
        assert_eq!(calls(&e, MEMSET), vec![vec![entry, 0, 0x100]]);
        assert_eq!(calls(&e, STRCPY_S), vec![vec![entry, 0x100, 0x500]]);
        // No data (or no size) and no name: nothing copied.
        e.call_log = Some(vec![]);
        e.call(0x00b7_fa40, &args![entry, 0u32, 0u32, 0x20u32]);
        assert_eq!(e.mem.u32(entry + 0x104), 0);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        assert!(calls(&e, STRCPY_S).is_empty());
        e.call(0x00b7_fa40, &args![entry, 0u32, 0x600u32, 0u32]);
        assert_eq!(e.mem.u32(entry + 0x100), 0);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
    }

    #[test]
    fn scalar_deleting_destructors_give_back_on_bit_0() {
        let mut e = rig();
        e.register(MAP_REMOVE_ALL_00B601A0, |_, _| Ret::default());
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        let map = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00b7_fd00, &args![map, 0u32]).u32(), map);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.mem.u32(map), MAP_VTABLE_010ADE18);
        e.call(0x00b7_fd00, &args![map, 1u32]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map]]
        );
        // The derived one stores its own vtable first and ends in the base.
        e.call_log = Some(vec![]);
        e.call(0x00b7_fd20, &args![map, 1u32]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0).len(), 2);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00b7_fd20, &args![map, 0u32]);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
    }

    #[test]
    fn fn_00b7fd90_adds_a_new_entry() {
        let mut e = rig();
        e.register(ENTRY_ARRAY_SET_SIZE_00A90180, |_, _| Ret::default());
        e.register(ENTRY_ARRAY_SET_AT_00BA3AA0, |e, a| {
            let entry = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 4, entry);
            Ret::default()
        });
        let array = e.mem.alloc(0x10);
        e.mem.set_u16(array + 8, 2);
        e.mem.set_u16(array + 0xa, 1);
        e.mem.set_u16(array + 0xe, 4);
        e.call(0x00b7_fd90, &args![array, 0x500u32, 0x600u32, 8u32]);
        let entry = e.mem.u32(array + 4);
        assert_ne!(entry, 0);
        assert_eq!(e.mem.u32(entry + 0x100), 8);
        assert!(calls(&e, ENTRY_ARRAY_SET_SIZE_00A90180).is_empty());
        assert_eq!(calls(&e, ENTRY_ARRAY_SET_AT_00BA3AA0)[0][..2], [array, 1]);
        // Full: grows by the grow-by count first; a failed allocation adds a null entry.
        e.mem.set_u16(array + 0xa, 2);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00b7_fd90, &args![array, 0x500u32, 0x600u32, 8u32]);
        assert_eq!(
            calls(&e, ENTRY_ARRAY_SET_SIZE_00A90180),
            vec![vec![array, 6]]
        );
        assert_eq!(calls(&e, ENTRY_ARRAY_SET_AT_00BA3AA0)[1][..2], [array, 2]);
        assert_eq!(e.mem.u32(array + 4), 0);
    }
}
