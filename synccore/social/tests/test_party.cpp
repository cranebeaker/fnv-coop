// Companion set pieces (campaign gap G34): party companions (ED-E), single-slot groups and the swap guard (Dead Money, Honest Hearts), the view policy, the escort.
// Twins 884..898 and 900 (companions.cpp, slots.cpp, escort.cpp).
#include "util.h"
#include "social/escort.h"
#include "social/slots.h"

using namespace social_test;

namespace {
FormRef ede(uint32_t n = 1) { return FormRef("falloutnv.esm", 0x10C760u + n); }

CompanionBook bookWithEde() {
    CompanionBook b;
    b.addFixedParty(ede());
    return b;
}

SlotRules hhRules() {
    SlotRules r;
    SlotGroup g;
    g.name = "honesthearts";
    g.members = {npc(41), npc(42)};   // Follows-Chalk, Waking Cloud
    r.addGroup(g);
    SlotGroup dm;
    dm.name = "deadmoney";
    dm.members = {npc(51), npc(52), npc(53)};   // Dean, Christine, Dog
    r.addGroup(dm);
    return r;
}
}  // namespace

TEST(party, a_party_companion_uses_nobodys_follower_slot) {   // twin 884
    CompanionBook b = bookWithEde();
    CHECK(b.recruit(npc(1), 1, CompanionKind::Critter).ok);       // Rex takes player 1's critter slot
    CHECK(b.recruit(ede(), 1, CompanionKind::Critter).ok);        // ED-E would be a second critter: party companions have no slot
    CHECK(b.isParty(ede()) && !b.isParty(npc(1)));
    CHECK(b.capCountOf(1, CompanionKind::Critter) == 1);
    CHECK(!b.overCap(1));
    // the party total counts the others only: with ED-E in the book three more fit
    CHECK(b.recruit(npc(2), 1, CompanionKind::Humanoid).ok);
    CHECK(b.recruit(npc(3), 2, CompanionKind::Humanoid).ok);
    CHECK(b.recruit(npc(4), 2, CompanionKind::Critter).ok);
    CHECK(b.total() == 5 && b.capTotal() == 4);
    CHECK(!b.partyOverCap());
    RecruitVerdict full = b.recruit(npc(5), 3, CompanionKind::Humanoid);
    CHECK(!full.ok && full.reason == RecruitReason::PartyCap);
    // a committed fact (a save, a rejoin) of the same shape is not an over-cap either
    CompanionBook c = bookWithEde();
    c.applyEffect(recruitFx(npc(1), 1, 1));
    c.applyEffect(recruitFx(ede(), 1, 1));
    CHECK(!c.overCap(1));
}

TEST(party, the_rules_engine_oracle_decides_and_a_refresh_follows_it) {   // twin 884 (flag), 890 (digest)
    CompanionBook b;
    bool lonesomeRoad = false;
    b.setPartyOracle([&](const FormRef& n) { return lonesomeRoad && n == ede(); });
    b.applyEffect(recruitFx(ede(), 1, 1));
    CHECK(!b.isParty(ede()));
    const uint64_t before = b.digest();
    lonesomeRoad = true;                                     // the rules engine saw the owner=party recruit
    std::vector<FormRef> changed = b.refreshParty();
    CHECK(changed.size() == 1 && changed[0] == ede() && b.isParty(ede()));
    CHECK(b.digest() != before);                             // the flag is part of what machines compare
    CHECK(b.refreshParty().empty());
    lonesomeRoad = false;
    CHECK(b.refreshParty().size() == 1 && !b.isParty(ede()));
}

