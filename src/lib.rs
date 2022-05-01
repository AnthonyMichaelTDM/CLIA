//! # CLI Argument Parser Library
//! 
//! A crate with tools for parsing command line arguments
//! 
//! As far as this crate is concerned, there are 4 types of arguments, in 2 main groups
//! 
//! Options:
//! - flags (ei. -r)
//! - flags w/ lists (ei -f <comma separated list> )
//! - flags w/ data (ei --format=NUMERIC)
//! 
//! and Others:
//! - (ei a file path, a string, etc.)
//! 
//! ### 
//! 
//! This crate makes the following assumptions about your command line program:
//! - that all options / flags start with a `-`
//! - that lists entered in the command line are comma separated
//! - options and their associated bits of data, are typed before any other arguments
//! - any and all "Others" are required, and must be included in the arguments for your program to work properly (optional arguments should be tied to flags anyway)

/// utilities for defining options
pub mod option_args;
/// utilities for parsing options
pub mod option_parser;
/// utilities for defining other arguments (ei a file path, a string, etc.)
pub mod other_args;
/// utilities for parsing others
pub mod other_parser;


/*
things that need to be done:

framework for user to define their flags and whatnot, and generate help docs

anything not used as an option or data attached to an option should be extracted as a OtherArgument 
*/
/*
what we need:
to generate a config based on some arguments
to generate Help documentation

to generate a config

there are 4 types of arguments, in 2 main groups
options:
    flags (ei. -r)
    flags w/ lists (ei -f <comma separated list> )
    flags w/ data (ei --format=NUMERIC)
and Others
    (ei a file path, a string, etc.)

*/
/*
how this should be used:

create a list of valid flags, 

call a function to create a new Parser, passing it the list of valid options and the arguments extracted from the command line with ```env::args().collect();```

call a getter function to get a list of the options (and their associated data) found in the arguments, as well as other arguments
*/

use std::error::Error;

pub struct Parser {
    args: Vec<String>,
    valid_options: Vec<option_args::ClOption>,
    valid_others: Vec<other_args::ClOther>,
    option_arguments_found: Vec<option_args::ClOption>,
    other_arguments_found: Vec<String>,
}
impl Parser {
    /// create a new Parser, and parses the specified `args`
    /// 
    /// 
    fn new(args: &[String], valid_options: &[option_args::ClOption], valid_others: &[other_args::ClOther]) -> Result<Parser, Box<dyn Error>> {
        //DATA
        let mut parser = Parser {
            args: args.into(),
            valid_options: Vec::from(valid_options),
            valid_others: Vec::from(valid_others),
            option_arguments_found: Vec::new(),
            other_arguments_found: Vec::new(),
        };

        //parse for valid options
        parser.option_arguments_found = match option_parser::parse_for_options(args, &parser.valid_options) {
            Ok(options) => options,
            Err(e) => return Err(e),
        };

        //parse for other arguments



        //return
        return Ok(parser);
    } 

    /// returns a string containing help documentation for your command line program, which you can then print
    /// 
    /// here's the format:
    /// ```text
    /// {title (the name of the compiled .exe)}
    /// {author}
    /// 
    /// {program description}
    /// 
    /// USAGE: {title} [OPTIONS] {the other arguments}
    /// 
    /// OPTIONS:
    /// {help lines for every option}
    /// 
    /// OTHER ARGUMENTS:
    /// {help lines for other arguments}
    /// 
    /// ```
    fn help(&self, title: &str, author: &str, program_description: &str) -> String {
        format!("{}\n{}\n\n{}\nUSAGE: {} [OPTIONS] {}\n\nOPTIONS:\n{}\n\nOTHER ARGUMENTS:\n{}",
            title,
            author,
            program_description,
            title,
            {""},
            {
                let mut option_help: String = String::new();
                for option in self.valid_options.iter() {
                    option_help += &option.gen_help_line();
                    option_help += "\n";
                }
                option_help
            },
            {""},
        )
    }
}