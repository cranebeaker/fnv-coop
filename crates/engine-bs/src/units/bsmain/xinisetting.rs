//! `BSMain/xinisetting.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `INISettingCollection`'s open handle (`+0x108`): set to the collection
/// itself while open, 0 when closed.
const OPEN_HANDLE_OFFSET: u32 = 0x108;

// Translated from 00c43d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `INISettingCollection::Open` (Xbox PDB, engine map): marks the
/// collection open (its handle at `+0x108` becomes `this`) and returns
/// true. The argument is not read.
pub fn ini_setting_collection_open(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    e.mem
        .set_u32(this.addr().wrapping_add(OPEN_HANDLE_OFFSET), this.addr());
    true
}

// Translated from 00c43d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `INISettingCollection::Close` (Xbox PDB, engine map): clears the open
/// handle at `+0x108` and returns true.
pub fn ini_setting_collection_close(e: &mut Engine, this: Ptr) -> bool {
    e.mem
        .set_u32(this.addr().wrapping_add(OPEN_HANDLE_OFFSET), 0);
    true
}

/// `NiGetViewerString` (`00a7d330`, `cdecl`: text), its overload `00a7d510` (`cdecl`: name,
/// value) and the viewer string array's add (`0096a610`, `thiscall`, `RET 4`: address of the
/// string).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_OV8: u32 = 0x00a7_d510;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// `SettingCollectionList<Setting>::GetViewerStrings` (`0044f890`, engine map; `thiscall`,
/// `RET 4`: the array; answers a count).
const LIST_GET_VIEWER_STRINGS: u32 = 0x0044_f890;
/// `"INISettings"` (`010c1a84`) and `"Total INISettings"` (`010c1a70`).
const TITLE_TEXT: u32 = 0x010c_1a84;
const TOTAL_TEXT: u32 = 0x010c_1a70;

/// Adds the viewer string `text` (by address, as the exe passes it) to `strings`.
fn add_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
    });
}

// Translated from 00c43d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `INISettingCollection::GetViewerStrings` (engine map; `thiscall`, `RET 4`: the viewer
/// string array): `"INISettings"` (`NiGetViewerString`), the list's own strings
/// (`0044f890`, which answers a count), then `"Total INISettings"` with that count
/// (`00a7d510`), each added to the array (`0096a610`). Answers the count.
pub fn ini_setting_collection_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) -> u32 {
    let text = e.call(VIEWER_STRING, &args![TITLE_TEXT]).u32();
    add_viewer_string(e, strings, text);
    let count = e.call(LIST_GET_VIEWER_STRINGS, &args![this, strings]).u32();
    let text = e.call(VIEWER_STRING_OV8, &args![TOTAL_TEXT, count]).u32();
    add_viewer_string(e, strings, text);
    count
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4_3d00,
            ini_setting_collection_get_viewer_strings(Ptr, Ptr) -> u32
        ),
        entry!(0x00c4_3d60, ini_setting_collection_open(Ptr, u32) -> bool),
        entry!(0x00c4_3d80, ini_setting_collection_close(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_and_close_set_the_handle() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10c);
        assert!(e.call(0x00c4_3d60, &args![this, 7u32]).bool());
        assert_eq!(e.mem.u32(this + 0x108), this);
        assert!(e.call(0x00c4_3d80, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x108), 0);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn viewer_strings_add_the_title_the_list_and_the_total() {
        let mut e = Engine::new();
        e.register(VIEWER_STRING, |_, _| 0x51u32.into_ret());
        e.register(LIST_GET_VIEWER_STRINGS, |_, _| 7u32.into_ret());
        e.register(VIEWER_STRING_OV8, |_, a| (0x60 + a[1]).into_ret());
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = added.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let count = e.call(0x00c4_3d00, &args![0x100u32, 0x900u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(count, 7);
        assert!(log.contains(&(VIEWER_STRING, vec![TITLE_TEXT])));
        assert!(log.contains(&(LIST_GET_VIEWER_STRINGS, vec![0x100, 0x900])));
        assert!(log.contains(&(VIEWER_STRING_OV8, vec![TOTAL_TEXT, 7])));
        assert_eq!(*added.borrow(), vec![(0x900, 0x51), (0x900, 0x67)]);
    }
}
