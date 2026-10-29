// View, controller, input translator, key blocker, surfaces, module API. Twins 1070..1079, 1080..1087, 1090..1094, 1100..1107, 1120..1121, 1140..1146.
#include <set>

#include "chat/chat_api.h"
#include "chat/chat_controller.h"
#include "chat/chat_input.h"
#include "chat/chat_keyblock.h"
#include "chat/chat_surface.h"
#include "chat_rig.h"

using namespace nvchat;

namespace {

Line mkLine(const std::string& name, const std::string& text, uint64_t at, uint32_t id, Slot slot = 1) {
    Line l;
    l.slot = slot; l.msgId = id; l.kind = LineKind::Player; l.name = name; l.text = text; l.atMs = at;
    return l;
}
Line sysLine(const std::string& text, uint64_t at) {
    Line l;
    l.slot = kSystemSlot; l.kind = LineKind::System; l.text = text; l.atMs = at;
    return l;
}
InputEvent ch(char c) { return InputEvent{InputType::Char, static_cast<unsigned char>(c)}; }
InputEvent key(Key k) { return InputEvent{InputType::Key, 0, k}; }
InputEvent open() { return InputEvent{InputType::Open}; }
std::vector<InputEvent> typed(const std::string& s) {
    std::vector<InputEvent> v;
    for (char c : s) v.push_back(ch(c));
    return v;
}

// A session-less host core that records what it would send, enough for the controller tests.
struct ControllerRig {
    Hub hub;
    bool menu = false;
    std::string clip;
    ChatController ctl;
    ControllerRig() : hub(), ctl(hub.host, ControllerDeps{[this] { return menu; }, [this] { return clip; }}) { hub.join(1, "Ann"); }
    void feed(std::vector<InputEvent> e) { ctl.frame(hub.now, e); hub.pump(); }
};

}  // namespace

// ---- view -------------------------------------------------------------------------------------------------------------------------------------------

TEST(chat_view, shows_the_newest_unexpired_lines_with_names) {
    ChatLog log(50);
    LineEditor ed;
    ViewOptions o; o.ttlMs = 8000; o.visibleLines = 3; o.columns = 60;
    log.add(mkLine("Ann", "very old", 100, 1));
    log.add(mkLine("Ann", "one", 5000, 2));
    log.add(mkLine("Bob", "two", 6000, 3));
    log.add(sysLine("Cy joined", 6500));
    log.add(mkLine("Ann", "three", 7000, 4));
    View v = buildView(log, ed, 9000, o);
    REQUIRE(v.lines.size() == 3);
    CHECK(v.lines[0].text == "Bob: two" && v.lines[1].text == "* Cy joined" && v.lines[2].text == "Ann: three");
    CHECK(v.lines[1].system && !v.lines[0].system);
    CHECK(!v.composing && v.compose.empty());
    CHECK(buildView(log, ed, 60000, o).empty());
}

TEST(chat_view, the_last_stretch_fades) {
    ChatLog log(50);
    LineEditor ed;
    ViewOptions o; o.ttlMs = 8000; o.fadeMs = 1000;
    log.add(mkLine("A", "x", 1000, 1));
    CHECK(buildView(log, ed, 5000, o).lines[0].alpha == 255);
    const uint8_t mid = buildView(log, ed, 8500, o).lines[0].alpha;   // 500 ms left of 1000
    CHECK(mid > 100 && mid < 160);
    CHECK(buildView(log, ed, 8990, o).lines[0].alpha < 10);
}

TEST(chat_view, while_typing_the_short_history_is_shown_and_nothing_fades) {
    ChatLog log(50);
    LineEditor ed;
    ViewOptions o; o.ttlMs = 8000; o.visibleLines = 2; o.typingHistory = 4;
    for (uint32_t i = 1; i <= 6; ++i) log.add(mkLine("A", "m" + std::to_string(i), 100, i));
    ed.open();
    ed.typeChar('h'); ed.typeChar('i');
    View v = buildView(log, ed, 100000, o);   // every line is long expired
    REQUIRE(v.lines.size() == 4);
    CHECK(v.lines[0].text == "A: m3" && v.lines[3].text == "A: m6");
    for (auto& l : v.lines) CHECK(l.alpha == 255);
    CHECK(v.composing && v.compose == "> hi|");
}

