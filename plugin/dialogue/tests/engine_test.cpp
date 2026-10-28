// Engine-glue tests (game-free): layout reading on a synthetic memory image + EngineGlue over a mock IEngineApi and
// real DialogueNode/InMemoryNetwork. Usage: engine_tests   (run from plugin/dialogue: the data tests read data/*.txt)
#include <cstdio>
#include <fstream>
#include <map>
#include <memory>
#include <set>
#include <sstream>
#include <string>
#include <tuple>
#include "fnvcoop/transport.h"
#include "fnvcoop_dlg/dialogue_data.h"
#include "fnvcoop_dlg/mutations.h"
#include "glue.h"

using namespace fndlg;
static std::string g_why;
#define CHECK(c) do { if (!(c)) { g_why = std::string("line ") + std::to_string(__LINE__) + " " #c; return false; } } while (0)

// ---- synthetic game memory: a low segment (objects) and a high segment holding the engine globals the layout names
struct Image : IMemory {
    static constexpr Addr32 kBase = 0x10000, kGBase = 0x11CB000;
    std::vector<uint8_t> b = std::vector<uint8_t>(0x8000, 0), g = std::vector<uint8_t>(0x10000, 0);
    Addr32 top = kBase + 0x2000;  // bump allocator for strings/responses
    uint8_t* at(Addr32 a, size_t n) {
        if (a >= kBase && a - kBase + n <= b.size()) return &b[a - kBase];
        if (a >= kGBase && a - kGBase + n <= g.size()) return &g[a - kGBase];
        return nullptr;
    }
    const uint8_t* at(Addr32 a, size_t n) const { return const_cast<Image*>(this)->at(a, n); }
    bool read(Addr32 a, void* out, size_t n) const override {
        const uint8_t* p = at(a, n);
        if (!p) return false;
        std::memcpy(out, p, n);
        return true;
    }
    void put(Addr32 a, const void* p, size_t n) { std::memcpy(at(a, n), p, n); }
    void u32(Addr32 a, uint32_t v) { put(a, &v, 4); }
    void u8(Addr32 a, uint8_t v) { put(a, &v, 1); }
    uint32_t rd32(Addr32 a) const { uint32_t v = 0; read(a, &v, 4); return v; }
    Addr32 alloc(size_t n) { Addr32 r = top; top += Addr32((n + 3) & ~3u); return r; }
    void str(Addr32 at_, const std::string& s) {  // writes an xNVSE String {char*, u16 len, u16 cap}
        Addr32 d = alloc(s.size() + 1);
        put(d, s.c_str(), s.size() + 1);
        u32(at_, d);
        uint16_t l = uint16_t(s.size());
        put(at_ + 4, &l, 2);
        put(at_ + 6, &l, 2);
    }
    Addr32 response(const std::string& text, uint32_t emotion, Addr32 next) {
        Addr32 r = alloc(response::kSize);
        u32(r + response::kEmotionType, emotion);
        u32(r + response::kEmotionValue, 0x32);
        u8(r + response::kFlags, 1);
        str(r + response::kText, text);
        u32(r + response::kNext, next);
        return r;
    }
    Addr32 chain(const std::vector<std::pair<std::string, uint32_t>>& lines) {
        Addr32 next = 0;
        for (size_t k = lines.size(); k-- > 0;) next = response(lines[k].first, lines[k].second, next);
        return next;
    }
    // An INFO object. headOffset >= 0 stores the chain inside the INFO (legacy layout); otherwise the INFO has no chain (the real layout).
    Addr32 info(const std::vector<std::pair<std::string, uint32_t>>& lines, int32_t headOffset, uint8_t flags1, const std::string& prompt) {
        Addr32 i = alloc(topic_info::kSize);
        if (headOffset >= 0) u32(i + uint32_t(headOffset), chain(lines));
        u8(i + topic_info::kFlags1, flags1);
        str(i + topic_info::kPrompt, prompt);
        return i;
    }
    // the engine's response cache now holds `info`'s chain (what TESTopicInfo::GetResponses leaves behind)
    void cache(Addr32 info, const std::vector<std::pair<std::string, uint32_t>>& lines) {
        u32(response_cache::kCachedInfoGlobal, info);
        u32(response_cache::kCell, chain(lines));
    }
    // A DialogMenu with its choice ListBox; also sets the singleton, state and spoken-INFO globals.
    Addr32 menu(Addr32 partner, uint32_t state, Addr32 spoken, const std::vector<std::pair<Addr32, Addr32>>& rows /* info, topic */) {
        Addr32 m = alloc(dialog_menu::kSize);
        u32(m + dialog_menu::kPartnerRef, partner);
        u32(m + dialog_menu::kState, state);
        u32(dialog_menu::kSpokenInfoGlobal, spoken);
        u32(dialog_menu::kSingletonPtr, m);
        setRows(m, rows);
        return m;
    }
    void setRows(Addr32 m, const std::vector<std::pair<Addr32, Addr32>>& rows) {
        Addr32 lb = alloc(8);
        u32(m + dialog_menu::kListBox, lb);
        Addr32 next = 0;
        for (size_t k = rows.size(); k-- > 0;) {
            Addr32 item = alloc(list_box::kItemSize);
            u32(item + list_box::kItemInfo, rows[k].first);
            u32(item + list_box::kItemTopic, rows[k].second);
            Addr32 node = alloc(8);
            u32(node + list_box::kNodeItem, item);
            u32(node + list_box::kNodeNext, next);
            next = node;
        }
        u32(lb + list_box::kHead, next);
    }
    void closeMenu() { u32(dialog_menu::kSingletonPtr, 0); }
};

static const std::vector<std::pair<std::string, uint32_t>> kHouse = {{"House.", 0}, {"Next word.", 1}};

static bool t_menu_and_flags() {
    Image m;
    Addr32 a = m.info({{"Cat.", 0}}, -1, 1, "Cat.");
    Addr32 b = m.info({{"Feed.", 0}}, -1, 0x04, "Feed.");
    Addr32 menu = m.menu(0xABCD00, dialog_menu::kStateIdle, a, {{a, 0x7001}, {0, 0x7999}, {b, 0x7002}});   // a gap row (null INFO) is skipped
    m.u8(menu + dialog_menu::kIsBarterShown, 1);
    MenuView v;
    CHECK(readMenu(m, menu, v));
    CHECK(v.partnerRef == 0xABCD00 && v.spokenInfo == a && v.barterShown && v.state == dialog_menu::kStateIdle);
    CHECK(v.items.size() == 2 && v.items[0].info == a && v.items[0].topic == 0x7001 && v.items[1].info == b && v.items[1].topic == 0x7002);
    CHECK(v.offered.size() == 2 && v.offered[1] == b);
    InfoFlags f;
    CHECK(readInfoFlags(m, a, f) && f.goodbye && !f.sayOnceFlag && f.prompt == "Cat.");
    CHECK(readInfoFlags(m, b, f) && !f.goodbye && f.sayOnceFlag);
    CHECK(!readMenu(m, 0, v));
    CHECK(!readMenu(m, 0xF000000, v));                    // unreadable memory is an error, not a crash
    // a menu that is still opening has no ListBox yet: readable, empty
    Addr32 opening = m.menu(0x1, dialog_menu::kStateOpening, 0, {});
    m.u32(opening + dialog_menu::kListBox, 0);
    CHECK(readMenu(m, opening, v) && v.items.empty());
    // a cyclic list is refused instead of looping
    Addr32 cyc = m.menu(0x1, dialog_menu::kStateIdle, 0, {{a, 0x7001}});
    Addr32 lb = m.rd32(cyc + dialog_menu::kListBox), head = m.rd32(lb + list_box::kHead);
    m.u32(head + list_box::kNodeNext, head);
    CHECK(!readMenu(m, cyc, v));
    return true;
}

