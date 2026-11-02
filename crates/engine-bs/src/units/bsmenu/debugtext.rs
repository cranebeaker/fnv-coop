//! `BSMenu/debugtext.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `DebugText` is the singleton (`011f33a8`, [`debug_text_instance`]) behind
//! the on-screen debug text and the console's printing. Its layout on PC:
//!
//! - `+0x04`: 200 [`DebugTextData`] entries of 0x2C bytes (the text drawn at
//!   a position; the Xbox PDB has 300 of them),
//! - `+0x2264`: the list of floating texts (an `NiTPointerList` of
//!   `DebugTextData`, [`fn_00a10c00`] adds to it),
//! - `+0x2270`: the queue of prints made since the last frame, a
//!   `BSSimpleArray` of [`SPrintData`] pointers that
//!   [`debug_text_handle_queued_prints`] drains,
//! - `+0x2280`: the font height, `+0x2284`: three line heights, `+0x2290`:
//!   three horizontal positions.
//!
//! Conventions of this file:
//!
//! - The exe never clears an entry's `spNode` by hand: every release goes
//!   through the `NiPointer` assignment at `0066b0d0` ([`set_node`]).
//! - `PrintX`, `PrintLine` and their `C` variants are `cdecl` variable-
//!   argument methods: `this` is the first stack word, the format string and
//!   the variable arguments follow. Their typed forms take the variable
//!   words as a slice and copy them into game memory for the `va_list` the
//!   virtual `VAPrintX` / `VAPrintLine` receive; the registered forms are
//!   [`print_x_entry`] and the others below.
//! - Compiler-only gaps stay open: C++ exception unwinding frames
//!   (`FS:[0]`, marked per function) and the `/GS` security cookie checks.
//! - Names the decompiler or the engine map gets wrong or leaves empty:
//!   `00a11d20` appends one pointer to the print queue (the same body is
//!   shared with another `BSSimpleArray` of pointers), `00a11c40` removes an
//!   element from the floating text list, `00a0df80` is the per-frame update
//!   (called from `Idle`).
//! - Unconfirmed: the meaning of the byte at `011c70eb` (it selects the
//!   `1.33333` horizontal scale in [`debug_text_create_debug_text`] and the
//!   wider line break width in [`debug_text_handle_queued_prints`]) and of
//!   the value `0xd` the code puts into the per-thread word at
//!   `TLS + 0x2b4` while it allocates.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSStringT, NiColorA, NiTPointerList};
use crate::units::platform::MEMORY_MANAGER;

layout! {
    /// `DebugText::DebugTextData` (Xbox PDB), 0x2C bytes: the text drawn at
    /// one position. On PC the float ones of the floating texts are used the
    /// same way (the node of the text is `spNode`).
    pub struct DebugTextData: 0x2c {
        /// `fX` (Xbox PDB).
        0x00 fX: f32,
        /// `fY` (Xbox PDB).
        0x04 fY: f32,
        /// `iJustify` (Xbox PDB): 0 marks a free entry of the table.
        0x08 iJustify: i32,
        /// `spNode` (Xbox PDB): `NiPointer<NiNode>`.
        0x0C spNode: Ptr,
        /// `pText` (Xbox PDB).
        0x10 pText: Inline<BSStringT>,
        /// `fDie` (Xbox PDB): seconds left; negative: never dies.
        0x18 fDie: f32,
        /// `Color` (Xbox PDB).
        0x1C Color: Inline<NiColorA>,
    }

    /// `DebugText::sPrintData` (Xbox PDB), 0x18 bytes: one queued print.
    pub struct SPrintData: 0x18 {
        /// `pTextData` (Xbox PDB).
        0x00 pTextData: Ptr<DebugTextData>,
        /// `eType` (Xbox PDB): 0 sets the text of the entry `iIndex`, 1
        /// prints at a position.
        0x04 eType: u32,
        /// `iIndex` (Xbox PDB).
        0x08 iIndex: i32,
        /// `iLineNumber` (Xbox PDB).
        0x0C iLineNumber: i32,
        /// `fDie` (Xbox PDB).
        0x10 fDie: f32,
        /// `iFont` (Xbox PDB).
        0x14 iFont: i32,
    }

    /// `DebugText` (Xbox PDB), 0x229C bytes on PC (0x33CC on the Xbox,
    /// which has 300 entries). The vtable is at +0.
    pub struct DebugText: 0x229c {
        /// `pDebugTextList` (Xbox PDB): the first of 200 entries.
        0x04 pDebugTextList: Inline<DebugTextData>,
        /// `xFloatingTextList` (Xbox PDB): `NiTList<DebugTextData *>`.
        0x2264 xFloatingTextList: Inline<NiTPointerList>,
        /// `xPrintData` (Xbox PDB): `BSSimpleArray<sPrintData *, 1024>`.
        0x2270 xPrintData: Inline<BSSimpleArray>,
        /// `iFontHeight` (Xbox PDB).
        0x2280 iFontHeight: i32,
        /// `iHeight` (Xbox PDB): three `i32`, one per line column.
        0x2284 iHeight: i32,
        /// `iXLocation` (Xbox PDB): three `i32`.
        0x2290 iXLocation: i32,
    }

    /// `DebugTextExtraData` (Xbox PDB), 0x20 bytes: the extra data a node
    /// carries to say it holds a floating text.
    pub struct DebugTextExtraData: 0x20 {
        /// `pBillboard` (Xbox PDB): `NiNode *`.
        0x0C pBillboard: Ptr,
        /// `xString` (Xbox PDB).
        0x10 xString: Inline<BSStringT>,
        /// `iHeight` (Xbox PDB).
        0x18 iHeight: i32,
        /// `iOldCenter` (Xbox PDB).
        0x1C iOldCenter: i32,
    }
}

/// Number of entries of `pDebugTextList` on PC.
const ENTRY_COUNT: u32 = 200;
/// Size of a [`DebugTextData`].
const ENTRY_SIZE: u32 = 0x2c;

/// The `DebugText` singleton pointer.
const INSTANCE: u32 = 0x011f_33a8;
/// `DebugText`'s own vtable, and the `IDebugText` one (every slot a pure
/// call) the constructor stores first.
const VTABLE: u32 = 0x0109_4efc;
const INTERFACE_VTABLE: u32 = 0x0109_4f30;
/// Vtable of the `BSSimpleArray<sPrintData *, 1024>` and of
/// `DebugTextExtraData`.
const PRINT_ARRAY_VTABLE: u32 = 0x0109_504c;
const EXTRA_DATA_VTABLE: u32 = 0x0109_4fb4;
/// The `RTTI` records: `DebugTextExtraData`'s (what its `GetRTTI` returns)
/// and `011f4428`, the base record of `BSTempNode` and `BSPortalSharedNode`
/// (the node class; its name is not confirmed here).
const EXTRA_DATA_RTTI: u32 = 0x011f_33d4;
const NODE_RTTI: u32 = 0x011f_4428;
/// The critical section `Lock` / `Unlock` take (only while the debug text is
/// visible).
const CRITICAL_SECTION: u32 = 0x011f_33dc;
/// Import slots of `EnterCriticalSection` / `LeaveCriticalSection` and
/// `InterlockedDecrement` (the names are Ghidra's).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `MemoryManager::Allocate` / `Deallocate` (Xbox PDB) on `MEMORY_MANAGER`.
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
/// `NiMemObject`-style `operator new(size)` and `operator delete(block,
/// size)` (both `cdecl`).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const DELETE_OBJECT: u32 = 0x00aa_1460;
/// `NiPointer` assignment: `0066b0d0(slot, value)` (thiscall on the slot).
const SET_NODE: u32 = 0x0066_b0d0;
/// `BSStringT<char>::Set(this, text, 0)` (Xbox PDB name `Set`), the copy
/// `Set(this, other string)` (`00438470`) and `StrCmp(this, text, 1)`
/// (`00408a80`).
const STRING_SET: u32 = 0x0040_37f0;
const STRING_COPY: u32 = 0x0043_8470;
const STRING_COMPARE: u32 = 0x0040_8a80;
/// `BSStringT<char>` constructor from a C string: `0040c0e0(this, text)`.
const STRING_FROM_TEXT: u32 = 0x0040_c0e0;
/// `Interface::` getters (all without arguments).
const GET_DEBUG_TEXT_ROOT: u32 = 0x0070_6d10;
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
const GET_FULL_HELP: u32 = 0x0070_3150;
const IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
const IS_DEBUG_TEXT_VISIBLE: u32 = 0x0070_2fc0;
/// `Interface::GetScreenHeight`: a `float` in ST0.
const GET_SCREEN_HEIGHT: u32 = 0x0070_6df0;
/// `Font::MakeString` (Xbox PDB), `Font::LineBreakText` (Xbox PDB).
const MAKE_STRING: u32 = 0x00a1_2460;
const LINE_BREAK_TEXT: u32 = 0x00a1_60c0;
/// `NiAVObject::GetProperty` / `UpdateProperties` / `Update` and
/// `AttachProperty` (Xbox PDB names), `NiObjectNET::SetName` (`00a5b950`,
/// takes the address of an `NiFixedString`), `NiObjectNET::AddExtraData_ov2`.
const GET_PROPERTY: u32 = 0x00a5_9d30;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE: u32 = 0x00a5_9c60;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const SET_NAME: u32 = 0x00a5_b950;
const ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// `NiFixedString` creation from a C string (`00a5b690`, cdecl, returns the
/// handle) and the destructor-like release `004381d0(&handle)` (cdecl).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const FIXED_STRING_RELEASE: u32 = 0x0043_81d0;
/// The empty `NiFixedString` handle.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `NiNode` constructor (`this`, 0) and the class that `0051cf00` constructs
/// (the floating text's parent node, 0xB4 bytes).
const NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
const FLOATING_NODE_CONSTRUCTOR: u32 = 0x0051_cf00;
/// The alpha-property-like object `004391c0` constructs (0x1C bytes) and the
/// two setters `00633ba0(this, a, b)` apply to it.
const PROPERTY_CONSTRUCTOR: u32 = 0x0043_91c0;
const PROPERTY_SETTER: u32 = 0x0063_3ba0;
/// `NiUpdateData`-like constructor `0043d410(this, 0.0, 0, 0)`.
const UPDATE_DATA_CONSTRUCTOR: u32 = 0x0043_d410;
/// `NiMatrix3::operator*` (`this`, result, other) returning the result.
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `BSShaderManager::PrepareObject(object, 0, 0)` (cdecl).
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// `NiTPointerList::AddTail(this, &element)` and `RemoveNode(this, &node)`,
/// `BSSimpleArray::Add`'s growth helper `00a11ee0`, `Clear(this, 1)`
/// (`00a1bfa0`), the array constructor body `00afcde0(this, 0, 0)`.
const LIST_ADD_TAIL: u32 = 0x00a1_1c00;
const LIST_REMOVE_NODE: u32 = 0x00a1_1e10;
const ARRAY_GROW: u32 = 0x00a1_1ee0;
const ARRAY_CLEAR: u32 = 0x00a1_bfa0;
const ARRAY_CONSTRUCTOR: u32 = 0x00af_cde0;
/// `NiExtraData` constructor / destructor bodies.
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// The C runtime: `_vsnprintf(buffer, size, format, va_list)`, `strlen`,
/// `strcmp` (Ghidra's name for `00ec6da0`), `strncpy`, `fabs(double)`,
/// `_ftol2` and `memset`.
const VSNPRINTF: u32 = 0x00ec_ddf1;
const STRLEN: u32 = 0x00ec_6130;
const STRCMP: u32 = 0x00ec_6da0;
const STRNCPY: u32 = 0x00ec_c7b0;
const FABS: u32 = 0x00ec_6cde;
const FTOL: u32 = 0x00ec_62c0;
const MEMSET: u32 = 0x00ec_61c0;
/// The EH vector constructor / destructor iterators (`cdecl`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The code that reports an error to the console / log: `005b5e40(text)`.
const REPORT_ERROR: u32 = 0x005b_5e40;
/// `"DEFAULT: Too many unique debug text items. \n"`.
const TOO_MANY_ITEMS_TEXT: u32 = 0x0109_4f60;
/// `"FloatingText"`, the name of the floating text's node.
const FLOATING_TEXT_NAME: u32 = 0x0109_4f9c;

/// Constants and settings the code reads from the exe: `-1.0f`, `0.0` as a
/// `double`, `255.0` (`double`), `640.0` (`double`), `1.33333f`, `2.0`
/// (`double`) and the `float` at `0104f7c4`.
const MINUS_ONE: u32 = 0x0101_2054;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const COLOR_DIVISOR: u32 = 0x0101_e568;
const SCREEN_HALF_WIDTH: u32 = 0x0107_4b88;
const WIDE_SCALE: u32 = 0x0109_4f98;
const TWO_DOUBLE: u32 = 0x0101_1590;
const TEXT_DEPTH: u32 = 0x0104_f7c4;
/// The angles of the floating text's rotations (`float`: `0101ff38`,
/// `01016b78`, `0102b3c8`) and `0.5f` (`01016248`).
const ANGLE_FIRST: u32 = 0x0101_ff38;
const ANGLE_SECOND: u32 = 0x0101_6b78;
const ANGLE_THIRD: u32 = 0x0102_b3c8;
const HALF: u32 = 0x0101_6248;
/// The `double` `Reset` adds to the font's line height, and the `1200.0`
/// scale `HandleQueuedPrints` multiplies the wrap width with.
const LINE_HEIGHT_BIAS: u32 = 0x0102_1928;
const WRAP_WIDTH_SCALE: u32 = 0x0109_4f90;
/// Seconds of the last frame (`float`).
const FRAME_TIME: u32 = 0x011f_63a0;
/// The settings: `iDebugTextFont` (value at `011f33cc`, record `011f33c8`),
/// `xDebugTextColor` (value `011f33c0`: four bytes), `011debb0` (added to the
/// line heights), `011deec4` (the left margin).
const FONT_SETTING_VALUE: u32 = 0x011f_33cc;
const COLOR_SETTING_VALUE: u32 = 0x011f_33c0;
const LINE_HEIGHT_SETTING_VALUE: u32 = 0x011d_ebb0;
const LEFT_MARGIN_SETTING_VALUE: u32 = 0x011d_eec4;
/// The `FontManager` (`011f33f8`): the `Font` pointers from `+0`, font `n`
/// at `+4 * (n - 1)`.
const FONT_MANAGER: u32 = 0x011f_33f8;
/// The byte at `011c70eb` (unconfirmed meaning, see the module note).
const WIDE_FLAG: u32 = 0x011c_70eb;
/// The world bound used when a node has none (`011f4288`).
const DEFAULT_BOUND: u32 = 0x011f_4288;
/// The color of the floating text (`011a9be0`, four floats).
const FLOATING_TEXT_COLOR: u32 = 0x011a_9be0;

/// Per-thread word at `TLS + 0x2b4` and the value the code stores in it while
/// it allocates.
const ALLOCATION_TAG_OFFSET: u32 = 0x2b4;
const ALLOCATION_TAG: u32 = 0xd;

/// Entry `index` of the table of 200 entries.
fn entry_at(this: Ptr<DebugText>, index: u32) -> Ptr<DebugTextData> {
    this.at(DebugText::pDebugTextList).index(index, ENTRY_SIZE)
}

/// Address of the line-height `column` (0..3).
fn height_address(this: Ptr<DebugText>, column: i32) -> u32 {
    this.addr()
        .wrapping_add(DebugText::iHeight.off)
        .wrapping_add((column as u32).wrapping_mul(4))
}

/// Address of the horizontal position `column` (0..3).
fn x_location_address(this: Ptr<DebugText>, column: i32) -> u32 {
    this.addr()
        .wrapping_add(DebugText::iXLocation.off)
        .wrapping_add((column as u32).wrapping_mul(4))
}

fn allocate_block(e: &mut Engine, size: u32) -> u32 {
    e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

fn free_block(e: &mut Engine, block: u32) {
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// `NiPointer` assignment to the slot at `slot`.
fn set_node(e: &mut Engine, slot: u32, node: u32) {
    e.call(SET_NODE, &args![slot, node]);
}

/// `BSStringT::Set(string, text, 0)`.
fn string_set(e: &mut Engine, string: u32, text: u32) {
    e.call(STRING_SET, &args![string, text, 0u32]);
}

/// Length of a `BSStringT`: `sLen`, or `strlen` when it is `0xffff`.
fn string_length(e: &mut Engine, string: Ptr<BSStringT>) -> u32 {
    let length = e.get(string, BSStringT::sLen);
    if length == 0xffff {
        let text = e.get(string, BSStringT::pString);
        e.call(STRLEN, &args![text]).u32()
    } else {
        length as u32
    }
}

fn debug_text_root(e: &mut Engine) -> u32 {
    e.call(GET_DEBUG_TEXT_ROOT, &[]).u32()
}

/// Removes `node` from the debug text root (virtual slot `+0xe8`).
fn remove_from_root(e: &mut Engine, root: u32, node: u32) {
    e.vcall(root, 0xe8, &args![node]);
}

/// `NiNode::AttachChild(child, 1)` (virtual slot `+0xdc`).
fn attach_child(e: &mut Engine, node: u32, child: u32) {
    e.vcall(node, 0xdc, &args![child, 1u32]);
}

/// The default text color from the setting: four bytes divided by 255.
fn default_color(e: &mut Engine) -> [f32; 4] {
    let packed: u32 = e.global(COLOR_SETTING_VALUE);
    let divisor: f64 = e.global(COLOR_DIVISOR);
    [
        (packed >> 24) & 0xff,
        (packed >> 16) & 0xff,
        (packed >> 8) & 0xff,
        packed & 0xff,
    ]
    .map(|byte| (byte as f64 / divisor) as f32)
}

/// Writes the color to `Color` of the entry: copied as four words from
/// `color` when given, else the default.
fn store_color(e: &mut Engine, data: Ptr<DebugTextData>, color: Ptr) {
    let target = data.at(DebugTextData::Color).addr();
    if color.is_null() {
        let value = default_color(e);
        for (i, lane) in value.iter().enumerate() {
            e.mem.set_f32(target + 4 * i as u32, *lane);
        }
    } else {
        for i in 0..4u32 {
            let word = e.mem.u32(color.addr() + 4 * i);
            e.mem.set_u32(target + 4 * i, word);
        }
    }
}

/// The color passed as pointer or else the default, as four floats.
fn color_or_default(e: &mut Engine, color: Ptr) -> [f32; 4] {
    if color.is_null() {
        default_color(e)
    } else {
        let mut value = [0.0f32; 4];
        for (i, lane) in value.iter_mut().enumerate() {
            *lane = e.mem.f32(color.addr() + 4 * i as u32);
        }
        value
    }
}

/// Font number `number` (1-based, as the settings count them) of the
/// `FontManager`.
fn font_object(e: &mut Engine, number: i32) -> u32 {
    let fonts: u32 = e.global(FONT_MANAGER);
    e.mem.u32(
        fonts
            .wrapping_add((number as u32).wrapping_mul(4))
            .wrapping_sub(4),
    )
}

/// `sin` and `cos` of a `float` angle as `FSINCOS` leaves them, stored to
/// `float`.
fn sin_cos(angle: f32) -> (f32, f32) {
    let angle = angle as f64;
    (angle.sin() as f32, angle.cos() as f32)
}

/// Rotation about the first axis, nine `float`s as the code lays them out:
/// `1 0 0 / 0 cos sin / 0 -sin cos`.
fn rotation_about_first_axis(angle: f32) -> [f32; 9] {
    let (sin, cos) = sin_cos(angle);
    [1.0, 0.0, 0.0, 0.0, cos, sin, 0.0, -sin, cos]
}

/// Rotation about the second axis: `cos 0 -sin / 0 1 0 / sin 0 cos`.
fn rotation_about_second_axis(angle: f32) -> [f32; 9] {
    let (sin, cos) = sin_cos(angle);
    [cos, 0.0, -sin, 0.0, 1.0, 0.0, sin, 0.0, cos]
}

/// Stores nine floats at `address`.
fn put_matrix(e: &mut Engine, address: u32, matrix: &[f32; 9]) {
    for (i, lane) in matrix.iter().enumerate() {
        e.mem.set_f32(address + 4 * i as u32, *lane);
    }
}

/// `target = (chain[0] * chain[1] * ...) * target` the way the code chains
/// `NiMatrix3::operator*` calls: each result lives in a temporary and the
/// last one is copied (nine words) over the target at `target`.
fn rotate_into(e: &mut Engine, target: u32, chain: &[[f32; 9]]) {
    let mut blocks = Vec::new();
    let mut accumulated = 0u32;
    let mut right = Vec::new();
    for matrix in chain {
        let block = e.mem.alloc(36);
        put_matrix(e, block, matrix);
        blocks.push(block);
        right.push(block);
    }
    right.push(target);
    for (i, next) in right.iter().enumerate() {
        if i == 0 {
            accumulated = *next;
            continue;
        }
        let output = e.mem.alloc(36);
        blocks.push(output);
        accumulated = e
            .call(MATRIX_MULTIPLY, &args![accumulated, output, *next])
            .u32();
    }
    for i in 0..9u32 {
        let word = e.mem.u32(accumulated + 4 * i);
        e.mem.set_u32(target + 4 * i, word);
    }
    for block in blocks {
        e.mem.free(block);
    }
}

/// Runs `body` with `TLS + 0x2b4` set to the allocation tag and restores it
/// afterwards (the pattern the allocating functions use).
fn with_allocation_tag<R>(e: &mut Engine, body: impl FnOnce(&mut Engine) -> R) -> R {
    let tls = e.tls();
    let saved = e.mem.u32(tls + ALLOCATION_TAG_OFFSET);
    e.mem.set_u32(tls + ALLOCATION_TAG_OFFSET, ALLOCATION_TAG);
    let result = body(e);
    e.mem.set_u32(tls + ALLOCATION_TAG_OFFSET, saved);
    result
}

/// Takes a node out of the root and clears the entry the way every release
/// in this file does: remove from the root, drop the node, zero the
/// position and justification, empty the text, `fDie = -1.0`.
fn release_entry(e: &mut Engine, entry: Ptr<DebugTextData>) {
    let root = debug_text_root(e);
    let node = e.get(entry, DebugTextData::spNode);
    remove_from_root(e, root, node.addr());
    set_node(e, entry.addr() + DebugTextData::spNode.off, 0);
    e.set(entry, DebugTextData::iJustify, 0);
    e.set(entry, DebugTextData::fX, 0.0);
    e.set(entry, DebugTextData::fY, 0.0);
    string_set(e, entry.at(DebugTextData::pText).addr(), 0);
    let never = e.global(MINUS_ONE);
    e.set(entry, DebugTextData::fDie, never);
}

/// Whether the object at `object` is of the class whose record is `rtti` or
/// a class derived from it: walks `GetRTTI` (virtual slot `+8`) and the base
/// record pointers (`+4`).
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, 8, &[]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record + 4);
    }
    false
}

