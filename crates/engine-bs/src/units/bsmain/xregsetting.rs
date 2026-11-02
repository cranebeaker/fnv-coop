//! `BSMain/xregsetting.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `RegSettingCollection`: settings kept as values of a registry key under
//! `HKEY_LOCAL_MACHINE` (`0x80000002`). The vtable at `01082ed8` belongs to
//! the class (its RTTI type descriptor reads `.?AVRegSettingCollection@@`);
//! `WriteSetting` (`00c5d670`), `ReadSetting` (`00c5d780`), `Open` and
//! `Close` are its slots +0x10 to +0x1C, as in the Xbox PDB's vtable.
//!
//! The registry functions are called through their import slots. A
//! `Setting` (Xbox PDB, 0x0C bytes) keeps its value at +4 and its key
//! (`pKey`) at +8.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `RegSettingCollection` (Xbox PDB), 0x114 bytes.
    pub struct RegSettingCollection: 0x114 {
        /// `pSettingFile` (Xbox PDB): the key path, `char[260]` (its first
        /// byte here).
        0x004 pSettingFile: u8,
        /// `pHandle` (Xbox PDB): the open `HKEY`, 0 when closed.
        0x108 pHandle: u32,
    }
}

/// `Setting::uValue` and `Setting::pKey` (Xbox PDB).
const SETTING_VALUE: u32 = 0x04;
const SETTING_KEY: u32 = 0x08;

/// Registry imports (import slots).
const REG_CREATE_KEY_EX_A: u32 = 0x00fd_f000;
const REG_CLOSE_KEY: u32 = 0x00fd_f004;
const REG_QUERY_VALUE_EX_A: u32 = 0x00fd_f008;
const REG_OPEN_KEY_EX_A: u32 = 0x00fd_f00c;
const REG_SET_VALUE_EX_A: u32 = 0x00fd_f010;
/// `HKEY_LOCAL_MACHINE`.
const HKEY_LOCAL_MACHINE: u32 = 0x8000_0002;
/// `KEY_ALL_ACCESS`.
const KEY_ALL_ACCESS: u32 = 0xf003f;
/// `REG_SZ`, `REG_BINARY`, `REG_DWORD`.
const REG_SZ: u32 = 1;
const REG_BINARY: u32 = 3;
const REG_DWORD: u32 = 4;

/// `NiGetViewerString` (engine map, `00a7d330`, `cdecl (text)`) and
/// `NiGetViewerString_ov8` (engine map, `00a7d510`, `cdecl (text, value)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_VALUE: u32 = 0x00a7_d510;
/// `0096a610(list, &string)`: appends a viewer string to the list.
const VIEWER_LIST_ADD: u32 = 0x0096_a610;
/// `SettingCollectionList<Setting>::GetViewerStrings` (engine map,
/// `0044f890`): `(this, list)`, returns a count.
const LIST_GET_VIEWER_STRINGS: u32 = 0x0044_f890;
/// `"RegSettings"` and `"Total RegSettings"`.
const REG_SETTINGS_TEXT: u32 = 0x010c_2f98;
const TOTAL_REG_SETTINGS_TEXT: u32 = 0x010c_2f84;

/// `00c32fb0(setting)`: the size of a setting's value.
const SETTING_VALUE_SIZE: u32 = 0x00c3_2fb0;
/// `00c33090(key)` (`cdecl`): the type code of a setting key.
const SETTING_KEY_TYPE: u32 = 0x00c3_3090;
/// `00c33170(setting, text)`: stores a string value into the setting.
const SETTING_SET_STRING: u32 = 0x00c3_3170;
/// `_strcpy_s` (engine map), `cdecl (destination, size, source)`.
const STRCPY_S: u32 = 0x00ec_65a6;

/// `NEG EAX; SBB EAX,EAX; ADD EAX,1` on a status: 1 for `ERROR_SUCCESS`.
fn succeeded(status: u32) -> bool {
    status == 0
}

