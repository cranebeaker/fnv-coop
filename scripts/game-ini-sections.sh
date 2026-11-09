#!/bin/bash
# game-ini-sections.sh [OUT]: writes nv-rs's four [nv-rs ...] sections of Fallout.ini, each key
# with its one comment line above it, from crates/host/data/game_ini.tsv (default OUT:
# distribution/nv-rs-sections.ini: the block a settings tool can put at the top of a Fallout.ini
# that has none, and the one the docs name; distribution/Settings.cmd does not use it yet). The
# [nv-rs Fixes] lines are written commented out, so sPreset
# decides until a player removes the ; in front of one. Keys the game never reads are not written.
# crates/host's game_ini tests check the file against the table; run this after changing the table.
set -e
ROOT="$(dirname "$0")/.."
TABLE="$ROOT/crates/host/data/game_ini.tsv"
OUT="${1:-$ROOT/distribution/nv-rs-sections.ini}"
{
  echo "; nv-rs's own options. Only nv-rs reads these sections; the original game never asks for them."
  echo "; Lines starting with ; are notes. A key nv-rs does not know is passed over, as the game does."
  awk -F'\t' '
    { sub(/\r$/, "") }
    /^#/ || NF == 0 { next }
    {
      if ($1 != section) {
        print ""
        print "[" $1 "]"
        section = $1
        if ($1 == "nv-rs Fixes")
          print "; Each fix follows sPreset. Remove the ; in front of a line to turn that one fix on (1) or off (0)."
      }
      print "; " $5
      print (($1 == "nv-rs Fixes") ? ";" : "") $2 "=" $4
    }' "$TABLE"
} > "$OUT"
