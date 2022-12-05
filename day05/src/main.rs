use std::fs;


fn main() {
   //Read Stacks
   let input = fs::read_to_string("input.txt").unwrap();
   //(crate,order,char)
   let emptyVec = Vec::new();
   let mut stacks: Vec<Vec<_>> = vec![emptyVec; 9];  
    let mut start_of_commands = 0;

   //fill stack
   for (i,line) in input.lines().enumerate() {
        let x:Vec<(usize,char)> = line.char_indices()
            .filter(|c| c.1.is_ascii_alphabetic())
            .map(|c| ((c.0 + 2 )/4 , c.1))
            .collect();
            
        if x.is_empty(){
            start_of_commands = i+1;
            break
        }       
        for c in x {
            stacks[c.0].push(c.1)
        }
    }


    stacks.iter_mut().for_each(|x| x.reverse());


    for (i,line) in input.lines().enumerate() {
        if (i>=start_of_commands+1){
            // amount, old, new
            // has bug for 2 digets
            let command:Vec<usize> = line.split_ascii_whitespace()
            .map(|c| c.to_string().parse::<usize>())
            .filter(|c|   !c.is_err())
            .map(|c| c.unwrap())
            .collect();

            println!("({:?})",command);
            for _ in 0..command[0] {
                let popped = stacks[command[1]-1].pop();
                match popped {
                    Some(i) => stacks[command[2]-1].push(i),
                    _ => unreachable!()
                }  
            }

        }
    }
    println!("({:?})",stacks);
}