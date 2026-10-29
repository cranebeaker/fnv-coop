// The module that plugs plugin/actors/engine into nvcoop.dll (x86; a DIRECT source of the DLL so the static registrar below is kept).
//
//   nvcoop.ini
//     [actors]  engine=1            0 = install nothing (the no-op port stays, coop.owner answers known:false)
//               scanMs=500 maxLinesPerFrame=24 driveMinMs=33 maxStandIns=64 restrain=1 swapLeveled=1 persistLedger=1
//     [dev]     actorport=1         registers the dev bridge methods below (the in-game proof tools/actors/proof_actor_port.py uses them)
//
//   bridge methods (main thread)
//     actors.port                   -> status: counters, ledger sizes, engine pins, tracked/stand-in counts
//     actors.read   {ref}           -> [dev] one read of an actor reference (pos/rot/cell/life/AVs/AI facts) for comparing with the console
//     actors.dev    {op, ...}       -> [dev] bind | sample | ai | place | drive | protect | swap | kill | revive | standin | unstandin | undo | release | scan
//
// The module owns the one RealEngineHost / ScriptActorEngine / Ledger / EnginePort / ActorFeed of the process and tells the coop glue
// (plugin/actors/actor_glue.h) about the port and the SyncConfig adjacency.
#include <Windows.h>

#include <cstdio>
#include <memory>
#include <string>
#include <variant>
#include <vector>

#include "PluginAPI.h"
#include "actors/actor_glue.h"
#include "actors/engine/actor_feed.h"
#include "actors/engine/engine_port.h"
#include "actors/engine/engine_real.h"
#include "actors/engine/equipment_real.h"
#include "actors/engine/ledger.h"
#include "actors/engine/registry.h"
#include "actors/engine/script_engine.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "effect_json.h"
#include "engine_hooks.h"
#include "reload/reload_state.h"
#ifdef NVCOOP_WITH_LIVE
#include "coop/live/live_actors_cfg.h"   // dll-wiring: [actors] ini parsing (readActorsConfig) and the shared ini getter
#include "coop/live/live_glue.h"
#endif

namespace {

using namespace nvcoop::actors;
using namespace nvcoop::actors::engine;
using nvcoop::json::Value;

std::string ModuleDir() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&ModuleDir), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return s == std::string::npos ? std::string() : p.substr(0, s + 1);
}

int IniInt(const char* section, const char* key, int def) {
    return static_cast<int>(GetPrivateProfileIntA(section, key, def, (ModuleDir() + "nvcoop.ini").c_str()));
}

// The ledger file lives next to the DLL. Written through a temp file so a crash cannot leave half a ledger.
class DiskStore : public IFileStore {
public:
    explicit DiskStore(std::string path) : path_(std::move(path)) {}
    bool read(std::string& out) override {
        HANDLE f = CreateFileA(path_.c_str(), GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);
        if (f == INVALID_HANDLE_VALUE) return false;
        out.clear();
        char buf[4096];
        DWORD n = 0;
        while (ReadFile(f, buf, sizeof buf, &n, nullptr) && n) out.append(buf, n);
        CloseHandle(f);
        return true;
    }
    bool write(const std::string& text) override {
        const std::string tmp = path_ + ".tmp";
        HANDLE f = CreateFileA(tmp.c_str(), GENERIC_WRITE, 0, nullptr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, nullptr);
        if (f == INVALID_HANDLE_VALUE) return false;
        DWORD n = 0;
        const BOOL ok = WriteFile(f, text.data(), static_cast<DWORD>(text.size()), &n, nullptr);
        CloseHandle(f);
        if (!ok || n != text.size()) return false;
        return MoveFileExA(tmp.c_str(), path_.c_str(), MOVEFILE_REPLACE_EXISTING) != 0;
    }

private:
    std::string path_;
};

