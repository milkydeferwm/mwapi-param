use std::collections::HashMap;

use crate::{MWAPIBuildable, builderfunc};
use super::{MWAPIFormat, MWAPIFormatVersion};

#[derive(Debug, Clone, Default)]
pub struct PHP {
    formatversion: Option<MWAPIFormatVersion>,
}

impl MWAPIFormat for PHP {}
impl MWAPIBuildable for PHP {
    fn get_name(&self) -> &'static str {
        "php"
    }

    fn build(&self) -> HashMap<String, String> {
        use crate::util::mw_insert;

        let mut map: HashMap<String, String> = HashMap::new();
        // map.insert("format".to_string(), "php".to_string());
        mw_insert(&mut map, "formatversion", self.formatversion);
        map
    }
}

impl PHP {
    pub fn new() -> Self {
        PHP {
            formatversion: None,
        }
    }

    builderfunc!(formatversion, MWAPIFormatVersion);
}
