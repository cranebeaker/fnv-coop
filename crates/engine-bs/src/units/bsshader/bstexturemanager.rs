//! `BSShader/bstexturemanager.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiPointer` constructor `00633c90` (`thiscall` on the pointer, `RET 4`: the object; it
/// stores it and holds it when not null) and destructor `0045cec0` (`thiscall`, no words; it
/// releases the object when not null).
const NI_POINTER_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// The add-at-head of an `NiTPointerList` of `NiPointer`s (`00c1b4a0`, `thiscall` on the list,
/// `RET 4`: the address of the `NiPointer`).
const LIST_ADD_HEAD_00C1B4A0: u32 = 0x00c1_b4a0;
/// The remove of an `NiTPointerList` of `NiPointer`s (`00b9fcc0`, `thiscall` on the list,
/// `RET 8`: the out `NiPointer`, the address of the `NiPointer` to remove; the engine map's
/// name is `NiTPointerListBase<..,NiPointer<NiProperty>>::Remove`).
const LIST_REMOVE_00B9FCC0: u32 = 0x00b9_fcc0;
/// The add-at-tail `00b58570` (`thiscall` on the list, `RET 4`: the address of the element)
/// and the remove-at-position `00e7d7f0` (`thiscall` on the list, `RET 4`: the address of the
/// position) of the borrowed-texture lists.
const LIST_ADD_TAIL_00B58570: u32 = 0x00b5_8570;
const LIST_REMOVE_POSITION_00E7D7F0: u32 = 0x00e7_d7f0;

/// The texture kinds (the word at the texture's +0x2c) `ReturnRenderedTexture` keeps a pair
/// of lists for, from the switch's byte map at `00b6dcd4` and jump table at `00b6dcbc` (read
/// in Ghidra): the list the texture is added to and the list it is removed from, as offsets
/// in the manager.
fn returned_texture_lists(kind: i32) -> Option<(u32, u32)> {
    match kind {
        0x2b => Some((0, 0x3c)),
        0x11 => Some((0xc, 0x48)),
        9 => Some((0x18, 0x54)),
        0x16 => Some((0x24, 0x60)),
        0x13 => Some((0x30, 0x6c)),
        _ => None,
    }
}

// Translated from 00b6da10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureManager::ReturnRenderedTexture` (engine map). `thiscall`, `RET 4`, one word (the
/// rendered texture). Nothing for a null texture. When the texture's kind (+0x2c, signed) is
/// at least 0: for kinds 0x2b, 0x11, 9, 0x16 and 0x13 the texture is put in an `NiPointer`
/// (the argument's own stack slot, `00633c90`), added at the head of the kind's free list
/// (`00c1b4a0`; lists at +0, +0xc, +0x18, +0x24, +0x30), the pointer released (`0045cec0`),
/// made again and removed from the kind's in-use list (`00b9fcc0` into a stack `NiPointer`;
/// lists at +0x3c, +0x48, +0x54, +0x60, +0x6c), then both pointers released; other kinds do
/// nothing. A negative kind searches the borrowed list at +0x84 for the entry whose first
/// word is the texture; when found the entry is added at the tail of the list at +0x78
/// (`00b58570`) and its node removed from +0x84 (`00e7d7f0`).
// Translation incomplete: 00b6da10 - the C++ exception frame (FS:[0] chain, unwind states 0 to 9 for the NiPointer temporaries) is not translated.
pub fn bs_texture_manager_return_rendered_texture(e: &mut Engine, this: Ptr, texture: Ptr) {
    let base = this.addr();
    if texture.is_null() {
        return;
    }
    let kind = e.mem.i32(texture.addr().wrapping_add(0x2c));
    if kind > -1 {
        let Some((free_list, used_list)) = returned_texture_lists(kind) else {
            return;
        };
        e.with_stack(8, |e, locals| {
            let pointer = locals.addr();
            let removed = pointer.wrapping_add(4);
            e.call(NI_POINTER_CONSTRUCT_00633C90, &args![pointer, texture]);
            e.call(
                LIST_ADD_HEAD_00C1B4A0,
                &args![base.wrapping_add(free_list), pointer],
            );
            e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![pointer]);
            e.call(NI_POINTER_CONSTRUCT_00633C90, &args![pointer, texture]);
            e.call(
                LIST_REMOVE_00B9FCC0,
                &args![base.wrapping_add(used_list), removed, pointer],
            );
            e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![removed]);
            e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![pointer]);
        });
        return;
    }
    let borrowed = base.wrapping_add(0x84);
    let mut node = e.mem.u32(borrowed);
    let mut found = 0u32;
    if node == 0 {
        return;
    }
    loop {
        if found != 0 {
            break;
        }
        let entry = e.mem.u32(node.wrapping_add(8));
        if e.mem.u32(entry) == texture.addr() {
            found = entry;
        } else {
            node = e.mem.u32(node);
        }
        if node == 0 {
            break;
        }
    }
    if found == 0 {
        return;
    }
    e.with_stack(8, |e, locals| {
        let element = locals.addr();
        let position = element.wrapping_add(4);
        e.mem.set_u32(element, found);
        e.mem.set_u32(position, node);
        e.call(
            LIST_ADD_TAIL_00B58570,
            &args![base.wrapping_add(0x78), element],
        );
        e.call(LIST_REMOVE_POSITION_00E7D7F0, &args![borrowed, position]);
    });
}

