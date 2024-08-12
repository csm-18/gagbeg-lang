use std::{env, fs, path::Path, process};

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
        println!("{code}");
    }
}