/// The node `object` is, or else its parent (`+0x18`) when that is one;
/// null when neither is.
fn node_of(e: &mut Engine, object: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    if is_kind_of(e, object, NODE_RTTI) {
        return object;
    }
    let parent = e.mem.u32(object + 0x18);
    if parent != 0 && is_kind_of(e, parent, NODE_RTTI) {
        parent
    } else {
        0
    }
}

/// Whether `object` carries the `DebugTextExtraData` class.
fn is_debug_text_extra_data(e: &mut Engine, object: u32) -> bool {
    object != 0 && is_kind_of(e, object, EXTRA_DATA_RTTI)
}

/// Copies the NUL-free `FloatingText` setup both node builders repeat:
/// constructs the 0xB4-byte node and names it `"FloatingText"`.
fn new_floating_text_node(e: &mut Engine) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0xb4u32]).u32();
    if block != 0 {
        e.call(FLOATING_NODE_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 00a0d9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::Instance` (Xbox PDB): the singleton; with `create` set and no
/// instance yet, allocates (0x229C bytes, with `TLS + 0x2b4` set to `0xd`)
/// and constructs it with [`fn_00a0db40`].
// Translation incomplete: 00a0d9e0 - C++ exception unwinding (SEH frame) is not modeled.
pub fn debug_text_instance(e: &mut Engine, create: u8) -> Ptr<DebugText> {
    if e.global::<u32>(INSTANCE) == 0 && create != 0 {
        let created = with_allocation_tag(e, |e| {
            let block = allocate_block(e, 0x229c);
            if block == 0 {
                Ptr::NULL
            } else {
                fn_00a0db40(e, Ptr::new(block))
            }
        });
        e.set_global(INSTANCE, created.addr());
    }
    Ptr::new(e.global(INSTANCE))
}

// Translated from 00a0dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the singleton: runs the destructor, frees the block and clears
/// the pointer.
pub fn fn_00a0dae0(e: &mut Engine) {
    let instance = Ptr::<DebugText>::new(e.global(INSTANCE));
    if !instance.is_null() {
        debug_text_destructor(e, instance);
        free_block(e, instance.addr());
        e.set_global(INSTANCE, 0u32);
    }
}

// Translated from 00a0db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText` constructor: stores the vtables, constructs the 200 entries
/// (`00a11880`, destructor `00a119d0`), empties the floating text list and
/// the print queue, resets every entry again and zeroes the font height and
/// the line columns. Returns `this`.
// Translation incomplete: 00a0db40 - C++ exception unwinding (SEH frame) is not modeled.
pub fn fn_00a0db40(e: &mut Engine, this: Ptr<DebugText>) -> Ptr<DebugText> {
    e.mem.set_u32(this.addr(), INTERFACE_VTABLE);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            this.at(DebugText::pDebugTextList).addr(),
            ENTRY_SIZE,
            ENTRY_COUNT,
            0x00a1_1880u32,
            0x00a1_19d0u32
        ],
    );
    let list = this.at(DebugText::xFloatingTextList);
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
    fn_00a11cd0(e, this.at(DebugText::xPrintData));
    for index in 0..ENTRY_COUNT {
        let entry = entry_at(this, index);
        e.set(entry, DebugTextData::fX, 0.0);
        e.set(entry, DebugTextData::fY, 0.0);
        e.set(entry, DebugTextData::iJustify, 0);
        set_node(e, entry.addr() + DebugTextData::spNode.off, 0);
        string_set(e, entry.at(DebugTextData::pText).addr(), 0);
        let never = e.global(MINUS_ONE);
        e.set(entry, DebugTextData::fDie, never);
    }
    e.set(this, DebugText::iFontHeight, 0);
    e.call(
        MEMSET,
        &args![this.addr() + DebugText::iHeight.off, 0u32, 0xcu32],
    );
    e.call(
        MEMSET,
        &args![this.addr() + DebugText::iXLocation.off, 0u32, 0xcu32],
    );
    this
}

// Translated from 00a0dcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::~DebugText` (Xbox PDB): deletes every entry's node, then
/// walks the floating text list (under the lock): a text whose node has more
/// than two references is taken out of its parent and released; the list
/// element is removed and destroyed. Finally the print queue and the list
/// are emptied and the entries destroyed.
// Translation incomplete: 00a0dcc0 - C++ exception unwinding (SEH frame) is not modeled.
pub fn debug_text_destructor(e: &mut Engine, this: Ptr<DebugText>) {
    e.mem.set_u32(this.addr(), VTABLE);
    debug_text_delete_all_debug_text(e, this);
    let list = this.at(DebugText::xFloatingTextList);
    let mut cursor = e.get(list, NiTPointerList::m_pkHead);
    debug_text_lock(e);
    while cursor != 0 {
        let element_slot = cursor + 8;
        cursor = e.mem.u32(cursor);
        let element = e.mem.u32(element_slot);
        let node = e.mem.u32(element + DebugTextData::spNode.off);
        if e.mem.u32(node + 4) > 2 {
            let parent = e.mem.u32(node + 0x18);
            remove_from_root(e, parent, node);
            set_node(e, element + DebugTextData::spNode.off, 0);
        }
        let removed = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), element);
            fn_00a11c40(e, list.cast(), slot)
        });
        if removed != 0 {
            fn_00a119d0(e, Ptr::new(removed));
            free_block(e, removed);
        }
    }
    debug_text_unlock(e);
    bs_simple_array_print_data_destructor(e, this.at(DebugText::xPrintData));
    nitpointerlist_remove_all(e, list.cast());
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![
            this.at(DebugText::pDebugTextList).addr(),
            ENTRY_SIZE,
            ENTRY_COUNT,
            0x00a1_19d0u32
        ],
    );
}

// Translated from 00a0de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::DeleteAllDebugText` (Xbox PDB): under the lock, takes every
/// entry that has a node out of the debug text root and clears the entry.
pub fn debug_text_delete_all_debug_text(e: &mut Engine, this: Ptr<DebugText>) {
    debug_text_lock(e);
    for index in 0..ENTRY_COUNT {
        let entry = entry_at(this, index);
        if !e.get(entry, DebugTextData::spNode).is_null() {
            release_entry(e, entry);
        }
    }
    debug_text_unlock(e);
}

// Translated from 00a0df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame update (called from `Idle`): outside menu mode, counts down
/// `fDie` of every table entry that has one (when it reaches zero or goes
/// below, the entry's node is released, under the lock) and of every floating
/// text (a floating text whose node is no longer shared, fewer than two
/// references, is dropped; one whose time went below zero is taken out of its
/// parent and dropped).
pub fn fn_00a0df80(e: &mut Engine) {
    if e.call(IS_IN_MENU_MODE, &[]).bool() {
        return;
    }
    let this = debug_text_instance(e, 1);
    let zero: f64 = e.global(ZERO_DOUBLE);
    for index in 0..ENTRY_COUNT {
        let entry = entry_at(this, index);
        if e.get(entry, DebugTextData::fDie) as f64 > zero {
            let frame: f32 = e.global(FRAME_TIME);
            let left = (e.get(entry, DebugTextData::fDie) as f64 - frame as f64) as f32;
            e.set(entry, DebugTextData::fDie, left);
            // `TEST AH,0x41` / `JP` at 00a0e01f: released at exactly zero
            // too (NaN skips). The floating texts below use `TEST AH,0x5`
            // (00a0e1dd), a strict `<`.
            if (left as f64) <= zero && !e.get(entry, DebugTextData::spNode).is_null() {
                debug_text_lock(e);
                release_entry(e, entry);
                debug_text_unlock(e);
            }
        }
    }
    let list = this.at(DebugText::xFloatingTextList);
    let mut cursor = e.get(list, NiTPointerList::m_pkHead);
    while cursor != 0 {
        let element_slot = cursor + 8;
        cursor = e.mem.u32(cursor);
        let element = Ptr::<DebugTextData>::new(e.mem.u32(element_slot));
        let node = e.get(element, DebugTextData::spNode);
        if e.mem.u32(node.addr() + 4) < 2 {
            remove_floating_text(e, list, element);
        } else if e.get(element, DebugTextData::fDie) as f64 > zero {
            let frame: f32 = e.global(FRAME_TIME);
            let left = (e.get(element, DebugTextData::fDie) as f64 - frame as f64) as f32;
            e.set(element, DebugTextData::fDie, left);
            if (left as f64) < zero {
                debug_text_lock(e);
                let parent = e.mem.u32(node.addr() + 0x18);
                remove_from_root(e, parent, node.addr());
                set_node(e, element.addr() + DebugTextData::spNode.off, 0);
                debug_text_unlock(e);
                remove_floating_text(e, list, element);
            }
        }
    }
}

/// Removes `element` from the floating text list and destroys it when the
/// removal hands it back.
fn remove_floating_text(e: &mut Engine, list: Ptr<NiTPointerList>, element: Ptr<DebugTextData>) {
    let removed = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), element.addr());
        fn_00a11c40(e, list.cast(), slot)
    });
    if removed != 0 {
        fn_00a119d0(e, Ptr::new(removed));
        free_block(e, removed);
    }
}

// Translated from 00a0e2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::Reset` (Xbox PDB): takes the font height from the debug text
/// font (the `float` that `Font + 0x38` points to, plus a constant, as an
/// integer), puts the three line heights at that many rows (four times the
/// height when the full help is on) below the margin setting and sets the
/// three horizontal positions: the left margin, the mirrored right one and
/// the middle.
pub fn debug_text_reset(e: &mut Engine, this: Ptr<DebugText>) {
    let font_number: i32 = e.global(FONT_SETTING_VALUE);
    let font = font_object(e, font_number);
    let metrics = e.mem.u32(font + 0x38);
    let line_height = if metrics != 0 {
        e.mem.f32(metrics)
    } else {
        0.0
    };
    let bias: f64 = e.global(LINE_HEIGHT_BIAS);
    let font_height = e.call(FTOL, &args![line_height as f64 + bias]).i32();
    e.set(this, DebugText::iFontHeight, font_height);
    for column in 0..3 {
        let rows: i32 = if e.call(GET_FULL_HELP, &[]).bool() {
            4
        } else {
            1
        };
        let margin: i32 = e.global(LINE_HEIGHT_SETTING_VALUE);
        e.mem.set_i32(
            height_address(this, column),
            rows.wrapping_mul(font_height).wrapping_add(margin),
        );
    }
    let left: i32 = e.global(LEFT_MARGIN_SETTING_VALUE);
    let right = 0x500i32.wrapping_sub(left);
    e.mem.set_i32(x_location_address(this, 1), right);
    e.mem.set_i32(x_location_address(this, 0), left);
    let middle = (right.wrapping_sub(left) >> 1).wrapping_add(left);
    e.mem.set_i32(x_location_address(this, 2), middle);
}

// Translated from 00a0e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::SkipLine` (Xbox PDB): moves line column `column` down by
/// `count` font heights.
pub fn debug_text_skip_line(e: &mut Engine, this: Ptr<DebugText>, column: i32, count: i32) {
    let font_height = e.get(this, DebugText::iFontHeight);
    let address = height_address(this, column);
    let moved = font_height
        .wrapping_mul(count)
        .wrapping_add(e.mem.i32(address));
    e.mem.set_i32(address, moved);
}

/// Copies the variable-argument words into game memory as the `va_list` the
/// virtual `VAPrint*` functions take, runs `body` with its address and
/// frees it.
fn with_va_list(e: &mut Engine, words: &[u32], body: impl FnOnce(&mut Engine, u32) -> u32) -> u32 {
    let list = e.mem.alloc(4 * words.len().max(1) as u32);
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(list + 4 * i as u32, *word);
    }
    let result = body(e, list);
    e.mem.free(list);
    result
}

// Translated from 00a0e470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::PrintX` (Xbox PDB), `cdecl` with variable arguments: prints
/// the formatted text at horizontal position `x` on line column 0, justify
/// value 1, through the virtual `VAPrintX` (slot `+0x10`).
pub fn debug_text_print_x(
    e: &mut Engine,
    this: Ptr<DebugText>,
    x: i32,
    format: u32,
    arguments: &[u32],
) -> u32 {
    with_va_list(e, arguments, |e, list| {
        e.vcall(this.addr(), 0x10, &args![x, 0i32, 1i32, format, list, 0u32])
            .u32()
    })
}

/// Registered form of [`debug_text_print_x`]: `this`, `x`, the format, then
/// the variable words.
fn print_x_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let result = debug_text_print_x(
        e,
        Ptr::new(words[0]),
        words[1] as i32,
        words[2],
        &words[3..],
    );
    result.into_ret()
}

// Translated from 00a0e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::PrintX_ov2` (Xbox PDB), `cdecl` with variable arguments:
/// prints at `x` on line column `column` with `justify`.
pub fn debug_text_print_x_ov2(
    e: &mut Engine,
    this: Ptr<DebugText>,
    x: i32,
    column: i32,
    justify: i32,
    format: u32,
    arguments: &[u32],
) -> u32 {
    with_va_list(e, arguments, |e, list| {
        e.vcall(
            this.addr(),
            0x10,
            &args![x, column, justify, format, list, 0u32],
        )
        .u32()
    })
}

/// Registered form of [`debug_text_print_x_ov2`].
fn print_x_ov2_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let result = debug_text_print_x_ov2(
        e,
        Ptr::new(words[0]),
        words[1] as i32,
        words[2] as i32,
        words[3] as i32,
        words[4],
        &words[5..],
    );
    result.into_ret()
}

/// Draws formatted text: `_vsnprintf` into a 200-byte buffer, then
/// [`debug_text_print`] with no line number, `fDie = -1.0` and font 0.
#[allow(clippy::too_many_arguments)] // `this`, the position, the format and the color.
fn print_formatted(
    e: &mut Engine,
    this: Ptr<DebugText>,
    x: f32,
    y: f32,
    justify: i32,
    format: u32,
    list: u32,
    color: u32,
) -> u32 {
    let buffer = e.mem.alloc(200);
    e.call(VSNPRINTF, &args![buffer, 200u32, format, list]);
    let never = e.global(MINUS_ONE);
    let result = debug_text_print(
        e,
        this,
        buffer,
        x,
        y,
        justify,
        -1,
        never,
        0,
        Ptr::new(color),
    );
    e.mem.free(buffer);
    result
}

// Translated from 00a0e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::VAPrintX` (Xbox PDB): formats into a 200-byte buffer and
/// prints it at `x` on the height of line column `column`.
// Translation incomplete: 00a0e4f0 - the /GS stack security cookie check is not modeled.
#[allow(clippy::too_many_arguments)] // The exe ABI has six stack arguments and `this`.
pub fn debug_text_va_print_x(
    e: &mut Engine,
    this: Ptr<DebugText>,
    x: i32,
    column: i32,
    justify: i32,
    format: u32,
    list: u32,
    color: u32,
) -> u32 {
    let y = e.mem.i32(height_address(this, column));
    print_formatted(e, this, x as f32, y as f32, justify, format, list, color)
}

// Translated from 00a0e580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::PrintLine` (Xbox PDB), `cdecl` with variable arguments:
/// prints a line in column `column` through the virtual `VAPrintLine` (slot
/// `+0x18`).
pub fn debug_text_print_line(
    e: &mut Engine,
    this: Ptr<DebugText>,
    column: i32,
    justify: i32,
    format: u32,
    arguments: &[u32],
) -> u32 {
    with_va_list(e, arguments, |e, list| {
        e.vcall(
            this.addr(),
            0x18,
            &args![column, justify, format, list, 0u32],
        )
        .u32()
    })
}

/// Registered form of [`debug_text_print_line`].
fn print_line_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let result = debug_text_print_line(
        e,
        Ptr::new(words[0]),
        words[1] as i32,
        words[2] as i32,
        words[3],
        &words[4..],
    );
    result.into_ret()
}

// Translated from 00a0e5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::VAPrintLine` (Xbox PDB): formats and prints at the horizontal
/// position and height of line column `column`, then moves that column down
/// by one font height.
// Translation incomplete: 00a0e5c0 - the /GS stack security cookie check is not modeled.
pub fn debug_text_va_print_line(
    e: &mut Engine,
    this: Ptr<DebugText>,
    column: i32,
    justify: i32,
    format: u32,
    list: u32,
    color: u32,
) -> u32 {
    let x = e.mem.i32(x_location_address(this, column));
    let y = e.mem.i32(height_address(this, column));
    let result = print_formatted(e, this, x as f32, y as f32, justify, format, list, color);
    let next = e
        .mem
        .i32(height_address(this, column))
        .wrapping_add(e.get(this, DebugText::iFontHeight));
    e.mem.set_i32(height_address(this, column), next);
    result
}

// Translated from 00a0e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::PrintXC` (Xbox PDB), `cdecl` with variable arguments:
/// [`debug_text_print_x`] with a color.
pub fn debug_text_print_x_c(
    e: &mut Engine,
    this: Ptr<DebugText>,
    x: i32,
    color: u32,
    format: u32,
    arguments: &[u32],
) -> u32 {
    with_va_list(e, arguments, |e, list| {
        e.vcall(
            this.addr(),
            0x10,
            &args![x, 0i32, 1i32, format, list, color],
        )
        .u32()
    })
}

/// Registered form of [`debug_text_print_x_c`].
fn print_x_c_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let result = debug_text_print_x_c(
        e,
        Ptr::new(words[0]),
        words[1] as i32,
        words[2],
        words[3],
        &words[4..],
    );
    result.into_ret()
}

// Translated from 00a0e6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::PrintXC_ov2` (Xbox PDB), `cdecl` with variable arguments:
/// [`debug_text_print_x_ov2`] with a color.
#[allow(clippy::too_many_arguments)] // The exe ABI has six fixed stack arguments and `this`.
pub fn debug_text_print_x_c_ov2(
    e: &mut Engine,
    this: Ptr<DebugText>,
    x: i32,
    color: u32,
    column: i32,
    justify: i32,
    format: u32,
    arguments: &[u32],
) -> u32 {
    with_va_list(e, arguments, |e, list| {
        e.vcall(
            this.addr(),
            0x10,
            &args![x, column, justify, format, list, color],
        )
        .u32()
    })
}

/// Registered form of [`debug_text_print_x_c_ov2`].
fn print_x_c_ov2_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let result = debug_text_print_x_c_ov2(
        e,
        Ptr::new(words[0]),
        words[1] as i32,
        words[2],
        words[3] as i32,
        words[4] as i32,
        words[5],
        &words[6..],
    );
    result.into_ret()
}

// Translated from 00a0e710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::PrintLineC` (Xbox PDB), `cdecl` with variable arguments:
/// [`debug_text_print_line`] with a color.
pub fn debug_text_print_line_c(
    e: &mut Engine,
    this: Ptr<DebugText>,
    color: u32,
    column: i32,
    justify: i32,
    format: u32,
    arguments: &[u32],
) -> u32 {
    with_va_list(e, arguments, |e, list| {
        e.vcall(
            this.addr(),
            0x18,
            &args![column, justify, format, list, color],
        )
        .u32()
    })
}

/// Registered form of [`debug_text_print_line_c`].
fn print_line_c_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let result = debug_text_print_line_c(
        e,
        Ptr::new(words[0]),
        words[1],
        words[2] as i32,
        words[3] as i32,
        words[4],
        &words[5..],
    );
    result.into_ret()
}

