# social: game-free followers (companions), crime / alarms / bounty / hostility and kill rewards (FULL-SYNC-PLAN 3.4, 3.8, 3.10). Included from synccore/CMakeLists.txt.
#   fnvcoop_social     production library (twins compiled out). Linked by plugin/companions.
#   fnvcoop_social_rt  the same sources with FNVCOOP_RUNTIME_MUT: mutation twins 801..899 are switched with FNVCOOP_MUT=<n> at run time.
#   social_tests       the unit tests (links the _rt library). One command: synccore\social\run_tests.cmd  (prints ALL OK)
set(SOCIAL_SRC ${CMAKE_CURRENT_LIST_DIR}/companions.cpp ${CMAKE_CURRENT_LIST_DIR}/slots.cpp ${CMAKE_CURRENT_LIST_DIR}/escort.cpp ${CMAKE_CURRENT_LIST_DIR}/crime.cpp ${CMAKE_CURRENT_LIST_DIR}/rewards.cpp ${CMAKE_CURRENT_LIST_DIR}/wire.cpp)

foreach(sfx "" "_rt")
  add_library(fnvcoop_social${sfx} STATIC ${SOCIAL_SRC})
  # "social/companions.h" resolves from synccore/; "combat/combat.h" too
  target_include_directories(fnvcoop_social${sfx} PUBLIC ${CMAKE_CURRENT_LIST_DIR}/.. ${CMAKE_CURRENT_LIST_DIR}/../combat)
  if(sfx STREQUAL "_rt")
    target_link_libraries(fnvcoop_social_rt PUBLIC fnvcoop_combat_rt)
    target_compile_definitions(fnvcoop_social_rt PUBLIC FNVCOOP_RUNTIME_MUT)
  else()
    target_link_libraries(fnvcoop_social PUBLIC fnvcoop_combat)
  endif()
endforeach()

add_executable(social_tests tests/test_companions.cpp tests/test_party.cpp tests/test_crime.cpp tests/test_rewards.cpp tests/test_wire.cpp ${CMAKE_CURRENT_LIST_DIR}/../tests/main.cpp)
target_include_directories(social_tests PRIVATE ${CMAKE_CURRENT_LIST_DIR}/../tests ${CMAKE_CURRENT_LIST_DIR}/tests)
target_link_libraries(social_tests PRIVATE fnvcoop_social_rt)

if(MSVC)
  foreach(t fnvcoop_social fnvcoop_social_rt social_tests)
    target_compile_options(${t} PRIVATE /W4 /permissive- /utf-8 /wd4456 /D_CRT_SECURE_NO_WARNINGS)
  endforeach()
else()
  foreach(t fnvcoop_social fnvcoop_social_rt social_tests)
    target_compile_options(${t} PRIVATE -Wall -Wextra)
  endforeach()
endif()

add_test(NAME social_unit COMMAND social_tests)
# the twins (FNVCOOP_MUT=801..899) are run by synccore/social/run_tests.cmd (tools/mut/run_twins.py); ctest only runs the real suite
