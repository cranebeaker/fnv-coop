// In-game co-op lobby (NV:MP style): pick Host / Join / Play alone at the main menu, or type coopjoin <address> in the console.
//
// Why at the main menu: the hooks, the UI hooks, the world hooks and the session read [coop] role once, when they install
// (DeferredInit / first frame). main.cpp holds that init back while the lobby is open ([coop] menu=1), writes the choice into
// nvcoop.ini, then lets the modules start with it. Loading a save or starting a new game without a choice keeps the role that is
// already in nvcoop.ini (host by default), so a host's server comes up with no clicks.
//
// This file is the logic only (no engine, no Windows): the menu buttons, the address check, and the nvcoop.ini keys for a choice.
#pragma once
#include <cstdint>
#include <string>
#include <utility>
#include <vector>

namespace nvlobby {

constexpr uint16_t kDefaultPort = 27960;
constexpr size_t kMaxButtons = 10;   // the engine's message box takes at most 10 buttons

struct Server {
    std::string name;      // shown on the button
    std::string address;   // "203.0.113.5" or "203.0.113.5:27960" or "friend.example.net:30000"
};

enum class Kind { Host, Join, Direct, Solo };

struct Action {
    Kind kind = Kind::Solo;
    std::string address;   // Join only
};

struct Menu {
    std::string text;                   // message box body
    std::vector<std::string> buttons;   // button labels, at most kMaxButtons
    std::vector<Action> actions;        // same size as buttons
};

// [servers] section of nvcoop.ini: one "name=address" per line (order kept; blank names or addresses skipped; duplicates by
// address dropped). Lines outside [servers] are ignored.
std::vector<Server> ParseServers(const std::string& iniText);

// The main-menu box: Host, Join <last host> (when lastHost is set and not already in the list), Join <each server>,
// Direct connect, Play alone. Servers that do not fit in the 10 buttons are left out (Host, Direct and Play alone always fit).
// currentRole: "host" / "client" from nvcoop.ini, shown in the text so the player knows what loading a save will do.
Menu BuildMenu(const std::vector<Server>& servers, const std::string& lastHost, uint16_t lastPort, const std::string& currentRole,
               bool enabled);

// "1.2.3.4", "1.2.3.4:27960", "host.name:30000" -> host + port (27960 when none). false + a short reason otherwise.
bool ParseAddress(const std::string& text, std::string* host, uint16_t* port, std::string* why);

// The [coop] keys to write for a choice. token: the client's slot token from nvcoop.ini (0 = none yet; newToken is used then and
// written, so the player gets the same slot on a rejoin). Direct writes nothing (it only explains the console command).
std::vector<std::pair<std::string, std::string>> CoopKeysFor(const Action& a, uint64_t token, uint64_t newToken);

// The console command text: "coophost", "coopjoin <address>", "coopsolo". Returns false (with a message) for an unknown or
// malformed one. Used by the tests and by the console command handlers.
bool ParseCommand(const std::string& name, const std::string& arg, Action* out, std::string* why);

}  // namespace nvlobby
