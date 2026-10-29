# L12 hooks_world. Works two ways:
#   * included by plugin/CMakeLists.txt (adds nvcoop_world_core / nvcoop_world_game; links the game glue into nvcoop)
#   * standalone, for the game-free unit tests + mutation twins (any arch):
#       plugin\hooks_world\run_tests.cmd
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_hooks_world CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(NVW_DIR "${CMAKE_CURRENT_LIST_DIR}")
if(NOT TARGET nvcoop_hooks_core)
  # standalone: pull in L5's game-free core (patcher, capture hub, effect json) without building its tests
  add_subdirectory("${NVW_DIR}/../hooks" "${CMAKE_BINARY_DIR}/l5_hooks" EXCLUDE_FROM_ALL)
endif()

function(nvw_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS)
endfunction()

# ---- game-free core + tests. Mutation twins are RUNTIME-switchable (tools/mut/fnvcoop_mut.h): the test code is compiled once
# (nvcoop_world_core_rt / nvcoop_world_tests, FNVCOOP_RUNTIME_MUT) and twin <n> is selected with FNVCOOP_MUT=<n>; each MUST fail.
# nvcoop_world_core is the production lib (twins compiled out); it is what the game glue links. ----
set(NVW_MUTANTS 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80)
set(NVW_MUT_INC "${NVW_DIR}/../../tools/mut")
foreach(sfx "" "_rt")
  add_library(nvcoop_world_core${sfx} STATIC ${NVW_DIR}/world_core.cpp ${NVW_DIR}/world_plan.cpp ${NVW_DIR}/world_query.cpp ${NVW_DIR}/world_refvars.cpp)
  target_include_directories(nvcoop_world_core${sfx} PUBLIC ${NVW_DIR} ${NVW_MUT_INC})
  target_link_libraries(nvcoop_world_core${sfx} PUBLIC nvcoop_hooks_core)
  nvw_flags(nvcoop_world_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_world_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()
add_executable(nvcoop_world_tests ${NVW_DIR}/tests/test_world.cpp)
target_link_libraries(nvcoop_world_tests PRIVATE nvcoop_world_core_rt)
if(TARGET nvcoop_story_core_rt)   # the identity tests read the real RefCommand op table (refOpDefaultsToPlayer)
  target_link_libraries(nvcoop_world_tests PRIVATE nvcoop_story_core_rt)
endif()
nvw_flags(nvcoop_world_tests)
target_compile_definitions(nvcoop_world_tests PRIVATE NVW_DUMP_PATH="${NVW_DIR}/tests/world_code/prologues.txt")
if(NOT CMAKE_RUNTIME_OUTPUT_DIRECTORY)
  set_target_properties(nvcoop_world_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/world_tests")
endif()
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_world_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/world_tests")
endforeach()

enable_testing()
add_test(NAME world_unit COMMAND nvcoop_world_tests)
foreach(v ${NVW_MUTANTS})
  add_test(NAME world_mut${v} COMMAND nvcoop_world_tests)
  set_tests_properties(world_mut${v} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${v}")
endforeach()

# ---- game glue (x86 only): detours, capture, apply. Compiled against xNVSE headers. ----
# world_peek.cpp (raw memory read for address verification) is always built into nvcoop.dll; the hook glue only once
# world_addrs.h carries verified prologues (the file GAME_READY marks that).
if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND TARGET nvcoop_hooks_game AND TARGET nvcoop)
  set(NVW_NVSE "${NVW_DIR}/../../third_party/NVSE" CACHE PATH "xNVSE checkout")
  # NVCOOP_MODULE registration must be a direct source of the final DLL (static-lib objects without references are dropped)
  target_sources(nvcoop PRIVATE ${NVW_DIR}/world_peek.cpp)
  if(EXISTS "${NVW_DIR}/GAME_READY")
    add_library(nvcoop_world_game STATIC ${NVW_DIR}/world_hooks.cpp)
    target_link_libraries(nvcoop_world_game PUBLIC nvcoop_world_core nvcoop_hooks_game)
    target_include_directories(nvcoop_world_game PUBLIC ${NVW_DIR}
      "${NVW_NVSE}" "${NVW_NVSE}/nvse" "${NVW_NVSE}/nvse/nvse")
    target_compile_definitions(nvcoop_world_game PRIVATE RUNTIME=1 RUNTIME_VERSION=0x040020D0 WIN32 _WINDOWS _CRT_SECURE_NO_WARNINGS)
    target_compile_options(nvcoop_world_game PRIVATE "SHELL:/FI ${NVW_NVSE}/nvse/nvse/prefix.h" /W3 /EHsc /permissive- /Zc:__cplusplus)
    target_link_libraries(nvcoop PRIVATE nvcoop_world_game)
    target_sources(nvcoop PRIVATE ${NVW_DIR}/world_module.cpp)
  endif()
  # world_peek.cpp needs the same compile environment as the rest of nvcoop (set on the target already) -- nothing more to do.
endif()
