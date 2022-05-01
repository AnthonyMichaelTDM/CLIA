//! # parameter_args
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
//! 'parameter_args' is a module containing utilities for 
//! defining arguments that fall under the "Parameters" category

/// stores data related to parameter arguments
#[derive(Clone)]
pub struct ClParameter {
    name: String,
    description: String,
    data: String,
}
impl ClParameter {
    /// creates a new ClParameter with the given info
    /// 
    /// `name` is the name of this Argument
    /// `description` is the description for this Argument. what is it? what is it for?
    /// 
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn new(name: &str, description: &str) -> ClParameter {
        let arg = ClParameter {
            name: name.to_string().to_ascii_uppercase(),
            description: description.to_string(),
            data: String::new(),
        };

        arg
    }

    /// Creates an instruction line for this option, usually used for documentation or manuals
    /// 
    /// #Examples
    /// ```
    /// let parameter_1 = argument_parser::parameter_args::ClParameter::new("PATH", "Path to search in"); 
    /// let parameter_2 = argument_parser::parameter_args::ClParameter::new("QUERY", "String to search for, all the stuff after the path wrap in \"'s if it contains spaces");
    /// 
    /// assert_eq!(parameter_1.gen_help_line(),     String::from("    PATH:\n        Path to search in"));
    /// assert_eq!(parameter_2.gen_help_line(),     String::from("    QUERY:\n        String to search for, all the stuff after the path wrap in \"'s if it contains spaces"));
    /// ```
    pub fn gen_help_line(&self) -> String {format!("    {}:\n        {}",self.name, self.description)}


    //getter methods
    /// get a reference to `name`
    /// # Example
    /// ```
    /// 
    /// ```
    pub fn get_name(&self) -> &str {&self.name}

    /// get a reference to `description`
    /// # Example
    /// ```
    /// 
    /// ```
    pub fn get_description(&self) -> &str {&self.description}

    /// get a reference to `data`
    /// # Example
    /// ```
    /// 
    /// ```
    pub fn get_data(&self) -> &str {&self.data}


    //setter methods

    /// set `name` to `new_name`
    /// # Example
    /// ``` 
    /// 
    /// ```
    pub fn set_name(&mut self, new_name: &str) {self.name = new_name.to_string();}

    /// set `description` to `new_description`
    /// # Example
    /// ``` 
    /// 
    /// ```
    pub fn set_description(&mut self, new_description: &str) {self.description = new_description.to_string();}

    /// set `data` to `new_data`
    /// # Example
    /// ``` 
    /// 
    /// ```
    pub fn set_data(&mut self, new_data: &str) {self.data = new_data.to_string();}
}
