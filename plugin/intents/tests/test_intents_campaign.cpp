// Campaign gaps G03 / G04 / G10 / G35 (docs/CAMPAIGN-GAPS.md), game-free: guest OnTrigger and OnStartCombat forwarding, a note picked up by a guest, the per-player gear
// stash on the shared storage containers, and the data file that names them.
// Runtime twins 401..415 (tools/mut/fnvcoop_mut.h): the same binary is run once per twin with FNVCOOP_MUT=<n> and every one must fail a check.
#include <fstream>
#include <iterator>

#include "fnvcoop/effect.h"
#include "host_rig.h"
#include "intents/container_redirect.h"
#include "intents/intent_service.h"
#include "common/minijson.h"
#include "intents/storage_book.h"
#include "minitest.h"

using namespace nvcoop::intents;
using namespace nvcoop::intents::testing;
using fnvcoop::AddItem;
using fnvcoop::AddNote;
using fnvcoop::DisableRef;

namespace {

Config on() { return testConfig(); }

const FormRef kStore = R(0x139B6B);    // the Fort's player storage
const FormRef kGear = R(0x136A26);     // a second shared container

StorageBook book() {
    StorageBook b;
    std::string err;
    const bool ok = b.load(R"({"stashes":[{"name":"NVCCPlayerStorageREF","target":"falloutnv.esm:139B6B"},{"name":"FortPlayerEquipmentContainerRef","target":"falloutnv.esm:136A26"}]})", &err);
    if (!ok) std::printf("  book load: %s\n", err.c_str());
    return b;
}

std::string slurp(const char* path) {
    std::ifstream f(path, std::ios::binary);
    return std::string((std::istreambuf_iterator<char>(f)), std::istreambuf_iterator<char>());
}

}  // namespace

// ---- G04: OnTrigger (per frame) ---------------------------------------------------------------------------------------------------------

TEST(intents_policy, per_frame_on_trigger_and_on_start_combat_are_forwarded) {
    Config c = on();
    TargetInfo trig = mkTarget(TargetClass::Trigger, 1), npc = mkTarget(TargetClass::Npc, 2), door = mkTarget(TargetClass::Door, 3);
    GuestDecision d = decideEvent(c, ev::OnTrigger, trig);
    CHECK(d.forward && d.kind == Kind::TriggerStay && d.suppressLocal);
    d = decideEvent(c, ev::OnStartCombat, npc);
    CHECK(d.forward && d.kind == Kind::StartCombat && !d.suppressLocal);
    d = decideEvent(c, ev::OnStartCombat, mkTarget(TargetClass::Creature, 4));
    CHECK(d.forward && d.kind == Kind::StartCombat);
    CHECK(!decideEvent(c, ev::OnStartCombat, door).forward);                  // only actors start combat
    Config noStay = c;
    noStay.triggerStay = false;
    CHECK(!decideEvent(noStay, ev::OnTrigger, trig).forward && decideEvent(noStay, ev::OnTriggerEnter, trig).forward);
    Config noTrig = c;
    noTrig.trigger = false;
    CHECK(!decideEvent(noTrig, ev::OnTrigger, trig).forward);
    Config noCombat = c;
    noCombat.combat = false;
    CHECK(!decideEvent(noCombat, ev::OnStartCombat, npc).forward);
    CHECK(!kindEnabled(noStay, Kind::TriggerStay) && !kindEnabled(noCombat, Kind::StartCombat) && kindEnabled(c, Kind::TriggerStay) && kindEnabled(c, Kind::StartCombat));
    Config parsed;
    std::string err;
    CHECK(parseConfig("[intents]\ntrigger_stay=0\ncombat=0\nstorage=0\nstay_keepalive_ms=500\nstay_lease_ms=2000\nstay_raise_ms=50\nstay_max=4\nreach_combat=900\n", &parsed, &err));
    CHECK(!parsed.triggerStay && !parsed.combat && !parsed.storage && parsed.stayKeepaliveMs == 500 && parsed.stayLeaseMs == 2000 && parsed.stayRaiseMs == 50 &&
          parsed.stayMax == 4 && parsed.reachCombat == 900.f);
    Config d0;
    CHECK(d0.triggerStay && d0.combat && d0.storage && d0.redirectFile == "nvcoop_intents.json");                 // the data file is read by default
    Config none;
    CHECK(parseConfig("[intents]\nredirect_file=none\n", &none, &err) && none.redirectFile.empty());
}

