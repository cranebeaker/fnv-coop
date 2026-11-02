//! `BSMain/bsrangenode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c457d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e0c without reading the receiver. The map names this body `BSRangeNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c457d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e0c
}

/// `BSRangeNode`'s vtable (its class confirmed by the RTTI before it).
const BS_RANGE_NODE_VTABLE: u32 = 0x010c_1b2c;
/// `BSNiNode::BSNiNode` (`thiscall`, no arguments), the base `CopyMembers` (`00c5fb60`,
/// `thiscall`, `(destination, cloning)`), `NiNode::LoadBinary` and `NiNode::LinkObject`
/// (`thiscall`, `(stream)`) and `NiStreamLoadBinary<unsigned char>` (`cdecl`, `(stream,
/// &byte)`).
const BS_NI_NODE_CONSTRUCTOR: u32 = 0x00c5_f570;
const BASE_COPY_MEMBERS: u32 = 0x00c5_fb60;
const NI_NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
const NI_NODE_LINK_OBJECT: u32 = 0x00a5_e9d0;
const STREAM_LOAD_BYTE: u32 = 0x00a6_43b0;
/// The callback `SetCurrent` calls when the node's visibility changes (`cdecl`, `(node,
/// visible)`), null when none is set.
const RANGE_CHANGED_CALLBACK: u32 = 0x0120_2e08;

layout! {
    /// `BSRangeNode` (Xbox PDB), 0xB0 bytes on PC (its own fields 0x24 lower than the Xbox PDB says).
    pub struct BSRangeNode: 0xB0 {
        /// `NiAVObject::m_uFlags` (Xbox PDB): `SetCurrent` sets bit 0 when the index is out of range.
        0x30 m_uFlags: u32,
        /// `bEnabled` (Xbox PDB +0xD0).
        0xAC bEnabled: u8,
        /// `cCurrent` (Xbox PDB +0xD1).
        0xAD cCurrent: u8,
        /// `cMin` (Xbox PDB +0xD2): the lowest index shown.
        0xAE cMin: u8,
        /// `cMax` (Xbox PDB +0xD3): the highest index shown.
        0xAF cMax: u8,
    }
}

// Translated from 00c450d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::BSRangeNode` (Xbox PDB): `BSNiNode`'s constructor, the class's vtable, the
/// range 0..=255, current 0 and `bEnabled` true. Answers `this`.
pub fn bs_range_node_constructor(e: &mut Engine, this: Ptr<BSRangeNode>) -> Ptr<BSRangeNode> {
    e.call(BS_NI_NODE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), BS_RANGE_NODE_VTABLE);
    e.set(this, BSRangeNode::cMin, 0);
    e.set(this, BSRangeNode::cMax, 0xff);
    e.set(this, BSRangeNode::cCurrent, 0);
    e.set(this, BSRangeNode::bEnabled, 1);
    this
}

// Translated from 00c455a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::SetCurrent` (Xbox PDB): the node is to be shown when `index` lies in
/// `[min, max]`; when that differs from its shown state (bit 0 of the flags
/// clear), the bit is set (hidden) or cleared (shown) and the callback at `01202e08`, when
/// set, is called with the node and the new state. The index itself is not stored. Answers
/// the new state.
pub fn bs_range_node_set_current(e: &mut Engine, this: Ptr<BSRangeNode>, index: u8) -> bool {
    let shown = e.get(this, BSRangeNode::m_uFlags) & 1 == 0;
    let visible =
        !(index < e.get(this, BSRangeNode::cMin) || index > e.get(this, BSRangeNode::cMax));
    if shown != visible {
        let flags = e.get(this, BSRangeNode::m_uFlags);
        let flags = if visible { flags & !1 } else { flags | 1 };
        e.set(this, BSRangeNode::m_uFlags, flags);
        let callback = e.global::<u32>(RANGE_CHANGED_CALLBACK);
        if callback != 0 {
            e.call(callback, &args![this, u32::from(visible)]);
        }
    }
    visible
}

// Translated from 00c451b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::CopyMembers` (Xbox PDB), `(destination, cloning)`: the base's
/// `CopyMembers` (`00c5fb60`), the range copied, and `SetCurrent` on the destination with
/// this node's current index.
pub fn bs_range_node_copy_members(
    e: &mut Engine,
    this: Ptr<BSRangeNode>,
    destination: Ptr<BSRangeNode>,
    cloning: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, destination, cloning]);
    let low = e.get(this, BSRangeNode::cMin);
    e.set(destination, BSRangeNode::cMin, low);
    let high = e.get(this, BSRangeNode::cMax);
    e.set(destination, BSRangeNode::cMax, high);
    let current = e.get(this, BSRangeNode::cCurrent);
    bs_range_node_set_current(e, destination, current);
}

