#include "parity/parity_json.h"

#include <cstdio>
#include <cstring>

#include "parity/parity_anim.h"
#include "fnvcoop_mut.h"

#ifdef _WIN32
#include <Windows.h>
#endif

// Twin 1815: the file is never rotated.
namespace nvcoop::parity {

namespace {

void esc(std::string& o, const std::string& s) {
    o += '"';
    for (char c : s) {
        const unsigned char u = static_cast<unsigned char>(c);
        if (c == '"' || c == '\\') { o += '\\'; o += c; }
        else if (u < 0x20) { char b[8]; std::snprintf(b, sizeof b, "\\u%04x", u); o += b; }
        else o += c;
    }
    o += '"';
}

std::string hex(uint32_t v) {
    char b[16];
    std::snprintf(b, sizeof b, "0x%08X", static_cast<unsigned>(v));
    return b;
}

}  // namespace

std::string sampleJson(const Sample& s, const LineMeta& m) {
    std::string o;
    o.reserve(256 + s.ents.size() * 150);
    char b[256];
    std::snprintf(b, sizeof b, "{\"v\":1,\"k\":\"s\",\"role\":\"%s\",\"slot\":%u,\"seq\":%u,\"ms\":%llu,\"wall\":%llu,\"gt\":%.4f,", m.role, s.slot, s.seq,
                  static_cast<unsigned long long>(s.ms), static_cast<unsigned long long>(s.wallMs), s.gameTime);
    o += b;
    o += "\"cell\":\"" + hex(s.cell) + "\",\"ws\":\"" + hex(s.ws) + "\",";
    std::snprintf(b, sizeof b, "\"rad\":%u,\"cov\":%u,\"menu\":%d,\"why\":\"%s\"", s.rad, s.cov, s.menu ? 1 : 0, m.why);
    o += b;
    if (m.cmpSlot >= 0) {
        std::snprintf(b, sizeof b, ",\"cmp\":{\"slot\":%d,\"seq\":%lld}", m.cmpSlot, static_cast<long long>(m.cmpSeq));
        o += b;
    }
    o += ",\"e\":[";
    bool first = true;
    for (const Ent& e : s.ents) {
        if (!first) o += ',';
        first = false;
        const char t = e.kind == EntKind::Npc ? 'n' : e.kind == EntKind::Puppet ? 'p' : 's';
        std::snprintf(b, sizeof b, "{\"t\":\"%c\",", t);
        o += b;
        if (e.kind != EntKind::Npc) {
            std::snprintf(b, sizeof b, "\"sl\":%u,", e.slot);
            o += b;
        }
        o += "\"r\":\"" + hex(e.ref) + "\",\"b\":\"" + hex(e.base) + "\",";
        std::snprintf(b, sizeof b, "\"x\":%d,\"y\":%d,\"z\":%d,\"sp\":%u,\"se\":%d,\"d\":%d,\"dis\":%d,\"w\":%d,", e.x, e.y, e.z, unsigned(e.speed16) * 16u, e.seated ? 1 : 0,
                      e.dead ? 1 : 0, e.disabled ? 1 : 0, e.weaponOut ? 1 : 0);
        o += b;
        std::snprintf(b, sizeof b, "\"hp\":%d,\"g\":%d", e.hp == kNoHp ? -1 : int(e.hp), e.group == kNoGroup ? -1 : int(e.group));
        o += b;
        if (e.group != kNoGroup) {
            const char* gn = nvcoop::actors::anim::groupName(e.group);
            o += ",\"gn\":";
            esc(o, gn ? gn : "");
            o += std::string(",\"gc\":\"") + className(groupClass(e.group)) + "\"";
        }
        if (e.kind != EntKind::Npc) {
            std::snprintf(b, sizeof b, ",\"ac\":%u", e.acts);
            o += b;
            if (e.look.known) {
                std::snprintf(b, sizeof b, ",\"look\":{\"race\":\"%s\",\"f\":%d,", hex(e.look.race).c_str(), e.look.female ? 1 : 0);
                o += b;
                o += "\"hair\":\"" + hex(e.look.hair) + "\",\"eyes\":\"" + hex(e.look.eyes) + "\",\"fh\":\"" + hex(e.look.faceHash) + "\"}";
            }
        }
        o += '}';
    }
    o += "]}";
    return o;
}

// ---- file -------------------------------------------------------------------------------------------------------------------------------------------------

bool RotatingFile::openFile(bool truncate) {
    if (f_) { std::fclose(f_); f_ = nullptr; }
    f_ = std::fopen(path_.c_str(), truncate ? "wb" : "ab");
    if (!f_) return false;
    std::fseek(f_, 0, SEEK_END);
    const long pos = std::ftell(f_);
    bytes_ = pos > 0 ? static_cast<uint64_t>(pos) : 0;
    return true;
}

void RotatingFile::roll() {
    const std::string old = path_ + ".1";
#ifdef _WIN32
    MoveFileExA(path_.c_str(), old.c_str(), MOVEFILE_REPLACE_EXISTING);
#else
    std::rename(path_.c_str(), old.c_str());
#endif
    ++rotations_;
}

void RotatingFile::open(const std::string& path, uint64_t maxBytes, bool rollExisting) {
    close();
    path_ = path;
    max_ = maxBytes;
    if (path_.empty()) return;
    if (rollExisting) {
        if (std::FILE* t = std::fopen(path_.c_str(), "rb")) {
            std::fseek(t, 0, SEEK_END);
            const long sz = std::ftell(t);
            std::fclose(t);
            if (sz > 0) roll();
        }
    }
    openFile(false);
}

void RotatingFile::close() {
    if (f_) { std::fclose(f_); f_ = nullptr; }
}

bool RotatingFile::append(const std::string& line) {
    if (!f_) return false;
    if (bytes_ + line.size() + 1 > max_ && bytes_ > 0 && !FNV_MUT(1815)) {
        std::fclose(f_);
        f_ = nullptr;
        roll();
        if (!openFile(true)) return false;
    }
    if (std::fwrite(line.data(), 1, line.size(), f_) != line.size() || std::fputc('\n', f_) == EOF) return false;
    std::fflush(f_);
    bytes_ += line.size() + 1;
    return true;
}

}  // namespace nvcoop::parity