// How many of `form` the reference carries, from the inventory provider (plugin/loot reads base container + changes): < 0 = unknown, then no AddItem.
int ItemCountOf(RefId ref, const fnvcoop::FormRef& form) {
    Registry& reg = registry();
    if (!reg.invRead || !reg.netOfRef) return -1;
    const NetId n = reg.netOfRef(ref);
    InvList items;
    if (!n || !reg.invRead(n, items)) return -1;
    for (const InvItem& it : items)
        if (it.form == form) return it.count;
    return 0;
}

struct State {
    RealEngineHost host;
    ScriptActorEngine eng{host};
    DiskStore store;
    Ledger ledger;
    EnginePort port;
    ActorFeed feed;
    EquipmentAccess equip;
    explicit State(const Config& c)
        : store(ModuleDir() + "nvcoop_actors_ledger.txt"), ledger(&store), port(eng, ledger, c),
          feed(eng, port, ledger, [] { return glueSync(); }, [] { return glueSessionGeneration(); }),
          equip(host, readWornEntries, ItemCountOf) {
        feed.setConfig(c);
    }
};

std::unique_ptr<State> g_st;

uint64_t NowMs() { return GetTickCount64(); }

Config ReadConfig() {
#ifdef NVCOOP_WITH_LIVE
    // dll-wiring: every [actors] key (same names as below, `restrain` included) is parsed and clamped in plugin/coop/live/live_actors_cfg.cpp
    // (unit-tested); environment NVCOOP_ACTORS_<KEY> overrides the ini. [live] actors=0 keeps this port installed but unfed: it only undoes the
    // restraints a crashed run left in its ledger, it never tracks an actor without a session.
    return nvcoop::livewire::readActorsConfig(nvcoop::livewire::glueIni());
#else
    Config c;
    c.enabled = IniInt("actors", "engine", 1) != 0;
    c.scanMs = static_cast<uint32_t>(IniInt("actors", "scanMs", static_cast<int>(c.scanMs)));
    c.maxLinesPerFrame = static_cast<uint32_t>(IniInt("actors", "maxLinesPerFrame", static_cast<int>(c.maxLinesPerFrame)));
    c.driveMinMs = static_cast<uint32_t>(IniInt("actors", "driveMinMs", static_cast<int>(c.driveMinMs)));
    c.maxStandIns = static_cast<uint32_t>(IniInt("actors", "maxStandIns", static_cast<int>(c.maxStandIns)));
    c.restrainNonOwned = IniInt("actors", "restrain", 1) != 0;
    c.swapLeveled = IniInt("actors", "swapLeveled", 1) != 0;
    c.persistLedger = IniInt("actors", "persistLedger", 1) != 0;
    return c;
#endif
}

// ---- bridge methods ---------------------------------------------------------------------------------------------------------------------
RefId ParseRef(const Value& v) {
    const std::string s = v.asString("");
    if (s.empty()) return 0;
    if (s.find(':') != std::string::npos) {
        fnvcoop::FormRef r;
        if (!nvhooks::strToRef(s, &r)) return 0;
        return nvhooks::resolver().fromRef(r);
    }
    return static_cast<RefId>(std::strtoul(s.c_str(), nullptr, 16));
}

// "Plugin.esm:HEX" or a runtime form id in hex -> the plugin / local-id pair every machine agrees on
bool ParseForm(const Value& v, fnvcoop::FormRef& out) {
    const std::string s = v.asString("");
    if (s.empty()) return false;
    if (s.find(':') != std::string::npos) return nvhooks::strToRef(s, &out);
    return g_st && g_st->host.toFormRef(static_cast<uint32_t>(std::strtoul(s.c_str(), nullptr, 16)), out);
}

std::string Hex(uint32_t v) {
    char b[16];
    std::snprintf(b, sizeof b, "%08X", v);
    return b;
}

