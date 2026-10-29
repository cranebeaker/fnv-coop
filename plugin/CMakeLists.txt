# nvcoop plugin build (owned by L1). Configure + build (Win32/x86 is mandatory for the game DLL):
#   cmake -S plugin -B build/L1 -G "Visual Studio 17 2022" -A Win32
#   cmake --build build/L1 --config Release
# Artifacts land in <repo>/out/L1/ (nvcoop.dll, nvbridge_fakehost*.exe).
cmake_minimum_required(VERSION 3.20)
project(nvcoop C CXX)   # C: third_party/opus (voice chat)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")   # static CRT: no vcruntime dependency inside the game
set(NVCOOP_REPO_ROOT "${CMAKE_CURRENT_LIST_DIR}/..")
set(NVCOOP_OUT_DIR "${NVCOOP_REPO_ROOT}/out/L1" CACHE PATH "Where artifacts are copied")
set(FNVCOOP_NVSE_DIR "${NVCOOP_REPO_ROOT}/third_party/NVSE" CACHE PATH "xNVSE checkout")

foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set(CMAKE_RUNTIME_OUTPUT_DIRECTORY${cfg} "${NVCOOP_OUT_DIR}")
  set(CMAKE_LIBRARY_OUTPUT_DIRECTORY${cfg} "${NVCOOP_OUT_DIR}")
  set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/lib")
endforeach()

if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 8)
  message(WARNING "nvcoop.dll must be built Win32/x86 (-A Win32); this 64-bit build is only good for host tests.")
endif()

# ---- build info (git hash + dirty flag), regenerated on every build ----
set(NVCOOP_BUILDINFO_DIR "${CMAKE_BINARY_DIR}/gen")
file(MAKE_DIRECTORY "${NVCOOP_BUILDINFO_DIR}")
add_custom_target(nvcoop_buildinfo ALL
  COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_CURRENT_LIST_DIR} -DOUT=${NVCOOP_BUILDINFO_DIR}/build_info.h -P ${CMAKE_CURRENT_LIST_DIR}/buildinfo.cmake
  BYPRODUCTS ${NVCOOP_BUILDINFO_DIR}/build_info.h)

# ---- game-free core (bridge + registries). The _mutant twin deliberately misbehaves (see bridge.cpp). ----
set(NVCOOP_CORE_SRC common/nvcoop_api.cpp bridge/bridge.cpp bridge/screenshot.cpp)
foreach(v "" _mutant _mutant_lock _mutant_save _mutant_execref)
  add_library(nvcoop_core${v} STATIC ${NVCOOP_CORE_SRC})
  target_include_directories(nvcoop_core${v} PUBLIC common bridge PRIVATE ${NVCOOP_BUILDINFO_DIR})
  add_dependencies(nvcoop_core${v} nvcoop_buildinfo)
  target_compile_definitions(nvcoop_core${v} PUBLIC _CRT_SECURE_NO_WARNINGS)
  if(MSVC)
    target_compile_options(nvcoop_core${v} PRIVATE /W3 /EHsc /permissive-)
  endif()
  add_executable(nvbridge_fakehost${v} bridge/fakehost_main.cpp)
  target_link_libraries(nvbridge_fakehost${v} PRIVATE nvcoop_core${v})
endforeach()
target_compile_definitions(nvcoop_core_mutant PUBLIC NVBRIDGE_MUTANT=1)
target_compile_definitions(nvcoop_core_mutant_lock PUBLIC NVBRIDGE_MUTANT_LOCK=1)
target_compile_definitions(nvcoop_core_mutant_save PUBLIC NVBRIDGE_MUTANT_SAVE=1)
target_compile_definitions(nvcoop_core_mutant_execref PUBLIC NVBRIDGE_MUTANT_EXECREF=1)   # exec {cmd, ref} ignores ref

# PNG encoder self-test exe (+ mutant that corrupts the zlib checksum)
add_executable(nvbridge_pngtest bridge/png_test_main.cpp bridge/screenshot.cpp)
add_executable(nvbridge_pngtest_mutant bridge/png_test_main.cpp bridge/screenshot.cpp)
target_compile_definitions(nvbridge_pngtest_mutant PRIVATE NVPNG_MUTANT=1)
target_include_directories(nvbridge_pngtest PRIVATE bridge)
target_include_directories(nvbridge_pngtest_mutant PRIVATE bridge)

# ---- the xNVSE plugin ----
# Other lanes: put `target_sources(nvcoop PRIVATE ...)` (and optional include dirs / libs) in
# plugin/<hooks|snapshot|nvmp>/CMakeLists.txt; they are picked up automatically when present.
# Hot-reload split (T3a, opt-in): -DNVCOOP_SPLIT_LOADER=ON makes the `nvcoop` target the reloadable CORE
# (out/L1/nvcoop_mod.dll, so every lane's target_sources(nvcoop ...) still works) and adds a tiny permanent
# loader target `nvcoop_loader` that builds as nvcoop.dll. Default OFF = the monolithic nvcoop.dll as before.
option(NVCOOP_SPLIT_LOADER "Build nvcoop.dll as a small loader + reloadable nvcoop_mod.dll (see docs/tools/T3a-loader.md)" OFF)
if(NVCOOP_SPLIT_LOADER)
  add_library(nvcoop SHARED loader/core_entry.cpp loader/core_thunks.cpp)
  set_target_properties(nvcoop PROPERTIES PREFIX "" OUTPUT_NAME "nvcoop_mod")
  target_include_directories(nvcoop PRIVATE ${CMAKE_CURRENT_LIST_DIR}/loader ${CMAKE_CURRENT_LIST_DIR}/common ${CMAKE_CURRENT_LIST_DIR}/bridge)
