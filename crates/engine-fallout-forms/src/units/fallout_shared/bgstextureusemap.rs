//! `fallout shared/bgstextureusemap.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSFile::BSFile` (Xbox PDB, `00b00260`; `RET 0x10`: name, mode, buffer
/// size, text flag).
const BS_FILE_CTOR: u32 = 0x00b0_0260;
/// `BSFile::Open` (Xbox PDB, `00aff300`; `RET 8`), `bool` in `AL`.
const BS_FILE_OPEN: u32 = 0x00af_f300;
/// `BSFile::Close` (Xbox PDB, `00affd10`).
const BS_FILE_CLOSE: u32 = 0x00af_fd10;
/// `BSFile::~BSFile` (Xbox PDB, `00aff240`).
const BS_FILE_DTOR: u32 = 0x00af_f240;
/// Writes a string to the file (`00affbb0`; `RET 8`: string, flag).
const BS_FILE_WRITE_STRING: u32 = 0x00af_fbb0;
/// The PC size of the `BSFile` the function keeps on its stack (`EBP-0x16C`
/// up to the cookie at `EBP-0x14`).
const BS_FILE_STACK_SIZE: u32 = 0x158;
/// The map's first position (`004b9ba0`, on `UserMap`), 0 when empty.
const MAP_FIRST: u32 = 0x004b_9ba0;
/// The map's next entry (`006b7f20`, on `UserMap`; `RET 0xC`: address of the
/// position, address of the key, address of the value).
const MAP_NEXT: u32 = 0x006b_7f20;
/// Casts the key through the `NiRTTI` at `011f444c` (`00653270`, cdecl:
/// RTTI, object; 0 for a null object).
const CAST_KEY: u32 = 0x0065_3270;
const KEY_RTTI: u32 = 0x011f_444c;
/// `BSStringT` constructor and destructor (`004037b0`, `004037d0`), the
/// 8-byte temporary.
const STRING_CTOR: u32 = 0x0040_37b0;
const STRING_DTOR: u32 = 0x0040_37d0;
/// The string's formatter (`00406f60`, cdecl: string, format, ...).
const STRING_FORMAT: u32 = 0x0040_6f60;
/// The user count of the record's list (`005ae380`, on `UserList`; the
/// engine map names the folded body `VATS::GetCount`).
const LIST_COUNT: u32 = 0x005a_e380;
/// `0043b1b0` (`this` only): the text of the object the texture's slot 0x9C
/// returns.
const OBJECT_TEXT: u32 = 0x0043_b1b0;
/// `008256d0` (`this` = list node): whether the node holds no item.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`this` = list node): the address of its item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`this` = list node): the word at +4, the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `00440e30` (`this` = form): a text of the form's.
const FORM_TEXT: u32 = 0x0044_0e30;
/// `TESForm`'s form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `"%s\t%d bytes\t%i users\r\n"`.
const TEXTURE_LINE: u32 = 0x0101_f534;
/// `"%s\t%s\t%08X\r\n"`.
const USER_LINE: u32 = 0x0101_f524;
/// `"\r\n"`.
const END_LINE: u32 = 0x0101_f520;

layout! {
    /// `BGSTextureUseMap` (Xbox PDB), 0x14 bytes.
    pub struct BGSTextureUseMap: 0x14 {
        /// `pDefaultUser` (Xbox PDB): `TESForm*`.
        0x00 pDefaultUser: u32,
        /// `UserMap` (Xbox PDB): `NiTMap<NiTexture*, TEX_USER_DATA*>`, used
        /// through its address.
        0x04 UserMap: u32,
    }
}

