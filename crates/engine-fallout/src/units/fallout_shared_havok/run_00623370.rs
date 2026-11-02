//! `fallout shared/havok/run_00623370` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006234d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0104dd5c` (RTTI `.?AVbhkContactListener@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006234f0`. One word, plain `RET`, returns nothing.
pub fn fn_006234d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_dd5c);
    e.call(0x0062_34f0, &args![this]);
}

/// `006232f0` (unnamed in the engine map): `thiscall`, `RET 4`.
const ENTRY_FOR: u32 = 0x0062_32f0;
/// `006815c0` (unnamed in the engine map): `thiscall`, plain `RET`.
const FIRST_VECTOR: u32 = 0x0068_15c0;
/// `00460140` (unnamed in the engine map): `thiscall`, plain `RET`.
const SECOND_VECTOR: u32 = 0x0046_0140;
/// `00458620` (unnamed in the engine map): `cdecl`, destination and source.
const COPY_VECTOR: u32 = 0x0045_8620;
/// `00c82870` (unnamed in the engine map): `cdecl`, a layer and a 64-bit mask (two words).
const SET_LAYER_MASK: u32 = 0x00c8_2870;
/// `00623570` (unnamed in the engine map): `thiscall`, plain `RET`; the base constructor.
const BASE_CONSTRUCT: u32 = 0x0062_3570;
/// `0096a2d0` (unnamed in the engine map): `thiscall`, plain `RET`.
const MEMBER_CONSTRUCT: u32 = 0x0096_a2d0;
/// `00470470` (unnamed in the engine map): `thiscall`, plain `RET`.
const MEMBER_INIT: u32 = 0x0047_0470;
/// The global that holds the listener `00623430` constructs.
const LISTENER_SINGLETON: u32 = 0x011c_c224;
/// The vtable `00623430` stores.
const LISTENER_VTABLE: u32 = 0x0104_dd38;
/// The vtable `006234b0` stores (RTTI `.?AVbhkContactListener@@`, as `006234d0` says).
const CONTACT_LISTENER_VTABLE: u32 = 0x0104_dd5c;
/// The vtable `006234f0` stores.
const BASE_VTABLE: u32 = 0x0104_dd7c;

// Translated from 00623370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 0xc`: for the entry of `key` (`006232f0`),
/// copies its first vector (`006815c0`) to `first` and its second (`00460140`) to `second`
/// (`00458620`, `cdecl`).
pub fn fn_00623370(e: &mut Engine, this: Ptr, key: u32, first: u32, second: u32) {
    let entry = e.call(ENTRY_FOR, &args![this, key]).u32();
    let vector = e.call(FIRST_VECTOR, &args![entry]).u32();
    e.call(COPY_VECTOR, &args![first, vector]);
    let vector = e.call(SECOND_VECTOR, &args![entry]).u32();
    e.call(COPY_VECTOR, &args![second, vector]);
}

// Translated from 006233c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4` (`this` is not read): for `kind` 0 sets the
/// collision mask of layer 0x27 to `0x26636`, for 1 to `0x20000100` (`00c82870`, high word 0);
/// any other kind does nothing.
pub fn fn_006233c0(e: &mut Engine, _this: Ptr, kind: u32) {
    let mask = match kind {
        0 => 0x0002_6636u32,
        1 => 0x2000_0100,
        _ => return,
    };
    e.call(SET_LAYER_MASK, &args![0x27u32, mask, 0u32]);
}

// Translated from 006234f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: stores the vtable `0104dd7c`.
pub fn fn_006234f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 006234b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: the base constructor `00623570`, then the vtable
/// `0104dd5c`. Answers `this`.
pub fn fn_006234b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), CONTACT_LISTENER_VTABLE);
    this
}

// Translated from 00623430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00623430 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `fastcall`, plain `RET`. A listener's constructor: `006234b0`, the
/// vtable `0104dd38`, the member at `+4` (`0096a2d0`), `this` kept in `011cc224`, then
/// `00470470` on the member. Answers `this`.
pub fn fn_00623430(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006234b0(e, this);
    e.mem.set_u32(this.addr(), LISTENER_VTABLE);
    let member = this.addr().wrapping_add(4);
    e.call(MEMBER_CONSTRUCT, &args![member]);
    e.mem.set_u32(LISTENER_SINGLETON, this.addr());
    e.call(MEMBER_INIT, &args![member]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_34d0, fn_006234d0(Ptr)),
        entry!(0x0062_3370, fn_00623370(Ptr, u32, u32, u32)),
        entry!(0x0062_33c0, fn_006233c0(Ptr, u32)),
        entry!(0x0062_3430, fn_00623430(Ptr) -> Ptr),
        entry!(0x0062_34b0, fn_006234b0(Ptr) -> Ptr),
        entry!(0x0062_34f0, fn_006234f0(Ptr)),
    ]
}

#[cfg(test)]
mod listener_tests {
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

    #[test]
    fn at_00623370_copies_both_vectors_of_the_entry() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ENTRY_FOR, |_, _| 0x5000u32.into_ret());
        e.register(FIRST_VECTOR, |_, a| (a[0] + 0x10).into_ret());
        e.register(SECOND_VECTOR, |_, a| (a[0] + 0x20).into_ret());
        e.register(COPY_VECTOR, |_, _| Ret::default());
        e.call(0x0062_3370, &args![0x4000u32, 7u32, 0x100u32, 0x200u32]);
        assert_eq!(calls(&e, ENTRY_FOR), vec![vec![0x4000, 7]]);
        assert_eq!(
            calls(&e, COPY_VECTOR),
            vec![vec![0x100, 0x5010], vec![0x200, 0x5020]]
        );
    }

    #[test]
    fn at_006233c0_sets_the_mask_of_each_kind() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(SET_LAYER_MASK, |_, _| Ret::default());
        e.call(0x0062_33c0, &args![0x4000u32, 0u32]);
        e.call(0x0062_33c0, &args![0x4000u32, 1u32]);
        e.call(0x0062_33c0, &args![0x4000u32, 2u32]);
        assert_eq!(
            calls(&e, SET_LAYER_MASK),
            vec![vec![0x27, 0x2_6636, 0], vec![0x27, 0x2000_0100, 0]]
        );
    }

    #[test]
    fn at_00623430_constructs_the_listener_and_keeps_it() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(0x011c_c000, 0x1000);
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(MEMBER_CONSTRUCT, |_, _| Ret::default());
        e.register(MEMBER_INIT, |e, a| {
            // The singleton is set before the member's set-up runs.
            assert_eq!(e.mem.u32(LISTENER_SINGLETON), a[0] - 4);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0062_3430, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LISTENER_VTABLE);
        assert_eq!(calls(&e, BASE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, MEMBER_CONSTRUCT), vec![vec![this + 4]]);
        assert_eq!(calls(&e, MEMBER_INIT), vec![vec![this + 4]]);
        assert_eq!(e.mem.u32(LISTENER_SINGLETON), this);
    }

    #[test]
    fn at_006234b0_and_006234f0_store_their_vtables() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x0062_34b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CONTACT_LISTENER_VTABLE);
        e.call(0x0062_34f0, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_006234d0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0062_34f0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0062_34d0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0062_34f0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0104_dd5c);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0104_dd5c,
            "the vtable was stored before the call"
        );
    }
}
