use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
    //day_two(Part::PartOne);
    //day_two(Part::PartTwo);
   // day_three(Part::PartOne);
    day_four(Part::PartOne);
    day_four(Part::PartTwo);
}
fn read_input(day:u16) -> String{
    let path = format!("day{}.txt", day);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
enum Part{
    PartOne,
    PartTwo
}
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point{
    x:i32,
    y:i32,
}
impl Point{
    fn update(&mut self, heading:&Direction){
        match heading {
            &Direction::Right => {
                //println!("right");
                self.x +=1;
            },
            &Direction::Left => {
                //println!("left");
                self.x -=1;
            },
            &Direction::Up => {
                //println!("up");
                self.y+=1;
            },
            &Direction::Down => {
                //println!("down");
                self.y-=1;
            }
        }
    }
    fn print(&self){
        println!("x: {}, y: {}", self.x, self.y);
    }
}
enum Direction{
    Up,
    Left,
    Down,
    Right
}

fn get_neighbors(point:&Point) -> Vec<Point>{
    let mut neighbors = Vec::new();
    for i in {-1 ..2}{
        for j in {-1..2}{
            if !(i==0 && j==0) {
                neighbors.push(Point { x: point.x +i, y: point.y +j })
            }
        }
    }
    neighbors
}
fn day_one(part:Part){
    let puzzle = read_input(1);
    let input = puzzle.trim();

    let length = input.len();
    let offset = match  part{
        part::PartOne => 1,
        part::PartTwo => length/2
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
fn day_two(part:Part){
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
fn day_three(part:Part){
    let input:i32 = read_input(3).trim().parse().unwrap();
    let mut target = 0;
    match part{
        Part::PartOne=>{
            let mut side = (input as f64).sqrt().ceil() as i32;
            if side %2 == 0{ side += 1;}
            let max = side * side;
            let mut values:Vec<i32> = Vec::new();
            for i in (side/2 .. side-1).rev(){
                values.push(i);
            }
            for i in (side/2 +1 .. side){
                values.push(i);
            }
            let mut mark = values.into_iter().cycle();
            for i in ((side-2).pow(2)+1 .. max){
                let temp = mark.next().unwrap();
                if i == input{
                    target =temp;
                }
            }
            println!("{}", target);
        },
        Part::PartTwo=>{
            let mut last_val = 1;
            let mut current_point = Point{x:1, y:0};
            let mut grid = HashMap::new();
            //state variable for spiral
            let mut side = 1;
            let mut mag = 0;
            let mut side_state = 1;
            let directions = vec![Direction::Up, Direction::Left, Direction::Down, Direction::Right];
            let mut spiral = directions.iter().cycle();
            let mut heading = spiral.next().unwrap();
            grid.insert(Point{x:0,y:0}, 1);
            while last_val <= input{
                let neighbors = get_neighbors(&current_point);
                let mut val = 0;
                for pt in neighbors{
                    //println!("{},{}", &pt.x, &pt.y);
                    val += match grid.get(&pt){
                        Some(x) => *x,
                        None =>0
                    }
                }
                if mag == side {
                    heading = spiral.next().unwrap();
                    mag =1;
                    if side_state == 1{
                        side+=1;
                        side_state=0;
                    }else{
                        side_state += 1;
                    }

                }else{
                    mag+=1;
                }

                let current = grid.entry(current_point).or_insert(0);
                *current = val;
                //println!("current val: {}", &val);
                last_val = val;
                current_point.update(heading);
            }
            println!("{}", last_val);
        }
    }
}
fn day_four(part:Part){
    let input = read_input(4);
    let mut count = 0;
    for line in input.lines(){
        let mut tokens:Vec<&str> = line.split_whitespace().collect();
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
    println!("{}",count);

}
fn day_five(part:part){
    let input = read_puzzle(5);
    let mut instructions:Vec<i32> = input.lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut index:usize = 0;
    let mut steps = 0;
    let bound = instructions.len();
    while index >= 0 && index < bound{
        steps+=1;
        let inst = instructions[index];
        match part{
            part::PartOne=>{
                instructions[index] = inst+1;
            },
            part::PartTwo=>{
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