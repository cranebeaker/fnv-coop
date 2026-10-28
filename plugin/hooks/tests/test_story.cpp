// Story-capture unit tests (game-free): the RefCommand op table against the engine data, capture -> command -> console line, who performs what,
// and the JSON codec. Runtime mutation twins (tools/mut/fnvcoop_mut.h) 1..6 live in ref_ops.cpp; each MUST make this binary fail.
// Output ends with "<n> checks, <m> failed" (run_twins.py --real-regex ", 0 failed").
#include <cstdio>
#include <cstring>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "../effect_json.h"
#include "../engine_addrs.h"
#include "../hook_patch.h"
#include "../move_queue.h"
#include "../ref_ops.h"
#include "../said_once.h"
#include "../guest_item.h"
#include "../spectator_say.h"
#include "../suppress_policy.h"
#include "../story_addrs.h"
#include "../story_animgroups.h"
#include "common/minijson.h"
#include "common/nvse_opcode_base.h"
#include "fnvcoop/refop.h"

using namespace nvhooks;
using fnvcoop::FormRef;
using fnvcoop::RefCommand;

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                       \
    do {                                                               \
        ++g_checks;                                                    \
        if (!(c)) {                                                    \
            ++g_fail;                                                  \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); \
        }                                                              \
    } while (0)
#define TEST(name) static void name()
static void run(const char* n, void (*f)()) {
    int before = g_fail;
    f();
    std::printf("[%s] %s\n", g_fail == before ? " ok " : "FAIL", n);
}

namespace {
FormRef Q(uint32_t i) { return FormRef("falloutnv.esm", i); }
// runtime ids on this fake machine: plugin "falloutnv.esm" has index 0, "honesthearts.esm" index 2
RefToForm toForm = [](uint32_t id, FormRef* out) {
    if ((id >> 24) == 0) { *out = FormRef("falloutnv.esm", id & 0xFFFFFF); return true; }
    if ((id >> 24) == 2) { *out = FormRef("honesthearts.esm", id & 0xFFFFFF); return true; }
    return false;   // dynamic refs (0xFF...) and unknown plugins have no stable name
};
FormToRef fromForm = [](const FormRef& r) -> uint32_t {
    if (r.plugin == "falloutnv.esm" && r.localId < 0x1000000) return r.localId;
    if (r.plugin == "honesthearts.esm") return 0x02000000u | r.localId;
    return 0;
};
const RefOpSpec& spec(uint16_t op) {
    const RefOpSpec* s = storySpecFor(op);
    static RefOpSpec none{};
    return s ? *s : none;
}
std::string readFile(const std::string& path) {
    std::string text;
    FILE* f = std::fopen(path.c_str(), "rb");
    if (!f) return text;
    char buf[4096];
    size_t n;
    while ((n = std::fread(buf, 1, sizeof buf, f)) > 0) text.append(buf, n);
    std::fclose(f);
    return text;
}
// engine ParamInfo type id -> the argument letter ref_ops.h uses ('s' = string, 'g' = animation group, '?' = not supported by this codec)
char classOfEngineType(int t) {
    switch (t) {
        case 1: case 18: return 'i';
        case 2: return 'f';
        case 5: return 'v';
        case 8: return 'x';
        case 0: return 's';
        case 10: return 'g';
        case 55: return 'c';
        default: return 'o';   // every form-pointer type: ObjectRef, Actor, Spell, Quest, Cell, Faction, Topic, FormList, ...
    }
}
}  // namespace

TEST(spec_table_matches_the_wire_op_table) {
    CHECK(kStorySpecCount == fnvcoop::kOpCount_ - 1);
    std::set<std::string> fns;
    std::set<Addr> targets;
    for (size_t i = 0; i < kStorySpecCount; ++i) {
        const RefOpSpec& s = kStorySpecs[i];
        const fnvcoop::RefOpInfo* info = fnvcoop::refOpInfo(s.op);
        CHECK(s.op == i + 1);                                       // dense, in id order
        CHECK(info != nullptr);
        if (!info) continue;
        CHECK(storySpecFor(s.op) == &s);
        CHECK(storySpecForFn(s.fn) == &s);
        CHECK(fns.insert(s.fn).second);
        for (const char* p = s.fn; *p; ++p) CHECK(!(*p >= 'A' && *p <= 'Z'));   // census keys are lower case
        CHECK(s.hook != nullptr);
        if (!s.hook) continue;
        CHECK(std::string(s.hook->name) == std::string("Cmd_") + info->name);   // the hooked handler is the command of this op
        std::string why;
        CHECK(specIsValid(*s.hook, &why));
        CHECK(targets.insert(s.hook->target).second);
        CHECK(s.hook->retBytes == 0);                               // script command handlers are plain cdecl
        CHECK(s.hook->target >= 0x401000 && s.hook->target < 0xFDE58B);
        CHECK(std::strlen(s.sig) == std::strlen(s.slots));
        CHECK(std::strlen(s.sig) <= (size_t)kMaxStoryArgs);
        CHECK(s.minArgs <= std::strlen(s.sig));
        const bool shared = (s.flags & kRoSharedHook) != 0;
        bool inEngineTable = false;                                 // a shared hook is one of engine_addrs.h's own entries
        for (const HookSpec* h : addr::kAll) inEngineTable |= (h == s.hook);
        CHECK(shared == inEngineTable);
        bool inStoryTable = false;
        for (const HookSpec* h : story_addr::kAll) inStoryTable |= (h == s.hook);
        CHECK(shared == !inStoryTable);
    }
    CHECK(storySpecFor(0) == nullptr && storySpecFor(fnvcoop::kOpCount_) == nullptr && storySpecForFn("nosuchfn") == nullptr);
    // every hook in the generated story table is either an op or the one dedicated handler (ForceActiveQuest, an existing Effect type)
    size_t extra = 0;
    for (const HookSpec* h : story_addr::kAll) {
        bool used = false;
        for (size_t i = 0; i < kStorySpecCount; ++i) used |= kStorySpecs[i].hook == h;
        if (!used) {
            ++extra;
            CHECK(std::string(h->name) == "Cmd_ForceActiveQuest");
        }
    }
    CHECK(extra == 1);
}

