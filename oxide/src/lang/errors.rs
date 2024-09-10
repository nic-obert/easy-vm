use std::io;
use std::cmp::min;

use colored::Colorize;
use indoc::printdoc;



/// Number of lines of source code to include before and after the highlighted line in error messages
const SOURCE_CONTEXT_RADIUS: u8 = 4;


/// Print the source code context around the specified line.
fn print_source_context(source: &[&str], line_index: usize, char_pointer: usize) {

    // Calculate the beginning of the context. Saturating subtraction is used interpret underflow as 0.
    let mut index = line_index.saturating_sub(SOURCE_CONTEXT_RADIUS as usize);
    let end_index = min(line_index + SOURCE_CONTEXT_RADIUS as usize + 1, source.len());

    let line_number_width = end_index.to_string().len();

    // Print the source lines before the highlighted line.
    while index < line_index {
        println!(" {:line_number_width$}  {}", index + 1, source[index]);
        index += 1;
    }

    // The highlighted line.
    println!("{}{:line_number_width$}  {}", ">".bright_red().bold(), index + 1, source[line_index]);
    println!(" {:line_number_width$} {:>char_pointer$}{}", "", "", "^".bright_red().bold());
    index += 1;

    // Lines after the highlighted line.
    while index < end_index {
        println!(" {:line_number_width$}  {}", index + 1, source[index]);
        index += 1;
    }
}


pub fn io_error(error: io::Error, hint: &str) -> ! {
    printdoc!("
        IO Error
        {}

        Hint:
        {}
    ",
        error, hint
    );

    std::process::exit(1);
}
