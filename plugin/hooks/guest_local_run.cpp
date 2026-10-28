#include "guest_local_run.h"

#include <algorithm>
#include <cctype>
#include <cstdlib>

#include "common/minijson.h"

namespace nvhooks {

namespace {
// "plugin:hexLocalId" -> lower-cased plugin and id; false when it is not that shape
bool parseScriptRef(const std::string& ref, fnvcoop::FormRef* out) {
    size_t colon = ref.rfind(':');
    if (colon == std::string::npos || colon == 0 || colon + 1 >= ref.size()) return false;
    char* end = nullptr;
    unsigned long id = std::strtoul(ref.c_str() + colon + 1, &end, 16);
    if (*end != '\0' || id == 0) return false;
    std::string plugin = ref.substr(0, colon);
    std::transform(plugin.begin(), plugin.end(), plugin.begin(), [](unsigned char ch) { return static_cast<char>(std::tolower(ch)); });
    *out = fnvcoop::FormRef(plugin, static_cast<uint32_t>(id));
    return true;
}
}  // namespace

bool parseGuestLocalRunRules(const std::string& rulesJsonText, GuestLocalRunList* out, std::string* err) {
    nvcoop::json::Value root;
    std::string perr;
    if (!nvcoop::json::parse(rulesJsonText, root, perr)) {
        if (err) *err = perr;
        return false;
    }
    const nvcoop::json::Value* quests = root.find("quests");
    if (!quests || quests->type() != nvcoop::json::Value::Object) {
        if (err) *err = "missing \"quests\" object";
        return false;
    }
    GuestLocalRunList list;
    for (const auto& q : quests->members()) {
        const nvcoop::json::Value* rules = q.second.find("rules");
        if (!rules || rules->type() != nvcoop::json::Value::Array) continue;
        for (const auto& r : rules->items()) {
#ifdef NVHOOKS_MUTANT
            if (r.get("kind").asString("") != "GuestLocalRunX") continue;   // mutant: the rule is not recognised (the list stays empty)
#else
            if (r.get("kind").asString("") != "GuestLocalRun") continue;
#endif
            const nvcoop::json::Value* scripts = r.get("params").find("scripts");
            if (!scripts || scripts->type() != nvcoop::json::Value::Array) continue;
            for (const auto& s : scripts->items()) {
                fnvcoop::FormRef ref;
                if (!parseScriptRef(s.asString(""), &ref)) { ++list.skipped; continue; }
                list.scripts.push_back(ref);
                list.quests.push_back(q.first);
                list.reasons.push_back(r.get("reason").asString(""));
            }
        }
    }
    if (out) *out = std::move(list);
    return true;
}

bool parseGuestLocalRun(const std::string& jsonText, GuestLocalRunList* out, std::string* err) {
    nvcoop::json::Value root;
    std::string perr;
    if (!nvcoop::json::parse(jsonText, root, perr)) {
        if (err) *err = perr;
        return false;
    }
    const nvcoop::json::Value* arr = root.find("scripts");
    if (!arr || arr->type() != nvcoop::json::Value::Array) {
        if (err) *err = "missing \"scripts\" array";
        return false;
    }
    GuestLocalRunList list;
    for (const auto& e : arr->items()) {
        std::string ref = e.get("script").asString("");
        size_t colon = ref.rfind(':');
        if (colon == std::string::npos || colon == 0 || colon + 1 >= ref.size()) { ++list.skipped; continue; }
        char* end = nullptr;
        unsigned long id = std::strtoul(ref.c_str() + colon + 1, &end, 16);
        if (*end != '\0' || id == 0) { ++list.skipped; continue; }
        std::string plugin = ref.substr(0, colon);
        std::transform(plugin.begin(), plugin.end(), plugin.begin(), [](unsigned char c) { return static_cast<char>(std::tolower(c)); });
        list.scripts.push_back(fnvcoop::FormRef(plugin, static_cast<uint32_t>(id)));
        list.quests.push_back(e.get("quest").asString(""));
        list.reasons.push_back(e.get("reason").asString(""));
    }
    if (out) *out = std::move(list);
    return true;
}

const std::vector<fnvcoop::FormRef>& hostQuestStateScripts() {
    static const std::vector<fnvcoop::FormRef> k = {
        fnvcoop::FormRef("honesthearts.esm", 0x008A32),   // NVDLC02MQ00Script
        fnvcoop::FormRef("oldworldblues.esm", 0x002FD0),  // NVDLC03MQ00SCRIPT
        fnvcoop::FormRef("lonesomeroad.esm", 0x003604),   // NVDLC04MQ00SCRIPT
        fnvcoop::FormRef("deadmoney.esm", 0x0062B1),      // NVDLC01MQ00Script
    };
    return k;
}

}  // namespace nvhooks
