# plugin/companions: companions, crime and kill rewards in the plugin (FULL-SYNC-PLAN 3.4 / 3.8 / 3.10). Works two ways:
#   * included by plugin/CMakeLists.txt (shared tree build/all): adds the game-free core, the tests, and (x86, nvcoop target present) the engine glue
#   * standalone, for the game-free tests + mutation twins:  plugin\companions\run_tests.cmd
# Targets
#   nvcoop_social_rules / _rt   synccore/social/*.cpp compiled here (production / FNVCOOP_RUNTIME_MUT), so the plugin tree does not depend on the
#                               synccore tree being configured first (the same trick plugin/combat plays)
#   nvcoop_social_core / _rt    SocialSync + SocialRuntime (production / runtime twins 901..949)
#   nvcoop_companions_tests     EXCLUDE_FROM_ALL; twins are switched with FNVCOOP_MUT=<n> (tools/mut/run_twins.py)
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_companions CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(NVSO_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVSO_REPO "${NVSO_DIR}/../..")
set(NVSO_INC "${NVSO_DIR}/.." "${NVSO_REPO}/synccore" "${NVSO_REPO}/synccore/combat" "${NVSO_REPO}/synccore/include" "${NVSO_REPO}/tools/mut"
    "${NVSO_REPO}/plugin/hooks")

function(nvso_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005 /wd4244)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS NOMINMAX)
endfunction()

# ---- synccore base (wire, ownership ...): reuse a sibling lane's lib inside the plugin build ----
if(TARGET nvcoop_coop_synccore)
  set(NVSO_SYNC nvcoop_coop_synccore)
elseif(TARGET nvcoop_actors_synccore)
  set(NVSO_SYNC nvcoop_actors_synccore)
elseif(TARGET nvcoop_combat_synccore)
  set(NVSO_SYNC nvcoop_combat_synccore)
else()
  file(GLOB NVSO_SYNC_SRC CONFIGURE_DEPENDS ${NVSO_REPO}/synccore/src/*.cpp)
  list(FILTER NVSO_SYNC_SRC EXCLUDE REGEX "enet_transport\\.cpp$")
  add_library(nvcoop_social_synccore STATIC ${NVSO_SYNC_SRC})
  target_include_directories(nvcoop_social_synccore PUBLIC ${NVSO_REPO}/synccore/include ${NVSO_REPO}/synccore)
  nvso_flags(nvcoop_social_synccore)
  set(NVSO_SYNC nvcoop_social_synccore)
endif()

# ---- combat-core rules (HostilityBook, attributeKillXp): reuse plugin/combat's copy, else compile it here ----
foreach(sfx "" "_rt")
  if(TARGET nvcoop_combat_rules${sfx})
    set(NVSO_COMBAT${sfx} nvcoop_combat_rules${sfx})
  else()
    add_library(nvcoop_social_combat${sfx} STATIC ${NVSO_REPO}/synccore/combat/combat.cpp)
    target_include_directories(nvcoop_social_combat${sfx} PUBLIC ${NVSO_REPO}/synccore/combat)
    target_link_libraries(nvcoop_social_combat${sfx} PUBLIC ${NVSO_SYNC})
    nvso_flags(nvcoop_social_combat${sfx})
    if(sfx STREQUAL "_rt")
      target_compile_definitions(nvcoop_social_combat${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
    endif()
    set(NVSO_COMBAT${sfx} nvcoop_social_combat${sfx})
  endif()
endforeach()

# ---- the rules (synccore/social): production and runtime-twin flavours ----
set(NVSO_RULES_SRC ${NVSO_REPO}/synccore/social/companions.cpp ${NVSO_REPO}/synccore/social/slots.cpp ${NVSO_REPO}/synccore/social/escort.cpp ${NVSO_REPO}/synccore/social/crime.cpp ${NVSO_REPO}/synccore/social/rewards.cpp ${NVSO_REPO}/synccore/social/wire.cpp)
foreach(sfx "" "_rt")
  add_library(nvcoop_social_rules${sfx} STATIC ${NVSO_RULES_SRC})
  target_include_directories(nvcoop_social_rules${sfx} PUBLIC ${NVSO_REPO}/synccore ${NVSO_REPO}/synccore/combat)
  target_link_libraries(nvcoop_social_rules${sfx} PUBLIC ${NVSO_COMBAT${sfx}})
  nvso_flags(nvcoop_social_rules${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_social_rules${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- the module core ----
set(NVSO_CORE_SRC ${NVSO_DIR}/social_sync.cpp ${NVSO_DIR}/social_runtime.cpp)
foreach(sfx "" "_rt")
  add_library(nvcoop_social_core${sfx} STATIC ${NVSO_CORE_SRC})
  target_include_directories(nvcoop_social_core${sfx} PUBLIC ${NVSO_INC})
  target_link_libraries(nvcoop_social_core${sfx} PUBLIC nvcoop_social_rules${sfx})
  nvso_flags(nvcoop_social_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_social_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests (+ runtime twins: run_tests.cmd runs the exe once per twin number) ----
set(NVSO_TEST_EXCL "")
if(TARGET nvcoop)
  set(NVSO_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
file(GLOB NVSO_TEST_SRC CONFIGURE_DEPENDS ${NVSO_DIR}/tests/test_*.cpp)
add_executable(nvcoop_companions_tests ${NVSO_TEST_EXCL} ${NVSO_TEST_SRC} ${NVSO_REPO}/synccore/tests/main.cpp)
target_include_directories(nvcoop_companions_tests PRIVATE ${NVSO_DIR}/tests ${NVSO_REPO}/synccore/tests)
target_link_libraries(nvcoop_companions_tests PRIVATE nvcoop_social_core_rt)
target_compile_definitions(nvcoop_companions_tests PRIVATE NVSO_DATA_DIR="${NVSO_DIR}/data")
nvso_flags(nvcoop_companions_tests)
set_target_properties(nvcoop_companions_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/companions_tests")
foreach(cfg _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_companions_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/companions_tests")
endforeach()

# ---- x86 game glue (compiled into nvcoop.dll): the real engine adapter, the module, the bridge methods ----
if(TARGET nvcoop AND TARGET nvcoop_hooks_game AND TARGET nvcoop_actor_engine_core AND MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4
   AND EXISTS "${NVSO_DIR}/companions_module.cpp")
  target_sources(nvcoop PRIVATE ${NVSO_DIR}/companions_module.cpp)
  target_link_libraries(nvcoop PRIVATE nvcoop_social_core nvcoop_actor_engine_core)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_COMPANIONS=1)
  target_include_directories(nvcoop PRIVATE ${NVSO_INC} ${NVSO_REPO}/plugin/coop)
endif()
