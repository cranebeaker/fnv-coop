// Party saves (plugin/coop/party_save.*, docs/party-save.md): names, sidecar and index texts, the orders, the host's sequence, the guest's pending-save
// state machine, the hand-save request (a guest saves by hand: the host makes the matching save and the requester adopts its own), and the real session's
// order channel carrying all of it. Game-free; the engine calls are fakes. Each test has twins (FNVCOOP_MUT=1100..1110): it must FAIL with its twin on.
#include <chrono>
#include <cstdio>
#include <functional>
#include <memory>
#include <thread>
#include <vector>

#include "fake_world.h"
#include "party_save.h"

using namespace nvcoop::coop;
using namespace nvcoop::coop::test;
namespace pt = nvcoop::coop::party;
using nvcoop::json::Value;

static int g_checks = 0, g_failed = 0;
#define CHECK(c)                                                                  \
    do {                                                                          \
        ++g_checks;                                                               \
        if (!(c)) { ++g_failed; std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); } \
    } while (0)

static uint64_t realNowMs() {
    return std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now().time_since_epoch()).count();
}
static bool pumpUntil(const std::function<void()>& frame, const std::function<bool()>& done, int timeoutMs = 5000) {
    uint64_t end = realNowMs() + timeoutMs;
    while (realNowMs() < end) {
        frame();
        if (done()) return true;
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    return done();
}

static const uint64_t kCamp = 0x00ab12cd34ef5678ull;

// ---- names, sidecar, index, order texts ---------------------------------------------------------------------------------------------------------
static bool launcherSafe(const std::string& n) {   // tools/launcher/fnv_party_saves.py valid_save_name
    if (n.empty() || n.size() > 150 || n.front() == ' ' || n.back() == ' ') return false;
    for (char c : n)
        if ((unsigned char)c < 32 || c == 127 || std::string("\\/:*?\"<>|").find(c) != std::string::npos) return false;
    return true;
}

static void testNames() {
    std::printf("party save: campaign id and save names\n");
    CHECK(pt::campaignHex(kCamp) == "00ab12cd34ef5678");
    uint64_t id = 0;
    CHECK(pt::parseCampaign("00AB12CD34EF5678", &id) && id == kCamp);
    CHECK(!pt::parseCampaign("0000000000000000", &id) && !pt::parseCampaign("abc", &id) && !pt::parseCampaign("00ab12cd34ef567x", &id) && !pt::parseCampaign("", &id));
    CHECK(pt::shortCampaign(kCamp) == "00ab12");
    CHECK(pt::saveBaseName("D:\\Documents\\My Games\\FalloutNV\\Saves\\Save 4 - Ann Primm.fos") == "Save 4 - Ann Primm");
    CHECK(pt::saveBaseName("Saves/Quicksave.FOS") == "Quicksave" && pt::saveBaseName("Save 4 - Ann Primm") == "Save 4 - Ann Primm" && pt::saveBaseName("").empty());
    // the guest's save name: readable in the load menu, tied to the campaign and the host save
    CHECK(pt::guestSaveName(kCamp, "Save 4 - Ann Primm.fos") == "Co-op 00ab12 Save 4 - Ann Primm");
    CHECK(pt::guestSaveName(kCamp, "Quicksave") == "Co-op 00ab12 Quicksave");
    CHECK(pt::guestSaveName(kCamp, "Co-op 00ab12 Host Save 5 - Bob") == "Co-op 00ab12 Save 5 - Bob");   // never wrapped a second time (twin 1106)
    CHECK(pt::guestSaveName(kCamp, "Co-op ffee00 Save 5") == "Co-op 00ab12 Save 5");
    CHECK(pt::hostSaveNameForRequest(kCamp, "Save 5 - Bob Primm") == "Co-op 00ab12 Host Save 5 - Bob Primm");
    CHECK(pt::hostSaveNameForRequest(kCamp, "Save 5 - Bob Primm") != pt::guestSaveName(kCamp, pt::hostSaveNameForRequest(kCamp, "Save 5 - Bob Primm")));   // host and guest saves never collide in a shared folder
    const std::string nasty = pt::guestSaveName(kCamp, "a:b*c?\"d<e>f|g\\h/i\tj");
    CHECK(launcherSafe(nasty));
    const std::string longOne = pt::guestSaveName(kCamp, std::string(300, 'x'));
    CHECK(launcherSafe(longOne) && longOne.size() <= 7 + 7 + pt::kMaxNamePart);
    CHECK(pt::guestSaveName(kCamp, "").find("Co-op 00ab12 ") == 0 && launcherSafe(pt::guestSaveName(kCamp, "...")));
    CHECK(pt::sameName("Save 4", "SAVE 4") && !pt::sameName("Save 4", "Save 5"));
}

static void testMetaAndIndex() {
    std::printf("party save: sidecar text, index (replace, pick)\n");
    pt::Meta m;
    m.campaign = kCamp;
    m.host = true;
    m.hostSave = "Save 4 - Ann Primm";
    pt::Meta back;
    CHECK(pt::parseMeta(pt::metaText(m), &back) && back.campaign == kCamp && back.host && back.hostSave == m.hostSave);
    m.host = false;
    CHECK(pt::parseMeta(pt::metaText(m), &back) && !back.host);
    CHECK(!pt::parseMeta("campaign=00ab12cd34ef5678\n", &back) && !pt::parseMeta("FNVCOOP-PARTY 1\nrole=host\n", &back) && !pt::parseMeta("", &back));
    CHECK(pt::metaText(m).find("FNVCOOP-PARTY 1\ncampaign=00ab12cd34ef5678\nrole=guest\n") == 0);   // the launcher reads exactly this

    pt::Index ix;
    ix.add({kCamp, "Save 3", "Co-op 00ab12 Save 3", 1000});
    ix.add({kCamp, "Save 4", "Co-op 00ab12 Save 4", 2000});
    ix.add({kCamp, "Save 5", "Co-op 00ab12 Save 5", 3000});
    ix.add({0xfeedull, "Save 3", "Co-op 00feed Save 3", 9000});
    ix.add({kCamp, "SAVE 4", "Co-op 00ab12 Save 4", 4000});   // the host saved over Save 4: one entry, the newer
    CHECK(ix.entries().size() == 4);                           // (twin 1109: five)
    const pt::Index again = pt::Index::parse(ix.text());
    CHECK(again.entries().size() == 4 && again.text() == ix.text());
    CHECK(ix.text().find("FNVCOOP-PARTY-INDEX 1\n") == 0 && ix.text().find("00ab12cd34ef5678\tSave 3\tCo-op 00ab12 Save 3\t1000\n") != std::string::npos);
    auto pick = [&](uint64_t c, const char* host, std::function<bool(const std::string&)> ex = nullptr) -> std::string {
        const pt::IndexEntry* e = ix.pick(c, host, ex);
        return e ? e->guestSave : std::string("-");
    };
    CHECK(pick(kCamp, "Save 3") == "Co-op 00ab12 Save 3");        // the save made for that host save (twin 1103 picks the newest)
    CHECK(pick(kCamp, "save 5") == "Co-op 00ab12 Save 5");
    CHECK(pick(kCamp, "Save 9") == "Co-op 00ab12 Save 4");        // no save for it: the newest of the campaign (Save 4 was rewritten at 4000)
    CHECK(pick(0xfeedull, "Save 9") == "Co-op 00feed Save 3");
    CHECK(pick(0x1234ull, "Save 3") == "-");                      // another campaign: nothing
    CHECK(pick(kCamp, "Save 5", [](const std::string& n) { return n != "Co-op 00ab12 Save 5"; }) == "Co-op 00ab12 Save 4");   // a deleted save is skipped
    CHECK(pt::Index::parse("junk").entries().empty() && pt::Index::parse("FNVCOOP-PARTY-INDEX 1\nbad line\n00ab12cd34ef5678\ta\tb\t\n").entries().empty());
}

static void testOrderTexts() {
    std::printf("party save: order texts\n");
    pt::SaveOrder o;
    o.campaign = kCamp;
    o.hostSave = "Save 4 - Ann Primm";
    o.seq = (123456ull << 24) | 7;
    Value v;
    std::string err;
    CHECK(nvcoop::json::parse(pt::saveOrderJson(o), v, err));
    pt::SaveOrder b;
    CHECK(pt::parseSaveOrder(v, &b) && b.campaign == kCamp && b.hostSave == o.hostSave && b.seq == o.seq && b.reqSlot == -1);   // a 52-bit sequence survives the JSON number
    o.reqSlot = 2;
    o.reqId = 9;
    CHECK(nvcoop::json::parse(pt::saveOrderJson(o), v, err) && pt::parseSaveOrder(v, &b) && b.reqSlot == 2 && b.reqId == 9);
    CHECK(nvcoop::json::parse(R"({"op":"partySave","campaign":"nope","hostSave":"x","seq":1})", v, err) && !pt::parseSaveOrder(v, &b));
    CHECK(nvcoop::json::parse(R"({"op":"partySave","campaign":"00ab12cd34ef5678","hostSave":"","seq":1})", v, err) && !pt::parseSaveOrder(v, &b));
    CHECK(nvcoop::json::parse(R"({"op":"partySave","campaign":"00ab12cd34ef5678","hostSave":"x","seq":0})", v, err) && !pt::parseSaveOrder(v, &b));
    CHECK(pt::isGuestOp("partySave") && pt::isGuestOp("campaign") && pt::isGuestOp("partySaveWait") && !pt::isGuestOp("kill"));
    CHECK(pt::isHostOp("partySaved") && pt::isHostOp("partySaveRequest") && !pt::isHostOp("note"));
}

// ---- the two machines against fakes -------------------------------------------------------------------------------------------------------------
struct Rig {
    struct G {
        pt::GuestParty* p = nullptr;
        bool canSave = true, linkUp = true;
        std::vector<std::string> saved, hud, sentToHost;
        std::string indexText;
        int slot = 0;
        bool saveOk = true;
    };
    uint64_t now = 100000;
    bool hostCanSave = true, hostSaveOk = true;
    std::vector<std::string> hostSaves, hostLog;
    std::vector<uint8_t> joined;
    std::map<uint8_t, bool> linkUp;                 // host -> guest delivery works
    std::map<uint8_t, std::vector<std::string>> inbox;   // orders each guest got
    std::vector<std::unique_ptr<G>> guests;         // slot = index + 1
    std::unique_ptr<pt::HostParty> host;
    std::vector<std::unique_ptr<pt::GuestParty>> parties;
    uint64_t rnd = 0x1111222233334444ull;

    explicit Rig(int n, bool withCampaign = true) {
        pt::HostEnv he;
        he.joined = [this] { return joined; };
        he.sendOrder = [this](uint8_t slot, const std::string& json) {
            if (!linkUp[slot]) return false;
            inbox[slot].push_back(json);
            Value v;
            std::string err;
            nvcoop::json::parse(json, v, err);
            pt::applyGuestOrder(*guests[slot - 1]->p, v);
            return true;
        };
        he.canSaveNow = [this] { return hostCanSave; };
        he.saveNow = [this](const std::string& name) {
            if (!hostSaveOk) return false;
            hostSaves.push_back(name);
            host->onHostSaved(name);   // the engine's SaveGame message arrives inside the save call
            return true;
        };
        he.log = [this](const std::string& s) { hostLog.push_back(s); };
        he.random64 = [this] { rnd = rnd * 6364136223846793005ull + 1442695040888963407ull; return rnd; };
        he.nowMs = [this] { return now; };
        host.reset(new pt::HostParty(he));
        if (withCampaign) host->setCampaign(kCamp);
        for (int i = 0; i < n; ++i) {
            auto g = std::make_unique<G>();
            G* gp = g.get();
            gp->slot = i + 1;
            pt::GuestEnv ge;
            ge.mySlot = [gp] { return gp->slot; };
            ge.sendToHost = [this, gp](const std::string& json) {
                if (!gp->linkUp) return false;
                gp->sentToHost.push_back(json);
                Value v;
                std::string err;
                nvcoop::json::parse(json, v, err);
                pt::handleHostOrder(*host, (uint8_t)gp->slot, v);
                return true;
            };
            ge.canSaveNow = [gp] { return gp->canSave; };
            ge.saveNow = [this, gp](const std::string& name) {
                if (!gp->saveOk) return false;
                gp->saved.push_back(name);
                gp->p->onLocalSave(name);   // the engine's SaveGame message arrives inside the save call
                return true;
            };
            ge.notify = [gp](const std::string& s) { gp->hud.push_back(s); };
            ge.wallSeconds = [this] { return (int64_t)(now / 1000); };
            ge.nowMs = [this] { return now; };
            ge.readIndex = [gp] { return gp->indexText; };
            ge.writeIndex = [gp](const std::string& t) { gp->indexText = t; return true; };
            parties.emplace_back(new pt::GuestParty(ge));
            gp->p = parties.back().get();
            joined.push_back((uint8_t)gp->slot);
            linkUp[(uint8_t)gp->slot] = true;
            guests.push_back(std::move(g));
        }
    }
    void frame(int n = 1) {
        for (int i = 0; i < n; ++i) {
            now += 50;
            host->tick();
            for (auto& g : guests) g->p->tick();
        }
    }
    void seconds(int s) { for (int i = 0; i < s * 20; ++i) frame(); }
};

static void testHostSaveEveryGuestSaves() {
    std::printf("party save: the host saves -> every joined guest saves its own game, once\n");
    Rig r(2);
    r.frame();
    const uint64_t seq1 = r.host->onHostSaved("C:\\x\\Saves\\Save 4 - Ann Primm.fos");
    r.seconds(2);
    for (auto& g : r.guests) {
        CHECK(g->saved.size() == 1 && g->saved[0] == "Co-op 00ab12 Save 4 - Ann Primm");
        CHECK(g->p->stats().saves == 1);
        const pt::Index ix = pt::Index::parse(g->indexText);
        CHECK(ix.entries().size() == 1 && ix.entries()[0].campaign == kCamp && ix.entries()[0].hostSave == "Save 4 - Ann Primm" &&
              ix.entries()[0].guestSave == "Co-op 00ab12 Save 4 - Ann Primm" && ix.entries()[0].time > 0);
        CHECK(g->sentToHost.size() == 1 && g->sentToHost[0].find("partySaved") != std::string::npos);
        CHECK(g->p->ownHostSave("Co-op 00ab12 Save 4 - Ann Primm.fos") == "Save 4 - Ann Primm");   // its sidecar will name the host save
        CHECK(g->p->ownHostSave("Save 9") == "");
    }
    CHECK(r.host->ackedCount(seq1) == 2);                       // the host knows both saved
    const uint64_t seq2 = r.host->onHostSaved("Quicksave");
    CHECK(seq2 != seq1 && seq2 != 0);
    r.seconds(2);
    CHECK(r.guests[0]->saved.size() == 2 && r.guests[0]->saved[1] == "Co-op 00ab12 Quicksave" && r.host->ackedCount(seq2) == 2);
    CHECK(pt::Index::parse(r.guests[0]->indexText).entries().size() == 2);
    r.host->onHostSaved("Quicksave");                           // quicksaved again: the same names, one index entry each
    r.seconds(2);
    CHECK(pt::Index::parse(r.guests[1]->indexText).entries().size() == 2 && r.guests[1]->saved.size() == 3);
    // a guest that is not in the session when the host saves gets no order
    Rig q(2);
    q.joined = {1};
    q.host->onHostSaved("Save 1");
    q.seconds(1);
    CHECK(q.guests[0]->saved.size() == 1 && q.guests[1]->saved.empty());
}

static void testGuestCannotSaveNow() {
    std::printf("party save: a guest that cannot save now saves as soon as it can, never twice for one sequence\n");
    Rig r(2);
    r.guests[0]->canSave = false;   // a menu or a scene on guest 1
    const uint64_t seq = r.host->onHostSaved("Save 7 - Ann Novac");
    r.seconds(3);
    CHECK(r.guests[0]->saved.empty() && r.guests[0]->p->pending() && r.guests[0]->p->stats().deferred == 1);   // (twin 1102 saves into the menu, twin 1108 forgets it)
    CHECK(r.guests[1]->saved.size() == 1 && r.host->ackedCount(seq) == 1);
    r.guests[0]->canSave = true;
    r.seconds(2);
    CHECK(r.guests[0]->saved.size() == 1 && !r.guests[0]->p->pending() && r.host->ackedCount(seq) == 2);   // and the host was told when it did
    // the same order again (a resend, a duplicate delivery): nothing
    r.host->onHostSaved("Save 7 - Ann Novac");
    r.seconds(2);
    const size_t n = r.guests[0]->saved.size();
    Value v;
    std::string err;
    std::vector<std::string> saves;   // the partySave orders guest 1 got, oldest first
    for (const auto& j : r.inbox[1]) if (j.find("partySave") != std::string::npos) saves.push_back(j);
    CHECK(saves.size() == 2);
    if (saves.size() == 2) {
        nvcoop::json::parse(saves[0], v, err);   // the first order of sequence `seq`
        pt::applyGuestOrder(*r.guests[0]->p, v);
        pt::applyGuestOrder(*r.guests[0]->p, v);
    }
    r.seconds(2);
    CHECK(r.guests[0]->saved.size() == n && r.guests[0]->p->stats().duplicates >= 2);                       // (twin 1101: saved again)
    // a still-pending order met by a duplicate: one save when the menu closes
    Rig s(1);
    s.guests[0]->canSave = false;
    s.host->onHostSaved("Save 8");
    s.seconds(1);
    Value w;
    nvcoop::json::parse(s.inbox[1].back(), w, err);
    pt::applyGuestOrder(*s.guests[0]->p, w);
    s.guests[0]->canSave = true;
    s.seconds(2);
    CHECK(s.guests[0]->saved.size() == 1);
    // the engine refuses the save a few times: it is tried again, then done once
    Rig t(1);
    t.guests[0]->saveOk = false;
    t.host->onHostSaved("Save 9");
    t.seconds(3);
    CHECK(t.guests[0]->saved.empty() && t.guests[0]->p->pending());
    t.guests[0]->saveOk = true;
    t.seconds(2);
    CHECK(t.guests[0]->saved.size() == 1);
}

static void testNewerHostSaveSupersedes() {
    std::printf("party save: a newer host save replaces a save that is still waiting\n");
    Rig r(1);
    r.guests[0]->canSave = false;
    const uint64_t a = r.host->onHostSaved("Save 1");
    r.seconds(1);
    const uint64_t b = r.host->onHostSaved("Save 2");
    r.seconds(1);
    r.guests[0]->canSave = true;
    r.seconds(2);
    CHECK(r.guests[0]->saved.size() == 1 && r.guests[0]->saved[0] == "Co-op 00ab12 Save 2");
    CHECK(r.guests[0]->p->stats().superseded == 1 && r.host->stats().skipped == 1 && r.host->ackedCount(a) == 1 && r.host->ackedCount(b) == 1);
    const pt::Index ix = pt::Index::parse(r.guests[0]->indexText);
    CHECK(ix.entries().size() == 1 && ix.entries()[0].hostSave == "Save 2");   // only the save that exists is indexed
}

static void testCampaignIsTold() {
    std::printf("party save: the campaign id reaches every guest, and again after a rejoin or a change\n");
    Rig r(2, false);
    r.frame(2);
    CHECK(r.guests[0]->p->campaign() == 0);            // no campaign yet: nothing to tell
    r.host->ensureCampaign();                          // the party starts from the start save
    const uint64_t id = r.host->campaign();
    CHECK(id != 0);
    r.frame(2);
    CHECK(r.guests[0]->p->campaign() == id && r.guests[1]->p->campaign() == id);   // (twin 1105)
    const size_t told = r.inbox[1].size();
    r.frame(5);
    CHECK(r.inbox[1].size() == told);                  // told once, not every frame
    r.joined = {2};                                    // guest 1 leaves ...
    r.frame(2);
    r.guests[0]->p->setCampaign(0);
    r.joined = {1, 2};                                 // ... and rejoins
    r.frame(2);
    CHECK(r.guests[0]->p->campaign() == id);
    r.host->setCampaign(0x5555ull);                    // the host loaded a save of another campaign
    r.frame(2);
    CHECK(r.guests[0]->p->campaign() == 0x5555ull && r.guests[1]->p->campaign() == 0x5555ull);
    // a host with no campaign that saves makes one: every host save belongs to a campaign
    Rig q(1, false);
    q.host->onHostSaved("Save 1");
    q.seconds(1);
    CHECK(q.host->campaign() != 0 && q.guests[0]->saved.size() == 1 && q.guests[0]->saved[0].find("Co-op " + pt::shortCampaign(q.host->campaign())) == 0);
}

static void testRebalanceSetting() {
    std::printf("party save: the campaign's rebalance setting (sidecar, campaign order, party size)\n");
    using nvcoop::rebalance::CampaignSettings;
    using nvcoop::rebalance::chooseSettings;
    const CampaignSettings bal = chooseSettings("balanced", true), nor = chooseSettings("normal", false);
    // the sidecar keeps it with the save, both roles; an older sidecar has none
    pt::Meta m;
    m.campaign = kCamp; m.host = true; m.hostSave = "Save 4"; m.settings = bal;
    pt::Meta back;
    CHECK(pt::parseMeta(pt::metaText(m), &back) && back.settings == bal);                                   // (twins 1260, 1200)
    m.host = false; m.settings = chooseSettings("normal", true);
    CHECK(pt::parseMeta(pt::metaText(m), &back) && back.settings.chosen && !back.settings.balanced() && back.settings.friendlyFire);
    CHECK(pt::parseMeta("FNVCOOP-PARTY 1\ncampaign=00ab12cd34ef5678\nrole=host\nhostSave=x\n", &back) && !back.settings.chosen);
    m.settings = CampaignSettings();
    CHECK(pt::metaText(m).find("rebalance") == std::string::npos);                                           // nothing written when none was chosen
    CHECK(pt::metaText(m).find("FNVCOOP-PARTY 1\ncampaign=00ab12cd34ef5678\nrole=guest\nhostSave=") == 0);   // the launcher reads the same first lines
    // the order: a host that sends no setting sends the old order text
    CHECK(pt::campaignOrderJson(kCamp) == "{\"op\":\"campaign\",\"id\":\"00ab12cd34ef5678\"}");

    // a new campaign takes what the host picked and keeps it; a loaded one keeps its own
    Rig r(2, false);
    r.host->setNewCampaignSettings(bal);
    r.frame(2);
    CHECK(r.guests[0]->p->campaign() == 0 && !r.guests[0]->p->settings().chosen);                          // no campaign yet: nothing told
    r.host->ensureCampaign();
    CHECK(r.host->settings() == bal);
    r.host->setNewCampaignSettings(nor);                                                                     // a later pick changes nothing of this campaign
    r.frame(2);
    CHECK(r.host->settings() == bal);
    CHECK(r.guests[0]->p->settings() == bal && r.guests[1]->p->settings() == bal);                           // every guest has it (twin 1201 through the order)
    CHECK(r.guests[0]->p->players() == 3 && r.guests[1]->p->players() == 3);                                // host + two guests
    // a guest joins / leaves: the balanced campaign tells the size again; a Normal one does not
    const size_t told = r.inbox[1].size();
    r.joined = {1};
    r.frame(2);
    CHECK(r.inbox[1].size() == told + 1 && r.guests[0]->p->players() == 2);                                  // (twin 1262)
    r.joined = {1, 2};
    r.frame(2);
    CHECK(r.guests[0]->p->players() == 3 && r.guests[1]->p->players() == 3);
    Rig n(2, false);
    n.host->setNewCampaignSettings(nor);
    n.host->ensureCampaign();
    n.frame(2);
    const size_t before = n.inbox[1].size();
    n.joined = {1};
    n.frame(2);
    n.joined = {1, 2};
    n.frame(2);
    CHECK(n.inbox[1].size() == before && n.guests[0]->p->settings() == nor);                                 // Normal does not care how many are playing
    // a host that loads a save of a campaign: that save's setting, not the new pick
    Rig l(1, false);
    l.host->setNewCampaignSettings(bal);
    l.host->setCampaign(0x77ull);
    l.host->setSettings(nor);
    l.frame(2);
    CHECK(l.guests[0]->p->settings() == nor && l.host->settings() == nor);
    // a loaded save without the setting stays without it (an old campaign plays Normal); the next campaign minted takes the pick
    Rig o(1, false);
    o.host->setNewCampaignSettings(bal);
    o.host->setCampaign(0x88ull);
    o.host->setSettings(CampaignSettings());
    o.frame(2);
    CHECK(!o.guests[0]->p->settings().chosen);
    o.host->setCampaign(0);
    o.host->ensureCampaign();
    CHECK(o.host->settings() == bal);
}

static void testOrdersSurviveABadLink() {
    std::printf("party save: an order the link could not take is sent again\n");
    Rig r(2);
    r.linkUp[2] = false;
    const uint64_t seq = r.host->onHostSaved("Save 1");
    r.seconds(2);
    CHECK(r.guests[0]->saved.size() == 1 && r.guests[1]->saved.empty());
    r.linkUp[2] = true;
    r.seconds(2);
    CHECK(r.guests[1]->saved.size() == 1 && r.host->ackedCount(seq) == 2);
    // the guest's note to the host is kept too
    Rig q(1);
    q.guests[0]->linkUp = false;
    const uint64_t s2 = q.host->onHostSaved("Save 2");
    q.seconds(2);
    CHECK(q.guests[0]->saved.size() == 1 && q.host->ackedCount(s2) == 0);
    q.guests[0]->linkUp = true;
    q.seconds(1);
    CHECK(q.host->ackedCount(s2) == 1);
}

static void testGuestHandSave() {
    std::printf("party save: a guest saves by hand -> the host makes the matching save, the requester adopts its own, the others save\n");
    Rig r(2);
    r.frame(2);
    // guest 1 presses Escape -> Save: the engine writes the save, the plugin sees the message
    CHECK(r.guests[0]->p->onLocalSave("Save 12 - Ann Primm.fos"));
    r.seconds(1);
    CHECK(r.guests[0]->p->stats().requests == 1 && r.host->stats().requests == 1);                         // (twin 1110: the host is never asked)
    CHECK(r.hostSaves.size() == 1 && r.hostSaves[0] == "Co-op 00ab12 Host Save 12 - Ann Primm");           // a real host save under a clear name derived from it
    CHECK(r.host->stats().requestSaves == 1);
    r.seconds(2);
    CHECK(r.guests[0]->saved.empty() && r.guests[0]->p->stats().adopted == 1);                              // the requester made no second save (twin 1104)
    CHECK(r.guests[1]->saved.size() == 1 && r.guests[1]->saved[0] == "Co-op 00ab12 Save 12 - Ann Primm");   // everybody else saved for the host save
    const pt::Index ix = pt::Index::parse(r.guests[0]->indexText);
    CHECK(ix.entries().size() == 1 && ix.entries()[0].hostSave == "Co-op 00ab12 Host Save 12 - Ann Primm" && ix.entries()[0].guestSave == "Save 12 - Ann Primm");
    // the continue pick finds it by the host save the host loads
    const pt::IndexEntry* e = ix.pick(kCamp, "Co-op 00ab12 Host Save 12 - Ann Primm");
    CHECK(e && e->guestSave == "Save 12 - Ann Primm");
    CHECK(r.host->queuedRequests() == 0);
    bool told = false;
    for (const auto& h : r.guests[0]->hud) told = told || h.find("Party save made") != std::string::npos;
    CHECK(told);
    // the saves a party save order makes are not hand saves: no request storm
    CHECK(r.guests[1]->p->stats().requests == 0 && r.host->stats().requests == 1);
    // a request sent twice (a resend) makes one host save
    r.host->onGuestRequest(1, 77, "Save 13");
    r.host->onGuestRequest(1, 77, "Save 13");
    CHECK(r.host->queuedRequests() == 1);
    r.seconds(2);
    CHECK(r.hostSaves.size() == 2);
    // a hand save outside a campaign is nobody's party save
    Rig q(1, false);
    CHECK(!q.guests[0]->p->onLocalSave("Save 1"));
}

static void testHostCannotSaveForAHandSave() {
    std::printf("party save: the host cannot save when a guest asks -> the guest is told, the party save follows\n");
    Rig r(2);
    r.frame(2);              // the guests learn the campaign
    r.hostCanSave = false;   // a SaveSuppress scene or a menu on the host
    CHECK(r.guests[0]->p->onLocalSave("Save 12 - Ann Primm"));
    r.seconds(3);
    CHECK(r.hostSaves.empty() && r.host->queuedRequests() == 1);                                            // (twin 1107 saves anyway)
    int waits = 0;
    for (const auto& h : r.guests[0]->hud) waits += h.find("cannot save") != std::string::npos;
    CHECK(waits == 1 && r.host->stats().waits == 1);                                                         // told once, not every frame
    CHECK(r.guests[1]->saved.empty());
    r.hostCanSave = true;
    r.seconds(2);
    CHECK(r.hostSaves.size() == 1 && r.guests[0]->p->stats().adopted == 1 && r.guests[1]->saved.size() == 1 && r.host->queuedRequests() == 0);
    // the engine refuses the host's save for a while: tried again, then done
    Rig q(1);
    q.frame(2);
    q.hostSaveOk = false;
    q.guests[0]->p->onLocalSave("Save 3");
    q.seconds(3);
    CHECK(q.hostSaves.empty() && q.host->queuedRequests() == 1);
    q.hostSaveOk = true;
    q.seconds(2);
    CHECK(q.hostSaves.size() == 1 && q.host->queuedRequests() == 0 && q.guests[0]->p->stats().adopted == 1);
    // a request, then the host saves by itself first: the host's own save is an ordinary party save (the hand save stays pending for its own)
    Rig s(2);
    s.frame(2);
    s.hostCanSave = false;
    s.guests[0]->p->onLocalSave("Save 20");
    s.seconds(1);
    s.host->onHostSaved("Save 99");
    s.seconds(2);
    CHECK(s.guests[0]->saved.size() == 1 && s.guests[0]->p->stats().adopted == 0 && s.guests[1]->saved.size() == 1);
    s.hostCanSave = true;
    s.seconds(2);
    CHECK(s.hostSaves.size() == 1 && s.guests[0]->p->stats().adopted == 1);
}

// ---- the real session: the orders travel over the order channel ---------------------------------------------------------------------------------
struct SessionNode {
    nvhooks::CaptureHub hub;
    FakeWorld world;
    std::unique_ptr<Runtime> rt;
    SessionNode(Role role, uint16_t port, uint64_t token) {
        Config c;
        c.enable = true;
        c.role = role;
        c.port = port;
        c.token = token;
        c.transport = "enet";
        rt.reset(new Runtime(c, &hub, &world, realNowMs));
        rt->setDumper(&world);
        rt->setActionExecutor([this](fnvcoop::PlayerId s, fnvcoop::IntentKind k, const fnvcoop::FormRef& r) { world.executeAction(hub, s, k, r); });
        rt->setWorldEffectApplier([this](const std::string& j) { return world.applyWorldJson(j); });
        std::string err;
        if (!rt->start(&err)) std::printf("  start failed: %s\n", err.c_str());
    }
};

static void testOverTheSession() {
    std::printf("party save: host save, hand save and acks over the real order channel (host + 2 guests)\n");
    SessionNode host(Role::Host, 0, 0);
    SessionNode a(Role::Client, host.rt->boundPort(), 7101), b(Role::Client, host.rt->boundPort(), 7102);
    bool hostCan = true;
    std::vector<std::string> hostSaves;
    pt::HostEnv he;
    pt::HostParty* hp = nullptr;
    he.joined = [&] { std::vector<uint8_t> v; for (auto s : host.rt->joinedSlots()) v.push_back((uint8_t)s); return v; };
    he.sendOrder = [&](uint8_t slot, const std::string& j) { return host.rt->sendOrder((fnvcoop::PlayerId)slot, j); };
    he.canSaveNow = [&] { return hostCan; };
    he.saveNow = [&](const std::string& n) { hostSaves.push_back(n); hp->onHostSaved(n); return true; };
    he.log = [](const std::string&) {};
    he.random64 = [] { return 0x0123456789abcdefull; };
    he.nowMs = realNowMs;
    pt::HostParty hostParty(he);
    hp = &hostParty;
    struct GNode { SessionNode* n; std::vector<std::string> saved, hud; std::string index; std::unique_ptr<pt::GuestParty> p; };
    GNode ga{&a, {}, {}, {}, nullptr}, gb{&b, {}, {}, {}, nullptr};
    for (GNode* g : {&ga, &gb}) {
        pt::GuestEnv ge;
        ge.mySlot = [g] { return g->n->rt->client() && g->n->rt->client()->joined() ? (int)g->n->rt->client()->slot() : -1; };
        ge.sendToHost = [g](const std::string& j) { return g->n->rt->sendOrderToHost(j); };
        ge.canSaveNow = [] { return true; };
        ge.saveNow = [g](const std::string& n) { g->saved.push_back(n); g->p->onLocalSave(n); return true; };
        ge.notify = [g](const std::string& t) { g->hud.push_back(t); };
        ge.log = [](const std::string&) {};
        ge.wallSeconds = [] { return (int64_t)1700000000; };
        ge.nowMs = realNowMs;
        ge.readIndex = [g] { return g->index; };
        ge.writeIndex = [g](const std::string& t) { g->index = t; return true; };
        g->p.reset(new pt::GuestParty(ge));
        pt::GuestParty* gp = g->p.get();
        g->n->rt->setOrderApplier([gp](const std::string& j) {
            Value v;
            std::string err;
            if (!nvcoop::json::parse(j, v, err)) return std::string("BadOrder");
            return pt::isGuestOp(v.get("op").asString("")) ? pt::applyGuestOrder(*gp, v) : std::string("UnknownOrder");
        });
    }
    host.rt->setOrderHandler([&](fnvcoop::PlayerId from, const std::string& j) {
        Value v;
        std::string err;
        if (!nvcoop::json::parse(j, v, err)) return std::string("BadOrder");
        return pt::isHostOp(v.get("op").asString("")) ? pt::handleHostOrder(hostParty, (uint8_t)from, v) : std::string("UnknownOrder");
    });
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); hostParty.tick(); ga.p->tick(); gb.p->tick(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined() && host.rt->host()->clients().size() == 2; }));
    hostParty.ensureCampaign();
    CHECK(pumpUntil(fr, [&] { return ga.p->campaign() == hostParty.campaign() && gb.p->campaign() == hostParty.campaign(); }));
    const std::string camp6 = pt::shortCampaign(hostParty.campaign());
    // 1. the host saves
    const uint64_t seq = hostParty.onHostSaved("Save 4 - Ann Primm");
    CHECK(pumpUntil(fr, [&] { return hostParty.ackedCount(seq) == 2; }));
    CHECK(ga.saved.size() == 1 && gb.saved.size() == 1 && ga.saved[0] == "Co-op " + camp6 + " Save 4 - Ann Primm");
    // 2. guest A saves by hand
    CHECK(ga.p->onLocalSave("Save 12 - Bob Primm"));
    CHECK(pumpUntil(fr, [&] { return ga.p->stats().adopted == 1 && gb.saved.size() == 2; }));
    CHECK(hostSaves.size() == 1 && hostSaves[0] == "Co-op " + camp6 + " Host Save 12 - Bob Primm" && ga.saved.size() == 1);
    CHECK(!hostSaves.empty() && pt::Index::parse(ga.index).pick(hostParty.campaign(), hostSaves[0]) != nullptr);
    // 3. the host cannot save for guest B's hand save: B is told, then it happens
    hostCan = false;
    CHECK(gb.p->onLocalSave("Save 13 - Cy Primm"));
    CHECK(pumpUntil(fr, [&] { return !gb.hud.empty() && gb.hud.back().find("cannot save") != std::string::npos; }));
    CHECK(hostSaves.size() == 1);
    hostCan = true;
    CHECK(pumpUntil(fr, [&] { return gb.p->stats().adopted == 1 && ga.saved.size() == 2; }));
    CHECK(hostSaves.size() == 2 && gb.saved.size() == 2);
}

int main() {
    std::setvbuf(stdout, nullptr, _IONBF, 0);   // a crashing twin still shows where
    testNames();
    testMetaAndIndex();
    testOrderTexts();
    testHostSaveEveryGuestSaves();
    testGuestCannotSaveNow();
    testNewerHostSaveSupersedes();
    testCampaignIsTold();
    testRebalanceSetting();
    testOrdersSurviveABadLink();
    testGuestHandSave();
    testHostCannotSaveForAHandSave();
    testOverTheSession();
    std::printf("%d checks, %d failed\n", g_checks, g_failed);
    return g_failed ? 1 : 0;
}
