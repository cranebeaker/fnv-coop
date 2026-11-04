//! `fallout shared/run_004d61b0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004d9190 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `004d91b0`, which stores the vtable `0102102c` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `004d91b0`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_004d9190(e: &mut Engine, this: Ptr) {
    e.call(0x004d_91b0, &args![this]);
}

// Translated from 004d8fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004d9070` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004d8fd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004d_9070, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004d9130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004d90d0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004d9130(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004d_90d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004d9160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004d9190` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004d9160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004d_9190, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004d91e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004d91b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004d91e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004d_91b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004d8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the 16-bit word at `this + 0x3c` (`thiscall`, result in `AX`).
pub fn fn_004d8a00(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x3c)
}

// ---- The scene-graph tree window (n039) ---------------------------------------------------
//
// The functions below build and drive a Win32 tree-view window over a scene graph: the window
// class is the string at `01189338` ("NiTreeCtrl"), the view is a "SysTreeView32" child. The
// object they work on (`this`) holds the module handle at +0x0, a word at +0x4, the frame
// window at +0x8, the tree view at +0xC, the root item at +0x10, the image list at +0x14, a map
// of objects to tree items at +0x18 (its vtable `01020fe4`, built by `004d8fa0`), the root
// object at +0x28 and the window title (a heap copy) at +0x2C. The engine map gives none of
// them a name, so the class is unconfirmed. Windows functions are called through their import
// slots (names from the executable's import table).

/// `SendMessageA` (import slot).
const SEND_MESSAGE_A: u32 = 0x00fd_f28c;
/// `DestroyWindow` (import slot).
const DESTROY_WINDOW: u32 = 0x00fd_f290;
/// `ImageList_Destroy` (import slot).
const IMAGE_LIST_DESTROY: u32 = 0x00fd_f01c;
/// `ImageList_LoadImageA` (import slot).
const IMAGE_LIST_LOAD_IMAGE_A: u32 = 0x00fd_f020;
/// `InitCommonControlsEx` (import slot).
const INIT_COMMON_CONTROLS_EX: u32 = 0x00fd_f024;
/// `LoadIconA` (import slot).
const LOAD_ICON_A: u32 = 0x00fd_f280;
/// `LoadCursorA` (import slot).
const LOAD_CURSOR_A: u32 = 0x00fd_f2a0;
/// `GetStockObject` (import slot).
const GET_STOCK_OBJECT: u32 = 0x00fd_f03c;
/// `RegisterClassA` (import slot).
const REGISTER_CLASS_A: u32 = 0x00fd_f2b4;
/// `CreateWindowExA` (import slot).
const CREATE_WINDOW_EX_A: u32 = 0x00fd_f2b8;
/// `GetClientRect` (import slot).
const GET_CLIENT_RECT: u32 = 0x00fd_f2bc;
/// `ShowWindow` (import slot).
const SHOW_WINDOW: u32 = 0x00fd_f288;
/// `UpdateWindow` (import slot).
const UPDATE_WINDOW: u32 = 0x00fd_f284;
/// `ShowCursor` (import slot).
const SHOW_CURSOR: u32 = 0x00fd_f2ac;
/// `GetWindow` (import slot).
const GET_WINDOW: u32 = 0x00fd_f2a8;
/// `SetWindowPos` (import slot).
const SET_WINDOW_POS: u32 = 0x00fd_f2a4;
/// `DefWindowProcA` (import slot).
const DEF_WINDOW_PROC_A: u32 = 0x00fd_f2e0;

/// Tree-view messages (Windows SDK values).
const TVM_INSERTITEMA: u32 = 0x1100;
const TVM_DELETEITEM: u32 = 0x1101;
const TVM_EXPAND: u32 = 0x1102;
const TVM_SETIMAGELIST: u32 = 0x1109;
const TVM_GETNEXTITEM: u32 = 0x110a;
const TVM_SELECTITEM: u32 = 0x110b;
const TVM_GETITEMA: u32 = 0x110c;
/// `TVI_ROOT` and `TVI_LAST` (Windows SDK values).
const TVI_ROOT: u32 = 0xffff_0000;
const TVI_LAST: u32 = 0xffff_0002;

/// `00464f30` (cdecl, two words pushed): the string the tree items show.
const ITEM_TEXT: u32 = 0x0046_4f30;
/// `00401000` (cdecl): the game's `operator new`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00658930` (thiscall): the number of strings in a string list.
const STRING_COUNT: u32 = 0x0065_8930;
/// `00877a30` (thiscall, `RET 4`): the address of a string list's entry.
const STRING_AT: u32 = 0x0087_7a30;
/// `005e03d0` (thiscall): empties a string list.
const STRING_CLEAR: u32 = 0x005e_03d0;
/// `00559450` (thiscall): the word at +0 of an object.
const FIRST_WORD: u32 = 0x0055_9450;
/// `004add50` (cdecl: destination, source, count): copies a name.
const COPY_NAME: u32 = 0x004a_dd50;
/// `00653270` (cdecl: type record, object): the object when it is of that type, else null.
const IS_KIND_OF: u32 = 0x0065_3270;
/// `0057cbe0` (thiscall, `RET 4`): steps a list position, returns the node it passed.
const LIST_NEXT: u32 = 0x0057_cbe0;
/// `00406d00` (cdecl, variable): `sprintf_s`.
const SPRINTF_S: u32 = 0x0040_6d00;
/// `00406d50` (cdecl: destination, size, source): appends a string.
const STRCAT_S: u32 = 0x0040_6d50;
/// The window procedure's callback (a function pointer the tree's user installs).
const TREE_CALLBACK: u32 = 0x011c_6f18;
/// The two words the tree window clears on close and shifts when it lists a controller
/// manager (`011f36f4` is the newest, `011f36f8` the one before).
const LAST_MANAGER: u32 = 0x011f_36f4;
const PREVIOUS_MANAGER: u32 = 0x011f_36f8;

/// `SendMessageA(window, message, wparam, lparam)`.
fn send_message(e: &mut Engine, window: u32, message: u32, wparam: u32, lparam: u32) -> u32 {
    e.call(SEND_MESSAGE_A, &args![window, message, wparam, lparam])
        .u32()
}

/// `TVM_INSERTITEMA` on the tree view (`this` +0xC) with the insert block at `item`.
fn insert_item(e: &mut Engine, this: u32, item: u32) -> u32 {
    let tree = e.mem.u32(this + 0xc);
    send_message(e, tree, TVM_INSERTITEMA, 0, item)
}

/// The string list the tree lists an object's attributes into: `operator new(0x10)` and the
/// list constructor `004d9100` (0x80, 0x80), or null when the allocation fails.
fn new_string_list(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block == 0 {
        0
    } else {
        fn_004d9100(e, Ptr::new(block), 0x80, 0x80).addr()
    }
}

/// Deletes a string list through its vtable slot 0 (with 1) unless it is null.
fn delete_string_list(e: &mut Engine, list: u32) {
    if list != 0 {
        e.vcall(list, 0, &args![1u32]);
    }
}

/// The loop every lister ends with: each string of `list` becomes a child item (image 6) under
/// the insert block's parent, then the list is emptied (`005e03d0`).
fn insert_list_strings(e: &mut Engine, this: u32, item: u32, list: u32) {
    let mut index = 0u32;
    while index < e.call(STRING_COUNT, &args![list]).u32() {
        let entry = e.call(STRING_AT, &args![list, index]).u32();
        let string = e.mem.u32(entry);
        let text = e.call(ITEM_TEXT, &args![string, 0u32]).u32();
        e.mem.set_u32(item + 0x18, text);
        e.mem.set_u32(item + 0x20, 6);
        e.mem.set_u32(item + 0x24, 6);
        insert_item(e, this, item);
        index += 1;
    }
    e.call(STRING_CLEAR, &args![list]);
}

/// The name an object shows: vtable slot 8, then `00559450`, copied (at most 0x3F bytes,
/// `004add50`) into the 0x40-byte `buffer`, whose last byte is cleared.
fn copy_object_name(e: &mut Engine, object: u32, buffer: u32) {
    let named = e.vcall(object, 8, &args![]).u32();
    let name = e.call(FIRST_WORD, &args![named]).u32();
    e.call(COPY_NAME, &args![buffer, name, 0x3fu32]);
    e.mem.set_u8(buffer + 0x3f, 0);
}

/// The group item the listers start with: an item `text` (image 5) under `parent`.
fn insert_group(e: &mut Engine, this: u32, item: u32, parent: u32, text: u32) -> u32 {
    let shown = e.call(ITEM_TEXT, &args![text, 0u32]).u32();
    e.mem.set_u32(item + 0x18, shown);
    e.mem.set_u32(item + 0x20, 5);
    e.mem.set_u32(item + 0x24, 5);
    e.mem.set_u32(item, parent);
    insert_item(e, this, item)
}

/// An object's own item: its copied name (image `image`) under the item at `item` +0, which becomes
/// that new item; then its attribute strings (vtable slot 0x60 into `list`) under it.
#[allow(clippy::too_many_arguments)]
fn insert_named_object(
    e: &mut Engine,
    this: u32,
    item: u32,
    buffer: u32,
    object: u32,
    image: u32,
    list: u32,
) -> u32 {
    copy_object_name(e, object, buffer);
    let shown = e.call(ITEM_TEXT, &args![buffer, 0u32]).u32();
    e.mem.set_u32(item + 0x18, shown);
    e.mem.set_u32(item + 0x20, image);
    e.mem.set_u32(item + 0x24, image);
    let created = insert_item(e, this, item);
    e.mem.set_u32(item, created);
    e.vcall(object, 0x60, &args![list]);
    insert_list_strings(e, this, item, list);
    created
}

// Translated from 004d61b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d61b0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The tree window's constructor (`RET 0x20`): builds the object map at +0x18 (`004d8fa0`
/// with 0x25), stores `module` (+0x0), `word` (+0x4) and `root` (+0x28), copies `title`
/// (`strlen` `0044a670`, `operator new`, `00464f30`, `004046f0`) to +0x2C, creates the windows
/// (`004d6340` with the rectangle), loads the image list (`004d6490`) and, with a root, fills
/// the tree (`004d64c0`, no title). Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_004d61b0(
    e: &mut Engine,
    this: Ptr,
    module: u32,
    word: u32,
    root: u32,
    title: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
) -> Ptr {
    let base = this.addr();
    fn_004d8fa0(e, Ptr::new(base + 0x18), 0x25);
    e.mem.set_u32(base, module);
    e.mem.set_u32(base + 0x4, word);
    e.mem.set_u32(base + 0x28, root);
    let length = e.call(0x0044_a670, &args![title]).u32();
    let copy = e.call(OPERATOR_NEW, &args![length.wrapping_add(1)]).u32();
    e.mem.set_u32(base + 0x2c, copy);
    let source = e.call(ITEM_TEXT, &args![title, 0u32]).u32();
    let destination = e.mem.u32(base + 0x2c);
    e.call(0x0040_46f0, &args![destination, source]);
    fn_004d6340(e, this, left, top, width, height);
    fn_004d6490(e, this);
    if root != 0 {
        fn_004d64c0(e, this, root, 0);
    }
    this
}

// Translated from 004d62a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d62a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The tree window's destructor body: frees the title (`00401030`), deletes every tree item
/// (`004d7490`), destroys the image list (`004d7470`) and the windows (`004d7440`), clears the
/// two manager words (`011f36f8`, then `011f36f4` from it) and destroys the object map at
/// +0x18 (`004d9070`).
pub fn fn_004d62a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let title = e.mem.u32(base + 0x2c);
    e.call(0x0040_1030, &args![title]);
    fn_004d7490(e, this);
    fn_004d7470(e, this);
    fn_004d7440(e, this);
    e.set_global::<u32>(PREVIOUS_MANAGER, 0);
    let cleared = e.global::<u32>(PREVIOUS_MANAGER);
    e.set_global::<u32>(LAST_MANAGER, cleared);
    fn_004d9070(e, Ptr::new(base + 0x18));
}

