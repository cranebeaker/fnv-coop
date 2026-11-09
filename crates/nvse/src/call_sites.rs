//! The events xNVSE raises at its own call sites (ADR-0016 build step I-9, part 2): translated
//! from xNVSE 6.4.9's `EventManager.cpp` (OnActivate, OnActorEquip and its AddWornItem flag,
//! OnSell), `Hooks_Gameplay.cpp` (OnDrop and OnDropItem of a dropped item) and
//! `Hooks_Other.cpp` (image-space modifiers, locks, terminals, enable and disable, 3D, cells),
//! each at the game's own instruction (engine-core's seam table) or, for the cell's load state,
//! over its setter (the wrap table).
//!
//! Every handler first reads the lock-free copy of which events have handlers
//! ([`events::LockFree`]): these sites run on every thread (3D and cell loading on the loading
//! threads), so with no extender script none of them takes the extender's lock. That check
//! changes nothing else: with no handler xNVSE's dispatch does nothing either.
//!
//! Not here: OnRepair (its first argument is an inventory reference, I-12), OnUnlock (xNVSE
//! registers it, but its three hooks are commented out, so it is never raised), the native
//! plugins' messages of these moments (no native plugins yet), and xNVSE's null check in
//! `MagicTarget::CheckAddEffect` (`008232B5`), a fix of the game's that belongs to a fixes row.

use crate::events::{self, id};
use engine_core::extend::{Owner, SiteSlot, WrapSlot};
use engine_core::prelude::*;

/// `TESObjectREFR::Activate`'s first instruction (`kActivate_HookAddr`).
pub const ACTIVATE: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0057_318e);
/// The equip of `004bffe0` (`kOnActorEquipHookAddr`).
pub const ACTOR_EQUIP: SiteSlot = SiteSlot::new(Owner::Nvse, 0x004c_0032);
/// `Actor::EquipObject`'s three calls to `Actor::AddWornItem` (`InstallAddWornItemHook`).
pub const ADD_WORN_ITEM: [SiteSlot; 3] = [
    SiteSlot::new(Owner::Nvse, 0x0088_caeb),
    SiteSlot::new(Owner::Nvse, 0x0088_cb97),
    SiteSlot::new(Owner::Nvse, 0x0088_cd11),
];
/// `BarterMenu::CompleteTransaction`'s count reads: an item the vendor sells, one the player
/// sells (`OnSell::WriteDelayedHooks`).
pub const SELL_BY_VENDOR: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0072_fe3e);
pub const SELL_BY_PLAYER: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0072_ff20);
/// The dropped reference made in `005750a0` (`kCreateDroppedReferenceHookAddr`).
pub const DROPPED_ITEM: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0057_515a);
/// `IMod::WriteHooks`.
pub const APPLY_IMOD: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0052_9a37);
pub const REMOVE_IMOD: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0052_9d37);
pub const IMOD_RAN_OUT: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0086_fe75);
/// `Locks::WriteHooks`.
pub const LOCK_BROKEN: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0079_0461);
pub const LOCK_PICK_SUCCESS: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0078_f8e5);
pub const LOCK_PICK_BROKEN: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0078_fe24);
/// `Terminal::WriteHooks`.
pub const TERMINAL_HACKED: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0076_717a);
pub const TERMINAL_HACK_FAILED: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0076_74b0);
/// `DisEnable::WriteHooks`.
pub const DISABLE: SiteSlot = SiteSlot::new(Owner::Nvse, 0x005c_47b6);
pub const ENABLE: SiteSlot = SiteSlot::new(Owner::Nvse, 0x005c_453c);
/// `RefLoading::WriteHooks`.
pub const REF_SET_3D: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0057_1090);
pub const REF_UNSET_3D: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0057_10ac);
pub const REF_ATTACH: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0054_9787);
/// `CellLoading::WriteHooks`: the cell's load state setter (replaced in xNVSE, wrapped here: the
/// setter is the store xNVSE does itself) and the per-frame update's first call.
pub const CELL_SET_STATE: WrapSlot = WrapSlot::new(Owner::Nvse, 0x0045_12a0);
pub const CELL_REFS_LOADED: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0055_18a5);
/// `FormExtraData`'s removal hooks (`Hooks::InitHooks`): a form destroyed, a reference trashed.
pub const FORM_DESTROYED: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0048_3669);
pub const REFERENCE_TRASHED: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0086_80a4);

