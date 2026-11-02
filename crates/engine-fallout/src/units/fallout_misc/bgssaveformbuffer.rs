//! `fallout/misc/saveload/bgssaveformbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSSaveFormBuffer` (Xbox PDB), PC layout verified in 008659c0.
    pub struct BGSSaveFormBuffer: 0x24 {
        /// `Header.iChangeFlags` (Xbox PDB `BGSSaveLoadFormHeader`, header at +0x14).
        0x17 iChangeFlags: u32,
        /// `Header.FormInfo` (Xbox PDB): the form type in the low six bits,
        /// the size type in the top two.
        0x1b FormInfo: u8,
        /// `Header.cVersion` (Xbox PDB).
        0x1c cVersion: u8,
        0x20 pForm: Ptr,
    }
}

/// `GetSizeTypeForSize` / `GetSizeTypeSize` (`bgssaveloaddefines.cpp`).
const GET_SIZE_TYPE_FOR_SIZE: u32 = 0x0084_5d40;
const GET_SIZE_TYPE_SIZE: u32 = 0x0084_5d70;
/// `SaveGameSizeToFile` (cdecl): writes `size` in the given size type.
const SAVE_GAME_SIZE_TO_FILE: u32 = 0x0084_5d80;
/// `BGSSaveLoadFile::Write` (`__thiscall` on the file: `(buffer, length)`).
const SAVE_LOAD_FILE_WRITE: u32 = 0x0084_6330;
/// `BGSSaveGameBuffer::Save` overload that writes the buffer's bytes to a
/// file (`__thiscall` on the form buffer: `(file, size)`).
const SAVE_BUFFER_TO_FILE: u32 = 0x0086_5c80;
/// `0084e3a0`: the word at `+0xc` of the buffer (the written size).
const BUFFER_WRITTEN_SIZE: u32 = 0x0084_e3a0;
/// `BGSNumericIDIndex::SetNumericID` (on the header at `+0x14`) and
/// `BGSSaveLoadFormInfo::SetFormType` (on the form info at `+0x1b`).
const SET_NUMERIC_ID: u32 = 0x0085_3570;
const SET_FORM_TYPE: u32 = 0x0085_3670;
/// Size of the form header written ahead of the data.
const HEADER_SIZE: u32 = 9;

// Translated from 008659c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveFormBuffer::BGSSaveFormBuffer` (Xbox PDB): constructs the
/// base buffer and embedded header, installs the derived vtable and clears pForm.
// Translation incomplete: 008659c0 - compiler exception-unwind cleanup remains omitted.
pub fn construct(e: &mut Engine, this: Ptr<BGSSaveFormBuffer>) -> Ptr<BGSSaveFormBuffer> {
    e.call(0x00865bd0, &args![this]);
    e.mem.set_u32(this.addr(), 0x01082308);
    e.call(0x00864450, &args![this.addr().wrapping_add(0x14)]);
    e.set(this, BGSSaveFormBuffer::pForm, Ptr::NULL);
    this
}

// Translated from 007af430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveFormBuffer::GetForm` (Xbox PDB): returns the word at +0x20.
pub fn get_form(e: &mut Engine, this: Ptr<BGSSaveFormBuffer>) -> Ptr {
    e.get(this, BGSSaveFormBuffer::pForm)
}

// Translated from 00865b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveFormBuffer::GetReference` (Xbox PDB): virtual GetForm, then
/// the form's reference-type predicate at vtable +0xf0 when non-null.
pub fn get_reference(e: &mut Engine, this: Ptr<BGSSaveFormBuffer>) -> Ptr {
    let form = e.vcall(this.addr(), 0, &[]).u32();
    if form != 0 && e.vcall(form, 0xf0, &[]).bool() {
        Ptr::new(form)
    } else {
        Ptr::NULL
    }
}

// Translated from 00865b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveFormBuffer::GetActor` (Xbox PDB): virtual GetReference, then
/// the reference's actor-type predicate at vtable +0x100 when non-null.
pub fn get_actor(e: &mut Engine, this: Ptr<BGSSaveFormBuffer>) -> Ptr {
    let reference = e.vcall(this.addr(), 4, &[]).u32();
    if reference != 0 && e.vcall(reference, 0x100, &[]).bool() {
        Ptr::new(reference)
    } else {
        Ptr::NULL
    }
}