// Translated from 00c45220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::LoadBinary` (Xbox PDB): `NiNode::LoadBinary`, then three bytes from the
/// stream (through one local): the lowest and highest index, and the current index given to
/// `SetCurrent`.
pub fn bs_range_node_load_binary(e: &mut Engine, this: Ptr<BSRangeNode>, stream: Ptr) {
    e.call(NI_NODE_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, local| {
        e.call(STREAM_LOAD_BYTE, &args![stream, local]);
        let low = e.mem.u8(local.addr());
        e.set(this, BSRangeNode::cMin, low);
        e.call(STREAM_LOAD_BYTE, &args![stream, local]);
        let high = e.mem.u8(local.addr());
        e.set(this, BSRangeNode::cMax, high);
        e.call(STREAM_LOAD_BYTE, &args![stream, local]);
        let current = e.mem.u8(local.addr());
        bs_range_node_set_current(e, this, current);
    });
}

// Translated from 00c452a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode`'s `LinkObject` (unnamed in the map): `NiNode::LinkObject` with the stream.
pub fn fn_00c452a0(e: &mut Engine, this: Ptr<BSRangeNode>, stream: Ptr) {
    e.call(NI_NODE_LINK_OBJECT, &args![this, stream]);
}

/// `00aa13e0` (`cdecl`, one word): the Ni allocator; `BSRangeNode` is 0xB0 bytes.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const BS_RANGE_NODE_SIZE: u32 = 0xb0;
/// `NiNode::RegisterStreamables` (`00a5e5b0`), `NiNode::SaveBinary` (`00a5e880`),
/// `NiNode::IsEqual` (`00a5e600`) and `NiNode::GetViewerStrings` (`00a5ea70`), each
/// `thiscall` with one word.
const NI_NODE_REGISTER_STREAMABLES: u32 = 0x00a5_e5b0;
const NI_NODE_SAVE_BINARY: u32 = 0x00a5_e880;
const NI_NODE_IS_EQUAL: u32 = 0x00a5_e600;
const NI_NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
/// `NiStreamSaveBinary<unsigned char>` (`00a29550`, `cdecl`, `(stream, &byte)`).
const STREAM_SAVE_BYTE: u32 = 0x00a2_9550;
/// `NiGetViewerString`'s `(label, unsigned char)` form `00a7d410` and `(label, bool)` form
/// `00a7d370` (`cdecl`), and the viewer-string array's add (`0096a610`, `thiscall`,
/// `(&string)`).
const NI_GET_VIEWER_STRING_BYTE: u32 = 0x00a7_d410;
const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// The labels `"Min"`, `"Max"`, `"Current"` and `"Enabled"`, and the `"-"` the name parser
/// looks for.
const MIN_LABEL: u32 = 0x010c_1c40;
const MAX_LABEL: u32 = 0x010c_1c3c;
const CURRENT_LABEL: u32 = 0x010c_1c34;
const ENABLED_LABEL: u32 = 0x0109_c7d8;
const DASH: u32 = 0x0103_45e0;
/// `strlen`, `strstr` and `atol` (CRT, `cdecl`).
const STRLEN: u32 = 0x00ec_6130;
const STRSTR: u32 = 0x00ec_7750;
const ATOL: u32 = 0x00ec_a6d3;

// Translated from 00c45120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c45120 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e512, state variable) is not translated
/// `BSRangeNode::CreateClone` (Xbox PDB, engine map): allocates 0xB0 bytes, constructs a
/// node in it when that succeeds, copies the members into it (into address 0 when the
/// allocation failed) and returns it.
pub fn bs_range_node_create_clone(
    e: &mut Engine,
    this: Ptr<BSRangeNode>,
    cloning: Ptr,
) -> Ptr<BSRangeNode> {
    let block = e.call(NI_ALLOCATE, &args![BS_RANGE_NODE_SIZE]).u32();
    let node = if block == 0 {
        Ptr::new(0)
    } else {
        bs_range_node_constructor(e, Ptr::new(block))
    };
    bs_range_node_copy_members(e, this, node, cloning);
    node
}

