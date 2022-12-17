use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

static FILENAME : &'static str = "input.txt";
fn main() {
    println!("Reading File: {FILENAME}\n");
    let file = File::open(FILENAME).expect("Unable to open {FILENAME}");
    let mut reader = io::BufReader::new(file);//
    
    let mut line = String::new();
    let mut len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");

    let mut totalscore = 0;
    while len>2
    {
        let theirs = line.chars().nth(0).expect("err");
        let mut mine = line.chars().nth(2).expect("err");
        mine = std::char::from_u32( mine as u32 - 23).expect("err");

        match mine{
            'A'=>totalscore += 1, //rock
            'B'=>totalscore += 2, //paper
            'C'=>totalscore += 3, //sissors
            _=>panic!("Invalid input {:?}",mine)
        }

        if theirs == mine //draw
        {
            totalscore += 3;
        }
        else if (  //I win
            ( (theirs ==('A')) && (mine == ('B') )) ||
            ( (theirs ==('B')) && (mine == ('C') )) ||
            ( (theirs ==('C')) && (mine == ('A') )) 
        )
        {
            totalscore += 6;
        }
        else // I win
        {
            totalscore += 0;
        }

        
        //prep next line
        line = String::new();
        len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");
    }
    println!("Total Score: {totalscore}\n");
}
