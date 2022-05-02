//! # Option parser
//! 'option_parser' is a module containing utilities for 
//! parsing CLI Arguments for arguments that fall under the "Options" category

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::error::Error;

use crate::option_args;

/// parse args for Options 
/// valid flags are given by valid_options
/// returns a vector containing all of the ClOptions in valid_options, with their associated data updated
/// 
/// # Errors
/// - `args` contains a flag (string starting with `-`) not in `valid_options` 
/// - the `args` passed would result in an error from `option_parser::get_list_after_flag()` or `option_parser::get_data_after_flag()`
/// 
/// # Examples
/// ```
/// use std::env; //allows access to the process's environment
/// 
/// use clia::{option_args,option_parser};
/// 
/// //collect cli arguments
/// let args: Vec<String> = env::args().collect();
/// 
/// //define valid options
/// let valid_options: Vec<option_args::ClOption> = Vec::new();
/// //...
/// 
/// //call option_parser::parse_for_options() to get a vector that's a copy of valid_options but with it's data updated
/// let parsed_options: Vec<option_args::ClOption> = option_parser::parse_for_options(&args, &valid_options).unwrap();
/// ```
/// 
pub fn parse_for_options(args: &[String], valid_options: &[option_args::ClOption]) -> Result<Vec<option_args::ClOption>,Box<dyn Error>> {
    //DATA
    let mut valid_flags: Vec<String> = Vec::new();
    let flags_in_args:Vec<String>;
    let mut results: Vec<option_args::ClOption>;

    //fill valid_flags with the long and short flags of the ClOptions in valid_options
    for option in valid_options.into_iter() {
        match option {
            option_args::ClOption::Flag { present:_, info } => {
                //add flags
                valid_flags.push(info.get_short_flag().to_string());
                valid_flags.push(info.get_long_flag().to_string());
            },
            option_args::ClOption::FlagList { present:_, list_name:_, list:_, info } => {
                //add flags
                valid_flags.push(info.get_short_flag().to_string());
                valid_flags.push(info.get_long_flag().to_string());
            },
            option_args::ClOption::FlagData { present:_, data_name:_, data:_, info } => {
                //add flags
                valid_flags.push(info.get_short_flag().to_string());
                valid_flags.push(info.get_long_flag().to_string());
            },
        }
    };

    //parse args for flags
    flags_in_args = (&args[0..]).iter() //iterator of arguments, ignoring the first one
    .filter(|arg| arg.starts_with("-")) //that start with a hyphen
    .map(|arg| arg.clone()) //clone them
    .collect(); //collect into vector

    //if there are invalid flags in args (flags not in valid_flags), throw an error
    if flags_in_args.iter().any(|arg| !valid_flags.contains(arg)) {
        return Err("User Error: One or more invalid flags given.".into());
    }

    //construct a list of options, with their associated data
    results = valid_options.to_vec();
    for cl_option in results.iter_mut() {
        match cl_option {
            option_args::ClOption::Flag { present, info } => {
                //update data
                *present = flags_in_args.contains(&info.get_short_flag().to_string()) || flags_in_args.contains(&info.get_long_flag().to_string());
            },
            option_args::ClOption::FlagList { present, list_name:_, list, info } => {
                //update data
                if flags_in_args.contains(&info.get_short_flag().to_string()) {
                    *present = true;
                    match get_list_after_flag(args, info.get_short_flag()) {
                        Ok(list_from_args) => *list = list_from_args,
                        Err(e) => return Err(e),
                    }
                } else if flags_in_args.contains(&info.get_long_flag().to_string()) {
                    *present = true;
                    match get_list_after_flag(args, info.get_long_flag()) {
                        Ok(list_from_args) => *list = list_from_args,
                        Err(e) => return Err(e),
                    }
                } 
                else {
                    *present = false;
                }
            },
            option_args::ClOption::FlagData { present, data_name:_, data, info } => {
                //update data
                if flags_in_args.contains(&info.get_short_flag().to_string()) {
                    *present = true;
                    match get_data_after_flag(args, info.get_short_flag()) {
                        Ok(data_from_args) => *data = data_from_args,
                        Err(e) => return Err(e),
                    }
                } else if flags_in_args.contains(&info.get_long_flag().to_string()){
                    *present = true;
                    match get_data_after_flag(args, info.get_long_flag()) {
                        Ok(data_from_args) => *data = data_from_args,
                        Err(e) => return Err(e),
                    }
                } 
                else {
                    *present = false;
                }
            },
        }
    }
    return Ok(results);
}

