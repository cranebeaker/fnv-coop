// Menu-less transfers (a take or put the engine did on a closed container without the container menu: console RemoveItem + AddItem, a quick-loot
// mod, the benchmark's "both grab at once"). They used to bypass the lock: both players kept all the items (3 duct tape became 6).
// Twins 158..162 are guarded here (158 transfers never looked for, 159 any difference believed to be the player's, 160 no grace, 161 one player move
// explains every container's difference, 162 a refused transfer closes a menu that is not ours).
#include "rig.h"

using namespace nvl_test;

namespace {

constexpr NetId kCorpse = 0x200;
constexpr uint32_t kTape = 7;

// the same corpse on every engine, registered by the host the way a loaded cell / a death would, then everybody settles
void corpse(Cluster& c, const Items& items) {
    for (auto& mc : c.m) mc->eng.add(kCorpse, items, lc::ContainerKind::Corpse);
    c.host().sync->onContainerLoaded(c.host().eng.conts[kCorpse].ref);
    c.run(800);
}
const lc::ContainerRec* tableOf(Cluster& c) { return c.host().sync->host()->container(kCorpse); }
int32_t inPlayers(Cluster& c, uint32_t base) {
    int32_t n = 0;
    for (auto& mc : c.m) n += count(mc->eng.player, base);
    return n;
}

// both players take one tape at a time, every 150 ms, from their own copy, until their copy is empty
void grab(Cluster& c, std::initializer_list<size_t> who, int rounds = 5) {
    for (int r = 0; r < rounds; ++r) {
        for (size_t i : who) c.machine(i).eng.silentTake(kCorpse, kTape, 1);
        c.run(150);
    }
}

}  // namespace

TEST(menuless, two_players_grabbing_at_once_end_up_with_exactly_what_the_corpse_held_host_and_guest) {
    Cluster c(2);
    corpse(c, I({{kTape, 3}}));
    REQUIRE(tableOf(c));
    CHECK(count(tableOf(c)->items, kTape) == 3);
    grab(c, {0, 1});
    c.settle();
    CHECK(inPlayers(c, kTape) == 3);
    CHECK(count(tableOf(c)->items, kTape) == 0);
    for (auto& mc : c.m) CHECK(count(mc->eng.conts[kCorpse].items, kTape) == 0);
    CHECK(c.host().sync->host()->stats().closeMismatch == 0);
    CHECK(c.host().sync->stats().transfersMenuless + c.machine(1).sync->stats().transfersMenuless >= 2);   // the loser asked as well and was refused
    CHECK(tableOf(c)->holder == lc::kNoPlayer);
}

TEST(menuless, two_guests_grabbing_at_once_end_up_with_exactly_what_the_corpse_held) {
    Cluster c(3);
    corpse(c, I({{kTape, 3}}));
    REQUIRE(tableOf(c));
    grab(c, {1, 2});
    c.settle();
    CHECK(inPlayers(c, kTape) == 3);
    CHECK(count(tableOf(c)->items, kTape) == 0);
    for (auto& mc : c.m) CHECK(count(mc->eng.conts[kCorpse].items, kTape) == 0);
    CHECK(tableOf(c)->holder == lc::kNoPlayer);
}

TEST(menuless, a_grab_by_all_three_at_once_splits_a_larger_stack_without_a_duplicate) {
    Cluster c(3);
    corpse(c, I({{kTape, 8}, {9, 2}}));
    for (int r = 0; r < 12; ++r) {
        for (size_t i = 0; i < 3; ++i) { c.machine(i).eng.silentTake(kCorpse, kTape, 1); c.machine(i).eng.silentTake(kCorpse, 9, 1); }
        c.run(110);
    }
    c.settle();
    CHECK(inPlayers(c, kTape) == 8);
    CHECK(inPlayers(c, 9) == 2);
    CHECK(tableOf(c)->items.empty());
}

