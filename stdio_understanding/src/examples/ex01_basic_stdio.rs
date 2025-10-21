// Example 1: Basic stdio - Reading from stdin, writing to stdout/stderr
//
// Try running this with:
//   cargo run --example ex01_basic_stdio
//   echo "Alice" | cargo run --example ex01_basic_stdio
//   cargo run --example ex01_basic_stdio < input.txt

use std::io::{self, Write};

fn main() {
    // stdout and stderr are different streams!
    // stdout: regular output (line-buffered when connected to terminal)
    // stderr: error output (unbuffered - appears immediately)

    eprintln!("=== Basic stdio Demo ===");
    eprintln!("This message goes to stderr (fd 2)");
    eprintln!();

    // Print prompt to stderr so it appears even if stdout is redirected
    eprint!("What is your name? ");
    io::stderr().flush().unwrap(); // Force stderr to show (though it's usually unbuffered)

    // Read from stdin
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(bytes_read) => {
            eprintln!("[Debug] Read {} bytes from stdin", bytes_read);

            let name = input.trim();

            if name.is_empty() {
                eprintln!("Error: No name provided!");
                std::process::exit(1);
            }

            // Write to stdout - this is the "main" output
            println!("Hello, {}!", name);
            println!("Welcome to stdio learning!");

            // stdout is buffered! Let's see this in action
            print!("Unbuffered message would appear immediately... ");
            // But this won't appear until we flush or print a newline!
            io::stdout().flush().unwrap();
            println!("Now it appears!");
        }
        Err(error) => {
            eprintln!("Error reading from stdin: {}", error);
            std::process::exit(1);
        }
    }
}
