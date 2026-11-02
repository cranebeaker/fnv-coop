//! `(unplaced)/run_0065bee0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0065bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `0065bf20` and frees 0x10 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_0065bef0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0065_bf20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x10u32]);
    }
    this
}

// Translated from 0065bee0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5b58 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_0065bee0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5b58
}

/// The `NiPointer` destructor (`0045cec0`; `thiscall`) and the `NiExtraData` destructor
/// (`00a7b300`; `thiscall`).
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// The memory context scope (`00404eb0`; `thiscall`: id, flag, source file, line) and its
/// end (`00404ee0`; `thiscall`).
const MEMORY_SCOPE_BEGIN: u32 = 0x0040_4eb0;
const MEMORY_SCOPE_END: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\...\BSFaceGenModel.cpp"` (`01065b00`) and the scope's id 0x38, line 0x4F0.
const FACE_GEN_MODEL_SOURCE: u32 = 0x0106_5b00;
/// `00559450`: answers the pointer an `NiPointer` holds (`thiscall`).
const POINTER_GET: u32 = 0x0055_9450;
/// The model data's readiness test (`004048e0`; `thiscall`).
const MODEL_READY: u32 = 0x0040_48e0;
/// `00401000`: the game's `operator new` (`cdecl`: size).
const GAME_NEW: u32 = 0x0040_1000;
/// The morph data constructor (`00659560`; `thiscall`: the model's object).
const MORPH_DATA_CONSTRUCTOR: u32 = 0x0065_9560;
/// The morph data's in-use test (`007af430`; `thiscall`) and scalar deleting destructor
/// (`006599d0`; `thiscall`: flags).
const MORPH_DATA_IN_USE: u32 = 0x007a_f430;
const MORPH_DATA_DELETING_DESTRUCTOR: u32 = 0x0065_99d0;
/// The morph list's count (`00657c00`), item (`00657c20`: index) and the item's own count
/// (`00657bc0`), all `thiscall`.
const MORPH_LIST_COUNT: u32 = 0x0065_7c00;
const MORPH_LIST_ITEM: u32 = 0x0065_7c20;
const MORPH_ITEM_COUNT: u32 = 0x0065_7bc0;

// Translated from 0065bf20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065bf20 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// A destructor (unnamed in the engine map; `thiscall`): destroys the `NiPointer` at
/// `this + 0xC` (`0045cec0`), then the `NiExtraData` destructor (`00a7b300`).
pub fn fn_0065bf20(e: &mut Engine, this: Ptr) {
    e.call(POINTER_DESTRUCTOR, &args![this.addr() + 0xc]);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
}

// Translated from 0065bf80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065bf80 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (`thiscall`, plain `RET`), inside a memory context scope
/// (`00404eb0` with id 0x38, flag 1, `BSFaceGenModel.cpp`, line 0x4F0 .. `00404ee0`):
/// false without model data at `this + 8`. When the data has no morph data yet (`+8`) it
/// is created if the data is ready (`004048e0`; false if not): 0x34 bytes from `00401000`
/// constructed by `00659560` with the data's object (`00559450`), or null. Then true.
pub fn fn_0065bf80(e: &mut Engine, this: Ptr) -> bool {
    e.with_stack(4, |e, scope| {
        e.call(
            MEMORY_SCOPE_BEGIN,
            &args![scope, 0x38u32, 1u32, FACE_GEN_MODEL_SOURCE, 0x4f0u32],
        );
        let data = e.mem.u32(this.addr() + 8);
        if data == 0 {
            e.call(MEMORY_SCOPE_END, &args![scope]);
            return false;
        }
        if e.mem.u32(data + 8) == 0 {
            if e.call(MODEL_READY, &args![data]).u32() == 0 {
                e.call(MEMORY_SCOPE_END, &args![scope]);
                return false;
            }
            let block = e.call(GAME_NEW, &args![0x34u32]).u32();
            let morphs = if block == 0 {
                0
            } else {
                let data = e.mem.u32(this.addr() + 8);
                let object = e.call(POINTER_GET, &args![data]).u32();
                e.call(MORPH_DATA_CONSTRUCTOR, &args![block, object]).u32()
            };
            let data = e.mem.u32(this.addr() + 8);
            e.mem.set_u32(data + 8, morphs);
        }
        e.call(MEMORY_SCOPE_END, &args![scope]);
        true
    })
}

// Translated from 0065c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): with model data at `this + 8` that
/// has morph data (`+8`): false while the morph data is in use (`007af430`), else the morph
/// data is deleted (`006599d0` with flags 1) and its pointer cleared. True otherwise.
pub fn fn_0065c090(e: &mut Engine, this: Ptr) -> bool {
    let data = e.mem.u32(this.addr() + 8);
    if data == 0 || e.mem.u32(data + 8) == 0 {
        return true;
    }
    if e.call(MORPH_DATA_IN_USE, &args![data]).u32() != 0 {
        return false;
    }
    let data = e.mem.u32(this.addr() + 8);
    let morphs = e.mem.u32(data + 8);
    if morphs != 0 {
        e.call(MORPH_DATA_DELETING_DESTRUCTOR, &args![morphs, 1u32]);
    }
    let data = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(data + 8, 0);
    true
}

