// The live layer inside nvcoop.dll (dll-wiring). What main.cpp and coop_module.cpp call. No game header in here.
//
//   glueConfigure(ini)   NVSEPlugin_Load, BEFORE any module's onLoad: reads [live] (switches, all OFF by default) and [actors] (tuning of the real
//                        actor port) and hands the result to the lanes (actors::engine::configure ...), so a lane that installs hooks at
//                        load time already sees whether it is switched on.
//   glueLoad()           coop module onLoad: builds the hub and the enabled services, registers the bridge method coop.live.
//   glueFrame(rt, now)   every frame, AFTER Runtime::frame() (which polls the mux): session lifecycle, channel routing, ticks.
//   glueMessage(...)     NVSE messages: PreLoadGame / PostLoadGame / SaveGame / ExitToMainMenu / ExitGame.
//   glueShutdown()       hot reload / unload: ends the session part of every service.
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include <vector>

#include "fnvcoop/effect.h"
#include "live_switches.h"
#include "../slot_facts.h"

namespace nvcoop::coop { class Runtime; }
namespace fnvcoop { struct SpawnRef; }

namespace nvcoop::livewire {

void glueConfigure(const std::string& iniPath);
void glueLoad();
void glueFrame(nvcoop::coop::Runtime* rt, uint64_t nowMs);
void glueMessage(uint32_t nvseMessageType, void* data, uint32_t len);
void glueShutdown();

const LiveSwitches& glueSwitches();
// Engine ref of the puppet that stands for a remote player slot (the ghost-ref map of the acting-player design); 0 = none / avatars off.
uint32_t glueAvatarRef(uint8_t slot);
// This machine's own character creation reached a point where the look is final (a face menu closed, the last screen was left): the avatar layer reads the look from
// the game at once and sends it without its settle time. No-op without avatars.
void glueLookEdited();
// W3a item 3 (host): what the avatar layer knows about the remote player `slot` for the party conditions (sex and worn items from its Look, sneaking from its newest
// snapshot, level / seat / actions / regions from its Facts message). false = no avatars service or the player is unknown; each group has its own have* flag.
bool glueSlotFacts(uint8_t slot, nvcoop::SlotFacts* out);
// Guest pack facts (host): what a guest carries, from its Pack message (plugin/coop/live/avatar_net.h). gluePackCount: a watched item form (runtime form id); false = unknown
// (no pack yet, or a form that is not on the list both machines share), the puppet's own pack stays the answer. gluePackWeight: the InventoryWeight actor value.
bool gluePackCount(uint8_t slot, uint32_t item, int64_t* count);
bool gluePackWeight(uint8_t slot, double* weight);
// The watched item forms as runtime form ids (nvcoop_pack_watch.json resolved against the loaded game); asked again at every session start and load. Both roles set it.
void glueSetPackWatch(std::function<std::vector<uint32_t>()> watch);
// The shared ini getter (environment NVCOOP_<SECTION>_<KEY> first, then the ini file); usable by every lane glue.
IniGetter glueIni();

// Story effects as this machine applies them (coop_module's EngineWorld::apply, BEFORE the applier runs): KillActor / ResurrectActor /
// CompanionOwner reach the ownership service. No-op without a session.
void glueOnEffect(const fnvcoop::Effect& e);
// A save the plugin itself triggers (the bridge's saveGame): before it, stand-in / leveled-pick clones and player puppets are taken out of the world so
// the .fos does not contain them; after it, puppets may respawn. (A save the player starts has no known pre-save hook; the load-time purge covers it.)
void glueBeforeSave();
void glueAfterSave();

// plugin/spawn: coop_module hands over the WorldApplier (makes this machine's own copy of a spawn it did not roll).
void glueSetSpawnApplier(std::function<bool(const fnvcoop::SpawnRef&)> f);
// The host's spawn-site table travels with its save (<save>.fnvspawn); empty when spawns are off or this is a client.
std::vector<uint8_t> glueSpawnSidecar();
void glueSpawnSidecarLoaded(const std::vector<uint8_t>& bytes);

}  // namespace nvcoop::livewire