// Translated from 00a0e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::GetCurrentLine` (Xbox PDB): the height of line column
/// `column`.
pub fn debug_text_get_current_line(e: &mut Engine, this: Ptr<DebugText>, column: i32) -> i32 {
    e.mem.i32(height_address(this, column))
}

// Translated from 00a11860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::SetCurrentLine` (Xbox PDB): sets the height of line column
/// `column`.
pub fn debug_text_set_current_line(e: &mut Engine, this: Ptr<DebugText>, column: i32, line: i32) {
    e.mem.set_i32(height_address(this, column), line);
}

// Translated from 00a0e770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::HandleQueuedPrints` (Xbox PDB): drains the print queue. A
/// type 0 item (set the text of entry `iIndex`) releases the entry's node
/// if any and builds the text again. A type 1 item (print at a position)
/// wraps the text at the line breaks with the debug font, keeps the lines
/// from `iLineNumber` on, finds the entry with the same position (or the
/// first free one, reporting `"Too many unique debug text items"` when
/// there is none) and, unless it already shows the same text and color,
/// replaces its node by a new one built with
/// [`debug_text_create_debug_text`]. Afterwards every queued item is
/// destroyed and the queue cleared.
///
/// As in the exe, a type 1 item that has no debug text root, that has fewer
/// lines than `iLineNumber` or that finds no free entry ends the whole
/// drain at once, leaving the rest of the queue (and the items already
/// handled) in place. The scan that follows the line copy in the exe (it
/// measures the rest of the text into a local nothing reads) is left out.
// Translation incomplete: 00a0e770 - C++ exception unwinding (SEH frame) is not modeled.
pub fn debug_text_handle_queued_prints(e: &mut Engine, this: Ptr<DebugText>) {
    let queue = this.at(DebugText::xPrintData);
    let mut index = 0u32;
    while index < e.get(queue, BSSimpleArray::iSize) {
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer + 4 * index));
        match e.get(print_data, SPrintData::eType) {
            0 => handle_text_update(e, this, print_data),
            1 if !handle_print(e, this, print_data) => return,
            _ => {}
        }
        index += 1;
    }
    let mut index = 0u32;
    while index < e.get(queue, BSSimpleArray::iSize) {
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer + 4 * index));
        let data = e.get(print_data, SPrintData::pTextData);
        if !data.is_null() {
            fn_00a119d0(e, data);
            free_block(e, data.addr());
        }
        free_block(e, print_data.addr());
        index += 1;
    }
    e.call(ARRAY_CLEAR, &args![queue.addr(), 1u32]);
}

/// A type 0 item of the queue: rebuilds the entry `iIndex` from the item's
/// text.
fn handle_text_update(e: &mut Engine, this: Ptr<DebugText>, print_data: Ptr<SPrintData>) {
    let index = e.get(print_data, SPrintData::iIndex);
    let entry = entry_at(this, index as u32);
    let data = e.get(print_data, SPrintData::pTextData);
    if !e.get(entry, DebugTextData::spNode).is_null() {
        debug_text_lock(e);
        release_entry(e, entry);
        debug_text_unlock(e);
    }
    let mut same_text = false;
    if !e.get(entry, DebugTextData::spNode).is_null() {
        let shown = e.get(entry.at(DebugTextData::pText), BSStringT::pString);
        let wanted = e.get(data.at(DebugTextData::pText), BSStringT::pString);
        same_text = e.call(STRCMP, &args![shown, wanted]).i32() == 0;
    }
    if !same_text {
        debug_text_lock(e);
        let text = e.get(data.at(DebugTextData::pText), BSStringT::pString);
        let x = e.get(entry, DebugTextData::fX);
        let y = e.get(entry, DebugTextData::fY);
        let justify = e.get(entry, DebugTextData::iJustify);
        let node = debug_text_create_debug_text(e, this, text, x, y, justify, 0, Ptr::NULL);
        set_node(e, entry.addr() + DebugTextData::spNode.off, node.addr());
        debug_text_unlock(e);
    }
}

/// A type 1 item of the queue. Returns false when the whole drain must stop
/// (the exe returns from `HandleQueuedPrints` there).
fn handle_print(e: &mut Engine, this: Ptr<DebugText>, print_data: Ptr<SPrintData>) -> bool {
    with_allocation_tag(e, |e| {
        if debug_text_root(e) == 0 {
            return false;
        }
        let mut font = e.get(print_data, SPrintData::iFont);
        if font == 0 {
            font = e.global(FONT_SETTING_VALUE);
        }
        e.with_stack(8, |e, text| {
            let keep_going = handle_print_item(e, this, print_data, text.cast(), font);
            string_set(e, text.addr(), 0);
            keep_going
        })
    })
}

/// The body of [`handle_print`] with the local `BSStringT` at `text`.
fn handle_print_item(
    e: &mut Engine,
    this: Ptr<DebugText>,
    print_data: Ptr<SPrintData>,
    text: Ptr<BSStringT>,
    font: i32,
) -> bool {
    let data = e.get(print_data, SPrintData::pTextData);
    let source = data.at(DebugTextData::pText);
    let mut is_empty = false;
    let mut have_same = false;
    if e.get(source, BSStringT::pString) == 0 || string_length(e, source) == 0 {
        is_empty = true;
        e.set(print_data, SPrintData::iLineNumber, 0);
    } else {
        e.call(STRING_COPY, &args![text, source]);
        let font_object = font_object(e, font);
        e.with_stack(8, |e, sizes| {
            let mut width = 0x4b0i32;
            if e.mem.u8(WIDE_FLAG) != 0 {
                let scale: f64 = e.global(WRAP_WIDTH_SCALE);
                width = e.call(FTOL, &args![width as f64 * scale]).i32();
            }
            e.mem.set_i32(sizes.addr(), width);
            e.mem.set_i32(sizes.addr() + 4, 0);
            e.call(
                LINE_BREAK_TEXT,
                &args![font_object, text, sizes, sizes.addr() + 4, 0u32, 10u32],
            );
        });
    }
    let first_line = e.get(print_data, SPrintData::iLineNumber);
    if first_line > 0 {
        // Skip to the line `iLineNumber` and move it to the front of the text.
        let length = string_length(e, text);
        let mut lines = 0i32;
        let mut position = 0u32;
        while position < length && lines < first_line - 1 {
            if char_at(e, text, position) == b'\n' {
                lines += 1;
            }
            position += 1;
        }
        if position == length {
            return false;
        }
        let mut copied = 0u32;
        loop {
            let c = char_at(e, text, position);
            if c == b'\n' || c == 0 {
                break;
            }
            set_char_at(e, text, copied, c);
            copied += 1;
            position += 1;
        }
        set_char_at(e, text, copied, 0);
    }

    // The entry with the same position, else the first free one.
    let wanted_x = e.get(data, DebugTextData::fX);
    let wanted_y = e.get(data, DebugTextData::fY);
    let wanted_justify = e.get(data, DebugTextData::iJustify);
    let mut slot = 0u32;
    while slot < ENTRY_COUNT {
        let entry = entry_at(this, slot);
        if e.get(entry, DebugTextData::fX) == wanted_x
            && e.get(entry, DebugTextData::fY) == wanted_y
            && e.get(entry, DebugTextData::iJustify) == wanted_justify
        {
            break;
        }
        slot += 1;
    }
    if slot == ENTRY_COUNT {
        slot = 0;
        while slot < ENTRY_COUNT && e.get(entry_at(this, slot), DebugTextData::iJustify) != 0 {
            slot += 1;
        }
        if slot == ENTRY_COUNT {
            e.call(REPORT_ERROR, &args![TOO_MANY_ITEMS_TEXT]);
            return false;
        }
    } else {
        let entry = entry_at(this, slot);
        if !is_empty {
            if !e.get(entry, DebugTextData::spNode).is_null() {
                let shown = entry.at(DebugTextData::pText);
                let text_pointer = e.get(text, BSStringT::pString);
                let equal = e
                    .call(STRING_COMPARE, &args![shown, text_pointer, 1u32])
                    .i32()
                    == 0;
                if equal {
                    let mut same_color = true;
                    for lane in 0..4u32 {
                        let new = e.mem.f32(data.at(DebugTextData::Color).addr() + 4 * lane);
                        let old = e.mem.f32(entry.at(DebugTextData::Color).addr() + 4 * lane);
                        if old != new {
                            same_color = false;
                            break;
                        }
                    }
                    if same_color {
                        have_same = true;
                    }
                }
            }
        } else {
            have_same = false;
        }
        if !have_same && !e.get(entry, DebugTextData::spNode).is_null() {
            debug_text_lock(e);
            let root = debug_text_root(e);
            let node = e.get(entry, DebugTextData::spNode);
            remove_from_root(e, root, node.addr());
            set_node(e, entry.addr() + DebugTextData::spNode.off, 0);
            debug_text_unlock(e);
        }
    }

    if !have_same {
        let entry = entry_at(this, slot);
        debug_text_lock(e);
        if !is_empty {
            let text_pointer = e.get(text, BSStringT::pString);
            let x = e.get(data, DebugTextData::fX);
            let y = e.get(data, DebugTextData::fY);
            let justify = e.get(data, DebugTextData::iJustify);
            let color = data.at(DebugTextData::Color).cast();
            let node =
                debug_text_create_debug_text(e, this, text_pointer, x, y, justify, font, color);
            set_node(e, entry.addr() + DebugTextData::spNode.off, node.addr());
        } else {
            set_node(e, entry.addr() + DebugTextData::spNode.off, 0);
        }
        let node = e.get(entry, DebugTextData::spNode);
        if node.is_null() {
            e.set(entry, DebugTextData::iJustify, 0);
            e.set(entry, DebugTextData::fX, 0.0);
            e.set(entry, DebugTextData::fY, 0.0);
            let never = e.global(MINUS_ONE);
            e.set(entry, DebugTextData::fDie, never);
        } else {
            e.call(STRING_COPY, &args![entry.at(DebugTextData::pText), text]);
            let text_pointer = e.get(text, BSStringT::pString);
            let handle = if text_pointer == 0 {
                e.global(EMPTY_FIXED_STRING)
            } else {
                e.call(FIXED_STRING_CREATE, &args![text_pointer]).u32()
            };
            e.with_stack(4, |e, slot_for_handle| {
                e.mem.set_u32(slot_for_handle.addr(), handle);
                e.call(SET_NAME, &args![node, slot_for_handle]);
            });
            if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
                e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
            }
            let x = e.get(data, DebugTextData::fX);
            let y = e.get(data, DebugTextData::fY);
            let justify = e.get(data, DebugTextData::iJustify);
            e.set(entry, DebugTextData::fX, x);
            e.set(entry, DebugTextData::fY, y);
            e.set(entry, DebugTextData::iJustify, justify);
            let die = e.get(print_data, SPrintData::fDie);
            e.set(entry, DebugTextData::fDie, die);
            for lane in 0..4u32 {
                let word = e.mem.u32(data.at(DebugTextData::Color).addr() + 4 * lane);
                e.mem
                    .set_u32(entry.at(DebugTextData::Color).addr() + 4 * lane, word);
            }
        }
        debug_text_unlock(e);
    }
    true
}

/// Byte `index` of the text of a `BSStringT` (the exe indexes through a
/// null-checked pointer, so a null string reads at 0).
fn char_at(e: &mut Engine, string: Ptr<BSStringT>, index: u32) -> u8 {
    let base = e.get(string, BSStringT::pString);
    let at = if base != 0 { index } else { 0 };
    e.mem.u8(base.wrapping_add(at))
}

fn set_char_at(e: &mut Engine, string: Ptr<BSStringT>, index: u32, value: u8) {
    let base = e.get(string, BSStringT::pString);
    let at = if base != 0 { index } else { 0 };
    e.mem.set_u8(base.wrapping_add(at), value);
}

// Translated from 00a0f730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues (type 0) the text for entry `index`: updates the text of an
/// already queued item with that index, else creates a data object and an
/// item. Returns `index`.
// Translation incomplete: 00a0f730 - C++ exception unwinding (SEH frame) is not modeled.
pub fn fn_00a0f730(e: &mut Engine, this: Ptr<DebugText>, text: u32, index: i32) -> i32 {
    let queue = this.at(DebugText::xPrintData);
    let mut found = false;
    let mut position = 0u32;
    while !found && position < e.get(queue, BSSimpleArray::iSize) {
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer + 4 * position));
        if e.get(print_data, SPrintData::iIndex) == index {
            let data = e.get(print_data, SPrintData::pTextData);
            string_set(e, data.at(DebugTextData::pText).addr(), text);
            e.set(print_data, SPrintData::eType, 0);
            found = true;
        }
        position += 1;
    }
    if !found {
        let block = allocate_block(e, 0x2c);
        let data = if block == 0 {
            Ptr::NULL
        } else {
            fn_00a11880(e, Ptr::new(block))
        };
        string_set(e, data.at(DebugTextData::pText).addr(), text);
        let print_block = allocate_block(e, 0x18);
        let print_data = Ptr::<SPrintData>::new(print_block);
        e.set(print_data, SPrintData::iIndex, index);
        e.set(print_data, SPrintData::pTextData, data);
        e.set(print_data, SPrintData::eType, 0);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), print_block);
            fn_00a11d20(e, queue, slot);
        });
    }
    index
}

// Translated from 00a0f8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::Print` (Xbox PDB): queues a print of `text` at (`x`, `y`)
/// with `justify`. When an item with the same position is queued already,
/// its text, line number, `fDie`, font and color are replaced; otherwise a
/// new data object and item are added to the queue. A null `color` means
/// the default color setting. Returns 0.
// Translation incomplete: 00a0f8b0 - C++ exception unwinding (SEH frame) is not modeled.
#[allow(clippy::too_many_arguments)] // The exe ABI has eight stack arguments.
pub fn debug_text_print(
    e: &mut Engine,
    this: Ptr<DebugText>,
    text: u32,
    x: f32,
    y: f32,
    justify: i32,
    line_number: i32,
    die: f32,
    font: i32,
    color: Ptr,
) -> u32 {
    let queue = this.at(DebugText::xPrintData);
    let mut found = false;
    let mut position = 0u32;
    while !found && position < e.get(queue, BSSimpleArray::iSize) {
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer + 4 * position));
        let data = e.get(print_data, SPrintData::pTextData);
        if x == e.get(data, DebugTextData::fX)
            && y == e.get(data, DebugTextData::fY)
            && e.get(data, DebugTextData::iJustify) == justify
        {
            string_set(e, data.at(DebugTextData::pText).addr(), text);
            e.set(print_data, SPrintData::eType, 1);
            e.set(print_data, SPrintData::iLineNumber, line_number);
            e.set(print_data, SPrintData::fDie, die);
            e.set(print_data, SPrintData::iFont, font);
            store_color(e, data, color);
            found = true;
        }
        position += 1;
    }
    if !found {
        let block = allocate_block(e, 0x2c);
        let data = if block == 0 {
            Ptr::NULL
        } else {
            fn_00a11880(e, Ptr::new(block))
        };
        string_set(e, data.at(DebugTextData::pText).addr(), text);
        e.set(data, DebugTextData::fX, x);
        e.set(data, DebugTextData::fY, y);
        e.set(data, DebugTextData::iJustify, justify);
        store_color(e, data, color);
        let print_block = allocate_block(e, 0x18);
        let print_data = Ptr::<SPrintData>::new(print_block);
        e.set(print_data, SPrintData::pTextData, data);
        e.set(print_data, SPrintData::eType, 1);
        e.set(print_data, SPrintData::iLineNumber, line_number);
        e.set(print_data, SPrintData::fDie, die);
        e.set(print_data, SPrintData::iFont, font);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), print_block);
            fn_00a11d20(e, queue, slot);
        });
    }
    0
}

// Translated from 00a0fe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::CreateDebugText` (Xbox PDB): builds the node for one text.
/// Returns null for an empty text or when neither the console nor the debug
/// text is visible. Otherwise: converts the position to node coordinates
/// (`x` from the 640 wide layout, `y` up from the screen's middle), maps
/// `justify` 3 and 2 to the alignment codes 4 and 2 (else 1), builds an
/// `NiNode`, the font's string geometry (`Font::MakeString`) with the color
/// (or the default), attaches the geometry, sets the color on its property
/// type 3, clears bit 0 of the node's flags, attaches the node to the debug
/// text root (under the lock), positions it, updates it and returns it.
// Translation incomplete: 00a0fe20 - C++ exception unwinding (SEH frame) is not modeled.
#[allow(clippy::too_many_arguments)] // The exe ABI has six stack arguments and `this`.
pub fn debug_text_create_debug_text(
    e: &mut Engine,
    _this: Ptr<DebugText>,
    text: u32,
    x: f32,
    y: f32,
    justify: i32,
    font: i32,
    color: Ptr,
) -> Ptr {
    with_allocation_tag(e, |e| {
        if e.mem.i8(text) == 0 {
            return Ptr::NULL;
        }
        if !e.call(IS_CONSOLE_VISIBLE, &[]).bool() && !e.call(IS_DEBUG_TEXT_VISIBLE, &[]).bool() {
            return Ptr::NULL;
        }
        let mut scale = 1.0f32;
        if e.mem.u8(WIDE_FLAG) != 0 {
            scale = e.global(WIDE_SCALE);
        }
        let half_width: f64 = e.global(SCREEN_HALF_WIDTH);
        let x = ((x as f64 - half_width) * scale as f64) as f32;
        let screen = e.call(GET_SCREEN_HEIGHT, &[]).f32();
        let up = screen as f64 - y as f64;
        let half = e.call(GET_SCREEN_HEIGHT, &[]).f32();
        let divisor: f64 = e.global(TWO_DOUBLE);
        let y = (up - half as f64 / divisor) as f32;
        let alignment = match justify {
            3 => 4u32,
            2 => 2u32,
            _ => 1u32,
        };
        let block = e.call(NEW_OBJECT, &args![0xacu32]).u32();
        let node = if block != 0 {
            e.call(NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
        } else {
            0
        };
        let color = color_or_default(e, color);
        let font = if font == 0 {
            e.global(FONT_SETTING_VALUE)
        } else {
            font
        };
        let font_object = font_object(e, font);
        e.with_stack(8 + 4 + 16, |e, locals| {
            let string = locals.addr();
            let size_slot = locals.addr() + 8;
            let color_block = locals.addr() + 12;
            e.call(STRING_FROM_TEXT, &args![string, text]);
            for (i, lane) in color.iter().enumerate() {
                e.mem.set_f32(color_block + 4 * i as u32, *lane);
            }
            let geometry = e
                .call(
                    MAKE_STRING,
                    &args![
                        font_object,
                        0.0f32,
                        0.0f32,
                        0.0f32,
                        string,
                        size_slot,
                        alignment,
                        color_block,
                        1u32,
                        1u32
                    ],
                )
                .u32();
            for lane in 0..3u32 {
                e.mem.set_f32(geometry + 0x58 + 4 * lane, 0.0);
            }
            attach_child(e, node, geometry);
            let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
            if property != 0 {
                for lane in 0..4u32 {
                    let word = e.mem.u32(color_block + 4 * lane);
                    e.mem.set_u32(property + 0x68 + 4 * lane, word);
                }
            }
            let flags = e.mem.u32(node + 0x30);
            e.mem.set_u32(node + 0x30, flags & !1);
            debug_text_lock(e);
            if debug_text_root(e) != 0 {
                let root = debug_text_root(e);
                attach_child(e, root, node);
            }
            let depth: f32 = e.global(TEXT_DEPTH);
            e.mem.set_f32(node + 0x58, x);
            e.mem.set_f32(node + 0x5c, depth);
            e.mem.set_f32(node + 0x60, y);
            e.call(UPDATE_PROPERTIES, &args![node]);
            e.with_stack(12, |e, update_data| {
                e.call(UPDATE, &args![node, update_data]);
            });
            debug_text_unlock(e);
            string_set(e, string, 0);
            Ptr::new(node)
        })
    })
}

// Translated from 00a103e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks for the text shown at (`x`, `y`) with `justify`: first among the
/// queued prints, then among the table entries. Copies up to 0x7fe
/// characters of it into `out` (when not null and the text is not empty) and
/// returns whether an item with that position exists. `out[0]` is cleared
/// first.
pub fn fn_00a103e0(
    e: &mut Engine,
    this: Ptr<DebugText>,
    out: u32,
    x: f32,
    y: f32,
    justify: i32,
) -> bool {
    let mut found = false;
    e.mem.set_u8(out, 0);
    let queue = this.at(DebugText::xPrintData);
    let mut position = 0u32;
    while !found && position < e.get(queue, BSSimpleArray::iSize) {
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer + 4 * position));
        let data = e.get(print_data, SPrintData::pTextData);
        if x == e.get(data, DebugTextData::fX)
            && y == e.get(data, DebugTextData::fY)
            && e.get(data, DebugTextData::iJustify) == justify
        {
            if out != 0 {
                let text = data.at(DebugTextData::pText);
                if string_length(e, text) != 0 {
                    let source = e.get(text, BSStringT::pString);
                    e.call(STRNCPY, &args![out, source, 0x7feu32]);
                }
            }
            found = true;
        }
        position += 1;
    }
    let mut slot = 0u32;
    while !found && slot < ENTRY_COUNT {
        let entry = entry_at(this, slot);
        if x == e.get(entry, DebugTextData::fX)
            && y == e.get(entry, DebugTextData::fY)
            && e.get(entry, DebugTextData::iJustify) == justify
        {
            if out != 0 {
                let text = entry.at(DebugTextData::pText);
                if string_length(e, text) != 0 {
                    let source = e.get(text, BSStringT::pString);
                    e.call(STRNCPY, &args![out, source, 0x7feu32]);
                }
            }
            found = true;
        }
        slot += 1;
    }
    found
}

