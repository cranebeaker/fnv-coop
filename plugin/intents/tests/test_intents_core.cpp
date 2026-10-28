// Game-free tests of the guest-intents core: wire, ini, guest and host policy, container redirect, the host pipeline, the guest client, the save gate.
// Runtime twins 301..331 (330: actor mode reported on every action, 331: a stand-in is never reported) (tools/mut/fnvcoop_mut.h): the same binary is run once per twin with FNVCOOP_MUT=<n> and every one must fail a check.
#include <cstring>

#include "fnvcoop/rules.h"
#include "fnvcoop/session.h"
#include "fnvcoop/transport.h"
#include "fnvcoop/wire.h"
#include "host_rig.h"
#include "intents/intent_service.h"
#include "minitest.h"

using namespace nvcoop::intents;
using namespace nvcoop::intents::testing;
using fnvcoop::AddItem;
using fnvcoop::DisableRef;
using fnvcoop::SetStage;

namespace {

Config on() { return testConfig(); }

fnvcoop::Effect stage(uint32_t q, uint32_t s) { return SetStage{R(q), s}; }

}  // namespace

// ---- wire -------------------------------------------------------------------------------------------------------------------------------

TEST(intents_wire, round_trip_every_kind) {
    for (uint8_t k = 1; k < static_cast<uint8_t>(Kind::Count_); ++k) {
        Intent in;
        in.session = 0x12345; in.id = 77 + k; in.kind = static_cast<Kind>(k);
        in.target = R(0xABCDEF); in.aux = R(0x42, "other.esp"); in.hasAux = (k % 2) == 0; in.value = -5 * k;
        in.path = {1, 2, 3};
        Bytes b = encode(in);
        Intent out;
        CHECK(decode(b, &out, nullptr) == WireType::Intent);
        CHECK(out.session == in.session && out.id == in.id && out.kind == in.kind && out.target == in.target && out.value == in.value && out.path == in.path);
        CHECK(out.hasAux == in.hasAux && (!in.hasAux || out.aux == in.aux));
    }
    Verdict v;
    v.session = 9; v.id = 3; v.result = Result::Busy; v.holder = 2; v.hasRedirect = true; v.redirect = R(0x99, "mine.esp");
    Bytes b = encode(v);
    Verdict o;
    CHECK(decode(b, nullptr, &o) == WireType::Verdict);
    CHECK(o.session == 9 && o.id == 3 && o.result == Result::Busy && o.holder == 2 && o.hasRedirect && o.redirect == v.redirect);
    CHECK(b[0] == kChannelIntents && b[1] == kIntentsWire);
}

TEST(intents_wire, hostile_input) {
    Intent in; in.session = 1; in.id = 1; in.kind = Kind::TerminalUse; in.target = R(5); in.path = {1, 2};
    Bytes good = encode(in);
    Intent out;
    for (size_t n = 0; n < good.size(); ++n) CHECK(decode(good.data(), n, &out, nullptr) == WireType::Bad);   // every truncation
    Bytes trailing = good; trailing.push_back(0);
    CHECK(decode(trailing, &out, nullptr) == WireType::Bad);
    Bytes wrongChannel = good; wrongChannel[0] = 0xCB;
    CHECK(decode(wrongChannel, &out, nullptr) == WireType::Bad);
    Bytes wrongVersion = good; wrongVersion[1] = 99;
    CHECK(decode(wrongVersion, &out, nullptr) == WireType::Bad);
    Bytes wrongType = good; wrongType[2] = 9;
    CHECK(decode(wrongType, &out, nullptr) == WireType::Bad);
    Verdict vout;
    CHECK(decode(good, nullptr, &vout) == WireType::Intent);   // an Intent is not mistaken for a Verdict (it reports its own type)
    // single-bit flips never crash and never produce an out-of-range enum
    for (size_t i = 3; i < good.size(); ++i)
        for (int bit = 0; bit < 8; ++bit) {
            Bytes f = good; f[i] ^= uint8_t(1u << bit);
            Intent o2; Verdict v2;
            WireType t = decode(f.data(), f.size(), &o2, &v2);
            if (t == WireType::Intent) CHECK(o2.kind != static_cast<Kind>(0) && o2.kind < Kind::Count_ && o2.path.size() <= kMaxPath);
        }
    // an over-long path (hostile sender) is rejected, not truncated
    fnvcoop::Archive a = fnvcoop::Archive::writer();
    uint32_t s = 1, id = 1; uint8_t k = static_cast<uint8_t>(Kind::TerminalUse);
    a.varu32(s); a.varu32(id); a.u8(k);
    FormRef t = R(5); fnvcoop::io(a, t);
    bool no = false; a.boolean(no);
    int32_t val = 0; a.vari32(val);
    std::vector<uint8_t> longPath(20, 1); a.bytes(longPath, 255);
    Bytes body = a.finish(), dg = {kChannelIntents, kIntentsWire, 1};
    dg.insert(dg.end(), body.begin(), body.end());
    CHECK(decode(dg, &out, nullptr) == WireType::Bad);
    // unknown kind / result
    Intent badKind = in; badKind.kind = static_cast<Kind>(77);
    Bytes bk = encode(badKind);
    CHECK(decode(bk, &out, nullptr) == WireType::Bad);
    Verdict badRes; badRes.result = static_cast<Result>(99);
    Bytes br = encode(badRes);
    CHECK(decode(br, nullptr, &vout) == WireType::Bad);
    Verdict badHolder; badHolder.holder = 9;
    CHECK(decode(encode(badHolder), nullptr, &vout) == WireType::Bad);
}

// ---- ini --------------------------------------------------------------------------------------------------------------------------------

TEST(intents_config, defaults_and_parse) {
    Config d;
    CHECK(!d.enable && !d.hit && !d.dialogue && d.activate && d.pickup && d.standIn && !d.standInUnsafe);   // inert by default; hit / dialogue are opt-in
    Config c;
    std::string err;
    CHECK(parseConfig("[coop]\nenable=1\n[intents]\nEnable=1\nhit = on ; why not\nPICKUP=0\nreach_activate=300\nrate_burst=5\nredirect_file = nvcoop_intents.json\n", &c, &err));
    CHECK(c.enable && c.hit && !c.pickup && c.reachActivate == 300.f && c.rateBurst == 5 && c.redirectFile == "nvcoop_intents.json");
    Config other;
    CHECK(parseConfig("[coop]\nenable=1\n", &other, &err) && !other.enable);   // only [intents] counts
    Config bad;
    CHECK(!parseConfig("[intents]\nenable=banana\n", &bad, &err) && err.find("enable") != std::string::npos);
    CHECK(!parseConfig("[intents]\nreach_activate=far\n", &bad, &err));
    Config clamp;
    CHECK(parseConfig("[intents]\nrate_burst=999999\nreach_pickup=-4\nrate_per_sec=0\n", &clamp, &err));
    CHECK(clamp.rateBurst == 1000 && clamp.reachPickup == 1.f && clamp.ratePerSec == 0.1f);
}

// ---- guest policy -----------------------------------------------------------------------------------------------------------------------

TEST(intents_policy, guest_activate_table) {
    Config c = on();
    auto t = [](TargetClass cls, bool dead = false) { TargetInfo i = mkTarget(cls, 1); i.dead = dead; return i; };
    GuestDecision d = decideActivate(c, t(TargetClass::Door));
    CHECK(d.forward && d.kind == Kind::Activate && !d.suppressLocal);          // animations stay local, the script half runs on the host
    d = decideActivate(c, t(TargetClass::Item));
    CHECK(d.forward && d.kind == Kind::PickUp && d.suppressLocal);             // a claim: the host decides, nothing is picked up locally
    d = decideActivate(c, t(TargetClass::Container));
    CHECK(d.forward && d.kind == Kind::ContainerOpen && !d.suppressLocal);
    d = decideActivate(c, t(TargetClass::Npc, true));                          // a body is looted like a chest
    CHECK(d.forward && d.kind == Kind::ContainerOpen);
    d = decideActivate(c, t(TargetClass::Terminal));
    CHECK(d.forward && d.kind == Kind::Activate && !d.suppressLocal);
    d = decideActivate(c, t(TargetClass::Npc));
    CHECK(!d.forward && !d.viaDialogue);                                       // dialogue off: vanilla talk, nothing forwarded
    Config withDlg = c; withDlg.dialogue = true;
    d = decideActivate(withDlg, t(TargetClass::Npc));
    CHECK(!d.forward && d.viaDialogue && d.kind == Kind::DialogueStart);
    CHECK(!decideActivate(c, t(TargetClass::Static)).forward && !decideActivate(c, t(TargetClass::Creature)).forward);
    TargetInfo unloaded = t(TargetClass::Door); unloaded.loaded = false;
    CHECK(!decideActivate(c, unloaded).forward);
    Config off = c; off.enable = false;
    CHECK(!decideActivate(off, t(TargetClass::Door)).forward);
    Config noPick = c; noPick.pickup = false;
    CHECK(!decideActivate(noPick, t(TargetClass::Item)).forward);
    Config noCont = c; noCont.container = false;
    CHECK(!decideActivate(noCont, t(TargetClass::Container)).forward);
    Config noAct = c; noAct.activate = false;
    CHECK(!decideActivate(noAct, t(TargetClass::Door)).forward && decideActivate(noAct, t(TargetClass::Item)).forward);
}

