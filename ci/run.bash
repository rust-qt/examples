#!/bin/bash

set -o errexit
set -x

export RUST_BACKTRACE=1

wget https://raw.githubusercontent.com/rust-qt/ritual/37cc01f27e2525fb9f6d5882f447089e2ad5d4bf/scripts/install_qt.py -O /tmp/install_qt.py

python --version

mkdir ~/qt
cd ~/qt
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    python /tmp/install_qt.py 5.13.0 mac_x64 clang_64
elif [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    python /tmp/install_qt.py 5.13.0 linux_x64 gcc_x64
elif [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    python /tmp/install_qt.py 5.13.0 windows_x86 win64_msvc2017_64
fi
