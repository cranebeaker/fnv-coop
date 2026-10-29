// JSON-RPC surface of the bot driver (T6a), game-free: everything engine-specific is behind IBotEngine, so tests drive the
// real parsing and input logic with a mock. plugin/bots/bots_module.cpp supplies the live engine and registers these methods
// (dev only, nvcoop.ini [dev] bots=1).
//
//   bot.status                                  -> {enabled, hooked, frame, held:[names], shotsLeft, alwaysRun}
//   bot.move {forward,strafe,run?,ms?}          -> {held:[...], run, runControl, alwaysRun}   axes in [-1,1]; ms>0 auto-stops, else until bot.stop
//                                                  run = the SPEED (true: run speed, false: walk), whatever the always-run toggle is.
//                                                  The driver holds Run (9) only when run != always-run, and follows a toggle change
//                                                  while the move lasts. alwaysRun: true/false = the engine's state, null = unknown
//                                                  (then Run is held for run:true, the key meaning).
//   bot.stop                                    -> {}             release every forced control
//   bot.look {yaw?,pitch?} | {ref}              -> {yaw,pitch}    degrees, same convention as player.SetAngle Z/X; ref = look at it
//   bot.fire {shots?=1,gapMs?=180,holdMs?=90}   -> {queued}       Attack pulses through the input driver (the real attack path)
//   bot.reload                                  -> {}             taps the Ready/Reload control
//   bot.sneak                                   -> {}             taps Crouch (a toggle: one edge)
//   bot.jump / bot.activate                     -> {}             tap Jump / Activate (activate acts on the crosshair target)
//   bot.input {hold?:[n],release?:[n],tap?:[n],frames?} -> {held}   generic, names or ids, e.g. "Forward", "Vats"
//   bot.menu                                    -> {kind,id,vatsMode,dialogState?,dialogReady?}  current menu / VATS / dialogue state
//   bot.menu.press {button}                     -> {menu,button,path}  answer the message box our ops layer is showing (consent, PNR confirm),
//                                                  path "ui"; when it shows none and the engine's own MessageMenu (1001) is the top menu, press that
//                                                  box's button natively: path "native", plus {value,buttons} (see pressNativeMessageButton)
//   bot.menu.close {kind?="container"}          -> {closed,menu,kind}  close the open menu the engine's own way (CloseAllMenusCore), only when the top menu is that
//                                                  kind ("any" = every kind a player could cancel out of, never a message box); gone from bot.menu a frame later
//   bot.consent {answer:"ready"|"notyet"}       -> {menu,button}  answer the party-consent prompt (buttons: Ready=0, Not yet=1)
//   bot.inventory {ref?,items?:[form]}          -> {items:[{item,count}],partial}  totals via the engine's own counting
//   bot.say {info}                              -> {said,index,nextState}  choose that INFO in the open DialogMenu (state must be ready)
//   bot.dialog                                  -> {choices:[{idx,info}]}  what the open dialogue offers (moves the selection like a click)
//   bot.vats {ref,limb?} / test.vats.hit        -> {started}      tick-driven VATS job (VatsJob in bot_engine.h); progress in bot.status.vats
//                                                  {phase,reason}, phase idle|enter|tutorial|zoom|target|queue|execute|playback|done|failed
//   test.fasttravel {marker}                    -> {genuine:false} MoveTo the marker (a teleport that crosses cells; bot.fasttravel is the genuine one). Needs env NVCOOP_TEST_HELPERS=1.
//   bot.fasttravel {marker}                     -> {genuine:true,queued}  the map menu's own travel path (see fastTravel in bot_engine.h); the move happens a few frames later
//   bot.drops {since?,base?,windowMs?}          -> {seq,drops:[{seq,ref,base,dropper}]}  what the engine's OnDrop event reported (native handler); seq is the log position
//   dev.refsOf {base,cell?}                     -> {refs:[..],count}  the loaded, enabled refs of that base form (fallback to find a dropped ref)
//   dev.terminal {ref,entry}                     -> {entries,used,index}  click that entry of the open terminal menu (recorded)
//   dev.terminalState {ref}                     -> {used:[{index,text}]}  what this bot picked on that terminal (engine has no used flag)
//   dev.countRefs {base,cell?}                  -> {count}        loaded, enabled refs of that base form in the cell (default the player's)
//   bot.questvar {quest,var}                    -> {quest,var,value}  the live value of a quest script variable on this copy, by name (nvhooks::questVarValue); the lab's
//                                                  "the client sees the quest variable" check reads it on both copies
#pragma once
#include <cmath>
#include <cstdio>
#include <map>
#include <cstdint>
#include <string>
#include <vector>

