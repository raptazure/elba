extern crate matic;
extern crate url;

use matic::{index::Index, package::Name};
use std::{path::PathBuf, str::FromStr};
use url::Url;

#[test]
fn index_success() {
    let url = Url::from_str("file://index").unwrap();
    let start = env!("CARGO_MANIFEST_DIR");
    let mut path = PathBuf::new();
    path.push(start);
    path.push("tests/index");

    let mut i = Index::new(url, path);

    let vs = i.summaries(&Name::from_str("a/a").unwrap());

    println!("{:#?}", vs);

    assert!(vs.is_ok())
}