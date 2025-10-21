# I/O Foundations: Understanding stdio and Beyond

## What is I/O?
**I/O** = **Input/Output** - the way programs communicate with the outside world.

## The I/O Hierarchy

```
┌─────────────────────────────────────────────┐
│         Your Rust Program                   │
├─────────────────────────────────────────────┤
│  Standard Streams (stdio)                   │
│  ┌────────┐  ┌────────┐  ┌────────┐       │
│  │ stdin  │  │ stdout │  │ stderr │       │
│  │ (fd 0) │  │ (fd 1) │  │ (fd 2) │       │
│  └───┬────┘  └────┬───┘  └────┬───┘       │
└──────┼───────────┼─────────────┼───────────┘
       │           │             │
       └───────────┴─────────────┘
               │
    ┌──────────┴──────────┐
    │   Operating System  │
    │   (Kernel)          │
    └──────────┬──────────┘
               │
    ┌──────────┴──────────┐
    │  Hardware Devices   │
    │  Terminal, Files,   │
    │  Networks, etc.     │
    └─────────────────────┘
```

## Core Concepts

### 1. Standard Streams (stdio)
Three pre-connected channels every program gets automatically:
- **stdin**: Standard Input (read data)
- **stdout**: Standard Output (normal output)
- **stderr**: Standard Error (error messages)

### 2. File Descriptors (fd)
Numbers the OS uses to track open files/streams:
- 0 = stdin
- 1 = stdout
- 2 = stderr
- 3+ = other opened files

### 3. Buffering
Data doesn't go directly to/from hardware - it's collected in memory first:
- **Line-buffered**: Flushes on newline (stdout to terminal)
- **Fully-buffered**: Flushes when buffer is full (files)
- **Unbuffered**: Immediate (stderr)

### 4. Redirection
Change where streams point:
```bash
program < input.txt        # stdin from file
program > output.txt       # stdout to file
program 2> errors.txt      # stderr to file
program1 | program2        # stdout → stdin (pipe)
```

## Related Concepts

### File I/O
Opening and working with files explicitly (not using inherited streams):
- Must open/close manually
- Gets new file descriptor
- Full control over location, permissions

### Network I/O
Communication over network connections:
- Sockets are also file descriptors!
- Same read/write operations
- stdio of network programs

### Memory-mapped I/O
Map file contents directly into memory:
- Access files like arrays
- OS handles reading/writing
- Very fast for large files

## Why This Matters
Understanding these layers helps you:
1. Debug when output doesn't appear (buffering)
2. Build composable CLI tools (pipes)
3. Handle errors properly (stderr vs stdout)
4. Optimize performance (buffering, file I/O)
5. Work with system-level programming
