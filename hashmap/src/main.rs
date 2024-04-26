use std::collections::HashMap;

fn average(nums: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum as f64 / nums.len() as f64
}

fn middle(nums: &Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    nums.sort();
    nums[nums.len() / 2]
}

fn mode(nums: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut mode = 0;
    for (num, count) in map {
        if count > max {
            max = count;
            mode = *num;
        }
    }
    mode
}

fn main() {
    // generate a number vector, length is 100, number is ramdom below 10
    let mut nums = vec![2, 5, 3, 4, 6, 7, 9, 1];

    println!("The average is {}", average(&nums));
    println!("The middle is {}", middle(&nums));
    println!("The mode is {}", mode(&nums));
}
