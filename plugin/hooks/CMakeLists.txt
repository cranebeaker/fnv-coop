# L5 hooks. Works two ways:
#   * included by plugin/CMakeLists.txt (adds nvcoop_hooks_core / nvcoop_hooks_game targets)
#   * standalone, for the game-free unit tests (any arch, any generator):
#       cmake -S plugin/hooks -B build/L5 && cmake --build build/L5 --config Release && ctest --test-dir build/L5 -C Release
#     or just run plugin\hooks\run_tests.bat
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_hooks CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(NVH_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVH_SYNCCORE_INC "${NVH_DIR}/../../synccore/include")

function(nvh_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS)
endfunction()

# ---- synccore, compiled for the tests only (rules_reducer.h drives a real RulesEngine); the plugin links its own copy elsewhere ----
set(NVH_SC "${NVH_DIR}/../../synccore")
if(NOT TARGET nvh_synccore)
  add_library(nvh_synccore STATIC ${NVH_SC}/src/wire.cpp ${NVH_SC}/src/shadow.cpp ${NVH_SC}/src/policy.cpp ${NVH_SC}/src/storylog.cpp
    ${NVH_SC}/src/protocol.cpp ${NVH_SC}/src/rules.cpp ${NVH_SC}/src/baseline.cpp ${NVH_SC}/src/replicator.cpp ${NVH_SC}/src/transport.cpp
    ${NVH_SC}/src/transport_util.cpp ${NVH_SC}/src/session.cpp)
  target_include_directories(nvh_synccore PUBLIC ${NVH_SYNCCORE_INC})
  target_compile_features(nvh_synccore PUBLIC cxx_std_17)
  if(MSVC)
    target_compile_options(nvh_synccore PRIVATE /W3 /EHsc /permissive- /utf-8 /wd4456 /D_CRT_SECURE_NO_WARNINGS)
  endif()
  if(COMMAND fnvcoop_pch)
    fnvcoop_pch(nvh_synccore)
  endif()
endif()

# ---- game-free core: prologue-checked patcher + capture hub (+ mutant twin) ----
foreach(v "" _mutant)
  add_library(nvcoop_hooks_core${v} STATIC ${NVH_DIR}/hook_patch.cpp ${NVH_DIR}/capture_hub.cpp ${NVH_DIR}/effect_json.cpp ${NVH_DIR}/suppress_policy.cpp ${NVH_DIR}/change_tracker.cpp ${NVH_DIR}/apply_lines.cpp ${NVH_DIR}/inventory_diff.cpp ${NVH_DIR}/guest_local_run.cpp ${NVH_DIR}/guest_objvars.cpp ${NVH_DIR}/party_cond.cpp)
  target_include_directories(nvcoop_hooks_core${v} PUBLIC ${NVH_DIR} ${NVH_DIR}/.. ${NVH_SYNCCORE_INC} ${NVH_DIR}/../../tools/mut)
  nvh_flags(nvcoop_hooks_core${v})
  if(COMMAND fnvcoop_pch)
    fnvcoop_pch(nvcoop_hooks_core${v})
  endif()
  add_executable(nvcoop_hooks_tests${v} ${NVH_DIR}/tests/test_hooks.cpp)
  target_link_libraries(nvcoop_hooks_tests${v} PRIVATE nvcoop_hooks_core${v} nvh_synccore)
  target_compile_definitions(nvcoop_hooks_tests${v} PRIVATE NVH_DATA_DIR="${NVH_DIR}/data" NVH_RULES_JSON="${NVH_DIR}/../../docs/quest-policies/rules.json" NVH_CAUSE_QUESTS_JSON="${NVH_DIR}/../../docs/quest-policies/cause_quests.json")
  nvh_flags(nvcoop_hooks_tests${v})
