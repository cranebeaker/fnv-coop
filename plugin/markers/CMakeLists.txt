# plugin/markers: friend markers (name over the puppet, compass, Pip-Boy map) and revive of a downed friend. Works two ways:
#   * included by plugin/CMakeLists.txt: adds nvcoop_markers_core (and, for the x86 plugin build, the glue mk_glue.cpp that registers the module)
#   * standalone, for the game-free tests + mutation twins:  plugin\markers\run_tests.cmd (uses the shared tree build\all)
# Everything in the game is controlled by nvcoop.ini [markers] enabled=1 (default on) and [revive] enabled=1 (default on).
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_markers CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(MK_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(MK_REPO "${MK_DIR}/../..")
set(MK_MUT_INC "${MK_REPO}/tools/mut")

function(mk_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

# game-free core: production (what the plugin links) and the runtime-twin copy (what the tests link)
set(MK_SRC ${MK_DIR}/mk_board.cpp ${MK_DIR}/mk_wire.cpp ${MK_DIR}/rv_authority.cpp ${MK_DIR}/rv_hold.cpp ${MK_DIR}/mk_service.cpp ${MK_DIR}/mk_layout.cpp)
foreach(sfx "" "_rt")
  add_library(nvcoop_markers_core${sfx} STATIC ${MK_SRC})
  target_include_directories(nvcoop_markers_core${sfx} PUBLIC ${MK_DIR} ${MK_DIR}/.. ${MK_MUT_INC})
  mk_flags(nvcoop_markers_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_markers_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests. One exe; twins 1301..1434 are chosen at run time with FNVCOOP_MUT=<n> ----
set(MK_TEST_EXCL "")
if(TARGET nvcoop)
  set(MK_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
add_executable(nvcoop_markers_tests ${MK_TEST_EXCL} ${MK_DIR}/tests/test_core.cpp ${MK_DIR}/tests/test_service.cpp ${MK_DIR}/tests/test_layout.cpp ${MK_REPO}/synccore/tests/main.cpp)
target_include_directories(nvcoop_markers_tests PRIVATE ${MK_REPO}/synccore/tests ${MK_DIR}/tests)
target_link_libraries(nvcoop_markers_tests PRIVATE nvcoop_markers_core_rt)
mk_flags(nvcoop_markers_tests)
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_markers_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/markers_tests")
endforeach()

enable_testing()
add_test(NAME markers_unit COMMAND nvcoop_markers_tests)

# ---- plugin glue (x86, linked into nvcoop.dll next to the coop module): the engine half of the marker service, the HUD scripts, the order hooks ----
if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND TARGET nvcoop AND TARGET nvcoop_coop_core AND EXISTS "${MK_DIR}/mk_glue.cpp")
  target_link_libraries(nvcoop PRIVATE nvcoop_markers_core)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_MARKERS=1)
  target_include_directories(nvcoop PRIVATE ${MK_DIR})
  target_sources(nvcoop PRIVATE ${MK_DIR}/mk_glue.cpp ${MK_DIR}/mk_hud.cpp)
endif()