// Translated from 004d6340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the tree window (`RET 0x10`): `InitCommonControlsEx` (size 8, classes 2), registers
/// the window class (style 3, window procedure `004d75b0`, the module at +0x0, `LoadIconA` and
/// `LoadCursorA` with 0x7F00, `GetStockObject(0)`, the class name `00464f30` makes of
/// `01189338`), creates the frame window (style 0xCE0200, title +0x2C, the rectangle given)
/// into +0x8 and, over its client rectangle (`GetClientRect`, width and height +1), the
/// "SysTreeView32" child (`01020e24`, title `01011584`, style 0x50010007) into +0xC.
pub fn fn_004d6340(e: &mut Engine, this: Ptr, left: u32, top: u32, width: u32, height: u32) {
    let base = this.addr();
    // The frame below EBP: the window class at +0x0 (EBP-0x44), the style local at +0x28,
    // the client rectangle at +0x2C and the controls block at +0x3C.
    e.with_stack(0x44, |e, frame| {
        let class = frame.addr();
        let rect = class + 0x2c;
        let controls = class + 0x3c;
        e.mem.set_u32(controls, 8);
        e.mem.set_u32(controls + 4, 2);
        e.call(INIT_COMMON_CONTROLS_EX, &args![controls]);
        e.mem.set_u32(class, 3);
        e.mem.set_u32(class + 0x4, 0x004d_75b0);
        e.mem.set_u32(class + 0x8, 0);
        e.mem.set_u32(class + 0xc, 0);
        let module = e.mem.u32(base);
        e.mem.set_u32(class + 0x10, module);
        let icon = e.call(LOAD_ICON_A, &args![0u32, 0x7f00u32]).u32();
        e.mem.set_u32(class + 0x14, icon);
        let cursor = e.call(LOAD_CURSOR_A, &args![0u32, 0x7f00u32]).u32();
        e.mem.set_u32(class + 0x18, cursor);
        let brush = e.call(GET_STOCK_OBJECT, &args![0u32]).u32();
        e.mem.set_u32(class + 0x1c, brush);
        let class_name = e.call(ITEM_TEXT, &args![0x0118_9338u32, 0u32]).u32();
        e.mem.set_u32(class + 0x24, class_name);
        e.mem.set_u32(class + 0x20, 0);
        e.call(REGISTER_CLASS_A, &args![class]);
        e.mem.set_u32(class + 0x28, 0x00ce_0200);
        let module = e.mem.u32(base);
        let style = e.mem.u32(class + 0x28);
        let title = e.mem.u32(base + 0x2c);
        let class_name = e.call(ITEM_TEXT, &args![0x0118_9338u32, 0u32]).u32();
        let frame_window = e
            .call(
                CREATE_WINDOW_EX_A,
                &args![
                    0u32, class_name, title, style, left, top, width, height, 0u32, 0u32, module,
                    0u32
                ],
            )
            .u32();
        e.mem.set_u32(base + 0x8, frame_window);
        e.mem.set_u32(class + 0x28, 0x5001_0007);
        let frame_window = e.mem.u32(base + 0x8);
        e.call(GET_CLIENT_RECT, &args![frame_window, rect]);
        let module = e.mem.u32(base);
        let frame_window = e.mem.u32(base + 0x8);
        let height = e
            .mem
            .u32(rect + 0xc)
            .wrapping_sub(e.mem.u32(rect + 0x4))
            .wrapping_add(1);
        let width = e
            .mem
            .u32(rect + 0x8)
            .wrapping_sub(e.mem.u32(rect))
            .wrapping_add(1);
        let style = e.mem.u32(class + 0x28);
        let tree = e
            .call(
                CREATE_WINDOW_EX_A,
                &args![
                    0u32,
                    0x0102_0e24u32,
                    0x0101_1584u32,
                    style,
                    0u32,
                    0u32,
                    width,
                    height,
                    frame_window,
                    0u32,
                    module,
                    0u32
                ],
            )
            .u32();
        e.mem.set_u32(base + 0xc, tree);
    });
}

// Translated from 004d6490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the tree's image list (`ImageList_LoadImageA`: the module at +0x0, resource 0xB4,
/// 16 wide, grow 1, mask colour 0xFF000000, type 0, flags 0) into +0x14.
pub fn fn_004d6490(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let module = e.mem.u32(base);
    let list = e
        .call(
            IMAGE_LIST_LOAD_IMAGE_A,
            &args![module, 0xb4u32, 0x10u32, 1u32, 0xff00_0000u32, 0u32, 0u32],
        )
        .u32();
    e.mem.set_u32(base + 0x14, list);
}

// Translated from 004d64c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the tree from `root` (`RET 8`): empties the object map (`00438af0` on +0x18), sets
/// the image list (`TVM_SETIMAGELIST`), inserts the root item (`TVI_ROOT`, `TVI_LAST`, mask
/// 0x27, text `00464f30` of `title` or the window title at +0x2C when `title` is null,
/// image 0, `root` as its parameter) into +0x10, lists the graph under it (`004d66d0`) and
/// shows the frame window (`ShowWindow` 10, `UpdateWindow`).
pub fn fn_004d64c0(e: &mut Engine, this: Ptr, root: u32, title: u32) {
    let base = this.addr();
    e.call(0x0043_8af0, &args![base + 0x18]);
    let images = e.mem.u32(base + 0x14);
    let tree = e.mem.u32(base + 0xc);
    send_message(e, tree, TVM_SETIMAGELIST, 0, images);
    e.with_stack(0x30, |e, item| {
        let item = item.addr();
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item, TVI_ROOT);
        let text = if title != 0 {
            e.call(ITEM_TEXT, &args![title, 0u32]).u32()
        } else {
            e.mem.u32(base + 0x2c)
        };
        e.mem.set_u32(item + 0x18, text);
        e.mem.set_u32(item + 0x20, 0);
        let image = e.mem.u32(item + 0x20);
        e.mem.set_u32(item + 0x24, image);
        e.mem.set_u32(item + 0x2c, root);
        let created = insert_item(e, base, item);
        e.mem.set_u32(base + 0x10, created);
    });
    let root_item = e.mem.u32(base + 0x10);
    fn_004d66d0(e, this, root_item, Ptr::new(root));
    let frame_window = e.mem.u32(base + 0x8);
    e.call(SHOW_WINDOW, &args![frame_window, 10u32]);
    let frame_window = e.mem.u32(base + 0x8);
    e.call(UPDATE_WINDOW, &args![frame_window]);
}

// Translated from 004d65a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d65a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Shows `object` in the tree (`RET 4`; nothing for null): finds its item in the object map
/// (`00853130` on +0x18), collects the item and each parent (`TVM_GETNEXTITEM` 3) in a local
/// list (constructor `0096a2d0`, add `005ae3d0`), expands each collected item
/// (`TVM_EXPAND` 2; the walk `006815c0`/`00726070` from the list itself), selects the last
/// item (`TVM_SELECTITEM` 5 and 9; the walk leaves it null, so this never runs) and destroys
/// the list (`0046ffb0`).
pub fn fn_004d65a0(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    if object == 0 {
        return;
    }
    // EBP-0x1C: the item; EBP-0x18: the list (8 bytes).
    e.with_stack(0xc, |e, frame| {
        let found = frame.addr();
        let list = found + 4;
        e.mem.set_u32(found, 0);
        e.call(0x0085_3130, &args![base + 0x18, object, found]);
        e.call(0x0096_a2d0, &args![list]);
        while e.mem.u32(found) != 0 {
            e.call(0x005a_e3d0, &args![list, found]);
            let tree = e.mem.u32(base + 0xc);
            let item = e.mem.u32(found);
            let parent = send_message(e, tree, TVM_GETNEXTITEM, 3, item);
            e.mem.set_u32(found, parent);
        }
        let mut node = list;
        while node != 0 {
            let entry = e.call(0x0068_15c0, &args![node]).u32();
            if e.mem.u32(entry) == 0 {
                break;
            }
            let entry = e.call(0x0068_15c0, &args![node]).u32();
            let item = e.mem.u32(entry);
            let tree = e.mem.u32(base + 0xc);
            send_message(e, tree, TVM_EXPAND, 2, item);
            node = e.call(0x0072_6070, &args![node]).u32();
        }
        if e.mem.u32(found) != 0 {
            let tree = e.mem.u32(base + 0xc);
            let item = e.mem.u32(found);
            send_message(e, tree, TVM_SELECTITEM, 5, item);
            let tree = e.mem.u32(base + 0xc);
            let item = e.mem.u32(found);
            send_message(e, tree, TVM_SELECTITEM, 9, item);
        }
        e.call(0x0046_ffb0, &args![list]);
    });
}

// Translated from 004d66d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d66d0 - the exception-unwinding frame (FS:[0]) and the security cookie check (00ec408c) are not translated.
/// Lists `object` and everything under it into the tree (`RET 8`), under the item `parent`:
/// the object's item (text `"%s \"%s\" (%.0f,%.0f,%.0f)"` from its vtable slot 8 name, its
/// name `00413f40` and the three floats `0043c490` points at, plus `" Scale %.1f"` when
/// `009cdae0` is not 1.0 and `" Culled"` when `00456610` says so; image 3 when vtable slot 0xC
/// answers, else 1 or 2 by the type tests `0043b300` with `011f4a18`/`011f4a28`, else 4),
/// recorded in the object map (`00844700` on +0x18); under it "Attributes" with the object's
/// strings (slot 0x60), "Controllers" (`0043b230` and the chain `004a8a90`, also the
/// controllers of the property list's entries when one has any, each with its strings and,
/// for `011f36ac` kinds, the sequences `004d8690`, for `011f36bc` kinds the interpolators
/// `004d8830`), the modifiers (`004d7ab0`), "Properties" (the property list `00430830`
/// unless `0076b610` says it is empty), "SkinInstance" (slot 0x18 and `0043fad0`) and the
/// collision object (`006838b0`, `004d7df0`); then, when slot 0xC answers, each child
/// (`0043b480`/`0043b4a0`) recursively.
pub fn fn_004d66d0(e: &mut Engine, this: Ptr, parent: u32, object: Ptr) {
    let base = this.addr();
    let object = object.addr();
    e.with_stack(0x3c0, |e, frame| {
        let ebp = frame.addr() + 0x3c0;
        let item = ebp - 0x274;
        let text = ebp - 0x234;
        let scale_text = ebp - 0x124;
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item, parent);
        e.mem.set_u32(item + 0x2c, object);
        let position = e.call(0x0043_c490, &args![object]).u32();
        let z = x87_load32(e.mem.u32(position + 8));
        let position = e.call(0x0043_c490, &args![object]).u32();
        let y = x87_load32(e.mem.u32(position + 4));
        let position = e.call(0x0043_c490, &args![object]).u32();
        let x = x87_load32(e.mem.u32(position));
        let named = e.call(0x0041_3f40, &args![object]).u32();
        let name = e.mem.u32(named);
        let kind = e.vcall(object, 8, &args![]).u32();
        let kind_name = e.call(FIRST_WORD, &args![kind]).u32();
        e.call(
            SPRINTF_S,
            &args![text, 0x104u32, 0x0102_0e7cu32, kind_name, name, x, y, z],
        );
        let scale = e.call(0x009c_dae0, &args![object]).f64();
        // FCOMP with 1.0 (`01012070`), `TEST AH,0x44`, `JNP`: skipped only when equal.
        if scale.partial_cmp(&e.global::<f64>(0x0101_2070)) != Some(std::cmp::Ordering::Equal) {
            let scale = e.call(0x009c_dae0, &args![object]).f64();
            e.call(
                SPRINTF_S,
                &args![scale_text, 0x104u32, 0x0102_0e70u32, scale],
            );
            e.call(STRCAT_S, &args![text, 0x104u32, scale_text]);
        }
        if e.call(0x0045_6610, &args![object]).bool() {
            e.call(STRCAT_S, &args![text, 0x104u32, 0x0102_0e68u32]);
        }
        let shown = e.call(ITEM_TEXT, &args![text, 0u32]).u32();
        e.mem.set_u32(item + 0x18, shown);
        let image = if e.vcall(object, 0xc, &args![]).u32() != 0 {
            3
        } else if e.call(0x0043_b300, &args![0x011f_4a18u32, object]).bool() {
            1
        } else if e.call(0x0043_b300, &args![0x011f_4a28u32, object]).bool() {
            2
        } else {
            4
        };
        e.mem.set_u32(item + 0x20, image);
        e.mem.set_u32(item + 0x24, image);
        let created = insert_item(e, base, item);
        e.mem.set_u32(item, created);
        e.mem.set_u32(item + 0x2c, 0);
        let created = e.mem.u32(item);
        e.call(0x0084_4700, &args![base + 0x18, object, created]);
        // [EBP+8]: the parent item, replaced by the object's own item.
        let own_item = e.mem.u32(item);
        let shown = e.call(ITEM_TEXT, &args![0x0102_0e5cu32, 0u32]).u32();
        e.mem.set_u32(item + 0x18, shown);
        e.mem.set_u32(item + 0x20, 5);
        e.mem.set_u32(item + 0x24, 5);
        let attributes = insert_item(e, base, item);
        e.mem.set_u32(item, attributes);
        let list = new_string_list(e);
        e.vcall(object, 0x60, &args![list]);
        insert_list_strings(e, base, item, list);

        // Whether an entry of the property list has controllers ([EBP-0x125]).
        let mut entry_has_controllers = false;
        let properties = e.call(0x0043_0830, &args![object]).u32();
        if !e.call(0x0076_b610, &args![properties]).bool() {
            let properties = e.call(0x0043_0830, &args![object]).u32();
            let position = ebp - 0x278;
            let first = e.call(FIRST_WORD, &args![properties]).u32();
            e.mem.set_u32(position, first);
            while e.mem.u32(position) != 0 {
                let properties = e.call(0x0043_0830, &args![object]).u32();
                let node = e.call(LIST_NEXT, &args![properties, position]).u32();
                let property = e.call(FIRST_WORD, &args![node]).u32();
                if e.call(0x0043_b230, &args![property]).u32() != 0 {
                    entry_has_controllers = true;
                    break;
                }
            }
        }
        let mut controller = e.call(0x0043_b230, &args![object]).u32();
        if controller != 0 || entry_has_controllers {
            let group = insert_group(e, base, item, own_item, 0x0102_0e50);
            while controller != 0 {
                e.mem.set_u32(item, group);
                insert_named_object(e, base, item, ebp - 0x2cc, controller, 6, list);
                list_controller_detail(e, this, item, controller);
                controller = e.call(0x004a_8a90, &args![controller]).u32();
            }
            if entry_has_controllers {
                let properties = e.call(0x0043_0830, &args![object]).u32();
                let position = ebp - 0x2d4;
                let first = e.call(FIRST_WORD, &args![properties]).u32();
                e.mem.set_u32(position, first);
                while e.mem.u32(position) != 0 {
                    let properties = e.call(0x0043_0830, &args![object]).u32();
                    let node = e.call(LIST_NEXT, &args![properties, position]).u32();
                    let property = e.call(FIRST_WORD, &args![node]).u32();
                    let mut controller = e.call(0x0043_b230, &args![property]).u32();
                    while controller != 0 {
                        e.mem.set_u32(item, group);
                        insert_named_object(e, base, item, ebp - 0x324, controller, 6, list);
                        list_controller_detail(e, this, item, controller);
                        controller = e.call(0x004a_8a90, &args![controller]).u32();
                    }
                }
            }
        }
        fn_004d7ab0(e, this, own_item, object);
        let properties = e.call(0x0043_0830, &args![object]).u32();
        if !e.call(0x0076_b610, &args![properties]).bool() {
            let group = insert_group(e, base, item, own_item, 0x0102_0e44);
            let properties = e.call(0x0043_0830, &args![object]).u32();
            let position = ebp - 0x32c;
            let first = e.call(FIRST_WORD, &args![properties]).u32();
            e.mem.set_u32(position, first);
            while e.mem.u32(position) != 0 {
                let properties = e.call(0x0043_0830, &args![object]).u32();
                let node = e.call(LIST_NEXT, &args![properties, position]).u32();
                let property = e.call(FIRST_WORD, &args![node]).u32();
                e.mem.set_u32(item, group);
                insert_named_object(e, base, item, ebp - 0x37c, property, 6, list);
            }
        }
        if e.vcall(object, 0x18, &args![]).u32() != 0 {
            let skin = e.vcall(object, 0x18, &args![]).u32();
            if e.call(0x0043_fad0, &args![skin]).u32() != 0 {
                let group = insert_group(e, base, item, own_item, 0x0102_0e34);
                e.mem.set_u32(item, group);
                let skin_list = new_string_list(e);
                let skin = e.vcall(object, 0x18, &args![]).u32();
                let skin_data = e.call(0x0043_fad0, &args![skin]).u32();
                e.vcall(skin_data, 0x60, &args![skin_list]);
                insert_list_strings(e, base, item, skin_list);
            }
        }
        let collision = e.call(0x0068_38b0, &args![object]).u32();
        if collision != 0 {
            fn_004d7df0(e, this, own_item, Ptr::new(collision));
        }
        delete_string_list(e, list);
        if e.vcall(object, 0xc, &args![]).u32() != 0 {
            let mut index = 0u32;
            while index < e.call(0x0043_b480, &args![object]).u32() {
                let child = e.call(0x0043_b4a0, &args![object, index]).u32();
                if child != 0 {
                    fn_004d66d0(e, this, own_item, Ptr::new(child));
                }
                index += 1;
            }
        }
    });
}

