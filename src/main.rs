use std::env;

use clio::{option_args::{ClOption,ClOptionInfo},parameter_args::ClParameter,Parser};

/// this is just an example of using this crate
fn main() {
    /*
    first step is to define options and parameters
    */
    let mut valid_options: Vec<ClOption> = Vec::new();
    let mut expected_parameters: Vec<ClParameter> = Vec::new();

    // define options
    valid_options.push( ClOption::new_flag_list( 
        &ClOptionInfo::new(
            "-f", 
            "--filter", 
            "Comma separated list of extensions, will only count lines of files with these extensions"
        ).unwrap(),
        "EXTENSIONS"
    ));
    valid_options.push( ClOption::new_flag_data( 
        &ClOptionInfo::new(
            "-F", 
            "--format", 
            "Format the output in a list, valid formats are: DEFAULT, BULLET, MARKDOWN, and NUMERIC"
        ).unwrap(),
        "FORMAT"
    ));
    valid_options.push( ClOption::new_flag( 
        &ClOptionInfo::new(
            "-r", 
            "--recursive", 
            "Search through subdirectories"
        ).unwrap()
    ));
    valid_options.push( ClOption::new_flag( 
        &ClOptionInfo::new(
            "-h", 
            "--help", 
            "Prints help information"
        ).unwrap()
    ));

    // define parameters
    expected_parameters.push( ClParameter::new(
        "PATH",
        "Path to file/folder to search"
    ));
    expected_parameters.push( ClParameter::new(
        "QUERY",
        "String to search for, all the stuff after the path wrap in \"'s if it contains spaces"
    ));


    /*
    second step is to collect CLI Arguments and call the parser
    */
    //this is how you'd do it, but we're just gonna not
    let args: Vec<String> = env::args().collect(); //read the argument values, and collect them into a string vector

    //call parser
    let arg_parser;
    match Parser::new(&args, &valid_options, &expected_parameters) {
        Ok(arg_par) => arg_parser = arg_par,
        Err(e) => {println!("{}", Parser::help("foo.exe", "Anthony Rubick", "Just here as an example of things you can do", &valid_options, &expected_parameters)); panic!("{}", e);},
    }

    /*
    third step is to access the "found" arguments from the parser
    */
    //store output from parser
    let found_options = arg_parser.get_option_arguments_found();
    let _found_parameters = arg_parser.get_parameter_arguments_found();

    /*
    fourth step is to process the users arguments, and run the program however it'll end up working
    */

    if found_options.iter().any(|opt| opt.get_info().get_short_flag().eq("-h")) {
        println!("{}", Parser::help("foo.exe", "Anthony Rubick", "Just here as an example of things you can do", &valid_options, &expected_parameters));
    }

    //how you handle the rest of the options / parameters is up to you
    
}