layout! {
    /// `BSTextureManager::RenderedTextureData` (Xbox PDB): one entry of the manager's
    /// rendered-texture pools. 0x18 bytes on PC (the size `BorrowRenderedTexture` allocates;
    /// the Xbox PDB's `uiBorrowedCount`, `uiReturnedCount` and `lUsage` are not there); the
    /// fields the PC code uses, at the Xbox PDB's offsets.
    pub struct RenderedTextureData: 0x18 {
        /// `spRenderedTexture` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x00 spRenderedTexture: u32,
        /// `iD3DFormat` (Xbox PDB).
        0x04 iD3DFormat: u32,
        /// `iEDRAMFormat` (Xbox PDB).
        0x08 iEDRAMFormat: u32,
        /// `uFlags` (Xbox PDB).
        0x0C uFlags: u32,
        /// `bUsedThisFrame` (Xbox PDB).
        0x10 bUsedThisFrame: u8,
        /// `eType` (Xbox PDB).
        0x14 eType: u32,
    }
}

/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The rendered-texture maker `00b6d170` (unnamed, `thiscall` on the manager, `RET 0x28`).
const MAKE_RENDERED_TEXTURE_00B6D170: u32 = 0x00b6_d170;
/// The list-item allocation `0043a010` (`thiscall` on the list's +8, no stack words).
const LIST_NEW_ITEM_0043A010: u32 = 0x0043_a010;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The D3D format the code puts in place of 0x24 when 0x24's byte is clear.
const FORMAT_0X24_REPLACEMENT: u32 = 0x71;

/// The format check at the start of `BorrowRenderedTexture` (the switch on `format - 0x14`,
/// byte map at `00b6d9ac`, jump table at `00b6d96c`, read in Ghidra): whether the format's
/// byte is set, which skips the replacement; formats without a byte take the replacement
/// test. 0x22 reads `011f91a1`; 0x32 reads `011f91a0` and then `011f91a1` (either set).
fn format_byte_set(e: &Engine, format: u32) -> bool {
    let byte = |address: u32| e.global::<u8>(address) != 0;
    match format {
        0x14 => byte(0x011f_9199),
        0x15 => byte(0x011f_919a),
        0x16 => byte(0x011f_919b),
        0x17 => byte(0x011f_9194),
        0x18 => byte(0x011f_9195),
        0x19 => byte(0x011f_9196),
        0x1a => byte(0x011f_9197),
        0x22 => byte(0x011f_91a1),
        0x24 => byte(0x011f_919e),
        0x32 => {
            let first = byte(0x011f_91a0);
            byte(0x011f_91a1) || first
        }
        0x51 => byte(0x011f_9198),
        0x70 => byte(0x011f_91a2),
        0x71 => byte(0x011f_919d),
        0x72 => byte(0x011f_919c),
        0x74 => byte(0x011f_919f),
        _ => false,
    }
}

