//! `Havok SDK/hknativepackfileutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dc0f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_0f20`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dc0f80(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_0f20, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dc0eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(anonymous namespace) hkNativeResource::getName` (Xbox PDB): takes no arguments and returns the constant 0x010d_6ab4.
/// The constant is the address of a NUL-terminated string in the exe.
pub fn hk_native_resource_get_name(_e: &mut Engine) -> u32 {
    0x010d_6ab4
}

// Layouts, by role as the code uses them (the Xbox PDB has no packfile types). The packfile
// header read by `validate` and `load`: the magic words at +0 and +4, four layout-rule bytes
// at +0x10..+0x13, the section count at +0x14, the index and offset of the contents section at
// +0x18 and +0x1c, the version string at +0x28, and the section headers (0x30 bytes each) at
// +0x40. A section header: a 19-byte tag, a byte at +0x13, the data start (+0x14), and the
// offsets of the local fixups (+0x18, also the size of the data), global fixups (+0x1c),
// virtual fixups (+0x20), exports (+0x24), imports (+0x28) and the end (+0x2c). The loaded
// object (a `hkNativeResource`'s buffer): the magic `0xd5109142` at +0, the contents offset at
// +4, a hash map (`hkMapBase<unsigned long, unsigned long>`: pairs array, element count, hash
// mask) at +8 and an `hkArray` of the 0x30-byte section headers (data, size, capacity word) at
// +0x14.
const SECTIONS_AT: u32 = 0x40;
const SECTION_SIZE: u32 = 0x30;
const LOADED_MAGIC: u32 = 0xd510_9142;
const DONT_DEALLOCATE: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The vtables of `hkNativeResource` (`010d6a94`) and its base (`0102e388`).
const RESOURCE_VTABLE: u32 = 0x010d_6a94;
const BASE_VTABLE: u32 = 0x0102_e388;
/// `hkString::memCpy` (`00cb1260`, cdecl `(destination, source, size)`), `hkString::memSet`
/// (`00cb1280`, cdecl `(destination, value, size)`) and `hkString::strCmp` (`00cb0fa0`, cdecl).
const MEM_COPY: u32 = 0x00cb_1260;
const MEM_SET: u32 = 0x00cb_1280;
const STR_CMP: u32 = 0x00cb_0fa0;
/// `hkVersionUtil::getCurrentVersion` (`00dc0fb0`, no arguments, the version string).
const GET_CURRENT_VERSION: u32 = 0x00dc_0fb0;
/// The map (`hkmap.cpp`): the constructor `(this, buffer, bytes)` (`00cb1830`), insert
/// `(this, allocator, key, value)` (`00cb1870`), `getWithDefault` `(this, key, default)`
/// (`00cb1930`, answers the value), `clearAndDeallocate` `(this, allocator)` (`00cb1a90`) and
/// the function that gives the bytes a map of `n` elements needs (`00cb17f0`, cdecl).
const MAP_CONSTRUCT: u32 = 0x00cb_1830;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_BYTES_FOR: u32 = 0x00cb_17f0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// the function at `00d9ec10` that swaps the three words of two arrays (`this`, the other).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_SWAP: u32 = 0x00d9_ec10;
/// `00a29680`: a function that only returns, which `unload` ends by jumping to with the map
/// in `ECX`.
const RETURN_ONLY: u32 = 0x00a2_9680;
/// The section header functions `00e02f60` and `00e03010` (thiscall `(section, data, out)`
/// as the code calls them: `ECX` the loaded section header, then two stack words).
const SECTION_FIRST: u32 = 0x00e0_2f60;
const SECTION_SECOND: u32 = 0x00e0_3010;
/// The singleton whose slot `0xc` answers the registry `load` uses when it is given none, and
/// the dword (four layout-rule bytes) the header's bytes `+0x10..+0x13` are compared with.
const REGISTRY_SINGLETON: u32 = 0x0126_8b08;
const LAYOUT_RULES: u32 = 0x011b_7b88;
/// The messages `validatePackfileHeader` can store (addresses of strings in the exe), in the
/// order of its checks: no data, bad magic, and one for each layout byte, alignment, a missing
/// version and a version that differs from the current one.
const MESSAGE_NO_DATA: u32 = 0x010d_6a80;
const MESSAGE_BAD_MAGIC: u32 = 0x010d_6a44;
const MESSAGE_LAYOUT_BYTE_0: u32 = 0x010d_69f0;
const MESSAGE_LAYOUT_BYTE_1: u32 = 0x010d_69a0;
const MESSAGE_LAYOUT_BYTE_2: u32 = 0x010d_6940;
const MESSAGE_LAYOUT_BYTE_3: u32 = 0x010d_68d8;
const MESSAGE_MISALIGNED: u32 = 0x010d_68a8;
const MESSAGE_NO_VERSION: u32 = 0x010d_6888;
const MESSAGE_WRONG_VERSION: u32 = 0x010d_6860;

/// The signed quotient by 4 the exe computes with `CDQ`, `AND 3`, `ADD`, `SAR 2`.
fn quarter(difference: u32) -> i32 {
    (difference as i32) / 4
}

/// The signed quotient by 12 the exe computes with a multiplication by `0x2aaaaaab` (exact for
/// every 32-bit word).
fn twelfth(difference: u32) -> i32 {
    (difference as i32) / 12
}

// Translated from 00dc0640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativePackfileUtils::validatePackfileHeader` (Xbox PDB), cdecl `(data, error)`: returns 1
/// for a packfile it rejects and 0 for one it accepts; for a rejection it stores the address of
/// the message through `error` when `error` is not null. Checks, in order: `data` is not null;
/// the magic words (`0x57e0e057`, `0x10c0c010`); the four layout bytes at `+0x10..+0x13`
/// against the dword at `011b7b88`; the address of `data` is a multiple of 4; the byte at
/// `+0x28` is not `0xff`; and that string equals the current version
/// (`hkVersionUtil::getCurrentVersion`, `hkString::strCmp`). A local block of 0x40 bytes is
/// filled with `0xff` (`hkString::memSet`) and gets the two magic words and a zero byte at +0x28
/// that nothing reads.
pub fn hk_native_packfile_utils_validate_packfile_header(
    e: &mut Engine,
    data: u32,
    error: u32,
) -> u32 {
    let reject = |e: &mut Engine, message: u32| -> u32 {
        if error != 0 {
            e.mem.set_u32(error, message);
        }
        1
    };
    if data == 0 {
        return reject(e, MESSAGE_NO_DATA);
    }
    e.with_stack(0x40, |e, local| {
        e.call(MEM_SET, &args![local, 0xffff_ffffu32, 0x40u32]);
        e.mem.set_u32(local.addr(), 0x57e0_e057);
        e.mem.set_u32(local.addr() + 4, 0x10c0_c010);
        e.mem.set_u8(local.addr() + 0x28, 0);
        if e.mem.u32(data) != 0x57e0_e057 || e.mem.u32(data + 4) != 0x10c0_c010 {
            return reject(e, MESSAGE_BAD_MAGIC);
        }
        let rules = e.mem.u32(LAYOUT_RULES);
        if e.mem.u8(data + 0x10) != rules as u8 {
            return reject(e, MESSAGE_LAYOUT_BYTE_0);
        }
        if e.mem.u8(data + 0x11) != (rules >> 8) as u8 {
            return reject(e, MESSAGE_LAYOUT_BYTE_1);
        }
        if e.mem.u8(data + 0x12) != e.mem.u8(LAYOUT_RULES + 2) {
            return reject(e, MESSAGE_LAYOUT_BYTE_2);
        }
        if e.mem.u8(data + 0x13) != e.mem.u8(LAYOUT_RULES + 3) {
            return reject(e, MESSAGE_LAYOUT_BYTE_3);
        }
        if data & 3 != 0 {
            return reject(e, MESSAGE_MISALIGNED);
        }
        let version = data + 0x28;
        if e.mem.u8(version) == 0xff {
            return reject(e, MESSAGE_NO_VERSION);
        }
        let current = e.call(GET_CURRENT_VERSION, &args![]).u32();
        if e.call(STR_CMP, &args![version, current]).u32() != 0 {
            return reject(e, MESSAGE_WRONG_VERSION);
        }
        0
    })
}

