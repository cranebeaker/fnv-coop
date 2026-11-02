//! `BSMain/run_00c3bb80` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00aa4c60` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_00AA4C60: u32 = 0x00aa_4c60;
/// The vtable stored at `+0` (RTTI class `MessageHandler`).
const VTABLE_00C3BCF0: u32 = 0x010c_14a8;
/// The global that receives `this` (`.data`).
const INSTANCE_01202D74: u32 = 0x0120_2d74;

// Translated from 00c3bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00aa4c60` on `this`, stores the vtable `010c14a8` (RTTI
/// class `MessageHandler`) at `+0`, the low byte of its stack word at `+4`, and `this` in the
/// global at `01202d74`. Returns `this`.
pub fn fn_00c3bcf0(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    e.call(BASE_CONSTRUCTOR_00AA4C60, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_00C3BCF0);
    e.mem.set_u8(this.addr() + 4, flag);
    e.set_global(INSTANCE_01202D74, this.addr());
    this
}

/// `_strcpy_s` (`00ec65a6`, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `_strcat_s` (`00ec6bd2`, cdecl: destination, size, source).
const STRCAT_S: u32 = 0x00ec_6bd2;
/// The 0x104-byte directory string every path below starts with.
const BASE_DIRECTORY: u32 = 0x0120_2fa0;
/// Four 0x104-byte path buffers, each filled from the base directory and a name.
const PATH_C50: u32 = 0x0120_2c50;
const PATH_B48: u32 = 0x0120_2b48;
const PATH_A40: u32 = 0x0120_2a40;
const PATH_938: u32 = 0x0120_2938;
/// The set-up called first by `00c3bd30` (`00c3c460`: makes the message handler).
const CREATE_MESSAGE_HANDLER: u32 = 0x00c3_c460;
/// The four bytes `00c3bd30` stores.
const SETTING_BYTES: u32 = 0x011a_f49c;
/// The word `00c3bd30` stores.
const SETTING_WORD: u32 = 0x0120_2d5c;
/// The 0x18 bytes `00c3bd30` clears, and the word after them that receives the handler.
const CLEARED_BYTES: u32 = 0x011f_6370;
const HANDLER_COPY: u32 = 0x011f_6388;
/// `0065c1f0` (cdecl: a path and an 8-byte out value; returns 0 on success).
const FILE_INFO: u32 = 0x0065_c1f0;
/// `BSSystemFile::DeleteFileA` (`00aff0b0`, engine map; cdecl: the path).
const DELETE_FILE: u32 = 0x00af_f0b0;
/// The base destructor `00aa4c80` (thiscall).
const BASE_DESTRUCTOR: u32 = 0x00aa_4c80;

/// `strcpy_s(buffer, 0x104, base directory)`, then `strcat_s(buffer, 0x104, name)`.
fn build_path(e: &mut Engine, buffer: u32, name: Ptr) {
    e.call(STRCPY_S, &args![buffer, 0x104u32, BASE_DIRECTORY]);
    e.call(STRCAT_S, &args![buffer, 0x104u32, name]);
}

// Translated from 00c3bb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the path buffer at `01202c50`.
pub fn fn_00c3bb80(_e: &mut Engine) -> u32 {
    PATH_C50
}

// Translated from 00c3bb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the path buffer at `01202c50` to the base directory (`01202fa0`) followed by `name`.
pub fn fn_00c3bb90(e: &mut Engine, name: Ptr) {
    build_path(e, PATH_C50, name);
}

// Translated from 00c3bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the path buffer at `01202b48`.
pub fn fn_00c3bbd0(_e: &mut Engine) -> u32 {
    PATH_B48
}

// Translated from 00c3bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the path buffer at `01202b48` to the base directory (`01202fa0`) followed by `name`.
pub fn fn_00c3bbe0(e: &mut Engine, name: Ptr) {
    build_path(e, PATH_B48, name);
}

// Translated from 00c3bc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the path buffer at `01202a40`.
pub fn fn_00c3bc20(_e: &mut Engine) -> u32 {
    PATH_A40
}

// Translated from 00c3bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the path buffer at `01202a40` to the base directory (`01202fa0`) followed by `name`.
pub fn fn_00c3bc30(e: &mut Engine, name: Ptr) {
    build_path(e, PATH_A40, name);
}

// Translated from 00c3bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the path buffer at `01202938` to the base directory (`01202fa0`) followed by `name`.
pub fn fn_00c3bc80(e: &mut Engine, name: Ptr) {
    build_path(e, PATH_938, name);
}

