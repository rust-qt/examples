#!/bin/bash

set -o errexit
set -x

export RUST_BACKTRACE=1


wget https://raw.githubusercontent.com/rust-qt/ritual/37cc01f27e2525fb9f6d5882f447089e2ad5d4bf/scripts/install_qt.py -O /tmp/install_qt.py

mkdir ~/qt
cd ~/qt
QT_VERSION=5.13.0
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    brew install p7zip

    PYTHON=python3
    QT_OS=mac_x64
    QT_COMPILER=clang_64
    QT_SUBDIR=$QT_COMPILER
elif [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    sudo apt-get -y install python3-pip p7zip

    PYTHON=python3
    QT_OS=linux_x64
    QT_COMPILER=gcc_64
    QT_SUBDIR=$QT_COMPILER
elif [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    choco install -y python3 7zip
    export PATH=/c/Python37:/c/Python37/Scripts:$PATH

    PYTHON=python
    QT_OS=windows_x86
    QT_COMPILER=win64_msvc2017_64
    QT_SUBDIR=msvc2017_64
fi

$PYTHON --version
pip3 install bs4 lxml

$PYTHON /tmp/install_qt.py $QT_VERSION $QT_OS $QT_COMPILER

QT_DIR=~/qt/$QT_VERSION/$QT_SUBDIR

export PATH=$QT_DIR/bin:$PATH
export LD_LIBRARY_PATH=$QT_DIR/lib:$LD_LIBRARY_PATH
export DYLD_FRAMEWORK_PATH=$QT_DIR/lib:$DYLD_FRAMEWORK_PATH
export QT_QPA_PLATFORM_PLUGIN_PATH=$QT_DIR/plugins
export QML2_IMPORT_PATH=$QT_DIR/qml

qmake -query

cd "$TRAVIS_BUILD_DIR"

if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
cmd.exe /C '"C:\Program Files (x86)\Microsoft Visual Studio\2017\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" amd64 && cargo build --all-targets -v'
else
    cargo build --all-targets -v
fi
