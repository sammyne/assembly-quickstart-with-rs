extern "C" {
    fn att_style_add(a: u64, b: u64) -> u64;
    fn intel_style_add(a: u64, b: u64) -> u64;
}

fn main() {
    let sum = unsafe { att_style_add(1, 2) };
    println!("{}", sum);

    let sum2 = unsafe { intel_style_add(2, 1) };
    println!("sum2 = {}", sum2);
}