TEST(story_address_table_is_sane) {
    std::set<Addr> seen;
    for (const HookSpec* h : story_addr::kAll) {
        std::string why;
        CHECK(specIsValid(*h, &why));
        CHECK(seen.insert(h->target).second);
        CHECK(h->expected[0] == 0x55 && h->expected[1] == 0x8b && h->expected[2] == 0xec);   // standard frame prologue
    }
    // no story hook sits on an address the main table already hooks under another name
    std::set<Addr> main;
    for (const HookSpec* h : addr::kAll) main.insert(h->target);
    for (const HookSpec* h : story_addr::kAll) CHECK(!main.count(h->target));
}

TEST(argument_layout_matches_the_engine_command_table) {
    nvcoop::json::Value root;
    std::string err;
    CHECK(nvcoop::json::parse(readFile(std::string(NVH_DATA_DIR) + "/story_cmd_params.json"), root, err));
    for (size_t i = 0; i < kStorySpecCount; ++i) {
        const RefOpSpec& s = kStorySpecs[i];
        const fnvcoop::RefOpInfo* info = fnvcoop::refOpInfo(s.op);
        if (!info) continue;
        const nvcoop::json::Value& e = root.get(info->name);
        CHECK(e.isObject());
        if (!e.isObject()) continue;
        char exec[16];
        std::snprintf(exec, sizeof exec, "0x%x", s.hook->target);
        CHECK(e.get("execute").asString("") == exec);                   // the table's handler is the function we pin
        const auto& params = e.get("params").items();
        CHECK(params.size() == std::strlen(s.sig));                     // one letter per ENGINE parameter
        const bool parent = e.get("parent").asInt() != 0;
        if (parent) CHECK((s.flags & kRoNeedsRef) != 0);                // the engine insists on a calling reference
        bool usedSlot[256] = {};
        for (size_t k = 0; k < params.size() && k < std::strlen(s.sig); ++k) {
            const int type = (int)params[k].items()[0].asInt();
            const bool optional = params[k].items()[1].asInt() != 0;
            CHECK(classOfEngineType(type) == s.sig[k]);
            CHECK(optional == (k >= s.minArgs));                         // minArgs = the number of non-optional parameters
            const char d = s.slots[k];
            const bool wide = fnvcoop::refOpIsWide(s.op);                 // the second shape also has the forms c d and the integer m
            if (s.sig[k] == 'o') CHECK(d == 'a' || d == 'b' || (wide && (d == 'c' || d == 'd')));
            else if (s.sig[k] == 'f') CHECK(d == 'f');
            else if (s.sig[k] == 's') CHECK(d == 's');
            else CHECK(d == 'i' || d == 'j' || d == 'k' || d == 'l' || (wide && d == 'm'));
            CHECK(!usedSlot[(unsigned char)d]);                          // two parameters never share a slot
            usedSlot[(unsigned char)d] = true;
        }
        // the slots the wire key relies on are filled by this op
        if (info->kind == fnvcoop::kRefOpState) {
            if (info->key & fnvcoop::kKeyA) CHECK(usedSlot['a']);
            if (info->key & fnvcoop::kKeyB) CHECK(usedSlot['b']);
            if (info->key & fnvcoop::kKeyRef) CHECK((s.flags & kRoNeedsRef) != 0);
        }
    }
}

