# live-core: actor snapshots, delta codec, interpolation buffer, interest management, channel mapping.
# Game-free. Included from synccore/CMakeLists.txt (after enable_testing()).
add_library(fnvcoop_live STATIC live.cpp)
target_include_directories(fnvcoop_live PUBLIC ${CMAKE_CURRENT_LIST_DIR})
target_link_libraries(fnvcoop_live PUBLIC fnvcoop)

add_executable(live_tests tests/test_live.cpp tests/test_interp.cpp tests/test_lossy.cpp tests/test_twins.cpp tests/test_anim.cpp ${CMAKE_CURRENT_LIST_DIR}/../tests/main.cpp)
target_include_directories(live_tests PRIVATE ${CMAKE_CURRENT_LIST_DIR}/../tests ${CMAKE_CURRENT_LIST_DIR}/../chaos)
target_link_libraries(live_tests PRIVATE fnvcoop_live)
if(MSVC)
  foreach(t fnvcoop_live live_tests)
    target_compile_options(${t} PRIVATE /W4 /permissive- /utf-8 /wd4456 /D_CRT_SECURE_NO_WARNINGS)
  endforeach()
else()
  target_compile_options(fnvcoop_live PRIVATE -Wall -Wextra)
  target_compile_options(live_tests PRIVATE -Wall -Wextra)
endif()

add_test(NAME live_codec COMMAND live_tests --group codec)
add_test(NAME live_receiver COMMAND live_tests --group receiver)
add_test(NAME live_interp COMMAND live_tests --group interp)
add_test(NAME live_interest COMMAND live_tests --group interest)
add_test(NAME live_channel COMMAND live_tests --group channel)
add_test(NAME live_lossy COMMAND live_tests --group lossy)
add_test(NAME live_anim COMMAND live_tests --group anim)
foreach(twin delta_from_latest accept_stale_seq accept_stale_epoch accept_non_owner accept_forged_future_epoch no_extrapolation_cap extrapolate_only
             no_snap no_smooth snap_ignores_cell near_rate_as_far no_far_tier ack_ignored drop_av_changes no_resync_request
             drop_anim_ext drop_anim_ext_one_shot anim_ext_no_delta accept_bad_anim_ext
             drop_sit drop_sit_loss sit_no_delta accept_bad_sit
             lossy_stale_seq lossy_stale_epoch lossy_non_owner lossy_delta_from_latest lossy_drop_av lossy_no_extrap_cap
             ll_fixed_delay ll_no_damping ll_no_blend ll_no_slew ll_old_interval no_teleport_snap)
  add_test(NAME twin_live_${twin} COMMAND live_tests --group twin --name ${twin})
endforeach()
