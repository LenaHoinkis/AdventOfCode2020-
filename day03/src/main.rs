use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    //part 1
    {
        let mut result = Vec::<u32>::new();
        for line in input.lines() {
            let compartments = line.split_at(line.len()/2);
    
            for c in compartments.1.chars() {
                if compartments.0.contains(c){
                   result.push(char_priority(c));
                    break;
                }
            }
       }
       println!("part1: {}",result.iter().sum::<u32>());
    }

    //part 2
    {
        let mut result =  vec![0; 54];
        let mut sum = 0;
        for (i, line) in input.lines().enumerate() {
            for c in line.chars() {
                if result[char_priority(c) as usize] == i%3{
                    result[char_priority(c) as usize] +=1
                }
            }
            println!("test: {:?}",result);
            if i%3 == 2{
                let max = result.iter().max().unwrap();
                let index = result.iter().position(|z| z == max);
                println!("index {} size{} i{}",index.unwrap(),max,i);
                result.clear();
                result.resize(54, 0);
                sum+=index.unwrap();
            }
            
       }
       println!("part2: {}",sum);
    }
}


fn char_priority (c:char) -> u32{
    if c.is_ascii_uppercase(){
       return c as u32 - 65 + 27;
    }
    c as u32 - 96
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calc_uppercase() {
            assert_eq!(char_priority('P'), 42);
            assert_eq!(char_priority('L'), 38);
        }
    #[test]
    fn calc_lowercase() {
            assert_eq!(char_priority('p'), 16);
            assert_eq!(char_priority('t'), 20);
        }
    }