Value StatsJson(const PortStats& s) {
    Value o = Value::object();
    auto n = [&](const char* k, uint64_t v) { o.set(k, static_cast<int64_t>(v)); };
    n("scans", s.scans); n("trackedNew", s.trackedNew); n("trackedGone", s.trackedGone); n("deathsPolled", s.deathsPolled);
    n("deathsReported", s.deathsReportedToSync); n("deathsRefused", s.deathsRefused); n("consoleLines", s.consoleLines);
    n("consoleFailed", s.consoleFailed); n("linesDeferred", s.linesDeferred); n("drives", s.drives); n("drivesSkippedSmall", s.drivesSkippedSmall);
    n("drivesSkippedRate", s.drivesSkippedRate); n("drivesHeldSeated", s.drivesHeldSeated); n("snaps", s.snaps); n("places", s.places); n("aiOff", s.aiOff); n("aiOn", s.aiOn);
    n("aiFailed", s.aiFailed); n("combatRestarted", s.combatRestarted); n("combatTargetUnresolved", s.combatTargetUnresolved);
    n("standInsMade", s.standInsMade); n("standInsRemoved", s.standInsRemoved); n("standInsRefused", s.standInsRefused);
    n("standInsPurged", s.standInsPurged); n("baseSwaps", s.baseSwaps); n("baseSwapFailed", s.baseSwapFailed);
    n("protectOn", s.protectOn); n("protectOff", s.protectOff); n("kills", s.kills); n("revives", s.revives);
    n("ledgerReleased", s.ledgerReleased); n("ledgerWrites", s.ledgerWrites); n("ledgerDropped", s.ledgerDropped);
    n("avDrifts", s.avDrifts);
    o.set("lastDriftNet", static_cast<int64_t>(s.lastDriftNet)).set("lastDriftCur", s.lastDriftCur).set("lastDriftWant", s.lastDriftWant);
    return o;
}

bool MPort(const std::string&, std::string& out) {
    Value o = Value::object();
    o.set("enabled", g_st != nullptr);
    if (g_st) {
        o.set("worldReady", g_st->host.worldReady()).set("epoch", static_cast<int64_t>(g_st->host.worldEpoch()));
        o.set("session", glueSync() != nullptr).set("fed", static_cast<int64_t>(g_st->feed.fedCount()));
        o.set("portEntries", static_cast<int64_t>(g_st->port.ids().size())).set("standIns", static_cast<int64_t>(g_st->port.standInCount()));
        Value led = Value::object();
        led.set("restrained", static_cast<int64_t>(g_st->ledger.all(LedgerKind::Restrained).size()))
            .set("standIn", static_cast<int64_t>(g_st->ledger.all(LedgerKind::StandIn).size()))
            .set("disabled", static_cast<int64_t>(g_st->ledger.all(LedgerKind::Disabled).size()))
            .set("essential", static_cast<int64_t>(g_st->ledger.all(LedgerKind::Essential).size()))
            .set("stale", static_cast<int64_t>(g_st->port.staleCount()));
        o.set("ledger", led);
        const RealEngineHost::Pins p = g_st->host.pins();
        Value pins = Value::object();
        pins.set("placeAtMe", p.placeAtMe).set("getActorValue", p.getActorValue).set("isDisabled", p.isDisabled);
        o.set("pins", pins).set("sehFaults", static_cast<int64_t>(g_st->host.sehFaults()));
        o.set("stats", StatsJson(portStats()));
        const EquipmentCounters& ec = g_st->equip.counters();
        Value eq = Value::object();
        eq.set("reads", static_cast<int64_t>(ec.reads)).set("writes", static_cast<int64_t>(ec.writes)).set("writesDone", static_cast<int64_t>(ec.writesDone))
            .set("equipLines", static_cast<int64_t>(ec.equipLines)).set("unequipLines", static_cast<int64_t>(ec.unequipLines))
            .set("addLines", static_cast<int64_t>(ec.addLines)).set("lineFailures", static_cast<int64_t>(ec.lineFailures))
            .set("unresolved", static_cast<int64_t>(ec.unresolved));
        o.set("equipment", eq);
    }
    out = o.dump();
    return true;
}

