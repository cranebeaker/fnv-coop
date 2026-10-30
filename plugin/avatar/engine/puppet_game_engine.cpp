#include "puppet_game_engine.h"

#include <algorithm>
#include <cmath>
#include <cstdio>
#include <map>
#include <set>

#include "../avatar_groups.h"
#include "../loco.h"
#include "../script_engine.h"
#include "fnvcoop_mut.h"

// Mutation twins in this file (FNVCOOP_MUT=n, tests/test_engine.cpp must fail for each):
//  1 guard off (foreign refs are touched)          2 Delete skips the tag check          3 spawn adopts an existing ref
//  4 always anchor on the player                   5 SetPos sends every axis             6 SetPos ignores missing 3D
//  7 heading not wrapped                           8 heading always re-sent              9 unknown group guessed by number
// 10 CurrentGroup = first slot only               11 restraint unreadable = not restrained  12 3D generation never bumps
// 13 recorded base not compared                   14 deleted flag ignored                 15 purge scans one cell only
// 16 purge includes the player                    17 Disable / MarkForDelete swapped      18 weapon draw does nothing
// 19 EquipItem without the no-unequip flag        20 line cost not recorded               21 appearance claims success
// 22 SameSpace ignores the worldspace             23 menus ignored                        24 GetPose never answers
// 25 spawn does not place the new ref             26 ambiguous spawn takes the oldest     27 ItemCount fallback = 0
// 28 ItemCount of a non-owned item uses fallback  29 spawn of an untagged base allowed    30 MoveToCell without anchor check
// 31 no spawn circuit breaker                     32 a new ref that fails the guard is kept   33 a placed (non 0xFF) ref with a tagged base counts as ours
// 34 puppets are ghosts (enemy shots pass through them)
// 86 the local player's place is never answered (no room is made for it)

