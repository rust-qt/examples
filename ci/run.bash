#!/bin/bash

set -o errexit
set -x

export RUST_BACKTRACE=1

# Make sure the correct toolchain is used on Windows.
echo "$TRAVIS_RUST_VERSION" > rust-toolchain

rustup component add clippy
rustup component add rustfmt

cargo fmt -- --check

wget https://raw.githubusercontent.com/rust-qt/ritual/cfc696cdebd2f977ba55bf3cf572c18bd7b767af/scripts/install_qt.py -O /tmp/install_qt.py

mkdir ~/qt
cd ~/qt

if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    export HOMEBREW_NO_AUTO_UPDATE=1
    brew install p7zip

    PYTHON=python3
    PIP=pip3
    PIP_SUDO=
    XVFB=
    QT_OS=mac_x64
    QT_COMPILER=clang_64
    QT_SUBDIR=$QT_COMPILER
elif [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    sudo apt-get update
    sudo apt-get -y install python3-pip python3-setuptools xvfb p7zip-full libxkbcommon-x11-0 mesa-common-dev

    PYTHON=python3
    PIP=pip3
    PIP_SUDO=sudo
    XVFB="xvfb-run -a"
    QT_OS=linux_x64
    QT_COMPILER=gcc_64
    QT_SUBDIR=$QT_COMPILER
elif [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    choco install -y python --version=3.7.0
    choco install -y 7zip
    export PATH=/c/Python37:/c/Python37/Scripts:$PATH

    PYTHON=python
    PIP=pip
    PIP_SUDO=
    XVFB=
    QT_OS=windows_x86
    QT_COMPILER=win64_msvc2017_64
    QT_SUBDIR=msvc2017_64
fi

$PYTHON --version
$PIP_SUDO $PIP install 'bs4' 'lxml'

$PYTHON /tmp/install_qt.py $QT_VERSION $QT_OS $QT_COMPILER

QT_DIR=~/qt/$QT_VERSION/$QT_SUBDIR

export PATH=$QT_DIR/bin:$PATH
export LD_LIBRARY_PATH=$QT_DIR/lib:$LD_LIBRARY_PATH
export DYLD_FRAMEWORK_PATH=$QT_DIR/lib:$DYLD_FRAMEWORK_PATH
export QT_QPA_PLATFORM_PLUGIN_PATH=$QT_DIR/plugins
export QML2_IMPORT_PATH=$QT_DIR/qml

qmake -query

cd "$TRAVIS_BUILD_DIR"

function build() {
    if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
        cmd.exe //C 'C:\Program Files (x86)\Microsoft Visual Studio\2017\BuildTools\VC\Auxiliary\Build\vcvarsall.bat' amd64 '&&' "$@"
    else
        "$@"
    fi
}

build cargo clippy --color=always --all-targets -- -D warnings

build $XVFB cargo test --color=always --manifest-path widgets/todo_list/Cargo.toml -p qt_core -p qt_gui -p qt_widgets -p qt_ui_tools
build $XVFB cargo test --color=always --manifest-path 3d/lights/Cargo.toml -p qt_3d_core -p qt_3d_render -p qt_3d_input -p qt_3d_logic -p qt_3d_extras
build $XVFB cargo test --color=always --manifest-path charts/chart/Cargo.toml -p qt_charts
build $XVFB cargo test --color=always --manifest-path qml/hello_qml/Cargo.toml -p qt_qml

build $XVFB cargo build --color=always --all-targets -v

ARGS=*
build $XVFB cargo run --color=always -p mime_types -- $ARGS
