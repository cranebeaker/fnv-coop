// The chat line being typed: characters, backspace, delete, a cursor that moves, and a recall of the lines already sent (Up / Down). Game-free; the input
// glue feeds it what the window messages say.
#pragma once
#include <string>
#include <string_view>
#include <vector>

#include "chat/chat_types.h"

namespace nvcoop::chat {

enum class Key : uint8_t { Left, Right, Home, End, Delete, Backspace, Up, Down, Enter, Escape };
enum class EditResult : uint8_t { None, Send, Cancel };   // Send: take() has the text; Cancel: the line is gone

class LineEditor {
public:
    explicit LineEditor(size_t maxBytes = kMaxTextBytes) : max_(maxBytes) {}
    void open();                              // an empty line, cursor at the start
    void close();                             // the text is dropped (a sent line was already taken)
    bool isOpen() const { return open_; }

    EditResult typeChar(unsigned char c);     // a typed character (Windows' WM_CHAR code): printable ones are inserted at the cursor; the rest is ignored
    EditResult key(Key k);
    void insertText(std::string_view s);      // a paste: every byte as typeChar would take it, cut at the maximum length

    const std::string& text() const { return text_; }
    size_t cursor() const { return cur_; }
    // The sanitised text, the editor closes. Remembered for Up / Down. "" = nothing worth sending.
    std::string take();
    // The line as shown: the text with the caret (a '|') at the cursor, at most `width` bytes, scrolled so the caret stays in view.
    std::string display(size_t width) const;

private:
    void insert(char c);
    size_t max_;
    bool open_ = false;
    std::string text_;
    size_t cur_ = 0;
    std::vector<std::string> sent_;           // oldest first, at most kRecall
    int recall_ = -1;                         // index into sent_ while browsing, -1 = the line being typed
    std::string draft_;                       // the line being typed when the browsing began
};

}  // namespace nvcoop::chat
