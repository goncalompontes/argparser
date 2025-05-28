use crate::args::Args;
use crate::defs::ArgName;
use crate::defs::Argument;
use crate::defs::ParseArgError;
use std::iter::Peekable;

///
pub fn parse(args: &[String]) -> Result<Args, ParseArgError> {
    let mut result = Vec::new();
    let mut args = args.iter().map(String::as_str).peekable();

    let mut positional = false;
    while let Some(arg) = args.next() {
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

fn parse_long<'a, I>(arg: &'a str, input: &mut Peekable<I>) -> Result<Argument<'a>, ParseArgError>
where
    I: Iterator<Item = &'a str>,
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
) -> Result<Vec<Argument<'a>>, ParseArgError>
where
    I: Iterator<Item = &'a str>,
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
