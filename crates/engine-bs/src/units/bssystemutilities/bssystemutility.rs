//! `BSSystemUtilities/bssystemutility.cpp` (Xbox PDB source unit), subsystem
//! `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The singleton getter and constructor (`00af2640`, `00af30a0`) are in
//! `engine-core`'s `platform.rs`. `BSSystemUtility` starts, steps and stops the other system
//! utilities, each through its singleton getter and its vtable (slot +4 its start, slot +8 its
//! stop).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its release (`00aa4060`, `thiscall`, one word), the
/// destructor body (`00af3120`, `thiscall`) and the C runtime's `strlen` (`00ec6130`),
/// `memset` (`00ec61c0`) and `strncpy_s` (`00ec8d5f`, destination, size, source, count),
/// cdecl.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
const DESTRUCT: u32 = 0x00af_3120;
const STRLEN: u32 = 0x00ec_6130;
const MEMSET: u32 = 0x00ec_61c0;
const STRNCPY_S: u32 = 0x00ec_8d5f;
/// The singleton getters of the utilities started through their vtables, in the order
/// `Initialize` starts them: system info (`00af3850`), disc boot (`00af37d0`), save data
/// (`00af2e50`), the strings (`00af35e0`), the game data (`00af34f0`), the cache drive
/// (`00af3470`).
const SYS_INFO: u32 = 0x00af_3850;
const DISC_BOOT: u32 = 0x00af_37d0;
const SAVE_DATA: u32 = 0x00af_2e50;
const STRINGS: u32 = 0x00af_35e0;
const GAME_DATA: u32 = 0x00af_34f0;
const CACHE_DRIVE: u32 = 0x00af_3470;
/// The awards (`00af22d0` getter, `00af2360` start, `00af2370` release, `00af2480` idle), the
/// network (`00af2c90` getter, `00af2d20` start, `00af2d30` stop, `00af2db0` idle) and the DLC
/// (`00af26c0` getter, `00af2750` start, `00af2760` release, `00af2c80` idle) utilities.
const AWARDS: u32 = 0x00af_22d0;
const AWARDS_INITIALIZE: u32 = 0x00af_2360;
const AWARDS_DESTROY: u32 = 0x00af_2370;
const AWARDS_IDLE: u32 = 0x00af_2480;
const NETWORK: u32 = 0x00af_2c90;
const NETWORK_INITIALIZE: u32 = 0x00af_2d20;
const NETWORK_DESTROY: u32 = 0x00af_2d30;
const NETWORK_IDLE: u32 = 0x00af_2db0;
const DLC: u32 = 0x00af_26c0;
const DLC_INITIALIZE: u32 = 0x00af_2750;
const DLC_DESTROY: u32 = 0x00af_2760;
const DLC_IDLE: u32 = 0x00af_2c80;

// Translated from 00af3140 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemUtility::Initialize` (engine map; `thiscall`, three words: a name and two words
/// for the disc boot utility, `RET 0xc`): the object's own vtable slot +8 first; a non-null
/// name is copied into the buffer at +4 (`strncpy_s` with at most 0x7f characters and that
/// count plus one as the size). Then the system info, disc boot (with the two words), save
/// data, strings, game data and cache drive utilities are started (their singletons' vtable
/// slot +4), and the awards, network and DLC utilities made and started. Answers the DLC
/// start's EAX.
pub fn bs_system_utility_initialize(e: &mut Engine, this: u32, name: u32, a: u32, b: u32) -> u32 {
    e.vcall(this, 0x8, &args![]);
    if name != 0 {
        let length = e.call(STRLEN, &args![name]).u32();
        let count = if length > 0x7f { 0x7f } else { length };
        e.call(
            STRNCPY_S,
            &args![this.wrapping_add(4), count.wrapping_add(1), name, count],
        );
    }
    let sys_info = e.call(SYS_INFO, &args![]).u32();
    e.vcall(sys_info, 0x4, &args![]);
    let disc_boot = e.call(DISC_BOOT, &args![]).u32();
    e.vcall(disc_boot, 0x4, &args![a, b]);
    for getter in [SAVE_DATA, STRINGS, GAME_DATA, CACHE_DRIVE] {
        let utility = e.call(getter, &args![]).u32();
        e.vcall(utility, 0x4, &args![]);
    }
    e.call(AWARDS, &args![]);
    e.call(AWARDS_INITIALIZE, &args![]);
    e.call(NETWORK, &args![]);
    e.call(NETWORK_INITIALIZE, &args![]);
    e.call(DLC, &args![]);
    e.call(DLC_INITIALIZE, &args![]).u32()
}

