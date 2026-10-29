# plugin/coop/live (dll-wiring): the live layer's glue. Added by plugin/CMakeLists.txt AFTER every lane (actors, avatar, combat, spawn)
# so it can link whatever exists. Game-free core (nvcoop_live[_rt]): ini switches, the LiveHub (session / load lifecycle + mux channel
# routing), the avatar network service, and the unit tests with their mutation twins (runtime switchable, tools/mut/fnvcoop_mut.h).
# The game-facing part (live_glue.cpp) is a direct source of nvcoop.dll, switched by [live] in nvcoop.ini (all OFF by default).
cmake_minimum_required(VERSION 3.20)

set(NVLIVE_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVLIVE_REPO "${NVLIVE_DIR}/../../..")

if(NOT (TARGET nvcoop_avatar_engine AND TARGET nvcoop_actors_live))
  message(STATUS "live wiring: plugin/avatar/engine or plugin/actors not configured; nvcoop_live skipped")
  return()
endif()

function(nvlive_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

# look_codec.cpp (plugin/avatar) holds the field codecs of Look / Facts / Pack and their twins: compiled here, in both flavours, next to the wire code that uses it
set(NVLIVE_SRC ${NVLIVE_DIR}/live_switches.cpp ${NVLIVE_DIR}/live_hub.cpp ${NVLIVE_DIR}/avatar_wire.cpp ${NVLIVE_DIR}/avatar_net.cpp ${NVLIVE_DIR}/player_port.cpp
  ${NVLIVE_REPO}/plugin/avatar/look_codec.cpp)
if(EXISTS ${NVLIVE_REPO}/plugin/actors/engine/actor_engine_api.h)   # [actors] ini -> actors::engine::Config
  list(APPEND NVLIVE_SRC ${NVLIVE_DIR}/live_actors_cfg.cpp)
endif()
foreach(sfx "" _rt)
  add_library(nvcoop_live${sfx} STATIC ${NVLIVE_SRC})
  target_include_directories(nvcoop_live${sfx} PUBLIC ${NVLIVE_DIR} ${NVLIVE_REPO}/tools/mut ${NVLIVE_REPO}/plugin)
  # the PRODUCTION avatar engine library in both variants: its own runtime twins (FNVCOOP_MUT numbers 1..70) must not fire in our test binary
  target_link_libraries(nvcoop_live${sfx} PUBLIC nvcoop_avatar_engine nvcoop_actors_live)
  if(TARGET nvcoop_actors_core)   # PlayerPort is an IActorPort (the part codecs and the default part calls live in plugin/actors/state_parts.cpp)
    target_link_libraries(nvcoop_live${sfx} PUBLIC nvcoop_actors_core)
  endif()
  nvlive_flags(nvcoop_live${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_live${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests (not shipped). One exe, twins picked with FNVCOOP_MUT=<n> at run time; tools/mut/run_twins.py drives them. ----
add_executable(nvcoop_live_tests EXCLUDE_FROM_ALL ${NVLIVE_DIR}/tests/test_live_wiring.cpp)
target_link_libraries(nvcoop_live_tests PRIVATE nvcoop_live_rt nvcoop_coop_synccore)
nvlive_flags(nvcoop_live_tests)
if(EXISTS ${NVLIVE_REPO}/plugin/actors/engine/actor_engine_api.h)
  target_compile_definitions(nvcoop_live_tests PRIVATE NVLIVE_HAVE_ACTOR_API=1)
endif()
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_live_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/live_tests")
endforeach()
if(NOT CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  enable_testing()
endif()
add_test(NAME live_wiring_unit COMMAND nvcoop_live_tests)

# ---- game glue: only inside the plugin build (nvcoop.dll with the coop module) ----
if(TARGET nvcoop AND TARGET nvcoop_coop_core)
  target_sources(nvcoop PRIVATE ${NVLIVE_DIR}/live_glue.cpp)
  target_link_libraries(nvcoop PRIVATE nvcoop_live)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_LIVE=1)
  target_include_directories(nvcoop PRIVATE ${NVLIVE_DIR})
  if(TARGET nvcoop_actor_engine_core AND EXISTS ${NVLIVE_REPO}/plugin/actors/engine/actor_engine_api.h)
    target_link_libraries(nvcoop PRIVATE nvcoop_actor_engine_core)   # actors::engine::configure() is called from [actors] in nvcoop.ini
    target_compile_definitions(nvcoop PRIVATE NVLIVE_ACTOR_ENGINE=1)
  endif()
endif()
