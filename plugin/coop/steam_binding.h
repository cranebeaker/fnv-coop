// The game's Steam, for co-op over Steam (docs/steam.md): its peer-to-peer packets for fnvcoop::SteamTransport, our friends list, and
// "rich presence" (the tag that tells friends' launchers we are hosting). Fallout: New Vegas ships a 2010 steam_api.dll, but the Steam
// client behind it still hands out the interfaces asked for by version through its SteamClient(), so no new DLL is shipped: the game
// (or the launcher's helper, tools/steamlink) uses what the player's Steam already offers, under the game's own AppID.
//
// Interface layouts (Steamworks SDK 1.1x vtables) are in steam_binding.cpp. The pieces that cannot run without Steam are compiled for
// x86 Windows only (the game's arch); elsewhere attach() just fails and the rest of the build is unchanged.
#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include "fnvcoop/steam_transport.h"

namespace nvcoop::coop {

constexpr uint32_t kFalloutNvAppId = 22380;

struct SteamFriend {
    fnvcoop::SteamId id = 0;
    std::string name;
    std::string mode;   // what the host's launcher / game shows next to the name ("" if nothing)
    bool sameRelease = true;
};

class SteamBinding : public fnvcoop::ISteamApi {
public:
    static SteamBinding& instance();   // one per process: Steam's callbacks need a fixed address

    // Attach to a loaded steam_api.dll (module = its HMODULE): the game's own (GetModuleHandle), or one the helper loaded and initialised.
    // Safe to call twice. False (with a reason a player can read) when this is not the Steam we know or Steam is not logged in.
    bool attach(void* module, std::string* why = nullptr);
    // The plugin: attach to the steam_api.dll the game already loaded. Cached; call after the main menu is up.
    bool attachToGame(std::string* why = nullptr);
    bool ready() const { return ready_; }
    std::string personaName() const;

    // ISteamApi
    fnvcoop::SteamId me() override;
    uint32_t nowMs() override;
    bool send(fnvcoop::SteamId to, const uint8_t* data, uint32_t size, bool reliable, int channel) override;
    bool read(fnvcoop::Bytes& out, fnvcoop::SteamId& from, int channel) override;
    void accept(fnvcoop::SteamId who) override;
    bool isFriend(fnvcoop::SteamId who) override;
    bool takeSessionRequest(fnvcoop::SteamId& who) override;
    bool takeConnectFail(fnvcoop::SteamId& who, uint8_t& reason) override;
    size_t queuedBytes(fnvcoop::SteamId to) override;
    void setHosting(bool on) override { setHostingPresence(on, tag_, mode_); }

    // Rich presence. tag: the release / protocol the host runs (a friend's launcher lists only matching hosts); mode: free text for the list.
    void setHostTag(const std::string& tag, const std::string& mode) override { tag_ = tag; mode_ = mode; }
    void setHostingPresence(bool on, const std::string& tag, const std::string& mode);
    // Friends who are in the game right now and show the hosting tag.
    std::vector<SteamFriend> hostingFriends(const std::string& wantedTag);
    // A friend clicked "Join Game" on us (or accepted an invite) while this process runs: who, and the connect string they sent.
    bool takeJoinRequest(fnvcoop::SteamId& who, std::string& connect);
    int friendCount();

    static constexpr const char* kPresenceKey = "fnvcoop";        // "host <tag>"
    static constexpr const char* kPresenceMode = "fnvcoopmode";
    static constexpr const char* kPresenceConnect = "connect";    // Steam's "Join Game" button: "+fnvcoop_join <steam id>"

private:
    SteamBinding() = default;
    bool ready_ = false;
    std::string tag_, mode_;
};

}  // namespace nvcoop::coop