TEST(intents_wire, the_new_kinds_survive_the_wire) {
    for (Kind k : {Kind::TriggerStay, Kind::StartCombat}) {
        Intent in;
        in.session = 3; in.id = 9; in.kind = k; in.target = R(0x600);
        Intent out;
        CHECK(decode(encode(in), &out, nullptr) == WireType::Intent && out.kind == k);
    }
    CHECK(std::string(kindName(Kind::TriggerStay)) == "TriggerStay" && std::string(kindName(Kind::StartCombat)) == "StartCombat");
}

TEST(intents_host, on_trigger_is_raised_every_host_frame_for_a_present_puppet_until_it_leaves) {
    HostRig h;
    h.add(0x600, TargetClass::Trigger, 100);
    CHECK(h.run(1, Kind::TriggerEnter, R(0x600), 1).result == Result::Executed);
    CHECK(h.host->presenceCount() == 1);
    h.eng.calls.clear();
    for (int frame = 0; frame < 5; ++frame) { h.now += 16; h.host->tick(); }
    size_t raises = 0;
    for (const auto& c : h.eng.calls)
        if (c.what == "event" && c.arg == ev::OnTrigger) {
            ++raises;
            CHECK(c.actor == kHostPlayerRef && c.contextActor == 0x1001 && c.contextSlot == 1 && c.depth == 1);   // the event is `player`'s, the puppet is the acting player inside the block
        }
    CHECK(raises == 5);                                                                                      // once per host frame, like the engine
    CHECK(h.run(1, Kind::TriggerLeave, R(0x600), 2).result == Result::Executed && h.host->presenceCount() == 0);
    h.eng.calls.clear();
    h.now += 16; h.host->tick();
    CHECK(h.eng.count("event") == 0 && h.eng.outsideScope == 0);                                            // nobody inside: nothing raised
}

// Follow-ups 1, item 1. The Dead Money return trigger is `OnTrigger Player`: MarkBaseExtraListScriptEvent keeps the mask under the entry of the event's object (and under "any
// object"), a block that names `Player` matches the entry of the player reference (0x14) only, and a mark only sets a flag: the block runs when the engine next runs the trigger's
// script, with no acting scope open. The fake engine models both: runScript consumes the marks and tells the block (Call::arg) which masks `Player` blocks would match.
static void blockEffect(nvcoop::intents::testing::HostRig& h, uint32_t trigger) {
    h.eng.onScript = [&h, trigger](const nvcoop::intents::testing::FakeEngine::Call& c) {
        if (c.arg & (ev::OnTrigger | ev::OnTriggerEnter)) h.hub.record(fnvcoop::EnableRef{R(trigger + 0x100)});   // the block ran for `Player`: it does its work
    };
}

TEST(intents_host, a_player_filtered_trigger_block_runs_now_as_the_guest_and_its_work_goes_to_the_guest) {
    HostRig h;
    h.add(0x600, TargetClass::Trigger, 100);
    blockEffect(h, 0x600);
    CHECK(h.run(1, Kind::TriggerEnter, R(0x600), 1).result == Result::Executed);
    REQUIRE(h.eng.scripts.size() == 1);
    const auto& s = h.eng.scripts[0];
    CHECK(s.target == R(0x600) && (s.arg & ev::OnTriggerEnter) != 0);   // the `Player` block matched the mark (the event's object is 0x14)
    CHECK(s.contextActor == 0x1001 && s.contextSlot == 1 && s.depth == 1);   // ... while the guest's puppet was the acting player
    REQUIRE(h.commits.size() == 1);
    CHECK(h.commits[0].origin == 1 && h.effects<fnvcoop::EnableRef>() == 1);   // the work was committed with the guest as origin
    CHECK(h.eng.marked.empty() && h.host->stats().eventScriptRuns == 1);        // nothing is left for the engine's next update to run as the host
    // the per-frame OnTrigger of the Dead Money return trigger: the same, once per host frame, for as long as the guest is inside
    h.commits.clear();
    for (int frame = 0; frame < 3; ++frame) { h.now += 16; h.host->tick(); }
    CHECK(h.eng.scripts.size() == 4 && h.host->stats().eventScriptRuns == 4);
    for (size_t i = 1; i < h.eng.scripts.size(); ++i) CHECK((h.eng.scripts[i].arg & ev::OnTrigger) != 0 && h.eng.scripts[i].contextSlot == 1 && h.eng.scripts[i].depth == 1);
    CHECK(h.commits.size() == 3 && h.commits.back().origin == 1);
    CHECK(h.eng.outsideScope == 0 && h.eng.hostScriptRuns == 0);
}