// Translated from 00c452c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the `BSRangeNode` vtable's `RegisterStreamables` place):
/// whether `NiNode::RegisterStreamables` succeeded.
pub fn fn_00c452c0(e: &mut Engine, this: Ptr<BSRangeNode>, stream: Ptr) -> bool {
    e.call(NI_NODE_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        != 0
}

// Translated from 00c452f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::SaveBinary` (Xbox PDB, engine map): `NiNode::SaveBinary`, then the lowest,
/// highest and current index, each copied into one local byte and saved
/// (`NiStreamSaveBinary<unsigned char>`).
pub fn bs_range_node_save_binary(e: &mut Engine, this: Ptr<BSRangeNode>, stream: Ptr) {
    e.call(NI_NODE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, local| {
        for field in [BSRangeNode::cMin, BSRangeNode::cMax, BSRangeNode::cCurrent] {
            let byte = e.get(this, field);
            e.mem.set_u8(local.addr(), byte);
            e.call(STREAM_SAVE_BYTE, &args![stream, local]);
        }
    });
}

// Translated from 00c45380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::IsEqual` (Xbox PDB, engine map): `NiNode::IsEqual`, and when that holds,
/// whether the lowest, highest and current indices are equal.
pub fn bs_range_node_is_equal(
    e: &mut Engine,
    this: Ptr<BSRangeNode>,
    other: Ptr<BSRangeNode>,
) -> bool {
    if e.call(NI_NODE_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    e.get(this, BSRangeNode::cMin) == e.get(other, BSRangeNode::cMin)
        && e.get(this, BSRangeNode::cMax) == e.get(other, BSRangeNode::cMax)
        && e.get(this, BSRangeNode::cCurrent) == e.get(other, BSRangeNode::cCurrent)
}

// Translated from 00c45440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c45440 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e53f, state variable) is not translated
/// Unnamed in the engine map (the `BSRangeNode` factory, no arguments): allocates 0xB0
/// bytes and constructs a node in it when that succeeds. Returns it, or 0.
pub fn fn_00c45440(e: &mut Engine) -> Ptr<BSRangeNode> {
    let block = e.call(NI_ALLOCATE, &args![BS_RANGE_NODE_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    bs_range_node_constructor(e, Ptr::new(block))
}

// Translated from 00c454c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::GetViewerStrings` (Xbox PDB, engine map): `NiNode::GetViewerStrings`, then
/// `"Min"`, `"Max"` and `"Current"` with their indices and `"Enabled"` with whether
/// `bEnabled` is set, each added to the list.
pub fn bs_range_node_get_viewer_strings(e: &mut Engine, this: Ptr<BSRangeNode>, strings: Ptr) {
    e.call(NI_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(0x10, |e, slots| {
        let lines = [
            (MIN_LABEL, BSRangeNode::cMin, 0xc),
            (MAX_LABEL, BSRangeNode::cMax, 8),
            (CURRENT_LABEL, BSRangeNode::cCurrent, 4),
        ];
        for (label, field, offset) in lines {
            let value = u32::from(e.get(this, field));
            let text = e
                .call(NI_GET_VIEWER_STRING_BYTE, &args![label, value])
                .u32();
            let slot = slots.addr().wrapping_add(offset);
            e.mem.set_u32(slot, text);
            e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        }
        let enabled = u32::from(e.get(this, BSRangeNode::bEnabled) != 0);
        let text = e
            .call(NI_GET_VIEWER_STRING_BOOL, &args![ENABLED_LABEL, enabled])
            .u32();
        e.mem.set_u32(slots.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slots]);
    });
}

// Translated from 00c45670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::ParseNameForRanges` (Xbox PDB, engine map): scans the name (+8) back from
/// its end over digits and `-` (the character before the name's start is read when the
/// whole name is such characters). In the rest, `low-high` sets both bounds (`atol` of each
/// side present; missing sides stay 0 and 255) and a single number sets both to it; the
/// lower of the two (signed) becomes `cMin` and the higher `cMax`, as bytes.
pub fn bs_range_node_parse_name_for_ranges(e: &mut Engine, this: Ptr<BSRangeNode>) {
    let name = e.mem.u32(this.addr().wrapping_add(8));
    let length = e.call(STRLEN, &args![name]).u32();
    let mut back = 1u32;
    loop {
        let c = e.mem.i8(name.wrapping_add(length.wrapping_sub(back)));
        let marker = (0x30..=0x39).contains(&c) || c == 0x2d;
        if !marker || back > length {
            break;
        }
        back = back.wrapping_add(1);
    }
    let mut rest = name.wrapping_add(length.wrapping_sub(back)).wrapping_add(1);
    let mut low = 0i32;
    let mut high = 0xffi32;
    let dash = e.call(STRSTR, &args![rest, DASH]).u32();
    if dash != 0 {
        if dash != rest {
            low = e.call(ATOL, &args![rest]).i32();
        }
        rest = dash.wrapping_add(1);
        if e.mem.u8(rest) != 0 {
            high = e.call(ATOL, &args![rest]).i32();
        }
    } else if e.mem.u8(rest) != 0 {
        low = e.call(ATOL, &args![rest]).i32();
        high = low;
    }
    let lower = if low < high { low } else { high };
    e.set(this, BSRangeNode::cMin, lower as u8);
    let higher = if low > high { low } else { high };
    e.set(this, BSRangeNode::cMax, higher as u8);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4_5120,
            bs_range_node_create_clone(Ptr<BSRangeNode>, Ptr) -> Ptr<BSRangeNode>
        ),
        entry!(0x00c4_52c0, fn_00c452c0(Ptr<BSRangeNode>, Ptr) -> bool),
        entry!(
            0x00c4_52f0,
            bs_range_node_save_binary(Ptr<BSRangeNode>, Ptr)
        ),
        entry!(
            0x00c4_5380,
            bs_range_node_is_equal(Ptr<BSRangeNode>, Ptr<BSRangeNode>) -> bool
        ),
        entry!(0x00c4_5440, fn_00c45440() -> Ptr<BSRangeNode>),
        entry!(
            0x00c4_54c0,
            bs_range_node_get_viewer_strings(Ptr<BSRangeNode>, Ptr)
        ),
        entry!(
            0x00c4_5670,
            bs_range_node_parse_name_for_ranges(Ptr<BSRangeNode>)
        ),
        entry!(0x00c4_57d0, fn_00c457d0(Ptr) -> u32),
        entry!(
            0x00c450d0,
            bs_range_node_constructor(Ptr<BSRangeNode>) -> Ptr<BSRangeNode>
        ),
        entry!(
            0x00c455a0,
            bs_range_node_set_current(Ptr<BSRangeNode>, u8) -> bool
        ),
        entry!(
            0x00c451b0,
            bs_range_node_copy_members(Ptr<BSRangeNode>, Ptr<BSRangeNode>, Ptr)
        ),
        entry!(0x00c45220, bs_range_node_load_binary(Ptr<BSRangeNode>, Ptr)),
        entry!(0x00c452a0, fn_00c452a0(Ptr<BSRangeNode>, Ptr)),
    ]
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c457d0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_57d0, &args![0x1000u32]).u32(), 0x0120_2e0c);
        assert_eq!(e.call(0x00c4_57d0, &args![0u32]).u32(), 0x0120_2e0c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn range_node(e: &mut Engine, low: u8, high: u8, flags: u32) -> Ptr<BSRangeNode> {
        let this = e.new_object::<BSRangeNode>();
        e.set(this, BSRangeNode::cMin, low);
        e.set(this, BSRangeNode::cMax, high);
        e.set(this, BSRangeNode::m_uFlags, flags);
        this
    }

    #[test]
    fn constructor_sets_the_full_range() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BS_NI_NODE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<BSRangeNode>();
        e.set(this, BSRangeNode::cCurrent, 9);
        assert_eq!(e.call(0x00c450d0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BS_RANGE_NODE_VTABLE);
        assert_eq!(e.get(this, BSRangeNode::cMin), 0);
        assert_eq!(e.get(this, BSRangeNode::cMax), 0xff);
        assert_eq!(e.get(this, BSRangeNode::cCurrent), 0);
        assert_eq!(e.get(this, BSRangeNode::bEnabled), 1);
        assert_eq!(
            calls_at(&e, BS_NI_NODE_CONSTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn set_current_toggles_bit_0_and_calls_back_on_change() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(RANGE_CHANGED_CALLBACK, 4);
        let this = range_node(&mut e, 2, 4, 0x10);
        // Shown and in range: no change, no callback (none set anyway).
        assert!(e.call(0x00c455a0, &args![this, 3u32]).bool());
        assert_eq!(e.get(this, BSRangeNode::m_uFlags), 0x10);
        // Out of range below: hidden, no callback set.
        assert!(!e.call(0x00c455a0, &args![this, 1u32]).bool());
        assert_eq!(e.get(this, BSRangeNode::m_uFlags), 0x11);
        e.set_global::<u32>(RANGE_CHANGED_CALLBACK, 0x0090_0500);
        e.register(0x0090_0500, |_, _| Ret::default());
        // Back in range: shown, callback with 1.
        assert!(e.call(0x00c455a0, &args![this, 4u32]).bool());
        assert_eq!(e.get(this, BSRangeNode::m_uFlags), 0x10);
        // Above the range: hidden, callback with 0.
        assert!(!e.call(0x00c455a0, &args![this, 5u32]).bool());
        assert_eq!(
            calls_at(&e, 0x0090_0500),
            vec![vec![this.addr(), 1], vec![this.addr(), 0]]
        );
        assert_eq!(e.get(this, BSRangeNode::cCurrent), 0);
    }

    #[test]
    fn copy_members_copies_the_range_and_applies_the_current_index() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.map(RANGE_CHANGED_CALLBACK, 4);
        let this = range_node(&mut e, 1, 3, 0);
        e.set(this, BSRangeNode::cCurrent, 7);
        let destination = range_node(&mut e, 0, 0xff, 0);
        e.call(0x00c451b0, &args![this, destination, 0x5555u32]);
        assert_eq!(
            calls_at(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), destination.addr(), 0x5555]]
        );
        assert_eq!(e.get(destination, BSRangeNode::cMin), 1);
        assert_eq!(e.get(destination, BSRangeNode::cMax), 3);
        assert_eq!(e.get(destination, BSRangeNode::m_uFlags), 1);
    }

    #[test]
    fn load_binary_reads_the_range_and_the_current_index() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NI_NODE_LOAD_BINARY, |_, _| Ret::default());
        e.map(RANGE_CHANGED_CALLBACK, 4);
        let mut bytes = vec![9u8, 6, 5].into_iter();
        e.register_double(STREAM_LOAD_BYTE, move |e, a| {
            e.mem.set_u8(a[1], bytes.next_back().unwrap());
            Ret::default()
        });
        let this = range_node(&mut e, 0, 0xff, 0);
        e.call(0x00c45220, &args![this, 0x6000u32]);
        assert_eq!(e.get(this, BSRangeNode::cMin), 5);
        assert_eq!(e.get(this, BSRangeNode::cMax), 6);
        assert_eq!(e.get(this, BSRangeNode::m_uFlags), 1);
        let reads = calls_at(&e, STREAM_LOAD_BYTE);
        assert_eq!(reads.len(), 3);
        assert!(reads.iter().all(|r| r[0] == 0x6000 && r[1] == reads[0][1]));
        assert_eq!(
            calls_at(&e, NI_NODE_LOAD_BINARY),
            vec![vec![this.addr(), 0x6000]]
        );
    }

    #[test]
    fn link_object_forwards_to_ni_node() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NI_NODE_LINK_OBJECT, |_, _| Ret::default());
        e.call(0x00c452a0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls_at(&e, NI_NODE_LINK_OBJECT),
            vec![vec![0x5000, 0x6000]]
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

    fn node(e: &mut Engine, low: u8, high: u8, current: u8) -> Ptr<BSRangeNode> {
        let this = e.new_object::<BSRangeNode>();
        e.set(this, BSRangeNode::cMin, low);
        e.set(this, BSRangeNode::cMax, high);
        e.set(this, BSRangeNode::cCurrent, current);
        this
    }

    #[test]
    fn create_clone_and_factory_build_range_nodes() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BS_NI_NODE_CONSTRUCTOR, 0);
        logged(&mut e, &log, BASE_COPY_MEMBERS, 0);
        e.map(RANGE_CHANGED_CALLBACK, 4);
        let block = e.mem.alloc(0xb0);
        logged(&mut e, &log, NI_ALLOCATE, block);
        let this = node(&mut e, 2, 9, 4);
        let clone = e.call(0x00c4_5120, &args![this, 0x5555u32]).u32();
        assert_eq!(clone, block);
        assert_eq!(e.mem.u32(block), BS_RANGE_NODE_VTABLE);
        assert_eq!((e.mem.u8(block + 0xae), e.mem.u8(block + 0xaf)), (2, 9));
        assert_eq!(
            calls(&log, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), block, 0x5555]]
        );
        assert_eq!(e.call(0x00c4_5440, &args![]).u32(), block);
        assert_eq!(calls(&log, NI_ALLOCATE), vec![vec![0xb0], vec![0xb0]]);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00c4_5440, &args![]).u32(), 0);
        assert_eq!(calls(&log, BS_NI_NODE_CONSTRUCTOR).len(), 2);
    }

    #[test]
    fn register_streamables_answers_the_node_result() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_REGISTER_STREAMABLES, 0x200);
        assert!(!e.call(0x00c4_52c0, &args![0x1000u32, 0x2000u32]).bool());
        logged(&mut e, &log, NI_NODE_REGISTER_STREAMABLES, 2);
        assert!(e.call(0x00c4_52c0, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn save_binary_writes_min_max_current() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_SAVE_BINARY, 0);
        let l = log.clone();
        e.register_double(STREAM_SAVE_BYTE, move |e, a| {
            l.borrow_mut()
                .push((STREAM_SAVE_BYTE, vec![a[0], u32::from(e.mem.u8(a[1]))]));
            Ret::default()
        });
        let this = node(&mut e, 3, 7, 5);
        e.call(0x00c4_52f0, &args![this, 0x6000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (NI_NODE_SAVE_BINARY, vec![this.addr(), 0x6000]),
                (STREAM_SAVE_BYTE, vec![0x6000, 3]),
                (STREAM_SAVE_BYTE, vec![0x6000, 7]),
                (STREAM_SAVE_BYTE, vec![0x6000, 5]),
            ]
        );
    }

    #[test]
    fn is_equal_compares_the_three_indices() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_IS_EQUAL, 1);
        let this = node(&mut e, 1, 2, 3);
        let same = node(&mut e, 1, 2, 3);
        assert!(e.call(0x00c4_5380, &args![this, same]).bool());
        for other in [
            node(&mut e, 0, 2, 3),
            node(&mut e, 1, 0, 3),
            node(&mut e, 1, 2, 0),
        ] {
            assert!(!e.call(0x00c4_5380, &args![this, other]).bool());
        }
        logged(&mut e, &log, NI_NODE_IS_EQUAL, 0);
        assert!(!e.call(0x00c4_5380, &args![this, this]).bool());
    }

    #[test]
    fn viewer_strings_add_the_indices_and_enabled() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_GET_VIEWER_STRINGS, 0);
        e.register(NI_GET_VIEWER_STRING_BYTE, |_, a| (a[0] + a[1]).into_ret());
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, a| (a[0] + a[1]).into_ret());
        let l = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            l.borrow_mut().push((0, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        let this = node(&mut e, 1, 2, 3);
        e.set(this, BSRangeNode::bEnabled, 7);
        e.call(0x00c4_54c0, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&log, 0),
            vec![
                vec![0x6000, MIN_LABEL + 1],
                vec![0x6000, MAX_LABEL + 2],
                vec![0x6000, CURRENT_LABEL + 3],
                vec![0x6000, ENABLED_LABEL + 1],
            ]
        );
        e.set(this, BSRangeNode::bEnabled, 0);
        log.borrow_mut().clear();
        e.call(0x00c4_54c0, &args![this, 0x6000u32]);
        assert_eq!(calls(&log, 0)[3], vec![0x6000, ENABLED_LABEL]);
    }

    fn parse(e: &mut Engine, text: &[u8]) -> (u8, u8) {
        let block = e.mem.alloc(0x40);
        e.mem.set_u8(block, b'x');
        e.mem.set_cstr(block + 1, text);
        let this = node(e, 9, 9, 0);
        e.mem.set_u32(this.addr() + 8, block + 1);
        e.call(0x00c4_5670, &args![this]);
        (
            e.get(this, BSRangeNode::cMin),
            e.get(this, BSRangeNode::cMax),
        )
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
    fn parse_name_for_ranges_reads_bounds() {
        let mut e = Engine::new();
        c_locale(&mut e);
        e.register(STRSTR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let found = text.iter().position(|c| *c == b'-');
            found.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.map(DASH, 2);
        e.mem.set_cstr(DASH, b"-");
        assert_eq!(parse(&mut e, b"Range3-7"), (3, 7));
        assert_eq!(parse(&mut e, b"Range7-3"), (3, 7));
        assert_eq!(parse(&mut e, b"Range-4"), (0, 4));
        assert_eq!(parse(&mut e, b"Range5-"), (5, 255));
        assert_eq!(parse(&mut e, b"Range6"), (6, 6));
        assert_eq!(parse(&mut e, b"Range"), (0, 255));
        assert_eq!(parse(&mut e, b"2-300"), (2, 44));
    }
}