TEST(chat_view, long_lines_wrap_and_the_total_is_capped) {
    ChatLog log(50);
    LineEditor ed;
    ViewOptions o; o.visibleLines = 2; o.columns = 20;
    log.add(mkLine("Ann", "the quick brown fox jumps over the lazy dog again and again", 1000, 1));
    log.add(mkLine("Bob", "short", 1000, 2));
    View v = buildView(log, ed, 1500, o);
    CHECK(v.lines.size() <= 4);   // never more than twice the logical lines
    for (auto& l : v.lines) CHECK(l.text.size() <= 20);
    CHECK(v.lines.back().text == "Bob: short");
    CHECK(v.lines.size() >= 2);
}

TEST(chat_view, a_wrapped_line_keeps_all_its_words) {
    ChatLog log(50);
    LineEditor ed;
    ViewOptions o; o.visibleLines = 3; o.columns = 24;
    log.add(mkLine("Ann", "alpha bravo charlie delta echo foxtrot golf hotel", 1000, 1));
    View v = buildView(log, ed, 1500, o);
    REQUIRE(v.lines.size() >= 3);
    std::string all;
    for (auto& l : v.lines) { CHECK(l.text.size() <= 24); all += l.text + " "; }
    for (const char* w : {"Ann:", "alpha", "delta", "foxtrot", "hotel"}) CHECK_MSG(all.find(w) != std::string::npos, w);
    CHECK(v.lines[1].text.compare(0, 2, "  ") == 0);   // continuation lines are indented under the name
}

TEST(chat_view, the_compose_line_scrolls_with_the_caret) {
    ChatLog log(5);
    LineEditor ed;
    ViewOptions o; o.columns = 30;
    ed.open();
    for (int i = 0; i < 80; ++i) ed.typeChar('a');
    View v = buildView(log, ed, 0, o);
    CHECK(v.compose.size() <= 30 && v.compose.compare(0, 2, "> ") == 0 && v.compose.back() == '|');
}

// ---- controller -------------------------------------------------------------------------------------------------------------------------------------

TEST(chat_controller, open_type_enter_says_it) {
    ControllerRig r;
    std::vector<InputEvent> e = {open()};
    for (auto& x : typed("gg wp")) e.push_back(x);
    e.push_back(key(Key::Enter));
    r.feed(e);
    CHECK(!r.ctl.composing());
    CHECK(has(r.hub.host.log(), "Player 1: gg wp"));
    CHECK(has(r.hub.g(1).log(), "Player 1: gg wp"));
    CHECK(r.ctl.sent() == 1 && r.ctl.opened() == 1);
}

TEST(chat_controller, escape_cancels_and_sends_nothing) {
    ControllerRig r;
    std::vector<InputEvent> e = {open()};
    for (auto& x : typed("never sent")) e.push_back(x);
    e.push_back(key(Key::Escape));
    r.feed(e);
    CHECK(!r.ctl.composing());
    CHECK(!has(r.hub.host.log(), "Player 1: never sent"));
    CHECK(r.ctl.cancelled() == 1 && r.ctl.sent() == 0);
}

TEST(chat_controller, the_line_edits_with_the_cursor_and_backspace) {
    ControllerRig r;
    std::vector<InputEvent> e = {open()};
    for (auto& x : typed("helo wrld")) e.push_back(x);
    for (int i = 0; i < 4; ++i) e.push_back(key(Key::Left));
    e.push_back(ch('o'));   // four lefts from the end of "helo wrld" put the cursor before "wrld"
    r.feed(e);
    CHECK(r.ctl.editor().text() == "helo owrld");
    r.feed({key(Key::Backspace), key(Key::End), key(Key::Backspace)});
    CHECK(r.ctl.editor().text() == "helo wrl");
}

TEST(chat_controller, enter_on_an_empty_line_just_closes) {
    ControllerRig r;
    r.feed({open(), key(Key::Enter)});
    CHECK(!r.ctl.composing() && r.ctl.sent() == 0);
}

TEST(chat_controller, a_menu_that_opens_closes_the_line_and_a_menu_blocks_opening) {
    ControllerRig r;
    r.menu = true;
    r.feed({open()});
    CHECK(!r.ctl.composing());   // Pip-Boy up: the key does nothing
    r.menu = false;
    r.feed({open(), ch('a')});
    CHECK(r.ctl.composing());
    r.menu = true;               // the game opens a menu (a death, a load, the Pip-Boy by a script)
    r.feed({});
    CHECK(!r.ctl.composing());
    CHECK(r.ctl.cancelled() == 1);
}

