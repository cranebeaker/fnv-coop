// Party saves (docs/party-save.md): when the HOST saves, every guest saves its OWN game at the same moment, into a save named after the campaign and
// the host save. Game-free: names, the sidecar and index texts, the orders, and the two small state machines (HostParty, GuestParty) that the engine
// glue (party_save_glue.cpp) drives with the real save calls. Everything the machines need from the game comes in through an Env of std::function,
// so plugin/coop/tests/test_party_save.cpp runs them against fakes and against the real session's order channel.
//
// Orders (kAppOrderChannel JSON, the same channel as the stash / kill orders in coop_module.cpp):
//   host -> guest  {"op":"campaign","id":"<16 hex>","mode"?,"ff"?,"players"?}              the campaign of this session (sent to a guest once, again when it changes);
//                                                                                     mode ("normal" | "balanced") and ff are the campaign's rebalance setting (docs/rebalance.md),
//                                                                                     players the party size now; both sent again when they change
//   host -> guest  {"op":"partySave","campaign","hostSave","seq","reqSlot"?,"reqId"?}   the host saved: save your game now (reqSlot/reqId: this save was made for
//                                                                                     that guest's hand save, which is its party save already)
//   host -> guest  {"op":"partySaveWait","text"}                                      the host cannot save right now (HUD line for the requesting guest)
//   guest -> host  {"op":"partySaved","seq","save","skipped"?}                        the guest's save for that sequence exists (or it gave the sequence up)
//   guest -> host  {"op":"partySaveRequest","id","save"}                              the guest saved by hand: make a party save for it
#pragma once
#include <cstdint>
#include <deque>
#include <functional>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "common/minijson.h"
#include "fnvcoop_mut.h"
#include "rebalance/campaign_settings.h"

