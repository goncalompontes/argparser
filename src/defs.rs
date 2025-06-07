/// Defines the expected arguments the parser can recognize.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgDef<'a> {
    /// A short argument definition (e.g., `-h`).
    Short(char),
    /// A long argument definition (e.g., `--help`).
    Long(&'a str),
    /// Defines both a short and long version of an argument.
    ShortAndLong {
        /// The short character.
        short: char,
        /// The long name.
        long: &'a str,
    },
}

/// Represents the name of an argument, used for identification and matching.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgName<'a> {
    /// A short name, e.g., `-h`.
    Short(char),
    /// A long name, e.g., `--help`.
    Long(&'a str),
}

/// A parsed command-line argument.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Argument<'a> {
    /// A raw positional value, e.g., a file path or input string.
    Positional {
        /// The string value.
        value: &'a str,
    },
    /// A flag that was present, e.g., `--verbose`.
    Flag {
        /// The name of the flag.
        name: ArgName<'a>,
    },
    /// An option with an associated value, e.g., `--output result.txt`.
    Option {
        /// The name of the option.
        name: ArgName<'a>,
        /// The associated value.
        value: &'a str,
    },
}

/// Represents possible parsing errors when processing a single argument.
#[derive(Debug)]
pub enum ParseArgError<'a> {
    /// The argument is syntactically malformed or not valid.
    MalformedArg(&'a str),
    /// The long argument name is not defined in the context.
    UnknownLong(String),
    /// The short argument name is not defined in the context.
    UnknownShort(String),
}


impl<'a> ArgDef<'a> {

    /// Returns `true` if the `ArgDef` matches the given `ArgName`.
    ///
    /// # Example
    /// ```
    /// # use argsparse::{ArgDef, ArgName};
    /// let def = ArgDef::Short('v');
    /// assert!(def.matches(&ArgName::Short('v')));
    /// ```
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

impl<'a> Argument<'a> {

    /// Returns the name of the argument if it's a `Flag` or `Option`.
    ///
    /// Returns `None` for `Positional` arguments.
    ///
    /// # Example
    /// ```
    ///
    /// use argsparse::{ArgName, Argument};
    /// let arg = Argument::Flag { name: ArgName::Short('v') };
    /// assert_eq!(arg.name(), Some(ArgName::Short('v')));
    pub fn name(&self) -> Option<ArgName> {
        match self {
            Argument::Flag { name } | Argument::Option { name, .. } => { Some(*name) },
            _ => None,
        }
    }
}
