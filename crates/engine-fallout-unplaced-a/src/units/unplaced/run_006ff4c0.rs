//! `(unplaced)/run_006ff4c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! None of these functions has an Xbox PDB name, so the source unit is
//! unconfirmed. What the exe itself shows:
//!
//! - `006ff4c0` walks a tree of four-way nodes (the engine map puts its
//!   neighbours in the distant-terrain system; the class name is not
//!   confirmed).
//! - `006ff580` .. `006ff880` are a singleton wrapper around the Steamworks
//!   API (`steam_api.dll` imports, named by Ghidra's import table): created
//!   lazily, held in the global at `011d8794`, `Steam::Steam` = `006ff690`.
//! - `006ff920` .. `006ffc20` are the three Steamworks `CCallback<Steam, ...>`
//!   objects the wrapper owns. The class names come from the RTTI of their
//!   vtables (`.?AV?$CCallback@VSteam@@U<type>@@$0...`): the received-stats,
//!   stored-stats and stored-achievement callbacks, plus their base
//!   `CCallbackBase`. The callback numbers (`0x44d`, `0x44e`, `0x44f`) are
//!   what the code passes to the import; what they are in Steamworks is not
//!   read from the exe.
//! - `006ffc90` .. `006ffd20` are a constructor family of an 8-byte value
//!   made of a word and a packed 20/4/8-bit word; its class is unknown.

#[allow(unused_imports)]
use crate::prelude::*;

/// `SteamAPI_Init` (import slot, Ghidra import name): returns a `bool` in AL.
const STEAM_API_INIT: u32 = 0x00fd_f478;
/// `SteamAPI_UnregisterCallback(callback)` (import slot, cdecl).
const STEAM_API_UNREGISTER_CALLBACK: u32 = 0x00fd_f47c;
/// `SteamAPI_Shutdown` (import slot).
const STEAM_API_SHUTDOWN: u32 = 0x00fd_f480;
/// `SteamUser()` (import slot): the interface pointer, or null.
const STEAM_USER: u32 = 0x00fd_f484;
/// `SteamAPI_RunCallbacks` (import slot).
const STEAM_API_RUN_CALLBACKS: u32 = 0x00fd_f488;
/// `SteamAPI_RegisterCallback(callback, number)` (import slot, cdecl).
const STEAM_API_REGISTER_CALLBACK: u32 = 0x00fd_f48c;
/// `SteamUserStats()` (import slot): the interface pointer, or null.
const STEAM_USER_STATS: u32 = 0x00fd_f490;
/// `SteamUtils()` (import slot): the interface pointer, or null.
const STEAM_UTILS: u32 = 0x00fd_f49c;

/// The global holding the lazily created Steam wrapper (or null).
const STEAM_SINGLETON: u32 = 0x011d_8794;
/// Size of the wrapper object.
const STEAM_SIZE: u32 = 0x44;

/// `operator new(size)` wrapper (`00401000`, cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` wrapper (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;

/// Vtables of the callback classes (RTTI names in the module doc).
const VTABLE_CALLBACK_BASE: u32 = 0x0106_e7b4;
const VTABLE_USER_STATS_RECEIVED: u32 = 0x0106_e7a4;
const VTABLE_USER_STATS_STORED: u32 = 0x0106_e7c4;
const VTABLE_USER_ACHIEVEMENT_STORED: u32 = 0x0106_e7d4;

/// Callback numbers each callback class registers with.
const CALLBACK_USER_STATS_RECEIVED: u32 = 0x44d;
const CALLBACK_USER_STATS_STORED: u32 = 0x44e;
const CALLBACK_USER_ACHIEVEMENT_STORED: u32 = 0x44f;

layout! {
    /// The Steam wrapper (PC only, no Xbox PDB class): three callback
    /// objects (`SteamCallback`, at +0x00, +0x14, +0x28) followed by its
    /// state; 0x44 bytes.
    pub struct Steam: 0x44 {
        // +0x00, +0x14 and +0x28 hold the three `SteamCallback` objects.
        /// Result of `SteamAPI_Init`.
        0x3c initialized: u8,
        /// Set by the received-stats handler when the result is 1.
        0x3d stats_received: u8,
        /// Set to 1 by the constructor; written by `006ff880`.
        0x3e flag_3e: u8,
        /// `SteamUtils()`'s slot `0x24` result (application id), 0 until read.
        0x40 app_id: u32,
    }
}

layout! {
    /// A `CCallback<Steam, T, N>` object (`CCallbackBase` + handler), 0x14
    /// bytes. Layout read from the constructors; not in the Xbox PDB.
    pub struct SteamCallback: 0x14 {
        /// Vtable (one per callback class).
        0x00 vtable: u32,
        /// Callback flags byte; bit 0 means registered with Steam.
        0x04 flags: u8,
        /// Set by Steam when it registers the object (0 from the base
        /// constructor).
        0x08 steam_value: u32,
        /// The object the handler is called on (the `Steam` wrapper).
        0x0c owner: u32,
        /// The member function called with the callback data.
        0x10 handler: u32,
    }
}

