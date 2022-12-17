use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

static FILENAME : &'static str = "input.txt";
fn main() {
    println!("Reading File: {FILENAME}\n");
    let mut cals: Vec<i32> = Vec::new();
    let file = File::open(FILENAME).expect("Unable to open {FILENAME}");
    let mut reader = io::BufReader::new(file);//
    
    let mut line = String::new();
    let mut len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");
    
    let mut cur = 0;
    while len>0
    {
        let trim = line.trim();
        let c = match trim.parse::<i32>() {
            Ok(val) => val,
            Err(error) => -1,
        };
        if c<0
        {
            cals.push( cur );
            cur = 0;
        }
        else //if (let Ok(c) = line.parse::<i32>()) 
        {
            cur = cur + c
        }
        line = String::new();
        len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");
    }
    if cur>0
    {
        cals.push( cur );
    }
    cals.sort();
    let mut cnt = 0;
    let mut top3 = 0;
    for x in cals.iter().rev() {
        cnt+=1;
        top3 += *x;
        if cnt == 3 {break;}
    }

    println!("Best: the top 3 are carrying {top3}");
}