// Synthetic game memory + call recorder implementing nvbots::IEngine for the game-free tests.
#pragma once
#include <cstring>
#include <map>
#include <string>
#include <vector>

#include "../bot_engine.h"

namespace nvbots_test {

using nvbots::Addr32;

struct FakeGame : nvbots::IEngine {
    std::map<Addr32, uint8_t> mem;
    std::vector<std::string> calls;
    uint32_t tick = 100000;
    Addr32 selectedItem = 0;
    uint32_t topId = 0;
    std::map<Addr32, bool> special;       // info -> special (flag 0x80, refused)
    std::map<Addr32, bool> speechCheck;   // info -> speech check / challenge (allowed)
    bool canTravel = true;
    Addr32 travelObj = 0x3B000000;
    std::map<Addr32, int> totals;         // form -> total count
    std::map<std::string, Addr32> forms;  // "Plugin.esm:HEX" -> form pointer
    std::map<std::string, uint32_t> formIds;
    Addr32 player = 0x40000000;
    int speakResult = 3;

    static constexpr Addr32 kDlg = 0x30000000, kLb = 0x31000000, kItems = 0x32000000, kInfos = 0x33000000, kMgr = 0x34000000,
                            kVmenu = 0x35000000, kData = 0x36000000, kCell = 0x37000000;

    void set32(Addr32 a, uint32_t v) { for (int i = 0; i < 4; ++i) mem[a + i] = uint8_t(v >> (8 * i)); }
    void set8(Addr32 a, uint8_t v) { mem[a] = v; }
    uint32_t get32(Addr32 a) { uint32_t v = 0; rd32(a, &v); return v; }

    bool rd32(Addr32 a, uint32_t* v) override {
        uint32_t r = 0;
        for (int i = 0; i < 4; ++i) {
            auto it = mem.find(a + i);
            if (it == mem.end()) { *v = 0; return false; }
            r |= uint32_t(it->second) << (8 * i);
        }
        *v = r;
        return true;
    }
    bool rd8(Addr32 a, uint8_t* v) override {
        auto it = mem.find(a);
        if (it == mem.end()) { *v = 0; return false; }
        *v = it->second;
        return true;
    }
    bool wr8(Addr32 a, uint8_t v) override { mem[a] = v; calls.push_back("wr8 " + hex(a) + "=" + std::to_string(v)); return true; }
    bool wr32(Addr32 a, uint32_t v) override { set32(a, v); calls.push_back("wr32 " + hex(a) + "=" + hex(v)); return true; }
    uint32_t nowTick() override { return tick; }

    // ---- dialogue: n choices with INFO form ids 0x1000+i
    int nItems = 0;
    void buildDialogue(int n, uint32_t state = 2) {
        nItems = n;
        set32(nvbots::eng::kDialogMenuPtr, kDlg);
        set32(kDlg + nvbots::eng::kDlgState, state);
        set32(kDlg + nvbots::eng::kDlgListBox, kLb);
        set32(kDlg + nvbots::eng::kDlgClickGate, 0);
        set32(kDlg + nvbots::eng::kDlgPending, 1);
        for (int i = 0; i < n; ++i) {
            const Addr32 item = kItems + i * 0x100, info = kInfos + i * 0x100;
            set32(item + nvbots::eng::kItemInfo, info);
            set8(item + nvbots::eng::kItemFlag, 7);
            set32(info + nvbots::eng::kFormId, 0x1000 + i);
        }
    }
    bool lbSelect(Addr32, int idx) override {
        calls.push_back("select " + std::to_string(idx));
        if (idx < 0 || idx >= nItems) return false;
        selectedItem = kItems + idx * 0x100;
        return true;
    }
    Addr32 lbSelected(Addr32) override { return selectedItem; }
    Addr32 itemInfo(Addr32 item) override { return get32(item + nvbots::eng::kItemInfo); }
    bool infoSpecial(Addr32 info) override { return special[info]; }
    bool infoSpeechCheck(Addr32 info) override { return speechCheck[info]; }
    void dlgItemClick(Addr32, Addr32 item) override { calls.push_back("click " + hex(item)); }
    void lbAfter(Addr32) override { calls.push_back("after"); }
    int dlgSpeak(Addr32, int flag) override { calls.push_back("speak " + std::to_string(flag)); return speakResult; }