TEST(menuless, a_lone_take_goes_through_the_lock_and_stays_with_the_player) {
    Cluster c(3);
    corpse(c, I({{kTape, 3}}));
    Machine& g = c.machine(1);
    CHECK(g.eng.silentTake(kCorpse, kTape, 2));
    c.settle();
    CHECK(count(g.eng.player, kTape) == 2);
    CHECK(count(tableOf(c)->items, kTape) == 1);
    CHECK(count(c.host().eng.conts[kCorpse].items, kTape) == 1);       // the others' copies follow
    CHECK(count(c.machine(2).eng.conts[kCorpse].items, kTape) == 1);
    CHECK(g.eng.adjustCalls == 0);                                      // nothing was taken back
    CHECK(g.eng.closeCalls == 0);                                       // there was never a menu
    CHECK(g.sync->stats().transfersMenuless == 1);
    CHECK(tableOf(c)->holder == lc::kNoPlayer);
}

TEST(menuless, a_take_the_host_refuses_is_taken_back_and_the_container_is_put_right_and_no_menu_is_closed) {
    Cluster c(3);
    corpse(c, I({{kTape, 3}}));
    Machine& holder = c.machine(2);
    holder.eng.openMenuOn(kCorpse);                                     // somebody has the corpse open
    c.run(300);
    REQUIRE(tableOf(c)->holder == 2);
    Machine& g = c.machine(1);
    CHECK(g.eng.silentTake(kCorpse, kTape, 3));
    CHECK(count(g.eng.player, kTape) == 3);
    c.run(600);
    CHECK(count(g.eng.player, kTape) == 0);                             // refused: the items go back
    CHECK(count(g.eng.conts[kCorpse].items, kTape) == 3);               // and the container shows the shared contents again
    CHECK(count(tableOf(c)->items, kTape) == 3);
    CHECK(g.eng.closeCalls == 0);                                       // twin 162: the engine is not asked to close a menu this machine never opened
    CHECK(holder.eng.menu != 0);
    // later, with the lock free, the same player's next menu-less take is a take again
    holder.eng.menu = 0;
    c.run(800);
    CHECK(g.eng.silentTake(kCorpse, kTape, 2));
    c.settle();
    CHECK(count(g.eng.player, kTape) == 2);
    CHECK(count(tableOf(c)->items, kTape) == 1);
    CHECK(count(c.host().eng.conts[kCorpse].items, kTape) == 1);
}

TEST(menuless, a_put_without_the_menu_is_a_put_and_one_the_host_refuses_gives_the_items_back) {
    Cluster c(3);
    corpse(c, I({{kTape, 1}, {50, 1}}));                                // (a put is attributed only for an item the container already holds: that is when the player's count was looked at)
    Machine& g = c.machine(1);
    g.eng.player = I({{50, 4}});
    c.run(200);                                                         // the player's count is looked at while the corpse still matches
    CHECK(g.eng.silentPut(kCorpse, 50, 3));
    c.settle();
    CHECK(count(tableOf(c)->items, 50) == 4);
    CHECK(count(c.host().eng.conts[kCorpse].items, 50) == 4);
    CHECK(count(g.eng.player, 50) == 1);
    // a put while somebody else holds the container
    Machine& h = c.machine(2);
    h.eng.openMenuOn(kCorpse);
    c.run(300);
    CHECK(g.eng.silentPut(kCorpse, 50, 1));
    c.run(600);
    CHECK(count(g.eng.player, 50) == 1);                                // given back
    CHECK(count(tableOf(c)->items, 50) == 4);
    CHECK(count(g.eng.conts[kCorpse].items, 50) == 4);
}

TEST(menuless, a_script_that_removes_items_without_the_player_getting_them_is_not_a_take) {
    Cluster c(3);
    corpse(c, I({{kTape, 5}}));
    Machine& g = c.machine(1);
    CHECK(g.eng.silentTake(kCorpse, kTape, 2, false));                  // removed, the player did not gain anything
    c.run(1200);
    CHECK(count(g.eng.conts[kCorpse].items, kTape) == 5);               // a guest's engine is put back to the shared contents
    CHECK(count(tableOf(c)->items, kTape) == 5);
    CHECK(g.sync->stats().transfersMenuless == 0);
    // the host's script is the authority: the table follows, and nobody's inventory is touched
    CHECK(c.host().eng.silentTake(kCorpse, kTape, 4, false));
    c.run(1500);
    CHECK(count(tableOf(c)->items, kTape) == 1);
    CHECK(count(g.eng.conts[kCorpse].items, kTape) == 1);
    CHECK(c.host().sync->stats().transfersMenuless == 0);
    CHECK(inPlayers(c, kTape) == 0);
}