TEST(intents_policy, guest_events) {
    Config c = on();
    TargetInfo trig = mkTarget(TargetClass::Trigger, 1), act = mkTarget(TargetClass::Activator, 2), npc = mkTarget(TargetClass::Npc, 3);
    GuestDecision d = decideEvent(c, ev::OnTriggerEnter, trig);
    CHECK(d.forward && d.kind == Kind::TriggerEnter && d.suppressLocal);
    d = decideEvent(c, ev::OnTriggerLeave, trig);
    CHECK(d.forward && d.kind == Kind::TriggerLeave && d.suppressLocal);
    d = decideEvent(c, ev::OnTrigger, trig);
    CHECK(d.forward && d.kind == Kind::TriggerStay && d.suppressLocal);       // the every-frame event becomes a throttled keep-alive (G04, test_intents_campaign.cpp)
    CHECK(!decideEvent(c, ev::OnHit, act).forward);                            // hit is opt-in
    Config hit = c; hit.hit = true;
    d = decideEvent(hit, ev::OnHit, act);
    CHECK(d.forward && d.kind == Kind::Hit && !d.suppressLocal);
    CHECK(!decideEvent(hit, ev::OnHit, npc).forward);                          // actors: the combat channel raises OnHit (twin 305 forwards them too)
    CHECK(!decideEvent(hit, ev::OnAdd, act).forward && !decideEvent(hit, ev::OnOpen, act).forward);
    Config noTrig = c; noTrig.trigger = false;
    CHECK(!decideEvent(noTrig, ev::OnTriggerEnter, trig).forward);
}

TEST(intents_policy, host_standin_matrix) {
    Config c = on();
    for (TargetClass cls : {TargetClass::Door, TargetClass::Activator, TargetClass::Furniture, TargetClass::Trigger})
        CHECK(standInAllowed(c, Kind::Activate, cls));
    for (TargetClass cls : {TargetClass::Container, TargetClass::Terminal, TargetClass::Npc, TargetClass::Unknown})
        CHECK(!standInAllowed(c, Kind::Activate, cls));                        // would open a menu on the HOST's screen
    CHECK(!standInAllowed(c, Kind::ContainerOpen, TargetClass::Container));
    CHECK(standInAllowed(c, Kind::PickUp, TargetClass::Item) && standInAllowed(c, Kind::TriggerEnter, TargetClass::Trigger) &&
          standInAllowed(c, Kind::TerminalUse, TargetClass::Terminal));
    Config unsafe = c; unsafe.standInUnsafe = true;
    CHECK(standInAllowed(unsafe, Kind::Activate, TargetClass::Container));
    Config strict = c; strict.requirePuppet = true;
    CHECK(!standInAllowed(strict, Kind::PickUp, TargetClass::Item));
    Config none = c; none.standIn = false;
    CHECK(!standInAllowed(none, Kind::Activate, TargetClass::Door));
    CHECK(reachOf(c, Kind::Activate) == c.reachActivate && reachOf(c, Kind::TriggerEnter) == c.reachTrigger && reachOf(c, Kind::DialogueStart) == 0.f);
    CHECK(kindEnabled(c, Kind::PickUp) && !kindEnabled(c, Kind::Hit) && !kindEnabled(c, Kind::DialogueStart));
}

// ---- container redirect -----------------------------------------------------------------------------------------------------------------

TEST(intents_redirect, per_slot_storage) {
    ContainerRedirect r;
    std::string err;
    const char* json = R"({"redirects":[{"target":"FalloutNV.esm:12AB34","slots":{"0":"falloutnv.esm:12AB34","1":"mymod.esp:000801","2":"MyMod.esp:000802"}}]})";
    CHECK(r.load(json, &err));
    FormRef shared = R(0x12AB34), out;
    CHECK(r.isShared(shared) && r.size() == 1);
    CHECK(r.resolve(shared, 1, &out) && out == FormRef("mymod.esp", 0x801));
    CHECK(r.resolve(shared, 2, &out) && out == FormRef("mymod.esp", 0x802));   // plugin name lower-cased
    CHECK(r.resolve(shared, 0, &out) && out == shared);
    CHECK(!r.resolve(shared, 3, &out));                                        // no storage for slot 3: refused, never somebody else's chest
    CHECK(!r.isShared(R(1)) && !r.resolve(R(1), 1, &out));
    CHECK(!r.load("{\"redirects\":[{\"target\":\"nonsense\",\"slots\":{\"1\":\"a.esp:1\"}}]}", &err) && r.size() == 1);   // a bad file keeps the old table
    CHECK(!r.load("{\"redirects\":[{\"target\":\"a.esp:1\",\"slots\":{\"9\":\"a.esp:2\"}}]}", &err));
    CHECK(!r.load("{\"redirects\":[{\"target\":\"a.esp:1\",\"slots\":{}}]}", &err));
    CHECK(!r.load("not json", &err));
    FormRef f;
    CHECK(parseFormRef("A.ESM:00ab", &f) && f == FormRef("a.esm", 0xAB));
    CHECK(!parseFormRef(":12", &f) && !parseFormRef("a.esm:", &f) && !parseFormRef("a.esm:xyz", &f) && !parseFormRef("a.esm:1000000", &f));
}

// ---- host pipeline ----------------------------------------------------------------------------------------------------------------------

TEST(intents_host, activate_runs_as_the_puppet_inside_the_guest_window) {
    HostRig h;
    h.add(0x500, TargetClass::Door, 100, 0, 0);
    h.eng.positions[0x1001] = {120, 0, 0};
    h.eng.onCall = [&](const FakeEngine::Call& c) { if (c.what == "activate") h.hub.record(stage(0x600, 10)); };   // the door's script advances a quest
    Verdict v = h.run(1, Kind::Activate, R(0x500), 1);
    CHECK(v.result == Result::Executed);
    REQUIRE(h.eng.calls.size() == 1);
    const auto& c = h.eng.calls[0];
    CHECK(c.what == "activate" && c.actor == 0x1001 && c.contextActor == 0x1001 && c.contextSlot == 1 && c.depth == 1);   // `player` is the puppet
    CHECK(h.eng.outsideScope == 0);
    REQUIRE(h.commits.size() == 1);
    CHECK(h.commits[0].origin == 1);                                                                                      // the guest is the origin
    CHECK(h.commits[0].txn.cause.kind == CauseKind::Intent && h.commits[0].txn.cause.source == R(0x500) && h.commits[0].txn.cause.aux == int(Kind::Activate));
    CHECK(h.effects<SetStage>() == 1);
    REQUIRE(h.sent.size() == 1);
    Verdict back;
    CHECK(decode(h.sent[0].second, nullptr, &back) == WireType::Verdict && h.sent[0].first == 1 && back.id == 1 && back.result == Result::Executed);
    CHECK(h.host->stats().actedAsPuppet == 1 && h.host->stats().executed == 1 && h.host->stats().byKind[int(Kind::Activate)] == 1);
}

TEST(intents_host, actor_choice_puppet_ghost_standin) {
    HostRig h;
    h.add(0x500, TargetClass::Door);
    h.add(0x501, TargetClass::Container);
    h.add(0x502, TargetClass::Terminal);
    h.puppets.clear();
    h.ghosts[1] = 0x7001;
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::Executed && h.eng.calls.back().actor == 0x7001);   // NV:MP ghost when there is no puppet
    CHECK(h.host->stats().actedAsGhost == 1);
    h.ghosts.clear();
    CHECK(h.run(1, Kind::Activate, R(0x500), 2).result == Result::Executed && h.eng.calls.back().actor == kHostPlayerRef);   // stand-in for a door
    CHECK(h.host->stats().actedAsStandIn == 1);
    size_t before = h.eng.calls.size();
    CHECK(h.run(1, Kind::Activate, R(0x502), 3).result == Result::NoPuppet);                                          // a terminal would open on the host
    CHECK(h.eng.calls.size() == before);
    h.cfg.requirePuppet = true;
    h.build();
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::NoPuppet);
    h.cfg.requirePuppet = false; h.cfg.standIn = false;
    h.build();
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::NoPuppet);
    h.puppets[1] = 0x1001;
    CHECK(h.run(1, Kind::Activate, R(0x500), 2).result == Result::Executed && h.eng.calls.back().actor == 0x1001);
}

