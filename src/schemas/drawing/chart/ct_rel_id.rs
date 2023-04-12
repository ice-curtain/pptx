use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_RelId
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "chart", deserialize = "chart"))]

pub struct CtRelId {
    #[serde(rename = "@id")]
    pub id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p")]
    pub p_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:s")]
    pub s_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns")]
    pub default_namespace_attr: Option<String>,
}
