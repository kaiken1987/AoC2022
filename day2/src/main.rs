use std::fs::File;
use std::io::{self, BufRead};

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
        let result = line.chars().nth(2).expect("err");
        let mut shift;
        
        match result{
            'X'=>{ //lose
                shift = -1i32;
                totalscore += 0; 
            },
            'Y'=>{//draw
                shift = 0;
                totalscore += 3; 
            },
            'Z'=>{//win
                shift = 1;
                totalscore += 6; 
            },
            _=>panic!("Invalid input {:?}",result)
        }
        shift = theirs as i32 + shift;
        if shift< ('A' as i32) { shift += 3; }
        if shift> ('C' as i32)  { shift -= 3; }
        let mine = std::char::from_u32( shift as u32 ).expect("err");
        match mine{
            'A'=>totalscore += 1, //rock
            'B'=>totalscore += 2, //paper
            'C'=>totalscore += 3, //sissors
            _=>panic!("Invalid input {:?}",mine)
        }
        
        //prep next line
        line = String::new();
        len = reader.read_line(&mut line).expect("Unable to read {FILENAME}");
    }
    println!("Total Score: {totalscore}\n");
}
