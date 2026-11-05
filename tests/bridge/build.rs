fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("riscv64") {
        let script = std::env::var("DEP_EXECUTION_LINKER_SCRIPT")
            .expect("execution publishes its legacy linker script");
        println!("cargo::rustc-link-arg=-T{script}");
    }
}
