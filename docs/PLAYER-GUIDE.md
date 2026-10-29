# FNV Co-Op: player guide

Play **Fallout: New Vegas** together: 2 to 4 players, one shared world and one shared story. One player hosts, the
others join. This guide is for players, not developers, and uses plain words.

## What you need

Every player needs:

* Windows and a copy of **Fallout: New Vegas** (Steam or GOG), updated to version **1.4.0.525** (the latest patch).
* **xNVSE** 6.3.0 or newer, the script extender that most New Vegas mods use. Get it from its official page,
  <https://github.com/xNVSE/NVSE/releases>, and unpack it into your game folder (the one that holds `FalloutNV.exe`).
  Start the game once through `nvse_loader.exe` to make sure it works.
* **Python 3** with tkinter, for the installer and the launcher. The installer from <https://www.python.org/downloads/>
  includes everything; tick "Add python.exe to PATH" in it.
* The **same release of FNV Co-Op as everyone else in your group.** The game checks this when you join and tells you
  if it does not match.

The installer never downloads anything. xNVSE and Python come from their own pages, above.

## Install (once per PC)

1. Unzip the release anywhere (for example your Downloads folder).
2. Double-click **Install.cmd**. It:
   * finds your New Vegas folder (Steam, GOG, or the common places), and asks if there is more than one;
   * checks the game is 1.4.0.525 and xNVSE is there;
   * checks the files in the download are undamaged (a SHA-256 check against the release's list);
   * saves a backup of what it will change, plus your `Fallout.ini` and `FalloutPrefs.ini`;
   * copies the plugin into `Data\NVSE\Plugins` in your game folder.
3. If it says something is wrong, it tells you what, and it has not changed anything yet. Fix that and run it again.

Your saves are never touched. If the game is running, close it first.

**Check.cmd** shows the same checks without changing anything, and whether the installed files are still intact.

## Host a game

1. Double-click **Launcher.cmd**. Your game folder is filled in; if it is not, press **Find** or **Browse**.
2. Type your **player name**, pick **Host a game**.
3. Press **Check port**. It tells you whether the port (default 27960, UDP) is free on your PC and shows the address
   friends on your home network use.
4. Friends over the internet need two things from you: your public IP address (search the web for "what is my IP")
   and a **port forwarding rule** on your router that sends **UDP** port 27960 to your PC. Every router calls this page
   a bit differently ("port forwarding", "virtual server", "NAT").
5. Optional but useful: press **Start test listener**, ask a friend to press **Test connection** (see "Join" below), and
   stop the listener again. If the friend sees "answered", the path works. Stop the listener before you start the game,
   because the game needs the port itself.
6. Press **Start game**. When you are in the world, friends can join. Load the save you want to play, or start a new
   game. **The host's save is the world.** Everybody else keeps their own character, inventory, level and skills. See "Save and continue a campaign" below for how saving works.

## Join a friend

1. Get the host's address from them, for example `203.0.113.5` or `203.0.113.5:27960`. If no port is given, 27960 is used.
2. Double-click **Launcher.cmd**, type your **player name**, pick **Join a friend** and type the address.
3. Optional: press **Test connection**. If it says "answered", the path to the host works (the host must have the test
   listener on, or already be hosting in the game).
4. Press **Start game**. Wait for the host to be in the world. (Steam players: leave Steam running, the game needs it.)

Your launcher remembers a private number for your PC, so when you rejoin you get the same player slot.

## Play over Steam (no address, no router settings)

If you and your host are **Steam friends**, you can skip the IP address and the port forwarding: Steam carries the game traffic between your PCs.

You need: Fallout: New Vegas **from Steam** (a GOG copy has no Steam to use), Steam running and logged in on every PC, and the host on your Steam friends list.
The launcher shows you as "playing Fallout: New Vegas" in Steam while it is open. That is normal.

**Host.** In the launcher's main menu press **CONNECTION** until it reads **STEAM FRIENDS**, then **HOST A GAME** (or **CONTINUE A GAME**). The lobby opens;
you give nobody an address. Your friends see you in their launcher, and in Steam's friends list a **Join Game** button appears next to your name.

**Join.** Set **CONNECTION** to **STEAM FRIENDS**, then press **JOIN A FRIEND**. The list shows your Steam friends who are hosting (the list refreshes itself).
Pick your host. If the host is still in the lobby you land in it and ready up as usual; if the host is already in the world, your game starts and joins them.
You can also keep the launcher open and click **Join Game** next to the host's name in Steam: the launcher then offers that host at the top of the list.

The lobby (ready-up, begin transmission, continue a saved campaign) works exactly as it does over an IP address. When the host presses begin, the launcher closes its
Steam link and each game starts and talks to Steam itself.

A host on a different release of FNV Co-Op is listed greyed out ("other release"): everyone needs the same release, as always.

## Save and continue a campaign

**Every save is a party save.** When the host saves (Escape, Save; quicksave; autosave), every player's game saves its own character at the same
moment, in a save called `Co-op <campaign code> <the host's save name>`, for example `Co-op 00ab12 Save 4 - Ann Primm`. The code is the same for the whole
campaign. If you cannot save right now (a menu is open, a story scene is running), your game saves as soon as you can; nothing else is needed.

**A friend saves, not the host?** A friend's Escape, Save (or quicksave) works too: the friend's game saves, then the host's game makes the matching save
(named `Co-op <campaign code> Host <the friend's save name>`) and every other player's game saves as well. If the host cannot save at that moment, the friend sees
a line on screen and the party save follows as soon as the host can.

**To continue later**, everybody starts the launcher as usual.

1. The host presses **CONTINUE A GAME** (next to HOST A GAME) and picks the save to continue. The list shows the host's co-op saves, newest first.
2. Friends press **JOIN A FRIEND** and connect to the lobby as always. The lobby says which save is being continued.
3. Everybody presses ready, the host presses **BEGIN TRANSMISSION**. The host's game loads the host's save; each friend's game loads that friend's own
   save for it (the one made at the same moment, or else the newest of the campaign).

A friend who has **no save of this campaign** (new to the group, or lost the file) joins as a new character: the game skips the opening, you make your
character as in a new game, and you are moved to the host. Ordinary single-player saves are not offered by CONTINUE.

Where the files are: your saves are where the game always keeps them (`Documents\My Games\FalloutNV\Saves`). Next to each co-op save is a small `.fnvparty`
file (keep it with the save); `Data\NVSE\Plugins\nvcoop_party_index.txt` in your game folder lists which of your saves belongs to which host save.
The design is in `docs/party-save.md`.

## Party settings

These are in the launcher and are saved into `nvcoop.ini`. The host's settings are the ones that count for the session.

| Setting | What it does |
|---|---|
| Share kill XP with the whole party | Off: the player who lands the killing blow gets the XP. On: everyone present shares it. Quest XP always goes to everyone. |
| Party slow-mo | Host option. When one player uses VATS, everyone slows down together for a short time. Off by default. |
| Tether | Story moments that need the group (a cutscene, a door everyone must pass) wait until the whole party is together. On by default. |
| Sync layers | Enemies and NPCs, seeing other players as characters, hits and projectiles, spawn arbitration. All on by default. Untick one only to narrow down a problem you are reporting. Combat and spawns need enemies and NPCs, so ticking them switches that one on too. |

Quest rewards, karma, reputation and the quest story are shared by everyone. Levels, perks, skills and SPECIAL are per player.

## Turn co-op off

Press **Turn co-op off** in the launcher, or just start the game the usual way after uninstalling. With co-op off, New Vegas
plays as normal single-player.

## Uninstall

Double-click **Uninstall.cmd**. It removes only the files the installer put there, and puts back the files and the
`nvcoop.ini` that were there before the install. A file you changed yourself after installing is copied to
`FNVCoOp\removed` in your game folder instead of being deleted. Other mods' files are never touched. Your saves stay.
The log file `nvcoop.log` also stays, so you can still send it when you ask for help; add `--remove-logs` to the
uninstall command (`python installer\fnv_install.py uninstall --remove-logs`) to delete it as well.

If the game's own settings ever look wrong, the installer's backup of `Fallout.ini` and `FalloutPrefs.ini` can be put back
with `python installer\fnv_install.py restore-inis` before you uninstall (the installer does not change those files; the
backup is just there in case).

## If something goes wrong

| What you see | What to try |
|---|---|
| Installer says the game is not 1.4.0.525 | Update or verify the game files in Steam or GOG Galaxy. |
| Installer says xNVSE is missing or too old | Install the latest xNVSE (link above) into the game folder, then run the installer again. |
| Installer says the download is damaged | Download the zip again and unzip it fresh. |
| "Fallout: New Vegas is already running" | Close the game, then try again. |
| Installer says Windows did not allow the change | Right-click **Install.cmd** and choose "Run as administrator" (a game under Program Files can need it), and close programs that may hold the files open. Nothing was changed. |
| Launcher: "UDP port ... already in use" | Another program, or a game that is already hosting, has the port. Pick another port (everyone must use the same one) or close the other program. |
| Launcher: "Steam is not running or you are not logged in" | Start Steam, log in, then press the button again. The game must be the Steam copy. |
| Steam: your host is not in the list | You must be Steam friends. The host must have pressed HOST A GAME with CONNECTION set to STEAM (or be in the world as host). The list refreshes every few seconds; press REFRESH. |
| Steam: "the host did not answer" / stays on "Calling the host through Steam" | The host's game may still be loading (it answers once it is in the world), or the host is not on your Steam friends list. Wait a minute, then pick the host from the list again. |
| Launcher: "this copy has no fnvsteam.exe" | The release is incomplete. Download it again; until then use the IP address instead. |
| Friend cannot join | Run Test connection. Check the router rule is for **UDP**, the port number matches, and Windows Firewall allows the game. If you both are on the same home network, use the local address that Check port shows. |
| Join is refused with a message | Read the message: the usual reasons are a different release, a different list of mods, or a full session (4 players). |
| The game opens its own small launcher window instead of the game | Start the game once through Steam or GOG so it writes its settings, then use the co-op launcher again. |

When you report a problem, send the file `Data\NVSE\Plugins\nvcoop.log` from your game folder (open it first and remove
anything you do not want to share).

## Limits to know about

* Windows, and New Vegas 1.4.0.525 only. Other game versions are refused on purpose.
* 2 to 4 players.
* Split-screen on one PC is a separate piece of work and is not part of this launcher.
* Mods that change quests, scripts or the same game systems as the co-op plugin can conflict with it. The join check
  compares the mod list, so everybody should run the same mods.