/// `Actor::AddWornItem` (`thiscall`, five stack words).
const ADD_WORN_ITEM_FN: u32 = 0x0088_db20;
/// `BGSSaveLoadGame::GetSingleton` (`0x11DDF38`) and its `globalFlags` (`+0x244`): bit 2 while a
/// save loads.
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
const SAVE_LOAD_FLAGS: u32 = 0x244;
/// `TESForm::flags` (`+8`): deleted, temporary and `0x200000` (`CanDispatchScriptEvent`).
const FORM_FLAGS: u32 = 8;
const NO_SCRIPT_EVENT_FLAGS: u32 = 0x0020_4020;
/// `TESForm::GetIsReference` (virtual `0x3C`).
const GET_IS_REFERENCE: u32 = 0xf0;
/// `ExtraContainerChanges::EntryData::type` (`+8`).
const ENTRY_TYPE: u32 = 8;
/// `kEvent_OnSell`.
const MASK_ON_SELL: u32 = 0x4000;
/// `HackingMenu::pTerminalRef` (`+0x198`).
const TERMINAL_REF: u32 = 0x198;
/// The image-space modifier instance's form (`+0x1C`), as xNVSE reads it; `AsForm` (virtual
/// `0x94`): the instance as a modifier form's instance, null for the others; `TESForm::refID`.
const INSTANCE_FORM: u32 = 0x1c;
const AS_FORM: u32 = 0x94;
const FORM_ID: u32 = 0xc;
/// `kFormType_TESObjectCELL`: the form removal hooks look at cells only.
const FORM_TYPE_CELL: u8 = 0x39;
/// `TESObjectREFR::renderState` (`+0x64`).
const RENDER_STATE: u32 = 0x64;
/// `TESObjectCELL`: `cCellState` (`+0x26`), `iCriticalQueuedRefCount` (`+0xA0`),
/// `iQueuedRefCount` (`+0xA4`), `pLoadedData` (`+0xC4`).
const CELL_STATE: u32 = 0x26;
const CELL_CRITICAL_QUEUED: u32 = 0xa0;
const CELL_QUEUED: u32 = 0xa4;
const CELL_LOADED_DATA: u32 = 0xc4;
/// `TESObjectCELL::CellLoadState`.
const CELL_DETACHING: u32 = 4;
const CELL_ATTACHED: u32 = 6;

/// The cells whose references all loaded (xNVSE's `NVSECellLoadData::bAllRefsLoaded`, a form
/// extra data): OnCellRefsLoaded comes once per cell, until the cell is destroyed. Behind its own
/// lock, as xNVSE's form extra data map is; `count` lets the form removal hooks skip it without
/// the lock while it is empty.
#[derive(Default)]
pub struct CellLoads {
    cells: std::sync::Mutex<std::collections::HashSet<u32>>,
    count: std::sync::atomic::AtomicUsize,
}

impl CellLoads {
    /// Marks `cell`; true the first time.
    fn mark(&self, cell: u32) -> bool {
        let mut cells = self.cells.lock().unwrap_or_else(|p| p.into_inner());
        let first = cells.insert(cell);
        self.count
            .store(cells.len(), std::sync::atomic::Ordering::Relaxed);
        first
    }

    /// `RemoveForForm`: what was kept for `form` goes.
    fn forget(&self, form: u32) {
        if self.count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
            return;
        }
        let mut cells = self.cells.lock().unwrap_or_else(|p| p.into_inner());
        if cells.remove(&form) {
            self.count
                .store(cells.len(), std::sync::atomic::Ordering::Relaxed);
        }
    }

    /// Whether `cell` is marked (tests).
    pub fn has(&self, cell: u32) -> bool {
        self.cells
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .contains(&cell)
    }
}

