# 415pr1

Placeholder repository for CENG415's project. </br>
https://github.com/He-Is-HaZaRdOuS/rt-rs

# About

The aim of this project is to build a simple ray tracer from the ground up.

# How?

fn main() { trace_rays(); } /s

# Why Rust?

![Alt](memes/silence.jpg)

# Usage

### Pre-requisites (Windows)

* C/C++ Toolchain (MSVC/MinGW/MSYS2)
* SDL2

#### C/C++ Toolchain

* Preferably You should have
  the [Visual Compiler](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) (MSVC) by
  installing Visual Studio 2017 or later and its C/C++ Build tools as rustup default to
  MSVC on Windows
* If not, You can install [MinGW](https://nuwen.net/files/mingw/mingw-18.0-without-git.exe) and copy the SDL2 lib/dll
  files to MinGW's lib/ directory
* Or use [MSYS2](https://www.msys2.org/) to install MinGW and copy the SDL2 lib/dll files to the lib/ directory inside
  ucrt64

### Pre-requisites (Unix)

* CMake
* C/C++ Toolchain (gcc11/g++11)
* SDL2
* SDL2_gfx

#### CMake

* Install CMake from your distribution's package manager or...

```bash
 version=3.28
 build=1
 limit=3.20
 result=$(echo "$version >= $limit" | bc -l)
 os=$([ "$result" == 1 ] && echo "linux" || echo "Linux")
 mkdir ~/temp
 cd ~/temp
 wget https://cmake.org/files/v$version/cmake-$version.$build-$os-x86_64.sh
 sudo mkdir /opt/cmake
 sudo sh cmake-$version.$build-$os-x86_64.sh --prefix=/opt/cmake #(Type "y" to accept the license agreement and type "n" to forego installing inside the subdirectory)
 cmake --version #(expected output is "cmake version 3.28.1") 
```

#### C/C++ Toolchain

* Install/Update gcc and g++ from your distribution's package manager

#### SDL2

* Debian and its derivatives (APT)

````
sudo apt-get install libsdl2-dev
````

* RHEL and its derivatives (DNF)

````
sudo dnf install SDL2-devel
````

* RHEL and its derivatives (YUM)

````
yum install SDL2-devel
````

* Or visit https://wiki.libsdl.org/SDL2/Installation to build SDL2 from source

#### SDL2_gfx

* Download the source from [here](https://www.ferzkopp.net/Software/SDL2_gfx/SDL2_gfx-1.0.4.tar.gz) and follow the build
  instructions from [here](https://www.ferzkopp.net/Software/SDL2_gfx/Docs/html/index.html)

### #1 Install the Rust build system (Cargo)

https://doc.rust-lang.org/cargo/getting-started/installation.html

### #2 Clone this repository

```
git clone https://github.com/He-Is-HaZaRdOuS/rt-rs.git
```

or just download as a zip and extract it

### #3 Run the program

```
cd rt-rs && cargo run --release
```
