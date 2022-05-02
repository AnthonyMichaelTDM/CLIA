//! # Parameter parser
//! 'parameter_parser' is a module containing utilities for 
//! parsing CLI Arguments for arguments that fall under the "Parameters" category

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

use std::error::Error;

use crate::parameter_args;


/// parse args for Parameters
/// expected types of Arguments are given by `expected_parameters`
/// returns a vector containing all of the `ClParameters` in `expected_parameters`, with their associated data updated
/// 
/// # Notes: 
/// - the order of elements in `expected_parameters` is the order these arguments must appear in.
/// - these arguments are the last things a user types in the command line (after Options)
/// 
/// # Errors
/// - `args` is too short to have all the expected data
/// 
/// # Examples
/// ```
/// use std::env; //allows access to the process's environment
/// use clia::{parameter_args::ClParameter,parameter_parser};
/// //...
///     //collect cli arguments
///     let args: Vec<String> = env::args().collect();
///     //define expected parameters
///     let expected_parameters: Vec<ClParameter> = Vec::new();
///     //..
///     
///     //call parameter_parser::parse_for_parameters() to get a vector that's a copy of expected_parameters but with it's data updated
///     let parsed_parameters: Vec<ClParameter> = parameter_parser::parse_for_parameters(&args, &expected_parameters).unwrap();
/// ```
/// 
pub fn parse_for_parameters(args: &[String], expected_parameters: &[parameter_args::ClParameter]) -> Result<Vec<parameter_args::ClParameter>,Box<dyn Error>> {
    //DATA
    let mut results: Vec<parameter_args::ClParameter> = Vec::new();

    //return an error is args is too short
    if args.len()-1 < expected_parameters.len() {
        return Err(format!("User Error: the amount of passed args is too small to possibly contain all the expected data").into());
    }

    //look at the last expected_parameters.len() elements of args
    for arg in (&args[args.len()-expected_parameters.len()..]).iter().enumerate() {
        if let Some(expected_parameter) = expected_parameters.get(arg.0) {
            results.push(expected_parameter.clone())
        } else { return Err(format!("Bug: index {} out of bounds of expected_parameters", arg.0).into());}

        results[arg.0].set_data(arg.1);
    }


    return Ok(results);

}