// Copyright © 2022 Brandon Li. All rights reserved.

// A tokenizer for Solis. A tokenizer takes in a Solis raw string input and turns it
// into a vector of Tokens. This is the first stage of the front end of the compiler. Working with
// tokens will be much easier to work with compared to the raw string.
//
// Internally, the tokenizer works by creating a regex pattern for each token. It will then match
// the raw string to find the correct token, and then "consume" the token and move on. The process
// repeats until all tokens have been consumed.

extern crate regex;
extern crate lazy_static;

use std::process::exit;
use self::regex::Regex;
use self::lazy_static::lazy_static;

/// Possible tokens returned by the tokenizer
#[derive(Debug)]
pub enum Token {
    Let,
    Colon,
    Final,
    Equals,
    Id(String),
    Int(i32),
}

lazy_static! {

    // Regex Strings
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"[ \n\t\s]+").unwrap();
    static ref LET_REGEX: Regex =        Regex::new(r"let\b").unwrap();
    static ref COLON_REGEX: Regex =      Regex::new(r":").unwrap();
    static ref FINAL_REGEX: Regex =      Regex::new(r"final\b").unwrap();
    static ref EQUALS_REGEX: Regex =     Regex::new(r"=").unwrap();
    static ref ID_REGEX: Regex =         Regex::new(r"([A-Za-z][A-Za-z0-9_]*)\b").unwrap();
    static ref INT_REGEX: Regex =        Regex::new(r"(-?[0-9]+)\b").unwrap();
}

/// Tokenize the input file into a vector of tokens
/// TODO: can we "stream" the file in, and "stream" the tokens out?
pub fn tokenize(file_string: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Keeps track of the index that has been searched up to already. This allows us to consume tokens iteratively.
    let mut current_position = 0;

    while current_position < file_string.len() {
      let next_token: Option<Token> = find_next_token(&file_string, &mut current_position);

      if next_token.is_some() {
        tokens.push(next_token.unwrap())
      }
    }

    print!("{:?}", tokens);
    return tokens;
}

/// The function will find and "consume" the next token from the search_position.
/// search_position is the address of an integer that describes where to consume the next token from.
/// It will return an Option<Token>, and "consume" the token by advancing the integer at search_position.
fn find_next_token(file_string: &String, search_position: &mut usize) -> Option<Token> {
  let mut matched_text: Option<String> = None;

  if is_match_at(file_string, search_position, &WHITESPACE_REGEX, &mut matched_text) {
    return None;
  }
  else if is_match_at(file_string, search_position, &LET_REGEX, &mut matched_text) {
    return Some(Token::Let);
  }
  else if is_match_at(file_string, search_position, &COLON_REGEX, &mut matched_text) {
    return Some(Token::Colon);
  }
  else if is_match_at(file_string, search_position, &FINAL_REGEX, &mut matched_text) {
    return Some(Token::Final);
  }
  else if is_match_at(file_string, search_position, &EQUALS_REGEX, &mut matched_text) {
    return Some(Token::Equals);
  }
  else if is_match_at(file_string, search_position, &ID_REGEX, &mut matched_text) {
    return Some(Token::Id(matched_text.unwrap()));
  }
  else if is_match_at(file_string, search_position, &INT_REGEX, &mut matched_text) {
    return Some(Token::Int(matched_text.unwrap().parse::<i32>().unwrap()));
  }
  else {
    println!("Invalid syntax at ch:{}.\n{}", search_position, file_string);
    exit(2)
  }
}

fn is_match_at(file_string: &String, search_position: &mut usize, regex: &Regex, matched_text: &mut Option<String>) -> bool {
  let regex_match: Option<regex::Match> = regex.find_at(file_string, *search_position);

  if regex_match.is_some() && regex_match.unwrap().start() == *search_position {
    *matched_text = Some(regex_match.unwrap().as_str().to_string());
    *search_position = regex_match.unwrap().end();
    return true;
  }
  return false;
}
