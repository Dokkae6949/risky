fn main() {
    // Provide the linker script to use.
    println!("cargo:rustc-link-arg=-Tsrc/linker.ld");
    // Some funcy magic stuff...
    println!("cargo:rustc-link-arg=--omagic");
}