TEST(intents_host, the_host_players_own_pending_mark_is_run_first_and_not_eaten_by_the_guests_run) {
    HostRig h;
    h.add(0x600, TargetClass::Trigger, 100);
    blockEffect(h, 0x600);
    h.eng.marked.insert(std::make_tuple(kHostPlayerRef, R(0x600), ev::OnTrigger));   // the host stands in the same volume; the engine marked it this frame
    CHECK(h.run(1, Kind::TriggerEnter, R(0x600), 1).result == Result::Executed);
    CHECK(h.eng.hostScriptRuns == 1 && h.host->stats().eventsFlushed == 1);             // the host's block ran first, outside any scope (as the host)
    REQUIRE(h.eng.scripts.size() == 1);
    CHECK((h.eng.scripts[0].arg & ev::OnTrigger) == 0 && (h.eng.scripts[0].arg & ev::OnTriggerEnter) != 0);   // the guest's run carries the guest's own mark only
    REQUIRE(h.commits.size() == 1);
    CHECK(h.commits[0].origin == 1);                                                    // the host's run committed nothing under the guest's origin
}

TEST(intents_host, a_trigger_with_no_script_instance_is_counted_and_still_marked) {
    HostRig h;
    h.add(0x600, TargetClass::Trigger, 100);
    h.eng.noScript.insert(R(0x600));
    CHECK(h.run(1, Kind::TriggerEnter, R(0x600), 1).result == Result::Executed);
    CHECK(h.host->stats().eventScriptMissed == 1 && h.host->stats().eventScriptRuns == 0 && h.eng.count("event") == 1);
}

TEST(intents_host, a_keepalive_extends_the_presence_and_a_lost_leave_expires_it) {
    Config c = on();
    c.stayLeaseMs = 3000;
    HostRig h(c);
    h.add(0x600, TargetClass::Trigger, 100);
    CHECK(h.run(1, Kind::TriggerStay, R(0x600), 1).result == Result::Executed && h.host->presenceCount() == 1);   // joined mid-volume: no Enter was seen
    h.now += 2000; h.host->tick();
    CHECK(h.run(1, Kind::TriggerStay, R(0x600), 2).result == Result::Executed);
    h.now += 2000; h.host->tick();
    CHECK(h.host->presenceCount() == 1);                                                                      // refreshed at +2 s, lease 3 s
    h.now += 2000; h.host->tick();
    CHECK(h.host->presenceCount() == 0 && h.host->stats().stayExpired == 1);                                  // the Leave never came
    h.eng.calls.clear();
    h.now += 16; h.host->tick();
    CHECK(h.eng.count("event") == 0);
}

TEST(intents_host, presence_is_bounded_ended_by_the_leaver_and_by_the_world) {
    Config c = on();
    c.stayMax = 2;
    HostRig h(c);
    for (uint32_t i = 0; i < 3; ++i) h.add(0x600 + i, TargetClass::Trigger, 100);
    CHECK(h.run(1, Kind::TriggerStay, R(0x600), 1).result == Result::Executed);
    CHECK(h.run(1, Kind::TriggerStay, R(0x601), 2).result == Result::Executed);
    CHECK(h.run(1, Kind::TriggerStay, R(0x602), 3).result == Result::RateLimited && h.host->presenceCount() == 2);   // a flood cannot make the host raise thousands a frame
    CHECK(h.run(1, Kind::TriggerStay, R(0x600), 4).result == Result::Executed);                                      // an existing presence is only refreshed
    h.host->playerLeft(1);
    CHECK(h.host->presenceCount() == 0);
    CHECK(h.run(2, Kind::TriggerStay, R(0x600), 1).result == Result::Executed && h.host->presenceCount() == 1);
    h.host->worldReset();
    CHECK(h.host->presenceCount() == 0);
    CHECK(h.run(2, Kind::TriggerEnter, R(0x602), 2).result == Result::Executed && h.host->presenceCount() == 1);
    h.eng.world[R(0x602)].disabled = true;                                                                           // the volume was disabled by a script
    h.eng.calls.clear();
    h.now += 16; h.host->tick();
    CHECK(h.host->presenceCount() == 0 && h.eng.count("event") == 0);
}