    // ---- menus
    void buildMenus() { set32(nvbots::eng::kInterfaceMgrPtr, kMgr); set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, 0); }
    uint32_t topMenuId(Addr32) override { return topId; }
    void termClick(Addr32, Addr32 tile) override { calls.push_back("termclick " + hex(tile)); }
    // terminal menu with entries named by `names`; entry i has tile 0x3A000000+i*0x10
    static constexpr Addr32 kTerm = 0x3A000000;
    void buildTerminal(const std::vector<std::string>& names, Addr32 ref) {
        set32(nvbots::eng::kInterfaceMgrPtr, kMgr);
        set32(nvbots::eng::kTerminalMenuPtr, kTerm);
        set32(kTerm + nvbots::eng::kTermRef, ref);
        // the ListBox at menu+0x68 embeds its tList head node at +4 (data@+4, next@+8): the first item is that head's data, as measured in the game
        Addr32 node = kTerm + nvbots::eng::kTermList + 4;
        set32(node, 0);
        set32(node + 4, 0);
        for (size_t i = 0; i < names.size(); ++i) {
            const Addr32 item = kTerm + 0x2000 + Addr32(i) * 0x20, entry = kTerm + 0x3000 + Addr32(i) * 0x20, text = kTerm + 0x4000 + Addr32(i) * 0x40;
            set32(item, kTerm + 0x10 + Addr32(i) * 0x10); set32(item + 4, entry); set32(entry, text);
            for (size_t k = 0; k <= names[i].size(); ++k) set8(text + Addr32(k), k < names[i].size() ? (uint8_t)names[i][k] : 0);
            set32(node, item);
            const Addr32 next = i + 1 < names.size() ? kTerm + 0x1000 + Addr32(i + 1) * 0x10 : 0;
            set32(node + 4, next);
            node = next;
        }
    }

    // ---- native message box (MessageMenu, id 1001): GetMenuByType chain, button ListBox at menu+0x40 (head node at +0x44), one current
    // message in the queue. values[i] is the engine value of the i-th listed button; its tile is msgTile(i). The six SetField tiles at
    // menu+0x28..0x3C are decoys (kMsgDecoy + i*0x10): a press must never pass them.
    static constexpr Addr32 kTileMenus = 0x3D000000, kTileMenu = 0x3D100000, kMsgMenu = 0x3D200000, kMsgNodes = 0x3D300000, kMsgItems = 0x3D400000,
                            kMsgTiles = 0x3D500000, kMsgDecoy = 0x3D600000;
    bool mainThread = true;
    bool clickOk = true;
    static Addr32 msgTile(size_t i) { return kMsgTiles + Addr32(i) * 0x10; }
    void buildMessageBox(const std::vector<int32_t>& values, uint32_t vtbl = nvbots::addr::MessageMenuVtbl) {
        set32(nvbots::eng::kInterfaceMgrPtr, kMgr);
        topId = nvbots::eng::kMenuIdMessage;
        set32(nvbots::eng::kTileMenuArrayData, kTileMenus);
        set32(kTileMenus + 4 * (nvbots::eng::kMenuIdMessage - nvbots::eng::kMenuIdMin), kTileMenu);
        set32(kTileMenu + nvbots::eng::kTileMenuMenu, kMsgMenu);
        set32(kMsgMenu, vtbl);
        for (Addr32 i = 0; i < 6; ++i) set32(kMsgMenu + 0x28 + 4 * i, kMsgDecoy + i * 0x10);
        set32(kMsgMenu + 0x40, 0x107286C);   // ListBox<int> vtable
        Addr32 node = kMsgMenu + nvbots::eng::kMsgButtonHead;
        set32(node, 0);
        set32(node + 4, 0);
        for (size_t i = 0; i < values.size(); ++i) {
            const Addr32 item = kMsgItems + Addr32(i) * 0x10;
            set32(item, msgTile(i));
            set32(item + 4, uint32_t(values[i]));
            set32(node, item);
            const Addr32 next = i + 1 < values.size() ? kMsgNodes + Addr32(i) * 0x10 : 0;
            set32(node + 4, next);
            node = next;
        }
        set32(nvbots::eng::kMsgQueueIdx, 0);   // message 0 of 1 is current
        set32(nvbots::eng::kMsgQueue + 4, 0x3D700000);
        set32(nvbots::eng::kMsgQueue + 8, 1);
    }
    bool onMainThread() override { return mainThread; }
    bool msgMenuHandleClick(Addr32 menu, int id, Addr32 tile) override {
        calls.push_back("msgclick " + hex(menu) + " " + std::to_string(id) + " " + hex(tile));
        return clickOk;
    }

    // ---- bot.menu.close: CloseAllMenusCore empties the menu stack (the top menu id reads 0 afterwards)
    bool closeOk = true;
    int closeAllCalls = 0;
    bool closeAllMenus() override {
        ++closeAllCalls;
        calls.push_back("closeallmenus");
        if (!closeOk) return false;
        topId = 0;
        return true;
    }

