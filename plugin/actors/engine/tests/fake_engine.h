// A small fake game world behind IActorEngine for the actor-port tests: refs with positions, cells, actor values, a "native AI" (a straight
// walk) that only runs while the actor is not restrained, console-like writes that are logged one per line, and the traps the real engine
// has (a script-restrained actor, a base form that must exist, a placement anchor that must exist, refused writes on demand).
#pragma once
#include <cmath>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <vector>

#include "actors/engine/engine_iface.h"

namespace nvae_test {

using namespace nvcoop::actors;
using namespace nvcoop::actors::engine;

struct FakeActor {
    RefId ref = 0;
    uint32_t base = 0;
    bool player = false, disabled = false, deleted = false, has3D = true, created = false;
    CellId cell = 1;
    live::Vec3 pos, rot;
    uint32_t life = 0;
    bool baseEssential = false, baseUnique = false;
    bool restrained = false, essentialRef = false, scriptRestrained = false;
    bool weaponOut = false, aiming = false, sneak = false;
    RefId target = 0;
    uint32_t package = 0;
    float vx = 0, vy = 0;              // native AI: walk velocity (units/s) while not restrained and alive
    std::map<uint8_t, float> av;
    int writes = 0;
};

class FakeEngine : public IActorEngine {
public:
    struct Cell { bool loaded = true; bool exterior = false; uint32_t world = 0; int x = 0, y = 0; };

    std::map<RefId, FakeActor> a;
    std::map<CellId, Cell> cells;

    std::map<uint32_t, fnvcoop::FormRef> formOf;   // runtime form id -> (plugin, local id)
    std::map<std::string, uint32_t> idOf;          // "plugin:local" -> runtime id
    RefId playerId = 0x14;
    bool ready = true;
    uint32_t epoch = 1;
    RefId nextCreated = 0xFF000001;
    size_t listLimit = ~size_t(0);
    std::set<RefId> hidden;                        // refs missing from loadedActors() (a truncated list)
    std::vector<std::string> log;                  // "op:ref:detail", one per console-line-equivalent
    uint64_t lines = 0, failed = 0;
    bool refuseWrites = false;                     // every write fails
    std::set<std::string> refuseOps;               // op names that fail ("setrestrained", "kill", ...)
    bool noAnchor = false;                         // anchorInCell finds nothing
    bool restrainOverwritesLife = true;            // the real engine (game 2026-10-22): SetRestrained 1 on a dying / dead actor makes its life state 5

    FakeEngine() {
        cells[1] = Cell{};
        FakeActor p;
        p.ref = playerId; p.player = true; p.cell = 1; p.base = 7;
        p.av[kAvHealth] = 100;
        a[p.ref] = p;
    }

    // ---- building the world ----
    static std::string key(const fnvcoop::FormRef& r) { return r.plugin + ":" + std::to_string(r.localId); }
    uint32_t form(const std::string& plugin, uint32_t local) {
        fnvcoop::FormRef r(plugin, local);
        auto it = idOf.find(key(r));
        if (it != idOf.end()) return it->second;
        const uint32_t id = (plugin == "falloutnv.esm" ? 0x00000000u : 0x01000000u) | local;
        idOf[key(r)] = id;
        formOf[id] = r;
        return id;
    }
    // A placed NPC. The ref's own id doubles as its local id in falloutnv.esm. `base` is a (plugin-local) base form id.
    FakeActor& addNpc(RefId ref, uint32_t baseLocal, live::Vec3 pos, CellId cell = 1, float vx = 0, float vy = 0) {
        FakeActor f;
        f.ref = ref; f.cell = cell; f.pos = pos; f.vx = vx; f.vy = vy;
        f.base = form("falloutnv.esm", baseLocal);
        f.av[kAvHealth] = 100;
        for (uint8_t l : {kAvHead, kAvTorso, kAvLeftArm, kAvRightArm, kAvLeftLeg, kAvRightLeg, kAvBrain}) f.av[l] = 100;
        f.av[kAvFatigue] = 80;
        formOf[ref] = fnvcoop::FormRef("falloutnv.esm", ref & 0xFFFFFF);
        idOf[key(formOf[ref])] = ref;
        if (!cells.count(cell)) cells[cell] = Cell{};
        a[ref] = f;
        return a[ref];
    }
    fnvcoop::FormRef refForm(RefId ref) const { return fnvcoop::FormRef("falloutnv.esm", ref & 0xFFFFFF); }
    int count(const std::string& op, RefId ref) const {
        int n = 0;
        const std::string pre = op + ":" + std::to_string(ref) + ":";
        for (auto& l : log) if (l.compare(0, pre.size(), pre) == 0) ++n;
        return n;
    }
    int countOp(const std::string& op) const {
        int n = 0;
        const std::string pre = op + ":";
        for (auto& l : log) if (l.compare(0, pre.size(), pre) == 0) ++n;
        return n;
    }
    bool did(const std::string& line) const { for (auto& l : log) if (l == line) return true; return false; }
    void simulate(uint32_t dtMs) {
        for (auto& kv : a) {
            FakeActor& f = kv.second;
            if (f.player || f.deleted || f.disabled || f.restrained || f.scriptRestrained || f.life != 0) continue;
            f.pos.x += f.vx * dtMs / 1000.0f;
            f.pos.y += f.vy * dtMs / 1000.0f;
        }
    }

