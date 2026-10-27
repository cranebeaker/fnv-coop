// Engine-facing logic of the bot methods bot.menu / bot.say / bot.inventory / bot.vats (T6a), game-free.
// Every engine access goes through IEngine (raw memory + a few thiscall wrappers), so the logic is unit-tested on a synthetic memory
// image with a call recorder (tests/test_bots.cpp). The live IEngine is in bots_module.cpp (SEH-guarded reads, address pins checked).
// Addresses and layouts come from docs/re/dialogmenu-menus-vats-inventory.md (static analysis only: NOTHING here has run in the game).
// Each method is defensive: it checks a state gate and refuses with a reason instead of poking the engine when a gate fails.
#pragma once
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <mutex>
#include <string>
#include <vector>

#include "bots_addrs.h"
#include "common/minijson.h"

#include "fnvcoop_mut.h"  // FNV_MUT(n): runtime mutation twins (env FNVCOOP_MUT=n); constant false in production builds
// Runtime twins for this file (13..56), every one must make tests/test_bots.cpp fail:
//  13 dialogue pick ignores the ready state    14 pick matches the wrong list index        15 pick skips the special-topic refusal
//  16 menu id map has 0x3F1/0x3F0 swapped      17 VATS wins nothing over the menu id       18 inventory counts deltas, not totals
//  19 vats job never gives up on a bad target  20 pick leaves the click gate untouched     21 consent buttons swapped
//  22 vats limb request silently accepted     23 terminal entry index off by one     24 terminal pick ignores the menu id gate
//  25 terminal pick is not recorded
//  26 speech-check INFOs refused again   27 drop log ignores the base form    28 drop log ignores the time window    29 fast travel skips the may-travel gate
//  30 fast travel skips the busy-menu gate  31 fast travel writes the marker at the wrong offset  32 refs list includes disabled refs
// Native message box press (33..41; 33, 36 and 40 are in bot_rpc.h):
//  34 MessageMenu vtable check skipped    35 button index off by one    37 null button tile not refused    38 top-menu gate skipped
//  39 closing-box gate skipped (no current message)    41 main-thread gate skipped
// Always-run state for bot.move (42..46; 42, 43 and 46 are in bot_rpc.h):
//  44 always-run state ignores gamepad mode  45 always-run state reads player+0x652 (the auto-move byte)
// Drop log base match (48):
//  48 the base form matches as case-sensitive text (the engine's "falloutnv.esm:015169" never equals "FalloutNV.esm:015169")
// bot.vats queue and execute (49..56):
//  49 the accept detour is never armed      50 execute is called with an empty queue     51 the tutorial box is never closed
//  52 the target cycle is skipped           53 the tutorial box (0x423) is reported as "vats"
//  57 a tutorial box that opens after the tutorial step is ignored
//  58 an attack queued just before a late tutorial box is forgotten (a second attack is queued after the box)
//  54 the accept gate forces every slot and the held query     55 the VATSMenu vtable check is skipped    56 the hit-chance gate is skipped