// Translated from 006ff4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_006ff4c0`: for a node of a four-way tree, hands the node's payload
/// (the object at +0x14) to `006f6270` when the node's level (`+4`) equals
/// the level of the tree it belongs to; otherwise, when the node's level is
/// above it, visits the four children (+0x20 .. +0x2c) that exist.
///
/// The node's level is `00726070` (the word at +4 of the node); the level
/// compared with is `0059bb30` of the pointer at +0 (the word at +0x24 of
/// that object). `00559450` reads the word at +0 of its argument; applied
/// twice it gives the third argument of `006f6270`. The class is not
/// confirmed.
pub fn fn_006ff4c0(e: &mut Engine, this: Ptr) {
    let node = this.addr();
    let level = e.call(0x0072_6070, &args![this]).u32();
    let parent = e.mem.u32(node);
    let target = e.call(0x0059_bb30, &args![parent]).u32();
    let payload = e.mem.u32(node + 0x14);
    if level == target && payload != 0 {
        let height = e.mem.u16(node + 0xa) as u32;
        let width = e.mem.u16(node + 0x8) as u32;
        let key = e.mem.u32(node + 4);
        let first = e.call(0x0055_9450, &args![this]).u32();
        let second = e.call(0x0055_9450, &args![first]).u32();
        e.call(0x006f_6270, &args![payload, second, key, width, height]);
    } else {
        let level = e.call(0x0072_6070, &args![this]).u32();
        let parent = e.mem.u32(node);
        let target = e.call(0x0059_bb30, &args![parent]).u32();
        if level > target {
            for index in 0..4u32 {
                let child = e.mem.u32(node + 0x20 + index * 4);
                if child != 0 {
                    fn_006ff4c0(e, Ptr::new(child));
                }
            }
        }
    }
}