TEST(intents_host, the_raise_interval_is_honoured_and_far_stays_are_refused) {
    Config c = on();
    c.stayRaiseMs = 100;
    HostRig h(c);
    h.add(0x600, TargetClass::Trigger, 100);
    h.eng.positions[0x1001] = {100000, 0, 0};
    CHECK(h.run(1, Kind::TriggerStay, R(0x600), 1).result == Result::OutOfRange && h.host->presenceCount() == 0);
    h.eng.positions[0x1001] = {120, 0, 0};
    CHECK(h.run(1, Kind::TriggerStay, R(0x600), 2).result == Result::Executed);
    h.eng.calls.clear();
    for (int i = 0; i < 20; ++i) { h.now += 16; h.run(1, Kind::TriggerStay, R(0x600), 3 + i); h.host->tick(); }   // 320 ms of frames
    size_t raises = h.eng.count("event");
    CHECK(raises >= 3 && raises <= 4);                                                                           // at most every 100 ms
    Config off = on();
    off.triggerStay = false;
    HostRig o(off);
    o.add(0x600, TargetClass::Trigger, 100);
    CHECK(o.run(1, Kind::TriggerStay, R(0x600), 1).result == Result::Disabled);
    CHECK(o.run(1, Kind::TriggerEnter, R(0x600), 2).result == Result::Executed && o.host->presenceCount() == 0);   // switched off: the Enter is a one-shot again
}

TEST(intents_client, one_keepalive_per_interval_not_one_per_engine_frame) {
    HostRig h;
    h.add(0x600, TargetClass::Trigger, 100);
    Loop loop(h);
    Config c = on();
    c.coalesceMs = 250;
    c.stayKeepaliveMs = 1000;
    Loop::Guest& g = loop.addGuest(1, c);
    size_t suppressed = 0;
    for (int frame = 0; frame < 120; ++frame) {                     // two seconds at 60 fps
        if (g.client->onEvent(ev::OnTrigger, R(0x600)) == Obs::Suppress) ++suppressed;
        loop.advance(16);
        loop.pump();
    }
    CHECK(suppressed == 120);                                       // the local per-frame event is never raised on the guest
    CHECK(g.client->stats().byKind[static_cast<size_t>(Kind::TriggerStay)] >= 2 && g.client->stats().byKind[static_cast<size_t>(Kind::TriggerStay)] <= 3);
    CHECK(h.host->presenceCount() == 1);
}

TEST(intents_e2e, a_guest_alone_in_the_legate_tent_runs_both_trigger_blocks_as_the_guest) {
    // VHDLegionEnterLegatesTentScript: `begin OnTriggerEnter player` (the guard's line) and `begin OnTrigger player` (a 1.5 s timer on GetSecondsPassed, then
    // SetStage VHDLegionBattle 10). The second one is a per-frame block: it only ever ran for the host's player.
    HostRig h;
    h.add(0x14DA58, TargetClass::Trigger, 100);
    Loop loop(h);
    Config c = on();
    Loop::Guest& g = loop.addGuest(1, c);
    CHECK(g.client->onEvent(ev::OnTriggerEnter, R(0x14DA58)) == Obs::Suppress);
    loop.pump();
    size_t frames = 0;
    for (int frame = 0; frame < 150; ++frame) {                       // 2.4 s at 60 fps: the guest's engine raises OnTrigger every frame, the host ticks every frame
        CHECK(g.client->onEvent(ev::OnTrigger, R(0x14DA58)) == Obs::Suppress);
        loop.advance(16);
        loop.pump();
        h.host->tick();
        ++frames;
    }
    size_t enters = 0, stays = 0;
    for (const auto& call : h.eng.calls) {
        if (call.what != "event") continue;
        CHECK(call.actor == kHostPlayerRef && call.contextActor == 0x1001 && call.contextSlot == 1 && call.depth == 1);   // the event is `player`'s, the guest's puppet is the acting player in both blocks
        if (call.arg == ev::OnTriggerEnter) ++enters;
        if (call.arg == ev::OnTrigger) ++stays;
    }
    CHECK(enters == 1 && stays == frames);                            // the Enter block ran once, the per-frame block ran every host frame
    CHECK(g.client->stats().byKind[static_cast<size_t>(Kind::TriggerStay)] <= 3);   // a few keep-alives, not 150 datagrams
    CHECK(g.client->onEvent(ev::OnTriggerLeave, R(0x14DA58)) == Obs::Suppress);
    loop.pump();
    h.eng.calls.clear();
    for (int frame = 0; frame < 10; ++frame) { loop.advance(16); h.host->tick(); }
    CHECK(h.eng.count("event") == 0 && h.host->presenceCount() == 0 && h.eng.outsideScope == 0);
}

