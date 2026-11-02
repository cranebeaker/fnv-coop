//! `NiMain/niparticlesdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiParticlesData::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLES_DATA_RTTI: u32 = 0x011f_5ef0;

// Translated from 00a967d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particles_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLES_DATA_RTTI)
}

// Translated from 00a96a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a96800`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x70 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_particles_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_6800;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x70u32]);
    }
    this
}

/// `NiParticlesData` fields (Xbox PDB names; PC offsets as the code uses
/// them): `NiGeometryData::m_usVertices` (the particle count, 16 bits)
/// +0x08, `m_pkVertex` +0x20, `m_pkNormal` +0x24, `m_pkColor` +0x28, then
/// `m_bIsOldRotatingParticlesObject` +0x40, `m_ausRenderOrder` +0x44,
/// `m_afDepths` +0x48, `m_pfRadii` +0x4c, `m_usActive` +0x50, `m_pfSizes`
/// +0x54, `m_pkRotations` +0x58, `m_pfRotationAngles` +0x5c,
/// `m_pkRotationAxes` +0x60, `m_pucTextureIndices` +0x64,
/// `m_pkSubTextureOffsets` +0x68 and `m_ucSubTextureOffsetCount` +0x6c.
const VERTEX_COUNT: u32 = 0x08;
const VERTICES: u32 = 0x20;
const NORMALS: u32 = 0x24;
const COLORS: u32 = 0x28;
const OLD_ROTATING: u32 = 0x40;
const RENDER_ORDER: u32 = 0x44;
const DEPTHS: u32 = 0x48;
const RADII: u32 = 0x4c;
const ACTIVE: u32 = 0x50;
const SIZES: u32 = 0x54;
const ROTATIONS: u32 = 0x58;
const ROTATION_ANGLES: u32 = 0x5c;
const ROTATION_AXES: u32 = 0x60;
const TEXTURE_INDICES: u32 = 0x64;
const SUB_TEXTURE_OFFSETS: u32 = 0x68;
const SUB_TEXTURE_OFFSET_COUNT: u32 = 0x6c;

/// The memory manager singleton and its `Allocate` (`00aa3e40`, `thiscall`,
/// one word) and `Deallocate` (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa1070` (unnamed in the engine map; `cdecl`, one word): the block
/// allocation `LoadBinary` uses for its float and byte arrays.
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiGeometryData::LoadBinary` (`00a67ea0`).
const GEOMETRY_DATA_LOAD_BINARY: u32 = 0x00a6_7ea0;
/// `00a41770` (unit `nimorphdata.cpp`, `cdecl`, `(stream, floats, count)`):
/// reads `count` floats.
const READ_FLOATS: u32 = 0x00a4_1770;
/// `NiColorA::LoadBinary` (`00a69800`) and `NiColor::LoadBinary`
/// (`00a59be0`), `thiscall` on the element, the stream as the argument.
const COLOR_A_LOAD_BINARY: u32 = 0x00a6_9800;
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `_memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
/// The four words a rotation without data starts as (`.data`).
const DEFAULT_ROTATION: u32 = 0x011a_9ea4;
/// The `NiRTTI` record whose objects keep old rotation data (the class is
/// not named in the code).
const ROTATIONS_KEPT_RTTI: u32 = 0x011f_5f0c;
/// The object's `GetRTTI` (virtual slot 2).
const SLOT_GET_RTTI: u32 = 0x08;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; PC offsets), the word at +4 that `LoadBinary` compares with 0x13
/// and 0x11 (the file's user version, by its use; unconfirmed), and the
/// input stream's position (+4) and read function (+8, `cdecl` `(stream,
/// buffer, size, &component size, component count)`, returns the bytes
/// read).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_USER_VERSION: u32 = 0x04;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// `MemoryManager::Allocate` of `count` elements of `size` bytes, the size
/// all ones when the product overflows (`MUL`, `SETO`, `NEG`, `OR`).
fn allocate_elements(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = count.checked_mul(size).unwrap_or(u32::MAX);
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32()
}

/// Copies `words` words from `from` to `to`, one at a time.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// Copies the float at index `from` to index `to` of the array at `array`
/// through the x87 stack (a signalling NaN comes out quiet).
fn copy_float(e: &mut Engine, array: u32, from: u32, to: u32) {
    let value = e.mem.u32(array.wrapping_add(from.wrapping_mul(4)));
    e.mem.set_u32(
        array.wrapping_add(to.wrapping_mul(4)),
        x87_quiet_bits(value),
    );
}

// Translated from 00a964c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::RemoveParticle` (Xbox PDB): removes particle `index`
/// by moving the last active particle into its place in every array there
/// is (the vertices always; colours, normals, radii, sizes, rotations,
/// rotation angles, rotation axes and texture indices when present; the
/// float arrays through the x87 stack), then lowers the active count. The
/// last particle itself only lowers the count.
pub fn ni_particles_data_remove_particle(e: &mut Engine, this: Ptr, index: u16) {
    let this = this.addr();
    let active = e.mem.u16(this.wrapping_add(ACTIVE));
    let last = active.wrapping_sub(1);
    if index != last {
        let from = u32::from(last);
        let to = u32::from(index);
        let vertices = e.mem.u32(this.wrapping_add(VERTICES));
        copy_words(
            e,
            vertices.wrapping_add(from * 12),
            vertices.wrapping_add(to * 12),
            3,
        );
        let colors = e.mem.u32(this.wrapping_add(COLORS));
        if colors != 0 {
            copy_words(
                e,
                colors.wrapping_add(from * 16),
                colors.wrapping_add(to * 16),
                4,
            );
        }
        let normals = e.mem.u32(this.wrapping_add(NORMALS));
        if normals != 0 {
            copy_words(
                e,
                normals.wrapping_add(from * 12),
                normals.wrapping_add(to * 12),
                3,
            );
        }
        let radii = e.mem.u32(this.wrapping_add(RADII));
        if radii != 0 {
            copy_float(e, radii, from, to);
        }
        let sizes = e.mem.u32(this.wrapping_add(SIZES));
        if sizes != 0 {
            copy_float(e, sizes, from, to);
        }
        let rotations = e.mem.u32(this.wrapping_add(ROTATIONS));
        if rotations != 0 {
            copy_words(
                e,
                rotations.wrapping_add(from * 16),
                rotations.wrapping_add(to * 16),
                4,
            );
        }
        let angles = e.mem.u32(this.wrapping_add(ROTATION_ANGLES));
        if angles != 0 {
            copy_float(e, angles, from, to);
        }
        let axes = e.mem.u32(this.wrapping_add(ROTATION_AXES));
        if axes != 0 {
            copy_words(
                e,
                axes.wrapping_add(from * 12),
                axes.wrapping_add(to * 12),
                3,
            );
        }
        let indices = e.mem.u32(this.wrapping_add(TEXTURE_INDICES));
        if indices != 0 {
            let byte = e.mem.u8(indices.wrapping_add(from));
            e.mem.set_u8(indices.wrapping_add(to), byte);
        }
    }
    let active = e.mem.u16(this.wrapping_add(ACTIVE));
    e.mem
        .set_u16(this.wrapping_add(ACTIVE), active.wrapping_add(0xffff));
}

