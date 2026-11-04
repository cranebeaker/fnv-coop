//! `fallout shared/run_00518f80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00519020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout shared`; the class is unconfirmed). `thiscall`, `RET 4`:
/// stores its stack word at `+0x688` of `this`, a plain 32-bit move (the word is not loaded as a
/// float).
pub fn fn_00519020(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x688), value);
}

/// `0050ee20` (thiscall, reference): the object's vtable +0x14C with (reference, 0),
/// the node this function returns.
const OBJECT_3D: u32 = 0x0050_ee20;
/// `00407820` (thiscall): the word at +0x78 (the object's sound, null for none).
const OBJECT_SOUND: u32 = 0x0040_7820;
/// `00453a70` `BSAudio::QInstance` (Xbox PDB name in the engine map).
const BS_AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `005bb4d0` (thiscall): the byte at +6 of the audio system (non-zero skips
/// the precache).
const AUDIO_SKIPS_PRECACHE: u32 = 0x005b_b4d0;
/// `00511840` (thiscall): the sound's file name (`00408da0` on `this +
/// 0x30`).
const SOUND_FILE_NAME: u32 = 0x0051_1840;
/// `00ad8100` `BSAudio::Precache_ov3` (Xbox PDB name in the engine map;
/// thiscall: name, flags, sound).
const BS_AUDIO_PRECACHE: u32 = 0x00ad_8100;
/// `NiObjectNET::GetController` (`00a5c570`, thiscall, RTTI).
const GET_CONTROLLER: u32 = 0x00a5_c570;
/// The RTTI record `NiObjectNET::GetController` is asked for.
const CONTROLLER_RTTI: u32 = 0x011d_feac;
/// `00517fa0` (thiscall): flag 2 of the byte at +0x84.
const OBJECT_ANIMATES: u32 = 0x0051_7fa0;
/// `00477ba0` (thiscall): bit 0x00800000 of the reference's flags.
const REFERENCE_FLAG_00800000: u32 = 0x0047_7ba0;
/// `00aa13e0` (cdecl, byte count): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `008d15d0` (thiscall): the controller's constructor (0x3C bytes).
const CONTROLLER_CONSTRUCT: u32 = 0x008d_15d0;
/// The setting whose value (+4) the new controller gets (`00403e20` on it).
const CONTROLLER_RATE_HOLDER: u32 = 0x011c_93c4;
/// `00403e20` (thiscall): `this + 4`, or a zeroed global for a null `this`.
const FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// `00929240` (thiscall, float).
const CONTROLLER_SET_FLOAT: u32 = 0x0092_9240;
/// `0040e500` (thiscall, callback).
const CONTROLLER_SET_CALLBACK: u32 = 0x0040_e500;
/// The callback `0040e500` receives (`00518f80`).
const CONTROLLER_CALLBACK: u32 = 0x0051_8f80;
/// `00448a80`: the word at `01202ddc` (the BSX extra data's name).
const BSX_NAME: u32 = 0x0044_8a80;
/// `NiObjectNET::GetExtraData` (`00a5bdd0`, thiscall, name).
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// `BSXFlags::BSXFlags` (`00c42f70`, thiscall).
const BSX_FLAGS_CONSTRUCT: u32 = 0x00c4_2f70;
/// `NiObjectNET::AddExtraData` (`00a5bc40`, thiscall: name, data).
const ADD_EXTRA_DATA: u32 = 0x00a5_bc40;
/// `NiObjectNET::RemoveController` (`00a5c480`, thiscall, controller).
const REMOVE_CONTROLLER: u32 = 0x00a5_c480;

// Translated from 00519270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the bits `bits` of the word at +0xC.
pub fn fn_00519270(e: &mut Engine, this: Ptr, bits: u32) {
    let flags = e.mem.u32(this.addr().wrapping_add(0xc));
    e.mem.set_u32(this.addr().wrapping_add(0xc), !bits & flags);
}

// Translated from 00519230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets (`set` non-zero) or clears
/// (`00519270`) the bits `bits` of the word at +0xC.
pub fn fn_00519230(e: &mut Engine, this: Ptr, bits: u32, set: u8) {
    if set != 0 {
        let flags = e.mem.u32(this.addr().wrapping_add(0xc));
        e.mem.set_u32(this.addr().wrapping_add(0xc), flags | bits);
    } else {
        fn_00519270(e, this, bits);
    }
}

// Translated from 00519040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: makes the object's 3D for `reference`
/// (`0050ee20`) and returns it. Precaches the object's sound first when it
/// has one and the audio system does not skip it. When there is a node:
/// for an object with flag 2 at +0x84 (`00517fa0`) and a reference without flag
/// 0x00800000, adds a new controller (0x3C bytes, `008d15d0`, the value of the
/// setting at `011c93c4`, callback `00518f80`, attached through its vtable +0x98) if
/// the node has none of the RTTI at `011dfeac`, and makes sure the node has
/// `BSXFlags` extra data with bit 1 set; otherwise removes that controller
/// if the node has one.
// Translation incomplete: 00519040 - SEH exception unwinding (the allocations' cleanup on unwind) is unsupported.
pub fn fn_00519040(e: &mut Engine, this: Ptr, reference: u32) -> u32 {
    let node = e.call(OBJECT_3D, &args![this, reference]).u32();
    if e.call(OBJECT_SOUND, &args![this]).u32() != 0 {
        let audio = e.call(BS_AUDIO_INSTANCE, &[]).u32();
        if !e.call(AUDIO_SKIPS_PRECACHE, &args![audio]).bool() {
            let sound = e.call(OBJECT_SOUND, &args![this]).u32();
            let named = e.call(OBJECT_SOUND, &args![this]).u32();
            let name = e.call(SOUND_FILE_NAME, &args![named]).u32();
            let audio = e.call(BS_AUDIO_INSTANCE, &[]).u32();
            e.call(BS_AUDIO_PRECACHE, &args![audio, name, 0x121u32, sound]);
        }
    }
    if node == 0 {
        return node;
    }
    let controller = e.call(GET_CONTROLLER, &args![node, CONTROLLER_RTTI]).u32();
    let animates = e.call(OBJECT_ANIMATES, &args![this]).bool()
        && (reference == 0 || !e.call(REFERENCE_FLAG_00800000, &args![reference]).bool());
    if animates {
        if controller == 0 {
            let block = e.call(NI_ALLOCATE, &args![0x3cu32]).u32();
            let created = if block != 0 {
                e.call(CONTROLLER_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            let rate_at = e.call(FLOAT_ADDRESS, &args![CONTROLLER_RATE_HOLDER]).u32();
            let rate = x87_quiet_bits(e.mem.u32(rate_at));
            e.call(CONTROLLER_SET_FLOAT, &args![created, rate]);
            e.call(
                CONTROLLER_SET_CALLBACK,
                &args![created, CONTROLLER_CALLBACK],
            );
            e.vcall(created, 0x98, &args![node]);
        }
        let name = e.call(BSX_NAME, &[]).u32();
        let mut flags = e.call(GET_EXTRA_DATA, &args![node, name]).u32();
        if flags == 0 {
            let block = e.call(NI_ALLOCATE, &args![0x10u32]).u32();
            flags = if block != 0 {
                e.call(BSX_FLAGS_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            let name = e.call(BSX_NAME, &[]).u32();
            e.call(ADD_EXTRA_DATA, &args![node, name, flags]);
        }
        fn_00519230(e, Ptr::new(flags), 1, 1);
    } else if controller != 0 {
        e.call(REMOVE_CONTROLLER, &args![node, controller]);
    }
    node
}

/// `TESObjectREFR::FindReferenceFor3D` (`0056f930`, engine map; `cdecl`,
/// one word), `BGSSaveFormBuffer::GetForm` (`007af430`, engine map name;
/// `thiscall` on the reference: its base form) and `00401170` (`thiscall`:
/// the form's type).
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
const FORM_TYPE: u32 = 0x0040_1170;
/// The player pointer (`011dea3c`), `008d8520` (`thiscall` on it, no stack
/// words; its engine map name, `MiddleHighProcess::GetSavedAcquireObject`,
/// is unconfirmed for this use: its answer is cast as a process) and the
/// type descriptors `.?AVBaseProcess@@` (`0118a6b8`) and
/// `.?AVHighProcess@@` (`0118a6d4`) for `__RTDynamicCast` (`00ec43fb`).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_008D8520: u32 = 0x008d_8520;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_BASE_PROCESS: u32 = 0x0118_a6b8;
const TYPE_HIGH_PROCESS: u32 = 0x0118_a6d4;

// Translated from 00518f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: the 3D object; the
/// callback `00519040` hands its controller): finds the reference for the
/// 3D (`0056f930`); when there is one whose base form is of type 0x1C and
/// whose `00477ba0` answers false, and the player's process (`008d8520`,
/// cast from `BaseProcess` to `HighProcess`) exists and answers other than
/// 4 at slot +0x610, stores the reference in the player (`00519020`).
pub fn fn_00518f80(e: &mut Engine, object: u32) {
    let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
    if reference == 0 {
        return;
    }
    let base = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    if e.call(FORM_TYPE, &args![base]).u32() != 0x1c {
        return;
    }
    if e.call(REFERENCE_FLAG_00800000, &args![reference]).bool() {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let process = e.call(PLAYER_008D8520, &args![player]).u32();
    let high = e
        .call(
            RT_DYNAMIC_CAST,
            &args![process, 0u32, TYPE_BASE_PROCESS, TYPE_HIGH_PROCESS, 0u32],
        )
        .u32();
    if high == 0 {
        return;
    }
    if e.vcall(high, 0x610, &args![]).u32() == 4 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    fn_00519020(e, Ptr::new(player), reference);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00519020, fn_00519020(Ptr, u32)),
        entry!(0x0051_9270, fn_00519270(Ptr, u32)),
        entry!(0x0051_9230, fn_00519230(Ptr, u32, u8)),
        entry!(0x0051_9040, fn_00519040(Ptr, u32) -> u32),
        entry!(0x0051_8f80, fn_00518f80(u32)),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00519020_stores_the_word_at_offset_0x688() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x700);
        e.mem.set_u32(this + 0x684, 0x1111_1111);
        e.mem.set_u32(this + 0x68c, 0x2222_2222);
        e.call(0x0051_9020, &args![this, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this + 0x688), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x684), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x68c), 0x2222_2222);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00519270_clears_the_bits_at_c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xff);
        e.call(0x0051_9270, &args![this, 0x0fu32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xf0);
    }

    #[test]
    fn fn_00519230_sets_or_clears() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x10);
        e.call(0x0051_9230, &args![this, 3u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x13);
        e.call(0x0051_9230, &args![this, 0x11u32, 0x100u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x02, "only the low byte selects");
    }

    /// The object: its sound at +0x78 (`00407820`), its animate flag at
    /// +0x84 (`00517fa0`), its node at +0x10 (`0050ee20`); the node's
    /// controller is its word at +4 and its BSX flags its word at +8; a
    /// reference's flag 0x00800000 is its word at +0.
    fn object_doubles(e: &mut Engine) {
        e.register(OBJECT_3D, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
        e.register(OBJECT_SOUND, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x78),
            ..Ret::default()
        });
        e.register(BS_AUDIO_INSTANCE, |_, _| Ret {
            eax: 0x6600,
            ..Ret::default()
        });
        e.register(AUDIO_SKIPS_PRECACHE, |_, _| Ret::default());
        e.register(SOUND_FILE_NAME, |_, a| Ret {
            eax: a[0] + 0x30,
            ..Ret::default()
        });
        e.register(BS_AUDIO_PRECACHE, |_, _| Ret::default());
        e.register(GET_CONTROLLER, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(OBJECT_ANIMATES, |e, a| Ret {
            eax: (e.mem.u8(a[0] + 0x84) & 2 != 0) as u32,
            ..Ret::default()
        });
        e.register(REFERENCE_FLAG_00800000, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(CONTROLLER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x9100_0000);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.put_vtable(0x9100_0000, &[0; 0x27]);
        e.mem.set_u32(0x9100_0098, 0x7000_0098);
        e.register(0x7000_0098, |_, _| Ret::default());
        e.map(CONTROLLER_RATE_HOLDER, 8);
        e.set_global(CONTROLLER_RATE_HOLDER + 4, 0.25f32);
        e.register(FLOAT_ADDRESS, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(CONTROLLER_SET_FLOAT, |_, _| Ret::default());
        e.register(CONTROLLER_SET_CALLBACK, |_, _| Ret::default());
        e.register(BSX_NAME, |_, _| Ret {
            eax: 0x0bad_0b5c,
            ..Ret::default()
        });
        e.register(GET_EXTRA_DATA, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(BSX_FLAGS_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(ADD_EXTRA_DATA, |e, a| {
            e.mem.set_u32(a[0] + 8, a[2]);
            Ret::default()
        });
        e.register(REMOVE_CONTROLLER, |_, _| Ret::default());
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00519040_precaches_and_sets_up_the_controller_and_flags() {
        let mut e = Engine::new();
        object_doubles(&mut e);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x78, 0x5100);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_9040, &args![this, 0u32]).u32(), 0, "no node");
        assert_eq!(
            calls_to(&e, BS_AUDIO_PRECACHE),
            vec![vec![0x6600, 0x5130, 0x121, 0x5100]]
        );
        assert!(calls_to(&e, GET_CONTROLLER).is_empty());

        let node = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x10, node);
        e.mem.set_u32(this + 0x78, 0);
        e.mem.set_u8(this + 0x84, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_9040, &args![this, 0u32]).u32(), node);
        let created = calls_to(&e, CONTROLLER_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, CONTROLLER_SET_FLOAT),
            vec![vec![created, 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, CONTROLLER_SET_CALLBACK),
            vec![vec![created, CONTROLLER_CALLBACK]]
        );
        assert_eq!(calls_to(&e, 0x7000_0098), vec![vec![created, node]]);
        let flags = e.mem.u32(node + 8);
        assert_eq!(
            calls_to(&e, ADD_EXTRA_DATA),
            vec![vec![node, 0x0bad_0b5c, flags]]
        );
        assert_eq!(e.mem.u32(flags + 0xc) & 1, 1);
        assert!(calls_to(&e, BS_AUDIO_PRECACHE).is_empty());

        // A controller already there; flagged reference: removed instead.
        e.mem.set_u32(node + 4, 0x7777);
        let reference = e.mem.alloc(4);
        e.mem.set_u32(reference, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_9040, &args![this, reference]);
        assert_eq!(calls_to(&e, REMOVE_CONTROLLER), vec![vec![node, 0x7777]]);
        assert!(calls_to(&e, NI_ALLOCATE).is_empty());
        e.mem.set_u8(this + 0x84, 0);
        e.mem.set_u32(reference, 0);
        e.call_log = Some(vec![]);
        e.call(0x0051_9040, &args![this, reference]);
        assert_eq!(calls_to(&e, REMOVE_CONTROLLER).len(), 1, "not animating");
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    /// The reference is the object plus 0x100; its base form is itself and
    /// its type the word at +0x10; `00477ba0` answers the word at +0x14; the
    /// player's process is the word at +0x20 of the player; the process
    /// answers the word at +0x24 at slot +0x610.
    fn setup(e: &mut Engine) -> (u32, u32) {
        e.register(FIND_REFERENCE_FOR_3D, |_, a| Ret {
            eax: if a[0] == 0 { 0 } else { a[0] + 0x100 },
            ..Ret::default()
        });
        e.register(REFERENCE_BASE_FORM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(FORM_TYPE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
        e.register(REFERENCE_FLAG_00800000, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x14),
            ..Ret::default()
        });
        e.register(PLAYER_008D8520, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x20),
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| {
            assert_eq!(a[1..], [0, TYPE_BASE_PROCESS, TYPE_HIGH_PROCESS, 0]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(0x0090_0610, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x24),
            ..Ret::default()
        });
        e.map(0x011d_e000, 0x1000);
        let player = e.mem.alloc(0x700);
        e.set_global(PLAYER, player);
        let process = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x700);
        e.mem.set_u32(process, vtable);
        e.mem.set_u32(vtable + 0x610, 0x0090_0610);
        e.mem.set_u32(player + 0x20, process);
        let object = e.mem.alloc(0x200);
        let reference = object + 0x100;
        e.mem.set_u32(reference + 0x10, 0x1c);
        (player, object)
    }

    #[test]
    fn fn_00518f80_stores_the_reference_in_the_player() {
        let mut e = Engine::new();
        let (player, object) = setup(&mut e);
        let reference = object + 0x100;
        let process = e.mem.u32(player + 0x20);
        // No reference.
        e.call(0x0051_8f80, &[0]);
        assert_eq!(e.mem.u32(player + 0x688), 0);
        // Wrong type.
        e.mem.set_u32(reference + 0x10, 0x1b);
        e.call(0x0051_8f80, &[object]);
        assert_eq!(e.mem.u32(player + 0x688), 0);
        e.mem.set_u32(reference + 0x10, 0x1c);
        // Flagged reference.
        e.mem.set_u32(reference + 0x14, 1);
        e.call(0x0051_8f80, &[object]);
        assert_eq!(e.mem.u32(player + 0x688), 0);
        e.mem.set_u32(reference + 0x14, 0);
        // The process answers 4.
        e.mem.set_u32(process + 0x24, 4);
        e.call(0x0051_8f80, &[object]);
        assert_eq!(e.mem.u32(player + 0x688), 0);
        // No process.
        e.mem.set_u32(process + 0x24, 3);
        e.mem.set_u32(player + 0x20, 0);
        e.call(0x0051_8f80, &[object]);
        assert_eq!(e.mem.u32(player + 0x688), 0);
        e.mem.set_u32(player + 0x20, process);
        e.call(0x0051_8f80, &[object]);
        assert_eq!(e.mem.u32(player + 0x688), reference);
    }
}
