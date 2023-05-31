//! # Help Crafter  
//! This crate provides simple method for generating help messages for your CLI application.  
//!  
//! ## Quick Start  
//!  
//! 1. command() for chaining commands to build help message  
//! 2. build() for generate help message  
//!  
//! ## Example  
//! ```rust  
//! use help_crafter::HelpMessageBuilder;  
//! let help = HelpMessageBuilder::default()  
//!     .command("i", "index", "", "index", false)  
//!     .command(  
//!         "k",  
//!         "kill",  
//!         "session_id",  
//!         "the big brown fox jumps over the lazy dog the big brown fox",  
//!         false,  
//!     )  
//!    .command("", "rub", "sub", "rub something", true)  
//!    .command("t", "", "something", "t something", false)  
//!    .command("o", "", "something", "o something", true)  
//!    .build();  
//! ```  
//! Returns following message.  
//! ```text  
//! -i   --index                  index                                                      
//! -k   --kill    <session_id>   the big brown fox jumps over the lazy  
//!                               dog the big brown fox  
//!        rub     <sub>          rub something                                                
//! -t             <something>    t something                                                
//!  o             <something>    o something  
//! ```  
