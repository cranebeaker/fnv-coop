// Entity identity and the EntityMap (docs/entity-core-plan.md step S1). The scenarios are functions so the mutation twins at the bottom run
// the very same checks against a broken map.
#include <set>
#include <string>
#include <vector>

#include "entity_map.h"
#include "minitest.h"

using namespace fnvcoop;
using namespace fnvcoop::live;

namespace {

struct Seen { EntityNetId id; uint32_t ref; EntityRole role; bool bound; };

bool rangesAreDisjoint() {
    bool ok = true;
    for (PlayerId s = 0; s < kMaxSlots; ++s) {
        const EntityNetId id = playerNet(s);
        ok = ok && isPlayerNet(id) && slotOfPlayerNet(id) == s && kindOf(id) == EntityKind::Player;
    }
    ok = ok && kindOf(0) == EntityKind::None && kindOf(1) == EntityKind::Placed && kindOf(0xFDFFFFFFu) == EntityKind::Placed;
    ok = ok && kindOf(0xFF000000u) == EntityKind::Created && kindOf(0xFFFFFFFFu) == EntityKind::Created;
    ok = ok && kindOf(0xFE000100u) == EntityKind::None && !isPlayerNet(0xFE000100u) && !isPlayerNet(0xFD0000FFu);
    return ok;
}

bool placedIdsNeverLandOnAPlayerOrACreatedId() {
    // every top byte, with low bytes that look like a player slot
    for (uint32_t top = 0; top < 256; ++top)
        for (uint32_t low : {0u, 1u, 7u, 0xFFu, 0x12345u}) {
            const EntityNetId id = maskPlacedNetId((top << 24) | low);
            if (kindOf(id) != EntityKind::Placed) return false;
        }
    return maskPlacedNetId(0) == 1u;
}

bool localSlotIsThePlayerRef() {
    EntityMap m;
    m.setLocalSlot(2);
    bool ok = m.refOf(playerNet(2)) == kLocalPlayerRef && m.netOf(kLocalPlayerRef) == playerNet(2);
    ok = ok && m.roleOf(playerNet(2)) == EntityRole::LocalPlayer && m.slotOfRef(kLocalPlayerRef) == 2 && m.actingRef(2) == kLocalPlayerRef;
    m.setLocalSlot(1);   // a rejoin into another slot: the old one is gone
    ok = ok && m.refOf(playerNet(2)) == 0 && m.refOf(playerNet(1)) == kLocalPlayerRef && m.slotOfRef(kLocalPlayerRef) == 1;
    m.setLocalSlot(kNoPlayer);
    return ok && m.netOf(kLocalPlayerRef) == 0 && m.actingRef(1) == 0;
}

bool unboundRefsAnswerNothing() {
    EntityMap m;
    bool ok = m.bind(playerNet(1), 0x0100AAAA, EntityRole::Puppet);
    ok = ok && m.slotOfRef(0x0100AAAA) == 1 && m.actingRef(1) == 0x0100AAAA;
    m.unbind(playerNet(1));   // parked: the engine may reuse that runtime id for anything
    ok = ok && m.netOf(0x0100AAAA) == 0 && m.slotOfRef(0x0100AAAA) == kNoPlayer && m.actingRef(1) == 0;
    ok = ok && m.bind(0x00123456, 0x0100AAAA, EntityRole::Npc);   // the reused id now belongs to an NPC
    ok = ok && m.netOf(0x0100AAAA) == 0x00123456 && m.slotOfRef(0x0100AAAA) == kNoPlayer;
    m.unbindRef(0x0100AAAA);
    return ok && m.size() == 0 && m.netOf(0x0100AAAA) == 0;
}

bool aRefBoundToAnotherEntityIsRefused() {
    EntityMap m;
    bool ok = m.bind(playerNet(1), 0x0100AAAA, EntityRole::Puppet);
    ok = ok && !m.bind(0x00123456, 0x0100AAAA, EntityRole::Npc);
    ok = ok && m.netOf(0x0100AAAA) == playerNet(1) && m.refOf(0x00123456) == 0 && m.stats().conflicts == 1;
    ok = ok && !m.bind(0, 5, EntityRole::Npc) && !m.bind(5, 0, EntityRole::Npc) && !m.bind(5, 6, EntityRole::None);
    return ok;
}

bool rebindMovesTheEntityAndBumpsItsGeneration() {
    EntityMap m;
    std::vector<Seen> seen;
    m.onChange([&](EntityNetId id, uint32_t ref, EntityRole role, bool bound) { seen.push_back({id, ref, role, bound}); });
    bool ok = m.bind(playerNet(3), 0x0100AAAA, EntityRole::Puppet);
    const uint32_t g1 = m.binding(playerNet(3))->generation;
    ok = ok && m.bind(playerNet(3), 0x0100AAAA, EntityRole::Puppet) && seen.size() == 1;   // the same again: nothing to tell
    ok = ok && m.bind(playerNet(3), 0x0100BBBB, EntityRole::Puppet);                        // the puppet was respawned
    ok = ok && m.netOf(0x0100AAAA) == 0 && m.refOf(playerNet(3)) == 0x0100BBBB && m.binding(playerNet(3))->generation > g1;
    ok = ok && seen.size() == 3 && !seen[1].bound && seen[1].ref == 0x0100AAAA && seen[2].bound && seen[2].ref == 0x0100BBBB;
    m.unbind(playerNet(3));
    ok = ok && m.bind(playerNet(3), 0x0100AAAA, EntityRole::Puppet) && m.binding(playerNet(3))->generation > g1 + 1;
    return ok && m.stats().rebinds == 1;
}

bool listenersHearEveryChangeInOrder() {
    EntityMap m;
    std::vector<Seen> seen;
    const size_t t = m.onChange([&](EntityNetId id, uint32_t ref, EntityRole role, bool bound) { seen.push_back({id, ref, role, bound}); });
    m.setLocalSlot(0);
    m.bind(playerNet(1), 0x0100AAAA, EntityRole::Puppet);
    m.bind(0x00123456, 0x0100CCCC, EntityRole::StandIn);
    m.clear();
    bool ok = seen.size() == 6;
    ok = ok && seen.size() >= 3 && seen[0].bound && seen[0].role == EntityRole::LocalPlayer && seen[1].role == EntityRole::Puppet && seen[2].role == EntityRole::StandIn;
    for (size_t i = 3; i < seen.size(); ++i) ok = ok && !seen[i].bound;
    ok = ok && m.size() == 0 && m.localSlot() == kNoPlayer;
    m.removeListener(t);
    m.bind(playerNet(1), 0x0100AAAA, EntityRole::Puppet);
    return ok && seen.size() == 6;
}

bool onlyPlayersHaveASlot() {
    EntityMap m;
    m.bind(0x00123456, 0x0100CCCC, EntityRole::Npc);
    m.bind(0x00123457, 0x0100DDDD, EntityRole::StandIn);
    m.bind(playerNet(4), 0x0100EEEE, EntityRole::Puppet);
    return m.slotOfRef(0x0100CCCC) == kNoPlayer && m.slotOfRef(0x0100DDDD) == kNoPlayer && m.slotOfRef(0x0100EEEE) == 4 && m.actingRef(9) == 0;
}

bool playerFormRefsRoundTrip() {
    bool ok = true;
    for (PlayerId s = 0; s < kMaxSlots; ++s) ok = ok && isPlayerFormRef(playerFormRef(s)) && slotOfPlayerFormRef(playerFormRef(s)) == s;
    ok = ok && !isPlayerFormRef(FormRef("falloutnv.esm", 0x14)) && !isPlayerFormRef(FormRef(kPlayerPlugin, kMaxSlots));
    std::set<FormRef> refs;
    for (PlayerId s = 0; s < kMaxSlots; ++s) refs.insert(playerFormRef(s));
    return ok && refs.size() == kMaxSlots && slotOfPlayerFormRef(FormRef("x.esp", 1)) == kNoPlayer;
}

}  // namespace

