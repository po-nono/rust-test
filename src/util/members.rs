use std::fs;
use serde_json;
use crate::controller::git::Member;
use std::path::Path;

pub fn read() -> Vec<Member> {
    let path = Path::new("./members.json").to_str().unwrap().to_string();
    let input_fn = fs::read_to_string(path).expect("JSON Read Failed.");
    let deserialized = serde_json::from_str::<Vec<Member>>(&input_fn).unwrap();
    deserialized
}