TEST(menuless, the_additem_that_follows_the_removeitem_a_moment_later_is_still_the_players_take) {
    Cluster c(2);
    corpse(c, I({{kTape, 3}}));
    Machine& h = c.host();
    CHECK(h.eng.silentTake(kCorpse, kTape, 3, false));                  // RemoveItem on the corpse ...
    c.run(120);                                                         // ... a few frames pass (more than a sweep) ...
    h.eng.player[F(kTape)] += 3;                                        // ... then player.AddItem
    c.settle();
    CHECK(h.sync->stats().transfersMenuless == 1);                      // twin 160: without the grace the sweep had already adopted it as a script's change
    CHECK(h.sync->stats().sweepsChanged == 0);
    CHECK(count(h.eng.player, kTape) == 3);
    CHECK(count(tableOf(c)->items, kTape) == 0);
}

TEST(menuless, with_menuless_switched_off_the_old_handling_stays) {
    // the service switched off by config ([loot] menuless=0): the old handling
    SyncConfig cfg = Cluster::defaultCfg();
    cfg.menuless = false;
    Cluster c(2, cfg);
    corpse(c, I({{kTape, 3}}));
    Machine& g = c.machine(1);
    g.eng.silentTake(kCorpse, kTape, 3);
    c.run(1500);
    CHECK(g.sync->stats().transfersMenuless == 0);
    CHECK(count(g.eng.conts[kCorpse].items, kTape) == 3);               // audited back, as before
}

TEST(menuless, one_gain_of_the_player_explains_one_container_only) {
    Cluster c(2);
    corpse(c, I({{kTape, 3}}));
    for (auto& mc : c.m) mc->eng.add(0x201, I({{kTape, 3}}), lc::ContainerKind::Corpse);
    c.host().sync->onContainerLoaded(c.host().eng.conts[0x201].ref);
    c.run(800);
    REQUIRE(c.host().sync->host()->container(0x201));
    Machine& g = c.machine(1);
    CHECK(g.eng.silentTake(kCorpse, kTape, 1));            // the player got one tape from the first corpse ...
    CHECK(g.eng.silentTake(0x201, kTape, 1, false));       // ... and something removed one from the second without the player getting it
    c.settle();
    CHECK(count(tableOf(c)->items, kTape) == 2);
    CHECK(count(c.host().sync->host()->container(0x201)->items, kTape) == 3);   // twin 161: it was believed there too
    CHECK(count(g.eng.conts[0x201].items, kTape) == 3);
    CHECK(count(g.eng.player, kTape) == 1);
}

// ---- the benchmark's loot step, with its real timing: each bot does RemoveItem on the corpse, a bridge round trip later AddItem on the player, and
// loops every ~200 ms until its OWN copy of the corpse reads empty. The bots are not in step (different bridge latencies) and the engine ticks at a frame
// rate. bench dino-20261027-144058 (the build with the menu-less scan in it): host gained 3, client gained 3 from a corpse with 3 duct tape.
namespace {

struct BenchBot {
    size_t who = 0;
    uint64_t next = 0;          // when this bot acts next
    int phase = 0;              // 0 = check + RemoveItem, 1 = AddItem
    uint32_t gap = 25;          // RemoveItem -> AddItem (one bridge round trip)
    uint32_t period = 200;      // AddItem -> next count check
    int32_t chunk = 1;          // items per RemoveItem
    int32_t held = 0;
    bool done = false;
};

void stepBot(Cluster& c, BenchBot& b) {
    if (b.done || c.now < b.next) return;
    FakeEngine& e = c.machine(b.who).eng;
    if (b.phase == 0) {
        const int32_t have = count(e.conts[kCorpse].items, kTape);
        if (have <= 0) { b.done = true; return; }
        b.held = std::min(b.chunk, have);
        e.silentTake(kCorpse, kTape, b.held, false);
        b.phase = 1;
        b.next = c.now + b.gap;
    } else {
        e.player[F(kTape)] += b.held;
        b.phase = 0;
        b.next = c.now + b.period;
    }
}

struct Race { size_t players = 2; uint32_t offsetMs = 0, gap = 25, frameMs = 16; int32_t stock = 3, chunk = 1; };

// the total the players ended up with (and the table's stock) after everybody ran the loop
int32_t benchRace(const Race& r, int32_t* tableLeft = nullptr) {
    Cluster c(static_cast<int>(r.players));
    corpse(c, I({{kTape, r.stock}}));
    std::vector<BenchBot> bots(r.players);
    for (size_t i = 0; i < r.players; ++i) {
        bots[i].who = i;
        bots[i].gap = r.gap + static_cast<uint32_t>(i) * 7;
        bots[i].chunk = r.chunk;
        bots[i].next = c.now + 100 + r.offsetMs * i;
    }
    for (int i = 0; i < 500; ++i) {
        c.now += r.frameMs;
        c.deliverDue();
        for (BenchBot& b : bots) stepBot(c, b);
        for (auto& mc : c.m) mc->sync->tick(c.now);
        c.deliverDue();
    }
    c.settle();
    if (tableLeft) *tableLeft = tableOf(c) ? count(tableOf(c)->items, kTape) : -1;
    return inPlayers(c, kTape);
}

}  // namespace

