use common::Part;
use common::read_input;
use std::collections::HashMap;
pub fn solve(part:Part){
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