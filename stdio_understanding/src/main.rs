// stdio Learning Project - Main Menu
//
// This project teaches stdio (standard input/output) and related I/O concepts in Rust
//
// Run: cargo run
// Run specific examples: cargo run --example ex01_basic_stdio

use std::io::{self, Write};

fn main() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║   stdio & I/O Fundamentals in Rust           ║");
    println!("╚═══════════════════════════════════════════════╝");
    println!();
    println!("📚 Start by reading: docs/00_foundations.md");
    println!();
    println!("🧪 Available Examples:");
    println!();
    println!("  1. ex01_basic_stdio  - stdin/stdout basics, reading input");
    println!("  2. ex02_stderr_demo  - Understanding stderr vs stdout");
    println!("  3. ex03_buffering    - How buffering works (+ hands-on exercise)");
    println!("  4. ex04_file_io      - File I/O vs stdio comparison");
    println!("  5. ex05_pipes        - Building pipe-friendly programs");
    println!();
    println!("▶️  Run examples with:");
    println!("   cargo run --example ex01_basic_stdio");
    println!();
    println!("💡 Tips:");
    println!("   - Try redirecting: cargo run --example ex02_stderr_demo > out.txt");
    println!("   - Try piping: echo 'test' | cargo run --example ex05_pipes");
    println!("   - Watch for TODO(human) comments for hands-on practice!");
    println!();

    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();

    let mut _input = String::new();
    io::stdin().read_line(&mut _input).ok();

    println!("\n🎯 Happy learning! Start with ex01_basic_stdio");
}
