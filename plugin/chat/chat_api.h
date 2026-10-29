// What other modules call to put a line into the chat (the party-save module for "party saved", a mod that wants to announce something ...). No game, no
// transport: it only reaches the running chat of this machine, and does nothing when the chat is off or not started.
//   postSystem  : on the host every machine shows it (a "* text" line); on a guest only this machine does.
//   postLocal   : only this machine, whichever role it has.
#pragma once
#include <functional>
#include <string>

namespace nvcoop::chat {

void postSystem(const std::string& text);
void postLocal(const std::string& text);
bool available();        // the chat is running and takes lines

// The chat module's side: the one place the two calls above end up. Null clears it.
using PostFn = std::function<void(const std::string& text, bool everyone)>;
void setPoster(PostFn f);

}  // namespace nvcoop::chat
