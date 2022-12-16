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
    let mut pointCount = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        elves.push(0);
        for line in lines {
            if let Ok(stratLine) = line {
                if stratLine != "" {
                    pointCount += CalcRoundPoints(stratLine);
                }
            }
        }
    }
    println!("points: {}", pointCount);
}   

fn CalcRoundPoints(stratLine : String) -> i32{
    //A kivi
    //B paperi
    //C Sakset
    //X -> tappio 
    //Y -> tasan
    //Z -> voitto 
    if stratLine == "A Y" {
        return 4; 
    } else if stratLine == "A X" {
        return 3; 
    } else if stratLine == "A Z" {
        return 8; 
    } else if stratLine == "B Y" {
        return 5; 
    } else if stratLine == "B X" {
        return 1; 
    } else if stratLine == "B Z" {
        return 9; 
    } else if stratLine == "C Y" {
        return 6; 
    } else if stratLine == "C X" {
        return 2; 
    } else if stratLine == "C Z" {
        return 7; 
    }else{
        return 0;
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}











