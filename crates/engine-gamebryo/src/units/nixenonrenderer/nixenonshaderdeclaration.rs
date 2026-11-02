//! `NiXenonRenderer/nixenonshaderdeclaration.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The base constructor `00e78840` (engine map unit `nixenonshaderdeclaration.cpp`, unnamed;
/// `thiscall`, no stack words) and the initializer `00e78c60` (unit
/// `nid3dshaderdeclaration.cpp`, unnamed; `thiscall`: renderer, entry count, stream count;
/// answers a byte).
const BASE_CONSTRUCT_00E78840: u32 = 0x00e7_8840;
const INITIALIZE_00E78C60: u32 = 0x00e7_8c60;
/// The declaration's vtable (`010ee70c`; its RTTI type descriptor reads
/// `.?AVNiDX9ShaderDeclaration@@`).
const DECLARATION_VTABLE: u32 = 0x010e_e70c;

// Translated from 00e76700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonShaderDeclaration::Create` (Xbox PDB), `cdecl` (renderer, entry count, stream
/// count). Answers 0 when the stream count is above the renderer's word at +0x6e8 (unsigned).
/// Otherwise allocates 0x38 bytes (0 when that fails), runs the base constructor `00e78840`,
/// stores the vtable `010ee70c`, clears the words +0x2c and +0x30 and the byte +0x34, and runs
/// the initializer `00e78c60` with the three arguments; when it answers a clear byte the
/// declaration is deleted through its vtable slot 0 with 1 and 0 is answered, else the
/// declaration.
pub fn ni_xenon_shader_declaration_create(
    e: &mut Engine,
    renderer: Ptr,
    entry_count: u32,
    stream_count: u32,
) -> Ptr {
    if stream_count > e.mem.u32(renderer.addr() + 0x6e8) {
        return Ptr::new(0);
    }
    let declaration = e.call(NI_ALLOCATE, &args![0x38u32]).u32();
    if declaration == 0 {
        return Ptr::new(0);
    }
    e.call(BASE_CONSTRUCT_00E78840, &args![declaration]);
    e.mem.set_u32(declaration, DECLARATION_VTABLE);
    e.mem.set_u32(declaration + 0x2c, 0);
    e.mem.set_u32(declaration + 0x30, 0);
    e.mem.set_u8(declaration + 0x34, 0);
    let ready = e
        .call(
            INITIALIZE_00E78C60,
            &args![declaration, renderer, entry_count, stream_count],
        )
        .bool();
    if !ready {
        e.vcall(declaration, 0, &args![1u32]);
        return Ptr::new(0);
    }
    Ptr::new(declaration)
}

/// The base constructor `00ee88b0` (no unit in the engine map; `thiscall`, no stack words)
/// and the `NiD3DShaderDeclaration` vtable (`010ee7cc`; RTTI
/// `.?AVNiD3DShaderDeclaration@@`).
const SHADER_DECLARATION_BASE_00EE88B0: u32 = 0x00ee_88b0;
const D3D_DECLARATION_VTABLE_010EE7CC: u32 = 0x010e_e7cc;

// Translated from 00e78840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`): the
/// `NiD3DShaderDeclaration` constructor (the vtable it stores has that class's RTTI),
/// `thiscall`, no stack words. Runs the base constructor `00ee88b0`, clears the words +0x8 to
/// +0x24 (`m_pkRenderer` to `m_pkStreamEntries`, Xbox PDB) and the byte +0x28 (`m_bModified`),
/// stores the vtable `010ee7cc` and answers `this`.
pub fn fn_00e78840(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(SHADER_DECLARATION_BASE_00EE88B0, &args![base]);
    for offset in (0x8u32..=0x24).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x28, 0);
    e.mem.set_u32(base, D3D_DECLARATION_VTABLE_010EE7CC);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_6700, ni_xenon_shader_declaration_create(Ptr, u32, u32) -> Ptr),
        entry!(0x00e7_8840, fn_00e78840(Ptr) -> Ptr),
        entry!(0x00e7_6350, fn_00e76350(Ptr, u32, u32, u32, u32, u32) -> bool),
        entry!(
            0x00e7_6450,
            fn_00e76450(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x00e7_65e0, fn_00e765e0(Ptr, u32) -> u32),
        entry!(0x00e7_6630, fn_00e76630(Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00e7_6770,
            fn_00e76770(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e7_6a80, fn_00e76a80(Ptr) -> u32),
        entry!(0x00e7_8510, fn_00e78510(Ptr, u32) -> Ptr),
        entry!(0x00e7_6500, fn_00e76500(Ptr, u32, u32) -> bool),
        entry!(0x00e7_6570, fn_00e76570(Ptr, u32) -> bool),
        entry!(0x00e7_6610, fn_00e76610(Ptr, u8)),
        entry!(0x00e7_6620, fn_00e76620(Ptr) -> u8),
    ]
}

/// Marks one 0x1c-byte entry unused: its words +8 and +0xc become `0xffffffff` and 0x11.
fn clear_entry(e: &mut Engine, at: u32) {
    e.mem.set_u32(at.wrapping_add(8), 0xffff_ffff);
    e.mem.set_u32(at.wrapping_add(0xc), 0x11);
}

/// After an entry change: `m_bModified` (+0x28) set and the Direct3D declaration
/// (`m_hVertexDecl`, +0x30) released (its slot +8) and cleared when there is one.
fn declaration_modified(e: &mut Engine, base: u32) {
    let declaration = e.mem.u32(base.wrapping_add(0x30));
    e.mem.set_u8(base.wrapping_add(0x28), 1);
    if declaration != 0 {
        e.vcall(declaration, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x30), 0);
    }
}

// Translated from 00e76500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010ee70c` slot +0x90, `RemoveEntry`'s in the Xbox PDB;
/// `thiscall`, two words): `false` for an entry at or past the entry count
/// (`m_uiMaxStreamEntryCount`, +0x1c) or a stream at or past the stream count (+0x20).
/// Otherwise the entry (0x1c bytes, in the entries at +8 of the 16-byte stream at +0x24) is
/// marked unused (+8 `0xffffffff`, +0xc 0x11), the declaration marked modified (its Direct3D
/// declaration released) and the result is `true`.
pub fn fn_00e76500(e: &mut Engine, this: Ptr, entry: u32, stream: u32) -> bool {
    let base = this.addr();
    if entry >= e.mem.u32(base + 0x1c) || stream >= e.mem.u32(base + 0x20) {
        return false;
    }
    let streams = e.mem.u32(base + 0x24);
    let entries = e.mem.u32(streams.wrapping_add(stream << 4).wrapping_add(8));
    clear_entry(e, entries.wrapping_add(entry.wrapping_mul(0x1c)));
    declaration_modified(e, base);
    true
}

