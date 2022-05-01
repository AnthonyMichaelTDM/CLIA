//! # option_args
//! 
//! As far as this library is concerned, there are 4 types of arguments, in 2 main groups:
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
//! 'option_args' is a module containing utilities for defining
//! arguments that fall under the "Options" category

use std::error::Error;

/// stores the short_flag, long_flag, and description of an option
#[derive(Clone)]
pub struct ClOptionInfo {
    short_flag: String,
    long_flag: String,
    description:String,
}
impl ClOptionInfo {
    /// creates a new ClOptionInfo with the given info
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn new(short_flag: &str, long_flag: &str, description: &str) -> Result<ClOptionInfo,Box<dyn Error>> {
        let info = ClOptionInfo {
            short_flag: short_flag.to_string(),
            long_flag: long_flag.to_string(),
            description: description.to_string(),
        };

        if info.are_flags_formatted_properly() {
            return Ok(info);
        } else {
            return Err(format!("BUG: short_flag (\"{}\") and/or long_flag (\"{}\") improperly formated!", short_flag, long_flag).into());
        }
    }

    /// returns `true` is both flags are formatted properly, `false` parameterwise
    fn are_flags_formatted_properly(&self) -> bool {
        //if both flags are empty, return false
        if self.short_flag.is_empty() && self.long_flag.is_empty() {
            return false;
        }

        //return error if flags aren't valid
        if ( self.short_flag.chars().any( |c| !(c.is_ascii_alphabetic() || c.eq(&'-')) ) ||   ( !self.short_flag.is_empty() &&(!self.short_flag.starts_with("-") || self.short_flag.len()!=2)) ) //if short flag: contains invalid characters OR (isn't empty AND (doesn't start with '-' OR isn't 2 characters long))
        || ( self.long_flag.chars().any(  |c| !(c.is_ascii_alphabetic() || c.eq(&'-')) ) ||   ( !self.long_flag.is_empty()  && !self.long_flag.starts_with("--")) )//if long flag: contain invalid characters OR (isn't empty AND deosn't start with "--")
        { //either flags are invalid:
            return false;
        } else {
            return true;
        }
    }

    /// returns a copy of the short_flag
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn get_short_flag(&self) -> String {self.short_flag.clone()}
    /// returns a copy of the long_flag
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn get_long_flag(&self) -> String {self.long_flag.clone()}
    /// returns a copy of the description
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn get_descriptioon(&self) -> String {self.description.clone()}

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
    /// # Examples
    /// ```
    /// let flag_info = argument_parser::option_args::ClOptionInfo::new("-r", "--recursive", "Search through subdirectories recursively").unwrap();
    /// let flag_option = argument_parser::option_args::ClOption::new_flag(&flag_info);
    /// 
    /// let flag_list_info = argument_parser::option_args::ClOptionInfo::new("-l", "--look-for", "Comma separated list of strings to look for").unwrap();
    /// let flag_list_option = argument_parser::option_args::ClOption::new_flag_list(&flag_list_info, "LIST");
    /// 
    /// let flag_data_info = argument_parser::option_args::ClOptionInfo::new("-f", "--format", "Format to print output in, valid formats are: DEFAULT, BULLET, and NUMERIC").unwrap();
    /// let flag_data_option = argument_parser::option_args::ClOption::new_flag_data(&flag_data_info, "FORMAT");
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

    
    //get methods
    pub fn get_info(&self) -> ClOptionInfo {
        match self {
            Self::Flag { present:_, info } => info.to_owned(),
            Self::FlagList { present:_, list_name:_, list:_, info } => info.to_owned(),
            Self::FlagData { present:_, data_name:_, data:_, info } => info.to_owned(),
        }
    }


    /// Creates and returns new ClOption::Flag with the given info
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn new_flag(info: &ClOptionInfo) -> ClOption {
        return ClOption::Flag { present: false, info: info.clone()};
    }
    /// Creates and returns new ClOption::FlagList with the given info
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn new_flag_list(info: &ClOptionInfo, list_name: &str) -> ClOption {
        return ClOption::FlagList { present: false, list_name: list_name.to_ascii_uppercase(), list: Vec::new(), info: info.clone()};
    }
    /// Creates and returns new ClOption::FlagData with the given info
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn new_flag_data(info: &ClOptionInfo, data_name: &str) -> ClOption {
        return ClOption::FlagData { present: false, data_name: data_name.to_ascii_uppercase(), data: String::new(), info: info.clone()};
    }
}
