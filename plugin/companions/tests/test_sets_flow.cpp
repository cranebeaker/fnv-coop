// Companion set pieces across machines (campaign gap G34): a party companion (ED-E), the Honest Hearts swap and the Dead Money single slot, the host's own
// companions committed at the start, the leaver's party companion handed to the host, the escort after a scripted move.
// Twins 933..943.
#include "harness.h"

using namespace stest;

namespace {
const FormRef BOONE = npcForm(1), EDE = npcForm(21), REX = npcForm(22), CHALK = npcForm(31), WC = npcForm(32), DEAN = npcForm(41), DOG = npcForm(43);

void sets(Config& k, const bool* lonesomeRoad) {
    sc::SlotGroup hh;
    hh.name = "honesthearts";
    hh.members = {CHALK, WC};
    k.sets.addGroup(hh);
    sc::SlotGroup dm;
    dm.name = "deadmoney";
    dm.members = {DEAN, npcForm(42), DOG};
    k.sets.addGroup(dm);
    // the rules engine of the host: ED-E is the whole party's once the Lonesome Road recruit went through its filter
    k.isPartyCompanion = [lonesomeRoad](const FormRef& n) { return *lonesomeRoad && n == EDE; };
}

void movePlayer(Cluster& c, PlayerId slot, cc::Vec3 pos, uint32_t cell) {
    for (auto& mc : c.m)
        for (sc::PlayerPlace& p : mc->world.places)
            if (p.slot == slot) { p.pos = pos; p.cell = cell; }
}

const fnvcoop::CompanionOwner* lastCommitted(const Cluster& c, const char* what) {
    for (size_t i = c.commits.size(); i-- > 0;) {
        if (c.commits[i].what != what) continue;
        for (const Effect& e : c.commits[i].fx) if (const auto* x = std::get_if<fnvcoop::CompanionOwner>(&e)) return x;
    }
    return nullptr;
}
}  // namespace

TEST(sets_flow, ed_e_is_the_whole_partys_companion_in_lonesome_road) {   // twins 936, 941, 942
    bool lr = true;
    Cluster c(3, [&](Config& k) { sets(k, &lr); });
    c.addNpc(REX, 32, 1);
    c.addNpc(EDE, 31, 1);
    c.s(0).onTxn({recruitFx(REX, 1, 1)});                 // player 1 already has Rex in the critter slot
    c.s(0).onTxn({recruitFx(EDE, 1, 1)});                 // ... and recruits ED-E too: no follower slot is used
    c.run(300);
    for (int i = 0; i < 3; ++i) {
        CHECK(c.s(i).book().ownerOf(EDE) == 1);
        CHECK(c.s(i).book().isParty(EDE) && !c.s(i).book().isParty(REX));
        CHECK(c.s(i).book().total() == 2);
    }
    CHECK(c.s(0).stats().recruitsDropped == 0);
    // player 2 orders him: the order goes through the host, the machine that simulates ED-E applies it
    CHECK(c.s(2).issueCommand(sc::CmdKind::Wait, EDE));
    c.run(300);
    for (int i = 0; i < 3; ++i) CHECK(c.s(i).book().find(EDE) && c.s(i).book().find(EDE)->state.mode == sc::FollowMode::Wait);
    CHECK(c.e(1).stateCalls.size() == 1 && c.e(1).stateCalls[0].first == EDE);
    CHECK(c.e(2).stateCalls.empty());                     // the commander's own copy of ED-E is not touched
    // a stranger still cannot order or open a normal companion
    CHECK(!c.s(2).issueCommand(sc::CmdKind::Wait, REX));
    CHECK(c.s(2).allowActivation(2, EDE));
    CHECK(!c.s(2).allowActivation(2, REX));
    // only the recruiter's machine flags him as a teammate
    c.run(2000);
    CHECK(c.e(1).teammates.count(EDE) == 1 && c.e(0).teammates.count(EDE) == 0 && c.e(2).teammates.count(EDE) == 0);
}

