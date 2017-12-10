use common::read_input;
use common::Part;
use common::Direction;
use common::get_neighbors;
use common::Point;

use std::collections::HashMap;

pub fn day_three(part:Part){
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
            for i in side/2 +1 .. side{
                values.push(i);
            }
            let mut mark = values.into_iter().cycle();
            for i in (side-2).pow(2)+1 .. max{
                let temp = mark.next().unwrap();
                if i == input{
                    target =temp;
                }
            }
            println!("distance to center is {}", target);
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
            println!("first greater value {}", last_val);
        }
    }
}