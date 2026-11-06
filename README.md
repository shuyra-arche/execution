# execution

Shuyra 旧程序启动接口的 `no_std` 兼容库。`src/lib.rs` 重导出 Kernel Client 的
`lock`、`memory`、`room`、`unit`；生产代码的堆、TLS 和任务机制由同一个 Client 提供。

`src/boot/` 保留 `entry`、`finish`、`Report`、`Exit`、参数入口与 RV64 `_start`。
`src/allocator.rs` 将全局 allocator 转发到 Client；`build.rs` 和 `link.ld` 提供
`DEP_EXECUTION_LINKER_SCRIPT`。已有程序及 `mold::entry` 通过这些接口启动。

## 当前启动约束

`boot::bootstrap` 在 fresh Task 中调用 unsafe `initialize_tls`。其 token 没有
Drop 效果；旧主 Task 路径未接入显式 `retire_tls`。映射随 Task 终态回收，
本地 `Live` 登记可留到地址空间结束，不能假定后续 sweep 会回收登记。
旧参数接口的 `&'static [usize]` 实际借用初始 Task 栈，其他 Task 的存活不延长该栈寿命。
启动汇编调用前将 `sp` 减 8，尚不满足 RISC-V 标准的 16 字节调用对齐要求。
这些启动限制需要独立修复；Client 的 `room::reap` 异常返回时进入终态 fault loop。

## 验证入口

`tests/bridge` 链接真实 execution 与 Client，验证类型身份、Report 和旧入口链接。
RV64 构建使用 Box 强制检查 allocator 链接；成功链接不代表初始化和退出已运行。
旧 `src/memory*`、`src/unit*`、`src/lock.rs` 等文件仅供 source-import 测试使用，
不编入生产库。`tests/` 与 `tests/unit/` 验证这些历史机制，不能当作 Client 验证。

## 工具链与维护

使用 `rust-toolchain.toml` 固定的 `nightly-2026-09-04`，首次运行需要 Rustup
下载工具链及 RV64 目标。依赖版本以 Cargo 清单和锁文件为准。
下列命令从仓库根目录执行；宿主测试不代表真实内核运行验证。

```sh
cargo build --lib --target x86_64-unknown-linux-gnu --locked
cargo build --lib --target riscv64gc-unknown-none-elf --locked
cargo run --manifest-path tests/bridge/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo build --manifest-path tests/bridge/Cargo.toml --target riscv64gc-unknown-none-elf --locked
cargo test --manifest-path tests/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/unit/Cargo.toml --target x86_64-unknown-linux-gnu --locked
```

## 贡献与来源

维护与 PR 规则见 [Shuyra 贡献规范](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md)。
来源、作者与许可情况见 [PROVENANCE.md](PROVENANCE.md)。