// Translated from 00af3250 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemUtility::Destroy` (engine map; `thiscall`, plain `RET`): the DLC utility is made
/// and released, the network one made and stopped, the awards one made and released; then the
/// system info, disc boot, save data, strings, game data and cache drive utilities are stopped
/// (their singletons' vtable slot +8). Last the 0x80-byte name at +4 is zeroed (`memset`), the
/// byte at +0x94 and the words at +0x90, +0x84 and +0x88 cleared.
pub fn bs_system_utility_destroy(e: &mut Engine, this: u32) {
    e.call(DLC, &args![]);
    e.call(DLC_DESTROY, &args![]);
    e.call(NETWORK, &args![]);
    e.call(NETWORK_DESTROY, &args![]);
    e.call(AWARDS, &args![]);
    e.call(AWARDS_DESTROY, &args![]);
    for getter in [
        SYS_INFO,
        DISC_BOOT,
        SAVE_DATA,
        STRINGS,
        GAME_DATA,
        CACHE_DRIVE,
    ] {
        let utility = e.call(getter, &args![]).u32();
        e.vcall(utility, 0x8, &args![]);
    }
    e.call(MEMSET, &args![this.wrapping_add(4), 0u32, 0x80u32]);
    e.mem.set_u8(this.wrapping_add(0x94), 0);
    e.mem.set_u32(this.wrapping_add(0x90), 0);
    e.mem.set_u32(this.wrapping_add(0x84), 0);
    e.mem.set_u32(this.wrapping_add(0x88), 0);
}

// Translated from 00af3340 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemUtility::Update` (engine map; `thiscall`, plain `RET`): the idle steps of the
/// awards (`00af2480`), network (`00af2db0`) and DLC (`00af2c80`) utilities, each on its
/// singleton.
pub fn bs_system_utility_update(e: &mut Engine, _this: u32) {
    let awards = e.call(AWARDS, &args![]).u32();
    e.call(AWARDS_IDLE, &args![awards]);
    let network = e.call(NETWORK, &args![]).u32();
    e.call(NETWORK_IDLE, &args![network]);
    let dlc = e.call(DLC, &args![]).u32();
    e.call(DLC_IDLE, &args![dlc]);
}

// Translated from 00af3370 (disassembled, FalloutNV.exe 1.4.0.525)
/// Sets a state byte (no label; `thiscall`, one word whose low byte is the state, `RET 4`):
/// when it differs from the byte at +0x94 and the callback at +0x88 is set, the callback
/// (cdecl, one word) gets the state (zero-extended); the byte at +0x94 takes the state.
pub fn fn_00af3370(e: &mut Engine, this: u32, state: u32) {
    let state = state as u8;
    if state != e.mem.u8(this.wrapping_add(0x94)) && e.mem.u32(this.wrapping_add(0x88)) != 0 {
        let callback = e.mem.u32(this.wrapping_add(0x88));
        e.call(callback, &args![u32::from(state)]);
    }
    e.mem.set_u8(this.wrapping_add(0x94), state);
}

