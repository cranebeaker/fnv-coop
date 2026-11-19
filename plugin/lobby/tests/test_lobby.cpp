// Game-free tests of the in-game lobby logic (plugin/lobby/lobby.cpp). Prints "<n> checks, <m> failed".
// Mutation twins (FNVCOOP_MUT / --mut): 1 duplicate servers kept, 2 no room kept for Direct / Play alone, 3 port > 65535 accepted,
// 4 no token written for a first join. Each must make this test fail.
#include <cstdio>
#include <string>

#include "fnvcoop_mut.h"
#include "lobby.h"

using namespace nvlobby;

static int g_checks = 0, g_failed = 0;
#define CHECK(cond, what)                                                   \
    do {                                                                    \
        ++g_checks;                                                         \
        if (!(cond)) { ++g_failed; std::printf("FAIL %s (line %d)\n", what, __LINE__); } \
    } while (0)

static std::string get(const std::vector<std::pair<std::string, std::string>>& kv, const std::string& k) {
    for (auto& p : kv)
        if (p.first == k) return p.second;
    return "<none>";
}

int main(int argc, char** argv) {
    fnvcoop_mut::initFromArgs(argc, argv);

    // ---- addresses
    std::string h, why;
    uint16_t p = 0;
    CHECK(ParseAddress("203.0.113.5", &h, &p, &why) && h == "203.0.113.5" && p == 27960, "plain address gets the default port");
    CHECK(ParseAddress(" 203.0.113.5:30000 ", &h, &p, &why) && h == "203.0.113.5" && p == 30000, "address with port");
    CHECK(ParseAddress("\"friend.example.net:27961\"", &h, &p, &why) && h == "friend.example.net" && p == 27961, "quoted host name");
    CHECK(!ParseAddress("", &h, &p, &why) && !why.empty(), "empty refused with a reason");
    CHECK(!ParseAddress("1.2.3.4:0", &h, &p, &why), "port 0 refused");
    CHECK(!ParseAddress("1.2.3.4:70000", &h, &p, &why), "port above 65535 refused");
    CHECK(!ParseAddress("1.2.3.4:27x", &h, &p, &why), "port with letters refused");
    CHECK(!ParseAddress("http://1.2.3.4", &h, &p, &why), "url refused");
    CHECK(!ParseAddress("1.2.3.4 5", &h, &p, &why), "space refused");
    CHECK(!ParseAddress("::1", &h, &p, &why), "IPv6 refused");
    CHECK(!ParseAddress(":27960", &h, &p, &why), "no host refused");
    CHECK(!ParseAddress("a;b", &h, &p, &why), "odd characters refused");

    // ---- [servers]
    const std::string ini =
        "[coop]\nenable=1\nrole=host\nAlice=1.1.1.1\n"
        "[servers]\n; friends\nAlice = 203.0.113.5\nBob=203.0.113.9:30000\nAlice again=203.0.113.5:27960\n=1.2.3.4\nEmpty=\nnot a line\n"
        "[combat]\nCarol=9.9.9.9\n";
    auto sv = ParseServers(ini);
    CHECK(sv.size() == 2, "two servers: [coop]/[combat] lines, blanks and a duplicate address skipped");
    CHECK(sv.size() == 2 && sv[0].name == "Alice" && sv[0].address == "203.0.113.5" && sv[1].name == "Bob", "order and trimming kept");

    // ---- the menu
    Menu m = BuildMenu(sv, "", 0, "host", true);
    CHECK(m.buttons.size() == m.actions.size(), "one action per button");
    CHECK(m.buttons.size() == 5 && m.buttons[0] == "Host a game" && m.buttons[1] == "Join Alice" && m.buttons[2] == "Join Bob" &&
              m.buttons[3] == "Direct connect" && m.buttons[4] == "Play alone",
          "Host, servers, Direct connect, Play alone");
    CHECK(m.actions[2].kind == Kind::Join && m.actions[2].address == "203.0.113.9:30000", "join action carries the address");
    CHECK(m.text.find("hosts a game") != std::string::npos, "text says loading a save hosts");
    Menu mc = BuildMenu(sv, "198.51.100.7", 27960, "client", true);
    CHECK(mc.buttons.size() == 6 && mc.buttons[1] == "Join 198.51.100.7", "last host first when not in the list");
    CHECK(mc.text.find("joins 198.51.100.7") != std::string::npos, "text says loading a save joins the last host");
    Menu ml = BuildMenu(sv, "203.0.113.5", 27960, "client", true);
    CHECK(ml.buttons.size() == 5, "last host already listed: not added twice");
    Menu mo = BuildMenu(sv, "", 0, "host", false);
    CHECK(mo.text.find("plays alone") != std::string::npos, "co-op off: text says so");
    std::vector<Server> many;
    for (int i = 0; i < 20; ++i) many.push_back(Server{"S" + std::to_string(i), "10.0.0." + std::to_string(i + 1)});
    Menu mm = BuildMenu(many, "", 0, "host", true);
    CHECK(mm.buttons.size() == kMaxButtons, "never more than 10 buttons");
    CHECK(!mm.buttons.empty() && mm.buttons.back() == "Play alone" && mm.buttons[mm.buttons.size() - 2] == "Direct connect",
          "Direct connect and Play alone always fit");

    // ---- ini keys
    auto kh = CoopKeysFor(Action{Kind::Host, ""}, 0, 99);
    CHECK(get(kh, "enable") == "1" && get(kh, "role") == "host" && get(kh, "listen") == "0.0.0.0" && get(kh, "token") == "<none>", "host keys");
    auto kj = CoopKeysFor(Action{Kind::Join, "203.0.113.9:30000"}, 0, 4242);
    CHECK(get(kj, "role") == "client" && get(kj, "host") == "203.0.113.9" && get(kj, "port") == "30000", "join keys");
    CHECK(get(kj, "token") == "4242", "first join writes a token (same slot on rejoin)");
    auto kj2 = CoopKeysFor(Action{Kind::Join, "203.0.113.9"}, 77, 4242);
    CHECK(get(kj2, "token") == "<none>" && get(kj2, "port") == "27960", "existing token kept, default port");
    CHECK(CoopKeysFor(Action{Kind::Join, "bad address!"}, 0, 1).empty(), "bad join address writes nothing");
    CHECK(get(CoopKeysFor(Action{Kind::Solo, ""}, 0, 1), "enable") == "0", "play alone turns co-op off");
    CHECK(CoopKeysFor(Action{Kind::Direct, ""}, 0, 1).empty(), "direct connect writes nothing");

    // ---- console commands
    Action a;
    CHECK(ParseCommand("CoopJoin", "203.0.113.5", &a, &why) && a.kind == Kind::Join && a.address == "203.0.113.5", "coopjoin");
    CHECK(ParseCommand("coopjoin", "203.0.113.5:30000", &a, &why) && a.address == "203.0.113.5:30000", "coopjoin with port");
    CHECK(!ParseCommand("coopjoin", "", &a, &why) && !why.empty(), "coopjoin without address explains");
    CHECK(ParseCommand("COOPHOST", "", &a, &why) && a.kind == Kind::Host, "coophost");
    CHECK(ParseCommand("coopsolo", "", &a, &why) && a.kind == Kind::Solo, "coopsolo");
    CHECK(!ParseCommand("coopfoo", "", &a, &why), "unknown refused");

    std::printf("%d checks, %d failed\n", g_checks, g_failed);
    return g_failed ? 1 : 0;
}
