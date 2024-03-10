
//! struct不会自动被derive Clone,Copy和Debug,因为struct是自定义类型,rust不对自定义数据类型应用.
//! tuple和array在其成员都是primitive类型时会自动derive Clone,Copy和Debug, 因为tuple和array是primitive类型.
fn main() {
    let a = ["1", "2"];
    let b = a;

    println!("{:?}", b);
    println!("{:?}", a);
    
}