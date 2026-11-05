# execution

Shuyra 旧程序启动接口的兼容库，`no_std`。任务、锁、内存来源、映射、堆和 TLS
由同一个 Kernel Client 提供：

```rust
pub use kernel_client::{lock, memory, room, unit};
```

这些是模块重导出，`execution::memory::Reference`、`execution::unit::task::Join`
等就是 Client 的原类型。生产库不再编译本仓旧 `src/lock.rs`、`src/memory*`、
`src/room.rs` 或 `src/unit*`，不会注册第二份堆、TLS 或 detached-task 表。
`kernel-client` 和它依赖的 API 由同一个 Kernel Git revision 固定。

本库暂时保留以下旧启动接入，供已有程序和 `mold::entry` 继续使用：

- `boot::{entry, finish, Report, Exit}`、`boot::args` 与 RV64 `_start`；
- RV64 panic handler，以及向 Client 唯一堆转发的全局 allocator 注册；
- `links = "execution"`、`DEP_EXECUTION_LINKER_SCRIPT` 元数据与原 `link.ld`。

`boot::bootstrap` 在 fresh Task 中调用 Client 的 unsafe `initialize_tls`。
返回的 token 没有 Drop 效果；旧主 Task 路径保留安装，没有接入显式 `retire_tls`。
Caller 映射随 Kernel 的 Task 终态回收，本地 `Live` 登记可留到整个地址空间结束；
不保证后续 sweep 会回收该登记。丢弃 token 本身不释放映射或清空登记。
新 Runtime 的退出范围、参数借用生命周期和 TLS 退休接入仍待单独对齐。
旧参数接口返回的 `&'static [usize]` 仍借初始 Task 的栈，其他 Task 存活不会延长
那段栈的生命周期。启动汇编仍在调用前将 `sp` 减 8，破坏 RISC-V 标准要求的
16 字节调用对齐；本次未修这两项旧行为。主 Task 返回和 panic 的退出路径也继续保留。
Client 的 `room::reap` 若异常返回，会进入终态 fault loop，
旧实现则会 panic；正常终止的 reason 和 note 参数相同。

## 构建与验证

工具链文件保留原 `nightly-2026-09-04`。本次检查使用环境已安装的
`nightly-2026-10-09`，见 [VALIDATION.md](VALIDATION.md)。

```sh
cargo build --lib --target x86_64-unknown-linux-gnu --locked
cargo build --lib --target riscv64gc-unknown-none-elf --locked
cargo run --manifest-path tests/bridge/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked
cargo build --manifest-path tests/bridge/Cargo.toml \
  --target riscv64gc-unknown-none-elf --locked
```

`tests/bridge` 链接真正的 execution 和 Client，检查公开类型身份与 `Report`，
并在 RV64 下用旧 `mold-entry` 包的 `#[entry]` 宏生成 `clean_ret`，再通过真实
`DEP_EXECUTION_LINKER_SCRIPT` 链接完整 ELF。宏仍以消费者惯用的 `mold` 名字引用。
这个最小程序强制使用 `Box`，检查 allocator 注册能随旧启动路径一起链接。
RV64 构建和符号检查不代表真实内核上已经执行过初始化、分配或终止。

## 历史夹具

原源码仍留在本仓，专供原有 source-import harness 使用；它们不属于生产库。
本次未把历史机制测试迁移到 Client，也没有删断言或把它们称为 Client 的机制验证：

```sh
cargo test --manifest-path tests/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/unit/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked
```

这两组分别有 14 个（5 个 allocator、1 个启动参数、8 个机制）和 1 个任务生命周期
测试，各自保留旧依赖和锁。`tests/` 直接导入旧 `src/memory` 文件，
`tests/unit/` 直接导入旧 `src/unit/task.rs` 与 `src/lock.rs`。
它们只说明历史夹具行为没有因本次切换改变。真实 Client 机制测试归 Kernel 仓负责。
根目录 `cargo test --target x86_64-unknown-linux-gnu --locked` 也保留自动发现的
14 个历史测试入口；它的 dev-dependencies 使用与 Client 同 revision 的纯 API，
不把旧 ABI 带回生产依赖。这仍是在检验历史源码，不是 Client 实现。

## 来源、作者与许可

旧实现从公开冻结基线 `66bb8b5c342c50b141eb935a85efdfcecb91451b`
（`codex/shuyra-execution-extraction`）的 `crates/execution` 提取。
[MIGRATION-MAP.md](MIGRATION-MAP.md) 和 [PROVENANCE.md](PROVENANCE.md)
记录的是该次历史提取；其逐字节和哈希说明不覆盖当前适配修改。

来源作者为 `anreonyr <anreonyr@gmail.com>`，历史另有
`Serial Device <anreonyr@gmail.com>`。来源树没有许可证声明；本仓不添加或授予许可证。
