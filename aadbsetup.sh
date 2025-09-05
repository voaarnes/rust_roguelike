#!/bin/bash
# Setup script for Bevy Android development in WSL

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install Android targets for Rust
echo "Adding Android targets for Rust..."
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# Install cargo-apk for building Android APKs
echo "Installing cargo-apk..."
cargo install cargo-apk

# Install Android NDK and SDK dependencies
echo "Installing Android build dependencies..."
sudo apt update
sudo apt install -y unzip wget openjdk-17-jdk

# Download Android NDK (required for Rust cross-compilation)
echo "Setting up Android NDK..."
mkdir -p ~/android-ndk
cd ~/android-ndk
wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip
unzip -q android-ndk-r25c-linux.zip
rm android-ndk-r25c-linux.zip

# Set up environment variables
cat >> ~/.bashrc << 'EOL'

# Android NDK for Bevy
export ANDROID_NDK_ROOT=$HOME/android-ndk/android-ndk-r25c
export PATH=$PATH:$ANDROID_NDK_ROOT
EOL

source ~/.bashrc

echo "Setup complete! Now configure your Bevy project for Android."
