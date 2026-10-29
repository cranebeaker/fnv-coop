#include "coop_runtime.h"

#include <algorithm>
#include <cctype>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <fstream>
#include <random>
#include <sstream>

#include "common/minijson.h"
#include "flight/flight.h"
#include "effect_json.h"
#include "fnvcoop_mut.h"
#include "fnvcoop_dlg/rules_link.h"
#include "steam_binding.h"

// Mutation twins: RUNTIME-switchable (tools/mut/fnvcoop_mut.h). The test builds (nvcoop_coop_core_rt) read FNVCOOP_MUT=<n>; the
// shipped core has them compiled out. They prove the integration tests can fail:
//   FNVCOOP_MUT=1 : client sink is not wired (received effects never reach the world)
//   FNVCOOP_MUT=2 : host capture hook not wired (captured txns are never committed)
//   FNVCOOP_MUT=3 : host ignores guest actions (onAction never executes)
//   FNVCOOP_MUT=4 : client skips setBaselineWorld (joins without a baseline transfer)
//   FNVCOOP_MUT=9 : a dialogue result script runs outside the result scope (the initiator's player facts never reach the script)
//   FNVCOOP_MUT=10: an ephemeral world effect (fade, sound) is remembered and replayed to a late joiner
//   FNVCOOP_MUT=11: the order channel is not wired (host orders never reach a guest, guest requests never reach the host)
//   FNVCOOP_MUT=12: killPlayer broadcasts the kill order to every guest instead of the one slot
//   FNVCOOP_MUT=20: the cause -> quest map is not loaded (a quest's own script never matches its rules)
//   FNVCOOP_MUT=21: the host never applies the control locks / moves the rules engine commits by itself
//   FNVCOOP_MUT=22: a guest run is not bracketed with the acting-player copies of per-player variables
//   FNVCOOP_MUT=23: anchor tethers are never watched (no near / far reports)
//   FNVCOOP_MUT=24: a guest's stage intent on a pass stage runs at once (no pass, the barrier never sees it)
//   FNVCOOP_MUT=25: the dialogue host is not linked to the rules engine (a ready check / fork INFO runs its end script without asking the party)
//   FNVCOOP_MUT=26: a late joiner is never held or warped
//   FNVCOOP_MUT=27: the combat gate lets every hit through
//   FNVCOOP_MUT=28: a death is never reported to the rules engine
//   FNVCOOP_MUT=29: the tether gather (auto-warp) callback is not wired
//   FNVCOOP_MUT=30: the nvcoop.ini [rules] policy never reaches the rules engine
//   FNVCOOP_MUT=31: a guest reports its grid cell, not the worldspace, for an exterior
//   FNVCOOP_MUT=32: the companion roster does not reach the dialogue registry
//   FNVCOOP_MUT=34: the rules engine's per-player variable views and ChoiceConsent boxes are not connected to the UI bridge
//   FNVCOOP_MUT=35: the roster's party flag is not given to the registry
//   FNVCOOP_MUT=33: the registry's reconcile fixes (shared hired flags) are never committed
//   FNVCOOP_MUT=13..18 are in party_state.h / save_sidecars.h (party state provider, present members, reducer, sidecars, slot registrar)
//   FNVCOOP_MUT=19: the dialogue node ignores the configured companion caps
//   FNVCOOP_MUT=8 : host polls the engine's quest vars only with a world baseline (baseline=0: story vars written by object scripts never leave the host)
//   FNVCOOP_MUT=5 : L11 wiring twin; env COOP_UI_MUTANT=n drops one wiring call (1 bridge config, 2 menu resume, 3 mux 0xE5 channel,
//                   4 SlideshowEnd calls (host + guest), 5 INFO reader install); COOP_LEAVE_MUTANT=n breaks leave()/join()
#ifdef FNVCOOP_RUNTIME_MUT
static bool leaveMut(int n) { const char* e = std::getenv("COOP_LEAVE_MUTANT"); return FNV_MUT(5) && e && std::atoi(e) == n; }   // twins for leave()/join(): 1 leave keeps the session, 2 join ignores the saved session view
static bool uiMut(int n) { const char* e = std::getenv("COOP_UI_MUTANT"); return FNV_MUT(5) && e && std::atoi(e) == n; }
#else
static constexpr bool uiMut(int) { return false; }
static constexpr bool leaveMut(int) { return false; }
#endif

