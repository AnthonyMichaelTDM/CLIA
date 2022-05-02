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

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

/// utilities for defining options
pub mod option_args;
/// utilities for parsing options
pub mod option_parser;
/// utilities for defining parameter arguments (ei a file path, a string, etc.)
pub mod parameter_args;
/// utilities for parsing parameters
pub mod parameter_parser;

use std::error::Error;

/// concentates option and parameter parsing into one place
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
    /// 
    /// ```
    /// use std::env;
    /// use clia::{option_args, parameter_args, Parser};
    /// //...
    /// 
    ///     //collect cli arguments
    ///     let args: Vec<String> = env::args().collect();
    ///     
    ///     //define valid options
    ///     let mut valid_options: Vec<option_args::ClOption> = Vec::new();
    ///     //...
    ///     
    ///     //define expected parameters
    ///     let mut expected_parameters: Vec<parameter_args::ClParameter> = Vec::new();
    ///     //...
    ///     
    ///     //create a new parser
    ///     let parser = Parser::new(&args, &valid_options, &expected_parameters);
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
    /// use std::env;
    /// use clia::{option_args::{ClOption, ClOptionInfo}, parameter_args::ClParameter, Parser};
    /// //...
    /// 
    ///     //define valid options
    ///     let mut valid_options: Vec<ClOption> = Vec::new();
    ///     //...
    ///     
    ///     //define expected parameters
    ///     let mut expected_parameters: Vec<ClParameter> = Vec::new();
    ///     //..
    ///     
    ///     //to print help message
    ///     println!("{}", Parser::help("foo.exe", "by Anthony Rubick", "Just here as an example of things you can do", &valid_options, &expected_parameters));
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
    /// get a reference to `valid_options`
    /// # Examples 
    /// ```
    /// use std::env;
    /// use clia::{option_args::{ClOptionInfo, ClOption}, parameter_args, Parser};
    /// //...
    ///     //collect cli arguments
    ///     let args: Vec<String> = env::args().collect();
    ///     # let args: Vec<String> = vec![String::from("path/to/executable/"), String::from("-h")];
    /// 
    ///     //define valid options
    ///     let valid_options: Vec<ClOption> = Vec::new();
    ///     //...
    ///     # let valid_options: Vec<ClOption> = vec![
    ///     #     ClOption::new_flag(&ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap()),
    ///     #     ClOption::new_flag_list( &ClOptionInfo::new("-f", "--filter", "Comma separated list of extensions, will only count lines of files with these extensions").unwrap(), "EXTENSIONS"),
    ///     #     ClOption::new_flag_list( &ClOptionInfo::new("-F", "--format", "Format the output in a list, valid formats are: DEFAULT, BULLET, MARKDOWN, and NUMERIC").unwrap(), "FORMAT"),
    ///     #     ClOption::new_flag(&ClOptionInfo::new("-h", "--help", "Show help").unwrap()),
    ///     # ];
    ///     
    ///     //define expected parameters
    ///     let expected_parameters: Vec<parameter_args::ClParameter> = Vec::new();
    ///     //..
    ///     
    ///     //create a new parser
    ///     let parser = Parser::new(&args, &valid_options, &expected_parameters).unwrap();
    ///     
    ///     assert_eq!(parser.get_valid_options(), &valid_options);
    /// ```
    pub fn get_valid_options(&self) -> &Vec<option_args::ClOption> {&self.valid_options}

    /// get a reference to `expected_parameters`
    /// # Examples 
    /// ```
    /// use std::env;
    /// use clia::{option_args::{ClOptionInfo, ClOption}, parameter_args::ClParameter, Parser};
    ///     //collect cli arguments
    ///     let args: Vec<String> = env::args().collect();
    ///     # let args: Vec<String> = vec![String::from("path/to/executable/"), String::from("path/to/search"), String::from("thing to search for")];
    ///     //define valid options
    ///     let valid_options: Vec<ClOption> = Vec::new();
    ///     //..
    ///     //define expected parameters
    ///     let expected_parameters: Vec<ClParameter> = Vec::new();
    ///     //..
    ///     # let expected_parameters: Vec<ClParameter> = vec![
    ///     #     ClParameter::new("PATH", "Path to search in"),
    ///     #     ClParameter::new("QUERY", "String to search for, all the stuff after the path wrap in \"'s if it contains spaces"),
    ///     # ];
    ///     //create a new parser
    ///     let parser = Parser::new(&args, &valid_options, &expected_parameters).unwrap();
    /// 
    ///     assert_eq!(parser.get_expected_parameters(), &expected_parameters);
    /// ```
    pub fn get_expected_parameters(&self) -> &Vec<parameter_args::ClParameter> {&self.expected_parameters}

    /// get a reference to `option_arguments_found`
    /// # Examples 
    /// ```
    /// use std::env;
    /// use clia::{option_args::{ClOptionInfo, ClOption}, parameter_args, Parser};
    /// //... 
    ///     //collect cli arguments
    ///     let args: Vec<String> = env::args().collect();
    ///     # let args: Vec<String> = vec![String::from("path/to/executable/"), String::from("-h")];
    ///     //define valid options
    ///     let valid_options: Vec<ClOption> = Vec::new();
    ///     //...
    ///     # let valid_options: Vec<ClOption> = vec![
    ///     #     ClOption::new_flag(&ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap()),
    ///     #     ClOption::new_flag_list( &ClOptionInfo::new("-f", "--filter", "Comma separated list of extensions, will only count lines of files with these extensions").unwrap(), "EXTENSIONS"),
    ///     #     ClOption::new_flag_list( &ClOptionInfo::new("-F", "--format", "Format the output in a list, valid formats are: DEFAULT, BULLET, MARKDOWN, and NUMERIC").unwrap(), "FORMAT"),
    ///     #     ClOption::new_flag(&ClOptionInfo::new("-h", "--help", "Show help").unwrap()),
    ///     # ];
    ///     //define expected parameters
    ///     let expected_parameters: Vec<parameter_args::ClParameter> = Vec::new();
    ///     //...
    ///     //create a new parser
    ///     let parser = Parser::new(&args, &valid_options, &expected_parameters).unwrap();
    ///     
    ///     assert_eq!(parser.get_option_arguments_found().iter().filter(|opt| opt.get_present()).collect::<Vec<&ClOption>>().get(0).unwrap().get_info(), &ClOptionInfo::new("-h", "--help", "Show help").unwrap());
    /// ```
    pub fn get_option_arguments_found(&self) -> &Vec<option_args::ClOption> {&self.option_arguments_found}

    /// get a reference to `parameter_arguments_found`
    /// # Examples 
    /// ```
    /// use std::env;
    /// use clia::{option_args::{ClOptionInfo, ClOption}, parameter_args::ClParameter, Parser};
    /// //... 
    ///     //collect cli arguments
    ///     let args: Vec<String> = env::args().collect();
    ///     # let args: Vec<String> = vec![String::from("path/to/executable/"), String::from("path/to/search"), String::from("thing to search for")];
    ///     //define valid options
    ///     let valid_options: Vec<ClOption> = Vec::new();
    ///     //..
    ///     //define expected parameters
    ///     let expected_parameters: Vec<ClParameter> = Vec::new();
    ///     //...
    ///     # let expected_parameters: Vec<ClParameter> = vec![
    ///     #    ClParameter::new("PATH", "Path to search in"),
    ///     #    ClParameter::new("QUERY", "String to search for, all the stuff after the path wrap in \"'s if it contains spaces"),
    ///     # ];
    ///     //create a new parser
    ///     let parser = Parser::new(&args, &valid_options, &expected_parameters).unwrap();
    ///     
    ///     assert_eq!(parser.get_parameter_arguments_found().iter().map(|param| param.get_data()).collect::<Vec<&str>>(), vec!["path/to/search", "thing to search for"]);
    /// ```
    pub fn get_parameter_arguments_found(&self) -> &Vec<parameter_args::ClParameter> {&self.parameter_arguments_found}
    
}