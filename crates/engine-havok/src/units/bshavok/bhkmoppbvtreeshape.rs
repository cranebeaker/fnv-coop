//! `BSHavok/bhkmoppbvtreeshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::bhklimitedhingeconstraint::{
    allocate_sized, enter_scope, increment, push_viewer_string, stream_transfer, ALLOCATE,
    POOL_ALLOCATE, POOL_OBJECT, REMOVE_REFERENCE,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cc9d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d5_4960`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cc9d80(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d5_4960, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cc9d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppBvTreeShape::getContainer` (map name, taken from a vtable; the
/// body is only this jump). `__fastcall` tail jump: loads the object at
/// `this + 0x34` and jumps to its virtual slot `+0x10` with that object as
/// `this`; the caller's own stack words (`forwarded`) reach it unchanged
/// and its result is returned.
pub fn fn_00cc9d70(e: &mut Engine, this: Ptr, forwarded: &[u32]) -> Ret {
    let child = e.mem.u32(this.addr() + 0x34);
    e.vcall(child, 0x10, forwarded)
}

/// Uniform form of [`fn_00cc9d70`]: `this`, then the forwarded words.
fn fn_00cc9d70_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_00cc9d70(e, Ptr::new(words[0]), &words[1..])
}

// Translated from 00cc9db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8658.
pub fn bhk_mopp_bv_tree_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8658
}

/// `bhkMoppBvTreeShape` (0x14 bytes, vtable `010ca24c`; the Havok shape at `+8`, `pInfo` at
/// `+0xC`). Its Havok shape keeps a source-shape pointer at `+0x14` and the mopp code
/// at `+0x40`. `pInfo` is a 0x10-byte block: `+4` the child shape, `+8` the mopp code and
/// `+0xC` a scale `float`.
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_BV_TREE: u32 = 0x0104_dbac;
const VTABLE_MOPP: u32 = 0x010c_a24c;
const VTABLE_SCALED_MOPP: u32 = 0x010c_a330;
const VTABLE_MOPP_CODE: u32 = 0x0102_e368;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_BV_TREE: u32 = 0x0126_82cc;
const COUNT_MOPP: u32 = 0x0126_8654;
/// `bhkSerializable::Kill` (`00c876a0`), `LoadCInfo` (`00c87710`), `PostInit` (`00c876d0`),
/// `CloneCritLock` (`00c876f0`), `CloneCritUnlock` (`00c87700`) and the `bhkRefObject`
/// constructor (`00c87c30`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const SERIALIZABLE_LOAD_CINFO: u32 = 0x00c8_7710;
const SERIALIZABLE_POST_INIT: u32 = 0x00c8_76d0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkBvTreeShape` (the base): destructor (`00ca4da0`), `LinkObject` (`00ca4f00`),
/// `RegisterStreamables` (`00ca4f40`), `CopyCinfo` (`00ca4e10`), `CopyMembers`
/// (`00ca4e50`, `(clone, process_info)`), `LoadBinary` (`00ca4ec0`), `SaveBinary`
/// (`00ca4f80`) and `GetViewerStrings` (`00ca5190`).
const BV_TREE_DESTRUCT: u32 = 0x00ca_4da0;
const BV_TREE_LINK_OBJECT: u32 = 0x00ca_4f00;
const BV_TREE_REGISTER_STREAMABLES: u32 = 0x00ca_4f40;
const BV_TREE_COPY_CINFO: u32 = 0x00ca_4e10;
const BV_TREE_COPY_MEMBERS: u32 = 0x00ca_4e50;
const BV_TREE_LOAD_BINARY: u32 = 0x00ca_4ec0;
const BV_TREE_SAVE_BINARY: u32 = 0x00ca_4f80;
const BV_TREE_GET_VIEWER_STRINGS: u32 = 0x00ca_5190;
/// `bhkTransformShape::IsEqual` (`00c8bf40`, a jump to `00c84dc0`).
const SHAPE_IS_EQUAL: u32 = 0x00c8_bf40;
/// The Havok mopp shape base constructor (`00d544f0`, `(shape, block)`), the mopp build
/// settings initialiser (`00d54de0`, `ECX` = a 0x21-byte block) and the mopp code builder
/// (`00d54cd0`, cdecl `(shape data, settings, 0)`).
const MOPP_BASE_CONSTRUCT: u32 = 0x00d5_44f0;
const MOPP_CONFIG_INIT: u32 = 0x00d5_4de0;
const MOPP_BUILD_CODE: u32 = 0x00d5_4cd0;
/// Takes a reference (`00c90510`, `ECX` = the object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// The stream functions: `LoadBinary_ov3` (`00cf1270`, cdecl `(stream, slot)`) and
/// `SaveBinary_ov2` (`00cf0dc0`, cdecl `(stream, word)`).
const STREAM_LOAD_WORD: u32 = 0x00cf_1270;
const STREAM_SAVE_WORD: u32 = 0x00cf_0dc0;
/// `hkOstream`'s constructor (`00cc0500`, `(buffer, size, flag)`), its append (`00cc01c0`,
/// one string, answers the stream) and destructor (`00cc00e0`).
const OSTREAM_CONSTRUCT: u32 = 0x00cc_0500;
const OSTREAM_APPEND: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The global (`01268360`) holding the object whose slot `0xC` receives the message,
/// and the message's strings: "File " (`010c784c`), " re-export\n" (`010c7878`), the
/// text after the file name (`010ca3ac`), the fallback for an unnamed file ("Please",
/// `010bdd3c`) and the source file name (`010ca368`).
const MESSAGE_SINK: u32 = 0x0126_8360;
const TEXT_FILE: u32 = 0x010c_784c;
const TEXT_RE_EXPORT: u32 = 0x010c_7878;
const TEXT_OLD_SHAPE: u32 = 0x010c_a3ac;
const TEXT_UNNAMED: u32 = 0x010b_dd3c;
const TEXT_SOURCE: u32 = 0x010c_a368;
/// `NiGetViewerString` overloads (`00a7d330` for an RTTI name, `00a7d5e0` for a name and
/// text), `NiPoint3::GetViewerString` (`00a59af0`), the Havok vector conversion
/// (`00458620`, cdecl `(destination, source)`) and the formatter (`00406d00`).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const POINT_FROM_VECTOR: u32 = 0x0045_8620;
const FORMAT: u32 = 0x0040_6d00;
/// The class's RTTI record (`01268658`, whose first word is the name), the names
/// "MoppCode" (`010ca3e4`) and "MoppCode Offset" (`010ca3d4`) and the code format
/// `0x%8X` (`010c4430`).
const RTTI_MOPP: u32 = 0x0126_8658;
const NAME_CODE: u32 = 0x010c_a3e4;
const NAME_CODE_OFFSET: u32 = 0x010c_a3d4;
const FORMAT_CODE: u32 = 0x010c_4430;
/// The sized free (`00aa1460`, cdecl `(block, size)`).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// Size of the wrapper and of the pool-owned `pInfo` block.
const SHAPE_SIZE: u32 = 0x14;
const INFO_SIZE: u32 = 0x10;

/// Constructs a `bhkMoppBvTreeShape` in `object`: the `bhkRefObject` constructor
/// (`00c87c30`), the `bhkShape`, `bhkBvTreeShape` and `bhkMoppBvTreeShape` vtables in turn
/// with their counters (`012680f8`, `012682cc`, `01268654`) up by one, `+0xC` and `+0x10`
/// cleared.
fn construct(e: &mut Engine, object: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_SHAPE);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    increment(e, COUNT_SHAPE);
    e.mem.set_u32(object, VTABLE_BV_TREE);
    increment(e, COUNT_BV_TREE);
    e.mem.set_u32(object, VTABLE_MOPP);
    increment(e, COUNT_MOPP);
}

// Translated from 00cc9c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9c60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMoppBvTreeShape::~bhkMoppBvTreeShape` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill` (`00c876a0`), takes one off the instance counter `01268654`
/// and runs the `bhkBvTreeShape` destructor (`00ca4da0`).
pub fn bhk_mopp_bv_tree_shape_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_MOPP);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_MOPP);
    e.mem.set_u32(COUNT_MOPP, count.wrapping_sub(1));
    e.call(BV_TREE_DESTRUCT, &args![this]);
}

// Translated from 00cc9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::LinkObject` (Xbox PDB): a tail jump to `bhkBvTreeShape::LinkObject`
/// (`00ca4f00`) with `this` and the stream.
pub fn bhk_mopp_bv_tree_shape_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(BV_TREE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00cc9ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::RegisterStreamables` (Xbox PDB): a tail jump to
/// `bhkBvTreeShape::RegisterStreamables` (`00ca4f40`) with `this` and the stream. Answers what
/// it answers (`EAX` after the `JMP` at 00cc9ce0; the Xbox PDB's `RegisterStreamables` returns
/// `bool`): NiObject-level callers test it, e.g. `TEST AL,AL` at 00a66f78 after the slot +0x54
/// call at 00a66f76 in NiGeometryData::RegisterStreamables.
pub fn bhk_mopp_bv_tree_shape_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(BV_TREE_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00cc9cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::IsEqual` (Xbox PDB): a tail jump to `00c8bf40` with `this` and the
/// other object; answers what it answers (`EAX`).
pub fn bhk_mopp_bv_tree_shape_is_equal(e: &mut Engine, this: Ptr, other: u32) -> u32 {
    e.call(SHAPE_IS_EQUAL, &args![this, other]).u32()
}

// Translated from 00cc9d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9d00 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name: constructs a `hkScaledMoppBvTreeShape` (vtable `010ca330`, RTTI-confirmed)
/// in `this`: the Havok mopp shape base constructor (`00d544f0`, `(first, second)`) runs,
/// the vtable is set, `code` is stored at `+0x40` and, when it is not null, a reference
/// is taken on it (`00c90510`). Returns `this`.
pub fn fn_00cc9d00(e: &mut Engine, this: Ptr, first: u32, second: u32, code: u32) -> Ptr {
    e.call(MOPP_BASE_CONSTRUCT, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_SCALED_MOPP);
    e.mem.set_u32(this.addr() + 0x40, code);
    if code != 0 {
        e.call(ADD_REFERENCE, &args![code]);
    }
    this
}

// Translated from 00cc9dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::QCinfoSize` (Xbox PDB), vtable slot `0xA8`: answers 0x10.
pub fn bhk_mopp_bv_tree_shape_q_cinfo_size(_e: &mut Engine) -> u32 {
    0x10
}

// Translated from 00cc9dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// (`00cc9c60`) and, when bit 0 of `flags` is set, the sized delete (`00aa1460`,
/// `(this, 0x14)`). Answers `this`.
pub fn bhk_mopp_bv_tree_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00cc_9c60, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, SHAPE_SIZE]);
    }
    this
}

