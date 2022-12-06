
use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}",part1(&input));
    println!("{}",part2(&input));
}

fn part1(s: &str) -> usize {
    calc_marker(3,s)
}

fn part2(s: &str) -> usize {
    calc_marker(13,s)
}

fn calc_marker(steps: usize, s: &str)-> usize{
    let result = s.char_indices()
    .skip(steps)
    .take_while(|x| !is_marker(&s[x.0-steps..x.0+1])).last();
    match result {
        Some(i) => return i.0+2,
        _ => unreachable!()
    }
}

fn is_marker(s : &str) -> bool{
    let mut uniq = HashSet::new();
    s.chars().into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
            assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
            assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
            assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
            assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        }
    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
    #[test]
    fn test_is_marker() {
        assert_eq!(is_marker("aaaa"), false);
        assert_eq!(is_marker("abxd"), true);
   
        let s = "abca";
        assert_eq!(is_marker(&s[0..3]), true);
    }
    }
