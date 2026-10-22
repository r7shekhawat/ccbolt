# Roadmap

## v0.1 (in progress)
- [x] `wrap` / `stats` / `clean` commands
- [ ] Stats file: hits, misses, time/bytes saved
- [ ] Integration test: two builds of a fixture project, second all hits
- [ ] Only hash codegen-relevant flags (ignore -o, -MF, etc.)

## v0.2 (planned)
- [ ] Direct mode: hash raw source + `#include` deps, skip preprocessing
- [ ] zstd compression of cached objects
- [ ] Cache size cap with LRU eviction; `clean --older-than 30d`
- [ ] clang / clang-cl / MSVC support

## v0.3 (planned)
- [ ] TUI dashboard (ratatui): live hit rate, cache size over time
- [ ] Config file (ccbolt.toml)
- [ ] Shell completions, man page, CI, prebuilt binaries

## Benchmarks (planned)
Cold vs warm clean build of sqlite and zlib, published here.
