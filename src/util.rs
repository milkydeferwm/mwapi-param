use std::collections::HashMap;

#[inline]
pub(crate) fn mw_insert_bool(map: &mut HashMap<String, String>, k: &str, v: Option<bool>) {
    if matches!(v, Some(true)) {
        map.insert(k.to_string(), "".to_string());
    }
}

#[inline]
pub(crate) fn mw_insert_collection<T: ToString>(map: &mut HashMap<String, String>, k: &str, v: Option<&[T]>) {
    if let Some(vs) = v {
        map.insert(k.to_string(), vs.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("|"));
    }
}

#[inline]
pub(crate) fn mw_insert<T: ToString>(map: &mut HashMap<String, String>, k: &str, v: Option<T>) {
    if let Some(v) = v {
        map.insert(k.to_string(), v.to_string());
    }
}

#[inline]
pub(crate) fn mw_insert_object_collection<T:crate::MWAPIBuildable + ?Sized>(map: &mut HashMap<String, String>, k: &str, v: Option<&[&T]>, prefix: &str) {
    if let Some(vs) = v {
        let mut values: Vec<String> = vec![];
        for v in vs {
            values.push(v.get_name().to_string());
            let sub_values = v.build();
            map.extend(sub_values.into_iter().map(|(k, v)| (prefix.to_string() + &k, v)));
        }
        map.insert(prefix.to_string() + k, values.join("|"));
    }
}

#[inline]
pub(crate) fn mw_insert_object<T:crate::MWAPIBuildable + ?Sized>(map: &mut HashMap<String, String>, k: &str, v: Option<&T>, prefix: &str) {
    if let Some(v) = v {
        map.insert(k.to_string(), v.get_name().to_string());
        map.extend(v.build().into_iter().map(|(k, v)| (prefix.to_string() + &k, v)));
    }
}