TEST(menuless, a_bench_style_grab_race_never_duplicates_or_loses_loot_whatever_the_timing) {
    int bad = 0, all = 0;
    for (size_t players : {2u, 3u})
        for (int32_t stock : {3, 8})
            for (int32_t chunk : {1, 3})
                for (uint32_t frame : {8u, 16u, 33u})
                    for (uint32_t off = 0; off < 100; off += 7)
                        for (uint32_t gap : {5u, 20u, 40u}) {
                            Race r;
                            r.players = players; r.stock = stock; r.chunk = chunk; r.frameMs = frame; r.offsetMs = off; r.gap = gap;
                            int32_t left = 0;
                            const int32_t got = benchRace(r, &left);
                            ++all;
                            if (got != stock || left != 0) {
                                if (++bad <= 8)
                                    std::printf("    race: %zu players, stock %d, chunk %d, frame %u ms, offset %u ms, gap %u ms -> players hold %d, table %d\n", players, stock, chunk, frame, off, gap, got, left);
                            }
                        }
    std::printf("    race: %d of %d timings duplicated or lost loot\n", bad, all);
    CHECK(bad == 0);
}

TEST(menuless, a_corpse_the_engine_never_listed_is_still_looked_at) {
    // the real engine lists only containers it described before; a corpse a death on ANOTHER machine registered is known from the shared table only
    Cluster c(2);
    corpse(c, I({{kTape, 3}}));
    Machine& g = c.machine(1);
    g.eng.listOnlyDescribed = true;
    g.eng.described.clear();
    CHECK(g.eng.silentTake(kCorpse, kTape, 2));
    c.settle();
    CHECK(g.sync->stats().transfersMenuless == 1);
    CHECK(count(tableOf(c)->items, kTape) == 1);
    CHECK(count(g.eng.player, kTape) == 2);
    // the host takes the last one while the guest grabs again: nobody ends up with more than the corpse held
    CHECK(c.host().eng.silentTake(kCorpse, kTape, 1));
    CHECK(g.eng.silentTake(kCorpse, kTape, 1));
    c.settle();
    CHECK(inPlayers(c, kTape) == 3);
    CHECK(count(tableOf(c)->items, kTape) == 0);
}

// Story fix 2026-10-28 (the lab's guest take): a placed chest nobody opened is announced by no cell attach, so no machine knew it: the guest took 1 of 3, its engine
// copy read 2 and the host's 3 for ever (the guest's service listed 0 containers, 0 polls). Every machine now looks for the containers of its player's cell.
TEST(menuless, a_placed_chest_nobody_opened_is_found_and_a_guests_take_from_it_adds_up) {
    Cluster c(2);
    for (auto& mc : c.m) { mc->eng.add(0x300, I({{kTape, 3}})); mc->eng.nearbyKnown = true; }   // no onContainerLoaded: nothing announced it
    c.run(8000);                                                                             // a few discovery passes, the resolves and the owner's grace time
    REQUIRE(c.host().sync->host()->container(0x300));
    CHECK(c.machine(1).sync->stats().containersDiscovered == 1 && c.host().sync->stats().containersDiscovered == 1);
    Machine& g = c.machine(1);
    CHECK(g.eng.silentTake(0x300, kTape, 1));
    c.settle();
    CHECK(count(g.eng.player, kTape) == 1);
    CHECK(count(c.host().sync->host()->container(0x300)->items, kTape) == 2);                // the host's table follows the guest's take
    for (auto& mc : c.m) CHECK(count(mc->eng.conts[0x300].items, kTape) == 2);               // and so does every engine copy
}

