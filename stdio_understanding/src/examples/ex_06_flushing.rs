use std::io;
use std::io::Write;
fn main() {
    for i in 0..=100 {
         print!("\rProgress: {}%", i);     // \r = carriage return 
        

         io::stdout().flush().unwrap();    // Make it appear NOW
     }
    
}