bool MRead(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    if (!g_st) { out = "actor port disabled"; return false; }
    const RefId ref = ParseRef(p.get("ref"));
    ActorRaw raw;
    raw.wantRestrained = true;
    if (!ref || !g_st->host.read(ref, raw)) { out = "not a loaded actor"; return false; }
    Value o = Value::object();
    o.set("ref", Hex(ref)).set("base", Hex(raw.baseId)).set("cell", Hex(raw.cell));
    o.set("x", raw.pos.x).set("y", raw.pos.y).set("z", raw.pos.z).set("rotX", raw.rot.x).set("rotY", raw.rot.y).set("rotZ", raw.rot.z);
    o.set("lifeState", static_cast<int64_t>(raw.lifeState)).set("dead", raw.dead()).set("has3D", raw.has3D).set("disabled", raw.disabled);
    o.set("weaponOut", raw.weaponOut).set("aiming", raw.aiming).set("swimming", raw.swimming).set("inCombat", raw.inCombat);
    o.set("combatTarget", Hex(raw.combatTarget)).set("package", Hex(raw.package));
    o.set("haveMove", raw.haveMove).set("moveFlags", static_cast<int64_t>(raw.moveFlags)).set("sneak", raw.haveMove && sneakFromMoveFlags(raw.moveFlags));
    o.set("restrainedKnown", raw.restrainedKnown).set("restrained", raw.restrained);
    o.set("baseEssential", raw.baseEssential).set("baseUnique", raw.baseUnique);
    Value av = Value::object();
    for (uint8_t a : watchedAvs()) {
        float v = 0;
        if (g_st->host.actorValue(ref, a, v)) av.set(std::to_string(a), static_cast<double>(v));
    }
    o.set("av", av);
    out = o.dump();
    return true;
}

