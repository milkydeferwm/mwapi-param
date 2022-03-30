use std::collections::HashMap;

use crate::{MWAPIBuildable, builderfunc};
use super::MWAPIFormat;

#[derive(Debug, Clone, Default)]
pub struct XML<'a> {
    xslt: Option<&'a str>,
    includexmlnamespace: Option<bool>,
}

impl<'a> MWAPIFormat for XML<'a> {}
impl<'a> MWAPIBuildable for XML<'a> {
    fn get_name(&self) -> &'static str {
        "xml"
    }

    fn build(&self) -> HashMap<String, String> {
        use crate::util::{mw_insert, mw_insert_bool};

        let mut map: HashMap<String, String> = HashMap::new();
        // map.insert("format".to_string(), "xml".to_string());
        mw_insert(&mut map, "xslt", self.xslt.as_ref());
        mw_insert_bool(&mut map, "includexmlnamespace", self.includexmlnamespace);
        map
    }
}

impl<'a> XML<'a> {
    pub fn new() -> Self {
        XML {
            xslt: None,
            includexmlnamespace: None,
        }
    }

    builderfunc!(xslt, &'a str);
    builderfunc!(includexmlnamespace, bool);
}
