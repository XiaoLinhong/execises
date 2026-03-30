/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let sentence = sentence.to_lowercase();
    for c in 'a'..='z' {
        if !sentence.contains(c) {
            return false
        }
    }
    true
}
