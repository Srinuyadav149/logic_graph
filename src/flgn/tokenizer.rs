use std::fs::File;
use std::io::{self, BufRead, BufReader};
use super::trie::Trie;

enum States {
    Start,
    Keyword,
    Integer,
    Comment,
    Unknown,
    Stop
}

#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Integer(String),
    Unknown(String),
    Comment(String),
}

const KEYWORDS:[&str; 13] = [
    "BUSWIDTH",
    "READ",
    "WRITE",
    "DO",
    "HALT",
    "OK",
    "INTERRUPT",
    "AND",
    "OR",
    "NOT",
    "XOR",
    "LEFTSHIFT",
    "RIGHTSHIFT"
    ];
    
fn get_keywords_trie() -> Trie {
    let mut trie = Trie::new();
    for keyword in KEYWORDS {
        trie.insert_word(keyword);
    }
    return trie;
}

pub fn tokenize_flgn(file_path: &str) -> io::Result<Vec<Token>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut is_stopped = false;
    let mut token_vector: Vec<Token> = vec![];
    let keywords_trie: Trie = get_keywords_trie();
    for line in reader.lines() {
        let line = line?;
        let mut current_state = States::Start;
        let mut index = 0;
        let chars: Vec<char> = line.trim().chars().collect();
        let mut char_buffer: Vec<char> = vec![];
        while index <  chars.len() {
            match current_state {
                States::Start => {
                    if chars[index] == '#' {
                        current_state = States::Comment;
                        continue;
                    }
                    if chars[index] >= 'A' && chars[index] <= 'Z' {
                        current_state = States::Keyword;
                        continue;
                    }
                    if chars[index] >= '0' && chars[index] <= '9' {
                        current_state = States::Integer;
                        continue;
                    }
                    println!("Unknown Character State");
                    current_state = States::Unknown;
                },
                States::Comment => {
                    token_vector.push(Token::Comment(line));
                    break;
                },
                States::Keyword => {
                    if chars[index] >= 'A' && chars[index] <= 'Z' {
                        char_buffer.push(chars[index]);
                        index += 1;
                        if index == chars.len() {
                            let word = String::from_iter(char_buffer);
                            if keywords_trie.contains(&word) {
                                token_vector.push(Token::Keyword(word));
                            }
                            else {
                                token_vector.push(Token::Unknown(word));
                            }
                            char_buffer = vec![];
                            index += 1;
                            current_state = States::Start;
                        }
                        continue;
                    }
                    if chars[index] == ' ' {
                        let word = String::from_iter(char_buffer);
                            if keywords_trie.contains(&word) {
                                token_vector.push(Token::Keyword(word));
                            }
                            else {
                                token_vector.push(Token::Unknown(word));
                            }
                        char_buffer = vec![];
                        index += 1;
                        current_state = States::Start;
                        continue;
                    }
                    char_buffer = vec![];
                    println!("Unknown Character State");
                    current_state = States::Unknown;
                },
                States::Integer => {
                    if chars[index] >= '0' && chars[index] <= '9' {
                        char_buffer.push(chars[index]);
                        index += 1;
                        if index == chars.len() {
                            token_vector.push(Token::Integer(String::from_iter(char_buffer)));
                            char_buffer = vec![];
                            index += 1;
                            current_state = States::Start;
                        }
                        continue;
                    }
                    if chars[index] == ' ' {
                        token_vector.push(Token::Integer(String::from_iter(char_buffer)));
                        char_buffer = vec![];
                        index += 1;
                        current_state = States::Start;
                        continue;
                    }
                    println!("Unknown Character State");
                    char_buffer = vec![];
                    current_state = States::Unknown;
                },
                States::Unknown => {
                    current_state = States::Stop;
                }
                States::Stop => {
                    is_stopped = true;
                    break;
                }
            }
        }
        if is_stopped {
            println!("Stopped parsing due to an unknown Character");
            return Ok(vec![]);
        }
    }
    Ok(token_vector)
}