    // ---- reads ----
    bool worldReady() override { return ready; }
    uint32_t worldEpoch() override { return epoch; }
    RefId playerRef() override { return playerId; }
    bool read(RefId ref, ActorRaw& out) override {
        auto it = a.find(ref);
        out = ActorRaw{};
        if (it == a.end()) return false;
        const FakeActor& f = it->second;
        out.valid = true; out.ref = ref; out.baseId = f.base; out.isPlayer = f.player; out.disabled = f.disabled; out.deleted = f.deleted;
        out.has3D = f.has3D; out.cell = f.cell; out.pos = f.pos; out.rot = f.rot; out.lifeState = f.life;
        out.baseEssential = f.baseEssential; out.baseUnique = f.baseUnique;
        out.haveProc = true; out.weaponOut = f.weaponOut; out.aiming = f.aiming; out.inCombat = f.target != 0; out.combatTarget = f.target;
        out.package = f.package;
        out.restrainedKnown = true; out.restrained = f.restrained || f.scriptRestrained;
        out.haveMove = true;
        out.moveFlags = (f.sneak ? 0x200u : 0u) | ((std::fabs(f.vx) + std::fabs(f.vy)) > 200.0f ? 0x100u : 0x80u);
        return true;
    }
    bool actorValue(RefId ref, uint8_t av, float& out) override {
        auto it = a.find(ref);
        if (it == a.end()) return false;
        auto v = it->second.av.find(av);
        if (v == it->second.av.end()) return false;
        out = v->second;
        return true;
    }
    void loadedActors(std::vector<RefId>& out, size_t max) override {
        out.clear();
        for (auto& kv : a) {
            const FakeActor& f = kv.second;
            if (f.deleted) continue;
            if (hidden.count(kv.first)) continue;
            auto c = cells.find(f.cell);
            if (c == cells.end() || !c->second.loaded) continue;
            out.push_back(kv.first);
            if (out.size() >= max || out.size() >= listLimit) break;
        }
    }
    bool cellLoaded(CellId cell) override { auto c = cells.find(cell); return c != cells.end() && c->second.loaded; }
    bool cellInfo(CellId cell, CellInfo& out) override {
        auto c = cells.find(cell);
        out = CellInfo{};
        if (c == cells.end()) return false;
        out.known = true; out.exterior = c->second.exterior; out.world = c->second.world; out.x = c->second.x; out.y = c->second.y;
        return true;
    }
    bool toFormRef(uint32_t id, fnvcoop::FormRef& out) override {
        if ((id >> 24) == 0xFF) return false;
        auto it = formOf.find(id);
        if (it == formOf.end()) return false;
        out = it->second;
        return true;
    }
    uint32_t fromFormRef(const fnvcoop::FormRef& r) override {
        auto it = idOf.find(key(r));
        return it == idOf.end() ? 0 : it->second;
    }
    int anchorCalls = 0;
    RefId anchorInCell(CellId cell, live::Vec3& p) override {
        ++anchorCalls;
        if (noAnchor) return 0;
        for (auto& kv : a) {
            const FakeActor& f = kv.second;
            if (f.cell == cell && !f.deleted && !f.disabled) { p = f.pos; return kv.first; }
        }
        return 0;
    }

