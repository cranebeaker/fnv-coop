// Dialogue protocol messages (L8). Game-free, header-only, C++17. Depends only on fnvcoop/types.h.
//
// Envelope: [0xD1][ver][type][body...][crc32 LE of everything before]. Integers are LEB128 varints (signed = zigzag),
// strings are length-prefixed, FormRefs are (plugin string, local id). Decode never throws and never reads past the
// buffer; any malformed input yields a non-Ok DecodeStatus. Proposed to L4: carry these as the opaque payload of a
// `DlgMsg { uint32_t session; std::vector<uint8_t> bytes; }` entry in fnvcoop::Message.
#pragma once
#include <cstdint>
#include <string>
#include <variant>
#include <vector>
#include "fnvcoop/types.h"

namespace fnvcoop_dlg {

using fnvcoop::FormRef;
using fnvcoop::PlayerId;

constexpr uint8_t kMagic = 0xD1;
constexpr uint8_t kVersion = 1;
constexpr size_t kMaxLines = 32, kMaxTopics = 64, kMaxStr = 1024, kMaxFacts = 128, kMaxButtons = 10;

using ConvId = uint32_t;  // host-assigned, never 0

enum class DenyReason : uint8_t {
    Busy = 1,            // NPC already in a conversation with another player
    AlreadyTalking = 2,  // requester is already in a conversation
    NpcDead = 3,
    NpcInCombat = 4,
    Unknown = 5,         // NPC not known to the host
    TooMany = 6,
};
enum class CloseReason : uint8_t {
    Goodbye = 1,        // goodbye INFO finished
    InitiatorLeft = 2,  // initiator pressed Esc / walked away
    Disconnect = 3,     // initiator dropped
    NpcDead = 4,
    Combat = 5,
    Timeout = 6,        // lease expired (initiator silent)
    HostShutdown = 7,
    ProtocolError = 8,
    Preempted = 9,      // the companion's owner took the NPC over from a non-owner conversation
    ConsentDenied = 10, // the party refused an irreversible choice (ChoiceConsent): the INFO's scripts never ran
};
enum class Phase : uint8_t { Begin = 0, End = 1 };

// What kind of check a topic/choice carries (initiator-evaluated; spectators only display it).
enum class CheckKind : uint8_t { None = 0, Speech = 1, Skill = 2, Perk = 3, Barter = 4, Item = 5 };

struct TopicEntry {
    FormRef topic;
    uint8_t check = 0;      // CheckKind
    uint8_t flags = 0;      // bit0 = already said, bit1 = greyed out
    bool operator==(const TopicEntry& o) const { return topic == o.topic && check == o.check && flags == o.flags; }
};
struct Line {
    std::string text;
    std::string voice;       // path relative to Data\Sound\Voice, empty = subtitle only
    uint8_t emotion = 0;
    uint8_t emotionValue = 50;
    bool operator==(const Line& o) const {
        return text == o.text && voice == o.voice && emotion == o.emotion && emotionValue == o.emotionValue;
    }
};
// A player-private fact the initiator machine observed and the host must use when running the result script
// (see dialogue-design.md "player facts"). kind: 1 = actor value, 2 = item count, 3 = has perk, 4 = equipped.
struct Fact {
    uint8_t kind = 0;
    FormRef what;
    int32_t value = 0;
    bool operator==(const Fact& o) const { return kind == o.kind && what == o.what && value == o.value; }
};

// ---- client (initiator) -> host
struct StartReq { uint32_t reqId = 0; FormRef npc; };
struct Offer { ConvId conv = 0; uint32_t step = 0; std::vector<TopicEntry> topics; };            // initiator -> host -> others
struct Pick {                                                                                     // initiator -> host
    ConvId conv = 0; uint32_t step = 0;
    FormRef topic, info;
    int32_t choice = -1;              // index within the offered list, -1 = n/a
    bool goodbye = false;             // INFO has the goodbye flag
    std::vector<Line> lines;          // resolved lines (random INFOs are resolved by the initiator)
    std::vector<Fact> facts;          // player facts the begin/end result scripts read
};
struct LineDone { ConvId conv = 0; uint32_t step = 0; };                                          // initiator -> host
struct EndReq { ConvId conv = 0; CloseReason reason = CloseReason::InitiatorLeft; };              // initiator -> host
struct Heartbeat { ConvId conv = 0; };                                                            // initiator -> host

// ---- host -> clients
struct StartDenied { uint32_t reqId = 0; DenyReason reason = DenyReason::Busy; PlayerId owner = fnvcoop::kNoPlayer; };
struct Opened { ConvId conv = 0; uint32_t reqId = 0; FormRef npc; PlayerId initiator = 0; };
struct Lines { ConvId conv = 0; uint32_t step = 0; FormRef topic, info; bool goodbye = false; std::vector<Line> lines; };  // to spectators
// Result-script transaction for (step, phase) is committed at txnSeq (0 = script produced no effects).
struct Result { ConvId conv = 0; uint32_t step = 0; Phase phase = Phase::Begin; uint64_t txnSeq = 0; };
struct Control { ConvId conv = 0; uint8_t lockMask = 0; bool locked = false; };                   // host -> initiator
struct Closed { ConvId conv = 0; CloseReason reason = CloseReason::Goodbye; };

// ---- menus raised by host-run scripts (ShowMessage with buttons, ShowBarterMenu, companion wheel, terminals, ...).
// The menu opens on the ORIGIN player (whose context the script runs in), never on the host by default; everybody
// else only gets a read-only notice. The host script waits for the origin's MenuReply (or the default on timeout).
enum class MenuKind : uint8_t { MessageBox = 1, Barter = 2, Repair = 3, Recipe = 4, Caravan = 5, CompanionWheel = 6,
                                Terminal = 7, Hacking = 8, Lockpick = 9, Other = 10 };
struct MenuRequest {                                                  // host -> origin only
    uint32_t menu = 0; MenuKind kind = MenuKind::MessageBox; PlayerId origin = 0;
    std::string text; std::vector<std::string> buttons; int32_t defaultChoice = -1; FormRef subject;
    bool consent = false;   // party-consent prompt for a non-origin player ("Ready" / "Not yet"), see ModalOptions
};
struct MenuReply { uint32_t menu = 0; int32_t choice = -1; };         // origin -> host
struct MenuNotice { uint32_t menu = 0; MenuKind kind = MenuKind::MessageBox; PlayerId origin = 0; std::string text; };  // host -> others
struct MenuClosed { uint32_t menu = 0; int32_t choice = -1; bool timedOut = false; std::string label; };            // host -> all

// ---- forced dialogue: scripted SayTo / StartConversation aimed at "the player" (host-run scripts).
// The host resolves the INFO (conditions evaluated for the target's facts), runs its scripts once (captured), and
// ships the resolved lines; receivers only play them. `scene` = cutscene: everyone must see/hear it.
enum class ForcedKind : uint8_t { Say = 1, Conversation = 2 };
struct ForcedLines {                                                    // host -> all
    uint32_t id = 0; ForcedKind kind = ForcedKind::Say; FormRef npc, topic, info;
    PlayerId target = fnvcoop::kNoPlayer;   // the player the line is aimed at (kNoPlayer = party)
    bool scene = false; uint64_t txnSeq = 0; std::vector<Line> lines;
};
struct ForcedStart { uint32_t id = 0; ConvId conv = 0; FormRef npc, topic; };   // host -> target only: open the forced conversation

// ---- say-once state (owner decision D11): a guest's engine marked `info` said (TESTopicInfo::MarkSaid), e.g. the scripted SayTo lines of a
// quest stage the guest applied while the host's copy of the NPC is not loaded. The host marks its own copy and commits SayOnceFlag.
struct SayOnceMark { FormRef info; };                                                              // guest -> host

using Msg = std::variant<StartReq, Offer, Pick, LineDone, EndReq, Heartbeat, StartDenied, Opened, Lines, Result,
                         Control, Closed, MenuRequest, MenuReply, MenuNotice, MenuClosed, ForcedLines, ForcedStart, SayOnceMark>;

enum class DecodeStatus { Ok, TooShort, BadMagic, BadVersion, BadCrc, Malformed, Trailing };

namespace detail {
inline uint32_t crc32(const uint8_t* d, size_t n) {
    uint32_t c = 0xFFFFFFFFu;
    for (size_t i = 0; i < n; ++i) {
        c ^= d[i];
        for (int k = 0; k < 8; ++k) c = (c >> 1) ^ (0xEDB88320u & (0u - (c & 1u)));
    }
    return ~c;
}
struct W {
    std::vector<uint8_t> b;
    void u8(uint8_t v) { b.push_back(v); }
    void var(uint64_t v) { while (v >= 0x80) { b.push_back(uint8_t(v) | 0x80); v >>= 7; } b.push_back(uint8_t(v)); }
    void vari(int64_t v) { var((uint64_t(v) << 1) ^ uint64_t(v >> 63)); }
    void str(const std::string& s) { var(s.size()); b.insert(b.end(), s.begin(), s.end()); }
    void form(const FormRef& f) { str(f.plugin); var(f.localId); }
};
struct R {
    const uint8_t* p; size_t n, pos = 0; bool ok = true;
    R(const uint8_t* d, size_t sz) : p(d), n(sz) {}
    uint8_t u8() { if (pos >= n) { ok = false; return 0; } return p[pos++]; }
    uint64_t var() {
        uint64_t v = 0; int sh = 0;
        for (;;) {
            if (pos >= n || sh > 63) { ok = false; return 0; }
            uint8_t c = p[pos++]; v |= uint64_t(c & 0x7F) << sh;
            if (!(c & 0x80)) return v;
            sh += 7;
        }
    }
    int64_t vari() { uint64_t v = var(); return int64_t(v >> 1) ^ -int64_t(v & 1); }
    uint32_t v32() { uint64_t v = var(); if (v > 0xFFFFFFFFull) { ok = false; return 0; } return uint32_t(v); }
    std::string str() {
        uint64_t l = var();
        if (!ok || l > kMaxStr || l > n - pos) { ok = false; return {}; }
        std::string s(reinterpret_cast<const char*>(p + pos), size_t(l)); pos += size_t(l); return s;
    }
    FormRef form() { FormRef f; f.plugin = str(); f.localId = v32(); return f; }
    size_t count(size_t maxN) { uint64_t c = var(); if (!ok || c > maxN || c > n - pos) { ok = false; return 0; } return size_t(c); }
};
inline void put(W& w, const TopicEntry& t) { w.form(t.topic); w.u8(t.check); w.u8(t.flags); }
inline void get(R& r, TopicEntry& t) { t.topic = r.form(); t.check = r.u8(); t.flags = r.u8(); }
inline void put(W& w, const Line& l) { w.str(l.text); w.str(l.voice); w.u8(l.emotion); w.u8(l.emotionValue); }
inline void get(R& r, Line& l) { l.text = r.str(); l.voice = r.str(); l.emotion = r.u8(); l.emotionValue = r.u8(); }
inline void put(W& w, const Fact& f) { w.u8(f.kind); w.form(f.what); w.vari(f.value); }
inline void get(R& r, Fact& f) { f.kind = r.u8(); f.what = r.form(); f.value = int32_t(r.vari()); }
template <class T, class F> void putVec(W& w, const std::vector<T>& v, F f) { w.var(v.size()); for (auto& e : v) f(w, e); }
template <class T, class F> void getVec(R& r, std::vector<T>& v, size_t maxN, F f) {
    size_t c = r.count(maxN); v.resize(r.ok ? c : 0);
    for (auto& e : v) { if (!r.ok) break; f(r, e); }
}
template <class E> E enumv(R& r, uint8_t lo, uint8_t hi) { uint8_t v = r.u8(); if (v < lo || v > hi) r.ok = false; return E(v); }
}  // namespace detail

inline std::vector<uint8_t> encode(const Msg& m) {
    using namespace detail;
    W w; w.u8(kMagic); w.u8(kVersion); w.u8(uint8_t(m.index() + 1));
    std::visit([&](const auto& x) {
        using T = std::decay_t<decltype(x)>;
        if constexpr (std::is_same_v<T, StartReq>) { w.var(x.reqId); w.form(x.npc); }
        else if constexpr (std::is_same_v<T, Offer>) { w.var(x.conv); w.var(x.step); putVec(w, x.topics, [](W& a, const TopicEntry& t) { put(a, t); }); }
        else if constexpr (std::is_same_v<T, Pick>) {
            w.var(x.conv); w.var(x.step); w.form(x.topic); w.form(x.info); w.vari(x.choice); w.u8(x.goodbye);
            putVec(w, x.lines, [](W& a, const Line& l) { put(a, l); });
            putVec(w, x.facts, [](W& a, const Fact& f) { put(a, f); });
        }
        else if constexpr (std::is_same_v<T, LineDone>) { w.var(x.conv); w.var(x.step); }
        else if constexpr (std::is_same_v<T, EndReq>) { w.var(x.conv); w.u8(uint8_t(x.reason)); }
        else if constexpr (std::is_same_v<T, Heartbeat>) { w.var(x.conv); }
        else if constexpr (std::is_same_v<T, StartDenied>) { w.var(x.reqId); w.u8(uint8_t(x.reason)); w.u8(x.owner); }
        else if constexpr (std::is_same_v<T, Opened>) { w.var(x.conv); w.var(x.reqId); w.form(x.npc); w.u8(x.initiator); }
        else if constexpr (std::is_same_v<T, Lines>) {
            w.var(x.conv); w.var(x.step); w.form(x.topic); w.form(x.info); w.u8(x.goodbye);
            putVec(w, x.lines, [](W& a, const Line& l) { put(a, l); });
        }
        else if constexpr (std::is_same_v<T, Result>) { w.var(x.conv); w.var(x.step); w.u8(uint8_t(x.phase)); w.var(x.txnSeq); }
        else if constexpr (std::is_same_v<T, Control>) { w.var(x.conv); w.u8(x.lockMask); w.u8(x.locked); }
        else if constexpr (std::is_same_v<T, Closed>) { w.var(x.conv); w.u8(uint8_t(x.reason)); }
        else if constexpr (std::is_same_v<T, MenuRequest>) {
            w.var(x.menu); w.u8(uint8_t(x.kind)); w.u8(x.origin); w.str(x.text); w.vari(x.defaultChoice); w.form(x.subject);
            w.var(x.buttons.size()); for (auto& b : x.buttons) w.str(b);
            w.u8(x.consent);
        }
        else if constexpr (std::is_same_v<T, ForcedLines>) {
            w.var(x.id); w.u8(uint8_t(x.kind)); w.form(x.npc); w.form(x.topic); w.form(x.info); w.u8(x.target);
            w.u8(x.scene); w.var(x.txnSeq); putVec(w, x.lines, [](W& a, const Line& l) { put(a, l); });
        }
        else if constexpr (std::is_same_v<T, ForcedStart>) { w.var(x.id); w.var(x.conv); w.form(x.npc); w.form(x.topic); }
        else if constexpr (std::is_same_v<T, SayOnceMark>) { w.form(x.info); }
        else if constexpr (std::is_same_v<T, MenuReply>) { w.var(x.menu); w.vari(x.choice); }
        else if constexpr (std::is_same_v<T, MenuNotice>) { w.var(x.menu); w.u8(uint8_t(x.kind)); w.u8(x.origin); w.str(x.text); }
        else if constexpr (std::is_same_v<T, MenuClosed>) { w.var(x.menu); w.vari(x.choice); w.u8(x.timedOut); w.str(x.label); }
    }, m);
    uint32_t c = crc32(w.b.data(), w.b.size());
    for (int i = 0; i < 4; ++i) w.u8(uint8_t(c >> (8 * i)));
    return w.b;
}

inline DecodeStatus decode(const uint8_t* d, size_t n, Msg& out, bool verifyCrc = true) {
    using namespace detail;
    if (n < 7) return DecodeStatus::TooShort;
    if (d[0] != kMagic) return DecodeStatus::BadMagic;
    if (d[1] != kVersion) return DecodeStatus::BadVersion;
    if (verifyCrc) {
        uint32_t want = 0; for (int i = 0; i < 4; ++i) want |= uint32_t(d[n - 4 + i]) << (8 * i);
        if (crc32(d, n - 4) != want) return DecodeStatus::BadCrc;
    }
    R r(d + 3, n - 7);
    Msg m;
    switch (d[2]) {
        case 1: { StartReq x; x.reqId = r.v32(); x.npc = r.form(); m = x; break; }
        case 2: { Offer x; x.conv = r.v32(); x.step = r.v32(); getVec(r, x.topics, kMaxTopics, [](R& a, TopicEntry& t) { get(a, t); }); m = x; break; }
        case 3: {
            Pick x; x.conv = r.v32(); x.step = r.v32(); x.topic = r.form(); x.info = r.form();
            x.choice = int32_t(r.vari()); x.goodbye = r.u8() != 0;
            getVec(r, x.lines, kMaxLines, [](R& a, Line& l) { get(a, l); });
            getVec(r, x.facts, kMaxFacts, [](R& a, Fact& f) { get(a, f); });
            m = x; break;
        }
        case 4: { LineDone x; x.conv = r.v32(); x.step = r.v32(); m = x; break; }
        case 5: { EndReq x; x.conv = r.v32(); x.reason = enumv<CloseReason>(r, 1, 10); m = x; break; }
        case 6: { Heartbeat x; x.conv = r.v32(); m = x; break; }
        case 7: { StartDenied x; x.reqId = r.v32(); x.reason = enumv<DenyReason>(r, 1, 6); x.owner = r.u8(); m = x; break; }
        case 8: { Opened x; x.conv = r.v32(); x.reqId = r.v32(); x.npc = r.form(); x.initiator = r.u8(); m = x; break; }
        case 9: {
            Lines x; x.conv = r.v32(); x.step = r.v32(); x.topic = r.form(); x.info = r.form(); x.goodbye = r.u8() != 0;
            getVec(r, x.lines, kMaxLines, [](R& a, Line& l) { get(a, l); });
            m = x; break;
        }
        case 10: { Result x; x.conv = r.v32(); x.step = r.v32(); x.phase = enumv<Phase>(r, 0, 1); x.txnSeq = r.var(); m = x; break; }
        case 11: { Control x; x.conv = r.v32(); x.lockMask = r.u8(); x.locked = r.u8() != 0; m = x; break; }
        case 12: { Closed x; x.conv = r.v32(); x.reason = enumv<CloseReason>(r, 1, 10); m = x; break; }
        case 13: {
            MenuRequest x; x.menu = r.v32(); x.kind = enumv<MenuKind>(r, 1, 10); x.origin = r.u8(); x.text = r.str();
            x.defaultChoice = int32_t(r.vari()); x.subject = r.form();
            size_t n = r.count(kMaxButtons); x.buttons.resize(r.ok ? n : 0);
            for (auto& b : x.buttons) { if (!r.ok) break; b = r.str(); }
            x.consent = r.u8() != 0;
            m = x; break;
        }
        case 14: { MenuReply x; x.menu = r.v32(); x.choice = int32_t(r.vari()); m = x; break; }
        case 15: { MenuNotice x; x.menu = r.v32(); x.kind = enumv<MenuKind>(r, 1, 10); x.origin = r.u8(); x.text = r.str(); m = x; break; }
        case 16: { MenuClosed x; x.menu = r.v32(); x.choice = int32_t(r.vari()); x.timedOut = r.u8() != 0; x.label = r.str(); m = x; break; }
        case 17: {
            ForcedLines x; x.id = r.v32(); x.kind = enumv<ForcedKind>(r, 1, 2); x.npc = r.form(); x.topic = r.form(); x.info = r.form();
            x.target = r.u8(); x.scene = r.u8() != 0; x.txnSeq = r.var();
            getVec(r, x.lines, kMaxLines, [](R& a, Line& l) { get(a, l); });
            m = x; break;
        }
        case 18: { ForcedStart x; x.id = r.v32(); x.conv = r.v32(); x.npc = r.form(); x.topic = r.form(); m = x; break; }
        case 19: { SayOnceMark x; x.info = r.form(); m = x; break; }
        default: return DecodeStatus::Malformed;
    }
    if (!r.ok) return DecodeStatus::Malformed;
    if (r.pos != r.n) return DecodeStatus::Trailing;
    out = std::move(m);
    return DecodeStatus::Ok;
}
inline DecodeStatus decode(const std::vector<uint8_t>& b, Msg& out) { return decode(b.data(), b.size(), out); }

}  // namespace fnvcoop_dlg
