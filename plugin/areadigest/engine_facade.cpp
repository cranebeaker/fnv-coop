#include "areadigest/engine_facade.h"

#include "actors/engine/actor_engine_api.h"
#include "actors/engine/engine_iface.h"

namespace nvcoop::areadigest {

namespace eng = nvcoop::actors::engine;

namespace {

std::function<bool(uint32_t)> g_resync;

// Reads one loaded reference. false = not a live, enabled, non-player actor.
bool readRef(eng::IActorEngine* e, eng::RefId ref, uint32_t id, ActorRead& out) {
    eng::ActorRaw raw;
    if (!e->read(ref, raw) || !raw.valid || raw.isPlayer || raw.disabled || raw.deleted) return false;
    float hp = 0;
    if (!e->actorValue(ref, eng::kAvHealth, hp)) hp = 0;
    out = ActorRead();
    out.netId = id;
    out.standIn = eng::isStandIn(ref);
    out.named = raw.baseUnique || raw.baseEssential;   // the closest the engine read gets to "story NPC"; wrong in the safe direction
    out.alive = !raw.dead();
    out.cell = raw.cell;
    out.hp = hp;
    out.pos.x = raw.pos.x; out.pos.y = raw.pos.y; out.pos.z = raw.pos.z;
    return true;
}

class EngineFacade : public IActorFacade {
public:
    bool ready() override {
        eng::IActorEngine* e = eng::actorEngine();
        return e && e->worldReady();
    }
    void tracked(std::vector<ActorRead>& out, size_t max) override {
        eng::IActorEngine* e = eng::actorEngine();
        if (!e) return;
        std::vector<eng::RefId> refs;
        e->loadedActors(refs, max * 2 + 8);              // untracked actors are skipped below, so ask for more than the cap
        for (eng::RefId ref : refs) {
            if (out.size() >= max) break;
            const uint32_t id = eng::netIdOf(ref);        // 0 = not a synced actor
            if (!id) continue;
            ActorRead r;
            if (readRef(e, ref, id, r)) out.push_back(r);
        }
    }
    bool read(uint32_t netId, ActorRead& out) override {
        eng::IActorEngine* e = eng::actorEngine();
        if (!e) return false;
        const eng::RefId ref = eng::refOf(netId);
        return ref && readRef(e, ref, netId, out);
    }
    bool owner(uint32_t netId, fnvcoop::areadigest::OwnerView& out) override {
        const eng::OwnerRef o = eng::ownerOfNet(netId);
        out = fnvcoop::areadigest::OwnerView();
        out.known = o.known;
        out.owner = o.ownerSlot;
        out.epoch = o.epoch;
        out.pending = o.pending;
        return o.known;
    }
    bool playerCell(CellId& out) override {
        eng::IActorEngine* e = eng::actorEngine();
        if (!e) return false;
        eng::ActorRaw raw;
        if (!e->read(e->playerRef(), raw) || !raw.valid || !raw.cell) return false;
        out = raw.cell;
        return true;
    }
    bool setHealth(uint32_t netId, float hp) override {
        eng::IActorEngine* e = eng::actorEngine();
        const eng::RefId ref = e ? eng::refOf(netId) : 0;
        return ref && e->forceAv(ref, eng::kAvHealth, hp);
    }
    bool setPos(uint32_t netId, const Vec3& pos) override {
        eng::IActorEngine* e = eng::actorEngine();
        const eng::RefId ref = e ? eng::refOf(netId) : 0;
        decltype(eng::ActorRaw::pos) p;
        p.x = pos.x; p.y = pos.y; p.z = pos.z;
        return ref && e->setPos(ref, p, 7);
    }
    bool moveToCell(uint32_t netId, CellId cell, const Vec3& pos) override {
        eng::IActorEngine* e = eng::actorEngine();
        const eng::RefId ref = e ? eng::refOf(netId) : 0;
        if (!ref) return false;
        eng::ActorRaw raw;
        const float yaw = e->read(ref, raw) ? raw.rot.z : 0.0f;
        decltype(eng::ActorRaw::pos) p;
        p.x = pos.x; p.y = pos.y; p.z = pos.z;
        return e->moveToCell(ref, cell, p, yaw);
    }
    bool requestOwnerResync(uint32_t netId) override { return g_resync && g_resync(netId); }
    bool moveHeld(uint32_t netId) override { return eng::poseLocked(netId); }
};

}  // namespace

IActorFacade* engineFacade() {
    static EngineFacade f;
    return &f;
}

void setOwnerResyncHook(std::function<bool(uint32_t netId)> hook) { g_resync = std::move(hook); }

}  // namespace nvcoop::areadigest
