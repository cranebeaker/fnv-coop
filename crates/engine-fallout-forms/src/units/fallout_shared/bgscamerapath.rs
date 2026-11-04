//! `fallout shared/bgscamerapath.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058a380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058a380 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5c (0x38 bytes); `ECX` is the object.
/// The engine map names it `BGSCameraPath::BGSCameraPath` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0103_245c`) is `BGSCameraPath`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058a380(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0103_245c);
    e.call(0x0068_0890, &args![base + 0x18]);
    e.call(0x0096_a2d0, &args![base + 0x20]);
    e.call(0x004f_15a0, &args![this, 0x5cu32]);
    e.mem.set_u32(base + 0x2c, 0x0);
    e.call(0x0058_a620, &args![this]);
    this
}

// Translated from 0058a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058a450` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058a420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_a450, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `00401000` (`cdecl`, one word): allocates a block.
const ALLOCATE: u32 = 0x0040_1000;
/// `00500940` (`thiscall`): the path's `TESCondition` (+0x18).
const CONDITIONS_OF: u32 = 0x0050_0940;
/// `006815c0` (`thiscall`): returns its `this`.
const IDENTITY: u32 = 0x0068_15c0;
/// The root camera path array (`011ca700`).
const ROOT_PATHS: u32 = 0x011c_a700;
/// `NiFormArray::GetIndexForForm` (engine map), `004d6100` (`RET 4`).
const ARRAY_INDEX_OF: u32 = 0x004d_6100;
/// `NiFormArray::GetFormAt` (engine map), `004d6170` (`RET 4`).
const ARRAY_FORM_AT: u32 = 0x004d_6170;
/// `00486b80` (`thiscall`, `RET 4`): removes the entry at an index.
const ARRAY_REMOVE_AT: u32 = 0x0048_6b80;
/// `0061bdc0` (`thiscall`): compacts a form array.
const ARRAY_COMPACT: u32 = 0x0061_bdc0;
/// `0061bc10` (`thiscall`, `RET 8`): inserts a form at an index.
const ARRAY_INSERT: u32 = 0x0061_bc10;
/// `0061bd20` (`thiscall`, `RET 8`): removes the entry at an index into an
/// out word.
const ARRAY_TAKE: u32 = 0x0061_bd20;
/// `0084e3a0` (`thiscall`): the number of entries of a form array.
const ARRAY_COUNT: u32 = 0x0084_e3a0;
/// `00877a30` (`thiscall`, `RET 4`): the address of entry `i`.
const ARRAY_ENTRY: u32 = 0x0087_7a30;
/// `005f36f0` (`thiscall`; the engine map's
/// `ActorMover::GetPreferredMoveMode` is folded code): `pPrevPath` (+0x34).
const PREVIOUS_PATH_OF: u32 = 0x005f_36f0;
/// `008d7dc0` (`thiscall`, `RET 4`; folded code named
/// `LowProcess::SetItemBeingUsed`): stores `pPrevPath` (+0x34).
const SET_PREVIOUS_PATH: u32 = 0x008d_7dc0;
/// `005f4bb0` (`thiscall`, `RET 4`): stores `pParentPath` (+0x30).
const SET_PARENT_PATH: u32 = 0x005f_4bb0;
/// `TESForm::GetFile` (engine map), `00484e60` (`RET 4`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (engine map), `00485d50` (`cdecl`: `&id`,
/// file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`): the form with a form ID.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (engine map), `00ec43fb`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `TESForm`, `BGSCameraPath`, `BGSCameraShot`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_CAMERA_PATH: u32 = 0x0118_c268;
const TYPE_CAMERA_SHOT: u32 = 0x0118_6334;
/// `005b5e40` (`cdecl`, variadic): writes a warning.
const WARN: u32 = 0x005b_5e40;
/// `BSStringT` constructor, characters and destructor (`004037b0`,
/// `00559450`, `004037d0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_CHARS: u32 = 0x0055_9450;
const STRING_DESTROY: u32 = 0x0040_37d0;
/// The `TESFile` reading functions: form type, `TESForm::LoadForm`, chunk
/// type, chunk size, chunk data (`RET 8`), chunk data (`RET 4`), next
/// chunk, byte order.
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FORM_LOAD_FORM: u32 = 0x0048_5110;
const FILE_CHUNK: u32 = 0x0047_26b0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_CHUNK_VALUE: u32 = 0x0047_27f0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_BIG_ENDIAN: u32 = 0x0040_1680;
/// Chunk words.
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_ANAM: u32 = 0x4d41_4e41;
const CHUNK_SNAM: u32 = 0x4d41_4e53;

/// `__RTDynamicCast(form, 0, TESForm, target, 0)`.
fn cast(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

/// The form a stored form ID names, as the path's file sees it:
/// `TESForm::AddCompileIndex(&id, GetFile(-1))` on a local, then the form
/// (`004839c0`) cast to `target`. Answers the cast and the adjusted ID.
fn resolve(e: &mut Engine, this: Ptr, id: u32, target: u32) -> (u32, u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        let id = e.mem.u32(slot.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        (cast(e, form, target), id)
    })
}

/// A warning with the path's name: a local `BSStringT` filled by virtual
/// slot 0x90, `005b5e40(format, words..., characters)`, then the string
/// destroyed.
fn warn_with_name(e: &mut Engine, this: Ptr, format: u32, id: Option<u32>) {
    e.with_stack(8, |e, name| {
        e.call(STRING_CONSTRUCT, &args![name]);
        e.vcall(this.addr(), 0x90, &args![name]);
        let chars = e.call(STRING_CHARS, &args![name]).u32();
        match id {
            Some(id) => e.call(WARN, &args![format, id, chars]),
            None => e.call(WARN, &args![format, chars]),
        };
        e.call(STRING_DESTROY, &args![name]);
    });
}

// Translated from 0058a450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058a450 - the C++ exception unwinding states (FS:[0] frame, handler 00ef800e) are not translated
/// `BGSCameraPath`'s destructor (no Xbox PDB name; the vtable `0103245c`):
/// clears the path ([`fn_0058a650`]); unless `004226e0` on the object at
/// `011c3f2c` says so, unlinks it: from its parent (`00671d10`, then
/// `0058b730` on the parent) or, without one, from the root array
/// (`011ca700`): the next root path's previous path becomes this one's,
/// the entry is removed and the array compacted. Every child (`0058b600`
/// of them, `0058b630`) loses its parent and is deleted (virtual slot 0x10
/// with 1), then the child array (+0x2C, slot 0 with 1). Then the shot
/// list (+0x20, `0046ffb0`), the conditions (+0x18, `006808b0`) and
/// `TESForm::~TESForm`.
pub fn fn_0058a450(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_245c);
    fn_0058a650(e, this);
    let guard = e.mem.u32(0x011c_3f2c);
    if !e.call(0x0042_26e0, &args![guard]).bool() {
        if e.call(0x0067_1d10, &args![this]).u32() != 0 {
            let parent = e.call(0x0067_1d10, &args![this]).u32();
            e.call(0x0058_b730, &args![parent, this]);
        } else {
            let roots = e.mem.u32(ROOT_PATHS);
            let roots = e.call(IDENTITY, &args![roots]).u32();
            if roots != 0 {
                let index = e.call(ARRAY_INDEX_OF, &args![roots, this]).u32();
                if index != 0xffff_ffff {
                    let next = e
                        .call(ARRAY_FORM_AT, &args![roots, index.wrapping_add(1)])
                        .u32();
                    if next != 0 {
                        let previous = e.call(PREVIOUS_PATH_OF, &args![this]).u32();
                        e.call(SET_PREVIOUS_PATH, &args![next, previous]);
                    }
                    e.call(ARRAY_REMOVE_AT, &args![roots, index]);
                    e.call(ARRAY_COMPACT, &args![roots]);
                }
            }
        }
    }
    if e.mem.u32(base + 0x2c) != 0 {
        let count = e.call(0x0058_b600, &args![this]).u32();
        if count != 0 {
            let mut i = 0u32;
            while i < count {
                let child = e.call(0x0058_b630, &args![this, i]).u32();
                e.call(SET_PARENT_PATH, &args![child, 0u32]);
                if child != 0 {
                    e.vcall(child, 0x10, &args![1u32]);
                }
                i += 1;
            }
        }
        let children = e.mem.u32(base + 0x2c);
        if children != 0 {
            e.vcall(children, 0, &args![1u32]);
        }
    }
    e.call(0x0046_ffb0, &args![base + 0x20]);
    e.call(0x0068_08b0, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0058a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of a camera path (no Xbox PDB name): `data` (+0x28) byte 0,
/// `pParentPath` (+0x30) and `pPrevPath` (+0x34) null.
pub fn fn_0058a620(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x28, 0);
    e.mem.set_u32(this.addr() + 0x30, 0);
    e.mem.set_u32(this.addr() + 0x34, 0);
}