// Follow-ups 1, item 4: the scan covered the player's own cell only. A chest in a loaded neighbour cell of the exterior grid (Goodsprings' houses are cells of their own, the
// streets are the grid) is as lootable: every pass looks at the own cell and at a few neighbours in rotation, so the whole grid is covered within a few seconds.
TEST(menuless, the_loaded_neighbour_cells_of_an_exterior_grid_are_scanned_in_rotation) {
    Cluster c(2);
    for (auto& mc : c.m) {
        mc->eng.nearbyKnown = true;
        mc->eng.cellAware = true;
        mc->eng.playerCell = 7;
        mc->eng.neighbours = {20, 21, 22, 23, 24, 25, 26, 27};                       // eight attached neighbours
        mc->eng.add(0x300, I({{kTape, 3}}), lc::ContainerKind::Container, 7);         // the player's own cell
        for (NetId k = 0; k < 8; ++k) mc->eng.add(0x310 + k, I({{kTape, 1}}), lc::ContainerKind::Container, 20 + k);   // one chest in each neighbour
    }
    c.run(100);                                                                       // the first pass (then one every 1.5 s)
    Machine& g = c.machine(1);
    CHECK(g.sync->stats().containersDiscovered == 1 + 3);                              // the own cell and the first three neighbours
    CHECK(g.eng.cellScans == 3);                                                       // a pass is bounded: three neighbour cells, not the whole grid
    c.run(1500 * 3);                                                                   // the next passes cover the other five (the cursor moves on, it wraps around)
    CHECK(g.sync->stats().containersDiscovered == 9);
    for (NetId k = 0; k < 8; ++k) CHECK(g.sync->client()->container(0x310 + k) != nullptr);
    // a take from a chest next door is seen and adds up on every machine
    c.settle();
    CHECK(g.eng.silentTake(0x313, kTape, 1));
    c.settle();
    CHECK(count(g.eng.player, kTape) == 1);
    for (auto& mc : c.m) CHECK(count(mc->eng.conts[0x313].items, kTape) == 0);
    // an interior has no neighbours: only the own cell is asked
    Cluster in(2);
    for (auto& mc : in.m) { mc->eng.nearbyKnown = true; mc->eng.cellAware = true; mc->eng.add(0x300, I({{kTape, 3}}), lc::ContainerKind::Container, 7); mc->eng.add(0x301, I({{kTape, 1}}), lc::ContainerKind::Container, 40); }
    in.run(4000);
    CHECK(in.machine(1).sync->stats().containersDiscovered == 1 && in.machine(1).eng.cellScans == 0);
    // switched off (discover_cells=0): the own cell alone, as before
    SyncConfig offCfg = Cluster::defaultCfg();
    offCfg.discoverCells = 0;
    Cluster off(2, offCfg);
    for (auto& mc : off.m) { mc->eng.nearbyKnown = true; mc->eng.cellAware = true; mc->eng.neighbours = {20}; mc->eng.add(0x300, I({{kTape, 3}}), lc::ContainerKind::Container, 7); mc->eng.add(0x310, I({{kTape, 1}}), lc::ContainerKind::Container, 20); }
    off.run(4000);
    CHECK(off.machine(1).sync->stats().containersDiscovered == 1);
}

TEST(menuless, a_remote_take_applied_before_the_scan_does_not_hide_the_local_one) {
    // the failure of the benchmark: the host's take reaches the guest after the guest's own RemoveItem and before its AddItem; the shared contents then equal
    // the guest's engine copy and the player's gain, when it comes, has nothing left to be matched with
    Cluster c(2);
    corpse(c, I({{kTape, 3}}));
    Machine& g = c.machine(1);
    Machine& h = c.host();
    CHECK(g.eng.silentTake(kCorpse, kTape, 1, false));    // RemoveItem on the guest ...
    CHECK(h.eng.silentTake(kCorpse, kTape, 1));           // ... the host takes one as well (RemoveItem + AddItem)
    c.run(100);                                           // the host's take is in the table and was applied on the guest, which has not finished its own yet
    g.eng.player[F(kTape)] += 1;                          // the guest's AddItem
    c.settle();
    CHECK(inPlayers(c, kTape) == 2);                      // two takes, two tapes in the players' hands
    CHECK(count(tableOf(c)->items, kTape) == 1);
    for (auto& mc : c.m) CHECK(count(mc->eng.conts[kCorpse].items, kTape) == 1);
}

