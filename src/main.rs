use std::{env, process};

const VERSION: &str = "v1.0.0";
fn main() {
    // cli args
    let args: Vec<String> = env::args().collect();
    // if no args then print version
    if args.len() == 1 {
        println!("gagbeg-lang {VERSION}");
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
    }
}
