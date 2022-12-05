use std::fs;

struct Pair {
    l: u32,
    h: u32,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    let mut sum1:u32 = 0;
    let mut sum2:u32 = 0;
    for line in input.lines() {
        let pairs: Vec<&str> = line.split(',').collect();
        let pair0: Vec<&str> = pairs[0].split('-').collect();
        let pair1: Vec<&str> = pairs[1].split('-').collect();
        let x = Pair {
            l: pair0[0].parse::<u32>().unwrap(),
            h: pair0[1].parse::<u32>().unwrap(),
        };
        let y= Pair {
            l: pair1[0].parse::<u32>().unwrap(),
            h: pair1[1].parse::<u32>().unwrap(),
        };
        if fully_contains(&x, &y){
            sum1+=1;
        }
        if part_overlaps(x,y){
            sum2+=1;
        }
   }
   println!("part1 {}",sum1);
   println!("part2 {}",sum2);
}


fn part_overlaps (x: Pair, y: Pair) -> bool{
    if y.l >= x.l && y.l <= x.h{
        return true;
    }
    if y.h >= x.l && y.h <= x.h{
        return true;
    }
    return fully_contains(&x,&y);
}

fn fully_contains (x: &Pair, y: &Pair) -> bool{
    let (lower, higher) = get_lower(&x, &y);
    if higher.h <= lower.h{
        return true;
    }else{
        if  x.l==y.l{
            return lower.h <= higher.h;
        }
        return false;
    }
}
fn get_lower<'a> (x: &'a Pair, y: &'a Pair) -> (&'a Pair,&'a Pair){
    if x.l<=y.l{
        (x,y)
    }else{
        (y,x)
    }
}