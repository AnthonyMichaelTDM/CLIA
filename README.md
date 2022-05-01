# CLI Argument Parser Library
 
 A crate with tools for parsing command line arguments
 
 As far as this crate is concerned, there are 4 types of arguments, in 2 main groups
 
 Options:
 - flags (ei. -r)
 - flags w/ lists (ei -f <comma separated list> )
 - flags w/ data (ei --format=NUMERIC)
 
 and Parameters:
 - (ei a file path, a string, etc.)
 
 ### 
 
 This crate makes the following assumptions about your command line program:
 - that all options / flags start with a `-`
 - that lists entered in the command line are comma separated
 - options and their associated bits of data, are typed before any parameter arguments
 - any and all "Parameters" are required, and must be included in the arguments for your program to work properly (optional arguments should be tied to flags anyway)
