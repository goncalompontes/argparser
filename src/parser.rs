use crate::ArgDef;
use crate::args::Args;
use crate::defs::ArgName;
use crate::defs::Argument;
use crate::defs::ParseArgError;
use std::collections::HashMap;
use std::iter::Peekable;

/// Maintains context for parsing arguments, including definitions and lookup maps.
pub struct ParserContext<'a> {
    /// A list of defined arguments.
    defs: Vec<ArgDef<'a>>,
    /// Maps short characters (e.g., `-h`) to their index in `defs`.
    short_map: HashMap<char, usize>,
    /// Maps long strings (e.g., `--help`) to their index in `defs`.
    long_map: HashMap<&'a str, usize>,
}

impl<'a> ParserContext<'a> {
    pub fn new() -> Self {
        Self {
            defs: Vec::new(),
            short_map: HashMap::new(),
            long_map: HashMap::new(),
        }
    }

    pub fn from(defs: Vec<ArgDef<'a>>) -> Self {
        let mut ctx = Self::new();
        defs.iter().for_each(|def| {
            ctx.register(*def).unwrap();
        });
        ctx
    }

    pub fn register(&mut self, arg: ArgDef<'a>) -> Result<&Self, String> {
        // Check for conflicts
        match &arg {
            ArgDef::Short(s) => {
                if self.short_map.contains_key(s) {
                    return Err(format!("Short argument -{} already defined", s));
                }
            }
            ArgDef::Long(l) => {
                if self.long_map.contains_key(l) {
                    return Err(format!("Long argument --{} already defined", l));
                }
            }
            ArgDef::ShortAndLong { short, long } => {
                if self.short_map.contains_key(short) {
                    return Err(format!("Short argument -{} already defined", short));
                }
                if self.long_map.contains_key(long) {
                    return Err(format!("Long argument --{} already defined", long));
                }
            }
        }

        // No conflict, insert and update maps
        let index = self.defs.len();
        match &arg {
            ArgDef::Short(s) => {
                self.short_map.insert(*s, index);
            }
            ArgDef::Long(l) => {
                self.long_map.insert(l, index);
            }
            ArgDef::ShortAndLong { short, long } => {
                self.short_map.insert(*short, index);
                self.long_map.insert(long, index);
            }
        }

        self.defs.push(arg);
        Ok(self)
    }
}

pub fn parse_with_ctx<'a>(
    args: &'a [&str],
    ctx: &ParserContext,
) -> Result<Args<'a>, ParseArgError<'a>> {
    let mut result = Vec::new();
    let mut args = args.iter().peekable();

    let mut positional = false;

    while let Some(&arg) = args.next() {
        if positional {
            result.push(parse_positional(arg));
            continue;
        }

        if arg == "--" {
            positional = true;
            continue;
        }

        if arg.starts_with("--") {
            // Long argument
            let parsed = parse_long(arg, &mut args)?;
            let name = match parsed.name() {
                Some(ArgName::Long(name)) => name,
                Some(_) => unreachable!("parse_long should never return a short name"),
                None => {
                    result.push(parsed);
                    return Ok(Args(result)); // or continue, depending on your logic
                }
            };

            if !ctx.long_map.contains_key(name) {
                return Err(ParseArgError::UnknownLong(name.into()));
            }
            result.push(parsed);
        } else if arg.starts_with("-") && arg.len() > 1 {
            // Short or cluster
            let mut parsed_args = parse_short(arg, &mut args)?;
            for short_arg in &parsed_args {
                if let Some(name) = short_arg.name() {
                    match name {
                        ArgName::Short(name) => {
                            if !ctx.short_map.contains_key(&name) {
                                return Err(ParseArgError::UnknownShort(name.into()));
                            }
                        }
                        _ => unreachable!(
                            "parse_short should never return an argument with a short name"
                        ),
                    }
                }
            }
            result.append(&mut parsed_args);
        } else {
            result.push(parse_positional(arg));
        }
    }

    Ok(Args(result))
}

pub fn parse<'a>(args: &'a [&str]) -> Result<Args<'a>, ParseArgError<'a>> {
    let mut result = Vec::new();
    let mut args = args.iter().peekable();

    let mut positional = false;
    while let Some(&arg) = args.next() {
        if positional {
            result.push(parse_positional(arg));
            continue;
        }

        if arg == "--" {
            positional = true;
            continue;
        }

        if arg.starts_with("--") {
            result.push(parse_long(arg, &mut args)?);
        } else if arg.starts_with("-") {
            result.append(&mut parse_short(arg, &mut args)?);
        } else {
            result.push(parse_positional(arg));
        }
    }

    Ok(Args(result))
}

fn parse_positional(arg: &str) -> Argument {
    Argument::Positional { value: arg.into() }
}

fn parse_long<'a, I>(
    arg: &'a str,
    input: &mut Peekable<I>,
) -> Result<Argument<'a>, ParseArgError<'a>>
where
    I: Iterator<Item = &'a &'a str>,
{
    if let Some((name, value)) = arg.split_once("=") {
        Ok(Argument::Option {
            name: ArgName::Long(name),
            value,
        })
    } else if let Some(long_name) = arg.strip_prefix("--") {
        if let Some(&next) = input.peek() {
            if next.starts_with("-") {
                Ok(Argument::Flag {
                    name: ArgName::Long(long_name),
                })
            } else {
                let value = input.next().unwrap();
                Ok(Argument::Option {
                    name: ArgName::Long(long_name),
                    value,
                })
            }
        } else {
            Ok(Argument::Flag {
                name: ArgName::Long(long_name),
            })
        }
    } else {
        Err(ParseArgError::MalformedArg(
            "Malformed argument at position {index}",
        ))
    }
}

fn parse_short<'a, I>(
    arg: &'a str,
    input: &mut Peekable<I>,
) -> Result<Vec<Argument<'a>>, ParseArgError<'a>>
where
    I: Iterator<Item = &'a &'a str>,
{
    if arg.len() < 2 {
        return Err(ParseArgError::MalformedArg(
            "Malformed argument at position {index}".into(),
        ));
    }

    if let Some((names, value)) = arg.split_once('=') {
        return Ok(names
            .chars()
            .skip(1)
            .map(|short| Argument::Option {
                name: ArgName::Short(short),
                value,
            })
            .collect());
    }

    let chars: Vec<char> = arg.chars().skip(1).collect();

    if let Some(&next) = input.peek() {
        if !next.starts_with('-') {
            let value = input.next().unwrap(); // consume the argument
            return Ok(chars
                .into_iter()
                .map(|short| Argument::Option {
                    name: ArgName::Short(short),
                    value,
                })
                .collect());
        }
    }

    Ok(chars
        .into_iter()
        .map(|short| Argument::Flag {
            name: ArgName::Short(short),
        })
        .collect())
}
