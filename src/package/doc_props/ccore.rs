use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "cp:coreProperties", deserialize = "coreProperties"))]
pub struct Core {
    #[serde(rename = "@xmlns:cp")]
    pub cp: String,

    #[serde(rename = "@xmlns:dc")]
    pub dc: String,

    #[serde(rename = "@xmlns:dcterms")]
    pub dcterms: String,

    #[serde(rename = "@xmlns:xsi")]
    pub xsi: String,

    #[serde(rename(deserialize = "title", serialize = "dc:title"))]
    pub title: String,

    #[serde(rename(deserialize = "creator", serialize = "dc:creator"))]
    pub creator: String,

    #[serde(rename(deserialize = "lastModifiedBy", serialize = "cp:lastModifiedBy"))]
    pub last_modified_by: String,

    #[serde(rename(deserialize = "revision", serialize = "cp:revision"))]
    pub revision: String,

    #[serde(rename(deserialize = "created", serialize = "dcterms:created"))]
    pub created: DcTerms,

    #[serde(rename(deserialize = "modified", serialize = "dcterms:modified"))]
    pub modified: DcTerms,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcTerms {
    #[serde(rename(deserialize = "type", serialize = "@xsi:type"))]
    pub created_type: Option<String>,

    #[serde(rename = "$value")]
    body: String,
}