TEST(party, any_player_commands_a_party_companion_but_only_the_owner_a_normal_one) {   // twins 885, 891
    CompanionBook b = bookWithEde();
    b.applyEffect(recruitFx(ede(), 1, 1));
    b.applyEffect(recruitFx(npc(7), 1, 0));
    CommandJudge j;
    CompanionCmd c;
    c.id = (2ull << 48) | 1; c.from = 2; c.npc = ede(); c.kind = CmdKind::Wait;
    CmdVerdict v = j.judge(c, 2, 1000, b);
    CHECK(v.apply && v.stateChanged && v.newState.mode == FollowMode::Wait);
    CompanionCmd n = c; n.id = (2ull << 48) | 2; n.npc = npc(7);
    v = j.judge(n, 2, 5000, b);
    CHECK(!v.apply && v.reason == CmdReason::NotOwner);        // the stranger is still refused for a normal companion
    CHECK(b.dismiss(ede(), 2));                                // a party companion may be let go by any player
    CHECK(!b.dismiss(npc(7), 2));
}

TEST(party, the_wheel_and_the_inventory_of_a_party_companion_are_for_everybody) {   // twins 886, 887
    CompanionBook b = bookWithEde();
    b.applyEffect(recruitFx(ede(), 1, 1));
    b.applyEffect(recruitFx(npc(7), 1, 0));
    CHECK(gateActivation(b, 2, ede(), ForeignPolicy::Deny).action == ActivateAction::PartyOpens);
    CHECK(!gateActivation(b, 2, ede(), ForeignPolicy::Deny).suppress());
    CHECK(gateActivation(b, 1, ede(), ForeignPolicy::Deny).action == ActivateAction::OwnerOpens);
    CHECK(gateActivation(b, 2, npc(7), ForeignPolicy::Deny).suppress());
    CompanionInventory inv;
    InvDelta d; d.txn = (2ull << 48) | 1; d.npc = ede(); d.item = item(1); d.delta = 3;
    CHECK(inv.judge(d, 2, b).apply);
    InvDelta e = d; e.txn = (2ull << 48) | 2; e.npc = npc(7);
    InvVerdict no = inv.judge(e, 2, b);
    CHECK(!no.apply && no.reason == InvReason::NotOwner);
}

TEST(party, a_leaving_recruiter_hands_the_party_companion_to_the_host) {   // twins 888, 889
    CompanionBook b = bookWithEde();
    b.applyEffect(recruitFx(ede(), 2, 1));
    b.applyEffect(recruitFx(npc(7), 2, 0));
    std::vector<FormRef> gone = b.playerLeft(2);
    CHECK(gone.size() == 1 && gone[0] == npc(7));              // the normal companion is released
    CHECK(b.isCompanion(ede()) && b.ownerOf(ede()) == 2);      // the party companion stays until it is handed over
    std::vector<FormRef> handed = b.handParty(2, kHostPlayer);
    CHECK(handed.size() == 1 && handed[0] == ede());
    CHECK(b.ownerOf(ede()) == kHostPlayer && b.isParty(ede()));
    CHECK(b.handParty(2, kHostPlayer).empty());                // nothing left to hand over
}

TEST(party, the_party_flag_travels_and_a_bad_flag_is_refused) {   // twin 890
    Companion c; c.npc = ede(); c.owner = 1; c.kind = CompanionKind::Critter; c.party = true;
    Companion c2 = roundTrip(c);
    CHECK(c2.party && c2.owner == 1 && c2.npc == ede());
    Companion n = c; n.party = false;
    CHECK(!roundTrip(n).party);
    // a flags byte above 1 is not a roster entry
    Archive w = Archive::writer();
    Companion x = c;
    io(w, x);
    Bytes bytes = w.finish();
    bytes.back() = 2;
    Archive r = Archive::reader(bytes.data(), bytes.size());
    Companion out;
    io(r, out);
    CHECK(!r.ok());
    // replaceRoster carries it to a joiner
    CompanionBook host = bookWithEde();
    host.applyEffect(recruitFx(ede(), 1, 1));
    CompanionBook joiner;
    joiner.replaceRoster(host.roster());
    CHECK(joiner.isParty(ede()) && joiner.digest() == host.digest());
}

