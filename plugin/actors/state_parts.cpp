#include "actors/state_parts.h"

#include <algorithm>

#include "fnvcoop/wire.h"

namespace nvcoop::actors {

namespace {
constexpr size_t kMaxInvEntries = 256;
constexpr size_t kMaxEquipEntries = 64;
}  // namespace

const char* partName(PartKind k) {
    switch (k) {
        case PartKind::Inventory: return "inventory";
        case PartKind::Equipment: return "equipment";
        case PartKind::Look: return "look";
        case PartKind::WeaponState: return "weapon";
        case PartKind::Facts: return "facts";
        case PartKind::Pack: return "pack";
    }
    return "?";
}

Bytes encodeInventory(InvList items) {
    std::sort(items.begin(), items.end(), [](const InvItem& a, const InvItem& b) { return a.form < b.form; });
    fnvcoop::Archive a = fnvcoop::Archive::writer();
    size_t n = items.size();
    a.count(n, kMaxInvEntries);
    for (auto& it : items) { fnvcoop::io(a, it.form); a.vari32(it.count); }
    return a.finish();
}

bool decodeInventory(const Bytes& b, InvList& out) {
    out.clear();
    fnvcoop::Archive a = fnvcoop::Archive::reader(b.data(), b.size());
    size_t n = 0;
    if (!a.count(n, kMaxInvEntries)) return false;
    out.assign(n, {});
    for (auto& it : out) { fnvcoop::io(a, it.form); a.vari32(it.count); }
    return a.ok() && a.atEnd();
}

Bytes encodeEquipment(EquipList forms) {
    std::sort(forms.begin(), forms.end());
    forms.erase(std::unique(forms.begin(), forms.end()), forms.end());
    fnvcoop::Archive a = fnvcoop::Archive::writer();
    size_t n = forms.size();
    a.count(n, kMaxEquipEntries);
    for (auto& f : forms) fnvcoop::io(a, f);
    return a.finish();
}

bool decodeEquipment(const Bytes& b, EquipList& out) {
    out.clear();
    fnvcoop::Archive a = fnvcoop::Archive::reader(b.data(), b.size());
    size_t n = 0;
    if (!a.count(n, kMaxEquipEntries)) return false;
    out.assign(n, {});
    for (auto& f : out) fnvcoop::io(a, f);
    return a.ok() && a.atEnd();
}

Bytes encodeWeaponState(const WeaponState& w) {
    fnvcoop::Archive a = fnvcoop::Archive::writer();
    FormRef f = w.weapon;
    fnvcoop::io(a, f);
    uint8_t mods = static_cast<uint8_t>(w.mods & 0x07);
    a.u8(mods);
    return a.finish();
}

bool decodeWeaponState(const Bytes& b, WeaponState& out) {
    fnvcoop::Archive a = fnvcoop::Archive::reader(b.data(), b.size());
    WeaponState w;
    fnvcoop::io(a, w.weapon);
    a.u8(w.mods);
    if (!a.ok() || !a.atEnd() || (w.mods & ~0x07)) return false;
    out = w;
    return true;
}

// The default bridge from the generic part calls to the typed ones of a port.
bool IActorPort::readPart(NetId id, PartKind kind, Bytes& out) {
    switch (kind) {
        case PartKind::Inventory: {
            InvList v;
            if (!sampleInventory(id, v)) return false;
            out = encodeInventory(std::move(v));
            return true;
        }
        case PartKind::Equipment: {
            EquipList v;
            if (!sampleEquipment(id, v)) return false;
            out = encodeEquipment(std::move(v));
            return true;
        }
        case PartKind::Look: case PartKind::WeaponState: case PartKind::Facts: case PartKind::Pack: break;   // player parts: the player port's
    }
    return false;
}

bool IActorPort::writePart(NetId id, PartKind kind, const Bytes& value) {
    switch (kind) {
        case PartKind::Inventory: {
            InvList v;
            return decodeInventory(value, v) && applyInventory(id, v);
        }
        case PartKind::Equipment: {
            EquipList v;
            return decodeEquipment(value, v) && applyEquipment(id, v);
        }
        case PartKind::Look: case PartKind::WeaponState: case PartKind::Facts: case PartKind::Pack: break;   // player parts: the player port's
    }
    return false;
}

}  // namespace nvcoop::actors
