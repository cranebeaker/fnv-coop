// The one x86 call behind EquipmentAccess (equipment.h): a guarded read of the worn entries of an actor reference (equipment_real.cpp).
#pragma once
#include <vector>

#include "actors/engine/equipment.h"

namespace nvcoop::actors::engine {

// false = the reference is not a loaded actor or its extra data cannot be read; an actor that wears nothing gives true and an empty list.
bool readWornEntries(RefId ref, std::vector<WornEntry>& out);

}  // namespace nvcoop::actors::engine