TEST(chat_controller, opening_twice_keeps_the_text) {
    ControllerRig r;
    r.feed({open(), ch('a'), ch('b'), open(), ch('c')});
    CHECK(r.ctl.editor().text() == "abc");
}

TEST(chat_controller, the_player_is_told_when_it_is_too_fast) {
    ControllerRig r;
    for (int i = 0; i < 6; ++i) {
        std::vector<InputEvent> e = {open(), ch('x'), key(Key::Enter)};
        r.feed(e);
    }
    CHECK(has(r.hub.host.log(), "* You are sending messages too fast. Wait a moment."));
    CHECK(r.ctl.sent() == 5);
}

TEST(chat_controller, outside_a_session_the_player_is_told_and_no_line_opens) {
    ChatCore lone(Config{}, 1);
    ChatController ctl(lone, ControllerDeps{});
    ctl.frame(1000, {open()});
    CHECK(!ctl.composing());
    CHECK(lone.log().lines().size() == 1 && lone.log().lines()[0].kind == LineKind::System);
}

TEST(chat_controller, a_paste_goes_into_the_line) {
    ControllerRig r;
    r.clip = "from the clipboard\nsecond line";
    r.feed({open(), ch('>'), InputEvent{InputType::Paste}});
    CHECK(r.ctl.editor().text() == ")from the clipboard second line");
}

TEST(chat_controller, up_recalls_the_last_line_sent) {
    ControllerRig r;
    r.feed({open(), ch('o'), ch('k'), key(Key::Enter)});
    r.feed({open(), key(Key::Up)});
    CHECK(r.ctl.editor().text() == "ok");
}

TEST(chat_controller, a_disabled_chat_does_not_open) {
    Config c; c.enabled = false;
    Hub h(c);
    h.join(1, "Ann");
    ChatController ctl(h.host, ControllerDeps{});
    ctl.frame(1000, {open()});
    CHECK(!ctl.composing());
}

// ---- input translator -------------------------------------------------------------------------------------------------------------------------------

namespace {
struct InRig {
    bool canOpen = true;
    std::set<int> down;
    InputTranslator t{InputTranslator::Deps{[this] { return canOpen; }, [this](int vk) { return down.count(vk) != 0; }}};
    InputTranslator::Verdict key(int vk, bool repeat = false) { return t.onMessage(kWmKeyDown, static_cast<uint64_t>(vk), repeat ? (1ll << 30) : 0); }
    InputTranslator::Verdict chr(unsigned c) { return t.onMessage(kWmChar, c, 0); }
    std::vector<InputEvent> take() { std::vector<InputEvent> v; t.drain(v); return v; }
};
using V = InputTranslator::Verdict;
}  // namespace

TEST(chat_input, the_open_key_opens_and_is_swallowed) {
    InRig r;
    CHECK(r.key('Y') == V::Swallow);
    auto e = r.take();
    REQUIRE(e.size() == 1);
    CHECK(e[0].type == InputType::Open);
    CHECK(r.t.composing());
}

TEST(chat_input, other_keys_pass_when_not_typing) {
    InRig r;
    CHECK(r.key('W') == V::Pass);
    CHECK(r.chr('w') == V::Pass);
    CHECK(r.t.onMessage(kWmKeyUp, 'Y', 0) == V::Pass);
    CHECK(r.take().empty());
}

TEST(chat_input, the_open_key_needs_no_modifier_no_repeat_and_no_menu) {
    InRig r;
    r.down.insert(kVkControl);
    CHECK(r.key('Y') == V::Pass);
    r.down.clear(); r.down.insert(kVkMenu);
    CHECK(r.key('Y') == V::Pass);
    r.down.clear();
    CHECK(r.key('Y', true) == V::Pass);       // held down from before
    r.canOpen = false;
    CHECK(r.key('Y') == V::Pass);             // a menu is up
    CHECK(r.take().empty());
    r.canOpen = true;
    CHECK(r.key('Y') == V::Swallow);
}

TEST(chat_input, a_different_open_key_can_be_set) {
    InRig r;
    r.t.setOpenKey(0x78);   // F9
    CHECK(r.key('Y') == V::Pass);
    CHECK(r.key(0x78) == V::Swallow);
    CHECK(r.take().size() == 1);
}