TEST(intents_host, start_combat_runs_the_actors_block_for_the_puppet_once) {
    HostRig h;
    h.add(0x700, TargetClass::Npc, 100);
    h.add(0x701, TargetClass::Door, 100);
    CHECK(h.run(1, Kind::StartCombat, R(0x700), 1).result == Result::Executed);
    REQUIRE(!h.eng.calls.empty());
    const auto& call = h.eng.calls.back();
    CHECK(call.what == "event" && call.arg == ev::OnStartCombat && call.target == R(0x700) && call.actor == kHostPlayerRef && call.contextActor == 0x1001 && call.contextSlot == 1 && call.depth == 1);   // the event is for `player`, the acting context is the puppet
    CHECK(h.run(1, Kind::StartCombat, R(0x700), 2).result == Result::Duplicate && h.eng.count("event") == 1);       // the same actor re-engaging inside the cooldown
    CHECK(h.run(2, Kind::StartCombat, R(0x700), 1).result == Result::Executed);                                     // another player's combat is its own event
    CHECK(h.run(1, Kind::StartCombat, R(0x701), 3).result == Result::BadTarget);
    h.eng.world[R(0x700)].dead = true;
    h.now += 5000;
    CHECK(h.run(1, Kind::StartCombat, R(0x700), 4).result == Result::Denied);                                       // no combat for a corpse
    CHECK(h.eng.outsideScope == 0);
}

// ---- G10: a note ------------------------------------------------------------------------------------------------------------------------

TEST(intents_host, a_picked_up_note_goes_to_the_journal_not_to_an_inventory) {
    HostRig h;
    h.item(0x710, 0x4A0, 1, 100);
    h.eng.world[R(0x710)].note = true;
    h.item(0x711, 0x4A1, 4, 100);   // a plain item for contrast
    h.eng.onCall = [&](const FakeEngine::Call& c) { if (c.what == "disable") h.hub.record(DisableRef{c.target}); };
    Verdict v = h.run(1, Kind::PickUp, R(0x710), 1);
    CHECK(v.result == Result::Executed);
    CHECK(h.eng.journal.count(R(0x4A0)) == 1);                                                  // the host's journal has it: GetHasNote on the host is true
    CHECK(h.effects<AddNote>() == 1 && h.effects<AddItem>() == 0 && h.commits.size() == 1 && h.commits[0].origin == 1);   // shared journal entry from the guest's txn, no item
    CHECK(h.eng.count("event") == 1 && h.eng.calls.back().arg == ev::OnAdd);                    // VMQ01NoteObjectSCRIPT's OnAdd block still runs, as the puppet
    CHECK(h.eng.calls.back().contextSlot == 1 && h.host->stats().noteGrants == 1);
    CHECK(h.run(2, Kind::PickUp, R(0x710), 1).result == Result::Taken);                         // one note, one winner
    CHECK(h.run(2, Kind::PickUp, R(0x711), 2).result == Result::Executed);
    CHECK(h.effects<AddItem>() == 1 && h.effects<AddNote>() == 1 && h.eng.journal.size() == 1);
    CHECK(h.eng.outsideScope == 0);
}

TEST(intents_host, a_note_the_engine_refuses_frees_the_claim) {
    HostRig h;
    h.item(0x710, 0x4A0, 1, 100);
    h.eng.world[R(0x710)].note = true;
    h.eng.noteOk = false;
    CHECK(h.run(1, Kind::PickUp, R(0x710), 1).result == Result::Failed && !h.host->claimed(R(0x710)) && h.effects<AddNote>() == 0);
    h.eng.noteOk = true;
    CHECK(h.run(2, Kind::PickUp, R(0x710), 1).result == Result::Executed && h.effects<AddNote>() == 1);
}

TEST(intents_e2e, a_guest_loots_the_note_and_the_host_journal_and_the_other_guest_agree) {
    HostRig h;
    h.item(0x710, 0x4A0, 1, 100);
    h.eng.world[R(0x710)].note = true;
    Loop loop(h);
    Loop::Guest& a = loop.addGuest(1, on());
    a.eng.world = h.eng.world;
    CHECK(a.client->onActivate(R(0x710)) == Obs::Suppress);                                     // the local pickup is suppressed: the guest gets it from the txn
    loop.pump();
    CHECK(h.eng.journal.count(R(0x4A0)) == 1 && h.effects<AddNote>() == 1 && a.client->stats().rejected[static_cast<size_t>(Result::Taken)] == 0);
}