// A3 (2026-10-22): with [live] avatars=1 a guest's action runs as its avatar puppet (actedAs.puppet), never as the host's player; with avatars off the
// stand-in stays but is reported (the module logs ACTING-STANDIN), once per change, not once per action.
TEST(intents_host, puppet_wins_and_the_stand_in_is_reported_on_change) {
    HostRig h;
    h.add(0x500, TargetClass::Door);
    for (uint32_t i = 1; i <= 3; ++i) CHECK(h.run(1, Kind::Activate, R(0x500), i).result == Result::Executed);   // avatars=1: puppet 0x1001
    CHECK(h.host->stats().actedAsPuppet == 3 && h.host->stats().actedAsStandIn == 0);
    for (const auto& c : h.eng.calls) CHECK(c.actor == 0x1001 && c.actor != kHostPlayerRef);                       // never the host player
    REQUIRE(h.modes.size() == 1);
    CHECK(std::get<0>(h.modes[0]) == 1 && std::get<1>(h.modes[0]) == ActorMode::Puppet && std::get<2>(h.modes[0]) == 0x1001);
    h.puppets.clear();                                                                                              // avatars off (or the puppet is gone)
    for (uint32_t i = 4; i <= 5; ++i) CHECK(h.run(1, Kind::Activate, R(0x500), i).result == Result::Executed);
    CHECK(h.host->stats().actedAsStandIn == 2 && h.eng.calls.back().actor == kHostPlayerRef);
    REQUIRE(h.modes.size() == 2);                                                                                   // reported once for the change
    CHECK(std::get<1>(h.modes[1]) == ActorMode::StandIn && std::get<2>(h.modes[1]) == kHostPlayerRef);
    h.puppets[1] = 0x1001;
    CHECK(h.run(1, Kind::Activate, R(0x500), 6).result == Result::Executed);
    REQUIRE(h.modes.size() == 3);
    CHECK(std::get<1>(h.modes[2]) == ActorMode::Puppet);
}

TEST(intents_host, container_without_puppet_still_locks_and_skips_the_script) {
    HostRig h;
    h.add(0x501, TargetClass::Container);
    h.puppets.clear();
    CHECK(h.run(1, Kind::ContainerOpen, R(0x501), 1).result == Result::Executed);
    CHECK(h.eng.calls.empty() && h.host->stats().scriptsSkipped == 1);                                                  // no Activate on the host's screen
    CHECK(h.host->locks().size() == 1);
}

// Story fix 2026-10-28: a refused intent writes a log line with the reason, both positions, the distance, the ref and the cell (the first Goodsprings story run
// counted OutOfRange / NotLoaded for a guest standing next to an NPC and the plugin said nothing about any of them). At most one line per kind and reason
// per second; the lines skipped in between are counted in the next one.
TEST(intents_host, a_refused_intent_is_logged_with_both_positions_rate_limited) {
    HostRig h;
    h.add(0x500, TargetClass::Door, 1000, 0, 0);
    h.eng.world[R(0x500)].cell = 0xA5;
    h.eng.positions[0x1001] = {0, 20, 0};
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::OutOfRange);
    REQUIRE(h.logLines.size() == 1);
    const std::string& l = h.logLines[0];
    CHECK(l.find("REFUSED Activate from slot 1: OutOfRange") != std::string::npos);
    CHECK(l.find("falloutnv.esm:500") != std::string::npos);
    CHECK(l.find("pos (1000,0,0)") != std::string::npos);                    // the target
    CHECK(l.find("pos (0,20,0)") != std::string::npos);                      // the guest's puppet
    CHECK(l.find("puppet 1001") != std::string::npos);
    CHECK(l.find("cell a5") != std::string::npos);
    CHECK(l.find("distance 1000 (reach 450)") != std::string::npos);
    CHECK(h.run(1, Kind::Activate, R(0x500), 2).result == Result::OutOfRange);   // the same kind and reason inside a second: quiet
    CHECK(h.run(1, Kind::Activate, R(0x500), 3).result == Result::OutOfRange);
    CHECK(h.logLines.size() == 1 && h.host->stats().refusalsQuiet == 2);
    h.now += 1500;
    CHECK(h.run(1, Kind::Activate, R(0x500), 4).result == Result::OutOfRange);
    REQUIRE(h.logLines.size() == 2);
    CHECK(h.logLines[1].find("2 more of this kind and reason") != std::string::npos);
    h.add(0x502, TargetClass::Npc);
    h.eng.world[R(0x502)].loaded = false;
    CHECK(h.run(1, Kind::Activate, R(0x502), 5).result == Result::NotLoaded);       // another reason is its own line, and says the target is not loaded
    REQUIRE(h.logLines.size() == 3);
    CHECK(h.logLines[2].find("NOT-loaded") != std::string::npos);
}

// Story fix 2026-10-28 (S07): the client harvested a Broc flower far from the host. The host's copy of the plant sat in a cell the host never loaded, the
// activation was refused as NotLoaded, and the guest ended with its local half only (XP, no ingredient, the plant still there). The plant is an activator whose script
// is all there is to run: it runs, as the guest's acting player, and the reach (a stale puppet position in another cell) is not judged for it.
TEST(intents_host, an_activator_in_a_cell_the_host_never_loaded_still_runs_its_script) {
    HostRig h;
    TargetInfo plant = mkTarget(TargetClass::Activator, 0x530, -61195, 12991, 10277);
    plant.loaded = false;
    h.eng.world[R(0x530)] = plant;
    h.eng.positions[0x1001] = {1260, 1929, 7361};                                      // the puppet is 63000 units away, in Doc's house
    h.eng.onCall = [&](const FakeEngine::Call& c) { if (c.what == "activate") h.hub.record(fnvcoop::EnableRef{R(0x534)}); };   // the script does something for a puppet
    CHECK(h.run(1, Kind::Activate, R(0x530), 1).result == Result::Executed);
    REQUIRE(h.eng.calls.size() == 1);
    CHECK(h.eng.calls[0].actor == 0x1001);                                             // as the guest's puppet
    CHECK(h.host->stats().ranUnloaded == 1 && h.host->stats().reachChecked == 0);
    TargetInfo door = mkTarget(TargetClass::Door, 0x531);
    door.loaded = false;
    h.eng.world[R(0x531)] = door;
    CHECK(h.run(1, Kind::Activate, R(0x531), 2).result == Result::NotLoaded);          // anything that needs the 3D world still needs the cell
    TargetInfo npc = mkTarget(TargetClass::Npc, 0x532);
    npc.loaded = false;
    h.eng.world[R(0x532)] = npc;
    CHECK(h.run(1, Kind::Activate, R(0x532), 3).result == Result::NotLoaded);
    TargetInfo ghost = plant;                                                          // a form that is not in memory at all (the plugin is loaded, the ref is not)
    ghost.ref = 0x533;
    ghost.resident = false;
    h.eng.world[R(0x533)] = ghost;
    CHECK(h.run(1, Kind::Activate, R(0x533), 4).result == Result::NotLoaded);
    h.cfg.runUnloaded = false;
    h.build();
    h.eng.calls.clear();
    CHECK(h.run(1, Kind::Activate, R(0x530), 1, 2).result == Result::NotLoaded && h.eng.calls.empty());   // switched off: refused as before
}

// Story fix 2026-10-28 (S07): the Broc flower's script says `if State == 0 && GetActionRef == player`, a puppet is not `player`, so the host ran it to no effect and the
// guest got no ingredient. An activation to no effect is redone by the host's own player standing in; the script's `player.AddItem` is captured for the guest
// (Recipient default = the txn origin) and the host does not keep the ingredient. A puppet activation that did something is NOT redone.
TEST(intents_host, an_activation_that_does_nothing_for_a_puppet_is_redone_by_the_stand_in_and_the_host_keeps_nothing) {
    HostRig h;
    h.add(0x540, TargetClass::Activator, 100, 0, 0);
    h.eng.world[R(0x540)].script = R(0xE9ACE);                                         // BrocFlowerScript
    h.retry.add(R(0xE9ACE));                                                           // ... which the generated list names
    h.eng.positions[0x1001] = {0, 0, 0};
    const FormRef broc = R(0x13B2BA);
    h.eng.onCall = [&](const FakeEngine::Call& c) {
        if (c.what != "activate" || c.actor != kHostPlayerRef) return;                  // only `GetActionRef == player` runs the script
        h.eng.hostItems[broc] += 1;                                                     // the engine gives `player` the ingredient ...
        h.hub.record(fnvcoop::AddItem{broc, 1, fnvcoop::Recipient::dflt()});            // ... and the capture records it
    };
    CHECK(h.run(1, Kind::Activate, R(0x540), 1).result == Result::Executed);
    REQUIRE(h.eng.calls.size() == 2);
    CHECK(h.eng.calls[0].actor == 0x1001 && h.eng.calls[1].actor == kHostPlayerRef);   // the puppet first, then the stand-in
    CHECK(h.host->stats().standInRetries == 1);
    REQUIRE(h.commits.size() == 1);
    CHECK(h.commits[0].origin == 1);                                                    // committed with the guest as origin: the guest gets the ingredient
    CHECK(h.effects<fnvcoop::AddItem>() == 1);
    CHECK(h.eng.hostItems[broc] == 0 && h.eng.hostReverted == 1 && h.host->stats().standInGainsReverted == 1);   // the host's own player does not keep it
    // an activation the puppet could do is not redone
    h.add(0x541, TargetClass::Activator, 100, 0, 0);
    h.eng.world[R(0x541)].script = R(0xE9ACE);
    h.eng.onCall = [&](const FakeEngine::Call& c) { if (c.what == "activate") h.hub.record(fnvcoop::EnableRef{R(0x542)}); };
    h.eng.calls.clear();
    CHECK(h.run(1, Kind::Activate, R(0x541), 2).result == Result::Executed);
    CHECK(h.eng.calls.size() == 1 && h.host->stats().standInRetries == 1);
    // switched off: the old behaviour (nothing for the guest)
    h.cfg.standInRetry = false;
    h.build();
    h.eng.calls.clear();
    h.eng.onCall = nullptr;
    CHECK(h.run(1, Kind::Activate, R(0x540), 1, 2).result == Result::Executed && h.eng.calls.size() == 1);
}

