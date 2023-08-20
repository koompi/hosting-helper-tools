use super::{fmt, Deserialize, Serialize, FromStr};

#[derive(Deserialize, Serialize, Default)]
pub enum NginxFeatures {
    Redirect,
    Proxy,
    SPA,
    FileHost,
    #[default]
    None
}

impl fmt::Display for NginxFeatures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NginxFeatures::Proxy => write!(f, "Proxy"),
            NginxFeatures::Redirect => write!(f, "Redirect"),
            NginxFeatures::FileHost => write!(f, "FileHost"),
            NginxFeatures::SPA => write!(f, "SPA"),
            NginxFeatures::None => write!(f, "None")
        }
    }
}

impl FromStr for NginxFeatures {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Proxy" | "proxy" => Ok(Self::Proxy),
            "Redirect" | "redirect" => Ok(Self::Redirect),
            "FileHost" | "filehost" => Ok(Self::FileHost),
            "SPA" | "spa" => Ok(Self::SPA),
            _ => Err(()),
        }
    }
}
