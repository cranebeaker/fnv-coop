// What the coop module needs from the intents module to keep the gear ledger with the save game (P4 handover 8). Defined in intents_module.cpp.
#pragma once
#include <string>

namespace nvcoop::intents {
// The gear ledger as text; empty while [intents] is off (nothing to store).
std::string LedgerSnapshot();
// A save was loaded: put back the ledger that was written with it. "" = that save has none (made before the ledger existed): the ledger is emptied.
void LedgerRestore(const std::string& text);
}  // namespace nvcoop::intents
