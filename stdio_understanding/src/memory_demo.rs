// MEMORY DEMO - Where bits actually live in RAM

use std::mem;

fn main() {
    // Let's create some variables and see where they live
    let x: u32 = 42;           // 4 bytes on stack
    let y: u64 = 123456789;    // 8 bytes on stack  
    let s: String = String::from("hello");  // Stack pointer + heap data
    
    println!("=== MEMORY LAYOUT DEMO ===");
    println!();
    
    // Show where each variable lives in memory
    println!("Variable locations:");
    println!("x (u32):        {:p} = {}", &x as *const u32 as *const (), x);
    println!("y (u64):        {:p} = {}", &y as *const u64 as *const (), y);
    println!("s (String):     {:p} = {} (ptr: {:p}, len: {}, cap: {})", 
             &s as *const String as *const (), s,
             s.as_ptr(), s.len(), s.capacity());
    
    println!();
    println!("=== SIZE OF EACH TYPE ===");
    println!("u32:      {} bytes", mem::size_of::<u32>());
    println!("u64:      {} bytes", mem::size_of::<u64>());
    println!("String:   {} bytes (on stack)", mem::size_of::<String>());
    println!("&str:     {} bytes", mem::size_of::<&str>());
    
    println!();
    println!("=== BIT PATTERNS IN RAM ===");
    show_bytes("x = 42", &x as *const u32 as *const u8, mem::size_of::<u32>());
    show_bytes("y = 123456789", &y as *const u64 as *const u8, mem::size_of::<u64>());
    
    // Show the actual heap data
    println!("String data on heap:");
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        print!("{:02x} ", byte);
        if (i + 1) % 8 == 0 { println!(); }
    }
    println!("\n");
    
    // ASCII representation
    println!("=== ASCII VISUALIZATION ===");
    println!("Stack (simplified):");
    println!("Address:        Bytes:                    Meaning:");
    
    let x_addr = &x as *const u32 as usize;
    let y_addr = &y as *const u64 as usize;
    let s_addr = &s as *const String as usize;
    
    println!("{:016x}: {:02x} {:02x} {:02x} {:02x}          x = 42", 
             x_addr, 0x2a, 0x00, 0x00, 0x00);
    println!("{:016x}: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}  y = 123456789", 
             y_addr, 0x15, 0xcd, 0x5b, 0x07, 0x00, 0x00, 0x00, 0x00);
    println!("{:016x}: ?? ?? ?? ?? ?? ?? ?? ??  s = String struct", s_addr);
    
    // Wait for user to see the output
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
}

fn show_bytes(name: &str, ptr: *const u8, size: usize) {
    print!("{}: ", name);
    unsafe {
        for i in 0..size {
            print!("{:02x} ", *ptr.add(i));
        }
    }
    println!("({} bytes)", size);
}
