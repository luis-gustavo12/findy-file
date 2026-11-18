# AGENTS.md

This file guides AI agents (Claude, Codex, Copilot, etc.) working in this repository. Read it in full before making any change.

## About the project

**findy-file** is a personal alternative to the Windows File Explorer, written in **Rust** with a **Slint** UI.

- **Motivation:** the Windows 11 File Explorer has flaws and limitations that bother the author in daily use — that frustration is what started the project. The goal is to build a tool of their own that fixes those issues.
- **Audience:** primarily the author's personal use. It is not a commercial product — prioritize what solves the author's real problems over generality for every case.
- **Platforms:** **cross-platform** — Windows, Linux and macOS. Windows 11 is where the motivation came from and the first target in use, but no architectural decision may tie the project to it. All OS-specific code is isolated in the platform layer (see Architecture).

## How this project is developed

The author uses AI to write the code, with the goal of **saving time**. The author sets the direction and **reviews all generated code**; the AI implements.

This means:

- **Readable, reviewable code.** Everything will be reviewed by a human. Prefer straightforward solutions over clever abstractions.
- **Small, focused changes.** One feature or fix at a time, with a diff that is easy to review.
- **No unsolicited refactoring.** Only change what the request needs. Don't rename, move, reorganize, reformat or "clean up" code outside that scope, even if it looks like an improvement — every extra line in the diff is one more line for the human reviewer. If you spot something worth refactoring, mention it in your reply and let the author decide; when a refactor is requested, do it in its own change, never mixed with a feature or fix.
- **Explain non-obvious decisions** in your reply (not with excessive comments in the code).
- **Ask when the decision belongs to the author** (library choice, UX behavior, relevant trade-offs). For implementation details with a clear standard, decide and move on.
- **Don't invent requirements.** If something isn't described here or in the request, don't add features on your own.
- Talk to the author in **Brazilian Portuguese (pt-BR)** in chat. Everything that goes into the repository follows the language rule below.

## Language

**Everything in the repository is in English**: code, identifiers, comments, doc comments, UI text, error and log messages, `Cargo.toml` descriptions, documentation and commit messages. No Portuguese in the project files — the goal is to make the project accessible to more people.

Internationalization (i18n) is planned for later. Until then, UI text is written directly in English; don't add an i18n system without the author asking for it.

## Explorer problems we want to solve

> To be filled in by the author. List the concrete flaws and limitations that motivate the project here — this list drives priorities.

- [ ] _(e.g., slow to open folders with many files)_
- [ ] _(e.g., slow / inaccurate search)_
- [ ] _(e.g., ...)_

## Tech stack

