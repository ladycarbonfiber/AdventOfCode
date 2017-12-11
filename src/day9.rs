use common::Part;
use common::read_input;

fn solve(part:Part){
    let input = read_input(9);
    let mut sum = 0;
    let mut garbage_count = 0;
    let mut group_level = 0;
    let mut is_garbage = false;
    let mut is_canceled = false;
    let stream = input.chars();
    for character in stream{
        if is_canceled{
            is_canceled = false;
            continue;
        }
        if is_garbage{
            match character{
                '!' =>{
                    is_canceled = true;
                },
                '>' => {
                    is_garbage = false;
                },
                _ => {
                    garbage_count +=1;
                }

            }
            continue;
        }
        match character{
            '{' =>{
                group_level +=1;
            },
            '}' =>{
                sum += group_level;
                group_level -=1;
            },
            '<' => {
                is_garbage = true;
            },
            _ => {

            }
        }
        continue;
    }
    match part{
        Part::PartOne=>{
            println!("Group Sum: {}", sum);
        },
        Part::PartTwo=>{
            println!("Garbage: {}", garbage_count);
        }
    }
}