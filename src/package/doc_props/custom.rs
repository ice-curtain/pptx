use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "Properties")]
pub struct Custom {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    #[serde(rename = "@xmlns:vt")]
    pub vt: String,

    #[serde(rename = "property")]
    properties:Option<Vec<Property>>

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Property {
    #[serde(rename = "@fmtid")]
    fmtid: String,

    #[serde(rename = "@pid")]
    pid: String,

    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "lpwstr", serialize = "vt:lpwstr"))]
    pub lpwstr: Option<String>,
}