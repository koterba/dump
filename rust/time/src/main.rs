use std::env;

mod creator;
use creator::segment_creator; // other file which returns a Vec<String> segment

fn num_to_seg(num: i32) -> Vec<String> { // the bool values indicate which part of the segment is one,
    match num {           // top, top left, top right, middle, bottom left, bottom right, bottom
        0 => segment_creator(true, true, true, false, true, true, true),
        1 => segment_creator(false, true, false, false, true, false, false),
        2 => segment_creator(true, false, true, true, true, false, true),
        3 => segment_creator(true, false, true, true, false, true, true),
        4 => segment_creator(false, true, true, true, false, true, false),
        5 => segment_creator(true, true, false, true, false, true, true),
        6 => segment_creator(true, true, false, true, true, true, true),
        7 => segment_creator(true, false, true, false, false, true, false),
        8 => segment_creator(true, true, true, true, true, true, true),
        9 => segment_creator(true, true, true, true, false, true, true),
        _ => segment_creator(false, false, false, false, false, false, false) // blank if invalid
    }
}

fn multiple_seg(num: i32) {
    let mut all_segs: Vec<Vec<String>> = vec![]; // vector for all segment numbers
    for i in num.to_string().chars() { // iterate over number to get the individual numbers, then turn into seg
        let i = i as i32 - '0' as i32; // as we turned it into char, turn it back into i32
        let seg = num_to_seg(i); // get the segment result for that number
        all_segs.push(seg); // push segment to vector
    }
    for i in 0..7 { // each segment has 7 lines, so we print line by line for each seg in vec.
        for seg in &all_segs {
            print!("{}", seg[i]); // print all top lines... all second lines... etc.
        }
        println!(""); // new line for next set of segment lines to be output
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let str_args_num = args[1].clone();
    let args_num: i32 = str_args_num.trim().parse().expect("Not an integer. Invalid argument.");
    multiple_seg(args_num);
}

// <3
