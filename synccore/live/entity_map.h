// One table per machine: which local ref is which entity (docs/entity-core-plan.md 2.1). Game-free.
//
// The player's own machine binds playerNet(slot) to 0x14 (setLocalSlot); every other machine binds it to that player's puppet while the
// puppet is usable. NPCs bind to their real ref or their stand-in. Producers unbind as soon as a ref stops being usable (deleted, parked,
// lost, load begin), so an answer is never a stale runtime id that the engine may already have handed to something else.
//
// Every slot <-> ref question of the plugin (combat, companions, vats, intents, the story layer, AI combat targets) is answered here.
#pragma once
#include <cstdint>
#include <functional>
#include <map>
#include <vector>

#include "entity.h"

namespace fnvcoop::live {

enum class EntityRole : uint8_t { None, LocalPlayer, Puppet, Npc, StandIn };
const char* toString(EntityRole r);

struct EntityBinding {
    EntityNetId id = 0;
    uint32_t ref = 0;
    EntityRole role = EntityRole::None;
    uint32_t generation = 0;   // bumped on every bind of this id: a consumer that cached (id, generation) sees a rebind
};

struct EntityMapStats {
    uint64_t binds = 0, unbinds = 0, rebinds = 0, conflicts = 0, refused = 0;
};

// Mutation twins (tests/test_entity.cpp): each one breaks one rule a scenario guards.
struct EntityFaults {
    bool staleAfterUnbind = false;     // unbind forgets the id but a ref lookup still answers the old entity
    bool acceptConflict = false;       // a ref already bound to another entity is taken over silently
    bool noLocalPlayer = false;        // setLocalSlot does not bind 0x14
    bool puppetIsLocal = false;        // slotOfRef answers for NPC and stand-in refs too
    bool noChangeEvents = false;       // onChange listeners are not told
};
inline EntityFaults& entityFaults() { static EntityFaults f; return f; }
struct EntityFaultScope {
    EntityFaults saved;
    explicit EntityFaultScope(const EntityFaults& f) : saved(entityFaults()) { entityFaults() = f; }
    ~EntityFaultScope() { entityFaults() = saved; }
};

class EntityMap {
public:
    using ChangeFn = std::function<void(EntityNetId id, uint32_t ref, EntityRole role, bool bound)>;

    // This machine's own player: playerNet(slot) <-> 0x14 until clear() or another setLocalSlot. kNoPlayer unbinds it.
    void setLocalSlot(PlayerId slot);
    PlayerId localSlot() const { return localSlot_; }

    // false: the ref is bound to another entity (counted as a conflict; unbind that one first), or id / ref is 0. Binding an id that is
    // bound to another ref moves it (a rebind: the old ref is released).
    bool bind(EntityNetId id, uint32_t ref, EntityRole role);
    void unbind(EntityNetId id);
    void unbindRef(uint32_t ref);
    void clear();   // session end, load begin, hot reload shutdown: every binding goes, listeners are told

    EntityNetId netOf(uint32_t ref) const;   // 0 = none
    uint32_t refOf(EntityNetId id) const;    // 0 = none
    EntityRole roleOf(EntityNetId id) const;
    const EntityBinding* binding(EntityNetId id) const;
    PlayerId slotOfRef(uint32_t ref) const;  // kNoPlayer unless the ref is the local player or a puppet
    uint32_t actingRef(PlayerId slot) const; // the local slot -> 0x14, a remote slot -> its puppet, else 0
    size_t size() const { return byId_.size(); }
    std::vector<EntityBinding> all() const;

    // Listeners in the order they were added; a bind that moves an id reports the unbind of the old ref first.
    size_t onChange(ChangeFn f);
    void removeListener(size_t token);

    const EntityMapStats& stats() const { return stats_; }

private:
    void drop(EntityNetId id);
    void tell(EntityNetId id, uint32_t ref, EntityRole role, bool bound);

    std::map<EntityNetId, EntityBinding> byId_;
    std::map<uint32_t, EntityNetId> byRef_;
    std::map<EntityNetId, uint32_t> generation_;   // survives unbind: a rebind of the same id gets a higher generation
    std::map<uint32_t, EntityNetId> ghost_;        // only under the staleAfterUnbind twin
    std::vector<std::pair<size_t, ChangeFn>> listeners_;
    size_t nextToken_ = 0;
    PlayerId localSlot_ = kNoPlayer;
    EntityMapStats stats_;
};

// The one table of this game process (the plugin): fed by the avatar module (puppets, the local slot) and the actor feed (NPCs), read by
// every module that asks "which ref is player N". Tests make their own EntityMap instances.
EntityMap& processEntities();

}  // namespace fnvcoop::live