// Translated from 00c5d610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RegSettingCollection::GetViewerStrings` (Xbox PDB): adds the heading
/// "RegSettings", the collection's own viewer strings, and "Total
/// RegSettings" with their count, which it returns.
pub fn reg_setting_collection_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<RegSettingCollection>,
    list: u32,
) -> u32 {
    e.with_stack(8, |e, locals| {
        let heading = locals.addr() + 4;
        let total = locals.addr();
        let text = e
            .call(NI_GET_VIEWER_STRING, &args![REG_SETTINGS_TEXT])
            .u32();
        e.mem.set_u32(heading, text);
        e.call(VIEWER_LIST_ADD, &args![list, heading]);
        let count = e.call(LIST_GET_VIEWER_STRINGS, &args![this, list]).u32();
        let text = e
            .call(
                NI_GET_VIEWER_STRING_VALUE,
                &args![TOTAL_REG_SETTINGS_TEXT, count],
            )
            .u32();
        e.mem.set_u32(total, text);
        e.call(VIEWER_LIST_ADD, &args![list, total]);
        count
    })
}

/// Opens the key through `Open` (virtual slot +0x18) when it is not open;
/// returns whether it did, so the caller closes it again.
fn open_if_closed(e: &mut Engine, this: Ptr<RegSettingCollection>, create: bool) -> bool {
    let opened = e.get(this, RegSettingCollection::pHandle) == 0;
    if opened {
        e.vcall(this.addr(), 0x18, &args![create]);
    }
    opened
}

/// The value name for a string setting: the key copied into `buffer` (64
/// bytes) with its first character replaced by `'s'`.
fn string_value_name(e: &mut Engine, buffer: Ptr, key: u32) -> u32 {
    e.call(STRCPY_S, &args![buffer, 0x40u32, key]);
    e.mem.set_u8(buffer.addr(), b's');
    buffer.addr()
}

// Translated from 00c5d670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5d670 - the /GS cookie check (00ec408c) is not translated
/// `RegSettingCollection::WriteSetting` (slot +0x10, Xbox PDB vtable):
/// writes a setting that has a key as a registry value, opening the key
/// (creating it) for the write when it is not open and closing it again.
/// The value type is the key's type code (`00c33090`) as is; type 1
/// (`REG_SZ`) writes under the key with its first character replaced by
/// `'s'`. Returns whether `RegSetValueExA` succeeded (false without a key).
pub fn reg_setting_collection_write_setting(
    e: &mut Engine,
    this: Ptr<RegSettingCollection>,
    setting: u32,
) -> bool {
    let mut written = false;
    let mut name = e.mem.u32(setting.wrapping_add(SETTING_KEY));
    if name == 0 {
        return written;
    }
    let opened = open_if_closed(e, this, true);
    if e.get(this, RegSettingCollection::pHandle) != 0 {
        let size = e.call(SETTING_VALUE_SIZE, &args![setting]).u32();
        let kind = e.call(SETTING_KEY_TYPE, &args![name]).u32();
        written = e.with_stack(0x40, |e, buffer| {
            if kind == REG_SZ {
                name = string_value_name(e, buffer, name);
            }
            let key = e.get(this, RegSettingCollection::pHandle);
            let status = e
                .call(
                    REG_SET_VALUE_EX_A,
                    &args![
                        key,
                        name,
                        0u32,
                        kind,
                        setting.wrapping_add(SETTING_VALUE),
                        size
                    ],
                )
                .u32();
            succeeded(status)
        });
    }
    if opened {
        e.vcall(this.addr(), 0x1c, &[]);
    }
    written
}

