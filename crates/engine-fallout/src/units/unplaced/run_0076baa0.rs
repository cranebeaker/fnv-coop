//! `(unplaced)/run_0076baa0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The list at `011d970c` that `0076bad0` adds to and sorts: `00905820`
/// (`thiscall`, a pointer to the element) and `0083fd60` (`thiscall`, a
/// compare function).
const SORTED_LIST: u32 = 0x011d_970c;
const LIST_ADD: u32 = 0x0090_5820;
const LIST_SORT: u32 = 0x0083_fd60;
/// The compare function `0076bad0` sorts with.
const COMPARE: u32 = 0x0076_bb00;
/// `HUDMainMenu`'s vtable (RTTI `.?AVHUDMainMenu@@`).
const HUD_MAIN_MENU_VTABLE: u32 = 0x0107_2df4;
/// The menu base constructor (`00a1c4a0`, `thiscall`; `BSMenu` by the
/// engine map).
const MENU_BASE_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The member constructors the HUD constructor runs (`thiscall`, no stack
/// words): `0096a2d0` (four members), `0077f970`, `0041a250`, `006815c0`.
const MEMBER_CONSTRUCTOR_0096A2D0: u32 = 0x0096_a2d0;
const MEMBER_CONSTRUCTOR_0077F970: u32 = 0x0077_f970;
const MEMBER_CONSTRUCTOR_0041A250: u32 = 0x0041_a250;
const MEMBER_CONSTRUCTOR_006815C0: u32 = 0x0068_15c0;
/// `memset` (`00403d30`, `cdecl`).
const MEMSET: u32 = 0x0040_3d30;
/// `00825c00` (`thiscall` on the object at `011f6394`): a word the
/// constructor samples twice.
const TIMER_OBJECT: u32 = 0x011f_6394;
const TIMER_READ: u32 = 0x0082_5c00;
/// The second sample's global.
const HUD_START_SAMPLE: u32 = 0x011d_96ac;
/// `00454af0` (`thiscall` on `011d8a38`; `fallout shared/tes.cpp`): the
/// answer that adds bit 6 to the HUD's element mask.
const TES_OBJECT_011D8A38: u32 = 0x011d_8a38;
const TES_QUERY_00454AF0: u32 = 0x0045_4af0;
/// `00457fe0` (no arguments; calls the import at `00fdf060`) and
/// `00aa50f0` (`thiscall`, one word; `BSCore`).
const CURRENT_THREAD_VALUE: u32 = 0x0045_7fe0;
const SET_OWNER_00AA50F0: u32 = 0x00aa_50f0;

// Translated from 0076bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): adds `element` (passed by
/// the address of its stack slot) to the list at `011d970c` (`00905820`),
/// then sorts the list with `0076bb00`.
pub fn fn_0076bad0(e: &mut Engine, element: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), element);
        e.call(LIST_ADD, &args![SORTED_LIST, slot]);
    });
    e.call(LIST_SORT, &args![SORTED_LIST, COMPARE]);
}

// Translated from 0076bb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): a compare function on
/// the signed word at +0x214 of both objects: 1 when the first's is
/// greater, -1 when it is smaller, 0 when equal (larger first).
pub fn fn_0076bb00(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let a = e.mem.u32(first.addr() + 0x214) as i32;
    let b = e.mem.u32(second.addr() + 0x214) as i32;
    if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    }
}

