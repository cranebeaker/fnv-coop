// nvcoop.ini [voice]: the settings of voice chat. Game-free. Every value is clamped to a sane range; a bad value falls back to its
// default and is named in `warnings`. NVCOOP_VOICE_<KEY> in the environment overrides the file (like the other modules).
//
//   enabled=1          off switch of the whole feature (no microphone is opened, nothing is sent or played)
//   mode=ptt           ptt (push to talk) | open (open microphone with a loudness gate; the talk key also forces it open)
//   key=LAlt           the talk key: a letter / digit, F1..F24, Insert Delete Home End PageUp PageDown Tab CapsLock Space,
//                      LAlt RAlt LCtrl RCtrl LShift RShift, Num0..Num9, Mouse3 (middle) Mouse4 Mouse5, or a number (0x41 or 65 = a virtual key code)
//   volume=1.0         master volume of the others' voices (0 to 2)
//   mic_volume=1.0     gain on your microphone (0 to 4)
//   range=3000         distance (game units, 70 units is about a metre) beyond which a voice is silent
//   full_range=400     distance within which a voice is at full volume
//   bitrate=24000      Opus bit rate in bit/s (6000 to 64000)
//   open_threshold=600 open mic: loudness that opens the gate (0 to 32767)
//   tail_ms=240        push to talk: how long the microphone stays on after the key goes up
//   hang_ms=400        open mic: how long the gate stays open after the voice dropped
//   jitter_ms=60       wait before a voice starts playing (smooths the network)
//   max_delay_ms=240   a voice never lags more than this behind what was sent (older audio is skipped)
//   hud=1              show a hint on the HUD while your microphone is on the air
//   global_fallback=1  no avatar layer = no places: hear everybody evenly at full volume (0: hear nobody)
//   muted=             comma separated slots whose voices you do not want to hear (slot 0 is the host)
//   mic_device=-1      audio device number, counted in the active microphones (-1: the system default)
//   speaker_device=-1  audio device number, counted in the active speakers (-1: the system default)
#pragma once
#include <functional>
#include <set>
#include <string>

#include "voice_jitter.h"
#include "voice_placement.h"
#include "voice_tx.h"

namespace nvcoop::voice {

using IniGetter = std::function<std::string(const char* section, const char* key, const char* def)>;

constexpr int kVkLAlt = 0xA4;   // VK_LMENU: the default talk key (FNV binds nothing to it, and it types nothing into a text box)

struct VoiceSettings {
    bool enabled = true;
    Mode mode = Mode::PushToTalk;
    int key = kVkLAlt;
    float volume = 1.0f;
    float micVolume = 1.0f;
    float range = 3000.0f;
    float fullRange = 400.0f;
    int bitrate = 24000;
    float openThreshold = 600.0f;
    uint32_t tailMs = 240;
    uint32_t hangMs = 400;
    uint32_t jitterMs = 60;
    uint32_t maxDelayMs = 240;
    bool hud = true;
    bool globalFallback = true;
    std::set<uint8_t> muted;
    int micDevice = -1;
    int speakerDevice = -1;
    std::string warnings;   // what was wrong in the file, one line
};

TxConfig txConfigFrom(const VoiceSettings& s);
JitterConfig jitterConfigFrom(const VoiceSettings& s);
PlacementConfig placementConfigFrom(const VoiceSettings& s);
VoiceSettings readSettings(const IniGetter& get);
std::string describe(const VoiceSettings& s);
// The virtual-key code a name stands for; 0 for a name it does not know.
int parseKey(const std::string& name);
std::string keyName(int vk);

}  // namespace nvcoop::voice
