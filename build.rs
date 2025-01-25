use std::env;

fn main() {
    let target_arch = match env::var("CARGO_CFG_TARGET_ARCH") {
        Ok(arch) => arch,
        Err(_) => panic!("Target architecture unknown"),
    };

    let linker_script = match target_arch.as_str() {
        "riscv64" => "src/arch/rv64/linker.ld",
        _ => panic!("Target architecture `{}` not supported", target_arch),
    };

    println!("cargo:rerun-if-changed={}", linker_script);
    println!("cargo:rustc-link-arg=-T{}", linker_script);
    println!("cargo:rustc-link-arg=--omagic");
}
