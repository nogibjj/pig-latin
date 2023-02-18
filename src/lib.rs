/*library that converts english to pig latin */

// function that accepts a character and returns true if the character is a vowel and false if the character is a consonant
pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

/*function that accepts a string in english and converts the string to pig latin and returns it
switch the first consonant or consonant cluster to the end of the term and then adding suffix “ay” to form a new word.*/
pub fn pig_latin(word: &str) -> String {
    let mut pig_latin = String::new();
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    if is_vowel(first_char) {
        pig_latin.push_str(word);
        pig_latin.push_str("-hay");
    } else {
        pig_latin.push_str(&word[1..]);
        pig_latin.push('-');
        pig_latin.push(first_char);
        pig_latin.push_str("ay");
    }
    pig_latin
}
