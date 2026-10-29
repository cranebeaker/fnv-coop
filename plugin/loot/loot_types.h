// plugin/loot: loot, containers, dropped items and physics clutter inside the plugin (FULL-SYNC-PLAN 3.5). Shared types and the engine port.
// Game-free. The engine is reached only through ILootEngine (in-game: loot_engine_real.cpp over xNVSE; tests: a fake world), the network only
// through the live hub's ILiveLink (plugin/coop/live/live_hub.h; tests: an in-memory link).
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include <vector>
#include "loot/loot.h"

namespace nvcoop::loot {

namespace lc = fnvcoop::loot;
using lc::Items;
using lc::NetId;
using fnvcoop::FormRef;
using fnvcoop::PlayerId;
using fnvcoop::kHostPlayer;
using fnvcoop::kNoPlayer;

using RefId = uint32_t;                 // runtime form id of a loaded reference (xNVSE refID); 0 = none
constexpr RefId kPlayerRef = 0x14;
constexpr uint8_t kChannelLoot = lc::kChannelLoot;

struct ContainerDesc {
    NetId net = 0;                      // 0 = not a syncable container (not lootable, or no identity can be derived)
    lc::ContainerKind kind = lc::ContainerKind::Container;
    lc::CellId cell = 0;
    lc::Vec3 pos;
    bool uniqueActor = false;           // a corpse of a unique (named) character: its kit is hand-written, scarcity leaves it alone (docs/rebalance.md)
};

// What an item is, for the scarcity rule of plugin/rebalance (supplies: food, drink, medicine; ammo).
enum class ItemClass : uint8_t { Other, Ammo, Supply };

// A scarcity hook (plugin/rebalance, Co-op balanced): called ONCE for every container or corpse this machine registers, with the real contents it read, BEFORE they become
// the shared ones. It may remove items from `items` and returns true when it did; LootSync then makes the engine copy match, so every machine ends with the same list.
struct TrimContext {
    NetId net = 0;
    lc::ContainerKind kind = lc::ContainerKind::Container;
    bool uniqueActor = false;
    std::function<ItemClass(const FormRef&)> classify;     // what an item is (the engine knows its form)
};
using TrimHook = std::function<bool(const TrimContext&, Items&)>;

struct ItemDesc {
    NetId net = 0;                      // placed: derived from the ref. dropped: the issued id it was bound to. 0 = unknown / not an item
    lc::WorldItem item;                 // base, count, meta, cell, pos, rot (id = net)
    bool boundDrop = false;             // a dropped item that carries an issued id
    FormRef placedRef;                  // a placed item: the world ref itself (plugin:localId); empty for a dropped / unknown one
};

class ILootEngine {
public:
    virtual ~ILootEngine() = default;
    virtual bool worldReady() = 0;                                              // a game is loaded (false in menus / loading)
    virtual uint32_t worldEpoch() = 0;                                          // changes on every load / new game

