// Convert strings to pig latin. The first consonant of each word is moved to the end of
// the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel
// have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the
// details about UTF-8 encoding!

pub fn pig_latin(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut char_iter = word.chars();
    let first_letter = char_iter.next().unwrap();
    if vowels.contains(&first_letter) {
        format!("{}-hay", &word)
    } else {
        let remaining: String = char_iter.take(word.len() - 1).collect();
        format!("{}-{}ay", &remaining, first_letter)
    }
}
