// nvcoop module for positional voice chat (x86, compiled into nvcoop.dll next to the coop module). NOTHING here has run in a game.
// docs/voice.md is the design; the game-free parts (wire, jitter buffer, placement, transmitter, receiver, session, settings) are
// the nvcoop_voice_core library and are unit tested.
//
//   * reads nvcoop.ini [voice] (voice_config.h); enabled=0 switches everything off
//   * a co-op session (host or joined guest) builds a VoiceSession on the session's mux (channel kChannelVoice, unreliable)
//   * the microphone (waveIn) feeds the session once per frame; the talk key is polled with GetAsyncKeyState while the game window
//     has the focus
//   * the speakers (waveOut) pull the mix on their own thread; every frame this module tells the receiver where each remote player
//     is (their puppet's place relative to the local player, from the avatar layer) and the receiver moves the volume and side there
//   * bridge methods: voice.status voice.mute voice.volume voice.mic voice.key voice.tone (the last two let a bot talk with no
//     microphone and no key press, voice.status shows what each voice sounds like, so a test needs no ears)
#include <Windows.h>

#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <cstdio>
#include <cstring>
#include <memory>
#include <mutex>
#include <string>
#include <vector>

#include "GameAPI.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "avatar/engine/engine_pins.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "coop_runtime.h"
#include "reload/reload_state.h"
#ifdef NVCOOP_WITH_LIVE
#include "live/live_glue.h"
#endif
#include "voice_audio_win.h"
#include "voice_config.h"
#include "voice_hint.h"
#include "voice_session.h"