#include "common/minijson.h"
#include "bot_engine.h"
#include "input_core.h"

namespace nvbots {

using nvcoop::json::Value;

struct IBotEngine {
    virtual ~IBotEngine() = default;
    virtual uint64_t nowMs() = 0;
    virtual bool exec(const std::string& consoleLine) = 0;                              // console line on the player
    virtual bool pos(const std::string& ref, double* x, double* y, double* z) = 0;      // "" = the player
    virtual bool hooked() = 0;                                                          // is the input detour installed
    // Optional engine glue; the defaults say "not wired" so the caller gets a clear error.
    virtual IEngine* engine() { return nullptr; }
    virtual bool pressBox(int button, uint32_t* menuId, std::string* err) { (void)button; (void)menuId; *err = "unsupported: no message box answer path"; return false; }
    virtual bool moveTo(const std::string& ref, std::string* err) { (void)ref; *err = "unsupported: moveTo not wired"; return false; }
    // The engine's always-run state (alwaysRunState in bot_engine.h): 1 on, 0 off or ignored, -1 unknown.
    virtual int alwaysRun() { return -1; }
    // test.* methods that change the world outside the game's rules (test.fasttravel teleports) run only when the game was started with the
    // env NVCOOP_TEST_HELPERS=1, like the other test helpers of the repo. The default (tests, mocks) is on.
    virtual bool testHelpers() { return true; }
};

// Runtime twins (FNVCOOP_MUT=<n>) for this file, every one must make tests/test_bots.cpp fail:
//  native message box press (33, 36, 40; the others of 33..41 are in bot_engine.h):
//  33 native path never used    36 native path tried before the UI layer    40 native reply says path "ui"
//  test.fasttravel gate (64): the env gate is skipped (a teleport helper is open on a normal game)
//  always-run (42, 43, 46; 44 and 45 are in bot_engine.h):
//  42 bot.move holds Run for run:true whatever the always-run toggle is (the behaviour the first in-game run measured)
//  43 a toggle change during a move is not followed     46 an unknown always-run state is treated as on

// The Run control (9) to hold so that the player moves at the wanted speed. alwaysRun: 1 on, 0 off, -1 unknown.
inline bool runControlFor(bool wantRun, int alwaysRun) {
    if (FNV_MUT(42)) return wantRun;
    if (FNV_MUT(46)) {
        if (alwaysRun < 0) return !wantRun;
        return wantRun != (alwaysRun == 1);
    }
    if (alwaysRun < 0) return wantRun;   // unknown: the key meaning, and the replies say alwaysRun:null
    return wantRun != (alwaysRun == 1);
}
inline Value alwaysRunValue(int alwaysRun) { return alwaysRun < 0 ? Value() : Value(alwaysRun == 1); }

inline const char* const kCtrlNames[kCount] = {
    "Forward", "Backward", "Left", "Right", "Attack", "Activate", "Block", "ReadyItem", "Crouch", "Run", "AlwaysRun", "AutoMove",
    "Jump", "TogglePOV", "MenuMode", "Rest", "Vats", "Hotkey1", "AmmoSwap", "Hotkey3", "Hotkey4", "Hotkey5", "Hotkey6", "Hotkey7",
    "Hotkey8", "QuickSave", "QuickLoad", "Grab"};

// Heading from `a` to `b` in degrees: Z = atan2(dx, dy) clockwise from +Y; X positive looks down. Same formula as tools/bots/bot.py.
inline void headingDeg(const double a[3], const double b[3], double* yaw, double* pitch) {
    const double dx = b[0] - a[0], dy = b[1] - a[1], dz = b[2] - a[2];
    const double rad = 180.0 / 3.14159265358979323846;
    double y = std::atan2(dx, dy) * rad;
    if (y < 0) y += 360.0;
    *yaw = y;
    if (FNV_MUT(9)) {
        *pitch = std::atan2(dz, std::sqrt(dx * dx + dy * dy)) * rad;
    } else {
        *pitch = -std::atan2(dz, std::sqrt(dx * dx + dy * dy)) * rad;
    }
}

class BotRpc {
public:
    BotRpc(IBotEngine& e, ForcedInput& in) : e_(e), in_(in) {}

