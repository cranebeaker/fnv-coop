# plugin/actors: actor ownership in the plugin (Milestone 3). Works two ways:
#   * included by plugin/CMakeLists.txt: adds nvcoop_actors_core (and, when the coop module is built, wires coop.owner + the per-frame hook)
#   * standalone, for the game-free tests + mutation twins:  plugin\actors\run_tests.cmd
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_actors C CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(NVA_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVA_REPO "${NVA_DIR}/../..")

function(nva_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

# ---- synccore (ownership service, wire, effects) + live-core: reuse the coop lane's lib inside the plugin build ----
if(TARGET nvcoop_coop_synccore)
  set(NVA_SYNC nvcoop_coop_synccore)
else()
  file(GLOB NVA_SYNC_SRC CONFIGURE_DEPENDS ${NVA_REPO}/synccore/src/*.cpp)
  list(FILTER NVA_SYNC_SRC EXCLUDE REGEX "enet_transport\\.cpp$")
  add_library(nvcoop_actors_synccore STATIC ${NVA_SYNC_SRC})
  target_include_directories(nvcoop_actors_synccore PUBLIC ${NVA_REPO}/synccore/include ${NVA_REPO}/synccore)
  nva_flags(nvcoop_actors_synccore)
  set(NVA_SYNC nvcoop_actors_synccore)
endif()
add_library(nvcoop_actors_live STATIC ${NVA_REPO}/synccore/live/live.cpp ${NVA_REPO}/synccore/live/entity_map.cpp)
target_include_directories(nvcoop_actors_live PUBLIC ${NVA_REPO}/synccore ${NVA_REPO}/synccore/include)
target_link_libraries(nvcoop_actors_live PUBLIC ${NVA_SYNC})
nva_flags(nvcoop_actors_live)

set(NVA_INC "${NVA_DIR}/.." "${NVA_REPO}/synccore" "${NVA_REPO}/synccore/include")

# ---- the module itself (shipped) ----
add_library(nvcoop_actors_core STATIC ${NVA_DIR}/actor_sync.cpp ${NVA_DIR}/actor_wire.cpp ${NVA_DIR}/state_parts.cpp ${NVA_DIR}/owner_rpc.cpp ${NVA_DIR}/actor_runtime.cpp)
target_include_directories(nvcoop_actors_core PUBLIC ${NVA_INC})
target_link_libraries(nvcoop_actors_core PUBLIC nvcoop_actors_live)
nva_flags(nvcoop_actors_core)

# ---- tests + mutation twins. The unmutated files and the test file are compiled once (OBJECT libs); each twin recompiles only
#      actor_sync.cpp with NVA_MUT=<n> and must FAIL the whole suite. ----
set(NVA_MUTANTS 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 20 21 22 23 24 25 26 27 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 64 67 69 72 73 80 81 82 83 84 85)
set(NVA_TEST_EXCL "")
if(TARGET nvcoop)
  set(NVA_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
add_library(nva_base_objs OBJECT ${NVA_TEST_EXCL} ${NVA_DIR}/actor_wire.cpp ${NVA_DIR}/state_parts.cpp ${NVA_DIR}/owner_rpc.cpp ${NVA_DIR}/actor_runtime.cpp)
add_library(nva_test_objs OBJECT ${NVA_TEST_EXCL} ${NVA_DIR}/tests/test_actors.cpp ${NVA_REPO}/synccore/tests/main.cpp)
foreach(t nva_base_objs nva_test_objs)
  target_include_directories(${t} PRIVATE ${NVA_INC} ${NVA_REPO}/synccore/tests ${NVA_DIR}/tests)
  nva_flags(${t})
endforeach()

foreach(v "" ${NVA_MUTANTS})
  if(v STREQUAL "")
    set(sfx "")
  else()
    set(sfx "_mut${v}")
  endif()
  add_library(nva_sync_objs${sfx} OBJECT ${NVA_TEST_EXCL} ${NVA_DIR}/actor_sync.cpp)
  target_include_directories(nva_sync_objs${sfx} PRIVATE ${NVA_INC})
  nva_flags(nva_sync_objs${sfx})
  if(NOT v STREQUAL "")
    target_compile_definitions(nva_sync_objs${sfx} PRIVATE NVA_MUT=${v})
  endif()
  add_executable(nvcoop_actors_tests${sfx} ${NVA_TEST_EXCL} $<TARGET_OBJECTS:nva_test_objs> $<TARGET_OBJECTS:nva_base_objs> $<TARGET_OBJECTS:nva_sync_objs${sfx}>)
  target_link_libraries(nvcoop_actors_tests${sfx} PRIVATE nvcoop_actors_live)
  nva_flags(nvcoop_actors_tests${sfx})
  set_target_properties(nvcoop_actors_tests${sfx} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/actors_tests")
  foreach(cfg _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
    set_target_properties(nvcoop_actors_tests${sfx} PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/actors_tests")
  endforeach()
endforeach()

enable_testing()
add_test(NAME actors_unit COMMAND nvcoop_actors_tests)
foreach(v ${NVA_MUTANTS})
  add_test(NAME actors_mut${v} COMMAND nvcoop_actors_tests_mut${v})
  set_tests_properties(actors_mut${v} PROPERTIES WILL_FAIL TRUE)
endforeach()

# ---- plugin glue: coop.owner + the per-frame hook (x86, linked into nvcoop.dll next to the coop module) ----
if(TARGET nvcoop AND TARGET nvcoop_coop_core)
  target_link_libraries(nvcoop PRIVATE nvcoop_actors_core)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_ACTORS=1)
  target_include_directories(nvcoop PRIVATE ${NVA_INC})
  target_sources(nvcoop PRIVATE ${NVA_DIR}/actors_glue.cpp)
endif()

# ---- the real engine port (plugin/actors/engine): only in the plugin / shared tree, never in the standalone ownership tests ----
if(TARGET nvcoop AND TARGET nvcoop_hooks_core AND EXISTS "${NVA_DIR}/engine/CMakeLists.txt")
  add_subdirectory(engine)
endif()

# ---- NPC animation layer (plugin/actors/anim, enemy-anim): sampler, driver, corpse bookkeeping, script backend, tests ----
if(EXISTS "${NVA_DIR}/anim/CMakeLists.txt")
  add_subdirectory(anim)
endif()
