// ============================================================================
// Example 3: Understanding Buffering - Why Output Doesn't Appear Immediately
// ============================================================================
//
// WHAT IS BUFFERING?
// When you write data (like println!), it doesn't go straight to the screen.
// Instead, it sits in a memory buffer (like a bucket) until:
//   1. The buffer is full (usually 8KB), OR
//   2. You print a newline (\n), OR
//   3. You manually flush, OR
//   4. The program ends
//
// WHY BUFFER?
// System calls (asking the OS to write to screen/file) are EXPENSIVE:
//   - Writing "Hello" one char at a time = 5 system calls
//   - Writing "Hello" all at once = 1 system call
//   - This makes programs 100-1000x faster for lots of output!
//
// Try running:
//   cargo run --example ex03_buffering

// ----------------------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------------------
use std::io::{self, Write};
// ↑   ↑   ↑
// │   │   └─ Import io module and Write trait (for .flush())
// │   └───── Path separator
// └─────────  Standard library

use std::thread;
// ↑   ↑
// │   └─ The thread module (for sleep function)
// └───── Standard library
//
// We need this to pause execution, so you can SEE buffering in action

use std::time::Duration;
// ↑   ↑    ↑
// │   │    └─ Duration type (represents a time span)
// │   └────── time module (time-related types)
// └────────── Standard library
//
// Duration is used to specify how long to sleep

fn main() {

    eprintln!("=== Buffering Demonstration ===\n");
    // ↑ This goes to stderr, which is UNBUFFERED - appears immediately
    // \n = escaped newline (creates blank line in output)

    // ========================================================================
    // DEMO 1: Line Buffering - The Default for Terminal Output
    // ========================================================================

    eprintln!("Demo 1: Line buffering");

    println!("This appears immediately (has newline)");
    // ↑ println! includes \n at the end
    // stdout is LINE-BUFFERED when connected to a terminal
    // The \n triggers a flush, so this appears right away

    print!("This won't appear yet (no newline)...");
    // ↑ print! has NO \n at the end
    // This text sits in the buffer...waiting...
    // You won't see it yet!

    thread::sleep(Duration::from_secs(2));
    // ↑      ↑     ↑        ↑
    // │      │     │        └─ from_secs(2) creates a 2-second Duration
    // │      │     └────────── Duration type (time span)
    // │      └──────────────── sleep() function (pauses execution)
    // └─────────────────────── thread module
    //
    // The program pauses for 2 seconds
    // During this time, the "This won't appear yet" message is STILL in the buffer!
    // You won't see it on screen

    println!(" now it appears!");
    // ↑ println! with \n triggers flush
    // NOW both messages appear together:
    //   "This won't appear yet (no newline)... now it appears!"

    thread::sleep(Duration::from_secs(1));
    // Pause for 1 second so you have time to see the output

    // ========================================================================
    // DEMO 2: Manual Flushing - Taking Control
    // ========================================================================

    eprintln!("\nDemo 2: Manual flushing");
    // \n at start creates a blank line before this message

    print!("Loading: ");
    // No newline, so this sits in the buffer...

    io::stdout().flush().unwrap();
    // ↑   ↑        ↑       ↑
    // │   │        │       └─ Crash if flush fails
    // │   │        └───────── Force buffer to write NOW
    // │   └────────────────── Get handle to stdout
    // └────────────────────── io module (from our import)
    //
    // Now "Loading: " appears immediately, even without \n!

    for i in 1..=5 {
    // Loop from 1 to 5 (inclusive)

        thread::sleep(Duration::from_millis(500));
        //                       ↑          ↑
        //                       │          └─ 500 milliseconds = 0.5 seconds
        //                       └──────────── from_millis() creates Duration from milliseconds
        //
        // Pause for half a second

        print!("{}...", i);
        // Print number with "..." (no newline)
        // This would normally sit in buffer

        io::stdout().flush().unwrap();
        // But we flush manually, so each number appears immediately!
        // You'll see: 1... (wait 0.5s) 2... (wait 0.5s) 3... etc.
    }

    println!(" Done!");
    // Final message with newline

    // WITHOUT manual flushing in the loop, you'd see nothing for 2.5 seconds,
    // then all at once: "Loading: 1...2...3...4...5... Done!"

    thread::sleep(Duration::from_secs(1));

    // ========================================================================
    // DEMO 3: stderr is Unbuffered - No Flush Needed
    // ========================================================================

    eprintln!("\nDemo 3: stderr is unbuffered (appears immediately)");

    eprint!("stderr: No flush needed...");
    // ↑ This is stderr, which is UNBUFFERED
    // It appears immediately, even without \n and without flush!

    thread::sleep(Duration::from_secs(2));
    // Wait 2 seconds (message is already visible)

    eprintln!(" See?");
    // Second part appears after the wait

    // ========================================================================
    // TODO(human): YOUR TURN TO EXPERIMENT!
    // ========================================================================
    //
    // Add code here to demonstrate the difference between buffered and
    // unbuffered output. Create a function that writes to stdout without
    // newlines in a loop, comparing behavior with and without manual flushing.
    //
    // HINT: Try a progress bar simulation that updates in place using \r
    //       (carriage return - moves cursor back to start of line)
    //
    // Example idea:
    // for i in 0..=100 {
    //     print!("\rProgress: {}%", i);  // \r moves cursor to line start
    //     io::stdout().flush().unwrap();
    //     thread::sleep(Duration::from_millis(50));
    // }
    //
    // EXPERIMENT: What happens if you don't flush?
    // EXPERIMENT: What happens if you redirect to a file?
    //             cargo run --example ex03_buffering > output.txt
    //             (stdout becomes FULLY buffered when going to a file!)

    eprintln!("\n=== Why Buffering Matters ===");
    eprintln!("- Efficiency: Writing 1000 bytes once is faster than 1 byte 1000 times");
    eprintln!("- System calls are expensive: Each write crosses user→kernel boundary");
    eprintln!("- Line buffering: Good for interactive programs (flush on \\n)");
    //                                                                  ↑↑
    // Double backslash \\ = escaped backslash (so it prints as \n)
    eprintln!("- Full buffering: Good for file writing (flush when buffer full)");
    eprintln!("- Unbuffered: Good for errors (appear immediately)");

} // End of main

