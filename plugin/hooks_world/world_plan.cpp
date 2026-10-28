#include "world_plan.h"

#include "apply_lines.h"

namespace nvworld {

bool stepRunsScripts(StepKind k) {
    if (FNV_MUT(66)) return false;   // MUTANT 66: console steps run under the gate again (the engine pretends the line ran, a native door change never reaches a client)
    return k == StepKind::ConsoleOnRef || k == StepKind::ConsoleGlobal;
}

using fnvcoop::FormRef;

const char* planStatusName(PlanStatus s) {
    switch (s) {
        case PlanStatus::Ok: return "Ok";
        case PlanStatus::FormMissing: return "FormMissing";
        case PlanStatus::Queued: return "Queued";
        case PlanStatus::Unsupported: return "Unsupported";
    }
    return "?";
}

bool handlesEffect(const fnvcoop::Effect& e) {
    return std::holds_alternative<fnvcoop::SpawnRef>(e) || std::holds_alternative<fnvcoop::DespawnRef>(e) ||
           std::holds_alternative<fnvcoop::RefScriptVar>(e) || std::holds_alternative<fnvcoop::ObjectState>(e) ||
           std::holds_alternative<fnvcoop::AdvanceTime>(e) || std::holds_alternative<fnvcoop::RequestSave>(e);
}

namespace {

Plan missing(const FormRef& r) {
    Plan p;
    p.status = PlanStatus::FormMissing;
    p.error = "form missing: " + r.plugin;
    return p;
}

// Does the form exist here? (resolver maps the ref and the engine finds the form)
bool exists(PlanEnv& env, const FormRef& r, uint32_t* id) {
    uint32_t i = env.res.fromRef(r);
    if (!i || !env.res.lookup(i)) return false;
    if (id) *id = i;
    return true;
}

Plan createStep(PlanEnv& env, const fnvcoop::SpawnRef& s) {
    Plan p;
    uint32_t base = 0, cell = 0;
    if (!exists(env, s.base, &base)) return missing(s.base);
    cell = env.res.fromRef(s.cell);  // may be 0 when the cell form is not loaded yet; the glue then places relative to the player
    Step st;
    st.kind = StepKind::CreateSpawn;
    st.spawn = s;
    st.baseID = base;
    st.cellID = cell;
    p.steps.push_back(st);
    return p;
}

}  // namespace

Plan planEffect(const fnvcoop::Effect& e, PlanEnv& env) {
    if (auto* x = std::get_if<fnvcoop::SpawnRef>(&e)) {
        bool ready = env.cellReady ? env.cellReady(x->cell) : true;
        if (FNV_MUT(25)) {
            ready = true;  // MUTANT: never queues, creates into the wrong (unloaded) cell
        }
        SpawnAction a = env.ledger.onSpawn(*x, ready);
        if (a == SpawnAction::Duplicate) return Plan{};  // replay: already have it
        if (a == SpawnAction::Queued) {
            Plan p;
            p.status = PlanStatus::Queued;
            return p;
        }
        return createStep(env, *x);
    }
    if (auto* x = std::get_if<fnvcoop::DespawnRef>(&e)) {
        DespawnResult r = env.ledger.onDespawn(*x);
        Plan p;
        switch (r.action) {
            case DespawnAction::CancelledPending:
            case DespawnAction::Unknown:
                return p;
            case DespawnAction::DeleteLocal: {
                Step st;
                st.kind = StepKind::ConsoleOnRef;
                if (FNV_MUT(23)) {
                    st.refID = x->id;  // MUTANT: host id used on the client instead of the local object
                } else {
                    st.refID = r.localRefID;
                }
                st.line = markForDeleteLine();
                p.steps.push_back(st);
                return p;
            }
        }
        return p;
    }
    if (auto* x = std::get_if<fnvcoop::RefScriptVar>(&e)) {
        uint32_t id = 0;
        if (!exists(env, x->ref, &id)) return missing(x->ref);
        Plan p;
        Step st;
        st.kind = StepKind::SetRefVar;
        st.refID = id;
        if (FNV_MUT(22)) {
            st.varIndex = (uint16_t)(x->index + 1);  // MUTANT
        } else {
            st.varIndex = x->index;
        }
        st.varValue = x->value;
        p.steps.push_back(st);
        return p;
    }
    if (auto* x = std::get_if<fnvcoop::ObjectState>(&e)) {
        uint32_t id = 0;
        if (!exists(env, x->ref, &id)) return missing(x->ref);
        Plan p;
        auto line = [&](const std::string& l) {
            Step st;
            st.kind = StepKind::ConsoleOnRef;
            st.refID = id;
            st.line = l;
            p.steps.push_back(st);
        };
        // same order and same console forms as L5's RemoteApplier used: lock, open state, destroyed
        if (x->mask & fnvcoop::kObjLock) line(x->locked ? nvhooks::lockCmd(x->level) : nvhooks::unlockCmd());
        if ((x->mask & fnvcoop::kObjOpen) && !FNV_MUT(49)) line(nvhooks::setOpenStateCmd(x->open));   // MUTANT 49: the open bit is ignored
        if (x->mask & fnvcoop::kObjDestroyed) line(nvhooks::setDestroyedCmd(x->destroyed));
        return p;
    }
    if (auto* x = std::get_if<fnvcoop::AdvanceTime>(&e)) {
        Plan p;
        Step st;
        st.kind = StepKind::AdvanceClock;
        st.minutes = x->minutes;
        p.steps.push_back(st);
        return p;
    }
    if (std::holds_alternative<fnvcoop::RequestSave>(e)) {
        Plan p;   // nothing to do on a client: a scripted autosave / ForceSave is the host's business (steps stay empty => Ok)
        if (FNV_MUT(50)) {   // MUTANT 50: the client would save too
            Step st;
            st.kind = StepKind::ConsoleGlobal;
            st.line = "Autosave";
            p.steps.push_back(st);
        }
        return p;
    }
    Plan p;
    p.status = PlanStatus::Unsupported;
    p.error = "not a world-lane effect";
    return p;
}

std::vector<Plan> planQueuedSpawns(PlanEnv& env) {
    std::vector<Plan> out;
    auto ready = [&](const fnvcoop::SpawnRef& s) { return env.cellReady ? env.cellReady(s.cell) : true; };
    for (auto& s : env.ledger.takeReady(ready)) out.push_back(createStep(env, s));
    return out;
}

Plan planWorldEffect(const WorldEffect& e, PlanEnv& env) {
    Plan p;
    auto onRef = [&](const FormRef& actor, const std::string& line) -> Plan {
        uint32_t id = 0;
        if (!exists(env, actor, &id)) return missing(actor);
        if (line.empty()) {
            Plan u;
            u.status = PlanStatus::Unsupported;
            u.error = "no console form for this effect";
            return u;
        }
        Plan q;
        Step st;
        st.kind = StepKind::ConsoleOnRef;
        st.refID = id;
        st.line = line;
        q.steps.push_back(st);
        return q;
    };
    if (auto* x = std::get_if<SetActorValueRef>(&e)) return onRef(x->actor, avLine(*x));
    if (auto* x = std::get_if<SetRestrained>(&e)) return onRef(x->actor, setRestrainedLine(x->on));
    if (auto* x = std::get_if<SetScriptPackage>(&e)) {
        uint32_t pk = 0;
        if (!exists(env, x->package, &pk)) return missing(x->package);
        return onRef(x->actor, scriptPackageLine(x->add, pk));
    }
    if (auto* x = std::get_if<SetEssential>(&e)) {
        uint32_t id = 0;
        if (!exists(env, x->actor, &id)) return missing(x->actor);
        if (FNV_MUT(24) ? false : x->refOnly)  // MUTANT 24: ref-only essential applied to the base form
            return onRef(x->actor, setEssentialLine(*x, id));
        Step st;
        st.kind = StepKind::ConsoleGlobal;
        st.line = setEssentialLine(*x, id);
        p.steps.push_back(st);
        return p;
    }
    if (auto* x = std::get_if<SetRelation>(&e)) {
        uint32_t a = 0, b = 0;
        if (!exists(env, x->a, &a)) return missing(x->a);
        if (!exists(env, x->b, &b)) return missing(x->b);
        Step st;
        st.kind = StepKind::ConsoleGlobal;
        st.line = setRelationLine(x->kind, a, b, x->flags1, x->flags2);
        p.steps.push_back(st);
        return p;
    }
    if (auto* x = std::get_if<SetIgnoreCrime>(&e)) return onRef(x->actor, ignoreCrimeLine(x->on));
    if (auto* x = std::get_if<SetOwner>(&e)) {
        if (x->clear) return onRef(x->ref, clearOwnershipLine());
        uint32_t owner = 0;
        if (!x->owner.plugin.empty() && !exists(env, x->owner, &owner)) return missing(x->owner);
        return onRef(x->ref, setOwnershipLine(owner));
    }
    if (auto* x = std::get_if<SetCellOwner>(&e)) {
        uint32_t cell = 0, owner = 0;
        if (!exists(env, x->cell, &cell)) return missing(x->cell);
        if (!exists(env, x->owner, &owner)) return missing(x->owner);
        Step st;
        st.kind = StepKind::ConsoleGlobal;
        st.line = setCellOwnershipLine(cell, owner);
        p.steps.push_back(st);
        return p;
    }
    if (auto* x = std::get_if<SetCellPublic>(&e)) {
        uint32_t cell = 0;
        if (!exists(env, x->cell, &cell)) return missing(x->cell);
        Step st;
        st.kind = StepKind::ConsoleGlobal;
        st.line = setCellPublicFlagLine(cell, x->on);
        p.steps.push_back(st);
        return p;
    }
    if (auto* x = std::get_if<SetObjectState>(&e)) {
        // the same console forms and order as the ObjectState effect (lock, open state, destroyed)
        fnvcoop::ObjectState os;
        os.ref = x->ref; os.mask = x->mask; os.open = x->open; os.locked = x->locked; os.level = x->level; os.destroyed = x->destroyed;
        return planEffect(fnvcoop::Effect(os), env);
    }
    if (auto* x = std::get_if<Present>(&e)) {
        uint32_t f = 0;
        if (!exists(env, x->form, &f)) return missing(x->form);
        Step st;
        st.kind = StepKind::ConsoleGlobal;
        st.line = presentLine(*x, f);
        p.steps.push_back(st);
        return p;
    }
    if (auto* x = std::get_if<SetFactionPlayerEnemy>(&e)) {
        uint32_t f = 0;
        if (!exists(env, x->faction, &f)) return missing(x->faction);
        Step st;
        st.kind = StepKind::ConsoleGlobal;
        st.line = setPCEnemyOfFactionLine(f, x->on);
        p.steps.push_back(st);
        return p;
    }
    p.status = PlanStatus::Unsupported;
    return p;
}

Plan planReconcile(const std::vector<fnvcoop::SpawnRef>& live, PlanEnv& env) {
    Plan p;
    for (uint32_t local : env.ledger.reconcile(live)) {
        Step st;
        st.kind = StepKind::ConsoleOnRef;
        st.refID = local;
        st.line = markForDeleteLine();
        p.steps.push_back(st);
    }
    return p;
}

namespace {
Outcome runPlan(const Plan& p, const RunPlan& run) {
    switch (p.status) {
        case PlanStatus::FormMissing: return Outcome::Missing;
        case PlanStatus::Unsupported: return Outcome::Other;
        case PlanStatus::Queued: return Outcome::Ok;
        case PlanStatus::Ok: break;
    }
    if (p.steps.empty()) return Outcome::Ok;
    return run(p);
}
}  // namespace

Outcome applyRouted(const fnvcoop::Effect& e, PlanEnv& env, const RunPlan& run, const FallbackApply& other, int attempts) {
    FormRef wait;
    if (routeEffect(e, env.ledger.map(), &wait) == Route::HoldForCreated) {
        HeldEffect h;
        h.fx = e;
        h.waitOn = wait;
        h.onCreated = true;
        h.attempts = attempts;
        env.ledger.deferred().hold(std::move(h));
        return Outcome::Ok;
    }
    Outcome o = handlesEffect(e) ? runPlan(planEffect(e, env), run) : (other ? other(e) : Outcome::Other);
    if (!FNV_MUT(37)) {
        if (o != Outcome::Ok && retriable(e, o)) {  // MUTANT 37: a form that is not loaded yet is simply lost
            HeldEffect h;
            h.fx = e;
            h.attempts = attempts;
            env.ledger.deferred().hold(std::move(h));
            return Outcome::Ok;
        }
    }
    if (!FNV_MUT(42)) {
        if (o == Outcome::Ok && std::holds_alternative<fnvcoop::SpawnRef>(e))  // MUTANT 42: follow-ups wait for the next tick
            flushDeferred(env, run, other, false);
    } // the object exists now: its follow-ups can run
    return o;
}

Outcome applyWorldRouted(const WorldEffect& e, PlanEnv& env, const RunPlan& run, int attempts) {
    FormRef wait;
    if (routeEffect(e, env.ledger.map(), &wait) == Route::HoldForCreated) {
        HeldEffect h;
        h.world = true;
        h.wfx = e;
        h.waitOn = wait;
        h.onCreated = true;
        h.attempts = attempts;
        env.ledger.deferred().hold(std::move(h));
        return Outcome::Ok;
    }
    Outcome o = runPlan(planWorldEffect(e, env), run);
    if (o != Outcome::Ok && retriable(e, o)) {
        HeldEffect h;
        h.world = true;
        h.wfx = e;
        h.attempts = attempts;
        env.ledger.deferred().hold(std::move(h));
        return Outcome::Ok;
    }
    return o;
}

size_t flushDeferred(PlanEnv& env, const RunPlan& run, const FallbackApply& other, bool retryTick, int maxAttempts) {
    std::vector<HeldEffect> ready = env.ledger.deferred().takeBound(env.ledger.map());
    if (retryTick) {
        auto r = env.ledger.deferred().takeRetries(maxAttempts);
        for (auto& h : r) ready.push_back(std::move(h));
    }
    size_t n = 0;
    for (const HeldEffect& h : ready) {
        const size_t holdsBefore = env.ledger.deferred().holds();
        Outcome o = h.world ? applyWorldRouted(h.wfx, env, run, h.attempts) : applyRouted(h.fx, env, run, other, h.attempts);
        // an effect that was held again (target still not loaded) answers Ok too, but it did not run
        if (o == Outcome::Ok && env.ledger.deferred().holds() == holdsBefore) ++n;
    }
    return n;
}

}  // namespace nvworld
