#include "fnvcoop/wire.h"
#include "fnvcoop/refop.h"
#include "fnvcoop/mutations.h"
#include <cmath>
#include <cstring>
#include <type_traits>

namespace fnvcoop {

FormRef FormRef::make(std::string plugin, uint32_t id) {
    for (auto& c : plugin) if (c >= 'A' && c <= 'Z') c = static_cast<char>(c - 'A' + 'a');
    return FormRef(std::move(plugin), id);
}

uint64_t hash64(const void* data, size_t size, uint64_t seed) {
    const uint8_t* p = static_cast<const uint8_t*>(data);
    uint64_t h = seed;
    for (size_t i = 0; i < size; ++i) { h ^= p[i]; h *= 1099511628211ull; }
    return h;
}

uint32_t crc32(const uint8_t* data, size_t size) {
    static uint32_t table[256];
    static bool init = false;
    if (!init) {
        for (uint32_t i = 0; i < 256; ++i) {
            uint32_t c = i;
            for (int k = 0; k < 8; ++k) c = (c & 1) ? 0xEDB88320u ^ (c >> 1) : c >> 1;
            table[i] = c;
        }
        init = true;
    }
    uint32_t c = 0xFFFFFFFFu;
    for (size_t i = 0; i < size; ++i) c = table[(c ^ data[i]) & 0xFF] ^ (c >> 8);
    return c ^ 0xFFFFFFFFu;
}

// ---- Archive ----------------------------------------------------------------------------------

Archive Archive::writer() { return Archive(); }

Archive Archive::reader(const uint8_t* data, size_t size) {
    Archive a;
    a.reading_ = true;
    a.in_ = data;
    a.size_ = size;
    size_t n = 0;
    if (a.count(n, 4096)) {
        for (size_t i = 0; i < n && a.ok_; ++i) {
            std::string s;
            a.str(s);
            a.table_.push_back(std::move(s));
        }
    }
    return a;
}

std::vector<uint8_t> Archive::finish() const {
    if (!ok_) return {};
    Archive head;
    uint64_t n = table_.size();
    head.varu(n);
    for (auto s : table_) head.str(s);
    std::vector<uint8_t> r = head.out_;
    r.insert(r.end(), out_.begin(), out_.end());
    return r;
}

void Archive::u8(uint8_t& v) {
    if (!ok_) { if (reading_) v = 0; return; }
    if (reading_) {
        if (pos_ + 1 > size_) { ok_ = false; v = 0; return; }
        v = in_[pos_++];
    } else {
        out_.push_back(v);
    }
}

template <class T>
static void fixedLE(Archive& a, T& v, bool reading, bool& ok, const uint8_t* in, size_t size, size_t& pos,
                    std::vector<uint8_t>& out) {
    if (!ok) { if (reading) v = 0; return; }
    if (reading) {
        if (size - pos < sizeof(T)) { ok = false; v = 0; return; }
        T r = 0;
        for (size_t i = 0; i < sizeof(T); ++i) r |= static_cast<T>(static_cast<T>(in[pos + i]) << (8 * i));
        pos += sizeof(T);
        v = r;
    } else {
        for (size_t i = 0; i < sizeof(T); ++i) out.push_back(static_cast<uint8_t>(v >> (8 * i)));
    }
    (void)a;
}
void Archive::u16(uint16_t& v) { fixedLE(*this, v, reading_, ok_, in_, size_, pos_, out_); }
void Archive::u32(uint32_t& v) { fixedLE(*this, v, reading_, ok_, in_, size_, pos_, out_); }
void Archive::u64(uint64_t& v) { fixedLE(*this, v, reading_, ok_, in_, size_, pos_, out_); }

void Archive::varu(uint64_t& v) {
    if (!ok_) { if (reading_) v = 0; return; }
    if (reading_) {
        uint64_t r = 0;
        for (int i = 0; i < 10; ++i) {
            if (pos_ >= size_) { ok_ = false; v = 0; return; }
            uint8_t b = in_[pos_++];
            if (i == 9 && b > 1) { ok_ = false; v = 0; return; }  // overflow past 64 bits
            r |= static_cast<uint64_t>(b & 0x7F) << (7 * i);
            if (!(b & 0x80)) {
                if (b == 0 && i > 0) { ok_ = false; v = 0; return; }  // non-canonical (overlong) encoding
                v = r;
                return;
            }
        }
        ok_ = false; v = 0;
    } else {
        uint64_t x = v;
        while (x >= 0x80) { out_.push_back(static_cast<uint8_t>(x | 0x80)); x >>= 7; }
        out_.push_back(static_cast<uint8_t>(x));
    }
}

void Archive::vari(int64_t& v) {
    if (reading_) {
        uint64_t z = 0;
        varu(z);
        v = static_cast<int64_t>(z >> 1) ^ -static_cast<int64_t>(z & 1);
    } else {
        uint64_t z = (static_cast<uint64_t>(v) << 1) ^ static_cast<uint64_t>(v >> 63);
        varu(z);
    }
}

void Archive::varu32(uint32_t& v) {
    uint64_t t = v;
    varu(t);
    if (reading_) {
        if (t > 0xFFFFFFFFull) { ok_ = false; v = 0; } else v = static_cast<uint32_t>(t);
    }
}

void Archive::vari32(int32_t& v) {
    int64_t t = v;
    vari(t);
    if (reading_) {
        if (t < INT32_MIN || t > INT32_MAX) { ok_ = false; v = 0; } else v = static_cast<int32_t>(t);
    }
}

void Archive::f32(float& v) {
    if (!ok_) { if (reading_) v = 0; return; }
    uint32_t bits = 0;
    if (!reading_) std::memcpy(&bits, &v, 4);
    u32(bits);
    if (reading_) {
        std::memcpy(&v, &bits, 4);
        if (!std::isfinite(v)) { ok_ = false; v = 0; }
    } else if (!std::isfinite(v)) {
        ok_ = false;
    }
}

void Archive::f64(double& v) {
    if (!ok_) { if (reading_) v = 0; return; }
    uint64_t bits = 0;
    if (!reading_) std::memcpy(&bits, &v, 8);
    u64(bits);
    if (reading_) {
        std::memcpy(&v, &bits, 8);
        if (!std::isfinite(v)) { ok_ = false; v = 0; }
    } else if (!std::isfinite(v)) {
        ok_ = false;
    }
}

void Archive::boolean(bool& v) {
    uint8_t b = v ? 1 : 0;
    u8(b);
    if (reading_) {
        if (b > 1) { ok_ = false; v = false; } else v = b == 1;
    }
}

bool Archive::count(size_t& n, size_t maxN) {
    uint64_t t = n;
    varu(t);
    if (!ok_) { n = 0; return false; }
    if (t > maxN || (reading_ && t > remaining())) { ok_ = false; n = 0; return false; }
    n = static_cast<size_t>(t);
    return true;
}

void Archive::str(std::string& s, size_t maxLen) {
    size_t n = s.size();
    if (!count(n, maxLen)) { s.clear(); return; }
    if (reading_) {
        s.assign(reinterpret_cast<const char*>(in_ + pos_), n);
        pos_ += n;
    } else {
        out_.insert(out_.end(), s.begin(), s.end());
    }
}

void Archive::bytes(std::vector<uint8_t>& b, size_t maxLen) {
    size_t n = b.size();
    if (!count(n, maxLen)) { b.clear(); return; }
    if (reading_) {
        b.assign(in_ + pos_, in_ + pos_ + n);
        pos_ += n;
    } else {
        out_.insert(out_.end(), b.begin(), b.end());
    }
}

void Archive::pluginRef(std::string& plugin) {
    if (!ok_) { if (reading_) plugin.clear(); return; }
    if (reading_) {
        uint64_t idx = 0;
        varu(idx);
        if (!ok_ || idx >= table_.size()) { ok_ = false; plugin.clear(); return; }
        plugin = table_[static_cast<size_t>(idx)];
    } else {
        auto it = tableIndex_.find(plugin);
        uint32_t idx;
        if (it == tableIndex_.end()) {
            if (plugin.size() > kMaxStringLen || table_.size() >= 4096) { ok_ = false; return; }
            idx = static_cast<uint32_t>(table_.size());
            table_.push_back(plugin);
            tableIndex_.emplace(plugin, idx);
        } else {
            idx = it->second;
        }
        uint64_t t = idx;
        varu(t);
    }
}

// ---- field serializers --------------------------------------------------------------------------

void io(Archive& a, FormRef& v) {
    a.pluginRef(v.plugin);
    a.varu32(v.localId);
}

void io(Archive& a, Recipient& v) {
    uint8_t m = v.mode;
    a.u8(m);
    if (a.reading()) {
        if (m > Recipient::Mask) { a.fail(); return; }
        v.mode = static_cast<Recipient::Mode>(m);
    }
    if (v.mode == Recipient::Mask) a.u8(v.mask);
    else if (a.reading()) v.mask = 0;
}

void io(Archive& a, Cause& v) {
    uint8_t k = static_cast<uint8_t>(v.kind);
    a.u8(k);
    if (a.reading()) {
        if (k >= static_cast<uint8_t>(CauseKind::Count_)) { a.fail(); return; }
        v.kind = static_cast<CauseKind>(k);
    }
    io(a, v.source);
    a.vari32(v.aux);
}

void io(Archive& a, SharePolicy& v) {
    uint8_t bits = static_cast<uint8_t>(int(v.journal) | int(v.globals) << 1 | int(v.kills) << 2 | int(v.karma) << 3 |
                                        int(v.reputation) << 4 | int(v.questRewardsToAll) << 5 |
                                        int(v.clientLoad == SharePolicy::ClientLoad::Block) << 6);
    a.u8(bits);
    a.u8(v.maxPlayers);
    if (a.reading()) {
        if (v.maxPlayers < 1 || v.maxPlayers > kMaxSlots || (bits & 0x80)) { a.fail(); return; }
        v.journal = bits & 1; v.globals = bits >> 1 & 1; v.kills = bits >> 2 & 1;
        v.karma = bits >> 3 & 1; v.reputation = bits >> 4 & 1; v.questRewardsToAll = bits >> 5 & 1;
        v.clientLoad = (bits >> 6 & 1) ? SharePolicy::ClientLoad::Block : SharePolicy::ClientLoad::Rejoin;
    }
}

namespace {
void ioE(Archive& a, SetStage& e) { io(a, e.quest); a.varu32(e.stage); }
void ioE(Archive& a, SetObjective& e) { io(a, e.quest); a.varu32(e.objective); a.u8(e.flags); }
void ioE(Archive& a, SetGlobal& e) { io(a, e.global); a.f32(e.value); }
void ioE(Archive& a, EnableRef& e) { io(a, e.ref); }
void ioE(Archive& a, DisableRef& e) { io(a, e.ref); }
void ioE(Archive& a, AddItem& e) { io(a, e.item); a.vari32(e.count); io(a, e.to); }
void ioE(Archive& a, RemoveItem& e) { io(a, e.item); a.vari32(e.count); io(a, e.to); }
void ioE(Archive& a, ModActorValue& e) { a.varu32(e.av); a.vari32(e.delta); io(a, e.to); }
void ioE(Archive& a, ModReputation& e) { io(a, e.faction); a.vari32(e.fame); a.vari32(e.infamy); io(a, e.to); }
void ioE(Archive& a, ModKarma& e) { a.vari32(e.delta); io(a, e.to); }
void ioE(Archive& a, StartQuest& e) { io(a, e.quest); }
void ioE(Archive& a, StopQuest& e) { io(a, e.quest); }
void ioE(Archive& a, MoveRef& e) { io(a, e.ref); io(a, e.cell); a.f32(e.x); a.f32(e.y); a.f32(e.z); a.boolean(e.partyWarp); }
void ioE(Archive& a, KillActor& e) { io(a, e.actor); }
void ioE(Archive& a, ResurrectActor& e) { io(a, e.actor); }
void ioE(Archive& a, SetScriptVar& e) {
    io(a, e.owner);
    uint32_t i = e.index;
    a.varu32(i);
    if (a.reading()) { if (i > 0xFFFF) { a.fail(); return; } e.index = static_cast<uint16_t>(i); }
    a.f64(e.value);
}
void ioE(Archive& a, SetPlayerControls& e) { a.varu32(e.controls); a.boolean(e.enable); io(a, e.to); }
void ioE(Archive& a, PartyTether& e) { io(a, e.cell); a.boolean(e.on); }
void ioE(Archive&, SlideshowEnd&) {}
void ioE(Archive& a, SaveSuppress& e) { a.boolean(e.on); }
void ioE(Archive& a, CompanionOwner& e) {
    io(a, e.npc); a.u8(e.owner); a.boolean(e.active); a.u8(e.kind);
    if (a.reading() && (e.owner >= kMaxSlots || e.kind > 1)) a.fail();
}
void ioE(Archive& a, ForcedDialogue& e) { io(a, e.npc); io(a, e.topic); a.u8(e.initiator); if (a.reading() && e.initiator >= kMaxSlots) a.fail(); }
void ioE(Archive& a, BarrierArrive& e) { io(a, e.barrier); a.u8(e.required); }
void ioE(Archive& a, BarrierRelease& e) { io(a, e.barrier); }
void ioE(Archive& a, SayOnceFlag& e) { io(a, e.info); }
void ioE(Archive& a, CompleteQuest& e) { io(a, e.quest); }
void ioE(Archive& a, FailQuest& e) { io(a, e.quest); }
void ioE(Archive& a, SpawnRef& e) { a.varu32(e.id); if (a.reading() && (e.id >> 24) != 0xFF) { a.fail(); return; } io(a, e.base); io(a, e.cell); a.f32(e.x); a.f32(e.y); a.f32(e.z); a.boolean(e.leveled); }
void ioE(Archive& a, DespawnRef& e) { a.varu32(e.id); if (a.reading() && (e.id >> 24) != 0xFF) a.fail(); a.boolean(e.forget); }
void ioE(Archive& a, ObjectState& e) {
    io(a, e.ref); a.u8(e.mask); a.boolean(e.open); a.boolean(e.locked); a.vari32(e.level); a.boolean(e.destroyed); a.boolean(e.activatedOnce);
    if (a.reading() && (e.mask & ~15u)) a.fail();
    a.boolean(e.hasBefore);
    if (e.hasBefore) { a.boolean(e.beforeLocked); a.vari32(e.beforeLevel); } else if (a.reading()) { e.beforeLocked = false; e.beforeLevel = 0; }
}
void ioE(Archive& a, RefScriptVar& e) {
    io(a, e.ref);
    uint32_t i = e.index;
    a.varu32(i);
    if (a.reading()) { if (i > 0xFFFF) { a.fail(); return; } e.index = static_cast<uint16_t>(i); }
    a.f64(e.value);
    a.boolean(e.hasBefore);
    if (e.hasBefore) a.f64(e.before); else if (a.reading()) e.before = 0;
}
void ioE(Archive& a, RefAddItem& e) { io(a, e.ref); io(a, e.item); a.vari32(e.count); }
void ioE(Archive& a, RefRemoveItem& e) { io(a, e.ref); io(a, e.item); a.vari32(e.count); }
void ioE(Archive& a, RemoveAllItems& e) {
    io(a, e.ref); a.boolean(e.undo); a.boolean(e.hasBefore);
    if (!e.hasBefore && !e.undo) { if (a.reading()) e.items.clear(); return; }
    size_t n = e.items.size();
    if (!a.count(n, 1024)) { e.items.clear(); return; }
    if (a.reading()) {
        e.items.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { FormRef f; int32_t c = 0; io(a, f); a.vari32(c); if (a.ok()) e.items.emplace_back(f, c); }
    } else {
        for (auto& [f, c] : e.items) { FormRef ff = f; int32_t cc = c; io(a, ff); a.vari32(cc); }
    }
}
void ioE(Archive& a, AddTopic& e) { io(a, e.topic); }
void ioE(Archive& a, AddNote& e) { io(a, e.note); }
void ioE(Archive& a, RemoveNote& e) { io(a, e.note); }
void ioE(Archive& a, SetMapMarker& e) {
    io(a, e.marker); a.u8(e.flags); if (a.reading() && e.flags > 3) a.fail();
    a.boolean(e.hasBefore);
    if (e.hasBefore) { a.u8(e.before); if (a.reading() && e.before > 3) a.fail(); } else if (a.reading()) e.before = 0;
}
void ioE(Archive& a, ForceActiveQuest& e) { io(a, e.quest); io(a, e.to); }
void ioE(Archive& a, RequestSave& e) { a.u8(e.kind); if (a.reading() && e.kind > kSaveForce) a.fail(); }
void ioE(Archive& a, SetWeather& e) {
    io(a, e.weather); a.boolean(e.force);
    a.boolean(e.hasBefore);
    if (e.hasBefore) { io(a, e.before); a.boolean(e.beforeForce); } else if (a.reading()) { e.before = FormRef(); e.beforeForce = false; }
}
void ioE(Archive& a, SetTimeScale& e) {
    a.f32(e.scale);
    a.boolean(e.hasBefore);
    if (e.hasBefore) a.f32(e.before); else if (a.reading()) e.before = 0;
}
void ioE(Archive& a, RefAngle& e) {
    io(a, e.ref); a.u8(e.axis); a.f32(e.degrees);
    if (a.reading() && e.axis > kAxisZ) a.fail();
    a.boolean(e.hasBefore);
    if (e.hasBefore) a.f32(e.before); else if (a.reading()) e.before = 0;
}
void ioE(Archive& a, RefCommand& e) {
    uint32_t op = e.op;
    a.varu32(op);
    if (a.reading()) { if (op > 0xFFFF || !refOpKnown(static_cast<uint16_t>(op))) { a.fail(); return; } e.op = static_cast<uint16_t>(op); }
    io(a, e.ref); io(a, e.a); io(a, e.b);
    a.vari32(e.i); a.vari32(e.j); a.vari32(e.k); a.vari32(e.l);
    a.f32(e.f);
    a.str(e.s);
    io(a, e.to);
    if (refOpIsWide(e.op) && !mutations().refCmdNarrow) { io(a, e.c); io(a, e.d); a.vari32(e.m); }   // the second shape (wide ops only)
}
void ioE(Archive& a, AdvanceTime& e) { a.varu32(e.minutes); }
void ioE(Archive& a, CompleteAllObjectives& e) { io(a, e.quest); }
void ioE(Archive& a, SetLock& e) {
    io(a, e.ref); a.boolean(e.locked); a.vari32(e.level);
    a.boolean(e.hasBefore);
    if (e.hasBefore) { a.boolean(e.beforeLocked); a.vari32(e.beforeLevel); } else if (a.reading()) { e.beforeLocked = false; e.beforeLevel = 0; }
}
void ioE(Archive& a, SetFactionRank& e) { io(a, e.actor); io(a, e.faction); a.vari32(e.rank); }
void ioE(Archive& a, SetPerk& e) { io(a, e.perk); a.vari32(e.rank); io(a, e.to); }
void ioE(Archive& a, SwapTexture& e) { io(a, e.ref); io(a, e.texture); a.u8(e.slot); }
void ioE(Archive& a, SetFastTravelLock& e) { io(a, e.scope); a.boolean(e.on); }
void ioE(Archive& a, ContainerTransfer& e) { io(a, e.src); io(a, e.dst); a.boolean(e.srcPlayer); a.boolean(e.dstPlayer); io(a, e.to); }

template <size_t I = 0>
bool readAlt(Archive& a, size_t idx, Effect& e) {
    if constexpr (I == std::variant_size_v<Effect>) {
        return false;
    } else {
        if (idx != I) return readAlt<I + 1>(a, idx, e);
        std::variant_alternative_t<I, Effect> v{};
        ioE(a, v);
        e = std::move(v);
        return a.ok();
    }
}
}  // namespace

void io(Archive& a, Effect& v) {
    uint8_t tag = static_cast<uint8_t>(v.index());
    a.u8(tag);
    if (a.reading()) {
        if (!a.ok() || !readAlt(a, tag, v)) a.fail();
    } else {
        std::visit([&](auto& e) { ioE(a, e); }, v);
    }
}

const char* effectName(const Effect& e) {
    static const char* names[] = {"SetStage", "SetObjective", "SetGlobal", "EnableRef", "DisableRef", "AddItem",
                                  "RemoveItem", "ModActorValue", "ModReputation", "ModKarma", "StartQuest",
                                  "StopQuest", "MoveRef", "KillActor", "ResurrectActor", "SetScriptVar",
                                  "SetPlayerControls", "PartyTether", "SlideshowEnd", "SaveSuppress", "CompanionOwner",
                                  "ForcedDialogue", "BarrierArrive", "BarrierRelease", "SayOnceFlag", "CompleteQuest",
                                  "CompleteAllObjectives", "SetLock", "SetFactionRank", "SetPerk", "SwapTexture", "SetFastTravelLock", "FailQuest", "SpawnRef", "DespawnRef", "ObjectState", "RefScriptVar", "RefAddItem", "RefRemoveItem", "RemoveAllItems", "AddTopic", "AddNote", "RemoveNote", "SetMapMarker", "ForceActiveQuest", "RequestSave", "SetWeather", "SetTimeScale", "AdvanceTime", "RefAngle", "RefCommand", "ContainerTransfer"};
    static_assert(sizeof(names) / sizeof(names[0]) == std::variant_size_v<Effect>, "one name per effect type");
    return names[e.index()];
}

void io(Archive& a, EffectTxn& v) {
    a.varu(v.seq);
    a.u8(v.origin);
    if (a.reading() && v.origin >= kMaxSlots) { a.fail(); return; }
    a.u8(v.flags);
    a.varu32(v.originEpoch);
    a.varu32(v.originIntent);
    io(a, v.cause);
    size_t n = v.effects.size();
    if (!a.count(n, kMaxEffectsPerTxn)) { v.effects.clear(); return; }
    if (a.reading()) {
        v.effects.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) {
            Effect e;
            io(a, e);
            if (a.ok()) v.effects.push_back(std::move(e));
        }
    } else {
        for (auto& e : v.effects) io(a, e);
    }
}

}  // namespace fnvcoop
