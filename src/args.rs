use crate::types::FromArgument;
use crate::{defs::*, parser};
use std::ops::Deref;

/// A parsed list of command-line arguments.
///
/// The `Args` struct wraps a `Vec<Argument<'a>>` and provides convenience
/// methods for querying and extracting data from parsed command-line arguments.
///
/// You typically obtain an `Args` instance by calling [`Args::parse()`].
///
/// # Examples
///
/// ```
/// # use argsparse::Args;
/// let args = Args::parse(&["--flag", "-o", "value"]).unwrap();
/// ```
#[derive(Debug)]
pub struct Args<'a>(pub Vec<Argument<'a>>);


/// Allows read-only access to the underlying `Vec<Argument<'a>>` using deref.
///
/// This enables calling slice and vector methods directly on `Args`,
/// such as `args.len()` or `args.iter()`.
impl<'a> Deref for Args<'a> {
    type Target = Vec<Argument<'a>>;

    fn deref(&self) -> &Vec<Argument<'a>> {
        &self.0
    }
}


/// Consumes the `Args` and returns an owning iterator over its arguments.
///
/// # Examples
///
/// ```
/// # use argsparse::Args;
/// let args = Args::parse(&["--foo", "bar"]).unwrap();
/// for arg in args {
///     println!("{:?}", arg);
/// }
/// ```
impl<'a> IntoIterator for Args<'a> {
    type Item = Argument<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


/// Returns a shared iterator over the arguments without consuming the `Args`.
///
/// # Examples
///
/// ```
/// # use argsparse::Args;
/// let args = Args::parse(&["--foo", "bar"]).unwrap();
/// for arg in &args {
///     println!("{:?}", arg);
/// }
/// ```
impl<'a, 'b> IntoIterator for &'b Args<'a> {
    type Item = &'b Argument<'a>;
    type IntoIter = std::slice::Iter<'b, Argument<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}


/// Returns a mutable iterator over the arguments without consuming the `Args`.
///
/// # Examples
///
/// ```
/// # use argsparse::{Args, Argument};
/// let mut args = Args::parse(&["--foo", "bar"]).unwrap();
/// for arg in &mut args {
///     // Modify or inspect arguments
///     println!("{:?}", arg);
/// }
/// ```
impl<'a, 'b> IntoIterator for &'b mut Args<'a> {
    type Item = &'b mut Argument<'a>;
    type IntoIter = std::slice::IterMut<'b, Argument<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}


impl<'a> Args<'a> {
    /// Finds all arguments of a given argument type.
    ///
    /// This method returns a `Vec<T>` containing all arguments that successfully convert
    /// from the internal representation using the [`FromArgument`] trait.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements [`FromArgument`], which defines how to extract the desired
    ///         data from the raw [`Argument`] enum.
    ///
    /// # Examples
    ///
    /// ```
    /// # use argsparse::{ArgName, Args, OptionArg};
    /// let args = Args::parse(&["testing", "--some", "args", "-here", "--", "--end"]).unwrap();
    ///
    /// let options = args.find_all::<OptionArg>();
    /// let expected = OptionArg {
    ///     name: &ArgName::Long("some"),
    ///     value: "args"
    /// };
    ///
    /// assert!(options.contains(&expected));
    /// ```
    ///
    /// [`FromArgument`]: crate::FromArgument
    /// [`Argument`]: crate::Argument
    pub fn find_all<T: FromArgument<'a>>(&'a self) -> Vec<T> {
        self.iter_all().collect()
    }


    /// Returns an iterator over all arguments that can be parsed into type `T`.
    ///
    /// This method is useful when you want to lazily iterate over matching arguments
    /// without collecting them into a `Vec`. It filters and maps the internal list
    /// of arguments using the [`FromArgument`] trait.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements [`FromArgument`], used to transform each raw argument.
    ///
    /// # Examples
    ///
    /// ```
    /// # use argsparse::{Args, OptionArg};
    /// let args = Args::parse(&["--foo", "bar", "--baz", "qux"]).unwrap();
    ///
    /// for opt in args.iter_all::<OptionArg>() {
    ///     println!("Option: {:?}", opt);
    /// }
    /// ```
    ///
    /// [`FromArgument`]: crate::FromArgument
    pub fn iter_all<T: FromArgument<'a>>(&'a self) -> impl Iterator<Item = T> + 'a {
        self.iter()
            .filter_map(|arg| T::from_argument(arg))
    }


    /// Finds a single argument matching the given [`ArgDef`], and parses it into type `T`.
    ///
    /// If an argument with a matching name is found and can be parsed into type `T`
    /// using [`FromArgument`], it is returned. Otherwise, returns `None`.
    ///
    /// # Arguments
    ///
    /// * `def` - The definition of the argument to search for, including its name.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements [`FromArgument`] to convert from the matched argument.
    ///
    /// # Examples
    ///
    /// ```
    /// # use argsparse::{Args, ArgDef, OptionArg};
    /// let args = Args::parse(&["--foo", "bar"]).unwrap();
    /// let def = ArgDef::Long("foo");
    ///
    /// let opt: Option<OptionArg> = args.find(def);
    /// assert!(opt.is_some());
    /// ```
    ///
    /// [`ArgDef`]: crate::ArgDef
    /// [`FromArgument`]: crate::FromArgument
    pub fn find<T: FromArgument<'a>>(&'a self, def: ArgDef) -> Option<T> {
        self.iter()
            .find(|&arg| match arg {
                Argument::Flag { name } | Argument::Option { name, .. } => def.matches(name),
                _ => false,
            })
            .and_then(|arg| T::from_argument(arg))
    }


    /// Checks if an argument matching the given [`ArgDef`] is present.
    ///
    /// Returns `true` if any flag or option matches the definition,
    ///
    /// # Arguments
    ///
    /// * `def` - The definition of the argument to look for.
    ///
    /// # Examples
    ///
    /// ```
    /// # use argsparse::{Args, ArgDef};
    /// let args = Args::parse(&["--debug"]).unwrap();
    /// let def = ArgDef::Long("debug");
    ///
    /// assert!(args.has(def));
    /// ```
    ///
    /// [`ArgDef`]: crate::ArgDef
    pub fn has(&self, def: ArgDef) -> bool {
        self.iter()
            .any(|arg| match arg {
                Argument::Flag { name } | Argument::Option { name, .. } => def.matches(name),
                _ => false,
            })
    }


    /// Parses a list of command-line arguments into an [`Args`] instance.
    ///
    /// This is the main entry point for parsing a raw slice of strings into a
    /// structured form. Returns a result containing either the parsed arguments
    /// or a [`ParseArgError`] if parsing fails.
    ///
    /// # Arguments
    ///
    /// * `args` - A slice of command-line argument strings.
    ///
    /// # Examples
    ///
    /// ```
    /// # use argsparse::Args;
    /// let args = Args::parse(&["--help", "-v", "input.txt"]).unwrap();
    /// ```
    ///
    /// [`Args`]: crate::Args
    /// [`ParseArgError`]: crate::ParseArgError
    pub fn parse(args: &'a [&str]) -> Result<Args<'a>, ParseArgError> {
        parser::parse(args)
    }

}