namespace nvcoop::coop::party {

constexpr const char* kSidecarExt = ".fnvparty";              // <save>.fos.fnvparty: Meta, next to every save made in a co-op session
constexpr const char* kIndexFile = "nvcoop_party_index.txt";   // in Data\NVSE\Plugins of the game folder: a guest's Index
constexpr size_t kMaxNamePart = 80;                            // the host save name inside a guest save name (the launcher accepts 150 in all)
constexpr uint64_t kPendingGiveUpMs = 10 * 60 * 1000;          // a guest that could not save for this long drops the party save
constexpr uint64_t kRetryMs = 1000;                            // between two tries of a save that was refused
constexpr int kMaxFailures = 20;                               // refused tries before a save is given up

// ---- campaign id and names ----------------------------------------------------------------------------------------------------------------------
std::string campaignHex(uint64_t id);                             // 16 lower-case hex digits
bool parseCampaign(const std::string& text, uint64_t* id);        // 16 hex digits, not zero
std::string shortCampaign(uint64_t id);                           // the first 6 digits: what a save name shows

// "C:\...\Saves\Save 4 - Ann Primm.fos" / "Save 4 - Ann Primm.fos" / "Save 4 - Ann Primm" -> "Save 4 - Ann Primm" (what LoadGame takes).
std::string saveBaseName(const std::string& pathOrName);
// Safe as a file name and as a console argument: no path characters, quotes or control characters, no edge spaces or dots, at most maxLen.
std::string cleanName(const std::string& raw, size_t maxLen);
// "Co-op ab12cd Save 4 ..." -> "Save 4 ..." (a name made by this code is never wrapped a second time).
std::string stripPartyPrefix(const std::string& name);
// The guest's save for a host save: "Co-op <short campaign> <host save name>". The load menu shows it as written.
std::string guestSaveName(uint64_t campaign, const std::string& hostSave);
// The host's save made for a guest's hand save: the same scheme around the guest's save name.
std::string hostSaveNameForRequest(uint64_t campaign, const std::string& guestSave);
bool sameName(const std::string& a, const std::string& b);        // file names compare without case

// ---- the sidecar next to a save -------------------------------------------------------------------------------------------------------------------
struct Meta {
    uint64_t campaign = 0;
    bool host = false;           // role=host: a save the host can continue the campaign from
    std::string hostSave;        // the host save this save belongs to (a host save names itself)
    nvcoop::rebalance::CampaignSettings settings;   // the campaign's rebalance setting (docs/rebalance.md): "rebalance=" and "friendlyFire=" lines, none when not chosen
};
std::string metaText(const Meta& m);
bool parseMeta(const std::string& text, Meta* out);

// ---- a guest's index of the saves it made for host saves ---------------------------------------------------------------------------------------
struct IndexEntry {
    uint64_t campaign = 0;
    std::string hostSave, guestSave;
    int64_t time = 0;            // unix seconds
};
class Index {
public:
    static Index parse(const std::string& text);
    std::string text() const;
    // Records a save; an entry of the same campaign and host save is replaced (the host save was overwritten, so was this one).
    void add(const IndexEntry& e);
    // The entry whose guest save to load when the host continues from `hostSave`: the one for that host save, else the newest of the campaign, else null.
    // `exists` (optional) rejects entries whose save file is gone.
    const IndexEntry* pick(uint64_t campaign, const std::string& hostSave, const std::function<bool(const std::string&)>& exists = nullptr) const;
    const std::vector<IndexEntry>& entries() const { return e_; }
private:
    std::vector<IndexEntry> e_;
};

// ---- orders ---------------------------------------------------------------------------------------------------------------------------------------
struct SaveOrder {
    uint64_t campaign = 0;
    std::string hostSave;
    uint64_t seq = 0;
    int reqSlot = -1;            // the guest whose hand save this host save was made for (-1: none)
    uint64_t reqId = 0;
};
std::string campaignOrderJson(uint64_t campaign, const nvcoop::rebalance::CampaignSettings& settings = {}, int players = 0);
std::string saveOrderJson(const SaveOrder& o);
bool parseSaveOrder(const nvcoop::json::Value& v, SaveOrder* out);
std::string saveWaitJson(const std::string& text);
std::string ackJson(uint64_t seq, const std::string& save, bool skipped);
std::string requestJson(uint64_t id, const std::string& save);

// ---- the host's side ------------------------------------------------------------------------------------------------------------------------------
struct HostEnv {
    std::function<std::vector<uint8_t>()> joined;                          // slots of the joined guests
    std::function<bool(uint8_t slot, const std::string& json)> sendOrder;  // false: not joined / still syncing (tried again for a while)
    std::function<bool()> canSaveNow;                                      // the engine's CanSaveNow (through the SaveGate)
    std::function<bool(const std::string& name)> saveNow;                  // the engine's direct save; true = saved (the SaveGame message follows)
    std::function<void(const std::string&)> log;
    std::function<uint64_t()> random64;
    std::function<uint64_t()> nowMs;
};

class HostParty {
public:
    explicit HostParty(HostEnv env) : env_(std::move(env)) {}
    uint64_t campaign() const { return campaign_; }
    void setCampaign(uint64_t id) { campaign_ = id; }       // a loaded save's campaign (0: that save has none)
    uint64_t ensureCampaign();                              // mints one when there is none: a party that starts from the start save is a new campaign; it takes the pending settings
    // The campaign's rebalance setting. A loaded save's own (setSettings, with its campaign); a new campaign takes the host's pick (setNewCampaignSettings), which is
    // locked into it when the campaign is minted and never changes after.
    void setSettings(const nvcoop::rebalance::CampaignSettings& s) { settings_ = s; }
    void setNewCampaignSettings(const nvcoop::rebalance::CampaignSettings& s) { pending_ = s; }
    const nvcoop::rebalance::CampaignSettings& settings() const { return settings_; }
    int partySize() const;                                  // this host plus the joined guests
    // The host's game saved (NVSE SaveGame message: manual, quick, auto or scripted). Orders every joined guest to save; returns the sequence.
    uint64_t onHostSaved(const std::string& savePathOrName);
    void onGuestRequest(uint8_t slot, uint64_t id, const std::string& guestSave);   // a guest saved by hand
    void onAck(uint8_t slot, uint64_t seq, const std::string& guestSave, bool skipped);
    void tick();                                            // each frame: tell guests the campaign, retry orders, serve queued requests
    // What the tests and coop.status read.
    struct Stats { uint64_t orders = 0, acks = 0, skipped = 0, requests = 0, requestSaves = 0, waits = 0, resent = 0; };
    const Stats& stats() const { return stats_; }
    size_t ackedCount(uint64_t seq) const { auto it = acked_.find(seq); return it == acked_.end() ? 0 : it->second.size(); }
    size_t queuedRequests() const { return requests_.size(); }
private:
    struct Request { uint8_t slot = 0; uint64_t id = 0; std::string guestSave; bool notified = false; int failures = 0; uint64_t lastTry = 0; };
    struct Resend { uint8_t slot; std::string json; uint64_t until; };
    void sendOrResend(uint8_t slot, const std::string& json);
    void serveRequest();
    uint64_t nextSeq();
    HostEnv env_;
    uint64_t campaign_ = 0;
    nvcoop::rebalance::CampaignSettings settings_, pending_;
    uint64_t run_ = 0, counter_ = 0;
    int players_ = 1;                                       // joined guests + 1, as of the last tick
    struct Told { uint64_t campaign = 0; nvcoop::rebalance::CampaignSettings settings; int players = 0; };
    std::map<uint8_t, Told> told_;                          // slot -> what it knows of the campaign
    std::map<uint64_t, std::set<uint8_t>> acked_;
    std::deque<Request> requests_;
    struct Expect { bool active = false; std::string name; Request req; uint64_t since = 0; } expect_;
    std::vector<Resend> resend_;
    Stats stats_;
};

// ---- the guest's side -----------------------------------------------------------------------------------------------------------------------------
struct GuestEnv {
    std::function<int()> mySlot;                             // this machine's slot in the session (-1: not joined)
    std::function<bool(const std::string& json)> sendToHost; // false: not joined / still syncing (tried again next frame)
    std::function<bool()> canSaveNow;
    std::function<bool(const std::string& name)> saveNow;    // the engine's direct save; the SaveGame message for `name` arrives during or after it
    std::function<void(const std::string&)> notify;          // the game's HUD line
    std::function<void(const std::string&)> log;
    std::function<int64_t()> wallSeconds;
    std::function<uint64_t()> nowMs;
    std::function<std::string()> readIndex;                  // the index file's text ("" = none)
    std::function<bool(const std::string& text)> writeIndex;
};

class GuestParty {
public:
    explicit GuestParty(GuestEnv env) : env_(std::move(env)) {}
    uint64_t campaign() const { return campaign_; }
    void setCampaign(uint64_t id) { campaign_ = id; }       // the host's campaign (order), or the loaded save's own
    // The campaign's rebalance setting and the party size, as the host told them (not chosen / 0 until it does).
    void setSettings(const nvcoop::rebalance::CampaignSettings& s) { settings_ = s; }
    void setPlayers(int n) { players_ = n; }
    const nvcoop::rebalance::CampaignSettings& settings() const { return settings_; }
    int players() const { return players_; }
    void onPartySaveOrder(const SaveOrder& o);
    void onWait(const std::string& text) { if (env_.notify) env_.notify(text); }   // the host cannot save for the hand save this machine just made
    // The game saved (NVSE SaveGame message). Returns true when it was a hand save (quick, menu) that has been sent to the host as a request; false for the
    // saves this machine makes for a party save order.
    bool onLocalSave(const std::string& savePathOrName);
    // Is this save one this machine made for a party save? Then its sidecar names the host save it belongs to (empty otherwise).
    std::string ownHostSave(const std::string& savePathOrName) const;
    void tick();
    bool pending() const { return pending_; }
    struct Stats { uint64_t saves = 0, adopted = 0, duplicates = 0, superseded = 0, deferred = 0, failed = 0, requests = 0, acks = 0; };
    const Stats& stats() const { return stats_; }
private:
    struct Request { uint64_t id = 0; std::string save; bool sent = false; };
    void remember(const std::string& hostSave, const std::string& guestSave);
    void ack(uint64_t seq, const std::string& save, bool skipped);
    bool handled(uint64_t seq) const { return done_.count(seq) != 0; }
    void markHandled(uint64_t seq);
    GuestEnv env_;
    uint64_t campaign_ = 0;
    nvcoop::rebalance::CampaignSettings settings_;
    int players_ = 0;
    bool pending_ = false;
    SaveOrder order_;
    uint64_t pendingSince_ = 0, lastTry_ = 0;
    int failures_ = 0;
    bool announced_ = false;
    std::set<uint64_t> done_;
    std::deque<uint64_t> doneOrder_;
    std::map<std::string, std::string> own_;                // lower-case save name -> the host save it was made for
    std::string inFlight_;
    bool haveRequest_ = false;
    Request request_;
    uint64_t nextRequest_ = 0;
    std::vector<std::string> outbox_;                       // acks to send; kept until the host took them
    Stats stats_;
};

// The order dispatchers the engine glue calls (and the tests, so the same JSON path is exercised). Return the order applier / handler status.
std::string applyGuestOrder(GuestParty& g, const nvcoop::json::Value& order);                          // campaign, partySave, partySaveWait
std::string handleHostOrder(HostParty& h, uint8_t from, const nvcoop::json::Value& order);             // partySaveRequest, partySaved
bool isGuestOp(const std::string& op);
bool isHostOp(const std::string& op);

}  // namespace nvcoop::coop::party
