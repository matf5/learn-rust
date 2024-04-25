fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let mut num = String::new();

    std::io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    // parse方法转数字
    let num: u32 = num.trim().parse().expect("Please type a number!");

    for i in (0..num) {
        print!("{} ", fibonacci(i));
    }
}
