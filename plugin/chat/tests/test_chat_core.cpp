// The rules of the chat (ChatCore) with a host and guests wired through a queue, and the session adapter on real mux transports.
// Twins 1040..1056 (core) and 1130..1136 (session), chosen with FNVCOOP_MUT=<n>.
#include "chat_rig.h"

using namespace nvchat;

// ---- core -------------------------------------------------------------------------------------------------------------------------------------------

TEST(chat_core, a_guest_line_reaches_the_host_and_the_other_guests_but_not_itself_twice) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    CHECK(h.g(1).say("hello everyone", h.now) == SayResult::Sent);
    h.pump();
    CHECK(count(h.host.log(), "Ann: hello everyone") == 1);
    CHECK(count(h.g(2).log(), "Ann: hello everyone") == 1);
    CHECK(count(h.g(1).log(), "Ann: hello everyone") == 1);   // its own line, shown once, at once
    CHECK(h.host.stats().relayed == 1);
    CHECK(h.g(1).stats().ownEchoes == 0);   // the host never sent it back
}

TEST(chat_core, a_guest_line_is_shown_at_once_before_anything_comes_back) {
    Hub h;
    h.join(1, "Ann");
    h.g(1).say("instant", h.now);
    CHECK(has(h.g(1).log(), "Ann: instant"));   // nothing was pumped
}

TEST(chat_core, a_host_line_reaches_every_guest) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    h.host.say("welcome", h.now);
    h.pump();
    CHECK(has(h.g(1).log(), "Player 1: welcome"));
    CHECK(has(h.g(2).log(), "Player 1: welcome"));
    CHECK(count(h.host.log(), "Player 1: welcome") == 1);
}

TEST(chat_core, a_configured_name_is_used_by_the_host_too) {
    Config c; c.name = "Overseer";
    Hub h(c);
    h.join(1, "Ann");
    h.host.say("hi", h.now);
    h.pump();
    CHECK(has(h.g(1).log(), "Overseer: hi"));
}

TEST(chat_core, a_joined_line_follows_the_hello_and_goes_to_everyone) {
    Hub h;
    h.join(1, "Ann");
    h.pump();
    CHECK(has(h.host.log(), "* Ann joined"));
    CHECK(has(h.g(1).log(), "* Ann joined"));
    h.join(2, "Bob");
    CHECK(has(h.host.log(), "* Bob joined"));
    CHECK(has(h.g(1).log(), "* Bob joined"));
    CHECK(has(h.g(2).log(), "* Bob joined"));
    CHECK(h.host.stats().joinsAnnounced == 2);
}

TEST(chat_core, a_second_hello_does_not_announce_again) {
    Hub h;
    h.join(1, "Ann");
    h.g(1).linkUp();   // a reconnect of the link
    h.pump();
    CHECK(count(h.host.log(), "* Ann joined") == 1);
}

TEST(chat_core, a_left_line_goes_to_the_rest) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    h.leave(1);
    CHECK(has(h.host.log(), "* Ann left"));
    CHECK(has(h.g(2).log(), "* Ann left"));
    CHECK(h.host.stats().leavesAnnounced == 1);
}

TEST(chat_core, a_guest_without_a_hello_is_not_announced_and_gets_a_default_name) {
    Hub h;
    h.joined.push_back(3);
    h.host.playerJoined(3);
    Config c;
    ChatCore g(c, 5);
    Ports p;
    p.sendToHost = [&](Bytes d) { h.q.push_back(Hub::Msg{3, 0, std::move(d)}); };
    g.setPorts(p);
    g.begin(false, 3);
    g.say("no hello sent", h.now);   // an older build or a lost Hello
    h.pump();
    CHECK(has(h.host.log(), "Player 4: no hello sent"));
    h.host.playerLeft(3, h.now);
    h.joined.clear();
    CHECK(!has(h.host.log(), "* Player 4 left"));   // never announced as joined, so no left line
}

