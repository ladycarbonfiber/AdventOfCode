use common::read_input;
use common::Part;
fn knot_hash(input:String)->String{
    let lengths:Vec<usize> = {
        let standard_lengths:Vec<usize> = vec![17, 31, 73, 47, 23];
        let mut input_chars:Vec<usize> = input.chars().map(|c| c as usize).collect();
        input_chars.extend(standard_lengths);
        input_chars
    };
    let iterations = 64;
    let mut start:usize = 0;
    let mut skip = 0;

    let mut line:Vec<i32> = (0..256).collect();
    let length = &line.len();
    for _ in 0 .. iterations{
        for value in lengths.iter() {
            //println!("{:?}", &line);
            for i in 0..value / 2 {
                line.swap((start + i as usize) % length, (start + value - 1 - i as usize) % length)
            }
            start += (skip + value) % length;
            skip += 1;
        }
    }
    let block_size = 16;
    let dense: Vec<String> = line.chunks(block_size)
        .map(|chunk| chunk.iter().fold(0 as u8, |acc, &v| (acc ^ v as u8) as u8))
        .map(|v| format!("{:08b}", v))
        .collect();
    //assert_eq!(dense.len(), 16 as usize);
    let out = dense.join("");
    //assert_eq!(out.len(), 128 as usize);
    //println!("{:?}", &dense);
    out
}
pub fn solve(part:Part){
    let input = read_input(14);
    let mut count = 0;
    for i in 0 .. 128{
        let val = format!("{}-{}", &input, &i);
        let row = knot_hash(val);
        //println!("{}", &row);
        count += row.matches("1").count();
    }
    println!("{}",count);
}