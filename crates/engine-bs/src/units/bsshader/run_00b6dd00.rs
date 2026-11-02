//! `BSShader/run_00b6dd00` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The 0xb0-byte object the shader manager's set-up (`00b57420`) keeps at `011f91a8`. Its
//! methods include `BSTextureManager::BorrowRenderedTexture` (engine map, `00b6d5e0`) and
//! its fields match the Xbox PDB's `BSTextureManager` with five lists per array on PC (the
//! Xbox has four): the unused aliased texture lists (`NiTPointerList`s at +0, +0xc, .. +0x30),
//! the used ones (+0x3c .. +0x6c), four more lists (+0x78 .. +0x9c) and two `NiPointer`s
//! (+0xa8, +0xac; `spSharedDepthBuffer` and `spSharedMSDepthBuffer` on the Xbox). The names
//! are described, not asserted: the engine map names neither function here.

#[allow(unused_imports)]
use crate::prelude::*;

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The renderer (`011f9508`), passed to the texture queries.
const RENDERER_011F9508: u32 = 0x011f_9508;

/// An `NiPointer`'s release: the count at +4 decremented, the object deleted through its
/// vtable slot +4 when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// An `NiPointer` member's assignment: nothing when the same, else the old one released, the
/// new one stored and its count incremented when not null.
fn assign(e: &mut Engine, member: u32, object: u32) {
    let old = e.mem.u32(member);
    if old != object {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(member, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
}

/// The compiler's `eh vector constructor iterator` (`00ec782f`, five stack words cleaned by
/// the callee: array, element size, count, constructor, destructor), with the list
/// constructor `00b998e0` and destructor `00b5c7c0`.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const LIST_CONSTRUCT_00B998E0: u32 = 0x00b9_98e0;
const LIST_DESTRUCT_00B5C7C0: u32 = 0x00b5_c7c0;
/// `NiTPointerListBase<..>::RemoveAll` (engine map, `00e74d40`, `thiscall` on a list).
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// The texture query `00b6c2c0` (`stdcall`, `RET 0x20`: eight stack words, the renderer, a
/// texture kind and six out-parameter addresses; it does not read `ECX`).
const TEXTURE_QUERY_00B6C2C0: u32 = 0x00b6_c2c0;
/// The object at `011f4748` whose vtable slot +0x94 (three words) the code calls.
const OBJECT_011F4748: u32 = 0x011f_4748;
/// The word cleared around each `00a9fe90` call and set to 1 after it (`011bf400`).
const FLAG_011BF400: u32 = 0x011b_f400;
/// The maker `00a9fe90` (`cdecl`, five words).
const MAKE_00A9FE90: u32 = 0x00a9_fe90;
/// The `NiPointer` assignment `0066b0d0` (`thiscall` on the member, `RET 4`).
const NI_POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0;

/// The words the constructor clears after the two arrays, in the exe's order.
const CLEARED_WORDS_00B6DD80: [u32; 14] = [
    0x80, 0x78, 0x7c, 0x8c, 0x84, 0x88, 0x98, 0x90, 0x94, 0xa4, 0x9c, 0xa0, 0xa8, 0xac,
];

/// The constructor's stack locals, by offset in the block: six out-parameter words of the
/// texture queries (`[ESP+0x14]` to `[ESP+0x30]` in the exe after its prologue) and the word
/// it keeps the second query's format result in (`[ESP+0x34]`).
const LOCAL_14: u32 = 0;
const LOCAL_18: u32 = 4;
const LOCAL_1C: u32 = 8;
const LOCAL_20: u32 = 0xc;
const LOCAL_24: u32 = 0x10;
const LOCAL_28: u32 = 0x14;
const LOCAL_2C: u32 = 0x18;
const LOCAL_30: u32 = 0x1c;
const LOCAL_34: u32 = 0x20;

/// The object's format word for a surface: `object_011f4748->vtable[0x94](format, 0x20, 8)`
/// with the format at +0xc of the surface's +0x10 (0 when that is null).
fn surface_format_word(e: &mut Engine, surface: u32) -> u32 {
    let inner = e.mem.u32(surface.wrapping_add(0x10));
    let format = if inner != 0 {
        e.mem.u32(inner.wrapping_add(0xc))
    } else {
        0
    };
    let object = e.global::<u32>(OBJECT_011F4748);
    e.vcall(object, 0x94, &args![format, 0x20u32, 8u32]).u32()
}

// Translated from 00b6dd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6dd00`): the constructor of the texture manager the
/// set-up keeps at `011f91a8` (see the module comment). `thiscall`, no stack words.
///
/// Constructs the two arrays of five lists (`00ec782f` with `00b998e0`/`00b5c7c0`), clears
/// the four lists and two pointers after them, then asks the texture query `00b6c2c0` for
/// kind 0x2b on the renderer (`011f9508`); maps its format answer 2, 4, 8 to 1, 2, 3 (else
/// 0); gets the renderer's device (vtable +0xc8). When the device's slot +0xb4 answers a
/// buffer, that one is assigned to +0xa8. Otherwise a buffer is made with `00a9fe90` (the
/// word at `011bf400` cleared around it) from the device's sizes (slots +0x8c, +0x90, each
/// at least the query's answer when the mapped value is 0) and the format word of the
/// device's surface (slot +0xb0, then `011f4748`'s slot +0x94); with a mapped value of 0 that
/// buffer goes to +0xa8 (`0066b0d0`) and is given to the device (slot +0xac); otherwise the
/// device gets the multisampled one made with the mapped value, and a second query (kind 4)
/// sizes the buffer assigned to +0xa8. Finally +0xac is released and cleared and every list
/// of both arrays is emptied (`00e74d40`). Answers `this`.
// Translation incomplete: 00b6dd80 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7) is not translated.
pub fn fn_00b6dd80(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for array in [base, base.wrapping_add(0x3c)] {
        e.call(
            VECTOR_CONSTRUCT,
            &args![
                array,
                0xcu32,
                5u32,
                LIST_CONSTRUCT_00B998E0,
                LIST_DESTRUCT_00B5C7C0
            ],
        );
    }
    for offset in CLEARED_WORDS_00B6DD80 {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let depth_member = base.wrapping_add(0xa8);
    e.with_stack(0x24, |e, frame| {
        let local = |offset: u32| frame.addr().wrapping_add(offset);
        e.call(
            TEXTURE_QUERY_00B6C2C0,
            &args![
                renderer,
                0x2bu32,
                local(LOCAL_1C),
                local(LOCAL_20),
                local(LOCAL_28),
                local(LOCAL_14),
                local(LOCAL_24),
                local(LOCAL_18)
            ],
        );
        let device = e.vcall(renderer, 0xc8, &args![]).u32();
        let samples = match e.mem.u32(local(LOCAL_18)) {
            2 => 1,
            4 => 2,
            8 => 3,
            _ => 0,
        };
        e.mem.set_u32(local(LOCAL_14), samples);
        if e.vcall(device, 0xb4, &args![]).u32() != 0 {
            let buffer = e.vcall(device, 0xb4, &args![]).u32();
            assign(e, depth_member, buffer);
        } else if e.mem.u32(local(LOCAL_14)) == 0 {
            let width = e.vcall(device, 0x8c, &args![0u32]).u32();
            let wanted = e.mem.u32(local(LOCAL_1C));
            e.mem
                .set_u32(local(LOCAL_18), if wanted > width { wanted } else { width });
            let height = e.vcall(device, 0x90, &args![0u32]).u32();
            let wanted = e.mem.u32(local(LOCAL_20));
            e.mem.set_u32(
                local(LOCAL_14),
                if wanted > height { wanted } else { height },
            );
            let surface = e.vcall(device, 0xb0, &args![0u32]).u32();
            let format = surface_format_word(e, surface);
            let width = e.mem.u32(local(LOCAL_18));
            let height = e.mem.u32(local(LOCAL_14));
            e.set_global(FLAG_011BF400, 0u32);
            let buffer = e
                .call(MAKE_00A9FE90, &args![width, height, renderer, format, 0u32])
                .u32();
            e.call(NI_POINTER_ASSIGN_0066B0D0, &args![depth_member, buffer]);
            e.set_global(FLAG_011BF400, 1u32);
            let held = e.mem.u32(depth_member);
            e.vcall(device, 0xac, &args![held]);
        } else {
            let surface = e.vcall(device, 0xb0, &args![0u32]).u32();
            let format = surface_format_word(e, surface);
            let samples = e.mem.u32(local(LOCAL_14));
            e.set_global(FLAG_011BF400, 0u32);
            e.mem.set_u32(local(LOCAL_34), format);
            let height = e.vcall(device, 0x90, &args![0u32]).u32();
            let width = e.vcall(device, 0x8c, &args![0u32]).u32();
            let buffer = e
                .call(
                    MAKE_00A9FE90,
                    &args![width, height, renderer, format, samples],
                )
                .u32();
            e.set_global(FLAG_011BF400, 1u32);
            e.vcall(device, 0xac, &args![buffer]);
            e.call(
                TEXTURE_QUERY_00B6C2C0,
                &args![
                    renderer,
                    4u32,
                    local(LOCAL_2C),
                    local(LOCAL_30),
                    local(LOCAL_28),
                    local(LOCAL_14),
                    local(LOCAL_24),
                    local(LOCAL_18)
                ],
            );
            let first = e.mem.u32(local(LOCAL_1C));
            let second = e.mem.u32(local(LOCAL_2C));
            let width = if first > second { first } else { second };
            let first = e.mem.u32(local(LOCAL_20));
            let second = e.mem.u32(local(LOCAL_30));
            let height = if first > second { first } else { second };
            let format = e.mem.u32(local(LOCAL_34));
            e.set_global(FLAG_011BF400, 0u32);
            let buffer = e
                .call(MAKE_00A9FE90, &args![width, height, renderer, format, 0u32])
                .u32();
            e.call(NI_POINTER_ASSIGN_0066B0D0, &args![depth_member, buffer]);
            e.set_global(FLAG_011BF400, 1u32);
        }
    });
    let held = e.mem.u32(base.wrapping_add(0xac));
    if held != 0 {
        release(e, held);
        e.mem.set_u32(base.wrapping_add(0xac), 0);
    }
    for list in 0..5u32 {
        let unused = base.wrapping_add(0xc * list);
        e.call(LIST_REMOVE_ALL, &args![unused]);
        e.call(LIST_REMOVE_ALL, &args![unused.wrapping_add(0x3c)]);
    }
    this
}

/// `BSTextureManager::BorrowRenderedTexture` (engine map, `00b6d5e0`: `thiscall`, ten stack
/// words) and `BSTextureManager::ReturnRenderedTexture` (engine map, `00b6da10`: `thiscall`,
/// one word).
const BORROW_RENDERED_TEXTURE: u32 = 0x00b6_d5e0;
const RETURN_RENDERED_TEXTURE: u32 = 0x00b6_da10;
/// The settings bytes the warm-up depends on (`011f941e`, `011f941f`, `011f9420`).
const SETTING_011F941E: u32 = 0x011f_941e;
const SETTING_011F941F: u32 = 0x011f_941f;
/// `011f9420`.
const SETTING_011F9420: u32 = 0x011f_9420;
/// The object kept at `011f91ac` (`00b97a80`'s), whose word at +0x318 the code reads.
const OBJECT_011F91AC: u32 = 0x011f_91ac;
/// The list operations of the texture lists: `00c1b4a0` (`thiscall` on a list, `RET 4`: the
/// address of an `NiPointer`) and `00b9e7c0` (`thiscall` on a list, `RET 4`: the address of
/// an `NiPointer` it fills).
const LIST_ADD_00C1B4A0: u32 = 0x00c1_b4a0;
const LIST_TAKE_00B9E7C0: u32 = 0x00b9_e7c0;

/// `00b6e450`'s stack locals used as out parameters, by offset in the block (`[ESP+0x14]` to
/// `[ESP+0x28]` in the exe after its prologue).
const SLOT_14: u32 = 0;
const SLOT_18: u32 = 4;
const SLOT_1C: u32 = 8;
const SLOT_20: u32 = 0xc;
const SLOT_24: u32 = 0x10;
const SLOT_28: u32 = 0x14;

/// The texture query `00b6c2c0` on `renderer` for `kind` with six out parameters (in the
/// order the query takes them), then `BorrowRenderedTexture` with the renderer, the first,
/// second, fifth, third and fourth answers, the kind, the sixth answer and two zeros.
fn borrow_of_kind(e: &mut Engine, this: Ptr, renderer: u32, kind: u32, outs: [u32; 6]) -> u32 {
    e.call(
        TEXTURE_QUERY_00B6C2C0,
        &args![renderer, kind, outs[0], outs[1], outs[2], outs[3], outs[4], outs[5]],
    );
    let answer: Vec<u32> = outs.iter().map(|out| e.mem.u32(*out)).collect();
    e.call(
        BORROW_RENDERED_TEXTURE,
        &args![
            this, renderer, answer[0], answer[1], answer[4], answer[2], answer[3], kind, answer[5],
            0u32, 0u32
        ],
    )
    .u32()
}

/// One of the four list kinds `00b6e450` sets up: the unused list (+0xc, +0x18, +0x24,
/// +0x30), its count word (the list's +8), the used list it is moved to (+0x3c more), the
/// texture kind and the flag bit of its (never set) temporary.
struct ListKind {
    unused: u32,
    kind: u32,
    flag: u32,
    clears_flag: bool,
}

const LIST_KINDS_00B6E450: [ListKind; 4] = [
    ListKind {
        unused: 0xc,
        kind: 0x11,
        flag: 4,
        clears_flag: true,
    },
    ListKind {
        unused: 0x18,
        kind: 9,
        flag: 8,
        clears_flag: true,
    },
    ListKind {
        unused: 0x24,
        kind: 0x16,
        flag: 0x10,
        clears_flag: true,
    },
    ListKind {
        unused: 0x30,
        kind: 0x13,
        flag: 0x20,
        clears_flag: false,
    },
];

// Translated from 00b6e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6dd00`): the texture manager's warm-up. `thiscall`,
/// `RET 4`, one word (the renderer used for the list kinds).
///
/// Borrows rendered textures and returns them at once: when the byte at `011f941e` is set,
/// three 1x1 (format 0x24), a 4x4, 16x16 and 64x64 (0x26) texture, kinds 2, 0, 4 through the
/// query `00b6c2c0`, and kind 1 when `011f941f` and `011f9420` are both set (else none);
/// always kind 3 and kind 6 twice; every one not null is returned (`00b6da10`). Then the
/// object at `011f91ac`: its word at +0x318 gives (+0x30) the object whose +0x24 gets the
/// call of its vtable slot +0x94 (a null word reads a local cleared to 0 instead, as the exe
/// does). Then for each of the four list kinds: an empty unused list gets a texture borrowed
/// for its kind on the argument's renderer; otherwise its head texture is added to the used
/// list (`00c1b4a0`) and taken off the unused list (`00b9e7c0`, the taken pointer released).
/// A texture got this way is added to the unused list, its kind stored at +0x2c, and the
/// call of vtable slot +0x94 made on the +0x24 of its +0x30 object.
///
/// The temporaries' flag bits other than bit 0 are never set, so their releases do not run;
/// they are translated as the exe has them.
// Translation incomplete: 00b6e450 - the C++ exception frame (FS:[0] chain, unwind states 0x2a to 0x40 around the list calls) is not translated.
pub fn fn_00b6e450(e: &mut Engine, this: Ptr, renderer: u32) {
    let base = this.addr();
    e.with_stack(0x18, |e, frame| {
        let slot = |offset: u32| frame.addr().wrapping_add(offset);
        let mut flags = 0u32;
        let mut borrowed = [0u32; 13];
        let mut count = 0usize;
        if e.global::<u8>(SETTING_011F941E) != 0 {
            for (size, format) in [
                (1u32, 0x24u32),
                (1, 0x24),
                (1, 0x24),
                (4, 0x26),
                (0x10, 0x26),
                (0x40, 0x26),
            ] {
                let main = e.global::<u32>(RENDERER_011F9508);
                borrowed[count] = e
                    .call(
                        BORROW_RENDERED_TEXTURE,
                        &args![
                            this,
                            main,
                            size,
                            size,
                            format,
                            0x71u32,
                            0u32,
                            0xffff_ffffu32,
                            0u32,
                            0u32,
                            0u32
                        ],
                    )
                    .u32();
                count += 1;
            }
            let main = e.global::<u32>(RENDERER_011F9508);
            borrowed[count] = borrow_of_kind(
                e,
                this,
                main,
                2,
                [
                    slot(SLOT_14),
                    slot(SLOT_18),
                    slot(SLOT_20),
                    slot(SLOT_24),
                    slot(SLOT_28),
                    slot(SLOT_1C),
                ],
            );
            count += 1;
            let later_outs = [
                slot(SLOT_1C),
                slot(SLOT_24),
                slot(SLOT_28),
                slot(SLOT_18),
                slot(SLOT_20),
                slot(SLOT_14),
            ];
            for kind in [0u32, 4] {
                let main = e.global::<u32>(RENDERER_011F9508);
                borrowed[count] = borrow_of_kind(e, this, main, kind, later_outs);
                count += 1;
            }
            borrowed[count] =
                if e.global::<u8>(SETTING_011F941F) != 0 && e.global::<u8>(SETTING_011F9420) != 0 {
                    let main = e.global::<u32>(RENDERER_011F9508);
                    borrow_of_kind(e, this, main, 1, later_outs)
                } else {
                    0
                };
            count = 10;
        }
        let later_outs = [
            slot(SLOT_1C),
            slot(SLOT_24),
            slot(SLOT_28),
            slot(SLOT_18),
            slot(SLOT_20),
            slot(SLOT_14),
        ];
        for kind in [3u32, 6, 6] {
            let main = e.global::<u32>(RENDERER_011F9508);
            borrowed[count] = borrow_of_kind(e, this, main, kind, later_outs);
            count += 1;
        }
        for texture in borrowed.iter().take(count) {
            if *texture != 0 {
                e.call(RETURN_RENDERED_TEXTURE, &args![this, *texture]);
            }
        }

        let holder = e
            .mem
            .u32(e.global::<u32>(OBJECT_011F91AC).wrapping_add(0x318));
        let (source, kept) = if holder != 0 {
            (holder.wrapping_add(0x30), e.mem.u32(slot(SLOT_1C)))
        } else {
            e.mem.set_u32(slot(SLOT_1C), 0);
            flags = 1;
            (slot(SLOT_1C), 0)
        };
        let object = e.mem.u32(source);
        if flags & 1 != 0 {
            flags &= !1;
            if kept != 0 {
                release(e, kept);
            }
        }
        let inner = e.mem.u32(object.wrapping_add(0x24));
        e.vcall(inner, 0x94, &args![]);

        for list in &LIST_KINDS_00B6E450 {
            let unused = base.wrapping_add(list.unused);
            let texture = if e.mem.u32(unused.wrapping_add(8)) == 0 {
                borrow_of_kind(e, this, renderer, list.kind, later_outs)
            } else {
                let head = e.mem.u32(unused);
                let texture = e.mem.u32(head.wrapping_add(8));
                e.mem.set_u32(slot(SLOT_14), texture);
                if texture != 0 {
                    e.call(
                        IMPORT_INTERLOCKED_INCREMENT,
                        &args![texture.wrapping_add(4)],
                    );
                }
                e.call(
                    LIST_ADD_00C1B4A0,
                    &args![unused.wrapping_add(0x3c), slot(SLOT_14)],
                );
                if texture != 0 {
                    release(e, texture);
                }
                e.call(LIST_TAKE_00B9E7C0, &args![unused, slot(SLOT_18)]);
                let taken = e.mem.u32(slot(SLOT_18));
                if taken != 0 {
                    release(e, taken);
                }
                texture
            };
            if texture == 0 {
                continue;
            }
            e.mem.set_u32(slot(SLOT_14), texture);
            e.call(
                IMPORT_INTERLOCKED_INCREMENT,
                &args![texture.wrapping_add(4)],
            );
            e.call(LIST_ADD_00C1B4A0, &args![unused, slot(SLOT_14)]);
            release(e, texture);
            let flagged = flags & list.flag != 0;
            e.mem.set_u32(texture.wrapping_add(0x2c), list.kind);
            let owner = e.mem.u32(texture.wrapping_add(0x30));
            if flagged {
                if list.clears_flag {
                    flags &= !list.flag;
                }
                let temporary = e.mem.u32(slot(SLOT_14));
                if temporary != 0 {
                    release(e, temporary);
                }
            }
            let inner = e.mem.u32(owner.wrapping_add(0x24));
            e.vcall(inner, 0x94, &args![]);
        }
    });
}

/// The `NiPointer` constructor from a pointer `00633c90` (`thiscall` on the pointer, one word)
/// and the `NiPointer` destructor `0045cec0` (`thiscall` on the pointer).
const NI_POINTER_FROM_00633C90: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;

// Translated from 00b6e110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6dd00`): a rendered texture of a kind, `thiscall` on the
/// texture manager with five stack words (`RET 0x14`): the renderer, the kind, extra flags, a
/// word that must be 0 (else 0 is answered at once) and a last word. Kinds 0x2b, 0x11, 9, 0x16
/// and 0x13 (the switch's byte map at `00b6e428` and jump table at `00b6e410`, read in
/// Ghidra) take the first texture of their free list (+0x0, +0xc, +0x18, +0x24, +0x30) when
/// its count (+8 of the list) is not 0: an `NiPointer` made from it (`00633c90`, in the
/// fourth argument's slot) is added to the in-use list (+0x3c, +0x48, +0x54, +0x60, +0x6c,
/// `00c1b4a0`) and released (`0045cec0`), the free list's first entry is taken into another
/// stack slot (`00b9e7c0`) and released, and the texture is answered. Every other case asks
/// `00b6c2c0` (`stdcall`: renderer, kind, then the addresses of width, height, format, a word,
/// the flags and the fourth argument's slot) and answers
/// `BSTextureManager::BorrowRenderedTexture` (`00b6d5e0`) with the renderer, width, height,
/// the flags or'ed with the extra flags, format, the word, the kind, the slot's word, 0 and
/// the last word.
// Translation incomplete: 00b6e110 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the temporary pointers) is not translated.
pub fn fn_00b6e110(
    e: &mut Engine,
    this: Ptr,
    renderer: u32,
    kind: u32,
    extra_flags: u32,
    must_be_zero: u32,
    last: u32,
) -> u32 {
    if must_be_zero != 0 {
        return 0;
    }
    let base = this.addr();
    // The frame: locals at +0x10..+0x24, the argument slots at +0x38..+0x48.
    e.with_stack(0x4c, |e, frame| {
        let frame = frame.addr();
        let argument_slot = |index: u32| frame + 0x34 + 4 * index;
        for (index, word) in [renderer, kind, extra_flags, must_be_zero, last]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(argument_slot(index as u32 + 1), *word);
        }
        // The arms of the switch: the free list, the in-use list and the argument whose slot
        // receives the taken pointer (0: a local).
        let taken_case = match kind {
            0x2b => Some((0x0u32, 0x3cu32, 2u32)),
            0x11 => Some((0xc, 0x48, 1)),
            9 => Some((0x18, 0x54, 5)),
            0x16 => Some((0x24, 0x60, 3)),
            0x13 => Some((0x30, 0x6c, 0)),
            _ => None,
        };
        if let Some((free_list, used_list, slot)) = taken_case {
            if e.mem.u32(base + free_list + 8) != 0 {
                let head = e.mem.u32(base + free_list);
                let texture = e.mem.u32(head + 8);
                let temporary = argument_slot(4);
                e.call(NI_POINTER_FROM_00633C90, &args![temporary, texture]);
                e.call(LIST_ADD_00C1B4A0, &args![base + used_list, temporary]);
                e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![temporary]);
                let taken = if slot == 0 {
                    frame + 0x10
                } else {
                    argument_slot(slot)
                };
                e.call(LIST_TAKE_00B9E7C0, &args![base + free_list, taken]);
                e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![taken]);
                return texture;
            }
        }
        let (width, height, format, word, flags) = (
            frame + 0x24,
            frame + 0x20,
            frame + 0x18,
            frame + 0x14,
            frame + 0x1c,
        );
        let slot = argument_slot(4);
        e.call(
            TEXTURE_QUERY_00B6C2C0,
            &args![renderer, kind, width, height, format, word, flags, slot],
        );
        let made_flags = e.mem.u32(flags) | e.mem.u32(argument_slot(3));
        let words = [
            base,
            renderer,
            e.mem.u32(width),
            e.mem.u32(height),
            made_flags,
            e.mem.u32(format),
            e.mem.u32(word),
            kind,
            e.mem.u32(slot),
            0,
            e.mem.u32(argument_slot(5)),
        ];
        e.call(BORROW_RENDERED_TEXTURE, &words).u32()
    })
}

