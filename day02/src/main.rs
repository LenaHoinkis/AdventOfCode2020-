use std::fs;

enum Shape {
    Rock,
    Paper,
    Scissors,
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    //part 1
    {
        let mut results = Vec::<(Shape,Shape)>::new();
    
        for line in input.lines() {
            let c: Vec<char> = line.chars().collect();
            results.push((get_shape(c[0]),get_shape(c[2])));
       }
    
       let calculated_points = results.into_iter().map( | (x,y) | value_of_shape(&y) + points((&x,&y)) ).collect::<Vec<_>>();
       println!("part1: {}",calculated_points.iter().sum::<i32>());
    }

    //part 2
    {
        let mut results = Vec::<i32>::new();
    
        for line in input.lines() {
            let c: Vec<char> = line.chars().collect();
            results.push(get_points_new_code(c[0],c[2]));
       }
       println!("part2: {}",results.iter().sum::<i32>());
    }

}

fn points(s: (&Shape,&Shape)) -> i32 {
    match s {
        (Shape::Rock,Shape::Rock) | (Shape::Paper,Shape::Paper) | (Shape::Scissors,Shape::Scissors) => 3,
        (Shape::Rock,Shape::Scissors) | (Shape::Paper,Shape::Rock) | (Shape::Scissors,Shape::Paper) => 0,
        _ => 6,
    }
}

fn get_points_new_code(played: char,code: char) -> i32 {
    match code {
        //lose
        'X' => 0 + value_of_shape(&get_loser(&get_shape(played))),
        //draw
        'Y' => 3 + value_of_shape(&get_shape(played)),
        //win
        'Z' => 6 + value_of_shape(&get_winner(&get_shape(played))),
        _  => 0,
      }
}

fn get_winner(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock =>  Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn get_loser(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock =>  Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

 
fn value_of_shape(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_shape(c: char) -> Shape {
    match c {
        'A'|'X' => Shape::Rock,
        'B'|'Y' => Shape::Paper,
        'C'|'Z' => Shape::Scissors,
        _ => panic!("INVALID SHAPE"),
      }
}
