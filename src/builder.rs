use crate::enums::{Parameter, DASHED};
use std::collections::HashMap;

/// Unit of help message
#[derive(Default, Clone, Debug)]
pub struct Command<'a> {
    pub long_command: String,
    pub short_command: String,
    pub description: String,
    pub parameter_name: Parameter<'a>,
    pub dash_status: DASHED,
}

impl<'a> Command<'a> {
    pub fn new(
        long_command: String,
        short_command: String,
        description: String,
        parameter_name: Parameter<'a>,
        dash_status: DASHED,
    ) -> Self {
        Self {
            long_command,
            short_command,
            description,
            parameter_name,
            dash_status,
        }
    }
}

/// Structure for building help message
#[derive(Debug, Default)]
pub struct HelpMessageBuilder<'a> {
    commands: Vec<Command<'a>>,
}

impl<'a> HelpMessageBuilder<'a> {
    /// Add command to help message.
    pub fn command(
        mut self,
        short_command: &str,
        long_command: &str,
        parameter_name: Parameter<'a>,
        description: &str,
        dash_status: DASHED,
    ) -> Self {
        self.commands.push(Command::new(
            long_command.to_owned(),
            short_command.to_owned(),
            description.to_owned(),
            parameter_name,
            dash_status,
        ));
        self
    }
    fn max_width(commands: &Vec<Command<'a>>) -> HashMap<&'a str, usize> {
        let mut result = HashMap::new();
        result.insert("short", 0);
        result.insert("long", 0);
        result.insert("description", 0);
        result.insert("parameter_name", 0);

        for command in commands {
            if command.short_command.len() > *result.get("short").unwrap() {
                *result.get_mut("short").unwrap() = command.short_command.len();
            }
            if command.long_command.len() > *result.get("long").unwrap() {
                *result.get_mut("long").unwrap() = command.long_command.len();
            }
            if command.description.len() > *result.get("description").unwrap() {
                *result.get_mut("description").unwrap() = command.description.len();
            }
            if command.parameter_name.get_len() > *result.get("parameter_name").unwrap() {
                *result.get_mut("parameter_name").unwrap() = command.parameter_name.get_len();
            }
        }
        *result.get_mut("short").unwrap() += 1;
        *result.get_mut("long").unwrap() += 2;
        *result.get_mut("parameter_name").unwrap() += 2;

        result
    }

    fn field_wrapper(
        max_widths: &HashMap<&str, usize>,
        field: &str,
        max_character_number: usize,
    ) -> String {
        let mut result = String::with_capacity(field.len() + max_character_number * 3);
        let mut line_limit = max_character_number;
        let mut previous_index = 0;
        let mut current_index = line_limit;
        let spaces = max_widths["short"] + max_widths["long"] + max_widths["parameter_name"] + 9;
        if line_limit >= field.len() {
            result.push_str(field);
        }
        'outer: while line_limit < field.len() {
            while current_index >= previous_index {
                if current_index >= field.len() {
                    result.push_str(&format!(
                        "{:<spaces$}{}",
                        " ",
                        &field[previous_index..field.len()].trim()
                    ));
                    break 'outer;
                }
                if &field[current_index..current_index + 1] == " " {
                    if previous_index == 0 {
                        result.push_str(&format!(
                            "{}\n",
                            &field[previous_index..current_index].trim()
                        ));
                    } else {
                        result.push_str(&format!(
                            "{:<spaces$}{}\n",
                            " ",
                            &field[previous_index..current_index].trim()
                        ));
                    }
                    previous_index = current_index;
                    line_limit = current_index + max_character_number;
                    current_index = line_limit;
                } else {
                    current_index -= 1;
                }
                if current_index == previous_index {
                    if previous_index == 0 {
                        result.push_str(&format!(
                            "{}-\n",
                            &field[current_index..current_index + max_character_number].trim()
                        ));
                    } else {
                        result.push_str(&format!(
                            "{:<spaces$}{}-\n",
                            " ",
                            &field[current_index..current_index + max_character_number].trim()
                        ));
                    }
                    current_index += max_character_number * 2;
                    line_limit = current_index;
                    previous_index = current_index - max_character_number;
                    if line_limit >= field.len() {
                        result.push_str(&format!(
                            "{:<spaces$}{}",
                            " ",
                            &field[previous_index..field.len()].trim()
                        ));
                        break 'outer;
                    }
                }
            }
        }
        result
    }

    fn craft(command: Command<'a>, max_widths: &HashMap<&str, usize>) -> String {
        let mut message = String::with_capacity(200);
        let parameter = match command.parameter_name {
            Parameter::OPTIONAL(param) => format!("[{}]", param),
            Parameter::REQUIRED(param) => format!("<{}>", param),
            Parameter::NO => String::new(),
        };

        let description_str = Self::field_wrapper(max_widths, &command.description, 40);

        message.push_str(&format!(
            "{:<short$}   {:<long$}   {:<parameter_name$}   {:<description$}\n",
            command.short_command,
            command.long_command,
            parameter,
            // command.parameter_name,
            // command.description,
            description_str,
            short = max_widths.get("short").unwrap(),
            long = max_widths.get("long").unwrap(),
            parameter_name = max_widths.get("parameter_name").unwrap(),
            description = max_widths.get("description").unwrap(),
        ));

        message
    }
    /// Build the help message string.
    pub fn build(self) -> String {
        let commands = self.commands;
        let max_widths = Self::max_width(&commands);
        let mut result = String::new();
        for mut command in commands.clone() {
            match command.dash_status {
                DASHED::YES => {
                    if !command.long_command.is_empty() {
                        let mut long = String::with_capacity(command.long_command.len() + 2);
                        long.push_str("--");
                        long.push_str(&command.long_command);
                        command.long_command = long
                    }
                    if !command.short_command.is_empty() {
                        let mut short = String::with_capacity(command.short_command.len() + 1);
                        short.push('-');
                        short.push_str(&command.short_command);
                        command.short_command = short
                    }
                }
                DASHED::NO => {
                    command.short_command = format!(" {}", command.short_command);
                    command.long_command = format!("  {}", command.long_command);
                }
            };

            result.push_str(&Self::craft(command, &max_widths));
        }

        result
    }
}
