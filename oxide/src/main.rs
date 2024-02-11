mod operations;
mod token;
mod data_types;
mod tokenizer;
mod cli_parser;
mod files;
mod error;
mod ast;
mod symbol_table;
mod token_tree;
mod utils;
mod icr;
mod function_parser;

use std::path::Path;

use clap::Parser;

use crate::cli_parser::CliParser;


fn main() {
    
    let args = CliParser::parse();

    let input_file = Path::new(&args.input_file);

    let source = match files::load_ir_code(input_file) {
        Ok(source) => source,
        Err(e) => {
            println!("Could not open file {}\n{}", input_file.display(), e);
            std::process::exit(1);
        }
    };

    let mut symbol_table = symbol_table::SymbolTable::new();

    let tokens = tokenizer::tokenize(&source, input_file, &mut symbol_table);

    let ast = ast::build_ast(tokens, &source, &mut symbol_table);

    let functions = function_parser::parse_functions(ast, args.optimize, &mut symbol_table, &source);

    let _ir_code = icr::generate(functions, &mut symbol_table, &source);

}

