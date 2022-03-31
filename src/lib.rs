/*
what we need:
to generate a config based on some arguments
to generate Help documentation



to generate a config

there are 4 types of arguments, in 2 main groups
options:
    flags (ei. -r)
    flags w/ lists (ei -f <comma separated list> )
    flags w/ date (ei --format=NUMERIC)
and Others
    (ei a file path, a string, etc.)

*/

//! # Argument Parser
//! 
//! A library with tools for parsing command line arguments
//! 
//! Makes the following assumptions about your command line program:
//! - that all options / flags start with a `-`
//! - that lists entered in the command line are to be space or comma separated (if space separated, the list should be enclosed by `"`'s by user)

/// utilities for parsing options
pub mod config;
