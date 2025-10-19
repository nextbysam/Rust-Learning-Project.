use std::io::{self, Write};

fn main() {
    print!("Enter first string: ");
    io::stdout().flush().unwrap(); // Force output to show
    
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).unwrap();
    
    print!("Enter second string: ");
    io::stdout().flush().unwrap();
    
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).unwrap();
    
    match longer_wish(&s1, &s2) {
        Some(longer) => println!("Longer string: '{}'", longer.trim()),
        None => println!("Strings are equal length or both empty"),
    }
}

pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1_trimmed = s1.trim();
    let s2_trimmed = s2.trim();
    
    if s1_trimmed.chars().count() > s2_trimmed.chars().count() {
        Some(s1_trimmed)
    } else if s2_trimmed.chars().count() > s1_trimmed.chars().count() {
        Some(s2_trimmed)
    } else {
        None
    }
}
