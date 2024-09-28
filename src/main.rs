use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

#[derive(Debug)]
#[derive(PartialEq)]
enum Lexem {
    NextCell,
    PreviousCell,
    Increment,
    Decrement,
    Write,
    Read,
    StartLoop,
    EndLoop
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Correct usage: bf <filename.bf>");
        process::exit(2);
    }

    let filename: &str = &args[1];

    let mut file: File = File::open(filename).expect("File not found.");
    let mut file_content: String = String::new();
    file.read_to_string(&mut file_content).expect("There was an error reading the file.");

    let _: Vec<Lexem> = lexer(file_content);
}

fn lexer(file_content: String) -> Vec<Lexem> {

    let mut lexems: Vec<Lexem> = Vec::new();

    for character in file_content.chars() {
        match character {
            '>' => lexems.push(Lexem::NextCell),
            '<' => lexems.push(Lexem::PreviousCell),
            '+' => lexems.push(Lexem::Increment),
            '-' => lexems.push(Lexem::Decrement),
            '.' => lexems.push(Lexem::Write),
            ',' => lexems.push(Lexem::Read),
            '[' => lexems.push(Lexem::StartLoop),
            ']' => lexems.push(Lexem::EndLoop),
            _ => { /* everything else is skipped */}
        }
    }

    return lexems;
}

#[test]
fn lexer_test() {
    let content: String = String::from("Text should not be counted ><+-.,[]");
    let lexems: Vec<Lexem> = lexer(content);

    assert_eq!(
        lexems,
        [
            Lexem::NextCell,
            Lexem::PreviousCell,
            Lexem::Increment,
            Lexem::Decrement,
            Lexem::Write,
            Lexem::Read,
            Lexem::StartLoop,
            Lexem::EndLoop
        ]
    );
    assert_eq!(lexems.len(), 8);
}