namespace nvcoop::avatar::engine {

namespace {
bool Finite(const Vec3& v) { return std::isfinite(v.x) && std::isfinite(v.y) && std::isfinite(v.z); }
float Dist(const Vec3& a, const Vec3& b) {
    float dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}
}  // namespace

GamePuppetEngine::GamePuppetEngine(IGameBackend& be, EngineOptions opt) : be_(be), opt_(std::move(opt)) {
    costs_.reserve(opt_.costWindow);
}

std::string GamePuppetEngine::Hex(FormId f) { return FormatForm(f); }

// A puppet is always a CREATED reference (PlaceAtMe gives ids 0xFFxxxxxx). A placed reference with a tagged base - the real Sunny Smiles when
// the template were her base form - is part of the game's world and never ours, whatever its base says.
bool GamePuppetEngine::IsCreatedRef(RefId ref) const { return FNV_MUT(33) || (ref >> 24) == 0xFF; }

bool GamePuppetEngine::IsTaggedBase(FormId base) const {
    return base && std::find(opt_.taggedBases.begin(), opt_.taggedBases.end(), base) != opt_.taggedBases.end();
}

// ---- console + guards ----------------------------------------------------------------------------------------------

bool GamePuppetEngine::Run(RefId ctx, const std::string& line) {
    const uint64_t t0 = be_.NowUs();
    const int rc = be_.RunLine(ctx, line);
    const uint64_t t1 = be_.NowUs();
    ++st_.lines;
    if (rc == 0) ++st_.linesRefused;
    else if (rc == -1) ++st_.linesException;
    else if (rc == -2) ++st_.linesRefGone;
    if (!FNV_MUT(20)) {
        uint32_t c = t1 >= t0 ? static_cast<uint32_t>(std::min<uint64_t>(t1 - t0, 0xFFFFFFFFu)) : 0u;
        if (costs_.size() < opt_.costWindow) {
            costs_.push_back(c);
        } else if (!costs_.empty()) {
            costs_[costNext_] = c;
            costNext_ = (costNext_ + 1) % costs_.size();
        }
        costMax_ = std::max(costMax_, c);
    }
    return rc == 1;
}

bool GamePuppetEngine::Ours(RefId ref, RefInfo& ri, bool count) {
    bool ok;
    if (FNV_MUT(1)) {
        ok = ref && be_.ReadRef(ref, ri) && ri.exists;   // broken twin: any loaded reference (even the player) is driven
    } else {
        ok = ref && ref != kPlayerRef && IsCreatedRef(ref) && be_.ReadRef(ref, ri) && ri.exists && ri.isActor && !ri.isPlayer && IsTaggedBase(ri.base);
        if (ok && !FNV_MUT(14) && (ri.deleted || ri.disabled)) ok = false;   // Delete ran on it: no longer ours to drive
    }
    if (ok && !FNV_MUT(13)) {
        auto it = owned_.find(ref);
        if (it != owned_.end() && it->second != ri.base) ok = false;   // the id was reused by another actor with a (tagged) base
    }
    if (!ok && count) ++st_.foreignRefused;
    return ok;
}

bool GamePuppetEngine::OursWith3D(RefId ref, RefInfo& ri) {
    if (!Ours(ref, ri)) return false;
    Observe3D(ref, ri);
    if (!ri.has3D && !FNV_MUT(6)) {
        ++st_.refused3D;   // a restrained / just spawned actor refuses commands without 3D; do not pay for a console line
        return false;
    }
    return true;
}

GamePuppetEngine::RefState& GamePuppetEngine::StateOf(RefId ref, const RefInfo& ri) {
    RefState& s = state_[ref];
    if (s.base != ri.base) s = RefState{ri.base};   // a different object behind the same id: forget everything about the old one
    return s;
}

void GamePuppetEngine::Observe3D(RefId ref, const RefInfo& ri) {
    RefState& s = StateOf(ref, ri);
    if (ri.havePos3D) {
        float d = Dist(ri.pos, ri.pos3D);
        st_.maxDrift3D = std::max(st_.maxDrift3D, d);
        if (d > 20.0f) ++st_.drift3D;
    }
    if (!ri.has3D) {
        s.lost3D = true;
        return;
    }
    if (s.gen == 0) {
        s.gen = 1;
        s.node = ri.node3D;
        s.lost3D = false;
        return;
    }
    if (ri.node3D != s.node || s.lost3D) {
        if (!FNV_MUT(12)) ++s.gen;
        s.node = ri.node3D;
        s.lost3D = false;
        s.restrainedKnown = false;   // a rebuilt 3D drops the restraint: what we asked for earlier says nothing about now
    }
}

// ---- world ---------------------------------------------------------------------------------------------------------

bool GamePuppetEngine::WorldReady() {
    RefInfo pl;
    return be_.ReadRef(kPlayerRef, pl) && pl.exists && pl.cell != 0 && pl.has3D;
}

uint32_t GamePuppetEngine::WorldEpoch() { return be_.WorldEpoch(); }

bool GamePuppetEngine::MenuOpen() { return FNV_MUT(23) ? false : be_.MenuOpen(); }

bool GamePuppetEngine::CellLoaded(CellId cell) {
    CellInfo ci;
    return cell && be_.ReadCell(cell, ci) && ci.exists && ci.attached;
}

bool GamePuppetEngine::SameSpace(CellId a, CellId b) {
    if (a == b) return true;
    CellInfo ca, cb;
    if (!be_.ReadCell(a, ca) || !be_.ReadCell(b, cb) || !ca.exists || !cb.exists) return false;
    if (ca.interior || cb.interior) return false;
    if (FNV_MUT(22)) return true;
    return ca.world != 0 && ca.world == cb.world;
}

// ---- life ----------------------------------------------------------------------------------------------------------

bool GamePuppetEngine::Place(RefId ref, const Vec3& pos, float yawRad) {
    RefInfo ri;
    if (!be_.ReadRef(ref, ri) || !ri.exists) return false;
    bool ok = SendAxes(ref, ri.pos, pos);
    double deg = YawRadToDegrees(yawRad);
    if (std::fabs(WrapRad(yawRad - ri.rot.z)) > opt_.yawEpsilonRad) ok = Run(ref, "SetAngle Z " + FormatNumber(deg)) && ok;
    return ok;
}

RefId GamePuppetEngine::Spawn(FormId base, CellId cell, const Vec3& pos, float yawRad) {
    ++st_.spawnCalls;
    if (!base || (!IsTaggedBase(base) && !FNV_MUT(29))) {
        ++st_.foreignRefused;   // a base we would not recognise as ours again could never be found, purged or deleted
        return 0;
    }
    if (!Finite(pos)) return 0;
    {   // circuit breaker: spawning is rare; a burst means something upstream keeps losing its refs
        const uint64_t now = be_.NowUs();
        spawnTimes_.erase(std::remove_if(spawnTimes_.begin(), spawnTimes_.end(), [&](uint64_t t) { return now - t > opt_.spawnWindowUs; }),
                          spawnTimes_.end());
        if (spawnTimes_.size() >= opt_.maxSpawnsPerWindow && !FNV_MUT(31)) {
            ++st_.spawnThrottled;
            return 0;
        }
    }
    CellInfo ci;
    if (!be_.ReadCell(cell, ci) || !ci.exists || !ci.attached) {
        ++st_.spawnNoAnchor;
        return 0;
    }
    RefInfo pl;
    const bool havePlayer = be_.ReadRef(kPlayerRef, pl) && pl.exists;
    RefId anchor = 0;
    Vec3 apos;
    if (havePlayer && (pl.cell == cell || FNV_MUT(4))) anchor = kPlayerRef;
    else anchor = be_.AnchorRef(cell, apos);
    if (!anchor) {
        ++st_.spawnNoAnchor;
        return 0;
    }
    std::vector<ActorEntry> before, after;
    be_.ListActors(cell, before);
    // what was there BEFORE the placement (id -> already gone). An id may be reused: a deleted-but-still-listed ref must not hide the
    // new object that took its id, while a ref that was alive before can never be the new one.
    std::map<RefId, bool> wasGone;
    for (const ActorEntry& e : before) {
        RefInfo old;
        if (be_.ReadRef(e.ref, old) && old.exists) wasGone[e.ref] = old.deleted || old.disabled;
    }
    if (!Run(anchor, "PlaceAtMe " + Hex(base) + " 1 0 0")) {
        ++st_.spawnRefused;
        return 0;
    }
    be_.ListActors(cell, after);
    RefId best = 0;
    int fresh = 0;
    for (const ActorEntry& e : after) {
        if (e.base != base) continue;
        if (FNV_MUT(3)) {   // broken twin: whatever actor of that base is found first
            if (!best) best = e.ref;
            continue;
        }
        RefInfo cand;
        if (!be_.ReadRef(e.ref, cand) || !cand.exists) continue;
        auto was = wasGone.find(e.ref);
        if (was != wasGone.end() && (!was->second || cand.deleted || cand.disabled)) continue;   // the same old object (alive, or still gone)
        ++fresh;
        if (FNV_MUT(26)) {
            if (!best || e.ref < best) best = e.ref;
        } else if (e.ref > best) {
            best = e.ref;   // created references get rising ids: the newest is ours
        }
    }
    if (!best) {
        ++st_.spawnNoNewRef;
        return 0;
    }
    if (fresh > 1) ++st_.spawnAmbiguous;
    spawnTimes_.push_back(be_.NowUs());
    {   // the new ref must pass our own guard, or every later call would refuse it and the driver would spawn again and again
        RefInfo ri;
        const bool ok = be_.ReadRef(best, ri) && ri.exists && ri.isActor && !ri.isPlayer && IsTaggedBase(ri.base) && !ri.deleted && !ri.disabled;
        if (!ok && !FNV_MUT(32)) {
            ++st_.spawnRejected;
            Run(best, "Disable");
            Run(best, "MarkForDelete");
            return 0;
        }
    }
    owned_[best] = base;
    state_.erase(best);
    if (!FNV_MUT(25)) Place(best, pos, yawRad);   // the first rendered frame must not show it at the anchor
    return best;
}

void GamePuppetEngine::Delete(RefId ref) {
    if (!ref) return;
    RefInfo ri;
    if (!be_.ReadRef(ref, ri) || !ri.exists) {
        owned_.erase(ref);
        state_.erase(ref);
        return;   // already gone: nothing to do, and no console line on an unknown id
    }
    if (!FNV_MUT(2) && (ref == kPlayerRef || ri.isPlayer || !ri.isActor || !IsTaggedBase(ri.base) || !IsCreatedRef(ref))) {
        ++st_.foreignRefused;
        return;   // never delete a foreign reference, whatever upstream asks
    }
    auto it = owned_.find(ref);
    if (it != owned_.end() && it->second != ri.base && !FNV_MUT(13)) {
        ++st_.foreignRefused;   // the id now belongs to something else
        owned_.erase(it);
        state_.erase(ref);
        return;
    }
    if (FNV_MUT(17)) {
        Run(ref, "MarkForDelete");
        Run(ref, "Disable");
    } else {
        Run(ref, "Disable");
        Run(ref, "MarkForDelete");
    }
    owned_.erase(ref);
    state_.erase(ref);
}

bool GamePuppetEngine::IsValid(RefId ref) {
    RefInfo ri;
    return Ours(ref, ri, false);
}

bool GamePuppetEngine::Has3D(RefId ref) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return false;
    Observe3D(ref, ri);
    return ri.has3D;
}

