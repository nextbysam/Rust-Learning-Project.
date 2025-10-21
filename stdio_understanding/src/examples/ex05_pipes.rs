// ============================================================================
// Example 5: Understanding Pipes - Composable Programs
// ============================================================================
//
// WHAT ARE PIPES?
// The | character in your shell connects programs together:
//   program1 | program2
//        ↓
//   program1's stdout becomes program2's stdin
//
// This is the Unix philosophy: "Write programs that do one thing well and
// work together." Each program is a building block.
//
// IMPORTANT: Only stdout is piped! stderr still goes to the terminal.
// This is why we separate data (stdout) from diagnostics (stderr).
//
// Try these command combinations:
//
//   cargo run --example ex05_pipes
//   ↑ Run alone - waits for keyboard input (stdin from terminal)
//
//   echo -e "apple\nzebra\nbanana" | cargo run --example ex05_pipes
//   ↑ echo's stdout becomes our stdin
//   ↑ -e enables escape sequences (\n = newline)
//
//   cargo run --example ex05_pipes < input.txt | sort
//   ↑ Read from file, pipe output to sort command
//
//   seq 1 10 | cargo run --example ex05_pipes | head -3
//   ↑ seq generates numbers 1-10, we process them, head shows first 3 lines
//   ↑ This is a 3-program pipeline!

// ----------------------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------------------
use std::io::{self, BufRead};
// ↑   ↑   ↑    ↑     ↑
// │   │   │    │     └─ BufRead trait (adds lines() method for reading line-by-line)
// │   │   │    └─────── Import io module itself
// │   │   └──────────── Multiple imports from io
// │   └──────────────── Path separator
// └──────────────────── Standard library
//
// BufRead is a trait that adds buffered reading methods
// stdin() returns something that implements BufRead

