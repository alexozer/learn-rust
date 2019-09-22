fn main() {
    let n = 43;
    // println!("fib({}) = {}", n, fib(n));
    for i in 1..10 {
        println!("{}", i);
    }
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