// Translated from 00dc07d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the bytes `hkNativePackfileUtils::load` needs for the packfile
/// at `data` (cdecl `(data, second word)`; the second word is not read): `count * 0x30 + 0x20`
/// plus, over the sections (the headers at `data + 0x40`, none when `count` is not above 0),
/// the sum of `(local fixups + end - exports)` offsets, plus the bytes the map function
/// `00cb17f0` gives for the sum of `(exports - virtual fixups) / 12` (signed quotients taken per
/// section), as the exe sums them: sections in pairs unrolled, then a single section left over
/// for an odd count.
pub fn fn_00dc07d0(e: &mut Engine, data: Ptr, _unused_2: u32) -> u32 {
    let data = data.addr();
    let count = e.mem.i32(data + 0x14);
    let sections = if count > 0 { data + SECTIONS_AT } else { 0 };
    let mut quotients = 0i32;
    let mut sizes = 0u32;
    let mut leftover_size = 0u32;
    let mut processed = 0i32;
    if count >= 2 {
        let pairs = (count - 2) as u32 >> 1;
        for pair in 0..=pairs {
            for half in 0..2u32 {
                let section = sections + 0x60 * pair + SECTION_SIZE * half;
                let virtual_fixups = e.mem.u32(section + 0x20);
                let exports = e.mem.u32(section + 0x24);
                quotients = quotients.wrapping_add(twelfth(exports.wrapping_sub(virtual_fixups)));
                sizes = sizes.wrapping_add(
                    e.mem
                        .u32(section + 0x18)
                        .wrapping_add(e.mem.u32(section + 0x2c))
                        .wrapping_sub(exports),
                );
            }
        }
        processed = (pairs as i32 + 1) * 2;
    }
    let mut single = 0i32;
    if processed < e.mem.i32(data + 0x14) {
        let section = sections + SECTION_SIZE * processed as u32;
        let virtual_fixups = e.mem.u32(section + 0x20);
        let exports = e.mem.u32(section + 0x24);
        single = twelfth(exports.wrapping_sub(virtual_fixups));
        leftover_size = e
            .mem
            .u32(section + 0x18)
            .wrapping_sub(e.mem.u32(section + 0x28))
            .wrapping_sub(exports)
            .wrapping_add(e.mem.u32(section + 0x2c))
            .wrapping_add(e.mem.u32(section + 0x28));
    }
    let map_bytes = e
        .call(MAP_BYTES_FOR, &args![quotients.wrapping_add(single)])
        .u32();
    let count_bytes = (e.mem.u32(data + 0x14)).wrapping_mul(SECTION_SIZE);
    count_bytes
        .wrapping_add(sizes)
        .wrapping_add(map_bytes)
        .wrapping_add(0x20)
        .wrapping_add(leftover_size)
}

// Translated from 00dc08f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativePackfileUtils::getImportsExports` (Xbox PDB), cdecl `(loaded, second, third)`:
/// `loaded` is the loaded object (see the layouts above). For each of its sections (the count
/// at `+0x18` of the object is read again for each; the headers' array pointer at `+0x14` too),
/// with `data` the section's data (`loaded + the header's +0x14`), it calls the section
/// functions `00e02f60` and `00e03010` (thiscall on the header) with `(data, second)` and
/// `(data, third)`.
pub fn hk_native_packfile_utils_get_imports_exports(
    e: &mut Engine,
    loaded: Ptr,
    second: u32,
    third: u32,
) {
    let loaded = loaded.addr();
    if e.mem.i32(loaded + 0x18) <= 0 {
        return;
    }
    let mut offset = 0u32;
    let mut index = 0i32;
    loop {
        let header = e.mem.u32(loaded + 0x14).wrapping_add(offset);
        let data = e.mem.u32(header + 0x14).wrapping_add(loaded);
        e.call(SECTION_FIRST, &args![header, data, second]);
        e.call(SECTION_SECOND, &args![header, data, third]);
        offset = offset.wrapping_add(SECTION_SIZE);
        index += 1;
        if index >= e.mem.i32(loaded + 0x18) {
            break;
        }
    }
}