/// The memory manager object and its deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The list removal `00e7d7f0` (`thiscall` on a list, `RET 4`: the address of the position,
/// which it moves on).
const LIST_REMOVE_AT_00E7D7F0: u32 = 0x00e7_d7f0;
/// The list operation `00b9e850` (`thiscall` on a list, `RET 4`: the address of an
/// `NiPointer` it fills with a taken entry).
const LIST_TAKE_00B9E850: u32 = 0x00b9_e850;
/// The list operation `00a5a310` (`thiscall` on a list, `RET 4`: the address of an
/// `NiPointer` it adds).
const LIST_ADD_00A5A310: u32 = 0x00a5_a310;
/// The rendered texture's target lookup `00b6b260` (`thiscall` on the texture).
const RENDERED_TARGET_00B6B260: u32 = 0x00b6_b260;
/// The `LinkObject` jump target `00ba8a70` (`thiscall`, one word).
const LINK_OBJECT_00BA8A70: u32 = 0x00ba_8a70;
/// The byte at `011a2fe4` that skips the destructor's trim of the +0x90 list.
const FLAG_011A2FE4: u32 = 0x011a_2fe4;
/// The list-node pool: the free node chain (`011c5f58`) and its spin lock (`011c5f80`, the
/// owner word; `011c5f84`, the count), entered with `0040fbf0` (`thiscall`, `RET 4`).
const NODE_POOL_FREE_011C5F58: u32 = 0x011c_5f58;
const NODE_POOL_LOCK_011C5F80: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT_011C5F84: u32 = 0x011c_5f84;
const SPIN_LOCK_ENTER_0040FBF0: u32 = 0x0040_fbf0;
/// `NiTPointerListBase<..>::RemoveAll` (engine map, `00b71300`, `thiscall` on a list).
const LIST_REMOVE_ALL_00B71300: u32 = 0x00b7_1300;
/// The compiler's `eh vector destructor iterator` (`00ec5fce`, four stack words cleaned by
/// the callee: array, element size, count, destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The thread block's word at `+0x2b4` (the allocation context), 0x20 while `00b6ee70` runs.
const TLS_ALLOCATION_CONTEXT: u32 = 0x2b4;

