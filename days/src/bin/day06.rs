// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let s1_trimmed = s1.trim();
    let s2_trimmed = s2.trim();

    let len_s1 = s1_trimmed.chars().count();
    let len_s2 = s2_trimmed.chars().count();

    if len_s1 > len_s2 {
        Some(s1_trimmed)
    } else if len_s2 > len_s1 {
        Some(s2_trimmed)
    } else {
        None
    }
}