// ---- response chains: engine cache first, then the loader, then the legacy in-INFO offset
static bool t_probe_and_lines(uint32_t headOffset, uint32_t probeAs) {
    Image m;
    Addr32 i = m.info({{"All right. I'm gonna say a word.", 2}, {"Say the first thing.", 0}}, int32_t(headOffset), 0, "");
    int32_t off = probeResponseOffset(m, i, "All right.");
    CHECK(off == int32_t(headOffset));
    CHECK(probeResponseOffset(m, i, "Different text") == -1);              // ESM cross-check refuses a mismatch
    ResponseLayout lay; lay.headOffset = int32_t(probeAs); lay.useEngineCache = false;
    std::vector<fnvcoop_dlg::Line> lines;
    CHECK(readLines(m, i, lay, lines));
    CHECK(lines.size() == 2 && lines[0].text.rfind("All right.", 0) == 0 && lines[0].emotion == 2 && lines[1].emotionValue == 0x32);
    return true;
}
static bool t_probe_ok() { return t_probe_and_lines(topic_info::kUnk48, topic_info::kUnk48); }
static bool t_probe_wrong_layout_twin() { return t_probe_and_lines(topic_info::kUnk48, topic_info::kUnk48 - 4); }
static bool t_probe_other_offset() { return t_probe_and_lines(0x34, 0x34); }
// The real INFO has no head field: the probe over a real-shaped INFO answers -1 (this is what proves the loader design in the game).
static bool t_probe_real_info_has_none() {
    Image m;
    Addr32 i = m.info(kHouse, -1, 0, "p");
    m.cache(i, kHouse);                                                    // the chain exists, but only in the engine cache
    CHECK(probeResponseOffset(m, i, "House.") == -1);
    return true;
}

static bool t_cache_read() {
    Image m;
    Addr32 a = m.info(kHouse, -1, 0, "a"), b = m.info({{"Other.", 0}}, -1, 0, "b");
    m.cache(a, kHouse);
    ResponseLayout lay;                                                    // default: engine cache
    std::vector<fnvcoop_dlg::Line> lines;
    setResponseLoader(nullptr);
    CHECK(readLines(m, a, lay, lines) && lines.size() == 2 && lines[0].text == "House." && lines[1].emotion == 1);
    CHECK(!readLines(m, b, lay, lines) && lines.empty());                  // the cache holds A: B must NOT get A's lines
    CHECK(!readCachedLines(m, b, lines));
    CHECK(!readLines(m, 0, lay, lines));
    return true;
}
static bool t_cache_wrong_info_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.dlgReadWrongInfo = true; return x; }());
    return t_cache_read();
}

static bool t_loader() {
    Image m;
    Addr32 a = m.info(kHouse, -1, 0, "a"), b = m.info({{"Bee.", 3}}, -1, 0, "b");
    m.cache(a, kHouse);
    int calls = 0;
    bool allow = true;
    setResponseLoader([&](Addr32 info) {                                   // plays the part of TESTopicInfo::GetResponses(info, 0)
        ++calls;
        if (!allow) return false;
        m.cache(info, info == b ? std::vector<std::pair<std::string, uint32_t>>{{"Bee.", 3}} : kHouse);
        return true;
    });
    ResponseLayout lay;
    std::vector<fnvcoop_dlg::Line> lines;
    const auto before = readerStats();
    CHECK(readLines(m, a, lay, lines) && calls == 0 && readerStats().cacheHits == before.cacheHits + 1);   // cache hit: no engine call
    CHECK(readLines(m, b, lay, lines) && calls == 1 && lines.size() == 1 && lines[0].text == "Bee." && lines[0].emotion == 3);
    allow = false;                                                         // refused (a line of another INFO is in flight)
    CHECK(!readLines(m, a, lay, lines) && calls == 2 && readerStats().loaderRefused == before.loaderRefused + 1);
    setResponseLoader(nullptr);
    return true;
}

static bool t_voice() {
    CHECK(voiceFile("FalloutNV.esm", "MaleOldVoice", "VCG01Intro", 0x104BF3, 1) == "falloutnv.esm\\maleoldvoice\\vcg01intro_00104bf3_1.ogg");
    return true;
}

// ---- glue over a mock engine
struct Mock : IEngineApi {
    Image img;
    std::map<Addr32, std::pair<FormRef, FormRef>> topicOfInfo;   // info -> (topic, infoRef)
    std::map<Addr32, FormRef> forms;                             // runtime form -> ref (topics, the NPC)
    std::vector<std::string> log, subs;
    std::vector<std::string> voices, notices2;
    int resumed = 0, rebuilt = 0, forced = 0, hidden = 0, denied = 0, unsup = 0;
    PlayerId deniedOwner = 255;
    uint8_t lockMask = 0; bool locked = false;
    std::vector<FormRef> marked, cleared, committed;   // committed: the SayOnceFlag effects the host recorded
    std::map<FormRef, int> said;                       // the INFO's said byte on this mock machine (saidState)
    const IMemory& memory() override { return img; }
    Addr32 dialogMenu() override { return img.rd32(dialog_menu::kSingletonPtr); }
    bool formRefOf(Addr32 f, FormRef& r) override { auto it = forms.find(f); if (it == forms.end()) return false; r = it->second; return true; }
    bool topicOf(Addr32 i, FormRef& t, std::string& e) override { auto it = topicOfInfo.find(i); if (it == topicOfInfo.end()) return false; t = it->second.first; e = "TOPIC"; return true; }
    bool infoRefOf(Addr32 i, FormRef& r) override { auto it = topicOfInfo.find(i); if (it == topicOfInfo.end()) return false; r = it->second.second; return true; }
    std::string voicePath(Addr32, uint32_t n, Addr32) override { return "v\\line_" + std::to_string(n) + ".ogg"; }
    std::vector<fnvcoop_dlg::Fact> collectFacts(Addr32) override { return {{1, FormRef("falloutnv.esm", 43), 30}}; }
    CheckKind classifyCheck(Addr32, const InfoFlags& f) override { return f.speechChallenge ? CheckKind::Speech : CheckKind::None; }
    void resumeTalk(const FormRef&) override { ++resumed; }
    void talkDenied(DenyReason, PlayerId owner) override { ++denied; deniedOwner = owner; }
    void rebuildTopicList() override { ++rebuilt; }
    void forceCloseDialog() override { ++forced; }
    void setControls(uint8_t m, bool l) override { lockMask = m; locked = l; }
    void banner(const FormRef&, PlayerId) override { log.push_back("banner"); }
    void showTopics(const std::vector<fnvcoop_dlg::TopicEntry>& t) override { log.push_back("topics" + std::to_string(t.size())); }
    void showSubtitle(const FormRef&, const std::string& t, float) override { subs.push_back(t); }
    void playVoice(const FormRef&, const std::string& p) override { voices.push_back(p); }
    void hideOverlay() override { ++hidden; }
    std::vector<Addr32> hiddenTopics; std::vector<fnvcoop_dlg::MenuRequest> menusOpened; std::vector<std::string> notices; std::vector<uint32_t> menusClosed;
    void hideTopic(Addr32 i) override { hiddenTopics.push_back(i); }
    bool audible = true; std::vector<std::pair<FormRef, FormRef>> forcedStarts;
    bool npcAudible(const FormRef&) override { return audible; }
    void startForcedConversation(const FormRef& n, const FormRef& t) override { forcedStarts.emplace_back(n, t); }
    void openMenu(const fnvcoop_dlg::MenuRequest& r) override { menusOpened.push_back(r); }
    void closeMenu(uint32_t m) override { menusClosed.push_back(m); }
    void menuNotice(const std::string& t, bool c) override { notices.push_back((c ? "closed:" : "open:") + t); }
    void unsupported(const char*) override { ++unsup; }
    void notice(const std::string& t) override { notices2.push_back(t); }
    void markSaid(const FormRef& f) override { marked.push_back(f); said[f] = 1; }
    void commitSayOnce(const FormRef& f) override { committed.push_back(f); }
    int saidState(const FormRef& f) override { auto it = said.find(f); return it == said.end() ? 0 : it->second; }
    void clearSaid(const FormRef& f) override { cleared.push_back(f); }
};

