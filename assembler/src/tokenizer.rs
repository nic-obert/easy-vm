use rust_vm_lib::registers::get_register;
use rust_vm_lib::token::{Token, TokenValue};
use std::mem;


fn is_name_character(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}


pub fn tokenize_operands(mut operands: String, line_number: usize, line: &str) -> Vec<Token> {

    let mut tokens: Vec<Token> = Vec::new();

    // Add a semicolon at the end in order to make the loop iterate one more time for simplicity
    operands.push(';');

    let mut current_token: Option<Token> = None;

    for c in operands.chars() {
        
        if let Some(token) = &mut current_token {

            match &mut token.value {
                TokenValue::AddressGeneric(_value) => {
                    if c.is_digit(10) {
                        current_token = Some(
                            Token::new(TokenValue::AddressLiteral(c.to_digit(10).unwrap() as usize))
                        );
                    }
                    else if is_name_character(c) {
                        current_token = Some(
                            Token::new(TokenValue::AddressInRegisterIncomplete(c.to_string()))
                        );
                    } else if c == ']' {
                        panic!("Empty address at line {}:\n{}", line_number, line);
                    } else {
                        panic!("Invalid character \"{}\" in token AddressGeneric at line {}:\n{}", c, line_number, line);
                    }

                    continue;
                },
                
                TokenValue::AddressLiteral(value) => {
                    if c.is_digit(10) {
                        *value = *value * 10 + c.to_digit(10).unwrap() as usize;
                    } else if c == ']' {
                        tokens.push(current_token.take().unwrap());                    
                    } else {
                        panic!("Invalid character \"{}\" in token AddressLiteral at line {}:\n{}", c, line_number, line);
                    }

                    continue;
                },

                TokenValue::AddressInRegisterIncomplete(value) => {
                    if is_name_character(c) {
                        value.push(c);
                        continue;
                    }

                    if c != ']' {
                        panic!("Expected ']' after address in argument list \"{}\", but '{}' was provided at line {}:\n{}", operands, c, line_number, line);
                    }

                    if let Some(register) = get_register(value) {
                        tokens.push(Token::new(TokenValue::AddressInRegister(register)));
                        current_token = None;
                    } else {
                        panic!("Unknown register \"{}\" in argument list \"{}\" at line {}:\n{}", value, operands, line_number, line);
                    }

                    continue;
                },

                TokenValue::Name(value) => {
                    if is_name_character(c) {
                        value.push(c);
                        continue;
                    }
                    if c == ':' {
                        tokens.push(Token::new(TokenValue::Label(mem::take(value))));
                        current_token = None;
                        continue;
                    }

                    if let Some(register) = get_register(&value) {
                        tokens.push(Token::new(TokenValue::Register(register)));
                        current_token = None;
                    }
                    else {
                        tokens.push(current_token.take().unwrap());
                    }
                }
                   
                TokenValue::Number(value) => {
                    if c.is_digit(10) {
                        *value = *value * 10 + c.to_digit(10).unwrap() as i64;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                },
                
                _ => { }
                
            }
        }


        if is_name_character(c) {
            current_token = Some(Token::new(TokenValue::Name(c.to_string())));
            continue;
        }

        if c.is_digit(10) {
            current_token = Some(Token::new(TokenValue::Number(c.to_digit(10).unwrap() as i64)));
            continue;
        }

        match c {
            ' ' | '\t' => continue,

            ';' => break,

            '[' => {
                current_token = Some(Token::new(TokenValue::AddressGeneric(0)));
                continue;
            },

            _ => panic!("Unhandled character '{}' at line {}, in operands \"{}\":\n{}", c, line_number, operands, line)
        }

    }   

    tokens
}