namespace nvcoop::coop {
using nvcoop::json::Value;
using namespace fnvcoop;

const char* roleName(Role r) { return r == Role::Host ? "host" : r == Role::Client ? "client" : "off"; }

namespace {
std::string trim(const std::string& s) {
    size_t b = s.find_first_not_of(" \t\r\n"), e = s.find_last_not_of(" \t\r\n");
    return b == std::string::npos ? std::string() : s.substr(b, e - b + 1);
}
std::string lower(std::string s) {
    for (char& c : s) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    return s;
}
bool parseBool(const std::string& v, bool* out) {
    std::string l = lower(v);
    if (l == "1" || l == "true" || l == "yes" || l == "on") { *out = true; return true; }
    if (l == "0" || l == "false" || l == "no" || l == "off") { *out = false; return true; }
    return false;
}
bool parseU64(const std::string& v, uint64_t* out) {
    if (v.empty()) return false;
    char* end = nullptr;
    unsigned long long n = std::strtoull(v.c_str(), &end, 0);
    if (*end) return false;
    *out = n;
    return true;
}
uint64_t randomToken() {
    std::random_device rd;
    uint64_t t = (uint64_t(rd()) << 32) ^ rd();
    return t ? t : 1;
}
uint32_t sessionId() {
    uint64_t ms = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count();
    uint32_t s = static_cast<uint32_t>(ms / 10);  // strictly increasing across reconnects >10 ms apart
    return s ? s : 1;
}
}  // namespace

bool ParseConfig(const std::string& text, Config* out, std::string* err) {
    Config c = *out;
    std::istringstream in(text);
    std::string line, section;
    int n = 0;
    auto bad = [&](const std::string& m) { if (err) *err = "line " + std::to_string(n) + ": " + m; return false; };
    while (std::getline(in, line)) {
        ++n;
        line = trim(line);
        if (line.empty() || line[0] == ';' || line[0] == '#') continue;
        if (line[0] == '[') { size_t e = line.find(']'); section = lower(e == std::string::npos ? line.substr(1) : line.substr(1, e - 1)); continue; }
        if (section != "coop") continue;
        size_t eq = line.find('=');
        if (eq == std::string::npos) continue;
        std::string k = lower(trim(line.substr(0, eq))), v = trim(line.substr(eq + 1));
        uint64_t u = 0;
        if (k == "enable") { if (!parseBool(v, &c.enable)) return bad("enable must be 0/1"); }
        else if (k == "role") {
            std::string r = lower(v);
            if (r == "host") c.role = Role::Host;
            else if (r == "client" || r == "guest") c.role = Role::Client;
            else if (r == "off" || r.empty()) c.role = Role::Off;
            else return bad("role must be host|client|guest|off");
        }
        else if (k == "listen") c.listen = v;
        else if (k == "host" || k == "hostaddress") c.host = v;
        else if (k == "port") { if (!parseU64(v, &u) || u == 0 || u > 65535) return bad("port out of range"); c.port = static_cast<uint16_t>(u); }
        else if (k == "transport") { std::string t = lower(v); if (t != "enet" && t != "udp" && t != "steam") return bad("transport must be enet|udp|steam"); c.transport = t; }
        else if (k == "forward_activate") { if (!parseBool(v, &c.forwardActivate)) return bad("forward_activate must be 0/1"); }
        else if (k == "clientload") { std::string t = lower(v); if (t != "rejoin" && t != "block") return bad("clientload must be rejoin|block"); c.clientLoadBlock = t == "block"; }
        else if (k == "rules") c.rulesFile = v;
        else if (k == "handshake") { if (!parseBool(v, &c.handshake)) return bad("handshake must be 0/1"); }
        else if (k == "worker") { if (!parseBool(v, &c.useWorker)) return bad("worker must be 0/1"); }
        else if (k == "dialogue") { if (!parseBool(v, &c.dialogue)) return bad("dialogue must be 0/1"); }
        else if (k == "baseline") { if (!parseBool(v, &c.baseline)) return bad("baseline must be 0/1"); }
        else if (k == "token") { if (!parseU64(v, &u)) return bad("token must be a number"); c.token = u; }
        else if (k == "capture") { if (!parseBool(v, &c.captureEnabled)) return bad("capture must be 0/1"); }
        else if (k == "apply") { if (!parseBool(v, &c.applyEnabled)) return bad("apply must be 0/1"); }
        else if (k == "maxplayers") { if (!parseU64(v, &u) || u < 1 || u > kMaxSlots) return bad("maxPlayers 1..8"); c.maxPlayers = static_cast<uint32_t>(u); }
        else if (k == "party") { if (!v.empty() && (!parseU64(v, &u) || u > kMaxSlots)) return bad("party 0..8"); c.party = v.empty() ? 0 : static_cast<int>(u); }
    }
    *out = c;
    return true;
}

// ---- client sink ----------------------------------------------------------------------------------------------
class Runtime::Sink : public IEffectSink {
public:
    explicit Sink(Runtime& rt) : rt_(rt) {}
    void apply(const Effect& e, const Context& ctx) override {
        if (!open_ || cur_.seq != ctx.seq || ctx.index == 0) {
            flush();
            cur_ = LogEntry();
            cur_.dir = ctx.repair ? "repair" : ctx.baseline ? "baseline" : "apply";
            cur_.seq = ctx.seq;
            cur_.origin = ctx.origin;
            if (ctx.cause) cur_.cause = *ctx.cause;
            cur_.atMs = rt_.now_();
            open_ = true;
        }
        cur_.effects.push_back(e);
        ApplyResult r;
        if (FNV_MUT(1)) {
            r.status = "MutantNotWired";
        } else {
            if (rt_.cfg_.applyEnabled && rt_.world_) r = rt_.world_->apply(e, ctx);
            else r.status = "ApplyDisabled";
        }
        if (ctx.baseline) ++rt_.counters_.baselineEffects;
        {
            std::string key = std::string(ctx.repair ? "repair " : ctx.baseline ? "baseline " : ctx.seq == 0 ? "snapshot " : "live ") + effectName(e) + ":" + r.status.substr(0, r.status.find(':'));
            ++rt_.tally_[key];
            if (!r.ok && rt_.firstErrors_.size() < 12 && r.status.find("Unsupported") == std::string::npos) {
                FormRef q;
                rt_.firstErrors_.push_back(key + " | " + r.status + " | " + effectName(e));
            }
        }
        if (r.ok) ++rt_.counters_.applied; else ++rt_.counters_.applyFailed;
        cur_.results.push_back(r.status);
        if (auto* co = std::get_if<CompanionOwner>(&e)) rt_.noteCompanion(*co);   // L8 registry: per-owner follower views
        if (rt_.uiEnabled_ && rt_.ui_.slideClient && !uiMut(4) && std::holds_alternative<SlideshowEnd>(e)) {   // L11: releases the slideshow control lock
            rt_.ui_.slideClient().onSlideshowEndEffect();
            ++rt_.uiStats_.slideEnds;
        }
    }
    void onJoined(PlayerId slot, const SharePolicy& p) override {
        if (rt_.world_) rt_.world_->onJoined(slot, p);
    }
    void onBaselineComplete(const BaselineResult& r) override {
        flush();
        LogEntry le;
        le.dir = "baselineDone";
        le.atMs = rt_.now_();
        le.results.push_back("effects=" + std::to_string(r.effectsApplied) + " reverted=" + std::to_string(r.reverted) +
                             " unrevertable=" + std::to_string(r.unrevertable) + " kept=" + std::to_string(r.entriesKept));
        rt_.pushLog(std::move(le));
    }
    void flush() {
        if (open_) { rt_.pushLog(std::move(cur_)); open_ = false; }
    }

private:
    Runtime& rt_;
    LogEntry cur_;
    bool open_ = false;
};

// ---- runtime --------------------------------------------------------------------------------------------------
Runtime::Runtime(Config cfg, nvhooks::CaptureHub* hub, IWorldApplier* world, std::function<uint64_t()> nowMs)
    : cfg_(std::move(cfg)), hub_(hub), world_(world), now_(std::move(nowMs)) {
    installSink_ = [hub](nvhooks::CaptureHub::Sink s) { if (hub) hub->addSink(std::move(s)); };  // L5 keeps slot 0 (bridge capture.end)
}

Runtime::~Runtime() { stop(); }

void Runtime::pushLog(LogEntry e) {
    if (onLog) onLog(e);
    log_.push_back(std::move(e));
    while (log_.size() > cfg_.logCapacity) log_.pop_front();
}

void Runtime::onCaptured(EffectTxn&& t) {
    ++counters_.captured;
    if (!captureOn_) return;
    // the rules engine keys on the OWNING QUEST: an INFO result script's cause.source is the INFO, map it to its quest
    if (t.cause.kind == CauseKind::DialogueResult && infoQuest_) {
        FormRef q;
        if (infoQuest_(t.cause.source, q)) t.cause.source = q;
    }
    if (FNV_MUT(2)) {
        (void)t;
        return;
    } else {
        if (!host_) return;
        capturedSnapshot_ = t.effects;
        inCapturedCommit_ = true;
        uint64_t seq = host_->commit(actionOrigin_ != kNoPlayer ? actionOrigin_ : t.origin, t.cause, std::move(t.effects), t.flags, t.originEpoch, t.originIntent);
        inCapturedCommit_ = false;
        capturedSnapshot_.clear();
        if (seq) ++counters_.committed; else ++counters_.droppedEmpty;
    }
}

bool Runtime::makeTransport(bool server, std::string* err) {
    if (cfg_.transport == "udp") {
        auto t = server ? UdpTransport::Server(cfg_.listen, cfg_.port, err) : UdpTransport::Client(cfg_.host, cfg_.port, err);
        if (!t) return false;
        udp_ = t.get();
        transport_ = std::move(t);
        return true;
    }
    if (cfg_.transport == "steam") {
        std::string why;
        fnvcoop::ISteamApi* api = nullptr;
        if (steamProvider_) api = steamProvider_(&why);
        else if (SteamBinding::instance().attachToGame(&why)) api = &SteamBinding::instance();
        if (!api) { if (err) *err = why.empty() ? "Steam is not available" : why; return false; }
        auto t = std::make_unique<fnvcoop::SteamTransport>(*api);
        bool ok;
        if (server) {
            if (!FNV_MUT(1271)) api->setHostTag(std::to_string(fnvcoop::kProtocolVersion), "");   // MUTANT 1271: friends are never told this host is joinable
            ok = t->listen(8);
        } else {
            char* end = nullptr;
            const unsigned long long id = std::strtoull(cfg_.host.c_str(), &end, 10);
            if (cfg_.host.empty() || *end || id < 76561197960265728ull) { if (err) *err = "host must be the host's Steam ID (a 17-digit number) with transport=steam"; return false; }
            ok = t->connect(id);
        }
        if (!ok) { if (err) *err = "steam " + std::string(server ? "listen" : "connect") + " failed"; return false; }
        steam_ = t.get();
        link_ = FNV_MUT(1270) ? nullptr : t.get();   // MUTANT 1270: the Steam link's events are not wired (no Disconnected for the host, no reconnect for a guest)
        transport_ = std::move(t);
        return true;
    }
    auto t = std::make_unique<fnvcoop::EnetTransport>();
    // ClientSession keeps sending Hello until Welcome, so it is fine that the ENet connect completes asynchronously.
    bool ok = server ? t->listen(cfg_.port, 8, cfg_.listen == "127.0.0.1" || cfg_.listen == "localhost") : t->connect(cfg_.host, cfg_.port);
    if (!ok) { if (err) *err = "enet " + std::string(server ? "listen" : "connect") + " failed"; return false; }
    enet_ = t.get();
    link_ = t.get();
    transport_ = std::move(t);
    return true;
}

bool Runtime::start(std::string* err) {
    if (running_) return true;
    std::string e;
    startMs_ = now_();
    if (cfg_.role == Role::Host) {
        if (!makeTransport(true, &e)) { lastError_ = "host transport: " + e; if (err) *err = lastError_; return false; }
        mux_.reset(new MuxTransport(sessionTransport()));
        SharePolicy pol;
        pol.maxPlayers = static_cast<uint8_t>(cfg_.maxPlayers);
        if (cfg_.clientLoadBlock) pol.clientLoad = SharePolicy::ClientLoad::Block;
        host_.reset(new HostSession(*mux_, pol));
        host_->onCommit = [this](const EffectTxn& t) {
            LogEntry le;
            le.dir = "commit";
            le.seq = t.seq;
            le.origin = t.origin;
            le.cause = t.cause;
            le.effects = t.effects;
            le.atMs = now_();
            pushLog(std::move(le));
            if (flight_) flight_->captured(0, 0, t.seq, t.effects, t.cause.kind, host_ ? host_->epoch() : 0);
            for (const Effect& e : t.effects)
                if (auto* co = std::get_if<CompanionOwner>(&e)) noteCompanion(*co);
            for (const Effect& e : t.effects)
                if (!uiMut(4) && std::holds_alternative<SlideshowEnd>(e)) uiSlideEnded();   // L11: End packet to the guests
            onHostCommitted(t);
        };
        if (cfg_.baseline) {
            if (!dumper_) { lastError_ = "baseline=1 but no world dumper"; if (err) *err = lastError_; return false; }
            ShadowState b;
            uint64_t hash = 0;
            std::string derr;
            if (!dumper_->dump(&b, &hash, &derr)) { lastError_ = "baseline dump: " + derr; if (err) *err = lastError_; return false; }
            host_->setWorldBaseline(b, hash);
            usedBaseline_.reset(new ShadowState(b));
            usedHash_ = hash;
            // Desync repair (issue #25): quests a client reports a running state for that the host's shared world lacks are settled with
            // the host engine's own running state, read through a fresh dump (rare: once committed the keys are part of the shared world).
            IWorldDumper* qd = dumper_;
            host_->hostQuestRunning = [qd](const std::vector<FormRef>& qs, std::map<FormRef, bool>& out) {
                ShadowState now;
                uint64_t nowHash = 0;
                std::string why;
                if (!qd->dump(&now, &nowHash, &why)) return false;
                for (auto& q : qs) { auto it = now.questRunning.find(q); if (it != now.questRunning.end()) out[q] = it->second; }
                return true;
            };
        }
        if (!host_->hostQuestRunning && questRunning_) {
            // No world baseline to dump from: read the running flag of just the quests a client reported, straight from this engine. Without a
            // reader the session falls back to the completeness default (not running) and stops every quest the client is running.
            QuestRunningReader rd = questRunning_;
            host_->hostQuestRunning = [rd](const std::vector<FormRef>& qs, std::map<FormRef, bool>& out) {
                for (auto& q : qs) {
                    bool on = false;
                    if (rd(q, on)) out[q] = on;
                }
                return true;
            };
        }
        if (globals_) {
            GlobalReader rd = globals_;
            host_->hostGlobals = [rd](const std::vector<FormRef>& gs, std::map<FormRef, double>& out) {
                for (auto& g : gs) {
                    double v = 0;
                    if (rd(g, v)) out[g] = v;
                }
                return true;
            };
        }
        if (questState_) {
            // Desync repair: client-only quest state (non-zero stage, completed, failed) is settled only where THIS engine confirms it.
            // Works with or without a world baseline (a baseline-less host never recorded what both copies loaded from the same save).
            QuestStateReader rd = questState_;
            host_->hostQuestState = [rd](const std::vector<FormRef>& qs, std::map<FormRef, HostSession::HostQuestState>& out) {
                for (auto& q : qs) {
                    HostSession::HostQuestState s;
                    int32_t stage = -1;
                    if (!rd(q, stage, s.completed, s.failed)) continue;
                    s.stage = stage;
                    out[q] = s;
                }
                return true;
            };
        }
        if (objectiveState_) {
            // Same for client-only objective state (non-zero flags, all objectives completed): settled only where THIS engine shows the same.
            ObjectiveStateReader rd = objectiveState_;
            host_->hostObjectives = [rd](const std::vector<FormRef>& qs, std::map<FormRef, std::map<uint32_t, uint8_t>>& out) {
                for (auto& q : qs) {
                    std::map<uint32_t, uint8_t> objs;
                    if (rd(q, objs)) out[q] = std::move(objs);
                }
                return true;
            };
        }
        if (scriptVars_) {
            // Same for client-only script vars: 0 is adopted as the default unless this engine shows another value, non-zero only where it shows the same.
            ScriptVarReader rd = scriptVars_;
            host_->hostScriptVars = [rd](const std::vector<std::pair<FormRef, uint16_t>>& vars, std::map<std::pair<FormRef, uint16_t>, double>& out) {
                std::map<FormRef, std::map<uint16_t, double>> byOwner;
                for (auto& v : vars) {
                    auto o = byOwner.find(v.first);
                    if (o == byOwner.end()) {
                        std::map<uint16_t, double> got;
                        o = byOwner.emplace(v.first, rd(v.first, got) ? std::move(got) : std::map<uint16_t, double>()).first;
                    }
                    auto it = o->second.find(v.second);
                    if (it != o->second.end()) out[v] = it->second;
                }
                return true;
            };
        }
        {
            HostSession::DesyncOptions dopt;
            dopt.compareClasses = cfg_.compareClasses;
            host_->setDesyncOptions(dopt);
        }
        replicator_.reset(new StateReplicator(*host_));
        if (!haveRules_ && !cfg_.rulesFile.empty()) {
            std::string rerr;
            if (!rulesDb_.loadFile(cfg_.rulesFile, &rerr)) { lastError_ = "rules: " + rerr; if (err) *err = lastError_; return false; }
            haveRules_ = true;
        }
        if (haveRules_) {
            EdidResolver res = edid_ ? edid_ : EdidResolver([](const std::string&, FormRef&) { return false; });
            RulesEngine::Options ropt;
            ropt.playerRef = [this](PlayerId slot, FormRef& out) {
                if (playerRef_) return playerRef_(slot, out);
                if (slot == kHostPlayer) { out = FormRef("falloutnv.esm", 0x14); return true; }   // the host's own player ref
                return false;                                                                    // guests: needs a ghost ref (L6)
            };
            if (!FNV_MUT(30)) ropt.policy = policy_;
            // W3d item 1: what a slot holds (pooled hand-ins) and how far two players are apart (a ranged PartyWarp: the Old World Blues Teleporter).
            if (!FNV_MUT(39)) ropt.itemCount = [this](PlayerId slot, const FormRef& item, int64_t& count) {
                if (!itemCountProbe_) return false;
                ++rulesStats_.itemCountAsks;
                return itemCountProbe_(slot, item, count);
            };
            if (!FNV_MUT(40)) ropt.playerDistance = [this](PlayerId a, PlayerId b, float& dist) {
                if (!anchorProbe_.slotPos) return false;
                float ax = 0, ay = 0, az = 0, bx = 0, by = 0, bz = 0;
                if (!anchorProbe_.slotPos(a, ax, ay, az) || !anchorProbe_.slotPos(b, bx, by, bz)) return false;
                ++rulesStats_.distanceAsks;
                const float dx = ax - bx, dy = ay - by, dz = az - bz;
                dist = std::sqrt(dx * dx + dy * dy + dz * dz);
                return true;
            };
            rules_.reset(new RulesEngine(*host_, rulesDb_, res, ropt));
            rules_->applyStage = [this](const FormRef& q, uint32_t st) {
                if (!stageApplier_) return;
                applyingDeferred_ = true;
                stageApplier_(q, st);
                applyingDeferred_ = false;
            };
            buildRulesIndexes();
            wireRulesCallbacks();
        }
        if (cfg_.useWorker) { worker_.reset(new ThreadWorker()); host_->setWorker(worker_.get()); }
        if (cfg_.handshake) {
            HandshakeInfo hs = hsBase_;
            if (haveRules_) hs.rulesHash = rulesDb_.textHash();
            hs.volatileHash = volatileKeys().contentHash();
            host_->setHandshake(hs);
        }
        host_->onAction = [this](PlayerId slot, IntentKind kind, const FormRef& ref, uint32_t, uint32_t) {
            ++counters_.actions;
            if (rules_ && rules_->handleAction(slot, kind, ref)) { ++counters_.rulesActions; return; }
            // Only interactions are run on the host. Presence reports (ReportCell / ReportNear / ReportFar) and ScenePass only feed the
            // rules engine: without one (or when it does not consume them) they are dropped. Their ref is a cell, an anchor or a quest,
            // and running "Activate" on a cell hung the host (two-copy fight test 2026-10-21).
            if (!FNV_MUT(7) && kind != IntentKind::ActivateRef && kind != IntentKind::PickUp && kind != IntentKind::TriggerEnter) {
                ++counters_.reportsDropped;
                return;
            }
            if (FNV_MUT(3)) {
                (void)slot; (void)kind; (void)ref;   // twin: host ignores guest actions
            } else {
                if (!executor_) return;
                actionOrigin_ = slot;
                if (actingHook_) actingHook_(slot, true);
                actingBegin(slot);
                executor_(slot, kind, ref);
                actingEnd(slot);
                if (actingHook_) actingHook_(slot, false);
                actionOrigin_ = kNoPlayer;
            }
        };
        if (hub_ && cfg_.captureEnabled) {
            if (!sinkInstalled_) {
                sinkInstalled_ = true;  // sinks cannot be removed from the hub: install once, gate with captureOn_
                std::weak_ptr<bool> alive = alive_;  // the hub outlives us in-game; never call into a dead Runtime
                installSink_([this, alive](EffectTxn&& t) { if (!alive.expired()) onCaptured(std::move(t)); });
            }
            hub_->setEnabled(true);
            captureOn_ = true;
        }
    } else if (cfg_.role == Role::Client) {
        if (!makeTransport(false, &e)) { lastError_ = "client transport: " + e; if (err) *err = lastError_; return false; }
        if (!cfg_.token) cfg_.token = randomToken();
        mux_.reset(new MuxTransport(sessionTransport()));
        sink_.reset(new Sink(*this));
        IEffectSink* sessionSink = sink_.get();
        if (flight_) { flightS_ = std::make_shared<fnvcoop::flight::RecordingSink>(*sink_, *flight_); sessionSink = flightS_.get(); }
        client_.reset(new ClientSession(*mux_, 0, cfg_.token, *sessionSink, hub_ ? &hub_->echo() : nullptr));
        if (!FNV_MUT(4)) { // twin 4: client never hands its own world to the session -> no baseline transfer
            if (cfg_.baseline) {
                if (!dumper_) { lastError_ = "baseline=1 but no world dumper"; if (err) *err = lastError_; return false; }
                ShadowState c;
                uint64_t hash = 0;
                std::string derr;
                if (!dumper_->dump(&c, &hash, &derr)) { lastError_ = "baseline dump: " + derr; if (err) *err = lastError_; return false; }
                client_->setBaselineWorld(c, hash);
            }
        }
        if (!FNV_MUT(4)) {
            if (dumper_) {   // Rebase / loading a save without a usable sidecar re-dumps the CURRENT world
                IWorldDumper* d = dumper_;
                client_->setBaselineProvider([this, d](ShadowState& w, uint64_t& h) {
                    std::string e;
                    bool ok = d->dump(&w, &h, &e);
                    if (ok && flight_) flightWorld_.reset(new ShadowState(w));   // T1a: the real world the desync digest was built from (pre-repair)
                    return ok;
                });
            }
        }
        client_->setDigestInterval(cfg_.digestIntervalMs);
        if (cfg_.useWorker) { worker_.reset(new ThreadWorker()); client_->setWorker(worker_.get()); }
        if (cfg_.handshake) {
            HandshakeInfo hs = hsBase_;
            if (!cfg_.rulesFile.empty() && !haveRules_) {   // the host compares rules.json text hashes: hash ours the same way
                std::string rerr;
                if (rulesDb_.loadFile(cfg_.rulesFile, &rerr)) haveRules_ = true;
                else { lastError_ = "rules: " + rerr; if (err) *err = lastError_; return false; }
            }
            if (haveRules_) hs.rulesHash = rulesDb_.textHash();
            hs.volatileHash = volatileKeys().contentHash();
            client_->setHandshake(hs);
        }
        client_->connect(now_(), sessionId());
    } else {
        lastError_ = "role is off";
        if (err) *err = lastError_;
        return false;
    }
    if (dlgEnabled_ && mux_) {
        mux_->registerChannel(kChannelDialogue, [this](PeerId from, Bytes&& d) {
            if (dlg_) dlg_->onPacket(from, d, static_cast<uint32_t>(now_()));
        });
    }
    if (client_) client_->onApp = [this](PlayerId from, uint16_t ch, const Bytes& d) {
        if (ch == kAppWorldChannel) onWorldPacket(d);
        else if (ch == kAppOrderChannel && !FNV_MUT(11)) onOrderPacket(from, d);
    };
    if (host_) host_->onApp = [this](PlayerId from, uint16_t ch, const Bytes& d) { if (ch == kAppOrderChannel && !FNV_MUT(11)) onOrderPacket(from, d); };
    if (uiEnabled_ && mux_) {
        if (cfg_.role == Role::Client && ui_.slideClient && !uiMut(3)) {   // protocol.md section 2: slideshow channel 0xE5
            mux_->registerChannel(nvui::kSlideChannel, [this](PeerId, Bytes&& d) {
                if (!ui_.slideClient) return;
                ++uiStats_.slidePackets;
                ui_.slideClient().onPacket(d, now_());
            });
        }
        if (ui_.installInfoReader && ui_.infoReader && !uiMut(5)) ui_.installInfoReader(ui_.infoReader);   // captured SayTo carries the INFO text
    }
    running_ = true;
    active_ = this;
    return true;
}

bool Runtime::runAsGuest(PlayerId slot, const std::function<void()>& body) {
    if (!host_ || !body || slot == kNoPlayer) return false;
    if (actionOrigin_ == slot) { body(); return true; }   // already inside this guest's window (the story-channel executor): no second on / off pair
    struct Window {   // closes the window even when body() throws (a test fake, never the game)
        Runtime& rt; PlayerId prev, slot;
        ~Window() { if (rt.actingHook_) rt.actingHook_(slot, false); rt.actionOrigin_ = prev; }
    } w{*this, actionOrigin_, slot};
    actionOrigin_ = slot;
    if (actingHook_) actingHook_(slot, true);
    actingBegin(slot);
    body();
    actingEnd(slot);
    return true;
}

void Runtime::stop() {
    if (active_ == this) active_ = nullptr;
    if (!running_) return;
    if (uiBridged_ && ui_.router) {
        if (nvui::UiRouter* r = ui_.router()) r->bridge() = nvui::Bridge();   // the bridge captures the DialogueNode
    }
    uiBridged_ = false;
    uiWasJoined_ = false;
    uiJoined_.clear();
    actingQuests_.clear();
    anchorRadius_.clear();
    anchorWatch_.clear();
    lateJoinSeen_.clear();
    lateJoinPending_.clear();
    lateJoinLocked_.clear();
    if (link_) link_->disconnect();   // graceful: peers see Disconnected now instead of a timeout
    if (cfg_.role == Role::Host) captureOn_ = false;
    if (dlg_) { dlg_->setRulesGates(nullptr, nullptr); dlg_->setVoteSink(nullptr); }   // before the engine they point at is destroyed
    rulesLinked_ = false;
    rules_.reset();
    replicator_.reset();
    client_.reset();
    host_.reset();
    worker_.reset();   // joins the thread after queued jobs finished (sessions are gone: no result will be drained)
    dlg_.reset();
    companionOwners_.clear();
    worldState_.clear();
    worldJoined_.clear();
    worldQueue_.clear();
    worldApplied_.clear();
    dlgJoined_.clear();
    dlgWasLive_ = false;
    client_.reset();
    sink_.reset();
    host_.reset();
    mux_.reset();
    if (flight_) flight_->dumpToDir(fnvcoop::flight::kDumpShutdown);
    flightS_.reset();
    flightT_.reset();
    transport_.reset();
    udp_ = nullptr;
    enet_ = nullptr;
    steam_ = nullptr;
    link_ = nullptr;
    running_ = false;
}

bool Runtime::leave(std::string* err) {
    if (!running_) { if (err) *err = left_ ? "already left the session" : "coop not running"; return false; }
    leaveView_.clear();
    if (client_ && client_->canSaveState()) leaveView_ = client_->saveState();   // what this machine already received: join resumes from it
    if (!leaveMut(1)) stop();
    left_ = true;
    LogEntry le;
    le.dir = "leave";
    le.atMs = now_();
    pushLog(std::move(le));
    return true;
}

bool Runtime::join(const std::string& host, uint16_t port, std::string* err) {
    if (running_) { if (err) *err = "already in a session: coop.leave first"; return false; }
    if (cfg_.role == Role::Client) {
        if (!host.empty()) cfg_.host = host;
        if (port) cfg_.port = port;
    }
    if (!start(err)) return false;
    // A rejoin is a local save load of the view saved at leave: missed host txns arrive as a diff and the rewards already received are not
    // granted again (a plain baseline join would send this slot's whole reward ledger on top of what the world already holds).
    if (client_ && !leaveView_.empty() && !leaveMut(2)) client_->onLocalSaveLoaded(now_(), &leaveView_);
    leaveView_.clear();
    left_ = false;
    LogEntry le;
    le.dir = "join";
    le.atMs = now_();
    pushLog(std::move(le));
    return true;
}

Bytes Runtime::saveSidecar() {
    Bytes b;
    if (host_) b = host_->saveSidecar();
    else if (client_ && client_->canSaveState()) b = client_->saveState();
    else if (!client_ && left_) b = leaveView_;   // saved while out of the session: the save still records what this world already received
    if (!b.empty()) ++counters_.saves;
    LogEntry le;
    le.dir = "save";
    le.atMs = now_();
    le.results.push_back("sidecar bytes=" + std::to_string(b.size()));
    pushLog(std::move(le));
    return b;
}

void Runtime::bindPlayerRefs(std::function<uint32_t(PlayerId)> ghostOf, std::function<bool(uint32_t, FormRef&)> idToRef) {
    ghostOf_ = std::move(ghostOf);
    idToRef_ = std::move(idToRef);
    ++playerRefBinds_;
    playerRef_ = [this](PlayerId slot, FormRef& out) {
        uint32_t id = slot == kHostPlayer ? 0x14u : (ghostOf_ ? ghostOf_(slot) : 0u);
        if (!id) return false;
        return idToRef_ ? idToRef_(id, out) : false;
    };
}

// Test tooling: the host's world was changed outside the session (a test helper reset quests), so its shared state no longer
// describes the world. Re-dump it as a new world epoch, exactly like a host load: guests rebase onto it.
bool Runtime::rebaselineHost(std::string* what) {
    std::string w;
    bool ok = false;
    if (!running_ || !host_) w = "not a running host";
    else if (!dumper_) w = "no world dumper";
    else {
        ShadowState st;
        uint64_t hash = 0;
        std::string derr;
        if (dumper_->dump(&st, &hash, &derr) && host_->reloadWorld(st, hash)) {
            ok = true;
            usedBaseline_.reset(new ShadowState(host_->world()));
            if (replicator_) replicator_->clear();
            w = "host: world re-dumped (epoch " + std::to_string(host_->epoch()) + ")";
        } else w = "host: re-dump failed: " + derr;
    }
    if (what) *what = w;
    return ok;
}

bool Runtime::onGameLoaded(const Bytes* sidecar, std::string* what) {
    if (ghostOf_) bindPlayerRefs(ghostOf_, idToRef_);    // re-assert after a load / rebase (ghosts are respawned)
    std::string w;
    bool ok = false;
    if (!running_) { w = "runtime not running"; }
    else if (!cfg_.baseline) { w = "save/load handling is off (baseline=0)"; }
    else if (host_) {
        if (sidecar && !sidecar->empty() && host_->loadSidecar(*sidecar)) { ok = true; w = "host: sidecar restored"; }
        else if (dumper_) {
            ShadowState st;
            uint64_t hash = 0;
            std::string derr;
            if (dumper_->dump(&st, &hash, &derr) && host_->reloadWorld(st, hash)) { ok = true; w = "host: reloaded world from a fresh engine dump"; }
            else w = "host: reload failed: " + derr;
        } else w = "host: no sidecar and no dumper";
        if (ok) { usedBaseline_.reset(new ShadowState(host_->world())); }
        if (ok) w += " (epoch " + std::to_string(host_->epoch()) + ")";
    } else if (client_) {
        if (!client_->clientLoadAllowed()) { w = "client: load refused by host policy (clientload=block)"; ++counters_.loadsBlocked; }
        else if (client_->onLocalSaveLoaded(now_(), sidecar && !sidecar->empty() ? sidecar : nullptr)) {
            ok = true;
            w = sidecar && !sidecar->empty() ? "client: resumed from sidecar or re-baselining" : "client: re-baselining (no sidecar)";
        } else { w = "client: onLocalSaveLoaded refused"; ++counters_.loadsBlocked; }
    }
    if (ok && replicator_) replicator_->clear();   // queued writes belong to the replaced world
    if (ok) ++counters_.loads;
    LogEntry le;
    le.dir = "load";
    le.atMs = now_();
    le.results.push_back(w);
    pushLog(std::move(le));
    if (what) *what = w;
    return ok;
}

bool Runtime::setRules(const std::string& jsonText, std::string* err) {
    if (!rulesDb_.load(jsonText, err)) return false;
    haveRules_ = true;
    return true;
}

bool Runtime::stageRequested(const FormRef& quest, uint32_t stage, PlayerId initiator) {
    if (!rules_ || applyingDeferred_) return true;
    // G26: the stage a Barrier rule names with `stages` (chargen end) is the player's own step; the host reaching it is the host's pass.
    if (initiator == kHostPlayer && !FNV_MUT(24))
        if (const QuestRules* qr = rules_->rulesFor(quest))
            if (const Rule* br = qr->findFor("Barrier", stage))
                if (br->lists.count("stages") && rules_->isPassStage(quest, stage))
                    return rules_->onGuestStageIntent(kHostPlayer, quest, stage) == StageDecision::Proceed;
    return rules_->onStageRequested(quest, stage, initiator) == StageDecision::Proceed;
}

bool Runtime::stageHeldForPass(PlayerId slot, const FormRef& quest, uint32_t stage) {
    if (!rules_ || FNV_MUT(24) || !rules_->isPassStage(quest, stage)) return false;
    return rules_->onGuestStageIntent(slot, quest, stage) == StageDecision::Deferred;
}

Runtime::HitVerdict Runtime::protectedHit(const FormRef& victim, PlayerId attacker, bool splash) {
    if (!rules_ || FNV_MUT(27)) return HitVerdict::Allow;
    switch (rules_->onProtectedHit(victim, attacker, splash, now_())) {
        case RulesEngine::FfVerdict::Block: return HitVerdict::Block;
        case RulesEngine::FfVerdict::BlockWarn:
            notifyPlayer(attacker, "That would end the quest for everyone. Hit it on purpose three times to go through with it.");
            return HitVerdict::BlockWarn;
        default: return HitVerdict::Allow;
    }
}

bool Runtime::stageExcluded(const FormRef& quest, uint32_t stage) const {
    return rules_ && rules_->stageExcluded(quest, stage);
}

void Runtime::reportCell(const FormRef& cell) {
    if (rules_) rules_->onCellReported(kHostPlayer, cell);
    else if (client_ && client_->joined() && !client_->baselineActive()) client_->submitAction(IntentKind::ReportCell, cell);
}

void Runtime::reportCell(const FormRef& cell, const FormRef& world) {
    if (rules_) rules_->onCellReported(kHostPlayer, cell, world);
    else if (client_ && client_->joined() && !client_->baselineActive()) client_->submitAction(IntentKind::ReportCell, world.plugin.empty() || FNV_MUT(31) ? cell : world);
}

void Runtime::scenePass(const FormRef& quest) {
    if (rules_) rules_->onPlayerPassed(kHostPlayer, quest);
    else if (client_ && client_->joined() && !client_->baselineActive()) client_->submitAction(IntentKind::ScenePass, quest);
}

bool Runtime::saveSuppressed() const { return client_ && client_->state().saveSuppress; }

uint32_t Runtime::submitAction(IntentKind kind, const FormRef& ref) {
    if (!client_ || !client_->joined() || client_->baselineActive()) return 0;
    return client_->submitAction(kind, ref);
}

void Runtime::tickDialogue() {
    if (!dlgEnabled_) return;
    uint32_t now = static_cast<uint32_t>(now_());
    if (host_) {
        if (!dlg_) {
            fnvcoop_dlg::DialogueNode::Config c;
            c.self = kHostPlayer;
            c.isHost = true;
            c.appliedSeq = [this]() -> uint64_t { return host_ ? host_->log().head() : 0; };
            c.peerOfSlot = [this](PlayerId p) {
                for (auto& cv : host_->clients()) if (cv.slot == p) return cv.peer;
                return PeerId(~0u);
            };
            c.slotOfPeer = [this](PeerId peer) {
                for (auto& cv : host_->clients()) if (cv.peer == peer && cv.joined) return cv.slot;
                return kNoPlayer;
            };
            dlg_.reset(new fnvcoop_dlg::DialogueNode(mux_.get(), c, [this](const fnvcoop_dlg::ResultRequest& rq) -> uint64_t { return dialogueResult(rq); }, dlgHooks_.sink));
            if (dlgHooks_.companionRules && !FNV_MUT(19)) dlg_->setCompanionRules(*dlgHooks_.companionRules);   // twin 19: the node keeps the built-in caps
            syncCompanions();
        }
        if (dlg_ && rules_ && !rulesLinked_ && !FNV_MUT(25)) {   // G08 / G27: INFO end scripts and fork boxes ask the party through the rules engine (set own callbacks BEFORE this)
            fnvcoop_dlg::linkRules(*dlg_, *rules_, [this]() { return static_cast<uint32_t>(now_()); });
            rulesLinked_ = true;
            ++rulesStats_.rulesLinks;
        }
        std::set<PlayerId> joinedNow;
        for (auto& cv : host_->clients()) if (cv.joined) joinedNow.insert(cv.slot);
        for (PlayerId p : joinedNow) if (!dlgJoined_.count(p)) dlg_->playerJoined(p, now);
        for (PlayerId p : dlgJoined_) if (!joinedNow.count(p)) dlg_->playerLeft(p, now);
        dlgJoined_ = joinedNow;
    } else if (client_ && client_->joined() && !client_->baselineActive()) {
        if (!dlg_) {
            fnvcoop_dlg::DialogueNode::Config c;
            c.self = client_->slot();
            c.isHost = false;
            c.hostPeer = 0;
            c.appliedSeq = [this]() -> uint64_t { return client_ ? client_->applied() : 0; };
            dlg_.reset(new fnvcoop_dlg::DialogueNode(mux_.get(), c, nullptr, dlgHooks_.sink));
            if (dlgHooks_.companionRules && !FNV_MUT(19)) dlg_->setCompanionRules(*dlgHooks_.companionRules);
            syncCompanions();
        }
    }
    if (dlg_ && client_) {   // the story session (re)joined or finished a (re)baseline: drop any stale conversation role
        bool live = client_->joined() && !client_->baselineActive();
        if (live && !dlgWasLive_) dlg_->resync(now);
        dlgWasLive_ = live;
    }
    if (dlg_) dlg_->tick(now);
}

void Runtime::frame() {
    if (!running_) return;
    ++counters_.frames;
    uint64_t t = now_();
    if (host_) {
        host_->tick(t);
        if (link_) {
            fnvcoop::ILinkTransport::Event ev;
            while (link_->nextEvent(ev)) {
                if (ev.type == fnvcoop::ILinkTransport::Event::Disconnected) { ++counters_.disconnects; host_->peerDisconnected(ev.peer); }
            }
        }
        if (rules_) { rules_->tick(t); tickRulesSession(t); }
        if (replicator_) {
            if (statePoll_ && (cfg_.baseline || !FNV_MUT(8))) {   // twin 8: the old gate, polling only with a world baseline
                const bool first = polledRep_ != replicator_.get();
                polledRep_ = replicator_.get();
                statePoll_(*replicator_, first, first && !cfg_.baseline);
            }
            replicator_->flush(t);
        }
    }
    if (client_) {
        if (link_) {
            // The host link dropped (a stall or long load past the peer timeout, a network blip): re-create it and rejoin. The session
            // keeps its shadow, so the host replays what it missed (or sends a snapshot) instead of a full rejoin.
            fnvcoop::ILinkTransport::Event ev;
            while (link_->nextEvent(ev)) {
                if (ev.type == fnvcoop::ILinkTransport::Event::Disconnected) {
                    ++counters_.disconnects;
                    client_->disconnect();
                    reconnectAt_ = t + kReconnectDelayMs;
                }
            }
            if (reconnectAt_ && t >= reconnectAt_) {
                if (link_->reconnect()) {
                    reconnectAt_ = 0;
                    ++counters_.reconnects;
                    client_->connect(t, client_->lastSession() + 1);
                } else {
                    reconnectAt_ = t + kReconnectDelayMs;
                }
            }
        }
        client_->tick(t);
        if (sink_) sink_->flush();
    }
    tickDialogue();
    tickUi();
    tickWorld();
    if (flight_) tickFlight();
}

fnvcoop::ITransport& Runtime::sessionTransport() {
    if (!flight_) return *transport_;
    if (!flightT_) flightT_ = std::make_shared<fnvcoop::flight::RecordingTransport>(*transport_, *flight_);
    return *flightT_;
}

// T1a: frame counter, a story digest every 2 s when the world moved, and desync detection -> dump.
void Runtime::tickFlight() {
    using namespace fnvcoop::flight;
    constexpr uint8_t kNoFlightSlot = fnvcoop::flight::kNoSlot;
    flight_->setFrame(static_cast<uint32_t>(counters_.frames));
    uint64_t t = now_();
    if (t - flightIdentAt_ >= 2000) {
        flightIdentAt_ = t;
        flight_->setIdentity(host_ ? kHostPlayer : client_ && client_->joined() ? client_->slot() : kNoFlightSlot, host_ ? kRoleHost : kRoleClient);
    }
    // Story digest at quiet points: the shared state is split into 256 key buckets (a one-level Merkle tree), so two machines'
    // digests of the same seq differ in exactly the buckets whose entries differ. Both sides sample 400 ms after the seq last moved
    // (the final seq of a burst), or every 5 s during constant activity. Idle cost: two integer compares per frame.
    const ShadowState* st = host_ ? &host_->state() : client_ ? &client_->state() : nullptr;
    if (st) {
        if (st->seq != flightSeenSeq_) { flightSeenSeq_ = st->seq; flightSeenAt_ = t; }
        if (st->seq != flightDigestSeq_ && (t - flightSeenAt_ >= 400 || t - flightDigestAt_ >= 5000)) {
            flightDigestSeq_ = st->seq;
            flightDigestAt_ = t;
            flight_->digest(bucketedDigestOf(kCatStory, host_ ? host_->epoch() : client_->worldEpoch(), st->seq, *st));
        }
    }
    if (host_ && host_->stats().digestsMismatch != flightHostMismatch_) {
        flightHostMismatch_ = host_->stats().digestsMismatch;
        flightDesync("host: a client digest did not match (mismatches=" + std::to_string(flightHostMismatch_) + ")");
    }
    if (client_ && client_->syncStats().repairsApplied != flightClientRepairs_) {
        flightClientRepairs_ = client_->syncStats().repairsApplied;
        flightDesync("client: host repaired our world (repairs=" + std::to_string(flightClientRepairs_) + ")");
    }
}

// A desync: first record the REAL engine world (what the desync detector compares) as a bucketed digest plus every entry's
// (key, value hash) and a text line per entry, then dump. Both machines do this, so the analyzer can name the exact key and value.
void Runtime::flightDesync(const std::string& why) {
    using namespace fnvcoop::flight;
    uint64_t t = now_();
    if (flightDetailAt_ == 0 || t - flightDetailAt_ >= 30000) {
        // client: the engine world its last digest was built from (before the host's repair); host: its shadow, the reference the digest is compared with
        const ShadowState* src = host_ ? &host_->state() : flightWorld_.get();
        if (src) {
            flightDetailAt_ = t;
            std::vector<BaselineEntry> be = baselineEntries(*src);
            std::vector<std::pair<uint64_t, uint32_t>> es;
            es.reserve(be.size());
            for (const BaselineEntry& e : be) es.emplace_back(e.key, e.value);
            uint32_t epoch = host_ ? host_->epoch() : client_ ? client_->worldEpoch() : 0;
            flight_->digest(bucketedDigest(kCatWorld, epoch, 0, es));
            flight_->digestDetail(kCatWorld, epoch, 0, es);
            size_t n = 0;
            for (const BaselineEntry& e : be) {
                if (++n > 20000) break;
                char key[32];
                std::snprintf(key, sizeof key, "%016llx", (unsigned long long)e.key);
                flight_->marker(kMarkNote, std::string("entry ") + key + " " + std::to_string(e.value) + " " + describeEntry(e.effect));
            }
        } else {
            flight_->marker(kMarkNote, "desync: no world snapshot available for key detail");
        }
    }
    flight_->onDesync(why);
}

static Bytes worldPayload(uint64_t id, const std::string& key, const std::string& json) {
    Value o = Value::object();
    o.set("id", (int64_t)id).set("k", key).set("e", json);
    std::string t = o.dump();
    return Bytes(t.begin(), t.end());
}

void Runtime::noteCompanion(const CompanionOwner& c) {
    if (c.active) companionOwners_[c.npc] = c.owner; else companionOwners_.erase(c.npc);
    if (dlg_) dlg_->onCompanionOwner(c.npc, c.active ? c.owner : kNoPlayer);
}

void Runtime::syncCompanions() {
    if (!dlg_) return;
    for (auto& [npc, owner] : companionOwners_) dlg_->onCompanionOwner(npc, owner);
}

void Runtime::pushWorldEffect(const std::string& key, const std::string& json, bool ephemeral) {
    if (!host_ || !running_) return;
    ++worldStats_.pushed;
    if (!worldId_) worldId_ = now_() << 12;   // a restarted host never reuses ids a long-lived guest already saw
    const uint64_t id = ++worldId_;
    const std::string k = key.empty() ? "#" + std::to_string(id) : key;
    if (ephemeral && !FNV_MUT(10)) ++worldStats_.ephemeral;   // one-shot presentation: the guests connected now see it, a late joiner does not replay it
    else worldState_[k] = WorldEntry{id, json};
    host_->sendApp(kNoPlayer, kAppWorldChannel, worldPayload(id, k, json));
    ++worldStats_.sent;
}

std::vector<PlayerId> Runtime::joinedSlots() const {
    std::vector<PlayerId> out;
    if (host_) for (auto& cv : host_->clients()) if (cv.joined) out.push_back(cv.slot);
    std::sort(out.begin(), out.end());
    return out;
}

bool Runtime::slotPerkRank(PlayerId slot, const FormRef& perk, int32_t* rank) const {
    if (!host_ || slot >= kMaxSlots) return false;
    const auto& perks = host_->state().ledger[slot].perks;
    auto it = perks.find(perk);
    if (rank) *rank = it == perks.end() ? 0 : it->second;
    return true;
}

bool Runtime::sharedJournalHas(const FormRef& note) const {
    const ShadowState* st = host_ ? &host_->state() : client_ ? &client_->state() : nullptr;
    return st && st->notes.count(note) != 0;
}

int Runtime::companionOwnerOf(const FormRef& npc) const {
    auto it = companionOwners_.find(npc);
    return it == companionOwners_.end() ? -1 : static_cast<int>(it->second);
}


// ---- campaign wave 2 (P6): the rules engine in the session --------------------------------------------------------------------------------------------
bool Runtime::slotPlayerRef(PlayerId slot, FormRef& out) {
    if (playerRef_) return playerRef_(slot, out);
    if (slot == kHostPlayer) { out = FormRef("falloutnv.esm", 0x14); return true; }
    return false;
}

void Runtime::buildRulesIndexes() {
    actingQuests_.clear();
    anchorRadius_.clear();
    if (!rules_) return;
    if (!causeQuestsJson_.empty() && !FNV_MUT(20)) {   // G02: a quest's own script, an INFO and a ref now find the quest whose rules apply
        std::string err;
        if (!rules_->loadCauseQuests(causeQuestsJson_, &err) && firstErrors_.size() < 12) firstErrors_.push_back("cause quests: " + err);
    }
    for (const auto& kv : rulesDb_.quests()) {
        FormRef q;
        if (!edid_ || !edid_(kv.first, q)) continue;
        std::vector<std::string> names = rules_->perPlayerVarNames(q);
        if (!names.empty()) actingQuests_.emplace_back(q, std::move(names));
        for (const Rule& r : kv.second.rules) {
            if (r.kind != "PartyTether") continue;
            const std::string a = r.scalar("anchor", r.scalar("anchor_ref"));
            FormRef af;
            if (!a.empty() && edid_(a, af)) anchorRadius_[af] = static_cast<float>(r.number("radius", 0));
        }
    }
}

void Runtime::wireRulesCallbacks() {
    if (!FNV_MUT(29)) rules_->gatherRest = [this](const FormRef& quest, uint32_t stage, const std::vector<PlayerId>& absent, const FormRef& target) {
        std::vector<Effect> moves;
        unsigned k = 0;
        for (PlayerId s : absent) {
            FormRef pr;
            if (!slotPlayerRef(s, pr)) continue;
            MoveRef m;
            m.ref = pr;
            m.cell = target;
            m.x = 64.f * static_cast<float>(k++);
            m.partyWarp = true;
            moves.push_back(m);
        }
        if (moves.empty() || !host_) return;
        Cause c;
        c.kind = CauseKind::Script;
        c.source = quest;
        c.aux = static_cast<int32_t>(stage);
        host_->commit(kHostPlayer, c, std::move(moves));   // the host's own move is applied by onHostCommitted, a guest's by its applier
        ++rulesStats_.casesGathered;
    };
    rules_->tetherWarn = [this](const FormRef&, uint32_t, const std::vector<PlayerId>& outside, const FormRef&) {
        for (PlayerId s : outside) notifyPlayer(s, "You are outside the scene zone. Go back to your party.");
    };
    rules_->respawnPlayer = [this](PlayerId slot, const FormRef& checkpoint) {
        Value o = Value::object();
        o.set("op", "respawn").set("checkpoint", nvhooks::refToStr(checkpoint));
        deliverOrder(slot, o.dump());
    };
    rules_->reloadHostSave = [this]() { if (reloadSave_) reloadSave_(); };
}

uint64_t Runtime::dialogueResult(const fnvcoop_dlg::ResultRequest& rq) {
    std::shared_ptr<void> scope;   // P2 PlayerFactsScope: while the script runs, player.GetItemCount / GetAV / HasPerk / GetEquipped answer for the initiator
    if (resultScope_ && !FNV_MUT(9)) scope = resultScope_(rq);
    if (runnerOverride_) return runnerOverride_(rq);
    return dlgHooks_.run ? dlgHooks_.run(rq) : 0;
}

// P7 handover: the roster listener of the social module. The party flag has no home in the registry yet (it needs CompanionRegistry::setPartyOwned, P3's file).
void Runtime::onRosterChanged(const std::vector<RosterEntry>& all) {
    ++rulesStats_.rosterFeeds;
    for (const RosterEntry& c : all) {
        companionOwners_[c.npc] = c.owner;
        if (dlg_ && !FNV_MUT(32)) dlg_->onCompanionOwner(c.npc, c.owner);
        if (dlg_ && !FNV_MUT(35)) dlg_->companions().setPartyOwned(c.npc, c.party);   // ED-E uses nobody's slot and reads as hired for everybody
    }
    if (!host_ || !dlg_ || FNV_MUT(33)) return;
    std::vector<Effect> fixes;
    for (const auto& f : dlg_->companions().reconcile({})) {   // every rule with the value the registry wants (nothing is known yet)
        bool known = false;
        double cur = 0;
        std::map<uint16_t, double> vars;
        if (scriptVars_ && scriptVars_(f.quest, vars)) {
            auto it = vars.find(static_cast<uint16_t>(f.idx));
            if (it != vars.end()) { known = true; cur = it->second; }
        }
        if (known && static_cast<int32_t>(cur) == f.value) continue;
        fixes.push_back(SetScriptVar{f.quest, static_cast<uint16_t>(f.idx), static_cast<double>(f.value)});
    }
    if (fixes.empty()) return;
    Cause c;
    c.kind = CauseKind::Script;
    c.source = FormRef("rules", 0x4A02u);
    host_->commit(kHostPlayer, c, fixes);
    for (const Effect& e : fixes)
        if (hostApplier_) hostApplier_(e);   // the host's own variable too: a committed effect is never applied on the machine that made it
    rulesStats_.reconcileFixes += fixes.size();
}

// G24: the acting player's copy of the per-player quest variables is in the engine while a script / INFO runs for that player; the host's copy returns after.
void Runtime::actingBegin(PlayerId slot) {
    if (!rules_ || !varIO_.read || !varIO_.write || FNV_MUT(22) || actingQuests_.empty()) return;
    for (auto& aq : actingQuests_) {
        std::vector<RulesEngine::VarValue> live;
        if (!varIO_.read(aq.first, aq.second, live)) continue;
        std::vector<RulesEngine::VarValue> vals = rules_->beginActing(aq.first, slot, live);
        if (!vals.empty()) varIO_.write(aq.first, vals);
    }
    ++rulesStats_.actingBrackets;
}

// W3d item 1: the acting copies of ONE quest (the script that runs per slot belongs to it), the same swap as actingBegin / actingEnd for a single quest.
void Runtime::actingBeginQuest(const FormRef& quest, PlayerId slot) {
    if (!rules_ || FNV_MUT(42)) return;   // MUTANT 42: a per-slot run does not open the slot's acting context (its copies and its effect routing)
    std::vector<RulesEngine::VarValue> live;
    const std::vector<std::string> names = rules_->perPlayerVarNames(quest);
    if (varIO_.read && !names.empty()) varIO_.read(quest, names, live);
    std::vector<RulesEngine::VarValue> vals = rules_->beginActing(quest, slot, live);
    if (varIO_.write && !vals.empty()) varIO_.write(quest, vals);
    ++rulesStats_.actingBrackets;
}

void Runtime::actingEndQuest(const FormRef& quest, PlayerId slot) {
    if (!rules_ || FNV_MUT(42)) return;
    std::vector<RulesEngine::VarValue> live;
    const std::vector<std::string> names = rules_->perPlayerVarNames(quest);
    if (varIO_.read && !names.empty()) varIO_.read(quest, names, live);
    rules_->endActing(quest, slot, live);
    if (slot == kHostPlayer) return;
    std::vector<RulesEngine::VarValue> mine = rules_->beginActing(quest, kHostPlayer, live);   // the host plays on with its own copy
    if (varIO_.write && !mine.empty()) varIO_.write(quest, mine);
    rules_->endActing(quest, kHostPlayer, mine);
}

std::vector<Runtime::PerSlotScriptInfo> Runtime::perSlotScripts() const {
    std::vector<PerSlotScriptInfo> out;
    if (!rules_ || !edid_ || FNV_MUT(43)) return out;   // MUTANT 43: the rules' per-slot script list is never read
    for (const std::string& name : rules_->perPlayerScripts()) {
        PerSlotScriptInfo info;
        info.edid = name;
        if (!edid_(name, info.script)) continue;   // a script this build cannot name
        Cause c;
        c.kind = CauseKind::Script;
        c.source = info.script;
        FormRef quest;
        if (rules_->questOfCause(c, quest)) {
            for (const std::string& ov : rules_->perPlayerObjVarNames(quest)) {   // "Script.var" of THIS script
                const size_t dot = ov.find('.');
                if (dot == std::string::npos || dot != name.size()) continue;
                bool same = true;
                for (size_t i = 0; i < name.size() && same; ++i) same = std::tolower(static_cast<unsigned char>(ov[i])) == std::tolower(static_cast<unsigned char>(name[i]));
                if (!same) continue;
                std::string v = ov.substr(dot + 1);
                for (char& ch : v) ch = static_cast<char>(std::tolower(static_cast<unsigned char>(ch)));
                if (std::find(info.vars.begin(), info.vars.end(), v) == info.vars.end()) info.vars.push_back(std::move(v));
            }
        }
        out.push_back(std::move(info));
    }
    return out;
}

size_t Runtime::runScriptPerSlot(uint32_t scriptRefID, const std::function<void(PlayerId)>& body) {
    if (!rules_ || !host_ || !body || !idToRef_ || FNV_MUT(41)) return 0;   // MUTANT 41: a script the rules run per slot runs for the host only
    FormRef sref, quest;
    if (!idToRef_(scriptRefID, sref)) return 0;
    Cause c;
    c.kind = CauseKind::Script;
    c.source = sref;
    if (!rules_->questOfCause(c, quest)) return 0;
    size_t ran = 0;
    for (PlayerId s : joinedSlots()) {
        struct Window {   // closes the window even when body() throws (a test fake, never the game)
            Runtime& rt; PlayerId prev, slot;
            ~Window() { if (rt.actingHook_) rt.actingHook_(slot, false); rt.actionOrigin_ = prev; }
        } w{*this, actionOrigin_, s};
        actionOrigin_ = s;
        if (actingHook_) actingHook_(s, true);
        actingBeginQuest(quest, s);
        body(s);
        actingEndQuest(quest, s);
        ++ran;
        ++rulesStats_.perSlotRuns;
    }
    return ran;
}

// W3a item 5: see coop_runtime.h. Mirrors the routing in synccore/src/rules.cpp (ctrlInitiator): the quest of the open scope's cause has a SetPlayerControls rule with scope=initiator.
bool Runtime::controlsGuestOnly(PlayerId slot, bool disable) {
    if (!rules_ || !hub_ || !host_ || slot == kHostPlayer || slot == kNoPlayer || FNV_MUT(37)) return false;   // MUTANT 37: the host always makes the call
    Cause c;
    FormRef quest;
    if (!hub_->currentCause(&c) || !rules_->questOfCause(c, quest)) return false;
    const QuestRules* q = rules_->rulesFor(quest);
    if (!q) return false;
    const Rule* r = c.kind == CauseKind::QuestResult ? q->findFor("SetPlayerControls", static_cast<uint32_t>(c.aux)) : nullptr;
    if (!r) r = q->find("SetPlayerControls");
    if (!r || r->scalar("scope", "all") != "initiator") return false;
    if (disable) ++skippedLocks_;   // the give-back in onHostCommitted must not "restore" bits the host never lost
    ++rulesStats_.controlsHostSkipped;
    return true;
}

// W3a item 4. A script that runs for a guest sees that guest's view of the companion variables (hired flags, party counts) and, for Dead Money, its own
// rCurrentFollower; a script that reads them party-wide (view-party) sees the shared variables as they are (the aggregate). The shared variables are put back after.
void Runtime::onScriptRun(uint32_t script, bool before) {
    if (!host_ || !dlg_ || !viewSlotOf_ || !varIO_.write || !varIO_.name || FNV_MUT(38)) return;   // MUTANT 38: the script run is not told (every script reads the aggregate)
    const PlayerId slot = actionOrigin_;
    if (before) {
        if (slot == kNoPlayer || slot == kHostPlayer) { ++idleRuns_; return; }   // the common case (hundreds of ambient scripts a frame): a counter, no allocation
        viewStack_.push_back(companionView_);
        const PlayerId want = viewSlotOf_(script, slot);
        if (want != companionView_) applyCompanionView(want);
    } else {
        if (viewStack_.empty()) { if (idleRuns_ > 0) --idleRuns_; return; }
        const PlayerId back = viewStack_.back();
        viewStack_.pop_back();
        if (back != companionView_) applyCompanionView(back);
    }
}

void Runtime::applyCompanionView(PlayerId view) {
    companionView_ = view;
    ++rulesStats_.companionViews;
    dlg_->companions().forEachRule([&](const FormRef& quest, uint32_t idx, fnvcoop_dlg::VarKind kind) {
        if (kind == fnvcoop_dlg::VarKind::Available || kind == fnvcoop_dlg::VarKind::Fired) return;   // not per-player state
        const int32_t v = view == 0xFE ? dlg_->companions().aggregateVar(quest, idx, 0) : dlg_->companions().viewVar(quest, idx, view, 0);
        std::string name;
        if (varIO_.name(quest, idx, &name)) varIO_.write(quest, {{name, static_cast<double>(v)}});
    });
    FollowerVar fv;
    if (followerOf_ && followerOf_(view == 0xFE ? kHostPlayer : view, &fv) && !fv.var.empty()) {
        if (view == 0xFE) {
            if (followerSaved_) varIO_.write(fv.quest, {{fv.var, followerReal_}});
            followerSaved_ = false;
        } else {
            if (!followerSaved_) {
                std::vector<RulesEngine::VarValue> cur;
                if (varIO_.read && varIO_.read(fv.quest, {fv.var}, cur) && !cur.empty()) { followerReal_ = cur[0].value; followerSaved_ = true; }
            }
            if (followerSaved_) varIO_.write(fv.quest, {{fv.var, fv.formId}});
        }
    }
}

void Runtime::actingEnd(PlayerId slot) {
    viewStack_.clear();
    if (companionView_ != 0xFE && dlg_ && varIO_.write && varIO_.name) applyCompanionView(0xFE);   // a guest's run never leaves its view in the shared variables
    if (!rules_ || !varIO_.read || !varIO_.write || FNV_MUT(22) || actingQuests_.empty()) return;
    for (auto& aq : actingQuests_) {
        std::vector<RulesEngine::VarValue> live;
        if (!varIO_.read(aq.first, aq.second, live)) continue;
        rules_->endActing(aq.first, slot, live);
        if (slot == kHostPlayer) continue;
        std::vector<RulesEngine::VarValue> mine = rules_->beginActing(aq.first, kHostPlayer, live);   // the host plays on with its own copy
        if (!mine.empty()) varIO_.write(aq.first, mine);
        rules_->endActing(aq.first, kHostPlayer, mine);
    }
}

// G18 / G25 / G41: commits the rules engine makes by itself reach the host's own player here (a captured effect already ran as the script).
void Runtime::onHostCommitted(const EffectTxn& t) {
    if (!hostApplier_ || FNV_MUT(21)) return;
    FormRef hostRef;
    const bool haveHostRef = slotPlayerRef(kHostPlayer, hostRef);
    auto inSnapshot = [&](const Effect& e) {
        if (!inCapturedCommit_) return false;
        for (const Effect& c : capturedSnapshot_) if (c == e) return true;
        return false;
    };
    for (const Effect& e : t.effects) {
        if (const auto* sc = std::get_if<SetPlayerControls>(&e)) {
            const bool mine = sc->to.mode == Recipient::All || sc->to.includes(kHostPlayer);
            if (mine) {
                if (inSnapshot(e)) continue;
                hostApplier_(e);
                ++rulesStats_.hostEffectsApplied;
            } else if (!sc->enable && inCapturedCommit_ && skippedLocks_ > 0) {
                --skippedLocks_;   // the hook never made the host's call for this lock (controlsGuestOnly): nothing to give back
            } else if (!sc->enable && inCapturedCommit_) {
                // scope=initiator: the script's own DisablePlayerControls already locked THIS machine, but the lock belongs to somebody else. Give the host its bits back.
                for (const Effect& c : capturedSnapshot_) {
                    const auto* o = std::get_if<SetPlayerControls>(&c);
                    if (o && o->controls == sc->controls && !o->enable && o->to.mode == Recipient::Default) {
                        hostApplier_(Effect(SetPlayerControls{sc->controls, true, Recipient::slot(kHostPlayer)}));
                        ++rulesStats_.hostEffectsApplied;
                        break;
                    }
                }
            }
        } else if (const auto* mv = std::get_if<MoveRef>(&e)) {
            if (!haveHostRef || !(mv->ref == hostRef) || inSnapshot(e)) continue;
            hostApplier_(e);
            ++rulesStats_.hostEffectsApplied;
        } else if (const auto* ri = std::get_if<RemoveItem>(&e)) {
            // Guest pack facts: a guest's hand-in runs as a script on the host, but its `player.RemoveItem` did not touch the host's pack (nvhooks::GuestItemScope). What the
            // rules charge to the HOST (a pooled hand-in: the host holds part of the totems) is taken here, from the host's own pack, once. A host script (origin 0) already ran
            // its RemoveItem in the engine; a txn that is not the host's own capture is applied by the machine of its recipient.
            const bool guestRun = inCapturedCommit_ && ((t.origin != kHostPlayer && t.origin < kMaxSlots) || FNV_MUT(963));   // MUTANT 963: a host script's own removal is taken a second time
            const bool mine = ri->to.includes(kHostPlayer) || FNV_MUT(964);                                                    // MUTANT 964: every removal is taken from the host
            if (FNV_MUT(962) || !guestRun || !mine || inSnapshot(e)) continue;                                                // MUTANT 962: the host's share is never taken
            hostApplier_(e);
            ++rulesStats_.hostEffectsApplied;
        } else if (const auto* ai = std::get_if<AddItem>(&e)) {
            // The same for what a guest's script gives: the hook did not put it into the host's pack (planPlayerAddItem), so the host gets its share here, once, when the committed
            // mask names it. A guest-only reward (a non-quest cause) never reaches the host's pack.
            const bool guestRun = inCapturedCommit_ && t.origin != kHostPlayer && t.origin < kMaxSlots;
            const bool mine = ai->to.includes(kHostPlayer);
            if (FNV_MUT(99) || !guestRun || !mine) continue;   // MUTANT 99: the host's share of a guest's reward is never given
            hostApplier_(e);
            ++rulesStats_.hostEffectsApplied;        } else if (const auto* av = std::get_if<ModActorValue>(&e)) {
            // XP a guest's script gives (RewardXP): the hook did not give it to the host's player; the host's share is given here once. Other actor values are real engine changes.
            const bool guestRun = inCapturedCommit_ && t.origin != kHostPlayer && t.origin < kMaxSlots;
            if (FNV_MUT(100) || av->av != kAvExperience || !guestRun || !av->to.includes(kHostPlayer)) continue;   // MUTANT 100 (here): the host's share of a guest's XP is never given
            hostApplier_(e);
            ++rulesStats_.hostEffectsApplied;
        } else if (const auto* km = std::get_if<ModKarma>(&e)) {
            const bool guestRun = inCapturedCommit_ && t.origin != kHostPlayer && t.origin < kMaxSlots;
            if (FNV_MUT(100) || !guestRun || !km->to.includes(kHostPlayer)) continue;
            hostApplier_(e);
            ++rulesStats_.hostEffectsApplied;
        } else if (const auto* mr = std::get_if<ModReputation>(&e)) {
            const bool guestRun = inCapturedCommit_ && t.origin != kHostPlayer && t.origin < kMaxSlots;
            if (FNV_MUT(100) || !guestRun || !mr->to.includes(kHostPlayer)) continue;
            hostApplier_(e);
            ++rulesStats_.hostEffectsApplied;
        }
    }
}

void Runtime::tickRulesSession(uint64_t t) {
    tickAnchors(t);
    tickLateJoin(t);
}

// G25: anchor tethers. The radius lives in the rules (host only), so the host watches every joined player itself: its own position from the engine, a guest's from
// that guest's puppet. The transitions go to the rules engine exactly like a guest's ReportNear / ReportFar action would.
void Runtime::tickAnchors(uint64_t t) {
    if (!rules_ || !host_ || FNV_MUT(23) || anchorRadius_.empty() || !anchorProbe_.anchorPos || !anchorProbe_.slotPos) return;
    if (t < anchorAt_ + 500) return;
    anchorAt_ = t;
    std::vector<AnchorWatch::Anchor> anchors;
    for (const auto& kv : host_->state().tethers) {
        if (!kv.second) continue;
        auto r = anchorRadius_.find(kv.first);
        if (r == anchorRadius_.end() || r->second <= 0) continue;
        AnchorWatch::Anchor a;
        a.id = kv.first;
        a.radius = r->second;
        if (anchorProbe_.anchorPos(kv.first, a.x, a.y, a.z)) anchors.push_back(a);
    }
    std::vector<PlayerId> slots{kHostPlayer};
    for (PlayerId s : joinedSlots()) slots.push_back(s);
    for (PlayerId s : slots) {
        AnchorWatch& w = anchorWatch_[s];
        w.set(anchors);
        float x = 0, y = 0, z = 0;
        if (!anchorProbe_.slotPos(s, x, y, z)) continue;
        for (const AnchorWatch::Change& c : w.update(x, y, z)) {
            rules_->onNearReported(s, c.anchor, c.within);
            ++rulesStats_.nearReports;
        }
    }
}

// G41: a player who joins while a scene runs waits (controls held) and is warped to the party's zone afterwards, as policy late_join.* says.
void Runtime::tickLateJoin(uint64_t t) {
    if (!rules_ || !host_ || FNV_MUT(26)) return;
    const std::vector<PlayerId> joined = joinedSlots();
    for (PlayerId s : joined)
        if (lateJoinSeen_.insert(s).second) lateJoinPending_.insert(s);
    for (auto it = lateJoinSeen_.begin(); it != lateJoinSeen_.end();) {
        if (std::find(joined.begin(), joined.end(), *it) == joined.end()) {
            lateJoinPending_.erase(*it);
            lateJoinLocked_.erase(*it);
            it = lateJoinSeen_.erase(it);
        } else ++it;
    }
    if (lateJoinPending_.empty() || t < lateJoinAt_ + 250) return;
    lateJoinAt_ = t;
    Cause c;
    c.kind = CauseKind::Script;
    c.source = FormRef("rules", 0x4A01u);
    auto lock = [&](PlayerId s, bool on) { host_->commit(kHostPlayer, c, {SetPlayerControls{kAllControls, !on, Recipient::slot(s)}}); };
    for (PlayerId s : std::vector<PlayerId>(lateJoinPending_.begin(), lateJoinPending_.end())) {
        const RulesEngine::JoinVerdict v = rules_->onPlayerJoining(s);
        if (v.kind == RulesEngine::JoinVerdict::WaitForScene) {
            if (lateJoinLocked_.insert(s).second) {
                lock(s, true);
                notifyPlayer(s, "A scene is running. You join your party when it ends.");
                ++rulesStats_.lateJoinHeld;
            }
            continue;
        }
        if (lateJoinLocked_.erase(s)) lock(s, false);
        if (v.kind == RulesEngine::JoinVerdict::WarpToZone) {
            FormRef pr;
            if (slotPlayerRef(s, pr)) {
                MoveRef m;
                m.ref = pr;
                m.cell = v.zone;
                m.partyWarp = true;
                host_->commit(kHostPlayer, c, {m});
                ++rulesStats_.lateJoinWarped;
            }
        }
        lateJoinPending_.erase(s);
    }
}

bool Runtime::deliverOrder(PlayerId slot, const std::string& json) {
    if (slot != kHostPlayer) return sendOrder(slot, json);
    Value v;
    std::string err;
    if (!nvcoop::json::parse(json, v, err)) return false;
    if (v.get("op").asString("") == "notice") { if (notice_) notice_(v.get("text").asString("")); return notice_ != nullptr; }
    return orderApplier_ && orderApplier_(json) == "Ok";
}

bool Runtime::notifyPlayer(PlayerId slot, const std::string& text) {
    Value o = Value::object();
    o.set("op", "notice").set("text", text);
    return deliverOrder(slot, o.dump());
}

RulesEngine::DeathVerdict Runtime::playerDied(PlayerId slot) {
    RulesEngine::DeathVerdict v;
    if (!rules_ || !host_ || FNV_MUT(28)) return v;
    ++rulesStats_.deaths;
    v = rules_->onPlayerDied(slot, now_());
    if (v.kind == RulesEngine::DeathVerdict::Downed) {
        Value o = Value::object();
        o.set("op", "downed").set("respawnMs", (int64_t)v.respawnAfterMs).set("checkpoint", v.hasCheckpoint ? nvhooks::refToStr(v.checkpoint) : std::string());
        deliverOrder(slot, o.dump());
    }
    return v;
}

void Runtime::localPlayerDied() {
    if (host_) playerDied(kHostPlayer);
    else sendOrderToHost("{\"op\":\"died\"}");
}

void Runtime::localPlayerRevived() {
    if (host_) { if (rules_) rules_->onPlayerRevived(kHostPlayer); }
    else sendOrderToHost("{\"op\":\"revived\"}");
}

// ---- player orders (G42 / G35 / G10) ----
static Bytes orderPayload(const std::string& json) { return Bytes(json.begin(), json.end()); }

bool Runtime::sendOrder(PlayerId toSlot, const std::string& json) {
    if (!host_ || !running_ || toSlot == kHostPlayer || toSlot == kNoPlayer) return false;
    bool joined = false;
    for (auto& cv : host_->clients()) joined = joined || (cv.slot == toSlot && cv.joined);
    if (!joined) return false;
    host_->sendApp(toSlot, kAppOrderChannel, orderPayload(json));
    ++orderStats_.sent;
    return true;
}

bool Runtime::sendOrderToHost(const std::string& json) {
    if (!client_ || !running_ || !client_->joined() || client_->baselineActive()) return false;
    client_->sendApp(kAppOrderChannel, orderPayload(json));
    ++orderStats_.sent;
    return true;
}

bool Runtime::killPlayer(PlayerId slot) {
    if (!host_ || slot == kHostPlayer || slot == kNoPlayer) return false;
    if (FNV_MUT(12)) {   // twin: the whole party dies
        bool any = false;
        for (auto& cv : host_->clients()) if (cv.joined) any = sendOrder(cv.slot, "{\"op\":\"kill\"}") || any;
        return any;
    }
    return sendOrder(slot, "{\"op\":\"kill\"}");
}

void Runtime::onOrderPacket(PlayerId from, const Bytes& payload) {
    Value v;
    std::string err;
    const std::string text(payload.begin(), payload.end());
    if (!nvcoop::json::parse(text, v, err)) return;
    ++orderStats_.received;
    const std::string op = v.get("op").asString("");
    if (!host_ && op == "notice") { if (notice_) notice_(v.get("text").asString("")); return; }   // a HUD line from the host
    if (host_ && op == "died") { playerDied(from); return; }                                    // G41: a guest's player died
    if (host_ && op == "revived") { if (rules_) rules_->onPlayerRevived(from); return; }
    if (host_) {   // a guest's request
        if (!orderHandler_) { ++orderStats_.handlerFailed; return; }
        const std::string st = orderHandler_(from, text);
        if (st == "Ok") ++orderStats_.handled; else ++orderStats_.handlerFailed;
        return;
    }
    if (!orderApplier_) { ++orderStats_.failed; return; }
    const std::string st = orderApplier_(text);
    if (st == "Ok") ++orderStats_.applied; else ++orderStats_.failed;
}

void Runtime::tickWorld() {
    if (host_) {
        std::set<PlayerId> joinedNow;
        for (auto& cv : host_->clients()) if (cv.joined) joinedNow.insert(cv.slot);
        for (PlayerId p : joinedNow) {
            if (worldJoined_.count(p)) continue;
            std::vector<std::pair<uint64_t, const std::pair<const std::string, WorldEntry>*>> order;
            for (auto& kv : worldState_) order.push_back({kv.second.id, &kv});
            std::sort(order.begin(), order.end(), [](auto& a, auto& b) { return a.first < b.first; });
            for (auto& o : order) { host_->sendApp(p, kAppWorldChannel, worldPayload(o.first, o.second->first, o.second->second.json)); ++worldStats_.replayed; }
        }
        worldJoined_ = joinedNow;
    } else if (client_ && client_->joined() && !client_->baselineActive() && worldApplier_) {
        while (!worldQueue_.empty()) {
            auto e = std::move(worldQueue_.front());
            worldQueue_.pop_front();
            applyWorldEntry(e.second.id, e.first, e.second.json);
        }
    }
}

void Runtime::onWorldPacket(const Bytes& payload) {
    Value v;
    std::string err;
    if (!nvcoop::json::parse(std::string(payload.begin(), payload.end()), v, err)) return;
    ++worldStats_.received;
    WorldEntry e{(uint64_t)v.get("id").asInt(), v.get("e").asString()};
    const std::string key = v.get("k").asString();
    if (!client_ || !client_->joined() || client_->baselineActive() || !worldApplier_) {
        if (worldQueue_.size() < 4096) worldQueue_.emplace_back(key, std::move(e));
        return;
    }
    applyWorldEntry(e.id, key, e.json);
}

void Runtime::applyWorldEntry(uint64_t id, const std::string& key, const std::string& json) {
    auto it = worldApplied_.find(key);
    if (it != worldApplied_.end() && it->second >= id) { ++worldStats_.stale; return; }   // replayed / reordered: never apply twice
    worldApplied_[key] = id;
    LogEntry le;
    le.dir = "world";
    le.seq = id;
    le.atMs = now_();
    std::string st = worldApplier_(json);
    le.results.push_back(st);
    if (st == "Ok") ++worldStats_.applied; else ++worldStats_.failed;
    pushLog(std::move(le));
}

void Runtime::uiSlideEnded() {
    if (!uiEnabled_ || !ui_.slideHost || cfg_.role != Role::Host) return;
    if (nvui::SlideHost* sh = slideHost()) sh->end();
    ++uiStats_.slideEnds;
}

// The host's slide broadcaster; its send fn goes out on the mux (first byte 0xE5 routes it on the guest).
nvui::SlideHost* Runtime::slideHost() {
    if (!slideHost_ && ui_.slideHost) {
        std::weak_ptr<bool> alive = alive_;
        slideHost_ = &ui_.slideHost([this, alive](PlayerId to, std::vector<uint8_t> bytes) {
            if (alive.expired() || !mux_ || !host_) return;
            for (auto& cv : host_->clients())
                if (cv.slot == to && cv.joined) { mux_->send(cv.peer, std::move(bytes)); return; }
        });
    }
    return slideHost_;
}

// Host: once the DialogueNode exists, point the router at it (boxes, consent, forced dialogue, condition views, slides).
void Runtime::wireUiHost() {
    if (uiBridged_ || !dlg_ || !host_ || !ui_.router) return;
    nvui::UiRouter* router = ui_.router();
    if (!router) return;
    nvui::DialogueBridgeConfig c;
    c.node = dlg_.get();
    c.nowMs = [this]() { return static_cast<uint32_t>(now_()); };
    if (!ui_.consentFile.empty()) {
        std::ifstream in(ui_.consentFile);
        if (in) c.consent = fnvcoop_dlg::loadConsentMessages(in);
    }
    uiStats_.consentRows = static_cast<uint32_t>(c.consent.size());
    c.slotOfActor = [this](uint32_t ref) -> nvui::PlayerId {
        if (ref == nvui::kPlayerRefID) return nvui::kHost;
        if (!ghostOf_ || !host_) return nvui::kNoSlot;
        for (auto& cv : host_->clients())
            if (cv.joined && ghostOf_(cv.slot) == ref) return cv.slot;
        return nvui::kNoSlot;
    };
    c.lastCommittedSeq = [this]() -> uint64_t { return host_ ? host_->log().head() : 0; };
    c.slides = slideHost();
    if (rules_ && !FNV_MUT(34)) {   // wave 1b: ChoiceConsent boxes and the per-player quest variables of the rules engine
        fnvcoop_dlg::RulesViews v = fnvcoop_dlg::rulesViews(*rules_);
        c.isRulesChoice = v.isRulesChoice;
        c.hasPlayerVars = v.hasPlayerVars;
        c.viewPlayerVar = v.viewPlayerVar;
        c.varName = ui_.varName;
    }
    if (!uiMut(1)) router->bridge() = nvui::makeDialogueBridge(c);
    router->invalidateViews();   // the rules were (re)loaded with this session
    if (!uiMut(2)) nvui::attachMenuResume(*dlg_, *router);
    uiBridged_ = true;
    ++uiStats_.bridges;
}

void Runtime::tickUi() {
    if (!uiEnabled_) return;
    uint64_t now = now_();
    if (host_) {
        nvui::SlideHost* sh = slideHost();
        wireUiHost();
        if (!sh) return;
        std::set<PlayerId> joinedNow;
        for (auto& cv : host_->clients()) if (cv.joined) joinedNow.insert(cv.slot);
        for (PlayerId p : joinedNow) if (!uiJoined_.count(p)) sh->playerJoined(p, now);
        for (PlayerId p : uiJoined_) if (!joinedNow.count(p)) sh->playerLeft(p);
        uiJoined_ = joinedNow;
        sh->tick(now);
    } else if (client_ && ui_.slideClient) {
        bool joined = client_->joined();
        if (uiWasJoined_ && !joined) {
            ui_.slideClient().onHostLost();
            ++uiStats_.hostLost;
        }
        uiWasJoined_ = joined;
        ui_.slideClient().tick(now);
    }
}

bool Runtime::inject(const std::string& txnJson, std::string* resultJson, std::string* err) {
    Value p;
    std::string perr;
    if (!nvcoop::json::parse(txnJson, p, perr)) { if (err) *err = "bad json: " + perr; return false; }
    const Value& v = p.get("txn").isObject() ? p.get("txn") : p;
    EffectTxn txn;
    if (!nvhooks::txnFromJson(v, &txn, &perr)) { if (err) *err = "bad txn: " + perr; return false; }
    if (!running_) { if (err) *err = "coop runtime not running"; return false; }
    Value o = Value::object();
    ++counters_.injected;
    if (host_) {
        uint64_t seq = host_->commit(txn.origin, txn.cause, std::move(txn.effects));
        o.set("committed", seq != 0).set("seq", (int64_t)seq);
    } else {
        // Deliver as if received: same sink + applier path as a real txn, under the echo guard.
        Sink sink(*this);
        EchoGuard guard(hub_ ? &hub_->echo() : nullptr);
        IEffectSink::Context ctx;
        ctx.origin = txn.origin;
        ctx.cause = &txn.cause;
        size_t idx = 0;
        uint64_t ok0 = counters_.applied, bad0 = counters_.applyFailed;
        for (const Effect& e : txn.effects) { ctx.index = idx++; sink.apply(e, ctx); }
        sink.flush();
        // flush() logged it as "apply"; relabel as inject
        if (!log_.empty()) log_.back().dir = "inject";
        o.set("effects", (int64_t)txn.effects.size()).set("applied", (int64_t)(counters_.applied - ok0))
            .set("failed", (int64_t)(counters_.applyFailed - bad0));
    }
    if (resultJson) *resultJson = o.dump();
    return true;
}

static Value entryToJson(const LogEntry& le) {
    EffectTxn t;
    t.origin = le.origin;
    t.cause = le.cause;
    t.effects = le.effects;
    Value o = nvhooks::txnToJson(t);
    o.set("dir", le.dir).set("seq", (int64_t)le.seq).set("atMs", (int64_t)le.atMs);
    if (!le.results.empty()) {
        Value r = Value::array();
        for (auto& s : le.results) r.push(s);
        o.set("results", r);
    }
    return o;
}

std::string Runtime::logJson(size_t maxEntries) const {
    Value a = Value::array();
    size_t start = log_.size() > maxEntries ? log_.size() - maxEntries : 0;
    for (size_t i = start; i < log_.size(); ++i) a.push(entryToJson(log_[i]));
    Value o = Value::object();
    o.set("count", (int64_t)log_.size()).set("txns", a);
    return o.dump();
}

std::string Runtime::statusJson() const {
    Value o = Value::object();
    o.set("role", roleName(cfg_.role)).set("enabled", cfg_.enable).set("running", running_).set("left", left_).set("mutant", (int64_t)(FNV_MUT_ANY() ? 1 : 0))
        .set("port", (int64_t)boundPort()).set("error", lastError_);
    Value c = Value::object();
    c.set("frames", (int64_t)counters_.frames).set("captured", (int64_t)counters_.captured).set("committed", (int64_t)counters_.committed)
        .set("droppedEmpty", (int64_t)counters_.droppedEmpty).set("applied", (int64_t)counters_.applied)
        .set("applyFailed", (int64_t)counters_.applyFailed).set("injected", (int64_t)counters_.injected)
        .set("saves", (int64_t)counters_.saves).set("loads", (int64_t)counters_.loads).set("loadsBlocked", (int64_t)counters_.loadsBlocked)
        .set("actions", (int64_t)counters_.actions).set("baselineEffects", (int64_t)counters_.baselineEffects)
        .set("disconnects", (int64_t)counters_.disconnects).set("reconnects", (int64_t)counters_.reconnects);
    o.set("counters", c);
    if (uiEnabled_) {
        Value u = Value::object();
        u.set("bridged", uiBridged_).set("bridges", (int64_t)uiStats_.bridges).set("slidePackets", (int64_t)uiStats_.slidePackets)
            .set("slideEnds", (int64_t)uiStats_.slideEnds).set("hostLost", (int64_t)uiStats_.hostLost).set("consentRows", (int64_t)uiStats_.consentRows);
        o.set("ui", u);
    }
    if (worldStats_.pushed || worldStats_.received) {
        Value w = Value::object();
        w.set("pushed", (int64_t)worldStats_.pushed).set("sent", (int64_t)worldStats_.sent).set("replayed", (int64_t)worldStats_.replayed)
            .set("received", (int64_t)worldStats_.received).set("applied", (int64_t)worldStats_.applied)
            .set("failed", (int64_t)worldStats_.failed).set("stale", (int64_t)worldStats_.stale).set("ephemeral", (int64_t)worldStats_.ephemeral)
            .set("state", (int64_t)worldState_.size());
        o.set("world", w);
    }
    if (orderStats_.sent || orderStats_.received) {
        Value od = Value::object();
        od.set("sent", (int64_t)orderStats_.sent).set("received", (int64_t)orderStats_.received).set("applied", (int64_t)orderStats_.applied)
            .set("failed", (int64_t)orderStats_.failed).set("handled", (int64_t)orderStats_.handled).set("handlerFailed", (int64_t)orderStats_.handlerFailed);
        o.set("orders", od);
    }
    if (playerRefBinds_) {
        Value pr = Value::object();
        pr.set("binds", (int64_t)playerRefBinds_);
        Value slots = Value::object();
        for (unsigned s = 0; s < 4; ++s) {
            FormRef r;
            bool ok = playerRef_ && playerRef_(PlayerId(s), r);
            slots.set(std::to_string(s), ok ? r.plugin + ":" + std::to_string(r.localId) : std::string());
        }
        pr.set("slots", slots);
        o.set("playerRefs", pr);
    }
    if (udp_) {
        const auto& s = udp_->stats();
        Value t = Value::object();
        t.set("kind", "udp").set("datagramsSent", (int64_t)s.datagramsSent).set("datagramsRecv", (int64_t)s.datagramsRecv)
            .set("messagesSent", (int64_t)s.messagesSent).set("messagesRecv", (int64_t)s.messagesRecv)
            .set("bad", (int64_t)s.badDatagrams).set("sendErrors", (int64_t)s.sendErrors);
        o.set("transport", t);
    } else if (enet_) {
        const auto& s = enet_->stats();
        Value t = Value::object();
        t.set("kind", "enet").set("datagramsIn", (int64_t)s.datagramsIn).set("datagramsOut", (int64_t)s.datagramsOut)
            .set("bytesIn", (int64_t)s.bytesIn).set("bytesOut", (int64_t)s.bytesOut).set("connected", enet_->connected());
        o.set("transport", t);
    } else if (steam_) {
        const auto& s = steam_->stats();
        Value t = Value::object();
        t.set("kind", "steam").set("packetsIn", (int64_t)s.packetsIn).set("packetsOut", (int64_t)s.packetsOut)
            .set("bytesIn", (int64_t)s.bytesIn).set("bytesOut", (int64_t)s.bytesOut).set("refused", (int64_t)s.refused)
            .set("connected", steam_->connected()).set("error", steam_->lastError());
        o.set("transport", t);
    }
    if (hub_) {
        const auto& s = hub_->stats();
        Value h = Value::object();
        h.set("enabled", hub_->enabled()).set("recorded", (int64_t)s.recorded).set("txns", (int64_t)s.txns)
            .set("droppedNoScope", (int64_t)s.droppedNoScope).set("droppedEcho", (int64_t)s.droppedEcho)
            .set("emptyScopes", (int64_t)s.emptyScopes);
        o.set("hub", h);
    }
    if (host_) {
        const auto& s = host_->stats();
        Value peers = Value::array();
        int joined = 0;
        for (auto& cv : host_->clients()) {
            Value p = Value::object();
            p.set("token", std::to_string(cv.token)).set("slot", (int64_t)cv.slot).set("joined", cv.joined).set("acked", (int64_t)cv.acked);
            if (cv.joined) ++joined;
            peers.push(p);
        }
        Value h = Value::object();
        h.set("lastSeq", (int64_t)host_->log().head()).set("peers", peers).set("connected", joined).set("allSynced", host_->allSynced())
            .set("commits", (int64_t)s.commits).set("droppedByPolicy", (int64_t)s.droppedByPolicy)
            .set("epoch", (int64_t)host_->epoch()).set("welcomes", (int64_t)s.welcomes).set("snapshots", (int64_t)s.snapshots).set("replays", (int64_t)s.replays)
            .set("retransmitBatches", (int64_t)s.retransmitBatches).set("badMessages", (int64_t)s.badMessages)
            .set("questRunningAdopted", (int64_t)s.questRunningAdopted).set("questStateAdopted", (int64_t)s.questStateAdopted)
            .set("questStateDisagreed", (int64_t)s.questStateDisagreed).set("resumesRefused", (int64_t)s.resumesRefused)
            .set("objectiveZeroAdopted", (int64_t)s.objectiveZeroAdopted).set("objectiveStateAdopted", (int64_t)s.objectiveStateAdopted)
            .set("objectiveStateDisagreed", (int64_t)s.objectiveStateDisagreed);
        if (replicator_) {
            const auto& rp = replicator_->stats();
            Value r = Value::object();
            r.set("writes", (int64_t)rp.writes).set("coalesced", (int64_t)rp.coalesced).set("skippedUnchanged", (int64_t)rp.skippedUnchanged)
                .set("effectsCommitted", (int64_t)rp.effectsCommitted).set("txns", (int64_t)rp.txns).set("deferredByBudget", (int64_t)rp.deferredByBudget)
                .set("volatileDropped", (int64_t)rp.volatileDropped).set("pending", (int64_t)replicator_->pending());
            o.set("replicator", r);
        }
        {
            Value dj;
            std::string derr;
            if (nvcoop::json::parse(host_->desyncsJson(), dj, derr)) o.set("desyncs", dj);
        }
        o.set("host", h);
        o.set("lastSeq", (int64_t)host_->log().head()).set("connected", joined);
    }
    if (client_) {
        const auto& s = client_->stats();
        Value c2 = Value::object();
        c2.set("joined", client_->joined()).set("rejected", client_->rejected()).set("slot", (int64_t)client_->slot())
            .set("applied", (int64_t)client_->applied()).set("txnsApplied", (int64_t)s.txnsApplied)
            .set("duplicatesDropped", (int64_t)s.duplicatesDropped).set("snapshots", (int64_t)s.snapshots)
            .set("replays", (int64_t)s.replays).set("token", std::to_string(cfg_.token))
            .set("worldEpoch", (int64_t)client_->worldEpoch()).set("rebases", (int64_t)s.rebases).set("staleEpochDropped", (int64_t)s.staleEpochDropped);
        c2.set("rejectText", client_->rejectText()).set("syncing", client_->baselineActive()).set("baselineDone", client_->baselineDone())
            .set("rejectReason", (int64_t)client_->rejectReason())
            .set("baselineEffects", (int64_t)client_->baselineResult().effectsApplied)
            .set("baselineKept", (int64_t)client_->baselineResult().entriesKept).set("baselineReverted", (int64_t)client_->baselineResult().reverted)
            .set("baselineUnrevertable", (int64_t)client_->baselineResult().unrevertable).set("baselineChunks", (int64_t)client_->baselineResult().chunks)
            .set("repairUnrevertable", (int64_t)client_->syncStats().repairRevertsUnrevertable);
        o.set("client", c2);
        o.set("lastSeq", (int64_t)client_->applied()).set("connected", client_->joined() ? 1 : 0);
    }
    {
        Value ty = Value::object();
        for (auto& [k, n] : tally_) ty.set(k, (int64_t)n);
        o.set("effectTally", ty);
        Value fe = Value::array();
        for (auto& x : firstErrors_) fe.push(x);
        o.set("firstErrors", fe);
    }
    if (rules_) {
        const auto& rs = rules_->stats();
        Value r = Value::object();
        r.set("quests", (int64_t)rulesDb_.quests().size()).set("pendingDeferrals", (int64_t)rules_->pendingDeferrals())
            .set("stagesDeferred", (int64_t)rs.stagesDeferred).set("stagesReleased", (int64_t)rs.stagesReleased)
            .set("barrierTimeouts", (int64_t)rs.barrierTimeouts).set("arrivals", (int64_t)rs.arrivals)
            .set("recipientRewrites", (int64_t)rs.recipientRewrites).set("rulesActions", (int64_t)counters_.rulesActions);
        r.set("causeQuests", (int64_t)rules_->causeQuestCount()).set("actingQuests", (int64_t)actingQuests_.size()).set("anchors", (int64_t)anchorRadius_.size())
            .set("causesResolved", (int64_t)rs.causesResolved).set("tetherWarnings", (int64_t)rs.tetherWarnings).set("lateJoinHeld", (int64_t)rulesStats_.lateJoinHeld)
            .set("lateJoinWarped", (int64_t)rulesStats_.lateJoinWarped).set("hostEffectsApplied", (int64_t)rulesStats_.hostEffectsApplied)
            .set("nearReports", (int64_t)rulesStats_.nearReports).set("rulesLinks", (int64_t)rulesStats_.rulesLinks).set("deaths", (int64_t)rulesStats_.deaths)
            .set("perSlotRuns", (int64_t)rulesStats_.perSlotRuns).set("itemCountAsks", (int64_t)rulesStats_.itemCountAsks).set("distanceAsks", (int64_t)rulesStats_.distanceAsks)
            .set("pooledDebits", (int64_t)rs.pooledDebits).set("rangeWarpsHeld", (int64_t)rs.rangeWarpsHeld);
        Value uk = Value::array();
        for (auto& k : rules_->unsupportedKinds()) uk.push(k);
        r.set("unsupportedKinds", uk);
        o.set("rules", r);
    }
    if (client_) {
        o.set("saveSuppress", client_->state().saveSuppress);
        if (client_->slot() < kMaxSlots) o.set("controlsOff", (int64_t)client_->state().ledger[client_->slot()].controlsOff);
    }
    o.set("volatileUnresolved", (int64_t)volatileKeys().unresolvedNames());
    o.set("logEntries", (int64_t)log_.size());
    return o.dump();
}

}  // namespace nvcoop::coop