// Translated from 00a10630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text pointer of the item shown at (`x`, `y`) with `justify`: first
/// among the queued prints, then among the table entries; 0 when there is
/// none.
pub fn fn_00a10630(e: &mut Engine, this: Ptr<DebugText>, x: f32, y: f32, justify: i32) -> u32 {
    let queue = this.at(DebugText::xPrintData);
    let mut position = 0u32;
    while position < e.get(queue, BSSimpleArray::iSize) {
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer + 4 * position));
        let data = e.get(print_data, SPrintData::pTextData);
        if x == e.get(data, DebugTextData::fX)
            && y == e.get(data, DebugTextData::fY)
            && e.get(data, DebugTextData::iJustify) == justify
        {
            return e.get(data.at(DebugTextData::pText), BSStringT::pString);
        }
        position += 1;
    }
    let mut slot = 0u32;
    while slot < ENTRY_COUNT {
        let entry = entry_at(this, slot);
        if x == e.get(entry, DebugTextData::fX)
            && y == e.get(entry, DebugTextData::fY)
            && e.get(entry, DebugTextData::iJustify) == justify
        {
            return e.get(entry.at(DebugTextData::pText), BSStringT::pString);
        }
        slot += 1;
    }
    0
}

// Translated from 00a10770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a floating text node (`cdecl`, eight stack words): an 0xB4-byte
/// node named `"FloatingText"`, the string geometry of font number `font`
/// (counted from 0 here) for `text` at (`x`, `y`) made with `MakeString`
/// (`alignment`, `color` and `flag` go to its slots where
/// [`debug_text_create_debug_text`] passes its alignment code, color pointer
/// and 1), attached to the node, which is rotated by three fixed angles,
/// scaled with `|height|` and given a property with two settings. Returns
/// the node, or null when `MakeString` makes nothing.
// Translation incomplete: 00a10770 - C++ exception unwinding (SEH frame) is not modeled.
#[allow(clippy::too_many_arguments)] // The exe ABI has eight stack arguments.
pub fn fn_00a10770(
    e: &mut Engine,
    text: u32,
    x: f32,
    y: f32,
    height: f32,
    font: u32,
    alignment: u32,
    color: u32,
    flag: u8,
) -> Ptr {
    let node = new_floating_text_node(e);
    name_node(e, node);
    e.with_stack(8 + 4, |e, locals| {
        let string = locals.addr();
        let width_slot = locals.addr() + 8;
        e.mem.set_u32(width_slot, 10000);
        e.call(STRING_FROM_TEXT, &args![string, text]);
        let fonts: u32 = e.global(FONT_MANAGER);
        let font_object = e.mem.u32(fonts.wrapping_add(font.wrapping_mul(4)));
        let geometry = e
            .call(
                MAKE_STRING,
                &args![
                    font_object,
                    x,
                    y,
                    0.0f32,
                    string,
                    width_slot,
                    alignment,
                    color,
                    0u32,
                    flag as u32
                ],
            )
            .u32();
        if geometry == 0 {
            string_set(e, string, 0);
            return Ptr::NULL;
        }
        attach_child(e, node, geometry);
        let first = rotation_about_first_axis(e.global(ANGLE_FIRST));
        rotate_into(e, geometry + 0x34, &[first]);
        for lane in 0..3u32 {
            e.mem.set_f32(node + 0x58 + 4 * lane, 0.0);
        }
        let second = rotation_about_first_axis(e.global(ANGLE_SECOND));
        let third = rotation_about_second_axis(e.global(ANGLE_THIRD));
        rotate_into(e, node + 0x34, &[second, third]);
        let scale = e.call(FABS, &args![height as f64]).f64() as f32;
        e.mem.set_f32(node + 0x64, scale);
        attach_alpha_property(e, node);
        e.call(UPDATE_PROPERTIES, &args![node]);
        e.with_stack(12, |e, update_data| {
            e.call(
                UPDATE_DATA_CONSTRUCTOR,
                &args![update_data, 0.0f32, 0u32, 0u32],
            );
            e.call(UPDATE, &args![node, update_data]);
        });
        string_set(e, string, 0);
        Ptr::new(node)
    })
}

/// Names `node` `"FloatingText"` through a temporary `NiFixedString`.
fn name_node(e: &mut Engine, node: u32) {
    let handle = e
        .call(FIXED_STRING_CREATE, &args![FLOATING_TEXT_NAME])
        .u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), handle);
        e.call(SET_NAME, &args![node, slot]);
        e.call(FIXED_STRING_RELEASE, &args![slot]);
    });
}

/// Names `node` `"FloatingText"` and releases the handle by hand (the
/// version `00a10c00` uses).
fn name_node_release_by_hand(e: &mut Engine, node: u32) {
    let handle = e
        .call(FIXED_STRING_CREATE, &args![FLOATING_TEXT_NAME])
        .u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), handle);
        e.call(SET_NAME, &args![node, slot]);
    });
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// Builds the 0x1C-byte property both floating text builders attach, sets
/// it twice (`00633ba0(1, 1)` and `00633ba0(0, 0x2000)`) and attaches it.
fn attach_alpha_property(e: &mut Engine, node: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x1cu32]).u32();
    let property = if block != 0 {
        e.call(PROPERTY_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    e.call(PROPERTY_SETTER, &args![property, 1u32, 1u32]);
    e.call(PROPERTY_SETTER, &args![property, 0u32, 0x2000u32]);
    e.call(ATTACH_PROPERTY, &args![node, property]);
    property
}

// Translated from 00a10c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows `text` floating above `node` (the model of a reference): finds the
/// `DebugTextExtraData` of the node with the same `height` (`00a11740`),
/// updates the dying time of the floating text it belongs to when that is
/// still in the list, else builds the billboard node again; without extra
/// data builds both. The billboard node gets the font number 1 string
/// geometry at the world bound's height plus `height`, rotated and scaled
/// like [`fn_00a10770`]'s, is attached to `node` and a new floating text
/// element (position, `fDie = die`, text, billboard) is appended to the list
/// at the end. Returns false for a null node or when `MakeString` makes
/// nothing.
// Translation incomplete: 00a10c00 - C++ exception unwinding (SEH frame) is not modeled.
pub fn fn_00a10c00(e: &mut Engine, node: Ptr, text: u32, height: i32, die: f32) -> bool {
    if node.is_null() {
        return false;
    }
    let bound = e.mem.u32(node.addr() + 0x20);
    let bound = if bound != 0 { bound } else { DEFAULT_BOUND };
    let center = e.mem.f32(bound + 8);
    let old_center = e.call(FTOL, &args![center as f64]).i32();
    // `00a11740` writes the old center of every extra data it examines here.
    e.with_stack(4, |e, old_center_slot| {
        e.mem.set_i32(old_center_slot.addr(), old_center);
        let found = fn_00a11740(e, node, 0, height, old_center_slot);
        let billboard;
        let extra;
        let mut reuse_extra = false;
        if found.is_null() {
            billboard = new_floating_text_node(e);
            name_node_release_by_hand(e, billboard);
            let block = e.call(NEW_OBJECT, &args![0x20u32]).u32();
            let old_center = e.mem.i32(old_center_slot.addr());
            extra = if block != 0 {
                fn_00a11a80(
                    e,
                    Ptr::new(block),
                    Ptr::new(billboard),
                    text,
                    height,
                    old_center,
                )
                .addr()
            } else {
                0
            };
        } else {
            let this = debug_text_instance(e, 1);
            let list = this.at(DebugText::xFloatingTextList);
            let wanted = e.mem.u32(found.addr() + DebugTextExtraData::pBillboard.off);
            let mut cursor = e.get(list, NiTPointerList::m_pkHead);
            while cursor != 0 {
                debug_text_instance(e, 1);
                let element = e.mem.u32(cursor + 8);
                cursor = e.mem.u32(cursor);
                if e.mem.u32(element + DebugTextData::spNode.off) == wanted {
                    e.mem.set_f32(element + DebugTextData::fDie.off, die);
                    return true;
                }
            }
            // Not in the list: the old billboard leaves the node and a new
            // one is built for the same extra data.
            e.vcall(node.addr(), 0xe8, &args![wanted]);
            billboard = new_floating_text_node(e);
            name_node_release_by_hand(e, billboard);
            reuse_extra = true;
            extra = found.addr();
        }
        build_floating_text(
            e,
            node.addr(),
            text,
            height,
            die,
            billboard,
            extra,
            reuse_extra,
        )
    })
}

/// The common part of [`fn_00a10c00`]: builds the string geometry for
/// `billboard`, rotates and scales it, attaches the property, the extra data
/// and the node and appends the new element to the floating text list.
#[allow(clippy::too_many_arguments)]
fn build_floating_text(
    e: &mut Engine,
    node: u32,
    text: u32,
    height: i32,
    die: f32,
    billboard: u32,
    extra: u32,
    reuse_extra: bool,
) -> bool {
    let anchor = e
        .mem
        .i32(extra + DebugTextExtraData::iOldCenter.off)
        .wrapping_add(height);
    e.with_stack(8 + 4, |e, locals| {
        let string = locals.addr();
        let width_slot = locals.addr() + 8;
        e.mem.set_u32(width_slot, 10000);
        e.call(STRING_FROM_TEXT, &args![string, text]);
        let fonts: u32 = e.global(FONT_MANAGER);
        let font_object = e.mem.u32(fonts.wrapping_add(4));
        let geometry = e
            .call(
                MAKE_STRING,
                &args![
                    font_object,
                    0.0f32,
                    0.0f32,
                    0.0f32,
                    string,
                    width_slot,
                    2u32,
                    FLOATING_TEXT_COLOR,
                    0u32,
                    1u32
                ],
            )
            .u32();
        if geometry == 0 {
            string_set(e, string, 0);
            return false;
        }
        attach_child(e, billboard, geometry);
        string_set(e, extra + DebugTextExtraData::xString.off, text);
        let first = rotation_about_first_axis(e.global(ANGLE_FIRST));
        rotate_into(e, geometry + 0x34, &[first]);
        e.mem.set_f32(billboard + 0x58, 0.0);
        e.mem.set_f32(billboard + 0x5c, 0.0);
        e.mem.set_f32(billboard + 0x60, anchor as f32);
        let second = rotation_about_first_axis(e.global(ANGLE_SECOND));
        let third = rotation_about_second_axis(e.global(ANGLE_THIRD));
        rotate_into(e, billboard + 0x34, &[second, third]);
        let half: f32 = e.global(HALF);
        let scale = e.call(FABS, &args![half as f64]).f64() as f32;
        e.mem.set_f32(billboard + 0x64, scale);
        // Raw store, no reference counting, as the exe does.
        e.mem
            .set_u32(extra + DebugTextExtraData::pBillboard.off, billboard);
        attach_alpha_property(e, billboard);
        e.call(PREPARE_OBJECT, &args![geometry, 0u32, 0u32]);
        if !reuse_extra {
            e.call(ADD_EXTRA_DATA, &args![node, extra]);
        }
        attach_child(e, node, billboard);
        e.call(UPDATE_PROPERTIES, &args![billboard]);
        e.with_stack(12, |e, update_data| {
            e.call(
                UPDATE_DATA_CONSTRUCTOR,
                &args![update_data, 0.0f32, 0u32, 0u32],
            );
            e.call(UPDATE, &args![billboard, update_data]);
        });
        let block = allocate_block(e, 0x2c);
        let element = if block != 0 {
            fn_00a11880(e, Ptr::new(block))
        } else {
            Ptr::NULL
        };
        e.set(element, DebugTextData::fX, anchor as f32);
        e.set(element, DebugTextData::fY, anchor as f32);
        e.set(element, DebugTextData::fDie, die);
        string_set(e, element.at(DebugTextData::pText).addr(), text);
        set_node(e, element.addr() + DebugTextData::spNode.off, billboard);
        let this = debug_text_instance(e, 1);
        let list = this.at(DebugText::xFloatingTextList);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), element.addr());
            e.call(LIST_ADD_TAIL, &args![list, slot]);
        });
        string_set(e, string, 0);
        true
    })
}

// Translated from 00a11470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows `text` floating above `object` when it is a node or has a node as
/// parent ([`fn_00a10c00`]); false when neither.
pub fn fn_00a11470(e: &mut Engine, object: Ptr, text: u32, height: i32, die: f32) -> bool {
    let node = node_of(e, object.addr());
    if node == 0 {
        return false;
    }
    fn_00a10c00(e, Ptr::new(node), text, height, die)
}

// Translated from 00a11560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the floating texts of `height` (all when negative) from `object`
/// when it is a node or has a node as parent ([`fn_00a11640`]).
pub fn fn_00a11560(e: &mut Engine, object: Ptr, height: i32) {
    let node = node_of(e, object.addr());
    if node != 0 {
        fn_00a11640(e, Ptr::new(node), height);
    }
}

// Translated from 00a11640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the extra data of `node` (`+0x10` array, `+0x14` count) and, for
/// each `DebugTextExtraData`, takes its billboard (`+0xc`) out of the node
/// (virtual slot `+0xe8`): all of them when `height` is negative, else the
/// first whose `iHeight` equals `height`, which ends the walk.
pub fn fn_00a11640(e: &mut Engine, node: Ptr, height: i32) {
    let count = e.mem.u16(node.addr() + 0x14) as u32;
    let mut position = 0u32;
    while position < count {
        let array = e.mem.u32(node.addr() + 0x10);
        let object = e.mem.u32(array + 4 * position);
        position += 1;
        if is_debug_text_extra_data(e, object) {
            if height < 0 {
                let billboard = e.mem.u32(object + DebugTextExtraData::pBillboard.off);
                e.vcall(node.addr(), 0xe8, &args![billboard]);
            } else if height == e.mem.i32(object + DebugTextExtraData::iHeight.off) {
                let billboard = e.mem.u32(object + DebugTextExtraData::pBillboard.off);
                e.vcall(node.addr(), 0xe8, &args![billboard]);
                return;
            }
        }
    }
}

// Translated from 00a11740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Searches the extra data of `node` (`+0x10` array, `+0x14` count) for a
/// `DebugTextExtraData` whose `iHeight` is `height`. Each such extra data
/// examined writes its `iOldCenter` to `old_center`. Returns the match or
/// null. The second stack word is not read.
pub fn fn_00a11740(e: &mut Engine, node: Ptr, _unused_1: u32, height: i32, old_center: Ptr) -> Ptr {
    if node.is_null() {
        return Ptr::NULL;
    }
    let count = e.mem.u16(node.addr() + 0x14) as u32;
    let mut position = 0u32;
    while position < count {
        let array = e.mem.u32(node.addr() + 0x10);
        let object = e.mem.u32(array + 4 * position);
        position += 1;
        if is_debug_text_extra_data(e, object) {
            let center = e.mem.u32(object + DebugTextExtraData::iOldCenter.off);
            e.mem.set_u32(old_center.addr(), center);
            if height == e.mem.i32(object + DebugTextExtraData::iHeight.off) {
                return Ptr::new(object);
            }
        }
    }
    Ptr::NULL
}

// Translated from 00a11820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::Lock` (Xbox PDB): enters the critical section while the
/// debug text is visible.
pub fn debug_text_lock(e: &mut Engine) {
    if e.call(IS_DEBUG_TEXT_VISIBLE, &[]).bool() {
        e.call(ENTER_CRITICAL_SECTION, &args![CRITICAL_SECTION]);
    }
}

// Translated from 00a11840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::Unlock` (Xbox PDB): leaves the critical section while the
/// debug text is visible.
pub fn debug_text_unlock(e: &mut Engine) {
    if e.call(IS_DEBUG_TEXT_VISIBLE, &[]).bool() {
        e.call(LEAVE_CRITICAL_SECTION, &args![CRITICAL_SECTION]);
    }
}

// Translated from 00a11880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::DebugTextData` constructor: no node, empty text, zero
/// position, justification and colors, `fDie = -1.0`. Returns `this`.
// Translation incomplete: 00a11880 - C++ exception unwinding (SEH frame) is not modeled.
pub fn fn_00a11880(e: &mut Engine, this: Ptr<DebugTextData>) -> Ptr<DebugTextData> {
    e.set(this, DebugTextData::spNode, Ptr::NULL);
    let text = this.at(DebugTextData::pText);
    e.set(text, BSStringT::pString, 0);
    e.set(text, BSStringT::sLen, 0);
    e.set(text, BSStringT::sMaxLen, 0);
    let color = this.at(DebugTextData::Color);
    e.set(color, NiColorA::r, 0.0);
    e.set(color, NiColorA::g, 0.0);
    e.set(color, NiColorA::b, 0.0);
    e.set(color, NiColorA::a, 0.0);
    e.set(this, DebugTextData::fX, 0.0);
    e.set(this, DebugTextData::fY, 0.0);
    e.set(this, DebugTextData::iJustify, 0);
    set_node(e, this.addr() + DebugTextData::spNode.off, 0);
    string_set(e, text.addr(), 0);
    let never = e.global(MINUS_ONE);
    e.set(this, DebugTextData::fDie, never);
    this
}

// Translated from 00a119d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugText::DebugTextData` destructor: empties the text and releases the
/// node reference (deleting the node when it was the last one).
// Translation incomplete: 00a119d0 - C++ exception unwinding (SEH frame) is not modeled.
pub fn fn_00a119d0(e: &mut Engine, this: Ptr<DebugTextData>) {
    let text = this.at(DebugTextData::pText).addr();
    string_set(e, text, 0);
    string_set(e, text, 0);
    let node = e.get(this, DebugTextData::spNode);
    if !node.is_null() {
        let remaining = e.call(INTERLOCKED_DECREMENT, &args![node.addr() + 4]).i32();
        if remaining == 0 {
            e.vcall(node.addr(), 4, &[]);
        }
    }
}

// Translated from 00a11a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugTextExtraData` constructor: runs the `NiExtraData` constructor,
/// sets the vtable, the billboard, the text (`Set(text, 0)`), the height and
/// the old center. Returns `this`.
// Translation incomplete: 00a11a80 - C++ exception unwinding (SEH frame) is not modeled.
pub fn fn_00a11a80(
    e: &mut Engine,
    this: Ptr<DebugTextExtraData>,
    billboard: Ptr,
    text: u32,
    height: i32,
    old_center: i32,
) -> Ptr<DebugTextExtraData> {
    e.call(EXTRA_DATA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), EXTRA_DATA_VTABLE);
    let string = this.at(DebugTextExtraData::xString);
    e.set(string, BSStringT::pString, 0);
    e.set(string, BSStringT::sLen, 0);
    e.set(string, BSStringT::sMaxLen, 0);
    e.set(this, DebugTextExtraData::pBillboard, billboard);
    string_set(e, string.addr(), text);
    e.set(this, DebugTextExtraData::iHeight, height);
    e.set(this, DebugTextExtraData::iOldCenter, old_center);
    this
}

// Translated from 00a11b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugTextExtraData::GetRTTI` (Xbox PDB): the class's `NiRTTI` record.
pub fn debug_text_extra_data_get_rtti(_e: &mut Engine, _this: Ptr<DebugTextExtraData>) -> u32 {
    EXTRA_DATA_RTTI
}

// Translated from 00a11b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugTextExtraData::_scalar_deleting_destructor_` (Xbox PDB): empties
/// the text, runs the `NiExtraData` destructor and, when bit 0 of `flags` is
/// set, deletes the 0x20-byte block. Returns `this`.
// Translation incomplete: 00a11b80 - C++ exception unwinding (SEH frame) is not modeled.
pub fn debug_text_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<DebugTextExtraData>,
    flags: u32,
) -> Ptr<DebugTextExtraData> {
    string_set(e, this.at(DebugTextExtraData::xString).addr(), 0);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(DELETE_OBJECT, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00a11c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the list element at `*item_slot` from the floating text list
/// (`NiTPointerList::Remove`): finds its node and removes it
/// (`00a11e10`), returning what that returns; when the element is not in
/// the list returns the element itself. `item_slot` is the address of a word
/// holding the element.
pub fn fn_00a11c40(e: &mut Engine, this: Ptr<NiTPointerList>, item_slot: Ptr) -> u32 {
    let wanted = e.mem.u32(item_slot.addr());
    let mut cursor = e.get(this, NiTPointerList::m_pkHead);
    let mut found = 0u32;
    while cursor != 0 {
        let node = cursor;
        let element_address = cursor + 8;
        cursor = e.mem.u32(cursor);
        if wanted == e.mem.u32(element_address) {
            found = node;
            break;
        }
    }
    if found != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), found);
            e.call(LIST_REMOVE_NODE, &args![this, slot]).u32()
        })
    } else {
        wanted
    }
}

