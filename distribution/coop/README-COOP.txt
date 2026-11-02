nv-rs co-op (experimental)

You need your own copy of Fallout: New Vegas, version 1.4.0.525, installed on this PC.
Nothing from the game is included here and the launcher never changes your game folder.

Setup (once)
1. Extract the whole ZIP to a folder of your own (not inside Program Files).
2. Double-click Play-Coop.cmd.
3. Choose 1 (Setup). The launcher looks for your install. Pick it, or type the path to the
   folder that holds FalloutNV.exe. Enter a player name.

Host
1. Choose 2 (Host a game). Pick New game, or Continue to resume a campaign.
2. Enter the address for the join code: your LAN IP for friends on your network, your public
   IP for friends over the internet. (Option 5, Port check, shows steps and tests your port.)
3. Send the join code (NVRS-...) to your friends. Start the game. They join while you play.
4. Friends over the internet need UDP port 27961 forwarded on your router to this PC, or a
   private-network app such as ZeroTier or Tailscale. Allow nv-viewer.exe in Windows Firewall
   when asked.

Join
1. Choose 3 (Join a game). Paste the join code, or type ip:port.
2. Optionally enter the campaign id the host shows, so your saves are kept with that campaign.

Saves
Press F5 in game to save and F9 to load. Choose 4 (Party saves) to see saves per campaign,
continue a hosted campaign, or delete one save (you must type its exact file name).
Saves live in %APPDATA%\nv-rs-coop\saves.

If something goes wrong
Choose 6 (Diagnostics). It makes a folder of logs with your Windows user name removed. Share
that folder. "nv-launcher.exe --selftest" checks the launcher itself.

Steam friends
Optional and not part of this build unless the viewer says so. UDP direct is always available.