uint32_t GamePuppetEngine::Generation3D(RefId ref) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return 0;
    Observe3D(ref, ri);
    return StateOf(ref, ri).gen;
}

std::vector<RefId> GamePuppetEngine::FindTaggedPuppets() {
    std::vector<CellId> cells;
    be_.LoadedCells(cells);
    if (FNV_MUT(15) && cells.size() > 1) cells.resize(1);
    std::vector<RefId> out;
    for (CellId c : cells) {
        std::vector<ActorEntry> es;
        be_.ListActors(c, es);
        for (const ActorEntry& e : es) {
            if (!e.ref || !IsTaggedBase(e.base)) continue;
            if (!IsCreatedRef(e.ref) && !(FNV_MUT(16) && e.ref == kPlayerRef)) continue;
            if (e.ref == kPlayerRef && !FNV_MUT(16)) continue;
            RefInfo ri;
            if (!be_.ReadRef(e.ref, ri) || !ri.exists || !ri.isActor || !IsTaggedBase(ri.base)) continue;
            if (ri.isPlayer && !FNV_MUT(16)) continue;
            if (ri.deleted || ri.disabled) continue;
            out.push_back(e.ref);
        }
    }
    std::sort(out.begin(), out.end());
    out.erase(std::unique(out.begin(), out.end()), out.end());
    return out;
}