    // ---- bot.vats: the engine side. vatsFrame() plays one game frame: the camera (enter, zoom 1 -> 3 -> 2, playback 4 -> 0) and
    // VATSMenu::Update (returns at once while TutorialMenu is open; asks the attack-button test, which the live detour answers from the gate).
    static constexpr Addr32 kVatsPart = 0x35100000, kTutorialTile = 0x35200000;
    nvbots::VatsAcceptGate gate;     // what the live detour holds
    bool acceptHook = true;          // the attack-button detour is installed
    bool tutorialOpen = false, tutorialCloses = true;
    int pickFrames = 0;              // frames in which the mouse pick 0x7F6AF0 ends Update before the queue path
    int zoomFrames = 3;              // frames of mode 1, then frames of mode 3, then mode 2
    int playbackFrames = 4;
    bool undoAfterQueue = false;     // the engine removes the queued attack on the next frame (an undo)
    float apCost = 20.0f;
    std::vector<Addr32> targets;     // the VATS target list in HandleClick(3) order; targets[0] is selected on entry
    size_t targetIdx = 0;
    int queue = 0, zoomLeft = 0, playLeft = 0;
    bool undoPending = false;
    std::vector<Addr32> queuedTargets;
    int execCalls = 0, execEmpty = 0, closeCalls = 0, clicks = 0, arms = 0, wrongForces = 0;

