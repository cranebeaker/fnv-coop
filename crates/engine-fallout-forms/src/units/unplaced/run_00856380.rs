//! `(unplaced)/run_00856380` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tessaveloadgame::TESSaveLoadGame;

/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The two-word list head constructor (`0096a2d0`, `thiscall`): clears both
/// words.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The scope guard's constructor (`00404eb0`, `thiscall(value, mode, source
/// file, line)`) and destructor (`00404ee0`).
const SCOPE_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// The first word the guard is given.
const SCOPE_GUARD_VALUE: u32 = 0x11;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESSaveLoadGame.cpp"`
/// and the line passed with it.
const SOURCE_FILE: u32 = 0x0108_04c8;
const SOURCE_LINE: u32 = 0x35f;
/// The game's `TESSaveLoadGame` pointer.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `ChangesMap`'s constructor (`00854ea0`).
const CHANGES_MAP_CONSTRUCT: u32 = 0x0085_4ea0;
/// `InteriorCellNewReferencesMap`'s constructor (`008552b0`).
const INTERIOR_MAP_CONSTRUCT: u32 = 0x0085_52b0;
/// `ExteriorCellNewReferencesMap`'s constructor (`008553e0`).
const EXTERIOR_MAP_CONSTRUCT: u32 = 0x0085_53e0;
/// `NumericIDBufferMap`'s constructor (`00855550`).
const NUMERIC_ID_MAP_CONSTRUCT: u32 = 0x0085_5550;
/// The constructor of the two id arrays (`008635a0`, `thiscall(first,
/// second)`; vtable `01081ef0`).
const ID_ARRAY_CONSTRUCT: u32 = 0x0086_35a0;
/// `SetAtGrow` of those arrays (`00864240`, `thiscall(index, element
/// pointer)`).
const ID_ARRAY_SET_AT_GROW: u32 = 0x0086_4240;
/// The game's `memset` wrapper (`00403d30`, `cdecl(dst, value, count)`).
const MEMSET: u32 = 0x0040_3d30;
/// Resets the save format version (`008620f0`, `thiscall`).
const RESET_VERSION: u32 = 0x0086_20f0;

/// Fields of `TESSaveLoadGame` (Xbox PDB names; the PC code uses the same
/// offsets) that the shared layout does not declare.
const SAVE_LOAD_STATES: u32 = 0x18; // m_iSaveLoadStates
const ALLOW_CHANGES: u32 = 0x1c; // m_bAllowChanges
const MOVED_REFERENCES_LIST: u32 = 0x24; // m_MovedReferencesList
const CREATED_BASE_OBJECTS_LIST: u32 = 0x2c; // m_CreatedBaseObjectsList
const DEFERRED_DELETE_LIST: u32 = 0x34; // m_DeferredDeleteList
const DEFERRED_INIT_ACTOR_PACKAGE_LIST: u32 = 0x3c; // m_DeferredInitActorPackageList
const NUMERIC_ID_ARRAY: u32 = 0x78; // m_pNumericIDArray
const WORLD_SPACE_ID_ARRAY: u32 = 0x7c; // m_pWorldSpaceIDArray
const CURRENTLY_LOADING_FORM_HEADER: u32 = 0x84; // m_pCurrentlyLoadingFormHeader
const RECONSTRUCTED_ALL_OBJECTS: u32 = 0xac; // m_bReconstructedAllObjects
const LOADING_FROM_MAIN_MENU: u32 = 0xad; // m_bLoadingFromMainMenu
const AUTOSAVE_DISABLED: u32 = 0xae; // m_bAutosaveDisabledDueToLackOfSpace
const QUICKLOADING: u32 = 0xaf; // m_bQuickloading
const INIT_ITEM_ARRAY: u32 = 0xb0; // m_pInitItemArray
const MOST_RECENT_SAVE_GAME: u32 = 0x1c4; // m_pMostRecentSaveGame

