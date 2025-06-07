use crate::defs::*;

/// A positional argument, typically representing a value not preceded by a flag or option.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PositionalArg<'a> {
    /// The raw string value of the positional argument.
    value: &'a str,
}

/// A flag argument, representing a switch with no associated value, such as `-v` or `--help`.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FlagArg<'a> {
    name: &'a ArgName<'a>,
}

/// An option argument with an associated value, such as `-o output.txt` or `--file=config.toml`.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OptionArg<'a> {
    pub name: &'a ArgName<'a>,
    pub value: &'a str,
}

pub trait FromArgument<'a>: Sized {
    /// Converts a reference to an `Argument` into `Self`, if possible.
    fn from_argument(arg: &'a Argument<'a>) -> Option<Self>;
}


impl<'a> FromArgument<'a> for PositionalArg<'a> {
    fn from_argument(arg: &'a Argument<'a>) -> Option<Self> {
        if let Argument::Positional { value } = arg {
            Some(PositionalArg { value })
        } else {
            None
        }
    }
}

impl<'a> FromArgument<'a> for FlagArg<'a> {
    fn from_argument(arg: &'a Argument<'a>) -> Option<Self> {
        if let Argument::Flag { name } = arg {
            Some(FlagArg { name })
        } else {
            None
        }
    }
}

impl<'a> FromArgument<'a> for OptionArg<'a> {
    fn from_argument(arg: &'a Argument<'a>) -> Option<Self> {
        if let Argument::Option { name, value } = arg {
            Some(OptionArg { name, value })
        } else {
            None
        }
    }
}
