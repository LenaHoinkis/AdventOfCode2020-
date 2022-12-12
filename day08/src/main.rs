use std::{borrow::BorrowMut, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
}

fn part1(s: &str) -> usize {
    let result = build_map(s);
    let size = result.1;
    let map = result.0;

    let mut sum = 0;
    let mut hits: Vec<bool> = vec![false; size * size];

    for (i, _) in map.iter().enumerate() {
        if is_edge(i as u32, size as u32) {
            hits[i] = true;
        }
    }

    //move left to right
    let mut x = 1;
    let mut y = 0; // the row before the last one
    let mut current_highest = 0;
    while x < size - 1 {
        while y < size - 1 {
            if map[y + x * size] > current_highest {
                current_highest = map[y + x * size];
            }
            if map[y + 1 + x * size] > current_highest {
                hits[y + 1 + x * size] = true;
            }
            y += 1;
        }
        current_highest = 0;
        x += 1;
        y = 0;
    }

    //move right to left
    let mut x = 1;
    let mut y = size - 1; // the row before the last one
    let mut current_highest = 0;
    while x < size - 1 {
        while y > 0 {
            if map[y + x * size] > current_highest {
                current_highest = map[y + x * size];
            }
            if map[y - 1 + x * size] > current_highest {
                hits[y - 1 + x * size] = true;
            }
            y -= 1;
        }
        current_highest = 0;
        x += 1;
        y = size - 1;
    }

    //move up to down
    let mut x = 1;
    let mut y = 0;
    let mut current_highest = 0;
    while x < size - 1 {
        while !is_edge((y + x + size) as u32, size as u32) {
            if map[y + x] > current_highest {
                current_highest = map[y + x];
            }
            if map[y + x + size] > current_highest {
                hits[y + x + size] = true;
            }
            y += size;
        }
        current_highest = 0;
        x += 1;
        y = 0;
    }

    //move down to up
    let mut x = 1;
    let mut y = size * (size - 2); // the row before the last one
    let mut current_highest = 0;
    while x < size - 1 {
        while !is_edge((y + x) as u32, size as u32) {
            if map[y + x + size] > current_highest {
                current_highest = map[y + x + size];
            }
            if map[y + x] > current_highest {
                hits[y + x] = true;
            }
            y -= size;
        }
        current_highest = 0;
        x += 1;
        y = size * (size - 2);
    }

    for is_hit in hits {
        if is_hit {
            sum += 1;
        }
    }

    return sum;
    /*
    0  1  2  3  4
    5  6  7  8  9
    10 11 12 13 14
    15 16 17 18 19
    20 21 22 23 24
    */
}

fn is_edge(index: u32, size: u32) -> bool {
    if index < size
        || index % size == 0
        || index % size == size - 1
        || index > ((size * size) - size)
    {
        return true;
    }
    false
}

fn build_map(s: &str) -> (Vec<u32>, usize) {
    let mut v: Vec<u32> = Vec::new();
    let lines = s.lines().count();
    let _ = s
        .replace("\n", "")
        .chars()
        .for_each(|char| v.push((char.to_string().parse::<u32>().unwrap())));
    (v, lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let example = "30373
25512
65332
33549
35390";
        assert_eq!(part1(example), 21);

        let example = "111111
122221
126621
126622
122221
111111";
        assert_eq!(part1(example), 35);
    }
    #[test]
    fn test_is_edge() {
        let size = 4;
        let field: Vec<u32> = (0..size * size).collect();
        assert_eq!(is_edge(field[3], size), true);
        assert_eq!(is_edge(field[4], size), true);
        assert_eq!(is_edge(field[15], size), true);
        assert_eq!(is_edge(field[5], size), false);
    }
}