// Translation incomplete: 0076bb40 - the FS:[0] exception frame (unwinding the constructed members when a constructor throws) is not translated
// Translated from 0076bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `HUDMainMenu` constructor (by its vtable's RTTI; unnamed in the engine
/// map; `thiscall`, no stack words). Runs the menu base constructor, sets
/// the vtable, constructs the members at +0x18C, +0x198, +0x1AC, +0x1C8,
/// +0x224 (`0076bda0`), +0x258, +0x264 and +0x26C, clears 0x158 bytes at
/// +0x28, then sets the fields: +0x184 = 0, +0x188 = 0.0, +0x1B8, +0x21C,
/// +0x244 = 0, +0x248 = a sample of `00825c00`, +0x24C and +0x250 = -1, the
/// element mask +0x1C0 = `0x1FFBF` (plus bit 6 when `00454af0` answers true),
/// +0x1C4 = 4, byte +0x254 = 0; a second sample goes to `011d96ac`; then
/// byte +0x1FC = 0, +0x200 = 0, +0x274 = 0.0, bytes +0x204, +0x205, +0x260,
/// +0x261 = 0 and words +0x194, +0x1A8, +0x20C = 0 (fields unnamed here).
/// Returns `this`.
pub fn fn_0076bb40(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.call(MENU_BASE_CONSTRUCTOR, &args![at]);
    e.mem.set_u32(at, HUD_MAIN_MENU_VTABLE);
    e.call(MEMBER_CONSTRUCTOR_0096A2D0, &args![at + 0x18c]);
    e.call(MEMBER_CONSTRUCTOR_0077F970, &args![at + 0x198]);
    e.call(MEMBER_CONSTRUCTOR_0041A250, &args![at + 0x1ac]);
    e.call(MEMBER_CONSTRUCTOR_006815C0, &args![at + 0x1c8]);
    fn_0076bda0(e, Ptr::new(at + 0x224));
    e.call(MEMBER_CONSTRUCTOR_0096A2D0, &args![at + 0x258]);
    e.call(MEMBER_CONSTRUCTOR_0096A2D0, &args![at + 0x264]);
    e.call(MEMBER_CONSTRUCTOR_0096A2D0, &args![at + 0x26c]);
    e.call(MEMSET, &args![at + 0x28, 0u32, 0x158u32]);
    e.mem.set_u32(at + 0x184, 0);
    e.mem.set_u32(at + 0x188, 0.0f32.to_bits());
    e.mem.set_u32(at + 0x1b8, 0);
    e.mem.set_u32(at + 0x21c, 0);
    e.mem.set_u32(at + 0x244, 0);
    let sample = e.call(TIMER_READ, &args![TIMER_OBJECT]).u32();
    e.mem.set_u32(at + 0x248, sample);
    e.mem.set_u32(at + 0x24c, 0xffff_ffff);
    e.mem.set_u32(at + 0x250, 0xffff_ffff);
    let extra = e.call(TES_QUERY_00454AF0, &args![TES_OBJECT_011D8A38]).u8();
    let mask = if extra != 0 { 0x40 } else { 0 } | 0x0001_ffbf;
    e.mem.set_u32(at + 0x1c0, mask);
    e.mem.set_u32(at + 0x1c4, 4);
    e.mem.set_u8(at + 0x254, 0);
    let sample = e.call(TIMER_READ, &args![TIMER_OBJECT]).u32();
    e.mem.set_u32(HUD_START_SAMPLE, sample);
    e.mem.set_u8(at + 0x1fc, 0);
    e.mem.set_u32(at + 0x200, 0);
    e.mem.set_u32(at + 0x274, 0.0f32.to_bits());
    e.mem.set_u8(at + 0x204, 0);
    e.mem.set_u32(at + 0x194, 0);
    e.mem.set_u32(at + 0x1a8, 0);
    e.mem.set_u8(at + 0x205, 0);
    e.mem.set_u8(at + 0x260, 0);
    e.mem.set_u8(at + 0x261, 0);
    e.mem.set_u32(at + 0x20c, 0);
    this
}

// Translated from 0076bda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): a member
/// constructor that hands `00457fe0`'s answer (the value of the import at
/// `00fdf060`) to `00aa50f0` on `this`. Returns `this`.
pub fn fn_0076bda0(e: &mut Engine, this: Ptr) -> Ptr {
    let value = e.call(CURRENT_THREAD_VALUE, &[]).u32();
    e.call(SET_OWNER_00AA50F0, &args![this, value]);
    this
}

/// `005ae3d0` (`thiscall`, a pointer to the element): the other add of the list at
/// `011d970c`.
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;