// Translated from 004aa900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004aa900 - the exception frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
/// `BGSTextureUseMap::WriteToFile` (Xbox PDB; `RET 4`): opens a `BSFile`
/// named `name` on the stack (mode 1, a 0x400 buffer, text) and, when it
/// opens, writes for each map entry whose key casts (`00653270`) and whose
/// `TEX_USER_DATA` exists a line `"%s\t%d bytes\t%i users"` (the text of the
/// object the texture's slot 0x9C returns, `ulTextureSize` at +4, the user
/// count), a line `"%s\t%s\t%08X"` per user in the `UserList` at +8 (the
/// form's `00440e30` text, its slot 0x130 text, its form id) and an empty
/// line; then closes it. The file is destroyed either way; returns whether it
/// opened.
pub fn bgs_texture_use_map_write_to_file(
    e: &mut Engine,
    this: Ptr<BGSTextureUseMap>,
    name: Ptr,
) -> bool {
    let map = this.addr() + BGSTextureUseMap::UserMap.off;
    e.with_stack(BS_FILE_STACK_SIZE, |e, file| {
        let mut opened = false;
        e.call(BS_FILE_CTOR, &args![file, name, 1u32, 0x400u32, 1u32]);
        if e.call(BS_FILE_OPEN, &args![file, 0u32, 0u32]).bool() {
            opened = true;
            // The position, key and value locals the iteration fills.
            e.with_stack(0xc, |e, locals| {
                let position = locals;
                let key = locals.byte_add(4);
                let value = locals.byte_add(8);
                let first = e.call(MAP_FIRST, &args![map]).u32();
                e.mem.set_u32(position.addr(), first);
                while e.mem.u32(position.addr()) != 0 {
                    e.mem.set_u32(key.addr(), 0);
                    e.mem.set_u32(value.addr(), 0);
                    e.call(MAP_NEXT, &args![map, position, key, value]);
                    let key_word = e.mem.u32(key.addr());
                    let texture = e.call(CAST_KEY, &args![KEY_RTTI, key_word]).u32();
                    let data = e.mem.u32(value.addr());
                    if texture == 0 || data == 0 {
                        continue;
                    }
                    write_texture(e, file, texture, data);
                }
            });
            e.call(BS_FILE_CLOSE, &args![file]);
        }
        e.call(BS_FILE_DTOR, &args![file]);
        opened
    })
}

/// One texture's lines of `bgs_texture_use_map_write_to_file`.
fn write_texture(e: &mut Engine, file: Ptr, texture: u32, data: u32) {
    e.with_stack(8, |e, text| {
        e.call(STRING_CTOR, &args![text]);
        // TEX_USER_DATA (Xbox PDB): ulTextureSize +4, UserList +8.
        let users = e.call(LIST_COUNT, &args![data + 8]).u32();
        let size = e.mem.u32(data + 4);
        let object = e.vcall(texture, 0x9c, &[]).u32();
        let texture_text = e.call(OBJECT_TEXT, &args![object]).u32();
        e.call(
            STRING_FORMAT,
            &args![text, TEXTURE_LINE, texture_text, size, users],
        );
        e.call(BS_FILE_WRITE_STRING, &args![file, text, 1u32]);
        let mut node = data + 8;
        while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            let form = e.mem.u32(item);
            node = e.call(NODE_NEXT, &args![node]).u32();
            let form_id = e.call(FORM_ID, &args![form]).u32();
            let slot_text = e.vcall(form, 0x130, &[]).u32();
            let form_text = e.call(FORM_TEXT, &args![form]).u32();
            e.call(
                STRING_FORMAT,
                &args![text, USER_LINE, form_text, slot_text, form_id],
            );
            e.call(BS_FILE_WRITE_STRING, &args![file, text, 1u32]);
        }
        e.call(STRING_FORMAT, &args![text, END_LINE]);
        e.call(BS_FILE_WRITE_STRING, &args![file, text, 1u32]);
        e.call(STRING_DTOR, &args![text]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x004a_a900,
        bgs_texture_use_map_write_to_file(Ptr<BGSTextureUseMap>, Ptr) -> bool
    )]
}

