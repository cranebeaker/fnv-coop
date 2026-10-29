// Tests for the identity layer (plugin/hooks/identity.h, hooks_world/world_identity.h): one translation for every effect that arrives from another player's
// machine. Included by test_world.cpp. Twins 67 to 80 (identity.h, world_identity.h): each must fail one of the tests below.
//
// The scene is the 2026-10-29 bug: this machine is guest slot 2, the host (slot 0) names its own player 0x14 in everything it sends, and 0x14 here is
// the guest's own character.
#pragma once

#include <functional>
#include <utility>
#include <variant>

#include "identity.h"
#include "ref_ops.h"
#include "world_identity.h"

namespace idt {

using fnvcoop::Effect;
using fnvcoop::PlayerId;
using nvhooks::Verdict;

constexpr PlayerId kHost = 0, kOther = 1, kMe = 2;

inline FormRef p0() { return esm(0x14); }
inline FormRef npc() { return esm(0x1234); }
// How the host's rules name THIS guest's player (a host-side ghost ref): never the P0 spelling.
inline FormRef myWire() { return created(0x10); }

inline nvhooks::IdentityView guest(bool withSelfWire = true) {
    nvhooks::IdentityView v;
    v.on = true;
    v.self = kMe;
    v.p0 = p0();
    if (withSelfWire) v.selfWire = myWire();
    v.opDefaultsToPlayer = &nvhooks::refOpDefaultsToPlayer;
    return v;
}

// Runs one story effect through the layer; `out` gets the rewritten effect.
inline Verdict story(Effect e, PlayerId sender, Effect* out, const nvhooks::IdentityView& v = guest()) {
    const Verdict r = nvhooks::translateIncoming(e, sender, v);
    if (out) *out = e;
    return r;
}
inline Verdict world(WorldEffect e, PlayerId sender, WorldEffect* out = nullptr, const nvhooks::IdentityView& v = guest()) {
    const Verdict r = translateWorldIncoming(e, sender, v);
    if (out) *out = e;
    return r;
}

inline fnvcoop::RefCommand cmd(uint16_t op, FormRef ref, fnvcoop::Recipient to, FormRef a = FormRef()) {
    fnvcoop::RefCommand c;
    c.op = op;
    c.ref = std::move(ref);
    c.a = std::move(a);
    c.to = to;
    return c;
}

template <size_t... I>
inline void eachEffect(std::index_sequence<I...>, const std::function<void(const Effect&)>& f) {
    (f(Effect(std::variant_alternative_t<I, Effect>{})), ...);
}
template <size_t... I>
inline void eachWorldEffect(std::index_sequence<I...>, const std::function<void(const WorldEffect&)>& f) {
    (f(WorldEffect(std::variant_alternative_t<I, WorldEffect>{})), ...);
}

}  // namespace idt

TEST(identity_table_names_every_effect_type) {
    // A new Effect or WorldEffect alternative must be looked at: it is either in the rows (it has a field that can name a player) or in the no-player list.
    auto inRows = [](const nvhooks::IdentityRow* rows, size_t n, const std::string& name) {
        for (size_t i = 0; i < n; ++i) if (name == rows[i].effect) return true;
        return false;
    };
    auto inList = [](const char* const* list, size_t n, const std::string& name) {
        for (size_t i = 0; i < n; ++i) if (name == list[i]) return true;
        return false;
    };
    std::set<std::string> seen;
    idt::eachEffect(std::make_index_sequence<std::variant_size_v<fnvcoop::Effect>>{}, [&](const fnvcoop::Effect& e) {
        const std::string name = nvhooks::effectTypeName(e);
        CHECK(name != "?");
        const bool row = inRows(nvhooks::kStoryRows, sizeof nvhooks::kStoryRows / sizeof nvhooks::kStoryRows[0], name);
        const bool none = inList(nvhooks::kStoryNoPlayerRef, sizeof nvhooks::kStoryNoPlayerRef / sizeof nvhooks::kStoryNoPlayerRef[0], name);
        CHECK(row != none);   // exactly one of the two
        seen.insert(name);
    });
    CHECK(seen.size() == std::variant_size_v<fnvcoop::Effect>);
    for (const auto& r : nvhooks::kStoryRows) CHECK(seen.count(r.effect) == 1);                    // no row for an effect that does not exist
    for (const char* n : nvhooks::kStoryNoPlayerRef) CHECK(seen.count(n) == 1);
    std::set<std::string> wseen;
    idt::eachWorldEffect(std::make_index_sequence<std::variant_size_v<WorldEffect>>{}, [&](const WorldEffect& e) {
        const std::string name = worldEffectName(e);
        const bool row = inRows(kWorldRows, sizeof kWorldRows / sizeof kWorldRows[0], name);
        const bool none = inList(kWorldNoPlayerRef, sizeof kWorldNoPlayerRef / sizeof kWorldNoPlayerRef[0], name);
        CHECK(row != none);
        wseen.insert(name);
    });
    CHECK(wseen.size() == std::variant_size_v<WorldEffect>);
    for (const auto& r : kWorldRows) CHECK(wseen.count(r.effect) == 1);
    for (const char* n : kWorldNoPlayerRef) CHECK(wseen.count(n) == 1);
    // a field with no row is dropped, never applied to the wrong player
    CHECK(nvhooks::storyAct("EnableRef", "nonsense") == nvhooks::PlayerRef::Drop);
}