// Translated from 00c3bc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the path buffer at `01202938` (no arguments, plain `RET`; nothing in the exe
/// refers to it).
pub fn fn_00c3bc70(_e: &mut Engine) -> u32 {
    PATH_938
}

// Translated from 00c3bcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (`PUSH EBP; MOV EBP,ESP; POP EBP; RET`; nothing in the exe refers to
/// it, so no caller shows arguments).
pub fn fn_00c3bcc0(_e: &mut Engine) {}

// Translated from 00c3bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0 (`XOR EAX,EAX`; plain `RET`, nothing in the exe refers to it).
pub fn fn_00c3bcd0(_e: &mut Engine) -> u32 {
    0
}

// Translated from 00c3bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (plain `RET`; nothing in the exe refers to it).
pub fn fn_00c3bce0(_e: &mut Engine) {}

// Translated from 00c3bd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start-up of this unit's state: makes the message handler (`00c3c460`), stores the low
/// bytes of the first four words in the bytes at `011af49c`..`011af49f` and the fifth word at
/// `01202d5c`, clears the 0x18 bytes at `011f6370` one by one, and copies the handler pointer
/// (`01202d74`) to `011f6388`.
pub fn fn_00c3bd30(e: &mut Engine, first: u8, second: u8, third: u8, fourth: u8, word: u32) {
    e.call(CREATE_MESSAGE_HANDLER, &args![]);
    for (i, byte) in [first, second, third, fourth].into_iter().enumerate() {
        e.set_global::<u8>(SETTING_BYTES + i as u32, byte);
    }
    e.set_global::<u32>(SETTING_WORD, word);
    for i in 0..0x18u32 {
        e.set_global::<u8>(CLEARED_BYTES + i, 0);
    }
    let handler = e.global::<u32>(INSTANCE_01202D74);
    e.set_global::<u32>(HANDLER_COPY, handler);
}

