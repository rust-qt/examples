#!/bin/bash

set -o errexit
set -x

export RUST_BACKTRACE=1

python3 --version

wget https://raw.githubusercontent.com/rust-qt/ritual/37cc01f27e2525fb9f6d5882f447089e2ad5d4bf/scripts/install_qt.py -O /tmp/install_qt.py

pyenv install --list
pyenv install 3.6.3
pyenv global 3.6.3
python --version

pip install bs4

mkdir ~/qt
cd ~/qt
QT_VERSION=5.13.0
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    QT_OS=mac_x64
    QT_COMPILER=clang_64
    QT_SUBDIR=$QT_COMPILER
elif [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    QT_OS=linux_x64
    QT_COMPILER=gcc_64
    QT_SUBDIR=$QT_COMPILER
elif [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    QT_OS=windows_x86
    QT_COMPILER=win64_msvc2017_64
    QT_SUBDIR=msvc2017_64
fi

python /tmp/install_qt.py $QT_VERSION $QT_OS $QT_COMPILER
QT_DIR=~/qt/$QT_VERSION/$QT_SUBDIR

export PATH=$QT_DIR/bin:$PATH
export LD_LIBRARY_PATH=$QT_DIR/lib:$LD_LIBRARY_PATH
export QT_QPA_PLATFORM_PLUGIN_PATH=$QT_DIR/plugins
export QML2_IMPORT_PATH=$QT_DIR/qml

cd "$TRAVIS_BUILD_DIR"

if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
cmd.exe /C '"C:\Program Files (x86)\Microsoft Visual Studio\2017\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" amd64 && cargo build --all-targets -v'
else
    cargo build --all-targets -v
fi
