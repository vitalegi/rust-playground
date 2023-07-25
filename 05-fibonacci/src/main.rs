use std::io;
use std::time::Instant;

// 0 1 1 2 3 4 5 6
fn main() {
    println!("Fibonacci");
    println!("What number? ");
    let n = read_number();
    println!("N: {n}");

    let time = now();
    let res = fibonacci_iterative(n);
    let elapsed = time.elapsed();
    let ns = elapsed.as_nanos();
    println!("Fibonacci of {n} = {res} ({ns})");

    let time = now();
    let res = fibonacci_recursive(n);
    let elapsed = time.elapsed();
    let ns = elapsed.as_nanos();
    println!("Fibonacci of {n} = {res} ({ns})");
}

fn now() -> Instant {
    Instant::now()
}

fn fibonacci_iterative(n: u64) -> u64 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        let mut v0 = 0;
        let mut v1 = 1;
        for _i in 2..n {
            let swap = v0 + v1;
            v0 = v1;
            v1 = swap;
        }
        v1
    }
}

fn fibonacci_recursive(n: u64) -> u64 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn read_number() -> u64 {
    let str = read_string();

    match str.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

fn read_string() -> String {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Error while reading the line");
    str
}
