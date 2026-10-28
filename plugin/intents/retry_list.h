// The activators whose script a puppet cannot run to any effect but the host's own player can, and whose second run is safe (follow-ups 1, item 2).
//
// The Broc flower, the Xander root and every other harvest plant say `if State == 0 && GetActionRef == player`: a guest's puppet is not `player`, the script does nothing, and the
// guest would get the XP of its own half and no ingredient. The host therefore redoes an activation that recorded nothing with its own player standing in (intent_host.cpp,
// doActivate). That second run is only safe for a script whose OnActivate block can be run twice without a host-local trace, so it is data, not a guess: tools/gen_intents_data.py
// lists exactly the object scripts that (1) have an OnActivate block testing `GetActionRef == player` / `IsActionRef player`, (2) have no block but OnActivate / OnLoad / OnReset
// (no GameMode or MenuMode block that opens a message box or a menu on the host), and (3) use only commands the capture records or that touch nothing outside the world
// (AddItem, SetDestroyed, PlayGroup, Set, Disable, Enable, SetStage, StartQuest, RewardXP ...) in their OnActivate. Not on the list: no retry.
//
// nvcoop_intents.json, key "retryScripts": [{"script":"falloutnv.esm:0E9ACE","name":"BrocFlowerScript"}, ...]. The key is the SCRIPT form (the base form's attached script).
#pragma once
#include <set>
#include <string>

#include "common/minijson.h"
#include "intents/container_redirect.h"
#include "intents/intent_types.h"

namespace nvcoop::intents {

class RetryList {
public:
    bool load(const std::string& jsonText, std::string* err) {
        nvcoop::json::Value root;
        std::string perr;
        if (!nvcoop::json::parse(jsonText, root, perr)) { if (err) *err = "invalid json: " + perr; return false; }
        std::set<FormRef> next;
        for (const auto& e : root.get("retryScripts").items()) {
            FormRef r;
            if (!parseFormRef(e.get("script").asString(""), &r)) { if (err) *err = "retryScripts: bad script '" + e.get("script").asString("") + "'"; return false; }
            next.insert(r);
        }
        scripts_ = std::move(next);
        return true;
    }
    bool contains(const FormRef& script) const { return !script.plugin.empty() && scripts_.count(script) != 0; }
    size_t size() const { return scripts_.size(); }
    void clear() { scripts_.clear(); }
    void add(const FormRef& script) { scripts_.insert(script); }

private:
    std::set<FormRef> scripts_;
};

}  // namespace nvcoop::intents