TEST(identity_story_sender_player_is_dropped_where_it_is_the_senders_own_state) {
    using namespace idt;
    const Effect drop[] = {fnvcoop::EnableRef{p0()}, fnvcoop::DisableRef{p0()}, fnvcoop::KillActor{p0()}, fnvcoop::ResurrectActor{p0()},
                           fnvcoop::SetScriptVar{p0(), 3, 1.0}, fnvcoop::CompanionOwner{p0()}, fnvcoop::ForcedDialogue{p0(), npc()},
                           fnvcoop::SetLock{p0()}, fnvcoop::SwapTexture{p0(), npc()}, fnvcoop::ObjectState{p0()}, fnvcoop::RefScriptVar{p0(), 1, 2.0},
                           fnvcoop::RefAddItem{p0(), npc(), 1}, fnvcoop::RefRemoveItem{p0(), npc(), 1}, fnvcoop::RemoveAllItems{p0()},
                           fnvcoop::RefAngle{p0()}, fnvcoop::MoveRef{p0(), esm(0x777), 1, 2, 3, false}};
    for (const Effect& e : drop) {
        Effect out;
        CHECK(story(e, kHost, &out) == Verdict::Dropped);                // the host's player is not this machine's player
        CHECK(story(e, kOther, &out) == Verdict::Dropped);               // nor is another guest's
        CHECK(story(e, kMe, &out) == Verdict::Unchanged);                // but my own effects name MY player: 0x14 is right
    }
    // the same effects about anyone else are untouched
    const Effect other[] = {fnvcoop::EnableRef{npc()}, fnvcoop::KillActor{npc()}, fnvcoop::SetLock{npc()}, fnvcoop::MoveRef{npc(), esm(0x777), 1, 2, 3, false},
                            fnvcoop::RefScriptVar{npc(), 1, 2.0}, fnvcoop::RefAngle{npc()}};
    for (const Effect& e : other) {
        Effect out;
        CHECK(story(e, kHost, &out) == Verdict::Unchanged);
        CHECK(out == e);
    }
    // the counter sees the drops
    const uint64_t before = nvhooks::identityStats().dropped;
    Effect out;
    CHECK(story(fnvcoop::KillActor{p0()}, kHost, &out) == Verdict::Dropped);
    CHECK(nvhooks::identityStats().dropped == before + 1);
}

TEST(identity_story_sender_player_as_target_becomes_its_puppet) {
    using namespace idt;
    // `Actor.MoveTo player`: the cell field carries the anchor marker; the mover is somebody else
    Effect out;
    CHECK(story(fnvcoop::MoveRef{npc(), p0(), 10, 0, 0, false}, kHost, &out) == Verdict::Rewritten);
    {
        const auto& m = std::get<fnvcoop::MoveRef>(out);
        CHECK(m.ref == npc());
        CHECK(m.cell == nvhooks::puppetRef(kHost));
        CHECK(m.x == 10);
    }
    CHECK(story(fnvcoop::MoveRef{npc(), p0(), 10, 0, 0, false}, kOther, &out) == Verdict::Rewritten);
    CHECK(std::get<fnvcoop::MoveRef>(out).cell == nvhooks::puppetRef(kOther));   // another guest's player is ITS puppet, not the host's
    // the character's faction membership is read by the AI of the NPCs on this machine, so it lands on the puppet
    CHECK(story(fnvcoop::SetFactionRank{p0(), esm(0x2222), 1}, kHost, &out) == Verdict::Rewritten);
    CHECK(std::get<fnvcoop::SetFactionRank>(out).actor == nvhooks::puppetRef(kHost));
    CHECK(std::get<fnvcoop::SetFactionRank>(out).faction == esm(0x2222));
    CHECK(story(fnvcoop::SetFactionRank{p0(), esm(0x2222), 1}, kMe, &out) == Verdict::Unchanged);   // my own: my player
    CHECK(story(fnvcoop::SetFactionRank{npc(), esm(0x2222), 1}, kHost, &out) == Verdict::Unchanged);
    CHECK(nvhooks::identityStats().toPuppet > 0);
}