// Translated from 00cc9df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::CopyCinfo` (Xbox PDB), with the cinfo `info`: `bhkBvTreeShape`'s
/// `CopyCinfo` (`00ca4e10`) runs, then `info + 8` takes the mopp code (`+0x40` of the Havok
/// shape; 0 without one) and, when the Havok shape has a source pointer (`+0x14`) and a
/// mopp code, `info + 0xC` becomes the quotient of the `float`s at `+0x10` of each (an x87
/// divide, stored as a `float`).
pub fn bhk_mopp_bv_tree_shape_copy_cinfo(e: &mut Engine, this: Ptr, info: u32) {
    e.call(BV_TREE_COPY_CINFO, &args![this, info]);
    let me = this.addr();
    let shape = if me != 0 { e.mem.u32(me + 8) } else { 0 };
    let code = if shape != 0 {
        e.mem.u32(shape + 0x40)
    } else {
        0
    };
    e.mem.set_u32(info + 8, code);
    if shape != 0 && e.mem.u32(shape + 0x14) != 0 && e.mem.u32(shape + 0x40) != 0 {
        let source = e.mem.u32(shape + 0x14);
        let numerator = x87_quiet_bits(e.mem.u32(source + 0x10));
        let denominator = x87_quiet_bits(e.mem.u32(code + 0x10));
        let ratio = f64::from(f32::from_bits(numerator)) / f64::from(f32::from_bits(denominator));
        e.mem.set_u32(info + 0xc, (ratio as f32).to_bits());
    }
}

