fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // no avalilable
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {}", first);
}