TEST(chat_input, a_disabled_chat_passes_everything) {
    InRig r;
    r.t.setEnabled(false);
    CHECK(r.key('Y') == V::Pass);
    CHECK(r.take().empty());
}

TEST(chat_input, while_typing_keys_do_not_reach_the_game_and_characters_arrive) {
    InRig r;
    r.key('Y'); r.take();
    CHECK(r.key('A') == V::SwallowAndTranslate);   // the glue turns it into a WM_CHAR
    CHECK(r.chr('a') == V::Swallow);
    CHECK(r.t.onMessage(kWmKeyUp, 'A', 0) == V::Swallow);
    CHECK(r.key('W') == V::SwallowAndTranslate);
    CHECK(r.key(0x20) == V::SwallowAndTranslate);   // space
    CHECK(r.chr('w') == V::Swallow);
    CHECK(r.chr(' ') == V::Swallow);
    auto e = r.take();
    REQUIRE(e.size() == 3);
    CHECK(e[0].ch == 'a' && e[1].ch == 'w' && e[2].ch == ' ');
    CHECK(r.t.onMessage(kWmSysChar, 'x', 0) == V::Swallow);
}

TEST(chat_input, control_characters_are_not_typed) {
    InRig r;
    r.key('Y'); r.take();
    for (unsigned c : {0x01u, 0x08u, 0x09u, 0x0Du, 0x16u, 0x1Bu, 0x7Fu}) CHECK(r.chr(c) == V::Swallow);
    CHECK(r.take().empty());
    r.chr(0xE9);
    auto e = r.take();
    REQUIRE(e.size() == 1);
    CHECK(e[0].ch == 0xE9);
}

TEST(chat_input, editing_keys_become_key_events) {
    InRig r;
    r.key('Y'); r.take();
    const std::pair<int, Key> keys[] = {{kVkBack, Key::Backspace}, {kVkDelete, Key::Delete}, {kVkLeft, Key::Left}, {kVkRight, Key::Right},
                                        {kVkHome, Key::Home}, {kVkEnd, Key::End}, {kVkUp, Key::Up}, {kVkDown, Key::Down}};
    for (auto& k : keys) CHECK(r.key(k.first) == V::Swallow);
    auto e = r.take();
    REQUIRE(e.size() == 8);
    for (size_t i = 0; i < 8; ++i) CHECK(e[i].type == InputType::Key && e[i].key == keys[i].second);
    CHECK(r.key(kVkBack, true) == V::Swallow);   // auto-repeat keeps deleting
    CHECK(r.take().size() == 1);
}

TEST(chat_input, enter_and_escape_end_the_typing) {
    InRig r;
    r.key('Y'); r.take();
    CHECK(r.key(kVkReturn) == V::Swallow);
    auto e = r.take();
    REQUIRE(e.size() == 1);
    CHECK(e[0].type == InputType::Key && e[0].key == Key::Enter);
    CHECK(!r.t.composing());
    CHECK(r.key('W') == V::Pass);   // the game has its keys back at once
    r.key('Y'); r.take();
    CHECK(r.key(kVkEscape) == V::Swallow);
    e = r.take();
    REQUIRE(e.size() == 1);
    CHECK(e[0].key == Key::Escape);
    CHECK(!r.t.composing());
}

TEST(chat_input, auto_repeat_does_not_send_or_cancel_twice) {
    InRig r;
    r.key('Y'); r.take();
    r.key(kVkReturn);
    r.take();
    r.t.setComposing(true);
    CHECK(r.key(kVkReturn, true) == V::Swallow);
    CHECK(r.take().empty());
}

TEST(chat_input, ctrl_v_is_a_paste_and_other_ctrl_keys_are_ignored) {
    InRig r;
    r.key('Y'); r.take();
    r.down.insert(kVkControl);
    CHECK(r.key('V') == V::Swallow);
    auto e = r.take();
    REQUIRE(e.size() == 1);
    CHECK(e[0].type == InputType::Paste);
    CHECK(r.key('C') == V::Swallow);
    CHECK(r.take().empty());
}