// Translated from 00cc9e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9e60 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name: allocates a 0x14-byte `bhkMoppBvTreeShape` (`00aa13e0`) and, when that
/// succeeded, constructs it (`bhkRefObject` constructor `00c87c30`, the three vtables with
/// their counters up by one, `+0xC` and `+0x10` cleared). Returns the object, or 0.
pub fn fn_00cc9e60(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![SHAPE_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    construct(e, object);
    object
}

// Translated from 00cc9f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::CopyMembers` (Xbox PDB), vtable slot `0xC4`: takes `pInfo` through
/// slot `0xB4`, multiplies its `float` at `+0xC` by the `float` at `+0x10` of
/// `process_info` (x87 multiply, stored as a `float`), takes a reference (`00c90510`) on
/// the object at its `+8` when there is one, and runs `bhkBvTreeShape::CopyMembers`
/// (`00ca4e50`) with `(clone, process_info)`.
pub fn bhk_mopp_bv_tree_shape_copy_members(
    e: &mut Engine,
    this: Ptr,
    clone: u32,
    process_info: u32,
) {
    let me = this.addr();
    let info = e.with_stack(4, |e, created| e.vcall(me, 0xb4, &args![created]).u32());
    let scale = x87_quiet_bits(e.mem.u32(process_info + 0x10));
    let current = x87_quiet_bits(e.mem.u32(info + 0xc));
    let product = f64::from(f32::from_bits(scale)) * f64::from(f32::from_bits(current));
    e.mem.set_u32(info + 0xc, (product as f32).to_bits());
    let object = e.mem.u32(info + 8);
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    e.call(BV_TREE_COPY_MEMBERS, &args![this, clone, process_info]);
}

// Translated from 00cc9f60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9f60 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkMoppBvTreeShape::LoadCInfo` (Xbox PDB), vtable slot `0xAC`, with the stream: asks
/// the cinfo size (slot `0xA8`) and takes `pInfo` (slot `0xB4`). For a stream version
/// (`+4`) below 6 the cinfo size minus 4 bytes are read raw into `pInfo` through the
/// reader at `stream + 0x24c` (slot `8`, 5 arguments) and the message "File <name at
/// stream + 0xE0> contains an old bhkMoppBvTreeShape! <file name at stream + 8, or
/// "Please"> re-export\n" is built in a 0x200-byte `hkOstream` and sent through slot `0xC`
/// of the object in `01268360` with `(1, 0x234f2250, text, source file, 0xBB)`. Otherwise
/// `bhkSerializable::LoadCInfo` (`00c87710`) runs and, unless the stream's byte at
/// `+0x1e5` is 1, the word at `pInfo + 0xC` is byte-swapped. Answers the cinfo size
/// (less 4 in the first case).
pub fn fn_00cc9f60(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    let me = this.addr();
    let size = e.vcall(me, 0xa8, &args![]).u32();
    let info = e.with_stack(4, |e, created| e.vcall(me, 0xb4, &args![created]).u32());
    if e.mem.u32(stream + 4) < 6 {
        let remaining = size.wrapping_sub(4);
        let io = e.mem.u32(stream + 0x24c);
        stream_transfer(e, io, 8, info, remaining, false);
        let name = stream + 0xe0;
        let file = if e.mem.u8(stream + 8) == 0 {
            TEXT_UNNAMED
        } else {
            stream + 8
        };
        e.with_stack(0x20c, |e, block| {
            let object = block.addr();
            let text = object + 0xc;
            // The flag word: the exe pushes ECX (00cc9ffb, what the reader's slot 8 left there)
            // and writes only its low byte (`MOV byte ptr [EAX],0x1`, 00cca010). `00cc0500`
            // passes the word on unread to `00d274c0`, which reads only that byte (`MOV DL,byte
            // ptr [ESP + 0xc]`); the upper bytes are never read, so only the low byte (1) counts.
            e.call(OSTREAM_CONSTRUCT, &args![object, text, 0x200u32, 1u32]);
            let mut chain = object;
            for part in [TEXT_FILE, name, TEXT_OLD_SHAPE, file, TEXT_RE_EXPORT] {
                chain = e.call(OSTREAM_APPEND, &args![chain, part]).u32();
            }
            let sink = e.mem.u32(MESSAGE_SINK);
            e.vcall(
                sink,
                0xc,
                &args![1u32, 0x234f_2250u32, text, TEXT_SOURCE, 0xbbu32],
            );
            e.call(OSTREAM_DESTRUCT, &args![object]);
        });
        remaining
    } else {
        e.call(SERIALIZABLE_LOAD_CINFO, &args![this, stream]);
        if e.mem.u8(stream + 0x1e5) != 1 {
            let word = e.mem.u32(info + 0xc);
            e.mem.set_u32(info + 0xc, word.swap_bytes());
        }
        size
    }
}

// Translated from 00cca0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::LoadBinary` (Xbox PDB), vtable slot `0x4C`: runs
/// `bhkBvTreeShape::LoadBinary` (`00ca4ec0`), takes `pInfo` through slot `0xB4`, has the
/// stream load one word into a local that starts at 0 (`00cf1270`, cdecl `(stream,
/// &word)`) and stores it at `pInfo + 8`.
pub fn bhk_mopp_bv_tree_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    e.call(BV_TREE_LOAD_BINARY, &args![this, stream]);
    let info = e.with_stack(4, |e, created| e.vcall(me, 0xb4, &args![created]).u32());
    let word = e.with_stack(4, |e, local| {
        e.call(STREAM_LOAD_WORD, &args![stream, local]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(info + 8, word);
}

// Translated from 00cca130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMoppBvTreeShape::SaveBinary` (Xbox PDB), vtable slot `0x58`: takes `pInfo` through
/// slot `0xB4`; when the stream's byte at `+0x1e4` is not 1 the word at `pInfo + 0xC` is
/// byte-swapped. Runs `bhkBvTreeShape::SaveBinary` (`00ca4f80`), saves the word at
/// `pInfo + 8` (`00cf0dc0`, cdecl `(stream, word)`) and calls slot `0xA4` (`00ccae90` in this vtable, unnamed in the map)
/// with the "was created" flag of `CreateCinfo` (the exe passes the whole word of that
/// local, whose upper bytes are unset in the exe; they are modeled as 0). If the cinfo existed before and the stream's
/// byte is not 1, the word at `+0xC` is swapped back.
pub fn bhk_mopp_bv_tree_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    let (info, created) = e.with_stack(4, |e, slot| {
        let info = e.vcall(me, 0xb4, &args![slot]).u32();
        (info, u32::from(e.mem.u8(slot.addr())))
    });
    let foreign = e.mem.u8(stream + 0x1e4) != 1;
    if foreign {
        let word = e.mem.u32(info + 0xc);
        e.mem.set_u32(info + 0xc, word.swap_bytes());
    }
    e.call(BV_TREE_SAVE_BINARY, &args![this, stream]);
    let code = e.mem.u32(info + 8);
    e.call(STREAM_SAVE_WORD, &args![stream, code]);
    e.vcall(me, 0xa4, &args![created]);
    if created == 0 && e.mem.u8(stream + 0x1e4) != 1 {
        let word = e.mem.u32(info + 0xc);
        e.mem.set_u32(info + 0xc, word.swap_bytes());
    }
}

// Translated from 00cca1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cca1b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMoppBvTreeShape::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the clone lock
/// (`00c876f0` / `00c87700`) allocates and constructs a `bhkMoppBvTreeShape` as `00cc9e60`
/// does, then calls slot `0xC4` (`CopyMembers`) of `this` with `(clone, process_info)`.
/// Answers the clone (0 when the allocation failed). The allocation-scope word is 0x10
/// meanwhile.
pub fn bhk_mopp_bv_tree_shape_create_clone(e: &mut Engine, this: Ptr, process_info: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_CRIT_LOCK, &args![]);
    let block = e.call(ALLOCATE, &args![SHAPE_SIZE]).u32();
    let clone = if block == 0 {
        0
    } else {
        construct(e, block);
        block
    };
    e.vcall(this.addr(), 0xc4, &args![clone, process_info]);
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cca2a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cca2a0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMoppBvTreeShape::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: the byte at `created`
/// is set to whether `pInfo` (`+0xC`) was empty. If so, a 0x10-byte block is taken from the
/// pool (`00aa3e40`) with `+0`, `+4` and `+8` cleared and the scale `float` 1.0 at `+0xC`
/// and becomes `pInfo` (0 when the allocation failed); when the Havok shape (`+8`)
/// exists `CopyCinfo` (`00cc9df0`) fills it. Answers `pInfo`. The allocation-scope word is
/// 0x10 meanwhile.
pub fn bhk_mopp_bv_tree_shape_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let me = this.addr();
    let was_empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created, was_empty as u8);
    if was_empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, INFO_SIZE]).u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            e.mem.set_u32(block + 0xc, 1.0f32.to_bits());
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block + 8, 0);
        }
        let has_shape = e.mem.u32(me + 8) != 0;
        e.mem.set_u32(me + 0xc, block);
        if has_shape {
            e.call(0x00cc_9df0, &args![this, block]);
        }
    }
    e.mem.set_u32(word, saved);
    e.mem.u32(me + 0xc)
}

