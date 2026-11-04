//! `fallout shared/magic/magicitemobject.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0` (thiscall on the form, returns `AL`; unnamed): when true, nothing is
/// initialized.
const FORM_TEST_004013E0: u32 = 0x0040_13e0;
/// `MagicItem::MagicItemInitItem` (`0040c030`, thiscall on the `MagicItem` part: the form).
const MAGIC_ITEM_MAGIC_ITEM_INIT_ITEM: u32 = 0x0040_c030;
/// `00484ab0` (thiscall on the form, one word; unnamed, `tesform.cpp`).
const FORM_00484AB0: u32 = 0x0048_4ab0;

// Translated from 0040c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicItemObject::InitItem` (Xbox PDB): unless `004013e0` says otherwise,
/// initializes the `MagicItem` part at +0x30 with the form, then calls `00484ab0`
/// with 1.
pub fn magic_item_object_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_TEST_004013E0, &args![this]).bool() {
        return;
    }
    e.call(
        MAGIC_ITEM_MAGIC_ITEM_INIT_ITEM,
        &args![this.addr() + 0x30, this],
    );
    e.call(FORM_00484AB0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_c9d0, magic_item_object_init_item(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_item_initializes_the_magic_item_part_unless_told_not_to() {
        for (skip, expected) in [(1u32, 0usize), (0, 2)] {
            let mut e = Engine::new();
            e.register_double(FORM_TEST_004013E0, move |_, _| Ret {
                eax: skip,
                ..Ret::default()
            });
            e.register(MAGIC_ITEM_MAGIC_ITEM_INIT_ITEM, |_, _| Ret::default());
            e.register(FORM_00484AB0, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0040_c9d0, &args![0x5000u32]);
            let log: Vec<(u32, Vec<u32>)> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 != 0x0040_c9d0 && c.0 != FORM_TEST_004013E0)
                .collect();
            assert_eq!(log.len(), expected);
            if expected != 0 {
                assert_eq!(
                    log,
                    vec![
                        (MAGIC_ITEM_MAGIC_ITEM_INIT_ITEM, vec![0x5030, 0x5000]),
                        (FORM_00484AB0, vec![0x5000, 1]),
                    ]
                );
            }
        }
    }
}
