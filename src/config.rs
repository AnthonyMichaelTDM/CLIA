//! # Config
//! 
//! 'config' is a collection of utilities to make parsing 
//! options from from command line arguments easy and convenient

use std::error::Error;

/// parse arguments for OPTION flags
/// valid flags are given by valid_options
/// returns a vector containing all of the ClOptions in valid_options, with their associated data updated
/// 
/// Notes: 
/// - an options short_flag must be a `-` followed by any alphabetic ascii character
/// - an options long_flag must be `--` followed by a word (or words separated by additional `-`'s)
/// 
/// # Errors
/// - `args` contains a flag (string starting with `-`) not in `valid_options` 
/// 
/// # Panics
/// - a ClOption in `valid_options` contains flags formatted improperly
/// 
/// # Examples
/// 
pub fn parse_for_options(args: &[String], valid_options: &[ClOption]) -> Result<Vec<ClOption>,Box<dyn Error>> {
    //DATA
    let mut valid_flags: Vec<String> = Vec::new();
    let flags_in_args:Vec<String>;
    let mut results: Vec<ClOption>;

    //fill valid_flags with the long and short flags of the ClOptions in valid_options
    for option in valid_options.into_iter() {
        match option {
            ClOption::Flag { present:_, info } => {
                //return error if flags are formatted improperly
                if !info.are_flags_formatted_properly() {
                    panic!("BUG: short_flag (\"{}\") and/or long_flag (\"{}\") improperly formated!", info.short_flag, info.long_flag);
                }
                //add flags
                valid_flags.push(info.short_flag.clone());
                valid_flags.push(info.long_flag.clone());
            },
            ClOption::FlagList { present:_, list_name:_, list:_, info } => {
                //return error if flags are formatted improperly
                if !info.are_flags_formatted_properly() {
                    panic!("BUG: short_flag (\"{}\") and/or long_flag (\"{}\") improperly formated!", info.short_flag, info.long_flag);
                }
                //add flags
                valid_flags.push(info.short_flag.clone());
                valid_flags.push(info.long_flag.clone());
            },
            ClOption::FlagData { present:_, data_name:_, data:_, info } => {
                //return error if flags are formatted improperly
                if !info.are_flags_formatted_properly() {
                    panic!("BUG: short_flag (\"{}\") and/or long_flag (\"{}\") improperly formated!", info.short_flag, info.long_flag);
                }
                //add flags
                valid_flags.push(info.short_flag.clone());
                valid_flags.push(info.long_flag.clone());
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
            ClOption::Flag { present, info } => {
                *present = flags_in_args.contains(&info.short_flag) || flags_in_args.contains(&info.long_flag);
            },
            ClOption::FlagList { present, list_name:_, list, info } => {
                if flags_in_args.contains(&info.short_flag) {
                    *present = true;
                    match get_list_after_flag(args, info.short_flag.as_str()) {
                        Ok(list_from_args) => *list = list_from_args,
                        Err(e) => return Err(e),
                    }
                } else if flags_in_args.contains(&info.long_flag){
                    *present = true;
                    match get_list_after_flag(args, info.long_flag.as_str()) {
                        Ok(list_from_args) => *list = list_from_args,
                        Err(e) => return Err(e),
                    }
                } 
                else {
                    *present = false;
                }
            },
            ClOption::FlagData { present, data_name:_, data, info } => {
                if flags_in_args.contains(&info.short_flag) {
                    *present = true;
                    match get_data_after_flag(args, info.short_flag.as_str()) {
                        Ok(data_from_args) => *data = data_from_args,
                        Err(e) => return Err(e),
                    }
                } else if flags_in_args.contains(&info.long_flag){
                    *present = true;
                    match get_data_after_flag(args, info.long_flag.as_str()) {
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
/// note: when using this, ensure that the returned list is as expected, as shown in examples, it will attempt to make a list out of whatever valid argument follows it
/// 
/// # Errors
/// - flag is not in args
/// - flag is last element in args
/// - item following flag in args starts with a `-` (is another flag)
/// 
/// # Examples
/// ```
/// let flag = "--your-flag";
/// let missing_flag   = vec![String::from("--not-your-flag"),String::from("your,list"),String::from("NotYourList")];
/// let missing_list   = vec![String::from("--your-flag"),String::from("--not-your-flag"),String::from("NotYourList")];
/// let flag_at_end    = vec![String::from("NotYourList"),String::from("your,list"),String::from("--your-flag")];
/// let comma_separated= vec![String::from("--your-flag"),String::from("your,list"),String::from("NotYourList")];
/// let space_separated= vec![String::from("--your-flag"),String::from("your list"),String::from("NotYourList")];
/// let wrong_list     = vec![String::from("--your-flag"),String::from("NotYourList"),String::from("your,list")]; //NOTE: this won't fail, so you need to double check the results of this function when using it
/// 
/// assert_eq!(argument_parser::config::get_list_after_flag(&missing_flag, flag).unwrap_err().to_string(),      "Could not find flag(--your-flag) in args([\"--not-your-flag\", \"your,list\", \"NotYourList\"])");
/// assert_eq!(argument_parser::config::get_list_after_flag(&missing_list, flag).unwrap_err().to_string(),      "No list found after flag(--your-flag) in args([\"--your-flag\", \"--not-your-flag\", \"NotYourList\"])");
/// assert_eq!(argument_parser::config::get_list_after_flag(&flag_at_end, flag).unwrap_err().to_string(),       "No arguments after flag(--your-flag) in args([\"NotYourList\", \"your,list\", \"--your-flag\"])");
/// assert_eq!(argument_parser::config::get_list_after_flag(&comma_separated, flag).unwrap(),                   vec!["your", "list"]);
/// assert_eq!(argument_parser::config::get_list_after_flag(&space_separated, flag).unwrap(),                   vec!["your", "list"]);
/// assert_eq!(argument_parser::config::get_list_after_flag(&wrong_list, flag).unwrap(),                        vec!["NotYourList"]);
/// ```
pub fn get_list_after_flag<'a>(args: &[String], flag: &'a str) -> Result<Vec<String>,Box<dyn Error>> {
    //DATA
    let flag_position:usize;
    let arg_after_flag: String;
    let list_separator:char;
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
    if arg_after_flag.contains(" ") { //list is space separated
        list_separator = ' ';
    } else {
        list_separator = ',';
    }
    return Ok(
        arg_after_flag.split(list_separator) //split the string up at list_separators
        .filter_map(|item| (!item.is_empty()).then(|| item.to_string())).collect() //remove empty items, convert others to Strings, and collect
    );
}
pub fn get_data_after_flag<'a>(args: &[String], flag: &'a str) -> Result<String,Box<dyn Error>> {
    //DATA
    let flag_position:usize;
    let arg_after_flag: String;
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

    return Ok(arg_after_flag);
}


/// stores the short_flag, long_flag, and description of an option
#[derive(Clone)]
pub struct ClOptionInfo {
    pub short_flag: String,
    pub long_flag: String,
    pub description:String,
}
impl ClOptionInfo {
    /// creates a new ClOptionInfo with the given info
    pub fn new(short_flag: &str, long_flag: &str, description: &str) -> ClOptionInfo {
        return ClOptionInfo {
            short_flag: short_flag.to_string(),
            long_flag: long_flag.to_string(),
            description: description.to_string(),
        };
    }

    /// returns `true` is both flags are formatted properly, `false` otherwise
    fn are_flags_formatted_properly(&self) -> bool {
        //return error if flags aren't valid
        if (self.short_flag.chars().any(|c| !(c.is_ascii_alphabetic()||c.eq(&'-'))) || (!self.short_flag.is_empty() && (!self.short_flag.starts_with("-") || self.short_flag.len()!=2))) //if short flag: contains invalid characters OR (isn't empty AND (doesn't start with '-' OR isn't 2 characters long))
        || (self.long_flag.chars().any(|c| !c.is_ascii_alphabetic() || !(c.eq(&'-')||c.eq(&'='))) || (!self.long_flag.is_empty() && !self.long_flag.starts_with("--")))//if long flag: contain invalid characters OR (isn't empty AND deosn't start with "--")
        { //either flags are invalid:
            return false;
        } else {
            return true;
        }
    }

}

/// consolidates the data of, and utilities for, the different types of options a command line program may use
/// the types of options a program may want to get from command line arguments
#[derive(Clone)]
pub enum ClOption {
    /// for options like '-r' or '--recursive'
    Flag {
        present:bool,
        info: ClOptionInfo,
    },
    /// for options like '-f <EXTENSIONS>...' or '--filter <EXTENSIONS>...'
    FlagList {
        present:bool,
        list_name: String,
        list: Vec<String>,
        info: ClOptionInfo,
    },
    /// for options like '--format <FORMAT>'
    FlagData {
        present:bool,
        data_name: String,
        data: String,
        info: ClOptionInfo,
    },
}
impl ClOption {
    /// Creates an instruction line for this option, usually used for documentation or manuals
    /// 
    /// #Examples
    /// ```
    /// let flag_info = argument_parser::config::ClOptionInfo {
    ///     short_flag:String::from("-r"),
    ///     long_flag:String::from("--recursive"),
    ///     description:String::from("Search through subdirectories recursively")
    /// };
    /// let flag_option = argument_parser::config::ClOption::new_flag(&flag_info);
    /// 
    /// let flag_list_info = argument_parser::config::ClOptionInfo {
    ///     short_flag:String::from("-l"),
    ///     long_flag:String::from("--look-for"),
    ///     description:String::from("Comma separated list of strings to look for")
    /// };
    /// let flag_list_option = argument_parser::config::ClOption::new_flag_list(&flag_list_info, "LIST");
    /// 
    /// let flag_data_info = argument_parser::config::ClOptionInfo {
    ///     short_flag:String::from("-f"),
    ///     long_flag:String::from("--format"),
    ///     description:String::from("Format to print output in, valid formats are: DEFAULT, BULLET, and NUMERIC")
    /// };
    /// let flag_data_option = argument_parser::config::ClOption::new_flag_data(&flag_data_info, "FORMAT");
    /// 
    /// assert_eq!(flag_option.gen_help_line(),      String::from("    -r, --recursive                   Search through subdirectories recursively"));
    /// assert_eq!(flag_list_option.gen_help_line(), String::from("    -l, --look-for <LIST>...          Comma separated list of strings to look for"));
    /// assert_eq!(flag_data_option.gen_help_line(), String::from("    -f, --format <FORMAT>             Format to print output in, valid formats are: DEFAULT, BULLET, and NUMERIC"));
    /// ```
    pub fn gen_help_line(&self) -> String {
        //if flags + their spacings are more than 38 characters, put description on next line
        //data
        let mut output: String = String::new();

        //build output
        match self {
            ClOption::Flag {present:_,info} => {
                //add short_flag
                output += format!("    {}{}", info.short_flag, {if info.short_flag.is_empty() {' '} else {','}}).as_str();

                //add long flag
                output += format!(
                    "{}{}",
                    {
                        if output.len() > 8 {
                            String::from("\n        ")
                        } else {
                            (0..(8-output.len())).map(|_| " ").collect::<String>()
                        }
                    },
                    info.long_flag
                ).as_str();

                //add description
                output += format!(
                    "{}{}",
                    {
                        if output.len() > 38 {
                            format!("\n{}", (0..38).map(|_| " ").collect::<String>()) //newline + 38 spaces
                        } else {
                            (0..(38-output.len())).map(|_| " ").collect::<String>()
                        }
                    },
                    info.description
                ).as_str();
            },
            ClOption::FlagList { present:_, list_name, list:_, info } => {
                //add short_flag
                output += format!("    {}{}", info.short_flag, {if info.short_flag.is_empty() {' '} else {','}}).as_str();

                //add long flag
                output += format!(
                    "{}{} <{}>...",
                    {
                        if output.len() > 8 {
                            String::from("\n        ")
                        } else {
                            (0..(8-output.len())).map(|_| " ").collect::<String>()
                        }
                    },
                    info.long_flag,
                    list_name
                ).as_str();

                //add description
                output += format!(
                    "{}{}",
                    {
                        if output.len() > 38 {
                            format!("\n{}", (0..38).map(|_| " ").collect::<String>()) //newline + 38 spaces
                        } else {
                            (0..(38-output.len())).map(|_| " ").collect::<String>()
                        }
                    },
                    info.description
                ).as_str();
            },
            ClOption::FlagData { present:_, data_name, data:_, info } => {
                //add short_flag
                output += format!("    {}{}", info.short_flag, {if info.short_flag.is_empty() {' '} else {','}}).as_str();

                //add long flag
                output += format!(
                    "{}{} <{}>",
                    {
                        if output.len() > 8 {
                            String::from("\n        ")
                        } else {
                            (0..(8-output.len())).map(|_| " ").collect::<String>()
                        }
                    },
                    info.long_flag,
                    data_name
                ).as_str();

                //add description
                output += format!(
                    "{}{}",
                    {
                        if output.len() > 38 {
                            format!("\n{}", (0..38).map(|_| " ").collect::<String>()) //newline + 38 spaces
                        } else {
                            (0..(38-output.len())).map(|_| " ").collect::<String>()
                        }
                    },
                    info.description
                ).as_str();
            },
        }

        output
    }

    
    /// Creates and returns new ClOption::Flag with the given info
    pub fn new_flag(info: &ClOptionInfo) -> ClOption {
        return ClOption::Flag { present: false, info: info.clone()};
    }
    /// Creates and returns new ClOption::FlagList with the given info
    pub fn new_flag_list(info: &ClOptionInfo, list_name: &str) -> ClOption {
        return ClOption::FlagList { present: false, list_name: list_name.to_ascii_uppercase(), list: Vec::new(), info: info.clone()};
    }
    /// Creates and returns new ClOption::FlagData with the given info
    pub fn new_flag_data(info: &ClOptionInfo, data_name: &str) -> ClOption {
        return ClOption::FlagData { present: false, data_name: data_name.to_ascii_uppercase(), data: String::new(), info: info.clone()};
    }
}
