// How often each player's avatar stream goes to each other player (FULL-SYNC-PLAN section 2: 20 Hz near, 5 Hz far; the shipped near rate is 30 Hz,
// engine/avatar_tuning.h sets nearPeriodMs and nvcoop.ini [avatar] nearHz overrides it). The planner itself lives in synccore/live/stream_rate.h since the
// actor module's player stream profile (docs/entity-core-plan.md S10) plans with the same code; these are the old names.
#pragma once
#include "live/stream_rate.h"

namespace nvcoop::avatar {

using StreamRateConfig = fnvcoop::live::StreamRateConfig;
using StreamTier = fnvcoop::live::StreamTier;
using StreamRatePlanner = fnvcoop::live::StreamRatePlanner;

}  // namespace nvcoop::avatar
