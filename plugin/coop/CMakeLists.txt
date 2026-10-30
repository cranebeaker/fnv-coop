# L10 co-op runtime. Works two ways:
#   * included by plugin/CMakeLists.txt (after hooks): adds the coop module + UDP transport + synccore to nvcoop.dll
#   * standalone, for the game-free tests (and the fake peers used by the in-game test):
#       cmake -S plugin/coop -B build/L10 -G Ninja && cmake --build build/L10 && ctest --test-dir build/L10
#     or just run plugin\coop\run_tests.cmd (x86 like the game; also builds the mutation-twin peers)
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_coop C CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

enable_language(C)   # vendored ENet is C
set(COOP_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(COOP_REPO "${COOP_DIR}/../..")
set(COOP_OUT "${COOP_REPO}/out/L10" CACHE PATH "Where the coop test exes land")

function(coop_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

# ---- synccore (L4) sources compiled into our own static lib (L4's CMake also builds its tests/sim) ----
file(GLOB COOP_SYNCCORE_SRC CONFIGURE_DEPENDS ${COOP_REPO}/synccore/src/*.cpp)   # follows L4 as it adds files
add_library(nvcoop_coop_synccore STATIC ${COOP_SYNCCORE_SRC} ${COOP_REPO}/synccore/flight/flight.cpp)   # + T1a flight recorder
target_include_directories(nvcoop_coop_synccore PUBLIC ${COOP_REPO}/synccore/include ${COOP_REPO}/synccore)
coop_flags(nvcoop_coop_synccore)
if(COMMAND fnvcoop_pch)
  fnvcoop_pch(nvcoop_coop_synccore)
endif()
# vendored ENet 1.3.18 (L4's third_party/enet), same sources as synccore's enet_static
set(COOP_ENET ${COOP_REPO}/third_party/enet)
add_library(nvcoop_coop_enet STATIC ${COOP_ENET}/callbacks.c ${COOP_ENET}/compress.c ${COOP_ENET}/host.c ${COOP_ENET}/list.c
  ${COOP_ENET}/packet.c ${COOP_ENET}/peer.c ${COOP_ENET}/protocol.c ${COOP_ENET}/win32.c)
target_include_directories(nvcoop_coop_enet PUBLIC ${COOP_ENET}/include)
target_compile_definitions(nvcoop_coop_enet PRIVATE _WINSOCK_DEPRECATED_NO_WARNINGS _CRT_SECURE_NO_WARNINGS)
target_link_libraries(nvcoop_coop_enet PUBLIC ws2_32 winmm)
target_link_libraries(nvcoop_coop_synccore PUBLIC nvcoop_coop_enet)

# ---- L5 game-free hooks core (capture hub + effect json): reuse the plugin's target when present ----
if(TARGET nvcoop_hooks_core)
  set(COOP_HOOKS_LIB nvcoop_hooks_core)
else()
  add_library(nvcoop_coop_hooks STATIC ${COOP_DIR}/../hooks/capture_hub.cpp ${COOP_DIR}/../hooks/effect_json.cpp ${COOP_DIR}/../hooks/change_tracker.cpp)
  target_include_directories(nvcoop_coop_hooks PUBLIC ${COOP_DIR}/../hooks ${COOP_DIR}/.. ${COOP_REPO}/synccore/include)
  coop_flags(nvcoop_coop_hooks)
  set(COOP_HOOKS_LIB nvcoop_coop_hooks)
endif()

# ---- L11 hooks_ui router (game-free): the plugin build already has it (nvui_core); standalone builds compile ui_core.cpp ----
if(TARGET nvui_core)
  set(COOP_UI_LIB nvui_core)
else()
  add_library(nvcoop_coop_uicore STATIC ${COOP_DIR}/../hooks_ui/ui_core.cpp)
  target_include_directories(nvcoop_coop_uicore PUBLIC ${COOP_DIR}/../hooks_ui ${COOP_DIR}/.. ${COOP_DIR}/../dialogue/include ${COOP_REPO}/synccore/include)
  coop_flags(nvcoop_coop_uicore)
  set(COOP_UI_LIB nvcoop_coop_uicore)
endif()

# ---- runtime + transport. Mutation twins are RUNTIME-switchable (tools/mut/fnvcoop_mut.h, see coop_runtime.cpp): nvcoop_coop_core is the
# production lib (twins compiled out, linked into nvcoop.dll); nvcoop_coop_core_rt carries FNVCOOP_RUNTIME_MUT and is what the tests and
# fake peers use. FNVCOOP_MUT=1..4 breaks one wiring call, FNVCOOP_MUT=5 + COOP_UI_MUTANT/COOP_LEAVE_MUTANT=n the L11 / leave twins. ----
foreach(v "" _rt)
  add_library(nvcoop_coop_core${v} STATIC ${COOP_DIR}/udp_transport.cpp ${COOP_DIR}/coop_runtime.cpp ${COOP_DIR}/party_save.cpp)
  target_include_directories(nvcoop_coop_core${v} PUBLIC ${COOP_DIR} ${COOP_DIR}/.. ${COOP_DIR}/../dialogue/include ${COOP_REPO}/tools/mut)
  target_link_libraries(nvcoop_coop_core${v} PUBLIC nvcoop_coop_synccore ${COOP_HOOKS_LIB} ${COOP_UI_LIB} ws2_32)
  coop_flags(nvcoop_coop_core${v})
endforeach()
target_compile_definitions(nvcoop_coop_core_rt PUBLIC FNVCOOP_RUNTIME_MUT)

# ---- tests + fake peers (not shipped in the plugin) ----
set(COOP_TEST_TARGETS)
add_executable(coop_tests ${COOP_DIR}/tests/test_coop.cpp)
target_link_libraries(coop_tests PRIVATE nvcoop_coop_core_rt)
list(APPEND COOP_TEST_TARGETS coop_tests)
# party saves (party_save.h): the game-free half, its own exe so the suite is quick to build and run
add_executable(party_save_tests ${COOP_DIR}/tests/test_party_save.cpp)
target_link_libraries(party_save_tests PRIVATE nvcoop_coop_core_rt)
list(APPEND COOP_TEST_TARGETS party_save_tests)
add_test(NAME party_save_unit COMMAND party_save_tests)
# the fake peer: tests/test_integration.py starts it with FNVCOOP_MUT=1..4 for the mutant-peer scenarios
add_executable(coop_peer ${COOP_DIR}/tests/coop_peer.cpp)
target_link_libraries(coop_peer PRIVATE nvcoop_coop_core_rt)
list(APPEND COOP_TEST_TARGETS coop_peer)
foreach(t ${COOP_TEST_TARGETS})
  coop_flags(${t})
  if(NOT CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
    set_target_properties(${t} PROPERTIES EXCLUDE_FROM_ALL TRUE)   # plugin build: skip tests unless asked for
  endif()
  foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
    set_target_properties(${t} PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${COOP_OUT}")
  endforeach()
endforeach()

enable_testing()
add_test(NAME coop_unit COMMAND coop_tests)

# ---- game glue: only when included by the plugin build and L5's game library exists ----
if(TARGET nvcoop AND TARGET nvcoop_hooks_game)
  target_sources(nvcoop PRIVATE ${COOP_DIR}/coop_module.cpp)   # NVCOOP_MODULE must be a direct source
  target_sources(nvcoop PRIVATE ${COOP_DIR}/party_save_glue.cpp)   # party saves: the engine half (direct save, save folder, sidecars)
  target_link_libraries(nvcoop PRIVATE nvcoop_coop_core)
  if(TARGET nvcoop_hooks_ui_game)
    target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_UI=1)   # coop_module.cpp wires L11 (router, slideshow channel, INFO reader)
  endif()
  if(TARGET nvcoop_world_game)
    target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_WORLD=1)   # coop_module.cpp wires L12 (SpawnRef/RefScriptVar apply, NPC world effects)
  endif()
  # L8 INFO response reader (game-free) for nvui::setInfoReader
  target_sources(nvcoop PRIVATE ${COOP_DIR}/../dialogue/engine/info_reader.cpp)
  target_include_directories(nvcoop PRIVATE ${COOP_DIR}/../dialogue/engine ${COOP_DIR}/../dialogue/include ${COOP_DIR}/../hooks_ui)
  if(TARGET nvcoop_nvmp)   # L6: GhostRefOfPlayer is the acting-player activator (ghost_ref.h); without it ACTING-STANDIN (host player)
    target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_NVMP=1)
    target_include_directories(nvcoop PRIVATE ${COOP_DIR}/..)
  endif()
endif()
