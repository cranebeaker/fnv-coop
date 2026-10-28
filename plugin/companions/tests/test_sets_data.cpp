// The companion set table (plugin/companions/data/companion_sets.txt): the embedded copy matches the file, it parses without a bad line, the groups name the same
// placed forms as the dialogue registry's follower table, and the policy answers for the scripts that matter.
#include <fstream>
#include <sstream>
#include "companions/companion_sets_default.h"
#include "harness.h"
#include "social/slots.h"

using namespace stest;

#ifndef NVSO_DATA_DIR
#define NVSO_DATA_DIR "."
#endif

namespace {
std::string readFile(const std::string& path) {
    std::ifstream in(path, std::ios::binary);
    std::stringstream ss;
    ss << in.rdbuf();
    std::string s = ss.str();
    std::string out;
    for (char c : s) if (c != '\r') out += c;
    return out;
}
}  // namespace

TEST(sets_data, the_embedded_table_is_the_data_file) {
    const std::string file = readFile(std::string(NVSO_DATA_DIR) + "/companion_sets.txt");
    REQUIRE(!file.empty());
    CHECK(file == std::string(kDefaultCompanionSets));   // regenerate with: python -I plugin/companions/proof/gen_sets.py
}

TEST(sets_data, it_parses_clean_and_names_the_two_single_slot_groups) {
    sc::SlotRules r;
    const sc::SlotRules::LoadStats st = r.load(kDefaultCompanionSets);
    CHECK(st.bad == 0 && st.groups == 2 && st.party == 0);
    CHECK(st.viewParty > 150);
    FormRef dean, christine, dog, chalk, wakingCloud;
    CHECK(sc::parseSetRef("DeadMoney.esm:0012FE", dean) && sc::parseSetRef("DeadMoney.esm:0012FF", christine) && sc::parseSetRef("DeadMoney.esm:001301", dog));
    CHECK(sc::parseSetRef("HonestHearts.esm:0094D0", chalk) && sc::parseSetRef("HonestHearts.esm:009665", wakingCloud));
    const sc::SlotGroup* dm = r.groupOf(dean);
    REQUIRE(dm != nullptr);
    CHECK(dm->name == "deadmoney" && r.groupOf(christine) == dm && r.groupOf(dog) == dm);
    const sc::SlotGroup* hh = r.groupOf(chalk);
    REQUIRE(hh != nullptr);
    CHECK(hh->name == "honesthearts" && r.groupOf(wakingCloud) == hh && hh != dm);
    FormRef boone;
    CHECK(sc::parseSetRef("FalloutNV.esm:096BCE", boone));
    CHECK(r.groupOf(boone) == nullptr);                  // the Mojave companions are the one-humanoid-one-critter rule, not a group
}

TEST(sets_data, the_groups_use_the_dialogue_registrys_forms) {
    // every ref of a group appears on the matching companion line of plugin/dialogue/data/follower_rules.txt
    const std::string follower = readFile(std::string(NVSO_DATA_DIR) + "/../../dialogue/data/follower_rules.txt");
    REQUIRE(!follower.empty());
    sc::SlotRules r;
    r.load(kDefaultCompanionSets);
    size_t checked = 0;
    for (const sc::SlotGroup& g : r.groups()) {
        for (const FormRef& m : g.members) {
            char tok[64];
            std::snprintf(tok, sizeof tok, ":%06X", static_cast<unsigned>(m.localId));
            std::string needle = m.plugin;
            // the follower table writes the plugin with its file case: compare case-insensitively
            std::string lowerFollower = follower;
            for (auto& c : lowerFollower) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
            needle += tok;
            for (auto& c : needle) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
            CHECK(lowerFollower.find(needle) != std::string::npos);
            ++checked;
        }
    }
    CHECK(checked == 10);
}

TEST(sets_data, the_movers_see_the_party_and_the_hire_and_fire_scripts_see_the_player) {   // twin 900
    sc::SlotRules r;
    r.load(kDefaultCompanionSets);
    for (const char* mover : {"vCCmovetothefortSCRIPT", "YesManDeadSCRIPT", "FortLuciusScript", "VHDIndependentStartSCRIPT", "NVDLC04WreckageDoorSCRIPT"})
        CHECK(sc::viewSlotFor(r, mover, 1) == sc::kViewParty);
    for (const char* actor : {"NVDLC01HireScript", "NVDLC01FireScript", "NVDLC02HireTriggerSCRIPT", "NVDLC02FireTriggerSCRIPT", "NVDLC02ZionDoorSCRIPT",
                              "P04CompanionFireQuestSCRIPT", "VNPCFollowersQuestSCRIPT", "SomeScriptThatNeverMentionsCompanions"})
        CHECK(sc::viewSlotFor(r, actor, 2) == 2);
}
