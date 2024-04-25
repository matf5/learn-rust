use std::cmp::Ordering;

use rand::Rng;

fn main() {
    // guess number
    println!("Guess the number!");

    println!("Please input your guess.");

    let apples = 5; // 不可变
    let mut bananas = 5; // 可变

    let a = [3; 5];
    a[6]; // panic

    // 区间表达式 1..100
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    // & 表示这个参数是一个引用（reference) 引用默认不可变, &muts就可变

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // parse方法转数字
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // {}是占位符, 占位符可以支持多个
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    // ramdom
}