#[cfg(test)]
mod n037_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// File, string and list doubles. Objects' slot 0x9C and 0x130 answer their address plus 0x9C
    /// and 0x130.
    fn engine(opens: bool) -> Engine {
        let mut e = Engine::new();
        for addr in [
            BS_FILE_CTOR,
            BS_FILE_CLOSE,
            BS_FILE_DTOR,
            BS_FILE_WRITE_STRING,
            STRING_CTOR,
            STRING_DTOR,
            STRING_FORMAT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        if opens {
            e.register(BS_FILE_OPEN, |_, _| ret(0x101));
        } else {
            e.register(BS_FILE_OPEN, |_, _| ret(0x100));
        }
        e.register(MAP_FIRST, |_, _| ret(1));
        e.register(CAST_KEY, |_, a| ret(a[1]));
        e.register(LIST_COUNT, |_, _| ret(2));
        e.register(OBJECT_TEXT, |_, a| ret(a[0] + 1));
        e.register(NODE_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_ID, |_, a| ret(a[0] + 0xc));
        e.register(FORM_TEXT, |_, a| ret(a[0] + 0xe30));
        e.register(0x7000_009c, |_, a| ret(a[0] + 0x9c));
        e.register(0x7000_0130, |_, a| ret(a[0] + 0x130));
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x9c / 4] = 0x7000_009c;
        slots[0x130 / 4] = 0x7000_0130;
        e.put_vtable(0x0600_0000, &slots);
        e.call_log = Some(vec![]);
        e
    }

    fn map_yields(e: &mut Engine, entries: Vec<(u32, u32)>) {
        let mut left = entries.into_iter();
        e.register_double(MAP_NEXT, move |e, a| {
            let (key, value) = left.next().unwrap();
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            let position = e.mem.u32(a[1]);
            e.mem
                .set_u32(a[1], if left.len() == 0 { 0 } else { position + 1 });
            Ret::default()
        });
    }

    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0600_0000);
        object
    }

    #[test]
    fn write_to_file_lists_each_texture_and_its_users() {
        let mut e = engine(true);
        let texture = object(&mut e);
        let user_a = object(&mut e);
        let user_b = object(&mut e);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 4, 0x400);
        let second = e.mem.alloc(8);
        e.mem.set_u32(data + 8, user_a);
        e.mem.set_u32(data + 0xc, second);
        e.mem.set_u32(second, user_b);
        // Entries without a texture or a value are skipped.
        map_yields(&mut e, vec![(0, data), (texture, 0), (texture, data)]);
        let this = e.mem.alloc(0x14);
        let name = 0x0101_0000u32;
        assert!(e.call(0x004a_a900, &[this, name]).bool());
        let file = calls_to(&e, BS_FILE_CTOR)[0][0];
        assert_eq!(
            calls_to(&e, BS_FILE_CTOR),
            vec![vec![file, name, 1, 0x400, 1]]
        );
        assert_eq!(calls_to(&e, MAP_FIRST), vec![vec![this + 4]]);
        let formats = calls_to(&e, STRING_FORMAT);
        let text = formats[0][0];
        assert_eq!(
            formats,
            vec![
                vec![text, TEXTURE_LINE, texture + 0x9c + 1, 0x400, 2],
                vec![
                    text,
                    USER_LINE,
                    user_a + 0xe30,
                    user_a + 0x130,
                    user_a + 0xc
                ],
                vec![
                    text,
                    USER_LINE,
                    user_b + 0xe30,
                    user_b + 0x130,
                    user_b + 0xc
                ],
                vec![text, END_LINE],
            ]
        );
        assert_eq!(calls_to(&e, BS_FILE_WRITE_STRING).len(), 4);
        assert_eq!(calls_to(&e, LIST_COUNT), vec![vec![data + 8]]);
        assert_eq!(calls_to(&e, BS_FILE_CLOSE), vec![vec![file]]);
        assert_eq!(calls_to(&e, BS_FILE_DTOR), vec![vec![file]]);
    }

    #[test]
    fn write_to_file_only_destroys_a_file_that_did_not_open() {
        let mut e = engine(false);
        let this = e.mem.alloc(0x14);
        assert!(!e.call(0x004a_a900, &[this, 0x0101_0000]).bool());
        assert!(calls_to(&e, MAP_FIRST).is_empty());
        assert!(calls_to(&e, BS_FILE_CLOSE).is_empty());
        assert_eq!(calls_to(&e, BS_FILE_DTOR).len(), 1);
    }
}