TEST(chat_input, alt_combinations_belong_to_windows) {
    InRig r;
    r.key('Y'); r.take();
    CHECK(r.t.onMessage(kWmSysKeyDown, 0x09, 0) == V::Pass);   // Alt+Tab
    CHECK(r.t.onMessage(kWmSysKeyUp, 0x09, 0) == V::Pass);
}

TEST(chat_input, the_controller_closing_the_line_gives_the_keys_back) {
    InRig r;
    r.key('Y'); r.take();
    r.t.setComposing(false);   // a menu opened
    CHECK(r.key('W') == V::Pass);
}

// ---- key blocker ------------------------------------------------------------------------------------------------------------------------------------

namespace {
struct ConsoleRig {
    std::vector<std::string> lines;
    bool ok = true;
    KeyBlocker::Exec exec() { return [this](const std::string& l) { lines.push_back(l); return ok; }; }
};
}  // namespace

TEST(chat_keyblock, disables_the_keys_over_a_few_frames_starting_with_the_important_ones) {
    ConsoleRig c;
    KeyBlocker b(c.exec(), 70);
    b.want(true);
    b.frame();
    CHECK(c.lines.size() == 70);
    CHECK(c.lines[0] == "DisableKey 1 2");   // Escape first
    bool wasd = false;
    for (auto& l : c.lines) wasd = wasd || l == "DisableKey 17 2";
    CHECK(wasd);
    CHECK(!b.settled());
    for (int i = 0; i < 5; ++i) b.frame();
    CHECK(b.settled());
    CHECK(b.disabledCount() == 263);
    CHECK(c.lines.size() == 263);
    b.frame();
    CHECK(c.lines.size() == 263);   // nothing more once settled
    CHECK(c.lines.back() == "DisableKey 263 2");   // the mouse buttons are in
}

TEST(chat_keyblock, enables_exactly_what_it_disabled) {
    ConsoleRig c;
    KeyBlocker b(c.exec(), 100);
    b.want(true);
    for (int i = 0; i < 4; ++i) b.frame();
    c.lines.clear();
    b.want(false);
    for (int i = 0; i < 4; ++i) b.frame();
    CHECK(b.settled() && b.disabledCount() == 0);
    CHECK(c.lines.size() == 263);
    std::set<std::string> uniq(c.lines.begin(), c.lines.end());
    CHECK(uniq.size() == 263);
    for (auto& l : c.lines) CHECK(l.compare(0, 10, "EnableKey ") == 0);
}

TEST(chat_keyblock, closing_in_the_middle_only_enables_what_was_disabled) {
    ConsoleRig c;
    KeyBlocker b(c.exec(), 70);
    b.want(true);
    b.frame();            // 70 keys
    c.lines.clear();
    b.want(false);
    b.frame();
    CHECK(c.lines.size() == 70);
    CHECK(b.settled());
    b.frame();
    CHECK(c.lines.size() == 70);
}

TEST(chat_keyblock, the_script_flag_only_is_used) {
    ConsoleRig c;
    KeyBlocker b(c.exec());
    b.want(true);
    b.frame();
    for (auto& l : c.lines) CHECK(l.size() > 2 && l.substr(l.size() - 2) == " 2");
}

TEST(chat_keyblock, a_console_that_refuses_ends_the_disabling_but_enabling_keeps_trying) {
    ConsoleRig c;
    KeyBlocker b(c.exec(), 70);
    b.want(true);
    c.ok = false;
    for (int i = 0; i < 5; ++i) b.frame();
    CHECK(b.failed());
    const size_t n = c.lines.size();
    CHECK(n <= 8);
    b.frame();
    CHECK(c.lines.size() == n);   // no retry storm
    // keys that did get through must come back
    c.ok = true;
    b.want(true);
    b.frame();                    // want() with the same value does not reset
    CHECK(b.failed());
    b.want(false);
    CHECK(!b.failed());
    b.frame();
    CHECK(b.settled());
}

TEST(chat_keyblock, enabling_does_not_give_up_while_the_console_is_down) {
    ConsoleRig c;
    KeyBlocker b(c.exec(), 70);
    b.want(true);
    for (int i = 0; i < 4; ++i) b.frame();
    CHECK(b.settled());
    b.want(false);
    c.ok = false;
    for (int i = 0; i < 3; ++i) b.frame();
    CHECK(!b.settled());
    CHECK(!b.failed());           // it waits and walks again
    c.ok = true;
    for (int i = 0; i < 400; ++i) b.frame();
    CHECK(b.settled() && b.disabledCount() == 0);
}

