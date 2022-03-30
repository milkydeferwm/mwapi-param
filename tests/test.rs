use std::collections::HashMap;

use mwapi_param::MWAPIBuildable;

#[test]
fn basic() {
    use mwapi_param::MWAPIParams;
    use mwapi_param::format::{json::JSON, MWAPIFormatVersion};
    use mwapi_param::action::query::Query;

    let params = MWAPIParams::new()
        .action(&Query::new().titles(&["Main Page"]))
        .format(&JSON::new().formatversion(MWAPIFormatVersion::Version2))
        .build();
    assert_eq!(params, HashMap::<String, String>::from([
        ("action".to_string(), "query".to_string()),
        ("format".to_string(), "json".to_string()),
        ("titles".to_string(), "Main Page".to_string()),
        ("formatversion".to_string(), "2".to_string())
    ]));
}