// Translated from 00b6dd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6dd00`): `thiscall`, two stack words (an unread word and
/// a byte). Walks the list at `+0x78`; with the byte 0, each entry whose byte at `+0x10` is 0
/// and whose word at `+0xc` lacks bit 9 (`0x200`) is removed (`00e7d7f0`, which moves the
/// position on), its `NiPointer` at `+0` released and the entry freed; the other entries get
/// their `+0x10` byte cleared. With a non-zero byte nothing changes.
pub fn fn_00b6dd00(e: &mut Engine, this: Ptr, _unused_1: u32, keep: u8) {
    let list = this.addr().wrapping_add(0x78);
    e.with_stack(4, |e, position| {
        let position = position.addr();
        let mut node = e.mem.u32(list);
        e.mem.set_u32(position, node);
        while node != 0 {
            let entry = e.mem.u32(node.wrapping_add(8));
            if keep == 0 {
                if e.mem.u8(entry.wrapping_add(0x10)) == 0
                    && e.mem.u32(entry.wrapping_add(0xc)) & 0x200 == 0
                {
                    e.call(LIST_REMOVE_AT_00E7D7F0, &args![list, position]);
                    let object = e.mem.u32(entry);
                    if object != 0 {
                        release(e, object);
                    }
                    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, entry]);
                    node = e.mem.u32(position);
                    continue;
                }
                e.mem.set_u8(entry.wrapping_add(0x10), 0);
            }
            node = e.mem.u32(node);
            e.mem.set_u32(position, node);
        }
    });
}

