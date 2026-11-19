#include "lobby.h"

#include <cctype>
#include <cstdlib>
#include <sstream>

#include "fnvcoop_mut.h"

namespace nvlobby {
namespace {

std::string trim(const std::string& s) {
    size_t a = 0, b = s.size();
    while (a < b && std::isspace(static_cast<unsigned char>(s[a]))) ++a;
    while (b > a && std::isspace(static_cast<unsigned char>(s[b - 1]))) --b;
    return s.substr(a, b - a);
}

std::string lower(std::string s) {
    for (char& c : s) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    return s;
}

// one address in a canonical form for duplicate checks: host lower-cased, port always present
std::string canon(const std::string& address) {
    std::string h, why;
    uint16_t p = 0;
    if (!ParseAddress(address, &h, &p, &why)) return lower(trim(address));
    return lower(h) + ":" + std::to_string(p);
}

}  // namespace

std::vector<Server> ParseServers(const std::string& iniText) {
    std::vector<Server> out;
    std::istringstream in(iniText);
    std::string line;
    bool inServers = false;
    while (std::getline(in, line)) {
        std::string t = trim(line);
        if (t.empty() || t[0] == ';' || t[0] == '#') continue;
        if (t.front() == '[') {
            inServers = lower(t) == "[servers]";
            continue;
        }
        if (!inServers) continue;
        size_t eq = t.find('=');
        if (eq == std::string::npos) continue;
        Server s{trim(t.substr(0, eq)), trim(t.substr(eq + 1))};
        if (s.name.empty() || s.address.empty()) continue;
        bool dup = false;
        for (const Server& o : out) dup = dup || (!FNV_MUT(1) && canon(o.address) == canon(s.address));
        if (!dup) out.push_back(std::move(s));
    }
    return out;
}

Menu BuildMenu(const std::vector<Server>& servers, const std::string& lastHost, uint16_t lastPort, const std::string& currentRole,
               bool enabled) {
    Menu m;
    std::string now = !enabled ? "Co-op is off: loading a save plays alone."
                      : currentRole == "client" ? "Loading a save joins " + (lastHost.empty() ? std::string("the host in nvcoop.ini") : lastHost) + "."
                                                : "Loading a save hosts a game for your friends.";
    m.text = "FNV Co-Op\n\n" + now + "\nOr pick one now. Direct connect: open the console (~) and type  coopjoin <address>";
    auto add = [&](const std::string& label, Action a) {
        m.buttons.push_back(label);
        m.actions.push_back(std::move(a));
    };
    add("Host a game", Action{Kind::Host, ""});
    // room for the servers: everything but Host, Direct and Play alone
    const size_t room = kMaxButtons - (FNV_MUT(2) ? 0 : 3);
    std::vector<Server> list;
    if (!lastHost.empty()) {
        std::string last = lastPort && lastPort != kDefaultPort ? lastHost + ":" + std::to_string(lastPort) : lastHost;
        bool listed = false;
        for (const Server& s : servers) listed = listed || canon(s.address) == canon(last);
        if (!listed) list.push_back(Server{last, last});
    }
    for (const Server& s : servers) list.push_back(s);
    for (const Server& s : list) {
        if (m.buttons.size() - 1 >= room) break;
        add("Join " + s.name, Action{Kind::Join, s.address});
    }
    add("Direct connect", Action{Kind::Direct, ""});
    add("Play alone", Action{Kind::Solo, ""});
    return m;
}

bool ParseAddress(const std::string& text, std::string* host, uint16_t* port, std::string* why) {
    std::string t = trim(text);
    if (t.size() >= 2 && t.front() == '"' && t.back() == '"') t = trim(t.substr(1, t.size() - 2));
    if (t.empty()) { if (why) *why = "type the host's address, like 203.0.113.5 or 203.0.113.5:27960"; return false; }
    if (t.find("://") != std::string::npos || t.find(' ') != std::string::npos) {
        if (why) *why = "type just the address and optional port, like 203.0.113.5:27960";
        return false;
    }
    std::string h = t;
    uint32_t p = kDefaultPort;
    size_t colon = t.rfind(':');
    if (colon != std::string::npos && t.find(':') == colon) {   // one colon = host:port (IPv6 is not supported by the transport)
        h = t.substr(0, colon);
        std::string ps = t.substr(colon + 1);
        if (ps.empty() || ps.size() > 5) { if (why) *why = "the port must be a number from 1 to 65535"; return false; }
        p = 0;
        for (char c : ps) {
            if (!std::isdigit(static_cast<unsigned char>(c))) { if (why) *why = "the port must be a number from 1 to 65535"; return false; }
            p = p * 10 + static_cast<uint32_t>(c - '0');
        }
        if (p == 0 || (!FNV_MUT(3) && p > 65535)) { if (why) *why = "the port must be a number from 1 to 65535"; return false; }
    } else if (colon != std::string::npos) {
        if (why) *why = "IPv6 addresses are not supported; use an IPv4 address or a host name";
        return false;
    }
    if (h.empty()) { if (why) *why = "the address has no host part"; return false; }
    for (char c : h)
        if (!(std::isalnum(static_cast<unsigned char>(c)) || c == '.' || c == '-')) {
            if (why) *why = "the address can only hold letters, digits, dots and dashes";
            return false;
        }
    if (host) *host = h;
    if (port) *port = static_cast<uint16_t>(p);
    return true;
}

std::vector<std::pair<std::string, std::string>> CoopKeysFor(const Action& a, uint64_t token, uint64_t newToken) {
    std::vector<std::pair<std::string, std::string>> kv;
    switch (a.kind) {
    case Kind::Host:
        kv = {{"enable", "1"}, {"role", "host"}, {"listen", "0.0.0.0"}};
        break;
    case Kind::Join: {
        std::string h, why;
        uint16_t p = kDefaultPort;
        if (!ParseAddress(a.address, &h, &p, &why)) return {};
        kv = {{"enable", "1"}, {"role", "client"}, {"host", h}, {"port", std::to_string(p)}};
        if (!token && !FNV_MUT(4)) kv.push_back({"token", std::to_string(newToken ? newToken : 1)});
        break;
    }
    case Kind::Solo:
        kv = {{"enable", "0"}};
        break;
    case Kind::Direct:
        break;
    }
    return kv;
}

bool ParseCommand(const std::string& name, const std::string& arg, Action* out, std::string* why) {
    const std::string n = lower(trim(name));
    Action a;
    if (n == "coophost") a.kind = Kind::Host;
    else if (n == "coopsolo") a.kind = Kind::Solo;
    else if (n == "coopjoin") {
        std::string h;
        uint16_t p = 0;
        if (!ParseAddress(arg, &h, &p, why)) return false;
        a.kind = Kind::Join;
        a.address = p == kDefaultPort ? h : h + ":" + std::to_string(p);
    } else {
        if (why) *why = "unknown command " + name;
        return false;
    }
    if (out) *out = a;
    return true;
}

}  // namespace nvlobby
