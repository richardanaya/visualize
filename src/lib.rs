use serde::*;

#[derive(Serialize, Deserialize, Default)]
pub struct Visual {
    pub html: Option<String>,
    pub text: Option<String>,
    pub image: Option<String>,
    pub log: Option<String>,
    pub markdown: Option<String>,
}

pub trait Visualize {
    fn visualize(&self) -> Visual;
}

pub fn visualize(v: impl Visualize) -> String {
    serde_json::to_string(&v.visualize()).unwrap()
}