/// `00b6ee70`'s stack slots, by offset in its block: the two argument slots the exe reuses as
/// `NiPointer`s, the six out parameters of the texture query (in the order the query takes
/// them: `[ESP+0x34]`, `+0x30`, `+0x28`, `+0x24`, `+0x2c`, `+0x20` after the prologue) and
/// the `NiPointer` local (`[ESP+0x38]`).
const FRAME_RENDERER: u32 = 0;
const FRAME_COUNT: u32 = 4;
const FRAME_QUERY_OUTS: [u32; 6] = [0x8, 0xc, 0x10, 0x14, 0x18, 0x1c];
const FRAME_POINTER: u32 = 0x20;

// Translated from 00b6ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6dd00`): `thiscall`, two stack words (a renderer and a
/// count). Brings the list at `+0x90` (count at `+0x98`) to `count + 1` textures, unless the
/// word at `+0xa4` is non-zero, with the thread's allocation context (TLS `+0x2b4`) at 0x20
/// meanwhile (restored on every way out). Too many: each one over is taken off the list
/// (`00b9e850`), its target (`00b6b260`) gets slot +0xac with 0, it is returned
/// (`BSTextureManager::ReturnRenderedTexture`) and released. Too few: each one missing is the
/// head of the free list at `+0` when its count (`+8`) is not 0 (moved to the in-use list at
/// `+0x3c` and taken off the free list, as `00b6e110` does), else a texture of kind 0x2b
/// borrowed for the renderer; its target gets slot +0xac with the word at `+0xa8`, and it is
/// added to the `+0x90` list (`00a5a310`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b6ee70 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the temporary pointers) is not translated
pub fn fn_00b6ee70(e: &mut Engine, this: Ptr, renderer: u32, count: u32) {
    let base = this.addr();
    let context = e.tls().wrapping_add(TLS_ALLOCATION_CONTEXT);
    let saved = e.mem.u32(context);
    e.mem.set_u32(context, 0x20);
    if e.mem.u32(base.wrapping_add(0xa4)) != 0 {
        e.mem.set_u32(context, saved);
        return;
    }
    e.with_stack(0x24, |e, frame| {
        let frame = frame.addr();
        let renderer_slot = frame + FRAME_RENDERER;
        let count_slot = frame + FRAME_COUNT;
        let pointer = frame + FRAME_POINTER;
        e.mem.set_u32(renderer_slot, renderer);
        e.mem.set_u32(count_slot, count);
        let wanted = count.wrapping_add(1);
        let have = e.mem.u32(base.wrapping_add(0x98));
        if have > wanted {
            let mut left = have - wanted;
            loop {
                e.call(
                    LIST_TAKE_00B9E850,
                    &args![base.wrapping_add(0x90), count_slot],
                );
                let texture = e.mem.u32(count_slot);
                let target = e.call(RENDERED_TARGET_00B6B260, &args![texture]).u32();
                e.vcall(target, 0xac, &args![0u32]);
                let texture = e.mem.u32(count_slot);
                e.call(RETURN_RENDERED_TEXTURE, &args![this, texture]);
                let texture = e.mem.u32(count_slot);
                if texture != 0 {
                    release(e, texture);
                }
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        } else if have < wanted {
            let mut left = wanted - have;
            loop {
                let texture = if e.mem.u32(base.wrapping_add(8)) != 0 {
                    let head = e.mem.u32(base);
                    let texture = e.mem.u32(head.wrapping_add(8));
                    e.call(NI_POINTER_FROM_00633C90, &args![count_slot, texture]);
                    e.call(
                        LIST_ADD_00C1B4A0,
                        &args![base.wrapping_add(0x3c), count_slot],
                    );
                    e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![count_slot]);
                    e.call(LIST_TAKE_00B9E7C0, &args![base, renderer_slot]);
                    e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![renderer_slot]);
                    texture
                } else {
                    let outs = FRAME_QUERY_OUTS.map(|offset| frame + offset);
                    borrow_of_kind(e, this, renderer, 0x2b, outs)
                };
                e.mem.set_u32(pointer, texture);
                if texture != 0 {
                    e.call(
                        IMPORT_INTERLOCKED_INCREMENT,
                        &args![texture.wrapping_add(4)],
                    );
                }
                let target = e.call(RENDERED_TARGET_00B6B260, &args![texture]).u32();
                let word = e.mem.u32(base.wrapping_add(0xa8));
                e.vcall(target, 0xac, &args![word]);
                e.call(LIST_ADD_00A5A310, &args![base.wrapping_add(0x90), pointer]);
                if texture != 0 {
                    release(e, texture);
                }
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        }
    });
    e.mem.set_u32(context, saved);
}

/// The destructor's removal of the head of the list at `list` (head, tail, count): the
/// entry's `NiPointer` (`+0` of the entry the node holds) released and cleared, the node
/// unlinked and given back to the node pool under its spin lock, the count lowered, the
/// pointer released again when set, and the entry freed.
fn remove_head_entry(e: &mut Engine, list: u32) {
    let node = e.mem.u32(list);
    let entry = e.mem.u32(node.wrapping_add(8));
    let object = e.mem.u32(entry);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(entry, 0);
    }
    let node = e.mem.u32(list);
    let next = e.mem.u32(node);
    e.mem.set_u32(list, next);
    if next != 0 {
        e.mem.set_u32(next.wrapping_add(4), 0);
    } else {
        e.mem.set_u32(list.wrapping_add(4), 0);
    }
    e.mem.set_u32(node.wrapping_add(8), 0);
    e.call(
        SPIN_LOCK_ENTER_0040FBF0,
        &args![NODE_POOL_LOCK_011C5F80, 0u32],
    );
    e.mem.set_u32(node.wrapping_add(4), 0);
    let free = e.mem.u32(NODE_POOL_FREE_011C5F58);
    e.mem.set_u32(node, free);
    let lock_count = e.mem.u32(NODE_POOL_LOCK_COUNT_011C5F84).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT_011C5F84, lock_count);
    e.mem.set_u32(NODE_POOL_FREE_011C5F58, node);
    if lock_count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK_011C5F80, 0);
    }
    let count = e.mem.u32(list.wrapping_add(8)).wrapping_sub(1);
    e.mem.set_u32(list.wrapping_add(8), count);
    let object = e.mem.u32(entry);
    if object != 0 {
        release(e, object);
    }
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, entry]);
}

