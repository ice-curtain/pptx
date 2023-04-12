use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtColorTransformHeader;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorTransformHeaderLst
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "colorsDefHdrLst", deserialize = "colorsDefHdrLst"))]

pub struct CtColorTransformHeaderLst {
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

    #[serde(rename(serialize = "colorsDefHdr", deserialize = "colorsDefHdr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors_def_hdr: Option<Vec<CtColorTransformHeader>>,
}
