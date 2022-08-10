fn main() {
    cc::Build::new().file("asm/add.S").compile("my-asm-lib");
}
