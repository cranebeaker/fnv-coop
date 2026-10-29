#include "ref_ops.h"

#include <cctype>
#include <cstdio>
#include <cstring>

#include "engine_addrs.h"
#include "fnvcoop_mut.h"
#include "story_addrs.h"
#include "story_animgroups.h"

namespace nvhooks {
namespace {
constexpr uint8_t AI = kRoAiOrder, REF = kRoNeedsRef, PLY = kRoNeedsRef | kRoPlayerRef, SHARED = kRoSharedHook;
#define S_(h) (&story_addr::h)
#define E_(h) (&addr::h)
}  // namespace

// Order = fnvcoop::RefOp ids (checked by test_story.cpp). sig / slots / minArgs follow the ENGINE parameter list of each command, taken from the
// command table of a reference exe (data/story_cmd_params.json); a mismatch fails the unit tests.
const RefOpSpec kStorySpecs[] = {
    // ---- actor AI orders: performed by the machine that owns the actor
    {fnvcoop::kOpEvaluatePackage, "evaluatepackage", S_(Cmd_EvaluatePackage), "", "", 0, REF | AI},
    {fnvcoop::kOpStartCombat, "startcombat", E_(Cmd_StartCombat), "o", "a", 1, REF | AI | SHARED},
    {fnvcoop::kOpStopCombat, "stopcombat", E_(Cmd_StopCombat), "", "", 0, REF | AI | SHARED},
    {fnvcoop::kOpResetAI, "resetai", S_(Cmd_ResetAI), "", "", 0, REF | AI},
    {fnvcoop::kOpStopLook, "stoplook", S_(Cmd_StopLook), "", "", 0, REF | AI},
    {fnvcoop::kOpLook, "look", S_(Cmd_Look), "oi", "ai", 1, REF | AI},
    {fnvcoop::kOpSetAlert, "setalert", S_(Cmd_SetAlert), "i", "i", 1, REF | AI},
    {fnvcoop::kOpStopCombatAlarmOnActor, "stopcombatalarmonactor", S_(Cmd_StopCombatAlarmOnActor), "", "", 0, REF | AI},
    {fnvcoop::kOpForceFlee, "forceflee", S_(Cmd_ForceFlee), "oo", "ab", 0, REF | AI},
    {fnvcoop::kOpPushActorAway, "pushactoraway", S_(Cmd_PushActorAway), "oi", "ai", 2, REF | AI},
    {fnvcoop::kOpResetHealth, "resethealth", S_(Cmd_ResetHealth), "", "", 0, REF | AI},
    // ---- actor flags (state: every machine keeps them)
    {fnvcoop::kOpSetUnconscious, "setunconscious", S_(Cmd_SetUnconscious), "i", "i", 1, REF},
    {fnvcoop::kOpSetGhost, "setghost", S_(Cmd_SetGhost), "i", "i", 1, REF},
    {fnvcoop::kOpSetIgnoreFriendlyHits, "setignorefriendlyhits", S_(Cmd_SetIgnoreFriendlyHits), "i", "i", 1, REF},
    {fnvcoop::kOpSetCombatStyle, "setcombatstyle", S_(Cmd_SetCombatStyle), "o", "a", 0, REF},
    {fnvcoop::kOpSetForceSneak, "setforcesneak", S_(Cmd_SetForceSneak), "i", "i", 1, REF},
    {fnvcoop::kOpSetPackDuration, "setpackduration", S_(Cmd_SetPackDuration), "f", "f", 1, REF | AI},
    {fnvcoop::kOpSetActorsAI, "setactorsai", S_(Cmd_SetActorsAI), "i", "i", 1, REF},
    {fnvcoop::kOpPickIdle, "pickidle", S_(Cmd_PickIdle), "", "", 0, REF | AI},
    {fnvcoop::kOpSetVATSTarget, "setvatstarget", S_(Cmd_SetVATSTarget), "i", "i", 1, REF | AI},
    {fnvcoop::kOpFireWeapon, "fireweapon", S_(Cmd_FireWeapon), "o", "a", 1, REF | AI},
    {fnvcoop::kOpSetDisposition, "setdisposition", S_(Cmd_SetDisposition), "oi", "ai", 2, REF},
    // ---- spells
    {fnvcoop::kOpAddSpell, "addspell", S_(Cmd_AddSpell), "o", "a", 1, REF},
    {fnvcoop::kOpRemoveSpell, "removespell", S_(Cmd_RemoveSpell), "o", "a", 1, REF},
    {fnvcoop::kOpDispel, "dispel", S_(Cmd_Dispel), "o", "a", 1, REF},
    {fnvcoop::kOpDispelAllSpells, "dispelallspells", S_(Cmd_DispelAllSpells), "", "", 0, REF},
    {fnvcoop::kOpCastImmediateOnSelf, "castimmediateonself", S_(Cmd_CastImmediateOnSelf), "o", "a", 1, REF},
    {fnvcoop::kOpPlayMagicShaderVisuals, "playmagicshadervisuals", S_(Cmd_PlayMagicShaderVisuals), "of", "af", 1, REF},
    {fnvcoop::kOpStopMagicShaderVisuals, "stopmagicshadervisuals", S_(Cmd_StopMagicShaderVisuals), "o", "a", 1, REF},
    // ---- equipment and inventory
    {fnvcoop::kOpEquipItem, "equipitem", S_(Cmd_EquipItem), "oii", "aij", 1, REF},
    {fnvcoop::kOpUnequipItem, "unequipitem", S_(Cmd_UnequipItem), "oii", "aij", 1, REF},
    {fnvcoop::kOpResetInventory, "resetinventory", S_(Cmd_ResetInventory), "", "", 0, REF},
    {fnvcoop::kOpDropMe, "dropme", S_(Cmd_DropMe), "", "", 0, REF},
    {fnvcoop::kOpDrop, "drop", S_(Cmd_Drop), "oi", "ai", 2, REF},
    {fnvcoop::kOpRemoveMe, "removeme", S_(Cmd_RemoveMe), "o", "a", 0, REF},
    {fnvcoop::kOpAddItemToLeveledList, "additemtoleveledlist", S_(Cmd_AddItemToLeveledList), "ooiif", "abijf", 5, 0},
    {fnvcoop::kOpAddFormToFormList, "addformtoformlist", S_(Cmd_AddFormToFormList), "oo", "ab", 2, 0},
    // ---- world state (the ownership commands SetOwnership / ClearOwnership / SetCellOwnership / SetCellPublicFlag belong to the world-time lane:
    //      plugin/hooks_world/world_owner_addrs.h hooks them, one detour per address)
    {fnvcoop::kOpSetActorAlpha, "setactoralpha", S_(Cmd_SetActorAlpha), "f", "f", 1, REF},
    {fnvcoop::kOpSetScale, "setscale", S_(Cmd_SetScale), "f", "f", 1, REF},
    {fnvcoop::kOpSetItemValue, "setitemvalue", S_(Cmd_SetItemValue), "i", "i", 1, REF},
    {fnvcoop::kOpClearDestruction, "cleardestruction", S_(Cmd_ClearDestruction), "", "", 0, REF},
    {fnvcoop::kOpRotate, "rotate", S_(Cmd_Rotate), "xi", "ij", 2, REF},
    // ---- quest meta
    {fnvcoop::kOpSetQuestDelay, "setquestdelay", S_(Cmd_SetQuestDelay), "of", "af", 2, 0},
    {fnvcoop::kOpResetQuest, "resetquest", S_(Cmd_ResetQuest), "o", "a", 1, 0},
    {fnvcoop::kOpSetQuestObject, "setquestobject", S_(Cmd_SetQuestObject), "oi", "ai", 2, 0},
    {fnvcoop::kOpAutoDisplayObjectives, "autodisplayobjectives", S_(Cmd_AutoDisplayObjectives), "i", "i", 1, 0},
    {fnvcoop::kOpActivate, "activate", E_(Cmd_Activate), "oi", "ai", 0, REF | SHARED | kRoActivate},
    // ---- character build (the acting player)
    {fnvcoop::kOpSetPlayerTagSkill, "setplayertagskill", S_(Cmd_SetPlayerTagSkill), "vi", "ij", 2, 0},
    {fnvcoop::kOpAddSPECIALPoints, "addspecialpoints", S_(Cmd_AddSPECIALPoints), "i", "i", 1, 0},
    {fnvcoop::kOpSetSPECIALPoints, "setspecialpoints", S_(Cmd_SetSPECIALPoints), "i", "i", 1, 0},
    {fnvcoop::kOpSetTagSkills, "settagskills", S_(Cmd_SetTagSkills), "ii", "ij", 1, 0},
    {fnvcoop::kOpAddTagSkills, "addtagskills", S_(Cmd_AddTagSkills), "i", "i", 1, 0},
    {fnvcoop::kOpSetLevel, "setlevel", S_(Cmd_SetLevel), "iiii", "ijkl", 1, PLY},
    {fnvcoop::kOpResetXP, "resetxp", S_(Cmd_ResetXP), "", "", 0, PLY},
    {fnvcoop::kOpAgeRace, "agerace", S_(Cmd_AgeRace), "i", "i", 1, PLY},
    {fnvcoop::kOpSexChange, "sexchange", S_(Cmd_SexChange), "ii", "ij", 0, 0},
    {fnvcoop::kOpSetPCCanUsePowerArmor, "setpccanusepowerarmor", S_(Cmd_SetPCCanUsePowerArmor), "i", "i", 1, 0},
    {fnvcoop::kOpSetPCSleepHours, "setpcsleephours", S_(Cmd_SetPCSleepHours), "i", "i", 1, 0},
    {fnvcoop::kOpWakeUpPC, "wakeuppc", S_(Cmd_WakeUpPC), "i", "i", 0, 0},
    {fnvcoop::kOpSetHardcore, "sethardcore", S_(Cmd_SetHardcore), "i", "i", 1, PLY},
    {fnvcoop::kOpSetInChargen, "setinchargen", S_(Cmd_SetInChargen), "i", "i", 1, 0},
    {fnvcoop::kOpOpenTeammateContainer, "openteammatecontainer", S_(Cmd_OpenTeammateContainer), "i", "i", 0, REF},
    // ---- challenges and achievements (everyone)
    {fnvcoop::kOpAddAchievement, "addachievement", S_(Cmd_AddAchievement), "i", "i", 1, 0},
    {fnvcoop::kOpIncrementScriptedChallenge, "incrementscriptedchallenge", S_(Cmd_IncrementScriptedChallenge), "o", "a", 1, 0},
    {fnvcoop::kOpUnlockChallenge, "unlockchallenge", S_(Cmd_UnlockChallenge), "o", "a", 1, 0},
    {fnvcoop::kOpRemoveRecurringFromChallenge, "removerecurringfromchallenge", S_(Cmd_RemoveRecurringFromChallenge), "o", "a", 1, 0},
    {fnvcoop::kOpAddCardToPlayer, "addcardtoplayer", S_(Cmd_AddCardToPlayer), "", "", 0, PLY},
    // ---- scene helpers
    {fnvcoop::kOpSetTalkingActivatorActor, "settalkingactivatoractor", S_(Cmd_SetTalkingActivatorActor), "o", "a", 0, REF},
    {fnvcoop::kOpAlwaysShowActorSubtitles, "alwaysshowactorsubtitles", S_(Cmd_AlwaysShowActorSubtitles), "i", "i", 1, 0},
    {fnvcoop::kOpStartRadioConversation, "startradioconversation", S_(Cmd_StartRadioConversation), "o", "a", 0, REF},
    {fnvcoop::kOpSetCellImageSpace, "setcellimagespace", S_(Cmd_SetCellImageSpace), "oo", "ab", 2, 0},
    // ---- item with a condition: the player's reward or a container's stock
    {fnvcoop::kOpAddItemHealthPercent, "additemhealthpercent", S_(Cmd_AddItemHealthPercent), "oifi", "aifj", 3, REF},
    // ---- animation and faction deltas
    {fnvcoop::kOpPlayGroup, "playgroup", S_(Cmd_PlayGroup), "gi", "ij", 2, REF | AI},
    {fnvcoop::kOpModFactionRank, "modfactionrank", E_(Cmd_ModFactionRank), "oi", "ai", 2, REF | SHARED},
    {fnvcoop::kOpPlayIdle, "playidle", S_(Cmd_PlayIdle), "s", "s", 1, REF | AI},
    {fnvcoop::kOpSetCriticalStage, "setcriticalstage", S_(Cmd_SetCriticalStage), "c", "i", 1, REF},
    {fnvcoop::kOpSetActorFullName, "setactorfullname", S_(Cmd_SetActorFullName), "o", "a", 1, REF},   // the name is a Message FORM (param type 0x31), not a text
    // ---- the second shape: nine parameters. weapon, two object refs, an int, four optional ints, an optional object ref (engine param types 21 4 4 1 1 1 1 1 4)
    {fnvcoop::kOpUseWeapon, "useweapon", S_(Cmd_UseWeapon), "oooiiiiio", "abcijklmd", 4, REF | AI},
};
const size_t kStorySpecCount = sizeof(kStorySpecs) / sizeof(kStorySpecs[0]);

const RefOpSpec* storySpecFor(uint16_t op) {
    // ids are dense and in order (checked by the unit tests): op N is row N - 1
    if (op >= 1 && op - 1u < kStorySpecCount && kStorySpecs[op - 1].op == op) return &kStorySpecs[op - 1];
    for (size_t i = 0; i < kStorySpecCount; ++i)
        if (kStorySpecs[i].op == op) return &kStorySpecs[i];
    return nullptr;
}

const RefOpSpec* storySpecForFn(const std::string& fn) {
    for (size_t i = 0; i < kStorySpecCount; ++i)
        if (fn == kStorySpecs[i].fn) return &kStorySpecs[i];
    return nullptr;
}

namespace {
// A text argument is put on a console line as one token: letters, digits and a few punctuation marks of EditorIDs / idle names, at most 255.
bool plainWord(const std::string& s) {
    if (FNV_MUT(11)) return true;   // twin 11: any text goes onto the console line (a script could append a second command)
    if (s.size() > 255) return false;
    for (unsigned char ch : s)
        if (!(std::isalnum(ch) || ch == '_' || ch == '-' || ch == '.' || ch == ':')) return false;
    return true;
}
int slotIndex(char c) {   // i j k l m -> 0..4
    switch (c) {
        case 'i': return 0;
        case 'j': return 1;
        case 'k': return 2;
        case 'l': return 3;
        case 'm': return 4;
        default: return -1;
    }
}
int32_t& intSlot(fnvcoop::RefCommand& c, char s) {
    switch (s) {
        case 'j': return c.j;
        case 'k': return c.k;
        case 'l': return c.l;
        case 'm': return c.m;
        default: return c.i;
    }
}
int32_t intOf(const fnvcoop::RefCommand& c, char s) {
    switch (s) {
        case 'j': return c.j;
        case 'k': return c.k;
        case 'l': return c.l;
        case 'm': return c.m;
        default: return c.i;
    }
}
fnvcoop::FormRef& formSlot(fnvcoop::RefCommand& c, char s) {
    switch (s) {
        case 'b': return c.b;
        case 'c': return c.c;
        case 'd': return c.d;
        default: return c.a;
    }
}
const fnvcoop::FormRef& formOf(const fnvcoop::RefCommand& c, char s) {
    switch (s) {
        case 'b': return c.b;
        case 'c': return c.c;
        case 'd': return c.d;
        default: return c.a;
    }
}
}  // namespace

bool buildRefCommand(const RefOpSpec& s, uint32_t callingRef, const ArgValue* args, const RefToForm& toForm, fnvcoop::RefCommand* out,
                     std::string* why) {
    auto fail = [&](const char* m) { if (why) *why = m; return false; };
    fnvcoop::RefCommand c;
    c.op = s.op;
    if (callingRef) {
        if (!toForm(callingRef, &c.ref)) return fail("calling reference has no stable name");
    } else if ((s.flags & kRoNeedsRef) && !(s.flags & kRoPlayerRef)) {
        return fail("command needs a calling reference and has none");
    }
    const size_t n = std::strlen(s.sig);
    for (size_t k = 0; k < n; ++k) {
        const char t = s.sig[k], d = s.slots[k];
        const ArgValue& a = args[k];
        if (t == 'o') {
            fnvcoop::FormRef& dst = formSlot(c, d);
            if (a.form && !toForm(a.form, &dst)) return fail("form argument has no stable name");
            if (!a.form && k < s.minArgs) return fail("required form argument is missing");
        } else if (t == 's') {
            if (a.s.empty() && k < s.minArgs) return fail("required text argument is missing");
            if (!plainWord(a.s)) return fail("text argument is not a plain word");
            c.s = a.s;
        } else if (t == 'c') {
            if (a.i < 0 || a.i >= kCriticalStageCount) return fail("critical stage out of range");
            intSlot(c, d) = a.i;
        } else if (t == 'g') {
            if (a.i < 0 || a.i >= kAnimGroupCount) return fail("animation group out of range");
            intSlot(c, d) = a.i;
        } else if (t == 'i' || t == 'v') {
            intSlot(c, d) = a.i;
        } else if (t == 'x') {
            int idx = a.i;
            if (idx != 'X' && idx != 'Y' && idx != 'Z' && idx != 'x' && idx != 'y' && idx != 'z') return fail("bad axis");
            intSlot(c, d) = (idx == 'X' || idx == 'x') ? 0 : (idx == 'Y' || idx == 'y') ? 1 : 2;
        } else if (t == 'f') {
            c.f = a.f;
        }
    }
    if (FNV_MUT(12)) { c.c = fnvcoop::FormRef(); c.d = fnvcoop::FormRef(); c.m = 0; }   // twin 12: the second shape is dropped (UseWeapon would shoot at the wrong target)
    if (FNV_MUT(1)) c.ref = fnvcoop::FormRef();   // twin 1: the calling reference is dropped (every command would hit "nothing")
    const fnvcoop::RefOpInfo* info = fnvcoop::refOpInfo(s.op);
    if (!info) return fail("unknown op");
    if (FNV_MUT(5)) c.to = fnvcoop::Recipient::dflt();   // twin 5: no op addresses the acting player (a chargen call would reach every player)
    else c.to = info->recipient == fnvcoop::kRcpOrigin ? fnvcoop::Recipient::origin()
         : info->recipient == fnvcoop::kRcpAll    ? fnvcoop::Recipient::all()
                                                  : fnvcoop::Recipient::dflt();
    *out = std::move(c);
    return true;
}

std::string forceActiveQuestCmd(uint32_t q) {
    char b[48];
    std::snprintf(b, sizeof b, "ForceActiveQuest %08X", q);
    return b;
}

const char* skillNameOf(int32_t av) {
    static const char* const kSkills[] = {"Barter", "BigGuns", "EnergyWeapons", "Explosives", "Lockpick", "Medicine", "MeleeWeapons",
                                          "Repair", "Science", "Guns", "Sneak", "Speech", "Survival", "Unarmed"};   // eActorVal_Barter (32) .. Unarmed (45)
    if (FNV_MUT(6)) av += 1;   // twin 6: off by one
    return av >= 32 && av <= 45 ? kSkills[av - 32] : nullptr;
}

LineResult buildRefCommandLine(const fnvcoop::RefCommand& c, const FormToRef& fromForm) {
    LineResult r;
    const RefOpSpec* s = storySpecFor(c.op);
    const fnvcoop::RefOpInfo* info = fnvcoop::refOpInfo(c.op);
    if (!s || !info) { r.why = "unknown RefCommand op"; return r; }
    if (!c.ref.plugin.empty()) {
        r.callingRef = fromForm(c.ref);
        if (!r.callingRef) { r.formMissing = true; r.why = "calling reference not loaded"; return r; }
    } else if (s->flags & kRoPlayerRef) {
        r.callingRef = 0x14;
    } else if (s->flags & kRoNeedsRef) {
        r.why = "command needs a calling reference";
        return r;
    }
    std::string line = info->name;
    size_t n = std::strlen(s->sig);
    // Trailing optional parameters that hold their default (no form, 0, 0.0) are left out: the script did not give them, and the engine's own
    // default for an omitted parameter is what the original call used.
    auto isDefault = [&](size_t k) {
        const char t = s->sig[k], d = s->slots[k];
        if (t == 'o') return formOf(c, d).plugin.empty();
        if (t == 's') return c.s.empty();
        if (t == 'f') return c.f == 0.f;
        return intOf(c, d) == 0;
    };
    if (!FNV_MUT(4))   // twin 4: every optional parameter is printed, defaults included
        while (n > s->minArgs && isDefault(n - 1)) --n;
    for (size_t k = 0; k < n; ++k) {
        const char t = s->sig[k], d = s->slots[k];
        char buf[48];
        if (t == 'o') {
            const fnvcoop::FormRef& f = formOf(c, d);
            if (f.plugin.empty()) { r.why = k < s->minArgs ? "required form argument is missing" : "an optional form is missing before a later argument"; return r; }
            uint32_t id = fromForm(f);
            if (!id) { r.formMissing = true; r.why = "form not loaded"; return r; }
            std::snprintf(buf, sizeof buf, " %08X", id);
        } else if (t == 'i') {
            std::snprintf(buf, sizeof buf, " %d", intOf(c, d));
        } else if (t == 's') {
            if (c.s.empty() || !plainWord(c.s)) { r.why = "text argument is missing or not a plain word"; return r; }
            line += " " + c.s;
            continue;
        } else if (t == 'c') {
            int st = intOf(c, d);
            if (st < 0 || st >= kCriticalStageCount) { r.why = "critical stage out of range"; return r; }
            std::snprintf(buf, sizeof buf, " %s", kCriticalStageNames[st]);
        } else if (t == 'g') {
            int g = intOf(c, d);
            if (g < 0 || g >= kAnimGroupCount) { r.why = "animation group out of range"; return r; }
            std::snprintf(buf, sizeof buf, " %s", kAnimGroupNames[g]);
        } else if (t == 'v') {
            const char* nm = skillNameOf(intOf(c, d));
            if (!nm) { r.why = "actor value has no console name"; return r; }
            std::snprintf(buf, sizeof buf, " %s", nm);
        } else if (t == 'x') {
            int ax = intOf(c, d);
            if (ax < 0 || ax > 2) { r.why = "bad axis"; return r; }
            std::snprintf(buf, sizeof buf, " %c", "XYZ"[ax]);
        } else {
            std::snprintf(buf, sizeof buf, " %.4f", c.f);
        }
        line += buf;
    }
    r.line = std::move(line);
    r.ok = true;
    return r;
}

bool opPerformedHere(const RefOpSpec& s, uint32_t callingRef, const std::function<bool(uint32_t)>& remoteDriven) {
    if (!(s.flags & kRoAiOrder)) return true;
    if (FNV_MUT(2)) return true;                        // twin 2: every machine performs AI orders (two machines would steer one actor)
    if (!remoteDriven || !callingRef) return true;      // no ownership layer, or no reference to ask about
    return !remoteDriven(callingRef);
}

bool refOpDefaultsToPlayer(uint16_t op) {
    const RefOpSpec* s = storySpecFor(op);
    return s && (s->flags & kRoPlayerRef) != 0;
}

bool activateTypeSafe(unsigned t) {
    // Base form types (xNVSE GameForms.h): TESObjectACTI 0x15 and TESObjectDOOR 0x1C are safe to activate from a script on a machine that did not
    // ask for it. BGSTalkingActivator 0x16 starts a conversation, BGSTerminal 0x17 opens the terminal, TESObjectCONT 0x1B opens a container menu,
    // and the base of an NPC / creature reference (0x2A / 0x2B) talks: all of them stay with the machine whose player used them.
    if (FNV_MUT(3)) return true;                        // twin 3: anything may be activated
    return t == 0x15 || t == 0x1C;
}

}  // namespace nvhooks
