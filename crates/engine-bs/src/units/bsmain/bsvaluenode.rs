//! `BSMain/bsvaluenode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

// Translated from 00a643b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStreamLoadBinary<unsigned char>` (engine map name
/// `NiStreamLoadBinary<unsigned_char>_ov2`, the second copy; `cdecl`, two stack words): reads one byte
/// from the stream's input stream into `destination`. Calls the input stream's read
/// function as `function(input, destination, 1, &count, 1)` with `count` a local set
/// to 1, and adds the byte count it returns to the input stream's position.
pub fn ni_stream_load_binary_unsigned_char_ov2(
    e: &mut Engine,
    stream: Ptr<NiStream>,
    destination: Ptr,
) {
    let input = e.get(stream, NiStream::m_pkIstr);
    let input = Ptr::<NiBinaryStream>::new(input);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(function, &args![input, destination, 1u32, count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

/// `00a5e9d0` (named `NiNode::LinkObject` in the engine map): the base body the wrapper
/// forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A5E9D0: u32 = 0x00a5_e9d0;

// Translated from 00c43900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSValueNode` vtable word 010c19ac, right after
/// `BSValueNode::LoadBinary` (010c19a8). Forwards `this` and the stream to `00a5e9d0`
/// (`NiNode::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c43900(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A5E9D0, &args![this, stream]);
}

// Translated from 00c43cf0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202de8 without reading the receiver. The map names this body `BSValueNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c43cf0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2de8
}

/// The vtable of `BSValueNode` (the constructor stores it).
const BS_VALUE_NODE_VTABLE: u32 = 0x010c_195c;
/// `BSValueNode` size on PC (the allocations and the sized delete).
const BS_VALUE_NODE_SIZE: u32 = 0xb8;
/// `BSValueNode` fields on PC (Xbox PDB names; the Xbox offsets are 0x24
/// higher): `m_uFlags` +0xac, `iValue` +0xb0, `iType` +0xb4.
const VALUE_NODE_FLAGS: u32 = 0xac;
const VALUE: u32 = 0xb0;
const VALUE_TYPE: u32 = 0xb4;
/// The world transform `UpdateWorldData` overwrites (`NiAVObject`, nine
/// words at +0x68) and the nine words it copies there (`011a9448`).
const WORLD_TRANSFORM: u32 = 0x68;
const FIXED_TRANSFORM: u32 = 0x011a_9448;

/// `00c5f570` (`BSNiNode`'s constructor body), `00c5fb60` (`thiscall`,
/// `(destination, cloning)`: its member copy) and `00c5faa0` (its
/// destructor body).
const BS_NI_NODE_CONSTRUCT: u32 = 0x00c5_f570;
const BS_NI_NODE_COPY_MEMBERS: u32 = 0x00c5_fb60;
const BS_NI_NODE_DESTRUCT: u32 = 0x00c5_faa0;
/// `NiNode::LoadBinary` (`00a5dd40`).
const NI_NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
/// `00a26df0` (unnamed in the engine map; `cdecl`, `(stream, &word)`):
/// reads 4 bytes.
const STREAM_READ_WORD: u32 = 0x00a2_6df0;
/// `NiAVObject::UpdateWorldData` (`00a68c60`).
const AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiStream`'s word at +4 that `LoadBinary` compares with 0x18 (the
/// file's user version, by its use; unconfirmed).
const STREAM_USER_VERSION: u32 = 0x04;

// Translated from 00c43a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c43a30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e472, state variable) is not translated
/// Unnamed in the engine map (the `BSValueNode` factory, no arguments):
/// allocates 0xb8 bytes and constructs a node in it (`00c43660`) when that
/// succeeds. Returns it, or 0.
pub fn fn_00c43a30(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![BS_VALUE_NODE_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    bs_value_node_construct(e, Ptr::new(block))
}

// Translated from 00c43660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::BSValueNode` (Xbox PDB): `BSNiNode`'s constructor body
/// (`00c5f570`), the vtable, value 0 and type 0. Returns `this`.
pub fn bs_value_node_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BS_NI_NODE_CONSTRUCT, &args![this]);
    let node = this.addr();
    e.mem.set_u32(node, BS_VALUE_NODE_VTABLE);
    e.mem.set_u32(node.wrapping_add(VALUE), 0);
    e.mem.set_u32(node.wrapping_add(VALUE_TYPE), 0);
    this
}