// ---- configuration -------------------------------------------------------------------------------------------------

bool GamePuppetEngine::SetRestrained(RefId ref, bool on) {
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    if (!Run(ref, on ? "SetRestrained 1" : "SetRestrained 0")) return false;
    RefState& s = StateOf(ref, ri);
    s.restrainedKnown = true;
    s.restrained = on;
    return true;
}

bool GamePuppetEngine::IsRestrained(RefId ref) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return false;
    double v = 0;
    if (be_.Eval(Cond::GetRestrained, ref, 0, v)) return v != 0;
    ++st_.evalFailures;
    if (FNV_MUT(11)) return false;
    const RefState& s = StateOf(ref, ri);
    return s.restrainedKnown && s.restrained;   // evaluator unavailable: trust what we last asked for
}

bool GamePuppetEngine::MakeNonCombatant(RefId ref) {
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    // A puppet stays HITTABLE by default: the combat layer needs the engine's hit event on it (an enemy of this machine shooting the puppet of a
    // remote player) to forward the damage to that player, and neutralises the damage locally. SetGhost would make hits pass through it.
    bool a = Run(ref, "SetIgnoreFriendlyHits 1");
    bool b = true;
    if (opt_.ghostPuppets || FNV_MUT(34)) b = Run(ref, "SetGhost 1");
    return a && b;
}

