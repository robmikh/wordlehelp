fn main() {
    let word_data = include_str!("../data/words.txt");
    let words: Vec<&str> = word_data.lines().collect();
    let words = remove_words_with_chars(words, &['w', 't', 'p', 's', 'g', 'l', 'c', 'n']);
    let words = remove_words_without_chars(words, &['r',]);
    let words = filter_words_with_char_and_position(words, 'a', 1);
    let words = filter_words_with_char_and_position(words, 'o', 3);
    println!("Found {} word(s):", words.len());
    for word in words {
        println!("{}", word);
    }
}

fn filter_words_with_char_and_position<'a>(words: Vec<&'a str>, letter: char, position: usize) -> Vec<&'a str> {
    let mut filtered_words = Vec::new();
    for word in words {
        if word.chars().nth(position).unwrap() == letter {
            filtered_words.push(word)
        }
    }
    filtered_words
}

fn remove_words_without_chars<'a>(words: Vec<&'a str>, chars: &[char]) -> Vec<&'a str> {
    let mut filtered_words = Vec::new();
    for word in words {
        if word.contains(chars) {
            filtered_words.push(word);
        }
    }
    filtered_words
}

fn remove_words_with_chars<'a>(words: Vec<&'a str>, chars: &[char]) -> Vec<&'a str> {
    let mut filtered_words = Vec::new();
    for word in words {
        if !word.contains(chars) {
            filtered_words.push(word);
        }
    }
    filtered_words
}

//fn filter_to_five_letter_words(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> Vec<String> {
//    let mut words = Vec::new();
//    for line in lines {
//        if let Ok(line) = line {
//            if line.len() == 5 {
//                words.push(line.to_lowercase());
//            }
//        }
//    }
//    words
//}
//
//fn read_lines<P: AsRef<Path>>(path: P) -> Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error> {
//    let file = std::fs::File::open(path)?;
//    let lines = std::io::BufReader::new(file).lines();
//    Ok(lines)
//}