/// gets the list after flag from command line arguments (args), if there is one
/// 
/// 
/// # Note
/// - you probably don't need to use this, try option_parser::parse_for_options() unless you know you need this
/// - when using this, ensure that the returned list is as expected, as shown in examples, it will attempt to make a list out of whatever valid argument follows it
/// 
/// # Errors
/// - flag is not in args
/// - flag is last element in args
/// - element following flag in args starts with a `-` (is another flag)
/// 
/// # Examples
/// ```
/// use clia::option_parser;
/// 
/// let flag = "--your-flag";
/// let args = vec![String::from("--your-flag"),String::from("your,list"),String::from("--not-your-flag")];
/// 
/// assert!( option_parser::get_list_after_flag(&args, flag).is_ok() );
/// assert_eq!( option_parser::get_list_after_flag(&args, flag).unwrap(), vec!["your", "list"]);
/// ```
/// 
/// some cases where it will fail
/// ```
/// use clia::option_parser;
/// 
/// let flag = "--your-flag";
/// let missing_flag   = vec![String::from("--not-your-flag"),String::from("your,list"),String::from("NotYourList")];
/// let missing_list   = vec![String::from("--your-flag"),String::from("--not-your-flag"),String::from("NotYourList")];
/// let flag_at_end    = vec![String::from("NotYourList"),String::from("your,list"),String::from("--your-flag")];
/// let comma_separated= vec![String::from("--your-flag"),String::from("your,list"),String::from("NotYourList")];
/// let wrong_list     = vec![String::from("--your-flag"),String::from("NotYourList"),String::from("your,list")]; //NOTE: this won't fail, so you need to double check the results of this function when using it
/// 
/// assert_eq!(option_parser::get_list_after_flag(&missing_flag, flag).unwrap_err().to_string(),      "Could not find flag(--your-flag) in args([\"--not-your-flag\", \"your,list\", \"NotYourList\"])");
/// assert_eq!(option_parser::get_list_after_flag(&missing_list, flag).unwrap_err().to_string(),      "No list found after flag(--your-flag) in args([\"--your-flag\", \"--not-your-flag\", \"NotYourList\"])");
/// assert_eq!(option_parser::get_list_after_flag(&flag_at_end, flag).unwrap_err().to_string(),       "No arguments after flag(--your-flag) in args([\"NotYourList\", \"your,list\", \"--your-flag\"])");
/// assert_eq!(option_parser::get_list_after_flag(&comma_separated, flag).unwrap(),                   vec!["your", "list"]);
/// assert_eq!(option_parser::get_list_after_flag(&wrong_list, flag).unwrap(),                        vec!["NotYourList"]);
/// ```
pub fn get_list_after_flag<'a>(args: &[String], flag: &'a str) -> Result<Vec<String>,Box<dyn Error>> {
    //DATA
    let flag_position:usize;
    let arg_after_flag: String;
    let list_separator:char = ',';
    //find the position of the flag
    match args.iter().position(|arg| arg.eq(&flag)).ok_or(format!("Could not find flag({}) in args({:?})",flag,args).into()) {
        Ok(pos) => flag_position = pos,
        Err(e) => return Err(e),
    }

    //if there is no list after the flag (no more arguments or next argument is another flag)
    //flag is at end of list
    match args.get(flag_position+1) {
        Some(arg) => arg_after_flag = arg.clone(),
        None => return Err(format!("No arguments after flag({}) in args({:?})", flag, args).into()),
    }
    //arg following the flag is another flag
    if arg_after_flag.starts_with("-") {
        return Err(format!("No list found after flag({}) in args({:?})",flag,args).into());
    }

    //create and return list from arg_after_flag
    return Ok(
        arg_after_flag.split(list_separator) //split the string up at list_separators
        .filter_map(|item| (!item.is_empty()).then(|| item.to_string())).collect() //remove empty items, convert parameters to Strings, and collect
    );
}