// Translated from 00b6f0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6dd00`): the texture manager's destructor (`thiscall`).
/// Empties the lists at `+0x9c` and `+0x90` (`00b9e850`, each taken pointer released) and the
/// entry lists at `+0x84` and `+0x78` (see `remove_head_entry`); unless the byte at
/// `011a2fe4` is set, trims the `+0x90` list ([`fn_00b6ee70`] with the object at `011f4748`
/// and 0); empties the five free lists at `+0` .. `+0x30` (`00b9e7c0`, each taken pointer
/// released); releases and clears the pointers at `+0xa8` and `+0xac` (then their member
/// destructors release them again when set); removes all from the lists at `+0x9c`, `+0x90`
/// (`00e74d40`), `+0x84` and `+0x78` (`00b71300`); and destroys both arrays of five lists
/// (`00ec5fce` with `00b5c7c0`, the in-use array at `+0x3c` first).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b6f0b0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the members) is not translated
pub fn fn_00b6f0b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0xc, |e, frame| {
        let frame = frame.addr();
        for (list, count, slot) in [(0x9cu32, 0xa4u32, frame), (0x90, 0x98, frame + 4)] {
            if e.mem.u32(base.wrapping_add(count)) == 0 {
                continue;
            }
            loop {
                e.call(LIST_TAKE_00B9E850, &args![base.wrapping_add(list), slot]);
                let taken = e.mem.u32(slot);
                if taken != 0 {
                    release(e, taken);
                }
                if e.mem.u32(base.wrapping_add(count)) == 0 {
                    break;
                }
            }
        }
        for list in [0x84u32, 0x78] {
            let list = base.wrapping_add(list);
            if e.mem.u32(list.wrapping_add(8)) == 0 {
                continue;
            }
            loop {
                remove_head_entry(e, list);
                if e.mem.u32(list.wrapping_add(8)) == 0 {
                    break;
                }
            }
        }
        if e.mem.u8(FLAG_011A2FE4) == 0 {
            let renderer = e.mem.u32(OBJECT_011F4748);
            fn_00b6ee70(e, this, renderer, 0);
        }
        let taken = frame + 8;
        for index in 0..5u32 {
            let list = base.wrapping_add(index * 0xc);
            if e.mem.u32(list.wrapping_add(8)) == 0 {
                continue;
            }
            loop {
                e.call(LIST_TAKE_00B9E7C0, &args![list, taken]);
                let object = e.mem.u32(taken);
                if object != 0 {
                    release(e, object);
                }
                if e.mem.u32(list.wrapping_add(8)) == 0 {
                    break;
                }
            }
        }
    });
    for offset in [0xa8u32, 0xac] {
        let member = base.wrapping_add(offset);
        let object = e.mem.u32(member);
        if object != 0 {
            release(e, object);
            e.mem.set_u32(member, 0);
        }
    }
    for offset in [0xacu32, 0xa8] {
        let object = e.mem.u32(base.wrapping_add(offset));
        if object != 0 {
            release(e, object);
        }
    }
    e.call(LIST_REMOVE_ALL, &args![base.wrapping_add(0x9c)]);
    e.call(LIST_REMOVE_ALL, &args![base.wrapping_add(0x90)]);
    e.call(LIST_REMOVE_ALL_00B71300, &args![base.wrapping_add(0x84)]);
    e.call(LIST_REMOVE_ALL_00B71300, &args![base.wrapping_add(0x78)]);
    for array in [base.wrapping_add(0x3c), base] {
        e.call(
            VECTOR_DESTRUCT,
            &args![array, 0xcu32, 5u32, LIST_DESTRUCT_00B5C7C0],
        );
    }
}

// Translated from 00b6f460 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `00ba8a70` (vtable slot +0x50, `LinkObject`, of the vtable at `010b9d28` and
/// others): `thiscall`, one word (the stream), passed on unchanged.
pub fn fn_00b6f460(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(LINK_OBJECT_00BA8A70, &args![this, stream]);
}