/// A pool entry's texture size: the texture's buffer (+0x30) slot +0x94 (`width`) or +0x98,
/// or 0 without a buffer.
fn texture_size(e: &mut Engine, entry: Ptr<RenderedTextureData>, slot: u32) -> u32 {
    let texture = e.get(entry, RenderedTextureData::spRenderedTexture);
    let buffer = e.mem.u32(texture.wrapping_add(0x30));
    if buffer == 0 {
        0
    } else {
        e.vcall(buffer, slot, &args![]).u32()
    }
}

// Translated from 00b6d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureManager::BorrowRenderedTexture` (engine map; the Xbox PDB's ten-parameter
/// overload: renderer, width, height, flags, D3D format, EDRAM format, texture type,
/// multisample type, rendered texture, one more word). `thiscall`, `RET 0x28`. Answers the
/// borrowed texture (`EAX`).
///
/// The renderer's slot +0xc8 is asked twice, once for its slot +0x8c and once for +0x90
/// (each with 0): the default width and height. Unless the format is 0 or the flags have bit
/// 8, a format whose byte is clear (`format_byte_set`) and is 0x24 becomes 0x71. Then the
/// unused pool (+0x78) is searched from its head for an entry whose texture's width (buffer
/// slot +0x94) is the width (or, with a width of 0, the default width), whose height (+0x98)
/// is the height (or, with 0, the default height), with the format at +4, the EDRAM format at
/// +8, for types 9 and 0x16 the type at +0x14, and the same flags apart from bits 0x22. A
/// found entry's node is removed (`00e7d7f0`) and the entry marked used (+0x10). Otherwise
/// an entry is allocated (0x18 bytes from the memory manager; a null allocation is used as it
/// is) with the formats, flags and used byte, and its texture made by `00b6d170` (renderer,
/// width and height, or with either 0 the default width and height, then the formats, flags
/// and the last four words) and assigned to it. The entry is added at the tail of the used
/// pool (+0x84, item from `0043a010`) and its texture answered.
#[allow(clippy::too_many_arguments)]
pub fn bs_texture_manager_borrow_rendered_texture(
    e: &mut Engine,
    this: Ptr,
    renderer: Ptr,
    width: u32,
    height: u32,
    flags: u32,
    format: u32,
    edram_format: u32,
    texture_type: u32,
    multisample: u32,
    rendered_texture: u32,
    last: u32,
) -> u32 {
    let base = this.addr();
    let target = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    let default_width = e.vcall(target, 0x8c, &args![0u32]).u32();
    let target = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    let default_height = e.vcall(target, 0x90, &args![0u32]).u32();
    let mut format = format;
    if format != 0 && flags & 8 == 0 && !format_byte_set(e, format) && format == 0x24 {
        format = FORMAT_0X24_REPLACEMENT;
    }

    let unused = base.wrapping_add(0x78);
    let mut node = e.mem.u32(unused);
    let mut found = 0u32;
    if node != 0 {
        while found == 0 {
            let entry = Ptr::<RenderedTextureData>::new(e.mem.u32(node.wrapping_add(8)));
            if entry_matches(
                e,
                entry,
                [width, height, default_width, default_height],
                flags,
                format,
                edram_format,
                texture_type,
            ) {
                found = entry.addr();
            } else {
                node = e.mem.u32(node);
            }
            if node == 0 {
                break;
            }
        }
    }
    if found != 0 {
        e.with_stack(4, |e, position| {
            e.mem.set_u32(position.addr(), node);
            e.call(LIST_REMOVE_POSITION_00E7D7F0, &args![unused, position]);
        });
        e.set(
            Ptr::<RenderedTextureData>::new(found),
            RenderedTextureData::bUsedThisFrame,
            1,
        );
    } else {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x18u32])
            .u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
        }
        let entry = Ptr::<RenderedTextureData>::new(block);
        found = block;
        e.set(entry, RenderedTextureData::iD3DFormat, format);
        e.set(entry, RenderedTextureData::iEDRAMFormat, edram_format);
        e.set(entry, RenderedTextureData::uFlags, flags);
        e.set(entry, RenderedTextureData::bUsedThisFrame, 1);
        let (made_width, made_height) = if width != 0 && height != 0 {
            (width, height)
        } else {
            (default_width, default_height)
        };
        let made = e
            .call(
                MAKE_RENDERED_TEXTURE_00B6D170,
                &args![
                    this,
                    renderer,
                    made_width,
                    made_height,
                    format,
                    edram_format,
                    flags,
                    texture_type,
                    multisample,
                    rendered_texture,
                    last
                ],
            )
            .u32();
        let old = e.get(entry, RenderedTextureData::spRenderedTexture);
        if old != made {
            if old != 0 {
                let left = e
                    .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                    .u32();
                if left == 0 {
                    e.vcall(old, 4, &args![]);
                }
            }
            e.set(entry, RenderedTextureData::spRenderedTexture, made);
            if made != 0 {
                e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
            }
        }
    }

    let count = base.wrapping_add(0x8c);
    let item = e.call(LIST_NEW_ITEM_0043A010, &args![count]).u32();
    e.mem.set_u32(item.wrapping_add(8), found);
    e.mem.set_u32(item, 0);
    let tail = e.mem.u32(base.wrapping_add(0x88));
    e.mem.set_u32(item.wrapping_add(4), tail);
    let tail = e.mem.u32(base.wrapping_add(0x88));
    if tail != 0 {
        e.mem.set_u32(tail, item);
        e.mem.set_u32(count, e.mem.u32(count).wrapping_add(1));
        e.mem.set_u32(base.wrapping_add(0x88), item);
    } else {
        e.mem.set_u32(count, e.mem.u32(count).wrapping_add(1));
        e.mem.set_u32(base.wrapping_add(0x84), item);
        e.mem.set_u32(base.wrapping_add(0x88), item);
    }
    e.mem.u32(found)
}

