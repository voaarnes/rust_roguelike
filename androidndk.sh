#!/bin/bash
# Complete Android SDK/NDK setup for rust_roguelike
# Run this first before building

echo "Installing Android SDK and NDK for Bevy development..."

# Create Android directories
mkdir -p ~/android-sdk
mkdir -p ~/android-ndk

# Install Java if not present
if ! command -v java &> /dev/null; then
    echo "Installing Java..."
    sudo apt update
    sudo apt install -y openjdk-17-jdk
fi

# Download and install Android Command Line Tools
echo "Downloading Android SDK Command Line Tools..."
cd ~/android-sdk
wget -q https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip
unzip -q commandlinetools-linux-11076708_latest.zip
rm commandlinetools-linux-11076708_latest.zip

# Fix directory structure for cmdline-tools
mkdir -p cmdline-tools/latest
mv cmdline-tools/* cmdline-tools/latest/ 2>/dev/null || true

# Download and install Android NDK
echo "Downloading Android NDK..."
cd ~/android-ndk
wget -q https://dl.google.com/android/repository/android-ndk-r25c-linux.zip
echo "Extracting NDK (this may take a minute)..."
unzip -q android-ndk-r25c-linux.zip
rm android-ndk-r25c-linux.zip

# Set up environment variables
echo "Setting up environment variables..."
cat >> ~/.bashrc << 'EOL'

# Android Development
export ANDROID_HOME=$HOME/android-sdk
export ANDROID_SDK_ROOT=$HOME/android-sdk
export ANDROID_NDK_ROOT=$HOME/android-ndk/android-ndk-r25c
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
export PATH=$PATH:$ANDROID_HOME/platform-tools
export PATH=$PATH:$ANDROID_NDK_ROOT
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
EOL

# Export for current session
export ANDROID_HOME=$HOME/android-sdk
export ANDROID_SDK_ROOT=$HOME/android-sdk
export ANDROID_NDK_ROOT=$HOME/android-ndk/android-ndk-r25c
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
export PATH=$PATH:$ANDROID_HOME/platform-tools
export PATH=$PATH:$ANDROID_NDK_ROOT
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64

# Accept licenses and install required SDK components
echo "Installing Android SDK components..."
cd $ANDROID_HOME/cmdline-tools/latest/bin

# Auto-accept all licenses
yes | ./sdkmanager --licenses 2>/dev/null

# Install required SDK packages
./sdkmanager "platform-tools" "platforms;android-33" "build-tools;33.0.2" "platforms;android-24"

echo ""
echo "========================================"
echo "Android SDK/NDK Installation Complete!"
echo "========================================"
echo ""
echo "Installed:"
echo "✓ Android SDK at: $ANDROID_HOME"
echo "✓ Android NDK at: $ANDROID_NDK_ROOT"
echo "✓ Platform tools and build tools"
echo ""
echo "Now run: source ~/.bashrc"
echo "Then: ./build_android.sh"
