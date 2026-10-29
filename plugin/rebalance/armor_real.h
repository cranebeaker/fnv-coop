// The armour class an actor wears now (armor_real.cpp), for Co-op balanced's armour factor. x86, nvcoop.dll only.
#pragma once
#include <cstdint>

#include "rebalance/balance.h"

namespace nvcoop::rebalance {

ArmorClass armorClassOf(uint32_t ref);   // None when nothing is worn, the ref is 0, or it cannot be read

}  // namespace nvcoop::rebalance