// Translated from 0065c110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the memory size of the model's
/// morph data, 0 without model data (`this + 8`) or morph data (`+8`): 0x34 plus, for both
/// morph lists (0x18 bytes apart from `+4`), 0x18 bytes per list entry (`00657c00`) and 6
/// bytes per count of each entry (`00657c20`, `00657bc0` on the entry's `+4`).
pub fn fn_0065c110(e: &mut Engine, this: Ptr) -> u32 {
    let data = e.mem.u32(this.addr() + 8);
    if data == 0 || e.mem.u32(data + 8) == 0 {
        return 0;
    }
    let mut total = 0x34u32;
    for list_index in 0..2u32 {
        let list = |e: &Engine| {
            let data = e.mem.u32(this.addr() + 8);
            e.mem
                .u32(data + 8)
                .wrapping_add(list_index * 0x18)
                .wrapping_add(4)
        };
        let at = list(e);
        let entries = e.call(MORPH_LIST_COUNT, &args![at]).u32();
        total = entries.wrapping_mul(0x18).wrapping_add(total);
        let mut index = 0u32;
        loop {
            let at = list(e);
            if index >= e.call(MORPH_LIST_COUNT, &args![at]).u32() {
                break;
            }
            let at = list(e);
            let item = e.call(MORPH_LIST_ITEM, &args![at, index]).u32();
            let count = e.call(MORPH_ITEM_COUNT, &args![item.wrapping_add(4)]).u32();
            total = count.wrapping_mul(6).wrapping_add(total);
            index = index.wrapping_add(1);
        }
    }
    total
}

/// The file object's constructor (`00b00900`; `thiscall`: path, then three zero words),
/// destructor (`00b00950`), and the size query (`0065c290`; `thiscall`: address of the
/// 64-bit result).
const FILE_CONSTRUCTOR: u32 = 0x00b0_0900;
const FILE_DESTRUCTOR: u32 = 0x00b0_0950;
const FILE_GET_SIZE: u32 = 0x0065_c290;