// ---- surfaces ---------------------------------------------------------------------------------------------------------------------------------------

namespace {
struct HudRig {
    std::vector<std::tuple<std::string, float, bool>> sent;
    bool ok = true;
    HudSurface s{[this](const std::string& t, float sec, bool next) { sent.emplace_back(t, sec, next); return ok; }, 8};
};
View composing(const std::string& text) { View v; v.composing = true; v.compose = text; return v; }
}  // namespace

TEST(chat_surface, the_hud_shows_each_new_line_once_for_the_configured_time) {
    HudRig h;
    h.s.onLine(mkLine("Ann", "hi", 1000, 1), 1000);
    h.s.onLine(sysLine("Bob joined", 1100), 1100);
    REQUIRE(h.sent.size() == 2);
    CHECK(std::get<0>(h.sent[0]) == "Ann: hi" && std::get<1>(h.sent[0]) == 8.0f && !std::get<2>(h.sent[0]));
    CHECK(std::get<0>(h.sent[1]) == "* Bob joined");
    h.s.present(View{}, 1200);
    h.s.present(View{}, 1300);
    CHECK(h.sent.size() == 2);   // present on its own repeats nothing
}

TEST(chat_surface, the_hud_echoes_the_typed_line_but_not_every_frame) {
    HudRig h;
    h.s.present(composing("> |"), 1000);
    REQUIRE(h.sent.size() == 1);   // the hint
    CHECK(std::get<0>(h.sent[0]).find("Enter") != std::string::npos);
    h.s.present(composing("> a|"), 1100);
    h.s.present(composing("> ab|"), 1200);
    CHECK(h.sent.size() == 1);     // too soon after the last message
    h.s.present(composing("> abc|"), 1000 + kEchoGapMs + 10);
    REQUIRE(h.sent.size() == 2);
    CHECK(std::get<0>(h.sent[1]) == "> abc|" && std::get<2>(h.sent[1]));
    h.s.present(composing("> abc|"), 5000);
    CHECK(h.sent.size() == 2);     // unchanged: nothing new
    h.s.present(View{}, 6000);     // closed
    h.s.present(composing("> |"), 7000);
    CHECK(h.sent.size() == 3);     // a new opening gives the hint again
}

TEST(chat_surface, the_hud_counts_what_the_engine_refuses) {
    HudRig h;
    h.ok = false;
    h.s.onLine(mkLine("Ann", "hi", 1, 1), 1);
    CHECK(h.s.refused() == 1 && h.s.messages() == 0);
}

TEST(chat_surface, the_overlay_panel_sits_bottom_left_and_fits) {
    View v;
    v.lines.push_back(ViewLine{"Ann: hello", 255, false});
    v.lines.push_back(ViewLine{"Bob: a longer line of text here", 255, false});
    v.composing = true;
    v.compose = "> hi|";
    OverlayLayout o = layoutOverlay(v, 1280, 720, 20, 8);
    CHECK(o.visible());
    CHECK(o.x == 16 && o.w == 31 * 8 + 12);
    CHECK(o.h == 3 * 20 + 12);
    CHECK(o.y + o.h == 720 - 16);
    REQUIRE(o.rowY.size() == 3);
    CHECK(o.rowY[0] == 6 && o.rowY[2] == 46);
    OverlayLayout narrow = layoutOverlay(v, 200, 720, 20, 8);
    CHECK(narrow.w == 200 - 32);
    CHECK(!layoutOverlay(View{}, 1280, 720, 20, 8).visible());
}

// ---- module API -------------------------------------------------------------------------------------------------------------------------------------

TEST(chat_api, posts_reach_the_poster_with_the_right_scope) {
    using namespace nvcoop::chat;
    CHECK(!available());
    postSystem("nobody listens");   // no crash
    std::vector<std::pair<std::string, bool>> got;
    setPoster([&](const std::string& t, bool everyone) { got.emplace_back(t, everyone); });
    CHECK(available());
    postSystem("Party saved");
    postLocal("just me");
    setPoster(nullptr);
    postSystem("after");
    REQUIRE(got.size() == 2);
    CHECK(got[0].first == "Party saved" && got[0].second);
    CHECK(got[1].first == "just me" && !got[1].second);
    CHECK(!available());
}
