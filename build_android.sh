#!/bin/bash
# Build and deploy to Android device

set -e

echo "Building for Android..."

# Ensure Android NDK is set (adjust path as needed)
export ANDROID_NDK_ROOT=${ANDROID_NDK_ROOT:-"$HOME/android-ndk/android-ndk-r25c"}

# Build the APK
cargo apk build --release

APK_PATH="target/release/apk/rust_roguelike.apk"

if [ ! -f "$APK_PATH" ]; then
    echo "Error: APK not found at $APK_PATH"
    exit 1
fi

echo "APK built successfully at: $APK_PATH"

# Check for Windows ADB
ADB_CMD=""
if [ -f "/mnt/c/platform-tools/adb.exe" ]; then
    ADB_CMD="/mnt/c/platform-tools/adb.exe"
elif command -v adb &> /dev/null; then
    ADB_CMD="adb"
else
    echo "ADB not found. Please install Android platform-tools."
    echo "APK location: $APK_PATH"
    exit 1
fi

# Check for connected device
echo "Checking for connected devices..."
DEVICES=$($ADB_CMD devices | grep -v "List" | grep "device$")

if [ -z "$DEVICES" ]; then
    echo "No devices connected. Please:"
    echo "1. Connect your Android phone via USB"
    echo "2. Enable Developer Mode and USB Debugging"
    echo "3. Accept the debugging prompt on your phone"
    exit 1
fi

echo "Installing on device..."
$ADB_CMD install -r "$APK_PATH"

echo "Launching game..."
$ADB_CMD shell am start -n com.roguelike.rust_roguelike/android.app.NativeActivity

echo ""
echo "Game launched! To view logs:"
echo "$ADB_CMD logcat -s rust_roguelike:V RustStdoutStderr:V"
