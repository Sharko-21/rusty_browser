use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::rc::Rc;
use std::collections::{HashMap};
use std::cell::{RefCell, Ref};
use std::cmp::Ordering;
use std::ops::Add;

use crate::dom;

const LEX_TAG_OPENER: char = '<';
const LEX_TAG_CLOSER: char = '>';
const LEX_CLOSING_TAG_MARKER: char = '/';
const LEX_ATTR_APPROPRIATION: char = '=';
const LEX_ATTR_VALUE_SINGLE_QUOTE: char = '\'';
const LEX_ATTR_VALUE_DOUBLE_QUOTE: char = '\"';

pub fn read_file(path: String) -> std::io::Result<()> {
    let file = File::open(&path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    //it's temple call
    parse_string_to_dom(contents.as_str());
    Ok(())
}

pub fn parse_string_to_dom(body: &str) -> Rc<RefCell<dom::DOM>> {
    //let lex_stack = vec![];
    tokenize(body);
    dom::DOM::new(dom::DomElem::None)
}

pub fn tokenize(body: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut token: String = String::new();
    for character in body.chars() {
        if character == '\n'{
            if token.chars().count() > 0 {
                tokens.push(token);
                token = String::new();
            }
            continue
        }
        if character == ' '{
            if !tokens[tokens.len() - 2].eq(&LEX_ATTR_VALUE_SINGLE_QUOTE.to_string()) && tokens.last().unwrap() == &LEX_ATTR_VALUE_SINGLE_QUOTE.to_string() {
                let string_character = character.to_string();
                token = token.add(string_character.as_ref());
                continue
            }

            if !tokens[tokens.len() - 2].eq(&LEX_ATTR_VALUE_DOUBLE_QUOTE.to_string()) && tokens.last().unwrap() == &LEX_ATTR_VALUE_DOUBLE_QUOTE.to_string() {
                let string_character = character.to_string();
                token = token.add(string_character.as_ref());
                continue
            }

            if token.chars().count() > 1 {
                let string_character = character.to_string();
                tokens.push(token);
                token = String::new();
                continue
            }
        } else if character == LEX_TAG_OPENER || character == LEX_TAG_CLOSER || character == LEX_CLOSING_TAG_MARKER || character == LEX_ATTR_APPROPRIATION || character == LEX_ATTR_VALUE_SINGLE_QUOTE || character == LEX_ATTR_VALUE_DOUBLE_QUOTE {
            if token != "".to_string() {
                tokens.push(token);
                token = String::new();
            }
            let string_character = character.to_string();
            token = token.add(string_character.as_ref());
            tokens.push(token);
            token = String::new();
        } else {
            let string_character = character.to_string();
            token = token.add(string_character.as_ref());
        }
    }
    return tokens;
}