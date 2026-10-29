# fnvsteam.exe: the launcher's helper for co-op over Steam (docs/steam.md). Standalone build, x86 like the game, static runtime so a player needs
# nothing installed:   python -I tools/build/build.py --source tools/steamlink --build build/steamlink --target fnvsteam
# The exe lands in out/steamlink/ and tools/installer/package_release.py ships it as launcher\fnvsteam.exe.
cmake_minimum_required(VERSION 3.20)
project(fnvsteam CXX)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
set(REPO "${CMAKE_CURRENT_LIST_DIR}/../..")
add_executable(fnvsteam main.cpp ${REPO}/plugin/coop/steam_binding.cpp ${REPO}/synccore/src/steam_transport.cpp)
target_include_directories(fnvsteam PRIVATE ${REPO}/synccore/include ${REPO}/plugin ${REPO}/plugin/coop)
target_compile_definitions(fnvsteam PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
if(MSVC)
  target_compile_options(fnvsteam PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4100)
endif()
target_link_libraries(fnvsteam PRIVATE ws2_32)
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(fnvsteam PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${REPO}/out/steamlink")
endforeach()