fn main() {

    // ========================================================================
    // HEADER: Diagnostics to stderr (won't be piped)
    // ========================================================================

    eprintln!("=== Pipe-friendly Program ===");
    eprintln!("Reading from stdin, processing, writing to stdout");
    eprintln!("(Diagnostics go to stderr - won't pollute the pipe)\n");
    //
    // All these go to stderr (fd 2)
    // When piped: program1 | program2
    //   program1's stdout → program2's stdin (piped)
    //   program1's stderr → terminal (NOT piped!)
    //
    // This means you see progress/debug info even when piping!

    // ========================================================================
    // SETUP: Get stdin handle and initialize counters
    // ========================================================================

    let stdin = io::stdin();
    // ↑   ↑     ↑  ↑
    // │   │     │  └─ stdin() function returns a handle to standard input
    // │   │     └──── io module
    // │   └────────── Variable name
    // └────────────── Declare immutable variable
    //
    // stdin is a handle we can read from
    // It could be:
    //   - Keyboard input (normal terminal)
    //   - File input (< input.txt)
    //   - Pipe input (program1 | program2)
    // Your program doesn't know and doesn't care!

    let mut line_count = 0;
    // ↑   ↑   ↑         ↑
    // │   │   │         └─ Initial value
    // │   │   └─────────── Variable name
    // │   └─────────────── mut = mutable (we'll increment it)
    // └─────────────────── Declare variable
    //
    // Counter for number of lines processed

    let mut word_count = 0;
    // Counter for total number of words across all lines

    // ========================================================================
    // MAIN LOOP: Read and process lines from stdin
    // ========================================================================

    for line in stdin.lock().lines() {
    // ↑   ↑    ↑     ↑      ↑
    // │   │    │     │      └─ lines() returns an iterator over lines
    // │   │    │     │         Each item is Result<String, Error>
    // │   │    │     └──────── lock() gets exclusive access to stdin
    // │   │    │               (needed for efficient buffered reading)
    // │   │    └────────────── The stdin handle from above
    // │   └─────────────────── Variable that holds each line
    // └─────────────────────── for loop - iterate over something
    //
    // This loop runs once per line of input
    // It reads until EOF (End Of File):
    //   - Keyboard: Ctrl+D (Unix) or Ctrl+Z (Windows)
    //   - File: end of file
    //   - Pipe: when previous program closes its stdout

        match line {
        // ↑     ↑
        // │     └─ The Result<String, Error> from lines()
        // └─────── Pattern matching (handle both Ok and Err cases)

            Ok(text) => {
            // ↑  ↑
            // │  └──────── Variable name - contains the line text (without \n)
            // └───────────  Pattern: if reading succeeded

                line_count += 1;
                // ↑          ↑  ↑
                // │          │  └─ Increment by 1
                // │          └──── += operator (add and assign)
                // └───────────────  The counter variable
                //
                // Equivalent to: line_count = line_count + 1;

                let words = text.split_whitespace().count();
                // ↑   ↑     ↑    ↑                  ↑
                // │   │     │    │                  └─ count() counts items in iterator
                // │   │     │    └──────────────────── split_whitespace() splits on spaces/tabs/newlines
                // │   │     │                          Returns iterator over word slices
                // │   │     └─────────────────────────  The line text
                // │   └───────────────────────────────  Variable name
                // └───────────────────────────────────  Declare variable
                //
                // Example: "hello  world\t!" → ["hello", "world", "!"] → count = 3

                word_count += words;
                // Add this line's word count to total

                // ============================================================
                // OUTPUT: Data goes to stdout (THIS is what gets piped!)
                // ============================================================

                println!("Line {}: {} (words: {})", line_count, text.to_uppercase(), words);
                // ↑        ↑     ↑  ↑  ↑          ↑   ↑          ↑    ↑              ↑
                // │        │     │  │  │          │   │          │    │              └─ Third value (words)
                // │        │     │  │  │          │   │          │    └──────────────── .to_uppercase() converts to uppercase
                // │        │     │  │  │          │   │          │                      Returns new String: "hello" → "HELLO"
                // │        │     │  │  │          │   │          └───────────────────── The line text
                // │        │     │  │  │          │   └──────────────────────────────── Second value (text)
                // │        │     │  │  │          └──────────────────────────────────── First value (line_count)
                // │        │     │  │  └─────────────────────────────────────────────── Third placeholder
                // │        │     │  └────────────────────────────────────────────────── Second placeholder
                // │        │     └───────────────────────────────────────────────────── First placeholder
                // │        └─────────────────────────────────────────────────────────── Format string
                // └──────────────────────────────────────────────────────────────────── Macro - writes to STDOUT
                //
                // This is the MAIN OUTPUT - goes to stdout (fd 1)
                // When piped, THIS text goes to the next program!
                // Example output: "Line 1: HELLO WORLD (words: 2)"

                // ============================================================
                // DEBUG: Diagnostics go to stderr (NOT piped!)
                // ============================================================

                eprintln!("[Debug] Processed line {}", line_count);
                // ↑ Goes to stderr (fd 2)
                // When piped: program1 | program2
                //   This appears on your SCREEN, not in program2's stdin!
                //
                // This is why stderr is so important for pipes
                // You can see debug info while data flows through the pipeline
            }

            Err(e) => {
            // ↑   ↑
            // │   └────── Variable name - contains the error object
            // └────────── Pattern: if reading failed

                eprintln!("Error reading line: {}", e);
                // Error message to stderr

                std::process::exit(1);
                // Exit with error code 1
                // This terminates the program immediately
            }
        } // End of match
    } // End of for loop
    //
    // Loop ends when stdin reaches EOF:
    //   - User pressed Ctrl+D
    //   - Input file ended
    //   - Previous program in pipe closed its stdout

    // ========================================================================
    // STATISTICS: Summary to stderr (diagnostics, not data)
    // ========================================================================

    eprintln!("\n=== Statistics ===");
    eprintln!("Total lines: {}", line_count);
    eprintln!("Total words: {}", word_count);
    //
    // All to stderr! These are ABOUT the processing, not the result
    // When piped, these appear on screen, not in the pipe

    // ========================================================================
    // HELP MESSAGE: If no input was received
    // ========================================================================

    if line_count == 0 {
    // ↑  ↑           ↑  ↑
    // │  │           │  └─ Compare to 0
    // │  │           └──── == operator (equality check)
    // │  └────────────────  The counter variable
    // └───────────────────  if conditional

        eprintln!("\nNote: No input received. Try:");
        eprintln!("  echo 'hello world' | cargo run --example ex05_pipes");
        // Helpful message to stderr if user ran without input
    }

} // End of main