TEST(chat_core, two_players_with_one_name_get_different_names) {
    Hub h;
    h.join(1, "Sam");
    h.join(2, "sam");
    h.join(3, "SAM");
    h.g(1).say("a", h.now);
    h.g(2).say("b", h.now);
    h.g(3).say("c", h.now);
    h.pump();
    CHECK(has(h.host.log(), "Sam: a"));
    CHECK(has(h.host.log(), "sam 2: b"));
    CHECK(has(h.host.log(), "SAM 3: c"));
}

TEST(chat_core, a_name_taken_by_the_host_is_not_given_to_a_guest) {
    Config c; c.name = "Alex";
    Hub h(c);
    h.join(1, "Alex");
    h.g(1).say("x", h.now);
    h.pump();
    CHECK(has(h.host.log(), "Alex 2: x"));
}

TEST(chat_core, the_text_a_guest_sends_is_cleaned_by_the_host) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    Bytes raw = {kChannelChat, kWireVersion, 1, 77, 0, 0, 0, 0};
    const std::string evil = std::string("%s%n \x01<b>&amp;\x1B[31m  spaced   out ") + std::string(100, 'z');
    raw.back() = static_cast<uint8_t>(evil.size() > 160 ? 160 : evil.size());
    raw.insert(raw.end(), evil.begin(), evil.begin() + raw.back());
    h.inject(1, raw);
    const Line& l = h.g(2).log().lines().back();
    CHECK(l.kind == LineKind::Player && l.name == "Ann");
    for (char c : h.host.log().lines().back().text) CHECK(static_cast<unsigned char>(c) >= 0x20 && c != '%' && c != '<' && c != '&' && c != '>');   // the host's own copy too
    for (const char* bad : {"%", "<", "&", ""}) CHECK(h.host.log().lines().back().text.find(bad) == std::string::npos);
    for (char c : l.text) CHECK(static_cast<unsigned char>(c) >= 0x20 && c != '%' && c != '<' && c != '&' && c != '>');
    CHECK(l.text.find("  ") == std::string::npos);
    CHECK(l.text.compare(0, 4, "*s*n") == 0);
}

TEST(chat_core, the_text_the_host_sends_is_cleaned_by_a_guest) {
    Hub h;
    h.join(1, "Ann");
    Bytes line = encodeLine(2, 5, LineKind::Player, "Mal\x01lory%s", "pay 100% now \x02<script>");
    h.injectToGuest(1, line);
    const Line& l = h.g(1).log().lines().back();
    CHECK(l.name == "Mallory*s");
    CHECK(l.text == "pay 100* now (script)");
    h.injectToGuest(1, encodeLine(kSystemSlot, 6, LineKind::System, "ignored", "%d system"));
    CHECK(h.g(1).log().lines().back().text == "*d system");
    CHECK(h.g(1).log().lines().back().name.empty());
}

TEST(chat_core, a_line_that_is_empty_after_cleaning_is_dropped) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    const size_t before = h.g(2).log().lines().size();
    Bytes raw = {kChannelChat, kWireVersion, 1, 5, 0, 0, 0, 3, 1, 2, 3};
    h.inject(1, raw);
    CHECK(h.g(2).log().lines().size() == before);
    CHECK(h.g(1).say(" \t ", h.now) == SayResult::Empty);
}

TEST(chat_core, the_host_limits_a_guest_that_ignores_its_own_limit) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    for (uint32_t i = 1; i <= 8; ++i) h.inject(1, encodeSay(100 + i, "spam " + std::to_string(i)));
    int shown = 0;
    for (const Line& l : h.g(2).log().lines()) shown += l.text.compare(0, 4, "spam") == 0 ? 1 : 0;
    CHECK(shown == 5);
    CHECK(h.host.stats().rateLimited == 3);
    CHECK(has(h.g(1).log(), "* You are sending messages too fast. The last one was not sent."));   // the sender is told
}

