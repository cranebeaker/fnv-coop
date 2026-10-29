# plugin/loot: loot, containers, dropped items, pickups and physics clutter in the plugin (FULL-SYNC-PLAN 3.5). Works two ways:
#   * included by plugin/CMakeLists.txt: adds nvcoop_loot_core (and, when the actors module is built, the real engine + module that nvcoop.dll links)
#   * standalone, for the game-free tests + mutation twins:  plugin\loot\run_tests.cmd (uses the shared tree build\all)
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_loot C CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(NVL_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVL_REPO "${NVL_DIR}/../..")
set(NVL_MUT_INC "${NVL_REPO}/tools/mut")

function(nvl_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

# ---- synccore (wire, effects) reused from the coop lane's lib inside the plugin build ----
if(TARGET nvcoop_coop_synccore)
  set(NVL_SYNC nvcoop_coop_synccore)
else()
  file(GLOB NVL_SYNC_SRC CONFIGURE_DEPENDS ${NVL_REPO}/synccore/src/*.cpp)
  list(FILTER NVL_SYNC_SRC EXCLUDE REGEX "enet_transport\\.cpp$")
  add_library(nvcoop_loot_synccore STATIC ${NVL_SYNC_SRC})
  target_include_directories(nvcoop_loot_synccore PUBLIC ${NVL_REPO}/synccore/include ${NVL_REPO}/synccore)
  nvl_flags(nvcoop_loot_synccore)
  set(NVL_SYNC nvcoop_loot_synccore)
endif()

# plugin/coop/live (live_hub.h: the service interface) is header-only for us
set(NVL_INC "${NVL_DIR}/.." "${NVL_REPO}/synccore" "${NVL_REPO}/synccore/include" "${NVL_MUT_INC}" "${NVL_REPO}/plugin/coop/live")

# synccore/loot compiled here too (production + runtime-twin copy): the plugin tree is configured before synccore/, so it cannot link
# synccore's own fnvcoop_loot target by name.
foreach(sfx "" "_rt")
  add_library(nvcoop_loot_proto${sfx} STATIC ${NVL_REPO}/synccore/loot/loot.cpp ${NVL_REPO}/synccore/loot/loot_wire.cpp)
  target_include_directories(nvcoop_loot_proto${sfx} PUBLIC ${NVL_INC})
  target_link_libraries(nvcoop_loot_proto${sfx} PUBLIC ${NVL_SYNC})
  nvl_flags(nvcoop_loot_proto${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_loot_proto${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- the game-free module: production (what the plugin links) and the runtime-twin copy (what the tests link) ----
foreach(sfx "" "_rt")
  add_library(nvcoop_loot_core${sfx} STATIC ${NVL_DIR}/loot_sync.cpp)
  target_include_directories(nvcoop_loot_core${sfx} PUBLIC ${NVL_INC})
  target_link_libraries(nvcoop_loot_core${sfx} PUBLIC nvcoop_loot_proto${sfx})
  nvl_flags(nvcoop_loot_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_loot_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests. One exe; twins 101..137 (synccore/loot, also compiled in) and 141..167 (this module) are chosen at run time with FNVCOOP_MUT=<n> ----
set(NVL_TEST_EXCL "")
if(TARGET nvcoop)
  set(NVL_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
add_executable(nvcoop_loot_tests ${NVL_TEST_EXCL} ${NVL_DIR}/tests/test_sync.cpp ${NVL_DIR}/tests/test_session.cpp ${NVL_DIR}/tests/test_menuless.cpp ${NVL_DIR}/tests/test_scarcity.cpp ${NVL_REPO}/synccore/tests/main.cpp)
target_include_directories(nvcoop_loot_tests PRIVATE ${NVL_REPO}/synccore/tests ${NVL_DIR}/tests)
target_link_libraries(nvcoop_loot_tests PRIVATE nvcoop_loot_core_rt)
nvl_flags(nvcoop_loot_tests)
if(NOT CMAKE_RUNTIME_OUTPUT_DIRECTORY)
  set_target_properties(nvcoop_loot_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/loot_plugin_tests")
endif()
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_loot_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/loot_plugin_tests")
endforeach()

enable_testing()
add_test(NAME loot_plugin_unit COMMAND nvcoop_loot_tests)
foreach(v RANGE 141 167)
  add_test(NAME loot_plugin_mut${v} COMMAND nvcoop_loot_tests)
  set_tests_properties(loot_plugin_mut${v} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${v}")
endforeach()
foreach(v 170 172 173 174 175 1490 1491 1492 1493 1494)   # the placed-item pickup twins (170..173 live in synccore/loot, 174 / 175 here), the scarcity hook twins 1490..1494 (test_scarcity.cpp)
  add_test(NAME loot_plugin_mut${v} COMMAND nvcoop_loot_tests)
  set_tests_properties(loot_plugin_mut${v} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${v}")
endforeach()

# ---- plugin glue: the real engine over xNVSE + the module (bridge method, OnDrop / OnActivate handlers). Linked into nvcoop.dll (x86). ----
# Inert unless nvcoop.ini has [loot] enabled=1. dll-wiring adds nvcoop::loot::service() to the live hub (one line, see loot_glue.h).
if(TARGET nvcoop AND TARGET nvcoop_actors_core)
  target_link_libraries(nvcoop PRIVATE nvcoop_loot_core)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_LOOT=1)
  target_include_directories(nvcoop PRIVATE ${NVL_INC})
  target_sources(nvcoop PRIVATE ${NVL_DIR}/loot_engine_real.cpp ${NVL_DIR}/loot_module.cpp)
endif()