/// `operator new(size)`, then `construct(block, extra...)` when the block is
/// not null: the constructor's result (`EAX`), or null.
fn new_object(e: &mut Engine, size: u32, construct: u32, extra: &[u32]) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut words = vec![block];
    words.extend_from_slice(extra);
    e.call(construct, &words).u32()
}

// Translated from 00856380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00856380 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map; by its body the `TESSaveLoadGame` constructor
/// (the guard is given `TESSaveLoadGame.cpp` as its source file, it sets the
/// game's `TESSaveLoadGame` pointer `011de45c`, and `TES::TES` allocates
/// 0x1C8 bytes, `TESSaveLoadGame`'s size, for it). Clears the fields, builds
/// the five embedded two-word lists (`0096a2d0`), sets
/// `m_bUseNumericIDArray`, then under a scope guard (`00404eb0`: 0x11, 1,
/// the source file, line 0x35F) stores `this` at `011de45c` and allocates:
/// the `ChangesMap`, the interior map, two exterior maps, four
/// `NumericIDBufferMap`s, and the two id arrays (`008635a0` with 100, 50 and
/// with 5, 1), each followed by `SetAtGrow(0, &0)` (`00864240`). Each
/// allocation that fails leaves null. Then clears `m_OriginalSaveTime` (16
/// bytes, `00403d30`), resets the version (`008620f0`), copies the minor
/// version to the current one ([`fn_00856850`]) and ends the guard
/// (`00404ee0`). Returns `this`. `m_pNewChanges` (+4) is cleared and not
/// allocated.
pub fn fn_00856380(e: &mut Engine, this: Ptr<TESSaveLoadGame>) -> Ptr<TESSaveLoadGame> {
    let base = this.addr();
    e.set(this, TESSaveLoadGame::m_pNewChanges, Ptr::NULL);
    e.set(this, TESSaveLoadGame::m_pInteriorCellMap, Ptr::NULL);
    e.set(this, TESSaveLoadGame::m_pExteriorCellMap, Ptr::NULL);
    e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::NULL);
    e.mem.set_u32(base + SAVE_LOAD_STATES, 0);
    e.mem.set_u8(base + ALLOW_CHANGES, 0);
    e.set(this, TESSaveLoadGame::m_pInitArray, Ptr::NULL);
    e.call(LIST_CONSTRUCT, &args![base + MOVED_REFERENCES_LIST]);
    e.call(LIST_CONSTRUCT, &args![base + CREATED_BASE_OBJECTS_LIST]);
    e.call(LIST_CONSTRUCT, &args![base + DEFERRED_DELETE_LIST]);
    e.call(
        LIST_CONSTRUCT,
        &args![base + DEFERRED_INIT_ACTOR_PACKAGE_LIST],
    );
    e.set(this, TESSaveLoadGame::m_pSaveLoadStats, Ptr::NULL);
    e.set(this, TESSaveLoadGame::m_pFileIndexArray, Ptr::NULL);
    e.set(this, TESSaveLoadGame::m_iQueuedRemoveChanges, 0);
    e.call(
        LIST_CONSTRUCT,
        &args![base + TESSaveLoadGame::m_QueuedFormList.off],
    );
    e.set(this, TESSaveLoadGame::m_pSaveGameList, Ptr::NULL);
    e.set(this, TESSaveLoadGame::m_cMajorVersion, 0);
    e.set(this, TESSaveLoadGame::m_cMinorVersion, 0);
    e.set(this, TESSaveLoadGame::m_bUseNumericIDArray, true);
    e.mem.set_u32(base + CURRENTLY_LOADING_FORM_HEADER, 0);
    e.set(
        this,
        TESSaveLoadGame::m_pCurrentlySavingFormHeader,
        Ptr::NULL,
    );
    e.set(this, TESSaveLoadGame::m_iNextSaveNumber, 0);
    e.set(this, TESSaveLoadGame::m_iFileStartPosition, 0);
    e.set(this, TESSaveLoadGame::m_iSimulationFileSize, 0);
    e.set(this, TESSaveLoadGame::m_iOriginalSaveVersion, 0);
    e.mem.set_u8(base + RECONSTRUCTED_ALL_OBJECTS, 0);
    e.mem.set_u8(base + LOADING_FROM_MAIN_MENU, 0);
    e.mem.set_u8(base + AUTOSAVE_DISABLED, 0);
    e.mem.set_u8(base + QUICKLOADING, 0);
    e.mem.set_u32(base + INIT_ITEM_ARRAY, 0);
    e.mem.set_u32(base + MOST_RECENT_SAVE_GAME, 0);
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, SCOPE_GUARD_VALUE, 1u32, SOURCE_FILE, SOURCE_LINE],
        );
        e.set_global(SAVE_LOAD_GAME, base);
        let changes = new_object(e, 0x10, CHANGES_MAP_CONSTRUCT, &[]);
        e.set(this, TESSaveLoadGame::m_pChanges, Ptr::new(changes));
        let interior = new_object(e, 0x10, INTERIOR_MAP_CONSTRUCT, &[]);
        e.set(
            this,
            TESSaveLoadGame::m_pInteriorCellMap,
            Ptr::new(interior),
        );
        let exterior = new_object(e, 0x10, EXTERIOR_MAP_CONSTRUCT, &[]);
        e.set(
            this,
            TESSaveLoadGame::m_pExteriorCellMap,
            Ptr::new(exterior),
        );
        let created = new_object(e, 0x10, EXTERIOR_MAP_CONSTRUCT, &[]);
        e.set(
            this,
            TESSaveLoadGame::m_pCreatedExteriorCells,
            Ptr::new(created),
        );
        let animation = new_object(e, 0x10, NUMERIC_ID_MAP_CONSTRUCT, &[]);
        e.set(this, TESSaveLoadGame::m_pAnimationMap, Ptr::new(animation));
        let attached = new_object(e, 0x10, NUMERIC_ID_MAP_CONSTRUCT, &[]);
        e.set(
            this,
            TESSaveLoadGame::m_pAttachedAnimationMap,
            Ptr::new(attached),
        );
        let controller = new_object(e, 0x10, NUMERIC_ID_MAP_CONSTRUCT, &[]);
        e.set(
            this,
            TESSaveLoadGame::m_pCharControllerMap,
            Ptr::new(controller),
        );
        let havok = new_object(e, 0x10, NUMERIC_ID_MAP_CONSTRUCT, &[]);
        e.set(this, TESSaveLoadGame::m_pHavokDataMap, Ptr::new(havok));
        let numeric = new_object(e, 0x18, ID_ARRAY_CONSTRUCT, &[100, 0x32]);
        e.mem.set_u32(base + NUMERIC_ID_ARRAY, numeric);
        e.with_stack(4, |e, zero| {
            e.mem.set_u32(zero.addr(), 0);
            let array = e.mem.u32(base + NUMERIC_ID_ARRAY);
            e.call(ID_ARRAY_SET_AT_GROW, &args![array, 0u32, zero]);
        });
        let world_space = new_object(e, 0x18, ID_ARRAY_CONSTRUCT, &[5, 1]);
        e.mem.set_u32(base + WORLD_SPACE_ID_ARRAY, world_space);
        e.with_stack(4, |e, zero| {
            e.mem.set_u32(zero.addr(), 0);
            let array = e.mem.u32(base + WORLD_SPACE_ID_ARRAY);
            e.call(ID_ARRAY_SET_AT_GROW, &args![array, 0u32, zero]);
        });
        e.call(
            MEMSET,
            &args![
                base + TESSaveLoadGame::m_OriginalSaveTime.off,
                0u32,
                0x10u32
            ],
        );
        e.call(RESET_VERSION, &args![this]);
        fn_00856850(e, this);
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
    });
    this
}

