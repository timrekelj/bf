use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Debug)]
#[derive(PartialEq)]
enum Instruction {
    NextCell,
    PreviousCell,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Correct usage: bf <filename.bf>");
        process::exit(2);
    }

    let filename: &str = &args[1];

    let mut file: File = File::open(filename).expect("File not found");
    let mut file_content: String = String::new();
    file.read_to_string(&mut file_content).expect("There was an error reading the file");

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
            _ => { /* everything else is skipped */ }
        }
    }

    return lexems;
}

fn parser(lexems: Vec<Lexem>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack: usize = 0;
    let mut loop_start: usize = 0;

    for (i, lexem) in lexems.iter().enumerate() {
        if loop_stack == 0 {
            match lexem {
                Lexem::NextCell => instructions.push(Instruction::NextCell),
                Lexem::PreviousCell => instructions.push(Instruction::PreviousCell),
                Lexem::Increment => instructions.push(Instruction::Increment),
                Lexem::Decrement => instructions.push(Instruction::Decrement),
                Lexem::Write => instructions.push(Instruction::Write),
                Lexem::Read => instructions.push(Instruction::Read),
                Lexem::StartLoop => {
                    loop_start = i;
                    loop_stack += 1;
                }
                Lexem::EndLoop => panic!("Loop ended without start")
            }
        } else {
            match lexem {
                Lexem::StartLoop => {
                    loop_stack += 1;
                }
                Lexem::EndLoop => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        instructions.push(Instruction::Loop(parser(lexems[loop_start+1..i].to_vec())));
                    }
                }
                _ => { /* this should never happen */ }
            }
        }
    }

    if loop_stack != 0 {
        panic!("Loop started and never ended")
    }

    return instructions;
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

#[test]
fn parser_test() {
    let content: String = String::from("Text should not be counted ><+-.,[[]>,<]");
    let lexems: Vec<Lexem> = lexer(content);
    let instructions: Vec<Instruction> = parser(lexems);

    assert_eq!(
        instructions,
        [
            Instruction::NextCell,
            Instruction::PreviousCell,
            Instruction::Increment,
            Instruction::Decrement,
            Instruction::Write,
            Instruction::Read,
            Instruction::Loop(Vec::from([
                Instruction::Loop(Vec::new()),
                Instruction::NextCell,
                Instruction::Read,
                Instruction::PreviousCell
            ])),
        ]
    );
    assert_eq!(instructions.len(), 7);
}
