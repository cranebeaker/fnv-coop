// Text rules of the chat. Everything that comes from another PC (or from the keyboard) goes through sanitizeText before it is stored or shown.
#pragma once
#include <string>
#include <string_view>
#include <vector>

namespace nvcoop::chat {

// True for a byte the chat keeps: printable ASCII and the printable Latin-1 range (0xA0..0xFF, the game's western font; the game window is ANSI).
bool isChatByte(unsigned char c);
// One typed or received byte as the chat keeps it: tab and line breaks become a space, markup characters are replaced (see sanitizeText), 0 = dropped.
char mapChatByte(unsigned char c);

// A line as shown: only printable characters; tab, line breaks and runs of spaces become one space; no spaces at the ends; the characters the
// game's text code or a C formatter could read as markup are replaced ('%' by '*', '&' by '+', '<' '>' by '(' ')', '\\' by '/');
// cut at maxBytes. Empty: nothing to say. Never returns a control character.
std::string sanitizeText(std::string_view in, size_t maxBytes);
// A display name: the same rules, cut at kMaxNameBytes; "" when nothing is left.
std::string sanitizeName(std::string_view in);

// Word wrap for a surface with a fixed column width: breaks at spaces, cuts a word longer than the width. Each returned line is at most
// `width` bytes (width 0 = no wrap). The continuation lines of a wrapped text start with `indent` spaces when that leaves room.
std::vector<std::string> wrapText(std::string_view text, size_t width, size_t indent = 0);

// Key names for nvcoop.ini [chat] key=: "Y", "F9", "Tab", "Backslash", "Insert", "0x59" or "89". Returns a Windows virtual key, 0 = not understood.
int parseKeyName(std::string_view s);
std::string keyName(int vk);

}  // namespace nvcoop::chat
