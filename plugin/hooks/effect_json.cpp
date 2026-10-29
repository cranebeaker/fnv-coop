#include "effect_json.h"
#include "fnvcoop/refop.h"

#include <cctype>
#include <cstdio>
#include <cstdlib>

namespace nvhooks {
using nvcoop::json::Value;
using namespace fnvcoop;

FormRef makeRef(std::string plugin, uint32_t id) {
    for (char& c : plugin) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    return FormRef(std::move(plugin), id);
}

std::string refToStr(const FormRef& r) {
    char b[16];
    // 6 digits for normal plugin-local ids; created refs (0xFF prefix, SpawnRef) keep all 8
    if (r.localId > 0xFFFFFFu) std::snprintf(b, sizeof b, ":%08X", r.localId);
    else std::snprintf(b, sizeof b, ":%06X", r.localId);
    return r.plugin + b;
}
bool strToRef(const std::string& s, FormRef* out) {
    size_t c = s.rfind(':');
    if (c == std::string::npos || c == 0 || c + 1 >= s.size()) return false;
    char* end = nullptr;
    unsigned long id = std::strtoul(s.c_str() + c + 1, &end, 16);
    if (*end) return false;
    *out = makeRef(s.substr(0, c), static_cast<uint32_t>(id));
    return true;
}

static std::string toStr(const Recipient& r) {
    switch (r.mode) {
        case Recipient::Default: return "default";
        case Recipient::Origin: return "origin";
        case Recipient::All: return "all";
        case Recipient::Mask: return "mask:" + std::to_string(r.mask);
    }
    return "default";
}
static bool toRecipient(const std::string& s, Recipient* r) {
    if (s.empty() || s == "default") { *r = Recipient::dflt(); return true; }
    if (s == "origin") { *r = Recipient::origin(); return true; }
    if (s == "all") { *r = Recipient::all(); return true; }
    if (s.rfind("mask:", 0) == 0) { *r = Recipient::slots(static_cast<uint8_t>(std::atoi(s.c_str() + 5))); return true; }
    return false;
}

static const char* effectNameLocal(const Effect& e) {
    static const char* n[] = {"SetStage", "SetObjective", "SetGlobal", "EnableRef", "DisableRef", "AddItem", "RemoveItem",
                              "ModActorValue", "ModReputation", "ModKarma", "StartQuest", "StopQuest", "MoveRef", "KillActor",
                              "ResurrectActor", "SetScriptVar", "SetPlayerControls", "PartyTether", "SlideshowEnd", "SaveSuppress",
                              "CompanionOwner", "ForcedDialogue", "BarrierArrive", "BarrierRelease", "SayOnceFlag", "CompleteQuest",
                              "CompleteAllObjectives", "SetLock", "SetFactionRank", "SetPerk", "SwapTexture", "SetFastTravelLock",
                              "FailQuest", "SpawnRef", "DespawnRef", "ObjectState", "RefScriptVar", "RefAddItem", "RefRemoveItem",
                              "RemoveAllItems", "AddTopic", "AddNote", "RemoveNote", "SetMapMarker", "ForceActiveQuest", "RequestSave",
                              "SetWeather", "SetTimeScale", "AdvanceTime", "RefAngle", "RefCommand"};
    static_assert(std::variant_size_v<Effect> == sizeof n / sizeof *n, "effect.h gained/lost an Effect: update effectNameLocal + the JSON codec");
    return e.index() < sizeof n / sizeof *n ? n[e.index()] : "?";
}

const char* effectTypeName(const Effect& e) { return effectNameLocal(e); }

Value effectToJson(const Effect& e) {
    Value o = Value::object();
    auto T = [&](const char* t) { o.set("type", t); };
    if (auto* x = std::get_if<SetStage>(&e)) { T("SetStage"); o.set("quest", refToStr(x->quest)).set("stage", (int64_t)x->stage); }
    else if (auto* x = std::get_if<SetObjective>(&e)) { T("SetObjective"); o.set("quest", refToStr(x->quest)).set("objective", (int64_t)x->objective).set("flags", (int64_t)x->flags); }
    else if (auto* x = std::get_if<SetGlobal>(&e)) { T("SetGlobal"); o.set("global", refToStr(x->global)).set("value", (double)x->value); }
    else if (auto* x = std::get_if<EnableRef>(&e)) { T("EnableRef"); o.set("ref", refToStr(x->ref)); }
    else if (auto* x = std::get_if<DisableRef>(&e)) { T("DisableRef"); o.set("ref", refToStr(x->ref)); }
    else if (auto* x = std::get_if<AddItem>(&e)) { T("AddItem"); o.set("item", refToStr(x->item)).set("count", (int64_t)x->count).set("to", toStr(x->to)); }
    else if (auto* x = std::get_if<RemoveItem>(&e)) { T("RemoveItem"); o.set("item", refToStr(x->item)).set("count", (int64_t)x->count).set("to", toStr(x->to)); }
    else if (auto* x = std::get_if<ModActorValue>(&e)) { T("ModActorValue"); o.set("av", (int64_t)x->av).set("delta", (int64_t)x->delta).set("to", toStr(x->to)); }
    else if (auto* x = std::get_if<ModReputation>(&e)) { T("ModReputation"); o.set("faction", refToStr(x->faction)).set("fame", (int64_t)x->fame).set("infamy", (int64_t)x->infamy).set("to", toStr(x->to)); }
    else if (auto* x = std::get_if<ModKarma>(&e)) { T("ModKarma"); o.set("delta", (int64_t)x->delta).set("to", toStr(x->to)); }
    else if (auto* x = std::get_if<StartQuest>(&e)) { T("StartQuest"); o.set("quest", refToStr(x->quest)); }
    else if (auto* x = std::get_if<StopQuest>(&e)) { T("StopQuest"); o.set("quest", refToStr(x->quest)); }
    else if (auto* x = std::get_if<MoveRef>(&e)) { T("MoveRef"); o.set("ref", refToStr(x->ref)).set("cell", x->cell.plugin.empty() ? std::string() : refToStr(x->cell)).set("x", (double)x->x).set("y", (double)x->y).set("z", (double)x->z); }
    else if (auto* x = std::get_if<KillActor>(&e)) { T("KillActor"); o.set("actor", refToStr(x->actor)); }
    else if (auto* x = std::get_if<ResurrectActor>(&e)) { T("ResurrectActor"); o.set("actor", refToStr(x->actor)); }
    else if (auto* x = std::get_if<SetScriptVar>(&e)) { T("SetScriptVar"); o.set("owner", refToStr(x->owner)).set("var", (int64_t)x->index).set("value", x->value); }
    else if (auto* x = std::get_if<CompleteQuest>(&e)) { T("CompleteQuest"); o.set("quest", refToStr(x->quest)); }
    else if (auto* x = std::get_if<SetPlayerControls>(&e)) { T("SetPlayerControls"); o.set("controls", (int64_t)x->controls).set("enable", x->enable).set("to", toStr(x->to)); }
    else if (auto* x = std::get_if<FailQuest>(&e)) { T("FailQuest"); o.set("quest", refToStr(x->quest)); }
    else if (auto* x = std::get_if<CompleteAllObjectives>(&e)) { T("CompleteAllObjectives"); o.set("quest", refToStr(x->quest)); }
    else if (auto* x = std::get_if<SetLock>(&e)) { T("SetLock"); o.set("ref", refToStr(x->ref)).set("locked", x->locked).set("level", (int64_t)x->level); if (x->hasBefore) o.set("beforeLocked", x->beforeLocked).set("beforeLevel", (int64_t)x->beforeLevel); }
    else if (auto* x = std::get_if<SetFactionRank>(&e)) { T("SetFactionRank"); o.set("actor", refToStr(x->actor)).set("faction", refToStr(x->faction)).set("rank", (int64_t)x->rank); }
    else if (auto* x = std::get_if<SetPerk>(&e)) { T("SetPerk"); o.set("perk", refToStr(x->perk)).set("rank", (int64_t)x->rank).set("to", toStr(x->to)); }
    else if (auto* x = std::get_if<SetMapMarker>(&e)) { T("SetMapMarker"); o.set("marker", refToStr(x->marker)).set("flags", (int64_t)x->flags); if (x->hasBefore) o.set("before", (int64_t)x->before); }
    else if (auto* x = std::get_if<CompanionOwner>(&e)) { T("CompanionOwner"); o.set("npc", refToStr(x->npc)).set("owner", (int64_t)x->owner); }
    else if (auto* x = std::get_if<ObjectState>(&e)) { T("ObjectState"); o.set("ref", refToStr(x->ref)).set("mask", (int64_t)x->mask).set("open", x->open).set("locked", x->locked).set("level", (int64_t)x->level).set("destroyed", x->destroyed).set("activatedOnce", x->activatedOnce); if (x->hasBefore) o.set("beforeLocked", x->beforeLocked).set("beforeLevel", (int64_t)x->beforeLevel); }
    else if (auto* x = std::get_if<RefAddItem>(&e)) { T("RefAddItem"); o.set("ref", refToStr(x->ref)).set("item", refToStr(x->item)).set("count", (int64_t)x->count); }
    else if (auto* x = std::get_if<RefRemoveItem>(&e)) { T("RefRemoveItem"); o.set("ref", refToStr(x->ref)).set("item", refToStr(x->item)).set("count", (int64_t)x->count); }
    else if (auto* x = std::get_if<RemoveAllItems>(&e)) { T("RemoveAllItems"); o.set("ref", refToStr(x->ref)); if (x->undo) o.set("undo", true);
        if (x->undo || x->hasBefore) { Value it = Value::array(); for (auto& [f, c] : x->items) { Value p = Value::object(); p.set("item", refToStr(f)).set("count", (int64_t)c); it.push(p); } o.set("items", it); } }
    else if (auto* x = std::get_if<RefScriptVar>(&e)) { T("RefScriptVar"); o.set("ref", refToStr(x->ref)).set("var", (int64_t)x->index).set("value", x->value); if (x->hasBefore) o.set("before", x->before); }
    else if (auto* x = std::get_if<AddTopic>(&e)) { T("AddTopic"); o.set("topic", refToStr(x->topic)); }
    else if (auto* x = std::get_if<AddNote>(&e)) { T("AddNote"); o.set("note", refToStr(x->note)); }
    else if (auto* x = std::get_if<RemoveNote>(&e)) { T("RemoveNote"); o.set("note", refToStr(x->note)); }
    else if (auto* x = std::get_if<RequestSave>(&e)) { T("RequestSave"); o.set("kind", (int64_t)x->kind); }
    else if (auto* x = std::get_if<SetWeather>(&e)) { T("SetWeather"); o.set("weather", refToStr(x->weather)).set("force", x->force); if (x->hasBefore) o.set("before", refToStr(x->before)).set("beforeForce", x->beforeForce); }
    else if (auto* x = std::get_if<SetTimeScale>(&e)) { T("SetTimeScale"); o.set("scale", (double)x->scale); if (x->hasBefore) o.set("before", (double)x->before); }
    else if (auto* x = std::get_if<RefAngle>(&e)) { T("RefAngle"); o.set("ref", refToStr(x->ref)).set("axis", (int64_t)x->axis).set("degrees", (double)x->degrees); if (x->hasBefore) o.set("before", (double)x->before); }
    else if (auto* x = std::get_if<SpawnRef>(&e)) { T("SpawnRef"); o.set("ref", refToStr(x->ref())).set("base", refToStr(x->base)).set("cell", x->cell.plugin.empty() ? std::string() : refToStr(x->cell)).set("x", (double)x->x).set("y", (double)x->y).set("z", (double)x->z).set("leveled", x->leveled); }
    else if (auto* x = std::get_if<DespawnRef>(&e)) { T("DespawnRef"); o.set("ref", refToStr(x->ref())); if (x->forget) o.set("forget", true); }
    else if (auto* x = std::get_if<RefCommand>(&e)) {
        T("RefCommand");
        const RefOpInfo* oi = refOpInfo(x->op);
        o.set("op", oi ? oi->name : "?").set("id", (int64_t)x->op);
        if (!x->ref.plugin.empty()) o.set("ref", refToStr(x->ref));
        if (!x->a.plugin.empty()) o.set("a", refToStr(x->a));
        if (!x->b.plugin.empty()) o.set("b", refToStr(x->b));
        o.set("i", (int64_t)x->i).set("j", (int64_t)x->j).set("k", (int64_t)x->k).set("l", (int64_t)x->l).set("f", (double)x->f).set("to", toStr(x->to));
        if (!x->s.empty()) o.set("s", x->s);
        if (refOpIsWide(x->op)) {   // the second shape: forms c / d and integer m
            if (!x->c.plugin.empty()) o.set("c", refToStr(x->c));
            if (!x->d.plugin.empty()) o.set("d", refToStr(x->d));
            o.set("m", (int64_t)x->m);
        }
    }
    else { T("Unsupported"); o.set("name", effectNameLocal(e)); }  // round-2 session-level effects: not carried by this codec
    return o;
}

static bool needRef(const Value& v, const char* k, FormRef* r, std::string* err) {
    if (!strToRef(v.get(k).asString(""), r)) { if (err) *err = std::string("bad or missing ref field: ") + k; return false; }
    return true;
}

bool effectFromJson(const Value& v, Effect* out, std::string* err) {
    std::string t = v.get("type").asString("");
    Recipient to = Recipient::dflt();
    if (!toRecipient(v.get("to").asString(""), &to)) { if (err) *err = "bad recipient"; return false; }
    FormRef r;
    if (t == "SetStage") { if (!needRef(v, "quest", &r, err)) return false; *out = SetStage{r, (uint32_t)v.get("stage").asInt()}; }
    else if (t == "SetObjective") { if (!needRef(v, "quest", &r, err)) return false; *out = SetObjective{r, (uint32_t)v.get("objective").asInt(), (uint8_t)v.get("flags").asInt()}; }
    else if (t == "SetGlobal") { if (!needRef(v, "global", &r, err)) return false; *out = SetGlobal{r, (float)v.get("value").asNumber()}; }
    else if (t == "EnableRef") { if (!needRef(v, "ref", &r, err)) return false; *out = EnableRef{r}; }
    else if (t == "DisableRef") { if (!needRef(v, "ref", &r, err)) return false; *out = DisableRef{r}; }
    else if (t == "AddItem") { if (!needRef(v, "item", &r, err)) return false; *out = AddItem{r, (int32_t)v.get("count").asInt(), to}; }
    else if (t == "RemoveItem") { if (!needRef(v, "item", &r, err)) return false; *out = RemoveItem{r, (int32_t)v.get("count").asInt(), to}; }
    else if (t == "ModActorValue") { *out = ModActorValue{(uint32_t)v.get("av").asInt(), (int32_t)v.get("delta").asInt(), to}; }
    else if (t == "ModReputation") { if (!needRef(v, "faction", &r, err)) return false; *out = ModReputation{r, (int32_t)v.get("fame").asInt(), (int32_t)v.get("infamy").asInt(), to}; }
    else if (t == "ModKarma") { *out = ModKarma{(int32_t)v.get("delta").asInt(), to}; }
    else if (t == "StartQuest") { if (!needRef(v, "quest", &r, err)) return false; *out = StartQuest{r}; }
    else if (t == "StopQuest") { if (!needRef(v, "quest", &r, err)) return false; *out = StopQuest{r}; }
    else if (t == "MoveRef") {
        if (!needRef(v, "ref", &r, err)) return false;
        FormRef c;
        strToRef(v.get("cell").asString(""), &c);
        *out = MoveRef{r, c, (float)v.get("x").asNumber(), (float)v.get("y").asNumber(), (float)v.get("z").asNumber()};
    }
    else if (t == "KillActor") { if (!needRef(v, "actor", &r, err)) return false; *out = KillActor{r}; }
    else if (t == "ResurrectActor") { if (!needRef(v, "actor", &r, err)) return false; *out = ResurrectActor{r}; }
    else if (t == "SetScriptVar") { if (!needRef(v, "owner", &r, err)) return false; *out = SetScriptVar{r, (uint16_t)v.get("var").asInt(), v.get("value").asNumber()}; }
    else if (t == "CompleteQuest") { if (!needRef(v, "quest", &r, err)) return false; *out = CompleteQuest{r}; }
    else if (t == "SetPlayerControls") { *out = SetPlayerControls{(uint32_t)v.get("controls").asInt(), v.get("enable").asBool(true), to}; }
    else if (t == "SetMapMarker") { if (!needRef(v, "marker", &r, err)) return false; SetMapMarker mm{r, (uint8_t)v.get("flags").asInt()}; if (v.get("before").isNumber()) { mm.hasBefore = true; mm.before = (uint8_t)v.get("before").asInt(); } *out = mm; }
    else if (t == "CompanionOwner") { if (!needRef(v, "npc", &r, err)) return false; *out = CompanionOwner{r, (uint8_t)v.get("owner").asInt()}; }
    else if (t == "ObjectState") {
        if (!needRef(v, "ref", &r, err)) return false;
        ObjectState o2;
        o2.ref = r; o2.mask = (uint8_t)v.get("mask").asInt(); o2.open = v.get("open").asBool(); o2.locked = v.get("locked").asBool();
        o2.level = (int32_t)v.get("level").asInt(); o2.destroyed = v.get("destroyed").asBool(); o2.activatedOnce = v.get("activatedOnce").asBool();
        if (v.get("beforeLevel").isNumber()) { o2.hasBefore = true; o2.beforeLocked = v.get("beforeLocked").asBool(); o2.beforeLevel = (int32_t)v.get("beforeLevel").asInt(); }
        *out = o2;
    }
    else if (t == "RefAddItem" || t == "RefRemoveItem") {
        FormRef it;
        if (!needRef(v, "ref", &r, err) || !needRef(v, "item", &it, err)) return false;
        if (t == "RefAddItem") *out = RefAddItem{r, it, (int32_t)v.get("count").asInt()};
        else *out = RefRemoveItem{r, it, (int32_t)v.get("count").asInt()};
    }
    else if (t == "RemoveAllItems") { if (!needRef(v, "ref", &r, err)) return false; RemoveAllItems ra; ra.ref = r; ra.undo = v.get("undo").asBool(); ra.hasBefore = v.get("items").type() == Value::Array && !ra.undo;
        for (const auto& it : v.get("items").items()) { FormRef f; if (!needRef(it, "item", &f, err)) return false; ra.items.emplace_back(f, (int32_t)it.get("count").asInt()); }
        *out = ra; }
    else if (t == "RefScriptVar") { if (!needRef(v, "ref", &r, err)) return false; RefScriptVar rv{r, (uint16_t)v.get("var").asInt(), v.get("value").asNumber()}; if (v.get("before").isNumber()) { rv.hasBefore = true; rv.before = v.get("before").asNumber(); } *out = rv; }
    else if (t == "AddTopic") { if (!needRef(v, "topic", &r, err)) return false; *out = AddTopic{r}; }
    else if (t == "AddNote") { if (!needRef(v, "note", &r, err)) return false; *out = AddNote{r}; }
    else if (t == "RemoveNote") { if (!needRef(v, "note", &r, err)) return false; *out = RemoveNote{r}; }
    else if (t == "RequestSave") { *out = RequestSave{(uint8_t)v.get("kind").asInt()}; }
    else if (t == "SetWeather") { if (!needRef(v, "weather", &r, err)) return false; SetWeather sw{r, v.get("force").asBool()}; FormRef bw; if (strToRef(v.get("before").asString(""), &bw)) { sw.hasBefore = true; sw.before = bw; sw.beforeForce = v.get("beforeForce").asBool(); } *out = sw; }
    else if (t == "SetTimeScale") { SetTimeScale ts{(float)v.get("scale").asNumber()}; if (v.get("before").isNumber()) { ts.hasBefore = true; ts.before = (float)v.get("before").asNumber(); } *out = ts; }
    else if (t == "RefCommand") {
        int64_t id = v.get("id").asInt(-1);
        if (id < 1 || id > 0xFFFF || !refOpKnown((uint16_t)id)) { if (err) *err = "RefCommand: unknown op"; return false; }
        RefCommand rc;
        rc.op = (uint16_t)id;
        FormRef f;
        if (v.get("ref").isString()) { if (!strToRef(v.get("ref").asString(""), &f)) { if (err) *err = "RefCommand: bad ref"; return false; } rc.ref = f; }
        if (v.get("a").isString()) { if (!strToRef(v.get("a").asString(""), &f)) { if (err) *err = "RefCommand: bad a"; return false; } rc.a = f; }
        if (v.get("b").isString()) { if (!strToRef(v.get("b").asString(""), &f)) { if (err) *err = "RefCommand: bad b"; return false; } rc.b = f; }
        rc.i = (int32_t)v.get("i").asInt(); rc.j = (int32_t)v.get("j").asInt(); rc.k = (int32_t)v.get("k").asInt(); rc.l = (int32_t)v.get("l").asInt();
        rc.f = (float)v.get("f").asNumber();
        if (refOpIsWide(rc.op)) {
            if (v.get("c").isString()) { if (!strToRef(v.get("c").asString(""), &f)) { if (err) *err = "RefCommand: bad c"; return false; } rc.c = f; }
            if (v.get("d").isString()) { if (!strToRef(v.get("d").asString(""), &f)) { if (err) *err = "RefCommand: bad d"; return false; } rc.d = f; }
            rc.m = (int32_t)v.get("m").asInt();
        }
        rc.s = v.get("s").asString("");
        if (rc.s.size() > 255) { if (err) *err = "RefCommand: text too long"; return false; }
        if (!toRecipient(v.get("to").asString("default"), &rc.to)) { if (err) *err = "RefCommand: bad recipient"; return false; }
        *out = rc;
    }
    else if (t == "RefAngle") { if (!needRef(v, "ref", &r, err)) return false; int64_t ax = v.get("axis").asInt(); if (ax < 0 || ax > 2) { if (err) *err = "RefAngle axis out of range"; return false; } RefAngle ra{r, (uint8_t)ax, (float)v.get("degrees").asNumber()}; if (v.get("before").isNumber()) { ra.hasBefore = true; ra.before = (float)v.get("before").asNumber(); } *out = ra; }
    else if (t == "SpawnRef") {
        FormRef base, cell;
        if (!needRef(v, "ref", &r, err) || !needRef(v, "base", &base, err)) return false;
        strToRef(v.get("cell").asString(""), &cell);
        SpawnRef sp;
        sp.id = r.localId; sp.base = base; sp.cell = cell; sp.x = (float)v.get("x").asNumber(); sp.y = (float)v.get("y").asNumber(); sp.z = (float)v.get("z").asNumber();
        sp.leveled = v.get("leveled").asBool();
        *out = sp;
    }
    else if (t == "DespawnRef") { if (!needRef(v, "ref", &r, err)) return false; DespawnRef dr{r.localId}; dr.forget = v.get("forget").asBool(); *out = dr; }
    else if (t == "FailQuest") { if (!needRef(v, "quest", &r, err)) return false; *out = FailQuest{r}; }
    else if (t == "CompleteAllObjectives") { if (!needRef(v, "quest", &r, err)) return false; *out = CompleteAllObjectives{r}; }
    else if (t == "SetLock") { if (!needRef(v, "ref", &r, err)) return false; SetLock lk{r, v.get("locked").asBool(), (int32_t)v.get("level").asInt()}; if (v.get("beforeLevel").isNumber()) { lk.hasBefore = true; lk.beforeLocked = v.get("beforeLocked").asBool(); lk.beforeLevel = (int32_t)v.get("beforeLevel").asInt(); } *out = lk; }
    else if (t == "SetFactionRank") {
        FormRef f;
        if (!needRef(v, "actor", &r, err) || !needRef(v, "faction", &f, err)) return false;
        *out = SetFactionRank{r, f, (int32_t)v.get("rank").asInt()};
    }
    else if (t == "SetPerk") { if (!needRef(v, "perk", &r, err)) return false; *out = SetPerk{r, (int32_t)v.get("rank").asInt(), to}; }
    else { if (err) *err = "unknown effect type: " + t; return false; }
    return true;
}

Value txnToJson(const EffectTxn& t) {
    Value c = Value::object();
    c.set("kind", (int64_t)t.cause.kind).set("source", t.cause.source.plugin.empty() ? std::string() : refToStr(t.cause.source)).set("aux", (int64_t)t.cause.aux);
    Value fx = Value::array();
    for (const Effect& e : t.effects) fx.push(effectToJson(e));
    Value o = Value::object();
    o.set("cause", c).set("origin", (int64_t)t.origin).set("effects", fx);
    return o;
}

bool txnFromJson(const Value& v, EffectTxn* out, std::string* err) {
    EffectTxn t;
    const Value& c = v.get("cause");
    t.cause.kind = static_cast<CauseKind>(c.get("kind").asInt());
    strToRef(c.get("source").asString(""), &t.cause.source);
    t.cause.aux = (int32_t)c.get("aux").asInt();
    t.origin = (PlayerId)v.get("origin").asInt(0);
    for (const Value& e : v.get("effects").items()) {
        Effect fx;
        if (!effectFromJson(e, &fx, err)) return false;
        t.effects.push_back(fx);
    }
    *out = t;
    return true;
}

}  // namespace nvhooks
