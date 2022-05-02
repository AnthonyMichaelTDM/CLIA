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

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::error::Error;

/// stores the short_flag, long_flag, and description of an option
#[derive(Clone, Debug, PartialEq)]
pub struct ClOptionInfo {
    short_flag: String,
    long_flag: String,
    description:String,
}
impl ClOptionInfo {
    /// creates a new ClOptionInfo with the given `short_flag`, `long_flag`, and `description`
    /// 
    /// # Notes: 
    /// - the `short_flag` must be a `-` followed by any alphabetic ascii character
    /// - the `long_flag` must be `--` followed by a word (or words separated by additional `-`'s)
    /// 
    /// see below for examples
    /// 
    /// # Errors
    /// - `short_flag` and/or `long_flag` are formatted improperly
    /// 
    /// # Examples
    /// ```
    /// use clia::option_args::ClOptionInfo;
    /// 
    /// let short_flag = "-r";
    /// let long_flag = "--recursive";
    /// let description = "Search through subdirectories";
    /// 
    /// let example_option_info =  ClOptionInfo::new(short_flag, long_flag, description).unwrap();
    /// 
    /// assert_eq!(example_option_info.get_short_flag(), short_flag);
    /// assert_eq!(example_option_info.get_long_flag(), long_flag);
    /// assert_eq!(example_option_info.get_description(), description);
    /// ```
    /// Note: short_flag and long_flag must be formatted properly, if they aren't then an error will be returned
    /// ```
    /// use clia::option_args::ClOptionInfo;
    /// 
    /// // examples of improperly formatted short_flags
    /// assert!(ClOptionInfo::new("-", "--recursive", "Search through subdirectories").is_err()); // too short
    /// assert!(ClOptionInfo::new("r", "--recursive", "Search through subdirectories").is_err()); // doesn't start with a '-' and is too short
    /// assert!(ClOptionInfo::new("rr", "--recursive", "Search through subdirectories").is_err()); // doesn't start with a '-'
    /// assert!(ClOptionInfo::new("-recursive", "--recursive", "Search through subdirectories").is_err()); // -recursive is more than 2 chacters long
    /// assert!(ClOptionInfo::new("-2", "--recursive", "Search through subdirectories").is_err()); //# isn't an alphabetic ascii character
    /// assert!(ClOptionInfo::new("-#", "--recursive", "Search through subdirectories").is_err()); //# isn't an alphabetic ascii character
    /// assert!(ClOptionInfo::new("", "", "Search through subdirectories").is_err()); //both flags are empty
    /// 
    /// // examples of properly formatted short_flags
    /// assert!(ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").is_ok());
    /// assert!(ClOptionInfo::new("-R", "--recursive", "Search through subdirectories").is_ok());
    /// assert!(ClOptionInfo::new("", "--recursive", "Search through subdirectories").is_ok()); //yes, short_flag can be empty if there is a long_flag
    /// 
    /// 
    /// // examples of improperly formatted long_flags
    /// assert!(ClOptionInfo::new("-r", "-recursive", "Search through subdirectories").is_err()); //doesn't start with '--'
    /// assert!(ClOptionInfo::new("-r", "recursive", "Search through subdirectories").is_err()); //doesn't start with '--'
    /// assert!(ClOptionInfo::new("-r", "--recursive-7", "Search through subdirectories").is_err()); //7 isn't a valid character
    /// assert!(ClOptionInfo::new("-r", "--recursive-%", "Search through subdirectories").is_err()); //% isn't a valid character
    /// assert!(ClOptionInfo::new("", "", "Search through subdirectories").is_err()); //both flags are empty
    /// 
    /// 
    /// // examples of properly formatted long_flags
    /// assert!(ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").is_ok());
    /// assert!(ClOptionInfo::new("-r", "--Recursive", "Search through subdirectories").is_ok());
    /// assert!(ClOptionInfo::new("-r", "--Recurse-through-subfolders", "Search through subdirectories").is_ok()); //multiple words should be separated with '-'
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