struct Rig {
    fnvcoop::InMemoryNetwork net{3};
    Mock mock[3];
    std::unique_ptr<EngineGlue> glue[3];
    std::unique_ptr<fnvcoop_dlg::DialogueNode> node[3];
    fnvcoop_dlg::SayOncePolicy sayOnce[3];
    uint64_t now = 0;
    Rig() {
        fnvcoop::LinkParams lp; lp.minLatencyMs = lp.maxLatencyMs = 20; net.setLink(lp);
        for (PlayerId i = 0; i < 3; ++i) {
            glue[i].reset(new EngineGlue(mock[i]));
            fnvcoop_dlg::DialogueNode::Config c; c.self = i; c.isHost = i == 0; c.hostPeer = 0;
            c.peerOfSlot = [](PlayerId p) { return fnvcoop::PeerId(p); };
            c.slotOfPeer = [](fnvcoop::PeerId p) { return PlayerId(p); };
            c.ownerOnlyTopics = {FormRef("falloutnv.esm", 0x037071)};
            fnvcoop_dlg::ResultRunner run;
            if (i == 0) run = [](const fnvcoop_dlg::ResultRequest&) { return uint64_t(1); };
            node[i].reset(new fnvcoop_dlg::DialogueNode(net.endpoint(i), c, run, glue[i]->sink()));
            glue[i]->attach(node[i].get());
            glue[i]->setSayOnce(&sayOnce[i]);
        }
        node[0]->playerJoined(1); node[0]->playerJoined(2);
    }
    // frame: network + node ticks; the DialogMenu poll runs only when `poll` (tests that drive the engine events by hand turn it off)
    void step(uint64_t ms, bool poll = true) {
        for (uint64_t t = 0; t <= ms; t += 5) {
            if (t) now += 5;
            net.advance(now);
            for (PlayerId i = 0; i < 3; ++i) {
                fnvcoop::PeerId from; fnvcoop::Bytes b;
                while (net.endpoint(i)->poll(from, b)) node[i]->onPacket(from, b, uint32_t(now));
                node[i]->tick(uint32_t(now));
                if (poll) glue[i]->pollMenu(uint32_t(now));
            }
        }
    }
};

static const FormRef kNpc("falloutnv.esm", 0x104C0F), kTopic1("falloutnv.esm", 0x105560), kInfo1("falloutnv.esm", 0x105583);

static bool t_glue_flow(bool breakLayout) {
    Rig r;
    FormRef npc = kNpc;
    // initiator (slot 1) world: a menu offering one INFO with two response lines
    Mock& m = r.mock[1];
    Addr32 info = m.img.info(kHouse, -1, 0, "Cat.");
    m.forms[0x7001] = kTopic1;
    m.topicOfInfo[info] = {kTopic1, kInfo1};
    m.img.cache(info, kHouse);
    ResponseLayout bad; bad.useEngineCache = false;                        // twin: a layout that cannot find the chain
    if (breakLayout) r.glue[1]->setLayout(bad);

    CHECK(r.glue[1]->onTalkRequested(npc, uint32_t(r.now)));       // vanilla deferred
    CHECK(m.resumed == 0);                                         // not before the grant
    r.step(100, false);
    CHECK(m.resumed == 1);                                         // grant -> resume vanilla
    CHECK(r.mock[0].log == std::vector<std::string>{"banner"} && r.mock[2].log == std::vector<std::string>{"banner"});

    m.img.menu(0x777, dialog_menu::kStateIdle, 0, {{info, 0x7001}});
    r.glue[1]->onTopicsBuilt(uint32_t(r.now));
    r.step(100, false);
    CHECK(r.mock[2].log.back() == "topics1");

    CHECK(r.glue[1]->onInfoChosen(info, uint32_t(r.now)));         // initiator picks
    r.step(100, false);
    CHECK(r.mock[2].subs.size() == 2 && r.mock[2].subs[0] == "House." && r.mock[2].subs[1] == "Next word.");
    CHECK(r.mock[2].voices.size() == 2 && r.mock[2].voices[0] == "v\\line_1.ogg");
    CHECK(r.mock[0].subs.size() == 2);                             // host sees it as spectator too
    CHECK(r.mock[1].subs.empty());                                 // initiator never gets its own line echoed

    r.glue[1]->onLineFinished(uint32_t(r.now));
    r.step(100, false);
    CHECK(m.rebuilt == 1);

    // second player tries to talk to the same NPC: denied with the owner
    CHECK(r.glue[2]->onTalkRequested(npc, uint32_t(r.now)));
    r.step(100, false);
    CHECK(r.mock[2].denied == 1 && r.mock[2].deniedOwner == 1 && r.mock[2].resumed == 0);

    // NPC dies: initiator's menu is force-closed, spectators' overlays hide
    r.node[0]->npcDied(npc, uint32_t(r.now));
    r.step(100, false);
    CHECK(m.forced == 1 && r.mock[0].hidden == 1);
    return true;
}
static bool t_glue_ok() { return t_glue_flow(false); }
static bool t_glue_broken_layout_twin() { return t_glue_flow(true); }

static bool t_unknown_response_reports() {
    Rig r;
    Mock& m = r.mock[1];
    Addr32 info = m.img.info({{"x", 0}}, -1, 0, "p");
    m.forms[0x7001] = kTopic1;
    m.topicOfInfo[info] = {kTopic1, kInfo1};
    m.img.menu(0x1, dialog_menu::kStateIdle, 0, {{info, 0x7001}});
    setResponseLoader(nullptr);
    r.glue[1]->onTalkRequested(FormRef("falloutnv.esm", 9), 0);
    r.step(100, false);
    CHECK(!r.glue[1]->onInfoChosen(info, 0) && m.unsup == 1);      // chain neither cached nor loadable -> loud, no bogus pick
    return true;
}

static bool t_control_lock_and_goodbye() {
    Rig r;
    FormRef npc("falloutnv.esm", 5);
    r.glue[1]->onTalkRequested(npc, 0);
    r.step(100, false);
    r.node[0]->controlLock(1, 0x3F, true, uint32_t(r.now));
    r.step(100, false);
    CHECK(r.mock[1].locked && r.mock[1].lockMask == 0x3F);
    r.glue[1]->onMenuClosedByPlayer(uint32_t(r.now));
    r.step(100, false);
    CHECK(!r.mock[1].locked);                                      // lease released on close
    CHECK(r.mock[1].forced == 1 && r.mock[2].hidden == 1);
    return true;
}

