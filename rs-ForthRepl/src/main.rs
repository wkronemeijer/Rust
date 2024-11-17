use std::io;
use std::io::stdin;
use std::io::BufRead as _;

use forth_repl::Interpreter;

fn get_next(
    iter: &mut impl Iterator<Item = Result<String, io::Error>>,
) -> Option<String> {
    match iter.next() {
        Some(Ok(line)) => match line.trim() {
            ".exit" | "\x04" => None, // \x04 is ^D in the terminal
            _ => Some(line),
        },
        _ => None,
    }
}

fn main() {
    let ref mut host = Interpreter::new();
    let ref mut stdin_lines = stdin().lock().lines();

    host.print_motd();
    loop {
        host.print_prompt();
        let Some(line) = get_next(stdin_lines) else {
            break;
        };
        let result = host.eval(&line);
        if let Err(e) = result {
            println!("\x1b[31merror: {}\x1b[0m", e);
        }
        host.print_stack();
    }
}
