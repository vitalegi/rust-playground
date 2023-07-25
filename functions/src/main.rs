fn main() {
    println!("Hello, world!");
    print1();
    print2(22);
    print3(String::from("Foo"));
    print_sum(1, 2);
}

fn print1() {
    println!("Hello!");
}

fn print2(x: u32) {
    println!("Hello! {x}");
}

fn print3(str: String) {
    println!("Hello, {str}!");
}

fn print_sum(v1: u32, v2: u32) {
    let sum = sum(v1, v2);
    println!("{v1} + {v2} = {sum}");
}

fn sum(v1: u32, v2: u32) -> u32 {
    v1 + v2
}