TEST(chat_core, the_limit_is_per_player) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    for (uint32_t i = 1; i <= 5; ++i) h.inject(1, encodeSay(i, "a" + std::to_string(i)));
    for (uint32_t i = 1; i <= 5; ++i) h.inject(2, encodeSay(i, "b" + std::to_string(i)));
    CHECK(h.host.stats().rateLimited == 0);
    CHECK(has(h.host.log(), "Bob: b5"));
}

TEST(chat_core, a_guest_stops_itself_before_it_is_told) {
    Hub h;
    h.join(1, "Ann");
    for (int i = 0; i < 5; ++i) CHECK(h.g(1).say("m" + std::to_string(i), h.now) == SayResult::Sent);
    const size_t sentBefore = h.q.size();
    CHECK(h.g(1).say("one too many", h.now) == SayResult::RateLimited);
    CHECK(h.q.size() == sentBefore);   // nothing went out
    CHECK(!has(h.g(1).log(), "Ann: one too many"));
    h.now += 11000;
    CHECK(h.g(1).say("free again", h.now) == SayResult::Sent);
}

TEST(chat_core, the_host_limits_its_own_player_too) {
    Hub h;
    h.join(1, "Ann");
    for (int i = 0; i < 5; ++i) CHECK(h.host.say("h" + std::to_string(i), h.now) == SayResult::Sent);
    CHECK(h.host.say("h5", h.now) == SayResult::RateLimited);
}

TEST(chat_core, a_repeated_say_is_relayed_once) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    const Bytes d = encodeSay(555, "twice sent");
    h.inject(1, d);
    h.inject(1, d);
    CHECK(count(h.g(2).log(), "Ann: twice sent") == 1);
    CHECK(count(h.host.log(), "Ann: twice sent") == 1);
    CHECK(h.host.stats().duplicates == 1);
}

TEST(chat_core, a_repeated_say_does_not_use_up_the_rate_limit) {
    Hub h;
    h.join(1, "Ann");
    const Bytes d = encodeSay(555, "same");
    for (int i = 0; i < 10; ++i) h.inject(1, d);
    h.inject(1, encodeSay(556, "next"));
    CHECK(h.host.stats().rateLimited == 0);
    CHECK(has(h.host.log(), "Ann: next"));
}

TEST(chat_core, a_repeated_line_is_shown_once_by_a_guest) {
    Hub h;
    h.join(1, "Ann");
    const Bytes d = encodeLine(2, 31, LineKind::Player, "Bob", "dup");
    h.injectToGuest(1, d);
    h.injectToGuest(1, d);
    CHECK(count(h.g(1).log(), "Bob: dup") == 1);
    const Bytes s = encodeLine(kSystemSlot, 32, LineKind::System, "", "sys dup");
    h.injectToGuest(1, s);
    h.injectToGuest(1, s);
    CHECK(count(h.g(1).log(), "* sys dup") == 1);
}

TEST(chat_core, a_guest_never_shows_a_line_that_names_it_as_the_origin) {
    Hub h;
    h.join(1, "Ann");
    h.g(1).say("mine", h.now);
    h.injectToGuest(1, encodeLine(1, 9999, LineKind::Player, "Ann", "mine again"));
    CHECK(!has(h.g(1).log(), "Ann: mine again"));
    CHECK(h.g(1).stats().ownEchoes == 1);
}

TEST(chat_core, datagrams_from_a_slot_that_is_not_joined_are_dropped) {
    Hub h;
    h.join(1, "Ann");
    h.inject(5, encodeSay(1, "ghost"));
    h.inject(0, encodeSay(2, "from the host slot"));
    h.inject(1, encodeHello("x"));   // fine
    CHECK(!has(h.host.log(), "Player 6: ghost"));
    CHECK(h.host.stats().unknownSender == 2);
}