    void setf(Addr32 a, float f) { uint32_t v = 0; std::memcpy(&v, &f, 4); set32(a, v); }
    float getf(Addr32 a) { uint32_t v = get32(a); float f = 0; std::memcpy(&f, &v, 4); return f; }
    uint32_t vatsMode() { return get32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode); }
    // Literal addresses from the RE (not the eng:: constants), so a wrong constant cannot pass its own test.
    void buildVats(const std::vector<Addr32>& tg, float hit = 55.0f, float ap = 60.0f, bool tutorial = false, uint32_t vtbl = 0x10700D4) {
        buildMenus();
        targets = tg;
        targetIdx = 0;
        set32(0x11DB0D4, kVmenu);
        set32(kVmenu, vtbl);
        set32(kVmenu + 0x100, kVatsPart);
        setf(kVatsPart + 0x28, hit);
        setf(kVmenu + 0xE0, ap);
        set32(0x11F21CC, 0);
        tutorialOpen = tutorial;
    }
    void vatsFrame(bool vatsPressed) {
        uint32_t mode = vatsMode();
        if (mode == 0) {
            if (vatsPressed && !targets.empty()) {
                mode = 1;
                zoomLeft = 2 * zoomFrames;
                topId = tutorialOpen ? 0x423 : 0x420;
                targetIdx = 0;
                set32(0x11F21CC, targets[0]);
            }
        } else if (mode == 1 || mode == 3) {
            mode = zoomLeft > zoomFrames ? 1 : zoomLeft > 0 ? 3 : 2;
            if (zoomLeft > 0) --zoomLeft;
        } else if (mode == 4) {
            if (playLeft-- <= 0) { mode = 0; queue = 0; topId = 0; }
        }
        set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, mode);
        if (mode == 2 && !tutorialOpen) vatsUpdate();
    }
    void vatsUpdate() {
        if (undoPending) { undoPending = false; if (queue > 0) --queue; }
        if (topId != 0x420) return;
        const bool p1 = gate.force(1, 0, queue);   // Update asks slot 1 first, then 2 and 3
        if (!p1 && (gate.force(2, 0, queue) || gate.force(3, 0, queue))) ++wrongForces;
        if (!p1) return;
        if (pickFrames > 0) { --pickFrames; return; }
        const float hit = getf(kVatsPart + 0x28), ap = getf(kVmenu + 0xE0);
        if (!(hit > 0.0f) || ap < apCost) return;   // the engine plays "UIVATSInsufficientAP" and queues nothing
        ++queue;
        setf(kVmenu + 0xE0, ap - apCost);
        queuedTargets.push_back(get32(0x11F21CC));
        if (undoAfterQueue) undoPending = true;
    }
    Addr32 menuByType(uint32_t id) override {
        if (id == 0x423) return tutorialOpen ? kTutorialTile : 0;
        if (id == 0x420) return vatsMode() ? kVmenu : 0;
        return 0;
    }
    bool closeTutorial() override {
        ++closeCalls;
        if (closeMode < 0) closeMode = static_cast<int>(vatsMode());   // the camera mode at the first close
        calls.push_back("closetutorial");
        if (tutorialCloses && tutorialOpen) { tutorialOpen = false; if (vatsMode()) topId = 0x420; }
        return true;
    }
    bool vatsMenuClick(Addr32 menu, int id) override {
        ++clicks;
        calls.push_back("vatsclick " + hex(menu) + " " + std::to_string(id));
        if (!clickOk) return false;
        if (id == 3 && !targets.empty()) { targetIdx = (targetIdx + 1) % targets.size(); set32(0x11F21CC, targets[targetIdx]); }
        return true;
    }
    int vatsQueueCount() override { return queue; }
    int closeMode = -1;              // VATS camera mode when the tutorial box was first closed
    int ignoreExecs = 0;             // the first N accepts start nothing (measured 2026-10-27: the accept in the frame after the queue did nothing)
    bool boxOverAccept = false;      // the first accept is swallowed by a tutorial box that opens over the VATS menu
    bool vatsExecute() override {
        ++execCalls;
        calls.push_back("vatsexec " + std::to_string(queue));
        if (tutorialOpen) return true;                       // VATSMenu is behind the box: nothing happens
        if (ignoreExecs > 0) { --ignoreExecs; return true; }
        if (boxOverAccept && queue > 0) { boxOverAccept = false; tutorialOpen = true; topId = 0x423; return true; }
        if (queue == 0) { ++execEmpty; set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, 0); topId = 0; }
        else { set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, 4); playLeft = playbackFrames; }
        return true;
    }
    bool vatsArmAccept(bool on) override {
        if (!acceptHook) return false;
        calls.push_back(on ? "arm" : "disarm");
        if (on) { ++arms; gate.arm(queue); } else { gate.disarm(); }
        return true;
    }

    // ---- inventory: entries {type, delta}
    void buildInventory(const std::vector<std::pair<Addr32, int>>& entries) {
        set32(player + 0x1000, kData);                               // containerData(player) result lives here
        Addr32 node = kData + 0x100;
        set32(kData, node);
        for (size_t i = 0; i < entries.size(); ++i) {
            const Addr32 entry = kData + 0x1000 + Addr32(i) * 0x20;
            set32(entry, 0); set32(entry + 4, uint32_t(entries[i].second)); set32(entry + 8, entries[i].first);
            set32(node, entry);
            const Addr32 next = i + 1 < entries.size() ? node + 0x10 : 0;
            set32(node + 4, next);
            node = next;
        }
        if (entries.empty()) { set32(kData + 0x100, 0); set32(kData + 0x104, 0); }
    }
    Addr32 containerData(Addr32 ref) override { return ref == player ? kData : 0; }
    int itemTotal(Addr32, Addr32 form) override { return totals[form]; }

    // ---- fast travel
    bool canFastTravel(Addr32) override { calls.push_back("cantravel"); return canTravel; }
    Addr32 newTravelObj() override { calls.push_back("newtravel"); return travelObj; }
    void setPendingTravel(Addr32 p, Addr32 obj) override { calls.push_back("pending " + hex(obj)); set32(p + nvbots::addr::kPlayerPending, obj); }

    // ---- always-run: the bytes the engine's run decision reads. Literal addresses from the RE (bots_addrs.h kRunPins), not the eng::
    // constants, so a wrong constant cannot pass its own test. player+0x652 (the auto-move byte next to the toggle) stays 0 like in game.
    void setAlwaysRun(bool on, bool padConnected = false, bool padDisabled = false) {
        set8(player + 0x651, on ? 1 : 0);
        if (!mem.count(player + 0x652)) set8(player + 0x652, 0);
        set8(0x11D8A84, padConnected ? 1 : 0);
        set8(0x11D8C50, padDisabled ? 1 : 0);
    }

    // ---- references
    bool resolveRef(const std::string& r, Addr32* out) override {
        if (r.empty()) { *out = player; return true; }
        auto it = forms.find(r);
        if (it == forms.end()) return false;
        *out = it->second;
        return true;
    }
    bool resolveForm(const std::string& f, Addr32* out) override { return resolveRef(f, out) && !f.empty(); }
    std::map<std::string, double> questVars;                                  // "<quest runtime id hex>/<var>" -> value (bot.questvar)
    bool questVar(uint32_t quest, const std::string& name, double* out) override {
        char b[16];
        std::snprintf(b, sizeof b, "%X/", quest);
        auto it = questVars.find(std::string(b) + name);
        if (it == questVars.end()) return false;
        *out = it->second;
        return true;
    }
    bool resolveFormId(const std::string& f, uint32_t* out) override {
        auto it = formIds.find(f);
        if (it == formIds.end()) return false;
        *out = it->second;
        return true;
    }
    std::string nameOfForm(Addr32 form) override { return "Form:" + hex(form); }

    static std::string hex(uint32_t v) { char b[16]; std::snprintf(b, sizeof b, "%X", v); return b; }
};

}  // namespace nvbots_test
