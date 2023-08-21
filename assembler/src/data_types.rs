use std::path::Path;

use rust_vm_lib::assembly::ByteCode;

use crate::error;
use crate::tokenizer::evaluate_string;


/// Represents a data type in static data declarations
pub enum DataType {

    String,
    Char,
    Unsigned1,
    Unsigned2,
    Unsigned4,
    Unsigned8,
    Signed1,
    Signed2,
    Signed4,
    Signed8,
    Array(Box<DataType>),
    
}


impl DataType {

    /// Returns a data type from its string name
    pub fn from_name(name: &str) -> Option<Self> {

        // Check if the data type is an array
        if let Some(name) = name.strip_prefix('[')
            .and_then(|name| name.strip_suffix(']')).map(|name| name.trim())
        {   
            // Recursively get the data type of the array item
            let item_type = Self::from_name(name)?;
            return Some(DataType::Array(Box::new(item_type)));
        }

        match name {

            "string" => Some(DataType::String),
            "char" => Some(DataType::Char),
            "u1" => Some(DataType::Unsigned1),
            "u2" => Some(DataType::Unsigned2),
            "u4" => Some(DataType::Unsigned4),
            "u8" => Some(DataType::Unsigned8),  
            "i1" => Some(DataType::Signed1),
            "i2" => Some(DataType::Signed2),
            "i4" => Some(DataType::Signed4),
            "i8" => Some(DataType::Signed8),

            _ => None,
        }
    }