// Translated from 00c438a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::LoadBinary` (Xbox PDB): `NiNode::LoadBinary`, the value (4
/// bytes through `00a26df0` into a local, then stored), and the flags byte
/// when the stream's word at +4 is above 0x18.
pub fn bs_value_node_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_NODE_LOAD_BINARY, &args![this, stream]);
    let node = this.addr();
    let value = e.with_stack(4, |e, local| {
        e.call(STREAM_READ_WORD, &args![stream, local]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(node.wrapping_add(VALUE), value);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_USER_VERSION)) > 0x18 {
        ni_stream_load_binary_unsigned_char_ov2(
            e,
            Ptr::new(stream.addr()),
            Ptr::new(node.wrapping_add(VALUE_NODE_FLAGS)),
        );
    }
}

// Translated from 00c437c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c437c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e445, state variable) is not translated
/// `BSValueNode::CreateClone` (Xbox PDB): allocates 0xb8 bytes, constructs
/// a node in it when that succeeds, runs the member copy (`00c43850`) into
/// it (into address 0 when the allocation failed) and returns it.
pub fn bs_value_node_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![BS_VALUE_NODE_SIZE]).u32();
    let node = if block == 0 {
        Ptr::new(0)
    } else {
        bs_value_node_construct(e, Ptr::new(block))
    };
    bs_value_node_copy_members(e, this, node, cloning);
    node
}

// Translated from 00c43850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::CopyMembers` (Xbox PDB): `BSNiNode`'s member copy
/// (`00c5fb60`), then the value and the flags byte.
pub fn bs_value_node_copy_members(e: &mut Engine, this: Ptr, destination: Ptr, cloning: u32) {
    e.call(BS_NI_NODE_COPY_MEMBERS, &args![this, destination, cloning]);
    let value = e.mem.u32(this.addr().wrapping_add(VALUE));
    e.mem.set_u32(destination.addr().wrapping_add(VALUE), value);
    let flags = e.mem.u8(this.addr().wrapping_add(VALUE_NODE_FLAGS));
    e.mem
        .set_u8(destination.addr().wrapping_add(VALUE_NODE_FLAGS), flags);
}

// Translated from 00c43b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::UpdateWorldData` (Xbox PDB): `NiAVObject::UpdateWorldData`,
/// then, with bit 0 of the flags byte, the nine words at `011a9448` copied
/// over the world transform (+0x68).
pub fn bs_value_node_update_world_data(e: &mut Engine, this: Ptr, update: u32) {
    e.call(AV_OBJECT_UPDATE_WORLD_DATA, &args![this, update]);
    let node = this.addr();
    if e.mem.u8(node.wrapping_add(VALUE_NODE_FLAGS)) & 1 != 0 {
        for k in 0..9 {
            let word = e.mem.u32(FIXED_TRANSFORM + 4 * k);
            e.mem
                .set_u32(node.wrapping_add(WORLD_TRANSFORM + 4 * k), word);
        }
    }
}

// Translated from 00c43cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsvaluenode.obj`): the `BSValueNode`
/// scalar deleting destructor. `BSNiNode`'s destructor body (`00c5faa0`),
/// then the sized delete of 0xb8 bytes when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00c43cb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BS_NI_NODE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, BS_VALUE_NODE_SIZE]);
    }
    this
}