// Translated from 00865a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveFormBuffer::Save` (Xbox PDB, `__thiscall`, one stack word):
/// takes the written size (`0084e3a0`), its size type
/// (`GetSizeTypeForSize`), stores the size type in the header's form info,
/// writes the 9-byte header to `file`, then the size in that size type and
/// the buffer's bytes.
pub fn bgs_save_form_buffer_save(e: &mut Engine, this: Ptr<BGSSaveFormBuffer>, file: Ptr) {
    let size = e.call(BUFFER_WRITTEN_SIZE, &args![this]).u32();
    let size_type = e.call(GET_SIZE_TYPE_FOR_SIZE, &args![size]).u32();
    fn_00865aa0(e, this.byte_add(0x1b), size_type);
    e.call(
        SAVE_LOAD_FILE_WRITE,
        &args![file, this.addr().wrapping_add(0x14), HEADER_SIZE],
    );
    e.call(SAVE_GAME_SIZE_TO_FILE, &args![file, size, size_type]);
    e.call(SAVE_BUFFER_TO_FILE, &args![this, file, size]);
}

// Translated from 00865aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a size type in the top two bits of the byte at `form_info` (the
/// header's `FormInfo`, `+0x1b` of the buffer) and keeps the low six. The map
/// has no name for it.
pub fn fn_00865aa0(e: &mut Engine, form_info: Ptr, size_type: u32) {
    let current = e.mem.u8(form_info.addr()) as u32;
    e.mem.set_u8(
        form_info.addr(),
        ((current & 0x3f) | (size_type << 6)) as u8,
    );
}

// Translated from 00865ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the buffer's 9-byte header (`+0x14`; the map has no name for this
/// function): the form id index (`BGSNumericIDIndex::SetNumericID` on the
/// header), the change flags word at `+0x17`, the form type
/// (`BGSSaveLoadFormInfo::SetFormType` on the form info at `+0x1b`) and the
/// version byte at `+0x1c`.
pub fn fn_00865ad0(
    e: &mut Engine,
    this: Ptr<BGSSaveFormBuffer>,
    form_id_index: u32,
    change_flags: u32,
    form_type: u32,
    version: u8,
) {
    e.call(
        SET_NUMERIC_ID,
        &args![this.addr().wrapping_add(0x14), form_id_index],
    );
    e.set(this, BGSSaveFormBuffer::iChangeFlags, change_flags);
    e.call(
        SET_FORM_TYPE,
        &args![this.addr().wrapping_add(0x1b), form_type],
    );
    e.set(this, BGSSaveFormBuffer::cVersion, version);
}