// Translated from 0076baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): adds `element` (passed by the address of
/// its stack slot) to the list at `011d970c` through `005ae3d0`, then sorts the list with
/// `0076bb00` (`0083fd60`).
pub fn fn_0076baa0(e: &mut Engine, element: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), element);
        e.call(LIST_ADD_005AE3D0, &args![SORTED_LIST, slot]);
    });
    e.call(LIST_SORT, &args![SORTED_LIST, COMPARE]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0076_baa0, fn_0076baa0(u32)),
        entry!(0x0076bad0, fn_0076bad0(u32)),
        entry!(0x0076bb00, fn_0076bb00(Ptr, Ptr) -> i32),
        entry!(0x0076bb40, fn_0076bb40(Ptr) -> Ptr),
        entry!(0x0076bda0, fn_0076bda0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn adds_by_address_then_sorts() {
        let mut e = Engine::new();
        e.register(LIST_ADD, |e, a| e.mem.u32(a[1]).into_ret());
        e.register(LIST_SORT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0076_bad0, &args![0x4321u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(order, [0x0076_bad0, LIST_ADD, LIST_SORT]);
        assert_eq!(log[1].1[0], SORTED_LIST);
        assert_eq!(log[2].1, [SORTED_LIST, COMPARE]);
    }

    #[test]
    fn compare_orders_larger_first() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x218);
        let b = e.mem.alloc(0x218);
        e.mem.set_u32(a + 0x214, 5);
        e.mem.set_u32(b + 0x214, (-3i32) as u32);
        assert_eq!(e.call(0x0076_bb00, &args![a, b]).i32(), 1);
        assert_eq!(e.call(0x0076_bb00, &args![b, a]).i32(), -1);
        assert_eq!(e.call(0x0076_bb00, &args![a, a]).i32(), 0);
    }

    fn hud_engine(extra: bool) -> Engine {
        let mut e = Engine::new();
        for address in [
            MENU_BASE_CONSTRUCTOR,
            MEMBER_CONSTRUCTOR_0096A2D0,
            MEMBER_CONSTRUCTOR_0077F970,
            MEMBER_CONSTRUCTOR_0041A250,
            MEMBER_CONSTRUCTOR_006815C0,
            MEMSET,
            SET_OWNER_00AA50F0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(CURRENT_THREAD_VALUE, |_, _| 0x99u32.into_ret());
        e.register(TIMER_READ, |_, _| 1234u32.into_ret());
        if extra {
            e.register(TES_QUERY_00454AF0, |_, _| 1u32.into_ret());
        } else {
            e.register(TES_QUERY_00454AF0, |_, _| 0x100u32.into_ret());
        }
        e.map(0x011d_9000, 0x1000);
        e
    }

    #[test]
    fn hud_constructor_sets_the_fields() {
        let mut e = hud_engine(true);
        let this = e.mem.alloc(0x280);
        e.mem.set_u32(this + 0x184, 7);
        e.mem.set_u8(this + 0x261, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_bb40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), HUD_MAIN_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x184), 0);
        assert_eq!(e.mem.u32(this + 0x248), 1234);
        assert_eq!(e.mem.u32(this + 0x24c), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x1c0), 0x0001_ffff);
        assert_eq!(e.mem.u32(this + 0x1c4), 4);
        assert_eq!(e.mem.u8(this + 0x261), 0);
        assert_eq!(e.mem.u32(HUD_START_SAMPLE), 1234);
        assert_eq!(
            calls_to(&e, MEMBER_CONSTRUCTOR_0096A2D0),
            vec![
                vec![this + 0x18c],
                vec![this + 0x258],
                vec![this + 0x264],
                vec![this + 0x26c]
            ]
        );
        assert_eq!(calls_to(&e, MEMSET), vec![vec![this + 0x28, 0, 0x158]]);
        assert_eq!(
            calls_to(&e, SET_OWNER_00AA50F0),
            vec![vec![this + 0x224, 0x99]]
        );
        // Only the low byte of the answer counts.
        let mut e = hud_engine(false);
        let this = e.mem.alloc(0x280);
        e.call(0x0076_bb40, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1c0), 0x0001_ffbf);
    }

    #[test]
    fn member_constructor_hands_on_the_value() {
        let mut e = hud_engine(false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_bda0, &args![0x500u32]).u32(), 0x500);
        assert_eq!(calls_to(&e, SET_OWNER_00AA50F0), vec![vec![0x500, 0x99]]);
    }
}

#[cfg(test)]
mod tests_n077 {
    use super::*;

    #[test]
    fn fn_0076baa0_adds_through_the_argument_slot_then_sorts() {
        let mut e = Engine::new();
        e.register(LIST_ADD_005AE3D0, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1234);
            Ret::default()
        });
        e.register(LIST_SORT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0076_baa0, &args![0x1234u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 3);
        assert_eq!(log[1].0, LIST_ADD_005AE3D0);
        assert_eq!(log[1].1[0], SORTED_LIST);
        assert_eq!(log[2], (LIST_SORT, vec![SORTED_LIST, COMPARE]));
    }
}