TEST(sets_flow, ed_e_simulated_by_the_host_obeys_a_guest) {   // twin 942
    bool lr = true;
    Cluster c(3, [&](Config& k) { sets(k, &lr); });
    c.addNpc(EDE, 31, 0);
    c.s(0).onTxn({recruitFx(EDE, 0, 1)});
    c.run(300);
    CHECK(c.s(2).issueCommand(sc::CmdKind::Wait, EDE));
    c.run(300);
    CHECK(c.e(0).stateCalls.size() == 1 && c.e(0).stateCalls[0].second.mode == sc::FollowMode::Wait);
    CHECK(c.e(2).stateCalls.empty());
}

TEST(sets_flow, the_rules_engine_may_name_him_after_the_recruit_was_seen) {   // twin 936
    bool lr = false;
    Cluster c(3, [&](Config& k) { sets(k, &lr); });
    c.addNpc(EDE, 31, 1);
    c.s(0).onTxn({recruitFx(EDE, 1, 1)});
    c.run(300);
    CHECK(!c.s(2).book().isParty(EDE));
    lr = true;                                            // the rules engine's filter caught up
    c.run(1500);
    for (int i = 0; i < 3; ++i) CHECK(c.s(i).book().isParty(EDE));
    CHECK(c.s(0).stats().partyFlagged == 1);
    CHECK(c.s(1).book().digest() == c.s(0).book().digest());
}

TEST(sets_flow, ed_e_stays_when_his_recruiter_leaves) {   // twins 937, 938
    bool lr = true;
    Cluster c(3, [&](Config& k) { sets(k, &lr); });
    c.addNpc(EDE, 31, 1);
    c.addNpc(REX, 32, 1);
    c.s(0).onTxn({recruitFx(EDE, 1, 1)});
    c.s(0).onTxn({recruitFx(BOONE, 1, 0)});
    c.run(300);
    c.s(0).playerLeft(1);
    c.run(300);
    CHECK(!c.s(0).book().isCompanion(BOONE));            // the ordinary companion is released
    CHECK(c.s(0).book().isCompanion(EDE) && c.s(0).book().ownerOf(EDE) == kHostPlayer && c.s(0).book().isParty(EDE));
    CHECK(c.s(2).book().ownerOf(EDE) == kHostPlayer);    // the other guest hears it
    const fnvcoop::CompanionOwner* handed = lastCommitted(c, "party-companion-handed");
    CHECK(handed != nullptr);
    if (handed) CHECK(handed->npc == EDE && handed->owner == kHostPlayer && handed->active);
    CHECK(c.s(0).stats().partyHanded == 1);
}

TEST(sets_flow, a_hire_that_fires_another_players_companion_is_put_right) {   // twins 934, 935
    bool lr = false;
    Cluster c(3, [&](Config& k) { sets(k, &lr); });
    c.addNpc(CHALK, 41, 1);
    c.addNpc(WC, 42, 2);
    c.s(0).onTxn({recruitFx(CHALK, 1)});
    c.run(300);
    const size_t fed = c.at(0).fed.size();
    // player 2 hires Waking Cloud; the INFO result fires "the hired Follows-Chalk" off the shared flag
    c.s(0).onTxn({recruitFx(WC, 2), dismissFx(CHALK, 2)});
    c.run(1000);
    for (int i = 0; i < 3; ++i) {
        CHECK(c.s(i).book().ownerOf(CHALK) == 1);
        CHECK(c.s(i).book().ownerOf(WC) == 2);
    }
    CHECK(c.s(0).stats().swapsReverted == 1);
    CHECK(c.committedCount("companion-repair") == 1);
    const fnvcoop::CompanionOwner* fix = lastCommitted(c, "companion-repair");
    CHECK(fix != nullptr);
    if (fix) CHECK(fix->npc == CHALK && fix->owner == 1 && fix->active);
    CHECK(c.at(0).fed.size() > fed);                      // the actor layer hears the corrected owner
    CHECK(c.s(0).stats().repairsCommitted == 1);
}

