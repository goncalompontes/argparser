use crate::types::FromArgument;
use crate::{defs::*, parser};
use std::ops::Deref;

#[derive(Debug)]
pub struct Args<'a>(pub Vec<Argument<'a>>);

impl<'a> Deref for Args<'a> {
    type Target = Vec<Argument<'a>>;

    fn deref(&self) -> &Vec<Argument<'a>> {
        &self.0
    }
}

impl<'a> IntoIterator for Args<'a> {
    type Item = Argument<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, 'b> IntoIterator for &'b Args<'a> {
    type Item = &'b Argument<'a>;
    type IntoIter = std::slice::Iter<'b, Argument<'a>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, 'b> IntoIterator for &'b mut Args<'a> {
    type Item = &'b mut Argument<'a>;
    type IntoIter = std::slice::IterMut<'b, Argument<'a>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a> Args<'a> {
    pub fn find_all<T: FromArgument<'a>>(&'a self) -> Vec<T> {
        self.iter()
            .filter_map(|arg| T::from_argument(arg))
            .collect()
    }

    pub fn find<T: FromArgument<'a>>(&'a self, def: ArgDef) -> Option<T> {
        self.iter()
            .find(|arg| match arg {
                Argument::Flag { name } | Argument::Option { name, .. } => def.matches(name),
                _ => false,
            })
            .and_then(|arg| T::from_argument(arg))
    }

    pub fn parse(args: &'a [String]) -> Result<Args<'a>, ParseArgError> {
        parser::parse(args)
    }
}
