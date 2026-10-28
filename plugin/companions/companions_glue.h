// What the other lanes call (only when built with NVCOOP_WITH_COMPANIONS). Everything behind it is tested game-free except the engine side
// (companions_module.cpp: the real ISocialWorld / ISocialEngine over the xNVSE layouts and console lines), which has never run in a game.
//
// Who calls what (all of it compiled only with NVCOOP_WITH_COMPANIONS, and a no-op / "allow" until [companions] enabled=1 and a session runs):
//   frame       plugin/coop/coop_module.cpp: glueFrame(g_rt) once per frame after livewire::glueFrame() and glueFrame(nullptr) at shutdown. THE ONE call
//               site: plugin/coop/live/live_glue.cpp's SocialService (`[live] social=1`, needs actors=1, default off) does not tick it again; it only
//               gives this module the avatar layer's puppet map (setPlayerRefMap) and a line in the live status.
//   intents     plugin/intents (HostDeps::allowActivation / interacted, filled in engine/intents_module.cpp) and, when the intents module is off,
//               coop_module.cpp's v1 action executor: before a guest's Activate / DialogueStart / ActivateRef runs, glueAllowActivation(slot, target)
//               (false = refused, Result::Denied); after a guest's Activate / PickUp / ContainerOpen ran, glueGuestInteraction(slot, pickup, target).
//   combat      plugin/combat (Config::social, filled in combat_module.cpp): the death path calls glueReportKill(...) from the machine that declared the
//               death and skips combat's own awardXp() while glueXpHandled(); the attack path (engine hit gate, forwarded hits, blast planning) and the
//               killing blow call glueReportCrime(...); the hit gate answers GateAction::Neutralize when glueHitPolicy(attackerRef, victimRef) is
//               sc::HitPolicy::Neutralize.
//   vats        (not wired by this task) before offering a target:  if (!nvcoop::social::glueVatsTargetAllowed(targetRef)) skip it.
// docs/tools/companions-crime.md lists the same with the reasons.
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include "companions/social_sync.h"

namespace nvcoop::coop { class Runtime; }

namespace nvcoop::social {

void glueFrame(nvcoop::coop::Runtime* rt);      // once per frame after Runtime::frame(); null = the session is gone
bool glueRunning();                             // a SocialSync exists (the coop session is up and [companions] enabled=1)
bool glueEnabled();                             // nvcoop.ini [companions] enabled=1
std::string glueStatusJson();                   // social.status

// ---- questions (always "allow" when the system is off or no session runs) ----------------------------------------------------------------
bool glueAllowActivation(uint8_t slot, const fnvcoop::FormRef& npc);      // false = suppress: somebody else's companion
sc::HitPolicy glueHitPolicy(uint32_t attackerRef, uint32_t victimRef);    // Neutralize inside the party
bool glueVatsTargetAllowed(uint32_t targetRef);
bool glueXpHandled();                           // true: this module pays kill XP / karma, the combat glue must not

// ---- reports -------------------------------------------------------------------------------------------------------------------------------
void glueReportCrime(const sc::CrimeEvent& e);
void glueReportKill(const sc::KillReport& r);
void glueGuestInteraction(uint8_t slot, bool pickup, const fnvcoop::FormRef& target);   // host only; a guest's intent just ran

// ---- G34 companion set pieces (called by the session wiring in plugin/coop; the rules themselves are tested game-free) ------------------------------------------
// (The rules engine's owner=party test needs no call: the host's HostSession::isPartyCompanion, installed by the RulesEngine, is read through the runtime's host() every frame.)
// Which player's hired flags a quest / object script should read (the viewSlotFor policy): the acting player for a hire / fire, sc::kViewParty (0xFE) for a mover or gate
// (Fort entry and exit, the Dam start, the ending): "use the shared variable as it is", it already holds the party aggregate (CompanionRegistry::aggregateVar).
// Called with the whole companion roster (npc, owner, kind, party flag) whenever it changed on this machine: feed the dialogue registry from it.
void glueSetRosterListener(std::function<void(const std::vector<sc::Companion>&)> onRoster);
// The companion a player has in a single-slot group ("deadmoney", "honesthearts"): what NVDLC01Followers.rCurrentFollower should read as for that player. false = none.
bool glueCurrentFollower(uint8_t slot, const std::string& group, fnvcoop::FormRef& out);
uint8_t glueViewSlot(const std::string& scriptEdid, uint8_t actingSlot);
// The EDIDs (lower case) of the scripts that read the hired flags party-wide (companion_sets.txt view-party): the session resolves them to form ids to name a running script.
std::vector<std::string> glueViewPartyScripts();

// The avatar layer's puppet map (optional; default is the NV:MP ghost map, like plugin/combat).
void setPlayerRefMap(std::function<bool(uint32_t ref, uint8_t& slot)> slotOfRef, std::function<uint32_t(uint8_t slot)> refOfSlot);

}  // namespace nvcoop::social
