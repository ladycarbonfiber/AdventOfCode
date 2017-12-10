mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day10;

use std::collections::HashSet;
use std::collections::HashMap;

use common::*;
fn main() {
    println!("Advent of Code 2017");
    //day1::day_one(Part::PartOne);
    //day2::day_two(Part::PartOne);
    //day_two(Part::PartTwo);
    //day3::day_three(Part::PartOne);
    //day4::day_four(Part::PartOne);
    //day_four(Part::PartTwo);
    //day_six(Part::PartOne);
    //day_six(Part::PartTwo);
    day_seven(Part::PartTwo);
    day10::day_ten(Part::PartOne);
    day10::day_ten(Part::PartTwo);

}



fn day_five(part:Part){
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
fn day_six(part:Part){
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
            println!("{}", cycles);
        },
        Part::PartTwo=>{
            cycles = 1;
            while blocks != loop_state {
                cycles += 1;
                blocks = redistribute(blocks);
            }
            println!("{}", cycles);
        }
    }
}
fn day_seven(part:Part){
    let input = read_input(7);
    let rows:Vec<String> = input.lines()
        .map(|s| s.replace("->", ""))
        .map(|s| s.replace(",", ""))
        .map(|s| s.replace("(", ""))
        .map(|s| s.replace(")",""))
        .collect();
    let mut nodes:Vec<Node> = Vec::new();
    let mut weights:HashMap<String, i32> = HashMap::new();
    for row in rows{
        let mut element_it = row.split_whitespace();
        let name = element_it.next().unwrap().to_string();
        let weight = element_it.next().unwrap().parse().unwrap();
        let mut children:Vec<String> = Vec::new();
        for token in element_it{
            children.push(token.to_string());
        }
        weights.insert(name.clone(), weight);
        nodes.push(Node{weight:weight, name:name, children:children});
    }
    let mut child_names:HashSet<String> = HashSet::new();
    for node in nodes.iter(){
        for child in node.children.iter(){
            child_names.insert(child.clone());
        }
    }
    let mut root = "";
    for node in nodes.iter() {
        if !child_names.contains(&node.name){
            //println!("{}", node.name);
            root = &node.name;
        }
    }
    println!("root {}", root);

    //println!("{}", nodes.len());
}
fn day_eight(part:Part){
    let input = read_input(8);
    let mut registers:HashMap<String, i32> = HashMap::new();
    let mut current_max = 0;
    for row in input.lines(){
        let contents:Vec<&str> = row.split("if").collect();
        assert_eq!(contents.len(),2);
        let mut iterator = contents.iter();
        let mut command = iterator.next().unwrap().split_whitespace();
        let mut predicate = iterator.next().unwrap().split_whitespace();
        let p_register_value = *registers.entry(predicate.next().unwrap().to_string()).or_insert(0);
        let comparator = predicate.next().unwrap();
        let value:i32 = predicate.next().unwrap().parse().unwrap();
        let execute = match comparator{
            "<" =>{
                p_register_value < value
            },
            "<=" => {
                p_register_value <= value
            },
            ">" => {
                p_register_value > value
            },
            ">=" =>{
                p_register_value >= value
            },
            "==" =>{
                p_register_value == value
            },
            "!=" =>{
                p_register_value != value
            },
            _ =>{
                false
            }
        };
        if execute{
            let c_register = command.next().unwrap();
            let c_register_value = *registers.entry(c_register.to_string()).or_insert(0);
            let action = command.next().unwrap();
            let value:i32 = command.next().unwrap().parse().unwrap();
            let new_value = match action {
                "inc" =>{
                      c_register_value + value
                },
                "dec" =>{
                    c_register_value - value
                },
                _ =>{
                    c_register_value
                }
            };
            if new_value > current_max{
                current_max = new_value;
            }
            *registers.entry(c_register.to_string()).or_insert(0) = new_value;
        }
    }
    match part{
        Part::PartOne=>{
            println!("Final highest register value is {}", registers.values().max().unwrap());
        },
        Part::PartTwo=>{
            println!("Top ever value was {}", current_max);
        }
    }
    //println!("{}", registers.values().max().unwrap());
    //println!("{}", current_max);
}
fn day_nine(part:Part){
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