    // Per game frame: timed stops and queued shots.
    void tick() {
        const uint64_t now = e_.nowMs();
        in_.advanceFrame();
        if (IEngine* g = e_.engine()) vats_.tick(*g, now, [this](int c) { in_.tap(c, 3); });
        if (!FNV_MUT(12)) {
            if (moveUntil_ && now >= moveUntil_) { in_.stopMove(); moveUntil_ = 0; moving_ = false; }
        }
        if (!FNV_MUT(43)) {
            if (moving_) followRun();
        }
        if (shotsLeft_ > 0 && now >= nextShotAt_) {
            in_.tap(kAttack, holdFrames_);
            --shotsLeft_;
            nextShotAt_ = now + gapMs_;
        }
    }
    int shotsLeft() const { return shotsLeft_; }
    const VatsJob& vats() const { return vats_; }
    DropLog& drops() { return drops_; }
    void setDropsLive(bool live) { dropsLive_ = live; }   // the module sets this once the OnDrop handler is registered

    bool handle(const std::string& method, const std::string& params, std::string& out) {
        Value p;
        std::string perr;
        if (!params.empty() && params != "null" && !nvcoop::json::parse(params, p, perr)) { out = "bad params: " + perr; return false; }
        Value o = Value::object();
        bool ok = true;
        if (method == "bot.status") {
            o.set("enabled", true).set("hooked", e_.hooked()).set("frame", in_.frame()).set("shotsLeft", (int64_t)shotsLeft_);
            o.set("held", heldList()).set("alwaysRun", alwaysRunValue(e_.alwaysRun()));
            Value v = Value::object();
            v.set("phase", VatsJob::name(vats_.phase())).set("reason", vats_.reason());
            o.set("vats", v);
        } else if (method == "bot.move") {
            const int64_t ms = p.get("ms").asInt(0);
            // `run` omitted: hold no Run key, so the game's own always-run toggle picks the speed (what bot.move did before
            // run meant the speed). Only an explicit run:true/false asks for a speed.
            speedGiven_ = !p.get("run").isNull();
            if (FNV_MUT(47)) speedGiven_ = true;   // twin 47: an omitted run counts as run:false and walks under always-run
            wantRun_ = p.get("run").asBool(false);
            const int alwaysRun = e_.alwaysRun();
            const bool runControl = speedGiven_ ? runControlFor(wantRun_, alwaysRun) : false;
            in_.setMove(p.get("forward").asNumber(0), p.get("strafe").asNumber(0), runControl);
            moving_ = in_.isHeld(kForward) || in_.isHeld(kBackward) || in_.isHeld(kLeft) || in_.isHeld(kRight);
            moveUntil_ = ms > 0 ? e_.nowMs() + (uint64_t)ms : 0;
            o.set("held", heldList()).set("run", wantRun_).set("runControl", runControl).set("alwaysRun", alwaysRunValue(alwaysRun));
        } else if (method == "bot.stop") {
            if (FNV_MUT(10)) {
                in_.stopMove();
            } else {
                in_.releaseAll();
            }
            shotsLeft_ = 0;
            moveUntil_ = 0;
            moving_ = false;
        } else if (method == "bot.look") {
            double yaw = 0, pitch = 0;
            if (p.find("ref")) {
                double a[3], b[3];
                if (!e_.pos("", &a[0], &a[1], &a[2])) { out = "player position unavailable"; return false; }
                if (!e_.pos(p.get("ref").asString(""), &b[0], &b[1], &b[2])) { out = "reference not loaded"; return false; }
                a[2] += p.get("eye").asNumber(120);
                b[2] += p.get("targetHeight").asNumber(60);
                headingDeg(a, b, &yaw, &pitch);
            } else {
                yaw = p.get("yaw").asNumber(0);
                pitch = p.get("pitch").asNumber(0);
            }
            char l1[64], l2[64];
            std::snprintf(l1, sizeof l1, "player.SetAngle Z %.2f", yaw);
            std::snprintf(l2, sizeof l2, "player.SetAngle X %.2f", pitch);
            if (!e_.exec(l1) || !e_.exec(l2)) { out = "SetAngle refused by the console"; return false; }
            o.set("yaw", yaw).set("pitch", pitch);
        } else if (method == "bot.fire") {
            int shots = (int)p.get("shots").asInt(1);
            if (shots < 1 || shots > 200) { out = "shots out of range"; return false; }
            if (FNV_MUT(11)) {
                ++shots;
            }
            gapMs_ = (uint64_t)p.get("gapMs").asInt(180);
            holdFrames_ = 3;
            shotsLeft_ = shots;
            nextShotAt_ = 0;
            o.set("queued", (int64_t)shots);
        } else if (method == "bot.reload") {
            in_.tap(kReadyItem, 3);
        } else if (method == "bot.sneak") {
            in_.tap(kCrouch, 3);
        } else if (method == "bot.jump") {
            in_.tap(kJump, 3);
        } else if (method == "bot.activate") {
            in_.tap(kActivate, 3);
        } else if (method == "bot.input") {
            const int frames = (int)p.get("frames").asInt(-1);
            for (const Value& v : p.get("hold").items()) { int c = ctrlOf(v); if (c < 0) { out = "unknown control"; return false; } in_.hold(c, frames); }
            for (const Value& v : p.get("release").items()) { int c = ctrlOf(v); if (c < 0) { out = "unknown control"; return false; } in_.release(c); }
            for (const Value& v : p.get("tap").items()) { int c = ctrlOf(v); if (c < 0) { out = "unknown control"; return false; } in_.tap(c, frames > 0 ? frames : 3); }
            o.set("held", heldList());
        } else if (method == "bot.menu") {
            IEngine* g = e_.engine();
            ok = g ? menuInfo(*g, &o, &out) : (out = "unsupported: engine glue not wired", false);
        } else if (method == "bot.menu.close") {
            IEngine* g = e_.engine();
            ok = g ? closeOpenMenu(*g, p.get("kind").asString(""), &o, &out) : (out = "unsupported: engine glue not wired", false);
        } else if (method == "bot.menu.press" || method == "bot.consent") {
            int button = (int)p.get("button").asInt(-1);
            if (method == "bot.consent") {
                const std::string a = p.get("answer").asString("");
                if (FNV_MUT(21)) {
                    button = a == "ready" ? 1 : a == "notyet" ? 0 : -1;
                } else {
                    button = a == "ready" ? 0 : a == "notyet" ? 1 : -1;
                }
            }
            if (button < 0 || button > 9) { out = "bad button/answer"; return false; }
            if (method == "bot.consent") {   // the party-consent prompt is always the UI layer's own box
                uint32_t menu = 0;
                ok = e_.pressBox(button, &menu, &out);
                if (ok) o.set("menu", (int64_t)menu).set("button", (int64_t)button);
            } else {
                ok = pressMenuButton(button, &o, &out);
            }
        } else if (method == "bot.inventory") {
            IEngine* g = e_.engine();
            std::vector<std::string> want;
            for (const Value& v : p.get("items").items()) want.push_back(v.asString(""));
            ok = g ? inventoryInfo(*g, p.get("ref").asString(""), want, &o, &out) : (out = "unsupported: engine glue not wired", false);
        } else if (method == "bot.say") {
            IEngine* g = e_.engine();
            uint32_t id = 0;
            if (!g) { out = "unsupported: engine glue not wired"; return false; }
            if (!g->resolveFormId(p.get("info").asString(""), &id)) { out = "unknown INFO " + p.get("info").asString(""); return false; }
            ok = dialogPick(*g, id, &o, &out);
            if (!ok && o.find("offered")) out += " " + o.get("offered").dump();
        } else if (method == "bot.dialog") {
            IEngine* g = e_.engine();
            std::vector<Choice> cs;
            if (!g) { out = "unsupported: engine glue not wired"; return false; }
            ok = dialogChoices(*g, &cs, &out);
            Value arr = Value::array();
            for (const Choice& c : cs) {
                Value v = Value::object();
                v.set("idx", (int64_t)c.idx).set("formId", (int64_t)c.formId).set("info", g->nameOfForm(c.info)).set("topicId", (int64_t)c.topicId);
                arr.push(v);
            }
            o.set("choices", arr);
        } else if (method == "bot.vats" || method == "test.vats.hit") {
            IEngine* g = e_.engine();
            if (!g) { out = "unsupported: engine glue not wired"; return false; }
            ok = vats_.start(*g, p.get("ref").asString(""), p.get("limb").asString(""), e_.nowMs(), &out);
            if (ok) o.set("started", true);
        } else if (method == "dev.terminal") {
            IEngine* g = e_.engine();
            if (!g) { out = "unsupported: engine glue not wired"; return false; }
            const std::string tref = p.get("ref").asString("");
            ok = terminalPick(*g, tref, (int)p.get("entry").asInt(-1), &o, &out);
            if (!FNV_MUT(25)) {
                if (ok) {
                    Value rec = Value::object();
                    rec.set("index", o.get("index")).set("text", o.get("used"));
                    termUsed_[tref].push_back(rec);
                }
            }
        } else if (method == "dev.terminalState") {
            Value arr = Value::array();
            for (const Value& v : termUsed_[p.get("ref").asString("")]) arr.push(v);
            o.set("used", arr);
        } else if (method == "bot.fasttravel") {
            IEngine* g = e_.engine();
            ok = g ? fastTravel(*g, p.get("marker").asString(""), &o, &out) : (out = "unsupported: engine glue not wired", false);
        } else if (method == "bot.drops") {
            if (!dropsLive_) { out = "unsupported: the OnDrop event handler is not registered"; return false; }
            o.set("seq", drops_.seq()).set("drops", drops_.query(p.get("since").asInt(0), p.get("base").asString(""), e_.nowMs(), (uint64_t)p.get("windowMs").asInt(5000)));
        } else if (method == "dev.refsOf") {
            IEngine* g = e_.engine();
            ok = g ? refsOf(*g, p.get("base").asString(""), p.get("cell").asString(""), &o, &out) : (out = "unsupported: engine glue not wired", false);
        } else if (method == "test.fasttravel") {
            if (!e_.testHelpers() && !FNV_MUT(64)) { out = "test helpers disabled (start the game with NVCOOP_TEST_HELPERS=1)"; return false; }
            ok = e_.moveTo(p.get("marker").asString(""), &out);
            if (ok) o.set("genuine", false);
        } else if (method == "bot.questvar") {
            IEngine* g = e_.engine();
            ok = g ? questVarInfo(*g, p.get("quest").asString(""), p.get("var").asString(""), &o, &out) : (out = "unsupported: engine glue not wired", false);
        } else if (method == "dev.countRefs") {
            IEngine* g = e_.engine();
            ok = g ? countRefs(*g, p.get("base").asString(""), p.get("cell").asString(""), &o, &out) : (out = "unsupported: engine glue not wired", false);
        } else {
            out = "unknown bot method";
            return false;
        }
        if (ok) out = o.dump();
        return ok;
    }

