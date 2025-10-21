// MEMORY VALIDATOR - How to cross-validate bit patterns

use std::mem;

fn main() {
    println!("=== CROSS-VALIDATING BIT PATTERNS ===");
    println!();
    
    // Create known values - we know exactly what bits should be there
    let x: u32 = 0x12345678;  // We know these exact bits
    let y: u32 = 0x87654321;  // Another known pattern
    
    println!("Known values:");
    println!("x = 0x{:08x}", x);
    println!("y = 0x{:08x}", y);
    println!();
    
    // Get their memory addresses
    let x_ptr = &x as *const u32;
    let y_ptr = &y as *const u32;
    
    println!("Memory addresses:");
    println!("x at {:p}", x_ptr);
    println!("y at {:p}", y_ptr);
    println!();
    
    // METHOD 1: Direct memory read through raw pointers
    println!("=== METHOD 1: RAW POINTER READ ===");
    unsafe {
        let x_read: u32 = *x_ptr;
        let y_read: u32 = *y_ptr;
        println!("Read x: 0x{:08x} (matches: {})", x_read, x_read == x);
        println!("Read y: 0x{:08x} (matches: {})", y_read, y_read == y);
    }
    println!();
    
    // METHOD 2: Byte-by-byte analysis
    println!("=== METHOD 2: BYTE-BY-BYTE ANALYSIS ===");
    unsafe {
        println!("x bytes (little-endian):");
        print!("  0x{:x}: ", x_ptr as usize);
        for i in 0..mem::size_of::<u32>() {
            let byte_ptr = (x_ptr as *const u8).add(i);
            print!("0x{:02x} ", *byte_ptr);
        }
        println!(" = 0x{:08x}", x);
        
        println!("y bytes (little-endian):");
        print!("  0x{:x}: ", y_ptr as usize);
        for i in 0..mem::size_of::<u32>() {
            let byte_ptr = (y_ptr as *const u8).add(i);
            print!("0x{:02x} ", *byte_ptr);
        }
        println!(" = 0x{:08x}", y);
    }
    println!();
    
    // METHOD 3: Cross-validate with reinterpretation
    println!("=== METHOD 3: REINTERPRETATION VALIDATION ===");
    
    // reinterpret the same bits as different types
    unsafe {
        let x_as_two_u16s: [u16; 2] = mem::transmute(x);
        
        println!("Original x:     0x{:08x}", x);
        println!("Reinterpret as [u16;2]: [0x{:04x}, 0x{:04x}]", x_as_two_u16s[0], x_as_two_u16s[1]);
        
        // Validate: re-transform back
        let x_restored: u32 = mem::transmute(x_as_two_u16s);
        println!("Restored from [u16;2]: 0x{:08x} (valid: {})", x_restored, x_restored == x);
    }
    println!();
    
    // METHOD 4: Memory layout visualization
    println!("=== METHOD 4: MEMORY MAP VISUALIZATION ===");
    
    let stack_bottom = std::env::args().next().unwrap();
    println!("Program: {}", stack_bottom);
    
    // Show relative distances
    let x_addr = x_ptr as usize;
    let y_addr = y_ptr as usize;
    let distance = if y_addr > x_addr { y_addr - x_addr } else { x_addr - y_addr };
    
    println!("Stack layout (grows downward):");
    println!("  Higher addresses");
    println!("  0x{:016x} ──┐", x_addr);
    println!("                │ x (4 bytes)");
    println!("  0x{:016x} │", x_addr + 4);
    println!("  0x{:016x} ├─┐", y_addr);
    println!("                │ │ y (4 bytes)");
    println!("  0x{:016x} │ │", y_addr + 4);
    println!("                └─ Distance: {} bytes", distance);
    println!("  Lower addresses");
    println!();
    
    // METHOD 5: Bit pattern manipulation
    println!("=== METHOD 5: BIT PATTERN MANIPULATION ===");
    
    let mut mutable_x = x;
    println!("Original x: 0x{:08x}", mutable_x);
    
    // Flip some bits manually
    mutable_x ^= 0xFF000000;  // Flip top 8 bits
    println!("After ^ 0xFF000000: 0x{:08x}", mutable_x);
    
    // Shift bits
    mutable_x = mutable_x.rotate_left(4);
    println!("After rotate_left(4): 0x {:08x}", mutable_x);
    
    // Mask operation
    mutable_x &= 0x00FF00FF;
    println!("After & 0x00FF00FF: 0x{:08x}", mutable_x);
    
    println!();
    println!("=== FUNDAMENTAL TRUTH DEMONSTRATED ===");
    println!("Variables are just:");
    println!("1. Memory addresses (pointers)");
    println!("2. Bit patterns at those addresses");
    println!("3. Our interpretation of those patterns (types)");
    println!();
    println!("The 'same' bits can be interpreted as:");
    println!("- u32: 0x{:08x}", x);
    println!("- [u8; 4]: [{:02x}, {:02x}, {:02x}, {:02x}]", 
             (x & 0xFF) as u8, ((x >> 8) & 0xFF) as u8, 
             ((x >> 16) & 0xFF) as u8, ((x >> 24) & 0xFF) as u8);
    let bool_val = x != 0;
    println!("- bool bits: {:08b}", u8::from(bool_val));
    
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
}
