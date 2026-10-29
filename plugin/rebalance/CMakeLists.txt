# plugin/rebalance: the co-op rebalance mode (docs/rebalance.md). Works two ways:
#   * included by plugin/CMakeLists.txt (shared tree build/all): adds the game-free core, the tests, and (x86, nvcoop target present) the engine glue
#   * standalone, for the game-free tests + mutation twins:  plugin\rebalance\run_tests.cmd
# Targets
#   nvcoop_rebalance_core / _rt   campaign setting, profile, ramp, GMST applier, Stimpak effect applier, loot scarcity rule, bigger fights, director (production / FNVCOOP_RUNTIME_MUT)
#   nvcoop_rebalance_tests        EXCLUDE_FROM_ALL in the plugin build; twins are switched with FNVCOOP_MUT=<n> (tools/mut/run_twins.py)
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_rebalance CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(RB_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(RB_REPO "${RB_DIR}/../..")

function(rb_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS NOMINMAX)
endfunction()

set(RB_SRC ${RB_DIR}/balance.cpp ${RB_DIR}/ramp.cpp ${RB_DIR}/gmst_applier.cpp ${RB_DIR}/effect_applier.cpp ${RB_DIR}/scarcity.cpp ${RB_DIR}/fights.cpp ${RB_DIR}/director.cpp)
foreach(sfx "" "_rt")
  add_library(nvcoop_rebalance_core${sfx} STATIC ${RB_SRC})
  # "rebalance/..." and "common/minijson.h" resolve from plugin/; fnvcoop_mut.h from tools/mut
  target_include_directories(nvcoop_rebalance_core${sfx} PUBLIC ${RB_DIR}/.. ${RB_REPO}/tools/mut)
  rb_flags(nvcoop_rebalance_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_rebalance_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests (one exe; twins 1200..1299 and 1470..1499 are chosen at run time with FNVCOOP_MUT=<n>) ----
set(RB_TEST_EXCL "")
if(TARGET nvcoop)
  set(RB_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
add_executable(nvcoop_rebalance_tests ${RB_TEST_EXCL} ${RB_DIR}/tests/test_rebalance.cpp)
target_link_libraries(nvcoop_rebalance_tests PRIVATE nvcoop_rebalance_core_rt)
rb_flags(nvcoop_rebalance_tests)
set_target_properties(nvcoop_rebalance_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/rebalance_tests")
foreach(cfg _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_rebalance_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/rebalance_tests")
endforeach()

# ---- x86 game glue (compiled into nvcoop.dll): the GMST port, the Stimpak form port, the loot hook, the fight observer, the HUD line, the ini ----
if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND TARGET nvcoop AND TARGET nvcoop_coop_core AND TARGET nvcoop_actor_engine_core
   AND EXISTS "${RB_DIR}/rebalance_glue.cpp")
  target_sources(nvcoop PRIVATE ${RB_DIR}/rebalance_glue.cpp ${RB_DIR}/effects_real.cpp ${RB_DIR}/armor_real.cpp)   # NVCOOP_MODULE must be a direct source
  target_link_libraries(nvcoop PRIVATE nvcoop_rebalance_core nvcoop_actor_engine_core)
  target_include_directories(nvcoop PRIVATE ${RB_DIR}/.. ${RB_REPO}/plugin/coop ${RB_REPO}/plugin/avatar)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_REBALANCE=1)
  if(TARGET nvcoop_avatar)   # puppet references (who is a player) for the fight observer
    target_link_libraries(nvcoop PRIVATE nvcoop_avatar)
    target_compile_definitions(nvcoop PRIVATE NVCOOP_REBALANCE_AVATAR=1)
  endif()
endif()