- **Language:** Rust (2024 edition), stable toolchain. The minimum version is set by `rust-version` in the root `Cargo.toml` (currently `1.94`); Cargo's resolver only picks dependencies compatible with it.
- **Build:** Cargo (workspace).
- **UI:** [Slint](https://slint.dev). `.slint` files live in `crates/app/ui/` and are compiled by `slint-build` in `build.rs`.
- **Native APIs:** used **only** inside the `platform` crate, each behind its own OS `cfg` (e.g., the `windows` crate on Windows).

When adding a new dependency, briefly justify why it is needed, and prefer mature, maintained crates that — outside the `platform` crate — are cross-platform.

## Architecture

Three layers, with dependencies pointing one way: `app` → `core` ← `platform`.

```
findy-file/
├── Cargo.toml              # workspace
├── crates/
│   ├── core/               # findy-core package: domain and the platform traits (ports)
│   ├── platform/           # findy-platform package: one adapter per operating system
│   │   └── src/
│   │       ├── lib.rs      # selects the OS adapter (at compile time)
│   │       ├── windows/
│   │       ├── linux/
│   │       └── macos/
│   └── app/                # findy-app package (findy-file binary): Slint UI
│       ├── ui/             # .slint files
│       └── src/
└── AGENTS.md
```

The packages use the `findy-` prefix because `core` is the name of one of Rust's built-in libraries; in code they appear as `findy_core`, `findy_platform`.

### Rule: the operating system is selected at compile time

The OS adapter is selected **at compile time, never at runtime**. Each binary contains only the adapter for the system it was compiled for.

- Selection is done with `#[cfg(target_os = "...")]` in `crates/platform/src/lib.rs`, which exports the `CurrentPlatform` type and the `current()` function.
- **Forbidden:** detecting the OS at runtime (`std::env::consts::OS`, `cfg!(...)` inside an `if`, etc.) to choose behavior, and using `dyn Platform` / `Box<dyn Platform>` to swap adapters.
- Code that uses the adapter receives it through **generics** (`fn x<P: Platform>(p: &P)`), with static dispatch.
- Compiling for an unsupported system must fail with `compile_error!`, not at runtime.

### `core` — the domain
- Holds the models (directory entry, metadata, etc.), the rules (sorting, filters, search) and the **traits describing what the operating system must provide** (e.g., list a directory, move to trash, open with the default app, get an icon).
- **Does not depend** on Slint or on any OS-specific API. No `#[cfg(target_os = ...)]` here.
- Testable in isolation, with fake implementations of the traits.

### `platform` — the adapters
- Implements the `core` traits for each system, each in its own module, compiled with `#[cfg(target_os = "windows" | "linux" | "macos")]`.
- `lib.rs` exposes a single entry point (`findy_platform::current()`) that returns the adapter for the system the program was compiled for. **OS selection happens only here**, at compile time (see the rule above).
- When a feature doesn't exist on a system, the adapter returns an explicit "unsupported" error — never `panic!` or silent behavior.
- OS `cfg` attributes **may only appear in this crate**.

### `app` — the UI
- Slint UI, screen state, and wiring between UI events and `core`.
- All UI colors come from the `Theme` global in `crates/app/ui/theme.slint` ("slate and blue" palette, light and dark following the OS setting). Never hardcode colors in components; add a token to `Theme` instead.
- Receives the adapter from `platform` at startup and passes it to `core`; never calls OS APIs directly.
- Disk operations (listing large folders, copying, searching) **must not block the UI** — they run off the UI thread and hand results back to Slint (e.g., `slint::invoke_from_event_loop`).

> The details of each layer (which traits exist, how each adapter is implemented) will be defined incrementally in future tasks. Update this section when the structure changes.

## Quality principles

- **Performance is a requirement**, not a detail: opening folders and searching files must be fast even with thousands of items.
- **User data safety:** destructive operations (delete, move, overwrite) should prefer the system trash and ask for confirmation when appropriate. Never permanently delete files by default.
- **Error handling:** no `unwrap()`/`expect()` on paths that depend on I/O or user input. Use `Result` and show understandable errors.
- **Cross-system differences:** use `Path`/`PathBuf` (never concatenate paths as strings), handle Unicode names and non-UTF-8 paths (`OsStr`), long paths on Windows, permission denied, and case-sensitivity differences between file systems.
- **No `unsafe`** outside native API calls in the `platform` crate, and even then encapsulated and commented.

## Commands

```bash
cargo build                                            # build
cargo run -p findy-app                                 # run the app
cargo test --workspace                                 # run tests
cargo fmt --all                                        # format
cargo clippy --workspace --all-targets -- -D warnings  # lint
```

To check that the `platform` crate compiles for another system (without running it), install the target and use `cargo check`:

```bash
rustup target add x86_64-unknown-linux-gnu
cargo check -p findy-platform --target x86_64-unknown-linux-gnu
```

Before considering a task done: the code compiles, `cargo fmt` has been applied, `cargo clippy` has no warnings and the tests pass. If the change touches the `platform` crate, state which systems it was verified on and which were not.

## Conventions

- Standard `rustfmt` formatting; `clippy` lints treated as errors.
- Unit tests next to the code (`#[cfg(test)]`); tests that touch the disk use temporary directories (e.g., the `tempfile` crate), never the user's real folders.
- Small, descriptive commits. Only commit/push when the author asks.

## Open decisions

- [x] GUI framework — **Slint**.
- [x] Platforms — **cross-platform**, with per-OS adapters selected at compile time.
- [x] Workspace structure (`findy-core`, `findy-platform`, `findy-app`).
- [x] Project language — **English**; i18n later.
- [ ] Prioritized list of Explorer problems to solve (section above).
- [ ] Scope of the first version (MVP).
- [ ] Initial `core` traits (the contract each platform adapter implements).