/// `00c5f680` (`thiscall`, `(node, 0)`): the `BSNiNode` constructor from an existing node.
const BS_NI_NODE_CONSTRUCT_FROM: u32 = 0x00c5_f680;
/// `NiBillboardNode::ms_RTTI` (as other units name it).
const NI_BILLBOARD_NODE_RTTI: u32 = 0x011f_4aa0;
/// `NiNode::RegisterStreamables` (`00a5e5b0`), `NiNode::SaveBinary` (`00a5e880`),
/// `NiNode::IsEqual` (`00a5e600`) and `NiNode::GetViewerStrings` (`00a5ea70`), each
/// `thiscall` with one word.
const NI_NODE_REGISTER_STREAMABLES: u32 = 0x00a5_e5b0;
const NI_NODE_SAVE_BINARY: u32 = 0x00a5_e880;
const NI_NODE_IS_EQUAL: u32 = 0x00a5_e600;
const NI_NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
/// `NiStreamSaveBinary<float>` (`00a3da50`, `cdecl`, `(stream, &word)`).
const STREAM_SAVE_WORD: u32 = 0x00a3_da50;
/// `NiGetViewerString` (`00a7d330`, `(name)`), its `(format, int)` form `00a7d4d0` and its
/// `(label, text)` form `00a7d5e0` (all `cdecl`), and the viewer-string array's add
/// (`0096a610`, `thiscall`, `(&string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// `BSValueNode::ms_RTTI` (its first word is the class name), and the strings `"Value"`,
/// `"Billboard"`, `"true"` and `"false"`.
const BS_VALUE_NODE_RTTI: u32 = 0x0120_2de8;
const VALUE_LABEL: u32 = 0x0104_4adc;
const BILLBOARD_LABEL: u32 = 0x010c_1a64;
const TRUE_TEXT: u32 = 0x0102_fd2c;
const FALSE_TEXT: u32 = 0x0102_fd24;
/// `strlen` and `atol` (CRT, `cdecl`).
const STRLEN: u32 = 0x00ec_6130;
const ATOL: u32 = 0x00ec_a6d3;

// Translated from 00a29550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStreamSaveBinary<unsigned char>` (engine map; `cdecl`, two stack words): writes the
/// byte at `source` to the stream's output stream (`m_pkOstr`): its write function as
/// `function(output, source, 1, &count, 1)` with `count` (the caller's first argument
/// slot) set to 1, and the byte count it returns added to the output's position.
pub fn ni_stream_save_binary_unsigned_char(e: &mut Engine, stream: Ptr<NiStream>, source: Ptr) {
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let function = e.get(output, NiBinaryStream::m_pfnWrite);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        e.call(function, &args![output, source, 1u32, count, 1u32])
            .u32()
    });
    let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        output,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

// Translated from 00c436a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c436a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e418, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, `(node)`): a `BSValueNode` made from `node`:
/// `BSNiNode`'s constructor from a node (`00c5f680` with `node` and 0), the vtable, and,
/// when `node` is an `NiBillboardNode` (its RTTI chain) whose word at +0xAC has 5 in its low
/// three bits, bit 0 of the flags byte set. Returns `this`.
pub fn fn_00c436a0(e: &mut Engine, this: Ptr, node: u32) -> Ptr {
    e.call(BS_NI_NODE_CONSTRUCT_FROM, &args![this, node, 0u32]);
    let me = this.addr();
    e.mem.set_u32(me, BS_VALUE_NODE_VTABLE);
    let billboard = if node == 0 {
        false
    } else {
        let mut rtti = e.vcall(node, 8, &[]).u32();
        loop {
            if rtti == 0 {
                break false;
            }
            if rtti == NI_BILLBOARD_NODE_RTTI {
                break true;
            }
            rtti = e.mem.u32(rtti.wrapping_add(4));
        }
    };
    if billboard && e.mem.u16(node.wrapping_add(0xac)) & 7 == 5 {
        // The code sets or clears the bit on a constant 1 (`MOV EDX,1; TEST EDX,EDX`):
        // only the set branch can run; the clear branch is kept as the code has it.
        let set = 1u32;
        let flags = e.mem.u8(me.wrapping_add(VALUE_NODE_FLAGS));
        let flags = if set != 0 { flags | 1 } else { flags & !1 };
        e.mem.set_u8(me.wrapping_add(VALUE_NODE_FLAGS), flags);
    }
    this
}