// Translated from 00a966c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niparticlesdata.cpp`): the quicksort
/// `SortParticles` runs on the render order (`m_ausRenderOrder`, 16-bit
/// particle indices) between `left` and `right` (signed, inclusive),
/// ordering by the particles' depths (`m_afDepths`) from far to near. The
/// pivot is the depth of the first entry; entries go left while their
/// depth is `<=` the pivot from the right end and right while `>=` from the
/// left end (both false for NaN, as the exe's flag tests); the left part
/// recurses, the right part loops.
pub fn fn_00a966c0(e: &mut Engine, this: Ptr, left: i32, right: i32) {
    let base = this.addr();
    let order = |e: &Engine, i: i32| {
        let array = e.mem.u32(base.wrapping_add(RENDER_ORDER));
        array.wrapping_add((i as u32).wrapping_mul(2))
    };
    let depth = |e: &Engine, particle: u16| {
        let depths = e.mem.u32(base.wrapping_add(DEPTHS));
        x87_load32(e.mem.u32(depths.wrapping_add(u32::from(particle) * 4)))
    };
    let mut left = left;
    loop {
        let pivot_index = e.mem.u16(order(e, left));
        // The pivot goes through the argument's stack word (an x87 store
        // and load).
        let pivot = depth(e, pivot_index);
        let mut i = left;
        let mut j = right;
        if left < right {
            loop {
                let mut value = depth(e, e.mem.u16(order(e, j)));
                while value <= pivot {
                    if i >= j {
                        break;
                    }
                    j -= 1;
                    value = depth(e, e.mem.u16(order(e, j)));
                }
                if i != j {
                    let moved = e.mem.u16(order(e, j));
                    e.mem.set_u16(order(e, i), moved);
                    i += 1;
                }
                let mut value = depth(e, e.mem.u16(order(e, i)));
                while value >= pivot {
                    if i >= j {
                        break;
                    }
                    i += 1;
                    value = depth(e, e.mem.u16(order(e, i)));
                }
                if i == j {
                    break;
                }
                let moved = e.mem.u16(order(e, i));
                e.mem.set_u16(order(e, j), moved);
                j -= 1;
                if i >= j {
                    break;
                }
            }
        }
        e.mem.set_u16(order(e, i), pivot_index);
        if left < i {
            fn_00a966c0(e, this, left, i - 1);
        }
        if right <= i {
            return;
        }
        left = i + 1;
    }
}

// Translated from 00a96910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::SortParticles` (Xbox PDB): makes the render order and
/// depth arrays (one entry per particle, from the memory manager) when
/// there are none, sets the render order to 0, 1, 2, ..., and with more
/// than one active particle computes each one's depth as the dot product
/// of its position with the camera's direction (the floats at +0x68, +0x74
/// and +0x80 of `camera`; x87, `y*dy + x*dx`, then `+ z*dz`, stored as a
/// float) and sorts the order by depth (`00a966c0`).
pub fn ni_particles_data_sort_particles(e: &mut Engine, this: Ptr, camera: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(RENDER_ORDER)) == 0 {
        let count = u32::from(e.mem.u16(base.wrapping_add(VERTEX_COUNT)));
        let order = allocate_elements(e, count, 2);
        e.mem.set_u32(base.wrapping_add(RENDER_ORDER), order);
        let count = u32::from(e.mem.u16(base.wrapping_add(VERTEX_COUNT)));
        let depths = allocate_elements(e, count, 4);
        e.mem.set_u32(base.wrapping_add(DEPTHS), depths);
    }
    let mut i = 0u16;
    while i < e.mem.u16(base.wrapping_add(ACTIVE)) {
        let order = e.mem.u32(base.wrapping_add(RENDER_ORDER));
        e.mem.set_u16(order.wrapping_add(u32::from(i) * 2), i);
        i = i.wrapping_add(1);
    }
    let active = e.mem.u16(base.wrapping_add(ACTIVE));
    if active <= 1 {
        return;
    }
    let camera = camera.addr();
    let dx = x87_load32(e.mem.u32(camera.wrapping_add(0x68)));
    let dy = x87_load32(e.mem.u32(camera.wrapping_add(0x74)));
    let dz = x87_load32(e.mem.u32(camera.wrapping_add(0x80)));
    let mut i = 0u16;
    if i < active {
        loop {
            let vertices = e.mem.u32(base.wrapping_add(VERTICES));
            let vertex = vertices.wrapping_add(u32::from(i) * 12);
            let y = x87_load32(e.mem.u32(vertex.wrapping_add(4)));
            let x = x87_load32(e.mem.u32(vertex));
            let z = x87_load32(e.mem.u32(vertex.wrapping_add(8)));
            let sum = x87_add(x87_mul(y, dy), x87_mul(x, dx));
            let sum = x87_add(sum, x87_mul(z, dz));
            let depths = e.mem.u32(base.wrapping_add(DEPTHS));
            e.mem.set_u32(
                depths.wrapping_add(u32::from(i) * 4),
                x87_quiet_bits(x87_store32(sum)),
            );
            i = i.wrapping_add(1);
            if i >= e.mem.u16(base.wrapping_add(ACTIVE)) {
                break;
            }
        }
    }
    let active = i32::from(e.mem.u16(base.wrapping_add(ACTIVE)));
    fn_00a966c0(e, this, 0, active - 1);
}

/// Sets `count` floats of the array at `array` to 1.0 (`FLD1`/`FST`).
fn fill_ones(e: &mut Engine, this: u32, array_field: u32) {
    let mut i = 0u16;
    if i < e.mem.u16(this.wrapping_add(VERTEX_COUNT)) {
        loop {
            let array = e.mem.u32(this.wrapping_add(array_field));
            e.mem.set_f32(array.wrapping_add(u32::from(i) * 4), 1.0);
            i = i.wrapping_add(1);
            if i >= e.mem.u16(this.wrapping_add(VERTEX_COUNT)) {
                break;
            }
        }
    }
}

/// Runs `load` (`thiscall` on the element, the stream as the argument) on
/// each of the 16-bit-counted elements of `size` bytes in the array at
/// `array_field`, while the index is below the particle count.
fn load_elements(e: &mut Engine, this: u32, stream: u32, array_field: u32, size: u32, load: u32) {
    let mut i = 0u16;
    if i < e.mem.u16(this.wrapping_add(VERTEX_COUNT)) {
        loop {
            let array = e.mem.u32(this.wrapping_add(array_field));
            let element = array.wrapping_add(u32::from(i).wrapping_mul(size));
            e.call(load, &args![element, stream]);
            i = i.wrapping_add(1);
            if i >= e.mem.u16(this.wrapping_add(VERTEX_COUNT)) {
                break;
            }
        }
    }
}