/// The end of each controller's entry in `004d66d0`: a controller of the kind `011f36ac`
/// becomes the newest manager (`011f36f4`, the one before moving to `011f36f8`) and has its
/// sequences listed (`004d8690`); else one of the kind `011f36bc` has its interpolators
/// listed (`004d8830`), both under the controller's item (the insert block's parent).
fn list_controller_detail(e: &mut Engine, this: Ptr, item: u32, controller: u32) {
    let manager = e.call(IS_KIND_OF, &args![0x011f_36acu32, controller]).u32();
    if manager != 0 {
        let previous = e.global::<u32>(LAST_MANAGER);
        e.set_global::<u32>(PREVIOUS_MANAGER, previous);
        e.set_global::<u32>(LAST_MANAGER, manager);
        let parent = e.mem.u32(item);
        fn_004d8690(e, this, parent, manager);
    } else {
        let interpolators = e.call(IS_KIND_OF, &args![0x011f_36bcu32, controller]).u32();
        if interpolators != 0 {
            let parent = e.mem.u32(item);
            fn_004d8830(e, this, parent, interpolators);
        }
    }
}

// Translated from 004d7440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the tree view (+0xC) and the frame window (+0x8) (`DestroyWindow`).
pub fn fn_004d7440(e: &mut Engine, this: Ptr) {
    let tree = e.mem.u32(this.addr() + 0xc);
    e.call(DESTROY_WINDOW, &args![tree]);
    let frame_window = e.mem.u32(this.addr() + 0x8);
    e.call(DESTROY_WINDOW, &args![frame_window]);
}

// Translated from 004d7470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the image list at +0x14 (`ImageList_Destroy`).
pub fn fn_004d7470(e: &mut Engine, this: Ptr) {
    let images = e.mem.u32(this.addr() + 0x14);
    e.call(IMAGE_LIST_DESTROY, &args![images]);
}

// Translated from 004d7490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every item of the tree view at +0xC (`TVM_DELETEITEM` with `TVI_ROOT`).
pub fn fn_004d7490(e: &mut Engine, this: Ptr) {
    let tree = e.mem.u32(this.addr() + 0xc);
    send_message(e, tree, TVM_DELETEITEM, 0, TVI_ROOT);
}

// Translated from 004d74c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Expands or collapses (`code`, `TVM_EXPAND`) every descendant of `parent` in the tree view
/// `window` (cdecl) whose item parameter's vtable slot 0xC answers an object that `00453470`
/// accepts: walks the children (`TVM_GETNEXTITEM` 4, then 1), reads each item into a cleared
/// 0x38-byte item block (`TVM_GETITEMA`, mask 0xD, text buffer `011c6e10` of 0x104 bytes) and
/// recurses into the ones it expands.
pub fn fn_004d74c0(e: &mut Engine, window: u32, parent: u32, code: u32) {
    // EBP-0x38: the item block (its handle at +0x4, its parameter at +0x24).
    e.with_stack(0x38, |e, block| {
        let block = block.addr();
        e.call(0x0040_3d30, &args![block, 0u32, 0x38u32]);
        e.mem.set_u32(block, 0xd);
        e.mem.set_u32(block + 0x10, 0x011c_6e10);
        e.mem.set_u32(block + 0x14, 0x104);
        let first = send_message(e, window, TVM_GETNEXTITEM, 4, parent);
        e.mem.set_u32(block + 0x4, first);
        while e.mem.u32(block + 0x4) != 0 {
            if send_message(e, window, TVM_GETITEMA, 0, block) != 0 {
                let parameter = e.mem.u32(block + 0x24);
                let mut object = 0;
                if parameter != 0 {
                    object = e.vcall(parameter, 0xc, &args![]).u32();
                }
                if object != 0 && e.call(0x0045_3470, &args![object]).u32() != 0 {
                    let child = e.mem.u32(block + 0x4);
                    send_message(e, window, TVM_EXPAND, code, child);
                    let child = e.mem.u32(block + 0x4);
                    fn_004d74c0(e, window, child, code);
                }
            }
            let current = e.mem.u32(block + 0x4);
            let next = send_message(e, window, TVM_GETNEXTITEM, 1, current);
            e.mem.set_u32(block + 0x4, next);
        }
    });
}