// Translated from 00b6f470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers 1 (`MOV EAX,1; RET`). It sits in many vtables; it
/// reads neither `ECX` nor the stack.
pub fn fn_00b6f470(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

// Translated from 00b6f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: slot +0xc8 of `BSShaderNoLightingProperty`'s vtable
/// `010ae670` (`SetClampD3D` in the Xbox PDB's slot list; `thiscall`, one word). Stores 0 in
/// `TextureClampMode` (`+0x6c` on PC, Xbox PDB +0x74) when the word is 3, otherwise 3.
pub fn fn_00b6f480(e: &mut Engine, this: Ptr, mode: u32) {
    let value: u32 = if mode.wrapping_sub(3) != 0 { 3 } else { 0 };
    e.mem.set_u32(this.addr().wrapping_add(0x6c), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_f480, fn_00b6f480(Ptr, u32)),
        entry!(0x00b6_e110, fn_00b6e110(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b6_dd80, fn_00b6dd80(Ptr) -> Ptr),
        entry!(0x00b6_e450, fn_00b6e450(Ptr, u32)),
        entry!(0x00b6_dd00, fn_00b6dd00(Ptr, u32, u8)),
        entry!(0x00b6_ee70, fn_00b6ee70(Ptr, u32, u32)),
        entry!(0x00b6_f0b0, fn_00b6f0b0(Ptr)),
        entry!(0x00b6_f460, fn_00b6f460(Ptr, u32)),
        entry!(0x00b6_f470, fn_00b6f470(Ptr) -> u32),
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

    fn counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    const DEVICE_SLOT_8C: u32 = 0x0f00_108c;
    const DEVICE_SLOT_90: u32 = 0x0f00_1090;
    const DEVICE_SLOT_AC: u32 = 0x0f00_10ac;
    const DEVICE_SLOT_B0: u32 = 0x0f00_10b0;
    const DEVICE_SLOT_B4: u32 = 0x0f00_10b4;
    const FORMAT_SLOT_94: u32 = 0x0f00_3094;

    /// A renderer whose device (vtable +0xc8) has sizes 800 x 600, a surface whose format
    /// is 0x15, and slot +0xb4 answering `existing`; the query answers its six out words as
    /// `first + 0..6` (the format word, sixth, is `format`).
    fn constructor_rig(existing: u32, format: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(0x011b_f000, 0x1000);
        counting(&mut e);
        let renderer = e.mem.alloc(8);
        let mut renderer_slots = vec![0u32; 0x33];
        renderer_slots[0xc8 / 4] = 0x0f00_20c8;
        e.put_vtable(0x0f00_2000, &renderer_slots);
        e.mem.set_u32(renderer, 0x0f00_2000);
        e.set_global(RENDERER_011F9508, renderer);
        let device = e.mem.alloc(8);
        let mut device_slots = vec![0u32; 0x2e];
        for (i, slot) in device_slots.iter_mut().enumerate() {
            *slot = 0x0f00_1000 + 4 * i as u32;
        }
        e.put_vtable(0x0f00_0100, &device_slots);
        e.mem.set_u32(device, 0x0f00_0100);
        e.register_double(0x0f00_20c8, move |_, _| device.into_ret());
        e.register_double(DEVICE_SLOT_B4, move |_, _| existing.into_ret());
        e.register(DEVICE_SLOT_8C, |_, _| 800u32.into_ret());
        e.register(DEVICE_SLOT_90, |_, _| 600u32.into_ret());
        e.register(DEVICE_SLOT_AC, |_, _| Ret::default());
        let surface = e.mem.alloc(0x14);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(surface + 0x10, inner);
        e.mem.set_u32(inner + 0xc, 0x15);
        e.register_double(DEVICE_SLOT_B0, move |_, _| surface.into_ret());
        let formats = e.mem.alloc(8);
        let mut format_slots = vec![0u32; 0x26];
        format_slots[0x94 / 4] = FORMAT_SLOT_94;
        e.put_vtable(0x0f00_3000, &format_slots);
        e.mem.set_u32(formats, 0x0f00_3000);
        e.set_global(OBJECT_011F4748, formats);
        e.register(FORMAT_SLOT_94, |_, a| (a[1] + 0x100).into_ret());
        e.register_double(TEXTURE_QUERY_00B6C2C0, move |e, a| {
            // (kind 0x2b: 1000 x 500; kind 4: 700 x 900), the format word last.
            let (width, height) = if a[1] == 0x2b {
                (1000, 500)
            } else {
                (700, 900)
            };
            e.mem.set_u32(a[2], width);
            e.mem.set_u32(a[3], height);
            e.mem.set_u32(a[7], format);
            Ret::default()
        });
        e.register_double(MAKE_00A9FE90, |e, a| {
            assert_eq!(e.global::<u32>(FLAG_011BF400), 0);
            (0x0bf0_0000 + a[4]).into_ret()
        });
        e.register(NI_POINTER_ASSIGN_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        for plain in [VECTOR_CONSTRUCT, LIST_REMOVE_ALL] {
            e.register(plain, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xb0);
        (e, this)
    }

    #[test]
    fn fn_00b6dd80_with_no_multisampling_makes_the_buffer_from_the_device() {
        let (mut e, this) = constructor_rig(0, 1);
        let renderer = e.global::<u32>(RENDERER_011F9508);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_dd80, &args![this]).u32(), this);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                vec![
                    this,
                    0xc,
                    5,
                    LIST_CONSTRUCT_00B998E0,
                    LIST_DESTRUCT_00B5C7C0
                ],
                vec![
                    this + 0x3c,
                    0xc,
                    5,
                    LIST_CONSTRUCT_00B998E0,
                    LIST_DESTRUCT_00B5C7C0
                ]
            ]
        );
        let query = calls(&e, TEXTURE_QUERY_00B6C2C0);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0].len(), 8);
        assert_eq!(&query[0][..2], &[renderer, 0x2b]);
        // Width max(1000, 800), height max(500, 600), format word 0x15 + 0x100.
        assert_eq!(
            calls(&e, MAKE_00A9FE90),
            vec![vec![1000, 600, renderer, 0x115, 0]]
        );
        assert_eq!(calls(&e, FORMAT_SLOT_94)[0][1..], [0x15, 0x20, 8]);
        assert_eq!(e.mem.u32(this + 0xa8), 0x0bf0_0000);
        assert_eq!(calls(&e, DEVICE_SLOT_AC)[0][1], 0x0bf0_0000);
        assert_eq!(e.global::<u32>(FLAG_011BF400), 1);
        let lists: Vec<u32> = calls(&e, LIST_REMOVE_ALL).iter().map(|c| c[0]).collect();
        let want: Vec<u32> = (0..5)
            .flat_map(|i| [this + 0xc * i, this + 0x3c + 0xc * i])
            .collect();
        assert_eq!(lists, want);
    }

    #[test]
    fn fn_00b6dd80_with_multisampling_makes_two_buffers() {
        let (mut e, this) = constructor_rig(0, 4);
        let renderer = e.global::<u32>(RENDERER_011F9508);
        e.call_log = Some(vec![]);
        e.call(0x00b6_dd80, &args![this]);
        assert_eq!(
            calls(&e, MAKE_00A9FE90),
            vec![
                vec![800, 600, renderer, 0x115, 2],
                vec![1000, 900, renderer, 0x115, 0]
            ]
        );
        assert_eq!(calls(&e, DEVICE_SLOT_AC)[0][1], 0x0bf0_0002);
        assert_eq!(e.mem.u32(this + 0xa8), 0x0bf0_0000);
        let query = calls(&e, TEXTURE_QUERY_00B6C2C0);
        assert_eq!(query.len(), 2);
        assert_eq!(query[1].len(), 8);
        assert_eq!(query[1][1], 4);
    }

    #[test]
    fn fn_00b6dd80_takes_the_device_buffer_when_there_is_one() {
        let (mut e, this) = constructor_rig(0, 8);
        let buffer = e.mem.alloc(8);
        e.register_double(DEVICE_SLOT_B4, move |_, _| buffer.into_ret());
        e.mem.set_u32(this + 0xac, 0xdead_beef);
        e.call_log = Some(vec![]);
        e.call(0x00b6_dd80, &args![this]);
        assert!(calls(&e, MAKE_00A9FE90).is_empty());
        assert_eq!(calls(&e, DEVICE_SLOT_B4).len(), 2);
        assert_eq!(e.mem.u32(this + 0xa8), buffer);
        assert_eq!(e.mem.u32(buffer + 4), 1);
        // The constructor cleared +0xac before, so nothing is released there.
        assert_eq!(e.mem.u32(this + 0xac), 0);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    /// The warm-up's rig: every borrow answers a new texture (+0x2c kind, +0x30 owner whose
    /// +0x24 object has a slot +0x94); the query writes `kind * 0x10 + i` into its i-th out.
    fn warm_up_rig() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        counting(&mut e);
        e.set_global(RENDERER_011F9508, 0x0777_0000u32);
        let inner = e.mem.alloc(8);
        let mut slots = vec![0u32; 0x26];
        slots[0x94 / 4] = 0x0f00_5094;
        e.put_vtable(0x0f00_5000, &slots);
        e.mem.set_u32(inner, 0x0f00_5000);
        e.register(0x0f00_5094, |_, _| Ret::default());
        e.put_vtable(0x0f00_6000, &[0, 0x0f00_6004]);
        e.register(0x0f00_6004, |_, _| Ret::default());
        let owner = e.mem.alloc(0x28);
        e.mem.set_u32(owner + 0x24, inner);
        let holder = e.mem.alloc(0x40);
        e.mem.set_u32(holder + 0x30, owner);
        let image_space = e.mem.alloc(0x31c);
        e.mem.set_u32(image_space + 0x318, holder);
        e.set_global(OBJECT_011F91AC, image_space);
        e.register_double(TEXTURE_QUERY_00B6C2C0, |e, a| {
            for i in 0..6 {
                e.mem.set_u32(a[2 + i as usize], a[1] * 0x10 + i);
            }
            Ret::default()
        });
        e.register_double(BORROW_RENDERED_TEXTURE, move |e, _| {
            let texture = e.mem.alloc(0x34);
            e.mem.set_u32(texture, 0x0f00_6000);
            e.mem.set_u32(texture + 0x30, owner);
            texture.into_ret()
        });
        e.register(RETURN_RENDERED_TEXTURE, |_, _| Ret::default());
        e.register(LIST_ADD_00C1B4A0, |_, _| Ret::default());
        // The take fills its out pointer (nothing taken here).
        e.register(LIST_TAKE_00B9E7C0, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let this = e.mem.alloc(0xb0);
        (e, this, inner)
    }

    #[test]
    fn fn_00b6e450_warms_up_every_kind_and_fills_empty_lists() {
        let (mut e, this, inner) = warm_up_rig();
        e.set_global(SETTING_011F941E, 1u8);
        e.set_global(SETTING_011F941F, 1u8);
        e.set_global(SETTING_011F9420, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x00b6_e450, &args![this, 0x0888_0000u32]);
        let borrows = calls(&e, BORROW_RENDERED_TEXTURE);
        // 6 fixed, kinds 2, 0, 4, 1, then 3, 6, 6, then one per empty list.
        assert_eq!(borrows.len(), 17);
        assert_eq!(
            borrows[0],
            vec![this, 0x0777_0000, 1, 1, 0x24, 0x71, 0, 0xffff_ffff, 0, 0, 0]
        );
        assert_eq!(borrows[5][2..5], [0x40, 0x40, 0x26]);
        // Kind 2: the first query order (outs 0, 1, 4, 2, 3 then 5).
        assert_eq!(
            borrows[6],
            vec![
                this,
                0x0777_0000,
                0x20,
                0x21,
                0x24,
                0x22,
                0x23,
                2,
                0x25,
                0,
                0
            ]
        );
        assert_eq!(
            borrows[7],
            vec![this, 0x0777_0000, 0, 1, 4, 2, 3, 0, 5, 0, 0]
        );
        let kinds: Vec<u32> = borrows[6..].iter().map(|b| b[7]).collect();
        assert_eq!(kinds, vec![2, 0, 4, 1, 3, 6, 6, 0x11, 9, 0x16, 0x13]);
        assert_eq!(borrows[13][1], 0x0888_0000);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE).len(), 13);
        // The holder's object first, then one per list texture.
        assert_eq!(calls(&e, 0x0f00_5094), vec![vec![inner]; 5]);
        let added = calls(&e, LIST_ADD_00C1B4A0);
        let lists: Vec<u32> = added.iter().map(|c| c[0]).collect();
        assert_eq!(
            lists,
            vec![this + 0xc, this + 0x18, this + 0x24, this + 0x30]
        );
        assert!(calls(&e, LIST_TAKE_00B9E7C0).is_empty());
    }

    #[test]
    fn fn_00b6e450_without_the_extra_textures_reuses_listed_ones() {
        let (mut e, this, _) = warm_up_rig();
        e.set_global(SETTING_011F941F, 1u8);
        // The first unused list holds one texture (held once by the list).
        let texture = e.mem.alloc(0x34);
        e.mem.set_u32(texture, 0x0f00_6000);
        e.mem.set_u32(texture + 4, 1);
        let owner = e.mem.alloc(0x28);
        let inner = e.mem.alloc(8);
        e.mem.set_u32(inner, 0x0f00_5000);
        e.mem.set_u32(owner + 0x24, inner);
        e.mem.set_u32(texture + 0x30, owner);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 8, texture);
        e.mem.set_u32(this + 0xc, item);
        e.mem.set_u32(this + 0x14, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b6_e450, &args![this, 0x0888_0000u32]);
        let borrows = calls(&e, BORROW_RENDERED_TEXTURE);
        let kinds: Vec<u32> = borrows.iter().map(|b| b[7]).collect();
        assert_eq!(kinds, vec![3, 6, 6, 9, 0x16, 0x13]);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE).len(), 3);
        let added = calls(&e, LIST_ADD_00C1B4A0);
        assert_eq!(added[0][0], this + 0x48);
        assert_eq!(added[1][0], this + 0xc);
        assert_eq!(calls(&e, LIST_TAKE_00B9E7C0)[0][0], this + 0xc);
        assert_eq!(e.mem.u32(texture + 0x2c), 0x11);
        assert_eq!(e.mem.u32(texture + 4), 1);
        assert_eq!(calls(&e, 0x0f00_5094)[1], vec![inner]);
    }
}