// Follow-ups 1, item 2. The second run puts a script's OnActivate through the engine with the HOST's player in it: a vending machine's message box would pop on the host's screen, a
// switch would be thrown twice. Only the scripts the generator listed (they test `GetActionRef == player` and leave no host-local trace) are redone; anything else keeps the
// puppet's run alone, and so does everything when the list is empty or missing.
TEST(intents_host, only_the_listed_scripts_are_redone_by_the_stand_in) {
    HostRig h;
    h.add(0x550, TargetClass::Activator, 100, 0, 0);
    h.add(0x551, TargetClass::Activator, 100, 0, 0);
    h.add(0x552, TargetClass::Unknown, 100, 0, 0);                                     // a flora plant is no TESObjectACTI: the engine's class is Unknown
    h.eng.world[R(0x550)].script = R(0xE9ACE);                                         // listed
    h.eng.world[R(0x551)].script = R(0x1234);                                          // a vending machine: tests the actor too, but is not in the list
    h.eng.world[R(0x552)].script = R(0xE9ACE);
    h.retry.add(R(0xE9ACE));
    h.eng.positions[0x1001] = {0, 0, 0};
    CHECK(h.run(1, Kind::Activate, R(0x550), 1).result == Result::Executed);
    CHECK(h.eng.calls.size() == 2 && h.host->stats().standInRetries == 1);             // redone
    h.eng.calls.clear();
    CHECK(h.run(1, Kind::Activate, R(0x551), 2).result == Result::Executed);
    CHECK(h.eng.calls.size() == 1 && h.eng.calls[0].actor == 0x1001 && h.host->stats().standInRetries == 1);   // not listed: the puppet's run alone
    h.eng.calls.clear();
    CHECK(h.run(1, Kind::Activate, R(0x552), 3).result == Result::Executed);
    CHECK(h.eng.calls.size() == 2 && h.host->stats().standInRetries == 2);             // the class is not what decides: the script is
    h.eng.world[R(0x550)].script = FormRef();                                          // a reference whose script could not be read
    h.eng.calls.clear();
    CHECK(h.run(1, Kind::Activate, R(0x550), 4).result == Result::Executed);
    CHECK(h.eng.calls.size() == 1 && h.host->stats().standInRetries == 2);
    h.retry.clear();                                                                   // no list (the data file is missing): nothing is ever redone
    h.eng.calls.clear();
    CHECK(h.run(1, Kind::Activate, R(0x552), 5).result == Result::Executed);
    CHECK(h.eng.calls.size() == 1 && h.host->stats().standInRetries == 2);
}

TEST(intents_host, reach) {
    HostRig h;
    h.add(0x500, TargetClass::Door, 1000, 0, 0);
    h.eng.positions[0x1001] = {0, 0, 0};
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::OutOfRange);                                         // 1000 away, reach 450
    CHECK(h.eng.calls.empty() && h.commits.empty());
    h.eng.positions[0x1001] = {900, 0, 0};
    CHECK(h.run(1, Kind::Activate, R(0x500), 2).result == Result::Executed);
    h.eng.positions.erase(0x1001);                                                                                       // puppet position unknown: not checkable
    CHECK(h.run(1, Kind::Activate, R(0x500), 3).result == Result::Executed && h.host->stats().reachUnchecked >= 1);
    h.cfg.checkReach = false;
    h.build();
    h.eng.positions[0x1001] = {0, 0, 0};
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::Executed);
}

TEST(intents_host, exactly_once_and_ordering) {
    HostRig h;
    h.add(0x500, TargetClass::Door);
    Verdict a = h.run(1, Kind::Activate, R(0x500), 5);
    CHECK(a.result == Result::Executed && h.eng.count("activate") == 1);
    Verdict again = h.run(1, Kind::Activate, R(0x500), 5);                      // a resend of the same intent
    CHECK(again.result == Result::Executed && h.eng.count("activate") == 1);    // the SAME verdict, nothing ran twice
    Verdict older = h.run(1, Kind::Activate, R(0x500), 4);                      // out of order
    CHECK(older.result == Result::Duplicate && h.eng.count("activate") == 1);
    Verdict zero = h.run(1, Kind::Activate, R(0x500), 0);
    CHECK(zero.result == Result::Stale && h.eng.count("activate") == 1);
    CHECK(h.run(1, Kind::Activate, R(0x500), 6).result == Result::Executed && h.eng.count("activate") == 2);
    CHECK(h.run(1, Kind::Activate, R(0x500), 1, 2).result == Result::Executed && h.eng.count("activate") == 3);   // a new session restarts the ids
    CHECK(h.run(2, Kind::Activate, R(0x500), 5).result == Result::Executed && h.eng.count("activate") == 4);     // ids are per player
    CHECK(h.host->stats().duplicates == 3);
}

TEST(intents_host, rate_limit_per_player) {
    Config c = testConfig();
    c.rateBurst = 5;
    c.ratePerSec = 2.f;
    HostRig h(c);
    h.add(0x500, TargetClass::Door);
    int executed = 0, limited = 0;
    for (uint32_t i = 1; i <= 10; ++i) {
        Result r = h.run(1, Kind::Activate, R(0x500), i).result;
        executed += r == Result::Executed;
        limited += r == Result::RateLimited;
    }
    CHECK(executed == 5 && limited == 5);
    CHECK(h.run(2, Kind::Activate, R(0x500), 1).result == Result::Executed);   // another player has their own bucket
    h.now += 1000;                                                              // two tokens back
    CHECK(h.run(1, Kind::Activate, R(0x500), 11).result == Result::Executed);
    CHECK(h.run(1, Kind::Activate, R(0x500), 12).result == Result::Executed);
    CHECK(h.run(1, Kind::Activate, R(0x500), 13).result == Result::RateLimited);
}

TEST(intents_host, switches_and_guards) {
    HostRig h;
    h.add(0x500, TargetClass::Door);
    h.add(0x510, TargetClass::Activator);
    CHECK(h.run(1, Kind::Hit, R(0x510), 1).result == Result::Disabled);                       // hit is off by default
    CHECK(h.run(1, Kind::DialogueStart, R(0x510), 2).result == Result::Disabled);
    CHECK(h.run(1, Kind::Activate, R(0xDEAD), 3).result == Result::NotLoaded);                // unknown reference
    TargetInfo gone = mkTarget(TargetClass::Door, 0x520);
    gone.loaded = false;
    h.eng.world[R(0x520)] = gone;
    CHECK(h.run(1, Kind::Activate, R(0x520), 4).result == Result::NotLoaded);
    h.joined.erase(1);
    size_t sent = h.sent.size();
    CHECK(h.run(1, Kind::Activate, R(0x500), 5).result == Result::NotJoined && h.sent.size() == sent && h.eng.calls.empty());   // not a player: no answer
    CHECK(h.run(3, Kind::Activate, R(0x500), 1).result == Result::NotJoined);
    uint8_t junk[] = {0x8D, 1, 1, 0xFF};
    h.host->onDatagram(2, junk, sizeof junk);
    CHECK(h.host->stats().badDatagrams == 1);
    CHECK(h.host->statusJson().find("\"role\":\"host\"") != std::string::npos);
}

TEST(intents_host, pickup_claims_and_grants) {
    HostRig h;
    h.item(0x700, 0xF, 25);                                                   // 25 caps lying around
    h.eng.onCall = [&](const FakeEngine::Call& c) { if (c.what == "disable") h.hub.record(DisableRef{c.target}); };   // the engine hook that records Disable
    Verdict first = h.run(1, Kind::PickUp, R(0x700), 1);
    CHECK(first.result == Result::Executed);
    REQUIRE(h.commits.size() == 1);
    CHECK(h.commits[0].origin == 1 && h.effects<DisableRef>() == 1 && h.effects<AddItem>() == 1);
    const auto* grant = std::get_if<AddItem>(&h.commits[0].txn.effects.back());
    REQUIRE(grant != nullptr);
    CHECK(grant->item == R(0xF) && grant->count == 25 && grant->to == fnvcoop::Recipient::origin());                    // to the guest only
    CHECK(h.eng.count("event") == 1 && h.eng.calls.back().arg == ev::OnAdd && h.eng.calls.back().actor == kHostPlayerRef && h.eng.calls.back().contextActor == 0x1001);   // OnAdd blocks run as the puppet
    Verdict second = h.run(2, Kind::PickUp, R(0x700), 1);
    CHECK(second.result == Result::Taken);                                      // first claim wins
    CHECK(h.effects<AddItem>() == 1 && h.commits.size() == 1);
    CHECK(h.host->stats().claimsWon == 1 && h.host->stats().claimsLost == 1 && h.host->claimed(R(0x700)));
}

