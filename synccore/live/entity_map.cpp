#include "entity_map.h"

namespace fnvcoop::live {

const char* toString(EntityRole r) {
    switch (r) {
        case EntityRole::LocalPlayer: return "local";
        case EntityRole::Puppet: return "puppet";
        case EntityRole::Npc: return "npc";
        case EntityRole::StandIn: return "standin";
        default: return "none";
    }
}

void EntityMap::tell(EntityNetId id, uint32_t ref, EntityRole role, bool bound) {
    if (entityFaults().noChangeEvents) return;
    const auto copy = listeners_;   // a listener may add or remove listeners
    for (const auto& l : copy) l.second(id, ref, role, bound);
}

void EntityMap::drop(EntityNetId id) {
    auto it = byId_.find(id);
    if (it == byId_.end()) return;
    const EntityBinding b = it->second;
    byId_.erase(it);
    byRef_.erase(b.ref);
    if (entityFaults().staleAfterUnbind) ghost_[b.ref] = id;
    ++stats_.unbinds;
    tell(b.id, b.ref, b.role, false);
}

void EntityMap::setLocalSlot(PlayerId slot) {
    if (localSlot_ != kNoPlayer) drop(playerNet(localSlot_));
    localSlot_ = slot;
    if (slot == kNoPlayer || slot >= kMaxSlots || entityFaults().noLocalPlayer) return;
    bind(playerNet(slot), kLocalPlayerRef, EntityRole::LocalPlayer);
}

bool EntityMap::bind(EntityNetId id, uint32_t ref, EntityRole role) {
    if (!id || !ref || role == EntityRole::None) { ++stats_.refused; return false; }
    auto r = byRef_.find(ref);
    if (r != byRef_.end() && r->second != id) {
        if (!entityFaults().acceptConflict) { ++stats_.conflicts; return false; }
        drop(r->second);
    }
    auto it = byId_.find(id);
    if (it != byId_.end()) {
        if (it->second.ref == ref && it->second.role == role) return true;   // already so: no event
        drop(id);
        ++stats_.rebinds;
    }
    ghost_.erase(ref);
    EntityBinding b;
    b.id = id; b.ref = ref; b.role = role; b.generation = ++generation_[id];
    byId_[id] = b;
    byRef_[ref] = id;
    ++stats_.binds;
    tell(id, ref, role, true);
    return true;
}

void EntityMap::unbind(EntityNetId id) { drop(id); }

void EntityMap::unbindRef(uint32_t ref) {
    auto r = byRef_.find(ref);
    if (r != byRef_.end()) drop(r->second);
}

void EntityMap::clear() {
    std::vector<EntityNetId> ids;
    for (const auto& kv : byId_) ids.push_back(kv.first);
    for (EntityNetId id : ids) drop(id);
    if (!entityFaults().staleAfterUnbind) ghost_.clear();
    localSlot_ = kNoPlayer;
}

EntityNetId EntityMap::netOf(uint32_t ref) const {
    auto r = byRef_.find(ref);
    if (r != byRef_.end()) return r->second;
    auto g = ghost_.find(ref);
    return g != ghost_.end() ? g->second : 0;
}

uint32_t EntityMap::refOf(EntityNetId id) const {
    auto it = byId_.find(id);
    return it != byId_.end() ? it->second.ref : 0;
}

EntityRole EntityMap::roleOf(EntityNetId id) const {
    auto it = byId_.find(id);
    return it != byId_.end() ? it->second.role : EntityRole::None;
}

const EntityBinding* EntityMap::binding(EntityNetId id) const {
    auto it = byId_.find(id);
    return it != byId_.end() ? &it->second : nullptr;
}

PlayerId EntityMap::slotOfRef(uint32_t ref) const {
    const EntityNetId id = netOf(ref);
    if (!isPlayerNet(id) && !entityFaults().puppetIsLocal) return kNoPlayer;
    const EntityRole role = roleOf(id);
    if (role != EntityRole::LocalPlayer && role != EntityRole::Puppet && !entityFaults().puppetIsLocal) return kNoPlayer;
    return slotOfPlayerNet(id);
}

uint32_t EntityMap::actingRef(PlayerId slot) const {
    if (slot >= kMaxSlots) return 0;
    return refOf(playerNet(slot));
}

std::vector<EntityBinding> EntityMap::all() const {
    std::vector<EntityBinding> v;
    v.reserve(byId_.size());
    for (const auto& kv : byId_) v.push_back(kv.second);
    return v;
}

size_t EntityMap::onChange(ChangeFn f) {
    if (!f) return 0;
    listeners_.emplace_back(++nextToken_, std::move(f));
    return nextToken_;
}

void EntityMap::removeListener(size_t token) {
    for (size_t i = 0; i < listeners_.size(); ++i)
        if (listeners_[i].first == token) { listeners_.erase(listeners_.begin() + static_cast<long>(i)); return; }
}

EntityMap& processEntities() {
    static EntityMap m;
    return m;
}

}  // namespace fnvcoop::live
