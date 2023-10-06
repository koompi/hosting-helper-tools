use super::{Deserialize, Serialize, fmt};

#[derive(Deserialize, Serialize, Default)]
// #[serde(tag = "qty", content = "data")]
#[serde(untagged)]
pub enum TargetSite {
    #[default]
    None,
    Single(String),
    Multiple(Vec<String>),
}

impl TargetSite {

    pub fn get_single_site(&self) -> &String {
        match &self {
            TargetSite::Single(singlesite) => singlesite,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for TargetSite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetSite::None => write!(f, "[]"),
            TargetSite::Single(t) => write!(f, "[\"{}\"]", t),
            TargetSite::Multiple(t) => write!(f, "{:?}", t),
        }
    }
}