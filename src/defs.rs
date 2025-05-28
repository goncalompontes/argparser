#[derive(Debug)]
pub enum ArgDef<'a> {
    Short(char),
    Long(&'a str),
    ShortAndLong { short: char, long: &'a str },
}

#[derive(Debug)]
pub enum ArgName<'a> {
    Short(char),
    Long(&'a str),
}

#[derive(Debug)]
pub enum Argument<'a> {
    Positional { value: &'a str },
    Flag { name: ArgName<'a> },
    Option { name: ArgName<'a>, value: &'a str },
}

pub enum ParseArgError {
    MalformedArg(&'static str),
}

impl<'a> ArgDef<'a> {
    pub fn matches(&self, other: &ArgName<'a>) -> bool {
        match (self, other) {
            (ArgDef::Short(s), ArgName::Short(o)) => s == o,
            (ArgDef::Long(s), ArgName::Long(o)) => s == o,
            (ArgDef::ShortAndLong { short, long: _ }, ArgName::Short(o)) => short == o,
            (ArgDef::ShortAndLong { short: _, long }, ArgName::Long(o)) => long == o,
            _ => false,
        }
    }
}