// Translated from 00856850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies `m_cMinorVersion` (+0x75) into
/// `m_cCurrentVersion` (+0x80).
pub fn fn_00856850(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let minor = e.get(this, TESSaveLoadGame::m_cMinorVersion);
    e.set(this, TESSaveLoadGame::m_cCurrentVersion, minor);
}

/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Called on each id array before it is deleted (`00863db0`, `thiscall`).
const ID_ARRAY_RELEASE: u32 = 0x0086_3db0;
/// The +0x70 list's "is empty" test (`008256d0`, `thiscall`, `bool`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Returns the address of the +0x70 list's first element (`006815c0`,
/// `thiscall`).
const LIST_FIRST: u32 = 0x0068_15c0;
/// Removes an element from the +0x70 list (`00905330`, `thiscall(element
/// pointer)`).
const LIST_REMOVE: u32 = 0x0090_5330;
/// The +0x70 list's scalar deleting destructor (`004702f0`,
/// `thiscall(flags)`).
const LIST_DELETE: u32 = 0x0047_02f0;
/// Removes every node of an embedded two-word list (`00470470`,
/// `thiscall`).
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// The embedded two-word list's destructor (`0046ffb0`, `thiscall`).
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `TESSaveLoadGame` pointers the destructor deletes that are not named
/// above: the four maps the constructor makes at +0x00..+0x10, the buffer at
/// +0x50, the four numeric-id maps at +0x58..+0x64 and the list at +0x70.
const DELETED_FIRST: [u32; 4] = [0x00, 0x08, 0x0c, 0x10];
const BUFFER: u32 = 0x50;
const DELETED_SECOND: [u32; 4] = [0x58, 0x5c, 0x60, 0x64];
const OWNED_LIST: u32 = 0x70;
/// The embedded list at +0x68.
const EMBEDDED_LIST_68: u32 = 0x68;