TEST(identity_story_effect_addressed_by_the_slots_wire_reference_lands_on_my_player) {
    using namespace idt;
    // a party warp names every slot's player by the reference the session gives it; for this guest that is not 0x14 (the host's ghost of it)
    Effect out;
    CHECK(story(fnvcoop::MoveRef{myWire(), esm(0x777), 0, 0, 0, true}, kHost, &out) == Verdict::Rewritten);   // twin 73
    CHECK(std::get<fnvcoop::MoveRef>(out).ref == p0());
    CHECK(std::get<fnvcoop::MoveRef>(out).partyWarp);
    CHECK(story(fnvcoop::MoveRef{myWire(), esm(0x777), 0, 0, 0, true}, kHost, &out, guest(false)) == Verdict::Unchanged);   // slot ref unknown: left alone
    // the host's own warp entry in the same expansion is the host's stream, not mine
    CHECK(story(fnvcoop::MoveRef{p0(), esm(0x777), 0, 0, 0, true}, kHost, &out) == Verdict::Dropped);
}

TEST(identity_refcommand_data_on_the_player_is_kept_only_when_addressed_to_me) {
    using namespace idt;
    using namespace fnvcoop;
    Effect out;
    // rewards the rules routed to this player: AddSpell, a conditioned item, a faction rank delta, a character-build call
    const uint16_t data[] = {kOpAddSpell, kOpRemoveSpell, kOpEquipItem, kOpAddItemHealthPercent, kOpModFactionRank, kOpSetLevel, kOpSetPlayerTagSkill, kOpWakeUpPC};
    for (uint16_t op : data) {
        CHECK(story(cmd(op, p0(), Recipient::slot(kMe)), kHost, &out) == Verdict::Unchanged);          // twin 72: never arrives
        CHECK(std::get<RefCommand>(out).ref == p0());                                                 // it is MY player: P0 resolves to 0x14 here
        CHECK(story(cmd(op, p0(), Recipient::all()), kHost, &out) == Verdict::Unchanged);
        CHECK(story(cmd(op, p0(), Recipient::slot(kOther)), kHost, &out) == Verdict::Dropped);         // twin 78: addressed to somebody else
        CHECK(story(cmd(op, p0(), Recipient::dflt()), kHost, &out) == Verdict::Dropped);              // not addressed at all
        CHECK(story(cmd(op, p0(), Recipient::slot(kHost)), kHost, &out) == Verdict::Dropped);
    }
    // the sender's body: animations, AI orders, flags of its own actor, however the command was routed
    const uint16_t body[] = {kOpPlayIdle, kOpPlayGroup, kOpEvaluatePackage, kOpStartCombat, kOpLook, kOpSetUnconscious, kOpSetGhost, kOpFireWeapon, kOpUseWeapon, kOpRotate, kOpPushActorAway,
                             kOpSetActorAlpha, kOpResetAI, kOpSetVATSTarget};
    for (uint16_t op : body) {
        CHECK(story(cmd(op, p0(), Recipient::all()), kHost, &out) == Verdict::Dropped);                // twin 71: the host's wake-up animation plays on my character
        CHECK(story(cmd(op, p0(), Recipient::slot(kMe)), kHost, &out) == Verdict::Dropped);
        CHECK(story(cmd(op, p0(), Recipient::all()), kMe, &out) == Verdict::Unchanged);                // my own
    }
}

