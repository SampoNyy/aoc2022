#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
use std::vec;

fn main(){
    let file_path = "input.txt";
    let mut elves: Vec<i32> = Vec::new();
    let mut elfPointer = 0;

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        elves.push(0);
        for line in lines {
            if let Ok(num) = line {
                if num != "" {
                    let cals = num.parse::<i32>().unwrap();
                    elves[elfPointer] += cals;
                }else{
                    
                    //println!("{}", elves[elfPointer]);
                    elfPointer += 1;
                    elves.push(0);
                }
            }
        }
    }
    println!("MaxValue: {}", elves.iter().max().unwrap());
    elves.sort();
    
    println!("{}", elves[elfPointer] + elves[elfPointer-1] + elves[elfPointer-2]);
    println!("{}", elves[elfPointer-1]);
    println!("{}", elves[elfPointer-2]);

}   



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}