    /// get a reference to  `short_flag`
    /// # Examples
    /// ```
    /// use clia::option_args::ClOptionInfo;
    /// 
    /// let example_info: ClOptionInfo = ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap();
    /// 
    /// assert_eq!(example_info.get_short_flag(), "-r");
    /// ```
    pub fn get_short_flag(&self) -> &str {&self.short_flag}
    /// get a reference to  `long_flag`
    /// # Examples
    /// ```
    /// use clia::option_args::ClOptionInfo;
    /// 
    /// let example_info: ClOptionInfo = ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap();
    /// 
    /// assert_eq!(example_info.get_long_flag(), "--recursive");
    /// ```
    pub fn get_long_flag(&self) -> &str {&self.long_flag}
    /// get a reference to  `description`
    /// # Examples
    /// ```
    /// use clia::option_args::ClOptionInfo;
    /// 
    /// let example_info: ClOptionInfo = ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap();
    /// 
    /// assert_eq!(example_info.get_description(), "Search through subdirectories");
    /// ```
    pub fn get_description(&self) -> &str {&self.description}

}

/// consolidates the data of, and utilities for, the different types of options a command line program may use
/// the types of options a program may want to get from command line arguments
#[derive(Clone, Debug, PartialEq)]
pub enum ClOption {
    /// for options like '-r' or '--recursive'
    Flag {
        /// is the flag present
        present:bool,
        /// the options info
        info: ClOptionInfo,
    },
    /// for options like '-f <EXTENSIONS>...' or '--filter <EXTENSIONS>...'
    FlagList {
        /// is the flag present
        present:bool,
        /// the name of this list (displayed in help messages)
        list_name: String,
        /// the list associated with this flag
        list: Vec<String>,
        /// the options info
        info: ClOptionInfo,
    },
    /// for options like '--format <FORMAT>'
    FlagData {
        /// is the flag present
        present:bool,
        /// the name of this data (displayed in help messages)
        data_name: String,
        /// the data associated with this flag
        data: String,
        /// the options info
        info: ClOptionInfo,
    },
}
impl ClOption {
    /// Creates an instruction line for this option, usually used for documentation or manuals
    /// 
    /// # Examples
    /// ```
    /// use clia::option_args::{ClOptionInfo, ClOption};
    /// 
    /// let flag_info = ClOptionInfo::new("-r", "--recursive", "Search through subdirectories recursively").unwrap();
    /// let flag_option = ClOption::new_flag(&flag_info);
    /// 
    /// let flag_list_info = ClOptionInfo::new("-l", "--look-for", "Comma separated list of strings to look for").unwrap();
    /// let flag_list_option = ClOption::new_flag_list(&flag_list_info, "LIST");
    /// 
    /// let flag_data_info = ClOptionInfo::new("-f", "--format", "Format to print output in, valid formats are: DEFAULT, BULLET, and NUMERIC").unwrap();
    /// let flag_data_option = ClOption::new_flag_data(&flag_data_info, "FORMAT");
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

    /// get a reference to `info`
    /// # Examples
    /// ```
    /// use clia::option_args::{ClOptionInfo, ClOption};
    /// 
    /// let info = ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap();
    /// let option = ClOption::new_flag(&info);
    /// 
    /// assert_eq!(option.get_info(), &ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap());
    /// ```
    pub fn get_info(&self) -> &ClOptionInfo {
        match self {
            Self::Flag { present:_, info } => &info,
            Self::FlagList { present:_, list_name:_, list:_, info } => &info,
            Self::FlagData { present:_, data_name:_, data:_, info } => &info,
        }
    }
    /// get a reference to  `short_flag`
    /// # Examples
    /// ```
    /// use clia::option_args::{ClOptionInfo, ClOption};
    /// 
    /// let example_option: ClOption = ClOption::new_flag( &ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap() );
    /// 
    /// assert_eq!(example_option.get_short_flag(), "-r");
    /// ```
    pub fn get_short_flag(&self) -> &str {self.get_info().get_short_flag()}

