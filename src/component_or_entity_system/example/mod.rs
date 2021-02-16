mod manager;
mod meta;

use std::fmt::{Display, Debug};

pub trait Component: Display + Debug + 'static {
    fn get_type(&self) -> Class;
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Class(String);