    // ---- writes ----
    bool w(const std::string& op, RefId ref, const std::string& detail = "", int nLines = 1) {
        lines += nLines;
        if (refuseWrites || refuseOps.count(op)) { failed += nLines; log.push_back("refused-" + op + ":" + std::to_string(ref) + ":" + detail); return false; }
        auto it = a.find(ref);
        if (it == a.end() || it->second.deleted) { failed += nLines; log.push_back("gone-" + op + ":" + std::to_string(ref) + ":" + detail); return false; }
        ++it->second.writes;
        log.push_back(op + ":" + std::to_string(ref) + ":" + detail);
        return true;
    }
    static std::string num(float v) { std::ostringstream o; o << v; return o.str(); }
    RefId placeAtMe(RefId anchor, uint32_t baseId) override {
        auto an = a.find(anchor);
        lines += 1;
        if (refuseWrites || refuseOps.count("placeatme") || an == a.end()) { ++failed; return 0; }
        FakeActor f;
        f.ref = nextCreated++; f.created = true; f.base = baseId; f.cell = an->second.cell; f.pos = an->second.pos;
        f.av[kAvHealth] = 100;
        a[f.ref] = f;
        log.push_back("placeatme:" + std::to_string(f.ref) + ":" + std::to_string(baseId));
        return f.ref;
    }
    bool setPos(RefId ref, const live::Vec3& p, uint8_t mask) override {
        const int n = (mask & 1 ? 1 : 0) + (mask & 2 ? 1 : 0) + (mask & 4 ? 1 : 0);
        std::string d = (mask & 1 ? "x" : "") + std::string(mask & 2 ? "y" : "") + (mask & 4 ? "z" : "");
        if (!w("setpos", ref, d, n ? n : 1)) return false;
        FakeActor& f = a[ref];
        if (mask & 1) f.pos.x = p.x;
        if (mask & 2) f.pos.y = p.y;
        if (mask & 4) f.pos.z = p.z;
        return true;
    }
    bool setYaw(RefId ref, float yaw) override {
        if (!w("setyaw", ref, num(yaw))) return false;
        a[ref].rot.z = yaw;
        return true;
    }
    bool moveToCell(RefId ref, CellId cell, const live::Vec3& p, float yaw) override {
        if (!w("moveto", ref, std::to_string(cell), 2)) return false;
        FakeActor& f = a[ref];
        f.cell = cell; f.pos = p; f.rot.z = yaw;
        return true;
    }
    bool setRestrained(RefId ref, bool on) override {
        if (!w(on ? "restrain" : "unrestrain", ref)) return false;
        a[ref].restrained = on;
        if (on && restrainOverwritesLife && (a[ref].life == 1 || a[ref].life == 2)) a[ref].life = 5;
        return true;
    }
    bool stopCombat(RefId ref) override { if (!w("stopcombat", ref)) return false; a[ref].target = 0; return true; }
    bool startCombat(RefId ref, RefId target) override { if (!w("startcombat", ref, std::to_string(target))) return false; a[ref].target = target; return true; }
    bool evaluatePackage(RefId ref) override { return w("evalpackage", ref); }
    bool setEssential(RefId ref, bool on) override { if (!w(on ? "essential1" : "essential0", ref)) return false; a[ref].essentialRef = on; return true; }
    bool forceAv(RefId ref, uint8_t av, float v) override {
        if (!w("forceav", ref, std::to_string(av) + "=" + num(v))) return false;
        a[ref].av[av] = v;
        if (av == kAvHealth && v <= 0) a[ref].life = 2;   // the engine kills at 0 health
        return true;
    }
    bool killActor(RefId ref) override { if (!w("kill", ref)) return false; a[ref].life = 2; a[ref].av[kAvHealth] = 0; return true; }
    bool killActorBy(RefId ref, RefId killer) override { if (!w("killby", ref, std::to_string(killer))) return false; a[ref].life = 2; a[ref].av[kAvHealth] = 0; return true; }
    bool resurrect(RefId ref) override { if (!w("resurrect", ref)) return false; a[ref].life = 0; if (a[ref].av[kAvHealth] <= 0) a[ref].av[kAvHealth] = 50; return true; }
    bool setDisabled(RefId ref, bool d) override { if (!w(d ? "disable" : "enable", ref)) return false; a[ref].disabled = d; return true; }
    bool markForDelete(RefId ref) override { if (!w("delete", ref)) return false; a[ref].deleted = true; return true; }
    uint64_t linesRun() const override { return lines; }
    uint64_t linesFailed() const override { return failed; }
};

}  // namespace nvae_test
