# Rust + Qt examples

[![Build Status](https://travis-ci.com/rust-qt/examples.svg?branch=master)](https://travis-ci.com/rust-qt/examples)

This repository contains examples of using Qt from Rust. This readme is also a beginner's guide.

Qt crates are generated with [ritual](https://github.com/rust-qt/ritual). This project maintains the following Qt crates (more crates may be added in the future):

| Crate       | Version | Docs |
| ----------- | ------- | ---- |
| qt_core     | [![](http://meritbadge.herokuapp.com/qt_core)](https://crates.io/crates/qt_core) | [![](https://docs.rs/qt_core/badge.svg)](https://docs.rs/qt_core) |
| qt_gui      | [![](http://meritbadge.herokuapp.com/qt_gui)](https://crates.io/crates/qt_gui) | [![](https://docs.rs/qt_gui/badge.svg)](https://docs.rs/qt_gui) |
| qt_widgets  | [![](http://meritbadge.herokuapp.com/qt_widgets)](https://crates.io/crates/qt_widgets) | [![](https://docs.rs/qt_widgets/badge.svg)](https://docs.rs/qt_widgets) |
| qt_ui_tools | [![](http://meritbadge.herokuapp.com/qt_ui_tools)](https://crates.io/crates/qt_ui_tools) | [![](https://docs.rs/qt_ui_tools/badge.svg)](https://docs.rs/qt_ui_tools) |
| qt_3d_core | [![](http://meritbadge.herokuapp.com/qt_3d_core)](https://crates.io/crates/qt_3d_core) | [![](https://docs.rs/qt_3d_core/badge.svg)](https://docs.rs/qt_3d_core) |
| qt_3d_render | [![](http://meritbadge.herokuapp.com/qt_3d_render)](https://crates.io/crates/qt_3d_render) | [![](https://docs.rs/qt_3d_render/badge.svg)](https://docs.rs/qt_3d_render) |
| qt_3d_input | [![](http://meritbadge.herokuapp.com/qt_3d_input)](https://crates.io/crates/qt_3d_input) | [![](https://docs.rs/qt_3d_input/badge.svg)](https://docs.rs/qt_3d_input) |
| qt_3d_logic | [![](http://meritbadge.herokuapp.com/qt_3d_logic)](https://crates.io/crates/qt_3d_logic) | [![](https://docs.rs/qt_3d_logic/badge.svg)](https://docs.rs/qt_3d_logic) |
| qt_3d_extras | [![](http://meritbadge.herokuapp.com/qt_3d_extras)](https://crates.io/crates/qt_3d_extras) | [![](https://docs.rs/qt_3d_extras/badge.svg)](https://docs.rs/qt_3d_extras) |
| qt_charts | [![](http://meritbadge.herokuapp.com/qt_charts)](https://crates.io/crates/qt_charts) | [![](https://docs.rs/qt_charts/badge.svg)](https://docs.rs/qt_charts) |
| qt_qml | [![](http://meritbadge.herokuapp.com/qt_qml)](https://crates.io/crates/qt_qml) | [![](https://docs.rs/qt_qml/badge.svg)](https://docs.rs/qt_qml) |

Supported environments: 64-bit Linux, 64-bit Windows (msvc toolchain), 64-bit macOS.

Supported Qt versions: from 5.9 to 5.13.

# Setting up build environment

In addition to Rust's own build tools, you'll need to set up a C++ compiler, Qt, and CMake.

## C++ compiler

On Linux, install `gcc` from the repository.

On Windows, install Visual Studio (e.g. [Visual Studio Community 2017](https://www.visualstudio.com/thank-you-downloading-visual-studio/?sku=Community&rel=15)). Make sure to enable the component for C++ application development when installing Visual Studio. 

Visual Studio will create a starting menu option (e.g. `x64 Native Tools Command Prompt for VS 2017`) for starting command prompt with environment variables set up. You need to use it for all build operations, so that VS compiler and linker are available. 

On macOS, install Xcode Command Line Tools. The install can be initiated with the `xcode-select --install` command. You don't need a full Xcode installation.

## Qt

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

## CMake

You'll also need `cmake`. On Linux and macOS, install it from the repository (or `brew`). 

On Windows, download the CMake installer at the [official site](https://cmake.org). During installation, choose to add `cmake` to the system `PATH`. You'll need to reopen command prompt or log out to apply the changes. Alternatively, add its installation directory to `PATH` in your prompt. 

Run `cmake --version` to verify that `cmake` is available.

## Verifying installation

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

On macOS:
```
cd /tmp
mkdir build
cd build
qmake ~/Qt/Examples/Qt-5.13.0/widgets/dialogs/standarddialogs
make
open standarddialogs.app
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

Finally, you can try to build the provided examples:
```
cd examples
cargo run --bin form1
```

# Getting started

To use Qt from Rust, add the crates as dependencies to your `Cargo.toml`, for example:

```
[dependencies]
qt_widgets = "0.2"
```

Each crate re-exports its dependencies, so, for example, you can access `qt_core` as `qt_widgets::qt_core` without adding an explicit dependency.

You can look at the examples in this repository to see how to use the API.

# Unsafety

It's impossible to bring Rust's safety to C++ APIs automatically, so most of the generated APIs are unsafe to use and require thinking in C++ terms. Most of the generated functions are unsafe because raw pointers are not guaranteed to be valid, and most functions dereference some pointers.

It's recommended to contain unsafe usage in a module and implement a safe interface for the parts of API required for your project.

You should be careful when working with Qt objects. Qt has its own [ownership system](https://doc.qt.io/qt-5/objecttrees.html) that must be respected. If you retain a pointer to an object owned by another object, it can be deleted and you may produce undefined behavior when trying to access the deleted object.

On the other hand, C++ doesn't require mutable access to be exclusive, so it's "safe" to mutate an object while there are other mutable pointers to it. Smart pointer types provided by ritual allow you to do that conveniently.

# Smart pointers

Smart pointers are provided by the [cpp_core](https://docs.rs/cpp_core/0.5.0/cpp_core/) crate to make working with C++ objects from Rust easier:

- `CppBox`: owned, non-null (corresponds to C++ objects passed by value)
- `Ptr` and `MutPtr`: possibly owned, possibly null (correspond to C++ pointers)
- `Ref` and `MutRef`: not owned, non-null (correspond to C++ references)

Unlike Rust references, these pointers can be freely copied, producing multiple mutable pointers to the same object, which is usually necessary to do when working with C++ libraries.

These smart pointers also allow you to use casts, iterators, and operators. 

# Adapters and helpers

Most of the Qt API is translated to Rust as-is (only modified according to Rust's identifier naming convention), so you can address the Qt documentation for information on it. However, Rust crates provide some additional helpers.

Qt application objects (`QApplication`, `QGuiApplication`, `QCoreApplication`) require `argc` and `argv` to be present, and these are not available directly in Rust. Use `init` helpers to initialize the application correctly:
```
fn main() {
    QApplication::init(|app| unsafe {
        //...
    })
}
```

`qt_core` provides API for using signals and slots conveniently. You can connect built-in signals to built-in slots like this:
```
let mut timer = QTimer::new_0a();
timer.timeout().connect(app.slot_quit());
```

You can also connect signals to Rust closures (see [form example](src/bin/form1.rs):
```
let button_clicked = Slot::new(move || { ... });
button.clicked().connect(&button_clicked);
```
Compatibility of signal's and slot's arguments is checked at compile time.

`QString::from_std_str`, `QString::to_std_string`, `QByteArray::from_slice`, and `impl<'a> From<&'a QString> for String` provide conversions from Qt's types to Rust types and back.

`QFlags` generic type mimics the functionality of C++'s `QFlags` class.

`qdebug` function from `qt_core` wraps a printable (with `QDebug`) Qt object into a shim object that implements Rust's `fmt::Debug`.

# Deployment

All (or most) builds of Qt available in the official installer, Linux repositories, and brew are shared libraries or frameworks. This means that any executable built with these libraries will depend on Qt and won't run if Qt is not present on the end user's system.

It's possible to build Qt statically, so that you can build a standalone executable, but it's a more complicated process. Removing dependency on `vc-redist` dynamic library on Windows is also hard to do. It's much easier to use `macdeployqt` and `windeployqt` tools to create a directory that contains all required files. Rust-Qt crates don't support linking against static Qt builds.

Executables produced by Rust-Qt are much like normal executables produced by C++ compilers, so the deployment process doesn't differ from deploying a C++/Qt application. You can use official Qt deployment guides:

- [Windows](https://doc.qt.io/qt-5/windows-deployment.html)
- [macOS](https://doc.qt.io/qt-5/macos-deployment.html)
- [Linux](https://doc.qt.io/qt-5/linux-deployment.html)

For Windows, the basic idea is to copy your executable to a new directory and run `windeployqt` to populate it with all the files required by Qt. Note that executables produced by Visual Studio depend on Visual C++ Redistributable. `windeployqt` will copy the `vc_redist.x64.exe` installer to your destination directory, and your installer should run that to make sure the proper version of this library is available on the end user's system.

A common approach on Linux is to declare that your package depends on Qt libraries and only include your executable in the package. The system's package manager will ensure that Qt packages are installed. Refer to the documentation of the target Linux distributions for detailed instructions. 

This page doesn't cover deploying QML projects yet. For QML, you will also need to copy your QML files, pass `--qmldir` flag to `*deployqt` tool, and make sure your application can find these files at runtime. Another approach is to put the QML files in Qt resources, but there is no Rust tooling for that yet.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

You should also take into account [Qt licensing](https://www.qt.io/licensing/).