TEST(intents_host, claim_table_holds_when_the_disabled_flag_is_not_visible) {
    HostRig h;
    h.item(0x700, 0xF, 10);
    h.eng.disableMarksWorld = false;   // the engine's flag lags (or its bit is read wrong): only the claim table can say Taken
    CHECK(h.run(1, Kind::PickUp, R(0x700), 1).result == Result::Executed);
    CHECK(h.run(2, Kind::PickUp, R(0x700), 1).result == Result::Taken);
    CHECK(h.run(1, Kind::PickUp, R(0x700), 2).result == Result::Taken);        // not even the same player twice
    CHECK(h.effects<AddItem>() == 1);
}

TEST(intents_host, pickup_failure_frees_the_claim) {
    HostRig h;
    h.item(0x700, 0xF, 1);
    h.eng.disableOk = false;
    CHECK(h.run(1, Kind::PickUp, R(0x700), 1).result == Result::Failed);
    CHECK(!h.host->claimed(R(0x700)) && h.effects<AddItem>() == 0);
    h.eng.disableOk = true;
    CHECK(h.run(2, Kind::PickUp, R(0x700), 1).result == Result::Executed);      // the next try works
    h.item(0x701, 0xF, 1);
    h.eng.world[R(0x701)].disabled = true;                                      // the host's own player took it first
    CHECK(h.run(1, Kind::PickUp, R(0x701), 2).result == Result::Taken);
    TargetInfo nameless = mkTarget(TargetClass::Item, 0x702);
    h.eng.world[R(0x702)] = nameless;                                           // no base form: the grant could not be named
    CHECK(h.run(1, Kind::PickUp, R(0x702), 3).result == Result::BadTarget);
}

TEST(intents_host, claims_are_bounded_and_reset_with_the_world) {
    Config c = testConfig();
    c.claimMemory = 16;
    HostRig h(c);
    for (uint32_t i = 0; i < 40; ++i) { h.item(0x800 + i, 0xF, 1); h.run(1, Kind::PickUp, R(0x800 + i), i + 1); }
    CHECK(h.host->claimCount() == 16);
    h.host->worldReset();
    CHECK(h.host->claimCount() == 0);
}

TEST(intents_host, container_lock_is_exclusive_leased_and_released) {
    HostRig h;
    h.add(0x501, TargetClass::Container, 0, 0, 0);
    CHECK(h.run(1, Kind::ContainerOpen, R(0x501), 1).result == Result::Executed);
    Verdict busy = h.run(2, Kind::ContainerOpen, R(0x501), 1);
    CHECK(busy.result == Result::Busy && busy.holder == 1);
    CHECK(h.run(1, Kind::ContainerOpen, R(0x501), 2).result == Result::Executed);          // the holder may reopen
    CHECK(h.run(2, Kind::ContainerClose, R(0x501), 2).result == Result::Executed && h.host->locks().size() == 1);   // closing somebody else's lock does nothing
    CHECK(h.run(2, Kind::ContainerOpen, R(0x501), 3).result == Result::Busy);
    CHECK(h.run(1, Kind::ContainerClose, R(0x501), 3).result == Result::Executed && h.host->locks().empty());
    CHECK(h.run(2, Kind::ContainerOpen, R(0x501), 4).result == Result::Executed);
    h.now += h.cfg.lockLeaseMs + 1;                                                          // player 2 crashed
    CHECK(h.run(1, Kind::ContainerOpen, R(0x501), 4).result == Result::Executed);            // the lease ran out
    CHECK(h.host->stats().lockExpired == 1);
    h.host->tick();
    h.host->playerLeft(1);                                                                   // a leaver's lock goes at once
    CHECK(h.host->locks().empty());
    CHECK(h.run(2, Kind::ContainerOpen, R(0x501), 5).result == Result::Executed);
}

TEST(intents_host, container_script_half_and_dead_bodies) {
    HostRig h;
    h.add(0x501, TargetClass::Container);
    CHECK(h.run(1, Kind::ContainerOpen, R(0x501), 1).result == Result::Executed && h.eng.count("activate") == 1 && h.eng.calls[0].actor == 0x1001);
    TargetInfo body = mkTarget(TargetClass::Npc, 0x503);
    body.dead = true;
    h.eng.world[R(0x503)] = body;
    CHECK(h.run(1, Kind::Activate, R(0x503), 2).result == Result::Executed && h.host->locks().size() == 2);   // a legacy ActivateRef on a corpse: container flow
    h.add(0x504, TargetClass::Npc);
    CHECK(h.run(1, Kind::Activate, R(0x504), 3).result == Result::BadTarget);                // living NPCs talk through 0xD1
}

TEST(intents_host, shared_storage_is_redirected_per_slot) {
    HostRig h;
    h.add(0x12AB34, TargetClass::Container);
    ContainerRedirect r;
    std::string err;
    CHECK(r.load(R"({"redirects":[{"target":"falloutnv.esm:12AB34","slots":{"1":"mymod.esp:000801","2":"mymod.esp:000802"}}]})", &err));
    h.build(r);
    Verdict a = h.run(1, Kind::ContainerOpen, R(0x12AB34), 1), b = h.run(2, Kind::ContainerOpen, R(0x12AB34), 1);
    CHECK(a.result == Result::Executed && a.hasRedirect && a.redirect == FormRef("mymod.esp", 0x801));
    CHECK(b.result == Result::Executed && b.hasRedirect && b.redirect == FormRef("mymod.esp", 0x802));   // both at once: nobody is Busy
    h.joined.insert(3); h.puppets[3] = 0x1003;
    CHECK(h.run(3, Kind::ContainerOpen, R(0x12AB34), 1).result == Result::Denied);
    CHECK(h.host->stats().redirects == 2 && h.host->locks().empty());
}

TEST(intents_host, triggers_and_hits) {
    Config c = testConfig();
    c.hit = true;
    HostRig h(c);
    h.add(0x600, TargetClass::Trigger, 100);
    h.add(0x610, TargetClass::Activator, 50);
    h.add(0x611, TargetClass::Npc, 50);
    CHECK(h.run(1, Kind::TriggerEnter, R(0x600), 1).result == Result::Executed);
    CHECK(h.eng.calls.back().what == "event" && h.eng.calls.back().arg == ev::OnTriggerEnter && h.eng.calls.back().actor == kHostPlayerRef && h.eng.calls.back().contextActor == 0x1001 && h.eng.calls.back().contextSlot == 1);
    CHECK(h.run(1, Kind::TriggerEnter, R(0x600), 2).result == Result::Duplicate && h.eng.count("event") == 1);    // standing in the volume is not a new event
    CHECK(h.run(2, Kind::TriggerEnter, R(0x600), 1).result == Result::Executed);                                   // another player is
    CHECK(h.run(1, Kind::TriggerLeave, R(0x600), 3).result == Result::Executed && h.eng.calls.back().arg == ev::OnTriggerLeave);
    h.now += c.triggerCooldownMs + 1;
    CHECK(h.run(1, Kind::TriggerEnter, R(0x600), 4).result == Result::Executed);
    CHECK(h.run(1, Kind::Hit, R(0x610), 5).result == Result::Executed && h.eng.calls.back().arg == ev::OnHit);
    size_t n = h.eng.calls.size();
    CHECK(h.run(1, Kind::Hit, R(0x611), 6).result == Result::Denied && h.eng.calls.size() == n);                  // actors: combat's business
    h.eng.eventOk = false;
    CHECK(h.run(1, Kind::TriggerLeave, R(0x600), 7).result == Result::Failed);
}

TEST(intents_host, terminal_entries) {
    HostRig h;
    h.add(0x900, TargetClass::Terminal);
    h.add(0x901, TargetClass::Door);
    Intent in = HostRig::mk(Kind::TerminalUse, R(0x900), 1);
    in.path = {2, 0};
    Verdict v = h.host->handle(1, in);
    CHECK(v.result == Result::Executed && h.eng.calls.back().what == "terminal" && h.eng.calls.back().arg == 2 && h.eng.calls.back().actor == 0x1001);
    in.id = 2; in.path.clear();
    CHECK(h.host->handle(1, in).result == Result::BadTarget);
    in.id = 3; in.path = {1}; in.target = R(0x901);
    CHECK(h.host->handle(1, in).result == Result::BadTarget);                       // not a terminal
    in.id = 4; in.target = R(0x900); h.eng.terminalSupported = false;
    CHECK(h.host->handle(1, in).result == Result::Unsupported);                     // no verified engine path: nothing ran, said so
    h.eng.terminalSupported = true; h.eng.terminalOk = false; in.id = 5;
    CHECK(h.host->handle(1, in).result == Result::Failed);
    h.puppets.clear();
    h.eng.terminalOk = true; in.id = 6;
    CHECK(h.host->handle(1, in).result == Result::Executed && h.eng.calls.back().actor == kHostPlayerRef);   // running an entry opens no menu: stand-in is fine
}