TEST(capture_turns_extracted_arguments_into_commands) {
    ArgValue a[kMaxStoryArgs];
    // StartCombat target
    a[0].form = 0x14;
    RefCommand c;
    std::string why;
    CHECK(buildRefCommand(spec(fnvcoop::kOpStartCombat), 0x00112233, a, toForm, &c, &why));
    CHECK(c.op == fnvcoop::kOpStartCombat && c.ref == Q(0x112233) && c.a == Q(0x14) && c.to.mode == fnvcoop::Recipient::Default);
    // a command that needs a calling reference and has none is not captured; the target's name must be stable
    CHECK(!buildRefCommand(spec(fnvcoop::kOpStartCombat), 0, a, toForm, &c, &why));
    ArgValue dyn[kMaxStoryArgs];
    dyn[0].form = 0xFF000123;
    CHECK(!buildRefCommand(spec(fnvcoop::kOpStartCombat), 0x00112233, dyn, toForm, &c, &why));
    CHECK(!buildRefCommand(spec(fnvcoop::kOpStartCombat), 0xFF000045, a, toForm, &c, &why));   // calling reference is a dynamic ref with no name
    ArgValue none[kMaxStoryArgs];
    CHECK(!buildRefCommand(spec(fnvcoop::kOpStartCombat), 0x00112233, none, toForm, &c, &why));   // the required target is missing
    // integers, floats, several slots
    ArgValue lv[kMaxStoryArgs];
    lv[0].i = 12; lv[1].i = 3;
    CHECK(buildRefCommand(spec(fnvcoop::kOpSetLevel), 0, lv, toForm, &c, &why));   // PLY: the player is implied
    CHECK(c.i == 12 && c.j == 3 && c.k == 0 && c.ref.plugin.empty());
    CHECK(c.to == fnvcoop::Recipient::origin());                                      // chargen / progression goes to the acting player only
    ArgValue al[kMaxStoryArgs];
    al[0].f = 0.25f;
    al[1].f = 0.25f;
    CHECK(buildRefCommand(spec(fnvcoop::kOpSetActorAlpha), 0x00AB, al, toForm, &c, &why) && c.f == 0.25f);
    ArgValue ll[kMaxStoryArgs];
    ll[0].form = 0x02000010; ll[1].form = 0x00000020; ll[2].i = 5; ll[3].i = 2; ll[4].f = 0.5f;
    CHECK(buildRefCommand(spec(fnvcoop::kOpAddItemToLeveledList), 0, ll, toForm, &c, &why));
    CHECK(c.a == FormRef("honesthearts.esm", 0x10) && c.b == Q(0x20) && c.i == 5 && c.j == 2 && c.f == 0.5f && c.ref.plugin.empty());
    // achievements reach everyone
    ArgValue ach[kMaxStoryArgs];
    ach[0].i = 9;
    CHECK(buildRefCommand(spec(fnvcoop::kOpAddAchievement), 0, ach, toForm, &c, &why) && c.to == fnvcoop::Recipient::all());
    // Rotate: the axis letter becomes 0 1 2; anything else is refused
    ArgValue rot[kMaxStoryArgs];
    rot[0].i = 'Y'; rot[1].i = 30;
    CHECK(buildRefCommand(spec(fnvcoop::kOpRotate), 0x44, rot, toForm, &c, &why) && c.i == 1 && c.j == 30);
    rot[0].i = 'q';
    CHECK(!buildRefCommand(spec(fnvcoop::kOpRotate), 0x44, rot, toForm, &c, &why));
}