// ---- EventManager.cpp ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// TESObjectREFR_ActivateHook (installed with an OnActivate handler, `s_ActivateHook`).
/// OnActivate: the reference (`this`) activated by `activator`.
fn activate(e: &mut Engine, a: &[u32]) -> Option<u32> {
    if events::marks_lock_free(e) & events::MASK_ON_ACTIVATE != 0
        && events::activate_hook_installed(e)
        && events::in_use(e) & events::MASK_ON_ACTIVATE != 0
    {
        events::handle_game_event(e, events::MASK_ON_ACTIVATE, a[0], a[1]);
    }
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: OnActorEquipHook
// (installed with an OnActorEquip handler, `s_ActorEquipHook`).
/// OnActorEquip: the actor and the item, whatever the main hook saw.
fn actor_equip(e: &mut Engine, a: &[u32]) -> Option<u32> {
    if events::marks_lock_free(e) & 2 != 0
        && events::equip_hook_installed(e)
        && events::in_use(e) & 2 != 0
    {
        events::handle_game_event(e, 2, a[0], a[1]);
    }
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: Actor__AddWornItem.
/// Runs `Actor::AddWornItem` with `g_inActivateItemInInventory` set, so the main hook's
/// OnActorEquip mark during it is not raised again (the equip hook raised it); takes the call.
fn add_worn_item(e: &mut Engine, a: &[u32]) -> Option<u32> {
    /// Clears the flag however the call ends (a stop unwinding through it too), so a caught
    /// stop does not leave this thread's later equip events dropped.
    struct Clear;
    impl Drop for Clear {
        fn drop(&mut self) {
            events::set_in_add_worn_item(false);
        }
    }
    events::set_in_add_worn_item(true);
    let _clear = Clear;
    let worn = e
        .call(ADD_WORN_ITEM_FN, &args![a[0], a[1], a[2], a[3], a[4]])
        .u32()
        & 0xff;
    Some(worn)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: OnSell::OnSellHook.
/// OnSell: the entry's item (a base form) and the seller, when the seller is a reference. The
/// count read goes on as the game's. (xNVSE also keeps the entry for `GetSoldItemInvRef`, an
/// inventory reference: I-12.)
fn sell(e: &mut Engine, a: &[u32]) -> Option<u32> {
    if !events::has_handlers_lock_free(e, id::ON_SELL) || events::in_use(e) & MASK_ON_SELL == 0 {
        return None;
    }
    let (entry, seller) = (a[0], a[1]);
    let item = e.mem.u32(entry + ENTRY_TYPE);
    if seller != 0 && e.vcall(seller, GET_IS_REFERENCE, &[]).bool() && item != 0 {
        events::handle_event(e, id::ON_SELL, item, seller, None);
    }
    None
}

// ---- Hooks_Gameplay.cpp ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Gameplay.cpp: HandleDroppedItem.
/// OnDrop (the dropper, the reference made) and OnDropItem (the dropper, the item).
fn dropped_item(e: &mut Engine, a: &[u32]) -> Option<u32> {
    let (dropper, item, dropped) = (a[0], a[1], a[2]);
    if dropped != 0 && events::has_handlers_lock_free(e, id::ON_DROP) {
        events::handle_event(e, id::ON_DROP, dropper, dropped, None);
    }
    if item != 0 && events::has_handlers_lock_free(e, id::ON_DROP_ITEM) {
        events::handle_event(e, id::ON_DROP_ITEM, dropper, item, None);
    }
    None
}

// ---- Hooks_Other.cpp ----

/// `EventManager::DispatchEvent(name, nullptr, form)` when the event has a handler.
fn dispatch_if_handled(e: &mut Engine, at: usize, name: &[u8], form: u32) {
    if events::has_handlers_lock_free(e, at) {
        events::dispatch(e, name, 0, &[form]);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp: CanDispatchScriptEvent.
/// Not while a save loads, and not for a deleted or temporary form.
fn can_dispatch_script_event(e: &mut Engine, form: u32) -> bool {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if save_load != 0 && e.mem.u32(save_load + SAVE_LOAD_FLAGS) & 2 != 0 {
        return false;
    }
    e.mem.u32(form + FORM_FLAGS) & NO_SCRIPT_EVENT_FLAGS == 0
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp: DispatchEventSafe.
/// `DispatchEventThreadSafe(name, nullptr, nullptr, form)` when the form may have script events.
fn dispatch_safe(e: &mut Engine, at: usize, name: &[u8], form: u32) {
    if events::has_handlers_lock_free(e, at) && can_dispatch_script_event(e, form) {
        events::dispatch_thread_safe(e, name, 0, &[form]);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp: IMod::ApplyIMOD,
// RemoveIMOD_1, RemoveIMOD_2.
fn apply_imod(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_if_handled(e, id::ON_APPLY_IMOD, b"onapplyimod", a[0]);
    None
}

fn remove_imod(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_if_handled(e, id::ON_REMOVE_IMOD, b"onremoveimod", a[0]);
    None
}

/// An instance that ran out. xNVSE reads its form at `+0x1C` whatever the instance is; a
/// depth-of-field or radial blur instance (iron sights, V.A.T.S., `SetGlobalRadialBlur`) has a
/// float there, and a handler reading that as a form would stop the game. With the fixes
/// preset's row ([`set_imod_form_check`]) only an instance that is a modifier form's
/// (`AsForm`, virtual `0x94`, as the game's own `Stop` asks) raises the event, with that form;
/// without it the word is read as xNVSE reads it, and a word that is no readable form drops
/// the event (ordinary play never stops).
fn imod_ran_out(e: &mut Engine, a: &[u32]) -> Option<u32> {
    if !events::has_handlers_lock_free(e, id::ON_REMOVE_IMOD) {
        return None;
    }
    let instance = a[0];
    let form = if e.ext::<crate::Nvse>().events.imod_form_check {
        let as_form = e.vcall(instance, AS_FORM, &[]).u32();
        if as_form == 0 {
            return None;
        }
        e.mem.u32(as_form + INSTANCE_FORM)
    } else {
        let form = e.mem.u32(instance + INSTANCE_FORM);
        let mut id = [0u8; 4];
        if form != 0 && e.mem.try_read(form + FORM_ID, &mut id).is_err() {
            return None;
        }
        form
    };
    events::dispatch(e, b"onremoveimod", 0, &[form]);
    None
}

/// The fixes preset's row (docs/FIXES.md, "The script extender"): OnRemoveIMOD only for a
/// modifier form's instance that ran out.
pub fn set_imod_form_check(e: &mut Engine, on: bool) {
    e.ext::<crate::Nvse>().events.imod_form_check = on;
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp: Locks::OnLockBroken,
// OnLockPickSuccess, OnLockPickBroken.
fn lock_broken(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_if_handled(e, id::ON_LOCK_BROKEN, b"onlockbroken", a[0]);
    None
}

fn lock_pick_success(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_if_handled(e, id::ON_LOCK_PICK_SUCCESS, b"onlockpicksuccess", a[0]);
    None
}

fn lock_pick_broken(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_if_handled(e, id::ON_LOCK_PICK_BROKEN, b"onlockpickbroken", a[0]);
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp:
// Terminal::OnTerminalHacked, OnTerminalHackFailed (the menu's terminal reference).
fn terminal_hacked(e: &mut Engine, a: &[u32]) -> Option<u32> {
    if events::has_handlers_lock_free(e, id::ON_TERMINAL_HACKED) {
        let terminal = e.mem.u32(a[0] + TERMINAL_REF);
        events::dispatch(e, b"onterminalhacked", 0, &[terminal]);
    }
    None
}

fn terminal_hack_failed(e: &mut Engine, a: &[u32]) -> Option<u32> {
    if events::has_handlers_lock_free(e, id::ON_TERMINAL_HACK_FAILED) {
        let terminal = e.mem.u32(a[0] + TERMINAL_REF);
        events::dispatch(e, b"onterminalhackfailed", 0, &[terminal]);
    }
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp: DisEnable::OnDisable,
// OnEnable.
fn disable(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_if_handled(e, id::ON_DISABLE, b"ondisable", a[0]);
    None
}

fn enable(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_if_handled(e, id::ON_ENABLE, b"onenable", a[0]);
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp:
// RefLoading::OnRefSet3DHook (after the loaded data is made), OnRefUnset3DHook (before it goes,
// when there is one), OnRefAttachHook.
fn ref_set_3d(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_safe(e, id::ON_REF_SET_3D, b"onrefset3d", a[0]);
    None
}

fn ref_unset_3d(e: &mut Engine, a: &[u32]) -> Option<u32> {
    if events::has_handlers_lock_free(e, id::ON_REF_UNSET_3D) && e.mem.u32(a[0] + RENDER_STATE) != 0
    {
        dispatch_safe(e, id::ON_REF_UNSET_3D, b"onrefunset3d", a[0]);
    }
    None
}

fn ref_attach(e: &mut Engine, a: &[u32]) -> Option<u32> {
    dispatch_safe(e, id::ON_REF_ATTACH, b"onrefattach", a[0]);
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp:
// CellLoading::OnCellStateChange.
/// The setter stores the state (the next link); then, for a cell that may have script events,
/// OnCellDetach for "detaching" and OnCellAttach for "attached".
fn cell_set_state(e: &mut Engine, a: &[u32]) -> Ret {
    let ret = e.call_next(CELL_SET_STATE.addr(), a);
    let (cell, state) = (a[0], a[1] & 0xff);
    let (at, name): (usize, &[u8]) = match state {
        CELL_DETACHING => (id::ON_CELL_DETACH, &b"oncelldetach"[..]),
        CELL_ATTACHED => (id::ON_CELL_ATTACH, &b"oncellattach"[..]),
        _ => return ret,
    };
    if events::has_handlers_lock_free(e, at) && can_dispatch_script_event(e, cell) {
        events::dispatch(e, name, 0, &[cell]);
    }
    ret
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp:
// CellLoading::OnAllRefsLoadedHook.
/// An attached cell with its loaded data and no reference queued: marked once (whether or not a
/// handler is set, as xNVSE's form extra data is), and OnCellRefsLoaded the first time.
fn cell_refs_loaded(e: &mut Engine, a: &[u32]) -> Option<u32> {
    let cell = a[0];
    if u32::from(e.mem.u8(cell + CELL_STATE)) == CELL_ATTACHED
        && e.mem.u32(cell + CELL_LOADED_DATA) != 0
        && e.mem.u32(cell + CELL_CRITICAL_QUEUED) == 0
        && e.mem.u32(cell + CELL_QUEUED) == 0
        && e.ext_free::<CellLoads>().mark(cell)
    {
        dispatch_safe(e, id::ON_CELL_REFS_LOADED, b"oncellrefsloaded", cell);
    }
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FormExtraData.cpp:
// RemoveFromAllFormsMapHook (kFormDeletion, kTrashedReference).
fn form_gone(e: &mut Engine, a: &[u32]) -> Option<u32> {
    // Only cells are kept here: any other form goes by without the set's lock.
    if e.mem.u8(a[0] + 4) == FORM_TYPE_CELL {
        e.ext_free::<CellLoads>().forget(a[0]);
    }
    None
}

/// Installs the call sites' handlers and the cell state wrap.
pub fn install(e: &mut Engine) {
    // Made before the threads start, so every thread's engine shares it.
    e.ext_free::<CellLoads>();
    e.wrap(CELL_SET_STATE, cell_set_state);
    e.on_site(FORM_DESTROYED, form_gone);
    e.on_site(ACTOR_EQUIP, actor_equip);
    e.on_site(APPLY_IMOD, apply_imod);
    e.on_site(REMOVE_IMOD, remove_imod);
    e.on_site(REF_ATTACH, ref_attach);
    e.on_site(CELL_REFS_LOADED, cell_refs_loaded);
    e.on_site(REF_SET_3D, ref_set_3d);
    e.on_site(REF_UNSET_3D, ref_unset_3d);
    e.on_site(ACTIVATE, activate);
    e.on_site(DROPPED_ITEM, dropped_item);
    e.on_site(ENABLE, enable);
    e.on_site(DISABLE, disable);
    e.on_site(SELL_BY_VENDOR, sell);
    e.on_site(SELL_BY_PLAYER, sell);
    e.on_site(TERMINAL_HACKED, terminal_hacked);
    e.on_site(TERMINAL_HACK_FAILED, terminal_hack_failed);
    e.on_site(LOCK_PICK_SUCCESS, lock_pick_success);
    e.on_site(LOCK_PICK_BROKEN, lock_pick_broken);
    e.on_site(LOCK_BROKEN, lock_broken);
    e.on_site(REFERENCE_TRASHED, form_gone);
    e.on_site(IMOD_RAN_OUT, imod_ran_out);
    for slot in ADD_WORN_ITEM {
        e.on_site(slot, add_worn_item);
    }
}
