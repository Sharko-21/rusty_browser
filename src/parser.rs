use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::rc::Rc;
use std::collections::{HashMap};
use std::cell::{RefCell, Ref};
use std::cmp::Ordering;
use std::ops::Add;

use crate::dom;

const LEX_TAG_OPENER:               char = '<';
const LEX_TAG_CLOSER:               char = '>';
const LEX_CLOSING_TAG_MARKER:       char = '/';
const LEX_ATTR_APPROPRIATION:       char = '=';
const LEX_ATTR_VALUE_SINGLE_QUOTE:  char = '\'';
const LEX_ATTR_VALUE_DOUBLE_QUOTE:  char = '\"';


const TEXT_MODE_OFF:                i32 = 0;
const TEXT_SINGLE_QUOTE_MODE_ON:    i32 = 1;
const TEXT_DOUBLE_QUOTE_MODE_ON:    i32 = 2;
const TEXT_BETWEEN_TAGS_MODE_ON:    i32 = 3;

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
    let mut i = -1;
    let mut text_mode = TEXT_MODE_OFF;
    for character in body.trim().chars() {
        i += 1;
        if character == '\n'{
            if token.chars().count() > 0 {
                tokens.push(token);
                token = String::new();
            }
            continue
        }
        if (text_mode == TEXT_DOUBLE_QUOTE_MODE_ON || text_mode == TEXT_SINGLE_QUOTE_MODE_ON || text_mode == TEXT_BETWEEN_TAGS_MODE_ON) {
            if text_mode == TEXT_DOUBLE_QUOTE_MODE_ON && character == LEX_ATTR_VALUE_DOUBLE_QUOTE {
                text_mode = TEXT_MODE_OFF;
            } else if text_mode == TEXT_SINGLE_QUOTE_MODE_ON && character == LEX_ATTR_VALUE_SINGLE_QUOTE {
                text_mode = TEXT_MODE_OFF;
            } else if text_mode == TEXT_BETWEEN_TAGS_MODE_ON && character == LEX_TAG_OPENER {
                text_mode = TEXT_MODE_OFF;
            }
            if text_mode == TEXT_MODE_OFF {
                if token != "".to_string() {
                    tokens.push(token);
                    token = String::new();
                }
            }
            if character != ' ' {
                let string_character = character.to_string();
                token = token.add(string_character.as_ref());
            }
            continue
        }
        if character == ' ' {
            if token != "".to_string() {
                tokens.push(token);
                token = String::new();
            }
            continue
        } else if character == LEX_TAG_OPENER || character == LEX_TAG_CLOSER || character == LEX_CLOSING_TAG_MARKER || character == LEX_ATTR_APPROPRIATION || character == LEX_ATTR_VALUE_SINGLE_QUOTE || character == LEX_ATTR_VALUE_DOUBLE_QUOTE {
            if text_mode == TEXT_MODE_OFF {
                if character == LEX_ATTR_VALUE_SINGLE_QUOTE {
                    text_mode = TEXT_SINGLE_QUOTE_MODE_ON;
                } else if character == LEX_ATTR_VALUE_DOUBLE_QUOTE {
                    text_mode = TEXT_DOUBLE_QUOTE_MODE_ON;
                } else if character == LEX_TAG_CLOSER {
                    text_mode = TEXT_BETWEEN_TAGS_MODE_ON;
                }
            }
            let string_character = character.to_string();
            if token == LEX_TAG_OPENER.to_string() {
                if string_character == LEX_CLOSING_TAG_MARKER.to_string() {
                    token = token.add(string_character.as_ref());
                    tokens.push(token);
                    token = String::new();
                    continue;
                }
            }
            if string_character == LEX_TAG_OPENER.to_string() {
                if token != "".to_string() {
                    tokens.push(token);
                    token = String::new();
                };
                token = token.add(string_character.as_ref());
                continue;
            }
            if token != "".to_string() {
                tokens.push(token);
                token = String::new();
            };
            token = token.add(string_character.as_ref());
            tokens.push(token);
            token = String::new();
        } else {
            let string_character = character.to_string();
            if token == LEX_TAG_OPENER.to_string() {
                tokens.push(token);
                token = String::new();
            }
            token = token.add(string_character.as_ref());
        }
    }
    return tokens;
}