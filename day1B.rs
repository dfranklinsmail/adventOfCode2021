use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut index = 1;
    let mut column1 = 0; 
    let mut column2 = 0;
    let mut column3 = 0;
    let mut column4 = 0;
    let mod_cons = 4;

    let mut count = 0;
    if let Ok(lines) = read_lines("day1.dat") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let current = ip.parse::<i32>().unwrap();
                let state = index % mod_cons;
                //println!("state {} with count {}", state, count);
                if state == 1 {
                    column1 = column1 + current; 
                    if index > 3 {
                        column3 = column3 + current;
                        column4 = column4 + current;
                        
                        //println!("column2 {} column 3 {}", column2, column3);
                        if column2 < column3 { //problem
                            count = count + 1
                        }
                        column2 = 0;
                    }
                } else if state == 2 {
                    column1 = column1 + current; 
                    column2 = column2 + current; 
                    
                    if index > 3 {
                        column4 = column4 + current;
                        
                        //println!("column3 {} column 4 {}", column3, column4);
                        if column3 < column4 {
                            count = count + 1
                        }
                        column3 = 0;
                    }
                } else if state == 3 {
                    column1 = column1 + current; 
                    column2 = column2 + current; 
                    column3 = column3 + current;
                    
                    if index > 3 {
                        //println!("column4 {} column 1 {}", column4, column1);
                        if column4 < column1 {
                            count = count + 1
                        }
                        column4 = 0;
                    }
                } else if state == 0 {
                    column2 = column2 + current; 
                    column3 = column3 + current;
                    column4 = column4 + current;
                    
                    //println!("column1 {} column 2 {}", column1, column2);
                    if column1 < column2 {
                        count = count + 1
                    }
                    column1 = 0;
                    
                }
                index = index + 1;
            }
        }
        println!("total {}", count);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}