// Translated from 004d75b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree window's window procedure (`RET 0x10`): `WM_SIZE` (5) resizes the first child
/// (`GetWindow` 5, `SetWindowPos` to the low and high words of `lparam`, flags 2, then
/// `ShowWindow` 1); `WM_ACTIVATE` (6) with the low word of `wparam` 1 or 2 shows the cursor
/// (`ShowCursor(1)`); `WM_NOTIFY` (0x4E) handles three tree notifications through the
/// callback `004d7890`: `TVN_KEYDOWN` (-0x19C: the caret item's parameter, 3, the key and the
/// flags; answers 1 when the callback does), `TVN_SELCHANGEDA` (-0x192: with a callback
/// installed, the old item's parameter with 1 and the new one's with 0; answers 1) and
/// `NM_RCLICK` (-5: the highlighted item's parameter with 2; then the highlighted or caret
/// item is selected and it and its children expanded or collapsed by its `TVIS_EXPANDED`
/// state, `004d74c0`; answers 1). Everything else goes to `DefWindowProcA`.
pub fn fn_004d75b0(e: &mut Engine, window: u32, message: u32, wparam: u32, lparam: u32) -> u32 {
    // The frame below EBP (0x7C bytes): the 0x38-byte item block at EBP-0x3C and the
    // 0x28-byte one at EBP-0x74.
    let handled = e.with_stack(0x7c, |e, frame| {
        let ebp = frame.addr() + 0x7c;
        let block = ebp - 0x3c;
        let key_item = ebp - 0x74;
        match message {
            5 => {
                let child = e.call(GET_WINDOW, &args![window, 5u32]).u32();
                if child != 0 {
                    let height = (lparam >> 16) & 0xffff;
                    let width = lparam & 0xffff;
                    e.call(
                        SET_WINDOW_POS,
                        &args![child, 0u32, 0u32, 0u32, width, height, 2u32],
                    );
                    e.call(SHOW_WINDOW, &args![child, 1u32]);
                }
                false
            }
            6 => {
                if wparam & 0xffff == 1 || wparam & 0xffff == 2 {
                    e.call(SHOW_CURSOR, &args![1u32]);
                }
                false
            }
            0x4e => {
                let header = lparam;
                match e.mem.u32(header + 8) {
                    0xffff_fe64 => {
                        e.call(0x0040_3d30, &args![block, 0u32, 0x38u32]);
                        let tree = e.mem.u32(header);
                        let caret = send_message(e, tree, TVM_GETNEXTITEM, 9, 0);
                        e.mem.set_u32(key_item + 0x4, caret);
                        if e.mem.u32(key_item + 0x4) == 0 {
                            return false;
                        }
                        e.mem.set_u32(key_item, 4);
                        let tree = e.mem.u32(header);
                        send_message(e, tree, TVM_GETITEMA, 0, key_item);
                        let flags = e.mem.u32(header + 0xe);
                        let key = u32::from(e.mem.u16(header + 0xc));
                        let parameter = e.mem.u32(key_item + 0x24);
                        fn_004d7890(e, parameter, 3, key, flags) != 0
                    }
                    0xffff_fe6e => {
                        if e.global::<u32>(TREE_CALLBACK) != 0 {
                            let old_parameter = e.mem.u32(header + 0x34);
                            fn_004d7890(e, old_parameter, 1, 0, 0);
                            let new_parameter = e.mem.u32(header + 0x5c);
                            fn_004d7890(e, new_parameter, 0, 0, 0);
                        }
                        true
                    }
                    0xffff_fffb => {
                        e.call(0x0040_3d30, &args![block, 0u32, 0x38u32]);
                        e.mem.set_u32(block, 0x14);
                        let tree = e.mem.u32(header);
                        let current = e.mem.u32(block + 0x4);
                        let highlighted = send_message(e, tree, TVM_GETNEXTITEM, 8, current);
                        e.mem.set_u32(block + 0x4, highlighted);
                        let parameter = e.mem.u32(block + 0x24);
                        fn_004d7890(e, parameter, 2, 0, 0);
                        if e.mem.u32(block + 0x4) == 0 {
                            let tree = e.mem.u32(header);
                            let current = e.mem.u32(block + 0x4);
                            let caret = send_message(e, tree, TVM_GETNEXTITEM, 9, current);
                            e.mem.set_u32(block + 0x4, caret);
                        }
                        e.mem.set_u32(block, 0xd);
                        e.mem.set_u32(block + 0x10, 0x011c_6e10);
                        e.mem.set_u32(block + 0x14, 0x104);
                        if e.mem.u32(block + 0x4) == 0 {
                            return false;
                        }
                        let tree = e.mem.u32(header);
                        if send_message(e, tree, TVM_GETITEMA, 0, block) == 0 {
                            return false;
                        }
                        let tree = e.mem.u32(header);
                        let selected = e.mem.u32(block + 0x4);
                        send_message(e, tree, TVM_SELECTITEM, 9, selected);
                        let code = if e.mem.u32(block + 0x8) & 0x20 != 0 {
                            1
                        } else {
                            2
                        };
                        let tree = e.mem.u32(header);
                        let selected = e.mem.u32(block + 0x4);
                        fn_004d74c0(e, tree, selected, code);
                        let tree = e.mem.u32(header);
                        let selected = e.mem.u32(block + 0x4);
                        send_message(e, tree, TVM_EXPAND, code, selected);
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    });
    if handled {
        return 1;
    }
    e.call(DEF_WINDOW_PROC_A, &args![window, message, wparam, lparam])
        .u32()
}

// Translated from 004d7890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the tree callback at `011c6f18` (cdecl, four words) and answers its value, or 0 when
/// none is installed.
pub fn fn_004d7890(e: &mut Engine, parameter: u32, event: u32, key: u32, flags: u32) -> u32 {
    let callback = e.global::<u32>(TREE_CALLBACK);
    if callback == 0 {
        return 0;
    }
    e.call(callback, &args![parameter, event, key, flags]).u32()
}

// Translated from 004d78c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d78c0 - the exception-unwinding frame (FS:[0]) and the security cookie check (00ec408c) are not translated.
/// A tree callback (cdecl: object, event, key; the shape `004d7890` calls): answers 1 for
/// events 0 and 1; for event 3 with an object, key 0x70 or 0x71 hands 1 or 0 to `00450f90`
/// and answers 1, key 0x72 updates it (`00a5a040`, then `00a59c60` with the update block
/// `0043d410` makes of 0.0, 0, 0) and answers 1, key 0x7A saves it to `"c:\<name>.nif"`
/// through a local stream (constructor `00a66150`, `NiStream::InsertObject` `00a66370`, the
/// name `0043b1b0` of `00413f40` or `"NullObject"`, `NiStream::Save` `00a640b0`, destructor
/// `00a65300`) and answers 1. Anything else answers 0.
pub fn fn_004d78c0(e: &mut Engine, object: u32, event: u32, key: u32) -> u32 {
    let mut result = 0;
    match event {
        0 | 1 => result = 1,
        3 if object != 0 => match key {
            0x70 => {
                e.call(0x0045_0f90, &args![object, 1u32]);
                result = 1;
            }
            0x71 => {
                e.call(0x0045_0f90, &args![object, 0u32]);
                result = 1;
            }
            0x72 => {
                e.call(0x00a5_a040, &args![object]);
                e.with_stack(0xc, |e, update| {
                    e.call(0x0043_d410, &args![update, 0.0f32, 0u32, 0u32]);
                    e.call(0x00a5_9c60, &args![object, update]);
                });
                result = 1;
            }
            0x7a => {
                // The frame below EBP: the stream at EBP-0x6F4, the path at EBP-0x12C.
                e.with_stack(0x6f8, |e, frame| {
                    let ebp = frame.addr() + 0x6f8;
                    let stream = ebp - 0x6f4;
                    let path = ebp - 0x12c;
                    e.call(0x00a6_6150, &args![stream]);
                    e.call(0x00a6_6370, &args![stream, object]);
                    let named = e.call(0x0041_3f40, &args![object]).u32();
                    let mut name = e.call(0x0043_b1b0, &args![named]).u32();
                    if name == 0 {
                        name = 0x0102_0ea4;
                    }
                    e.call(0x0040_6d30, &args![path, 0x104u32, 0x0102_0ea0u32]);
                    e.call(STRCAT_S, &args![path, 0x104u32, name]);
                    e.call(STRCAT_S, &args![path, 0x104u32, 0x0102_0e98u32]);
                    e.call(0x00a6_40b0, &args![stream, path]);
                    e.call(0x00a6_5300, &args![stream]);
                });
                result = 1;
            }
            _ => {}
        },
        _ => {}
    }
    result
}

// Translated from 004d7ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d7ab0 - the exception-unwinding frame (FS:[0]) and the security cookie check (00ec408c) are not translated.
/// Lists the modifiers of `object` (`RET 8`) when it is of the kind `012024e0`
/// (`00653270`): a "Modifiers" group under `parent`, then for each modifier
/// (`004d7d50`/`004d7d70`) its name item (image 0) with its strings (vtable slot 0x60).
pub fn fn_004d7ab0(e: &mut Engine, this: Ptr, parent: u32, object: u32) {
    let base = this.addr();
    let owner = e.call(IS_KIND_OF, &args![0x0120_24e0u32, object]).u32();
    if owner == 0 {
        return;
    }
    let list = new_string_list(e);
    // EBP-0x5C: the insert block; EBP-0xAC: the name buffer.
    e.with_stack(0xc0, |e, frame| {
        let ebp = frame.addr() + 0xc0;
        let item = ebp - 0x5c;
        let buffer = ebp - 0xac;
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item + 0x2c, object);
        let group = insert_group(e, base, item, parent, 0x0102_0eb0);
        let mut index = 0u32;
        while index < fn_004d7d50(e, Ptr::new(owner)) {
            let modifier = fn_004d7d70(e, Ptr::new(owner), index);
            if modifier != 0 {
                e.mem.set_u32(item, group);
                insert_named_object(e, base, item, buffer, modifier, 0, list);
            }
            index += 1;
        }
    });
    delete_string_list(e, list);
}

// Translated from 004d7d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of entries of the list at +0xC8 (`0044ddc0`).
pub fn fn_004d7d50(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0044_ddc0, &args![this.addr() + 0xc8]).u32()
}

// Translated from 004d7d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of the list at +0xC8 (`RET 4`), or null when `index` is not below its count
/// (`0044ddc0`) or the walk (`00559450`, `0057cbe0`) ends first.
pub fn fn_004d7d70(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let list = this.addr() + 0xc8;
    if index >= e.call(0x0044_ddc0, &args![list]).u32() {
        return 0;
    }
    e.with_stack(4, |e, position| {
        let mut count = 0u32;
        let first = e.call(FIRST_WORD, &args![list]).u32();
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            let node = e.call(LIST_NEXT, &args![list, position]).u32();
            let value = e.call(FIRST_WORD, &args![node]).u32();
            let current = count;
            count = count.wrapping_add(1);
            if current == index {
                return value;
            }
        }
        0
    })
}

// Translated from 004d7df0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d7df0 - the exception-unwinding frame (FS:[0]) and the security cookie check (00ec408c) are not translated.
/// Lists the collision object `object` (`RET 8`) when it is of the kind `01267b70`
/// (`00653270`): a "Havok" group under `parent`, the object's name item (image 0) with its
/// strings (vtable slot 0x60), then its body (`006fa820`) through `004d80b0`.
pub fn fn_004d7df0(e: &mut Engine, this: Ptr, parent: u32, object: Ptr) {
    let base = this.addr();
    let object = object.addr();
    let collision = e.call(IS_KIND_OF, &args![0x0126_7b70u32, object]).u32();
    if collision == 0 {
        return;
    }
    let list = new_string_list(e);
    // EBP-0xAC: the insert block; EBP-0x64: the name buffer.
    e.with_stack(0xc0, |e, frame| {
        let ebp = frame.addr() + 0xc0;
        let item = ebp - 0xac;
        let buffer = ebp - 0x64;
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item + 0x2c, object);
        let group = insert_group(e, base, item, parent, 0x0102_0ebc);
        copy_object_name(e, collision, buffer);
        let shown = e.call(ITEM_TEXT, &args![buffer, 0u32]).u32();
        e.mem.set_u32(item + 0x18, shown);
        e.mem.set_u32(item + 0x20, 0);
        e.mem.set_u32(item + 0x24, 0);
        e.mem.set_u32(item, group);
        let created = insert_item(e, base, item);
        e.mem.set_u32(item, created);
        e.vcall(object, 0x60, &args![list]);
        insert_list_strings(e, base, item, list);
        let body = e.call(0x006f_a820, &args![collision]).u32();
        fn_004d80b0(e, this, created, body);
    });
    delete_string_list(e, list);
}

// Translated from 004d80b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d80b0 - the exception-unwinding frame (FS:[0]) and the security cookie check (00ec408c) are not translated.
/// Lists a Havok object (`RET 8`; nothing for null): its name item (image 0) under `parent`
/// with its strings (vtable slot 0x60), then by its kind (`00653270`): `01268110` lists the
/// object `004ae6a0` gives and, for kind `012681c0` with `00c8d0d0` set, every entry of the
/// list `007d6bb0` gives; `01268100` with `012681b4` lists `004d8630`'s object, with `012681a8`
/// each of `004d8560`'s objects (`004d85a0`); `01268198` lists `004d8510`'s object and
/// `0126818c` lists `004d8530`'s object, each recursively under the new item.
pub fn fn_004d80b0(e: &mut Engine, this: Ptr, parent: u32, object: u32) {
    let base = this.addr();
    if object == 0 {
        return;
    }
    let list = new_string_list(e);
    // EBP-0x9C: the insert block; EBP-0x5C: the name buffer.
    let created = e.with_stack(0xa0, |e, frame| {
        let ebp = frame.addr() + 0xa0;
        let item = ebp - 0x9c;
        let buffer = ebp - 0x5c;
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item + 0x2c, object);
        e.mem.set_u32(item, parent);
        insert_named_object(e, base, item, buffer, object, 0, list)
    });
    let shape = e.call(IS_KIND_OF, &args![0x0126_8110u32, object]).u32();
    if shape != 0 {
        let inner = e.call(0x004a_e6a0, &args![shape]).u32();
        fn_004d80b0(e, this, created, inner);
        let compound = e.call(IS_KIND_OF, &args![0x0126_81c0u32, object]).u32();
        if compound != 0 && e.call(0x00c8_d0d0, &args![compound]).u32() != 0 {
            let mut node = e.call(0x007d_6bb0, &args![compound]).u32();
            while node != 0 {
                let entry = e.call(0x0068_15c0, &args![node]).u32();
                if e.call(FIRST_WORD, &args![entry]).u32() == 0 {
                    break;
                }
                let entry = e.call(0x0068_15c0, &args![node]).u32();
                let child = e.call(FIRST_WORD, &args![entry]).u32();
                fn_004d80b0(e, this, created, child);
                node = e.call(0x0072_6070, &args![node]).u32();
            }
        }
    } else {
        let wrapper = e.call(IS_KIND_OF, &args![0x0126_8100u32, object]).u32();
        if wrapper != 0 {
            let single = e.call(IS_KIND_OF, &args![0x0126_81b4u32, wrapper]).u32();
            if single != 0 {
                let inner = fn_004d8630(e, Ptr::new(single));
                fn_004d80b0(e, this, created, inner);
            } else {
                let several = e.call(IS_KIND_OF, &args![0x0126_81a8u32, wrapper]).u32();
                if several != 0 {
                    let count = fn_004d8560(e, Ptr::new(several));
                    if count != 0 {
                        let mut index = 0u32;
                        while index < count {
                            let inner = fn_004d85a0(e, Ptr::new(several), index);
                            fn_004d80b0(e, this, created, inner);
                            index += 1;
                        }
                    }
                }
            }
        } else {
            let first = e.call(IS_KIND_OF, &args![0x0126_8198u32, object]).u32();
            if first != 0 {
                let inner = fn_004d8510(e, Ptr::new(first));
                fn_004d80b0(e, this, created, inner);
            } else {
                let last = e.call(IS_KIND_OF, &args![0x0126_818cu32, object]).u32();
                if last != 0 {
                    let inner = fn_004d8530(e, Ptr::new(last));
                    fn_004d80b0(e, this, created, inner);
                }
            }
        }
    }
    delete_string_list(e, list);
}

// Translated from 004d8510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00c8aa00` with 0x42 on `this`, and its value.
pub fn fn_004d8510(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x00c8_aa00, &args![this, 0x42u32]).u32()
}

// Translated from 004d8530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004b5a80` (cdecl) on what vtable slot 0xD0 gives, and its value.
pub fn fn_004d8530(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e.vcall(this.addr(), 0xd0, &args![]).u32();
    e.call(0x004b_5a80, &args![inner]).u32()
}

// Translated from 004d8560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The count vtable slot 0x4 of the part at +0x10 of `004ae750`'s object gives, or 0 when
/// `004ae750` gives null.
pub fn fn_004d8560(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e.call(0x004a_e750, &args![this]).u32();
    if inner == 0 {
        0
    } else {
        e.vcall(inner + 0x10, 4, &args![]).u32()
    }
}

// Translated from 004d85a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004a3a40` (cdecl) on entry `index` (`004d85d0`, `RET 4`), and its value.
pub fn fn_004d85a0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let entry = fn_004d85d0(e, this, index);
    e.call(0x004a_3a40, &args![entry]).u32()
}

