use std::fs;
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("input.txt");
    let mut x = 0;
    let mut results = Vec::new();

    let binding = contents.expect("REASON");
    for line in binding.lines() {
        if !line.is_empty(){
            let num: i32 = line.parse().unwrap();
            x += num
        }else{
            results.push(x);
            x = 0
        }
   }
    let max_value = *results.iter().max().unwrap();
    println!("{}",max_value);

    let sorted_values = Vec::from_iter(results.into_iter().sorted().rev().take(3));
    println!("{}",sorted_values.iter().sum::<i32>());
}
