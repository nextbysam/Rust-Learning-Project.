// Example 2: Understanding stderr vs stdout
//
// Try running with different redirections:
//   cargo run --example ex02_stderr_demo
//   cargo run --example ex02_stderr_demo > output.txt
//   cargo run --example ex02_stderr_demo 2> errors.txt
//   cargo run --example ex02_stderr_demo > output.txt 2> errors.txt
//   cargo run --example ex02_stderr_demo > all.txt 2>&1

fn main() {
    // Let's produce output to both stdout and stderr

    println!("STDOUT: This is regular output");
    eprintln!("STDERR: This is error/diagnostic output");

    println!("STDOUT: Processing data...");
    eprintln!("STDERR: [Debug] Starting processing");

    // Simulate some work
    for i in 1..=3 {
        println!("STDOUT: Result #{}: Data processed", i);
        eprintln!("STDERR: [Debug] Processed item {}", i);
    }

    println!("STDOUT: === Final Results ===");
    println!("STDOUT: Total items: 3");
    println!("STDOUT: Status: Success");

    eprintln!("STDERR: [Debug] Program completed");

    // Why separate them?
    // 1. You can redirect stdout to a file while still seeing progress on screen
    // 2. Errors don't pollute your data output
    // 3. stderr is unbuffered - appears immediately (good for real-time logs)
    // 4. Tools can distinguish between data (stdout) and diagnostics (stderr)
}