// ============================================================================
// BUFFERING MODES EXPLAINED:
// ============================================================================
//
// 1. LINE-BUFFERED (stdout to terminal)
//    - Data waits in buffer until \n appears
//    - Good for: Interactive programs, command-line tools
//    - Flushes on: newline, manual flush, buffer full, program end
//
// 2. FULLY-BUFFERED (stdout to file, stderr to file)
//    - Data waits until buffer is full (usually 8KB)
//    - Good for: Maximum throughput, file writing
//    - Flushes on: buffer full, manual flush, program end
//    - Example: cargo run --example ex03_buffering > output.txt
//
// 3. UNBUFFERED (stderr to terminal)
//    - Data writes immediately, no buffering
//    - Good for: Error messages, real-time logs, debugging
//    - Flushes on: every write (always!)
//
// ============================================================================
// SYSTEM CALL COST:
// ============================================================================
//
// What happens during a system call (write to screen/file)?
//
//   1. Your program (user space)
//      ↓ System call instruction (crosses security boundary)
//   2. CPU switches to kernel mode
//   3. Operating system kernel
//      ↓ Handles the write operation
//   4. CPU switches back to user mode
//   5. Your program continues
//
// This context switch takes ~1000-5000 CPU cycles!
// For comparison, adding two numbers takes 1 cycle.
//
// Buffering reduces system calls from thousands to dozens:
//   - Without buffer: print 10,000 chars = 10,000 system calls
//   - With buffer:    print 10,000 chars = 2 system calls (2 buffer flushes)
//   - Speed improvement: 5000x faster!
//
// ============================================================================
// EXPERIMENTS TO TRY:
// ============================================================================
//
// 1. Run normally (see the delays):
//    $ cargo run --example ex03_buffering
//
// 2. Redirect to file (stdout becomes fully buffered!):
//    $ cargo run --example ex03_buffering > output.txt
//    Notice: The delays still happen (sleep), but output is different!
//    The file gets written in bigger chunks
//
// 3. See only stdout (remove stderr):
//    $ cargo run --example ex03_buffering 2> /dev/null
//    You'll only see the stdout messages
//
// 4. Measure the difference in speed (advanced):
//    Try writing 100,000 lines with and without manual flushing!
//
// ============================================================================
// KEY TAKEAWAY:
// ============================================================================
//
// Buffering is a tradeoff:
//   + Efficiency (fewer system calls)
//   - Latency (output delayed)
//
// Choose based on your use case:
//   - Interactive UI, progress bars: flush manually
//   - Large file writes: let it buffer
//   - Error messages: use stderr (unbuffered)