TEST(intents_host, dialogue_start_checks_the_npc) {
    Config c = testConfig();
    c.dialogue = true;
    HostRig h(c);
    h.add(0xA00, TargetClass::Npc);
    TargetInfo dead = mkTarget(TargetClass::Npc, 0xA01); dead.dead = true;
    TargetInfo fighting = mkTarget(TargetClass::Npc, 0xA02); fighting.inCombat = true;
    h.eng.world[R(0xA01)] = dead; h.eng.world[R(0xA02)] = fighting;
    h.add(0xA03, TargetClass::Door);
    CHECK(h.run(1, Kind::DialogueStart, R(0xA00), 1).result == Result::Executed && h.dialogues.size() == 1 && h.dialogues[0].second == 1);
    CHECK(h.run(1, Kind::DialogueStart, R(0xA01), 2).result == Result::Denied && h.run(1, Kind::DialogueStart, R(0xA02), 3).result == Result::Denied);
    CHECK(h.run(1, Kind::DialogueStart, R(0xA03), 4).result == Result::BadTarget);
    CHECK(h.eng.calls.empty());                                                      // never run as an activation
}

TEST(intents_host, legacy_story_channel_intents) {
    HostRig h;
    h.add(0x500, TargetClass::Door);
    h.item(0x700, 0xF, 3);
    h.add(0x600, TargetClass::Trigger);
    CHECK(h.host->runLegacy(1, fnvcoop::IntentKind::ActivateRef, R(0x500)) == Result::Executed && h.eng.calls.back().actor == 0x1001);   // the puppet, not the v1 stand-in
    CHECK(h.host->runLegacy(1, fnvcoop::IntentKind::PickUp, R(0x700)) == Result::Executed && h.effects<AddItem>() == 1);
    CHECK(h.host->runLegacy(2, fnvcoop::IntentKind::ActivateRef, R(0x700)) == Result::Taken);                                            // an Activate on the same item is a claim
    CHECK(h.host->runLegacy(1, fnvcoop::IntentKind::TriggerEnter, R(0x600)) == Result::Executed);
    CHECK(h.host->runLegacy(1, fnvcoop::IntentKind::ScenePass, R(0x600)) == Result::BadTarget);
    h.joined.clear();
    CHECK(h.host->runLegacy(1, fnvcoop::IntentKind::ActivateRef, R(0x500)) == Result::NotJoined);
    CHECK(h.sent.empty());                                                            // legacy intents never get a verdict
    CHECK(h.host->stats().legacy == 6);
}

// ---- guest client -----------------------------------------------------------------------------------------------------------------------

TEST(intents_client, observers_forward_and_answer) {
    HostRig h;
    h.add(0x500, TargetClass::Door);
    h.item(0x700, 0xF, 4);
    h.add(0x800, TargetClass::Static);
    Loop loop(h);
    Loop::Guest& g = loop.addGuest(1, on());
    g.eng.world = h.eng.world;
    CHECK(g.client->onActivate(R(0x500)) == Obs::Proceed);          // door: animations stay, script half goes to the host
    CHECK(g.client->onActivate(R(0x700)) == Obs::Suppress);         // item: a claim
    CHECK(g.client->onActivate(R(0x800)) == Obs::Proceed);          // nothing to forward
    CHECK(g.client->onActivate(R(0xDEAD)) == Obs::Proceed);         // unknown to the engine: vanilla
    CHECK(g.client->stats().forwarded == 2 && g.client->pending() == 2);
    loop.pump();
    CHECK(g.client->pending() == 0 && g.client->stats().verdicts == 2);
    CHECK(h.eng.count("activate") == 1 && h.effects<AddItem>() == 1);
    loop.advance(300);
    g.ready = false;                                                // not joined / still syncing
    CHECK(g.client->onActivate(R(0x500)) == Obs::Proceed && g.client->stats().notReady == 1);   // vanilla goes on
    g.ready = true;
    CHECK(g.client->onEvent(ev::OnTriggerEnter, R(0x500)) == Obs::Suppress);
    g.client->onActivate(R(0x500));
    loop.pump();
}

TEST(intents_client, double_click_is_one_intent) {
    HostRig h;
    h.item(0x700, 0xF, 1);
    h.add(0x500, TargetClass::Door);
    Loop loop(h);
    Loop::Guest& g = loop.addGuest(1, on());
    CHECK(g.client->onActivate(R(0x700)) == Obs::Suppress);
    CHECK(g.client->onActivate(R(0x700)) == Obs::Suppress);         // still swallowed
    CHECK(g.client->onActivate(R(0x500)) == Obs::Proceed);
    CHECK(g.client->onActivate(R(0x500)) == Obs::Proceed);
    CHECK(g.client->stats().forwarded == 2 && g.client->stats().coalesced == 2);
    loop.advance(300);
    CHECK(g.client->onActivate(R(0x500)) == Obs::Proceed && g.client->stats().forwarded == 3);   // a real second click
    loop.pump();
}

TEST(intents_client, busy_container_closes_the_local_menu) {
    HostRig h;
    h.add(0x501, TargetClass::Container);
    Loop loop(h);
    Loop::Guest& a = loop.addGuest(1, on());
    Loop::Guest& b = loop.addGuest(2, on());
    a.eng.world = b.eng.world = h.eng.world;
    a.client->onActivate(R(0x501));
    loop.pump();
    CHECK(!a.eng.uiHas("closeMenus") && h.host->locks().size() == 1);
    b.client->onActivate(R(0x501));
    loop.pump();
    CHECK(b.eng.uiHas("closeMenus") && b.eng.uiHas("notify:Player 2 is using that."));      // the host's holder is slot 1 = "Player 2" in the player-facing numbering
    CHECK(b.client->stats().rejected[int(Result::Busy)] == 1);
}

TEST(intents_client, redirect_opens_the_players_own_container_without_re_forwarding) {
    HostRig h;
    h.add(0x12AB34, TargetClass::Container);
    ContainerRedirect r;
    std::string err;
    CHECK(r.load(R"({"redirects":[{"target":"falloutnv.esm:12AB34","slots":{"1":"mymod.esp:000801"}}]})", &err));
    h.build(r);
    Loop loop(h);
    Loop::Guest& g = loop.addGuest(1, on());
    g.eng.world = h.eng.world;
    g.client->onActivate(R(0x12AB34));
    loop.pump();
    CHECK(g.eng.ui.size() >= 2 && g.eng.ui[0] == "closeMenus" && g.eng.uiHas("open:mymod.esp:2049"));
    CHECK(g.client->stats().redirectsOpened == 1 && g.client->openContainer() == nullptr);   // no lock to watch for a per-player container
}

TEST(intents_client, container_menu_close_releases_the_lock) {
    HostRig h;
    h.add(0x501, TargetClass::Container);
    Loop loop(h);
    Loop::Guest& a = loop.addGuest(1, on());
    Loop::Guest& b = loop.addGuest(2, on());
    a.eng.world = b.eng.world = h.eng.world;
    a.client->onActivate(R(0x501));
    loop.pump();
    REQUIRE(a.client->openContainer() != nullptr);
    a.menuOpen = true;
    a.client->tick();
    CHECK(a.client->openContainer() != nullptr);                 // still open: still watching
    a.menuOpen = false;
    a.client->tick();                                            // the menu went away: ContainerClose
    loop.pump();
    CHECK(a.client->stats().closesSent == 1 && h.host->locks().empty() && a.client->openContainer() == nullptr);
    b.client->onActivate(R(0x501));
    loop.advance(300);
    loop.pump();
    CHECK(!b.eng.uiHas("closeMenus") && h.host->locks().size() == 1);   // now b may open it
    // a menu that never showed up (the local activation failed) is released after a grace period
    b.client->tick();
    loop.advance(3500);
    b.client->tick();
    loop.pump();
    CHECK(h.host->locks().empty());
}

TEST(intents_client, taken_item_tells_the_player) {
    HostRig h;
    h.item(0x700, 0xF, 1);
    Loop loop(h);
    Loop::Guest& a = loop.addGuest(1, on());
    Loop::Guest& b = loop.addGuest(2, on());
    a.eng.world = b.eng.world = h.eng.world;
    a.client->onActivate(R(0x700));
    b.client->onActivate(R(0x700));
    loop.pump();
    CHECK(h.effects<AddItem>() == 1);
    CHECK(!a.eng.uiHas("notify") && b.eng.uiHas("notify:Somebody else got there first."));
}

