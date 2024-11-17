use std::io::stdin;
use std::io::BufRead as _;

use forth_repl::Interpreter;

fn main() {
    let ref mut host = Interpreter::new();
    let mut stdin_lines = stdin().lock().lines();

    loop {
        host.print_prompt();
        let Some(Ok(line)) = stdin_lines.next() else {
            break;
        };
        if line.trim() == ".exit" {
            break;
        }
        let result = host.eval(&line);
        if let Err(e) = result {
            println!("\x1b[31merror: {}\x1b[0m", e);
        }
        host.print_stack();
    }
}
