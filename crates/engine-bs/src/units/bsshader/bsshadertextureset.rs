//! `BSShader/bsshadertextureset.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderManager::GetTexture` (`00b55840`, `cdecl`, five words: the file
/// name, 1, the result `NiPointer`, 1, whether the slot is 4).
const SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiObject::LoadBinary` (`00a5d610`, `thiscall`, one word).
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiStream::LoadCString` (`00a64ce0`, `thiscall`, one word: the result slot).
const STREAM_LOAD_C_STRING: u32 = 0x00a6_4ce0;
/// `004037f0` (unnamed, `thiscall` on a `BSStringT`, two words: the text, 0):
/// sets the string.
const STRING_SET: u32 = 0x0040_37f0;
/// `00aa10f0` (`cdecl`, one word): frees the loaded C string.
const NI_FREE_STRING: u32 = 0x00aa_10f0;

/// Releases one reference of the `NiRefObject` at `object` (non-null).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

// Translated from 00ba87f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba87f0 - the FS:[0] exception frame that releases the local texture pointer during unwinding (handler 00f2ad18) is not translated
/// `BSShaderTextureSet::GetTexture` (Xbox PDB; `thiscall`, two words: the
/// slot, the result `NiPointer`): the slot's file name (vtable `+0x8C`);
/// when there is one, `BSShaderManager::GetTexture` loads it into a local
/// `NiPointer` (null to begin with; the last word says whether the slot is
/// 4). The local goes into `result` (reference counted) and is then
/// released.
pub fn bs_shader_texture_set_get_texture(e: &mut Engine, this: Ptr, slot: u32, result: Ptr) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        let name = e.vcall(this.addr(), 0x8c, &args![slot]).u32();
        if name != 0 {
            e.call(
                SHADER_MANAGER_GET_TEXTURE,
                &args![name, 1u32, local, 1u32, u32::from(slot == 4)],
            );
        }
        let old = e.mem.u32(result.addr());
        let texture = e.mem.u32(local.addr());
        if old != texture {
            if old != 0 {
                release(e, old);
            }
            let texture = e.mem.u32(local.addr());
            e.mem.set_u32(result.addr(), texture);
            if texture != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![texture + 4]);
            }
        }
        let texture = e.mem.u32(local.addr());
        if texture != 0 {
            release(e, texture);
        }
    });
}

// Translated from 00ba88d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderTextureSet::LoadBinary` (Xbox PDB): the `NiObject` part, the file
/// name count (four bytes from the stream's input file `+0x24c`: its read
/// function `+8`, `cdecl (file, dest, 4, &count, 1)`, the result added to its
/// position `+4`), then for each of them (a signed count) a C string
/// (`NiStream::LoadCString`, into the argument slot, cleared first) set into
/// the `BSStringT` at `+8 + 8 * i` (`004037f0`; behind an `i >= 0` check that
/// always holds) and freed (`00aa10f0`); the slot is cleared after each.
pub fn bs_shader_texture_set_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        // frame + 0: the name count; + 4: the count word of the read; + 8:
        // the argument slot the strings are loaded into.
        let f = frame.addr();
        let file = e.mem.u32(stream.addr() + 0x24c);
        e.mem.set_u32(f, 0);
        e.mem.set_u32(f + 8, 0);
        let read = e.mem.u32(file + 8);
        e.mem.set_u32(f + 4, 4);
        let got = e.call(read, &args![file, f, 4u32, f + 4, 1u32]).u32();
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(got));
        let mut index: i32 = 0;
        if e.mem.i32(f) <= 0 {
            return;
        }
        let mut string = this.addr() + 8;
        loop {
            e.call(STREAM_LOAD_C_STRING, &args![stream, f + 8]);
            if index >= 0 {
                let text = e.mem.u32(f + 8);
                e.call(STRING_SET, &args![string, text, 0u32]);
            }
            let text = e.mem.u32(f + 8);
            e.call(NI_FREE_STRING, &args![text]);
            index += 1;
            string += 8;
            e.mem.set_u32(f + 8, 0);
            if index >= e.mem.i32(f) {
                break;
            }
        }
    });
}

// Translated from 00ba89e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderTextureSet::SetTextureFilename` (Xbox PDB; `thiscall`, two
/// words): sets the `BSStringT` at `+8 + 8 * slot` to `name` (`004037f0`).
pub fn bs_shader_texture_set_set_texture_filename(e: &mut Engine, this: Ptr, slot: u32, name: u32) {
    let string = this
        .addr()
        .wrapping_add(slot.wrapping_mul(8))
        .wrapping_add(8);
    e.call(STRING_SET, &args![string, name, 0u32]);
}

/// `NiObject::SaveBinary` (`00a89af0`, `thiscall`, one word: the stream) and the stream's
/// string save (`00a64160`, `thiscall` on the stream, the text: its length, then its
/// characters).
const OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
const STREAM_SAVE_STRING: u32 = 0x00a6_4160;

