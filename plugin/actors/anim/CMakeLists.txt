# plugin/actors/anim: NPC animation sync (enemy-anim). Added by plugin/actors/CMakeLists.txt (so nvcoop_actors_core / nvcoop_actors_live exist).
#   nvcoop_anim_core     production library (sampler, driver, corpse, system, script backend). Linked into nvcoop.dll with the game glue.
#   nvcoop_anim_core_rt  the same sources with run-time mutation twins (tools/mut/fnvcoop_mut.h), tests only.
#   nvcoop_anim_tests    ONE test exe; FNVCOOP_MUT=<n> picks a twin. Run: plugin\actors\anim\run_tests.cmd
set(NVN_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVN_REPO "${NVN_DIR}/../../..")
set(NVN_MUT_INC "${NVN_REPO}/tools/mut")
set(NVN_INC "${NVN_DIR}" "${NVN_DIR}/../.." "${NVN_REPO}/synccore" "${NVN_REPO}/synccore/include" "${NVN_MUT_INC}")
set(NVN_SRC ${NVN_DIR}/corpse.cpp ${NVN_DIR}/npc_anim_sampler.cpp ${NVN_DIR}/npc_anim_driver.cpp ${NVN_DIR}/npc_anim_system.cpp ${NVN_DIR}/script_npc_engine.cpp)
set(NVN_MUTANTS 1 2 3 4 5 6 7 8 20 21 22 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 60 61 62 63 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87)

function(nvn_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

add_library(nvcoop_anim_core STATIC ${NVN_SRC})
target_include_directories(nvcoop_anim_core PUBLIC ${NVN_INC})
target_link_libraries(nvcoop_anim_core PUBLIC nvcoop_actors_live)
nvn_flags(nvcoop_anim_core)

add_library(nvcoop_anim_core_rt STATIC EXCLUDE_FROM_ALL ${NVN_SRC})
target_include_directories(nvcoop_anim_core_rt PUBLIC ${NVN_INC})
target_link_libraries(nvcoop_anim_core_rt PUBLIC nvcoop_actors_live)
target_compile_definitions(nvcoop_anim_core_rt PUBLIC FNVCOOP_RUNTIME_MUT)
nvn_flags(nvcoop_anim_core_rt)

add_executable(nvcoop_anim_tests EXCLUDE_FROM_ALL
  ${NVN_DIR}/tests/test_sampler.cpp ${NVN_DIR}/tests/test_driver.cpp ${NVN_DIR}/tests/test_sit.cpp ${NVN_DIR}/tests/test_pipeline.cpp ${NVN_REPO}/synccore/tests/main.cpp)
target_include_directories(nvcoop_anim_tests PRIVATE ${NVN_DIR}/tests ${NVN_REPO}/synccore/tests ${NVN_DIR}/../tests)
target_link_libraries(nvcoop_anim_tests PRIVATE nvcoop_anim_core_rt nvcoop_actors_core)
nvn_flags(nvcoop_anim_tests)
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(nvcoop_anim_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/anim_tests")
endforeach()

enable_testing()
add_test(NAME anim_unit COMMAND nvcoop_anim_tests)
foreach(v ${NVN_MUTANTS})
  add_test(NAME anim_mut${v} COMMAND nvcoop_anim_tests)
  set_tests_properties(anim_mut${v} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${v}")
endforeach()

# ---- game glue (x86 only): the module (self-registering) and the xNVSE-layout backend are direct sources of nvcoop.dll ----
if(MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND TARGET nvcoop AND TARGET nvcoop_actor_engine_core AND TARGET nvcoop_hooks_game AND TARGET fnvcoop_nvse_gameapi
   AND EXISTS "${NVN_DIR}/anim_module.cpp" AND EXISTS "${NVN_DIR}/anim_game.cpp")
  target_link_libraries(nvcoop PRIVATE nvcoop_anim_core nvcoop_actor_engine_core nvcoop_hooks_game)
  target_sources(nvcoop PRIVATE ${NVN_DIR}/anim_module.cpp ${NVN_DIR}/anim_game.cpp)
  target_include_directories(nvcoop PRIVATE ${NVN_INC})
endif()