// Translated from 00e76570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010ee70c` slot +0x9c, `ClearAllEntries`' in the Xbox
/// PDB; `thiscall`, one word): `false` for a stream at or past the stream count (+0x20).
/// Otherwise every entry of the stream (up to the entry count +0x1c, re-read each time) is
/// marked unused as in `fn_00e76500`, the declaration marked modified and the result is
/// `true`.
pub fn fn_00e76570(e: &mut Engine, this: Ptr, stream: u32) -> bool {
    let base = this.addr();
    if stream >= e.mem.u32(base + 0x20) {
        return false;
    }
    let record = (stream << 4).wrapping_add(e.mem.u32(base + 0x24));
    let mut index = 0u32;
    let mut offset = 0u32;
    while index < e.mem.u32(base + 0x1c) {
        let at = e.mem.u32(record.wrapping_add(8)).wrapping_add(offset);
        index += 1;
        clear_entry(e, at);
        offset = offset.wrapping_add(0x1c);
    }
    declaration_modified(e, base);
    true
}

// Translated from 00e76610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010ee70c` slot +0xa4, `SetSoftwareVertexProcessing`'s
/// in the Xbox PDB): stores the byte at +0x34 (a PC-only field; the Xbox class ends at
/// +0x34).
pub fn fn_00e76610(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x34, value);
}

// Translated from 00e76620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010ee70c` slot +0xa8, `GetSoftwareVertexProcessing`'s
/// in the Xbox PDB): the byte at +0x34.
pub fn fn_00e76620(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x34)
}

// Translated from 00e76350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`), `thiscall` with five
/// words: the stream, the entry, the parameter kind and two words passed on. The kind picks a
/// declaration type and usage index: 0 to 3 themselves, 4 type 10, 5 type 5, 6 to 12 type 5 with
/// index 1 to 7, 13 type 6, 14 type 7; any other kind answers 0 (only AL is cleared). The
/// declaration's slot 0x94 then runs (`thiscall`: the fifth word, the stream, the entry, the
/// kind, the fourth word, the type, the index and 0) and its answer is passed on.
pub fn fn_00e76350(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    entry: u32,
    kind: u32,
    fourth: u32,
    fifth: u32,
) -> bool {
    let (declaration_type, index) = match kind {
        0..=3 => (kind, 0u32),
        4 => (10, 0),
        5 => (5, 0),
        6..=12 => (5, kind - 5),
        13 => (6, 0),
        14 => (7, 0),
        _ => return false,
    };
    e.vcall(
        this.addr(),
        0x94,
        &args![
            fifth,
            stream,
            entry,
            kind,
            fourth,
            declaration_type,
            index,
            0u32
        ],
    )
    .bool()
}

// Translated from 00e76450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`): a stream entry's set,
/// `thiscall` with eight words: the stream, the entry, a word never read and the five values
/// for the entry's +8, +0xc, +0x14, +0x18 and +0x10. 0 for an entry at or past `this`'s entry
/// count (+0x1c) or a stream at or past the stream count (+0x20). When any value differs from
/// the 0x1c-byte entry's (stream +8 of the 16-byte stream at +0x24), all five are stored (with
/// +8 equal to 1 and +0xc not 0x10, +0xc becomes 2), the modified byte +0x28 is set and the
/// built declaration (+0x30) released (slot 8, `stdcall`) and cleared. Answers 1.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e76450(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    entry: u32,
    _unused_3: u32,
    usage: u32,
    kind: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
) -> bool {
    let base = this.addr();
    if entry >= e.mem.u32(base.wrapping_add(0x1c)) {
        return false;
    }
    if stream >= e.mem.u32(base.wrapping_add(0x20)) {
        return false;
    }
    let streams = e.mem.u32(base.wrapping_add(0x24));
    let entries = e
        .mem
        .u32(stream.wrapping_shl(4).wrapping_add(streams).wrapping_add(8));
    let at = entries.wrapping_add(entry.wrapping_mul(0x1c));
    let same = e.mem.u32(at.wrapping_add(8)) == usage
        && e.mem.u32(at.wrapping_add(0xc)) == kind
        && e.mem.u32(at.wrapping_add(0x14)) == third
        && e.mem.u32(at.wrapping_add(0x18)) == fourth
        && e.mem.u32(at.wrapping_add(0x10)) == fifth;
    if same {
        return true;
    }
    e.mem.set_u32(at.wrapping_add(8), usage);
    e.mem.set_u32(at.wrapping_add(0xc), kind);
    e.mem.set_u32(at.wrapping_add(0x14), third);
    e.mem.set_u32(at.wrapping_add(0x18), fourth);
    e.mem.set_u32(at.wrapping_add(0x10), fifth);
    if usage == 1 && kind != 0x10 {
        e.mem.set_u32(at.wrapping_add(0xc), 2);
    }
    let built = e.mem.u32(base.wrapping_add(0x30));
    e.mem.set_u8(base.wrapping_add(0x28), 1);
    if built != 0 {
        e.vcall(built, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x30), 0);
    }
    true
}

// Translated from 00e765e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`), `thiscall` with a stream:
/// 0 at or past the stream count (+0x20); otherwise the declaration's slot 0xb4 runs (it builds
/// the declaration) and the stream's +0xc (16-byte streams at +0x24) is answered.
pub fn fn_00e765e0(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    let base = this.addr();
    if stream >= e.mem.u32(base.wrapping_add(0x20)) {
        return 0;
    }
    e.vcall(base, 0xb4, &args![]);
    let streams = e.mem.u32(base.wrapping_add(0x24));
    e.mem.u32(
        streams
            .wrapping_add(stream.wrapping_shl(4))
            .wrapping_add(0xc),
    )
}

// Translated from 00e76630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`), `thiscall` with the
/// addresses of two words: the first gets what the declaration's slot 0xb4 answers (the built
/// declaration), the second the stream count (+0x20); answers whether the first is not null.
pub fn fn_00e76630(e: &mut Engine, this: Ptr, declaration: Ptr, streams: Ptr) -> bool {
    let base = this.addr();
    let built = e.vcall(base, 0xb4, &args![]).u32();
    e.mem.set_u32(declaration.addr(), built);
    let count = e.mem.u32(base.wrapping_add(0x20));
    e.mem.set_u32(streams.addr(), count);
    e.mem.u32(declaration.addr()) != 0
}

