use std::fs::File;
use std::io::{self, BufRead};

static FILENAME : &'static str = "testinput.txt";
fn main() {
    println!("Reading File: {FILENAME}\n");
    let file = File::open(FILENAME).expect("Unable to open {FILENAME}");
    let mut reader = io::BufReader::new(file);//
    
    let mut line = String::new();
    let mut len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");

    while len>2
    {
        
        //prep next line
        line = String::new();
        len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");
    }
    //println!("Total Score: {totalscore}\n");
}
