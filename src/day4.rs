use std::collections::HashSet;
use common::read_input;
use common::Part;
pub fn day_four(part:Part){
    let input = read_input(4);
    let mut count = 0;
    for line in input.lines(){
        let tokens:Vec<&str> = line.split_whitespace().collect();
        let total = tokens.len();
        let mut comp:HashSet<String> = HashSet::new();
        for element in tokens.into_iter(){
            let mut temp:Vec<char> = element.chars().collect();
            match part{
                Part::PartOne=>{

                }
                Part::PartTwo=>{
                    temp.sort();
                }
            }
            let str_value:String = temp.into_iter().collect();
            comp.insert(str_value.clone());
        }
        if total == comp.len(){
            count +=1;
        }
    }
    println!("day 4 : {}",count);

}