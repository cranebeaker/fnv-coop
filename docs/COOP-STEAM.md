# Co-op over Steam

Play co-op with Steam friends without opening a router port. This is optional: the default is UDP
(see [COOP.md](COOP.md)). The Steam path is mock-tested only; nothing here has run against real
Steam yet (see the last section).

## What "Spacewar" means

The game is not on Steam, so it borrows Valve's free sample app, Spacewar (AppID 480). While the
mod runs, Steam shows you as playing "Spacewar" and your friends see that. Nothing is installed
or bought. Everyone else who runs a Spacewar-based mod shares the same name, so our lobbies carry a
marker and the mod ignores every lobby without it.

## What you need

* Steam running and logged in, on both machines. Offline mode does not work.
* You and the host are Steam friends (lobbies are friends-only).
* `steam_api64.dll` from Valve next to `nv-viewer.exe` (the release zip ships it; the mod never
  downloads it). `steam_appid.txt` (containing `480`) is written next to the exe by the mod.
* A viewer built with the `coop-steam` feature. Other builds ignore the Steam request and use UDP.

If any of these is missing the host plays on UDP instead and the log has a `transport_fallback`
event saying why. That is not an error. A guest cannot fall back (a Steam code names a Steam
lobby), so it gets a message saying what to fix.

## Hosting

Launcher: choose Host; with Steam available it passes `--coop-steam`. Or by hand:

    nv-viewer "<game dir>\Data" --new-game --coop-host --coop-steam

The game creates a friends-only lobby and prints a join code (`NVRS-...`, also the
`coop_join_code` and `steam_lobby` events in the co-op log). The code names the lobby and carries
a random token for this session. Friends can join three ways:

1. Accept your Steam invite (or Friends list, Join Game) while the mod is already running.
2. Type your code.
3. Run `--coop-steam-friend` and join the first friend who is hosting, no code needed.

## Joining

    nv-viewer "<game dir>\Data" --coop-join NVRS-...
    nv-viewer "<game dir>\Data" --coop-steam-friend

The launcher's Join menu accepts the code or the word `friend`.

## When "Join Game" launches Spacewar instead

If the mod is not running, Steam answers an accepted invite or Join Game by starting Spacewar
itself, not our game. **Start the mod first** (launcher or viewer), then accept the invite or use
Friends, Join Game. If you already clicked and Spacewar opened, close it, start the mod, and use
the host's code or `--coop-steam-friend`. The code always works.

## Plain addresses

A plain `HOST:PORT` no longer works for UDP play: it has no session token. Use the `NVRS-` code.
`--coop-test-token` (fixed token on both sides) exists for the automated test harness only.

## Log events

`coop_join_code` (code, token), `steam_lobby` (lobby id, code, friends online), `steam_joined`,
`transport_fallback` (`from`, `to`, `reason`).

## What only a two-account test can verify

Run once by the owner with two real Steam accounts that are friends. The first 13 are from the
Steam plan (research 07, section 5.3); the last 3 were added with the viewer wiring. Everything
else is covered by `MockSteam` tests only, and the release notes must say so.

1. x64 `SteamNetworkingMessage_t` offsets: `data` 0, `size` 8, sender identity 16. The code probes
   candidates and `RealSteam::sender_offset_won` should report 16; the log must show which won.
2. `SteamNetworkingMessagesSessionRequest_t` / `Failed_t` identity at offset 0 on x64, and the
   identity buffer size.
3. `InitFlat` and every export present in the pinned `steam_api64.dll`; interface versions
   (`SteamUser_v023`, `Matchmaking_v009`, `Friends_v018`, Messages `v002`, Utils `v004`).
4. `LobbyCreated` delivered twice; `LobbyEnter` ordering relative to the first message.
5. A friends-only lobby is visible to a friend and invisible to a non-friend; the marker key keeps
   us out of other Spacewar users' lists.
6. `InviteUserToLobby` arrives in Steam chat; the overlay invite dialog opens (or not) in our
   renderer.
7. Callbacks 333, 337, 503 on accept or Join Game while running; behaviour when the mod is not
   running (above).
8. Session accept rule: the first message is held until `AcceptSessionWithUser`; strangers are
   dropped.
9. Real relay across two networks (not only LAN): RTT, packet loss, reliable ordering on channel 0
   under real loss, unreliable maximum size.
10. Keep-alive, the 20 s timeout, `LobbyChatUpdate` leave/disconnect flags, kick, host crash and
    rejoin.
11. Large message fragmentation and the 32 MB buffer settings on the real DLL; the "buffer full"
    retry path.
12. `steam_appid.txt` location behaviour and that Spacewar presence shows.
13. Rich presence `connect` string delivery.
14. The `token` lobby data key is readable by the friend and `--coop-steam-friend` joins with it
    (friend auto-join via `GetFriendGamePlayed` app id 480 plus a non-zero lobby id).
15. `steam_api64.dll` is found next to the viewer exe (not only the working directory) when the
    launcher starts the viewer from a campaign folder.
16. The launcher's Steam check (`--help` lists `--coop-steam`, DLL present) matches what the
    viewer then does: Steam host works, and a viewer without the feature logs `transport_fallback`.
