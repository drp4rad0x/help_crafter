/// Dash status of command.
#[derive(Default, Clone, Debug)]
pub enum DASHED {
    #[default]
    YES,
    NO,
}

/// Parameter of command
#[derive(Default, Clone, Debug)]
pub enum Parameter<'a> {
    REQUIRED(&'a str),
    OPTIONAL(&'a str),
    #[default]
    NO,
}

impl<'a> Parameter<'a> {
    pub fn get_len(&self) -> usize {
        match self {
            Parameter::OPTIONAL(value) | Parameter::REQUIRED(value) => value.len(),
            Parameter::NO => 0,
        }
    }
}
