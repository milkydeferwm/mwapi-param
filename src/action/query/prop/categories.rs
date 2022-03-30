use std::collections::HashMap;

use crate::{MWAPIBuildable, builderfunc, action::{MWAPIAction, LimitOrMax}};

use super::MWAPIActionQueryProp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLProp {
    Hidden,
    SortKey,
    TimeStamp,
}

impl core::fmt::Display for CLProp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hidden => write!(f, "hidden"),
            Self::SortKey => write!(f, "sortkey"),
            Self::TimeStamp => write!(f, "timestamp"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLShow {
    NotHidden,
    Hidden,
}

impl core::fmt::Display for CLShow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotHidden => write!(f, "!hidden"),
            Self::Hidden => write!(f, "hidden"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLDir {
    Ascending,
    Descending,
}

impl core::fmt::Display for CLDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascending => write!(f, "ascending"),
            Self::Descending => write!(f, "descending"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Categories<'a> {
    prop: Option<&'a [CLProp]>,
    show: Option<&'a [CLShow]>,
    limit: Option<LimitOrMax>,
    param_continue: Option<&'a str>,
    categories: Option<&'a [&'a str]>,
    dir: Option<CLDir>,
}

impl<'a> Categories<'a> {
    pub fn new() -> Self {
        Categories {
            prop: None,
            show: None,
            limit: None,
            param_continue: None,
            categories: None,
            dir: None
        }
    }

    builderfunc!(prop, &'a [CLProp]);
    builderfunc!(show, &'a [CLShow]);
    builderfunc!(limit, LimitOrMax);
    builderfunc!(param_continue, &'a str);
    builderfunc!(categories, &'a [&'a str]);
    builderfunc!(dir, CLDir);
}

impl<'a> MWAPIActionQueryProp for Categories<'a> {}
impl<'a> MWAPIAction for Categories<'a> {}
impl<'a> MWAPIBuildable for Categories<'a> {
    fn get_name(&self) -> &'static str {
        "categories"
    }

    fn build(&self) -> HashMap<String, String> {
        use crate::util::{mw_insert, mw_insert_collection};

        let mut map: HashMap<String, String> = HashMap::new();
        mw_insert_collection(&mut map, "clprop", self.prop);
        mw_insert_collection(&mut map, "clshow", self.show);
        mw_insert(&mut map, "cllimit", self.limit);
        mw_insert(&mut map, "clcontinue", self.param_continue);
        mw_insert_collection(&mut map, "clcategories", self.categories);
        mw_insert(&mut map, "cldir", self.dir);
        map
    }
}