// Translated from 00a96cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::LoadBinary` (Xbox PDB), after `NiGeometryData`'s load:
/// - files older than `0x04010007`: a 16-bit word skipped;
/// - the radii (`00aa1070`, 4 bytes per particle): one float for all of
///   them before `0x0a00010d`; from then on a "has radii" byte and, when
///   set and the word at stream +4 is below 0x13, the floats (`00a41770`),
///   otherwise 1.0 each;
/// - the active count (16 bits);
/// - the sizes: a "has sizes" flag (the old files' 4-byte array pointer,
///   kept in `m_pfSizes`, being non-zero before `0x04010007`; a byte from
///   then on, in the argument's own stack word), the array (4 bytes per
///   particle), then the floats in one read or 1.0 each, as for the radii;
/// - the rotations (`m_pkRotations`, 16 bytes per particle, memory
///   manager): for an old rotating-particles object, a flag (the old
///   pointer before `0x04010000`, a byte after) and either one
///   `NiColorA::LoadBinary` per particle or the default rotation (the four
///   words at `011a9ea4`) in each; otherwise, from `0x05000008` on, a byte
///   and, when set, the array read per particle when the word at stream +4
///   is below 0x13, and up to `0x0a000100` dropped again (deallocated and
///   cleared) unless the object is of the class with `NiRTTI` `011f5f0c`
///   (walking its `GetRTTI` base chain);
/// - from `0x0a030005` on: the rotation angles (a byte, then 4 bytes per
///   particle, read one by one when the word at stream +4 is below 0x13)
///   and the rotation axes (a byte, then 12 bytes per particle from the
///   memory manager, `NiColor::LoadBinary` each under the same condition);
/// - when the word at stream +4 is above 0x11: the texture indices (a
///   byte, then one zeroed byte per particle), the sub-texture offset count
///   (a byte) and that many `NiColorA` offsets (16 bytes each).
///
/// Stack words follow the exe: three locals (`L0` -0x0c, `L1` -8, `L2` -4
/// from the return address, flags in `L0`'s top byte) and the argument's
/// own word.
pub fn ni_particles_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(GEOMETRY_DATA_LOAD_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    let version = |e: &Engine| e.mem.u32(stream.wrapping_add(STREAM_VERSION));
    let user_version = |e: &Engine| e.mem.u32(stream.wrapping_add(STREAM_USER_VERSION));
    let count = |e: &Engine| e.mem.u16(this.wrapping_add(VERTEX_COUNT));
    // Frame: L0 +0, L1 +4, L2 +8, the argument word +0x0c.
    e.with_stack(16, |e, frame| {
        let l0 = frame.addr();
        let flag = l0 + 3;
        let l1 = l0 + 4;
        let l2 = l0 + 8;
        let argument = l0 + 0x0c;
        e.mem.set_u32(argument, stream);
        if version(e) < 0x0401_0007 {
            stream_read(e, stream, l1, 2, 2, l2);
        }
        let radii = e
            .call(NI_ALLOC, &args![u32::from(count(e)).wrapping_mul(4)])
            .u32();
        e.mem.set_u32(this.wrapping_add(RADII), radii);
        if version(e) < 0x0a00_010d {
            stream_read(e, stream, l1, 4, 4, l2);
            let mut i = 0u16;
            if i < count(e) {
                loop {
                    let radii = e.mem.u32(this.wrapping_add(RADII));
                    let value = x87_quiet_bits(e.mem.u32(l1));
                    e.mem.set_u32(radii.wrapping_add(u32::from(i) * 4), value);
                    i = i.wrapping_add(1);
                    if i >= count(e) {
                        break;
                    }
                }
            }
        } else {
            stream_read(e, stream, flag, 1, 1, l2);
            if e.mem.u8(flag) != 0 && user_version(e) < 0x13 {
                let radii = e.mem.u32(this.wrapping_add(RADII));
                e.call(READ_FLOATS, &args![stream, radii, u32::from(count(e))]);
            } else {
                fill_ones(e, this, RADII);
            }
        }
        stream_read(e, stream, this.wrapping_add(ACTIVE), 2, 2, l2);
        if version(e) < 0x0401_0007 {
            stream_read(e, stream, this.wrapping_add(SIZES), 4, 4, l2);
            let has = u8::from(e.mem.u32(this.wrapping_add(SIZES)) != 0);
            e.mem.set_u8(argument, has);
        } else {
            stream_read(e, stream, argument, 1, 1, l2);
        }
        let sizes = e
            .call(NI_ALLOC, &args![u32::from(count(e)).wrapping_mul(4)])
            .u32();
        e.mem.set_u32(this.wrapping_add(SIZES), sizes);
        if e.mem.u8(argument) != 0 && user_version(e) < 0x13 {
            let bytes = u32::from(count(e)).wrapping_mul(4);
            stream_read(e, stream, sizes, bytes, 4, l2);
        } else {
            fill_ones(e, this, SIZES);
        }
        if e.mem.u8(this.wrapping_add(OLD_ROTATING)) != 0 {
            if version(e) < 0x0401_0000 {
                stream_read(e, stream, this.wrapping_add(ROTATIONS), 4, 4, l2);
                let has = u8::from(e.mem.u32(this.wrapping_add(ROTATIONS)) != 0);
                e.mem.set_u8(flag, has);
            } else {
                stream_read(e, stream, flag, 1, 1, l2);
            }
            let rotations = allocate_elements(e, u32::from(count(e)), 16);
            e.mem.set_u32(this.wrapping_add(ROTATIONS), rotations);
            if e.mem.u8(flag) != 0 {
                let mut i = 0u32;
                if count(e) != 0 {
                    loop {
                        let rotations = e.mem.u32(this.wrapping_add(ROTATIONS));
                        e.call(
                            COLOR_A_LOAD_BINARY,
                            &args![rotations.wrapping_add(i * 16), stream],
                        );
                        i += 1;
                        if i >= u32::from(count(e)) {
                            break;
                        }
                    }
                }
            } else {
                let mut i = 0u32;
                if count(e) != 0 {
                    loop {
                        let rotations = e.mem.u32(this.wrapping_add(ROTATIONS));
                        let to = rotations.wrapping_add(i * 16);
                        copy_words(e, DEFAULT_ROTATION, to, 4);
                        i += 1;
                        if i >= u32::from(count(e)) {
                            break;
                        }
                    }
                }
            }
        } else if version(e) >= 0x0500_0008 {
            stream_read(e, stream, flag, 1, 1, l2);
            if e.mem.u8(flag) != 0 {
                let rotations = allocate_elements(e, u32::from(count(e)), 16);
                e.mem.set_u32(this.wrapping_add(ROTATIONS), rotations);
                if user_version(e) < 0x13 {
                    load_elements(e, this, stream, ROTATIONS, 16, COLOR_A_LOAD_BINARY);
                }
                if version(e) <= 0x0a00_0100 {
                    let mut rtti = e.vcall(this, SLOT_GET_RTTI, &args![]).u32();
                    let mut kept = false;
                    while rtti != 0 {
                        if rtti == ROTATIONS_KEPT_RTTI {
                            kept = true;
                            break;
                        }
                        rtti = e.mem.u32(rtti.wrapping_add(4));
                    }
                    if !kept {
                        let rotations = e.mem.u32(this.wrapping_add(ROTATIONS));
                        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, rotations]);
                        e.mem.set_u32(this.wrapping_add(ROTATIONS), 0);
                    }
                }
            }
        }
        if version(e) >= 0x0a03_0005 {
            stream_read(e, stream, flag, 1, 1, l2);
            if e.mem.u8(flag) != 0 {
                let angles = e
                    .call(NI_ALLOC, &args![u32::from(count(e)).wrapping_mul(4)])
                    .u32();
                e.mem.set_u32(this.wrapping_add(ROTATION_ANGLES), angles);
                if user_version(e) < 0x13 {
                    let mut i = 0u16;
                    if i < count(e) {
                        loop {
                            let angles = e.mem.u32(this.wrapping_add(ROTATION_ANGLES));
                            let at = angles.wrapping_add(u32::from(i) * 4);
                            stream_read(e, stream, at, 4, 4, l2);
                            i = i.wrapping_add(1);
                            if i >= count(e) {
                                break;
                            }
                        }
                    }
                }
            }
            stream_read(e, stream, l1, 1, 1, l2);
            if e.mem.u8(l1) != 0 {
                let axes = allocate_elements(e, u32::from(count(e)), 12);
                e.mem.set_u32(this.wrapping_add(ROTATION_AXES), axes);
                if user_version(e) < 0x13 {
                    load_elements(e, this, stream, ROTATION_AXES, 12, COLOR_LOAD_BINARY);
                }
            }
        }
        if user_version(e) > 0x11 {
            stream_read(e, stream, l1, 1, 1, l2);
            if e.mem.u8(l1) != 0 {
                let indices = e.call(NI_ALLOC, &args![u32::from(count(e))]).u32();
                e.mem.set_u32(this.wrapping_add(TEXTURE_INDICES), indices);
                e.call(MEMSET, &args![indices, 0u32, u32::from(count(e))]);
            }
            let offset_count = this.wrapping_add(SUB_TEXTURE_OFFSET_COUNT);
            stream_read(e, stream, offset_count, 1, 1, l2);
            let offsets = e.mem.u8(offset_count);
            if offsets != 0 {
                let array = e.call(NI_ALLOC, &args![u32::from(offsets) << 4]).u32();
                e.mem.set_u32(this.wrapping_add(SUB_TEXTURE_OFFSETS), array);
                let mut i = 0u16;
                if e.mem.u8(offset_count) != 0 {
                    loop {
                        let array = e.mem.u32(this.wrapping_add(SUB_TEXTURE_OFFSETS));
                        e.call(
                            COLOR_A_LOAD_BINARY,
                            &args![array.wrapping_add(u32::from(i) << 4), stream],
                        );
                        i = i.wrapping_add(1);
                        if i >= u16::from(e.mem.u8(offset_count)) {
                            break;
                        }
                    }
                }
            }
        }
    });
}