/// What `00e76770` calls: the buffer data's update `00e8f040` (unit `nigeometrybufferdata.cpp`;
/// `thiscall`, one word), the vertex buffer manager's (+0xc) chip allocation check `00e8bfa0`
/// (unnamed; `thiscall`: buffer data, stream; answers a byte), lock `00e8bff0` (unnamed;
/// `thiscall`, four words; answers the locked address) and unlock `00e8c0d0` (unnamed;
/// `thiscall`, the vertex buffer; answers a byte), the stream's chip maker `00e69b40` (unit
/// `nixenonrenderer.cpp`; `thiscall` on the buffer data, the stream), the entry packer
/// `00e7a200` (unit `nid3dshaderdeclaration.cpp`; `thiscall`, nine words) and the geometry
/// data's pack of the derived vectors `00a855f0` (unnamed; `thiscall`, eight words).
const BUFFER_DATA_UPDATE_00E8F040: u32 = 0x00e8_f040;
const CHIP_CHECK_00E8BFA0: u32 = 0x00e8_bfa0;
const LOCK_00E8BFF0: u32 = 0x00e8_bff0;
const UNLOCK_00E8C0D0: u32 = 0x00e8_c0d0;
const NEW_CHIP_00E69B40: u32 = 0x00e6_9b40;
const PACK_ENTRY_00E7A200: u32 = 0x00e7_a200;
const PACK_DERIVED_00A855F0: u32 = 0x00a8_55f0;

/// What an entry of the declaration does in `00e76770`, by its usage: the bit of the flags
/// that enables it and, for the derived vectors, the local slot the address goes to when the
/// geometry data packs them itself.
enum Usage {
    Pack,
    Bit(u8),
    Derived(u8, u32),
}

/// The usage's case in the code's byte table (`00e76a68`) and jump table (`00e76a4c`).
fn usage_case(usage: u32) -> Usage {
    match usage {
        0 => Usage::Derived(1, 0x18),
        3 => Usage::Derived(2, 0x14),
        4 => Usage::Bit(4),
        5..=12 => Usage::Bit(8),
        13 => Usage::Derived(2, 0xc),
        14 => Usage::Derived(2, 0x10),
        _ => Usage::Pack,
    }
}

// Translated from 00e76770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`): packs one stream of a
/// geometry, `thiscall` with nine words: the geometry data, a second geometry word (the
/// geometry data that packs the derived vectors itself; never read otherwise), the skin
/// partition, the flags, the chip, the stream, a locked address, and two words for the
/// packers. The vertex count is the partition's 16-bit +0x1c (without one, the data's slot
/// 0x90); with none the chip is answered. The declaration's slot 0xb0 must answer set (it
/// gives a word for `00e8f040` and another), else 0. The buffer data (the partition's +0x28,
/// else the data's +0x34) is updated (`00e8f040`), takes the stream's stride (stream +0xc) and,
/// with the renderer's byte +0x5d4 set, the byte the declaration's slot 0xa8 answers at +0x14.
/// With a locked address that chip is used as is; otherwise the chip is kept when its vertex
/// buffer (+8) is vertex data (`GetDesc`, slot 0x34, format 100) big enough for the count times
/// the declaration's slot 0xa0 for the stream, else a new one is made (`00e8bfa0` must answer
/// set, else 0; `00e69b40`; the flags become 0xfff) and in either case locked (`00e8bff0`).
/// Each declaration entry of the stream is then packed (`00e7a200`) at the locked address plus
/// its offset, the derived vectors (usages 0, 3, 13, 14) only when their flag bit is set and,
/// with the second geometry word set and no partition, kept for `00a855f0` instead, which then
/// packs them; usages 4 and 5 to 12 need the flag bits 4 and 8. Without a locked address the
/// chip's vertex buffer is unlocked (`00e8c0d0`, 0 when it answers 0). Answers the chip.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e76770(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    derived: u32,
    partition: u32,
    flags: u32,
    chip: u32,
    stream: u32,
    locked: u32,
    eighth: u32,
    ninth: u32,
) -> u32 {
    let base = this.addr();
    let count = if partition != 0 {
        u32::from(e.mem.u16(partition.wrapping_add(0x1c)))
    } else {
        e.vcall(data, 0x90, &args![]).u32() & 0xffff
    };
    if count == 0 {
        return chip;
    }
    // The locals passed by address: the two words of slot 0xb0 (+0, +4) and the vertex
    // buffer description (+8: format, +0x18: size).
    e.with_stack(0x20, |e, locals| {
        let locals = locals.addr();
        let first = locals;
        let second = locals + 4;
        let description = locals + 8;
        if !e.vcall(base, 0xb0, &args![first, second]).bool() {
            return 0;
        }
        let buffer_data = if partition != 0 {
            e.mem.u32(partition.wrapping_add(0x28))
        } else {
            e.mem.u32(data.wrapping_add(0x34))
        };
        let word = e.mem.u32(first);
        e.call(BUFFER_DATA_UPDATE_00E8F040, &args![buffer_data, word]);
        if stream < e.mem.u32(buffer_data.wrapping_add(0x20)) {
            let streams = e.mem.u32(base.wrapping_add(0x24));
            let stride = e.mem.u32(
                stream
                    .wrapping_shl(4)
                    .wrapping_add(streams)
                    .wrapping_add(0xc),
            );
            let strides = e.mem.u32(buffer_data.wrapping_add(0x24));
            e.mem
                .set_u32(strides.wrapping_add(stream.wrapping_mul(4)), stride);
        }
        let renderer = e.mem.u32(base.wrapping_add(8));
        if e.mem.u8(renderer.wrapping_add(0x5d4)) != 0 {
            let byte = e.vcall(base, 0xa8, &args![]).u32() as u8;
            e.mem.set_u8(buffer_data.wrapping_add(0x14), byte);
        }
        let mut unlock = true;
        let mut flags = flags;
        let mut result = chip;
        let mut address = locked;
        if locked != 0 {
            unlock = false;
        } else {
            let mut keep = false;
            if chip != 0 {
                let buffer = e.mem.u32(chip.wrapping_add(8));
                if buffer != 0 {
                    e.vcall(buffer, 0x34, &args![description]);
                    if e.mem.u32(description) == 0x64 {
                        let per_vertex = e.vcall(base, 0xa0, &args![stream]).u32();
                        if e.mem.u32(description + 0x10) >= per_vertex.wrapping_mul(count) {
                            keep = true;
                        }
                    }
                }
            }
            if !keep {
                let manager = e.mem.u32(base.wrapping_add(0xc));
                if !e
                    .call(CHIP_CHECK_00E8BFA0, &args![manager, buffer_data, stream])
                    .bool()
                {
                    return 0;
                }
                result = e.call(NEW_CHIP_00E69B40, &args![buffer_data, stream]).u32();
                flags = 0xfff;
            }
            let manager = e.mem.u32(base.wrapping_add(0xc));
            let buffer = e.mem.u32(result.wrapping_add(8));
            let offset = e.mem.u32(result.wrapping_add(0xc));
            let size = e.mem.u32(result.wrapping_add(0x14));
            let other = e.mem.u32(result.wrapping_add(0x10));
            address = e
                .call(LOCK_00E8BFF0, &args![manager, buffer, offset, size, other])
                .u32();
        }
        let packs_itself = derived != 0 && partition == 0;
        let stream_entry = e
            .mem
            .u32(base.wrapping_add(0x24))
            .wrapping_add(stream.wrapping_shl(4));
        // The derived vectors' addresses, by their local slot (+0xc, +0x10, +0x14, +0x18).
        let mut kept = [0u32; 4];
        if stream_entry != 0 {
            let mut index = 0u32;
            let mut offset = 0u32;
            while index < e.mem.u32(base.wrapping_add(0x1c)) {
                let entries = e.mem.u32(stream_entry.wrapping_add(8));
                let at = entries.wrapping_add(offset);
                let target = e.mem.u32(at.wrapping_add(4)).wrapping_add(address);
                let pack = match usage_case(e.mem.u32(at.wrapping_add(8))) {
                    Usage::Pack => true,
                    Usage::Bit(bit) => flags as u8 & bit != 0,
                    Usage::Derived(bit, slot) => {
                        if flags as u8 & bit == 0 {
                            false
                        } else if packs_itself {
                            kept[((slot - 0xc) / 4) as usize] = target;
                            false
                        } else {
                            true
                        }
                    }
                };
                if pack {
                    let step = e.mem.u32(stream_entry.wrapping_add(0xc));
                    e.call(
                        PACK_ENTRY_00E7A200,
                        &args![
                            base, count, data, derived, partition, at, step, target, eighth, ninth
                        ],
                    );
                }
                index += 1;
                offset = offset.wrapping_add(0x1c);
            }
        }
        if packs_itself {
            let step = e.mem.u32(stream_entry.wrapping_add(0xc));
            let vertices = e.mem.u32(data.wrapping_add(0x20));
            let normals = e.mem.u32(data.wrapping_add(0x24));
            let vertex_count = u32::from(e.mem.u16(data.wrapping_add(8)));
            e.call(
                PACK_DERIVED_00A855F0,
                &args![
                    derived,
                    vertices,
                    normals,
                    vertex_count,
                    kept[3],
                    kept[2],
                    kept[1],
                    kept[0],
                    step
                ],
            );
        }
        // Diagnostic only (`NV_HEAD_CHECK`, not exe behaviour): reads what was packed.
        if stream_entry != 0
            && super::run_00e6fbb0::head_check_enabled()
            && super::run_00e6fbb0::head_check_is_head_data(data)
        {
            super::run_00e6fbb0::head_pack_check(
                e,
                data,
                partition,
                stream,
                e.mem.u32(stream_entry.wrapping_add(8)),
                e.mem.u32(base.wrapping_add(0x1c)),
                e.mem.u32(stream_entry.wrapping_add(0xc)),
                address,
                count,
                result,
                flags,
            );
        }
        if unlock {
            let manager = e.mem.u32(base.wrapping_add(0xc));
            let buffer = e.mem.u32(result.wrapping_add(8));
            if !e.call(UNLOCK_00E8C0D0, &args![manager, buffer]).bool() {
                return 0;
            }
        }
        result
    })
}