// W3a item 6: the commit asks the book first, so the dismissal never leaves the host (twin 944: the commit is not vetted).
TEST(sets_flow, the_commit_vet_takes_a_swap_dismissal_out_of_the_txn_before_it_is_sent) {
    bool lr = false;
    Cluster c(3, [&](Config& k) { sets(k, &lr); });
    c.addNpc(CHALK, 41, 1);
    c.addNpc(WC, 42, 2);
    c.s(0).onTxn({recruitFx(CHALK, 1)});
    c.run(300);
    std::vector<fnvcoop::Effect> fx{recruitFx(WC, 2), dismissFx(CHALK, 2), fnvcoop::AddNote{FormRef("falloutnv.esm", 0x77)}};
    CHECK(c.s(0).vetOutgoing(fx) == 1);                   // the dismissal of player 1's companion by player 2's hire
    REQUIRE(fx.size() == 2);
    CHECK(std::holds_alternative<fnvcoop::CompanionOwner>(fx[0]) && std::holds_alternative<fnvcoop::AddNote>(fx[1]));   // the recruit and the unrelated effect stay, in order
    std::vector<fnvcoop::Effect> own{recruitFx(WC, 1), dismissFx(CHALK, 1)};                                           // a player's own swap is untouched
    CHECK(c.s(0).vetOutgoing(own) == 0 && own.size() == 2);
    CHECK(c.s(1).vetOutgoing(fx) == 0);                   // only the host vets
}

TEST(sets_flow, a_players_own_swap_is_left_alone) {   // twins 934, 935
    bool lr = false;
    Cluster c(3, [&](Config& k) { sets(k, &lr); });
    c.addNpc(CHALK, 41, 2);
    c.addNpc(WC, 42, 2);
    c.s(0).onTxn({recruitFx(CHALK, 2)});
    c.run(300);
    c.s(0).onTxn({recruitFx(WC, 2), dismissFx(CHALK, 2)});
    c.run(1000);
    for (int i = 0; i < 3; ++i) CHECK(!c.s(i).book().isCompanion(CHALK) && c.s(i).book().ownerOf(WC) == 2);
    CHECK(c.s(0).stats().swapsReverted == 0 && c.s(0).stats().slotDenied == 0);
    CHECK(c.committedCount("companion-repair") == 0);
}

TEST(sets_flow, a_player_has_one_dead_money_companion) {   // twins 934, 935
    bool lr = false;
    Cluster c(3, [&](Config& k) { sets(k, &lr); });
    c.addNpc(DEAN, 51, 1);
    c.addNpc(DOG, 53, 1);
    c.s(0).onTxn({recruitFx(DEAN, 1, 0)});
    c.run(300);
    c.e(0).teammates.insert(DOG);                          // the host's engine ran the hire script and flagged the dog
    c.s(0).onTxn({recruitFx(DOG, 1, 1)});                  // the same player: a second member of the one-at-a-time group
    c.run(1000);
    CHECK(!c.s(0).book().isCompanion(DOG) && c.s(0).book().ownerOf(DEAN) == 1);
    CHECK(c.s(0).stats().slotDenied == 1);
    const fnvcoop::CompanionOwner* undo = lastCommitted(c, "companion-repair");
    CHECK(undo != nullptr);
    if (undo) CHECK(undo->npc == DOG && !undo->active);
    c.run(3500);
    CHECK(c.e(0).teammates.count(DOG) == 0);               // the stray flag comes off
    // another player's slot is their own
    c.addNpc(npcForm(42), 52, 2);
    c.s(0).onTxn({recruitFx(npcForm(42), 2, 0)});
    c.run(300);
    CHECK(c.s(0).book().ownerOf(npcForm(42)) == 2);
}

