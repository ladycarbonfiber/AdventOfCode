use common::Part;
use common::read_input;

use std::collections::HashSet;
fn redistribute(mut input:Vec<i32>) -> Vec<i32>{
    let temp = input.clone();
    let max = temp.iter()
        .max()
        .unwrap();
    let mut index = temp.iter().position(|x| x== max).unwrap();
    let mut current = *max;
    // println!("max:{}, index{}", max, index);
    input[index] = 0;
    let size = input.len();
    while current > 0{
        index+=1;
        input[index%size] += 1;
        current -= 1;
    }
    input

}
pub fn solve(part:Part){
    let input = read_input(6);
    let mut blocks:Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut seen_set = HashSet::new();
    let mut cycles = 0;
    loop{
        //println!("{:?}", blocks);

        if seen_set.contains(&blocks){
            break;
        }else{
            seen_set.insert(blocks.clone());
        }
        cycles += 1;
        blocks = redistribute(blocks);
    }
    let loop_state = blocks.clone();
    blocks = redistribute(blocks);
    match part{
        Part::PartOne=>{
            println!("Takes {} cycles to repeat", cycles);
        },
        Part::PartTwo=>{
            cycles = 1;
            while blocks != loop_state {
                cycles += 1;
                blocks = redistribute(blocks);
            }
            println!("Length of cycle {}", cycles);
        }
    }
}