// Translated from 00dc0940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativePackfileUtils::load` (Xbox PDB), cdecl `(data, second word, out, size, registry)`
/// (the second word is not read): builds the loaded object at `out` (`size` bytes) from the
/// packfile at `data` and returns the address of its contents (`out + the word at out + 4`).
/// The header gets the magic `0xd5109142` and the `hkArray` of section headers (at `out + 0x14`,
/// the headers at `out + 0x20`, the capacity word marked as not owning). Each section's data
/// goes after the headers at an offset that grows by the size it takes (`hkString::memCpy`
/// for the header's tag, the data with its local fixups, the exports and the imports); the
/// local fixups are applied (pairs `(offset, target)`, `-1` skipped: a pointer to the target),
/// and when the section is the contents section (`data + 0x18`) the word at `out + 4` is
/// the contents offset plus the section's offset. Then the global fixups are applied
/// (triples `(offset, section, target)`: a pointer into the other section, or 0 when that
/// section's local fixups offset is 0), the hash map at `out + 8` is constructed in the rest of
/// the buffer (`00cb1830`), the registry is the one the singleton at `01268b08` answers (slot
/// `0xc`) when it is 0, and the virtual fixups (triples `(offset, section, class name)`) are
/// run: the registry's slot `0x10` is called with the object's address and the class name's
/// address (0 when that section's local fixups offset is 0), and a nonzero answer is inserted in
/// the map (`00cb1870`) under the object's address, with the memory router's allocator.
pub fn hk_native_packfile_utils_load(
    e: &mut Engine,
    data: Ptr,
    _unused_2: u32,
    out: Ptr,
    size: i32,
    registry: u32,
) -> u32 {
    let (data, out) = (data.addr(), out.addr());
    let count = e.mem.i32(data + 0x14);
    let sections = if count > 0 { data + SECTIONS_AT } else { 0 };
    e.mem.set_u32(out, LOADED_MAGIC);
    let array = out.wrapping_add(0x14);
    if array != 0 {
        let sections_count = e.mem.u32(data + 0x14);
        e.mem.set_u32(array + 4, sections_count);
        e.mem.set_u32(array, out + 0x20);
        e.mem.set_u32(array + 8, sections_count | DONT_DEALLOCATE);
    }
    let mut next = (e.mem.u32(data + 0x14))
        .wrapping_mul(SECTION_SIZE)
        .wrapping_add(0x20);
    if e.mem.i32(data + 0x14) > 0 {
        let mut index = 0u32;
        loop {
            let header = sections.wrapping_add(SECTION_SIZE * index);
            let loaded = e.mem.u32(out + 0x14).wrapping_add(SECTION_SIZE * index);
            e.call(MEM_COPY, &args![loaded, header, 0x13u32]);
            let null_byte = e.mem.u8(header + 0x13);
            e.mem.set_u32(loaded + 0x14, next);
            e.mem.set_u8(loaded + 0x13, null_byte);
            for offset in [0x18, 0x1c, 0x20, 0x24] {
                let near_fixups = e.mem.u32(header + 0x18);
                e.mem.set_u32(loaded + offset, near_fixups);
            }
            let after_data = e
                .mem
                .u32(header + 0x28)
                .wrapping_sub(e.mem.u32(header + 0x24))
                .wrapping_add(e.mem.u32(header + 0x18));
            e.mem.set_u32(loaded + 0x28, after_data);
            let destination = out.wrapping_add(next);
            let end = e
                .mem
                .u32(header + 0x2c)
                .wrapping_sub(e.mem.u32(header + 0x24))
                .wrapping_add(e.mem.u32(header + 0x18));
            e.mem.set_u32(loaded + 0x2c, end);
            let source = e.mem.u32(header + 0x14).wrapping_add(data);
            let data_size = e.mem.u32(header + 0x18);
            e.call(MEM_COPY, &args![destination, source, data_size]);
            let exports = e.mem.u32(header + 0x24);
            let imports = e.mem.u32(header + 0x28);
            let exports_at = e.mem.u32(loaded + 0x24);
            e.call(
                MEM_COPY,
                &args![
                    exports_at.wrapping_add(destination),
                    exports.wrapping_add(source),
                    imports.wrapping_sub(exports)
                ],
            );
            let imports = e.mem.u32(header + 0x28);
            let end_of_imports = e.mem.u32(header + 0x2c);
            let imports_at = e.mem.u32(loaded + 0x28);
            e.call(
                MEM_COPY,
                &args![
                    imports_at.wrapping_add(destination),
                    imports.wrapping_add(source),
                    end_of_imports.wrapping_sub(imports)
                ],
            );
            if index as i32 == e.mem.i32(data + 0x18) {
                let contents = e.mem.u32(data + 0x1c).wrapping_add(next);
                e.mem.set_u32(out + 4, contents);
            }
            // The local fixups: pairs (offset in the section, target offset).
            let near_fixups = e.mem.u32(header + 0x18);
            let table = near_fixups.wrapping_add(source);
            let words = quarter(e.mem.u32(header + 0x1c).wrapping_sub(near_fixups));
            if words > 0 {
                let mut word = 0i32;
                loop {
                    let at = table.wrapping_add(word as u32 * 4);
                    let offset = e.mem.u32(at);
                    if offset != 0xffff_ffff {
                        let target = e.mem.u32(at + 4).wrapping_add(destination);
                        e.mem.set_u32(offset.wrapping_add(destination), target);
                    }
                    word += 2;
                    let words = quarter(
                        e.mem
                            .u32(header + 0x1c)
                            .wrapping_sub(e.mem.u32(header + 0x18)),
                    );
                    if word >= words {
                        break;
                    }
                }
            }
            next = next.wrapping_add(e.mem.u32(loaded + 0x2c));
            index += 1;
            if index as i32 >= e.mem.i32(data + 0x14) {
                break;
            }
        }
    }
    // The global fixups: triples (offset, section, target offset).
    if e.mem.i32(data + 0x14) > 0 {
        let mut index = 0u32;
        loop {
            let header = sections.wrapping_add(SECTION_SIZE * index);
            let loaded = e.mem.u32(out + 0x14).wrapping_add(SECTION_SIZE * index);
            let section_at = e.mem.u32(loaded + 0x14).wrapping_add(out);
            let table = e
                .mem
                .u32(header + 0x14)
                .wrapping_add(e.mem.u32(header + 0x1c))
                .wrapping_add(data);
            let words = quarter(
                e.mem
                    .u32(header + 0x20)
                    .wrapping_sub(e.mem.u32(header + 0x1c)),
            );
            if words > 0 {
                let mut word = 0i32;
                loop {
                    let at = table.wrapping_add(word as u32 * 4);
                    let offset = e.mem.u32(at);
                    if offset != 0xffff_ffff {
                        let other = e
                            .mem
                            .u32(out + 0x14)
                            .wrapping_add(e.mem.u32(at + 4).wrapping_mul(SECTION_SIZE));
                        let target = if e.mem.u32(other + 0x18) != 0 {
                            e.mem
                                .u32(other + 0x14)
                                .wrapping_add(e.mem.u32(at + 8))
                                .wrapping_add(out)
                        } else {
                            0
                        };
                        e.mem.set_u32(offset.wrapping_add(section_at), target);
                    }
                    word += 3;
                    let words = quarter(
                        e.mem
                            .u32(header + 0x20)
                            .wrapping_sub(e.mem.u32(header + 0x1c)),
                    );
                    if word >= words {
                        break;
                    }
                }
            }
            index += 1;
            if index as i32 >= e.mem.i32(data + 0x14) {
                break;
            }
        }
    }
    // The hash map goes in what is left of the buffer.
    let map = out.wrapping_add(8);
    if map != 0 {
        let rest = (size as u32).wrapping_sub(next);
        e.call(MAP_CONSTRUCT, &args![map, next.wrapping_add(out), rest]);
    }
    let mut registry = registry;
    if registry == 0 {
        let singleton = e.mem.u32(REGISTRY_SINGLETON);
        registry = e.vcall(singleton, 0xc, &args![]).u32();
    }
    // The virtual fixups: triples (offset, section, class name offset).
    if e.mem.i32(data + 0x14) > 0 {
        let mut index = 0u32;
        loop {
            let header = sections.wrapping_add(SECTION_SIZE * index);
            let loaded = e.mem.u32(out + 0x14).wrapping_add(SECTION_SIZE * index);
            let section_at = e.mem.u32(loaded + 0x14).wrapping_add(out);
            let table = e
                .mem
                .u32(header + 0x14)
                .wrapping_add(e.mem.u32(header + 0x20))
                .wrapping_add(data);
            let words = quarter(
                e.mem
                    .u32(header + 0x24)
                    .wrapping_sub(e.mem.u32(header + 0x20)),
            );
            if words > 0 {
                let mut word = 0i32;
                loop {
                    let at = table.wrapping_add(word as u32 * 4);
                    let offset = e.mem.u32(at);
                    if offset != 0xffff_ffff {
                        let other = e
                            .mem
                            .u32(out + 0x14)
                            .wrapping_add(e.mem.u32(at + 4).wrapping_mul(SECTION_SIZE));
                        let name = if e.mem.u32(other + 0x18) != 0 {
                            e.mem
                                .u32(other + 0x14)
                                .wrapping_add(e.mem.u32(at + 8))
                                .wrapping_add(out)
                        } else {
                            0
                        };
                        let object = offset.wrapping_add(section_at);
                        let type_info = e.vcall(registry, 0x10, &args![object, name]).u32();
                        if type_info != 0 {
                            let router = e.call(MEMORY_ROUTER, &args![]).u32();
                            let allocator = e.mem.u32(router + 0x10);
                            e.call(MAP_INSERT, &args![out + 8, allocator, object, type_info]);
                        }
                    }
                    word += 3;
                    let words = quarter(
                        e.mem
                            .u32(header + 0x24)
                            .wrapping_sub(e.mem.u32(header + 0x20)),
                    );
                    if word >= words {
                        break;
                    }
                }
            }
            index += 1;
            if index as i32 >= e.mem.i32(data + 0x14) {
                break;
            }
        }
    }
    e.mem.u32(out + 4).wrapping_add(out)
}