// Translated from 00af33c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemUtility`'s scalar deleting destructor (engine map; `thiscall`, one word: the
/// flags, `RET 4`): the destructor body (`00af3120`), then with bit 0 of the flags the block
/// is freed (`00aa4060`). Answers `this`.
pub fn bs_system_utility_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00af_3140,
            bs_system_utility_initialize(u32, u32, u32, u32) -> u32
        ),
        entry!(0x00af_3250, bs_system_utility_destroy(u32)),
        entry!(0x00af_3340, bs_system_utility_update(u32)),
        entry!(0x00af_3370, fn_00af3370(u32, u32)),
        entry!(
            0x00af_33c0,
            bs_system_utility_scalar_deleting_destructor(u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const GETTERS: [u32; 6] = [
        SYS_INFO,
        DISC_BOOT,
        SAVE_DATA,
        STRINGS,
        GAME_DATA,
        CACHE_DRIVE,
    ];
    const OBJECTS: u32 = 0x0800_0000;
    const VTABLES: u32 = 0x0900_0000;
    const START: u32 = 0x0a00_0000;
    const STOP: u32 = 0x0a00_0100;
    const OWN_STOP: u32 = 0x0a00_0200;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles for the utilities: getter `i` answers object `OBJECTS + 0x10 * i`, whose vtable
    /// has the start at +4 and the stop at +8; the awards, network and DLC calls answer fixed
    /// words.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(OBJECTS, 0x100);
        e.put_vtable(VTABLES, &[0, START, STOP]);
        e.put_vtable(VTABLES + 0x10, &[0, 0, OWN_STOP]);
        for (i, getter) in GETTERS.iter().enumerate() {
            let object = OBJECTS + 0x10 * i as u32;
            e.mem.set_u32(object, VTABLES);
            e.register_double(*getter, move |_, _| object.into_ret());
        }
        for (addr, value) in [
            (AWARDS, 0xa1u32),
            (AWARDS_INITIALIZE, 0),
            (AWARDS_DESTROY, 0),
            (AWARDS_IDLE, 0),
            (NETWORK, 0xb1),
            (NETWORK_INITIALIZE, 0),
            (NETWORK_DESTROY, 0),
            (NETWORK_IDLE, 0),
            (DLC, 0xc1),
            (DLC_INITIALIZE, 0x1234_5601),
            (DLC_DESTROY, 0),
            (DLC_IDLE, 0),
            (START, 0),
            (STOP, 0),
            (OWN_STOP, 0),
        ] {
            e.register_double(addr, move |_, _| value.into_ret());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn order(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| wanted.contains(&c.0))
            .map(|c| c.0)
            .collect()
    }

    #[test]
    fn initialize_copies_the_name_and_starts_every_utility_in_order() {
        let mut e = engine();
        let this = e.mem.alloc(0x98);
        e.mem.set_u32(this, VTABLES + 0x10);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"nv\0");
        assert_eq!(
            e.call(0x00af_3140, &args![this, name, 5u32, 6u32]).u32(),
            0x1234_5601
        );
        assert_eq!(calls(&e, OWN_STOP), vec![vec![this]]);
        assert_eq!(calls(&e, STRNCPY_S), vec![vec![this + 4, 3, name, 2]]);
        assert_eq!(e.mem.cstr(this + 4), b"nv".to_vec());
        let starts = calls(&e, START);
        assert_eq!(starts.len(), 6);
        assert_eq!(starts[0], vec![OBJECTS]);
        assert_eq!(starts[1], vec![OBJECTS + 0x10, 5, 6]);
        for (i, s) in starts.iter().enumerate().skip(2) {
            assert_eq!(*s, vec![OBJECTS + 0x10 * i as u32]);
        }
        assert_eq!(
            order(
                &e,
                &[
                    AWARDS,
                    AWARDS_INITIALIZE,
                    NETWORK,
                    NETWORK_INITIALIZE,
                    DLC,
                    DLC_INITIALIZE
                ]
            ),
            vec![
                AWARDS,
                AWARDS_INITIALIZE,
                NETWORK,
                NETWORK_INITIALIZE,
                DLC,
                DLC_INITIALIZE
            ]
        );
    }

    #[test]
    fn a_long_name_is_cut_to_0x7f_characters() {
        let mut e = engine();
        let this = e.mem.alloc(0x98);
        e.mem.set_u32(this, VTABLES + 0x10);
        let name = e.mem.alloc(0x100);
        e.mem.write(name, &[b'a'; 0x90]);
        bs_system_utility_initialize(&mut e, this, name, 0, 0);
        assert_eq!(calls(&e, STRNCPY_S), vec![vec![this + 4, 0x80, name, 0x7f]]);
        assert_eq!(e.mem.cstr(this + 4).len(), 0x7f);
        // Without a name nothing is copied.
        e.call_log = Some(vec![]);
        bs_system_utility_initialize(&mut e, this, 0, 0, 0);
        assert!(calls(&e, STRLEN).is_empty());
    }

    #[test]
    fn destroy_stops_everything_and_clears_the_fields() {
        let mut e = engine();
        let this = e.mem.alloc(0x98);
        for offset in (0..0x98).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call(0x00af_3250, &args![this]);
        assert_eq!(
            order(
                &e,
                &[
                    DLC,
                    DLC_DESTROY,
                    NETWORK,
                    NETWORK_DESTROY,
                    AWARDS,
                    AWARDS_DESTROY
                ]
            ),
            vec![
                DLC,
                DLC_DESTROY,
                NETWORK,
                NETWORK_DESTROY,
                AWARDS,
                AWARDS_DESTROY
            ]
        );
        let stops = calls(&e, STOP);
        assert_eq!(stops.len(), 6);
        for (i, s) in stops.iter().enumerate() {
            assert_eq!(*s, vec![OBJECTS + 0x10 * i as u32]);
        }
        assert_eq!(e.mem.u32(this), 0xffff_ffff);
        for offset in (4..0x84).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert_eq!(e.mem.u32(this + 0x88), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x90), 0);
        assert_eq!(e.mem.u32(this + 0x94), 0xffff_ff00);
    }

    #[test]
    fn update_idles_each_utility_on_its_singleton() {
        let mut e = engine();
        e.call(0x00af_3340, &args![0u32]);
        assert_eq!(calls(&e, AWARDS_IDLE), vec![vec![0xa1]]);
        assert_eq!(calls(&e, NETWORK_IDLE), vec![vec![0xb1]]);
        assert_eq!(calls(&e, DLC_IDLE), vec![vec![0xc1]]);
    }

    #[test]
    fn the_state_callback_hears_only_changes() {
        let mut e = engine();
        let this = e.mem.alloc(0x98);
        e.register(0x0b00_0000, |_, _| Ret::default());
        // No callback: only the byte.
        e.call(0x00af_3370, &args![this, 0x301u32]);
        assert_eq!(e.mem.u8(this + 0x94), 1);
        e.mem.set_u32(this + 0x88, 0x0b00_0000);
        fn_00af3370(&mut e, this, 1);
        assert!(calls(&e, 0x0b00_0000).is_empty());
        fn_00af3370(&mut e, this, 0x200);
        assert_eq!(calls(&e, 0x0b00_0000), vec![vec![0]]);
        assert_eq!(e.mem.u8(this + 0x94), 0);
    }

    #[test]
    fn the_deleting_destructor_restores_the_vtable_and_frees_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x98);
        e.register(DESTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x010a_42b4);
            Ret::default()
        });
        e.register(DEALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00af_33c0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010a_42b4);
        assert!(calls(&e, DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00af_33c0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, this]]);
    }
}