// ============================================================================
// PIPE MECHANICS: How | Actually Works
// ============================================================================
//
// When you run: program1 | program2
//
// The shell (bash/zsh) does this:
//
//   1. Creates a pipe (kernel object with read and write ends)
//   2. Forks (creates) two processes
//   3. Connects program1's stdout (fd 1) to pipe's write end
//   4. Connects program2's stdin (fd 0) to pipe's read end
//   5. Both programs run simultaneously
//   6. Data flows: program1 writes → pipe → program2 reads
//
// Diagram:
//
//   ┌──────────┐     ┌──────┐     ┌──────────┐
//   │program1  │────→│ PIPE │────→│program2  │
//   │stdout(1) │     │kernel│     │stdin(0)  │
//   └──────────┘     └──────┘     └──────────┘
//        ↓                              ↓
//   ┌──────────┐                  ┌──────────┐
//   │stderr(2) │─────────────────→│stderr(2) │
//   │  ↓       │    (terminal)    │  ↓       │
//   └──────────┘                  └──────────┘
//       ↓                              ↓
//    screen                         screen
//
// Notice: stderr from BOTH programs goes to terminal!
//
// ============================================================================
// UNIX PHILOSOPHY IN ACTION
// ============================================================================
//
// "Write programs that do one thing well and work together"
//
// This program:
//   - Reads from stdin (doesn't care where data comes from)
//   - Writes to stdout (doesn't care where data goes)
//   - Diagnostics to stderr (visible even when piped)
//   - Does ONE thing: uppercase text and count words
//
// Now it can combine with ANY other program:
//
//   # Generate numbers, process, sort
//   seq 1 5 | cargo run --example ex05_pipes | sort -r
//
//   # Read file, process, save
//   cat file.txt | cargo run --example ex05_pipes > output.txt
//
//   # User input, process, first 3 lines
//   cargo run --example ex05_pipes | head -3
//
// Each program is a LEGO block - mix and match!
//
// ============================================================================
// EXPERIMENTS TO TRY:
// ============================================================================
//
// 1. Keyboard input (Ctrl+D to finish):
//    $ cargo run --example ex05_pipes
//    hello world
//    test line
//    [Ctrl+D]
//
// 2. Echo to pipe:
//    $ echo "hello world" | cargo run --example ex05_pipes
//
// 3. Multi-line echo:
//    $ echo -e "line1\nline2\nline3" | cargo run --example ex05_pipes
//    (-e enables escape sequences like \n)
//
// 4. Pipe from file:
//    $ cat README.md | cargo run --example ex05_pipes
//
// 5. Pipe to another program:
//    $ echo "hello world" | cargo run --example ex05_pipes | wc -l
//    (wc -l counts lines)
//
// 6. Multi-stage pipeline:
//    $ seq 1 100 | cargo run --example ex05_pipes | grep "5" | head -5
//    (seq generates numbers, we process, grep filters, head limits)
//
// 7. Save output but see diagnostics:
//    $ echo "test" | cargo run --example ex05_pipes > output.txt
//    (stdout goes to file, stderr appears on screen!)
//
// 8. Silence diagnostics:
//    $ echo "test" | cargo run --example ex05_pipes 2> /dev/null
//    (send stderr to /dev/null "black hole")
//
// 9. Redirect both:
//    $ echo "test" | cargo run --example ex05_pipes > data.txt 2> log.txt
//
// ============================================================================
// KEY TAKEAWAYS:
// ============================================================================
//
// 1. STDIN is flexible - can be keyboard, file, or pipe
//    Your program doesn't know and doesn't need to know!
//
// 2. STDOUT is for DATA - goes through pipes
//    println!() writes to stdout
//
// 3. STDERR is for DIAGNOSTICS - bypasses pipes
//    eprintln!() writes to stderr
//
// 4. SEPARATE DATA and DIAGNOSTICS
//    This makes your program pipe-friendly and composable
//
// 5. EOF (End Of File) ends input
//    Keyboard: Ctrl+D, File: end of file, Pipe: previous program closes
//
// 6. PIPES make programs COMPOSABLE
//    Small tools that do one thing well, working together
//
// ============================================================================
// ADVANCED: What about stderr in pipes?
// ============================================================================
//
// By default: program1 | program2
//   - program1 stdout → program2 stdin (PIPED)
//   - program1 stderr → terminal (NOT PIPED)
//   - program2 stderr → terminal (NOT PIPED)
//
// To pipe stderr too: program1 2>&1 | program2
//   - Redirects stderr to stdout first, then pipes both
//
// To pipe stderr separately: program1 2> >(program2) | program3
//   - Advanced bash feature: process substitution
//
// Most of the time, you want the default: only stdout piped!
