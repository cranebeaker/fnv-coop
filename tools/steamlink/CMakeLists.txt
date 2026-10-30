# fnvsteam.exe: the launcher's helper for co-op over Steam (docs/steam.md). Standalone build, x86 like the game, static runtime so a player needs
# nothing installed:   python -I tools/build/build.py --source tools/steamlink --build build/steamlink --target fnvsteam
# The exe lands in out/steamlink/ and tools/installer/package_release.py ships it as launcher\fnvsteam.exe, next to Valve's steam_api.dll
# (loaded by name at run time: no Steamworks SDK header or import library is used) and steam_appid.txt.
cmake_minimum_required(VERSION 3.20)
project(fnvsteam CXX)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
set(REPO "${CMAKE_CURRENT_LIST_DIR}/../..")
add_executable(fnvsteam main.cpp steam_backend.cpp mock_backend.cpp)
target_include_directories(fnvsteam PRIVATE ${REPO}/plugin)
target_compile_definitions(fnvsteam PRIVATE _CRT_SECURE_NO_WARNINGS WIN32_LEAN_AND_MEAN NOMINMAX)
if(MSVC)
  target_compile_options(fnvsteam PRIVATE /W4 /EHsc /permissive- /utf-8 /wd4100)
endif()
target_link_libraries(fnvsteam PRIVATE ws2_32)
foreach(cfg "" _DEBUG _RELEASE _RELWITHDEBINFO _MINSIZEREL)
  set_target_properties(fnvsteam PROPERTIES RUNTIME_OUTPUT_DIRECTORY${cfg} "${REPO}/out/steamlink")
endforeach()
