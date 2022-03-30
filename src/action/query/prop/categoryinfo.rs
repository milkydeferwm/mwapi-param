use std::collections::HashMap;

use crate::{MWAPIBuildable, builderfunc, action::MWAPIAction};

use super::MWAPIActionQueryProp;

#[derive(Debug, Default, Clone)]
pub struct CategoryInfo<'a> {
    param_continue: Option<&'a str>,
}

impl<'a> CategoryInfo<'a> {
    pub fn new() -> Self {
        CategoryInfo {
            param_continue: None,
        }
    }

    builderfunc!(param_continue, &'a str);
}

impl<'a> MWAPIActionQueryProp for CategoryInfo<'a> {}
impl<'a> MWAPIAction for CategoryInfo<'a> {}
impl<'a> MWAPIBuildable for CategoryInfo<'a> {
    fn get_name(&self) -> &'static str {
        "categoryinfo"
    }

    fn build(&self) -> HashMap<String, String> {
        use crate::util::mw_insert;

        let mut map: HashMap<String, String> = HashMap::new();
        mw_insert(&mut map, "cicontinue", self.param_continue);
        map
    }
}