TEST(identity_refcommand_arguments_naming_the_player_become_its_puppet) {
    using namespace idt;
    using namespace fnvcoop;
    Effect out;
    // an NPC ordered to fight the host's player: the NPC is somebody's actor, the target is the host's puppet here
    CHECK(story(cmd(kOpStartCombat, npc(), Recipient::all(), p0()), kHost, &out) == Verdict::Rewritten);
    CHECK(std::get<RefCommand>(out).ref == npc());
    CHECK(std::get<RefCommand>(out).a == nvhooks::puppetRef(kHost));
    CHECK(story(cmd(kOpStartCombat, npc(), Recipient::all(), p0()), kOther, &out) == Verdict::Rewritten);
    CHECK(std::get<RefCommand>(out).a == nvhooks::puppetRef(kOther));
    CHECK(story(cmd(kOpStartCombat, npc(), Recipient::all(), p0()), kMe, &out) == Verdict::Unchanged);   // my own order: my player is the target
    // the wide shape: UseWeapon has four form slots
    RefCommand uw = cmd(kOpUseWeapon, npc(), Recipient::all(), esm(0x55));
    uw.b = npc();
    uw.c = p0();
    uw.d = myWire();
    CHECK(story(uw, kHost, &out) == Verdict::Rewritten);
    CHECK(std::get<RefCommand>(out).c == nvhooks::puppetRef(kHost));
    CHECK(std::get<RefCommand>(out).d == p0());                      // the slot reference of this player: me
    CHECK(std::get<RefCommand>(out).a == esm(0x55));
    // an unrelated command is untouched
    const RefCommand plain = cmd(kOpAddSpell, npc(), Recipient::all(), esm(0x66));
    CHECK(story(plain, kHost, &out) == Verdict::Unchanged);
    CHECK(std::get<RefCommand>(out) == plain);
}

TEST(identity_refcommand_without_a_reference_runs_on_the_player) {
    using namespace idt;
    using namespace fnvcoop;
    Effect out;
    // SetLevel, ResetXP, AgeRace, SetHardcore, AddCardToPlayer run on the player when the script names no reference (kRoPlayerRef)
    CHECK(nvhooks::refOpDefaultsToPlayer(kOpSetLevel));
    CHECK(nvhooks::refOpDefaultsToPlayer(kOpAddCardToPlayer));
    CHECK(!nvhooks::refOpDefaultsToPlayer(kOpAddSpell));
    CHECK(!nvhooks::refOpDefaultsToPlayer(kOpNone));
    CHECK(story(cmd(kOpSetLevel, FormRef(), Recipient::slot(kMe)), kHost, &out) == Verdict::Unchanged);    // addressed to me: my player, still implicit
    CHECK(std::get<RefCommand>(out).ref.plugin.empty());
    CHECK(story(cmd(kOpSetLevel, FormRef(), Recipient::dflt()), kHost, &out) == Verdict::Dropped);          // twin 76: an unaddressed implicit player is the host's
    CHECK(story(cmd(kOpResetXP, FormRef(), Recipient::slot(kOther)), kHost, &out) == Verdict::Dropped);
    CHECK(story(cmd(kOpSetLevel, FormRef(), Recipient::dflt()), kMe, &out) == Verdict::Unchanged);
    CHECK(std::get<RefCommand>(out).ref.plugin.empty());
    // a command with no reference that does not default to the player has no player in it
    CHECK(story(cmd(kOpSetQuestDelay, FormRef(), Recipient::dflt(), esm(0x99)), kHost, &out) == Verdict::Unchanged);
}

TEST(identity_off_changes_nothing) {
    using namespace idt;
    nvhooks::IdentityView off = guest();
    off.on = false;
    Effect out;
    CHECK(story(fnvcoop::KillActor{p0()}, kHost, &out, off) == Verdict::Unchanged);        // the host: its player IS 0x14
    CHECK(std::get<fnvcoop::KillActor>(out).actor == p0());
    CHECK(world(SetScriptPackage{p0(), esm(0x1055C4), true}, kHost, nullptr, off) == Verdict::Unchanged);
    nvhooks::resetIdentity();
    CHECK(!nvhooks::identityActive());
    CHECK(!nvhooks::currentIdentity().on);
    FakeResolver inner;
    CHECK(nvhooks::IdentityResolver(inner).fromRef(nvhooks::puppetRef(0)) == 0);   // no puppets known
}

