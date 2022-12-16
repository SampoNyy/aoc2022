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
            if let Ok(taskLine) = line {
                if taskLine != "" {


                    let (count,from,to) = readTask(taskLine);
                    println!("count {}, from {}, to {}",count,from,to);
                    doTask(count,from,to, &mut stacks);
                    println!("taskDone");

                }
            }
        }
        println!("newLine:");
        for stack in stacks {
            print!("{}", stack[stack.len()-1]);
        }
    }
}   


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn readTask(mut s: String) -> (i32,usize,usize){

    let mut separ = s.find("from").unwrap();
    let mut subs = &s[5..(separ-1)];
    
    println!("subs: !{}!", subs);
    let s1 = &s[(separ+5)..(s.len())];
    let count = subs.parse().unwrap();
    println!("s1: {}", s1);
    separ = s1.find("to").unwrap();

    subs = &s1[..(separ-1)];
    
    let s2 = &s1[(separ+3)..(s1.len())];

    let from: usize = subs.parse().unwrap();
    println!("s2: {}", s2);

    let to: usize = s2.parse().unwrap();
    return (count,from,to);
}
fn doTask(count: i32,from: usize, to: usize, stacks: &mut Vec<Vec<char>>) -> bool{
    let mut i = 0;
    
    let startPt = stacks[from-1].len()-usize::try_from(count).unwrap();
    let endPt = stacks[from-1].len()-1;
    let u: Vec<_> = stacks[from-1].drain(startPt..).collect();
    stacks[to-1].extend_from_slice(&u);
    /*while i < count {
        let lchar = stacks[from-1][stacks[from-1].len()-1];
        stacks[to-1].push(lchar);
        stacks[from-1].pop();
        i +=1;
    }
    */
    
    println!("from: {:?}",stacks[from-1]);
    return true;
}