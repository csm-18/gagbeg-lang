use std::{env, fs, path::Path, process};
use chrono::prelude::*;
use eval::eval;

const VERSION: &str = "v1.0.0";
fn main() {
    // cli args
    let args: Vec<String> = env::args().collect();
    // if no args then print version
    if args.len() == 1 {
        println!("gagbeg-lang {VERSION}");
        process::exit(1);
    }
    if args.len() == 2 {
        // the first arg should be the source file name
        let filename = &args[1];
        if filename.len() < 5 {
            println!("Provided file is not a gagbeg-lang source file!");
            process::exit(1);
        }
        // source file should have .gbg extension
        let filename_extension = &filename[filename.len() - 4..];
        if filename_extension != ".gbg" {
            println!("Provided file is not a gagbeg-lang source file!");
            process::exit(1);    
        }

        // working directory(from where the user called the gagbeg binary)
        let current_dir = env::current_dir().expect("Failed to retrieve the current directory!");
        let _pwd = current_dir.to_str().unwrap();

        // check if source file exits or not
        if !Path::new(filename).exists() {
            println!("File does not exist!");
            process::exit(1);
        }

        // get source code from source file
        let code = fs::read_to_string(filename).unwrap();
        let code = code.trim().to_string();

        
        // Program execution starts from here
        println!("===== start =====\n");
        // break the code into lines
        for line in code.lines() {
            let code_line = line.trim().to_string();
            // if empty line continue
            if code_line.replace(" ", "").len() == 0 {
                continue;
            }
            
            // if not a valid statement give error
            if !code_line.starts_with("(") || !code_line.ends_with(")") {
                println!("-> Syntax error!");
                process::exit(1);
            }

          
            // date function
            if &code_line == "(date)" {
                let now = Utc::now();
                let date = now.format("%Y-%m-%d").to_string();
                println!("date: {}", date);
            }
            // comment function
            else if &code_line[1..8] == "ignore," {
                continue;
            }
            // eval function
            else if &code_line[1..6] == "eval," {
                let exp = &code_line[6..code_line.len()-1].to_string();
                for n in  exp.chars(){
                    match n {
                        ' '=> continue,
                        '0' => continue,
                        '1' => continue,
                        '2' => continue,
                        '3' => continue,
                        '4' => continue,
                        '5' => continue,
                        '6' => continue,
                        '7' => continue,
                        '8' => continue,
                        '9' => continue,
                        '.' => continue,
                        '+' => continue,
                        '-' => continue,
                        '*' => continue,
                        '/' => continue,
                        '%' => continue,
                        '(' => continue,
                        ')' => continue,
                        _ => {
                            println!("Invalid expression error!");
                            process::exit(1);
                        }
                    }
                }
                let exp_error = eval(&exp);
                match exp_error {
                    Err(_error) => {
                        println!("Expression error!")
                    },
                    Ok(result) => println!("{} = {}",exp.replace(" ", ""),result),
                }
            }
            // say function
            else if &code_line[1..6] == "say,\"" && &code_line[code_line.len()-2..] == "\")" {
                let string = &code_line[6..code_line.len()-2];
                println!("{}",string);
            }else {
                println!("-> Syntax error!");
                process::exit(1);
            } 
            
        }
        println!("====== end ======\n");
    }
    // if more than one arguments are passed!
    else {
        println!("Incorrect arguments!");
        process::exit(1);        
    }
}
