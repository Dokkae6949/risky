use std::env;

fn main() {
    let target_arch = match env::var("CARGO_CFG_TARGET_ARCH") {
        Ok(arch) => arch,
        Err(_) => panic!("Target architecture unknown"),
    };

    let linker_script = match target_arch.as_str() {
        "riscv64" => "src/arch/rv64/linker.ld",
        _ => panic!("Target architecture not supported: {}", target_arch),
    };

    // Tell cargo to rerun this build script if the linker script changes.
    println!("cargo:rerun-if-changed={}", linker_script);
    // Provide the linker script to use.
    println!("cargo:rustc-link-arg=-T{}", linker_script);
    // Some fancy magic stuff...
    println!("cargo:rustc-link-arg=--omagic");
}
