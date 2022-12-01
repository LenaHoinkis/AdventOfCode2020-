use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut x = 0;
    let mut results = Vec::new();

    for line in input.lines() {
        if !line.is_empty(){
            let num: i32 = line.parse().unwrap();
            x += num
        }else{
            results.push(x);
            x = 0
        }
   }
   //part1
    let max_value = *results.iter().max().unwrap();
    println!("part1: {}",max_value);

    //part2 
    let sorted_values = Vec::from_iter(results.into_iter().sorted().rev().take(3));
    println!("part2: {}",sorted_values.iter().sum::<i32>());
}
