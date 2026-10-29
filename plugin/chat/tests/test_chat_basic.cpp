// Text rules, rate limit, history, line editor, wire format, ini settings of plugin/chat. Twins 1001..1039, 1060..1064, 1110..1113 (FNVCOOP_MUT=<n>).
#include <map>
#include <random>

#include "chat/chat_config.h"
#include "chat/chat_limiter.h"
#include "chat/chat_log.h"
#include "chat/chat_text.h"
#include "chat/chat_wire.h"
#include "chat/line_editor.h"
#include "minitest.h"

using namespace nvcoop::chat;

namespace {
Line mk(Slot s, uint32_t id, const std::string& text, uint64_t at = 0, LineKind k = LineKind::Player) {
    Line l;
    l.slot = s; l.msgId = id; l.kind = k; l.name = k == LineKind::Player ? "N" : ""; l.text = text; l.atMs = at;
    return l;
}
}  // namespace

// ---- text -------------------------------------------------------------------------------------------------------------------------------------------

TEST(chat_text, controls_are_dropped_and_whitespace_collapses) {
    CHECK(sanitizeText("hello\x01\x02 world\x7F", 160) == "hello world");
    CHECK(sanitizeText("a\tb\nc\rd", 160) == "a b c d");
    CHECK(sanitizeText("   many    spaces   here   ", 160) == "many spaces here");
    CHECK(sanitizeText("\x01\x02\x03", 160).empty());
    CHECK(sanitizeText("", 160).empty());
    CHECK(sanitizeText(std::string("nul\0inside", 10), 160) == "nulinside");
}

TEST(chat_text, markup_and_format_characters_are_replaced) {
    CHECK(sanitizeText("100% sure %s %n %d", 160) == "100* sure *s *n *d");
    CHECK(sanitizeText("<b>x</b> & y", 160) == "(b)x(/b) + y");
    CHECK(sanitizeText("C:\\dir\\file", 160) == "C:/dir/file");
    const std::string s = sanitizeText("%%%&&&<<<>>>\\\\\\", 160);
    for (char c : s) CHECK(c != '%' && c != '&' && c != '<' && c != '>' && c != '\\');
}

TEST(chat_text, the_latin_one_range_stays_and_the_c1_range_goes) {
    CHECK(sanitizeText("caf\xE9 \xFC\xF1", 160) == "caf\xE9 \xFC\xF1");
    CHECK(sanitizeText("a\x80\x85\x9F" "b", 160) == "ab");
    CHECK(sanitizeText("x\xA0y", 160) == "x\xA0y");
}

TEST(chat_text, the_text_is_cut_at_the_limit) {
    const std::string long300(300, 'a');
    CHECK(sanitizeText(long300, kMaxTextBytes).size() == kMaxTextBytes);
    CHECK(sanitizeText("abcdef", 3) == "abc");
    CHECK(sanitizeText("abc   ", 4) == "abc");   // the space the cut leaves at the end is trimmed
    CHECK(sanitizeName(std::string(100, 'n')).size() == kMaxNameBytes);
    CHECK(sanitizeName("  Ann\n").size() == 3);
    CHECK(sanitizeName("\x01").empty());
}

