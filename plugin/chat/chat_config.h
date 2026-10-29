// nvcoop.ini [chat]. Every key is optional; a bad value keeps the default and is named in `warnings`.
//   enabled=1          0 turns the chat off completely (no key hook, nothing on the channel)
//   key=Y              the key that opens the line: a letter / digit, F1..F24, Tab, Insert, Backslash, ... or a number (virtual key code)
//   name=              the name other players see; empty = "Player 2" by slot. (The launcher fills this in from the lobby name.)
//   display=auto       auto | hud | overlay: where the lines are drawn (see docs/chat.md)
//   seconds=8          how long a new line stays on screen (2..60)
//   lines=6            lines on screen together (1..12)
//   history=40         lines kept; the newest are shown while typing (10..200)
//   rate_lines=5       a player may say this many lines ... (1..20)
//   rate_seconds=10    ... in this many seconds (2..120)
#pragma once
#include <functional>
#include <string>

#include "chat/chat_types.h"

namespace nvcoop::chat {

using IniGetter = std::function<std::string(const char* section, const char* key, const char* def)>;

struct ChatSettings {
    Config cfg;
    std::string warnings;
};

ChatSettings readSettings(const IniGetter& get);
std::string describe(const Config& c);

}  // namespace nvcoop::chat
