# Client adapter validation

Checked on 2026-11-05 from execution production base
`1b333a28bc4c34beaea3f7986a9bb6e8ab2db414`.
The commands used installed `nightly-2026-10-09` cargo/rustc/rustdoc from
`/workspace/.sqware-tools/rustup/toolchains/`, with
`CARGO_HOME=/workspace/.sqware-tools/cargo`. The repository's older toolchain
pin was not changed or installed for this check.

## Canonical dependency and bridge

The root and `tests/bridge` locks resolve `kernel-client`, `kernel-api-env`,
`kernel-api-wire` and `kernel-api-macros` to the same published Kernel commit
`90d43dbf3ee43fd146f487718eb39e88f2507c05`. No local Cargo patch is used.
The bridge fixture selects the unchanged `mold-entry` package at published
`8e8b1c2bdf188cf01494a3a93398cc77cd12b3e6` under the alias `mold`.
It exercises the old entry macro, not the separate `mold` facade's other macros.

The initial full checks below used
`a9a1d8d788213597adb69605d1787bdef65d2b64`; the Client source changes in the final
revision only clarify its legacy TLS contract. The final exact-Git checks are
recorded below.

| Check | Result |
| --- | --- |
| `cargo build --lib --target x86_64-unknown-linux-gnu --locked` | Passed |
| `cargo build --lib --target riscv64gc-unknown-none-elf --locked` | Passed |
| `cargo run --manifest-path tests/bridge/Cargo.toml --target x86_64-unknown-linux-gnu --locked` | Passed; type identities and report assertions execute, `Box` uses host allocator |
| `cargo build --manifest-path tests/bridge/Cargo.toml --target riscv64gc-unknown-none-elf --locked --offline` | Passed; complete legacy-entry debug ELF |
| Same RV64 bridge build with `--release` | Passed in independent review |
| Bridge `cargo tree --target riscv64gc-unknown-none-elf --locked --offline --duplicates` | Empty in independent review |
| Root `cargo tree --target all --locked --offline --duplicates` | Empty, including dev-dependencies |
| `git diff --check` | Passed |

Independent review also repeated the root host check, RV64 library build and
bridge host execution in its own target directories.

The bridge's build script reads the real `DEP_EXECUTION_LINKER_SCRIPT` emitted
by execution. RV64 debug disassembly retains the old order: `save_args`,
`tls_bootstrap`, `sp -= 8`, `clean_ret`, then the terminal loop. The ELF defines
one each of `_start`, `clean_ret`, `save_args` and `tls_bootstrap`.
Debug symbols contain one canonical `kernel_client::memory::allocator::HEAP`,
one `REGIONS`, and one `kernel_client::unit::tls::BLOCKS`; they contain no
`execution::memory` or `execution::unit` mechanism definitions.
Independent review found no unresolved symbols in either debug or release ELF,
and confirmed that execution's allocator registration handle has size zero;
the release canonical heap has size `0x38`.

Host compilation still reports pre-existing unused panic-buffer items, because
the actual panic handler is RV64-only. These warnings do not affect linking.

## Final published revision

Root production Client, root test-only pure API, and the bridge's Client pin all
use `90d43dbf3ee43fd146f487718eb39e88f2507c05`. Updating the two locks replaced
only the four canonical packages' Git revision; no registry version changed.
The final checks use the published Git source, without a Kernel path override.

The legacy bootstrap does not retain a TLS token for explicit retirement.
Kernel terminal cleanup handles the Caller mapping; the Client's local `Live`
record can remain until the entire address space ends. Later sweep is not
guaranteed to retire that record. Dropping the token performs no cleanup.

| Final-pin check | Result |
| --- | --- |
| `cargo run --manifest-path tests/bridge/Cargo.toml --target x86_64-unknown-linux-gnu --locked --offline` | Passed by worker and independent reviewer |
| `cargo build --manifest-path tests/bridge/Cargo.toml --target riscv64gc-unknown-none-elf --release --locked --offline` | Passed by worker and independent reviewer |
| Root `cargo test --target x86_64-unknown-linux-gnu --locked --offline` | Independent review: 14 passed, 0 failed |
| Root and bridge `cargo tree ... --duplicates --locked --offline` | Empty |
| RV64 release `llvm-nm --undefined-only` | Empty |

The root dev-dependencies and final TLS contract were also reviewed by the
original PR reviewer; the final increment passed review. Its release ELF retains
the zero-size registration handle, one Client heap of size `0x38`, and exactly
one of each of the four startup symbols. The final revision was not used to rerun the unchanged
historical owner workspaces or the earlier debug-symbol inspection.

## Historical regression fixtures

The existing owner harnesses and all assertions are unchanged:

| Command | Result |
| --- | --- |
| `cargo test --manifest-path tests/Cargo.toml --target x86_64-unknown-linux-gnu --locked` | 14 passed, 0 failed |
| `cargo test --manifest-path tests/unit/Cargo.toml --target x86_64-unknown-linux-gnu --locked` | 1 passed, 0 failed |
| Root `cargo test --target x86_64-unknown-linux-gnu --locked --offline` | 14 passed, 0 failed |

These harnesses source-import the archived execution mechanism files and keep
their previous env dependency and locks. They are historical regression checks,
not validation of canonical Client's heap, TLS or detached-task cleanup.
The root integration entry is also retained: its dev-dependencies name the
canonical pure API at the Client revision and the same Talc/spinning_top versions.
It does not reintroduce a legacy ABI or mechanism into production dependencies.

## Limits

No QEMU or real Kernel execution was performed. RV64 linkage does not exercise
actual environment calls, allocation, TLS installation, cleanup or terminal
handoff. Kernel owns the canonical mechanism tests.

This adapter preserves the old global startup-argument borrow from the initial
task's stack and the old `_start` stack adjustment that breaks standard RV64
16-byte call alignment. It also preserves the initial task's existing exit path:
bootstrap's TLS token is not explicitly retired, and dropping it performs no
cleanup. Its local `Live` record may remain until the address space ends; a later
sweep is not guaranteed. These are recorded legacy behaviors pending Runtime design, not newly
validated lifecycle guarantees. See [README.md](README.md).