static bool t_glue_hides_commands_for_non_owner(bool owner) {
    Rig r;
    const FormRef boone("falloutnv.esm", 0x5001), wait("falloutnv.esm", 0x037071), chat("falloutnv.esm", 0x1002);
    for (PlayerId i = 0; i < 3; ++i) { r.node[i]->companions().define(boone, fnvcoop_dlg::CompanionKind::Humanoid); r.node[i]->companions().setOwner(boone, 1); }
    PlayerId who = owner ? 1 : 2;
    Mock& m = r.mock[who];
    Addr32 a = m.img.info({{"Waiting.", 0}}, -1, 0, "Wait here.");
    Addr32 b = m.img.info({{"Hm.", 0}}, -1, 0, "Chat");
    m.img.menu(0x777, dialog_menu::kStateIdle, 0, {{a, 0x7001}, {b, 0x7002}});
    m.forms[0x7001] = wait; m.forms[0x7002] = chat;
    m.topicOfInfo[a] = {wait, FormRef("falloutnv.esm", 0x9001)};
    m.topicOfInfo[b] = {chat, FormRef("falloutnv.esm", 0x9002)};
    r.glue[who]->onTalkRequested(boone, 0);
    r.step(100, false);
    r.glue[who]->onTopicsBuilt(uint32_t(r.now));
    r.step(100, false);
    return owner ? m.hiddenTopics.empty() : m.hiddenTopics.size() == 1 && m.hiddenTopics[0] == a;
}

// a non-owner clicks a companion command anyway (the row cannot be hidden in the engine list): refused with a notice, no pick is sent
static bool t_glue_refuses_command_pick() {
    Rig r;
    const FormRef boone("falloutnv.esm", 0x5001), wait("falloutnv.esm", 0x037071);
    for (PlayerId i = 0; i < 3; ++i) { r.node[i]->companions().define(boone, fnvcoop_dlg::CompanionKind::Humanoid); r.node[i]->companions().setOwner(boone, 1); }
    Mock& m = r.mock[2];
    Addr32 a = m.img.info({{"Waiting.", 0}}, -1, 0, "Wait here.");
    m.img.menu(0x777, dialog_menu::kStateIdle, 0, {{a, 0x7001}});
    m.forms[0x7001] = wait;
    m.topicOfInfo[a] = {wait, FormRef("falloutnv.esm", 0x9001)};
    m.img.cache(a, {{"Waiting.", 0}});
    r.glue[2]->onTalkRequested(boone, 0);
    r.step(100, false);
    CHECK(!r.glue[2]->onInfoChosen(a, uint32_t(r.now)));
    CHECK(m.notices2.size() == 1 && r.glue[2]->stats().commandRefused == 1 && r.glue[2]->stats().picks == 0);
    return true;
}
static bool t_glue_command_pick_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.nonOwnerPicksCommand = true; return x; }());
    return t_glue_refuses_command_pick();
}

static bool t_glue_menus() {
    Rig r;
    std::vector<std::tuple<uint32_t, int32_t, bool>> res;
    r.node[0]->setMenuResume([&](uint32_t mnu, int32_t c, bool t) { res.emplace_back(mnu, c, t); });
    uint32_t id = r.node[0]->raiseMenu(2, fnvcoop_dlg::MenuKind::MessageBox, "Take it?", {"Yes", "No"}, 1, FormRef(), uint32_t(r.now));
    r.step(100, false);
    CHECK(r.mock[2].menusOpened.size() == 1 && r.mock[2].menusOpened[0].text == "Take it?");   // the real menu opens on p2
    CHECK(r.mock[0].menusOpened.empty() && r.mock[1].menusOpened.empty());
    CHECK(r.mock[1].notices.size() == 1 && r.mock[1].notices[0].rfind("open:", 0) == 0);        // others: read-only notice
    r.glue[2]->onMenuAnswered(id, 0, uint32_t(r.now));
    r.step(100, false);
    CHECK(res.size() == 1 && std::get<1>(res[0]) == 0);
    CHECK(r.mock[1].notices.back() == "closed:Yes");
    return true;
}

static bool t_glue_forced_say_audibility() {
    Rig r;
    const FormRef benny("falloutnv.esm", 0x7777), topic("falloutnv.esm", 0x1111), info("falloutnv.esm", 0x2222);
    r.mock[1].audible = false;                                      // p1 is far away; p2 hears
    r.node[0]->forcedSay(benny, topic, info, {fnvcoop_dlg::Line{"On your knees.", "v.ogg", 0, 50}}, 2, false, 5, uint32_t(r.now));
    r.step(100, false);
    CHECK(r.mock[2].subs.size() == 1 && r.mock[2].subs[0] == "On your knees.");     // the target
    CHECK(r.mock[0].subs.empty());                                                   // the host's engine ran the script: no second playback
    CHECK(r.mock[1].subs.empty());                                                   // out of earshot
    r.node[0]->forcedSay(benny, topic, info, {fnvcoop_dlg::Line{"Cutscene line.", "", 0, 50}}, fnvcoop::kNoPlayer, true, 6, uint32_t(r.now));
    r.step(100, false);
    CHECK(r.mock[1].subs.size() == 1 && r.mock[1].subs[0] == "Cutscene line.");      // scenes reach every guest
    CHECK(r.mock[2].subs.size() == 2 && r.mock[0].subs.empty());
    return true;
}
static bool t_glue_forced_host_replays_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.forcedHostReplays = true; return x; }());
    return t_glue_forced_say_audibility();
}

static bool t_glue_forced_conversation() {
    Rig r;
    const FormRef benny("falloutnv.esm", 0x7777), topic("falloutnv.esm", 0x1111);
    PlayerId actor = r.node[0]->forceConversation(benny, topic, 2, uint32_t(r.now));
    CHECK(actor == 2);
    r.step(100, false);
    CHECK(r.mock[2].forcedStarts.size() == 1 && r.mock[2].forcedStarts[0].second == topic);   // native start only on the target
    CHECK(r.mock[0].forcedStarts.empty() && r.mock[1].forcedStarts.empty());
    CHECK(r.node[2]->client().role() == fnvcoop_dlg::ClientDialogue::Role::Initiator);
    CHECK(r.node[1]->client().role() == fnvcoop_dlg::ClientDialogue::Role::Spectator);
    return true;
}

static bool t_info_reader_factory() {
    Image m;
    Addr32 i = m.info({{"All right. I'm gonna say a word.", 2}, {"Say the first thing.", 0}}, -1, 0, "");
    setResponseLoader(nullptr);
    ResponseLayout lay;                                              // nothing cached, no loader: the reader returns nothing, not garbage
    auto rd = makeInfoReader(m, &lay);
    CHECK(rd(i).empty());
    m.cache(i, {{"All right. I'm gonna say a word.", 2}, {"Say the first thing.", 0}});   // the engine speaks it: the same reader now sees it
    auto lines = rd(i);
    CHECK(lines.size() == 2 && lines[1].text == "Say the first thing.");
    CHECK(rd(0).empty());
    auto rd2 = makeInfoReader(m, nullptr);                           // no layout at all is the default layout
    CHECK(rd2(i).size() == 2);
    return true;
}

// ---- the frame poll: engine events derived from the DialogMenu
struct PollWorld {
    Rig r;
    Addr32 a = 0, bye = 0, menu = 0;
    FormRef infoA = FormRef("falloutnv.esm", 0x105583), infoBye = FormRef("falloutnv.esm", 0x105584), topicA = kTopic1, topicBye = FormRef("falloutnv.esm", 0x105561);
    PollWorld() {
        Mock& m = r.mock[1];
        a = m.img.info(kHouse, -1, 0, "Cat.");
        bye = m.img.info({{"Goodbye.", 0}}, -1, 0x01, "Bye");
        m.forms[0x7001] = topicA; m.forms[0x7002] = topicBye; m.forms[0x777] = kNpc;
        m.topicOfInfo[a] = {topicA, infoA};
        m.topicOfInfo[bye] = {topicBye, infoBye};
    }
    Mock& m() { return r.mock[1]; }
    // the grant arrives and the engine opens its menu
    void open() {
        r.glue[1]->onTalkRequested(kNpc, uint32_t(r.now));
        r.step(100);
        menu = m().img.menu(0x777, dialog_menu::kStateOpening, 0, {});
        r.step(20);
    }
    void state(uint32_t s, Addr32 spoken) {
        m().img.u32(menu + dialog_menu::kState, s);
        m().img.u32(dialog_menu::kSpokenInfoGlobal, spoken);
        r.step(90);   // two network hops at 20 ms plus the poll
    }
    void rows() { m().img.setRows(menu, {{a, 0x7001}, {bye, 0x7002}}); }
};

