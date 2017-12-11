use common::read_input;
use common::Part;
pub fn solve(part:Part){
    let input = read_input(2);
    let mut row_vals:Vec<i32> = Vec::new();
    for row in input.lines(){
        let mut numbers: Vec<i32> = row.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        numbers.sort();
        match part{
            Part::PartOne=>row_vals.push(numbers.last().unwrap() - numbers[0]),
            Part::PartTwo=>{
                for (i, entry) in numbers.iter().enumerate(){
                    for j in i+1 ..numbers.len() {
                        if numbers[j as usize] % entry == 0{
                            row_vals.push(numbers[j as usize] / entry)
                        }
                    }
                }
            }
        }

    }
    let sum:i32 = row_vals.iter().sum();

    println!("{}",sum);
}