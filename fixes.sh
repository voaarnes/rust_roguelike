#!/bin/bash
# Fix missing Android platform 30

echo "Installing missing Android platform 30..."

# Install the required platform
$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager "platforms;android-30"

# Also install some commonly needed platforms for compatibility
$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager \
    "platforms;android-30" \
    "platforms;android-31" \
    "platforms;android-32" \
    "platforms;android-33" \
    "build-tools;30.0.3"

# Fix the deprecated ANDROID_SDK_ROOT warning
sed -i '/ANDROID_SDK_ROOT/d' ~/.bashrc
echo "export ANDROID_HOME=$HOME/android-sdk" >> ~/.bashrc
source ~/.bashrc

echo "Platform 30 installed!"
echo "Retrying build..."

# Now run the build
./build_android.sh
