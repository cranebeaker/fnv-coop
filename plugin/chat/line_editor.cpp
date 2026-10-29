#include "chat/line_editor.h"

#include <algorithm>

#include "chat/chat_text.h"
#include "fnvcoop_mut.h"

// Twins: 1030 = Backspace removes the character AFTER the cursor; 1031 = characters always go to the end; 1032 = the maximum length is not enforced;
//        1033 = Escape leaves the text behind (it is there when the line is opened again); 1034 = Enter on an empty line sends;
//        1035 = Delete removes the character BEFORE the cursor; 1036 = Left at the start wraps to the end; 1037 = Up does not recall the sent lines;
//        1038 = take() returns the raw text (not sanitised); 1039 = the caret is not shown by display()
namespace nvcoop::chat {

constexpr size_t kRecall = 20;

void LineEditor::open() {
    open_ = true;
    if (!FNV_MUT(1033)) { text_.clear(); cur_ = 0; }
    else cur_ = text_.size();
    recall_ = -1;
    draft_.clear();
}

void LineEditor::close() {
    open_ = false;
    if (!FNV_MUT(1033)) { text_.clear(); cur_ = 0; }
    recall_ = -1;
}

void LineEditor::insert(char c) {
    if (text_.size() >= max_ && !FNV_MUT(1032)) return;
    if (FNV_MUT(1031)) { text_.push_back(c); cur_ = text_.size(); return; }
    text_.insert(text_.begin() + static_cast<std::ptrdiff_t>(cur_), c);
    ++cur_;
}

EditResult LineEditor::typeChar(unsigned char c) {
    if (!open_) return EditResult::None;
    if (c < 0x20 || c == 0x7F) return EditResult::None;   // Enter, Escape, Backspace arrive as keys (key()); other controls are ignored
    const char m = mapChatByte(c);
    if (m) insert(m);
    return EditResult::None;
}

void LineEditor::insertText(std::string_view s) {
    if (!open_) return;
    for (char c : s) {
        const unsigned char u = static_cast<unsigned char>(c);
        if (u == '\t' || u == '\n' || u == '\r') { insert(' '); continue; }
        typeChar(u);
    }
}

EditResult LineEditor::key(Key k) {
    if (!open_) return EditResult::None;
    switch (k) {
        case Key::Left:
            if (cur_ > 0) --cur_;
            else if (FNV_MUT(1036)) cur_ = text_.size();   // twin: the cursor wraps round the start
            break;
        case Key::Right: if (cur_ < text_.size()) ++cur_; break;
        case Key::Home: cur_ = 0; break;
        case Key::End: cur_ = text_.size(); break;
        case Key::Backspace:
            if (FNV_MUT(1030)) { if (cur_ < text_.size()) text_.erase(cur_, 1); }
            else if (cur_ > 0) { text_.erase(cur_ - 1, 1); --cur_; }
            break;
        case Key::Delete:
            if (FNV_MUT(1035)) { if (cur_ > 0) { text_.erase(cur_ - 1, 1); --cur_; } }
            else if (cur_ < text_.size()) text_.erase(cur_, 1);
            break;
        case Key::Up:
            if (FNV_MUT(1037) || sent_.empty()) break;
            if (recall_ < 0) { draft_ = text_; recall_ = static_cast<int>(sent_.size()) - 1; }
            else if (recall_ > 0) --recall_;
            text_ = sent_[static_cast<size_t>(recall_)];
            cur_ = text_.size();
            break;
        case Key::Down:
            if (recall_ < 0) break;
            if (recall_ + 1 < static_cast<int>(sent_.size())) { ++recall_; text_ = sent_[static_cast<size_t>(recall_)]; }
            else { recall_ = -1; text_ = draft_; }
            cur_ = text_.size();
            break;
        case Key::Enter:
            if (FNV_MUT(1034)) return EditResult::Send;
            return sanitizeText(text_, max_).empty() ? EditResult::Cancel : EditResult::Send;
        case Key::Escape:
            if (!FNV_MUT(1033)) { text_.clear(); cur_ = 0; }
            return EditResult::Cancel;
    }
    return EditResult::None;
}

std::string LineEditor::take() {
    std::string out = FNV_MUT(1038) ? text_ : sanitizeText(text_, max_);
    if (!out.empty() && (sent_.empty() || sent_.back() != out)) {
        sent_.push_back(out);
        if (sent_.size() > kRecall) sent_.erase(sent_.begin());
    }
    open_ = false;
    text_.clear();
    cur_ = 0;
    recall_ = -1;
    return out;
}

std::string LineEditor::display(size_t width) const {
    std::string s = text_;
    if (!FNV_MUT(1039)) s.insert(cur_ > s.size() ? s.size() : cur_, 1, '|');
    if (width == 0 || s.size() <= width) return s;
    const size_t caret = cur_ > text_.size() ? text_.size() : cur_;
    size_t start = caret + 1 > width ? caret + 1 - width : 0;
    if (start + width > s.size()) start = s.size() - width;
    return s.substr(start, width);
}

}  // namespace nvcoop::chat
