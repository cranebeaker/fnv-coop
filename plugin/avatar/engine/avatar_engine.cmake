# The real puppet engine (plugin/avatar/engine). include()d by plugin/avatar/CMakeLists.txt, so NVAV_DIR / NVAV_SYNC / NVAV_EXCLUDE are set.
#
#   nvcoop_avatar_engine      game-free policy (GamePuppetEngine, LocalSampler, AvatarRuntime); what nvcoop.dll links. NO mutation twins.
#   nvcoop_avatar_engine_rt   the same sources compiled with FNVCOOP_RUNTIME_MUT: only the test exe links it (never both into one binary).
#   avatar_engine_tests       unit + integration tests on FakeGameBackend; FNVCOOP_MUT=<n> picks a twin (plugin\avatar\engine\run_tests.cmd).
#   nvcoop sources            win/game_backend_win.cpp + win/avatar_module.cpp (x86 only, xNVSE headers): the only code that touches the game.
set(NVAE_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVAE_MUT_INC "${NVAE_DIR}/../../../tools/mut")
set(NVAE_SRC ${NVAE_DIR}/puppet_game_engine.cpp ${NVAE_DIR}/local_sampler.cpp ${NVAE_DIR}/avatar_runtime.cpp)

add_library(nvcoop_avatar_engine STATIC ${NVAV_EXCLUDE} ${NVAE_SRC})
add_library(nvcoop_avatar_engine_rt STATIC ${NVAV_EXCLUDE} ${NVAE_SRC})
target_compile_definitions(nvcoop_avatar_engine_rt PUBLIC FNVCOOP_RUNTIME_MUT)
foreach(t nvcoop_avatar_engine nvcoop_avatar_engine_rt)
  target_include_directories(${t} PUBLIC ${NVAE_DIR} ${NVAV_DIR} ${NVAV_SYNC} ${NVAV_SYNC}/include PRIVATE ${NVAE_MUT_INC})
  target_link_libraries(${t} PUBLIC nvcoop_avatar)
  if(MSVC)
    target_compile_options(${t} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4100 /wd4127 /D_CRT_SECURE_NO_WARNINGS)
  else()
    target_compile_options(${t} PRIVATE -Wall -Wextra -Wno-unused-parameter)
  endif()
endforeach()

# Interpolation buffer + snapshot codec. The plugin build already has them in nvcoop_actors_live (same sources), so reuse that one.
if(TARGET nvcoop_actors_live)
  set(NVAE_LIVE nvcoop_actors_live)
else()
  add_library(nvcoop_avatar_live STATIC ${NVAV_EXCLUDE} ${NVAV_SYNC}/live/live.cpp)
  target_include_directories(nvcoop_avatar_live PUBLIC ${NVAV_SYNC} ${NVAV_SYNC}/include)
  if(TARGET nvcoop_coop_synccore)
    target_link_libraries(nvcoop_avatar_live PUBLIC nvcoop_coop_synccore)
  else()
    target_sources(nvcoop_avatar_live PRIVATE ${NVAV_SYNC}/src/wire.cpp)
  endif()
  if(MSVC)
    target_compile_options(nvcoop_avatar_live PRIVATE /W3 /EHsc /permissive- /utf-8 /wd4456 /D_CRT_SECURE_NO_WARNINGS)
  endif()
  set(NVAE_LIVE nvcoop_avatar_live)
endif()

add_executable(avatar_engine_tests ${NVAV_EXCLUDE} ${NVAE_DIR}/tests/test_engine.cpp)
target_include_directories(avatar_engine_tests PRIVATE ${NVAE_DIR} ${NVAV_DIR} ${NVAV_DIR}/.. ${NVAE_MUT_INC})
target_link_libraries(avatar_engine_tests PRIVATE nvcoop_avatar_engine_rt ${NVAE_LIVE})
if(MSVC)
  target_compile_options(avatar_engine_tests PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4100 /D_CRT_SECURE_NO_WARNINGS)
else()
  target_compile_options(avatar_engine_tests PRIVATE -Wall -Wextra -Wno-unused-parameter)
endif()
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(avatar_engine_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/avatar_tests")
endforeach()

set(NVAE_TWINS 1-34 40-56 60-71 80-85)
add_test(NAME avatar_engine COMMAND avatar_engine_tests)
foreach(v 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56
          60 61 62 63 64 65 66 67 68 69 70 71 80 81 82 83 84 85)
  add_test(NAME avatar_engine_mut${v} COMMAND avatar_engine_tests)
  set_tests_properties(avatar_engine_mut${v} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${v}")
endforeach()

# How far behind the remote player's puppet appears (round of 2026-10-27): walking tracks through the real sender chain, the chaos profiles and the
# real receiver, before / after (tests/test_lag_sim.cpp). Twins 201..208 (FNVCOOP_MUT) are in the same exe. With ActorSync in the tree the same tracks also run through its
# player stream profile (--path actorsync) and --gate compares the two paths (docs/entity-core-plan.md S10, S11).
add_executable(avatar_lag_tests ${NVAV_EXCLUDE} ${NVAE_DIR}/tests/test_lag_sim.cpp)
target_include_directories(avatar_lag_tests PRIVATE ${NVAE_DIR} ${NVAV_DIR} ${NVAV_DIR}/.. ${NVAE_MUT_INC} ${NVAV_SYNC}/chaos)
target_link_libraries(avatar_lag_tests PRIVATE nvcoop_avatar_engine_rt ${NVAE_LIVE})
if(TARGET nvcoop_actors_core)   # the stream through ActorSync's player profile (--path actorsync, --gate); not in the standalone avatar build
  target_link_libraries(avatar_lag_tests PRIVATE nvcoop_actors_core)
  target_compile_definitions(avatar_lag_tests PRIVATE LAG_HAVE_ACTORSYNC=1)
endif()
if(MSVC)
  target_compile_options(avatar_lag_tests PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4100 /D_CRT_SECURE_NO_WARNINGS)
else()
  target_compile_options(avatar_lag_tests PRIVATE -Wall -Wextra -Wno-unused-parameter)
endif()
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(avatar_lag_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/avatar_tests")
endforeach()
add_test(NAME avatar_lag COMMAND avatar_lag_tests)
foreach(v 201 202 203 204 205 206 207 208)
  add_test(NAME avatar_lag_mut${v} COMMAND avatar_lag_tests)
  set_tests_properties(avatar_lag_mut${v} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${v}")
endforeach()

# ---- the game side, compiled into nvcoop.dll (x86 only; needs the xNVSE headers nvcoop already has) ----
option(NVCOOP_WITH_AVATAR_ENGINE "link the real puppet engine (plugin/avatar/engine) into nvcoop.dll; inert unless nvcoop.ini [avatar] enabled=1" ON)
if(NVCOOP_WITH_AVATAR_ENGINE AND MSVC AND CMAKE_SIZEOF_VOID_P EQUAL 4 AND TARGET nvcoop
   AND EXISTS ${NVAE_DIR}/win/game_backend_win.cpp AND EXISTS ${NVAE_DIR}/win/avatar_module.cpp)
  target_sources(nvcoop PRIVATE ${NVAE_DIR}/win/game_backend_win.cpp ${NVAE_DIR}/win/avatar_module.cpp)
  target_link_libraries(nvcoop PRIVATE nvcoop_avatar_engine ${NVAE_LIVE})
  target_include_directories(nvcoop PRIVATE ${NVAE_DIR} ${NVAV_DIR} ${NVAV_SYNC} ${NVAV_SYNC}/include)
  target_compile_definitions(nvcoop PRIVATE NVCOOP_WITH_AVATAR_ENGINE=1)
endif()