    /// get a reference to  `long_flag`
    /// # Examples
    /// ```
    /// use clia::option_args::{ClOptionInfo, ClOption};
    /// 
    /// let example_option: ClOption = ClOption::new_flag( &ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap() );
    /// 
    /// assert_eq!(example_option.get_long_flag(), "--recursive");
    /// ```
    pub fn get_long_flag(&self) -> &str {self.get_info().get_long_flag()}

    /// get a reference to  `description`
    /// # Examples
    /// ```
    /// use clia::option_args::{ClOptionInfo, ClOption};
    /// 
    /// let example_option: ClOption = ClOption::new_flag( &ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap() );
    /// 
    /// assert_eq!(example_option.get_description(), "Search through subdirectories");
    /// ```
    pub fn get_description(&self) -> &str {self.get_info().get_description()}


    /// gets a reference to `present`
    /// 
    /// # Examples
    /// ```
    /// use clia::{option_args::{ClOptionInfo, ClOption}, Parser};
    /// 
    /// let flag_option = ClOption::new_flag(&ClOptionInfo::new("-r", "--recursive", "Search through subdirectories recursively").unwrap());
    /// let args: Vec<String> = vec![String::from("path/to/executable/"), String::from("-r")];
    /// let valid_options = vec![flag_option.clone()];
    /// let expected_parameters = Vec::new();
    /// 
    /// //default is false
    /// assert_eq!(flag_option.get_present(), false );
    /// 
    /// //use the Parser to get updated data from args
    /// let parser: Parser = Parser::new(&args, &valid_options, &expected_parameters).unwrap();
    /// 
    /// let found_flag = parser.get_option_arguments_found().get(0).unwrap();
    /// 
    /// //now is true
    /// assert!(found_flag.get_present());
    /// 
    /// ```
    pub fn get_present(&self) -> bool {
        match self {
            ClOption::Flag { present, info:_ } => *present,
            ClOption::FlagList { present, list_name:_, list:_, info:_ } => *present,
            ClOption::FlagData { present, data_name:_, data:_, info:_ } => *present,
        }
    }

    /// gets a reference to `list`
    /// # None
    /// - returns none is self is not of type ClOption::FlagList
    /// 
    /// # Examples
    /// ```
    /// use clia::{option_args::{ClOptionInfo, ClOption}, Parser};
    /// 
    /// let flag_list_option = ClOption::new_flag_list(&ClOptionInfo::new("-l", "--look-for", "Comma separated list of strings to look for").unwrap(), "LIST");
    /// let args: Vec<String> = vec!["path/to/executable/".to_string(), "-l".to_string(), "a,list,of,stuff".to_string()];
    /// let valid_options = vec![flag_list_option.clone()];
    /// let expected_parameters = Vec::new();
    /// 
    /// //default is empty
    /// assert!( flag_list_option.get_list().unwrap().is_empty());
    /// 
    /// //will return a poulated vec if Parser found one
    /// let parser: Parser = Parser::new(&args, &valid_options, &expected_parameters).unwrap();
    /// let found_flag = parser.get_option_arguments_found().get(0).unwrap();
    /// assert_eq!(found_flag.get_list(), Some(&vec!["a".to_string(),"list".to_string(),"of".to_string(),"stuff".to_string()]) );
    /// 
    /// //returns none if ClOption is not of type FlagList
    /// let flag_option = ClOption::new_flag(&ClOptionInfo::new("-r", "--recursive", "Search through subdirectories recursively").unwrap());
    /// assert_eq!(flag_option.get_list(), None);
    /// 
    /// 
    /// ```
    pub fn get_list(&self) ->  Option<&Vec<String>> {
        match self {
            ClOption::Flag { present:_, info:_ } => None,
            ClOption::FlagList { present:_, list_name:_, list, info:_ } => Some(list),
            ClOption::FlagData { present:_, data_name:_, data:_, info:_ } => None,
        }
    }

