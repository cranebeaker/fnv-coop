# The whole avatar chain (plugin/avatar/full), included by plugin/avatar/CMakeLists.txt AFTER engine/avatar_engine.cmake.
#   avatar_full_tests   three machines on a lossy in-memory link: the REAL AvatarRuntime (on a fake game), the REAL AvatarNet (live hub, Roster /
#                       Look / snapshot streams with the 20 Hz near / 5 Hz far rate), the REAL live codec, the combat rules (read-only).
#   Twins 101..109 live in plugin/coop/live/avatar_wire.cpp and avatar_net.cpp; their sources are compiled a second time HERE with
#   FNVCOOP_RUNTIME_MUT (the dll-wiring suite compiles them for its own twins 31..51); the puppet engine and the combat rules are the PRODUCTION
#   builds, so their own twins (1..85, 301..322) never fire in this binary.
# Needs the shared plugin build (nvcoop_coop_synccore, nvcoop_actors_live); skipped in the standalone avatar build.
set(NVF_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(NVF_LIVE "${NVF_DIR}/../../coop/live")
set(NVF_REPO "${NVF_DIR}/../../..")
if(NOT (TARGET nvcoop_coop_synccore AND TARGET nvcoop_actors_live AND TARGET nvcoop_avatar_engine AND TARGET nvcoop_actors_core))
  return()
endif()

function(nvf_flags tgt)
  if(MSVC)
    target_compile_options(${tgt} PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4456 /wd4100 /wd4127 /wd4245 /wd4005)
  endif()
  target_compile_definitions(${tgt} PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
endfunction()

# the combat rules, production flavour (read-only use: HitValidator, applyHit)
add_library(nvcoop_avatar_full_combat STATIC ${NVAV_EXCLUDE} ${NVF_REPO}/synccore/combat/combat.cpp)
target_include_directories(nvcoop_avatar_full_combat PUBLIC ${NVF_REPO}/synccore/combat ${NVF_REPO}/synccore ${NVF_REPO}/synccore/include)
target_link_libraries(nvcoop_avatar_full_combat PUBLIC nvcoop_coop_synccore)
nvf_flags(nvcoop_avatar_full_combat)

# the network service and the hub, runtime-twin flavour
add_library(nvcoop_avatar_full_net STATIC ${NVAV_EXCLUDE} ${NVF_LIVE}/avatar_net.cpp ${NVF_LIVE}/avatar_wire.cpp ${NVF_LIVE}/live_hub.cpp ${NVF_LIVE}/player_port.cpp
  ${NVF_DIR}/../look_codec.cpp)
target_include_directories(nvcoop_avatar_full_net PUBLIC ${NVF_LIVE} ${NVF_REPO}/tools/mut ${NVF_REPO}/plugin)
target_link_libraries(nvcoop_avatar_full_net PUBLIC nvcoop_avatar_engine nvcoop_actors_live nvcoop_coop_synccore)
target_link_libraries(nvcoop_avatar_full_net PUBLIC nvcoop_actors_core)   # PlayerPort is an IActorPort (state_parts.cpp)
target_compile_definitions(nvcoop_avatar_full_net PUBLIC FNVCOOP_RUNTIME_MUT)
nvf_flags(nvcoop_avatar_full_net)

add_executable(avatar_full_tests ${NVAV_EXCLUDE} ${NVF_DIR}/tests/test_full.cpp)
target_link_libraries(avatar_full_tests PRIVATE nvcoop_avatar_full_net nvcoop_avatar_full_combat)
nvf_flags(avatar_full_tests)
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(avatar_full_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${CMAKE_BINARY_DIR}/avatar_tests")
endforeach()
add_test(NAME avatar_full COMMAND avatar_full_tests)
foreach(v 101 102 103 104 105 106 107 108 109)
  add_test(NAME avatar_full_mut${v} COMMAND avatar_full_tests)
  set_tests_properties(avatar_full_mut${v} PROPERTIES WILL_FAIL TRUE ENVIRONMENT "FNVCOOP_MUT=${v}")
endforeach()
