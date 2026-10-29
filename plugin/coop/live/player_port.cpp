#include "player_port.h"

namespace nvcoop::livewire {

namespace live = fnvcoop::live;

bool PlayerPort::loaded(NetId id) {
    if (!live::isPlayerNet(id)) return false;
    if (!ep_.playerSessionUp()) return false;
    return true;   // the local player is always there; a remote one has a puppet entry from the roster on (its ref may still be spawning: writes are remembered)
}

bool PlayerPort::sample(NetId id, live::ActorSnapshot& out) {
    if (!live::isPlayerNet(id) || live::slotOfPlayerNet(id) != ep_.playerSelf()) return false;
    return ep_.takeLocalSample(out);
}

bool PlayerPort::readPart(NetId id, PartKind kind, Bytes& out) {
    if (!live::isPlayerNet(id) || live::slotOfPlayerNet(id) != ep_.playerSelf()) return false;   // only the local player is read here
    return ep_.readLocalPart(kind, out);
}

bool PlayerPort::writePart(NetId id, PartKind kind, const Bytes& value) {
    if (!live::isPlayerNet(id)) { ++wrongCalls_; return false; }
    const PlayerId slot = live::slotOfPlayerNet(id);
    if (slot == ep_.playerSelf()) return false;   // never written onto the local player
    return ep_.writeRemotePart(slot, kind, value);
}

void PlayerPort::onSnapshot(NetId id, const live::ActorSnapshot& s, uint32_t arrivalMs) {
    if (!live::isPlayerNet(id)) { ++wrongCalls_; return; }
    const PlayerId slot = live::slotOfPlayerNet(id);
    if (slot == ep_.playerSelf()) return;
    ep_.remoteSnapshot(slot, s, arrivalMs);
}

}  // namespace nvcoop::livewire
