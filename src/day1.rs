
use common::read_input;
use common::Part;
pub fn solve(part:Part){
    let puzzle = read_input(1);
    let input = puzzle.trim();

    let length = input.len();
    let offset = match  part{
        Part::PartOne => 1,
        Part::PartTwo => length/2
    };

    let mut sum = 0;

    let mut match_to;
    let mut current;

    let chars = input.chars();
    let mut off_set_chars = input.chars().cycle().skip(offset);

    for c in chars {
        current = c.to_digit(10).unwrap();
        match_to  = off_set_chars.next().unwrap().to_digit(10).unwrap();
        if current == match_to{
            sum+=current
        }
    }
    println!("{}",sum);

}