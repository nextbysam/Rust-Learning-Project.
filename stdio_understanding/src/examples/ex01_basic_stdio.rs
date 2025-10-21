// ============================================================================
// Example 1: Basic stdio - Reading from stdin, writing to stdout/stderr
// ============================================================================
//
// Try running this with:
//   cargo run --example ex01_basic_stdio
//   echo "Alice" | cargo run --example ex01_basic_stdio
//   cargo run --example ex01_basic_stdio < input.txt

// ----------------------------------------------------------------------------
// IMPORTS: Bringing functionality into scope
// ----------------------------------------------------------------------------
use std::io::{self, Write};
// ↑   ↑   ↑   ↑      ↑      ↑
// │   │   │   │      │      └─ Import the Write trait (adds .flush() method)
// │   │   │   │      └──────── Import the io module itself (lets us use io::stdin())
// │   │   │   └─────────────── Curly braces {} for multiple imports from same module
// │   │   └─────────────────── Path separator :: means "inside of"
// │   └─────────────────────── The io module (contains input/output functions)
// └─────────────────────────── std = standard library (built-in Rust library)
//
// Why do we need this?
// - `self` lets us write io::stdin() instead of std::io::stdin()
// - `Write` is a trait that adds the .flush() method to stdout/stderr

// ----------------------------------------------------------------------------
// MAIN FUNCTION: Entry point of the program
// ----------------------------------------------------------------------------
fn main() {
// ↑  ↑
// │  └─ Function name (must be "main" for executables)
// └──── Keyword "fn" means "function"
//
// When you run "cargo run", Rust calls this main() function first.

    // ========================================================================
    // PART 1: Writing to stderr (Standard Error Stream)
    // ========================================================================

    // CONCEPT: There are 3 standard streams:
    // - stdin  (fd 0): Where input comes from (keyboard by default)
    // - stdout (fd 1): Where regular output goes (screen by default)
    // - stderr (fd 2): Where error/diagnostic messages go (also screen)

    eprintln!("=== Basic stdio Demo ===");
    // ↑       ↑
    // │       └─ Exclamation mark ! means this is a MACRO (not a regular function)
    // │          Macros are special - they run at compile time and can do things
    // │          functions can't (like check format strings)
    // └───────── e = error (goes to stderr), print, ln = line (adds newline \n)
    //
    // So: "error print line" = write to stderr with newline
    // Output goes to file descriptor 2 (stderr)

    eprintln!("This message goes to stderr (fd 2)");
    // Same as above - writes to stderr
    // "fd 2" = file descriptor 2 (how the OS identifies stderr)

    eprintln!();
    // eprintln with empty parentheses () = print just a newline to stderr
    // Creates a blank line in the output

    // ========================================================================
    // PART 2: Prompting for input (also on stderr)
    // ========================================================================

    eprint!("What is your name? ");
    // ↑
    // └─ Like eprintln! but WITHOUT the "ln" - no newline at the end
    //    Cursor stays on same line, so user can type next to the prompt

    io::stderr().flush().unwrap();
    // ↑  ↑  ↑      ↑       ↑
    // │  │  │      │       └──── .unwrap() = if flush fails, crash the program
    // │  │  │      │             (fine for learning, not for production!)
    // │  │  │      └────────────  .flush() = force buffered data to actually write
    // │  │  │                     Returns Result<(), Error>
    // │  │  └─────────────────── () means "call this function"
    // │  └────────────────────── stderr() function returns a handle to stderr
    // └───────────────────────── io:: uses the import from line 8
    //
    // Method chaining: io::stderr().flush().unwrap()
    //   Step 1: Get stderr handle → io::stderr()
    //   Step 2: Flush it → .flush()
    //   Step 3: Unwrap result → .unwrap()

    // ========================================================================
    // PART 3: Reading from stdin (Standard Input Stream)
    // ========================================================================

    let mut input = String::new();
    // ↑   ↑   ↑       ↑      ↑
    // │   │   │       │      └─── () calls the function
    // │   │   │       └────────── new() is an "associated function" (like static method)
    // │   │   │                   Creates a new, empty String
    // │   │   └────────────────── String = growable text type (can change size)
    // │   └────────────────────── Variable name (we chose this name)
    // └────────────────────────── "let" declares a new variable
    //
    // "mut" = mutable (can be changed after creation)
    // Without "mut", variables are immutable (can't change)
    // We need "mut" because read_line() will modify this string

    match io::stdin().read_line(&mut input) {
    // ↑     ↑  ↑       ↑          ↑    ↑
    // │     │  │       │          │    └──── The variable to read into
    // │     │  │       │          └───────── &mut = mutable reference (borrow it, can modify)
    // │     │  │       └──────────────────── read_line() reads until user presses Enter
    // │     │  │                             Returns Result<usize, Error>
    // │     │  └──────────────────────────── () calls the function
    // │     └─────────────────────────────── stdin() returns handle to standard input
    // └───────────────────────────────────── "match" = pattern matching (like switch, but powerful)
    //
    // "match" forces you to handle all possible outcomes:
    //   - Success case: Ok(bytes_read)
    //   - Error case: Err(error)

        Ok(bytes_read) => {
        // ↑  ↑           ↑
        // │  │           └─ => means "if this pattern matches, do this"
        // │  └───────────── Variable name - captures the number of bytes read
        // └──────────────── Pattern: if read_line succeeded, it returns Ok(number)

            eprintln!("[Debug] Read {} bytes from stdin", bytes_read);
            //                       ↑                    ↑
            //                       │                    └─ Value to insert into {}
            //                       └────────────────────── {} is a placeholder
            //
            // Example output: "[Debug] Read 6 bytes from stdin"
            // (5 characters for "Alice" + 1 for newline \n)

            let name = input.trim();
            //         ↑     ↑
            //         │     └──── .trim() removes whitespace from start and end
            //         │           Removes spaces, tabs, newlines (\n)
            //         └────────── The variable we read into above
            //
            // User typed "Alice\n" (Enter adds \n)
            // trim() returns "Alice"

            if name.is_empty() {
            // ↑  ↑    ↑
            // │  │    └───────── () calls the method
            // │  └────────────── .is_empty() returns true if string has length 0
            // └───────────────── "if" = conditional statement

                eprintln!("Error: No name provided!");
                // Write error to stderr

                std::process::exit(1);
                // ↑   ↑       ↑    ↑
                // │   │       │    └─ Exit code 1 (non-zero = error)
                // │   │       │       Code 0 = success, 1+ = error
                // │   │       └────── exit() function terminates the program
                // │   └────────────── process module (process control)
                // └────────────────── std = standard library (full path, since we only imported io)
                //
                // The program stops here if name is empty
            }

            // If we get here, name is not empty - continue...

            // ================================================================
            // PART 4: Writing to stdout (Standard Output Stream)
            // ================================================================

            println!("Hello, {}!", name);
            // ↑        ↑            ↑
            // │        │            └─ Variable value to insert
            // │        └────────────── {} placeholder in format string
            // └─────────────────────── println! = print to stdout with newline
            //
            // KEY DIFFERENCE:
            // - println!  → stdout (fd 1) - regular output, data
            // - eprintln! → stderr (fd 2) - diagnostics, errors

            println!("Welcome to stdio learning!");
            // Another line to stdout

            // ================================================================
            // PART 5: Demonstrating buffering
            // ================================================================

            print!("Unbuffered message would appear immediately... ");
            // ↑
            // └─ print! (no "ln") = print to stdout WITHOUT newline
            //    Because stdout is LINE-BUFFERED, this sits in memory buffer
            //    and won't appear until:
            //      1. We print a newline (\n), OR
            //      2. We manually flush, OR
            //      3. The buffer fills up (usually 8KB)

            io::stdout().flush().unwrap();
            // ↑   ↑
            // │   └─ stdout() returns handle to standard output (fd 1)
            // └───── Same as stderr() before, but for stdout
            //
            // .flush() forces the buffer to empty NOW
            // Without this, the message above would wait in the buffer!

            println!("Now it appears!");
            // println! includes \n, which triggers flush anyway
        }

        Err(error) => {
        // ↑   ↑
        // │   └───────── Variable name - captures the error object
        // └───────────── Pattern: if read_line failed, it returns Err(error_object)
        //
        // This handles errors like:
        // - stdin was closed
        // - I/O error occurred

            eprintln!("Error reading from stdin: {}", error);
            //                                   ↑   ↑
            //                                   │   └─ The error object (will format itself)
            //                                   └───── Placeholder for error

            std::process::exit(1);
            // Exit with error code 1
        }
    } // End of match
} // End of main function

// ============================================================================
// KEY CONCEPTS SUMMARY:
// ============================================================================
//
// SYMBOLS:
// ::  = Path separator (navigate modules) - like / in file paths
// .   = Method call (call function on a value)
// !   = Macro (not a regular function, runs at compile time)
// &   = Reference (borrow, don't take ownership)
// mut = Mutable (can be changed)
// =>  = Match arm (pattern matching)
// ;   = Statement terminator (required in Rust)
//
// STANDARD STREAMS (inherited from parent process):
// stdin  (fd 0) = Standard Input  - where data comes from
// stdout (fd 1) = Standard Output - where results go
// stderr (fd 2) = Standard Error  - where diagnostics/errors go
//
// MACROS:
// println!()  = print to stdout with newline
// print!()    = print to stdout without newline
// eprintln!() = print to stderr with newline
// eprint!()   = print to stderr without newline
//
// BUFFERING:
// - stdout is LINE-BUFFERED (flushes on \n or manual flush)
// - stderr is UNBUFFERED (appears immediately)
// - .flush() forces buffer to write now