TEST(intents_client, timeouts_session_changes_and_caps) {
    HostRig h;
    h.add(0x500, TargetClass::Door);
    h.cfg.pendingTimeoutMs = 2000;
    Loop loop(h);
    Config cc = on();
    cc.pendingTimeoutMs = 2000;
    cc.coalesceMs = 0;
    Loop::Guest& g = loop.addGuest(1, cc);
    g.eng.world = h.eng.world;
    g.client->onActivate(R(0x500));                              // never pumped: the answer never comes
    CHECK(g.client->pending() == 1);
    loop.advance(1000);
    g.client->tick();
    CHECK(g.client->pending() == 1);
    loop.advance(1500);
    g.client->tick();
    CHECK(g.client->pending() == 0 && g.client->stats().timeouts == 1);   // given up (twin 323 keeps it for ever)
    g.toHost.clear();
    g.client->onActivate(R(0x500));
    CHECK(g.client->pending() == 1);
    g.session = 8;                                               // rejoined: a new session
    g.client->tick();
    CHECK(g.client->pending() == 0);                             // the old answers can never come (twin 322)
    g.toHost.clear();
    uint32_t id = g.client->submit(Kind::Activate, R(0x500));
    CHECK(id == 1);                                              // ids restart
    for (int i = 0; i < 100; ++i) g.client->submit(Kind::Activate, R(0x500));
    CHECK(g.client->pending() == 64 && g.client->stats().pendingCapped > 0);
}

TEST(intents_client, ignores_verdicts_it_did_not_ask_for) {
    HostRig h;
    h.add(0x500, TargetClass::Door);
    Loop loop(h);
    Loop::Guest& g = loop.addGuest(1, on());
    Verdict v; v.session = 7; v.id = 99; v.result = Result::Busy;
    Bytes b = encode(v);
    g.client->onDatagram(b.data(), b.size());
    CHECK(g.client->stats().unknownVerdicts == 1 && g.eng.ui.empty());
    uint8_t junk[] = {1, 2, 3};
    g.client->onDatagram(junk, sizeof junk);
    CHECK(g.client->stats().badDatagrams == 1);
    Intent in = HostRig::mk(Kind::Activate, R(5), 1);
    Bytes ib = encode(in);                                       // an Intent arriving at a client is not a verdict
    g.client->onDatagram(ib.data(), ib.size());
    CHECK(g.client->stats().badDatagrams == 2);
}

// ---- guest quest stage (the lab's "client sets stage 20 through the console, nobody hears of it") -----------------------------------------

namespace {
Intent stageIntent(const FormRef& q, int32_t stage, uint32_t id) {
    Intent in = HostRig::mk(Kind::SetStage, q, id);
    in.value = stage;
    return in;
}
}  // namespace

TEST(intents_host, guest_stage_request_runs_the_real_stage_as_the_guest) {
    HostRig h;
    h.eng.quests[R(0x1000)] = 10;
    Verdict v = h.host->handle(1, stageIntent(R(0x1000), 20, 1));
    CHECK(v.result == Result::Executed);
    CHECK(h.eng.quests[R(0x1000)] == 20 && h.eng.count("setStage") == 1);
    const FakeEngine::Call& c = h.eng.calls.back();
    CHECK(c.what == "setStage" && c.arg == 20 && c.contextActor == 0x1001 && c.contextSlot == 1 && c.depth >= 1);   // the guest's puppet is `player` for the result scripts
    REQUIRE(h.commits.size() == 1);
    CHECK(h.commits[0].origin == 1 && h.commits[0].txn.cause.kind == fnvcoop::CauseKind::Intent && h.effects<SetStage>() == 1);   // one txn, the guest is the origin
    CHECK(std::get<SetStage>(h.commits[0].txn.effects[0]).stage == 20);
    CHECK(h.eng.outsideScope == 0 && h.host->stats().stagesApplied == 1 && h.host->stats().byKind[static_cast<size_t>(Kind::SetStage)] == 1);
    h.puppets.clear();   // no puppet: the stage's scripts open no menu, the host's player stands in
    CHECK(h.host->handle(1, stageIntent(R(0x1000), 30, 2)).result == Result::Executed && h.eng.calls.back().contextActor == kHostPlayerRef);
}

TEST(intents_host, guest_stage_request_never_reruns_a_stage_the_quest_reached) {
    HostRig h;
    h.eng.quests[R(0x1000)] = 20;
    CHECK(h.host->handle(1, stageIntent(R(0x1000), 20, 1)).result == Result::NoAdvance);   // the host's own copy of the script got there first
    CHECK(h.host->handle(1, stageIntent(R(0x1000), 10, 2)).result == Result::NoAdvance);   // and a lower stage is no advance either (merge is max)
    CHECK(h.eng.count("setStage") == 0 && h.commits.empty());                               // nothing ran: no second set of rewards
    CHECK(h.host->stats().stagesNoAdvance == 2 && h.stageRefusals.size() == 2 && std::get<3>(h.stageRefusals[0]) == Result::NoAdvance);   // and the host said so
}

TEST(intents_host, guest_stage_request_obeys_the_hosts_policy_and_says_so) {
    HostRig h;
    h.eng.quests[R(0x1000)] = 10;
    h.allowStage = [](PlayerId, const FormRef&, uint32_t st) { return st != 120; };         // ExclusiveStages: the other outcome of the fork was committed first
    CHECK(h.host->handle(1, stageIntent(R(0x1000), 120, 1)).result == Result::Denied);
    CHECK(h.eng.count("setStage") == 0 && h.commits.empty() && h.eng.quests[R(0x1000)] == 10);
    CHECK(h.host->stats().stagesRefused == 1 && h.stageRefusals.size() == 1 && std::get<2>(h.stageRefusals[0]) == 120 && std::get<3>(h.stageRefusals[0]) == Result::Denied);
    CHECK(h.host->handle(1, stageIntent(R(0x1000), 110, 2)).result == Result::Executed);   // the allowed outcome goes through
    // the rules engine's barrier / tether holds the stage: the guest is told, nothing is announced as done, the host releases it itself later
    h.eng.quests[R(0x1001)] = 0;
    h.eng.stageDefer.insert(R(0x1001));
    CHECK(h.host->handle(1, stageIntent(R(0x1001), 30, 3)).result == Result::Deferred);
    CHECK(h.host->stats().stagesDeferred == 1 && h.host->stats().stagesApplied == 1);
    CHECK(h.stageRefusals.size() == 2 && std::get<3>(h.stageRefusals.back()) == Result::Deferred && std::get<2>(h.stageRefusals.back()) == 30);   // the host said so
    h.eng.stageDefer.clear();
    h.eng.stageFails = true;
    CHECK(h.host->handle(1, stageIntent(R(0x1001), 40, 4)).result == Result::Failed && h.host->stats().stagesFailed == 1);
}

TEST(intents_host, guest_stage_request_guards) {
    HostRig h;
    h.eng.quests[R(0x1000)] = 10;
    CHECK(h.host->handle(1, stageIntent(R(0x1000), 0, 1)).result == Result::BadTarget);
    CHECK(h.host->handle(1, stageIntent(R(0x1000), 256, 2)).result == Result::BadTarget);
    CHECK(h.host->handle(1, stageIntent(R(0x1000), -3, 3)).result == Result::BadTarget);
    CHECK(h.host->handle(1, stageIntent(R(0x2222), 20, 4)).result == Result::NotLoaded);    // a quest this machine does not have
    CHECK(h.eng.calls.empty() && h.host->stats().stagesBad == 3);
    h.host->handle(1, stageIntent(R(0x1000), 20, 5));
    h.host->handle(1, stageIntent(R(0x1000), 20, 5));                                         // a resend: the same verdict, nothing runs twice
    CHECK(h.eng.count("setStage") == 1);
    Config off = testConfig();
    off.stage = false;
    HostRig h2(off);
    h2.eng.quests[R(0x1000)] = 10;
    CHECK(h2.host->handle(1, stageIntent(R(0x1000), 20, 1)).result == Result::Disabled && h2.eng.calls.empty());   // [intents] stage=0
    Config parsed;
    std::string err;
    CHECK(parseConfig("[intents]\nenable=1\nstage=0\n", &parsed, &err) && !parsed.stage && Config().stage);
}

// ---- G26 (wave 1b): a guest's SetStage on a barrier stage is its pass; the rules engine performs the stage when the whole party passed ---------------

namespace {
struct NullSink : fnvcoop::IEffectSink {
    void apply(const fnvcoop::Effect&, const Context&) override {}
};
uint32_t passHash(const std::string& s) {
    uint32_t h = 2166136261u;
    for (char c : s) { h ^= static_cast<uint8_t>(c); h *= 16777619u; }
    return h;
}
bool passResolve(const std::string& edid, FormRef& out) { out = R(passHash(edid)); return true; }
}  // namespace