TEST(slots, another_players_companion_fired_by_my_swap_is_put_back) {   // twins 892, 894
    SlotRules rules = hhRules();
    CompanionBook b;
    b.applyEffect(recruitFx(npc(41), 1));                      // player 1 has Follows-Chalk
    // player 2 hires Waking Cloud; the INFO result fires "the hired Follows-Chalk" off the shared flag
    std::vector<Effect> txn = {recruitFx(npc(42), 2), dismissFx(npc(41), 2)};
    TxnVet v = vetCompanionTxn(b, rules, txn);
    CHECK(v.swapsReverted == 1);
    REQUIRE(v.learn.size() == 1);
    CHECK(std::get<CompanionOwner>(v.learn[0]).npc == npc(42) && std::get<CompanionOwner>(v.learn[0]).active);
    REQUIRE(v.repairs.size() == 1);
    const CompanionOwner& fix = std::get<CompanionOwner>(v.repairs[0]);
    CHECK(fix.npc == npc(41) && fix.owner == 1 && fix.active);
    // the book that learns only what vet kept: player 1 still owns Chalk, player 2 owns Waking Cloud
    for (const Effect& e : v.learn) b.applyEffect(e);
    CHECK(b.ownerOf(npc(41)) == 1 && b.ownerOf(npc(42)) == 2);
}

TEST(slots, a_players_own_swap_is_the_vanilla_behaviour) {   // twins 894, 895
    SlotRules rules = hhRules();
    CompanionBook b;
    b.applyEffect(recruitFx(npc(41), 2));
    std::vector<Effect> txn = {recruitFx(npc(42), 2), dismissFx(npc(41), 2)};
    TxnVet v = vetCompanionTxn(b, rules, txn);
    CHECK(v.swapsReverted == 0 && v.slotDenied == 0 && v.repairs.empty());
    CHECK(v.learn.size() == 2);
    for (const Effect& e : v.learn) b.applyEffect(e);
    CHECK(!b.isCompanion(npc(41)) && b.ownerOf(npc(42)) == 2);
}

TEST(slots, a_second_member_of_a_single_slot_group_is_turned_back) {   // twin 893
    SlotRules rules = hhRules();
    CompanionBook b;
    b.applyEffect(recruitFx(npc(51), 1));                      // Dean is player 1's Dead Money companion
    TxnVet v = vetCompanionTxn(b, rules, {recruitFx(npc(53), 1, 1)});   // the same player recruits the dog too
    CHECK(v.slotDenied == 1 && v.learn.empty());
    REQUIRE(v.repairs.size() == 1);
    CHECK(!std::get<CompanionOwner>(v.repairs[0]).active && std::get<CompanionOwner>(v.repairs[0]).npc == npc(53));
    // another player has their own slot
    TxnVet other = vetCompanionTxn(b, rules, {recruitFx(npc(52), 2)});
    CHECK(other.slotDenied == 0 && other.learn.size() == 1);
    // a companion outside every group is not touched
    TxnVet free = vetCompanionTxn(b, rules, {recruitFx(npc(99), 1)});
    CHECK(free.slotDenied == 0 && free.learn.size() == 1);
}

TEST(slots, a_dismissal_that_no_recruit_caused_is_a_real_fire) {   // twin 892
    SlotRules rules = hhRules();
    CompanionBook b;
    b.applyEffect(recruitFx(npc(41), 1));
    // the Joshua story fork fires Waking Cloud for whoever owns her: no recruit in the same txn, so it goes through
    TxnVet v = vetCompanionTxn(b, rules, {dismissFx(npc(41), 2)});
    CHECK(v.swapsReverted == 0 && v.learn.size() == 1);
}

TEST(slots, each_player_reads_their_own_current_follower) {   // twin 899
    SlotRules rules = hhRules();
    CompanionBook b;
    b.applyEffect(recruitFx(npc(51), 1));   // Dean is player 1's
    b.applyEffect(recruitFx(npc(52), 2));   // Christine is player 2's
    CHECK(currentMember(b, rules, "deadmoney", 1) == npc(51));
    CHECK(currentMember(b, rules, "deadmoney", 2) == npc(52));
    CHECK(currentMember(b, rules, "deadmoney", 3) == FormRef());      // nobody's: rCurrentFollower reads 0 for them
    CHECK(currentMember(b, rules, "honesthearts", 1) == FormRef());
    CHECK(currentMember(b, rules, "nosuchgroup", 1) == FormRef());
}

