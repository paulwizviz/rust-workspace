use std::str;

pub fn form_sentence(words: &[&str]) -> String{
    let mut index = 0;
    let last_index = words.len() - 1;
    let mut sentence = String::new();
    for word in words {
        if index == 0 {
            sentence = format!("{}", word);
            index = index + 1;
        } else if index == last_index{
            sentence = format!("{} {}.", sentence, word);
            break;
        } else{
            sentence = format!("{} {}", sentence, word);
            index = index + 1;
        }
    }    
    return sentence
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(crate::form_sentence(&["Hello", "World", "and", "Earth"]), "Hello World and Earth.");
    }
}