    // ---- containers and corpses ----
    virtual bool describeContainer(RefId ref, ContainerDesc& out) = 0;          // false: not a lootable container / corpse
    virtual RefId refOfContainer(NetId net) = 0;                                // loaded ref of a known container; 0 = not loaded here
    // The engine copy's contents as TOTALS (base container + changes). exact=false: the read is partial (an unresolved leveled list is in it, or
    // the totals call was not available): such a read is never offered as the shared contents.
    virtual bool readContents(NetId net, Items& out, bool& exact) = 0;
    virtual bool setContents(NetId net, const Items& want) = 0;                 // make the engine copy hold exactly `want` (diff, then add / remove)
    virtual ItemClass classifyItem(const FormRef& base) { (void)base; return ItemClass::Other; }   // ammo / food, drink, medicine / anything else
    virtual RefId openMenuRef() = 0;                                            // the container ref whose menu is up right now; 0 = no container menu
    virtual bool closeMenu() = 0;                                               // close the container menu (TODO-RE in the real engine)
    virtual bool adjustPlayer(const FormRef& base, int32_t delta) = 0;          // add / remove items of the LOCAL player; false when it could not
    // How many of `base` the LOCAL player carries right now. false: unknown (menu-less transfers are then not attributed to the player and are
    // handled as a script's change, the way they were before the menu-less path existed).
    virtual bool playerCount(const FormRef& base, int32_t& out) { (void)base; out = 0; return false; }
    virtual void notify(lc::Notice n, NetId id) = 0;                            // a short message to the player
    virtual void loadedContainers(std::vector<NetId>& out, size_t max) = 0;     // containers with a loaded ref here (audit / host sweep), at most `max`
    // The container references (chests, lockers: not corpses) in the cell the local player stands in, at most `max`. Nothing tells the service when a cell attaches, so it
    // asks: a container nobody ever opened (a placed chest) is otherwise unknown here and a take from it is invisible to every scan (story fix 2026-10-28).
    virtual void nearbyContainers(std::vector<RefId>& out, size_t max) { (void)max; out.clear(); }
    // The attached cells around the player, the player's own cell first and then the loaded neighbours of an exterior grid (an interior has none), at most `max`. Each is
    // asked for its containers with containersInCell (same rule as nearbyContainers). Follow-ups 1 item 4: a chest in the cell next door is as lootable as one in the player's own.
    virtual void nearbyCells(std::vector<lc::CellId>& out, size_t max) { (void)max; out.clear(); }
    virtual void containersInCell(lc::CellId cell, std::vector<RefId>& out, size_t max) { (void)cell; (void)max; out.clear(); }

    // ---- world items ----
    virtual bool describeItem(RefId ref, ItemDesc& out) = 0;
    virtual RefId createItem(const lc::WorldItem& it) = 0;                      // make a world ref for a remote drop; 0 = failed (cell not loaded: retried by the caller)
    virtual void bindItem(RefId ref, NetId net) = 0;                            // the local world ref IS item `net`
    // Delete the world ref of item `net` (it was taken). A PLACED item (a ref of a plugin) is found from `placed` (plugin:localId): its net id is a hash
    // of that and nothing maps it back, and the actor registry only knows actors. A dropped item is found from `net`. false: no such ref here
    // (its cell is not loaded: the caller asks again). true: removed, or already gone.
    virtual bool removeItem(NetId net, const FormRef& placed) = 0;
    // A NOTE (BGSNote, form type 0x31) is never an inventory item: picking one up adds it to the Pip-Boy journal. G10 / P4 interface (docs/handover/P4-requests.md 6).
    virtual bool isNoteBase(const FormRef& base) { (void)base; return false; }
    // Host: the host's journal gains `note` (the console's AddNote, inside a capture scope so the engine's AddNote hook records the shared effect; never record it by hand as well).
    virtual bool addNote(const FormRef& note) { (void)note; return false; }