namespace nvbots {

// A call pin compares 16 bytes at an engine entry with the prologue pinned in bots_addrs.h. Another module of ours may have detoured that entry
// before the bots module looks (world_time's pending-travel hook sits on PendingTravelSet 0x93BE30; after a core reload the bots module starts
// second and read "pin does not match this exe", which switched off all of bot.menu / bot.inventory / bot.dialog / bot.fasttravel: worldtime_proof
// "bot.menu: unsupported: engine glue not wired", sweep 2026-10-22). A detoured entry is accepted when it holds a `jmp rel32` AND the trampoline
// of our own hook there starts with the pinned bytes (the stolen prologue). `tramp` = nvhooks::trampolineOfAny(entry) contents, or null.
inline bool pinBytesOk(const unsigned char* got, const unsigned char* want, const unsigned char* tramp) {
    if (std::memcmp(got, want, 16) == 0) return true;
    if (FNV_MUT(65) || !tramp) return false;                       // twin 65: a detoured entry is never accepted
    if (got[0] != 0xE9) return false;
    return FNV_MUT(66) || std::memcmp(tramp, want, 5) == 0;         // twin 66: any jmp is accepted, whatever the trampoline holds
}

// The kKnownRewrites entry for `addr` when `got` is exactly that rewrite (bots_addrs.h), else null.
inline const addr::KnownRewrite* knownRewrite(unsigned addr, const unsigned char* got) {
    for (const addr::KnownRewrite& r : addr::kKnownRewrites)
        if (r.addr == addr && (FNV_MUT(67) || std::memcmp(got, r.bytes, 16) == 0)) return &r;   // twin 67: any bytes pass as the rewrite
    return nullptr;
}

using nvcoop::json::Value;
using Addr32 = uint32_t;

namespace eng {  // addresses and offsets (see the RE note)
constexpr Addr32 kDialogMenuPtr = 0x11D9510, kInfoSpoken = 0x11D9518, kClockObj = 0x11F6394;
constexpr uint32_t kDlgState = 0x28, kDlgListBox = 0x70, kDlgPartner = 0x80, kDlgClickGate = 0x10C, kDlgPending = 0x104;
constexpr uint32_t kItemInfo = 0x18, kItemFlag = 0x25, kFormId = 0x0C;
constexpr Addr32 kInterfaceMgrPtr = 0x11D8A80, kVatsCamera = 0x11F2250, kVatsMenuPtr = 0x11DB0D4;
constexpr uint32_t kVatsMode = 8;
// bot.vats (static RE of 1.4.0.525, see the VatsJob comment). Camera modes: 0 off, 1 target overview, 3 zoom, 2 limb select, 4 playback.
constexpr Addr32 kVatsSelTarget = 0x11F21CC;      // Actor* of the selected VATS target (HandleClick 2/3 writes it), 0 = none
constexpr uint32_t kVatsMenuPart = 0x100;         // VATSMenu+0x100: selected body-part entry of the current target
constexpr uint32_t kPartHitChance = 0x28;         // body-part entry +0x28: hit chance (float, percent)
constexpr uint32_t kVatsMenuAp = 0xE0;            // VATSMenu+0xE0: action points left (float)
constexpr uint32_t kMenuIdVats = 0x420, kMenuIdTutorial = 0x423;
constexpr int kVatsClickNextTarget = 3;           // VATSMenu::HandleClick tile 3 = next target
constexpr int kDlgReady = 2;
constexpr Addr32 kTerminalMenuPtr = 0x11D9334;   // *(TerminalMenu**), set while the terminal menu (id 0x421) is open
constexpr uint32_t kTermList = 0x68, kTermTerminal = 0xA8, kTermRef = 0xAC;
// Always-run (the code that uses each one is pinned in bots_addrs.h kRunPins):
constexpr uint32_t kPlayerAlwaysRun = 0x651;      // PlayerCharacter byte, flipped on the AlwaysRun (10) press edge
constexpr Addr32 kPadConnected = 0x11D8A84;       // byte: an XInput pad answered this frame
constexpr Addr32 kDisable360Value = 0x11D8C50;    // value byte of Setting bDisable360Controller:Interface (object 0x11D8C4C, +4)
// Native MessageMenu (id 0x3E9). The object comes from xNVSE InterfaceManager::GetMenuByType(0x3E9): g_TileMenuArray (NiTArray<TileMenu*>
// 0x11F3508, data pointer at +4) [id - 0x3E9], then TileMenu::menu (+0x3C). The engine also keeps it in its MessageMenu singleton 0x11DA4F0.
constexpr Addr32 kTileMenuArrayData = 0x11F350C;
constexpr uint32_t kTileMenuMenu = 0x3C, kMenuIdMin = 0x3E9, kMenuIdMessage = 0x3E9;
// MessageMenu+0x40 is a ListBox<int> (ctor 0x7642F0, vtable 0x107286C) of the buttons: its tList head node {item*, next*} is at +4, so at
// menu+0x44. Each item is {Tile* @0, int value @4}. The builder 0x7A92E0 appends one item per non-empty button text in order, with
// value = (signed char)entry[0x1C] + text index (ShowMessageBox passes 0, so the value is the button index scripts get).
// The six tiles at menu+0x28..0x3C are the XML <id> 0..5 parts set by SetField (0x7A8AB0); they are NOT the buttons.
constexpr uint32_t kMsgButtonHead = 0x44;
constexpr int kMsgClickId = 7;   // HandleClick acts only on id 7 (a button tile)
// The current message: FUN 0x7A92A0 = (*0x11DA4F4 < queue.count) ? queue.data[*0x11DA4F4] : 0; queue = BSSimpleArray at 0x11DA53C
// (data @+4, count @+8). HandleClick runs the message callback only for a current message; none means the box is closing.
constexpr Addr32 kMsgQueue = 0x11DA53C, kMsgQueueIdx = 0x11DA4F4;
// Prologue pins for the engine functions the live engine calls (thiscall unless noted). The pin verifier that checked them,
// tools/bots/re/verify_pins.py, is not in the public repo.
}  // namespace eng

struct IEngine {
    virtual ~IEngine() = default;
    virtual bool rd32(Addr32 a, uint32_t* v) = 0;
    virtual bool rd8(Addr32 a, uint8_t* v) = 0;
    virtual bool wr8(Addr32 a, uint8_t v) = 0;
    virtual bool wr32(Addr32 a, uint32_t v) = 0;
    virtual uint32_t nowTick() = 0;                                  // the game's millisecond clock (0x825C00 on 0x11F6394)
    // DialogMenu helpers
    virtual bool lbSelect(Addr32 lb, int idx) = 0;                   // 0x83E430
    virtual Addr32 lbSelected(Addr32 lb) = 0;                        // 0x83E4C0
    virtual Addr32 itemInfo(Addr32 item) = 0;                        // 0x9611E0 (item -> TESTopicInfo*)
    virtual bool infoSpecial(Addr32 info) = 0;                       // 0x7627C0 (flag 0x80): the click path takes another branch (item select 0x83E850, speak flag 1): refused
    virtual bool infoSpeechCheck(Addr32 info) = 0;                   // 0x7627E0 || 0x762820: speech-check / challenge INFO; HandleClick only plays a UI sound for these, so the plain path is the same
    virtual void dlgItemClick(Addr32 menu, Addr32 item) = 0;         // 0x762860
    virtual void lbAfter(Addr32 lb) = 0;                             // 0x83EBB0
    virtual int dlgSpeak(Addr32 menu, int flag) = 0;                 // 0x762FF0 -> next state
    // menus / inventory
    virtual uint32_t topMenuId(Addr32 mgr) = 0;                      // 0x714F00
    virtual void termClick(Addr32 menu, Addr32 tile) = 0;            // 0x757F70 thiscall(menu, -1, tile), ret 8
    virtual Addr32 containerData(Addr32 ref) = 0;                    // 0x4BF220 (creates the data if missing)
    virtual int itemTotal(Addr32 data, Addr32 form) = 0;             // 0x4C8F30
    // genuine fast travel (the map menu's deferred worker 0x798A00, replicated)
    virtual bool canFastTravel(Addr32 player) = 0;                   // 0x93D660 (the engine's own may-travel check, shows its refusal message)
    virtual Addr32 newTravelObj() = 0;                               // heap 0x34 + ctor 0x456540, 0 on failure
    virtual void setPendingTravel(Addr32 player, Addr32 obj) = 0;    // 0x93BE30
    // native message box
    virtual bool onMainThread() = 0;                                 // the calling thread is the game main thread
    virtual bool msgMenuHandleClick(Addr32 menu, int id, Addr32 tile) = 0;   // MessageMenu vtable slot 3 (0x7AA070); false = refused or faulted
    virtual bool closeAllMenus() = 0;                                // 0x7037E0 CloseAllMenusCore (cdecl, no arguments), main thread; false = refused or faulted
    // bot.vats (all main thread; false / 0 / -1 = refused or faulted)
    virtual Addr32 menuByType(uint32_t id) = 0;                      // 0xA09030: TileMenu* of an open menu, 0 when it is not open
    virtual bool closeTutorial() = 0;                                // 0x7E8D60 (cdecl, no arguments)
    virtual bool vatsMenuClick(Addr32 menu, int id) = 0;             // VATSMenu vtable slot 3 (0x7EECB0): HandleClick(id, nullptr)
    virtual int vatsQueueCount() = 0;                                // 0x5AE380 (ECX = 0x11F2250): queued VATS actions
    virtual bool vatsExecute() = 0;                                  // 0x705780: start playback of the queue
    virtual bool vatsArmAccept(bool on) = 0;                         // arm / disarm the attack-button detour (VatsAcceptGate); false = no detour
    // references
    virtual bool resolveRef(const std::string& ref, Addr32* out) = 0;    // "" = the player
    virtual bool resolveForm(const std::string& form, Addr32* out) = 0;
    virtual bool resolveFormId(const std::string& form, uint32_t* refId) = 0;   // runtime form id of "Plugin.esm:HEX"
    virtual std::string nameOfForm(Addr32 form) = 0;                 // "Plugin.esm:HEX"
};

inline const char* menuKindOf(uint32_t id) {
    switch (id) {
        case 0x3E9: return "message";
        case 0x3EA: case 0x3EB: case 0x3FF: case 0x40B: case 0x425: return "pipboy";
        case 0x3EC: return "hud";
        case 0x3EF: return "loading";
        case 0x3F0: return FNV_MUT(16) ? "dialogue" : "container";   // twin 16: 0x3F0/0x3F1 swapped
        case 0x3F1: return FNV_MUT(16) ? "container" : "dialogue";
        case 0x3F5: return "pause";
        case 0x3F6: return "lockpick";
        case 0x41D: return "barter";
        case 0x41F: return "hacking";
        case 0x420: return "vats";
        case 0x421: return "terminal";
        case 0x423: return FNV_MUT(53) ? "vats" : "tutorial";   // TutorialMenu (1059): VATS input stops while it is open
        default: return id ? "other" : "none";
    }
}

// ---- bot.menu ----
inline bool menuInfo(IEngine& e, Value* out, std::string* err) {
    uint32_t mgr = 0, mode = 0;
    if (!e.rd32(eng::kInterfaceMgrPtr, &mgr) || !mgr) { *err = "interface manager not ready"; return false; }
    e.rd32(eng::kVatsCamera + eng::kVatsMode, &mode);
    const uint32_t id = e.topMenuId(mgr);
    const char* kind = menuKindOf(id);
    if (FNV_MUT(17)) {
        (void)mode;
    } else {
        // a VATS camera mode makes the kind "vats", but never hides a tutorial box (it blocks every VATS input)
        if (mode != 0 && (id != eng::kMenuIdTutorial || FNV_MUT(53))) kind = "vats";
    }
    *out = Value::object();
    out->set("kind", kind).set("id", (int64_t)id).set("vatsMode", (int64_t)mode);
    uint32_t qIdx = 0, qCount = 0;      // the engine's message box queue: a backlog (idx climbing to a fixed count) or a feed (count growing)
    if (e.rd32(eng::kMsgQueueIdx, &qIdx) && e.rd32(eng::kMsgQueue + 8, &qCount)) out->set("msgIdx", (int64_t)qIdx).set("msgCount", (int64_t)qCount);
    uint32_t dlg = 0;
    if (e.rd32(eng::kDialogMenuPtr, &dlg) && dlg) {
        uint32_t st = 0;
        e.rd32(dlg + eng::kDlgState, &st);
        out->set("dialogState", (int64_t)st).set("dialogReady", st == (uint32_t)eng::kDlgReady);
    }
    return true;
}

// ---- always-run state for bot.move ----
// What the player-controls update (PlayerCharacter::vf2F8, run decision at 0x941A2B) does with the Run control (9):
//   keyboard mode, always-run on:  Run up = run speed, Run down = walk     (so holding Run for "run" made the player walk)
//   keyboard mode, always-run off: Run down = run speed, Run up = walk
//   gamepad mode (a pad answers and bDisable360Controller is 0): the toggle is ignored, Run down = run speed
// Returns 1 when the engine treats always-run as on, 0 when off (or ignored), -1 when the bytes cannot be read.
inline int alwaysRunState(IEngine& e) {
    Addr32 player = 0;
    uint8_t on = 0, pad = 0, padOff = 0;
    if (!e.resolveRef("", &player) || !player) return -1;
    if (!e.rd8(player + eng::kPlayerAlwaysRun + (FNV_MUT(45) ? 1 : 0), &on)) return -1;   // twin 45: reads the byte after the flag
    if (!e.rd8(eng::kPadConnected, &pad) || !e.rd8(eng::kDisable360Value, &padOff)) return -1;
    if (!FNV_MUT(44)) {
        if (pad && !padOff) return 0;
    }
    return on ? 1 : 0;
}

// ---- bot.menu.press, native path: press a button of the engine's own message box (MessageMenu, menu id 1001) ----
// The caller uses it only when the UI layer (ui.answer) has no box. Gates, in this order: the game main thread; the interface manager is
// ready and the message menu is the top menu (if not, *applies stays false and the caller reports the UI layer's error); GetMenuByType(1001)
// gives an object with the MessageMenu vtable; a current message exists (none = the box is closing); `button` (0-based, in the order the
// box lists its buttons) is in the button list and has a tile. Then HandleClick(7, tile): what the engine does for a mouse click and for its
// own Enter / hotkey path, so the engine records the button value, runs the message callback and closes the box (or shows the next one).
// On success `out` gets {menu, button, value, buttons}: value is what the engine records (GetButtonPressed), buttons is how many the box has.
inline bool pressNativeMessageButton(IEngine& e, int button, Value* out, std::string* err, bool* applies) {
    *applies = false;
    if (!FNV_MUT(41)) {
        if (!e.onMainThread()) { *applies = true; *err = "the native message press must run on the game main thread"; return false; }
    }
    uint32_t mgr = 0;
    if (!e.rd32(eng::kInterfaceMgrPtr, &mgr) || !mgr) { *err = "interface manager not ready"; return false; }
    const uint32_t top = e.topMenuId(mgr);
    if (!FNV_MUT(38)) {
        if (top != eng::kMenuIdMessage) { *err = "the top menu is not the native message box (top menu id " + std::to_string(top) + ")"; return false; }
    }
    *applies = true;
    uint32_t arr = 0, tileMenu = 0, menu = 0, vt = 0;
    if (!e.rd32(eng::kTileMenuArrayData, &arr) || !arr || !e.rd32(arr + 4 * (eng::kMenuIdMessage - eng::kMenuIdMin), &tileMenu) || !tileMenu ||
        !e.rd32(tileMenu + eng::kTileMenuMenu, &menu) || !menu) {
        *err = "message menu object missing (GetMenuByType(1001) is null)";
        return false;
    }
    e.rd32(menu, &vt);
    if (!FNV_MUT(34)) {
        if (vt != addr::MessageMenuVtbl) {
            char b[96];
            std::snprintf(b, sizeof b, "menu 1001 is not a MessageMenu (vtable %08X, want %08X): refused", vt, addr::MessageMenuVtbl);
            *err = b;
            return false;
        }
    }
    uint32_t idx = 0, count = 0;
    const bool current = e.rd32(eng::kMsgQueueIdx, &idx) && e.rd32(eng::kMsgQueue + 8, &count) && idx < count;
    if (!FNV_MUT(39)) {
        if (!current) { *err = "the message box has no current message (it is closing)"; return false; }
    }
    struct Btn { Addr32 tile; int32_t value; };
    std::vector<Btn> bs;
    uint32_t node = menu + eng::kMsgButtonHead;
    for (int i = 0; node && i < 64; ++i) {
        uint32_t item = 0, next = 0;
        if (!e.rd32(node, &item) || !e.rd32(node + 4, &next)) break;
        if (item) {
            uint32_t tile = 0, value = 0;
            e.rd32(item, &tile);
            e.rd32(item + 4, &value);
            bs.push_back({tile, (int32_t)value});
        }
        node = next;
    }
    if (button < 0 || button >= (int)bs.size()) {
        *err = "button " + std::to_string(button) + " is not shown (the box has " + std::to_string(bs.size()) + " buttons)";
        return false;
    }
    const Btn& pick = FNV_MUT(35) ? bs[(size_t)(button + 1 < (int)bs.size() ? button + 1 : button)] : bs[(size_t)button];
    if (!FNV_MUT(37)) {
        if (!pick.tile) { *err = "button " + std::to_string(button) + " has no tile"; return false; }
    }
    if (!e.msgMenuHandleClick(menu, eng::kMsgClickId, pick.tile)) { *err = "the live glue refused the HandleClick call (pins or vtable did not match)"; return false; }
    out->set("menu", (int64_t)eng::kMenuIdMessage).set("button", (int64_t)button).set("value", (int64_t)pick.value).set("buttons", (int64_t)bs.size());
    return true;
}

// ---- bot.menu.close {kind?} ----
// Closes the open menu the engine's own way: CloseAllMenusCore (0x7037E0, what the CloseAllMenus command calls, bots_addrs.h) closes every menu of
// the stack, the path the game takes when the player leaves a container with Tab or B. Gates, in this order: the game main thread; the interface
// manager is ready; a menu is open; the top menu is of the wanted kind. `kind` defaults to "container". "any" accepts every kind a human could leave
// with the cancel key, but never the message box (bot.menu.press answers those, closing one here skips its callback), the loading screen, the pause
// menu, VATS or the tutorial box. A kind is a bot.menu kind name (container, barter, terminal, lockpick, hacking, pipboy, dialogue, other).
// The close happens inside the call; the menu is gone from bot.menu a frame or two later (the caller polls). On success `out` gets {closed, menu, kind}.
inline bool kindIsLeavable(const std::string& kind) {
    if (FNV_MUT(63)) return kind != "none";
    return kind != "none" && kind != "message" && kind != "loading" && kind != "pause" && kind != "vats" && kind != "tutorial" && kind != "hud";
}
inline bool closeOpenMenu(IEngine& e, const std::string& want, Value* out, std::string* err) {
    if (!FNV_MUT(60)) {
        if (!e.onMainThread()) { *err = "the menu close must run on the game main thread"; return false; }
    }
    uint32_t mgr = 0;
    if (!e.rd32(eng::kInterfaceMgrPtr, &mgr) || !mgr) { *err = "interface manager not ready"; return false; }
    const uint32_t top = e.topMenuId(mgr);
    const std::string kind = menuKindOf(top);
    if (!top && !FNV_MUT(61)) { *err = "no menu is open"; return false; }
    const std::string wanted = want.empty() ? "container" : want;
    if (!FNV_MUT(59)) {
        if (wanted == "any") {
            if (!kindIsLeavable(kind)) { *err = "the top menu is " + kind + " (id " + std::to_string(top) + "): not closed from code"; return false; }
        } else if (kind != wanted) {
            *err = "the top menu is " + kind + " (id " + std::to_string(top) + "), not " + wanted + ": left open";
            return false;
        }
    }
    const bool called = e.closeAllMenus();
    if (!called && !FNV_MUT(62)) { *err = "the live glue refused the CloseAllMenusCore call (pins did not match, or it faulted)"; return false; }
    out->set("closed", true).set("menu", (int64_t)top).set("kind", kind);
    return true;
}

// ---- bot.say ----
struct Choice { int idx; Addr32 item, info; uint32_t formId; };

inline bool dialogChoices(IEngine& e, std::vector<Choice>* out, std::string* err) {
    uint32_t dlg = 0, lb = 0;
    if (!e.rd32(eng::kDialogMenuPtr, &dlg) || !dlg) { *err = "no dialogue open"; return false; }
    if (!e.rd32(dlg + eng::kDlgListBox, &lb) || !lb) { *err = "dialogue has no choice list"; return false; }
    for (int i = 0; i < 64 && e.lbSelect(lb, i); ++i) {   // moves the selection, exactly like a click would
        Choice c{i, e.lbSelected(lb), 0, 0};
        if (!c.item) break;
        uint32_t info = 0;
        if (e.rd32(c.item + eng::kItemInfo, &info) && info) { c.info = info; e.rd32(info + eng::kFormId, &c.formId); }
        out->push_back(c);
    }
    return true;
}

// Replicates the plain-topic path of DialogMenu::HandleClick (0x7624F0) for the choice whose INFO form id is `wantFormId`:
// select, clear item flag, item click (0x762860), list after-click (0x83EBB0), clear +0x104, record the spoken INFO, speak (0x762FF0).
inline bool dialogPick(IEngine& e, uint32_t wantFormId, Value* out, std::string* err) {
    uint32_t dlg = 0, lb = 0, state = 0;
    if (!e.rd32(eng::kDialogMenuPtr, &dlg) || !dlg) { *err = "no dialogue open"; return false; }
    e.rd32(dlg + eng::kDlgState, &state);
    if (!FNV_MUT(13)) {
        if (state != (uint32_t)eng::kDlgReady) { *err = "dialogue not ready (state " + std::to_string(state) + ", need 2)"; return false; }
    }
    uint32_t gate = 0;
    e.rd32(dlg + eng::kDlgClickGate, &gate);
    if (!FNV_MUT(20)) {
        if (gate) {
            if (e.nowTick() - gate < 500) { *err = "click gate closed (a click happened < 500 ms ago)"; return false; }
            e.wr32(dlg + eng::kDlgClickGate, 0);
        }
    }
    std::vector<Choice> cs;
    if (!dialogChoices(e, &cs, err)) return false;
    const Choice* hit = nullptr;
    for (const Choice& c : cs)
        if (c.formId == wantFormId) { hit = &c; break; }
    if (!hit) {
        Value ids = Value::array();
        for (const Choice& c : cs) ids.push((int64_t)c.formId);
        *err = "INFO not offered in this dialogue";
        out->set("offered", ids);
        return false;
    }
    if (FNV_MUT(15)) {
    } else {
        if (e.infoSpecial(hit->info)) { *err = "special INFO (flag 0x80): the replicated click path does not cover it"; return false; }
    }
    if (FNV_MUT(26)) {
        if (e.infoSpeechCheck(hit->info)) { *err = "special INFO (speech check): refused"; return false; }
    }
    const bool speechCheck = e.infoSpeechCheck(hit->info);
    e.rd32(dlg + eng::kDlgListBox, &lb);
    if (FNV_MUT(14)) {
        e.lbSelect(lb, hit->idx + 1);
    } else {
        e.lbSelect(lb, hit->idx);
    }
    const Addr32 item = e.lbSelected(lb);
    if (item != hit->item) { *err = "selection did not land on the requested choice"; return false; }
    e.wr8(item + eng::kItemFlag, 0);
    e.dlgItemClick(dlg, item);
    e.lbAfter(lb);
    e.wr8(dlg + eng::kDlgPending, 0);
    e.wr32(eng::kInfoSpoken, hit->info);
    const int next = e.dlgSpeak(dlg, 0);
    e.wr32(dlg + eng::kDlgState, (uint32_t)next);
    out->set("said", true).set("index", (int64_t)hit->idx).set("nextState", (int64_t)next).set("speechCheck", speechCheck);
    return true;
}

// ---- bot.inventory ----
// Totals (base container + entry delta, equipped included) through 0x4C8F30. Explicit `items` are always answered; the entry list
// adds every item the reference has an ExtraContainerChanges entry for. Base-container-only items are NOT enumerated (unknown
// layout), so the reply says partial:true unless explicit items were requested.
inline bool inventoryInfo(IEngine& e, const std::string& ref, const std::vector<std::string>& wanted, Value* out, std::string* err) {
    Addr32 r = 0;
    if (!e.resolveRef(ref, &r) || !r) { *err = "reference not loaded"; return false; }
    const Addr32 data = e.containerData(r);
    if (!data) { *err = "no container data"; return false; }
    Value items = Value::array();
    auto add = [&](Addr32 form, int count) {
        Value it = Value::object();
        it.set("item", e.nameOfForm(form)).set("count", (int64_t)count);
        items.push(it);
    };
    for (const std::string& w : wanted) {
        Addr32 f = 0;
        if (!e.resolveForm(w, &f)) { *err = "unknown item " + w; return false; }
        add(f, e.itemTotal(data, f));
    }
    if (wanted.empty()) {
        uint32_t list = 0, node = 0;
        e.rd32(data, &list);                       // EntryDataList*, its head node is embedded {data, next}
        node = list;
        for (int guard = 0; node && guard < 4096; ++guard) {
            uint32_t entry = 0, next = 0, type = 0;
            int32_t delta = 0;
            if (!e.rd32(node, &entry) || !e.rd32(node + 4, &next)) break;
            if (entry && e.rd32(entry + 8, &type) && type) {
                uint32_t d = 0;
                e.rd32(entry + 4, &d);
                delta = (int32_t)d;
                if (FNV_MUT(18)) {
                    if (delta) add(type, delta);
                } else {
                    const int total = e.itemTotal(data, type);
                    if (total) add(type, total);
                }
            }
            node = next;
        }
    }
    out->set("items", items).set("partial", wanted.empty());
    return true;
}

// ---- dev.terminal / dev.terminalState ----
// TerminalMenu (id 0x421): ListBox at menu+0x68 (head node at lb+4), node {data,next}, data = item {Tile*@0, TerminalEntry*@4},
// entry text String {char*@0}. Picking calls the click handler 0x757F70(menu, -1, tile). The engine keeps NO per-entry "used" flag,
// so dev.terminalState reports what THIS bot picked (recorded by BotRpc), not what a human clicked earlier.
struct TermEntry { int idx; Addr32 tile; Addr32 entry; std::string text; };

inline bool terminalEntries(IEngine& e, const std::string& ref, std::vector<TermEntry>* out, Addr32* menuOut, std::string* err) {
    uint32_t mgr = 0, menu = 0, top = 0;
    if (!e.rd32(eng::kInterfaceMgrPtr, &mgr) || !mgr) { *err = "interface manager not ready"; return false; }
    top = e.topMenuId(mgr);
    if (!FNV_MUT(24)) {
        if (top != 0x421) { *err = "terminal menu is not open (top menu id " + std::to_string(top) + "); activate the terminal first"; return false; }
    }
    if (!e.rd32(eng::kTerminalMenuPtr, &menu) || !menu) { *err = "terminal menu object missing"; return false; }
    Addr32 want = 0, shown = 0;
    if (!e.resolveRef(ref, &want) || !want) { *err = "terminal reference not loaded"; return false; }
    if (!e.rd32(menu + eng::kTermRef, &shown) || shown != want) { *err = "the open terminal is a different reference"; return false; }
    uint32_t node = 0;
    e.rd32(menu + eng::kTermList + 4, &node);
    for (int i = 0; node && i < 64; ++i) {
        uint32_t item = 0, next = 0, tile = 0, entry = 0, text = 0;
        if (!e.rd32(node, &item) || !e.rd32(node + 4, &next)) break;
        if (item) {
            e.rd32(item, &tile);
            e.rd32(item + 4, &entry);
            TermEntry t{(int)out->size(), tile, entry, ""};
            if (entry && e.rd32(entry, &text) && text) {
                for (int k = 0; k < 120; ++k) {
                    uint8_t ch = 0;
                    if (!e.rd8(text + k, &ch) || !ch) break;
                    t.text.push_back((char)ch);
                }
            }
            out->push_back(t);
        }
        node = next;
    }
    *menuOut = menu;
    return true;
}

inline bool terminalPick(IEngine& e, const std::string& ref, int entry, Value* out, std::string* err) {
    std::vector<TermEntry> es;
    Addr32 menu = 0;
    if (!terminalEntries(e, ref, &es, &menu, err)) return false;
    Value names = Value::array();
    for (const TermEntry& t : es) names.push(t.text);
    out->set("entries", names);
    if (entry < 0 || entry >= (int)es.size()) { *err = "entry index out of range (" + std::to_string(es.size()) + " entries shown)"; return false; }
    // twin 23: terminal entry index off by one
    const TermEntry& pick = FNV_MUT(23) ? es[(size_t)(entry + 1 < (int)es.size() ? entry + 1 : entry)] : es[(size_t)entry];
    if (!pick.tile) { *err = "entry has no tile"; return false; }
    e.termClick(menu, pick.tile);
    out->set("used", pick.text).set("index", (int64_t)pick.idx);
    return true;
}

// ---- dev.countRefs {base, cell?} / dev.refsOf {base, cell?} ----
// Walks the references of base form `base` in a cell (default: the player's cell) through TESObjectCELL::objectList (cell+0xAC, an
// embedded tList head {data,next}); skips deleted/disabled refs. Offsets: ref->baseForm +0x20, ref->parentCell +0x40, form flags +0x08
// (disabled 0x800, deleted 0x20), refID +0x0C: from xNVSE headers, unverified at runtime.
inline bool walkRefs(IEngine& e, const std::string& base, const std::string& cell, std::vector<Addr32>* refs, std::string* err) {
    uint32_t baseId = 0, cellAddr = 0;
    if (!e.resolveFormId(base, &baseId)) { *err = "unknown base form " + base; return false; }
    if (cell.empty()) {
        Addr32 player = 0;
        if (!e.resolveRef("", &player) || !player || !e.rd32(player + 0x40, &cellAddr) || !cellAddr) { *err = "player cell unavailable"; return false; }
    } else if (!e.resolveForm(cell, &cellAddr) || !cellAddr) {
        *err = "cell not loaded";
        return false;
    }
    uint32_t node = cellAddr + 0xAC;
    for (int guard = 0; node && guard < 100000; ++guard) {
        uint32_t ref = 0, next = 0;
        if (!e.rd32(node, &ref) || !e.rd32(node + 4, &next)) break;
        if (ref) {
            uint32_t bf = 0, id = 0, flags = 0;
            const uint32_t skip = FNV_MUT(32) ? 0x20 : 0x820;   // twin 32: the disabled flag is not skipped
            if (e.rd32(ref + 0x20, &bf) && bf && e.rd32(bf + eng::kFormId, &id) && id == baseId && e.rd32(ref + 0x08, &flags) && !(flags & skip))
                refs->push_back(ref);
        }
        node = next;
    }
    return true;
}

inline bool countRefs(IEngine& e, const std::string& base, const std::string& cell, Value* out, std::string* err) {
    std::vector<Addr32> refs;
    if (!walkRefs(e, base, cell, &refs, err)) return false;
    *out = Value::object();
    out->set("count", (int64_t)refs.size());
    return true;
}

// The references themselves ("Plugin.esm:HEX"), capped at 512: the fallback for finding what a Drop created when the OnDrop event is unavailable
// (list before, drop, list after, take the difference).
inline bool refsOf(IEngine& e, const std::string& base, const std::string& cell, Value* out, std::string* err) {
    std::vector<Addr32> refs;
    if (!walkRefs(e, base, cell, &refs, err)) return false;
    Value arr = Value::array();
    for (size_t i = 0; i < refs.size() && i < 512; ++i) arr.push(e.nameOfForm(refs[i]));
    *out = Value::object();
    out->set("refs", arr).set("count", (int64_t)refs.size());
    return true;
}

// ---- bot.fasttravel {marker} ----
// The engine's own map-menu path, minus the clicks: gate on the busy menus and on the engine's may-travel check (0x93D660), then
// do what the deferred worker 0x798A00 does after the player answers Yes: travel object (0x34 bytes) with the marker at +0x30, handed to
// the player (0x93BE30). The main loop moves the player on a later frame, so success here means "queued"; the caller waits for the cell change.
inline bool fastTravel(IEngine& e, const std::string& marker, Value* out, std::string* err) {
    Addr32 mk = 0, player = 0;
    if (!e.resolveRef(marker, &mk) || !mk || marker.empty()) { *err = "marker not loaded"; return false; }
    if (!e.resolveRef("", &player) || !player) { *err = "player not loaded"; return false; }
    uint32_t mgr = 0, menuKindId = 0, vatsMode = 0;
    if (e.rd32(eng::kInterfaceMgrPtr, &mgr) && mgr) menuKindId = e.topMenuId(mgr);
    e.rd32(eng::kVatsCamera + eng::kVatsMode, &vatsMode);
    if (!FNV_MUT(30)) {
        if (vatsMode != 0 || menuKindId == 0x3EF || menuKindId == 0x3F1 || menuKindId == 0x420 || menuKindId == 0x3F0 || menuKindId == 0x421 || menuKindId == 0x41D) {
            *err = "busy: a loading, dialogue, VATS, container, terminal or barter menu is open";
            return false;
        }
    }
    uint32_t pending = 0;
    if (e.rd32(player + addr::kPlayerPending, &pending) && pending) { *err = "busy: a travel or teleport is already pending"; return false; }
    if (!FNV_MUT(29)) {
        if (!e.canFastTravel(player)) { *err = "the engine refuses fast travel right now (combat, enemies near, overencumbered, in air or blocked here)"; return false; }
    }
    const Addr32 obj = e.newTravelObj();
    if (!obj) { *err = "could not allocate the travel object"; return false; }
    if (FNV_MUT(31)) {
        e.wr32(obj + 0x2C, mk);
    } else {
        e.wr32(obj + addr::kTravelObjMarker, mk);
    }
    e.setPendingTravel(player, obj);
    out->set("genuine", true).set("queued", true);
    return true;
}

// True when two form names ("Plugin.esm:HEX", or a bare HEX) name the same form. The plugin part compares without case and the
// hex part compares as a number. The engine names forms "falloutnv.esm:015169" and callers send "FalloutNV.esm:15169" (in game, 2026-10-21).
inline bool sameFormName(const std::string& a, const std::string& b) {
    if (FNV_MUT(48)) return a == b;   // twin 48: names compared as plain text
    const size_t ca = a.rfind(':'), cb = b.rfind(':');
    const std::string pa = ca == std::string::npos ? "" : a.substr(0, ca), pb = cb == std::string::npos ? "" : b.substr(0, cb);
    if (pa.size() != pb.size()) return false;
    for (size_t i = 0; i < pa.size(); ++i) {
        const char x = pa[i] >= 'A' && pa[i] <= 'Z' ? (char)(pa[i] - 'A' + 'a') : pa[i];
        const char y = pb[i] >= 'A' && pb[i] <= 'Z' ? (char)(pb[i] - 'A' + 'a') : pb[i];
        if (x != y) return false;
    }
    auto hex = [](const std::string& s, uint32_t* v) {
        if (s.empty() || s.size() > 8) return false;
        uint32_t n = 0;
        for (char c : s) {
            const int d = c >= '0' && c <= '9' ? c - '0' : c >= 'a' && c <= 'f' ? c - 'a' + 10 : c >= 'A' && c <= 'F' ? c - 'A' + 10 : -1;
            if (d < 0) return false;
            n = n * 16 + (uint32_t)d;
        }
        *v = n;
        return true;
    };
    uint32_t va = 0, vb = 0;
    return hex(ca == std::string::npos ? a : a.substr(ca + 1), &va) && hex(cb == std::string::npos ? b : b.substr(cb + 1), &vb) && va == vb;
}

// ---- bot.drops: what the engine's OnDrop event reported ----
// xNVSE's event manager calls a native handler with (dropper, droppedRef) when something is dropped, from Cell and world code (the same
// point the OnDrop script event fires). The handler records strings only, so this class is game-free. Correlate by base form plus a time window.
// The base form matches by sameFormName, not by text.
struct DropRec { int64_t seq; std::string ref, base, dropper; uint64_t atMs; };
class DropLog {
public:
    void record(const std::string& ref, const std::string& base, const std::string& dropper, uint64_t nowMs) {
        std::lock_guard<std::mutex> lk(mu_);
        recs_.push_back({++seq_, ref, base, dropper, nowMs});
        if (recs_.size() > 256) recs_.erase(recs_.begin());
    }
    int64_t seq() {
        std::lock_guard<std::mutex> lk(mu_);
        return seq_;
    }
    // drops newer than `since`, of `base` when given, no older than windowMs
    Value query(int64_t since, const std::string& base, uint64_t nowMs, uint64_t windowMs) {
        std::lock_guard<std::mutex> lk(mu_);
        Value arr = Value::array();
        for (const DropRec& r : recs_) {
            if (r.seq <= since) continue;
            if (!FNV_MUT(27)) {
                if (!base.empty() && !sameFormName(r.base, base)) continue;
            }
            if (!FNV_MUT(28)) {
                if (nowMs > r.atMs && nowMs - r.atMs > windowMs) continue;
            }
            Value v = Value::object();
            v.set("seq", r.seq).set("ref", r.ref).set("base", r.base).set("dropper", r.dropper);
            arr.push(v);
        }
        return arr;
    }

private:
    std::mutex mu_;
    std::vector<DropRec> recs_;
    int64_t seq_ = 0;
};

// ---- bot.vats: the arm-and-confirm gate of the attack-button detour (0x7EEB50) ----
// The live module detours 0x7EEB50(slot, held, binding). While the gate is armed, the press-edge query of slot 1 (slot 1, held 0) answers
// true. VATSMenu::Update then runs its own queue path with all its own checks (target, hit chance, AP cost, sounds, queue text). Every other
// query goes to the original. The gate disarms itself when the queue count rises above the count at arm time. The mouse pick 0x7F6AF0 can
// end a frame before the queue path, so the gate stays armed over frames until the count rises or the job disarms it.
class VatsAcceptGate {
public:
    void arm(int queueCount) { armed_ = true; base_ = queueCount; forced_ = 0; }
    void disarm() { armed_ = false; }
    bool armed() const { return armed_; }
    int forced() const { return forced_; }
    // The detour asks this first. True = answer "pressed" and do not call the original.
    bool force(int slot, int held, int queueCount) {
        if (!armed_) return false;
        if (queueCount > base_) { armed_ = false; return false; }
        if (!FNV_MUT(54)) {   // twin 54: the gate also forces the other slots and the held query
            if (slot != 1 || held != 0) return false;
        }
        ++forced_;
        return true;
    }

private:
    bool armed_ = false;
    int base_ = 0, forced_ = 0;
};

// ---- bot.vats / test.vats.hit : a tick-driven job (main thread, one tick per frame) ----
// Static RE of 1.4.0.525; nothing here has run in the game yet. VATS never reads the Activate or Right controls (it reads raw keys and the
// left mouse button), so the job uses the input driver only to enter VATS. The other steps use the engine's own functions:
//   enter     tap Vats (16), wait for camera mode != 0
//   tutorial  while GetMenuByType(0x423) (TutorialMenu) is open, close it with 0x7E8D60 (VATSMenu::Update does nothing while it is open)
//   zoom      wait for camera mode 2 (the 1 -> 3 -> 2 zoom ends) and top menu 0x420
//   target    while the target global 0x11F21CC is not the wanted ref, VATSMenu::HandleClick(3, nullptr) (next target), up to 12 times
//   queue     gate: a body part is selected, its hit chance is > 0 and AP left is > 0 (the engine checks the exact cost). Then arm the
//             attack-button detour. Done when the queue count (0x5AE380) rises.
//   execute   0x705780, only when the queue count is >= 1 (with an empty queue it leaves VATS)
//   playback  camera mode 4 seen, done when the mode is 0 again
// Every failed step stops the job with a reason. Limb choice is not implemented.
class VatsJob {
public:
    enum Phase { kIdle, kEnter, kTutorial, kZoom, kTarget, kQueue, kExecute, kPlayback, kDone, kFailed };
    static constexpr uint64_t kSettleMs = 1500;   // kQueue: how long a 0 hit chance / missing body part may last before it is a failure
    static constexpr int kMaxCycles = 12;
    bool start(IEngine& e, const std::string& ref, const std::string& limb, uint64_t now, std::string* err) {
        if (!FNV_MUT(22)) {
            if (!limb.empty()) { *err = "unsupported: limb selection in VATS is not implemented"; return false; }
        } else {
            (void)limb;
        }
        if (running()) { *err = "a VATS job is already running"; return false; }
        if (!e.resolveRef(ref, &want_) || !want_) { *err = "target not loaded"; return false; }
        uint32_t mode = 0;
        e.rd32(eng::kVatsCamera + eng::kVatsMode, &mode);
        if (mode != 0) { *err = "VATS is already active"; return false; }
        phase_ = kEnter; reason_.clear(); t0_ = tPhase_ = tClick_ = tClose_ = now; cycles_ = 0; closeCalls_ = 0;
        sawPlayback_ = false; tapped_ = false; armed_ = false; queued_ = false; queueBase_ = 0; hit_ = ap_ = 0.0f;
        return true;
    }
    // Called every frame while running. `tap(ctrl)` presses a control through the input driver.
    template <class Tap>
    void tick(IEngine& e, uint64_t now, Tap tap) {
        if (!running()) return;
        uint32_t mode = 0;
        e.rd32(eng::kVatsCamera + eng::kVatsMode, &mode);
        char b[200];
        if (now - t0_ > 25000) {
            std::snprintf(b, sizeof b, "timeout (phase %s)", name(phase_));
            return fail(e, b);
        }
        if (phase_ != kEnter && phase_ != kExecute && phase_ != kPlayback && mode == 0) return fail(e, "vats_closed (camera mode 0 before the attack was queued)");
        // In game 2026-10-21 the VATS tutorial box (1059) opened AFTER the job had passed the tutorial step, and the hit chance then read 0
        // (VATSMenu::Update does nothing while it is open). So a tutorial box sends the job back to the tutorial step from any later step.
        if ((phase_ == kZoom || phase_ == kTarget || phase_ == kQueue) && !FNV_MUT(57) && e.menuByType(eng::kMenuIdTutorial)) {
            // the attack may already be in the queue: keep that, so the job does not queue a second one after the box (PR #27 review)
            if (armed_ && !FNV_MUT(58) && e.vatsQueueCount() > queueBase_) queued_ = true;
            if (armed_) disarm(e);
            armed_ = false;
            return to(kTutorial, now);
        }
        switch (phase_) {
            case kEnter:
                if (!tapped_) { tap(16); tapped_ = true; tPhase_ = now; }
                else if (mode != 0) to(kTutorial, now);
                else if (now - tPhase_ > 3000) fail(e, "vats_not_entered (no weapon, no target in view, or no AP)");
                break;
            case kTutorial:
                if (!e.menuByType(eng::kMenuIdTutorial)) { to(kZoom, now); break; }
                if (!FNV_MUT(51)) {   // twin 51: the tutorial box is never closed
                    if (!closeCalls_ || now - tClose_ >= 500) { e.closeTutorial(); ++closeCalls_; tClose_ = now; }
                }
                if (now - tPhase_ > 3000) fail(e, "tutorial_box_stays_open (VATS input is blocked while TutorialMenu 1059 is open)");
                break;
            case kZoom: {
                if (e.menuByType(eng::kMenuIdTutorial)) { to(kTutorial, now); break; }
                const uint32_t top = topMenu(e);
                if (mode == 2 && top == eng::kMenuIdVats) { to(kTarget, now); break; }
                if (now - tPhase_ > 5000) {
                    std::snprintf(b, sizeof b, "vats_not_ready (camera mode %u, top menu %u; need mode 2 and menu 1056)", mode, top);
                    fail(e, b);
                }
                break;
            }
            case kTarget: {
                if (mode == 4) return fail(e, "vats_playback_started_before_the_attack_was_queued");
                Addr32 menu = 0;
                if (!vatsMenu(e, &menu)) return;
                uint32_t sel = 0;
                e.rd32(eng::kVatsSelTarget, &sel);
                if (sel == want_ || FNV_MUT(52)) { to(kQueue, now); break; }   // twin 52: the target cycle is skipped
                if (mode != 2) break;                                         // a zoom to the new target: wait
                if (cycles_ && now - tClick_ < 300) break;                    // one click per 300 ms, so the menu can update
                if (cycles_ >= kMaxCycles && !FNV_MUT(19)) {                  // twin 19: the job never gives up on a bad target
                    std::snprintf(b, sizeof b, "target_not_selectable (cycled %d times, selected %08X, want %08X)", cycles_, sel, want_);
                    return fail(e, b);
                }
                if (!e.vatsMenuClick(menu, eng::kVatsClickNextTarget)) return fail(e, "vats_click_refused (VATSMenu HandleClick pins or vtable did not match)");
                ++cycles_;
                tClick_ = now;
                break;
            }
            case kQueue: {
                if (!armed_) {
                    if (mode != 2) {
                        if (mode == 4 || now - tPhase_ > 3000) {
                            std::snprintf(b, sizeof b, "attack_not_queued (camera mode %u, need 2)", mode);
                            fail(e, b);
                        }
                        break;
                    }
                    if (queued_) { to(kExecute, now); break; }   // queued before a late tutorial box: execute it, never queue again
                    Addr32 menu = 0;
                    if (!vatsMenu(e, &menu)) return;
                    uint32_t part = 0;
                    e.rd32(menu + eng::kVatsMenuPart, &part);
                    // the menu fills the body part and its hit chance on its next updates (after a zoom or a closed tutorial box): wait a little
                    if (!part) { if (now - tPhase_ < kSettleMs) break; return fail(e, "attack_not_queued (no body part is selected)"); }
                    hit_ = rdf(e, part + eng::kPartHitChance);
                    ap_ = rdf(e, menu + eng::kVatsMenuAp);
                    if (!(hit_ > 0.0f) && now - tPhase_ < kSettleMs) break;
                    if (!(hit_ > 0.0f) && !FNV_MUT(56)) {   // twin 56: the hit-chance gate is skipped
                        std::snprintf(b, sizeof b, "attack_not_queued (hit chance %.1f, AP %.1f: the engine refuses a 0 hit chance)", hit_, ap_);
                        return fail(e, b);
                    }
                    if (!(ap_ > 0.0f)) {
                        std::snprintf(b, sizeof b, "attack_not_queued (no AP left: hit chance %.1f, AP %.1f)", hit_, ap_);
                        return fail(e, b);
                    }
                    queueBase_ = e.vatsQueueCount();
                    if (queueBase_ < 0) return fail(e, "attack_not_queued (the VATS queue count cannot be read)");
                    if (!FNV_MUT(49)) {   // twin 49: the accept detour is never armed
                        if (!e.vatsArmAccept(true)) return fail(e, "accept_hook_missing (the VATS attack-button detour is not installed)");
                    }
                    armed_ = true;
                    tPhase_ = now;
                    break;
                }
                const int n = e.vatsQueueCount();
                if (n > queueBase_) { disarm(e); to(kExecute, now); break; }
                if (mode == 4 || now - tPhase_ > 3000) {
                    std::snprintf(b, sizeof b, "attack_not_queued (hit chance %.1f, AP %.1f, queue %d, camera mode %u)", hit_, ap_, n, mode);
                    fail(e, b);
                }
                break;
            }
            case kExecute: {
                const int n = e.vatsQueueCount();
                if (n < 1 && !FNV_MUT(50)) {   // twin 50: execute is called with an empty queue (that leaves VATS)
                    std::snprintf(b, sizeof b, "queue_empty_at_execute (queue %d: the queued attack was removed)", n);
                    return fail(e, b);
                }
                if (!e.vatsExecute()) return fail(e, "vats_execute_refused (pins did not match or not on the main thread)");
                to(kPlayback, now);
                break;
            }
            case kPlayback:
                if (mode == 4) sawPlayback_ = true;
                if (mode == 0) { if (sawPlayback_) phase_ = kDone; else fail(e, "left_vats_without_playback"); }
                break;
            default: break;
        }
    }
    Phase phase() const { return phase_; }
    const std::string& reason() const { return reason_; }
    bool running() const { return phase_ != kIdle && phase_ != kDone && phase_ != kFailed; }
    int cycles() const { return cycles_; }
    static const char* name(Phase p) {
        static const char* const n[] = {"idle", "enter", "tutorial", "zoom", "target", "queue", "execute", "playback", "done", "failed"};
        return n[p];
    }

private:
    void to(Phase p, uint64_t now) { phase_ = p; tPhase_ = now; }
    void disarm(IEngine& e) {
        if (armed_) e.vatsArmAccept(false);
        armed_ = false;
    }
    void fail(IEngine& e, const std::string& why) {
        disarm(e);
        phase_ = kFailed;
        reason_ = why;
    }
    uint32_t topMenu(IEngine& e) {
        uint32_t mgr = 0;
        return e.rd32(eng::kInterfaceMgrPtr, &mgr) && mgr ? e.topMenuId(mgr) : 0;
    }
    static float rdf(IEngine& e, Addr32 a) {
        uint32_t v = 0;
        float f = 0.0f;
        if (!e.rd32(a, &v)) return 0.0f;
        std::memcpy(&f, &v, sizeof f);
        return f;
    }
    // The VATSMenu object (*0x11DB0D4). Refuses any object whose vtable is not VATSMenu's: HandleClick and the offsets are only known for it.
    bool vatsMenu(IEngine& e, Addr32* menu) {
        uint32_t m = 0, vt = 0;
        if (!e.rd32(eng::kVatsMenuPtr, &m) || !m) { fail(e, "vats_menu_missing (VATSMenu pointer 0x11DB0D4 is null)"); return false; }
        e.rd32(m, &vt);
        if (vt != addr::VatsMenuVtbl && !FNV_MUT(55)) {   // twin 55: the VATSMenu vtable check is skipped
            char b[120];
            std::snprintf(b, sizeof b, "vats_menu_wrong_type (vtable %08X, want %08X): refused", vt, addr::VatsMenuVtbl);
            fail(e, b);
            return false;
        }
        *menu = m;
        return true;
    }
    Phase phase_ = kIdle;
    Addr32 want_ = 0;
    std::string reason_;
    uint64_t t0_ = 0, tPhase_ = 0, tClick_ = 0, tClose_ = 0;
    int cycles_ = 0, closeCalls_ = 0, queueBase_ = 0;
    float hit_ = 0.0f, ap_ = 0.0f;
    bool sawPlayback_ = false, tapped_ = false, armed_ = false, queued_ = false;
};

}  // namespace nvbots