/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `NiGeometryData::NiGeometryData` (Xbox PDB, `00a67c00`; `thiscall`, no stack word).
const GEOMETRY_DATA_CONSTRUCTOR: u32 = 0x00a6_7c00;
/// `NiGeometryData::SetConsistency` (Xbox PDB, `thiscall`, the consistency).
const SET_CONSISTENCY: u32 = 0x00a6_7050;
/// `NiParticlesData`'s vtable (its RTTI names `NiParticlesData`).
const NI_PARTICLES_DATA_VTABLE: u32 = 0x010a_05e4;
/// The byte at +0x3c the creation functions clear after setting the consistency (a
/// `NiGeometryData` field, not named here).
const GEOMETRY_BYTE_3C: u32 = 0x3c;
/// `NiGeometryData::IsEqual`, `SaveBinary` and `GetViewerStrings` (Xbox PDB, `thiscall`,
/// one stack word each).
const GEOMETRY_DATA_IS_EQUAL: u32 = 0x00a6_7840;
const GEOMETRY_DATA_SAVE_BINARY: u32 = 0x00a6_8520;
const GEOMETRY_DATA_GET_VIEWER_STRINGS: u32 = 0x00a6_88b0;
/// `NiColorA::SaveBinary` (Xbox PDB, `thiscall` on the colour, the stream).
const COLOR_A_SAVE_BINARY: u32 = 0x00a6_98a0;
/// `NiStream::m_pkOstr` +0x250 (Xbox PDB name): the output, write function at +0xc.
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word) and its pointer (`_ov10`),
/// `unsigned short` (`_ov6`) and `unsigned char` (`_ov4`) overloads (`cdecl`, name and
/// value word).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const GET_VIEWER_STRING_USHORT: u32 = 0x00a7_d490;
const GET_VIEWER_STRING_UCHAR: u32 = 0x00a7_d410;
/// `0096ad30` (`thiscall`, new capacity) and `0096ae90` (`thiscall`, index, pointer to the
/// element): the viewer-string array's set-size and set-at.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The field names the viewer strings use: `m_pfRadii`, `m_usActive`, `m_pfSizes`,
/// `m_pkRotations`, `m_pfRotationAngles`, `m_pkRotationAxes`, `m_pucTextureIndices`,
/// `m_ucSubTextureOffsetCount`, `m_pkSubTextureOffsets`.
const NAME_RADII: u32 = 0x010a_0724;
const NAME_ACTIVE: u32 = 0x010a_0718;
const NAME_SIZES: u32 = 0x010a_070c;
const NAME_ROTATIONS: u32 = 0x010a_06fc;
const NAME_ROTATION_ANGLES: u32 = 0x010a_06e8;
const NAME_ROTATION_AXES: u32 = 0x010a_06d4;
const NAME_TEXTURE_INDICES: u32 = 0x010a_06c0;
const NAME_SUB_TEXTURE_OFFSET_COUNT: u32 = 0x010a_06a4;
const NAME_SUB_TEXTURE_OFFSETS: u32 = 0x010a_068c;

