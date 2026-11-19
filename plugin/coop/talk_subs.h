// Conversation subtitles for the other players. A conversation opens only on the machine of the player who has it (talk_gate.h); the
// others saw nothing of it. Each machine reads the line its own DialogMenu shows (speaker name + text) and sends every new line on mux
// channel kChannel; the host shows a guest's line itself and relays it to the other guests. A receiver shows it as HUD messages when its
// player is near the NPC. Game-free: the wire, the new-line detector and the subtitle chunking; coop_module.cpp does the engine part.
//   [0xD5][ver=2][slot u8][npc][speaker: u8 len + bytes][player: u8 len + bytes][text: u16 LE len + bytes][topic][info]
//   with a form as [plugin: u8 len + bytes][localId u32 LE]. topic + info name the INFO being spoken (empty for a reply line): the receiver's
//   NPC says exactly that INFO, so the watcher hears the voice in step with the text.
#pragma once
#include <cstdint>
#include <string>
#include <vector>

#include "fnvcoop/types.h"

namespace nvcoop { namespace coop { namespace talksubs {

constexpr uint8_t kChannel = 0xD5;   // 0xAC actors, 0xAD avatars, 0xD1 dialogue node, 0xDA area digest, 0xE5 slides
constexpr uint8_t kVersion = 2;
constexpr size_t kMaxName = 64, kMaxText = 1024;
constexpr float kHearDistance = 2000.0f;   // game units: about the range a nearby player would hear the NPC

struct Line {
    uint8_t slot = 0;              // the player having the conversation
    fnvcoop::FormRef npc;          // who speaks the line
    std::string speaker, player;   // display names (the NPC's, the talking player's)
    std::string text;
    fnvcoop::FormRef topic, info;   // the INFO this NPC line belongs to (empty: the player's reply)
};

inline std::vector<uint8_t> encode(const Line& l) {
    std::vector<uint8_t> b{kChannel, kVersion, l.slot};
    auto str8 = [&](const std::string& s, size_t max) {
        const size_t n = s.size() < max ? s.size() : max;
        b.push_back(static_cast<uint8_t>(n));
        b.insert(b.end(), s.begin(), s.begin() + static_cast<std::ptrdiff_t>(n));
    };
    auto form = [&](const fnvcoop::FormRef& f) {
        str8(f.plugin, 255);
        for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(f.localId >> (8 * i)));
    };
    form(l.npc);
    str8(l.speaker, kMaxName);
    str8(l.player, kMaxName);
    const size_t n = l.text.size() < kMaxText ? l.text.size() : kMaxText;
    b.push_back(static_cast<uint8_t>(n & 0xFF));
    b.push_back(static_cast<uint8_t>(n >> 8));
    b.insert(b.end(), l.text.begin(), l.text.begin() + static_cast<std::ptrdiff_t>(n));
    form(l.topic);
    form(l.info);
    return b;
}

inline bool decode(const uint8_t* p, size_t n, Line* out) {
    size_t at = 0;
    auto u8 = [&](uint8_t* v) { if (at >= n) return false; *v = p[at++]; return true; };
    auto str = [&](size_t len, std::string* s) { if (n - at < len) return false; s->assign(reinterpret_cast<const char*>(p + at), len); at += len; return true; };
    uint8_t ch = 0, ver = 0, len = 0;
    Line l;
    if (!u8(&ch) || ch != kChannel || !u8(&ver) || ver != kVersion || !u8(&l.slot)) return false;
    auto form = [&](fnvcoop::FormRef* f) {
        if (!u8(&len) || !str(len, &f->plugin) || n - at < 4) return false;
        f->localId = uint32_t(p[at]) | uint32_t(p[at + 1]) << 8 | uint32_t(p[at + 2]) << 16 | uint32_t(p[at + 3]) << 24;
        at += 4;
        return true;
    };
    if (!form(&l.npc)) return false;
    if (!u8(&len) || len > kMaxName || !str(len, &l.speaker)) return false;
    if (!u8(&len) || len > kMaxName || !str(len, &l.player)) return false;
    if (n - at < 2) return false;
    const size_t tn = size_t(p[at]) | size_t(p[at + 1]) << 8;
    at += 2;
    if (tn > kMaxText || !str(tn, &l.text) || !form(&l.topic) || !form(&l.info) || at != n) return false;
    *out = std::move(l);
    return true;
}

// Feed it what the local DialogMenu shows every poll; true = a new line to send. The menu keeps showing a line until the next one, so
// a line is sent once; closing the menu forgets it (the same greeting next time is a new line).
class Watch {
public:
    bool update(bool menuOpen, uint32_t npc, const std::string& text) {
        if (!menuOpen) { npc_ = 0; text_.clear(); return false; }
        if (text.empty() || (npc == npc_ && text == text_)) return false;
        npc_ = npc;
        text_ = text;
        return true;
    }
private:
    uint32_t npc_ = 0;
    std::string text_;
};

// The reply the player picked: the DialogMenu's spoken INFO changes once per pick (its prompt, or its topic's name, is what the player
// clicked). True = a new pick with text to send, sent as its own Line ("<player>: <reply>") before the NPC's answer. A greeting and other
// lines the NPC starts have no text and are skipped; closing the menu forgets the last INFO.
class ReplyWatch {
public:
    bool update(bool menuOpen, uint32_t info, const std::string& replyText) {
        if (!menuOpen) { info_ = 0; return false; }
        if (!info || info == info_) return false;
        info_ = info;
        return !replyText.empty();
    }
private:
    uint32_t info_ = 0;
};

// HUD messages are one short line each: "Lucas Simms (to Host): ..." cut at word boundaries into pieces of at most `width` characters.
inline std::vector<std::string> chunks(const Line& l, size_t width = 160) {
    std::string all = l.speaker.empty() ? std::string() : l.speaker;
    if (!l.player.empty()) all += (all.empty() ? "(to " : " (to ") + l.player + ")";
    if (!all.empty()) all += ": ";
    all += l.text;
    std::vector<std::string> out;
    size_t at = 0;
    while (at < all.size()) {
        while (at < all.size() && all[at] == ' ') ++at;
        if (at >= all.size()) break;
        size_t end = at + width;
        if (end >= all.size()) { out.push_back(all.substr(at)); break; }
        size_t cut = all.rfind(' ', end);
        if (cut == std::string::npos || cut <= at) cut = end;   // one long word: hard cut
        out.push_back(all.substr(at, cut - at));
        at = cut;
    }
    return out;
}

// How long a HUD piece stays: about as long as the voice takes to say it (~14 characters a second), at least 2 s; a reply (not voiced in
// FO3 / FNV) 1.5 s, so the NPC's answer follows straight away.
inline float seconds(const std::string& piece, bool reply = false) {
    if (reply) return 1.5f;
    const float s = float(piece.size()) / 14.0f;
    return s < 2.0f ? 2.0f : s;
}

}}}  // namespace nvcoop::coop::talksubs