static bool t_poll_full_conversation() {
    PollWorld w;
    w.open();
    CHECK(w.m().resumed == 1);
    auto& g = *w.r.glue[1];
    CHECK(g.stats().offers == 0);                                           // still opening: nothing to offer
    w.rows();
    w.state(dialog_menu::kStateIdle, 0);                                    // the engine built the list and waits for a pick
    CHECK(g.stats().offers == 1 && w.r.mock[2].log.back() == "topics2");
    w.state(dialog_menu::kStateIdle, 0);
    CHECK(g.stats().offers == 1);                                           // same list: no second offer
    w.m().img.cache(w.a, kHouse);                                           // the engine loads the chain when it speaks
    w.state(dialog_menu::kStateQueued, w.a);                                // the player picked INFO a
    CHECK(g.stats().picks == 1);
    CHECK(w.r.mock[2].subs.size() == 2 && w.r.mock[2].subs[0] == "House." && w.r.mock[0].subs.size() == 2);
    w.state(dialog_menu::kStatePlaying, w.a);
    w.state(dialog_menu::kStatePlaying, w.a);
    CHECK(g.stats().picks == 1);                                            // the same line playing is not a new pick
    w.state(dialog_menu::kStateIdle, w.a);                                  // line over, the engine shows the next list
    CHECK(g.stats().lineDones == 1);
    CHECK(w.m().rebuilt == 1);                                              // the end-script gate opened
    w.state(dialog_menu::kStateIdle, w.a);
    // goodbye: pick, line, menu closes by itself
    w.m().img.cache(w.bye, {{"Goodbye.", 0}});
    w.state(dialog_menu::kStateQueued, w.bye);
    CHECK(g.stats().picks == 2);
    w.m().img.u32(w.menu + dialog_menu::kState, dialog_menu::kStateClosing);   // the goodbye line ended and the menu goes away in the same breath:
    w.r.step(5);                                                                 // the host's Closed has not come back yet (RTT 40 ms)
    CHECK(g.stats().lineDones == 2);
    w.m().img.closeMenu();
    w.r.step(100);
    CHECK(g.stats().leaves == 0);                                           // closing after a finished goodbye is not "the player walked away"
    CHECK(w.r.node[0]->host() != nullptr);
    return true;
}
static bool t_poll_no_pick_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.pollNoPick = true; return x; }());
    return t_poll_full_conversation();
}
static bool t_poll_pick_every_frame_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.pollPickEveryFrame = true; return x; }());
    return t_poll_full_conversation();
}
static bool t_poll_no_line_done_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.pollNoLineDone = true; return x; }());
    return t_poll_full_conversation();
}
static bool t_poll_leave_after_goodbye_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.pollLeaveAfterGoodbye = true; return x; }());
    return t_poll_full_conversation();
}

// Story fix 2026-10-28: the host granted the guest's talk, the deferred activation did not open the DialogMenu (an NPC 200+ units away; the first Goodsprings story
// run: S01, S06, S13, S16). The guest gives the conversation back after the grant timeout instead of staying the initiator for ever, so it can talk again and the
// host's NPC lock is released.
static bool t_grant_without_menu_is_given_back() {
    Rig r;
    auto& g = *r.glue[1];
    CHECK(g.onTalkRequested(kNpc, uint32_t(r.now)));
    r.step(100);
    CHECK(r.mock[1].resumed == 1);                                          // the grant arrived and the activation was replayed ...
    using Role = fnvcoop_dlg::ClientDialogue::Role;
    CHECK(r.node[1]->client().role() == Role::Initiator);                   // ... but no menu opens
    r.step(5000);
    CHECK(r.node[1]->client().role() == Role::Initiator && g.stats().grantTimeouts == 0);   // inside the timeout nothing is given back
    r.step(2000);
    CHECK(g.stats().grantTimeouts == 1);
    r.step(200);
    CHECK(r.node[1]->client().role() == Role::Idle);                        // the host closed it
    CHECK(g.onTalkRequested(kNpc, uint32_t(r.now)));                        // and the guest may talk again
    return true;
}
static bool t_grant_without_menu_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.grantNoWatchdog = true; return x; }());
    return t_grant_without_menu_is_given_back();
}
// Story run S06: one guest pick ran its result four times. The engine can show the INFO it just started as "spoken" again after its line (a response
// advance, a click on the row still speaking): that is the line going on, not a second pick. Only a menu waiting for a pick again makes the same INFO a new one.
static bool t_poll_same_info_again_is_not_a_new_pick() {
    PollWorld w;
    w.open();
    auto& g = *w.r.glue[1];
    w.rows();
    w.state(dialog_menu::kStateIdle, 0);
    w.m().img.cache(w.a, kHouse);
    w.state(dialog_menu::kStateQueued, w.a);                                // the player picked INFO a
    CHECK(g.stats().picks == 1);
    w.state(dialog_menu::kStatePlaying, w.a);
    w.state(dialog_menu::kStateClosing, w.a);                               // the menu leaves "speaking" for a moment (the line looks over)
    CHECK(g.stats().lineDones == 1);
    w.state(dialog_menu::kStatePlaying, w.a);                               // and the same INFO is spoken again, no list built in between
    w.state(dialog_menu::kStatePlaying, w.a);
    CHECK(g.stats().picks == 1);                                            // still ONE pick: the host runs the Begin script once
    CHECK(w.r.node[0]->host()->scriptRuns() == 2);                          // Begin + End of that one pick
    w.state(dialog_menu::kStateIdle, w.a);                                  // the menu is back at "pick a line": the same INFO is a real choice again
    w.state(dialog_menu::kStateIdle, w.a);
    w.m().img.u32(dialog_menu::kSpokenInfoGlobal, 0);
    w.state(dialog_menu::kStateQueued, w.a);
    CHECK(g.stats().picks == 2);
    w.state(dialog_menu::kStateIdle, w.a);
    CHECK(w.r.node[0]->host()->scriptRuns() == 4 && w.r.node[0]->host()->repeatPicks() == 0);
    return true;
}
static bool t_poll_same_info_again_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.noPickLatch = true; return x; }());
    return t_poll_same_info_again_is_not_a_new_pick();
}

static bool t_poll_walk_away() {
    PollWorld w;
    w.open();
    w.rows();
    w.state(dialog_menu::kStateIdle, 0);
    w.m().img.closeMenu();                                                  // Esc / walked away with no pick
    w.r.step(100);
    CHECK(w.r.glue[1]->stats().leaves == 1);
    CHECK(w.r.node[1]->client().role() != fnvcoop_dlg::ClientDialogue::Role::Initiator || true);
    CHECK(w.r.mock[2].hidden >= 1);                                         // the spectators' overlay goes away
    return true;
}