TEST(identity_view_is_built_from_the_machines_hooks) {
    nvhooks::resetIdentity();
    nvhooks::IdentityHooks h;
    h.self = [] { return idt::kMe; };
    h.wireRefOf = [](fnvcoop::PlayerId s, FormRef* out) { *out = s == idt::kMe ? idt::myWire() : esm(0x14); return true; };
    nvhooks::configureIdentity(h);
    nvhooks::IdentityView v = nvhooks::currentIdentity();
    CHECK(v.on);
    CHECK(v.self == idt::kMe);
    CHECK(v.p0 == esm(0x14));
    CHECK(v.selfWire == idt::myWire());
    // a session that names this player P0 gives no separate wire reference
    h.wireRefOf = [](fnvcoop::PlayerId, FormRef* out) { *out = esm(0x14); return true; };
    nvhooks::configureIdentity(h);
    CHECK(nvhooks::currentIdentity().selfWire.plugin.empty());
    // before the guest knows its slot, the host is still somebody else
    h.self = [] { return fnvcoop::kNoPlayer; };
    nvhooks::configureIdentity(h);
    v = nvhooks::currentIdentity();
    CHECK(v.on && v.self == fnvcoop::kNoPlayer);
    fnvcoop::Effect e = fnvcoop::KillActor{esm(0x14)};
    v.opDefaultsToPlayer = &nvhooks::refOpDefaultsToPlayer;
    CHECK(nvhooks::translateIncoming(e, idt::kHost, v) == nvhooks::Verdict::Dropped);
    nvhooks::resetIdentity();
}

TEST(identity_local_slot_decides_whose_companion_it_is) {
    using namespace idt;
    // CompanionOwner{owner} is applied by the machine of that slot only. The applier's own slot stayed 0 in the game, so guest 2 recruited the host's companion and
    // refused its own.
    const nvhooks::IdentityView g = guest();
    CHECK(nvhooks::localSlotOf(g, 0) == kMe);
    CHECK(nvhooks::companionForMe(kMe, nvhooks::localSlotOf(g, 0)));
    CHECK(!nvhooks::companionForMe(kHost, nvhooks::localSlotOf(g, 0)));
    nvhooks::IdentityView host = g;
    host.on = false;                                           // the host never configures the layer
    CHECK(nvhooks::localSlotOf(host, 0) == 0);
    CHECK(nvhooks::companionForMe(kHost, nvhooks::localSlotOf(host, 0)));
    nvhooks::IdentityView unknown = g;
    unknown.self = fnvcoop::kNoPlayer;                          // a guest not yet told its slot falls back to the applier's value
    CHECK(nvhooks::localSlotOf(unknown, 7) == 7);
}

TEST(identity_resolver_maps_the_placeholders_to_the_puppets) {
    nvhooks::resetIdentity();
    nvhooks::IdentityHooks h;
    h.puppetOf = [](fnvcoop::PlayerId s) -> uint32_t { return s == 0 ? 0xFF000A00u : s == 1 ? 0xFF000B00u : 0u; };
    nvhooks::configureIdentity(h);
    FakeResolver inner;
    nvhooks::IdentityResolver r(inner);
    CHECK(r.fromRef(nvhooks::puppetRef(0)) == 0xFF000A00u);
    CHECK(r.fromRef(nvhooks::puppetRef(1)) == 0xFF000B00u);   // twin 77: every slot would answer the host's puppet
    CHECK(r.fromRef(nvhooks::puppetRef(2)) == 0);              // no puppet (yet): the form is missing, the apply reports it
    CHECK(r.fromRef(esm(0x77)) == 0x77);                       // everything else is the inner resolver's
    CHECK(r.fromRef(FormRef(nvhooks::puppetPlugin(), 9)) == 0);   // not a slot
    FormRef f;
    CHECK(r.toRef(0x1234, &f) && f == esm(0x1234));
    CHECK(r.lookup(0x55) == inner.lookup(0x55));
    fnvcoop::PlayerId s = 99;
    CHECK(nvhooks::isPuppetRef(nvhooks::puppetRef(3), &s) && s == 3);
    CHECK(!nvhooks::isPuppetRef(esm(3)));
    nvhooks::resetIdentity();
}