bool MDev(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    if (!g_st) { out = "actor port disabled"; return false; }
    State& st = *g_st;
    const std::string op = p.get("op").asString("");
    Value o = Value::object();
    const uint64_t now = NowMs();
    st.port.beginFrame(now);
    if (op == "bind" || op == "sample" || op == "ai" || op == "place" || op == "kill" || op == "revive" || op == "drive" || op == "protect" || op == "swap") {
        // by "ref" (a placed actor: bound on first use) or by "net" (what an earlier standin returned: a clone has no placed identity)
        NetId id = 0;
        RefId ref = 0;
        ActorRaw raw;
        if (p.find("net")) {
            id = static_cast<NetId>(p.get("net").asInt(0));
            ref = st.port.activeRef(id);
            if (!ref || !st.host.read(ref, raw)) { out = "net id is not bound to a loaded actor"; return false; }
        } else {
            ref = ParseRef(p.get("ref"));
            fnvcoop::FormRef fr;
            if (!ref || !st.host.toFormRef(ref, fr)) { out = "ref is not a placed actor"; return false; }
            id = deriveNetId(fr);
            if (!st.host.read(ref, raw)) { out = "not a loaded actor"; return false; }
            if (!st.port.bound(id)) {
                ActorMeta m;
                m.netId = id; m.ref = fr;
                st.host.toFormRef(raw.baseId, m.base);
                m.cell = raw.cell; m.pos = raw.pos;
                st.port.bindReal(m, ref);
            }
        }
        o.set("netId", static_cast<int64_t>(id));
        if (op == "sample") {
            live::ActorSnapshot s, s2;
            if (!st.port.sample(id, s)) { out = "sample failed"; return false; }
            st.port.beginFrame(now + 500);   // a second sample half a second later gives a velocity
            st.port.sample(id, s2);
            o.set("x", s2.pos.x).set("y", s2.pos.y).set("z", s2.pos.z).set("yaw", s2.rot.y).set("cell", Hex(s2.cell));
            o.set("vx", s2.vel.x).set("vy", s2.vel.y);
            o.set("weaponDrawn", s2.anim.weaponDrawn).set("sneak", s2.anim.sneak).set("aim", s2.anim.aim).set("loco", static_cast<int64_t>(s2.anim.locomotion));
            o.set("alert", static_cast<int64_t>(s2.ai.alert)).set("package", Hex(s2.ai.package));
            Value av = Value::object();
            for (auto& v : s2.avDeltas) av.set(std::to_string(v.av), static_cast<int64_t>(v.value));
            o.set("av", av);
        } else if (op == "ai") {
            st.port.setAi(id, p.get("on").asBool(true));
            o.set("aiOff", st.port.aiOff(id));
        } else if (op == "place") {
            live::ActorSnapshot s;
            s.netId = id; s.cell = raw.cell;
            s.pos = {static_cast<float>(p.get("x").asNumber(raw.pos.x)), static_cast<float>(p.get("y").asNumber(raw.pos.y)), static_cast<float>(p.get("z").asNumber(raw.pos.z))};
            s.rot.y = static_cast<float>(p.get("yaw").asNumber(raw.rot.z));
            if (p.find("health")) s.avDeltas.push_back({kAvHealth, static_cast<int32_t>(p.get("health").asInt(100))});
            if (p.find("head")) s.avDeltas.push_back({kAvHead, static_cast<int32_t>(p.get("head").asInt(100))});
            st.port.place(id, s);
        } else if (op == "drive") {
            live::InterpOut d;
            d.state = live::InterpState::Interpolating;
            d.cell = raw.cell;
            d.pos = {static_cast<float>(p.get("x").asNumber(raw.pos.x)), static_cast<float>(p.get("y").asNumber(raw.pos.y)), static_cast<float>(p.get("z").asNumber(raw.pos.z))};
            d.rot.y = static_cast<float>(p.get("yaw").asNumber(raw.rot.z));
            d.snapped = p.get("snap").asBool(false);
            st.port.drive(id, d);
        } else if (op == "protect") {
            st.port.setProtected(id, p.get("on").asBool(true));
            o.set("protected", st.port.isProtected(id));
        } else if (op == "swap") {
            fnvcoop::FormRef pick;
            if (!ParseForm(p.get("pick"), pick)) { out = "bad pick"; return false; }
            st.port.setBase(id, pick);
            o.set("active", Hex(st.port.activeRef(id))).set("real", Hex(st.port.realRef(id)));
        } else if (op == "kill") {
            st.port.kill(id);
        } else if (op == "revive") {
            st.port.revive(id);
        }
    } else if (op == "standin") {
        const RefId nearRef = ParseRef(p.get("near"));
        fnvcoop::FormRef base;
        if (!ParseForm(p.get("base"), base)) { out = "bad base"; return false; }
        ActorRaw raw;
        if (!nearRef || !st.host.read(nearRef, raw)) { out = "near is not a loaded actor"; return false; }
        ActorMeta m;
        m.ref = fnvcoop::FormRef("nvcoop-proof.esp", static_cast<uint32_t>(p.get("n").asInt(1)));
        m.netId = deriveNetId(m.ref);
        m.base = base;
        live::ActorSnapshot at;
        at.netId = m.netId; at.cell = raw.cell;
        at.pos = {raw.pos.x + 150.0f, raw.pos.y, raw.pos.z};
        at.rot.y = raw.rot.z;
        const bool ok = st.port.spawnStandIn(m, at);
        o.set("ok", ok).set("netId", static_cast<int64_t>(m.netId)).set("ref", Hex(st.port.activeRef(m.netId)));
    } else if (op == "unstandin") {
        const NetId id = deriveNetId(fnvcoop::FormRef("nvcoop-proof.esp", static_cast<uint32_t>(p.get("n").asInt(1))));
        st.port.removeStandIn(id);
        o.set("standIns", static_cast<int64_t>(st.port.standInCount()));
    } else if (op == "undo") {
        o.set("undone", static_cast<int64_t>(st.port.undoStale()));
    } else if (op == "release") {
        st.port.releaseAll();
    } else if (op == "scan") {
        st.feed.markDirty();
    } else {
        out = "unknown op";
        return false;
    }
    st.port.flush();
    o.set("lines", static_cast<int64_t>(st.eng.linesRun())).set("failed", static_cast<int64_t>(st.eng.linesFailed()));
    out = o.dump();
    return true;
}