// Translated from 006ff580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ff580 - C++ exception unwinding frame (the
// state saved around the constructor) is not translated.
/// The Steam wrapper singleton getter: creates it on first use (`0x44`
/// bytes from `00401000`, constructed by `006ff690`; a failed allocation
/// stores null) and returns it.
pub fn fn_006ff580(e: &mut Engine) -> Ptr<Steam> {
    if e.global::<u32>(STEAM_SINGLETON) == 0 {
        let block = e.call(OPERATOR_NEW, &args![STEAM_SIZE]).u32();
        let created = if block != 0 {
            fn_006ff690(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set_global(STEAM_SINGLETON, created);
    }
    Ptr::new(e.global::<u32>(STEAM_SINGLETON))
}

// Translated from 006ff610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether Steam initialised: the byte at +0x3c of the singleton.
pub fn fn_006ff610(e: &mut Engine) -> u8 {
    let steam = fn_006ff580(e);
    e.get(steam, Steam::initialized)
}

// Translated from 006ff620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the singleton (destructor `006ff660` with the delete flag) if
/// there is one, and clears the global.
pub fn fn_006ff620(e: &mut Engine) {
    let steam = e.global::<u32>(STEAM_SINGLETON);
    if steam != 0 {
        fn_006ff660(e, Ptr::new(steam), 1);
    }
    e.set_global(STEAM_SINGLETON, 0u32);
}

// Translated from 006ff660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The wrapper's scalar deleting destructor: runs `006ff7b0`, and frees the
/// block through `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_006ff660(e: &mut Engine, this: Ptr<Steam>, flags: u32) -> Ptr<Steam> {
    fn_006ff7b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006ff690 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ff690 - C++ exception unwinding frame (the
// partially constructed members are not destroyed on a throw) is not
// translated.
/// The Steam wrapper's constructor: builds its three callback objects with
/// their handlers (`006ff8a0` for the received-stats callback, `006ff8f0`
/// for the other two), clears the state, calls `SteamAPI_Init`, and when it
/// succeeds reads the login state (`006ff820`), asks `SteamUtils` (slot
/// `0x24`) for the application id and, when logged in, calls slot 0 of
/// `SteamUserStats`. Returns `this`.
pub fn fn_006ff690(e: &mut Engine, this: Ptr<Steam>) -> Ptr<Steam> {
    let base = this.addr();
    fn_006ff920(e, Ptr::new(base), this.addr(), 0x006f_f8a0);
    fn_006ff9f0(e, Ptr::new(base + 0x14), this.addr(), 0x006f_f8f0);
    fn_006ffa70(e, Ptr::new(base + 0x28), this.addr(), 0x006f_f8f0);
    e.set(this, Steam::initialized, 0);
    e.set(this, Steam::stats_received, 0);
    e.set(this, Steam::flag_3e, 1);
    e.set(this, Steam::app_id, 0);
    let initialized = e.call(STEAM_API_INIT, &args![]).u8();
    e.set(this, Steam::initialized, initialized);
    if initialized != 0 {
        let logged_on = fn_006ff820(e, this);
        if e.call(STEAM_UTILS, &args![]).u32() != 0 {
            let utils = e.call(STEAM_UTILS, &args![]).u32();
            let app_id = e.vcall(utils, 0x24, &args![]).u32();
            e.set(this, Steam::app_id, app_id);
        }
        if logged_on != 0 && e.call(STEAM_USER_STATS, &args![]).u32() != 0 {
            let stats = e.call(STEAM_USER_STATS, &args![]).u32();
            e.vcall(stats, 0, &args![]);
        }
    }
    this
}

// Translated from 006ff7b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ff7b0 - C++ exception unwinding frame states
// (only the order of member destruction on a throw) are not translated.
/// The wrapper's destructor body: `SteamAPI_Shutdown`, then the callback
/// objects are destroyed in reverse order (`006ffac0` at +0x28, `006ffa40`
/// at +0x14, `006ff9a0` at +0).
pub fn fn_006ff7b0(e: &mut Engine, this: Ptr<Steam>) {
    e.call(STEAM_API_SHUTDOWN, &args![]);
    fn_006ffac0(e, Ptr::new(this.addr() + 0x28));
    fn_006ffa40(e, Ptr::new(this.addr() + 0x14));
    fn_006ff9a0(e, Ptr::new(this.addr()));
}

// Translated from 006ff820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the user is logged on: false unless Steam initialised and
/// `SteamUser()` is non-null; then the AL result of slot 4 of that
/// interface.
pub fn fn_006ff820(e: &mut Engine, this: Ptr<Steam>) -> u8 {
    if e.get(this, Steam::initialized) != 0 && e.call(STEAM_USER, &args![]).u32() != 0 {
        let user = e.call(STEAM_USER, &args![]).u32();
        return e.vcall(user, 4, &args![]).u8();
    }
    0
}

// Translated from 006ff860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the Steam callbacks (`SteamAPI_RunCallbacks`).
pub fn fn_006ff860(e: &mut Engine, _this: Ptr<Steam>) {
    e.call(STEAM_API_RUN_CALLBACKS, &args![]);
}

// Translated from 006ff880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte argument at +0x3e.
pub fn fn_006ff880(e: &mut Engine, this: Ptr<Steam>, value: u8) {
    e.set(this, Steam::flag_3e, value);
}

// Translated from 006ff8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The received-stats handler: when the callback data starts with the 64-bit
/// value `(app_id, 0)` (the wrapper's application id at +0x40 and zero),
/// stores whether the data's word at +8 equals 1 at +0x3d.
pub fn fn_006ff8a0(e: &mut Engine, this: Ptr<Steam>, data: Ptr) {
    let low = e.get(this, Steam::app_id);
    if low == e.mem.u32(data.addr()) && e.mem.u32(data.addr() + 4) == 0 {
        let matched = e.mem.u32(data.addr() + 8) == 1;
        e.set(this, Steam::stats_received, matched as u8);
    }
}

// Translated from 006ff8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The handler of the stored-stats and stored-achievement callbacks: it
/// copies values into locals and does nothing else.
pub fn fn_006ff8f0(_e: &mut Engine, _this: Ptr<Steam>, _data: Ptr) {}

/// Constructor shared by the three `CCallback` classes: base constructor
/// (`006ff970`), the class's vtable, the owner and handler, then the
/// registration (`register`) when both are non-null.
fn callback_construct(
    e: &mut Engine,
    this: Ptr<SteamCallback>,
    vtable: u32,
    owner: u32,
    handler: u32,
    register: fn(&mut Engine, Ptr<SteamCallback>, u32, u32),
) -> Ptr<SteamCallback> {
    fn_006ff970(e, this);
    e.set(this, SteamCallback::vtable, vtable);
    e.set(this, SteamCallback::owner, owner);
    e.set(this, SteamCallback::handler, handler);
    if owner != 0 && handler != 0 {
        register(e, this, owner, handler);
    }
    this
}

/// Destructor shared by the three classes: sets the class's vtable and
/// unregisters (`006ffc00`) when bit 0 of the flags byte is set.
fn callback_destroy(e: &mut Engine, this: Ptr<SteamCallback>, vtable: u32) {
    e.set(this, SteamCallback::vtable, vtable);
    if e.get(this, SteamCallback::flags) & 1 != 0 {
        fn_006ffc00(e, this);
    }
}

/// Registration shared by the three classes. The code that sets bit 1 of the
/// flags was folded away by the compiler (a mask of 0), so it is not here.
fn callback_register(
    e: &mut Engine,
    this: Ptr<SteamCallback>,
    owner: u32,
    handler: u32,
    number: u32,
) {
    if owner == 0 || handler == 0 {
        return;
    }
    if e.get(this, SteamCallback::flags) & 1 != 0 {
        fn_006ffc00(e, this);
    }
    e.set(this, SteamCallback::owner, owner);
    e.set(this, SteamCallback::handler, handler);
    e.call(STEAM_API_REGISTER_CALLBACK, &args![this, number]);
}

// Translated from 006ff920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<Steam, UserStatsReceived_t, ...>` constructor (RTTI name).
pub fn fn_006ff920(
    e: &mut Engine,
    this: Ptr<SteamCallback>,
    owner: u32,
    handler: u32,
) -> Ptr<SteamCallback> {
    callback_construct(
        e,
        this,
        VTABLE_USER_STATS_RECEIVED,
        owner,
        handler,
        fn_006ffb20,
    )
}

// Translated from 006ff970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallbackBase` constructor (RTTI name): vtable, flags byte and the word
/// at +8 cleared. Returns `this`.
pub fn fn_006ff970(e: &mut Engine, this: Ptr<SteamCallback>) -> Ptr<SteamCallback> {
    e.set(this, SteamCallback::vtable, VTABLE_CALLBACK_BASE);
    e.set(this, SteamCallback::flags, 0);
    e.set(this, SteamCallback::steam_value, 0);
    this
}

// Translated from 006ff9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<Steam, UserStatsReceived_t, ...>` destructor.
pub fn fn_006ff9a0(e: &mut Engine, this: Ptr<SteamCallback>) {
    callback_destroy(e, this, VTABLE_USER_STATS_RECEIVED);
}

// Translated from 006ff9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<..>::Run(data)` (received-stats class, vtable slot 1): calls
/// the handler (+0x10) on the owner (+0x0c) with the data pointer.
pub fn fn_006ff9d0(e: &mut Engine, this: Ptr<SteamCallback>, data: u32) {
    let owner = e.get(this, SteamCallback::owner);
    let handler = e.get(this, SteamCallback::handler);
    e.call(handler, &args![owner, data]);
}

// Translated from 006ff9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<Steam, UserStatsStored_t, ...>` constructor (RTTI name).
pub fn fn_006ff9f0(
    e: &mut Engine,
    this: Ptr<SteamCallback>,
    owner: u32,
    handler: u32,
) -> Ptr<SteamCallback> {
    callback_construct(
        e,
        this,
        VTABLE_USER_STATS_STORED,
        owner,
        handler,
        fn_006ffb90,
    )
}

// Translated from 006ffa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<Steam, UserStatsStored_t, ...>` destructor.
pub fn fn_006ffa40(e: &mut Engine, this: Ptr<SteamCallback>) {
    callback_destroy(e, this, VTABLE_USER_STATS_STORED);
}

// Translated from 006ffa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<Steam, UserAchievementStored_t, ...>` constructor (RTTI name).
pub fn fn_006ffa70(
    e: &mut Engine,
    this: Ptr<SteamCallback>,
    owner: u32,
    handler: u32,
) -> Ptr<SteamCallback> {
    callback_construct(
        e,
        this,
        VTABLE_USER_ACHIEVEMENT_STORED,
        owner,
        handler,
        fn_006ffc20,
    )
}

// Translated from 006ffac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<Steam, UserAchievementStored_t, ...>` destructor.
pub fn fn_006ffac0(e: &mut Engine, this: Ptr<SteamCallback>) {
    callback_destroy(e, this, VTABLE_USER_ACHIEVEMENT_STORED);
}

// Translated from 006ffaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<..>::Run(data, ioFailure, apiCall)` (slot 0 of the
/// stored-stats and stored-achievement classes): calls the handler on the
/// owner with the data pointer only. The other three words (a flag and a
/// 64-bit call handle) are not read.
pub fn fn_006ffaf0(
    e: &mut Engine,
    this: Ptr<SteamCallback>,
    data: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
    fn_006ff9d0(e, this, data);
}

// Translated from 006ffb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetCallbackSizeBytes` of the stored-achievement class (vtable slot 2):
/// 0x98.
pub fn fn_006ffb10(_e: &mut Engine, _this: Ptr<SteamCallback>) -> u32 {
    0x98
}

// Translated from 006ffb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CCallback<Steam, UserStatsReceived_t, ...>::Register(owner, handler)`:
/// registers the object with Steam under callback number `0x44d`
/// (unregistering first when it already was).
pub fn fn_006ffb20(e: &mut Engine, this: Ptr<SteamCallback>, owner: u32, handler: u32) {
    callback_register(e, this, owner, handler, CALLBACK_USER_STATS_RECEIVED);
}

// Translated from 006ffb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Register(owner, handler)` of the stored-stats class: callback number
/// `0x44e`.
pub fn fn_006ffb90(e: &mut Engine, this: Ptr<SteamCallback>, owner: u32, handler: u32) {
    callback_register(e, this, owner, handler, CALLBACK_USER_STATS_STORED);
}

// Translated from 006ffc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SteamAPI_UnregisterCallback(this)`.
pub fn fn_006ffc00(e: &mut Engine, this: Ptr<SteamCallback>) {
    e.call(STEAM_API_UNREGISTER_CALLBACK, &args![this]);
}

// Translated from 006ffc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Register(owner, handler)` of the stored-achievement class: callback
/// number `0x44f`.
pub fn fn_006ffc20(e: &mut Engine, this: Ptr<SteamCallback>, owner: u32, handler: u32) {
    callback_register(e, this, owner, handler, CALLBACK_USER_ACHIEVEMENT_STORED);
}

layout! {
    /// An 8-byte value of unknown class: a word, then a word packing a
    /// 20-bit field (bits 0..19), a 4-bit field (bits 20..23) and a byte
    /// (bits 24..31).
    pub struct PackedPair: 0x08 {
        /// The first word.
        0x00 first: u32,
        /// The packed word.
        0x04 packed: u32,
    }
}

// Translated from 006ffc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of the packed pair: clears the first word and, by
/// the compiler's chain of masks, the packed word. Returns `this`.
pub fn fn_006ffc90(e: &mut Engine, this: Ptr<PackedPair>) -> Ptr<PackedPair> {
    // The compiler ANDs the first word with 0 (it ends up cleared).
    e.set(this, PackedPair::first, 0);
    let packed = e.get(this, PackedPair::packed) & 0xff0f_ffff;
    let packed = packed & 0x00ff_ffff;
    let packed = packed & 0xfff0_0000;
    e.set(this, PackedPair::packed, packed);
    this
}

// Translated from 006ffcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Four-argument constructor of the packed pair: forwards its words to
/// `006ffd20`. Returns `this`.
pub fn fn_006ffcf0(
    e: &mut Engine,
    this: Ptr<PackedPair>,
    first: u32,
    low_bits: u32,
    high_byte: u32,
    nibble: u32,
) -> Ptr<PackedPair> {
    fn_006ffd20(e, this, first, low_bits, high_byte, nibble);
    this
}

// Translated from 006ffd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the pair: the first word; bits 24..31 of the packed word from the low
/// byte of `high_byte`; bits 20..23 from the low nibble of `nibble`; bits
/// 0..19 from `low_bits`.
pub fn fn_006ffd20(
    e: &mut Engine,
    this: Ptr<PackedPair>,
    first: u32,
    low_bits: u32,
    high_byte: u32,
    nibble: u32,
) {
    // The compiler ANDs the old word with 0 before OR-ing the argument in.
    e.set(this, PackedPair::first, first);
    let packed = e.get(this, PackedPair::packed);
    let packed = (packed & 0x00ff_ffff) | ((high_byte & 0xff) << 24);
    let packed = (packed & 0xff0f_ffff) | ((nibble & 0xf) << 20);
    let packed = (packed & 0xfff0_0000) | (low_bits & 0xf_ffff);
    e.set(this, PackedPair::packed, packed);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006ff4c0, fn_006ff4c0(Ptr)),
        entry!(0x006ff580, fn_006ff580() -> Ptr<Steam>),
        entry!(0x006ff610, fn_006ff610() -> u8),
        entry!(0x006ff620, fn_006ff620()),
        entry!(0x006ff660, fn_006ff660(Ptr<Steam>, u32) -> Ptr<Steam>),
        entry!(0x006ff690, fn_006ff690(Ptr<Steam>) -> Ptr<Steam>),
        entry!(0x006ff7b0, fn_006ff7b0(Ptr<Steam>)),
        entry!(0x006ff820, fn_006ff820(Ptr<Steam>) -> u8),
        entry!(0x006ff860, fn_006ff860(Ptr<Steam>)),
        entry!(0x006ff880, fn_006ff880(Ptr<Steam>, u8)),
        entry!(0x006ff8a0, fn_006ff8a0(Ptr<Steam>, Ptr)),
        entry!(0x006ff8f0, fn_006ff8f0(Ptr<Steam>, Ptr)),
        entry!(
            0x006ff920,
            fn_006ff920(Ptr<SteamCallback>, u32, u32) -> Ptr<SteamCallback>
        ),
        entry!(
            0x006ff970,
            fn_006ff970(Ptr<SteamCallback>) -> Ptr<SteamCallback>
        ),
        entry!(0x006ff9a0, fn_006ff9a0(Ptr<SteamCallback>)),
        entry!(0x006ff9d0, fn_006ff9d0(Ptr<SteamCallback>, u32)),
        entry!(
            0x006ff9f0,
            fn_006ff9f0(Ptr<SteamCallback>, u32, u32) -> Ptr<SteamCallback>
        ),
        entry!(0x006ffa40, fn_006ffa40(Ptr<SteamCallback>)),
        entry!(
            0x006ffa70,
            fn_006ffa70(Ptr<SteamCallback>, u32, u32) -> Ptr<SteamCallback>
        ),
        entry!(0x006ffac0, fn_006ffac0(Ptr<SteamCallback>)),
        entry!(
            0x006ffaf0,
            fn_006ffaf0(Ptr<SteamCallback>, u32, u32, u32, u32)
        ),
        entry!(0x006ffb10, fn_006ffb10(Ptr<SteamCallback>) -> u32),
        entry!(0x006ffb20, fn_006ffb20(Ptr<SteamCallback>, u32, u32)),
        entry!(0x006ffb90, fn_006ffb90(Ptr<SteamCallback>, u32, u32)),
        entry!(0x006ffc00, fn_006ffc00(Ptr<SteamCallback>)),
        entry!(0x006ffc20, fn_006ffc20(Ptr<SteamCallback>, u32, u32)),
        entry!(0x006ffc90, fn_006ffc90(Ptr<PackedPair>) -> Ptr<PackedPair>),
        entry!(
            0x006ffcf0,
            fn_006ffcf0(Ptr<PackedPair>, u32, u32, u32, u32) -> Ptr<PackedPair>
        ),
        entry!(0x006ffd20, fn_006ffd20(Ptr<PackedPair>, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles every import and the allocator wrappers; records the calls.
    fn steam_engine() -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        for slot in [
            STEAM_API_INIT,
            STEAM_API_UNREGISTER_CALLBACK,
            STEAM_API_SHUTDOWN,
            STEAM_USER,
            STEAM_API_RUN_CALLBACKS,
            STEAM_API_REGISTER_CALLBACK,
            STEAM_USER_STATS,
            STEAM_UTILS,
            OPERATOR_NEW,
            OPERATOR_DELETE,
        ] {
            let log = log.clone();
            e.register_double(slot, move |_, a| {
                log.borrow_mut().push((slot, a.to_vec()));
                Ret::default()
            });
        }
        e.map(STEAM_SINGLETON & !0xfff, 0x1000);
        (e, log)
    }

    fn calls_to(log: &Log, slot: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(addr, _)| *addr == slot)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn tree_node_with_matching_level_hands_over_payload() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x30);
        let payload = e.mem.alloc(4);
        e.mem.set_u32(node + 4, 3);
        e.mem.set_u16(node + 8, 7);
        e.mem.set_u16(node + 0xa, 9);
        e.mem.set_u32(node + 0x14, payload);
        e.mem.set_u32(node, 0x5000);
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0059_bb30, |_, _| 3u32.into_ret());
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.map(0x5000, 0x1000);
        e.mem.set_u32(0x5000, 0x6000);
        e.call_log = Some(vec![]);
        e.register(0x006f_6270, |_, _| Ret::default());
        e.call(0x006f_f4c0, &args![node]);
        let log = e.call_log.take().unwrap();
        let hand_over = log.iter().find(|(addr, _)| *addr == 0x006f_6270).unwrap();
        assert_eq!(hand_over.1, vec![payload, 0x6000, 3, 7, 9]);
    }

    #[test]
    fn tree_node_above_the_level_visits_existing_children() {
        let mut e = Engine::new();
        let parent_node = e.mem.alloc(0x30);
        let child = e.mem.alloc(0x30);
        e.mem.set_u32(parent_node + 4, 5);
        e.mem.set_u32(parent_node + 0x28, child);
        e.mem.set_u32(child + 4, 2);
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0059_bb30, |_, _| 3u32.into_ret());
        e.register(0x0055_9450, |_, _| 0u32.into_ret());
        e.register(0x006f_6270, |_, _| panic!("no payload"));
        e.call_log = Some(vec![]);
        e.call(0x006f_f4c0, &args![parent_node]);
        let log = e.call_log.take().unwrap();
        // The child (level 2, below 3, no payload) was visited and ended the walk.
        assert!(log
            .iter()
            .any(|(addr, args)| *addr == 0x0072_6070 && args[0] == child));
    }

    #[test]
    fn tree_node_below_the_level_stops() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node + 4, 1);
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0059_bb30, |_, _| 3u32.into_ret());
        e.call(0x006f_f4c0, &args![node]);
    }

    #[test]
    fn singleton_is_created_once_and_reused() {
        let (mut e, log) = steam_engine();
        let block = e.mem.alloc(STEAM_SIZE);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        let first = e.call(0x006f_f580, &args![]).u32();
        let second = e.call(0x006f_f580, &args![]).u32();
        assert_eq!(first, block);
        assert_eq!(second, block);
        assert_eq!(e.global::<u32>(STEAM_SINGLETON), block);
        assert_eq!(calls_to(&log, STEAM_API_INIT).len(), 1);
    }

    #[test]
    fn singleton_with_failed_allocation_stores_null() {
        let (mut e, _log) = steam_engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x006f_f580, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(STEAM_SINGLETON), 0);
    }

    #[test]
    fn initialised_flag_comes_from_the_singleton() {
        let (mut e, _log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        e.set(steam, Steam::initialized, 1);
        e.set_global(STEAM_SINGLETON, steam.addr());
        assert_eq!(e.call(0x006f_f610, &args![]).u8(), 1);
        e.set(steam, Steam::initialized, 0);
        assert_eq!(e.call(0x006f_f610, &args![]).u8(), 0);
    }

    #[test]
    fn deleting_the_singleton_clears_the_global() {
        let (mut e, log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        e.set_global(STEAM_SINGLETON, steam.addr());
        e.call(0x006f_f620, &args![]);
        assert_eq!(e.global::<u32>(STEAM_SINGLETON), 0);
        assert_eq!(calls_to(&log, STEAM_API_SHUTDOWN).len(), 1);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![steam.addr()]]);
        // Nothing to delete the second time.
        e.call(0x006f_f620, &args![]);
        assert_eq!(calls_to(&log, STEAM_API_SHUTDOWN).len(), 1);
    }

    #[test]
    fn destructor_deletes_only_when_asked() {
        let (mut e, log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        let result = e.call(0x006f_f660, &args![steam, 0u32]).u32();
        assert_eq!(result, steam.addr());
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        e.call(0x006f_f660, &args![steam, 1u32]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn constructor_without_steam_leaves_it_uninitialised() {
        let (mut e, log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        e.set(steam, Steam::app_id, 99);
        let result = e.call(0x006f_f690, &args![steam]).u32();
        assert_eq!(result, steam.addr());
        assert_eq!(e.get(steam, Steam::initialized), 0);
        assert_eq!(e.get(steam, Steam::flag_3e), 1);
        assert_eq!(e.get(steam, Steam::app_id), 0);
        assert!(calls_to(&log, STEAM_USER).is_empty());
        // The three callback objects were built and registered.
        assert_eq!(e.mem.u32(steam.addr()), VTABLE_USER_STATS_RECEIVED);
        assert_eq!(e.mem.u32(steam.addr() + 0x14), VTABLE_USER_STATS_STORED);
        assert_eq!(
            e.mem.u32(steam.addr() + 0x28),
            VTABLE_USER_ACHIEVEMENT_STORED
        );
        assert_eq!(e.mem.u32(steam.addr() + 0x10), 0x006f_f8a0);
        assert_eq!(e.mem.u32(steam.addr() + 0x24), 0x006f_f8f0);
        assert_eq!(
            calls_to(&log, STEAM_API_REGISTER_CALLBACK),
            vec![
                vec![steam.addr(), 0x44d],
                vec![steam.addr() + 0x14, 0x44e],
                vec![steam.addr() + 0x28, 0x44f],
            ]
        );
    }

    #[test]
    fn constructor_with_steam_reads_the_application_id_and_requests_stats() {
        let (mut e, _log) = steam_engine();
        let events: Rc<RefCell<Vec<&'static str>>> = Rc::default();
        let utils = e.mem.alloc(4);
        let user = e.mem.alloc(4);
        let stats = e.mem.alloc(4);
        e.put_vtable(0x7000_0000, &[0x7100_0000]);
        e.put_vtable(0x7000_1000, &[0, 0x7100_0010]);
        e.put_vtable(0x7000_2000, &[0; 10]);
        e.mem.set_u32(utils, 0x7000_2000);
        e.mem.set_u32(user, 0x7000_1000);
        e.mem.set_u32(stats, 0x7000_0000);
        e.mem.set_u32(0x7000_2000 + 0x24, 0x7100_0020);
        e.register(STEAM_API_INIT, |_, _| 1u32.into_ret());
        e.register_double(STEAM_UTILS, move |_, _| utils.into_ret());
        e.register_double(STEAM_USER, move |_, _| user.into_ret());
        e.register_double(STEAM_USER_STATS, move |_, _| stats.into_ret());
        let log = events.clone();
        e.register_double(0x7100_0000, move |_, _| {
            log.borrow_mut().push("request stats");
            Ret::default()
        });
        e.register(0x7100_0010, |_, _| 1u32.into_ret());
        e.register(0x7100_0020, |_, _| 480u32.into_ret());
        let steam: Ptr<Steam> = e.new_object();
        e.call(0x006f_f690, &args![steam]);
        assert_eq!(e.get(steam, Steam::initialized), 1);
        assert_eq!(e.get(steam, Steam::app_id), 480);
        assert_eq!(*events.borrow(), vec!["request stats"]);
    }

    #[test]
    fn constructor_not_logged_on_skips_the_stats_request() {
        let (mut e, _log) = steam_engine();
        let user = e.mem.alloc(4);
        e.put_vtable(0x7000_1000, &[0, 0x7100_0010]);
        e.mem.set_u32(user, 0x7000_1000);
        e.register(STEAM_API_INIT, |_, _| 1u32.into_ret());
        e.register_double(STEAM_USER, move |_, _| user.into_ret());
        e.register(0x7100_0010, |_, _| Ret::default());
        e.register(STEAM_USER_STATS, |_, _| panic!("not logged on"));
        let steam: Ptr<Steam> = e.new_object();
        e.call(0x006f_f690, &args![steam]);
        assert_eq!(e.get(steam, Steam::initialized), 1);
        assert_eq!(e.get(steam, Steam::app_id), 0);
    }

    #[test]
    fn destructor_body_shuts_down_and_unregisters_registered_callbacks() {
        let (mut e, log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        e.mem.set_u8(steam.addr() + 0x04, 1);
        e.mem.set_u8(steam.addr() + 0x28 + 4, 1);
        e.call(0x006f_f7b0, &args![steam]);
        assert_eq!(calls_to(&log, STEAM_API_SHUTDOWN).len(), 1);
        assert_eq!(
            calls_to(&log, STEAM_API_UNREGISTER_CALLBACK),
            vec![vec![steam.addr() + 0x28], vec![steam.addr()]]
        );
        assert_eq!(e.mem.u32(steam.addr()), VTABLE_USER_STATS_RECEIVED);
        assert_eq!(e.mem.u32(steam.addr() + 0x14), VTABLE_USER_STATS_STORED);
    }

    #[test]
    fn login_state_needs_initialisation_and_an_interface() {
        let (mut e, _log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        assert_eq!(e.call(0x006f_f820, &args![steam]).u8(), 0);
        e.set(steam, Steam::initialized, 1);
        assert_eq!(e.call(0x006f_f820, &args![steam]).u8(), 0);
        let user = e.mem.alloc(4);
        e.put_vtable(0x7000_1000, &[0, 0x7100_0010]);
        e.mem.set_u32(user, 0x7000_1000);
        e.register_double(STEAM_USER, move |_, _| user.into_ret());
        e.register_double(0x7100_0010, move |_, a| (a[0] == user).into_ret());
        assert_eq!(e.call(0x006f_f820, &args![steam]).u8(), 1);
    }

    #[test]
    fn run_callbacks_calls_the_import() {
        let (mut e, log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        e.call(0x006f_f860, &args![steam]);
        assert_eq!(calls_to(&log, STEAM_API_RUN_CALLBACKS).len(), 1);
    }

    #[test]
    fn byte_setter_writes_offset_3e() {
        let (mut e, _log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        e.call(0x006f_f880, &args![steam, 0u8]);
        assert_eq!(e.get(steam, Steam::flag_3e), 0);
        e.call(0x006f_f880, &args![steam, 1u8]);
        assert_eq!(e.get(steam, Steam::flag_3e), 1);
    }

    #[test]
    fn received_stats_handler_matches_the_application_id() {
        let (mut e, _log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        e.set(steam, Steam::app_id, 22380);
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, 22380);
        e.mem.set_u32(data + 8, 1);
        e.call(0x006f_f8a0, &args![steam, data]);
        assert_eq!(e.get(steam, Steam::stats_received), 1);
        // A result other than 1 clears it.
        e.mem.set_u32(data + 8, 2);
        e.call(0x006f_f8a0, &args![steam, data]);
        assert_eq!(e.get(steam, Steam::stats_received), 0);
        // Another application id, or a nonzero high word, leaves it alone.
        e.set(steam, Steam::stats_received, 1);
        e.mem.set_u32(data, 5);
        e.call(0x006f_f8a0, &args![steam, data]);
        assert_eq!(e.get(steam, Steam::stats_received), 1);
        e.mem.set_u32(data, 22380);
        e.mem.set_u32(data + 4, 1);
        e.call(0x006f_f8a0, &args![steam, data]);
        assert_eq!(e.get(steam, Steam::stats_received), 1);
    }

    #[test]
    fn other_handler_does_nothing() {
        let (mut e, _log) = steam_engine();
        let steam: Ptr<Steam> = e.new_object();
        let data = e.mem.alloc(16);
        e.call(0x006f_f8f0, &args![steam, data]);
        assert_eq!(e.get(steam, Steam::stats_received), 0);
    }

    #[test]
    fn base_constructor_clears_state() {
        let (mut e, _log) = steam_engine();
        let callback: Ptr<SteamCallback> = e.new_object();
        e.set(callback, SteamCallback::flags, 3);
        e.set(callback, SteamCallback::steam_value, 9);
        let result = e.call(0x006f_f970, &args![callback]).u32();
        assert_eq!(result, callback.addr());
        assert_eq!(e.get(callback, SteamCallback::vtable), VTABLE_CALLBACK_BASE);
        assert_eq!(e.get(callback, SteamCallback::flags), 0);
        assert_eq!(e.get(callback, SteamCallback::steam_value), 0);
    }

    #[test]
    fn callback_constructors_register_only_with_owner_and_handler() {
        let cases = [
            (0x006f_f920, VTABLE_USER_STATS_RECEIVED, 0x44d),
            (0x006f_f9f0, VTABLE_USER_STATS_STORED, 0x44e),
            (0x006f_fa70, VTABLE_USER_ACHIEVEMENT_STORED, 0x44f),
        ];
        for (addr, vtable, number) in cases {
            let (mut e, log) = steam_engine();
            let callback: Ptr<SteamCallback> = e.new_object();
            let result = e.call(addr, &args![callback, 0x1111u32, 0x2222u32]).u32();
            assert_eq!(result, callback.addr());
            assert_eq!(e.get(callback, SteamCallback::vtable), vtable);
            assert_eq!(e.get(callback, SteamCallback::owner), 0x1111);
            assert_eq!(e.get(callback, SteamCallback::handler), 0x2222);
            assert_eq!(
                calls_to(&log, STEAM_API_REGISTER_CALLBACK),
                vec![vec![callback.addr(), number]]
            );
            // Without a handler the fields are stored but nothing registers.
            let (mut e, log) = steam_engine();
            let callback: Ptr<SteamCallback> = e.new_object();
            e.call(addr, &args![callback, 0x1111u32, 0u32]);
            assert_eq!(e.get(callback, SteamCallback::owner), 0x1111);
            assert!(calls_to(&log, STEAM_API_REGISTER_CALLBACK).is_empty());
        }
    }

    #[test]
    fn callback_destructors_unregister_when_registered() {
        for (addr, vtable) in [
            (0x006f_f9a0, VTABLE_USER_STATS_RECEIVED),
            (0x006f_fa40, VTABLE_USER_STATS_STORED),
            (0x006f_fac0, VTABLE_USER_ACHIEVEMENT_STORED),
        ] {
            let (mut e, log) = steam_engine();
            let callback: Ptr<SteamCallback> = e.new_object();
            e.call(addr, &args![callback]);
            assert_eq!(e.get(callback, SteamCallback::vtable), vtable);
            assert!(calls_to(&log, STEAM_API_UNREGISTER_CALLBACK).is_empty());
            e.set(callback, SteamCallback::flags, 1);
            e.call(addr, &args![callback]);
            assert_eq!(
                calls_to(&log, STEAM_API_UNREGISTER_CALLBACK),
                vec![vec![callback.addr()]]
            );
        }
    }

    #[test]
    fn run_calls_the_handler_on_the_owner() {
        for addr in [0x006f_f9d0u32, 0x006f_faf0] {
            let (mut e, _log) = steam_engine();
            let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
            let sink = seen.clone();
            e.register_double(0x7100_0000, move |_, a| {
                sink.borrow_mut().push(a.to_vec());
                Ret::default()
            });
            let callback: Ptr<SteamCallback> = e.new_object();
            e.set(callback, SteamCallback::owner, 0x1234);
            e.set(callback, SteamCallback::handler, 0x7100_0000);
            if addr == 0x006f_f9d0 {
                e.call(addr, &args![callback, 0x5678u32]);
            } else {
                e.call(addr, &args![callback, 0x5678u32, 1u32, 2u32, 3u32]);
            }
            assert_eq!(*seen.borrow(), vec![vec![0x1234, 0x5678]]);
        }
    }

    #[test]
    fn callback_size_is_0x98() {
        let (mut e, _log) = steam_engine();
        let callback: Ptr<SteamCallback> = e.new_object();
        assert_eq!(e.call(0x006f_fb10, &args![callback]).u32(), 0x98);
    }

    #[test]
    fn register_replaces_an_existing_registration() {
        for (addr, number) in [
            (0x006f_fb20, 0x44d),
            (0x006f_fb90, 0x44e),
            (0x006f_fc20, 0x44f),
        ] {
            let (mut e, log) = steam_engine();
            let callback: Ptr<SteamCallback> = e.new_object();
            e.set(callback, SteamCallback::flags, 1);
            e.call(addr, &args![callback, 0xaaaau32, 0xbbbbu32]);
            assert_eq!(
                calls_to(&log, STEAM_API_UNREGISTER_CALLBACK),
                vec![vec![callback.addr()]]
            );
            assert_eq!(
                calls_to(&log, STEAM_API_REGISTER_CALLBACK),
                vec![vec![callback.addr(), number]]
            );
            assert_eq!(e.get(callback, SteamCallback::owner), 0xaaaa);
            assert_eq!(e.get(callback, SteamCallback::handler), 0xbbbb);
            // A null owner or handler does nothing at all.
            let (mut e, log) = steam_engine();
            let callback: Ptr<SteamCallback> = e.new_object();
            e.call(addr, &args![callback, 0u32, 0xbbbbu32]);
            e.call(addr, &args![callback, 0xaaaau32, 0u32]);
            assert!(log.borrow().is_empty());
            assert_eq!(e.get(callback, SteamCallback::owner), 0);
        }
    }

    #[test]
    fn unregister_calls_the_import() {
        let (mut e, log) = steam_engine();
        let callback: Ptr<SteamCallback> = e.new_object();
        e.call(0x006f_fc00, &args![callback]);
        assert_eq!(
            calls_to(&log, STEAM_API_UNREGISTER_CALLBACK),
            vec![vec![callback.addr()]]
        );
    }

    #[test]
    fn packed_pair_default_is_zero() {
        let mut e = Engine::new();
        let pair: Ptr<PackedPair> = e.new_object();
        e.set(pair, PackedPair::first, 0xdead);
        e.set(pair, PackedPair::packed, 0xffff_ffff);
        let result = e.call(0x006f_fc90, &args![pair]).u32();
        assert_eq!(result, pair.addr());
        assert_eq!(e.get(pair, PackedPair::first), 0);
        assert_eq!(e.get(pair, PackedPair::packed), 0);
    }

    #[test]
    fn packed_pair_setter_places_each_field() {
        let mut e = Engine::new();
        let pair: Ptr<PackedPair> = e.new_object();
        e.call(
            0x006f_fd20,
            &args![pair, 0x1234u32, 0x1_2345u32, 0x1abu32, 0x1fu32],
        );
        assert_eq!(e.get(pair, PackedPair::first), 0x1234);
        // Low 20 bits of the value (0x12345), nibble 0xf, byte 0xab.
        assert_eq!(e.get(pair, PackedPair::packed), 0xabf1_2345);
    }

    #[test]
    fn packed_pair_constructor_forwards_and_returns_this() {
        let mut e = Engine::new();
        let pair: Ptr<PackedPair> = e.new_object();
        let result = e
            .call(0x006f_fcf0, &args![pair, 7u32, 0xfffffu32, 0xffu32, 0u32])
            .u32();
        assert_eq!(result, pair.addr());
        assert_eq!(e.get(pair, PackedPair::first), 7);
        assert_eq!(e.get(pair, PackedPair::packed), 0xff0f_ffff);
    }
}