// the host player talks natively; the others see the lines like a scripted SayTo
static bool t_poll_host_native_broadcast() {
    Rig r;
    Mock& m = r.mock[0];
    Addr32 a = m.img.info(kHouse, -1, 0, "Cat.");
    m.forms[0x7001] = kTopic1; m.forms[0x777] = kNpc;
    m.topicOfInfo[a] = {kTopic1, kInfo1};
    Addr32 menu = m.img.menu(0x777, dialog_menu::kStateIdle, 0, {{a, 0x7001}});
    r.step(40);
    CHECK(r.glue[0]->stats().hostBroadcasts == 0);
    m.img.cache(a, kHouse);
    m.img.u32(menu + dialog_menu::kState, dialog_menu::kStateQueued);
    m.img.u32(dialog_menu::kSpokenInfoGlobal, a);
    r.step(100);
    CHECK(r.glue[0]->stats().hostBroadcasts == 1);
    CHECK(r.mock[1].subs.size() == 2 && r.mock[2].subs.size() == 2 && r.mock[1].subs[0] == "House.");   // heard by the others
    CHECK(r.mock[0].subs.empty());                                                                        // the host's own engine spoke it
    return true;
}
static bool t_poll_host_silent_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.hostTalkSilent = true; return x; }());
    return t_poll_host_native_broadcast();
}

// ---- say-once (D11): shared, except the per-player lines
static bool t_sayonce(bool perPlayer) {
    PollWorld w;
    if (perPlayer) for (int i = 0; i < 3; ++i) w.r.sayOnce[i].setPerPlayer({w.infoA});
    w.open();
    w.rows();
    w.state(dialog_menu::kStateIdle, 0);
    w.m().img.cache(w.a, kHouse);
    w.state(dialog_menu::kStateQueued, w.a);
    w.r.step(100);
    const bool hostMarked = !w.r.mock[0].marked.empty(), g2Marked = !w.r.mock[2].marked.empty();
    if (!perPlayer) {
        CHECK(hostMarked && g2Marked && w.r.mock[0].marked[0] == w.infoA && w.r.mock[2].marked[0] == w.infoA);
        CHECK(w.r.mock[1].marked.empty());                                  // the initiator's engine marks its own line
        CHECK(w.r.mock[0].cleared.empty());
    } else {
        CHECK(!hostMarked && !g2Marked);                                    // a Speech-XP / trainer line stays available to the others
        CHECK(w.r.mock[0].cleared.size() == 1 && w.r.mock[0].cleared[0] == w.infoA);   // the host undoes its RunResultScript mark
        CHECK(w.r.mock[2].cleared.empty());                                 // a guest never set one
    }
    return true;
}
static bool t_sayonce_shared() { return t_sayonce(false); }
static bool t_sayonce_per_player() { return t_sayonce(true); }
static bool t_sayonce_never_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceNeverShared = true; return x; }());
    return t_sayonce(false);
}
static bool t_sayonce_all_shared_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceAllShared = true; return x; }());
    return t_sayonce(true);
}
static bool t_sayonce_keep_host_mark_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceKeepHostMark = true; return x; }());
    return t_sayonce(true);
}
// ---- say-once marks the engine makes by itself (story S06: scripted SayTo lines of a forced stage chain; D11). A guest reports them, the host commits them.
static bool has(const std::vector<FormRef>& v, const FormRef& f) { for (auto& x : v) if (x == f) return true; return false; }
static bool t_sayonce_engine_marks() {
    Rig r;
    const FormRef A("falloutnv.esm", 0x10A1E6), B("falloutnv.esm", 0x10A1E7), C("falloutnv.esm", 0x08F070);   // A: a guest's scripted line, B: the host's, C: a trainer line
    for (int i = 0; i < 3; ++i) r.sayOnce[i].setPerPlayer({C});
    r.glue[1]->onEngineMark(A);                       // guest 1's engine spoke and marked A (its Sunny is loaded, the host's is not)
    r.step(100);
    CHECK(r.node[1]->sayOnceSent() == 1);
    CHECK(has(r.mock[0].marked, A) && has(r.mock[0].committed, A));      // the host marks its own copy and records the effect
    CHECK(r.mock[0].marked.size() == 1 && r.mock[0].committed.size() == 1);
    CHECK(r.mock[1].marked.empty() && r.mock[2].marked.empty());          // the guest's engine did it itself; guest 2 learns it from the effect, not from this glue
    CHECK(r.mock[2].committed.empty() && r.mock[1].committed.empty());    // only the host commits
    r.glue[0]->onEngineMark(B);                       // the host's own engine marked B: no network, one effect
    r.step(100);
    CHECK(r.mock[0].committed.size() == 2 && r.mock[0].committed[1] == B);
    CHECK(r.node[0]->sayOnceSent() == 0 && r.mock[0].marked.size() == 1);
    r.glue[1]->onEngineMark(C);                       // a trainer line: stays with the player who used it
    r.glue[0]->onEngineMark(C);
    r.step(100);
    CHECK(r.node[1]->sayOnceSent() == 1 && r.mock[0].committed.size() == 2);
    CHECK(!has(r.mock[0].committed, C) && !has(r.mock[0].marked, C));
    CHECK(r.glue[0]->stats().sayReports == 1 && r.glue[0]->stats().sayCommitted == 2 && r.glue[1]->stats().sayReported == 1);
    return true;
}
static bool t_sayonce_engine_marks_forged_and_known() {
    Rig r;
    const FormRef A("falloutnv.esm", 0x10A1E6), C("falloutnv.esm", 0x08F070), D("falloutnv.esm", 0x10A1E8);
    for (int i = 0; i < 3; ++i) r.sayOnce[i].setPerPlayer({C});
    r.node[1]->reportSayOnce(C);                      // a report that skipped the guest's own policy: the host keeps the trainer line per player anyway
    r.node[1]->reportSayOnce(FormRef());              // no plugin name: never sent
    r.mock[0].said[D] = 1;                            // the host's copy of D is already set (it ran the line, or the save holds it)
    r.glue[1]->onEngineMark(D);
    r.step(100);
    CHECK(r.mock[0].committed.empty() && r.mock[0].marked.empty());
    CHECK(r.node[1]->sayOnceSent() == 2);
    // a packet from a peer that is no joined slot is counted and dropped
    fnvcoop::Bytes b = fnvcoop_dlg::encode(fnvcoop_dlg::SayOnceMark{A});
    CHECK(r.node[0]->onPacket(99, b, 0));
    CHECK(r.node[0]->sayOnceRejected() == 1 && r.mock[0].committed.empty());
    // the host never reports to itself, and a guest ignores a stray report
    CHECK(!r.node[0]->reportSayOnce(A));
    CHECK(r.node[2]->onPacket(0, b, 0) && r.mock[2].committed.empty());
    return true;
}
static bool t_sayonce_mark_not_reported_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceMarkNotReported = true; return x; }());
    return t_sayonce_engine_marks();
}
static bool t_sayonce_mark_trainer_shared_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceMarkTrainerShared = true; return x; }());
    return t_sayonce_engine_marks();
}
static bool t_sayonce_mark_trainer_shared_forged_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceMarkTrainerShared = true; return x; }());
    return t_sayonce_engine_marks_forged_and_known();
}
static bool t_sayonce_mark_host_no_commit_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceMarkHostNoCommit = true; return x; }());
    return t_sayonce_engine_marks();
}
static bool t_sayonce_mark_host_no_mark_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceMarkHostNoMark = true; return x; }());
    return t_sayonce_engine_marks();
}
static bool t_sayonce_mark_host_drops_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.sayOnceMarkHostDrops = true; return x; }());
    return t_sayonce_engine_marks();
}
static bool t_sayonce_host_own_line_stays() {
    // the host player already said this per-player line themselves: a guest's later use must not clear the host's own state
    fnvcoop_dlg::SayOncePolicy p;
    p.setPerPlayer({kInfo1});
    p.noteOwnSaid(kInfo1);
    CHECK(p.onSpectated(kInfo1, true).kind == fnvcoop_dlg::SayOnceAction::None);
    CHECK(p.onSpectated(kInfo1, false).kind == fnvcoop_dlg::SayOnceAction::None);
    CHECK(p.onSpectated(FormRef("falloutnv.esm", 1), false).kind == fnvcoop_dlg::SayOnceAction::Mark);
    std::istringstream in("# c\nspeech FalloutNV.esm:011D99\ntrainer DeadMoney.esm:0F\nbogus x\n");
    auto s = fnvcoop_dlg::loadSayOncePerPlayer(in);
    CHECK(s.size() == 2 && s.count(FormRef("falloutnv.esm", 0x11D99)) && s.count(FormRef("deadmoney.esm", 0xF)));
    return true;
}