// Translated from 0058a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a camera path (no Xbox PDB name): its conditions (`00680fe0` on
/// what `00500940` answers) and its shot list (+0x20, `00470470`).
pub fn fn_0058a650(e: &mut Engine, this: Ptr) {
    let conditions = e.call(CONDITIONS_OF, &args![this]).u32();
    e.call(0x0068_0fe0, &args![conditions]);
    e.call(0x0047_0470, &args![this.addr() + 0x20]);
}

// Translated from 0058a9e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058a9e0 - the /GS cookie check (00ec408c) of its frame is not translated
/// `BGSCameraPath::Load` (Xbox PDB, matched by the engine map): false
/// unless the record's form type is 0x5C. The default path (`0058b960`)
/// is first taken out of the root array (`011ca700`); then
/// `TESForm::LoadForm` and the chunks: `OBND` (virtual slot 0xE0), `CTDA`
/// (`TESCondition::Load`), `DATA` (`TESForm::LoadData` into +0x28, 1
/// byte), `EDID` (a stack buffer of the chunk's size, at most 0x200 bytes,
/// to slot 0x134), `ANAM` (8 bytes, byte-swapped for a big-endian file, to
/// `pParentPath`/`pPrevPath` as form IDs, then `005ffbf0` adds the path to
/// the root array), `SNAM` (a shot ID added to the shot list, `00905820`).
/// True at the end.
pub fn bgs_camera_path_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(FILE_FORM_TYPE, &args![file]).u32() != 0x5c {
        return false;
    }
    if base == e.call(0x0058_b960, &args![]).u32() {
        let roots = e.mem.u32(ROOT_PATHS);
        let roots = e.call(IDENTITY, &args![roots]).u32();
        let index = e.call(ARRAY_INDEX_OF, &args![roots, this]).u32();
        if index != 0xffff_ffff {
            e.call(ARRAY_REMOVE_AT, &args![roots, index]);
            e.call(ARRAY_COMPACT, &args![roots]);
        }
    }
    e.call(FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(FILE_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_CTDA => {
                let conditions = e.call(CONDITIONS_OF, &args![this]).u32();
                e.call(0x0068_0960, &args![conditions, file]);
            }
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x28, 1u32]);
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_ANAM => {
                e.with_stack(8, |e, ids| {
                    e.call(0x0040_3d30, &args![ids, 0u32, 8u32]);
                    e.call(FILE_CHUNK_DATA, &args![file, ids, 8u32]);
                    if e.call(FILE_BIG_ENDIAN, &args![file]).bool() {
                        e.call(0x0046_2230, &args![ids]);
                    }
                    let parent = e.mem.u32(ids.addr());
                    let previous = e.mem.u32(ids.addr() + 4);
                    e.mem.set_u32(base + 0x30, parent);
                    e.mem.set_u32(base + 0x34, previous);
                });
                let roots = e.mem.u32(ROOT_PATHS);
                e.call(0x005f_fbf0, &args![roots, this]);
            }
            CHUNK_SNAM => {
                e.with_stack(8, |e, words| {
                    e.mem.set_u32(words.addr(), 0);
                    e.call(FILE_CHUNK_VALUE, &args![file, words]);
                    let shot = e.mem.u32(words.addr());
                    e.mem.set_u32(words.addr() + 4, shot);
                    e.call(0x0090_5820, &args![base + 0x20, words.addr() + 4]);
                });
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0058ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCameraPath::InitItem` (Xbox PDB, matched by the engine map): unless
/// `004013e0` says so, initializes the conditions
/// (`TESCondition::InitItem`) and resolves each shot ID of the shot list
/// (from `00891170`, until `008256d0` says the list is empty) to a
/// `BGSCameraShot`; a node whose shot is not found is removed (the head by
/// `0063f7b0`, a later one by `00905330` on the node before it) with a
/// `MASTERFILE` warning naming the path by editor ID (slot 0x130, when
/// `00474cb0` has one) or form ID (`0084e3a0`); found shots are stored in
/// their node (`00726c60`). Then `00484ab0(1)`.
pub fn bgs_camera_path_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let conditions = e.call(CONDITIONS_OF, &args![this]).u32();
    e.call(0x0068_0a20, &args![conditions, this]);
    let mut node = e.call(0x0089_1170, &args![this]).u32();
    let mut previous = 0u32;
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let item = e.call(IDENTITY, &args![node]).u32();
        let id = e.mem.u32(item);
        let (shot, id) = resolve(e, this, id, TYPE_CAMERA_SHOT);
        if shot == 0 {
            if previous != 0 {
                let item = e.call(IDENTITY, &args![node]).u32();
                e.call(0x0090_5330, &args![previous, item]);
                node = e.call(0x0072_6070, &args![previous]).u32();
            } else {
                e.call(0x0063_f7b0, &args![node]);
            }
            if e.call(0x0047_4cb0, &args![this]).u32() != 0 {
                let name = e.vcall(this.addr(), 0x130, &args![]).u32();
                e.call(WARN, &args![0x0103_25e8u32, id, name]);
            } else {
                let form_id = e.call(0x0084_e3a0, &args![this]).u32();
                e.call(WARN, &args![0x0103_2598u32, id, form_id]);
            }
        } else {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), shot);
                e.call(0x0072_6c60, &args![node, slot]);
            });
            previous = node;
            node = e.call(0x0072_6070, &args![node]).u32();
        }
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

// Translated from 0058aef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058aef0 - the C++ exception unwinding states (FS:[0] frame, handler 00ef8048) are not translated
/// `BGSCameraPath::InitParents` (Xbox PDB, matched by the engine map):
/// resolves `pParentPath` (+0x30) and `pPrevPath` (+0x34) from their form
/// IDs (a `DEFAULT` warning and a false answer for one not found; the
/// pointer is stored either way), then inserts the path into its parent's
/// child array ([`fn_0058b350`]) or the root array (`011ca700`): right
/// after its previous path, or at the end (count + 1) without one or when
/// `00440d80` says so, and fixes the order ([`bgs_camera_path_fix_next_paths`]).
/// Without an array: a warning and false.
pub fn bgs_camera_path_init_parents(e: &mut Engine, this: Ptr) -> bool {
    let mut ok = true;
    let mut parent = 0u32;
    let id = e.mem.u32(this.addr() + 0x30);
    if id != 0 {
        let (found, id) = resolve(e, this, id, TYPE_CAMERA_PATH);
        parent = found;
        if parent == 0 {
            warn_with_name(e, this, 0x0103_269c, Some(id));
            ok = false;
        }
        e.call(SET_PARENT_PATH, &args![this, parent]);
    }
    let mut previous = 0u32;
    let id = e.mem.u32(this.addr() + 0x34);
    if id != 0 {
        let (found, id) = resolve(e, this, id, TYPE_CAMERA_PATH);
        previous = found;
        if previous == 0 {
            warn_with_name(e, this, 0x0103_2664, Some(id));
            ok = false;
        }
        e.call(SET_PREVIOUS_PATH, &args![this, previous]);
    }
    let mut index = 0u32;
    let array = if parent != 0 {
        fn_0058b350(e, Ptr::new(parent))
    } else {
        let roots = e.mem.u32(ROOT_PATHS);
        e.call(IDENTITY, &args![roots]).u32()
    };
    if array == 0 {
        warn_with_name(e, this, 0x0103_2634, None);
        return false;
    }
    if previous != 0 {
        let mut at = e.call(ARRAY_INDEX_OF, &args![array, previous]).u32();
        if at != 0xffff_ffff {
            at = at.wrapping_add(1);
        }
        index = at;
    }
    if index == 0xffff_ffff || e.call(0x0044_0d80, &args![this]).bool() {
        index = e.call(ARRAY_COUNT, &args![array]).u32().wrapping_add(1);
    }
    e.call(ARRAY_INSERT, &args![array, index, this]);
    bgs_camera_path_fix_next_paths(e, this, array);
    ok
}

// Translated from 0058b190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058b190 - the C++ exception unwinding states (FS:[0] frame, handler 00ef8078) are not translated
/// `BGSCameraPath::FixNextPaths` (Xbox PDB, matched by the engine map):
/// in `array`, finds this path, the paths that are not camera paths
/// (counted) and the one path whose previous path is this one (a second
/// one gives a `DEFAULT` warning); when that next path is not right after
/// this one, it is taken out and inserted after it (the index corrected
/// for the non-paths before it) and the order fixed again from it.
pub fn bgs_camera_path_fix_next_paths(e: &mut Engine, this: Ptr, array: u32) {
    if array == 0 {
        return;
    }
    let mut skipped = 0u32;
    let mut self_index = 0xffff_ffffu32;
    let count = e.call(ARRAY_COUNT, &args![array]).u32();
    let mut next_index = 0xffff_ffffu32;
    let mut next = 0u32;
    let mut i = 0u32;
    while i < count {
        let entry = e.call(ARRAY_ENTRY, &args![array, i]).u32();
        let form = e.mem.u32(entry);
        let path = cast(e, form, TYPE_CAMERA_PATH);
        if path == 0 {
            skipped = skipped.wrapping_add(1);
        } else if path == this.addr() {
            self_index = i;
        } else if e.call(PREVIOUS_PATH_OF, &args![path]).u32() == this.addr() {
            if next == 0 {
                next = path;
                next_index = i;
            } else {
                warn_with_name(e, this, 0x0103_26d0, None);
            }
        }
        i += 1;
    }
    if self_index == 0xffff_ffff
        || next_index == 0xffff_ffff
        || next_index == self_index.wrapping_add(1)
    {
        return;
    }
    e.with_stack(4, |e, taken| {
        e.mem.set_u32(taken.addr(), 0);
        e.call(ARRAY_TAKE, &args![array, next_index, taken]);
    });
    e.call(ARRAY_COMPACT, &args![array]);
    if next_index < self_index {
        skipped = skipped.wrapping_add(1);
    }
    self_index = self_index.wrapping_sub(skipped);
    e.call(
        ARRAY_INSERT,
        &args![array, self_index.wrapping_add(1), next],
    );
    e.call(ARRAY_COMPACT, &args![array]);
    bgs_camera_path_fix_next_paths(e, Ptr::new(next), array);
}

// Translated from 0058b350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058b350 - the C++ exception unwinding states (FS:[0] frame, handler 00ef80ab) are not translated
/// The child array (`pChildPaths`, +0x2C), made on first use (no Xbox PDB
/// name): a 0x18-byte block constructed by `0058b3f0` (null when the
/// allocation fails).
pub fn fn_0058b350(e: &mut Engine, this: Ptr) -> u32 {
    if e.mem.u32(this.addr() + 0x2c) != 0 {
        return e.mem.u32(this.addr() + 0x2c);
    }
    let block = e.call(ALLOCATE, &args![0x18u32]).u32();
    let array = if block == 0 {
        0
    } else {
        e.call(0x0058_b3f0, &args![block]).u32()
    };
    e.mem.set_u32(this.addr() + 0x2c, array);
    array
}

/// `TESForm::CopyAllComponents` (decompiler name of `004851b0`; `RET 4`, the other form).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `TESForm::CompareAllComponents` (decompiler name of `00485270`; `RET 4`, `AL` true when
/// different).
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `00680a70` (`ECX` = conditions, `RET 8`: the other conditions, 0): copies conditions.
const CONDITIONS_COPY: u32 = 0x0068_0a70;
/// `00680b70` (`ECX` = conditions, `RET 4`, `AL` true when different): compares
/// conditions.
const CONDITIONS_COMPARE: u32 = 0x0068_0b70;
/// `TESCondition::Save` (decompiler name of `00680910`, `ECX` = conditions).
const CONDITIONS_SAVE: u32 = 0x0068_0910;
/// `00401460` (`cdecl`: destination, source, size): copies memory.
const MEMCPY: u32 = 0x0040_1460;
/// `memcmp` (`00ec4835`, `cdecl`: first, second, size).
const MEMCMP: u32 = 0x00ec_4835;
/// `00891170` (`ECX` = path): the address of its shot list (+0x20).
const SHOT_LIST_OF: u32 = 0x0089_1170;
/// `008256d0` (`ECX` = list node, `AL`): true when the node is empty.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `00726070` (`ECX` = list node): the next node (the word at +4).
const NODE_NEXT: u32 = 0x0072_6070;
/// `00470470` (`ECX` = list): removes every item.
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// `00905820` (`ECX` = list, `RET 4`): appends the item a pointer points at.
const LIST_ADD_TAIL: u32 = 0x0090_5820;
/// `004867a0` (`ECX` = form, `RET 8`: a byte, the references map): the form's duplicate.
const FORM_DUPLICATE: u32 = 0x0048_67a0;
/// `00853130` (`ECX` = references map, `RET 8`: a form, the address of the answer; `AL`):
/// looks a form up in the map.
const MAP_GET_AT: u32 = 0x0085_3130;
/// `00864280` (`ECX` = form array, `RET 4`): sizes the array to a count.
const ARRAY_SET_SIZE: u32 = 0x0086_4280;
/// The child's virtual slot that duplicates it (`RET 8`: the byte and the map).
const SLOT_DUPLICATE: u32 = 0x40;
/// `00671d10` (`ECX` = path): the parent path.
const PARENT_PATH_OF: u32 = 0x0067_1d10;
/// `00440d80` (`ECX` = form, `AL`): whether bit 0x20 of the form flags (+8) is set.
const FORM_FLAG_20_SET: u32 = 0x0044_0d80;
/// `00403d30` (`cdecl`: address, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `00401500` (`cdecl`, no arguments, `AL`): whether saved data is byte-swapped.
const SWAP_ON_SAVE: u32 = 0x0040_1500;
/// `00462230` (`ECX` = the two `ANAM` words): swaps their byte order.
const SWAP_ANAM: u32 = 0x0046_2230;
/// `00483710` (`ECX` = the data byte): the byte's swap, which does nothing.
const SWAP_DATA_BYTE: u32 = 0x0048_3710;
/// `TESForm::__AddChunkData` (`00485990`, `cdecl`: chunk signature, address, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESForm::__SaveData` (`00485e20`, `RET 8`: address, size).
const FORM_SAVE_DATA: u32 = 0x0048_5e20;
/// `00485910` (`cdecl`: chunk signature, word): saves a chunk holding one word.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// `TESForm::StartForm` and `TESForm::CloseForm` (`004855a0`, `00485680`, `thiscall`).
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `0058b680` (`ECX` = path, `RET 4`): the index of a child in the child array.
const CHILD_INDEX_OF: u32 = 0x0058_b680;
/// `0058b6c0` (`ECX` = path, `RET 4`): removes the child at an index.
const REMOVE_CHILD_AT: u32 = 0x0058_b6c0;
/// The path's data byte (+0x28) and child array (+0x2C).
const DATA_OFFSET: u32 = 0x28;
const CHILDREN_OFFSET: u32 = 0x2c;
/// `pParentPath` (+0x30) and `pPrevPath` (+0x34).
const PARENT_OFFSET: u32 = 0x30;
const PREVIOUS_OFFSET: u32 = 0x34;

// Translated from 0058a680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies another camera path into this one (`thiscall`, `RET 4`; unnamed in the engine
/// map). When `other` casts from `TESForm` to `BGSCameraPath`: every component
/// (`004851b0`), the conditions (`00680a70` with 0), the data byte (+0x28), and the shot
/// list: this list is emptied (`00470470`) and every item of the other's list is appended
/// (`00905820`) until a null or empty node.
pub fn fn_0058a680(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = cast(e, other.addr(), TYPE_CAMERA_PATH);
    if source == 0 {
        return;
    }
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    let their_conditions = e.call(CONDITIONS_OF, &args![source]).u32();
    let our_conditions = e.call(CONDITIONS_OF, &args![this]).u32();
    e.call(
        CONDITIONS_COPY,
        &args![our_conditions, their_conditions, 0u32],
    );
    e.call(
        MEMCPY,
        &args![this.addr() + DATA_OFFSET, source + DATA_OFFSET, 1u32],
    );
    let shots = this.addr() + 0x20;
    e.call(LIST_REMOVE_ALL, &args![shots]);
    let mut node = e.call(SHOT_LIST_OF, &args![source]).u32();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(IDENTITY, &args![node]).u32();
        let item = e.mem.u32(slot);
        node = e.call(NODE_NEXT, &args![node]).u32();
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), item);
            e.call(LIST_ADD_TAIL, &args![shots, local]);
        });
    }
}

