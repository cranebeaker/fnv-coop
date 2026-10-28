// social/slots: data and judgement for the companion set pieces (campaign gap G34). C++17, no engine, no network, no clock.
//
// Three things the story scripts assume about ONE courier and that break with several recruiters:
//   * SINGLE SLOT GROUPS. Dead Money lets one companion in at a time (NVDLC01Followers.bHaveCompanion / rCurrentFollower), Honest Hearts swaps
//     Follows-Chalk for Waking Cloud (the hire INFO fires the other one). Each PLAYER has their own slot (owner decision D2), so
//       - a recruit that would give a player a second member of a group is turned back (the dialogue conditions normally prevent it; this is the net), and
//       - a swap that one player's recruit fires onto ANOTHER player's companion (the INFO reads the shared "hired" flag) is reverted: the casualty
//         keeps its owner. A swap on one's own companion is the vanilla behaviour and goes through.
//   * VIEW POLICY. Scripts that move or disable "every hired companion" by name (Fort entry and exit, Yes Man's Dam start, the ending slideshow) must read
//     the hired flags as the UNION of the party; scripts that hire or fire read them as the acting player's own. `viewSlotFor` answers which.
//   * PARTY COMPANIONS that data (not the rules engine) names: `party` lines.
//
// Data file (plugin/companions/data/companion_sets.txt, embedded in the DLL, a file next to the DLL overrides it):
//   group <name> <plugin>:<HEX>[,<plugin>:<HEX>...]      members of one single-slot group (every placed form of the companions)
//   party <plugin>:<HEX>[,...]                           always a party companion (the rules engine's owner=party adds more at run time)
//   view-party <ScriptEDID>                              the script reads the hired flags party-wide
#pragma once
#include <cstdint>
#include <set>
#include <string>
#include <vector>
#include "social/companions.h"

namespace fnvcoop::social {

struct SlotGroup {
    std::string name;
    std::set<FormRef> members;
};

constexpr PlayerId kViewParty = 0xFE;   // "no single player": the union over the party

class SlotRules {
public:
    struct LoadStats { size_t groups = 0, party = 0, viewParty = 0, bad = 0; };
    // Adds to what is there. Lines that do not parse are counted in `bad` and skipped.
    LoadStats load(const std::string& text);
    void addGroup(SlotGroup g);
    const SlotGroup* groupOf(const FormRef& npc) const;
    const std::vector<SlotGroup>& groups() const { return groups_; }
    const std::set<FormRef>& partyNpcs() const { return party_; }
    bool viewsParty(const std::string& scriptEdid) const;
    size_t viewPartyCount() const { return viewParty_.size(); }
    const std::set<std::string>& viewPartyNames() const { return viewParty_; }   // lower case
private:
    std::vector<SlotGroup> groups_;
    std::set<FormRef> party_;
    std::set<std::string> viewParty_;   // lower case
};

// Which slot's hired flags a script reads: the party union for the movers, else the acting player's.
PlayerId viewSlotFor(const SlotRules& rules, const std::string& scriptEdid, PlayerId acting);

// "Plugin.esm:0ABCDE" -> FormRef (plugin lower-cased, like the follower tables). false = malformed.
bool parseSetRef(const std::string& tok, FormRef& out);

// The companion a player has in a single-slot group (Dead Money: what NVDLC01Followers.rCurrentFollower holds in a one-courier game), else an empty FormRef.
// The shared ref variable keeps the last hire of ANY player; a script that runs for one player (the Gala elevator doors, the collar quest) must read this one.
FormRef currentMember(const CompanionBook& book, const SlotRules& rules, const std::string& group, PlayerId owner);

struct TxnVet {
    std::vector<Effect> learn;      // the CompanionOwner effects of the txn the book should learn (casualties and slot-less recruits left out)
    std::vector<Effect> repairs;    // effects the host commits afterwards to put the session back (a restored recruit, a dismissed stray)
    size_t swapsReverted = 0;       // dismissals of another player's companion fired by a recruit in the same group
    size_t slotDenied = 0;          // recruits that would have given a player a second member of a group
};
// Judges the CompanionOwner effects of ONE committed transaction against the book as it was before. Pure.
TxnVet vetCompanionTxn(const CompanionBook& book, const SlotRules& rules, const std::vector<Effect>& effects);

}  // namespace fnvcoop::social