TEST(chat_core, roles_only_take_the_messages_that_are_theirs) {
    Hub h;
    h.join(1, "Ann");
    h.injectToGuest(1, encodeSay(1, "a guest takes no Say"));
    h.injectToGuest(1, encodeHello("x"));
    CHECK(!has(h.g(1).log(), "Player 1: a guest takes no Say"));
    const size_t n = h.host.log().lines().size();
    h.inject(1, encodeLine(1, 4, LineKind::Player, "Ann", "a host takes no Line"));
    CHECK(h.host.log().lines().size() == n);
}

TEST(chat_core, garbage_is_counted_not_shown) {
    Hub h;
    h.join(1, "Ann");
    const size_t n = h.host.log().lines().size();
    h.inject(1, Bytes{kChannelChat, kWireVersion, 77, 1, 2, 3});
    h.inject(1, Bytes{});
    CHECK(h.host.stats().undecodable == 2);
    CHECK(h.host.log().lines().size() == n);
}

TEST(chat_core, system_lines_from_the_host_reach_everyone_and_a_guests_stay_home) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    h.host.system("Party saved", true, h.now);
    h.pump();
    CHECK(has(h.g(1).log(), "* Party saved"));
    CHECK(has(h.g(2).log(), "* Party saved"));
    h.g(1).system("local only", true, h.now);
    h.pump();
    CHECK(has(h.g(1).log(), "* local only"));
    CHECK(!has(h.host.log(), "* local only") && !has(h.g(2).log(), "* local only"));
    h.host.system("host local", false, h.now);
    h.pump();
    CHECK(has(h.host.log(), "* host local") && !has(h.g(1).log(), "* host local"));
    h.host.system("%s \x01", true, h.now);
    h.pump();
    CHECK(has(h.g(2).log(), "* *s"));
}

TEST(chat_core, without_a_session_nothing_is_sent) {
    ChatCore c(Config{}, 1);
    CHECK(c.say("hello", 1000) == SayResult::NoSession);
    CHECK(c.log().lines().empty());
    c.notice("a notice", 1000);
    CHECK(c.log().lines().size() == 1);
    uint8_t junk[] = {kChannelChat, kWireVersion, 1, 0, 0, 0, 0, 0};
    c.onDatagram(0, junk, sizeof junk, 1000);
    CHECK(c.stats().received == 0);
}

TEST(chat_core, a_rejoining_player_can_use_the_same_ids_again) {
    Hub h;
    h.join(1, "Ann", 500);
    h.g(1).say("first life", h.now);
    h.pump();
    h.leave(1);
    h.join(1, "Ann", 500);   // a new core with the same id sequence in the same slot
    h.g(1).say("second life", h.now);
    h.pump();
    CHECK(has(h.host.log(), "Ann: second life"));
}

TEST(chat_core, the_history_keeps_only_the_newest_lines) {
    Config c; c.historyLines = 10;
    Hub h(c);
    h.join(1, "Ann");
    for (int i = 0; i < 4; ++i) {
        h.now += 20000;
        for (int k = 0; k < 4; ++k) h.g(1).say("m" + std::to_string(i * 4 + k), h.now);
        h.pump();
    }
    CHECK(h.host.log().lines().size() == 10);
    CHECK(has(h.host.log(), "Ann: m15"));
    CHECK(!has(h.host.log(), "Ann: m0"));
}

TEST(chat_core, a_long_line_is_cut_to_the_limit) {
    Hub h;
    h.join(1, "Ann");
    h.g(1).say(std::string(500, 'q'), h.now);
    h.pump();
    CHECK(h.host.log().lines().back().text.size() == kMaxTextBytes);
    CHECK(h.host.log().lines().back().text == std::string(kMaxTextBytes, 'q'));
}

TEST(chat_core, lines_of_one_player_arrive_in_the_order_they_were_said) {
    Hub h;
    h.join(1, "Ann");
    h.join(2, "Bob");
    for (const char* t : {"one", "two", "three", "four"}) h.g(1).say(t, h.now);
    h.pump();
    std::vector<std::string> mine;
    for (const std::string& t : texts(h.g(2).log()))
        if (t.compare(0, 5, "Ann: ") == 0) mine.push_back(t);
    const std::vector<std::string> want = {"Ann: one", "Ann: two", "Ann: three", "Ann: four"};
    CHECK(mine == want);
}

