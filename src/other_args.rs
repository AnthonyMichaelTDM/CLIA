//! # other_args
//! 
//! As far as this library is concerned, there are 4 types of arguments, in 2 main groups:
//! 
//! Options:
//! - flags (ei. -r)
//! - flags w/ lists (ei -f <comma separated list> )
//! - flags w/ data (ei --format=NUMERIC)
//! 
//! and Others:
//! - (ei a file path, a string, etc.)
//! 
//! ### 
//! 
//! 'other_args' is a module containing utilities for 
//! defining arguments that fall under the "Others" category


use std::error::Error;

/// stores data related to other arguments
#[derive(Clone)]
pub struct ClOther {
    name: String,
    description: String,
    data: String,
    present: bool,
}
impl ClOther {
    /// creates a new ClOther with the given info
    pub fn new(name: &str, description: &str) -> ClOther {
        let arg = ClOther {
            name: name.to_string().to_ascii_uppercase(),
            description: description.to_string(),
            data: String::new(),
            present: false,
        };

        arg
    }

    /// Creates an instruction line for this option, usually used for documentation or manuals
    /// 
    /// #Examples
    /// ```
    /// let other_1 = argument_parser::other_args::ClOther::new("PATH", "Path to search in"); 
    /// let other_2 = argument_parser::other_args::ClOther::new("QUERY", "String to search for, all the stuff after the path wrap in \"'s if it contains spaces");
    /// 
    /// assert_eq!(other_1.gen_help_line(),     String::from("    PATH:\n    Path to search in"));
    /// assert_eq!(other_2.gen_help_line(),     String::from("    QUERY:\n    String to search for, all the stuff after the path wrap in \"'s if it contains spaces"));
    /// ```
    pub fn gen_help_line(&self) -> String {format!("    {}:\n    {}",self.name, self.description)}
}

