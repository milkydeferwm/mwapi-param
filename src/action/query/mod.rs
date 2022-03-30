pub mod prop;
pub mod meta;
pub mod list;
pub mod generator;

use std::collections::HashMap;

use crate::{MWAPIBuildable, builderfunc};
use super::MWAPIAction;

#[derive(Debug, Default, Clone)]
pub struct Query<'a> {
    prop: Option<&'a [&'a dyn prop::MWAPIActionQueryProp]>,
    list: Option<&'a [&'a dyn list::MWAPIActionQueryList]>,
    meta: Option<&'a [&'a dyn meta::MWAPIActionQueryMeta]>,
    indexpageids: Option<bool>,
    export: Option<bool>,
    exportnowrap: Option<bool>,
    exportschema: Option<&'a str>,
    iwurl: Option<bool>,
    param_continue: Option<&'a str>,
    raw_continue: Option<bool>,
    titles: Option<&'a [&'a str]>,
    pageids: Option<&'a [i64]>,
    revids: Option<&'a [i64]>,
    generator: Option<&'a dyn generator::MWAPIActionQueryGenerator>,
    redirects: Option<bool>,
    converttitles: Option<bool>,
}

impl<'a> Query<'a> {
    pub fn new() -> Self {
        Query {
            prop: None,
            list: None,
            meta: None,
            indexpageids: None,
            export: None,
            exportnowrap: None,
            exportschema: None,
            iwurl: None,
            param_continue: None,
            raw_continue: None,
            titles: None,
            pageids: None,
            revids: None,
            generator: None,
            redirects: None,
            converttitles: None,
        }
    }

    builderfunc!(prop, &'a [&'a dyn prop::MWAPIActionQueryProp]);
    builderfunc!(list, &'a [&'a dyn list::MWAPIActionQueryList]);
    builderfunc!(meta, &'a [&'a dyn meta::MWAPIActionQueryMeta]);
    builderfunc!(indexpageids, bool);
    builderfunc!(export, bool);
    builderfunc!(iwurl, bool);
    builderfunc!(param_continue, &'a str);
    builderfunc!(raw_continue, bool);
    builderfunc!(titles, &'a [&'a str]);
    builderfunc!(pageids, &'a [i64]);
    builderfunc!(revids, &'a [i64]);
    builderfunc!(generator, &'a dyn generator::MWAPIActionQueryGenerator);
    builderfunc!(redirects, bool);
    builderfunc!(converttitles, bool);

    pub fn exportnowrap(mut self, exportnowrap: bool) -> Self {
        if !matches!(self.export, Some(true)) {
            panic!("exportnowrap can only be used with query+export");
        }
        self.exportnowrap = Some(exportnowrap);
        self
    }

    pub fn exportschema(mut self, exportschema: &'a str) -> Self {
        if !matches!(self.export, Some(true)) {
            panic!("exportschema can only be used with query+export");
        }
        self.exportschema = Some(exportschema);
        self
    }
}

impl<'a> MWAPIAction for Query<'a> {}
impl<'a> MWAPIBuildable for Query<'a> {
    fn get_name(&self) -> &'static str {
        "query"
    }

    fn build(&self) -> HashMap<String, String> {
        use crate::util::{mw_insert, mw_insert_bool, mw_insert_collection, mw_insert_object, mw_insert_object_collection};

        let mut map: HashMap<String, String> = HashMap::new();
        mw_insert_object_collection(&mut map, "prop", self.prop, "");
        mw_insert_object_collection(&mut map, "list", self.list, "");
        mw_insert_object_collection(&mut map, "meta", self.meta, "");
        mw_insert_bool(&mut map, "indexpageids", self.indexpageids);
        mw_insert_bool(&mut map, "export", self.export);
        mw_insert_bool(&mut map, "exportnowrap", self.exportnowrap);
        mw_insert(&mut map, "exportschema", self.exportschema);
        mw_insert_bool(&mut map, "iwurl", self.iwurl);
        mw_insert(&mut map, "continue", self.param_continue);
        mw_insert_bool(&mut map, "raw_continue", self.raw_continue);
        mw_insert_collection(&mut map, "titles", self.titles);
        mw_insert_collection(&mut map, "pageids", self.pageids);
        mw_insert_collection(&mut map, "revids", self.revids);
        mw_insert_object(&mut map, "generator", self.generator, "g");
        mw_insert_bool(&mut map, "redirects", self.redirects);
        mw_insert_bool(&mut map, "converttitles", self.converttitles);
        map
    }
}