/// `if (p) p->~T(1)`: the scalar deleting destructor in vtable slot 0.
fn delete_virtual(e: &mut Engine, object: u32) {
    if object != 0 {
        e.vcall(object, 0x0, &args![1u32]);
    }
}

// Translated from 00856870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00856870 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map; by its body the `TESSaveLoadGame` destructor
/// (it releases what `00856380` builds): deletes the four maps at
/// +0x00/+0x08/+0x0C/+0x10 through their virtual destructors, frees the
/// buffer at +0x50, deletes the four numeric-id maps, releases (`00863db0`)
/// and deletes the two id arrays (+0x78, +0x7C), frees
/// `m_pMostRecentSaveGame` (+0x1C4), empties the list at +0x70 (deleting
/// each element) and deletes it (`004702f0(1)`), then empties the embedded
/// lists at +0x24 and +0x2C and destroys the five embedded lists in reverse
/// order (+0x68, +0x3C, +0x34, +0x2C, +0x24).
pub fn fn_00856870(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let base = this.addr();
    for offset in DELETED_FIRST {
        let object = e.mem.u32(base + offset);
        delete_virtual(e, object);
    }
    let buffer = e.mem.u32(base + BUFFER);
    if buffer != 0 {
        e.call(OPERATOR_DELETE, &args![buffer]);
    }
    for offset in DELETED_SECOND {
        let object = e.mem.u32(base + offset);
        delete_virtual(e, object);
    }
    for offset in [NUMERIC_ID_ARRAY, WORLD_SPACE_ID_ARRAY] {
        let array = e.mem.u32(base + offset);
        if array != 0 {
            e.call(ID_ARRAY_RELEASE, &args![array]);
            let array = e.mem.u32(base + offset);
            delete_virtual(e, array);
        }
    }
    let most_recent = e.mem.u32(base + MOST_RECENT_SAVE_GAME);
    if most_recent != 0 {
        e.call(OPERATOR_DELETE, &args![most_recent]);
    }
    if e.mem.u32(base + OWNED_LIST) != 0 {
        loop {
            let list = e.mem.u32(base + OWNED_LIST);
            if e.call(LIST_IS_EMPTY, &args![list]).u8() != 0 {
                break;
            }
            let list = e.mem.u32(base + OWNED_LIST);
            let first = e.call(LIST_FIRST, &args![list]).u32();
            let element = e.mem.u32(first);
            let element = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), element);
                let list = e.mem.u32(base + OWNED_LIST);
                e.call(LIST_REMOVE, &args![list, slot]);
                e.mem.u32(slot.addr())
            });
            delete_virtual(e, element);
        }
        let list = e.mem.u32(base + OWNED_LIST);
        if list != 0 {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
    }
    e.call(LIST_REMOVE_ALL, &args![base + MOVED_REFERENCES_LIST]);
    e.call(LIST_REMOVE_ALL, &args![base + CREATED_BASE_OBJECTS_LIST]);
    for offset in [
        EMBEDDED_LIST_68,
        DEFERRED_INIT_ACTOR_PACKAGE_LIST,
        DEFERRED_DELETE_LIST,
        CREATED_BASE_OBJECTS_LIST,
        MOVED_REFERENCES_LIST,
    ] {
        e.call(LIST_DESTRUCT, &args![base + offset]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00856380,
            fn_00856380(Ptr<TESSaveLoadGame>) -> Ptr<TESSaveLoadGame>
        ),
        entry!(0x00856850, fn_00856850(Ptr<TESSaveLoadGame>)),
        entry!(0x0085_6870, fn_00856870(Ptr<TESSaveLoadGame>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = [(u32, Vec<u32>)];
    type Seen = Rc<RefCell<Vec<u32>>>;

    const CONSTRUCTORS: [u32; 5] = [
        CHANGES_MAP_CONSTRUCT,
        INTERIOR_MAP_CONSTRUCT,
        EXTERIOR_MAP_CONSTRUCT,
        NUMERIC_ID_MAP_CONSTRUCT,
        ID_ARRAY_CONSTRUCT,
    ];

    /// Every callee doubled: the constructors return their block plus 1 (so
    /// the stored value is visibly the constructor's result), `operator new`
    /// fails for the sizes in `failing`, and the `SetAtGrow` double records
    /// the word its element pointer points at.
    fn engine(failing: &'static [u32]) -> (Engine, Seen) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        for addr in [
            LIST_CONSTRUCT,
            SCOPE_GUARD_CONSTRUCT,
            SCOPE_GUARD_DESTRUCT,
            MEMSET,
            RESET_VERSION,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        for addr in CONSTRUCTORS {
            e.register(addr, |_, a| (a[0] + 1).into_ret());
        }
        e.register_double(OPERATOR_NEW, move |e, a| {
            if failing.contains(&a[0]) {
                0u32.into_ret()
            } else {
                e.mem.alloc(a[0]).into_ret()
            }
        });
        let seen: Seen = Rc::default();
        let record = seen.clone();
        e.register_double(ID_ARRAY_SET_AT_GROW, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[2]));
            a[1].into_ret()
        });
        (e, seen)
    }

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_clears_the_fields_and_builds_the_maps() {
        let (mut e, seen) = engine(&[]);
        let game: Ptr<TESSaveLoadGame> = e.new_object();
        let base = game.addr();
        for offset in (0..0x1c8).step_by(4) {
            e.mem.set_u32(base + offset, 0xaaaa_aaaa);
        }
        e.call_log = Some(vec![]);
        let back = e.call(0x0085_6380, &args![game]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, base);
        assert_eq!(e.global::<u32>(SAVE_LOAD_GAME), base);
        for offset in [
            0x04, 0x14, 0x18, 0x20, 0x44, 0x50, 0x54, 0x70, 0x84, 0x88, 0x8c, 0x90, 0x94, 0xa8,
            0xac, 0xb0, 0x1c4,
        ] {
            assert_eq!(e.mem.u32(base + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u8(base + 0x1c), 0);
        assert_eq!(e.mem.u8(base + 0x74), 0);
        assert_eq!(e.mem.u8(base + 0x75), 0);
        assert_eq!(e.mem.u8(base + 0x81), 1);
        // fn_00856850 copied the minor version (left 0 by the doubled reset).
        assert_eq!(e.mem.u8(base + 0x80), 0);
        assert_eq!(
            calls(&log, LIST_CONSTRUCT),
            vec![
                vec![base + 0x24],
                vec![base + 0x2c],
                vec![base + 0x34],
                vec![base + 0x3c],
                vec![base + 0x68],
            ]
        );
        assert_eq!(
            calls(&log, SCOPE_GUARD_CONSTRUCT)[0][1..],
            [0x11, 1, SOURCE_FILE, 0x35f]
        );
        let guard = calls(&log, SCOPE_GUARD_CONSTRUCT)[0][0];
        assert_eq!(calls(&log, SCOPE_GUARD_DESTRUCT), vec![vec![guard]]);
        let sizes: Vec<u32> = calls(&log, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        assert_eq!(
            sizes,
            [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x18, 0x18]
        );
        // Each pointer is its constructor's result (block + 1).
        let constructed = |e: &Engine, offset: u32, construct: u32, nth: usize| {
            let block = calls(&log, construct)[nth][0];
            assert_eq!(e.mem.u32(base + offset), block + 1, "+{offset:#x}");
        };
        constructed(&e, 0x00, CHANGES_MAP_CONSTRUCT, 0);
        constructed(&e, 0x08, INTERIOR_MAP_CONSTRUCT, 0);
        constructed(&e, 0x0c, EXTERIOR_MAP_CONSTRUCT, 0);
        constructed(&e, 0x10, EXTERIOR_MAP_CONSTRUCT, 1);
        for (nth, offset) in [0x58, 0x5c, 0x60, 0x64].into_iter().enumerate() {
            constructed(&e, offset, NUMERIC_ID_MAP_CONSTRUCT, nth);
        }
        constructed(&e, 0x78, ID_ARRAY_CONSTRUCT, 0);
        constructed(&e, 0x7c, ID_ARRAY_CONSTRUCT, 1);
        let arrays = calls(&log, ID_ARRAY_CONSTRUCT);
        assert_eq!(&arrays[0][1..], &[100, 0x32]);
        assert_eq!(&arrays[1][1..], &[5, 1]);
        let grows = calls(&log, ID_ARRAY_SET_AT_GROW);
        assert_eq!(grows.len(), 2);
        assert_eq!(&grows[0][..2], &[e.mem.u32(base + 0x78), 0]);
        assert_eq!(&grows[1][..2], &[e.mem.u32(base + 0x7c), 0]);
        assert_eq!(*seen.borrow(), vec![0, 0]);
        assert_eq!(calls(&log, MEMSET), vec![vec![base + 0x98, 0, 0x10]]);
        assert_eq!(calls(&log, RESET_VERSION), vec![vec![base]]);
    }

    #[test]
    fn constructor_leaves_null_where_an_allocation_fails() {
        let (mut e, _) = engine(&[0x10, 0x18]);
        let game: Ptr<TESSaveLoadGame> = e.new_object();
        let base = game.addr();
        for offset in [0x00, 0x08, 0x0c, 0x10, 0x58, 0x5c, 0x60, 0x64, 0x78, 0x7c] {
            e.mem.set_u32(base + offset, 0xaaaa_aaaa);
        }
        e.call_log = Some(vec![]);
        e.call(0x0085_6380, &args![game]);
        let log = e.call_log.take().unwrap();
        for construct in CONSTRUCTORS {
            assert!(calls(&log, construct).is_empty());
        }
        for offset in [0x00, 0x08, 0x0c, 0x10, 0x58, 0x5c, 0x60, 0x64, 0x78, 0x7c] {
            assert_eq!(e.mem.u32(base + offset), 0, "+{offset:#x}");
        }
        // SetAtGrow still runs, on the null array pointers.
        let grows = calls(&log, ID_ARRAY_SET_AT_GROW);
        assert_eq!(grows.len(), 2);
        assert_eq!(grows[0][0], 0);
        assert_eq!(grows[1][0], 0);
    }

    #[test]
    fn copies_the_minor_version_to_the_current_one() {
        let mut e = Engine::new();
        let game: Ptr<TESSaveLoadGame> = e.new_object();
        e.set(game, TESSaveLoadGame::m_cMinorVersion, 0x7d);
        e.set(game, TESSaveLoadGame::m_cCurrentVersion, 3);
        e.call(0x0085_6850, &args![game]);
        assert_eq!(e.get(game, TESSaveLoadGame::m_cCurrentVersion), 0x7d);
        assert_eq!(e.get(game, TESSaveLoadGame::m_cMinorVersion), 0x7d);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DELETING_DESTRUCTOR: u32 = 0x0077_0000;
    const VTABLE: u32 = 0x0077_1000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn object(e: &mut Engine) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_u32(p, VTABLE);
        p
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.put_vtable(VTABLE, &[DELETING_DESTRUCTOR]);
        for addr in [
            DELETING_DESTRUCTOR,
            OPERATOR_DELETE,
            ID_ARRAY_RELEASE,
            LIST_DELETE,
            LIST_REMOVE_ALL,
            LIST_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn destructor_with_everything_null_only_tears_down_the_lists() {
        let mut e = engine();
        let game: Ptr<TESSaveLoadGame> = e.new_object();
        let base = game.addr();
        e.call_log = Some(vec![]);
        e.call(0x0085_6870, &args![game]);
        assert!(calls(&e, DELETING_DESTRUCTOR).is_empty());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert!(calls(&e, ID_ARRAY_RELEASE).is_empty());
        assert!(calls(&e, LIST_DELETE).is_empty());
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL),
            vec![vec![base + 0x24], vec![base + 0x2c]]
        );
        assert_eq!(
            calls(&e, LIST_DESTRUCT),
            vec![
                vec![base + 0x68],
                vec![base + 0x3c],
                vec![base + 0x34],
                vec![base + 0x2c],
                vec![base + 0x24],
            ]
        );
    }

    #[test]
    fn destructor_deletes_every_owned_object_and_drains_the_list() {
        let mut e = engine();
        let game: Ptr<TESSaveLoadGame> = e.new_object();
        let base = game.addr();
        let mut owned = vec![];
        for offset in [0x00, 0x08, 0x0c, 0x10, 0x58, 0x5c, 0x60, 0x64, 0x78, 0x7c] {
            let p = object(&mut e);
            e.mem.set_u32(base + offset, p);
            owned.push(p);
        }
        e.mem.set_u32(base + 0x50, 0x4440);
        e.mem.set_u32(base + 0x1c4, 0x4450);
        // The list at +0x70 holds two elements, then a null one.
        let list = e.mem.alloc(8);
        e.mem.set_u32(base + 0x70, list);
        let first = object(&mut e);
        let second = object(&mut e);
        let items: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![first, second, 0]));
        let cell = e.mem.alloc(4);
        let left = items.clone();
        e.register_double(LIST_IS_EMPTY, move |_, _| {
            u32::from(left.borrow().is_empty()).into_ret()
        });
        let head = items.clone();
        e.register_double(LIST_FIRST, move |e, _| {
            e.mem.set_u32(cell, head.borrow()[0]);
            cell.into_ret()
        });
        let removed = items;
        e.register_double(LIST_REMOVE, move |_, _| {
            removed.borrow_mut().remove(0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0085_6870, &args![game]);
        let deleted: Vec<u32> = calls(&e, DELETING_DESTRUCTOR)
            .iter()
            .map(|w| {
                assert_eq!(w[1], 1);
                w[0]
            })
            .collect();
        let mut expected = owned.clone();
        expected.extend([first, second]);
        assert_eq!(deleted, expected);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x4440], vec![0x4450]]);
        assert_eq!(
            calls(&e, ID_ARRAY_RELEASE),
            vec![vec![owned[8]], vec![owned[9]]]
        );
        assert_eq!(calls(&e, LIST_REMOVE).len(), 3);
        assert_eq!(calls(&e, LIST_DELETE), vec![vec![list, 1]]);
    }
}