TEST(sets_flow, the_hosts_own_companions_are_committed_when_the_session_starts) {   // twin 933
    Cluster c(3, nullptr, true, false);
    c.addNpc(BOONE, 11, 0);
    c.e(0).teammates.insert(BOONE);                        // a save with Boone already in the party
    c.tick(10);
    CHECK(c.s(0).book().ownerOf(BOONE) == kHostPlayer);
    CHECK(c.committedCount("seed-companions") == 1);
    CHECK(c.s(0).stats().seedCommitted == 1);
    const fnvcoop::CompanionOwner* x = lastCommitted(c, "seed-companions");
    CHECK(x != nullptr);
    if (x) CHECK(x->npc == BOONE && x->owner == kHostPlayer && x->active);
    c.run(2000);
    CHECK(c.committedCount("seed-companions") == 1);       // once
    CHECK(c.s(1).book().ownerOf(BOONE) == kHostPlayer);
}

TEST(sets_flow, a_recruiters_companion_is_brought_after_a_scripted_move_of_its_player) {   // twins 939, 940
    Cluster c(3);
    c.addNpc(BOONE, 11, 1, FormRef(), {0, 0, 0}, 1);
    c.s(0).onTxn({recruitFx(BOONE, 1)});
    c.run(20000);
    CHECK(c.e(1).summons.empty());                         // next to its player: left alone
    movePlayer(c, 1, {60000, 0, 0}, 7);                    // the Dam start moved player 1
    movePlayer(c, 0, {-90000, 0, 0}, 8);                   // the others are elsewhere too: Boone is far from THEM as well, and is still nobody else's to move
    movePlayer(c, 2, {0, 90000, 0}, 9);
    c.run(3000);
    CHECK(c.e(1).summons.empty());                         // inside the grace: it may be walking after its player
    c.run(3000);
    CHECK(c.e(1).summons.size() == 1 && c.e(1).summons[0] == BOONE);
    CHECK(c.e(0).summons.empty() && c.e(2).summons.empty());   // nobody else moves another player's companion
    CHECK(c.s(1).stats().escorted == 1);
    c.run(2000);
    CHECK(c.e(1).summons.size() == 1);                     // the cooldown holds
    // told to wait: it stays where it is
    for (auto& mc : c.m) mc->world.npcs[BOONE].waiting = true;
    c.run(40000);
    CHECK(c.e(1).summons.size() == 1);
}

TEST(sets_flow, the_status_names_the_new_counters) {
    bool lr = false;
    Cluster c(2, [&](Config& k) { sets(k, &lr); });
    const std::string j = c.s(0).statusJson();
    for (const char* key : {"partyFlagged", "partyHanded", "swapsReverted", "slotDenied", "repairsCommitted", "seedCommitted", "escorted", "escortFailed"})
        CHECK(j.find(key) != std::string::npos);
}

TEST(sets_flow, the_dialogue_registry_is_fed_the_roster_with_the_party_flag_on_every_machine) {   // twin 943
    bool lr = true;
    std::map<PlayerId, std::vector<sc::Companion>> seen;
    std::map<PlayerId, int> calls;
    Cluster c(3, [&](Config& k) {
        sets(k, &lr);
        const PlayerId me = k.self;
        k.onRoster = [&seen, &calls, me](const std::vector<sc::Companion>& all) { seen[me] = all; ++calls[me]; };
    });
    c.addNpc(EDE, 31, 1);
    c.s(0).onTxn({recruitFx(EDE, 1, 1)});
    c.run(300);
    for (PlayerId i = 0; i < 3; ++i) {
        CHECK(calls[i] > 0);
        REQUIRE(seen[i].size() == 1);
        CHECK(seen[i][0].npc == EDE && seen[i][0].owner == 1 && seen[i][0].party);
    }
}
