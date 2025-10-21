// ============================================================================
// Example 4: File I/O vs stdio - Explicit vs Inherited Streams
// ============================================================================
//
// THE DIFFERENCE:
//
// stdio (stdin/stdout/stderr):
//   - Automatically opened by OS when program starts
//   - File descriptors 0, 1, 2 are already connected
//   - Inherited from parent process (usually your shell)
//   - You just use them - no open/close needed
//
// File I/O (File::open/create):
//   - YOU explicitly open files in your code
//   - Gets new file descriptors (3, 4, 5, ...)
//   - You control: path, permissions, read/write mode
//   - You must handle open/close (or Rust does it automatically via RAII)
//
// Try running:
//   cargo run --example ex04_file_io

// ----------------------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------------------
use std::fs::File;
// ↑   ↑   ↑
// │   │   └─ File type (represents an open file)
// │   └───── fs = filesystem module
// └─────────  Standard library

use std::io::{self, Read, Write};
// ↑   ↑   ↑    ↑     ↑     ↑
// │   │   │    │     │     └─ Write trait (adds write methods)
// │   │   │    │     └─────── Read trait (adds read methods)
// │   │   │    └───────────── Import io module itself
// │   │   └────────────────── Multiple imports from io
// │   └────────────────────── Path separator
// └────────────────────────── Standard library
//
// Read and Write are TRAITS - they add methods to types
// File implements both Read and Write

