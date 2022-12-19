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
    let (mut c0,mut c1,mut c2,mut c3,mut c4,mut c5,mut c6,mut c7,mut c8,mut c9,mut c10,mut c11,mut c12,mut c13) = (' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ');
    let mut cArr :  Vec<char> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(taskLine) = line {
                if taskLine != "" {
                    let charLine = taskLine.chars();
                    let len : usize = taskLine.len();
                    
                    for (i, c ) in charLine.enumerate() {
                        cArr.insert(0,c);
                        if cArr.len() > 14 {
                            cArr.pop();
                            println!("{:?}",cArr);
                            let mut sameFound = false;
                            let len = cArr.len()-1;
                            let (mut j, mut k) = (0,0);
                            while k < len {
                                j = 0;
                                while j < len {
                                    if cArr[k] == cArr[j] && k != j{
                                        sameFound = true;
                                        break;
                                    }
                                    j+=1;
                                }
                                
                                print!("{}{}:",cArr[k], cArr[j]);
                                if cArr[k] == cArr[j] && k != j{
                                    sameFound = true;
                                    break;
                                }
                                k+=1;
                            }
                            println!("");
                            if !sameFound {
                                println!("start of found: {:?} \nAT: {}", cArr, i);
                                break;
                            }
                        }
                        /* 
                        c14 = c13;
                        c13 = c12;
                        c12 = c11;
                        c11 = c10;
                        c10 = c9;
                        c9 = c8;
                        c8 = c7;
                        c7 = c6;
                        c6 = c5;
                        c5 = c4;
                        c4 = c3;
                        c3 = c2;
                        c2 = c1;
                        c1 = c0;
                        c0 = c;
                        if checkSames(c0,c1,c2,c3) && i > 15{
                            println!("start of found: {:?}", (c0,c1,c2,c3));
                            println!("start of message: {} AT: {}", &taskLine[i-1..i+3], i+3);
                        }*/

                    }
                }
            }
        }
    }
}   

fn checkSames(s1 : char, s2 : char, s3 : char, s4 : char) -> bool {

    if s1 != s2 && s1 != s3 && s1 != s4
    && s2 != s3 && s2 != s4 
    && s3 != s4 {
        return true;
    }else {
        return false;
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