// ---- G35: the gear stash ----------------------------------------------------------------------------------------------------------------

TEST(intents_storage, book_loads_files_persists_and_rejects_nonsense) {
    StorageBook b = book();
    CHECK(b.entries().size() == 2 && b.isStash(kStore) && b.isStash(kGear) && !b.isStash(R(1)));
    std::string err;
    CHECK(!b.load(R"({"stashes":[{"target":"falloutnv.esm:1"},{"target":"falloutnv.esm:1"}]})", &err) && b.entries().size() == 2);   // twice: refused, the list stays
    CHECK(!b.load(R"({"stashes":[{"target":"nonsense"}]})", &err));
    CHECK(!b.load("not json", &err));
    b.put(kStore, 1, {{R(0xA), 2}, {R(0xA), 3}, {R(0xB), 0}, {R(0xC), -4}});
    REQUIRE(b.peek(kStore, 1) != nullptr);
    CHECK(*b.peek(kStore, 1) == std::vector<StoredItem>({{R(0xA), 5}}));                      // merged, zero and negative lines dropped
    CHECK(b.peek(kStore, 2) == nullptr && b.stashes() == 1 && b.totalItems(1) == 5 && b.dirty());
    b.put(kGear, 2, {{R(0xD), 1}});
    const std::string state = b.stateJson();
    StorageBook c = book();
    CHECK(c.loadState(state, &err) && !c.dirty() && c.stashes() == 2 && *c.peek(kGear, 2) == std::vector<StoredItem>({{R(0xD), 1}}) && c.totalItems(1) == 5);
    CHECK(!c.loadState("{\"version\":2,\"stash\":[]}", &err) && c.stashes() == 2);            // a bad file keeps what is in memory
    CHECK(!c.loadState("{\"version\":1,\"stash\":[{\"container\":\"falloutnv.esm:1\",\"slot\":9,\"items\":[]}]}", &err));
    CHECK(c.take(kStore, 1).size() == 1 && c.peek(kStore, 1) == nullptr && c.take(kStore, 1).empty());
    c.put(kGear, 2, {});                                                                      // an empty list forgets the slot
    CHECK(c.stashes() == 0);
    b.forgetSlot(1);
    CHECK(b.peek(kStore, 1) == nullptr && b.peek(kGear, 2) != nullptr);
}

TEST(intents_storage, the_shipped_data_file_names_real_containers) {
    const std::string text = slurp("plugin/intents/data/nvcoop_intents.json");
    REQUIRE(!text.empty());
    StorageBook b;
    ContainerRedirect r;
    std::string err;
    CHECK(b.load(text, &err));
    CHECK(r.load(text, &err));                                                                 // the same file is also the redirect table (empty: no container to redirect to)
    CHECK(b.entries().size() >= 5 && r.size() == 0);
    CHECK(b.isStash(FormRef("falloutnv.esm", 0x139B6B)));                                      // NVCCPlayerStorageREF
    CHECK(b.isStash(FormRef("falloutnv.esm", 0x136A26)));                                      // FortPlayerEquipmentContainerRef
    CHECK(b.isStash(FormRef("deadmoney.esm", 0xEFCC)));                                        // NVDLC01PlayerEquipmentContainerRef
    const std::string edids = slurp("out/L10/data/nvcoop_edids.json");
    if (edids.empty()) { std::printf("    (no EditorID map: names not compared with the game data)\n"); return; }
    nvcoop::json::Value root;
    std::string perr;
    REQUIRE(nvcoop::json::parse(edids, root, perr));
    for (const auto& e : b.entries()) {
        FormRef want;
        CHECK(!e.name.empty() && parseFormRef(root.get(e.name).asString(""), &want) && want == e.container);
    }
}

