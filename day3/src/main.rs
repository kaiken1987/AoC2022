use std::fs::File;
use std::io::{self, BufRead};

static FILENAME : &'static str = "input.txt";
fn main() {
    println!("Reading File: {FILENAME}\n");
    let file = File::open(FILENAME).expect("Unable to open {FILENAME}");
    let mut reader = io::BufReader::new(file);//
    
    let mut line = String::new();
    let mut len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");

    let mut totalPriority = 0;
    while len>2
    {
        let trim = line.trim();
        let at = trim.len()/2;
        let (compa, compb) = trim.split_at(at);
        //println!( "{compa}    {compb}");
        let common = findCommon(compa.to_string(),compb.to_string());
        if common != '0' {
            let pri : i32;
            match common {
                'a'..='z' => {pri = common as i32 - ('a' as i32) + 1},
                'A'..='Z' => {pri = common as i32 - ('A' as i32) + 27},
                _=> panic!("err"),
            };
            //println!( "common item is {common}, Priority {pri}");
            totalPriority += pri;
        }
        //prep next line
        line = String::new();
        len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");
    }
    println!("Total priority: {totalPriority}\n");
}

fn findCommon( astr:String, bstr:String ) -> char{
    for ac in astr.as_bytes().iter() {
        for bc in bstr.as_bytes().iter(){
            if  *ac == *bc{
                return std::char::from_u32( *ac as u32 ).expect("err");
            }
        }
    }    
    return '0';
}