TEST(commands_become_exact_console_lines) {
    auto line = [&](RefCommand c) { return buildRefCommandLine(c, fromForm); };
    RefCommand c;
    c.op = fnvcoop::kOpStartCombat; c.ref = Q(0x112233); c.a = Q(0x14);
    LineResult r = line(c);
    CHECK(r.ok && r.line == "StartCombat 00000014" && r.callingRef == 0x112233);
    c = RefCommand(); c.op = fnvcoop::kOpEvaluatePackage; c.ref = Q(0x77);
    r = line(c);
    CHECK(r.ok && r.line == "EvaluatePackage" && r.callingRef == 0x77);
    // optional trailing parameters at their default are left out, set ones are kept in order
    c = RefCommand(); c.op = fnvcoop::kOpForceFlee; c.ref = Q(0x77);
    CHECK(line(c).line == "ForceFlee");
    c.b = Q(0x90);
    CHECK(!line(c).ok);                                                    // an optional form missing before a later one cannot be expressed
    c.a = Q(0x91);
    CHECK(line(c).line == "ForceFlee 00000091 00000090");
    c = RefCommand(); c.op = fnvcoop::kOpSetLevel; c.i = 5;
    r = line(c);
    CHECK(r.ok && r.line == "SetLevel 5" && r.callingRef == 0x14);        // the player is the calling reference when none was given
    c.j = 2;
    CHECK(line(c).line == "SetLevel 5 2");
    c = RefCommand(); c.op = fnvcoop::kOpSetCombatStyle; c.ref = Q(0x500);
    CHECK(line(c).line == "SetCombatStyle");                              // no combat style given
    c.a = FormRef("honesthearts.esm", 0x1234);
    CHECK(line(c).line == "SetCombatStyle 02001234");
    c = RefCommand(); c.op = fnvcoop::kOpSetActorAlpha; c.ref = Q(1); c.f = 0.5f;
    CHECK(line(c).line == "SetActorAlpha 0.5000");
    c = RefCommand(); c.op = fnvcoop::kOpRotate; c.ref = Q(1); c.i = 2; c.j = 15;
    CHECK(line(c).line == "Rotate Z 15");
    c = RefCommand(); c.op = fnvcoop::kOpSetPlayerTagSkill; c.i = 43; c.j = 1;
    CHECK(line(c).line == "SetPlayerTagSkill Speech 1");
    c.i = 5;
    CHECK(!line(c).ok);                                                    // not a skill: no console name
    c = RefCommand(); c.op = fnvcoop::kOpAddItemToLeveledList; c.a = Q(0x10); c.b = Q(0x11); c.i = 1; c.j = 1; c.f = 0.75f;
    CHECK(line(c).line == "AddItemToLeveledList 00000010 00000011 1 1 0.7500");
    c = RefCommand(); c.op = fnvcoop::kOpEquipItem; c.ref = Q(3); c.a = Q(4); c.i = 1;
    CHECK(line(c).line == "EquipItem 00000004 1");
    // forms that are not loaded on this machine
    c = RefCommand(); c.op = fnvcoop::kOpAddSpell; c.ref = Q(3); c.a = FormRef("deadmoney.esm", 5);
    r = line(c);
    CHECK(!r.ok && r.formMissing);
    c.a = Q(5); c.ref = FormRef("deadmoney.esm", 9);
    r = line(c);
    CHECK(!r.ok && r.formMissing);
    // a command that needs a reference refuses to run without one; unknown ops never produce a line
    c = RefCommand(); c.op = fnvcoop::kOpResetAI;
    CHECK(!line(c).ok);
    c.op = 0;
    CHECK(!line(c).ok);
    c.op = 9999;
    CHECK(!line(c).ok);
    // the line is the engine's own long command name
    for (size_t i = 0; i < kStorySpecCount; ++i) {
        RefCommand k;
        k.op = kStorySpecs[i].op;
        k.ref = Q(1); k.a = Q(2); k.b = Q(3); k.c = Q(4); k.d = Q(5); k.i = 1; k.j = 1; k.k = 1; k.l = 1; k.m = 1; k.f = 1.f; k.s = "IdleStretch";
        if (k.op == fnvcoop::kOpSetPlayerTagSkill) k.i = 33;
        LineResult lr = line(k);
        CHECK(lr.ok);
        CHECK(lr.line.rfind(fnvcoop::refOpInfo(k.op)->name, 0) == 0);
    }
}

TEST(only_the_owner_performs_actor_ai_orders) {
    auto remote = [](uint32_t ref) { return ref == 0x500; };   // the actor 0x500 is driven from snapshots on this machine
    CHECK(!opPerformedHere(spec(fnvcoop::kOpStartCombat), 0x500, remote));
    CHECK(opPerformedHere(spec(fnvcoop::kOpStartCombat), 0x501, remote));          // my own actor
    CHECK(opPerformedHere(spec(fnvcoop::kOpStartCombat), 0x500, nullptr));         // no ownership layer: perform
    CHECK(opPerformedHere(spec(fnvcoop::kOpStartCombat), 0, remote));              // nothing to ask about
    CHECK(opPerformedHere(spec(fnvcoop::kOpSetGhost), 0x500, remote));             // a flag is state: every machine keeps it
    CHECK(opPerformedHere(spec(fnvcoop::kOpAddSpell), 0x500, remote));
    CHECK(!opPerformedHere(spec(fnvcoop::kOpEvaluatePackage), 0x500, remote));
    CHECK(!opPerformedHere(spec(fnvcoop::kOpFireWeapon), 0x500, remote));
    size_t ai = 0;
    for (size_t i = 0; i < kStorySpecCount; ++i) ai += (kStorySpecs[i].flags & kRoAiOrder) != 0;
    CHECK(ai == 18);   // 17 + UseWeapon (the shooter's AI performs it)
}

TEST(activate_is_limited_to_object_types_without_a_menu) {
    CHECK(activateTypeSafe(0x15));    // TESObjectACTI
    CHECK(activateTypeSafe(0x1C));    // TESObjectDOOR
    CHECK(!activateTypeSafe(0x1B));   // container
    CHECK(!activateTypeSafe(0x17));   // terminal
    CHECK(!activateTypeSafe(0x16));   // talking activator
    CHECK(!activateTypeSafe(0x2A));   // NPC base
    CHECK(!activateTypeSafe(0));
}

TEST(animation_groups_come_from_the_engine_table_and_play_as_names) {
    CHECK(kAnimGroupCount == 245);
    CHECK(std::string(kAnimGroupNames[0]) == "Idle" && std::string(kAnimGroupNames[3]) == "Forward" && std::string(kAnimGroupNames[244]) == "JumpLandRight");
    std::set<std::string> seen;
    for (int i = 0; i < kAnimGroupCount; ++i) CHECK(kAnimGroupNames[i] && *kAnimGroupNames[i] && seen.insert(kAnimGroupNames[i]).second);
    ArgValue a[kMaxStoryArgs];
    a[0].i = 3; a[1].i = 4;
    RefCommand c;
    std::string why;
    CHECK(buildRefCommand(spec(fnvcoop::kOpPlayGroup), 0x77, a, toForm, &c, &why) && c.i == 3 && c.j == 4);
    CHECK(buildRefCommandLine(c, fromForm).line == "PlayGroup Forward 4");
    a[0].i = 245;
    CHECK(!buildRefCommand(spec(fnvcoop::kOpPlayGroup), 0x77, a, toForm, &c, &why));
    c.i = -1;
    CHECK(!buildRefCommandLine(c, fromForm).ok);
    // the faction delta is a command of its own (the rank it leads to depends on the replica's rank)
    ArgValue f[kMaxStoryArgs];
    f[0].form = 0x1234; f[1].i = -3;
    CHECK(buildRefCommand(spec(fnvcoop::kOpModFactionRank), 0x55, f, toForm, &c, &why));
    CHECK(buildRefCommandLine(c, fromForm).line == "ModFactionRank 00001234 -3");
}