endforeach()
target_compile_definitions(nvcoop_hooks_core_mutant PUBLIC NVHOOKS_MUTANT=1)
if(NOT CMAKE_RUNTIME_OUTPUT_DIRECTORY)
  # keep test exes out of L1's out/L1 deployables
  set_target_properties(nvcoop_hooks_tests nvcoop_hooks_tests_mutant PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/hooks_tests")
endif()
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_hooks_tests nvcoop_hooks_tests_mutant PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/hooks_tests")
endforeach()

enable_testing()
add_test(NAME hooks_unit COMMAND nvcoop_hooks_tests)
add_test(NAME hooks_unit_mutation_twin COMMAND nvcoop_hooks_tests_mutant)
set_tests_properties(hooks_unit_mutation_twin PROPERTIES WILL_FAIL TRUE)

# ---- hook chaining twins (hook_patch.cpp rule 4), one at a time: NVHOOKS_CHAIN_MUT=<n> picks the broken variant at run time.
# Only this test copy compiles them in (NVHOOKS_CHAIN_TWINS); the plugin and every other build get chainMut() == 0. ----
if(NOT TARGET nvcoop_hooks_core_twins AND CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  add_library(nvcoop_hooks_core_twins STATIC ${NVH_DIR}/hook_patch.cpp ${NVH_DIR}/capture_hub.cpp ${NVH_DIR}/effect_json.cpp ${NVH_DIR}/suppress_policy.cpp ${NVH_DIR}/change_tracker.cpp ${NVH_DIR}/apply_lines.cpp ${NVH_DIR}/inventory_diff.cpp ${NVH_DIR}/guest_local_run.cpp ${NVH_DIR}/guest_objvars.cpp ${NVH_DIR}/party_cond.cpp)
  target_include_directories(nvcoop_hooks_core_twins PUBLIC ${NVH_DIR} ${NVH_DIR}/.. ${NVH_SYNCCORE_INC})
  target_compile_definitions(nvcoop_hooks_core_twins PUBLIC NVHOOKS_CHAIN_TWINS=1)   # PUBLIC: extract_pad.h twin 4 lives in the test TU
  nvh_flags(nvcoop_hooks_core_twins)
  add_executable(nvcoop_hooks_tests_chaintwins ${NVH_DIR}/tests/test_hooks.cpp)
  target_link_libraries(nvcoop_hooks_tests_chaintwins PRIVATE nvcoop_hooks_core_twins nvh_synccore)
  target_compile_definitions(nvcoop_hooks_tests_chaintwins PRIVATE NVH_DATA_DIR="${NVH_DIR}/data" NVH_RULES_JSON="${NVH_DIR}/../../docs/quest-policies/rules.json" NVH_CAUSE_QUESTS_JSON="${NVH_DIR}/../../docs/quest-policies/cause_quests.json")
  nvh_flags(nvcoop_hooks_tests_chaintwins)
  set_target_properties(nvcoop_hooks_tests_chaintwins PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/hooks_tests")
  foreach(cfg _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
    set_target_properties(nvcoop_hooks_tests_chaintwins PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/hooks_tests")
  endforeach()
  add_test(NAME hooks_chain_real COMMAND nvcoop_hooks_tests_chaintwins)
  foreach(n 1 2 3 4)   # 4 = extract_pad.h without padding
    add_test(NAME hooks_chain_twin${n} COMMAND nvcoop_hooks_tests_chaintwins)
    set_tests_properties(hooks_chain_twin${n} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "NVHOOKS_CHAIN_MUT=${n}")
  endforeach()
endif()

# ---- story-capture (docs/tools/story-capture.md): the RefCommand op table, its console-line builder and the capture decoder. Game-free. Twins are
# RUNTIME-switchable (tools/mut/fnvcoop_mut.h, FNVCOOP_MUT=<n>): the test binary links the _rt copy; the game glue (nvcoop_hooks_game) compiles
# ref_ops.cpp itself, with the twins compiled out. ----
set(NVH_STORY_TWINS 1 2 3 4 5 6 7 8 9 10 11)
add_library(nvcoop_story_core_rt STATIC ${NVH_DIR}/ref_ops.cpp)
target_include_directories(nvcoop_story_core_rt PUBLIC ${NVH_DIR} ${NVH_DIR}/.. ${NVH_SYNCCORE_INC} ${NVH_DIR}/../../tools/mut)
target_compile_definitions(nvcoop_story_core_rt PUBLIC FNVCOOP_RUNTIME_MUT)
nvh_flags(nvcoop_story_core_rt)
add_executable(nvcoop_story_tests ${NVH_DIR}/tests/test_story.cpp)
target_link_libraries(nvcoop_story_tests PRIVATE nvcoop_story_core_rt nvcoop_hooks_core nvh_synccore)
target_compile_definitions(nvcoop_story_tests PRIVATE NVH_DATA_DIR="${NVH_DIR}/data")
nvh_flags(nvcoop_story_tests)
if(NOT CMAKE_RUNTIME_OUTPUT_DIRECTORY)
  set_target_properties(nvcoop_story_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/hooks_tests")
endif()
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_story_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/hooks_tests")
endforeach()
add_test(NAME story_unit COMMAND nvcoop_story_tests)
foreach(v ${NVH_STORY_TWINS})
  add_test(NAME story_mut${v} COMMAND nvcoop_story_tests)
  set_tests_properties(story_mut${v} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${v}")
endforeach()

# ---- game glue (x86 only): detours, effect capture hooks, remote-effect apply. Compiled against xNVSE headers. ----
if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND EXISTS "${NVH_DIR}/engine_hooks.cpp")
  set(NVH_NVSE "${NVH_DIR}/../../third_party/NVSE" CACHE PATH "xNVSE checkout")
  if(NOT EXISTS "${NVH_NVSE}/nvse/nvse/GameForms.h")
    message(STATUS "hooks: xNVSE headers not found at ${NVH_NVSE}; nvcoop_hooks_game skipped")
  else()
    add_library(nvcoop_hooks_game STATIC ${NVH_DIR}/engine_hooks.cpp ${NVH_DIR}/apply_remote.cpp ${NVH_DIR}/ref_ops.cpp)
    target_link_libraries(nvcoop_hooks_game PUBLIC nvcoop_hooks_core)
    target_include_directories(nvcoop_hooks_game PUBLIC ${NVH_DIR} ${NVH_DIR}/../../tools/mut
      "${NVH_NVSE}" "${NVH_NVSE}/nvse" "${NVH_NVSE}/nvse/nvse")
    target_compile_definitions(nvcoop_hooks_game PRIVATE RUNTIME=1 RUNTIME_VERSION=0x040020D0 WIN32 _WINDOWS _CRT_SECURE_NO_WARNINGS)
    target_compile_options(nvcoop_hooks_game PRIVATE "SHELL:/FI ${NVH_NVSE}/nvse/nvse/prefix.h" /W3 /EHsc /permissive- /Zc:__cplusplus)
    # Linking into the plugin is harmless: nothing happens until nvhooks::Init() is called by main.cpp (L1).
    if(TARGET nvcoop)
      target_link_libraries(nvcoop PRIVATE nvcoop_hooks_game)
      # bridge methods + role from nvcoop.ini; registered through NVCOOP_MODULE so it must be a direct source
      target_sources(nvcoop PRIVATE ${NVH_DIR}/hooks_module.cpp)
      # GuestLocalRun allow-list data, deployed next to nvcoop.dll (deploy_instances.py copies nvcoop_*.json)
      configure_file(${NVH_DIR}/data/guest_local_run.json ${NVCOOP_OUT_DIR}/nvcoop_guest_local_run.json COPYONLY)
    endif()
  endif()
endif()
