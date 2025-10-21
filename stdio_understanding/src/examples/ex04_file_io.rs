// Example 4: File I/O vs stdio
//
// File I/O is different from stdio:
// - You explicitly open/close files
// - You get a new file descriptor (fd >= 3)
// - Not inherited from parent process
// - More control over permissions, location
//
// Try running:
//   cargo run --example ex04_file_io

use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    eprintln!("=== File I/O vs stdio ===\n");

    // Create a test file
    let filename = "test_output.txt";

    // Writing to a file (similar to stdout, but explicit)
    {
        eprintln!("Writing to file '{}'...", filename);
        let mut file = File::create(filename)?;

        // File I/O: We explicitly open and control the file
        writeln!(file, "This is line 1")?;
        writeln!(file, "This is line 2")?;
        writeln!(file, "This is line 3")?;

        // file is closed when it goes out of scope (RAII)
        eprintln!("File written successfully");
    }

    // Reading from a file (similar to stdin, but explicit)
    {
        eprintln!("\nReading from file '{}'...", filename);
        let mut file = File::open(filename)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        eprintln!("File contents:");
        println!("{}", contents);
    }

    // Compare: stdio vs file I/O
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

    // Cleanup
    std::fs::remove_file(filename)?;
    eprintln!("\nCleaned up test file");

    Ok(())
}