TEST(menuless, the_status_counts_what_the_transfer_detector_saw_arbitrated_and_took_back) {
    Cluster c(3);
    corpse(c, I({{kTape, 3}}));
    Machine& holder = c.machine(2);
    holder.eng.openMenuOn(kCorpse);
    c.run(300);
    Machine& g = c.machine(1);
    CHECK(g.eng.silentTake(kCorpse, kTape, 2));
    c.run(900);
    const std::string j = g.sync->statusJson();
    CHECK(j.find("\"menuless\":{\"enabled\":true") != std::string::npos);
    CHECK(g.sync->stats().transferScans > 0);
    CHECK(g.sync->stats().transferDiffs >= 1);            // the detector saw the engine copy differ
    CHECK(g.sync->stats().transfersMenuless == 1);        // and arbitrated it
    CHECK(g.sync->stats().revertedItems == 2);            // the host refused (the corpse is open elsewhere): two tapes taken back
    CHECK(j.find("\"arbitrated\":1") != std::string::npos);
    CHECK(j.find("\"revertedItems\":2") != std::string::npos);
    CHECK(j.find("\"seen\":") != std::string::npos);
}

// ---- the same race with the container MENU: both open the corpse and take one tape every 150 ms until their menu or the corpse is empty
namespace {

int32_t menuRace(size_t players, uint32_t offsetMs, uint32_t frameMs, bool closeWorks, int32_t stock, int32_t* tableLeft) {
    Cluster c(static_cast<int>(players));
    corpse(c, I({{kTape, stock}}));
    for (auto& mc : c.m) mc->eng.closeWorks = closeWorks;
    std::vector<uint64_t> next(players);
    std::vector<bool> opened(players, false), done(players, false);
    for (size_t i = 0; i < players; ++i) next[i] = c.now + 100 + offsetMs * i;
    for (int i = 0; i < 400; ++i) {
        c.now += frameMs;
        c.deliverDue();
        for (size_t p = 0; p < players; ++p) {
            if (done[p] || c.now < next[p]) continue;
            FakeEngine& e = c.machine(p).eng;
            if (!opened[p]) { e.openMenuOn(kCorpse); opened[p] = true; next[p] = c.now + 150; continue; }
            if (e.menu != e.conts[kCorpse].ref || count(e.conts[kCorpse].items, kTape) <= 0) { if (e.menu == e.conts[kCorpse].ref) e.menu = 0; done[p] = true; continue; }
            e.take(kCorpse, kTape, 1);
            next[p] = c.now + 150;
        }
        for (auto& mc : c.m) mc->sync->tick(c.now);
        c.deliverDue();
    }
    for (auto& mc : c.m) mc->eng.menu = 0;
    c.settle();
    if (tableLeft) *tableLeft = tableOf(c) ? count(tableOf(c)->items, kTape) : -1;
    return inPlayers(c, kTape);
}

}  // namespace

TEST(menuless, a_menu_race_for_the_same_corpse_hands_every_item_out_once) {
    int bad = 0, all = 0;
    for (size_t players : {2u, 3u})
        for (bool closes : {true, false})
            for (uint32_t frame : {8u, 16u, 33u})
                for (uint32_t off = 0; off < 160; off += 9) {
                    int32_t left = 0;
                    const int32_t got = menuRace(players, off, frame, closes, 3, &left);
                    ++all;
                    if (got != 3 || left != 0) {
                        if (++bad <= 8) std::printf("    menu race: %zu players, menu %s, frame %u, offset %u -> players hold %d, table %d\n", players, closes ? "closes" : "stays", frame, off, got, left);
                    }
                }
    std::printf("    menu race: %d of %d timings duplicated or lost loot\n", bad, all);
    CHECK(bad == 0);
}