// Translated from 0058a750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares another camera path with this one (`thiscall`, `RET 4`, `AL`; unnamed in the
/// engine map): true ("differs") when `other` is not a `BGSCameraPath`, the components
/// (`00485270`) or the conditions (`00680b70`) differ, the data byte differs (`memcmp`),
/// or the shot lists differ item by item (walking the other's list while its node is set
/// and not empty; this list ending first differs, and this list having a non-empty node
/// left afterwards differs).
pub fn fn_0058a750(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = cast(e, other.addr(), TYPE_CAMERA_PATH);
    if source == 0 {
        return true;
    }
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, source])
        .u8()
        != 0
    {
        return true;
    }
    let their_conditions = e.call(CONDITIONS_OF, &args![source]).u32();
    let our_conditions = e.call(CONDITIONS_OF, &args![this]).u32();
    if e.call(CONDITIONS_COMPARE, &args![our_conditions, their_conditions])
        .u8()
        != 0
    {
        return true;
    }
    if e.call(
        MEMCMP,
        &args![this.addr() + DATA_OFFSET, source + DATA_OFFSET, 1u32],
    )
    .u32()
        != 0
    {
        return true;
    }
    let mut ours = e.call(SHOT_LIST_OF, &args![this]).u32();
    let mut theirs = e.call(SHOT_LIST_OF, &args![source]).u32();
    while theirs != 0 && !e.call(NODE_IS_EMPTY, &args![theirs]).bool() {
        if ours == 0 {
            return true;
        }
        let our_slot = e.call(IDENTITY, &args![ours]).u32();
        let their_slot = e.call(IDENTITY, &args![theirs]).u32();
        if e.mem.u32(our_slot) != e.mem.u32(their_slot) {
            return true;
        }
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
        ours = e.call(NODE_NEXT, &args![ours]).u32();
    }
    ours != 0 && !e.call(NODE_IS_EMPTY, &args![ours]).bool()
}

