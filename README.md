# ccbolt

A fast compiler cache for C/C++ builds, written in Rust. Wrap your compiler
invocations with `ccbolt` and repeated compilations of unchanged code are
served from a content-addressed cache — clean rebuilds go from minutes to
seconds.

Built because I got tired of waiting on clean builds of my C++ projects.

## Status
Early development. See [ROADMAP.md](ROADMAP.md) for planned work.

## Install
```sh
cargo install --git https://github.com/r7shekhawat/ccbolt
```

## Usage
```sh
# Wrap a single compilation
ccbolt wrap gcc -c hello.c -o hello.o

# Via environment variables (works with Make, CMake, etc.)
export CC="ccbolt wrap gcc"
export CXX="ccbolt wrap g++"

# Inspect and manage the cache
ccbolt stats
ccbolt clean
```

## How it works
1. Preprocess the source (stable hash input)
2. Hash preprocessor output + compiler + flags → cache key
3. Hit → copy cached object out. Miss → compile, then store.

## License
MIT
