fn main() {
    println!("Hello, world!");
    let mut index = 1;

    while index < 16 {
        let value: String = foobar(index);
        println!("{index}: {value}");
        index += 1;
    }
    
    println!("");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        let value: String = foobar(element);
        println!("{element}: {value}");
    }

    println!("");
    
    for number in 10..16 {
        let value: String = foobar(number);
        println!("{number}: {value}");
    }
}

fn foobar(v1: u32) -> String {
    let mut fb = String::from("");
    let foo: String = String::from("foo");
    let bar: String = String::from("bar");
    if v1 % 3 == 0 {
        fb.push_str(&foo);
    }
    if v1 % 5 == 0 {
        fb.push_str(&bar);
    }
    fb
}