// Translated from 00c5d780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5d780 - the /GS cookie check (00ec408c) is not translated
/// `RegSettingCollection::ReadSetting` (slot +0x14, Xbox PDB vtable): reads
/// a setting that has a key from the registry, opening the key (without
/// creating it) when it is not open and closing it again. The expected type
/// comes from `fn_00c5da10`; a string (`REG_SZ`) is read under the key with
/// its first character replaced by `'s'` into a 0x104-byte buffer and, on
/// success with the type still `REG_SZ`, stored through `00c33170`; other
/// values are read straight into the setting's value. Returns whether
/// `RegQueryValueExA` succeeded.
pub fn reg_setting_collection_read_setting(
    e: &mut Engine,
    this: Ptr<RegSettingCollection>,
    setting: u32,
) -> bool {
    let mut read = false;
    let mut name = e.mem.u32(setting.wrapping_add(SETTING_KEY));
    if name == 0 {
        return read;
    }
    let opened = open_if_closed(e, this, false);
    if e.get(this, RegSettingCollection::pHandle) != 0 {
        // Locals: the size and type words, the 0x104-byte string buffer
        // (0x10F bytes in the frame) and the 64-byte value name.
        read = e.with_stack(0x160, |e, frame| {
            let size_slot = frame.addr();
            let type_slot = size_slot + 4;
            let text = size_slot + 8;
            let name_buffer = Ptr::new(size_slot + 0x118);
            let size = e.call(SETTING_VALUE_SIZE, &args![setting]).u32();
            e.mem.set_u32(size_slot, size);
            let kind = fn_00c5da10(e, setting);
            e.mem.set_u32(type_slot, kind);
            let data = if e.mem.u32(type_slot) == REG_SZ {
                name = string_value_name(e, name_buffer, name);
                e.mem.set_u8(text, 0);
                e.mem.set_u32(size_slot, 0x104);
                text
            } else {
                setting.wrapping_add(SETTING_VALUE)
            };
            let key = e.get(this, RegSettingCollection::pHandle);
            let status = e
                .call(
                    REG_QUERY_VALUE_EX_A,
                    &args![key, name, 0u32, type_slot, data, size_slot],
                )
                .u32();
            let ok = succeeded(status);
            if ok && e.mem.u32(type_slot) == REG_SZ {
                e.call(SETTING_SET_STRING, &args![setting, text]);
            }
            ok
        });
    }
    if opened {
        e.vcall(this.addr(), 0x1c, &[]);
    }
    read
}

// Translated from 00c5d920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RegSettingCollection::Open` (Xbox PDB): opens the key named by
/// `pSettingFile` under `HKEY_LOCAL_MACHINE` with all access into
/// `pHandle`, creating it (`RegCreateKeyExA`) when `create` is set,
/// otherwise `RegOpenKeyExA`. Returns whether the call succeeded.
pub fn reg_setting_collection_open(
    e: &mut Engine,
    this: Ptr<RegSettingCollection>,
    create: bool,
) -> bool {
    let path = this.addr() + RegSettingCollection::pSettingFile.off;
    let handle = this.addr() + RegSettingCollection::pHandle.off;
    if create {
        e.with_stack(4, |e, disposition| {
            let status = e
                .call(
                    REG_CREATE_KEY_EX_A,
                    &args![
                        HKEY_LOCAL_MACHINE,
                        path,
                        0u32,
                        0u32,
                        0u32,
                        KEY_ALL_ACCESS,
                        0u32,
                        handle,
                        disposition
                    ],
                )
                .u32();
            succeeded(status)
        })
    } else {
        let status = e
            .call(
                REG_OPEN_KEY_EX_A,
                &args![HKEY_LOCAL_MACHINE, path, 0u32, KEY_ALL_ACCESS, handle],
            )
            .u32();
        succeeded(status)
    }
}

// Translated from 00c5d9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RegSettingCollection::Close` (Xbox PDB): closes an open key and clears
/// `pHandle`. Returns whether `RegCloseKey` succeeded (true when nothing
/// was open).
pub fn reg_setting_collection_close(e: &mut Engine, this: Ptr<RegSettingCollection>) -> bool {
    let mut closed = true;
    if e.get(this, RegSettingCollection::pHandle) != 0 {
        let key = e.get(this, RegSettingCollection::pHandle);
        let status = e.call(REG_CLOSE_KEY, &args![key]).u32();
        closed = succeeded(status);
        e.set(this, RegSettingCollection::pHandle, 0);
    }
    closed
}

