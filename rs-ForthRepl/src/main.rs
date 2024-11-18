use std::env::args;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead as _;
use std::io::Write as _;

use forth_repl::Interpreter;

//////////////
// Printing //
//////////////

pub fn print_motd() {
    print!("Welcome to ");
    print!("\x1b[1m");
    print!("ForthRepl");
    print!("\x1b[22m");
    print!(" v0.0.1");
    println!();
}

fn print_prompt(host: &Interpreter) {
    print!("{}", host.prompt());
    stdout().flush().expect("couldn't flush 🤢");
}

fn print_stack(host: &Interpreter) {
    let stack = host.stack();
    if stack.depth() > 0 {
        print!("\x1b[34m");
        print!("{}", stack);
        println!("\x1b[39m");
    }
}

fn print_stack_after_eval(host: &mut Interpreter, line: &str) {
    let result = host.eval(line);
    if let Err(error) = result {
        eprintln!("\x1b[31merror: {error}\x1b[0m");
    }
    print_stack(host);
}

///////////
// Modes //
///////////

fn run_line(line: &str) {
    let ref mut host = Interpreter::new();
    print_stack_after_eval(host, line);
}

const HELP_TEXT: &str = include_str!("./help.txt").trim_ascii();
const END_OF_TRANSMISSION: &str = "\x04"; // ^D in the terminal

fn run_repl() {
    let ref mut host = Interpreter::new();
    let ref mut stdin_lines = stdin().lock().lines();

    print_motd();
    loop {
        print_prompt(host);
        let Some(Ok(ref line)) = stdin_lines.next() else { break };
        match line.trim() {
            ".exit" | END_OF_TRANSMISSION => break,
            ".help" => println!("{HELP_TEXT}"),
            _ => print_stack_after_eval(host, line),
        }
    }
}

//////////
// Main //
//////////

fn main() {
    let args: Vec<_> = args()
        .skip(1) // skip executable name
        .collect();
    match *args {
        [ref input] => run_line(input),
        [] => run_repl(),
        _ => println!("invalid args"),
    }
}
