# Rust + Qt examples

[![Build Status](https://travis-ci.com/rust-qt/examples.svg?branch=master)](https://travis-ci.com/rust-qt/examples)

## Setting up build environment

In addition to Rust's own build tools, you'll need to set up a C++ compiler, Qt, and CMake.

### C++ compiler

On Linux, install `gcc` from the repository.

On Windows, install Visual Studio (e.g. [Visual Studio Community 2017](https://www.visualstudio.com/thank-you-downloading-visual-studio/?sku=Community&rel=15)). Make sure to enable the component for C++ application development when installing Visual Studio. 

Visual Studio will create a starting menu option (e.g. `x64 Native Tools Command Prompt for VS 2017`) for starting command prompt with environment variables set up. You need to use it for all build operations, so that VS compiler and linker are available. 

On macOS, install Xcode Command Line Tools. The install can be initiated with the `xcode-select --install` command. You don't need a full Xcode installation.

### Qt

You can install Qt on any OS using the [official installer](https://www.qt.io/download). The installer allows you to select one of multiple available versions and builds. Make sure to select a `Desktop` build, not a mobile OS build. On Windows, also make sure to select a build corresponding to your Visual Studio version (e.g. `MSVC 2017`), not a MinGW build. Select a 64-bit version, not a 32-bit version.

On Linux and macOS, you can also install Qt development packages from the repository (or `brew`).

If Qt is not installed system-wide, you need to set up `PATH` to point at the directory where `qmake` executable is stored. On Linux and macOS:
```
export PATH="/usr/local/opt/qt/bin:$PATH"
```

On Windows (in the VS command prompt):
```
set PATH=C:\Qt\5.13.0\msvc2017_64\bin;%PATH%
```

### CMake

You'll also need `cmake`. On Linux and macOS, install it from the repository (or `brew`). 

On Windows, download the CMake installer at the [official site](https://cmake.org). During installation, choose to add `cmake` to the system `PATH`. You'll need to reopen command prompt or log out to apply the changes. Alternatively, add its installation directory to `PATH` in your prompt. 

Run `cmake --version` to verify that `cmake` is available.

### Verifying installation

To check that everything is set up correctly, try to build a C++/Qt project in your environment. If you've installed Qt via the official installer, it will store examples in the `Examples` directory of your Qt installation. You can also find them in the [Qt git repository](https://code.qt.io/cgit/qt/qtbase.git/tree/examples).

On Linux:
```
cd /tmp
mkdir build
cd build
qmake ~/Qt/Examples/Qt-5.13.0/widgets/dialogs/standarddialogs
make
./standarddialogs
```

On Windows (in the VS command prompt):
```
cd C:\tmp
mkdir build
cd build
qmake C:\Qt\Examples\Qt-5.13.0\widgets\dialogs\standarddialogs
nmake
release\standarddialogs.exe
```

Finally, you can try to build the Rust + Qt examples repo:
```
cd examples
cargo run --bin form1
```


# Deployment

All (or most) builds of Qt available in the official installer, Linux repositories, and brew are shared libraries or frameworks. This means that any executable built with these libraries will depend on Qt and won't run if Qt is not present on the end user's system.

It's possible to build Qt statically, so that you can build a standalone executable, but it's a more complicated process. Removing dependency on `vc-redist` dynamic library on Windows is also hard to do. It's much easier to use `macdeployqt` and `windeployqt` tools to create a directory that contains all required files. Rust-Qt crates don't support linking against static Qt builds.


This guide doesn't cover deploying QML projects yet. For QML, you will also need to copy your QML files, pass `--qmldir` flag to `*deployqt` tool, and make sure your application can find these files at runtime. Another approach is to put the QML files in Qt resources, but there is no Rust tooling for that yet.

## Linux

On Linux, the executables you get depend directly on Qt libraries, so the deployment process doesn't differ from deploying a C++/Qt application.

A common approach on Linux is to declare that your package depends on Qt libraries and only include your executable in the package. The system's package manager will ensure that Qt packages are installed. Refer to the documentation of the target Linux distributions for detailed instructions. 

See also the official [Qt for Linux/X11 - Deployment](https://doc.qt.io/qt-5/linux-deployment.html) guide for deploying a standalone application.

## macOS

On macOS, the executables you get depend directly on Qt libraries, so the deployment process doesn't differ from deploying a C++/Qt application.

See the official [Qt for macOS - Deployment](https://doc.qt.io/qt-5/macos-deployment.html) guide for deploying a standalone application.

## Windows

On Windows, Qt FFI wrappers are built as dynamic libraries because MSVC linker fails to link statically built libraries due to large amount of symbols. This means that in addition to Qt DLLs and resources, your executable also depends on the FFI wrapper DLLs.

`cargo` doesn't seem to provide an easy way to get paths to these DLLs, but you can find them manually:

```
> cd target\release
> dir *.dll /S /B | findstr \c_lib_install\
c:\(...)\target\release\build\qt_3d_core-978f68d93b02bd73\out\c_lib_install\qt_3d_core_c.dll
c:\(...)\target\release\build\qt_3d_extras-58593a661392f39f\out\c_lib_install\qt_3d_extras_c.dll
c:\(...)\target\release\build\qt_3d_input-03038c13aaba25c0\out\c_lib_install\qt_3d_input_c.dll
c:\(...)\target\release\build\qt_3d_logic-449a7502c6495738\out\c_lib_install\qt_3d_logic_c.dll
c:\(...)\target\release\build\qt_3d_render-a76854afbc715aa2\out\c_lib_install\qt_3d_render_c.dll
c:\(...)\target\release\build\qt_charts-d6a54029d499e74e\out\c_lib_install\qt_charts_c.dll
c:\(...)\target\release\build\qt_core-4950421d9d572f09\out\c_lib_install\qt_core_c.dll
c:\(...)\target\release\build\qt_gui-f713a5bc4d4da53e\out\c_lib_install\qt_gui_c.dll
c:\(...)\target\release\build\qt_qml-938892fb39a50c07\out\c_lib_install\qt_qml_c.dll
c:\(...)\target\release\build\qt_ui_tools-9c8697100cd55104\out\c_lib_install\qt_ui_tools_c.dll
c:\(...)\target\release\build\qt_widgets-ad85dfe845e3c1fa\out\c_lib_install\qt_widgets_c.dll
```

Copy your main executable and these wrapper DLLs to a new directory, then follow the official [Qt for Windows - Deployment](https://doc.qt.io/qt-5/windows-deployment.html) guide. The `windeployqt` will copy the required Qt's files to the directory. Note that the main executable doesn't depend on Qt directly, only wrapper DLLs do, so pass them to `windeployqt` instead. 

The following simple batch script will give you an idea on how it can be automated:
```
cd target\release
set OUT_DIR=c:\tmp\deploy
cp *.exe %OUT_DIR%\
dir *.dll /S /B | findstr \c_lib_install\ > %TEMP%\files.txt
for /F %%i in (%TEMP%\files.txt) do cp %%i %OUT_DIR%\
for /F %%i in ('dir %OUT_DIR%\*.dll /B /S') do windeployqt %%i
```

Note that executables produced by Visual Studio depend on Visual C++ Redistributable. `windeployqt` will copy the `vc_redist.x64.exe` installer to your destination directory, and your installer should run that to make sure the proper version of this library is available on the end user's system.

