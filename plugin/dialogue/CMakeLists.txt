# fnvcoop_dlg: dialogue sync protocol + conversation state machines (L8). Header-only; tests are game-free.
cmake_minimum_required(VERSION 3.20)
if(NOT PROJECT_NAME)
  project(fnvcoop_dialogue CXX)
endif()
add_library(fnvcoop_dlg INTERFACE)
target_include_directories(fnvcoop_dlg INTERFACE include ${CMAKE_CURRENT_LIST_DIR}/../../synccore/include)
target_compile_features(fnvcoop_dlg INTERFACE cxx_std_17)

include(CTest)
add_executable(dialogue_tests tests/main.cpp)
target_link_libraries(dialogue_tests PRIVATE fnvcoop_dlg)
add_test(NAME dialogue_unit COMMAND dialogue_tests --group unit)
foreach(twin no_npc_lock anyone_picks replay_duplicate_pick drop_end_on_close no_topic_gate
             spectator_stale_lines skip_control_release leak_lock_on_close
             spectators_see_commands non_owner_commands no_preempt no_owner_rewrite cap_is_party_wide anyone_dismisses
             keep_companions_of_leaver menu_on_host menu_to_spectators anyone_replies_menu resume_menu_twice any_menu_choice
             no_menu_default no_menu_default_disconnect
             forced_no_dedupe forced_start_to_all forced_no_preempt forced_always_host skip_consent no_menu_default_consent
             anyone_replies_consent
             vote_is_unanimous vote_is_unanimous_dissent vote_no_host_tie_break vote_timeout_approves vote_skip_consent actor_asks_party
             no_choice_consent no_choice_consent_denied
             no_info_gate held_end_never_released held_end_runs_on_deny held_end_never_released_goodbye vote_answer_dropped vote_answer_inverted
             box_gate_ignored rules_owner_keeps_old_hold no_menu_default_rules
             party_companion_counts party_hired_per_slot)
  add_test(NAME dialogue_twin_${twin} COMMAND dialogue_tests --group twin --name ${twin})
endforeach()

# Engine glue: game-free layout reader + EngineGlue (mock-tested), and the adapter over fnvcoop::ITransport.
set(FNVCOOP_SYNCCORE_DIR "${CMAKE_CURRENT_LIST_DIR}/../../synccore")
add_library(fndlg_engine_core STATIC engine/info_reader.cpp)
target_include_directories(fndlg_engine_core PUBLIC engine)
target_link_libraries(fndlg_engine_core PUBLIC fnvcoop_dlg)
add_library(fndlg_transport STATIC ${FNVCOOP_SYNCCORE_DIR}/src/transport.cpp)
target_include_directories(fndlg_transport PUBLIC ${FNVCOOP_SYNCCORE_DIR}/include)
foreach(t adapter_tests engine_tests)
  set(src tests/adapter_test.cpp)
  if(t STREQUAL engine_tests)
    set(src tests/engine_test.cpp)
  endif()
  add_executable(${t} ${src})
  target_link_libraries(${t} PRIVATE fndlg_engine_core fndlg_transport)
  add_test(NAME dialogue_${t} COMMAND ${t} WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR})
endforeach()
# engine/engine_xnvse.cpp is added to the nvcoop target by plugin/CMakeLists.txt (needs xNVSE headers, Win32 only):
#   target_sources(nvcoop PRIVATE dialogue/engine/engine_xnvse.cpp)  + include dirs dialogue/include dialogue/engine
