mod forth;

use std::io::{stdin, stdout, BufRead, Error, Write};

use forth::ForthInterpreter;

fn print_prompt() {
    // IDEA: Change the prompt when compiling
    print!("> ");
    stdout().flush().expect("couldn't flush 🤢");
}

fn main() -> Result<(), Error> {
    let mut host = ForthInterpreter::new();
    let mut stdin_lines = stdin().lock().lines();
    loop {
        print_prompt();
        if let Some(line) = stdin_lines.next() {
            let input = line?;

            if input == ".exit" {
                break;
            }
            let result = host.parse(&input);
            if let Err(e) = result {
                println!("\x1b[31merror: {}\x1b[0m", e);
            }
            host.print_stack();
        } else {
            break;
        }
    }
    Ok(())
}