// ---- session adapter on real mux transports ---------------------------------------------------------------------------------------------------------

TEST(chat_session, a_line_travels_guest_to_host_to_the_other_guest_over_the_mux) {
    MuxRig r;
    r.addGuest(1, "Ann");
    r.addGuest(2, "Bob");
    r.pump();
    CHECK(has(r.host.core().log(), "* Ann joined"));
    CHECK(has(r.guest[2]->core().log(), "* Ann joined"));
    CHECK(r.guest[1]->core().say("over the wire", r.now) == SayResult::Sent);
    r.pump();
    CHECK(has(r.host.core().log(), "Ann: over the wire"));
    CHECK(has(r.guest[2]->core().log(), "Ann: over the wire"));
    CHECK(count(r.guest[1]->core().log(), "Ann: over the wire") == 1);
    r.host.core().say("from the host", r.now);
    r.pump();
    CHECK(has(r.guest[1]->core().log(), "Player 1: from the host") && has(r.guest[2]->core().log(), "Player 1: from the host"));
}

TEST(chat_session, hello_is_sent_once_per_link_up) {
    MuxRig r;
    r.addGuest(1, "Ann");
    r.pump();
    CHECK(r.host.core().stats().joinsAnnounced == 1);
    r.pump();
    CHECK(r.host.core().stats().joinsAnnounced == 1);
    // the link drops and comes back: a new Hello, no second "joined" line
    r.guest[1]->frame(r.guestInfo(1, false));
    r.guest[1]->frame(r.guestInfo(1, true));
    r.pump();
    CHECK(count(r.host.core().log(), "* Ann joined") == 1);
    CHECK(r.host.core().stats().received >= 2);
}

TEST(chat_session, a_guest_that_leaves_is_announced) {
    MuxRig r;
    r.addGuest(1, "Ann");
    r.addGuest(2, "Bob");
    r.pump();
    r.slots.erase(r.slots.begin());   // Ann is no longer a client of the session
    r.guest.erase(1);
    r.pump();
    CHECK(has(r.host.core().log(), "* Ann left"));
    CHECK(has(r.guest[2]->core().log(), "* Ann left"));
}

TEST(chat_session, a_datagram_from_an_unknown_peer_is_dropped) {
    MuxRig r;
    r.addGuest(1, "Ann");
    r.pump();
    fnvcoop::MuxTransport stranger{*r.net.endpoint(9)};
    const size_t before = r.host.core().log().lines().size();
    stranger.sendEx(0, encodeSay(1, "I am nobody"), fnvcoop::ITransport::Channel::Reliable);
    r.pump();
    CHECK(r.host.core().log().lines().size() == before);   // not shown as anybody's line
    CHECK(r.host.droppedUnknownPeer() == 1);
}

TEST(chat_session, a_peer_that_joined_this_very_frame_is_found_by_asking_again) {
    MuxRig r;
    r.addGuest(1, "Ann");
    r.frames();
    r.net.advance(r.now);
    // guest 2 joins: its Hello arrives before the host's next frame lists it
    r.addGuest(2, "Bob");
    r.guest[2]->frame(r.guestInfo(2));
    r.net.advance(r.now);
    FrameInfo hi = r.hostInfo();
    FrameInfo before = hi;
    before.clients.pop_back();          // what the host's last frame knew
    before.clientsNow = [&] { return hi.clients; };
    r.host.frame(before);
    fnvcoop::PeerId from; fnvcoop::Bytes d;
    while (r.hostMux.poll(from, d)) {}
    CHECK(has(r.host.core().log(), "* Bob joined"));
}

