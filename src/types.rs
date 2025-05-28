use crate::defs::*;

pub struct PositionalArg<'a> {
    value: &'a str,
}

pub struct FlagArg<'a> {
    name: &'a ArgName<'a>,
}

pub struct OptionArg<'a> {
    name: &'a ArgName<'a>,
    value: &'a str,
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