// ---- cosave: what the port changed in the world travels WITH each save -----------------------------------------------------------------
// A save written while an NPC is restrained, a clone stands or an original is disabled keeps all of it; loading that save later (in single
// player, or in a new session after the old one released everything) must undo it, and the ledger file of the run that wrote it is long
// gone. So the ledger is also written into the save's .nvse cosave and merged into the ledger when that save is loaded; the next world
// change then counts every entry as left behind and undoes it (EnginePort::undoStale).
NVSESerializationInterface* g_ser = nullptr;
constexpr UInt32 kRecordLedger = 0x474C4341;   // 'ACLG'

void CosaveSave(void*) {
    if (!g_st || !g_ser) return;
    const std::string text = g_st->ledger.serialize();
    g_ser->WriteRecord(kRecordLedger, 1, text.data(), static_cast<UInt32>(text.size()));
    g_st->ledger.flush(NowMs(), true);
}

void CosaveLoad(void*) {
    if (!g_st || !g_ser) return;
    UInt32 type = 0, version = 0, length = 0;
    size_t merged = 0;
    while (g_ser->GetNextRecordInfo(&type, &version, &length)) {
        if (type != kRecordLedger || version != 1 || length == 0 || length > (1u << 20)) continue;
        std::string text(length, '\0');
        if (g_ser->ReadRecordData(&text[0], length) != length) continue;
        merged += g_st->ledger.merge(text, [](uint32_t& id) {
            if ((id >> 24) == 0xFF) return true;   // created refs keep their ids
            UInt32 out = 0;
            if (!g_ser->ResolveRefID(id, &out)) return false;   // the mod that owned it is not loaded now
            id = out;
            return true;
        });
    }
    nvcoop::Log("actors: cosave of the loaded game added %zu ledger entries to undo", merged);
}

// ---- lifecycle ----------------------------------------------------------------------------------------------------------------------------
void Teardown() {
    if (!g_st) return;
    g_st->port.releaseAll();
    nvcoop::actors::setGamePort(nullptr);
    nvcoop::actors::setSyncConfigurer(nullptr);
    registry().reset();
    g_st.reset();
}

