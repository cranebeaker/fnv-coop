// JSON form of Effects/EffectTxns for the bridge (capture.end / apply.txn) and for the static-oracle comparison
// (plugin/snapshot/tools/effect_oracle.py dict form: {"type":"SetStage","quest":"FalloutNV.esm:10A214","stage":10}).
// Game-free. FormRef strings are "<plugin>:<6 hex digits>"; plugin is the (lower-cased) FormRef plugin.
#pragma once
#include <string>

#include "common/minijson.h"
#include "fnvcoop/effect.h"

namespace nvhooks {

// Local replacement for FormRef::make (that one lives in the synccore .cpp; the plugin does not link synccore).
fnvcoop::FormRef makeRef(std::string plugin, uint32_t id);
std::string refToStr(const fnvcoop::FormRef& r);
bool strToRef(const std::string& s, fnvcoop::FormRef* out);

// The type name of an Effect as it appears in the JSON "type" field and in the identity table (identity.h); "?" for a bad index.
const char* effectTypeName(const fnvcoop::Effect& e);

nvcoop::json::Value effectToJson(const fnvcoop::Effect& e);
bool effectFromJson(const nvcoop::json::Value& v, fnvcoop::Effect* out, std::string* err);

// {"cause":{"kind":n,"source":"..","aux":n},"origin":n,"effects":[...]}
nvcoop::json::Value txnToJson(const fnvcoop::EffectTxn& t);
bool txnFromJson(const nvcoop::json::Value& v, fnvcoop::EffectTxn* out, std::string* err);

}  // namespace nvhooks
