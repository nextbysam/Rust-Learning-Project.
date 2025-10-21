// ============================================================================
// Example 2: Understanding stderr vs stdout - Stream Separation
// ============================================================================
//
// This example demonstrates WHY we have two separate output streams.
// The real learning happens when you redirect them!
//
// Try running with different redirections:
//
//   cargo run --example ex02_stderr_demo
//   ↑ Normal: both stdout and stderr go to terminal screen
//
//   cargo run --example ex02_stderr_demo > output.txt
//   ↑ Redirects ONLY stdout to file (stderr still on screen)
//   ↑ The ">" is shorthand for "1>" (redirect file descriptor 1)
//
//   cargo run --example ex02_stderr_demo 2> errors.txt
//   ↑ Redirects ONLY stderr to file (stdout still on screen)
//   ↑ "2>" means redirect file descriptor 2 (stderr)
//
//   cargo run --example ex02_stderr_demo > output.txt 2> errors.txt
//   ↑ Redirects stdout to output.txt AND stderr to errors.txt
//   ↑ Screen shows nothing! Both streams go to separate files
//
//   cargo run --example ex02_stderr_demo > all.txt 2>&1
//   ↑ Redirects stdout to all.txt, then redirects stderr to wherever stdout goes
//   ↑ "2>&1" means "make fd 2 point to the same place as fd 1"
//   ↑ Everything ends up in all.txt

// ----------------------------------------------------------------------------
// NO IMPORTS NEEDED
// ----------------------------------------------------------------------------
// This example uses only println! and eprintln! which are built-in macros
// They're always available without any "use" statements

fn main() {
// No parameters, nothing fancy - just demonstrating output separation

    // ========================================================================
    // THE PATTERN: Alternating stdout and stderr
    // ========================================================================
    // We label each line so you can see where it goes when you redirect

    println!("STDOUT: This is regular output");
    // ↑ println! goes to stdout (fd 1)
    // This is "data" - the main output of the program

    eprintln!("STDERR: This is error/diagnostic output");
    // ↑ eprintln! goes to stderr (fd 2)
    // This is diagnostics - information ABOUT the process, not the results

    // Try: cargo run --example ex02_stderr_demo > data.txt
    // Your screen shows: STDERR lines only
    // data.txt contains: STDOUT lines only

    // ========================================================================
    // REALISTIC EXAMPLE: Processing with progress
    // ========================================================================

    println!("STDOUT: Processing data...");
    // Main output: telling the user we're working

    eprintln!("STDERR: [Debug] Starting processing");
    // Diagnostic: internal state for debugging
    // Marked with [Debug] to show it's not part of the result

    // ========================================================================
    // LOOP: for loop basics
    // ========================================================================

    for i in 1..=3 {
    // ↑   ↑ ↑  ↑
    // │   │ │  └─ 3 (inclusive end - this IS included)
    // │   │ └──── = makes the range inclusive (1, 2, 3)
    // │   │       Without =, it would be 1..3 = (1, 2) - excludes 3
    // │   └────── .. is the range operator
    // │          1..=3 means: 1, 2, 3
    // └────────── Variable that holds current iteration value
    //
    // "for" loops iterate over ranges, arrays, collections, etc.
    // This will run 3 times: i=1, then i=2, then i=3

        println!("STDOUT: Result #{}: Data processed", i);
        //                      ↑                     ↑
        //                      └─ First placeholder  └─ Value to insert
        //
        // Output goes to stdout (fd 1) - this is DATA
        // Example: "STDOUT: Result #1: Data processed"

        eprintln!("STDERR: [Debug] Processed item {}", i);
        // Output goes to stderr (fd 2) - this is DIAGNOSTICS
        // Example: "STDERR: [Debug] Processed item 1"
    }
    // Loop runs 3 times, producing 6 total lines (3 stdout + 3 stderr)

    // ========================================================================
    // FINAL OUTPUT: Summary
    // ========================================================================

    println!("STDOUT: === Final Results ===");
    println!("STDOUT: Total items: 3");
    println!("STDOUT: Status: Success");
    // These are the "result" - what you'd want if piping to another program
    // Example: program1 | program2
    //   program1's stdout becomes program2's stdin

    eprintln!("STDERR: [Debug] Program completed");
    // Final diagnostic message - not part of the data stream

    // ========================================================================
    // WHY SEPARATE THEM?
    // ========================================================================
    //
    // 1. FILTERED OUTPUT
    //    You can redirect stdout to a file while still seeing progress:
    //    $ cargo run --example ex02_stderr_demo > results.txt
    //    Screen shows: All the STDERR lines (progress/debug)
    //    File contains: Only the STDOUT lines (clean data)
    //
    // 2. CLEAN DATA
    //    Errors/diagnostics don't pollute your data output:
    //    $ ./data_processor > clean_data.csv 2> processing_log.txt
    //    Now clean_data.csv has ONLY the CSV output, no debug messages!
    //
    // 3. REAL-TIME FEEDBACK
    //    stderr is UNBUFFERED - appears immediately
    //    Good for progress indicators, logging, real-time monitoring
    //    stdout is buffered - good for throughput
    //
    // 4. COMPOSABILITY
    //    Tools can distinguish between data and diagnostics:
    //    $ program1 2>log1.txt | program2 2>log2.txt | program3 2>log3.txt
    //    Data flows through the pipe, but each program's logs go to separate files!
    //
    // 5. UNIX PHILOSOPHY
    //    "Write programs that do one thing well and work together"
    //    Separate streams make programs composable building blocks

} // End of main

// ============================================================================
// EXPERIMENTS TO TRY:
// ============================================================================
//
// 1. Normal run (see both streams):
//    $ cargo run --example ex02_stderr_demo
//
// 2. See only data (stderr still visible on screen):
//    $ cargo run --example ex02_stderr_demo > output.txt
//    $ cat output.txt
//
// 3. See only diagnostics (stdout still visible on screen):
//    $ cargo run --example ex02_stderr_demo 2> errors.txt
//    $ cat errors.txt
//
// 4. Separate both streams completely:
//    $ cargo run --example ex02_stderr_demo > data.txt 2> debug.txt
//    $ cat data.txt
//    $ cat debug.txt
//
// 5. Merge stderr into stdout:
//    $ cargo run --example ex02_stderr_demo > combined.txt 2>&1
//    $ cat combined.txt
//
// 6. Discard stderr, keep only stdout:
//    $ cargo run --example ex02_stderr_demo 2> /dev/null
//    (/dev/null is a "black hole" - discards everything written to it)
//
// 7. Discard stdout, keep only stderr:
//    $ cargo run --example ex02_stderr_demo > /dev/null
//
// ============================================================================
// SHELL REDIRECTION CHEAT SHEET:
// ============================================================================
//
// >     = Redirect stdout (same as 1>)
// 2>    = Redirect stderr
// &>    = Redirect both stdout and stderr (bash shorthand)
// 2>&1  = Redirect stderr to wherever stdout is going
// >>    = Append to file instead of overwriting
// |     = Pipe stdout to next command (stderr not piped!)
// |&    = Pipe both stdout and stderr (bash shorthand for 2>&1 |)