// Translated from 00cca360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cca360 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// No map name: builds a 0x10-byte local cinfo (`+4` the Havok shape of `shape` (`+8`), `+8` the mopp code, `+0xC` 1.0).
/// The mopp code comes from the build settings (a local block set up by `00d54de0`;
/// its byte at `+0x1E` is set when the Havok shape's word at `+0xC` is 0xE) and the
/// builder (`00d54cd0` with the Havok shape's `+0x10` address, or 0, the settings and 0);
/// then slot `0xB0` (`Init`) of `this` runs with the local cinfo. The allocation-scope
/// word is 0x10 meanwhile.
pub fn fn_00cca360(e: &mut Engine, this: Ptr, shape: u32) {
    let (word, saved) = enter_scope(e);
    // The exe's frame (`AND ESP,0xfffffff0`, exception record, `SUB ESP,0x58`, four pushes):
    // 0x74 bytes up to the 16-byte boundary. The cinfo at +0x14 (00cca418), the build settings
    // at +0x24 (00cca3d6).
    e.with_stack(0x74 + 0x10, |e, raw| {
        let frame = ((raw.addr() + 4 + 0xf) & !0xf) - 4;
        let child = e.mem.u32(shape + 8);
        let info = frame + 0x14;
        e.mem.set_u32(info, 0);
        e.mem.set_u32(info + 4, child);
        e.mem.set_u32(info + 0xc, 1.0f32.to_bits());
        let config = frame + 0x24;
        e.call(MOPP_CONFIG_INIT, &args![config]);
        if e.mem.u32(child + 0xc) == 0xe {
            e.mem.set_u8(config + 0x1e, 1);
        }
        let data = if child != 0 { child + 0x10 } else { 0 };
        let code = e.call(MOPP_BUILD_CODE, &args![data, config, 0u32]).u32();
        e.mem.set_u32(info + 8, code);
        e.vcall(this.addr(), 0xb0, &args![info]);
    });
    e.mem.set_u32(word, saved);
}

