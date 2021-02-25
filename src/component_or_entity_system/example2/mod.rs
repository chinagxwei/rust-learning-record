mod manager;
mod meta;

use std::any::Any;
use std::fmt::{Display, Debug};

pub trait Component: Any {}

impl<T: Any> Component for T {}
