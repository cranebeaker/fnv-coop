# plugin/chat: text chat between the players (docs/chat.md). Works two ways:
#   * included by plugin/CMakeLists.txt (shared tree build/all): adds the game-free core, the tests, and (x86, nvcoop target + coop core present) the game glue
#   * standalone, for the game-free tests + mutation twins:  plugin\chat\run_tests.cmd
# Targets
#   nvcoop_chat_core / _rt   the game-free logic (text rules, rate limit, history, line editor, wire, core, session adapter, view, controller, input
#                            translator, key blocker, surfaces' logic); _rt is compiled with FNVCOOP_RUNTIME_MUT (chat twins 1001..1146, chosen with FNVCOOP_MUT=<n>)
#   nvcoop_chat_tests        EXCLUDE_FROM_ALL; the exe run once per twin by tools/mut/run_twins.py
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_chat CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(NVCH_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVCH_REPO "${NVCH_DIR}/../..")
set(NVCH_INC "${NVCH_DIR}/.." "${NVCH_REPO}/synccore/include" "${NVCH_REPO}/synccore" "${NVCH_REPO}/tools/mut")

function(nvch_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005 /wd4244)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

# ---- synccore (the mux, the in-memory network) reused from the coop lane's lib inside the plugin build ----
if(TARGET nvcoop_coop_synccore)
  set(NVCH_SYNC nvcoop_coop_synccore)
elseif(TARGET nvcoop_actors_synccore)
  set(NVCH_SYNC nvcoop_actors_synccore)
elseif(TARGET nvcoop_vats_synccore)
  set(NVCH_SYNC nvcoop_vats_synccore)
else()
  file(GLOB NVCH_SYNC_SRC CONFIGURE_DEPENDS ${NVCH_REPO}/synccore/src/*.cpp)
  list(FILTER NVCH_SYNC_SRC EXCLUDE REGEX "enet_transport\\.cpp$")
  add_library(nvcoop_chat_synccore STATIC ${NVCH_SYNC_SRC})
  target_include_directories(nvcoop_chat_synccore PUBLIC ${NVCH_REPO}/synccore/include ${NVCH_REPO}/synccore)
  nvch_flags(nvcoop_chat_synccore)
  set(NVCH_SYNC nvcoop_chat_synccore)
endif()

# ---- the game-free core: production (what the plugin links) and the runtime-twin copy (what the tests link) ----
set(NVCH_CORE_SRC
    ${NVCH_DIR}/chat_text.cpp ${NVCH_DIR}/chat_limiter.cpp ${NVCH_DIR}/chat_log.cpp ${NVCH_DIR}/line_editor.cpp ${NVCH_DIR}/chat_wire.cpp
    ${NVCH_DIR}/chat_core.cpp ${NVCH_DIR}/chat_session.cpp ${NVCH_DIR}/chat_view.cpp ${NVCH_DIR}/chat_controller.cpp ${NVCH_DIR}/chat_input.cpp
    ${NVCH_DIR}/chat_keyblock.cpp ${NVCH_DIR}/chat_surface.cpp ${NVCH_DIR}/chat_config.cpp ${NVCH_DIR}/chat_api.cpp)
foreach(sfx "" "_rt")
  add_library(nvcoop_chat_core${sfx} STATIC ${NVCH_CORE_SRC})
  target_include_directories(nvcoop_chat_core${sfx} PUBLIC ${NVCH_INC})
  target_link_libraries(nvcoop_chat_core${sfx} PUBLIC ${NVCH_SYNC})
  nvch_flags(nvcoop_chat_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_chat_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests (twins are chosen at run time: run_tests.cmd runs the exe once per twin number) ----
set(NVCH_TEST_EXCL "")
if(TARGET nvcoop)
  set(NVCH_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
set(NVCH_TEST_SRC ${NVCH_DIR}/tests/test_chat_basic.cpp ${NVCH_DIR}/tests/test_chat_core.cpp ${NVCH_DIR}/tests/test_chat_ui.cpp ${NVCH_REPO}/synccore/tests/main.cpp)
if(WIN32)
  list(APPEND NVCH_TEST_SRC ${NVCH_DIR}/tests/test_chat_overlay.cpp ${NVCH_DIR}/chat_overlay_win.cpp)
endif()
add_executable(nvcoop_chat_tests ${NVCH_TEST_EXCL} ${NVCH_TEST_SRC})
target_include_directories(nvcoop_chat_tests PRIVATE ${NVCH_REPO}/synccore/tests ${NVCH_DIR}/tests)
target_link_libraries(nvcoop_chat_tests PRIVATE nvcoop_chat_core_rt)
if(WIN32)
  target_link_libraries(nvcoop_chat_tests PRIVATE user32 gdi32)
endif()
nvch_flags(nvcoop_chat_tests)
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_chat_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/chat_tests")
endforeach()

# ---- game glue (x86, compiled into nvcoop.dll next to the coop module): the module (ini, bridge methods, per-frame work), the window-message hook, the
#      engine calls (menu check, HUD message, key disabling) and the overlay window ----
if(TARGET nvcoop AND TARGET nvcoop_coop_core AND MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4)
  target_link_libraries(nvcoop PRIVATE nvcoop_chat_core user32 gdi32)
  target_include_directories(nvcoop PRIVATE ${NVCH_INC} ${NVCH_REPO}/plugin/coop ${NVCH_REPO}/plugin/hooks)
  target_sources(nvcoop PRIVATE ${NVCH_DIR}/chat_module.cpp ${NVCH_DIR}/chat_overlay_win.cpp)   # NVCOOP_MODULE must be a direct source
endif()