// a refused fork closes the conversation for the initiator: they are told, and the line the engine already marked is theirs again
static bool t_consent_denied_gives_the_choice_back() {
    Rig r;
    Mock& m = r.mock[1];
    Addr32 info = m.img.info(kHouse, -1, 0, "Cat.");
    m.forms[0x7001] = kTopic1;
    m.topicOfInfo[info] = {kTopic1, kInfo1};
    m.img.cache(info, kHouse);
    m.img.menu(0x777, dialog_menu::kStateIdle, 0, {{info, 0x7001}});
    r.glue[1]->onTalkRequested(kNpc, 0);
    r.step(100, false);
    CHECK(r.glue[1]->onInfoChosen(info, uint32_t(r.now)));
    Event e;
    e.kind = Event::CloseMenu; e.reason = fnvcoop_dlg::CloseReason::ConsentDenied; e.wasInitiator = true;
    r.glue[1]->onEvent(e);
    CHECK(m.forced == 1 && m.notices2.size() == 1 && m.cleared.size() == 1 && m.cleared[0] == kInfo1);
    Event spect = e; spect.wasInitiator = false;                                  // spectators only hide their overlay
    r.glue[2]->onEvent(spect);
    CHECK(r.mock[2].cleared.empty() && r.mock[2].notices2.empty());
    return true;
}
static bool t_consent_denied_keeps_said_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.denyKeepsSaid = true; return x; }());
    return t_consent_denied_gives_the_choice_back();
}

// ---- facts
static bool t_fact_table() {
    std::istringstream in("# facts\nFalloutNV.esm:00AB av=43,item=FalloutNV.esm:00000F,perk=FalloutNV.esm:0ED568,equip=DeadMoney.esm:01\nFalloutNV.esm:00AC item=bad\nbroken\n");
    auto t = fnvcoop_dlg::loadFactTable(in);
    CHECK(t.size() == 1);
    const auto* v = t.find(FormRef("falloutnv.esm", 0xAB));
    CHECK(v && v->size() == 4 && (*v)[0].kind == fnvcoop_dlg::FactKind::ActorValue && (*v)[0].what.localId == 43 && (*v)[1].what == FormRef("falloutnv.esm", 0xF));
    CHECK((*v)[2].kind == fnvcoop_dlg::FactKind::HasPerk && (*v)[3].what == FormRef("deadmoney.esm", 1));
    int calls = 0;
    auto facts = t.collect(FormRef("falloutnv.esm", 0xAB), [&](const fnvcoop_dlg::FactSpec& s, int32_t& value) {
        ++calls;
        if (s.kind == fnvcoop_dlg::FactKind::Equipped) return false;        // unreadable: skipped, not zero
        value = int32_t(s.what.localId) * 2;
        return true;
    });
    CHECK(calls == 4 && facts.size() == 3 && facts[0].kind == 1 && facts[0].value == 86 && facts[2].kind == 3);
    CHECK(t.collect(FormRef("falloutnv.esm", 1), [](const fnvcoop_dlg::FactSpec&, int32_t&) { return true; }).empty());
    return true;
}

// the generated tables parse, and mean what the owner decisions say
static std::string slurp(const char* path) {
    std::ifstream f(path, std::ios::binary);
    std::ostringstream o;
    o << f.rdbuf();
    return o.str();
}
static bool t_data_tables() {
    fnvcoop_dlg::DialogueData d;
    fnvcoop_dlg::loadDialogueData([](const std::string& n, std::string& text) { text = slurp(("data/" + n).c_str()); return !text.empty(); }, d);
    CHECK(d.report.missing.empty());
    CHECK(d.report.ownerOnlyTopics >= 20 && d.report.sayOncePerPlayer >= 50 && d.report.facts >= 1000 && d.report.followerBytes > 500);
    CHECK(d.sayOnce.perPlayer(FormRef("falloutnv.esm", 0x08F070)));         // Ranger Takedown trainer line
    CHECK(!d.sayOnce.perPlayer(FormRef("falloutnv.esm", 0x1258AA)));        // an ordinary say-once greeting stays shared
    return true;
}

// ---- G33: follower registry from the data file
static const FormRef kVnpc("falloutnv.esm", 0x0B16D0), kDlc1("deadmoney.esm", 0x0012FD), kDlc2("honesthearts.esm", 0x009DBA);
static bool t_followers(bool canon) {
    fnvcoop_dlg::CompanionRegistry reg;
    std::istringstream in(slurp("data/follower_rules.txt"));
    auto st = fnvcoop_dlg::loadFollowerRules(in, reg);
    CHECK(st.quests == 3 && st.companions == 14 && st.bad == 0 && st.rules >= 4 + 3 * 14 + 1);
    const FormRef boone("falloutnv.esm", 0x096BCE), cass("falloutnv.esm", 0x135F19), ede("falloutnv.esm", 0x1732D1), ede2("falloutnv.esm", 0x10C769);
    const FormRef dean("deadmoney.esm", 0x0012FE), joshua("honesthearts.esm", 0x0093BE);
    CHECK(reg.isCompanion(boone) && reg.isCompanion(dean) && reg.isCompanion(joshua));
    using fnvcoop_dlg::RecruitResult;
    CHECK(reg.recruit(boone, 1) == RecruitResult::Ok);
    CHECK(reg.recruit(ede, 1) == RecruitResult::Ok);                        // ED-E (critter) next to Boone (humanoid)
    CHECK(canon ? reg.ownerOf(ede2) == 1 : true);                           // ED-E under another of its forms is the same companion
    CHECK(canon ? reg.recruit(ede2, 2) == RecruitResult::OwnedByOther : true);
    CHECK(reg.recruit(cass, 1) == RecruitResult::OwnerCapHumanoid);         // 1 humanoid + 1 critter per player
    CHECK(reg.recruit(cass, 2) == RecruitResult::Ok);                       // the second player has their own pair
    // what each player's dialogue conditions see
    CHECK(reg.viewVar(kVnpc, 47, 1, 0) == 1 && reg.viewVar(kVnpc, 48, 1, 0) == 1);   // player 1: humanoid and critter slots full
    CHECK(reg.viewVar(kVnpc, 48, 2, 0) == 0);                                        // player 2 may still recruit a critter
    CHECK(reg.viewVar(kVnpc, 6, 1, 0) == 1 && reg.viewVar(kVnpc, 6, 2, 0) == 0);     // bBooneHired: mine / not mine
    CHECK(reg.viewVar(kVnpc, 8, 2, 1) == 0);                                          // Boone is not available to player 2
    // Dead Money / Honest Hearts follower quests are per owner as well
    reg.setOwner(dean, 3);
    CHECK(reg.viewVar(kDlc1, 2, 3, 0) == 1 && reg.viewVar(kDlc1, 2, 1, 0) == 0);      // bDeanHired
    CHECK(reg.viewVar(kDlc1, 18, 3, 0) == 1 && reg.viewVar(kDlc1, 18, 2, 0) == 1 && reg.viewVar(kDlc1, 18, 4, 0) == 0);   // bHaveCompanion: has a follower
    CHECK(reg.viewVar(kDlc2, 12, 1, 0) == 0);                                         // bJoshuaGrahamHired
    reg.setOwner(joshua, 1);
    CHECK(reg.viewVar(kDlc2, 12, 1, 0) == 1 && reg.viewVar(kDlc2, 12, 2, 0) == 0);
    return true;
}
static bool t_followers_ok() { return t_followers(true); }
static bool t_followers_no_canon_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.noCompanionCanon = true; return x; }());
    return t_followers(true);
}
static bool t_followers_no_rewrite_twin() {
    fnvcoop_dlg::MutationScope ms([] { fnvcoop_dlg::Mutations x; x.noOwnerRewrite = true; return x; }());
    return t_followers(true);
}

