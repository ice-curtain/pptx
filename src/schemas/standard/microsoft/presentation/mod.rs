use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p188:authorLst", deserialize = "authorLst"))]
pub struct CtAuthorList {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p188")]
    pub p188_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "p188:author", deserialize = "author"))]
    authors: Option<Vec<CtAuthor>>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CtAuthor {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@initials")]
    initials: String,

    #[serde(rename = "@userId")]
    user_id: String,

    #[serde(rename = "@providerId")]
    provider_id: String,
}