namespace {

using namespace nvcoop::voice;
namespace P = nvcoop::avatar::pins;

// ------------------------------------------------------------------------------------------------------------------------ state

VoiceSettings g_set;
bool g_loaded = false, g_codecFailed = false;
std::mutex g_mtx;                                // the session and everything the audio thread reads
std::unique_ptr<IVoiceCodecFactory> g_codecs;
std::unique_ptr<VoiceSession> g_session;         // guarded by g_mtx
fnvcoop::MuxTransport* g_mux = nullptr;
std::shared_ptr<bool> g_alive;
bool g_isHost = false;
uint8_t g_self = 0xFF;

struct Link { uint8_t slot = 0; fnvcoop::PeerId peer = 0; bool joined = false; };
std::vector<Link> g_clients;                     // host: the guests, refreshed every frame (main thread)
bool g_wasJoined[kMaxSlots] = {};                // host: who was joined last frame (to forget a guest who left)

PcmQueue g_queue;
AudioCapture g_capture;
AudioPlayback g_playback;
HintGate g_hint;

// controls (main thread; the bridge sets them)
bool g_selfMuted = false, g_forceKey = false, g_tone = false;
float g_toneHz = 440.0f;
uint64_t g_toneSamples = 0, g_toneLastMs = 0;
uint64_t g_framesFed = 0, g_keyFrames = 0;

// what the speakers last played (audio thread writes under g_mtx, status reads under it)
int g_peakL = 0, g_peakR = 0;

// last placement per slot, for voice.status (main thread)
struct SlotView { bool present = false; bool audible = false; float gain = 0, pan = 0, dist = 0; };
SlotView g_view[kMaxSlots];

uint64_t NowMs() { return static_cast<uint64_t>(GetTickCount64()); }

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    const size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

IniGetter MakeGetter() {
    const std::string ini = IniPath();
    return [ini](const char* section, const char* key, const char* def) -> std::string {
        std::string env = "NVCOOP_";
        for (const char* p = section; *p; ++p) env += static_cast<char>(std::toupper(static_cast<unsigned char>(*p)));
        env += '_';
        for (const char* p = key; *p; ++p) env += static_cast<char>(std::toupper(static_cast<unsigned char>(*p)));
        char ebuf[64] = {};
        if (GetEnvironmentVariableA(env.c_str(), ebuf, sizeof ebuf) > 0) return ebuf;
        char buf[256] = {};
        GetPrivateProfileStringA(section, key, def, buf, sizeof buf, ini.c_str());
        return buf;
    };
}

// ------------------------------------------------------------------------------------------------------------------- engine reads
// Same rules as plugin/avatar/engine/win/game_backend_win.cpp: addresses only from engine_pins.h, every raw read inside a __try helper
// that touches plain data only. The player and the puppets are read, never written.

struct RawPose {
    bool ok = false;
    float x = 0, y = 0, z = 0, yaw = 0;
    uint32_t cell = 0, world = 0;
    bool interior = false;
};

void ReadRefRaw(TESObjectREFR* r, RawPose& o) {
    __try {
        o.x = r->posX;
        o.y = r->posY;
        o.z = r->posZ;
        o.yaw = r->rotZ;
        TESObjectCELL* c = r->parentCell;
        if (!c) return;
        o.cell = c->refID;
        o.interior = c->worldSpace == nullptr;
        o.world = c->worldSpace ? c->worldSpace->refID : 0;
        o.ok = true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.ok = false;
    }
}

TESForm* LookupSEH(uint32_t id) {
    __try {
        return id ? reinterpret_cast<TESForm*(__cdecl*)(UInt32)>(P::LookupFormByID.addr)(id) : nullptr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

bool PrologueMatches(const P::Pin& pin) {
    __try {
        return std::memcmp(reinterpret_cast<const void*>(static_cast<uintptr_t>(pin.addr)), pin.bytes, sizeof pin.bytes) == 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

uintptr_t PlayerPtr() {
    uintptr_t p = 0;
    __try {
        p = *reinterpret_cast<const uintptr_t*>(static_cast<uintptr_t>(P::kThePlayerPtr));
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        p = 0;
    }
    return p;
}

Where ToWhere(const RawPose& r) {
    Where w;
    if (!r.ok || !std::isfinite(r.x) || !std::isfinite(r.y) || !std::isfinite(r.z)) return w;
    w.present = true;
    w.pos = Vec3{r.x, r.y, r.z};
    w.yaw = std::isfinite(r.yaw) ? r.yaw : 0.0f;
    w.cell = r.cell;
    w.world = r.world;
    w.interior = r.interior;
    return w;
}

Where LocalPlayerWhere() {
    RawPose r;
    if (const uintptr_t p = PlayerPtr()) ReadRefRaw(reinterpret_cast<TESObjectREFR*>(p), r);
    return ToWhere(r);
}

Where PuppetWhere(uint32_t ref) {
    static int s_pin = -1;   // -1 unknown, 0 mismatch, 1 ok
    if (s_pin < 0) {
        s_pin = PrologueMatches(P::LookupFormByID) ? 1 : 0;
        if (!s_pin) nvcoop::Log("voice: LookupFormByID does not match the pinned bytes: voices have no place (heard evenly if global_fallback=1)");
    }
    Where w;
    if (!ref || s_pin != 1) return w;
    TESForm* f = LookupSEH(ref);
    if (!f || f->typeID < kFormType_TESObjectREFR || f->typeID > kFormType_Creature) return w;
    RawPose r;
    ReadRefRaw(static_cast<TESObjectREFR*>(f), r);
    return ToWhere(r);
}

bool AvatarsOn() {
#ifdef NVCOOP_WITH_LIVE
    return nvcoop::livewire::glueSwitches().avatars;
#else
    return false;
#endif
}

uint32_t PuppetRef(uint8_t slot) {
#ifdef NVCOOP_WITH_LIVE
    return nvcoop::livewire::glueAvatarRef(slot);
#else
    (void)slot;
    return 0;
#endif
}

// The talk key counts only while the game window has the focus (GetAsyncKeyState sees keys typed into other programs too).
bool GameHasFocus() {
    HWND fg = GetForegroundWindow();
    if (!fg) return false;
    DWORD pid = 0;
    GetWindowThreadProcessId(fg, &pid);
    return pid == GetCurrentProcessId();
}

bool TalkKeyDown() {
    if (g_set.key <= 0) return false;
    return (GetAsyncKeyState(g_set.key) & 0x8000) != 0 && GameHasFocus();
}

// ------------------------------------------------------------------------------------------------------------------------ session

uint8_t SlotOfPeer(fnvcoop::PeerId from) {
    if (!g_isHost) return 0;
    for (const Link& l : g_clients) if (l.peer == from && l.joined) return l.slot;
    return 0xFF;
}

void Teardown() {
    g_alive.reset();
    g_capture.stop();
    g_playback.stop();
    {
        std::lock_guard<std::mutex> lock(g_mtx);
        g_session.reset();
        g_peakL = g_peakR = 0;
    }
    g_mux = nullptr;
    g_self = 0xFF;
    g_clients.clear();
    std::memset(g_wasJoined, 0, sizeof g_wasJoined);
    g_queue.clear();
    g_hint.reset();
    for (SlotView& v : g_view) v = SlotView{};
}

bool Build(fnvcoop::MuxTransport* mux, bool isHost, uint8_t self) {
    if (!g_codecs && !g_codecFailed) {
        g_codecs = makeOpusFactory(g_set.bitrate);
        if (!g_codecs) {
            g_codecFailed = true;
            nvcoop::Log("voice: the speech codec did not start; voice chat is off for this run");
        }
    }
    if (!g_codecs) return false;
    SessionConfig cfg;
    cfg.isHost = isHost;
    cfg.self = self;
    cfg.tx = txConfigFrom(g_set);
    cfg.jitter = jitterConfigFrom(g_set);
    auto send = [mux, isHost](uint8_t slot, const Bytes& b) {
        if (!mux) return;
        fnvcoop::PeerId to = 0;   // a guest's only peer is the host
        if (isHost) {
            bool found = false;
            for (const Link& l : g_clients) if (l.slot == slot && l.joined) { to = l.peer; found = true; break; }
            if (!found) return;
        }
        mux->sendEx(to, Bytes(b), fnvcoop::ITransport::Channel::Unreliable);
    };
    auto slots = []() {
        std::vector<uint8_t> v;
        for (const Link& l : g_clients) if (l.joined) v.push_back(l.slot);
        return v;
    };
    {
        std::lock_guard<std::mutex> lock(g_mtx);
        g_session.reset(new VoiceSession(cfg, *g_codecs, send, slots));
        g_session->rx().setMasterVolume(g_set.volume);
        for (uint8_t s : g_set.muted) g_session->rx().setMuted(s, true);
    }
    g_mux = mux;
    g_isHost = isHost;
    g_self = self;
    g_alive = std::make_shared<bool>(true);
    std::weak_ptr<bool> alive = g_alive;
    mux->registerChannel(kChannelVoice, [alive](fnvcoop::PeerId from, fnvcoop::Bytes&& data) {
        auto a = alive.lock();
        if (!a || !*a) return;
        const uint8_t slot = SlotOfPeer(from);
        if (slot == 0xFF) return;   // not a joined guest
        std::lock_guard<std::mutex> lock(g_mtx);
        if (g_session) g_session->onDatagram(slot, data.data(), data.size(), NowMs());
    });
    g_queue.clear();
    g_capture.start(g_set.micDevice, &g_queue);
    g_playback.start(g_set.speakerDevice, [](int16_t* stereo, size_t frames) {
        std::lock_guard<std::mutex> lock(g_mtx);
        if (g_session) {
            g_session->render(stereo, frames);
            int l = 0, r = 0;
            for (size_t i = 0; i < frames; ++i) {
                l = (std::max)(l, std::abs(static_cast<int>(stereo[i * 2])));
                r = (std::max)(r, std::abs(static_cast<int>(stereo[i * 2 + 1])));
            }
            g_peakL = l;
            g_peakR = r;
        } else {
            std::memset(stereo, 0, frames * 2 * sizeof(int16_t));
        }
    });
    nvcoop::Log("voice: session up as %s slot %u (%s)", isHost ? "host" : "guest", static_cast<unsigned>(self), describe(g_set).c_str());
    return true;
}

// A frame of the test tone instead of the microphone.
void ToneFrame(int16_t* out) {
    for (size_t i = 0; i < kFrameSamples; ++i) {
        const double t = static_cast<double>(g_toneSamples++) / kSampleRate;
        out[i] = static_cast<int16_t>(8000.0 * std::sin(6.283185307179586 * g_toneHz * t));
    }
}

// Where every remote player is heard from, read from the engine (no lock held: the audio thread keeps playing meanwhile).
void ComputePlacements(Placement* out) {
    const Where listener = LocalPlayerWhere();
    const PlacementConfig pc = placementConfigFrom(g_set);
    const bool avatars = AvatarsOn();
    for (uint8_t slot = 0; slot < kMaxSlots; ++slot) {
        out[slot] = Placement{};
        g_view[slot] = SlotView{};
        if (slot == g_self) continue;
        Placement p;
        SlotView v;
        if (!avatars) {
            // no avatar layer, no places: everybody is heard evenly (or nobody)
            if (g_set.globalFallback) p = globalPlacement(1.0f);
        } else {
            const Where speaker = PuppetWhere(PuppetRef(slot));
            v.present = speaker.present;
            p = computePlacement(listener, speaker, pc);
            if (speaker.present && listener.present) v.dist = distance(listener.pos, speaker.pos);
        }
        v.audible = p.audible;
        v.gain = p.gain;
        v.pan = p.pan;
        g_view[slot] = v;
        out[slot] = p;
    }
}

void OnFrame() {
    if (!g_set.enabled) return;
    nvcoop::coop::Runtime* rt = nvcoop::coop::Runtime::active();
    bool active = rt && rt->running();
    bool isHost = false;
    uint8_t self = 0xFF;
    std::vector<Link> clients;
    if (active) {
        if (auto* host = rt->host()) {
            isHost = true;
            self = 0;
            for (const auto& cv : host->clients()) {
                Link l;
                l.slot = static_cast<uint8_t>(cv.slot);
                l.peer = cv.peer;
                l.joined = cv.joined;
                if (l.slot < kMaxSlots) clients.push_back(l);
            }
        } else if (auto* cl = rt->client()) {
            if (cl->joined() && cl->slot() < kMaxSlots) self = static_cast<uint8_t>(cl->slot());
        }
    }
    if (!active || self == 0xFF || !rt->mux()) {
        if (g_session) Teardown();
        return;
    }
    if (!g_session || rt->mux() != g_mux || isHost != g_isHost || self != g_self) {
        if (g_session) Teardown();
        g_clients = clients;
        if (!Build(rt->mux(), isHost, self)) return;
    }
    g_clients = clients;
    const uint64_t now = NowMs();
    if (isHost) {
        bool joinedNow[kMaxSlots] = {};
        for (const Link& l : clients) if (l.joined) joinedNow[l.slot] = true;
        for (uint8_t s = 0; s < kMaxSlots; ++s) {
            if (g_wasJoined[s] && !joinedNow[s]) {
                std::lock_guard<std::mutex> lock(g_mtx);
                if (g_session) g_session->playerLeft(s);   // the guest left: its jitter buffer and decoder go
            }
            g_wasJoined[s] = joinedNow[s];
        }
    }

    const bool keyDown = g_forceKey || TalkKeyDown();
    Placement places[kMaxSlots];
    ComputePlacements(places);
    int fed = 0;
    bool showTalk = false, showMic = false;
    {
        std::lock_guard<std::mutex> lock(g_mtx);
        if (!g_session) return;
        int16_t frame[kFrameSamples];
        if (g_tone) {
            g_queue.clear();   // the real microphone is not used while a test tone plays
            uint64_t due = g_toneLastMs ? (now - g_toneLastMs) / kFrameMs : 1;
            if (due > 5) due = 5;
            g_toneLastMs = g_toneLastMs ? g_toneLastMs + due * kFrameMs : now;
            for (uint64_t i = 0; i < due; ++i) {
                ToneFrame(frame);
                g_session->feedMic(frame, keyDown, g_selfMuted);
                ++fed;
            }
        } else {
            g_toneLastMs = 0;
            while (fed < 6 && g_queue.popFrame(frame)) {
                g_session->feedMic(frame, keyDown, g_selfMuted);
                ++fed;
            }
        }
        g_framesFed += static_cast<uint64_t>(fed);
        if (keyDown) ++g_keyFrames;
        for (uint8_t slot = 0; slot < kMaxSlots; ++slot) g_session->rx().setPlacement(slot, places[slot]);
        if (g_set.hud) {
            showTalk = g_hint.talking(g_session->tx().talking(), now);
            showMic = g_hint.micFailed(g_capture.failed());
        }
    }
    // (the engine's message queue is called outside the lock the audio thread waits on)
    if (showTalk) {
        char msg[96];
        std::snprintf(msg, sizeof msg, "Voice: you are on the air (%s)", g_set.mode == Mode::OpenMic ? "open mic" : "push to talk");
        QueueUIMessage(msg, 0, nullptr, nullptr, 3.0f, false);
    }
    if (showMic) QueueUIMessage("Voice: no microphone found, you can hear others but not talk", 0, nullptr, nullptr, 5.0f, false);
}

// ------------------------------------------------------------------------------------------------------------------------ bridge

std::string StatusJson() {
    char b[1600];
    std::string out;
    std::lock_guard<std::mutex> lock(g_mtx);
    if (!g_set.enabled) return "{\"enabled\":false}";
    std::snprintf(b, sizeof b, "{\"enabled\":true,\"running\":%s,\"settings\":\"%s\"", g_session ? "true" : "false", describe(g_set).c_str());
    out = b;
    if (!g_session) return out + "}";
    const SessionStats& st = g_session->stats();
    const TxStats& tx = g_session->tx().stats();
    const RxStats& rs = g_session->rx().stats();
    std::snprintf(b, sizeof b,
                  ",\"host\":%s,\"self\":%u,\"talking\":%s,\"micLevel\":%.0f,\"micOpen\":%s,\"micFailed\":%s,\"speakersOpen\":%s,\"speakersFailed\":%s,"
                  "\"forceKey\":%s,\"tone\":%s,\"selfMuted\":%s,\"framesFed\":%llu,\"keyFrames\":%llu,\"peak\":[%d,%d],"
                  "\"sent\":%llu,\"received\":%llu,\"relayed\":%llu,\"forged\":%llu,\"ownEcho\":%llu,\"limited\":%llu,\"malformed\":%llu,"
                  "\"txBursts\":%llu,\"concealed\":%llu,\"duplicates\":%llu,\"late\":%llu,\"decodeErrors\":%llu,\"queueDropped\":%llu,\"voices\":[",
                  g_isHost ? "true" : "false", static_cast<unsigned>(g_self), g_session->tx().talking() ? "true" : "false", g_session->tx().level(),
                  g_capture.running() ? "true" : "false", g_capture.failed() ? "true" : "false", g_playback.running() ? "true" : "false",
                  g_playback.failed() ? "true" : "false", g_forceKey ? "true" : "false", g_tone ? "true" : "false", g_selfMuted ? "true" : "false",
                  static_cast<unsigned long long>(g_framesFed), static_cast<unsigned long long>(g_keyFrames), g_peakL, g_peakR,
                  static_cast<unsigned long long>(st.sent), static_cast<unsigned long long>(st.received), static_cast<unsigned long long>(st.relayed),
                  static_cast<unsigned long long>(st.forged), static_cast<unsigned long long>(st.ownEcho), static_cast<unsigned long long>(st.limited),
                  static_cast<unsigned long long>(st.malformed), static_cast<unsigned long long>(tx.bursts), static_cast<unsigned long long>(rs.concealed),
                  static_cast<unsigned long long>(rs.duplicates), static_cast<unsigned long long>(rs.late),
                  static_cast<unsigned long long>(rs.decodeErrors), static_cast<unsigned long long>(g_queue.dropped()));
    out += b;
    bool first = true;
    for (uint8_t slot = 0; slot < kMaxSlots; ++slot) {
        if (slot == g_self) continue;
        const SlotView& v = g_view[slot];
        std::snprintf(b, sizeof b, "%s{\"slot\":%u,\"present\":%s,\"audible\":%s,\"gain\":%.3f,\"pan\":%.3f,\"dist\":%.0f,\"muted\":%s,\"talking\":%s,\"heard\":%.3f,\"depth\":%u}",
                      first ? "" : ",", static_cast<unsigned>(slot), v.present ? "true" : "false", v.audible ? "true" : "false", v.gain, v.pan, v.dist,
                      g_session->rx().muted(slot) ? "true" : "false", g_session->rx().talking(slot) ? "true" : "false", g_session->rx().gainOf(slot),
                      static_cast<unsigned>(g_session->rx().depthOf(slot)));
        out += b;
        first = false;
    }
    return out + "]}";
}

bool ParseParams(const std::string& text, nvcoop::json::Value& out) {
    std::string err;
    if (text.empty() || text == "null") { out = nvcoop::json::Value::object(); return true; }
    return nvcoop::json::parse(text, out, err);
}
bool Flag(const nvcoop::json::Value& p, const char* key, bool def) {
    const nvcoop::json::Value* v = p.find(key);
    if (!v) return def;
    return v->type() == nvcoop::json::Value::Bool ? v->asBool(def) : v->asInt(def ? 1 : 0) != 0;
}

void RegisterBridge() {
    nvcoop::RegisterMethodClosure("voice.status", [](const std::string&, std::string& out) {
        out = StatusJson();
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("voice.mute", [](const std::string& params, std::string& out) {
        // {"slot":2,"muted":true}: stop (or resume) hearing one player
        nvcoop::json::Value p;
        if (!ParseParams(params, p)) { out = "bad params"; return false; }
        const int slot = static_cast<int>(p.get("slot").asInt(-1));
        if (slot < 0 || slot >= kMaxSlots) { out = "slot must be 0..7"; return false; }
        const bool muted = Flag(p, "muted", true);
        if (muted) g_set.muted.insert(static_cast<uint8_t>(slot)); else g_set.muted.erase(static_cast<uint8_t>(slot));
        std::lock_guard<std::mutex> lock(g_mtx);
        if (g_session) g_session->rx().setMuted(static_cast<uint8_t>(slot), muted);
        out = std::string("{\"slot\":") + std::to_string(slot) + ",\"muted\":" + (muted ? "true" : "false") + "}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("voice.volume", [](const std::string& params, std::string& out) {
        // {"volume":0.8}: the master volume of the others' voices (0 to 2)
        nvcoop::json::Value p;
        if (!ParseParams(params, p)) { out = "bad params"; return false; }
        const float v = static_cast<float>(p.get("volume").asNumber(g_set.volume));
        g_set.volume = v < 0 ? 0 : v > 2 ? 2 : v;
        std::lock_guard<std::mutex> lock(g_mtx);
        if (g_session) g_session->rx().setMasterVolume(g_set.volume);
        out = std::string("{\"volume\":") + std::to_string(g_set.volume) + "}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("voice.mic", [](const std::string& params, std::string& out) {
        // {"muted":true}: mute (or open) your own microphone
        nvcoop::json::Value p;
        if (!ParseParams(params, p)) { out = "bad params"; return false; }
        g_selfMuted = Flag(p, "muted", g_selfMuted);
        out = std::string("{\"muted\":") + (g_selfMuted ? "true" : "false") + "}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("voice.key", [](const std::string& params, std::string& out) {
        // {"down":true}: hold the talk key without a keyboard (a test bot)
        nvcoop::json::Value p;
        if (!ParseParams(params, p)) { out = "bad params"; return false; }
        g_forceKey = Flag(p, "down", g_forceKey);
        out = std::string("{\"down\":") + (g_forceKey ? "true" : "false") + "}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("voice.tone", [](const std::string& params, std::string& out) {
        // {"on":true,"hz":440}: send a test tone instead of the microphone (a test bot with no microphone)
        nvcoop::json::Value p;
        if (!ParseParams(params, p)) { out = "bad params"; return false; }
        g_tone = Flag(p, "on", g_tone);
        const double hz = p.get("hz").asNumber(g_toneHz);
        g_toneHz = static_cast<float>(hz < 80 ? 80 : hz > 4000 ? 4000 : hz);
        out = std::string("{\"on\":") + (g_tone ? "true" : "false") + ",\"hz\":" + std::to_string(g_toneHz) + "}";
        return true;
    }, true, 0);
}

// ------------------------------------------------------------------------------------------------------------------------ module

void OnLoad(void*) {
    if (g_loaded) return;
    g_loaded = true;
    g_set = readSettings(MakeGetter());
    nvcoop::Log("voice: %s", describe(g_set).c_str());
    if (!g_set.warnings.empty()) nvcoop::Log("voice: %s", g_set.warnings.c_str());
    RegisterBridge();
}

void Shutdown() {
    Teardown();
    g_codecs.reset();
    g_codecFailed = false;
    g_loaded = false;
}

void OnMessage(uint32_t type, void*, uint32_t) {
    switch (type) {
        case NVSEMessagingInterface::kMessage_ExitGame:
        case NVSEMessagingInterface::kMessage_ExitGame_Console: Shutdown(); break;
        default: break;
    }
}

}  // namespace

NVCOOP_MODULE(voice_chat, "voice_chat", OnLoad, OnMessage, OnFrame);

// Hot reload: the audio threads are stopped and the session dropped before the core is replaced; the new core builds a fresh session on its
// first frame (a few words may be lost, nothing else).
NVCOOP_RELOAD_STATE(voice_chat, "voice_chat", 1, 60,
    /*canUnload*/ nullptr,
    /*save*/ [](nvreload::Writer& w) {
        w.boolean(g_selfMuted);
        w.f64(g_set.volume);
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        const bool muted = r.boolean();
        const double vol = r.f64();
        if (r.ok()) {
            g_selfMuted = muted;
            g_set.volume = static_cast<float>(vol);
        }
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { Teardown(); },
    /*fresh*/ nullptr);