TEST(chat_text, wrap_breaks_at_spaces_and_cuts_long_words) {
    auto w = wrapText("the quick brown fox jumps over the lazy dog", 16);
    for (auto& l : w) CHECK(l.size() <= 16);
    std::string joined;
    for (auto& l : w) joined += (joined.empty() ? "" : " ") + l;
    CHECK(joined == "the quick brown fox jumps over the lazy dog");
    CHECK(w.size() >= 3);
    auto one = wrapText("short", 16);
    CHECK(one.size() == 1 && one[0] == "short");
    auto nowrap = wrapText("anything at all goes on one line", 0);
    CHECK(nowrap.size() == 1);
    auto big = wrapText("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 10);
    CHECK(big.size() == 3);
    for (auto& l : big) CHECK(l.size() <= 10);
    auto ind = wrapText("Name: one two three four five six seven eight", 20, 2);
    CHECK(ind.size() >= 2);
    CHECK(ind[1].compare(0, 2, "  ") == 0);
    for (auto& l : ind) CHECK(l.size() <= 20);
}

TEST(chat_text, key_names) {
    CHECK(parseKeyName("Y") == 'Y');
    CHECK(parseKeyName("y") == 'Y');
    CHECK(parseKeyName("7") == '7');
    CHECK(parseKeyName("F9") == 0x78);
    CHECK(parseKeyName("f1") == 0x70);
    CHECK(parseKeyName("F24") == 0x87);
    CHECK(parseKeyName("F25") == 0);
    CHECK(parseKeyName("Tab") == 0x09);
    CHECK(parseKeyName("backslash") == 0xDC);
    CHECK(parseKeyName("0x59") == 0x59);
    CHECK(parseKeyName("89") == 89);
    CHECK(parseKeyName("300") == 0);
    CHECK(parseKeyName("") == 0);
    CHECK(parseKeyName("nonsense") == 0);
    CHECK(keyName('Y') == "Y");
    CHECK(keyName(0x78) == "F9");
    CHECK(keyName(0xDC) == "Backslash");
    CHECK(parseKeyName(keyName(0x2D)) == 0x2D);
}

// ---- rate limit -------------------------------------------------------------------------------------------------------------------------------------

TEST(chat_limiter, five_in_a_window_then_refused_then_free_again) {
    RateLimiter r(5, 10000);
    for (int i = 0; i < 5; ++i) CHECK(r.allow(1, 1000 + i * 100));
    CHECK(!r.allow(1, 1600));
    CHECK(!r.allow(1, 9000));
    CHECK(r.denied() == 2);
    CHECK(r.allow(1, 11001));   // the first line is more than a window old
}

TEST(chat_limiter, senders_are_independent) {
    RateLimiter r(2, 10000);
    CHECK(r.allow(1, 0)); CHECK(r.allow(1, 1)); CHECK(!r.allow(1, 2));
    CHECK(r.allow(2, 3)); CHECK(r.allow(2, 4)); CHECK(!r.allow(2, 5));
    r.forget(1);
    CHECK(r.allow(1, 6));
}

TEST(chat_limiter, a_refused_line_does_not_extend_the_lockout) {
    RateLimiter r(2, 1000);
    CHECK(r.allow(1, 0)); CHECK(r.allow(1, 100));
    for (int i = 0; i < 20; ++i) CHECK(!r.allow(1, 200 + i * 40));   // hammering until t = 960
    CHECK(r.allow(1, 1001));   // t = 0 is out of the window; the refused ones did not count
}

TEST(chat_limiter, a_clock_that_goes_backwards_starts_over) {
    RateLimiter r(1, 10000);
    CHECK(r.allow(1, 50000));
    CHECK(!r.allow(1, 50001));
    CHECK(r.allow(1, 100));   // a new session's clock
}

// ---- history ----------------------------------------------------------------------------------------------------------------------------------------

TEST(chat_log, keeps_the_newest_lines) {
    ChatLog log(5);
    for (uint32_t i = 1; i <= 8; ++i) CHECK(log.add(mk(1, i, "m" + std::to_string(i), i)));
    CHECK(log.lines().size() == 5);
    CHECK(log.lines().front().text == "m4");
    CHECK(log.lines().back().text == "m8");
    CHECK(log.lastSeq() == 8);
    log.setCapacity(2);
    CHECK(log.lines().size() == 2 && log.lines().front().text == "m7");
}

TEST(chat_log, visible_is_the_newest_unexpired) {
    ChatLog log(50);
    log.add(mk(1, 1, "old", 1000));
    log.add(mk(1, 2, "a", 9000));
    log.add(mk(1, 3, "b", 9500));
    log.add(mk(1, 4, "c", 9900));
    auto v = log.visible(10000, 5000, 10);   // "old" is 9000 ms old: gone
    REQUIRE(v.size() == 3);
    CHECK(v[0]->text == "a" && v[2]->text == "c");
    v = log.visible(10000, 5000, 2);         // only two fit: the newest two
    REQUIRE(v.size() == 2);
    CHECK(v[0]->text == "b" && v[1]->text == "c");
    CHECK(log.visible(30000, 5000, 5).empty());
    auto r = log.recent(2);
    REQUIRE(r.size() == 2);
    CHECK(r[0]->text == "b" && r[1]->text == "c");
    CHECK(log.recent(100).size() == 4);
}

TEST(chat_log, a_line_with_the_same_sender_and_id_is_refused) {
    ChatLog log;
    CHECK(log.add(mk(2, 10, "hi", 1)));
    CHECK(!log.add(mk(2, 10, "hi", 2)));
    CHECK(log.add(mk(3, 10, "other sender", 3)));
    CHECK(log.add(mk(2, 11, "next id", 4)));
    CHECK(log.add(mk(kSystemSlot, 0, "no id", 5, LineKind::System)));
    CHECK(log.add(mk(kSystemSlot, 0, "no id", 6, LineKind::System)));   // lines without an id are this machine's own notices: always taken
    CHECK(log.lines().size() == 5);
    CHECK(log.seen(2, 10));
    CHECK(!log.seen(2, 99));
}

TEST(chat_log, forgetting_a_sender_lets_a_new_player_reuse_the_ids) {
    ChatLog log;
    CHECK(log.add(mk(2, 1, "first", 1)));
    CHECK(!log.add(mk(2, 1, "first", 2)));
    log.forgetSender(2);
    CHECK(log.add(mk(2, 1, "a new player in slot 2", 3)));
    CHECK(!log.add(mk(2, 1, "again", 4)));
}

TEST(chat_log, the_memory_of_ids_is_bounded) {
    ChatLog log(10);
    for (uint32_t i = 1; i <= 2000; ++i) log.add(mk(1, i, "x", i));
    CHECK(log.add(mk(1, 1, "id 1 is long forgotten", 5000)));   // beyond the memory
    CHECK(!log.add(mk(1, 2000, "but the recent ones are kept", 5001)));
}

// ---- line editor ------------------------------------------------------------------------------------------------------------------------------------

namespace {
void typeStr(LineEditor& e, const std::string& s) { for (char c : s) e.typeChar(static_cast<unsigned char>(c)); }
}

TEST(chat_editor, typing_and_backspace) {
    LineEditor e;
    e.open();
    typeStr(e, "helo");
    CHECK(e.text() == "helo" && e.cursor() == 4);
    e.key(Key::Left);
    typeStr(e, "l");
    CHECK(e.text() == "hello" && e.cursor() == 4);
    e.key(Key::Backspace);
    CHECK(e.text() == "helo" && e.cursor() == 3);
    e.key(Key::Home);
    e.key(Key::Backspace);   // nothing before the cursor
    CHECK(e.text() == "helo" && e.cursor() == 0);
    e.key(Key::Delete);
    CHECK(e.text() == "elo" && e.cursor() == 0);
    e.key(Key::End);
    e.key(Key::Delete);      // nothing after
    CHECK(e.text() == "elo" && e.cursor() == 3);
    e.key(Key::Backspace); e.key(Key::Backspace); e.key(Key::Backspace); e.key(Key::Backspace);
    CHECK(e.text().empty() && e.cursor() == 0);
}

TEST(chat_editor, the_cursor_stops_at_the_ends) {
    LineEditor e;
    e.open();
    typeStr(e, "ab");
    e.key(Key::Right);
    CHECK(e.cursor() == 2);
    e.key(Key::Left); e.key(Key::Left); e.key(Key::Left); e.key(Key::Left);
    CHECK(e.cursor() == 0);
    typeStr(e, "X");
    CHECK(e.text() == "Xab");
}

TEST(chat_editor, characters_go_in_at_the_cursor_and_the_length_is_limited) {
    LineEditor e(10);
    e.open();
    typeStr(e, "0123456789");
    typeStr(e, "ZZZ");
    CHECK(e.text() == "0123456789");
    e.key(Key::Home);
    typeStr(e, "Q");
    CHECK(e.text() == "0123456789");   // full: the cursor position does not matter
    e.key(Key::Delete);
    typeStr(e, "Q");
    CHECK(e.text() == "Q123456789");
}

TEST(chat_editor, control_characters_and_a_closed_editor_take_nothing) {
    LineEditor e;
    typeStr(e, "closed");
    CHECK(e.text().empty());
    e.open();
    e.typeChar(0x01); e.typeChar(0x08); e.typeChar(0x0D); e.typeChar(0x1B); e.typeChar(0x7F);
    CHECK(e.text().empty());
    e.typeChar('%');
    CHECK(e.text() == "*");   // the same replacement the received text gets: what is typed is what is sent
    e.typeChar(0xE9);
    CHECK(e.text() == "*\xE9");
}

TEST(chat_editor, enter_sends_text_and_cancels_an_empty_line) {
    LineEditor e;
    e.open();
    CHECK(e.key(Key::Enter) == EditResult::Cancel);
    typeStr(e, "   ");
    CHECK(e.key(Key::Enter) == EditResult::Cancel);   // only spaces: nothing to say
    typeStr(e, " hi  there ");
    CHECK(e.key(Key::Enter) == EditResult::Send);
    CHECK(e.take() == "hi there");
    CHECK(!e.isOpen());
    CHECK(e.text().empty());
}

TEST(chat_editor, escape_throws_the_text_away) {
    LineEditor e;
    e.open();
    typeStr(e, "not sent");
    CHECK(e.key(Key::Escape) == EditResult::Cancel);
    CHECK(e.text().empty());
    e.close();
    e.open();
    CHECK(e.text().empty());   // and it is not there when the line is opened again
}

TEST(chat_editor, up_and_down_recall_the_sent_lines) {
    LineEditor e;
    for (const char* s : {"one", "two", "three"}) {
        e.open();
        typeStr(e, s);
        e.take();
    }
    e.open();
    typeStr(e, "draft");
    e.key(Key::Up);
    CHECK(e.text() == "three" && e.cursor() == 5);
    e.key(Key::Up);
    CHECK(e.text() == "two");
    e.key(Key::Up); e.key(Key::Up);
    CHECK(e.text() == "one");
    e.key(Key::Down);
    CHECK(e.text() == "two");
    e.key(Key::Down); e.key(Key::Down);
    CHECK(e.text() == "draft");   // back to what was being typed
    e.key(Key::Down);
    CHECK(e.text() == "draft");
}

TEST(chat_editor, a_paste_goes_in_cleaned_and_cut) {
    LineEditor e(20);
    e.open();
    e.insertText("line one\nline two\t& more");
    CHECK(e.text() == "line one line two + ");   // cut at 20 bytes; the tab and the line break became spaces, '&' became '+'
    LineEditor f;
    f.open();
    typeStr(f, "ab");
    f.key(Key::Left);
    f.insertText("XY");
    CHECK(f.text() == "aXYb" && f.cursor() == 3);
}

TEST(chat_editor, take_returns_clean_text) {
    LineEditor e;
    e.open();
    typeStr(e, "  100%  <b>  ");
    CHECK(e.take() == "100* (b)");
}

TEST(chat_editor, display_shows_the_caret_and_scrolls) {
    LineEditor e;
    e.open();
    typeStr(e, "hello");
    CHECK(e.display(40) == "hello|");
    e.key(Key::Left); e.key(Key::Left);
    CHECK(e.display(40) == "hel|lo");
    e.key(Key::End);
    typeStr(e, " world, this is a long line");
    const std::string d = e.display(12);
    CHECK(d.size() == 12);
    CHECK(d.find('|') != std::string::npos);
    CHECK(d.back() == '|');   // the caret is at the end and in view
    e.key(Key::Home);
    const std::string h = e.display(12);
    CHECK(h.size() == 12 && h[0] == '|');
}

// ---- wire -------------------------------------------------------------------------------------------------------------------------------------------

TEST(chat_wire, every_message_round_trips) {
    Message m;
    Bytes b = encodeSay(0xDEADBEEF, "hello there");
    REQUIRE(decode(b.data(), b.size(), m));
    CHECK(m.kind == Kind::Say && m.msgId == 0xDEADBEEF && m.text == "hello there");
    b = encodeLine(3, 42, LineKind::Player, "Ann", "hi all");
    REQUIRE(decode(b.data(), b.size(), m));
    CHECK(m.kind == Kind::Line && m.slot == 3 && m.msgId == 42 && m.lineKind == LineKind::Player && m.name == "Ann" && m.text == "hi all");
    b = encodeLine(kSystemSlot, 9, LineKind::System, "", "Ann joined");
    REQUIRE(decode(b.data(), b.size(), m));
    CHECK(m.kind == Kind::Line && m.slot == kSystemSlot && m.lineKind == LineKind::System && m.name.empty() && m.text == "Ann joined");
    b = encodeHello("Bob");
    REQUIRE(decode(b.data(), b.size(), m));
    CHECK(m.kind == Kind::Hello && m.name == "Bob");
    b = encodeNotice(kNoticeRateLimited);
    REQUIRE(decode(b.data(), b.size(), m));
    CHECK(m.kind == Kind::Notice && m.code == kNoticeRateLimited);
    CHECK(encodeSay(1, "x")[0] == kChannelChat);
}

TEST(chat_wire, the_layout_is_the_documented_one) {
    const Bytes b = encodeSay(0x01020304, "ab");
    const Bytes want = {kChannelChat, kWireVersion, 1, 0x04, 0x03, 0x02, 0x01, 2, 'a', 'b'};
    CHECK(b == want);
    const Bytes l = encodeLine(2, 5, LineKind::Player, "N", "t");
    const Bytes wantL = {kChannelChat, kWireVersion, 2, 2, 5, 0, 0, 0, 0, 1, 'N', 1, 't'};
    CHECK(l == wantL);
}

TEST(chat_wire, an_over_long_text_is_cut_by_the_encoder) {
    Message m;
    Bytes b = encodeSay(1, std::string(300, 'x'));
    REQUIRE(decode(b.data(), b.size(), m));
    CHECK(m.text.size() == kMaxTextBytes);
    b = encodeLine(1, 1, LineKind::Player, std::string(80, 'n'), "t");
    REQUIRE(decode(b.data(), b.size(), m));
    CHECK(m.name.size() == kMaxNameBytes);
}

TEST(chat_wire, the_decoder_refuses_what_it_does_not_know) {
    Message m;
    Bytes good = encodeLine(1, 1, LineKind::Player, "N", "text");
    CHECK(decode(good.data(), good.size(), m));
    Bytes trailing = good; trailing.push_back(0);
    CHECK(!decode(trailing.data(), trailing.size(), m));
    for (size_t n = 0; n < good.size(); ++n) CHECK(!decode(good.data(), n, m));   // every truncation
    Bytes chan = good; chan[0] = 0xC9;
    CHECK(!decode(chan.data(), chan.size(), m));
    Bytes ver = good; ver[1] = 2;
    CHECK(!decode(ver.data(), ver.size(), m));
    Bytes kind = good; kind[2] = 9;
    CHECK(!decode(kind.data(), kind.size(), m));
    Bytes kind0 = good; kind0[2] = 0;
    CHECK(!decode(kind0.data(), kind0.size(), m));
    Bytes lk = good; lk[8] = 7;   // lineKind
    CHECK(!decode(lk.data(), lk.size(), m));
    Bytes longText = {kChannelChat, kWireVersion, 1, 1, 0, 0, 0, 200};
    longText.insert(longText.end(), 200, 'x');
    CHECK(!decode(longText.data(), longText.size(), m));   // over kMaxTextBytes
    Bytes longName = {kChannelChat, kWireVersion, 3, 100};
    longName.insert(longName.end(), 100, 'n');
    CHECK(!decode(longName.data(), longName.size(), m));
    CHECK(!decode(nullptr, 0, m));
}

TEST(chat_wire, random_bytes_never_break_the_decoder) {
    std::mt19937 rng(12345);
    int accepted = 0;
    for (int i = 0; i < 20000; ++i) {
        Bytes b(rng() % 40);
        for (auto& x : b) x = static_cast<uint8_t>(rng());
        if (b.size() > 3 && (rng() & 3)) { b[0] = kChannelChat; b[1] = kWireVersion; b[2] = static_cast<uint8_t>(1 + rng() % 4); }
        Message m;
        if (decode(b.data(), b.size(), m)) {
            ++accepted;
            CHECK(m.text.size() <= kMaxTextBytes && m.name.size() <= kMaxNameBytes);
        }
    }
    CHECK(accepted > 0);
}

// ---- ini --------------------------------------------------------------------------------------------------------------------------------------------

namespace {
IniGetter iniOf(std::map<std::string, std::string> kv) {
    return [kv](const char*, const char* key, const char* def) { auto it = kv.find(key); return it == kv.end() ? std::string(def) : it->second; };
}
}

TEST(chat_config, defaults) {
    auto s = readSettings(iniOf({}));
    CHECK(s.cfg.enabled && s.cfg.openKey == 'Y' && s.cfg.name.empty() && s.cfg.display == "auto");
    CHECK(s.cfg.lineSeconds == 8 && s.cfg.visibleLines == 6 && s.cfg.historyLines == 40 && s.cfg.rateLines == 5 && s.cfg.rateSeconds == 10);
    CHECK(s.warnings.empty());
}

TEST(chat_config, values_are_read) {
    auto s = readSettings(iniOf({{"enabled", "0"}, {"key", "F9"}, {"name", " Wanderer\n"}, {"display", "Overlay"}, {"seconds", "12"}, {"lines", "4"},
                                  {"history", "100"}, {"rate_lines", "3"}, {"rate_seconds", "30"}}));
    CHECK(!s.cfg.enabled && s.cfg.openKey == 0x78 && s.cfg.name == "Wanderer" && s.cfg.display == "overlay");
    CHECK(s.cfg.lineSeconds == 12 && s.cfg.visibleLines == 4 && s.cfg.historyLines == 100 && s.cfg.rateLines == 3 && s.cfg.rateSeconds == 30);
    CHECK(s.warnings.empty());
}

TEST(chat_config, bad_values_keep_or_clamp_and_are_named) {
    auto s = readSettings(iniOf({{"key", "NotAKey"}, {"display", "neon"}, {"seconds", "999"}, {"lines", "abc"}, {"history", "1"}, {"rate_lines", "-4"}}));
    CHECK(s.cfg.openKey == 'Y' && s.cfg.display == "auto");
    CHECK(s.cfg.lineSeconds == 60);
    CHECK(s.cfg.visibleLines == 6);
    CHECK(s.cfg.historyLines == 10);
    CHECK(s.cfg.rateLines == 5);
    for (const char* k : {"key=", "display=", "seconds=", "lines=", "history=", "rate_lines="}) CHECK_MSG(s.warnings.find(k) != std::string::npos, k);
}

TEST(chat_config, the_name_is_cleaned) {
    auto s = readSettings(iniOf({{"name", "A%B<C>\t" + std::string(60, 'z')}}));
    CHECK(s.cfg.name.size() <= kMaxNameBytes);
    CHECK(s.cfg.name.find('%') == std::string::npos && s.cfg.name.find('<') == std::string::npos && s.cfg.name.find('\t') == std::string::npos);
    CHECK(describe(s.cfg).find("key=Y") != std::string::npos);
}