    // ---- clutter ----
    virtual bool readPose(NetId net, lc::Vec3& pos, lc::Vec3& rot) = 0;
    virtual bool setPose(NetId net, const lc::Vec3& pos, const lc::Vec3& rot, bool moving) = 0;
};

struct SyncConfig {
    bool enabled = false;
    uint32_t pollMs = 50;               // open container menu: engine copy read this often
    uint32_t corpseDelayMs = 700;       // after an actor died: its inventory has settled, register the corpse
    bool discover = true;               // look for unknown containers in the player's cell and resolve them (onContainerLoaded has no caller in a real game)
    uint32_t discoverEveryMs = 1500;    // ... this often
    uint32_t discoverCells = 3;         // ... and this many of the loaded neighbour cells of an exterior grid per pass, in rotation (0 = the player's own cell only)
    size_t discoverMaxRefs = 512;       // ... at most this many container references handed to the resolver per pass, the own cell and the neighbours together
    uint32_t resolveRetryMs = 1500;     // a container whose read was not exact is tried again after this long
    uint32_t sweepPerTick = 6;          // host: containers compared with the table per tick (scripts change them)
    uint32_t sweepEveryMs = 500;
    uint32_t auditPerTick = 6;          // every machine: engine copies of closed containers compared with the shared contents
    uint32_t auditEveryMs = 500;
    uint32_t clutterEveryMs = 66;       // owned clutter is sampled this often
    uint32_t createRetryMs = 1000;      // a remote drop whose cell is not loaded is retried
    uint32_t maxPendingCreates = 256;
    uint32_t removeRetryMs = 2000;      // a taken placed item whose ref is not here (its cell is not loaded yet) is removed when it loads
    uint32_t maxPendingRemoves = 4096;
    uint32_t maxRemoveTries = 900;      // ... asked this often at most (about 30 minutes), then given up
    bool resolveOnLoad = true;          // register containers when their cell loads (the owner's roll becomes the shared one)
    bool hostSweep = true;
    // Menu-less transfers: a take / put the engine did on a CLOSED container without the container menu (a console RemoveItem + AddItem, a quick-loot
    // style mod, a script of the player's). Without this the move never reaches the lock / claim rules and two players can both keep the same items.
    // A difference between a closed container's engine copy and the shared contents that the local player's own inventory explains is turned into an
    // open / take / close of that container (the host's answer decides, a refusal takes the items back from the player).
    bool menuless = true;
    uint32_t transferEveryMs = 50;      // closed containers are compared this often ...
    uint32_t transferPerTick = 8;       // ... this many per pass, round-robin (containers already seen to differ are looked at every pass)
    uint32_t transferGraceMs = 400;     // a difference the player's inventory does not (yet) explain is left alone this long (the AddItem may follow) before it is a script's change
    lc::HostConfig host;
    lc::ClientConfig client;
};

struct Stats {
    uint64_t ticks = 0, datagramsIn = 0, datagramsOut = 0, undecodable = 0, spoofed = 0, wrongDirection = 0;
    uint64_t menusOpened = 0, menusClosed = 0, polls = 0, readFailures = 0, inexactReads = 0;
    uint64_t actionsRun = 0, actionsFailed = 0, rollbacksApplied = 0;
    uint64_t containersResolved = 0, corpsesResolved = 0, resolveRetries = 0;
    uint64_t dropsSeen = 0, dropsIgnored = 0, pickupsSeen = 0, pickupsIgnored = 0, createsDeferred = 0, createsDropped = 0;
    uint64_t sweepsCompared = 0, sweepsChanged = 0, auditsCompared = 0;
    uint64_t transferScans = 0, transfersMenuless = 0, transferSuspects = 0;
    uint64_t transferDiffs = 0;          // looks at a closed container whose engine copy differed from what it last matched (the transfer detector SAW something)
    uint64_t transferGraceExpired = 0;   // ... of which the player's inventory never explained: handed to the old script handling
    uint64_t setsDeferred = 0;           // eager writes of the shared contents NOT done because the engine copy had moved on its own (a take waiting to be attributed)
    uint64_t playerCountFails = 0;       // playerCount() could not answer while attributing (no attribution is possible then)
    uint64_t revertedItems = 0;          // items taken back from / given back to the local player by a refused transfer (sum of |delta|)
    uint64_t scanKnown = 0;              // containers looked at by the last transfer pass (engine list + shared table entries with a ref here)
    uint64_t sessions = 0, loads = 0;
    uint64_t trimmedContainers = 0, trimmedItems = 0;   // scarcity: containers and corpses the hook thinned / items it took out
    uint64_t containersDiscovered = 0;   // containers found in the player's cell by tickDiscover (not announced by a cell attach)
    uint64_t activateEvents = 0, activateByPlayer = 0;   // OnActivate events the module saw / of those, the ones the player did (the pickup detector's input)
    uint64_t pickupsClaimed = 0;                         // pickups handed to the claim rules (a world item the player took)
    uint64_t removesRun = 0, removesDone = 0, removesGone = 0, removesPlaced = 0, removesDeferred = 0, removesRetried = 0, removesDropped = 0, notesGranted = 0, notesRefused = 0;   // notesGranted: a won claim of a note added to the host journal   // world refs removed because somebody else took the item: asked / done / ref not here (already gone) / of which placed
};

}  // namespace nvcoop::loot
