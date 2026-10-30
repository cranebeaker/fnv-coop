#!/bin/bash
# Steam launch option for Fallout: New Vegas under Proton (Steam Deck / Linux), docs/tools/proton-deck.md:
#     <full path of the checkout>/tools/runner/proton/launch_copy.sh 0 %command%
# Steam passes its Proton command for the Steam folder's exe. This script swaps that exe for copy <n>'s nvse_loader.exe and
# starts it from the copy's folder, so Proton, its Steam bridge (the game's DRM check) and the Steam overlay work as usual.
# The copies are where tools/runner/instances.py makes them: $FNV_INSTANCES_ROOT, else fnv-instances next to the checkout.
# Clear the launch option to get the normal game back.
n="${1:-}"
shift
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="${FNV_INSTANCES_ROOT:-$(cd "$here/../../../.." && pwd)/fnv-instances}"
copy="$root/$n"
mkdir -p "$root/logs"
log="$root/logs/steam-launch-$n.log"
if [ -z "$n" ] || [ ! -f "$copy/nvse_loader.exe" ]; then
    echo "launch_copy.sh: no copy '$n' with nvse_loader.exe under $root (run tools/runner/instances.py provision first)" | tee "$log" >&2
    exit 2
fi
args=()
for a in "$@"; do
    case "$a" in
        */FalloutNVLauncher.exe|*/FalloutNV.exe) args+=("$copy/nvse_loader.exe") ;;
        *) args+=("$a") ;;
    esac
done
cd "$copy" || exit 1
# Opens Steam's launch service for this game (com.steampowered.App22380): in_game.sh runs Windows Python and the other copies
# inside the game's own wineserver through it, where the plugin's named pipes \\.\pipe\nvcoop-<n> exist.
export STEAM_COMPAT_LAUNCHER_SERVICE=proton
# The playbooks' scenario reset (test.quest.reset) needs the plugin's test helpers.
export NVCOOP_TEST_HELPERS=1
{ date; printf '%q ' "$@"; echo; printf '%q ' "${args[@]}"; echo; } > "$log"
exec "${args[@]}" >> "$log" 2>&1
