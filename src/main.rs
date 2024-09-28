use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: bf <filename.bf>");
        process::exit(2);
    }

    let filename: &str = &args[1];

    let mut file: File = File::open(filename).expect("File not found.");
    let mut file_content: String = String::new();
    file.read_to_string(&mut file_content).expect("There was an error reading the file.");


}