TEST(intents_host, a_guest_stage_on_a_barrier_is_the_guests_pass) {
    fnvcoop::InMemoryNetwork net(3);
    fnvcoop::HostSession session(*net.endpoint(0));
    NullSink sink;
    fnvcoop::EffectCapture cap;
    fnvcoop::ClientSession guest(*net.endpoint(1), 0, 1001, sink, &cap);
    const char* rules = R"({"schema":1,"quests":{
      "Chargen":{"tier":"T4","questline":"x","curated":true,"rules":[{"kind":"Barrier","params":{"step":"all-players-pass","stages":[200]},"origin":"curated"}]}}})";
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.load(rules, &err));
    fnvcoop::RulesEngine eng(session, db, passResolve);
    std::vector<std::pair<FormRef, uint32_t>> released;
    eng.applyStage = [&](const FormRef& q, uint32_t st) { released.emplace_back(q, st); };
    uint64_t now = 0;
    guest.connect(now, guest.lastSession() + 1);
    for (uint32_t t = 0; t < 400; t += 5) { now += 5; net.advance(now); session.tick(now); guest.tick(now); net.advance(now); }

    FormRef chargen, other;
    passResolve("Chargen", chargen);
    passResolve("Plain", other);
    HostRig h;
    h.eng.quests[chargen] = 10;
    h.eng.quests[other] = 10;
    h.guestStagePass = [&](PlayerId s, const FormRef& q, uint32_t st) { return stagePassOf(eng, s, q, st); };

    // a stage that is no barrier: the rules engine is asked, says NotPass, the intent runs as it always did
    CHECK(h.host->handle(1, stageIntent(other, 30, 1)).result == Result::Executed && h.eng.count("setStage") == 1);

    // the guest's chargen end reaches the barrier stage: the host does not run it, it is the guest's pass
    Verdict v = h.host->handle(1, stageIntent(chargen, 200, 2));
    CHECK(v.result == Result::Deferred);
    CHECK(h.eng.count("setStage") == 1 && h.eng.quests[chargen] == 10 && released.empty());   // nothing ran, nothing was released yet
    CHECK(h.host->stats().stagesDeferred == 1 && h.host->stats().stagesApplied == 1);
    CHECK(h.stageRefusals.size() == 1 && std::get<3>(h.stageRefusals.back()) == Result::Deferred);   // the host said so
    CHECK(eng.stats().passIntents == 1 && eng.sceneActive());

    // the host's own chargen end is the last pass: the engine performs the stage itself, exactly once
    CHECK(eng.onGuestStageIntent(0, chargen, 200) == fnvcoop::StageDecision::Deferred);
    for (uint32_t t = 0; t < 100; t += 5) { now += 5; net.advance(now); session.tick(now); guest.tick(now); net.advance(now); eng.tick(now); }
    CHECK(released.size() == 1 && released[0].first == chargen && released[0].second == 200 && !eng.sceneActive());
    CHECK(h.eng.count("setStage") == 1);   // the host intent path never ran the stage itself

    // a late resend of the guest's request: the quest is behind the stage now, no second run
    h.eng.quests[chargen] = 200;
    CHECK(h.host->handle(1, stageIntent(chargen, 200, 3)).result == Result::NoAdvance && h.eng.count("setStage") == 1);

    // no hook wired: every stage is an ordinary stage (the old behaviour)
    HostRig plain;
    plain.eng.quests[chargen] = 10;
    CHECK(plain.host->handle(1, stageIntent(chargen, 200, 1)).result == Result::Executed && plain.eng.count("setStage") == 1);
}

TEST(intents_client, blocked_stage_becomes_one_request_per_burst) {
    HostRig h;
    h.eng.quests[R(0x1000)] = 10;
    Loop loop(h);
    Loop::Guest& g = loop.addGuest(1, on());
    CHECK(g.client->onStageBlocked(R(0x1000), 20));                      // the guest's console / script setstage was dropped by the suppression ...
    CHECK(g.client->stats().byKind[static_cast<size_t>(Kind::SetStage)] == 1 && g.client->pending() == 1);   // ... and went to the host
    CHECK(!g.client->onStageBlocked(R(0x1000), 20) && g.client->stats().coalesced == 1);   // a script that retries every frame is one intent
    CHECK(g.client->onStageBlocked(R(0x1000), 30));                      // a different stage of the same quest is not a retry
    loop.pump();
    CHECK(g.client->pending() == 0 && g.client->stats().verdicts == 2 && h.eng.quests[R(0x1000)] == 30);
    loop.advance(5000);
    CHECK(g.client->onStageBlocked(R(0x1000), 40));                      // long after: a new request
    g.ready = false;
    CHECK(!g.client->onStageBlocked(R(0x1000), 50) && g.client->stats().notReady == 1);   // not joined: nothing to send, the drop stays a drop
    g.ready = true;
    CHECK(!g.client->onStageBlocked(R(0x1000), 0) && !g.client->onStageBlocked(R(0x1000), 300) && g.client->stats().stageBad == 2);
    Config off = on();
    off.stage = false;
    Loop::Guest& g2 = loop.addGuest(2, off);
    CHECK(!g2.client->onStageBlocked(R(0x1000), 60) && g2.client->stats().stageOff == 1 && g2.client->stats().forwarded == 0);
    Config disabled = on();
    disabled.enable = false;
    Loop::Guest& g3 = loop.addGuest(2, disabled);
    CHECK(!g3.client->onStageBlocked(R(0x1000), 60) && g3.client->stats().forwarded == 0);
}

TEST(intents_e2e, guest_console_stage_reaches_the_host_and_both_agree) {
    // the lab's quest room: the host is at stage 10, the CLIENT sets stage 20 through the console. Before: the suppression dropped it, host and client both
    // stayed at 10. Now: the dropped call is forwarded, the host runs it for the guest, the txn comes back with the guest as origin.
    HostRig h;
    h.eng.quests[R(0x1000)] = 10;
    Loop loop(h);
    Loop::Guest& a = loop.addGuest(1, on());
    Loop::Guest& b = loop.addGuest(2, on());
    a.eng.quests[R(0x1000)] = b.eng.quests[R(0x1000)] = 10;
    CHECK(a.client->onStageBlocked(R(0x1000), 20));
    loop.pump();
    CHECK(h.eng.quests[R(0x1000)] == 20);
    REQUIRE(h.commits.size() == 1);
    CHECK(h.commits[0].origin == 1 && h.effects<SetStage>() == 1);
    CHECK(a.client->stats().verdicts == 1 && a.client->stats().rejected[static_cast<size_t>(Result::Executed)] == 0);
    CHECK(a.eng.quests[R(0x1000)] == 10 && b.eng.quests[R(0x1000)] == 10);                   // a guest never applies an intent itself; the txn does that
    // the host's own copy of the same script ran too: the guest's request for the same stage must not run the stage a second time
    CHECK(b.client->onStageBlocked(R(0x1000), 20));
    loop.pump();
    CHECK(h.commits.size() == 1 && h.eng.count("setStage") == 1 && b.client->stats().rejected[static_cast<size_t>(Result::NoAdvance)] == 1);
    CHECK(h.eng.outsideScope == 0);
}

// ---- end to end: the loopback host and guests ------------------------------------------------------------------------------------------

TEST(intents_e2e, pickup_race_between_two_guests_gives_one_winner) {
    HostRig h;
    h.item(0x700, 0xF, 10);
    h.eng.onCall = [&](const FakeEngine::Call& c) { if (c.what == "disable") h.hub.record(DisableRef{c.target}); };
    Loop loop(h);
    Loop::Guest& a = loop.addGuest(1, on());
    Loop::Guest& b = loop.addGuest(2, on());
    a.eng.world = b.eng.world = h.eng.world;
    CHECK(a.client->onActivate(R(0x700)) == Obs::Suppress && b.client->onActivate(R(0x700)) == Obs::Suppress);
    loop.pump();
    CHECK(h.effects<AddItem>() == 1 && h.commits.size() == 1 && h.commits[0].origin == 1);
    CHECK(b.eng.uiHas("notify:Somebody"));
    CHECK(h.eng.outsideScope == 0);
}

// ---- save gate --------------------------------------------------------------------------------------------------------------------------

TEST(intents_savegate, blocks_while_suppressed_and_says_so_rarely) {
    bool flag = false;
    uint64_t now = 1000;
    std::vector<std::string> said;
    SaveGate g(SaveGate::Deps{[&] { return flag; }, [&](const std::string& s) { said.push_back(s); }, [&] { return now; }});
    CHECK(g.allow(true) && said.empty());
    flag = true;
    CHECK(!g.allow(true) && said.size() == 1);
    CHECK(!g.allow(true) && said.size() == 1);                // not one line per refused call
    now += 6000;
    CHECK(!g.allow(true) && said.size() == 2);
    CHECK(!g.allow(false));                                   // the engine said no anyway
    flag = false;
    CHECK(g.allow(true) && !g.allow(false));
    CHECK(g.stats().blocked == 3 && g.stats().asked == 7);
}