/// The form `map` gives for `form` (`00853130` into a cleared local), or `form` itself
/// when the lookup fails or answers null.
fn mapped_form(e: &mut Engine, map: u32, form: u32) -> u32 {
    e.with_stack(4, |e, answer| {
        e.mem.set_u32(answer.addr(), 0);
        let found = e.call(MAP_GET_AT, &args![map, form, answer]).u8() != 0;
        let mapped = e.mem.u32(answer.addr());
        if !found || mapped == 0 {
            form
        } else {
            mapped
        }
    })
}

// Translated from 0058a870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Duplicates the camera path (`thiscall`, `RET 8`: a byte and a references map; unnamed
/// in the engine map): the form's duplicate (`004867a0`) cast to `BGSCameraPath`; its
/// parent and previous paths are this one's, mapped through `map` (`00853130`) when the
/// map has them; with a child array, it is compacted, the duplicate's array (`0058b350`)
/// sized to the count (`00864280`) and each child's duplicate (virtual slot `0x40`, same
/// byte and map) stored at its index (`0061bd20`). Answers the duplicate.
pub fn fn_0058a870(e: &mut Engine, this: Ptr, flag: u8, map: u32) -> u32 {
    let form = e
        .call(FORM_DUPLICATE, &args![this, u32::from(flag), map])
        .u32();
    let duplicate = cast(e, form, TYPE_CAMERA_PATH);
    let parent = e.mem.u32(this.addr() + PARENT_OFFSET);
    let parent = mapped_form(e, map, parent);
    e.call(SET_PARENT_PATH, &args![duplicate, parent]);
    let previous = e.mem.u32(this.addr() + PREVIOUS_OFFSET);
    let previous = mapped_form(e, map, previous);
    e.call(SET_PREVIOUS_PATH, &args![duplicate, previous]);
    if e.mem.u32(this.addr() + CHILDREN_OFFSET) != 0 {
        let children = e.mem.u32(this.addr() + CHILDREN_OFFSET);
        e.call(ARRAY_COMPACT, &args![children]);
        let children = e.mem.u32(this.addr() + CHILDREN_OFFSET);
        let count = e.call(ARRAY_COUNT, &args![children]).u32();
        let array = fn_0058b350(e, Ptr::new(duplicate));
        e.call(ARRAY_SET_SIZE, &args![array, count]);
        let mut index = 0u32;
        while index < count {
            let children = e.mem.u32(this.addr() + CHILDREN_OFFSET);
            let slot = e.call(ARRAY_ENTRY, &args![children, index]).u32();
            let child = e.mem.u32(slot);
            let copy = e
                .vcall(child, SLOT_DUPLICATE, &args![u32::from(flag), map])
                .u32();
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), copy);
                e.call(ARRAY_TAKE, &args![array, index, local]);
            });
            index += 1;
        }
    }
    duplicate
}

