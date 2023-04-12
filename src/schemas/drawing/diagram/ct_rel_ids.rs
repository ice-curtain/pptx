use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_RelIds
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "relIds", deserialize = "relIds"))]

pub struct CtRelIds {
    #[serde(rename = "@dm")]
    pub dm_attr: String,

    #[serde(rename = "@lo")]
    pub lo_attr: String,

    #[serde(rename = "@qs")]
    pub qs_attr: String,

    #[serde(rename = "@cs")]
    pub cs_attr: String,

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