// Translated from 00a11cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<DebugText::sPrintData *, 1024>` constructor: sets the
/// vtable and runs the array constructor body (`00afcde0(this, 0, 0)`).
/// Returns `this`.
pub fn fn_00a11cd0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), PRINT_ARRAY_VTABLE);
    e.call(ARRAY_CONSTRUCTOR, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00a11d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<DebugText::sPrintData *, 1024>` destructor (Xbox PDB):
/// sets the vtable and clears the array (`Clear(this, 1)`).
pub fn bs_simple_array_print_data_destructor(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), PRINT_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00a11d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the pointer stored at `item` to the array: makes room
/// (`00a11ee0`, which returns the index) and stores it. Returns the index.
/// The body is shared with other `BSSimpleArray`s of pointers.
pub fn fn_00a11d20(e: &mut Engine, this: Ptr<BSSimpleArray>, item: Ptr) -> u32 {
    let index = e.call(ARRAY_GROW, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let value = e.mem.u32(item.addr());
    e.mem.set_u32(buffer + 4 * index, value);
    index
}

// Translated from 00a11d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<DebugText::sPrintData *, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor and, when bit 0 of `flags` is set, frees the
/// block. Returns `this`.
pub fn bs_simple_array_print_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    bs_simple_array_print_data_destructor(e, this);
    if flags & 1 != 0 {
        free_block(e, this.addr());
    }
    this
}

// Translated from 00a1c2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<..DebugText::DebugTextData *>::RemoveAll` (Xbox PDB):
/// frees every node of the list and empties it.
pub fn nitpointerlist_remove_all(e: &mut Engine, this: Ptr<NiTPointerList>) {
    let mut node = e.get(this, NiTPointerList::m_pkHead);
    while node != 0 {
        let next = e.mem.u32(node);
        e.mem.set_u32(node + 8, 0);
        free_block(e, node);
        node = next;
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a0d9e0, debug_text_instance(u8) -> Ptr<DebugText>),
        entry!(0x00a0dae0, fn_00a0dae0()),
        entry!(0x00a0db40, fn_00a0db40(Ptr<DebugText>) -> Ptr<DebugText>),
        entry!(0x00a0dcc0, debug_text_destructor(Ptr<DebugText>)),
        entry!(0x00a0de80, debug_text_delete_all_debug_text(Ptr<DebugText>)),
        entry!(0x00a0df80, fn_00a0df80()),
        entry!(0x00a0e2a0, debug_text_reset(Ptr<DebugText>)),
        entry!(0x00a0e430, debug_text_skip_line(Ptr<DebugText>, i32, i32)),
        (0x00a0e470, print_x_entry as AbiFn),
        (0x00a0e4b0, print_x_ov2_entry as AbiFn),
        entry!(
            0x00a0e4f0,
            debug_text_va_print_x(Ptr<DebugText>, i32, i32, i32, u32, u32, u32) -> u32
        ),
        (0x00a0e580, print_line_entry as AbiFn),
        entry!(
            0x00a0e5c0,
            debug_text_va_print_line(Ptr<DebugText>, i32, i32, u32, u32, u32) -> u32
        ),
        (0x00a0e690, print_x_c_entry as AbiFn),
        (0x00a0e6d0, print_x_c_ov2_entry as AbiFn),
        (0x00a0e710, print_line_c_entry as AbiFn),
        entry!(
            0x00a0e750,
            debug_text_get_current_line(Ptr<DebugText>, i32) -> i32
        ),
        entry!(0x00a0e770, debug_text_handle_queued_prints(Ptr<DebugText>)),
        entry!(0x00a0f730, fn_00a0f730(Ptr<DebugText>, u32, i32) -> i32),
        entry!(
            0x00a0f8b0,
            debug_text_print(Ptr<DebugText>, u32, f32, f32, i32, i32, f32, i32, Ptr) -> u32
        ),
        entry!(
            0x00a0fe20,
            debug_text_create_debug_text(Ptr<DebugText>, u32, f32, f32, i32, i32, Ptr) -> Ptr
        ),
        entry!(
            0x00a103e0,
            fn_00a103e0(Ptr<DebugText>, u32, f32, f32, i32) -> bool
        ),
        entry!(
            0x00a10630,
            fn_00a10630(Ptr<DebugText>, f32, f32, i32) -> u32
        ),
        entry!(
            0x00a10770,
            fn_00a10770(u32, f32, f32, f32, u32, u32, u32, u8) -> Ptr
        ),
        entry!(0x00a10c00, fn_00a10c00(Ptr, u32, i32, f32) -> bool),
        entry!(0x00a11470, fn_00a11470(Ptr, u32, i32, f32) -> bool),
        entry!(0x00a11560, fn_00a11560(Ptr, i32)),
        entry!(0x00a11640, fn_00a11640(Ptr, i32)),
        entry!(0x00a11740, fn_00a11740(Ptr, u32, i32, Ptr) -> Ptr),
        entry!(0x00a11820, debug_text_lock()),
        entry!(0x00a11840, debug_text_unlock()),
        entry!(
            0x00a11860,
            debug_text_set_current_line(Ptr<DebugText>, i32, i32)
        ),
        entry!(
            0x00a11880,
            fn_00a11880(Ptr<DebugTextData>) -> Ptr<DebugTextData>
        ),
        entry!(0x00a119d0, fn_00a119d0(Ptr<DebugTextData>)),
        entry!(
            0x00a11a80,
            fn_00a11a80(Ptr<DebugTextExtraData>, Ptr, u32, i32, i32) -> Ptr<DebugTextExtraData>
        ),
        entry!(
            0x00a11b70,
            debug_text_extra_data_get_rtti(Ptr<DebugTextExtraData>) -> u32
        ),
        entry!(
            0x00a11b80,
            debug_text_extra_data_scalar_deleting_destructor(
                Ptr<DebugTextExtraData>,
                u32,
            )
                -> Ptr<DebugTextExtraData>
        ),
        entry!(0x00a11c40, fn_00a11c40(Ptr<NiTPointerList>, Ptr) -> u32),
        entry!(
            0x00a11cd0,
            fn_00a11cd0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x00a11d00,
            bs_simple_array_print_data_destructor(Ptr<BSSimpleArray>)
        ),
        entry!(0x00a11d20, fn_00a11d20(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(
            0x00a11d80,
            bs_simple_array_print_data_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(0x00a1c2d0, nitpointerlist_remove_all(Ptr<NiTPointerList>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A root node (the "debug text root"), its vtable and the node, extra
    /// data and `NiRTTI` stand-ins the tests use. Slots: `+4` DeleteThis,
    /// `+8` GetRTTI, `+0xdc` AttachChild, `+0xe8` RemoveChild.
    const ROOT: u32 = 0x0800_2000;
    const ROOT_VTABLE: u32 = 0x0800_0000;
    const NODE_VTABLE: u32 = 0x0800_1000;
    const OTHER_VTABLE: u32 = 0x0800_1400;
    const ATTACH: u32 = 0x0900_00dc;
    const REMOVE: u32 = 0x0900_00e8;
    const DELETE_THIS: u32 = 0x0900_0004;
    const NODE_RTTI_TARGET: u32 = 0x0900_0008;
    const OTHER_RTTI_TARGET: u32 = 0x0900_000c;
    const OTHER_RTTI_RECORD: u32 = 0x0800_3000;
    const EXTRA_CLASS_VTABLE: u32 = 0x0800_1800;

    type Log = Vec<(u32, Vec<u32>)>;
    /// The argument words of every `MakeString` call.
    type MadeStrings = Rc<RefCell<Vec<(Vec<u32>, String)>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn put_text(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text.as_bytes());
        block
    }

    fn text_at(e: &Engine, string: u32) -> Option<String> {
        let pointer = e.mem.u32(string);
        (pointer != 0).then(|| String::from_utf8(e.mem.cstr(pointer)).unwrap())
    }

    fn set_string(e: &mut Engine, string: u32, text: &str) {
        let copy = put_text(e, text);
        e.mem.set_u32(string, copy);
        e.mem.set_u16(string + 4, text.len() as u16);
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn take_log(e: &mut Engine) -> Log {
        e.call_log.take().unwrap_or_default()
    }

    fn vtable(e: &mut Engine, address: u32, slots: &[(u32, u32)]) {
        e.mem.map(address, 0x100);
        for (slot, target) in slots {
            e.mem.set_u32(address + slot, *target);
        }
    }

    // ---- doubles shared by the tests ------------------------------------

    /// `BSStringT::Set(this, text, 0)`: copies the text into a new block.
    fn string_set_double(e: &mut Engine, a: &[u32]) -> Ret {
        let (string, text) = (a[0], a[1]);
        if text == 0 {
            e.mem.set_u32(string, 0);
            e.mem.set_u16(string + 4, 0);
        } else {
            let bytes = e.mem.cstr(text);
            let copy = put_text(e, std::str::from_utf8(&bytes).unwrap());
            e.mem.set_u32(string, copy);
            e.mem.set_u16(string + 4, bytes.len() as u16);
        }
        ret(string)
    }

    /// `BSStringT::Set(this, other string)`.
    fn string_copy_double(e: &mut Engine, a: &[u32]) -> Ret {
        let source = e.mem.u32(a[1]);
        string_set_double(e, &[a[0], source, 0]);
        ret(a[0])
    }

    fn set_node_double(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0], a[1]);
        ret(a[0])
    }

    /// `BSSimpleArray` growth: room for one more, returns the new index.
    fn array_grow_double(e: &mut Engine, a: &[u32]) -> Ret {
        let array = a[0];
        let size = e.mem.u32(array + 8);
        let reserved = e.mem.u32(array + 12);
        if size == reserved {
            let new_reserved = reserved + 8;
            let block = e.mem.alloc(new_reserved * 4);
            let old = e.mem.u32(array + 4);
            for i in 0..size {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(block + 4 * i, word);
            }
            e.mem.set_u32(array + 4, block);
            e.mem.set_u32(array + 12, new_reserved);
        }
        e.mem.set_u32(array + 8, size + 1);
        ret(size)
    }

    fn array_clear_double(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0] + 8, 0);
        ret(0)
    }

    /// `NiTPointerList::RemoveNode(this, &node)`: hands back the node's
    /// element.
    fn list_remove_node_double(e: &mut Engine, a: &[u32]) -> Ret {
        let node = e.mem.u32(a[1]);
        ret(e.mem.u32(node + 8))
    }

    /// `NiTPointerList::AddTail(this, &element)`: links a new node.
    fn list_add_tail_double(e: &mut Engine, a: &[u32]) -> Ret {
        let list = a[0];
        let element = e.mem.u32(a[1]);
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, element);
        let tail = e.mem.u32(list + 4);
        if tail == 0 {
            e.mem.set_u32(list, node);
        } else {
            e.mem.set_u32(tail, node);
            e.mem.set_u32(node + 4, tail);
        }
        e.mem.set_u32(list + 4, node);
        let count = e.mem.u32(list + 8);
        e.mem.set_u32(list + 8, count + 1);
        ret(node)
    }

    fn interlocked_decrement_double(e: &mut Engine, a: &[u32]) -> Ret {
        let value = e.mem.i32(a[0]).wrapping_sub(1);
        e.mem.set_i32(a[0], value);
        ret(value as u32)
    }

    /// An engine with the exe constants the code reads and doubles for every
    /// callee outside this file. The debug text is visible, the debug text
    /// root is [`ROOT`].
    fn world() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_6000,
            0x0101_e000,
            0x0101_f000,
            0x0102_1000,
            0x0102_b000,
            0x0104_f000,
            0x0107_4000,
            0x0109_4000,
            0x0109_b000,
            0x011a_9000,
            0x011c_7000,
            0x011d_e000,
            0x011f_3000,
            0x011f_4000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(COLOR_DIVISOR, 255.0f64);
        e.set_global(SCREEN_HALF_WIDTH, 640.0f64);
        e.set_global(WIDE_SCALE, 1.33333f32);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(TEXT_DEPTH, -50.0f32);
        e.set_global(ANGLE_FIRST, std::f32::consts::FRAC_PI_2);
        e.set_global(ANGLE_SECOND, -std::f32::consts::FRAC_PI_2);
        e.set_global(ANGLE_THIRD, std::f32::consts::PI);
        e.set_global(HALF, 0.5f32);
        e.set_global(LINE_HEIGHT_BIAS, 3.0f64);
        e.set_global(WRAP_WIDTH_SCALE, 1.5f64);
        e.set_global(FRAME_TIME, 1.0f32);
        e.set_global(COLOR_SETTING_VALUE, 0xff80_4000u32);
        e.set_global(FONT_SETTING_VALUE, 1i32);
        e.set_global(LINE_HEIGHT_SETTING_VALUE, 5i32);
        e.set_global(LEFT_MARGIN_SETTING_VALUE, 20i32);
        // Two fonts; font number 1 has a metrics float of 14.0.
        let fonts = e.mem.alloc(16);
        let font_one = e.mem.alloc(0x40);
        let font_two = e.mem.alloc(0x40);
        let metrics = e.mem.alloc(4);
        e.mem.set_f32(metrics, 14.0);
        e.mem.set_u32(font_one + 0x38, metrics);
        e.mem.set_u32(fonts, font_one);
        e.mem.set_u32(fonts + 4, font_two);
        e.set_global(FONT_MANAGER, fonts);

        vtable(&mut e, ROOT_VTABLE, &[(0xdc, ATTACH), (0xe8, REMOVE)]);
        vtable(
            &mut e,
            NODE_VTABLE,
            &[
                (4, DELETE_THIS),
                (8, NODE_RTTI_TARGET),
                (0xdc, ATTACH),
                (0xe8, REMOVE),
            ],
        );
        vtable(
            &mut e,
            OTHER_VTABLE,
            &[(4, DELETE_THIS), (8, OTHER_RTTI_TARGET), (0xe8, REMOVE)],
        );
        vtable(&mut e, EXTRA_CLASS_VTABLE, &[(8, 0x00a1_1b70)]);
        e.mem.map(OTHER_RTTI_RECORD, 0x10);
        e.mem.map(ROOT, 0x100);
        e.mem.set_u32(ROOT, ROOT_VTABLE);

        e.register(SET_NODE, set_node_double);
        e.register(STRING_SET, string_set_double);
        e.register(STRING_COPY, string_copy_double);
        e.register(IS_DEBUG_TEXT_VISIBLE, |_, _| ret(1));
        e.register(GET_DEBUG_TEXT_ROOT, |_, _| ret(ROOT));
        e.register(IS_IN_MENU_MODE, |_, _| ret(0));
        e.register(ENTER_CRITICAL_SECTION, |_, _| ret(0));
        e.register(LEAVE_CRITICAL_SECTION, |_, _| ret(0));
        e.register(INTERLOCKED_DECREMENT, interlocked_decrement_double);
        e.register(ATTACH, |_, _| ret(0));
        e.register(REMOVE, |_, _| ret(0));
        e.register(DELETE_THIS, |_, _| ret(0));
        e.register(NODE_RTTI_TARGET, |_, _| ret(NODE_RTTI));
        e.register(OTHER_RTTI_TARGET, |_, _| ret(OTHER_RTTI_RECORD));
        e.register(NEW_OBJECT, |e, a| ret(e.mem.alloc(a[0])));
        e.register(DELETE_OBJECT, |e, a| {
            e.mem.free(a[0]);
            ret(0)
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register(ARRAY_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(ARRAY_CLEAR, array_clear_double);
        e.register(ARRAY_GROW, array_grow_double);
        e.register(LIST_REMOVE_NODE, list_remove_node_double);
        e.register(LIST_ADD_TAIL, list_add_tail_double);
        e
    }

    fn new_debug_text(e: &mut Engine) -> Ptr<DebugText> {
        e.new_object()
    }

    /// An entry of the table with a node (reference count 1 by default).
    fn show(
        e: &mut Engine,
        this: Ptr<DebugText>,
        index: u32,
        text: &str,
        x: f32,
        y: f32,
        justify: i32,
    ) -> u32 {
        let entry = entry_at(this, index);
        let node = e.mem.alloc(0xb4);
        e.mem.set_u32(node, NODE_VTABLE);
        e.mem.set_u32(node + 4, 1);
        e.set(entry, DebugTextData::spNode, Ptr::new(node));
        e.set(entry, DebugTextData::fX, x);
        e.set(entry, DebugTextData::fY, y);
        e.set(entry, DebugTextData::iJustify, justify);
        e.set(entry, DebugTextData::fDie, -1.0);
        set_string(e, entry.at(DebugTextData::pText).addr(), text);
        node
    }

    /// A queued item (appended straight to the queue's buffer).
    fn queue(
        e: &mut Engine,
        this: Ptr<DebugText>,
        kind: u32,
        text: &str,
        x: f32,
        y: f32,
        justify: i32,
    ) -> Ptr<SPrintData> {
        let data = Ptr::<DebugTextData>::new(e.mem.alloc(0x2c));
        e.set(data, DebugTextData::fX, x);
        e.set(data, DebugTextData::fY, y);
        e.set(data, DebugTextData::iJustify, justify);
        set_string(e, data.at(DebugTextData::pText).addr(), text);
        let print_data = Ptr::<SPrintData>::new(e.mem.alloc(0x18));
        e.set(print_data, SPrintData::pTextData, data);
        e.set(print_data, SPrintData::eType, kind);
        e.set(print_data, SPrintData::fDie, 2.5);
        let queue = this.at(DebugText::xPrintData);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, print_data.addr());
        array_grow_double(e, &[queue.addr()]);
        let size = e.get(queue, BSSimpleArray::iSize);
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        e.mem.set_u32(buffer + 4 * (size - 1), print_data.addr());
        e.mem.free(slot);
        print_data
    }

    // ---- instance, construction, destruction ---------------------------

    #[test]
    fn instance_is_created_once_under_the_allocation_tag() {
        let mut e = world();
        e.map(INSTANCE & !0xfff, 0x1000);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 7);
        // Without `create` nothing happens.
        assert!(debug_text_instance(&mut e, 0).is_null());
        e.call_log = Some(vec![]);
        let created = debug_text_instance(&mut e, 1);
        let log = take_log(&mut e);
        assert!(!created.is_null());
        assert_eq!(e.global::<u32>(INSTANCE), created.addr());
        assert_eq!(e.mem.block_size(created.addr()), Some(0x22a0));
        assert_eq!(e.mem.u32(created.addr()), VTABLE);
        assert_eq!(e.mem.u32(tls + 0x2b4), 7);
        assert_eq!(calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR).len(), 1);
        // The second call returns the same object without constructing.
        e.call_log = Some(vec![]);
        assert_eq!(debug_text_instance(&mut e, 1), created);
        assert!(calls_to(&take_log(&mut e), VECTOR_CONSTRUCTOR_ITERATOR).is_empty());
    }

    #[test]
    fn destroying_the_instance_frees_it_and_clears_the_pointer() {
        let mut e = world();
        let instance = debug_text_instance(&mut e, 1);
        e.call_log = Some(vec![]);
        fn_00a0dae0(&mut e);
        let log = take_log(&mut e);
        assert_eq!(e.global::<u32>(INSTANCE), 0);
        assert_eq!(e.mem.block_size(instance.addr()), None);
        assert_eq!(calls_to(&log, VECTOR_DESTRUCTOR_ITERATOR).len(), 1);
        // Nothing to destroy: no calls.
        e.call_log = Some(vec![]);
        fn_00a0dae0(&mut e);
        assert!(take_log(&mut e).is_empty());
    }

    #[test]
    fn constructor_initializes_the_tables() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        e.mem.set_u32(this.addr() + DebugText::iFontHeight.off, 99);
        e.mem.set_u32(this.addr() + DebugText::iHeight.off, 99);
        e.mem
            .set_u32(this.addr() + DebugText::iXLocation.off + 8, 99);
        e.call_log = Some(vec![]);
        assert_eq!(fn_00a0db40(&mut e, this), this);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(
            calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![this.addr() + 4, 0x2c, 200, 0x00a1_1880, 0x00a1_19d0]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_CONSTRUCTOR),
            vec![vec![this.addr() + 0x2270, 0, 0]]
        );
        for index in [0, 199] {
            let entry = entry_at(this, index);
            assert_eq!(e.get(entry, DebugTextData::fDie), -1.0);
            assert_eq!(e.get(entry, DebugTextData::iJustify), 0);
        }
        assert_eq!(e.get(this, DebugText::iFontHeight), 0);
        assert_eq!(e.mem.u32(this.addr() + DebugText::iHeight.off), 0);
        assert_eq!(e.mem.u32(this.addr() + DebugText::iXLocation.off + 8), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x2264 + 8), 0);
    }

    #[test]
    fn destructor_releases_texts_and_empties_the_containers() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        let shown = show(&mut e, this, 3, "hello", 1.0, 2.0, 1);
        // One floating text whose node has three references.
        let element = Ptr::<DebugTextData>::new(e.mem.alloc(0x2c));
        let billboard = e.mem.alloc(0xb4);
        e.mem.set_u32(billboard, NODE_VTABLE);
        e.mem.set_u32(billboard + 4, 3);
        e.mem.set_u32(billboard + 0x18, ROOT);
        e.set(element, DebugTextData::spNode, Ptr::new(billboard));
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, element.addr());
        let list = this.at(DebugText::xFloatingTextList);
        e.set(list, NiTPointerList::m_pkHead, node);
        e.set(list, NiTPointerList::m_pkTail, node);
        e.set(list, NiTPointerList::m_uiCount, 1);
        e.call_log = Some(vec![]);
        debug_text_destructor(&mut e, this);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        // The entry's node and the floating node were taken out of the root.
        assert_eq!(
            calls_to(&log, REMOVE),
            vec![vec![ROOT, shown], vec![ROOT, billboard]]
        );
        assert_eq!(e.get(entry_at(this, 3), DebugTextData::iJustify), 0);
        assert_eq!(
            calls_to(&log, LIST_REMOVE_NODE).len(),
            1,
            "the element is removed from the list"
        );
        assert_eq!(e.mem.block_size(element.addr()), None);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 0);
        assert_eq!(
            calls_to(&log, ARRAY_CLEAR),
            vec![vec![this.addr() + 0x2270, 1]]
        );
        assert_eq!(
            calls_to(&log, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![this.addr() + 4, 0x2c, 200, 0x00a1_19d0]]
        );
        assert_eq!(calls_to(&log, ENTER_CRITICAL_SECTION).len(), 2);
    }

    #[test]
    fn delete_all_clears_only_the_entries_that_have_a_node() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        let node = show(&mut e, this, 5, "abc", 3.0, 4.0, 2);
        e.set(entry_at(this, 6), DebugTextData::fX, 9.0);
        e.call_log = Some(vec![]);
        debug_text_delete_all_debug_text(&mut e, this);
        let log = take_log(&mut e);
        let entry = entry_at(this, 5);
        assert_eq!(calls_to(&log, REMOVE), vec![vec![ROOT, node]]);
        assert!(e.get(entry, DebugTextData::spNode).is_null());
        assert_eq!(e.get(entry, DebugTextData::iJustify), 0);
        assert_eq!(e.get(entry, DebugTextData::fX), 0.0);
        assert_eq!(e.get(entry, DebugTextData::fY), 0.0);
        assert_eq!(e.get(entry, DebugTextData::fDie), -1.0);
        assert_eq!(text_at(&e, entry.at(DebugTextData::pText).addr()), None);
        // An entry without a node keeps its other fields.
        assert_eq!(e.get(entry_at(this, 6), DebugTextData::fX), 9.0);
        assert_eq!(calls_to(&log, ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(calls_to(&log, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn frame_update_does_nothing_in_menu_mode() {
        let mut e = world();
        e.register(IS_IN_MENU_MODE, |_, _| ret(1));
        e.call_log = Some(vec![]);
        fn_00a0df80(&mut e);
        assert_eq!(take_log(&mut e).len(), 1);
    }

    #[test]
    fn frame_update_counts_down_entries_and_floating_texts() {
        let mut e = world();
        e.map(INSTANCE & !0xfff, 0x1000);
        let this = debug_text_instance(&mut e, 1);
        // Entry 0 runs out (0.5 - 1.0 < 0), entry 1 has 5.0 left, entry 2
        // never dies (-1.0).
        let node_zero = show(&mut e, this, 0, "a", 1.0, 1.0, 1);
        e.set(entry_at(this, 0), DebugTextData::fDie, 0.5);
        show(&mut e, this, 1, "b", 2.0, 2.0, 1);
        e.set(entry_at(this, 1), DebugTextData::fDie, 5.0);
        show(&mut e, this, 2, "c", 3.0, 3.0, 1);
        // Floating texts: A (one reference: dropped), B (time runs out:
        // taken out of the parent), C (time left).
        let list = this.at(DebugText::xFloatingTextList);
        // (element, billboard, list node) for each.
        let mut elements: Vec<(Ptr<DebugTextData>, u32, u32)> = vec![];
        for (references, die) in [(1u32, 9.0f32), (2, 0.25), (2, 4.0)] {
            let element = Ptr::<DebugTextData>::new(e.mem.alloc(0x2c));
            let billboard = e.mem.alloc(0xb4);
            e.mem.set_u32(billboard, NODE_VTABLE);
            e.mem.set_u32(billboard + 4, references);
            e.mem.set_u32(billboard + 0x18, ROOT);
            e.set(element, DebugTextData::spNode, Ptr::new(billboard));
            e.set(element, DebugTextData::fDie, die);
            let node = e.mem.alloc(12);
            e.mem.set_u32(node + 8, element.addr());
            match elements.last() {
                Some((_, _, previous)) => e.mem.set_u32(*previous, node),
                None => e.set(list, NiTPointerList::m_pkHead, node),
            }
            elements.push((element, billboard, node));
        }
        e.call_log = Some(vec![]);
        fn_00a0df80(&mut e);
        let log = take_log(&mut e);
        // Entry 0 released; the others only counted down.
        assert!(e.get(entry_at(this, 0), DebugTextData::spNode).is_null());
        assert_eq!(e.get(entry_at(this, 1), DebugTextData::fDie), 4.0);
        assert_eq!(e.get(entry_at(this, 2), DebugTextData::fDie), -1.0);
        assert!(calls_to(&log, REMOVE).contains(&vec![ROOT, node_zero]));
        // A and B were removed from the list and destroyed, C counted down.
        assert_eq!(calls_to(&log, LIST_REMOVE_NODE).len(), 2);
        assert_eq!(e.mem.block_size(elements[0].0.addr()), None);
        assert_eq!(e.mem.block_size(elements[1].0.addr()), None);
        assert_eq!(e.get(elements[2].0, DebugTextData::fDie), 3.0);
        assert!(calls_to(&log, REMOVE).contains(&vec![ROOT, elements[1].1]));
    }

    /// One floating text with a shared node (two references) and `die`.
    fn one_floating_text(e: &mut Engine, this: Ptr<DebugText>, die: f32) -> Ptr<DebugTextData> {
        let element = Ptr::<DebugTextData>::new(e.mem.alloc(0x2c));
        let billboard = e.mem.alloc(0xb4);
        e.mem.set_u32(billboard, NODE_VTABLE);
        e.mem.set_u32(billboard + 4, 2);
        e.mem.set_u32(billboard + 0x18, ROOT);
        e.set(element, DebugTextData::spNode, Ptr::new(billboard));
        e.set(element, DebugTextData::fDie, die);
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, element.addr());
        e.set(
            this.at(DebugText::xFloatingTextList),
            NiTPointerList::m_pkHead,
            node,
        );
        element
    }

    #[test]
    fn frame_update_releases_an_entry_at_exactly_zero_but_not_a_floating_text() {
        let mut e = world();
        e.map(INSTANCE & !0xfff, 0x1000);
        let this = debug_text_instance(&mut e, 1);
        // 1.0 - 1.0 = 0.0: the table entry is released (`TEST AH,0x41` /
        // `JP` at 00a0e01f), the floating text is not (`TEST AH,0x5` at
        // 00a0e1dd).
        let node = show(&mut e, this, 0, "a", 1.0, 1.0, 1);
        e.set(entry_at(this, 0), DebugTextData::fDie, 1.0);
        let element = one_floating_text(&mut e, this, 1.0);
        e.call_log = Some(vec![]);
        fn_00a0df80(&mut e);
        let log = take_log(&mut e);
        let entry = entry_at(this, 0);
        assert!(e.get(entry, DebugTextData::spNode).is_null());
        assert_eq!(e.get(entry, DebugTextData::fDie), -1.0);
        assert_eq!(calls_to(&log, REMOVE), vec![vec![ROOT, node]]);
        assert_eq!(e.get(element, DebugTextData::fDie), 0.0);
        assert!(calls_to(&log, LIST_REMOVE_NODE).is_empty());
    }

    #[test]
    fn frame_update_releases_nothing_when_the_countdown_is_nan() {
        let mut e = world();
        e.map(INSTANCE & !0xfff, 0x1000);
        e.set_global(FRAME_TIME, f32::NAN);
        let this = debug_text_instance(&mut e, 1);
        // 0.5 - NaN is unordered: both release compares skip.
        let node = show(&mut e, this, 0, "a", 1.0, 1.0, 1);
        e.set(entry_at(this, 0), DebugTextData::fDie, 0.5);
        let element = one_floating_text(&mut e, this, 0.5);
        e.call_log = Some(vec![]);
        fn_00a0df80(&mut e);
        let log = take_log(&mut e);
        let entry = entry_at(this, 0);
        assert_eq!(e.get(entry, DebugTextData::spNode).addr(), node);
        assert!(e.get(entry, DebugTextData::fDie).is_nan());
        assert!(e.get(element, DebugTextData::fDie).is_nan());
        assert!(calls_to(&log, REMOVE).is_empty());
        assert!(calls_to(&log, LIST_REMOVE_NODE).is_empty());
    }

    #[test]
    fn reset_measures_the_font_and_places_the_columns() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        e.register(GET_FULL_HELP, |_, _| ret(0));
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            ret(value as i32 as u32)
        });
        debug_text_reset(&mut e, this);
        // 14.0 + 3.0 = 17; one row plus the margin setting 5 per column.
        assert_eq!(e.get(this, DebugText::iFontHeight), 17);
        for column in 0..3 {
            assert_eq!(debug_text_get_current_line(&mut e, this, column), 22);
        }
        assert_eq!(e.mem.i32(x_location_address(this, 0)), 20);
        assert_eq!(e.mem.i32(x_location_address(this, 1)), 0x500 - 20);
        assert_eq!(e.mem.i32(x_location_address(this, 2)), 640);
        // Full help: four rows. A font without metrics: height 3.
        e.register(GET_FULL_HELP, |_, _| ret(1));
        let fonts: u32 = e.global(FONT_MANAGER);
        let font = e.mem.u32(fonts);
        e.mem.set_u32(font + 0x38, 0);
        debug_text_reset(&mut e, this);
        assert_eq!(e.get(this, DebugText::iFontHeight), 3);
        assert_eq!(debug_text_get_current_line(&mut e, this, 1), 4 * 3 + 5);
    }

    #[test]
    fn skip_line_and_current_line_move_one_column() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        e.set(this, DebugText::iFontHeight, 17);
        debug_text_set_current_line(&mut e, this, 1, 100);
        debug_text_skip_line(&mut e, this, 1, 3);
        assert_eq!(debug_text_get_current_line(&mut e, this, 1), 151);
        assert_eq!(debug_text_get_current_line(&mut e, this, 0), 0);
        debug_text_skip_line(&mut e, this, 1, -2);
        assert_eq!(debug_text_get_current_line(&mut e, this, 1), 117);
    }

    // ---- the variable-argument printers -----------------------------------

    const VA_PRINT_X: u32 = 0x0900_0010;
    const VA_PRINT_LINE: u32 = 0x0900_0018;
    const FORMAT: u32 = 0x0800_5000;

    /// A `DebugText` whose `VAPrintX` / `VAPrintLine` slots record their
    /// arguments and the two `va_list` words they were given.
    #[allow(clippy::type_complexity)]
    fn printer() -> (
        Engine,
        Ptr<DebugText>,
        Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>>,
    ) {
        let mut e = world();
        let this = new_debug_text(&mut e);
        vtable(
            &mut e,
            0x0800_4000,
            &[(0x10, VA_PRINT_X), (0x18, VA_PRINT_LINE)],
        );
        e.mem.set_u32(this.addr(), 0x0800_4000);
        let seen = Rc::new(RefCell::new(Vec::new()));
        for (target, list_index) in [(VA_PRINT_X, 5usize), (VA_PRINT_LINE, 4usize)] {
            let seen = Rc::clone(&seen);
            e.register_double(target, move |e, a| {
                let list = a[list_index];
                let words = vec![e.mem.u32(list), e.mem.u32(list + 4)];
                seen.borrow_mut().push((a.to_vec(), words));
                ret(77)
            });
        }
        (e, this, seen)
    }

    #[test]
    fn print_x_uses_column_zero_and_justify_one() {
        let (mut e, this, seen) = printer();
        let result = e.call(0x00a0e470, &args![this, 100i32, FORMAT, 7u32, 8u32]);
        assert_eq!(result.u32(), 77);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let t = this.addr();
        assert_eq!(seen[0].0[..5], [t, 100, 0, 1, FORMAT]);
        assert_eq!(seen[0].0[6], 0);
        assert_eq!(seen[0].1, vec![7, 8]);
    }

    #[test]
    fn print_x_ov2_passes_column_and_justify() {
        let (mut e, this, seen) = printer();
        e.call(
            0x00a0e4b0,
            &args![this, 100i32, 2i32, 3i32, FORMAT, 7u32, 8u32],
        );
        let seen = seen.borrow();
        let t = this.addr();
        assert_eq!(seen[0].0[..5], [t, 100, 2, 3, FORMAT]);
        assert_eq!(seen[0].0[6], 0);
        assert_eq!(seen[0].1, vec![7, 8]);
    }

    #[test]
    fn print_line_calls_the_line_formatter() {
        let (mut e, this, seen) = printer();
        let result = e.call(0x00a0e580, &args![this, 1i32, 2i32, FORMAT, 7u32, 8u32]);
        assert_eq!(result.u32(), 77);
        let seen = seen.borrow();
        let t = this.addr();
        assert_eq!(seen[0].0[..4], [t, 1, 2, FORMAT]);
        assert_eq!(seen[0].0[5], 0);
        assert_eq!(seen[0].1, vec![7, 8]);
    }

    #[test]
    fn print_x_c_passes_the_color() {
        let (mut e, this, seen) = printer();
        e.call(
            0x00a0e690,
            &args![this, 100i32, 0xabcdu32, FORMAT, 7u32, 8u32],
        );
        let seen = seen.borrow();
        let t = this.addr();
        assert_eq!(seen[0].0[..5], [t, 100, 0, 1, FORMAT]);
        assert_eq!(seen[0].0[6], 0xabcd);
        assert_eq!(seen[0].1, vec![7, 8]);
    }

    #[test]
    fn print_x_c_ov2_passes_color_column_and_justify() {
        let (mut e, this, seen) = printer();
        e.call(
            0x00a0e6d0,
            &args![this, 100i32, 0xabcdu32, 2i32, 3i32, FORMAT, 7u32, 8u32],
        );
        let seen = seen.borrow();
        let t = this.addr();
        assert_eq!(seen[0].0[..5], [t, 100, 2, 3, FORMAT]);
        assert_eq!(seen[0].0[6], 0xabcd);
        assert_eq!(seen[0].1, vec![7, 8]);
    }

    #[test]
    fn print_line_c_passes_the_color() {
        let (mut e, this, seen) = printer();
        e.call(
            0x00a0e710,
            &args![this, 0xabcdu32, 1i32, 2i32, FORMAT, 7u32, 8u32],
        );
        let seen = seen.borrow();
        let t = this.addr();
        assert_eq!(seen[0].0[..4], [t, 1, 2, FORMAT]);
        assert_eq!(seen[0].0[5], 0xabcd);
        assert_eq!(seen[0].1, vec![7, 8]);
    }

    /// A `DebugText` whose `_vsnprintf` writes "hello 7"; `Print` runs for
    /// real and queues the item.
    fn formatter() -> (Engine, Ptr<DebugText>) {
        let mut e = world();
        let this = new_debug_text(&mut e);
        e.register(VSNPRINTF, |e, a| {
            e.mem.set_cstr(a[0], b"hello 7");
            ret(7)
        });
        (e, this)
    }

    /// What the first queued item holds: text, x, y, justify, line number,
    /// fDie and font.
    type QueuedItem = (String, f32, f32, i32, i32, f32, i32);

    fn first_queued(e: &mut Engine, this: Ptr<DebugText>) -> QueuedItem {
        let queue = this.at(DebugText::xPrintData);
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer));
        let data = e.get(print_data, SPrintData::pTextData);
        (
            text_at(e, data.at(DebugTextData::pText).addr()).unwrap(),
            e.get(data, DebugTextData::fX),
            e.get(data, DebugTextData::fY),
            e.get(data, DebugTextData::iJustify),
            e.get(print_data, SPrintData::iLineNumber),
            e.get(print_data, SPrintData::fDie),
            e.get(print_data, SPrintData::iFont),
        )
    }

    #[test]
    fn va_print_x_formats_and_prints_at_the_column_height() {
        let (mut e, this) = formatter();
        e.mem.set_i32(height_address(this, 1), 40);
        let colors = e.mem.alloc(16);
        e.mem.set_f32(colors + 8, 0.75);
        e.call_log = Some(vec![]);
        let result = debug_text_va_print_x(&mut e, this, 100, 1, 3, FORMAT, 0x1234, colors);
        let log = take_log(&mut e);
        assert_eq!(result, 0);
        let formatted = calls_to(&log, VSNPRINTF);
        assert_eq!(formatted.len(), 1);
        assert_eq!(formatted[0][1..], [200, FORMAT, 0x1234]);
        assert_eq!(
            first_queued(&mut e, this),
            ("hello 7".to_string(), 100.0, 40.0, 3, -1, -1.0, 0)
        );
        // The color pointer is passed on: its words are copied.
        let queue = this.at(DebugText::xPrintData);
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer));
        let data = e.get(print_data, SPrintData::pTextData);
        assert_eq!(e.get(data.at(DebugTextData::Color), NiColorA::b), 0.75);
    }

    #[test]
    fn va_print_line_prints_then_moves_the_column_down() {
        let (mut e, this) = formatter();
        e.set(this, DebugText::iFontHeight, 18);
        e.mem.set_i32(x_location_address(this, 2), 300);
        e.mem.set_i32(height_address(this, 2), 50);
        let result = debug_text_va_print_line(&mut e, this, 2, 1, FORMAT, 0x1234, 0);
        assert_eq!(result, 0);
        assert_eq!(
            first_queued(&mut e, this),
            ("hello 7".to_string(), 300.0, 50.0, 1, -1, -1.0, 0)
        );
        assert_eq!(debug_text_get_current_line(&mut e, this, 2), 68);
    }

    // ---- queueing ----------------------------------------------------------

    #[test]
    fn print_queues_a_new_item_with_the_default_color() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        let text = put_text(&mut e, "abc");
        let result = debug_text_print(&mut e, this, text, 10.0, 20.0, 2, 3, 4.5, 6, Ptr::NULL);
        assert_eq!(result, 0);
        let queue = this.at(DebugText::xPrintData);
        assert_eq!(e.get(queue, BSSimpleArray::iSize), 1);
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer));
        assert_eq!(e.get(print_data, SPrintData::eType), 1);
        assert_eq!(e.get(print_data, SPrintData::iLineNumber), 3);
        assert_eq!(e.get(print_data, SPrintData::fDie), 4.5);
        assert_eq!(e.get(print_data, SPrintData::iFont), 6);
        let data = e.get(print_data, SPrintData::pTextData);
        assert_eq!(
            text_at(&e, data.at(DebugTextData::pText).addr()).unwrap(),
            "abc"
        );
        assert_eq!(e.get(data, DebugTextData::fX), 10.0);
        assert_eq!(e.get(data, DebugTextData::fY), 20.0);
        assert_eq!(e.get(data, DebugTextData::iJustify), 2);
        // 0xff804000: r = 255/255, g = 128/255, b = 64/255, a = 0.
        let color = data.at(DebugTextData::Color);
        assert_eq!(e.get(color, NiColorA::r), 1.0);
        assert_eq!(e.get(color, NiColorA::g), 128.0 / 255.0);
        assert_eq!(e.get(color, NiColorA::b), 64.0 / 255.0);
        assert_eq!(e.get(color, NiColorA::a), 0.0);
        assert_eq!(e.get(data, DebugTextData::fDie), -1.0);
    }

    #[test]
    fn print_replaces_the_item_queued_at_the_same_position() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        let first = queue(&mut e, this, 1, "old", 10.0, 20.0, 2);
        let colors = e.mem.alloc(16);
        for (i, lane) in [0.25f32, 0.5, 0.75, 1.0].iter().enumerate() {
            e.mem.set_f32(colors + 4 * i as u32, *lane);
        }
        let text = put_text(&mut e, "new");
        debug_text_print(
            &mut e,
            this,
            text,
            10.0,
            20.0,
            2,
            5,
            9.0,
            7,
            Ptr::new(colors),
        );
        let queue = this.at(DebugText::xPrintData);
        assert_eq!(e.get(queue, BSSimpleArray::iSize), 1, "nothing was added");
        let data = e.get(first, SPrintData::pTextData);
        assert_eq!(
            text_at(&e, data.at(DebugTextData::pText).addr()).unwrap(),
            "new"
        );
        assert_eq!(e.get(first, SPrintData::iLineNumber), 5);
        assert_eq!(e.get(first, SPrintData::fDie), 9.0);
        assert_eq!(e.get(first, SPrintData::iFont), 7);
        assert_eq!(e.get(data.at(DebugTextData::Color), NiColorA::g), 0.5);
        // Another position adds a second item.
        debug_text_print(&mut e, this, text, 11.0, 20.0, 2, 5, 9.0, 7, Ptr::NULL);
        assert_eq!(e.get(queue, BSSimpleArray::iSize), 2);
    }

    #[test]
    fn index_queueing_updates_or_adds_the_item_with_that_index() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        let text = put_text(&mut e, "first");
        assert_eq!(fn_00a0f730(&mut e, this, text, 4), 4);
        let queue = this.at(DebugText::xPrintData);
        assert_eq!(e.get(queue, BSSimpleArray::iSize), 1);
        let buffer = e.get(queue, BSSimpleArray::pBuffer);
        let print_data = Ptr::<SPrintData>::new(e.mem.u32(buffer));
        assert_eq!(e.get(print_data, SPrintData::eType), 0);
        assert_eq!(e.get(print_data, SPrintData::iIndex), 4);
        let data = e.get(print_data, SPrintData::pTextData);
        assert_eq!(
            text_at(&e, data.at(DebugTextData::pText).addr()).unwrap(),
            "first"
        );
        // The same index again only changes the text (and the type).
        e.set(print_data, SPrintData::eType, 1);
        let again = put_text(&mut e, "second");
        assert_eq!(fn_00a0f730(&mut e, this, again, 4), 4);
        assert_eq!(e.get(queue, BSSimpleArray::iSize), 1);
        assert_eq!(e.get(print_data, SPrintData::eType), 0);
        assert_eq!(
            text_at(&e, data.at(DebugTextData::pText).addr()).unwrap(),
            "second"
        );
        // Another index adds an item.
        assert_eq!(fn_00a0f730(&mut e, this, again, 5), 5);
        assert_eq!(e.get(queue, BSSimpleArray::iSize), 2);
    }

    // ---- HandleQueuedPrints -------------------------------------------------

    /// `builder_world()` plus the doubles the drain needs and a debug text.
    fn drain_world() -> (Engine, Ptr<DebugText>, MadeStrings) {
        let (mut e, made) = builder_world();
        let this = new_debug_text(&mut e);
        e.register(LINE_BREAK_TEXT, |_, _| ret(0));
        e.register(STRING_COMPARE, |e, a| {
            let shown = e.mem.u32(a[0]);
            ret((e.mem.cstr(shown) != e.mem.cstr(a[1])) as u32)
        });
        e.register(REPORT_ERROR, |_, _| ret(0));
        (e, this, made)
    }

    /// The texts `MakeString` was asked to build.
    fn made_texts(made: &MadeStrings) -> Vec<String> {
        made.borrow().iter().map(|(_, text)| text.clone()).collect()
    }

    #[test]
    fn a_queued_print_fills_the_first_free_entry() {
        let (mut e, this, made) = drain_world();
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 9);
        let print_data = queue(&mut e, this, 1, "abc", 10.0, 20.0, 1);
        let data = e.get(print_data, SPrintData::pTextData);
        e.set(data.at(DebugTextData::Color), NiColorA::g, 0.5);
        e.call_log = Some(vec![]);
        debug_text_handle_queued_prints(&mut e, this);
        let log = take_log(&mut e);
        let entry = entry_at(this, 0);
        // The node was built from the text at the converted position.
        assert_eq!(made_texts(&made), vec!["abc".to_string()]);
        let node = e.get(entry, DebugTextData::spNode);
        assert!(!node.is_null());
        assert_eq!(e.mem.f32(node.addr() + 0x58), 10.0 - 640.0);
        assert_eq!(e.mem.f32(node.addr() + 0x60), 1080.0 - 20.0 - 540.0);
        // Font 1 (the setting), alignment 1, the item's color.
        let fonts: u32 = e.global(FONT_MANAGER);
        assert_eq!(made.borrow()[0].0[0], e.mem.u32(fonts));
        assert_eq!(made.borrow()[0].0[6], 1);
        assert_eq!(e.mem.f32(made.borrow()[0].0[7] + 4), 0.5);
        // The entry shows the item.
        assert_eq!(e.get(entry, DebugTextData::fX), 10.0);
        assert_eq!(e.get(entry, DebugTextData::fY), 20.0);
        assert_eq!(e.get(entry, DebugTextData::iJustify), 1);
        assert_eq!(e.get(entry, DebugTextData::fDie), 2.5);
        assert_eq!(
            text_at(&e, entry.at(DebugTextData::pText).addr()).unwrap(),
            "abc"
        );
        assert_eq!(e.get(entry.at(DebugTextData::Color), NiColorA::g), 0.5);
        assert_eq!(calls_to(&log, SET_NAME).len(), 1);
        assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT).len(), 1);
        // The queue was emptied and its items destroyed.
        assert_eq!(
            calls_to(&log, ARRAY_CLEAR),
            vec![vec![this.addr() + 0x2270, 1]]
        );
        assert_eq!(e.mem.block_size(print_data.addr()), None);
        assert_eq!(e.mem.block_size(data.addr()), None);
        assert_eq!(e.mem.u32(tls + 0x2b4), 9);
    }

    #[test]
    fn a_queued_print_with_the_same_text_and_color_keeps_the_node() {
        let (mut e, this, made) = drain_world();
        let shown = show(&mut e, this, 0, "abc", 10.0, 20.0, 1);
        queue(&mut e, this, 1, "abc", 10.0, 20.0, 1);
        e.call_log = Some(vec![]);
        debug_text_handle_queued_prints(&mut e, this);
        let log = take_log(&mut e);
        assert!(made.borrow().is_empty());
        assert!(calls_to(&log, REMOVE).is_empty());
        assert_eq!(
            e.get(entry_at(this, 0), DebugTextData::spNode).addr(),
            shown
        );
    }

    #[test]
    fn a_queued_print_with_other_text_replaces_the_node() {
        let (mut e, this, made) = drain_world();
        let shown = show(&mut e, this, 2, "old", 10.0, 20.0, 1);
        queue(&mut e, this, 1, "new", 10.0, 20.0, 1);
        e.call_log = Some(vec![]);
        debug_text_handle_queued_prints(&mut e, this);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, REMOVE), vec![vec![ROOT, shown]]);
        assert_eq!(made_texts(&made), vec!["new".to_string()]);
        let entry = entry_at(this, 2);
        let node = e.get(entry, DebugTextData::spNode);
        assert!(!node.is_null());
        assert_ne!(node.addr(), shown);
        assert_eq!(
            text_at(&e, entry.at(DebugTextData::pText).addr()).unwrap(),
            "new"
        );
    }

    #[test]
    fn a_queued_print_keeps_only_the_wanted_line() {
        let (mut e, this, made) = drain_world();
        let print_data = queue(&mut e, this, 1, "one\ntwo\nthree", 10.0, 20.0, 1);
        e.set(print_data, SPrintData::iLineNumber, 2);
        debug_text_handle_queued_prints(&mut e, this);
        assert_eq!(made_texts(&made), vec!["two".to_string()]);
    }

    #[test]
    fn a_queued_print_with_too_few_lines_ends_the_drain() {
        let (mut e, this, made) = drain_world();
        let print_data = queue(&mut e, this, 1, "one\ntwo", 10.0, 20.0, 1);
        e.set(print_data, SPrintData::iLineNumber, 3);
        queue(&mut e, this, 1, "next", 11.0, 20.0, 1);
        e.call_log = Some(vec![]);
        debug_text_handle_queued_prints(&mut e, this);
        let log = take_log(&mut e);
        assert!(made.borrow().is_empty());
        assert!(calls_to(&log, ARRAY_CLEAR).is_empty(), "the queue stays");
        assert_eq!(
            e.get(this.at(DebugText::xPrintData), BSSimpleArray::iSize),
            2
        );
    }

    #[test]
    fn a_queued_empty_print_leaves_its_entry_free() {
        let (mut e, this, made) = drain_world();
        queue(&mut e, this, 1, "", 10.0, 20.0, 1);
        debug_text_handle_queued_prints(&mut e, this);
        assert!(made.borrow().is_empty());
        let entry = entry_at(this, 0);
        assert!(e.get(entry, DebugTextData::spNode).is_null());
        assert_eq!(e.get(entry, DebugTextData::iJustify), 0);
        assert_eq!(e.get(entry, DebugTextData::fDie), -1.0);
    }

    #[test]
    fn a_full_table_reports_the_error_and_ends_the_drain() {
        let (mut e, this, made) = drain_world();
        for index in 0..200 {
            e.set(entry_at(this, index), DebugTextData::iJustify, 1);
            e.set(
                entry_at(this, index),
                DebugTextData::fX,
                index as f32 + 100.0,
            );
        }
        queue(&mut e, this, 1, "abc", 10.0, 20.0, 1);
        e.call_log = Some(vec![]);
        debug_text_handle_queued_prints(&mut e, this);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, REPORT_ERROR),
            vec![vec![TOO_MANY_ITEMS_TEXT]]
        );
        assert!(made.borrow().is_empty());
        assert!(calls_to(&log, ARRAY_CLEAR).is_empty());
    }

    #[test]
    fn a_queued_print_needs_the_debug_text_root() {
        let (mut e, this, made) = drain_world();
        e.register(GET_DEBUG_TEXT_ROOT, |_, _| ret(0));
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 9);
        queue(&mut e, this, 1, "abc", 10.0, 20.0, 1);
        e.call_log = Some(vec![]);
        debug_text_handle_queued_prints(&mut e, this);
        let log = take_log(&mut e);
        assert!(made.borrow().is_empty());
        assert!(calls_to(&log, ARRAY_CLEAR).is_empty());
        assert_eq!(e.mem.u32(tls + 0x2b4), 9);
    }

    #[test]
    fn a_queued_text_update_rebuilds_the_indexed_entry() {
        let (mut e, this, made) = drain_world();
        let shown = show(&mut e, this, 4, "old", 12.0, 14.0, 2);
        let print_data = queue(&mut e, this, 0, "fresh", 0.0, 0.0, 0);
        e.set(print_data, SPrintData::iIndex, 4);
        e.call_log = Some(vec![]);
        debug_text_handle_queued_prints(&mut e, this);
        let log = take_log(&mut e);
        // The old node left the root, the entry was cleared and the text
        // created again at the cleared position (0, 0).
        assert_eq!(calls_to(&log, REMOVE), vec![vec![ROOT, shown]]);
        assert_eq!(made_texts(&made), vec!["fresh".to_string()]);
        let node = e.get(entry_at(this, 4), DebugTextData::spNode);
        assert!(!node.is_null());
        assert_ne!(node.addr(), shown);
        assert_eq!(e.mem.f32(node.addr() + 0x58), -640.0);
        assert_eq!(e.mem.f32(node.addr() + 0x60), 540.0);
        assert_eq!(calls_to(&log, ARRAY_CLEAR).len(), 1);
    }

    // ---- CreateDebugText and the lookups -----------------------------------

    /// `world()` plus doubles for the node, font and property callees the node
    /// builders use. `MakeString` returns a fresh 0xA0-byte geometry and
    /// records its arguments.
    #[allow(clippy::type_complexity)]
    fn builder_world() -> (Engine, MadeStrings) {
        let mut e = world();
        e.register(IS_CONSOLE_VISIBLE, |_, _| ret(1));
        e.register(GET_SCREEN_HEIGHT, |_, _| Ret {
            st0: 1080.0,
            ..Ret::default()
        });
        e.register(NODE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], NODE_VTABLE);
            e.mem.set_u32(a[0] + 0x30, 3);
            ret(a[0])
        });
        e.register(FLOATING_NODE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], NODE_VTABLE);
            ret(a[0])
        });
        e.register(STRING_FROM_TEXT, |e, a| {
            string_set_double(e, &[a[0], a[1], 0]);
            ret(a[0])
        });
        e.register(GET_PROPERTY, |e, _| ret(e.mem.alloc(0x80)));
        e.register(PROPERTY_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(PROPERTY_SETTER, |_, _| ret(0));
        e.register(ATTACH_PROPERTY, |_, _| ret(0));
        e.register(UPDATE_PROPERTIES, |_, _| ret(0));
        e.register(UPDATE, |_, _| ret(0));
        e.register(UPDATE_DATA_CONSTRUCTOR, |_, _| ret(0));
        e.register(SET_NAME, |_, _| ret(0));
        e.register(FIXED_STRING_CREATE, |e, _| ret(e.mem.alloc(16) + 8));
        e.register(FIXED_STRING_RELEASE, |_, _| ret(0));
        e.register(PREPARE_OBJECT, |_, _| ret(0));
        e.register(ADD_EXTRA_DATA, |_, _| ret(0));
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(FABS, |_, a| Ret {
            st0: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32).abs(),
            ..Ret::default()
        });
        // `out = this`, so the product is the left matrix.
        e.register(MATRIX_MULTIPLY, |e, a| {
            for i in 0..9 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            ret(a[1])
        });
        let made = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&made);
        e.register_double(MAKE_STRING, move |e, a| {
            let text = e.mem.u32(a[4]);
            let text = String::from_utf8(e.mem.cstr(text)).unwrap();
            sink.borrow_mut().push((a.to_vec(), text));
            ret(e.mem.alloc(0xa0))
        });
        (e, made)
    }

    fn matrix_at(e: &Engine, address: u32) -> [f32; 9] {
        let mut matrix = [0.0; 9];
        for (i, lane) in matrix.iter_mut().enumerate() {
            *lane = e.mem.f32(address + 4 * i as u32);
        }
        matrix
    }

    fn assert_close(actual: &[f32], expected: &[f32]) {
        assert_eq!(actual.len(), expected.len());
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-5, "{actual:?} != {expected:?}");
        }
    }

    #[test]
    fn create_debug_text_builds_and_attaches_a_node() {
        let (mut e, made) = builder_world();
        let this = new_debug_text(&mut e);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 5);
        let text = put_text(&mut e, "hi");
        e.call_log = Some(vec![]);
        let node = debug_text_create_debug_text(&mut e, this, text, 700.0, 100.0, 3, 0, Ptr::NULL);
        let log = take_log(&mut e);
        assert!(!node.is_null());
        // x = (700 - 640) * 1; z stays; y = (1080 - 100) - 1080 / 2.
        assert_eq!(e.mem.f32(node.addr() + 0x58), 60.0);
        assert_eq!(e.mem.f32(node.addr() + 0x5c), -50.0);
        assert_eq!(e.mem.f32(node.addr() + 0x60), 440.0);
        // Bit 0 of the flags is cleared.
        assert_eq!(e.mem.u32(node.addr() + 0x30), 2);
        // Font 1 (the setting), alignment 4 for justify 3, 0xff804000 as
        // the default color.
        let made = made.borrow();
        assert_eq!(made.len(), 1);
        let fonts: u32 = e.global(FONT_MANAGER);
        assert_eq!(made[0].0[0], e.mem.u32(fonts));
        assert_eq!(made[0].0[6], 4);
        assert_eq!(made[0].0[8..], [1, 1]);
        let color = made[0].0[7];
        assert_eq!(e.mem.f32(color + 4), 128.0 / 255.0);
        let geometry_attach = calls_to(&log, ATTACH);
        assert_eq!(geometry_attach.len(), 2);
        assert_eq!(geometry_attach[0][0], node.addr());
        assert_eq!(geometry_attach[1], vec![ROOT, node.addr(), 1]);
        assert_eq!(e.mem.u32(tls + 0x2b4), 5);
    }

    #[test]
    fn create_debug_text_uses_the_given_color_font_and_wide_scale() {
        let (mut e, made) = builder_world();
        let this = new_debug_text(&mut e);
        e.mem.set_u8(WIDE_FLAG, 1);
        let color = e.mem.alloc(16);
        for (i, lane) in [0.1f32, 0.2, 0.3, 0.4].iter().enumerate() {
            e.mem.set_f32(color + 4 * i as u32, *lane);
        }
        let text = put_text(&mut e, "hi");
        let node =
            debug_text_create_debug_text(&mut e, this, text, 700.0, 100.0, 2, 2, Ptr::new(color));
        let x = e.mem.f32(node.addr() + 0x58);
        assert!((x - 60.0 * 1.33333).abs() < 1e-3, "{x}");
        let made = made.borrow();
        let fonts: u32 = e.global(FONT_MANAGER);
        assert_eq!(made[0].0[0], e.mem.u32(fonts + 4));
        assert_eq!(made[0].0[6], 2);
        assert_eq!(e.mem.f32(made[0].0[7] + 8), 0.3);
    }

    #[test]
    fn create_debug_text_returns_null_for_empty_text_or_nothing_visible() {
        let (mut e, made) = builder_world();
        let this = new_debug_text(&mut e);
        let empty = put_text(&mut e, "");
        assert!(
            debug_text_create_debug_text(&mut e, this, empty, 1.0, 2.0, 1, 0, Ptr::NULL).is_null()
        );
        e.register(IS_CONSOLE_VISIBLE, |_, _| ret(0));
        e.register(IS_DEBUG_TEXT_VISIBLE, |_, _| ret(0));
        let text = put_text(&mut e, "hi");
        assert!(
            debug_text_create_debug_text(&mut e, this, text, 1.0, 2.0, 1, 0, Ptr::NULL).is_null()
        );
        assert!(made.borrow().is_empty());
        // Either one visible is enough.
        e.register(IS_DEBUG_TEXT_VISIBLE, |_, _| ret(1));
        assert!(
            !debug_text_create_debug_text(&mut e, this, text, 1.0, 2.0, 1, 0, Ptr::NULL).is_null()
        );
    }

    #[test]
    fn text_lookup_finds_queued_items_before_entries() {
        let mut e = world();
        e.register(STRNCPY, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            ret(a[0])
        });
        let this = new_debug_text(&mut e);
        queue(&mut e, this, 1, "queued", 10.0, 20.0, 1);
        show(&mut e, this, 7, "shown", 10.0, 20.0, 1);
        show(&mut e, this, 8, "other", 30.0, 40.0, 2);
        let out = e.mem.alloc(0x800);
        e.mem.set_cstr(out, b"junk");
        assert!(fn_00a103e0(&mut e, this, out, 10.0, 20.0, 1));
        assert_eq!(e.mem.cstr(out), b"queued");
        assert!(fn_00a103e0(&mut e, this, out, 30.0, 40.0, 2));
        assert_eq!(e.mem.cstr(out), b"other");
        // No match: false, and the buffer starts with a NUL.
        assert!(!fn_00a103e0(&mut e, this, out, 1.0, 1.0, 1));
        assert_eq!(e.mem.cstr(out), b"");
        // A null buffer only answers whether the item exists... after the
        // first byte write, as in the exe, so give it a real address.
    }

    #[test]
    fn text_pointer_lookup_returns_the_text_of_the_match() {
        let mut e = world();
        let this = new_debug_text(&mut e);
        let queued = queue(&mut e, this, 1, "queued", 10.0, 20.0, 1);
        show(&mut e, this, 199, "last", 30.0, 40.0, 2);
        let data = e.get(queued, SPrintData::pTextData);
        let queued_text = e.get(data.at(DebugTextData::pText), BSStringT::pString);
        assert_eq!(fn_00a10630(&mut e, this, 10.0, 20.0, 1), queued_text);
        let last = e.get(
            entry_at(this, 199).at(DebugTextData::pText),
            BSStringT::pString,
        );
        assert_eq!(fn_00a10630(&mut e, this, 30.0, 40.0, 2), last);
        assert_eq!(fn_00a10630(&mut e, this, 30.0, 40.0, 3), 0);
    }

    // ---- the floating text builders ---------------------------------------

    #[test]
    fn fn_00a10770_builds_the_rotated_scaled_node() {
        let (mut e, made) = builder_world();
        let text = put_text(&mut e, "float");
        let color = 0x0800_6000u32;
        e.call_log = Some(vec![]);
        let node = e
            .call(
                0x00a10770,
                &args![text, 1.5f32, 2.5f32, -3.0f32, 1u32, 2u32, color, 1u8],
            )
            .ptr::<()>();
        let log = take_log(&mut e);
        assert!(!node.is_null());
        let made = made.borrow();
        let fonts: u32 = e.global(FONT_MANAGER);
        assert_eq!(made[0].0[0], e.mem.u32(fonts + 4), "font number 1, from 0");
        assert_eq!(f32::from_bits(made[0].0[1]), 1.5);
        assert_eq!(f32::from_bits(made[0].0[2]), 2.5);
        assert_eq!(e.mem.u32(made[0].0[5]), 10000);
        assert_eq!(made[0].0[6..], [2, color, 0, 1]);
        // The geometry is attached and rotated about the first axis by the
        // first angle; the node by the second; its scale is |height|.
        let attached = calls_to(&log, ATTACH);
        let geometry = attached[0][1];
        assert_eq!(attached[0], vec![node.addr(), geometry, 1]);
        assert_close(
            &matrix_at(&e, geometry + 0x34),
            &rotation_about_first_axis(std::f32::consts::FRAC_PI_2),
        );
        assert_close(
            &matrix_at(&e, node.addr() + 0x34),
            &rotation_about_first_axis(-std::f32::consts::FRAC_PI_2),
        );
        assert_eq!(e.mem.f32(node.addr() + 0x64), 3.0);
        assert_eq!(calls_to(&log, PROPERTY_SETTER).len(), 2);
        assert_eq!(calls_to(&log, ATTACH_PROPERTY).len(), 1);
        assert_eq!(calls_to(&log, FIXED_STRING_RELEASE).len(), 1);
    }

    #[test]
    fn fn_00a10770_returns_null_when_nothing_is_made() {
        let (mut e, _made) = builder_world();
        e.register(MAKE_STRING, |_, _| ret(0));
        let text = put_text(&mut e, "float");
        let node = e.call(
            0x00a10770,
            &args![text, 1.5f32, 2.5f32, -3.0f32, 0u32, 2u32, 0u32, 0u8],
        );
        assert_eq!(node.u32(), 0);
    }

    /// The extra data of a node: an array of pointers at `+0x10`, count at
    /// `+0x14`.
    fn set_extras(e: &mut Engine, node: u32, objects: &[u32]) {
        let array = e.mem.alloc(4 * objects.len().max(1) as u32);
        for (i, object) in objects.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *object);
        }
        e.mem.set_u32(node + 0x10, array);
        e.mem.set_u16(node + 0x14, objects.len() as u16);
    }

    fn new_extra(e: &mut Engine, height: i32, old_center: i32, billboard: u32) -> u32 {
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra, EXTRA_CLASS_VTABLE);
        e.mem.set_u32(extra + 0xc, billboard);
        e.mem.set_i32(extra + 0x18, height);
        e.mem.set_i32(extra + 0x1c, old_center);
        extra
    }

    fn new_node(e: &mut Engine) -> u32 {
        let node = e.mem.alloc(0xb4);
        e.mem.set_u32(node, NODE_VTABLE);
        node
    }

    #[test]
    fn extra_data_search_finds_the_height_and_reports_the_old_center() {
        let mut e = world();
        let node = new_node(&mut e);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other, OTHER_VTABLE);
        let first = new_extra(&mut e, 5, 111, 0);
        let second = new_extra(&mut e, 7, 222, 0);
        set_extras(&mut e, node, &[other, first, second]);
        let out = e.mem.alloc(4);
        let found = fn_00a11740(&mut e, Ptr::new(node), 0, 7, Ptr::new(out));
        assert_eq!(found.addr(), second);
        assert_eq!(e.mem.i32(out), 222);
        // No match: null, and the last examined old center stays.
        let found = fn_00a11740(&mut e, Ptr::new(node), 0, 9, Ptr::new(out));
        assert!(found.is_null());
        assert_eq!(e.mem.i32(out), 222);
        // A null node, and a node without extra data.
        assert!(fn_00a11740(&mut e, Ptr::NULL, 0, 7, Ptr::new(out)).is_null());
        let bare = new_node(&mut e);
        assert!(fn_00a11740(&mut e, Ptr::new(bare), 0, 7, Ptr::new(out)).is_null());
    }

    /// An engine with an instance, the builder doubles and a node `node`
    /// that has no extra data; the world bound's `+8` is 12.0.
    fn floating_world() -> (Engine, Ptr<DebugText>, u32, MadeStrings) {
        let (mut e, made) = builder_world();
        e.map(INSTANCE & !0xfff, 0x1000);
        e.mem.set_f32(DEFAULT_BOUND + 8, 12.0);
        let this = debug_text_instance(&mut e, 1);
        let node = new_node(&mut e);
        (e, this, node, made)
    }

    #[test]
    fn floating_text_on_a_new_node_adds_extra_data_billboard_and_element() {
        let (mut e, this, node, made) = floating_world();
        let text = put_text(&mut e, "msg");
        e.call_log = Some(vec![]);
        assert!(fn_00a10c00(&mut e, Ptr::new(node), text, 30, 4.5));
        let log = take_log(&mut e);
        let attached = calls_to(&log, ATTACH);
        // geometry on the billboard, then the billboard on the node.
        let billboard = attached[1][1];
        assert_eq!(attached[0][0], billboard);
        assert_eq!(attached[1], vec![node, billboard, 1]);
        // 12 (bound) + 30 above the node.
        assert_eq!(e.mem.f32(billboard + 0x60), 42.0);
        assert_eq!(e.mem.f32(billboard + 0x64), 0.5);
        let extra_adds = calls_to(&log, ADD_EXTRA_DATA);
        assert_eq!(extra_adds.len(), 1);
        let extra = extra_adds[0][1];
        assert_eq!(e.mem.i32(extra + 0x18), 30);
        assert_eq!(e.mem.i32(extra + 0x1c), 12);
        assert_eq!(e.mem.u32(extra + 0xc), billboard);
        assert_eq!(text_at(&e, extra + 0x10).unwrap(), "msg");
        assert_eq!(e.mem.u32(extra), EXTRA_DATA_VTABLE);
        // One element in the list, carrying the billboard and the time.
        let list = this.at(DebugText::xFloatingTextList);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 1);
        let node_of_list = e.get(list, NiTPointerList::m_pkHead);
        let element = Ptr::<DebugTextData>::new(e.mem.u32(node_of_list + 8));
        assert_eq!(e.get(element, DebugTextData::fDie), 4.5);
        assert_eq!(e.get(element, DebugTextData::spNode).addr(), billboard);
        assert_eq!(e.get(element, DebugTextData::fX), 42.0);
        assert_eq!(
            text_at(&e, element.at(DebugTextData::pText).addr()).unwrap(),
            "msg"
        );
        // Font number 2 (offset 4), alignment 2, the fixed color.
        let made = made.borrow();
        let fonts: u32 = e.global(FONT_MANAGER);
        assert_eq!(made[0].0[0], e.mem.u32(fonts + 4));
        assert_eq!(made[0].0[6..], [2, FLOATING_TEXT_COLOR, 0, 1]);
    }

    #[test]
    fn floating_text_with_extra_data_in_the_list_only_updates_the_time() {
        let (mut e, this, node, _made) = floating_world();
        let billboard = new_node(&mut e);
        let extra = new_extra(&mut e, 30, 12, billboard);
        set_extras(&mut e, node, &[extra]);
        let element = Ptr::<DebugTextData>::new(e.mem.alloc(0x2c));
        e.set(element, DebugTextData::spNode, Ptr::new(billboard));
        let list_node = e.mem.alloc(12);
        e.mem.set_u32(list_node + 8, element.addr());
        let list = this.at(DebugText::xFloatingTextList);
        e.set(list, NiTPointerList::m_pkHead, list_node);
        let text = put_text(&mut e, "msg");
        e.call_log = Some(vec![]);
        assert!(fn_00a10c00(&mut e, Ptr::new(node), text, 30, 7.5));
        let log = take_log(&mut e);
        assert_eq!(e.get(element, DebugTextData::fDie), 7.5);
        assert!(calls_to(&log, NEW_OBJECT).is_empty());
    }

    #[test]
    fn floating_text_with_extra_data_missing_from_the_list_is_rebuilt() {
        let (mut e, this, node, _made) = floating_world();
        let old_billboard = new_node(&mut e);
        let extra = new_extra(&mut e, 30, 12, old_billboard);
        set_extras(&mut e, node, &[extra]);
        let text = put_text(&mut e, "msg");
        e.call_log = Some(vec![]);
        assert!(fn_00a10c00(&mut e, Ptr::new(node), text, 30, 7.5));
        let log = take_log(&mut e);
        // The old billboard leaves the node, the extra data is reused (not
        // added again) and now points at the new billboard.
        assert_eq!(calls_to(&log, REMOVE), vec![vec![node, old_billboard]]);
        assert!(calls_to(&log, ADD_EXTRA_DATA).is_empty());
        assert_ne!(e.mem.u32(extra + 0xc), old_billboard);
        assert_eq!(
            e.get(
                this.at(DebugText::xFloatingTextList),
                NiTPointerList::m_uiCount
            ),
            1
        );
    }

    #[test]
    fn floating_text_needs_a_node_and_a_string_geometry() {
        let (mut e, _this, node, _made) = floating_world();
        let text = put_text(&mut e, "msg");
        assert!(!fn_00a10c00(&mut e, Ptr::NULL, text, 30, 1.0));
        e.register(MAKE_STRING, |_, _| ret(0));
        assert!(!fn_00a10c00(&mut e, Ptr::new(node), text, 30, 1.0));
    }

    #[test]
    fn floating_text_goes_to_a_node_or_its_parent() {
        let (mut e, this, node, _made) = floating_world();
        let text = put_text(&mut e, "msg");
        let list = this.at(DebugText::xFloatingTextList);
        // A node itself: the floating text is built on it.
        e.call_log = Some(vec![]);
        assert!(fn_00a11470(&mut e, Ptr::new(node), text, 3, 2.0));
        let log = take_log(&mut e);
        assert!(calls_to(&log, ATTACH).iter().any(|call| call[0] == node));
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 1);
        // Not a node, but its parent is one.
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other, OTHER_VTABLE);
        e.mem.set_u32(other + 0x18, node);
        e.call_log = Some(vec![]);
        assert!(fn_00a11470(&mut e, Ptr::new(other), text, 4, 2.0));
        let log = take_log(&mut e);
        assert!(calls_to(&log, ATTACH).iter().any(|call| call[0] == node));
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 2);
        // Neither: false and nothing is built.
        let orphan = e.mem.alloc(0x40);
        e.mem.set_u32(orphan, OTHER_VTABLE);
        e.call_log = Some(vec![]);
        assert!(!fn_00a11470(&mut e, Ptr::new(orphan), text, 3, 2.0));
        assert!(!fn_00a11470(&mut e, Ptr::NULL, text, 3, 2.0));
        assert!(calls_to(&take_log(&mut e), ATTACH).is_empty());
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 2);
    }

    #[test]
    fn removal_goes_to_a_node_or_its_parent() {
        let mut e = world();
        let node = new_node(&mut e);
        let first = new_extra(&mut e, 7, 0, 0xb1);
        set_extras(&mut e, node, &[first]);
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other, OTHER_VTABLE);
        e.mem.set_u32(other + 0x18, node);
        let orphan = e.mem.alloc(0x40);
        e.mem.set_u32(orphan, OTHER_VTABLE);
        // A node itself, every height.
        e.call_log = Some(vec![]);
        fn_00a11560(&mut e, Ptr::new(node), -1);
        assert_eq!(calls_to(&take_log(&mut e), REMOVE), vec![vec![node, 0xb1]]);
        // Its child object: the parent node handles the height.
        e.call_log = Some(vec![]);
        fn_00a11560(&mut e, Ptr::new(other), 7);
        assert_eq!(calls_to(&take_log(&mut e), REMOVE), vec![vec![node, 0xb1]]);
        // Neither: nothing.
        e.call_log = Some(vec![]);
        fn_00a11560(&mut e, Ptr::new(orphan), 7);
        fn_00a11560(&mut e, Ptr::NULL, 7);
        assert!(calls_to(&take_log(&mut e), REMOVE).is_empty());
    }

    #[test]
    fn extra_data_removal_takes_billboards_out_of_the_node() {
        let mut e = world();
        let node = new_node(&mut e);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other, OTHER_VTABLE);
        let first = new_extra(&mut e, 5, 0, 0xb1);
        let second = new_extra(&mut e, 7, 0, 0xb2);
        let third = new_extra(&mut e, 7, 0, 0xb3);
        set_extras(&mut e, node, &[other, first, second, third]);
        // Negative: every one of them.
        e.call_log = Some(vec![]);
        fn_00a11640(&mut e, Ptr::new(node), -1);
        assert_eq!(
            calls_to(&take_log(&mut e), REMOVE),
            vec![vec![node, 0xb1], vec![node, 0xb2], vec![node, 0xb3]]
        );
        // A height: the first with that height, then stop.
        e.call_log = Some(vec![]);
        fn_00a11640(&mut e, Ptr::new(node), 7);
        assert_eq!(calls_to(&take_log(&mut e), REMOVE), vec![vec![node, 0xb2]]);
        // No match: nothing.
        e.call_log = Some(vec![]);
        fn_00a11640(&mut e, Ptr::new(node), 9);
        assert!(calls_to(&take_log(&mut e), REMOVE).is_empty());
    }

    // ---- locks, small constructors and containers -------------------------

    #[test]
    fn lock_and_unlock_follow_the_visibility_check() {
        let mut e = world();
        e.call_log = Some(vec![]);
        debug_text_lock(&mut e);
        debug_text_unlock(&mut e);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, ENTER_CRITICAL_SECTION),
            vec![vec![CRITICAL_SECTION]]
        );
        assert_eq!(
            calls_to(&log, LEAVE_CRITICAL_SECTION),
            vec![vec![CRITICAL_SECTION]]
        );
        e.register(IS_DEBUG_TEXT_VISIBLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        debug_text_lock(&mut e);
        debug_text_unlock(&mut e);
        assert_eq!(take_log(&mut e).len(), 2, "only the two visibility checks");
    }

    #[test]
    fn entry_constructor_clears_every_field() {
        let mut e = world();
        let entry = Ptr::<DebugTextData>::new(e.mem.alloc(0x2c));
        for word in 0..11u32 {
            e.mem.set_u32(entry.addr() + 4 * word, 0xdead_beef);
        }
        assert_eq!(fn_00a11880(&mut e, entry), entry);
        assert_eq!(e.get(entry, DebugTextData::fX), 0.0);
        assert_eq!(e.get(entry, DebugTextData::fY), 0.0);
        assert_eq!(e.get(entry, DebugTextData::iJustify), 0);
        assert!(e.get(entry, DebugTextData::spNode).is_null());
        assert_eq!(e.mem.u32(entry.addr() + 0x10), 0);
        assert_eq!(e.mem.u16(entry.addr() + 0x14), 0);
        assert_eq!(e.mem.u16(entry.addr() + 0x16), 0);
        assert_eq!(e.get(entry, DebugTextData::fDie), -1.0);
        for lane in 0..4 {
            assert_eq!(e.mem.u32(entry.addr() + 0x1c + 4 * lane), 0);
        }
    }

    #[test]
    fn entry_destructor_empties_the_text_and_releases_the_node() {
        let mut e = world();
        let entry = Ptr::<DebugTextData>::new(e.mem.alloc(0x2c));
        set_string(&mut e, entry.addr() + 0x10, "x");
        let node = new_node(&mut e);
        e.mem.set_u32(node + 4, 1);
        e.set(entry, DebugTextData::spNode, Ptr::new(node));
        e.call_log = Some(vec![]);
        fn_00a119d0(&mut e, entry);
        let log = take_log(&mut e);
        assert_eq!(text_at(&e, entry.addr() + 0x10), None);
        // The last reference: DeleteThis (slot 4) runs.
        assert_eq!(calls_to(&log, DELETE_THIS), vec![vec![node]]);
        // A node that is still referenced elsewhere stays.
        let shared = new_node(&mut e);
        e.mem.set_u32(shared + 4, 2);
        e.set(entry, DebugTextData::spNode, Ptr::new(shared));
        e.call_log = Some(vec![]);
        fn_00a119d0(&mut e, entry);
        assert!(calls_to(&take_log(&mut e), DELETE_THIS).is_empty());
        assert_eq!(e.mem.u32(shared + 4), 1);
    }

    #[test]
    fn extra_data_constructor_stores_billboard_text_height_and_center() {
        let mut e = world();
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, a| ret(a[0]));
        let extra = Ptr::<DebugTextExtraData>::new(e.mem.alloc(0x20));
        let text = put_text(&mut e, "label");
        e.call_log = Some(vec![]);
        let result = fn_00a11a80(&mut e, extra, Ptr::new(0xb111), text, 30, -4);
        assert_eq!(result, extra);
        assert_eq!(
            calls_to(&take_log(&mut e), EXTRA_DATA_CONSTRUCTOR),
            vec![vec![extra.addr()]]
        );
        assert_eq!(e.mem.u32(extra.addr()), EXTRA_DATA_VTABLE);
        assert_eq!(e.get(extra, DebugTextExtraData::pBillboard).addr(), 0xb111);
        assert_eq!(text_at(&e, extra.addr() + 0x10).unwrap(), "label");
        assert_eq!(e.get(extra, DebugTextExtraData::iHeight), 30);
        assert_eq!(e.get(extra, DebugTextExtraData::iOldCenter), -4);
    }

    #[test]
    fn extra_data_class_answers_its_rtti_and_deletes_itself() {
        let mut e = world();
        e.register(EXTRA_DATA_DESTRUCTOR, |_, _| ret(0));
        let extra = Ptr::<DebugTextExtraData>::new(e.mem.alloc(0x20));
        assert_eq!(e.call(0x00a11b70, &args![extra]).u32(), EXTRA_DATA_RTTI);
        set_string(&mut e, extra.addr() + 0x10, "label");
        e.call_log = Some(vec![]);
        let result = e.call(0x00a11b80, &args![extra, 0u32]);
        assert_eq!(result.u32(), extra.addr());
        let log = take_log(&mut e);
        assert_eq!(text_at(&e, extra.addr() + 0x10), None);
        assert_eq!(
            calls_to(&log, EXTRA_DATA_DESTRUCTOR),
            vec![vec![extra.addr()]]
        );
        assert!(calls_to(&log, DELETE_OBJECT).is_empty());
        // Bit 0 of the flags: the 0x20-byte block is deleted too.
        e.call_log = Some(vec![]);
        e.call(0x00a11b80, &args![extra, 1u32]);
        assert_eq!(
            calls_to(&take_log(&mut e), DELETE_OBJECT),
            vec![vec![extra.addr(), 0x20]]
        );
    }

    #[test]
    fn list_removal_hands_back_the_element_found_or_given() {
        let mut e = world();
        let list = Ptr::<NiTPointerList>::new(e.mem.alloc(12));
        let element_a = 0xa000u32;
        let element_b = 0xb000u32;
        let node_b = e.mem.alloc(12);
        e.mem.set_u32(node_b + 8, element_b);
        let node_a = e.mem.alloc(12);
        e.mem.set_u32(node_a, node_b);
        e.mem.set_u32(node_a + 8, element_a);
        e.set(list, NiTPointerList::m_pkHead, node_a);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, element_b);
        e.call_log = Some(vec![]);
        // Found: RemoveNode(list, &node) with the second node.
        let removed = fn_00a11c40(&mut e, list, Ptr::new(slot));
        let log = take_log(&mut e);
        assert_eq!(removed, element_b);
        let calls = calls_to(&log, LIST_REMOVE_NODE);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], list.addr());
        // Not in the list: the element itself comes back, nothing is removed.
        e.mem.set_u32(slot, 0xc000);
        e.call_log = Some(vec![]);
        assert_eq!(fn_00a11c40(&mut e, list, Ptr::new(slot)), 0xc000);
        assert!(calls_to(&take_log(&mut e), LIST_REMOVE_NODE).is_empty());
    }

    #[test]
    fn print_queue_array_constructor_destructor_and_append() {
        let mut e = world();
        let array = Ptr::<BSSimpleArray>::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        assert_eq!(fn_00a11cd0(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), PRINT_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&take_log(&mut e), ARRAY_CONSTRUCTOR),
            vec![vec![array.addr(), 0, 0]]
        );
        // Append: the growth helper's index is where the pointer goes.
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x1111);
        assert_eq!(fn_00a11d20(&mut e, array, Ptr::new(slot)), 0);
        e.mem.set_u32(slot, 0x2222);
        assert_eq!(fn_00a11d20(&mut e, array, Ptr::new(slot)), 1);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_eq!([e.mem.u32(buffer), e.mem.u32(buffer + 4)], [0x1111, 0x2222]);
        // Destructor: vtable and Clear(this, 1).
        e.mem.set_u32(array.addr(), 0);
        e.call_log = Some(vec![]);
        bs_simple_array_print_data_destructor(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), PRINT_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&take_log(&mut e), ARRAY_CLEAR),
            vec![vec![array.addr(), 1]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
    }

    #[test]
    fn print_queue_array_deleting_destructor_frees_only_with_the_flag() {
        let mut e = world();
        let array = Ptr::<BSSimpleArray>::new(e.mem.alloc(0x10));
        let result = bs_simple_array_print_data_scalar_deleting_destructor(&mut e, array, 0);
        assert_eq!(result, array);
        assert_eq!(e.mem.block_size(array.addr()), Some(0x10));
        let result = e.call(0x00a11d80, &args![array, 1u32]);
        assert_eq!(result.u32(), array.addr());
        assert_eq!(e.mem.block_size(array.addr()), None);
    }

    #[test]
    fn list_remove_all_frees_every_node() {
        let mut e = world();
        let list = Ptr::<NiTPointerList>::new(e.mem.alloc(12));
        let second = e.mem.alloc(12);
        let first = e.mem.alloc(12);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x1234);
        e.mem.set_u32(second + 8, 0x5678);
        e.set(list, NiTPointerList::m_pkHead, first);
        e.set(list, NiTPointerList::m_pkTail, second);
        e.set(list, NiTPointerList::m_uiCount, 2);
        nitpointerlist_remove_all(&mut e, list);
        assert_eq!(e.mem.block_size(first), None);
        assert_eq!(e.mem.block_size(second), None);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), 0);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 0);
        // An empty list stays empty.
        nitpointerlist_remove_all(&mut e, list);
    }
}