void OnLoad(void* nvse) {
    const Config c = ReadConfig();
    configure(c);
    if (!c.enabled) {
        nvcoop::Log("actors: engine port disabled ([actors] engine=0)");
        return;
    }
    g_st.reset(new State(c));
    g_st->ledger.load();
    State* st = g_st.get();
    wireApi(st->port, st->feed, st->eng, [] { return glueSync(); });
    // Equipment part (what an NPC holds and wears): the port asks by NetId, the access works on the live ref the port has bound to it.
    setEquipmentAccess(
        [st](NetId id, EquipList& out) {
            const RefId ref = registry().refOfNet ? registry().refOfNet(id) : 0;
            return ref && st->equip.read(ref, out);
        },
        [st](NetId id, const EquipList& want) {
            const RefId ref = registry().refOfNet ? registry().refOfNet(id) : 0;
            return ref && st->equip.write(ref, want);
        });
    nvcoop::actors::setGamePort(&st->port);
    nvcoop::actors::setSyncConfigurer([st](SyncConfig& sc) {
        sc.adjacent = [st](CellId a, CellId b) { return st->feed.adjacent(a, b); };
        sc.acceptRepeatedFinal = IniInt("actors", "norepeatfinal", 0) == 0;       // switches for an A/B run in game: 1 = the behaviour before the lab run of 2026-10-28
        sc.commitWithoutFinal = IniInt("actors", "nocommitnofinal", 0) == 0;
        if (IniInt("actors", "noavaudit", 0) != 0) sc.avAuditMs = 0;
    });
    if (nvse) {
        NVSEInterface* n = static_cast<NVSEInterface*>(nvse);
        // QueryInterface is fine at any time; GetPluginHandle / SetSaveCallback are not (a hot reload crashed NVSE there), so the
        // callbacks go through the forwarders NVSEPlugin_Load registered once.
        g_ser = static_cast<NVSESerializationInterface*>(n->QueryInterface(kInterface_Serialization));
        if (g_ser) nvcoop::SetCosaveCallbacks(CosaveSave, CosaveLoad);
    }
    nvcoop::RegisterMethod("actors.port", MPort, true);
    if (IniInt("dev", "actorport", 0) != 0) {
        nvcoop::RegisterMethod("actors.read", MRead, true);
        nvcoop::RegisterMethod("actors.dev", MDev, true);
        nvcoop::Log("actors: dev methods enabled ([dev] actorport=1): actors.read actors.dev");
    }
    nvcoop::Log("actors: engine port installed (%zu ledger entries from the last run)", st->ledger.size());
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (!g_st) return;
    switch (type) {
        case NVSEMessagingInterface::kMessage_PreLoadGame: g_st->host.setLoading(true); break;
        case NVSEMessagingInterface::kMessage_PostLoadGame: g_st->host.setLoading(false); g_st->host.bumpEpoch(); break;
        case NVSEMessagingInterface::kMessage_NewGame:
        case NVSEMessagingInterface::kMessage_ExitToMainMenu: g_st->host.setLoading(false); g_st->host.bumpEpoch(); break;
        // xNVSE reference events (only sent by builds that have them): a ref appeared / lost its 3D / a cell finished loading -> scan sooner
        case NVSEMessagingInterface::kMessage_OnRefAttach:
        case NVSEMessagingInterface::kMessage_OnRefSet3D:
        case NVSEMessagingInterface::kMessage_OnRefUnset3D:
        case NVSEMessagingInterface::kMessage_OnCellRefsLoaded: g_st->feed.markDirty(); break;
        default: break;
    }
}

// Host side: story effects the capture hub records (a script ran KillActor / ResurrectActor / set a companion owner). They are queued and
// handed to ActorSync from the frame loop, never from inside the script that produced them (a console line must not nest in a script).
// The guests get the same effects through the coop applier: that path calls engine::onEffect() from plugin/coop (see docs/tools/actor-port.md).
std::vector<fnvcoop::Effect> g_pendingEffects;
bool g_sinkInstalled = false;

void InstallEffectSink() {
    if (g_sinkInstalled) return;
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!hub) return;
    hub->addSink([](fnvcoop::EffectTxn&& t) {
        for (const fnvcoop::Effect& e : t.effects)
            if (std::holds_alternative<fnvcoop::KillActor>(e) || std::holds_alternative<fnvcoop::ResurrectActor>(e) ||
                std::holds_alternative<fnvcoop::CompanionOwner>(e))
                g_pendingEffects.push_back(e);
    });
    g_sinkInstalled = true;
}

void OnFrame() {
    if (!g_st) return;
    InstallEffectSink();
    if (!g_pendingEffects.empty()) {
        std::vector<fnvcoop::Effect> fx;
        fx.swap(g_pendingEffects);
        for (const fnvcoop::Effect& e : fx) onEffect(e);
    }
    g_st->feed.tick(NowMs());
}

}  // namespace

NVCOOP_MODULE(m3_actor_engine, "m3_actor_engine", OnLoad, OnMessage, OnFrame);

// Hot reload (T3b): the old core gives every actor back (AI flags, clones, protection) before it goes; the new core starts its own port.
NVCOOP_RELOAD_STATE(m3_actor_engine, "m3_actor_engine", 1, 40,
    /*canUnload*/ nullptr,
    /*save*/ [](nvreload::Writer&) { return true; },
    /*restore*/ [](uint32_t, nvreload::Reader&) { return true; },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { Teardown(); },
    /*fresh*/ nullptr);
