use std::fs::File;
use std::io::prelude::*;
pub fn read_input(day:u16) -> String{
    let path = format!("day{}.txt", day);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
pub enum Part{
    PartOne,
    PartTwo
}
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point{
    pub x:i32,
    pub y:i32,
}
impl Point{
    pub fn update(&mut self, heading:&Direction){
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
    pub fn print(&self){
        println!("x: {}, y: {}", self.x, self.y);
    }
}
pub enum Direction{
    Up,
    Left,
    Down,
    Right
}

pub fn get_neighbors(point:&Point) -> Vec<Point>{
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

