// Party saves, engine half (docs/party-save.md): wires party_save.h's HostParty / GuestParty to the session (Runtime), the engine's direct save (the same
// calls as plugin/main.cpp's DirectSaveSEH, which the split loader's core cannot reach), the save folder, the sidecar and index files, and the HUD.
// coop_module.cpp forwards: the frame, the NVSE SaveGame / PostLoadGame messages, and the party orders. Nothing here has run in a game.
#pragma once
#include <cstdint>
#include <string>

#include "common/minijson.h"
#include "coop_runtime.h"
#include "rebalance/campaign_settings.h"

namespace nvcoop::coop::partyglue {

// Each frame, after the session ticked. Null runtime = the session is gone: the machines are dropped.
void frame(Runtime* rt);
// NVSE kMessage_SaveGame with the raw name the message carries. Host: orders every guest to save; both: writes <save>.fos.fnvparty.
void onSaved(Runtime* rt, const std::string& rawName);
// NVSE kMessage_PostLoadGame, with the path PreLoadGame announced: the loaded save's campaign (none: forgotten, a new one is made at the next save).
void onLoaded(Runtime* rt, const std::string& loadedPath);
// The host's party starts from the start save: a new campaign (a campaign loaded from a save is kept).
void startCampaign(Runtime* rt);
// The campaign's rebalance setting (docs/rebalance.md): the host's own, or what the host told this guest; not chosen until there is a campaign that has one. The party
// size is the host plus the joined guests (the host's count, told to guests with the campaign order; 0 = not known yet). isHost: this machine holds the party.
nvcoop::rebalance::CampaignSettings campaignSettings();
int partySize();
bool isHost();
// Host: what the next campaign this host STARTS will play (it is locked into the campaign when it is minted and kept in the save sidecar from then on).
void setNewCampaignSettings(const nvcoop::rebalance::CampaignSettings& s);
// Orders. `applyOrder` is the guest's (campaign, partySave, partySaveWait), `handleOrder` the host's (partySaveRequest, partySaved).
bool isGuestOp(const std::string& op);
bool isHostOp(const std::string& op);
std::string applyOrder(Runtime* rt, const nvcoop::json::Value& order);
std::string handleOrder(Runtime* rt, fnvcoop::PlayerId from, const nvcoop::json::Value& order);

}  // namespace nvcoop::coop::partyglue