// Translated from 00c5da10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The registry type of a setting from its key's type code (`00c33090`):
/// 0 is `REG_BINARY` (3), 6 is `REG_SZ` (1), anything else `REG_DWORD` (4).
pub fn fn_00c5da10(e: &mut Engine, setting: u32) -> u32 {
    let key = e.mem.u32(setting.wrapping_add(SETTING_KEY));
    match e.call(SETTING_KEY_TYPE, &args![key]).u32() {
        0 => REG_BINARY,
        6 => REG_SZ,
        _ => REG_DWORD,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c5d610,
            reg_setting_collection_get_viewer_strings(Ptr<RegSettingCollection>, u32) -> u32
        ),
        entry!(
            0x00c5d670,
            reg_setting_collection_write_setting(Ptr<RegSettingCollection>, u32) -> bool
        ),
        entry!(
            0x00c5d780,
            reg_setting_collection_read_setting(Ptr<RegSettingCollection>, u32) -> bool
        ),
        entry!(
            0x00c5d920,
            reg_setting_collection_open(Ptr<RegSettingCollection>, bool) -> bool
        ),
        entry!(
            0x00c5d9b0,
            reg_setting_collection_close(Ptr<RegSettingCollection>) -> bool
        ),
        entry!(0x00c5da10, fn_00c5da10(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPEN_SLOT: u32 = 0x0050_0018;
    const CLOSE_SLOT: u32 = 0x0050_001c;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A collection whose `Open` slot stores a handle and `Close` slot
    /// clears it, plus a setting with a key.
    fn setup() -> (Engine, Ptr<RegSettingCollection>, u32) {
        let mut e = Engine::new();
        e.put_vtable(0x0090_0000, &[0, 0, 0, 0, 0, 0, OPEN_SLOT, CLOSE_SLOT]);
        e.register(OPEN_SLOT, |e, a| {
            e.mem.set_u32(a[0] + 0x108, 0x4242);
            ret(1)
        });
        e.register(CLOSE_SLOT, |e, a| {
            e.mem.set_u32(a[0] + 0x108, 0);
            ret(1)
        });
        e.register(SETTING_VALUE_SIZE, |_, _| ret(4));
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let this: Ptr<RegSettingCollection> = e.new_object();
        e.mem.set_u32(this.addr(), 0x0090_0000);
        let setting = e.mem.alloc(0x10);
        let key = e.mem.alloc(0x10);
        e.mem.set_cstr(key, b"bFlag:General");
        e.mem.set_u32(setting + 8, key);
        (e, this, setting)
    }

    #[test]
    fn viewer_strings_heading_list_and_total() {
        let mut e = Engine::new();
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0x100));
        e.register(NI_GET_VIEWER_STRING_VALUE, |_, a| ret(0x200 + a[1]));
        e.register(LIST_GET_VIEWER_STRINGS, |_, _| ret(7));
        let added = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = added.clone();
        e.register_double(VIEWER_LIST_ADD, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let this: Ptr<RegSettingCollection> = e.new_object();
        e.call_log = Some(vec![]);
        let count = e.call(0x00c5_d610, &args![this, 0x77u32]).u32();
        assert_eq!(count, 7);
        assert_eq!(*added.borrow(), vec![(0x77, 0x100), (0x77, 0x207)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_GET_VIEWER_STRING, vec![REG_SETTINGS_TEXT]));
        assert!(log.contains(&(LIST_GET_VIEWER_STRINGS, vec![this.addr(), 0x77])));
        assert!(log.contains(&(NI_GET_VIEWER_STRING_VALUE, vec![TOTAL_REG_SETTINGS_TEXT, 7])));
    }

    #[test]
    fn write_setting_opens_writes_and_closes() {
        let (mut e, this, setting) = setup();
        e.register(SETTING_KEY_TYPE, |_, _| ret(REG_DWORD));
        e.register(REG_SET_VALUE_EX_A, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c5_d670, &args![this, setting]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (OPEN_SLOT, vec![this.addr(), 1]));
        let key = e.mem.u32(setting + 8);
        let set = log.iter().find(|(a, _)| *a == REG_SET_VALUE_EX_A).unwrap();
        assert_eq!(set.1, vec![0x4242, key, 0, REG_DWORD, setting + 4, 4]);
        assert_eq!(log.last().unwrap().0, CLOSE_SLOT);
        // Already open: neither opened nor closed; a failure returns false.
        e.set(this, RegSettingCollection::pHandle, 0x99);
        e.register(REG_SET_VALUE_EX_A, |_, _| ret(5));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c5_d670, &args![this, setting]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != OPEN_SLOT && *a != CLOSE_SLOT));
        // No key: nothing.
        e.mem.set_u32(setting + 8, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c5_d670, &args![this, setting]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn write_string_setting_uses_the_s_name() {
        let (mut e, this, setting) = setup();
        e.register(SETTING_KEY_TYPE, |_, _| ret(REG_SZ));
        e.register(REG_SET_VALUE_EX_A, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"sFlag:General");
            assert_eq!(a[3], REG_SZ);
            ret(0)
        });
        assert!(e.call(0x00c5_d670, &args![this, setting]).bool());
        // Open failed: no write, still closes (it "opened").
        let (mut e, this, setting) = setup();
        e.register(OPEN_SLOT, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c5_d670, &args![this, setting]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != REG_SET_VALUE_EX_A));
        assert_eq!(log.last().unwrap().0, CLOSE_SLOT);
    }

    #[test]
    fn read_setting_numbers_and_strings() {
        let (mut e, this, setting) = setup();
        // Type code 1 (not 0 or 6): REG_DWORD, read into the value.
        e.register(SETTING_KEY_TYPE, |_, _| ret(1));
        e.register(REG_QUERY_VALUE_EX_A, |e, a| {
            assert_eq!(e.mem.u32(a[3]), REG_DWORD);
            assert_eq!(e.mem.u32(a[5]), 4);
            e.mem.set_u32(a[4], 0x1234);
            ret(0)
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c5_d780, &args![this, setting]).bool());
        assert_eq!(e.mem.u32(setting + 4), 0x1234);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (OPEN_SLOT, vec![this.addr(), 0]));
        assert!(log.iter().all(|(a, _)| *a != SETTING_SET_STRING));

        // Type code 6: a string under the 's' name, stored on success.
        let (mut e, this, setting) = setup();
        e.register(SETTING_KEY_TYPE, |_, _| ret(6));
        e.register(REG_QUERY_VALUE_EX_A, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"sFlag:General");
            assert_eq!(e.mem.u32(a[5]), 0x104);
            e.mem.set_cstr(a[4], b"hello");
            ret(0)
        });
        let stored = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = stored.clone();
        e.register_double(SETTING_SET_STRING, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.cstr(a[1])));
            Ret::default()
        });
        assert!(e.call(0x00c5_d780, &args![this, setting]).bool());
        assert_eq!(*stored.borrow(), vec![(setting, b"hello".to_vec())]);

        // The registry reports another type: not stored as a string.
        e.register(REG_QUERY_VALUE_EX_A, |e, a| {
            e.mem.set_u32(a[3], REG_DWORD);
            ret(0)
        });
        assert!(e.call(0x00c5_d780, &args![this, setting]).bool());
        assert_eq!(stored.borrow().len(), 1);
        // Failure.
        e.register(REG_QUERY_VALUE_EX_A, |_, _| ret(2));
        assert!(!e.call(0x00c5_d780, &args![this, setting]).bool());
        assert_eq!(stored.borrow().len(), 1);
    }

    #[test]
    fn open_creates_or_opens_the_key() {
        let mut e = Engine::new();
        e.register(REG_CREATE_KEY_EX_A, |_, _| ret(0));
        e.register(REG_OPEN_KEY_EX_A, |_, _| ret(2));
        let this: Ptr<RegSettingCollection> = e.new_object();
        let t = this.addr();
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c5_d920, &args![this, 0x101u32]).bool());
        assert!(!e.call(0x00c5_d920, &args![this, 0x100u32]).bool());
        let log = e.call_log.take().unwrap();
        let create = &log[1];
        assert_eq!(create.0, REG_CREATE_KEY_EX_A);
        assert_eq!(
            &create.1[..8],
            &[
                HKEY_LOCAL_MACHINE,
                t + 4,
                0,
                0,
                0,
                KEY_ALL_ACCESS,
                0,
                t + 0x108
            ]
        );
        assert_eq!(
            log[3],
            (
                REG_OPEN_KEY_EX_A,
                vec![HKEY_LOCAL_MACHINE, t + 4, 0, KEY_ALL_ACCESS, t + 0x108]
            )
        );
    }

    #[test]
    fn close_closes_an_open_key() {
        let mut e = Engine::new();
        e.register(REG_CLOSE_KEY, |_, _| ret(6));
        let this: Ptr<RegSettingCollection> = e.new_object();
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c5_d9b0, &args![this]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set(this, RegSettingCollection::pHandle, 0x55);
        assert!(!e.call(0x00c5_d9b0, &args![this]).bool());
        assert_eq!(e.get(this, RegSettingCollection::pHandle), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (REG_CLOSE_KEY, vec![0x55]));
    }

    #[test]
    fn value_type_from_the_key_type() {
        let mut e = Engine::new();
        e.register(SETTING_KEY_TYPE, |_, a| ret(a[0]));
        let setting = e.mem.alloc(0x10);
        for (code, want) in [
            (0u32, REG_BINARY),
            (6, REG_SZ),
            (1, REG_DWORD),
            (9, REG_DWORD),
        ] {
            e.mem.set_u32(setting + 8, code);
            assert_eq!(e.call(0x00c5_da10, &args![setting]).u32(), want);
        }
    }
}
