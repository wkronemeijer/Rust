use std::env::args;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead as _;
use std::io::Write as _;

use forth_repl::forth::host::StandardHost;
use forth_repl::forth::state::State;

//////////////
// Printing //
//////////////

fn print_motd() {
    println!("Welcome to \x1b[1mForthRepl\x1b[22m v0.0.1");
}

fn print_prompt() {
    print!("> ");
    stdout().flush().expect("couldn't flush 🤢");
}

fn print_stack(state: &State) {
    let stack = state.stack();
    if stack.depth() > 0 {
        print!("\x1b[34m");
        print!("{}", stack);
        println!("\x1b[39m");
    }
}

fn print_stack_after_eval(state: &mut State, line: &str) {
    let result = state.eval(line);
    if let Err(ref error) = result {
        eprintln!("\x1b[31merror: {error}\x1b[39m");
    }
    print_stack(state);
}

///////////
// Modes //
///////////

fn run_line(line: &str) {
    let ref mut host = StandardHost::new();
    let ref mut interpreter = State::new(host);
    print_stack_after_eval(interpreter, line);
}

const HELP_TEXT: &str = include_str!("./help.txt").trim_ascii();
const END_OF_TRANSMISSION: &str = "\x04"; // ^D in the terminal

fn run_repl() {
    let ref mut host = StandardHost::new();
    let ref mut state = State::new(host);
    let ref mut lines = stdin().lock().lines();

    print_motd();
    loop {
        print_prompt();
        let Some(Ok(ref line)) = lines.next() else { break };
        match line.trim() {
            ".exit" | END_OF_TRANSMISSION => break,
            ".help" => println!("{HELP_TEXT}"),
            _ => print_stack_after_eval(state, line),
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
        _ => {}
    }
}
