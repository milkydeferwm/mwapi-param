use std::collections::HashMap;

use crate::MWAPIBuildable;
use super::MWAPIFormat;

#[derive(Debug, Clone, Default)]
pub struct None {}

impl MWAPIFormat for None {}
impl MWAPIBuildable for None {
    fn get_name(&self) -> &'static str {
        "none"
    }

    fn build(&self) -> HashMap<String, String> {
        HashMap::<String, String>::new()
    }
}

impl None {
    pub fn new() -> Self {
        None {}
    }
}
