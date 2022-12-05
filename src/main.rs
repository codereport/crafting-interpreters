use std::env;
use std::io;
use std::io::Write;

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Valid input");
        if buffer.len() == 1 {
            println!("exit");
            break;
        } else {
            println!("{buffer}")
        }
    }
}

fn run_file(path: &String) {
    // TODO
    println!("{path}\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        x if x > 2 => println!("Usage: lox [script]"),
        2 => run_file(&args[1]),
        _ => run_prompt(),
    }
}