TEST(intents_host, each_guest_sees_only_their_own_gear_in_the_shared_container) {
    HostRig h;
    StorageBook b = book();
    h.build(ContainerRedirect(), &b);
    h.add(0x500, TargetClass::Door, 100);       // the barge gate: strips into the storage
    h.add(0x501, TargetClass::Door, 100);       // the return: hands back what the storage holds
    h.eng.containers[kStore] = {{R(0xAAA), 1}};     // the host's own gear sits in the container
    h.eng.containers[kGear] = {};
    std::vector<StoredItem> seenByReturn;
    std::map<PlayerId, int32_t> mine = {{1, 2}, {2, 3}};
    h.eng.onCall = [&](const FakeEngine::Call& c) {
        if (c.what == "activate" && c.target == R(0x500)) {   // strip: the acting player's gear goes into the storage
            auto& cont = h.eng.containers[kStore];
            cont.push_back({R(0xB00 + c.contextSlot), mine[c.contextSlot]});
        }
        if (c.what == "activate" && c.target == R(0x501)) seenByReturn = h.eng.containers[kStore];   // restore: the script reads the whole container
    };
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::Executed);
    CHECK(h.eng.containers[kStore] == std::vector<StoredItem>({{R(0xAAA), 1}}));            // the host's gear is back, the guest's is filed
    CHECK(h.run(2, Kind::Activate, R(0x500), 1).result == Result::Executed);
    CHECK(h.eng.containers[kStore] == std::vector<StoredItem>({{R(0xAAA), 1}}));
    REQUIRE(b.peek(kStore, 1) != nullptr && b.peek(kStore, 2) != nullptr);
    CHECK(*b.peek(kStore, 1) == std::vector<StoredItem>({{R(0xB01), 2}}) && *b.peek(kStore, 2) == std::vector<StoredItem>({{R(0xB02), 3}}));
    CHECK(h.run(1, Kind::Activate, R(0x501), 2).result == Result::Executed);
    CHECK(seenByReturn == std::vector<StoredItem>({{R(0xB01), 2}}));                        // guest 1 gets guest 1's gear: not guest 2's, not the host's
    CHECK(h.run(2, Kind::Activate, R(0x501), 2).result == Result::Executed);
    CHECK(seenByReturn == std::vector<StoredItem>({{R(0xB02), 3}}));
    CHECK(h.eng.containers[kStore] == std::vector<StoredItem>({{R(0xAAA), 1}}) && h.eng.containers[kGear].empty());   // the host's container is exactly as it was
    CHECK(h.eng.replaceInScope == 0 && h.eng.outsideScope == 0);                            // the swap is never inside the capture scope: nothing is recorded
    CHECK(h.commits.empty() || h.effects<fnvcoop::RefAddItem>() == 0);                      // and the ledger moves are not world changes
}

TEST(intents_host, the_hosts_own_player_and_untouched_containers_cost_nothing) {
    HostRig h;
    StorageBook b = book();
    h.build(ContainerRedirect(), &b);
    h.eng.containers[kStore] = {{R(0xAAA), 1}};
    h.eng.containers[kGear] = {};
    IntentHost::StashHold hold = h.host->stashEnter(kHostPlayer);
    CHECK(!hold.active && h.eng.readCalls == 0 && h.eng.replaceCalls == 0);                 // the host's gear is the container's real content
    h.host->stashExit(hold);
    h.add(0x500, TargetClass::Door, 100);
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::Executed);
    CHECK(h.eng.containers[kStore] == std::vector<StoredItem>({{R(0xAAA), 1}}));
    CHECK(h.host->stats().stashSwaps == 1 && h.host->stats().stashKept == 1);                // the full one was swapped (parked), the empty one only looked at
    CHECK(h.eng.replaceCalls == 2);                                                         // park + restore for the full one, nothing for the empty one
    Config noStorage = on();
    noStorage.storage = false;
    HostRig o(noStorage);
    StorageBook b2 = book();
    o.build(ContainerRedirect(), &b2);
    o.eng.containers[kStore] = {{R(0xAAA), 1}};
    o.add(0x500, TargetClass::Door, 100);
    CHECK(o.run(1, Kind::Activate, R(0x500), 1).result == Result::Executed && o.eng.readCalls == 0);
}