// Translated from 00c43920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the `BSValueNode` vtable's `RegisterStreamables` place):
/// whether `NiNode::RegisterStreamables` succeeded.
pub fn fn_00c43920(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(NI_NODE_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        != 0
}

// Translated from 00c43950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::SaveBinary` (Xbox PDB, engine map): `NiNode::SaveBinary`, the value (4
/// bytes through `00a3da50` from a local copy) and the flags byte
/// (`NiStreamSaveBinary<unsigned char>`).
pub fn bs_value_node_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_NODE_SAVE_BINARY, &args![this, stream]);
    let value = e.mem.u32(this.addr().wrapping_add(VALUE));
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(STREAM_SAVE_WORD, &args![stream, local]);
    });
    ni_stream_save_binary_unsigned_char(
        e,
        Ptr::new(stream.addr()),
        Ptr::new(this.addr().wrapping_add(VALUE_NODE_FLAGS)),
    );
}

// Translated from 00c439b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::IsEqual` (Xbox PDB, engine map): `NiNode::IsEqual`, and when that holds,
/// whether the values and the flags bytes are equal.
pub fn bs_value_node_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let equal = e.call(NI_NODE_IS_EQUAL, &args![this, other]).u8();
    if equal == 0 {
        return false;
    }
    let mine = this.addr();
    let theirs = other.addr();
    e.mem.u32(mine.wrapping_add(VALUE)) == e.mem.u32(theirs.wrapping_add(VALUE))
        && e.mem.u8(mine.wrapping_add(VALUE_NODE_FLAGS))
            == e.mem.u8(theirs.wrapping_add(VALUE_NODE_FLAGS))
}