/// The match test of `BorrowRenderedTexture`'s search, in the exe's order (each size read
/// again from the texture for its second comparison). `sizes` is (width, height, default
/// width, default height).
fn entry_matches(
    e: &mut Engine,
    entry: Ptr<RenderedTextureData>,
    sizes: [u32; 4],
    flags: u32,
    format: u32,
    edram_format: u32,
    texture_type: u32,
) -> bool {
    let [width, height, default_width, default_height] = sizes;
    if texture_size(e, entry, 0x94) != width
        && (width != 0 || texture_size(e, entry, 0x94) != default_width)
    {
        return false;
    }
    if texture_size(e, entry, 0x98) != height
        && (height != 0 || texture_size(e, entry, 0x98) != default_height)
    {
        return false;
    }
    if e.get(entry, RenderedTextureData::iD3DFormat) != format
        || e.get(entry, RenderedTextureData::iEDRAMFormat) != edram_format
    {
        return false;
    }
    if (texture_type == 9 || texture_type == 0x16)
        && e.get(entry, RenderedTextureData::eType) != texture_type
    {
        return false;
    }
    e.get(entry, RenderedTextureData::uFlags) | 0x22 == flags | 0x22
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b6_da10,
            bs_texture_manager_return_rendered_texture(Ptr, Ptr)
        ),
        entry!(
            0x00b6_d5e0,
            bs_texture_manager_borrow_rendered_texture(
                Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
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

    fn return_rig() -> Engine {
        let mut e = Engine::new();
        for addr in [
            LIST_ADD_HEAD_00C1B4A0,
            LIST_REMOVE_00B9FCC0,
            NI_POINTER_DESTRUCT_0045CEC0,
            LIST_ADD_TAIL_00B58570,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NI_POINTER_CONSTRUCT_00633C90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        // The position's node is checked when the double runs.
        e.register(LIST_REMOVE_POSITION_00E7D7F0, |e, a| {
            e.mem.u32(a[1]).into_ret()
        });
        e
    }

    #[test]
    fn return_rendered_texture_moves_a_kind_texture_between_its_lists() {
        for (kind, free_list, used_list) in [
            (0x2bu32, 0u32, 0x3cu32),
            (0x11, 0xc, 0x48),
            (9, 0x18, 0x54),
            (0x16, 0x24, 0x60),
            (0x13, 0x30, 0x6c),
        ] {
            let mut e = return_rig();
            let this = e.mem.alloc(0x90);
            let texture = e.mem.alloc(0x30);
            e.mem.set_u32(texture + 0x2c, kind);
            e.call_log = Some(vec![]);
            e.call(0x00b6_da10, &args![this, texture]);
            let made = calls(&e, NI_POINTER_CONSTRUCT_00633C90);
            assert_eq!(made.len(), 2);
            let pointer = made[0][0];
            assert_eq!(made[0][1], texture);
            assert_eq!(
                calls(&e, LIST_ADD_HEAD_00C1B4A0),
                vec![vec![this + free_list, pointer]]
            );
            let removed = calls(&e, LIST_REMOVE_00B9FCC0);
            assert_eq!(removed.len(), 1);
            assert_eq!(removed[0][0], this + used_list);
            assert_eq!(removed[0][2], pointer);
            assert_eq!(
                calls(&e, NI_POINTER_DESTRUCT_0045CEC0),
                vec![vec![pointer], vec![removed[0][1]], vec![pointer]]
            );
            let order: Vec<u32> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .skip(1)
                .collect();
            assert_eq!(
                order,
                [
                    NI_POINTER_CONSTRUCT_00633C90,
                    LIST_ADD_HEAD_00C1B4A0,
                    NI_POINTER_DESTRUCT_0045CEC0,
                    NI_POINTER_CONSTRUCT_00633C90,
                    LIST_REMOVE_00B9FCC0,
                    NI_POINTER_DESTRUCT_0045CEC0,
                    NI_POINTER_DESTRUCT_0045CEC0
                ]
            );
        }
    }

    #[test]
    fn return_rendered_texture_ignores_null_and_other_kinds() {
        let mut e = return_rig();
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x00b6_da10, &args![this, 0u32]);
        let texture = e.mem.alloc(0x30);
        for kind in [0u32, 8, 0x10, 0x2a, 0x2c, 0x7fff_ffff] {
            e.mem.set_u32(texture + 0x2c, kind);
            e.call(0x00b6_da10, &args![this, texture]);
        }
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 != 0x00b6_da10)
                .count(),
            0
        );
    }

    #[test]
    fn return_rendered_texture_with_a_negative_kind_hands_back_a_borrowed_entry() {
        let mut e = return_rig();
        let this = e.mem.alloc(0x90);
        let texture = e.mem.alloc(0x30);
        e.mem.set_u32(texture + 0x2c, 0xffff_ffff);
        // Borrowed list: an entry for another texture, then the texture's entry.
        let other_entry = e.mem.alloc(0x18);
        e.mem.set_u32(other_entry, 0x5555);
        let entry = e.mem.alloc(0x18);
        e.mem.set_u32(entry, texture);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 8, entry);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, other_entry);
        e.mem.set_u32(this + 0x84, first);
        // The doubles note what the element and the position words hold.
        let seen = e.mem.alloc(8);
        e.register_double(LIST_ADD_TAIL_00B58570, move |e, a| {
            let element = e.mem.u32(a[1]);
            e.mem.set_u32(seen, element);
            Ret::default()
        });
        e.register_double(LIST_REMOVE_POSITION_00E7D7F0, move |e, a| {
            let node = e.mem.u32(a[1]);
            e.mem.set_u32(seen + 4, node);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b6_da10, &args![this, texture]);
        let tail = calls(&e, LIST_ADD_TAIL_00B58570);
        assert_eq!(tail.len(), 1);
        assert_eq!(tail[0][0], this + 0x78);
        let removed = calls(&e, LIST_REMOVE_POSITION_00E7D7F0);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], this + 0x84);
        assert_eq!(e.mem.u32(seen), entry);
        assert_eq!(e.mem.u32(seen + 4), second);

        // Not in the list (or an empty list): nothing.
        e.mem.set_u32(entry, 0x6666);
        e.call_log = Some(vec![]);
        e.call(0x00b6_da10, &args![this, texture]);
        e.mem.set_u32(this + 0x84, 0);
        e.call(0x00b6_da10, &args![this, texture]);
        assert!(calls(&e, LIST_ADD_TAIL_00B58570).is_empty());
        assert!(calls(&e, LIST_REMOVE_POSITION_00E7D7F0).is_empty());
    }

    const RENDERER_TARGET: u32 = 0x0f00_1000;
    const TARGET_WIDTH: u32 = 0x0f00_208c;
    const TARGET_HEIGHT: u32 = 0x0f00_2090;
    const BUFFER_WIDTH: u32 = 0x0f00_3094;
    const BUFFER_HEIGHT: u32 = 0x0f00_3098;
    const MADE_TEXTURE: u32 = 0x0f00_7000;

    /// A manager, a renderer whose slot +0xc8 answers a target with a default size of
    /// 640 x 480, and buffers whose width and height are their words at +8 and +0xc.
    fn borrow_rig() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        let mut renderer_slots = vec![0u32; 0x33];
        renderer_slots[0x32] = 0x0f00_10c8;
        e.put_vtable(0x0f00_0000, &renderer_slots);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, 0x0f00_0000);
        e.register(0x0f00_10c8, |_, _| RENDERER_TARGET.into_ret());
        let mut target_slots = vec![0u32; 0x25];
        target_slots[0x23] = TARGET_WIDTH;
        target_slots[0x24] = TARGET_HEIGHT;
        e.put_vtable(0x0f00_2000, &target_slots);
        e.map(RENDERER_TARGET, 4);
        e.mem.set_u32(RENDERER_TARGET, 0x0f00_2000);
        e.register(TARGET_WIDTH, |_, _| 640u32.into_ret());
        e.register(TARGET_HEIGHT, |_, _| 480u32.into_ret());
        let mut buffer_slots = vec![0u32; 0x27];
        buffer_slots[0x25] = BUFFER_WIDTH;
        buffer_slots[0x26] = BUFFER_HEIGHT;
        e.put_vtable(0x0f00_3000, &buffer_slots);
        e.register(BUFFER_WIDTH, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(BUFFER_HEIGHT, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(LIST_REMOVE_POSITION_00E7D7F0, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(LIST_NEW_ITEM_0043A010, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(MAKE_RENDERED_TEXTURE_00B6D170, |_, _| {
            MADE_TEXTURE.into_ret()
        });
        e.map(MADE_TEXTURE, 0x40);
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        let this = e.mem.alloc(0x90);
        (e, this, renderer)
    }

    /// A pool entry whose texture's buffer is `width` x `height`.
    fn pool_entry(e: &mut Engine, width: u32, height: u32, format: u32, flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, 0x0f00_3000);
        e.mem.set_u32(buffer + 8, width);
        e.mem.set_u32(buffer + 0xc, height);
        let texture = e.mem.alloc(0x40);
        e.mem.set_u32(texture + 0x30, buffer);
        let entry = e.mem.alloc(0x18);
        e.mem.set_u32(entry, texture);
        e.mem.set_u32(entry + 4, format);
        e.mem.set_u32(entry + 8, 0x1b);
        e.mem.set_u32(entry + 0xc, flags);
        entry
    }

    /// Puts `entries` in the unused pool (+0x78), in order; answers their nodes.
    fn fill_unused(e: &mut Engine, this: u32, entries: &[u32]) -> Vec<u32> {
        let nodes: Vec<u32> = entries.iter().map(|_| e.mem.alloc(0xc)).collect();
        for (i, (node, entry)) in nodes.iter().zip(entries).enumerate() {
            e.mem.set_u32(node + 8, *entry);
            if let Some(next) = nodes.get(i + 1) {
                e.mem.set_u32(*node, *next);
            }
        }
        e.mem.set_u32(this + 0x78, nodes[0]);
        nodes
    }

    fn borrow(e: &mut Engine, this: u32, renderer: u32, size: (u32, u32), format: u32) -> u32 {
        e.call(
            0x00b6_d5e0,
            &args![
                this, renderer, size.0, size.1, 0x20u32, format, 0x1bu32, 3u32, 0u32, 0x44u32,
                0x55u32
            ],
        )
        .u32()
    }

    #[test]
    fn borrow_rendered_texture_takes_a_matching_unused_entry() {
        let (mut e, this, renderer) = borrow_rig();
        let wrong_size = pool_entry(&mut e, 320, 240, 0x15, 0);
        // Flags differ only in bits 0x22: still a match.
        let right = pool_entry(&mut e, 640, 480, 0x15, 0x02);
        let nodes = fill_unused(&mut e, this, &[wrong_size, right]);
        let seen = e.mem.alloc(4);
        e.register_double(LIST_REMOVE_POSITION_00E7D7F0, move |e, a| {
            let node = e.mem.u32(a[1]);
            e.mem.set_u32(seen, node);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        // A width and height of 0 take the target's 640 x 480.
        let texture = borrow(&mut e, this, renderer, (0, 0), 0x15);
        assert_eq!(texture, e.mem.u32(right));
        assert_eq!(calls(&e, LIST_REMOVE_POSITION_00E7D7F0)[0][0], this + 0x78);
        assert_eq!(e.mem.u32(seen), nodes[1]);
        assert_eq!(e.mem.u8(right + 0x10), 1);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        // Added at the tail of the empty used pool.
        let item = e.mem.u32(this + 0x84);
        assert_eq!(e.mem.u32(this + 0x88), item);
        assert_eq!(e.mem.u32(this + 0x8c), 1);
        assert_eq!(e.mem.u32(item + 8), right);
        assert_eq!(calls(&e, LIST_NEW_ITEM_0043A010), vec![vec![this + 0x8c]]);
        assert_eq!(calls(&e, TARGET_WIDTH), vec![vec![RENDERER_TARGET, 0]]);
        assert_eq!(calls(&e, TARGET_HEIGHT), vec![vec![RENDERER_TARGET, 0]]);
    }

    #[test]
    fn borrow_rendered_texture_makes_a_new_entry_when_nothing_matches() {
        for (size, made_size) in [((256u32, 128u32), (256u32, 128u32)), ((256, 0), (640, 480))] {
            let (mut e, this, renderer) = borrow_rig();
            let other = pool_entry(&mut e, 256, 128, 0x16, 0);
            fill_unused(&mut e, this, &[other]);
            // A used pool with one item already.
            let first = e.mem.alloc(0xc);
            e.mem.set_u32(this + 0x84, first);
            e.mem.set_u32(this + 0x88, first);
            e.mem.set_u32(this + 0x8c, 1);
            e.call_log = Some(vec![]);
            let texture = borrow(&mut e, this, renderer, size, 0x15);
            assert_eq!(texture, MADE_TEXTURE);
            assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x18]]);
            let made = calls(&e, MAKE_RENDERED_TEXTURE_00B6D170);
            assert_eq!(
                made,
                vec![vec![
                    this,
                    renderer,
                    made_size.0,
                    made_size.1,
                    0x15,
                    0x1b,
                    0x20,
                    3,
                    0,
                    0x44,
                    0x55
                ]]
            );
            let item = e.mem.u32(this + 0x88);
            let entry = e.mem.u32(item + 8);
            assert_eq!(e.mem.u32(entry), MADE_TEXTURE);
            assert_eq!(e.mem.u32(MADE_TEXTURE + 4), 1);
            assert_eq!(e.mem.u32(entry + 4), 0x15);
            assert_eq!(e.mem.u32(entry + 8), 0x1b);
            assert_eq!(e.mem.u32(entry + 0xc), 0x20);
            assert_eq!(e.mem.u8(entry + 0x10), 1);
            assert_eq!(e.mem.u32(first), item);
            assert_eq!(e.mem.u32(item + 4), first);
            assert_eq!(e.mem.u32(this + 0x84), first);
            assert_eq!(e.mem.u32(this + 0x8c), 2);
            assert!(calls(&e, LIST_REMOVE_POSITION_00E7D7F0).is_empty());
        }
    }

    #[test]
    fn borrow_rendered_texture_replaces_format_0x24_only_when_its_byte_is_clear() {
        for (byte, flags, want) in [(0u8, 0x20u32, 0x71u32), (1, 0x20, 0x24), (0, 0x28, 0x24)] {
            let (mut e, this, renderer) = borrow_rig();
            e.set_global(0x011f_919e, byte);
            e.call_log = Some(vec![]);
            e.call(
                0x00b6_d5e0,
                &args![
                    this, renderer, 16u32, 16u32, flags, 0x24u32, 0x1bu32, 3u32, 0u32, 0u32, 0u32
                ],
            );
            assert_eq!(calls(&e, MAKE_RENDERED_TEXTURE_00B6D170)[0][4], want);
        }
        let e = borrow_rig().0;
        assert!(!format_byte_set(&e, 0x23));
        let mut e = e;
        e.set_global(0x011f_91a0, 1u8);
        assert!(format_byte_set(&e, 0x32));
        assert!(!format_byte_set(&e, 0x22));
    }

    #[test]
    fn borrow_rendered_texture_checks_the_type_for_types_9_and_0x16() {
        let (mut e, this, renderer) = borrow_rig();
        let entry = pool_entry(&mut e, 16, 16, 0x15, 0x20);
        e.mem.set_u32(entry + 0x14, 9);
        fill_unused(&mut e, this, &[entry]);
        e.call_log = Some(vec![]);
        // Type 0x16 against an entry of type 9: no match, a new one is made.
        e.call(
            0x00b6_d5e0,
            &args![
                this, renderer, 16u32, 16u32, 0x20u32, 0x15u32, 0x1bu32, 0x16u32, 0u32, 0u32, 0u32
            ],
        );
        assert_eq!(calls(&e, MAKE_RENDERED_TEXTURE_00B6D170).len(), 1);
        // Type 9: it matches.
        e.call_log = Some(vec![]);
        let texture = e
            .call(
                0x00b6_d5e0,
                &args![
                    this, renderer, 16u32, 16u32, 0x20u32, 0x15u32, 0x1bu32, 9u32, 0u32, 0u32, 0u32
                ],
            )
            .u32();
        assert_eq!(texture, e.mem.u32(entry));
        assert!(calls(&e, MAKE_RENDERED_TEXTURE_00B6D170).is_empty());
    }
}