// Translated from 0058ac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the camera path (`thiscall`, no stack word; unnamed in the engine map): starts
/// the form record, saves the conditions (`00680910`), then an `ANAM` chunk of two words
/// (cleared by `00403d30`): the parent's form id (when there is a parent, `00671d10`) and
/// the form id of the first previous path (`005f36f0`, walking past those with form flag
/// 0x20) unless this path has flag 0x20 itself, byte swapped (`00462230`) around the save
/// when `00401500` says so; then the data byte (+0x28, `TESForm::__SaveData`, its no-op
/// swap `00483710` around it), one `SNAM` chunk per shot's form id, and closes the record.
pub fn fn_0058ac10(e: &mut Engine, this: Ptr) {
    e.call(FORM_START_FORM, &args![this]);
    let conditions = e.call(CONDITIONS_OF, &args![this]).u32();
    e.call(CONDITIONS_SAVE, &args![conditions]);
    e.with_stack(8, |e, ids| {
        e.call(FILL_MEMORY, &args![ids, 0u32, 8u32]);
        if e.call(PARENT_PATH_OF, &args![this]).u32() != 0 {
            let parent = e.call(PARENT_PATH_OF, &args![this]).u32();
            let id = e.call(FORM_ID, &args![parent]).u32();
            e.mem.set_u32(ids.addr(), id);
        }
        if e.call(FORM_FLAG_20_SET, &args![this]).u8() == 0 {
            let mut previous = e.call(PREVIOUS_PATH_OF, &args![this]).u32();
            while previous != 0 && e.call(FORM_FLAG_20_SET, &args![previous]).u8() != 0 {
                previous = e.call(PREVIOUS_PATH_OF, &args![previous]).u32();
            }
            if previous != 0 {
                let id = e.call(FORM_ID, &args![previous]).u32();
                e.mem.set_u32(ids.addr() + 4, id);
            }
        }
        if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
            e.call(SWAP_ANAM, &args![ids]);
        }
        e.call(ADD_CHUNK_DATA, &args![CHUNK_ANAM, ids, 8u32]);
        if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
            e.call(SWAP_ANAM, &args![ids]);
        }
    });
    let data = this.addr() + DATA_OFFSET;
    if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
        e.call(SWAP_DATA_BYTE, &args![data]);
    }
    e.call(FORM_SAVE_DATA, &args![this, data, 1u32]);
    if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
        e.call(SWAP_DATA_BYTE, &args![data]);
    }
    let mut node = e.call(SHOT_LIST_OF, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(IDENTITY, &args![node]).u32();
        let shot = e.mem.u32(slot);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let id = e.call(FORM_ID, &args![shot]).u32();
        e.call(ADD_CHUNK_WORD, &args![CHUNK_SNAM, id]);
    }
    e.call(FORM_CLOSE_FORM, &args![this]);
}

