// Example 5: Understanding Pipes - Composable Programs
//
// Pipes connect stdout of one program to stdin of another.
// This is a core Unix philosophy: small tools that work together.
//
// Try these command combinations:
//   cargo run --example ex05_pipes
//   echo -e "apple\nzebra\nbanana" | cargo run --example ex05_pipes
//   cargo run --example ex05_pipes < input.txt | sort
//   seq 1 10 | cargo run --example ex05_pipes | head -3

use std::io::{self, BufRead};

fn main() {
    eprintln!("=== Pipe-friendly Program ===");
    eprintln!("Reading from stdin, processing, writing to stdout");
    eprintln!("(Diagnostics go to stderr - won't pollute the pipe)\n");

    let stdin = io::stdin();
    let mut line_count = 0;
    let mut word_count = 0;

    // Read line by line from stdin
    for line in stdin.lock().lines() {
        match line {
            Ok(text) => {
                line_count += 1;
                let words = text.split_whitespace().count();
                word_count += words;

                // Main output goes to stdout (can be piped)
                println!("Line {}: {} (words: {})", line_count, text.to_uppercase(), words);

                // Debug info goes to stderr (won't be piped)
                eprintln!("[Debug] Processed line {}", line_count);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Final statistics to stderr (won't pollute the data stream)
    eprintln!("\n=== Statistics ===");
    eprintln!("Total lines: {}", line_count);
    eprintln!("Total words: {}", word_count);

    if line_count == 0 {
        eprintln!("\nNote: No input received. Try:");
        eprintln!("  echo 'hello world' | cargo run --example ex05_pipes");
    }
}
