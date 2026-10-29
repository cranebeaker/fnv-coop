cmake_minimum_required(VERSION 3.20)
project(fnvcoop_synccore LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
if(NOT CMAKE_CONFIGURATION_TYPES AND NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

# Core library: game-free, links into the game plugin (build Win32/x86 there).
add_library(fnvcoop STATIC
  src/wire.cpp src/shadow.cpp src/policy.cpp src/storylog.cpp
  src/protocol.cpp src/rules.cpp src/baseline.cpp src/replicator.cpp src/transport.cpp src/steam_transport.cpp src/transport_util.cpp src/session.cpp src/ownership.cpp)
target_include_directories(fnvcoop PUBLIC include)

# Real transport (ENet, vendored MIT source). Separate so the pure core has no socket dependency.
set(ENET_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../third_party/enet)
if(WIN32)
  add_library(enet_static STATIC ${ENET_DIR}/callbacks.c ${ENET_DIR}/compress.c ${ENET_DIR}/host.c ${ENET_DIR}/list.c
    ${ENET_DIR}/packet.c ${ENET_DIR}/peer.c ${ENET_DIR}/protocol.c ${ENET_DIR}/win32.c)
  target_include_directories(enet_static PUBLIC ${ENET_DIR}/include)
  target_compile_definitions(enet_static PRIVATE _WINSOCK_DEPRECATED_NO_WARNINGS)
  target_link_libraries(enet_static PUBLIC ws2_32 winmm)
else()
  # The vendored copy has the Windows backend only (no unix.c). Elsewhere link the system ENet (same 1.3.18 API):
  # Debian/Ubuntu libenet-dev, Fedora enet-devel, Arch enet, macOS brew install enet.
  find_path(FNVCOOP_ENET_INCLUDE enet/enet.h)
  find_library(FNVCOOP_ENET_LIB enet)
  if(NOT FNVCOOP_ENET_INCLUDE OR NOT FNVCOOP_ENET_LIB)
    message(FATAL_ERROR "ENet not found. This platform uses the system ENet: install libenet-dev (or the equivalent for your system).")
  endif()
  add_library(enet_static INTERFACE)
  target_include_directories(enet_static INTERFACE ${FNVCOOP_ENET_INCLUDE})
  target_link_libraries(enet_static INTERFACE ${FNVCOOP_ENET_LIB})
endif()
add_library(fnvcoop_enet STATIC src/enet_transport.cpp)
target_link_libraries(fnvcoop_enet PUBLIC fnvcoop enet_static)

# Simulator + tests (not shipped in the plugin).
add_library(fnvcoop_sim STATIC sim/sim.cpp)
target_include_directories(fnvcoop_sim PUBLIC sim)
target_link_libraries(fnvcoop_sim PUBLIC fnvcoop)

add_executable(fnvcoop_tests tests/main.cpp tests/test_wire.cpp tests/test_core.cpp tests/test_session.cpp tests/test_sim.cpp tests/test_transport.cpp tests/test_baseline.cpp tests/test_saveload.cpp tests/test_rules.cpp tests/test_rules_wave1.cpp tests/test_rules_w3b.cpp tests/test_desync.cpp tests/test_desync_clientonly.cpp tests/test_handshake.cpp tests/test_perf.cpp tests/test_perfguard.cpp tests/test_rejoin.cpp tests/test_rejoin_dlg.cpp tests/test_failquest.cpp tests/test_round3.cpp tests/test_hostload.cpp tests/test_refcmd.cpp tests/test_prepared_load.cpp tests/test_steam_transport.cpp)
target_compile_definitions(fnvcoop_tests PRIVATE FNVCOOP_RULES_JSON="${CMAKE_CURRENT_SOURCE_DIR}/../docs/quest-policies/rules.json"
  FNVCOOP_CAUSE_JSON="${CMAKE_CURRENT_SOURCE_DIR}/../docs/quest-policies/cause_quests.json")
target_include_directories(fnvcoop_tests PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../plugin/dialogue/include)
# perf_guard.h uses runtime mutation twins (tools/mut/fnvcoop_mut.h, FNVCOOP_MUT=<n>); only this test exe, never the plugin
target_include_directories(fnvcoop_tests PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../tools/mut)
target_compile_definitions(fnvcoop_tests PRIVATE FNVCOOP_RUNTIME_MUT)
target_link_libraries(fnvcoop_tests PRIVATE fnvcoop_sim fnvcoop_enet)

if(MSVC)
  foreach(t fnvcoop fnvcoop_sim fnvcoop_tests)
    target_compile_options(${t} PRIVATE /W4 /permissive- /utf-8 /wd4456 /D_CRT_SECURE_NO_WARNINGS)
  endforeach()
else()
  foreach(t fnvcoop fnvcoop_sim fnvcoop_tests)
    target_compile_options(${t} PRIVATE -Wall -Wextra)
  endforeach()
endif()

add_executable(fnvcoop_loopback tests/loopback.cpp)
target_link_libraries(fnvcoop_loopback PRIVATE fnvcoop_sim fnvcoop_enet)
if(MSVC)
  target_compile_options(fnvcoop_enet PRIVATE /W4 /permissive- /utf-8)
  target_compile_options(fnvcoop_loopback PRIVATE /W4 /permissive- /utf-8 /wd4456 /D_CRT_SECURE_NO_WARNINGS)
endif()

add_executable(fnvcoop_soak soak/soak.cpp)
target_link_libraries(fnvcoop_soak PRIVATE fnvcoop)
target_compile_definitions(fnvcoop_soak PRIVATE
  FNVCOOP_PROFILE_DEFAULT="${CMAKE_CURRENT_SOURCE_DIR}/data/goodsprings_profile.txt"
  FNVCOOP_RULES_DEFAULT="${CMAKE_CURRENT_SOURCE_DIR}/../docs/quest-policies/rules.json")
if(MSVC)
  target_compile_options(fnvcoop_soak PRIVATE /W4 /permissive- /utf-8 /wd4456 /D_CRT_SECURE_NO_WARNINGS)
endif()

enable_testing()
add_test(NAME soak_smoke COMMAND fnvcoop_soak --seconds 300 --players 4 --seed 3)
add_test(NAME soak_smoke_threaded COMMAND fnvcoop_soak --seconds 60 --players 4 --seed 4 --threaded)
# Issue #43: a client that drops on the last event frame with the longest reconnect delay must still rejoin and converge; the twin
# restores the old harness (reconnect after the run ended) and must fail.
add_test(NAME soak_late_disconnect COMMAND fnvcoop_soak --seconds 60 --players 4 --seed 4 --late-disconnect)
add_test(NAME twin_soak_late_disconnect COMMAND fnvcoop_soak --seconds 60 --players 4 --seed 4 --late-disconnect --mutate late-reconnect)
set_tests_properties(twin_soak_late_disconnect PROPERTIES WILL_FAIL TRUE)
add_test(NAME loopback_clean       COMMAND fnvcoop_loopback selftest)
add_test(NAME loopback_faults      COMMAND fnvcoop_loopback selftest --loss 15)
add_test(NAME twin_loopback_idempotency COMMAND fnvcoop_loopback selftest --loss 15 --mutate idempotency)
add_test(NAME twin_loopback_order      COMMAND fnvcoop_loopback selftest --loss 15 --mutate order)
add_test(NAME loopback_unreliable  COMMAND fnvcoop_loopback selftest --loss 10 --unreliable)
add_test(NAME unit_wire     COMMAND fnvcoop_tests --group wire)
add_test(NAME unit_core     COMMAND fnvcoop_tests --group core)
add_test(NAME unit_session  COMMAND fnvcoop_tests --group session)
add_test(NAME unit_round3 COMMAND fnvcoop_tests --group round3)
add_test(NAME unit_refcmd COMMAND fnvcoop_tests --group refcmd)
add_test(NAME unit_hostload COMMAND fnvcoop_tests --group hostload)
add_test(NAME unit_rejoin COMMAND fnvcoop_tests --group rejoin)
add_test(NAME unit_perf COMMAND fnvcoop_tests --group perf)
add_test(NAME unit_perfguard COMMAND fnvcoop_tests --group perfguard)
foreach(n 1 2 3 4 5 6 7 8 9 10)   # perf_guard.h mutation twins: each must make the perfguard group fail
  add_test(NAME twin_perfguard_${n} COMMAND fnvcoop_tests --group perfguard)
  set_tests_properties(twin_perfguard_${n} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${n}")
endforeach()
# End-to-end twin: the real timing tests with every budget 10x tighter (what a 10x slower code base looks like) must STILL fail, load guard or not.
add_test(NAME twin_perf_regression_still_fails COMMAND fnvcoop_tests --group perf --name heavy_operations_with_an_8k_entry_world_are_cheap_or_offloadable)
set_tests_properties(twin_perf_regression_still_fails PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_PERF_SCALE=0.1")
add_test(NAME unit_handshake COMMAND fnvcoop_tests --group handshake)
add_test(NAME unit_desync COMMAND fnvcoop_tests --group desync)
add_test(NAME unit_desync_clientonly COMMAND fnvcoop_tests --group desync_clientonly --name client_only_keys_with_an_inverse_converge_even_above_the_repair_cap)
# Issue #25 (fixed): client-only quest running keys are settled by the host's own running state (twin unit_desync_clientonly_quest_running_reverted below).
add_test(NAME unit_desync_client_only_quest_keys COMMAND fnvcoop_tests --group desync_clientonly --name client_only_quest_running_keys_converge)
# Client-only "SetStage q = 0" keys (the real game: 275 of them, a repair of ~200 effects every digest round, the host never ok): the host commits the same stage 0 (twin unit_desync_clientonly_stage_zero_reverted below).
add_test(NAME unit_desync_client_only_stage_zero COMMAND fnvcoop_tests --group desync_clientonly --name client_only_stage_zero_keys_converge)
# Client-only real quest state (non-zero SetStage, CompleteQuest, FailQuest): adopted only after the host engine confirms it (twin unit_desync_clientonly_quest_state_reverted below).
add_test(NAME unit_desync_client_only_stage_nonzero COMMAND fnvcoop_tests --group desync_clientonly --name client_only_nonzero_stage_keys_converge_when_the_host_engine_agrees)
add_test(NAME unit_desync_client_only_completed COMMAND fnvcoop_tests --group desync_clientonly --name client_only_completed_quest_keys_converge_when_the_host_engine_agrees)
add_test(NAME unit_desync_client_only_failed COMMAND fnvcoop_tests --group desync_clientonly --name client_only_failed_quest_keys_converge_when_the_host_engine_agrees)
add_test(NAME unit_desync_client_only_state_disagrees COMMAND fnvcoop_tests --group desync_clientonly --name client_only_quest_state_the_host_engine_disagrees_with_is_not_adopted)
add_test(NAME unit_desync_client_only_state_no_reader COMMAND fnvcoop_tests --group desync_clientonly --name client_only_quest_state_without_a_working_host_reader_adopts_nothing)
# Client-only objectives: flags 0 is adopted outright (twin unit_desync_clientonly_objective_zero_reverted), real flags / all done only after the host engine confirms (twin unit_desync_clientonly_objective_state_reverted).
add_test(NAME unit_desync_client_only_objective_zero COMMAND fnvcoop_tests --group desync_clientonly --name client_only_objective_zero_keys_converge)
add_test(NAME unit_desync_client_only_objective_nonzero COMMAND fnvcoop_tests --group desync_clientonly --name client_only_nonzero_objective_keys_converge_when_the_host_engine_agrees)
add_test(NAME unit_desync_client_only_objective_all_done COMMAND fnvcoop_tests --group desync_clientonly --name client_only_all_objectives_done_keys_converge_when_the_host_engine_agrees)
add_test(NAME unit_desync_client_only_objective_disagrees COMMAND fnvcoop_tests --group desync_clientonly --name client_only_objective_state_the_host_engine_disagrees_with_is_not_adopted)
add_test(NAME unit_desync_client_only_objective_no_reader COMMAND fnvcoop_tests --group desync_clientonly --name client_only_objective_state_without_a_working_host_reader_adopts_nothing)
# Client-only SetScriptVar (the 2026-10-26 two-PC benchmark: 256 revert keys per digest round, 1180 repair effects per minute, a mismatch in idle): 0 is adopted outright
# (twin unit_desync_clientonly_script_var_reverted), non-zero only after the host engine reads the same value, nothing without an inverse is resent (twin unit_desync_clientonly_no_inverse_resent).
add_test(NAME unit_desync_client_only_script_var_zero COMMAND fnvcoop_tests --group desync_clientonly --name client_only_zero_script_vars_converge_without_any_reader)
add_test(NAME unit_desync_client_only_script_var_zero_reader COMMAND fnvcoop_tests --group desync_clientonly --name client_only_zero_script_vars_the_host_engine_shows_at_zero_converge)
add_test(NAME unit_desync_client_only_script_var_zero_other COMMAND fnvcoop_tests --group desync_clientonly --name client_only_zero_script_vars_the_host_engine_holds_differently_are_not_adopted)
add_test(NAME unit_desync_client_only_script_var_nonzero COMMAND fnvcoop_tests --group desync_clientonly --name client_only_nonzero_script_vars_converge_when_the_host_engine_agrees)
add_test(NAME unit_desync_client_only_script_var_disagrees COMMAND fnvcoop_tests --group desync_clientonly --name client_only_nonzero_script_vars_the_host_engine_disagrees_with_are_not_adopted)
add_test(NAME unit_desync_client_only_script_var_no_reader COMMAND fnvcoop_tests --group desync_clientonly --name client_only_nonzero_script_vars_without_a_host_reader_adopt_nothing_and_resend_nothing)
add_test(NAME unit_desync_client_only_no_inverse_not_resent COMMAND fnvcoop_tests --group desync_clientonly --name unrevertable_client_only_keys_are_not_resent_every_round)
add_test(NAME unit_desync_client_only_script_var_idle COMMAND fnvcoop_tests --group desync_clientonly --name idle_play_after_the_benchmark_shaped_script_var_drift_has_no_mismatch_and_no_repair)
# 2026-10-27 two-PC benchmark, after the script var fix: keys the host can neither adopt nor repair are acknowledged once (no mismatch in every round), keys repaired in vain
# are given up on (no resend for ever), client-only globals and quest running flags are settled from the host engine, settling is not repair (twins: unit_desync_clientonly_known_divergence_never_acknowledged,
# unit_desync_clientonly_global_reverted, unit_desync_clientonly_host_stops_the_clients_running_quests_blindly, unit_desync_clientonly_settled_keys_counted_as_repair_effects, unit_desync_settle_*).
add_test(NAME unit_desync_client_only_ack COMMAND fnvcoop_tests --group desync_clientonly --name known_divergences_are_acknowledged_once_and_the_digest_goes_ok)
add_test(NAME unit_desync_client_only_ack_withdrawn COMMAND fnvcoop_tests --group desync_clientonly --name an_acknowledged_divergence_is_withdrawn_when_the_client_value_changes)
add_test(NAME unit_desync_client_only_globals COMMAND fnvcoop_tests --group desync_clientonly --name client_only_globals_converge_when_the_host_engine_agrees)
add_test(NAME unit_desync_client_only_globals_unsettled COMMAND fnvcoop_tests --group desync_clientonly --name client_only_globals_the_host_engine_disagrees_with_or_cannot_read_are_not_adopted_and_not_resent)
add_test(NAME unit_desync_client_only_running_reader COMMAND fnvcoop_tests --group desync_clientonly --name a_host_engine_reader_keeps_the_clients_running_quests_running)
add_test(NAME unit_desync_client_only_running_unread COMMAND fnvcoop_tests --group desync_clientonly --name quests_the_host_engine_cannot_read_are_left_alone_then_acknowledged)
add_test(NAME unit_desync_client_only_bench_world COMMAND fnvcoop_tests --group desync_clientonly --name the_benchmark_world_settles_in_a_few_rounds_and_then_stays_quiet)
add_test(NAME unit_desync_client_only_clock_volatile COMMAND fnvcoop_tests --group desync_clientonly --name the_clock_global_on_the_volatile_list_never_mismatches)
add_test(NAME unit_desync_client_only_gave_up COMMAND fnvcoop_tests --group desync_clientonly --name a_host_global_the_client_dump_cannot_carry_is_given_up_after_a_few_repairs)
add_test(NAME unit_desync_client_only_settled_not_repair COMMAND fnvcoop_tests --group desync_clientonly --name settled_keys_are_not_counted_as_repair_effects)
# Prepared-save load: a client resume cursor from another timeline is rebased (twin prepared_load_trusts_foreign_cursor below).
add_test(NAME unit_preparedload COMMAND fnvcoop_tests --group preparedload)
add_test(NAME unit_rules COMMAND fnvcoop_tests --group rules)
add_test(NAME unit_rules_wave1 COMMAND fnvcoop_tests --group rules_w1)
add_test(NAME unit_rules_w3b COMMAND fnvcoop_tests --group rules_w3b)
add_test(NAME unit_saveload COMMAND fnvcoop_tests --group saveload)
add_test(NAME unit_baseline COMMAND fnvcoop_tests --group baseline)
add_test(NAME unit_transport COMMAND fnvcoop_tests --group transport)
# Steam transport (docs/steam.md) on a pretend Steam; the steamtwin group re-introduces one bug at a time and asserts the guarding check fails.
add_test(NAME unit_steam COMMAND fnvcoop_tests --group steam)
add_test(NAME twin_steam COMMAND fnvcoop_tests --group steamtwin)
add_test(NAME sim_seeds     COMMAND fnvcoop_tests --group sim)
foreach(twin round3_companion_vet_ignored hostload_unit_set_weather hostload_unit_set_time_scale hostload_unit_ref_script_var hostload_unit_set_map_marker hostload_unit_lock hostload_unit_object_state_open hostload_unit_object_state_sticky_open hostload_unit_ref_add_item hostload_unit_ref_remove_item hostload_unit_remove_all_items hostload_unit_clear_pre_travels hostload_sim_clear_pre_travels hostload_unit_spawn_ref hostload_unit_despawn_ref hostload_sim_set_weather hostload_sim_set_time_scale hostload_sim_ref_script_var hostload_sim_set_map_marker hostload_sim_object_state_open hostload_sim_ref_inventory hostload_sim_spawn_despawn origins_ignored round3_origins_ignored round3_ref_inv_preimage_ignored round3_spawn_rebase_ignored no_echo_guard no_idempotency stage_last_wins flags_and kills_replace apply_out_of_order lower_stage_swallowed
             unit_merge unit_objective_or unit_idempotency unit_echo no_guest_suppression unit_ghost guest_runs_quest_scripts baseline_skipped unit_volatile unit_volatile_ref_script_vars no_epoch_bump unit_epoch unit_flood rules_barrier_release_early a_guest_pass_list_that_ignores_released_barriers_passes_forever rules_disabled rules_extended_disabled rules_tether_ignores_anchor rules_stages_param_ignored rules_sum_as_any rules_exclusive_last_wins rules_ready_proceeds_alone rules_slideshow_no_hold rules_per_player_scene_shared rules_per_player_vars_shared rules_payment_shares_all rules_w1_cause_not_resolved rules_w1_choice_initiator_decides rules_w1_perk_route_ignored rules_w1_acting_copy_shared rules_w1_tether_ignores_world rules_w1_anchor_no_hysteresis rules_w1_pass_intent_ignored rules_w1_ready_info_unmapped rules_w1_party_owner_ignored rules_w1_host_level_as_initiator rules_w1_friendly_fire_off rules_w1_death_policy_off rules_w1_per_player_state_kills_all rules_w1_controls_scope_ignored rules_w1_controls_release_all rules_w1_shipped_cause rules_w1_shipped_perks rules_w1_shipped_exclusive rules_w1_shipped_pooled rules_w1_shipped_pass rules_w1_shipped_friendly rules_w1_shipped_party_owner rules_w1_shipped_host_level rules_w1_shipped_scope rules_w1_shipped_collar rules_w1_tether_mode_ignored unit_heal_disabled heal_disabled unit_chatty_baseline unit_handshake unit_slow_digest fail_not_completed round3_spawn_resurrects round3_sticky_objects_reset round3_ref_inv_no_clear round3_active_quest_shared round3_ignore_follower_caps round3_disconnect_keeps_lock round3_guest_local_replicated round3_angle_keeps_first angle_preimage_ignored round3_angle_preimage_ignored rejoin_sidecar_loses_ledger rejoin_no_reward_ids plain_rejoin_no_reward_ids rejoin_no_epoch_bump rejoin_no_idempotency rejoin_departed_hold_barrier rejoin_barrier_release_early rejoin_dlg_no_join_catchup rejoin_dlg_ignore_result_link rejoin_dlg_drop_end_on_close rejoin_dlg_leak_lock_on_close rejoin_dlg_skip_control_release rejoin_dlg_no_resync rejoin_dlg_replay_duplicate_pick refcmd_keeps_first refcmd_events_become_state refcmd_player_shared refcmd_wide_op_narrowed rules_w3b_collar_timed rules_w3b_collar_zone_timed rules_w3b_per_player_scripts_ignored rules_w3b_pooled_on_origin rules_w3b_once_event_off rules_w3b_warp_range_ignored rules_w3b_shipped_collar_timed rules_w3b_shipped_collar_kills_all rules_w3b_shipped_collar_scripts_ignored rules_w3b_shipped_countdown_shared rules_w3b_shipped_holds_unscoped rules_w3b_shipped_cause_unmapped rules_w3b_shipped_autodoc_perk_route rules_w3b_shipped_autodoc_lock_all rules_w3b_shipped_teleporter_range_ignored rules_w3b_shipped_lr_ede_not_party rules_w3b_shipped_lr_clock_twice rules_w3b_shipped_hh_pooled_on_origin rules_w3b_shipped_hh_sum_as_any rules_w3b_shipped_hh_fail_path_wins rules_w3b_shipped_collar_equip_shared rules_w3b_script_only_copy_shared
             unit_desync_clientonly_heal_disabled unit_desync_clientonly_quest_running_reverted unit_desync_clientonly_stage_zero_reverted unit_desync_clientonly_quest_state_reverted unit_desync_clientonly_objective_zero_reverted unit_desync_clientonly_objective_state_reverted unit_desync_clientonly_script_var_reverted unit_desync_clientonly_no_inverse_resent unit_desync_clientonly_script_var_idle_mismatch unit_desync_clientonly_known_divergence_never_acknowledged unit_desync_clientonly_global_reverted unit_desync_clientonly_host_stops_the_clients_running_quests_blindly unit_desync_clientonly_settled_keys_counted_as_repair_effects unit_desync_settle_no_acknowledgement unit_desync_settle_lowest_shards_first unit_desync_settle_no_follow_up unit_desync_settle_ask_never_shrinks unit_desync_clientonly_repair_forever prepared_load_trusts_foreign_cursor unit_async_completion_leak)
  add_test(NAME twin_${twin} COMMAND fnvcoop_tests --group twin --name ${twin})
endforeach()

# docs/protocol.md effect table must match effect.h / wire.cpp (regenerate: python -I synccore/tools/gen_effect_table.py --write)
find_program(FNVCOOP_PYTHON NAMES python python3)
if(FNVCOOP_PYTHON)
  add_test(NAME drift_effect_table COMMAND ${FNVCOOP_PYTHON} -I ${CMAKE_CURRENT_SOURCE_DIR}/tools/gen_effect_table.py --check)
  add_test(NAME twin_drift_effect_table COMMAND ${FNVCOOP_PYTHON} -I ${CMAKE_CURRENT_SOURCE_DIR}/tools/gen_effect_table.py --check --mutate)
  set_tests_properties(twin_drift_effect_table PROPERTIES WILL_FAIL TRUE)
  # The real ENet transport and the session protocol through the UDP chaos relay (tools/chaos), seeded profiles, two clients, 150 txns.
  # 'hell' is not gated: see docs/tools/T5-chaos.md ("Through the relay"). A twin passes (exit 0) when its deliberate breakage is caught.
  set(_relay_py ${CMAKE_CURRENT_SOURCE_DIR}/../tools/chaos/enet_relay.py)
  foreach(_p bad_wifi mobile_tether transatlantic)
    add_test(NAME loopback_relay_${_p} COMMAND ${FNVCOOP_PYTHON} -I ${_relay_py} --loopback $<TARGET_FILE:fnvcoop_loopback> --profile ${_p})
  endforeach()
  add_test(NAME twin_loopback_relay_bypass    COMMAND ${FNVCOOP_PYTHON} -I ${_relay_py} --loopback $<TARGET_FILE:fnvcoop_loopback> --profile bad_wifi --twin bypass)
  add_test(NAME twin_loopback_relay_partition COMMAND ${FNVCOOP_PYTHON} -I ${_relay_py} --loopback $<TARGET_FILE:fnvcoop_loopback> --profile bad_wifi --twin partition --timeout 20)
  # The twins must not count an unrelated failure as caught: with python itself as the "loopback" no host ever starts, so both must exit 1.
  foreach(_t bypass partition)
    add_test(NAME twin_loopback_relay_${_t}_needs_its_own_failure COMMAND ${FNVCOOP_PYTHON} -I ${_relay_py} --loopback ${FNVCOOP_PYTHON} --profile bad_wifi --twin ${_t} --timeout 5)
    set_tests_properties(twin_loopback_relay_${_t}_needs_its_own_failure PROPERTIES WILL_FAIL TRUE)
  endforeach()
  # The same with the real clients running: the host's port is taken, so only the host fails, and the clients' connect datagrams
  # still reach the partitioned relay as drops. The partition twin must still not count that as caught.
  add_test(NAME twin_loopback_relay_partition_needs_a_host COMMAND ${FNVCOOP_PYTHON} -I ${_relay_py} --loopback $<TARGET_FILE:fnvcoop_loopback> --profile bad_wifi --twin partition --timeout 5 --occupy-host-port)
  set_tests_properties(twin_loopback_relay_partition_needs_a_host PROPERTIES WILL_FAIL TRUE)
endif()

# T5 chaos mirror (synccore/chaos/CMakeLists.txt)
add_subdirectory(chaos)

# combat-core: game-free combat rules (synccore/combat/CMakeLists.txt); before enginemodel, which links fnvcoop_combat
add_subdirectory(combat)

# companions-crime: followers, crime / alarms / bounty / hostility, kill rewards (synccore/social/CMakeLists.txt); after combat, which it links
add_subdirectory(social)

# T2a: fake game engine model + actor ownership tests (own CMakeLists; ownership.cpp itself is in the core library above)
add_subdirectory(enginemodel)

# T1a flight recorder (synccore/flight/CMakeLists.txt); after enginemodel: it adapts T2a's observer
add_subdirectory(flight)

# live-core: actor snapshots, delta codec, interpolation, interest management (synccore/live/CMakeLists.txt)
add_subdirectory(live)

# spawn-arb: spawn-site arbitration, server-issued net ids, respawn / cell reset rules (synccore/spawn/CMakeLists.txt)
add_subdirectory(spawn)

# loot: containers, corpses, dropped items, pickups, physics clutter; idempotent transfers, locks, claims (synccore/loot/CMakeLists.txt)
add_subdirectory(loot)

# area-desync: per-area digests, targeted diff and heal (synccore/areadigest/CMakeLists.txt); after flight/, the heal counters are exported there
add_subdirectory(areadigest)

# Precompiled headers for the heavy std headers (only in the shared tree, tools/build/pch.cmake; a no-op for standalone builds).
# Targets of subdirectories are handled in their own CMakeLists; these are this directory's.
if(COMMAND fnvcoop_pch)
  foreach(_t fnvcoop fnvcoop_sim fnvcoop_tests)
    fnvcoop_pch(${_t})
  endforeach()
endif()

# ctest -j: timing-sensitive tests run alone, everything else (about 150 twin tests of a few seconds each) can overlap.
foreach(_t unit_perf twin_perf_regression_still_fails soak_smoke soak_smoke_threaded soak_late_disconnect twin_soak_late_disconnect loopback_clean loopback_faults loopback_unreliable
        loopback_relay_bad_wifi loopback_relay_mobile_tether loopback_relay_transatlantic twin_loopback_relay_bypass twin_loopback_relay_partition)
  if(TEST ${_t})
    set_tests_properties(${_t} PROPERTIES RUN_SERIAL TRUE)
  endif()
endforeach()
