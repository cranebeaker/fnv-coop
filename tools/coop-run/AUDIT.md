# Vacuity audit of the coop-run scenarios

Question for each scenario: if the feature under test did nothing, would the assertions still pass?
Each scenario that can be broken by removing its stimulus has a `twin_*.json` (`expect_fail: true`): the same
scenario with the guest line or host line dropped. The runner reports `PASS twin` only if an assertion fails, so a
twin that passes outright (`FAIL twin`) proves the assertions are vacuous. Twins are run by `suite scenarios`.

Caveat for everything below: it is a reading of the scenario and of what the viewer does for each line (host
scripts run on the host, guest `run` lines become intents the host applies, `coop VERB` lines run on the machine that
has them, `local` lines run locally). Only a real run settles it.

## Scenarios

| Scenario | Vacuous before? | Verdict now |
|---|---|---|
| doc_positions | Yes, fully | Fixed. `player.setpos` moved nobody; both avatars stayed on the spawn point and the old `near [100,0,0]` could not even have passed. Now `coop teleport`, and every avatar must be 150+ units from the spawn point on the owner and on the peer, and equal within 40 units (XY). Twin: nobody moves. |
| live_avatars_visible | Yes, fully | Fixed: teleport, then real `--key-at` walking (`--walk`), `dist_from` the spawn and from the teleport point (it walked), equal within 40, `live.avatars >= 1`. Twin: nobody moves. |
| live_reconnect | Yes (setpos) | Fixed: host at point A before the outage, point B (about 400 units away) after it; guest must see B and not A. Twin expects A and must fail. |
| live_tether_warp | Yes | The guest's `local player.setpos x 9000` moved nobody, so there was nothing to warp back, and `positions.guest.0 < 4096` plus `equal tol 600` held for a guest standing at the spawn point. Now the guest teleports 9000 units away and must end within 1500 of the spawn point. Twin: no tether, the guest stays far. |
| live_guest_kills_npc | No | The host only enables and moves the gecko; nothing on the host damages it. Dead on the host can only come from the guest's hit request being resolved by the host ledger, and `hits_sent >= 2` is a counter that is 0 if the layer does nothing. Caveat: dead on the guest alone could come from the guest's local `hurt`, which is why dead-host is the load-bearing check. Twin: the guest sends no hits. |
| live_loot_race | Partly | The container being empty and host/guest copies agreeing would also hold if the host alone took everything and the guest's take silently did nothing (host wins the race on a good link). Now the host takes 20 of 50, the guest tries 50: host + guest must hold exactly 50 (`sum`, no double grant, no loss), host at most 20, guest at least 30 (it can only get 30+ through the host ledger). Twin: the guest does nothing, so the guest holds 0. Open assumption: the ledger clips an over-asked take to what is left (the old description implied that). |
| live_dialogue_session | No | `live.lines_played >= 1` on both peers is 0 if no session runs, and the guest's `--talk` is the only trigger. Strengthened with `lines_played` equal on both peers (both play every line). Does not check that the host ran each line's result scripts exactly once (no counter for it). Twin: the guest never talks. |
| doc_stage_sync | Unknown, probably not | `quests.VCG03.stage >= 10` on the guest needs the guest to hold the quest at 10; the only source is the host's `--stage`, unless the guest starts there or the quest row defaults to a high stage. The twin (host sets nothing) proves it: it must fail. |
| doc_stage_bad_wifi | No | `items.guest.Caps001 == 25` on the host exists only if the guest's intent reached the host and was applied once; the hollow-pass guards also demand that the profile really dropped datagrams and the outage fired. Twin: no caps, no stage. |
| doc_player_quest_sync | Unknown, probably not | Same shape as doc_stage_sync (twin proves the guest does not start at stage 10). Added: the quest must NOT appear under the shared `quests.VCG01` on either peer, which is the per-player policy claim in the description. |

## Not audited here

`doc_flaky_items` and `live_guest_hits_essential`. The latter checks Doc stays alive and `hits_sent >= 1`; without
the essential rule the host would kill him, so the dead-absent check bites, but with no twin: dropping the hits
makes the first check true trivially, which is why `hits_sent` is there.

## Known risks of the new movement scenarios

* The teleport targets (spawn plus 200 in x, and so on) are guesses inside Doc Mitchell's house. If a target is inside
  a wall the character is pushed and may land under 150 units from the spawn point; move the coordinates, not the
  assertions.
* `--key-at` times count from the player being placed, `run` times from coop ready. The walk keys are at second 14,
  later than the teleports at 3 plus any join delay.