TEST(text_arguments_travel_as_plain_words) {
    ArgValue a[kMaxStoryArgs];
    a[0].s = "Idle_Stretch-01";
    RefCommand c;
    std::string why;
    CHECK(buildRefCommand(spec(fnvcoop::kOpPlayIdle), 0x77, a, toForm, &c, &why) && c.s == "Idle_Stretch-01");
    LineResult r = buildRefCommandLine(c, fromForm);
    CHECK(r.ok && r.line == "PlayIdle Idle_Stretch-01" && r.callingRef == 0x77);
    a[0].s = "";
    CHECK(!buildRefCommand(spec(fnvcoop::kOpPlayIdle), 0x77, a, toForm, &c, &why));            // required text missing
    a[0].s = "two words";
    CHECK(!buildRefCommand(spec(fnvcoop::kOpPlayIdle), 0x77, a, toForm, &c, &why));            // would be two console tokens
    a[0].s = "x; Kill";
    CHECK(!buildRefCommand(spec(fnvcoop::kOpPlayIdle), 0x77, a, toForm, &c, &why));            // a script cannot smuggle a second command
    a[0].s = std::string(256, 'a');
    CHECK(!buildRefCommand(spec(fnvcoop::kOpPlayIdle), 0x77, a, toForm, &c, &why));
    c.s = "evil\nKill";
    CHECK(!buildRefCommandLine(c, fromForm).ok);                                                 // the same check on the receiving side
    c.s = "";
    CHECK(!buildRefCommandLine(c, fromForm).ok);
    // JSON keeps the text
    c.s = "IdleWave";
    fnvcoop::Effect e = c, back;
    std::string err;
    CHECK(effectFromJson(effectToJson(e), &back, &err) && back == e);
}

TEST(critical_stages_come_from_the_engine_table) {
    CHECK(kCriticalStageCount == 5);
    CHECK(std::string(kCriticalStageNames[0]) == "None" && std::string(kCriticalStageNames[1]) == "GooStart" && std::string(kCriticalStageNames[4]) == "DisintegrateEnd");
    ArgValue a[kMaxStoryArgs];
    a[0].i = 3;
    RefCommand c;
    std::string why;
    CHECK(buildRefCommand(spec(fnvcoop::kOpSetCriticalStage), 0x66, a, toForm, &c, &why));
    CHECK(buildRefCommandLine(c, fromForm).line == "SetCriticalStage DisintegrateStart");
    a[0].i = 5;
    CHECK(!buildRefCommand(spec(fnvcoop::kOpSetCriticalStage), 0x66, a, toForm, &c, &why));
}

TEST(skill_names_cover_barter_to_unarmed_only) {
    CHECK(std::string(skillNameOf(32)) == "Barter" && std::string(skillNameOf(45)) == "Unarmed" && std::string(skillNameOf(41)) == "Guns");
    CHECK(skillNameOf(31) == nullptr && skillNameOf(46) == nullptr && skillNameOf(-1) == nullptr);
}

TEST(json_codec_round_trips_ref_commands_and_rejects_bad_ones) {
    RefCommand c;
    c.op = fnvcoop::kOpAddItemToLeveledList; c.a = Q(0x10); c.b = FormRef("honesthearts.esm", 0x11); c.i = 3; c.j = -2; c.f = 0.75f;
    c.to = fnvcoop::Recipient::slot(2);
    fnvcoop::Effect e = c, back;
    std::string err;
    CHECK(effectFromJson(effectToJson(e), &back, &err));
    CHECK(back == e);
    c = RefCommand(); c.op = fnvcoop::kOpEvaluatePackage; c.ref = Q(5);
    e = c;
    CHECK(effectFromJson(effectToJson(e), &back, &err) && back == e);
    CHECK(effectToJson(e).get("op").asString("") == "EvaluatePackage");
    nvcoop::json::Value bad = effectToJson(e);
    bad.set("id", (int64_t)0);
    CHECK(!effectFromJson(bad, &back, &err));
    bad.set("id", (int64_t)5000);
    CHECK(!effectFromJson(bad, &back, &err));
}