#[cfg(test)]
mod manager_lifetime_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Reference counting on the word at +4, objects deleted through 0x0f00_0004 (vtable
    /// 0x0f00_1000), frees recorded.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        let mut slots = [0u32; 0x2c];
        slots[1] = 0x0f00_0004;
        slots[0xac / 4] = 0x0f00_00ac;
        e.put_vtable(0x0f00_1000, &slots);
        for addr in [0x0f00_0004, 0x0f00_00ac, MEMORY_DEALLOCATE] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, references: u32) -> u32 {
        let o = e.mem.alloc(0x40);
        e.mem.set_u32(o, 0x0f00_1000);
        e.mem.set_u32(o + 4, references);
        o
    }

    #[test]
    fn fn_00b6dd00_removes_unflagged_entries_and_clears_the_rest() {
        let mut e = rig();
        // The removal unlinks the node at the position and moves the position on.
        e.register(LIST_REMOVE_AT_00E7D7F0, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            let previous = e.mem.u32(node + 4);
            if previous == 0 {
                e.mem.set_u32(a[0], next);
            } else {
                e.mem.set_u32(previous, next);
            }
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let this = e.mem.alloc(0xb0);
        let texture = object(&mut e, 1);
        let mut entries = vec![];
        let mut nodes = vec![];
        for (byte, flags) in [(0u8, 0u32), (1, 0), (0, 0x200)] {
            let entry = e.mem.alloc(0x14);
            e.mem.set_u32(entry, texture);
            e.mem.set_u8(entry + 0x10, byte);
            e.mem.set_u32(entry + 0xc, flags);
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node + 8, entry);
            entries.push(entry);
            nodes.push(node);
        }
        e.mem.set_u32(this + 0x78, nodes[0]);
        e.mem.set_u32(nodes[0], nodes[1]);
        e.mem.set_u32(nodes[1], nodes[2]);
        e.mem.set_u32(nodes[1] + 4, nodes[0]);
        e.mem.set_u32(nodes[2] + 4, nodes[1]);
        // A non-zero byte: nothing changes.
        e.call(0x00b6_dd00, &args![this, 0u32, 1u32]);
        assert!(calls(&e, LIST_REMOVE_AT_00E7D7F0).is_empty());
        assert_eq!(e.mem.u8(entries[1] + 0x10), 1);
        e.call(0x00b6_dd00, &args![this, 0u32, 0x100u32]);
        assert_eq!(calls(&e, LIST_REMOVE_AT_00E7D7F0).len(), 1);
        assert_eq!(e.mem.u32(this + 0x78), nodes[1]);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![texture]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, entries[0]]]
        );
        assert_eq!(e.mem.u8(entries[1] + 0x10), 0);
    }

    fn manager(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xb0);
        e.register(RENDERED_TARGET_00B6B260, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        for addr in [
            RETURN_RENDERED_TEXTURE,
            NI_POINTER_FROM_00633C90,
            NI_POINTER_DESTRUCT_0045CEC0,
            LIST_ADD_00C1B4A0,
            LIST_TAKE_00B9E7C0,
            TEXTURE_QUERY_00B6C2C0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // The +0x90 and +0x9c lists as counts: taking hands out the word at the list's +4.
        e.register(LIST_TAKE_00B9E850, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count - 1);
            let texture = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[1], texture);
            Ret::default()
        });
        e.register(LIST_ADD_00A5A310, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count + 1);
            Ret::default()
        });
        this
    }

    fn texture_with_target(e: &mut Engine, references: u32) -> u32 {
        let texture = object(e, references);
        let target = object(e, 1);
        e.mem.set_u32(texture + 0x10, target);
        texture
    }

    #[test]
    fn fn_00b6ee70_returns_the_extra_textures() {
        let mut e = rig();
        let this = manager(&mut e);
        let texture = texture_with_target(&mut e, 3);
        e.mem.set_u32(this + 0x94, texture);
        e.mem.set_u32(this + 0x98, 4);
        let context = e.tls() + TLS_ALLOCATION_CONTEXT;
        e.mem.set_u32(context, 7);
        e.call(0x00b6_ee70, &args![this, 0x5000u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0x98), 2);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE).len(), 2);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE)[0], vec![this, texture]);
        let target = e.mem.u32(texture + 0x10);
        assert_eq!(calls(&e, 0x0f00_00ac), vec![vec![target, 0]; 2]);
        assert_eq!(e.mem.u32(texture + 4), 1);
        assert_eq!(e.mem.u32(context), 7);
    }

    #[test]
    fn fn_00b6ee70_fills_up_from_the_free_list_or_by_borrowing() {
        let mut e = rig();
        let this = manager(&mut e);
        let listed = texture_with_target(&mut e, 1);
        let borrowed = texture_with_target(&mut e, 1);
        e.register_double(BORROW_RENDERED_TEXTURE, move |_, _| borrowed.into_ret());
        // The free list holds one texture; taking it empties the list.
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, listed);
        e.mem.set_u32(this, node);
        e.mem.set_u32(this + 8, 1);
        e.register(LIST_TAKE_00B9E7C0, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.mem.set_u32(this + 0xa8, 0x77);
        let context = e.tls() + TLS_ALLOCATION_CONTEXT;
        e.register(TEXTURE_QUERY_00B6C2C0, |e, _| {
            let context = e.tls() + 0x2b4;
            assert_eq!(e.mem.u32(context), 0x20);
            Ret::default()
        });
        e.mem.set_u32(context, 9);
        e.call(0x00b6_ee70, &args![this, 0x5000u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0x98), 2);
        assert_eq!(calls(&e, LIST_ADD_00C1B4A0)[0][0], this + 0x3c);
        assert_eq!(calls(&e, NI_POINTER_FROM_00633C90)[0][1], listed);
        assert_eq!(calls(&e, LIST_TAKE_00B9E7C0)[0][0], this);
        let borrows = calls(&e, BORROW_RENDERED_TEXTURE);
        assert_eq!(borrows.len(), 1);
        assert_eq!((borrows[0][1], borrows[0][7]), (0x5000, 0x2b));
        let targets: Vec<Vec<u32>> = calls(&e, 0x0f00_00ac);
        assert_eq!(
            targets,
            vec![
                vec![e.mem.u32(listed + 0x10), 0x77],
                vec![e.mem.u32(borrowed + 0x10), 0x77]
            ]
        );
        assert_eq!(e.mem.u32(listed + 4), 1);
        assert_eq!(e.mem.u32(context), 9);
    }

    #[test]
    fn fn_00b6ee70_does_nothing_with_the_word_at_0xa4() {
        let mut e = rig();
        let this = manager(&mut e);
        e.mem.set_u32(this + 0xa4, 1);
        e.mem.set_u32(this + 0x98, 5);
        e.call(0x00b6_ee70, &args![this, 0x5000u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0x98), 5);
        assert!(calls(&e, LIST_TAKE_00B9E850).is_empty());
    }

    #[test]
    fn fn_00b6f0b0_empties_everything() {
        let mut e = rig();
        let this = manager(&mut e);
        e.map(0x011a_2000, 0x1000);
        e.map(0x011c_5000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.set_global(FLAG_011A2FE4, 1u8);
        for addr in [
            SPIN_LOCK_ENTER_0040FBF0,
            LIST_REMOVE_ALL,
            LIST_REMOVE_ALL_00B71300,
            VECTOR_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(LIST_TAKE_00B9E7C0, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count - 1);
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        // The +0x9c list (count at +0xa4) is taken like the +0x90 one.
        let texture = object(&mut e, 10);
        e.mem.set_u32(this + 0x94, texture);
        e.mem.set_u32(this + 0xa0, texture);
        e.mem.set_u32(this + 0xa4, 1);
        e.mem.set_u32(this + 0x98, 2);
        // One entry in the +0x84 list.
        let held = object(&mut e, 1);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, held);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, entry);
        e.mem.set_u32(this + 0x84, node);
        e.mem.set_u32(this + 0x88, node);
        e.mem.set_u32(this + 0x8c, 1);
        e.mem.set_u32(NODE_POOL_FREE_011C5F58, 0x4444);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT_011C5F84, 1);
        e.mem.set_u32(NODE_POOL_LOCK_011C5F80, 0x99);
        // Two free lists hold entries.
        e.mem.set_u32(this + 0x8, 2);
        e.mem.set_u32(this + 0x24 + 8, 1);
        let shared = object(&mut e, 1);
        e.mem.set_u32(this + 0xa8, shared);
        e.call(0x00b6_f0b0, &args![this]);
        assert_eq!(e.mem.u32(texture + 4), 7);
        assert_eq!(e.mem.u32(this + 0xa4), 0);
        assert_eq!(e.mem.u32(this + 0x98), 0);
        assert_eq!(calls(&e, LIST_TAKE_00B9E850).len(), 3);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert_eq!(e.mem.u32(this + 0x88), 0);
        assert_eq!(e.mem.u32(entry), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_011C5F58), node);
        assert_eq!(e.mem.u32(node), 0x4444);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_011C5F80), 0);
        assert_eq!(
            calls(&e, SPIN_LOCK_ENTER_0040FBF0),
            vec![vec![NODE_POOL_LOCK_011C5F80, 0]]
        );
        assert!(calls(&e, MEMORY_DEALLOCATE).contains(&vec![MEMORY_MANAGER, entry]));
        // The free lists: two takes from +0, one from +0x24.
        let takes: Vec<u32> = calls(&e, LIST_TAKE_00B9E7C0).iter().map(|c| c[0]).collect();
        assert_eq!(takes, vec![this, this, this + 0x24]);
        assert_eq!(e.mem.u32(this + 0xa8), 0);
        assert_eq!(
            calls(&e, 0x0f00_0004)
                .iter()
                .filter(|c| c[0] == shared)
                .count(),
            1
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x3c, 0xc, 5, LIST_DESTRUCT_00B5C7C0],
                vec![this, 0xc, 5, LIST_DESTRUCT_00B5C7C0]
            ]
        );
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL),
            vec![vec![this + 0x9c], vec![this + 0x90]]
        );
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL_00B71300),
            vec![vec![this + 0x84], vec![this + 0x78]]
        );
    }

    #[test]
    fn fn_00b6f0b0_trims_the_list_without_the_flag() {
        let mut e = rig();
        let this = manager(&mut e);
        e.map(0x011a_2000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.set_global(OBJECT_011F4748, 0x6000u32);
        for addr in [LIST_REMOVE_ALL, LIST_REMOVE_ALL_00B71300, VECTOR_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(BORROW_RENDERED_TEXTURE, |_, _| Ret::default());
        e.register(RENDERED_TARGET_00B6B260, |_, _| 0x0f00_2000u32.into_ret());
        e.map(0x0f00_2000, 0x10);
        e.mem.set_u32(0x0f00_2000, 0x0f00_1000);
        e.call(0x00b6_f0b0, &args![this]);
        // An empty +0x90 list is filled to one texture for the renderer at 011f4748.
        let borrows = calls(&e, BORROW_RENDERED_TEXTURE);
        assert_eq!(borrows.len(), 1);
        assert_eq!(borrows[0][1], 0x6000);
    }

    #[test]
    fn fn_00b6f460_and_fn_00b6f470() {
        let mut e = rig();
        e.register(LINK_OBJECT_00BA8A70, |_, _| Ret::default());
        e.call(0x00b6_f460, &args![0x5000u32, 0x6000u32]);
        assert_eq!(calls(&e, LINK_OBJECT_00BA8A70), vec![vec![0x5000, 0x6000]]);
        assert_eq!(e.call(0x00b6_f470, &args![0x5000u32]).u32(), 1);
    }
}

#[cfg(test)]
mod rendered_texture_tests {
    use super::*;

    const RENDERER: u32 = 0x0f10_0000;
    const MADE: u32 = 0x0f20_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for addr in [
            NI_POINTER_FROM_00633C90,
            LIST_ADD_00C1B4A0,
            NI_POINTER_DESTRUCT_0045CEC0,
            LIST_TAKE_00B9E7C0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(TEXTURE_QUERY_00B6C2C0, |e, a| {
            for (index, value) in [0x400u32, 0x300, 0x15, 7, 0x10, 0x99].iter().enumerate() {
                e.mem.set_u32(a[2 + index], *value);
            }
            Ret::default()
        });
        e.register(BORROW_RENDERED_TEXTURE, |_, _| MADE.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x78);
        (e, this)
    }

    fn get(e: &mut Engine, this: u32, kind: u32, zero: u32) -> u32 {
        e.call(
            0x00b6_e110,
            &args![this, RENDERER, kind, 0x100u32, zero, 0x55u32],
        )
        .u32()
    }

    #[test]
    fn fn_00b6e110_takes_a_free_texture_of_a_listed_kind() {
        for (kind, free_list, used_list) in [
            (0x2bu32, 0u32, 0x3cu32),
            (0x11, 0xc, 0x48),
            (9, 0x18, 0x54),
            (0x16, 0x24, 0x60),
            (0x13, 0x30, 0x6c),
        ] {
            let (mut e, this) = engine();
            let head = e.mem.alloc(0xc);
            e.mem.set_u32(head + 8, 0x0f30_0000);
            e.mem.set_u32(this + free_list, head);
            e.mem.set_u32(this + free_list + 8, 1);
            assert_eq!(get(&mut e, this, kind, 0), 0x0f30_0000);
            let made = calls(&e, NI_POINTER_FROM_00633C90);
            assert_eq!(made.len(), 1);
            assert_eq!(made[0][1], 0x0f30_0000);
            assert_eq!(
                calls(&e, LIST_ADD_00C1B4A0),
                vec![vec![this + used_list, made[0][0]]]
            );
            let taken = calls(&e, LIST_TAKE_00B9E7C0);
            assert_eq!(taken.len(), 1);
            assert_eq!(taken[0][0], this + free_list);
            assert_eq!(calls(&e, NI_POINTER_DESTRUCT_0045CEC0).len(), 2);
            assert!(calls(&e, BORROW_RENDERED_TEXTURE).is_empty());
        }
    }

    #[test]
    fn fn_00b6e110_borrows_otherwise() {
        // An empty free list, then a kind without a list.
        for kind in [0x2bu32, 3] {
            let (mut e, this) = engine();
            assert_eq!(get(&mut e, this, kind, 0), MADE);
            let query = calls(&e, TEXTURE_QUERY_00B6C2C0);
            assert_eq!(query.len(), 1);
            assert_eq!(query[0][..2], [RENDERER, kind]);
            assert_eq!(
                calls(&e, BORROW_RENDERED_TEXTURE),
                vec![vec![
                    this, RENDERER, 0x400, 0x300, 0x110, 0x15, 7, kind, 0x99, 0, 0x55
                ]]
            );
        }
    }

    #[test]
    fn fn_00b6e110_answers_0_for_a_nonzero_fourth_word() {
        let (mut e, this) = engine();
        assert_eq!(get(&mut e, this, 3, 1), 0);
        assert!(calls(&e, TEXTURE_QUERY_00B6C2C0).is_empty());
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00b6f480_maps_3_to_0_and_the_rest_to_3() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x88);
        e.call(0x00b6_f480, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        e.call(0x00b6_f480, &args![this, 2u32]);
        assert_eq!(e.mem.u32(this + 0x6c), 3);
    }
}
