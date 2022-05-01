use std::env;

use argument_parser;

/// this is just an example of using this crate
fn main() {
    /*
    first step is to define options and parameters
    */
    let mut valid_options: Vec<argument_parser::option_args::ClOption> = Vec::new();
    let mut expected_parameters: Vec<argument_parser::parameter_args::ClParameter> = Vec::new();

    // define options
    valid_options.push( argument_parser::option_args::ClOption::new_flag_list( 
        &argument_parser::option_args::ClOptionInfo::new(
            "-f", 
            "--filter", 
            "Comma separated list of extensions, will only count lines of files with these extensions"
        ).unwrap(),
        "EXTENSIONS"
    ));
    valid_options.push( argument_parser::option_args::ClOption::new_flag_data( 
        &argument_parser::option_args::ClOptionInfo::new(
            "-F", 
            "--format", 
            "Format the output in a list, valid formats are: DEFAULT, BULLET, MARKDOWN, and NUMERIC"
        ).unwrap(),
        "FORMAT"
    ));
    valid_options.push( argument_parser::option_args::ClOption::new_flag( 
        &argument_parser::option_args::ClOptionInfo::new(
            "-r", 
            "--recursive", 
            "Search through subdirectories"
        ).unwrap()
    ));
    valid_options.push( argument_parser::option_args::ClOption::new_flag( 
        &argument_parser::option_args::ClOptionInfo::new(
            "-h", 
            "--help", 
            "Prints help information"
        ).unwrap()
    ));

    // define parameters
    expected_parameters.push( argument_parser::parameter_args::ClParameter::new(
        "PATH",
        "Path to file/folder to search"
    ));
    expected_parameters.push( argument_parser::parameter_args::ClParameter::new(
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
    match argument_parser::Parser::new(&args, &valid_options, &expected_parameters) {
        Ok(arg_par) => arg_parser = arg_par,
        Err(e) => {println!("{}", argument_parser::Parser::help("foo.exe", "Anthony Rubick", "Just here as an example of things you can do", &valid_options, &expected_parameters)); panic!("{}", e);},
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
        println!("{}", argument_parser::Parser::help("foo.exe", "Anthony Rubick", "Just here as an example of things you can do", &valid_options, &expected_parameters));
    }

    //how you handle the rest of the options / parameters is up to you
    
}
