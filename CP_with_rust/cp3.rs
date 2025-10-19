// Write a function that returns the reference to the longer string
// without any new allocations
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