else()
  add_library(nvcoop SHARED main.cpp)
  set_target_properties(nvcoop PROPERTIES PREFIX "" OUTPUT_NAME "nvcoop")
  target_link_libraries(nvcoop PRIVATE nvcoop_core)
endif()
target_include_directories(nvcoop PUBLIC
  "${CMAKE_CURRENT_LIST_DIR}" "${CMAKE_CURRENT_LIST_DIR}/../tools/mut"
  "${FNVCOOP_NVSE_DIR}" "${FNVCOOP_NVSE_DIR}/nvse" "${FNVCOOP_NVSE_DIR}/nvse/nvse")
target_compile_definitions(nvcoop PRIVATE RUNTIME=1 RUNTIME_VERSION=0x040020D0 WIN32 _WINDOWS _CRT_SECURE_NO_WARNINGS)
if(MSVC)
  target_compile_options(nvcoop PRIVATE "SHELL:/FI ${FNVCOOP_NVSE_DIR}/nvse/nvse/prefix.h" /W3 /EHsc /permissive- /Zc:__cplusplus)
endif()

# NOTE: xNVSE headers use max()/min(): do NOT define NOMINMAX for targets that include them.
option(NVCOOP_LINK_SNAPSHOT "Link L7 fnvcoop_snapshot into nvcoop.dll (needs the xNVSE game-API sources; links in L7 snapshot)" ON)
set(NVCOOP_SKIP_SUBDIRS "" CACHE STRING "semicolon list of plugin/<dir> to leave out (e.g. a lane whose code does not build yet)")
foreach(sub hooks hooks_ui hooks_world snapshot nvmp coop flight animprobe reload bots actors avatar combat spawn splitscreen intents world_time vats companions areadigest loot rebalance threat chat voice markers)
  if(sub IN_LIST NVCOOP_SKIP_SUBDIRS)
    continue()
  endif()
  if(EXISTS "${CMAKE_CURRENT_LIST_DIR}/${sub}/CMakeLists.txt")
    if(sub STREQUAL "snapshot" AND NOT NVCOOP_LINK_SNAPSHOT)
      add_subdirectory(${sub} EXCLUDE_FROM_ALL)
    else()
      add_subdirectory(${sub})
    endif()
  endif()
endforeach()
# dll-wiring: the live layer's glue (plugin/coop/live) comes AFTER every lane above, so it links whichever of actors / avatar / combat / spawn exist.
# Everything it adds to nvcoop.dll stays inert until nvcoop.ini [live] switches it on (all OFF by default).
if(NOT "coop/live" IN_LIST NVCOOP_SKIP_SUBDIRS AND EXISTS "${CMAKE_CURRENT_LIST_DIR}/coop/live/CMakeLists.txt")
  add_subdirectory(coop/live)
endif()
# Track B focus guard: it subclasses the game window, so it goes only into the DLL that is never unloaded
# (this monolithic nvcoop.dll, or the split-mode loader in plugin/loader/CMakeLists.txt).
if(TARGET nvsplit_focus AND NOT NVCOOP_SPLIT_LOADER)
  target_link_libraries(nvcoop PRIVATE nvsplit_focus)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_FOCUS_GUARD=1)
endif()
if(NVCOOP_LINK_SNAPSHOT AND TARGET fnvcoop_snapshot)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_SNAPSHOT=1)
endif()
# story-capture: RefCommand AI orders are performed only by the machine that owns the actor; needs the actors engine API (plugin/hooks/story_owner_gate.cpp)
if(TARGET nvcoop AND TARGET nvcoop_actor_engine_core AND EXISTS "${CMAKE_CURRENT_LIST_DIR}/hooks/story_owner_gate.cpp")
  target_sources(nvcoop PRIVATE ${CMAKE_CURRENT_LIST_DIR}/hooks/story_owner_gate.cpp)
endif()


# parity probe: the read-only bridge method parity.look for the lab's parity room (tools/lab); xNVSE headers only, no NVSE .cpp
if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND TARGET nvcoop AND EXISTS "${CMAKE_CURRENT_LIST_DIR}/probe/parity_probe.cpp")
  target_sources(nvcoop PRIVATE ${CMAKE_CURRENT_LIST_DIR}/probe/parity_probe.cpp)
endif()

if(NVCOOP_SPLIT_LOADER)
  add_subdirectory(loader)   # nvloader_host + the nvcoop_loader target (nvcoop.dll)
endif()