FormId GamePuppetEngine::LookBase(Slot slot, FormId templateBase, const Appearance& a) {
    if (!LooksByBase() || !templateBase) return templateBase;
    if (!a.known() && !FNV_MUT(961)) {   // MUTANT 961: the bare Appearance{} of a player whose look has not arrived is written into a copy (sex forced to male, nothing else)
        lookNotes_[slot] = "template, the look of this player is not known yet";
        return templateBase;
    }
    // A fresh copy for every spawn: the game keeps the head it built on the NPC base (TESNPC +0x1C4 / +0x1C8, BSFaceGenNiNode), so a copy
    // rewritten for a second look could hand the new puppet the old head. A copy is one NPC record (0x20C bytes).
    LookCopy& c = lookCopies_[slot];
    const FormId copy = be_.CloneNpcBase(templateBase);
    if (!copy) {
        ++st_.lookFailures;
        // One refusal can be a moment (the data handler busy); three in a row mean the game does not do it: every puppet is the template, as before.
        // The first version gave up for the whole session after one refusal and every puppet then wore the LOCAL player's look.
        ++cloneFailStreak_;
        if (cloneFailStreak_ >= (FNV_MUT(962) ? 1u : 3u)) looksByBase_ = false;
        lookNotes_[slot] = std::string("template, the game made no copy of the base (refusal ") + std::to_string(cloneFailStreak_) +
                           (looksByBase_ ? ", will try again" : ", given up for this session") + ")";
        return templateBase;
    }
    cloneFailStreak_ = 0;
    c = LookCopy{copy, templateBase, be_.WorldEpoch()};
    opt_.taggedBases.push_back(copy);   // a ref made from it is one of ours (Ours / FindTaggedPuppets)
    ++st_.lookCopies;
    bool face = false;
    if (!be_.WriteNpcLook(c.copy, templateBase, a, &face)) {
        ++st_.lookFailures;
        lookNotes_[slot] = "template, the look could not be written into the copy";
        return templateBase;
    }
    ++st_.lookWrites;
    if (face) ++st_.lookFaces;
    char buf[64];
    std::snprintf(buf, sizeof buf, "look copy %08X of %08X, ", static_cast<unsigned>(copy), static_cast<unsigned>(templateBase));
    lookNotes_[slot] = std::string(buf) + be_.LastLookNote();
    return c.copy;
}

bool GamePuppetEngine::ApplyAppearance(RefId ref, const Appearance& a) {
    RefInfo ri;
    if (!Ours(ref, ri)) return false;
    for (const auto& lc : lookCopies_)
        if (lc.second.copy && lc.second.copy == ri.base) return true;   // spawned from its own look copy (LookBase): the base carries the look
    // The console has no race / face / hair command (command_table.json), so the look is the template NPC's. Race and sex are
    // chosen by the template (Config::templates); the body height is matched with SetScale on the PUPPET (a calling-reference command:
    // it never touches the player or the shared template); anything finer is reported as approximate, never faked.
    // TODO-RE (static, command table + CommandInfo of a reference exe): ModifyFaceGen (String, int?, int?) and HairTint (r g b) take no
    // calling reference and ModifyFaceGen's string grammar is unknown: both could change the LOCAL player, so they are not used.
    // MatchFaceGeometry(Actor, Value) copies a face between two actors on this machine, which needs the remote player's NPC here.
    if (a.height > 0 && !FNV_MUT(80)) {
        Appearance tmpl;
        const double base = (be_.ReadAppearance(ref, tmpl) && tmpl.height > 0) ? tmpl.height / 1000.0 : 1.0;   // the puppet's own base height
        const double ratio = (a.height / 1000.0) / base;
        if (ratio >= 0.5 && ratio <= 2.0 && std::fabs(ratio - 1.0) > 0.004 && OursWith3D(ref, ri)) {
            if (Run(ref, "SetScale " + FormatNumber(ratio))) ++st_.scaleApplied;
        }
    }
    const bool exact = a.hair == 0 && a.eyes == 0 && a.faceHash == 0 && a.hairColor == 0 && a.weight == 0 && a.faceCoeffs.empty();
    if (!exact) ++st_.appearanceApproximate;
    return FNV_MUT(21) ? true : exact;
}

// ---- driving -------------------------------------------------------------------------------------------------------

bool GamePuppetEngine::SendAxes(RefId ref, const Vec3& actual, const Vec3& target) {
    static const char kAxis[3] = {'X', 'Y', 'Z'};
    const float a[3] = {actual.x, actual.y, actual.z};
    const float t[3] = {target.x, target.y, target.z};
    int sent = 0;
    bool ok = true;
    for (int i = 0; i < 3; ++i) {
        bool need = std::fabs(t[i] - a[i]) > opt_.axisEpsilon;
        if (FNV_MUT(5)) need = true;
        if (!need) continue;
        ++sent;
        ok = Run(ref, std::string("SetPos ") + kAxis[i] + " " + FormatNumber(t[i])) && ok;
    }
    if (!sent) ++st_.setPosSkipped;
    return ok;
}

