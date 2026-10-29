#include "chat/chat_text.h"

#include <cctype>
#include <cstdlib>

#include "chat/chat_types.h"
#include "fnvcoop_mut.h"

// Runtime mutation twins (FNVCOOP_MUT=<n>, tools/mut/fnvcoop_mut.h) for plugin\chat\run_tests.cmd (each must make the tests fail); chat twins are 1001..1099:
//   1001 = control characters are kept
//   1002 = '%' is kept (a format string reaches the game's text code)
//   1003 = '&' '<' '>' are kept (markup reaches the game's text tiles)
//   1004 = runs of spaces are not collapsed
//   1005 = the text is not cut at maxBytes
//   1006 = the high Latin-1 range is cut away too (names with an accent lose it)
//   1007 = a name is not cut at kMaxNameBytes
//   1008 = wrap lets a word longer than the width through whole
namespace nvcoop::chat {

bool isChatByte(unsigned char c) {
    if (FNV_MUT(1006)) return c >= 0x20 && c <= 0x7E;
    return (c >= 0x20 && c <= 0x7E) || c >= 0xA0;
}

char mapChatByte(unsigned char c) {
    if (c == '\t' || c == '\n' || c == '\r') c = ' ';
    if (!isChatByte(c)) return 0;
    switch (c) {
        case '%': return FNV_MUT(1002) ? '%' : '*';
        case '&': return FNV_MUT(1003) ? '&' : '+';
        case '<': return FNV_MUT(1003) ? '<' : '(';
        case '>': return FNV_MUT(1003) ? '>' : ')';
        case '\\': return '/';
        default: return static_cast<char>(c);
    }
}

std::string sanitizeText(std::string_view in, size_t maxBytes) {
    std::string out;
    out.reserve(in.size() < maxBytes ? in.size() : maxBytes);
    for (char ch : in) {
        const unsigned char raw = static_cast<unsigned char>(ch);
        if (FNV_MUT(1001) && raw < 0x20 && raw != '\t' && raw != '\n' && raw != '\r') {
            if (out.size() < maxBytes) out.push_back(ch);
            continue;
        }
        const char c = mapChatByte(raw);
        if (!c) continue;
        if (c == ' ' && !FNV_MUT(1004) && (out.empty() || out.back() == ' ')) continue;
        if (out.size() >= maxBytes && !FNV_MUT(1005)) break;
        out.push_back(c);
    }
    while (!out.empty() && out.back() == ' ') out.pop_back();
    return out;
}

std::string sanitizeName(std::string_view in) { return sanitizeText(in, FNV_MUT(1007) ? in.size() : kMaxNameBytes); }

std::vector<std::string> wrapText(std::string_view text, size_t width, size_t indent) {
    std::vector<std::string> out;
    if (width == 0 || text.size() <= width) {
        out.emplace_back(text);
        return out;
    }
    const size_t pad = indent + 8 < width ? indent : 0;   // an indent that leaves no room is dropped
    std::string cur;
    bool hasWord = false;
    auto flush = [&] {
        out.push_back(cur);
        cur.assign(pad, ' ');
        hasWord = false;
    };
    size_t i = 0;
    while (i < text.size()) {
        size_t j = text.find(' ', i);
        if (j == std::string_view::npos) j = text.size();
        std::string_view word = text.substr(i, j - i);
        i = j + 1;
        if (word.empty()) continue;
        if (hasWord && cur.size() + 1 + word.size() > width) flush();
        if (hasWord) cur.push_back(' ');
        while (cur.size() + word.size() > width && !FNV_MUT(1008)) {   // a word longer than a line is cut
            const size_t room = width - cur.size();
            cur.append(word.substr(0, room));
            word.remove_prefix(room);
            flush();
        }
        cur.append(word);
        hasWord = true;
    }
    if (hasWord) out.push_back(cur);
    return out;
}

namespace {
struct KeyRow { const char* name; int vk; };
constexpr KeyRow kKeys[] = {
    {"Tab", 0x09}, {"Backspace", 0x08}, {"Pause", 0x13}, {"Space", 0x20}, {"PageUp", 0x21}, {"PageDown", 0x22}, {"End", 0x23}, {"Home", 0x24},
    {"Insert", 0x2D}, {"Delete", 0x2E}, {"Semicolon", 0xBA}, {"Equals", 0xBB}, {"Comma", 0xBC}, {"Minus", 0xBD}, {"Period", 0xBE},
    {"Slash", 0xBF}, {"Backquote", 0xC0}, {"LeftBracket", 0xDB}, {"Backslash", 0xDC}, {"RightBracket", 0xDD}, {"Quote", 0xDE},
    {"Numpad0", 0x60}, {"Numpad1", 0x61}, {"Numpad2", 0x62}, {"Numpad3", 0x63}, {"Numpad4", 0x64}, {"Numpad5", 0x65}, {"Numpad6", 0x66},
    {"Numpad7", 0x67}, {"Numpad8", 0x68}, {"Numpad9", 0x69},
};
bool ieq(std::string_view a, std::string_view b) {
    if (a.size() != b.size()) return false;
    for (size_t i = 0; i < a.size(); ++i)
        if (std::tolower(static_cast<unsigned char>(a[i])) != std::tolower(static_cast<unsigned char>(b[i]))) return false;
    return true;
}
}  // namespace

int parseKeyName(std::string_view s) {
    while (!s.empty() && s.front() == ' ') s.remove_prefix(1);
    while (!s.empty() && s.back() == ' ') s.remove_suffix(1);
    if (s.empty()) return 0;
    if (s.size() == 1) {
        const unsigned char c = static_cast<unsigned char>(s[0]);
        if (std::isalpha(c)) return std::toupper(c);
        if (std::isdigit(c)) return c;
    }
    if ((s[0] == 'F' || s[0] == 'f') && s.size() <= 3 && std::isdigit(static_cast<unsigned char>(s[1]))) {
        const int n = std::atoi(std::string(s.substr(1)).c_str());
        if (n >= 1 && n <= 24) return 0x70 + n - 1;
    }
    for (const KeyRow& k : kKeys) if (ieq(s, k.name)) return k.vk;
    char* end = nullptr;
    const std::string str(s);
    const long v = std::strtol(str.c_str(), &end, 0);
    if (end && *end == 0 && v > 0 && v < 256) return static_cast<int>(v);
    return 0;
}

std::string keyName(int vk) {
    if ((vk >= 'A' && vk <= 'Z') || (vk >= '0' && vk <= '9')) return std::string(1, static_cast<char>(vk));
    if (vk >= 0x70 && vk <= 0x87) return "F" + std::to_string(vk - 0x70 + 1);
    for (const KeyRow& k : kKeys) if (k.vk == vk) return k.name;
    return std::to_string(vk);
}

}  // namespace nvcoop::chat