// Translated from 00dc0cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativePackfileUtils::load_ov2` (Xbox PDB), cdecl `(data, second word, registry)` (the
/// second word is only passed on): works out the size (`00dc07d0`), makes a local `hkArray` of
/// bytes (null, size 0, capacity word `0x80000000`) big enough (the memory router's allocator,
/// `00c85750` field `+0x10`, `hkArrayUtil::_reserve` `00c908c0` with the larger of twice the
/// capacity and the size, element size 1, when the capacity is below the size), sets its size
/// and loads into it (`load`, `00dc0940`). The value the map at the buffer's `+8` has under the
/// contents address (`getWithDefault`, default 0; the first word of what it points to) goes
/// into a new `hkNativeResource` (0x1c bytes from the allocator's slot 4, reference count 1,
/// size 0x1c, vtable `010d6a94`, an empty array at `+8` that takes the local array by
/// `00d9ec10`, the contents address at `+0x14` and that value at `+0x18`). The local array's
/// size is then cleared and its data freed (slot `0x10`) unless its capacity word says it is not
/// owned. Returns the resource.
pub fn hk_native_packfile_utils_load_ov2(
    e: &mut Engine,
    data: Ptr,
    second: u32,
    registry: u32,
) -> Ptr {
    e.with_stack(0xc, |e, array| {
        let base = array.addr();
        e.mem.set_u32(base, 0);
        e.mem.set_u32(base + 4, 0);
        e.mem.set_u32(base + 8, DONT_DEALLOCATE);
        let needed = fn_00dc07d0(e, data, second);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(base + 8) & CAPACITY_MASK;
        if (capacity as i32) < needed as i32 {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if (needed as i32) < doubled as i32 {
                doubled
            } else {
                needed
            };
            e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 1u32]);
        }
        let buffer = e.mem.u32(base);
        e.mem.set_u32(base + 4, needed);
        let contents = hk_native_packfile_utils_load(
            e,
            data,
            second,
            Ptr::new(buffer),
            needed as i32,
            registry,
        );
        let found = e
            .call(
                MAP_GET_WITH_DEFAULT,
                &args![buffer.wrapping_add(8), contents, 0u32],
            )
            .u32();
        let value = if found != 0 { e.mem.u32(found) } else { 0 };
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let resource = e.vcall(allocator, 4, &args![0x1cu32]).u32();
        e.mem.set_u16(resource + 6, 1);
        e.mem.set_u16(resource + 4, 0x1c);
        e.mem.set_u32(resource, RESOURCE_VTABLE);
        e.mem.set_u32(resource + 8, 0);
        e.mem.set_u32(resource + 0xc, 0);
        e.mem.set_u32(resource + 0x10, DONT_DEALLOCATE);
        e.mem.set_u32(resource + 0x14, contents);
        e.mem.set_u32(resource + 0x18, value);
        e.call(ARRAY_SWAP, &args![resource + 8, array]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(base + 8);
        let allocator = e.mem.u32(router + 0x10);
        e.mem.set_u32(base + 4, 0);
        if (capacity as i32) >= 0 {
            let freed = e.mem.u32(base);
            e.vcall(allocator, 0x10, &args![freed, capacity & CAPACITY_MASK]);
        }
        Ptr::new(resource)
    })
}

