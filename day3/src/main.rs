use std::fs::File;
use std::io::{self, BufRead};

static FILENAME : &'static str = "input.txt";
fn main() {
    println!("Reading File: {FILENAME}\n");
    let file = File::open(FILENAME).expect("Unable to open {FILENAME}");
    let mut reader = io::BufReader::new(file);//
    
    let mut elfa = String::new();
    let mut elfb = String::new();
    let mut elfc = String::new();
    let mut len = reader.read_line(&mut elfa).expect("Unable to read {FILENAME}");
    if len>2 {len = reader.read_line(&mut elfb).expect("Unable to read {FILENAME}");}
    if len>2 {len = reader.read_line(&mut elfc).expect("Unable to read {FILENAME}");}

    let mut total_priority = 0;
    while len>2
    {
        //println!( "{compa}    {compb}");
        let common = find_common3(elfa.to_string(),elfb.to_string(),elfc.to_string());
        if common != '\r' {
            let pri : i32;
            match common {
                'a'..='z' => {pri = common as i32 - ('a' as i32) + 1},
                'A'..='Z' => {pri = common as i32 - ('A' as i32) + 27},
                _=> panic!("err"),
            };
            println!( "common item is {common}, Priority {pri}");
            total_priority += pri;
        }
        //prep next line
        elfa = String::new();
        elfb = String::new();
        elfc = String::new();
        len = reader.read_line(&mut elfa).expect("Unable to read {FILENAME}");
        if len>2 {len = reader.read_line(&mut elfb).expect("Unable to read {FILENAME}");}
        if len>2 {len = reader.read_line(&mut elfc).expect("Unable to read {FILENAME}");}
    }
    println!("Total priority: {total_priority}\n");
}

fn find_common( astr:String, bstr:String ) -> char{
    for ac in astr.as_bytes().iter() {
        for bc in bstr.as_bytes().iter(){
            if  *ac == *bc{
                return std::char::from_u32( *ac as u32 ).expect("err");
            }
        }
    }    
    return '0';
}
fn find_common3( astr:String, bstr:String, cstr:String ) -> char{
    for ac in astr.as_bytes().iter() {
        for bc in bstr.as_bytes().iter(){
            if  *ac == *bc{
                for cc in cstr.as_bytes().iter(){
                    if  *ac == *cc{
                        return std::char::from_u32( *ac as u32 ).expect("err");
                    }
                }
            }
        }
    }    
    return '\r';
}