use crate::action::MWAPIAction;

use super::{prop, list, meta};

pub trait MWAPIActionQueryGenerator: MWAPIAction {}

// The following modules can be used as generators
impl<'a> MWAPIActionQueryGenerator for prop::categories::Categories<'a> {}