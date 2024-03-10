
//! rust的闭包非常灵活和强大, 可以深刻理解上下文, 比如:
//! 1. 闭包捕获变量时,永远不会发生复制,即使是实现了Copy的类型. (和C++的闭包值捕获的意义完全不同,C++捕获的值会发生复制)
//! 2. 移动捕获时根据实际Context情况不一定非要用FnOnce接收.

fn main() {

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc()
}

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main2() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);
}

/// 移动捕获, 但却使用了Fn接收.
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}