// Translated from 0065c1f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065c1f0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (`cdecl`, two words: path, address of a 64-bit size). Opens a
/// 0x14-byte file object on the stack (`00b00900`); when its first check (`00559450`)
/// answers 0 the size query (`0065c290`) runs and its answer is returned, otherwise the
/// size is cleared and that check's answer returned. The file object is destroyed
/// (`00b00950`).
pub fn fn_0065c1f0(e: &mut Engine, path: u32, size: Ptr) -> u32 {
    e.with_stack(0x14, |e, file| {
        e.call(FILE_CONSTRUCTOR, &args![file, path, 0u32, 0u32, 0u32]);
        let mut answer = e.call(POINTER_GET, &args![file]).u32();
        if answer == 0 {
            answer = e.call(FILE_GET_SIZE, &args![file, size]).u32();
        } else {
            e.mem.set_u32(size.addr(), 0);
            e.mem.set_u32(size.addr() + 4, 0);
        }
        e.call(FILE_DESTRUCTOR, &args![file]);
        answer
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_bee0, fn_0065bee0(Ptr) -> u32),
        entry!(0x0065_bef0, fn_0065bef0(Ptr, u32) -> Ptr),
        entry!(0x0065_bf20, fn_0065bf20(Ptr)),
        entry!(0x0065_bf80, fn_0065bf80(Ptr) -> bool),
        entry!(0x0065_c090, fn_0065c090(Ptr) -> bool),
        entry!(0x0065_c110, fn_0065c110(Ptr) -> u32),
        entry!(0x0065_c1f0, fn_0065c1f0(u32, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn fn_0065bef0_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0065_bef0, 0x0065_bf20, 0x10);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0065bee0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0065_bee0, &args![0x1000u32]).u32(), 0x011d_5b58);
        assert_eq!(e.call(0x0065_bee0, &args![0u32]).u32(), 0x011d_5b58);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn model(e: &mut Engine, morphs: u32) -> (u32, u32) {
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 8, morphs);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, data);
        (this, data)
    }

    #[test]
    fn destructor_releases_the_pointer_then_the_base() {
        let mut e = Engine::new();
        e.register(POINTER_DESTRUCTOR, |_, _| Ret::default());
        e.register(EXTRA_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_bf20, &args![0x1000u32]);
        assert_eq!(calls(&e, POINTER_DESTRUCTOR), vec![vec![0x100c]]);
        assert_eq!(calls(&e, EXTRA_DATA_DESTRUCTOR), vec![vec![0x1000]]);
    }

    #[test]
    fn morph_data_is_created_when_ready() {
        let mut e = Engine::new();
        for address in [MEMORY_SCOPE_BEGIN, MEMORY_SCOPE_END] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MODEL_READY, |_, _| 1u32.into_ret());
        e.register(GAME_NEW, |_, _| 0x7000u32.into_ret());
        e.register(POINTER_GET, |_, _| 0x7100u32.into_ret());
        e.register(MORPH_DATA_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        let (this, data) = model(&mut e, 0);
        assert!(e.call(0x0065_bf80, &args![this]).bool());
        assert_eq!(e.mem.u32(data + 8), 0x7001);
        assert_eq!(
            calls(&e, MORPH_DATA_CONSTRUCTOR),
            vec![vec![0x7000, 0x7100]]
        );
        let scope = &calls(&e, MEMORY_SCOPE_BEGIN)[0];
        assert_eq!(scope[1..], [0x38, 1, FACE_GEN_MODEL_SOURCE, 0x4f0]);
        assert_eq!(calls(&e, MEMORY_SCOPE_END), vec![vec![scope[0]]]);
        // Already there: nothing new, still true.
        assert!(e.call(0x0065_bf80, &args![this]).bool());
        assert_eq!(calls(&e, GAME_NEW).len(), 1);
    }

    #[test]
    fn morph_data_creation_fails_without_data_or_readiness() {
        let mut e = Engine::new();
        for address in [MEMORY_SCOPE_BEGIN, MEMORY_SCOPE_END] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MODEL_READY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let (this, _) = model(&mut e, 0);
        assert!(!e.call(0x0065_bf80, &args![this]).bool());
        e.mem.set_u32(this + 8, 0);
        assert!(!e.call(0x0065_bf80, &args![this]).bool());
        assert_eq!(calls(&e, MEMORY_SCOPE_END).len(), 2);
        // A failed allocation stores null and still succeeds.
        e.register(MODEL_READY, |_, _| 1u32.into_ret());
        e.register(GAME_NEW, |_, _| Ret::default());
        let (this, data) = model(&mut e, 0);
        assert!(e.call(0x0065_bf80, &args![this]).bool());
        assert_eq!(e.mem.u32(data + 8), 0);
    }

    #[test]
    fn unload_deletes_unused_morph_data() {
        let mut e = Engine::new();
        e.register(MORPH_DATA_IN_USE, |_, _| Ret::default());
        e.register(MORPH_DATA_DELETING_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let (this, data) = model(&mut e, 0x6000);
        assert!(e.call(0x0065_c090, &args![this]).bool());
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(
            calls(&e, MORPH_DATA_DELETING_DESTRUCTOR),
            vec![vec![0x6000, 1]]
        );
        e.register(MORPH_DATA_IN_USE, |_, _| 1u32.into_ret());
        let (this, data) = model(&mut e, 0x6000);
        assert!(!e.call(0x0065_c090, &args![this]).bool());
        assert_eq!(e.mem.u32(data + 8), 0x6000);
        let (this, _) = model(&mut e, 0);
        assert!(e.call(0x0065_c090, &args![this]).bool());
        assert_eq!(calls(&e, MORPH_DATA_IN_USE).len(), 2);
    }

    #[test]
    fn size_counts_both_lists() {
        let mut e = Engine::new();
        let morphs = e.mem.alloc(0x40);
        // List 0 holds two entries (counts 1 and 2), list 1 one entry (count 3).
        e.register_double(MORPH_LIST_COUNT, move |_, a| {
            (if a[0] == morphs + 4 { 2u32 } else { 1 }).into_ret()
        });
        e.register_double(MORPH_LIST_ITEM, move |_, a| {
            (if a[0] == morphs + 4 {
                0x100 + a[1]
            } else {
                0x200
            })
            .into_ret()
        });
        e.register(MORPH_ITEM_COUNT, |_, a| {
            (match a[0] {
                0x104 => 1u32,
                0x105 => 2,
                _ => 3,
            })
            .into_ret()
        });
        e.call_log = Some(vec![]);
        let (this, _) = model(&mut e, morphs);
        let size = e.call(0x0065_c110, &args![this]).u32();
        assert_eq!(size, 0x34 + 3 * 0x18 + 6 * (1 + 2 + 3));
        assert_eq!(calls(&e, MORPH_LIST_ITEM)[2], vec![morphs + 0x1c, 0]);
        let (this, _) = model(&mut e, 0);
        assert_eq!(e.call(0x0065_c110, &args![this]).u32(), 0);
    }

    #[test]
    fn file_size_queries_only_an_open_file() {
        let mut e = Engine::new();
        e.register(FILE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(FILE_DESTRUCTOR, |_, _| Ret::default());
        e.register(POINTER_GET, |_, _| Ret::default());
        e.register(FILE_GET_SIZE, |_, _| 0x55u32.into_ret());
        let size = e.mem.alloc(8);
        e.mem.set_u32(size, 0xffff_ffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_c1f0, &args![0x9000u32, size]).u32(), 0x55);
        let file = calls(&e, FILE_CONSTRUCTOR)[0].clone();
        assert_eq!(file[1..], [0x9000, 0, 0, 0]);
        assert_eq!(calls(&e, FILE_GET_SIZE), vec![vec![file[0], size]]);
        assert_eq!(calls(&e, FILE_DESTRUCTOR), vec![vec![file[0]]]);
        e.register(POINTER_GET, |_, _| 3u32.into_ret());
        assert_eq!(e.call(0x0065_c1f0, &args![0x9000u32, size]).u32(), 3);
        assert_eq!((e.mem.u32(size), e.mem.u32(size + 4)), (0, 0));
    }
}