// Translated from 004d85d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` (`004d8610`) of `004ae750`'s object (`RET 4`), or 0 when that is null.
pub fn fn_004d85d0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let inner = e.call(0x004a_e750, &args![this]).u32();
    if inner == 0 {
        0
    } else {
        fn_004d8610(e, Ptr::new(inner), index)
    }
}

// Translated from 004d8610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at the address `0062e150` gives for `index` in the array at +0x18 (`RET 4`).
pub fn fn_004d8610(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let slot = e.call(0x0062_e150, &args![this.addr() + 0x18, index]).u32();
    e.mem.u32(slot)
}

// Translated from 004d8630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004a3a40` (cdecl) on `004d8670` of `004ae750`'s object (0 when that is null), and its
/// value.
pub fn fn_004d8630(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e.call(0x004a_e750, &args![this]).u32();
    let entry = if inner == 0 {
        0
    } else {
        fn_004d8670(e, Ptr::new(inner))
    };
    e.call(0x004a_3a40, &args![entry]).u32()
}

// Translated from 004d8670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00726070` on the part at +0x10, and its value.
pub fn fn_004d8670(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0072_6070, &args![this.addr() + 0x10]).u32()
}

// Translated from 004d8690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lists a controller manager's sequences (`RET 8`; nothing for null): an "Object Palette"
/// item (image 5) with `004d8a40` when `00537bd0` gives a palette, then "Active Sequences"
/// and "Inactive Sequences" items (the second keeps the first's images) with the sequences
/// (`00495d00`/`00495d20`) whose state (`008041a0`) is non-zero or zero, each through
/// `004d8b90`.
pub fn fn_004d8690(e: &mut Engine, this: Ptr, parent: u32, manager: u32) {
    let base = this.addr();
    if manager == 0 {
        return;
    }
    // EBP-0x48: the insert block.
    e.with_stack(0x30, |e, item| {
        let item = item.addr();
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item + 0x2c, manager);
        if e.call(0x0053_7bd0, &args![manager]).u32() != 0 {
            let palette_item = insert_group(e, base, item, parent, 0x0102_0eec);
            let palette = e.call(0x0053_7bd0, &args![manager]).u32();
            fn_004d8a40(e, this, palette_item, palette);
        }
        let active = insert_group(e, base, item, parent, 0x0102_0ed8);
        let mut index = 0u32;
        while index < e.call(0x0049_5d00, &args![manager]).u32() {
            let sequence = e.call(0x0049_5d20, &args![manager, index]).u32();
            if sequence != 0 && e.call(0x0080_41a0, &args![sequence]).u32() != 0 {
                fn_004d8b90(e, this, active, sequence);
            }
            index += 1;
        }
        let shown = e.call(ITEM_TEXT, &args![0x0102_0ec4u32, 0u32]).u32();
        e.mem.set_u32(item + 0x18, shown);
        let inactive = insert_item(e, base, item);
        let mut index = 0u32;
        while index < e.call(0x0049_5d00, &args![manager]).u32() {
            let sequence = e.call(0x0049_5d20, &args![manager, index]).u32();
            if sequence != 0 && e.call(0x0080_41a0, &args![sequence]).u32() == 0 {
                fn_004d8b90(e, this, inactive, sequence);
            }
            index += 1;
        }
    });
}

// Translated from 004d8830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d8830 - the security cookie check (00ec408c) is not translated.
/// Lists an interpolator controller's interpolators (`RET 8`; nothing for null): a
/// "NiBlendTransformInterps" item (image 5) under `parent`, then for each index below
/// `004d8a00` with an entry (`004d8a20`) and an interpolator (`004b5b00`) an item
/// `"Interp: %d, Target:%s"` (the name `00413f40` of the entry) under it with the
/// interpolator's strings (`004d8a40`).
pub fn fn_004d8830(e: &mut Engine, this: Ptr, parent: u32, controller: u32) {
    let base = this.addr();
    if controller == 0 {
        return;
    }
    // EBP-0x150: the insert block; EBP-0x108: the text buffer.
    e.with_stack(0x154, |e, frame| {
        let ebp = frame.addr() + 0x154;
        let item = ebp - 0x150;
        let text = ebp - 0x108;
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item + 0x2c, controller);
        let group = insert_group(e, base, item, parent, 0x0102_0f14);
        e.mem.set_u32(item, group);
        let mut index = 0u16;
        while index < fn_004d8a00(e, Ptr::new(controller)) {
            let entry = fn_004d8a20(e, Ptr::new(controller), index);
            if entry != 0 {
                let interpolator = e.call(0x004b_5b00, &args![controller, index]).u32();
                if interpolator != 0 {
                    e.mem.set_u32(item + 0x2c, interpolator);
                    let named = e.call(0x0041_3f40, &args![entry]).u32();
                    let name = e.mem.u32(named);
                    e.call(
                        SPRINTF_S,
                        &args![text, 0x100u32, 0x0102_0efcu32, u32::from(index), name],
                    );
                    let shown = e.call(ITEM_TEXT, &args![text, 0u32]).u32();
                    e.mem.set_u32(item + 0x18, shown);
                    let created = insert_item(e, base, item);
                    fn_004d8a40(e, this, created, interpolator);
                }
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 004d8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of the word array at +0x38 (`RET 4`, the low 16 bits of the word read).
pub fn fn_004d8a20(e: &mut Engine, this: Ptr, index: u16) -> u32 {
    let array = e.mem.u32(this.addr() + 0x38);
    e.mem.u32(array.wrapping_add(u32::from(index) * 4))
}

// Translated from 004d8a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d8a40 - the exception-unwinding frame (FS:[0]) is not translated.
/// Lists an object's strings (`RET 8`; nothing for null): vtable slot 0x60 into a new string
/// list, each string an item (image 6) under `parent`, then the list deleted.
pub fn fn_004d8a40(e: &mut Engine, this: Ptr, parent: u32, object: u32) {
    let base = this.addr();
    if object == 0 {
        return;
    }
    let list = new_string_list(e);
    // EBP-0x54: the insert block.
    e.with_stack(0x30, |e, item| {
        let item = item.addr();
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item + 0x2c, object);
        e.mem.set_u32(item, parent);
        e.vcall(object, 0x60, &args![list]);
        insert_list_strings(e, base, item, list);
    });
    delete_string_list(e, list);
}

// Translated from 004d8b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d8b90 - the exception-unwinding frame (FS:[0]) and the security cookie check (00ec408c) are not translated.
/// Lists an animation sequence (`RET 8`; nothing for null): an item `"%s: %s, Count: %d"`
/// (its name `00413f40`, its state `008041a0` as INACTIVE, ANIMATING, EASEIN, EASEOUT,
/// TRANSSOURCE, TRANSDEST or MORPHSOURCE, a null string above 6, the count `0084e3a0`), with
/// `", Blend: %0.2f"` or `", BlendIn: %0.2f, BlendOut: %0.2f"` (the frame counts over the
/// double at `0101db88`) when the sequence casts (`__RTDynamicCast` to `011878c8`) to one
/// whose `0048f7f0` part has a blend (`00759d80`) or blend-in/out (`004954e0`, `00495520`)
/// byte, under `parent`, with its strings (vtable slot 0x60) under it.
pub fn fn_004d8b90(e: &mut Engine, this: Ptr, parent: u32, sequence: u32) {
    let base = this.addr();
    if sequence == 0 {
        return;
    }
    let list = new_string_list(e);
    // EBP-0x16C: the insert block; EBP-0x124: the text buffer.
    e.with_stack(0x170, |e, frame| {
        let ebp = frame.addr() + 0x170;
        let item = ebp - 0x16c;
        let text = ebp - 0x124;
        e.mem.set_u32(item + 0x4, TVI_LAST);
        e.mem.set_u32(item + 0x8, 0x27);
        e.mem.set_u32(item + 0x2c, sequence);
        // The jump table at 004d8f7c: states 0..6 to the seven names.
        let state = match e.call(0x0080_41a0, &args![sequence]).u32() {
            0 => 0x0102_0fd4,
            1 => 0x0102_0fc8,
            2 => 0x0102_0fc0,
            3 => 0x0102_0fb8,
            4 => 0x0102_0fac,
            5 => 0x0102_0fa0,
            6 => 0x0102_0f94,
            _ => 0u32,
        };
        let cast = e
            .call(
                0x00ec_43fb,
                &args![sequence, 0u32, 0x0118_78ecu32, 0x0118_78c8u32, 0u32],
            )
            .u32();
        let mut blend = 0u8;
        let mut blend_in = 0u8;
        let mut blend_out = 0u8;
        if cast != 0 {
            let part = e.call(0x0048_f7f0, &args![cast]).u32();
            blend = e.call(0x0075_9d80, &args![part]).u8();
            let part = e.call(0x0048_f7f0, &args![cast]).u32();
            blend_in = e.call(0x0049_54e0, &args![part]).u8();
            let part = e.call(0x0048_f7f0, &args![cast]).u32();
            blend_out = e.call(0x0049_5520, &args![part]).u8();
        }
        let frames = e.global::<f64>(0x0101_db88);
        if blend != 0 {
            let seconds = x87_div(f64::from(blend), frames);
            let count = e.call(0x0084_e3a0, &args![sequence]).u32();
            let named = e.call(0x0041_3f40, &args![sequence]).u32();
            let name = e.mem.u32(named);
            e.call(
                SPRINTF_S,
                &args![text, 0x100u32, 0x0102_0f74u32, name, state, count, seconds],
            );
        } else if blend_in != 0 || blend_out != 0 {
            let out_seconds = x87_div(f64::from(blend_out), frames);
            let in_seconds = x87_div(f64::from(blend_in), frames);
            let count = e.call(0x0084_e3a0, &args![sequence]).u32();
            let named = e.call(0x0041_3f40, &args![sequence]).u32();
            let name = e.mem.u32(named);
            e.call(
                SPRINTF_S,
                &args![
                    text,
                    0x100u32,
                    0x0102_0f40u32,
                    name,
                    state,
                    count,
                    in_seconds,
                    out_seconds
                ],
            );
        } else {
            let count = e.call(0x0084_e3a0, &args![sequence]).u32();
            let named = e.call(0x0041_3f40, &args![sequence]).u32();
            let name = e.mem.u32(named);
            e.call(
                SPRINTF_S,
                &args![text, 0x100u32, 0x0102_0f2cu32, name, state, count],
            );
        }
        let shown = e.call(ITEM_TEXT, &args![text, 0u32]).u32();
        e.mem.set_u32(item + 0x18, shown);
        e.mem.set_u32(item + 0x20, 5);
        e.mem.set_u32(item + 0x24, 5);
        e.mem.set_u32(item, parent);
        let created = insert_item(e, base, item);
        e.mem.set_u32(item, created);
        e.vcall(sequence, 0x60, &args![list]);
        insert_list_strings(e, base, item, list);
    });
    delete_string_list(e, list);
}

// Translated from 004d8fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object map's constructor (`RET 4`): the base `004d9000` with `size` buckets, then
/// the vtable `01020fe4`. Returns `this`.
pub fn fn_004d8fa0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    fn_004d9000(e, this, size);
    e.mem.set_u32(this.addr(), 0x0102_0fe4);
    this
}

// Translated from 004d9000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map base's constructor (`RET 4`): the vtable `01021004`, `size` at +0x4, 0 at +0xC and
/// a cleared block of `size` words (`00aa1070`, `memset` `00403d30`) at +0x8. Returns `this`.
pub fn fn_004d9000(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_1004);
    e.mem.set_u32(base + 0x4, size);
    e.mem.set_u32(base + 0xc, 0);
    let bytes = e.mem.u32(base + 0x4) << 2;
    let block = e.call(0x00aa_1070, &args![bytes]).u32();
    e.mem.set_u32(base + 0x8, block);
    let bytes = e.mem.u32(base + 0x4) << 2;
    let block = e.mem.u32(base + 0x8);
    e.call(0x0040_3d30, &args![block, 0u32, bytes]);
    this
}

// Translated from 004d9070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d9070 - the exception-unwinding frame (FS:[0]) is not translated.
/// The object map's destructor body: the vtable `01020fe4`, empties the map (`00438af0`) and
/// runs the base destructor `004d90d0`.
pub fn fn_004d9070(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_0fe4);
    e.call(0x0043_8af0, &args![this]);
    fn_004d90d0(e, this);
}

// Translated from 004d90d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map base's destructor body: the vtable `01021004`, empties the map (`00438af0`) and
/// frees the bucket block at +0x8 (`00aa10f0`).
pub fn fn_004d90d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_1004);
    e.call(0x0043_8af0, &args![this]);
    let block = e.mem.u32(this.addr() + 0x8);
    e.call(0x00aa_10f0, &args![block]);
}

