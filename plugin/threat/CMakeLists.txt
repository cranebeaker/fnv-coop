# plugin/threat: enemies split their attention between the players (docs/threat.md). Works two ways:
#   * included by plugin/CMakeLists.txt: adds the game-free core, the tests, and (x86, nvcoop target present) the engine glue
#   * standalone, for the game-free tests + mutation twins:  plugin\threat\run_tests.cmd (uses the shared tree build\all)
# Targets
#   nvcoop_threat_core / _rt   scoring, director and combat-style tactics (production / FNVCOOP_RUNTIME_MUT twins 1001..1048)
#   nvcoop_threat_tests        EXCLUDE_FROM_ALL; twins are switched with FNVCOOP_MUT=<n> (tools/mut/run_twins.py)
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_threat CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(NVTH_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVTH_REPO "${NVTH_DIR}/../..")
set(NVTH_INC "${NVTH_DIR}/.." "${NVTH_REPO}/tools/mut")

function(nvth_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4100 /wd4127)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS NOMINMAX)
endfunction()

# ---- the game-free core: production (what the plugin links) and the runtime-twin copy (what the tests link) ----
foreach(sfx "" "_rt")
  add_library(nvcoop_threat_core${sfx} STATIC ${NVTH_DIR}/threat_model.cpp ${NVTH_DIR}/threat_director.cpp ${NVTH_DIR}/tactics.cpp)
  target_include_directories(nvcoop_threat_core${sfx} PUBLIC ${NVTH_INC})
  nvth_flags(nvcoop_threat_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_threat_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests (run_tests.cmd runs the exe once per twin number) ----
set(NVTH_TEST_EXCL "")
if(TARGET nvcoop)
  set(NVTH_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
add_executable(nvcoop_threat_tests ${NVTH_TEST_EXCL} ${NVTH_DIR}/tests/test_model.cpp ${NVTH_DIR}/tests/test_director.cpp ${NVTH_DIR}/tests/test_tactics.cpp
  ${NVTH_REPO}/synccore/tests/main.cpp)
target_include_directories(nvcoop_threat_tests PRIVATE ${NVTH_DIR}/tests ${NVTH_REPO}/synccore/tests)
target_link_libraries(nvcoop_threat_tests PRIVATE nvcoop_threat_core_rt)
nvth_flags(nvcoop_threat_tests)
set_target_properties(nvcoop_threat_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/threat_tests")
foreach(cfg _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_threat_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/threat_tests")
endforeach()

# ---- x86 game glue (compiled into nvcoop.dll): the engine side of the port, the module, the ini ----
if(TARGET nvcoop AND TARGET nvcoop_hooks_game AND TARGET nvcoop_actor_engine_core AND MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4
   AND EXISTS "${NVTH_DIR}/threat_module.cpp")
  target_sources(nvcoop PRIVATE ${NVTH_DIR}/threat_module.cpp)   # NVCOOP_MODULE must be a direct source
  target_link_libraries(nvcoop PRIVATE nvcoop_threat_core nvcoop_actor_engine_core)
  if(TARGET nvcoop_avatar)
    target_link_libraries(nvcoop PRIVATE nvcoop_avatar)
  endif()
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_THREAT=1)
  target_include_directories(nvcoop PRIVATE ${NVTH_INC} ${NVTH_REPO}/plugin/hooks)
endif()
