use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long, default_value = "")]
    without: String,

    #[clap(short, long, default_value = "")]
    contains: String,

    #[clap(short, long, default_value = "*****", help = "Five characters that represent positions of known characters (e.g. *a*o*).")]
    positions: String,
}

fn main() {
    let args = Args::parse();

    let without = args.without.to_lowercase();
    let contains = args.contains.to_lowercase();
    let positions = args.positions.to_lowercase();

    let without_chars: Vec<_> = without.chars().collect();
    let contains_chars: Vec<_> = contains.chars().collect();
    let positions = {
        if positions.len() != 5 {
            panic!("Positions string should contain 5 characters! (e.g. *a*o*)");
        }
        let mut new_positions = Vec::new();
        for (i, position) in positions.chars().enumerate() {
            if position != '*' {
                new_positions.push((i, position));
            }
        }
        new_positions
    };

    let word_data = include_str!("../data/words.txt");
    let words: Vec<&str> = word_data.lines().collect();
    let words = remove_words_with_chars(words, &without_chars);
    let mut words = remove_words_without_chars(words, &contains_chars);
    for (i, letter) in positions {
        words = filter_words_with_char_and_position(words, letter, i);
    }
    
    println!("Found {} word(s):", words.len());
    for word in words {
        println!("{}", word);
    }
}

fn filter_words_with_char_and_position<'a>(
    words: Vec<&'a str>,
    letter: char,
    position: usize,
) -> Vec<&'a str> {
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
        if word.chars().all(|x| chars.contains(&x)) {
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
