@echo off
rem Display settings for nv-rs: window or borderless, resolution and more. Applied at the next start.
start "" powershell -NoProfile -ExecutionPolicy Bypass -STA -WindowStyle Hidden -File "%~dp0Settings.ps1"
