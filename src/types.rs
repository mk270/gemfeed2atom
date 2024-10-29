use yaserde_derive::YaSerialize;

// used for the <link> elements in the header
#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "link")]
pub struct Link {
    #[yaserde(attribute)]
    pub href: String,
    #[yaserde(attribute)]
    pub rel: String
}

// used for the <link> elements in the individual gemlog post entries
#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "link")]
pub struct BareLink {
    #[yaserde(attribute)]
    pub href: String
}

#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "generator")]
pub struct Generator {
    #[yaserde(attribute)]
    pub uri: String,
    #[yaserde(attribute)]
    pub version: String,
    #[yaserde(text)]
    pub contents: String
}

#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "entry")]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub updated: String,
    pub link: BareLink
}

#[derive(PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "feed")]
pub struct Feed {
    #[yaserde(attribute)]
    pub xmlns: String,
    pub id: String,
    pub title: String,
    pub updated: String,
    pub links: Vec<Link>,
    pub generator: Generator,
    pub entry: Vec<Entry> //sic: the singular needs to be the name of each tag
}