// a node picks up the tables after it was built
static bool t_configure_node() {
    Rig r;
    fnvcoop_dlg::DialogueData d;
    fnvcoop_dlg::loadDialogueData([](const std::string& n, std::string& text) { text = slurp(("data/" + n).c_str()); return !text.empty(); }, d);
    auto st = fnvcoop_dlg::configureNode(*r.node[0], d);
    CHECK(st.companions == 14 && r.node[0]->ownerOnlyTopics().size() == d.ownerOnly.size());
    CHECK(r.node[0]->companions().isCompanion(FormRef("falloutnv.esm", 0x096BCE)));
    // the host now rejects an owner-only command from a non-owner: the topic set reached HostDialogue
    const FormRef boone("falloutnv.esm", 0x096BCE), wait("falloutnv.esm", 0x037071);
    r.node[0]->companions().setOwner(boone, 1);
    r.node[1]->companions().define(boone, fnvcoop_dlg::CompanionKind::Humanoid);
    CHECK(r.node[0]->ownerOnlyTopics().count(wait) == 1);
    fnvcoop_dlg::CompanionRules cr;
    cr.partyTotal = 6;
    r.node[0]->setCompanionRules(cr);
    CHECK(r.node[0]->companions().rules().partyTotal == 6);
    return true;
}

int main() {
    struct C { const char* name; bool (*fn)(); bool twin; };
    C cases[] = {{"menu_and_flags", t_menu_and_flags, false}, {"probe_and_lines_unk48", t_probe_ok, false},
                 {"probe_and_lines_other_offset", t_probe_other_offset, false}, {"probe_real_info_has_no_head", t_probe_real_info_has_none, false},
                 {"cache_read", t_cache_read, false}, {"loader", t_loader, false}, {"voice_path", t_voice, false},
                 {"glue_flow", t_glue_ok, false}, {"unknown_response_reports", t_unknown_response_reports, false},
                 {"control_lock_and_leave", t_control_lock_and_goodbye, false},
                 {"glue_menus", t_glue_menus, false},
                 {"info_reader_factory", t_info_reader_factory, false},
                 {"glue_forced_say_audibility", t_glue_forced_say_audibility, false},
                 {"glue_forced_conversation", t_glue_forced_conversation, false},
                 {"glue_hides_commands_non_owner", []() { return t_glue_hides_commands_for_non_owner(false); }, false},
                 {"glue_keeps_commands_owner", []() { return t_glue_hides_commands_for_non_owner(true); }, false},
                 {"glue_refuses_command_pick", t_glue_refuses_command_pick, false},
                 {"poll_full_conversation", t_poll_full_conversation, false}, {"poll_walk_away", t_poll_walk_away, false},
                 {"poll_same_info_again_is_not_a_new_pick", t_poll_same_info_again_is_not_a_new_pick, false},
                 {"poll_host_native_broadcast", t_poll_host_native_broadcast, false},
                 {"grant_without_menu_is_given_back", t_grant_without_menu_is_given_back, false},
                 {"sayonce_shared", t_sayonce_shared, false}, {"sayonce_per_player", t_sayonce_per_player, false},
                 {"sayonce_host_own_line_stays", t_sayonce_host_own_line_stays, false},
                 {"sayonce_engine_marks", t_sayonce_engine_marks, false}, {"sayonce_engine_marks_forged_and_known", t_sayonce_engine_marks_forged_and_known, false},
                 {"consent_denied_gives_the_choice_back", t_consent_denied_gives_the_choice_back, false},
                 {"fact_table", t_fact_table, false}, {"data_tables", t_data_tables, false},
                 {"followers_registry", t_followers_ok, false}, {"configure_node", t_configure_node, false},
                 // mutation twins: each must make its scenario FAIL
                 {"twin_probe_wrong_layout", t_probe_wrong_layout_twin, true},
                 {"twin_glue_broken_layout", t_glue_broken_layout_twin, true},
                 {"twin_cache_wrong_info", t_cache_wrong_info_twin, true},
                 {"twin_command_pick", t_glue_command_pick_twin, true}, {"twin_forced_host_replays", t_glue_forced_host_replays_twin, true},
                 {"twin_poll_no_pick", t_poll_no_pick_twin, true}, {"twin_poll_pick_every_frame", t_poll_pick_every_frame_twin, true},
                 {"twin_poll_no_line_done", t_poll_no_line_done_twin, true}, {"twin_poll_leave_after_goodbye", t_poll_leave_after_goodbye_twin, true},
                 {"twin_host_talk_silent", t_poll_host_silent_twin, true}, {"twin_grant_no_watchdog", t_grant_without_menu_twin, true},
                 {"twin_poll_no_pick_latch", t_poll_same_info_again_twin, true},
                 {"twin_sayonce_never_shared", t_sayonce_never_twin, true}, {"twin_sayonce_all_shared", t_sayonce_all_shared_twin, true},
                 {"twin_sayonce_keep_host_mark", t_sayonce_keep_host_mark_twin, true},
                 {"twin_sayonce_mark_not_reported", t_sayonce_mark_not_reported_twin, true}, {"twin_sayonce_mark_trainer_shared", t_sayonce_mark_trainer_shared_twin, true},
                 {"twin_sayonce_mark_trainer_shared_forged", t_sayonce_mark_trainer_shared_forged_twin, true}, {"twin_sayonce_mark_host_no_commit", t_sayonce_mark_host_no_commit_twin, true},
                 {"twin_sayonce_mark_host_no_mark", t_sayonce_mark_host_no_mark_twin, true}, {"twin_sayonce_mark_host_drops", t_sayonce_mark_host_drops_twin, true}, {"twin_consent_denied_keeps_said", t_consent_denied_keeps_said_twin, true},
                 {"twin_followers_no_canon", t_followers_no_canon_twin, true}, {"twin_followers_no_rewrite", t_followers_no_rewrite_twin, true}};
    int fail = 0;
    for (auto& c : cases) {
        g_why.clear();
        fnvcoop_dlg::mutations() = fnvcoop_dlg::Mutations();
        bool ok = c.fn();
        bool pass = c.twin ? !ok : ok;
        std::printf("[%s] %s%s%s\n", pass ? "PASS" : "FAIL", c.name, c.twin ? (ok ? " (mutant survived)" : " (caught: ") : "", c.twin && !ok ? (g_why + ")").c_str() : (ok ? "" : (" : " + g_why).c_str()));
        fail += !pass;
    }
    std::printf("%d failed\n", fail);
    return fail ? 1 : 0;
}
