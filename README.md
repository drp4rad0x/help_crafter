 # Help Crafter
 This crate provides simple method for generating help messages for your CLI application.

 ## Quick Start

 1. command() for chaining commands to build help message
 2. build() for generate help message

 ## Example
 ```rust
use help_crafter::enums::{DASHED, Parameter};
use help_crafter::HelpMessageBuilder;

    let help = HelpMessageBuilder::default()
    .command("i", "index", Parameter::NO, "index", DASHED::YES)
    .command(
	    "k",
	    "kill",
	    Parameter::REQUIRED("sessions_id"),
	    "the big brown fox jumps over the lazy dog the big brown fox",
	    DASHED::YES,
    )
    .command(
	    "",
	    "rub",
	    Parameter::OPTIONAL("something"),
	    "rub something",
	    DASHED::NO,
    )
    .command(
	    "t",
	    "",
	    Parameter::REQUIRED("something"),
	    "t something",
	    DASHED::YES,
    )
    .command(
	    "o",
	    "",
	    Parameter::OPTIONAL("something"),
	    "o something",
	    DASHED::NO,
    )
    .build();
 ```
 Returns following message.
 ```text
-i   --index                   index                                                      
-k   --kill    <sessions_id>   the big brown fox jumps over the lazy
                               dog the big brown fox
       rub     [something]     rub something                                              
-t             <something>     t something                                                
 o             [something]     o something   
 ```
