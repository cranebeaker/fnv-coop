#include "npc_anim_sampler.h"

#include <cmath>

#include "anim_groups.h"
#include "anim_sit.h"
#include "avatar/loco.h"      // locomotion codes + direction convention: the single definition shared with the player puppets
#include "fnvcoop_mut.h"

namespace nvcoop::actors::anim {

namespace {
constexpr uint32_t kBitWalk = 1u << 7, kBitRun = 1u << 8;
}

bool NpcAnimSampler::debounce(Flag& f, bool raw) {
    uint32_t need = FNV_MUT(1) ? 1u : (cfg_.flickerSamples ? cfg_.flickerSamples : 1u);   // twin 1: no debouncing, every flicker is published
    if (!f.init) { f.init = true; f.value = raw; f.pending = raw; f.same = 0; return f.value; }
    if (raw == f.value) { if (f.pending != f.value) ++stats_.flickersAbsorbed; f.pending = f.value; f.same = 0; return f.value; }
    if (raw == f.pending) ++f.same; else { f.pending = raw; f.same = 1; }
    if (f.same >= need) { f.value = raw; f.same = 0; }
    return f.value;
}

uint8_t NpcAnimSampler::debounceLevel(Level& l, uint8_t raw) {
    uint32_t need = FNV_MUT(71) ? 1u : (cfg_.flickerSamples ? cfg_.flickerSamples : 1u);   // twin 71: the sit state is not debounced, every flicker is published
    if (!l.init) { l.init = true; l.value = raw; l.pending = raw; l.same = 0; return l.value; }
    if (raw == l.value) { if (l.pending != l.value) ++stats_.flickersAbsorbed; l.pending = l.value; l.same = 0; return l.value; }
    if (raw == l.pending) ++l.same; else { l.pending = raw; l.same = 1; }
    if (l.same >= need) { l.value = raw; l.same = 0; }
    return l.value;
}

bool NpcAnimSampler::last(NetId id, live::AnimState& out) const {
    auto it = st_.find(id);
    if (it == st_.end() || !it->second.haveLast) return false;
    out = it->second.last;
    return true;
}

bool NpcAnimSampler::sample(NetId id, live::ActorSnapshot& snap, uint32_t nowMs) {
    ++stats_.samples;
    NpcReads r;
    if (!eng_.Read(id, r) || !r.valid) { ++stats_.unreadable; return false; }
    State& s = st_[id];
    live::AnimState a;   // starts neutral: every field is decided below

    if (r.dead) {
        // Death is a pose: nothing moves, nothing plays. (The declaration itself is the reliable MDeath; the corpse pose is a separate event.)
        a.dead = true;
        if (FNV_MUT(4)) {   // twin 4: a dead actor keeps publishing its last locomotion and action
            a.locomotion = s.haveLast ? s.last.locomotion : 0;
            a.action = s.haveLast ? s.last.action : 0;
            a.actionSeq = s.haveLast ? s.last.actionSeq : 0;
        } else {
            a.actionSeq = s.actionSeq;   // the counter is kept so a resurrect continues it
        }
        a.ragdoll = r.ragdoll;
        if (FNV_MUT(74) && s.haveLast) { a.sit = s.last.sit; a.sleep = s.last.sleep; a.furniture = s.last.furniture; }   // twin 74: a dead actor keeps publishing the furniture it sat in
        s.sit = Level{}; s.furniture = 0;   // (the pose is over: a resurrected actor is read afresh)
        s.last = a; s.haveLast = true; s.curAction = 0; s.holding = false;
        snap.anim = a;
        return true;
    }

    // ---- stance flags (debounced) ----
    a.weaponDrawn = r.creature ? false : debounce(s.weaponOut, r.weaponOut);
    a.aim = r.creature ? false : debounce(s.aiming, r.aiming);
    a.knocked = FNV_MUT(7) ? false : debounce(s.knocked, r.knocked);   // twin 7: a knockdown is never published
    a.swim = debounce(s.swimming, r.swimming);
    a.talk = debounce(s.talking, r.talking);
    a.sneak = !r.creature && (r.sneaking || (r.moveFlagsKnown && (r.moveFlags & (1u << 9)) != 0));
    a.ragdoll = r.ragdoll;
    a.jump = (r.inAir || std::fabs(snap.vel.z) > cfg_.fallVz) && !a.swim && !a.knocked && !r.ragdoll;

    // ---- furniture: the engine's sit / sleep state (debounced) and the furniture it is in ----
    {
        uint8_t raw = 0;
        if (r.sitKnown && !FNV_MUT(72)) {   // twin 72: the sit state is never read, a seated actor is published standing
            const SitView v = sitFromEngine(r.sitSleep);
            raw = static_cast<uint8_t>(static_cast<uint8_t>(v.state) | (v.bed ? 4 : 0));
        }
        const uint8_t code = debounceLevel(s.sit, raw);
        if (!s.sitKnownSeen || s.sitKnownLast != r.sitKnown) {
            // Logged on the first read of an actor and when it changes: an actor whose process does not answer the pinned getter is published standing for ever.
            if (!s.sitKnownSeen && r.sitKnown) { /* the normal case: nothing to say */ }
            else nvcoop::actors::trace("owner %s: the sit state is %s (engine sit raw=%d)", nvcoop::actors::who(id).c_str(), r.sitKnown ? "readable" : "NOT readable (published as standing)", r.sitSleep);
            s.sitKnownSeen = true; s.sitKnownLast = r.sitKnown;
        }
        if (code != s.pubSit) {
            nvcoop::actors::trace("owner %s: sit state published %d -> %d%s (engine sit raw=%d, furniture=%08X)", nvcoop::actors::who(id).c_str(), s.pubSit & 3, code & 3, (code & 4) ? " bed" : "", r.sitSleep, r.furniture);
            s.pubSit = code;
        }
        a.sit = static_cast<live::SitState>(code & 3);
        a.sleep = a.sit != live::SitState::None && !FNV_MUT(85) && (code & 4) != 0;   // twin 85: a bed is published as a chair
        if (a.sit == live::SitState::None || r.ragdoll || a.knocked) { a.sit = live::SitState::None; a.sleep = false; s.furniture = 0; }   // (a body on the floor is not in the furniture)
        else {
            if (r.furniture) s.furniture = r.furniture;
            a.furniture = FNV_MUT(73) ? r.furniture : s.furniture;   // twin 73: the reference is not kept, one empty read publishes "unknown" (the copy cannot sit)
        }
    }

    // ---- locomotion code from velocity + facing (+ the mover's walk / run bits when the backend can read them) ----
    {
        avatar::LocalRead lr;
        lr.vel = {snap.vel.x, snap.vel.y, snap.vel.z};
        lr.yaw = snap.rot.y;
        lr.haveMoveFlags = r.moveFlagsKnown;
        lr.moverRun = (r.moveFlags & kBitRun) != 0;
        lr.moverWalk = (r.moveFlags & kBitWalk) != 0;
        avatar::SpeedBands bands;
        bands.standMax = cfg_.standMax; bands.runEnter = cfg_.runEnter; bands.runExit = cfg_.runExit;
        uint8_t code = avatar::CaptureLoco(lr, bands);
        if (FNV_MUT(6) && code != avatar::kLocoIdle) code = avatar::MakeLoco(avatar::LocoIsRun(code), 0);   // twin 6: strafing and backing up are published as forward
        if (a.knocked || r.ragdoll) code = avatar::kLocoIdle;
        a.locomotion = code;
        s.haveLoco = true;
    }

    // ---- one-shot action ----
    const bool rawIsAction = r.actionGroup > 0 && isActionGroup(r.actionGroup);
    bool started = false;
    if (rawIsAction) {
        const bool newGroup = r.actionGroup != s.rawAction;
        const bool restarted = !newGroup && r.actionPlayMs >= 0 && s.rawPlayMs >= 0 && r.actionPlayMs + cfg_.restartBackstepMs < s.rawPlayMs;
        if (FNV_MUT(5)) started = true;                         // twin 5: every sample while it plays counts as a new event (the attack is replayed over and over)
        else started = newGroup || (restarted && !FNV_MUT(3));   // twin 3: a restart of the same group is not an event (two swings look like one)
        if (started) {
            if (restarted) ++stats_.actionsRestarted;
            ++stats_.actionsStarted;
            s.actionSeq = static_cast<uint8_t>(s.actionSeq + 1);
            s.curAction = r.actionGroup;
            s.heldUntil = nowMs + (FNV_MUT(2) ? 0u : cfg_.actionHoldMs);   // twin 2: not held, the event lives only as long as the engine's sequence
            s.holding = true;
        } else {
            // still the same sequence: keep it alive in the stream until the hold after its end runs out
            s.heldUntil = nowMs + (FNV_MUT(2) ? 0u : cfg_.actionHoldMs);
        }
    }
    s.rawAction = rawIsAction ? r.actionGroup : -1;
    s.rawPlayMs = rawIsAction ? r.actionPlayMs : -1;
    if (s.holding && !rawIsAction && static_cast<int32_t>(s.heldUntil - nowMs) <= 0) { s.holding = false; s.curAction = 0; }
    a.action = static_cast<uint8_t>(s.holding ? s.curAction : 0);
    a.actionSeq = s.actionSeq;
    a.reload = isReloadGroup(a.action) || (rawIsAction && isReloadGroup(r.actionGroup));
    a.block = !FNV_MUT(8) && (a.action == kGroupBlockIdle || (rawIsAction && r.actionGroup == kGroupBlockIdle));   // twin 8: blocking is never published

    s.last = a; s.haveLast = true;
    snap.anim = a;
    return true;
}

}  // namespace nvcoop::actors::anim