/// gets the data after flag from command line arguments (args), if there is one
/// 
/// # Note
/// - you probably don't need to use this, try option_parser::parse_for_options() unless you know you need this
/// 
/// # Errors
/// - flag is not in args
/// - flag is last element in args
/// - element following flag in args starts with a `-` (is another flag)
/// 
/// # Examples
/// ```
/// use clia::option_parser;
/// 
/// let flag = "--your-flag";
/// let args = vec![String::from("--your-flag"),String::from("your-data"),String::from("--not-your-flag")];
/// 
/// assert!( option_parser::get_data_after_flag(&args, flag).is_ok() );
/// assert_eq!( option_parser::get_data_after_flag(&args, flag).unwrap(), "your-data" );
/// 
/// ```
/// 
/// some cases where it will fail
/// ```
/// use clia::option_parser;
/// 
/// let flag = "--your-flag";
/// let missing_flag   = vec![String::from("--not-your-flag"),String::from("your-data"),String::from("Not,Your,Data")];
/// let missing_data   = vec![String::from("--your-flag"),String::from("--not-your-flag"),String::from("Not,Your,Data")];
/// let flag_at_end    = vec![String::from("Not,Your,Data"),String::from("your-data"),String::from("--your-flag")];
/// let wrong_data     = vec![String::from("--your-flag"),String::from("Not,Your,Data"),String::from("your-data")]; //NOTE: this won't fail, so you need to double check the results of this function when using it
/// 
/// assert_eq!(option_parser::get_data_after_flag(&missing_flag, flag).unwrap_err().to_string(),      "Could not find flag(--your-flag) in args([\"--not-your-flag\", \"your-data\", \"Not,Your,Data\"])");
/// assert_eq!(option_parser::get_data_after_flag(&missing_data, flag).unwrap_err().to_string(),      "No list found after flag(--your-flag) in args([\"--your-flag\", \"--not-your-flag\", \"Not,Your,Data\"])");
/// assert_eq!(option_parser::get_data_after_flag(&flag_at_end, flag).unwrap_err().to_string(),       "No arguments after flag(--your-flag) in args([\"Not,Your,Data\", \"your-data\", \"--your-flag\"])");
/// assert_eq!(option_parser::get_data_after_flag(&wrong_data, flag).unwrap(),                        "Not,Your,Data");
/// ```
pub fn get_data_after_flag<'a>(args: &[String], flag: &'a str) -> Result<String,Box<dyn Error>> {
    //DATA
    let flag_position:usize;
    let arg_after_flag: String;
    //find the position of the flag
    match args.iter().position(|arg| arg.eq(&flag)).ok_or(format!("Could not find flag({}) in args({:?})",flag,args).into()) {
        Ok(pos) => flag_position = pos,
        Err(e) => return Err(e),
    }

    //if there is no data after the flag (no more arguments or next argument is another flag)
    //flag is at end of list
    match args.get(flag_position+1) {
        Some(arg) => arg_after_flag = arg.clone(),
        None => return Err(format!("No arguments after flag({}) in args({:?})", flag, args).into()),
    }
    //arg following the flag is another flag
    if arg_after_flag.starts_with("-") {
        return Err(format!("No list found after flag({}) in args({:?})",flag,args).into());
    }

    return Ok(arg_after_flag);
}
