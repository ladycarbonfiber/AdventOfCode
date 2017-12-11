use common::read_input;
use common::Part;
use std::cmp::max;
struct HexPoint{
    x:i32,
    y:i32
}
enum Direction{
    NW,
    N,
    NE,
    SE,
    S,
    SW
}
impl HexPoint{
    fn update(&mut self, heading:Direction){
        match heading{
            Direction::N=>{
                self.y += 1;
            },
            Direction::NE =>{
                self.x += 1;
                self.y+= 1;
            },
            Direction::SE =>{
                self.x += 1;
                self.y += -1;
            },
            Direction::S=>{
                self.y+= -1;
            },
            Direction::SW=>{
                self.x += -1;
                self.y += -1;
            },
            Direction::NW=>{
                self.x += -1;
                self.y += 1;
            }
        }
    }
}
fn get_hex_distance(a:&HexPoint, b:&HexPoint) -> i32{
    let z_a = (-1*a.x) + (-1*a.y);
    let z_b = (-1*b.x) + (-1*b.y);
    max(max(b.x - a.x, b.y - a.y), z_b - z_a)
}
fn string_to_heading(input:&str)-> Direction{
    match input{
        "n"=>{
            Direction::N
        },
        "ne"=>{
            Direction::NE
        },
        "se"=>{
            Direction::SE
        },
        "s"=>{
            Direction::S
        },
        "sw"=>{
            Direction::SW
        },
        "nw"=>{
            Direction::NW
        },
        _ =>{
            println!("Parse Error");
            Direction::N
        }
    }
}
pub fn solve(part:Part){
    let input = read_input(11);
    let steps:Vec<Direction> = input.split(",").map(|s| string_to_heading(s)).collect();
    let start = HexPoint{x:0, y:0};
    let mut end = HexPoint{x:0, y:0};
    let mut max_distance = 0;
    for step in steps{
        end.update(step);
        let cur_distance = get_hex_distance(&start, &end);
        max_distance = if cur_distance > max_distance
            { cur_distance }
            else{max_distance};

    }
    match part{
        Part::PartOne=>println!("Current distance from start {}", get_hex_distance(&start, &end)),
        Part::PartTwo=>println!("Max distance from start {}", max_distance)
    }




}