// Translated from 00cca450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cca450 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMoppBvTreeShape::PostInit` (Xbox PDB), vtable slot `0xBC`, with the cinfo `info`:
/// releases the object at `info + 8` (`00c905b0`) when there is one, then runs
/// `bhkSerializable::PostInit` (`00c876d0`) with `info`. The allocation-scope word is 0x10
/// meanwhile.
pub fn bhk_mopp_bv_tree_shape_post_init(e: &mut Engine, this: Ptr, info: u32) {
    let (word, saved) = enter_scope(e);
    let object = e.mem.u32(info + 8);
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.call(SERIALIZABLE_POST_INIT, &args![this, info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cca4e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cca4e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMoppBvTreeShape::bhkMoppBvTreeShape` (Xbox PDB), with the wrapped `shape`: the
/// `bhkRefObject` constructor (`00c87c30`), the three vtables with their counters up by
/// one, `+0xC` and `+0x10` cleared, then `00cca360` with `shape`. Returns `this`.
pub fn bhk_mopp_bv_tree_shape_construct(e: &mut Engine, this: Ptr, shape: u32) -> Ptr {
    construct(e, this.addr());
    e.call(0x00cc_a360, &args![this, shape]);
    this
}

// Translated from 00cca570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cca570 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkMoppBvTreeShape::GetViewerStrings` (Xbox PDB), vtable slot `0x60`: runs
/// `bhkBvTreeShape::GetViewerStrings` (`00ca5190`), then adds to `strings` the class name
/// (the first word of the RTTI record at `01268658`, through `00a7d330`), the text "0x%8X"
/// of the Havok shape's word at `+0x14` (0 without a Havok shape) under the name
/// "MoppCode" (`00406d00` into a 0x20-byte buffer, then `00a7d5e0`), and the point built
/// from that word's `+0x10` (`00458620`) under "MoppCode Offset" (`00a59af0`). The
/// allocation-scope word is 0x10 meanwhile.
pub fn bhk_mopp_bv_tree_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(BV_TREE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(RTTI_MOPP);
    let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let shape = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    let source = if shape != 0 {
        e.mem.u32(shape + 0x14)
    } else {
        0
    };
    let text = e.with_stack(0x20, |e, buffer| {
        e.call(FORMAT, &args![buffer, 0x20u32, FORMAT_CODE, source]);
        e.call(VIEWER_STRING_TEXT, &args![NAME_CODE, buffer]).u32()
    });
    push_viewer_string(e, strings, text);
    let text = e.with_stack(12, |e, point| {
        e.call(POINT_FROM_VECTOR, &args![point, source.wrapping_add(0x10)]);
        e.call(POINT_GET_VIEWER_STRING, &args![point, NAME_CODE_OFFSET])
            .u32()
    });
    push_viewer_string(e, strings, text);
    e.mem.set_u32(word, saved);
}

// Translated from 00cca6f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cca6f0 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkMoppBvTreeShape::Init` (Xbox PDB), vtable slot `0xB0`, with the cinfo `info`: when
/// `info + 8` (the mopp code) is empty one is built from the child shape at `info + 4` as
/// `00cca360` does (settings `00d54de0`, byte `+0x1E` set when the shape's word at `+0xC`
/// is 0xE, builder `00d54cd0`). The code used is `info + 8` or the built one. A 0x30-byte
/// `hkpMoppCode` (vtable `0102e368`, RTTI-confirmed) is made: `+0x10` is the code's
/// 16-byte vector at `+0x10` multiplied lane by lane by `(s, s, s, 1/s)` for the scale `s`
/// at `info + 0xC` (the reciprocal an x87 divide stored as a `float`), `+0x20` and `+0x24`
/// are the code's, `+0x28` the code's `+0x24` with the top bit set, the byte `+0x2C` is 2
/// and the 16-bit word `+6` is 1. A 0x50-byte `hkScaledMoppBvTreeShape` is constructed
/// (`00cc9d00` with the child, the new code and the original code); the new code and the
/// built code (if any) are released (`00c905b0`), slot `0x8C` of `this` takes the shape,
/// which is then released, and slot `0xBC` (`PostInit`) runs with `info`. The
/// allocation-scope word is 0x10 meanwhile.
pub fn bhk_mopp_bv_tree_shape_init(e: &mut Engine, this: Ptr, info: u32) {
    let (word, saved) = enter_scope(e);
    let mut built = 0;
    if e.mem.u32(info + 8) == 0 {
        // The exe's frame (`AND ESP,0xfffffff0`, exception record, `SUB ESP,0x68`, four
        // pushes): 0x84 bytes up to the 16-byte boundary, the build settings at +0x34
        // (00cca767).
        built = e.with_stack(0x84 + 0x10, |e, raw| {
            let frame = ((raw.addr() + 4 + 0xf) & !0xf) - 4;
            let config = frame + 0x34;
            e.call(MOPP_CONFIG_INIT, &args![config]);
            let child = e.mem.u32(info + 4);
            if e.mem.u32(child + 0xc) == 0xe {
                e.mem.set_u8(config + 0x1e, 1);
            }
            let data = if child != 0 { child + 0x10 } else { 0 };
            e.call(MOPP_BUILD_CODE, &args![data, config, 0u32]).u32()
        });
    }
    let mut code = e.mem.u32(info + 8);
    if code == 0 {
        code = built;
    }
    let scale = e.mem.u32(info + 0xc);
    let inverse = (1.0f64 / f64::from(f32::from_bits(x87_quiet_bits(scale)))) as f32;
    let factors = [scale, scale, scale, inverse.to_bits()];
    let mut lanes = [0u32; 4];
    for (lane, factor) in factors.iter().enumerate() {
        let value = f32::from_bits(e.mem.u32(code + 0x10 + 4 * lane as u32));
        lanes[lane] = (value * f32::from_bits(*factor)).to_bits();
    }
    let block = allocate_sized(e, 0x30);
    let copy_high = e.mem.u32(code + 0x24);
    let copy_low = e.mem.u32(code + 0x20);
    e.mem.set_u32(block, VTABLE_MOPP_CODE);
    e.mem.set_u16(block + 6, 1);
    for (lane, value) in lanes.iter().enumerate() {
        e.mem.set_u32(block + 0x10 + 4 * lane as u32, *value);
    }
    e.mem.set_u32(block + 0x24, copy_high);
    e.mem.set_u32(block + 0x20, copy_low);
    e.mem.set_u32(block + 0x28, copy_high | 0x8000_0000);
    e.mem.set_u8(block + 0x2c, 2);
    let object = allocate_sized(e, 0x50);
    let child = e.mem.u32(info + 4);
    let shape = e
        .call(0x00cc_9d00, &args![object, child, block, code])
        .u32();
    e.call(REMOVE_REFERENCE, &args![block]);
    if built != 0 {
        e.call(REMOVE_REFERENCE, &args![built]);
    }
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(REMOVE_REFERENCE, &args![shape]);
    e.vcall(this.addr(), 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc9db0, bhk_mopp_bv_tree_shape_get_r_t_t_i() -> u32),
        (0x00cc9d70, fn_00cc9d70_entry as AbiFn),
        entry!(0x00cc_9c60, bhk_mopp_bv_tree_shape_destruct(Ptr)),
        entry!(0x00cc_9cd0, bhk_mopp_bv_tree_shape_link_object(Ptr, u32)),
        entry!(
            0x00cc_9ce0,
            bhk_mopp_bv_tree_shape_register_streamables(Ptr, u32) -> u32
        ),
        entry!(0x00cc_9cf0, bhk_mopp_bv_tree_shape_is_equal(Ptr, u32) -> u32),
        entry!(0x00cc_9d00, fn_00cc9d00(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00cc_9dc0, bhk_mopp_bv_tree_shape_q_cinfo_size() -> u32),
        entry!(0x00cc_9dd0, bhk_mopp_bv_tree_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc_9df0, bhk_mopp_bv_tree_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00cc_9e60, fn_00cc9e60() -> u32),
        entry!(
            0x00cc_9f10,
            bhk_mopp_bv_tree_shape_copy_members(Ptr, u32, u32)
        ),
        entry!(0x00cc_9f60, fn_00cc9f60(Ptr, u32) -> u32),
        entry!(0x00cc_a0e0, bhk_mopp_bv_tree_shape_load_binary(Ptr, u32)),
        entry!(0x00cc_a130, bhk_mopp_bv_tree_shape_save_binary(Ptr, u32)),
        entry!(0x00cc_a1b0, bhk_mopp_bv_tree_shape_create_clone(Ptr, u32) -> u32),
        entry!(0x00cc_a2a0, bhk_mopp_bv_tree_shape_create_cinfo(Ptr, u32) -> u32),
        entry!(0x00cc_a360, fn_00cca360(Ptr, u32)),
        entry!(0x00cc_a450, bhk_mopp_bv_tree_shape_post_init(Ptr, u32)),
        entry!(0x00cc_a4e0, bhk_mopp_bv_tree_shape_construct(Ptr, u32) -> Ptr),
        entry!(
            0x00cc_a570,
            bhk_mopp_bv_tree_shape_get_viewer_strings(Ptr, u32)
        ),
        entry!(0x00cc_a6f0, bhk_mopp_bv_tree_shape_init(Ptr, u32)),
        entry!(0x00cc_9d80, deleting_destructor_00cc9d80(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00cc9d80_frees_only_when_bit_zero_is_set() {
        check(0x00cc_9d80, 0x00d5_4960);
    }

    #[test]
    fn fn_00cc9d70_jumps_to_slot_0x10_of_the_object_at_0x34() {
        let mut e = Engine::new();
        e.put_vtable(0x0090_2000, &[0, 0, 0, 0, 0x0090_5000]);
        e.register(0x0090_5000, |_, a| Ret {
            eax: a[0] + a.len() as u32,
            ..Ret::default()
        });
        let child = e.mem.alloc(8);
        e.mem.set_u32(child, 0x0090_2000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, child);
        assert_eq!(e.call(0x00cc_9d70, &args![this]).u32(), child + 1);
    }

    #[test]
    fn bhk_mopp_bv_tree_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_9db0, &args![]).u32(), 0x0126_8658);
    }
}

#[cfg(test)]
mod shape_tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::rc::Rc;

    const STRINGS_SET_AT: u32 = 0x0096_ae90;

    /// An object with a Havok shape (`+8`) of 0x50 bytes.
    fn wrapper(e: &mut Engine, slots: &[(u32, u32)]) -> (u32, u32) {
        let this = vobject(e, 0x14, slots);
        let shape = e.mem.alloc(0x50);
        e.mem.set_u32(this + 8, shape);
        (this, shape)
    }

    #[test]
    fn destruct_sets_the_vtable_kills_and_counts_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_MOPP, 2u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, BV_TREE_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x14);
        e.call(0x00cc_9c60, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_MOPP);
        assert_eq!(e.global::<u32>(COUNT_MOPP), 1);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, BV_TREE_DESTRUCT]);
        assert_eq!(calls_to(&log, BV_TREE_DESTRUCT), [[this]]);
    }

    #[test]
    fn register_streamables_answers_what_the_jump_target_answers() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, BV_TREE_REGISTER_STREAMABLES, 0x101, &log);
        // EAX after the `JMP` at 00cc9ce0 is the target's.
        assert_eq!(e.call(0x00cc_9ce0, &args![0x11u32, 0x21u32]).u32(), 0x101);
    }

    #[test]
    fn the_tail_jumps_forward_their_words() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, BV_TREE_LINK_OBJECT, 0, &log);
        record(&mut e, BV_TREE_REGISTER_STREAMABLES, 0, &log);
        record(&mut e, SHAPE_IS_EQUAL, 0x1_0001, &log);
        e.call(0x00cc_9cd0, &args![0x10u32, 0x20u32]);
        e.call(0x00cc_9ce0, &args![0x11u32, 0x21u32]);
        assert_eq!(calls_to(&log, BV_TREE_LINK_OBJECT), [[0x10, 0x20]]);
        assert_eq!(calls_to(&log, BV_TREE_REGISTER_STREAMABLES), [[0x11, 0x21]]);
        assert_eq!(
            e.call(0x00cc_9cf0, &args![0x12u32, 0x22u32]).u32(),
            0x1_0001
        );
        assert_eq!(calls_to(&log, SHAPE_IS_EQUAL), [[0x12, 0x22]]);
    }

    #[test]
    fn fn_00cc9d00_constructs_the_scaled_shape_and_takes_a_reference() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, MOPP_BASE_CONSTRUCT, 0, &log);
        record(&mut e, ADD_REFERENCE, 0, &log);
        let this = e.mem.alloc(0x50);
        assert_eq!(
            e.call(0x00cc_9d00, &args![this, 1u32, 2u32, 0x7000u32])
                .u32(),
            this
        );
        assert_eq!(calls_to(&log, MOPP_BASE_CONSTRUCT), [[this, 1, 2]]);
        assert_eq!(e.mem.u32(this), VTABLE_SCALED_MOPP);
        assert_eq!(e.mem.u32(this + 0x40), 0x7000);
        assert_eq!(calls_to(&log, ADD_REFERENCE), [[0x7000]]);
        e.call(0x00cc_9d00, &args![this, 1u32, 2u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(calls_to(&log, ADD_REFERENCE).len(), 1);
    }

    #[test]
    fn q_cinfo_size_answers_0x10() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_9dc0, &args![0u32]).u32(), 0x10);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, 0x00cc_9c60, 0, &log);
        record(&mut e, SIZED_DELETE, 0, &log);
        assert_eq!(e.call(0x00cc_9dd0, &args![0x40u32, 0xfffe_u32]).u32(), 0x40);
        assert!(calls_to(&log, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00cc_9dd0, &args![0x40u32, 1u32]).u32(), 0x40);
        assert_eq!(calls_to(&log, SIZED_DELETE), [[0x40, 0x14]]);
        assert_eq!(calls_to(&log, 0x00cc_9c60), [[0x40], [0x40]]);
    }

    #[test]
    fn copy_cinfo_takes_the_code_and_divides_the_floats() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, BV_TREE_COPY_CINFO, 0, &log);
        let (this, shape) = wrapper(&mut e, &[]);
        let source = e.mem.alloc(0x20);
        let code = e.mem.alloc(0x20);
        e.mem.set_f32(source + 0x10, 3.0);
        e.mem.set_f32(code + 0x10, 2.0);
        e.mem.set_u32(shape + 0x14, source);
        e.mem.set_u32(shape + 0x40, code);
        let info = e.mem.alloc(0x10);
        e.call(0x00cc_9df0, &args![this, info]);
        assert_eq!(calls_to(&log, BV_TREE_COPY_CINFO), [[this, info]]);
        assert_eq!(e.mem.u32(info + 8), code);
        assert_eq!(e.mem.f32(info + 0xc), 1.5);
        // Without a source pointer only the code is copied.
        e.mem.set_f32(info + 0xc, 9.0);
        e.mem.set_u32(shape + 0x14, 0);
        e.call(0x00cc_9df0, &args![this, info]);
        assert_eq!(e.mem.f32(info + 0xc), 9.0);
        // Without a Havok shape the code is 0.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00cc_9df0, &args![this, info]);
        assert_eq!(e.mem.u32(info + 8), 0);
    }

    #[test]
    fn fn_00cc9e60_builds_the_object_and_counts_it() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + 0xc, 3);
        e.mem.set_u32(object + 0x10, 4);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        assert_eq!(e.call(0x00cc_9e60, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x14]]);
        assert_eq!(e.mem.u32(object), VTABLE_MOPP);
        assert_eq!((e.mem.u32(object + 0xc), e.mem.u32(object + 0x10)), (0, 0));
        assert_eq!(e.global::<u32>(COUNT_SHAPE), 1);
        assert_eq!(e.global::<u32>(COUNT_BV_TREE), 1);
        assert_eq!(e.global::<u32>(COUNT_MOPP), 1);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_9e60, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn copy_members_scales_the_cinfo_and_takes_a_reference() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, ADD_REFERENCE, 0, &log);
        record(&mut e, BV_TREE_COPY_MEMBERS, 0, &log);
        let info = e.mem.alloc(0x10);
        e.mem.set_f32(info + 0xc, 4.0);
        e.mem.set_u32(info + 8, 0x7000);
        e.register_double(0x0900_0040, move |_, _| ret(info));
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_0040)]);
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 0.5);
        e.call(0x00cc_9f10, &args![this, 0x8000u32, process]);
        assert_eq!(e.mem.f32(info + 0xc), 2.0);
        assert_eq!(calls_to(&log, ADD_REFERENCE), [[0x7000]]);
        assert_eq!(
            calls_to(&log, BV_TREE_COPY_MEMBERS),
            [[this, 0x8000, process]]
        );
        e.mem.set_u32(info + 8, 0);
        e.call(0x00cc_9f10, &args![this, 0x8000u32, process]);
        assert_eq!(calls_to(&log, ADD_REFERENCE).len(), 1);
        assert_eq!(e.mem.f32(info + 0xc), 1.0);
    }

    #[test]
    fn load_cinfo_reads_a_new_cinfo_and_swaps_a_foreign_word() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_LOAD_CINFO, 0, &log);
        let info = e.mem.alloc(0x10);
        record(&mut e, 0x0900_0044, 0x10, &log);
        e.register_double(0x0900_0048, move |_, _| ret(info));
        let this = vobject(&mut e, 0x14, &[(0xa8, 0x0900_0044), (0xb4, 0x0900_0048)]);
        let stream = stream_fixture(&mut e, &log);
        e.mem.set_u32(stream + 4, 6);
        e.mem.set_u32(info + 0xc, 0x0102_0304);
        assert_eq!(e.call(0x00cc_9f60, &args![this, stream]).u32(), 0x10);
        assert_eq!(calls_to(&log, SERIALIZABLE_LOAD_CINFO), [[this, stream]]);
        assert_eq!(e.mem.u32(info + 0xc), 0x0102_0304);
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cc_9f60, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0xc), 0x0403_0201);
    }

    #[test]
    fn load_cinfo_of_an_old_stream_reads_raw_and_reports_the_message() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        e.map(0x0126_8000, 0x1000);
        record(&mut e, SERIALIZABLE_LOAD_CINFO, 0, &log);
        let info = e.mem.alloc(0x10);
        record(&mut e, 0x0900_0044, 0x10, &log);
        e.register_double(0x0900_0048, move |_, _| ret(info));
        let this = vobject(&mut e, 0x14, &[(0xa8, 0x0900_0044), (0xb4, 0x0900_0048)]);
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x24c);
        e.mem.set_u32(stream + 4, 5);
        record(&mut e, OSTREAM_CONSTRUCT, 0, &log);
        let chain = Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = Rc::clone(&chain);
        e.register_double(OSTREAM_APPEND, move |_, a| {
            seen.borrow_mut().push((a[0], a[1]));
            ret(a[0])
        });
        record(&mut e, OSTREAM_DESTRUCT, 0, &log);
        let sink = vobject(&mut e, 0x10, &[(0xc, 0x0900_004c)]);
        record(&mut e, 0x0900_004c, 0, &log);
        e.set_global(MESSAGE_SINK, sink);
        assert_eq!(e.call(0x00cc_9f60, &args![this, stream]).u32(), 0xc);
        // The cinfo is read raw (size - 4 bytes, 5 words).
        let reads = calls_to(&log, 0x0900_0008);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0], [io, info, 0xc, 0, 0]);
        assert_eq!(e.mem.u32(io + 4), 0xc);
        assert!(calls_to(&log, SERIALIZABLE_LOAD_CINFO).is_empty());
        // The message: "File ", the name, the middle text, "Please" (no file name), the end.
        let parts: Vec<u32> = chain.borrow().iter().map(|c| c.1).collect();
        assert_eq!(
            parts,
            [
                TEXT_FILE,
                stream + 0xe0,
                TEXT_OLD_SHAPE,
                TEXT_UNNAMED,
                TEXT_RE_EXPORT
            ]
        );
        let constructed = calls_to(&log, OSTREAM_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0][2], 0x200);
        // Only the flag word's low byte is written (and read) in the exe.
        assert_eq!(constructed[0][3] & 0xff, 1);
        assert_eq!(constructed[0][1], constructed[0][0] + 0xc);
        let sent = calls_to(&log, 0x0900_004c);
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0][0], sink);
        assert_eq!(sent[0][1..3], [1, 0x234f_2250]);
        assert_eq!(sent[0][3], constructed[0][1]);
        assert_eq!(sent[0][4..], [TEXT_SOURCE, 0xbb]);
        assert_eq!(calls_to(&log, OSTREAM_DESTRUCT), [[constructed[0][0]]]);
        // A named file is used as is.
        e.mem.set_u8(stream + 8, b'x');
        e.call(0x00cc_9f60, &args![this, stream]);
        assert_eq!(chain.borrow()[8].1, stream + 8);
    }

    #[test]
    fn load_binary_loads_the_word_into_the_cinfo() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, BV_TREE_LOAD_BINARY, 0, &log);
        let info = e.mem.alloc(0x10);
        e.register_double(0x0900_0048, move |_, _| ret(info));
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_0048)]);
        e.register_double(STREAM_LOAD_WORD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 0x6600);
            Ret::default()
        });
        e.call(0x00cc_a0e0, &args![this, 0x1234u32]);
        assert_eq!(calls_to(&log, BV_TREE_LOAD_BINARY), [[this, 0x1234]]);
        assert_eq!(e.mem.u32(info + 8), 0x6600);
    }

    #[test]
    fn save_binary_swaps_around_the_save_and_kills_a_temporary_cinfo() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, BV_TREE_SAVE_BINARY, 0, &log);
        record(&mut e, STREAM_SAVE_WORD, 0, &log);
        record(&mut e, 0x0900_0050, 0, &log);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 8, 0x7700);
        e.mem.set_u32(info + 0xc, 0x0102_0304);
        let created = Rc::new(std::cell::Cell::new(1u8));
        let flag = Rc::clone(&created);
        e.register_double(0x0900_0048, move |e, a| {
            e.mem.set_u8(a[1], flag.get());
            ret(info)
        });
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_0048), (0xa4, 0x0900_0050)]);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call(0x00cc_a130, &args![this, stream]);
        assert_eq!(calls_to(&log, BV_TREE_SAVE_BINARY), [[this, stream]]);
        assert_eq!(calls_to(&log, STREAM_SAVE_WORD), [[stream, 0x7700]]);
        assert_eq!(calls_to(&log, 0x0900_0050), [[this, 1]]);
        assert_eq!(e.mem.u32(info + 0xc), 0x0102_0304);
        // A foreign stream: swapped for the save and, for an existing cinfo, swapped back.
        e.mem.set_u8(stream + 0x1e4, 0);
        created.set(0);
        let seen = Rc::new(std::cell::Cell::new(0u32));
        let at_save = Rc::clone(&seen);
        e.register_double(BV_TREE_SAVE_BINARY, move |e, _| {
            at_save.set(e.mem.u32(info + 0xc));
            Ret::default()
        });
        e.call(0x00cc_a130, &args![this, stream]);
        assert_eq!(seen.get(), 0x0403_0201);
        assert_eq!(calls_to(&log, 0x0900_0050)[1], [this, 0]);
        assert_eq!(e.mem.u32(info + 0xc), 0x0102_0304);
        // A temporary cinfo is not swapped back.
        created.set(1);
        e.call(0x00cc_a130, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0xc), 0x0403_0201);
    }

    #[test]
    fn create_clone_copies_the_members_inside_the_lock() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        let clone = e.mem.alloc(0x14);
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, clone, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, 0x0900_0054, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0xc4, 0x0900_0054)]);
        assert_eq!(e.call(0x00cc_a1b0, &args![this, 0x55u32]).u32(), clone);
        assert_eq!(e.mem.u32(clone), VTABLE_MOPP);
        assert_eq!(calls_to(&log, 0x0900_0054), [[this, clone, 0x55]]);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                0x0900_0054,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(e.mem.u32(word), 0x77);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_a1b0, &args![this, 0x56u32]).u32(), 0);
        assert_eq!(calls_to(&log, 0x0900_0054)[1], [this, 0, 0x56]);
    }

    #[test]
    fn create_cinfo_builds_the_block_once_and_fills_it() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        let block = e.mem.alloc(0x10);
        for offset in 0..4 {
            e.mem.set_u32(block + 4 * offset, 0xdead);
        }
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, 0x00cc_9df0, 0, &log);
        let (this, _) = wrapper(&mut e, &[]);
        e.mem.set_u32(this + 0xc, 0);
        let created = e.mem.alloc(4);
        assert_eq!(e.call(0x00cc_a2a0, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, 0x10]]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(block + 4 * i)).collect();
        assert_eq!(words, [0, 0, 0, 0x3f80_0000]);
        assert_eq!(calls_to(&log, 0x00cc_9df0), [[this, block]]);
        assert_eq!(e.call(0x00cc_a2a0, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 8, 0);
        record(&mut e, POOL_ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_a2a0, &args![this, created]).u32(), 0);
        assert_eq!(calls_to(&log, 0x00cc_9df0).len(), 1);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn fn_00cca360_builds_the_code_and_runs_init_with_a_stack_cinfo() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, MOPP_CONFIG_INIT, 0, &log);
        record(&mut e, MOPP_BUILD_CODE, 0x7000, &log);
        let seen = Rc::new(std::cell::RefCell::new(Vec::new()));
        let record_info = Rc::clone(&seen);
        e.register_double(0x0900_0058, move |e, a| {
            let words: Vec<u32> = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            record_info.borrow_mut().push(words);
            Ret::default()
        });
        let this = vobject(&mut e, 0x14, &[(0xb0, 0x0900_0058)]);
        let (wrapper_object, shape) = wrapper(&mut e, &[]);
        e.mem.set_u32(shape + 0xc, 0xe);
        e.call(0x00cc_a360, &args![this, wrapper_object]);
        assert_eq!(*seen.borrow(), [[0, shape, 0x7000, 0x3f80_0000]]);
        let builds = calls_to(&log, MOPP_BUILD_CODE);
        assert_eq!(builds.len(), 1);
        assert_eq!((builds[0][0], builds[0][2]), (shape + 0x10, 0));
        // The settings block has its byte at +0x1E set for the type 0xE.
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn fn_00cca360_keeps_the_cinfo_and_the_settings_at_the_exe_frame_offsets() {
        let (mut e, _, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, MOPP_CONFIG_INIT, 0, &log);
        record(&mut e, MOPP_BUILD_CODE, 0x7000, &log);
        record(&mut e, 0x0900_0058, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0xb0, 0x0900_0058)]);
        let (wrapper_object, shape) = wrapper(&mut e, &[]);
        e.mem.set_u32(shape + 0xc, 0xe);
        e.call(0x00cc_a360, &args![this, wrapper_object]);
        // The frame's top (+0x74) is on a 16-byte boundary: the cinfo at +0x14 sits on one,
        // the settings at +0x24 0x10 above it, with the byte +0x1e set.
        let info = calls_to(&log, 0x0900_0058)[0][1];
        assert_eq!(info & 0xf, 0);
        assert_eq!(calls_to(&log, MOPP_CONFIG_INIT), [[info + 0x10]]);
        assert_eq!(calls_to(&log, MOPP_BUILD_CODE)[0][1], info + 0x10);
        assert_eq!(e.mem.u8(info + 0x10 + 0x1e), 1);
    }

    #[test]
    fn init_keeps_the_settings_at_the_exe_frame_offset() {
        let (mut e, _, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, MOPP_CONFIG_INIT, 0, &log);
        let code = e.mem.alloc(0x30);
        record(&mut e, MOPP_BUILD_CODE, code, &log);
        record(&mut e, 0x00cc_9d00, 0x8800, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, 0x0900_0060, 0, &log);
        record(&mut e, 0x0900_0064, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0x8c, 0x0900_0060), (0xbc, 0x0900_0064)]);
        let child = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 4, child);
        e.mem.set_f32(info + 0xc, 1.0);
        let raw = e.next_stack_local(0x94);
        e.call(0x00cc_a6f0, &args![this, info]);
        // Frame +0x34, with the frame's top (+0x84) on a 16-byte boundary.
        let frame = ((raw + 4 + 0xf) & !0xf) - 4;
        assert_eq!(calls_to(&log, MOPP_CONFIG_INIT), [[frame + 0x34]]);
        assert_eq!((frame + 0x84) & 0xf, 0);
    }

    #[test]
    fn post_init_releases_the_cinfo_object_and_runs_the_base() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, SERIALIZABLE_POST_INIT, 0, &log);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 8, 0x7000);
        e.call(0x00cc_a450, &args![0x44u32, info]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x7000]]);
        assert_eq!(calls_to(&log, SERIALIZABLE_POST_INIT), [[0x44, info]]);
        e.mem.set_u32(info + 8, 0);
        e.call(0x00cc_a450, &args![0x44u32, info]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE).len(), 1);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn construct_builds_the_wrapper_and_runs_the_builder() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let log: Calls = Rc::default();
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, 0x00cc_a360, 0, &log);
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00cc_a4e0, &args![this, 0x9000u32]).u32(), this);
        assert_eq!(calls_to(&log, 0x00cc_a360), [[this, 0x9000]]);
        assert_eq!(e.mem.u32(this), VTABLE_MOPP);
        assert_eq!(e.global::<u32>(COUNT_SHAPE), 1);
        assert_eq!(e.global::<u32>(COUNT_BV_TREE), 1);
        assert_eq!(e.global::<u32>(COUNT_MOPP), 1);
    }

    #[test]
    fn get_viewer_strings_adds_the_name_the_code_and_the_offset() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        e.set_global(RTTI_MOPP, 0x9100u32);
        let log: Calls = Rc::default();
        record(&mut e, BV_TREE_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, VIEWER_STRING_NAME, 0x3000, &log);
        record(&mut e, FORMAT, 0, &log);
        record(&mut e, VIEWER_STRING_TEXT, 0x3001, &log);
        record(&mut e, POINT_FROM_VECTOR, 0, &log);
        record(&mut e, POINT_GET_VIEWER_STRING, 0x3002, &log);
        e.register_double(STRINGS_SET_AT, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(0x0900_1000 + 4 * a[1], text);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.map(0x0900_1000, 0x100);
        let (this, shape) = wrapper(&mut e, &[]);
        e.mem.set_u32(shape + 0x14, 0x5000);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00cc_a570, &args![this, strings]);
        assert_eq!(
            calls_to(&log, BV_TREE_GET_VIEWER_STRINGS),
            [[this, strings]]
        );
        assert_eq!(calls_to(&log, VIEWER_STRING_NAME), [[0x9100]]);
        let formats = calls_to(&log, FORMAT);
        assert_eq!(formats[0][1..], [0x20, FORMAT_CODE, 0x5000]);
        let texts = calls_to(&log, VIEWER_STRING_TEXT);
        assert_eq!((texts[0][0], texts[0][1]), (NAME_CODE, formats[0][0]));
        let conversions = calls_to(&log, POINT_FROM_VECTOR);
        assert_eq!(conversions[0][1], 0x5010);
        assert_eq!(
            calls_to(&log, POINT_GET_VIEWER_STRING),
            [[conversions[0][0], NAME_CODE_OFFSET]]
        );
        let pushed: Vec<u32> = (0..3).map(|i| e.mem.u32(0x0900_1000 + 4 * i)).collect();
        assert_eq!(pushed, [0x3000, 0x3001, 0x3002]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn init_builds_the_scaled_code_and_hands_it_to_the_shape() {
        let (mut e, word, block) = engine_with_allocator();
        // The allocator hands out `block` for both blocks; give the second its own.
        let second = e.mem.alloc(0x100);
        let allocated = Rc::new(std::cell::Cell::new(0u32));
        let counter = Rc::clone(&allocated);
        e.register_double(0x0900_0004, move |_, _| {
            counter.set(counter.get() + 1);
            ret(if counter.get() == 1 { block } else { second })
        });
        let log: Calls = Rc::default();
        record(&mut e, MOPP_CONFIG_INIT, 0, &log);
        record(&mut e, MOPP_BUILD_CODE, 0x7000, &log);
        record(&mut e, 0x00cc_9d00, 0x8800, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, 0x0900_0060, 0, &log);
        record(&mut e, 0x0900_0064, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0x8c, 0x0900_0060), (0xbc, 0x0900_0064)]);
        let code = e.mem.alloc(0x30);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(code + 0x10 + 4 * lane as u32, value);
        }
        e.mem.set_u32(code + 0x20, 0x1111);
        e.mem.set_u32(code + 0x24, 0x2222);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 4, 0x6000);
        e.mem.set_u32(info + 8, code);
        e.mem.set_f32(info + 0xc, 4.0);
        e.call(0x00cc_a6f0, &args![this, info]);
        // The code is multiplied by (s, s, s, 1/s).
        let lanes: Vec<f32> = (0..4).map(|i| e.mem.f32(block + 0x10 + 4 * i)).collect();
        assert_eq!(lanes, [4.0, 8.0, 12.0, 1.0]);
        assert_eq!(e.mem.u32(block), VTABLE_MOPP_CODE);
        assert_eq!(e.mem.u16(block + 4), 0x30);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block + 0x20), 0x1111);
        assert_eq!(e.mem.u32(block + 0x24), 0x2222);
        assert_eq!(e.mem.u32(block + 0x28), 0x8000_2222);
        assert_eq!(e.mem.u8(block + 0x2c), 2);
        assert_eq!(e.mem.u16(second + 4), 0x50);
        assert_eq!(calls_to(&log, 0x00cc_9d00), [[second, 0x6000, block, code]]);
        assert!(calls_to(&log, MOPP_BUILD_CODE).is_empty());
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[block], [0x8800]]);
        assert_eq!(calls_to(&log, 0x0900_0060), [[this, 0x8800]]);
        assert_eq!(calls_to(&log, 0x0900_0064), [[this, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn init_builds_the_code_when_the_cinfo_has_none() {
        let (mut e, _, block) = engine_with_allocator();
        let second = e.mem.alloc(0x100);
        let allocated = Rc::new(std::cell::Cell::new(0u32));
        let counter = Rc::clone(&allocated);
        e.register_double(0x0900_0004, move |_, _| {
            counter.set(counter.get() + 1);
            ret(if counter.get() == 1 { block } else { second })
        });
        let log: Calls = Rc::default();
        record(&mut e, MOPP_CONFIG_INIT, 0, &log);
        let code = e.mem.alloc(0x30);
        e.mem.set_f32(code + 0x10, 2.0);
        record(&mut e, MOPP_BUILD_CODE, code, &log);
        record(&mut e, 0x00cc_9d00, 0x8800, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, 0x0900_0060, 0, &log);
        record(&mut e, 0x0900_0064, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0x8c, 0x0900_0060), (0xbc, 0x0900_0064)]);
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child + 0xc, 0xe);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 4, child);
        e.mem.set_f32(info + 0xc, 0.5);
        e.call(0x00cc_a6f0, &args![this, info]);
        let builds = calls_to(&log, MOPP_BUILD_CODE);
        assert_eq!(builds.len(), 1);
        assert_eq!((builds[0][0], builds[0][2]), (child + 0x10, 0));
        assert_eq!(e.mem.f32(block + 0x10), 1.0);
        assert_eq!(e.mem.f32(block + 0x1c), 0.0);
        // The built code is released after the new code.
        assert_eq!(
            calls_to(&log, REMOVE_REFERENCE),
            [[block], [code], [0x8800]]
        );
        assert_eq!(calls_to(&log, 0x00cc_9d00), [[second, child, block, code]]);
    }
}
