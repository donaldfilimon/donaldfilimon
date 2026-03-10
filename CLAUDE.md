# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a monorepo home directory containing two major projects sharing architectural DNA (vector databases, AI agents, distributed systems):

| Project | Language | Purpose |
|---------|----------|---------|
| `abi/` | Zig 0.16 | Framework for AI services, vector search (WDBX), GPU compute (5 backends), 40+ CLI commands |
| `lilex/` | Rust 2024 | Autonomous desktop agent with Tauri v2/Yew WASM UI, distributed WDBX vector DB, neural training |

Each project has its own `CLAUDE.md`, `AGENTS.md`, and task tracking (`tasks/todo.md`, `tasks/lessons.md`). **Always read the project-specific CLAUDE.md before working in that directory.**

## Toolchain Requirements

| Tool | Version | Source | Notes |
|------|---------|--------|-------|
| Zig | `0.16.0-dev.1503+738d2be9d` | `abi/.zigversion` | Dev build pinned; do not upgrade without testing |
| Rust | stable (edition 2024) | `lilex/rust-toolchain.toml` | Components: rustfmt, clippy |
| trunk | latest | `cargo install trunk` | Required for Tauri/WASM desktop builds |
| cargo-watch | latest | `cargo install cargo-watch` | Optional; useful for dev loop |
| wasm-bindgen-cli | match Cargo.lock | `cargo install wasm-bindgen-cli` | Must match the version in lilex's lockfile |
| Tauri CLI | v2 | `cargo install tauri-cli@2` | Only needed for desktop feature builds |
| Node/npm/bun | any recent | Homebrew | Used by Tauri bundler |

### Homebrew packages (macOS)

```bash
brew install cmake make llvm lld git gh node
```

LLVM/LLD are needed for Zig's linker fallback on macOS. cmake is used by native dependency builds.

## Environment Setup

### From scratch (both projects)

```bash
# 1. Install Homebrew packages
brew install cmake make llvm lld git gh node

# 2. Rust toolchain (rustup will read lilex/rust-toolchain.toml automatically)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown   # for Tauri/Yew WASM

# 3. Cargo tools for lilex desktop builds
cargo install trunk tauri-cli@2 wasm-bindgen-cli

# 4. Zig — install the exact pinned version
# Use zigup, zvm, or download from ziglang.org/builds/
# Verify: zig version == contents of abi/.zigversion

# 5. Verify both projects build
cd ~/abi  && zig build test --summary all
cd ~/lilex && cargo test --no-default-features
```

### Key environment variables

| Variable | Purpose |
|----------|---------|
| `ZIG_LIB_DIR` | Override if Zig can't find its stdlib |
| `CC`, `CXX` | Set to Homebrew clang if system clang causes issues |
| `RUSTFLAGS` | Use `-C target-cpu=native` for optimized local builds |
| `TAURI_SIGNING_PRIVATE_KEY` | Required only for signed desktop releases |

## Quick Reference

### ABI (Zig)

```bash
cd ~/abi
zig build test --summary all          # 1290 tests
zig build feature-tests --summary all # 2836 tests
zig build full-check                  # format + tests + flags + CLI smoke
zig build verify-all                  # release gate
zig build lint                        # check formatting
zig build fix                         # auto-format
```

Key pattern: every feature in `src/features/<name>/` has `mod.zig` + `stub.zig` — both must stay in sync.

### Lilex (Rust)

```bash
cd ~/lilex
cargo build --no-default-features              # core only (no Tauri)
cargo test --no-default-features               # 650+ tests
cargo fmt --all                                # format
cargo clippy --all-targets --no-default-features  # lint (never --all-features)
cargo run -- agent                             # headless agent
cargo run -- agent --tui                       # TUI dashboard
```

Key pattern: `desktop` is the default feature (requires trunk + Tauri). Use `--no-default-features` for CI/headless builds.

## Cross-Project Patterns

### WDBX (Vector Database)

Both projects implement WDBX vector databases with HNSW indexing (`abi/src/wdbx/`, `lilex/src/wdbx/`). They are independent implementations (Zig vs Rust) sharing the same design:
- **Changing the indexing algorithm or data format?** Update both implementations and verify with tests in each project.
- **ABI tests**: `zig build test --summary all` covers WDBX unit tests.
- **Lilex tests**: `cargo test --no-default-features -p lilex wdbx` (or full test suite).
- The on-disk format is not yet guaranteed compatible across the two implementations.

### AI Personas (Abbey/Aviva/ABI)

Both projects use the persona triad for agent cognition. Persona definitions live in each project's source; if you change the cognitive architecture or prompt patterns, audit both.

### Testing Strategy

When working on shared concepts:
1. Run the full test suite in the project you changed first.
2. If the change touches WDBX or persona logic, run the other project's tests too.
3. Use the verification gates before marking work done:
   - ABI: `zig build full-check`
   - Lilex: `cargo test --no-default-features && cargo clippy --all-targets --no-default-features`

### Task Workflow

- Both use `tasks/todo.md` for plans and `tasks/lessons.md` for correction logs.
- Review `tasks/lessons.md` at the start of every session.
- **Conventional Commits** required in both projects.
- **No `rm`**: Use `TRASH/` directory for deletions in lilex. In abi, prefer safe alternatives.

## Environment Notes

- **macOS (Darwin 25+)**: ABI has a known Zig linker issue — use CEL toolchain (`./tools/scripts/cel_migrate.sh`) or fallback builds.
- **Package managers**: Homebrew (`/opt/homebrew`), Cargo, npm/bun available.

## Workflow Contract

Both projects share the workflow defined in their respective `AGENTS.md`:
1. Plan first (write to `tasks/todo.md`)
2. Verify before marking done — run tests, prove correctness
3. Update `tasks/lessons.md` after any correction
4. Never mark complete without passing `full-check` (abi) or `cargo test` (lilex)