bool GamePuppetEngine::SetPos(RefId ref, const Vec3& pos) {
    ++st_.setPosCalls;
    if (!Finite(pos)) return false;
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    return SendAxes(ref, ri.pos, pos);
}

bool GamePuppetEngine::SetYaw(RefId ref, float yawRad) {
    ++st_.setYawCalls;
    if (!std::isfinite(yawRad)) return false;
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    if (!FNV_MUT(8) && std::fabs(WrapRad(yawRad - ri.rot.z)) <= opt_.yawEpsilonRad) {
        ++st_.setYawSkipped;
        return true;
    }
    double deg = FNV_MUT(7) ? static_cast<double>(yawRad) * 180.0 / 3.14159265358979323846 : YawRadToDegrees(yawRad);
    return Run(ref, "SetAngle Z " + FormatNumber(deg));
}

bool GamePuppetEngine::MoveToCell(RefId ref, CellId cell, const Vec3& pos, float yawRad) {
    (void)pos;
    (void)yawRad;   // the exact spot is applied by the driver once the rebuilt 3D is there
    RefInfo ri;
    if (!Ours(ref, ri)) return false;
    CellInfo ci;
    if (!FNV_MUT(30) && (!be_.ReadCell(cell, ci) || !ci.exists || !ci.attached)) return false;
    RefInfo pl;
    RefId anchor = 0;
    Vec3 apos;
    if (be_.ReadRef(kPlayerRef, pl) && pl.exists && pl.cell == cell) anchor = kPlayerRef;
    else anchor = be_.AnchorRef(cell, apos);
    if (!anchor) return false;
    return Run(ref, "MoveToMarker " + Hex(anchor) + " 0 0 0");
}

bool GamePuppetEngine::GetPose(RefId ref, CellId& cell, Vec3& pos) {
    if (FNV_MUT(24)) return false;
    RefInfo ri;
    if (!Ours(ref, ri, false)) return false;
    cell = ri.cell;
    pos = ri.pos;
    return ri.cell != 0;
}

bool GamePuppetEngine::GetLocalPose(CellId& cell, Vec3& pos) {
    if (FNV_MUT(86)) return false;
    RefInfo pl;
    if (!be_.ReadRef(kPlayerRef, pl) || !pl.exists || !pl.isPlayer || pl.cell == 0 || !Finite(pl.pos)) return false;
    cell = pl.cell;
    pos = pl.pos;
    return true;
}

bool GamePuppetEngine::PlayGroup(RefId ref, int baseGroup, int flags) {
    const char* name = GroupName(baseGroup);
    std::string n = name ? name : "";
    if (!name) {
        if (!FNV_MUT(9)) return false;   // never guess a group name
        n = "Group" + std::to_string(baseGroup);
    }
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    if (!Run(ref, "PlayGroup " + n + ((flags & 1) ? " 1" : " 0"))) return false;
    StateOf(ref, ri).lastGroup = baseGroup;
    return true;
}

int GamePuppetEngine::CurrentGroup(RefId ref) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return -1;
    AnimRead ar;
    if (!be_.ReadAnim(ref, ar) || !ar.valid) return -1;
    if (ar.nGroups <= 0) return kGrpIdle;
    // A group counts as playing when ANY slot holds it: the slot layout of AnimData is not proven, and a wrong guess would
    // make the driver restart a looping group every few hundred milliseconds. A reset (menu, rebuilt 3D) empties every slot.
    const int want = StateOf(ref, ri).lastGroup;
    if (FNV_MUT(10)) return ar.groups[0];
    if (want >= 0)
        for (int i = 0; i < 9; ++i)
            if (ar.groups[i] == want) return want;
    for (int i = 0; i < 9; ++i)
        if (ar.groups[i] >= 0) return ar.groups[i];
    return kGrpIdle;
}

bool GamePuppetEngine::SetSneak(RefId ref, bool on) {
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    return Run(ref, on ? "SetForceSneak 1" : "SetForceSneak 0");
}