TEST(slots, the_data_file_parses_and_the_view_policy_picks_the_union_for_the_movers) {   // twin 900
    SlotRules r;
    const char* txt =
        "# comment\n"
        "group deadmoney DeadMoney.esm:0012FE,DeadMoney.esm:001309 \n"
        "group honesthearts HonestHearts.esm:0094D0,HonestHearts.esm:009665\r\n"
        "party FalloutNV.esm:10C769,FalloutNV.esm:1572E2\n"
        "view-party vCCmovetothefortSCRIPT\n"
        "view-party YesManDeadSCRIPT\n"
        "nonsense line\n"
        "group broken NotARef\n";
    SlotRules::LoadStats st = r.load(txt);
    CHECK(st.groups == 2 && st.party == 1 && st.viewParty == 2 && st.bad == 2);
    FormRef dean;
    CHECK(parseSetRef("DeadMoney.esm:0012FE", dean));
    CHECK(dean == FormRef("deadmoney.esm", 0x12FEu));
    CHECK(r.groupOf(dean) && r.groupOf(dean)->name == "deadmoney");
    CHECK(r.partyNpcs().size() == 2);
    CHECK(viewSlotFor(r, "vccmovetothefortscript", 1) == kViewParty);   // case does not matter
    CHECK(viewSlotFor(r, "NVDLC02HireTriggerSCRIPT", 1) == 1);          // a hire reads the acting player's own flags
}

TEST(escort, a_companion_left_behind_by_a_scripted_move_is_brought_after_the_grace) {   // twins 897, 898
    Escort e;
    PlayerPlace me; me.slot = 1; me.cell = 9; me.pos = {50000, 0, 0};
    EscortSubject s; s.npc = npc(1); s.loaded = true; s.cell = 2; s.pos = {0, 0, 0};   // the player was warped far away
    CHECK(e.poll(1000, me, {s}).empty());                        // just noticed: it may be walking after the player
    CHECK(e.poll(4000, me, {s}).empty());                        // 3 s: still inside the grace
    std::vector<FormRef> bring = e.poll(5200, me, {s});
    CHECK(bring.size() == 1 && bring[0] == npc(1));
    CHECK(e.poll(6000, me, {s}).empty());                        // the next grace starts over, and the cooldown holds
    CHECK(e.poll(10500, me, {s}).empty());                       // grace over again, cooldown (12 s) not
    CHECK(e.poll(17500, me, {s}).size() == 1);                   // now again
}

TEST(escort, a_companion_that_is_close_waiting_dead_or_not_here_is_left_alone) {   // twin 896
    Escort e;
    PlayerPlace me; me.slot = 1; me.cell = 9; me.pos = {0, 0, 0};
    EscortSubject close; close.npc = npc(1); close.loaded = true; close.pos = {300, 0, 0};
    EscortSubject waiting; waiting.npc = npc(2); waiting.loaded = true; waiting.waiting = true; waiting.pos = {90000, 0, 0};
    EscortSubject dead; dead.npc = npc(3); dead.loaded = true; dead.alive = false; dead.pos = {90000, 0, 0};
    EscortSubject away; away.npc = npc(4); away.loaded = false; away.pos = {90000, 0, 0};
    for (uint64_t t = 1000; t < 40000; t += 500) CHECK(e.poll(t, me, {close, waiting, dead, away}).empty());
    // a companion that comes back inside the radius before the grace ends is forgotten
    EscortSubject lag = close; lag.npc = npc(5); lag.pos = {9000, 0, 0};
    CHECK(e.poll(50000, me, {lag}).empty());
    lag.pos = {400, 0, 0};
    CHECK(e.poll(52000, me, {lag}).empty());
    lag.pos = {9000, 0, 0};
    CHECK(e.poll(55000, me, {lag}).empty());                     // far again: the grace starts from here, not from 50000
    CHECK(e.poll(59500, me, {lag}).size() == 1);
}
