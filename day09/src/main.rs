use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
}

fn part1(s: &str) -> usize {
    let mut visited = HashSet::new();
    let mut tail = (0, 0);
    let mut head = (0, 0);
    visited.insert(tail);

    for l in s.lines() {
        let mut iter = l.split_whitespace();
        let command = iter.next().unwrap();
        let moves = iter.next().unwrap().parse::<u32>().unwrap();
        let mut i = 0;
        for _ in 1..=moves {
            if one_step(&mut tail, &mut head, command) {
                visited.insert(tail);
            }
        }
    }
    return visited.len();
}

fn one_step(t: &mut (i32, i32), h: &mut (i32, i32), dir: &str) -> bool {
    move_one_step(h, dir);
    if (t.0 - h.0).abs() == 2 {
        move_one_step(t, dir);
        t.1 = h.1;
        return true;
    }
    if (t.1 - h.1).abs() == 2 {
        move_one_step(t, dir);
        t.0 = h.0;
        return true;
    }
    return false;
}

fn move_one_step(x: &mut (i32, i32), dir: &str) {
    match dir {
        "R" => x.0 += 1,
        "L" => x.0 -= 1,
        "U" => x.1 += 1,
        "D" => x.1 -= 1,
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let example = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part1(example), 13);
    }
}
