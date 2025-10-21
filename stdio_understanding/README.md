# stdio & I/O Fundamentals in Rust

A hands-on learning project to understand stdio (standard input/output) and related I/O concepts from first principles.

## 🎯 What You'll Learn

- **What stdio really is**: The three pre-connected streams (stdin, stdout, stderr) every program inherits
- **File descriptors**: How the OS tracks open files and streams
- **Buffering**: Why output doesn't always appear immediately and how to control it
- **Redirection & Pipes**: Building composable command-line tools
- **File I/O**: How explicit file operations differ from stdio
- **Rust specifics**: How Rust's std::io module wraps these concepts safely

## 📚 Getting Started

1. **Read the foundations**: `docs/00_foundations.md` - Understanding I/O from the ground up

2. **Run the main menu**:
   ```bash
   cargo run
   ```

3. **Work through examples in order**:

### Example 1: Basic stdio
```bash
cargo run --example ex01_basic_stdio
```
Learn stdin/stdout basics and interactive input.

### Example 2: stderr vs stdout
```bash
cargo run --example ex02_stderr_demo
cargo run --example ex02_stderr_demo > output.txt  # Watch what happens!
cargo run --example ex02_stderr_demo 2> errors.txt
```
Understand why we have separate streams for data and diagnostics.

### Example 3: Buffering ⚡
```bash
cargo run --example ex03_buffering
```
See buffering in action. **Contains TODO(human)** - hands-on exercise!

### Example 4: File I/O
```bash
cargo run --example ex04_file_io
```
Compare explicit file operations with stdio.

### Example 5: Pipes
```bash
echo -e "apple\nzebra\nbanana" | cargo run --example ex05_pipes
seq 1 10 | cargo run --example ex05_pipes | sort
```
Build Unix-style composable programs.

## 🔧 Key Commands to Try

```bash
# Redirect stdout to file
cargo run --example ex02_stderr_demo > output.txt

# Redirect stderr to file
cargo run --example ex02_stderr_demo 2> errors.txt

# Redirect both (stderr to stdout, then both to file)
cargo run --example ex02_stderr_demo > all.txt 2>&1

# Pipe output to another program
echo "test input" | cargo run --example ex05_pipes

# Provide input from file
cargo run --example ex01_basic_stdio < input.txt
```

## 🧠 Core Concepts Reference

| Concept | Description | File Descriptor |
|---------|-------------|-----------------|
| **stdin** | Standard Input - where data comes from | 0 |
| **stdout** | Standard Output - where results go | 1 |
| **stderr** | Standard Error - where diagnostics go | 2 |
| **File I/O** | Explicitly opened files | 3+ |

### Buffering Types
- **Line-buffered**: Flushes on newline (stdout to terminal)
- **Fully-buffered**: Flushes when full (files)
- **Unbuffered**: Immediate (stderr)

## 💡 Learning Path

1. ✅ Read `docs/00_foundations.md`
2. ✅ Run each example sequentially
3. ✅ Complete the TODO(human) exercises
4. ✅ Experiment with redirections and pipes
5. ✅ Try combining examples with system commands

## 🎓 Next Steps

After mastering these basics, explore:
- **Async I/O**: tokio, async-std
- **Memory-mapped I/O**: memmap2 crate
- **Network I/O**: TcpStream, UDP sockets
- **Binary I/O**: Reading/writing non-text data
- **Custom Read/Write implementations**: Creating your own I/O types

---

**Remember**: stdio is just three special files that are already open when your program starts. Everything else builds on this foundation!
