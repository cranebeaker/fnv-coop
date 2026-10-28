#include "loot/loot_sync.h"

#include <algorithm>
#include <cstdio>
#include "fnvcoop_mut.h"
#include "loot/loot_wire.h"

namespace nvcoop::loot {

LootSync::LootSync(ILootEngine& eng, SyncConfig cfg) : eng_(eng), cfg_(cfg) {}
LootSync::~LootSync() = default;

// ================================================================================================================ session

void LootSync::makeClient(PlayerId self) {
    lc::ClientConfig cc = cfg_.client;
    cc.self = self;
    client_ = std::make_unique<lc::LootClient>(cc, [this](const lc::Body& b) { sendUp(b); });
}

void LootSync::sessionBegin(const nvcoop::livewire::SessionInfo& info, nvcoop::livewire::ILiveLink& link) {
    if (!cfg_.enabled) return;
    link_ = &link;
    self_ = info.isHost ? kHostPlayer : info.self;
    ++stats_.sessions;
    inbox_.clear();
    menuRef_ = 0; menuNet_ = 0;
    suppressClose_.clear(); blockedMenu_.clear();
    ownedClutter_.clear();
    playerSeen_.clear(); suspect_.clear(); implicit_.clear(); engineBase_.clear();
    if (!FNV_MUT(153)) { resolves_.clear(); creates_.clear(); createsDue_.clear(); removes_.clear(); notesDone_.clear(); }
    discovered_.clear(); nextDiscover_ = 0;
    host_.reset();
    if (info.isHost) {
        host_ = std::make_unique<lc::LootHost>(cfg_.host);
        if (!pendingSidecar_.empty()) {
            host_->load(pendingSidecar_);
            pendingSidecar_.clear();
        }
    }
    makeClient(self_);
    if (info.isHost) {
        linkUp_ = true;
        client_->hostLinkUp(now_);   // the host's own client: its table request is answered in-process
        pump();
    } else {
        linkUp_ = false;             // a guest waits for hostLinkUp()
    }
}

void LootSync::sessionEnd() {
    host_.reset();
    client_.reset();
    link_ = nullptr;
    linkUp_ = false;
    inbox_.clear();
    menuRef_ = 0; menuNet_ = 0; blockedMenu_.clear();
    ownedClutter_.clear();
    playerSeen_.clear(); suspect_.clear(); implicit_.clear(); engineBase_.clear();
    if (!FNV_MUT(153)) { resolves_.clear(); creates_.clear(); createsDue_.clear(); removes_.clear(); notesDone_.clear(); }   // twin 153: scheduled work of a dead session survives into the next one
    discovered_.clear();
}

void LootSync::playerJoined(PlayerId p) {
    if (!host_) return;
    lc::LootHost::Out out;
    host_->playerJoined(p, out);
    deliver(out);
}

void LootSync::playerLeft(PlayerId p) {
    if (!host_ || FNV_MUT(157)) return;   // twin 157: a leaver's locks and clutter stay
    lc::LootHost::Out out;
    host_->playerLeft(p, now_, out);
    deliver(out);
    pump();
}

void LootSync::hostLinkUp() {
    if (!client_ || host_) return;
    linkUp_ = true;
    client_->hostLinkUp(now_);
    pump();
}

void LootSync::hostLinkDown() {
    if (!client_ || host_) return;
    linkUp_ = false;
    if (!FNV_MUT(156)) client_->hostLinkDown();   // twin 156: pending requests are never abandoned
    pump();
}

void LootSync::loadBegin() {
    ++stats_.loads;
    menuRef_ = 0; menuNet_ = 0;
    suppressClose_.clear(); blockedMenu_.clear();
    ownedClutter_.clear();
    resolves_.clear(); creates_.clear(); createsDue_.clear(); removes_.clear(); notesDone_.clear();
    discovered_.clear(); nextDiscover_ = 0;
    inbox_.clear();
    playerSeen_.clear(); suspect_.clear(); implicit_.clear(); engineBase_.clear();   // engine counts of the old world mean nothing in the loaded one
    if (client_ && !FNV_MUT(152)) client_->worldReset();   // twin 152: the view (and its "already has a ref" memory) survives a load
}

void LootSync::loadEnd() {
    if (!client_) return;
    if (linkUp_) {
        client_->hostLinkUp(now_);   // fetch the table again; world items get refs again
        pump();
    }
}

void LootSync::gameExit() { loadBegin(); }

// ================================================================================================================ network

void LootSync::sendUp(const lc::Body& b) {
    if (host_) {
        lc::LootHost::Out out;
        host_->onRequest(kHostPlayer, b, now_, out);
        grantNotes(out);
        deliver(out);
        return;
    }
    if (!link_) return;
    lc::Envelope e;
    e.src = self_;
    e.body = b;
    std::vector<uint8_t> bytes = lc::encodeEnvelope(e);
    if (bytes.empty()) return;
    ++stats_.datagramsOut;
    link_->send(kHostPlayer, !lc::isUnreliable(b), std::move(bytes));
}

// A pickup claim the host just WON for a note: the picker's own engine added the note to ITS journal; the host's journal (GetHasNote on the host, the quest
// scripts that poll it) and every other copy get it through AddNote. Claims that lost, and anything that is not a note, grant nothing.
void LootSync::grantNotes(const lc::LootHost::Out& out) {
    if (FNV_MUT(178)) return;   // twin 178: a picked-up note never reaches the host's journal
    for (const auto& o : out) {
        const auto* r = std::get_if<lc::Result>(&o.body);
        if (!r || r->op != lc::OpKind::Claim || r->base.plugin.empty()) continue;
        if (r->status != lc::Status::Ok) continue;                      // a lost claim grants nothing
        if (!notesDone_.insert(r->id).second && !FNV_MUT(179)) continue;   // twin 179: the winner's repeated claim of the same note is granted again
        if (!FNV_MUT(180) && !eng_.isNoteBase(r->base)) continue;      // twin 180: every won claim is granted as a note (a Stimpak becomes a journal entry)
        if (eng_.addNote(r->base)) ++stats_.notesGranted; else ++stats_.notesRefused;
    }
}

void LootSync::deliver(lc::LootHost::Out& out) {
    for (auto& o : out) {
        if (o.to == kHostPlayer) {
            if (!FNV_MUT(149)) inbox_.push_back(std::move(o.body));   // twin 149: the host's own answers are dropped
            continue;
        }
        if (!link_) continue;
        lc::Envelope e;
        e.src = kHostPlayer;
        e.body = std::move(o.body);
        std::vector<uint8_t> bytes = lc::encodeEnvelope(e);
        if (bytes.empty()) continue;
        ++stats_.datagramsOut;
        link_->send(o.to, !lc::isUnreliable(e.body), std::move(bytes));
    }
    out.clear();
}

void LootSync::receive(PlayerId from, const uint8_t* data, size_t size) {
    if (!link_ || !client_) return;
    ++stats_.datagramsIn;
    lc::Envelope e;
    if (!lc::decodeEnvelope(data, size, e)) { ++stats_.undecodable; return; }
    if (host_) {
        // the slot is the transport peer's, never the one the datagram claims; a request that claims another sender is counted
        if (e.src != from) ++stats_.spoofed;
        const PlayerId who = FNV_MUT(146) ? e.src : from;   // twin 146: the claimed sender is believed (a guest can act as another player)
        if (!lc::isRequest(e.body)) { ++stats_.wrongDirection; return; }
        poll();
        lc::LootHost::Out out;
        host_->onRequest(who, e.body, now_, out);
        grantNotes(out);
        deliver(out);
        pump();
        return;
    }
    if (from != kHostPlayer) { ++stats_.spoofed; return; }   // a guest only listens to the host
    if (lc::isRequest(e.body) && !std::holds_alternative<lc::MClutter>(e.body)) { ++stats_.wrongDirection; return; }
    if (!FNV_MUT(141)) poll();   // twin 141: the container is not read before a correction from the host is handled (it can swallow the player's last move)
    client_->onMessage(e.body, now_);
    drain();
}

void LootSync::pump() {
    if (pumping_ || !client_) return;
    pumping_ = true;
    for (int guard = 0; guard < 256 && !inbox_.empty(); ++guard) {
        lc::Body b = std::move(inbox_.front());
        inbox_.pop_front();
        poll();
        client_->onMessage(b, now_);
        drain();
    }
    pumping_ = false;
}

// ================================================================================================================ the container watcher

bool LootSync::readExact(NetId net, Items& out) {
    bool exact = true;
    if (!eng_.readContents(net, out, exact)) { ++stats_.readFailures; return false; }
    if (!exact && !FNV_MUT(145)) { ++stats_.inexactReads; return false; }   // twin 145: a partial read (an unresolved leveled list inside) is offered as the shared contents
    return true;
}

// A menu the host refused and the engine could not close: whatever the player moves in it is undone (items back into the container, taken out of the player).
void LootSync::revertBlockedMenu() {
    const lc::ContainerRec* rec = client_->container(menuNet_);
    if (!rec) return;
    Items have;
    bool exact = true;
    if (!eng_.readContents(menuNet_, have, exact)) { ++stats_.readFailures; return; }
    lc::normalize(have);
    const Items shared = rec->items;
    if (lc::digestOf(have) == lc::digestOf(shared)) return;
    for (const lc::ItemDiff& x : lc::diffItems(shared, have)) {
        const int32_t delta = x.b < x.a ? -(x.a - x.b) : (x.b - x.a);   // the container lost -> the player gained it: take it back
        if (!eng_.adjustPlayer(x.base, delta)) ++stats_.actionsFailed;
        ++stats_.rollbacksApplied;
        stats_.revertedItems += static_cast<uint64_t>(delta < 0 ? -delta : delta);
    }
    applyContents(menuNet_, shared);
}

void LootSync::poll() {
    if (!client_ || !menuNet_) return;
    if (blockedMenu_.count(menuNet_)) { revertBlockedMenu(); return; }
    Items now;
    bool exact = true;
    if (!eng_.readContents(menuNet_, now, exact)) { ++stats_.readFailures; return; }   // an inexact read of an OPEN container still shows what the player moved
    ++stats_.polls;
    client_->observed(menuNet_, now, now_);
}

void LootSync::openMenu(RefId ref) {
    menuRef_ = ref;
    menuNet_ = 0;
    ContainerDesc d;
    if (!eng_.describeContainer(ref, d) || !d.net) return;
    Items items;
    bool exact = true;
    if (!eng_.readContents(d.net, items, exact)) { ++stats_.readFailures; return; }
    menuNet_ = d.net;
    engineBase_.erase(d.net);   // the menu owns the engine copy now; the reference is taken again when the container matches after it closes
    ++stats_.menusOpened;
    lastPollMs_ = now_;
    client_->opened(d.net, d.kind, d.cell, d.pos, items, now_);
}

void LootSync::closeMenu() {
    const NetId net = menuNet_;
    menuRef_ = 0;
    menuNet_ = 0;
    if (!net) return;
    engineBase_.erase(net);
    blockedMenu_.erase(net);
    ++stats_.menusClosed;
    if (suppressClose_.erase(net) && !FNV_MUT(144)) return;   // twin 144: a menu WE closed (busy / lock lost) is reported as the player closing it
    Items items;
    bool exact = true;
    if (!eng_.readContents(net, items, exact)) { ++stats_.readFailures; items = Items(); }
    client_->closed(net, items, now_);
}

void LootSync::pollMenu(uint64_t now) {
    const RefId top = eng_.openMenuRef();
    if (top != menuRef_) {
        if (menuRef_) closeMenu();
        if (top) openMenu(top);
        return;
    }
    if (menuNet_ && now - lastPollMs_ >= cfg_.pollMs) {
        lastPollMs_ = now;
        poll();
    }
}

// ================================================================================================================ actions

void LootSync::applyContents(NetId net, const Items& want) {
    // A closed container whose engine copy moved on its own since it last matched (a menu-less take whose AddItem is still to come, a script) is not
    // overwritten by a change the others made: the write would turn the player's move into "the engine already shows the shared contents" and nobody
    // would ever ask the host for it (both grabbers of the same stack kept everything, bench dino-20261027-144058). screenTransfer attributes the move; its
    // Open / Take answer writes the right contents, a script's change is handed back after the grace.
    if (cfg_.menuless && net != menuNet_ && !implicit_.count(net) && client_ && !FNV_MUT(165)) {
        auto b = engineBase_.find(net);
        if (b != engineBase_.end() && client_->transferIdle(net)) {
            Items cur;
            bool exact = true;
            if (eng_.readContents(net, cur, exact)) {
                lc::normalize(cur);
                if (lc::digestOf(cur) != lc::digestOf(b->second)) { ++stats_.setsDeferred; return; }
            }
        }
    }
    const bool ok = eng_.setContents(net, want);
    if (!ok) ++stats_.actionsFailed;
    // what the engine really holds now is the new baseline: if it could not be set, the watcher must not mistake the difference for a player move
    Items now;
    bool exact = true;
    const bool read = eng_.readContents(net, now, exact);
    if (!FNV_MUT(143) && read) client_->rebaseline(net, now);
    else if (!FNV_MUT(143)) client_->rebaseline(net, want);
    if (cfg_.menuless && net != menuNet_) {
        Items b = read && exact ? now : want;
        lc::normalize(b);
        engineBase_[net] = std::move(b);
    }
}

void LootSync::run(const std::vector<lc::Action>& acts) {
    for (const lc::Action& a : acts) {
        ++stats_.actionsRun;
        switch (a.kind) {
        case lc::Action::PlayerAdjust:
            if (!FNV_MUT(142)) {   // twin 142: a refused transfer is never taken back
                if (!eng_.adjustPlayer(a.base, a.delta)) ++stats_.actionsFailed;
                ++stats_.rollbacksApplied;
                if (!FNV_MUT(166)) stats_.revertedItems += static_cast<uint64_t>(a.delta < 0 ? -a.delta : a.delta);   // twin 166: the status does not count what was taken back
            }
            break;
        case lc::Action::ContainerSet: applyContents(a.id, a.items); break;
        case lc::Action::CloseMenu:
            if (menuNet_ != a.id && implicit_.count(a.id) && !FNV_MUT(162)) break;   // twin 162: a refused menu-less transfer closes whatever menu is up (there is none of ours to close)
            if (menuNet_ == a.id) suppressClose_.insert(a.id);
            // The real engine cannot close the menu from code yet (TODO-RE): the player then sits in a menu the host refused (busy, lock lost) and goes on taking.
            // Those moves are never reported (the lock is not ours), so they are taken back here, every poll, until the player closes the menu (twin 167).
            if (!eng_.closeMenu() && menuNet_ == a.id && !FNV_MUT(167)) {
                blockedMenu_.insert(a.id);
                if (const lc::ContainerRec* rec = client_->container(a.id)) applyContents(a.id, rec->items);   // from here on every difference is the player's move
            }
            break;
        case lc::Action::Notify: eng_.notify(a.notice, a.id); break;
        case lc::Action::BindDrop: eng_.bindItem(a.ref, a.id); break;
        case lc::Action::CreateItem:
            if (eng_.createItem(a.item) == 0) {
                if (creates_.size() < cfg_.maxPendingCreates) {
                    creates_.push_back(a.item);
                    createsDue_.push_back(now_ + cfg_.createRetryMs);
                    ++stats_.createsDeferred;
                } else {
                    ++stats_.createsDropped;
                }
            }
            break;
        case lc::Action::RemoveItem: {
            ++stats_.removesRun;
            if (!a.placed.plugin.empty()) ++stats_.removesPlaced;
            // twin 174: the engine gets the id only. false: no such ref here (already gone, or its cell is not loaded)
            if (eng_.removeItem(a.id, FNV_MUT(174) ? FormRef() : a.placed)) ++stats_.removesDone;
            else {
                ++stats_.removesGone;
                // A placed item whose cell is not loaded here has no ref to remove yet: it would be back on the floor when the cell loads. Ask again later.
                if (!a.placed.plugin.empty() && !FNV_MUT(176)) {   // twin 176: a placed item that was not here at that moment stays in the world for good
                    if (removes_.size() < cfg_.maxPendingRemoves) {
                        PendingRemove p;
                        p.id = a.id; p.placed = a.placed; p.due = now_ + cfg_.removeRetryMs;
                        removes_.push_back(std::move(p));
                        ++stats_.removesDeferred;
                    } else {
                        ++stats_.removesDropped;
                    }
                }
            }
            for (size_t i = 0; i < creates_.size();) {
                if (creates_[i].id == a.id) { creates_.erase(creates_.begin() + static_cast<long>(i)); createsDue_.erase(createsDue_.begin() + static_cast<long>(i)); }
                else ++i;
            }
            break;
        }
        case lc::Action::SetPose: eng_.setPose(a.id, a.pos, a.rot, a.flag); break;
        case lc::Action::ReadyToClose: {
            Items items;
            bool exact = true;
            if (!eng_.readContents(a.id, items, exact)) { ++stats_.readFailures; items = Items(); }
            client_->closeNow(a.id, items, now_);
            break;
        }
        case lc::Action::ClutterOwner:
            if (a.flag) ownedClutter_.insert(a.id); else ownedClutter_.erase(a.id);
            break;
        }
    }
}

void LootSync::drain() {
    if (!client_) return;
    for (int guard = 0; guard < 8; ++guard) {
        std::vector<lc::Action> acts = client_->takeActions();
        if (acts.empty()) return;
        run(acts);
        poll();
    }
}

// ================================================================================================================ time

void LootSync::idleTick(uint64_t nowMs) { now_ = nowMs; }

void LootSync::tick(uint64_t nowMs) {
    if (!link_ || !client_) return;
    now_ = nowMs;
    ++stats_.ticks;
    if (!eng_.worldReady()) return;
    pump();
    pollMenu(nowMs);
    drain();
    client_->tick(nowMs);
    drain();
    if (host_) {
        lc::LootHost::Out out;
        host_->tick(nowMs, out);
        deliver(out);
        pump();
    }
    tickDiscover(nowMs);
    tickResolve(nowMs);
    tickCreates(nowMs);
    tickRemoves(nowMs);
    tickTransfers(nowMs);
    tickSweep(nowMs);
    tickAudit(nowMs);
    tickClutter(nowMs);
    pump();
}

void LootSync::onContainerLoaded(RefId ref) {
    if (!client_ || !cfg_.resolveOnLoad || !ref) return;
    Resolve r;
    r.ref = ref; r.due = now_;
    resolves_.push_back(r);
}

void LootSync::onActorDied(NetId actor, RefId ref) {
    if (!client_ || !ref) return;
    Resolve r;
    r.ref = ref; r.corpse = true; r.actor = actor; r.due = now_ + cfg_.corpseDelayMs;
    resolves_.push_back(r);
}

// The containers of the player's cell: whoever stands next to a chest nobody opened before has to know it, or its menu-less takes pass unseen (a guest took 1 of 3
// from a placed chest: the guest read 2, the host 3, and the guest's service listed 0 containers). Each ref is queued once per world; the resolve is the same one a
// cell attach would have started ("first resolver wins", a machine that does not own the cell waits for the owner's real items).
void LootSync::tickDiscover(uint64_t now) {
    if (!client_ || !cfg_.discover || FNV_MUT(184) || now < nextDiscover_) return;   // twin 184: unknown containers are never looked for
    nextDiscover_ = now + cfg_.discoverEveryMs;
    std::vector<RefId> refs;
    eng_.nearbyContainers(refs, 256);
    if (cfg_.discoverCells > 0 && !FNV_MUT(185)) {   // twin 185: only the player's own cell is looked at
        // the loaded neighbour cells of an exterior grid, a few per pass in rotation (a pass is bounded: each cell is a walk over its whole object list)
        std::vector<lc::CellId> cells;
        eng_.nearbyCells(cells, 64);
        if (cells.size() > 1) {
            const size_t others = cells.size() - 1;   // cells[0] is the player's own cell: nearbyContainers answered it
            const size_t take = std::min<size_t>(cfg_.discoverCells, others);
            for (size_t k = 0; k < take && refs.size() < cfg_.discoverMaxRefs; ++k) {
                const size_t idx = 1 + (discoverCursor_ + k) % others;
                std::vector<RefId> more;
                eng_.containersInCell(cells[idx], more, 256);
                refs.insert(refs.end(), more.begin(), more.end());
            }
            if (!FNV_MUT(186)) discoverCursor_ = (discoverCursor_ + take) % others;   // twin 186: the rotation never moves on (the same first neighbours every pass)
        }
    }
    for (RefId r : refs) {
        if (!r || !discovered_.insert(r).second) continue;
        onContainerLoaded(r);
        ++stats_.containersDiscovered;
    }
    if (discovered_.size() > 4096) discovered_.clear();
}

void LootSync::tickResolve(uint64_t now) {
    size_t budget = 8;
    for (size_t n = resolves_.size(); n > 0 && budget > 0; --n) {
        Resolve r = resolves_.front();
        resolves_.pop_front();
        if (r.due > now) { resolves_.push_back(r); continue; }
        --budget;
        ContainerDesc d;
        if (!eng_.describeContainer(r.ref, d) || !d.net) continue;
        if (r.corpse) d.kind = lc::ContainerKind::Corpse;
        if (client_->container(d.net) != nullptr) continue;   // somebody resolved it already: ours is replaced when we open it
        if (!r.corpse && !r.deferredForOwner && ownsCell_ && !ownsCell_(d.cell)) {   // not the cell's owner: give the owner's real items time to arrive first
            r.deferredForOwner = true;
            r.due = now + 3000;
            resolves_.push_back(r);
            continue;
        }
        Items items;
        if (!readExact(d.net, items)) {
            if (++r.tries < 6) { r.due = now + cfg_.resolveRetryMs; resolves_.push_back(r); ++stats_.resolveRetries; }
            continue;
        }
        client_->resolved(d.net, d.kind, d.cell, d.pos, items, now);
        if (cfg_.menuless) { Items b = items; lc::normalize(b); engineBase_[d.net] = b; }
        seedPlayer(d.net, items);
        if (r.corpse) ++stats_.corpsesResolved; else ++stats_.containersResolved;
    }
}

void LootSync::tickCreates(uint64_t now) {
    if (FNV_MUT(154)) { creates_.clear(); createsDue_.clear(); return; }   // twin 154: a remote drop whose cell was not loaded is forgotten
    for (size_t i = 0; i < creates_.size();) {
        if (createsDue_[i] > now) { ++i; continue; }
        if (eng_.createItem(creates_[i]) != 0) {
            creates_.erase(creates_.begin() + static_cast<long>(i));
            createsDue_.erase(createsDue_.begin() + static_cast<long>(i));
        } else {
            createsDue_[i] = now + cfg_.createRetryMs;
            ++i;
        }
    }
}

void LootSync::tickRemoves(uint64_t now) {
    for (size_t i = 0; i < removes_.size();) {
        PendingRemove& p = removes_[i];
        if (p.due > now) { ++i; continue; }
        ++stats_.removesRetried;
        if (eng_.removeItem(p.id, p.placed)) {
            ++stats_.removesDone;
            removes_.erase(removes_.begin() + static_cast<long>(i));
        } else if (++p.tries >= cfg_.maxRemoveTries && !FNV_MUT(177)) {   // twin 177: a ref that never shows up is asked about for ever
            ++stats_.removesDropped;
            removes_.erase(removes_.begin() + static_cast<long>(i));
        } else {
            p.due = now + cfg_.removeRetryMs;
            ++i;
        }
    }
}

void LootSync::tickSweep(uint64_t now) {
    if (!host_ || !cfg_.hostSweep || FNV_MUT(147) || now < nextSweep_) return;   // twin 147: scripts' changes to a container never reach the table
    nextSweep_ = now + cfg_.sweepEveryMs;
    std::vector<NetId> ids;
    scanIds(ids);
    if (ids.empty()) return;
    for (uint32_t n = 0; n < cfg_.sweepPerTick && n < ids.size(); ++n) {
        const NetId id = ids[(sweepCursor_ + n) % ids.size()];
        const lc::ContainerRec* c = host_->container(id);
        if (!c || c->holder != lc::kNoPlayer || id == menuNet_) continue;
        Items items;
        if (!readExact(id, items)) continue;
        if (screenTransfer(id, items)) continue;   // the local player's own menu-less take / put is not a script's change
        ++stats_.sweepsCompared;
        if (lc::digestOf(items) == lc::digestOf(c->items)) continue;
        lc::LootHost::Out out;
        if (host_->externalSet(id, items, now, out)) ++stats_.sweepsChanged;
        deliver(out);
    }
    sweepCursor_ = (sweepCursor_ + cfg_.sweepPerTick) % ids.size();
}

void LootSync::tickAudit(uint64_t now) {
    if (host_) return;   // the host's engine is the authority for what scripts do (tickSweep carries it into the table); it is never corrected from the table
    if (FNV_MUT(148) || now < nextAudit_) return;   // twin 148: engine copies are never compared with the shared contents
    nextAudit_ = now + cfg_.auditEveryMs;
    std::vector<NetId> ids;
    scanIds(ids);
    if (ids.empty()) return;
    for (uint32_t n = 0; n < cfg_.auditPerTick && n < ids.size(); ++n) {
        const NetId id = ids[(auditCursor_ + n) % ids.size()];
        if (id == menuNet_ || !client_->container(id)) continue;
        Items items;
        if (!readExact(id, items)) continue;
        if (screenTransfer(id, items)) continue;   // the local player's own menu-less take / put is not a stray difference to correct
        ++stats_.auditsCompared;
        client_->audit(id, items, now);
    }
    auditCursor_ = (auditCursor_ + cfg_.auditPerTick) % ids.size();
    drain();
}

void LootSync::tickClutter(uint64_t now) {
    if (FNV_MUT(155) || ownedClutter_.empty() || now < nextClutter_) return;   // twin 155: owned clutter is never sampled (it never moves on anybody else's screen)
    nextClutter_ = now + cfg_.clutterEveryMs;
    std::vector<lc::LootClient::Sample> samples;
    for (NetId id : ownedClutter_) {
        lc::LootClient::Sample s;
        s.net = id;
        if (eng_.readPose(id, s.pos, s.rot)) samples.push_back(s);
    }
    if (!samples.empty()) client_->clutterSamples(samples, now);
}

// ================================================================================================================ menu-less transfers

// A take or put the engine did on a CLOSED container without the container menu (console RemoveItem + AddItem, a quick-loot mod, a script of the
// player's). The menu watcher never sees it, so it used to bypass the lock: on the host the sweep adopted it as a script's change, on a guest the
// audit threw the container back while the player kept the items; two players doing it at once both kept everything.
// Now the difference between the engine copy and the shared contents is attributed to the local player when the player's own inventory moved the
// other way by that much, and then goes through exactly the menu path: open (lock), take / put, close. The host's answer decides; a refusal
// takes the items back from the player (Action::PlayerAdjust) and puts the shared contents back (Action::ContainerSet).

void LootSync::seedPlayer(NetId net, const Items& shared) {
    if (!cfg_.menuless) return;
    if (playerSeen_.size() > 4096) playerSeen_.clear();
    std::map<FormRef, int32_t>& m = playerSeen_[net];
    m.clear();
    for (const auto& kv : shared) {
        int32_t n = 0;
        if (eng_.playerCount(kv.first, n)) m[kv.first] = n;
    }
}

void LootSync::shiftPlayerSeen(const FormRef& base, int32_t delta, NetId except) {
    for (auto& c : playerSeen_) {
        if (c.first == except) continue;
        auto f = c.second.find(base);
        if (f != c.second.end()) f->second += delta;
    }
}

// The containers worth comparing, sorted: the ones the engine lists as loaded and every container of the shared table that has a ref on this machine.
// (The real engine lists only containers it described before: a corpse somebody else's death registered would never be looked at on this machine, and its
// menu-less takes would pass unseen.)
void LootSync::scanIds(std::vector<NetId>& ids) {
    eng_.loadedContainers(ids, 512);
    if (client_ && !FNV_MUT(163)) {   // twin 163: only containers the engine already described are looked at
        std::vector<NetId> known;
        client_->containerIds(known, 512);
        std::sort(ids.begin(), ids.end());
        for (NetId id : known)
            if (!std::binary_search(ids.begin(), ids.end(), id) && eng_.refOfContainer(id) != 0) ids.push_back(id);
    }
    std::sort(ids.begin(), ids.end());
    ids.erase(std::unique(ids.begin(), ids.end()), ids.end());
}

bool LootSync::screenTransfer(NetId id, const Items& engineItems) {
    if (!cfg_.menuless || !client_ || FNV_MUT(158)) return false;   // twin 158: menu-less transfers are never looked for (the old behaviour: they bypass the lock)
    const lc::ContainerRec* rec = client_->container(id);
    if (!rec || id == menuNet_) return false;
    const Items shared = rec->items;
    Items have = engineItems;
    lc::normalize(have);
    // What the engine copy is compared with: what it held when it last matched a shared state. NOT the shared contents as they are now: a change the others
    // made (their take, applied here before this machine's scan ran) would otherwise make the local player's own, still unattributed take look like "the
    // engine already shows the shared contents" and the player would keep the items (twin 164).
    auto bi = engineBase_.find(id);
    const bool hasBase = bi != engineBase_.end() && !FNV_MUT(164);
    const Items ref = hasBase ? bi->second : shared;
    if (lc::digestOf(have) == lc::digestOf(ref)) {
        suspect_.erase(id);
        if (lc::digestOf(have) == lc::digestOf(shared) && client_->transferIdle(id)) {
            implicit_.erase(id);
            seedPlayer(id, shared);
            if (cfg_.menuless) engineBase_[id] = shared;
        }
        return false;
    }
    ++stats_.transferDiffs;
    if (!client_->transferIdle(id)) return true;   // a menu or a transfer of this container is under way: it owns the engine copy
    const lc::ContainerKind kind = rec->kind;
    const lc::CellId cell = rec->cell;
    const lc::Vec3 pos = rec->pos;
    Items settled = ref;                            // the reference with the explained differences applied
    bool any = false;
    auto seenIt = playerSeen_.find(id);
    for (const lc::ItemDiff& x : lc::diffItems(ref, have)) {
        if (seenIt == playerSeen_.end()) break;
        auto seen = seenIt->second.find(x.base);
        int32_t now = 0;
        if (seen == seenIt->second.end()) continue;
        if (!eng_.playerCount(x.base, now)) { ++stats_.playerCountFails; continue; }
        const bool take = x.b < x.a;
        const int32_t want = take ? x.a - x.b : x.b - x.a;                       // what the container lost / gained
        int32_t moved = take ? now - seen->second : seen->second - now;          // what the player gained / gave up since the container last matched
        if (FNV_MUT(159)) moved = want;                                          // twin 159: any difference is believed to be the player's (a script's change becomes a take)
        const int32_t n = std::min(want, moved);
        if (n <= 0) continue;
        any = true;
        settled[x.base] = take ? x.a - n : x.a + n;
        if (settled[x.base] <= 0) settled.erase(x.base);
        // the player's move is used up: no other container may claim it, and this one now stands at the player's count
        if (!FNV_MUT(161)) shiftPlayerSeen(x.base, take ? n : -n, id);   // twin 161: an explained move explains every other container's difference too
        seen->second = now;
    }
    if (!any) {
        auto s = suspect_.find(id);
        if (s == suspect_.end()) { suspect_[id] = now_; ++stats_.transferSuspects; s = suspect_.find(id); }
        // twin 160: no grace: the AddItem that follows a RemoveItem comes too late and the sweep adopts the RemoveItem as a script's change
        if (!FNV_MUT(160) && now_ - s->second < cfg_.transferGraceMs) return true;
        // the grace is over and nothing of the player's explains it: a script's change. The old handling takes over (host sweep adopts it, guest audit
        // corrects it) and the reference is dropped so that nothing is held back any more.
        if (hasBase) { engineBase_.erase(id); ++stats_.transferGraceExpired; }
        return false;
    }
    suspect_.erase(id);
    implicit_.insert(id);
    ++stats_.transfersMenuless;
    if (cfg_.menuless) engineBase_[id] = settled;
    client_->opened(id, kind, cell, pos, ref, now_);
    client_->observed(id, settled, now_);
    client_->closed(id, settled, now_);
    drain();
    return true;
}

void LootSync::tickTransfers(uint64_t now) {
    if (!client_ || !cfg_.menuless || FNV_MUT(158) || now < nextTransfer_) return;
    nextTransfer_ = now + cfg_.transferEveryMs;
    std::vector<NetId> ids;
    scanIds(ids);
    stats_.scanKnown = ids.size();
    if (ids.empty()) return;
    auto look = [&](NetId id) {
        if (!client_ || !client_->container(id) || id == menuNet_) return;
        Items items;
        if (!readExact(id, items)) return;
        ++stats_.transferScans;
        screenTransfer(id, items);
    };
    std::vector<NetId> sus;
    for (const auto& kv : suspect_) sus.push_back(kv.first);
    for (NetId id : sus) look(id);                  // differences seen before are looked at again every pass
    for (uint32_t n = 0; n < cfg_.transferPerTick && n < ids.size(); ++n) look(ids[(transferCursor_ + n) % ids.size()]);
    transferCursor_ = (transferCursor_ + cfg_.transferPerTick) % ids.size();
    pump();
}

// ================================================================================================================ items

void LootSync::onDrop(RefId dropper, RefId dropped) {
    if (!client_ || !dropped) return;
    ++stats_.dropsSeen;
    if (dropper != kPlayerRef && !FNV_MUT(150)) { ++stats_.dropsIgnored; return; }   // twin 150: any actor's drop is announced as the local player's (and takes the item from that player)
    ItemDesc d;
    if (!eng_.describeItem(dropped, d) || d.item.base.plugin.empty() || d.item.count <= 0) { ++stats_.dropsIgnored; return; }
    client_->dropped(dropped, d.item.base, d.item.count, d.item.meta, d.item.cell, d.item.pos, d.item.rot, now_);
    pump();
}

void LootSync::onPickup(RefId picker, RefId item) {
    if (!client_ || !item) return;
    ++stats_.pickupsSeen;
    if (picker != kPlayerRef && !FNV_MUT(151)) { ++stats_.pickupsIgnored; return; }   // twin 151: an NPC's pickup is claimed for the local player
    ItemDesc d;
    if (!eng_.describeItem(item, d) || d.item.base.plugin.empty()) { ++stats_.pickupsIgnored; return; }
    ++stats_.pickupsClaimed;
    client_->pickedUp(item, d.net, d.item.base, d.item.count, d.item.meta, d.item.cell, d.item.pos, now_, FNV_MUT(175) ? FormRef() : d.placedRef);   // twin 175: the claim does not name the ref
    pump();
}

void LootSync::onTouch(RefId ref) {
    if (!client_) return;
    ItemDesc d;
    if (!eng_.describeItem(ref, d) || !d.net) return;
    lc::Vec3 pos, rot;
    if (!eng_.readPose(d.net, pos, rot)) { pos = d.item.pos; rot = d.item.rot; }
    client_->touch(d.net, pos, rot, now_);
    pump();
}

void LootSync::onCellReset(lc::CellId cell) {
    if (!host_) return;
    lc::LootHost::Out out;
    host_->resetCell(cell, out);
    deliver(out);
    pump();
}

// ================================================================================================================ sidecar and status

std::vector<uint8_t> LootSync::saveState() const { return host_ ? host_->save() : pendingSidecar_; }

bool LootSync::loadState(const std::vector<uint8_t>& bytes) {
    if (host_) return host_->load(bytes);
    pendingSidecar_ = bytes;
    return true;
}

std::string LootSync::statusJson() const {
    char b[3200];
    const lc::ClientStats* cs = client_ ? &client_->stats() : nullptr;
    const lc::HostStats* hs = host_ ? &host_->stats() : nullptr;
    std::snprintf(b, sizeof b,
                  "{\"enabled\":%s,\"session\":%s,\"host\":%s,\"link\":%s,\"openContainer\":%u,\"containers\":%zu,\"items\":%zu,\"pending\":%zu,"
                  "\"digest\":\"%016llx\",\"hostDigest\":\"%016llx\",\"tick\":{\"in\":%llu,\"out\":%llu,\"undecodable\":%llu,\"spoofed\":%llu,\"polls\":%llu,"
                  "\"actions\":%llu,\"actionsFailed\":%llu,\"rollbacks\":%llu,\"resolved\":%llu,\"corpses\":%llu,\"createsDeferred\":%llu,\"sweepsChanged\":%llu},"
                  "\"client\":{\"rollbacks\":%llu,\"busy\":%llu,\"claimsLost\":%llu,\"resent\":%llu,\"gaps\":%llu,\"tables\":%llu,\"closeMismatch\":%llu,\"audit\":%llu},"
                  "\"hostStats\":{\"requests\":%llu,\"replays\":%llu,\"busy\":%llu,\"closeMismatch\":%llu,\"claimsWon\":%llu,\"claimsLost\":%llu,\"heals\":%llu},"
                  "\"pickups\":{\"activates\":%llu,\"byPlayer\":%llu,\"seen\":%llu,\"ignored\":%llu,\"claimed\":%llu,\"claims\":%llu,\"duplicate\":%llu,\"won\":%llu,\"lost\":%llu,"
                  "\"removesRun\":%llu,\"removesDone\":%llu,\"removesGone\":%llu,\"removesPlaced\":%llu,\"hostWon\":%llu,\"hostLost\":%llu,\"hostRepeated\":%llu},"
                  "\"menuless\":{\"enabled\":%s,\"known\":%llu,\"scans\":%llu,\"seen\":%llu,\"suspects\":%llu,\"arbitrated\":%llu,\"graceExpired\":%llu,"
                  "\"setsDeferred\":%llu,\"countFails\":%llu,\"reverted\":%llu,\"revertedItems\":%llu,\"refused\":%llu,\"busy\":%llu,\"rollbackItems\":%llu}}",
                  cfg_.enabled ? "true" : "false", link_ ? "true" : "false", host_ ? "true" : "false", linkUp_ ? "true" : "false", menuNet_,
                  client_ ? client_->containerCount() : 0, client_ ? client_->itemCount() : 0, client_ ? client_->pendingCount() : 0,
                  (unsigned long long)(client_ ? client_->digest() : 0), (unsigned long long)(host_ ? host_->digest() : 0),
                  (unsigned long long)stats_.datagramsIn, (unsigned long long)stats_.datagramsOut, (unsigned long long)stats_.undecodable,
                  (unsigned long long)stats_.spoofed, (unsigned long long)stats_.polls, (unsigned long long)stats_.actionsRun,
                  (unsigned long long)stats_.actionsFailed, (unsigned long long)stats_.rollbacksApplied, (unsigned long long)stats_.containersResolved,
                  (unsigned long long)stats_.corpsesResolved, (unsigned long long)stats_.createsDeferred, (unsigned long long)stats_.sweepsChanged,
                  (unsigned long long)(cs ? cs->rollbacks : 0), (unsigned long long)(cs ? cs->busy : 0), (unsigned long long)(cs ? cs->claimsLost : 0),
                  (unsigned long long)(cs ? cs->resent : 0), (unsigned long long)(cs ? cs->gaps : 0), (unsigned long long)(cs ? cs->tablesApplied : 0),
                  (unsigned long long)(cs ? cs->closeMismatch : 0), (unsigned long long)(cs ? cs->auditCorrections : 0),
                  (unsigned long long)(hs ? hs->requests : 0), (unsigned long long)(hs ? hs->replays : 0), (unsigned long long)(hs ? hs->busy : 0),
                  (unsigned long long)(hs ? hs->closeMismatch : 0), (unsigned long long)(hs ? hs->claimsWon : 0), (unsigned long long)(hs ? hs->claimsLost : 0),
                  (unsigned long long)(hs ? hs->healsSent : 0),
                  (unsigned long long)stats_.activateEvents, (unsigned long long)stats_.activateByPlayer, (unsigned long long)stats_.pickupsSeen,
                  (unsigned long long)stats_.pickupsIgnored, (unsigned long long)stats_.pickupsClaimed, (unsigned long long)(cs ? cs->claims : 0),
                  (unsigned long long)(cs ? cs->claimsDuplicate : 0), (unsigned long long)(cs ? cs->claimsWon : 0), (unsigned long long)(cs ? cs->claimsLost : 0),
                  (unsigned long long)stats_.removesRun, (unsigned long long)stats_.removesDone, (unsigned long long)stats_.removesGone,
                  (unsigned long long)stats_.removesPlaced, (unsigned long long)(hs ? hs->claimsWon : 0), (unsigned long long)(hs ? hs->claimsLost : 0),
                  (unsigned long long)(hs ? hs->claimsRepeated : 0),
                  cfg_.menuless ? "true" : "false", (unsigned long long)stats_.scanKnown, (unsigned long long)stats_.transferScans, (unsigned long long)stats_.transferDiffs,
                  (unsigned long long)stats_.transferSuspects, (unsigned long long)stats_.transfersMenuless, (unsigned long long)stats_.transferGraceExpired,
                  (unsigned long long)stats_.setsDeferred, (unsigned long long)stats_.playerCountFails, (unsigned long long)stats_.rollbacksApplied,
                  (unsigned long long)stats_.revertedItems, (unsigned long long)(cs ? cs->rollbacks : 0), (unsigned long long)(cs ? cs->busy : 0),
                  (unsigned long long)(cs ? cs->rollbackItems : 0));
    return b;
}

}  // namespace nvcoop::loot