TEST(chat_session, a_new_session_keeps_the_history_and_works_on_the_new_mux) {
    MuxRig r;
    r.addGuest(1, "Ann");
    r.pump();
    r.guest[1]->core().say("session one", r.now);
    r.pump();
    CHECK(r.host.sessions() == 1);
    fnvcoop::MuxTransport second{*r.net.endpoint(0)};
    r.hostMuxOverride = &second;
    r.pump();
    CHECK(r.host.sessions() == 2);
    r.guest[1]->frame(r.guestInfo(1, false));   // the guest's side of the new session: its link comes up again and it says hello again
    r.pump();
    CHECK(has(r.host.core().log(), "Ann: session one"));   // the history survives
    r.guest[1]->core().say("session two", r.now);
    r.pump();
    CHECK(has(r.host.core().log(), "Ann: session two"));
}

TEST(chat_session, the_handler_of_an_ended_session_does_nothing) {
    MuxRig r;
    r.addGuest(1, "Ann");
    r.pump();
    fnvcoop::MuxTransport second{*r.net.endpoint(0)};
    FrameInfo f = r.hostInfo();
    f.mux = &second;
    r.host.frame(f);                       // session 2 on a new mux; the handler on hostMux belongs to session 1
    CHECK(r.host.sessions() == 2);
    const size_t n = r.host.core().log().lines().size();
    r.guestMux[1]->sendEx(0, encodeSay(77, "to a dead handler"), fnvcoop::ITransport::Channel::Reliable);
    r.net.advance(r.now);
    fnvcoop::PeerId from; fnvcoop::Bytes d;
    while (r.hostMux.poll(from, d)) {}     // the old mux takes the datagram
    CHECK(r.host.core().log().lines().size() == n);
}

TEST(chat_session, the_session_ending_stops_the_chat) {
    MuxRig r;
    r.addGuest(1, "Ann");
    r.pump();
    CHECK(r.host.up() && r.host.core().active());
    FrameInfo off;
    off.active = false;
    r.host.frame(off);
    CHECK(!r.host.up() && !r.host.core().active());
    CHECK(r.host.core().say("anyone?", r.now) == SayResult::NoSession);
}

TEST(chat_session, a_guest_waits_for_its_slot) {
    MuxRig r;
    r.addGuest(1, "Ann");
    FrameInfo f = r.guestInfo(1);
    f.self = kSystemSlot;   // not joined yet
    r.guest[1]->frame(f);
    CHECK(!r.guest[1]->up());
    r.guest[1]->frame(r.guestInfo(1));
    CHECK(r.guest[1]->up());
}

namespace {
// An ITransport that remembers the channel of everything sent through it.
class RecTransport : public fnvcoop::ITransport {
public:
    explicit RecTransport(fnvcoop::ITransport& in) : in_(in) {}
    void send(fnvcoop::PeerId to, fnvcoop::Bytes d) override { sendEx(to, std::move(d), Channel::Reliable); }
    void sendEx(fnvcoop::PeerId to, fnvcoop::Bytes d, Channel ch) override { channels.push_back(ch); in_.sendEx(to, std::move(d), ch); }
    bool poll(fnvcoop::PeerId& from, fnvcoop::Bytes& d) override { return in_.poll(from, d); }
    std::vector<Channel> channels;
private:
    fnvcoop::ITransport& in_;
};
}  // namespace

TEST(chat_session, everything_goes_on_the_reliable_channel) {
    fnvcoop::InMemoryNetwork net{3};
    RecTransport rec{*net.endpoint(1)};
    fnvcoop::MuxTransport mux{rec};
    ChatSession guest(Config{}, 1);
    FrameInfo f;
    f.active = true; f.isHost = false; f.self = 1; f.hostLinkUp = true; f.mux = &mux; f.nowMs = 1000;
    guest.frame(f);                                     // the Hello
    guest.core().say("a line", 1000);                   // and a Say
    REQUIRE(rec.channels.size() == 2);
    for (auto c : rec.channels) CHECK(c == fnvcoop::ITransport::Channel::Reliable);
}