// ----------------------------------------------------------------------------
// MAIN FUNCTION with ERROR HANDLING
// ----------------------------------------------------------------------------
fn main() -> io::Result<()> {
//         ↑  ↑  ↑       ↑
//         │  │  │       └─ () = "unit type" - like void, means "no value"
//         │  │  └───────── Result<Success, Error> - can be Ok(()) or Err(error)
//         │  └──────────── io module's Result type
//         └─────────────── Arrow means "returns"
//
// This is different from previous examples!
// Previous: fn main() { ... }
// This one: fn main() -> io::Result<()> { ... }
//
// Why? File operations can FAIL (file doesn't exist, no permissions, etc.)
// By returning Result, we can use the ? operator for error handling

    eprintln!("=== File I/O vs stdio ===\n");

    let filename = "test_output.txt";
    // ↑   ↑         ↑
    // │   │         └─ String literal - file name
    // │   └─────────── Variable name
    // └─────────────── Declare immutable variable
    //
    // This is a &str (string slice) - just text

    // ========================================================================
    // PART 1: WRITING to a file (like using stdout, but explicit)
    // ========================================================================

    {
    // ↑ Opening curly brace creates a NEW SCOPE
    // Variables inside will be dropped (cleaned up) when we reach }
    // This ensures the file is closed before we try to read it

        eprintln!("Writing to file '{}'...", filename);
        // Diagnostic message to stderr

        let mut file = File::create(filename)?;
        // ↑   ↑   ↑     ↑    ↑       ↑         ↑
        // │   │   │     │    │       │         └─ ? operator: if error, return it to caller
        // │   │   │     │    │       │            (This is WHY main returns Result!)
        // │   │   │     │    │       └─────────── The filename to create
        // │   │   │     │    └─────────────────── create() opens file for writing
        // │   │   │     │                         (creates if doesn't exist, truncates if exists)
        // │   │   │     └──────────────────────── File type (from our import)
        // │   │   └────────────────────────────── Variable name
        // │   └────────────────────────────────── mut = mutable (we'll write to it)
        // └────────────────────────────────────── Declare variable
        //
        // THE ? OPERATOR is shorthand for:
        // match File::create(filename) {
        //     Ok(f) => f,
        //     Err(e) => return Err(e),
        // }

        writeln!(file, "This is line 1")?;
        // ↑        ↑     ↑                ↑
        // │        │     │                └─ ? operator: if write fails, return error
        // │        │     └──────────────────  String to write
        // │        └────────────────────────  The file to write to
        // └─────────────────────────────────  writeln! macro (like println! but to a file)
        //
        // writeln! works with ANYTHING that implements the Write trait
        // File implements Write, so we can use writeln!(file, ...)
        // Compare: println! always writes to stdout

        writeln!(file, "This is line 2")?;
        writeln!(file, "This is line 3")?;
        // Write two more lines

        // IMPORTANT: file is automatically closed here at }
        // This is RAII (Resource Acquisition Is Initialization)
        // When `file` goes out of scope, Rust calls its destructor
        // The destructor closes the file handle

        eprintln!("File written successfully");
    }
    // ↑ Closing brace: `file` goes out of scope and is CLOSED automatically
    // No need for file.close() like in Python or fclose() like in C!

    // ========================================================================
    // PART 2: READING from a file (like using stdin, but explicit)
    // ========================================================================

    {
    // Another scope block - ensures file is closed after reading

        eprintln!("\nReading from file '{}'...", filename);

        let mut file = File::open(filename)?;
        // ↑   ↑   ↑     ↑    ↑     ↑         ↑
        // │   │   │     │    │     │         └─ ? operator for error handling
        // │   │   │     │    │     └─────────── Filename to open
        // │   │   │     │    └───────────────── open() opens file for READING
        // │   │   │     └────────────────────── File type
        // │   │   └──────────────────────────── Variable name
        // │   └──────────────────────────────── mut = we'll read from it (modifies internal position)
        // └──────────────────────────────────── Declare variable
        //
        // open() vs create():
        // - open() requires file to exist, opens for reading
        // - create() creates if doesn't exist, opens for writing

        let mut contents = String::new();
        // Create empty String to hold file contents
        // Must be mut because read_to_string will fill it

        file.read_to_string(&mut contents)?;
        // ↑    ↑              ↑    ↑         ↑
        // │    │              │    │         └─ ? operator
        // │    │              │    └─────────── Variable to fill with file contents
        // │    │              └──────────────── &mut = mutable reference (can modify it)
        // │    └─────────────────────────────── read_to_string() method
        // │                                      Reads entire file into the String
        // └──────────────────────────────────── The file we're reading from
        //
        // read_to_string() is provided by the Read trait
        // File implements Read, so we can call file.read_to_string()

        eprintln!("File contents:");
        // Label (to stderr)

        println!("{}", contents);
        // ↑        ↑   ↑
        // │        │   └─ The String containing file contents
        // │        └───── {} placeholder
        // └────────────── print to stdout (not stderr!)
        //
        // The actual file contents go to stdout
        // Diagnostics ("File contents:") go to stderr
    }
    // File is automatically closed here

    // ========================================================================
    // COMPARISON: stdio vs File I/O
    // ========================================================================

    eprintln!("\n=== Comparison ===");
    eprintln!("stdio (stdin/stdout/stderr):");
    eprintln!("  - Pre-opened by OS");
    eprintln!("  - Inherited from parent process");
    eprintln!("  - File descriptors: 0, 1, 2");
    eprintln!("  - Can be redirected: program < in.txt > out.txt");
    eprintln!("");
    eprintln!("File I/O (File::open/create):");
    eprintln!("  - Explicitly opened by your code");
    eprintln!("  - Gets new file descriptor (3+)");
    eprintln!("  - Full control: path, permissions, seek position");
    eprintln!("  - Must handle open/close");

    // ========================================================================
    // CLEANUP: Remove the test file
    // ========================================================================

    std::fs::remove_file(filename)?;
    // ↑   ↑   ↑            ↑         ↑
    // │   │   │            │         └─ ? operator
    // │   │   │            └─────────── File to delete
    // │   │   └──────────────────────── remove_file() function
    // │   └──────────────────────────── fs module
    // └──────────────────────────────── Full path (we only imported File, not fs module)
    //
    // Deletes the file from disk
    // Returns Result - might fail if file doesn't exist or no permissions

    eprintln!("\nCleaned up test file");

    // ========================================================================
    // RETURN SUCCESS
    // ========================================================================

    Ok(())
    // ↑  ↑
    // │  └─ () = unit type (no value)
    // └──── Ok variant of Result
    //
    // This means: "function completed successfully, no error"
    // If any of the ? operators hit an error, we would have returned early
    // Getting here means everything worked!

} // End of main

