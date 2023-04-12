use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtStringTag;

/**
 * @author : zhilong.zhou
 * @description : CT_TagList
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:tagLst", deserialize = "tagLst"))]

pub struct CtTagList {
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

    #[serde(rename(serialize = "p:tag", deserialize = "tag"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CtStringTag>>,
}
