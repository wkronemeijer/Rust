use std::env::args;
use std::io::stdin;
use std::io::BufRead as _;

use forth_repl::Interpreter;

fn eval_print(host: &mut Interpreter, line: &str) {
    let result = host.eval(line);
    if let Err(error) = result {
        eprintln!("\x1b[31merror: {error}\x1b[0m");
    }
    host.print_stack();
}

fn run_line(line: &str) { eval_print(&mut Interpreter::new(), line); }

const HELP_TEXT: &str = include_str!("./help.txt").trim_ascii();

fn run_repl() {
    let ref mut host = Interpreter::new();
    let ref mut stdin_lines = stdin().lock().lines();

    host.print_motd();
    loop {
        host.print_prompt();
        let Some(Ok(ref line)) = stdin_lines.next() else { break };
        match line.trim() {
            ".exit" | "\x04" => break, // \x04 is ^D in the terminal
            ".help" => println!("{HELP_TEXT}"),
            _ => eval_print(host, line),
        }
    }
}

fn main() {
    let args: Vec<_> = args().skip(1).collect(); // skip executable name
    match *args {
        [ref input] => run_line(input),
        [] => run_repl(),
        _ => println!("invalid args"),
    }
}
