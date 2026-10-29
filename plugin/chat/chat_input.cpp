#include "chat/chat_input.h"

#include "fnvcoop_mut.h"

// Twins: 1070 = the open key also opens with Ctrl / Alt held; 1071 = the open key opens in a menu (canOpen is not asked); 1072 = while typing the messages
//        pass to the game; 1073 = Enter and Escape act on every auto-repeat; 1074 = Escape does not cancel; 1075 = typed characters are not swallowed (the game
//        sees them too); 1076 = after Enter the translator stays in the typing state; 1077 = Ctrl+V is not a paste; 1078 = the open key's auto-repeat opens again
//        after a cancel; 1079 = a swallowed key down is not translated (no characters ever arrive)
namespace nvcoop::chat {

void InputTranslator::setOpenKey(int vk) { std::lock_guard<std::mutex> l(mu_); openKey_ = vk; }
void InputTranslator::setEnabled(bool on) {
    std::lock_guard<std::mutex> l(mu_);
    enabled_ = on;
    if (!on) composing_ = false;
}
void InputTranslator::setComposing(bool on) { std::lock_guard<std::mutex> l(mu_); composing_ = on; }
bool InputTranslator::composing() const { std::lock_guard<std::mutex> l(mu_); return composing_; }
uint64_t InputTranslator::swallowed() const { std::lock_guard<std::mutex> l(mu_); return swallowed_; }
void InputTranslator::drain(std::vector<InputEvent>& out) {
    std::lock_guard<std::mutex> l(mu_);
    out.insert(out.end(), events_.begin(), events_.end());
    events_.clear();
}

InputTranslator::Verdict InputTranslator::onMessage(uint32_t message, uint64_t wParam, int64_t lParam) {
    std::lock_guard<std::mutex> l(mu_);
    const bool repeat = (static_cast<uint64_t>(lParam) >> 30) & 1;
    auto swallow = [&](bool translate) {
        ++swallowed_;
        return translate && !FNV_MUT(1079) ? Verdict::SwallowAndTranslate : Verdict::Swallow;
    };
    if (!enabled_) return Verdict::Pass;

    if (!composing_) {
        if (message != kWmKeyDown || openKey_ == 0 || static_cast<int>(wParam) != openKey_) return Verdict::Pass;
        if (repeat && !FNV_MUT(1078)) return Verdict::Pass;
        if (!FNV_MUT(1070) && (held(kVkControl) || held(kVkMenu))) return Verdict::Pass;
        if (!FNV_MUT(1071) && deps_.canOpen && !deps_.canOpen()) return Verdict::Pass;
        events_.push_back(InputEvent{InputType::Open});
        composing_ = true;
        return swallow(false);
    }

    if (FNV_MUT(1072)) return Verdict::Pass;
    switch (message) {
        case kWmKeyDown: {
            const int vk = static_cast<int>(wParam);
            auto key = [&](Key k, bool closes) {
                if (repeat && closes && !FNV_MUT(1073)) return swallow(false);
                events_.push_back(InputEvent{InputType::Key, 0, k});
                if (closes && !FNV_MUT(1076)) composing_ = false;
                return swallow(false);
            };
            switch (vk) {
                case kVkReturn: return key(Key::Enter, true);
                case kVkEscape: return FNV_MUT(1074) ? swallow(false) : key(Key::Escape, true);
                case kVkBack: return key(Key::Backspace, false);
                case kVkDelete: return key(Key::Delete, false);
                case kVkLeft: return key(Key::Left, false);
                case kVkRight: return key(Key::Right, false);
                case kVkUp: return key(Key::Up, false);
                case kVkDown: return key(Key::Down, false);
                case kVkHome: return key(Key::Home, false);
                case kVkEnd: return key(Key::End, false);
                default: break;
            }
            if (vk == 'V' && held(kVkControl) && !held(kVkMenu)) {
                if (!FNV_MUT(1077) && !repeat) events_.push_back(InputEvent{InputType::Paste});
                return swallow(false);
            }
            if (held(kVkControl) && !held(kVkMenu)) return swallow(false);   // other Ctrl combinations make control characters we ignore
            return swallow(true);
        }
        case kWmKeyUp: return swallow(false);
        case kWmChar: {
            const unsigned c = static_cast<unsigned>(wParam & 0xFFFFu);
            if (c >= 0x20 && c <= 0xFF && c != 0x7F) events_.push_back(InputEvent{InputType::Char, static_cast<unsigned char>(c)});
            if (FNV_MUT(1075)) return Verdict::Pass;
            return swallow(false);
        }
        case kWmSysChar: return swallow(false);
        default: return Verdict::Pass;   // WM_SYSKEYDOWN / UP (Alt+Tab, Alt+F4) and everything else belong to Windows
    }
}

}  // namespace nvcoop::chat
