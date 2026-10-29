# plugin/actors/engine: the real actor port (EnginePort + ActorFeed + the engine adapter). Added by plugin/actors/CMakeLists.txt when the plugin
# tree is built (shared tree build/all). Targets:
#   nvcoop_actor_engine_core     production lib (mutation twins compiled out): what nvcoop.dll links
#   nvcoop_actor_engine_core_rt  the same sources with FNVCOOP_RUNTIME_MUT: what the tests link (twin n = env FNVCOOP_MUT=n)
#   nvcoop_actor_engine_tests    game-free tests, EXCLUDE_FROM_ALL:  plugin\actors\engine\run_tests.cmd  (prints ALL OK)
#   nvcoop_actor_engine_game     x86 only: RealEngineHost over the xNVSE layouts (engine_real.cpp); engine_module.cpp is a direct source of nvcoop.dll
cmake_minimum_required(VERSION 3.20)

set(NVAE_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVAE_REPO "${NVAE_DIR}/../../..")
set(NVAE_MUT_INC "${NVAE_REPO}/tools/mut")

function(nvae_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS NOMINMAX)
endfunction()

set(NVAE_CORE_SRC
  ${NVAE_DIR}/actor_engine_api.cpp
  ${NVAE_DIR}/actor_feed.cpp
  ${NVAE_DIR}/engine_port.cpp
  ${NVAE_DIR}/equipment.cpp
  ${NVAE_DIR}/ledger.cpp
  ${NVAE_DIR}/script_engine.cpp)

foreach(sfx "" "_rt")
  add_library(nvcoop_actor_engine_core${sfx} STATIC ${NVAE_CORE_SRC})
  target_include_directories(nvcoop_actor_engine_core${sfx} PUBLIC ${NVAE_DIR}/../.. ${NVAE_MUT_INC})
  target_link_libraries(nvcoop_actor_engine_core${sfx} PUBLIC nvcoop_actors_core nvcoop_hooks_core)
  nvae_flags(nvcoop_actor_engine_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_actor_engine_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- game-free tests (+ runtime twins: run_tests.cmd runs the exe once per twin number) ----
file(GLOB NVAE_TEST_SRC CONFIGURE_DEPENDS ${NVAE_DIR}/tests/test_*.cpp)
add_executable(nvcoop_actor_engine_tests EXCLUDE_FROM_ALL ${NVAE_TEST_SRC} ${NVAE_REPO}/synccore/tests/main.cpp)
target_include_directories(nvcoop_actor_engine_tests PRIVATE ${NVAE_DIR} ${NVAE_DIR}/tests ${NVAE_REPO}/synccore/tests ${NVAE_DIR}/../tests)
target_link_libraries(nvcoop_actor_engine_tests PRIVATE nvcoop_actor_engine_core_rt)
nvae_flags(nvcoop_actor_engine_tests)
set_target_properties(nvcoop_actor_engine_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/actor_engine_tests")
foreach(cfg _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_actor_engine_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/actor_engine_tests")
endforeach()

# ---- game glue (x86 only): the real engine host and the module that plugs the port into the coop runtime ----
if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND TARGET nvcoop AND TARGET nvcoop_hooks_game AND TARGET nvcoop_world_game AND TARGET nvcoop_coop_core
   AND EXISTS "${NVAE_DIR}/engine_real.cpp" AND EXISTS "${NVAE_DIR}/engine_module.cpp")
  set(NVAE_NVSE "${NVAE_REPO}/third_party/NVSE" CACHE PATH "xNVSE checkout")
  add_library(nvcoop_actor_engine_game STATIC ${NVAE_DIR}/engine_real.cpp ${NVAE_DIR}/equipment_real.cpp)
  target_link_libraries(nvcoop_actor_engine_game PUBLIC nvcoop_actor_engine_core nvcoop_hooks_game nvcoop_world_game)
  target_include_directories(nvcoop_actor_engine_game PUBLIC ${NVAE_REPO}/plugin/hooks_world ${NVAE_REPO}/plugin/hooks
    "${NVAE_NVSE}" "${NVAE_NVSE}/nvse" "${NVAE_NVSE}/nvse/nvse")
  target_compile_definitions(nvcoop_actor_engine_game PRIVATE RUNTIME=1 RUNTIME_VERSION=0x040020D0 WIN32 _WINDOWS _CRT_SECURE_NO_WARNINGS)
  target_compile_options(nvcoop_actor_engine_game PRIVATE "SHELL:/FI ${NVAE_NVSE}/nvse/nvse/prefix.h" /W3 /EHsc /permissive- /Zc:__cplusplus)
  # NVCOOP_MODULE registration must be a direct source of the final DLL (static-library objects without references are dropped)
  target_link_libraries(nvcoop PRIVATE nvcoop_actor_engine_game nvcoop_actor_engine_core)
  target_sources(nvcoop PRIVATE ${NVAE_DIR}/engine_module.cpp)
  target_include_directories(nvcoop PRIVATE ${NVAE_DIR}/../.. ${NVAE_MUT_INC})
endif()