    /// Encodes a string into a byte code vector based on the data type
    pub fn encode(&self, string: &str, line_number: usize, line: &str, unit_path: &Path) -> ByteCode {

        match self {

            DataType::Char => {

                // Remove the enclosing single quotes
                let string = string.strip_prefix('\'').and_then(|string| string.strip_suffix('\'')).unwrap_or_else(
                    || error::invalid_data_declaration(unit_path, line_number, line, format!("Expected a character literal. Got \"{}\"", string).as_str())
                );

                // Evaluate the string by escaping characters
                let evaluated_string = evaluate_string(string, '\'', line_number, line, unit_path);

                // Check if the character literal is only one character long
                if evaluated_string.len() != 1 {
                    error::invalid_data_declaration(unit_path, line_number, line, "Character literals must be exactly one character long.");
                }

                // Return the byte representation of the character assuming the string is only one character long
                evaluated_string.into_bytes()
            },

            DataType::String => {

                // Remove the enclosing double quotes
                let string = string.strip_prefix('"').unwrap_or_else(
                    || error::invalid_data_declaration(unit_path, line_number, line, format!("Expected a string literal. Got \"{}\"", string).as_str())
                ).strip_suffix('"').unwrap_or_else(
                    || error::invalid_data_declaration(unit_path, line_number, line, format!("Expected a string literal. Got \"{}\"", string).as_str())
                );

                // Return the evaluated and encoded string
                evaluate_string(string, '"', line_number, line, unit_path).into_bytes()
            },

            DataType::Unsigned1 => {
                let number = string.parse::<u8>().unwrap_or_else(
                    |_| error::invalid_data_declaration(unit_path, line_number, line, format!("Expected an unsigned 1 byte integer. Got \"{}\"", string).as_str())
                );
                vec![number]
            },

            DataType::Unsigned2 => {
                let number = string.parse::<u16>().unwrap_or_else(
                    |_| error::invalid_data_declaration(unit_path, line_number, line, format!("Expected an unsigned 2 byte integer. Got \"{}\"", string).as_str())
                );
                vec![number as u8, (number >> 8) as u8]
            },

            DataType::Unsigned4 => {
                let number = string.parse::<u32>().unwrap_or_else(
                    |_| error::invalid_data_declaration(unit_path, line_number, line, format!("Expected an unsigned 4 byte integer. Got \"{}\"", string).as_str())
                );
                vec![number as u8, (number >> 8) as u8, (number >> 16) as u8, (number >> 24) as u8]
            },

            DataType::Unsigned8 => {
                let number = string.parse::<u64>().unwrap_or_else(
                    |_| error::invalid_data_declaration(unit_path, line_number, line, format!("Expected an unsigned 8 byte integer. Got \"{}\"", string).as_str())
                );
                vec![number as u8, (number >> 8) as u8, (number >> 16) as u8, (number >> 24) as u8, (number >> 32) as u8, (number >> 40) as u8, (number >> 48) as u8, (number >> 56) as u8]
            }

            DataType::Signed1 => {
                let number = string.parse::<i8>().unwrap_or_else(
                    |_| error::invalid_data_declaration(unit_path, line_number, line, format!("Expected a signed 1 byte integer. Got \"{}\"", string).as_str())
                );
                vec![number as u8]
            },

            DataType::Signed2 => {
                let number = string.parse::<i16>().unwrap_or_else(
                    |_| error::invalid_data_declaration(unit_path, line_number, line, format!("Expected a signed 2 byte integer. Got \"{}\"", string).as_str())
                );
                vec![number as u8, (number >> 8) as u8]
            },

            DataType::Signed4 => {
                let number = string.parse::<i32>().unwrap_or_else(
                    |_| error::invalid_data_declaration(unit_path, line_number, line, format!("Expected a signed 4 byte integer. Got \"{}\"", string).as_str())
                );
                vec![number as u8, (number >> 8) as u8, (number >> 16) as u8, (number >> 24) as u8]
            },

            DataType::Signed8 => {
                let number = string.parse::<i64>().unwrap_or_else(
                    |_| error::invalid_data_declaration(unit_path, line_number, line, format!("Expected a signed 8 byte integer. Got \"{}\"", string).as_str())
                );
                vec![number as u8, (number >> 8) as u8, (number >> 16) as u8, (number >> 24) as u8, (number >> 32) as u8, (number >> 40) as u8, (number >> 48) as u8, (number >> 56) as u8]
            },

            DataType::Array(item_type) => {

                let data_string = string.strip_prefix('[').and_then(|data| data.strip_suffix(']')).unwrap_or_else(
                    || error::invalid_data_declaration(unit_path, line_number, line, format!("Expected an array literal. Got \"{}\"", string).as_str())
                );

                let mut byte_code = ByteCode::new();

                match item_type.as_ref() {

                    DataType::String => {
                        for string in iter_strings(data_string) {
                            match string {
                                Ok(string) => {
                                    byte_code.extend(item_type.encode(string, line_number, line, unit_path));
                                },
                                Err(error) => error::invalid_data_declaration(unit_path, line_number, line, error.as_str())
                            }
                        }
                    },

                    DataType::Char => {
                        for char_string in iter_chars(data_string) {
                            match char_string {
                                Ok(char_string) => {
                                    byte_code.extend(item_type.encode(char_string, line_number, line, unit_path));
                                },
                                Err(error) => error::invalid_data_declaration(unit_path, line_number, line, error.as_str())
                            }
                        }
                    },

                    DataType::Array(_) => {
                        for array_string in iter_arrays(data_string) {
                            match array_string {
                                Ok(array_string) => {
                                    byte_code.extend(item_type.encode(array_string, line_number, line, unit_path));
                                },
                                Err(error) => error::invalid_data_declaration(unit_path, line_number, line, error.as_str())
                            }
                        }
                    },

                    _ => {
                        for item in data_string.split(',') {
                            byte_code.extend(item_type.encode(item.trim(), line_number, line, unit_path));
                        }
                    }
                }

                byte_code
            }

        }

    }

}


struct StringDataIterator<'a> {
    string: &'a str,
    index: usize,
    string_delimiter: char,
}


impl<'a> Iterator for StringDataIterator<'a> {
    type Item = Result<&'a str, String>;