// Translated from 00c43ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::GetViewerStrings` (Xbox PDB, engine map): `NiNode::GetViewerStrings`, then
/// the class name (`NiGetViewerString` of `BSValueNode::ms_RTTI`'s name), `"Value"` with
/// the value, and, with bit 0 of the flags byte, `"Billboard"` with `"true"` (`"false"`
/// when the bit, tested again, is clear) added to the list.
pub fn bs_value_node_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    let me = this.addr();
    e.with_stack(0xc, |e, slots| {
        let name = e.global::<u32>(BS_VALUE_NODE_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        let slot = slots.addr().wrapping_add(8);
        e.mem.set_u32(slot, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        let value = e.mem.u32(me.wrapping_add(VALUE));
        let text = e
            .call(NI_GET_VIEWER_STRING_INT, &args![VALUE_LABEL, value])
            .u32();
        let slot = slots.addr().wrapping_add(4);
        e.mem.set_u32(slot, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        if e.mem.u8(me.wrapping_add(VALUE_NODE_FLAGS)) & 1 != 0 {
            let state = if e.mem.u8(me.wrapping_add(VALUE_NODE_FLAGS)) & 1 != 0 {
                TRUE_TEXT
            } else {
                FALSE_TEXT
            };
            let text = e
                .call(NI_GET_VIEWER_STRING_TEXT, &args![BILLBOARD_LABEL, state])
                .u32();
            let slot = slots.addr();
            e.mem.set_u32(slot, text);
            e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        }
    });
}

// Translated from 00c43be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::ParseNameForValue` (Xbox PDB, engine map): scans the name (+8) back from
/// its end over digits, `#` and `@` (the character before the name's start is read when
/// the whole name is such characters), stores `atol` of the rest after them as the value
/// and answers whether that rest was not empty.
pub fn bs_value_node_parse_name_for_value(e: &mut Engine, this: Ptr) -> bool {
    let name = e.mem.u32(this.addr().wrapping_add(8));
    let length = e.call(STRLEN, &args![name]).u32();
    let mut back = 1u32;
    loop {
        let c = e.mem.i8(name.wrapping_add(length.wrapping_sub(back)));
        let marker = (0x30..=0x39).contains(&c) || c == 0x23 || c == 0x40;
        if !marker || back > length {
            break;
        }
        back = back.wrapping_add(1);
    }
    let rest = name.wrapping_add(length.wrapping_sub(back)).wrapping_add(1);
    let found = e.mem.u8(rest) != 0;
    let value = e.call(ATOL, &args![rest]).u32();
    e.mem.set_u32(this.addr().wrapping_add(VALUE), value);
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a2_9550,
            ni_stream_save_binary_unsigned_char(Ptr<NiStream>, Ptr)
        ),
        entry!(0x00c4_36a0, fn_00c436a0(Ptr, u32) -> Ptr),
        entry!(0x00c4_3920, fn_00c43920(Ptr, Ptr) -> bool),
        entry!(0x00c4_3950, bs_value_node_save_binary(Ptr, Ptr)),
        entry!(0x00c4_39b0, bs_value_node_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c4_3ab0, bs_value_node_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c4_3be0, bs_value_node_parse_name_for_value(Ptr) -> bool),
        entry!(0x00c4_3cf0, fn_00c43cf0(Ptr) -> u32),
        entry!(
            0x00a6_43b0,
            ni_stream_load_binary_unsigned_char_ov2(Ptr<NiStream>, Ptr)
        ),
        entry!(0x00c4_3900, fn_00c43900(Ptr, Ptr)),
        entry!(0x00c4_3a30, fn_00c43a30() -> Ptr),
        entry!(0x00c4_3660, bs_value_node_construct(Ptr) -> Ptr),
        entry!(0x00c4_38a0, bs_value_node_load_binary(Ptr, Ptr)),
        entry!(0x00c4_37c0, bs_value_node_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00c4_3850, bs_value_node_copy_members(Ptr, Ptr, u32)),
        entry!(0x00c4_3b90, bs_value_node_update_world_data(Ptr, u32)),
        entry!(0x00c4_3cb0, fn_00c43cb0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c43900_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A5E9D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_3900, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A5E9D0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn load_binary_unsigned_char_reads_one_byte_through_the_input_stream() {
        const READ: u32 = 0x00aa_2e60;
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 4, 7);
        e.mem.set_u32(input.addr() + 8, READ);
        let destination = e.mem.alloc(8);
        e.register(READ, |e, a| {
            // (input, destination, 1, &count, 1) with count set to 1
            assert_eq!(a[2], 1);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 1);
            e.mem.set_u8(a[1], 0x5a);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00a6_43b0, &args![stream, destination]);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|call| call.0 == READ).collect();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].1[..3], [input.addr(), destination, 1]);
        assert_eq!(e.mem.u8(destination), 0x5a);
        assert_eq!(e.mem.u32(input.addr() + 4), 8);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c43cf0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_3cf0, &args![0x1000u32]).u32(), 0x0120_2de8);
        assert_eq!(e.call(0x00c4_3cf0, &args![0u32]).u32(), 0x0120_2de8);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    #[test]
    fn factory_and_constructor_build_a_value_node() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0xb8);
        e.mem.write(block, &[0xaa; 0xb8]);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, BS_NI_NODE_CONSTRUCT, 0);
        assert_eq!(e.call(0x00c4_3a30, &args![]).u32(), block);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0xb8]),
                (BS_NI_NODE_CONSTRUCT, vec![block])
            ]
        );
        assert_eq!(e.mem.u32(block), BS_VALUE_NODE_VTABLE);
        assert_eq!(e.mem.u32(block + VALUE), 0);
        assert_eq!(e.mem.u32(block + VALUE_TYPE), 0);
        assert_eq!(e.mem.u8(block + VALUE_NODE_FLAGS), 0xaa);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00c4_3a30, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_the_value_and_the_flags_of_newer_files() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_LOAD_BINARY, 0);
        let l = log.clone();
        e.register_double(STREAM_READ_WORD, move |e, a| {
            l.borrow_mut().push((STREAM_READ_WORD, vec![a[0]]));
            e.mem.set_u32(a[1], 0x1234);
            0u32.into_ret()
        });
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        let read = 0x00ee_0400u32;
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, read);
        e.register_double(read, |e, a| {
            e.mem.set_u8(a[1], 0x5);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(stream + STREAM_USER_VERSION, 0x18);
        e.call(0x00c4_38a0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + VALUE), 0x1234);
        assert_eq!(e.mem.u8(this + VALUE_NODE_FLAGS), 0);
        e.mem.set_u32(stream + STREAM_USER_VERSION, 0x19);
        e.call(0x00c4_38a0, &args![this, stream]);
        assert_eq!(e.mem.u8(this + VALUE_NODE_FLAGS), 5);
        assert_eq!(e.mem.u32(input + 4), 1);
        assert_eq!(log.borrow()[0], (NI_NODE_LOAD_BINARY, vec![this, stream]));
        assert_eq!(log.borrow()[1], (STREAM_READ_WORD, vec![stream]));
    }

    #[test]
    fn create_clone_copies_value_and_flags() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0xb8);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, BS_NI_NODE_CONSTRUCT, 0);
        logged(&mut e, &log, BS_NI_NODE_COPY_MEMBERS, 0);
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + VALUE, 77);
        e.mem.set_u8(this + VALUE_NODE_FLAGS, 3);
        e.mem.set_u32(this + VALUE_TYPE, 9);
        assert_eq!(e.call(0x00c4_37c0, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(e.mem.u32(block + VALUE), 77);
        assert_eq!(e.mem.u8(block + VALUE_NODE_FLAGS), 3);
        assert_eq!(e.mem.u32(block + VALUE_TYPE), 0);
        assert_eq!(
            log.borrow().last().unwrap(),
            &(BS_NI_NODE_COPY_MEMBERS, vec![this, block, 0x9000])
        );
        e.map(0, 0x100);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00c4_37c0, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(e.mem.u32(VALUE), 77);
    }

    #[test]
    fn update_world_data_fixes_the_transform_when_flagged() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, AV_OBJECT_UPDATE_WORLD_DATA, 0);
        e.map(FIXED_TRANSFORM, 0x24);
        for k in 0..9 {
            e.mem.set_u32(FIXED_TRANSFORM + 4 * k, 0x100 + k);
        }
        let this = e.mem.alloc(0xb8);
        e.call(0x00c4_3b90, &args![this, 0x77u32]);
        assert_eq!(
            *log.borrow(),
            [(AV_OBJECT_UPDATE_WORLD_DATA, vec![this, 0x77])]
        );
        assert_eq!(e.mem.u32(this + WORLD_TRANSFORM), 0);
        e.mem.set_u8(this + VALUE_NODE_FLAGS, 1);
        e.call(0x00c4_3b90, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(this + WORLD_TRANSFORM), 0x100);
        assert_eq!(e.mem.u32(this + WORLD_TRANSFORM + 0x20), 0x108);
        assert_eq!(e.mem.u32(this + WORLD_TRANSFORM + 0x24), 0);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BS_NI_NODE_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        assert_eq!(e.call(0x00c4_3cb0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        e.call(0x00c4_3cb0, &args![0x5000u32, 0u32]);
        assert_eq!(
            *log.borrow(),
            [
                (BS_NI_NODE_DESTRUCT, vec![0x5000]),
                (NI_SIZED_DELETE, vec![0x5000, 0xb8]),
                (BS_NI_NODE_DESTRUCT, vec![0x5000]),
            ]
        );
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_binary_unsigned_char_writes_one_byte_through_the_output_stream() {
        const WRITE: u32 = 0x00aa_2f00;
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 4, 10);
        e.mem.set_u32(output + 0xc, WRITE);
        let source = e.mem.alloc(4);
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], a[4]), (1, 1));
            assert_eq!(e.mem.u32(a[3]), 1);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a2_9550, &args![stream, source]);
        let log = e.call_log.take().unwrap();
        let writes: Vec<_> = log.iter().filter(|c| c.0 == WRITE).collect();
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0].1[..2], [output, source]);
        assert_eq!(e.mem.u32(output + 4), 11);
    }

    fn node_with_rtti(e: &mut Engine, rtti: u32, mode: u16) -> u32 {
        e.register(0x0040_5008, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x0040_5008);
        let node = e.mem.alloc(0xb8);
        e.mem.set_u32(node, vtable);
        e.mem.set_u32(node + 0x10, rtti);
        e.mem.set_u16(node + 0xac, mode);
        node
    }

    #[test]
    fn construct_from_a_billboard_node_in_mode_5_sets_the_flag() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BS_NI_NODE_CONSTRUCT_FROM, 0);
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent + 4, NI_BILLBOARD_NODE_RTTI);
        let billboard = node_with_rtti(&mut e, parent, 0xfd);
        let this = e.mem.alloc(0xb8);
        e.mem.set_u8(this + VALUE_NODE_FLAGS, 0x10);
        assert_eq!(e.call(0x00c4_36a0, &args![this, billboard]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_VALUE_NODE_VTABLE);
        assert_eq!(e.mem.u8(this + VALUE_NODE_FLAGS), 0x11);
        assert_eq!(
            calls(&log, BS_NI_NODE_CONSTRUCT_FROM),
            vec![vec![this, billboard, 0]]
        );
        // Another mode, another class or no node: the flag stays.
        for node in [
            node_with_rtti(&mut e, parent, 4),
            node_with_rtti(&mut e, 0, 5),
            0,
        ] {
            e.mem.set_u8(this + VALUE_NODE_FLAGS, 0x10);
            e.call(0x00c4_36a0, &args![this, node]);
            assert_eq!(e.mem.u8(this + VALUE_NODE_FLAGS), 0x10);
        }
    }

    #[test]
    fn register_streamables_answers_the_node_result() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_REGISTER_STREAMABLES, 0x100);
        assert!(!e.call(0x00c4_3920, &args![0x1000u32, 0x2000u32]).bool());
        logged(&mut e, &log, NI_NODE_REGISTER_STREAMABLES, 1);
        assert!(e.call(0x00c4_3920, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(
            calls(&log, NI_NODE_REGISTER_STREAMABLES)[0],
            vec![0x1000, 0x2000]
        );
    }

    #[test]
    fn save_binary_writes_the_value_and_the_flags() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_SAVE_BINARY, 0);
        e.register(STREAM_SAVE_WORD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1234);
            Ret::default()
        });
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x00aa_2f10);
        e.register(0x00aa_2f10, |e, a| {
            assert_eq!(e.mem.u8(a[1]), 3);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + VALUE, 0x1234);
        e.mem.set_u8(this + VALUE_NODE_FLAGS, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c4_3950, &args![this, stream]);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x00c4_3950,
                NI_NODE_SAVE_BINARY,
                STREAM_SAVE_WORD,
                0x00aa_2f10
            ]
        );
        assert_eq!(e.mem.u32(output + 4), 1);
    }

    #[test]
    fn is_equal_compares_value_and_flags_after_the_node() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_IS_EQUAL, 1);
        let this = e.mem.alloc(0xb8);
        let other = e.mem.alloc(0xb8);
        e.mem.set_u32(this + VALUE, 5);
        e.mem.set_u32(other + VALUE, 5);
        assert!(e.call(0x00c4_39b0, &args![this, other]).bool());
        e.mem.set_u8(other + VALUE_NODE_FLAGS, 1);
        assert!(!e.call(0x00c4_39b0, &args![this, other]).bool());
        e.mem.set_u8(this + VALUE_NODE_FLAGS, 1);
        e.mem.set_u32(other + VALUE, 6);
        assert!(!e.call(0x00c4_39b0, &args![this, other]).bool());
        e.mem.set_u32(other + VALUE, 5);
        logged(&mut e, &log, NI_NODE_IS_EQUAL, 0);
        assert!(!e.call(0x00c4_39b0, &args![this, other]).bool());
    }

    #[test]
    fn viewer_strings_add_name_value_and_billboard() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_GET_VIEWER_STRINGS, 0);
        e.map(BS_VALUE_NODE_RTTI, 4);
        e.set_global::<u32>(BS_VALUE_NODE_RTTI, 0x0101_0000);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x7001);
        logged(&mut e, &log, NI_GET_VIEWER_STRING_INT, 0x7002);
        logged(&mut e, &log, NI_GET_VIEWER_STRING_TEXT, 0x7003);
        let l = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            l.borrow_mut().push((0, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + VALUE, 42);
        e.call(0x00c4_3ab0, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&log, 0),
            vec![vec![0x6000, 0x7001], vec![0x6000, 0x7002]]
        );
        assert_eq!(calls(&log, NI_GET_VIEWER_STRING), vec![vec![0x0101_0000]]);
        assert_eq!(
            calls(&log, NI_GET_VIEWER_STRING_INT),
            vec![vec![VALUE_LABEL, 42]]
        );
        log.borrow_mut().clear();
        e.mem.set_u8(this + VALUE_NODE_FLAGS, 1);
        e.call(0x00c4_3ab0, &args![this, 0x6000u32]);
        assert_eq!(calls(&log, 0)[2], vec![0x6000, 0x7003]);
        assert_eq!(
            calls(&log, NI_GET_VIEWER_STRING_TEXT),
            vec![vec![BILLBOARD_LABEL, TRUE_TEXT]]
        );
        assert_eq!(
            calls(&log, NI_NODE_GET_VIEWER_STRINGS)[0],
            vec![this, 0x6000]
        );
    }

    fn parse(e: &mut Engine, text: &[u8]) -> (bool, u32) {
        let block = e.mem.alloc(0x40);
        // The byte before the name is read when the whole name is digits.
        e.mem.set_u8(block, b'x');
        e.mem.set_cstr(block + 1, text);
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + 8, block + 1);
        let found = e.call(0x00c4_3be0, &args![this]).bool();
        (found, e.mem.u32(this + VALUE))
    }

    /// The C runtime's "C" locale as the image holds it, for the real `atol`: the initial
    /// locale object (`011c2400`) points at a locale information block with `mb_cur_max` 1
    /// (+0xAC) and the C character class table (+0xC8); `__locale_changed` (`0126fdf4`) is 0.
    fn c_locale(e: &mut Engine) {
        e.map(0x011c_2000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(0x0126_fdf4, 0);
        let table = e.mem.alloc(0x200);
        for c in 0..=255u8 {
            let mut bits = 0u16;
            if c.is_ascii_uppercase() {
                bits |= 0x101;
            }
            if c.is_ascii_lowercase() {
                bits |= 0x102;
            }
            if c.is_ascii_digit() {
                bits |= 4;
            }
            if c == b' ' || (9..=13).contains(&c) {
                bits |= 8;
            }
            if c.is_ascii_punctuation() {
                bits |= 0x10;
            }
            if c < 0x20 || c == 0x7f {
                bits |= 0x20;
            }
            if c == b' ' {
                bits |= 0x40;
            }
            if c.is_ascii_hexdigit() {
                bits |= 0x80;
            }
            e.mem.set_u16(table + u32::from(c) * 2, bits);
        }
        let info = e.mem.alloc(0xd8);
        e.mem.set_u32(info + 0xac, 1);
        e.mem.set_u32(info + 0xc8, table);
        e.mem.set_u32(0x011c_2400, info);
    }

    #[test]
    fn parse_name_for_value_reads_the_trailing_number() {
        let mut e = Engine::new();
        c_locale(&mut e);
        assert_eq!(parse(&mut e, b"Node42"), (true, 42));
        assert_eq!(parse(&mut e, b"Node#7"), (true, 0));
        assert_eq!(parse(&mut e, b"A@12"), (true, 0));
        assert_eq!(parse(&mut e, b"Node"), (false, 0));
        assert_eq!(parse(&mut e, b"123"), (true, 123));
    }
}
