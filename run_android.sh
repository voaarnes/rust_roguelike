#!/bin/bash
# Quick run script - rebuilds and deploys

./build_android.sh

# Follow logs
if [ -f "/mnt/c/platform-tools/adb.exe" ]; then
    /mnt/c/platform-tools/adb.exe logcat -s rust_roguelike:V RustStdoutStderr:V
elif command -v adb &> /dev/null; then
    adb logcat -s rust_roguelike:V RustStdoutStderr:V
fi
