use crate::defs::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PositionalArg<'a> {
    value: &'a str,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FlagArg<'a> {
    name: &'a ArgName<'a>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OptionArg<'a> {
    pub name: &'a ArgName<'a>,
    pub value: &'a str,
}

pub trait FromArgument<'a>: Sized {
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