/// The memory manager's allocate and free (`00aa3e40` and `00aa4060`, `thiscall` on
/// `011f6238`, one word) and the element adder `00e76660` (unnamed; `thiscall`: the entry and
/// the stream; answers the bytes it covers).
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const ADD_ELEMENT_00E76660: u32 = 0x00e7_6660;

/// Whether a declaration entry is in use: its +8 is not -1 and its type +0xc not 0x11.
fn entry_used(e: &Engine, entry: u32) -> bool {
    e.mem.u32(entry.wrapping_add(8)) != 0xffff_ffff && e.mem.u32(entry.wrapping_add(0xc)) != 0x11
}

// Translated from 00e76a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`): the declaration's build,
/// `thiscall`, no stack words. Without the modified byte (+0x28) answers the built declaration
/// (+0x30). Otherwise releases it (slot 8, `stdcall`) and clears it; marks each stream (16 bytes
/// at +0x24; the address tested for null) whose entries (+8, 0x1c bytes each, count +0x1c) are
/// in use and counts them, plus one for the end marker when there are any. The element array
/// (+0x2c, capacity +0x14) is given back to the memory manager when too small; with nothing
/// in use the capacity is cleared and 0 answered. Otherwise a new array (8 bytes each, all ones
/// on overflow) is taken when there is none, the element count +0x18 cleared, and each marked
/// stream's used entries get their offset (+4, the running total) and are added (`00e76660`
/// with the stream index, which answers the bytes they cover); the stream's +0xc gets the
/// total. The end element (`0xff`, 0, type 0x11, 0, 0, 0) is written at the count, and the
/// device (+0x10) makes the declaration (`CreateVertexDeclaration`, slot 0x158, into +0x30): a
/// negative result answers 0, otherwise the modified byte is cleared and the declaration
/// answered.
pub fn fn_00e76a80(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let built = e.mem.u32(base.wrapping_add(0x30));
    if e.mem.u8(base.wrapping_add(0x28)) == 0 {
        return built;
    }
    if built != 0 {
        e.vcall(built, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x30), 0);
    }
    let mut used = 0u32;
    let mut index = 0u32;
    let mut offset = 0u32;
    if e.mem.u32(base.wrapping_add(0x20)) != 0 {
        while index < e.mem.u32(base.wrapping_add(0x20)) {
            let stream = e.mem.u32(base.wrapping_add(0x24)).wrapping_add(offset);
            if stream != 0 {
                e.mem.set_u8(stream, 0);
                let entries = e.mem.u32(stream.wrapping_add(8));
                let mut at = entries;
                let mut entry = 0u32;
                while entry < e.mem.u32(base.wrapping_add(0x1c)) {
                    if entry_used(e, at) {
                        used += 1;
                        e.mem.set_u8(stream, 1);
                    }
                    entry += 1;
                    at = at.wrapping_add(0x1c);
                }
            }
            index += 1;
            offset = offset.wrapping_add(0x10);
        }
        if used != 0 {
            used += 1;
        }
    }
    if e.mem.u32(base.wrapping_add(0x14)) < used {
        let elements = e.mem.u32(base.wrapping_add(0x2c));
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, elements]);
        e.mem.set_u32(base.wrapping_add(0x2c), 0);
    }
    if used == 0 {
        e.mem.set_u32(base.wrapping_add(0x14), 0);
        return 0;
    }
    if e.mem.u32(base.wrapping_add(0x2c)) == 0 {
        e.mem.set_u32(base.wrapping_add(0x14), used);
        let bytes = u64::from(used) * 8;
        let bytes = if bytes > u64::from(u32::MAX) {
            u32::MAX
        } else {
            bytes as u32
        };
        let elements = e
            .call(
                MEMORY_MANAGER_ALLOCATE,
                &args![MEMORY_MANAGER_OBJECT, bytes],
            )
            .u32();
        e.mem.set_u32(base.wrapping_add(0x2c), elements);
    }
    e.mem.set_u32(base.wrapping_add(0x18), 0);
    let mut stream_index = 0u32;
    let mut offset = 0u32;
    while stream_index < e.mem.u32(base.wrapping_add(0x20)) {
        let stream = e.mem.u32(base.wrapping_add(0x24)).wrapping_add(offset);
        if e.mem.u8(stream) != 0 {
            let mut total = 0u32;
            let mut entry = 0u32;
            let mut entry_offset = 0u32;
            while entry < e.mem.u32(base.wrapping_add(0x1c)) {
                let at = e.mem.u32(stream.wrapping_add(8)).wrapping_add(entry_offset);
                if entry_used(e, at) {
                    e.mem.set_u32(at.wrapping_add(4), total);
                    let covered = e
                        .call(ADD_ELEMENT_00E76660, &args![base, at, stream_index])
                        .u32();
                    total = total.wrapping_add(covered);
                }
                entry += 1;
                entry_offset = entry_offset.wrapping_add(0x1c);
            }
            e.mem.set_u32(stream.wrapping_add(0xc), total);
        }
        offset = offset.wrapping_add(0x10);
        stream_index += 1;
    }
    let end = |e: &Engine| {
        let count = e.mem.u32(base.wrapping_add(0x18));
        let elements = e.mem.u32(base.wrapping_add(0x2c));
        elements.wrapping_add(count.wrapping_mul(8))
    };
    let at = end(e);
    e.mem.set_u16(at, 0xff);
    let at = end(e);
    e.mem.set_u16(at.wrapping_add(2), 0);
    let at = end(e);
    e.mem.set_u8(at.wrapping_add(4), 0x11);
    for byte in 5..8u32 {
        let at = end(e);
        e.mem.set_u8(at.wrapping_add(byte), 0);
    }
    let device = e.mem.u32(base.wrapping_add(0x10));
    let elements = e.mem.u32(base.wrapping_add(0x2c));
    let result = e
        .vcall(device, 0x158, &args![elements, base.wrapping_add(0x30)])
        .u32();
    if (result as i32) < 0 {
        return 0;
    }
    e.mem.set_u8(base.wrapping_add(0x28), 0);
    e.mem.u32(base.wrapping_add(0x30))
}