// Translated from 00865b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveFormBuffer::GetOverhead` (Xbox PDB): the size of the header (9)
/// plus the bytes the size takes in its size type.
pub fn bgs_save_form_buffer_get_overhead(e: &mut Engine, this: Ptr<BGSSaveFormBuffer>) -> i32 {
    let size = e.call(BUFFER_WRITTEN_SIZE, &args![this]).u32();
    let size_type = e.call(GET_SIZE_TYPE_FOR_SIZE, &args![size]).u32();
    e.call(GET_SIZE_TYPE_SIZE, &args![size_type]).i32() + HEADER_SIZE as i32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00865a30,
            bgs_save_form_buffer_save(Ptr<BGSSaveFormBuffer>, Ptr)
        ),
        entry!(0x00865aa0, fn_00865aa0(Ptr, u32)),
        entry!(
            0x00865ad0,
            fn_00865ad0(Ptr<BGSSaveFormBuffer>, u32, u32, u32, u8)
        ),
        entry!(
            0x00865b10,
            bgs_save_form_buffer_get_overhead(Ptr<BGSSaveFormBuffer>) -> i32
        ),
        entry!(
            0x008659c0,
            construct(Ptr<BGSSaveFormBuffer>) -> Ptr<BGSSaveFormBuffer>
        ),
        entry!(0x007af430, get_form(Ptr<BGSSaveFormBuffer>) -> Ptr),
        entry!(0x00865b40, get_reference(Ptr<BGSSaveFormBuffer>) -> Ptr),
        entry!(0x00865b80, get_actor(Ptr<BGSSaveFormBuffer>) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_and_health_write_through_real_registry_with_native_change_flags() {
        use crate::exe::Import;
        use crate::extra_data::ExtraList;

        let mut e = Engine::new();
        e.map(0x011de134, 4);
        e.map(0x011c3000, 0x1000);
        e.map(0x01012054, 4);
        e.mem.set_f32(0x01012054, -1.0); // Native absent-health constant.
        let imports = [
            (0x00fdf1d0, "GetCurrentThreadId"),
            (0x00fdf1bc, "InterlockedCompareExchange"),
        ]
        .map(|(slot, name)| Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        });
        assert_eq!(e.register_imports(&imports), 2);
        e.map(0x01183d30, 0x300);
        // PC table at 01183dc0/01183dc4: count/health use mask 0x400.
        e.mem.set_u32(0x01183dc0, 0x400);
        e.mem.set_u32(0x01183dc4, 0x400);
        let mode = e.mem.alloc(16);
        e.mem.set_u8(mode + 12, 0);
        e.set_global(0x011de134, mode);
        e.put_vtable(0x01082308, &[0x007af430, 0x00865b40, 0x00865b80]);
        let buffer = e.new_object::<BGSSaveFormBuffer>();
        e.call(0x008659c0, &[buffer.addr()]);
        e.call(0x00428130, &[buffer.addr(), 0x400]);
        let list = ExtraList::new(&mut e);
        list.set_item_count(&mut e, 5);
        list.set_health(&mut e, 42.5);
        e.call(0x00426a30, &[list.as_ptr().addr(), buffer.addr()]);
        let bytes = e.mem.u32(buffer.addr() + 4);
        let length = e.mem.u32(buffer.addr() + 12);
        assert_eq!(length, 14);
        assert_eq!(
            e.mem.bytes(bytes, length),
            [8, b'|', 0x24, b'|', 5, 0, b'|', 0x25, b'|', 0, 0, 0x2a, 0x42, b'|',]
        );
        assert_eq!(e.mem.u32(buffer.addr() + 8), 0x10000);
        assert_eq!(e.mem.u32(buffer.addr() + 16), 5);
        assert_eq!(list.item_count(&mut e), 5);
        assert_eq!(list.health(&mut e), 42.5);
        assert_eq!(list.len(&mut e), 2);

        // Different change flags filter both nodes but still emit an empty
        // variable-sized count. No writer, list, allocator or getter doubles.
        e.mem.set_u32(buffer.addr() + 12, 0);
        e.mem.set_u32(buffer.addr() + 16, 0);
        e.call(0x00428130, &[buffer.addr(), 2]);
        e.call(0x00426a30, &[list.as_ptr().addr(), buffer.addr()]);
        assert_eq!(e.mem.u32(buffer.addr() + 12), 2);
        assert_eq!(e.mem.bytes(bytes, 2), [0, b'|']);
        assert_eq!(e.mem.u32(buffer.addr() + 16), 1);
    }

    #[test]
    fn buffer_and_header_constructors_match_native_guarded_vectors() {
        // Generated nv-call cases, FalloutNV.exe 1.4.0.525 SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80. Only addresses/results retained.
        for (address, bytes, zeroed, vtable) in [
            (0x00865bd0, 24, 20, Some(0x01082324u32)),
            (0x008659c0, 40, 36, Some(0x01082308u32)),
            (0x00864450, 16, 9, None),
            (0x0044dee0, 8, 4, None),
            (0x0084eaf0, 8, 3, None),
            (0x00864490, 4, 1, None),
        ] {
            let mut e = Engine::new();
            let object = e.mem.alloc(bytes);
            e.mem.write(object, &vec![0xaa; bytes as usize]);
            assert_eq!(e.call(address, &[object]).u32(), object);
            let mut expected = vec![0xaa; bytes as usize];
            expected[..zeroed].fill(0);
            if let Some(vtable) = vtable {
                expected[..4].copy_from_slice(&vtable.to_le_bytes());
            }
            if address == 0x008659c0 {
                expected[0x1d..0x20].fill(0xaa); // Alignment padding before pForm.
            }
            assert_eq!(e.mem.bytes(object, bytes), expected, "{address:08x}");
        }
    }

    #[test]
    fn form_reference_and_actor_getters_obey_virtual_predicates() {
        let mut e = Engine::new();
        let buffer = e.new_object::<BGSSaveFormBuffer>();
        e.call(0x008659c0, &[buffer.addr()]);
        e.put_vtable(0x01082308, &[0x007af430, 0x00865b40, 0x00865b80]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007af430, &[buffer.addr()]).u32(), 0);
        assert_eq!(e.call(0x00865b40, &[buffer.addr()]).u32(), 0);
        assert_eq!(e.call(0x00865b80, &[buffer.addr()]).u32(), 0);

        let form = e.mem.alloc(4);
        let table = e.mem.alloc(0x104);
        e.mem.set_u32(form, table);
        e.mem.set_u32(table + 0xf0, 0x03000000);
        e.mem.set_u32(table + 0x100, 0x03000004);
        e.set(buffer, BGSSaveFormBuffer::pForm, Ptr::new(form));
        for (is_reference, is_actor) in [(false, false), (true, false), (true, true)] {
            e.register_double(0x03000000, move |_, args| {
                assert_eq!(args, &[form]);
                is_reference.into_ret()
            });
            e.register_double(0x03000004, move |_, args| {
                assert_eq!(args, &[form]);
                is_actor.into_ret()
            });
            assert_eq!(e.call(0x007af430, &[buffer.addr()]).u32(), form);
            assert_eq!(
                e.call(0x00865b40, &[buffer.addr()]).u32(),
                if is_reference { form } else { 0 }
            );
            assert_eq!(
                e.call(0x00865b80, &[buffer.addr()]).u32(),
                if is_reference && is_actor { form } else { 0 }
            );
        }
    }

    #[test]
    fn save_writes_the_header_the_size_and_the_bytes_in_order() {
        let mut e = Engine::new();
        let buffer = e.new_object::<BGSSaveFormBuffer>();
        e.mem.set_u32(buffer.addr() + 0x0c, 0x1234);
        e.mem.set_u8(buffer.addr() + 0x1b, 0x15);
        e.register(0x0084_e3a0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(0x0084_5d40, |_, a| {
            assert_eq!(a[0], 0x1234);
            1u32.into_ret()
        });
        e.register(0x0084_6330, |e, a| {
            // The size type is already in the form info when the header is written.
            assert_eq!(e.mem.u8(a[1] + 7), 0x15 | (1 << 6));
            Ret::default()
        });
        e.register(0x0084_5d80, |_, _| Ret::default());
        e.register(0x0086_5c80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_5a30, &[buffer.addr(), 0x7000]);
        let b = buffer.addr();
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0086_5a30, vec![b, 0x7000]),
                (0x0084_e3a0, vec![b]),
                (0x0084_5d40, vec![0x1234]),
                (0x0084_6330, vec![0x7000, b + 0x14, 9]),
                (0x0084_5d80, vec![0x7000, 0x1234, 1]),
                (0x0086_5c80, vec![b, 0x7000, 0x1234]),
            ]
        );
    }

    #[test]
    fn size_type_replaces_only_the_top_two_bits() {
        let mut e = Engine::new();
        let byte = e.mem.alloc(1);
        e.mem.set_u8(byte, 0xff);
        e.call(0x0086_5aa0, &[byte, 2]);
        assert_eq!(e.mem.u8(byte), 0xbf);
        e.mem.set_u8(byte, 0x2a);
        e.call(0x0086_5aa0, &[byte, 0]);
        assert_eq!(e.mem.u8(byte), 0x2a);
        e.call(0x0086_5aa0, &[byte, 3]);
        assert_eq!(e.mem.u8(byte), 0xea);
    }

    #[test]
    fn header_setter_fills_the_nine_byte_header() {
        let mut e = Engine::new();
        let buffer = e.new_object::<BGSSaveFormBuffer>();
        e.register(0x0085_3570, |_, _| Ret::default());
        e.register(0x0085_3670, |_, _| Ret::default());
        e.mem.set_u8(buffer.addr() + 0x1b, 0x99);
        e.call_log = Some(vec![]);
        e.call(
            0x0086_5ad0,
            &[buffer.addr(), 0xabc, 0x1122_3344, 0x2b, 0xfe01],
        );
        let b = buffer.addr();
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0086_5ad0, vec![b, 0xabc, 0x1122_3344, 0x2b, 0xfe01]),
                (0x0085_3570, vec![b + 0x14, 0xabc]),
                (0x0085_3670, vec![b + 0x1b, 0x2b]),
            ]
        );
        assert_eq!(e.mem.u32(b + 0x17), 0x1122_3344);
        assert_eq!(e.mem.u8(b + 0x1b), 0x99); // Only the double would change it.
        assert_eq!(e.mem.u8(b + 0x1c), 0x01); // The version is one byte.
    }

    #[test]
    fn overhead_is_nine_plus_the_size_type_width() {
        let mut e = Engine::new();
        let buffer = e.new_object::<BGSSaveFormBuffer>();
        e.mem.set_u32(buffer.addr() + 0x0c, 70000);
        e.register(0x0084_e3a0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(0x0084_5d40, |_, a| {
            assert_eq!(a[0], 70000);
            2u32.into_ret()
        });
        e.register(0x0084_5d70, |_, a| (a[0] * 2).into_ret());
        assert_eq!(e.call(0x0086_5b10, &[buffer.addr()]).i32(), 13);
    }
}
