use common::read_input;
use common::Part;
use std::collections::HashMap;

struct Firewall {
    range:usize,
    position:usize,
}
impl Firewall {
    fn step(&mut self, step_size:usize){
        //println!("{}, {}, {}", self.range, self.position, self.dx);
        self.position = (self.position +step_size) % ((self.range -1) *2 );
        //ange = self.range;
        //Firewall{position, range,dx}
    }
    fn reset(&mut self){
        self.position = 0;
    }
    fn print(&self){
        println!("Node state range {}, position {}", self.range, self.position);
    }
}
pub fn solve(part:Part){
    let input= read_input(13);
    let mut firewalls:HashMap<usize, Firewall> = HashMap::new();

    for line in input.lines(){
        let split:Vec<usize> = line.split(":")
            .map(|s| s.trim())
            .map(|s| s.parse().unwrap())
            .collect();

        let position = split[0];
        println!("Inserting into {}", position);
        let count = split[1];
        firewalls.insert(position,Firewall {range:count, position:0});
    }
    let steps = *(firewalls.keys().max().unwrap());
    let mut count = 0;
    let mut caught = false;
    let mut delay = 0;
    'delay: loop{
        if delay > 0 {
            for value in firewalls.values_mut() {
                value.step(delay as usize);
                //firewalls.get_mut(key).unwrap().step();
            }
        }
        'run: for i in 0..steps+1 {
            match firewalls.get(&i) {
                Some(f) => {
                    //&f.print();
                    if f.position == 0 {
                        count += i * f.range;
                        caught = true;
                        match part {
                            Part::PartTwo => {
                                break 'run;
                            },
                            _ => {}
                        };
                    }
                },
                None => {}
            }
            for value in firewalls.values_mut() {
                value.step(1);
            }

        }
        match part{
            Part::PartOne=>{
                println!("dectection count {}", count);
                break 'delay;
            },
            Part::PartTwo=>{
                if !caught{
                    println!("Wait a minuium of {}", delay);
                    break 'delay;
                }else{
                    count = 0;
                    caught = false;
                    for value in firewalls.values_mut() {
                        value.reset();
                    }
                    delay+=1;
                    if delay % 10000 == 0 {
                        println!("Trying with {}", delay);
                    }
                }
            }
        }
    }


}