#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
use std::vec;
use std::collections::HashMap;
use regex::Regex;

//Tehdään mappi jonne laitetaan valueen dataa aina kun luetaan dataa
//kun avataan kansio -> lisätään kansionimi vectori listaan joihin kaikkiin mapataan dataa
//kun kansio suljetaan cd .. otetaan vektorista pois viimeisin 

fn main(){
    let file_path = "input.txt";
    let mut elves: Vec<i32> = Vec::new();
    let mut elfPointer = 0;
    let mut overLapCount = 0;
    let mut openFolders : Vec<String> = Vec::new();
    let mut folderMap : HashMap<String,u32> = HashMap::new();
    let re = Regex::new(r"(?P<size>\d.*) (?P<name>.*)").unwrap();
    let mut requiredSize = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(cLine) = line {

                if cLine.find("$") != None {
                    let cdPos = cLine.find("$ cd ");
                    //changing folders with .. we pop from list
                    //with something else we add to list
                    if cdPos != None {
                        if cLine.find("..") != None {
                            openFolders.pop();
                        }else{
                            if openFolders.len() > 0 {
                                openFolders.push(openFolders[openFolders.len()-1].clone() + &cLine[4..cLine.len()].to_string());
                            }else {
                                openFolders.push(cLine[0..cLine.len()].to_string());
                            }
                            
                            folderMap.entry(cLine[4..cLine.len()].to_string());

                        }
                    } 
                }else if re.is_match(&cLine) {
                    for caps in re.captures_iter(&cLine){
                        //Tämä rivi lisää kyseiseen kansioon tavaraa
                        //println!("Name: {} Value: {}", caps["name"].to_string() , caps["size"].parse::<u32>().unwrap());
                        //println!("folders: {:?}", &openFolders);
                        for entry in &openFolders {

                            let copy = entry.clone();   

                            folderMap.entry(entry.to_string()).and_modify(|size| *size += caps["size"].parse::<u32>().unwrap()).or_insert(caps["size"].parse::<u32>().unwrap());
                            let val = folderMap.get(&copy);
                            //println!("Kansio: {} Koko: {}", copy , folderMap.get(&copy.to_string()).unwrap());
                            
                            if folderMap.get(&copy) > Some(&1000000) {
                                println!("Kansio: {} Koko: {}", entry , folderMap.get(&copy).unwrap());
                                
                            }
                        }
                        //folderMap.entry(caps["name"].to_string()).and_modify(|size| *size += caps["size"].parse::<u32>().unwrap()).or_insert(caps["size"].parse().unwrap());

                        //folderMap.insert(caps["name"].to_string(), folderMap[&caps["name"]] + caps["size"].parse().unwrap()) ;
                    }
                    
                    let root = "$ cd /".to_string();
                    requiredSize = (&70000000 - *folderMap.get(&root).unwrap() as i32 - &30000000).abs();
                    println!("Required size = {}",  (&70000000 - *folderMap.get(&root).unwrap() as i32 - &30000000).abs());
                }
            }
        }
        let mut sizeSum = 0;
        
        let mut minSize = 70000000;
        for (folder, size) in folderMap {
            if size >= requiredSize.try_into().unwrap() && size < minSize  {
                println!("Folder: {}, size {}", folder, size);
                minSize = size;
            }
        }
        println!("minSize: {}", minSize);
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