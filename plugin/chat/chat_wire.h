// Chat datagrams (channel 0xC8 on the co-op mux, always reliable). Layout, all integers little endian:
//   [0xC8][version=1][kind u8] then
//     Say    (guest -> host)   : msgId u32, textLen u8, text            the sender is the peer the host got it from
//     Line   (host -> guests)  : slot u8, msgId u32, lineKind u8, nameLen u8, name, textLen u8, text
//                                slot = the origin player (0xFF for a system line), msgId = the origin's own number, lineKind 0 player / 1 system
//     Hello  (guest -> host)   : nameLen u8, name                        sent when the guest has joined; the host answers with a "joined" line
//     Notice (host -> guest)   : code u8                                 1 = your last line was refused, you are sending too fast
// A decoder takes exactly the bytes of one message (no trailing bytes), lengths within kMaxNameBytes / kMaxTextBytes, a known kind. It does NOT trust the
// text: the receiver sanitises it again (chat_text.h).
#pragma once
#include "chat/chat_types.h"

namespace nvcoop::chat {

enum class Kind : uint8_t { Say = 1, Line = 2, Hello = 3, Notice = 4 };
constexpr uint8_t kNoticeRateLimited = 1;

struct Message {
    Kind kind = Kind::Say;
    uint32_t msgId = 0;           // Say, Line
    Slot slot = kSystemSlot;      // Line
    LineKind lineKind = LineKind::Player;   // Line
    std::string name;             // Line, Hello
    std::string text;             // Say, Line
    uint8_t code = 0;             // Notice
};

Bytes encodeSay(uint32_t msgId, const std::string& text);
Bytes encodeLine(Slot slot, uint32_t msgId, LineKind lineKind, const std::string& name, const std::string& text);
Bytes encodeHello(const std::string& name);
Bytes encodeNotice(uint8_t code);
bool decode(const uint8_t* data, size_t size, Message& out);

}  // namespace nvcoop::chat
