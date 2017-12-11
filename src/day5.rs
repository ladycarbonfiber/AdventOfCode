use common::read_input;
use common::Part;

pub fn solve(part:Part){
    let input = read_input(5);
    let mut instructions:Vec<i32> = input.lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut index:usize = 0;
    let mut steps = 0;
    let bound = instructions.len();
    while index < bound{
        steps+=1;
        let inst = instructions[index];
        match part{
            Part::PartOne=>{
                instructions[index] = inst+1;
            },
            Part::PartTwo=>{
                if inst>= 3{
                    instructions[index] = inst-1;
                } else {
                    instructions[index] = inst +1;
                }
            }
        }
        index = (index as i32 + inst) as usize;

    }
    println!("{}", steps);
}