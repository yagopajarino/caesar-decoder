use std::env;
use top_english_words::is_top_word;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_text_rot(cipher_text: &str, shift: usize) -> String {
    let alphabet_length = ALPHABET.len();
    let mut plain_text = String::new();
    for c in cipher_text.chars() {
        if c != ' ' {
            let index = ALPHABET.find(c).unwrap();
            let new_index = (index + shift) % alphabet_length;
            let new_char = ALPHABET.chars().nth(new_index).unwrap();
            plain_text.push(new_char);
        } else {
            plain_text.push(c);
        }
    }
    plain_text
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Falta agregar texo!");
        return;
    }
    let cipher_text = args[1].clone().to_lowercase();
    let alphabet_length = ALPHABET.len();
    let mut res: bool = false;
    for i in 0..alphabet_length {
        let plain_text: String = get_text_rot(&cipher_text, i);
        for word in plain_text.split_whitespace() {
            res = match is_top_word(word) {
                Some(_) => true,
                None => false,
            };
        }
        if res {
            println!(
                "Tu mensaje fue encriptado usando el Caesar Cipher!\n\nEncrypted => {}\nDecrypted => {}",
                cipher_text, plain_text
            );
            return;
        }
    }
}