    static const char* const* names() { return kCtrlNames; }

private:
    // While a move lasts: the always-run toggle can change (a human key, bot.input tap AlwaysRun), so hold or release Run again.
    void followRun() {
        if (!speedGiven_) return;   // no speed asked: the game's toggle decides, and the bot holds no Run key
        const bool k = runControlFor(wantRun_, e_.alwaysRun());
        if (k == in_.isHeld(kRun)) return;
        if (k) in_.hold(kRun);
        else in_.release(kRun);
    }
    // bot.menu.press: the UI layer's box first (ui.answer). Only when it has none and the engine's own message box (MessageMenu, id 1001) is
    // the top menu, press that box natively (pressNativeMessageButton in bot_engine.h). The reply says which path answered.
    bool pressMenuButton(int button, Value* o, std::string* out) {
        IEngine* g = e_.engine();
        bool applies = false;
        std::string uiErr, nativeErr;
        if (FNV_MUT(36)) {
            if (g && pressNativeMessageButton(*g, button, o, &nativeErr, &applies)) { o->set("path", "native"); return true; }
        }
        uint32_t menu = 0;
        if (e_.pressBox(button, &menu, &uiErr)) {
            o->set("menu", (int64_t)menu).set("button", (int64_t)button).set("path", "ui");
            return true;
        }
        if (!FNV_MUT(33)) {
            if (g && pressNativeMessageButton(*g, button, o, &nativeErr, &applies)) {
                o->set("path", FNV_MUT(40) ? "ui" : "native");
                return true;
            }
            if (applies) { *out = "native message box: " + nativeErr + " (ui layer: " + uiErr + ")"; return false; }
        }
        *out = uiErr;
        return false;
    }
    Value heldList() const {
        Value a = Value::array();
        for (int c = 0; c < kCount; ++c)
            if (in_.isHeld(c)) a.push(kCtrlNames[c]);
        return a;
    }
    static int ctrlOf(const Value& v) {
        if (v.isNumber()) { int c = (int)v.asInt(); return (c >= 0 && c < kCount) ? c : -1; }
        for (int c = 0; c < kCount; ++c)
            if (v.asString() == kCtrlNames[c]) return c;
        return -1;
    }
    IBotEngine& e_;
    ForcedInput& in_;
    uint64_t moveUntil_ = 0, nextShotAt_ = 0, gapMs_ = 180;
    int shotsLeft_ = 0, holdFrames_ = 3;
    bool moving_ = false, wantRun_ = false, speedGiven_ = false;   // a bot.move with an axis held is active; the speed it asked for
    VatsJob vats_;
    DropLog drops_;
    bool dropsLive_ = false;
    std::map<std::string, std::vector<Value>> termUsed_;
};

}  // namespace nvbots
