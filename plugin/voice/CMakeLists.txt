# plugin/voice: positional voice chat between the players (docs/voice.md). Works two ways:
#   * included by plugin/CMakeLists.txt: adds nvcoop_voice_core (and, in the x86 plugin build, the glue + module that go into nvcoop.dll;
#     everything is inert until a co-op session runs, and nvcoop.ini [voice] enabled=0 switches it off)
#   * standalone, for the game-free tests + mutation twins:  plugin\voice\run_tests.cmd (uses the shared tree build\all)
# Targets
#   nvcoop_voice_core / _rt   wire, jitter buffer, placement, transmitter, receiver, session, settings, Opus glue (production / runtime twins 301..399)
#   nvcoop_voice_tests        EXCLUDE_FROM_ALL in the plugin build; twins are switched with FNVCOOP_MUT=<n> (tools/mut/run_twins.py)
#   nvcoop_opus               third_party/opus (C library)
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_voice C CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(VC_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(VC_REPO "${VC_DIR}/../..")
set(VC_MUT_INC "${VC_REPO}/tools/mut")

function(vc_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

if(NOT TARGET nvcoop_opus)
  include(${VC_REPO}/third_party/opus/CMakeLists.txt)
endif()

set(VC_SRC ${VC_DIR}/voice_wire.cpp ${VC_DIR}/voice_jitter.cpp ${VC_DIR}/voice_placement.cpp ${VC_DIR}/voice_tx.cpp ${VC_DIR}/voice_rx.cpp
           ${VC_DIR}/voice_session.cpp ${VC_DIR}/voice_config.cpp ${VC_DIR}/opus_codec.cpp)
foreach(sfx "" "_rt")
  add_library(nvcoop_voice_core${sfx} STATIC ${VC_SRC})
  target_include_directories(nvcoop_voice_core${sfx} PUBLIC ${VC_DIR} ${VC_MUT_INC})
  target_link_libraries(nvcoop_voice_core${sfx} PUBLIC nvcoop_opus)
  vc_flags(nvcoop_voice_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_voice_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests. One exe; twins 301..399 are chosen at run time with FNVCOOP_MUT=<n> ----
set(VC_TEST_EXCL "")
if(TARGET nvcoop)
  set(VC_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
add_executable(nvcoop_voice_tests ${VC_TEST_EXCL} ${VC_DIR}/tests/test_voice.cpp ${VC_DIR}/tests/test_opus.cpp ${VC_REPO}/synccore/tests/main.cpp)
target_include_directories(nvcoop_voice_tests PRIVATE ${VC_REPO}/synccore/tests ${VC_DIR}/tests)
target_link_libraries(nvcoop_voice_tests PRIVATE nvcoop_voice_core_rt)
vc_flags(nvcoop_voice_tests)
if(NOT CMAKE_RUNTIME_OUTPUT_DIRECTORY)
  set_target_properties(nvcoop_voice_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/voice_tests")
endif()
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_voice_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/voice_tests")
endforeach()

enable_testing()
add_test(NAME voice_unit COMMAND nvcoop_voice_tests)

# ---- plugin glue (x86, linked into nvcoop.dll next to the coop module) ----
# voice_module.cpp holds the NVCOOP_MODULE registration, so it must be a direct source of the nvcoop target.
if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND TARGET nvcoop AND TARGET nvcoop_coop_core AND EXISTS "${VC_DIR}/voice_module.cpp")
  target_link_libraries(nvcoop PRIVATE nvcoop_voice_core winmm)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_VOICE=1)
  target_sources(nvcoop PRIVATE ${VC_DIR}/voice_module.cpp ${VC_DIR}/voice_audio_win.cpp)
  target_include_directories(nvcoop PRIVATE ${VC_DIR})
endif()
