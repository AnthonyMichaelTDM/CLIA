# CLIA
pronouced KLEE-uh

Rust command-line argument parser with no extra dependencies.

this is a crate with tools for parsing command-line arguments.
## Features
- Generating help messages based on available options,
- Support for all the types of arguments specified later

## Usage
### supported types of arguments
As far as this crate is concerned, there are 4 types of arguments, in 2 main groups

Options:
- flags (ei. `-r`)
- flags w/ lists (ei `-f [comma separated list]` )
- flags w/ data (ei `--format <NUMERIC>`)

and Parameters:
- (ei a file path, a string, etc.)
 
This crate makes the following assumptions about your command line program:
- all options / flags start with a `-`
- lists entered in the command line are comma separated
- options and their associated bits of data, are typed before any parameter arguments (programs usage follows this pattern: `foo.exe [OPTIONS]... [PARAMETERS]`)
- any and all "Parameters" are required, and must be included in the arguments for your program to work properly (optional arguments should be tied to flags anyway)

### installing
add `clio=0.1` to the [dependencies] of your projects cargo.toml like so:

```rust
[dependencies]
clio="0.1.0"
```

NOTE. this doesn't actually work rn, i don't have this added to crates.io yet

## Example
Here is an example showcasing all the basic features of CLIO.
```rust
use std::env;

use clio::{option_args::{ClOption,ClOptionInfo},parameter_args::ClParameter,Parser};

/// this is just an example of using this crate
fn main() {
    /* step 1: define options and parameters */
    let mut valid_options: Vec<ClOption> = Vec::new();
    let mut expected_parameters: Vec<ClParameter> = Vec::new();

    // define options
    //  this is an example of making an option with a list
    valid_options.push( ClOption::new_flag_list( 
        &ClOptionInfo::new(
            "-f", 
            "--filter", 
            "Comma separated list of extensions, will only count lines of files with these extensions"
        ).unwrap(),
        "EXTENSIONS"
    ));
    //  this is an example of making an option with some data
    valid_options.push( ClOption::new_flag_data( 
        &ClOptionInfo::new(
            "-F", 
            "--format", 
            "Format the output in a list, valid formats are: DEFAULT, BULLET, MARKDOWN, and NUMERIC"
        ).unwrap(),
        "FORMAT"
    ));
    //  this is an example of making a simple option
    valid_options.push( ClOption::new_flag( 
        &ClOptionInfo::new(
            "-r", 
            "--recursive", 
            "Search through subdirectories"
        ).unwrap()
    ));
    //  this is an example of making a simple option
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
    
    
    
    /* step 2: collect CLI Arguments and call the parser */
    let args: Vec<String> = env::args().collect(); //read the argument values from env, and collect them into a string vector
    
    //call parser
    let arg_parser;
    match Parser::new(&args, &valid_options, &expected_parameters) {
        Ok(arg_par) => arg_parser = arg_par,
        Err(e) => {println!("{}", Parser::help("foo.exe", "by Anthony Rubick", "Just here as an example of things you can do", &valid_options, &expected_parameters)); panic!("{}", e);}, //print any errors that occur
    }
    
    
    
    /* step 3: access the "found" fields of the parser */
    //store data from the parser
    let found_options = arg_parser.get_option_arguments_found();
    let found_parameters = arg_parser.get_parameter_arguments_found();
    
    
    
    /* process the arguments */
    //user passed the -h flag
    if found_options.iter().any(|opt| opt.get_info().get_short_flag().eq("-h")) {
        println!("{}", Parser::help("foo.exe", "by Anthony Rubick", "Just here as an example of things you can do", &valid_options, &expected_parameters));
    }
    
    // ...
}
```
output of -h
```
foo.exe
by Anthony Rubick

Just here as an example of things you can do

USAGE: foo.exe [OPTIONS]... [PATH] [QUERY]

OPTIONS:
    -f, --filter <EXTENSIONS>...      Comma separated list of extensions, will only count lines of files with these extensions
    -F, --format <FORMAT>             Format the output in a list, valid formats are: DEFAULT, BULLET, MARKDOWN, and NUMERIC
    -r, --recursive                   Search through subdirectories
    -h, --help                        Prints help information

PARAMETER ARGUMENTS:
    PATH:
        Path to file/folder to search
    QUERY:
        String to search for, all the stuff after the path wrap in "'s if it contains spaces
```
 