bool GamePuppetEngine::SetAlert(RefId ref, bool on) {
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    return Run(ref, on ? "SetAlert 1" : "SetAlert 0");
}

bool GamePuppetEngine::SetWeaponDrawn(RefId ref, bool on) {
    // There is no script command that draws a weapon; an alerted actor with a weapon holds it out (VaultMP drives its puppets
    // this way). TODO-RE: unmeasured here, proof_puppet.py asserts avatar.wrong_weapon == 0.
    if (FNV_MUT(18)) return false;
    return SetAlert(ref, on);
}

bool GamePuppetEngine::IsWeaponDrawn(RefId ref) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return false;
    double v = 0;
    if (be_.Eval(Cond::IsWeaponOut, ref, 0, v)) return v != 0;
    ++st_.evalFailures;
    AnimRead ar;
    return be_.ReadAnim(ref, ar) && ar.valid && ar.weaponOut;
}

// ---- equipment -----------------------------------------------------------------------------------------------------

bool GamePuppetEngine::IsDead(RefId ref) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return false;
    double v = 0;
    if (be_.Eval(Cond::GetDead, ref, 0, v)) return !FNV_MUT(85) && v != 0;
    ++st_.evalFailures;
    AnimRead ar;
    return be_.ReadAnim(ref, ar) && ar.valid && ar.dead;   // evaluator unavailable: the process knows too
}

bool GamePuppetEngine::ResetHealth(RefId ref) {
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    return Run(ref, "ResetHealth");   // command table: ResetHealth, needs a calling reference, no parameters
}

int GamePuppetEngine::WeaponMods(RefId ref, FormId weapon) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return -1;
    std::vector<EquippedItem> items;
    if (!be_.ReadEquipped(ref, items)) return -1;
    for (const EquippedItem& e : items)
        if (e.form == weapon) {
            if (!e.modsKnown && !FNV_MUT(81)) return -1;
            ++st_.modReads;
            return e.mods & kWeaponModMask;
        }
    return -1;   // not worn
}

bool GamePuppetEngine::EnableWeaponMod(RefId ref, FormId weapon, int slot) {
    if (!weapon || slot < 1 || slot > 3) return false;
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    // command table: EnableWeaponMod(Weapon, Mod Slot optional), needs a calling reference, "Mods the current weapon with the slot".
    // TODO-RE: slot numbering (1-based assumed) and whether it acts on the worn weapon of the calling actor are unmeasured; the
    // corrector verifies by reading the ExtraWeaponModFlags back and gives up after equipMaxFails refusals.
    ++st_.modLines;
    return Run(ref, "EnableWeaponMod " + Hex(weapon) + " " + std::to_string(FNV_MUT(82) ? slot - 1 : slot));
}

bool GamePuppetEngine::GetEquipped(RefId ref, std::vector<FormId>& out) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return false;
    std::vector<EquippedItem> items;
    if (!be_.ReadEquipped(ref, items)) return false;
    out.clear();
    for (const EquippedItem& e : items)
        if (e.form) out.push_back(e.form);
    return true;
}

int GamePuppetEngine::ItemCount(RefId ref, FormId item) {
    RefInfo ri;
    if (!Ours(ref, ri, false)) return 0;
    double v = 0;
    if (be_.Eval(Cond::GetItemCount, ref, item, v) && !FNV_MUT(28)) return v < 0 ? 0 : static_cast<int>(v + 0.5);
    ++st_.evalFailures;
    if (FNV_MUT(27)) return 0;
    RefState& s = StateOf(ref, ri);
    auto it = s.added.find(item);
    return it == s.added.end() ? 0 : it->second;   // evaluator unavailable: what we added ourselves
}

bool GamePuppetEngine::AddItem(RefId ref, FormId item, int count) {
    if (count <= 0 || !item) return false;
    RefInfo ri;
    if (!Ours(ref, ri)) return false;
    if (!Run(ref, "AddItem " + Hex(item) + " " + std::to_string(count) + " 1")) return false;
    StateOf(ref, ri).added[item] += count;
    return true;
}