// Translated from 004d9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string list's constructor (`RET 8`): the base `004d9210` with both words, then the
/// vtable `01021024`. Returns `this`.
pub fn fn_004d9100(e: &mut Engine, this: Ptr, capacity: u32, growth: u32) -> Ptr {
    fn_004d9210(e, this, capacity as u16, growth as u16);
    e.mem.set_u32(this.addr(), 0x0102_1024);
    this
}

// Translated from 004d91b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The array base's destructor body: the vtable `0102102c`, then frees the block at +0x4
/// (`004ede70`, cdecl).
pub fn fn_004d91b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_102c);
    let block = e.mem.u32(this.addr() + 0x4);
    e.call(0x004e_de70, &args![block]);
}

// Translated from 004d9210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The array base's constructor (`RET 8`, the low 16 bits of each word read): the vtable
/// `0102102c`, the capacity at +0x8, the growth at +0xE, 0 at +0xA and +0xC, and a block of
/// `capacity` words (`0096afc0`) at +0x4, or null for capacity 0. Returns `this`.
pub fn fn_004d9210(e: &mut Engine, this: Ptr, capacity: u16, growth: u16) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_102c);
    e.mem.set_u16(base + 0x8, capacity);
    e.mem.set_u16(base + 0xe, growth);
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    let capacity = e.mem.u16(base + 0x8);
    if capacity == 0 {
        e.mem.set_u32(base + 0x4, 0);
    } else {
        let block = e.call(0x0096_afc0, &args![u32::from(capacity)]).u32();
        e.mem.set_u32(base + 0x4, block);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004d_9190, fn_004d9190(Ptr)),
        entry!(0x004d_8fd0, fn_004d8fd0(Ptr, u32) -> Ptr),
        entry!(0x004d_9130, fn_004d9130(Ptr, u32) -> Ptr),
        entry!(0x004d_9160, fn_004d9160(Ptr, u32) -> Ptr),
        entry!(0x004d_91e0, fn_004d91e0(Ptr, u32) -> Ptr),
        entry!(0x004d_8a00, fn_004d8a00(Ptr) -> u16),
        entry!(0x004d_61b0, fn_004d61b0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x004d_62a0, fn_004d62a0(Ptr)),
        entry!(0x004d_6340, fn_004d6340(Ptr, u32, u32, u32, u32)),
        entry!(0x004d_6490, fn_004d6490(Ptr)),
        entry!(0x004d_64c0, fn_004d64c0(Ptr, u32, u32)),
        entry!(0x004d_65a0, fn_004d65a0(Ptr, u32)),
        entry!(0x004d_66d0, fn_004d66d0(Ptr, u32, Ptr)),
        entry!(0x004d_7440, fn_004d7440(Ptr)),
        entry!(0x004d_7470, fn_004d7470(Ptr)),
        entry!(0x004d_7490, fn_004d7490(Ptr)),
        entry!(0x004d_74c0, fn_004d74c0(u32, u32, u32)),
        entry!(0x004d_75b0, fn_004d75b0(u32, u32, u32, u32) -> u32),
        entry!(0x004d_7890, fn_004d7890(u32, u32, u32, u32) -> u32),
        entry!(0x004d_78c0, fn_004d78c0(u32, u32, u32) -> u32),
        entry!(0x004d_7ab0, fn_004d7ab0(Ptr, u32, u32)),
        entry!(0x004d_7d50, fn_004d7d50(Ptr) -> u32),
        entry!(0x004d_7d70, fn_004d7d70(Ptr, u32) -> u32),
        entry!(0x004d_7df0, fn_004d7df0(Ptr, u32, Ptr)),
        entry!(0x004d_80b0, fn_004d80b0(Ptr, u32, u32)),
        entry!(0x004d_8510, fn_004d8510(Ptr) -> u32),
        entry!(0x004d_8530, fn_004d8530(Ptr) -> u32),
        entry!(0x004d_8560, fn_004d8560(Ptr) -> u32),
        entry!(0x004d_85a0, fn_004d85a0(Ptr, u32) -> u32),
        entry!(0x004d_85d0, fn_004d85d0(Ptr, u32) -> u32),
        entry!(0x004d_8610, fn_004d8610(Ptr, u32) -> u32),
        entry!(0x004d_8630, fn_004d8630(Ptr) -> u32),
        entry!(0x004d_8670, fn_004d8670(Ptr) -> u32),
        entry!(0x004d_8690, fn_004d8690(Ptr, u32, u32)),
        entry!(0x004d_8830, fn_004d8830(Ptr, u32, u32)),
        entry!(0x004d_8a20, fn_004d8a20(Ptr, u16) -> u32),
        entry!(0x004d_8a40, fn_004d8a40(Ptr, u32, u32)),
        entry!(0x004d_8b90, fn_004d8b90(Ptr, u32, u32)),
        entry!(0x004d_8fa0, fn_004d8fa0(Ptr, u32) -> Ptr),
        entry!(0x004d_9000, fn_004d9000(Ptr, u32) -> Ptr),
        entry!(0x004d_9070, fn_004d9070(Ptr)),
        entry!(0x004d_90d0, fn_004d90d0(Ptr)),
        entry!(0x004d_9100, fn_004d9100(Ptr, u32, u32) -> Ptr),
        entry!(0x004d_91b0, fn_004d91b0(Ptr)),
        entry!(0x004d_9210, fn_004d9210(Ptr, u16, u16) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_004d9190_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x004d_91b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004d_9190, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x004d_91b0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004d8fd0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004d_9070, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004d_8fd0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004d_8fd0)
                .collect();
            let mut expected = vec![(0x004d_9070, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004d9130_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004d_90d0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004d_9130, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004d_9130)
                .collect();
            let mut expected = vec![(0x004d_90d0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004d9160_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004d_9190, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004d_9160, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004d_9160)
                .collect();
            let mut expected = vec![(0x004d_9190, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004d91e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004d_91b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004d_91e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004d_91e0)
                .collect();
            let mut expected = vec![(0x004d_91b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    #[test]
    fn fn_004d8a00_reads_the_word_at_0x3c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, 0xffff_8123);
        assert_eq!(e.call(0x004d_8a00, &args![this]).u16(), 0x8123);
    }
}

#[cfg(test)]
mod n039_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An inserted tree item: parent, text, image, parameter.
    type Inserted = Rc<RefCell<Vec<(u32, u32, u32, u32)>>>;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// `SendMessageA` that answers 0x7000 + n for the n-th insert (recording it) and `answer`
    /// for every other message.
    fn tree_double(e: &mut Engine, answer: u32) -> Inserted {
        let inserted: Inserted = Rc::default();
        let record = inserted.clone();
        e.register_double(SEND_MESSAGE_A, move |e, a| {
            if a[1] == TVM_INSERTITEMA {
                let block = a[3];
                let entry = (
                    e.mem.u32(block),
                    e.mem.u32(block + 0x18),
                    e.mem.u32(block + 0x20),
                    e.mem.u32(block + 0x2c),
                );
                record.borrow_mut().push(entry);
                return (0x7000 + record.borrow().len() as u32).into_ret();
            }
            answer.into_ret()
        });
        inserted
    }

    /// The item text double (`00464f30` answers its first word) and `operator new`.
    fn text_and_new(e: &mut Engine) {
        e.register(ITEM_TEXT, |_, a| a[0].into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
    }

    /// An object with a vtable whose slots answer 0 except those given.
    fn object_with(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let mut table = vec![0x00e0_0000u32; 0x40];
        for &(slot, target) in slots {
            table[(slot / 4) as usize] = target;
        }
        e.put_vtable(vtable, &table);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The string-list helpers used by the listers, answering an empty list.
    fn empty_lists(e: &mut Engine) {
        e.put_vtable(0x0102_1024, &[0x00e5_0000]);
        e.register(0x00e5_0000, |_, _| Ret::default());
        zero(
            e,
            &[
                0x00e0_0000,
                STRING_COUNT,
                STRING_CLEAR,
                0x0096_afc0,
                FIRST_WORD,
                COPY_NAME,
            ],
        );
    }

    #[test]
    fn fn_004d61b0_builds_the_window_object() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        e.register(0x0044_a670, |_, _| 5u32.into_ret());
        e.register(0x00aa_1070, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        zero(
            &mut e,
            &[
                0x0040_3d30,
                0x0040_46f0,
                INIT_COMMON_CONTROLS_EX,
                LOAD_ICON_A,
                LOAD_CURSOR_A,
                GET_STOCK_OBJECT,
                REGISTER_CLASS_A,
                CREATE_WINDOW_EX_A,
                GET_CLIENT_RECT,
                IMAGE_LIST_LOAD_IMAGE_A,
            ],
        );
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        let result = e.call(
            0x004d_61b0,
            &args![
                this,
                0x11u32,
                0x22u32,
                0u32,
                0x0600_0000u32,
                1u32,
                2u32,
                3u32,
                4u32
            ],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0x11);
        assert_eq!(e.mem.u32(this + 4), 0x22);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0x0102_0fe4);
        assert_eq!(e.mem.u32(this + 0x1c), 0x25);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![6]]);
        let copy = e.mem.u32(this + 0x2c);
        assert_eq!(calls_to(&e, 0x0040_46f0), vec![vec![copy, 0x0600_0000]]);
        assert_eq!(calls_to(&e, CREATE_WINDOW_EX_A).len(), 2);
        assert_eq!(calls_to(&e, SEND_MESSAGE_A).len(), 0);
    }

    #[test]
    fn fn_004d62a0_frees_and_destroys_in_order() {
        let mut e = Engine::new();
        zero(
            &mut e,
            &[
                0x0040_1030,
                SEND_MESSAGE_A,
                IMAGE_LIST_DESTROY,
                DESTROY_WINDOW,
                0x0043_8af0,
                0x00aa_10f0,
            ],
        );
        e.map(0x011f_36f4, 8);
        e.set_global::<u32>(LAST_MANAGER, 5);
        e.set_global::<u32>(PREVIOUS_MANAGER, 6);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x2c, 0x4444);
        e.mem.set_u32(this + 0x20, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x004d_62a0, &args![this]);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x004d_62a0,
                0x0040_1030,
                SEND_MESSAGE_A,
                IMAGE_LIST_DESTROY,
                DESTROY_WINDOW,
                DESTROY_WINDOW,
                0x0043_8af0,
                0x0043_8af0,
                0x00aa_10f0
            ]
        );
        assert_eq!(e.global::<u32>(LAST_MANAGER), 0);
        assert_eq!(e.global::<u32>(PREVIOUS_MANAGER), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0x0102_1004);
    }

    #[test]
    fn fn_004d6340_sizes_the_tree_to_the_client_rectangle() {
        let mut e = Engine::new();
        e.register(ITEM_TEXT, |_, a| a[0].into_ret());
        zero(
            &mut e,
            &[
                INIT_COMMON_CONTROLS_EX,
                LOAD_ICON_A,
                LOAD_CURSOR_A,
                GET_STOCK_OBJECT,
                REGISTER_CLASS_A,
            ],
        );
        let mut windows = 0x900u32;
        e.register_double(CREATE_WINDOW_EX_A, move |_, _| {
            windows += 1;
            windows.into_ret()
        });
        e.register(GET_CLIENT_RECT, |e, a| {
            for (i, v) in [10u32, 20, 110, 220].iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, 0x77);
        e.mem.set_u32(this + 0x2c, 0x0600_0000);
        e.call_log = Some(vec![]);
        e.call(0x004d_6340, &args![this, 1u32, 2u32, 3u32, 4u32]);
        let windows = calls_to(&e, CREATE_WINDOW_EX_A);
        assert_eq!(
            windows[0],
            vec![
                0,
                0x0118_9338,
                0x0600_0000,
                0x00ce_0200,
                1,
                2,
                3,
                4,
                0,
                0,
                0x77,
                0
            ]
        );
        assert_eq!(
            windows[1],
            vec![
                0,
                0x0102_0e24,
                0x0101_1584,
                0x5001_0007,
                0,
                0,
                101,
                201,
                0x901,
                0,
                0x77,
                0
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 0x901);
        assert_eq!(e.mem.u32(this + 0xc), 0x902);
    }

    #[test]
    fn fn_004d6490_loads_the_image_list() {
        let mut e = Engine::new();
        e.register(IMAGE_LIST_LOAD_IMAGE_A, |_, _| 0x31u32.into_ret());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x004d_6490, &args![this]);
        assert_eq!(
            calls_to(&e, IMAGE_LIST_LOAD_IMAGE_A),
            vec![vec![0x77, 0xb4, 0x10, 1, 0xff00_0000, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0x31);
    }

    /// A leaf scene object for `004d66d0`: no name, no controllers, no properties, no
    /// children, scale 1.0, not culled.
    fn leaf_object(e: &mut Engine) -> u32 {
        empty_lists(e);
        let position = e.mem.alloc(12);
        e.register_double(0x0043_c490, move |_, _| position.into_ret());
        let named = e.mem.alloc(4);
        e.mem.set_u32(named, 0x0600_0100);
        e.register_double(0x0041_3f40, move |_, _| named.into_ret());
        e.map(0x0101_2070, 8);
        e.set_global::<f64>(0x0101_2070, 1.0);
        e.register(0x009c_dae0, |_, _| 1.0f32.into_ret());
        zero(
            e,
            &[
                SPRINTF_S,
                STRCAT_S,
                0x0045_6610,
                0x0043_b300,
                0x0084_4700,
                0x0043_0830,
                0x0043_b230,
                IS_KIND_OF,
                0x0068_38b0,
            ],
        );
        e.register(0x0076_b610, |_, _| true.into_ret());
        object_with(e, 0x00d0_0000, &[])
    }

    #[test]
    fn fn_004d64c0_inserts_the_root_and_shows_the_window() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        let inserted = tree_double(&mut e, 0);
        zero(&mut e, &[0x0043_8af0, SHOW_WINDOW, UPDATE_WINDOW]);
        let root = leaf_object(&mut e);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x2c, 0x0600_0200);
        e.call_log = Some(vec![]);
        e.call(0x004d_64c0, &args![this, root, 0u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0x7001);
        assert_eq!(inserted.borrow()[0], (TVI_ROOT, 0x0600_0200, 0, root));
        assert_eq!(inserted.borrow()[1].0, 0x7001);
        assert_eq!(calls_to(&e, SHOW_WINDOW).len(), 1);
        assert_eq!(calls_to(&e, UPDATE_WINDOW).len(), 1);
    }

    #[test]
    fn fn_004d65a0_expands_the_parents_of_the_found_item() {
        let mut e = Engine::new();
        e.register(0x0085_3130, |e, a| {
            e.mem.set_u32(a[2], 0x55);
            true.into_ret()
        });
        // The local list: one entry (the found item), then the end.
        e.register(0x0096_a2d0, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
        e.register(0x005a_e3d0, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        zero(&mut e, &[0x0072_6070, 0x0046_ffb0]);
        e.register(SEND_MESSAGE_A, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x004d_65a0, &args![this, 0x1234u32]);
        assert_eq!(
            calls_to(&e, SEND_MESSAGE_A),
            vec![
                vec![0x99, TVM_GETNEXTITEM, 3, 0x55],
                vec![0x99, TVM_EXPAND, 2, 0x55]
            ]
        );
        assert_eq!(calls_to(&e, 0x0046_ffb0).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x004d_65a0, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_004d66d0_lists_a_leaf_object() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        let inserted = tree_double(&mut e, 0);
        let object = leaf_object(&mut e);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004d_66d0, &args![this, 0x10u32, object]);
        let text = calls_to(&e, ITEM_TEXT)[0][0];
        assert_eq!(
            *inserted.borrow(),
            vec![(0x10, text, 4, object), (0x7001, 0x0102_0e5c, 5, 0)]
        );
        assert_eq!(
            calls_to(&e, 0x0084_4700),
            vec![vec![this + 0x18, object, 0x7001]]
        );
        assert_eq!(calls_to(&e, SPRINTF_S)[0][2], 0x0102_0e7c);
        assert!(calls_to(&e, STRCAT_S).is_empty());
    }

    #[test]
    fn fn_004d7440_destroys_both_windows() {
        let mut e = Engine::new();
        zero(&mut e, &[DESTROY_WINDOW]);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, 0x81);
        e.mem.set_u32(this + 0xc, 0x82);
        e.call_log = Some(vec![]);
        e.call(0x004d_7440, &args![this]);
        assert_eq!(calls_to(&e, DESTROY_WINDOW), vec![vec![0x82], vec![0x81]]);
    }

    #[test]
    fn fn_004d7470_destroys_the_image_list() {
        let mut e = Engine::new();
        zero(&mut e, &[IMAGE_LIST_DESTROY]);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x14, 0x83);
        e.call_log = Some(vec![]);
        e.call(0x004d_7470, &args![this]);
        assert_eq!(calls_to(&e, IMAGE_LIST_DESTROY), vec![vec![0x83]]);
    }

    #[test]
    fn fn_004d7490_deletes_every_item() {
        let mut e = Engine::new();
        zero(&mut e, &[SEND_MESSAGE_A]);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, 0x84);
        e.call_log = Some(vec![]);
        e.call(0x004d_7490, &args![this]);
        assert_eq!(
            calls_to(&e, SEND_MESSAGE_A),
            vec![vec![0x84, TVM_DELETEITEM, 0, TVI_ROOT]]
        );
    }

    #[test]
    fn fn_004d74c0_expands_accepted_children() {
        let mut e = Engine::new();
        zero(&mut e, &[0x0040_3d30]);
        let parameter = object_with(&mut e, 0x00d1_0000, &[(0xc, 0x00e1_000c)]);
        e.register(0x00e1_000c, |_, _| 0x4444u32.into_ret());
        e.register(0x0045_3470, |_, _| 1u32.into_ret());
        // Item 0x51 is the only child of 0x50; it has no children.
        e.register_double(SEND_MESSAGE_A, move |e, a| match (a[1], a[2], a[3]) {
            (TVM_GETNEXTITEM, 4, 0x50) => 0x51u32.into_ret(),
            (TVM_GETITEMA, _, block) => {
                e.mem.set_u32(block + 0x24, parameter);
                1u32.into_ret()
            }
            _ => Ret::default(),
        });
        e.call_log = Some(vec![]);
        e.call(0x004d_74c0, &args![0x90u32, 0x50u32, 2u32]);
        let messages: Vec<Vec<u32>> = calls_to(&e, SEND_MESSAGE_A)
            .into_iter()
            .filter(|m| m[1] != TVM_GETITEMA)
            .collect();
        assert_eq!(
            messages,
            vec![
                vec![0x90, TVM_GETNEXTITEM, 4, 0x50],
                vec![0x90, TVM_EXPAND, 2, 0x51],
                vec![0x90, TVM_GETNEXTITEM, 4, 0x51],
                vec![0x90, TVM_GETNEXTITEM, 1, 0x51]
            ]
        );
    }

    #[test]
    fn fn_004d75b0_handles_size_selection_and_default() {
        let mut e = Engine::new();
        zero(&mut e, &[SET_WINDOW_POS, SHOW_WINDOW, SHOW_CURSOR]);
        e.register(GET_WINDOW, |_, _| 0x66u32.into_ret());
        e.register(DEF_WINDOW_PROC_A, |_, _| 0x99u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004d_75b0, &args![0x10u32, 5u32, 0u32, 0x0020_0030u32])
                .u32(),
            0x99
        );
        assert_eq!(
            calls_to(&e, SET_WINDOW_POS),
            vec![vec![0x66, 0, 0, 0, 0x30, 0x20, 2]]
        );
        assert_eq!(
            e.call(0x004d_75b0, &args![0x10u32, 6u32, 0x0005_0002u32, 0u32])
                .u32(),
            0x99
        );
        assert_eq!(calls_to(&e, SHOW_CURSOR), vec![vec![1]]);
        // TVN_SELCHANGEDA with a callback: old parameter with 1, new with 0; answers 1.
        e.map(TREE_CALLBACK, 4);
        e.set_global::<u32>(TREE_CALLBACK, 0x00e2_0000);
        e.register(0x00e2_0000, |_, _| Ret::default());
        let header = e.mem.alloc(0x60);
        e.mem.set_u32(header + 8, 0xffff_fe6e);
        e.mem.set_u32(header + 0x34, 0xa1);
        e.mem.set_u32(header + 0x5c, 0xa2);
        assert_eq!(
            e.call(0x004d_75b0, &args![0x10u32, 0x4eu32, 0u32, header])
                .u32(),
            1
        );
        assert_eq!(
            calls_to(&e, 0x00e2_0000),
            vec![vec![0xa1, 1, 0, 0], vec![0xa2, 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, DEF_WINDOW_PROC_A).len(), 2);
    }

    #[test]
    fn fn_004d7890_calls_the_installed_callback() {
        let mut e = Engine::new();
        e.map(TREE_CALLBACK, 4);
        assert_eq!(e.call(0x004d_7890, &args![1u32, 2u32, 3u32, 4u32]).u32(), 0);
        e.set_global::<u32>(TREE_CALLBACK, 0x00e2_0000);
        e.register(0x00e2_0000, |_, a| (a[0] + a[1] + a[2] + a[3]).into_ret());
        assert_eq!(
            e.call(0x004d_7890, &args![1u32, 2u32, 3u32, 4u32]).u32(),
            10
        );
    }

    #[test]
    fn fn_004d78c0_answers_by_event_and_key() {
        let mut e = Engine::new();
        zero(&mut e, &[0x0045_0f90]);
        assert_eq!(e.call(0x004d_78c0, &args![0u32, 0u32, 0u32]).u32(), 1);
        assert_eq!(e.call(0x004d_78c0, &args![0u32, 1u32, 0u32]).u32(), 1);
        assert_eq!(e.call(0x004d_78c0, &args![0u32, 3u32, 0x70u32]).u32(), 0);
        assert_eq!(e.call(0x004d_78c0, &args![0u32, 2u32, 0x70u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_78c0, &args![0x40u32, 3u32, 0x71u32]).u32(), 1);
        assert_eq!(e.call(0x004d_78c0, &args![0x40u32, 3u32, 0x73u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0045_0f90), vec![vec![0x40, 0]]);
        // Key 0x7A: the path is "c:\" + "NullObject" + ".nif".
        zero(
            &mut e,
            &[
                0x00a6_6150,
                0x00a6_6370,
                0x0041_3f40,
                0x0043_b1b0,
                0x0040_6d30,
                STRCAT_S,
                0x00a6_40b0,
                0x00a6_5300,
            ],
        );
        assert_eq!(e.call(0x004d_78c0, &args![0x40u32, 3u32, 0x7au32]).u32(), 1);
        let appended: Vec<u32> = calls_to(&e, STRCAT_S).iter().map(|c| c[2]).collect();
        assert_eq!(appended, vec![0x0102_0ea4, 0x0102_0e98]);
    }

    #[test]
    fn fn_004d7ab0_lists_modifiers_of_the_right_kind() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        empty_lists(&mut e);
        let inserted = tree_double(&mut e, 0);
        e.register(IS_KIND_OF, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004d_7ab0, &args![this, 0x10u32, 0x3000u32]);
        assert!(inserted.borrow().is_empty());
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        // Of the kind, no modifiers: only the group.
        e.register(IS_KIND_OF, |_, a| a[1].into_ret());
        e.register(0x0044_ddc0, |_, _| Ret::default());
        let owner = e.mem.alloc(0x100);
        e.call(0x004d_7ab0, &args![this, 0x10u32, owner]);
        assert_eq!(*inserted.borrow(), vec![(0x10, 0x0102_0eb0, 5, owner)]);
    }

    #[test]
    fn fn_004d7d50_counts_the_list() {
        let mut e = Engine::new();
        e.register(0x0044_ddc0, |_, a| (a[0] & 0xfff).into_ret());
        let this = e.mem.alloc(0x100);
        assert_eq!(
            e.call(0x004d_7d50, &args![this]).u32(),
            (this + 0xc8) & 0xfff
        );
    }

    #[test]
    fn fn_004d7d70_walks_to_the_index() {
        let mut e = Engine::new();
        e.register(0x0044_ddc0, |_, _| 3u32.into_ret());
        // Nodes 0x10, 0x20, 0x30 hold the values 0xa, 0xb, 0xc.
        e.register(FIRST_WORD, |_, a| match a[0] {
            0x10 => 0xau32.into_ret(),
            0x20 => 0xbu32.into_ret(),
            0x30 => 0xcu32.into_ret(),
            _ => 0x10u32.into_ret(),
        });
        e.register(LIST_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = if node == 0x30 { 0 } else { node + 0x10 };
            e.mem.set_u32(a[1], next);
            node.into_ret()
        });
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x004d_7d70, &args![this, 1u32]).u32(), 0xb);
        assert_eq!(e.call(0x004d_7d70, &args![this, 2u32]).u32(), 0xc);
        assert_eq!(e.call(0x004d_7d70, &args![this, 3u32]).u32(), 0);
    }

    #[test]
    fn fn_004d7df0_lists_the_collision_object() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        empty_lists(&mut e);
        let inserted = tree_double(&mut e, 0);
        e.register(IS_KIND_OF, |_, a| a[1].into_ret());
        e.register(0x006f_a820, |_, _| Ret::default());
        let object = object_with(&mut e, 0x00d2_0000, &[]);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004d_7df0, &args![this, 0x10u32, object]);
        let buffer = calls_to(&e, COPY_NAME)[0][0];
        assert_eq!(
            *inserted.borrow(),
            vec![(0x10, 0x0102_0ebc, 5, object), (0x7001, buffer, 0, object)]
        );
        assert_eq!(calls_to(&e, 0x006f_a820), vec![vec![object]]);
        let slots = calls_to(&e, 0x00e0_0000);
        assert_eq!(slots.len(), 2);
        assert_eq!((slots[0].clone(), slots[1][0]), (vec![object], object));
    }

    #[test]
    fn fn_004d80b0_recurses_by_kind() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        empty_lists(&mut e);
        let inserted = tree_double(&mut e, 0);
        let first = object_with(&mut e, 0x00d3_0000, &[]);
        let second = e.mem.alloc(0x100);
        e.mem.set_u32(second, 0x00d3_0000);
        // `first` is of the kind 01268198, whose `00c8aa00` part is `second`.
        e.register_double(IS_KIND_OF, move |_, a| {
            if a[0] == 0x0126_8198 && a[1] == first {
                first.into_ret()
            } else {
                Ret::default()
            }
        });
        e.register_double(0x00c8_aa00, move |_, _| second.into_ret());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004d_80b0, &args![this, 0x10u32, first]);
        let parents: Vec<(u32, u32)> = inserted.borrow().iter().map(|i| (i.0, i.3)).collect();
        assert_eq!(parents, vec![(0x10, first), (0x7001, second)]);
        assert_eq!(calls_to(&e, 0x00c8_aa00), vec![vec![first, 0x42]]);
        e.call(0x004d_80b0, &args![this, 0x10u32, 0u32]);
        assert_eq!(inserted.borrow().len(), 2);
    }

    #[test]
    fn fn_004d8510_asks_for_part_0x42() {
        let mut e = Engine::new();
        e.register(0x00c8_aa00, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x004d_8510, &args![0x100u32]).u32(), 0x142);
    }

    #[test]
    fn fn_004d8530_converts_slot_0xd0() {
        let mut e = Engine::new();
        let object = object_with(&mut e, 0x00d4_0000, &[(0xd0, 0x00e3_0000)]);
        e.register(0x00e3_0000, |_, _| 0x77u32.into_ret());
        e.register(0x004b_5a80, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x004d_8530, &args![object]).u32(), 0x78);
    }

    #[test]
    fn fn_004d8560_counts_through_the_inner_object() {
        let mut e = Engine::new();
        e.register(0x004a_e750, |_, _| Ret::default());
        assert_eq!(e.call(0x004d_8560, &args![0x100u32]).u32(), 0);
        let inner = e.mem.alloc(0x20);
        e.put_vtable(0x00d5_0000, &[0, 0x00e4_0000]);
        e.mem.set_u32(inner + 0x10, 0x00d5_0000);
        e.register_double(0x004a_e750, move |_, _| inner.into_ret());
        e.register_double(0x00e4_0000, move |_, a| {
            assert_eq!(a[0], inner + 0x10);
            9u32.into_ret()
        });
        assert_eq!(e.call(0x004d_8560, &args![0x100u32]).u32(), 9);
    }

    #[test]
    fn fn_004d85a0_converts_the_entry() {
        let mut e = Engine::new();
        let inner = e.mem.alloc(0x20);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x55);
        e.register_double(0x004a_e750, move |_, _| inner.into_ret());
        e.register_double(0x0062_e150, move |_, _| slot.into_ret());
        e.register(0x004a_3a40, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_85a0, &args![0x100u32, 2u32]).u32(), 0x56);
        assert_eq!(calls_to(&e, 0x0062_e150), vec![vec![inner + 0x18, 2]]);
    }

    #[test]
    fn fn_004d85d0_answers_zero_without_an_inner_object() {
        let mut e = Engine::new();
        e.register(0x004a_e750, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_85d0, &args![0x100u32, 2u32]).u32(), 0);
        assert!(calls_to(&e, 0x0062_e150).is_empty());
    }

    #[test]
    fn fn_004d8610_reads_the_slot() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x66);
        e.register_double(0x0062_e150, move |_, _| slot.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_8610, &args![0x100u32, 3u32]).u32(), 0x66);
        assert_eq!(calls_to(&e, 0x0062_e150), vec![vec![0x118, 3]]);
    }

    #[test]
    fn fn_004d8630_converts_zero_without_an_inner_object() {
        let mut e = Engine::new();
        e.register(0x004a_e750, |_, _| Ret::default());
        e.register(0x004a_3a40, |_, a| (a[0] + 7).into_ret());
        assert_eq!(e.call(0x004d_8630, &args![0x100u32]).u32(), 7);
        e.register(0x004a_e750, |_, _| 0x200u32.into_ret());
        e.register(0x0072_6070, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x004d_8630, &args![0x100u32]).u32(), 0x217);
    }

    #[test]
    fn fn_004d8670_reads_the_part_at_0x10() {
        let mut e = Engine::new();
        e.register(0x0072_6070, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x004d_8670, &args![0x100u32]).u32(), 0x110);
    }

    #[test]
    fn fn_004d8690_lists_the_sequence_groups() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        let inserted = tree_double(&mut e, 0);
        zero(&mut e, &[0x0053_7bd0, 0x0049_5d00]);
        let this = e.mem.alloc(0x30);
        e.call(0x004d_8690, &args![this, 0x10u32, 0u32]);
        assert!(inserted.borrow().is_empty());
        e.call(0x004d_8690, &args![this, 0x10u32, 0x3000u32]);
        assert_eq!(
            *inserted.borrow(),
            vec![
                (0x10, 0x0102_0ed8, 5, 0x3000),
                (0x10, 0x0102_0ec4, 5, 0x3000)
            ]
        );
    }

    #[test]
    fn fn_004d8830_lists_the_interpolators() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        empty_lists(&mut e);
        let inserted = tree_double(&mut e, 0);
        zero(&mut e, &[SPRINTF_S]);
        let controller = e.mem.alloc(0x40);
        let entries = e.mem.alloc(8);
        e.mem.set_u32(controller + 0x38, entries);
        e.mem.set_u16(controller + 0x3c, 2);
        e.mem.set_u32(entries, 0);
        e.mem.set_u32(entries + 4, 0x5000);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x0600_0300);
        e.register_double(0x0041_3f40, move |_, _| name.into_ret());
        let interpolator = object_with(&mut e, 0x00d6_0000, &[]);
        e.register_double(0x004b_5b00, move |_, _| interpolator.into_ret());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004d_8830, &args![this, 0x10u32, controller]);
        let text = calls_to(&e, SPRINTF_S)[0][0];
        assert_eq!(
            calls_to(&e, SPRINTF_S)[0][1..],
            [0x100, 0x0102_0efc, 1, 0x0600_0300]
        );
        assert_eq!(
            *inserted.borrow(),
            vec![
                (0x10, 0x0102_0f14, 5, controller),
                (0x7001, text, 5, interpolator)
            ]
        );
    }

    #[test]
    fn fn_004d8a20_reads_the_entry() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let entries = e.mem.alloc(12);
        e.mem.set_u32(this + 0x38, entries);
        e.mem.set_u32(entries + 8, 0x99);
        assert_eq!(
            e.call(0x004d_8a20, &args![this, 0xffff_0002u32]).u32(),
            0x99
        );
    }

    #[test]
    fn fn_004d8a40_lists_strings_and_deletes_the_list() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        empty_lists(&mut e);
        let inserted = tree_double(&mut e, 0);
        let mut counts = vec![1u32, 0];
        e.register_double(STRING_COUNT, move |_, _| counts.remove(0).into_ret());
        let string = e.mem.alloc(4);
        e.mem.set_u32(string, 0x0600_0400);
        e.register_double(STRING_AT, move |_, _| string.into_ret());
        let object = object_with(&mut e, 0x00d7_0000, &[]);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004d_8a40, &args![this, 0x10u32, object]);
        assert_eq!(*inserted.borrow(), vec![(0x10, 0x0600_0400, 6, object)]);
        let list = calls_to(&e, STRING_CLEAR)[0][0];
        assert_eq!(e.mem.u32(list), 0x0102_1024);
        assert_eq!(calls_to(&e, 0x00e5_0000), vec![vec![list, 1]]);
    }

    #[test]
    fn fn_004d8b90_formats_the_blend() {
        let mut e = Engine::new();
        text_and_new(&mut e);
        empty_lists(&mut e);
        let _inserted = tree_double(&mut e, 0);
        zero(&mut e, &[SPRINTF_S, 0x0048_f7f0, 0x0049_54e0, 0x0049_5520]);
        e.register(0x0080_41a0, |_, _| 1u32.into_ret());
        e.register(0x0084_e3a0, |_, _| 4u32.into_ret());
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0075_9d80, |_, _| 15u8.into_ret());
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x0600_0500);
        e.register_double(0x0041_3f40, move |_, _| name.into_ret());
        e.map(0x0101_db88, 8);
        e.set_global::<f64>(0x0101_db88, 30.0);
        let sequence = object_with(&mut e, 0x00d8_0000, &[]);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004d_8b90, &args![this, 0x10u32, sequence]);
        let format = &calls_to(&e, SPRINTF_S)[0];
        let half = 0.5f64.to_bits();
        assert_eq!(
            format[1..],
            [
                0x100,
                0x0102_0f74,
                0x0600_0500,
                0x0102_0fc8,
                4,
                half as u32,
                (half >> 32) as u32
            ]
        );
    }

    #[test]
    fn fn_004d8fa0_sets_the_map_vtable() {
        let mut e = Engine::new();
        e.register(0x00aa_1070, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        zero(&mut e, &[0x0040_3d30]);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x004d_8fa0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0102_0fe4);
        assert_eq!(e.mem.u32(this + 4), 3);
    }

    #[test]
    fn fn_004d9000_clears_the_bucket_block() {
        let mut e = Engine::new();
        e.register(0x00aa_1070, |_, _| 0x4000u32.into_ret());
        zero(&mut e, &[0x0040_3d30]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 9);
        e.call_log = Some(vec![]);
        e.call(0x004d_9000, &args![this, 5u32]);
        assert_eq!(calls_to(&e, 0x00aa_1070), vec![vec![20]]);
        assert_eq!(calls_to(&e, 0x0040_3d30), vec![vec![0x4000, 0, 20]]);
        assert_eq!(
            (e.mem.u32(this), e.mem.u32(this + 8), e.mem.u32(this + 0xc)),
            (0x0102_1004, 0x4000, 0)
        );
    }

    #[test]
    fn fn_004d9070_empties_then_runs_the_base() {
        let mut e = Engine::new();
        zero(&mut e, &[0x0043_8af0, 0x00aa_10f0]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x004d_9070, &args![this]);
        assert_eq!(calls_to(&e, 0x0043_8af0), vec![vec![this], vec![this]]);
        assert_eq!(calls_to(&e, 0x00aa_10f0), vec![vec![0x4000]]);
        assert_eq!(e.mem.u32(this), 0x0102_1004);
    }

    #[test]
    fn fn_004d90d0_frees_the_bucket_block() {
        let mut e = Engine::new();
        zero(&mut e, &[0x0043_8af0, 0x00aa_10f0]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x4100);
        e.call_log = Some(vec![]);
        e.call(0x004d_90d0, &args![this]);
        assert_eq!(calls_to(&e, 0x00aa_10f0), vec![vec![0x4100]]);
        assert_eq!(e.mem.u32(this), 0x0102_1004);
    }

    #[test]
    fn fn_004d9100_builds_the_string_list() {
        let mut e = Engine::new();
        e.register(0x0096_afc0, |_, _| 0x4200u32.into_ret());
        let this = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x004d_9100, &args![this, 0x80u32, 0x0001_0040u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), 0x0102_1024);
        assert_eq!(e.mem.u32(this + 4), 0x4200);
        assert_eq!(e.mem.u16(this + 0xe), 0x40);
    }

    #[test]
    fn fn_004d91b0_frees_the_block() {
        let mut e = Engine::new();
        zero(&mut e, &[0x004e_de70]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x4300);
        e.call_log = Some(vec![]);
        e.call(0x004d_91b0, &args![this]);
        assert_eq!(calls_to(&e, 0x004e_de70), vec![vec![0x4300]]);
        assert_eq!(e.mem.u32(this), 0x0102_102c);
    }

    #[test]
    fn fn_004d9210_allocates_only_with_a_capacity() {
        let mut e = Engine::new();
        e.register(0x0096_afc0, |_, a| (0x4400 + a[0]).into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004d_9210, &args![this, 0u32, 3u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls_to(&e, 0x0096_afc0).is_empty());
        e.call(0x004d_9210, &args![this, 0xffff_0004u32, 3u32]);
        assert_eq!(e.mem.u32(this + 4), 0x4404);
        assert_eq!(
            (
                e.mem.u16(this + 8),
                e.mem.u16(this + 0xa),
                e.mem.u16(this + 0xc),
                e.mem.u16(this + 0xe)
            ),
            (4, 0, 0, 3)
        );
    }
}