// Translated from 00ba8970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderTextureSet::SaveBinary` (engine map; the body matches): `thiscall`, one word (the
/// stream). `NiObject::SaveBinary`, the name count 6 (a local word written through the
/// stream's file, `+0x250`: its write function `+0xc`, `cdecl (file, &count, 4, &size, 1)`,
/// the result added to its position `+4`; the size word is the argument slot), then the six
/// names (the text of each `BSStringT` at `+8 + 8 * i`, through `00a64160`).
pub fn bs_shader_texture_set_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(OBJECT_SAVE_BINARY, &args![this, stream]);
    let file = e.mem.u32(stream.wrapping_add(0x250));
    e.with_stack(8, |e, frame| {
        let count = frame.addr();
        let size = count + 4;
        e.mem.set_u32(count, 6);
        let write = e.mem.u32(file.wrapping_add(0xc));
        e.mem.set_u32(size, 4);
        let written = e.call(write, &args![file, count, 4u32, size, 1u32]).u32();
        let position = e.mem.u32(file.wrapping_add(4));
        e.mem
            .set_u32(file.wrapping_add(4), position.wrapping_add(written));
    });
    let mut string = this.addr() + 8;
    for _ in 0..6 {
        let text = e.mem.u32(string);
        e.call(STREAM_SAVE_STRING, &args![stream, text]);
        string += 8;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_87f0,
            bs_shader_texture_set_get_texture(Ptr, u32, Ptr)
        ),
        entry!(0x00ba_88d0, bs_shader_texture_set_load_binary(Ptr, Ptr)),
        entry!(
            0x00ba_89e0,
            bs_shader_texture_set_set_texture_filename(Ptr, u32, u32)
        ),
        entry!(0x00ba_8970, bs_shader_texture_set_save_binary(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn get_texture_loads_the_named_texture_into_the_result() {
        const FILE_NAME: u32 = 0x0100_008c;
        const DELETE_THIS: u32 = 0x0100_0004;
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        let texture_vtable = e.mem.alloc(8);
        e.mem.set_u32(texture_vtable + 4, DELETE_THIS);
        let texture = e.mem.alloc(8);
        e.mem.set_u32(texture, texture_vtable);
        e.register_double(SHADER_MANAGER_GET_TEXTURE, move |e, a| {
            e.mem.set_u32(a[2], texture);
            e.mem.set_u32(texture + 4, 1);
            Ret::default()
        });
        e.register(FILE_NAME, |_, a| word(if a[1] == 9 { 0 } else { 0x5555 }));
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x8c, FILE_NAME);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        let result = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00ba_87f0, &args![this, 4u32, result]);
        let load = &calls_to(&e, SHADER_MANAGER_GET_TEXTURE)[0];
        assert_eq!((load[0], load[1], load[3], load[4]), (0x5555, 1, 1, 1));
        assert_eq!(e.mem.u32(result), texture);
        assert_eq!(e.mem.u32(texture + 4), 1);
        // No file name: the result is cleared, releasing the texture.
        e.call(0x00ba_87f0, &args![this, 9u32, result]);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![texture]]);
    }

    #[test]
    fn load_binary_sets_each_file_name() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            e.mem.set_u32(a[1], 2);
            word(4)
        });
        e.register(STREAM_LOAD_C_STRING, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 0x7700);
            Ret::default()
        });
        e.register(STRING_SET, |_, _| Ret::default());
        e.register(NI_FREE_STRING, |_, _| Ret::default());
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00ba_88d0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![this + 8, 0x7700, 0], vec![this + 0x10, 0x7700, 0]]
        );
        assert_eq!(calls_to(&e, NI_FREE_STRING).len(), 2);
        assert_eq!(e.mem.u32(file + 4), 4);
    }

    #[test]
    fn set_texture_filename_sets_the_slots_string() {
        let mut e = Engine::new();
        e.register(STRING_SET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ba_89e0, &args![0x1000u32, 3u32, 0x2222u32]);
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![0x1020, 0x2222, 0]]);
    }
}

#[cfg(test)]
mod n012_tests {
    use super::*;

    #[test]
    fn save_binary_writes_the_count_and_six_names() {
        const WRITE: u32 = 0x0100_0030;
        let mut e = Engine::new();
        e.register(OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_STRING, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((e.mem.u32(a[1]), a[2], e.mem.u32(a[3]), a[4]), (6, 4, 4, 1));
            4u32.into_ret()
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, file);
        let this = e.mem.alloc(0x38);
        for k in 0..6u32 {
            e.mem.set_u32(this + 8 + 8 * k, 0x100 + k);
        }
        e.call_log = Some(vec![]);
        e.call(0x00ba_8970, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OBJECT_SAVE_BINARY, vec![this, stream])));
        let names: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == STREAM_SAVE_STRING)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(names, (0x100..0x106).collect::<Vec<u32>>());
        assert_eq!(e.mem.u32(file + 4), 4);
    }
}