    fn next(&mut self) -> Option<Self::Item> {

        let mut start_index = 0;
        let mut in_string = false;
        let mut escape = false;
        let mut data_string: Option<&'a str> = None;

        for c in self.string.chars() {

            if in_string {

                if escape {

                    escape = false;

                } else if c == self.string_delimiter {
                    in_string = false;
                    data_string = Some(&self.string[start_index..=self.index]);

                } else if c == '\\' {
                    escape = true;
                }

            } else {
                // Not in_string

                if c == self.string_delimiter {
                    if data_string.is_some() {
                        return Some(Err(format!("Expected a comma after \"{}\"", data_string.unwrap())));
                    }

                    in_string = true;
                    start_index = self.index;

                } else {

                    match c {

                        ',' => {
                            if data_string.is_none() {
                                return Some(Err("Expected a string or character literal".to_string()));
                            }
    
                            self.string = &self.string[self.index + 1..];
                            self.index = 0;
                            return Some(Ok(data_string.unwrap()));
                        },

                        ' ' | '\t' => {},
    
                        _ => {
                            if data_string.is_some() {
                                return Some(Err(format!("Expected a comma after \"{}\"", data_string.unwrap())));
                            }
                            return Some(Err(format!("Unexpected character \"{}\"", c)));
                        }
    
                    }

                }
                
            }

            self.index += 1;
        }

        if let Some(data_string) = data_string {
            self.string = "";
            return Some(Ok(data_string));
        }

        None
    }

}


fn iter_chars(string: &str) -> StringDataIterator<'_> {
    StringDataIterator {
        string,
        index: 0,
        string_delimiter: '\'',
    }
}


fn iter_strings(string: &str) -> StringDataIterator<'_> {
    StringDataIterator {
        string,
        index: 0,
        string_delimiter: '"',
    }
}


struct ArrayDataIterator<'a> {
    string: &'a str,
    index: usize,
}


impl<'a> Iterator for ArrayDataIterator<'a> {
    type Item = Result<&'a str, String>;

    fn next(&mut self) -> Option<Self::Item> {

        enum TextType {
            String,
            Char,
            Array,
            None,
        }
        
        let mut text_type = TextType::None;
        let mut string_escape = false;
        let mut array_depth: usize = 0;
        let mut start_index = 0;
        let mut data_string: Option<&'a str> = None;

        for c in self.string.chars() {

            match text_type {

                TextType::Array => match c {

                    '[' => array_depth += 1,

                    ']' => {
                        array_depth -= 1;
                        if array_depth == 0 {
                            text_type = TextType::None;
                            data_string = Some(&self.string[start_index..self.index]);
                        }
                    },

                    _ => {}

                },

                TextType::None => match c {

                    '[' => {
                        if data_string.is_some() {
                            return Some(Err(format!("Expected a comma after \"{}\"", data_string.unwrap())));
                        }

                        text_type = TextType::Array;
                        start_index = self.index;
                        array_depth = 1;
                    },

                    ' ' | '\t' => {},

                    ',' => {
                        if data_string.is_none() {
                            return Some(Err("Expected an array literal".to_string()));
                        }

                        self.string = &self.string[self.index + 1..];
                        self.index = 0;
                        return Some(Ok(data_string.unwrap()));
                    },

                    '\'' => {
                        if data_string.is_some() {
                            return Some(Err(format!("Expected a comma after \"{}\"", data_string.unwrap())));
                        }

                        text_type = TextType::Char;
                    },

                    '"' => {
                        if data_string.is_some() {
                            return Some(Err(format!("Expected a comma after \"{}\"", data_string.unwrap())));
                        }

                        text_type = TextType::String;
                    },

                    _ => {
                        if data_string.is_some() {
                            return Some(Err(format!("Expected a comma after \"{}\"", data_string.unwrap())));
                        }
                        return Some(Err(format!("Unexpected character \"{}\"", c)));
                    }
                    
                },

                TextType::String => {
                        
                    if string_escape {

                        string_escape = false;

                    } else if c == '"' {
                        text_type = TextType::None;
                        data_string = Some(&self.string[start_index..self.index]);

                    } else if c == '\\' {
                        string_escape = true;
                    }

                },

                TextType::Char => {

                    if string_escape {

                        string_escape = false;

                    } else if c == '\'' {
                        text_type = TextType::None;
                        data_string = Some(&self.string[start_index..self.index]);

                    } else if c == '\\' {
                        string_escape = true;
                    }

                },
            }

            self.index += 1;
        }

        None
    }
}


fn iter_arrays(string: &str) -> ArrayDataIterator<'_> {
    ArrayDataIterator {
        string,
        index: 0,
    }
}