bool GamePuppetEngine::RemoveItem(RefId ref, FormId item, int count) {
    if (count <= 0 || !item) return false;
    RefInfo ri;
    if (!Ours(ref, ri)) return false;
    if (!Run(ref, "RemoveItem " + Hex(item) + " " + std::to_string(count) + " 1")) return false;
    RefState& s = StateOf(ref, ri);
    int& have = s.added[item];
    have = std::max(0, have - count);
    return true;
}

bool GamePuppetEngine::EquipItem(RefId ref, FormId item) {
    if (!item) return false;
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    // second argument 1 = the actor's own AI must not unequip it again, third 1 = no message
    return Run(ref, "EquipItem " + Hex(item) + (FNV_MUT(19) ? " 0 1" : " 1 1"));
}

bool GamePuppetEngine::UnequipItem(RefId ref, FormId item) {
    if (!item) return false;
    RefInfo ri;
    if (!OursWith3D(ref, ri)) return false;
    return Run(ref, "UnequipItem " + Hex(item) + " 0 1");
}

// ---- stats ---------------------------------------------------------------------------------------------------------

EngineStats GamePuppetEngine::stats() const {
    EngineStats s = st_;
    if (!costs_.empty()) {
        std::vector<uint32_t> v = costs_;
        std::sort(v.begin(), v.end());
        auto at = [&](double q) { return v[std::min(v.size() - 1, static_cast<size_t>(q * static_cast<double>(v.size() - 1) + 0.5))]; };
        s.costP50Us = at(0.5);
        s.costP95Us = at(0.95);
    }
    s.costMaxUs = costMax_;
    return s;
}

std::string GamePuppetEngine::StatsJson() const {
    EngineStats s = stats();
    char buf[1500];
    std::snprintf(buf, sizeof buf,
                  "{\"lines\":%llu,\"linesRefused\":%llu,\"linesException\":%llu,\"linesRefGone\":%llu,\"evalFailures\":%llu,"
                  "\"refused3D\":%llu,\"foreignRefused\":%llu,\"spawnCalls\":%llu,\"spawnNoAnchor\":%llu,\"spawnRefused\":%llu,"
                  "\"spawnNoNewRef\":%llu,\"spawnAmbiguous\":%llu,\"spawnThrottled\":%llu,\"spawnRejected\":%llu,\"setPosCalls\":%llu,\"setPosSkipped\":%llu,\"setYawCalls\":%llu,"
                  "\"setYawSkipped\":%llu,\"appearanceApproximate\":%llu,\"scaleApplied\":%llu,\"lookCopies\":%llu,\"lookWrites\":%llu,\"lookFaces\":%llu,\"lookFailures\":%llu,\"modLines\":%llu,\"modReads\":%llu,\"drift3D\":%llu,\"maxDrift3D\":%.1f,"
                  "\"costP50Us\":%u,\"costP95Us\":%u,\"costMaxUs\":%u}",
                  (unsigned long long)s.lines, (unsigned long long)s.linesRefused, (unsigned long long)s.linesException,
                  (unsigned long long)s.linesRefGone, (unsigned long long)s.evalFailures, (unsigned long long)s.refused3D,
                  (unsigned long long)s.foreignRefused, (unsigned long long)s.spawnCalls, (unsigned long long)s.spawnNoAnchor,
                  (unsigned long long)s.spawnRefused, (unsigned long long)s.spawnNoNewRef, (unsigned long long)s.spawnAmbiguous,
                  (unsigned long long)s.spawnThrottled, (unsigned long long)s.spawnRejected,
                  (unsigned long long)s.setPosCalls, (unsigned long long)s.setPosSkipped, (unsigned long long)s.setYawCalls,
                  (unsigned long long)s.setYawSkipped, (unsigned long long)s.appearanceApproximate, (unsigned long long)s.scaleApplied,
                  (unsigned long long)s.lookCopies, (unsigned long long)s.lookWrites, (unsigned long long)s.lookFaces, (unsigned long long)s.lookFailures,
                  (unsigned long long)s.modLines, (unsigned long long)s.modReads, (unsigned long long)s.drift3D,
                  static_cast<double>(s.maxDrift3D), s.costP50Us, s.costP95Us, s.costMaxUs);
    return buf;
}

}  // namespace nvcoop::avatar::engine
