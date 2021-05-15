use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Test(String);

impl Test {
    pub fn new(s: &str) -> Self {
        Self { 0: s.to_owned() }
    } 
}