use std::collections::HashMap;

use crate::{MWAPIBuildable, builderfunc};
use super::{MWAPIFormat, MWAPIFormatVersion};

#[derive(Debug, Clone, Default)]
pub struct JSON<'a> {
    callback: Option<&'a str>,
    utf8: Option<bool>,
    ascii: Option<bool>,
    formatversion: Option<MWAPIFormatVersion>,
}

impl MWAPIFormat for JSON<'_> {}
impl MWAPIBuildable for JSON<'_> {
    fn get_name(&self) -> &'static str {
        "json"
    }

    fn build(&self) -> HashMap<String, String> {
        use crate::util::{mw_insert_bool, mw_insert};

        let mut map: HashMap<String, String> = HashMap::new();
        mw_insert(&mut map, "callback", self.callback.as_ref());
        mw_insert_bool(&mut map, "utf8", self.utf8);
        mw_insert_bool(&mut map, "ascii", self.ascii);
        mw_insert(&mut map, "formatversion", self.formatversion);
        map
    }
}

impl<'a> JSON<'a> {
    pub fn new() -> Self {
        JSON {
            callback: None,
            utf8: None,
            ascii: None,
            formatversion: None
        }
    }

    builderfunc!(callback, &'a str);
    builderfunc!(utf8, bool);
    builderfunc!(ascii, bool);
    builderfunc!(formatversion, MWAPIFormatVersion);
}

#[cfg(test)]
mod test {
    use crate::{format::MWAPIFormatVersion, MWAPIBuildable};
    use super::JSON;
    use std::collections::HashMap;

    #[test]
    fn format_json() {
        let params = JSON::new().build();
        assert_eq!(params, HashMap::<String, String>::from([]));
    }

    #[test]
    fn format_json_with_callback() {
        let params = JSON::new().callback("hello").build();
        assert_eq!(params, HashMap::<String, String>::from([
            ("callback".to_string(), "hello".to_string()),
        ]));
    }

    #[test]
    fn format_json_with_utf8() {
        let params = JSON::new().utf8(true).build();
        assert_eq!(params, HashMap::<String, String>::from([
            ("utf8".to_string(), "".to_string()),
        ]));
    }

    #[test]
    fn format_json_with_ascii() {
        let params = JSON::new().ascii(true).build();
        assert_eq!(params, HashMap::<String, String>::from([
            ("ascii".to_string(), "".to_string()),
        ]));
    }

    #[test]
    fn format_json_with_version2() {
        let params = JSON::new().formatversion(MWAPIFormatVersion::Version2).build();
        assert_eq!(params, HashMap::<String, String>::from([
            ("formatversion".to_string(), "2".to_string()),
        ]));
    }

    #[test]
    fn format_json_with_version1_and_utf8() {
        let params = JSON::new().formatversion(MWAPIFormatVersion::Version1).utf8(true).build();
        assert_eq!(params, HashMap::<String, String>::from([
            ("formatversion".to_string(), "1".to_string()),
            ("utf8".to_string(), "".to_string()),
        ]));
    }
}