TEST(capture_and_apply_agree_for_every_op) {
    // capture decoded arguments -> command -> console line, for each op with plausible arguments: nothing is lost on the way
    for (size_t i = 0; i < kStorySpecCount; ++i) {
        const RefOpSpec& s = kStorySpecs[i];
        ArgValue a[kMaxStoryArgs];
        for (size_t k = 0; k < std::strlen(s.sig); ++k) {
            if (s.sig[k] == 'o') a[k].form = 0x00001000u + (uint32_t)k;
            else if (s.sig[k] == 's') a[k].s = "IdleStretch";
            else if (s.sig[k] == 'f') a[k].f = 2.5f;
            else if (s.sig[k] == 'x') a[k].i = 'X';
            else if (s.sig[k] == 'v') a[k].i = 33;
            else a[k].i = (int32_t)k + 1;
        }
        RefCommand c;
        std::string why;
        CHECK(buildRefCommand(s, (s.flags & kRoNeedsRef) && !(s.flags & kRoPlayerRef) ? 0x2000u : 0u, a, toForm, &c, &why));
        LineResult lr = buildRefCommandLine(c, fromForm);
        CHECK(lr.ok);
        if (s.flags & kRoNeedsRef && !(s.flags & kRoPlayerRef)) CHECK(lr.callingRef == 0x2000);
    }
}

// A5 (2026-10-22): UseWeapon has nine parameters (four forms, five integers), more than a RefCommand's a / b / i..l: the second shape also fills c, d and m.
TEST(use_weapon_uses_the_second_command_shape) {
    const RefOpSpec& s = spec(fnvcoop::kOpUseWeapon);
    CHECK(s.op == fnvcoop::kOpUseWeapon && fnvcoop::refOpIsWide(fnvcoop::kOpUseWeapon) && !fnvcoop::refOpIsWide(fnvcoop::kOpStartCombat));
    CHECK(std::strlen(s.sig) == 9 && (s.flags & kRoNeedsRef) && (s.flags & kRoAiOrder));
    // `ref.useWeapon weapon target1 target2 4 1 1 0 0 extraForm` as ExtractArgs hands it over
    ArgValue a[kMaxStoryArgs];
    a[0].form = 0x00001000; a[1].form = 0x00002000; a[2].form = 0x00003000;
    a[3].i = 4; a[4].i = 1; a[5].i = 1; a[6].i = 2; a[7].i = 7;
    a[8].form = 0x02004000;
    RefCommand c;
    std::string why;
    CHECK(buildRefCommand(s, 0x00005000, a, toForm, &c, &why));
    CHECK(c.ref == Q(0x5000) && c.a == Q(0x1000) && c.b == Q(0x2000) && c.c == Q(0x3000) && c.d == FormRef("honesthearts.esm", 0x4000));
    CHECK(c.i == 4 && c.j == 1 && c.k == 1 && c.l == 2 && c.m == 7);
    LineResult r = buildRefCommandLine(c, fromForm);
    CHECK(r.ok && r.callingRef == 0x5000 && r.line == "UseWeapon 00001000 00002000 00003000 4 1 1 2 7 02004000");
    // trailing optionals at their default are left out, the required four stay
    c.d = FormRef(); c.m = 0; c.l = 0; c.k = 0; c.j = 0;
    r = buildRefCommandLine(c, fromForm);
    CHECK(r.ok && r.line == "UseWeapon 00001000 00002000 00003000 4");
    // an optional integer cannot be skipped over a later one that is set
    c.m = 3;
    CHECK(buildRefCommandLine(c, fromForm).line == "UseWeapon 00001000 00002000 00003000 4 0 0 0 3");
    // the missing weapon (required) is refused at capture
    ArgValue none[kMaxStoryArgs];
    CHECK(!buildRefCommand(s, 0x5000, none, toForm, &c, &why));
    // JSON: the second shape's slots survive
    RefCommand w;
    w.op = fnvcoop::kOpUseWeapon; w.ref = Q(7); w.a = Q(1); w.b = Q(2); w.c = Q(3); w.d = FormRef("honesthearts.esm", 4); w.i = 4; w.l = 2; w.m = 9;
    fnvcoop::Effect e = w, back;
    std::string err;
    CHECK(effectFromJson(effectToJson(e), &back, &err) && back == e);
    CHECK(effectToJson(e).get("m").asInt() == 9 && effectToJson(e).get("c").asString("") == "falloutnv.esm:000003");
}

TEST(parked_moves_keep_the_newest_and_give_up) {
    auto mv = [](uint32_t r, float x) { fnvcoop::MoveRef m; m.ref = Q(r); m.cell = Q(0x900); m.x = x; return m; };
    MoveQueue q;
    q.add(mv(1, 10.f)); q.add(mv(2, 20.f)); q.add(mv(1, 30.f));          // ref 1 moved twice while its cell was away
    CHECK(q.size() == 2 && q.holds(Q(1)) && q.holds(Q(2)) && !q.holds(Q(3)));
    std::vector<float> seen;
    CHECK(q.run([&](const fnvcoop::MoveRef& m) { seen.push_back(m.x); return false; }, 3) == 0);   // cell not loaded yet: nothing done, still parked
    CHECK(seen.size() == 2 && seen[0] == 30.f && seen[1] == 20.f);      // the newest move of ref 1, in first-parked order
    CHECK(q.size() == 2);
    seen.clear();
    CHECK(q.run([&](const fnvcoop::MoveRef& m) { seen.push_back(m.x); return m.ref == Q(2); }, 3) == 1);   // ref 2's cell loaded
    CHECK(q.size() == 1 && q.holds(Q(1)));
    CHECK(q.run([&](const fnvcoop::MoveRef&) { return false; }, 3) == 0);   // third failed attempt of ref 1: given up
    CHECK(q.size() == 0);
    q.add(mv(5, 1.f));
    q.clear();
    CHECK(q.size() == 0);
}

