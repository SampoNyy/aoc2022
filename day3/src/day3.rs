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
    let mut pointCount = 0;
    if let Ok(mut lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let stdin = io::stdin();

        //let mut iterator = lines.lines();
        
        while true{
            let mut singles1 = "".to_owned();
            let mut singles2 = "".to_owned();
            let mut singles3 = "".to_owned();
            
            let line1 = lines.next().unwrap().unwrap();
            findSingles(&line1, &mut singles1);
            println!("singles1: {}", singles1);
            let line2 = lines.next().unwrap().unwrap();
            findSingles(&line2, &mut singles2);
            println!("singles2: {}", singles2);
            let line3 = lines.next().unwrap().unwrap();
            findSingles(&line3, &mut singles3);
            println!("singles3: {}", singles3);
            let c = findMatch3(&singles1, &singles2,&singles3);
            
            pointCount += getVal(c);
            
            println!("points: {}", pointCount);
        }


        /*for line in lines {
            if let Ok(sacks) = line {
                //etsitään yksittäiset
                //verrataan yksittäiset keskenään
                let (s1, s2) = sacks.split_at(sacks.len()/2);
                let c = findMatch(s1, s2);
                pointCount += getVal(c);
            }
        }*/
    }
    println!("points: {}", pointCount);
}   


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn findSingles(s : &str, retStr : &mut String)->bool{

    let mut singles: String = "".to_owned();
    let mut charCount: HashMap<char, i32> = HashMap::from([('a',0),]);

    for c1 in s.chars() { 
        if !charCount.contains_key(&c1){
            charCount.insert(c1,1 );
        }else{
            let val = 2;// charCount.get(&c1).unwrap() +1;
            charCount.insert(c1,val );
        }
    }
    for (key, value) in charCount {
        if (value != 0){
            retStr.push(key);
        }
    }

    return true;
}
fn findMatch3(s1 : &String, s2 : &String, s3 : &String)-> char{
    for c1 in s1.chars() { 
        for c2 in s2.chars() { 
            if (c1 == c2){
                for c3 in s3.chars() { 
                    if (c1 == c3){
                        return c1;
                    } 
                }
            } 
        }
    }
    return ' ';
    
}
fn findMatch(s1 : &str, s2 : &str)-> char{
    for c1 in s1.chars() { 
        for c2 in s2.chars() { 
            if (c1 == c2){
                return c1;
            } 
        }
    }
    return ' ';
    
}

fn getVal(c: char) -> i32{
    
    let charVals: HashMap<char,i32> = HashMap::from([
            ('a',1),
            ('b',2),
            ('c',3),
            ('d',4),
            ('e',5),
            ('f',6),
            ('g',7),
            ('h',8),
            ('i',9),
            ('j',10),
            ('k',11),
            ('l',12),
            ('m',13),
            ('n',14),
            ('o',15),
            ('p',16),
            ('q',17),
            ('r',18),
            ('s',19),
            ('t',20),
            ('u',21),
            ('v',22),
            ('w',23),
            ('x',24),
            ('y',25),
            ('z',26),
            ('A',27),
            ('B',28),
            ('C',29),
            ('D',30),
            ('E',31),
            ('F',32),
            ('G',33),
            ('H',34),
            ('I',35),
            ('J',36),
            ('K',37),
            ('L',38),
            ('M',39),
            ('N',40),
            ('O',41),
            ('P',42),
            ('Q',43),
            ('R',44),
            ('S',45),
            ('T',46),
            ('U',47),
            ('V',48),
            ('W',49),
            ('X',50),
            ('Y',51),
            ('Z',52),
            (' ', 0),
        ]);
    
    return *charVals.get(&c).unwrap();
}