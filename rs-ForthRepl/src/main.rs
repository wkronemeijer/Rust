use std::env::args;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead as _;
use std::io::Write as _;

use forth_repl::Interpreter;
use forth_repl::StandardHost;

//////////////
// Printing //
//////////////

pub fn print_motd() {
    println!("Welcome to \x1b[1mForthRepl\x1b[22m v0.0.1");
}

fn print_prompt(interpreter: &Interpreter) {
    print!("{}", interpreter.prompt());
    stdout().flush().expect("couldn't flush 🤢");
}

fn print_stack(interpreter: &Interpreter) {
    let stack = interpreter.stack();
    if stack.depth() > 0 {
        print!("\x1b[34m");
        print!("{}", stack);
        println!("\x1b[39m");
    }
}

fn print_stack_after_eval(interpreter: &mut Interpreter, line: &str) {
    let result = interpreter.eval(line);
    if let Err(error) = result {
        eprintln!("\x1b[31merror: {error}\x1b[0m");
    }
    print_stack(interpreter);
}

///////////
// Modes //
///////////

fn run_line(line: &str) {
    let ref mut host = StandardHost::new();
    let ref mut interpreter = Interpreter::new(host);
    print_stack_after_eval(interpreter, line);
}

const HELP_TEXT: &str = include_str!("./help.txt").trim_ascii();
const END_OF_TRANSMISSION: &str = "\x04"; // ^D in the terminal

fn run_repl() {
    let ref mut host = StandardHost::new();
    let ref mut interpreter = Interpreter::new(host);
    let ref mut stdin_lines = stdin().lock().lines();

    print_motd();
    loop {
        print_prompt(interpreter);
        let Some(Ok(ref line)) = stdin_lines.next() else { break };
        match line.trim() {
            ".exit" | END_OF_TRANSMISSION => break,
            ".help" => println!("{HELP_TEXT}"),
            _ => print_stack_after_eval(interpreter, line),
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