TEST(an_applier_without_its_own_executor_uses_the_process_wide_one) {
    int own = 0, glob = 0;
    ExecFn o = [&](const std::string&, uint32_t) { ++own; return true; };
    ExecFn g = [&](const std::string&, uint32_t) { ++glob; return true; };
    ExecFn a = pickExec(o, g);
    CHECK(a && a("x", 0) && own == 1 && glob == 0);                        // its own wins
    ExecFn b = pickExec(nullptr, g);
    CHECK(b && b("x", 0) && glob == 1);                                    // nothing of its own: the process-wide one
    CHECK(!pickExec(nullptr, nullptr));                                    // neither: no executor, console effects stay Unsupported
    CHECK(forceActiveQuestCmd(0x00ABCDEF) == "ForceActiveQuest 00ABCDEF");
}

TEST(a_said_once_effect_sets_the_flag_byte_of_the_info) {
    // door_terminal_container_matrix / pnr_vote / fast_travel_tether 2026-10-22: the client could not apply SayOnceFlag, so its dump never held the
    // host's said-once INFOs and the digest resent them every round. The flag is TESTopicInfo + 0x22.
    CHECK(kInfoSaidOnceOffset == 0x22);
    uint8_t info[0x40] = {};
    CHECK(markInfoSaidOnce(info));
    CHECK(info[0x22] == 1);
    for (int i = 0; i < 0x40; ++i) CHECK(i == 0x22 || info[i] == 0);        // only that byte
    CHECK(!markInfoSaidOnce(info) && info[0x22] == 1);                       // already set: nothing changed
    CHECK(!markInfoSaidOnce(nullptr));
}

TEST(the_opcode_base_given_to_nvse_is_one_a_release_nvse_serializes) {
    // actor_port_save_check 2026-10-22: autosaveaclgcheck.nvse had no 'ACLG' record. NVSE (Serialization.cpp HandleSaveGame) skips the save callback
    // of a plugin whose opcode base is 0, which is what the default base 0x2000 (or none at all) gives in a release NVSE.
    CHECK(nvcoop::opcodeBaseSerializes(nvcoop::opcodeBaseToRegister()));
    CHECK(nvcoop::opcodeBaseToRegister() != nvcoop::kNvseDefaultOpcodeBase);
    CHECK(!nvcoop::opcodeBaseSerializes(0) && !nvcoop::opcodeBaseSerializes(nvcoop::kNvseDefaultOpcodeBase));
    CHECK(!nvcoop::opcodeBaseSerializes(nvcoop::kNvseOpcodeLimit));            // NVSE asserts opcode < 0x8000
    CHECK(nvcoop::kNvseOpcodeBase == 0x7F10);                                  // saves written by an older build are keyed by it: it must not move
}

TEST(a_spectator_say_does_not_run_the_infos_result_scripts_a_second_time) {
    // Story run S06 (2026-10-28): the engine's own run of INFO 10A1EB's Begin script for each response line the host played with `<npc>.Say` paid the reward again.
    CHECK(isSayLine("Say 0010A1DC") && isSayLine("  say 10A1DC 1") && isSayLine("SAY	F") && isSayLine("Say 1"));
    CHECK(!isSayLine("SayTo player 10A1DC") && !isSayLine("Say") && !isSayLine("") && !isSayLine(nullptr) && !isSayLine("player.AddItem F 5") && !isSayLine("Sayonara 1"));
    SpectatorSayGate g;
    const uint32_t sunny = 0x104E85, doc = 0x104E84;
    CHECK(!g.blocks(sunny, false, 1000));                                       // nobody spoke: the engine's own dialogue runs its scripts as ever
    g.arm(sunny, 1000);
    CHECK(g.blocks(sunny, false, 1500) && g.blocks(sunny, false, 5000));         // each response line of the voice: skipped
    CHECK(!g.blocks(sunny, true, 1500));                                        // the INFO result runner's window (an acting window is open): the one run that counts
    CHECK(!g.blocks(doc, false, 1500));                                         // another speaker is not Sunny's voice
    CHECK(g.blocks(0, false, 1500) && g.blocks(0x14, false, 1500));             // an unnamed speaker is covered while a voice is armed
    CHECK(!g.blocks(sunny, false, 1000 + SpectatorSayGate::kWindowMs + 1));      // the window closes: Sunny's own later talk is hers again
    g.arm(sunny, 20000);
    g.arm(sunny, 25000);                                                        // a second line re-arms the same slot
    CHECK(g.blocks(sunny, false, 25000 + SpectatorSayGate::kWindowMs - 1) && !g.blocks(sunny, false, 25000 + SpectatorSayGate::kWindowMs + 1));
    g.clear();
    CHECK(!g.blocks(sunny, false, 25001));
    for (uint32_t n = 1; n <= 12; ++n) g.arm(0x1000 + n, 40000);                 // more NPCs than slots: the newest are kept, nothing breaks
    CHECK(g.blocks(0x100C, false, 40001) && g.armedCount() >= 12 && g.blockedCount() > 0);
}