// ============================================================================
// KEY CONCEPTS:
// ============================================================================
//
// 1. RESULT TYPE and ERROR HANDLING
//    Result<T, E> is an enum with two variants:
//      - Ok(value): Success, contains value of type T
//      - Err(error): Failure, contains error of type E
//
//    The ? operator is shorthand for:
//      match result {
//          Ok(value) => value,
//          Err(e) => return Err(e),
//      }
//
// 2. TRAITS: Read and Write
//    Traits add functionality to types:
//      - Read trait: adds read(), read_to_string(), etc.
//      - Write trait: adds write(), flush(), etc.
//    Both File and stdout/stdin implement these traits!
//
// 3. RAII (Resource Acquisition Is Initialization)
//    When a variable goes out of scope, Rust automatically calls its destructor
//    For File, this closes the file handle
//    No manual close() needed!
//    This prevents resource leaks (forgetting to close files)
//
// 4. SCOPE BLOCKS { }
//    Used to control when variables are dropped:
//      {
//          let file = File::open("test.txt")?;
//          // Use file...
//      } // ← file is closed here automatically
//      // Now safe to delete or re-open the file
//
// 5. FILE DESCRIPTORS
//    The OS uses numbers to track open files:
//      0 = stdin  (already open)
//      1 = stdout (already open)
//      2 = stderr (already open)
//      3 = your first file
//      4 = your second file
//      etc.
//
// ============================================================================
// STDIO vs FILE I/O SUMMARY:
// ============================================================================
//
// ┌─────────────────┬──────────────────────┬──────────────────────┐
// │                 │ stdio                │ File I/O             │
// ├─────────────────┼──────────────────────┼──────────────────────┤
// │ Opening         │ Pre-opened by OS     │ Explicit open/create │
// │ File Descriptor │ 0, 1, 2 (fixed)      │ 3+ (dynamic)         │
// │ Redirection     │ Shell can redirect   │ Not redirectable     │
// │ Location        │ Inherited from shell │ You specify path     │
// │ Permissions     │ Inherited            │ You control          │
// │ Closing         │ Auto (on exit)       │ Auto (RAII) or manual│
// │ Use Case        │ CLI programs         │ Reading/writing files│
// └─────────────────┴──────────────────────┴──────────────────────┘
//
// ============================================================================
// EXPERIMENTS TO TRY:
// ============================================================================
//
// 1. Run normally:
//    $ cargo run --example ex04_file_io
//    $ cat test_output.txt  # (won't exist - we deleted it!)
//
// 2. Comment out the remove_file line and run again:
//    $ cargo run --example ex04_file_io
//    $ cat test_output.txt  # Now you can see the file!
//
// 3. Try opening a file that doesn't exist:
//    Change File::open(filename) to File::open("nonexistent.txt")
//    See the error message!
//
// 4. Compare with stdio redirection:
//    $ cargo run --example ex04_file_io > captured.txt
//    $ cat captured.txt  # Only stdout (the file contents line)
//    stderr (diagnostics) still appeared on screen!
//
// ============================================================================
// ADVANCED: Understanding ? operator deeply
// ============================================================================
//
// The ? operator is "try" - it propagates errors up:
//
// Without ?:
//   let file = match File::open(filename) {
//       Ok(f) => f,
//       Err(e) => {
//           eprintln!("Error: {}", e);
//           return Err(e);
//       }
//   };
//
// With ?:
//   let file = File::open(filename)?;
//
// Much cleaner! Rust forces you to handle errors, but makes it ergonomic.
