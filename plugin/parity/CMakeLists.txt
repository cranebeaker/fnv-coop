# plugin/parity: the live sync parity checker (docs/tools/parity.md). Works two ways:
#   * included by plugin/CMakeLists.txt (shared tree build/all): adds the game-free core, the tests, and (x86, nvcoop target + avatar engine present) the game glue
#   * standalone, for the game-free tests + mutation twins:  plugin\parity\run_tests.cmd
# Targets
#   nvcoop_parity_core / _rt   the game-free logic (types, anim classes, config, wire, comparer, sampler, json + file, service); _rt is compiled with
#                              FNVCOOP_RUNTIME_MUT (twins 1800..1827, chosen with FNVCOOP_MUT=<n>)
#   nvcoop_parity_tests        EXCLUDE_FROM_ALL; the exe run once per twin by tools/mut/run_twins.py
cmake_minimum_required(VERSION 3.20)
if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_LIST_DIR)
  project(nvcoop_parity CXX)
  set(CMAKE_CXX_STANDARD 20)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

set(NVPY_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVPY_REPO "${NVPY_DIR}/../..")
set(NVPY_INC "${NVPY_DIR}/.." "${NVPY_REPO}/synccore/include" "${NVPY_REPO}/synccore" "${NVPY_REPO}/tools/mut")

function(nvpy_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005 /wd4244)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

# ---- synccore (the mux, crc32, the in-memory network) reused from the coop lane's lib inside the plugin build ----
if(TARGET nvcoop_coop_synccore)
  set(NVPY_SYNC nvcoop_coop_synccore)
elseif(TARGET nvcoop_actors_synccore)
  set(NVPY_SYNC nvcoop_actors_synccore)
elseif(TARGET nvcoop_chat_synccore)
  set(NVPY_SYNC nvcoop_chat_synccore)
else()
  file(GLOB NVPY_SYNC_SRC CONFIGURE_DEPENDS ${NVPY_REPO}/synccore/src/*.cpp)
  list(FILTER NVPY_SYNC_SRC EXCLUDE REGEX "enet_transport\\.cpp$")
  add_library(nvcoop_parity_synccore STATIC ${NVPY_SYNC_SRC})
  target_include_directories(nvcoop_parity_synccore PUBLIC ${NVPY_REPO}/synccore/include ${NVPY_REPO}/synccore)
  nvpy_flags(nvcoop_parity_synccore)
  set(NVPY_SYNC nvcoop_parity_synccore)
endif()

# ---- the game-free core: production (what the plugin links) and the runtime-twin copy (what the tests link) ----
set(NVPY_CORE_SRC
    ${NVPY_DIR}/parity_config.cpp ${NVPY_DIR}/parity_wire.cpp ${NVPY_DIR}/parity_compare.cpp ${NVPY_DIR}/parity_sampler.cpp
    ${NVPY_DIR}/parity_json.cpp ${NVPY_DIR}/parity_service.cpp)
foreach(sfx "" "_rt")
  add_library(nvcoop_parity_core${sfx} STATIC ${NVPY_CORE_SRC})
  target_include_directories(nvcoop_parity_core${sfx} PUBLIC ${NVPY_INC})
  target_link_libraries(nvcoop_parity_core${sfx} PUBLIC ${NVPY_SYNC})
  nvpy_flags(nvcoop_parity_core${sfx})
  if(sfx STREQUAL "_rt")
    target_compile_definitions(nvcoop_parity_core${sfx} PUBLIC FNVCOOP_RUNTIME_MUT)
  endif()
endforeach()

# ---- tests (twins are chosen at run time: run_tests.cmd runs the exe once per twin number) ----
set(NVPY_TEST_EXCL "")
if(TARGET nvcoop)
  set(NVPY_TEST_EXCL EXCLUDE_FROM_ALL)   # plugin build: skip tests unless asked for
endif()
add_executable(nvcoop_parity_tests ${NVPY_TEST_EXCL} ${NVPY_DIR}/tests/test_parity.cpp ${NVPY_REPO}/synccore/tests/main.cpp)
target_include_directories(nvcoop_parity_tests PRIVATE ${NVPY_REPO}/synccore/tests ${NVPY_DIR}/tests)
target_link_libraries(nvcoop_parity_tests PRIVATE nvcoop_parity_core_rt)
target_compile_definitions(nvcoop_parity_tests PRIVATE PARITY_VECTORS_PATH="${NVPY_REPO}/tools/parity/vectors.json")
nvpy_flags(nvcoop_parity_tests)
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_parity_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/parity_tests")
endforeach()

# ---- game glue (x86, compiled into nvcoop.dll next to the coop module): the module (ini, bridge methods, per-frame work) and the engine source, which reads
#      through the avatar layer's backend (so it needs the avatar engine in the dll) ----
if(TARGET nvcoop AND TARGET nvcoop_coop_core AND TARGET nvcoop_avatar_engine AND NVCOOP_WITH_AVATAR_ENGINE AND MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4)
  target_link_libraries(nvcoop PRIVATE nvcoop_parity_core)
  target_include_directories(nvcoop PRIVATE ${NVPY_INC} ${NVPY_REPO}/plugin/coop)
  target_sources(nvcoop PRIVATE ${NVPY_DIR}/parity_module.cpp)   # NVCOOP_MODULE must be a direct source
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_PARITY=1)
endif()