    /// gets a reference to `data`
    /// # None
    /// - returns none is self is not of type ClOption::FlagData
    /// 
    /// # Examples
    /// ```
    /// use clia::{option_args::{ClOptionInfo, ClOption}, Parser};
    /// let flag_data_option = ClOption::new_flag_data(&ClOptionInfo::new("-f", "--format", "Format to print output in, valid formats are: DEFAULT, BULLET, and NUMERIC").unwrap(), "FORMAT");
    /// let args: Vec<String> = vec![String::from("path/to/executable/"), String::from("-f"), String::from("DEFAULT")];
    /// let valid_options = vec![flag_data_option.clone()];
    /// let expected_parameters = Vec::new();
    /// 
    /// //default is an empty String
    /// assert_eq!( flag_data_option.get_data().unwrap(), &String::new());
    /// 
    /// //will return a poulated string if Parser found one
    /// let parser: Parser = Parser::new(&args, &valid_options, &expected_parameters).unwrap();
    /// let found_flag = parser.get_option_arguments_found().get(0).unwrap();
    /// assert_eq!(found_flag.get_data(), Some(&String::from("DEFAULT")) );
    /// 
    /// //returns none if ClOption is not of type FlagData 
    /// let flag_option = ClOption::new_flag(&ClOptionInfo::new("-r", "--recursive", "Search through subdirectories recursively").unwrap());
    /// assert_eq!(flag_option.get_list(), None);
    /// 
    /// ```
    pub fn get_data(&self) ->  Option<&String> {
        match self {
            ClOption::Flag { present:_, info:_ } => None,
            ClOption::FlagList { present:_, list_name:_, list:_, info:_ } => None,
            ClOption::FlagData { present:_, data_name:_, data, info:_ } => Some(data),
        }
    }


    
    
    
        





    /// Creates and returns new ClOption::Flag with the given info
    /// # Examples
    /// ```
    /// use clia::option_args::{ClOptionInfo, ClOption};
    /// 
    /// let example_option: ClOption = ClOption::new_flag(&ClOptionInfo::new("-r", "--recursive", "Search through subdirectories").unwrap() ); 
    /// ```
    pub fn new_flag(info: &ClOptionInfo) -> ClOption {
        return ClOption::Flag { present: false, info: info.clone()};
    }
    /// Creates and returns new ClOption::FlagList with the given info
    /// # Examples
    /// ```
    /// use clia::option_args::{ClOptionInfo, ClOption};
    /// 
    /// let example_option: ClOption = ClOption::new_flag_list( &ClOptionInfo::new("-f", "--filter", "Comma separated list of extensions, will only count lines of files with these extensions").unwrap(), "EXTENSIONS"); 
    /// ```
    pub fn new_flag_list(info: &ClOptionInfo, list_name: &str) -> ClOption {
        return ClOption::FlagList { present: false, list_name: list_name.to_ascii_uppercase(), list: Vec::new(), info: info.clone()};
    }
    /// Creates and returns new ClOption::FlagData with the given info
    /// # Examples
    /// ```
    /// use clia::option_args::{ClOptionInfo, ClOption};
    /// 
    /// let example_option: ClOption = ClOption::new_flag_list( &ClOptionInfo::new("-F", "--format", "Format the output in a list, valid formats are: DEFAULT, BULLET, MARKDOWN, and NUMERIC").unwrap(), "FORMAT"); 
    /// ```
    pub fn new_flag_data(info: &ClOptionInfo, data_name: &str) -> ClOption {
        return ClOption::FlagData { present: false, data_name: data_name.to_ascii_uppercase(), data: String::new(), info: info.clone()};
    }
}