TEST(identity_world_effects_of_the_sender_player_are_dropped) {
    using namespace idt;
    // the 2026-10-29 case, as it comes off the wire: VCG01's package on the waking host player
    const WorldEffect vcg01 = SetScriptPackage{esm(0x14), esm(0x1055C4), true};
    WorldEffect wire;
    std::string err;
    CHECK(worldEffectFromJson(worldEffectToJson(vcg01), &wire, &err));
    CHECK(world(wire, kHost) == Verdict::Dropped);                 // twins 67, 70, 75
    const WorldEffect drop[] = {SetActorValueRef{esm(0x14), 5, 10.f, AvMode::Set}, SetEssential{esm(0x14), true, true}, SetRestrained{esm(0x14), true},
                                SetScriptPackage{esm(0x14), esm(0x1055C4), false}, SetIgnoreCrime{esm(0x14), true},
                                SetObjectState{esm(0x14), 1, true, false, 0, false}};
    for (const WorldEffect& e : drop) {
        CHECK(world(e, kHost) == Verdict::Dropped);
        CHECK(world(e, kOther) == Verdict::Dropped);
        CHECK(world(e, kMe) == Verdict::Unchanged);               // my own client state is mine: 0x14 is right
    }
    SetOwner owned;
    owned.ref = esm(0x14);
    owned.owner = esm(0x2222);
    CHECK(world(owned, kHost) == Verdict::Dropped);
    // other actors are untouched
    const WorldEffect fine[] = {SetActorValueRef{npc(), 5, 10.f, AvMode::Set}, SetEssential{npc(), true, true}, SetRestrained{npc(), true},
                                SetScriptPackage{npc(), esm(0x1055C4), true}, SetIgnoreCrime{npc(), true}, SetObjectState{npc(), 1, true, false, 0, false}};
    for (const WorldEffect& e : fine) {
        WorldEffect out;
        CHECK(world(e, kHost, &out) == Verdict::Unchanged);
        CHECK(out == e);
    }
    // effects without an actor reference never change
    CHECK(world(WorldEffect(SetFactionPlayerEnemy{esm(0x14), true}), kHost) == Verdict::Unchanged);
    CHECK(world(WorldEffect(Present{}), kHost) == Verdict::Unchanged);
}

TEST(identity_world_ownership_by_the_player_stays_the_local_player) {
    using namespace idt;
    // `SetOwnership player` / `SetCellOwnership cell player`: the owner is "the player", whoever plays on this machine; the owned object is a world object
    SetOwner o;
    o.ref = esm(0x3333);
    o.owner = esm(0x14);
    WorldEffect out;
    CHECK(world(o, kHost, &out) == Verdict::Unchanged);    // twin 79
    CHECK(std::get<SetOwner>(out).owner == esm(0x14));
    SetCellOwner c;
    c.cell = esm(0x4444);
    c.owner = esm(0x14);
    CHECK(world(c, kHost, &out) == Verdict::Unchanged);
    CHECK(std::get<SetCellOwner>(out).owner == esm(0x14));
    // an owner named by this player's slot reference is this player
    c.owner = idt::myWire();
    CHECK(world(c, kHost, &out) == Verdict::Rewritten);
    CHECK(std::get<SetCellOwner>(out).owner == esm(0x14));
}

static void runIdentityTests() {
    run("identity_table_names_every_effect_type", identity_table_names_every_effect_type);
    run("identity_story_sender_player_is_dropped_where_it_is_the_senders_own_state", identity_story_sender_player_is_dropped_where_it_is_the_senders_own_state);
    run("identity_story_sender_player_as_target_becomes_its_puppet", identity_story_sender_player_as_target_becomes_its_puppet);
    run("identity_story_effect_addressed_by_the_slots_wire_reference_lands_on_my_player", identity_story_effect_addressed_by_the_slots_wire_reference_lands_on_my_player);
    run("identity_refcommand_data_on_the_player_is_kept_only_when_addressed_to_me", identity_refcommand_data_on_the_player_is_kept_only_when_addressed_to_me);
    run("identity_refcommand_arguments_naming_the_player_become_its_puppet", identity_refcommand_arguments_naming_the_player_become_its_puppet);
    run("identity_refcommand_without_a_reference_runs_on_the_player", identity_refcommand_without_a_reference_runs_on_the_player);
    run("identity_off_changes_nothing", identity_off_changes_nothing);
    run("identity_view_is_built_from_the_machines_hooks", identity_view_is_built_from_the_machines_hooks);
    run("identity_local_slot_decides_whose_companion_it_is", identity_local_slot_decides_whose_companion_it_is);
    run("identity_resolver_maps_the_placeholders_to_the_puppets", identity_resolver_maps_the_placeholders_to_the_puppets);
    run("identity_world_effects_of_the_sender_player_are_dropped", identity_world_effects_of_the_sender_player_are_dropped);
    run("identity_world_ownership_by_the_player_stays_the_local_player", identity_world_ownership_by_the_player_stays_the_local_player);
}
