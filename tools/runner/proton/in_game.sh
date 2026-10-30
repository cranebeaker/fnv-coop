#!/bin/bash
# Run something inside the running game's Proton session (docs/tools/proton-deck.md). Copy 0 must have been started by Steam
# with launch_copy.sh, which opens Steam's launch service for the game. The plugin's named pipes \\.\pipe\nvcoop-<n> exist in
# that session's wineserver only: a wine started any other way (protontricks, a bare proton run) gets its own and sees no pipe.
#
#     tools/runner/proton/in_game.sh py tools/runner/nvbridge.py --instance 0 ping    # Windows Python, paths relative to the checkout
#     tools/runner/proton/in_game.sh start 1                                          # start copy 1 next to copy 0
#
# NVCOOP_WIN_PYTHON: the Windows Python in the game's prefix (default C:\Python311\python.exe).
# FNV_INSTANCES_ROOT: where the copies are (default: fnv-instances next to the checkout, as tools/runner/instances.py).
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../../.." && pwd)"
root="${FNV_INSTANCES_ROOT:-$(cd "$repo/.." && pwd)/fnv-instances}"
client="${STEAM_RUNTIME_LAUNCH_CLIENT:-$HOME/.local/share/Steam/steamapps/common/SteamLinuxRuntime_sniper/pressure-vessel/bin/steam-runtime-launch-client}"
bus="com.steampowered.App22380"
py="${NVCOOP_WIN_PYTHON:-C:\\Python311\\python.exe}"

usage() {
    echo "usage: $0 py <script.py> [args...]   |   $0 start <copy number>" >&2
    exit 2
}

case "${1:-}" in
    py)
        shift
        [ $# -ge 1 ] || usage
        exec "$client" --bus-name="$bus" --directory="$repo" -- wine "$py" -I "$@"
        ;;
    start)
        n="${2:-}"
        [ -n "$n" ] || usage
        copy="$root/$n"
        if [ ! -f "$copy/nvse_loader.exe" ]; then
            echo "in_game.sh: no copy '$n' with nvse_loader.exe under $root (run tools/runner/instances.py provision first)" >&2
            exit 2
        fi
        mkdir -p "$root/logs"
        log="$root/logs/start-$n.log"
        # The game keeps running after this script ends; its output goes to the log.
        NVCOOP_TEST_HELPERS=1 nohup "$client" --bus-name="$bus" --pass-env=NVCOOP_TEST_HELPERS --directory="$copy" \
            -- wine "$copy/nvse_loader.exe" > "$log" 2>&1 &
        echo "copy $n starting in the game's session (log: $log)"
        ;;
    *)
        usage
        ;;
esac
