
//! - Box相当于C++的std::unique
//! - RC比Box可以降低生命周期标注带来的复杂性
fn main() {
    let b = Box::new(1);
    let rb = &b;
    let rb2 = &b;
    println!("{rb}");
    println!("{rb2}");
}