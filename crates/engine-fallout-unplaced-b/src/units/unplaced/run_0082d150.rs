//! `(unplaced)/run_0082d150` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `004615d0` `TESDataHandler::GetSound` (Xbox PDB name in the engine map;
/// thiscall, one word).
const GET_SOUND: u32 = 0x0046_15d0;
/// `00511840` (thiscall): the text of the string at the sound's +0x30
/// (`00408da0` on `this + 0x30`), copied out as its file name.
const SOUND_FILE_NAME: u32 = 0x0051_1840;
/// `00406d30` (cdecl: destination, size, source): `strcpy_s`.
const STRCPY_S: u32 = 0x0040_6d30;
/// `004e75d0` (thiscall, out pointer): copies the 0x24 bytes at `this +
/// 0x44` (the sound's data) to the out block and returns it.
const COPY_SOUND_DATA: u32 = 0x004e_75d0;
/// `005e39b0` (cdecl, flags): maps sound flags 0x10 / 0x200 / 0x400 to
/// 0x10 / 0x2000000 / 0x4000000.
const SOUND_FLAG_BITS: u32 = 0x005e_39b0;

/// The flag word (+4) of a copy of the sound's data made by `004e75d0`.
fn sound_data_flags(e: &mut Engine, sound: u32) -> u32 {
    e.with_stack(0x24, |e, data| {
        let copy = e.call(COPY_SOUND_DATA, &args![sound, data]).u32();
        e.mem.u32(copy.wrapping_add(4))
    })
}

// Translated from 0082d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: looks the sound `sound_id` up in the data
/// handler (`TESDataHandler::GetSound`); when it exists, has a file name and
/// `file_name` is given, copies the name (at most 0x104 bytes), adjusts the
/// play flags at `flags` from the sound's data flags (0x40 sets bit 0 and
/// clears bit 1; otherwise bit 1 is set and bit 0 cleared when neither is
/// set; 0x1 adds 0x40; 0x800 adds 0x8000000; then `005e39b0`'s bits), stores
/// the sound at `sound_out` and returns true. Otherwise returns false.
pub fn fn_0082d150(
    e: &mut Engine,
    sound_id: u32,
    file_name: Ptr,
    flags: Ptr,
    sound_out: Ptr,
) -> bool {
    let handler = e.global::<u32>(DATA_HANDLER);
    let sound = e.call(GET_SOUND, &args![handler, sound_id]).u32();
    if sound == 0 || e.call(SOUND_FILE_NAME, &args![sound]).u32() == 0 || file_name.addr() == 0 {
        return false;
    }
    let name = e.call(SOUND_FILE_NAME, &args![sound]).u32();
    e.call(STRCPY_S, &args![file_name, 0x104u32, name]);
    let flags = flags.addr();
    if sound_data_flags(e, sound) & 0x40 != 0 {
        let value = e.mem.u32(flags) | 1;
        e.mem.set_u32(flags, value);
        let value = e.mem.u32(flags) & 0xffff_fffd;
        e.mem.set_u32(flags, value);
    } else if e.mem.u32(flags) & 1 == 0 && e.mem.u32(flags) & 2 == 0 {
        let value = e.mem.u32(flags) | 2;
        e.mem.set_u32(flags, value);
        let value = e.mem.u32(flags) & 0xffff_fffe;
        e.mem.set_u32(flags, value);
    }
    if sound_data_flags(e, sound) & 1 != 0 {
        let value = e.mem.u32(flags) | 0x40;
        e.mem.set_u32(flags, value);
    }
    if sound_data_flags(e, sound) & 0x800 != 0 {
        let value = e.mem.u32(flags) | 0x0800_0000;
        e.mem.set_u32(flags, value);
    }
    let data_flags = sound_data_flags(e, sound);
    let bits = e.call(SOUND_FLAG_BITS, &args![data_flags]).u32();
    let value = bits | e.mem.u32(flags);
    e.mem.set_u32(flags, value);
    e.mem.set_u32(sound_out.addr(), sound);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082_d150, fn_0082d150(u32, Ptr, Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sound_doubles(e: &mut Engine) {
        e.map(DATA_HANDLER, 4);
        e.set_global(DATA_HANDLER, 0x6600u32);
        // The sound id is the sound's address (0 for none).
        e.register(GET_SOUND, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(SOUND_FILE_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(COPY_SOUND_DATA, |e, a| {
            let flags = e.mem.u32(a[0] + 0x48);
            e.mem.set_u32(a[1] + 4, flags);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(SOUND_FLAG_BITS, |_, a| Ret {
            eax: if a[0] & 0x10 != 0 { 0x10 } else { 0 },
            ..Ret::default()
        });
    }

    #[test]
    fn fn_0082d150_copies_the_name_and_builds_the_flags() {
        let mut e = Engine::new();
        sound_doubles(&mut e);
        let sound = e.mem.alloc(0x70);
        let name = e.mem.alloc(0x104);
        let flags = e.mem.alloc(4);
        let out = e.mem.alloc(4);
        let call = |e: &mut Engine, id: u32, name: u32| {
            e.call(0x0082_d150, &args![id, name, flags, out]).bool()
        };
        assert!(!call(&mut e, 0, name), "no sound");
        assert!(!call(&mut e, sound, name), "no file name");
        e.mem.set_u32(sound, 0x4444);
        assert!(!call(&mut e, sound, 0), "no destination");

        e.call_log = Some(vec![]);
        e.mem.set_u32(sound + 0x48, 0x40 | 1 | 0x800 | 0x10);
        e.mem.set_u32(flags, 2);
        assert!(call(&mut e, sound, name));
        assert_eq!(e.mem.u32(flags), 1 | 0x40 | 0x0800_0000 | 0x10);
        assert_eq!(e.mem.u32(out), sound);
        let copies: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == STRCPY_S)
            .collect();
        assert_eq!(copies, vec![(STRCPY_S, vec![name, 0x104, 0x4444])]);

        e.mem.set_u32(sound + 0x48, 0);
        e.mem.set_u32(flags, 0x100);
        assert!(call(&mut e, sound, name));
        assert_eq!(e.mem.u32(flags), 0x102, "neither bit: bit 1 set");
        e.mem.set_u32(flags, 0x101);
        assert!(call(&mut e, sound, name));
        assert_eq!(e.mem.u32(flags), 0x101, "bit 0 kept");
    }
}