// Translated from 00c3bda0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3bda0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2dc48, state variable) is not translated
/// The `MessageHandler` destructor (vtable `010c14a8` by RTTI): stores the vtable; when
/// `0065c1f0` on the path at `01202c50` succeeds (returns 0) with the 8-byte value it fills in
/// equal to 0x46, deletes that file (`BSSystemFile::DeleteFileA`); clears the handler global
/// (`01202d74`) when it holds `this`; runs the base destructor `00aa4c80`.
pub fn fn_00c3bda0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_00C3BCF0);
    e.with_stack(8, |e, info| {
        let path = fn_00c3bb80(e);
        let result = e.call(FILE_INFO, &args![path, info]).u32();
        if result == 0 && e.mem.u32(info.addr()) == 0x46 && e.mem.u32(info.addr() + 4) == 0 {
            let path = fn_00c3bb80(e);
            e.call(DELETE_FILE, &args![path]);
        }
    });
    if e.global::<u32>(INSTANCE_01202D74) == this.addr() {
        e.set_global::<u32>(INSTANCE_01202D74, 0);
    }
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_bc70, fn_00c3bc70() -> u32),
        entry!(0x00c3_bcc0, fn_00c3bcc0()),
        entry!(0x00c3_bcd0, fn_00c3bcd0() -> u32),
        entry!(0x00c3_bce0, fn_00c3bce0()),
        entry!(0x00c3_bcf0, fn_00c3bcf0(Ptr, u8) -> Ptr),
        entry!(0x00c3_bb80, fn_00c3bb80() -> u32),
        entry!(0x00c3_bb90, fn_00c3bb90(Ptr)),
        entry!(0x00c3_bbd0, fn_00c3bbd0() -> u32),
        entry!(0x00c3_bbe0, fn_00c3bbe0(Ptr)),
        entry!(0x00c3_bc20, fn_00c3bc20() -> u32),
        entry!(0x00c3_bc30, fn_00c3bc30(Ptr)),
        entry!(0x00c3_bc80, fn_00c3bc80(Ptr)),
        entry!(0x00c3_bd30, fn_00c3bd30(u8, u8, u8, u8, u32)),
        entry!(0x00c3_bda0, fn_00c3bda0(Ptr)),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vtable_the_byte_and_the_instance() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.register(BASE_CONSTRUCTOR_00AA4C60, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xaaaa_aaaa);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_bcf0, &args![this, 0x7fu32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_00AA4C60),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_00C3BCF0);
        assert_eq!(e.mem.u32(this + 4), 0xaaaa_aa7f);
        assert_eq!(e.global::<u32>(INSTANCE_01202D74), this);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn path_getters_return_their_buffers() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_bb80, &args![]).u32(), PATH_C50);
        assert_eq!(e.call(0x00c3_bbd0, &args![]).u32(), PATH_B48);
        assert_eq!(e.call(0x00c3_bc20, &args![]).u32(), PATH_A40);
    }

    #[test]
    fn path_setters_copy_the_base_directory_then_append_the_name() {
        for (addr, buffer) in [
            (0x00c3_bb90u32, PATH_C50),
            (0x00c3_bbe0, PATH_B48),
            (0x00c3_bc30, PATH_A40),
            (0x00c3_bc80, PATH_938),
        ] {
            let mut e = Engine::new();
            e.register(STRCPY_S, |_, _| Ret::default());
            e.register(STRCAT_S, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(addr, &args![0x4444u32]);
            assert_eq!(
                log_of(&mut e, addr),
                vec![
                    (STRCPY_S, vec![buffer, 0x104, BASE_DIRECTORY]),
                    (STRCAT_S, vec![buffer, 0x104, 0x4444]),
                ]
            );
        }
    }

    #[test]
    fn start_up_stores_the_settings_and_clears_the_bytes() {
        let mut e = Engine::new();
        e.map(SETTING_BYTES, 4);
        e.map(SETTING_WORD, 4);
        e.map(INSTANCE_01202D74, 4);
        e.map(CLEARED_BYTES, 0x1c);
        for i in 0..0x18 {
            e.mem.set_u8(CLEARED_BYTES + i, 0xff);
        }
        e.register(CREATE_MESSAGE_HANDLER, |e, _| {
            e.mem.set_u32(INSTANCE_01202D74, 0x9999);
            Ret::default()
        });
        e.call(
            0x00c3_bd30,
            &args![0x101u32, 0x102u32, 0x103u32, 0x104u32, 0x5555u32],
        );
        assert_eq!(e.mem.u32(SETTING_BYTES), 0x0403_0201);
        assert_eq!(e.mem.u32(SETTING_WORD), 0x5555);
        for i in 0..0x18 {
            assert_eq!(e.mem.u8(CLEARED_BYTES + i), 0);
        }
        assert_eq!(e.mem.u32(HANDLER_COPY), 0x9999);
    }

    #[test]
    fn destructor_deletes_the_file_of_size_0x46_and_clears_the_instance() {
        type Case = (u32, u32, u32, bool, bool);
        // (result, low, high, deleted, this is the instance)
        let cases: [Case; 5] = [
            (0, 0x46, 0, true, true),
            (1, 0x46, 0, false, true),
            (0, 0x47, 0, false, false),
            (0, 0x46, 1, false, false),
            (0, 0x46, 0, true, false),
        ];
        for (result, low, high, deleted, instance) in cases {
            let mut e = Engine::new();
            e.map(INSTANCE_01202D74, 4);
            e.register_double(FILE_INFO, move |e, a| {
                e.mem.set_u32(a[1], low);
                e.mem.set_u32(a[1] + 4, high);
                result.into_ret()
            });
            e.register(DELETE_FILE, |_, _| Ret::default());
            e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
            let this = e.mem.alloc(8);
            e.mem
                .set_u32(INSTANCE_01202D74, if instance { this } else { 0x1234 });
            e.call_log = Some(vec![]);
            e.call(0x00c3_bda0, &args![this]);
            let log = log_of(&mut e, 0x00c3_bda0);
            assert_eq!(log[0].0, FILE_INFO);
            assert_eq!(log[0].1[0], PATH_C50);
            let deletes: Vec<_> = log.iter().filter(|(a, _)| *a == DELETE_FILE).collect();
            if deleted {
                assert_eq!(deletes, vec![&(DELETE_FILE, vec![PATH_C50])]);
            } else {
                assert!(deletes.is_empty());
            }
            assert_eq!(log.last().unwrap(), &(BASE_DESTRUCTOR, vec![this]));
            assert_eq!(e.mem.u32(this), VTABLE_00C3BCF0);
            assert_eq!(
                e.mem.u32(INSTANCE_01202D74),
                if instance { 0 } else { 0x1234 }
            );
        }
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn the_last_path_getter_and_the_stubs() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_bc70, &args![]).u32(), PATH_938);
        assert_eq!(e.call(0x00c3_bcd0, &args![]).u32(), 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_bcc0, &args![]);
        e.call(0x00c3_bce0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
