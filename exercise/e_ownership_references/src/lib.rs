pub fn inspect(word: &String) -> bool {

    if word.ends_with("s") {
        println!("{} is plural", word);
        true
    } else {
        println!("word is not plural");
        false
    }
}

pub fn change(singular_word: &mut String) {
    let is_plural = inspect(singular_word);

    if !is_plural {
        singular_word.push_str("s")
    }
}

pub fn eat(word: String) -> bool {
    // if word.starts_with("b") && word.contains("a") {
    //     true
    // } else {
    //     false
    // }

    word.starts_with("b") && word.contains("a")
}

pub fn bedazzle(arg: &mut String) {
    *arg = "sparkly".to_string();
}