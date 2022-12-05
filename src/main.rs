use std::env;
use std::io;
use std::io::Write;

fn run_file(path: &String) {
    // TODO
    println!("{path}\n");
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Valid input");
        if line.len() == 1 {
            println!("exit");
            break;
        } else {
            run(&line);
        }
    }
}

fn run(source: &String) {
    let tokens = vec![source];
    for token in tokens {
        println!("{token}");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        x if x > 2 => println!("Usage: lox [script]"),
        2 => run_file(&args[1]),
        _ => run_prompt(),
    }
}
