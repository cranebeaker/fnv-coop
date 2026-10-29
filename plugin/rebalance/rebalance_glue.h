// What other modules read from plugin/rebalance (docs/rebalance.md). Only linked into nvcoop.dll (NVCOOP_WITH_REBALANCE); all three answer "nothing" before the first frame.
#pragma once
#include "rebalance/campaign_settings.h"

namespace nvcoop::rebalance {

bool glueBalancedOn();                 // this campaign plays Co-op balanced and the module is on (the threat AI of feat/threat switches itself on with this)
CampaignSettings glueSettings();       // the campaign's setting as this machine has it (host: its own; guest: what the host said)
int gluePartySize();                   // the party size the numbers are scaled for (0 = not known yet)

}  // namespace nvcoop::rebalance