/// The base destructor `00e78be0` (unit `nid3dshaderdeclaration.cpp`; `thiscall`, no stack
/// words) and the Ni free `00aa1460` (`cdecl`: block, size).
const BASE_DESTRUCT_00E78BE0: u32 = 0x00e7_8be0;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e78510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`): the declaration's scalar
/// deleting destructor, `thiscall` with the flags. Stores the table `010ee70c`, gives the
/// element array (+0x2c) back to the memory manager, releases the built declaration (+0x30,
/// slot 8) when set and clears it, runs the base destructor `00e78be0` and, with bit 0 set,
/// frees the 0x38 bytes (`00aa1460`). Answers `this`.
pub fn fn_00e78510(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let elements = e.mem.u32(base.wrapping_add(0x2c));
    e.mem.set_u32(base, DECLARATION_VTABLE);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, elements]);
    let built = e.mem.u32(base.wrapping_add(0x30));
    if built != 0 {
        e.vcall(built, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x30), 0);
    }
    e.call(BASE_DESTRUCT_00E78BE0, &args![base]);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![base, 0x38u32]);
    }
    this
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x0e30_0000;

    fn engine(ready: u32, allocation: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register_double(NI_ALLOCATE, move |e, a| {
            if allocation {
                e.mem.alloc(a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(BASE_CONSTRUCT_00E78840, |_, a| a[0].into_ret());
        e.register_double(INITIALIZE_00E78C60, move |_, _| ready.into_ret());
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(DECLARATION_VTABLE, &[DELETE]);
        e.call_log = Some(vec![]);
        let renderer = e.mem.alloc(0x700);
        e.mem.set_u32(renderer + 0x6e8, 2);
        (e, renderer)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn create_builds_and_initializes_the_declaration() {
        let (mut e, renderer) = engine(1, true);
        let made = e.call(0x00e7_6700, &args![renderer, 5u32, 2u32]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made), DECLARATION_VTABLE);
        assert_eq!(e.mem.u32(made + 0x2c), 0);
        assert_eq!(e.mem.u8(made + 0x34), 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x38]]);
        assert_eq!(calls(&e, BASE_CONSTRUCT_00E78840), vec![vec![made]]);
        assert_eq!(
            calls(&e, INITIALIZE_00E78C60),
            vec![vec![made, renderer, 5, 2]]
        );
        assert!(calls(&e, DELETE).is_empty());
    }

    #[test]
    fn create_deletes_a_declaration_that_fails_to_initialize() {
        let (mut e, renderer) = engine(0x100, true);
        assert_eq!(e.call(0x00e7_6700, &args![renderer, 5u32, 1u32]).u32(), 0);
        let made = calls(&e, BASE_CONSTRUCT_00E78840)[0][0];
        assert_eq!(calls(&e, DELETE), vec![vec![made, 1]]);
    }

    #[test]
    fn create_refuses_too_many_streams_and_a_failed_allocation() {
        let (mut e, renderer) = engine(1, true);
        assert_eq!(e.call(0x00e7_6700, &args![renderer, 5u32, 3u32]).u32(), 0);
        assert!(calls(&e, NI_ALLOCATE).is_empty());
        let (mut e, renderer) = engine(1, false);
        assert_eq!(e.call(0x00e7_6700, &args![renderer, 5u32, 2u32]).u32(), 0);
        assert!(calls(&e, BASE_CONSTRUCT_00E78840).is_empty());
    }
}

#[cfg(test)]
mod b000l_tests {
    use super::*;

    #[test]
    fn fn_00e78840_clears_the_declaration() {
        let mut e = Engine::new();
        e.register(SHADER_DECLARATION_BASE_00EE88B0, |e, a| {
            e.mem.set_u32(a[0], 0x0e40_0000);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xcc; 0x2c]);
        assert_eq!(e.call(0x00e7_8840, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), D3D_DECLARATION_VTABLE_010EE7CC);
        assert_eq!(e.mem.u32(this + 4), 0xcccc_cccc);
        assert_eq!(e.mem.bytes(this + 8, 0x21), vec![0; 0x21]);
        assert_eq!(e.mem.bytes(this + 0x29, 3), vec![0xcc; 3]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SHADER_DECLARATION_BASE_00EE88B0, vec![this]));
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const TABLE: u32 = 0x0ea0_0000;
    const SET_ENTRY: u32 = 0x0ea1_0094;
    const BUILD: u32 = 0x0ea1_00b4;
    const GET_WORDS: u32 = 0x0ea1_00b0;
    const BYTE_A8: u32 = 0x0ea1_00a8;
    const PER_VERTEX: u32 = 0x0ea1_00a0;
    const COUNT_90: u32 = 0x0ea1_0090;
    const RELEASE: u32 = 0x0ea1_0008;
    const GET_DESC: u32 = 0x0ea1_0034;
    const CREATE_DECLARATION: u32 = 0x0ea1_0158;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// One table for the declaration, the device, the data and the vertex buffers.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x57];
        for slot in [
            SET_ENTRY,
            BUILD,
            GET_WORDS,
            BYTE_A8,
            PER_VERTEX,
            COUNT_90,
            RELEASE,
            GET_DESC,
            CREATE_DECLARATION,
        ] {
            slots[((slot & 0xffff) / 4) as usize] = slot;
        }
        e.put_vtable(TABLE, &slots);
        for no_op in [
            RELEASE,
            MEMORY_MANAGER_FREE,
            BASE_DESTRUCT_00E78BE0,
            NI_FREE_00AA1460,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, TABLE);
        object
    }

    /// A declaration with `streams` streams of `entries` entries each (all unused).
    fn declaration(e: &mut Engine, streams: u32, entries: u32) -> u32 {
        let this = object(e, 0x38);
        let array = e.mem.alloc(0x10 * streams.max(1));
        for stream in 0..streams {
            let list = e.mem.alloc(0x1c * entries.max(1));
            for entry in 0..entries {
                e.mem.set_u32(list + 0x1c * entry + 8, 0xffff_ffff);
                e.mem.set_u32(list + 0x1c * entry + 0xc, 0x11);
            }
            e.mem.set_u32(array + 0x10 * stream + 8, list);
        }
        e.mem.set_u32(this + 0x1c, entries);
        e.mem.set_u32(this + 0x20, streams);
        e.mem.set_u32(this + 0x24, array);
        this
    }

    fn entry_at(e: &Engine, this: u32, stream: u32, entry: u32) -> u32 {
        let array = e.mem.u32(this + 0x24);
        e.mem.u32(array + 0x10 * stream + 8) + 0x1c * entry
    }

    #[test]
    fn fn_00e76350_maps_the_kind() {
        let mut e = engine();
        e.register(SET_ENTRY, |_, _| 1u32.into_ret());
        let this = object(&mut e, 0x38);
        e.call_log = Some(vec![]);
        for kind in [0u32, 4, 5, 9, 13, 14] {
            assert!(e
                .call(0x00e7_6350, &args![this, 1u32, 2u32, kind, 3u32, 4u32])
                .bool());
        }
        let seen: Vec<(u32, u32)> = calls(&e, SET_ENTRY).iter().map(|w| (w[6], w[7])).collect();
        assert_eq!(seen, vec![(0, 0), (10, 0), (5, 0), (5, 4), (6, 0), (7, 0)]);
        assert_eq!(calls(&e, SET_ENTRY)[0], vec![this, 4, 1, 2, 0, 3, 0, 0, 0]);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e7_6350, &args![this, 1u32, 2u32, 15u32, 3u32, 4u32])
            .bool());
        assert!(calls(&e, SET_ENTRY).is_empty());
    }

    #[test]
    fn fn_00e76450_stores_a_changed_entry() {
        let mut e = engine();
        let this = declaration(&mut e, 2, 3);
        let built = object(&mut e, 4);
        e.mem.set_u32(this + 0x30, built);
        let set = |e: &mut Engine, stream: u32, entry: u32, usage: u32, kind: u32| {
            e.call(
                0x00e7_6450,
                &args![this, stream, entry, 0u32, usage, kind, 5u32, 6u32, 7u32],
            )
            .bool()
        };
        e.call_log = Some(vec![]);
        assert!(set(&mut e, 1, 2, 1, 3));
        let at = entry_at(&e, this, 1, 2);
        assert_eq!(e.mem.u32(at + 8), 1);
        assert_eq!(e.mem.u32(at + 0xc), 2);
        assert_eq!(e.mem.u32(at + 0x14), 5);
        assert_eq!(e.mem.u32(at + 0x18), 6);
        assert_eq!(e.mem.u32(at + 0x10), 7);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        assert_eq!(calls(&e, RELEASE), vec![vec![built]]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        // The same values (type 0x10 kept): nothing new.
        assert!(set(&mut e, 0, 0, 1, 0x10));
        let at = entry_at(&e, this, 0, 0);
        assert_eq!(e.mem.u32(at + 0xc), 0x10);
        e.mem.set_u8(this + 0x28, 0);
        assert!(set(&mut e, 0, 0, 1, 0x10));
        assert_eq!(e.mem.u8(this + 0x28), 0);
        // Out of range.
        assert!(!set(&mut e, 2, 0, 0, 0));
        assert!(!set(&mut e, 0, 3, 0, 0));
    }

    #[test]
    fn fn_00e765e0_and_00e76630_build_first() {
        let mut e = engine();
        e.register(BUILD, |_, _| 0xbb00u32.into_ret());
        let this = declaration(&mut e, 2, 1);
        let array = e.mem.u32(this + 0x24);
        e.mem.set_u32(array + 0x1c, 0x24);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_65e0, &args![this, 1u32]).u32(), 0x24);
        assert_eq!(calls(&e, BUILD), vec![vec![this]]);
        assert_eq!(e.call(0x00e7_65e0, &args![this, 2u32]).u32(), 0);
        let (built, count) = (e.mem.alloc(4), e.mem.alloc(4));
        assert!(e.call(0x00e7_6630, &args![this, built, count]).bool());
        assert_eq!((e.mem.u32(built), e.mem.u32(count)), (0xbb00, 2));
        e.register(BUILD, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e7_6630, &args![this, built, count]).bool());
    }

    #[test]
    fn fn_00e76a80_builds_the_elements() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ADD_ELEMENT_00E76660, |e, a| {
            let count = e.mem.u32(a[0] + 0x18);
            e.mem.set_u32(a[0] + 0x18, count + 1);
            0xcu32.into_ret()
        });
        e.register(CREATE_DECLARATION, |e, a| {
            e.mem.set_u32(a[2], 0xdec0);
            0u32.into_ret()
        });
        let this = declaration(&mut e, 2, 2);
        let device = object(&mut e, 4);
        e.mem.set_u32(this + 0x10, device);
        // Not modified: the built declaration as is.
        e.mem.set_u32(this + 0x30, 0x1234);
        assert_eq!(e.call(0x00e7_6a80, &args![this]).u32(), 0x1234);
        e.mem.set_u32(this + 0x30, 0);
        e.mem.set_u8(this + 0x28, 1);
        // Nothing in use: 0 and the capacity cleared.
        e.mem.set_u32(this + 0x14, 9);
        assert_eq!(e.call(0x00e7_6a80, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        // Stream 1 uses both entries.
        for entry in 0..2 {
            let at = entry_at(&e, this, 1, entry);
            e.mem.set_u32(at + 8, 0);
            e.mem.set_u32(at + 0xc, 2);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_6a80, &args![this]).u32(), 0xdec0);
        assert_eq!(e.mem.u32(this + 0x14), 3);
        let array = e.mem.u32(this + 0x24);
        assert_eq!(e.mem.u8(array), 0);
        assert_eq!(e.mem.u8(array + 0x10), 1);
        assert_eq!(e.mem.u32(array + 0x1c), 0x18);
        assert_eq!(e.mem.u32(entry_at(&e, this, 1, 1) + 4), 0xc);
        let adds: Vec<u32> = calls(&e, ADD_ELEMENT_00E76660)
            .iter()
            .map(|w| w[2])
            .collect();
        assert_eq!(adds, vec![1, 1]);
        let elements = e.mem.u32(this + 0x2c);
        assert_eq!(
            e.mem.bytes(elements + 0x10, 8),
            vec![0xff, 0, 0, 0, 0x11, 0, 0, 0]
        );
        assert_eq!(
            calls(&e, CREATE_DECLARATION),
            vec![vec![device, elements, this + 0x30]]
        );
        assert_eq!(e.mem.u8(this + 0x28), 0);
        // A failing device: 0, still modified.
        e.register(CREATE_DECLARATION, |_, _| 0x8000_0000u32.into_ret());
        e.mem.set_u32(this + 0x30, 0);
        e.mem.set_u8(this + 0x28, 1);
        assert_eq!(e.call(0x00e7_6a80, &args![this]).u32(), 0);
        assert_eq!(e.mem.u8(this + 0x28), 1);
    }

    #[test]
    fn fn_00e78510_destroys_and_frees() {
        let mut e = engine();
        let this = object(&mut e, 0x38);
        let built = object(&mut e, 4);
        e.mem.set_u32(this + 0x2c, 0x2c00);
        e.mem.set_u32(this + 0x30, built);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_8510, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, 0x2c00]]
        );
        assert_eq!(calls(&e, RELEASE), vec![vec![built]]);
        assert_eq!(calls(&e, BASE_DESTRUCT_00E78BE0), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0x38]]);
        assert_eq!(e.mem.u32(this), DECLARATION_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        e.call_log = Some(vec![]);
        e.call(0x00e7_8510, &args![this, 0u32]);
        assert!(calls(&e, RELEASE).is_empty());
        assert!(calls(&e, NI_FREE_00AA1460).is_empty());
    }

    /// A shader declaration for `00e76770`: one stream (stride 0x14 at +0xc) of three entries
    /// (offsets 0, 0xc, 0x10; usages 0, 4 and 1), a renderer, a vertex buffer manager.
    fn pack_rig(e: &mut Engine) -> (u32, u32, u32) {
        for no_op in [
            BUFFER_DATA_UPDATE_00E8F040,
            PACK_ENTRY_00E7A200,
            PACK_DERIVED_00A855F0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(GET_WORDS, |e, a| {
            e.mem.set_u32(a[1], 0x0b01);
            1u32.into_ret()
        });
        e.register(LOCK_00E8BFF0, |_, _| 0x7000u32.into_ret());
        e.register(UNLOCK_00E8C0D0, |_, _| 1u32.into_ret());
        let this = declaration(e, 1, 3);
        let array = e.mem.u32(this + 0x24);
        e.mem.set_u32(array + 0xc, 0x14);
        for (entry, (offset, usage)) in [(0u32, 0u32), (0xc, 4), (0x10, 1)].into_iter().enumerate()
        {
            let at = entry_at(e, this, 0, entry as u32);
            e.mem.set_u32(at + 4, offset);
            e.mem.set_u32(at + 8, usage);
            e.mem.set_u32(at + 0xc, 2);
        }
        let renderer = e.mem.alloc(0x5d8);
        e.mem.set_u32(this + 8, renderer);
        e.mem.set_u32(this + 0xc, 0x0c00);
        let buffer_data = e.mem.alloc(0x2c);
        let strides = e.mem.alloc(4);
        e.mem.set_u32(buffer_data + 0x20, 1);
        e.mem.set_u32(buffer_data + 0x24, strides);
        let data = object(e, 0x38);
        e.mem.set_u32(data + 0x34, buffer_data);
        e.mem.set_u16(data + 8, 6);
        e.mem.set_u32(data + 0x20, 0x2000);
        e.mem.set_u32(data + 0x24, 0x2400);
        (this, data, buffer_data)
    }

    #[test]
    fn fn_00e76770_makes_a_chip_and_packs_the_entries() {
        let mut e = engine();
        let (this, data, buffer_data) = pack_rig(&mut e);
        e.register(COUNT_90, |_, _| 0x0001_0006u32.into_ret());
        e.register(CHIP_CHECK_00E8BFA0, |_, _| 1u32.into_ret());
        let chip = e.mem.alloc(0x18);
        e.mem.set_u32(chip + 8, 0xbf00);
        e.mem.set_u32(chip + 0xc, 0x40);
        e.mem.set_u32(chip + 0x10, 0x10);
        e.mem.set_u32(chip + 0x14, 0x60);
        e.register_double(NEW_CHIP_00E69B40, move |_, _| chip.into_ret());
        e.call_log = Some(vec![]);
        let answered = e
            .call(
                0x00e7_6770,
                &args![this, data, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0x88u32, 0x99u32],
            )
            .u32();
        assert_eq!(answered, chip);
        assert_eq!(
            calls(&e, BUFFER_DATA_UPDATE_00E8F040),
            vec![vec![buffer_data, 0x0b01]]
        );
        assert_eq!(e.mem.u32(e.mem.u32(buffer_data + 0x24)), 0x14);
        assert_eq!(
            calls(&e, LOCK_00E8BFF0),
            vec![vec![0x0c00, 0xbf00, 0x40, 0x60, 0x10]]
        );
        // The flags became 0xfff: all three entries packed, at the lock plus their offsets.
        let packed: Vec<(u32, u32)> = calls(&e, PACK_ENTRY_00E7A200)
            .iter()
            .map(|w| (w[1], w[7]))
            .collect();
        assert_eq!(packed, vec![(6, 0x7000), (6, 0x700c), (6, 0x7010)]);
        let first = &calls(&e, PACK_ENTRY_00E7A200)[0];
        assert_eq!(
            first.clone(),
            vec![
                this,
                6,
                data,
                0,
                0,
                entry_at(&e, this, 0, 0),
                0x14,
                0x7000,
                0x88,
                0x99
            ]
        );
        assert_eq!(calls(&e, UNLOCK_00E8C0D0), vec![vec![0x0c00, 0xbf00]]);
        assert!(calls(&e, PACK_DERIVED_00A855F0).is_empty());
        // The manager refusing the chip: 0.
        e.register(CHIP_CHECK_00E8BFA0, |_, _| 0u32.into_ret());
        let answered = e
            .call(
                0x00e7_6770,
                &args![this, data, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(answered, 0);
    }

    #[test]
    fn fn_00e76770_with_a_locked_address_and_derived_vectors() {
        let mut e = engine();
        let (this, data, _) = pack_rig(&mut e);
        e.register(COUNT_90, |_, _| 6u32.into_ret());
        e.call_log = Some(vec![]);
        // Flags 1: the vertices (usage 0) are kept for the data's own packer; usage 4 needs 4.
        let answered = e
            .call(
                0x00e7_6770,
                &args![this, data, 0x3300u32, 0u32, 1u32, 0x5500u32, 0u32, 0x9000u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(answered, 0x5500);
        assert!(calls(&e, LOCK_00E8BFF0).is_empty());
        assert!(calls(&e, UNLOCK_00E8C0D0).is_empty());
        let packed: Vec<u32> = calls(&e, PACK_ENTRY_00E7A200)
            .iter()
            .map(|w| w[7])
            .collect();
        assert_eq!(packed, vec![0x9010]);
        assert_eq!(
            calls(&e, PACK_DERIVED_00A855F0),
            vec![vec![0x3300, 0x2000, 0x2400, 6, 0x9000, 0, 0, 0, 0x14]]
        );
        // No vertices: the chip answered at once.
        e.register(COUNT_90, |_, _| 0x0001_0000u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00e7_6770,
                &args![this, data, 0u32, 0u32, 0u32, 0x5500u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32(),
            0x5500
        );
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_00e76770_keeps_a_big_enough_chip() {
        let mut e = engine();
        let (this, data, _) = pack_rig(&mut e);
        e.register(COUNT_90, |_, _| 6u32.into_ret());
        e.register(PER_VERTEX, |_, _| 0x14u32.into_ret());
        e.register(GET_DESC, |e, a| {
            e.mem.set_u32(a[1], 0x64);
            e.mem.set_u32(a[1] + 0x10, 0x78);
            Ret::default()
        });
        let buffer = object(&mut e, 4);
        let chip = e.mem.alloc(0x18);
        e.mem.set_u32(chip + 8, buffer);
        e.call_log = Some(vec![]);
        let answered = e
            .call(
                0x00e7_6770,
                &args![this, data, 0u32, 0u32, 4u32, chip, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(answered, chip);
        assert!(calls(&e, NEW_CHIP_00E69B40).is_empty());
        assert_eq!(calls(&e, PER_VERTEX), vec![vec![this, 0]]);
        // Flags 4: usage 4 and usage 1 packed; usage 0 (bit 1) not.
        let packed: Vec<u32> = calls(&e, PACK_ENTRY_00E7A200)
            .iter()
            .map(|w| w[7])
            .collect();
        assert_eq!(packed, vec![0x700c, 0x7010]);
    }
}

#[cfg(test)]
mod n010_tests {
    use super::*;

    const RELEASE: u32 = 0x0e50_0008;

    /// A declaration with 3 entries per stream and 2 streams, entries filled with 7, and a
    /// Direct3D declaration whose slot +8 is `RELEASE`.
    fn declaration(e: &mut Engine) -> (u32, u32) {
        e.register(RELEASE, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, RELEASE);
        let d3d = e.mem.alloc(4);
        e.mem.set_u32(d3d, vtable);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u32(this + 0x20, 2);
        e.mem.set_u32(this + 0x30, d3d);
        let streams = e.mem.alloc(0x20);
        for stream in 0..2u32 {
            let entries = e.mem.alloc(3 * 0x1c);
            e.mem.write(entries, &[7; 3 * 0x1c]);
            e.mem.set_u32(streams + 0x10 * stream + 8, entries);
        }
        e.mem.set_u32(this + 0x24, streams);
        e.call_log = Some(vec![]);
        (this, d3d)
    }

    fn entry_at(e: &Engine, this: u32, stream: u32, entry: u32) -> u32 {
        let streams = e.mem.u32(this + 0x24);
        e.mem.u32(streams + 0x10 * stream + 8) + 0x1c * entry
    }

    fn releases(e: &mut Engine) -> usize {
        let log = e.call_log.replace(vec![]).unwrap();
        log.iter().filter(|c| c.0 == RELEASE).count()
    }

    #[test]
    fn fn_00e76500_marks_one_entry_unused() {
        let mut e = Engine::new();
        let (this, _) = declaration(&mut e);
        assert!(!e.call(0x00e7_6500, &args![this, 3u32, 0u32]).bool());
        assert!(!e.call(0x00e7_6500, &args![this, 0u32, 2u32]).bool());
        assert_eq!(e.mem.u8(this + 0x28), 0);
        assert_eq!(releases(&mut e), 0);
        assert!(e.call(0x00e7_6500, &args![this, 2u32, 1u32]).bool());
        let at = entry_at(&e, this, 1, 2);
        assert_eq!(e.mem.u32(at + 8), 0xffff_ffff);
        assert_eq!(e.mem.u32(at + 0xc), 0x11);
        assert_eq!(e.mem.u32(at + 4), 0x0707_0707);
        assert_eq!(e.mem.u32(entry_at(&e, this, 1, 1) + 8), 0x0707_0707);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(releases(&mut e), 1);
        // No Direct3D declaration left: nothing to release.
        assert!(e.call(0x00e7_6500, &args![this, 0u32, 0u32]).bool());
        assert_eq!(releases(&mut e), 0);
    }

    #[test]
    fn fn_00e76570_marks_every_entry_of_the_stream_unused() {
        let mut e = Engine::new();
        let (this, _) = declaration(&mut e);
        assert!(!e.call(0x00e7_6570, &args![this, 2u32]).bool());
        assert_eq!(releases(&mut e), 0);
        assert!(e.call(0x00e7_6570, &args![this, 0u32]).bool());
        for entry in 0..3 {
            let at = entry_at(&e, this, 0, entry);
            assert_eq!(e.mem.u32(at + 8), 0xffff_ffff);
            assert_eq!(e.mem.u32(at + 0xc), 0x11);
        }
        assert_eq!(e.mem.u32(entry_at(&e, this, 1, 0) + 8), 0x0707_0707);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        assert_eq!(releases(&mut e), 1);
    }

    #[test]
    fn fn_00e76610_and_00e76620_store_and_read_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        e.call(0x00e7_6610, &args![this, 0x5au8]);
        assert_eq!(e.mem.u8(this + 0x34), 0x5a);
        assert_eq!(e.call(0x00e7_6620, &args![this]).u8(), 0x5a);
    }

    #[test]
    fn fn_00e76620_reads_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        e.mem.set_u8(this + 0x34, 1);
        assert_eq!(e.call(0x00e7_6620, &args![this]).u8(), 1);
    }
}
