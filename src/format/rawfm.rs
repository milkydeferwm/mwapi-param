use std::collections::HashMap;

use crate::MWAPIBuildable;
use super::MWAPIFormat;

#[derive(Debug, Clone, Default)]
pub struct RawFm {}

impl MWAPIFormat for RawFm {}
impl MWAPIBuildable for RawFm {
    fn get_name(&self) -> &'static str {
        "rawfm"
    }

    fn build(&self) -> HashMap<String, String> {
        HashMap::<String, String>::new()
    }
}

impl RawFm {
    pub fn new() -> Self {
        RawFm {}
    }
}