/// The creation functions' body: a new particles data (0x70 bytes from `00aa13e0`;
/// `NiGeometryData`'s constructor, the vtable, every particle array and count cleared,
/// `m_bIsOldRotatingParticlesObject` cleared, the consistency set to 0, the byte at +0x3c
/// cleared), or 0 when the allocation fails.
fn new_particles_data(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![0x70u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(GEOMETRY_DATA_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, NI_PARTICLES_DATA_VTABLE);
    e.mem.set_u32(block + RADII, 0);
    e.mem.set_u16(block + ACTIVE, 0);
    for offset in [
        SIZES,
        ROTATIONS,
        ROTATION_ANGLES,
        ROTATION_AXES,
        TEXTURE_INDICES,
    ] {
        e.mem.set_u32(block + offset, 0);
    }
    e.mem.set_u32(block + SUB_TEXTURE_OFFSETS, 0);
    e.mem.set_u8(block + SUB_TEXTURE_OFFSET_COUNT, 0);
    e.mem.set_u8(block + OLD_ROTATING, 0);
    e.call(SET_CONSISTENCY, &args![block, 0u32]);
    e.mem.set_u8(block + GEOMETRY_BYTE_3C, 0);
    block
}

// Translated from 00a96600 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; the creation function the
/// stream registers for `NiParticlesData` and `NiAutoNormalParticlesData`): a new particles
/// data ([`new_particles_data`]), or null.
pub fn fn_00a96600(e: &mut Engine) -> Ptr {
    Ptr::new(new_particles_data(e))
}

// Translated from 00a96660 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; the creation function the
/// stream registers for `NiRotatingParticlesData`): a new particles data
/// ([`new_particles_data`]) with `m_bIsOldRotatingParticlesObject` (+0x40) set. When the
/// allocation fails the exe still writes that byte at address 0x40.
pub fn fn_00a96660(e: &mut Engine) -> Ptr {
    let object = new_particles_data(e);
    e.mem.set_u8(object.wrapping_add(OLD_ROTATING), 1);
    Ptr::new(object)
}

/// Whether `count` floats at `mine` and `theirs` are all equal (`FUCOMPP` on each pair,
/// a NaN never equal), compared in order.
fn floats_equal(e: &Engine, mine: u32, theirs: u32, count: u32) -> bool {
    (0..count).all(|i| {
        let a = x87_load32(e.mem.u32(mine.wrapping_add(4 * i)));
        let b = x87_load32(e.mem.u32(theirs.wrapping_add(4 * i)));
        a == b
    })
}

// Translated from 00a96a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::IsEqual` (Xbox PDB): `NiGeometryData::IsEqual`; the radii present on
/// both or neither and, when present, equal for every vertex (the vertex count, +8); the
/// same active count; the sizes and the rotations likewise present on both or neither and
/// equal for the active particles; then, only by this object's arrays (the other's are read
/// without a check), the rotation angles, rotation axes (3 floats), texture indices (bytes)
/// and the sub-texture offsets (4 floats, this object's count +0x6c) equal.
pub fn ni_particles_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let mine = this.addr();
    let theirs = other.addr();
    if !e.call(GEOMETRY_DATA_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let radii = e.mem.u32(mine + RADII);
    if (radii != 0) != (e.mem.u32(theirs + RADII) != 0) {
        return false;
    }
    if radii != 0 {
        let vertices = u32::from(e.mem.u16(mine + VERTEX_COUNT));
        let other_radii = e.mem.u32(theirs + RADII);
        if !floats_equal(e, radii, other_radii, vertices) {
            return false;
        }
    }
    let active = e.mem.u16(mine + ACTIVE);
    if active != e.mem.u16(theirs + ACTIVE) {
        return false;
    }
    let active = u32::from(active);
    for (field, floats) in [(SIZES, 1u32), (ROTATIONS, 4)] {
        let array = e.mem.u32(mine + field);
        let other_array = e.mem.u32(theirs + field);
        if (array != 0) != (other_array != 0) {
            return false;
        }
        if array != 0 && !floats_equal(e, array, other_array, active * floats) {
            return false;
        }
    }
    for (field, floats) in [(ROTATION_ANGLES, 1u32), (ROTATION_AXES, 3)] {
        let array = e.mem.u32(mine + field);
        if array != 0 {
            let other_array = e.mem.u32(theirs + field);
            if !floats_equal(e, array, other_array, active * floats) {
                return false;
            }
        }
    }
    let indices = e.mem.u32(mine + TEXTURE_INDICES);
    if indices != 0 {
        let other_indices = e.mem.u32(theirs + TEXTURE_INDICES);
        for i in 0..active {
            if e.mem.u8(indices.wrapping_add(i)) != e.mem.u8(other_indices.wrapping_add(i)) {
                return false;
            }
        }
    }
    let offsets = e.mem.u32(mine + SUB_TEXTURE_OFFSETS);
    if offsets != 0 {
        let count = u32::from(e.mem.u8(mine + SUB_TEXTURE_OFFSET_COUNT));
        let other_offsets = e.mem.u32(theirs + SUB_TEXTURE_OFFSETS);
        if !floats_equal(e, offsets, other_offsets, count * 4) {
            return false;
        }
    }
    true
}

/// One write through the stream output's write function: `function(file, from, size,
/// count, 1)` with the word at `count` set to `count_value` first, the answer added to the
/// file's position.
fn stream_write(e: &mut Engine, stream: u32, from: u32, size: u32, count: u32, count_value: u32) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
    e.mem.set_u32(count, count_value);
    let bytes = e.call(write, &args![output, from, size, count, 1u32]).u32();
    let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        output.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

// Translated from 00a97290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::SaveBinary` (Xbox PDB): `NiGeometryData::SaveBinary`, then to the
/// stream's output: a byte saying whether there are radii (kept in the low byte of the
/// stream argument's stack word), the active count (2 bytes), one byte each for sizes,
/// rotations, rotation angles, rotation axes and texture indices being present (five stack
/// bytes), the sub-texture offset count (1 byte), and each sub-texture offset
/// (`NiColorA::SaveBinary`). The byte-count word is one local, which the offset loop then
/// keeps its offset in.
pub fn ni_particles_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let me = this.addr();
    let s = stream.addr();
    e.call(GEOMETRY_DATA_SAVE_BINARY, &args![this, stream]);
    e.with_stack(0x10, |e, frame| {
        // The frame's 0xc bytes of locals (the five flag bytes at +3 .. +7, the byte-count
        // word at +8) and the stream argument's word at +0xc.
        let base = frame.addr();
        let count = base + 8;
        let argument = base + 0xc;
        e.mem.set_u32(argument, s);
        e.mem.set_u8(argument, u8::from(e.mem.u32(me + RADII) != 0));
        stream_write(e, s, argument, 1, count, 1);
        stream_write(e, s, me + ACTIVE, 2, count, 2);
        for (byte, field) in [
            (3u32, SIZES),
            (4, ROTATIONS),
            (5, ROTATION_ANGLES),
            (6, ROTATION_AXES),
            (7, TEXTURE_INDICES),
        ] {
            let present = u8::from(e.mem.u32(me + field) != 0);
            e.mem.set_u8(base + byte, present);
            stream_write(e, s, base + byte, 1, count, 1);
        }
        stream_write(e, s, me + SUB_TEXTURE_OFFSET_COUNT, 1, count, 1);
        let mut i = 0u32;
        if e.mem.u8(me + SUB_TEXTURE_OFFSET_COUNT) != 0 {
            e.mem.set_u32(count, 0);
            loop {
                let offset = e.mem.u32(count);
                let color = e.mem.u32(me + SUB_TEXTURE_OFFSETS).wrapping_add(offset);
                e.call(COLOR_A_SAVE_BINARY, &args![color, stream]);
                let total = u32::from(e.mem.u8(me + SUB_TEXTURE_OFFSET_COUNT));
                e.mem.set_u32(count, offset.wrapping_add(0x10));
                i += 1;
                if i >= total {
                    break;
                }
            }
        }
    });
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array: when
/// the count (`m_usSize`, +0xa) is not below the capacity (`m_usMaxSize`, +8) the array first
/// grows to count + `m_usGrowBy` (+0xe, `0096ad30`); then `0096ae90` stores it at the old
/// count.
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a97420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::GetViewerStrings` (Xbox PDB): `NiGeometryData::GetViewerStrings`, then
/// ten strings appended to `strings`: the class name (the first word of
/// `NiParticlesData::ms_RTTI`, `011f5ef0`), the radii pointer, the active count (16 bits),
/// the size, rotation, rotation angle, rotation axis and texture index pointers, the
/// sub-texture offset count (a byte) and the sub-texture offset pointer. Each string pointer
/// is kept in the `strings` argument's stack word, whose address the array's set-at
/// receives.
pub fn ni_particles_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let me = this.addr();
    e.call(GEOMETRY_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_PARTICLES_DATA_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
        let items = [
            (GET_VIEWER_STRING_POINTER, NAME_RADII, RADII),
            (GET_VIEWER_STRING_USHORT, NAME_ACTIVE, ACTIVE),
            (GET_VIEWER_STRING_POINTER, NAME_SIZES, SIZES),
            (GET_VIEWER_STRING_POINTER, NAME_ROTATIONS, ROTATIONS),
            (
                GET_VIEWER_STRING_POINTER,
                NAME_ROTATION_ANGLES,
                ROTATION_ANGLES,
            ),
            (GET_VIEWER_STRING_POINTER, NAME_ROTATION_AXES, ROTATION_AXES),
            (
                GET_VIEWER_STRING_POINTER,
                NAME_TEXTURE_INDICES,
                TEXTURE_INDICES,
            ),
            (
                GET_VIEWER_STRING_UCHAR,
                NAME_SUB_TEXTURE_OFFSET_COUNT,
                SUB_TEXTURE_OFFSET_COUNT,
            ),
            (
                GET_VIEWER_STRING_POINTER,
                NAME_SUB_TEXTURE_OFFSETS,
                SUB_TEXTURE_OFFSETS,
            ),
        ];
        for (function, name, field) in items {
            let value = match function {
                GET_VIEWER_STRING_USHORT => u32::from(e.mem.u16(me + field)),
                GET_VIEWER_STRING_UCHAR => u32::from(e.mem.u8(me + field)),
                _ => e.mem.u32(me + field),
            };
            let text = e.call(function, &args![name, value]).u32();
            e.mem.set_u32(slot, text);
            append_viewer_string(e, strings, slot);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_6600, fn_00a96600() -> Ptr),
        entry!(0x00a9_6660, fn_00a96660() -> Ptr),
        entry!(0x00a9_6a30, ni_particles_data_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_7290, ni_particles_data_save_binary(Ptr, Ptr)),
        entry!(0x00a9_7420, ni_particles_data_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a967d0, ni_particles_data_get_rtti() -> Ptr),
        entry!(0x00a96a10, ni_particles_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_64c0, ni_particles_data_remove_particle(Ptr, u16)),
        entry!(0x00a9_66c0, fn_00a966c0(Ptr, i32, i32)),
        entry!(0x00a9_6910, ni_particles_data_sort_particles(Ptr, Ptr)),
        entry!(0x00a9_6cf0, ni_particles_data_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_particles_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a967d0, &args![]).u32(), NI_PARTICLES_DATA_RTTI);
    }

    #[test]
    fn ni_particles_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_6800, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a96a10, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a96a10, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x70]]);
        assert_eq!(calls_to(&e, 0x00a9_6800), vec![vec![this]; 2]);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const READ: u32 = 0x00ee_0400;
    const GET_RTTI: u32 = 0x00ee_0500;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    #[test]
    fn remove_particle_moves_the_last_particle_into_the_gap() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.mem.set_u16(this + ACTIVE, 3);
        let arrays = [
            (VERTICES, 12u32),
            (COLORS, 16),
            (NORMALS, 12),
            (RADII, 4),
            (SIZES, 4),
            (ROTATIONS, 16),
            (ROTATION_ANGLES, 4),
            (ROTATION_AXES, 12),
            (TEXTURE_INDICES, 1),
        ];
        for (field, size) in arrays {
            let array = e.mem.alloc(3 * size);
            for b in 0..3 * size {
                e.mem.set_u8(array + b, (b / size) as u8 + 1);
            }
            e.mem.set_u32(this + field, array);
        }
        // A signalling NaN radius is quieted by the x87 copy.
        let radii = e.mem.u32(this + RADII);
        e.mem.set_u32(radii + 8, 0x7f80_0001);
        e.call(0x00a9_64c0, &args![this, 0u32]);
        assert_eq!(e.mem.u16(this + ACTIVE), 2);
        for (field, size) in arrays {
            let array = e.mem.u32(this + field);
            if field == RADII {
                assert_eq!(e.mem.u32(array), 0x7fc0_0001);
            } else {
                assert_eq!(e.mem.bytes(array, size), vec![3; size as usize]);
            }
        }
        // Only the vertices are required; the last particle only counts down.
        for (field, _) in arrays[1..].iter() {
            e.mem.set_u32(this + field, 0);
        }
        e.mem.set_u16(this + ACTIVE, 3);
        e.call(0x00a9_64c0, &args![this, 1u32]);
        assert_eq!(e.mem.u16(this + ACTIVE), 2);
        let vertices = e.mem.u32(this + VERTICES);
        assert_eq!(e.mem.bytes(vertices + 12, 12), vec![3; 12]);
        e.mem.set_u8(vertices + 12, 9);
        e.call(0x00a9_64c0, &args![this, 1u32]);
        assert_eq!(e.mem.u16(this + ACTIVE), 1);
        assert_eq!(e.mem.u8(vertices + 12), 9);
    }

    /// Particles data with the given depths and the render order 0, 1, ...
    fn sorted(depths: &[f32]) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        let order = e.mem.alloc(2 * depths.len() as u32);
        let array = e.mem.alloc(4 * depths.len() as u32);
        for (i, d) in depths.iter().enumerate() {
            e.mem.set_u16(order + 2 * i as u32, i as u16);
            e.mem.set_f32(array + 4 * i as u32, *d);
        }
        e.mem.set_u32(this + RENDER_ORDER, order);
        e.mem.set_u32(this + DEPTHS, array);
        (e, this)
    }

    fn order(e: &Engine, this: u32, n: u32) -> Vec<u16> {
        let order = e.mem.u32(this + RENDER_ORDER);
        (0..n).map(|i| e.mem.u16(order + 2 * i)).collect()
    }

    #[test]
    fn fn_00a966c0_orders_far_to_near() {
        let (mut e, this) = sorted(&[1.0, 5.0, 3.0, 4.0, 2.0, 3.0]);
        e.call(0x00a9_66c0, &args![this, 0i32, 5i32]);
        let indices = order(&e, this, 6);
        let depths = e.mem.u32(this + DEPTHS);
        let values: Vec<f32> = indices
            .iter()
            .map(|&i| e.mem.f32(depths + 4 * u32::from(i)))
            .collect();
        assert_eq!(values, [5.0, 4.0, 3.0, 3.0, 2.0, 1.0]);
        // One element, and an empty range, leave the order alone.
        let (mut e, this) = sorted(&[2.0, 1.0]);
        e.call(0x00a9_66c0, &args![this, 1i32, 1i32]);
        e.call(0x00a9_66c0, &args![this, 1i32, 0i32]);
        assert_eq!(order(&e, this, 2), [0, 1]);
    }

    #[test]
    fn sort_particles_makes_the_arrays_computes_depths_and_sorts() {
        let mut e = Engine::new();
        let log = Log::default();
        let blocks = [e.mem.alloc(8), e.mem.alloc(16)];
        let l = log.clone();
        let mut next = blocks.into_iter();
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            l.borrow_mut().push((MEMORY_MANAGER_ALLOCATE, a.to_vec()));
            ret(next.next().unwrap())
        });
        let this = e.mem.alloc(0x70);
        e.mem.set_u16(this + VERTEX_COUNT, 4);
        e.mem.set_u16(this + ACTIVE, 3);
        let vertices = e.mem.alloc(36);
        for (i, v) in [[1.0f32, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]
            .iter()
            .enumerate()
        {
            for (k, c) in v.iter().enumerate() {
                e.mem.set_f32(vertices + 12 * i as u32 + 4 * k as u32, *c);
            }
        }
        e.mem.set_u32(this + VERTICES, vertices);
        let camera = e.mem.alloc(0x84);
        e.mem.set_f32(camera + 0x68, 1.0);
        e.mem.set_f32(camera + 0x74, 1.0);
        e.mem.set_f32(camera + 0x80, 1.0);
        e.call(0x00a9_6910, &args![this, camera]);
        assert_eq!(
            *log.borrow(),
            [
                (MEMORY_MANAGER_ALLOCATE, vec![MEMORY_MANAGER, 8]),
                (MEMORY_MANAGER_ALLOCATE, vec![MEMORY_MANAGER, 16]),
            ]
        );
        assert_eq!(e.mem.u32(this + RENDER_ORDER), blocks[0]);
        assert_eq!(e.mem.u32(this + DEPTHS), blocks[1]);
        assert_eq!(e.mem.f32(blocks[1]), 1.0);
        assert_eq!(e.mem.f32(blocks[1] + 4), 2.0);
        assert_eq!(e.mem.f32(blocks[1] + 8), 3.0);
        assert_eq!(order(&e, this, 3), [2, 1, 0]);
        // One active particle: only the identity order.
        e.mem.set_u16(this + ACTIVE, 1);
        e.mem.set_u16(blocks[0], 7);
        e.call(0x00a9_6910, &args![this, camera]);
        assert_eq!(e.mem.u16(blocks[0]), 0);
        assert_eq!(log.borrow().len(), 2);
    }

    /// `LoadBinary` of two particles from a stream of file `version` and
    /// user `user` whose reads hand out `words` in turn (each read fills its
    /// buffer with the word's bytes, repeated).
    fn load(
        version: u32,
        user: u32,
        old_rotating: bool,
        words: &'static [u32],
    ) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(stream + STREAM_USER_VERSION, user);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        let mut next = words.iter();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[1], a[2], e.mem.u32(a[3]), a[4]]));
            let word = next.next().unwrap().to_le_bytes();
            for b in 0..a[2] {
                e.mem.set_u8(a[1] + b, word[(b % 4) as usize]);
            }
            ret(a[2])
        });
        for address in [
            GEOMETRY_DATA_LOAD_BINARY,
            READ_FLOATS,
            COLOR_A_LOAD_BINARY,
            COLOR_LOAD_BINARY,
            MEMSET,
            MEMORY_MANAGER_DEALLOCATE,
        ] {
            logged(&mut e, &log, address, 0);
        }
        for address in [NI_ALLOC, MEMORY_MANAGER_ALLOCATE] {
            let l = log.clone();
            e.register_double(address, move |e, a| {
                l.borrow_mut().push((address, a.to_vec()));
                ret(e.mem.alloc(*a.last().unwrap() + 4))
            });
        }
        let this = e.mem.alloc(0x70);
        e.mem.set_u16(this + VERTEX_COUNT, 2);
        e.mem.set_u8(this + OLD_ROTATING, u8::from(old_rotating));
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + SLOT_GET_RTTI, GET_RTTI);
        e.mem.set_u32(this, vtable);
        let rtti = e.mem.alloc(8);
        let base = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, base);
        logged(&mut e, &log, GET_RTTI, rtti);
        e.map(DEFAULT_ROTATION, 0x10);
        for k in 0..4 {
            e.mem.set_f32(DEFAULT_ROTATION + 4 * k, k as f32);
        }
        e.call(0x00a9_6cf0, &args![this, stream]);
        (e, log, this, stream)
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn load_binary_reads_every_optional_array_of_new_files() {
        let words = &[
            1,
            2,
            1,
            0x3f00_0000,
            1,
            1,
            0x4000_0000,
            0x4040_0000,
            1,
            1,
            1,
        ];
        let (e, log, this, stream) = load(0x1402_0007, 0x12, false, words);
        assert_eq!(e.mem.u16(this + ACTIVE), 2);
        let radii = e.mem.u32(this + RADII);
        assert_eq!(calls(&log, READ_FLOATS), [vec![stream, radii, 2]]);
        let sizes = e.mem.u32(this + SIZES);
        assert_eq!(e.mem.f32(sizes + 4), 0.5);
        let rotations = e.mem.u32(this + ROTATIONS);
        assert_eq!(
            calls(&log, COLOR_A_LOAD_BINARY)[..2],
            [vec![rotations, stream], vec![rotations + 16, stream]]
        );
        let angles = e.mem.u32(this + ROTATION_ANGLES);
        assert_eq!(e.mem.f32(angles), 2.0);
        assert_eq!(e.mem.f32(angles + 4), 3.0);
        let axes = e.mem.u32(this + ROTATION_AXES);
        assert_eq!(
            calls(&log, COLOR_LOAD_BINARY),
            [vec![axes, stream], vec![axes + 12, stream]]
        );
        let indices = e.mem.u32(this + TEXTURE_INDICES);
        assert_eq!(calls(&log, MEMSET), [vec![indices, 0, 2]]);
        assert_eq!(e.mem.u8(this + SUB_TEXTURE_OFFSET_COUNT), 1);
        let offsets = e.mem.u32(this + SUB_TEXTURE_OFFSETS);
        assert_eq!(calls(&log, COLOR_A_LOAD_BINARY)[2], [offsets, stream]);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_ALLOCATE),
            [vec![MEMORY_MANAGER, 32], vec![MEMORY_MANAGER, 24]]
        );
        assert_eq!(
            calls(&log, NI_ALLOC),
            [vec![8], vec![8], vec![8], vec![2], vec![16]]
        );
        let reads = calls(&log, READ);
        assert_eq!(reads.len(), 11);
        assert_eq!(reads[3][1..], [8, 4, 1]);
        assert!(calls(&log, GET_RTTI).is_empty());
    }

    #[test]
    fn load_binary_fills_defaults_for_old_files() {
        let words = &[0, 0x40a0_0000, 2, 0, 0];
        let (e, log, this, _) = load(0x0400_0002, 0, true, words);
        let radii = e.mem.u32(this + RADII);
        assert_eq!(e.mem.f32(radii), 5.0);
        assert_eq!(e.mem.f32(radii + 4), 5.0);
        let sizes = e.mem.u32(this + SIZES);
        assert_eq!(e.mem.f32(sizes), 1.0);
        assert_eq!(e.mem.f32(sizes + 4), 1.0);
        let rotations = e.mem.u32(this + ROTATIONS);
        assert_eq!(e.mem.f32(rotations + 16 + 12), 3.0);
        let reads = calls(&log, READ);
        assert_eq!(
            reads.iter().map(|r| r[1]).collect::<Vec<_>>(),
            [2, 4, 2, 4, 4]
        );
        assert!(calls(&log, COLOR_A_LOAD_BINARY).is_empty());
    }

    #[test]
    fn load_binary_drops_rotations_of_other_classes_in_mid_files() {
        let words = &[0x3f80_0000, 2, 0, 1];
        let (e, log, this, _) = load(0x0a00_0100, 0, false, words);
        assert_eq!(calls(&log, COLOR_A_LOAD_BINARY).len(), 2);
        assert_eq!(calls(&log, GET_RTTI).len(), 1);
        assert_eq!(calls(&log, MEMORY_MANAGER_DEALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(this + ROTATIONS), 0);
        let sizes = e.mem.u32(this + SIZES);
        assert_eq!(e.mem.f32(sizes), 1.0);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for i in 0..a[0] {
                e.mem.set_u8(block + i, 0xcd);
            }
            block.into_ret()
        });
        e.register(GEOMETRY_DATA_CONSTRUCTOR, |_, _| Ret::default());
        e.register(SET_CONSISTENCY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn creation_functions_build_empty_data() {
        let mut e = engine();
        let data = e.call(0x00a9_6600, &args![]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x70]]);
        assert_eq!(calls_to(&e, GEOMETRY_DATA_CONSTRUCTOR), vec![vec![data]]);
        assert_eq!(calls_to(&e, SET_CONSISTENCY), vec![vec![data, 0]]);
        assert_eq!(e.mem.u32(data), NI_PARTICLES_DATA_VTABLE);
        assert_eq!(e.mem.bytes(data + 0x4c, 6), vec![0; 6]);
        assert_eq!(e.mem.bytes(data + 0x54, 0x19), vec![0; 0x19]);
        assert_eq!(e.mem.u8(data + 0x40), 0);
        assert_eq!(e.mem.u8(data + 0x3c), 0);
        // The padding after the 16-bit active count is left as allocated.
        assert_eq!(e.mem.u16(data + 0x52), 0xcdcd);
        let rotating = e.call(0x00a9_6660, &args![]).u32();
        assert_eq!(e.mem.u8(rotating + 0x40), 1);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_6600, &args![]).u32(), 0);
    }

    /// A particles data with 2 vertices, 2 active particles and every array filled with
    /// `seed`-based values.
    fn data(e: &mut Engine, seed: f32) -> u32 {
        let this = e.mem.alloc(0x70);
        e.mem.set_u16(this + VERTEX_COUNT, 2);
        e.mem.set_u16(this + ACTIVE, 2);
        let floats = |e: &mut Engine, n: u32| {
            let a = e.mem.alloc(4 * n);
            for i in 0..n {
                e.mem.set_f32(a + 4 * i, seed + i as f32);
            }
            a
        };
        for (field, n) in [
            (RADII, 2u32),
            (SIZES, 2),
            (ROTATIONS, 8),
            (ROTATION_ANGLES, 2),
            (ROTATION_AXES, 6),
            (SUB_TEXTURE_OFFSETS, 4),
        ] {
            let array = floats(e, n);
            e.mem.set_u32(this + field, array);
        }
        let indices = e.mem.alloc(2);
        e.mem.set_u8(indices + 1, 7);
        e.mem.set_u32(this + TEXTURE_INDICES, indices);
        e.mem.set_u8(this + SUB_TEXTURE_OFFSET_COUNT, 1);
        this
    }

    #[test]
    fn is_equal_compares_every_array() {
        let mut e = engine();
        e.register(GEOMETRY_DATA_IS_EQUAL, |_, _| 1u32.into_ret());
        let a = data(&mut e, 1.0);
        let b = data(&mut e, 1.0);
        assert!(e.call(0x00a9_6a30, &args![a, b]).bool());
        // A last element differing in each array.
        for (field, last) in [
            (RADII, 4u32),
            (SIZES, 4),
            (ROTATIONS, 28),
            (ROTATION_ANGLES, 4),
            (ROTATION_AXES, 20),
            (SUB_TEXTURE_OFFSETS, 12),
        ] {
            let b = data(&mut e, 1.0);
            let array = e.mem.u32(b + field);
            e.mem.set_f32(array + last, 99.0);
            assert!(!e.call(0x00a9_6a30, &args![a, b]).bool(), "{field:x}");
        }
        let c = data(&mut e, 1.0);
        let indices = e.mem.u32(c + TEXTURE_INDICES);
        e.mem.set_u8(indices + 1, 8);
        assert!(!e.call(0x00a9_6a30, &args![a, c]).bool());
        // Presence must match for radii, sizes and rotations.
        for field in [RADII, SIZES, ROTATIONS] {
            let d = data(&mut e, 1.0);
            e.mem.set_u32(d + field, 0);
            assert!(!e.call(0x00a9_6a30, &args![a, d]).bool(), "{field:x}");
            assert!(!e.call(0x00a9_6a30, &args![d, a]).bool(), "{field:x}");
        }
        // Without its own rotation angles the other's are not looked at.
        let f = data(&mut e, 1.0);
        e.mem.set_u32(f + ROTATION_ANGLES, 0);
        assert!(e.call(0x00a9_6a30, &args![f, a]).bool());
        // A NaN radius is never equal; a different active count or base fails too.
        let g = data(&mut e, 1.0);
        e.mem.set_u16(g + ACTIVE, 1);
        assert!(!e.call(0x00a9_6a30, &args![a, g]).bool());
        let h = data(&mut e, f32::NAN);
        assert!(!e.call(0x00a9_6a30, &args![h, h]).bool());
        e.register(GEOMETRY_DATA_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_6a30, &args![a, b]).bool());
    }

    type Writes = Rc<RefCell<Vec<(u32, u32, u32, u32, u32)>>>;

    #[test]
    fn save_binary_writes_the_presence_bytes_and_offsets() {
        let mut e = engine();
        e.register(GEOMETRY_DATA_SAVE_BINARY, |_, _| Ret::default());
        e.register(COLOR_A_SAVE_BINARY, |_, _| Ret::default());
        let log: Writes = Rc::default();
        let writes = log.clone();
        e.register_double(0x0700_000c, move |e, a| {
            let value = if a[2] == 1 {
                u32::from(e.mem.u8(a[1]))
            } else {
                u32::from(e.mem.u16(a[1]))
            };
            writes
                .borrow_mut()
                .push((a[1], a[2], a[3], e.mem.u32(a[3]), value));
            a[2].into_ret()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, 0x0700_000c);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = data(&mut e, 1.0);
        e.mem.set_u32(this + ROTATIONS, 0);
        e.mem.set_u8(this + SUB_TEXTURE_OFFSET_COUNT, 2);
        e.call(0x00a9_7290, &args![this, stream]);
        let log = log.borrow();
        let values: Vec<(u32, u32)> = log.iter().map(|w| (w.1, w.4)).collect();
        assert_eq!(
            values,
            vec![
                (1, 1),
                (2, 2),
                (1, 1),
                (1, 0),
                (1, 1),
                (1, 1),
                (1, 1),
                (1, 2)
            ]
        );
        assert_eq!(log[1].0, this + ACTIVE);
        assert_eq!(log[7].0, this + SUB_TEXTURE_OFFSET_COUNT);
        assert!(log.iter().all(|w| w.2 == log[0].2 && w.3 == w.1));
        let offsets = e.mem.u32(this + SUB_TEXTURE_OFFSETS);
        assert_eq!(
            calls_to(&e, COLOR_A_SAVE_BINARY),
            vec![vec![offsets, stream], vec![offsets + 0x10, stream]]
        );
        assert_eq!(e.mem.u32(output + 4), 9);
    }

    #[test]
    fn viewer_strings_list_the_arrays() {
        let mut e = engine();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_PARTICLES_DATA_RTTI, 0x011f_5100);
        e.register(GEOMETRY_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        for function in [
            GET_VIEWER_STRING_POINTER,
            GET_VIEWER_STRING_USHORT,
            GET_VIEWER_STRING_UCHAR,
        ] {
            e.register(function, |_, _| 0x6000u32.into_ret());
        }
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 16);
        let this = data(&mut e, 1.0);
        e.call(0x00a9_7420, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x011f_5100]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_USHORT),
            vec![vec![NAME_ACTIVE, 2]]
        );
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_UCHAR),
            vec![vec![NAME_SUB_TEXTURE_OFFSET_COUNT, 1]]
        );
        let pointers = calls_to(&e, GET_VIEWER_STRING_POINTER);
        assert_eq!(pointers.len(), 7);
        assert_eq!(pointers[0], vec![NAME_RADII, e.mem.u32(this + RADII)]);
        assert_eq!(
            pointers[6],
            vec![
                NAME_SUB_TEXTURE_OFFSETS,
                e.mem.u32(this + SUB_TEXTURE_OFFSETS)
            ]
        );
        assert_eq!(e.mem.u16(strings + 0xa), 10);
    }
}