TEST(entity, ranges_are_disjoint) { CHECK(rangesAreDisjoint()); }
TEST(entity, a_placed_id_never_lands_on_a_player_or_a_created_id) { CHECK(placedIdsNeverLandOnAPlayerOrACreatedId()); }
TEST(entity, the_local_slot_is_the_player_ref) { CHECK(localSlotIsThePlayerRef()); }
TEST(entity, an_unbound_ref_answers_nothing_even_when_the_engine_reuses_it) { CHECK(unboundRefsAnswerNothing()); }
TEST(entity, a_ref_bound_to_another_entity_is_refused) { CHECK(aRefBoundToAnotherEntityIsRefused()); }
TEST(entity, a_rebind_moves_the_entity_and_bumps_its_generation) { CHECK(rebindMovesTheEntityAndBumpsItsGeneration()); }
TEST(entity, listeners_hear_every_change_in_order) { CHECK(listenersHearEveryChangeInOrder()); }
TEST(entity, only_players_have_a_slot) { CHECK(onlyPlayersHaveASlot()); }
TEST(entity, player_form_refs_round_trip) { CHECK(playerFormRefsRoundTrip()); }

#define ENTITY_TWIN(name, flag, scenario) \
    TEST(twin, name) { EntityFaults f; f.flag = true; { EntityFaultScope s(f); CHECK(!(scenario)); } }

ENTITY_TWIN(entity_stale_after_unbind, staleAfterUnbind, unboundRefsAnswerNothing())
ENTITY_TWIN(entity_accept_conflict, acceptConflict, aRefBoundToAnotherEntityIsRefused())
ENTITY_TWIN(entity_no_local_player, noLocalPlayer, localSlotIsThePlayerRef())
ENTITY_TWIN(entity_puppet_is_local, puppetIsLocal, onlyPlayersHaveASlot())
ENTITY_TWIN(entity_no_change_events, noChangeEvents, listenersHearEveryChangeInOrder())