// Translated from 00dc0de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativePackfileUtils::unload` (Xbox PDB), cdecl `(loaded, second word)` (the second word is
/// not read). Clears the first word of the loaded object, then walks its hash map (at `+8`; the
/// pairs `(key, value)` whose key is not `-1`, indices `0..=` the mask) and for each pair calls
/// the function at `+8` of the structure the value points to, when it is not null, with the key.
/// Then it empties the section header array (size 0, frees the data through slot `0x10` of the
/// memory router's allocator unless the capacity word marks it as not owned, with
/// `capacity * 0x30` bytes; pointer 0, capacity word `0x80000000`), runs `clearAndDeallocate`
/// (`00cb1a90`) on the map and ends in a jump to `00a29680`.
pub fn hk_native_packfile_utils_unload(e: &mut Engine, loaded: Ptr, _unused_2: u32) {
    let loaded = loaded.addr();
    let map = loaded + 8;
    e.mem.set_u32(loaded, 0);
    let mut mask = e.mem.i32(map + 8);
    let mut index = 0i32;
    // The first occupied pair.
    if mask >= 0 {
        let pairs = e.mem.u32(map);
        loop {
            if e.mem.u32(pairs.wrapping_add(index as u32 * 8)) != 0xffff_ffff {
                break;
            }
            index += 1;
            if index > mask {
                break;
            }
        }
    }
    if index <= mask {
        loop {
            let pairs = e.mem.u32(map);
            let slot = pairs.wrapping_add(index as u32 * 8);
            let value = e.mem.u32(slot + 4);
            let function = e.mem.u32(value + 8);
            if function != 0 {
                let key = e.mem.u32(slot);
                e.call(function, &args![key]);
            }
            mask = e.mem.i32(map + 8);
            index += 1;
            if index <= mask {
                let pairs = e.mem.u32(map);
                loop {
                    if e.mem.u32(pairs.wrapping_add(index as u32 * 8)) != 0xffff_ffff {
                        break;
                    }
                    index += 1;
                    if index > mask {
                        break;
                    }
                }
            }
            if index > mask {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(loaded + 0x1c);
    e.mem.set_u32(loaded + 0x18, 0);
    if (capacity as i32) >= 0 {
        let sections = e.mem.u32(loaded + 0x14);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(SECTION_SIZE);
        e.vcall(allocator, 0x10, &args![sections, bytes]);
    }
    e.mem.set_u32(loaded + 0x14, 0);
    e.mem.set_u32(loaded + 0x1c, DONT_DEALLOCATE);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(RETURN_ONLY, &args![map]);
}

// Translated from 00dc0ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual method with two stack words it never reads that returns
/// the word at `this + 0x14` (the `hkNativeResource`'s contents address).
pub fn fn_00dc0ec0(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    e.mem.u32(this.addr() + 0x14)
}

// Translated from 00dc0ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(anonymous namespace) hkNativeResource::getImportsExports` (Xbox PDB), `(first, second)`:
/// calls `getImportsExports` (`00dc08f0`) with the resource's buffer (the word at `+8`) and the
/// two stack words in the opposite order: `(buffer, second, first)`.
pub fn hk_native_resource_get_imports_exports(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let buffer = e.mem.u32(this.addr() + 8);
    hk_native_packfile_utils_get_imports_exports(e, Ptr::new(buffer), second, first);
}

// Translated from 00dc0ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(anonymous namespace) hkNativeResource::callDestructors` (Xbox PDB): when the word at
/// `this + 0x14` (the contents) is not 0, runs `unload` (`00dc0de0`) with the buffer (`+8`) and
/// the size (`+0xc`), and clears the words at `+0x14` and `+0x18`.
pub fn hk_native_resource_call_destructors(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x14) != 0 {
        let buffer = e.mem.u32(this.addr() + 8);
        let size = e.mem.u32(this.addr() + 0xc);
        hk_native_packfile_utils_unload(e, Ptr::new(buffer), size);
        e.mem.set_u32(this.addr() + 0x14, 0);
        e.mem.set_u32(this.addr() + 0x18, 0);
    }
}

// Translated from 00dc0f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(anonymous namespace) hkNativeResource::~hkNativeResource` (Xbox PDB): sets the vtable
/// (`010d6a94`) and, when the contents word (`+0x14`) is not 0, unloads as `callDestructors`
/// does. Then clears the size word (`+0xc`), frees the buffer (`+8`) through slot `0x10` of the
/// memory router's allocator (`00c85750`, field `+0x10`) with the capacity word's low bits
/// unless the capacity word (`+0x10`) is negative, sets the buffer to 0 and the capacity word to
/// `0x80000000`, and ends with the base vtable (`0102e388`).
pub fn hk_native_resource_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, RESOURCE_VTABLE);
    if e.mem.u32(base + 0x14) != 0 {
        let buffer = e.mem.u32(base + 8);
        let size = e.mem.u32(base + 0xc);
        hk_native_packfile_utils_unload(e, Ptr::new(buffer), size);
        e.mem.set_u32(base + 0x14, 0);
        e.mem.set_u32(base + 0x18, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0xc, 0);
    if (capacity as i32) >= 0 {
        let buffer = e.mem.u32(base + 8);
        e.vcall(allocator, 0x10, &args![buffer, capacity & CAPACITY_MASK]);
    }
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, DONT_DEALLOCATE);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc0640, hk_native_packfile_utils_validate_packfile_header(u32, u32) -> u32),
        entry!(0x00dc07d0, fn_00dc07d0(Ptr, u32) -> u32),
        entry!(
            0x00dc08f0,
            hk_native_packfile_utils_get_imports_exports(Ptr, u32, u32)
        ),
        entry!(0x00dc0940, hk_native_packfile_utils_load(Ptr, u32, Ptr, i32, u32) -> u32),
        entry!(0x00dc0cd0, hk_native_packfile_utils_load_ov2(Ptr, u32, u32) -> Ptr),
        entry!(0x00dc0de0, hk_native_packfile_utils_unload(Ptr, u32)),
        entry!(0x00dc0ec0, fn_00dc0ec0(Ptr, u32, u32) -> u32),
        entry!(
            0x00dc0ed0,
            hk_native_resource_get_imports_exports(Ptr, u32, u32)
        ),
        entry!(0x00dc0ef0, hk_native_resource_call_destructors(Ptr)),
        entry!(0x00dc0f20, hk_native_resource_destructor(Ptr)),
        entry!(0x00dc0eb0, hk_native_resource_get_name() -> u32),
        entry!(0x00dc_0f80, deleting_destructor_00dc0f80(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00dc0f80_frees_only_when_bit_zero_is_set() {
        check(0x00dc_0f80, 0x00dc_0f20);
    }

    #[test]
    fn hk_native_resource_get_name_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00dc_0eb0, &args![]).u32(), 0x010d_6ab4);
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1)
            .collect()
    }

    fn copy_double(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[1], a[2]);
        e.mem.write(a[0], &bytes);
        Ret {
            eax: a[0],
            ..Ret::default()
        }
    }

    /// A router whose allocator allocates in the heap (slot 4) and frees (slot `0x10`, nothing).
    /// Returns the allocator.
    fn install_allocator(e: &mut Engine) -> u32 {
        e.put_vtable(0x4400_0000, &[0, 0x4500_0004, 0, 0, 0x4500_0010]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_0000);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x4500_0004, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(0x4500_0010, |_, _| Ret::default());
        allocator
    }

    /// A valid packfile header at an address that is a multiple of 8 (plus `skew`), with the
    /// layout rules `01 02 03 04` and the version "7.1.0-r1".
    fn header(e: &mut Engine, skew: u32) -> u32 {
        e.mem.set_u32(LAYOUT_RULES, 0x0403_0201);
        let base = e.mem.alloc(0x80) + skew;
        e.mem.set_u32(base, 0x57e0_e057);
        e.mem.set_u32(base + 4, 0x10c0_c010);
        e.mem.write(base + 0x10, &[1, 2, 3, 4]);
        e.mem.set_cstr(base + 0x28, b"7.1.0-r1");
        base
    }

    fn validation_engine() -> Engine {
        let mut e = engine();
        let current = e.mem.alloc(16);
        e.mem.set_cstr(current, b"7.1.0-r1");
        e.register(MEM_SET, |_, _| Ret::default());
        e.register_double(GET_CURRENT_VERSION, move |_, _| Ret {
            eax: current,
            ..Ret::default()
        });
        e.register(STR_CMP, |e, a| Ret {
            eax: u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])),
            ..Ret::default()
        });
        e
    }

    #[test]
    fn a_valid_header_is_accepted_and_the_error_is_left_alone() {
        let mut e = validation_engine();
        let data = header(&mut e, 0);
        let error = e.mem.alloc(4);
        e.mem.set_u32(error, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dc_0640, &args![data, error]).u32(), 0);
        assert_eq!(e.mem.u32(error), 0x1234);
        // the null error pointer is allowed too
        assert_eq!(e.call(0x00dc_0640, &args![data, 0u32]).u32(), 0);
        let fills = calls_to(&e, MEM_SET);
        assert_eq!(fills.len(), 2);
        assert_eq!(fills[0][1..], [0xffff_ffff, 0x40]);
    }

    /// Breaks one thing in a valid header: 0 and 1 the magic words, 2 to 5 the layout bytes,
    /// 6 the version byte.
    fn break_header(e: &mut Engine, data: u32, what: u32) {
        match what {
            0 => e.mem.set_u32(data, 0),
            1 => e.mem.set_u32(data + 4, 0),
            2..=5 => e.mem.set_u8(data + 0x10 + (what - 2), 9),
            _ => e.mem.set_u8(data + 0x28, 0xff),
        }
    }

    #[test]
    fn each_check_of_the_header_stores_its_own_message() {
        let mut e = validation_engine();
        let error = e.mem.alloc(4);
        let mut seen = vec![];
        // no data at all
        assert_eq!(e.call(0x00dc_0640, &args![0u32, error]).u32(), 1);
        seen.push(e.mem.u32(error));
        // the two magic words and the four layout bytes, in the order of the checks
        for what in 0..6 {
            let data = header(&mut e, 0);
            break_header(&mut e, data, what);
            assert_eq!(e.call(0x00dc_0640, &args![data, error]).u32(), 1);
            seen.push(e.mem.u32(error));
        }
        // a misaligned address
        let data = header(&mut e, 1);
        assert_eq!(e.call(0x00dc_0640, &args![data, error]).u32(), 1);
        seen.push(e.mem.u32(error));
        // the version byte is 0xff
        let data = header(&mut e, 0);
        break_header(&mut e, data, 6);
        assert_eq!(e.call(0x00dc_0640, &args![data, error]).u32(), 1);
        seen.push(e.mem.u32(error));
        // a different version
        let data = header(&mut e, 0);
        e.mem.set_cstr(data + 0x28, b"6.5.0-r1");
        assert_eq!(e.call(0x00dc_0640, &args![data, error]).u32(), 1);
        seen.push(e.mem.u32(error));
        assert_eq!(
            seen,
            vec![
                MESSAGE_NO_DATA,
                MESSAGE_BAD_MAGIC,
                MESSAGE_BAD_MAGIC,
                MESSAGE_LAYOUT_BYTE_0,
                MESSAGE_LAYOUT_BYTE_1,
                MESSAGE_LAYOUT_BYTE_2,
                MESSAGE_LAYOUT_BYTE_3,
                MESSAGE_MISALIGNED,
                MESSAGE_NO_VERSION,
                MESSAGE_WRONG_VERSION,
            ]
        );
        // without an error pointer a failure still answers 1
        assert_eq!(e.call(0x00dc_0640, &args![0u32, 0u32]).u32(), 1);
    }

    /// A packfile with `count` sections whose offsets exercise the size computation.
    fn sized_packfile(e: &mut Engine, count: u32) -> (u32, u32) {
        let data = e.mem.alloc(0x40 + 0x30 * count.max(1));
        e.mem.set_u32(data + 0x14, count);
        let mut sizes = 0u32;
        let mut quotients = 0u32;
        for i in 0..count {
            let section = data + 0x40 + 0x30 * i;
            let virtual_fixups = 0x100 * i + 5;
            let exports = virtual_fixups + 12 * (i + 1) + 7;
            let imports = exports + 8;
            let end = imports + 4 * i;
            e.mem.set_u32(section + 0x18, 0x20 + 4 * i);
            e.mem.set_u32(section + 0x20, virtual_fixups);
            e.mem.set_u32(section + 0x24, exports);
            e.mem.set_u32(section + 0x28, imports);
            e.mem.set_u32(section + 0x2c, end);
            quotients += (exports - virtual_fixups) / 12;
            sizes += 0x20 + 4 * i + end - exports;
        }
        (data, 0x30 * count + sizes + (quotients * 16 + 1) + 0x20)
    }

    #[test]
    fn the_size_adds_the_sections_the_map_and_the_header() {
        for count in 0..=5u32 {
            let mut e = Engine::new();
            e.register(MAP_BYTES_FOR, |_, a| Ret {
                eax: a[0] * 16 + 1,
                ..Ret::default()
            });
            let (data, expected) = sized_packfile(&mut e, count);
            let got = e.call(0x00dc_07d0, &args![data, 0u32]).u32();
            assert_eq!(got, expected, "{count} sections");
        }
    }

    /// A loaded object with two section headers whose data starts are 0x10 and 0x30.
    fn loaded_object(e: &mut Engine) -> u32 {
        let loaded = e.mem.alloc(0x40);
        let headers = e.mem.alloc(0x60);
        e.mem.set_u32(headers + 0x14, 0x10);
        e.mem.set_u32(headers + 0x30 + 0x14, 0x30);
        e.mem.set_u32(loaded + 0x14, headers);
        e.mem.set_u32(loaded + 0x18, 2);
        loaded
    }

    #[test]
    fn imports_and_exports_are_asked_of_every_section() {
        let mut e = Engine::new();
        e.register(SECTION_FIRST, |_, _| Ret::default());
        e.register(SECTION_SECOND, |_, _| Ret::default());
        let loaded = loaded_object(&mut e);
        let headers = e.mem.u32(loaded + 0x14);
        e.call_log = Some(vec![]);
        e.call(0x00dc_08f0, &args![loaded, 0xaau32, 0xbbu32]);
        assert_eq!(
            calls_to(&e, SECTION_FIRST),
            vec![
                vec![headers, loaded + 0x10, 0xaa],
                vec![headers + 0x30, loaded + 0x30, 0xaa]
            ]
        );
        assert_eq!(
            calls_to(&e, SECTION_SECOND),
            vec![
                vec![headers, loaded + 0x10, 0xbb],
                vec![headers + 0x30, loaded + 0x30, 0xbb]
            ]
        );
        // no sections: nothing is asked
        e.mem.set_u32(loaded + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x00dc_08f0, &args![loaded, 1u32, 2u32]);
        assert!(calls_to(&e, SECTION_FIRST).is_empty());
    }

    #[test]
    fn the_resource_method_passes_its_words_in_the_other_order() {
        let mut e = Engine::new();
        e.register(SECTION_FIRST, |_, _| Ret::default());
        e.register(SECTION_SECOND, |_, _| Ret::default());
        let loaded = loaded_object(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, loaded);
        e.call_log = Some(vec![]);
        e.call(0x00dc_0ed0, &args![this, 0x11u32, 0x22u32]);
        // (buffer, second, first): the first section function gets the second word
        assert_eq!(calls_to(&e, SECTION_FIRST)[0][2], 0x22);
        assert_eq!(calls_to(&e, SECTION_SECOND)[0][2], 0x11);
        // the contents word is returned by 00dc0ec0
        e.mem.set_u32(this + 0x14, 0x7777);
        assert_eq!(e.call(0x00dc_0ec0, &args![this, 1u32, 2u32]).u32(), 0x7777);
    }

    /// A packfile of two sections: the class names (0x10 bytes) and the data (0x20 bytes of data
    /// followed by a local fixup pair, a global fixup triple and a virtual fixup triple).
    /// Returns its address.
    fn two_section_packfile(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x200);
        e.mem.set_u32(data + 0x14, 2);
        e.mem.set_u32(data + 0x18, 1);
        e.mem.set_u32(data + 0x1c, 0x10);
        let first = data + 0x40;
        e.mem.write(first, b"__classnames__");
        e.mem.set_u8(first + 0x13, 1);
        e.mem.set_u32(first + 0x14, 0x100);
        for offset in [0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            e.mem.set_u32(first + offset, 0x10);
        }
        e.mem.write(data + 0x100, b"ClassA\0........");
        let second = data + 0x70;
        e.mem.write(second, b"__data__");
        e.mem.set_u8(second + 0x13, 2);
        e.mem.set_u32(second + 0x14, 0x110);
        for (offset, value) in [
            (0x18, 0x20),
            (0x1c, 0x28),
            (0x20, 0x34),
            (0x24, 0x40),
            (0x28, 0x40),
            (0x2c, 0x40),
        ] {
            e.mem.set_u32(second + offset, value);
        }
        for i in 0..0x20 {
            e.mem.set_u8(data + 0x110 + i, 0xa0 + i as u8);
        }
        // local fixup (offset 0 points at offset 0x18), global (offset 4, section 0, target 8),
        // virtual (object at 8, section 0, class name at 0)
        for (offset, value) in [
            (0x20, 0u32),
            (0x24, 0x18),
            (0x28, 4),
            (0x2c, 0),
            (0x30, 8),
            (0x34, 8),
            (0x38, 0),
            (0x3c, 0),
        ] {
            e.mem.set_u32(data + 0x110 + offset, value);
        }
        data
    }

    fn loading_engine() -> Engine {
        let mut e = engine();
        e.register(MEM_COPY, copy_double);
        e.register(MAP_CONSTRUCT, |_, _| Ret::default());
        e.register(MAP_INSERT, |_, _| Ret::default());
        e
    }

    /// A registry object whose slot `0x10` answers `0x7777` for every object.
    fn install_registry(e: &mut Engine) -> u32 {
        e.put_vtable(0x4400_1000, &[0, 0, 0, 0, 0x4500_0020]);
        let registry = e.mem.alloc(8);
        e.mem.set_u32(registry, 0x4400_1000);
        e.register(0x4500_0020, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        registry
    }

    #[test]
    fn load_builds_the_sections_applies_the_fixups_and_registers_the_objects() {
        let mut e = loading_engine();
        let allocator = install_allocator(&mut e);
        let registry = install_registry(&mut e);
        let data = two_section_packfile(&mut e);
        let out = e.mem.alloc(0x400);
        e.mem.write(out, &[0xcd; 0x400]);
        e.call_log = Some(vec![]);
        let contents = e
            .call(0x00dc_0940, &args![data, 0u32, out, 0x400i32, registry])
            .u32();
        assert_eq!(contents, out + 0xa0);
        assert_eq!(e.mem.u32(out), 0xd510_9142);
        assert_eq!(e.mem.u32(out + 4), 0xa0);
        // the header array: the headers at +0x20, two of them, not owned
        assert_eq!(e.mem.u32(out + 0x14), out + 0x20);
        assert_eq!(e.mem.u32(out + 0x18), 2);
        assert_eq!(e.mem.u32(out + 0x1c), 0x8000_0002);
        // the loaded headers: tag, byte, data offset, the local fixups offset four times, ends
        let first = out + 0x20;
        let second = out + 0x50;
        assert_eq!(e.mem.bytes(first, 14), b"__classnames__");
        assert_eq!(e.mem.u8(first + 0x13), 1);
        assert_eq!(e.mem.u32(first + 0x14), 0x80);
        for offset in [0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(first + offset), 0x10);
        }
        assert_eq!(e.mem.bytes(second, 8), b"__data__");
        assert_eq!(e.mem.u8(second + 0x13), 2);
        assert_eq!(e.mem.u32(second + 0x14), 0x90);
        for offset in [0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(second + offset), 0x20);
        }
        // the data copied after the headers
        assert_eq!(e.mem.bytes(out + 0x80, 7), b"ClassA\0");
        assert_eq!(e.mem.u8(out + 0x90 + 0x08), 0xa8);
        assert_eq!(e.mem.u8(out + 0x90 + 0x1f), 0xbf);
        // the local fixup: offset 0 now points at out + 0x90 + 0x18
        assert_eq!(e.mem.u32(out + 0x90), out + 0x90 + 0x18);
        // the global fixup: offset 4 points into the class names, at offset 8
        assert_eq!(e.mem.u32(out + 0x94), out + 0x80 + 8);
        // the map is built in the rest of the buffer, then the virtual fixup is registered
        assert_eq!(
            calls_to(&e, MAP_CONSTRUCT),
            vec![vec![out + 8, out + 0xb0, 0x400 - 0xb0]]
        );
        assert_eq!(
            calls_to(&e, 0x4500_0020),
            vec![vec![registry, out + 0x98, out + 0x80]]
        );
        assert_eq!(
            calls_to(&e, MAP_INSERT),
            vec![vec![out + 8, allocator, out + 0x98, 0x7777]]
        );
        // the four copies of each section: header, data, exports, imports
        let copies = calls_to(&e, MEM_COPY);
        assert_eq!(copies.len(), 8);
        assert_eq!(copies[0], vec![first, data + 0x40, 0x13]);
        assert_eq!(copies[1], vec![out + 0x80, data + 0x100, 0x10]);
        assert_eq!(copies[4], vec![second, data + 0x70, 0x13]);
        assert_eq!(copies[5], vec![out + 0x90, data + 0x110, 0x20]);
    }

    #[test]
    fn load_asks_the_singleton_for_a_registry_when_given_none() {
        let mut e = loading_engine();
        install_allocator(&mut e);
        let registry = install_registry(&mut e);
        e.put_vtable(0x4400_2000, &[0, 0, 0, 0x4500_0030]);
        let singleton = e.mem.alloc(8);
        e.mem.set_u32(singleton, 0x4400_2000);
        e.mem.set_u32(REGISTRY_SINGLETON, singleton);
        e.register_double(0x4500_0030, move |_, _| Ret {
            eax: registry,
            ..Ret::default()
        });
        let data = two_section_packfile(&mut e);
        let out = e.mem.alloc(0x400);
        e.call_log = Some(vec![]);
        e.call(0x00dc_0940, &args![data, 0u32, out, 0x400i32, 0u32]);
        assert_eq!(calls_to(&e, 0x4500_0030).len(), 1);
        assert_eq!(calls_to(&e, 0x4500_0020).len(), 1);
    }

    #[test]
    fn load_without_sections_only_writes_the_header() {
        let mut e = loading_engine();
        install_allocator(&mut e);
        let registry = install_registry(&mut e);
        let data = e.mem.alloc(0x100);
        let out = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        let contents = e
            .call(0x00dc_0940, &args![data, 0u32, out, 0x100i32, registry])
            .u32();
        // the header words are zero: the contents are at the buffer's start
        assert_eq!(contents, out);
        assert_eq!(e.mem.u32(out), 0xd510_9142);
        assert_eq!(e.mem.u32(out + 0x18), 0);
        assert_eq!(e.mem.u32(out + 0x1c), 0x8000_0000);
        // the map gets the buffer after the 0x20 header bytes
        assert_eq!(
            calls_to(&e, MAP_CONSTRUCT),
            vec![vec![out + 8, out + 0x20, 0xe0]]
        );
        assert!(calls_to(&e, MEM_COPY).is_empty());
    }

    #[test]
    fn load_ov2_makes_the_buffer_and_a_resource_that_owns_it() {
        let mut e = loading_engine();
        let allocator = install_allocator(&mut e);
        let registry = install_registry(&mut e);
        let data = two_section_packfile(&mut e);
        e.register(MAP_BYTES_FOR, |_, a| Ret {
            eax: 0x100 + 8 * a[0],
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            // (allocator, array, capacity, element size): the data and a capacity word it owns
            let block = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register(ARRAY_SWAP, |e, a| {
            for word in 0..3 {
                let mine = e.mem.u32(a[0] + 4 * word);
                let theirs = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, theirs);
                e.mem.set_u32(a[1] + 4 * word, mine);
            }
            Ret::default()
        });
        let value_cell = e.mem.alloc(4);
        e.mem.set_u32(value_cell, 0x5555);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, _| Ret {
            eax: value_cell,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let resource = e.call(0x00dc_0cd0, &args![data, 0u32, registry]).u32();
        // 2 headers, the section sizes 0x10 + 0x20, a 0x108 map (one virtual fixup) and 0x20
        let needed = 0x60 + 0x30 + 0x108 + 0x20;
        assert_eq!(calls_to(&e, ARRAY_RESERVE).len(), 1);
        assert_eq!(calls_to(&e, ARRAY_RESERVE)[0][0], allocator);
        assert_eq!(calls_to(&e, ARRAY_RESERVE)[0][2..], [needed, 1]);
        let load_calls = calls_to(&e, MAP_CONSTRUCT);
        assert_eq!(load_calls.len(), 1);
        let buffer = load_calls[0][0] - 8;
        assert_eq!(load_calls[0][2], needed - 0xb0);
        // the resource
        assert_eq!(e.mem.u16(resource + 4), 0x1c);
        assert_eq!(e.mem.u16(resource + 6), 1);
        assert_eq!(e.mem.u32(resource), 0x010d_6a94);
        assert_eq!(e.mem.u32(resource + 0x14), buffer + 0xa0);
        assert_eq!(e.mem.u32(resource + 0x18), 0x5555);
        // it took the buffer, its size and capacity from the local array
        assert_eq!(e.mem.u32(resource + 8), buffer);
        assert_eq!(e.mem.u32(resource + 0xc), needed);
        assert_eq!(e.mem.u32(resource + 0x10), needed);
        // the local array was left with the empty one: nothing is freed
        assert!(calls_to(&e, 0x4500_0010).is_empty());
        // the map was asked about the contents address, with default 0
        assert_eq!(
            calls_to(&e, MAP_GET_WITH_DEFAULT),
            vec![vec![buffer + 8, buffer + 0xa0, 0]]
        );
    }

    #[test]
    fn load_ov2_frees_a_stack_array_that_stayed_owned() {
        let mut e = loading_engine();
        install_allocator(&mut e);
        let registry = install_registry(&mut e);
        let data = two_section_packfile(&mut e);
        e.register(MAP_BYTES_FOR, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let block = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        // a swap that does nothing: the local array keeps its owned data
        e.register(ARRAY_SWAP, |_, _| Ret::default());
        e.register(MAP_GET_WITH_DEFAULT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let resource = e.call(0x00dc_0cd0, &args![data, 0u32, registry]).u32();
        let reserve = calls_to(&e, ARRAY_RESERVE);
        // no value in the map: 0 in the resource; the local data is freed with its capacity
        assert_eq!(e.mem.u32(resource + 0x18), 0);
        let frees = calls_to(&e, 0x4500_0010);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][2], reserve[0][2]);
    }

    /// A loaded object with a hash map of four slots (mask 3): keys `-1, 0x1000, -1, 0x2000`;
    /// the first value's structure has a function at `+8`, the second's has none. Its header
    /// array holds capacity 4 (owned).
    fn unloadable(e: &mut Engine) -> u32 {
        let loaded = e.mem.alloc(0x40);
        let pairs = e.mem.alloc(0x20);
        let with_function = e.mem.alloc(0x10);
        e.mem.set_u32(with_function + 8, 0x4500_0040);
        let without = e.mem.alloc(0x10);
        for (slot, key, value) in [
            (0u32, 0xffff_ffffu32, 0u32),
            (1, 0x1000, with_function),
            (2, 0xffff_ffff, 0),
            (3, 0x2000, without),
        ] {
            e.mem.set_u32(pairs + 8 * slot, key);
            e.mem.set_u32(pairs + 8 * slot + 4, value);
        }
        e.mem.set_u32(loaded, 0xd510_9142);
        e.mem.set_u32(loaded + 8, pairs);
        e.mem.set_u32(loaded + 0xc, 2);
        e.mem.set_u32(loaded + 0x10, 3);
        let headers = e.mem.alloc(0xc0);
        e.mem.set_u32(loaded + 0x14, headers);
        e.mem.set_u32(loaded + 0x18, 2);
        e.mem.set_u32(loaded + 0x1c, 4);
        e.register(0x4500_0040, |_, _| Ret::default());
        e.register(MAP_CLEAR_AND_DEALLOCATE, |_, _| Ret::default());
        e.register(RETURN_ONLY, |_, _| Ret::default());
        loaded
    }

    #[test]
    fn unload_runs_the_destructors_of_the_map_then_empties_the_object() {
        let mut e = Engine::new();
        let allocator = install_allocator(&mut e);
        let loaded = unloadable(&mut e);
        let headers = e.mem.u32(loaded + 0x14);
        e.call_log = Some(vec![]);
        e.call(0x00dc_0de0, &args![loaded, 0x99u32]);
        // only the pair whose structure has a function runs it, with the key
        assert_eq!(calls_to(&e, 0x4500_0040), vec![vec![0x1000]]);
        // the header array is freed with capacity * 0x30 bytes, then reset
        assert_eq!(
            calls_to(&e, 0x4500_0010),
            vec![vec![allocator, headers, 0xc0]]
        );
        assert_eq!(e.mem.u32(loaded), 0);
        assert_eq!(e.mem.u32(loaded + 0x14), 0);
        assert_eq!(e.mem.u32(loaded + 0x18), 0);
        assert_eq!(e.mem.u32(loaded + 0x1c), 0x8000_0000);
        assert_eq!(
            calls_to(&e, MAP_CLEAR_AND_DEALLOCATE),
            vec![vec![loaded + 8, allocator]]
        );
        assert_eq!(calls_to(&e, RETURN_ONLY), vec![vec![loaded + 8]]);
        // an array that is not owned is not freed, an empty map runs nothing
        let again = unloadable(&mut e);
        e.mem.set_u32(again + 0x1c, 0x8000_0004);
        e.mem.set_u32(again + 0x10, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x00dc_0de0, &args![again, 0u32]);
        assert!(calls_to(&e, 0x4500_0040).is_empty());
        assert!(calls_to(&e, 0x4500_0010).is_empty());
    }

    #[test]
    fn the_resource_unloads_before_it_goes() {
        let mut e = Engine::new();
        let allocator = install_allocator(&mut e);
        let loaded = unloadable(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x1111);
        e.mem.set_u32(this + 8, loaded);
        e.mem.set_u32(this + 0xc, 0x1b8);
        e.mem.set_u32(this + 0x10, 0x1b8);
        e.mem.set_u32(this + 0x14, 0x7777);
        e.mem.set_u32(this + 0x18, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x00dc_0ef0, &args![this]);
        assert_eq!(calls_to(&e, 0x4500_0040), vec![vec![0x1000]]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        // nothing more to unload: the second call does nothing
        e.call_log = Some(vec![]);
        e.call(0x00dc_0ef0, &args![this]);
        assert!(calls_to(&e, 0x4500_0040).is_empty());
        // the destructor with the contents set again unloads, then frees the buffer
        let loaded = unloadable(&mut e);
        e.mem.set_u32(this + 8, loaded);
        e.mem.set_u32(this + 0x14, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x00dc_0f20, &args![this]);
        assert_eq!(calls_to(&e, 0x4500_0040), vec![vec![0x1000]]);
        assert_eq!(
            calls_to(&e, 0x4500_0010)
                .into_iter()
                .filter(|call| call[1] == loaded)
                .collect::<Vec<_>>(),
            vec![vec![allocator, loaded, 0x1b8]]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
    }
}