// Translated from 0058b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `child` from the path's children (`thiscall`, `RET 4`; unnamed in the engine
/// map): when `child` is set and there is a child array, removes the entry at the child's
/// index (`0058b680`, then `0058b6c0`).
pub fn fn_0058b730(e: &mut Engine, this: Ptr, child: u32) {
    if child == 0 || e.mem.u32(this.addr() + CHILDREN_OFFSET) == 0 {
        return;
    }
    let index = e.call(CHILD_INDEX_OF, &args![this, child]).u32();
    e.call(REMOVE_CHILD_AT, &args![this, index]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_a380, fn_0058a380(Ptr) -> Ptr),
        entry!(0x0058_a420, fn_0058a420(Ptr, u32) -> Ptr),
        entry!(0x0058a450, fn_0058a450(Ptr)),
        entry!(0x0058a620, fn_0058a620(Ptr)),
        entry!(0x0058a650, fn_0058a650(Ptr)),
        entry!(0x0058a9e0, bgs_camera_path_load(Ptr, Ptr) -> bool),
        entry!(0x0058ad90, bgs_camera_path_init_item(Ptr)),
        entry!(0x0058aef0, bgs_camera_path_init_parents(Ptr) -> bool),
        entry!(0x0058b190, bgs_camera_path_fix_next_paths(Ptr, u32)),
        entry!(0x0058b350, fn_0058b350(Ptr) -> u32),
        entry!(0x0058_a680, fn_0058a680(Ptr, Ptr)),
        entry!(0x0058_a750, fn_0058a750(Ptr, Ptr) -> bool),
        entry!(0x0058_a870, fn_0058a870(Ptr, u8, u32) -> u32),
        entry!(0x0058_ac10, fn_0058ac10(Ptr)),
        entry!(0x0058_b730, fn_0058b730(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0058a380_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_a380,
            size: 0x38,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0068_0890, &[A::T(0x18)]),
                (0x0096_a2d0, &[A::T(0x20)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5c)]),
                (0x0058_a620, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_245c), (0x2c, 4, 0x0)],
        });
    }
    #[test]
    fn at_0058a420_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_a450, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_a420, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_a420)
                .collect();
            let mut expected = vec![(0x0058_a450, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b019_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for a in addresses {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// A path object (0x38 bytes) whose vtable sends `slots` to doubles at
    /// `0x00ab_0000 + slot`.
    fn path(e: &mut Engine, slots: &[u32]) -> Ptr {
        let object = e.mem.alloc(0x38);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(object, vtable);
        for slot in slots {
            e.mem.set_u32(vtable + slot, 0x00ab_0000 + slot);
            e.register(0x00ab_0000 + slot, |_, _| Ret::default());
        }
        Ptr::new(object)
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011c_a000, 0x1000);
        e.register(IDENTITY, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn start_values_and_clear() {
        let mut e = engine();
        let p = path(&mut e, &[]);
        for o in [0x28, 0x30, 0x34] {
            e.mem.set_u32(p.addr() + o, 0xffff_ffff);
        }
        e.call(0x0058_a620, &args![p]);
        assert_eq!(e.mem.u8(p.addr() + 0x28), 0);
        assert_eq!(e.mem.u8(p.addr() + 0x29), 0xff);
        assert_eq!(
            (e.mem.u32(p.addr() + 0x30), e.mem.u32(p.addr() + 0x34)),
            (0, 0)
        );
        e.register(CONDITIONS_OF, |_, a| Ret {
            eax: a[0] + 0x18,
            ..Ret::default()
        });
        quiet(&mut e, &[0x0068_0fe0, 0x0047_0470]);
        e.call(0x0058_a650, &args![p]);
        assert_eq!(calls(&e, 0x0068_0fe0), vec![vec![p.addr() + 0x18]]);
        assert_eq!(calls(&e, 0x0047_0470), vec![vec![p.addr() + 0x20]]);
    }

    #[test]
    fn destructor_unlinks_from_the_root_array_and_deletes_children() {
        let mut e = engine();
        let p = path(&mut e, &[]);
        e.mem.set_u32(p.addr() + 0x34, 0x6666);
        let child = path(&mut e, &[0x10]);
        let children = path(&mut e, &[0]);
        e.mem.set_u32(p.addr() + 0x2c, children.addr());
        e.mem.set_u32(ROOT_PATHS, 0x9000);
        e.register(CONDITIONS_OF, |_, a| Ret {
            eax: a[0] + 0x18,
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x0068_0fe0,
                0x0047_0470,
                0x0042_26e0,
                0x0067_1d10,
                ARRAY_REMOVE_AT,
                ARRAY_COMPACT,
                SET_PREVIOUS_PATH,
                SET_PARENT_PATH,
                0x0046_ffb0,
                0x0068_08b0,
                0x0048_3630,
            ],
        );
        e.register(ARRAY_INDEX_OF, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        e.register(ARRAY_FORM_AT, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register(PREVIOUS_PATH_OF, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        e.register(0x0058_b600, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let c = child.addr();
        e.register_double(0x0058_b630, move |_, _| Ret {
            eax: c,
            ..Ret::default()
        });
        e.call(0x0058_a450, &args![p]);
        assert_eq!(e.mem.u32(p.addr()), 0x0103_245c);
        assert_eq!(calls(&e, ARRAY_FORM_AT), vec![vec![0x9000, 5]]);
        assert_eq!(calls(&e, SET_PREVIOUS_PATH), vec![vec![0x7777, 0x6666]]);
        assert_eq!(calls(&e, ARRAY_REMOVE_AT), vec![vec![0x9000, 4]]);
        assert_eq!(calls(&e, SET_PARENT_PATH), vec![vec![c, 0]]);
        assert_eq!(calls(&e, 0x00ab_0010), vec![vec![c, 1]]);
        assert_eq!(calls(&e, 0x00ab_0000), vec![vec![children.addr(), 1]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![p.addr()]]);
    }

    #[test]
    fn destructor_with_a_parent_leaves_the_root_array_alone() {
        let mut e = engine();
        let p = path(&mut e, &[]);
        e.register(CONDITIONS_OF, |_, a| Ret {
            eax: a[0] + 0x18,
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x0068_0fe0,
                0x0047_0470,
                0x0042_26e0,
                0x0058_b730,
                0x0046_ffb0,
                0x0068_08b0,
                0x0048_3630,
            ],
        );
        e.register(0x0067_1d10, |_, _| Ret {
            eax: 0x5100,
            ..Ret::default()
        });
        e.call(0x0058_a450, &args![p]);
        assert_eq!(calls(&e, 0x0058_b730), vec![vec![0x5100, p.addr()]]);
        assert!(calls(&e, IDENTITY).is_empty());
    }

    /// A record of `chunks` read through doubles of the file functions.
    fn file(e: &mut Engine, form_type: u32, chunks: Vec<u32>) {
        let queue = Rc::new(RefCell::new(chunks));
        e.register_double(FILE_FORM_TYPE, move |_, _| Ret {
            eax: form_type,
            ..Ret::default()
        });
        let q = queue.clone();
        e.register_double(FILE_CHUNK, move |_, _| Ret {
            eax: q.borrow().first().copied().unwrap_or(0),
            ..Ret::default()
        });
        let q = queue;
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            q.borrow_mut().remove(0);
            Ret {
                eax: !q.borrow().is_empty() as u32,
                ..Ret::default()
            }
        });
        e.register(FILE_CHUNK_SIZE, |_, _| Ret {
            eax: 6,
            ..Ret::default()
        });
        e.register(FILE_CHUNK_DATA, |e, a| {
            if a[2] == 8 {
                e.mem.set_u32(a[1], 0x0101);
                e.mem.set_u32(a[1] + 4, 0x0202);
            }
            Ret::default()
        });
        e.register(FILE_CHUNK_VALUE, |e, a| {
            e.mem.set_u32(a[1], 0x0303);
            Ret::default()
        });
    }

    #[test]
    fn load_reads_every_chunk() {
        let mut e = engine();
        let p = path(&mut e, &[0xe0, 0x134]);
        file(
            &mut e,
            0x5c,
            vec![
                CHUNK_OBND, CHUNK_CTDA, CHUNK_DATA, CHUNK_EDID, CHUNK_ANAM, CHUNK_SNAM,
            ],
        );
        e.mem.set_u32(ROOT_PATHS, 0x9000);
        e.register(0x0058_b960, |_, _| Ret::default());
        e.register(CONDITIONS_OF, |_, a| Ret {
            eax: a[0] + 0x18,
            ..Ret::default()
        });
        e.register(FILE_BIG_ENDIAN, |_, _| Ret::default());
        e.register(0x0090_5820, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x0303);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                FORM_LOAD_FORM,
                0x0068_0960,
                0x0048_61f0,
                0x0040_3d30,
                0x005f_fbf0,
            ],
        );
        let f = 0x7000u32;
        assert!(e.call(0x0058_a9e0, &args![p, f]).bool());
        assert_eq!(calls(&e, 0x00ab_00e0), vec![vec![p.addr(), f]]);
        assert_eq!(calls(&e, 0x0068_0960), vec![vec![p.addr() + 0x18, f]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![p.addr(), f, p.addr() + 0x28, 1]]
        );
        assert_eq!(calls(&e, 0x00ab_0134).len(), 1);
        assert_eq!(e.mem.u32(p.addr() + 0x30), 0x0101);
        assert_eq!(e.mem.u32(p.addr() + 0x34), 0x0202);
        assert_eq!(calls(&e, 0x005f_fbf0), vec![vec![0x9000, p.addr()]]);
        assert_eq!(calls(&e, 0x0090_5820)[0][0], p.addr() + 0x20);
        assert!(calls(&e, ARRAY_INDEX_OF).is_empty());
    }

    #[test]
    fn loading_the_default_path_takes_it_out_of_the_roots_first() {
        let mut e = engine();
        let p = path(&mut e, &[]);
        file(&mut e, 0x5c, vec![0x1111_1111]);
        e.mem.set_u32(ROOT_PATHS, 0x9000);
        let a = p.addr();
        e.register_double(0x0058_b960, move |_, _| Ret {
            eax: a,
            ..Ret::default()
        });
        e.register(ARRAY_INDEX_OF, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        quiet(&mut e, &[ARRAY_REMOVE_AT, ARRAY_COMPACT, FORM_LOAD_FORM]);
        assert!(e.call(0x0058_a9e0, &args![p, 0x7000u32]).bool());
        assert_eq!(calls(&e, ARRAY_REMOVE_AT), vec![vec![0x9000, 2]]);
        // Another form type: false before anything else.
        file(&mut e, 0x5b, vec![]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0058_a9e0, &args![p, 0x7000u32]).bool());
        assert!(calls(&e, FORM_LOAD_FORM).is_empty());
    }

    /// Form lookup: IDs below 0x100 are not found; the cast passes the
    /// form through.
    fn forms(e: &mut Engine) {
        e.register(FORM_GET_FILE, |_, _| Ret {
            eax: 0x4400,
            ..Ret::default()
        });
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: if a[0] & 0xff00 != 0 { a[0] & 0xffff } else { 0 },
            ..Ret::default()
        });
        e.register(DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        quiet(e, &[WARN, STRING_CONSTRUCT, STRING_DESTROY]);
        e.register(STRING_CHARS, |_, _| Ret {
            eax: 0x5555,
            ..Ret::default()
        });
    }

    #[test]
    fn init_item_keeps_found_shots_and_drops_missing_ones() {
        let mut e = engine();
        forms(&mut e);
        let p = path(&mut e, &[0x130]);
        // A list of two nodes: a missing shot (0x12) then a found one
        // (0x3400).
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x3400);
        let head = e.mem.alloc(8);
        e.mem.set_u32(head, 0x12);
        e.mem.set_u32(head + 4, second);
        e.register(0x0040_13e0, |_, _| Ret::default());
        e.register(CONDITIONS_OF, |_, a| Ret {
            eax: a[0] + 0x18,
            ..Ret::default()
        });
        e.register_double(0x0089_1170, move |_, _| Ret {
            eax: head,
            ..Ret::default()
        });
        e.register(0x0082_56d0, |_, _| Ret::default());
        // Dropping the head moves the second item into it.
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let item = e.mem.u32(next);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(0x0072_6070, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(0x0072_6c60, |e, a| {
            let shot = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], shot);
            Ret::default()
        });
        e.register(0x0047_4cb0, |_, _| Ret::default());
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0xabcd,
            ..Ret::default()
        });
        quiet(&mut e, &[0x0068_0a20, 0x0048_4ab0]);
        e.call(0x0058_ad90, &args![p]);
        assert_eq!(
            calls(&e, WARN),
            vec![vec![0x0103_2598, 0x0100_0012, 0xabcd]]
        );
        assert_eq!(e.mem.u32(head), 0x3400);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![p.addr(), 1]]);
        assert_eq!(
            calls(&e, 0x0068_0a20),
            vec![vec![p.addr() + 0x18, p.addr()]]
        );
    }

    #[test]
    fn init_parents_inserts_after_the_previous_path() {
        let mut e = engine();
        forms(&mut e);
        let p = path(&mut e, &[0x90]);
        e.mem.set_u32(p.addr() + 0x34, 0x2200);
        e.mem.set_u32(ROOT_PATHS, 0x9000);
        quiet(&mut e, &[SET_PREVIOUS_PATH, ARRAY_INSERT]);
        e.register(ARRAY_INDEX_OF, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(0x0044_0d80, |_, _| Ret::default());
        // FixNextPaths runs for real on an empty array.
        e.register(ARRAY_COUNT, |_, _| Ret::default());
        assert!(e.call(0x0058_aef0, &args![p]).bool());
        assert_eq!(calls(&e, SET_PREVIOUS_PATH), vec![vec![p.addr(), 0x2200]]);
        assert_eq!(calls(&e, ARRAY_INSERT), vec![vec![0x9000, 4, p.addr()]]);
        assert!(calls(&e, WARN).is_empty());
    }

    #[test]
    fn init_parents_warns_for_a_missing_parent_and_a_missing_array() {
        let mut e = engine();
        forms(&mut e);
        let p = path(&mut e, &[0x90]);
        e.mem.set_u32(p.addr() + 0x30, 0x12);
        quiet(&mut e, &[SET_PARENT_PATH]);
        assert!(!e.call(0x0058_aef0, &args![p]).bool());
        let warnings: Vec<u32> = calls(&e, WARN).iter().map(|w| w[0]).collect();
        assert_eq!(warnings, vec![0x0103_269c, 0x0103_2634]);
        assert_eq!(calls(&e, WARN)[0], vec![0x0103_269c, 0x0100_0012, 0x5555]);
        assert_eq!(calls(&e, SET_PARENT_PATH), vec![vec![p.addr(), 0]]);
    }

    #[test]
    fn fix_next_paths_moves_the_next_path_after_this_one() {
        let mut e = engine();
        let p = path(&mut e, &[0x90]);
        let next = path(&mut e, &[]);
        e.mem.set_u32(next.addr() + 0x34, p.addr());
        // Array: next, a non-path, this.
        let data = e.mem.alloc(12);
        e.mem.set_u32(data, next.addr());
        e.mem.set_u32(data + 4, 0x0bad);
        e.mem.set_u32(data + 8, p.addr());
        let entries = Rc::new(RefCell::new(vec![next.addr(), 0x0bad, p.addr()]));
        let q = entries.clone();
        e.register_double(ARRAY_COUNT, move |_, _| Ret {
            eax: q.borrow().len() as u32,
            ..Ret::default()
        });
        e.register_double(ARRAY_ENTRY, move |e, a| {
            let at = e.mem.alloc(4);
            e.mem.set_u32(at, entries.borrow()[a[1] as usize]);
            Ret {
                eax: at,
                ..Ret::default()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| Ret {
            eax: if a[0] == 0x0bad { 0 } else { a[0] },
            ..Ret::default()
        });
        e.register(PREVIOUS_PATH_OF, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        quiet(&mut e, &[ARRAY_TAKE, ARRAY_COMPACT, ARRAY_INSERT]);
        let array = 0x9000u32;
        e.call(0x0058_b190, &args![p, array]);
        assert_eq!(calls(&e, ARRAY_TAKE)[0][..2], [array, 0]);
        // self 2, one non-path, next before this: 2 - 2 + 1 = 1.
        assert_eq!(calls(&e, ARRAY_INSERT), vec![vec![array, 1, next.addr()]]);
        // The recursion on the next path ran (one more count).
        assert_eq!(calls(&e, ARRAY_COUNT).len(), 2);
    }

    #[test]
    fn child_array_is_made_once() {
        let mut e = engine();
        let p = path(&mut e, &[]);
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x8800,
            ..Ret::default()
        });
        e.register(0x0058_b3f0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x0058_b350, &args![p]).u32(), 0x8800);
        assert_eq!(e.call(0x0058_b350, &args![p]).u32(), 0x8800);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Builds a shot list (nodes of item, next) and answers its head; an empty list is
    /// one node whose item and next are 0.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        if next == 0 {
            next = e.mem.alloc(8);
        }
        next
    }

    /// An engine whose list helpers work on such nodes; `00891170` answers the word at
    /// +0x20 of a path (the head node here).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SHOT_LIST_OF, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(IDENTITY, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CONDITIONS_OF, |_, a| (a[0] + 0x18).into_ret());
        e
    }

    fn path(e: &mut Engine, shots: &[u32], data: u8) -> u32 {
        let this = e.mem.alloc(0x38);
        let head = list(e, shots);
        e.mem.set_u32(this + 0x20, head);
        e.mem.set_u8(this + 0x28, data);
        this
    }

    #[test]
    fn copy_takes_components_conditions_data_and_shots() {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        for addr in [
            FORM_COPY_ALL_COMPONENTS,
            CONDITIONS_COPY,
            MEMCPY,
            LIST_REMOVE_ALL,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(LIST_ADD_TAIL, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(0x0003_1000 + item * 4, a[0]);
            Ret::default()
        });
        e.mem.map(0x0003_1000, 0x100);
        let this = path(&mut e, &[], 0);
        let other = path(&mut e, &[3, 5], 7);
        e.call_log = Some(vec![]);
        e.call(0x0058_a680, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(to(&log, FORM_COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(
            to(&log, CONDITIONS_COPY),
            vec![vec![this + 0x18, other + 0x18, 0]]
        );
        assert_eq!(to(&log, MEMCPY), vec![vec![this + 0x28, other + 0x28, 1]]);
        assert_eq!(to(&log, LIST_REMOVE_ALL), vec![vec![this + 0x20]]);
        assert_eq!(to(&log, LIST_ADD_TAIL).len(), 2);
        assert_eq!(e.mem.u32(0x0003_1000 + 12), this + 0x20);
        assert_eq!(e.mem.u32(0x0003_1000 + 20), this + 0x20);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0058_a680, &args![this, other]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    fn compare_engine() -> Engine {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_COMPARE_ALL_COMPONENTS, |_, _| 0u32.into_ret());
        e.register(CONDITIONS_COMPARE, |_, _| 0u32.into_ret());
        e.register(MEMCMP, |e, a| {
            u32::from(e.mem.u8(a[0]) != e.mem.u8(a[1])).into_ret()
        });
        e
    }

    #[test]
    fn compare_checks_components_conditions_data_and_shots() {
        let mut e = compare_engine();
        let this = path(&mut e, &[3, 5], 1);
        let same = path(&mut e, &[3, 5], 1);
        assert!(!e.call(0x0058_a750, &args![this, same]).bool());
        let other_data = path(&mut e, &[3, 5], 2);
        assert!(e.call(0x0058_a750, &args![this, other_data]).bool());
        let other_shot = path(&mut e, &[3, 6], 1);
        assert!(e.call(0x0058_a750, &args![this, other_shot]).bool());
        let longer = path(&mut e, &[3, 5, 7], 1);
        assert!(e.call(0x0058_a750, &args![this, longer]).bool());
        let shorter = path(&mut e, &[3], 1);
        assert!(e.call(0x0058_a750, &args![this, shorter]).bool());
        let empty = path(&mut e, &[], 1);
        let also_empty = path(&mut e, &[], 1);
        assert!(!e.call(0x0058_a750, &args![empty, also_empty]).bool());
        e.register(CONDITIONS_COMPARE, |_, _| 1u32.into_ret());
        assert!(e.call(0x0058_a750, &args![this, same]).bool());
        e.register(FORM_COMPARE_ALL_COMPONENTS, |_, _| 1u32.into_ret());
        assert!(e.call(0x0058_a750, &args![this, same]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0058_a750, &args![this, same]).bool());
    }

    #[test]
    fn duplicate_maps_parents_and_duplicates_children() {
        let mut e = engine();
        let this = path(&mut e, &[], 0);
        let copy = path(&mut e, &[], 0);
        e.mem.set_u32(this + 0x30, 0x0100);
        e.mem.set_u32(this + 0x34, 0x0200);
        let children = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x2c, children);
        e.mem.set_u32(copy + 0x2c, 0x0004_4000);
        e.put_vtable(0x0190_0000, &[0x5040; 0x11]);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x0190_0000);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x0190_0000);
        e.mem.set_u32(children + 4, first);
        e.mem.set_u32(children + 8, second);
        e.register_double(FORM_DUPLICATE, move |_, _| copy.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        // The map knows 0x100 (as 0x110) but not 0x200.
        e.register(MAP_GET_AT, |e, a| {
            if a[1] == 0x100 {
                e.mem.set_u32(a[2], 0x110);
            }
            u32::from(a[1] == 0x100).into_ret()
        });
        for addr in [
            SET_PARENT_PATH,
            SET_PREVIOUS_PATH,
            ARRAY_COMPACT,
            ARRAY_SET_SIZE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ARRAY_COUNT, |_, _| 2u32.into_ret());
        e.register(ARRAY_ENTRY, |_, a| (children_slot(a[0], a[1])).into_ret());
        e.register(0x5040, |_, a| (a[0] + 1).into_ret());
        e.register_double(ARRAY_TAKE, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x0003_2000 + a[1] * 4, value);
            Ret::default()
        });
        e.mem.map(0x0003_2000, 0x10);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x0058_a870, &args![this, 0x0000_0101u32, 0x7000u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, copy);
        assert_eq!(to(&log, FORM_DUPLICATE), vec![vec![this, 1, 0x7000]]);
        assert_eq!(to(&log, SET_PARENT_PATH), vec![vec![copy, 0x110]]);
        assert_eq!(to(&log, SET_PREVIOUS_PATH), vec![vec![copy, 0x200]]);
        assert_eq!(to(&log, ARRAY_SET_SIZE), vec![vec![0x0004_4000, 2]]);
        assert_eq!(to(&log, 0x5040).len(), 2);
        assert_eq!(e.mem.u32(0x0003_2000), first + 1);
        assert_eq!(e.mem.u32(0x0003_2004), second + 1);
    }

    /// `00877a30` on the test's child array: entry `i` at `array + 4 + 4 * i`.
    fn children_slot(array: u32, index: u32) -> u32 {
        array + 4 + index * 4
    }

    #[test]
    fn duplicate_without_children_only_links() {
        let mut e = engine();
        let this = path(&mut e, &[], 0);
        e.register(FORM_DUPLICATE, |_, _| 0x8000u32.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(MAP_GET_AT, |_, _| 0u32.into_ret());
        e.register(SET_PARENT_PATH, |_, _| Ret::default());
        e.register(SET_PREVIOUS_PATH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0058_a870, &args![this, 0u32, 0x7000u32]).u32(),
            0x8000
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(to(&log, SET_PARENT_PATH), vec![vec![0x8000, 0]]);
        assert!(to(&log, ARRAY_COMPACT).is_empty());
    }

    fn save_engine(swap: bool) -> Engine {
        let mut e = engine();
        for addr in [
            FORM_START_FORM,
            FORM_CLOSE_FORM,
            CONDITIONS_SAVE,
            SWAP_ANAM,
            SWAP_DATA_BYTE,
            FORM_SAVE_DATA,
            ADD_CHUNK_WORD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FILL_MEMORY, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(ARRAY_COUNT, |_, a| (a[0] | 0x0100_0000).into_ret());
        e.register(PARENT_PATH_OF, |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        e.register(PREVIOUS_PATH_OF, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(FORM_FLAG_20_SET, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x20 != 0).into_ret()
        });
        e.register_double(SWAP_ON_SAVE, move |_, _| u32::from(swap).into_ret());
        e.register_double(ADD_CHUNK_DATA, |e, a| {
            let first = e.mem.u32(a[1]);
            let second = e.mem.u32(a[1] + 4);
            e.mem.set_u32(0x0003_3000, first);
            e.mem.set_u32(0x0003_3004, second);
            Ret::default()
        });
        e.mem.map(0x0003_3000, 8);
        e
    }

    #[test]
    fn save_writes_conditions_links_data_and_shots() {
        let mut e = save_engine(true);
        let this = path(&mut e, &[0x40, 0x50], 1);
        let parent = path(&mut e, &[], 0);
        let deleted = path(&mut e, &[], 0);
        let previous = path(&mut e, &[], 0);
        e.mem.set_u32(deleted + 8, 0x20);
        e.mem.set_u32(this + 0x30, parent);
        e.mem.set_u32(this + 0x34, deleted);
        e.mem.set_u32(deleted + 0x34, previous);
        e.call_log = Some(vec![]);
        e.call(0x0058_ac10, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(to(&log, CONDITIONS_SAVE), vec![vec![this + 0x18]]);
        assert_eq!(e.mem.u32(0x0003_3000), parent | 0x0100_0000);
        assert_eq!(e.mem.u32(0x0003_3004), previous | 0x0100_0000);
        assert_eq!(to(&log, SWAP_ANAM).len(), 2);
        assert_eq!(to(&log, SWAP_DATA_BYTE), vec![vec![this + 0x28]; 2]);
        assert_eq!(to(&log, FORM_SAVE_DATA), vec![vec![this, this + 0x28, 1]]);
        assert_eq!(
            to(&log, ADD_CHUNK_WORD),
            vec![vec![CHUNK_SNAM, 0x0100_0040], vec![CHUNK_SNAM, 0x0100_0050]]
        );
        assert_eq!(log.last().unwrap(), &(FORM_CLOSE_FORM, vec![this]));
    }

    #[test]
    fn save_of_a_flagged_path_leaves_the_links_cleared() {
        let mut e = save_engine(false);
        let this = path(&mut e, &[], 1);
        let previous = path(&mut e, &[], 0);
        e.mem.set_u32(this + 8, 0x20);
        e.mem.set_u32(this + 0x34, previous);
        e.mem.set_u32(0x0003_3000, 0xffff_ffff);
        e.mem.set_u32(0x0003_3004, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x0058_ac10, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!((e.mem.u32(0x0003_3000), e.mem.u32(0x0003_3004)), (0, 0));
        assert!(to(&log, PREVIOUS_PATH_OF).is_empty());
        assert!(to(&log, SWAP_ANAM).is_empty());
        assert!(to(&log, ADD_CHUNK_WORD).is_empty());
    }

    #[test]
    fn remove_child_needs_a_child_and_an_array() {
        let mut e = Engine::new();
        e.register(CHILD_INDEX_OF, |_, _| 3u32.into_ret());
        e.register(REMOVE_CHILD_AT, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        e.call(0x0058_b730, &args![this, 0x900u32]);
        e.mem.set_u32(this + 0x2c, 0x4000);
        e.call(0x0058_b730, &args![this, 0u32]);
        e.call(0x0058_b730, &args![this, 0x900u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(to(&log, CHILD_INDEX_OF), vec![vec![this, 0x900]]);
        assert_eq!(to(&log, REMOVE_CHILD_AT), vec![vec![this, 3]]);
    }
}
