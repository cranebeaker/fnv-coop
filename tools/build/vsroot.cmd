@echo off
rem Sets VSROOT to the Visual Studio 2022 install with the C++ tools (any edition). Same order as vsroot.py:
rem a VSROOT already set wins, then vswhere, then the Community path the scripts used to hard-code.
if defined VSROOT exit /b 0
set "_VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if exist "%_VSWHERE%" for /f "usebackq delims=" %%i in (`"%_VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do if not defined VSROOT set "VSROOT=%%i"
set "_VSWHERE="
if not defined VSROOT set "VSROOT=C:\Program Files\Microsoft Visual Studio\2022\Community"
exit /b 0
