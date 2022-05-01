//! # CLI Argument Parser Library
//! 
//! A crate with tools for parsing command line arguments
//! 
//! As far as this crate is concerned, there are 4 types of arguments, in 2 main groups
//! 
//! Options:
//! - flags (ei. `-r`)
//! - flags w/ lists (ei `-f [comma separated list]` )
//! - flags w/ data (ei `--format <NUMERIC>`)
//! 
//! and Parameters:
//! - (ei a file path, a string, etc.)
//! 
//! ### 
//! 
//! This crate makes the following assumptions about your command line program:
//! - that all options / flags start with a `-`
//! - that lists entered in the command line are comma separated
//! - options and their associated bits of data, are typed before any parameter arguments
//! - any and all "Parameters" are required, and must be included in the arguments for your program to work properly (optional arguments should be tied to flags anyway)

/// utilities for defining options
pub mod option_args;
/// utilities for parsing options
pub mod option_parser;
/// utilities for defining parameter arguments (ei a file path, a string, etc.)
pub mod parameter_args;
/// utilities for parsing parameters
pub mod parameter_parser;


/*
things that need to be done:

framework for user to define their flags and whatnot, and generate help docs

anything not used as an option or data attached to an option should be extracted as a ParameterArgument 
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
and Parameters
    (ei a file path, a string, etc.)

*/
/*
how this should be used:

create a list of valid flags, 

call a function to create a new Parser, passing it the list of valid options and the arguments extracted from the command line with ```env::args().collect();```

call a getter function to get a list of the options (and their associated data) found in the arguments, as well as parameter arguments
*/

use std::error::Error;

pub struct Parser {
    valid_options: Vec<option_args::ClOption>,
    expected_parameters: Vec<parameter_args::ClParameter>,
    option_arguments_found: Vec<option_args::ClOption>,
    parameter_arguments_found: Vec<parameter_args::ClParameter>,
}
impl Parser {
    /// create a new Parser, and parses the specified `args`
    /// 
    /// # Examples
    /// ```
    /// use std::env; //allows access to the process's environment
    /// 
    /// use clia;
    /// 
    /// //collect cli arguments
    /// let args: Vec<String> = env::args().collect();
    /// 
    /// //define valid options
    /// let valid_options: Vec<clia::option_args::ClOption> = Vec::new();
    /// //...
    /// 
    /// //define expected parameters
    /// let expected_parameters: Vec<clia::parameter_args::ClParameter> = Vec::new();
    /// //..
    /// 
    /// //create a new parser
    /// let parser = clia::Parser::new(&args, &valid_options, &parameter_args);
    /// ```
    pub fn new(args: &[String], valid_options: &[option_args::ClOption], expected_parameters: &[parameter_args::ClParameter]) -> Result<Parser, Box<dyn Error>> {
        //DATA
        let mut parser = Parser {
            valid_options: Vec::from(valid_options),
            expected_parameters: Vec::from(expected_parameters),
            option_arguments_found: Vec::new(),
            parameter_arguments_found: Vec::new(),
        };

        //parse for valid options
        parser.option_arguments_found = match option_parser::parse_for_options(args, &parser.valid_options) {
            Ok(options) => options,
            Err(e) => return Err(e),
        };

        //parse for parameter arguments
        parser.parameter_arguments_found = match parameter_parser::parse_for_parameters(args, &parser.expected_parameters) {
            Ok(parameters) => parameters,
            Err(e) => return Err(e),
        };

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
    /// USAGE: {title} [OPTIONS] {the parameter arguments}
    /// 
    /// OPTIONS:
    /// {help lines for every option}
    /// 
    /// PARAMETER ARGUMENTS:
    /// {help lines for parameter arguments}
    /// 
    /// ```
    /// 
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn help(title: &str, author: &str, program_description: &str, valid_options: &[option_args::ClOption], expected_parameters: &[parameter_args::ClParameter]) -> String {
        format!("{}\n{}\n\n{}\n\nUSAGE: {} [OPTIONS]... {}\n\nOPTIONS:\n{}\nPARAMETER ARGUMENTS:\n{}",
            title,
            author,
            program_description,
            title,
            {
                let mut param_usage: String = String::new();
                for parameter in expected_parameters.into_iter() {
                    param_usage += format!("[{}] ",parameter.get_name()).as_str();
                }
                param_usage
            },
            {
                let mut option_help: String = String::new();
                for option in valid_options.iter() {
                    option_help += &option.gen_help_line();
                    option_help += "\n";
                }
                option_help
            },
            {
                let mut parameter_help: String = String::new();
                for option in expected_parameters.iter() {
                    parameter_help += &option.gen_help_line();
                    parameter_help += "\n";
                }
                parameter_help
            },
        )
    }

    //getter methods
    /// get a copy of `valid_options`
    /// # Example 
    /// ```
    /// 
    /// ```
    pub fn get_valid_options(&self) -> Vec<option_args::ClOption> {self.valid_options.to_owned()}

    /// get a copy of `expected_parameters`
    /// # Example 
    /// ```
    /// 
    /// ```
    pub fn get_expected_parameters(&self) -> Vec<parameter_args::ClParameter> {self.expected_parameters.to_owned()}

    /// get a copy of `option_arguments_found`
    /// # Example 
    /// ```
    /// 
    /// ```
    pub fn get_option_arguments_found(&self) -> Vec<option_args::ClOption> {self.option_arguments_found.to_owned()}

    /// get a copy of `parameter_arguments_found`
    /// # Example 
    /// ```
    /// 
    /// ```
    pub fn get_parameter_arguments_found(&self) -> Vec<parameter_args::ClParameter> {self.parameter_arguments_found.to_owned()}
    
}