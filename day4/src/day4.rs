#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
use std::vec;
use std::collections::HashMap;

fn main(){
    let file_path = "input.txt";
    let mut elves: Vec<i32> = Vec::new();
    let mut elfPointer = 0;
    let mut overLapCount = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(choreLine) = line {
                if choreLine != "" {
                    let (min1,max1,min2,max2) = getRanges(choreLine);
                    if (checkRanges(min1,max1,min2,max2)){
                        println!("overlapfound {},{},{},{}", min1,max1,min2,max2);
                        overLapCount +=1;
                    }
                }
            }
        }
    }
    println!("overlaps: {}", overLapCount);
}   


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn getRanges(mut s: String) -> (i32,i32,i32,i32){
    let mut separ = s.find('-').unwrap();
    let mut subs = &s[..(separ)];
    let s1 = &s[(separ+1)..(s.len())];
    let min1 = subs.parse().unwrap();

    separ = s1.find(',').unwrap();
    subs = &s1[..(separ)];
    
    let s2 = &s1[(separ+1)..(s1.len())];
    let max1 = subs.parse().unwrap();
    println!("s2: {}", s2);

    separ = s2.find('-').unwrap();
    subs = &s2[..(separ)];;
    let s3 = &s2[(separ+1)..(s2.len())];
    let min2 = subs.parse().unwrap();
    let max2 = s3.parse().unwrap();
    return (min1,max1,min2,max2);
}
fn checkRanges(min1: i32,max1: i32,min2: i32,max2: i32) -> bool{
    if (min1 >= min2 && max1 <= max2){
        return true;
    }else if (min1 <= min2 && max1 >= max2){
        return true;
    }else if (min1 >= min2 && min1 <= max2){
        return true;
    }
    else if (max1 >= min2 && max1 <= max2){
        return true;
    }
    else {
        return false;
    }
}