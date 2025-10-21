// Example 3: Understanding Buffering
//
// Buffering is when data is collected in memory before being sent to its destination.
// This is more efficient than sending every single character immediately.
//
// Try running:
//   cargo run --example ex03_buffering

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    eprintln!("=== Buffering Demonstration ===\n");

    // Demo 1: stdout is line-buffered (when connected to terminal)
    eprintln!("Demo 1: Line buffering");
    println!("This appears immediately (has newline)");
    print!("This won't appear yet (no newline)...");
    thread::sleep(Duration::from_secs(2));
    println!(" now it appears!");

    thread::sleep(Duration::from_secs(1));

    // Demo 2: Manual flushing
    eprintln!("\nDemo 2: Manual flushing");
    print!("Loading: ");
    io::stdout().flush().unwrap(); // Force it to appear

    for i in 1..=5 {
        thread::sleep(Duration::from_millis(500));
        print!("{}...", i);
        io::stdout().flush().unwrap(); // Show each number immediately
    }
    println!(" Done!");

    thread::sleep(Duration::from_secs(1));

    // Demo 3: stderr is unbuffered
    eprintln!("\nDemo 3: stderr is unbuffered (appears immediately)");
    eprint!("stderr: No flush needed...");
    thread::sleep(Duration::from_secs(2));
    eprintln!(" See?");

    // TODO(human): Experiment with buffering behavior
    // Add code here to demonstrate the difference between buffered and unbuffered output.
    // Create a function that writes to stdout without newlines in a loop,
    // comparing behavior with and without manual flushing.
    //
    // Hint: Try a progress bar simulation that updates in place using \r
    // Consider: What happens if you don't flush? What about file redirection?

    eprintln!("\n=== Why Buffering Matters ===");
    eprintln!("- Efficiency: Writing 1000 bytes once is faster than 1 byte 1000 times");
    eprintln!("- System calls are expensive: Each write crosses user→kernel boundary");
    eprintln!("- Line buffering: Good for interactive programs (flush on \\n)");
    eprintln!("- Full buffering: Good for file writing (flush when buffer full)");
    eprintln!("- Unbuffered: Good for errors (appear immediately)");
}