TEST(intents_host, a_stash_survives_a_failed_swap_and_a_failed_read) {
    HostRig h;
    StorageBook b = book();
    b.put(kStore, 1, {{R(0xB01), 2}});
    h.build(ContainerRedirect(), &b);
    h.add(0x500, TargetClass::Door, 100);
    h.eng.containers[kStore] = {{R(0xAAA), 1}};
    h.eng.containers[kGear] = {};
    h.eng.replaceFails = true;                                                              // the engine refuses to rewrite the container
    CHECK(h.run(1, Kind::Activate, R(0x500), 1).result == Result::Executed);
    CHECK(h.eng.containers[kStore] == std::vector<StoredItem>({{R(0xAAA), 1}}));            // nothing moved ...
    REQUIRE(b.peek(kStore, 1) != nullptr);
    CHECK(*b.peek(kStore, 1) == std::vector<StoredItem>({{R(0xB01), 2}}) && h.host->stats().stashFailed >= 1);   // ... and nothing was lost
    h.eng.replaceFails = false;
    h.eng.readFails = true;                                                                 // the container cannot be read at all: it is not part of the interaction
    h.eng.replaceCalls = 0;
    CHECK(h.run(1, Kind::Activate, R(0x500), 2).result == Result::Executed && h.eng.replaceCalls == 0);
    h.eng.readFails = false;
    // the read fails only on the way back: the guest's stash is assumed unchanged and the host's gear still comes back
    int reads = 0;
    h.eng.onCall = [&](const FakeEngine::Call&) { h.eng.readFails = true; ++reads; };
    CHECK(h.run(1, Kind::Activate, R(0x500), 3).result == Result::Executed);
    h.eng.readFails = false;
    CHECK(reads == 1 && h.eng.containers[kStore] == std::vector<StoredItem>({{R(0xAAA), 1}}));
    REQUIRE(b.peek(kStore, 1) != nullptr);
    CHECK(*b.peek(kStore, 1) == std::vector<StoredItem>({{R(0xB01), 2}}));
}

TEST(intents_host, the_fort_scenario_two_players_hand_in_gear_and_each_gets_their_own_back) {
    // Two guests walk onto the barge, the gate strips each into NVCCPlayerStorageREF, they cross, and at the Fort each one asks for the gear back.
    HostRig h;
    StorageBook b = book();
    h.build(ContainerRedirect(), &b);
    h.add(0x510, TargetClass::Activator, 100);  // NVCCTheFortGate: stash
    h.add(0x511, TargetClass::Activator, 100);  // the Fort's return counter: RemoveAllTypedItems from the storage to `player`
    h.eng.containers[kStore] = {};
    h.eng.containers[kGear] = {};
    std::map<PlayerId, std::vector<StoredItem>> inventory;   // what each puppet carries
    inventory[1] = {{R(0xC01), 1}, {R(0xC02), 20}};
    inventory[2] = {{R(0xC03), 1}};
    h.eng.onCall = [&](const FakeEngine::Call& c) {
        if (c.what != "activate") return;
        auto& cont = h.eng.containers[kStore];
        if (c.target == R(0x510)) { for (auto& i : inventory[c.contextSlot]) cont.push_back(i); inventory[c.contextSlot].clear(); }
        if (c.target == R(0x511)) { for (auto& i : cont) inventory[c.contextSlot].push_back(i); cont.clear(); }
    };
    CHECK(h.run(1, Kind::Activate, R(0x510), 1).result == Result::Executed && h.run(2, Kind::Activate, R(0x510), 1).result == Result::Executed);
    CHECK(inventory[1].empty() && inventory[2].empty() && h.eng.containers[kStore].empty());
    CHECK(h.run(2, Kind::Activate, R(0x511), 2).result == Result::Executed);               // the second guest asks first
    CHECK(inventory[2] == std::vector<StoredItem>({{R(0xC03), 1}}) && inventory[1].empty());
    CHECK(h.run(1, Kind::Activate, R(0x511), 2).result == Result::Executed);
    CHECK(inventory[1] == std::vector<StoredItem>({{R(0xC01), 1}, {R(0xC02), 20}}) && inventory[2] == std::vector<StoredItem>({{R(0xC03), 1}}));
    CHECK(b.stashes() == 0 && h.eng.containers[kStore].empty());                            // everything handed back, nothing left behind
    // the host reloads its game: the ledger survives through the state file
    h.eng.onCall = [&](const FakeEngine::Call& c) {
        if (c.what != "activate" || c.target != R(0x510)) return;
        h.eng.containers[kStore].push_back({R(0xC09), 5});
        h.hub.record(fnvcoop::RefAddItem{R(0x510), R(0xC09), 5});   // the engine's hooks record a script's item move: an activation with an effect is not redone by the stand-in
    };
    CHECK(h.run(1, Kind::Activate, R(0x510), 3).result == Result::Executed);
    StorageBook reloaded = book();
    std::string err;
    CHECK(reloaded.loadState(b.stateJson(), &err) && reloaded.peek(kStore, 1) != nullptr && *reloaded.peek(kStore, 1) == std::vector<StoredItem>({{R(0xC09), 5}}));
}
