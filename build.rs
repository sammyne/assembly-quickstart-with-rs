fn main() {
    cc::Build::new().file("asm/att_style_add.S").file("asm/intel_style_add.S").compile("my-asm-lib");
}
