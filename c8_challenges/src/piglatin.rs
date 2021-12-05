// Convert strings to pig latin. The first consonant of each word is moved to the 
// end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that 
// start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
// Keep in mind the details about UTF-8 encoding!

fn piglatin(mut word: String) -> String {
    let first_letter = &word[0..1];
    if ["a","e","i","o","u"].contains(&first_letter){
        word.push_str("-hay");
        return word
    } else {
        // let mut new_word: str = word[1..];
        word[1..].to_string() + "-" + first_letter + "ay"
    }
}


pub fn run(){
    for word in ["apple", "first"] {
        println!("{} in piglatin is {}", word, piglatin(String::from(word)));
    }
}
