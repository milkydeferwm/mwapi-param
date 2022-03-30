use std::collections::HashMap;

pub mod action;
pub mod format;

mod util;

#[macro_export]
macro_rules! builderfunc {
    ( $param:ident , $type:ty ) => {
        pub fn $param(mut self, $param:$type) -> Self {
            self.$param = Some($param);
            self
        }
    };
}

pub trait MWAPIBuildable: core::fmt::Debug {
    fn get_name(&self) -> &'static str;
    fn build(&self) -> HashMap<String, String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MWAPIAssert {
    Anon,
    User,
    Bot,
}

impl core::fmt::Display for MWAPIAssert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Anon => write!(f, "anon"),
            Self::User => write!(f, "user"),
            Self::Bot => write!(f, "bot"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MWAPIErrorFormat {
    BC,
    HTML,
    None,
    PlainText,
    Raw,
    Wikitext,
}

impl core::fmt::Display for MWAPIErrorFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BC => write!(f, "bc"),
            Self::HTML => write!(f, "html"),
            Self::None => write!(f, "none"),
            Self::PlainText => write!(f, "plaintext"),
            Self::Raw => write!(f, "raw"),
            Self::Wikitext => write!(f, "wikitext"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MWAPIParams<'a> {
    action: Option<&'a dyn action::MWAPIAction>,
    format: Option<&'a dyn format::MWAPIFormat>,
    maxlag: Option<i32>,
    smaxage: Option<i32>,
    maxage: Option<i32>,
    assert: Option<MWAPIAssert>,
    assertuser: Option<&'a str>,
    requestid: Option<&'a str>,
    servedby: Option<bool>,
    curtimestamp: Option<bool>,
    responselanginfo: Option<bool>,
    origin: Option<&'a str>,
    uselang: Option<&'a str>,
    variant: Option<&'a str>,
    errorformat: Option<MWAPIErrorFormat>,
    errorlang: Option<&'a str>,
    erroruselocal: Option<bool>,
    centralauthtoken: Option<&'a str>,
}

impl<'a> MWAPIParams<'a> {
    pub fn new() -> Self {
        MWAPIParams {
            action: None,
            format: None,
            maxlag: None,
            smaxage: None,
            maxage: None,
            assert: None,
            assertuser: None,
            requestid: None,
            servedby: None,
            curtimestamp: None,
            responselanginfo: None,
            origin: None,
            uselang: None,
            variant: None,
            errorformat: None,
            errorlang: None,
            erroruselocal: None,
            centralauthtoken: None,
        }
    }

    builderfunc!(action, &'a dyn action::MWAPIAction);
    builderfunc!(format, &'a dyn format::MWAPIFormat);
    builderfunc!(maxlag, i32);
    builderfunc!(smaxage, i32);
    builderfunc!(maxage, i32);
    builderfunc!(assert, MWAPIAssert);
    builderfunc!(assertuser, &'a str);
    builderfunc!(requestid, &'a str);
    builderfunc!(servedby, bool);
    builderfunc!(curtimestamp, bool);
    builderfunc!(responselanginfo, bool);
    builderfunc!(origin, &'a str);
    builderfunc!(uselang, &'a str);
    builderfunc!(variant, &'a str);
    builderfunc!(errorformat, MWAPIErrorFormat);
    builderfunc!(errorlang, &'a str);
    builderfunc!(erroruselocal, bool);
    builderfunc!(centralauthtoken, &'a str);
}

impl<'a> MWAPIBuildable for MWAPIParams<'a> {
    fn get_name(&self) -> &'static str {
        ""
    }

    /// Build the query parameters. Calling this function will consume the whole object.
    fn build(&self) -> HashMap<String, String> {
        use util::{mw_insert, mw_insert_bool, mw_insert_object};

        let mut map: HashMap<String, String> = HashMap::new();
        if self.action.is_none() {
            panic!("action not defined")
        }
        if self.format.is_none() {
            panic!("format not defined")
        }
        mw_insert_object(&mut map, "action", self.action, "");
        mw_insert_object(&mut map, "format", self.format, "");
        mw_insert(&mut map, "maxlag", self.maxlag);
        mw_insert(&mut map, "smaxage", self.smaxage);
        mw_insert(&mut map, "maxage", self.maxage);
        mw_insert(&mut map, "assert", self.assert);
        mw_insert(&mut map, "assertuser", self.assertuser.as_ref());
        mw_insert(&mut map, "requestid", self.requestid.as_ref());
        mw_insert_bool(&mut map, "servedby", self.servedby);
        mw_insert_bool(&mut map, "curtimestamp", self.curtimestamp);
        mw_insert_bool(&mut map, "responselanginfo", self.responselanginfo);
        mw_insert(&mut map, "origin", self.origin.as_ref());
        mw_insert(&mut map, "uselang", self.uselang.as_ref());
        mw_insert(&mut map, "variant", self.variant.as_ref());
        mw_insert(&mut map, "errorformat", self.errorformat);
        mw_insert(&mut map, "errorlang", self.errorlang.as_ref());
        mw_insert_bool(&mut map, "erroruselocal", self.erroruselocal);
        mw_insert(&mut map, "centralauthtoken", self.centralauthtoken.as_ref());
        map
    }

}
