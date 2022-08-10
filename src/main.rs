use std::arch::global_asm;

global_asm!(include_str!("../asm/dummy_add.S"));

extern "C" {
    fn att_style_add(a: u64, b: u64) -> u64;
    // still needs explicit declarations
    fn dummy_add(a: u64, b: u64) -> u64;
    fn intel_style_add(a: u64, b: u64) -> u64;
}


fn main() {
    let sum = unsafe { att_style_add(1, 2) };
    println!(" sum = {sum}");

    let sum2 = unsafe { intel_style_add(2, 1) };
    println!("sum2 = {}", sum2);

    let sum3 = unsafe { dummy_add(1, 2) };
    println!("sum3 = {sum3}");
}
