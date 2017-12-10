use common::read_input;
use common::Part;

pub fn day_ten(part:Part){
    let input = read_input(10);
    let lengths:Vec<usize> = match part{
        Part::PartOne =>{
            input.split(",").map(|s| s.parse().unwrap()).collect()
        },
        Part::PartTwo=>{
            let standard_lengths:Vec<usize> = vec![17, 31, 73, 47, 23];
            let mut input_chars:Vec<usize> = input.chars().map(|c| c as usize).collect();
            input_chars.extend(standard_lengths);
            input_chars
        }
    };
    let iterations = match part{
        Part::PartOne => 1,
        Part::PartTwo=> 64
    };
    //println!("{:?}", lengths);
    //let mut cursor = lengths.iter().cycle();
    let mut start:usize = 0;
    let mut skip = 0;

    let mut line:Vec<i32> = (0..256).collect();
    let length = &line.len();
    //println!("{:?}", line);
    for _ in 0.. iterations {
        for value in lengths.iter() {
            //println!("{:?}", &line);
            for i in 0..value / 2 {
                line.swap((start + i as usize) % length, (start + value - 1 - i as usize) % length)
            }
            start += (skip + value) % length;
            skip += 1;
        }
    }
    match part{
        Part::PartOne=>{
            let val = line[0] * line[1];
            println!("product of the first two numbers is: {}", val);
        },
        Part::PartTwo=>{
            let block_size = 16;
            let mut cursor = line.into_iter();
            let mut tmp = cursor.next().unwrap();
            let mut dense_hash:Vec<i32> = Vec::new();
            let mut count = 1;
            for val in cursor{
                if count ==block_size {
                    count = 0;
                    dense_hash.push(tmp);
                    tmp = val;
                } else{
                    tmp = tmp ^ val;
                }
                count+=1;
            }
            dense_hash.push(tmp);
            //println!("{:?}", &dense_hash);
            for val in dense_hash{
                print!("{:x}", val);
            }
            //println!("Not implemented");
        }
    }

}