TEST(a_guest_with_no_puppet_is_still_a_guest_in_its_window_and_pays_not_the_host) {
    ActingContext a;
    CHECK(a.playerTargetOf(0x14).onHost && a.slot() == 0);               // nobody acts: the host's own player
    a.push(0x14, 2);                                                      // guest 2's window with the host's player standing in (no puppet)
    const ActingContext::PlayerTarget t = a.playerTargetOf(0x14);
    CHECK(t.player && t.slot == 2 && !t.onHost);
    CHECK(!planPlayerRemoveItem(true, !t.onHost, true, true).runEngine);  // the engine does not take the guest's payment from the host's player
    CHECK(!planPlayerAddItem(true, !t.onHost, true, true, false).runEngine && planPlayerAddItem(true, !t.onHost, true, true, false).record);   // nor give the guest's reward to it
    CHECK(planPlayerAddItem(true, !t.onHost, true, true, true).runEngine);   // a leveled list still goes through the engine (resolved items are read back)
    CHECK(!planGuestReward(!t.onHost, true, true).runEngine && planGuestReward(!t.onHost, true, true).record);   // XP / karma / reputation of a guest: recorded, the host's player is not touched
    CHECK(planGuestReward(false, true, true).runEngine && planGuestReward(true, false, true).runEngine);       // the host's own script, or no capture scope open: the engine as ever
    a.push(0x14);                                                         // a nested window without a slot: the host again
    CHECK(a.playerTargetOf(0x14).onHost && planPlayerRemoveItem(true, !a.playerTargetOf(0x14).onHost, true, true).runEngine && planPlayerAddItem(true, false, true, true, false).runEngine);
    a.pop();
    CHECK(a.playerTargetOf(0x14).slot == 2);
    a.pop();
    CHECK(a.playerTargetOf(0x14).onHost);
    a.registerSlot(0x1234, 3);                                            // a guest with a puppet: found by the ghost table as before
    a.push(0x1234);
    CHECK(a.playerTargetOf(0x14).slot == 3 && !a.playerTargetOf(0x14).onHost);
    a.pop();
}

int main() {
    run("spec_table_matches_the_wire_op_table", spec_table_matches_the_wire_op_table);
    run("story_address_table_is_sane", story_address_table_is_sane);
    run("argument_layout_matches_the_engine_command_table", argument_layout_matches_the_engine_command_table);
    run("capture_turns_extracted_arguments_into_commands", capture_turns_extracted_arguments_into_commands);
    run("commands_become_exact_console_lines", commands_become_exact_console_lines);
    run("only_the_owner_performs_actor_ai_orders", only_the_owner_performs_actor_ai_orders);
    run("activate_is_limited_to_object_types_without_a_menu", activate_is_limited_to_object_types_without_a_menu);
    run("animation_groups_come_from_the_engine_table_and_play_as_names", animation_groups_come_from_the_engine_table_and_play_as_names);
    run("text_arguments_travel_as_plain_words", text_arguments_travel_as_plain_words);
    run("critical_stages_come_from_the_engine_table", critical_stages_come_from_the_engine_table);
    run("skill_names_cover_barter_to_unarmed_only", skill_names_cover_barter_to_unarmed_only);
    run("json_codec_round_trips_ref_commands_and_rejects_bad_ones", json_codec_round_trips_ref_commands_and_rejects_bad_ones);
    run("capture_and_apply_agree_for_every_op", capture_and_apply_agree_for_every_op);
    run("use_weapon_uses_the_second_command_shape", use_weapon_uses_the_second_command_shape);
    run("parked_moves_keep_the_newest_and_give_up", parked_moves_keep_the_newest_and_give_up);
    run("an_applier_without_its_own_executor_uses_the_process_wide_one", an_applier_without_its_own_executor_uses_the_process_wide_one);
    run("a_said_once_effect_sets_the_flag_byte_of_the_info", a_said_once_effect_sets_the_flag_byte_of_the_info);
    run("the_opcode_base_given_to_nvse_is_one_a_release_nvse_serializes", the_opcode_base_given_to_nvse_is_one_a_release_nvse_serializes);
    run("a_spectator_say_does_not_run_the_infos_result_scripts_a_second_time", a_spectator_say_does_not_run_the_infos_result_scripts_a_second_time);
    run("a_guest_with_no_puppet_is_still_a_guest_in_its_window_and_pays_not_the_host", a_guest_with_no_puppet_is_still_a_guest_in_its_window_and_pays_not_the_host);
    std::printf("%d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
